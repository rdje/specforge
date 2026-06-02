# TEMPORAL-RULE-EVAL: supervised precision/recall/F1 for mined temporal rules

## Metadata

- Tree ID: `TEMPORAL-RULE-EVAL`
- Status: `done` (CLOSED `2026-06-02` — `.1`–`.4` done; supervised temporal-rule P/R/F1
  live on real APB at P=0.400/R=0.667, surfacing a real antecedent-under-capture + degenerate
  header finding)
- Roadmap lane: `R15d`/`R15e` (extraction evaluation / completeness)
- Created: `2026-06-02`
- Owner: repo-local workflow
- Parent context: the **first Tier-1 item** of the `LITERATURE-GROUNDING`
  reach-full-potential backlog (`docs/research/grounding/README.md`), flagged independently
  by the protocol/temporal grounding (`protocol-temporal-semantics.md`) and the
  IE-evaluation grounding (`extraction-evaluation.md`): *"extend the eval with a
  `temporal_rule` fact kind so mined temporal properties get a precision/recall score
  (mirrors spec-miner evaluation)."* SpecForge's `temporal_rules` are the
  `G(antecedent → consequent)`-with-holes LTL template shape that GoldMine/Texada
  formalized — and the spec-mining literature **evaluates mined properties against a gold
  set with precision/recall**. SpecForge measures `signal_constraint` and
  `actor_signal_relation` this way already (`LLM-EXTRACTION-EVAL`, CLOSED); temporal rules
  are the unmeasured third extraction surface.

## Goal

Give the **temporal-rule extraction** a supervised, closed-world precision/recall/F1 score
against a hand-labeled gold set, reusing the existing `eval.rs` scoring machinery — so the
same measure→catch→fix loop that found `CONSTRAINT-SUBJECT-PRECISION` can run on temporal
rules. Concretely:

1. A new `EvalTask::TemporalRule` (label `temporal_rule`) and `GoldFact::TemporalRule`
   variant whose canonical key normalizes the **semantic content** of a rule (clock edge,
   antecedent predicates, consequent predicates, cycle window) and is provenance-free.
2. A `temporal_rule_record_key` that maps a produced `TemporalRuleRecord`
   (`crates/specforge/src/ir/semantic.rs`) to the **same** canonical key, so set overlap
   measures genuine agreement (mirrors `signal_constraint_record_key` /
   `actor_signal_relation_record_key`).
3. A small hand-labeled temporal gold seed (APB), gold drafted **independently from the
   prose**, with ≥2 negative items (statements that must produce *no* temporal rule).
4. A runner path that builds the temporal rules from a source EvidenceIR (the deterministic
   built-in temporal parser during EvidenceIR→SemanticIR lowering is the primary producer)
   and scores the produced set vs gold — no corpus mutation, dependency-injected like
   `eval-extraction`'s `build_predictions`.

## Producer (precise)

`TemporalRuleRecord`s are produced by the **built-in temporal parser** during the
EvidenceIR→SemanticIR lowering (`semantic.rs`), carried onto `SemanticIr.temporal_rules`
(`ir/semantic.rs:79`) and `IntentIr.temporal_rules` (`ir/intent.rs:80`). This eval scores
that **deterministic** extraction (spec-miner-style P/R of mined properties). Whether an LLM
pass (`nlp-enrich`/VLM) also contributes temporal rules — and should be scored as a separate
tier — is an Open Question for a later leaf (it also sets up a future capture–recapture
recall gauge over a `FactKind::TemporalRule`, per `extraction-evaluation.md`).

## Canonical key design (semantic, provenance-free)

