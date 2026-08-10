---
id: chain-currency-doctrine
title: CHAIN-CURRENCY replays every persisted corpus artifact against the current binary
answers:
  - "how does SpecForge prove a persisted corpus artifact is still what the current binary produces"
  - "what does scripts/check_chain_currency.sh check"
  - "why does the chain-currency check ignore validation_reports"
  - "which corpus stages are measurable without re-ingesting a document"
  - "what makes a corpus document unmeasurable for chain currency"
  - "how is a CI-tier doctrine registered without slowing down the pre-commit hook"
  - "what does DEFER mean in the doctrine enforcement report"
date: 2026-08-10
status: current
tags: [doctrine-enforcement, corpus, currency, artifacts, ci]
evidence: scripts/check_chain_currency.sh; scripts/check_doctrines.sh; docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md; docs/tasks/CORPUS-CHAIN-CURRENCY.md
reverify: bash scripts/check_chain_currency.sh --self-test
---

`CHAIN-CURRENCY` (`scripts/check_chain_currency.sh`) is the registered oracle for ADR 0025: a persisted
corpus artifact must be exactly what the current binary reproduces from its persisted input. It replays
`evidence`, `semantic`, `intent`, and `adapt --target isf` with `--dry-run` — writing nothing — and
compares each result with the stored artifact, then compares each emitted `.isf` file with the replayed
adapter's rendered `isf.source_text` and rejects any unlicensed or missing sibling, mirroring
`AdapterArtifact::write_to_disk` plus `reconcile_emitted_isf_files`.

Each stage replays from the **persisted** upstream artifact, not the replayed one. That composes: once a
stage is proven current, both inputs are the same artifact; when a stage is stale the check already fails
there while every later stage still answers its own question exactly.

Content identity excludes `validation_reports`. `specforge validate` back-annotates that section *after*
the stage has run, and the product's own `source_ir_fingerprint` / `evidence_ir_fingerprint` /
`semantic_ir_fingerprint` / `intent_ir_fingerprint` / `isf_adapter_fingerprint` helpers in
`crates/specforge/src/commands/validate.rs` clear the same field before hashing. The check adopts the
code's identity rule rather than inventing one.

**Only the evidence stage needs a normalized markdown bundle.** `EvidenceIr::build_with_prior_memory`
resolves `normalization_plan.promoted_markdown_path`, so a document whose bundle was reclaimed cannot be
replayed there and is reported as an explicit *unmeasurable* count — never as a pass. `SemanticIr::build`,
`IntentIr::build`, and `AdapterArtifact::build` each read only the persisted upstream JSON, so the whole
downstream chain is measurable — and rebuildable — for the entire corpus without any re-ingest.

An absent `generated/` skips loudly and exits 0: a fresh clone and a hosted CI runner have no corpus, and
silence would read as a pass. `--self-test` proves the comparison core fail-closed in ten cases
(identity, the `validation_reports` exclusion, key-order independence, shrunken and vanished sections,
the four `.isf` emission outcomes, and the end-to-end absent-corpus skip) before any PASS is trusted.

A full corpus run is **measured at about seven minutes** (80 documents, debug binary, `2026-08-10`) plus
a cargo freshness build, so it is the registry's first **CI-tier** doctrine
(`DOCTRINE_ENFORCEMENT.md` §4.7). `scripts/check_doctrines.sh` entries carry a
tier: the default run executes `gate`-tier doctrines and prints each CI-tier one as `DEFER`, so a
deferred rule can never read as an absent rule; `--all` runs every tier and is what `scripts/run_ci.sh`
invokes. Deferred entries are meta-checked for existence and executability like any other.

See [[persisted-chain-currency-is-measured-not-assumed]].
