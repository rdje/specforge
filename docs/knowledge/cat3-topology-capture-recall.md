---
id: cat3-topology-capture-recall
title: Cat-3 (platform/system-IP) topology-capture recall (DOC-INTENT-TAXONOMY.4c.i) — the signal_connectivity + infrastructure_signals capture is sparse (0.355 edges/actor vs wire's 4.108), three-quarters half-connected (24% both-endpoint vs wire's 85%), and rootless (0/10 infra signals have a resolved clock/reset source); the gap is capture-recall, NOT a missing ISF abstraction → no FSMGen FR, honest residual
answers:
  - "is the cat-3 platform/system-IP topology capture faithful enough to lower to ISF (NO — measured 0.355 signal_connectivity edges/actor over 380 actors / 15 docs, only 24% of edges have both a producer AND a consumer, 0/10 infrastructure_signals have a resolved source; lowering it would synthesize an unfaithful sliver)"
  - "how dense is SpecForge's captured component topology on cat-3 docs vs cat-1 wire docs (cat-3 = 0.355 edges/actor + 24% both-endpoint; cat-1 wire baseline = 4.108 edges/actor + 85% both-endpoint — the SAME signal_connectivity surface is ~12x denser and fully-connected on wire docs, so the surface is capable; the shortfall is capture-recall on platform TRMs)"
  - "should SpecForge file an FSMGen FR for a declarative static-topology ISF construct (NO / not yet — DOC-INTENT-TAXONOMY.4c.i: the bottleneck is upstream extraction-recall, not the missing ISF abstraction; an FR on a 12x-too-sparse / three-quarters-half-connected capture would be unfalsifiable — feedback_verify_fsmgen_before_fr)"
  - "is the cat-3 topology problem name-noise or sparsity (primarily SPARSITY + half-connectedness + rootless clock/reset, NOT noise — endpoints are 95% clean / only 12 escaped edges corpus-wide; refines the .4c 'sparse and noisy' to 'sparse + half-connected + rootless-infra with minor name noise')"
  - "is the clock/reset distribution tree captured for platform docs (only partially — infrastructure_signals is a near-fixed 2-per-doc surface; 6/10 carry a fan-out distributed_to_actor_ids list but 0/10 carry a resolved recovered_source_actor_ids root, so the tree has no captured origin)"
  - "what is the buildable lever for cat-3 topology if pursued (upstream EXTRACTION-RECALL owned OUTSIDE the .4 ISF-lowering program — denser+fully-connected signal_connectivity capture from TRM integration prose/diagrams + clock/reset source resolution; mirrors .4d.i cat-4 CSR recovery and the cat-2 structure-recall frontier; recorded as a cross-reference, NOT a .4 gap)"
  - "which 15 docs are category-3 platform/system-IP in the corpus (GIC-600/400 TRMs, CoreSight SoC-600 x3 / SDC-600 / TMC TRMs, MMU-700 TRM, Cortex-A76 TRM, CoreSight Base System Arch, CoreSight/GIC/SMMU/ARM-Debug-v6/Advanced-Comms-Channel architecture specs; the .1 census never persisted the per-doc labels — .4c.i enumerates them in scripts/measure_cat3_topology_recall.py)"
  - "does lowering cat-3 topology need only an ISF construct or also a multi-actor emit (also a multi-actor emit — ISF is per-actor / one .isf = one FSMGen module and SpecForge's emit is single-initiator-actor; a declarative cross-component netlist is an architectural change, decided WITH FSMGen only after capture-recall clears the bar — not today)"
date: 2026-06-23
tags: [doc-intent-taxonomy, cat-3, platform-system-ip, topology, connectivity, clock-reset, capture-recall, extraction-recall, isf-adapter, honest-residual, verify-fsmgen-before-fr, isf-no-hacks, adr-0006, measured]
evidence: scripts/measure_cat3_topology_recall.py (reproducer); generated/intent_ir/*/intent_ir.json (15 cat-3 docs: signal_connectivity + infrastructure_signals + actors); docs/research/cat3-topology-capture-recall-measurement.md
reverify: "python3 scripts/measure_cat3_topology_recall.py -> CAT-3 (15 docs): actors=380, signal_connectivity edges=135, edges/actor=0.355, both-endpoint=33 (24%), clean endpoints 457/480 (95%), infrastructure_signals=10 with-distribution=6 with-RESOLVED-source=0; CAT-1 wire baseline (4 docs): edges/actor=4.108, both-endpoint 227 (85%), infra with-source=1. Robust to dropping the 4 borderline docs (cat-3 core: 0.395 edges/actor, 20% both-endpoint, 0 resolved source). Decision: capture-recall-gated not abstraction-gated -> no FSMGen FR, honest residual, extraction-recall lever owned outside .4. Read-only/docs-only -> golds/kg-bench orthogonal."
---

`DOC-INTENT-TAXONOMY.4c.i` is the cat-3 (platform / system-IP) topology-capture recall measurement spun out of
`.4c` — a read-only, docs-only leaf that decides, from evidence, whether cat-3's distinctive topology intent is
captured faithfully enough to be worth a new ISF static-topology construct (a verified FSMGen FR), or whether
the bottleneck is upstream capture-recall (making an FR premature).

**Measured (full 15-doc cat-3 set, reproducer `scripts/measure_cat3_topology_recall.py`).** The two typed
topology surfaces capture an **unfaithful** picture of a multi-component subsystem:

- **Sparse:** 380 actors → 135 `signal_connectivity` edges = **0.355 edges/actor**. The *same surface* on the
  cat-1 wire reference baseline is **4.108 edges/actor** (the protocol's small, fully-declared signal graph) —
  ~12× denser. The surface is *capable*; the shortfall is how much of a TRM's prose interconnect the extractor
  recovers.
- **Half-connected:** only **24%** of cat-3 edges carry both a producer and a consumer (vs **85%** on wire
  docs); the rest are unusable stubs. GIC-600's 66 edges collapse to 4 usable; GIC-400's 8 to 0.
- **Rootless infra:** `infrastructure_signals` is a near-fixed 2-per-doc surface; **0/10** carry a resolved
  clock/reset *source* (only 6 carry a fan-out list) — a distribution tree with no captured root.
- Name quality is the *smaller* problem (endpoints 95% clean; 12 escaped edges corpus-wide), refining `.4c`'s
  "sparse **and noisy**" to **sparse + half-connected + rootless-infra**.

All three findings are **robust** to dropping the 4 borderline docs (cat-3 core: 0.395 edges/actor, 20%
both-endpoint, 0 resolved source).

**Decision.** Cat-3 topology capture is **capture-recall-gated, not abstraction-gated** → **no FSMGen FR**;
topology stays an **honest residual**. Cat-3's register half already lowers
([[register-bit-field-isf-lowering-gap]]). The buildable lever — if ever pursued — is **upstream
extraction-recall owned OUTSIDE the `.4` ISF-lowering program** (denser/fully-connected connectivity capture +
clock/reset source resolution), recorded as a cross-reference, not a `.4` gap. Even with faithful capture,
lowering would additionally need a multi-actor ISF emit + a construct FSMGen's behavioral ATL frontier does not
subsume — an architectural question resolved *with* FSMGen only after capture clears this bar. Mirrors
[[cat4-isa-csr-lowering-decision]] and completes the `.4c` decision ([[cat3-topology-isf-lowering-decision]]).
