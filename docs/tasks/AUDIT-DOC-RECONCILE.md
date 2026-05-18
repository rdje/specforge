# AUDIT-DOC-RECONCILE: fix doc drift found by the post-ISF-ONLY audit

## Metadata

- Tree ID: `AUDIT-DOC-RECONCILE`
- Status: `active`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Goal

Reconcile the two doc-drift findings from the audit:

1. ROADMAP R15 goal / `remaining:` / one completion criterion still
   describe adapter-side actor-relative direction computation and the
   deleted `.fsm` paths. The only adapter (`.isf`) defaults
   direction/width by design; the actor-relative graph model itself
   remains canonical in `SemanticIR`/`IntentIR`. The text contradicts
   the code (the `.1`/`.7` reconcile added a superseding note but left
   the goal/criterion/remaining wording contradictory).
2. `docs/book/src/commands/overview.md` (and `commands/pipeline.md`)
   document `converge` with only 5 flags; `cli.rs ConvergeArgs` also has
   `--max-iterations`, `--vlm-model`, `--nlp-model`,
   `--nlp-max-sentences`, `--prior-memory`, `--rescan-plan-limit` plus
   unstated defaults.

## Non-Goals

- Do not rewrite ROADMAP R15's historical `done:` bullets (they are a
  true record of completed migration work; only the forward/normative
  goal + criterion + `remaining:` are reconciled).
- Do not change CLI behavior — only document what exists.

## Acceptance Criteria

- R15 `goals:` / completion criteria / `remaining:` are consistent with
  ISF-only reality (graph model canonical in IR; adapter defers
  direction to FSMGen by design).
- The book documents the full `converge` flag surface + defaults
  accurately vs `cli.rs`.
- Docs only; mdBook builds; every leaf committed through `COMMIT.md`.

## Task Tree

- ID: `AUDIT-DOC-RECONCILE`
  Status: `active`
  Goal: `R15 text + book converge flags reconciled to code reality.`
  Children: `.1`, `.2`

- ID: `AUDIT-DOC-RECONCILE.1`
  Status: `pending`
  Goal: >
    Reconcile ROADMAP R15 `goals:` / completion criteria / `remaining:`
    to the ISF-only reality: the actor-relative directed graph is
    canonical in `SemanticIR`/`IntentIR`; the `.isf` adapter
    intentionally defaults unknown direction/width and defers scheduling
    to FSMGen; the deleted `.fsm` adapter-direction paths are removed
    from the forward text (historical `done:` bullets stay as record).
  Acceptance: `R15 forward text non-contradictory vs code; mdBook build green; docs only.`
  Verification: `pending`
  Commit: `pending`

- ID: `AUDIT-DOC-RECONCILE.2`
  Status: `pending`
  Goal: >
    Document the full `converge` flag surface + defaults in
    `docs/book/src/commands/overview.md` (and `commands/pipeline.md`
    where relevant) accurately vs `crates/specforge/src/cli.rs`
    `ConvergeArgs`.
  Acceptance: `Book converge flags/defaults match cli.rs; mdBook build green; docs only.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `AUDIT-DOC-RECONCILE.1` | `pending` | Highest-signal doc drift (roadmap contradicts code) |
| 2 | `AUDIT-DOC-RECONCILE.2` | `pending` | Book converge-flag completeness |

## Decisions

- `2026-05-18`: Only R15's forward/normative text is reconciled;
  historical `done:` bullets are preserved as record.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `AUDIT-DOC-RECONCILE.1` | `pending` | `pending` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `AUDIT-DOC-RECONCILE.1` | `pending` | `pending` |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit (R15 text drift +
  book converge-flag completeness).
