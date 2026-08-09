---
id: live-document-coverage-authority
title: Parent Git index defines SpecForge live-Markdown coverage
answers:
  - "which Markdown files must the live-document containment registry cover"
  - "does SpecForge classify Markdown inside the FSMGen submodule"
  - "what is the live-document coverage authority"
  - "which task tree crossed its live-document byte warning"
  - "what owns containment of the live document adoption task history"
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

The `.9a` resulting-tree report measured that same program task file at 2,420 lines / 229,064 bytes. It is the
largest `task_evidence` member, 6,241 bytes beyond the 222,823-byte warning with 21,611 bytes of pre-rollover
headroom before the 250,676-byte threshold. `.10a`/`.10b` own a lossless bounded current/history topology before rollover;
the existing ceiling may not be widened and historical evidence may not be trimmed.
