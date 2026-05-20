# BOOK-METHOD-DOC: per-task-tree implementation & verification, in the book

## Metadata

- Tree ID: `BOOK-METHOD-DOC`
- Status: `done`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-19`
- Last updated: `2026-05-20`
- Closed: `2026-05-20`
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
  Status: `done` (`2026-05-20`; all closed pre-R16 trees per the
  placement map are now backfilled; active trees self-document at
  their own close per the standing rule)
  Goal: backfill existing/closed trees to the standard, each in its
  mapped chapter (batched by chapter to keep diffs reviewable).
  Acceptance: `Each existing tree has an accurate method-doc section in its mapped chapter; mdBook green.`

  - ID: `BOOK-METHOD-DOC.2.a`
    Status: `done` (`2026-05-20`)
    Goal: backfill the ISF-adapter chapter — `R6-ISF-ADAPTER`,
    `ISF-ONLY-CONSOLIDATION`, `ISF-ONLY-IR-PRUNE`,
    `ISF-TEMPORAL-LOWERING`, `ISF-HANDSHAKE-STAGE-LOWERING`,
    `FSMGEN-ISSUE-REPORTING`, `FSMGEN-SUBMODULE-BUMP` (7 trees).
    Acceptance: `Each of the 7 ISF/FSMGen trees has a topically-placed method-doc subsection under "Closed task trees — how each was implemented and verified" in docs/book/src/pipeline/isf-adapter.md; mdBook green.`
    Verification: `passed` — appended a "Closed task trees" section
      to `docs/book/src/pipeline/isf-adapter.md` with 7 per-tree
      subsections (R6-ISF-ADAPTER; ISF-ONLY-CONSOLIDATION;
      ISF-ONLY-IR-PRUNE; ISF-TEMPORAL-LOWERING with its
      R16-CONTRACT-IR.3 subsumption note; ISF-HANDSHAKE-STAGE-LOWERING
      with its R16-CONTRACT-IR.4 supersession note;
      FSMGEN-ISSUE-REPORTING; FSMGEN-SUBMODULE-BUMP). Each subsection
      is a short topical paragraph + `*Authoritative tracking:*` pointer.
      `mdbook build` green.
    Commit: `see Commit Log`

  - ID: `BOOK-METHOD-DOC.2.b`
    Status: `done` (`2026-05-20`)
    Goal: backfill `reference/*` chapters for `AUDIT-DOC-RECONCILE`,
    `SIGNOFF-REMEDIATION`.
    Acceptance: `Both trees have topically-placed method-doc subsections in reference/*.md; mdBook green.`
    Verification: `passed` — `AUDIT-DOC-RECONCILE` subsection
      appended to `docs/book/src/reference/documentation-scope.md`
      under a new "Closed task trees" section (its job is reconciling
      documentation-scope claims against code reality — topically
      correct); `SIGNOFF-REMEDIATION` subsection appended to
      `docs/book/src/reference/live-docs.md` under the same section
      heading (its job is the signoff/CI gate — topically correct).
      Each subsection is a short topical paragraph + `*Authoritative
      tracking:*` pointer; the recorded doctrine ("text describes
      code; signoff is non-negotiable") is the load-bearing piece
      every subsequent close-leaf inherits. `mdbook build` green.
    Commit: `see Commit Log`

  - ID: `BOOK-METHOD-DOC.2.c`
    Status: `done` (`2026-05-20`)
    Goal: backfill the remaining mapped trees per the placement map
    (`R6-*-HARDENING`, `PROVENANCE-HARDENING`). Per-chapter
    subsections in `pipeline/*` / `quality/*`.
    Acceptance: `Each remaining closed tree from the placement map has a topically-placed method-doc subsection; mdBook green.`
    Verification: `passed` — 7 closed hardening trees backfilled to
      their topically-correct chapters:
      `R6-SOURCE-HARDENING` → `pipeline/sourceir.md`;
      `R6-EVIDENCE-HARDENING` → `pipeline/evidenceir.md`;
      `R6-SEMANTIC-HARDENING` → `pipeline/semanticir.md`;
      `R6-INTENT-HARDENING` + `R6-CONVERGE-HARDENING` →
      `pipeline/intentir.md`;
      `R6-PRIOR-MEMORY-HARDENING` → `quality/corpus-memory.md`;
      `PROVENANCE-HARDENING` → `quality/validation.md`.
      Each subsection records the mutation-testing methodology
      (cargo-mutants delta-to-zero on the targeted symbols) and
      includes an `*Authoritative tracking:*` pointer.
      `R7-VALIDATION` and `R15-GRAPH-DIRECTION-MIGRATION` are still
      `active` — per the standing close-rule, their book sections
      are added by their own close-leaves, not by backfill. `mdbook
      build` green.
    Commit: `see Commit Log`

- ID: `BOOK-METHOD-DOC.3`
  Status: `done` (`2026-05-20`)
  Goal: encode the standing close-rule in the workflow docs
  (`docs/TASK_TREE.md` / `COMMIT.md`) so every future tree's close leaf
  refreshes its book section; then close.
  Acceptance: `Standing rule encoded in workflow doctrine; tree closed.`
  Verification: `passed` — added a "**Book method-doc close-rule
    (`BOOK-METHOD-DOC`)**" bullet to `docs/TASK_TREE.md`'s
    "Completion Rules" section: a leaf is **incomplete** if it
    closes a tree without adding/refreshing the tree's
    implementation+verification subsection in the topically-correct
    mdBook chapter (per the placement map). Added a parallel bullet
    to `COMMIT.md`'s "Required Commit Workflow" section so the rule
    is enforced at commit time as well as at the tree-completion
    boundary. The bullets cross-reference each other and the
    `AUDIT-DOC-RECONCILE` doctrine ("book / ROADMAP language must
    describe what the code does"). Docs-only; `mdbook build` green.
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-20`.** All leaves done; the close-rule is now
structurally enforced via the `Completion Rules` section of
`docs/TASK_TREE.md` and the `Required Commit Workflow` section of
`COMMIT.md`. Every future tree's closing leaf inherits the
discipline: book method-doc section in the topically-correct
chapter is a hard completion criterion.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `BOOK-METHOD-DOC.1` | `done` | Convention + map + first worked instance (R16-CONTRACT-IR) |
| 2.a | `BOOK-METHOD-DOC.2.a` | `done` | ISF-adapter chapter backfilled (7 trees) — `2026-05-20` |
| 2.b | `BOOK-METHOD-DOC.2.b` | `done` | reference/* chapters backfilled (AUDIT-DOC-RECONCILE, SIGNOFF-REMEDIATION) — `2026-05-20` |
| 2.c | `BOOK-METHOD-DOC.2.c` | `done` | 7 hardening trees backfilled across pipeline/*/quality/* — `2026-05-20` |
| 3 | `BOOK-METHOD-DOC.3` | `done` | Close-rule encoded in `docs/TASK_TREE.md` Completion Rules + `COMMIT.md` Required Workflow; tree closed `2026-05-20` |

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
- `2026-05-20`: **"Thorough" means user-friendly thorough** —
  explicit clarification by the user during the
  `R7-VALIDATION.5` book expansion:
  *"Yes, the user (me) shall be able to understand everything
  about R7-VALIDATION.5. The should be user friendly. The goal
  is to engage people not to scare them. The goal is to explain
  things to the user so that they understand. And we want them
  to understand if we want them to use specforge."*

  The book section is the **user's primary surface** for
  understanding a tree, not a condensed pointer to the
  task-tree file. A reader of the book chapter alone must be
  able to understand the tree's design end-to-end without
  needing to open the tree file. Concretely, the close-rule
  for every tree from `2026-05-20` onward is:
  - **Thorough** — mirror every concept the tree introduces,
    not just the typed names. If the tree's design has a
    "When required" subsection, the book has one too. If the
    tree spec has Non-Goals, the book lists them.
  - **Accurate** — language describes what the design / code
    does (the `AUDIT-DOC-RECONCILE` doctrine extended to
    book-vs-design).
  - **User-friendly** — explain the *why* before the *what*;
    introduce typed names alongside plain-language summaries;
    walk through worked examples; frame safety properties as
    user benefits (*"You never lose work to a silent
    rewrite"*) rather than restrictions.
  - **Engage, do not scare** — open with the problem the
    user has, not the type lattice; use second-person; keep
    paragraphs short; use code blocks sparingly and only when
    they illuminate.

  The `R7-VALIDATION.5` book subsection in
  `docs/book/src/quality/validation.md` (rewritten
  `2026-05-20`) is the worked template for the standard going
  forward — refer to it as the canonical example when closing
  any future tree.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-19` | `BOOK-METHOD-DOC.1` | convention + placement map + standing rule recorded; first instance = R16-CONTRACT-IR book section | `passed` |
| `2026-05-20` | `BOOK-METHOD-DOC.2.a` | 7 ISF/FSMGen trees backfilled into `docs/book/src/pipeline/isf-adapter.md` "Closed task trees" section; mdBook builds | `passed` |
| `2026-05-20` | `BOOK-METHOD-DOC.2.b` | `AUDIT-DOC-RECONCILE` → `reference/documentation-scope.md`; `SIGNOFF-REMEDIATION` → `reference/live-docs.md`; both topically placed; mdBook builds | `passed` |
| `2026-05-20` | `BOOK-METHOD-DOC.2.c` | 7 hardening trees backfilled across pipeline/*/quality/* (sourceir/evidenceir/semanticir/intentir × 2 trees/corpus-memory/validation); R7-VALIDATION + R15-GRAPH-DIRECTION-MIGRATION still active (self-document at close per standing rule); mdBook builds | `passed` |
| `2026-05-20` | `BOOK-METHOD-DOC.3` | close-rule encoded in `docs/TASK_TREE.md` Completion Rules + `COMMIT.md` Required Workflow; cross-referenced + AUDIT-DOC-RECONCILE doctrine reference; mdBook builds | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `BOOK-METHOD-DOC.1` | `BOOK-METHOD-DOC.1 — convention + placement map (with R16-CONTRACT-IR.1)` | docs-only; worked template = R16-CONTRACT-IR book section |
| `BOOK-METHOD-DOC.2.a` | `BOOK-METHOD-DOC.2.a — backfill ISF-adapter chapter (7 closed pre-R16 trees)` (`3fae27dc`) | docs-only; per the convention; mdBook green |
| `BOOK-METHOD-DOC.2.b` | `BOOK-METHOD-DOC.2.b — backfill reference/* chapters (AUDIT-DOC-RECONCILE, SIGNOFF-REMEDIATION)` (`95687ed4`) | docs-only; topical placement per map; mdBook green |
| `BOOK-METHOD-DOC.2.c` | `BOOK-METHOD-DOC.2.c — backfill hardening trees across pipeline/* + quality/* (7 trees)` (`f43a1b49`) | docs-only; topical placement per map; mdBook green; active trees self-document at close per standing rule |
| `BOOK-METHOD-DOC.3` | `BOOK-METHOD-DOC.3 — encode close-rule into workflow doctrine + close tree` | docs-only; close-rule now structurally enforced via TASK_TREE / COMMIT |

## Changelog

- `2026-05-19`: Created by explicit user direction — the book must
  thoroughly and accurately explain how each task-tree is implemented
  and verified, topically placed; with a standing close-rule so it
  never drifts.
- `2026-05-20`: **Tree CLOSED.** `.3` done — encoded the standing
  close-rule into the workflow doctrine: added a "**Book method-doc
  close-rule (`BOOK-METHOD-DOC`)**" bullet to `docs/TASK_TREE.md`'s
  "Completion Rules" section (a leaf is incomplete if it closes a
  tree without adding/refreshing the tree's
  implementation+verification subsection in the topically-correct
  mdBook chapter); added the parallel bullet to `COMMIT.md`'s
  "Required Commit Workflow" so the rule is enforced at commit time
  as well as at tree-completion. Both bullets cross-reference each
  other and the `AUDIT-DOC-RECONCILE` doctrine. The discipline is
  now structural: every future tree's closing leaf inherits the
  requirement; the convention will not silently drift. Docs-only;
  mdBook green. Tree CLOSED.
- `2026-05-20`: `.2.c` done — backfilled 7 closed hardening trees
  to their topically-correct chapters per the placement map:
  `R6-SOURCE-HARDENING` → `pipeline/sourceir.md`;
  `R6-EVIDENCE-HARDENING` → `pipeline/evidenceir.md`;
  `R6-SEMANTIC-HARDENING` → `pipeline/semanticir.md`;
  `R6-INTENT-HARDENING` + `R6-CONVERGE-HARDENING` →
  `pipeline/intentir.md`; `R6-PRIOR-MEMORY-HARDENING` →
  `quality/corpus-memory.md`; `PROVENANCE-HARDENING` →
  `quality/validation.md`. Each subsection records the
  mutation-testing methodology (cargo-mutants delta-to-zero on
  targeted symbols). `R7-VALIDATION` and
  `R15-GRAPH-DIRECTION-MIGRATION` are still active; per the
  standing close-rule their book sections are added by their own
  close-leaves, not by backfill. **`BOOK-METHOD-DOC.2` parent leaf
  marked DONE** (all closed pre-R16 trees per the placement map
  are now backfilled; active trees self-document at close per the
  standing rule). Frontier → `.3` (encode close-rule in workflow
  doctrine + close tree). Docs-only; mdBook green.
- `2026-05-20`: `.2.b` done — appended per-tree method-doc
  subsections for `AUDIT-DOC-RECONCILE` (in
  `docs/book/src/reference/documentation-scope.md`, topical: docs-vs-
  code reconciliation) and `SIGNOFF-REMEDIATION` (in
  `docs/book/src/reference/live-docs.md`, topical: signoff/CI gate as
  part of live-doc continuity). Each subsection records the
  load-bearing doctrine the tree established (docs describe code;
  signoff is non-negotiable). Docs-only; mdBook green.
- `2026-05-20`: `.2.a` done — appended "Closed task trees" section
  to `docs/book/src/pipeline/isf-adapter.md` with topically-placed
  per-tree subsections for the 7 closed pre-R16 ISF/FSMGen trees
  (`R6-ISF-ADAPTER`, `ISF-ONLY-CONSOLIDATION`, `ISF-ONLY-IR-PRUNE`,
  `ISF-TEMPORAL-LOWERING` with R16-CONTRACT-IR.3 subsumption note,
  `ISF-HANDSHAKE-STAGE-LOWERING` with R16-CONTRACT-IR.4 supersession
  note, `FSMGEN-ISSUE-REPORTING`, `FSMGEN-SUBMODULE-BUMP`). Honest
  split of `.2` into `.2.a` (done), `.2.b` (reference/*), `.2.c`
  (R6/R7/R15* hardening) keeps each PNT step bounded per rule 5.
  mdBook green.