A rule's identity for scoring = `(edge, sorted antecedent predicate-keys, sorted consequent
predicate-keys, cycle_window)`. Excluded from the key: `rule_id`, `source_text`,
`supporting_statement_ids`, `automation_confidence` (provenance/confidence, not identity).
Each `TemporalPredicateRecord` lowers to a normalized predicate-key:

- signals/actors `trim().to_ascii_uppercase()`, values `trim().to_ascii_uppercase()`,
  phases/edges as their snake_case strings — exactly the normalization
  `constraint_key`/`relation_key` already use.
- antecedents and consequents are **sorted** before joining (rule semantics are
  order-insensitive across the conjunction), so two records with the same predicates in a
  different emission order collapse to one key.
- `cycle_window` → `min..max` with empty slots blank (mirrors `target.unwrap_or("")`).

The gold `GoldFact::TemporalRule` carries the same logical fields (edge, antecedents,
consequents as a small typed sub-schema, optional cycle window) so `canonical_key()` is
computed identically on both sides.

## Non-Goals

- NOT changing any extraction behavior — this is a pure measurement surface (additive
  `eval.rs` types + a runner path + test_data). Zero change to `TemporalRuleRecord`
  production or the IRs.
- NOT an LLM-only eval — the primary producer is the deterministic temporal parser
  (contrast `LLM-EXTRACTION-EVAL`, which scores `nlp-enrich`/`signal-resolve`).
- NOT manufacturing gold — gold is drafted independently from the prose; a label that
  cannot be justified from the statement is dropped, not guessed.
- NOT a capture–recapture recall gauge for temporal rules (that is a separate downstream
  tree; this provides the canonical key it would reuse).

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: `EvalTask::TemporalRule` + `GoldFact::TemporalRule` + `temporal_rule_record_key` +
  predicate-key normalization + prediction indexing, with unit tests (gold↔record key
  agreement; order-insensitivity; negative item; cycle-window equality). No producer/command
  change. `scripts/run_ci.sh` green.
- `.3`: hand-labeled APB temporal gold seed under `crates/specforge/test_data/llm_eval/`
  (≥6 items incl. ≥2 negatives), gold drafted independently from prose; loads via the
  existing dataset loader (extended for the new variant).
- `.4`: runner path scores the built temporal rules vs gold (dependency-injected; no corpus
  mutation), report integrated; verified on a real EvidenceIR; book note in the topically
  correct chapter (`quality/extraction-eval.md` or `domain/temporal-semantics.md`); per the
  BOOK-METHOD-DOC close-rule the closing leaf refreshes the book; tree CLOSED.

## Task Tree

- ID: `TEMPORAL-RULE-EVAL`
  Status: `active`
  Children: `.1` (design) · `.2` (scorer types + keys) · `.3` (gold seed) · `.4` (runner +
    book + close)

- ID: `TEMPORAL-RULE-EVAL.1`
  Status: `done`
  Goal: own + design (this file) — the eval extension surface, the semantic provenance-free
    canonical-key design, the precise producer, the gold-seed plan, and the
    no-behavior-change / anti-fabrication discipline. Docs-only.
  Acceptance: design recorded + registered in `docs/TASK_TREE.md`.
  Verification: passed (`2026-06-02`) — design fixed against the real code: `eval.rs`
    `EvalTask`/`GoldFact`/`canonical_key`/closed-world `score_dataset` structure and the
    `TemporalRuleRecord` + `TemporalPredicateRecord` shapes (`ir/semantic.rs:884–947`) read
    directly (not from memory); canonical key normalizes (edge, sorted antecedent/consequent
    predicate-keys, cycle_window) and excludes provenance/confidence; producer identified as
    the deterministic temporal parser in the EvidenceIR→SemanticIR lowering; LLM-temporal
    contribution + capture–recapture recall deferred as Open Questions. Docs-only.
  Commit: `see Commit Log`

- ID: `TEMPORAL-RULE-EVAL.2`
  Status: `done`
  Goal: `EvalTask::TemporalRule` + `GoldFact::TemporalRule` (typed predicate sub-schema) +
    `temporal_rule_record_key` + a `temporal_predicate_key` helper + prediction indexing;
    extend `GoldFact::task`/`canonical_key` and the report task list. Pure types + keys, no
    producer change.
  Acceptance: unit tests (gold↔record agreement; antecedent/consequent order-insensitivity;
    negative item; cycle-window in/equality); `scripts/run_ci.sh` green.
  Verification: passed (`2026-06-02`) — added to `crates/specforge/src/eval.rs`:
    `EvalTask::TemporalRule` (label `temporal_rule`), `GoldFact::TemporalRule { edge,
    antecedents, consequents, cycle_window }` (authored with the IR's own
    `TemporalPredicateRecord` shapes so gold↔record keys are computed identically),
    `temporal_predicate_key` (per-variant normalized: signals/actors/values uppercased,
    phases/edges as snake_case, kind-tagged), `temporal_rule_key` (clock edge + **sorted**
    antecedent/consequent predicate-keys + cycle_window; provenance/confidence excluded),
    `temporal_rule_record_key`, and `index_temporal_rule_predictions`. The closed-world
    `score_dataset` needed **no change** (already generic over `item.task` +
    `GoldFact::canonical_key`). 3 new unit tests: gold↔record key match with reversed
    consequent order + lower-cased signals (order- + case-insensitive); window/edge
    discrimination; closed-world score over a labeled temporal statement (TP + spurious FP +
    ignored unlabeled). The runner (`commands/eval_extraction.rs`) keeps compiling honestly:
    its LLM-command path returns a clear error for `TemporalRule` (temporal rules come from
    the deterministic parser → `.4`), and the test closure gets an `unreachable!` arm.
    **Zero extraction-behavior change.** Full `scripts/run_ci.sh` GREEN (1211→1214 tests;
    fmt/clippy-D/rustdoc-D/mdBook all pass).

- ID: `TEMPORAL-RULE-EVAL.3`
  Status: `done`
  Goal: hand-labeled APB temporal gold seed (≥6 items, ≥2 negatives), gold drafted
    independently from prose; extend `load_eval_dataset`/validation for the new variant.
  Acceptance: dataset loads + validates; each gold item justifiable from its statement.
  Verification: passed (`2026-06-02`) — `crates/specforge/test_data/llm_eval/seed_apb_temporal.json`
    written: 6 `temporal_rule` items over AMBA APB (IHI0024_E), all real `statement_id`s,
    labels judged independently from the prose using the recorded IR vocabulary. Composition:
    2 clean positives (`statement_0285` PNSE-valid-when-PSEL-asserted; `statement_0221`
    Requester-drives-PSTRB-LOW), 1 faithful-gold antecedent **under-capture** case
    (`statement_0339` PBUSER valid when PSEL+PENABLE+PREADY — gold keeps all three; the
    parser is expected to drop PSEL+PENABLE → a recall gap the seed surfaces), and 3
    negatives (`statement_0419` + `statement_0238` list-introducer headers whose
    self-referential rules are degenerate FPs; `statement_0003` a licence notice the
    extractor should ignore). New loader test `committed_temporal_seed_loads_and_validates`
    (≥6 items, all temporal_rule, ≥2 negatives, well-formed edge-led keys, the 3-condition
    PBUSER antecedent present). `README.md` documents the temporal seed + the temporal gold
    schema. Full `scripts/run_ci.sh` GREEN (1214→1215). Anti-fabrication held — labels are
    correctness judgments from prose, not copied predictions (the under-capture case is
    deliberately authored to *differ* from the extractor).
  Producer calibration (`2026-06-02`, recorded so `.3` resumes without re-deriving): the APB
    SemanticIR (built in-memory via `cargo run -p specforge -- semantic
    generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json
    --dry-run`) emits **153 `temporal_rules`**. Representation conventions observed (so the
    gold uses the *same* vocabulary while correctness is judged independently from prose):
    `edge: "rising"`, `clock_signal: "PCLK"`; a `"must be valid"` constraint lowers to a
    consequent `signal_value {signal, "VALID", post_tick}` with `cycle_window {1,1}`, plus an
    `actor_drives_signal {actor, signal, post_tick}` consequent **when a driver actor is
    known**; rules are emitted **per signal** (one statement → multiple rules), antecedents
    often empty. Methodology for `.3`: take temporal-bearing statements + their real
    `statement_id`s from the APB EvidenceIR `extracted_statements` (e.g. `statement_0201`
    "PSEL … means PADDR, PWRITE, PWDATA must be valid"; `statement_0202` "PADDR, PWDATA …
    must be stable until the transfer completes"), judge the CORRECT rule(s) from the prose
    in this vocabulary (TP where the parser agrees, FN for a clear miss, and the wrong rule
    becomes an FP against the correct gold), and add ≥2 negatives (no temporal content).

- ID: `TEMPORAL-RULE-EVAL.4`
  Status: `done`
  Goal: runner path — build temporal rules from a source EvidenceIR copy (no corpus
    mutation; dependency-injected predictions) and score vs gold; integrate into the
    `eval-extraction` report (or a sibling surface); verify on a real EvidenceIR; book note;
    close per BOOK-METHOD-DOC.
  Acceptance: report shows temporal_rule P/R/F1; verified on real data; mdBook green; CLOSED.
  Verification: passed (`2026-06-02`) — wired the deterministic-producer runner in
    `commands/eval_extraction.rs`: new `TaskRecords::TemporalRules`, a `build_predictions`
    arm (→ `index_temporal_rule_predictions`), and the `EvalTask::TemporalRule` arm of
    `extract_on_copy` now **builds the SemanticIR** (`SemanticIr::build`) from the TEMP COPY
    of the doc's EvidenceIR — artifacts confined to the temp dir, corpus untouched, no
    LLM/provider — and returns `semantic.temporal_rules`. The generic `format_report` renders
    the new task automatically. **Verified end-to-end on the real APB EvidenceIR**:
    `eval-extraction seed_apb_temporal.json` → `temporal_rule P=0.400 R=0.667 F1=0.500
    (tp=2 fp=3 fn=1; gold=3 over 6 statements)` — matching the seed design exactly (2 clean
    TPs; the 2 degenerate-header rules + the PREADY-only PBUSER rule as the 3 FPs; the
    faithful 3-condition PBUSER gold as the 1 FN; the licence-notice negative correctly
    scored as nothing). **The eval surfaces a real finding** (antecedent under-capture +
    degenerate self-referential header rules) — the measure→catch→fix loop validated on live
    data, as `CONSTRAINT-SUBJECT-PRECISION` was. +1 runner test
    (`build_predictions_indexes_temporal_rules`, also asserting clock_signal/provenance are
    key-excluded). User-friendly book subsection "Temporal rules — the third measured surface"
    added to `quality/extraction-eval.md`. Full `scripts/run_ci.sh` GREEN (1215→1216).
    **Zero extraction-behavior change** (pure measurement path). Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TEMPORAL-RULE-EVAL.1` | `done` | owned + designed (this file) |
