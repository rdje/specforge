# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`EXTRACTION-QUALITY-GAUGE.3j.2.c`** — a multi-cell table row's obligation drops the row key
  that scopes it (`LTI_MMU = True LTI_GPC = False`), and the key is inside the record's own `source_text`.
  Six of LTI's nine ungrounded records are one matrix cell. Measure the row-keyed population first.
- Next action: `.3j.2.c` must pick its population explicitly before measuring. `.3j.4` proved the
  *refreshed corpus* it asks for does not exist — LTI has no measured-stratum counterpart (only AXI and APB
  of the seven do). Either measure on the historical LTI under `ADR 0048` §6, which fits because the defect
  is a property of the proposal shape and not of the superseded producer, and label it dated evidence; or
  own the LTI re-ingest first. Do not silently take the first route.
- Current state: `.3j.4` closed by reading the artifacts instead of the plan. The measured stratum's
  promotable population is **5 documents / 62 provider calls**, not 27: the recall universe is the distinct
  persisted `source_text`, and 22 of the 27 have none. The gauge hazard the leaf opened with is **absent**
  — no measured document carries a gauge; the only seven that do are the seven already-promoted historical
  ones. `.3j.4.a` holds the frozen five (`ihi0022_l_2025_08`, `ihi0024_e`, `ihi0033_c`, `um10204`,
  `ihi0074_a`) and is blocked on a provider — neither `:11434` nor `:1234` answered on `2026-09-18`. It is
  now `.3j.1.b`'s prerequisite. `BOOK-CORPUS-STRATUM` opened and closed on the same day: the book now
  states both strata and the rule, with the split published as dated evidence rather than a current figure.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `.3j.4.a` alone is provider-gated; every other open leaf is actionable.
