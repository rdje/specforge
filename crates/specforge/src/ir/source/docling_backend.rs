use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};
use std::time::{Duration, Instant};

use serde::Deserialize;
use tempfile::tempdir;

use crate::error::{AppError, Result};

use super::{
    ContentElementRecord, ContentSectionRecord, DocumentProfile, PageArtifact, PlaceholderBinding,
    SourceArtifactLayout, StructuredTableRecord, VisualAsset,
};

const DOCLING_HELPER_ENV: &str = "SPECFORGE_DOCLING_HELPER";
pub const DOCLING_PYTHON_ENV: &str = "SPECFORGE_DOCLING_PYTHON";
pub const DEFAULT_DOCLING_BOOTSTRAP_SCRIPT: &str = "scripts/bootstrap_docling.sh";
pub const DEFAULT_DOCLING_VENV_DIR: &str = ".venv-docling";

/// Used-memory percentage at or above which an in-flight ingest is aborted to protect the host.
/// Default is the owner's non-negotiable safety policy (kill at ≥85% used, below the 90% danger
/// floor). Set to `off`/`none`/`disabled` (or any value `<= 0` / `>= 100`) to disable the guard;
/// any other value is parsed as a percentage, and unparseable text falls back to the default.
const INGEST_RAM_ABORT_PERCENT_ENV: &str = "SPECFORGE_INGEST_RAM_ABORT_PERCENT";
/// Seconds between system-memory samples while the Docling subprocess runs (default 2, floor 1).
const INGEST_RAM_SAMPLE_SECS_ENV: &str = "SPECFORGE_INGEST_RAM_SAMPLE_SECS";
const DEFAULT_INGEST_RAM_ABORT_PERCENT: f64 = 85.0;
const DEFAULT_INGEST_RAM_SAMPLE_SECS: u64 = 2;

/// Minimum free disk (MB) the ingest pre-flight requires before launching. Unset → estimate from
/// the source PDF size; a positive integer → a fixed MB floor; `off`/`none`/`disabled`/`0` →
/// disabled. The pre-flight runs before any staging directory is created, so a refusal touches
/// nothing on disk.
const INGEST_MIN_FREE_DISK_MB_ENV: &str = "SPECFORGE_INGEST_MIN_FREE_DISK_MB";
/// Base free-disk headroom (MB) the size-scaled estimate always requires (markdown + backend raw
/// JSON + figure/table region crops), on top of the source-size multiple.
const DEFAULT_INGEST_DISK_BASE_HEADROOM_MB: u64 = 128;
/// Multiple of the source PDF size the estimate adds to the base headroom (conservative — the `.3`
/// disk bounding makes a large doc's bundle closer to `O(source)` than this assumes).
const DEFAULT_INGEST_DISK_SIZE_MULTIPLIER: u64 = 4;
/// Cadence at which the in-flight child is polled for completion. Kept short (independent of the
/// memory-sample interval) so a fast ingest is noticed promptly while memory is sampled only every
/// `INGEST_RAM_SAMPLE_SECS`.
const RAM_GUARD_POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Page-range batch size CEILING (MEMORY-BOUNDED-INGEST.1/.4c). Adaptive sizing (`.4c`) only ever
/// LOWERS this on a small machine; the Docling helper reads the same env, and `materialize_pdf` sets
/// the child's value to the resolved effective size. Kept in sync with the Python helper default.
const INGEST_BATCH_PAGES_ENV: &str = "SPECFORGE_INGEST_BATCH_PAGES";
const DEFAULT_INGEST_BATCH_PAGES: usize = 64;
/// Whether ingest auto-sizes the batch down to the host's TOTAL physical RAM (default on). Set
/// `off`/`none`/`disabled` to force the fixed ceiling (the exact `.1` behavior). Total RAM is a
/// per-machine constant, so the chosen batch — and the run's output — stays deterministic
/// per-machine (free memory would jitter run-to-run; see the `evidence-build-nondeterminism` KM
/// card), while transient pressure stays the `.4a` RAM guard's job.
const INGEST_ADAPTIVE_BATCH_ENV: &str = "SPECFORGE_INGEST_ADAPTIVE_BATCH";
/// Floor below which adaptive sizing never shrinks the batch (clamped to `<=` the ceiling).
const DEFAULT_INGEST_MIN_BATCH_PAGES: usize = 8;
/// Total-RAM bands (MB) for adaptive batch sizing: at/above the full band the ceiling is used; the
/// mid/low bands cap the batch at progressively smaller sizes so a 64-page-batch peak (~4.8 GB, the
/// `.2` CHI datum) stays near ~30 % of RAM; below the low band the floor is used.
const ADAPTIVE_BATCH_FULL_CEILING_MIN_MB: u64 = 16 * 1024;
const ADAPTIVE_BATCH_MID_MIN_MB: u64 = 8 * 1024;
const ADAPTIVE_BATCH_LOW_MIN_MB: u64 = 4 * 1024;
const ADAPTIVE_BATCH_MID_PAGES: usize = 32;
const ADAPTIVE_BATCH_LOW_PAGES: usize = 16;

const PATH_PYTHON_CANDIDATES: &[&str] = &[
    "python3.11",
    "python3.12",
    "python3.10",
    "python3",
    "python",
];
const DOCLING_HELPER_SCRIPT: &str = r###"
import argparse
import gc
import json
import os
import re
import sys
from importlib import metadata
from pathlib import Path


def _env_int(name, default):
    """Read an int env var, falling back to default on absence/garbage."""
    try:
        return int((os.environ.get(name) or "").strip())
    except (TypeError, ValueError):
        return default


def _env_flag(name, default):
    """Read a bool-ish env var, falling back to `default` on absence/garbage.

    Accepts 1/true/yes/on and 0/false/no/off (case-insensitive). Any other
    value falls back to `default`, so a typo never silently flips behavior.
    """
    raw = (os.environ.get(name) or "").strip().lower()
    if not raw:
        return default
    if raw in ("1", "true", "yes", "on"):
        return True
    if raw in ("0", "false", "no", "off"):
        return False
    return default


def detect_pdf_page_count(pdf_path):
    """Cheaply count PDF pages without running the Docling pipeline.

    Uses pypdfium2 (a Docling dependency) which only parses the page tree, so it
    costs almost no memory. Returns None on any failure, which makes the caller
    fall back to the unchanged single-pass conversion path.
    """
    try:
        import pypdfium2 as pdfium

        pdf = pdfium.PdfDocument(str(pdf_path))
        try:
            return len(pdf)
        finally:
            pdf.close()
    except Exception:  # noqa: BLE001 - unknown count => single-pass fallback
        return None


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    parser.add_argument("--markdown", required=True)
    parser.add_argument("--page-image-root", required=True)
    parser.add_argument("--visual-asset-root", required=True)
    parser.add_argument("--backend-raw-output", required=True)
    parser.add_argument("--metadata-output", required=True)
    parser.add_argument("--summary-output", required=True)
    parser.add_argument("--document-key", required=True)
    return parser.parse_args()


def normalize_text(value):
    if value is None:
        return None
    text = str(value).strip()
    return text or None


def as_posix(path):
    return Path(path).as_posix()


def relative_uri(from_dir, target):
    return Path(os.path.relpath(target, from_dir)).as_posix()


def picture_asset_kind(caption_text):
    lowered = (caption_text or "").lower()
    if "screenshot" in lowered or "screen shot" in lowered:
        return "screenshot"
    if "diagram" in lowered or "block diagram" in lowered:
        return "diagram"
    if "chart" in lowered or "plot" in lowered or "graph" in lowered:
        return "chart"
    return "figure"


def save_json(path, payload):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")


def docling_label_to_kind(label):
    """Map a Docling DocItemLabel to our ContentElementKind string."""
    try:
        # DocItemLabel is a StrEnum; its .value is the canonical lowercase name.
        label_val = label.value if hasattr(label, "value") else str(label)
    except Exception:
        label_val = str(label)
    label_val = label_val.lower().replace("-", "_")
    if "title" in label_val and "sub" not in label_val:
        return "title"
    if "section_header" in label_val or "heading" in label_val:
        return "section_header"
    if "list_item" in label_val or "list-item" in label_val:
        return "list_item"
    if "code" in label_val:
        return "code"
    if "caption" in label_val:
        return "caption"
    if "footnote" in label_val:
        return "footnote"
    if "formula" in label_val:
        return "formula"
    if "page_header" in label_val:
        return "page_header"
    if "page_footer" in label_val:
        return "page_footer"
    if "abstract" in label_val:
        return "abstract"
    return "body_text"


def classify_diagram_kind(caption_text, asset_kind):
    """Classify a visual asset's diagram type from its caption and asset kind.

    Returns one of: timing_diagram, state_machine_diagram, block_diagram,
    register_bitfield, truth_table, flow_chart, unknown.
    """
    lowered = (caption_text or "").lower()
    # Always unknown for table regions (they are handled structurally).
    if asset_kind == "table_region":
        return "unknown"
    # Timing diagram — pass 1: explicit timing/waveform vocabulary.
    if any(kw in lowered for kw in [
        "timing diagram", "timing waveform", "waveform diagram", "waveform",
        "timing", "handshake timing", "clock timing", "signal timing",
        "transfer timing", "cycle timing", "setup and hold",
        "high and low",
        # AMBA-style transfer/burst diagrams (the figure shows a clocked waveform).
        "read transfer", "write transfer",
        "wait state", "waited transfer",
        "wrapping burst", "incrementing burst",
        "undefined length burst",
        "locked transfer",
        "error response",
        "transfer type example",
        "four-beat", "eight-beat", "sixteen-beat",
    ]):
        return "timing_diagram"
    # Timing diagram — pass 2: figures whose captions use protocol execution vocabulary.
    # Bus protocol specs name clocked waveform figures after the operation they depict
    # ("write transaction", "VALID before READY handshake", "exit from reset", …).
    # Any figure—identified by "figure" in the caption—that mentions a transfer,
    # transaction, handshake, or burst operation is treated as a timing waveform.
    # This is intentionally inclusive: the VLM handles borderline cases gracefully;
    # it is worse to discard a real timing diagram than to forward a data-layout one.
    if "figure" in lowered and any(kw in lowered for kw in [
        "transfer",        # write/read/failed/example transfer diagrams
        "transaction",     # AXI uses “transaction” where AHB/APB use “transfer”
        "handshake",       # VALID/READY handshake waveforms
        "burst",           # burst transfer/transaction waveforms
        "exit from reset", # reset de-assertion waveform
        "sequence diagram", # sequential/credit-control visualisations
    ]):
        return "timing_diagram"
    # State machine / state transition diagram.
    if any(kw in lowered for kw in [
        "state machine", "state diagram", "state transition", "transfer state",
        "fsm", "finite state", "states and transitions",
    ]):
        return "state_machine_diagram"
    # Block / architecture / system diagram.
    if any(kw in lowered for kw in [
        "block diagram", "architecture diagram", "system diagram",
        "interconnect", "system block", "component diagram",
        "bus matrix", "top-level", "high-level",
        # AMBA interface/interconnect figures.
        "manager interface", "subordinate interface",
        "multiplexor interconnection", "select signal",
    ]):
        return "block_diagram"
    # Register bit-field layout.
    if any(kw in lowered for kw in [
        "register", "bit field", "bitfield", "register map", "register layout",
    ]):
        return "register_bitfield"
    # Truth table.
    if any(kw in lowered for kw in [
        "truth table", "encoding table", "lookup table",
    ]):
        return "truth_table"
    # Flow chart.
    if any(kw in lowered for kw in [
        "flow chart", "flowchart", "flow diagram", "decision flow",
    ]):
        return "flow_chart"
    return "unknown"


def classify_section(title):
    """Heuristic section kind classification based on the heading title."""
    lowered = title.lower()
    # Boilerplate / legal / admin
    if any(kw in lowered for kw in [
        "licence", "license", "copyright", "proprietary", "trademark",
        "disclaimer", "change history", "revision history", "release note",
        "release information", "acknowledgement", "preface", "foreword",
        "feedback", "about this",
    ]):
        return "boilerplate"
    # Table of contents
    if any(kw in lowered for kw in ["table of contents", "contents"]):
        return "table_of_contents"
    # Glossary / definitions
    if any(kw in lowered for kw in ["glossary", "abbreviation", "acronym", "definition"]):
        return "glossary"
    # Appendix
    if lowered.startswith("appendix") or lowered.startswith("annex"):
        return "appendix"
    # Signal / port description tables
    if any(kw in lowered for kw in [
        "signal", "port", "pin", "interface", "i/o",
    ]):
        return "signal_description"
    # Register / memory maps
    if any(kw in lowered for kw in [
        "register", "memory map", "address map", "configuration", "csr",
    ]):
        return "register_description"
    # Timing sections
    if any(kw in lowered for kw in [
        "timing", "waveform", "clock", "latency", "throughput",
    ]):
        return "timing"
    return "normative"


