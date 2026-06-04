# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 30 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: `1b49b59c` — "SPEC-MINING-PROVENANCE.2 …" (this `SPEC-MINING-PROVENANCE.3` close commit pending → becomes 25 ahead of pushed `d92a73e3`; **PUSH approaching ~30**). NOTE: CI green baseline = 1229 tests. KM live (`KNOWLEDGE_MAP.md`, 7 facts/36 keys).
- active_work_unit: none — `SPEC-MINING-PROVENANCE` **CLOSED `2026-06-04`** (forward-spec-mining framing + a complete per-author adopt/defer ledger `docs/research/grounding/adopt-defer-ledger.md` [Take/Leave-out+why/Instantiated-at for the temporal trio + ~10 other clusters] + synthesis + KM cards `spec-mining-framing`/`adopt-defer-ledger`; advisory; CI green 1229). NINE trees CLOSED this run.
- next_action: PNT — first capture the user's LTL/CTL/TLA+ question as a durable decision (it is design-rationale, archaeology-prone): write **ADR `docs/decisions/0005-temporal-logic-ltl-mtl-not-ctl-tla.md`** + a KM card (`temporal-logic-choice`): LTL/MTL = YES (used now — `temporal_rules` ARE `G(ante→cons)`, MTL `F[min,max]` for cycle windows, rendered by `ir/temporal_ltl.rs`; grounded Pnueli/GoldMine/Texada, all linear-time); CTL = NO (branching-time doesn't match a spec's single intended linear behavior); TLA+ = NO (an authoring + model-checking environment, orthogonal to mining; could be a future EXPORT target like PSL/SVA, not planned); SpecForge mines properties, does NOT model-check (checking is downstream/out-of-scope). Update `docs/decisions/INDEX.md`. Then continue PNT: backlog `.isf`→PSL/SVA export (FSMGen contract first); conformal calibration; NLI verifier; Dempster combiner; or the deferred-adopt items in the ledger. **PUSH approaching (~25 ahead; push at ~30 or on ask).**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
