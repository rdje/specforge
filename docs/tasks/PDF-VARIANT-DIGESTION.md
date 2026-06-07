# PDF-VARIANT-DIGESTION: make SpecForge digest as many chip-spec PDF variants as possible

## Metadata

- Tree ID: `PDF-VARIANT-DIGESTION`
- Status: `active` (high-priority program)
- Roadmap lane: `R15`/`R16` (ingestion + extraction breadth)
- Created: `2026-06-07`
- Parent: owner high-priority directive — "make SPECFORGE capable to handle any chip-spec PDF we can throw
  at it, that's the dream, the aim … we should do everything we can to get as close as possible." Defer
  IntentIR→ISF lowering. (`project_pdf_variant_digestion` memory.)

## Goal (the aim — this is the ORIGINAL roadmap goal, not a pivot)

Per the README objective, SpecForge extracts intent from "**protocol, component, system, and
software-interface specifications**" into a backend-independent `IntentIR` — i.e. **any chip-spec PDF** has
always been the goal (forward specification mining). AMBA / `WIRE-BASED-100` was the FIRST *measured* class
(a starting point to prove + per-fact-harden the extraction), never the target. This program pursues that
original goal at full breadth. SpecForge ingests and meaningfully extracts intent from **any chip-spec
PDF** — across every vendor, doc type, and layout. The concrete near-term target is the **82-PDF library** (ARM AMBA/debug/system-IP/ISA/
TRMs, CXL/OpenCAPI/CCIX, USB-IF, RISC-V, Intel/AMD, JEDEC, NXP, NVMe, OpenCores) plus whatever the owner
adds. "Digest" = ingest cleanly (no crash / handle protection) AND produce non-garbage typed extraction
appropriate to the doc (signals/constraints/relations/temporal/FSM/registers/encodings), or an honest
diagnostic when a construct is genuinely out of model.

## Non-goals

- NOT IntentIR→ISF lowering (deferred by the owner; revisit later).
- Do NOT regress the four wire-based specs: APB/AHB/AXI = 100% (constraints/relations/temporal),
  SWD = 100% (frame/operations/FSM). kg-bench stays green.
- Not perfect extraction of every fact in a 1000-page ISA manual — meaningful, measured progress per class.

## Acceptance criteria

- A triage matrix over the corpus: per PDF, ingest status + extraction stats (tables/kinds/signals/
  constraints/relations) — so coverage is measured, not guessed.
- Each addressed variant CLASS: a general feature (ADR-0006-safe, no hardcoded chip names) that unlocks it,
  + a regression PDF copied into `corpus/` + git-tracked, + per-fact measurement where a gold applies.
- No regression on the four wire-based specs; full `scripts/run_ci.sh` green; KM card per durable finding.

## Task tree

- ID: `PDF-VARIANT-DIGESTION` · Status: `active` · Children: `.1` (triage) + per-class feature leaves (TBD)
- ID: `PDF-VARIANT-DIGESTION.1` · Status: `in_progress` · Goal: **triage sweep** — ingest a diverse sample
  (one per family: ARM-TRM GIC-400, Wishbone, NXP I2C, RISC-V Debug, CCIX, Avalon, USB4, OpenCAPI) through
  `ingest`→`evidence`, record ingest status + table-kind census + extraction stats, and identify the
  per-variant FAILURE MODES (ingest crash / password protection / empty or garbage extraction / non-table
  structures / missing catalogs). Output `/tmp/digest_triage.txt` → distilled into this tree + KM. Then
  decompose into per-class feature leaves prioritized by impact (how many of the 82 each unlocks).

## Triage matrix (`.1`, `2026-06-07`, 8 diverse families — all INGEST cleanly; docling robust)

