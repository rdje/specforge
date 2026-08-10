---
id: opencapi-data-link-layer-refresh-is-signal-empty
title: The OpenCAPI Data Link Layer v2.0 refresh declares no interface signals, so ISF lowering blocks honestly
answers:
  - "why does the OpenCAPI Data Link Layer specification emit no isf target"
  - "were the CDR and DL outputs in endpoint_dlx.isf grounded protocol authority (no)"
  - "what did refresh 50 change in the OpenCAPI data link chain"
  - "why did OpenCAPI data link semantic phases gates and contracts drop to zero"
  - "how many pages elements and normalized files does the OpenCAPI data link ingest produce"
date: 2026-08-10
tags: [corpus-coverage, opencapi, refresh, isf-lowering, honest-residual, evidence-ir]
evidence: docs/tasks/CORPUS-COVERAGE.md (CORPUS-COVERAGE.2.50); generated/adapters/isf/opencapi_data_link_layer_v20_09jul2020/adapter.json
reverify: "target/release/specforge adapt generated/intent_ir/opencapi_data_link_layer_v20_09jul2020/intent_ir.json --target isf --dry-run"
---

The OpenCAPI Data Link Layer v2.0 specification describes a link layer in prose, state narration, and flit-format
tables. It never declares an interface signal catalog. Its stale chain nevertheless carried four interfaces, four
ports, four actor-signal relations, 54 phases, 71 gates, 38 contracts, and an emitted `endpoint_dlx.isf` whose
`CDR`/`DL` outputs and `REPRESENTATION`/`DL` enums came from retired heuristic topology rather than from any
declared signal.

Two guarded CPU ingests reproduce 57 pages / 64 visuals / 53 tables / 111 sections / 527 elements and a 184-file /
52,570,034-byte normalized bundle; two full downstream cascades reproduce every artifact and report hash. The
current chain keeps 1,018 statements, 1,308 evidence links, and 14 conditional rules, and it carries zero
actor-signal relations, signal constraints, temporal rules, temporal conflicts, registers, and timing constraints.
SemanticIR retains six actors, 87 invariants, and four assertions with no interface, phase, gate, or contract;
IntentIR retains four actors, 24 behaviors, 87 constraints, and one assumption. `adapt --target isf` blocks with
the single reason `no signals declared in interface` and emits nothing, leaving only `adapter.json`.

This is an honest boundary, not an extraction gap: the correct output for a document with no declared signal
authority is a validated IntentIR plus a blocked lowering, never a synthesized interface. The refresh also
provided the reproduction case for [[passive-binding-subject-authority]], whose four post-binding uppercase
subjects (`CAPI`, `OCDE`, `DLX`) were the last fabricated surface this document produced.
