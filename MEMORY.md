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
- latest_commit (baseline): **`5ad7d5c9`** `KG-ISF-COMPLETENESS.3 — CLOSED: north-star bar #2 relation-completeness resolved (verification-only)`. **58 ahead** of `origin/main` (push at ~200 → HOLD); this re-ingest-#29 commit → **59 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session started `2026-06-23`, now `2026-06-24`). Bootstrap-read all entry docs; surveyed ROADMAP + Rust codebase + mdBook + active frontiers (4 agents); doctrines GREEN. PNT this session: **slice 1 `KG-ISF-COMPLETENESS.3` CLOSED** (`5ad7d5c9`) → **slice 2 `CORPUS-COVERAGE.2` re-ingest #29** (this commit).
- **✅ `CORPUS-COVERAGE.2` re-ingest #29 DONE (`2026-06-24`) — AMBA CHI C2C `ihi0098_a`, marquee message-field refresh.** Release binary first rebuilt to current (persisted one predated `.2a.v/.vi`; `isf_rule_value` symbol now present). `DOCLING_DEVICE=cpu ingest` (108pp/88 visual/0 residuals; normalized bundle RESTORED) → deterministic `evidence→semantic→intent→adapt`. **`message_field_records` 0→149 / 13 containers** (current `.10` families fire on CHI packet/flit field tables absent in stale pre-`.10` evidence — real KG gain, not bundle-restore); 0 reg / 0 rel (coherency/message protocol — honest absence, `KG-ISF-COMPLETENESS.3`); cond_rules 17 + txns 2 held. `agent.isf` renderable (69 ports/3 enums/1 rule), **FSMGen `--strict --check --json` 0 diagnostics**. Surfaced (NOT fixed in-slice — upstream extraction-precision residuals): generic-`TABLE` mega-enum conflation (#28 class) + signal-acronym prose noise (`AES`/`AMBA`/`ARM`, `KG-ISF-COMPLETENESS.4` class). After #29: 29 re-ingested, **27 real chip-spec docs still normalized-missing**.
- next_action (PNT): **continue from the next eligible leaf.** Two viable lanes: **(a)** `CORPUS-COVERAGE.2` continue the re-ingest sweep (27 real docs remain — binary now current, RAM headroom; next candidates the OpenCAPI×13 thin tail / USB / RISC-V-AIA / JEDEC HBM gen1 — most are bundle-restoration + confirmation per the #22 phase finding, marquee jumps now the exception); **(b)** RAM-safe doc-drift reconciliation (own under a task-tree leaf first): the shipped `nli-verify` lacks a dedicated mdBook command section (`docs/book/src/commands/quality-and-learning.md`), and `RUST_CODEBASE_ANALYSIS.md` is STALE (states ~103.2k LoC / 1435 tests; actual ~128.7k LoC / lib 1709) — both fix real ROADMAP↔codebase↔mdBook drift (no-drift doctrine).
- in_flight_uncommitted: **none** once committed — re-ingest #29 is docs-only (CORPUS-COVERAGE log row + changelog + CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY); `generated/` artifacts are git-ignored (durable trace = the `.2` log table). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** host RAM headroom this session (~70–84% free; well under the ≥85%-used kill line, [[feedback_ram_ceiling_monitor]]); Docling re-ingest is viable — serialize vs the 14B model + monitor. **Release binary is now CURRENT (rebuilt `2026-06-24` post-`.2a.vi`)** — future re-ingest/adapt slices can reuse it (no rebuild) until the next code change. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. `generated/` git-ignored. Artifact-cleanup standing (≥24h).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
