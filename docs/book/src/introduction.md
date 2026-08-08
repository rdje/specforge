# Introduction

`specforge` is a staged Rust toolchain for turning specifications into typed, backend-independent intent.

The key idea is simple:

1. Do not treat the spec as something to "summarize better".
2. Convert more and more of it into structured, grounded, machine-checkable records.
3. Keep ambiguity explicit instead of hiding it.

The current canonical pipeline is:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`

The `.isf` adapter is a downstream consumer of `IntentIR`, not the core product boundary. `.fsm`/HDL are out of scope — FSMGen consumes `.isf` and owns them downstream.

This book is the canonical user-facing documentation surface for `specforge`. It is organized in increasing depth:

- start with `Getting Started` if you want to run the tool
- move to `Commands` if you want the operational workflow
- move to `Pipeline Model` if you want to understand what each IR stage means
- move to `Domain Model` if you want to understand hardware semantics such as clock/reset infrastructure, handshake roles, and actor connectivity
- move to `Validation And Learning` if you want to understand how `specforge` judges truthfulness and accumulates reusable extraction experience
- use `Reference` when you need artifact paths, troubleshooting, or continuity guidance

This book is also the public-facing explanation of the project.
It should keep growing until every meaningful user-facing aspect of `specforge` is covered here with a clear section or chapter.
That includes:

- what `specforge` does
- why it is architected this way
- how each user-visible surface works
- why major architectural choices were made
- where current limits and truthfulness boundaries still are

Two important project principles shape everything in this book:

- `specforge` is provenance-first: the current document must justify the facts that enter canonical IR
- AI is bounded: models can propose hypotheses, but validation and arbitration decide what survives

Representative behavior that is already real today includes:

- staged CLI commands for `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `converge`, `validate`, `kg-bench`, `project-validation`, `rescan-plan`, `learn-priors`, and `corpus-kb`
- tracked IR artifacts under `generated/`
- a local-first runtime with Docling plus Ollama or LM Studio preflighted through `doctor`
- a cross-document learning plane that stores typed priors in `generated/prior_memory/corpus_memory.json`

What this book does not replace:

- live continuity docs such as `ROADMAP.md`, `CHANGES.md`, and `MEMORY.md`
- internal engineering records intended for active development sessions

Those remain outside the book because they serve a different purpose: continuity, live validation projection, steering, and recovery after session loss or crash. Stable root user/architecture filenames are compatibility pointers back into this book, not a second public manual.
That split is described later in [Documentation Scope And Continuity](reference/documentation-scope.md).
