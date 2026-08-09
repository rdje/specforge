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
date: 2026-08-08
status: current
tags: [documentation, containment, git, submodule]
evidence: docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md
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
only 207 bytes below warning. Its next append requires separately task-owned active-tree containment; it may not
borrow the terminal topology or a wider ceiling.

`scripts/check_task_tree_archive.pl` now enforces `migrated`: the capsule retains the exact locked identity, and
the checker validates the closed root, exact index/manifest routes, provenance, milestones, and ceilings. Its
15-case self-test runs unconditionally through `LIVE-DOC-SIZE`.
