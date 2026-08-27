---
id: required-residual-actionability-denominator
title: Residual actionability counts required residuals, not declared residual queries
answers:
  - "why is residual actionability only 4 of 16 in the current reviewed result"
  - "why did the residual actionability denominator change from 24 to 16"
  - "what does the residual actionability denominator actually count"
  - "why can a canonical cell never satisfy its residual actionability observations"
  - "which reviewed cells still need a typed actionable residual"
  - "what does SPEC-TO-INTENT-ALIGNMENT.8 repair"
  - "which residual family does SPEC-TO-INTENT-ALIGNMENT.8c implement first"
  - "which production carrier already emits typed actionable residuals"
  - "why does cargo test -p specforge --lib ir::source_to_intent_eval run zero tests"
  - "which crate owns the source-to-intent vertical evaluator tests"
  - "how does SpecForge stop a published gap reproduction from silently running no test"
  - "where is the required-residual rule frozen"
date: 2026-08-27
status: current
tags: [spec-to-intent-alignment, evaluation, residuals, stage-conservation, measurement-integrity]
evidence: crates/specforge/src/ir/source_to_intent_eval.rs; crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; crates/specforge/src/test_support/trajectory_snapshot.rs; crates/specforge-core/src/ir/source.rs; docs/tasks/spec-to-intent-alignment/residual-actionability.md
reverify: "cargo test --offline -p specforge-conformance --lib ir::source_to_intent_eval && python3 -c \"import json; d=json.load(open('crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json')); print(d['global']['residual_actionability'])\""
---

`summarize_global` in `crates/specforge/src/ir/source_to_intent_eval.rs` used to add `residual_total += 2` for
every cell that merely *declared* residual queries — one observation for SemanticIR and one for IntentIR —
without asking whether the review required a residual there. The published `.7c.ii` result therefore reported
4/24 and the controller repeated it as "20 of 24 required residual observations". Twenty was never the number of
production defects. `SPEC-TO-INTENT-ALIGNMENT.8b` replaced that denominator with
`required_residual_observations`, and the published ratio is now 4/16 over an affected population of twelve.

The 24 declared observations decompose exactly three ways. Four are actionable: both OpenCAPI `analog_channel_loss`
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
to conservation. The denominator, not the pipeline, was what made those eight observations unmeetable.

The corrected rule counts one observation per *required* residual: one per promoted stage for a residual or
non-applicable cell, and one per reviewed canonical key a promoted stage fails to promote. It is fail-closed in
the direction that matters — a missing canonical key always adds a required observation and is met only by an
exact, provenanced, actionable residual for that same key — and a stage whose residual set duplicates a promoted
key credits nothing. Applied to the frozen `.4b` first result the denominator grows from 24 to 82, because a
canonical cell losing fifteen keys owes fifteen explanations rather than one.

Production owns exactly one typed residual carrier today. `TimingIntentDisposition::NonApplicable` in
`crates/specforge-core/src/ir/source.rs` keeps `quantity_domain`, `reason`, `first_failing_stage`, and `replay`
on a scalar timing row, which is why the two OpenCAPI cells pass. No carrier exists for a captured prose
statement, table region, or visual region that reaches no canonical `IntentIR` surface.

[[source-to-intent-vertical-evaluator]] owns the oracle and [[spec-to-intent-category-contract]] owns the
acceptance floor, whose denominator is "all residuals" rather than all declared residual queries.
`SPEC-TO-INTENT-ALIGNMENT.8b` shipped that correction; `.8c` implements the first bounded production carrier for
`static_component_topology`, the only single family whose closure can move a reviewed category to `supported`.

One measurement-integrity defect travelled with this gap and is now closed. The controller composed its
reproduction command as `cargo test -p specforge --lib ir::source_to_intent_eval`, but
`ir::source_to_intent_eval` lives in `specforge-conformance` and is only re-exported through the `specforge::ir`
compatibility facade, so that command matched zero tests and exited zero;
`cargo test -p specforge-conformance --lib ir::source_to_intent_eval` runs the real 14. `.8a` repaired the whole
class structurally rather than the literal: `crates/specforge/src/test_support/trajectory_snapshot.rs` derives
the conformance-owned test roots from that crate's own `pub mod` declarations, resolves the owning package for a
composed filter, and fails controller-input composition when a published reproduction is not an executable
`cargo test -p <package> --lib <filter>` form for its owning package.

`.8a` also freezes the rule itself in `doctrine/spec_to_intent/residual_actionability_contract.json`, whose
checker executes it over 18 closed cases and rejects 21 mutations, including relabeling a missing canonical fact
as residual success.