def classify_table_kind(header_rows, body_rows=None, caption_text=None):
    """Classify a table's purpose from its header cells, body content, and caption.

    Returns one of: signal_description, encoding, register_map,
    timing_parameter, feature_matrix, unknown.

    Classification order:
    1. Caption-based positive: tables whose caption contains "signal" / "signals"
       are interface signal description tables — caption authorship intent is the
       most reliable single signal.
    2. Caption-based exclusion: payload/message-field tables share the
       Name|Width|Description header layout but are not interface signal tables.
    3. Header-based: existing vocabulary checks on column headers.
    4. Content-based encoding detection: when headers lack explicit encoding
       vocabulary, scan the first column of body rows for binary/hex literals
       or bit-field references (SIGNAL[N], SIGNAL[N:M]) — these are the patterns
       that identify value-encoding tables regardless of how their headers are named.
    """
    body = body_rows or []
    if not header_rows and not body:
        return "unknown"
    cap_lower = (caption_text or "").lower()
    # Flatten all header cell texts to lowercase for pattern matching.
    all_headers = [cell["text"].lower() for row in header_rows for cell in row]
    header_set = set(all_headers)
    first_header = all_headers[0] if all_headers else ""

    # ── 1. Caption-based positive: signal / interface table ──────────────────────
    # Chip-design PDFs consistently include "signal" or "signals" in the caption
    # of interface signal tables ("Table 2-1 APB signal descriptions",
    # "Table 2-2 Manager signals", …).  This is more reliable than header vocab.
    cap_words = set(re.split(r'[\s\-_:/]+', cap_lower))
    caption_names_signals = "table" in cap_lower and bool(
        cap_words & {"signal", "signals", "port", "ports", "pin", "pins"}
    )
    # Still exclude payload tables even if they happen to mention "signal".
    caption_is_payload = any(kw in cap_lower for kw in [
        "message field", "message fields",
        "payload field", "payload fields",
        "packet field", "packet fields",
        "command field", "command fields",
        "frame field", "frame fields",
    ])
    if caption_names_signals and not caption_is_payload:
        return "signal_description"

    # ── 2+3. Header-based signal description (with payload exclusion) ────────
    has_name_col = any(kw in first_header for kw in ["name", "signal", "port", "pin"])
    has_width_col = any(any(kw in h for kw in ["width", "bits", "size"]) for h in all_headers)
    has_dir_col = any(any(kw in h for kw in ["direction", "source", "destination"]) for h in all_headers)
    if not caption_is_payload and has_name_col and (has_width_col or has_dir_col):
        return "signal_description"

    # ── Header-based encoding (explicit vocabulary) ───────────────────────
    has_value_col = any(any(kw in h for kw in ["value", "encoding", "code", "binary", "hex"]) for h in all_headers)
    has_meaning_col = any(any(kw in h for kw in ["name", "meaning", "description", "transfer type", "type"]) for h in all_headers)
    if has_value_col and has_meaning_col:
        return "encoding"

    # ── 4. Content-based encoding detection (body scan) ───────────────────
    # Encoding/value tables for individual signal fields often have no explicit
    # "value" or "encoding" column header.  Instead, look at what the first column
    # of body rows actually contains:
    #   • Binary / hex literals  — 0b00, 2'b01, 0x1A  → value encoding table
    #   • Bit-field references   — PPROT[0], HTRANS[1:0]  → bit-field description
    # If at least 2 rows match, treat as encoding.
    if len(body) >= 2:
        first_col = [row[0]["text"].strip() for row in body[:12] if row]
        binary_re = re.compile(r"0b[01]+|[0-9]+'b[01]+|0x[0-9a-fA-F]+")
        bitfield_re = re.compile(r"\w+\[\d+(?::\d+)?\]")
        encoding_hits = sum(
            1 for v in first_col
            if binary_re.search(v) or bitfield_re.search(v)
        )
        if encoding_hits >= 2:
            return "encoding"

    # ── Encoding cross-reference (caption "encoding(s)" or "X in <field>[x]") ──
    # Field-encoding cross-reference tables (e.g. CHI "Table B8.10: Security field
    # encodings for each DVMType") map bit POSITIONS to per-channel fields.  Their
    # first column holds bare positions ("2:0", "3", "4", …) — not SIGNAL[N] refs
    # or literals — so the content scan above misses them; meanwhile a header like
    # "X in REQ.Addr[x] DAT.Data[x]" carries the "addr" substring and a body
    # bit-range, so they would otherwise be mis-read as register maps and emit
    # phantom bit-range-named registers.  Intercept them as encoding here, before
    # the register gate:  the caption ("… encodings …") catches captioned tables,
    # the "X in" cross-reference idiom catches caption-less continuation pages.
    caption_is_encoding = "encoding" in cap_lower and not caption_is_payload
    header_is_xref = any(re.search(r"\bx in\b", h) for h in all_headers)
    if caption_is_encoding or header_is_xref:
        return "encoding"

    # ── Register map (header signal GATED by body-structure) ──────────────
    # A bare address/offset header is NOT sufficient: address-assignment tables,
    # data-frame layouts, tables of contents, feature matrices, and value-encoding
    # tables all carry an "address"/"offset"/"r/w" header without being registers
    # (e.g. I2C "Target address | R/W bit", eMMC RPMB "… Address | Block Count",
    # eMMC TOC "… [177] …", APB "Physical address space").  Require genuine
    # register-field STRUCTURE in the headers or body: a bit RANGE in colon form
    # (7:0, [31:16]) — a single "[177]" page reference is deliberately excluded —
    # or a standalone access-type token (RO/RW/WO/RC/W1C/…) as a whole cell.
    has_addr_col = any(any(kw in h for kw in ["offset", "address", "addr", "base"]) for h in all_headers)
    has_access_col = any(any(kw in h for kw in ["access", "r/w", "rw", "read", "write"]) for h in all_headers)
    bitrange_re = re.compile(r"\[?\d+\s*:\s*\d+\]?")
    access_tokens = {"ro", "rw", "wo", "rc", "rs", "w1c", "w1s", "w0c", "rw1c", "r/w"}
    struct_cells = list(all_headers)
    for row in body[:16]:
        struct_cells += [c.get("text", "").strip() for c in row]
    has_register_structure = (
        any(bitrange_re.search(c) for c in struct_cells)
        or any(c.strip().lower() in access_tokens for c in struct_cells)
    )
    # Some NON-register tables carry genuine bit-ranges or address columns and so
    # pass the structure gate above — but are not register maps.  Exclude the two
    # classes seen in the corpus: tables of contents (dotted-leader cells, e.g.
    # "BOOT_BUS_CONDITIONS [177]....184") and data-frame / packet layouts
    # (frame-specific column vocabulary, e.g. eMMC RPMB "Stuff Bytes | Nonce |
    # Write Counter | Block Count" with body fields "[511:316]").
    is_toc = any("...." in c for c in struct_cells)
    frame_vocab = ["stuff bytes", "nonce", "block count", "(mac)", "write counter"]
    frame_cols = sum(1 for kw in frame_vocab if any(kw in h for h in all_headers))
    is_non_register_layout = is_toc or frame_cols >= 2
    if (
        not is_non_register_layout
        and (has_addr_col or (has_access_col and has_name_col))
        and has_register_structure
    ):
        return "register_map"

    # Timing parameter: min/max/typical + unit columns.
    has_minmax = any(any(kw in h for kw in ["min", "max", "typ", "typical", "maximum", "minimum"]) for h in all_headers)
    has_unit = any(any(kw in h for kw in ["unit", "ns", "ps", "cycles", "period"]) for h in all_headers)
    if has_minmax and (has_unit or "parameter" in header_set or "symbol" in header_set):
        return "timing_parameter"

    # Feature matrix: mandatory/optional/prohibited support levels.
    has_feature_col = any(any(kw in h for kw in ["feature", "property", "capability", "option"]) for h in all_headers)
    has_support_col = any(any(kw in h for kw in ["mandatory", "optional", "prohibited", "required", "supported"]) for h in all_headers)
    if has_feature_col or has_support_col:
        return "feature_matrix"

    return "unknown"


def extract_table_grid(element):
    """Extract header_rows and body_rows from a Docling TableItem."""
    header_rows = []
    body_rows = []
    try:
        if element.data is None:
            return header_rows, body_rows
        grid = element.data.grid
        if not grid:
            return header_rows, body_rows
        for row in grid:
            row_cells = []
            row_is_header = False
            for cell in row:
                cell_text = normalize_text(getattr(cell, "text", "")) or ""
                is_header = bool(
                    getattr(cell, "column_header", False)
                    or getattr(cell, "row_header", False)
                )
                if is_header:
                    row_is_header = True
                row_cells.append({
                    "text": cell_text,
                    "row_span": max(1, getattr(cell, "row_span", 1) or 1),
                    "col_span": max(1, getattr(cell, "col_span", 1) or 1),
                    "is_header": is_header,
                })
            if row_is_header:
                header_rows.append(row_cells)
            else:
                body_rows.append(row_cells)
    except Exception:
        pass
    return header_rows, body_rows


class _IngestAccumulator:
    """Cross-batch accumulators for page-range batched conversion.

    All record lists and id counters live here so single-pass and page-batched
    conversion share one extraction routine and ids/reading-order continue
    monotonically across batches.
    """

    def __init__(self):
        self.page_metadata_records = {}
        self.page_artifacts = []
        self.visual_assets = []
        self.structured_tables = []
        self.content_elements = []
        self.document_sections = []
        self.picture_counter = 0
        self.table_counter = 0
        self.element_reading_order = 0
        self.document_title = None


