---
id: agent-coordinated-subject-split
title: The .1b.iii coordinated-subject split replaces a relation whose subject is "X and Y" with one relation per conjunct (so both agents connect to the signal), splitting on "and" only — "or" is left intact because a disjunction is ambiguous (fabrication); a post-pass in actor_signal_relation_surface before dedup, no name list (ADR 0006)
answers:
  - "how does specforge split a coordinated X and Y relation subject into both agents"
  - "what is the KG-ISF-COMPLETENESS.1b.iii coordinated-subject split"
  - "why does specforge split on 'and' but not 'or' for a coordinated actor subject"
  - "where is split_coordinated_actor_relations and split_coordinated_actor_subject wired"
  - "how did the AHB decoder become connected (Subordinate and decoder read HADDR)"
  - "why is the coordination split safe for WIRE-BASED-100 (AHB relation gold)"
date: 2026-06-16
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, completeness, consolidation, coordination, adr-0006, measured, wire-based-100]
evidence: crates/specforge/src/ir/evidence.rs (split_coordinated_actor_subject + split_coordinated_actor_relations + relation_actor_id_slug, wired as a post-pass in actor_signal_relation_surface between augment_check_signal_relations_from_tables and dedup_actor_signal_relations); docs/research/agent-surface-fidelity-measurement.md §7; docs/tasks/KG-ISF-COMPLETENESS.md (.1b.iii)
reverify: "cargo test -p specforge --lib coordinated (coordinated_subject_splits_into_real_conjuncts + coordinated_relation_post_pass_attributes_to_both_agents). Live: build fresh AHB evidence->semantic->intent (Pattern, --dry-run into a temp evidence-root) and confirm decoder 4/2->8/6, Subordinate 27/26->32/31, Exclusive Access Monitor 4/2->5/3, and the actors 'Subordinate and decoder'/'Exclusive Access Monitor and Subordinate' are GONE. WIRE-BASED-100: eval-extraction seed_{axi,apb,ahb,swd}{,_temporal}.json --provider skip --evidence-root <fresh-Pattern root> stays 1.000 on constraints(AXI/APB/AHB)+relations(x4)+temporal(x3); kg-bench 156/156."
---

**Landed `2026-06-16` (`KG-ISF-COMPLETENESS.1b.iii`).** The second Class-B completeness recovery for the
agent surface (`[[agent-surface-defect-taxonomy]]` Class B), after the `.1b.i` trailing-fragment
consolidation (`[[agent-trailing-fragment-consolidation]]`). It connects BOTH agents named in a
coordinated relation subject to the signal they jointly act on.

**The defect.** A coordinated subject — *"the Subordinate and decoder read HADDR"* — was captured as a
single fragment actor `Subordinate and decoder`, stranding relations that belong to two genuine agents.
AHB `decoder` was a 0/0 phantom in part because its reads lived under this coordinated form.

**The fix.** Two functions in `ir/evidence.rs`: `split_coordinated_actor_subject(value)` splits on the
word-bounded conjunction `and`, re-validates each segment through the full agent gate
`normalize_relation_actor_name` (the `.1a` reject + `.1b.i` consolidation), and returns the distinct
survivors (empty when not a coordination or fewer than two survive). `split_coordinated_actor_relations`
is a post-pass that replaces each coordinated relation with one per conjunct (signal / relation /
provenance preserved; a deterministic per-conjunct id `<id>__<slug>` via `relation_actor_id_slug`). It is
wired into `actor_signal_relation_surface` **between `augment_check_signal_relations_from_tables` and
`dedup_actor_signal_relations`**, so a split relation that duplicates an existing one merges (first-wins by
actor/signal/is_drives) and the coordinated-fragment actor — left relation-less — is never minted.

**`and` only, never `or` (ADR 0006 — a closed-class conjunction, no name list).** A conjunction means
BOTH agents act, so both get the relation. A disjunction (`or`) is ambiguous — only one acts — so splitting
it would fabricate a relation for an agent that may not act; `or` is left intact (honest residual over
fabrication).

**Safe + proven.** WIRE-BASED-100 risk was analysed before coding: the AHB relation gold is 6 `drives`
facts on unrelated statements (HRESP/HREADYOUT/H*USER), none coordinated, and the eval scorer does not
penalise off-gold relations (the `.1b.i` eval had AXI 343 rels / fp=0). Measured: exactly 2 coordinated
subjects corpus-wide, both AHB, both conjuncts real. Live AHB rebuild: `decoder` 4/2 → **8/6** (now a
fully-connected agent), `Subordinate` 27/26 → **32/31**, `Exclusive Access Monitor` 4/2 → **5/3**, the two
coordinated-fragment actors gone, actors 19 → 17. WIRE-BASED-100 held at 1.000 (constraints ×3 / relations
×4 / temporal ×3); `kg-bench` 156/156; `run_ci.sh` green (+2 tests, lib 1637).

See `[[agent-trailing-fragment-consolidation]]` (the sibling `.1b.i` consolidation it runs alongside),
`[[agent-surface-defect-taxonomy]]`, and `[[nlp-coordination-already-handled]]` (object-side coordination,
already handled; this is the subject-side split).
