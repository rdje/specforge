---
id: an-uncited-obligation-is-a-candidate-miss
title: validate's completeness summary counts as candidate misses the normative statements AND the statements classed a signal-value constraint or a conditional rule that no signal constraint or conditional rule cites; before 2026-09-26 the second group (130 on the 27 live chains) was counted nowhere, and the normative finding listed every normative statement, cited or not (EXTRACTION-REPRESENTATION-GAPS.34)
answers:
  - "what does completeness_candidate_misses count"
  - "what is classified_obligation_residuals / evidence_classified_obligation_residuals_remaining"
  - "is a refused constraint reading still visible somewhere"
  - "why did candidate_misses rise on 2026-09-26"
  - "which statements does evidence_normative_residuals_remaining list"
  - "which statement classes are obligations for the completeness count (timing_constraint and derived_rule are not)"
date: 2026-09-26
tags: [validate, completeness, evidence, residuals, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/completeness.rs (uncaptured_normative_statement_ids, uncaptured_classified_obligation_statement_ids); crates/specforge/src/commands/validate.rs (evidence_captured_statement_ids, evidence_normative_residual_statement_ids, evidence_classified_obligation_residual_statement_ids); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.34)
reverify: "cargo test --offline -p specforge --lib -- an_uncited_classified_obligation_is_a_candidate_miss validate_evidence_ir_completeness_summary_aggregates_components; cargo test --offline -p specforge-conformance --lib -- an_uncited_classified_obligation_is_a_residual. jq '[.validation_reports[-1].metrics[] | select(.name == \"classified_obligation_residuals\")][0].value' generated/evidence_ir/wbspec_b4_wishbone_b4_specification/evidence_ir.json -> \"15\" on 2026-09-26."
---

**Measured `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.34`).** A statement is *cited* when a signal constraint or a
conditional rule names it. `validate` counted only uncited statements of class `normative_statement` (3,008 on the 27
live chains). A statement the classifier typed as a `signal_value_constraint` or a `conditional_rule`, for which no
reader minted a record, was counted nowhere. That covers every refusal on the passive constraint path, which reads
only that class: *a kind the clause does not state* (EXTRACTION-QUALITY-GAUGE.3k.2a), a relation, and
`.17`'s event-bounded level. Their comments claimed the statement *stays counted as an uncaptured normative
statement*.

Now `classified_obligation_residuals` counts them and joins `completeness_candidate_misses`, and
`evidence_classified_obligation_residuals_remaining` lists them: 130 on the live chains (96 constraint-classed,
34 rule-classed; AXI 30, USB 3.2 22, ADIv6 17, Wishbone 15). The normative finding now lists exactly the statements
it counts. It listed every normative statement before: 3,026 ids for a count of 3,008.

Not counted: `timing_constraint` and `derived_rule` are topic classes, not obligations (*… typically has a data width
of 32 bits*). A statement cited only by another family, such as the 93 normative statements protocol states cite,
stays counted, because those records carry no typed obligation (`.12`). No normative count fell. Every persisted
evidence report gained the metric; semantic, intent and adapter artifacts moved only in their proof and validation
records, and no `.isf` changed.
`[[an-invariant-holds-at-every-cycle]]` · `[[a-polarity-rule-reaches-its-stated-scope]]`.
