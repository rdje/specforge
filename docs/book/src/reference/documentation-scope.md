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

## Closed task trees — how each was implemented and verified

### `AUDIT-DOC-RECONCILE` — reconcile docs against code reality

When the documentation lane drifts ahead of (or behind) the code,
the book/ROADMAP/README must be brought back to what the code
actually does — never the other way around. This tree reconciled
two specific findings from a doc-audit pass:

1. `ROADMAP` R15 still described adapter-side actor-relative
   direction computation and the deleted `.fsm` paths even though
   `.isf` (the only adapter) defaults direction/width by design
   and the actor-relative graph remained canonical in
   `SemanticIR`/`IntentIR`. The text contradicted the code; the
   text was rewritten to match.
2. Stale references to multi-target HDL adapters were removed
   from the book and ROADMAP after `ISF-ONLY-CONSOLIDATION` made
   `.isf` the sole target.

The doctrine recorded by this tree (and now load-bearing for
every subsequent close-leaf): *book / ROADMAP language must
describe what the code does — when it doesn't, the text is
wrong, not the code*. Verified by leaf-by-leaf diff against the
relevant code surfaces + `scripts/run_docs_ci.sh`.
*Authoritative tracking:* `docs/tasks/AUDIT-DOC-RECONCILE.md`.