| spec | tables unknown/total | signals | relations | constraints |
|---|---|---|---|---|
| Avalon (Intel interface) | 8/34 | 34 | 179 | 0 |
| GIC-400 (ARM TRM) | 14/25 | 19 | 11 | 0 |
| NXP I2C | 14/23 | 0 | 0 | 19 |
| RISC-V Debug | 116/134 | 10 | 20 | 0 |
| CCIX | 220/261 | 0 | 0 | 35 |
| USB4 | 23/23 | 0 | 0 | 6 |
| Wishbone / OpenCAPI | (ingest ok; stats pending) | | | |

**Two dominant, genericity-preserving levers (structure/grammar, not names):**
- **Lever A — table-kind classification.** "unknown" dominates (CCIX 220/261, RISC-V 116/134, USB4 23/23);
  Avalon, which classifies well, yields 34 signals + 179 relations. Generalizing the table classifier lifts
  every doc at once. #1 impact.
- **Lever B — prose ENTITY capture: signals AND actors/agents** (owner `2026-06-07`). I2C/CCIX/USB4 show 0
  signals AND 0 relations because their entities live in prose/figures, not classified tables. Extend the
  prose-pin-appositive (`SWD-SERIAL-EXTRACTION.2`) to capture signals from more prose forms AND to ground
  the ACTOR/AGENT model from prose ("the host debugger issues requests", "the Manager initiates", "the SMMU
  translates", "the requesting agent") — not only as the inferred subject of a relation.

## `.1` refinement (why tables are "unknown") — investigated `2026-06-07`

The "unknown" tables are a MIX, so the raw count overstates the gap:
- **Non-data noise** — TOC entries ("Preface … 1", "1.1. SCOPE 8"), chapter/section indexes, revision
  history ("Version | Comments | Issue Date"), dotted page-leaders. These SHOULD stay unextracted (CCIX's
  220 is largely its huge TOC). A general structural filter (page-number column, dotted leaders, "Table of
  Contents"/"Chapter"/"Version…Issue Date" headers) cleans the signal + avoids noise extraction.
- **Real data tables, unclassified** — e.g. RISC-V `Field | Description | Access | Reset` (register-field
  tables), access legends, argument tables. These ARE intent and are dropped today. OpenCAPI shows the
  upside (it classified 16 register_map → register intent captured).

OpenCAPI: 49 tables (27 unknown, 16 register_map, …), 0 signals/relations/constraints. (Wishbone stats hit
a stats-script regex bug — ingest OK; re-measure with the hardened helper.)

## Whole-corpus re-triage (`.1` re-run, `2026-06-08`) — Lever A+B uplift measured

Ran the deterministic pipeline (ingest→evidence, no VLM) over all 82: **75 OK** (+Wishbone), **5 ingest
timeouts** (giant ISA manuals — ARM A32/T32, A64, registers; Intel SDM; USB4 v2.0 — need a longer ingest
budget), **1 evidence-fail** (USB 3.2 — bug). **Aggregate (vs the original triage where most non-AMBA docs
were 0): 62 docs yield signals, 34 registers, 49 relations; totals 1,908 signals / 2,953 registers / 10,632
fields / 3,077 relations.** #3 confirmed: CCIX (was 0 → sig/reg/fld/con/rel), I2C (sig 10), OpenCAPI, USB4
CM/inter-domain all extract. ~9 OK docs still yield 0 (guides / ISA / image-table-heavy) = the VLM frontier
(`.2b`/`.2b'`). Matrix `docs/corpus_coverage_2026-06-08.md`; KM `corpus-coverage-sweep`. NEXT: `#2` VLM
grid-repair demo on a bits-bearing doc; giants' ingest budget; USB 3.2 evid-fail.

## Current frontier

- `PDF-VARIANT-DIGESTION.2` (Lever A, deterministic strategy) — **DONE**: `synthesize_register_field_tables`
  recovers register-FIELD tables the classifier left `unknown` (header-in-body `Field|Description|Access|
  Reset`, `Bits|Type|Reset|Description`, …) → `RegisterRecord`s. Designed from a corpus survey of real
  register-table shapes/access-notations/bit-formats ([[project_flexible_register_model]]); access/reset are
  free strings, bit ranges parse zero-padded, header-echo legend rows dropped. **RISC-V Debug: 60 regs / 179
  fields** from previously-`unknown` tables; APB/AHB/AXI/SWD source-tolerant stay 1.000 (additive); +4
  hermetic tests; full `run_ci.sh` green. RISC-V Debug PDF copied into `corpus/`. KM
  `register-field-table-extraction`.
- `PDF-VARIANT-DIGESTION.2b` (Lever A, VLM strategy) — **DONE (classification)**: Qwen2.5VL reads the
  `table_region` images and reclassifies `unknown` tables (validated live — it read the RISC-V `dmcontrol`
  table's kind + all 5 field names from the image). `enrich --vlm-provider ollama` now runs
  `classify_unknown_tables_via_vlm` (shared `vlm_image_query`; `parse_vlm_table_kind`), best-wins (only
  `unknown` tables touched; `register_field`/TOC/other not reapplied — grammar path / noise), writes
  `table_kind` back so a re-run of `evidence` fires the deterministic extractor. Gated (`skip` = no-op);
  encryption irrelevant (docling renders the images). **VERIFICATION GATE** (`vlm_kind_structurally_consistent`):
  the VLM proposes, structure disposes — a kind is applied only when the table header matches it, so the VLM's
  over-classifications (register-field / operation tables → `signal_description`) are rejected. Measured on
  RISC-V: without gate 22 reclassified (+9 real DMI signals but +5 garbage); WITH gate **1 reclassified → 9
  genuine DMI signals (`REQ_*/RSP_*`), 0 garbage**. +2 hermetic tests; full CI green. KM `vlm-table-strategy`.
- `PDF-VARIANT-DIGESTION.2b'` (Lever A, VLM extraction) — **DONE**: VLM GRID REPAIR. ~10% of corpus tables
  (262) are degenerate (Docling failed to structure them: ≤1 column). `enrich --vlm-provider` now runs
  `repair_degenerate_tables_via_vlm` — the VLM transcribes the table image to a JSON grid
  (`build_table_extract_prompt`/`parse_vlm_grid`, serde_json, ≥2 columns), REPLACING the degenerate
  header/body so the deterministic extractors run (best-wins at the STRUCTURE level: Docling grid vs VLM
  grid). Kind set only when the repaired header is structurally consistent (`.2b` gate). +1 hermetic test;
  full CI green; gated (skip = no-op).
- `PDF-VARIANT-DIGESTION.2c` (model flexibility) — **DONE (core)**: the register model now carries, all
  backward-compatible: `RegisterRecord.size_bits`; `RegisterFieldRecord.bit_width` (a field is
  `name + offset(=bits_low, LSb) + width`; range/single-bit/offset+width all map) + `enumerated_values`
  (`RegisterFieldEnumRecord { value, meaning }`); access/reset stay FREE strings. Populated deterministically
  where data exists: width from `[high:low]`, register size from max field MSb, inline enums from
  binary/hex/Verilog literals in descriptions (conservative — no bare-int false positives). A no-garbage
  filter drops bit-LAYOUT grids (register-diagram tables whose "fields" are bare bit numbers). +3 hermetic
  tests. **Real-data demo: NVMe → 44 registers / 199 fields, all with `bit_width`** (3 bit-layout grids
  filtered); RISC-V unaffected (60/179); APB/AHB/AXI/SWD stay 100%. NVMe added to `corpus/`. Owner-confirmed
  model
  ([[project_flexible_register_model]]). **Width is MANDATORY (physical):** a register is bit-storage so a
  width always exists; an unresolved `size_bits` is a COMPLETENESS GAP (parametric XLEN / cross-document),
  not optional — `validate` reports `registers_unresolved_width` (RISC-V 60/60 vs NVMe 0/44). REMAINING:
  register NAME from preceding heading (synthetic today — tables aren't in Docling's content_elements
  reading-order, so reliable association is deferred, not faked); block/base grouping; array/instance;
  parametric `size_expr` + cross-document width resolution.
- `PDF-VARIANT-DIGESTION.2d` (deterministic) — **DONE**: `table_is_noise` flags non-data NOISE tables (table
  of contents, list of tables/figures, revision history, section index) by general structure — dotted
  page-leaders, contents/revision caption-or-header, or rows mostly prefixed by a section number (no chip
  names). The VLM passes skip them (no wasted calls; CCIX has ~220 TOC tables) and `enrich` reports
  `tables_skipped_as_noise`. +1 hermetic test; full CI green.
- `PDF-VARIANT-DIGESTION.3a` (Lever B, prose SIGNALS) — **DONE**: `synthesize_signal_declarations_from_prose`
  gained the parenthetical-abbreviation form ("a serial data line (SDA)") on top of the pin appositive (`.2`).
  Guards (from live I2C runs): sparse-catalog FALLBACK gate (parenthetical runs only when <8 table signals —
  AXI etc. untouched, fixed a 1.000→0.857 AXI regression), uppercase-acronym gate (rejects "(resulting…)"),
  universal denylist (READ/WRITE/MODE). **I2C: 0 → 10 declared signals** (SDA/SCL + Hs SCLH/SDAH + USCL/USDA
  + ACK/NACK/DDC/SDR). Wire-based specs stay 1.000; +3 hermetic tests; full CI green. I2C PDF added to
  `corpus/`. KM `prose-signal-capture`.
- `PDF-VARIANT-DIGESTION.3b` (Lever B, prose ACTORS/AGENTS) — **DONE**: new `ProtocolActorRecord` surface +
  `extract_protocol_actors` capture agents a spec DEFINES in prose — "A <name> is the device which/that
  <capability>" and "considered a/the <name>" — grounding the agent model from prose, not only as a relation
  subject. `is_agent_noun` rejects function/structural words (general; admits vendor agents like SMMU). **I2C:
  2 actors — controller (def: "the device that initiates a data transfer … and generates the clock") +
  target.** Additive new surface (no eval impact; wire-based unaffected); +2 hermetic tests; full CI green.
  KM `prose-signal-capture`.

## Planned next — from the corpus-sweep learnings (`2026-06-08`)

The sweep proved BREADTH (66/74 in-scope docs yield extraction; 1,908 signals / 2,953 registers / 10,632
fields / 3,077 relations) but only **4/74 (5%) have verified precision** (APB/AHB/AXI/SWD golds). The
through-line for these leaves: make the breadth TRUSTWORTHY (objectively measured, no faking) before widening
it further. Priority order ① → ⑤.

- ID: `PDF-VARIANT-DIGESTION.4` · Status: `pending` · **① Correctness/precision verification of the broadened
  extraction** (the 95% that is coverage-only). Children:
  - ID: `PDF-VARIANT-DIGESTION.4a` · Status: `pending` · Goal: sample-gold the new surfaces (register fields,
    prose signals) on ~5–8 diverse in-scope docs (CCIX, NVMe, RISC-V IOMMU/Debug, OpenCAPI, a GIC/CoreSight
    TRM); score per-fact with WIRE-BASED-100 rigor. Accept: per-fact P/R/F1 reported per doc; gold facts
    independently verified against the source (no faking, [[feedback_scoring_rigor]]).
  - ID: `PDF-VARIANT-DIGESTION.4b` · Status: `pending` · Goal: automated proposer/verifier AUDIT — re-read a
    random sample of extracted registers/signals against their table IMAGE with the VLM (the `.2b`
    consistency gate run as an audit) → a corpus-scale precision ESTIMATE + a flagged-mismatch list. Accept:
    a measured precision estimate over a stated sample size; garbage surfaced, not hidden.
- ID: `PDF-VARIANT-DIGESTION.5` · Status: `pending` · **② Doc-class routing + per-doc completeness gauge.**
  Children:
  - ID: `PDF-VARIANT-DIGESTION.5a` · Status: `pending` · Goal: detect doc class (protocol / register /
    interface / guide) from structure; apply class-appropriate surfaces; report GUIDES as "low structured
    design-intent" honestly (not a 0 failure). Accept: each doc tagged with a class; the 8 zero-yield docs
    correctly identified as guides / image-heavy, not silent misses.
  - ID: `PDF-VARIANT-DIGESTION.5b` · Status: `pending` · Goal: per-doc COMPLETENESS gauge (every register has
    fields? every signal a direction? unaccounted intent-bearing tables?) — extend the mandatory-width flag
    into a coverage/quality report surfaced by `validate`. Accept: honest per-doc gap counts; no fabrication.
- ID: `PDF-VARIANT-DIGESTION.6` · Status: `pending` · **③ VLM levers on the addressable zero-yield** — run
  `.2b`/`.2b'` on the image-table-heavy zero docs (OpenCAPI PHY-mech / AFU). Accept: measured uplift (tables
  reclassified/repaired → records) with 0 garbage (verification gate); honest report where the VLM also can't.
- ID: `PDF-VARIANT-DIGESTION.7` · Status: `pending` · **④ Concrete defects from the sweep** — investigate +
  fix the USB 3.2 evidence-build FAIL; adopt "measure from typed `evidence_ir/` artifacts" as the convention
  (the sweep's `rel` variable-collision bug). (Giant-ingest chunking deferred — those were ISA, now out of
  scope.)
- ID: `PDF-VARIANT-DIGESTION.8` · Status: `pending` · **⑤ Broaden prose-actor capture** (only 16/74 today) —
  add agent-definition forms and/or ground actors via relations. Accept: more docs with actors, `is_agent_noun`
  gate keeps it garbage-free, no regression on the wire-based specs.

`.2b'` grid-repair is proven end-to-end on RISC-V (2 tables) + the `parse_vlm_grid` test. SCALING FINDING
(`2026-06-08`): `enrich --vlm` calls the VLM per unknown table → impractically slow on table-heavy docs
(NVMe 100s) → the VLM is a TARGETED/SAMPLED tool, not a full-doc pass; `.4b`/`.6` must operate on a bounded
set, not the whole doc/corpus.

## Decisions

- Triage-first: measure what breaks across variants before building, so features target real gaps (the
  owner's measure/think-before-coding ethos).
- Operate ingestion from the owner library for the survey; copy a PDF into `corpus/` + git-track it only
  when SpecForge gains a feature for it (do NOT log the library path — `feedback_source_pdfs_in_repo`).

## Open questions

- Which variant classes are highest-impact (unlock the most of the 82)? — answered by `.1`.
- How to score "digestion" for non-signal-table docs (TRMs/ISA manuals) — a coverage/quality gauge vs a
  per-fact gold?

## Blockers

- None. (Some PDFs may be password-protected — e.g. the ADI one — or very large; expect and handle.)

## Verification log

- `.1`: triage sweep launched `2026-06-07` over 8 diverse families.

## Changelog

- `2026-06-07`: Created (owner high-priority directive — digest any chip-spec PDF). `.1` triage sweep in flight.

## Tooling + multi-strategy (`2026-06-07`)

PDF reading is multi-strategy, best-wins-per-PDF ([[?]]): **docling** (structured `content_elements`, the
pipeline path) + **`scripts/pdf_text.py`** (raw text via pypdf) both read all 82 (incl. the 12
permission-encrypted ones, which open with an empty password). The Claude Read tool is unreliable here and
NOT fixable by a plugin (GitHub #38530) — use the two paths above. `scripts/decrypt_pdf.py` strips
encryption for tools that need it. pypdf+cryptography installed in `.venv-docling`. KM
`pdf-encryption-and-read-access`. **Lever A** is likewise multi-strategy: deterministic header-grammar
classifier + the VLM (Qwen2.5VL) on unknown tables, best-wins-per-PDF.

