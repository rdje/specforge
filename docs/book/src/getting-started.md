# Getting Started

This chapter is the shortest path from clone to useful output.

## What you need

- Rust `1.89.0`
- a working Docling Python runtime for PDF ingest
- a local multimodal model path:
  - Ollama is the default
  - LM Studio is the supported fallback

If you only want to inspect existing generated artifacts, you can do that without the full runtime stack.

## First commands to run

From the repository root:

```bash
cargo run --manifest-path Cargo.toml -- doctor --strict
```

That is the recommended preflight before any long `converge` run.

Then try a cheap command first:

```bash
cargo run --manifest-path Cargo.toml -- inspect README.md
```

## Fast path for a local source file

```bash
cargo run --manifest-path Cargo.toml -- ingest README.md
cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/readme/source_ir.json
cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/readme/evidence_ir.json
cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/readme/semantic_ir.json
```

That gives you the four staged artifacts without invoking the full convergent loop.

## Fast path for a real spec

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm
```

By default, `converge` now does the heavy local-first path:

- ingest once
- run VLM figure enrichment
- run NLP Level 3 backannotation
- rebuild downstream IR stages
- stop when the persisted knowledge snapshot stabilizes

Use a narrower run only when you mean to:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm --vlm-provider skip --nlp-provider skip
```

If you already generated a validation rescan queue, `converge` can inspect it after the fixed-point loop stabilizes:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target fsm --rescan-plan generated/validation/rescan_plan.json
```

Execution stays explicitly gated with `--execute-rescan-plan`.
Even then, changed validation deltas are reported for review and are not treated as automatic truth promotion.

## Local verification

The canonical local CI entrypoint is:

```bash
bash scripts/run_ci.sh
```

That now checks:

- Rust formatting
- Clippy with warnings denied
- Rust tests with Rust warnings denied
- Rustdoc with warnings denied
- mdBook build

## Where the outputs go

Generated artifacts are written under `generated/`:

- `generated/source_ir/<document_key>/source_ir.json`
- `generated/evidence_ir/<document_key>/evidence_ir.json`
- `generated/semantic_ir/<document_key>/semantic_ir.json`
- `generated/intent_ir/<document_key>/intent_ir.json`
- `generated/adapters/fsm/<document_key>/adapter.json`
- `generated/prior_memory/corpus_memory.json`

The `generated/` tree stays local and untracked.
