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

**ACTIVE FRONTIER (`2026-06-08`): `PDF-VARIANT-DIGESTION.5a`** — doc-class routing + per-doc completeness gauge
(detect protocol/register/interface/guide class from structure; report GUIDES as honest "low structured
design-intent", not a 0 failure). **`.4` (correctness/precision verification, item ①) is COMPLETE** — both
`.4a` (per-fact gold on register fields + prose signals) and `.4b` (VLM proposer/verifier audit) done.
**`.4b` DONE** (`.4b.1` harness + `.4b.2` live measurement): the `audit-extraction` VLM audit gives a
table-kind precision ESTIMATE that **discriminates extraction quality and independently corroborates `.4a`** —
RISC-V Debug **0.250/0.375** (register tables flagged for lacking in-table bit positions, matching `.4a.2`'s
bit-extent 0/179) vs NVMe **0.750** (register tables confirmed, matching `.4a.3`'s 0.931 bit-structure recall;
caught a real feature-matrix→timing misclassification). **`.4a` is DONE** (`.4a.1`–`.4a.5`): the
register-field surface is
measured on two opposite-shaped docs (RISC-V field-name recall 0.588; NVMe bit-structure recall 0.931) and the
declared-signal surface on I2C (recall 1.000 / precision 0.600). Five extraction-fix targets are now quantified
(RISC-V register-name + bit-graphic; NVMe mnemonic; I2C acronym/condition filter; …). The `.2`–`.3b` leaves
below are DONE (Lever A + B); `.4`–`.8` are the "make the breadth trustworthy" program.

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

- ID: `PDF-VARIANT-DIGESTION.4` · Status: `done` (`2026-06-08`; `.4a` + `.4b` done) · **① Correctness/precision
  verification of the broadened extraction** (the 95% that is coverage-only). Two complementary methods landed:
  per-fact GOLD on in-corpus docs (`.4a`) + a VLM proposer/verifier AUDIT that needs no gold (`.4b`); the two
  agree on which docs have strong vs weak extraction (cross-validated). Children:
  - ID: `PDF-VARIANT-DIGESTION.4a` · Status: `done` (`2026-06-08`; all children `.4a.1`–`.4a.5` done) · Goal: sample-gold the new surfaces (register fields,
    prose signals) on diverse in-scope docs whose PDFs are git-tracked in `corpus/` (so the gold is
    reproducible + independently verifiable); score per-fact with WIRE-BASED-100 rigor. **SPLIT `2026-06-08`:**
    the eval scorer (`eval.rs` / `commands/eval_extraction.rs`) has NO register-field or declared-signal task
    yet — a real lower-level dependency, so the eval SURFACE is built first (additive, hermetic), THEN per-doc
    gold on FRESH re-ingested evidence (the eval scores persisted evidence — [[eval-scores-persisted-evidence]]).
    In-corpus docs that exercise the new surfaces: RISC-V Debug + NVMe (register fields, `.2`/`.2c`), I2C
    (prose signals, `.3a`). Children:
    - ID: `PDF-VARIANT-DIGESTION.4a.1` · Status: `done` (`2026-06-08`) · Goal: register-field per-fact EVAL SURFACE —
      `EvalTask::RegisterField` + `GoldFact::RegisterField {register, field, bits_high?, bits_low?, bit_width?}`
      + a canonical key (normalize to `register|field|offset|width`) + `index_register_field_predictions` + an
      `extract_on_copy` branch reading EvidenceIR `register_records` (deterministic, like the SWD surfaces).
      Acceptance: additive (no extraction-behavior change); gold↔record key-match + closed-world scoring
      hermetic tests; full `run_ci.sh` green; APB/AHB/AXI/SWD eval unaffected.
    - ID: `PDF-VARIANT-DIGESTION.4a.2` · Status: `done` (`2026-06-08`) · Goal: RISC-V Debug register-field gold — author an
      independently source-verified gold for a bounded set of RISC-V Debug registers (e.g. `dmcontrol`,
      `dmstatus`, `abstractcs`) from the `corpus/` PDF, FRESH re-ingest (`DOCLING_DEVICE=cpu`), measure per-fact
      P/R/F1. Acceptance: each gold field checked against the source table (no faking, [[feedback_scoring_rigor]]).
      **DONE (measure & surface, owner directive):** authored `seed_riscv_debug_registers.json` — `dmstatus`
      (20 fields) + `dmcontrol` (14 fields) = 34, bit positions transcribed from the spec's bit-layout
      graphics (§3.14.1/3.14.2, RISC-V Debug 1.0). FRESH re-ingest + evidence rebuild. Added a register-agnostic
      `register_field_name_recall` + `register_field_completeness` (additive measurement, no extraction change).
      **Measured: field-name recall 20/34 = 0.588** (`dmcontrol` 14/14, `dmstatus` 6/20 — docling dropped the
      middle page-fragment of the page-split `dmstatus` table); register-name association 0/60 (synthetic names);
      bit-extent completeness 0/179 (bits live in the graphic, not the field table) → strict per-fact 0.000,
      honestly surfaced not hidden. 14 misses independently verified as REAL (not gold-spelling drift). +3
      hermetic tests; book section in `quality/extraction-eval.md`. Gaps → candidate fix leaves (register-name
      heading association; bit-layout-graphic parsing).
    - ID: `PDF-VARIANT-DIGESTION.4a.3` · Status: `done` (`2026-06-08`) · Goal: NVMe register-field gold — same rigor on a
      diverse vendor/layout (NVMe controller registers) from the `corpus/` PDF; measure per-fact P/R/F1.
      **DONE:** authored `seed_nvme_registers.json` — CAP (15) + CC (8) + CSTS (6) = 29 fields, bit ranges +
      mnemonics transcribed from the spec's `Bits|Type|Reset|Description` tables (§3.1.3.1/3.1.3.5/3.1.3.6).
      Discovered NVMe is the **INVERSE failure** of RISC-V Debug → added `register_bit_structure_recall`
      (register-scoped via `register_name_has_token`, mnemonic-agnostic, pools page-split fragments;
      additive measurement, no extraction change). **Measured: bit-structure recall 27/29 = 0.931**
      (2 misses `CAP.CRMS`/`CC.EN` at page-fragment boundaries, both REAL); field-name recall 0/29
      (mnemonics live in the DESCRIPTION, `field_name` is the bit-range); register-name 44/44; bit-extent
      199/199 → strict per-fact 0.000 on BOTH docs for OPPOSITE reasons (why two recall views are essential).
      Measured against verified-current persisted evidence (postdates last extraction commit; PDF git-tracked).
      +3 hermetic tests; book section + KM updated. Third fix leaf identified: extract mnemonic from description.
    - ID: `PDF-VARIANT-DIGESTION.4a.4` · Status: `done` (`2026-06-08`) · Goal: declared-signal (prose-capture) per-fact EVAL
      SURFACE — `EvalTask::DeclaredSignal` + `GoldFact::DeclaredSignal {signal, direction?}` + key + indexer +
      an `extract_on_copy` branch reading the EvidenceIR signal inventory. Acceptance: additive; hermetic
      tests; CI green. **DONE:** added `EvalTask::DeclaredSignal` + `GoldFact::DeclaredSignal {signal,
      direction?}` + `declared_signal_key` (name + optional direction; name-only gold matches a no-direction
      record) + `declared_signal_record_key` + `index_declared_signal_predictions`. The canonical inventory
      lives on SemanticIR (`interfaces[].signal_records`, `InterfaceSignalRecord` = name + `direction_hint`
      Input/Output/Internal), so the runner branch builds SemanticIR on a temp copy (like the TemporalRule
      task) and pools all interfaces' signal_records. Additive — no extraction change. +2 hermetic tests
      (gold↔record key match incl. name-only/direction discrimination; closed-world scoring). full
      `run_ci.sh` green (1371); kg-bench 151/151. Book note + the live I2C measurement land with `.4a.5`
      (the surface is latent until a gold ships, mirroring `.4a.1`).
    - ID: `PDF-VARIANT-DIGESTION.4a.5` · Status: `done` (`2026-06-08`) · Goal: I2C prose-signal gold — independently
      source-verified gold for the I2C prose signals (SDA/SCL/…) from the `corpus/` PDF; measure per-fact P/R/F1.
      **DONE (closes `.4a`):** authored `seed_i2c_signals.json` — the COMPLETE set of I2C-bus signals
      (SDA/SCL §3.1.1, Hs SDAH/SCLH §3.6, UFm USDA/USCL §3.2.1), each verified against UM10204's "signals"
      sections. Added `declared_signal_complete_gold_precision` (document-level precision over the produced
      signal set + named false positives; valid only for a complete-enumeration gold — additive, no extraction
      change). **Measured (`--provider skip`): recall 1.000** (all 6 genuine signals, source-tolerant) but
      **precision 6/10 = 0.600** — 4 over-captures named: ACK/NACK (conditions on SDA, §3.1.6), DDC (different
      bus, §4.6), SDR (I3C rate acronym). +2 hermetic tests; book section (declared signals) + KM card. Fix
      leaf: tighten the prose acronym/condition filter (now quantified). Full `run_ci.sh` green; kg-bench green.
  - ID: `PDF-VARIANT-DIGESTION.4b` · Status: `done` (`2026-06-08`; `.4b.1` + `.4b.2` done) · Goal: automated proposer/verifier AUDIT — re-read a
    random sample of extracted registers/signals against their table IMAGE with the VLM (the `.2b`
    consistency gate run as an audit) → a corpus-scale precision ESTIMATE + a flagged-mismatch list. Accept:
    a measured precision estimate over a stated sample size; garbage surfaced, not hidden. **SPLIT
    `2026-06-08`** into the audit HARNESS (`.4b.1`) and the live measurement (`.4b.2`) — the harness is an
    independently reviewable, hermetic, additive capability; the live estimate depends on it + a running VLM
    (mirrors `.4a`'s eval-surface → per-doc-gold split). Children:
    - ID: `PDF-VARIANT-DIGESTION.4b.1` · Status: `done` (`2026-06-08`) · Goal: the audit HARNESS — a new
      `audit-extraction <source-ir>` command that selects intent-bearing tables by STRUCTURE (the same
      predicates the extractors use: `RegisterMap`/`SignalDescription`/`Encoding`/`TimingParameter` kinds +
      `Unknown` tables matching `is_register_field_header`), takes a bounded reproducible sample
      (`--sample`/`--seed`; FNV-1a order, no RNG dep), and (live) asks the VLM per table "an extractor read
      this as a <kind> table — correct?" → a `table_kind_precision_estimate` (consistent/judged, errors
      excluded, `None` when nothing judged) + a named flagged-mismatch list. Default `--provider skip` =
      plan-only (lists the sample, no VLM calls; CI-safe). Agnostic by construction (ADR 0006): structural
      selection/judgment, no chip names in the prompt (hermetic test asserts none leak), structural sampling.
      **DONE:** `commands/audit_extraction.rs` (+ `vlm_image_query`/`is_register_field_header` made
      `pub(crate)`); 5 hermetic tests (classification, deterministic+seed-sensitive sampling, tolerant verdict
      parse, agnostic+STRICT-JSON prompt, precision/flag math). Additive — no extraction-behavior change;
      APB/AHB/AXI/SWD eval + kg-bench 151/151 unaffected; full `run_ci.sh` green (lib 1373 → 1378). Plan-only
      verified on RISC-V Debug (78 intent-bearing tables) + I2C (7 timing tables); seed reshuffle confirmed on
      real data; ONE live VLM call proved the execute path end-to-end (`table_0080` → consistent, estimate
      1.000, 0 flagged). Book section in `quality/extraction-eval.md`; KM card `extraction-audit-vlm`.
    - ID: `PDF-VARIANT-DIGESTION.4b.2` · Status: `done` (`2026-06-08`) · Goal: the live measurement — run
      `audit-extraction --provider ollama` over a bounded sample on the in-corpus docs that exercise the
      broadened extraction (RISC-V Debug / NVMe register fields; a register/signal doc for breadth), record
      the table-kind precision ESTIMATE per doc + the flagged-mismatch list into this tree + the book + KM.
      Accept: a measured estimate over a stated sample size per doc; every disagreement surfaced by name, not
      hidden; the VLM kept to a bounded sample (the `.2b` scaling finding). **DONE — live qwen2.5vl:7b audit,
      bounded sample 8/doc:**
      - **RISC-V Debug** (78 intent-bearing tables): seed 0 → **estimate 0.250** (2/8 consistent, 6 flagged,
        0 VLM errors, ~71 s); seed 1 → **0.375** (3/8, 5 flagged, ~46 s). Consistently LOW; the VLM flags the
        register tables for **lacking in-table bit positions** ("not a detailed bit-field definition table",
        "the table lacks bit positions") — INDEPENDENTLY corroborating `.4a.2`'s gold finding (bit-extent
        0/179; RISC-V's bits live in the layout graphic, not the field table).
      - **NVMe 2.0a** (118 intent-bearing tables): seed 0 → **estimate 0.750** (6/8 consistent, 2 flagged,
        ~51 s). HIGH; the VLM confirms the register tables (corroborating `.4a.3`'s 0.931 bit-structure
        recall) AND caught a REAL false positive — `table_0035` classified `timing` is actually a feature
        matrix ("lists controller features and supported modes with 0/1 instead of min/typ/max units").
      - **Cross-validation:** the audit's precision ESTIMATE tracks the per-fact gold quality (RISC-V weak ↔
        low estimate; NVMe strong ↔ high estimate) — two independent methods agreeing is what makes the audit
        a trustworthy instrument for the ungolded breadth.
      - **Robustness:** Avalon returned 8 VLM errors → `n/a (no table judged)` and **fabricated no number**
        (its `normalized/` images were reclaimed by an earlier `clean`; the audit needs on-disk table images,
        which only the git-tracked re-ingested RISC-V/NVMe retain — a signal-bearing breadth audit on a fresh
        re-ingest is a noted follow-up). Book section refreshed with the real numbers; KM `extraction-audit-vlm`
        updated. Commit subject: `PDF-VARIANT-DIGESTION.4b.2`.
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
- `.4a.1` (`2026-06-08`): register-field per-fact eval surface added to `eval.rs` + `commands/eval_extraction.rs`
  (`EvalTask::RegisterField`, `GoldFact::RegisterField`, normalized `register|field|offset|width` key,
  `index_register_field_predictions`, deterministic `extract_on_copy` branch over EvidenceIR `register_records`).
  Additive — no extraction-behavior change. +3 hermetic tests (bit-extent normalizer; gold↔record key match incl.
  range≡offset+width, wrong-extent divergence, register-as-identity; closed-world scoring). `cargo fmt --check`
  clean, `cargo clippy -D warnings` clean, `kg-bench` 151/151, full lib suite 1360 → 1363. Book note deferred to
  `.4a.2` (the surface is latent until a gold ships). Commit subject: `PDF-VARIANT-DIGESTION.4a.1`.

