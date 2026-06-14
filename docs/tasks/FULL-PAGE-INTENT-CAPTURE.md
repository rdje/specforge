# FULL-PAGE-INTENT-CAPTURE: use the full scope of a page's visual information

## Metadata

- Tree ID: `FULL-PAGE-INTENT-CAPTURE`
- Status: `done` (CLOSED `2026-06-14`, **NO-GO** — `.1` measured the gap; SpecForge already
  uses the full scope of a page's intent-bearing visual information; `.2` stays unbuilt)
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
  Status: `done` (`2026-06-14`, **NO-GO**) — report `docs/research/full-page-capture-gap.md`
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
  Verification: `done` — `docs/research/full-page-capture-gap.md` landed with corpus-wide Stage 1
  numbers (79 docs / 14,762 pages: 170 fully-blank = 1.15%, 16,180 "unknown" crops = NOT a gap),
  rigorous Stage 2 ink-outside-bbox over 16 backend docs (dark px-weighted-outside ≤5% worst-case,
  ≈0 typical), and a 5-page eyeball sample — every "escape" resolves to decoration/shading/furniture
  /blank-divider. NO pipeline/IR code touched; deterministic; probe scripts throwaway (/tmp only).
  Recommendation: **NO-GO**. `scripts/check_memory_architecture.sh` + mdBook build green.
  Commit: `FULL-PAGE-INTENT-CAPTURE.1 — measured gap report (NO-GO)`

- ID: `FULL-PAGE-INTENT-CAPTURE.2`
  Status: `not_started` (GATED on a `.1` GO — `.1` returned **NO-GO**, so `.2` stays unbuilt;
  re-open only if a future doc class empirically shows dark content escaping all boxes on a real page)
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
| — | `FULL-PAGE-INTENT-CAPTURE.1` | `done` (**NO-GO**) | report `docs/research/full-page-capture-gap.md` landed; measured the gap corpus-wide + pixel-level — no material intent-bearing content escapes both paths |
| — | `FULL-PAGE-INTENT-CAPTURE.2` | `not_started` | GATED on a `.1` GO; `.1` returned NO-GO → unbuilt. **Tree CLOSED.** |

**TREE CLOSED `2026-06-14` (NO-GO).** The owner's question is answered with measured evidence:
SpecForge already uses the full scope of a page's intent-bearing visual information. The honest
residual outside Docling's segmented regions is decoration (heading rules, page borders, table
grid lines), admonition shading, header/footer furniture, and blank divider pages — not lost
intent. Building a whole-page VLM pass would trade non-determinism + RAM/disk cost for ≈zero
recoverable intent.

## Decisions

- `2026-06-14` (`.1` result, **NO-GO**): the rigorous measurement landed. **Stage 1** over all 79
  persisted docs (14,762 pages): only **170 pages (1.15%) are fully-blank** to both summary paths,
  and the 16,180 "unknown `diagram_kind`" crops are NOT a gap (the region IS cropped + VLM-fed; the
  kind just defaults to unknown without a caption). **Stage 2** ink-outside-bbox over 16
  backend-available docs (raster decoded from the embedded base64; bboxes from raw `prov`): the
  **dark-ink (<150) px-weighted-outside aggregate is ≤5% worst-case (AHB/ARM-Debug/AXI+ACE) and ≈0
  typical** (RISC-V-Debug 0.0000, AXI 0.0002, LTI 0.0005, I2C 0.0006, CAN 0.001). The large
  *all-ink* numbers (CAN mean 0.146, LTI max 0.420) are entirely light-gray admonition shading +
  decorative borders/rules + furniture, which vanish under the dark threshold. **5-page eyeball**
  (LTI p49, CAN p72, SWP p7, APB_d p12, plus the fully-blank divider class) confirms every escape is
  decoration/shading/furniture/blank-page — never content. The LTI "Note" callout body text is a
  captured `body_text` element (only the gray box fill was outside boxes). **Honest limits:** CHI's
  raster is absent (bounded-ingest path doesn't embed it — content still captured as elements/tables);
  62 docs lack a backend JSON so the pixel stage covered 16, but those 16 include the two highest
  fully-blank-count docs with rasters (AXI+ACE 27, ARM-Debug 15) and all three classes. RAM stayed
  ≥48% free (kill line never approached). Report: `docs/research/full-page-capture-gap.md`.
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

- None. (`.2` was gated on the `.1` GO/NO-GO outcome; `.1` returned NO-GO → tree closed.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-14` | `FULL-PAGE-INTENT-CAPTURE.1` | read-only probe (no code); `scripts/check_memory_architecture.sh`; mdBook build | **done** — report landed, NO-GO; deterministic; corpus untouched; RAM ≥48% free throughout |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FULL-PAGE-INTENT-CAPTURE.1` | `FULL-PAGE-INTENT-CAPTURE.1 — measured gap report (NO-GO)` | docs-only; report + tree + live docs + book note; no pipeline/IR code |

## Changelog

- `2026-06-14`: Created task tree (owner-directed, `2026-06-14`, after `MEMORY-BOUNDED-INGEST.4c`):
  scope whether SpecForge uses the full scope of a page's visual information. Probe-first `.1` (measure
  the gap on the persisted corpus, no code) → `.2` mechanism design gated on a GO.
- `2026-06-14`: `.1` **done (NO-GO)** + **tree CLOSED**. Measured the gap two ways (Stage 1 corpus
  proxy over 79 docs / 14,762 pages; Stage 2 rigorous ink-outside-bbox over 16 backend docs) plus a
  5-page eyeball. No material intent-bearing content escapes both capture paths; the residual is
  decoration/shading/furniture/blank pages. Report `docs/research/full-page-capture-gap.md`; book note
  in `docs/book/src/pipeline/multimodal-evidence.md`. `.2` stays unbuilt.
