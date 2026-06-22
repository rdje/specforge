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
- latest_commit (baseline): **`368970b8`** `FSMGEN-REFRESH-INTEGRATE-4.1 — bump subs/fsmgen 030f8c273->5ce0335c5 + integrate the accepted field-structured-storage FR answer`. **31 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`FSMGEN-REFRESH-INTEGRATE-5.1`) → 32 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-22`). FSMGen refresh cycle 5 in flight; `DOC-INTENT-TAXONOMY` `.4` levers open.
- **✅ `FSMGEN-REFRESH-INTEGRATE-5.1` DONE (this slice), committing now** — owner-directed: FSMGen **SHIPPED** the field-structured-storage construct the `.4a` FR requested. Bumped `subs/fsmgen` **`5ce0335c5 → d327129b7`** (+2 commits: `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1`/`.2`); **verified on the new binary** (`*_passes_fsmgen_strict_validation` ×6 PASS + `run_ci.sh` green + `kg-bench 156/156`; feature is additive/metadata-only → emitted `.isf` byte-identical, WIRE-BASED-100 orthogonal, no contract break). FR marked SHIPPED in `docs/FSMGEN_FEEDBACK.md`; README pin updated; tree `FSMGEN-REFRESH-INTEGRATE-5` CLOSED.
- **GATES (this slice):** `*_passes_fsmgen_strict_validation` ×6 PASS; `run_ci.sh` green (mdBook built); `kg-bench 156/156`; `check_doctrines.sh` 3/3; KM in sync (115 facts / 828 keys). No SpecForge Rust code changed (submodule pin + docs only).
- next_action (PNT): **`DOC-INTENT-TAXONOMY.4a.ii`** (now **UN-GATED + buildable** — the highest-leverage lever) — emit the register bit-field map into the SHIPPED ISF construct `(storage (var NAME (width N) [(reset V)] (fields (field FNAME (bits HI LO) [(access …)] [(reset V)] [(enum …)]))))`. Map `RegisterFieldRecord`→fields (access normalized to `ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved`, OMIT if unmapped; field `(reset)` only when parent reset composed; `(enum)` members that fit). REUSE the `classify_register_reset`/`register_field_extent` tiling gate in `isf_ir.rs` (overlap/out-of-width fail closed). Metadata-only/schedule-safe → 4 wire golds byte-identical. Verify via report `inferred_storage[].fields[]` + `*_passes_fsmgen_strict_validation` 0-new-diagnostics. FULL task-acceptance checklist + `run_ci.sh`. Design/grammar captured in the `.4a.ii` node + `[[register-bit-field-isf-lowering-gap]]`. (`.4a.i` honest-residual = cheap fallback; `.4b` Gap-B packet/flit STAYS gated — FSMGen-deferred.)
- in_flight_uncommitted: the **`FSMGEN-REFRESH-INTEGRATE-5.1`** staged set (subs/fsmgen gitlink, README.md, docs/FSMGEN_FEEDBACK.md, docs/tasks/FSMGEN-REFRESH-INTEGRATE-5.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, docs/knowledge/register-bit-field-isf-lowering-gap.md, KNOWLEDGE_MAP.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** (owner will `/exit`).
- **OPERATIONAL REALITY:** host RAM healthy (~83% free). Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). `.4a.ii` is a Rust emitter slice (isf_ir.rs) — RAM-light (no Docling/VLM), but a fresh focused session is recommended for signoff quality.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
