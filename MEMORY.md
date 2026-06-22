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
- latest_commit (baseline): **`f008d1d2`** `DOC-INTENT-TAXONOMY.4a.ii — emit register bit-field map into ISF field-structured storage (6,570 fields / 24 docs)`. **33 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.4d`) → 34 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-23`). `DOC-INTENT-TAXONOMY` `.4` per-category levers; Gap A (register bit-fields) CLOSED (`.4a.ii`); cat-4 ISA decision RESOLVED (`.4d`).
- **✅ `DOC-INTENT-TAXONOMY.4d` DONE (this slice), committing now** — measurement + decision packet (read-only, docs-only, no Rust code). Resolved the cat-4 (CPU ISA) ISF-lowering Open Question. Measured the 2 cat-4 docs (RISC-V Debug **44 regs / 179 fields / 0 located**; RISC-V AIA **0 registers**, CSR intent in prose) + re-verified FSMGen pin `d327129b7`. **Decision (cat-4 splits 3 ways):** (1) CSR/register intent REUSES the existing register/storage abstraction — no new ISF construct, no FR (FSMGen titles `(storage (var … (fields …)))` the register-map/CSR construct, the same `.4a.ii` lowers cat-2/3 into); (2) the cat-4 register gap is EXTRACTION RECALL not abstraction → spun out **`.4d.i`** (locate RISC-V CSR fields + capture AIA CSR blocks → auto-lower via `.4a.ii`); (3) instruction/privilege/exception/memory-ordering = honest NON-TARGET (no FR; conditional-future only if FSMGen's SV/UVM path scopes ISA-model verification).
- **GATES (this slice):** no Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal); `check_doctrines.sh` 3/3 green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; KM derive-and-diff in sync (**116 facts** after adding `[[cat4-isa-csr-lowering-decision]]`).
- next_action (PNT): pick the next `DOC-INTENT-TAXONOMY.4` lever — **`.4d.i` cat-4 RISC-V CSR bit-position recovery** is the genuine buildable cat-4 lever (CODE — fresh focused session recommended for signoff; structural RISC-V CSR-layout grammar, no name list); **`.4e` conditional-rule lowering triage** (read-only, RAM-light) and **`.4c` cat-3 topology** (FSMGen-FR-likely — measure the submodule first, `[[feedback_verify_fsmgen_before_fr]]`) are the other measurement-first slices; **`.4b` Gap B (message-field structures)** STAYS FSMGen-gated (packet/flit deferred) AND needs an `Evidence→IntentIR` carrier. Other active trees: `CORPUS-COVERAGE` re-ingest 57 normalized-missing docs (RAM/Docling-gated, host-local source); `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` frontiers parked/deferred.
- in_flight_uncommitted: the **`DOC-INTENT-TAXONOMY.4d`** staged docs-only set (docs/research/cat4-isa-csr-lowering-decision.md, docs/knowledge/cat4-isa-csr-lowering-decision.md, KNOWLEDGE_MAP.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, docs/book/src/document-categories.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM healthy (~85% free), disk ~102G free. Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). `.4d`/`.4e`/`.4c` are read-only/measurement-first (RAM-light); the next CODE slice (`.4d.i`) warrants a fresh focused session for signoff sharpness. Artifact-cleanup directive standing (≥24h) — no new build artifacts generated this session.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
