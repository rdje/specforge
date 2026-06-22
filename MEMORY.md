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
- latest_commit (baseline): **`ea39ec4f`** `DOC-INTENT-TAXONOMY.4c — cat-3 topology lowering decision: topology captured-but-sparse, no static-topology ISF construct, no FR yet (spun out .4c.i)`. **35 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.4e`) → 36 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-23`). `DOC-INTENT-TAXONOMY` `.4` per-category levers. This session committed **`.4d` + `.4c` + `.4e`** (3 docs-only measurement/decision slices). Gap A CLOSED (`.4a.ii`); cat-4 ISA RESOLVED (`.4d`); cat-3 topology RESOLVED (`.4c`); conditional-rule triage CLOSED (`.4e`). **The `.2` per-category ISF-completeness measurement phase is COMPLETE — remaining `.4` work is CODE.**
- **✅ `DOC-INTENT-TAXONOMY.4e` DONE (this slice), committing now** — measurement triage (read-only, docs-only, no Rust code). The `conditional_rules` ISF-lowering shortfall (`.2` Result 3) is HONEST RESIDUAL, not a gap. Classified 603 `conditional_rules` / 9 representative docs (all 4 buildable categories) vs each doc's declared-signal inventory + consequent quality: A no-consequent prose 392 (65%) / B undeclared 14 (2%) / C placeholder 33 (6%) / D declared+action 164 (27%). The D bucket is **161/164 bare deontic modals** (`shall`/`must`), only **3/603** concrete → lowering would fabricate the obligation. **No ISF lever, no FR** (the adapter already lowers the 516 cleanly-grounded conditional obligations corpus-wide); the only upside is upstream EXTRACTION quality (`extract-constraints-llm`), recorded as a cross-reference NOT a `.4` gap (`[[feedback_scoring_rigor]]`). Closes `.2` Result 3.
- **GATES (this slice):** no Rust code / no canonical mutation → wire golds / `kg-bench` / emitted `.isf` byte-identical by construction (WIRE-BASED-100 orthogonal); `check_doctrines.sh` 3/3 green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; KM derive-and-diff in sync (**118 facts** after adding `[[conditional-rule-lowering-triage]]`).
- next_action (PNT): the `.4` measurement phase is done — the remaining levers are **CODE**: **`.4d.i` cat-4 RISC-V CSR bit-position recovery** (the genuine buildable lever — locate RISC-V Debug's 179 fields + capture AIA's IMSIC/APLIC CSR blocks → auto-lower via `.4a.ii`; structural RISC-V CSR-layout grammar, no name list) and **`.4c.i` cat-3 topology-capture recall** (a read-only measurement that needs the 15 cat-3 doc set, then decides FR-vs-non-target). **`.4b` Gap B** STAYS FSMGen-gated (packet/flit) + needs an `Evidence→IntentIR` carrier. **A fresh focused session is recommended before the next CODE slice (`.4d.i`) for signoff sharpness** (this session already did extensive bootstrap reading + 3 decision packets). Other active trees: `CORPUS-COVERAGE` re-ingest 57 normalized-missing docs (RAM/Docling-gated, host-local source); `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` frontiers parked/deferred.
- in_flight_uncommitted: the **`DOC-INTENT-TAXONOMY.4e`** staged docs-only set (docs/research/conditional-rule-lowering-triage.md, docs/knowledge/conditional-rule-lowering-triage.md, KNOWLEDGE_MAP.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM healthy (~85% free), disk ~102G free. Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). `.4c.i` is read-only/RAM-light; the next CODE slice (`.4d.i`) warrants a fresh focused session for signoff sharpness. Artifact-cleanup directive standing (≥24h) — no new build artifacts generated this session (only the gitignored `generated/mdbook` book render).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
