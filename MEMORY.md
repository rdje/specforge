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
- latest_commit (baseline): **`b04f2b4a`** `DOC-INTENT-TAXONOMY.4d — cat-4 ISA/CSR lowering decision: CSRs reuse register/storage; gap is extraction recall (spun out .4d.i)`. **34 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.4c`) → 35 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-23`). `DOC-INTENT-TAXONOMY` `.4` per-category levers; Gap A (register bit-fields) CLOSED (`.4a.ii`); cat-4 ISA decision RESOLVED (`.4d`); cat-3 topology decision RESOLVED (`.4c`). This session: `.4d` + `.4c` committed.
- **✅ `DOC-INTENT-TAXONOMY.4c` DONE (this slice), committing now** — measurement + decision packet (read-only, docs-only, no Rust code). Resolved the cat-3 (platform / system-IP topology) ISF-lowering question. Measured 3 representative cat-3 docs (CoreSight SoC-600 60 actors / **6** `signal_connectivity` / 833 regs; GIC-600 55 actors / **66** `signal_connectivity` / **2** `infrastructure_signals`; CoreSight BSA 5 actors / prose) + re-verified FSMGen pin `d327129b7`. **Decision (two halves):** (1) cat-3's register half already lowers (register maps + bit-fields via `.4a.ii`; same road as cat-2 — not the gap); (2) topology IS captured (typed `signal_connectivity` producer→consumer + `infrastructure_signals` clock/reset distribution; refining `.2`'s "hint-level" to captured-but-sparse-and-unlowered) but ISF has NO declarative static-topology construct (composition is transaction-level only; FSMGen's ATL frontier is behavioral generated-child wiring, not a netlist; the emit is single-initiator-actor). **NO FR yet** (premature — capture sparse/noisy + ISF is a per-actor format where static topology may be the integrator's concern above per-module synthesis) → spun out **`.4c.i`** topology-capture recall measurement.
- **GATES (this slice):** no Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal); `check_doctrines.sh` 3/3 green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; KM derive-and-diff in sync (**117 facts** after adding `[[cat3-topology-isf-lowering-decision]]`).
- next_action (PNT): pick the next `DOC-INTENT-TAXONOMY.4` lever — **`.4c.i` cat-3 topology-capture recall** (measurement, read-only, RAM-light) and **`.4e` conditional-rule lowering triage** (`.2` Result 3, read-only, RAM-light) are the buildable measurement-first slices; **`.4d.i` cat-4 RISC-V CSR bit-position recovery** is the genuine buildable CODE lever (fresh focused session recommended for signoff; structural RISC-V CSR-layout grammar, no name list); **`.4b` Gap B (message-field structures)** STAYS FSMGen-gated (packet/flit deferred) AND needs an `Evidence→IntentIR` carrier. Other active trees: `CORPUS-COVERAGE` re-ingest 57 normalized-missing docs (RAM/Docling-gated, host-local source); `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` frontiers parked/deferred.
- in_flight_uncommitted: the **`DOC-INTENT-TAXONOMY.4c`** staged docs-only set (docs/research/cat3-topology-isf-lowering-decision.md, docs/knowledge/cat3-topology-isf-lowering-decision.md, KNOWLEDGE_MAP.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, docs/book/src/document-categories.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM healthy (~85% free), disk ~102G free. Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). `.4c.i`/`.4e` are read-only/measurement-first (RAM-light); the next CODE slice (`.4d.i`) warrants a fresh focused session for signoff sharpness. Artifact-cleanup directive standing (≥24h) — no new build artifacts generated this session (only the gitignored `generated/mdbook` book render).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
