---
id: cat3-topology-fsmgen-actor-network-reassessment
title: FSMGen a51dcdad0 now has bounded static actor-network metadata and actor/pin handoffs, so cat-3 topology needs a fresh abstraction-fit assessment; the sparse/rootless SpecForge carrier is not thereby lowerable
answers:
  - "does current FSMGen have any static actor instance or group construct (yes at a51dcdad0 — bounded actor instances/groups plus transaction-scoped actor/pin handoffs)"
  - "is the old claim that FSMGen has no declarative static-topology construct still current (no — that premise is superseded and must be reassessed)"
  - "can SpecForge category-3 signal_connectivity lower to current FSMGen actor networks now (not yet proven; actor types, typed endpoints, widths, ownership, transactions, and multi-actor emission remain missing or unmeasured)"
  - "what is DOC-INTENT-TAXONOMY.4c.ii"
  - "why does FSMGen actor-network support not erase the cat3 topology capture-recall gate"
  - "does SpecForge need to file a topology feature request now (not before .4c.ii measures the current contract and carrier fit)"
date: 2026-08-12
tags: [cat-3, topology, fsmgen, actor-network, static-instance, actor-handoff, pin-handoff, reassessment, capture-recall]
status: current
supersedes: cat3-topology-isf-lowering-decision
evidence: FSMGen pin a51dcdad0 docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md and docs/isf-spec/10-downstream-readiness-source.md / 11-downstream-actor-transactions.md; docs/tasks/FSMGEN-REFRESH-INTEGRATE-6.md; docs/tasks/DOC-INTENT-TAXONOMY.md (.4c.ii); docs/knowledge/cat3-topology-capture-recall.md
reverify: "At subs/fsmgen pin a51dcdad0, inspect the downstream ISF spec for direct static instance aliases, group/concurrent metadata, actor-to-actor and actor/pin handoffs, and the network-wrapper fail-closed boundary; then run scripts/measure_cat3_topology_recall.py to retain the measured SpecForge carrier limits. Execute DOC-INTENT-TAXONOMY.4c.ii before claiming a mapping or filing correspondence."
---

The historical `.4c` decision was correct for FSMGen pin `d327129b7`, but one premise is no longer current.
FSMGen `a51dcdad0` can describe bounded static actor instances (including compact/library-qualified forms),
report-only actor groups, and transaction-scoped actor-to-actor or actor/pin data handoffs. The broader
`(network …)` wrapper still fails closed.

That upstream capability does not automatically make SpecForge's category-3 topology executable. The measured
carrier remains 0.355 connectivity edges per actor, only 24% of edges have both endpoints, and 0/10 captured
infrastructure signals have a resolved source. Current `signal_connectivity` also does not necessarily ground the
actor type, endpoint direction/type/width, transaction-scoped movement, or generated child/library context that
FSMGen requires; SpecForge still emits one initiator actor.

`DOC-INTENT-TAXONOMY.4c.ii` therefore reopens only the abstraction-fit question. It must measure the exact current
grammar and compare every required binding with the typed carrier. Until that work proves a faithful mapping,
topology remains an honest residual and no speculative FSMGen request or emitter lowering is authorized.

This card supersedes [[cat3-topology-isf-lowering-decision]] only where that older card claims no static
actor-network construct exists. Its capture-recall evidence remains valid and is preserved in
[[cat3-topology-capture-recall]].
