# Documentation Scope And Continuity

The `mdBook` is the public-facing documentation product for `specforge`.
It is what the outside world should read to understand:

- what `specforge` does
- how the user-facing commands and workflows operate
- how the IR pipeline works
- why the tool is structured the way it is
- what the current truthfulness boundaries and limitations are

## Book contract

The book is not just onboarding material.
It should grow until every meaningful user-facing aspect of the project has a clear home here, ideally as its own section or chapter.

That means the book should cover:

- operational workflows
- runtime setup
- stage semantics
- validation surfaces
- cross-document learning behavior
- rationale for major architectural decisions that affect how users should understand the tool

## Separate continuity plane

The repo root markdown docs serve a different job.
They are live engineering and continuity infrastructure for:

- crash recovery
- session handoff
- roadmap steering
- validation projection
- implementation-state tracking

They are not the primary public product docs, even when they contain valuable detail.

## Practical rule

When a change affects what users should understand about `specforge`, it belongs in the book.

When a change mainly preserves live state, sequencing, recovery context, or session continuity, it belongs in the root continuity docs.