def process_converted_document(
    doc,
    acc,
    page_image_root,
    visual_asset_root,
    markdown_path,
    backend_raw_output_path,
    save_page_images=True,
):
    """Extract one converted Docling document (the whole doc in single-pass mode,
    or one page-range batch in bounded-memory mode) into the shared accumulators.
    Figure/table region images are always saved to disk here so the heavy image
    data can be freed with the batch. The full-res per-page image is always
    GENERATED (region cropping below crops from it) but only PERSISTED when
    `save_page_images` is set — see the disk-footprint note below. The logic is
    otherwise identical to the historical single-pass extraction; only the
    accumulators are externalized."""
    from docling_core.types.doc import PictureItem, TableItem

    # ── Page artifacts ─────────────────────────────────────────────────────────
    for page in doc.pages.values():
        page_number = int(page.page_no)
        page_id = f"page_{page_number:04d}"
        page_image_path = page_image_root / f"page-{page_number:04d}.png"
        page_metadata_path = page_image_root / f"page-{page_number:04d}.json"

        # Disk-footprint bounding (MEMORY-BOUNDED-INGEST.3): the full-res page
        # image is always generated in memory (the figure/table region crops
        # below read it via element.get_image(doc)), but persisting one PNG per
        # page is O(pages) and writes tens of GB on multi-thousand-page PDFs. No
        # downstream consumer reads the per-page PNG — only figure/table region
        # images are read — so on large docs we skip the write entirely while
        # keeping every region image full-res and intact. The page's full-res
        # dimensions are still recorded, so the image is well-defined for
        # on-demand regeneration if a consumer ever needs it.
        if save_page_images:
            page.image.pil_image.save(page_image_path, format="PNG")
            saved_image_path = as_posix(page_image_path)
        else:
            saved_image_path = None

        page_record = {
            "page_id": page_id,
            "page_number": page_number,
            "size_points": {
                "width": getattr(page.size, "width", None),
                "height": getattr(page.size, "height", None),
            },
            "rendered_image": {
                "path": saved_image_path,
                "width_px": int(round(page.image.size.width)),
                "height_px": int(round(page.image.size.height)),
                "dpi": int(round(page.image.dpi)) if page.image.dpi is not None else None,
            },
            "picture_refs": [],
            "table_refs": [],
        }
        acc.page_metadata_records[page_number] = {
            "record": page_record,
            "metadata_path": page_metadata_path,
        }
        acc.page_artifacts.append(
            {
                "page_id": page_id,
                "page_number": page_number,
                "page_image_path": saved_image_path,
                "layout_metadata_path": as_posix(page_metadata_path),
                "width_px": int(round(page.image.size.width)),
                "height_px": int(round(page.image.size.height)),
            }
        )

    # ── Single-pass element extraction ─────────────────────────────────────────
    # We iterate once and collect all four categories: visual assets, structured
    # table cell grids, typed content elements, and section headings.
    for element, level in doc.iterate_items():
        acc.element_reading_order += 1
        page_number = int(element.prov[0].page_no) if getattr(element, "prov", None) else None
        page_id = f"page_{page_number:04d}" if page_number is not None else None
        source_ref = getattr(element, "self_ref", None)

        if isinstance(element, PictureItem):
            # ── Figure / diagram / chart ───────────────────────────────────────
            acc.picture_counter += 1
            asset_id = f"picture_{acc.picture_counter:04d}"
            asset_path = visual_asset_root / f"picture-{acc.picture_counter:04d}.png"
            element.get_image(doc).save(asset_path, "PNG")
            if element.image is not None:
                element.image.uri = relative_uri(markdown_path.parent, asset_path)
            caption_text = normalize_text(element.caption_text(doc))
            if page_number in acc.page_metadata_records:
                acc.page_metadata_records[page_number]["record"]["picture_refs"].append(source_ref)
            asset_kind_str = picture_asset_kind(caption_text)
            acc.visual_assets.append({
                "asset_id": asset_id,
                "asset_kind": asset_kind_str,
                "page_id": page_id,
                "image_path": as_posix(asset_path),
                "caption_text": caption_text,
                "caption_source_path": as_posix(backend_raw_output_path),
                "source_ref": source_ref,
                "placeholder_text": None,
                "note": None,
                "diagram_kind": classify_diagram_kind(caption_text, asset_kind_str),
            })

        elif isinstance(element, TableItem):
            # ── Table image + structured cell grid ────────────────────────────
            acc.table_counter += 1
            asset_id = f"table_{acc.table_counter:04d}"
            asset_path = visual_asset_root / f"table-{acc.table_counter:04d}.png"
            element.get_image(doc).save(asset_path, "PNG")
            caption_text = normalize_text(element.caption_text(doc))
            if page_number in acc.page_metadata_records:
                acc.page_metadata_records[page_number]["record"]["table_refs"].append(source_ref)
            acc.visual_assets.append({
                "asset_id": asset_id,
                "asset_kind": "table_region",
                "page_id": page_id,
                "image_path": as_posix(asset_path),
                "caption_text": caption_text,
                "caption_source_path": as_posix(backend_raw_output_path),
                "source_ref": source_ref,
                "placeholder_text": None,
                "note": None,
                "diagram_kind": "unknown",
            })
            # Extract the cell grid from the structured table representation.
            header_rows, body_rows = extract_table_grid(element)
            col_count = 0
            try:
                col_count = element.data.num_cols if element.data else 0
            except Exception:
                pass
            table_kind = classify_table_kind(header_rows, body_rows, caption_text)
            acc.structured_tables.append({
                "table_id": asset_id,
                "asset_id": asset_id,
                "page_id": page_id,
                "caption_text": caption_text,
                "source_ref": source_ref,
                "table_kind": table_kind,
                "header_rows": header_rows,
                "body_rows": body_rows,
                "row_count": len(header_rows) + len(body_rows),
                "col_count": col_count,
            })

        else:
            # ── Typed text elements ───────────────────────────────────────────
            label = getattr(element, "label", None)
            if label is None:
                continue
            kind = docling_label_to_kind(label)
            # Skip page headers/footers and unknown non-text elements.
            if kind in ("page_header", "page_footer"):
                continue
            text = normalize_text(getattr(element, "text", None))
            if not text:
                continue
            # Derive heading level: try element.level attribute first, then the
            # iteration level, capping at 6.
            heading_level = None
            if kind == "section_header":
                elem_level = getattr(element, "level", None)
                if elem_level is not None:
                    try:
                        heading_level = max(1, min(6, int(elem_level)))
                    except (TypeError, ValueError):
                        pass
                if heading_level is None and level is not None:
                    try:
                        heading_level = max(1, min(6, int(level)))
                    except (TypeError, ValueError):
                        pass
                if heading_level is None:
                    heading_level = 1

            acc.content_elements.append({
                "element_id": f"elem_{acc.element_reading_order:05d}",
                "kind": kind,
                "text": text,
                "heading_level": heading_level,
                "page_id": page_id,
                "source_ref": source_ref,
                "reading_order": acc.element_reading_order,
            })

            # Capture the first document title.
            if kind == "title" and acc.document_title is None:
                acc.document_title = text

            # Collect section headings as a flat ordered list with classification.
            if kind == "section_header":
                section_idx = len(acc.document_sections) + 1
                # Build a safe section ID from the title text.
                safe_title = re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")[:60]
                acc.document_sections.append({
                    "section_id": f"sec_{section_idx:04d}_{safe_title}",
                    "title": text,
                    "heading_level": heading_level or 1,
                    "page_id": page_id,
                    "source_ref": source_ref,
                    "reading_order": acc.element_reading_order,
                    "section_kind": classify_section(text),
                })


