---
id: cat3-topology-isf-lowering-decision
title: Cat-3 (platform/system-IP topology) ISF-lowering decision (DOC-INTENT-TAXONOMY.4c) — topology IS captured (signal_connectivity / infrastructure_signals) but sparse+noisy and ISF has no declarative static-topology construct (composition is transaction-level only); no FR filed — next lever is a capture-recall measurement (.4c.i), not code
answers:
  - "does platform/system-IP (category 3) topology intent need a new ISF construct or map onto an existing one (ISF has NO declarative static-topology/connectivity construct — composition is transaction-level only; decision deferred to a capture-recall measurement .4c.i before any FR)"
  - "does SpecForge capture component topology / connectivity for platform docs (YES — a typed signal_connectivity producer->consumer graph + infrastructure_signals clock/reset distribution; correcting the .2 'hint-level' to 'captured-but-sparse-and-unlowered')"
  - "why is category 3 (platform/system-IP) topology not lowered to .isf (ISF has no static-topology construct + the emit is single-initiator-actor; cross-component topology is structurally absent from the emit by design — KG-ISF-COMPLETENESS.2a.ii)"
  - "does FSMGen's multi-actor ATL frontier provide a home for a connectivity netlist (NO — the ATL backlog wires children GENERATED from transaction composition spawn/do; it is behavioral orchestration, not a declarative static IP-interconnect netlist; verified 14-feature-backlog.md)"
  - "is the cat-3 topology capture good enough to lower (NO — sparse + noisy: CoreSight SoC-600 has 6 signal_connectivity edges across 60 actors, GIC-600 66 edges / 2 infra; None/escaped actor names; a capture-recall measurement .4c.i must precede any lowering)"
  - "should SpecForge file an FSMGen FR for cat-3 topology (NOT YET — premature: capture is sparse/noisy AND ISF may deliberately be a per-actor format with topology owned by the integrator above per-module synthesis; resolve with FSMGen after .4c.i, never a speculative FR — feedback_verify_fsmgen_before_fr)"
  - "what already lowers for category-3 platform docs (the register half — register maps + bit-fields via .4a.ii e.g. CoreSight SoC-600 ~3,250 fields, infrastructure signals, actor ports; cat-3's register intent is the same road as cat-2 and is not the gap)"
  - "is ISF a single-actor or multi-actor format (per-actor — one .isf describes one actor/module; SpecForge's emit collapses to one initiator via select_initiator_actor; lowering cross-component topology would need a multi-actor emit, an architectural change not an emitter tweak)"
date: 2026-06-23
tags: [doc-intent-taxonomy, cat-3, platform-system-ip, topology, connectivity, clock-reset, isf-adapter, honest-residual, verify-fsmgen-before-fr, isf-no-hacks, adr-0006, measured, decision-packet]
status: superseded
evidence: generated/intent_ir/100806_0701_17_2025_06_30_coresight_soc_600_technical_reference_manual/intent_ir.json (60 actors / 6 signal_connectivity / 833 registers); generated/intent_ir/100336_0106_00_2019_02_08_gic_600_technical_reference_manual/intent_ir.json (55 actors / 66 signal_connectivity / 2 infrastructure_signals); subs/fsmgen/docs/book/src/13f-composition.md (composition = transaction-level (do child) only); subs/fsmgen/docs/book/src/14-feature-backlog.md (ATL multi-actor = generated-child transaction wiring, not declarative topology); crates/specforge/src/ir/isf_ir.rs (select_initiator_actor — single-initiator-actor emit, KG-ISF-COMPLETENESS.2a.ii); docs/research/cat3-topology-isf-lowering-decision.md
reverify: "Cat-3 docs: python3 -c to count signal_connectivity + infrastructure_signals + actors over generated/intent_ir/100806_0701_17_2025_06_30_coresight_soc_600_technical_reference_manual (expect 60 actors / 6 connectivity / 833 regs) and 100336_0106_00_2019_02_08_gic_600_technical_reference_manual (55 actors / 66 connectivity / 2 infra). FSMGen: sed -n '1,40p' subs/fsmgen/docs/book/src/13f-composition.md -> composition is (do child) transaction-level; grep -niE 'topolog|connectivity|netlist|interconnect' subs/fsmgen/docs/book/src/13*.md -> no declarative static-topology construct; backlog ATL is generated-child transaction wiring. Decision: topology captured-but-sparse + no ISF construct -> no FR yet; next = .4c.i capture-recall measurement, then FR-or-honest-non-target. Docs-only leaf -> golds/kg-bench orthogonal. Related: [[cat4-isa-csr-lowering-decision]], [[register-bit-field-isf-lowering-gap]], [[document-intent-isf-completeness]]."
---

> Superseded in part on `2026-08-12`: FSMGen pin `a51dcdad0` now ships bounded static actor-network
> metadata and actor/pin handoffs. See [[cat3-topology-fsmgen-actor-network-reassessment]]. The measured
> sparse/rootless SpecForge capture limit remains current in [[cat3-topology-capture-recall]].

`DOC-INTENT-TAXONOMY.4c` is the cat-3 (platform / system-IP topology & integration) ISF-lowering decision
packet — a read-only, docs-only leaf resolving whether cat-3's distinctive *topology* intent maps onto an
existing ISF construct or needs a new one.

**Measured (3 representative cat-3 docs of 15).** Cat-3 docs **do** carry a structured topology surface —
correcting the `.2` "hint-level" label to **captured-but-sparse-and-unlowered**: `signal_connectivity` is a
typed producer→consumer graph (GIC-600 = 66 edges, e.g. `DATA` produced by `MISC ignores` → consumed by
`cache several`/`pmu_int`; CoreSight SoC-600 = 6) and `infrastructure_signals` carries a clock/reset
distribution shape (`infrastructure_topology` / `distributed_to_actor_ids`). But the capture is **sparse and
noisy** (6 edges across SoC-600's 60 actors; `None`/escaped actor names), and the register half (SoC-600
~3,250 fields) already lowers via [[register-bit-field-isf-lowering-gap]].

**FSMGen ISF (`d327129b7`) has no declarative static-topology construct.** Composition is transaction-level
only (`(do child)` — one transaction calling another within one actor); the backlog's multi-actor "ATL"
frontier wires children *generated from transaction composition* (`spawn`/`do` fan-out, data-movement
handoff) — **behavioral orchestration, not a declarative IP-interconnect netlist**. And SpecForge's ISF emit
is a **single-initiator-actor** model ([[isf-initiator-perspective-direction]] / KG-ISF-COMPLETENESS.2a.ii),
so cross-component topology is structurally absent from the emit by design; lowering it would need a
multi-actor emit — an architectural change.

**Decision.** Cat-3's register intent already lowers (same road as cat-2). Topology is **captured but
unlowerable** with today's ISF, and **no FR is filed yet** — premature for two measured reasons: (a) the
capture is too sparse/noisy to lower faithfully (lowering 6 edges of a 60-component subsystem = an unfaithful
sliver), and (b) ISF is a per-actor format, so static cross-component topology may be deliberately the
integrator's concern above per-module synthesis (resolve *with* FSMGen, not assume —
[[feedback_verify_fsmgen_before_fr]]). Topology stays an honest residual; the buildable next step is a
**measurement** — `.4c.i`: quantify topology-capture recall/quality across all 15 cat-3 docs — after which the
construct decision is real: a verified FSMGen FR for a declarative static-topology construct + multi-actor
emit, *or* honest-non-target confirmation. Mirrors the cat-4 finding ([[cat4-isa-csr-lowering-decision]]):
the lowerable register half is built; the distinctive half is gated on measurement, never fabrication.
