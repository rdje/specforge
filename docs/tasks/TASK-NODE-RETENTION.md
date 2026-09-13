# TASK-NODE-RETENTION: every other memory layer is guarded; the contents of a task tree were not

## Metadata

- Tree ID: `TASK-NODE-RETENTION`
- Status: `active` (`2026-09-13`; `.0` closed)
- Roadmap lane: repository durability and portability (sibling of `MEMORY_ARCHITECTURE.md` enforcement)
- Created: `2026-09-13`
- Last updated: `2026-09-13`
- Owner: repo-local workflow

## Goal

The task-trees under `docs/tasks/` are **layer B** of `MEMORY_ARCHITECTURE.md` — the project's work memory,
and the only record of *why* a closed leaf was closed the way it was. Every other layer has a mechanical
guard: the resume pointer has `memory-arch`, the fact cards have the Knowledge Map gate, the catalog INDEX
has `check_task_tree_catalog.pl`, the decision records have their index check. The **contents** of a tree
file had none, so an editing mistake could delete a node and every registered doctrine would still report
green.

Make a node's disappearance mechanically impossible to land silently, without refusing the legitimate
operations — a rename into descendants, and a deliberate, declared retirement.

## Non-Goals

- Do not freeze node text. Nodes are amended constantly; this tree is about a node CEASING TO EXIST.
- Do not forbid removal. Make it declared, owned and dated, the way
  `doctrine/chain_currency/retained_bundles.json` makes a bundle reclamation accountable.
- Do not re-implement the catalog index check. That governs which TREES are listed; this governs the nodes
  inside them.

## How this was found

Not by audit. `EXTRACTION-QUALITY-GAUGE.3k.2j` (`ab2c6ee0`) replaced one node by splicing from its `- ID:`
line to the next `### Acceptance Checklist` header. That boundary is wrong — every node in between went
with it — and **eight nodes were deleted**: `EXTRACTION-QUALITY-GAUGE` `.3j`, `.3k.2c`, `.3k.2f`, `.3k.2g`,
`.3k.2h`, `.3k.2i`, `.3k.3` and `.3k.4`, four of them closed leaves' complete records. All fourteen
registered doctrines passed on that commit. It surfaced one commit later only because an unrelated edit to
one of the deleted nodes failed to find its anchor; nothing in the repository was looking.

## Task Tree

- ID: `TASK-NODE-RETENTION` · Status: `active` (`2026-09-13`) · Children: `.0`

- ID: `TASK-NODE-RETENTION.0` · Status: `done` (`2026-09-13`, DOCTRINE) · Goal: **a node may be renamed,
  split or deliberately retired; it may not vanish.**
  Shipped `scripts/check_task_node_retention.py`, registered as `TASK-NODE-RETENTION|gate` with its
  `DOCTRINE_ENFORCEMENT.md` §10 row. It compares the node-id set `HEAD` carries across `docs/tasks/`,
  `docs/task-catalog/` and `docs/archive/` with the working tree's, and a vanished id is a breach unless a
  DESCENDANT id appeared in its place or `doctrine/task_nodes/removals.jsonl` names it with an owning leaf,
  a date and a reason. The registry is schema-closed and carries its own `expected_removals` total, so a
  record cannot be added or dropped without moving the declared number.
  **The discriminator was derived from history, not invented.** Over the last 200 revisions a node id
  vanished from every tracked task surface in exactly **two**: `3c17ae5c`, where
  `CLAIM-VERIFICATION-ADOPTION.7.2` split into `.7.2.0` and `.7.2.1` — a correct operation — and `ab2c6ee0`,
  the accident above. A naive "no id may vanish" rule fires on both. The descendant rule was checked against
  both instances **before it was written** and reproduces both verdicts: replaying each revision's real
  before/after id sets, `3c17ae5c` is GREEN and `ab2c6ee0` is RED naming all eight. A control with a tracked
  known-bad case and a tracked known-good case, both from this repository's own history.
  **Observed RED from the driver**, not only from the script: deleting one live node makes
  `check_doctrines.sh` report `FAIL TASK-NODE-RETENTION` naming that node; restored green. Six self-test
  cases pin the discriminator's edges — the split, the outright deletion, a declared removal covering only
  its own id, a sibling's descendant not covering a lost node, `A.1` not covered by `A.10` (the dotted
  boundary), and no-`HEAD` not being a breach — with the case total declared independently of the case list.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `TASK-NODE-RETENTION.0`

## Acceptance Checklist (enforced) — `TASK-NODE-RETENTION.0`

- [x] **REPRODUCE / MEASURE** — the defect reproduced from history: replaying `ab2c6ee0`'s real before/after
  node-id sets yields 8 unaccounted losses, and `git show` confirms all fourteen doctrines were green on
  that commit. The population is measured rather than assumed: 2 vanishing revisions in 200, one of them a
  legitimate split.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_doctrines.sh` registers no check over the CONTENTS of a
  `docs/tasks/*.md` file. `scripts/check_task_tree_catalog.pl` governs which trees the catalog lists and
  never reads their node ids; `check_live_document_size.pl` governs size and route, not identity. Layer B
  was the one memory layer with no identity guard.
- [x] **ADDRESSED (verified)** — `TASK-NODE-RETENTION|gate` registered; the driver executes 15 doctrines and
  reports PASS. **Observed RED from the driver** on a live node deletion, naming the exact node; restored
  green. **Observed GREEN on the legitimate operation**, replayed from `3c17ae5c`'s own id sets, so the rule
  is a discriminator and not a tripwire. `--self-test` 6/6 with the total declared independently.
- [x] **NO REGRESSION** — no production code touched; `crates/` byte-identical. All 15 gate-tier doctrines
  PASS including the meta-check that every registered enforcer exists and is executable. `cargo fmt`,
  `clippy -D warnings` and the workspace suite stay green.
- [x] **GENERICITY (ADR 0006)** — repository-process enforcement; no extraction rule, vocabulary, document,
  protocol or vendor name. The check is portable: it reads a node-id syntax the task-tree convention already
  defines and needs nothing from this project's domain.
- [x] **LOCKSTEP** — the check, the registry line, the §10 row, this tree and its `docs/TASK_TREE.md` index
  row are written together. The mdBook's doctrine-enforcement chapter routes to `DOCTRINE_ENFORCEMENT.md`
  for the registry rather than mirroring rows, so it needs no edit.

## Changelog

- `2026-09-13` — `.0` closed. Opened and closed in the same slice because the defect that motivated it had
  already happened and been repaired; what remained was to make the next one impossible to land silently.
