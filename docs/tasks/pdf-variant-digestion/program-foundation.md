# PDF-VARIANT-DIGESTION — program foundation and legacy planning

- Part ID: `program-foundation`
- State: `legacy`

<!-- active-task-source-region:program-identity:start -->
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

<!-- active-task-source-region:program-identity:end -->

<!-- active-task-source-region:triage-node-results:start -->
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

<!-- active-task-source-region:triage-node-results:end -->

<!-- active-task-source-region:activities-02-08:start -->
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
- ID: `PDF-VARIANT-DIGESTION.5` · Status: `done` (`2026-06-08`; all children `.5a`/`.5b`/`.5c` done) ·
  **② Doc-class routing + per-doc completeness gauge.** Children: `.5a` (done) · `.5c` (done) · `.5b` (done).
  Item ② COMPLETE — `validate <evidence>` now reports the document class, the front-matter self-declared type
  (true guide vs under-extracted spec), AND a class-aware per-doc completeness gauge. Frontier moves to `.6`
  (VLM levers on the addressable zero-yield) / `.7` (USB 3.2 evidence-fail) / `.8` (broaden prose-actor capture).
  - ID: `PDF-VARIANT-DIGESTION.5a` · Status: `done` (`2026-06-08`) · Goal: detect doc class (protocol / register /
    interface / guide) from structure; apply class-appropriate surfaces; report GUIDES as "low structured
    design-intent" honestly (not a 0 failure). Accept: each doc tagged with a class; the 8 zero-yield docs
    correctly identified as guides / image-heavy, not silent misses. **DONE — pure
    `crate::ir::completeness::classify_document(DocumentClassCensus) -> DocumentClassification`
    (`{Protocol,Register,Interface,Guide}` + rationale), surfaced in `validate` as a `document_class` metric +
    an `evidence_document_class` Info finding + a console "Document Class" block. Decision (low-noise surfaces
    only): Register (registers dominate connectivity AND behavioral) → Protocol (`signal_constraints ≥ 3` or
    FSM/serial-frame) → Interface (signal inventory + connectivity, no behavior) → Guide (honest floor).
    REAL-DATA correction baked in: `conditional_rules` is OVER-PRODUCED (12× on the GIC overview guide, 78× on
    RISC-V Debug, 250× on NVMe) → EXCLUDED from the decision (kept in census, flagged "not class-determining").
    Agnostic (ADR 0006 — generic floors 2/3/3, no chip names). +11 hermetic tests (real APB/AHB/AXI/SWD/I2C/
    NVMe/RISC-V/Avalon/guide shapes); fmt + clippy `-D warnings` clean; lib 1400 → 1410; kg-bench 151/151.
    Live-verified over all 74 persisted evidence docs: protocol 25 / register 11 / interface 22 / guide 16 (the
    software/overview/optimization guides correctly → guide). No wire-based regression. KM card
    `document-class-from-structure`.**
  - ID: `PDF-VARIANT-DIGESTION.5c` · Status: `done` (`2026-06-08`) · Goal (owner-suggested `2026-06-08`): read the document's
    OWN front-matter — title, table of contents, preface/about/scope of the first chapter ("usually clearly
    stated in the early pages of the first chapter") — for a self-declared doc-TYPE signal (generic grammar:
    guide/overview/manual/specification/architecture/protocol/datasheet words, NO chip/vendor names, ADR 0006).
    Use it to (a) corroborate the `.5a` structural class, and (b) crucially separate a TRUE guide from a
    SPECIFICATION we under-extracted (a doc whose structure is empty but whose title says "specification /
    architecture / protocol / manual" is an image/table-heavy extraction GAP → the VLM frontier `.6`, NOT a
    real guide). Accept: a title/front-matter doc-type hint surfaced in `validate`; structurally-empty docs whose
    front-matter self-declares a spec are flagged as under-extracted (not silently called "guide"). **DONE —
    pure `front_matter_doc_type_hint(&str) -> DeclaredDocType {Guide,Specification,Unknown}` over generic
    doc-type vocabulary (guide/tutorial/"learn the architecture"/application-note vs specification/architecture/
    protocol/standard/datasheet/reference-manual; guide phrasings win over spec words so Arm's "Learn the
    architecture …" series reads as a guide; "overview"/"introduction" EXCLUDED because every spec has those
    chapters; whole-word match so "guidelines" ≠ "guide"). `classify_document` now takes the front-matter signal
    and sets `under_extracted_spec = (class==Guide && declared==Specification)`. `validate` reads the title +
    first 12 section headings off the sibling SourceIR (the `document_profile.title` is empty in practice → the
    early headings carry the signal), prints `document_type_declared` + an under-extracted line, adds a
    `document_type_declared` metric + a WARNING `evidence_document_underextracted_spec` finding. REALITY: the
    document title is empty; the early first-chapter headings hold the type (owner was right). +6 hermetic tests
    (real corpus framings). Agnostic (ADR 0006). fmt + clippy `-D warnings` clean; lib 1410 → 1416; kg-bench
    151/151. **Live: of the 16 `guide`-classed docs, 11 are TRUE guides (declared guide/unknown) and 5 are
    UNDER-EXTRACTED specs flagged for the VLM frontier** — RISC-V Advanced Interrupt *Architecture*, JESD235
    *JEDEC STANDARD* HBM, CoreSight Base System *Architecture* (+2). No wire-based regression. KM card
    `document-class-from-structure` updated.**
  - ID: `PDF-VARIANT-DIGESTION.5b` · Status: `done` (`2026-06-08`) · Goal: per-doc COMPLETENESS
    gauge (every register has fields? every signal a direction? unaccounted intent-bearing tables?) — extend the
    mandatory-width flag into a coverage/quality report surfaced by `validate`. Accept: honest per-doc gap
    counts; no fabrication; class-aware. **DONE** — pure `document_completeness_gauge` + `DocumentCompletenessGauge`
    / `CompletenessGap` in `ir/completeness.rs`; `validate <evidence>` prints a Document Completeness Gauge block +
    `document_completeness_gaps` / `document_completeness_applicable` metrics + an Info `evidence_document_completeness`
    finding. +5 hermetic tests (guide→not-applicable; register-doc held to fields+width; protocol-with-no-registers
    shows no register gap; fully-attributed doc is complete; sample bounded). fmt + clippy `-D warnings` clean; lib
    1416 → 1421; kg-bench 151/151; APB/AHB/AXI/SWD eval unaffected (additive). Live over persisted evidence: APB
    (protocol) 1 gap (signals_without_direction 1/32, tables 0/8); RISC-V Debug (register) registers_unresolved_width
    60/60 (XLEN-parametric, corroborates `.4a.2`) + all 60 registers have fields; Avalon (interface) signals 11/34 +
    1/13 tables; GIC overview guide → not applicable (never penalized). Book `quality/validation.md` subsection; KM
    `document-class-from-structure` updated.
    **Design (`2026-06-08`):** a pure `crate::ir::completeness::document_completeness_gauge(class, registers,
    declared_signal_count, signals_missing_direction, intent_bearing_table_count, unexplained_tables) ->
    DocumentCompletenessGauge { class, applicable, gaps: Vec<CompletenessGap{kind, missing, total, sample}> }`.
    CLASS-AWARE keyed off the `.5a` `DocumentClass`: a `Guide` is `applicable=false` (low structured
    design-intent — nothing to gauge; the `.5c` under-extracted flag carries the "actually a spec" case), so a
    guide is NEVER penalized for 0 registers/signals. For Register/Protocol/Interface, each dimension is gauged
    only when its denominator (`total`) > 0 — so a protocol with no registers shows no register gap and a
    register doc IS held to "every register has fields / a resolved width". Dimensions (all read off
    already-built IR — pure observation, no fabrication): `registers_without_fields`,
    `registers_unresolved_width` (reuses `registers_with_unresolved_width`), `signals_without_direction`
    (declared inventory − `collect_signals_with_explicit_direction_declarations`, which already folds in
    relation-derived directions), `unexplained_intent_bearing_tables` (reuses `unexplained_intent_bearing_tables`).
    Each gap carries a bounded `sample` (≤8) of affected names/ids for review. Surfaced in `validate <evidence>`
    as a console block + a `document_completeness_gaps` metric + an Info `evidence_document_completeness`
    finding (Info, not Warning — an honest known-incomplete report, not a correctness error; matches the
    severity-gating doctrine). Agnostic (ADR 0006 — no chip names; structural only). Keep APB/AHB/AXI/SWD at
    100%; additive (no extraction-behavior change).
