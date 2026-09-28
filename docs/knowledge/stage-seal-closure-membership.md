---
id: stage-seal-closure-membership
title: Only the items a stage's rule registry reaches are sealed, so an edit re-seals the corpus exactly when it touches that closure
answers:
  - "will editing this function re-seal the persisted corpus"
  - "which functions does the SourceIR proof seal cover"
  - "how do I list a stage's production seal closure"
  - "why did every chain report SourceIR proof verification failed proof ledger ruleset hash is stale after a source.rs edit"
  - "why does an evidence.rs or semantic.rs production edit not un-seal the corpus"
  - "does a doc comment change move the production seal"
  - "how do I avoid re-sealing all 31 chains for a behaviour-neutral refactor"
date: 2026-09-28
status: current
tags: [proof, seal, chain-currency, production-digest, source-ir, operations]
evidence: crates/specforge-core/build.rs (STAGE_ROOTS, SemanticIndex::closure, attribute_is_nonsemantic); target/debug/build/specforge-core-*/out/production_semantic_digests.rs (<STAGE>_PRODUCTION_SEMANTIC_ITEMS); docs/tasks/extraction-representation-gaps/program-and-census.md (.3.f)
reverify: "cargo build --offline, then awk '/SOURCE_PRODUCTION_SEMANTIC_ITEMS/,/\\];/' on the newest target/debug/build/specforge-core-*/out/production_semantic_digests.rs and expect the fn:timing_* classifier helpers (timing_structural_header_columns, timing_table_has_structural_authority, …) and no fn:timing_table_columns or fn:timing_table_layouts; the EVIDENCE, SEMANTIC, INTENT and ADAPTER lists hold only their registry, rule-field constants and use items"
---

`crates/specforge-core/build.rs` seals each stage with the digest of one **closure**: starting from the stage's
rule-registry constructor (`source_rule_registry`, `evidence_rule_registry`, …) in that stage's root file, it
follows identifiers to every item of the same file they name, and hashes the tokens of that set with the
derivation kernel. Doc comments and lint attributes are stripped first, so prose never moves a seal; token changes
to a closure item always do. Every persisted artifact records its stage's digest, and each downstream stage folds
in its upstream's, so a moved SourceIR digest un-seals every stage of every proof-carrying chain at once.

The closures are very unequal. On `2026-09-28` the EVIDENCE, SEMANTIC and ADAPTER closures held 16 items each
and INTENT 17: the registry, its relation verifier, the rule-field constants and 13 `use` items; SOURCE held 69.
The extractors themselves are outside, which is why an ordinary `evidence.rs` or `semantic.rs` change moves
content but not the seal. The SOURCE closure is different: it reaches SourceIR's classification, including the timing classifier
helpers (`timing_structural_header_columns`, `timing_table_has_structural_authority`, `timing_columns_have_context`,
`timing_header_cell_is_identity`, `timing_header_words`, `timing_text_has_word`). `timing_table_columns` and the
per-condition reader `timing_table_layouts` are called only by EvidenceIR and are outside.

**Measured instance (`EXTRACTION-REPRESENTATION-GAPS.3.f`).** A first draft refactored
`timing_structural_header_columns` to share a new row helper with the per-condition reader. SourceIR behaviour was
identical, but the helper joined the closure and `check_chain_currency.sh` failed all 31 chains at every stage (124
refusals, each `SourceIR proof verification failed: proof ledger ruleset hash is stale`). Restoring the function's
tokens restored the HEAD digest, and the new reader repeats the row rule instead of sharing it, with a comment
saying why.

**How to use it.** Before editing `source.rs`, list the closure (the `reverify` command). If the edit touches a
listed item and changes no behaviour, keep that item's tokens; if it does change SourceIR behaviour, the re-seal is
the designed cost and `scripts/rebuild_stage_cascade.sh --write` is its remedy for content-identical chains
([[proof-seal-currency-gate]], [[retained-chain-rebuild-order]]).
