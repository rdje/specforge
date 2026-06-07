# WIRE-BASED-100: max every score (100%) on wire-based interface specs (APB/AHB/AXI/SWD/…)

## Metadata

- Tree ID: `WIRE-BASED-100`
- Status: `active` (feature program)
- Roadmap lane: `R15e`/`R16` (extraction quality / signoff)
- Created: `2026-06-06`
- **Owner bar (NON-NEGOTIABLE):** SpecForge shall max ALL scores — 100% on every aspect — for every
  *wire-based* interface spec, where the design signals are literally wires on the bus (APB, AHB, AXI,
  SWD/ADI, …), unlike the packet/flit protocol CHI. Anything below 100% on a wire-based spec is a defect.

## Governing principle — no fake scoring (owner, NON-NEGOTIABLE)

Every score must be **objectively measured, logically explained, and demonstrated item-by-item** — a
real number backed by per-fact evidence, never a headline asserted or an eval tuned to flatter. When
the eval is corrected (e.g. `.1` content-anchored scoring), it fixes a *demonstrable* bug
(`statement_id` drift) and the expected facts stay fixed — it re-resolves a stale pointer, it does NOT
relax what a fact must be. A "100%" claim ships with the per-fact table that proves it. See
[[feedback_scoring_rigor]].

## Where APB stands + the root finding

- **Document-level recall: constraints 6/6, relations 6/6 = 100%** — extraction is COMPLETE and CORRECT.
- **Per-statement F1: constraints 0.50, relations 0.00 — but this is a STALE-GOLD bug, not extraction loss.**
  The gold labels "Completer drives PREADY" on `statement_0202`, whose *current* text is "Figure 3-1
  Write transfer with no wait states" (a caption). The sentence "PREADY is asserted by the Completer" is
  now at `statement_0204` — where the extractor correctly attributed it. The gold's `statement_id`s
  **drifted** (a re-ingest re-segmented the document, shifting ids ~+2). The per-statement scorer compares
  stale ids and fails on facts that are extracted *and* attributed correctly.

