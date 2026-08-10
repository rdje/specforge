---
id: corpus-task-evidence-containment-design
title: The active corpus task requires a bounded root, seven semantic parts, and exact provenance before refresh 49
answers:
  - "why can CORPUS-COVERAGE not accept refresh 49 yet"
  - "what is the exact CORPUS-COVERAGE task evidence boundary"
  - "how much headroom remains in the corpus coverage task file"
  - "what did the corpus task evidence containment census find"
  - "how many corpus task ids and source regions must containment preserve"
  - "which topology contains the active corpus coverage task"
  - "why not create a new corpus coverage continuation tree"
  - "why is an exact archive alone insufficient for the active corpus task"
  - "who reads and writes docs tasks CORPUS-COVERAGE md"
  - "what is the current corpus refresh frontier after refresh 48"
  - "why was corpus task maximum line width corrected from 4747 to 4746"
  - "is the corpus task migration contract locked"
  - "has the corpus task evidence migration landed"
date: 2026-08-10
status: current
tags: [documentation, containment, task-tree, corpus-coverage, continuity]
evidence: docs/research/corpus-task-evidence-containment-census.md; docs/tasks/CORPUS-TASK-EVIDENCE-CONTAINMENT.md
reverify: "git show d78d842e4e7fc8d1fd902937cf6902f07d96b68e:docs/tasks/CORPUS-COVERAGE.md | shasum -a 256"
---

`docs/tasks/CORPUS-COVERAGE.md` is locked before containment at commit `d78d842e`, Git blob
`d7ac9aa2c07723cb4a7a8f3a15332ef84fe09f07`, SHA-256
`5d7acb0c973a75d821c9d5a963aac5c3899da19261dcf6427274cb123804123f`, and 2,308 lines / 277,636 bytes /
4,746 maximum content-line bytes. The registered ceiling is 278,528 bytes, leaving 892 bytes; the smallest of the
latest fourteen growth intervals was 1,215 bytes, so another ordinary refresh record cannot fit.

The width is 4,746 because the doctrine counts content bytes without LF or optional CR. The initial ownership
leaf reported 4,747 by counting LF; `.2` root-caused and corrected only that notation. Commit, blob, SHA, line
count, total bytes, and the untouched source all remain exact.

The census partitions every source byte into seven natural regions, all below 59,259 bytes before marker
scaffolding, and finds 48 formal fully qualified task ids. Forty-one also occur in completion-subject history and
use `legacy` routes; seven formal container ids absent from those subjects use source-backed `structural` routes.
There are no commit-only or shorthand-only routes. At census boundary `3a40b152`, 42 tracked files cite the
stable path (33 fact cards, three book chapters, three root live docs, one task owner, two immutable capsules),
none with a fragment or query, and no executable opens the target. The only writer is the manual task-tree and
`COMMIT.md` slice transaction.

The current product truth is 48 of 56 real chip-spec refreshes done, eight remaining, stages 80/20/80/79, and
59/59 emitted ISFs strict-clean. The source metadata is one refresh stale, while row 48, the cumulative table
statement, newest changelog, roadmap, status, book, and committed result agree. Literal stale text remains exact
history; a bounded root will normalize current authority. Containment does not select #49, so the migrated root
will name no eligible product leaf and direct the next clean slice to create `CORPUS-COVERAGE.2.49`.

A new top-level continuation is rejected because it would break the established `.2.<refresh>` namespace or
misstate the active `.2` parent. Archive-only is rejected because unique completed task authority would become a
277 KiB terminal-only read. A chronological ledger cannot represent task hierarchy. The selected topology keeps
the stable active root over seven semantic legacy parts and an exact source capsule, using the existing neutral
active-task checker with a separate corpus-specific contract. Future refreshes atomically update root, one active
part, index, and manifest; legacy parts seal and the active part splits at a refresh boundary before rollover.

The complete contract is `doctrine/live_document_size/corpus_task_evidence.json`. It authenticates boundary
commit `f1b202f2`, Git blob `d7ac9aa2…`, the exact source SHA/metrics, all seven region hashes, 41
completion-history `legacy` routes, and seven source-backed `structural` container routes. The guarded transaction
advanced it from `source_locked` to `migrated`, writing a 65-line / 2,822-byte root last, a 75-line / 4,349-byte
index, seven parts totaling 2,357 lines / 279,157 bytes, a 488-line manifest, and an exact source capsule. All
parts are below warning and every source region reconstructs byte-for-byte. The unconditional driver invokes this
contract separately from the earlier active-task contract, while three dedicated live-document surfaces govern
the corpus index, parts, and archive terminal.
