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
- latest_commit (baseline): **`b3f05097`** `DOC-INTENT-TAXONOMY.3c — recognizer fixtures + user-facing mdBook chapter + KM card (.3 recognizer complete)`. **29 ahead** of `origin/main` (push at ~200 → HOLD). Next commit (`DOC-INTENT-TAXONOMY.4a`) → 30 ahead. Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (session `2026-06-22`). Executing the `DOC-INTENT-TAXONOMY` frontier — `.3` recognizer COMPLETE; `.4` per-category ISF-lowering levers OPEN.
- **✅ `DOC-INTENT-TAXONOMY.4a` DONE (this slice), committing now** — Gap A (register bit-field ISF lowering) measurement/design + verified FSMGen FR (docs-only, **no Rust code**). The bit-field intent reaches IntentIR FULLY (`RegisterFieldRecord` `source.rs:414` → `IntentIr.register_records` clone `intent.rs:193`), dropped ONLY at ISF emit (`IsfStorageVar` `isf_ir.rs:852`, opaque `(var NAME (width N) [(reset V)])` `:391`) → no SpecForge carry gap; ISF has NO field-structured-storage construct (verified pin `030f8c273`: opaque `(var)` only, `set-field`/`extract` are runtime ops) → filed verified FSMGen FR (`docs/FSMGEN_FEEDBACK.md`), no emitter hack. Gap B (1,220 msg-fields) shares the abstraction + lacks an Evidence→Intent carrier.
- **GATES (this slice):** `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (114→115 facts / 824 keys); **no code/canonical mutation → WIRE-BASED-100 + `kg-bench 156/156` + emitted `.isf` orthogonal by construction** (no cargo build needed — docs-only).
- next_action (PNT): **`DOC-INTENT-TAXONOMY.4a.i`** — emit the adapter honest residual `isf_register_fields_not_lowered` (today the field drop at `isf_ir.rs:852` is silent; only the *reset* drop is recorded as `isf_storage_reset_not_lowered`). RAM-light CODE slice, NO FSMGen dependency → full task-acceptance checklist + `run_ci.sh` + FSMGen `--strict --check` 0-new-diagnostics. Then `.4b` (Gap B Evidence→Intent message-field carrier). `.4a.ii` (field-structured emit) is GATED on FSMGen shipping the FR'd construct.
- in_flight_uncommitted: the **`.4a`** staged set (docs/FSMGEN_FEEDBACK.md, docs/research/register-bit-field-isf-lowering-design.md, docs/knowledge/register-bit-field-isf-lowering-gap.md, KNOWLEDGE_MAP.md, docs/book/src/pipeline/isf-adapter.md, docs/tasks/DOC-INTENT-TAXONOMY.md, docs/TASK_TREE.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM healthy. Builds run `CARGO_BUILD_JOBS=2`, RAM monitored, kill ≥85% used ([[feedback_ram_ceiling_monitor]]). `.4a.i` is the RAM-light next build (no Docling/VLM).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