- ID: `PDF-VARIANT-DIGESTION.6` · Status: `pending` · **③ VLM levers on the addressable zero-yield** — run
  `.2b`/`.2b'` on the image-table-heavy zero docs (OpenCAPI PHY-mech / AFU). Accept: measured uplift (tables
  reclassified/repaired → records) with 0 garbage (verification gate); honest report where the VLM also can't.
- ID: `PDF-VARIANT-DIGESTION.7` · Status: `pending` · **④ Concrete defects from the sweep** — investigate +
  fix the USB 3.2 evidence-build FAIL; adopt "measure from typed `evidence_ir/` artifacts" as the convention
  (the sweep's `rel` variable-collision bug). (Giant-ingest chunking deferred — those were ISA, now out of
  scope.)
- ID: `PDF-VARIANT-DIGESTION.8` · Status: `done` (`2026-06-08`) · **⑤ Broaden prose-actor capture**
  (14/74 on the current persisted set) — add agent-definition forms. Accept: more docs with actors,
  garbage-free, no regression on the wire-based specs.
  **DONE — robust STRUCTURAL grammar (no fragile denylist; reviewer-flagged + redesigned).** Generalized the
  `.3b` Form 1 (literal `"<NAME> is the device which/that <capability>"`) into `agent_definitions(text)`:
  `"<NAME> is a/an/the <agent-class> {that|which} <capability>"` over a CONSERVATIVE generic agent-class
  ALLOWLIST `AGENT_CLASS_NOUNS` (device/component/agent/module/entity/controller/manager/master/initiator/
  peripheral/bridge/engine/processor/host/node/subsystem — the ambiguous data/structural words "unit"/"block"
  deliberately EXCLUDED; an allowlist fails safe toward fewer captures). **First pass over-captured 4 garbage
  actors on real data; an expert reviewer flagged a growing structural-noun DENYLIST as fragile → redesigned to
  pure STRUCTURAL signals (no enumeration):** (1) `strip_trailing_parenthetical` drops a `"(refer to section
  3.1.2.1)"` cross-ref before NAME extraction (recovers the real "controller", kills "section"); (2) the NAME
  search is confined to the CURRENT SENTENCE so an anaphor `"… host system. It is the entity that …"` can't reach
  back across a period (kills "system"); (3) a NO-PREPOSITION-in-subject guard (prepositions are a CLOSED
  grammatical class, not an open noun list) rejects a prepositional-phrase object `"a use case FOR multiple HSELx
  signals is a peripheral that …"` (kills "signals"). NAME still gated by the pre-existing `is_agent_noun`
  function-word denylist (UNCHANGED — no growth) + deduped; Form 2 ("considered a/the/an <NAME>") unchanged.
  Generic grammar, no chip/vendor names (ADR 0006). +6 hermetic tests (generalized class nouns; reject
  non-agent-class; reject part-of; parenthetical cross-ref; prepositional-object; cross-sentence anaphora). fmt +
  clippy `-D warnings` clean; lib 1421 → 1427; kg-bench 151/151. **VERIFIED on real data:** canonical (real Rust
  extractor, the only 3 docs with un-reclaimed `normalized/`): NVMe 0→1 clean "controller" (section gone via the
  parenthetical strip), I2C controller/target + RISC-V "trap" unchanged. Breadth PROJECTION (faithful Python
  mirror of the grammar over persisted statements; canonical requires re-ingest of the 72 reclaimed-`normalized`
  docs): **14 → 20 docs (+6)**, every newly-gained actor a genuine agent (A76 core / ETM trace unit, CoreSight
  splitter, CCIX Transport port, AXI manager, CoreSight component) — verified per-item. APB/AHB/AXI/AXI-Stream/SWD
  project 0 actors (clean; eval unaffected — actors are an additive surface, not scored).

