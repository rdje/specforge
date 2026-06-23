---
id: isf-unconditional-rule-overlap-conflict
title: The `.isf` emitter drops a rule that conflicts with an UNCONDITIONAL driver on the same signal (an empty-guard rule overlaps every guard, so FSMGen `isf_conflicting_rule_writes` rejects any different-value rule on that target) and records an `isf_unconditional_overlap_<name>` residual — closing the cross-guard conflict the same-guard dedup missed (KG-ISF-COMPLETENESS.2a.v, Lever C, surfaced on AMBA Low Power Interface)
answers:
  - "why does an .isf rule get dropped when it conflicts with an unconditional rule on the same signal"
  - "what is KG-ISF-COMPLETENESS.2a.v (ISF unconditional-rule-overlap conflict residual / Lever C)"
  - "why did the AMBA LPI controller.isf fail fsmgen strict with isf_conflicting_rule_writes on PREQ/PACCEPT"
  - "why did the AXI/AHB/AXI-Stream wire-gold .isf fail fsmgen --strict on a fresh re-emit (isf_conflicting_rule_writes), and what fixed them"
  - "why does the same-guard dedup_conflicting_rules miss a conflict between an unconditional rule and a guarded rule"
  - "how does FSMGen decide two rule data-writes conflict (same target, different value, NOT compatible/disjoint/priority/resource resolved) and when is a guard proven disjoint (_condition_terms_prove_disjoint: shared eq: signal with different values; an absent/empty condition is NEVER proven disjoint)"
  - "what does drop_unconditional_overlap_conflicts / unconditional_overlap_residual_packet do in ir/isf_ir.rs"
  - "why not emit a (priority A over B) to resolve a rule/rule conflict instead of dropping (the unconditional minority would conflict with EVERY same-value unconditional rule → an ungrounded precedence over each = fabrication; tested: priority rule_5 over _0012 cleared one pair then rule_6 conflicted next)"
  - "are currently-strict-clean .isf affected by the overlap drop (no — a clean doc cannot contain an unconditional-overlap config or FSMGen would already reject it → byte-identical by construction; 100/107 emitted .isf unchanged)"
date: 2026-06-23
tags: [kg-isf-completeness, isf, emitter, rule-conflict, fsmgen-strict, isf_conflicting_rule_writes, adr-0006, residual, lever-c, fix, wire-gold]
evidence: crates/specforge/src/ir/isf_ir.rs (drop_unconditional_overlap_conflicts — per-signal unconditional value, drop different-value rules; unconditional_overlap_residual_packet → isf_unconditional_overlap_<name>; wired in from_intent_ir after dedup_conflicting_rules, before priority emit); subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm (_build_conflict_issues:10888, _condition_terms_prove_disjoint:10458, _priority_resolved_record_pair:10953); docs/tasks/KG-ISF-COMPLETENESS.md (.2a.v node + acceptance checklist)
reverify: "RAM-safe, no VLM/Docling. cargo build -p specforge. LPI: specforge adapt generated/intent_ir/ihi0068_d_2021_10_amba_low_power_interface_specification/intent_ir.json --target isf; perl subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0068_d_*/controller.isf -> success=true 0 diagnostics; adapter.json residual_decisions carry isf_unconditional_overlap_temporal_temporal_signal_constraint_dyn_sigcon_0011 + _0012. No-regression: re-emit all generated/adapters/isf/*/*.isf with the change vs without (git stash the isf_ir.rs change, rebuild, re-emit) and diff -> exactly 7 .isf differ, 100 byte-identical; fsmgen on the 7 -> 6 FAIL->PASS (AXI ihi0022_l, AHB ihi0033_c, AXI-Stream ihi0051_b, LPI, LTI ihi0089_d, NVMe), AXI+ACE ihi0022_h_c stays FAIL on the orthogonal '(port expr)' grammar (pre-existing Non-Goal), 0 PASS->FAIL. run_ci.sh GREEN (lib 1708); kg-bench 156/156."
---

