---
id: a-tie-off-is-not-a-drive
title: A level bound by *tied* in a clause with no condition is a tie-off (`SignalConstraintKind::TiedTo`) — the level an input is tied to when its driver omits the signal — and no stage lowers it as a drive; a *tied* level with a stated scope (*tied HIGH for the entire burst*) stays a hold; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.10.e)
answers:
  - "why does AXI manager.isf not drive AWSNOOP ARSNOOP ARLOCK AWLOCK WSTRB from the tied LOW sentences"
  - "what is SignalConstraintKind::TiedTo / tied_to"
  - "how is a tie-off told apart from a drive or a hold"
  - "where is a tie-off's configuration (the signal's own absence on the driver, its presence condition)"
  - "how will a tie-off reach .isf (FSMGen absent-input value, EXTRACTION-REPRESENTATION-GAPS.4)"
  - "what does LevelRefusal::TieOff mean in isf_obligations_not_lowered"
date: 2026-09-26
tags: [evidence, constraints, tie-off, isf, extraction-representation-gaps, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (type_tie_offs, tie_off_bindings, level_bindings_after); crates/specforge/src/ir/source.rs (SignalConstraintKind::TiedTo); crates/specforge/src/ir/semantic.rs (temporal_consequents_from_signal_constraint); crates/specforge/src/ir/intent.rs (ConstraintLevel::TieOff, signal_constraint_to_temporal_invariant_kind); crates/specforge/src/ir/isf_ir.rs (LevelRefusal::TieOff); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.10.e, .4)
reverify: "cargo test --offline -p specforge-core --lib -- a_tied_level_is_a_tie_off_unless_its_clause_states_a_scope a_tie_off_is_listed_not_driven a_tie_off_is_no_rules_consequent. jq -r '.signal_constraints[] | select(.constraint_kind.kind == \"tied_to\") | .subject_signal' generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json -> ARLOCK AWLOCK ARSNOOP AWSNOOP WSTRB on 2026-09-26 (no other live chain has one)."
---

**Measured `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.10.e`).** The level reader counted *tied* as a binding
verb like *drive*, so AXI's *AWLOCK and ARLOCK are tied LOW* (a compatibility-table cell) and *An attached
Subordinate must have its AWSNOOP input tied LOW* became `must_be_low` obligations. The adapter then lowered them as
Manager drives: 9 rules in `manager.isf`, and 3 `AWSNOOP` overlaps with a transaction that drives it.

A tie-off states what an input is tied to **when the actor that would drive it omits the signal**. No actor drives
that level. So:

- evidence retypes a *tied* level in a clause with no condition as `TiedTo { value }` (`type_tie_offs`), whichever
  producer read it; a *tied* level with a stated scope (*HWRITE is tied HIGH for the entire burst*) is a hold and
  keeps its level kind;
- semantic gives it no rule consequent, IntentIR no invariant, and the adapter lists it in
  `isf_obligations_not_lowered` (*is a tie-off*).

Its configuration is the signal's own absence on the driver, which the signal's presence condition states
(`AWSNOOP_WIDTH` 0, `Exclusive_Accesses` False) and `KG-ISF-COMPLETENESS.2a.vii.c.vi` reads. `.isf` can state the tied
value once FSMGen ships the absent-input value requested in `.4` (`sf-isf-optional-signal`, R3).
`[[a-condition-never-contains-its-obligation]]` · `[[isf-initiator-perspective-direction]]`.
