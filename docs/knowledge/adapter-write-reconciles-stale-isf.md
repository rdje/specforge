---
id: adapter-write-reconciles-stale-isf
title: Successful adapter writes reconcile obsolete generated ISF siblings instead of retaining stale actors
answers:
  - "does specforge adapt remove an old actor isf when actor selection changes"
  - "why did channel.isf and setportfeature_port_over_current.isf coexist after rebuilding USB 3.2"
  - "what does AdapterArtifact write_to_disk reconcile"
  - "does a blocked adapter remove a previously emitted isf"
  - "does adapter output reconciliation delete unrelated files or directories"
  - "how are stale generated isf symlinks handled"
date: 2026-08-09
tags: [adapter, isf, generated-artifacts, stale-output, convergence, artifact-lifecycle, corpus-coverage]
evidence: crates/specforge/src/ir/adapters.rs (AdapterArtifact::write_to_disk; reconcile_emitted_isf_files; adapter_write_reconciles_obsolete_isf_files); docs/tasks/CORPUS-COVERAGE.md (.2.33d.iv.a); generated/adapters/isf/usb_3_2_revision_1_0_2017_09
reverify: "Build the release binary; place an obsolete regular or symlink *.isf beside a document adapter; rerun specforge adapt --target isf <intent_ir.json>; expect adapter.json plus only its emitted_target_path, with unrelated files/directories preserved. Run cargo test -p specforge ir::adapters::tests::adapter_write_reconciles_obsolete_isf_files -- --exact."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.33d.iv.a`).** A successful adapter write is the
authoritative materialization boundary for one document. Before this leaf, `write_to_disk` overwrote
`adapter.json` and the currently selected actor file but did not inspect sibling outputs. If actor selection
changed, both the new and old `.isf` remained visible even though only the new path appeared in the manifest.

The real USB 3.2 rebuild reproduced the defect exactly: the repaired IntentIR selected `channel`, but
`setportfeature_port_over_current.isf` from the semantically false prior model survived beside
`channel.isf`. The old file was residue only; it did not appear in current `adapter.json` or current rendered
source.

After writing the current manifest and optional target, `reconcile_emitted_isf_files` now scans only that
resolved document artifact root. It retains the manifest-selected target, removes other regular or symlink
leaf files whose extension is exactly `.isf`, and ignores unrelated files and directories. A blocked adapter
has no selected target, so a successful renderable-to-blocked transition removes every former `.isf` while
leaving `adapter.json` and unrelated material intact. No actor, document, vendor, or signal vocabulary enters
the rule.

This is output convergence, not semantic validation. The same USB rerun also exposed a separate upstream
authority-empty interface fallback that produces hundreds of low-confidence ports. That blocker belongs to
`CORPUS-COVERAGE.2.33d.iv.b`; eliminating stale files does not make their replacement trustworthy.
