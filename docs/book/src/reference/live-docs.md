# Live Docs And Continuity

The book is the canonical user-facing documentation surface.
The repo root still contains the live operational and continuity documentation needed to keep work recoverable across long sessions, crashes, and handoffs.

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

## Why the split exists

The book is what the world should see.
It should explain the product clearly and transparently.

The root docs need to stay live, operational, and sometimes noisy, because they are part of the project’s continuity system for active development and session recovery.

## Practical rule

Use the book to understand and evaluate `specforge`.
Use the root docs to understand the repo’s live state and recover work safely.
