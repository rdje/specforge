---
id: required-residual-actionability-denominator
title: Residual actionability counts declared residual queries, so eight of its 24 observations cannot be satisfied
answers:
  - "why is residual actionability only 4 of 24 in the current reviewed result"
  - "what does the residual actionability denominator actually count"
  - "why can a canonical cell never satisfy its residual actionability observations"
  - "which reviewed cells still need a typed actionable residual"
  - "what does SPEC-TO-INTENT-ALIGNMENT.8 repair"
  - "which residual family does SPEC-TO-INTENT-ALIGNMENT.8c implement first"
  - "which production carrier already emits typed actionable residuals"
  - "why does cargo test -p specforge --lib ir::source_to_intent_eval run zero tests"
  - "which crate owns the source-to-intent vertical evaluator tests"
date: 2026-08-27
status: current
tags: [spec-to-intent-alignment, evaluation, residuals, stage-conservation, measurement-integrity]
evidence: crates/specforge/src/ir/source_to_intent_eval.rs; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/src/test_support/trajectory_snapshot.rs; crates/specforge-core/src/ir/source.rs; docs/tasks/spec-to-intent-alignment/residual-actionability.md
reverify: "cargo test --offline -p specforge-conformance --lib ir::source_to_intent_eval && python3 -c \"import json; d=json.load(open('crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json')); print(d['global']['residual_actionability'])\""
---

`summarize_global` in `crates/specforge/src/ir/source_to_intent_eval.rs` adds `residual_total += 2` for every
cell that *declares* residual queries — one observation for SemanticIR and one for IntentIR — and never asks
whether the review actually requires a residual there. The published `.7c.ii` result therefore reports 4/24,
and the trajectory controller repeats it as "20 of 24 required residual observations". Twenty is not the number
of production defects.

The 24 observations decompose exactly three ways. Four are actionable: both OpenCAPI `analog_channel_loss`
cells emit typed residuals. Eight belong to four `canonical` cells — APB `setup_signal_state`, I2S
`receiver_timing`, GIC-400 `register_summary`, and Arm Debug `debug_register_summary` — whose review declares
the same keys as canonical gold and as residual gold, so the residual query is only the conservation fallback
that explains a canonical key if it is ever lost. All four are exact at EvidenceIR, SemanticIR, and IntentIR
with zero false positives, false negatives, and unprovenanced records, so nothing is lost and no residual is
required; emitting one would assert that the same fact both reached and did not reach `IntentIR`. Twelve are
genuinely required and absent, across the six cells that are the complete current hard-failure set:
`informational_disclaimer` and `software_guidance` (prose), `table_of_contents` and `packed_page_table_entry`
(table), and `static_component_topology` in both platform documents (figure).

`boundary_scores` confirms the fallback reading: for a canonical cell, matched residual keys are only used to
explain canonical keys that are *missing* at a boundary, so a residual duplicating a promoted key adds nothing
to conservation. The denominator, not the pipeline, is what makes those eight observations unmeetable.

Production owns exactly one typed residual carrier today. `TimingIntentDisposition::NonApplicable` in
`crates/specforge-core/src/ir/source.rs` keeps `quantity_domain`, `reason`, `first_failing_stage`, and `replay`
on a scalar timing row, which is why the two OpenCAPI cells pass. No carrier exists for a captured prose
statement, table region, or visual region that reaches no canonical `IntentIR` surface.

[[source-to-intent-vertical-evaluator]] owns the oracle and [[spec-to-intent-category-contract]] owns the
acceptance floor, whose denominator is "all residuals" rather than all declared residual queries.
`SPEC-TO-INTENT-ALIGNMENT.8` therefore corrects the accounting fail-closed in `.8b` — a canonical key missing at
a stage still *adds* a required observation and fails it unless an exact actionable residual explains it — and
implements the first bounded production carrier for `static_component_topology` in `.8c`, the only single
family whose closure can move a reviewed category to `supported`.

One measurement-integrity defect travels with this gap. The controller composes its reproduction command as
`cargo test -p specforge --lib ir::source_to_intent_eval`, but `ir::source_to_intent_eval` lives in
`specforge-conformance` and is only re-exported through the `specforge::ir` compatibility facade. That command
matches zero tests and exits zero; `cargo test -p specforge-conformance --lib ir::source_to_intent_eval` runs
the real 14. The same wrong crate is composed for the `.7` gap in
`crates/specforge/src/test_support/trajectory_snapshot.rs`.
