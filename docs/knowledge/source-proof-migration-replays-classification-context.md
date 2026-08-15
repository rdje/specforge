---
id: source-proof-migration-replays-classification-context
title: Source proof migration replays neutral classification context instead of blessing stale labels
answers:
  - "how does SourceIR proof migration handle a classifier implementation change"
  - "does source proof migration rerun Docling"
  - "why did proof-only SourceIR refresh fail after a table classifier change"
  - "how is SourceIR validation updated during classifier migration"
date: 2026-08-15
status: current
tags: [sourceir, proof-migration, classification, validation, chain-currency]
evidence: crates/specforge/src/ir/source.rs; crates/specforge/examples/source_proof_migrate.rs; docs/research/reviewed-population-clean-replay-diagnostic.md
reverify: "cargo test --offline -p specforge-core --features source-proof-migration retained_capture_migration_ -- --nocapture"
---

The feature-gated SourceIR migrator is an audited corpus-maintenance seam. For current schema-3 artifacts it
verifies the retained source/bundle backing, reads the exact neutral table/visual/section capture from the proof
context, executes the current generic classifier, reapplies the recorded grounded proposals, and reconstructs
the current proof. It does not rerun Docling and does not treat the stale classified conclusion as authority.

When classifier output changes, the previous validation report is also stale. Migration first proves the rebuilt
artifact with an empty validation surface, computes the registered current validation report, and applies that
report through the ordinary proof-carrying mutation path. Ordinary canonical loaders remain fail-closed on stale
proof; only builds with the explicit `source-proof-migration` feature expose this reconstruction capability.
