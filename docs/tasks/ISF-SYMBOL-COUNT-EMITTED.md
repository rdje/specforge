# ISF-SYMBOL-COUNT-EMITTED: make the adapter `constant_count`/`enum_count` reflect emitted content

## Metadata

- Tree ID: `ISF-SYMBOL-COUNT-EMITTED`
- Status: `done`
- Roadmap lane: `R6` (`.isf` adapter)
- Created: `2026-05-29`
- Last updated: `2026-05-29`
- Owner: repo-local workflow

## Goal

Finish the count-vs-emission honesty work begun in `ISF-SYMBOL-SURFACE-EMIT.2`.
The adapter artifact derives `transaction_count`/`rule_count` from
`isf_model.emitted_*_count()` ("metric == emitted content",
`ISF-TEMPORAL-LOWERING.2.4`), but `constant_count`/`enum_count` are still
counted from the **recovered** `intent_ir.symbol_definitions`. Now that
`render()` emits only the *safe-value subset* of constants/enums (operator-
expression values are excluded), the recovered count can over-report versus
the emitted `.isf`. Make `constant_count`/`enum_count` reflect what
`render()` actually emits — the same metric-honesty doctrine.

## Non-Goals

- No change to the emission policy itself (that is `ISF-SYMBOL-SURFACE-EMIT`);
  only the metric so the count matches the emitted surface.

## Acceptance Criteria

- `IsfIr` exposes `emitted_constant_count()` / `emitted_enum_count()` that
  count exactly what `render()` emits (DRY-shared via private
  `emitted_constants()` / `emitted_enums()`); `render()` uses the same
  shared accessors.
- The adapter artifact's `constant_count`/`enum_count` derive from those.
- A unit test locks count == emitted (incl. an excluded expression-valued
  constant); `scripts/run_ci.sh` green.

## Task Tree

- ID: `ISF-SYMBOL-COUNT-EMITTED`
  Status: `done`
  Goal: count-emitted metric for constants/enums
  Children: `ISF-SYMBOL-COUNT-EMITTED.1`

- ID: `ISF-SYMBOL-COUNT-EMITTED.1`
  Status: `done`
  Goal: >
    Add `emitted_constant_count`/`emitted_enum_count` (DRY-shared with
    `render()`); point the adapter `constant_count`/`enum_count` at them;
    unit test; close.
  Acceptance: `metric == emitted; render + counts share the safe-subset accessors; test green; scripts/run_ci.sh green.`
  Verification: >
    passed (`2026-05-29`) — added private `emitted_constants()` /
    `emitted_enums()` (the safe-value subsets `render()` uses) +
    `pub(crate) emitted_constant_count()` / `emitted_enum_count()`;
    refactored `render()` to call them (single source of truth);
    `adapters.rs` now derives `constant_count`/`enum_count` from them
    (dropped the recovered-`symbol_definitions` count + the now-unused
    `SymbolDefinitionKind` import). New `emitted_symbol_counts_reflect_safe_emitted_subset`
    test (safe + operator-expression mix → counts equal the safe subset);
    existing `isf_adapter_counts_equal_emitted_content` still green. Lib
    `1150 → 1151`; full `scripts/run_ci.sh` green; book subsection added.
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-29`** — the single fix leaf is `done`.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-SYMBOL-COUNT-EMITTED.1` | `done` | metric == emitted; render + counts share the safe-subset accessors; tree CLOSED |

## Decisions

- `2026-05-29`: created as the metric-side completion of the
  `ISF-SYMBOL-SURFACE-EMIT` honesty gap, consistent with the
  `ISF-TEMPORAL-LOWERING.2.4` "metric == emitted content" pattern for
  transactions/rules.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-29` | `ISF-SYMBOL-COUNT-EMITTED.1` | `emitted_constant_count`/`emitted_enum_count` DRY-shared with `render()`; adapter counts re-pointed; new + existing count tests green; lib `1150 → 1151`; full `scripts/run_ci.sh` (incl. mdBook) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-SYMBOL-COUNT-EMITTED.1` | `ISF-SYMBOL-COUNT-EMITTED.1 — constant_count/enum_count count emitted content; close tree` | metric == emitted (mirrors transaction/rule counts); DRY accessors; book |

## Changelog

- `2026-05-29`: `.1` — `constant_count`/`enum_count` now count emitted
  content via DRY-shared `emitted_constants()`/`emitted_enums()` accessors
  (dropped the recovered-`symbol_definitions` count + unused import); new
  count-lock test; book subsection; **tree CLOSED**.
- `2026-05-29`: Created — make `constant_count`/`enum_count` count emitted
  content, completing the `ISF-SYMBOL-SURFACE-EMIT` honesty gap.
