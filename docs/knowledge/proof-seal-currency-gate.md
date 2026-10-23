---
id: proof-seal-currency-gate
title: The persisted corpus's proof seal is censused totally and probed read-only at gate tier
answers:
  - "what reports that the persisted corpus is out of seal"
  - "which doctrine checks the persisted proof seal on every commit"
  - "how do I check whether the persisted corpus seal is stale"
  - "why does a source edit un-seal every persisted artifact"
  - "how can a script ask the SpecForge canonical loader without mutating the artifact"
  - "why must a gate never probe persisted artifacts with specforge validate"
  - "which stage has no read-only canonical probe"
  - "what does PROOF-SEAL-CURRENCY prove and not prove"
  - "how is the proof seal read from a large stage artifact"
  - "how expensive is reading the proof seal from the whole persisted corpus"
  - "what is the difference between PROOF-SEAL-CURRENCY and CHAIN-CURRENCY"
  - "where is the proof-seal reader defined (cargo xtask check proof-seal-currency)"
date: 2026-08-29
status: current
tags: [doctrine, proof, seal, chain-currency, corpus, gate, read-only]
evidence: scripts/check_proof_seal_currency.sh; xtask/src/checks/proof_seal_currency.rs; scripts/lib/proof_seal_scan.sh; docs/tasks/RUSTDOC-AND-XTASK.md (.14.e); scripts/check_doctrines.sh; docs/tasks/SOURCE-IR-REPRODUCIBILITY.md (.14, .15, .16); DOCTRINE_ENFORCEMENT.md §10
reverify: bash scripts/check_proof_seal_currency.sh --self-test && bash scripts/check_proof_seal_currency.sh && bash scripts/rebuild_stage_cascade.sh --self-test
---

Every persisted stage artifact records the `ruleset_sha256` of the rule registry that proved it, and every
downstream stage records the cumulative digest of its upstream's ruleset folded with its own
(`cumulative_ruleset_sha256`, `crates/specforge/src/ir/derivation.rs`). That digest is generated at build time
from each stage's rule-registry constructor and its stage-local production items, so an ordinary edit to a stage
root un-seals the entire persisted corpus while every artifact's content stays exactly right. `SOURCE-IR-REPRODUCIBILITY.14`
found the corpus in that state after 13 days and 54 commits: nothing an ordinary slice ran could observe the
transition, because `CHAIN-CURRENCY` reports it only at CI tier.

`PROOF-SEAL-CURRENCY` (`scripts/check_proof_seal_currency.sh`, registered `gate` in `scripts/check_doctrines.sh`)
closes that latency. It censuses the recorded seal of every persisted artifact at all five chain stages for the
whole proof-carrying stratum, then asks the current build's own canonical loader whether it still accepts that
seal. The census is total and the probe is representative — one per *distinct* seal per stage — so
representativeness is a measured consequence of the census rather than an assumption: a per-document divergence
raises the distinct count and earns its own probe. A tree with no `generated/` skips loudly and passes.

**Never probe a persisted artifact with `specforge validate`.** It is not idempotent: each call appends a
`validation_backannotation` mutation and moves the artifact's digest, and every downstream stage retains its
upstream ledger as an exact prefix, so validating an upstream invalidates everything built below it. The
read-only way to reach the same verified-load path is to run the *consuming* stage with `--dry-run`, measured to
leave the artifact byte-identical. The terminal `adapters/isf` stage has no consumer and therefore no read-only
canonical probe at all — and `CHAIN-CURRENCY` does not close that gap either, because its content comparison
excludes the proof surface by construction. That stage is censused and reported as unprobed.

Reading the seal is the composition of two predicates, native since `RUSTDOC-AND-XTASK.14.e` in
`cargo xtask check proof-seal-currency` and reached through `scripts/lib/proof_seal_scan.sh`, which the remedy
(`scripts/rebuild_stage_cascade.sh`) shares so a gate and a remedy cannot disagree about scope; the stage table
and the read-only probe stay defined in that library. A necessary-condition prefilter (a search for the
`"proof_ledger"` key, one process for a whole list) runs first, and the exact depth-aware streaming scan then
decides each candidate. For the Perl reader that measured 0.24 s and 1.23 s (`2026-08-29`, 24 sealed); the
composition over all 78 `source_ir.json` is 0.30 s native against 2.12 s for the Perl library (`2026-10-24`,
31 sealed, identical rows). Proving absence still reads every byte — what the prefilter removes is
the parser, not the read. The prefilter is never an authority: a nested `proof_ledger` is a candidate the exact
scan rejects.

The boundary matters: a current seal says the persisted proof is one today's build issues. It says nothing about
whether an artifact is still the content today's binary would reproduce — that stays `CHAIN-CURRENCY`'s question
at CI tier (ADR 0025 decision 2). See [[validate-explicit-artifact-path-contained-backannotation]] and
[[reviewed-fixture-projection-digest-lockstep]].
