# SpecForge

SpecForge is a Rust toolchain that mines traceable implementation intent from protocol,
component, system, and software-interface specifications. Typed stages transform source
material into backend-independent `IntentIR`, then lower only to FSMGen's `.isf` boundary.
It favors deterministic transforms, explicit validation, and structured residuals over
opaque generation.

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

`IntentIR` is the product boundary. FSMGen consumes `.isf` and owns scheduling, `.fsm`, and
HDL generation; SpecForge does not generate HDL directly.

The [roadmap](ROADMAP.md) and [task-tree index](docs/TASK_TREE.md) own current direction and
delivery state; this landing page does not duplicate them.

## Prerequisites

- Rust `1.95.0`
- Docling for PDF ingest via [`scripts/bootstrap_docling.sh`](scripts/bootstrap_docling.sh)
- Ollama for the default local VLM/NLP path, or LM Studio as the supported fallback

Existing-artifact inspection and deterministic commands do not need the PDF/model stack.

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

Generated artifacts stay under `generated/`. Run the complete gate with:

```bash
bash scripts/run_ci.sh
```

See [Getting Started](docs/book/src/getting-started.md),
[Commands](docs/book/src/commands/overview.md), and
[Generated Artifacts](docs/book/src/reference/generated-artifacts.md).

## Architecture at a glance

- `crates/specforge-core/` is the specification-instance-neutral extraction/IR/adapter boundary and owns the
  sealed opaque-identity and proof-verification kernel; its checked 38-family / 168-field producer graph is
  frozen for staged proof-carrying migration;
  `crates/specforge-conformance/` owns evaluation, replay, calibration, and named fixtures; and
  `crates/specforge/` is the CLI and compatibility facade over both.
- `docs/book/` is the maintained user-facing manual.
- `docs/tasks/` and `docs/decisions/` preserve engineering state and durable rationale.
- `generated/` holds repository-local, rebuildable outputs and is not tracked.
- `subs/fsmgen/` is the pinned downstream consumer reference and an independent Git authority.

Evidence provenance survives every stage; unresolved ambiguity stays explicit.

See [Architecture Rationale](docs/book/src/architecture-rationale.md).

## Documentation

The maintained manual starts at the [introduction](docs/book/src/introduction.md) and
[book index](docs/book/src/SUMMARY.md); operational help is in
[Troubleshooting](docs/book/src/reference/troubleshooting.md).

For contributors and project continuity:

- [Agent bootstrap](AGENTS.md)
- [Task-tree workflow](docs/TASK_TREE_README.md),
  [collection catalogs](docs/catalogs/INDEX.md), and [commit workflow](COMMIT.md)
- [Memory architecture](MEMORY_ARCHITECTURE.md) and bounded [resume pointer](MEMORY.md)
- [Knowledge-map architecture](knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md),
  [fact cards](docs/knowledge/INDEX.md), and [question search](KNOWLEDGE_MAP.md)
- [Doctrine enforcement](DOCTRINE_ENFORCEMENT.md) and [diagnostic toolbox](TOOLBOX.md)
- [README stability policy](README_POLICY.md)
- [Live-document containment doctrine](LIVE_DOCUMENT_SIZE_CONTAINMENT.md)
- [Project-data locality standard](PROJECT_DATA_LOCALITY.md)

## Contributing and support

Before changing anything, follow the [agent bootstrap](AGENTS.md), select an owning task leaf,
and use the [commit workflow](COMMIT.md). Run `bash scripts/run_ci.sh` before handoff.

Report defects and documentation drift through
[GitHub Issues](https://github.com/rdje/specforge/issues) with the command, relevant artifact,
and smallest safely shareable input.

## License

This repository currently has no project-level license file. Do not assume permission to copy,
modify, or redistribute the project without authorization from the owner.
