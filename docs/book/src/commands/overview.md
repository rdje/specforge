# Commands

This chapter is the operational map of the CLI.

Current surface:

- `inspect <path>`
- `doctor [--strict]`
- `converge <source> [--target isf] [--max-iterations 8] [--vlm-provider ollama|open-ai|lm-studio|skip] [--vlm-model <name>] [--nlp-provider ollama|open-ai|lm-studio|skip] [--nlp-model <name>] [--nlp-max-sentences 0] [--prior-memory <path>] [--rescan-plan <plan>] [--execute-rescan-plan] [--rescan-plan-limit 0]` — full flag/default reference in the [pipeline chapter](pipeline.md#converge)
- `ingest <source> [--dry-run]`
- `evidence <source-ir> [--prior-memory <path>] [--dry-run]`
- `semantic <evidence-ir> [--dry-run]`
- `intent <semantic-ir> [--dry-run]`
- `adapt <intent-ir> --target isf [--dry-run]`
- `enrich <source-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--classify-only] [--dry-run]`
- `nlp-enrich <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--dry-run]`
- `extract-contracts <evidence-ir> [--provider ollama|open-ai|lm-studio|skip] [--model <name>] [--dry-run] [--max-statements 0]`
- `signal-resolve <evidence-ir> [--provider ollama|open-ai|lm-studio|skip] [--model <name>] [--dry-run] [--max-statements 0] [--grounding-signals <csv>]`
- `validate <artifact>`
- `kg-bench`
- `project-validation <artifact>... [--rescan-vlm-provider auto-local|ollama|lm-studio|skip]`
- `rescan-plan [--plan <plan>] [--execute] [--limit <n>] [--document-key <key>]`
- `learn-priors <intent-ir>... [--dry-run]`
- `corpus-kb [validation-report]... [--kg-fixtures-root <fixture-root>] [--kg-fixture <fixture>]`
- `clean [--generated-root <root>] [--scope source-normalized|document|all-generated] [--document-key <key>] [--execute]`

Only `--target isf` materializes an adapter artifact.
`.fsm` and HDL lowering (SystemVerilog/Verilog/VHDL) are out of scope — FSMGen consumes `.isf` and owns scheduling, `.fsm`, and HDL downstream.

The commands fall into three groups:

1. pipeline execution
2. quality and validation
3. cross-document learning

## The everyday command paths

If you want one end-to-end run:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf
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

If you want to reclaim rebuildable generated artifacts before they eat your disk:

```bash
cargo run --manifest-path Cargo.toml -- clean
cargo run --manifest-path Cargo.toml -- clean --execute
```

If you want to inspect the targeted rescan queue:

```bash
cargo run --manifest-path Cargo.toml -- rescan-plan
```

If you want the stabilized convergence loop to inspect that same queue:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf --rescan-plan generated/validation/rescan_plan.json
```

If you want to update the cross-document prior store:

```bash
cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/.../intent_ir.json
```

If you want to refresh the tracked corpus knowledge-base synthesis pages:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb generated/intent_ir/.../validation_report.json
cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality
```

The next two chapters break those paths down in more detail.
