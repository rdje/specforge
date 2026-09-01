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

- ID: `WIRE-BASED-100.8` · Status: `done` (`2026-09-01`) · Goal: **restore the scoring oracle — `eval-extraction` refuses
  every document in the corpus, so no WIRE-BASED-100 number can currently be re-derived.** Found
  `2026-08-31` while gating `KG-ISF-COMPLETENESS.5.iv.a`, whose inherited gate is "before/after
  WIRE-BASED-100 on rebuilt gold evidence"; that protocol could not be executed at all. **Not caused by that
  slice** — the same failure reproduces on the pre-change `target/release/specforge` built `2026-08-28`.
  **Two failure modes, whole corpus.** The 54 legacy documents (APB `ihi0024_e`, AHB `ihi0033_c`, AXI
  `ihi0022_l`, NVMe, RISC-V debug …) fail as `EvidenceIR schema version 2 is legacy/proofless and
  inspection-only; rebuild it from verified SourceIR`. The 24 current documents (including the SWD/ADI and
  I2C golds, which ARE rebuildable) fail as `EvidenceIR proof verification failed: registered derivation
  'evidence.claim.schema_version.root' output or input topology is stale`.
  **ROOT CAUSE, isolated read-only.** The second mode is not seal staleness. Rewriting only an EvidenceIR's
  `artifact_layout` (`artifact_root` + `evidence_ir_path`) and leaving every other byte identical fails
  canonical verification with that exact diagnostic, while a byte-identical copy that KEEPS its original
  layout verifies and runs (`specforge entity-type` on both; `specforge semantic --dry-run` also replays the
  original fine, and `check_chain_currency.sh` reports 24/24 current). So the EvidenceIR proof binds the
  artifact's own storage LOCATION into a derivation's topology — and `extract_on_copy`
  (`crates/specforge/src/commands/eval_extraction.rs:157-163`) must relocate the artifact into a temp root
  precisely so the corpus is never mutated. Every `eval-extraction` task therefore fails before scoring.
  **Why this is a gate-integrity defect, not a nuisance.** `WIRE-BASED-100`'s governing principle is no fake
  scoring, and `KG-ISF-COMPLETENESS` requires a before/after eval on every extraction change that touches a
  scored document. A published `1.000` that cannot be re-derived on demand is exactly the claim
  `CLAIM_VERIFICATION.md` refuses. The last re-derivation of record is `SWD-SERIAL-EXTRACTION` (`2026-08-09`).
  Acceptance: `eval-extraction --provider skip` runs to a score on at least the two rebuildable golds
  (`ihi0074_a` SWD/ADI, `um10204` I2C) with the relocation defect fixed at its cause rather than by
  mutating the corpus; the legacy-stratum refusal is stated as an explicit UNMEASURABLE disposition with its
  own re-ingest route rather than presented as a score; and a regression control pins that relocating a
  verified artifact preserves verification. **SPLIT (`2026-09-01`), because the two halves are separately
  reviewable exactly as the leaf anticipated:** `.8a` is a kernel-seam change in `EvidenceIr` that has to be
  judged against the proof doctrine, and `.8b` is a command-behaviour change in `eval-extraction` that has to
  be judged against the scoring/honesty doctrine. `.8c` was then opened by what the restored oracle found,
  and `.8d` by what `.8c` localised. Children: `WIRE-BASED-100.8a`, `WIRE-BASED-100.8b`,
  `WIRE-BASED-100.8c`, `WIRE-BASED-100.8d`.
  **CLOSED `2026-09-01`.** `.8a`/`.8b`/`.8c` are `done` and `.8d` is `deferred` with its consequence recorded,
  so every child is resolved. The oracle runs, refuses nothing silently, and its first re-derivation since
  `2026-08-09` retired a published score rather than confirming it — which is the outcome this leaf existed to
  make possible. What `.8` does NOT deliver, stated plainly: the wire golds it was meant to protect (APB, AHB,
  AXI) remain unmeasurable because their EvidenceIR is legacy, so "the oracle is restored" means restored over
  the 24 rebuildable chains only. Re-ingesting the legacy stratum stays with the corpus refresh frontier.
  Non-goal: re-ingesting the 54 legacy chains (owned by the corpus refresh frontier); changing any gold.
  Prerequisite: none. KM `[[evidence-proof-binds-artifact-location]]`.
  Verification: see `.8a`/`.8b`.
  Commit: see `.8a`/`.8b`.

