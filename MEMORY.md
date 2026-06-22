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
- latest_commit (baseline): **`810b510b`** `DOCTRINE-ENFORCEMENT-ADOPT.0 — adopt the portable Doctrine-Enforcement architecture`. **25 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOCTRINE-ENFORCEMENT-ADOPT.1`) → 26 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`). **ACTIVE owner directive interrupts the prior `DOC-INTENT-TAXONOMY.3b` frontier** (resumed after `.2`).
- **OWNER DIRECTIVE (`2026-06-22`): "adopt this doctrine enforcement system" → `DOCTRINE_ENFORCEMENT.md`** (the portable 4th architecture, sibling of `MEMORY_ARCHITECTURE.md`). OWNED as tree **`DOCTRINE-ENFORCEMENT-ADOPT`** ([[feedback_task_tree_doctrine]]). Owner clarification: **`TOOLBOX.md` catalogs SpecForge's OWN debug tools** (doctor/inspect/validate/adapt --target isf/kg-bench/WIRE-BASED-100 golds/--dry-run byte-identical/FSMGen --strict --check/nli-verify/grits-consensus/measure_isf_completeness.py) — DONE in `.1`.
- **✅ `DOCTRINE-ENFORCEMENT-ADOPT.0` DONE + COMMITTED `810b510b`** (framework, no Rust): `DOCTRINE_ENFORCEMENT.md` (standard) + `scripts/check_doctrines.sh` (registry+driver w/ meta-check) registering EXISTING `MEMORY-ARCH` + `KNOWLEDGE-MAP`; `.githooks/pre-commit` (E3) + `scripts/run_ci.sh` (E4) route through driver (KM regen+stage preserved); decision `0006`; discovery pointers. The commit was itself gated by the new driver (E3 proven live).
- **✅ `DOCTRINE-ENFORCEMENT-ADOPT.1` DONE (this slice), committing now** (scripts+docs, no Rust): `scripts/check_task_acceptance.sh` (bash-3.2-safe; staged Rust `crates/**/*.rs`+`crates/**/test_data/**` ⇒ needs a staged owning `docs/tasks/*.md` leaf with ROOT-CAUSE/ADDRESSED/NO-REGRESSION ticked+evidence-backed; docs/scripts EXEMPT) + SpecForge `TOOLBOX.md`; registered `TASK-ACCEPTANCE` → driver **3/3 PASS**; all 5 gate paths tested+reverted.
- next_action (PNT): **`DOCTRINE-ENFORCEMENT-ADOPT.2`** — user-facing mdBook chapter on the doctrine-enforcement system (what it is, registry/driver, E1→E4 gates, the acceptance checklist, how to run `check_doctrines.sh`) wired into `docs/book/src/SUMMARY.md`; KM fact card; book-method-doc CLOSE leaf. **THEN resume `DOC-INTENT-TAXONOMY.3b`** (implement `classify_document_intent_category` per the committed `.3a` design — the FIRST Rust slice gated by the new task-acceptance check, so its leaf MUST carry the acceptance checklist).
- in_flight_uncommitted: **`DOCTRINE-ENFORCEMENT-ADOPT.1`** staged set (scripts/check_task_acceptance.sh, scripts/check_doctrines.sh, TOOLBOX.md, tree, CHANGES/DEVELOPMENT_NOTES/LIVE_ACHIEVEMENT_STATUS/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy (`memory_pressure`). Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). ollama up, no model loaded (serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
