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
- `nli-verify <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>]`
- `entity-type <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-subjects 0]`
- `extract-conditions <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-constraints 0]`
- `extract-constraints-llm <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--model <name>] [--max-sentences 0]`
- `eval-extraction <dataset> [--provider ollama|open-ai|lm-studio|skip] [--model <name>] [--evidence-root <root>]`
- `audit-extraction <source-ir> [--provider ollama|open-ai|lm-studio|skip] [--model <name>] [--sample 12] [--seed 0]`
- `grits-consensus <witnesses-json> [--min-agree 2] [--adjudicate-out <queue>]`
- `recover-register-bits <evidence-ir> [--vlm-provider ollama|open-ai|lm-studio|skip] [--vlm-model <name>] [--dry-run]`
- `validate <artifact>`
- `kg-bench`
- `project-validation <artifact>... [--rescan-vlm-provider auto-local|ollama|lm-studio|skip]`
- `rescan-plan [--plan <plan>] [--execute] [--limit <n>] [--document-key <key>]`
- `learn-priors <intent-ir>... [--dry-run]`
- `corpus-cluster [--evidence-root <root>] [--threshold <0.0-1.0>]`
- `corpus-kb [validation-report]... [--validation-snapshot <reviewed-snapshot>] [--kg-fixtures-root <fixture-root>] [--kg-fixture <fixture>]`
- `clean [--generated-root <root>] [--scope source-normalized|document|all-generated] [--document-key <key>] [--execute]`

Only `--target isf` materializes an adapter artifact.
`.fsm` and HDL lowering (SystemVerilog/Verilog/VHDL) are out of scope — FSMGen consumes `.isf` and owns scheduling, `.fsm`, and HDL downstream.

The commands fall into three groups:

1. pipeline execution
2. quality and validation
3. cross-document learning

The LLM-backed extraction and judge commands (`extract-contracts`,
`signal-resolve`, `nli-verify`, `entity-type`, `extract-conditions`,
`extract-constraints-llm`) and the extraction-quality measurement commands
(`grits-consensus`, `eval-extraction`, `audit-extraction`) are each detailed in
[Quality, Validation, And Learning](quality-and-learning.md) and
[Extraction Eval](../quality/extraction-eval.md). Each one can run as a CI-safe
no-op: the extraction/judge commands default to a live local provider
(`--vlm-provider`/`--provider ollama`) and no-op when passed `skip`, while
`eval-extraction` and `audit-extraction` default to `skip` (the deterministic
baseline / plan-only mode) and `grits-consensus` is a fully offline measurement
over pre-collected witnesses — so the deterministic pipeline never depends on a
live model.

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

This refresh mutates the selected artifacts, local rescan plan, snapshot, and live-status projection.
Review the result before committing it: the tracked files represent the last reviewed validation
boundary, not automatically the newest git-ignored artifact state.

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

If you want to see which ingested documents form structural families (the basis for cross-document pattern reuse):

```bash
cargo run --manifest-path Cargo.toml -- corpus-cluster
```

If you want to refresh the tracked corpus knowledge-base synthesis pages:

```bash
cargo run --manifest-path Cargo.toml -- corpus-kb --validation-snapshot VALIDATION_SNAPSHOT.md
cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality
```

The next two chapters break those paths down in more detail.
