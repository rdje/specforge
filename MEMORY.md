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
- latest_commit (baseline): **`7c46b569`** `BOOK-COMMAND-COVERAGE.3 — complete the Commands overview "Current surface" list (21->28)`. **65 ahead** of `origin/main` (push at ~200 → HOLD); this `CORPUS-COVERAGE.2 #32` commit → **66 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (this conversation re-ran the full README/SESSION_BOOTSTRAP bootstrap `2026-06-24`: read entry docs + `docs/TASK_TREE.md`; re-surveyed ROADMAP + codebase (RUST_CODEBASE_ANALYSIS.md verified CURRENT) + mdBook + the 4 active frontiers via agents; doctrines GREEN). PNT slices this conversation (2): **`BOOK-COMMAND-COVERAGE.3`** (`7c46b569`, mdBook overview-list 21→28) → **`CORPUS-COVERAGE.2` re-ingest #32** (this commit).
- **✅ `CORPUS-COVERAGE.2` re-ingest #32 DONE (`2026-06-24`) — JEDEC HBM-gen1 (`jesd235`) bundle-restoration + corpus-provenance finding.** The library `JESD235_2013-10_HBM_DRAM.pdf` is a **6-page legal-exhibit cover** (`DOCKET`/`ALARM`/`Netlist Ex 2021` + 12 cover images, 0 tables), NOT the real HBM gen1 standard → thin yield is honest source-driven absence (register/relations/constraints 0 held; statements 69→68; intent actors 2→2 / interfaces 1→7 minor regroup); `adapt` recovers 16 signals on actor `channel` but **BLOCKS honestly** ("no behavioral content", 1 residual, no `.isf` fabricated — the `CORPUS-COVERAGE.0` intent-no-isf class). normalized bundle RESTORED (Docling CPU, RAM 83% free, staged-swap). Durable trace = the `.2` ledger row.
- next_action (PNT): **the re-ingest tail is now empirically confirmed low-value** (#32 = a legal-exhibit excerpt; the remaining **24 real docs** are OpenCAPI×13 PHY/mech/TL + USB3.2/USB4 guides + CoreSight/debug guides — thin/bundle-restoration). PER `[[feedback_not_complete_attack_substantive_gaps]]` the next PNT slice should PIVOT to OWN a **measurement-first leaf for a surfaced upstream extraction-precision lever**: (a) generic-`TABLE` mega-enum conflation (HBM2 #28 + CHI #29 — conflates ~10 distinct value-tables into one `(type TABLE)` enum w/ dup values + fragment member names; reaches `.isf` → bar #6), or (b) signal-inventory prose-acronym noise (`AES`/`AMBA`/`ARM` minted as interface ports — `KG-ISF-COMPLETENESS.4` bar #1/#3 class). Both RAM-light, read-only-first. Re-ingest stays available when RAM is free but is no longer the high-value lane.
- in_flight_uncommitted: **none** once committed — re-ingest #32 docs = `CORPUS-COVERAGE.2` ledger row + frontier count + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY (`generated/` git-ignored). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
