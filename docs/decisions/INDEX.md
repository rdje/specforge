# Decision records (layer C of `MEMORY_ARCHITECTURE.md`)

Durable, cross-cutting facts/decisions that must survive across sessions, AI models,
and harnesses — one record per file, dated, `Context → Decision → Consequences`.
Append + dedupe + supersede (never silently rewrite). Link records from the related
task-trees under `docs/tasks/`.

This is **layer C**: facts that outlive any single unit of work and don't belong in the
resume pointer (`MEMORY.md`, layer A) or a single task-tree (layer B). History of *what
changed* lives in git (layer D), not here.

| # | Title | Date | Status | Tags |
|---|---|---|---|---|
| [0001](0001-docling-device-cpu.md) | Docling ingest runs on CPU on this stack (torch MPS lacks float64) | 2026-06-01 | accepted | ingest, environment |
| [0002](0002-llm-vlm-provider-default.md) | Production LLM/VLM provider is local Ollama + qwen2.5vl:7b | 2026-06-01 | accepted | llm, provider |
| [0003](0003-task-tree-and-commit-doctrine.md) | Task-tree ownership before any code change; strict commit workflow | 2026-06-01 | accepted | process, doctrine |
| [0004](0004-severity-never-gated-by-verbosity.md) | Severity ≥ Warning is never gated by a verbosity/trace level | 2026-06-01 | accepted | observability, invariant |
| [0005](0005-temporal-logic-ltl-mtl-not-ctl-tla.md) | Temporal behavior captured in LTL/MTL, not CTL or TLA+ (mine, don't model-check) | 2026-06-04 | accepted | temporal, ltl, formalism |
| [0006](0006-doctrine-enforcement-architecture.md) | Doctrine-enforcement architecture: every doctrine becomes a mechanically-gated check (registry/driver, E1→E4) | 2026-06-22 | accepted | process, doctrine, enforcement |

## How to add a record
1. Copy the shape of an existing record (`Date`, `Status`, `## Context / Decision /
   Consequences / Links`).
2. Use the next sequential number; add a row to the table above.
3. Link it from the task-tree(s) it relates to.
4. To change a fact, add a new record (or mark the old one `superseded by 00NN`) — do
   not rewrite history.
