---
id: partitioning-a-task-tree-has-a-fixed-registration-price
title: Partitioning a task tree costs four surface records, one driver entry and two census records per current surface — and three of those numbers are refused if you guess them
answers:
  - "what does it cost to partition a task tree under the active-task-evidence contract (four surfaces in surfaces.jsonl, one line in check_task_evidence_contracts.sh, two census records per CURRENT surface, expected_current_surfaces +3)"
  - "why is a partitioned tree's archived capsule not counted in expected_current_surfaces (its state is terminal, so only index/parts/route_parts are current)"
  - "why did the surface registry refuse my new partitioned-tree parts record (a surface's lines_total and bytes_total must equal files x lines_each and files x bytes_each, or declare an aggregate_composition)"
  - "which file must the parts surface's census evidence record name (the collection's ALPHABETICALLY first part file, not the first in contract order)"
  - "why does a task-evidence contract declare parts lines_total below files x lines_each when the surface registry forbids that (the contract has its own portable caps of 9600 lines and 1179648 bytes, which sit below the product; the two authorities are sized independently and the stricter one binds)"
  - "in what order do I run a task-tree partition (write the closing record, commit, correct every node Status, commit, lock, migrate)"
date: 2026-09-17
status: current
tags: [live-document-size, task-tree, doctrine, partition, surfaces, claim-verification, live-document-pressure-headroom]
evidence: doctrine/live_document_size/surfaces.jsonl; doctrine/claim_verification/current_claim_census.jsonl; scripts/check_task_evidence_contracts.sh; docs/tasks/live-document-pressure-headroom/current-and-open-work.md (.31)
reverify: "perl scripts/check_live_document_size.pl && perl scripts/check_current_claim_census.pl --check && bash scripts/check_task_evidence_contracts.sh"
---

Partitioning a tree is not free, and every part of the price is refused rather than absorbed if it is
guessed. Measured end to end while partitioning `EXTRACTION-QUALITY-GAUGE` on `2026-09-17`.

## The four surfaces, and why only three are current

`surfaces.jsonl` gains `<tree>_task_evidence_index`, `_parts`, `_route_parts` and `_archive`. The archive
record's `state` is `terminal`, so it is not a *current* surface: `expected_current_surfaces` in
`current_claim_census.jsonl` moves by **three**, not four, and the census gains **two records per current
surface** — one `surface` record and one `evidence` record, six in all. The archive gets none.

## Three numbers the registries compute for you, and refuse if you don't

**A surface's totals are a product, not a judgement.** The registry refuses a record whose
`lines_total` is below `files x lines_each` (or `bytes_total` below `files x bytes_each`) unless it
declares an `aggregate_composition`. A partitioned-parts record therefore reads `16 x 768 = 12288` and
`24 x 896 = 21504`, not whatever the tree expects to use.

**The contract's own totals are lower, and that is not a contradiction.** `check_active_task_evidence.pl`
caps a contract at 9,600 lines and 1,179,648 bytes total, both *below* the same product. The two
authorities are sized independently against different portable caps; the stricter one binds, which is the
contract.

**The parts evidence record must name the collection's ALPHABETICALLY first part file.** Pointing it at
the first part in contract order is refused with `frozen candidate '<surface>:<view>:<path>:1' lacks
exact evidence`, naming the file it actually wanted. `live-document-pressure-headroom` reads
`book-chapter-routing.md` for the same reason, which looks arbitrary until you sort the list.

## The order, which is forced rather than chosen

A migration seals every marked payload byte-exact against the archived capsule, and
`validate_route_lifecycles` refuses a leaf declared twice in its primary part, so **a leaf cannot close
itself inside the transaction it seals** and **every node's Status must already be true when the source
is locked**: the migration publishes each lifecycle as derived fact. Write the closing record, commit;
derive every node's lifecycle and correct what disagrees, commit; then migrate. Both corrections are
cheap before the lock and impossible after it.

Links: [[node-status-has-two-declaration-shapes]], [[live-surface-edit-bookkeeping-chain]],
[[claim-verification-task-evidence-migrated]].
