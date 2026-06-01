# LLM extraction eval — labeled dataset

Labeled gold for the `LLM-EXTRACTION-EVAL` task-tree (the supervised precision/recall/F1
eval of the LLM extraction passes). Loaded by `crate::eval::load_eval_dataset`, which
accepts either a single `.json` array file (this seed) or a directory of per-item files.

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

Gold facts: `{"fact":"constraint","subject_signal":..,"constraint_kind":"must_be_stable|must_be_low|must_be_value|..","negated":false,"target_value":null}`
or `{"fact":"relation","actor":..,"relation":"drives|reads","signal":..}`.

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