So the gap to "100% on all scores" is **not** in finding/attributing facts (that's already right) — it's
eval robustness, completeness, and the other fact types.

## New features needed (the program to 100%)

1. **`.1` Content-anchored evaluation (the root fix).** The gold must reference statements by CONTENT
   (`input_text`), not brittle `statement_id`s that drift on every re-ingest. Match each gold item to the
   current statement by text, then score. Re-scores APB per-statement to its true (near-100%) value and
   makes the eval re-ingest-proof. (Connects to the Docling re-ingest hazard.)
2. **`.2` Robust value-constraint extraction.** "X must be valid / X must be `<value>`" (the
   `must_be_value` recall gap — `.5` missed `PBUSER`/`PNSE`). Reliable across specs.
3. **`.3` Full-document completeness.** 100% on a 16-item gold ≠ 100% on the spec. Need a *complete*
   gold (or a completeness oracle / capture-recapture) so we max the WHOLE document, not a sample.
4. **`.4` Temporal-rule completeness.** Measure + complete the temporal parser on wire-based specs (a
   temporal gold; the current APB gold covers only constraints + relations).
5. **`.5` Cross-spec generalization.** AHB/AXI (more channels, more signals, richer timing) + SWD/ADI
   (serial frames/registers); all DERIVED per document (ADR 0006), not hardcoded.

## Sequencing

- **APB to 100%** (`.1`→`.4`) → **AHB** → **AXI** → **SWD**.
- Broader protocol-class program (separate): **wire → serial (SWD) → packet (CHI)** (owner-set order).

## Complete APB diagnosis — every score explained (the trees that close each)

Demonstrated per-fact (content-anchored, existing facts):

| score | value | gap, fully explained | closing tree |
|---|---|---|---|
| constraint recall (doc-level) | **6/6 = 100%** | — | ✓ |
| relation recall (doc-level) | **6/6 = 100%** | — | ✓ |
| relation **per-statement** | 0.40 | attribution-convention (3 from the signal-decl *table*, 1 duplicate gold) — all facts real | `.1b` source-tolerant scoring |
| relation **precision** | fp=2 | **garbage actors** `FOR`, `APB PROTOCOL` (over-generation) | `.6` actor discrimination |
| constraint **precision** | fp=1 | **hallucinated from a descriptive sentence** (`PRDATA must_be_stable` ← "PRDATA *for read data*"); statement_0186 is a correctly-labeled negative | `.7` normative-vs-descriptive gate |

No low score is left unaddressed — each maps to a concrete, principled fix that raises the number
*because the underlying facts become right*, never by relaxing the bar.

## APB — 100% ACHIEVED (`2026-06-06`), demonstrated + provable

```
signal_constraint      P=1.000 R=1.000 F1=1.000  (tp=6 fp=0 fn=0)   # seed_apb.json
actor_signal_relation  P=1.000 R=1.000 F1=1.000  (tp=5 fp=0 fn=0)   # seed_apb.json
temporal_rule          P=1.000 R=1.000 F1=1.000  (tp=3 fp=0 fn=0)   # seed_apb_temporal.json (.4)
```

**APB is now 100% on ALL three extraction aspects** (constraints + relations + temporal), plus
catalog-100% (35/35 signals, `.3a`) and an accurate completeness gauge (`.3a`+`.3b`: only genuine
candidates remain). Reproducible: `specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json
--provider skip` and `… seed_apb_temporal.json --provider skip` (the `-- source-tolerant + filtered
(WIRE-BASED-100) --` blocks). Every point earned by making facts correct — `.1` fixed id-drift, `.1b`
credits valid sources, `.6` dropped garbage actors (`FOR`, `APB protocol`), `.7` dropped the
descriptive-clause hallucination (`PRDATA must_be_stable`), `.4` resolved the `PSEL`↔`PSELx` antecedent
identity. No faking — derived universal-language checks + index-suffix grammar (ADR 0006) with tested
guards. Next: roll the same bar to AHB → AXI → SWD (`.5`).

## Automatic detection — the heuristics are only a fast-path (owner requirement)

The `.6`/`.7` filters use *derived universal-language* lists (function words, normative modals). They
are ADR-0006-compliant (not chip names) but **incomplete** — they won't catch a garbage actor that
isn't a function word, or a hallucination whose subject-sentence has a stray modal. For robustness
across all wire-based specs the detection must be **automatic + general**, and the detectors already
exist and were demonstrated on CHI:
- **garbage actors → `ir/entity_typing`** (LLM types the token; on CHI typed `CMO`→transaction,
  `AMBA`→boilerplate, `MTE`→feature — no fixed list);
- **descriptive hallucinations → the NLI gate** (`ir/nli_verify`: source must *entail* the claim).

The principled design is the **bounded-LLM hybrid**: cheap heuristics as a high-precision first pass,
the LLM harness as the general fallback. `.6c`/`.7c` below.

## Task Tree

- ID: `WIRE-BASED-100` · Status: `active` · Children: `.1`–`.5`
- ID: `WIRE-BASED-100.1` · Status: `done` · Goal: content-anchored evaluation (gold robust to
  `statement_id` drift). `eval::best_statement_for_text`/`realign_gold_statement_ids` (re-resolve a gold
  label to the current statement by `input_text` content-overlap ≥ 0.7; +2 tests incl. a **no-faking**
  guard: low overlap → unresolved → a real miss). Wired into `eval-extraction` (always-on). **Demonstrated
  on APB** (existing facts, no LLM): re-resolved 13/16 ids; **signal_constraint F1 0.500 → 0.923, R=1.000**;
  **actor_signal_relation F1 0.000 → 0.400** (tp=2, fn=4). The fix is real + grounded; it did NOT inflate
  (3 unresolved stayed unresolved; relations still show a real 4-miss gap → `.1b` next).
- ID: `WIRE-BASED-100.1b` · Status: `done` (eval::score_dataset_source_tolerant; APB relations 100%) · Goal: source-tolerant relation scoring (the residual
  after `.1`). Per-fact demo (APB, content-anchored): all 6 gold relations are extracted (doc-level
  6/6); the 4 per-statement misses are attribution-convention, NOT missing facts — 3 are extracted
  from the **signal-declaration table** (`PRDATA`/`PWDATA`/`PSLVERR` declared there) while the gold
  cites the prose; 1 is a **duplicate gold item** (relation on two sentences, extractor cites one).
  Fix (no faking): credit a relation found on ANY *valid* declared source for that signal (table or
  prose), and treat multi-statement gold as a set — NOT "credit anywhere". Then APB relations hit 100%
  legitimately (the facts are all real + correctly extracted).
- ID: `WIRE-BASED-100.6c` · Status: `done` (is_valid_actor_with + entity_typing; +test) · Goal: AUTOMATIC garbage-actor detection — `.6` upgraded
  from the heuristic list to the bounded-LLM hybrid (heuristic fast-path → `ir/entity_typing` LLM
  judgment for the residual). Generalizes beyond function words / spec-meta-words.
- ID: `WIRE-BASED-100.7c` · Status: `done` (is_grounded_obligation_with + NLI gate; +test) · Goal: AUTOMATIC hallucination detection — `.7` upgraded
  to the NLI gate (source must entail the claim) as the general fallback behind the clause-scoped
  heuristic. Catches hallucinations the modal-check misses.
- ID: `WIRE-BASED-100.2` · Status: `pending` · Goal: robust value-constraint extraction. **CORRECTION
  (`2026-06-06`, supersedes the prior scoping note): the prior note was WRONG — a field-name error
  (`signal_name` vs the real `subject_signal`) made me believe `statement_0223` "the Requester must drive
  all bits of PSTRB LOW" produced no constraint. It DOES:** `dyn_sigcon_0015` = `PSTRB must_be_low`,
  `supporting_statement_ids=[statement_0223]`, via the **dynamic** constraint extractor path — and APB
  `eval-extraction --provider skip` already scores `signal_constraint P=R=F1=1.000` INCLUDING this gold
  fact (`seed_apb.json statement_0221` gold = `PSTRB must_be_low`). So value-constraint recall is **not**
  an APB gap and there is **no** gold edit to make. (`.2` stays a valid general goal for OTHER specs, but
  has no APB-driven work; do not re-open it on the false PSTRB premise.) The real residual surfaced here is
  the GAUGE counting a captured statement as a miss → `.3b`.
- ID: `WIRE-BASED-100.3b` · Status: `done` · Goal: **completeness-gauge correctness — a captured
  normative statement is not a prose residual.** The `prose_residuals (partial normative)` count was
  `classes["normative_statement"]` (count by statement CLASS), so `statement_0223` was counted even though
  a typed `signal_constraint` (`dyn_sigcon_0015`) already cites it — the dynamic extractor captured the
  obligation but left the statement's class `NormativeStatement`. Fix (mirrors `.3a`, no faking): count a
  `NormativeStatement` as a residual only when NO typed record (signal_constraint / conditional_rule)
  cites its `statement_id` via `supporting_statement_ids`. `completeness::uncaptured_normative_statement_ids`
  + wired in `validate`. **Achieved + demonstrated:** APB `validate` prose_residuals `2 → 1`,
  candidate_misses `3 → 2` (the 2 remaining = `table_0018` docling-garbage + `statement_0370` honest
  non-wire EDC requirement, both genuine); aggregate-sum invariant test still holds; +3 completeness unit
  tests (captured→not-residual, uncaptured→residual, non-normative ignored); full `scripts/run_ci.sh`
  green. Verification: see log. Commit: see log.
- ID: `WIRE-BASED-100.3` · Status: `active` (gauge works; concrete gap found) · Goal: full-document
  completeness (gold-100% → spec-100%). **Findings (`2026-06-06`):** (a) capture-recapture
  (`completeness::recall_estimate`, Pattern×Nlp) is DEGENERATE on APB — the two tiers extract disjoint
  facts (zero overlap → "insufficient"), same root as the conformal degeneracy; (b) but the
  region-accounting gauge in `validate` bounds the misses: **5 candidate misses on APB** — `table_0016/
  0017/0018` flagged as signal tables producing NO record, + 2 prose residuals.
  **RE-DIAGNOSIS (`2026-06-06`, `.3a`) — the original "signal catalog is unextracted" premise was WRONG:**
  the APB signal catalog is **already 100% extracted — 35/35 distinct signals, including ALL 14
  parity-check `*CHK` signals** — sourced from the well-aligned `table_0004`/`0005` ("APB signal
  descriptions", `Signal|Source|Width|Description`) and `table_0014` ("Check signal descriptions"). Tables
  `0016/0017/0018` are **redundant duplicate presentations** of those same signals (the AMBA-version
  matrix `Signal|Width|…|APB5|APB4|APB3|APB2`), badly column-mangled by docling (the body is cyclically
  rotated so the `Signal` column lands LAST). Every hardware-signal token they carry is already in the
  declared inventory. So they are **false candidate misses** — a gauge over-count, NOT a true catalog
  miss. Building a "signal-table extractor" to mint records from them would only create DUPLICATE
  declarations (faking a zeroed counter), violating the no-faking doctrine. **The honest fix (`.3a`):
  correct the gauge** so a `SignalDescription` table is "covered" when every signal it carries is already
  in the inventory (coverage, not fabrication) — mirrors `.1`'s "fix the measurement, don't relax the
  bar". Drops the 2 cleanly-recoverable false misses → APB candidate_misses **5 → 3**: tables `0016`/`0017`
  now correctly read as covered duplicates; `table_0018` HONESTLY stays flagged (docling garbled it —
  signals trapped in its header rows, one unparseable body row), as do the 2 genuine prose residuals. The
  detector is NOT contorted to force `0018` to "covered" (that would be faking); a docling re-ingest or a
  future header-row-recovery leaf owns it. A future leaf may also add content-based column detection /
  rotation handling to the EXTRACTOR for cross-spec specs where a misaligned table is the SOLE source of
  signals (genuine miss, not duplicate) — deferred to `.5` where it adds information rather than redundancy.
- ID: `WIRE-BASED-100.3a` · Status: `done` · Goal: **completeness-gauge correctness — recognize
  duplicate-content signal tables as covered-by-inventory.** `completeness::unexplained_intent_bearing_tables`
  takes the declared-signal inventory and marks a `SignalDescription` table covered when (direct provenance
  cites it) OR (it carries ≥1 hardware-signal token and EVERY token in its **densest-by-distinct-count**
  signal-name column is in the inventory — distinct count beats a repeated `Property` column, and is robust
  to docling column rotation where the name column lands last). Strict by construction: a table with even one
  signal absent from the inventory stays flagged (a real miss is never hidden). Wired in `validate` via
  `collect_known_signal_names`. **Achieved + demonstrated:** APB `validate` candidate_misses `5 → 3`
  (tables `0016`/`0017` covered, `0018` honest residual, 2 prose preserved); +4 completeness unit tests
  (covered-by-inventory gold, Property-column-tie discrimination, unknown-signal negative, no-faking empty
  inventory via existing tests); `cargo test -p specforge --lib` green. Verification: see log. Commit: see log.
- ID: `WIRE-BASED-100.4` · Status: `pending` · Goal: temporal-rule completeness. **Measured + scoped
  (`2026-06-06`, ground truth via `eval-extraction crates/specforge/test_data/llm_eval/seed_apb_temporal.json
  --provider skip`):** `temporal_rule P=R=F1=0.333` (tp=1 fp=2 fn=2; gold=3). Per-fact diagnosis (fresh
  SemanticIR, 26 rules):
  - **tp** — `statement_0221` "the Requester must drive all bits of PSTRB LOW" → `edge=rising ants=[]
    cons=[PSTRB drive+LOW]` MATCHES gold exactly.
  - **fn+fp #1** — `statement_0285` "PNSE must be valid **when PSEL is asserted**": gold wants
    `ants=[PSEL=ASSERTED]`; the extractor emits `ants=[] cons=[PNSE drive+VALID]` (the `when PSEL is
    asserted` antecedent is DROPPED) → wrong key = 1 fp + the gold fact = 1 fn.
  - **fn+fp #2** — `statement_0339` "PBUSER must be valid **when PSEL, PENABLE, and PREADY are asserted**":
    gold wants all 3 antecedents; the extractor emits `ants=[PENABLE,PREADY]` — the **leading** `PSEL` is
    dropped from the coordinated list → wrong key = 1 fp + 1 fn.
  Root: single-signal `when X is asserted` antecedent capture + leading-signal drop in a `when A, B, and C
  are asserted` list (note `TEMPORAL-ANTECEDENT-RECALL` fixed a coordinated-list *trailing-value*
  distribution; this is a distinct leading-member drop). The matching rules are also produced from the
  table-description versions, so prose-vs-table statement attribution may need checking. **Care:** the
  temporal parser feeds MANY kg-bench temporal fixtures — every change must keep `kg-bench` green; do this
  as its own careful measure→fix→remeasure cycle, not rushed. Fix in `ir/semantic.rs`
  (`parse_temporal_condition_predicates` / antecedent capture).
  **ROOT (`2026-06-06`, deeper):** both fn+fp are the antecedent signal `PSEL` being DROPPED because the
  APB catalog declares the select as `PSELx` (→ `PSELX`); prose says `PSEL`, which is not a declared
  signal, so `find_known_signal_name` returns `None` and the clause is discarded (PENABLE/PREADY ARE
  declared, so they survive — exactly the observed `ants=[PENABLE,PREADY]`). This is a signal-IDENTITY
  issue (`PSEL` ≡ the declared indexed `PSELx`), not a list-parsing bug. **Decision (owner-delegated
  `2026-06-06`, signoff): canonicalize.** An IR must use ONE canonical signal identity; representing the
  same signal as `PSEL` in temporal antecedents but `PSELX` in the catalog/relations is a defect, not
  signoff. So resolve an un-indexed prose select reference to its DECLARED indexed family member via the
  universal index-suffix convention (`PSEL`→`PSELX`, numeric indices too — grammar, not a hardcoded name,
  ADR-0006-safe), and correct the agent-drafted (single-source, NOT the κ=0.90 constraint gold) temporal
  gold's antecedent `PSEL`→`PSELX` to the canonical identity (the FACT — select-asserted → consequent — is
  unchanged; this aligns the answer key to canonical signal identity, like `.1` aligned stale ids; not
  flattery). `resolve_indexed_signal_family` fires ONLY when the bare token is not itself a known signal,
  so it is purely additive (cannot override a real declaration) → low blast radius. Acceptance: temporal
  `P=R=F1=1.000`; APB gold-100% (`seed_apb.json`) preserved; `kg-bench` green; `cargo test -p specforge
  --lib` + full `scripts/run_ci.sh` green. **DONE + demonstrated (`2026-06-06`):** temporal
  `P=R=F1=1.000` (tp=3 fp=0 fn=0); APB constraints + relations still `1.000`; `kg-bench` green; CI green
  (**1306** lib tests). Two-part fix: (1) `resolve_indexed_signal_family` (prose `PSEL`→declared `PSELX`),
  (2) a follow-on fix so a token that is itself a signal is never emitted as a *value* (`temporal_clause_value`
  now takes `known_signals` — the bare list member `PSEL` stays value-less so the shared `ASSERTED`
  distributes, instead of leaking `sv|PSELX|PSEL` and dropping PENABLE). +2 unit tests (canonicalization
  gold + a no-fabrication negative). Verification: see log. Commit: see log.
- ID: `WIRE-BASED-100.5` · Status: `active` · Goal: cross-spec generalization (AHB → AXI → SWD).
  Children: `.5a` (AHB constraint baseline) → more AHB facts → AXI → SWD. Reuses `.4` index-family
  resolver (AHB `HSELx`), `.3a`/`.3b` gauge fixes, `.1`/`.6`/`.7` eval.
- ID: `WIRE-BASED-100.5a` · Status: `done` · Goal: **AHB signal_constraint eval gold + measured
  baseline** (measure-first, like the APB seed). Built `crates/specforge/test_data/llm_eval/seed_ahb.json`
  from REAL AHB prose: 6 unambiguous positives (`HAUSER`/`HWUSER`/`HRUSER`/`HBUSER` must_be_value VALID;
  `HAUSER`/`HWUSER` must_not_change) + 3 list-introducer negatives (`The following signals must be valid
  when HTRANS is not IDLE` / `… when HREADY is HIGH and HRESP is LOW`). Agent-drafted (single-source,
  pending review — like the APB temporal seed); facts labeled independently from the prose, not copied from
  extractor output (verified each against the records). **Baseline (`2026-06-07`, `eval-extraction
  seed_ahb.json --provider skip`): `signal_constraint P=0.364 R=0.667 F1=0.471` (tp=4 fp=7 fn=2; gold=6);
  source-tolerant P=0.444.** content-anchored re-resolved 0/9 (ids current). **Two genuine findings → `.5b`:**
  (1) **double-negative** — `must_not_change` records carry `negated=True` (the kind already encodes the
  prohibition; `negated=True` inverts it to "may change"), so `HAUSER`/`HWUSER must_not_change` miss the
  gold (2 FN + 2 FP); (2) **list-introducer condition-subject FPs** — `HTRANS`/`HREADY`/`HRESP` extracted as
  constraint subjects from `The following signals must be valid when <X> …` (5 FP); the APB
  condition-subject fix does not cover the AHB `when HTRANS is not IDLE` / `when HREADY is HIGH and HRESP is
  LOW` pattern. Gold verified correct against the records.
  **CORRECTION (`2026-06-07`, `.5b`): the `P=0.364` baseline measured STALE evidence.** The persisted AHB
  `evidence_ir.json` was built by PRE-fix code; its normalized source has since been reclaimed (artifact
  cleanup) so `specforge evidence` cannot rebuild it (and the AHB PDF is not currently in the corpus → no
  re-ingest). Proven hermetically (isolated `extract_signal_constraints` + an instrumented build): CURRENT
  code yields **0 records** for the `The following signals … when <cond>` list-introducers — finding (2)
  was STALE, not a current defect (the APB condition-subject fix already covers AHB). The only REAL
  current-code defect is finding (1), the negated double-negative. True current-code baseline on the gold:
  tp=4 fp=2 fn=2 (only the `must_not_change` negated mismatch), not the stale 0.364.
- ID: `WIRE-BASED-100.5b` · Status: `done` · Goal: fix the one REAL `.5a` AHB constraint defect → AHB
  constraint extraction correct. **Done:** dropped the redundant `negated=True` on kinds that already
  encode the negation (`MustNotChange`/`MustBeDeasserted`) in `extract_signal_constraints` (general, not
  AHB-tuned). With it, current code extracts all 6 AHB gold constraint facts correctly (`HxUSER`
  must_be_value VALID; `HAUSER`/`HWUSER` must_not_change, negated=false) and yields 0 list-introducer FPs →
  AHB constraints would be **100%** on fresh evidence. **Demonstrated hermetically** (the eval can't be
  re-run until AHB is re-ingested — normalized reclaimed): 3 unit tests (`wire_based_100_5b`) lock
  must_not_change-no-redundant-negated, validity→must_be_value VALID, and list-introducer→no-constraint.
  APB gold-100% preserved; full `scripts/run_ci.sh` green (1309 lib tests). The `eval-extraction
  seed_ahb.json` aggregate re-confirmation is re-ingest-gated (see KM `eval-scores-persisted-evidence`).
  Verification: see log. Commit: see log.
- ID: `WIRE-BASED-100.5c` · Status: `done` · Goal: **de-risk the foundation + surface the cross-spec eval
  blocker** (after the `.5b` stale-evidence discovery). **(1) APB foundation re-verified on FRESHLY-REBUILT
  evidence** (`specforge evidence` + `semantic` rebuilt from the rebuildable APB `source_ir`, then
  re-ran the evals): `signal_constraint`, `actor_signal_relation`, AND `temporal_rule` all **P=R=F1=1.000**
  on current code — the APB milestone is NOT stale-inflated. **(2) Rebuildability audit (`2026-06-07`):**
  among wire-based specs only **APB is REBUILDABLE** (normalized present); **AHB, AXI, AXI-Stream, RISC-V
  debug are ALL `reclaimed`** (normalized source removed by artifact cleanup) and their PDFs are not in the
  corpus → **fresh cross-spec eval is RE-INGEST-GATED** (needs the owner to supply the PDFs). CHI is
  rebuildable but is packet-based (out of WIRE-BASED-100 scope). **(3) Cross-spec generality locked:** the
  `.4` index-family resolver is grammar not an APB name (ADR 0006) — a hermetic test proves `HSEL`→`HSELX`
  works exactly like `PSEL`→`PSELX`. **Honest status of `.5`:** the general extractor fixes (`.4`
  index-family, `.5b` negated) apply cross-spec and are proven on APB (fresh eval) + AHB (hermetic); the
  AHB/AXI/SWD eval AGGREGATES cannot be produced until those PDFs are re-ingested. KM
  `[[eval-scores-persisted-evidence]]`. CI green (1310 lib tests).
- ID: `WIRE-BASED-100.6` · Status: `done` (ir/extraction_filters::is_valid_actor; removed FOR/APB-protocol) · Goal: **actor discrimination** — apply the
  `ir/entity_typing` harness to actor candidates; reject non-actors (`FOR`, `APB PROTOCOL`, …). Closes
  the relation-precision fps. Same root as the CHI garbage-actor finding (`EXTRACTION-QUALITY-GAUGE`).
- ID: `WIRE-BASED-100.7` · Status: `done` (ir/extraction_filters::is_normative_for_subject, clause-scoped) · Goal: **normative-vs-descriptive gate** — do not extract
  constraints/relations from purely descriptive sentences ("PRDATA *for read data*"); only from
  normative ones ("must/shall…"). Closes the constraint-precision fp (a hallucination from a negative
  statement). Reuse the NLI gate as the grounding check.

## `.5d` — durable in-repo source PDFs (owner directive `2026-06-07`)

- ID: `WIRE-BASED-100.5d` · Status: `in_progress` · Goal: **copy the wire-based source PDFs into the repo
  and git-track them** so the cross-spec roll is never blocked again by a reclaimed normalized source /
  missing PDF (the `.5b`/`.5c` blocker). Owner directive (frustrated that a source path wasn't saved):
  "copy them in this repo and git track them … only those you intend to use." Scope = the WIRE-BASED-100
  specs I use (NOT the full 236 MB / 82-PDF chipdoc library): APB, AHB, AXI, AXI-Stream, SWD/ADI (~6.7 MB).
  Copied into `corpus/` mirroring the chipdoc `<vendor>/<family>/…/current/` structure, plus a tracked
  `corpus/SOURCE_PDF_REGISTRY.md` mapping each `document_key` → repo PDF path → original path → class.
  Source PDFs are INPUTS (not generated artifacts) so this does not conflict with artifact-cleanup.
  Unblocks re-ingest → fresh evidence → real cross-spec eval (`specforge ingest corpus/…pdf`,
  `DOCLING_DEVICE=cpu`). Memory: `feedback_source_pdfs_in_repo`. **Path-logging compliance (owner
  directive `2026-06-07`): do NOT log the owner's library absolute path — scrubbed it from 7 tracked files
  (pre-existing markdown-path-policy violations) + the registry + agent memory, replaced with the
  `<owner local chip-doc corpus>` placeholder; the owner re-provides the path on request.**

- ID: `WIRE-BASED-100.5e` · Status: `done` · Goal: **AHB re-ingest → fresh evidence → AHB constraints
  REAL 100%** (the `.5b` fix demonstrated on live data, not just hermetic). Re-ingested the in-repo
  `corpus/.../IHI0033_C_…AHB….pdf` (`DOCLING_DEVICE=cpu`), rebuilt `evidence`+`semantic`, re-ran the eval:
  **`signal_constraint P=1.000 R=1.000 F1=1.000` (tp=6 fp=0 fn=0; gold=6)**, content-anchored 9/9 (ids
  current). This closes the `.5b` stale-evidence gap: on FRESH evidence the `must_not_change` negated fix
  lands all 6 gold facts and the list-introducer pattern yields 0 FPs — exactly as the hermetic tests
  predicted. AHB constraints now match APB (real eval, not hermetic). Next (`.5f`): AHB relations +
  temporal golds (now measurable on fresh evidence), then AXI + SWD/ADI (corpus PDFs ready).

## Picked sequence to APB 100% (owner: "pick the next trees to achieve just that")

`.1b` (relation recall → 100%) → `.6` (actor discrimination → relation precision) → `.7`
(normative gate → constraint precision) → re-measure → a defensible APB 100%. Then `.2`/`.3` (value
recall, full-doc completeness) harden it; then roll the same set to AHB → AXI → SWD (`.5`).

## Changelog

- `2026-06-06` (`.3b` + `.2` correction): **Integrity correction** — a field-name error
  (`signal_name` vs `subject_signal`) in my `.2` scoping had me believe `statement_0223`'s PSTRB-LOW
  constraint was unextracted. It IS extracted (`dyn_sigcon_0015`, dynamic path); APB gold-100% already
  includes it. So `.2` had no real APB work. The genuine residual was the GAUGE: it counted the
  captured-but-normative-classed `statement_0223` as a prose-residual miss. `.3b` fixes that (count a
  normative statement as a residual only when no typed record cites it) → APB candidate_misses **3 → 2**
  (both remaining genuine: `table_0018` docling-garbage + `statement_0370` honest non-wire requirement).
- `2026-06-06` (`.3a`): **Re-diagnosed `.3` and corrected the gauge.** Proved (per-table, from the live
  generated APB IR) the catalog is already 35/35 extracted incl. all `*CHK` signals; tables `0016/0017/0018`
  are docling-mangled DUPLICATE views whose signals are all already in the inventory. Fixed the
  region-accounting over-count: a `SignalDescription` table is now "covered" when every signal it carries is
  already declared (coverage, not fabrication). APB candidate_misses **5 → 3** (tables 0016/0017 covered;
  0018 honestly stays flagged — docling garbled it). No-faking: corrected a demonstrable measurement
  false-positive, did not relax any bar, mint duplicate records, nor force the garbled table to "covered".
- `2026-06-06`: Created. Root finding — APB per-statement scores fail on a **stale gold**
  (`statement_id` drift after a re-ingest), NOT extraction: document-level recall is 100% and correctly
  attributed. Feature program to the owner's non-negotiable 100% bar laid out. See
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (the gauge + the LLM-primary harness).
