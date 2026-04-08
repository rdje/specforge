# Live Docs And Continuity

The book is the canonical user-facing documentation surface.
The repo root still contains the live operational and continuity documentation needed to run the project safely over long sessions.

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

The book should stay readable and stable for users.

The root docs need to stay live, operational, and sometimes noisy, because they are also part of the project’s continuity system for long-running development.

## Practical rule

Use the book to understand and use `specforge`.
Use the root docs to understand the repo’s live state.

