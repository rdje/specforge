# Live Docs And Continuity

The book is the canonical user-facing documentation surface.
The repo root still contains the live operational and continuity documentation needed to keep work recoverable across long sessions, crashes, and handoffs.

These are separate documentation planes.

- the book explains the product to the outside world
- the root live docs preserve the current engineering state

Both matter, but they are not interchangeable.

## Containment doctrine active; migrations in progress

SpecForge enforces a bounded-live-document architecture under
`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`. The purpose is not to reduce durable
information or user documentation. It is to separate bounded current views,
maintained reference prose, generated projections, rolling chronology, and
retrievable history so no mandatory read grows forever.

The project-owned authority is `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`. A complete JSONL registry
classifies every parent-tracked Markdown path exactly once, and the `LIVE-DOC-SIZE` doctrine check
re-derives the resulting-tree metrics on every commit and CI run. The pinned FSMGen submodule is a
separate Git authority and is not swept into SpecForge's registry.

The control plane is bounded too: schema version, allowed fields, record count, registry and record
bytes, array cardinality, and scalar bytes all fail closed. A 48-case fixture suite exercises every
local lifecycle plus index, freshness, currency, routing, transition-baseline, and ceiling-history
paths. Its disposable workspaces live only under repository-local `generated/`.

Because the mdBook is a maintained reference, its current-state contract is executable as well.
`scripts/check_book_current_truth.sh` binds the actor-direction and constrained-extraction claims to
their live code seams and canonical detail chapters; `LIVE-DOC-SIZE` runs that currency check on every
commit and CI build.

All four root-ledger migrations have landed. `CHANGES.md`, `DEVELOPMENT_NOTES.md`,
`LIVE_ACHIEVEMENT_STATUS.md`, and `RUST_CODEBASE_ANALYSIS.md` are bounded current views, while their
exact source capsules are the historical authority.
No historical content is removed before its identity, replacement route, consumers, and retrieval
procedure are proved.
The top-level README will remain a first-class landing page; changing detail and
chronology will route to controlled canonical destinations.

### Lossless rolling-ledger protocol

The four large root ledgers have an executable migration contract. `.4a` moved no record; `.4b` then
used that contract for the first atomic transition. Their record boundaries are not interchangeable:

| Live root | Whole-record boundary | Stable content outside records |
| --- | --- | --- |
| `CHANGES.md` | modern H3 records, then legacy H2 records, plus two named detached compatibility records | none |
| `DEVELOPMENT_NOTES.md` | every H2 after the H1 title | H1 prologue |
| `LIVE_ACHIEVEMENT_STATUS.md` | each top-level bullet in `Current snapshot` | H1/current heading plus the gap and generated validation trailer |
| `RUST_CODEBASE_ANALYSIS.md` | every H2 after `Purpose` | H1 and `Purpose` prologue |

`doctrine/live_document_size/rolling_ledgers.jsonl` pins the measured source identity, reviewed live
window, local record/line/byte/width limits, consumers, and future archive route for each root.
`scripts/check_rolling_ledger_protocol.pl` parses and reconstructs every byte, then derives the
planned live view from whole records and checks each pressure axis independently. Its grammar tests
also reject non-bullet content inside the status record region.

Each initial migration copied the exact pre-migration file into an immutable, repository-local source
capsule before shortening the stable root. The capsule manifest records its digest and dimensions; a
bounded index links both the current root and historical capsule; the checker retrieves and revalidates
both. Each capsule deliberately overlaps the retained current window so complete-source identity
remains independently reproducible. Future rotation seals only newly aged-out whole records, not
another full copy.

This matters especially for `LIVE_ACHIEVEMENT_STATUS.md`: project validation replaces a managed block
at its existing root path. The protocol therefore requires both validation projection markers and
keeps the entire generated trailer outside the rolled bullet region.

The two containment records initially written after the old 2026-03-31 `CHANGES.md` tail are not
silently reordered in historical evidence. The bounded live view will promote those exact records
after its newest prefix, while the source capsule retains their measured original ordinals and bytes.

#### `CHANGES.md` migration landed

At the `.4b` migration boundary, the root change ledger became 1,368 lines / 199,851 bytes instead of
32,682 lines / 2,629,033 bytes.
Its initial capsule at `docs/archive/rolling-ledgers/changes/source-through-2026-08-08.md` retains all
1,798 pre-migration records byte-for-byte under SHA-256
`d89809322857aab3c506dde1cc6caaf57e22d0349655b37ddaab1f7bf22ba994`. The live root holds the newest
85 capsule records, then the exact `.1`/`.0` compatibility records, with `.4b` as the first
post-capsule prepend. `docs/archive/rolling-ledgers/INDEX.md` is the bounded current/history route.

