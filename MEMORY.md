# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Derive the current repository revision on read with `git rev-parse HEAD`; never store a
  latest-commit shadow that the recording commit would immediately invalidate.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver for memory,
  knowledge-map, task-acceptance, README policy, and live-document containment); hooks + CI
  run it too. Retrieval starts at bounded `KNOWLEDGE_MAP.md`, then searches its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `CORPUS-COVERAGE.2.33d.iv`; `.d.iii` closes the conditional downstream backstop as measured
  unnecessary without code.
- Current state: `.d.ii`'s three signal-authority gates and combined fixed-point regression remain byte-identical
  and pass five focused oracles. Relations/directions cannot originate a rejected name, so no duplicate
  convergence/adapter filter is justified. Root-ledger pressure remains closed and warning-safe.
- Next action: rebuild the real USB EvidenceIR→SemanticIR→IntentIR→ISF cascade with the repaired release binary;
  prove the four false ports, phantom initiator, and 29-rule model disappear, then run WIRE/KG/FSMGen/full gates.
- In-flight uncommitted: none after the `.d.iii` commit; product behavior, generated artifacts, shared inputs,
  thresholds, ceilings, capsules, and segment bytes are unchanged.
- Blockers: none. The user-owned `.claude/settings.json` remains untouched.
