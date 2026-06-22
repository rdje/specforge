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
- latest_commit (baseline): **`fc493b2b`** `DOC-INTENT-TAXONOMY.4a — Gap A register bit-field ISF lowering: verified FSMGen field-structured-storage FR (measurement/design)`. **30 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`FSMGEN-REFRESH-INTEGRATE-4.1`) → 31 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-22`). `DOC-INTENT-TAXONOMY` `.4` levers open; FSMGen refresh cycle 4 in flight.
- **✅ `FSMGEN-REFRESH-INTEGRATE-4.1` DONE (this slice), committing now** — owner-directed: FSMGen answered the `.4a` field-structured-storage FR. Bumped `subs/fsmgen` **`030f8c273 → 5ce0335c5`** (+106 commits); **verified on the new binary** (`*_passes_fsmgen_strict_validation` ×6 PASS + `run_ci.sh` 1696/0 + `kg-bench 156/156`; emitted `.isf` unchanged → no contract break, WIRE-BASED-100 orthogonal). **FSMGen ACCEPTED the FR** (real ISF gap + valid direction + the proposed shape) but **NOT shipped** — gated on FSMGen's `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1`; FSMGen confirmed the no-hack stance + the keep-as-metadata posture. FR RESOLVED in `docs/FSMGEN_FEEDBACK.md`; pin updated in README; tree `FSMGEN-REFRESH-INTEGRATE-4` CLOSED.
- **GATES (this slice):** `*_passes_fsmgen_strict_validation` ×6 PASS; `run_ci.sh` green (1696 tests / 0 failed, rustdoc, mdBook); `kg-bench 156/156`; `check_doctrines.sh` 3/3 PASS; KM in sync (115 facts / 826 keys). No SpecForge Rust code changed (submodule pin + docs only).
- next_action (PNT): **`DOC-INTENT-TAXONOMY.4a.i`** (now FSMGen-ENDORSED) — emit the adapter honest residual `isf_register_fields_not_lowered` (today the field drop at `isf_ir.rs:852` is silent; only the *reset* drop is recorded as `isf_storage_reset_not_lowered`); keep field maps as IntentIR metadata. RAM-light CODE slice, NO FSMGen dependency → full task-acceptance checklist + `run_ci.sh` + FSMGen `--strict --check` 0-new-diagnostics. Then `.4b` (Gap B Evidence→Intent message-field carrier). **`.4a.ii`** (field-structured emit) STAYS gated on FSMGen shipping the FR'd construct.
- in_flight_uncommitted: the **`FSMGEN-REFRESH-INTEGRATE-4.1`** staged set (subs/fsmgen gitlink, README.md, docs/FSMGEN_FEEDBACK.md, docs/tasks/FSMGEN-REFRESH-INTEGRATE-4.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, docs/knowledge/register-bit-field-isf-lowering-gap.md, KNOWLEDGE_MAP.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy (~83% free). Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). `.4a.i` is the RAM-light next build (no Docling/VLM).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
