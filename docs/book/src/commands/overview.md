# Commands

This chapter is the operational map of the CLI.

Current surface:

- `inspect <path>`
- `doctor [--strict]`
- `converge <source> --target fsm`
- `ingest <source>`
- `evidence <source-ir>`
- `semantic <evidence-ir>`
- `intent <semantic-ir>`
- `adapt <intent-ir> --target fsm`
- `enrich <source-ir>`
- `nlp-enrich <evidence-ir>`
- `validate <artifact>`
- `kg-bench`
- `project-validation <artifact>...`
- `learn-priors <intent-ir>...`

The commands fall into three groups:

1. pipeline execution
2. quality and validation
3. cross-document learning

## The everyday command paths

If you want one end-to-end run:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm
```

If you want stage-by-stage visibility:

```bash
cargo run --manifest-path Cargo.toml -- ingest /path/to/spec.pdf
cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/<document_key>/source_ir.json
cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/<document_key>/evidence_ir.json
cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/<document_key>/semantic_ir.json
cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/<document_key>/intent_ir.json
```

If you want to benchmark extraction truthfulness:

```bash
cargo run --manifest-path Cargo.toml -- kg-bench
```

If you want to refresh the tracked validation snapshot:

```bash
cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/.../intent_ir.json
```

If you want to update the cross-document prior store:

```bash
cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/.../intent_ir.json
```

The next two chapters break those paths down in more detail.

