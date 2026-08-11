---
id: corpus-refresh-frontier-derivation
title: The corpus refresh frontier derives the cohort and gates an exact root-neutral lifecycle partition
answers:
  - "how do I derive how many corpus refreshes remain (run scripts/check_corpus_frontier.sh; it derives the SourceIR cohort and requires every member in exactly one explicit refreshed or remaining set)"
  - "how many corpus refreshes are done and how many remain (52 of 57 done and five remaining as of 2026-08-11; re-run the gate rather than trusting an older count)"
  - "why was nvme_base_specification_2_0a_2021_07_26 missing from the corpus refresh frontier (the old count was decremented rather than re-derived, so an expired denominator adjustment silently removed it for twenty-two slices)"
  - "why must source-library paths not determine whether a corpus document was refreshed"
  - "does moving a PDF from the boot volume to SSD complete a current-binary corpus refresh"
  - "what gates the corpus refresh frontier / what is the CORPUS-FRONTIER doctrine"
  - "what does the corpus frontier refreshed list mean"
  - "must a corpus refresh update the frontier declaration"
  - "why is the in-repo corpus tree outside the host-library refresh cohort"
  - "can a correct refreshed count hide a missing corpus document"
date: 2026-08-11
status: current
tags: [corpus-coverage, census, task-tree, continuity, chain-currency, project-data-locality]
evidence: scripts/check_corpus_frontier_census.pl; doctrine/corpus_frontier/census.json; docs/tasks/corpus-coverage/frontier-census-integrity.md; docs/tasks/CORPUS-COVERAGE.md
reverify: "bash scripts/check_corpus_frontier.sh && perl scripts/check_corpus_frontier_census.pl --report"
---

The `.2` refresh program's progress is not a carried number. `CORPUS-FRONTIER` derives the cohort from every
persisted `generated/source_ir/*/source_ir.json`: documents whose `source.requested_path` begins `corpus/` are
the 21 tracked in-repository gold/evaluation sources and are outside the host-library refresh program; the other
57 documents form the cohort.

Lifecycle is an exact, explicit partition in `doctrine/corpus_frontier/census.json`:

| Quantity | Authority | Count (`2026-08-11`) |
| --- | --- | ---: |
| Persisted documents | one SourceIR per key | 78 |
| Outside the refresh cohort | repository `corpus/` source | 21 |
| Refresh cohort | derived persisted set | 57 |
| Refreshed | explicit completed set | 52 |
| Remaining | explicit unfinished set | 5 |

The gate proves that the refreshed and remaining sets are duplicate-free, disjoint, entirely within the
derived cohort, and together cover it exactly. It also requires every retained cohort bundle to be refreshed,
forbids retention on a remaining key, and checks that the root task file states the same counts. Its 13-case
self-test kills missing members, overlaps, duplicates, out-of-cohort entries, retention disagreements, count
drift, and prose drift.

Source location deliberately does **not** classify lifecycle state. The earlier gate used departure from a
retired workstation prefix as an omission witness. That coupled locality repair to extraction history: after a
required library move, an unfinished document could name its correct new source and become invisible to the
omission scan. `SPEC-TO-INTENT-ALIGNMENT.4b` removed that coupling. The self-test now gives refreshed and
remaining documents the same synthetic SSD root and still distinguishes them solely through the lifecycle
partition. Moving or repairing a source path never completes a current-binary refresh.

The historical defect remains instructive. From `.2.29` through `.2.51`, one document was lost because each
slice decremented the previous count and never re-derived the denominator. Re-measuring rows inside an already
incomplete list cannot recover a missing row. The whole-cohort partition is what prevents recurrence: deleting
a remaining key now creates an uncovered cohort member even if its source path is perfectly current. A refresh
must update the exact lifecycle declaration in the same transaction or the doctrine fails closed.
