---
id: an-invariant-holds-at-every-cycle
title: IntentIR makes a temporal invariant from a signal constraint only when the constraint holds at every cycle — not when its condition cannot be read (.22), and not when the temporal rule SemanticIR derives from it carries a cycle window (.33); the windowed rule is then its only representation; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.33)
answers:
  - "why is there no tinv_sc invariant for Wishbone RST_I must be asserted for at least one complete clock cycle"
  - "when does synthesize_temporal_invariants skip a signal constraint"
  - "how does IntentIR find the temporal rule derived from a signal constraint (signal_constraint_rule_id)"
  - "where is a minimum duration such as for at least one clock cycle kept"
  - "why did APB-E PADDR PWDATA PSEL and AHB HEXOKAY lose their tinv_sc invariants"
date: 2026-09-26
tags: [intent, invariants, temporal, windows, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/intent.rs (synthesize_temporal_invariants); crates/specforge/src/ir/semantic.rs (signal_constraint_rule_id, resolve_cycle_window_from_text, extract_cycle_window_from_text); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.33, .22, .29, .11)
reverify: "cargo test --offline -p specforge-core --lib -- a_constraint_placed_at_a_window_is_not_an_invariant. jq '[.temporal_invariants[] | select(.invariant_id == \"tinv_sc_sigcon_0002\")] | length' generated/intent_ir/wbspec_b4_wishbone_b4_specification/intent_ir.json -> 0 on 2026-09-26."
---

**Measured `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.33`).** A temporal invariant states a fact at every cycle.
`synthesize_temporal_invariants` built one from every level or stability constraint whose condition it could read,
including constraints whose statement places them at a cycle or a window. Wishbone's *[RST_I] MUST be asserted for
at least one complete clock cycle* became `tinv_sc_sigcon_0002`, *RST_I is 1* always. Yet SemanticIR's rule for
the same constraint (`temporal_signal_constraint_sigcon_0002`) keeps `cycle_window {min_cycles: 1}`.

Now a constraint whose own rule, found by `signal_constraint_rule_id`, carries a cycle window yields no invariant.
On the 27 live chains that removes 5 false invariants and changes no emitted `.isf`, since each was an input's or
already listed:

| chain | constraint | window from |
| --- | --- | --- |
| Wishbone | `RST_I` asserted | *for at least one complete clock cycle* |
| APB-E | `PADDR`, `PWDATA` stable; `PSEL` asserted | a figure's *T2* / *T1* (waveform narration) |
| AHB | `HEXOKAY` not asserted | *in the same cycle as HRESP is asserted*, an exclusion `.11` lowers |

The windowed rule is the obligation's representation. With no event to count from, the adapter lists it as an
unguarded window (`.29`).
`[[a-polarity-rule-reaches-its-stated-scope]]` · `[[a-condition-never-contains-its-obligation]]`.