| 2 | `TEMPORAL-RULE-EVAL.2` | `done` | scorer types + canonical keys + tests (CI green, 1214) |
| 3 | `TEMPORAL-RULE-EVAL.3` | `done` | hand-labeled temporal gold seed (6 items, CI green 1215) |
| 4 | `TEMPORAL-RULE-EVAL.4` | `done` | runner + report + book + close (live APB P=0.400/R=0.667; CI green 1216) |

**Tree CLOSED `2026-06-02`.** Supervised temporal-rule precision/recall/F1 is live end-to-end
(`eval-extraction <temporal seed>` → `temporal_rule P=0.400 R=0.667 F1=0.500`), reusing the
`eval.rs` closed-world scorer with a provenance-free semantic key, producer = the
deterministic temporal parser. The first run already surfaced a real finding (antecedent
under-capture + degenerate self-referential header rules) — a candidate fix-tree, the
measure→catch→fix loop continuing past `CONSTRAINT-SUBJECT-PRECISION`.

## Decisions

- `2026-06-02`: reuse `eval.rs`'s closed-world scorer + canonical-key pattern verbatim
  (do not re-derive); the only new machinery is the temporal canonical key.
- `2026-06-02`: key on **semantic content, provenance-free**, with sorted antecedents/
  consequents — a rule's identity is its logical content, not emission order or its id.
