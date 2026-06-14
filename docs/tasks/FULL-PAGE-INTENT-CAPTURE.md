# FULL-PAGE-INTENT-CAPTURE: use the full scope of a page's visual information

## Metadata

- Tree ID: `FULL-PAGE-INTENT-CAPTURE`
- Status: `active`
- Roadmap lane: `R16`/`R15e` (intent capture / completeness)
- Created: `2026-06-14`
- Last updated: `2026-06-14`
- Owner: repo-local workflow

## Goal

Answer — with measured evidence first, then (only if warranted) a bounded, grounded implementation —
the owner's question (`2026-06-14`): **does SpecForge use the full scope of a page's available
information, or is intent-bearing content slipping through because nothing reads the full page?**

Today a page is ingested via two paths only: (a) Docling's structured **text / table / section**
elements, and (b) **figure/table region crops** (`VisualAsset.image_path`) fed to the VLM arm
(`enrich`, `audit-extraction`, `recover-register-bits`). The per-page **full-page raster**
(`page-NNNN.png`) is read by no consumer (KM `page-image-disk-bounding`); `MEMORY-BOUNDED-INGEST.3`
therefore skips persisting it for large docs **without reducing captured information** (region crops
stay full-res; the page raster is only an intermediate). The honest gap this tree investigates:
**content that Docling does NOT segment into a text element OR a figure/table region** — an unlabeled
inline drawing, a misclassified diagram, sparse margin/annotation content, a watermark-obscured
region — currently has no capture path and is an honest residual/miss
(`[[project_intent_completeness_research]]`).

The deliverable of the FIRST leaf is **measurement, not code**: quantify how much intent-bearing page
content escapes both existing paths on the *persisted corpus*, so the decision to build (or not) a
whole-page capture path is evidence-led — matching the project's "probe over all persisted docs
BEFORE coding" method (`PDF-VARIANT-DIGESTION.9.x`).

## Non-Goals

- **No fabrication.** Whatever path is (or is not) built, content that is genuinely not present, or
  not legible, stays an honest residual — never a guessed signal/width/field/name (the standing
  honesty guardrail; `[[feedback_scoring_rigor]]`).
- **No genericity regression (ADR 0006).** Any capture path must work on ANY chip-spec PDF — no
  per-PDF vocabulary, no name lists; the model proposes, the document's own declared surfaces decide.
- **No determinism regression.** A whole-page VLM read is non-deterministic; if built, its output
  must be grounded/gated exactly like the existing VLM arm (advisory → typed surface only when the
  document corroborates it), so canonical IR stays reproducible (`[[evidence-build-nondeterminism]]`).
- **No resource regression.** Any new vision work must respect `MEMORY-BOUNDED-INGEST` (RAM/disk
  bounded, never crash the host; whole-page VLM on a 930p doc is exactly the size-immunity frontier).
- **Not the wire-based docs' problem.** APB/AHB/AXI/SWD are at 100% via structured surfaces; this
  tree must not perturb them (they carry no un-segmented intent that the structured path misses).

## Acceptance Criteria

- `.1` produces a measured, per-document gap report over the persisted corpus: for each doc, how much
  page content falls outside BOTH the structured-element path and the region-crop path, with a bounded
  human-review sample, written to a tracked report (`docs/research/…`) — read-only, no extraction
  change, deterministic.
- The report yields an explicit, evidence-backed GO / NO-GO recommendation for a whole-page capture
  path (and, if GO, the grounded+bounded mechanism sketch as `.2`).
- Focused checks pass; the probe touches no canonical IR; live docs/book updated where state changed;
  committed per `COMMIT.md`.

## Task Tree

- ID: `FULL-PAGE-INTENT-CAPTURE`
  Status: `active`
  Goal: use the full scope of a page's visual information, evidence-led, without regressing honesty /
  genericity / determinism / resource bounds
  Children: `.1` (probe), `.2` (mechanism design — gated on `.1`)

- ID: `FULL-PAGE-INTENT-CAPTURE.1`
  Status: `in_progress`
  Goal: **probe-first gap measurement (NO code change to the pipeline).** Over the persisted
  `generated/source_ir/*` corpus, measure per-document how much page content is captured by NEITHER
  the structured text/table/section elements NOR the figure/table region crops. Candidate signals
  (read-only): per-page count/area of Docling-segmented regions vs. the full page; pages with high
  visual density (figures/diagrams) but low captured-element coverage; pages where a region was
  classified `unknown`/dropped; cross-check a bounded sample of pages by rendering the full-page image
  (re-ingest one or two docs with `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1`) and eyeballing what the
  structured + region surfaces missed. Output a tracked report
  (`docs/research/full-page-capture-gap.md`) with per-doc numbers, a bounded review sample, and a
  GO/NO-GO recommendation. Honest framing: if the gap is negligible (Docling already segments
  everything intent-bearing), the honest answer is "SpecForge already uses the full scope" and the
  tree closes NO-GO — that is a valid, valuable outcome.
  Acceptance: the report exists with per-document measured numbers + sample + recommendation; no
  pipeline/IR code changed; deterministic (same corpus → same report); wire-based docs untouched.
  Verification: `pending`
  Commit: `pending`

