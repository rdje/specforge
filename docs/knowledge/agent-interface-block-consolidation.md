---
id: agent-interface-block-consolidation
title: The .1b.ii named-interface consolidation folds an "X interface" relation subject onto the bare agent "X" (Subordinate interface -> Subordinate) ONLY when X is independently a connected relation subject in the same doc, so a distinct named block like the GIC CPU interface (whose CPU is never an agent) is preserved; a post-pass in actor_signal_relation_surface after the coordinated split and before dedup, per-doc connected-agent gate, no name list (ADR 0006)
answers:
  - "how does specforge fold an X interface relation subject onto the bare agent X"
  - "what is the KG-ISF-COMPLETENESS.1b.ii named-interface consolidation"
  - "why is Subordinate interface merged to Subordinate but GIC CPU interface kept intact"
  - "where is consolidate_interface_actor_relations and strip_interface_suffix wired"
  - "why is the X interface strip gated on X being a connected agent in this doc"
  - "why does the same token AXI interface merge in one doc but not another"
  - "why is the interface consolidation safe for WIRE-BASED-100 (gold docs have no interface actor)"
date: 2026-06-17
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, completeness, consolidation, interface, adr-0006, measured, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (strip_interface_suffix + consolidate_interface_actor_relations, wired as a post-pass in actor_signal_relation_surface AFTER split_coordinated_actor_relations and BEFORE dedup_actor_signal_relations); docs/research/agent-surface-fidelity-measurement.md §7.1 + §9; docs/tasks/KG-ISF-COMPLETENESS.md (.1b.ii)
reverify: "cargo test -p specforge --lib interface_consolidation (interface_consolidation_merges_when_lead_is_connected_agent + interface_consolidation_keeps_named_block_when_lead_not_connected) and interface_suffix_strip_returns_lead_only_for_trailing_interface. Live: rebuild AXI-Stream (ihi0051_b) + AXI+ACE (ihi0022_h_c) evidence->semantic->intent into a temp evidence-root (symlink canonical generated/source_ir in, run the release binary from the temp CWD) and confirm 'Transmitter interface'/'Subordinate interface' actors are GONE while Transmitter/Subordinate absorbed their relations. WIRE-BASED-100: the 4 gold docs (APB/AHB/AXI/SWD) carry no '* interface' actor, so eval-extraction seed_{apb,ahb,axi}{,_temporal}.json --provider skip --evidence-root <fresh root> stays source-tolerant 1.000; kg-bench 156/156."
---

**Landed `2026-06-17` (`KG-ISF-COMPLETENESS.1b.ii`).** The third Class-B completeness recovery for the
agent surface (`[[agent-surface-defect-taxonomy]]` Class B), alongside `.1b.i` trailing-fragment
consolidation (`[[agent-trailing-fragment-consolidation]]`) and `.1b.iii` coordinated-subject split
(`[[agent-coordinated-subject-split]]`). It completes the `.1b` umbrella, so with `.1a`
(`[[agent-identity-structural-gate]]`) the agent-surface fidelity goal (`KG-ISF-COMPLETENESS.1`) is fully
built.

**The defect + why it was deferred.** A specification often names a real agent as its *interface* —
"Subordinate interface", "Transmitter interface" — written separately from the bare agent, stranding that
form's relations under a phantom actor. The naive `"X interface"→"X"` strip is UNSAFE: the GIC
"CPU interface" is the GICC, a distinct architectural block, NOT a generic "CPU"; "Q-Channel interface" and
an unconnected "AXI interface" are the same. The `.1b` measurement (§7.1) deferred the strip because the
safe gate needs the document's actor-set context, which the pure-string relation-subject seam
`normalize_relation_actor_name` does not have.

**The rule (per-doc connected-agent gate).** Strip `"X interface"`→`X` **iff `X` is, in this document, an
independent connected relation subject** (one that is not itself an "* interface" form). The connected set
is the document's OWN evidence, never a name list (ADR 0006) — the decisive census datum is that
"AXI interface" is SAFE in CoreSight `100806_0701` (where `AXI` is connected) but a RISK in
`100806_0100`/`0200` (where it is not): same token, opposite status by document, exactly the genericity
guardrail `transmitter` proved (`[[agent-surface-defect-taxonomy]]` §4).

**The seam.** `strip_interface_suffix` (case-insensitive trailing " interface", returns the lead) +
`consolidate_interface_actor_relations` (builds the connected set, rewrites a subject only when its stripped
lead is in that set), a post-pass in `actor_signal_relation_surface` placed AFTER
`split_coordinated_actor_relations` (so a split-produced "X interface" conjunct is caught) and BEFORE
`dedup_actor_signal_relations` (so the rewritten relation merges with X's existing ones) — the same slot the
`.1b.iii` split uses, where the full relation list is available.

**Safe + proven.** Only 6 corpus docs carry an "* interface" actor (3 safe-merge, 5 conflation-risk); NONE
of the 4 WIRE-BASED-100 gold docs (APB/AHB/AXI/SWD) do, so the gold relation surface is structurally
untouched. Live (fresh post-`.1a`/`.1b` rebuild): AXI-Stream `Transmitter interface`→`Transmitter` (23
rels), AXI+ACE `Subordinate interface`→`Subordinate` (49 rels); the 4 wire docs unchanged. WIRE-BASED-100
held 1.000 (constraints + relations + temporal, APB/AHB/AXI); `kg-bench` 156/156; `run_ci.sh` green (lib
1657, +3 tests). The reclaimed CoreSight + missing-source GIC RISK docs are not live-rebuildable, so the
conflation guard is locked by the `interface_consolidation_keeps_named_block_when_lead_not_connected`
(`CPU interface`) unit test. See `[[project_kg_isf_completeness]]`.