- `2026-06-02`: score the **deterministic** temporal parser first; LLM-temporal scoring +
  capture–recapture recall are deferred (Open Questions) to keep this tree bounded.

## Open Questions

- Does any LLM pass (`nlp-enrich`/VLM enrich) contribute `TemporalRuleRecord`s that should be
  scored as a separate tier? If so, a later leaf adds a tier split (and a
  `FactKind::TemporalRule` capture–recapture gauge, mirroring
  `COMPLETENESS-RECALL-RELATIONS`).
- Should `cycle_window` mismatch be a partial-match (MUC-style) rather than a hard miss?
  (`extraction-evaluation.md` Tier-2 backlog item — likely a separate scorer upgrade.)

## Blockers

- None for `.1` (docs-only). `.2`+ trigger a full recompile (`target/` was `cargo clean`ed).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-02` | `.1` | design fixed against real `eval.rs` + `TemporalRuleRecord`/`TemporalPredicateRecord` shapes; semantic provenance-free canonical key; producer = deterministic temporal parser; LLM-tier + capture–recapture deferred; docs-only; registered | `passed` |
| `2026-06-02` | `.2` | `EvalTask::TemporalRule` + `GoldFact::TemporalRule` + `temporal_predicate_key`/`temporal_rule_key`/`temporal_rule_record_key` + `index_temporal_rule_predictions` in `eval.rs`; `score_dataset` unchanged (already generic); 3 unit tests (order/case-insensitive key match, window/edge discrimination, closed-world scoring); runner kept compiling (LLM path errors for temporal → `.4`; test closure `unreachable!`); zero extraction-behavior change; full CI GREEN (1211→1214) | `passed` |
| `2026-06-02` | `.3` | `seed_apb_temporal.json` (6 APB `temporal_rule` items, real statement_ids, labels judged from prose): 2 clean TPs, 1 faithful antecedent-under-capture (FN/FP) case, 3 negatives (2 degenerate-header FPs + 1 licence-notice abstention); loader test `committed_temporal_seed_loads_and_validates`; README updated with the temporal gold schema; full CI GREEN (1214→1215) | `passed` |
| `2026-06-02` | `.4` | deterministic-producer runner (`TaskRecords::TemporalRules`; `extract_on_copy` builds the SemanticIR from the temp EvidenceIR copy → `temporal_rules`; corpus untouched); generic report renders it; **live APB run** `temporal_rule P=0.400 R=0.667 F1=0.500 (tp=2 fp=3 fn=1)` matches the seed design exactly + surfaces a real finding; +1 runner test; user-friendly book subsection in `quality/extraction-eval.md`; full CI GREEN (1215→1216); zero extraction-behavior change; **tree CLOSED** | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TEMPORAL-RULE-EVAL.1` | `TEMPORAL-RULE-EVAL.1 — own + design supervised temporal-rule eval (first Tier-1 grounding-backlog item)` (`542bfdf3`) | docs-only |
| `TEMPORAL-RULE-EVAL.2` | `TEMPORAL-RULE-EVAL.2 — temporal-rule eval scorer types + provenance-free canonical key + tests` (`c2e1b05b`) | code; +3 tests; CI green 1214; zero behavior change |
| `TEMPORAL-RULE-EVAL.3` | `TEMPORAL-RULE-EVAL.3 — hand-labeled APB temporal gold seed (6 items; TP/FN/FP + negatives)` (`9699abf0`) | test_data + loader test + README; CI green 1215 |
| `TEMPORAL-RULE-EVAL.4` | `TEMPORAL-RULE-EVAL.4 — deterministic-producer runner + live APB score + book; close tree` | code + book; live P=0.400/R=0.667; CI green 1216; tree CLOSED |