- ID: `WIRE-BASED-100.8a` · Status: `done` (`2026-09-01`) · Goal: **a supported, proof-carrying way to relocate a
  verified EvidenceIR** — the kernel half of `.8`. Today the only way to move an artifact is to rewrite
  `artifact_layout` in place, which silently invalidates the seal: the proof's registered replay is taken
  over `public_field_values()`, and that map *includes* `artifact_layout`, so every claim premise's
  `inputs_sha256` binds the artifact's own storage path. Add `EvidenceIr::load_relocated_to_artifact_base_root`,
  which verifies the artifact where it is (relocation may not launder authority), moves it to
  `<new base>/<document_key>/evidence_ir.json` — the same `<base>/<document_key>` convention
  `build_unproved_from_source_ir` replays, so the independent rebuild still agrees — and re-derives the proof
  for the new location from the same verified SourceIR prefix and the same sealed proof context, mutation
  chain intact.
  Move `extract_on_copy` onto that seam in the same slice, so the seam ships with its only caller and
  `eval-extraction` reaches a score again.
  Acceptance: a hermetic control proves (a) an unsealed `artifact_layout` rewrite is still refused by the
  production read path, and (b) an artifact relocated through the new seam reloads and verifies at its new
  root with its content unchanged; `eval-extraction --provider skip` reaches a score on the rebuildable
  golds; `kg-bench` and the CI suite stay green; the corpus is provably unmutated.
  Non-goal: unbinding the artifact's location from the replay topology (that is the deeper fix; it changes
  the frozen 38-family/170-field producer graph AND invalidates all 24 sealed chains at once — see Decisions).
  Prerequisite: none.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.8b` · Status: `done` (`2026-09-01`) · Goal: **make the legacy stratum an explicit UNMEASURABLE
  disposition** — the honesty half of `.8`. With `.8a` landed the oracle runs, but one refused document still
  aborts the whole run: `build_predictions` propagates the load error, so a dataset naming any of the 54
  legacy chains produces no output at all. A legacy/proofless EvidenceIR must instead be reported as
  UNMEASURABLE with its re-ingest route and its gold items withheld from scoring, never folded into a score as
  false negatives — a refusal presented as `R=0.000` is a fake number in the exact sense this tree forbids.
  Any other failure must still abort: a real defect may not be absorbed into a disposition.
  Acceptance: a legacy-stratum dataset (`seed_apb.json`) prints the UNMEASURABLE disposition, scores nothing for
  that document, and exits successfully; a rebuildable dataset is unchanged in its scored output; a hermetic
  control pins that a non-legacy failure still aborts.
  **CORRECTION while implementing, and it is bigger than this leaf.** The leaf said `ihi0024_e` is "schema 1".
  It is **schema 2**. Censusing every persisted artifact rather than assuming: the 54 legacy documents are
  **SourceIR schema 1 / EvidenceIR schema 2 / SemanticIR schema 1 / IntentIR schema 1**, against 3 / 3 / 2 / 2
  for the 24 current ones — **zero schema-1 EvidenceIRs exist in the corpus**. So "the 54 legacy schema-1
  chains" is true of their SourceIR and false of the EvidenceIR that `eval-extraction` actually refuses. The
  shorthand came from `.8`'s own text and had spread into `MEMORY.md`, `LIVE_ACHIEVEMENT_STATUS.md`, the book's
  extraction-eval chapter, the `.8a` fact card, two SWD cards, and `.5j`'s own correction written hours earlier
  in this same session — all corrected here. The legacy version is per STAGE; naming a stratum by one schema
  number is what made it wrong, which is exactly why the disposition prints the artifact's OWN version.
  Prerequisite: `WIRE-BASED-100.8a`.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.8c` · Status: `done` (`2026-09-01`) · Goal: **the first thing the restored oracle found — the
  published SWD `29/29 at 1.000` is stale, and the live docs still present it as current.** Re-derived
  `2026-09-01` with `.8a`'s working oracle on the rebuildable `ihi0074_a` chain:
  `seed_swd_derivation.json` scores `protocol_operation` 4/4 = 1.000 and `interface_edge_timing` 1/1 = 1.000,
  but `serial_frame_field` **0/11** and `protocol_state` **0/13** — document-level recall 5/29, not 29/29.
  **This is NOT an extraction regression, and it is not caused by `.8a`.** It is the published, deliberate
  effect of `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c` (`89d8dee7`, `2026-08-12`), which replaced the named protocol
  carriers and protocol-gated extractors with schema-2 structural semantics on ADR-0006 genericity grounds and
  said so in its own ledger entry: *"Exact comparison retires 22 fixed-phase frame and four named-operation
  records."* The SWD frame fields were exactly those fixed-phase records; the surviving `protocol_states` also
  lost their `machine_name` binding, so the gold's `machine|state` keys no longer resolve. `extract_serial_frame_fields`
  now admits a field only from a statement carrying a document-stated phase name, and on this document that
  gate is satisfied by 61 statements none of which carry the gold's bit-range fields.
  **Why it stayed invisible for 20 days:** the trade was published in `CHANGES.md`, but the score it retired
  lives in `WIRE-BASED-100.5j`, `SWD-SERIAL-EXTRACTION`, `ROADMAP.md`, and `LIVE_ACHIEVEMENT_STATUS.md`, and
  nothing re-derived it — because the oracle was down from at least `2026-08-28` (`.8`). A genericity trade may
  retire a score; it may not leave the retired score standing as current.
  Acceptance: every live surface that presents SWD serial-frame/state recovery as current carries the
  re-derived number and its cause, with the retired number kept as dated history rather than deleted;
  `CLAIM_VERIFICATION.md` §6 is honoured — both competing explanations are named and separated by the
  `89d8dee7` ledger entry rather than by assuming the newer instrument wins; the residual recall frontier gets
  an owning leaf or an explicit deferral with its consequence.
  Non-goal: restoring the retired records by reintroducing protocol-named extractors (ADR 0006/0035 forbid it);
  changing `seed_swd_derivation.json`, which is a faithful gold whose facts are real.
  Prerequisite: `WIRE-BASED-100.8a`.
  **OUTCOME — the account above was right about the cause and WRONG about the blast radius, in the project's
  favour.** `89d8dee7` did NOT simply leave its retirement unpropagated: it superseded four SWD fact cards
  (`swd-canonical-protocol-artifact-is-current`, `swd-protocol-surfaces-reach-intentir`,
  `swd-protocol-convergence-snapshots-are-exact`, `swd-serial-frame-surface`) and updated four book chapters. The
  defect is a **partial** retirement, and its shape is diagnostic: it missed `swd-derivation-scored-100` — the one
  card whose title asserts the score — plus `swd-adi-not-signal-table-spec`, `swd-intent-is-the-fsm-driving-swdio`,
  `ROADMAP.md`, `docs/book/src/quality/extraction-eval.md` (the chapter that publishes the number), and both owning
  task trees. **It retired the artifact-authority surfaces and missed the score-assertion surfaces.** That is a
  reusable rule, not an anecdote: when a change retires a producer, the surfaces to hunt are the ones publishing
  its NUMBER, which are rarely the ones describing its ARTIFACT.
  **And the cause is stronger than "a genericity trade":** the retired `extract_serial_frame_fields` switched
  itself on for any document containing `serial wire`/`packet request`/`shift-dr`/`swdio`/`swclk` and sorted
  fields with a fixed `SerialFramePhase {Request, Acknowledge, Data}` enum keyed on `wdata`/`rdata`/`datain`/
  `ack[`. ADR 0006 forbids that outright, so the 29/29 was never evidence of generic capability — it measured a
  protocol recogniser. Retiring it was mandatory, not a trade.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.8d` · Status: `deferred` (`2026-09-01`, measured read-only; **its own proposed fix is
  DISPROVEN**) · Goal: **recover the retired frame fields generically — bind a document-stated phase to the
  fields in its scope, not only within one sentence.** `.8c` localised the residual
  exactly: `extract_serial_frame_fields` admits a field only when ONE statement both passes `stated_phase_name`
  and parses a bit range / named-bit list. SWD carries 61 statements with a stated phase name and writes
  `A[3:2]`, `WDATA[31:0]`, `RDATA[31:0]` in different statements, so the conjunction never holds and the surface
  is empty. Corpus-wide the current grammar emits **0** serial frame fields across all 24 measurable documents,
  so this is not an SWD quirk; but the one document that would exercise the composition-list path
  (`PDF-VARIANT-DIGESTION.9.3b`, the CAN specification) is a legacy chain and unmeasurable, so "0
  everywhere" must NOT be read as "the grammar is dead" — it is untested on its designed input.
  The admissible fix is scope binding: a phase named in a section or paragraph binds the fields inside that
  scope. That is document grammar, so ADR 0006 admits it; a phase VOCABULARY would not be.
  Acceptance: SWD frame recall rises from 0/11 on a rule stated structurally and demonstrated on at least one
  identity-renamed control; no protocol, vendor or signal name enters production; APB/AHB/AXI/I2C scores and
  `kg-bench` are unchanged; whatever residual remains is measured and published rather than rounded away.
  Prerequisite: `WIRE-BASED-100.8c`.

  **MEASURED `2026-09-01`, READ-ONLY — SCOPE BINDING DOES NOT WORK, AND THE MEASUREMENT IS THE DELIVERABLE.**
  The fix this leaf proposed was written from a plausible mechanism, not from evidence, and the evidence refutes
  it. Two candidate scopes were measured against the 11 gold frame facts on the current `ihi0074_a` EvidenceIR:
  - **Nearest preceding phase-stating statement.** It resolves for every field, so it would *fire* — and it
    would be WRONG for at least 7 of 11. `APnDP`/`RnW` (gold `request`) would inherit `transfer`;
    `Start`/`Parity`/`Stop` (gold `request`) would inherit `transfer` from 28–44 statements back; `Park` (gold
    `request`) would inherit `data`; `A`, `ACK` and `DATAIN` sit **278, 303 and 324** statements after the
    nearest one, which names `response`. A rule that fires everywhere and is right a third of the time is
    fabrication with a structural alibi.
  - **The owning section's title.** Resolving each gold statement's span to its section anchor: `Packet
    requests` → `request` ✓ (`APnDP`, `RnW`) and `Data transfers (WDATA and RDATA)` → `data` ✓ (`WDATA`,
    `RDATA`), but `Start`/`Parity`/`Stop` sit under `B4.2 SWD protocol operation`, `Park` under `B4.2.5 Protocol
    error response`, `A` under `Attributes`, `ACK` under a table caption and `DATAIN` under `OK or FAULT
    response to a DPACC or APACC access`. At most 4 of 11 land, and no title even contains the word `phase`.
  **WHERE THE BINDING ACTUALLY LIVES — and it explains the retired extractor.** For `Start`/`Parity`/`Stop`/
  `Park`/`A` the document never states the phase in text at all; the frame's field-to-phase membership is drawn
  in **`Figure B4-1 SWD successful write operation`** and `Figure B4-2`. Both are captured — `picture_0038` and
  `picture_0039` carry the caption — but their role is `ambiguous` and their only observation is that caption,
  so the diagram's content was never read. That is why the retired `extract_serial_frame_fields` scored 11/11:
  it did not read the frame, it keyed the phase off the FIELD NAME (`wdata`/`rdata`/`datain`/`ack[`), i.e. it
  carried SWD's field→phase table in the code. No prose-level rule can replace a lookup that was never a
  reading.
  **DEFERRED with its consequence stated:** SWD frame recall stays **0/11** and the gold's 11 frame facts stay
  honestly unreachable. Nothing is minted, because minting a wrong phase is worse than a measured zero. The
  score is published as-is by `.8c`.
  **Re-open trigger:** figure-content extraction reaching this class of diagram — the pipeline already has the
  typed carrier (`VisualObservationKind::TimingDiagramExtraction`) and the assets are already captured, so the
  gap is the extraction pass, not the schema. A slice that reads `picture_0038` into typed per-signal frame
  structure makes this leaf buildable and measurable in one step.
  Non-goal (restated, now with evidence): any rule keyed on field NAMES, which is what the retired code was.
  Verification: read-only measurement over `generated/evidence_ir/ihi0074_a_.../evidence_ir.json` and
  `crates/specforge/test_data/llm_eval/seed_swd_derivation.json`; no code changed, no artifact written.
  Commit: see log.


## Acceptance Checklist (enforced) — `WIRE-BASED-100.8a` — DONE `2026-09-01`

- [x] **REPRODUCE / MEASURE** — the oracle is down on every document, and it is not this session's binary:
  `./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip`
  on the pre-change `2026-08-28` release binary prints the header, the dataset line, and then
  `error: invalid stage artifact: EvidenceIR proof verification failed: registered derivation
  'evidence.claim.schema_version.root' output or input topology is stale` — no score at all. Baseline for
  every WIRE-BASED-100 number: **not re-derivable**, last re-derivation of record `2026-08-09`.
- [x] **ROOT CAUSE (WHY + WHERE)** — isolated read-only, and it is the artifact's own recorded location, not
  its content. Two copies of `generated/evidence_ir/um10204_.../evidence_ir.json` were placed under
  `.project-data/tmp/evprobe`: **(A)** convention-preserving (`<base>/<document_key>/evidence_ir.json`) and
  **(B)** flat (what `extract_on_copy` did), each with ONLY `artifact_layout.artifact_root` and
  `artifact_layout.evidence_ir_path` rewritten and every other byte identical. `specforge entity-type` fails on
  **both** with the exact diagnostic above, while the same command on the artifact in place succeeds and prints
  its `4 filtered (non-signal)` census. So the failure is not the flat layout — it is relocation as such.
  Mechanism, at `file:line`: `EvidenceIr::proof_kernel` registers one `evidence.current-replay` derivation over
  `replay_bytes` (`crates/specforge/src/ir/evidence.rs:1416`), and every `evidence.claim.<surface>.<key>`
  derivation takes it as its sole input (`crates/specforge/src/ir/evidence.rs:1424`); `replay_bytes` is
  `serde_json::to_vec(public_field_values())`, and `public_field_values` inserts `artifact_layout`
  (`crates/specforge/src/ir/evidence.rs:1244`). So each claim premise's `inputs_sha256` binds the storage path,
  and `validate_premise`'s `RegisteredDerivation` arm (`crates/specforge/src/ir/derivation.rs:2149`) raises
  exactly `output or input topology is stale` when the recomputed topology differs.
  `extract_on_copy` (`crates/specforge/src/commands/eval_extraction.rs:151`) must relocate, precisely so the
  corpus is never mutated — so every `eval-extraction` task failed before scoring.
- [x] **ADDRESSED (verified)** — added `EvidenceIr::load_relocated_to_artifact_base_root`: verify where the
  artifact is, move it to `<base>/<document_key>/evidence_ir.json` (the convention the independent rebuild
  replays), then re-derive the proof for that location from the same verified SourceIR prefix and the same
  sealed proof context, mutation chain intact; `extract_on_copy` now uses it. **Before → after, per gold, on
  the rebuildable stratum** (`--provider skip`, so this is the deterministic pattern baseline):
  `seed_i2c_signals.json` refusal → `declared_signal` source-tolerant **P=R=F1=1.000 (tp=6 fp=0 fn=0)**,
  complete-gold precision **6/6**; `seed_swd.json` refusal → `signal_constraint` **1.000 (tp=1 fp=0 fn=0)** and
  `actor_signal_relation` source-tolerant **1.000 (tp=1 fp=0 fn=0)**; `seed_swd_derivation.json` refusal →
  `protocol_operation` **1.000 (4/4)**, `interface_edge_timing` **1.000 (1/1)**, `serial_frame_field` **0.000
  (0/11)**, `protocol_state` **0.000 (0/13)**. The last two are NOT caused by this slice and are not a
  regression: they are the published effect of `89d8dee7` (`2026-08-12`) retiring the fixed-phase frame and
  named-operation carriers on genericity grounds, which the down oracle had hidden — owned as `.8c`.
- [x] **NO REGRESSION** — `kg-bench` **156/156** (`fixtures_passed: 156, fixtures_failed: 0`).
  `cargo test --workspace` green: specforge lib **470**, specforge-core lib **1381** (+1, the new relocation
  control), conformance **168**, production-graph **4**, isf round-trip **5**. `cargo fmt --all --check`,
  `cargo clippy --workspace --all-targets -- -D warnings`, and `RUSTDOCFLAGS="-D warnings" cargo doc --workspace
  --no-deps` all clean. **The corpus is provably unmutated**: after three eval runs the two exercised artifacts
  keep their pre-change mtimes (`generated/evidence_ir/ihi0074_a_.../evidence_ir.json` `Aug 30 21:04`,
  `um10204_.../evidence_ir.json` `Aug 30 21:05`), and `scripts/check_chain_currency.sh --check` replays
  **24 replayed / 24 current / 0 stale** at `evidence`, `semantic`, `intent`, and `isf-adapter`. The pinned
  production-genericity flow census moved exactly the three size counters the two new functions add —
  `analyzed_functions` 2,376→2,378, `helper_edges` 14,737→14,758, `decision_sites` 12,696→12,700 — while
  `boundary_rows`, `rule_roots`, `grammar_declassifiers`, `canonical_seams`, `proof_gates`, `trusted_regions`,
  `non_authoritative_regions`, and every `protected_*` count are UNCHANGED: the seam adds no decision authority,
  no trusted region, and no protected construction.
- [x] **GENERICITY (ADR 0006)** — N/A to document identity: the seam is pure artifact-storage plumbing over the
  proof kernel. It reads no document text and branches on no name; its only document-derived value is
  `document_identity.document_key`, used as a path component exactly as `build_unproved_from_source_ir` already
  does, so a renamed document relocates identically.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (the relocation seam and why an unsealed move is refused);
  KM fact card `evidence-proof-binds-artifact-location` updated from "defect, oracle down" to the resolved
  mechanism plus the supported operation; `CHANGES.md`; `MEMORY.md`; `LIVE_ACHIEVEMENT_STATUS.md`.


## Acceptance Checklist (enforced) — `WIRE-BASED-100.8c` (MEASUREMENT + CORRECTION) — DONE `2026-09-01`

- [x] **REPRODUCE / MEASURE** — `./target/release/specforge eval-extraction
  crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip` on the `.8a` binary:
  `protocol_operation` **1.000 (tp=4 fp=0 fn=0)**, `interface_edge_timing` **1.000 (tp=1 fp=0 fn=0)**,
  `serial_frame_field` **0.000 (tp=0 fp=0 fn=11)**, `protocol_state` **0.000 (tp=0 fp=9 fn=13)** — document-level
  **5/29** against a published `29/29 at 1.000`. Content anchoring is not the explanation: 28 of 29 gold
  statement ids already resolve, and spot-checked gold sentences match current statements at ratio 1.00.
- [x] **ROOT CAUSE (WHY + WHERE)** — three legs, each dimensionally different, separating "regression" from
  "retirement" per `CLAIM_VERIFICATION.md` §6. **(1) The artifact is not stale:** `scripts/check_chain_currency.sh
  --check` reports 24 replayed / 24 current / 0 stale at evidence, semantic, intent and isf-adapter, so the zeros
  are the current producer's real output. **(2) The loss partitions along the retirement boundary, corpus-wide:**
  a read-only census of all 24 schema-3 `generated/evidence_ir/*/evidence_ir.json` finds **0 serial_frame_fields
  in every document** and `machine_name` set on **0 of 40** `protocol_states`, while `protocol_operations` total
  **5** (4 SWD + 1 Wishbone) — and `89d8dee7`'s own `CHANGES.md` entry from `2026-08-12` states *"retires 22
  fixed-phase frame and four named-operation records; the generic producer retains five operations and 40
  structurally admitted states."* Five and forty re-derive exactly. **(3) Per-revision producer evidence, not a
  diff reading:** `git show 89d8dee7^:crates/specforge/src/ir/evidence.rs` shows `extract_serial_frame_fields`
  gating the whole document on `l.contains("serial wire") || l.contains("packet request") ||
  l.contains("shift-dr") || l.contains("swdio") || l.contains("swclk")` and then assigning a fixed
  `enum SerialFramePhase {Request, Acknowledge, Data}` from `wdata`/`rdata`/`datain`/`ack[`; that enum is absent
  from today's source. ADR 0006 forbids exactly that class of production decision.
- [x] **ADDRESSED (verified)** — every surface that published the retired number now carries the re-derived one
  with its cause and its date, and the retired number is kept as dated history rather than deleted:
  `ROADMAP.md` (the `SWD-SERIAL-EXTRACTION` bullet), `docs/book/src/quality/extraction-eval.md` (a new
  *"The 29/29 signoff is retired"* section with the current scorecard, plus the stale `swd_operation` task name
  corrected to `protocol_operation`), `docs/tasks/WIRE-BASED-100.md` `.5j`, `docs/tasks/SWD-SERIAL-EXTRACTION.md`,
  the new fact card `swd-serial-frame-score-retired-by-genericity`, `swd-derivation-scored-100` (now
  `status: superseded` with its answers carrying the correction), `swd-adi-not-signal-table-spec`, and
  `swd-intent-is-the-fsm-driving-swdio`. Measured blast radius of the original miss: `89d8dee7` DID supersede four
  SWD cards and update four book chapters, so the defect is a **partial** retirement that hit the
  artifact-authority surfaces and missed the score-assertion surfaces.
- [x] **NO REGRESSION** — docs/measurement only; no Rust behaviour changed and no gold changed
  (`seed_swd_derivation.json` is byte-identical). `scripts/check_doctrines.sh` GREEN including KNOWLEDGE-MAP
  derive-and-diff, LIVE-DOC-SIZE, CLAIM-VERIFICATION and PUBLISHED-ASSERTIONS; the roadmap projection contract
  still passes with `Current strategic priorities` unchanged at 100.0% of its 56-line bound (the correction is
  line-neutral). `kg-bench` 156/156 and the workspace suite were re-earned by `.8a` at this same tree state.
- [x] **GENERICITY (ADR 0006)** — this leaf is the doctrine being enforced rather than applied: it publishes that
  the retired extractor keyed on protocol identity, and it deliberately does NOT restore the records, because the
  only way to restore them at the old score is to reinstate a protocol recogniser. The admissible replacement —
  phase-scope binding — is stated in `.8d` as document grammar with a phase *vocabulary* explicitly excluded.
- [x] **LOCKSTEP** — book, roadmap, both task trees, five fact cards (one new, one superseded, three corrected),
  `CHANGES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `MEMORY.md`.


## Acceptance Checklist (enforced) — `WIRE-BASED-100.8b` — DONE `2026-09-01`

- [x] **REPRODUCE / MEASURE** — with `.8a`'s oracle working,
  `./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider skip`
  printed the header and then died on the first document, producing **no output at all** for the whole run:
  `build_predictions` propagated the load error through `?`. Any dataset naming any of the 54 legacy chains was
  therefore unscoreable end to end, even for its measurable documents.
- [x] **ROOT CAUSE (WHY + WHERE)** — `build_predictions`
  (`crates/specforge/src/commands/eval_extraction.rs`) accepted `F: FnMut(&str, EvalTask) -> Result<TaskRecords>`,
  so an extraction failure had exactly one representation: abort. There was no way to say "this document has no
  score" without either aborting or — worse — letting its gold items fall through to the scorers, where a
  withheld measurement renders as `R=0.000`.
- [x] **ADDRESSED (verified)** — the extractor now returns `TaskOutcome::{Records, Unmeasurable}`;
  `unmeasurable_disposition` probes the artifact's own `schema_version` through the new
  `EvidenceIr::persisted_schema_version` and classifies a below-current artifact as UNMEASURABLE with its
  re-ingest route; `build_predictions` returns the unmeasurable set and stops retrying that document's remaining
  tasks; `run` prints the disposition, reports how many gold items are withheld, and filters them out before
  every scorer. **Before → after on `seed_apb.json`:** a bare abort → `ihi0024_e_2023_02_amba_5_apb_protocol_specification:
  persisted EvidenceIR is schema 2, below the current canonical schema 3; it is legacy/proofless and
  inspection-only. Re-ingest the document … / 16 gold item(s) withheld from scoring` then
  `=== Extraction eval: no measurable document in this dataset ===`, exit 0.
- [x] **NO REGRESSION** — the three rebuildable golds are **unchanged, value for value**, against the `.8a` run:
  I2C `declared_signal` source-tolerant 1.000 (tp=6 fp=0 fn=0) and complete-gold precision 6/6; SWD
  `signal_constraint` 1.000 (1/1) and `actor_signal_relation` source-tolerant 1.000 (1/1); SWD derivation
  `protocol_operation` 1.000 (4/4), `interface_edge_timing` 1.000 (1/1), `serial_frame_field` 0.000 (0/11),
  `protocol_state` 0.000 (0/13). `kg-bench` **156/156**. `cargo test --workspace` green: specforge lib **472**
  (+2, the two new controls), core **1381**, conformance **168**, production-graph **4**, isf round-trip **5**.
  `cargo fmt --all --check`, warning-deny clippy, and warning-deny rustdoc clean. The pinned flow census moved
  only its three size counters — `analyzed_functions` 2,378→2,380, `helper_edges` 14,758→14,763,
  `decision_sites` 12,700→12,705 — with `boundary_rows`, `rule_roots`, `grammar_declassifiers`,
  `canonical_seams`, `proof_gates`, `trusted_regions`, `non_authoritative_regions` and every `protected_*` count
  UNCHANGED, re-derived directly from `cargo run --release -p specforge-production-graph -- --flow --json`.
- [x] **GENERICITY (ADR 0006)** — the disposition is keyed on the artifact's own recorded `schema_version`
  against the binary's current constant; no document, vendor, protocol or document-key list exists anywhere in
  the path, and the message quotes the artifact's own number rather than a hardcoded one.
- [x] **LOCKSTEP** — book `quality/extraction-eval.md` (what the runner does with a document it cannot read);
  `CHANGES.md`; `LIVE_ACHIEVEMENT_STATUS.md`; `MEMORY.md`. No KM card: the mechanism is the existing
  `evidence-proof-binds-artifact-location` fact plus a command-behaviour change the book now documents.

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

- ID: `WIRE-BASED-100.5f` · Status: `done` · Goal: **AHB relation eval gold + measured baseline** (on
  fresh AHB evidence). Added 6 relation items to `seed_ahb.json` from real AHB prose/tables: `Subordinate
  drives HRESP` (0705), `Subordinate drives HREADYOUT` (0311), and the 4 `*USER` drives (`HAUSER`/`HWUSER`
  Manager; `HRUSER`/`HBUSER` Subordinate, table-grounded, scored source-tolerant). **Baseline:
  `actor_signal_relation` source-tolerant `P=0.714 R=0.833 F1=0.769`** (tp=5 fp=2 fn=1; constraints stay
  `1.000`). **Finding → `.5g`:** the gap is ACTOR RESOLUTION, not the signals — the 2 FP + 1 FN are
  `(address, drives, HREADYOUT)` (0311 — anaphora "it" = Subordinate mis-resolved to the noun "address")
  and `(response it, drives, HRESP)` (0705 — "it must drive HRESP" → garbage phrase "response it"). The
  correct `(Subordinate, drives, HRESP)` is also extracted (source-tolerant TP); `HREADYOUT`'s only
  drive-actor is the wrong "address" → FN. Same root as APB `.6`/`.6c` (garbage-actor discrimination) +
  pronoun-subject resolution. `near-miss wrong_actor=2` confirms it. Gold faithful (the hard anaphora 0311
  included on purpose, not cherry-picked away).
- ID: `WIRE-BASED-100.5g` · Status: `done` · Goal: AHB relation actor discrimination → 100%. **Root:
  `extract_subject_phrase` picked the last determiner+noun (`the address`/`the response`), ignoring the
  pronoun subject `it` closest to the verb.** Fix: pronoun-subject **anaphora** — `resolve_pronoun_subject_anaphora`
  detects a bare `it`/`they` subject head (skipping aux/modal/infinitive helpers) and resolves it to the
  clause's FIRST canonical actor role (`is_canonical_actor_role` — generic protocol roles, ADR-0006
  grammar not chip names); wired as a pre-check in `extract_subject_phrase`. This produces the correct
  `(Subordinate, drives, HREADYOUT/HRESP)` AND eliminates the garbage `(address,…)`/`(response it,…)` in
  one move (fixes both the FP and the recall FN). Fires ONLY on pronoun subjects → non-pronoun extraction
  unchanged (low blast radius). **Verified:** 3 hermetic tests (`wire_based_100_5g`: anaphora-it →
  Subordinate ×2 + a non-pronoun no-regression guard); APB gold-100% preserved; kg-bench green; full
  `scripts/run_ci.sh` green (1313 lib tests). **Real-eval CONFIRMED:** re-ingested AHB from `corpus/`,
  rebuilt evidence+semantic, re-ran the eval → **`actor_signal_relation` source-tolerant `P=1.000 R=1.000
  F1=1.000`** (tp=6 fp=0 fn=0; `near-miss wrong_actor=0` — the garbage `address`/`response it` actors are
  gone, HREADYOUT/HRESP resolved to `Subordinate`). AHB constraints stay `1.000`. **AHB is now 100% on
  constraints AND relations** (real eval, fresh evidence), matching APB.
- ID: `WIRE-BASED-100.5h` · Status: `done` · Goal: AHB temporal eval gold + to 100% (the third AHB
  aspect). Built `seed_ahb_temporal.json` (4 items: HAUSER/HWUSER valid [empty antecedent] + HRUSER/HBUSER
  valid [when HREADY HIGH]). **Baseline 0.500** (tp=2 fp=2 fn=2) → root cause traced to TWO defects, both
  fixed: **(1) the `.3a`-deferred EXTRACTOR fix — content-based name-column detection.** `synthesize_signal_declarations`
  now finds the name column by CONTENT (the column with the most distinct hardware-signal tokens) and remaps
  the other header-derived columns by the rotation offset, so a docling-rotated table whose name column is
  last (AHB `table_0009`: `[Name|Destination|Width|Description]` header but the name in the LAST body column)
  still extracts — recovering `HREADY` (its SOLE source) so the temporal antecedent `HREADY=HIGH` resolves.
  Override fires only on a clear content disagreement → aligned tables unchanged (offset 0). **(2) `unless`/`except`
  exception drop in `parse_temporal_condition_predicates`** — "valid when HREADY is HIGH, **unless HRESP is
  ERROR**" → antecedent `HREADY=HIGH` only (the exception is a negative caveat, not a conjunctive condition;
  was wrongly adding `HRESP=ERROR`). **Achieved + demonstrated on FRESH re-ingested evidence: AHB
  `temporal_rule P=R=F1=1.000`** (tp=4 fp=0 fn=0). No regression: APB constraints/relations/temporal all
  still `1.000`, AHB constraints/relations still `1.000`; +3 hermetic tests (rotated-table extract, aligned
  unchanged, unless-drop); full `scripts/run_ci.sh` green (1316 lib tests, kg-bench green). **AHB is now
  100% on ALL THREE aspects (constraints + relations + temporal), matching APB.** Closes the `.3a`-deferred
  misaligned-table extractor work. Next: AXI + SWD/ADI (corpus PDFs ready).
- ID: `WIRE-BASED-100.5i` · Status: `done` (constraints; relations/temporal → `.5k`) · Goal: **AXI
  constraints to 100%.** Re-ingested AXI (`corpus/.../IHI0022_L`, `DOCLING_DEVICE=cpu`); AXI is
  channel-organized (AW/W/B/AR/R/AC, 310 signals — `[[axi-channel-structure]]`). Built `seed_axi.json`
  (4 `must_be_low` activation/system positives + 3 property/doc-meta negatives). **Baseline 0.375** (recall
  1.000, precision killed by property-prose FPs). **Root + fix:** AXI carries heavy property/config prose
  ("RME_Support must be False", "MPAM_WIDTH must be 11", "granted to LICENSEE") whose subjects (`RME`/`GDI`/
  `MPAM`/`LICENSEE`/`AXI`) are NOT declared signals (measured: 27/28 garbage non-declared, 18/18 real
  declared). Fix: **a constraint subject must be a declared signal** (`collect_known_signal_names`),
  applied in BOTH `extract_signal_constraints` AND `extract_dynamic_signal_constraints` (the AXI FPs were
  `dyn_sigcon_`); ADR-0006-safe (the doc's own catalog), gated on a non-empty catalog (kg-bench safe).
  **Achieved on fresh evidence: AXI `signal_constraint` source-tolerant `P=R=F1=1.000`** (tp=3 fp=0 fn=0).
  No regression: APB/AHB constraints still 1.000 (PSTRB-LOW `dyn_sigcon_` survives); +2 hermetic tests;
  full `scripts/run_ci.sh` green (1318 lib tests, kg-bench green). KM `[[axi-constraint-subject-must-be-declared]]`.
- ID: `WIRE-BASED-100.5k` · Status: `done` · Goal: AXI relations + temporal → 100% (AXI is large: 514
  relations / 107 temporal rules). **Catalog-precision fix:** scrambled multi-signal tables
  (`table_0059`/`0187`) + prose put common English words in the name column ("Signal THE is width AWPROT,
  ARPROT"), and `THE`/`HIGH`/`SECURE`/`PHYSICAL`/`INDICATES`/`ASSERTED` pass `is_hardware_signal_token`
  (all-uppercase) → garbage signal declarations that polluted relations (hundreds on `THE`). Fixed by
  extending `is_signal_synthesis_non_signal` with common English / description / logic words (used by both
  synthesis AND the `.5h` content-detection density count; ADR-0006-safe — universal words, not chip
  names) → garbage declarations gone, relations on `THE` = 0, real signals intact, APB/AHB unaffected
  (zero garbage there). **Golds (per-channel, real prose):** `seed_axi.json` +6 relation items
  (AW/W/B/AR/R handshake + data, source-tolerant) → `actor_signal_relation` `P=R=F1=1.000` (tp=6 fp=0
  fn=0); `seed_axi_temporal.json` (ASKSTOP-when-ACTIVATEACK-LOW; SYSCOREQ/SYSCOACK-LOW-when-ARESETn) →
  `temporal_rule` `P=R=F1=1.000` (tp=3 fp=0 fn=0). **AXI now 100% on ALL THREE aspects.** +2 hermetic
  tests; APB/AHB stay 100%; full `scripts/run_ci.sh` green. KM `[[axi-channel-structure]]`,
  `[[axi-constraint-subject-must-be-declared]]`. Three wire-based specs (APB/AHB/AXI) fully done; SWD/ADI
  = `.5j`.
- ID: `WIRE-BASED-100.5j` · Status: `done` (`2026-08-09`; serial-specific work delegated to and closed by
  `SWD-SERIAL-EXTRACTION`) · Goal: SWD/ADI to 100%. Ingested
  `corpus/.../IHI0074_A_*.pdf` (fresh key). **Honest finding (`2026-06-07`, no-faking):** SWD/ADI is an
  **architecture/serial spec, not a parallel-bus signal-table spec** — ~7032 statements (DAP, DP/AP
  registers, the SWD & JTAG serial protocols); the core **SWD wire contract `SWCLK`/`SWDIO` lives in
  prose/figures, never a signal table → NOT declared** (SWCLK 13× / SWDIO 28× in prose, 0 declarations);
  only ~9 real signals declared (JTAG `TCK`/`TDI`/`TDO`, `DBG*`, `NSRSTOUT`, `CSYSPWRUPACK`,
  `PORTCONNECTED`). The parallel-bus playbook (signal tables → constraints/relations/temporal) does NOT
  transfer. **Delivered:** a general garbage-cleanup precision fix — `is_hardware_signal_token` now
  requires a leading LETTER (drops number-literals `0B0`/`0B1`/`0X1F` a value cell mis-declares), and
  `is_signal_synthesis_non_signal` gained `IN`/`OUT`/`LEVEL` (direction/common words). Result: ADI garbage
  declarations dropped (`0B0`/`0B1`/`IN` gone, the `IN`-actor relations gone); APB/AHB/AXI all still 100%
  (no real signal leads with a digit); +1 hermetic test; full CI green. **Delegated closure:** the separate
  serial-protocol/architecture tree captured SWCLK/SWDIO, packet fields/operations, protocol states, and the
  complete interface-edge timing tuple; scored the independently verified 29-fact gold at 1.000; projected exact
  11/4/13/1 collections through IntentIR; accounted for all 29 records at the adapter/convergence boundaries;
  and promoted a fresh portable tracked-PDF chain. KM `[[swd-adi-not-signal-table-spec]]` and
  `[[swd-canonical-protocol-artifact-is-current]]`. APB/AHB/AXI remain 1.000 on constraints, relations, and
  temporal; SWD is 1.000 on its faithful serial frame/operation/state/edge metric without a cherry-picked gold.
  **CORRECTION `2026-09-01` (`.8c`): that closing sentence is no longer true and must not be cited as current.**
  SWD's serial metric is **5/29** — operations 4/4 and edge timing 1/1 survive, frame fields 0/11 and states 0/13
  do not — because `89d8dee7` (`2026-08-12`) retired the protocol-name-bound frame extractor on ADR 0006 grounds.
  The APB/AHB/AXI `1.000` is also unverifiable today: those three chains are legacy and `eval-extraction`
  refuses them. `[[swd-serial-frame-score-retired-by-genericity]]`.

## Picked sequence to APB 100% (owner: "pick the next trees to achieve just that")

`.1b` (relation recall → 100%) → `.6` (actor discrimination → relation precision) → `.7`
(normative gate → constraint precision) → re-measure → a defensible APB 100%. Then `.2`/`.3` (value
recall, full-doc completeness) harden it; then roll the same set to AHB → AXI → SWD (`.5`).

## Changelog

- `2026-08-31` (`.8` opened): **the scoring oracle itself is down.** `eval-extraction` refuses every document
  in the corpus — the 54 legacy chains as proofless/inspection-only, the 24 current ones as
  `registered derivation 'evidence.claim.schema_version.root' output or input topology is stale` — and it
  reproduces on the pre-change `2026-08-28` binary, so it is not a regression from the slice that found it
  (`KG-ISF-COMPLETENESS.5.iv.a`). Isolated read-only: rewriting only an EvidenceIR's `artifact_layout` fails
  canonical verification while the byte-identical copy that keeps its layout verifies, and `extract_on_copy`
  must relocate so the corpus is never mutated. Recorded as a gate-integrity defect because a published
  `1.000` that cannot be re-derived is precisely the claim `CLAIM_VERIFICATION.md` refuses.

- `2026-08-09` (`.5j` delegated closure): `SWD-SERIAL-EXTRACTION` completed the serial-specific work that the
  parallel-bus tree correctly deferred: fresh 29/29 scoring at 1.000, exact canonical 11/4/13/1 projection,
  complete adapter/convergence accounting, portable tracked-PDF promotion, and full WIRE/KG/FSMGen signoff.
  `.5j` is now done without pretending the sparse constraint/relation metric represents SWD's packet/FSM intent.
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
