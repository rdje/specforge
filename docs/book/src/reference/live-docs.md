# Live Docs And Continuity

The book is the canonical user-facing documentation surface.
The repo root still contains the live operational and continuity documentation needed to keep work recoverable across long sessions, crashes, and handoffs.

These are separate documentation planes.

- the book explains the product to the outside world
- the root live docs preserve the current engineering state

Both matter, but they are not interchangeable.

## Root docs and what they are for

- `README.md`
  - entry point and high-level navigation
- `ROADMAP.md`
  - live sequencing and remaining work
- `LIVE_ACHIEVEMENT_STATUS.md`
  - current status snapshot
- `VALIDATION_SNAPSHOT.md`
  - tracked projection of persisted validation reports
- `RUST_CODEBASE_ANALYSIS.md`
  - architecture and codebase state
- `DEVELOPMENT_NOTES.md`
  - engineering rationale
- `CHANGES.md`
  - detailed current change history
- `MEMORY.md`
  - crash-safe continuity record

## How to think about each surface

The book should be stable enough for a new reader.
It should explain:

- what `specforge` does
- how the pipeline works
- why the project is structured this way
- how to run and inspect the tool
- how validation and learning are supposed to behave

The root live docs are allowed to be more operational.
They track:

- current baseline scores
- active roadmap slices
- implementation notes
- handoff context
- recent decisions
- crash recovery breadcrumbs

That means root docs may mention temporary states, current artifacts, local blockers, or implementation details that would be too noisy for the public book.

## Why the split exists

The book is what the world should see.
It should explain the product clearly and transparently.

The root docs need to stay live, operational, and sometimes noisy, because they are part of the project’s continuity system for active development and session recovery.

## What belongs in the book

Every durable user-facing concept should eventually land in the book.

Examples:

- runtime setup
- command usage
- generated artifact layout
- the four IR stages
- validation scoring
- KG fixture benchmarking
- cross-document prior memory
- clock/reset infrastructure semantics
- corpus knowledge-base direction
- troubleshooting guidance

If a concept is part of how `specforge` works or why it is built that way, the book should explain it at the right level of depth.

## What belongs in live docs

The root live docs are the right place for:

- current score snapshots
- detailed change chronology
- session handoff notes
- roadmap sequencing
- implementation rationale that may still be in motion
- local runtime findings
- warnings about artifact freshness

Those docs can be more detailed, more current, and less polished because their main job is continuity.

## When to update both

Many changes should touch both surfaces.

For example:

- a new public command should update the book and the live docs
- a new IR concept should update the book and the live docs
- a score-only refresh may update only the live docs
- a docs-architecture clarification may update only the book plus continuity notes

The guiding rule is:

- update the book when the public understanding of `specforge` changes
- update live docs when the repo’s current engineering state changes

## Practical rule

Use the book to understand and evaluate `specforge`.
Use the root docs to understand the repo’s live state and recover work safely.

## Why this matters for long-running work

`specforge` is being built across many small truthfulness and architecture slices.
Some runs are long, and some work depends on local generated artifacts that are intentionally not committed.

The live docs make that workflow survivable.
They let a future session answer:

- what was the last known baseline?
- what changed recently?
- what is the next likely slice?
- which artifacts were refreshed?
- which caveats are still real?

The book, meanwhile, keeps the project understandable to someone who did not live through those sessions.
