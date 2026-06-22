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
- latest_commit (baseline): **`37a9f406`** `DOC-INTENT-TAXONOMY.4c.i — cat-3 topology-capture recall: capture-recall-gated, not abstraction-gated (no FR, honest residual)`. **37 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.4d.i` pre-investigation note) → 38 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-23`). `DOC-INTENT-TAXONOMY` `.4` per-category levers. This session committed **`.4c.i`** (cat-3 topology capture-recall measurement) `37a9f406`, then a **`.4d.i` pre-investigation note** (read-only feasibility probe, committing now). Bootstrap-read README/AGENTS/MEMORY-ARCH/MEMORY/active-tree first. Prior session: `.4a.ii` (Gap A CLOSED) + `.4d`/`.4c`/`.4e`. **The `.2` per-category ISF-completeness MEASUREMENT PHASE is COMPLETE (incl. cat-3 capture-recall) → remaining `.4` work is CODE.**
- **✅ `.4c.i` DONE + committed `37a9f406`** — cat-3 topology capture is **capture-recall-gated, NOT abstraction-gated** (0.355 `signal_connectivity` edges/actor vs wire's 4.108; 24% both-endpoint vs 85%; 0/10 infra rooted) → no FSMGen FR, honest residual, lever = upstream extraction outside `.4`. Reproducer `scripts/measure_cat3_topology_recall.py`.
- **`.4d.i` PRE-INVESTIGATION DONE (committing now, read-only, no code, NO decision — leaf STAYS `pending`):** RISC-V Debug bundle PRESENT (no re-ingest); 0/179 fields located. **The bit positions DO exist in the source** — a Docling-flattened register-DIAGRAM table adjacent to the `Field|Description|Access|Reset` table the current `field_table` strategy reads (e.g. `dmstatus` normalized `.md` ~L1042). Intricate/noisy: doubled wide-field cells, separate width + high-bit rows, two stacked half-rows, reserved `0` fields; diagram also an image (VLM NOT required). **Verdict: deterministically tractable but a substantial, regression-sensitive build** (touches the shared register path `.4a.ii`'s 24 docs / 6,570 fields rely on). Full design recorded in the `.4d.i` node.
- next_action (PNT): **`.4d.i` cat-4 RISC-V CSR bit-position recovery** — CODE, the genuine buildable lever; design starts from the recorded Pre-investigation (parse the register-diagram table → `bits_high`/`bits_low`, join to the field-desc table by name, ADR-0006 diagram-*shape* grammar; AIA 0-register recogniser is a larger sibling). Needs full task-acceptance checklist + `run_ci.sh` + FSMGen `--strict --check` 0-new-diagnostics. **A FRESH FOCUSED SESSION is recommended before `.4d.i`** for signoff sharpness (intricate, regression-sensitive). `.4b` Gap B stays FSMGen-gated. Other trees: `CORPUS-COVERAGE` re-ingest 57 docs (RAM/Docling-gated); `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` parked.
- in_flight_uncommitted: the **`.4d.i` pre-investigation note** staged set (docs/tasks/DOC-INTENT-TAXONOMY.md, CHANGES.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM healthy (~70% used / ~7G free, below the 85% kill line), disk ~102G free. Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). This session was read-only/RAM-light; `.4d.i` is a careful signoff Rust build (RISC-V register-diagram-table grammar) — keep RAM monitored, run `run_ci.sh` before committing it. Artifact-cleanup directive standing (≥24h) — no new build artifacts this session (only the gitignored `generated/mdbook` book render).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
