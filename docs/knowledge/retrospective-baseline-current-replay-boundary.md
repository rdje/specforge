---
id: retrospective-baseline-current-replay-boundary
title: Frozen stage artifacts are retrospective baselines until the current binary replays them
answers:
  - "is the frozen source to intent result current binary output"
  - "why did the AIA table of contents have 19 timing false positives"
  - "does the current SpecForge binary still fabricate AIA TOC timing constraints"
  - "what did SPEC-TO-INTENT-ALIGNMENT.6a prove"
  - "how do I replay a source through SourceIR EvidenceIR SemanticIR and IntentIR without overwriting generated artifacts"
  - "where is current binary replay evidence stored"
  - "why is current binary replay coverage 1 of 12"
date: 2026-08-12
status: current
tags: [spec-to-intent-alignment, artifact-currency, replay, timing-table, provenance, locality]
evidence: crates/specforge/test_data/trajectory/replays/aia_toc_current_binary_replay.json; crates/specforge/src/ir/source_to_intent_replay.rs; crates/specforge/src/ir/trajectory_snapshot.rs; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6a)
reverify: "cargo test -p specforge --lib ir::source_to_intent_replay && cargo test -p specforge --lib ir::trajectory_snapshot && cargo run --quiet -p specforge --example trajectory_snapshot -- --check"
---

The `.4c` reviewed report is a hash-pinned retrospective observation of selected stage artifacts. Selection was
frozen after commit `46af2eca` introduced structural timing-table authority, but persisted documents were not
automatically rebuilt. A selected stage hash therefore proves exactly which bytes reviewers scored; it does not
prove those bytes are what the later/current binary would produce.

The dominant example is RISC-V AIA `table_0004`. Both artifacts contain the same complete reviewed 20-row ×
two-column table of contents and expect zero canonical facts. The frozen SourceIR classifies it
`timing_parameter`, and EvidenceIR, SemanticIR, and IntentIR each carry 19 unprovenanced false timing records.
The hash-equal 827,669-byte source replay classifies the same table `unknown` and all three promoted stages carry
zero timing records. True positives remain 0→0. Thus current code already removes 19/41 frozen fabrications and
19/45 frozen provenance failures; no production extractor patch belongs to this family.

`ir::source_to_intent_replay` provides the generic isolated four-stage harness. Inputs must be repository-
relative, resolve to contained files, and write below a fresh `.project-data/tmp` child. Absolute/traversal paths,
symlink escapes, existing roots, and canonical `generated/` mutation fail closed. External same-SSD authority is
copied/size-hash-verified onto the repository volume first, and all exact scratch roots are removed after stage
hashes and counts enter the tracked evidence report.

Controller v2 used this evidence to gate operational currentness at 1/12 and name 11 unreplayed documents. That
intermediate state is now superseded by `[[qualified-current-source-to-intent-result]]`: the same isolated method
covers all 12 documents and confirms which non-AIA failures remain. The frozen 41/45/33 counts remain diagnostic
baseline measures, never current product authority.
