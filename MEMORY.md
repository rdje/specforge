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
- latest_commit (baseline): **`fd54ca80`** `AUDIT-DOC-RECONCILE.3 — refresh RUST_CODEBASE_ANALYSIS.md size/command/module/test counts`. **64 ahead** of `origin/main` (push at ~200 → HOLD); this `BOOK-COMMAND-COVERAGE.3` commit → **65 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (this conversation re-ran the full README/SESSION_BOOTSTRAP bootstrap `2026-06-24`: read entry docs + `docs/TASK_TREE.md`; re-surveyed ROADMAP + Rust codebase (RUST_CODEBASE_ANALYSIS.md verified CURRENT at HEAD — no update needed) + mdBook + the 4 active frontiers via 3 agents; doctrines GREEN; RAM 84% free; release binary current). PNT slice this turn: **`BOOK-COMMAND-COVERAGE.3`** (mdBook overview-list drift fix).
- **✅ `BOOK-COMMAND-COVERAGE.3` DONE (`2026-06-24`, docs-only) — completed the Commands-overview "Current surface" list (21→28).** mdBook survey + hand re-derivation (don't trust the agent count — the `.1` precedent): `cli.rs` has **28** subcommands; `docs/book/src/commands/overview.md` listed **21** — missing the 7 EXTRACTION-QUALITY-GAUGE/GRITS members (`nli-verify`/`entity-type`/`extract-conditions`/`extract-constraints-llm`/`eval-extraction`/`audit-extraction`/`grits-consensus`), each of which already had a dedicated section (so `.1`/`.2`'s "every command has a section" bar held; only the overview list was stale). Added the 7 after `signal-resolve` (README order, flags cross-checked vs the `cli.rs` clap structs not README) + a nav pointer whose no-op claim was CORRECTED after re-deriving defaults (extraction/judge default live `ollama`; only eval/audit default `skip`; grits offline). `mdbook build` exit 0; list = 28 bullets. Owned by re-opening `BOOK-COMMAND-COVERAGE` (`.3`); also refreshed the stale TASK_TREE.md row (it still showed only `.1`).
- next_action (PNT): **book overview-list drift reconciled** (third doc/book drift this session-class, after the nli-verify section + RUST_CODEBASE_ANALYSIS counts). Continue from the next eligible leaf: **(a)** `CORPUS-COVERAGE.2` resume the re-ingest sweep (**25 real docs remain** normalized-missing — binary current; tail = thin OpenCAPI×~13 PHY/mech/TL + USB3.2 (548pp) + USB4 guides + JEDEC HBM-gen1 `jesd235` + CoreSight/debug guides; mostly bundle-restoration, `jesd235` HBM-gen1 = the best remaining register/timing-table candidate for a real delta); **(b)** own a measurement-first leaf for a surfaced upstream extraction-precision gap (generic-`TABLE` mega-enum conflation [HBM2/CHI] or signal-inventory prose-acronym noise [`AES`/`AMBA`/`ARM`, the `KG-ISF-COMPLETENESS.4` class]).
- in_flight_uncommitted: **none** once committed — BOOK-COMMAND-COVERAGE.3 is docs-only (`commands/overview.md` + tree `.3` + `docs/TASK_TREE.md` row + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
