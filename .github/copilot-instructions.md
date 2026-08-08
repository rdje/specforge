# GitHub Copilot instructions

Follow **`AGENTS.md`** (the tool-neutral agent bootstrap). Start by reading
**`README.md`** and **`MEMORY_ARCHITECTURE.md`**, then resume from **`MEMORY.md`**.

Before re-deriving any fact from code or runtime, check **`KNOWLEDGE_MAP.md`** (grep your
question, follow the one pointer, trust the dated fact or run its `reverify`); write a fact
card under `docs/knowledge/` when you establish a durable fact or catch archaeology
(`knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`).

Non-negotiable: no code change without an owning task-tree leaf first (`docs/tasks/`;
doctrine in `docs/decisions/0003-task-tree-and-commit-doctrine.md`); route every durable
thing to a memory layer and commit per `COMMIT.md` with the work-unit id in the subject;
run `scripts/check_doctrines.sh` before committing (memory, knowledge-map, task-acceptance,
README-policy, and live-document-size checks; git hooks + CI enforce it).
