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
- latest_commit (baseline): **`d6239217`** `DOC-INTENT-TAXONOMY.3b — implement the 6-category document PURPOSE recognizer (validate-reported)`. **28 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.3c`) → 29 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`). Doctrine-enforcement tree CLOSED; executing the `DOC-INTENT-TAXONOMY` frontier — `.3` recognizer now COMPLETE.
- **✅ `DOC-INTENT-TAXONOMY.3c` DONE (this slice), committing now** (test + docs — closes the `.3` recognizer sub-tree): added end-to-end integration test `validate_evidence_ir_reports_document_intent_category` in `crates/specforge/src/commands/validate.rs` (metric + confidence + `evidence_document_intent_category` finding reach the report; near-empty → honest `unresolved` residual); per-category golds stay locked by the 13 `.3b` unit tests. User-facing mdBook chapter (`docs/book/src/quality/validation.md` purpose-category section beside `document_class`; `document-categories.md` flipped "target"→"now CLI-reported"). KM card `document-intent-category-recognizer` (map 113→114 facts / 815 keys, in sync). ISA/PHY vocab precision-verified (ISA 0 corpus docs — honest fall-through; PHY recovers all 4 OpenCAPI PHY docs).
- **GATES (this slice):** `kg-bench 156/156`; full `cargo test` green (warning-deny, +1 integration test); `cargo fmt`/`clippy -D warnings` clean; `mdbook build` green; knowledge-map derive-and-diff in sync; WIRE-BASED-100 orthogonal (test + docs only). Run `scripts/run_ci.sh` green before commit.
- next_action (PNT): **`DOC-INTENT-TAXONOMY.4+`** — per-category completeness levers. `.2` makes **Gap A = register bit-field ISF lowering** the highest-leverage first lever (12,638 fields across 32 docs lower to ZERO today; registers emit opaque width-only storage), then **Gap B = message-field structure carry → IntentIR → lowering** (1,220 fields, no Intent carrier). BOTH converge on the SAME FSMGen ISF-abstraction need (field-structured storage + packet/structure layouts) → before any emitter hack, EMPIRICALLY verify the current `subs/fsmgen` pin + file a verified FR (`[[feedback_verify_fsmgen_before_fr]]`, `[[feedback_isf_no_hacks]]`, `docs/FSMGEN_FEEDBACK.md`). Owner context: FSMGen adding SV/UVM+VHDL verification path → both paths want richer ISF (memory banks, single/dual-port memory). Decompose `.4` into a measurement/design leaf first (no code before task-tree ownership). Other active frontiers if `.4+` blocks: `KG-ISF-COMPLETENESS.1b`, `KG-ISF-TRANSACTIONS`, `CORPUS-COVERAGE`.
- in_flight_uncommitted: the **`DOC-INTENT-TAXONOMY.3c`** staged set (validate.rs, docs/book/src/quality/validation.md, docs/book/src/document-categories.md, docs/knowledge/document-intent-category-recognizer.md, KNOWLEDGE_MAP.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy (`memory_pressure` ~81% free). Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). ollama up, no model loaded (serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
