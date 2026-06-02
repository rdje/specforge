# TEMPORAL-RULE-EVAL: supervised precision/recall/F1 for mined temporal rules

## Metadata

- Tree ID: `TEMPORAL-RULE-EVAL`
- Status: `active` (`.1` design done; `.2`+ = code)
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
  Status: `pending`
  Goal: `EvalTask::TemporalRule` + `GoldFact::TemporalRule` (typed predicate sub-schema) +
    `temporal_rule_record_key` + a `temporal_predicate_key` helper + prediction indexing;
    extend `GoldFact::task`/`canonical_key` and the report task list. Pure types + keys, no
    producer change.
  Acceptance: unit tests (gold↔record agreement; antecedent/consequent order-insensitivity;
    negative item; cycle-window in/equality); `scripts/run_ci.sh` green.

- ID: `TEMPORAL-RULE-EVAL.3`
  Status: `pending`
  Goal: hand-labeled APB temporal gold seed (≥6 items, ≥2 negatives), gold drafted
    independently from prose; extend `load_eval_dataset`/validation for the new variant.
  Acceptance: dataset loads + validates; each gold item justifiable from its statement.

- ID: `TEMPORAL-RULE-EVAL.4`
  Status: `pending`
  Goal: runner path — build temporal rules from a source EvidenceIR copy (no corpus
    mutation; dependency-injected predictions) and score vs gold; integrate into the
    `eval-extraction` report (or a sibling surface); verify on a real EvidenceIR; book note;
    close per BOOK-METHOD-DOC.
  Acceptance: report shows temporal_rule P/R/F1; verified on real data; mdBook green; CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TEMPORAL-RULE-EVAL.1` | `done` | owned + designed (this file) |
| 2 | `TEMPORAL-RULE-EVAL.2` | `pending` | scorer types + canonical keys + tests (needs recompile) |
| 3 | `TEMPORAL-RULE-EVAL.3` | `pending` | hand-labeled temporal gold seed |
| 4 | `TEMPORAL-RULE-EVAL.4` | `pending` | runner + report + book + close |

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TEMPORAL-RULE-EVAL.1` | `TEMPORAL-RULE-EVAL.1 — own + design supervised temporal-rule eval (first Tier-1 grounding-backlog item)` | docs-only |

## Changelog

- `2026-06-02`: Created — own + design the supervised temporal-rule evaluation surface, the
  first Tier-1 item promoted from the `LITERATURE-GROUNDING` reach-full-potential backlog.
  Extends the closed `LLM-EXTRACTION-EVAL` measurement loop to SpecForge's third extraction
  surface (temporal rules), reusing the `eval.rs` closed-world scorer with a new
  semantic, provenance-free canonical key.
