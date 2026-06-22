# Agent bootstrap — read this first, whatever AI or harness you are

This file is the tool-neutral entrypoint (Codex, and the common `AGENTS.md` convention).
Other harnesses' bootstrap files (`CLAUDE.md`, `.cursorrules`,
`.github/copilot-instructions.md`, …) point back here. The system of record is
**`README.md`** + **`MEMORY_ARCHITECTURE.md`** (durability) + **`KNOWLEDGE_MAP_ARCHITECTURE.md`**
(retrieval — the `knowledge-map/` bundle).

## On every session start / resume

1. Read **`README.md`** — project objective, layout, standard commands.
2. Read **`MEMORY_ARCHITECTURE.md`** — how memory + continuity work in this repo
   (MANDATORY; it is enforced — see below).
   - and **`DOCTRINE_ENFORCEMENT.md`** — the 4th portable architecture: every doctrine is a
     mechanically-gated check run from one registry/driver (`scripts/check_doctrines.sh`). The
     diagnostic toolbox + the acceptance-checklist template a code change must satisfy live in
     **`TOOLBOX.md`**.
3. Resume from **`MEMORY.md`** — the bounded resume pointer: latest commit, the active
   task-tree frontier, the single next action, any in-flight uncommitted work.
4. Open the active **task-tree** under `docs/tasks/` (index: `docs/TASK_TREE.md`); its
   frontier row is your precise next step.
5. Pull only the relevant **decision records** under `docs/decisions/`.
6. **Before re-deriving any fact from code or runtime, check `KNOWLEDGE_MAP.md`** — grep your
   question, follow the one pointer to the canonical home, and trust the dated fact or run its
   `reverify` command. Re-deriving a fact that was already logged once is *archaeology*
   (`KNOWLEDGE_MAP_ARCHITECTURE.md`).

## Non-negotiable working rules

- **No code change without an owning task-tree leaf first** (`docs/TASK_TREE_README.md`;
  doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md`).
- **Route every durable thing to a layer and commit before the turn ends** — resume
  pointer (`MEMORY.md`, overwrite-only, capped) / task-trees (`docs/tasks/`) / decision
  records (`docs/decisions/`) / git history. Nothing important may live only in this
  conversation.
- **Commit per `COMMIT.md`** after every slice, with the **work-unit id in the subject**.
- **Before committing, run `scripts/check_doctrines.sh`** (the general doctrine enforcer — it runs
  `check_memory_architecture.sh`, the knowledge-map check, the task-acceptance check, …) — git hooks
  and CI run it too, and a non-compliant change fails the build and cannot merge
  (`DOCTRINE_ENFORCEMENT.md`).
- **Write a Knowledge Map fact card** (`docs/knowledge/<id>.md`, front-matter with an
  `answers:` list of the questions an agent would grep) whenever you establish a durable
  structural/causal fact, or catch yourself re-deriving one — so the next session finds it
  instead of excavating it. The map (`KNOWLEDGE_MAP.md`) is **derived + gated**; never
  hand-edit it. See `KNOWLEDGE_MAP_ARCHITECTURE.md`.

## Enforcement (why this is hard to ignore)

`MEMORY_ARCHITECTURE.md` §9 wires four gates: these bootstrap pointers (discovery),
`scripts/check_memory_architecture.sh` (the invariants), `.githooks/` (local gate —
activate once with `git config core.hooksPath .githooks`), and the CI step (the
un-bypassable backstop). The composed **Knowledge Map** layer adds a parallel gate in the
same pre-commit hook and `run_ci.sh` step: `knowledge-map/scripts/check_knowledge_map.sh`
regenerates + validates the derived map (derive-and-diff), so it cannot drift.
