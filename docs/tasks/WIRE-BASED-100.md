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

## APB — where the three scores actually stand (re-derived `2026-09-10` by `.9b`)

```
signal_constraint      P=1.000 R=1.000 F1=1.000  (tp=6 fp=0 fn=0)   # seed_apb.json          HOLDS
actor_signal_relation  P=1.000 R=1.000 F1=1.000  (tp=5 fp=0 fn=0)   # seed_apb.json          HOLDS
temporal_rule          P=0.333 R=0.333 F1=0.333  (tp=1 fp=2 fn=2)   # seed_apb_temporal.json WITHDRAWN
```

**Two of the three aspects re-derive at 1.000 on the current binary; the third does not, and the
`2026-06-06` "100% on ALL three aspects" headline is withdrawn** rather than carried. Constraints and
relations re-derive exactly, per fact, together with catalog-100% (35/35 signals, `.3a`) and the
completeness gauge (`.3a`+`.3b`). Temporal returned to its own pre-`.4` baseline because
`SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` (`f88d463d`, `2026-08-12`) deleted `.4`'s
`resolve_indexed_signal_family` when it made document identifiers opaque — so the gold's canonical `PSELX`
antecedent is no longer producible and `PNSE`/`PBUSER` each lose one. Recovery is owned by `.4a`; the gold
is untouched. Reproducible:
`specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider skip` and
`… seed_apb_temporal.json --provider skip` (the `-- source-tolerant + filtered (WIRE-BASED-100) --`
blocks). What still stands was earned by making facts correct — `.1` fixed id-drift, `.1b` credits valid
sources, `.6` dropped garbage actors (`FOR`, `APB protocol`), `.7` dropped the descriptive-clause
hallucination (`PRDATA must_be_stable`) — with derived universal-language checks (ADR 0006) and tested
guards. **The durable lesson is the four-week blind spot, not the number:** a legacy chain cannot be
scored, so a retired fix cost a published `1.000` and no gate said so until `.9b` re-ingested the document.
Next: `.4a` (temporal identity), then roll the bar to AHB → AXI → SWD (`.5`).

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

- ID: `WIRE-BASED-100` · Status: `active` · Children: `.1`–`.5`, `.6`/`.7` (automatic actor + normative
  discrimination), `.8` (scoring-oracle restoration, closed `2026-09-01`), `.9` (re-ingest the legacy wire
  golds so this tree's numbers can be re-derived at all — opened `2026-09-01` because `.8` closed with 5 of
  7 gold documents unscoreable and the work owned by nobody)
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
  **RETIRED `2026-08-12`, discovered `2026-09-10` by `.9b`:** `resolve_indexed_signal_family` was DELETED by
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` (`f88d463d`), which removed identifier-spelling authority from the
  pipeline; the temporal score returned to exactly the `0.333` (tp=1 fp=2 fn=2) baseline diagnosed above. The
  fix and its demonstration stand as history; the current number does not. Recovery is `.4a`.
- ID: `WIRE-BASED-100.4a` · Status: `pending` · Goal: **decide how an un-indexed prose signal reference binds to
  its declared indexed identity under the identifier-opacity doctrine, and restore APB temporal to a number the
  binary can produce.** Opened by `.9b`, which re-derived `seed_apb_temporal` at `P=R=F1=0.333` (tp=1 fp=2 fn=2)
  against the carried `1.000` and root-caused it: `.4` canonicalized prose `PSEL` to the declared `PSELX` and
  corrected the gold's antecedent to that identity; `f88d463d` deleted the resolver and installed the opposite
  behaviour as a tested invariant
  (`temporal_condition_does_not_alias_an_undeclared_name_from_suffix_spelling`: with `PSELX` declared,
  `"PSEL is asserted"` must yield NO predicate). `PNSE`/`PBUSER` each lose an antecedent as a result.
  **CORRECTED `2026-09-10` by `.9d`:** `.9b` also blamed the same commit's identifier-opacity change, which
  moved the declared select from `PSELX` to `PSELx`, and called the gold unreachable "by two independent
  routes". That second route does not exist. `eval::temporal_predicate_key`
  (`crates/specforge/src/eval.rs:458`) uppercases every name before comparison, so a spelling change of case
  alone cannot move a score — demonstrated independently by `.9d`, where the produced antecedent `ARESETn`
  matched the gold's `ARESETN` and AXI temporal scored `3/3`. There is exactly ONE cause: the deleted
  resolver.
  **The two candidate resolutions are genuinely different, and the leaf must choose on evidence, not
  convenience.** (a) Reinstate the binding on a SOURCE-GROUNDED footing — the document itself declares the
  select and its per-completer index, so the link would be read from the declaration rather than inferred from
  spelling, which is what the opacity doctrine actually forbids. (b) Rule that an un-indexed prose reference is
  a genuinely distinct identity and re-anchor the gold to what the document declares — a gold correction, which
  this tree permits ONLY as its own leaf with the fact left unchanged, never to recover a headline.
  Whichever wins must state why the other is wrong, and (a) must not resurrect the suffix-spelling inference
  the alignment tree deliberately removed.
  **Scope beyond APB — MEASURED `2026-09-10` by `.9c`, and narrower than this leaf first stated.** `.5` does
  reuse this resolver for AHB `HSELx` and the same test names `HSELX`/`HSEL`, so `.9c` was expected to fail the
  same way. It scored `4/4 = 1.000`. The reason is exact: the AHB temporal gold's only antecedent is `HREADY`,
  which the document declares with the spelling the prose uses, and no AHB gold item references the un-indexed
  `HSEL`. So the defect is confined to a gold whose antecedent names an INDEXED-FAMILY signal by its
  un-indexed prose spelling. That does not make it APB-only — every un-indexed select reference in any spec
  still loses its antecedent silently, and only a gold that scores one makes the loss visible — so `.4a` must
  fix the mechanism, not the one gold.
  Non-goal: reverting `f88d463d`, or weakening
  `temporal_condition_does_not_alias_an_undeclared_name_from_suffix_spelling` to make a score move.
  Prerequisite: `WIRE-BASED-100.9b` (the re-derivation that exposed it).
  Verification: pending
  Commit: pending
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
  `WIRE-BASED-100.8c`, `WIRE-BASED-100.8d`, `WIRE-BASED-100.8e`, `WIRE-BASED-100.8f`.
  **CLOSED `2026-09-01`.** `.8a`/`.8b`/`.8c` are `done` and `.8d` is `deferred` with its consequence recorded,
  so every child is resolved. The oracle runs, refuses nothing silently, and its first re-derivation since
  `2026-08-09` retired a published score rather than confirming it — which is the outcome this leaf existed to
  make possible. What `.8` does NOT deliver, stated plainly: the wire golds it was meant to protect (APB, AHB,
  AXI) remain unmeasurable because their EvidenceIR is legacy, so "the oracle is restored" means restored over
  the 24 rebuildable chains only. ~~Re-ingesting the legacy stratum stays with the corpus refresh frontier.~~
  **CORRECTED `2026-09-01` by `.9a`: that hand-off names an owner that excludes the work.** The frontier's
  cohort rule is `excluded_source_prefixes: ["corpus/"]`, so all 18 legacy in-repo gold/eval documents — the
  three wire golds among them — are outside its cohort by construction; it would never have reported them
  outstanding. The wire re-ingest is now owned by `WIRE-BASED-100.9`.
  Non-goal: re-ingesting the 54 legacy chains (**as written, "owned by the corpus refresh frontier" — false
  for 18 of them; `.9` owns the three wire golds and `.9a` routes the rest**); changing any gold.
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
  gate is satisfied by 56 statements none of which carry the gold's bit-range fields (`.8e` corrected this
  from 61: the probe that produced it had dropped the gate's own `parse_count_word` rejection).
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
  and parses a bit range / named-bit list. SWD carries 56 statements with a stated phase name and writes
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
  - **Nearest preceding phase-stating statement.** It resolves for every field, so it would *fire* — and it is
    wrong on **11 of 11** (`.8e`; the figure first published here said "at least 7", which was asserted rather
    than computed). `APnDP`/`RnW` (gold `request`) inherit `transfer`; `Start`/`Parity`/`Stop` (gold `request`)
    inherit `transfer` from 28–44 statements back; `Park` (gold `request`) inherits `data`; `A`, `ACK` and
    `DATAIN` sit **278, 303 and 324** statements after the nearest one, which names `response`. A rule that
    fires everywhere and is right **nowhere** is fabrication with a structural alibi.
  - **The owning section's title.** Resolving each gold statement's span to its section anchor: `Packet
    requests` → `request` ✓ (`APnDP`, `RnW`) and `Data transfers (WDATA and RDATA)` → `data` ✓ (`WDATA`,
    `RDATA`), but `Start`/`Parity`/`Stop` sit under `B4.2 SWD protocol operation`, `Park` under `B4.2.5 Protocol
    error response`, `A` under `Attributes`, `ACK` under a table caption and `DATAIN` under `OK or FAULT
    response to a DPACC or APACC access`. **4 of 11** land under a literal title-states-the-phase reading and
    **5 of 11** even counting any appearance of the phase word or its stem (`ACK responses`), so the honest
    ceiling is 5, not the 4 first published here. No title contains the word `phase` — 0 of 11.
  **WHERE THE BINDING ACTUALLY LIVES — and it explains the retired extractor.** For `Start`/`Parity`/`Stop`/
  `Park`/`A` no statement ever ASSIGNS the field to a phase (`.8f` sharpened this from "never states the phase
  in text at all", which was imprecise: the phase WORDS do occur nearby — `statement_1678` names both phases in
  one sentence — but never as an assignment); the frame's field-to-phase membership is drawn
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


- ID: `WIRE-BASED-100.8e` · Status: `done` (`2026-09-01`) · Goal: **audit `.8`'s published findings on the
  director's challenge, and repair the three figures that do not survive re-derivation.** Every finding's
  CONCLUSION holds; three of `.8d`'s NUMBERS do not, and the reason is the same one this lane spent four slices
  correcting in other people's work: a figure was read off a probe instead of derived, and the probe was not a
  faithful port of the thing it claimed to measure.
  **What survives, re-derived rather than restated.** `.8a`'s relocation seam and the restored oracle (the
  `seed_swd_derivation` scorecard reproduces exactly at HEAD: 0/11, 4/4, 0/13, 1/1); `.8c`'s three legs — chain
  currency 24/24, the corpus-wide surface-selective loss, and `89d8dee7`'s own published counts; the retired
  extractor's identity gate and its now-absent `SerialFramePhase` enum; `.8`'s blast-radius rule (that commit
  DID supersede exactly four SWD cards and update four book chapters, and DID NOT touch `ROADMAP.md`,
  `docs/book/src/quality/extraction-eval.md`, either owning tree, or `swd-derivation-scored-100`); the schema
  census; the corpus-frontier population; and the figure claim (`picture_0038`/`picture_0039`, role
  `ambiguous`, a single `caption` observation each).
  **A population trap that did NOT bite, checked because MEMORY says to.** `.8c` cited `89d8dee7`'s "five
  operations and 40 structurally admitted states" as re-deriving today, measured over the 24 schema-3
  documents, while the ledger sentence was about all 78. Re-measuring both frames: the 54 legacy artifacts
  carry **0** frame fields, **0** states and **0** operations, so the two frames give identical totals and the
  citation is sound. Sound by luck of the migration, not by construction — it was worth checking.
  **The three figures that failed.** (a) "61 statements carry a stated phase name" is **56**: the probe dropped
  the gate's own `parse_count_word` rejection, so "two or three phases" was counted as the phase name `three`,
  and it stripped non-alphabetic characters anywhere in a token instead of trimming only the ends as Rust's
  `trim_matches` does. (b) "wrong for at least 7 of 11" was never computed — the true figure is **wrong on 11
  of 11, correct on 0**. It was an eyeballed floor published in the voice of a measurement; the evidence was
  always stronger than the claim. (c) "at most 4 of 11" section titles is **not a valid ceiling**: 4 land under
  a literal reading but **5** do once `ACK responses` is counted, so the bound is 5.
  **The fix is a derivation, not a reword** (`KG-ISF-COMPLETENESS.5.iv.a`'s lesson, applied to my own numbers):
  `scripts/measure_swd_frame_phase_scope.py` ports `stated_phase_name` and `parse_count_word` exactly, prints
  all three figures plus the per-field table, and **checks its ported count-word list against the Rust source**
  so the two cannot drift silently. Demonstrated RED: adding a `"thirteen" => Some(13)` arm to a copy of
  `evidence.rs` makes the script exit with `parse_count_word drifted`.
  **And the rule generalises to its author.** `.8c` published that a retired producer stales the surfaces
  carrying its NUMBER. These three figures had spread to exactly six — `CHANGES.md`, `MEMORY.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, this tree, the fact card and the book — in under a day.
  **`.8`'s conclusion is unchanged and slightly strengthened:** scope binding does not work, the binding is in
  the figure, and `serial_frame_field` stays 0/11 deliberately.
  Non-goal: re-opening `.8d`, whose deferral the corrected figures support more strongly than the originals.
  Prerequisite: `WIRE-BASED-100.8d`.
  Verification: `python3 scripts/measure_swd_frame_phase_scope.py`
  Commit: see log.

