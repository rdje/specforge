# Terminal task-evidence containment design

Owning leaf: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.10a` (PROBE/DOC). This report selects the
`.10b.i`/`.10b.ii` implementation contract. It changes no task topology, archive, checker, threshold,
ceiling, product behavior, or historical record.

## Pressure and update-rate boundary

After `.10a` activation, `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` is 2,451 lines /
232,649 bytes with a 746-byte maximum content line. Its `task_evidence` health target and ceiling are
3,000 lines / 278,528 bytes; warning begins at 2,400 lines / 222,823 bytes and mandatory rollover at
2,700 / 250,676. The file is 51 lines and 9,826 bytes past warning, leaving 248 lines / 18,026 bytes
of pre-rollover headroom.

The tree began at 214 lines / 13,414 bytes and reached its present size through 38 commits. Across the
last nine update intervals it grew by an average 49.7 lines / 5,370 bytes; the ten-commit window added
447 lines / 48,334 bytes. At that recent rate, only about 3.4 ordinary updates fit in the remaining byte
margin. `.10b` therefore may not defer containment or spend the margin on another long embedded design.

The next-largest task file is `docs/tasks/PDF-VARIANT-DIGESTION.md` at 2,393 lines / 222,616 bytes,
only 207 bytes below its warning. Unlike this program, that tree is active and cannot use a terminal
summary. Its next append is blocked on a separately owned active-tree containment design after `.10b`
closes; this leaf does not silently apply the terminal topology to it.

## Semantic-region census

The source divides into four roles, not four independently current documents:

| Region | Lines | Bytes | Retrieval role |
| --- | ---: | ---: | --- |
| prologue, metadata, goal, baseline, acceptance, tree, frontier | 651 | 49,843 | stable identity plus accumulated execution state |
| decisions | 356 | 33,460 | durable rationale, much of it already routed to ADRs/facts |
| blockers and slice-specific contracts/audits | 1,148 | 85,976 | revision-bound implementation history |
| verification, commit, and changelog logs | 296 | 63,370 | 121 table rows plus exact completion evidence |

The current front half is itself accumulated history: 54 status-bearing task records coexist with one
two-row frontier. Splitting on H2 headings would retain a 49,843-byte mandatory read and scatter the one
task-tree authority across multiple catalog members. A newest-first rolling window would also be wrong:
the tree is a finite dependency graph, not an indefinitely ordered ledger. The program has exactly one
remaining implementation parent and can become terminal when it closes.

## Reader and authority census

Nineteen tracked files outside the task and archive trees contain 21 exact references to the stable task
path. Their needs fall into four classes:

- `docs/TASK_TREE.md` and `scripts/check_task_tree_catalog.pl` read only the stable H1/id/title and first
  metadata status. They must continue to use the bounded root.
- `MEMORY.md` and `ROADMAP.md` need only current frontier/outcome routing. At closure they use the bounded
  root; no bootstrap reader needs the capsule.
- ADRs, fact cards, research records, `CHANGES.md`, and the FSMGen-feedback consumer census cite the
  program as evidence. The bounded root retains stable routes and the `docs/FSMGEN_FEEDBACK.md` literal;
  the archive index makes detailed evidence directly recoverable.
- `containment_task_verification_evidence` classifies the root's `## Verification Log` and exact table
  header as maintained revision-bound evidence. The compact root retains one final verification table;
  the capsule preserves all 80 historical verification rows and 39 commit rows.

No reader requires the current 232,649-byte file as a mandatory startup read. No writer other than the
task-tree workflow appends to it.

## Selected two-commit topology

ADR 0018 selects a terminal boundary with a committed provenance seam:

1. `.10b.i` implements the neutral contract/checker and self-tests while the source remains live. It
   completes and commits the pre-migration task record, then records its exact SHA-256, metrics, required
   semantic markers, and commit provenance.
2. `.10b.ii` copies that committed source byte-for-byte to
   `docs/archive/tasks/live-document-size-containment-adoption/source-through-2026-08-09.md`, creates a
   bounded `INDEX.md` plus JSON manifest, and replaces the stable root with a closed summary.
3. The compact root keeps the canonical H1, `done` metadata, goal/outcome, activity completion map, empty
   frontier, decision/ADR routes, one final `## Verification Log`, commit summary, and direct archive link.
4. The checker validates exact capsule identity/metrics, provenance metadata, safe same-volume paths,
   root and index bounds, required once-only sections, no active/pending frontier, exact direct links, and
   manifest/index/root agreement. Mutation tests cover hash, metric, marker, status, route, traversal,
   overflow, and archive-change failures.

The compact-root local contract ratchets this path to 256 lines / 32,768 bytes / 512 maximum line bytes
as health and 384 / 49,152 / 1,024 as ceilings. The archive index uses 64 / 6,144 / 256 health and
96 / 8,192 / 512 ceilings. The exact capsule metrics become both health and ceiling because it is an
immutable terminal. The broader `task_evidence` collection limits remain unchanged for other trees.

## Atomic migration and acceptance

`.10b.ii` must prove the capsule bytes equal the `.10b.i` committed source before replacing the root;
classify the root, index, and terminal exactly once; switch every semantic reader in the same commit;
regenerate the task and fact catalogs plus Knowledge Map; run the checker mutations and complete doctrine/
CI gates; and leave no temporary source copy. The user-owned `.claude/settings.json` remains untouched.

The migration closes this program permanently. New containment work opens a new top-level task tree, and
the active PDF-variant tree cannot receive another task update until that new tree owns a non-terminal
partition design.

## Acceptance disposition

- **Reproduce/measure:** exact source pressure, semantic regions, 38-commit growth, recent deltas, 19-file /
  21-occurrence readers, and the adjacent active-tree warning boundary are pinned above.
- **Root cause:** one finite program mixed current navigation with all historical design and evidence in a
  single mandatory task member; the generic collection budget did not provide a per-tree terminal route.
- **Addressed design:** a committed source boundary, exact terminal, bounded closed root/index, and neutral
  fail-closed checker preserve every byte while removing the mandatory large read.
- **No regression:** this leaf changes planning/current-truth documentation only. `.10b.i` and `.10b.ii`
  own executable enforcement, exact copying, reader migration, mutation proof, and final closure.
