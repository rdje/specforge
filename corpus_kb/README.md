# Corpus Knowledge Base

This directory is the tracked `R15g` corpus knowledge base plane.

It is separate from both:

- per-document canonical truth in `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`
- machine-usable typed priors in `generated/prior_memory/corpus_memory.json`

The corpus knowledge base exists for durable, reviewable cross-document synthesis:

- recurring extraction motifs
- validation and conflict patterns
- negative-knowledge/failure archetypes
- table, timing, visual, and infrastructure families
- candidate ideas for future typed prior-memory harvesters or KG-bench fixtures

It must not directly author canonical IR facts.
It can inform humans, future LLM sessions, benchmark design, rescan targeting, and prior-candidate design, but machine-usable promotion still belongs in typed schemas plus validation gates.

## Page Families

- `failures/validation-findings.md`: auto-refreshable validation finding projection plus human synthesis notes.
- `benchmarks/kg-fixtures.md`: auto-refreshable KG fixture-result and fixture-family summary projection plus human synthesis notes.
- `patterns/kg-fixtures.md`: auto-refreshable semantic/truthfulness pattern fixture-family projection plus human synthesis notes.
- `tables/kg-fixtures.md`: auto-refreshable table extraction fixture-family projection plus human synthesis notes.
- `visuals/kg-fixtures.md`: auto-refreshable visual/VLM fixture-family projection plus human synthesis notes.
- `timing/kg-fixtures.md`: auto-refreshable timing motif fixture-family projection plus human synthesis notes.
- `infra/kg-fixtures.md`: auto-refreshable infrastructure/polarity fixture-family projection plus human synthesis notes.
- `protocols/amba-kg-fixtures.md`: auto-refreshable AMBA-family fixture projection plus human synthesis notes.
- `prior_candidates/kg-fixture-candidates.md`: auto-refreshable review-only prior-candidate projection, family-level gate matrix, and human synthesis notes.

Future page families can include:

- `protocols/`
- `tables/`
- `visuals/`
- `timing/`
- `infra/`
- `failures/`
- `benchmarks/`

## Refresh

The validation-finding refresh surface is:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb \
  --repo-root . \
  generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/validation_report.json \
  generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/validation_report.json \
  generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/validation_report.json \
  generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/validation_report.json
```

The KG fixture-result refresh surface also refreshes the dedicated pattern, family, and prior-candidate pages listed above:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb \
  --kg-fixtures-root crates/specforge/test_data/kg_quality
```

The command updates only managed blocks and preserves human-authored synthesis around them.
