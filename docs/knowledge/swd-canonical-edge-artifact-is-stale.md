---
id: swd-canonical-edge-artifact-is-stale
title: The canonical SWD edge-timing artifact predates the fresh disposable 29/29 proof
answers:
  - "why does canonical SWD EvidenceIR have zero interface edge timings"
  - "did the path migration remove the SWD edge timing record"
  - "why can current SWD EvidenceIR not be rebuilt from its SourceIR"
  - "was the fresh SWD 29 of 29 artifact promoted"
  - "what must be refreshed before closing SWD projection"
date: 2026-08-09
status: current
tags: [swd, canonical-artifact, staleness, ingest, interface-edge-timing]
evidence: docs/tasks/SWD-SERIAL-EXTRACTION.md (.4e, .7a); generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json; generated/source_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/source_ir.json
reverify: "jq '{frame:(.serial_frame_fields|length),operations:(.swd_operations|length),states:(.protocol_states|length),edges:(.interface_edge_timings|length)}' generated/evidence_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/evidence_ir.json"
---

The tracked canonical SWD EvidenceIR currently contains 11 serial-frame fields, four operations, 13
protocol states, and zero interface-edge timing records. Its extraction manifest likewise has no
`interface_edge_timings` run, while `statement_1948` still contains the exact rising-edge sampling and
drive-change clause.

This is expected stale execution state, not an extraction or path-migration regression.
`SWD-SERIAL-EXTRACTION.4e` proved the complete record and 29/29 score using a fresh repository-local,
disposable CPU re-ingest. It deliberately did not promote that chain because the then-open cross-stage
absolute-path defect made canonical promotion unsafe. `ARTIFACT-PATH-PORTABILITY` later migrated the
existing cache without changing its semantic membership.

The canonical SourceIR still carries 400 pages, 210 tables, and 6,784 content elements, but its reclaimed
normalized Markdown leaf is absent. `specforge evidence <that-source-ir> --dry-run` therefore fails closed
instead of rebuilding from incomplete material. The tracked ADI PDF remains present, so
`SWD-SERIAL-EXTRACTION.7e` must perform and promote a fresh repository-local CPU ingest after the downstream
projection/accounting code lands.
