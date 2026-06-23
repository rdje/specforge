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
- latest_commit (baseline): **`c212a6d0`** `BOOK-COMMAND-COVERAGE.2 — MEMORY post-commit refresh (resume pointer -> HEAD 20ec8e78)`. **63 ahead** of `origin/main` (push at ~200 → HOLD); this `AUDIT-DOC-RECONCILE.3` commit → **64 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session started `2026-06-23`, now `2026-06-24`). Bootstrap-read all entry docs; surveyed ROADMAP + Rust codebase + mdBook + active frontiers (4 agents); doctrines GREEN. PNT this session (6 slices): **`KG-ISF-COMPLETENESS.3` CLOSED** (`5ad7d5c9`) → re-ingest **#29 CHI C2C** (`4e15acb7`) → **#30 RISC-V AIA** (`afc46ba9`) → **#31 CHI C2C 2026** (`b18f9729`) → **`BOOK-COMMAND-COVERAGE.2`** (`20ec8e78`) → **`AUDIT-DOC-RECONCILE.3`** (this commit).
- **✅ `AUDIT-DOC-RECONCILE.3` DONE (`2026-06-24`, docs-only) — refreshed `RUST_CODEBASE_ANALYSIS.md` to live counts.** The second discovered live-doc drift (after the nli-verify book gap). The doc's `2026-06-08` currency section had drifted four counts; refreshed each with a reproducible command, **verified at HEAD (not trusting the survey agent's numbers):** ≈103.2K→**128,742 LoC / 65 files**; 25→**28 subcommands**; 25→**28 IR modules**; 1435→**1709 lib tests** (`cargo test --lib` ran fresh, 0 failed / 4 ignored). Added a largest-modules table; older dated sections left verbatim; no architecture claim reversed. **Self-check withdrew one over-specific sub-claim** (the old entry's command enumeration undercounted — described `recover-register-bits` but omitted it). Owned by re-opening `AUDIT-DOC-RECONCILE` (`.3`; the doc-reconciliation tree, now the standing home for one-off live-doc↔code currency fixes). Lib unchanged; oracles orthogonal; doctrines GREEN.
- next_action (PNT): **both discovered live-doc drifts now reconciled** (nli-verify book section + RUST_CODEBASE_ANALYSIS.md counts). Continue from the next eligible leaf: **(a)** `CORPUS-COVERAGE.2` resume the re-ingest sweep (25 real docs remain — binary current; tail mostly thin OpenCAPI×~10 PHY/mech + USB3.2 (548pp) + USB4 guides + JEDEC HBM-gen1 + CoreSight/debug guides; mostly bundle-restoration per #22, pick pre-`.10`-stale register/protocol docs for real deltas); **(b)** revisit a substantive north-star upstream gap (dense-prose agent precision `.1c.ii` is NLP-gated; the `KG-ISF-COMPLETENESS.4` signal-inventory prose-noise + generic-`TABLE` mega-enum levers re-surfaced by the CHI re-ingests are upstream extraction-precision candidates needing their own owned measurement-first leaves).
- in_flight_uncommitted: **none** once committed — AUDIT-DOC-RECONCILE.3 is docs-only (`RUST_CODEBASE_ANALYSIS.md` + tree + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
