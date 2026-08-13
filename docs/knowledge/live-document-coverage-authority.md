---
id: live-document-coverage-authority
title: Parent Git index defines SpecForge live-Markdown coverage
answers:
  - "which Markdown files must the live-document containment registry cover"
  - "does SpecForge classify Markdown inside the FSMGen submodule"
  - "what is the live-document coverage authority"
  - "which task tree crossed its live-document byte warning"
  - "what owns containment of the live document adoption task history"
  - "how is a completed oversized task tree contained without losing evidence"
  - "where is the complete live document containment adoption task history"
  - "why must terminal task tree containment use two commits"
  - "can an active task tree use the terminal task archive topology"
  - "which active task tree is next at the live document warning"
  - "how is the terminal task source archive boundary verified"
  - "what is the exact active PDF task evidence baseline"
  - "is the PDF-VARIANT-DIGESTION current frontier internally consistent"
  - "who reads or writes the active PDF task tree"
  - "what did the active PDF task containment census find"
  - "what architecture contains an oversized active task tree"
  - "what is ADR 0019"
  - "how is an active task source locked before migration"
  - "what does the active task evidence checker verify"
  - "have the PDF task migration destinations been created"
  - "which active task tree now requires bounded evidence containment"
  - "what is the exact SPEC-TO-INTENT-ALIGNMENT task evidence baseline"
  - "what did the SPEC-TO-INTENT-ALIGNMENT task containment census find"
  - "why must the bounded SPEC-TO-INTENT-ALIGNMENT root retain every task id"
  - "who reads and writes the SPEC-TO-INTENT-ALIGNMENT task tree"
  - "can the active-task evidence checker support the alignment task through data"
date: 2026-08-14
status: current
tags: [documentation, containment, git, submodule]
evidence: docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md; docs/tasks/SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.md; docs/research/spec-to-intent-task-evidence-containment-census.md
reverify: git ls-files '*.md' | wc -l
---

The complete SpecForge live-document set is the Markdown path set returned by the parent
repository's Git index. Every such path must match exactly one registry surface. The pinned
`subs/fsmgen` gitlink contributes no parent-tracked Markdown and remains under FSMGen's independent
authority. Untracked generated mdBook output is project artifact data, not a tracked live document.

The bounded closed program summary lives at `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`; its archive
index routes the exact source containing the full `.3a` lifecycle/checker contract and all later evidence.

ADR 0018's migration is complete. `.10b.i` committed the 2,538-line / 242,172-byte source; `.10b.ii` copied it
byte-for-byte to `docs/archive/tasks/live-document-size-containment-adoption/source-through-2026-08-09.md` at
SHA-256 `f8e10e…96b68`, then left a 119-line / 7,978-byte closed summary at the stable task path. Exact history is
directly retrievable through the bounded archive index/manifest; no ceiling widened and no evidence was trimmed.

This rule applies only to a completed tree. Active `PDF-VARIANT-DIGESTION` is 2,393 lines / 222,616 bytes,
191 maximum content-line bytes, SHA-256 `9284dce40ad896c3de3811e95c3fdd347132b1849083499e9c543fc9026a19d4`,
and Git blob `7d89ea4fe59e53e55614b5730bded40d99b4b555`. Its 222,616 bytes sit only 207 below warning. The
`ACTIVE-TASK-EVIDENCE-CONTAINMENT` tree now owns its active boundary; `.0` pins the unchanged source and `.1.1`
owns the current/history and reader/writer census. The target may not accept another ordinary append, borrow the
terminal topology, or receive a wider ceiling before that migration.

The legacy current-state text is internally inconsistent: `## Current frontier` names `.9.3` active while the
same section records `.9.3a` and `.9.3b` done, several node headers still say `in_progress` despite inline
completion evidence, and the later `.10i` completion is absent from that frontier paragraph. The containment
census must explicitly distinguish historical evidence from current authority; it may neither preserve stale
text as current truth nor rewrite literal history silently.

The `.1.1` read-only census accounts for every source byte in 13 regions. The nominal `Current frontier` is
1,392 lines / 124,201 bytes because it contains the `.9`–`.13` activity ledgers. Excluding the census itself, 32
files hold the exact stable path and 89 hold the task id; current route-sensitive inputs are ROADMAP, 18 citations
across two mdBook chapters, 24 fact cards, one prior report, and the containment owner. Generic readers consume
only the stable H1/status/catalog membership or staged task evidence. No executable opens or writes the target,
and no route uses a Markdown fragment. The only writer is the manual `COMMIT.md` slice transaction.

