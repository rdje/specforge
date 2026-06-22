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
- latest_commit (baseline): **`8ebc664c`** `DOCTRINE-ENFORCEMENT-ADOPT.2 — user-facing mdBook chapter + KM card; tree CLOSED`. **27 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.3b`) → 28 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`). Doctrine-enforcement tree CLOSED; now executing the `DOC-INTENT-TAXONOMY` frontier.
- **✅ `DOC-INTENT-TAXONOMY.3b` DONE (this slice), committing now** (code — the FIRST Rust slice through the new `TASK-ACCEPTANCE` gate): implemented the 6-category PURPOSE recognizer `classify_document_intent_category` (+ `DocumentIntentCategory`/`IntentCategoryConfidence`/`DocumentIntentClassification`) in `crates/specforge/src/ir/completeness.rs` (sibling to `classify_document`, 13 in-file tests), wired into `validate.rs` at the shared census site (bound once → both classifiers): printed block + `evidence_document_intent_category` Info finding + `document_intent_category`/`_confidence` metrics. Added generic front-matter ISA/PHY helpers (ADR 0006). **Refined the `.3a` flit clause** its own per-doc data falsified (flit ⇒ cat-1 only without a register map; wire-vs-structure weight dominance; register count never vetoes clean wire — AXI wire 401 ≥ struct 229). Only clean wire + self-declared guide HIGH; else LOW + residual. **Live over 78 docs: 21 wire / 8 guide (high) / 28 register-or-platform / 16 unresolved / 5 PHY (low), 0 high-confidence false positives.** Leaf carries the evidence-backed Acceptance Checklist.
- **GATES (this slice):** `kg-bench 156/156`; completeness lib `66/66` (13 new); full `cargo test 1695 passed; 0 failed` (warning-deny); `cargo fmt`/`clippy -D warnings` clean; WIRE-BASED-100 provably orthogonal (pure new fn + additive `validate` reporting; no extraction/emitter path touched). Run `scripts/run_ci.sh` green before commit.
- next_action (PNT): **`DOC-INTENT-TAXONOMY.3c`** — gold/negative + honest-residual fixtures locking each category (register-heavy-protocol rescue, guide front-matter override, cat 2↔3 / 4 / 5↔6 residuals); user-facing mdBook chapter (`docs/book/src/quality/validation.md` beside `document_class`; flip `document-categories.md` from "target" to "now CLI-reported"); KM fact card for the recognizer; empirically calibrate/widen the ISA/PHY front-matter vocab against real front-matter strings; `scripts/run_ci.sh` green. Then `.4+` per-category levers (Gap-A register bit-field lowering) + FSMGen ISF-abstraction FRs.
- in_flight_uncommitted: the **`DOC-INTENT-TAXONOMY.3b`** staged set (completeness.rs, validate.rs, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, README.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, RUST_CODEBASE_ANALYSIS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy (`memory_pressure` ~81% free). Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). ollama up, no model loaded (serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
