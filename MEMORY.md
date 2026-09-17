# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-QUALITY-GAUGE.3j.2.b`** — the catalog can hold a **parameterised declaration
  template**. APB declares `PSELx`, so the catalog holds `PSELX`/`PSELXCHK` and the document's own family
  name `PSEL` is unresolvable; ATB's `ATB` -> `ATBYTES` is a truncation and is NOT the same thing.
- Next action: establish whether a declaration template is recognisable from document grammar alone
  (ADR 0006 — never from the spelling), and what a resolver may do with one. Read-only over persisted
  artifacts, no provider needed. Adjudicate before wiring, and do not let it absorb `.3j.2.a.i`'s case.
- Current state: `.3j.2.a.i` closed `2026-09-18` — `X[w-1:0]` now resolves to `X` against the document's
  stated width, and nothing else does. Both guards proved load-bearing by separate A/Bs: a top-bit slice
  `X[w-1]` satisfies the width comparison alone, so `low == 0` is not redundant. Shipped reach re-derived
  through the production function: 3 of 16 carried-name subjects. `PRODUCTION-GENERICITY` re-derived via
  `aggregate_change` (+3 functions, +17 sites, +17 edges; every boundary count unmoved).
  `.3j.2.c` (a row-keyed table obligation that drops its key) is open. `.3j.1.b` stays blocked: no model.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
