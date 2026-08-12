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
- `benchmarks/kg-fixtures.md`: auto-refreshable KG fixture-result and structural-capability summary projection plus human synthesis notes.
- `patterns/kg-fixtures.md`: auto-refreshable semantic/truthfulness structural-capability projection plus human synthesis notes.
- `prior_memory/kg-fixtures.md`: auto-refreshable typed prior-memory capability projection plus human synthesis notes.
- `tables/kg-fixtures.md`: auto-refreshable table-extraction capability projection plus human synthesis notes.
- `visuals/kg-fixtures.md`: auto-refreshable visual-evidence capability projection plus human synthesis notes.
- `state_machines/kg-fixtures.md`: auto-refreshable state-machine capability projection plus human synthesis notes.
- `timing/kg-fixtures.md`: auto-refreshable temporal-semantics capability projection plus human synthesis notes.
- `infra/kg-fixtures.md`: auto-refreshable infrastructure/polarity capability projection plus human synthesis notes.
- `prior_candidates/kg-fixture-candidates.md`: auto-refreshable review-only prior-candidate projection, readiness summary, family-level gate matrix, and human synthesis notes.
- `prior_candidates/kg-fixture-candidates.json`: auto-refreshable schema-versioned prior-candidate readiness manifest for review automation; it is not a `CorpusMemory` artifact or approval record.

Future page families can include:

- `prior_memory/`
- `tables/`
- `visuals/`
- `state_machines/`
- `timing/`
- `infra/`
- `failures/`
- `benchmarks/`

## Refresh

The validation-finding refresh surface is:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb \
  --repo-root . \
  generated/intent_ir/<document-key>/validation_report.json
```

The KG fixture-result refresh surface also refreshes the dedicated structural-capability and prior-candidate pages listed above:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb \
  --kg-fixtures-root crates/specforge/test_data/kg_quality
```

The command updates only managed blocks and preserves human-authored synthesis around them.