- ID: `WIRE-BASED-100.8f` · Status: `done` (`2026-09-01`) · Goal: **re-challenge `.8e`'s own audit, and record
  the trap the next attempt at `.8d` will fall into.** `.8e` verified `.8d`'s figures against the production
  gate; it did not test whether `.8d`'s CONCLUSION survives a *better* detector. It does, and the test that
  proves it is worth keeping because its raw number is a lie.
  **The adversarial test.** Replace the production phase detector with a deliberately permissive proximity rule
  — a statement "states phase P" if `P` occurs within four tokens of `phase`/`phases` — and ask how many gold
  fields a nearest-preceding rule would then get right. The answer looks like **5 of 11**, which would read as
  "the detector was the problem, `.8d` gave up too early". **Every one of those five is a false positive.** The
  matched statements are `statement_1678` (*"A simple parity check is applied to all packet request and data
  transfer phases"* — claimed for seven fields; it names both phases and assigns neither), `statement_1798`
  (*"A FAULT response to a read or write packet request consists of two phases"* — claimed for `Park`, and it
  is about a FAULT response), and `statement_0813` 702 statements away (*"a parity or framing error on the data
  phase of a write"* — claimed for `DATAIN`). Not one assigns a field to a phase.
  **So `.8d` is confirmed by an attempt to break it**, which is a stronger result than `.8e`'s: the conclusion
  no longer rests on the production detector being right, because a strictly more permissive detector recovers
  nothing real either.
  **And a precision defect in `.8d`'s own wording, corrected.** It said the document "never states the phase in
  text at all" for `Start`/`Parity`/`Stop`/`Park`/`A`. The phase WORDS do occur near them; what never occurs is
  an ASSIGNMENT of a field to a phase. The distinction is the whole point — a proximity rule sees the words and
  mints the wrong phase — so the loose wording would have taught the next reader the wrong lesson.
  **Also checked and sound, so the audit's other legs stand:** every gold statement falls strictly inside the
  section anchor attributed to it (11/11, `line_start <= line <= line_end`); every frame gold item carries
  exactly one fact, and the two statements carrying two items each (`statement_1679`, `statement_1682`) are
  handled per item rather than per statement.
  **Residual honestly stated:** `56` remains a PORT-derived figure — `.8e`'s script ports `stated_phase_name`
  rather than calling it, and only the count-word list is machine-checked against the source. It is not
  load-bearing: the production-derived fact is the extraction manifest's `serial_frame.bit_range` and
  `serial_frame.composition` both at `produced: 0`, which owes nothing to the port. `11 of 11` is robust to the
  port's accuracy — the loose probe and the faithful port give the same answer.
  Non-goal: re-opening `.8d`. This slice strengthens its deferral.
  Prerequisite: `WIRE-BASED-100.8e`.
  Verification: `python3 scripts/measure_swd_frame_phase_scope.py` (its `adversarial control` section)
  Commit: see log.

- ID: `WIRE-BASED-100.9` · Status: `done` (`2026-09-10`; `.9a` `2026-09-01`, `.9b`/`.9c`/`.9d` `2026-09-10`) · Goal: **make the
  APB/AHB/AXI wire golds measurable again by re-ingesting them, and own that work here.** `.8` restored the
  oracle over the 24 rebuildable chains and closed by handing its remainder away in one sentence —
  *"Re-ingesting the legacy stratum stays with the corpus refresh frontier"*, repeated in its `Non-goal` as
  *"the 54 legacy chains (owned by the corpus refresh frontier)"*. **That routing is disproven by the
  frontier's own contract** (`.9a`): its cohort rule is `excluded_source_prefixes: ["corpus/"]`, so every
  in-repo gold/eval document — the three wire golds among them — is outside the cohort by construction. The
  frontier could not have picked this up, would never report it outstanding, and its `5 remaining` would still
  read `5` after every wire gold had rotted. The work has no OPEN owner; this node is the owner.
  **The real cause is an invalidated refresh, not neglect — found by auditing CLOSED leaves.**
  `CORPUS-PATTERN-REUSE.3c` (`done`, `2026-06-09`) re-ingested APB/AHB/AXI/AXI-Stream with
  `DOCLING_DEVICE=cpu`, rebuilt their evidence, and re-verified the wire scores at `1.000`. The persisted wire
  SourceIRs were last written `2026-08-09`; canonical schema 3 landed `2026-08-12` (`bb5047c2`) three days
  later and made that completed refresh legacy, **and no gate reported it** — the frontier excludes these
  documents by cohort rule, chain-currency counts only the already-current stratum. `.9a` first published this
  as "no owning leaf at all"; that was corrected before commit, because searching only the open frontier missed
  the leaf that owned the work and closed.
  **Why it is the highest-leverage unblock in the tree.** `WIRE-BASED-100`'s governing principle is no fake
  scoring, and `.9a` measured what the tree can currently score: **2 of 7 gold-carrying documents.** APB, AHB
  and AXI hold six of the eleven tracked eval datasets and none of them can be scored at all, so every APB/AHB/
  AXI number this tree has published is currently un-re-derivable — exactly the claim `CLAIM_VERIFICATION.md`
  refuses. Re-ingesting three documents restores six datasets.
  Children: `.9a` (measurement + ownership), `.9b` (APB), `.9c` (AHB), `.9d` (AXI) — all done `2026-09-10`
  except `.9a` (`2026-09-01`); smallest document first, one per leaf, each carrying its own before/after
  evidence.
  **CLOSED `2026-09-10`. Acceptance met in full:** all six datasets — `seed_apb`, `seed_apb_temporal`,
  `seed_ahb`, `seed_ahb_temporal`, `seed_axi`, `seed_axi_temporal` — run to a real score under
  `--provider skip`; each leaf published its per-fact re-derivation against the carried numbers; the census
  reports **27 measurable (34.6%) / 51 legacy** with **5 of 7** gold documents measurable and the two that
  are not (RISC-V Debug, NVMe) are exactly the ones `.9a` routed OUT by name. **Twelve carried numbers were
  re-derived: eleven held and one was withdrawn** (APB temporal `1.000 → 0.333`). Three findings were opened
  rather than absorbed: `.4a` (the deleted antecedent resolver), `RETAINED-BUNDLE-POPULATION-FROZEN` (the
  retained set can neither grow nor shrink) and `.10` (AXI's 115 lost typed declarations). **The durable
  lesson: a legacy chain cannot be scored, so for four weeks nothing could tell a held number from a rotted
  one — and re-derivation moved a number in one document out of three, in the direction nobody predicted.** **`.9b`'s outcome changes what
  `.9c`/`.9d` must expect:** the route itself needs no change, but a re-ingested wire gold can return a
  DIFFERENT number from the one the tree carries, and `.9b` found one — so each remaining leaf publishes its
  re-derivation as a verdict per aspect, and any temporal antecedent that names an indexed select
  (AHB `HSELx`, AXI) is expected to fail the same way until `.4a` closes.
  **Findings routed OUT, deliberately not absorbed** (`.9a` measured them; this tree does not own them):
  (1) `1_0_risc_v_debug_specification` is the fourth unmeasurable gold document with no open owner and belongs
  to the `PDF-VARIANT-DIGESTION` register class, not to a wire-protocol tree. (2) **31 of the refresh
  frontier's 52 declared-`refreshed` documents are still legacy** — `refreshed` records completion of the
  host-library re-ingest PROGRAM, not canonical currency, because the sweep finished under a SourceIR schema
  that no longer carries authority. Whether `refreshed` should keep meaning that is a corpus-program question.
  (3) **Nothing fails when a persisted chain falls below the canonical schema.** That missing gate is why a
  completed refresh could go legacy unreported for three weeks; a check would have fired on `2026-08-12`
  instead of leaving this to be found by a leaf that happened to need the scorer. It is a
  `DOCTRINE-ENFORCEMENT`-class decision (this census is the derivation it would use), left explicitly unowned
  rather than half-adopted inside a wire-protocol tree.
  Non-goal: re-ingesting the other 51 legacy chains; changing any gold; re-opening `.8`.
  Acceptance (closes `.9`): `eval-extraction --provider skip` runs to a real score on `seed_apb`,
  `seed_apb_temporal`, `seed_ahb`, `seed_ahb_temporal`, `seed_axi`, and `seed_axi_temporal`; each leaf
  publishes its re-derived per-fact numbers against the pre-re-ingest published ones, with any number that
  moved stated as a correction rather than carried; and `scripts/measure_corpus_canonical_currency.py` reports
  the three documents measurable with no gold document left unowned inside this tree's scope.
  Prerequisite: `WIRE-BASED-100.8` (met — the oracle must work before a re-ingest can be judged by it).
  KM `[[corpus-canonical-currency-and-ownership]]`.

- ID: `WIRE-BASED-100.9a` · Status: `done` (`2026-09-01`; MEASUREMENT + ownership, read-only, docs-only —
  no code change) · Goal: **derive how much of the persisted corpus can actually be scored, and who owns each
  shortfall**, before spending a re-ingest on a guess. **Measured** with the tracked deterministic reproducer
  `scripts/measure_corpus_canonical_currency.py` (reads persisted artifact headers, the four Rust schema
  constants that define canonical currency, the frontier's own contract, and the tracked eval datasets; no
  model, no rebuild, no write):

  | population | count |
  | --- | ---: |
  | persisted documents | 78 |
  | **measurable** (EvidenceIR at canonical schema 3) | **24 (30.8%)** |
  | legacy (refused, inspection-only) | 54 |
  | …with no OPEN owner (`corpus/`-sourced, outside the frontier cohort) | 18 |
  | …frontier-declared `refreshed`, yet still legacy | 31 |
  | …frontier-declared `remaining` | 5 |
  | documents carrying an eval gold | 7 |
  | **…measurable** | **2** (SWD/ADI, I2C) |

  **Three results, each of which changes what a reader should believe.** (1) **The measurable share is 30.8%,
  and no gate publishes it.** `check_chain_currency.sh` reads `24/24 current` — true, and a statement about the
  *rebuildable* stratum, which declares the other 54 UNMEASURABLE and does not count them, so it reads 100%
  while describing 31% of the corpus. `check_corpus_frontier.sh` reads `52 refreshed + 5 remaining` — true, and
  a statement about the host-library re-ingest PROGRAM. (2) **`refreshed` does not mean current** — 31 of the
  52 are still legacy. The census contract is internally honest: it defines `refreshed` as declared completed
  keys, and its membership check constrains that declaration in one direction only (a RETAINED bundle forces a
  key into `refreshed`; a declared-`remaining` key must have no retained bundle) — nothing ties it to a schema.
  But the line printed on the terminal invites exactly the inference `.8` made. (3) **The frontier excludes
  `corpus/` by contract**, so the 18 in-repo gold/eval legacy documents have no OPEN owner — including APB,
  AHB, AXI and RISC-V Debug.
  **The predicate is taken from the code, not invented:** measurable ⇔ persisted EvidenceIR at the current
  canonical schema, exactly what `unmeasurable_disposition`
  (`crates/specforge/src/commands/eval_extraction.rs`) applies before it will score. The reproducer reads each
  stage constant rather than restating it, so a schema bump makes the census disagree loudly instead of
  agreeing with itself. **Stated as a bound, not a demonstration:** 24 is an ADMISSION count. Only 2 of the 24
  carry an eval gold, so only two are scoreable today; and a schema-3 document that fails canonical
  verification for any OTHER reason is not dispositioned but propagates and aborts the run
  (`build_predictions`) — deliberately, so a real defect cannot hide inside a disposition.
  **Decision: GO on `.9b`/`.9c`/`.9d`, NO-GO on absorbing the three shortfalls** (routed out in `.9`).
  The route is demonstrated, not assumed: the three PDFs are git-tracked under `corpus/` by `.5d`
  (APB 516 KB, AHB 957 KB, AXI 2.0 MB, all in `corpus/SOURCE_PDF_REGISTRY.md`), `specforge doctor` reports the
  repo-local Docling runtime ready (Python 3.11.15 / Docling 2.84.0), and the three `corpus/`-sourced documents
  that ARE measurable — SWD/ADI, I2C, I2S — were produced through this same route after the `2026-08-12` schema
  bump (`bb5047c2`); and `CORPUS-PATTERN-REUSE.3c` already ran this exact sequence on these exact documents
  (`2026-06-09`), so `.9b`–`.9d` repeat a demonstrated procedure under a newer binary rather than attempting a
  new one. The golds are content-anchored (`.1`), so they survive the re-segmentation a re-ingest
  causes; scoring needs no model server (`--provider skip`).
  Report `docs/research/corpus-canonical-currency-census.md`; KM `[[corpus-canonical-currency-and-ownership]]`.
  Verification: see the acceptance checklist below.
  Commit: `WIRE-BASED-100.9a — the corpus refresh frontier cannot own the wire re-ingest, and 5 of 7 gold documents cannot be scored`

- ID: `WIRE-BASED-100.9b` · Status: `done` (`2026-09-10`; APB re-ingested, six datasets re-derived, **one carried
  number withdrawn**) · Goal: **re-ingest the APB gold `ihi0024_e` and re-derive its score.** Smallest of the
  three (516 KB), so it proves the whole route end-to-end at the lowest cost and its outcome decides whether
  `.9c`/`.9d` run unchanged.
  Acceptance: the persisted APB chain is preserved on the repository volume BEFORE the rebuild (the current
  binary cannot regenerate a legacy artifact, so the outgoing evidence is unrecoverable once replaced — see the
  caution in `.9`); `ingest` → `evidence` → `semantic` produces SourceIR 3 / EvidenceIR 3 / SemanticIR 2;
  `eval-extraction --provider skip` runs to a real score on `seed_apb` and `seed_apb_temporal` with the
  per-fact table this tree's governing principle requires; the re-derived numbers are published **against**
  the ones the tree currently carries (`constraint 6/6`, `relation 6/6`, `temporal 3/3`), and any number that
  moved is stated as a correction rather than quietly replaced; `scripts/measure_corpus_canonical_currency.py`
  shows 25 measurable; `check_chain_currency.sh` and `scripts/check_doctrines.sh` green.
  Non-goal: touching the gold labels. A gold edit to make a score look better is the failure mode this tree's
  governing principle exists to forbid; if a gold is genuinely stale the correction gets its own leaf.
  Prerequisite: `WIRE-BASED-100.9a`.
  **RESULT — two of the three aspects re-derive exactly; the third does not, and is withdrawn.** The route ran
  unchanged (`DOCLING_DEVICE=cpu ingest` → `evidence` → `semantic` → `intent` → `adapt --target isf`, no model
  server, ~4 min): 48 page artifacts / 35 visual / 0 residuals / `automation_confidence high`, and the chain is
  canonical **SourceIR 3 / EvidenceIR 3 / SemanticIR 2 / IntentIR 2**. The census
  moves `24 → 25` measurable (30.8% → 32.1%), legacy `54 → 53`, gold-carrying documents measurable `2 → 3`.

  | dataset · aspect | carried | re-derived `2026-09-10` | verdict |
  | --- | --- | --- | --- |
  | `seed_apb` signal_constraint | `P=R=F1=1.000` (tp=6 fp=0 fn=0) | `P=R=F1=1.000` (tp=6 fp=0 fn=0) | **holds** |
  | `seed_apb` actor_signal_relation (source-tolerant + filtered) | `P=R=F1=1.000` (tp=5 fp=0 fn=0) | `P=R=F1=1.000` (tp=5 fp=0 fn=0) | **holds** |
  | `seed_apb` document-level recall | constraints 6/6, relations 6/6 | constraints 6/6, relations 6/6 | **holds** |
  | `seed_apb_temporal` temporal_rule | `P=R=F1=1.000` (tp=3 fp=0 fn=0) | **`P=R=F1=0.333` (tp=1 fp=2 fn=2)** | **WITHDRAWN** |

  Per-fact, the twelve `seed_apb` gold facts are all true positives — constraints `PADDR`/`PWDATA`
  `must_be_stable`, `PSTRB` `must_be_low`, `PWAKEUP` `must_be_asserted`, `PBUSER`/`PNSE` `must_be_value VALID`;
  relations Completer→`PREADY` (×2 gold statements), Requester→`PSTRB`, Completer→`PRDATA`,
  Requester→`PWDATA`, Completer→`PSLVERR`. Of the three temporal gold facts only `PSTRB`
  (`Requester drives PSTRB` + `PSTRB LOW`, no antecedent) still matches; `PNSE` and `PBUSER` are both missed
  **on their antecedent**, and the scorer prints both missed keys.
  **ROOT CAUSE — `.4`'s canonicalization was retired by the identity-opacity doctrine, and no gate connected
  the two.** `.4` (`1c28516b`, `2026-06-06`) added `resolve_indexed_signal_family` so an un-indexed prose
  reference (`PSEL`) resolved to the declared indexed family member (`PSELX`), and corrected the temporal gold's
  antecedent to that canonical identity. `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` (`f88d463d`, `2026-08-12`,
  *"make document identifiers opaque"*) **deleted that function** and installed the opposite behaviour as a
  test: `temporal_condition_does_not_alias_an_undeclared_name_from_suffix_spelling` asserts that with `PSELX`
  declared, `"PSEL is asserted"` yields **no** predicate. So `PNSE`'s antecedent is dropped entirely and
  `PBUSER`'s antecedent set loses its `PSELX` member (`PENABLE`+`PREADY` survive, both declared verbatim).
  The same commit also removed identifier-spelling authority upstream, so the declared identity is now the
  document's own `PSELx` — visible as `sigcon_0011`'s antecedent moving `PSELX → PSELx` across the re-ingest.
  **CORRECTION (`2026-09-10`, `.9d`): that spelling move is real but SCORING-NEUTRAL, and this leaf was wrong
  to call it a second independent route.** `eval::temporal_predicate_key` (`crates/specforge/src/eval.rs:458`)
  uppercases every name before comparison, so case alone cannot move a score; `.9d` demonstrated it from the
  other side, where the produced `ARESETn` matched the gold's `ARESETN` and AXI temporal scored `3/3`. The
  single cause of APB's loss is the deleted resolver, and the fix `.4a` owes is the antecedent, not the
  spelling.
  **The re-ingest did not cause this, and the preserved bytes prove it.** In the PRESERVED legacy SemanticIR
  (`generated/preserved/WIRE-BASED-100.9b/pre-reingest/`, digests below) `temporal_signal_constraint_sigcon_0009`
  (`PNSE`) already carries `antecedents: []` and `…_sigcon_0014` (`PBUSER`) already carries only
  `PENABLE`+`PREADY`. Both defects were already persisted in the artifact the `1.000` was last associated with;
  the schema bump three days earlier had made that artifact unscoreable, so **the regression was invisible for
  four weeks and the re-ingest is what made it visible**. This is the same blindness `.9a` routed out as an
  unowned defect — nothing fails when a persisted chain falls below the canonical schema — now demonstrated
  costing a real published number.
  **Not fixed here, and the gold was not touched.** Restoring the number needs a decision between reinstating a
  source-grounded (not spelling-inferred) binding of `PSEL` to declared `PSELx` and re-anchoring the gold to the
  opaque identity; that is `WIRE-BASED-100.4a`, opened by this leaf. Editing the gold to recover the headline is
  exactly what this tree's governing principle forbids.
  **Second observed movement, and it is NOT a regression:** the ISF adapter went `renderable` → `blocked`
  (`no source-grounded system clock/reset contract`; `(clock __specforge_unresolved_clock)`), because `PCLK` /
  `PRESETn` were recognised as clock/reset by spelling and that authority is gone. This is the documented
  current policy for the whole measurable stratum — the book already states every retained adapter manifest is
  honestly blocked with zero emitted `.isf` — so APB joins that set as the 25th rather than breaking a rule.
  **The re-ingest's normalized bundle is HELD OUT of the retained declaration, and that is a finding, not
  bookkeeping.** The rebuild restored the bundle, and `check_chain_currency.sh` fails closed on an undeclared
  bundle on disk — but declaring it turns `PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` red, because the
  retained set is frozen at 24 by a size literal in two contract validators and joined by SET EQUALITY to a
  release-blocking behavioral qualification whose 24-row population, 7/17 split and 51 held-out attempts are
  frozen at a declared selection boundary. Recording a deliberate reclamation instead is refused by the same
  two validators (`reclamations != []`). ADR 0025 mandates BOTH operations, so the first refresh to exercise it
  had no compliant move. `.9b` therefore MOVED the bundle to
  `generated/preserved/WIRE-BASED-100.9b/apb-normalized-bundle-held-out/` (repository volume, 25 MB,
  byte-identical, nothing deleted) and left the declaration at 24. Stated rather than hidden: APB's
  SourceIR→EvidenceIR replay reads UNMEASURABLE until the bundle returns, while its SemanticIR, IntentIR and
  adapter replays are measurable and current and its EvidenceIR is canonical and scoreable — which is what this
  leaf existed to restore. The whole finding is owned by `RETAINED-BUNDLE-POPULATION-FROZEN`, whose `.3`
  restores the bundle once both gates can accept a 25th key.
  Preservation (repo-volume, `2026-09-10`, before the rebuild; the current binary cannot regenerate legacy
  bytes): `generated/preserved/WIRE-BASED-100.9b/pre-reingest/` — `source_ir.json`
  `0eb30dfe48d44f3c…`, `evidence_ir.json` `0fd404bbace952b5…`, `semantic_ir.json` `217b261236bde70a…`,
  `intent_ir.json` `c1fbd376fa46faba…`, `adapters_isf/adapter.json` `83b0083095e6b42b…`; full manifest in that
  directory's `SHA256SUMS.txt`.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.9c` · Status: `done` (`2026-09-10`; AHB re-ingested, **all three aspects re-derive at
  1.000**) · Goal: **re-ingest the AHB gold `ihi0033_c` and re-derive its score** (957 KB). Same acceptance
  shape as `.9b`, against the currently carried `constraint 6/6`, `relation 6/6`, `temporal 4/4`; `.5e` already
  demonstrated this exact document's re-ingest → eval route once, which is why it follows APB rather than
  leading.
  Prerequisite: `WIRE-BASED-100.9b` (its outcome decides whether the route runs unchanged).
  **RESULT — every carried AHB number survives, and `.9b`'s prediction that it would not is CORRECTED.** The
  route ran unchanged (`DOCLING_DEVICE=cpu ingest` → `evidence` → `semantic` → `intent` →
  `adapt --target isf`, no model server): 104 page artifacts / 70 visual / 0 residuals /
  `automation_confidence high`, chain canonical **3 / 3 / 2 / 2**. The census moves `25 → 26` measurable
  (32.1% → 33.3%), legacy `53 → 52`, gold documents measurable `3 → 4` of 7.

  | dataset · aspect | carried | re-derived `2026-09-10` | verdict |
  | --- | --- | --- | --- |
  | `seed_ahb` signal_constraint | `6/6` | `P=R=F1=1.000` (tp=6 fp=0 fn=0) | **holds** |
  | `seed_ahb` actor_signal_relation (source-tolerant + filtered) | `6/6` | `P=R=F1=1.000` (tp=6 fp=0 fn=0) | **holds** |
  | `seed_ahb` document-level recall | constraints 6/6, relations 6/6 | constraints 6/6, relations 6/6 | **holds** |
  | `seed_ahb_temporal` temporal_rule | `4/4` | `P=R=F1=1.000` (tp=4 fp=0 fn=0) | **holds** |

  Per fact: constraints `HAUSER`/`HWUSER`/`HRUSER`/`HBUSER` `must_be_value VALID` and `HAUSER`/`HWUSER`
  `must_not_change`; relations Subordinate→`HRESP`/`HREADYOUT`/`HRUSER`/`HBUSER` and
  Manager→`HAUSER`/`HWUSER`; temporal `HAUSER`, `HWUSER` (no antecedent) and `HRUSER`, `HBUSER`
  (antecedent `HREADY HIGH`).
  **CORRECTION to `.9b` and to `.9`'s expectation.** `.9b` predicted `.9c` would fail the same way because
  `.5` reuses `.4`'s index-family resolver for AHB `HSELx`. It does not, and the reason is exact rather than
  lucky: **the AHB temporal gold's only antecedent is `HREADY`, a signal the document declares with the same
  spelling the prose uses.** No gold item references the un-indexed `HSEL`. So `.4a`'s defect is not
  "wire-wide" — it is confined to a gold whose antecedent names an indexed-family signal by its un-indexed
  prose spelling, which the APB gold does and the AHB gold does not. `.4a`'s scope note is corrected
  accordingly; the resolver is still gone for AHB, it simply has nothing scored to lose there.
  **The re-ingest was structurally inert for AHB**, which is why the golds needed no re-anchoring at all
  (`content-anchored: re-resolved 0/17` and `0/4` — the ids were already current): the rebuilt EvidenceIR has
  the SAME 1,322 statements / 172 anchors / 1,199 spans / 481 links / 70 visual records as the preserved
  legacy artifact. Only proof authority and identifier spelling moved.
  **Same two movements as `.9b`, same dispositions.** The ISF adapter went `renderable` → `blocked`
  (`no source-grounded system clock/reset contract`; `HCLK` demoted from `(clock HCLK)` into the interface),
  the documented policy for the whole measurable stratum; and the declared select identity moved
  `HSELX`/`HSELXCHK` → `HSELx`/`HSELxCHK` under identifier opacity, with no signal lost. The normalized bundle
  is HELD OUT at `generated/preserved/WIRE-BASED-100.9c/ahb-normalized-bundle-held-out/` for the reason `.9b`
  established and `RETAINED-BUNDLE-POPULATION-FROZEN` owns, so AHB's EvidenceIR replay reads UNMEASURABLE
  while its SemanticIR/IntentIR/adapter replays are current.
  Preservation (repo-volume, `2026-09-10`, before the rebuild): `generated/preserved/WIRE-BASED-100.9c/
  pre-reingest/` — `source_ir.json` `97444457673da6fd…`, `evidence_ir.json` `3bae4b77e7b5afe6…`,
  `semantic_ir.json` `8b77ad97561ea6cc…`, `intent_ir.json` `c17e46464fed8656…`,
  `adapters_isf/adapter.json` `073ec73d3e486f97…`; full manifest in that directory's `SHA256SUMS.txt`.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.9d` · Status: `done` (`2026-09-10`; AXI re-ingested, **all six numbers hold**, and two
  disproven predictions corrected) · Goal: **re-ingest the AXI gold `ihi0022_l` and re-derive its
  score** (2.0 MB — the largest, and the one whose persisted chain is 14.5 MB across four stages). Same
  acceptance shape as `.9b`, against the currently carried `constraint 4/4`, `relation 6/6`, `temporal 3/3`.
  Carries one extra check the smaller two do not: AXI is the document `KG-ISF-COMPLETENESS.5.iv.a` predicted
  would mint a new `AWATOP` enum and could not verify, because `ihi0022_l` was unrebuildable — so this leaf
  finally answers that open prediction, either way.
  **What `.9b`/`.9c` say to expect, and what they say NOT to assume.** Expect: the chain to reach canonical
  3/3/2/2; the ISF adapter to move `renderable` → `blocked` on `no source-grounded system clock/reset
  contract`; declared identifiers to lose their uppercasing; and the normalized bundle to be HELD OUT
  (`RETAINED-BUNDLE-POPULATION-FROZEN`). Do NOT assume a verdict either way on the scores: `.9b` withdrew a
  `1.000` and `.9c` kept three, and the difference was decided by whether a gold antecedent names an
  indexed-family signal by its un-indexed prose spelling — check the AXI temporal gold's antecedents against
  the declared catalog BEFORE predicting. AXI is also the largest (2.0 MB, a 14.5 MB persisted chain), so
  preserve first and watch RAM.
  Prerequisite: `WIRE-BASED-100.9c`.
  **RESULT — every carried AXI number re-derives, and BOTH of this leaf's stated predictions were wrong.**
  Route unchanged (`DOCLING_DEVICE=cpu ingest` → `evidence` → `semantic` → `intent` → `adapt --target isf`):
  320 page artifacts / 333 visual / 0 residuals / `high`, chain canonical **3 / 3 / 2 / 2**. Census moves
  `26 → 27` measurable (33.3% → 34.6%), legacy `52 → 51`, gold documents measurable `4 → 5` of 7 — the
  remaining two (RISC-V Debug, NVMe) are the ones `.9a` routed OUT of this tree by name.

  | dataset · aspect | carried | re-derived `2026-09-10` | verdict |
  | --- | --- | --- | --- |
  | `seed_axi` signal_constraint | `4/4` | `P=R=F1=1.000` (tp=4 fp=0 fn=0) | **holds** |
  | `seed_axi` actor_signal_relation (source-tolerant + filtered) | `6/6` | `P=R=F1=1.000` (tp=6 fp=0 fn=0) | **holds** |
  | `seed_axi` document-level recall | constraints 4/4, relations 6/6 | constraints 4/4, relations 6/6 | **holds** |
  | `seed_axi_temporal` temporal_rule | `3/3` | `P=R=F1=1.000` (tp=3 fp=0 fn=0) | **holds** |

  **PREDICTION 1, DISPROVEN — and it corrects a `.9b` claim rather than only this leaf's.** Before the
  re-ingest this leaf predicted AXI temporal would score `1/2`, because the gold's antecedent is spelled
  `ARESETN` while identifier opacity makes the declared identity `ARESETn`. It scored `3/3`: the produced
  antecedent IS `ARESETn` (`temporal_signal_constraint_sigcon_0046`/`0047`) and it MATCHED, because
  `eval::temporal_predicate_key` (`crates/specforge/src/eval.rs:458`) uppercases every name before comparing.
  **Case can never move a score**, so `.9b`'s "unreachable by two independent routes" is withdrawn: the sole
  cause of APB's temporal loss is `.4`'s deleted resolver, and `.4a` owes the antecedent, not the spelling.
  **PREDICTION 2, DISPROVEN.** `KG-ISF-COMPLETENESS.5.iv.a` predicted this document "would mint a NEW `AWATOP`
  enum" and could not verify it because `ihi0022_l` was unrebuildable. It mints nothing new: `(type AWATOP
  (bits 4))` is present in the PRESERVED legacy adapter and in the new one, byte-for-byte the same
  declaration. The open prediction is answered NO. The adapter's enum count did move `14 → 15`, so which enum
  is new is a separate, smaller question and belongs to that tree, not this one.
  **THE FINDING THIS LEAF DID NOT EXPECT: AXI's typed signal-declaration capture lost 115 declarations.**
  The declared inventory moves **289 → 159 distinct signals** and **every one of the 110 `*CHK` parity
  signals disappears** from SemanticIR; `table_signal_declaration_provenance` drops `411 → 265` records and
  `304 → 170` distinct signals (`115 → 0` ending in `CHK`). ~~**It is not a re-ingest artifact and the SourceIR
  proves it:** old and new SourceIR are structurally identical — 320 pages / 333 visual / **286 tables** /
  3,652 content elements / 527 sections — so the ingest is stable and the change is entirely in the EvidenceIR
  producer between the binary that wrote the legacy chain (`2026-08-12`) and today's.~~ **CORRECTED by `.10`
  (`2026-09-11`): those counts hold and every cell is byte-identical, but equal counts are not an identical
  artifact — `table_kind` differs on 67 of the 286 tables, `section_kind` on 56 of 527 sections and
  `diagram_kind` on 20 of 333 assets. The change is in the SourceIR classifier, not the EvidenceIR
  producer.** **Nothing is
  unrecoverable:** the raw table row survives in both (`statement_4788` = `| AWVALIDCHK | AWVALID | 1 |
  ARESETn |`); what stopped is the SYNTHESIS of the typed declaration the old chain also carried
  (`statement_6025` = `Signal AWVALIDCHK is width 1.`). ~~The source tables are shaped
  `Name | Signals covered | Width | Check enable` with a usable width, which is exactly what
  `_ => continue` in the width/direction synthesis (`crates/specforge/src/ir/evidence.rs`) is supposed to
  admit~~ — **CORRECTED by `.10`: those tables never reach that arm.
  `should_treat_table_as_top_level_signal_description` (`evidence.rs:4041`) rejects any table whose kind is
  not `SignalDescription`, and `dee0740f` had made these `unknown`.** The leaf was right to leave the
  attribution to a bisect rather than guess it; the located seam was the guess. Owned by
  `WIRE-BASED-100.10`, NOT fixed here; the six scored numbers are unaffected, which is precisely why a score
  alone would never have shown it. The AXI adapter also gained junk interface members minted from prose
  (`The`, `Asserted`, `Secure`, `Stream`, `VALID`, `PENDING`, …) and its actor count moved `21 → 134`; ~~both
  are routed to `.10` with the declaration loss because they share the table/actor synthesis boundary.~~
  **CORRECTED by `.10`: they share nothing measurable — restoring 118 declarations left the actor count at
  134 and removed no prose member. Re-routed to `.10b` as an independent regression.**
  **Same two dispositions as `.9b`/`.9c`:** the ISF adapter moved `renderable` → `blocked`
  (`ACLK` demoted out of `(clock ACLK)`), and the normalized bundle is HELD OUT at
  `generated/preserved/WIRE-BASED-100.9d/axi-normalized-bundle-held-out/` per
  `RETAINED-BUNDLE-POPULATION-FROZEN`.
  Preservation (repo-volume, `2026-09-10`, before the rebuild; 15 MB): `generated/preserved/
  WIRE-BASED-100.9d/pre-reingest/` — `source_ir.json` `e010c13c9007b7f7…`, `evidence_ir.json`
  `ea500fc02ed380c4…`, `semantic_ir.json` `80ba2ac6e61ae363…`, `intent_ir.json` `d20578e3d796f292…`,
  `adapters_isf/adapter.json` `d2dca5ad2c904ba2…`; full manifest in that directory's `SHA256SUMS.txt`.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.10` · Status: `done` (`2026-09-11`; cause attributed to a revision and a producer
  function, 123 declarations restored, and two of `.9d`'s own attributions corrected) · Goal: **recover
  AXI's 115 lost typed signal declarations, and the junk interface members that arrived with them.**
  Measured by `.9d` from artifacts, not inferred: the declared inventory was `289 → 159` distinct signals
  with all `110` `*CHK` parity signals gone, `table_signal_declaration_provenance` was `411 → 265` records /
  `304 → 170` distinct / `115 → 0` `*CHK`, and the actor count was `21 → 134` with prose words reaching the
  ISF interface.
  Prerequisite: `WIRE-BASED-100.9d`.

  **THE SEAM IS THE SourceIR CLASSIFIER, AND `.9d`'s TWO ATTRIBUTIONS ARE BOTH CORRECTED.** `.9d` placed
  the loss at the `_ => continue` width/direction arm in `crates/specforge/src/ir/evidence.rs` and
  exonerated the ingest because "the SourceIR is structurally identical". Neither holds:

  1. **The tables never reach that arm.** `should_treat_table_as_top_level_signal_description`
     (`crates/specforge/src/ir/evidence.rs:4041`) returns `false` unless
     `effective_table_kind(..) == TableKind::SignalDescription`, so a table classified `unknown` is never
     offered to `synthesize_signal_declarations` at all. The width arm was never reached and never guilty.
  2. **The SourceIR is not identical — `.9d` compared counts, and the classification fields moved.** The
     counts it quoted do hold (320 pages / 333 visual / 286 tables / 3,652 content / 527 sections) and every
     table's cell text is byte-identical across the re-ingest, but three *classification* fields differ
     between the `2026-08-12` chain and `.9d`'s: **`table_kind` on 67 of 286 tables** (`encoding → unknown`
     40, `signal_description → unknown` 17, `feature_matrix → unknown` 9, `timing_parameter → unknown` 1),
     **`section_kind` on 56 of 527 sections**, and **`diagram_kind` on 20 of 333 assets**. Equal counts are
     not an identical artifact; the ingest was never exonerated.

  **ATTRIBUTION — by re-deriving each revision's own producer, never a diff.** The five AXI tables are
  shaped `Name | Signals covered | Width | Check enable`. Running `classify_table_kind` extracted from
  `git show <rev>:crates/specforge/src/ir/source/docling_backend.rs` against that exact header row:

  | revision | date | leaf | verdict on the header row |
  | --- | --- | --- | --- |
  | `46af2eca` | `2026-08-10` | `CORPUS-COVERAGE.2.47a` | `signal_description` |
  | `f9434368` | `2026-08-12` | `SPEC-TO-INTENT-ALIGNMENT.6b.iii` | `signal_description` |
  | **`dee0740f`** | **`2026-08-12`** | **`SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`** | **`unknown`** ← the seam |
  | `e125aac7` · `2172ad9e` · `5f568381` · `HEAD` | `2026-08-15`…`2026-09-10` | — | `unknown` |

  `dee0740f` ("make SourceIR classification neutral") replaced substring role matching with **whole-label**
  role equality, so a header proves a role only when its entire normalized label *is* that role.
  `bb5047c2` (`2026-08-12`, `.6d.ii.e.iv.ii`) then made the Rust `classified_table_kind` the persisted
  authority and re-encoded the same narrowed rule — its `header_has_role` was whole-label equality with no
  qualifier admission at all.

  **INDEPENDENT CONFIRMATION FROM THE CORPUS ITSELF.** Classifying every table in all 77 persisted
  SourceIRs with both producers: **30 documents match HEAD's classifier exactly** (the re-ingested cohort)
  and **47 match the pre-seam classifier at 99–100%** and HEAD at far less. `table_kind` is a SourceIR
  field that no EvidenceIR producer can write, so the competing account `.9d` proposed — "the change is
  entirely in the EvidenceIR producer" — cannot produce this observation.

  **VERDICT: REGRESSION, NOT RETIREMENT — and the same repair was already adjudicated here once.** The
  narrowing's own rule is that a signal table needs a name role plus an *explicit* signal/port/pin column;
  `Name | Signals covered | Width | Check enable` satisfies it. Whole-label equality simply could not see a
  role carrying a qualifier. Three days after the narrowing, `e125aac7` (`.6d.ii.f.iv.a`, "restore
  structural register carriers") hit the identical wall for the register role and repaired it by admitting
  **one balanced parenthesized qualifier** — so "a closed role may carry a qualifier" is already this
  repository's ruling; only the un-parenthesized form was left unhandled. APB and AHB keeping their `*CHK`
  signals is the corroborating evidence that no blanket ADR 0006 retirement of parity capture was intended.

  **THE FIX.** `header_names_signals` (`crates/specforge/src/ir/source.rs`) and its embedded-Python mirror
  `classifier_header_names_signals`: a header proves the explicit-signal role when a **generic interface
  noun** (`signal`/`signals`/`port`/`ports`/`pin`/`pins`) appears as a **whole word** in its normalized
  label. That is the same authority form the caption rule already used (`Table … signals`), and the noun
  set is universal digital-interface vocabulary, never document identity (ADR 0006). It only ever *adds*
  the explicit-signal leg; a name role and a width are still both required.

  **BLAST RADIUS, MEASURED BEFORE SHIPPING — over all 77 persisted SourceIRs (11,033 tables).** Exactly
  **7 tables change, all `unknown → signal_description`**: the 5 AXI `*CHK` tables and 2 in
  `ihi0089_d` (AMBA LTI) of the same shape. Zero `register_map` / `encoding` / `timing_parameter` /
  `feature_matrix` changes. Two looser variants were measured and **rejected**: admitting a role as a
  label *prefix* minted **+425** register maps, and whole-word matching for *every* role minted **+1,277**
  encodings and **+35** feature matrices. Each of the 7 was read by hand and is a real declaration table.

  **ADDRESSED — re-ingested and re-derived (`2026-09-11`, route unchanged, no model server).** SourceIR
  reproduces `.9d` exactly except the 5 intended tables: 320 pages / 333 visual / 286 tables / 3,652
  content / 527 sections, **zero cell-text differences**, and **5 kind changes, all `unknown →
  signal_description`** on `table_0245`–`table_0249`.

  | measure | legacy `2026-08-12` | `.9d` `2026-09-10` | `.10` `2026-09-11` |
  | --- | --- | --- | --- |
  | `table_signal_declaration_provenance` records | 411 | 265 | **388** |
  | … distinct signals | 304 | 170 | **293** |
  | … distinct `*CHK` | 115 | 0 | **115** |
  | SemanticIR declared inventory | 289 | 159 | **277** (110 `*CHK`) |
  | SemanticIR interfaces | 263 | 298 | 321 |
  | ISF interface ports | 287 | 159 | **277** (110 `*CHK`) |

  **THE SIX SCORED NUMBERS ARE UNCHANGED** (`--provider skip`, the `-- source-tolerant + filtered
  (WIRE-BASED-100) --` blocks): `seed_axi` `signal_constraint` `P=R=F1=1.000` (tp=4 fp=0 fn=0),
  `actor_signal_relation` `1.000` (tp=6 fp=0 fn=0), document-level 4/4 and 6/6; `seed_axi_temporal`
  `1.000` (tp=3 fp=0 fn=0). That is the point of the leaf: 118 declarations came back and no score moved,
  exactly as no score moved when they left.

  **WHAT IS NOT FIXED, PUBLISHED WITH ITS COUNT RATHER THAN LEFT IMPLIED.** **30** distinct table-declared
  signals are still missing against the legacy chain, in two disjoint groups → `.10a`:
  `table_0255` (15) and `table_0251` (7) are `Name | Width | Source | Description` continuation pages that
  lost a *different* authority (`Source` stopped counting as direction, and a "Continued from previous
  page" caption does not name signals); `table_0059` (4), `table_0187` (3) and `table_0259` (1) are
  **still classified `signal_description`** and lost rows inside the synthesis — that residual is where
  `.9d`'s `_ => continue` hypothesis may genuinely apply, and `.10a` owns testing it.

  **THE JUNK ACTORS ARE NOT PART OF THIS DEFECT — `.9d`'s bundling premise is disproven by measurement.**
  `.9d` routed them here "because they share the table/actor synthesis boundary". Restoring 118
  declarations moved the actor count **134 → 134**, and all 11 prose members (`The`, `Asserted`, `Secure`,
  `Stream`, `VALID`, `PENDING`, `RP`, `CRDT`, `CRDTSH`, `SHAREDCRD`, `AxLEN`) are still in the ISF
  interface. They are an independent regression → `.10b`. (`ACLK`/`ARESETn` also appear as ports; that is
  the already-recorded `renderable → blocked` clock/reset disposition, not junk.)

  **THE COST THIS SLICE PAID, STATED PLAINLY.** This is a production-semantics change, so
  `SOURCE_PRODUCTION_SEMANTIC_SHA256` (build-generated from the reachable producer graph) moved and **every
  persisted SourceIR proof went stale — 26 documents**, which would have silently emptied the measurable
  census exactly the way a retired fix silently cost a published `1.000` in `.9b`. It was repaired inside
  this slice, not deferred: 24 documents were rebuilt from their retained normalized bundles with
  `source_proof_migrate --retained-manifest doctrine/chain_currency/retained_bundles.json --write`
  (**zero** `table_kind` / `section_kind` / `diagram_kind` / count changes across all 24 — the proof
  refreshed, the content did not), their `evidence → semantic → intent → adapt` chains were re-run, and
  APB and AHB were re-ingested because their bundles are held out under
  `RETAINED-BUNDLE-POPULATION-FROZEN`. The AXI bundle the re-ingest recreated is held out the same way at
  `generated/preserved/WIRE-BASED-100.10/axi-normalized-bundle-held-out/`, keeping the retained set at
  exactly the declared 24.
  Preservation (repo-volume, `2026-09-11`, 185 MB, before the rebuild): `generated/preserved/
  WIRE-BASED-100.10/pre-reingest/` — `source_ir.json` `40a17cb5fd86d8f9…`, `evidence_ir.json`
  `1395c5cfc37eedae…`, `semantic_ir.json` `4dc43aea1a1f3004…`, `intent_ir.json` `e04f22dda5bf605a…`,
  `adapters_isf/adapter.json` `0b84a318cab939ad…`; full manifest in that directory's `SHA256SUMS.txt`.
  Verification: see the acceptance checklist below.
  Commit: see log.

- ID: `WIRE-BASED-100.10a` · Status: `pending` · Goal: **the 30 AXI table declarations `.10` did not
  recover**, measured per table and split by mechanism rather than reported as one number.
  Group A (22): `table_0251` (7) and `table_0255` (15) are `Name | Width | Source | Description`
  continuation pages of `Table B1.1`/`B1.4`. Their first pages still classify (their captions name
  signals); the continuations do not, because `dee0740f` also removed `source`/`destination` from the
  direction roles and a "Continued from previous page" caption carries no signal noun. Decide the general
  question — **does a continuation caption inherit its parent table's kind, or is a `Source` column
  direction authority?** — and do not answer it by adding a caption phrase to a list.
  Group B (8): `table_0059` (4), `table_0187` (3), `table_0259` (1) are **already** `signal_description`
  and lose rows *inside* `synthesize_signal_declarations`. This is the only place `.9d`'s `_ => continue`
  hypothesis can still be true; test it against the 56 AXI sections whose `section_kind` moved
  `signal_description → normative` at the same seam, since that field feeds
  `infer_signal_direction_from_section` and therefore the `default_dir` a row falls back to.
  Acceptance: each group's mechanism named at `file:line`; recovered or its retirement published with the
  count; the six scored numbers unchanged; corpus blast radius measured over all persisted SourceIRs
  before shipping, as `.10` did.
  Prerequisite: `WIRE-BASED-100.10`.
  Verification: pending
  Commit: pending

- ID: `WIRE-BASED-100.10b` · Status: `pending` · Goal: **the 11 prose words in AXI's ISF interface and the
  `21 → 134` actor inflation**, now known to be independent of the declaration loss.
  `.10` measured the coupling `.9d` assumed: restoring 118 typed declarations moved the actor count
  `134 → 134` and removed none of `The`, `Asserted`, `Secure`, `Stream`, `VALID`, `PENDING`, `RP`, `CRDT`,
  `CRDTSH`, `SHAREDCRD`, `AxLEN`. APB (8 actors) and AHB (25) were re-ingested by the same binary and did
  not inflate, so this is AXI-shaped, not a blanket actor-synthesis change. Attribute it the way `.10`
  attributed the classifier — re-derive from each revision's own producer across `2026-08-12..HEAD` — and
  note that `AxLEN` reaching the interface *while the declared `AXLEN` disappeared* is one observation, not
  two. `.6c`'s `ir/entity_typing` bounded-LLM actor discrimination already exists and is the candidate
  general fallback.
  Acceptance: cause named with a revision and a producer function; the prose members gone or their
  presence explained; APB/AHB actor counts unmoved; the six scored numbers unchanged.
  Prerequisite: `WIRE-BASED-100.10`.
  Verification: pending
  Commit: pending

## Acceptance Checklist (enforced) — `WIRE-BASED-100.10` (RUST CODE CHANGE) — DONE `2026-09-11`

- [x] **REPRODUCE / MEASURE** — baseline from the persisted artifacts before any edit: AXI
  `table_signal_declaration_provenance` 265 records / 170 distinct / **0** `*CHK`, against the preserved
  legacy chain's 411 / 304 / **115**; SemanticIR declared inventory 159 vs 289; ISF ports 159 vs 287. The
  `.9d` chain (185 MB) was preserved with a SHA-256 manifest before the rebuild.
- [x] **ROOT CAUSE (WHY + WHERE)** — `dee0740f` (`2026-08-12`, `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`)
  narrowed `classify_table_kind`'s role matching to whole-label equality, so
  `Name | Signals covered | Width | Check enable` stopped being `signal_description`; `bb5047c2` re-encoded
  the narrowed rule in the Rust authority `classified_table_kind` (`crates/specforge/src/ir/source.rs`).
  Attributed by running each revision's own extracted producer on that header row, never by reading a
  diff. `.9d`'s located seam (`_ => continue` in `evidence.rs`) is corrected: an `unknown` table is
  rejected by `should_treat_table_as_top_level_signal_description` (`evidence.rs:4041`) and never reaches
  the synthesis. Independent oracle: 47 of 77 persisted SourceIRs match the pre-seam classifier and 30
  match HEAD's, which no EvidenceIR-side account can produce.
- [x] **ADDRESSED (verified)** — per-artifact, before → after: provenance records `265 → 388`, distinct
  `170 → 293`, `*CHK` `0 → 115`; SemanticIR declared inventory `159 → 277`; ISF interface ports
  `159 → 277`. SourceIR reproduces `.9d` with **zero** cell-text differences and exactly the 5 intended
  `unknown → signal_description` kind changes. The residual 30 is published per table and owned by `.10a`.
- [x] **NO REGRESSION** — named, re-runnable oracles, all green: `cargo test -p specforge-core --lib`
  (1,383 + the 3 new cases), `-p specforge --lib` (472), `-p specforge-conformance --lib` (168);
  `cargo fmt --all`; the six WIRE-BASED-100 numbers re-derived unchanged via
  `specforge eval-extraction … --provider skip`; `bash scripts/check_chain_currency.sh`;
  `bash scripts/check_doctrines.sh`. **Stated rather than implied:** the production-semantics digest moved
  and staled 26 SourceIR proofs; that was repaired inside this slice (24 migrated from retained bundles
  with zero content change, APB and AHB re-ingested), not deferred to a gate nobody runs.
- [x] **GENERICITY (ADR 0006)** — the rule reads a closed set of universal interface nouns as whole words
  in a column label. No chip, vendor, protocol, or document name; no corpus phrase (`signals covered` is
  never mentioned). Guarded by an alpha-renamed positive and four negatives, including the ambiguous
  `Name | Width | Description` layout the narrowing deliberately retired and a non-noun qualifier
  (`Values covered`), plus the Rust↔Python parity probe.
- [x] **LOCKSTEP** — mdBook, `CHANGES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `ROADMAP.md`, the resume pointer
  and a Knowledge Map fact card updated; `.9d`'s two corrected attributions rewritten where they were
  published; `.10a`/`.10b` opened.

## Acceptance Checklist (enforced) — `WIRE-BASED-100.9d` (RE-INGEST + RE-DERIVATION, closes `.9`) — DONE `2026-09-10`

- [x] **REPRODUCE / MEASURE** — baseline from the binary before the rebuild: `eval-extraction … seed_axi.json`
  and `… seed_axi_temporal.json --provider skip` both printed the UNMEASURABLE disposition, withholding 11 and
  2 gold items; the census read **26 measurable / 52 legacy**, gold documents 4 of 7. The outgoing chain
  (SourceIR 1 / EvidenceIR 2 / SemanticIR 1 / IntentIR 1, 15 MB) was preserved with a SHA-256 manifest first.
- [x] **ROOT CAUSE (WHY + WHERE)** — three, each named with a location. (1) The predicted `ARESETN`/`ARESETn`
  mismatch does not exist: `eval::temporal_predicate_key` (`crates/specforge/src/eval.rs:458`) uppercases every
  name, so case cannot move a score — which withdraws `.9b`'s second "route". (2) `AWATOP` is not newly
  minted: `(type AWATOP (bits 4))` is identical in the preserved legacy adapter and the new one. (3) ~~The 115
  lost typed declarations are located at the synthesis seam, not the ingest: the SourceIR is structurally
  identical across the re-ingest (320/333/286/3,652/527) … the `_ => continue` width/direction arm in
  `crates/specforge/src/ir/evidence.rs`.~~ **WITHDRAWN by `.10` (`2026-09-11`): the seam is the SourceIR
  table classifier (`dee0740f`), the SourceIR is not identical in its classification fields, and an
  `unknown` table never reaches that arm.** What survives is the observation this leg was built on — the
  raw row lives in both chains (`statement_4788`) and only the synthesized declaration
  (`statement_6025`) stopped — and the decision to leave attribution to `.10`'s bisect rather than guess
  it; the located seam was the part that was guessed.
- [x] **ADDRESSED (verified)** — the chain is canonical and scoreable: `ingest` → 320 page artifacts / 333
  visual / 0 residuals / `high`; `evidence` → 527 anchors / 5,909 spans / 220 links / 6,283 statements;
  `semantic` → 134 actors / 298 interfaces / 923 invariants; `intent` → 129 actors / 604 behaviors / 939
  constraints; schema **3 / 3 / 2 / 2**. Scores: `seed_axi` constraints `1.000` (tp=4 fp=0 fn=0) and relations
  `1.000` (tp=6 fp=0 fn=0, source-tolerant + filtered), document-level 4/4 and 6/6; `seed_axi_temporal`
  `1.000` (tp=3 fp=0 fn=0). Census **27 measurable (34.6%) / 51 legacy**, gold documents **5 of 7** — closing
  `.9`.
- [x] **NO REGRESSION** — in the scored sense: no Rust source touched, the AXI gold files are byte-identical,
  and all six numbers hold. **Stated plainly rather than implied: a real un-scored regression WAS found** (115
  typed declarations, 130 declared signals, 113 spurious actors) and is owned by `.10` rather than reported
  and dropped. Named re-runnable oracles: `bash scripts/check_chain_currency.sh`, `bash
  scripts/check_doctrines.sh`.
- [x] **GENERICITY (ADR 0006)** — N/A for the rebuild. `.10` must decide whether the declaration loss is a
  genericity retirement or a defect, and APB/AHB keeping their `*CHK` signals is the evidence that says it is
  not a blanket retirement.
- [x] **LOCKSTEP** — `.9` is closed with its acceptance quoted against the measured result; `.9b`'s
  two-routes claim is withdrawn where it was published (tree, `CHANGES.md`); `KG-ISF-COMPLETENESS.5.iv.a`'s
  `AWATOP` prediction is answered; `.10` is opened; the corpus counts that moved (26 → 27 measurable, 52 → 51
  legacy, 26 → 27 adapter manifests) are corrected in the book, `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md`,
  `ROADMAP.md` and the resume pointer.

## Acceptance Checklist (enforced) — `WIRE-BASED-100.9c` (RE-INGEST + RE-DERIVATION) — DONE `2026-09-10`

- [x] **REPRODUCE / MEASURE** — baseline from the binary before the rebuild:
  `eval-extraction … seed_ahb.json --provider skip` and `… seed_ahb_temporal.json --provider skip` both printed
  the UNMEASURABLE disposition (`persisted EvidenceIR is schema 2`), withholding 17 and 4 gold items and
  scoring nothing; the census read **25 measurable / 53 legacy**, gold documents 3 of 7. The outgoing chain
  (SourceIR 1 / EvidenceIR 2 / SemanticIR 1 / IntentIR 1) was preserved with a SHA-256 manifest first.
- [x] **ROOT CAUSE (WHY + WHERE)** — N/A as a defect: every carried number re-derived. The leaf's real
  question was whether `.9b`'s regression generalises, and the answer is derived rather than assumed —
  the AHB temporal gold's only antecedent is `HREADY` (declared with the prose spelling), and no AHB gold
  references the un-indexed `HSEL`, so `.4a`'s deleted resolver has nothing scored to lose here. Recorded as a
  correction to `.9b`'s prediction and to `.4a`'s scope note.
- [x] **ADDRESSED (verified)** — the chain is canonical and scoreable. `ingest` → 104 page artifacts / 70
  visual / 0 residuals / `high`; `evidence` → 172 anchors / 1,199 spans / 481 links / **1,322** statements —
  identical to the preserved legacy artifact, which is why `content-anchored` re-resolved `0/17` and `0/4`
  (ids already current); `semantic` → 25 actors / 41 interfaces / 223 invariants; `intent` → 23 actors / 64
  behaviors / 225 constraints; schema **3 / 3 / 2 / 2**. Scores: `seed_ahb` constraints `1.000`
  (tp=6 fp=0 fn=0) and relations `1.000` (tp=6 fp=0 fn=0, source-tolerant + filtered), document-level 6/6 and
  6/6; `seed_ahb_temporal` `1.000` (tp=4 fp=0 fn=0). Census now **26 measurable (33.3%) / 52 legacy**, gold
  documents **4 of 7**.
- [x] **NO REGRESSION** — no Rust source touched; the AHB gold files are byte-identical (`git status` shows no
  change under `crates/specforge/test_data/llm_eval/`). Named re-runnable oracles: `bash
  scripts/check_chain_currency.sh` and `bash scripts/check_doctrines.sh`. The ISF adapter's
  `renderable` → `blocked` move and the `HSELX` → `HSELx` identity move are the same two dispositions `.9b`
  established, and no ISF signal is lost (`HCLK` enters the interface, `HSELX`/`HSELXCHK` become
  `HSELx`/`HSELxCHK`).
- [x] **GENERICITY (ADR 0006)** — N/A for the rebuild (no rule changed); the observed identity move is
  identifier opacity doing what ADR 0006 requires.
- [x] **LOCKSTEP** — the tree's `.9d` expectations, `.4a`'s scope note and `.9b`'s prediction are corrected
  where they were published; the corpus counts that moved (25 → 26 measurable, 53 → 52 legacy, 25 → 26 adapter
  manifests) are corrected in the book, `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md` and the resume pointer.

## Acceptance Checklist (enforced) — `WIRE-BASED-100.9b` (RE-INGEST + RE-DERIVATION) — DONE `2026-09-10`

- [x] **REPRODUCE / MEASURE** — baseline captured before the rebuild, from the binary itself:
  `eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider skip` printed the UNMEASURABLE
  disposition (`persisted EvidenceIR is schema 2, below the current canonical schema 3`), withheld all 16 gold
  items and scored nothing; `scripts/measure_corpus_canonical_currency.py` read **24 measurable (30.8%) / 54
  legacy**, gold documents **2 of 7** measurable. The outgoing chain was SourceIR 1 / EvidenceIR 2 /
  SemanticIR 1 / IntentIR 1 and was preserved with a SHA-256 manifest **before** any command wrote over it.
- [x] **ROOT CAUSE (WHY + WHERE)** — for the one number that moved. `.4`'s `resolve_indexed_signal_family`
  (`crates/specforge/src/ir/semantic.rs`, added `1c28516b`) is **gone**, deleted by `f88d463d`
  (`SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii`, *make document identifiers opaque*), which also installed the
  opposite behaviour as a tested invariant — `temporal_condition_does_not_alias_an_undeclared_name_from_suffix_spelling`
  asserts that with `PSELX` declared, `"PSEL is asserted"` yields no predicate. Attribution is by
  producer-history, not by reading a diff: `git log -S resolve_indexed_signal_family -- crates/specforge/src/ir/semantic.rs`
  returns exactly two commits, the leaf that added it and the leaf that removed it. Consequence in the
  artifacts: `temporal_signal_constraint_sigcon_0009` (`PNSE`) has `antecedents: []` and `…_0014` (`PBUSER`)
  has `PENABLE`+`PREADY` only, against a gold that requires `PSELX`.
- [x] **ADDRESSED (verified)** — the chain is canonical and scoreable again, per stage and per fact.
  `DOCLING_DEVICE=cpu ingest` → 48 page artifacts / 35 visual / 0 residuals / `high`, normalized bundle
  RESTORED; `evidence` → 99 anchors / 517 spans / 150 links / **598** statements (was 597 — the
  re-segmentation `.1`'s content anchoring exists to absorb); `semantic` → 8 actors / 35 interfaces / 65
  invariants; `intent` → 6 actors / 16 behaviors / 66 constraints; schema now **3 / 3 / 2 / 2**. Scores:
  `seed_apb` signal_constraint `1.000` (tp=6 fp=0 fn=0) and actor_signal_relation `1.000` (tp=5 fp=0 fn=0,
  source-tolerant + filtered), document-level recall 6/6 and 6/6 — all identical to the carried numbers;
  `seed_apb_temporal` `0.333` (tp=1 fp=2 fn=2) — **withdrawn, not carried**, with both missed keys printed by
  the scorer. `scripts/measure_corpus_canonical_currency.py` now reads **25 measurable (32.1%) / 53 legacy**
  and **3 of 7** gold documents measurable.
- [x] **NO REGRESSION** — no Rust source touched, so the extractor's behaviour on every other document is
  unchanged by construction; the movement observed here is a re-measurement of behaviour that shipped on
  `2026-08-12`. Named re-runnable oracles: `bash scripts/check_chain_currency.sh` (replays every persisted
  stage against the current binary, including the newly current APB chain, and compares the retained-bundle
  declaration with what is on disk) and `bash scripts/check_doctrines.sh`. The APB gold files are byte-identical
  — `git status` shows no change under `crates/specforge/test_data/llm_eval/`.
- [x] **GENERICITY (ADR 0006)** — N/A for the rebuild (no rule changed). It is, however, the substance of the
  finding: `f88d463d` removed a spelling-derived inference precisely because ADR 0006 forbids identifier
  authority, and `.4a` must recover the fact without resurrecting it.
- [x] **LOCKSTEP** — the tree's `2026-06-06` "100% on ALL three aspects" headline is withdrawn where it was
  published, `.4` records that its fix was retired, `.4a` owns the recovery, and the corpus counts that moved
  (24 → 25 measurable, 54 → 53 legacy, 24 → 25 adapter manifests; retained bundles deliberately unchanged at
  24) are corrected in the book, `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md` and the resume pointer, and the
  frozen-population finding is opened as `RETAINED-BUNDLE-POPULATION-FROZEN`.
  KM `[[corpus-canonical-currency-and-ownership]]`, `[[chain-currency-doctrine]]`.

## Acceptance Checklist (enforced) — `WIRE-BASED-100.9a` (MEASUREMENT) — DONE `2026-09-01`

- [x] **REPRODUCE / MEASURE** — read-only census with the tracked deterministic reproducer
  `scripts/measure_corpus_canonical_currency.py` (no model, no rebuild, no write; repository-root-relative).
  78 persisted documents → **24 measurable (30.8%) / 54 legacy**, stratified totally at every stage (SourceIR
  54×1 / 24×3, EvidenceIR 54×2 / 24×3, SemanticIR 54×1 / 24×2, IntentIR 54×1 / 24×2). Legacy ownership
  partitions **18 unowned + 31 declared-refreshed + 5 declared-remaining = 54**. Gold documents 7 → **2
  measurable**. Cross-checked live against the binary: `eval-extraction crates/specforge/test_data/llm_eval/
  seed_apb.json --provider skip` prints the UNMEASURABLE disposition, withholds all 16 gold items, and scores
  nothing. Report `docs/research/corpus-canonical-currency-census.md`.
- [x] **ROOT CAUSE (WHY + WHERE)** — the ownership gap is structural, not an oversight in wording.
  `doctrine/corpus_frontier/census.json` declares `cohort_rule.excluded_source_prefixes: ["corpus/"]`, and
  `scripts/check_corpus_frontier_census.pl` documents why: *"`corpus/` is the tracked in-repo gold/eval
  corpus — copied into the repository, never part of the host-library refresh program."* So `.8`'s hand-off
  named an owner whose contract excludes the documents handed to it. The second half of the gap is that
  nothing ties `refreshed` to a schema, so 31 documents are simultaneously "refreshed" and refused by every
  canonical gate. **And the deeper cause, found only by auditing CLOSED leaves:** the wire chains were
  refreshed by `CORPUS-PATTERN-REUSE.3c` (`done`, `2026-06-09`, re-verified at `1.000`), last written
  `2026-08-09`, and made legacy by the `2026-08-12` schema bump three days later — with **no gate reporting
  it**, because no check fails when a persisted chain falls below the canonical schema.
- [x] **CHALLENGED ON REVIEW, AND TWO PUBLISHED CLAIMS DID NOT SURVIVE** (director audit before commit;
  `[[specforge-verify-before-publishing]]`). (a) *"APB/AHB/AXI had no owning leaf at all"* — **FALSE as
  written.** `CORPUS-PATTERN-REUSE.3c` owned and performed exactly this re-ingest and closed; the true claim
  is "no OPEN owner", and the corrected story points at a missing currency gate rather than at neglect.
  Corrected everywhere before commit, and the search that would have caught it is now in the fact card's
  `reverify`. (b) *"24 of 78 can be scored"* — **an admission bound, not a demonstration.** Only 2 of the 24
  carry a gold; the other 22 pass the schema gate with nothing to score against, and a non-schema refusal
  aborts rather than dispositions. Restated as a bound. Every other conclusion re-derived unchanged: the
  cohort-rule exclusion, 24/54, 18/31/5, 31-of-52, 7-golds-2-measurable, and the live `seed_apb` refusal.
- [x] **DECISION (per-item, `[[feedback_scoring_rigor]]`)** — **GO** on owning the wire re-ingest here as
  `.9b`/`.9c`/`.9d`, smallest document first, each with its own before/after evidence. **NO-GO** on absorbing
  the three shortfalls that are not wire-protocol work: RISC-V Debug (`PDF-VARIANT-DIGESTION` register class),
  the 31 refreshed-but-legacy cohort documents (a corpus-program question about what `refreshed` should mean),
  and the missing canonical-currency gate (a `DOCTRINE-ENFORCEMENT`-class decision). All three are recorded in
  `.9`'s node as findings routed out, so the next session finds them without this tree claiming them.
  **Feasibility is demonstrated rather than assumed** — the three PDFs are git-tracked by `.5d`, the
  repo-local Docling runtime is ready, the three measurable `corpus/`-sourced documents were produced through
  this route after the `2026-08-12` schema bump, and `CORPUS-PATTERN-REUSE.3c` ran this exact sequence on
  these exact documents on `2026-06-09`.
- [x] **HONEST CAUTION RECORDED, NOT RESOLVED** — a re-ingest destroys evidence the current binary cannot
  regenerate, and every wire number published before `2026-08-12` was measured on that evidence. `.9b` carries
  a preserve-before-rebuild acceptance item so the before/after comparison is exact rather than remembered, and
  the publishing posture is `.8c`'s: a re-derived number replaces the published one, and a number that cannot
  be re-derived is withdrawn rather than carried.
- [x] **NO CODE → ORACLES ORTHOGONAL** — measurement/docs-only; no Rust touched, so WIRE-BASED-100, the
  register/wire golds, and `kg-bench` are orthogonal by construction. `scripts/check_doctrines.sh` GREEN.
- [x] **GENERICITY (ADR 0006)** — no document, vendor, or protocol name participates in the reproducer's
  logic. The canonical schema of each stage is read from the Rust constant that defines it; frontier ownership
  is read from the frontier's own contract including its cohort rule; the scored-document set is derived from
  the tracked eval datasets' own `doc_key` fields. Reading the constants rather than restating them is what
  makes the census stale-detecting under a schema bump instead of self-confirming.
- [x] **LOCKSTEP** — report `docs/research/corpus-canonical-currency-census.md` + tracked reproducer
  `scripts/measure_corpus_canonical_currency.py`; `.9`/`.9a`/`.9b`/`.9c`/`.9d` nodes; `.8`'s incorrect
  hand-off corrected in place where it was published; KM card
  `[[corpus-canonical-currency-and-ownership]]` + `KNOWLEDGE_MAP.md`/shard/fact-catalog regeneration;
  `CHANGES.md` / `MEMORY.md`. **`LIVE_ACHIEVEMENT_STATUS.md` is deliberately unchanged**: its routing contract
  moves it only when the current product-status snapshot moves, and a read-only measurement ships no
  capability — the product does today exactly what it did before, and says so. **`ROADMAP.md` is deliberately
  unchanged**: the near-term north star and the wire gate are unchanged; what moved is who owns a leaf inside
  `WIRE-BASED-100`, which the task tree owns. **The mdBook is deliberately unchanged**: `quality/
  extraction-eval.md` already tells the reader that the APB/AHB/AXI golds are refused as proofless until
  re-ingested and that a published score is worth only its last re-derivation, so nothing it states is now
  false; the chapter changes when `.9b` actually re-derives a score, which is a change in what the product can
  demonstrate rather than a change of owner. Editing it here would also perturb the frozen book quantitative
  census for no truth gain, which the routing contract forbids ("a path must never be edited merely to prove
  that it was reviewed").

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

- `2026-09-10` (`.9d` done, `.9` CLOSED, `.10` opened): **every AXI number holds, both of the leaf's
  predictions were wrong, and the re-ingest exposed a 115-declaration loss no score could see.** All six AXI
  numbers re-derive — constraints `1.000` (tp=4 fp=0 fn=0), relations `1.000` (tp=6 fp=0 fn=0), document-level
  4/4 and 6/6, temporal `1.000` (tp=3 fp=0 fn=0) — taking the corpus to **27 measurable (34.6%) / 51 legacy**
  and gold documents to **5 of 7**, which closes `.9`: all six datasets score, and the two documents that
  still cannot are exactly the ones `.9a` routed out. **Prediction 1 disproven, and it withdraws a `.9b`
  claim:** AXI temporal was predicted to lose the `ARESETN`/`ARESETn` item; it scored `3/3` because
  `eval::temporal_predicate_key` (`eval.rs:458`) uppercases every name, so case can never move a score — the
  sole cause of APB's loss is the deleted resolver, and `.9b`'s "two independent routes" is withdrawn.
  **Prediction 2 disproven:** `KG-ISF-COMPLETENESS.5.iv.a`'s open `AWATOP` prediction is answered NO —
  `(type AWATOP (bits 4))` is identical in the preserved legacy adapter and the new one. **And the finding
  nobody was looking for:** AXI's declared inventory fell `289 → 159` signals with all `110` `*CHK` parity
  signals gone (`table_signal_declaration_provenance` `411 → 265` records, `115 → 0` `*CHK`) while the actor
  count rose `21 → 134` with prose words reaching the ISF interface. The SourceIR is structurally identical
  across the re-ingest and the raw table row survives in both chains, so the loss sits in typed-declaration
  synthesis, not ingest — owned by `.10`, attributed by bisect there rather than guessed here. **Twelve
  carried numbers re-derived across `.9`: eleven held, one was withdrawn, and the largest defect found was
  invisible to all twelve.**

- `2026-09-10` (`.9c` done): **AHB survives its re-ingest intact, and `.9b`'s prediction that it would not is
  corrected.** All four carried numbers re-derive exactly — constraints `1.000` (tp=6 fp=0 fn=0), relations
  `1.000` (tp=6 fp=0 fn=0), document-level 6/6 and 6/6, temporal `1.000` (tp=4 fp=0 fn=0) — moving the corpus
  to **26 measurable (33.3%) / 52 legacy** and gold documents to **4 of 7**. The reason AHB is not APB is
  exact: its temporal gold's only antecedent is `HREADY`, declared with the spelling the prose uses, and no
  AHB gold names the un-indexed `HSEL`. `.4a`'s scope is narrowed to a gold whose antecedent names an
  indexed-family signal by its un-indexed prose spelling — while noting the mechanism still loses every such
  antecedent silently wherever no gold scores it. The rebuild was structurally inert (same 1,322 statements /
  172 anchors / 1,199 spans / 481 links / 70 visual as the preserved legacy artifact), so the golds needed no
  re-anchoring. Same two dispositions as `.9b`: the ISF adapter moved `renderable` → `blocked`
  (`HCLK` demoted out of `(clock …)`) and the declared select became `HSELx`/`HSELxCHK` under identifier
  opacity, with no signal lost; the normalized bundle is HELD OUT per
  `RETAINED-BUNDLE-POPULATION-FROZEN`.

- `2026-09-10` (`.9b` done, `.4a` opened): **APB is scoreable again, and the price of finding out is one
  withdrawn number.** The re-ingest ran the demonstrated route unchanged and restored the chain to canonical
  3/3/2/2, moving the corpus to **25 measurable (32.1%) / 53 legacy** and gold documents to **3 of 7**.
  `seed_apb` re-derives EXACTLY — constraints `1.000` (tp=6 fp=0 fn=0), relations `1.000` (tp=5 fp=0 fn=0),
  document-level recall 6/6 and 6/6. `seed_apb_temporal` does not: `0.333` (tp=1 fp=2 fn=2) against the carried
  `1.000`, so the `2026-06-06` "100% on ALL three aspects" headline is **withdrawn**. Cause, attributed from
  producer history rather than a diff: `.4`'s `resolve_indexed_signal_family` was deleted by
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` (`f88d463d`, `2026-08-12`) when identifiers became opaque, and the
  opposite behaviour now ships as a test — so the gold's canonical `PSELX` antecedent is unproducible and
  `PNSE`/`PBUSER` each lose one. **The re-ingest did not cause it:** the preserved pre-rebuild SemanticIR
  already carries both defects, so the loss had been persisted and unscoreable — invisible — for four weeks.
  That is the unowned no-canonical-currency-gate defect `.9a` routed out, now measured in a real published
  number. Recovery is `.4a`; the gold was not touched. The ISF adapter also moved `renderable` → `blocked`
  (`no source-grounded system clock/reset contract`), which is the documented policy for the whole measurable
  stratum, not a new defect.

- `2026-09-01` (`.9` opened, `.9a` done): **`.8` closed by handing the wire re-ingest to an owner that
  excludes it, and 5 of 7 gold documents cannot be scored.** Censused read-only
  (`scripts/measure_corpus_canonical_currency.py`): 78 persisted documents are **24 measurable (30.8%) / 54
  legacy**, and the legacy set partitions **18 unowned + 31 frontier-declared-`refreshed`-but-still-legacy +
  5 frontier-declared-`remaining`**. The refresh frontier `.8` handed off to declares
  `excluded_source_prefixes: ["corpus/"]`, so every in-repo gold/eval document is outside its cohort by
  construction — its `5 remaining` would still read `5` after every wire gold had rotted. Two green gates
  publish coverage (`24/24 current`, `52 refreshed + 5 remaining`) and neither answers "how much can be
  scored". **The cause, found by auditing closed leaves on review:** `CORPUS-PATTERN-REUSE.3c` refreshed these
  documents on `2026-06-09` and re-verified `1.000`; they were last written `2026-08-09` and the `2026-08-12`
  schema bump made that refresh legacy with no gate reporting it. `.9` now owns the re-ingest as `.9b` APB /
  `.9c` AHB / `.9d` AXI, smallest first; RISC-V Debug, the 31 refreshed-but-legacy documents, and the missing
  canonical-currency gate are routed out rather than absorbed. KM
  `[[corpus-canonical-currency-and-ownership]]`.
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