The main surface registry now classifies the root as a normal rolling ledger and the capsule as an
archive terminal. The generic checker skips archive pressure warnings—the capsule is immutable and
outside mandatory reads—but still enforces its exact ceiling; the rolling-ledger checker independently
reopens the file, verifies every metric and digest, checks the manifest/index, and proves the retained
root suffix. This is the retrieval contract, not a reliance on Git history alone.

#### `DEVELOPMENT_NOTES.md` migration landed

At the `.4c` migration boundary, the engineering-rationale root became 1,480 lines / 194,412 bytes
instead of 20,921 lines / 2,170,230 bytes. Its initial capsule at
`docs/archive/rolling-ledgers/development-notes/source-through-2026-08-08.md` retains all 1,601
pre-migration records under SHA-256
`76b51a3f450cdb1e764922dc366cf6ff55529cb95f7a6f410bfba1f1f378fedc`. The live root keeps its H1
prologue and newest 60 capsule records; `.4c` is the first post-capsule prepend.

The retained-record check is byte-sensitive beyond visible prose. During `.4c`, an ordinary edit at
the top of the derived root exposed an ownership ambiguity for the blank line between records 60 and
61. The complete-source capsule keeps that byte exactly, but a bounded view ending at record 60 must
not turn the removed successor's separator into a blank-at-EOF artifact. ADR 0008 and the renderer now
define one such terminal separator as boundary structure: it is omitted only at live EOF, while every
record-content byte must match. “Looks the same” remains insufficient; the exception is exact and
mechanical rather than a whitespace normalization.

#### `LIVE_ACHIEVEMENT_STATUS.md` migration landed

At the `.4d` migration boundary, the current-status root became 90 lines / 88,414 bytes instead of
1,960 lines / 581,239 bytes. Its initial capsule at
`docs/archive/rolling-ledgers/live-achievement-status/source-through-2026-08-08.md` retains all 1,920
pre-migration records under SHA-256
`b00ff5f5c4a29554a20eea9d901a95848d54a644749eba74ad9618798dd9bd6a`. The live root keeps its
H1/current heading and newest 50 capsule bullets; `.4d` is the first post-capsule prepend.

Status rotation does not own the material below `## Highest-priority remaining gap`. The complete gap
section, `## Validation Projection`, and both managed markers remain in the root outside the bullet
record region. The repository's focused project-validation test runs the real Rust writer with
temporary storage forced under `generated/tmp`, proving the managed block can still be replaced at the
stable path after migration. Capsule identity, suffix identity, marker literals, pressure, manifest,
and index routing are all rechecked by the unconditional doctrine gate.

#### `RUST_CODEBASE_ANALYSIS.md` migration landed

The Rust architecture root is now 1,056 lines / 89,727 bytes instead of 9,039 lines / 1,046,679
bytes. Its initial capsule at
`docs/archive/rolling-ledgers/rust-codebase-analysis/source-through-2026-08-08.md` retains all 1,350
pre-migration records under SHA-256
`95e1665628b615498e94f67d6dc6e0083d4d104a6ca8a815b4c23cf2f97cc7ce`. The live root keeps the H1,
complete Purpose prologue, and newest 60 whole H2 records.

No `.4e` record was added to that root. The migration changes documentation lifecycle, not the Rust
architecture, and the repository's impact-based routing contract reserves this surface for material
subsystem, integration, implementation, or current-risk changes. The task tree, change/status/rationale
ledgers, ADR, Knowledge Map fact, and this chapter record the migration without manufacturing an
architecture fact.

### Lifecycle controls

Every surface declares one of seven lifecycles:

- a `bounded_snapshot` is overwritten or reviewed in place under independent line, byte, and
  line-width ceilings;
- a `rolling_ledger` must roll whole records into indexed durable history before its live window
  becomes unbounded;
- a `partitioned_canonical` collection keeps bounded parts, count/aggregate controls, and an index or
  deterministic query contract;
- a `generated_projection` has both size ceilings and a freshness verifier against canonical inputs;
- a `frozen_legacy` record is hash-locked, while an `archive_terminal` is governed by an immutable
  manifest and is never an author-overflow destination;
