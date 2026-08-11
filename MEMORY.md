# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.3`; `.0` through `.2` are complete. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: the six-category source-to-IntentIR contract remains the acceptance authority. `converge` now
  emits 17 machine-readable capability rows covering all 16 production subcommands; each separates stable
  participation (`integrated`, `scheduled`, `omitted`) from this run's state and reason. A Clap-derived test
  partitions all 28 commands and prevents unreported production capability. Contract extraction, Tier-3
  relation resolution, register-bit recovery, and IntentIR NLI demotion remain explicit omissions; the ledger
  changes stdout/report state only, not canonical IR or adapter bytes.
- Next action: execute `.3` by tracing the real PDF visual/layout producer path into typed multimodal IR,
  selecting a retained source fixture with an applicable timing/state/register figure, and proving real input
  reaches the required consumer or is reported unavailable. Do not count synthetic waveform tests as delivery.
- In-flight uncommitted: none after the `.2` commit. No background job runs.
- Blockers: none for the provider-free `.3` audit. Startup `doctor --strict` found no ready Ollama or LM Studio
  endpoint, so a live VLM proof may require the endpoint to become ready; do not substitute a fabricated
  observation. Rolling-ledger and corpus-frontier reports pass. The user-owned `.claude/settings.json` is
  untouched.
