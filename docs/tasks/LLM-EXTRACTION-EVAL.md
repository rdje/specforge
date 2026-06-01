# LLM-EXTRACTION-EVAL: a labeled precision/recall eval set for the LLM extraction passes

## Metadata

- Tree ID: `LLM-EXTRACTION-EVAL`
- Status: `active`
- Roadmap lane: `R15d`/`R16` (eval / extraction quality)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: user wants to compare VLM/LLM models (qwen2.5vl:7b vs qwen3-vl:8b,
  Instruct vs Thinking, 8B vs 30B) **rigorously**. Today there is NO labeled gold
  for the LLM passes — `kg-bench`'s 151 fixtures test the deterministic pipeline
  (provider: none), and the capture–recapture recall gauge is unsupervised. This
  tree builds the missing piece: a labeled precision/recall eval for the LLM tasks.

## Goal

A small, versioned, **labeled** eval set + a scorer + a runner that produces
per-task **precision / recall / F1** for the LLM extraction passes against gold —
so any model swap (or prompt change) is *measured*, not eyeballed. Make it
re-runnable per model so model A/Bs are a one-flag exercise.

## Scope (v1)

The three **text** tasks (the bulk of the calls; tractable to label + score):
- `nlp-enrich` → `SignalConstraintRecord` (subject signal + constraint kind + negation)
- `signal-resolve` → `ActorSignalRelation` (actor / relation / signal)
- `extract-contracts` → `ActorContract` (actor / obligation / primary signal)

**Out of v1 (noted follow-on):** the VLM `enrich` task (image→observation) — images
are harder to label and score rigorously; a separate leaf once the text harness is
proven.

## Design

### Eval item (one labeled statement)

```
{ "task": "signal-resolve",
  "doc_key": "ihi0024_e_..._apb...",
  "statement_id": "statement_0184",
  "input_text": "The Completer drives PREADY ...",
  "grounding": ["PREADY", "PWDATA", ...],          // declared-signal context
  "gold": [ {"actor":"Completer","relation":"drives","signal":"PREADY"} ],
  "label_status": "agent_drafted",                 // pending human review
  "label_note": "clear: 'Completer drives PREADY'" }
```

`gold` is the set of typed outputs a correct extraction MUST produce for that
statement (possibly empty — a negative item, where the correct output is nothing).

### Scoring (pure, deterministic; the rigorous core)

Per task, **closed-world over the labeled statements only**: for each labeled
statement, compare the model's outputs *for that statement* (matched by the typed
record's statement-provenance) to `gold`, using a task canonical key:
- SignalConstraint: `(subject_signal↑, constraint_kind, negated)`
- ActorSignalRelation: `(actor↑, relation, signal↑)` — reuses the recall-gauge key
- ActorContract: `(actor↑, obligation_kind, primary_signal↑)`

Aggregate TP (in both) / FP (model-only) / FN (gold-only) across labeled statements
→ `precision = TP/(TP+FP)`, `recall = TP/(TP+FN)`, `F1`. Outputs on *unlabeled*
statements are ignored (we did not label them, so they are neither right nor wrong).

### Runner (faithful — evaluates the real production path)

For each distinct `doc_key` in the dataset: run the task's actual command
(`nlp-enrich` / `signal-resolve` / `extract-contracts`) with `--provider/--model`
on that doc's EvidenceIR; read the produced typed records **with their statement
provenance**; match to the labeled statements; score. `--provider skip` no-ops the
LLM (lets the runner + scorer be tested without a server). This evaluates the *real*
prompt + parsing + grounding + fails-closed behavior — which is exactly what changes
between models.

### Honest labels

Gold is **agent-drafted from real corpus statements** (AMBA core + i2c) and marked
`label_status: agent_drafted` / `human_reviewed`. The eval is only as good as its
labels; the set is versioned and review-able. Never reported as human-validated
until a human reviews it.

## Non-Goals

- NOT the VLM `enrich` eval (v1) — follow-on leaf.
- NOT replacing `kg-bench` (deterministic) or the recall gauge (unsupervised) — this
  complements them with supervised precision/recall for the LLM passes.
- NOT auto-pulling/benchmarking qwen3-vl — that A/B is a trivial follow-on once this
  exists (run the runner twice with different `--model`).

## Acceptance Criteria

- Labeled dataset format + loader; pure scorer (precision/recall/F1, task canonical
  keys) with unit tests; a seed labeled set (~24 items, ≥8/text-task) drafted from
  real corpus statements + flagged for review; a provider-gated runner that scores
  the real command outputs; a baseline run recorded (qwen2.5vl:7b). fmt + clippy +
  full CI green; book note; tree CLOSED. The qwen3-vl A/B is then one flag away.

## Task Tree

- ID: `LLM-EXTRACTION-EVAL`
  Status: `active`
  Children: `.1`–`.5`

