---
id: a-polarity-rule-reaches-its-stated-scope
title: A polarity the document states for a scope of signals reaches every signal it places in that scope — Wishbone's interface-class rule (23 signals), AHB's *the only active LOW signal in the protocol* (the other 40), ADIv6's section rule (1) — and never a signal named by spelling (AXI's *Lowercase n* convention stays unread); since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.17)
answers:
  - "where does SpecForge read a document-wide or class-wide polarity statement"
  - "why do Wishbone's CYC_O STB_O ACK_I have an active_high polarity record"
  - "why are AHB's signals other than HRESETn active_high"
  - "why does AXI still list asserted/deasserted obligations as polarity unknown"
  - "does SpecForge read the Lowercase n active-LOW naming convention"
  - "what is scope_rule_polarity_observations / PolarityScopes"
  - "why did Wishbone's CYC_O must be asserted no later than constraint disappear"
date: 2026-09-26
tags: [evidence, polarity, adr-0037, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/evidence.rs (scope_rule_polarity_observations, interface_class_polarity_rule, interface_class_members, PolarityScopes, statement_section_indices, synthesize_reset_class_rule_polarity_statements, states_a_level_bounded_by_an_event); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.17, .17.a, .33); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.1e)
reverify: "cargo test --offline -p specforge-core --lib -- a_class_rule_reaches_every_signal the_only_signal_of_one_polarity a_section_rule_reaches a_reset_polarity_stated_over_a_class a_level_bounded_by_an_event. jq '.signal_polarities | length' generated/evidence_ir/{wbspec_b4_wishbone_b4_specification,ihi0033_c_2021_09_amba_5_ahb_protocol_specification,ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification}/evidence_ir.json -> 23, 41, 5 on 2026-09-26."
---

**Measured `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.17`).** Across the 78 evidence chains, five documents state
polarity for a *scope* of signals rather than for one. Three scopes are read, each observation citing the rule's
own statement and arbitrated with every other polarity observation (a disagreement is a conflict, never a choice):

| document | statement | scope | reaches |
| --- | --- | --- | --- |
| Wishbone | *All WISHBONE interface signals MUST use active high logic.* | the roles *WISHBONE MASTER and SLAVE interfaces* names; their signals from the *MUST include the following signals* lists and their own signal sections (`.32`) | 23 signals, not SYSCON's `CLK_O`/`RST_O` |
| AHB | *The reset signal, HRESETn, is the only active LOW signal in the protocol* | every other signal of the declared catalog | 40, including Table A-2's `HRESET` (the PDF's own spelling) |
| ADIv6 | *All signals that are described in this section are active-high* | declared signals named by the statements of its section (B2.3.2) | `CSYSPWRUPACK`; the section's other three wires are rows the catalog does not declare (`SIGNAL-DECLARATION-ROW-DROP.1e`) |

Wishbone's membership cannot come from its minimum lists alone: *As a minimum, the MASTER interface MUST include …
[CYC_O] …* is split across two statements, so the list reads only `ACK_I`. The reset's `Reset RST_I is active high.`
statement now follows the resolved record the class rule gives it, so a contested reset gets none.

**Not read:** AXI's only convention is *Lowercase n … denotes an active-LOW signal*, a rule about spelling that ADR
0037 forbids, so its 17 obligations, 14 temporal rules and 1 guard stay listed as polarity unknown (honest
residuals). Avalon's *In the following table, all signal roles are active high* and GIC-400's *All PPI signals are
active-LOW* scope members this reader does not ground, and neither is a live chain (`.17.a`); GIC-600's is a
configurable default.

**Found by reading it:** Wishbone's `CYC_O` constraint came from *[CYC_O] MUST be asserted no later than the rising
[CLK_I] edge that qualifies the assertion of [STB_O]* — an order between two events, not a level. With its polarity
read it lowered as three always-HIGH `(CYC_O 1)` rules. `states_a_level_bounded_by_an_event` now mints no
constraint for *must be `<level>` no later/earlier than …*; reading the order is `.33`, and `master.isf` is
byte-identical to before.
`[[wishbone-clock-and-reset-come-from-prose]]` · `[[a-condition-never-contains-its-obligation]]`.
