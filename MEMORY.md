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
- latest_commit (baseline): **`7dc19d03`** `DOC-INTENT-TAXONOMY.4d.i — pre-investigation feasibility probe (read-only, no code; leaf stays pending)`. **38 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`.4d.i` resolution) → 39 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-23`). `DOC-INTENT-TAXONOMY` `.4`. This session bootstrap-read README/AGENTS/MEMORY-ARCH/MEMORY/active-tree + roadmap + mdBook + register codebase, then **resolved `.4d.i`** (committing now). Prior sessions: `.4a.ii` (Gap A CLOSED), `.4d`/`.4c`/`.4e`/`.4c.i`. **The `.2` measurement phase is COMPLETE; the `.4` actionable frontier is now EXHAUSTED except the FSMGen-gated `.4b`.**
- **✅ `.4d.i` DONE (committing now)** — the planned cat-4 deterministic CSR bit-recovery CODE was MEASURED non-viable BEFORE writing it (gold-checked, overturns the pre-investigation's optimism): RISC-V Debug bit positions live in the layout **IMAGE for 53/56** registers (cleanest gold reg `dmcontrol` image-only, no table); the 7 flattened tables are garbled (`dmstatus` off-by-8 + dropped 7-field band) / XLEN-symbolic (`tdata1`); the tiling gates are **order-blind** → a deterministic parse fabricates/recovers ~0. Genuine lever = sharper VLM read for the existing `register_bits.rs`/`recover-register-bits` path (VLM-accuracy-bound), owned OUTSIDE `.4`; AIA blocked on re-ingest + prose-bound. **Honest residual, no code, no FR.** Reproducer `scripts/measure_cat4_csr_bit_recovery.py`; packet `docs/research/cat4-csr-bit-position-recovery-measurement.md`; KM `[[cat4-csr-bit-position-recovery-not-deterministic]]`.
- next_action (PNT): the `DOC-INTENT-TAXONOMY.4` actionable frontier is exhausted (only the FSMGen-gated `.4b` packet/flit remains) → **PNT-select the next substantive leaf across the other active trees**, honoring [[feedback_not_complete_attack_substantive_gaps]] (attack north-star gaps, not doc-hygiene). Candidates: `CORPUS-COVERAGE` re-ingest 57 docs (RAM/Docling-gated — check RAM first); `NLP-SHALLOW-PARSE` spike (`.2h`, in_progress); `CORPUS-PATTERN-REUSE`; `MEMORY-BOUNDED-INGEST`. `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` parked (FSMGen-gated).
- in_flight_uncommitted: the **`.4d.i` resolution** staged set (docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, docs/research/cat4-csr-bit-position-recovery-measurement.md, docs/knowledge/cat4-csr-bit-position-recovery-not-deterministic.md, KNOWLEDGE_MAP.md, scripts/measure_cat4_csr_bit_recovery.py, docs/book/src/document-categories.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit.
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]). This session was read-only/RAM-light (no Rust build, no Docling). Artifact-cleanup directive standing (≥24h) — no new build artifacts this session (only the gitignored `generated/mdbook` book render).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