**Built `2026-06-23` (`KG-ISF-COMPLETENESS.2a.v`, CODE — "Lever C").** The last open ISF strict-FAIL lever:
the AMBA Low Power Interface (`ihi0068_d`) `controller.isf` failed FSMGen `--strict --check` with
`ISF conflict 'isf_conflicting_rule_writes' on target 'PREQ': … rule 'rule_5' (rule_action, <- 1) conflicts
with rule 'temporal_temporal_signal_constraint_dyn_sigcon_0012' (rule_action, <- 0)` (plus a masked twin on
`PACCEPT`).

## Root cause (WHY + WHERE)
`rule_5`/`constraint_3` are **unconditional** (`IsfRule.condition == ""` → rendered with no guard, always
active) and drive their target ←1; `..._dyn_sigcon_0012`/`..._0011` are **guarded** (`(== PACCEPT 0)`) and
drive ←0. SpecForge's existing conflict dedup `dedup_conflicting_rules` (`ir/isf_ir.rs`) keys on
`(signal, condition)` — the **same-guard** overlap model — so an unconditional rule (guard `""`) and a guarded
rule hash to different keys and the conflict is never detected. FSMGen flags it anyway: its
`_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) can NEVER prove an
absent/empty condition disjoint, so an unconditional rule's firing set ⊇ every guard → it overlaps any
different-value rule on the same target (`_build_conflict_issues:10888`).

## The fix (drop + honest residual; the priority hatch is fabrication)
After the same-guard dedup (which leaves ≤1 unconditional value per signal — two unconditional rules on one
signal share key `(S, "")` so the second is already gone), `drop_unconditional_overlap_conflicts` computes the
unconditional driver value `V` per signal and DROPS every other rule driving that signal to a value `≠ V`,
recording an `isf_unconditional_overlap_<name>` `ResidualDecisionPacket` (the unconditional value wins; the
dropped obligation is explicit, never silently lost). It is **precise** — it drops exactly FSMGen's flagged
overlap and nothing else, so a strict-clean doc (which by construction cannot contain such a config) re-emits
byte-identical. FSMGen's `(priority …)` escape-hatch (`_priority_resolved_record_pair:10953`) was tested and
**rejected**: it cleared one pair but the guarded minority rule then conflicted with the next unconditional
same-value rule (`rule_6`), so keeping it would require asserting an ungrounded precedence over EVERY
unconditional rule — fabrication (`[[feedback_isf_no_hacks]]`).

## Measured impact (the resume-pointer "27/28 clean, 1 FAIL (LPI)" tally was STALE)
A full fresh current-binary re-emit + FSMGen sweep showed the conflict was **not** LPI-only: the
AXI/AHB/AXI-Stream **wire golds**, LTI, and NVMe were all FSMGen-strict-FAILING on the same
unconditional-overlap `isf_conflicting_rule_writes`. The fix takes **6 docs FAIL→PASS** (AXI `ihi0022_l`, AHB
`ihi0033_c`, AXI-Stream `ihi0051_b`, LPI `ihi0068_d`, LTI `ihi0089_d`, NVMe), with **0 PASS→FAIL
regressions** and **100/107 emitted `.isf` byte-identical**. The combined AXI+ACE `ihi0022_h_c` stays FAIL on
the **orthogonal** `rule … assignment actions require '(port expr)'` grammar (a pre-existing spun-out Non-Goal
of `ISF-VALUE-WIDTH-EMIT`), unchanged by this lever. WIRE-BASED-100 is orthogonal **by construction** — the
change is confined to the `.isf` emitter (`isf_ir.rs`); `eval-extraction` reads the IR, never the `.isf`.

ADR-0006: structural ISF semantics (an empty-guard rule is unconditional ⇒ overlaps every guard on its
target, mirroring FSMGen's own disjointness model), no chip/vendor/protocol name list. Sibling of the
same-guard `[[isf-rule-conflict-residual]]`-style drop and `[[isf-enum-value-literal-emit-gate]]` (Lever F).
`[[project_kg_isf_completeness]]` (bar #6) / `[[feedback_verify_fsmgen_before_fr]]`.
