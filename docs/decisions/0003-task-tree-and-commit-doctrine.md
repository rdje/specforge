# 0003 — Task-tree ownership before any code change; strict commit workflow

- Date: 2026-06-01
- Status: accepted
- Tags: process, doctrine, continuity

## Context

The project's non-negotiable working doctrine (re-stated by the user multiple times)
governs every coding and non-coding activity. It must be discoverable by any AI in any
harness, so it is recorded here as a layer-C fact that points at the authoritative,
tracked docs — not duplicated.

## Decision

Follow these, without exception:

1. **Task-tree ownership FIRST.** No code change — however small — occurs without an
   owning task-tree leaf first. The whole roadmap and all activities (coding +
   non-coding) are tracked under `docs/tasks/*`; the index is `docs/TASK_TREE.md`; the
   system is defined in `docs/TASK_TREE_README.md`. Past (pre-system) work is audited
   into trees. (Task-trees are **layer B** of `MEMORY_ARCHITECTURE.md`.)
2. **Signoff-level code quality.** Quality over speed; no rushing; sloppy code is
   rejected. "We have all the time we need."
3. **Strict commit workflow** per `COMMIT.md` after every completed task/slice/lane:
   sync the relevant live docs/live-book; use `git_message_brief.txt` (untracked,
   cleared to 0 bytes after); put the work-unit id in the subject; the completion
   message reports commit id + message + files + live-status snapshot.
4. **Zero drift** between ROADMAP ↔ codebase ↔ mdBook; the mdBook always reflects what
   the code does. `BOOK-METHOD-DOC`: a tree's closing leaf refreshes its mdBook section.
5. **Push** every ~30 commits or on explicit ask (not at milestones).
6. **Artifact cleanup** ≥ every 24h: reclaim unused/generated artifacts (logs, `.bin`,
   normalized bundles) when 100% safe; disk is a critical resource.

## Consequences

- A new AI/harness reads this record (and `MEMORY_ARCHITECTURE.md` → `AGENTS.md`) and
  is bound by the same doctrine — continuity survives a model/harness switch.
- Enforcement of (1) and (3) is mechanical via `MEMORY_ARCHITECTURE.md` §9 (self-check
  script + git hooks + CI gate).

## Links

- Authoritative docs: `docs/TASK_TREE_README.md`, `docs/TASK_TREE.md`, `COMMIT.md`,
  `MEMORY_ARCHITECTURE.md`, `README.md`.
