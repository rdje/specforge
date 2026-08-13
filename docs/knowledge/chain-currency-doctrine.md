---
id: chain-currency-doctrine
title: CHAIN-CURRENCY replays every persisted corpus artifact against the current binary
answers:
  - "how does SpecForge prove a persisted corpus artifact is still what the current binary produces"
  - "what does scripts/check_chain_currency.sh check"
  - "why does the chain-currency check ignore validation_reports"
  - "which corpus stages are measurable without re-ingesting a document"
  - "what makes a corpus document unmeasurable for chain currency"
  - "how does chain currency distinguish a checked blocked adapter from an emitted ISF file"
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
code's identity rule rather than inventing one. For proof-carrying stages, the proof bytes derived from that
typed validation mutation belong to the same exclusion; canonical loaders still execute current proof before
downstream use.

**Only the evidence stage needs a normalized markdown bundle for content replay.** `EvidenceIr::build_with_prior_memory`
resolves `normalization_plan.promoted_markdown_path`, so a document whose bundle was reclaimed cannot be
replayed there and is reported as an explicit *unmeasurable* count — never as a pass. `SemanticIr::build`,
`IntentIr::build`, and `AdapterArtifact::build` read their persisted upstream JSON, but current canonical builds
also require that upstream artifact's verified cumulative proof. A legacy/proofless frontier therefore remains
explicitly unmeasurable at every later proof-gated stage instead of being mistaken for stage-local currency.

Adapter output has two equally checkable states. A renderable manifest names an emitted target whose bytes must
equal replayed `isf.source_text`; a blocked manifest names no target and requires every obsolete sibling `.isf`
to be absent. The summary tracks checked adapter states, actual emitted files, and blocked/no-file states as
separate counts and rejects impossible arithmetic. This corrected an old reporting label that described one
checked state per document as one emitted file even when every state was blocked; the state/file comparison itself
was already exact.

An absent `generated/` skips loudly and exits 0: a fresh clone and a hosted CI runner have no corpus, and
silence would read as a pass. `--self-test` currently proves 22 controlled comparison, arithmetic, absent-corpus,
and retention outcomes before any PASS is trusted, including emitted/no-file state counting and impossible
adapter summary counts.

Its second leg checks *which documents are measurable at all*: the retained normalized bundles must be
exactly the set declared in `doctrine/chain_currency/retained_bundles.json`, so neither a silent
reclamation nor an unrecorded retention survives. See [[normalized-bundle-retention-is-declared]].

A full corpus run is **measured at about seven minutes** (80 documents, debug binary, `2026-08-10`) plus
a cargo freshness build, so it is the registry's first **CI-tier** doctrine
(`DOCTRINE_ENFORCEMENT.md` §4.7). `scripts/check_doctrines.sh` entries carry a
tier: the default run executes `gate`-tier doctrines and prints each CI-tier one as `DEFER`, so a
deferred rule can never read as an absent rule; `--all` runs every tier and is what `scripts/run_ci.sh`
invokes. Deferred entries are meta-checked for existence and executability like any other.

See [[persisted-chain-currency-is-measured-not-assumed]].
