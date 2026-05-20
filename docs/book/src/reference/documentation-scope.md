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

### `AUDIT-DOC-RECONCILE` — the doctrine that keeps the docs honest

When you read a SpecForge guide, a ROADMAP entry, or a
README sentence, you should be able to trust that what it
says is what the code actually does. That sounds obvious. It
isn't free, and `AUDIT-DOC-RECONCILE` is the tree that paid
for it.

#### The user-facing guarantee

> **Every claim in the book / ROADMAP / README either
> describes what the code does, or it's recorded as wrong
> and gets fixed. The text is never allowed to silently lie
> about the code. When in doubt: the code is the truth, the
> text is the suspect.**

That's the doctrine. It's load-bearing for every later
tree's close-leaf — including every R16 subsection you just
read in this book.

#### Why this isn't free

Doc drift is the gentlest kind of bug: it doesn't break a
test, it doesn't crash a build, and it survives every
refactor that doesn't happen to touch the documented
surface. A typical drift case looks like this:

> *The ROADMAP says: "the adapter computes actor-relative
> direction from the flat `direction_hint` field."*
>
> *Six months earlier, R15-GRAPH-DIRECTION-MIGRATION moved
> direction-computation into the canonical KG. The adapter
> stopped consulting `direction_hint` for that purpose.*
>
> *The ROADMAP sentence stayed. Anyone reading the ROADMAP
> today gets a clean, plausible explanation of how the
> adapter works — except the explanation is wrong, by a
> migration nobody remembered to also write about.*

Multiply that by every active subsystem, every long-running
program, every README that promised an architecture before
the code arrived, and you get a documentation lane that
quietly drifts ahead of (or, more often, behind) the code
that's actually shipping.

#### What this tree fixed (concretely)

Two specific drifts were identified by a doc-audit pass and
reconciled in this tree:

1. **`ROADMAP` R15** still described adapter-side
   actor-relative direction computation and the deleted
   `.fsm` paths even though `.isf` (the only adapter
   target) defaults direction and width by design, and the
   actor-relative graph remained canonical in `SemanticIR` /
   `IntentIR`. The text contradicted the code; the text was
   rewritten to match.
2. **Stale references to multi-target HDL adapters**
   (SystemVerilog / Verilog / VHDL) lingered in the book and
   ROADMAP after `ISF-ONLY-CONSOLIDATION` made `.isf` the
   sole adapter target. The references were removed.

Both fixes followed the doctrine: the code was the truth;
the text was rewritten to describe it.

#### How the doctrine is enforced now

The doctrine became **structural** through the
`BOOK-METHOD-DOC` close-rule (see `Required Commit
Workflow` in `COMMIT.md` and `Completion Rules` in
`docs/TASK_TREE.md`):

- Every tree's closing leaf MUST add or refresh that tree's
  method-doc subsection in the topically-correct mdBook
  chapter — describing what the code does now.
- A close leaf whose book section is missing or stale is, by
  definition, **incomplete**.
- The CI step (`scripts/run_docs_ci.sh`) builds the book on
  every leaf, so the book is exercised continuously.

In other words: this doctrine + that close-rule mean drift
becomes a per-leaf bug instead of a six-month-later audit
finding. The book section you're reading right now exists
because the close-leaf that delivered the R7-VALIDATION tree
was required to refresh it.

#### What this buys you, as a SpecForge user

- **You can read this book and trust it.** If a section
  claims SpecForge does X, the code does X. If the section
  is wrong, that's a bug — file it, and the next close-leaf
  to touch the affected area will reconcile it.
- **You don't have to read the source to be sure.** The
  task-tree files remain the machine-tracked authority for
  what was delivered; the book is the topically-placed
  explanation that mirrors them. If you read either and
  reach the same conclusion, the doctrine is doing its job.
- **You can spot drift cheaply.** If you find a sentence
  here that doesn't match the code, you've found a real bug
  (and a small one — drifts are usually one paragraph in
  one chapter, not a tangled refactor).

*Authoritative tracking:*
`docs/tasks/AUDIT-DOC-RECONCILE.md`. The doctrine is
recorded as a standing rule in `docs/TASK_TREE.md`'s
Completion Rules and `COMMIT.md`'s Required Commit
Workflow.
