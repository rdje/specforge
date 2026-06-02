# LLM extraction eval — labeled dataset

Labeled gold for the supervised precision/recall/F1 evaluation of SpecForge's extraction
passes. Loaded by `crate::eval::load_eval_dataset`, which accepts either a single `.json`
array file (these seeds) or a directory of per-item files. Two seeds live here:

- `seed_apb.json` — `signal_constraint` + `actor_signal_relation` tasks (`LLM-EXTRACTION-EVAL`).
- `seed_apb_temporal.json` — the `temporal_rule` task (`TEMPORAL-RULE-EVAL`).

## Format

One item per labeled **statement** of a real corpus document:

```json
{
  "task": "signal_constraint" | "actor_signal_relation",
  "doc_key": "<EvidenceIR document key>",
  "statement_id": "<real statement id in that EvidenceIR>",
  "input_text": "<the statement prose, for human review>",
  "grounding": ["<declared signals to ground the prompt>"],
  "gold": [ <typed outputs a correct extraction MUST produce; [] = a negative item> ],
  "label_status": "agent_drafted" | "human_reviewed",
  "label_note": "<why this label>"
}
```

Gold facts: `{"fact":"constraint","subject_signal":..,"constraint_kind":"must_be_stable|must_be_low|must_be_value|..","negated":false,"target_value":null}`,
`{"fact":"relation","actor":..,"relation":"drives|reads","signal":..}`,
or `{"fact":"temporal_rule","edge":"rising|falling|unknown","antecedents":[..],"consequents":[..],"cycle_window":{"min_cycles":..,"max_cycles":..}}`
where each predicate is a `TemporalPredicateRecord` (`{"kind":"signal_value","signal_name":..,"value":..,"phase":"pre_tick|post_tick"}`,
`{"kind":"actor_drives_signal",..}`, `{"kind":"signal_stable",..}`, …). The temporal key is
**provenance-free** — clock edge + the *sorted* antecedent/consequent predicate keys +
cycle window — so antecedent/consequent order and signal casing do not matter.

## Labeling contract

- Gold is **independently drafted from the statement prose** — NOT copied from the
  current pattern/LLM output (that would be circular, and the pattern tier has known
  errors, e.g. a bogus `APB protocol reads PWDATA` edge that the gold deliberately omits).
- Scoring is **closed-world per labeled statement**: label *every* fact a statement
  should yield, so an extra model output on a labeled statement is a true false positive.
- `statement_id` must be a real id in the named document's `EvidenceIR` so the runner can
  match produced records by provenance.

## Status

`seed_apb.json` — **16 items, all `agent_drafted` (pending human review)**: 8
`signal_constraint` + 8 `actor_signal_relation`, drawn from AMBA APB (IHI0024_E),
including ≥2 negatives per task. A seed to be reviewed and extended (more docs, the
`extract_contracts` ActorContract task, and the VLM `enrich` task are deferred — see
`docs/tasks/LLM-EXTRACTION-EVAL.md`).

`seed_apb_temporal.json` — **6 items, all `agent_drafted`** for the `temporal_rule` task
(AMBA APB IHI0024_E; producer = the deterministic temporal parser at EvidenceIR→SemanticIR).
Two clean positives (single-condition `PNSE valid when PSEL asserted`; `Requester drives
PSTRB LOW`), one faithful-gold antecedent **under-capture** case (`PBUSER valid when PSEL,
PENABLE, and PREADY are asserted` — gold keeps all three preconditions; the parser is
expected to drop PSEL+PENABLE → a recall gap the seed surfaces), and three negatives (two
list-introducer headers whose self-referential rules are degenerate FPs; one front-matter
licence notice the extractor should ignore). See `docs/tasks/TEMPORAL-RULE-EVAL.md` (the
`.3` node records the producer-representation calibration). Scoring is wired in `.4`.
