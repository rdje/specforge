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
- latest_commit (baseline): **`b18f9729`** `CORPUS-COVERAGE.2 — re-ingest #31: AMBA CHI C2C 2026 variant (ihi0098_a_b) marquee message-field refresh 0->143`. **61 ahead** of `origin/main` (push at ~200 → HOLD); this BOOK-COMMAND-COVERAGE.2 commit → **62 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session started `2026-06-23`, now `2026-06-24`). Bootstrap-read all entry docs; surveyed ROADMAP + Rust codebase + mdBook + active frontiers (4 agents); doctrines GREEN. PNT this session: **`KG-ISF-COMPLETENESS.3` CLOSED** (`5ad7d5c9`) → re-ingest **#29 CHI C2C** (`4e15acb7`) → **#30 RISC-V AIA** (`afc46ba9`) → **#31 CHI C2C 2026** (`b18f9729`) → **slice 5 `BOOK-COMMAND-COVERAGE.2`** (this commit).
- **✅ `BOOK-COMMAND-COVERAGE.2` DONE (`2026-06-24`, docs-only) — dedicated `nli-verify` section in the Commands chapter.** Lane-switch off the thinning re-ingest tail to the user's #1 no-drift concern. The session-start mdBook survey found `nli-verify` was the ONLY quality command without a dedicated `##` section in `commands/quality-and-learning.md` (14 of 15 had one; its treatment lived only in `architecture-rationale.md`). NOT book-vs-code drift (`.1` correctly judged it covered under the ≥1-mention bar) — a commands-chapter navigability gap. Added the `## nli-verify` section (invocation, NLI premise/hypothesis entailment, CI-safe abstain, text-model, persisted `extraction_quality_gauge` + vacuous-pass-never-persisted rule, active-gate `intent --nli-verify` → `nli_demoted_contracts`); facts cross-checked vs `commands/nli_verify.rs` + `cli.rs`; `mdbook build` exit 0; sections 14→15. Re-opened `BOOK-COMMAND-COVERAGE` with `.2` (bar raised to "every command has a dedicated Commands-chapter section"). Lib unchanged; oracles orthogonal; doctrines GREEN.
- next_action (PNT): **continue from the next eligible leaf.** Lanes: **(a)** the other discovered drift — refresh STALE `RUST_CODEBASE_ANALYSIS.md` (states ~103.2k LoC / 1435 tests; actual ~128.7k LoC / lib 1709; +the "25 vs 26 commands" + largest-modules table) — clearer, higher-impact drift than the nli-verify nav gap was (own a small leaf or fold into AUDIT-DOC-RECONCILE family); **(b)** `CORPUS-COVERAGE.2` continue the sweep (25 real docs remain — binary current; tail mostly thin OpenCAPI/USB/guides + bundle-restoration; pick pre-`.10`-stale register/protocol docs for real deltas). Recommend (a): the codebase-analysis staleness actively misstates the codebase to any resuming agent.
- in_flight_uncommitted: **none** once committed — BOOK-COMMAND-COVERAGE.2 is docs-only (book `commands/quality-and-learning.md` + tree + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
