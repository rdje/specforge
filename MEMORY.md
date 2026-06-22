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
- latest_commit (baseline): **`5e1a1dab`** `FSMGEN-REFRESH-INTEGRATE-5.1 — bump subs/fsmgen 5ce0335c5->d327129b7 (shipped storage fields) + un-gate DOC-INTENT-TAXONOMY.4a.ii`. **32 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.4a.ii`) → 33 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-22`). `DOC-INTENT-TAXONOMY` `.4` per-category levers; Gap A (register bit-fields) now CLOSED.
- **✅ `DOC-INTENT-TAXONOMY.4a.ii` DONE (this slice), committing now** — emitted the register bit-field map into FSMGen's shipped ISF field-structured storage. New `IsfStorageField` + `IsfStorageVar.fields` + the nested `(fields (field …))` render, fed by the pure structural fail-closed `register_storage_fields` + `normalize_field_access` (`crates/specforge/src/ir/isf_ir.rs`); `isf_register_fields_not_lowered` residual (`ir/adapters.rs`, folds in `.4a.i`); test-only `run_fsmgen_schedule_json` helper (`ir/mod.rs`). **Measured live (real emitter): 6,570 register bit-fields / 2,531 registers / 24 docs now reach `.isf`** (was 0). Gate: located fields only / drop sanitized-name-collision groups / overlap fails register closed / access→FSMGen 10-token set / field reset=parent slice / width-fitting enums (ADR 0006, no name list).
- **GATES (this slice):** `cargo fmt`/`clippy -D warnings` clean; full `cargo test 1702/0` (+6 tests incl. `register_fields_pass_fsmgen_strict_and_round_trip`); `kg-bench 156/156`; 4 WIRE-BASED-100 golds emit 0 fields → emitted `.isf` BYTE-IDENTICAL (old-vs-new `adapt` diff empty); RISC-V IOMMU (122) / GIC (424) / CoreSight SoC-600 (974) `fsmgen --strict` `success/0 diags` 0-NEW; `inferred_storage[].fields[]` round-trip asserted; `run_ci.sh` green; `check_doctrines.sh` 3/3; KM in sync (115 facts / 830 keys).
- next_action (PNT): pick the next `DOC-INTENT-TAXONOMY.4` lever — **`.4e` conditional-rule lowering triage** (read-only, RAM-light) or **`.4d` cat-4 ISA/CSR lowering decision packet** (read-only) are the buildable measurement-first slices; **`.4c` cat-3 topology** likely needs a verified FSMGen FR (measure the submodule first — `[[feedback_verify_fsmgen_before_fr]]`); **`.4b` Gap B (message-field structures)** STAYS FSMGen-gated (packet/flit deferred) AND needs an `Evidence→IntentIR` carrier. Other active trees: `CORPUS-COVERAGE` re-ingest 57 normalized-missing docs (RAM/Docling-gated, host-local source); `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` frontiers parked/deferred.
- in_flight_uncommitted: the **`DOC-INTENT-TAXONOMY.4a.ii`** staged set (crates/specforge/src/ir/{isf_ir.rs, adapters.rs, mod.rs}, README.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, docs/knowledge/register-bit-field-isf-lowering-gap.md, KNOWLEDGE_MAP.md, docs/book/src/pipeline/isf-adapter.md, docs/book/src/document-categories.md, CHANGES.md, DEVELOPMENT_NOTES.md, RUST_CODEBASE_ANALYSIS.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM healthy (~80% free). Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). The next `.4c`/`.4d`/`.4e` levers are read-only/measurement-first (RAM-light); a fresh focused session is recommended before the next CODE slice for signoff sharpness.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