## Changelog

- `2026-06-02`: Created — own + design the supervised temporal-rule evaluation surface, the
  first Tier-1 item promoted from the `LITERATURE-GROUNDING` reach-full-potential backlog.
  Extends the closed `LLM-EXTRACTION-EVAL` measurement loop to SpecForge's third extraction
  surface (temporal rules), reusing the `eval.rs` closed-world scorer with a new
  semantic, provenance-free canonical key.
- `2026-06-02`: `.2` done — implemented the scorer types + canonical keys + tests in
  `eval.rs` (`EvalTask::TemporalRule`, `GoldFact::TemporalRule`, `temporal_predicate_key`,
  `temporal_rule_key`, `temporal_rule_record_key`, `index_temporal_rule_predictions`); the
  closed-world `score_dataset` needed no change (already generic). 3 new unit tests; runner
  kept compiling honestly (LLM path errors for temporal rules → `.4`). Zero
  extraction-behavior change; full CI green (1211→1214). Frontier → `.3` (hand-labeled APB
  temporal gold seed).
- `2026-06-02`: `.3` done — wrote `seed_apb_temporal.json` (6 APB `temporal_rule` items,
  real statement_ids, labels judged independently from prose using the recorded IR
  vocabulary): 2 clean TPs, 1 faithful antecedent-under-capture (FN/FP) case, 3 negatives
  (2 degenerate-header FPs + 1 licence-notice abstention). New loader test + README schema
  doc. Full CI green (1214→1215). Frontier → `.4` (deterministic-producer runner builds the
  SemanticIR from a temp EvidenceIR copy → scores `temporal_rules` vs this seed → report +
  book + close).
- `2026-06-02`: **Tree CLOSED.** `.4` done — wired the deterministic-producer runner
  (`TaskRecords::TemporalRules`; `extract_on_copy` builds the SemanticIR from the temp
  EvidenceIR copy and reads `temporal_rules`, corpus untouched, no LLM), verified
  end-to-end on the real APB EvidenceIR (`temporal_rule P=0.400 R=0.667 F1=0.500`,
  tp=2/fp=3/fn=1 — matches the seed design exactly and surfaces a real
  antecedent-under-capture + degenerate-header finding → a candidate fix-tree), +1 runner
  test, user-friendly book subsection in `quality/extraction-eval.md`. Full CI green
  (1215→1216); zero extraction-behavior change. The whole `LITERATURE-GROUNDING` Tier-1
  item #2 (per-relation/temporal eval) is delivered for the temporal surface.
