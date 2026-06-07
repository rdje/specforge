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

## Current frontier

- `PDF-VARIANT-DIGESTION.2` (Lever A, deterministic strategy) — **DONE**: `synthesize_register_field_tables`
  recovers register-FIELD tables the classifier left `unknown` (header-in-body `Field|Description|Access|
  Reset`, `Bits|Type|Reset|Description`, …) → `RegisterRecord`s. Designed from a corpus survey of real
  register-table shapes/access-notations/bit-formats ([[project_flexible_register_model]]); access/reset are
  free strings, bit ranges parse zero-padded, header-echo legend rows dropped. **RISC-V Debug: 60 regs / 179
  fields** from previously-`unknown` tables; APB/AHB/AXI/SWD source-tolerant stay 1.000 (additive); +4
  hermetic tests; full `run_ci.sh` green. RISC-V Debug PDF copied into `corpus/`. KM
  `register-field-table-extraction`.
- `PDF-VARIANT-DIGESTION.2b` (Lever A, VLM strategy) — NEXT: point Qwen2.5VL at rendered table images for
  `unknown` tables; best-wins-per-PDF vs the deterministic classifier ([[feedback_multi_strategy_best_wins]]).
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
- `PDF-VARIANT-DIGESTION.2d` (deterministic) — TOC/revision/index NOISE filter (general structure).
- `PDF-VARIANT-DIGESTION.3` (Lever B) — prose ENTITY capture: extend prose signal capture + add prose
  ACTOR/AGENT capture (ground the agent model from prose, not only as a relation subject).

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