- a `maintained_reference`, such as this mdBook, keeps bounded directly indexed semantic parts while
  exact task-owned aggregate-change authority permits legitimate product-scope evolution.

### Collection and projection closure order

The post-ledger census found that collection pressure is not one problem. The decision index and
mdBook membership/current-truth controls are already complete and remain in place. Other surfaces
need independent work in dependency order:

1. rebuild the task catalog as a concise index of every real task tree;
2. close direct fact, decision, research, and remaining canonical-collection navigation;
3. shard the generated Knowledge Map without losing question-key retrieval or derive-and-diff;
4. separate current roadmap direction and FSMGen requests from their historical chronology;
5. replace validation, source-registry, and corpus-KB currency labels with non-mutating producer oracles;
6. audit root maintained references against this book, then close aggregate membership/currentness.

At entry, `docs/TASK_TREE.md` links only 117 of 122 tracked task files. Four real trees are missing;
the fifth difference is the author template and must not appear as active work. Its widest row is
18,932 bytes because it mirrors execution history instead of acting as an index. No collection content
moves in the planning slice; each migration has its own source/retrieval proof.

#### Task catalog migration landed

`docs/TASK_TREE.md` now derives one concise row for all 121 real task trees, with `TEMPLATE.md` linked
separately for authors. The workflow contract remains outside the generated section. The index moved
from 167,583 bytes and an 18,932-byte widest row to 27,228 bytes and a 207-byte widest row while adding
the four missing real routes.

The catalog intentionally omits execution narrative. Open the selected tree for its frontier,
decisions, evidence, and commits; use `MEMORY.md` for the single resume pointer. The generator/checker
validates filename/H1 identity, status vocabulary, ordering, escaping, membership, and independent
title/row/section/count bounds. Its derive-and-diff and seven self-tests run on every live-document
doctrine gate.

#### Decision and fact-card navigation landed

The decision collection keeps its existing 30-line index: all nine ADR files are linked exactly
once, and the generic membership gate continues to enforce that route. Fact cards now have a
different, complementary route at `docs/knowledge/INDEX.md`. Its 150-line / 28,392-byte generated
catalog lists all 136 immediate cards by id, establishment date, status, and a bounded title preview.
Long historical titles are shortened only in the index; the linked card remains canonical and
unchanged.

The catalog count deliberately excludes `docs/knowledge/README.md` and the generated index itself.
The root question map reports 137 facts because it additionally scans ADR 0007, which has Knowledge
Map front matter. Thus directory files, fact cards, and cross-layer generated facts are three explicit
counts rather than one overloaded metric.

The focused generator/checker requires filename/id identity, constrained front matter, questions,
date, valid status (with the architecture-defined `current` default), evidence or reverify, stable
ordering, and independent field/row/count/index bounds. It runs its nine fail-closed cases and the
real derive-and-diff check through the live-document doctrine. The root README now offers both routes:
use the bounded catalog to browse known ids/titles, and the generated Knowledge Map to search by a
question. Sharding that still-large question map remains a separate migration.

#### Remaining canonical collection catalogs landed

No partitioned canonical Markdown collection now relies only on a Git query. The bounded
`docs/catalogs/` plane contains six generated direct indexes for 241 workflow-standard, root-
architecture, FSMGen-issue, research, corpus-KB, and KG-fixture members. Its 13-line landing index
links every generated catalog; the whole seven-file plane is 314 lines / 49,305 bytes, and its largest
part is the 167-line / 34,615-byte KG-fixture catalog.

Two collections keep more natural existing routes. `docs/TASK_TREE.md` directly covers all 122 task
files—121 real trees plus the separately linked author template—and the portable Knowledge Map bundle
uses its own README to link all four bundle documents. In total, the eight former query-only surfaces
now have direct membership proof over 367 canonical files without moving or copying their prose.

The doctrine now distinguishes internal membership from `external_membership`. An external index must
be one safe repository-relative Markdown file outside the member surface, must itself belong to a
separately classified bounded surface, and must link every member directly. This lets a large fixture
catalog remain outside fixture inputs and lets the task collection preserve one canonical catalog.
Fifty-three lifecycle/control-plane fixtures cover the positive path plus missing, stale, off-root,
and improperly internal external-index failures. The common catalog generator adds independent
path/title/row/member/index bounds and derive-and-diff; no generated catalog contains canonical facts.

#### Knowledge Map projection migration landed

The committed entry baseline was one 2,231-line / 1,038,010-byte generated file
with 138 facts and 978 question rows. An audit found only 977 unique questions: the same register-bit-
field question appeared in both a broad completeness scorecard and its focused lowering-gap card. The
focused card now owns that key, and the contract rejects any future question with two destinations.