def main():
    args = parse_args()

    try:
        from docling.datamodel.base_models import InputFormat
        from docling.datamodel.pipeline_options import PdfPipelineOptions
        from docling.document_converter import DocumentConverter, PdfFormatOption
        from docling_core.types.doc import ImageRefMode, PictureItem, TableItem
    except ImportError as exc:
        print(f"docling import failed: {exc}", file=sys.stderr)
        return 2

    input_path = Path(args.input)
    markdown_path = Path(args.markdown)
    page_image_root = Path(args.page_image_root)
    visual_asset_root = Path(args.visual_asset_root)
    backend_raw_output_path = Path(args.backend_raw_output)
    metadata_output_path = Path(args.metadata_output)
    summary_output_path = Path(args.summary_output)

    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    page_image_root.mkdir(parents=True, exist_ok=True)
    visual_asset_root.mkdir(parents=True, exist_ok=True)
    backend_raw_output_path.parent.mkdir(parents=True, exist_ok=True)
    metadata_output_path.parent.mkdir(parents=True, exist_ok=True)
    summary_output_path.parent.mkdir(parents=True, exist_ok=True)

    pipeline_options = PdfPipelineOptions()
    pipeline_options.images_scale = 2.0
    pipeline_options.generate_page_images = True
    pipeline_options.generate_picture_images = True

    # Device selection. Docling defaults `accelerator_options.device` to `auto`,
    # which resolves to Apple's MPS backend on Apple Silicon — but torch's MPS
    # cannot perform the float64 ops Docling's layout/table models require, so
    # every page fails ("Cannot convert a MPS Tensor to float64 dtype"). Avoid
    # `auto`: honor an explicit DOCLING_DEVICE (cpu/cuda/mps); otherwise use CUDA
    # when present, else CPU. CUDA hosts keep GPU acceleration; Apple Silicon falls
    # back to correct-but-slower CPU; opt back into MPS via DOCLING_DEVICE=mps on a
    # torch build that supports it. Defensive: any import/version drift keeps the
    # prior default so ingest still runs.
    try:
        from docling.datamodel.accelerator_options import AcceleratorOptions
        import torch

        env_device = os.environ.get("DOCLING_DEVICE")
        if env_device:
            pipeline_options.accelerator_options = AcceleratorOptions(device=env_device)
        else:
            device = "cuda" if torch.cuda.is_available() else "cpu"
            pipeline_options.accelerator_options = AcceleratorOptions(device=device)
    except Exception as exc:  # noqa: BLE001 - keep ingest working on any drift
        print(f"docling device selection fell back to default: {exc}", file=sys.stderr)

    converter = DocumentConverter(
        format_options={
            InputFormat.PDF: PdfFormatOption(pipeline_options=pipeline_options)
        }
    )
    # ── Bounded-memory conversion (MEMORY-BOUNDED-INGEST.1) ─────────────────────
    # Converting a whole large PDF at once holds a full-resolution image for every
    # page in memory simultaneously (peak memory grows with page count), which
    # OOM-kills the backend on big docs. Above a page threshold we convert in
    # bounded page ranges and free each batch, so peak memory is O(batch size).
    # Small docs (the common case, and every current corpus doc <= 500 pages) keep
    # the exact single-pass `convert(path)` call and are byte-identical.
    threshold = _env_int("SPECFORGE_INGEST_BATCH_THRESHOLD", 512)
    batch_pages = max(1, _env_int("SPECFORGE_INGEST_BATCH_PAGES", 64))
    total_pages = detect_pdf_page_count(input_path)
    large_doc = total_pages is not None and total_pages > threshold
    # Disk-footprint bounding (MEMORY-BOUNDED-INGEST.3): a doc large enough to
    # need batched RAM bounding is exactly the doc whose per-page full-res PNGs
    # would blow up disk (O(pages) -> tens of GB). No consumer reads page images
    # (only figure/table region images), so by default we do NOT persist them
    # for large docs. Small docs keep the historical bundle byte-identical.
    # Override explicitly with SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1/0.
    save_page_images = _env_flag("SPECFORGE_INGEST_SAVE_PAGE_IMAGES", not large_doc)
    if large_doc:
        page_batches = [
            (lo, min(lo + batch_pages - 1, total_pages))
            for lo in range(1, total_pages + 1, batch_pages)
        ]
        print(
            f"docling: large PDF ({total_pages} pages) -> converting in "
            f"{len(page_batches)} page-range batch(es) of <= {batch_pages} pages "
            f"to bound peak memory",
            file=sys.stderr,
        )
    else:
        page_batches = [None]
    if not save_page_images:
        print(
            "docling: per-page full-res images are generated in memory for "
            "figure/table region cropping but NOT persisted to disk "
            "(MEMORY-BOUNDED-INGEST.3 disk-footprint bounding: O(assets), not "
            "O(pages)); set SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1 to keep them",
            file=sys.stderr,
        )

    acc = _IngestAccumulator()
    markdown_parts = []
    raw_batches = []
    for _page_batch in page_batches:
        if _page_batch is None:
            result = converter.convert(str(input_path))
        else:
            result = converter.convert(str(input_path), page_range=_page_batch)
        doc = result.document
        process_converted_document(
            doc,
            acc,
            page_image_root,
            visual_asset_root,
            markdown_path,
            backend_raw_output_path,
            save_page_images,
        )
        markdown_parts.append(doc.export_to_markdown(image_mode=ImageRefMode.REFERENCED))
        raw_batches.append(doc.export_to_dict())
        # Free the heavy converted document (per-page + per-figure images) before
        # the next batch so peak memory stays bounded by the batch, not the doc.
        del result
        del doc
        gc.collect()

    page_metadata_records = acc.page_metadata_records
    page_artifacts = acc.page_artifacts
    visual_assets = acc.visual_assets
    structured_tables = acc.structured_tables
    content_elements = acc.content_elements
    document_sections = acc.document_sections
    picture_counter = acc.picture_counter
    table_counter = acc.table_counter
    document_title = acc.document_title

    for page_number in sorted(page_metadata_records):
        entry = page_metadata_records[page_number]
        save_json(entry["metadata_path"], entry["record"])

    # Markdown is a lossy convenience view; concatenate per-batch markdown (a
    # single-pass run yields one part, byte-identical to the historical output).
    markdown_text = "\n\n".join(markdown_parts)
    markdown_path.write_text(markdown_text, encoding="utf-8")
    # The raw backend dict is a provenance pointer only (caption_source_path). A
    # single-pass run writes exactly the historical dict; a batched run writes the
    # per-batch dicts under an envelope so no captured structure is lost.
    if len(raw_batches) == 1:
        save_json(backend_raw_output_path, raw_batches[0])
    else:
        save_json(
            backend_raw_output_path,
            {
                "batched": True,
                "page_batches": [list(b) for b in page_batches],
                "documents": raw_batches,
            },
        )

    docling_version = None
    try:
        docling_version = metadata.version("docling")
    except metadata.PackageNotFoundError:
        docling_version = None

    save_json(
        metadata_output_path,
        {
            "backend_name": "docling",
            "backend_version": docling_version,
            "document_key": args.document_key,
            "input_path": as_posix(input_path),
            "promoted_markdown_path": as_posix(markdown_path),
            "page_count": len(page_artifacts),
            "picture_count": picture_counter,
            "table_count": table_counter,
            "images_scale": 2.0,
            "generate_page_images": True,
            "generate_picture_images": True,
        },
    )
    save_json(
        summary_output_path,
        {
            "backend_version": docling_version,
            "page_artifacts": page_artifacts,
            "visual_assets": visual_assets,
            "structured_tables": structured_tables,
            "content_elements": content_elements,
            "document_sections": document_sections,
            "document_profile": {
                "title": document_title,
                "page_count": len(page_artifacts),
                "table_count": table_counter,
                "figure_count": picture_counter,
                "content_element_count": len(content_elements),
                "section_count": len(document_sections),
            },
            "placeholder_bindings": [],
            "metadata": {
                "page_count": len(page_artifacts),
                "picture_count": picture_counter,
                "table_count": table_counter,
            },
        },
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
"###;

#[derive(Debug, Deserialize)]
pub struct DoclingBackendSummary {
    pub backend_version: Option<String>,
    pub page_artifacts: Vec<PageArtifact>,
    pub visual_assets: Vec<VisualAsset>,
    /// Structured cell-level representation of every table in the document.
    /// Each record links back to its VisualAsset via `asset_id` and carries
    /// the full header/body row grid that Docling extracts natively.
    #[serde(default)]
    pub structured_tables: Vec<StructuredTableRecord>,
    /// Every non-page-header/footer text element with its Docling type label,
    /// reading order position, and page provenance.
    #[serde(default)]
    pub content_elements: Vec<ContentElementRecord>,
    /// Flat ordered list of section headings with heading level and section kind
    /// classification (signal_description, boilerplate, normative, etc.).
    #[serde(default)]
    pub document_sections: Vec<ContentSectionRecord>,
    /// High-level document statistics and document title.
    pub document_profile: Option<DocumentProfile>,
    #[serde(default)]
    pub placeholder_bindings: Vec<PlaceholderBinding>,
    pub metadata: DoclingDocumentMetadata,
}

#[derive(Debug, Deserialize)]
pub struct DoclingDocumentMetadata {
    pub page_count: usize,
    pub picture_count: usize,
    pub table_count: usize,
}

struct BackendCommand {
    display_name: String,
    command: Command,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoclingRuntimeSource {
    EnvironmentOverride,
    RepoLocalVenv,
    PathProbe,
}

impl DoclingRuntimeSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentOverride => "environment_override",
            Self::RepoLocalVenv => "repo_local_venv",
            Self::PathProbe => "path_probe",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoclingRuntimeCandidateStatus {
    Ready,
    MissingCommand,
    ImportFailed,
}

impl DoclingRuntimeCandidateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::MissingCommand => "missing_command",
            Self::ImportFailed => "import_failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoclingRuntimeCandidate {
    pub label: String,
    pub path: PathBuf,
    pub source: DoclingRuntimeSource,
    pub status: DoclingRuntimeCandidateStatus,
    pub detail: Option<String>,
    pub python_version: Option<String>,
    pub docling_version: Option<String>,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoclingRuntimeDiagnosis {
    pub selected_python: Option<PathBuf>,
    pub selected_source: Option<DoclingRuntimeSource>,
    pub selected_label: Option<String>,
    pub selected_python_version: Option<String>,
    pub selected_docling_version: Option<String>,
    pub candidates: Vec<DoclingRuntimeCandidate>,
}

impl DoclingRuntimeDiagnosis {
    pub fn is_ready(&self) -> bool {
        self.selected_python.is_some()
    }

    pub fn resolution(&self) -> String {
        if let Some(candidate) = self
            .candidates
            .iter()
            .find(|candidate| candidate.source == DoclingRuntimeSource::EnvironmentOverride)
        {
            return match candidate.status {
                DoclingRuntimeCandidateStatus::MissingCommand => format!(
                    "`{DOCLING_PYTHON_ENV}` points at `{}` but that command is not available; set it to a working Python interpreter or unset `{DOCLING_PYTHON_ENV}`",
                    candidate.path.display()
                ),
                DoclingRuntimeCandidateStatus::ImportFailed => format!(
                    "`{DOCLING_PYTHON_ENV}` points at `{}` but `import docling` failed there: {}; install `docling` into that interpreter or unset `{DOCLING_PYTHON_ENV}`",
                    candidate.path.display(),
                    candidate
                        .detail
                        .as_deref()
                        .unwrap_or("no error details captured")
                ),
                DoclingRuntimeCandidateStatus::Ready => "docling runtime is ready".to_string(),
            };
        }

        if let Some(candidate) = self
            .candidates
            .iter()
            .find(|candidate| candidate.source == DoclingRuntimeSource::RepoLocalVenv)
        {
            return match candidate.status {
                DoclingRuntimeCandidateStatus::Ready => "docling runtime is ready".to_string(),
                DoclingRuntimeCandidateStatus::MissingCommand => format!(
                    "the repo-local Docling runtime at `{}` is missing; run `bash {DEFAULT_DOCLING_BOOTSTRAP_SCRIPT}` from the repository root to recreate `{DEFAULT_DOCLING_VENV_DIR}`, or set `{DOCLING_PYTHON_ENV}` to a working interpreter",
                    candidate.path.display()
                ),
                DoclingRuntimeCandidateStatus::ImportFailed => format!(
                    "the repo-local Docling runtime at `{}` exists but `import docling` failed there: {}; run `bash {DEFAULT_DOCLING_BOOTSTRAP_SCRIPT}` from the repository root to recreate `{DEFAULT_DOCLING_VENV_DIR}`, or set `{DOCLING_PYTHON_ENV}` to a working interpreter",
                    candidate.path.display(),
                    candidate
                        .detail
                        .as_deref()
                        .unwrap_or("no error details captured")
                ),
            };
        }

        format!(
            "run `bash {DEFAULT_DOCLING_BOOTSTRAP_SCRIPT}` from the repository root to create `{DEFAULT_DOCLING_VENV_DIR}`, or set `{DOCLING_PYTHON_ENV}` to an interpreter where `import docling` succeeds"
        )
    }
}

struct PythonProbe {
    status: DoclingRuntimeCandidateStatus,
    detail: Option<String>,
    python_version: Option<String>,
    docling_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PythonProbePayload {
    ready: bool,
    #[serde(default)]
    python_version: Option<String>,
    #[serde(default)]
    docling_version: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

/// Autonomous host-memory safeguard applied while the Docling subprocess runs.
///
/// `specforge` samples system memory and aborts the ingest cleanly before the host crosses a
/// configurable danger ceiling, directly encoding the owner's "never crash the host" rule into the
/// tool rather than relying on an external shell wrapper.
#[derive(Debug, Clone, Copy)]
struct RamGuardConfig {
    /// Used-memory percentage at or above which the ingest is aborted; `None` disables the guard.
    ceiling_percent: Option<f64>,
    /// Interval between system-memory samples while the subprocess runs.
    sample_interval: Duration,
}

impl RamGuardConfig {
    fn from_env() -> Self {
        Self {
            ceiling_percent: parse_ram_abort_percent(
                env::var(INGEST_RAM_ABORT_PERCENT_ENV).ok().as_deref(),
            ),
            sample_interval: parse_ram_sample_secs(
                env::var(INGEST_RAM_SAMPLE_SECS_ENV).ok().as_deref(),
            ),
        }
    }
}

/// Parse the abort-percentage env value into an active ceiling (`Some`) or a disabled guard
/// (`None`). Absent/empty → default; `off`/`none`/`disabled`/`disable` → disabled; a value outside
/// `(0, 100)` → disabled (a guard that can never or always fire is meaningless); unparseable text
/// falls back to the default rather than silently disabling the safeguard.
fn parse_ram_abort_percent(raw: Option<&str>) -> Option<f64> {
    let Some(value) = raw else {
        return Some(DEFAULT_INGEST_RAM_ABORT_PERCENT);
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Some(DEFAULT_INGEST_RAM_ABORT_PERCENT);
    }
    if matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "off" | "none" | "disabled" | "disable"
    ) {
        return None;
    }
    match trimmed.parse::<f64>() {
        Ok(percent) if percent > 0.0 && percent < 100.0 => Some(percent),
        Ok(_) => None,
        Err(_) => Some(DEFAULT_INGEST_RAM_ABORT_PERCENT),
    }
}

/// Parse the sample-interval env value into a `Duration`, flooring at one second and falling back
/// to the default when absent or unparseable.
fn parse_ram_sample_secs(raw: Option<&str>) -> Duration {
    let secs = raw
        .and_then(|value| value.trim().parse::<u64>().ok())
        .map(|value| value.max(1))
        .unwrap_or(DEFAULT_INGEST_RAM_SAMPLE_SECS);
    Duration::from_secs(secs)
}

/// The abort decision is a single comparison, isolated so it is trivially unit-tested.
fn should_abort_for_memory(used_percent: f64, ceiling_percent: f64) -> bool {
    used_percent >= ceiling_percent
}

/// Resolve a memory breach: returns `(used, ceiling)` only when the guard is active, a reading is
/// available, and the reading is at or above the ceiling. Flattening the three conditions here
/// keeps the spawn/poll loop readable.
fn ram_breach(
    guard: &RamGuardConfig,
    used_percent_fn: &dyn Fn() -> Option<f64>,
) -> Option<(f64, f64)> {
    let ceiling = guard.ceiling_percent?;
    let used = used_percent_fn()?;
    should_abort_for_memory(used, ceiling).then_some((used, ceiling))
}

/// Read the host's current used-memory percentage via the platform's own tool — matching the exact
/// metric the owner monitors — so the guard needs no extra crate dependency. Returns `None` when
/// the reading is unavailable (unsupported OS, or the tool failed), in which case the guard stays
/// inert rather than aborting on missing data.
fn current_used_memory_percent() -> Option<f64> {
    #[cfg(target_os = "macos")]
    {
        read_macos_used_memory_percent()
    }
    #[cfg(target_os = "linux")]
    {
        read_linux_used_memory_percent()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

#[cfg(target_os = "macos")]
fn read_macos_used_memory_percent() -> Option<f64> {
    let output = Command::new("memory_pressure").output().ok()?;
    if !output.status.success() {
        return None;
    }
    parse_macos_memory_pressure_used_percent(&String::from_utf8_lossy(&output.stdout))
}

/// Parse `memory_pressure` output: the "System-wide memory free percentage: 85%" line gives the
/// free percentage; used = 100 − free.
#[cfg(any(target_os = "macos", test))]
fn parse_macos_memory_pressure_used_percent(text: &str) -> Option<f64> {
    for line in text.lines() {
        if !line.to_ascii_lowercase().contains("free percentage") {
            continue;
        }
        let after_colon = line.rsplit(':').next().unwrap_or(line);
        let free = parse_leading_number(after_colon)?;
        if (0.0..=100.0).contains(&free) {
            return Some(100.0 - free);
        }
    }
    None
}

/// Extract the leading numeric token (e.g. `85` from ` 85%`) from a trimmed string.
#[cfg(any(target_os = "macos", test))]
fn parse_leading_number(text: &str) -> Option<f64> {
    let digits: String = text
        .trim()
        .chars()
        .take_while(|character| character.is_ascii_digit() || *character == '.')
        .collect();
    digits.parse::<f64>().ok()
}

#[cfg(target_os = "linux")]
fn read_linux_used_memory_percent() -> Option<f64> {
    parse_linux_meminfo_used_percent(&fs::read_to_string("/proc/meminfo").ok()?)
}

/// Parse `/proc/meminfo`: used% = (1 − MemAvailable/MemTotal) × 100.
#[cfg(any(target_os = "linux", test))]
fn parse_linux_meminfo_used_percent(text: &str) -> Option<f64> {
    let mut mem_total: Option<f64> = None;
    let mut mem_available: Option<f64> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("MemTotal:") {
            mem_total = parse_meminfo_kb(rest);
        } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
            mem_available = parse_meminfo_kb(rest);
        }
    }
    let total = mem_total?;
    let available = mem_available?;
    if total <= 0.0 {
        return None;
    }
    Some(((1.0 - (available / total)) * 100.0).clamp(0.0, 100.0))
}

/// Read the leading kB count from a `/proc/meminfo` value (e.g. `   16384256 kB`).
#[cfg(any(target_os = "linux", test))]
fn parse_meminfo_kb(rest: &str) -> Option<f64> {
    rest.split_whitespace().next()?.parse::<f64>().ok()
}

/// Read the host's TOTAL physical RAM (MB) via the platform's own tool — no new dependency, matching
/// the `.4a` reader's philosophy. `None` on an unsupported OS or a failed read, in which case
/// adaptive batch sizing keeps the ceiling (behave exactly as today on missing data).
fn current_total_memory_mb() -> Option<u64> {
    #[cfg(target_os = "macos")]
    {
        read_macos_total_memory_mb()
    }
    #[cfg(target_os = "linux")]
    {
        read_linux_total_memory_mb()
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

#[cfg(target_os = "macos")]
fn read_macos_total_memory_mb() -> Option<u64> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.memsize")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_sysctl_memsize_bytes(&String::from_utf8_lossy(&output.stdout))
}

/// Parse `sysctl -n hw.memsize` output (total physical memory in bytes) into MB.
#[cfg(any(target_os = "macos", test))]
fn parse_sysctl_memsize_bytes(text: &str) -> Option<u64> {
    let bytes = text.trim().parse::<u64>().ok()?;
    Some(bytes / (1024 * 1024))
}

#[cfg(target_os = "linux")]
fn read_linux_total_memory_mb() -> Option<u64> {
    parse_linux_meminfo_total_mb(&fs::read_to_string("/proc/meminfo").ok()?)
}

/// Parse `/proc/meminfo` `MemTotal:` (kB) into MB.
#[cfg(any(target_os = "linux", test))]
fn parse_linux_meminfo_total_mb(text: &str) -> Option<u64> {
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("MemTotal:") {
            return parse_meminfo_kb(rest).map(|kb| (kb / 1024.0) as u64);
        }
    }
    None
}

/// Parse the adaptive-batch toggle: absent/empty/garbage -> enabled (a typo never silently disables
/// the safeguard); `off`/`none`/`disabled`/`disable` -> disabled.
fn parse_adaptive_batch_enabled(raw: Option<&str>) -> bool {
    !matches!(
        raw.map(|value| value.trim().to_ascii_lowercase())
            .as_deref(),
        Some("off" | "none" | "disabled" | "disable")
    )
}

/// Parse the batch-pages ceiling, defaulting to 64 and requiring `>= 1` (an absent/garbage/zero
/// value yields the historical default), so the ceiling is always a usable batch size.
fn parse_batch_pages_ceiling(raw: Option<&str>) -> usize {
    raw.and_then(|value| value.trim().parse::<usize>().ok())
        .filter(|pages| *pages >= 1)
        .unwrap_or(DEFAULT_INGEST_BATCH_PAGES)
}

/// Resolve the effective page-range batch size from the host's TOTAL physical RAM. Total RAM is a
/// per-machine constant (unlike free memory, which jitters run-to-run and would make cross-batch
/// boundary artifacts non-deterministic — see the `evidence-build-nondeterminism` KM card), so the
/// SAME machine always resolves the SAME batch and re-ingest stays reproducible. Discrete bands keep
/// a 64-page-batch peak (~4.8 GB, the `.2` CHI datum) near ~30 % of RAM. `total_mb == None`
/// (unreadable) -> ceiling, so a host whose RAM cannot be read behaves exactly as today.
fn adaptive_batch_pages(total_mb: Option<u64>, ceiling: usize, floor: usize) -> usize {
    let ceiling = ceiling.max(1);
    let floor = floor.clamp(1, ceiling);
    let Some(total_mb) = total_mb else {
        return ceiling;
    };
    let banded = if total_mb >= ADAPTIVE_BATCH_FULL_CEILING_MIN_MB {
        ceiling
    } else if total_mb >= ADAPTIVE_BATCH_MID_MIN_MB {
        ADAPTIVE_BATCH_MID_PAGES
    } else if total_mb >= ADAPTIVE_BATCH_LOW_MIN_MB {
        ADAPTIVE_BATCH_LOW_PAGES
    } else {
        floor
    };
    banded.clamp(floor, ceiling)
}

/// Resolves the page-range batch size handed to the Docling subprocess before launch.
#[derive(Debug, Clone, Copy)]
struct BatchSizePolicy {
    /// Whether to auto-size the batch down to the host's total RAM (off -> fixed ceiling).
    adaptive: bool,
    /// Upper bound on the batch size (also the value used when adaptive is off / RAM unreadable).
    ceiling_pages: usize,
    /// Lower bound adaptive sizing never shrinks below (clamped to `<=` the ceiling).
    floor_pages: usize,
}

impl BatchSizePolicy {
    fn from_env() -> Self {
        Self {
            adaptive: parse_adaptive_batch_enabled(
                env::var(INGEST_ADAPTIVE_BATCH_ENV).ok().as_deref(),
            ),
            ceiling_pages: parse_batch_pages_ceiling(
                env::var(INGEST_BATCH_PAGES_ENV).ok().as_deref(),
            ),
            floor_pages: DEFAULT_INGEST_MIN_BATCH_PAGES,
        }
    }

    /// The batch size to hand the Docling helper: the ceiling when adaptive is off, else the
    /// total-RAM-banded size from `total_mb_fn` (injected so tests need no real hardware).
    fn effective_pages(self, total_mb_fn: &dyn Fn() -> Option<u64>) -> usize {
        if !self.adaptive {
            return self.ceiling_pages.max(1);
        }
        adaptive_batch_pages(total_mb_fn(), self.ceiling_pages, self.floor_pages)
    }
}

/// Run the backend command under the RAM guard: sample memory once before spawning (don't even
/// launch a heavy ingest if the host is already in danger), then poll the child while sampling
/// memory on the configured cadence, killing the child and returning a typed error on breach.
/// Child stdout/stderr are redirected to files (not pipes) so polling can never deadlock on a full
/// pipe buffer. The memory reader is injected (`used_percent_fn`) so tests exercise every branch
/// without real memory pressure.
fn run_backend_with_ram_guard(
    command: &mut Command,
    display_name: &str,
    guard: &RamGuardConfig,
    stdout_path: &Path,
    stderr_path: &Path,
    used_percent_fn: &dyn Fn() -> Option<f64>,
) -> Result<ExitStatus> {
    if let Some((used, ceiling)) = ram_breach(guard, used_percent_fn) {
        return Err(AppError::IngestAbortedForMemory {
            program: display_name.to_string(),
            used_percent: used,
            ceiling_percent: ceiling,
        });
    }

    command
        .stdout(fs::File::create(stdout_path)?)
        .stderr(fs::File::create(stderr_path)?);
    let mut child = command.spawn()?;

    let mut last_sample = Instant::now();
    loop {
        if let Some(status) = child.try_wait()? {
            return Ok(status);
        }
        if last_sample.elapsed() >= guard.sample_interval {
            last_sample = Instant::now();
            if let Some((used, ceiling)) = ram_breach(guard, used_percent_fn) {
                let _ = child.kill();
                let _ = child.wait();
                return Err(AppError::IngestAbortedForMemory {
                    program: display_name.to_string(),
                    used_percent: used,
                    ceiling_percent: ceiling,
                });
            }
        }
        std::thread::sleep(RAM_GUARD_POLL_INTERVAL);
    }
}

/// Render captured stdout/stderr (written to files by the guarded runner) for an error message,
/// mirroring [`render_command_output`]'s formatting for piped `Output`.
fn render_backend_output_files(stdout_path: &Path, stderr_path: &Path) -> String {
    let stdout = fs::read_to_string(stdout_path)
        .unwrap_or_default()
        .trim()
        .to_string();
    let stderr = fs::read_to_string(stderr_path)
        .unwrap_or_default()
        .trim()
        .to_string();

    match (stdout.is_empty(), stderr.is_empty()) {
        (false, false) => format!("stdout: {stdout}; stderr: {stderr}"),
        (false, true) => format!("stdout: {stdout}"),
        (true, false) => format!("stderr: {stderr}"),
        (true, true) => "no stdout or stderr captured".to_string(),
    }
}

/// How much free disk the ingest pre-flight requires before launching.
///
/// A precise per-document bundle-size estimate is ill-posed pre-ingest (the figure/table asset
/// count is unknown, and the page count is only computed inside the Docling subprocess), so the
/// pre-flight scales off the one cheap pre-ingest signal Rust already has — the **source PDF file
/// size** — with a conservative base headroom, plus the staged-swap as the backstop for the
/// imprecise middle ground. `EstimateFromSource` is the default; an explicit `Floor` overrides it
/// with a fixed MB requirement; `Disabled` turns the check off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskPreflightRequirement {
    Disabled,
    Floor(u64),
    EstimateFromSource,
}

impl DiskPreflightRequirement {
    fn from_env() -> Self {
        parse_disk_preflight_requirement(env::var(INGEST_MIN_FREE_DISK_MB_ENV).ok().as_deref())
    }

    /// The required free MB for this document, or `None` when the check is disabled.
    fn required_mb(self, source_bytes: u64) -> Option<u64> {
        match self {
            Self::Disabled => None,
            Self::Floor(mb) => Some(mb),
            Self::EstimateFromSource => Some(estimate_required_disk_mb(source_bytes)),
        }
    }
}

/// Parse the disk-preflight env value: absent/empty/garbage → estimate from source size (never
/// silently disable on a typo); `off`/`none`/`disabled`/`disable`/`0` → disabled; a positive
/// integer → a fixed MB floor.
fn parse_disk_preflight_requirement(raw: Option<&str>) -> DiskPreflightRequirement {
    let Some(value) = raw else {
        return DiskPreflightRequirement::EstimateFromSource;
    };
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return DiskPreflightRequirement::EstimateFromSource;
    }
    if matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "off" | "none" | "disabled" | "disable"
    ) {
        return DiskPreflightRequirement::Disabled;
    }
    match trimmed.parse::<u64>() {
        Ok(0) => DiskPreflightRequirement::Disabled,
        Ok(mb) => DiskPreflightRequirement::Floor(mb),
        Err(_) => DiskPreflightRequirement::EstimateFromSource,
    }
}

