---
id: a-task-evidence-part-whose-leaves-are-all-closed-is-archived
title: A task-evidence part whose every routed leaf is closed may move byte-exact to the tree's archive directory as an archived part, sealed where it was live and counted by an archive surface instead of the live parts total
answers:
  - "what is an archived task-evidence part (state archived, archived_from, sealed_commit, git_blob)"
  - "how do I bring a task tree's parts bytes_total or lines_total warning down (archive the all-closed parts)"
  - "why can't a task tree's live part total be lowered by adding parts or raising the target (ADR 0032; the 1179648-byte portable cap)"
  - "which parts may be archived (every leaf it routes closed, not the current frontier's part, live copy deleted)"
  - "how is an archived part verified (its bytes equal archived_from at sealed_commit, an ancestor of HEAD)"
  - "where do I record a correction to a leaf whose part is archived (a new record in a live part)"
  - "why does the gap owner check find a closed leaf under docs/archive/tasks (task_evidence_declares)"
  - "what is erg_task_evidence_archived_parts"
  - "what is EXTRACTION-REPRESENTATION-GAPS.56, .56.a and .56.b"
  - "does archiving parts contradict ADR 0045 (no tree is archived for capacity; the tree stays live)"
date: 2026-10-21
status: current
tags: [task-evidence, live-document-size, archive-terminal, containment, partitioned-tree, adr-0032, extraction-representation-gaps]
evidence: scripts/check_active_task_evidence.pl (validate_archived_part; validate_part_and_regions; validate_migrated's archived_part_metrics; render_index; archive_fixture_activity_part; the archived-part self-test cases); crates/specforge/src/ir/trajectory.rs (task_evidence_declares); doctrine/live_document_size/extraction_representation_gaps_task_evidence.json; doctrine/live_document_size/surfaces.jsonl (erg_task_evidence_parts, erg_task_evidence_archived_parts); docs/tasks/extraction-representation-gaps/packet-and-memory-leaves.md (.56, .56.a, .56.b)
reverify: "perl scripts/check_active_task_evidence.pl --self-test && perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/extraction_representation_gaps_task_evidence.json --check"
---

A partitioned task tree grows by one leaf record and one acceptance checklist per slice. Rolling over to a new part
bounds each file, but the parts total keeps growing: LDPH `.14e` kept `bytes_total` and `lines_total` as the bound
on what a tree holds live, ADR 0032 found that adding parts never lowers that total, and the contract's ceiling is the
portable cap of 1,179,648 bytes. ERG reached 84.1% of its 786,432-byte target on `2026-10-21`. Half of it was
history: eight parts routed only closed leaves (`EXTRACTION-REPRESENTATION-GAPS.56`).

**The transition (`.56.a`).** A part whose every routed leaf is closed may become `state: archived`. Its file moves
byte-exact from `collection_directory` to `archive_directory`, keeping the state literal it had while live. The
contract part names `archived_from` (the live path), `sealed_commit` and `git_blob`. `check_active_task_evidence.pl`
checks four things. The file equals `archived_from` at `sealed_commit`, which must be an ancestor of HEAD. The live
copy is gone. No routed leaf is open. The part is not the current frontier's. Its source regions still resolve once
each. An archived part leaves the live parts total (`archived_part_metrics` reports it), and the index lists it with
*— archived*. A leaf it holds is amended by a new record in a live part, never by editing the archive.

**Registry.** The archived files drop out of the tree's parts surface and join one `archive_terminal` collection
surface per tree (`erg_task_evidence_archived_parts`), whose exact bounds are 8 files with totals of `files ×` the
per-file bounds (ADR 0032). A later archival extends that surface's targets instead of adding a registry record:
the census notes that every new archive record raises the registry's floor. The current-claim census excludes
archive terminals, so only the parts surface's evidence moves to its new first member.

**Consumers.** The gap owner check (`trajectory.rs` `task_evidence_declares`) also reads the tree's archive directory
(`docs/archive/tasks/<tree>/`), so a gap owned by a closed leaf in an archived part still resolves. Every reference to
a moved path is repointed (24 KM cards for ERG); `archived_from` keeps the live path.

**Measured `2026-10-21` (ERG):** 17 parts and 661,007 live bytes became 9 live parts with 337,798 bytes (43.0%) plus
8 archived parts with 327,205 bytes. The `bytes_total` warning is gone. The `bytes_each` warning (85.4%) is the
rolled-over `clause-and-register-leaves`. **CORPUS-COVERAGE (`.56.b`):** its three parts that route no leaf
(`refresh-ledger`, `recent-chronology`, `legacy-chronology`, all unchanged since its 2026-08-10 migration) moved the same
way under `corpus_task_evidence_archived_parts`. Its live lines went from 85.9% to 69.6% of target, and no other
contract's live total warns. ADR 0045's rule that no *tree* is archived for capacity is unchanged: the tree stays live,
and only its closed parts move.