- ID: `LLM-EXTRACTION-EVAL.1`
  Status: `done`
  Goal: own + design (this doc) — item format, scoring semantics, faithful runner,
    honest labeling, v1 scope (3 text tasks). Docs-only.
  Verification: passed (`2026-06-01`) — design fixed; scoped to the 3 text tasks;
    faithful-runner + closed-world-on-labeled scoring decided; registered.
  Commit: `see Commit Log`

- ID: `LLM-EXTRACTION-EVAL.2`
  Status: `done`
  Goal: implement the pure scorer + dataset format/loader (no LLM): canonical keys
    per task, TP/FP/FN → precision/recall/F1, JSON item schema + loader. Unit tests.
  Acceptance: scorer + loader + tests; CI green.
  Verification: passed (`2026-06-01`) — new `crates/specforge/src/eval.rs` (pub module):
    `EvalTask` (SignalConstraint, ActorSignalRelation), `GoldFact` (tagged enum
    Constraint|Relation) + `canonical_key` (normalized: signals/actors uppercased,
    kind/relation lowercased, negation in the key), `constraint_kind_str`/
    `relation_kind_str`, record-key fns (`signal_constraint_record_key`,
    `actor_signal_relation_record_key`) that MATCH the gold keys, `EvalItem` + serde +
    `validate` (gold fact ↔ task), `PredictedKeys = BTreeMap<(EvalTask, statement_id),
    Set<key>>` + `index_constraint_predictions`/`index_relation_predictions` (attribute
    each record's key to its supporting statements), `Scorecard` (tp/fp/fn →
    precision/recall/F1, computed from counts), `score_dataset` (closed-world over
    labeled statements), `load_eval_dataset` (dir of `*.json`). Provider-free.
    6 unit tests (key match incl. MustBeValue; negation/direction discrimination;
    P/R/F1 closed-world; negative-item FP; task/gold validate; loader round-trip).
    fmt + clippy clean; full CI green.
  Commit: `see Commit Log`

- ID: `LLM-EXTRACTION-EVAL.3`
  Status: `pending`
  Goal: seed labeled dataset (~24 items, ≥8/text-task) drafted from real AMBA/i2c
    statements; each `agent_drafted` + note; include negatives.
  Acceptance: dataset under `test_data/llm_eval/`; loads + validates clean.

- ID: `LLM-EXTRACTION-EVAL.4`
  Status: `pending`
  Goal: provider-gated runner (`eval-extraction` command): run the task command with
    `--model`, read typed records w/ statement provenance, score, report per-task +
    overall P/R/F1. `--provider skip` testable path.
  Acceptance: runner scores the seed set; skip-mode wired test; CI green.

- ID: `LLM-EXTRACTION-EVAL.5`
  Status: `pending`
  Goal: baseline run with qwen2.5vl:7b (server-gated; record honestly if queued);
    book note; close. qwen3-vl A/B noted as the immediate follow-on.
  Acceptance: baseline numbers recorded; book; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `.1` | `done` | owned + design |
| 2 | `.2` | `done` | pure scorer + dataset format/loader (`eval.rs`); 6 unit tests; CI green |
| 3 | `.3` | `pending` | seed labeled data from real corpus — next |
| 4 | `.4` | `pending` | provider-gated runner over the real command path |
| 5 | `.5` | `pending` | baseline + book + close |

## Decisions

- `2026-06-01`: v1 = 3 text tasks; VLM enrich eval deferred (labeling/scoring images
  rigorously is its own problem).
- `2026-06-01`: evaluate the **real command path** (not a re-implemented prompt), so
  the eval measures what actually changes between models.
- `2026-06-01`: **closed-world on the labeled subset** — score only labeled
  statements; ignore model outputs on unlabeled statements (avoids penalizing
  un-labeled-but-correct extractions).
- `2026-06-01`: labels are agent-drafted + review-flagged; never reported as
  human-validated until reviewed.

## Open Questions

- Exact `ActorContract` canonical key (obligation taxonomy) — settle in `.2` against
  the real `ActorContract` schema.

## Blockers

- `.5` baseline is server-gated (single-model Ollama can be saturated) — recorded
  honestly + re-runnable, as with prior LLM passes.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | design fixed (format/scoring/runner/labels/scope); registered; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `LLM-EXTRACTION-EVAL.1` | `LLM-EXTRACTION-EVAL.1 — own + design labeled precision/recall eval for the LLM passes` | docs-only |
| `LLM-EXTRACTION-EVAL.2` | `LLM-EXTRACTION-EVAL.2 — pure scorer + dataset format/loader (eval.rs); 6 tests` | provider-free core |

## Changelog

- `2026-06-01`: Created — labeled precision/recall eval for the LLM extraction
  passes (the missing measurement piece for rigorous model A/Bs). Frontier → `.2`.
