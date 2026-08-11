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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.2`; `.0` and `.1` are complete. Tracking-only:
  `STATUS-LEDGER-ROLLOVER.2` and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: `doctrine/spec_to_intent_category_contract.json` now defines six category-aware,
  source-to-IntentIR contracts with exact reviewed precision/recall, provenance, conservation, residual, and
  anti-fabrication floors. Residuals preserve loss but do not count as complete typed capture; non-applicability
  needs independent source evidence. Message-field transport and platform-topology/clock-domain carrier gaps
  are explicitly incomplete. No category is retroactively claimed supported; `.4` will decide support from
  held-out documents. ISF remains a separate, later boundary (ADR 0033).
- Next action: execute `.2` by auditing every production extractor against the canonical workflow, then make
  `converge` invoke, deliberately schedule, or explicitly report omission of contract extraction, signal
  resolution, register recovery, NLI enforcement, and every other production capability. Pin the accounting
  with tests so standalone capability cannot be reported as default end-to-end delivery.
- In-flight uncommitted: none after the `.1` commit. No background job runs.
- Blockers: none for `.2`. Optional Ollama and LM Studio endpoints were unavailable during startup `doctor
  --strict`; provider-free orchestration work is not blocked. Rolling-ledger pressure is derived by
  `perl scripts/check_rolling_ledger_protocol.pl --report`; the current report passes. The user-owned
  `.claude/settings.json` is untouched.
