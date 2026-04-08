# Introduction

`specforge` is a staged Rust toolchain for turning specifications into typed, backend-independent intent.

The key idea is simple:

1. Do not treat the spec as something to "summarize better".
2. Convert more and more of it into structured, grounded, machine-checkable records.
3. Keep ambiguity explicit instead of hiding it.

The current canonical pipeline is:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

Adapters such as `.fsm` are downstream consumers of `IntentIR`, not the core product boundary.

This book is the canonical user-facing documentation surface for `specforge`. It is organized in increasing depth:

- start with `Getting Started` if you want to run the tool
- move to `Commands` if you want the operational workflow
- move to `Pipeline Model` if you want to understand what each IR stage means
- use `Reference` when you need artifact paths, troubleshooting, or continuity guidance

Two important project principles shape everything in this book:

- `specforge` is provenance-first: the current document must justify the facts that enter canonical IR
- AI is bounded: models can propose hypotheses, but validation and arbitration decide what survives

What is already real today:

- staged CLI commands for `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `converge`, `validate`, `kg-bench`, `project-validation`, and `learn-priors`
- tracked IR artifacts under `generated/`
- a local-first runtime with Docling plus Ollama or LM Studio preflighted through `doctor`
- a cross-document learning plane that stores typed priors in `generated/prior_memory/corpus_memory.json`

What this book does not replace:

- live continuity docs such as `ROADMAP.md`, `CHANGES.md`, and `MEMORY.md`
- internal architecture deep dives intended for active development sessions

Those remain in the repo root and are described later in [Live Docs And Continuity](reference/live-docs.md).