The migrated read path keeps `KNOWLEDGE_MAP.md` stable as a small landing page. It links every
deterministically packed `docs/knowledge-map/questions-NNNN.md` shard and the separate fact-card
catalog. Search all questions with:

```bash
rg -i --glob 'questions-*.md' 'your terms' docs/knowledge-map
```

Each preserved question links directly to one canonical fact card. Question entries sort by UTF-8
bytes, wrap under a fixed physical-line limit, and omit repeated date/reverify fields; those fields
remain in the linked card. The repeated generated fact section also disappears because the bounded
fact catalog already owns id/title browsing.

The current 139 facts / 981 unique questions occupy seven shards.
The landing is 19 lines / 898 bytes; the largest shard is 300 lines / 30,427 bytes, and the complete
question set is 2,000 lines / 183,756 bytes. Independent limits govern input ids, paths, question
bytes, line width, files, lines, and bytes; explicit 4,096-line / 393,216-byte aggregate caps prevent
unused per-shard capacity from becoming banked growth. A SHA-256 identity covers participating paths
and content. The hook stages every current output and tracked deletion, while the checker regenerates
under repository-local `generated/` and compares exact root/shard membership plus content. Eight
contract cases, eight portable-bundle integration cases, and the common lifecycle suite prove parsing,
collision rejection, ordering, wrapping, rollover, stale cleanup/staging, bounds, drift, and no
temporary residue. A ninth contract case guards raw-byte identity after the initial simulator was
found hashing scalars consumed by checked UTF-8 decoding. Every bootstrap, CI, bundle, doctrine, and mdBook reader moved in the same slice;
the generated surface is now normal rather than transition debt.

Lines, bytes, file count, collection totals, and maximum content-line width are independent axes.
Existing oversized ledgers are explicit transition debt: their measured baseline cannot move, and a
separate bounded allowance exists only for the containment program's continuity updates until the
owning migration lands. A legacy ceiling is not advertised as health.

Documentation synchronization is now impact-based. Every completed slice updates
its owning task-tree leaf. Other surfaces change only when their own truth changes:
the resume pointer when the next action or in-flight state changes, the status page
when product status changes, and the book when user understanding changes. Editing
an unrelated document merely to show that it was reviewed is prohibited.

The resume pointer also does not copy Git's current revision. Run
`git rev-parse HEAD` and `git status --short --branch` when resuming; `MEMORY.md`
owns only the active unit, concise current state, next action, in-flight work, and
blockers.

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

The root live docs are more operational, but they are still governed.
They track:

- current baseline scores
- active roadmap slices
- implementation notes
- handoff context
- recent decisions
- crash recovery breadcrumbs

That means root docs may mention temporary states, current artifacts, local blockers, or implementation
details that would be too specific for the public book. It does not mean one mandatory file may grow
forever; chronology and large evidence move to indexed, query-first storage at lifecycle boundaries.

## Why the split exists

The book is what the world should see.
It should explain the product clearly and transparently.

The root docs stay live and operational because they are part of the project's continuity system for
active development and session recovery. Their working sets remain bounded; durable detail stays
recoverable through the registered task, decision, fact, shard, projection, or Git-history route.

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

Those docs can be more current and operational because their main job is continuity. Detail still has
to obey the registered lifecycle: update a bounded snapshot, append only within a rolling window,
partition canonical evidence, or route exact history to its controlled terminal.

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

## The README is a bounded landing page

Root `README.md` is intentionally smaller than either documentation plane. It keeps only the project
purpose and audience, top-level scope, prerequisites, one verified first-use path, stable architecture,
and navigation. Current delivery state belongs in the roadmap and task trees; command and feature
detail belongs in this book; chronology belongs in Git or a governed historical surface.

The repository-owned `README_POLICY.md` makes that boundary explicit. Its unconditional doctrine
check independently enforces 150 lines and 5,800 bytes, values derived from the reviewed 127-line,
4,834-byte survivor rather than copied from another project. A data-only route inventory distinguishes
reader navigation from places authors may put changing detail. Missing, off-repository, duplicated, or
uncontrolled routes fail the same pre-commit and CI doctrine driver even when `README.md` itself was not
changed.

This is containment, not documentation reduction: material removed from the old landing page was
already duplicated by richer canonical homes. The user manual remains the complete maintained product
surface, while task state and rationale remain available through their dedicated continuity layers.

