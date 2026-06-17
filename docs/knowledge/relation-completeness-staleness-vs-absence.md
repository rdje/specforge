---
id: relation-completeness-staleness-vs-absence
title: Docs with actors+constraints but ZERO actor_signal_relations are NOT an extraction gap — it is (A) stale IntentIR recoverable by a deterministic rebuild, or (B) honest absence on register/command/coherency protocols that declare ~0 wire signals
answers:
  - "why do nvme / tilelink / wbspec / i2c / ccix / vt-d / iommu have zero actor_signal_relations"
  - "is the relation-incompleteness on the 0-relation docs a recoverable gap or genuine absence"
  - "why does tilelink have 39 relations in evidence_ir but 0 in intent_ir"
  - "is the canonical intent_ir stale relative to its evidence_ir"
  - "does a deterministic semantic->intent rebuild recover lost actor_signal_relations"
  - "should specforge mint actor-signal relations for nvme / iommu / ccix / register protocols"
  - "is relation-completeness the right bar dimension for register / command / coherency protocols"
  - "what is the north-star bar #2 relation-completeness finding (KG-ISF-COMPLETENESS.3)"
date: 2026-06-17
tags: [kg-isf-completeness, actor-signal-relations, relation-completeness, staleness, register-protocols, message-fields, honest-absence, north-star, adr-0006, measured]
evidence: docs/research/relation-completeness-measurement.md (full census + content sampling + live rebuild proof); generated/evidence_ir/* vs generated/intent_ir/* (relation-count divergence); crates/specforge/src/ir/semantic.rs + ir/intent.rs (deterministic actor_signal_relations carry); docs/tasks/KG-ISF-COMPLETENESS.md (.3)
reverify: "Staleness: python3 -c \"import json; e=len(json.load(open('generated/evidence_ir/tilelink_1_7_1_specification/evidence_ir.json'))['actor_signal_relations']); i=len(json.load(open('generated/intent_ir/tilelink_1_7_1_specification/intent_ir.json'))['actor_signal_relations']); print('ev',e,'int',i)\" then ./target/release/specforge semantic generated/evidence_ir/tilelink_1_7_1_specification/evidence_ir.json && ./target/release/specforge intent generated/semantic_ir/tilelink_1_7_1_specification/semantic_ir.json -> int recovers 0->39, 40/40 actors connected. Honest absence: nvme/iommu/vt-d/ccix declare ~0 'Signal X ...' statements (register/command/coherency protocols)."
---

**Measured `2026-06-17` (read-only census over 78 evidence + 36 intent artifacts + content
sampling + one live rebuild; `docs/research/relation-completeness-measurement.md`).**

Whole documents carrying actors and constraints but **0** `actor_signal_relations` are NOT a
relation-extraction gap. Two cleanly-separated causes:

**(A) STALE IntentIR — recoverable, fabrication-free.** The canonical `intent_ir.json` is
stale relative to its `evidence_ir.json`: `tilelink_1_7_1` carries **39** relations in
evidence but **0** in its (05-16-dated) intent; `tilelink_1_8_0` 40→0, `um10204` I2C 17→0,
`wbspec` 1→0 (plus a broader "intent older than evidence" set: gic_600/mmu_700/ATS/dti/
opencapi×3/usb4). A deterministic `semantic`→`intent` rebuild (no LLM/Docling) recovers them —
**proven live on `tilelink_1_7_1`: 0→39 relations, 0→69 actor_ports, all 40 actors connected.**
Operational cause (evidence rebuilt under a sweep without cascading downstream; the per-stage
commands do not auto-cascade, only `converge` rebuilds the whole chain), not a code bug.

**(B) HONEST ABSENCE — not a gap.** `nvme`/`risc_v_iommu`/VT-d/`ccix` declare **~0 wire
signals** (their drive/read "cues" are ToC entries, register-access legends `RO`/`RW`, and
agent-MESSAGE/transaction prose — never agent-SIGNAL relations). These register/command/
coherency protocols express intent through `register_records`/`message_field_records`/
`transactions`, NOT actor-signal relations. 0 relations is correct; minting them = fabrication.

**Genericity:** the `actor_signal_relations` surface is intrinsically wire-protocol-shaped;
relation-completeness is the wrong bar dimension for register/message protocols (register- and
message-field-completeness is). A doc can score 0 on the relation bar and be complete on its
own dominant surface — honest, not a miss. Follow-ups: corpus refresh (land the recovered
relations canonically) + a generic stage-staleness `validate` detector. See
[[agent-surface-defect-taxonomy]] (the disconnected-agent fabrication caution).
