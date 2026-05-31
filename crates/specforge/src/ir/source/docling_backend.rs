use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

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
const PATH_PYTHON_CANDIDATES: &[&str] = &[
    "python3.11",
    "python3.12",
    "python3.10",
    "python3",
    "python",
];
const DOCLING_HELPER_SCRIPT: &str = r###"
import argparse
import json
import os
import re
import sys
from importlib import metadata
from pathlib import Path


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
    if (has_addr_col or (has_access_col and has_name_col)) and has_register_structure:
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

    converter = DocumentConverter(
        format_options={
            InputFormat.PDF: PdfFormatOption(pipeline_options=pipeline_options)
        }
    )
    result = converter.convert(str(input_path))
    doc = result.document

    # ── Page artifacts ─────────────────────────────────────────────────────────
    page_metadata_records = {}
    page_artifacts = []
    for page in doc.pages.values():
        page_number = int(page.page_no)
        page_id = f"page_{page_number:04d}"
        page_image_path = page_image_root / f"page-{page_number:04d}.png"
        page_metadata_path = page_image_root / f"page-{page_number:04d}.json"

        page.image.pil_image.save(page_image_path, format="PNG")

        page_record = {
            "page_id": page_id,
            "page_number": page_number,
            "size_points": {
                "width": getattr(page.size, "width", None),
                "height": getattr(page.size, "height", None),
            },
            "rendered_image": {
                "path": as_posix(page_image_path),
                "width_px": int(round(page.image.size.width)),
                "height_px": int(round(page.image.size.height)),
                "dpi": int(round(page.image.dpi)) if page.image.dpi is not None else None,
            },
            "picture_refs": [],
            "table_refs": [],
        }
        page_metadata_records[page_number] = {
            "record": page_record,
            "metadata_path": page_metadata_path,
        }
        page_artifacts.append(
            {
                "page_id": page_id,
                "page_number": page_number,
                "page_image_path": as_posix(page_image_path),
                "layout_metadata_path": as_posix(page_metadata_path),
                "width_px": int(round(page.image.size.width)),
                "height_px": int(round(page.image.size.height)),
            }
        )

    # ── Single-pass element extraction ─────────────────────────────────────────
    # We iterate once and collect all four categories: visual assets, structured
    # table cell grids, typed content elements, and section headings.
    visual_assets = []
    structured_tables = []
    content_elements = []
    document_sections = []
    picture_counter = 0
    table_counter = 0
    element_reading_order = 0
    document_title = None

    for element, level in doc.iterate_items():
        element_reading_order += 1
        page_number = int(element.prov[0].page_no) if getattr(element, "prov", None) else None
        page_id = f"page_{page_number:04d}" if page_number is not None else None
        source_ref = getattr(element, "self_ref", None)

        if isinstance(element, PictureItem):
            # ── Figure / diagram / chart ───────────────────────────────────────
            picture_counter += 1
            asset_id = f"picture_{picture_counter:04d}"
            asset_path = visual_asset_root / f"picture-{picture_counter:04d}.png"
            element.get_image(doc).save(asset_path, "PNG")
            if element.image is not None:
                element.image.uri = relative_uri(markdown_path.parent, asset_path)
            caption_text = normalize_text(element.caption_text(doc))
            if page_number in page_metadata_records:
                page_metadata_records[page_number]["record"]["picture_refs"].append(source_ref)
            asset_kind_str = picture_asset_kind(caption_text)
            visual_assets.append({
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
            table_counter += 1
            asset_id = f"table_{table_counter:04d}"
            asset_path = visual_asset_root / f"table-{table_counter:04d}.png"
            element.get_image(doc).save(asset_path, "PNG")
            caption_text = normalize_text(element.caption_text(doc))
            if page_number in page_metadata_records:
                page_metadata_records[page_number]["record"]["table_refs"].append(source_ref)
            visual_assets.append({
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
            structured_tables.append({
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

            content_elements.append({
                "element_id": f"elem_{element_reading_order:05d}",
                "kind": kind,
                "text": text,
                "heading_level": heading_level,
                "page_id": page_id,
                "source_ref": source_ref,
                "reading_order": element_reading_order,
            })

            # Capture the first document title.
            if kind == "title" and document_title is None:
                document_title = text

            # Collect section headings as a flat ordered list with classification.
            if kind == "section_header":
                section_idx = len(document_sections) + 1
                # Build a safe section ID from the title text.
                safe_title = re.sub(r"[^a-z0-9]+", "_", text.lower()).strip("_")[:60]
                document_sections.append({
                    "section_id": f"sec_{section_idx:04d}_{safe_title}",
                    "title": text,
                    "heading_level": heading_level or 1,
                    "page_id": page_id,
                    "source_ref": source_ref,
                    "reading_order": element_reading_order,
                    "section_kind": classify_section(text),
                })

    for page_number in sorted(page_metadata_records):
        entry = page_metadata_records[page_number]
        save_json(entry["metadata_path"], entry["record"])

    markdown_text = doc.export_to_markdown(image_mode=ImageRefMode.REFERENCED)
    markdown_path.write_text(markdown_text, encoding="utf-8")
    save_json(backend_raw_output_path, doc.export_to_dict())

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

pub fn materialize_pdf(
    source_path: &Path,
    promoted_markdown_path: &Path,
    metadata_output_path: &Path,
    artifact_layout: &SourceArtifactLayout,
    document_key: &str,
) -> Result<DoclingBackendSummary> {
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

    let output = backend_command.command.output()?;
    if !output.status.success() {
        cleanup_path_if_exists(&staged_normalized_root)?;
        return Err(AppError::ExternalCommandFailed {
            program: backend_command.display_name,
            exit_code: output.status.code(),
            stderr: render_command_output(&output),
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
    use std::fs;
    use std::path::{Path, PathBuf};

    use tempfile::tempdir;

    use super::{
        DOCLING_PYTHON_ENV, DoclingRuntimeCandidateStatus, DoclingRuntimeSource,
        inspect_docling_runtime,
    };
    use crate::error::Result;
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
}
