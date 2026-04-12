# Corpus Knowledge Base

The corpus knowledge base is the public-facing explanation for `R15g`.

It is a tracked, reviewable synthesis plane under `corpus_kb/`.
It sits beside, but does not replace, the per-document IR pipeline or the typed prior store.

## Why it exists

Some cross-document knowledge is useful but should not become canonical document truth by itself.

Examples include:

- recurring validation finding patterns
- repeatable KG fixture-result patterns
- extractor failure archetypes
- negative-knowledge patterns
- table, timing, visual, and infrastructure families
- ideas for future KG-bench fixtures
- candidate priors that still need typed schemas and validation gates

That synthesis needs a durable home.
It should not be hidden in chat history, and it should not be forced into `IntentIR` or `CorpusMemory` before it is ready.

## The three-plane boundary

`specforge` keeps three planes separate:

- per-document truth: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`
- typed reusable priors: `generated/prior_memory/corpus_memory.json`
- corpus synthesis: `corpus_kb/`

The corpus KB can inform humans, future LLM sessions, benchmark design, and prior-candidate design.
It cannot directly author canonical IR facts.

## Refreshable pages

The first refreshable page families are:

- `corpus_kb/failures/validation-findings.md`
- `corpus_kb/benchmarks/kg-fixtures.md`

The validation page is refreshed from validation report sidecars:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb \
  --repo-root . \
  generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/validation_report.json \
  generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/validation_report.json \
  generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/validation_report.json \
  generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/validation_report.json
```

The benchmark page is refreshed by running the tracked KG fixture suite:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb \
  --kg-fixtures-root crates/specforge/test_data/kg_quality
```

The command updates only managed blocks.
Human-authored synthesis outside those blocks is preserved.

The KG fixture projection records fixture paths plus pass/fail status.
It is a review surface, not a replacement for the executable `specforge kg-bench` gate.
Fixture-local validation runs quietly during this projection, so the command output remains a concise corpus-KB refresh summary.

## Promotion rule

Corpus KB content is never self-promoting.

If a corpus observation should become machine-usable, it must move through a stricter surface:

- a typed prior family in `CorpusMemory`
- a KG-bench fixture proving safe behavior
- a validation or rescan hint that still requires current-document evidence
- a future explicit approval artifact if canonical IR mutation is ever introduced

Until then, the corpus KB is reviewable guidance, not truth promotion.
