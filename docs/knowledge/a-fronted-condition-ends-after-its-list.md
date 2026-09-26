---
id: a-fronted-condition-ends-after-its-list
title: A fronted condition (When/If/After …,) ends at the comma after any list it names, with or without an Oxford comma, only where the main clause keeps a subject before a modal in the same sentence; with every comma a list comma it ends at its verb, and with no verb at the first comma; conditional rules and constraints read it through one helper, fronted_condition_end; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.35)
answers:
  - "why did condrule_0152 read only AWSNOOP is not CMO"
  - "where does a fronted condition end when it contains a list"
  - "what is fronted_condition_end"
  - "why are AXI's AWSTASHNID AWSTASHNIDEN AWSTASHLPID AWSTASHLPIDEN LOW drives gone from manager.isf"
  - "why does split_conditional_sentence not cut at the first comma"
date: 2026-09-26
tags: [evidence, conditions, conditional-rules, constraints, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (fronted_condition_end, main_clause_of_fronted_conditional, split_conditional_sentence, extract_condition_clause); docs/book/src/pipeline/obligation-reading.md; docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.35, .10.g.iii, .4)
reverify: "cargo test --offline -p specforge-core --lib -- a_fronted_condition_keeps_every_member_of_its_list the_boundary_is_a_comma_that_is_not_part_of_a_list. jq -r '.conditional_rules[] | select(.rule_id == \"condrule_0152\") | .antecedent_text' generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json -> AWSNOOP is not CMO, WritePtlCMO or WriteFullCMO on 2026-09-26."
---

**Measured and landed `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.35`, found by `.10.g.iii`).** There were two readers
of a fronted condition, and both cut it too early:

- `split_conditional_sentence`, used for conditional rules, stopped at the **first** comma. AXI's *When AWSNOOP is not
  CMO, WritePtlCMO or WriteFullCMO, AWCMO must be 0b000* became `condrule_0152` with antecedent *AWSNOOP is not
  CMO*. A guard built from it would have exempted only `CMO`.
- `main_clause_of_fronted_conditional`, used for constraints, knew a list only by an Oxford comma. It read no condition
  at all when every comma was a list comma and the condition had no verb: *If present, AWSTASHNID, AWSTASHNIDEN,
  AWSTASHLPID, and AWSTASHLPIDEN must be LOW* lost *present*, and `manager.isf` drove the four LOW in every
  configuration.

`fronted_condition_end` now serves both readers:
- The documented Oxford rule is kept.
- A list without the Oxford comma closes at the comma after its coordinating member, but only when a subject is
  left before the modal. USB4's *…, a Connection Manager that supports … and also supports Gen T, shall read …*
  coordinates inside its subject, and keeps its first comma.
- The conditional-rule reader applies the list rule only with a *must*/*shall* in the same sentence; without one,
  it keeps the first comma, since the rest would be its main clause.
- A verbless all-list condition ends at the first comma.

On the 27 live chains, 16 conditional rules change, every one a fix (AXI 2, ADIv6 3, USB 3.2 7, USB4 Inter-Domain 4).
So do 8 AXI constraints, which regain *present*. `manager.isf` loses 18 unconditional `AWSTASH*` rules, which are
now listed under a presence condition (`.4`), and it stays strict-clean.
`[[one-record-per-obligation-clause]]` · `[[an-operation-kind-a-level-states-is-a-guard]]`.
