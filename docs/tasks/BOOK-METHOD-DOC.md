# BOOK-METHOD-DOC: per-task-tree implementation & verification, in the book

## Metadata

- Tree ID: `BOOK-METHOD-DOC`
- Status: `active`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

The mdBook must carry, for **every task-tree**, a clear, thorough, and
accurate description of **how that tree is (being) implemented and how it
is verified** — placed in the topically-correct chapter (not a dump).
Per explicit user direction (2026-05-19): "these types of precise,
thorough and accurate information about how each task-tree is supposed to
be implemented and verified is of utmost importance for the user."

The task-tree files remain the machine-tracked authority; the book is the
human-facing, topically-organized explanation that mirrors them.

## Non-Goals

- Not duplicating task-tree files verbatim — the book section is the
  *explanatory* "how + why + how-verified", topically placed.
- No production code (docs only).
- Do not let the book drift from the trees: a stale book section is a
  bug this tree's standing rule prevents.

## Acceptance Criteria

- A documented **convention** exists: every task-tree has a book section
  in the topically-correct chapter covering goal, implementation
  approach, verification method, and status; and a **standing close
  rule** — a tree's close leaf must add/refresh its book section.
- Existing trees are **backfilled** to that standard, each in its right
  chapter (mapping recorded here).
- `scripts/run_ci.sh` green (mdBook builds) per leaf; every leaf via
  `COMMIT.md`.

## Chapter placement map (where each tree's method-doc lives)

| Tree(s) | Book chapter/section |
| --- | --- |
| `R16-*` (program + 6 sub-trees) | `direction/temporal-intent-capture.md` (per-sub-tree subsections) |
| `ISF-TEMPORAL-LOWERING`, `ISF-HANDSHAKE-STAGE-LOWERING` | `pipeline/isf-adapter.md` + `domain/temporal-semantics.md` |
| `ISF-ONLY-CONSOLIDATION`, `ISF-ONLY-IR-PRUNE`, `R6-ISF-ADAPTER` | `pipeline/isf-adapter.md` |
| `FSMGEN-ISSUE-REPORTING`, `FSMGEN-SUBMODULE-BUMP` | `pipeline/isf-adapter.md` (FSMGen-contract section) + `reference/live-docs.md` |
| `AUDIT-DOC-RECONCILE`, `SIGNOFF-REMEDIATION` | `reference/documentation-scope.md` / `reference/live-docs.md` |
| `R6-*-HARDENING`, `R7-VALIDATION`, `PROVENANCE-HARDENING` | the matching `pipeline/*` or `quality/*` chapter |
| `R15*-…` (graph/temporal/converge/eval) | `domain/*` + `quality/*` chapters |

## Task Tree

- ID: `BOOK-METHOD-DOC`
  Status: `active`
  Goal: book carries per-tree implementation+verification, topically placed
  Children: `.1`, `.2`, `.3`

- ID: `BOOK-METHOD-DOC.1`
  Status: `done`
  Goal: >
    Define the convention + standing close-rule + the chapter-placement
    map; record it here and as durable doctrine (memory + this tree).
    First application is `R16-CONTRACT-IR` (its `.1` design lands a
    thorough book section — the worked template).
  Acceptance: `Convention + close-rule + placement map recorded; first instance demonstrated by R16-CONTRACT-IR's book section.`
  Verification: `passed` — convention + placement map recorded above;
    standing close-rule recorded in Decisions + memory
    `feedback-book-method-doc`; the worked template is the
    `R16-CONTRACT-IR` section added to `direction/temporal-intent-capture.md`
    in the same batch.
  Commit: `see Commit Log`

- ID: `BOOK-METHOD-DOC.2`
  Status: `pending`
  Goal: backfill existing/closed trees to the standard, each in its
  mapped chapter (batched by chapter to keep diffs reviewable).
  Acceptance: `Each existing tree has an accurate method-doc section in its mapped chapter; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-METHOD-DOC.3`
  Status: `pending`
  Goal: encode the standing close-rule in the workflow docs
  (`docs/TASK_TREE.md` / `COMMIT.md`) so every future tree's close leaf
  refreshes its book section; then close.
  Acceptance: `Standing rule encoded in workflow doctrine; tree closed.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `BOOK-METHOD-DOC.1` | `done` | Convention + map + first worked instance (R16-CONTRACT-IR) |
| 2 | `BOOK-METHOD-DOC.2` | `pending` | Next — backfill existing trees by chapter (after R16 work, or interleaved per user priority) |
| 3 | `BOOK-METHOD-DOC.3` | `pending` | Encode the standing close-rule into workflow doctrine |

## Decisions

- `2026-05-19`: Standing close-rule — **every task-tree's close (`.N`)
  leaf must add/refresh its book method-doc section in the mapped
  chapter**; verification of a close leaf includes "book section
  accurate vs the tree". R16 sub-trees self-document as they execute
  (so `BOOK-METHOD-DOC.2` backfill is bounded to pre-2026-05-19 trees).
- `2026-05-19`: User priority is `R16-CONTRACT-IR.1` next (option b);
  `BOOK-METHOD-DOC.2` backfill is scheduled, not blocking, and proceeds
  on PNT/explicit direction. The convention is proven immediately by the
  `R16-CONTRACT-IR` book section landing with its design.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-19` | `BOOK-METHOD-DOC.1` | convention + placement map + standing rule recorded; first instance = R16-CONTRACT-IR book section | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `BOOK-METHOD-DOC.1` | `BOOK-METHOD-DOC.1 — convention + placement map (with R16-CONTRACT-IR.1)` | docs-only; worked template = R16-CONTRACT-IR book section |

## Changelog

- `2026-05-19`: Created by explicit user direction — the book must
  thoroughly and accurately explain how each task-tree is implemented
  and verified, topically placed; with a standing close-rule so it
  never drifts.
