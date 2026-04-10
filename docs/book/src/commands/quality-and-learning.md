# Quality, Validation, And Learning Commands

These commands are what make `specforge` more than a one-shot extractor.

This page is the operational command map.
For the deeper rationale behind validation, fixture truthfulness, and cross-document learning, continue with:

- [Validation And Learning](../quality/validation.md)
- [KG Bench And Fixtures](../quality/kg-bench.md)
- [Corpus Memory And Priors](../quality/corpus-memory.md)

## `validate`

```bash
cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/<document_key>/intent_ir.json
```

`validate` writes a deterministic stage-local `validation_report.json` and backannotates the artifact.

It reports things like:

- direction coverage
- graph direction coverage
- width coverage
- visual classification observations
- negative-knowledge prior matches
- temporal conflicts
- semantic conflicts
- residual decisions
- overall score

## `project-validation`

```bash
cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/.../intent_ir.json
```

This command validates the passed artifacts and refreshes the tracked crash-safe snapshot docs, especially:

- `VALIDATION_SNAPSHOT.md`
- the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`

It also consumes validation-level negative-knowledge rescan guidance and writes a local generated extractor-selection target list:

`generated/validation/rescan_plan.json`

That plan is deliberately advisory.
It tells downstream loops which current conflict or residual ids deserve targeted rechecking and stronger local corroboration, but it does not mutate IR, suppress findings, or promote facts from prior memory.

Use it when the live baseline should be updated, not just an individual artifact.

## `kg-bench`

```bash
cargo run --manifest-path Cargo.toml -- kg-bench
```

This runs the tracked truthfulness fixture set under `crates/specforge/test_data/kg_quality/`.

The benchmark surface exists to lock:

- gold paths
- negative expectations
- conflict surfacing
- residual quality
- prior-guided before/after behavior

It is the repo’s main extraction-truthfulness regression harness.

## `learn-priors`

```bash
cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/.../intent_ir.json
```

This updates the cross-document learning plane.

The main output is:

`generated/prior_memory/corpus_memory.json`

What it learns today:

- actor-taxonomy priors
- semantic phrase priors
- semantic modality-reliability priors
- temporal phrase priors
- table-shape priors
- visual-motif priors
- negative-knowledge priors

The safety rule is critical:

- priors widen interpretation of the current document
- priors do not directly author canonical truth

Visual-motif priors remember recurring visual patterns, while negative-knowledge priors remember conflict and residual archetypes that should make future extraction more careful.
Visual-motif priors now have a first bounded consumer: `EvidenceIR` may add a prior-memory classification observation for a current unknown visual asset when its local caption matches a unique learned motif.
Negative-knowledge priors now have bounded validation consumers too: `EvidenceIR` may surface caution for repeated local signal-semantic conflict patterns, while `SemanticIR` and `IntentIR` may surface caution for repeated carried conflict and residual packet patterns.
That caution does not suppress evidence, remove residuals, or decide semantic truth.

## `enrich` and `nlp-enrich`

These are narrower enrichment entrypoints used when you want to operate on the staged pipeline more manually.

They are still valuable, but `converge` is the preferred user-facing path when you want the full loop.