`.2b'` grid-repair is proven end-to-end on RISC-V (2 tables) + the `parse_vlm_grid` test. SCALING FINDING
(`2026-06-08`): `enrich --vlm` calls the VLM per unknown table → impractically slow on table-heavy docs
(NVMe 100s) → the VLM is a TARGETED/SAMPLED tool, not a full-doc pass; `.4b`/`.6` must operate on a bounded
set, not the whole doc/corpus.

<!-- active-task-source-region:activities-02-08:end -->

<!-- active-task-source-region:decisions-questions-blockers:start -->
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

<!-- active-task-source-region:decisions-questions-blockers:end -->

<!-- active-task-source-region:tooling-note:start -->
## Tooling + multi-strategy (`2026-06-07`)

PDF reading is multi-strategy, best-wins-per-PDF ([[?]]): **docling** (structured `content_elements`, the
pipeline path) + **`scripts/pdf_text.py`** (raw text via pypdf) both read all 82 (incl. the 12
permission-encrypted ones, which open with an empty password). The Claude Read tool is unreliable here and
NOT fixable by a plugin (GitHub #38530) — use the two paths above. `scripts/decrypt_pdf.py` strips
encryption for tools that need it. pypdf+cryptography installed in `.venv-docling`. KM
`pdf-encryption-and-read-access`. **Lever A** is likewise multi-strategy: deterministic header-grammar
classifier + the VLM (Qwen2.5VL) on unknown tables, best-wins-per-PDF.

<!-- active-task-source-region:tooling-note:end -->

