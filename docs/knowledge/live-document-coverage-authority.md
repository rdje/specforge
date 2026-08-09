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

The full lifecycle and checker contract lives in
`docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` under `.3a`.

The `.10a` audit measures that program task file at 2,451 lines / 232,649 bytes with 18,026 bytes of
pre-rollover headroom. ADR 0018 selects a two-commit terminal boundary: `.10b.i` commits the complete live
source plus its verifier, and `.10b.ii` copies that durable source byte-for-byte to an immutable capsule before
leaving a bounded closed summary at the stable task path. Exact history remains directly retrievable through a
bounded index/manifest; the existing ceiling may not be widened and evidence may not be trimmed.

This rule applies only to a completed tree. Active `PDF-VARIANT-DIGESTION` is 2,393 lines / 222,616 bytes,
only 207 bytes below warning. Its next append requires separately task-owned active-tree containment; it may not
borrow the terminal topology or a wider ceiling.
