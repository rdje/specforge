# Claude Code bootstrap

Follow **`AGENTS.md`** (the tool-neutral agent bootstrap). Start by reading
**`README.md`** and **`MEMORY_ARCHITECTURE.md`**, then resume from **`MEMORY.md`**.

Before re-deriving any fact from code or runtime, open **`KNOWLEDGE_MAP.md`** and use its command to
grep all question shards (question → one pointer → dated fact or `reverify`); write a fact
card under `docs/knowledge/` when you establish a durable fact or catch archaeology
(`knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`).

Non-negotiable: no code change without an owning task-tree leaf first (`docs/tasks/`,
doctrine in `docs/decisions/0003-task-tree-and-commit-doctrine.md`); route every durable
thing to a memory layer and commit per `COMMIT.md`. Doctrines are **mechanically enforced**
(`DOCTRINE_ENFORCEMENT.md`, the 4th portable architecture): run `scripts/check_doctrines.sh`
before committing — the registry/driver that runs every doctrine check (memory-arch,
knowledge-map, task-acceptance, README-policy, live-document-size); hooks + CI run it too. A Rust code change must carry an
evidence-backed acceptance checklist in its task leaf (template in `TOOLBOX.md`).
