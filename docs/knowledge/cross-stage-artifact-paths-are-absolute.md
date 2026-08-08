---
id: cross-stage-artifact-paths-are-absolute
title: Every canonical stage now writes move-safe paths while the legacy generated corpus awaits migration
answers:
  - "why do generated IR files contain the old SpecForge repository path"
  - "are SourceIR EvidenceIR SemanticIR and IntentIR pointers repository relative"
  - "will generated stage artifacts survive moving the repository"
  - "where does SpecForge canonicalize upstream artifact paths before serialization"
  - "how many generated artifacts still mention the deleted boot-volume repository"
  - "which IR stages have adopted move safe persisted paths"
date: 2026-08-08
status: current
tags: [artifact-paths, portability, locality, generated, source-ir, evidence-ir, semantic-ir, intent-ir]
evidence: docs/tasks/ARTIFACT-PATH-PORTABILITY.md (.0-.3); crates/specforge/src/persisted_path.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs
reverify: "cargo test -p specforge downstream_artifacts_store_relative_paths_and_rebase_legacy_lineage"
---

Before the portability repair, every cross-stage builder canonicalized its existing input before storing it in
the next IR. That turned a repository-relative CLI argument into an absolute `source_ir_path`,
`evidence_ir_path`, `semantic_ir_path`, or adapter input pointer. On the 2026-08-08 SSD workspace, a read-only
census found 335 generated JSON/Markdown artifacts still embedding the deleted
`/Users/richarddje/Documents/github/specforge` root. Those files contain 262,996 matching scalar values: 262,592
are evidence span/anchor `source_path` entries; the rest include every stage's upstream pointer, SourceIR
canonical paths, adapter inputs, and prior-memory evidence paths.

Those strings are stale references, not proof of current boot-volume access, but the serialization behavior
violates the repository-root-relative persistence contract and makes generated stages fragile across moves.
The finding arose during `SWD-SERIAL-EXTRACTION.4e`; `ARTIFACT-PATH-PORTABILITY` owns the repair. Its `.1`
common codec/resolver and `.2` SourceIR/EvidenceIR producer/consumer activation are implemented and tested.
`ARTIFACT-PATH-PORTABILITY.3` extends that contract through SemanticIR, IntentIR, adapter artifacts, and typed
prior-memory source records. New artifacts at every canonical stage serialize repository-owned layout, upstream,
provenance, emitted-target, and learned-source paths relative while loaders expose current absolute runtime paths;
unlabeled old absolute forms remain readable only through bounded rebasing. Validation, project-validation,
learning, recovery, KG fixtures, and convergence now dereference or create these paths through the common seam.

The 335-file count intentionally remains the current generated-data baseline because the activation leaves did
not rewrite ignored artifacts. `.4` owns verified local-artifact migration and a fail-closed census. Code readiness is now
complete, but the 335-file legacy-data baseline remains until that copy/verify/use/delete leaf lands.