## Closed task trees — how each was implemented and verified

### `SIGNOFF-REMEDIATION` — the doctrine that keeps `main` always-shippable

If you `git pull main` and run `scripts/run_ci.sh`, it should
be green. That sentence is meant to be unconditional. The
`SIGNOFF-REMEDIATION` tree is the work that paid for it, and
the doctrine that keeps it true.

#### The user-facing guarantee

> **`scripts/run_ci.sh` on `main` is always green. If you
> branch from `main`, your starting point is clean —
> formatted, clippy-clean, every test passing. Every leaf
> that lands has to keep it that way; nothing about a leaf is
> "done" until CI is green again.**

That's the doctrine. It's non-negotiable: a leaf isn't
complete until `scripts/run_ci.sh` runs through to green at
the leaf's commit.

#### Why this isn't free either

When `main` drifts to "fails CI but probably works", several
nasty things happen at once:

- **New leaves can't tell whether they broke something.**
  Running CI on a branch and getting failures, the author
  has no way to know which failures are theirs vs which were
  inherited from `main`. The signal-to-noise on every
  subsequent leaf collapses.
- **The bar slips, quietly.** "We'll fix the 3 clippy
  warnings later" becomes "we'll fix the 7", becomes "we'll
  fix the 25." There's no natural moment to stop the slide;
  there's always a more urgent leaf.
- **Idiomatic fixes get replaced by `#[allow]`s.** Under
  pressure to make CI green again, the temptation is to
  silence the lint rather than fix the code. The lint is
  there for a reason; silencing it loses the signal.

`SIGNOFF-REMEDIATION` showed up after the bar had drifted to
25 clippy errors plus formatting failures. The tree
restored CI to green and turned the bar into a doctrine that
prevents the drift from happening again.

#### What this tree fixed (concretely)

When the tree opened, HEAD failed both `cargo fmt --all
--check` and `cargo clippy -- -D warnings` (the second with
25 errors). The remediation followed two rules strictly:

- **Idiomatic fixes only.** Every clippy lint was addressed
  by rewriting the code the way clippy wants it (use
  `Result::ok()` not a match; use `.contains(&x)` not
  `.iter().any(|y| *y == x)`; etc.). No blanket
  `#[allow(clippy::…)]` at module level.
- **Narrow, recorded `#[allow]` only where genuinely
  inapplicable.** When a lint truly doesn't apply (e.g.
  `#[allow(clippy::too_many_arguments)]` on a verbose test
  helper), the allow goes on the function or line — never
  the module — and the reason is recorded in a comment.
- **Zero production behaviour change.** Every fix had to
  preserve what the code does. The tree was a CI restoration,
  not a refactor.

After this tree, the bar held for every subsequent close —
the long R16 program shipped 40+ leaves with
`scripts/run_ci.sh` green at every single commit, because the
doctrine and the per-leaf signoff discipline kept it green.

#### How the doctrine is enforced now

The doctrine became **structural** through `COMMIT.md`'s
"Required Commit Workflow":

- Every commit's workflow ends with `scripts/run_ci.sh`. The
  step isn't optional; it's part of what a commit means.
- Per the close-rule (and the `BOOK-METHOD-DOC`
  reinforcement), a tree's close leaf has the same CI
  requirement: green or it isn't closed.
- Idiomatic fixes only is recorded as the canonical way to
  land clippy/fmt deltas; `#[allow]` is permitted only at
  line/function level when the lint is genuinely
  inapplicable, with a brief recorded reason.

The result: a contributor who follows the workflow doesn't
have to choose between "ship it" and "keep CI green" — the
workflow makes them the same choice.

#### What this buys you, as a SpecForge user

- **You can `git pull main` and start working immediately.**
  CI is green from your starting commit; failures you see
  while developing are yours, not inherited drift.
- **Every leaf you read about in the book or in
  `CHANGES.md` shipped under a green CI gate.** That's not
  decoration; it's a hard precondition the workflow
  enforces.
- **clippy / fmt deltas are small and idiomatic.** If you
  send a contribution, the precedent is clear: fix the lint
  the idiomatic way; reach for `#[allow]` only when the
  lint genuinely doesn't apply, and only at the narrowest
  scope.

*Authoritative tracking:*
`docs/tasks/SIGNOFF-REMEDIATION.md`. The doctrine is
recorded in `COMMIT.md`'s "Required Commit Workflow" — every
commit closes on `scripts/run_ci.sh` green.