/// Conservative free-disk estimate from the source PDF size: a base headroom (markdown, backend raw
/// JSON, region crops) plus a multiple of the source size. Deliberately an upper-ish bound — the
/// `.3` disk bounding makes a large doc's bundle closer to `O(source)` than the multiplier assumes,
/// so the gate stays conservative without precise per-asset accounting. Saturating throughout.
fn estimate_required_disk_mb(source_bytes: u64) -> u64 {
    let source_mb = source_bytes / (1024 * 1024);
    DEFAULT_INGEST_DISK_BASE_HEADROOM_MB
        .saturating_add(source_mb.saturating_mul(DEFAULT_INGEST_DISK_SIZE_MULTIPLIER))
}

/// Parse `df -P -k` output (POSIX single-line rows: `Filesystem 1024-blocks Used Available Capacity
/// Mounted-on`) and return the Available 1K-blocks (field index 3) of the first data row.
fn parse_df_available_kb(text: &str) -> Option<u64> {
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("Filesystem") {
            continue;
        }
        let fields: Vec<&str> = trimmed.split_whitespace().collect();
        if let Some(available) = fields.get(3)
            && let Ok(kb) = available.parse::<u64>()
        {
            return Some(kb);
        }
    }
    None
}

/// Read free disk (MB) on the filesystem holding `path` via the platform's POSIX `df` (no new
/// dependency). `-P` forces single-line rows; `-k` reports 1K blocks. `None` when `df` is
/// unavailable or unparseable — in which case the pre-flight stays inert rather than refusing on
/// missing data.
fn available_disk_mb(path: &Path) -> Option<u64> {
    let output = Command::new("df")
        .arg("-P")
        .arg("-k")
        .arg(path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_df_available_kb(&String::from_utf8_lossy(&output.stdout)).map(|kb| kb / 1024)
}

/// Walk up from `path` to the nearest ancestor that exists, so `df` has a real target even before
/// the artifact root is created (first ingest). Falls back to the current directory.
fn nearest_existing_ancestor(path: &Path) -> PathBuf {
    let mut current = path;
    loop {
        if current.exists() {
            return current.to_path_buf();
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => return PathBuf::from("."),
        }
    }
}

/// Pure gate: refuse when a free reading is available and below the requirement; stay permissive
/// when free disk is unreadable (`None`) so missing data never blocks a legitimate ingest.
fn check_disk_preflight(target: &Path, required_mb: u64, free_mb: Option<u64>) -> Result<()> {
    match free_mb {
        Some(free) if free < required_mb => Err(AppError::IngestAbortedForDisk {
            path: target.display().to_string(),
            free_mb: free,
            required_mb,
        }),
        _ => Ok(()),
    }
}

/// Pre-flight the ingest's disk need before any staging directory is created (so a refusal touches
/// nothing — the prior bundle is trivially intact). Reads free disk on the artifact root's
/// filesystem and compares against the size-scaled requirement.
fn preflight_ingest_disk(
    artifact_root: &Path,
    source_bytes: u64,
    requirement: DiskPreflightRequirement,
) -> Result<()> {
    let Some(required_mb) = requirement.required_mb(source_bytes) else {
        return Ok(());
    };
    let target = nearest_existing_ancestor(artifact_root);
    let free_mb = available_disk_mb(&target);
    check_disk_preflight(&target, required_mb, free_mb)
}

pub fn materialize_pdf(
    source_path: &Path,
    promoted_markdown_path: &Path,
    metadata_output_path: &Path,
    artifact_layout: &SourceArtifactLayout,
    document_key: &str,
) -> Result<DoclingBackendSummary> {
    // Pre-flight disk BEFORE creating any staging directory, so a refusal leaves the prior
    // normalized bundle and source_ir.json untouched.
    let source_bytes = fs::metadata(source_path)
        .map(|meta| meta.len())
        .unwrap_or(0);
    preflight_ingest_disk(
        &artifact_layout.artifact_root,
        source_bytes,
        DiskPreflightRequirement::from_env(),
    )?;

    fs::create_dir_all(&artifact_layout.artifact_root)?;

    let staged_normalized_root = artifact_layout.artifact_root.join("normalized.staging");
    cleanup_path_if_exists(&staged_normalized_root)?;

    let staged_promoted_markdown_path =
        staged_child_path(&staged_normalized_root, promoted_markdown_path)?;
    let staged_metadata_output_path =
        staged_child_path(&staged_normalized_root, metadata_output_path)?;
    let staged_page_image_root = staged_normalized_root.join("pages");
    let staged_visual_asset_root = staged_normalized_root.join("assets");
    let staged_backend_raw_output_path = staged_child_path(
        &staged_normalized_root,
        &artifact_layout.backend_raw_output_path,
    )?;

    fs::create_dir_all(&staged_normalized_root)?;
    fs::create_dir_all(&staged_page_image_root)?;
    fs::create_dir_all(&staged_visual_asset_root)?;

    let tempdir = tempdir()?;
    let summary_output_path = tempdir.path().join("docling_summary.json");
    let backend_stdout_path = tempdir.path().join("docling_stdout.log");
    let backend_stderr_path = tempdir.path().join("docling_stderr.log");
    let mut backend_command = build_backend_command(tempdir.path())?;
    backend_command
        .command
        .arg("--input")
        .arg(source_path)
        .arg("--markdown")
        .arg(&staged_promoted_markdown_path)
        .arg("--page-image-root")
        .arg(&staged_page_image_root)
        .arg("--visual-asset-root")
        .arg(&staged_visual_asset_root)
        .arg("--backend-raw-output")
        .arg(&staged_backend_raw_output_path)
        .arg("--metadata-output")
        .arg(&staged_metadata_output_path)
        .arg("--summary-output")
        .arg(&summary_output_path)
        .arg("--document-key")
        .arg(document_key);

    // Size the page-range batch to the host (MEMORY-BOUNDED-INGEST.4c). On a small machine the fixed
    // 64-page batch can be too large to convert under the RAM guard, so adaptive sizing lowers it
    // deterministically off TOTAL physical RAM and the child uses that value (the Python helper
    // already reads SPECFORGE_INGEST_BATCH_PAGES, so setting it on the child is the whole wiring). A
    // >= 16 GB host resolves the unchanged 64 ceiling, so its normalized bundle stays byte-identical.
    let batch_pages = BatchSizePolicy::from_env().effective_pages(&current_total_memory_mb);
    backend_command
        .command
        .env(INGEST_BATCH_PAGES_ENV, batch_pages.to_string());

    let guard = RamGuardConfig::from_env();
    let status = run_backend_with_ram_guard(
        &mut backend_command.command,
        &backend_command.display_name,
        &guard,
        &backend_stdout_path,
        &backend_stderr_path,
        &current_used_memory_percent,
    )
    .inspect_err(|_| {
        // The staged-swap means the last-good `normalized/` + `source_ir.json` are untouched;
        // only the in-flight staging tree is discarded so the host is left clean.
        let _ = cleanup_path_if_exists(&staged_normalized_root);
    })?;
    if !status.success() {
        cleanup_path_if_exists(&staged_normalized_root)?;
        return Err(AppError::ExternalCommandFailed {
            program: backend_command.display_name,
            exit_code: status.code(),
            stderr: render_backend_output_files(&backend_stdout_path, &backend_stderr_path),
        });
    }

    let summary_text = fs::read_to_string(&summary_output_path).map_err(|error| {
        AppError::InvalidBackendOutput(format!(
            "docling backend completed without writing a summary manifest at {}: {error}",
            summary_output_path.display()
        ))
    })?;
    let summary =
        serde_json::from_str::<DoclingBackendSummary>(&summary_text).map_err(|error| {
            let _ = cleanup_path_if_exists(&staged_normalized_root);
            AppError::InvalidBackendOutput(format!(
                "failed to parse docling backend summary at {}: {error}",
                summary_output_path.display()
            ))
        })?;

    cleanup_path_if_exists(&artifact_layout.normalized_root)?;
    fs::rename(&staged_normalized_root, &artifact_layout.normalized_root)?;

    Ok(summary.relocate_paths(&staged_normalized_root, &artifact_layout.normalized_root))
}

fn cleanup_path_if_exists(path: &Path) -> std::io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

fn staged_child_path(staged_normalized_root: &Path, final_path: &Path) -> Result<PathBuf> {
    let file_name = final_path.file_name().ok_or_else(|| {
        AppError::InvalidBackendOutput(format!(
            "cannot derive staged artifact path from {}",
            final_path.display()
        ))
    })?;
    Ok(staged_normalized_root.join(file_name))
}

impl DoclingBackendSummary {
    fn relocate_paths(mut self, from_root: &Path, to_root: &Path) -> Self {
        for page_artifact in &mut self.page_artifacts {
            relocate_optional_path(&mut page_artifact.page_image_path, from_root, to_root);
            relocate_optional_path(&mut page_artifact.layout_metadata_path, from_root, to_root);
        }

        for visual_asset in &mut self.visual_assets {
            relocate_optional_path(&mut visual_asset.image_path, from_root, to_root);
            relocate_optional_path(&mut visual_asset.caption_source_path, from_root, to_root);
        }

        for binding in &mut self.placeholder_bindings {
            relocate_path(&mut binding.normalized_source_path, from_root, to_root);
        }

        self
    }
}

fn relocate_optional_path(path: &mut Option<PathBuf>, from_root: &Path, to_root: &Path) {
    if let Some(path_buf) = path {
        relocate_path(path_buf, from_root, to_root);
    }
}

fn relocate_path(path: &mut PathBuf, from_root: &Path, to_root: &Path) {
    if let Ok(relative_path) = path.strip_prefix(from_root) {
        *path = to_root.join(relative_path);
    }
}

fn build_backend_command(tempdir: &Path) -> Result<BackendCommand> {
    if let Some(helper_override) = env::var_os(DOCLING_HELPER_ENV) {
        let helper_path = PathBuf::from(helper_override);
        return Ok(BackendCommand {
            display_name: helper_path.display().to_string(),
            command: Command::new(helper_path),
        });
    }

    let python = resolve_docling_python()?;
    let helper_script_path = tempdir.join("specforge_docling_backend.py");
    fs::write(&helper_script_path, DOCLING_HELPER_SCRIPT)?;

    let mut command = Command::new(&python);
    command.arg(&helper_script_path);

    Ok(BackendCommand {
        display_name: format!("{} {}", python.display(), helper_script_path.display()),
        command,
    })
}

pub fn inspect_docling_runtime() -> Result<DoclingRuntimeDiagnosis> {
    let mut candidates = Vec::new();

    if let Some(runtime_override) = env::var_os(DOCLING_PYTHON_ENV) {
        let candidate = PathBuf::from(runtime_override);
        push_runtime_candidate(
            &mut candidates,
            "env_override".to_string(),
            candidate,
            DoclingRuntimeSource::EnvironmentOverride,
        )?;
        return Ok(finalize_docling_runtime_diagnosis(candidates));
    }

    let repo_local_candidates = discover_repo_local_docling_candidates()?;
    for candidate in repo_local_candidates {
        let label = format!("repo_local:{}", candidate.display());
        push_runtime_candidate(
            &mut candidates,
            label,
            candidate,
            DoclingRuntimeSource::RepoLocalVenv,
        )?;
    }

    if candidates
        .iter()
        .any(|candidate| candidate.source == DoclingRuntimeSource::RepoLocalVenv)
    {
        return Ok(finalize_docling_runtime_diagnosis(candidates));
    }

    for candidate in PATH_PYTHON_CANDIDATES {
        let candidate_path = PathBuf::from(candidate);
        push_runtime_candidate(
            &mut candidates,
            candidate.to_string(),
            candidate_path,
            DoclingRuntimeSource::PathProbe,
        )?;
    }

    Ok(finalize_docling_runtime_diagnosis(candidates))
}

fn resolve_docling_python() -> Result<PathBuf> {
    let diagnosis = inspect_docling_runtime()?;
    let resolution = diagnosis.resolution();
    diagnosis
        .selected_python
        .clone()
        .ok_or(AppError::MissingRuntimeDependency {
            dependency: "docling",
            resolution,
        })
}

fn push_runtime_candidate(
    candidates: &mut Vec<DoclingRuntimeCandidate>,
    label: String,
    path: PathBuf,
    source: DoclingRuntimeSource,
) -> Result<()> {
    let probe = probe_docling_python(&path)?;
    candidates.push(DoclingRuntimeCandidate {
        label,
        path,
        source,
        status: probe.status,
        detail: probe.detail,
        python_version: probe.python_version,
        docling_version: probe.docling_version,
        selected: false,
    });
    Ok(())
}

fn finalize_docling_runtime_diagnosis(
    mut candidates: Vec<DoclingRuntimeCandidate>,
) -> DoclingRuntimeDiagnosis {
    let selected_index = candidates
        .iter()
        .position(|candidate| candidate.status == DoclingRuntimeCandidateStatus::Ready);

    let (
        selected_python,
        selected_source,
        selected_label,
        selected_python_version,
        selected_docling_version,
    ) = if let Some(index) = selected_index {
        candidates[index].selected = true;
        (
            Some(candidates[index].path.clone()),
            Some(candidates[index].source),
            Some(candidates[index].label.clone()),
            candidates[index].python_version.clone(),
            candidates[index].docling_version.clone(),
        )
    } else {
        (None, None, None, None, None)
    };

    DoclingRuntimeDiagnosis {
        selected_python,
        selected_source,
        selected_label,
        selected_python_version,
        selected_docling_version,
        candidates,
    }
}

fn discover_repo_local_docling_candidates() -> Result<Vec<PathBuf>> {
    let cwd = env::current_dir()?;
    let mut candidates = Vec::new();

    for ancestor in cwd.ancestors() {
        for suffix in ["bin/python", "bin/python3", "Scripts/python.exe"] {
            let candidate = ancestor.join(DEFAULT_DOCLING_VENV_DIR).join(suffix);
            if candidate.exists() && !candidates.contains(&candidate) {
                candidates.push(candidate);
            }
        }
    }

    Ok(candidates)
}

fn probe_docling_python(candidate: &Path) -> Result<PythonProbe> {
    match Command::new(candidate)
        .args([
            "-c",
            r#"import json
import platform
from importlib import metadata

payload = {"ready": False, "python_version": platform.python_version()}
try:
    import docling
    payload["ready"] = True
    payload["docling_version"] = metadata.version("docling")
except Exception as exc:
    payload["error"] = f"{type(exc).__name__}: {exc}"

print(json.dumps(payload))"#,
        ])
        .output()
    {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let payload =
                serde_json::from_str::<PythonProbePayload>(&stdout).unwrap_or(PythonProbePayload {
                    ready: false,
                    python_version: None,
                    docling_version: None,
                    error: Some(format!(
                        "invalid probe output from `{}`: {}",
                        candidate.display(),
                        stdout.trim()
                    )),
                });

            Ok(PythonProbe {
                status: if payload.ready {
                    DoclingRuntimeCandidateStatus::Ready
                } else {
                    DoclingRuntimeCandidateStatus::ImportFailed
                },
                detail: payload.error,
                python_version: payload.python_version,
                docling_version: payload.docling_version,
            })
        }
        Ok(output) => Ok(PythonProbe {
            status: DoclingRuntimeCandidateStatus::ImportFailed,
            detail: Some(render_command_output(&output)),
            python_version: None,
            docling_version: None,
        }),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(PythonProbe {
            status: DoclingRuntimeCandidateStatus::MissingCommand,
            detail: None,
            python_version: None,
            docling_version: None,
        }),
        Err(error) => Err(error.into()),
    }
}