The census therefore favors a hybrid for ADR selection: a bounded active root and bounded semantic live parts for
future writes, plus an exact pre-migration capsule for literal legacy evidence. The active root cannot be a
terminal summary, the capsule cannot accept future writes, and the design must declare current-state precedence,
complete leaf routing, bounds, rotation, and atomic root/part updates.

ADR 0019 accepts that hybrid. The stable task path remains active current authority; seven semantic legacy parts
under `docs/tasks/pdf-variant-digestion/` provide bounded detail; and an exact capsule under
`docs/archive/tasks/pdf-variant-digestion/` proves provenance. Fifteen source regions cover every byte. Completion
commits plus inline evidence outrank stale open headers, unmet dependencies block, and ambiguity never enters the
frontier, so the normalized root initially reports no eligible leaf. Future work creates a new bounded activity
part and updates root + part atomically; semantic split happens before rollover and completed parts seal.

`.2.1` makes the pre-migration boundary executable through `scripts/check_active_task_evidence.pl`. The contract
binds boundary commit `6205f9d5`, Git blob `7d89ea4f…b555`, stage-zero index and working-file bytes, SHA-256,
metrics, 15 contiguous regions, seven planned payloads, and fixed root/index/part/manifest caps. Both declared
destination directories and all files remain absent. Twenty-nine focused cases exercise malformed source,
Git, schema, region, route, marker payload, manifest, frontier, pressure, seal, and residue states. `.2.2` alone
may complete the exact region digests and boundary-history routes before migration.

`.2.2` now pins final clean boundary `f04db37a`, all 15 region hashes/metrics, and 52 primary routes while every
destination remains absent. Ten routes use an exact tree-relative source literal because the historical body and
fully qualified commit subjects do not always use the same spelling; six canonical IDs never occur fully
qualified in the source at all. The checker permits only the full ID or its exact prefix-stripped form, token-
matches that value in the primary payload, and keeps index/manifest identity fully qualified. Thirty-two focused
cases exercise the complete source-locked contract before `.3.1` migration.

`.3.1` has now migrated that boundary. The stable path is a 106-line active current root over a 79-line index,
seven semantic parts, and 52 canonical routes; the exact 2,393-line source is an archive terminal at the locked
SHA-256. All 15 source slices remain byte-identical in their marked semantic payloads. Independent surface
records govern the index, part collection, and capsule, and the 34-case checker covers the root-last writer plus
non-ASCII raw-byte preservation and rollback. `.3.2` independently reproduces the committed result from a clean
clone and adds a positive post-migration root+part continuation fixture, bringing the suite to 35 cases and
closing the containment tree.

The next measured active target is `docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md`. At clean commit `112bc333` it is
2,049 lines / 278,178 bytes, SHA-256 `e70892a5c6acbe794bea6a9dd4484a90481e31d0aba64d8b8e8dcaaeae90a26c`,
and Git blob `66ae9b6cde51476d5b45a180c756f0f9463d3632`, with 40 path commits. That leaves only 350 bytes below the
unchanged 278,528-byte `task_evidence` target and ceiling, so the `.6d.ii.f` behavioral frontier cannot receive
an ordinary activation plus durable results safely. `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT` owns a separate
target-specific measurement, decision, enforcement, migration, and fresh-reader audit; the alignment source
remains byte-identical through its `.0` ownership boundary.

The `.1.1` census closes that source in 21 exact regions and finds 58 formal IDs, 45 frontier rows, 40
ID-bearing leaf commits, 31 exact-path consumers, and 92 identifier consumers. Unlike the PDF target, alignment
has a production reader: `trajectory.rs::validate_task_owner` opens the stable root and searches exact `- ID:`
owner declarations used by historical tests and current `.6e`/`.7`/`.8`/`.9` controller artifacts. A migrated
root must therefore retain all 58 compact ID declarations. Seven measured semantic groups fit the existing
contract-driven checker's portable 24-part / 32-region / 128-route, 98,304-byte-part, and 6,400-byte-line caps;
the 1,605-byte legacy maximum requires only a target-derived part limit, not a global-cap change. The source's
root-child lists and `.4`/`.5b` open-question/blocker prose are stale current-state surfaces, so `.1.2` must set
explicit precedence while the exact capsule retains every literal byte.

`scripts/check_task_tree_archive.pl` now enforces `migrated`: the capsule retains the exact locked identity, and
the checker validates the closed root, exact index/manifest routes, provenance, milestones, and ceilings. Its
15-case self-test runs unconditionally through `LIVE-DOC-SIZE`.
