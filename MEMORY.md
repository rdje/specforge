# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver: memory-arch +
  knowledge-map + task-acceptance); hooks + CI run it too. Retrieval: `KNOWLEDGE_MAP.md`.

## Current state (OVERWRITE this block each update — do not append)
- latest_commit (baseline): **`179e7ae7`** `DOC-INTENT-TAXONOMY.3a — recognizer design`. **24 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOCTRINE-ENFORCEMENT-ADOPT.0`) → 25 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`). **ACTIVE owner directive interrupts the prior frontier** (see next line).
- **OWNER DIRECTIVE (`2026-06-22`): "adopt this doctrine enforcement system" → `DOCTRINE_ENFORCEMENT.md`** (the portable 4th architecture, sibling of `MEMORY_ARCHITECTURE.md`). OWNED as tree **`DOCTRINE-ENFORCEMENT-ADOPT`** ([[feedback_task_tree_doctrine]]). Owner clarification: **`TOOLBOX.md` must catalog SpecForge's OWN debug/diagnostic tools** (inspect/validate/doctor/converge/adapt --strict/kg-bench/eval-extraction/audit-extraction/--dry-run/FSMGen --strict --check) — for `.1`.
- **✅ `DOCTRINE-ENFORCEMENT-ADOPT.0` DONE (this slice), committing now** (framework, no Rust): `DOCTRINE_ENFORCEMENT.md` (standard; §10 = SpecForge registry) + `scripts/check_doctrines.sh` (registry+driver w/ meta-check) registering EXISTING `MEMORY-ARCH` + `KNOWLEDGE-MAP`; `.githooks/pre-commit` (E3) + `scripts/run_ci.sh` (E4) route through the driver (KM regen+stage preserved); decision `0006`; discovery pointers (README/AGENTS/CLAUDE). Driver verified standalone **2/2 PASS** before wiring.
- next_action (PNT): **`DOCTRINE-ENFORCEMENT-ADOPT.1`** — write SpecForge-native `scripts/check_task_acceptance.sh` (a staged Rust code change `crates/**/*.rs`+`crates/**/test_data/**` must have a staged owning `docs/tasks/*.md` leaf with ROOT-CAUSE/ADDRESSED/NO-REGRESSION ticked + evidence-backed via SpecForge signatures `kg-bench 156/156`/`WIRE-BASED-100 1.000`/`run_ci`) + **SpecForge `TOOLBOX.md`** (own debug tools + checklist template); register `TASK-ACCEPTANCE`; add `TOOLBOX.md` to discovery. Then `.2` mdBook chapter (book-method-doc close) + KM card. **THEN resume `DOC-INTENT-TAXONOMY.3b`** (implement `classify_document_intent_category` per the committed `.3a` design — that work will be the FIRST gated by the new task-acceptance check, so its leaf needs the acceptance checklist).
- in_flight_uncommitted: **`DOCTRINE-ENFORCEMENT-ADOPT.0`** staged set (DOCTRINE_ENFORCEMENT.md, scripts/check_doctrines.sh, .githooks/pre-commit, scripts/run_ci.sh, docs/decisions/0006 + INDEX, tree + TASK_TREE.md, README/AGENTS/CLAUDE, CHANGES/DEVELOPMENT_NOTES/LIVE_ACHIEVEMENT_STATUS/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy (`memory_pressure`). Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). ollama up, no model loaded (serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