- ID: `FULL-PAGE-INTENT-CAPTURE.2`
  Status: `proposed`
  Goal: (GATED on a `.1` GO) design the grounded, bounded, deterministic whole-page capture mechanism
  — likely a whole-page VLM read whose proposals are gated against the document's own declared
  surfaces (ADR 0006), emitted as a typed evidence surface with honest residuals, respecting the
  `MEMORY-BOUNDED-INGEST` resource bounds and reusing `SPECFORGE_INGEST_SAVE_PAGE_IMAGES` / the `.3b`
  on-demand render to obtain the page raster only when needed.
  Acceptance: a design recorded in this tree with the grounding gate, the resource plan, and the
  no-regression gates (wire-based 100%, determinism, genericity) before any code.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FULL-PAGE-INTENT-CAPTURE.1` | `in_progress` | **ACTIVE** — recon done (no bboxes in persisted summary → two-stage method decided); next = run the count/density proxy corpus-wide, then bbox-area + eyeball on the flagged subset → GO/NO-GO report |
| 2 | `FULL-PAGE-INTENT-CAPTURE.2` | `proposed` | mechanism design, GATED on a `.1` GO |

## Decisions

- `2026-06-14` (`.1` recon): **The persisted SourceIR summary carries NO bounding boxes** — its
  `page_artifacts` (page_id/number/dims), `visual_assets` (kind/page_id/caption/diagram_kind),
  `content_elements` (kind/text/page_id/reading_order), and `structured_tables` records expose only
  counts, page-ids, and reading order; no element/region coordinates. So true page-*area* coverage is
  NOT computable from the persisted summary alone. The Docling raw `*.backend.json` (`export_to_dict`)
  DOES retain element `prov` bounding boxes, but those files are large (CHI's normalized bundle was
  528 MB), so a blind corpus-wide raw parse would stress RAM (`[[feedback_ram_ceiling_monitor]]`).
  **Chosen `.1` method (resource-bounded, two-stage):** (1) a CHEAP count/density proxy over all 79
  persisted SourceIR docs — per page, captured `content_elements` + `visual_assets`, flagging pages
  with visual density (figures/diagrams) but sparse captured elements, or `unknown`-`diagram_kind`
  regions — to surface CANDIDATE under-captured pages corpus-wide; then (2) the rigorous bbox-area
  "uncovered page area" measurement from the raw `prov` boxes + a bounded human eyeball
  (re-ingest one or two flagged docs with `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1`) on the FLAGGED
  SUBSET only. Corpus 79 source_ir docs persisted as of this recon.
- `2026-06-14`: **Probe-first, not build-first.** Owner asked (after `MEMORY-BOUNDED-INGEST.4c`) to
  scope whether SpecForge uses the full scope of a page's information. The disciplined answer is to
  MEASURE the gap on the persisted corpus before committing to a whole-page VLM path — a NO-GO
  ("Docling already segments everything intent-bearing") is a valid outcome that avoids building
  speculative machinery. Matches the `PDF-VARIANT-DIGESTION.9.x` method.
- `2026-06-14`: **`.3`/`.4c` do not cause the gap.** Skipping page-PNG persistence (`.3`) only drops
  an artifact no consumer reads — region crops stay full-res and ARE used; adaptive batch sizing
  (`.4c`) only changes batch size, not what is captured. The gap (if any) is upstream of both: it is
  whatever Docling fails to segment into an element or a region, which a whole-page read could recover.

## Open Questions

- Is the residual gap material on real chip-spec PDFs, or does Docling already segment essentially all
  intent-bearing content? (This is exactly what `.1` measures — it does not block the frontier.)
- If GO: whole-page VLM vs. better/secondary segmentation — which is the more grounded, deterministic,
  resource-bounded path? (Owner-steerable; decided in `.2` off the `.1` evidence.)

## Blockers

- None. (`.2` is gated on the `.1` GO/NO-GO outcome.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-14` | `FULL-PAGE-INTENT-CAPTURE.1` | `pending` | `pending` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FULL-PAGE-INTENT-CAPTURE.1` | `pending` | `pending` |

## Changelog

- `2026-06-14`: Created task tree (owner-directed, `2026-06-14`, after `MEMORY-BOUNDED-INGEST.4c`):
  scope whether SpecForge uses the full scope of a page's visual information. Probe-first `.1` (measure
  the gap on the persisted corpus, no code) → `.2` mechanism design gated on a GO.
