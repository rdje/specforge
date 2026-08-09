---
id: swd-canonical-protocol-artifact-is-current
title: The canonical SWD chain contains all 29 scored protocol facts through the adapter boundary
answers:
  - "does canonical SWD EvidenceIR contain interface edge timing"
  - "was the fresh SWD 29 of 29 artifact promoted"
  - "what are the canonical SWD protocol surface counts"
  - "does the current SWD chain come from the tracked ADI PDF"
  - "is the canonical SWD normalized bundle path portable"
  - "did the USB4 signoff check change canonical SWD"
  - "why did the canonical SWD relation count change from 25 to 21"
date: 2026-08-09
status: current
tags: [swd, canonical-artifact, promotion, interface-edge-timing, intent-ir]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.7e.ii); generated/source_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/source_ir.json; generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json; generated/intent_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/intent_ir.json
reverify: "jq '{actors:(.actors|length),relations:(.actor_signal_relations|length),frame:(.serial_frame_fields|length),operations:(.swd_operations|length),states:(.protocol_states|length),edges:(.interface_edge_timings|length)}' generated/intent_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/intent_ir.json && target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_swd_derivation.json --provider skip"
---

The canonical SWD chain was rebuilt from the tracked 2,925,300-byte ADI PDF by a fresh repository-local CPU
ingest after protocol projection, adapter accounting, convergence accounting, and path portability landed. Its
SourceIR is ready with 400 pages, 386 visual assets, 210 structured tables, and 6,784 content elements. The
Docling sidecar names the tracked source and final normalized Markdown with repository-relative paths.

Canonical EvidenceIR contains 11 serial-frame fields, four SWD operations, 13 protocol states, and one complete
interface-edge timing record. The edge is `target / SWDIO / SWCLK / rising / samples=true /
drive_changes=true`, supported by `statement_1948`; the extraction manifest records
`timing.interface_edge_prose` as eligible, produced 1, and kept 1. Source-tolerant scoring is
`P=R=F1=1.000` on all four tasks, covering all 29 independently verified facts.

SemanticIR and IntentIR preserve the same four ordered vectors exactly. The ISF adapter accounts for all 29
records with stable `isf_protocol_*` residual packets because the current record schemas do not license direct
behavioral lowering; the adapter remains renderable and its emitted ISF passes FSMGen strict with zero
diagnostics.

During `CORPUS-COVERAGE.2.34b.i`, a temporary fresh-SWD signoff command resolved its artifact root to the
canonical cache. The downstream chain was preserved first, then validation and a complete deterministic cascade
promoted the current generic dense-prose authority result. That result removes the synthetic `LEVEL` signal,
three phrase-shaped actors (`Class x`, `DbgSwEnable flag`, and `system`), and four associated relations. The
canonical graph is now 19 actors / 21 actor-signal relations / 13 interfaces / 19 ports; the adapter is
renderable with 11 signals, one rule, one enum, 45 storage records, and 32 explicit residuals. No legitimate SWD
wire or protocol vector was lost.

Two complete cascade replays reproduced the final EvidenceIR, SemanticIR, IntentIR, and adapter hashes
`e125616e…df9`, `0a9d75b5…a82`, `dd32a0a7…2d5`, and `5dbf105c…b5b` byte-for-byte. SWD derivation remains
1.000 for all 29 facts; the separate base relation score remains 1/1, while the long-standing Pattern-only
`CSYSPWRUPACK` constraint miss remains 0/1. That constraint score is not part of the 29-fact derivation surface.
