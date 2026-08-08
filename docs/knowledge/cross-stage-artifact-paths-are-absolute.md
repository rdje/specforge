---
id: cross-stage-artifact-paths-are-absolute
title: Canonical stages and the migrated generated corpus contain move-safe repository paths
answers:
  - "why do generated IR files contain the old SpecForge repository path"
  - "are SourceIR EvidenceIR SemanticIR and IntentIR pointers repository relative"
  - "will generated stage artifacts survive moving the repository"
  - "where does SpecForge canonicalize upstream artifact paths before serialization"
  - "how many generated artifacts still mention the deleted boot-volume repository"
  - "which IR stages have adopted move safe persisted paths"
date: 2026-08-09
status: current
tags: [artifact-paths, portability, locality, generated, source-ir, evidence-ir, semantic-ir, intent-ir]
evidence: docs/tasks/ARTIFACT-PATH-PORTABILITY.md (.0-.4); crates/specforge/src/persisted_path.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs; scripts/check_persisted_artifact_paths.pl
reverify: "perl scripts/check_persisted_artifact_paths.pl --check"
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

`ARTIFACT-PATH-PORTABILITY.4` then migrated the ignored corpus from a verified same-volume rollback copy.
Exactly 392 of 978 files changed: 262,996 retired-root values became relative and 157 origin labels were added;
no file was added or removed. The migrated tree has 721,679,372 logical bytes, content fingerprint
`f07b773373f3e13b3b21f558c5ed51eae2e545f9241c50c7b4603d2890bb4f1c`, and zero retired-root values.
The 82 remaining absolute path values are explicitly labeled external source-library provenance.

The locality doctrine now runs a self-tested fail-closed oracle over every JSON artifact present below
`generated/` and pins the producer seams. Any path/root-valued absolute string outside the narrow labeled
SourceIR/EvidenceIR external-provenance allowance fails the gate. The pre-migration 335-file count is historical
evidence, not the current corpus state.
