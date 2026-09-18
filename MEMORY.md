# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-QUALITY-GAUGE.3j.3`** — the LLM pass's universe is the distinct `source_text`
  of constraints the DETERMINISTIC paths already emitted, so it can never see a span they missed. That is
  a hard recall ceiling and nothing states it. Measure it before widening anything.
- Next action: measure obligation-bearing statements against distinct spans the LLM path actually visits,
  per document, through the production selection rather than a proxy — the `.3j.2`/`.3j.2.c` censuses are
  the pattern, and both live as `--ignored` local measurements beside the code they measure.
- Current state: `.3j.2.c` + `.3j.2.c.i` closed today. An obligation is now **refused** when its span is a
  matrix row whose first cell binds a configuration — regardless of any condition, because the condition
  test is literal occurrence and the key occurs in the row with everything else. Reach 7 of 7, re-derived
  through the shipped predicate; A/B RED in both directions. `.3j.4` (5 docs / 62 calls, no gauge to drop)
  and `BOOK-CORPUS-STRATUM` also closed today.
- Open finding, not yet owned: `CORPUS-FRONTIER` publishes **52 of 57 refreshed** while only **27 of 78**
  persisted documents load canonically. Probably two different meanings of current — refreshed-once versus
  re-derivable-today — but it is unverified, and a reader of `CORPUS-COVERAGE.md` would take 52 as current.
  Verify the set relation before asserting either way; do not classify and move on.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none here. `.3j.4.a` is provider-gated; `.3k.2h`/`.3k.2i` wait on a legacy re-ingest that
  `CORPUS-COVERAGE` owns (five documents remain in its declared frontier).
