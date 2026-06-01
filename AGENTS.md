# Agent bootstrap — read this first, whatever AI or harness you are

This file is the tool-neutral entrypoint (Codex, and the common `AGENTS.md` convention).
Other harnesses' bootstrap files (`CLAUDE.md`, `.cursorrules`,
`.github/copilot-instructions.md`, …) point back here. The system of record is
**`README.md`** + **`MEMORY_ARCHITECTURE.md`**.

## On every session start / resume

1. Read **`README.md`** — project objective, layout, standard commands.
2. Read **`MEMORY_ARCHITECTURE.md`** — how memory + continuity work in this repo
   (MANDATORY; it is enforced — see below).
3. Resume from **`MEMORY.md`** — the bounded resume pointer: latest commit, the active
   task-tree frontier, the single next action, any in-flight uncommitted work.
4. Open the active **task-tree** under `docs/tasks/` (index: `docs/TASK_TREE.md`); its
   frontier row is your precise next step.
5. Pull only the relevant **decision records** under `docs/decisions/`.

## Non-negotiable working rules

- **No code change without an owning task-tree leaf first** (`docs/TASK_TREE_README.md`;
  doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md`).
- **Route every durable thing to a layer and commit before the turn ends** — resume
  pointer (`MEMORY.md`, overwrite-only, capped) / task-trees (`docs/tasks/`) / decision
  records (`docs/decisions/`) / git history. Nothing important may live only in this
  conversation.
- **Commit per `COMMIT.md`** after every slice, with the **work-unit id in the subject**.
- **Before committing, run `scripts/check_memory_architecture.sh`** — git hooks and CI
  run it too, and a non-compliant change fails the build and cannot merge.

## Enforcement (why this is hard to ignore)

`MEMORY_ARCHITECTURE.md` §9 wires four gates: these bootstrap pointers (discovery),
`scripts/check_memory_architecture.sh` (the invariants), `.githooks/` (local gate —
activate once with `git config core.hooksPath .githooks`), and the CI step (the
un-bypassable backstop).
