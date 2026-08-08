# SpecForge

SpecForge is a staged Rust toolchain for mining implementation-relevant intent from
protocol, component, system, and software-interface specifications. It turns human-authored
source material into a backend-independent `IntentIR`, then lowers that intent through one
adapter boundary: `.isf` for FSMGen.

The project is aimed at engineers who need traceable, reviewable design intent before an
implementation exists. It favors typed evidence, deterministic transforms, explicit
validation, and structured residuals over opaque end-to-end generation.

## Scope

SpecForge owns this pipeline:

```text
source documents
    -> SourceIR
    -> EvidenceIR
    -> SemanticIR
    -> IntentIR
    -> .isf adapter
```

`IntentIR` is the canonical product boundary. FSMGen consumes `.isf` and owns scheduling,
`.fsm`, and HDL generation downstream. SpecForge does not generate SystemVerilog, Verilog,
or VHDL directly.

The tool is under active development. The [roadmap](ROADMAP.md) and
[task-tree index](docs/TASK_TREE.md) carry current work and delivery state; this landing page
deliberately does not duplicate them.

## Prerequisites

- Rust `1.95.0`
- Docling for PDF ingest; use the repository-local
  [`scripts/bootstrap_docling.sh`](scripts/bootstrap_docling.sh) setup path
- Ollama for the default local VLM/NLP path, or LM Studio as the supported fallback

Inspecting existing artifacts and running deterministic commands do not require the full
PDF/model stack.

## Quick start

From the repository root:

```bash
cargo run --manifest-path Cargo.toml -- doctor --strict
cargo run --manifest-path Cargo.toml -- inspect README.md
```

For a stage-by-stage local text run:

```bash
cargo run --manifest-path Cargo.toml -- ingest README.md
cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/readme/source_ir.json
cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/readme/evidence_ir.json
cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/readme/semantic_ir.json
```

For a real specification and the `.isf` adapter:

```bash
cargo run --manifest-path Cargo.toml -- converge /path/to/spec.pdf --target isf
```

Generated artifacts stay under the repository-local `generated/` tree. Run the complete
project gate with:

```bash
bash scripts/run_ci.sh
```

See [Getting Started](docs/book/src/getting-started.md) for setup and first-use detail,
[Commands](docs/book/src/commands/overview.md) for the current CLI, and
[Generated Artifacts](docs/book/src/reference/generated-artifacts.md) for output paths and
lifecycle.

## Architecture at a glance

- `crates/specforge/` contains the Rust CLI, typed IRs, extraction pipeline, validators, and
  adapters.
- `docs/book/` is the maintained user-facing manual.
- `docs/tasks/` and `docs/decisions/` preserve engineering state and durable rationale.
- `generated/` holds repository-local, rebuildable outputs and is not tracked.
- `subs/fsmgen/` is the pinned downstream consumer reference and an independent Git authority.

Evidence provenance is retained through the stages. Automation may propose or enrich facts,
but unresolved ambiguity remains explicit rather than being silently promoted to truth.

For the design rationale, start with [Architecture Rationale](docs/book/src/architecture-rationale.md).

## Documentation

For users:

- [mdBook introduction](docs/book/src/introduction.md)
- [Getting Started](docs/book/src/getting-started.md)
- [Commands](docs/book/src/commands/overview.md)
- [Pipeline and domain reference](docs/book/src/SUMMARY.md)
- [Troubleshooting](docs/book/src/reference/troubleshooting.md)

For contributors and project continuity:

- [Agent bootstrap](AGENTS.md)
- [Task-tree workflow](docs/TASK_TREE_README.md) and [complete catalog](docs/TASK_TREE.md)
- [Canonical collection catalogs](docs/catalogs/INDEX.md)
- [Commit workflow](COMMIT.md)
- [Memory architecture](MEMORY_ARCHITECTURE.md) and bounded [resume pointer](MEMORY.md)
- [Knowledge-map architecture](knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md), bounded
  [fact-card catalog](docs/knowledge/INDEX.md), and generated [question-search landing](KNOWLEDGE_MAP.md)
- [Doctrine enforcement](DOCTRINE_ENFORCEMENT.md) and [diagnostic toolbox](TOOLBOX.md)
- [README stability policy](README_POLICY.md)
- [Live-document containment doctrine](LIVE_DOCUMENT_SIZE_CONTAINMENT.md)
- [Project-data locality standard](PROJECT_DATA_LOCALITY.md)

## Contributing and support

Before changing anything, follow the [agent bootstrap](AGENTS.md), create or select an owning
task-tree leaf, and use the [commit workflow](COMMIT.md). Run `bash scripts/run_ci.sh` before
handoff when the change warrants the full gate.

Report defects and documentation drift through
[GitHub Issues](https://github.com/rdje/specforge/issues). Include the command, relevant artifact
path, and the smallest reproducible input that can be shared safely.

## License

This repository currently has no project-level license file. Do not assume permission to copy,
modify, or redistribute the project without authorization from the owner.