- `.4b.1` (`2026-06-08`): audit harness landed — `commands/audit_extraction.rs` (new `audit-extraction`
  command) + `vlm_image_query`/`is_register_field_header` made `pub(crate)`. Structural sampler
  (`audited_kind` mirrors the extractor predicates; `select_sample` FNV-1a deterministic, seed-sensitive),
  kind-aware agnostic STRICT-JSON audit prompt, tolerant `{consistent,reason}` verdict parser,
  `table_kind_precision_estimate` (consistent/judged, errors out of the denominator, `None` when nothing
  judged) + named flagged-mismatch list. `--provider skip` plan-only default (CI-safe). +5 hermetic tests.
  `cargo fmt --check` clean, `cargo clippy -D warnings` clean (dropped the `enum_variant_names` postfix),
  full lib suite 1373 → 1378, kg-bench 151/151, APB/AHB/AXI/SWD eval unaffected (additive). Plan-only run on
  RISC-V Debug (78 intent-bearing tables sampled 8) + I2C (7 timing tables); seed-1 reshuffle confirmed on
  real data; ONE live `--provider ollama` call proved the execute path (`table_0080` → consistent, estimate
  1.000, 0 flagged). Book `quality/extraction-eval.md` + KM `extraction-audit-vlm`. Commit subject:
  `PDF-VARIANT-DIGESTION.4b.1`.

