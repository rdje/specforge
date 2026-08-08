---
id: cross-stage-artifact-paths-are-absolute
title: Cross-stage builders persist canonical absolute input paths, so generated IR is not move-portable
answers:
  - "why do generated IR files contain the old SpecForge repository path"
  - "are SourceIR EvidenceIR SemanticIR and IntentIR pointers repository relative"
  - "will generated stage artifacts survive moving the repository"
  - "where does SpecForge canonicalize upstream artifact paths before serialization"
  - "how many generated artifacts still mention the deleted boot-volume repository"
date: 2026-08-08
status: current
tags: [artifact-paths, portability, locality, generated, source-ir, evidence-ir, semantic-ir, intent-ir]
evidence: docs/tasks/ARTIFACT-PATH-PORTABILITY.md (.0-.1); crates/specforge/src/persisted_path.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs
reverify: "rg -n 'canonicalize_existing_path' crates/specforge/src/ir/evidence.rs crates/specforge/src/ir/semantic.rs crates/specforge/src/ir/intent.rs crates/specforge/src/ir/adapters.rs"
---

The cross-stage builders canonicalize an existing input path before storing it in the next IR. That turns a
repository-relative CLI argument into an absolute `source_ir_path`, `evidence_ir_path`, `semantic_ir_path`, or
adapter input pointer. On the 2026-08-08 SSD workspace, a read-only census found 335 generated JSON/Markdown
artifacts still embedding the deleted `/Users/richarddje/Documents/github/specforge` root. Those files contain
262,996 matching scalar values: 262,592 are evidence span/anchor `source_path` entries; the rest include every
stage's upstream pointer, SourceIR canonical paths, adapter inputs, and prior-memory evidence paths.

Those strings are stale references, not proof of current boot-volume access, but the serialization behavior
violates the repository-root-relative persistence contract and makes generated stages fragile across moves.
The finding arose during `SWD-SERIAL-EXTRACTION.4e`; `ARTIFACT-PATH-PORTABILITY` owns the repair. Its `.1`
common codec/resolver is now implemented and tested, but no stage producer or generated artifact has adopted it
yet. `.2` and `.3` own those typed seams; `.4` owns verified local-artifact migration and a fail-closed census.
Do not bulk-rewrite ignored artifacts before those consumers can read both legacy and relative forms.
