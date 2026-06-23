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
- latest_commit (baseline): **`afc46ba9`** `CORPUS-COVERAGE.2 — re-ingest #30: RISC-V AIA (1_0_2025_03_12) bundle-restoration + confirmation`. **60 ahead** of `origin/main` (push at ~200 → HOLD); this re-ingest-#31 commit → **61 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session started `2026-06-23`, now `2026-06-24`). Bootstrap-read all entry docs; surveyed ROADMAP + Rust codebase + mdBook + active frontiers (4 agents); doctrines GREEN. PNT this session: **slice 1 `KG-ISF-COMPLETENESS.3` CLOSED** (`5ad7d5c9`) → **#29 CHI C2C** (`4e15acb7`) → **#30 RISC-V AIA** (`afc46ba9`) → **slice 4 `CORPUS-COVERAGE.2` re-ingest #31 CHI C2C 2026** (this commit).
- **✅ `CORPUS-COVERAGE.2` re-ingest #31 DONE (`2026-06-24`) — AMBA CHI C2C 2026 variant `ihi0098_a_b`, marquee message-field refresh.** Binary current (no rebuild). `DOCLING_DEVICE=cpu ingest` (122pp/96 visual/0 residuals; normalized bundle RESTORED) → deterministic cascade. **`message_field_records` 0→143 / 12 containers** (sibling of #29's 0→149 — confirms the CHI-C2C family is a genuine marquee, not a one-off); transactions 2→3; lone stale `actor_signal_relations` 1→0 (fragment drop — coherency/message protocol, honest 0); conditional_rules 25 held; 0 registers. `agent.isf` renderable (69 ports/3 enums/0 rules), **FSMGen `--strict --check --json` 0 diagnostics**. Same surfaced residuals as #29 (generic-`TABLE` mega-enum + signal-acronym noise — not fixed in-slice). After #31: 31 re-ingested, **25 real chip-spec docs still normalized-missing**.
- next_action (PNT): **continue from the next eligible leaf.** Lanes: **(a)** `CORPUS-COVERAGE.2` continue the sweep (25 real docs remain — binary current, RAM headroom; remaining tail = OpenCAPI×~10 thin PHY/mech/TL + USB3.2 (548pp, large) + USB4 guides + JEDEC HBM-gen1 + CoreSight/debug guides — mostly bundle-restoration per the #22 finding; the CHI-C2C family marquees are now exhausted; pick pre-`.10`-stale register/protocol docs for real deltas); **(b)** RAM-safe doc-drift reconciliation (own a task-tree leaf first): shipped `nli-verify` lacks a dedicated mdBook command section (`docs/book/src/commands/quality-and-learning.md`); `RUST_CODEBASE_ANALYSIS.md` STALE (~103.2k LoC/1435 tests stated vs ~128.7k/lib 1709) — both fix real no-drift-doctrine ROADMAP↔codebase↔mdBook drift. **Consider switching to (b) soon — the re-ingest tail is now mostly thin/bundle-restoration; drift fixes are higher durable value + the user's #1 emphasis.**
- in_flight_uncommitted: **none** once committed — re-ingest #31 is docs-only (CORPUS-COVERAGE log row #31 + cumulative + changelog + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY); `generated/` artifacts git-ignored (durable trace = the `.2` log table). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