## Changelog

- `2026-06-07`: Created (owner high-priority directive — digest any chip-spec PDF). `.1` triage sweep in flight.
- `2026-06-08`: Split `.4b` (proposer/verifier VLM audit) into `.4b.1` (audit harness — DONE) + `.4b.2` (live
  measurement — pending); `.4b` → `active`, frontier moves to `.4b.2`. Mirrors `.4a`'s eval-surface → per-doc
  split (the harness is the lower-level dependency of the measurement).
- `2026-06-08`: `.4b.2` live measurement DONE → `.4b` + `.4` (precision-verification item ①) CLOSED. Live
  qwen2.5vl:7b audit: RISC-V Debug 0.250/0.375 (bit-position gap, corroborates `.4a.2`), NVMe 0.750 (confirmed +
  caught a feature→timing misclassification, corroborates `.4a.3`). The audit estimate tracks gold quality
  (cross-validated). Frontier moves to `.5a` (doc-class routing + per-doc completeness gauge — item ②).
- `2026-06-08`: Split `.4a` (precision verification) into `.4a.1`–`.4a.5` — the eval scorer has no
  register-field/declared-signal task yet (a real lower-level dependency, PNT split rule). `.4` + `.4a` →
  `active`; `.4a.1` (register-field eval surface) → `in_progress` and onto the frontier. Scope kept to
  in-corpus reproducible docs (RISC-V Debug, NVMe, I2C) so gold is independently verifiable, not the
  owner-library-only CCIX/OpenCAPI/GIC.

## Tooling + multi-strategy (`2026-06-07`)

PDF reading is multi-strategy, best-wins-per-PDF ([[?]]): **docling** (structured `content_elements`, the
pipeline path) + **`scripts/pdf_text.py`** (raw text via pypdf) both read all 82 (incl. the 12
permission-encrypted ones, which open with an empty password). The Claude Read tool is unreliable here and
NOT fixable by a plugin (GitHub #38530) — use the two paths above. `scripts/decrypt_pdf.py` strips
encryption for tools that need it. pypdf+cryptography installed in `.venv-docling`. KM
`pdf-encryption-and-read-access`. **Lever A** is likewise multi-strategy: deterministic header-grammar
classifier + the VLM (Qwen2.5VL) on unknown tables, best-wins-per-PDF.

