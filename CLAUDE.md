# Claude Code bootstrap

Follow **`AGENTS.md`** (the tool-neutral agent bootstrap). Start by reading
**`README.md`** and **`MEMORY_ARCHITECTURE.md`**, then resume from **`MEMORY.md`**.

Non-negotiable: no code change without an owning task-tree leaf first (`docs/tasks/`,
doctrine in `docs/decisions/0003-task-tree-and-commit-doctrine.md`); route every durable
thing to a memory layer and commit per `COMMIT.md`; run
`scripts/check_memory_architecture.sh` before committing (hooks + CI enforce it).
