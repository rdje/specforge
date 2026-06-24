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
- latest_commit (baseline): **`3795151c`** `CORPUS-COVERAGE.2 — re-ingest #32: JEDEC HBM-gen1 (jesd235) bundle-restoration + provenance finding`. **66 ahead** of `origin/main` (push at ~200 → HOLD); this `KG-ISF-COMPLETENESS.5` commit → **67 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (this conversation re-ran the full README/SESSION_BOOTSTRAP bootstrap `2026-06-24`: read entry docs + `docs/TASK_TREE.md`; re-surveyed ROADMAP + codebase (RUST_CODEBASE_ANALYSIS.md verified CURRENT) + mdBook + the 4 active frontiers; doctrines GREEN). PNT slices this conversation (3): **`BOOK-COMMAND-COVERAGE.3`** (`7c46b569`, mdBook overview-list 21→28) → **`CORPUS-COVERAGE.2` re-ingest #32** (`3795151c`, HBM-gen1 legal-exhibit-cover finding) → **`KG-ISF-COMPLETENESS.5`** (this commit, generic-enum measurement).
- **🧭 `KG-ISF-COMPLETENESS.5` DONE (`2026-06-24`, measurement, read-only, docs-only) — generic-`TABLE` mega-enum conflation localized + measured + decided.** The `.isf` emits a junk generic enum (HBM2 `(type TABLE (bits 6))`) fusing ~7 value-tables (bar #6 defect, deferred by `.2a.iv`). **EXTRACTION-born:** `derive_encoding_enum_name` fallback names an unmatched encoding table after its caption keyword (`Table N`→`TABLE`, `evidence.rs:4457-4461`/`:7106`); `build_symbol_definitions` merges same-named tables into one (`semantic.rs:2782`). 56/78 docs / 96 generic + **271 real-named-but-junk** enums → name-only gate insufficient, the signal is member quality. Decision GO, extraction-side. Report `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]` (map 125→126).
- next_action (PNT): the next buildable code leaf is **`KG-ISF-COMPLETENESS.5.i`** — the safe extraction fallback name-gate (`derive_encoding_enum_name` returns `None` unless the token is a declared signal → kills the 96 generic enums + the conflation) + the emitter orphan-`(type)` fix (`isf_ir.rs:403-409` gate types block by `emitted_enums()`). **⚠️ HIGH-STAKES GATE CODE — best on a FRESH session:** it changes wire-gold `.isf` BYTES (each of APB/AHB/AXI/SWP emits a junk `TABLE`; AHB's fuses HTRANS+HSIZE) — a strict improvement, WIRE-BASED-100 scores orthogonal, but needs a deliberate per-doc before/after snapshot refresh + the enforced TASK-ACCEPTANCE checklist + `run_ci`/`kg-bench`/`fsmgen --strict` + the book update (the `.isf` change is user-visible). `.5.ii` (member-quality gate) is calibration-gated. Re-ingest (`CORPUS-COVERAGE.2`, 24 thin docs left) stays available when RAM is free but is the low-value lane.
- in_flight_uncommitted: **none** once committed — `.5` docs = research report + KM card + `KNOWLEDGE_MAP.md` regen + `KG-ISF-COMPLETENESS.md` leaves + `docs/TASK_TREE.md` row + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY. blockers: none. **Repo HANDOFF-READY** (clean boundary for the `.5.i` code lever on a fresh session).
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
