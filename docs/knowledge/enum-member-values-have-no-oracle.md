---
id: enum-member-values-have-no-oracle
title: Neither standing oracle can see an enum member value — 0 of 156 kg-bench fixtures mention one and eval-extraction has 9 task kinds without one — which is why every `.5` defect was found by a census, never by a test
answers:
  - "does kg-bench test enum member values"
  - "does WIRE-BASED-100 test enum member values"
  - "why did the enum defects go undetected for so long"
  - "which fact kinds does eval-extraction score"
  - "is a green WIRE-BASED-100 evidence that enums are correct"
  - "what oracle should an enum repair be verified against"
  - "why is kg-bench 156/156 not enough for an enum change"
  - "how do I verify a change to enum member values"
  - "which enums publish one code on two members"
  - "do the colliding enums reach IntentIR"
  - "do the colliding enums reach the isf file"
  - "why does HSIZE publish BYTE and HALFWORD both as 0"
  - "why does PPROT publish members named 0 and 1"
date: 2026-09-22
status: current
tags: [kg-isf-completeness, verification, oracles, wire-based-100, kg-bench, enums, claim-verification]
evidence: crates/specforge/test_data/kg_quality/*/fixture.json (156 fixtures); crates/specforge/test_data/llm_eval/*.json (93 labelled items, 9 task kinds); scripts/adjudicate_enum_member_name_column.py --collisions; docs/research/enum-code-collisions.md
reverify: "grep -rli 'enum\\|symbol_definition' crates/specforge/test_data/kg_quality/*/fixture.json | wc -l — expect 1 of 157 since .5.xiv.a (enum_code_collision_holdout_gold); it was 0 of 156 when measured. Then enumerate the `task` field across crates/specforge/test_data/llm_eval/*.json — expect exactly signal_constraint, actor_signal_relation, temporal_rule, declared_signal, register_field, serial_frame_field, protocol_operation, protocol_state, interface_edge_timing, and no enum task. python3 scripts/adjudicate_enum_member_name_column.py --collisions — expect 266 colliding enums, 9 replayable, 8 of them reaching IntentIR symbol_definitions."
---

## Both headline gates are structurally blind to enum member values

| oracle | scope | sees an enum member value? |
| --- | --- | --- |
| `kg-bench` | 156 fixtures (measured `2026-09-22`) | **no** — 0 mentioned `enum` or `symbol_definition` |
| WIRE-BASED-100 / `eval-extraction` | 93 labelled items | **no** — 9 task kinds, none of them an enum |

**CLOSED `2026-09-22` by `.5.xiv.a`, for one fact kind.** `CanonicalStageExpectations` now carries
`symbol_members_include` (symbol / member / value) and `symbol_names_exclude` (the hold-out side — a
passing state that is an ABSENCE, which no `_include` can express), and
`enum_code_collision_holdout_gold` is the first fixture in the repository to assert an enum member
value. `kg-bench` is **157/157**. **The schema gap was the real one**: no fixture could have asserted
an enum member value even if an author had wanted to, which is why "nobody wrote one" understates it.
`eval-extraction` is unchanged and still has no enum task, so WIRE-BASED-100 remains blind.

`eval-extraction`'s task kinds are `signal_constraint`, `actor_signal_relation`, `temporal_rule`,
`declared_signal`, `register_field`, `serial_frame_field`, `protocol_operation`, `protocol_state`,
`interface_edge_timing`.

**So `kg-bench 156/156` and `WIRE-BASED-100 = 1.000` are necessary and NOT sufficient for any change to
enum identity, naming or values.** Citing them as the acceptance bar for such a change ships against a
bar that cannot fail — `.5.viii`'s *"a count cannot verify a value"* one level up. It is also the whole
explanation of the `.5` programme's history: mega-conflation, positional values, the quoted radix, the
unquoted radix, the naming column and the code collisions were each found by a CENSUS, because no test
was watching. [[qualified-role-header-proves-no-role]] recorded the same for signal tables.

**An enum change needs a per-member diff over the replayable chains, plus a fixture that asserts a
member value** — the missing red case.

## The live defect this was measured for

266 enums publish one code on two members; **9 are replayable and 8 cross into IntentIR
`symbol_definitions`, which `README.md` declares the product boundary**. None reaches `.isf`: AXI, AHB
and APB are all `lowering_status: blocked` for reasons unrelated to enums.

```text
Enum PPROT 0 = 0.   Enum PPROT 0 = 1.   Enum PPROT 1 = 0.   Enum PPROT 1 = 1.
Enum HSIZE BYTE = 0.  HALFWORD = 0.  WORD = 0.  DOUBLEWORD = 0.
```

They survive `.5.vii.a`'s fallback removal **because the cell does parse** — the value is
document-stated and the COLUMN is wrong. Three causes, not one: a split bit field (`HSIZE` under
`HSIZE[2] | HSIZE[1] | HSIZE[0] | …`), one bit of a wider field with the rest absent (`PPROT[1]`), a
genuine multi-key table (`AWSNOOP` = `(AWSNOOP, AWDOMAIN) -> opcode`), and cross-table conflation
(`AWATOP`). The unifying statement: **the value column does not functionally determine the member.**

Related: [[enum-member-name-column-needs-adjudication-not-a-score]],
[[minted-enum-values-are-mostly-row-indices]], [[decimal-before-binary-needs-a-notational-witness]].