fn render_command_output(output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    match (stdout.is_empty(), stderr.is_empty()) {
        (false, false) => format!("stdout: {stdout}; stderr: {stderr}"),
        (false, true) => format!("stdout: {stdout}"),
        (true, false) => format!("stderr: {stderr}"),
        (true, true) => "no stdout or stderr captured".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{Duration, Instant};

    use tempfile::tempdir;

    use super::{
        BatchSizePolicy, DOCLING_PYTHON_ENV, DiskPreflightRequirement,
        DoclingRuntimeCandidateStatus, DoclingRuntimeSource, INGEST_ADAPTIVE_BATCH_ENV,
        INGEST_BATCH_PAGES_ENV, INGEST_MIN_FREE_DISK_MB_ENV, INGEST_RAM_ABORT_PERCENT_ENV,
        INGEST_RAM_SAMPLE_SECS_ENV, RamGuardConfig, adaptive_batch_pages, check_disk_preflight,
        estimate_required_disk_mb, inspect_docling_runtime, nearest_existing_ancestor,
        parse_adaptive_batch_enabled, parse_batch_pages_ceiling, parse_df_available_kb,
        parse_disk_preflight_requirement, parse_leading_number, parse_linux_meminfo_total_mb,
        parse_linux_meminfo_used_percent, parse_macos_memory_pressure_used_percent,
        parse_meminfo_kb, parse_ram_abort_percent, parse_ram_sample_secs,
        parse_sysctl_memsize_bytes, preflight_ingest_disk, run_backend_with_ram_guard,
        should_abort_for_memory,
    };
    use crate::error::{AppError, Result};
    use crate::test_support::env_var_lock;

    struct EnvVarGuard {
        key: &'static str,
        original: Option<std::ffi::OsString>,
    }

    impl EnvVarGuard {
        fn set_path(key: &'static str, value: &Path) -> Self {
            let original = std::env::var_os(key);
            // SAFETY: tests serialize environment mutation with env_var_lock().
            unsafe { std::env::set_var(key, value) };
            Self { key, original }
        }

        fn unset(key: &'static str) -> Self {
            let original = std::env::var_os(key);
            // SAFETY: tests serialize environment mutation with env_var_lock().
            unsafe { std::env::remove_var(key) };
            Self { key, original }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            match self.original.take() {
                Some(value) => {
                    // SAFETY: tests serialize environment mutation with env_var_lock().
                    unsafe { std::env::set_var(self.key, value) };
                }
                None => {
                    // SAFETY: tests serialize environment mutation with env_var_lock().
                    unsafe { std::env::remove_var(self.key) };
                }
            }
        }
    }

    struct CurrentDirGuard {
        original: PathBuf,
    }

    impl CurrentDirGuard {
        fn set(path: &Path) -> Result<Self> {
            let original = std::env::current_dir()?;
            std::env::set_current_dir(path)?;
            Ok(Self { original })
        }
    }

    impl Drop for CurrentDirGuard {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.original);
        }
    }

    #[test]
    fn inspect_docling_runtime_prefers_repo_local_venv() -> Result<()> {
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let repo_python = tempdir
            .path()
            .join(".venv-docling")
            .join("bin")
            .join("python");
        fs::create_dir_all(repo_python.parent().expect("python parent"))?;
        fs::write(
            &repo_python,
            r##"#!/bin/sh
printf '{"ready": true, "python_version": "3.11.9", "docling_version": "2.84.0"}\n'
"##,
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&repo_python, fs::Permissions::from_mode(0o755))?;
        }

        let _docling_python_env = EnvVarGuard::unset(DOCLING_PYTHON_ENV);
        let _path_env = EnvVarGuard::set_path("PATH", Path::new(""));
        let _cwd_guard = CurrentDirGuard::set(tempdir.path())?;

        let diagnosis = inspect_docling_runtime()?;

        assert!(diagnosis.is_ready());
        assert_eq!(
            diagnosis.selected_source,
            Some(DoclingRuntimeSource::RepoLocalVenv)
        );
        assert_eq!(
            diagnosis
                .selected_python
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str()),
            Some("python")
        );
        assert_eq!(
            diagnosis
                .selected_python
                .as_ref()
                .map(|path| path.ends_with(".venv-docling/bin/python")),
            Some(true)
        );
        assert_eq!(
            diagnosis.selected_docling_version.as_deref(),
            Some("2.84.0")
        );

        Ok(())
    }

    #[test]
    fn inspect_docling_runtime_prefers_python311_path_probe_over_generic_python3() -> Result<()> {
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let python311 = tempdir.path().join("python3.11");
        let python3 = tempdir.path().join("python3");
        fs::write(
            &python311,
            r##"#!/bin/sh
printf '{"ready": true, "python_version": "3.11.9", "docling_version": "2.84.0"}\n'
"##,
        )?;
        fs::write(
            &python3,
            r##"#!/bin/sh
printf '{"ready": false, "python_version": "3.14.0", "error": "ModuleNotFoundError: No module named docling"}\n'
"##,
        )?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            fs::set_permissions(&python311, fs::Permissions::from_mode(0o755))?;
            fs::set_permissions(&python3, fs::Permissions::from_mode(0o755))?;
        }

        let _docling_python_env = EnvVarGuard::unset(DOCLING_PYTHON_ENV);
        let _path_env = EnvVarGuard::set_path("PATH", tempdir.path());
        let _cwd_guard = CurrentDirGuard::set(tempdir.path())?;

        let diagnosis = inspect_docling_runtime()?;

        assert!(diagnosis.is_ready());
        assert_eq!(
            diagnosis.selected_source,
            Some(DoclingRuntimeSource::PathProbe)
        );
        assert_eq!(diagnosis.selected_label.as_deref(), Some("python3.11"));
        assert_eq!(
            diagnosis.selected_docling_version.as_deref(),
            Some("2.84.0")
        );
        let python3_probe = diagnosis
            .candidates
            .iter()
            .find(|candidate| candidate.label == "python3")
            .expect("python3 candidate");
        assert_eq!(
            python3_probe.status,
            DoclingRuntimeCandidateStatus::ImportFailed
        );

        Ok(())
    }

    #[test]
    fn parse_ram_abort_percent_handles_default_disable_and_bounds() {
        // Absent / empty / whitespace → owner-policy default.
        assert_eq!(parse_ram_abort_percent(None), Some(85.0));
        assert_eq!(parse_ram_abort_percent(Some("")), Some(85.0));
        assert_eq!(parse_ram_abort_percent(Some("   ")), Some(85.0));
        // Real percentages in (0, 100).
        assert_eq!(parse_ram_abort_percent(Some("90")), Some(90.0));
        assert_eq!(parse_ram_abort_percent(Some(" 72.5 ")), Some(72.5));
        // Explicit disable words.
        assert_eq!(parse_ram_abort_percent(Some("off")), None);
        assert_eq!(parse_ram_abort_percent(Some("NONE")), None);
        assert_eq!(parse_ram_abort_percent(Some("disabled")), None);
        assert_eq!(parse_ram_abort_percent(Some("disable")), None);
        // Out-of-range values disable a guard that could never / always fire.
        assert_eq!(parse_ram_abort_percent(Some("0")), None);
        assert_eq!(parse_ram_abort_percent(Some("100")), None);
        assert_eq!(parse_ram_abort_percent(Some("150")), None);
        assert_eq!(parse_ram_abort_percent(Some("-5")), None);
        // Garbage falls back to the default rather than silently disabling the safeguard.
        assert_eq!(parse_ram_abort_percent(Some("garbage")), Some(85.0));
    }

    #[test]
    fn parse_ram_sample_secs_floors_and_defaults() {
        assert_eq!(parse_ram_sample_secs(None), Duration::from_secs(2));
        assert_eq!(parse_ram_sample_secs(Some("5")), Duration::from_secs(5));
        assert_eq!(parse_ram_sample_secs(Some("0")), Duration::from_secs(1));
        assert_eq!(parse_ram_sample_secs(Some("nope")), Duration::from_secs(2));
    }

    #[test]
    fn should_abort_for_memory_is_at_or_above_ceiling() {
        assert!(should_abort_for_memory(85.0, 85.0));
        assert!(should_abort_for_memory(91.0, 85.0));
        assert!(!should_abort_for_memory(84.9, 85.0));
    }

    #[test]
    fn parses_macos_memory_pressure_free_percentage() {
        assert_eq!(parse_leading_number(" 85%"), Some(85.0));
        assert_eq!(
            parse_macos_memory_pressure_used_percent("System-wide memory free percentage: 85%\n"),
            Some(15.0)
        );
        let verbose = "The system has pages free etc.\n\
             System-wide memory free percentage: 3%\n";
        assert_eq!(
            parse_macos_memory_pressure_used_percent(verbose),
            Some(97.0)
        );
        assert_eq!(
            parse_macos_memory_pressure_used_percent("no percentage line here"),
            None
        );
    }

    #[test]
    fn parses_linux_meminfo_available_fraction() {
        assert_eq!(parse_meminfo_kb("   16384000 kB"), Some(16384000.0));
        let text = "MemTotal:       16384000 kB\n\
             MemFree:         1000000 kB\n\
             MemAvailable:    4096000 kB\n";
        let used = parse_linux_meminfo_used_percent(text).expect("used percent");
        assert!((used - 75.0).abs() < 1e-9, "used was {used}");
        // Missing MemAvailable → no reading rather than a wrong one.
        assert_eq!(parse_linux_meminfo_used_percent("MemTotal: 100 kB\n"), None);
    }

    #[test]
    fn ram_guard_config_reads_env() {
        let _env_lock = env_var_lock();
        {
            let _percent = EnvVarGuard::set_path(INGEST_RAM_ABORT_PERCENT_ENV, Path::new("70"));
            let _secs = EnvVarGuard::set_path(INGEST_RAM_SAMPLE_SECS_ENV, Path::new("5"));
            let config = RamGuardConfig::from_env();
            assert_eq!(config.ceiling_percent, Some(70.0));
            assert_eq!(config.sample_interval, Duration::from_secs(5));
        }
        {
            let _percent = EnvVarGuard::set_path(INGEST_RAM_ABORT_PERCENT_ENV, Path::new("off"));
            let _secs = EnvVarGuard::unset(INGEST_RAM_SAMPLE_SECS_ENV);
            let config = RamGuardConfig::from_env();
            assert_eq!(config.ceiling_percent, None);
            assert_eq!(config.sample_interval, Duration::from_secs(2));
        }
        {
            let _percent = EnvVarGuard::unset(INGEST_RAM_ABORT_PERCENT_ENV);
            let config = RamGuardConfig::from_env();
            assert_eq!(config.ceiling_percent, Some(85.0));
        }
    }

    #[test]
    fn parse_adaptive_batch_enabled_defaults_on_and_honors_disable() {
        // Absent / empty / unknown -> enabled (a typo never silently disables the safeguard).
        assert!(parse_adaptive_batch_enabled(None));
        assert!(parse_adaptive_batch_enabled(Some("")));
        assert!(parse_adaptive_batch_enabled(Some("on")));
        assert!(parse_adaptive_batch_enabled(Some("garbage")));
        // Explicit disable spellings (case/space-insensitive).
        assert!(!parse_adaptive_batch_enabled(Some("off")));
        assert!(!parse_adaptive_batch_enabled(Some("  NONE ")));
        assert!(!parse_adaptive_batch_enabled(Some("disabled")));
        assert!(!parse_adaptive_batch_enabled(Some("disable")));
    }

    #[test]
    fn parse_batch_pages_ceiling_defaults_and_floors() {
        assert_eq!(parse_batch_pages_ceiling(None), 64);
        assert_eq!(parse_batch_pages_ceiling(Some("")), 64);
        assert_eq!(parse_batch_pages_ceiling(Some("garbage")), 64);
        assert_eq!(parse_batch_pages_ceiling(Some("0")), 64);
        assert_eq!(parse_batch_pages_ceiling(Some("  32 ")), 32);
        assert_eq!(parse_batch_pages_ceiling(Some("128")), 128);
    }

    #[test]
    fn adaptive_batch_pages_bands_by_total_ram() {
        // Unreadable RAM keeps the ceiling (permissive — behave as today).
        assert_eq!(adaptive_batch_pages(None, 64, 8), 64);
        // >= 16 GB -> ceiling (the verified path; re-ingest byte-identical).
        assert_eq!(adaptive_batch_pages(Some(24 * 1024), 64, 8), 64);
        assert_eq!(adaptive_batch_pages(Some(16 * 1024), 64, 8), 64);
        // 8-16 GB -> 32; 4-8 GB -> 16; < 4 GB -> floor.
        assert_eq!(adaptive_batch_pages(Some(12 * 1024), 64, 8), 32);
        assert_eq!(adaptive_batch_pages(Some(6 * 1024), 64, 8), 16);
        assert_eq!(adaptive_batch_pages(Some(2 * 1024), 64, 8), 8);
        // A small ceiling caps every band (operator pinned a smaller batch).
        assert_eq!(adaptive_batch_pages(Some(24 * 1024), 16, 8), 16);
        assert_eq!(adaptive_batch_pages(Some(12 * 1024), 16, 8), 16);
        // Floor is clamped to the ceiling so it can never exceed it.
        assert_eq!(adaptive_batch_pages(Some(3 * 1024), 4, 8), 4);
    }

    #[test]
    fn parse_sysctl_memsize_bytes_reads_total_mb() {
        // 24 GiB and 16 GiB in bytes.
        assert_eq!(parse_sysctl_memsize_bytes("25769803776\n"), Some(24 * 1024));
        assert_eq!(
            parse_sysctl_memsize_bytes("  17179869184 "),
            Some(16 * 1024)
        );
        assert_eq!(parse_sysctl_memsize_bytes("not-a-number"), None);
    }

    #[test]
    fn parse_linux_meminfo_total_mb_reads_total() {
        let meminfo =
            "MemTotal:       16384000 kB\nMemFree:          1000 kB\nMemAvailable: 8000000 kB\n";
        // 16384000 kB / 1024 = 16000 MB.
        assert_eq!(parse_linux_meminfo_total_mb(meminfo), Some(16000));
        assert_eq!(parse_linux_meminfo_total_mb("MemFree: 100 kB\n"), None);
    }

    #[test]
    fn batch_size_policy_reads_env() {
        let _env_lock = env_var_lock();
        {
            let _adaptive = EnvVarGuard::unset(INGEST_ADAPTIVE_BATCH_ENV);
            let _ceiling = EnvVarGuard::set_path(INGEST_BATCH_PAGES_ENV, Path::new("48"));
            let policy = BatchSizePolicy::from_env();
            assert!(policy.adaptive);
            assert_eq!(policy.ceiling_pages, 48);
        }
        {
            let _adaptive = EnvVarGuard::set_path(INGEST_ADAPTIVE_BATCH_ENV, Path::new("off"));
            let _ceiling = EnvVarGuard::unset(INGEST_BATCH_PAGES_ENV);
            let policy = BatchSizePolicy::from_env();
            assert!(!policy.adaptive);
            assert_eq!(policy.ceiling_pages, 64);
        }
    }

    #[test]
    fn batch_size_policy_effective_pages_uses_injected_reader() {
        // Adaptive on: ample RAM -> ceiling; small RAM -> smaller; unreadable -> ceiling.
        let ample = BatchSizePolicy {
            adaptive: true,
            ceiling_pages: 64,
            floor_pages: 8,
        };
        assert_eq!(ample.effective_pages(&|| Some(24u64 * 1024)), 64);
        assert_eq!(ample.effective_pages(&|| Some(6u64 * 1024)), 16);
        assert_eq!(ample.effective_pages(&|| None), 64);
        // Adaptive off: always the ceiling regardless of RAM.
        let fixed = BatchSizePolicy {
            adaptive: false,
            ceiling_pages: 64,
            floor_pages: 8,
        };
        assert_eq!(fixed.effective_pages(&|| Some(2u64 * 1024)), 64);
    }

    #[cfg(unix)]
    #[test]
    fn ram_guard_aborts_before_spawn_when_already_over_ceiling() -> Result<()> {
        let tempdir = tempdir()?;
        let stdout_path = tempdir.path().join("out.log");
        let stderr_path = tempdir.path().join("err.log");
        let guard = RamGuardConfig {
            ceiling_percent: Some(85.0),
            sample_interval: Duration::from_millis(0),
        };
        let mut command = Command::new("sleep");
        command.arg("30");
        let reader = || Some(99.0);

        let error = run_backend_with_ram_guard(
            &mut command,
            "sleep 30",
            &guard,
            &stdout_path,
            &stderr_path,
            &reader,
        )
        .expect_err("should abort before spawning a heavy ingest");

        match error {
            AppError::IngestAbortedForMemory {
                used_percent,
                ceiling_percent,
                ..
            } => {
                assert_eq!(used_percent, 99.0);
                assert_eq!(ceiling_percent, 85.0);
            }
            other => panic!("unexpected error: {other:?}"),
        }
        // Never spawned: no child output files were created.
        assert!(!stdout_path.exists());
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn ram_guard_kills_child_when_memory_breaches_mid_run() -> Result<()> {
        // Hold the env lock: spawning by PATH lookup races with tests that set PATH="".
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let stdout_path = tempdir.path().join("out.log");
        let stderr_path = tempdir.path().join("err.log");
        let guard = RamGuardConfig {
            ceiling_percent: Some(85.0),
            sample_interval: Duration::from_millis(0),
        };
        let mut command = Command::new("sleep");
        command.arg("30");
        // Safe before spawn (10%), then a breach (99%) on the first in-flight sample.
        let calls = Cell::new(0u32);
        let reader = || {
            let n = calls.get();
            calls.set(n + 1);
            if n == 0 { Some(10.0) } else { Some(99.0) }
        };

        let start = Instant::now();
        let error = run_backend_with_ram_guard(
            &mut command,
            "sleep 30",
            &guard,
            &stdout_path,
            &stderr_path,
            &reader,
        )
        .expect_err("should kill the child on a mid-run breach");
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "kill should be prompt, took {:?}",
            start.elapsed()
        );

        match error {
            AppError::IngestAbortedForMemory { used_percent, .. } => {
                assert_eq!(used_percent, 99.0)
            }
            other => panic!("unexpected error: {other:?}"),
        }
        Ok(())
    }

    #[test]
    fn parse_disk_preflight_requirement_modes() {
        assert_eq!(
            parse_disk_preflight_requirement(None),
            DiskPreflightRequirement::EstimateFromSource
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some("")),
            DiskPreflightRequirement::EstimateFromSource
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some("   ")),
            DiskPreflightRequirement::EstimateFromSource
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some("off")),
            DiskPreflightRequirement::Disabled
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some("NONE")),
            DiskPreflightRequirement::Disabled
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some("disabled")),
            DiskPreflightRequirement::Disabled
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some("0")),
            DiskPreflightRequirement::Disabled
        );
        assert_eq!(
            parse_disk_preflight_requirement(Some(" 500 ")),
            DiskPreflightRequirement::Floor(500)
        );
        // Garbage estimates from source rather than silently disabling the safeguard.
        assert_eq!(
            parse_disk_preflight_requirement(Some("lots")),
            DiskPreflightRequirement::EstimateFromSource
        );
    }

    #[test]
    fn estimate_required_disk_mb_scales_with_source_size() {
        // Base headroom only when the source is sub-MB.
        assert_eq!(estimate_required_disk_mb(0), 128);
        // 10 MB source → 128 + 10*4.
        assert_eq!(estimate_required_disk_mb(10 * 1024 * 1024), 168);
        // 100 MB source → 128 + 100*4.
        assert_eq!(estimate_required_disk_mb(100 * 1024 * 1024), 528);
    }

    #[test]
    fn disk_preflight_requirement_resolves_required_mb() {
        assert_eq!(DiskPreflightRequirement::Disabled.required_mb(999), None);
        assert_eq!(
            DiskPreflightRequirement::Floor(750).required_mb(10 * 1024 * 1024),
            Some(750)
        );
        assert_eq!(
            DiskPreflightRequirement::EstimateFromSource.required_mb(0),
            Some(128)
        );
    }

    #[test]
    fn parse_df_available_kb_reads_posix_rows() {
        let macos = "Filesystem 1024-blocks      Used Available Capacity  Mounted on\n\
             /dev/disk1s1 488245288 123456789 360000000      26%    /\n";
        assert_eq!(parse_df_available_kb(macos), Some(360_000_000));
        let linux = "Filesystem     1024-blocks      Used Available Capacity Mounted on\n\
             /dev/sda1         41251136  12345678  26805458      32% /\n";
        assert_eq!(parse_df_available_kb(linux), Some(26_805_458));
        // Header only / nonsense → no reading.
        assert_eq!(
            parse_df_available_kb("Filesystem 1024-blocks Used Available Capacity Mounted on\n"),
            None
        );
        assert_eq!(parse_df_available_kb("not df output"), None);
    }

    #[test]
    fn check_disk_preflight_refuses_only_below_requirement() {
        let path = Path::new("/tmp");
        // Below requirement → typed refusal carrying the numbers.
        match check_disk_preflight(path, 500, Some(100)) {
            Err(AppError::IngestAbortedForDisk {
                free_mb,
                required_mb,
                ..
            }) => {
                assert_eq!(free_mb, 100);
                assert_eq!(required_mb, 500);
            }
            other => panic!("expected disk refusal, got {other:?}"),
        }
        // At/above requirement → ok.
        assert!(check_disk_preflight(path, 500, Some(500)).is_ok());
        assert!(check_disk_preflight(path, 500, Some(900)).is_ok());
        // Unreadable free disk → permissive (never refuse on missing data).
        assert!(check_disk_preflight(path, 500, None).is_ok());
    }

    #[test]
    fn nearest_existing_ancestor_walks_up_to_a_real_dir() -> Result<()> {
        let tempdir = tempdir()?;
        let missing = tempdir.path().join("a").join("b").join("c");
        assert_eq!(nearest_existing_ancestor(&missing), tempdir.path());
        assert_eq!(nearest_existing_ancestor(tempdir.path()), tempdir.path());
        Ok(())
    }

    #[test]
    fn disk_preflight_requirement_reads_env() {
        let _env_lock = env_var_lock();
        {
            let _floor = EnvVarGuard::set_path(INGEST_MIN_FREE_DISK_MB_ENV, Path::new("750"));
            assert_eq!(
                DiskPreflightRequirement::from_env(),
                DiskPreflightRequirement::Floor(750)
            );
        }
        {
            let _off = EnvVarGuard::set_path(INGEST_MIN_FREE_DISK_MB_ENV, Path::new("off"));
            assert_eq!(
                DiskPreflightRequirement::from_env(),
                DiskPreflightRequirement::Disabled
            );
        }
        {
            let _unset = EnvVarGuard::unset(INGEST_MIN_FREE_DISK_MB_ENV);
            assert_eq!(
                DiskPreflightRequirement::from_env(),
                DiskPreflightRequirement::EstimateFromSource
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn preflight_ingest_disk_refuses_impossible_floor_and_allows_disabled() -> Result<()> {
        // Hold the env lock: `df` is spawned by PATH lookup, which races with tests that set PATH="".
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        // A floor no real filesystem can satisfy → refuse, with the prior bundle untouched (nothing
        // was created — the check runs before any staging).
        match preflight_ingest_disk(tempdir.path(), 0, DiskPreflightRequirement::Floor(u64::MAX)) {
            Err(AppError::IngestAbortedForDisk { required_mb, .. }) => {
                assert_eq!(required_mb, u64::MAX)
            }
            other => panic!("expected disk refusal, got {other:?}"),
        }
        // Disabled never refuses regardless of free space.
        assert!(
            preflight_ingest_disk(tempdir.path(), 0, DiskPreflightRequirement::Disabled).is_ok()
        );
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn ram_guard_returns_status_when_child_completes() -> Result<()> {
        // Hold the env lock: spawning by PATH lookup races with tests that set PATH="".
        let _env_lock = env_var_lock();
        let tempdir = tempdir()?;
        let stdout_path = tempdir.path().join("out.log");
        let stderr_path = tempdir.path().join("err.log");
        let guard = RamGuardConfig {
            ceiling_percent: Some(85.0),
            sample_interval: Duration::from_millis(0),
        };
        let mut command = Command::new("true");
        let reader = || Some(10.0);

        let status = run_backend_with_ram_guard(
            &mut command,
            "true",
            &guard,
            &stdout_path,
            &stderr_path,
            &reader,
        )?;

        assert!(status.success());
        assert!(stdout_path.exists());
        Ok(())
    }
}
