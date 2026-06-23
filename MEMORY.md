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
- latest_commit (baseline): **`4e15acb7`** `CORPUS-COVERAGE.2 — re-ingest #29: AMBA CHI C2C (ihi0098_a) marquee message-field refresh 0->149`. **59 ahead** of `origin/main` (push at ~200 → HOLD); this re-ingest-#30 commit → **60 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session started `2026-06-23`, now `2026-06-24`). Bootstrap-read all entry docs; surveyed ROADMAP + Rust codebase + mdBook + active frontiers (4 agents); doctrines GREEN. PNT this session: **slice 1 `KG-ISF-COMPLETENESS.3` CLOSED** (`5ad7d5c9`) → **slice 2 re-ingest #29 CHI C2C** (`4e15acb7`) → **slice 3 `CORPUS-COVERAGE.2` re-ingest #30 RISC-V AIA** (this commit).
- **✅ `CORPUS-COVERAGE.2` re-ingest #30 DONE (`2026-06-24`) — RISC-V AIA `1_0_2025_03_12`, bundle-restoration + confirmation.** Binary already current from #29 (no rebuild). `DOCLING_DEVICE=cpu ingest` (89pp/105 visual/0 residuals; normalized bundle RESTORED) → deterministic cascade. **All deterministic surfaces byte-near-identical** (statements 1193, register_records 0, message_field_records 0, actor_signal_relations 0, conditional_rules 39 — all held; intent 8 actors/0 rel/0 txns) = the #21/#22 "already current-binary-equivalent" class; value = bundle restoration + confirmation. Honest absence: 0 registers (APLIC/IMSIC layouts in 12 `unknown` structured tables not matching the `.10` families → re-confirms the #21 RISC-V Lever-D recall opportunity). `agent.isf` renderable (91 ports/5 enums/22 rules), **FSMGen `--strict --check --json` 0 diagnostics**. (Prior slice #29 CHI C2C: message_field_records 0→149 marquee.) After #30: 30 re-ingested, **26 real chip-spec docs still normalized-missing**.
- next_action (PNT): **continue from the next eligible leaf.** Lanes: **(a)** `CORPUS-COVERAGE.2` continue the sweep (26 real docs remain — binary current, RAM headroom; remaining tail = OpenCAPI×~10 thin PHY/mech/TL + USB3.2 (548pp, large) + USB4 guides + JEDEC HBM-gen1 + CoreSight/debug guides — per the #22 phase finding mostly bundle-restoration, marquee jumps the exception; pick pre-`.10`-stale register/protocol docs for real deltas); **(b)** RAM-safe doc-drift reconciliation (own a task-tree leaf first): shipped `nli-verify` lacks a dedicated mdBook command section (`docs/book/src/commands/quality-and-learning.md`); `RUST_CODEBASE_ANALYSIS.md` STALE (~103.2k LoC/1435 tests stated vs ~128.7k/lib 1709) — both fix real no-drift-doctrine ROADMAP↔codebase↔mdBook drift.
- in_flight_uncommitted: **none** once committed — re-ingest #30 is docs-only (CORPUS-COVERAGE log row #30 + cumulative + changelog + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY); `generated/` artifacts git-ignored (durable trace = the `.2` log table). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
