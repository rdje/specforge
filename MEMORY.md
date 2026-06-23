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
- latest_commit (baseline): **`4509ae8c`** `CORPUS-COVERAGE.2 — re-ingest #24 CoreSight SoC-600 0200 TRM …`. **42 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2 #25` commit → **43 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (fresh session `2026-06-23`). Active tree: **`CORPUS-COVERAGE.2`** (host-local re-ingest of the 57 normalized-missing docs; owner re-provisioned `chipdoc` via the git-ignored `.cache/local-references/chipdoc` symlink). This session bootstrap-read README/AGENTS/MEMORY-ARCH/MEMORY + ROADMAP + mapped codebase + mdBook, surveyed the active trees (NLP-SHALLOW-PARSE / MEMORY-BOUNDED-INGEST = exhausted; CORPUS-PATTERN-REUSE = owner-gated; KG-ISF-COMPLETENESS/TRANSACTIONS = FSMGen-parked), and PNT-picked the active owner-provisioned re-ingest frontier. **Done this session: #22 (`5327c1e3`), #23 (`130672e9`), #24 (`4509ae8c`), #25 (committing now).**
- **🚧 `CORPUS-COVERAGE.2` re-ingest #25 DONE (committing now) — CoreSight SoC-600 0100 TRM** (`100806_0100`, 702pp / 1157 visual). Docling CPU (0 residuals; RAM 67–77% free; ~5 min) → deterministic cascade. **GENUINE refresh (same #23/#24/#17 class) — COMPLETES the CoreSight SoC-600 cluster (0701/0200/0100):** actors 44→34 / relations 41→31 / interfaces 12→5 (consolidation) + transactions 0→1 (NEW); register_records 597 held (exact), 0 msg-fields (honest). `.isf` renderable (`dp.isf`, 597 storage / 12 enums / 3 rules / 4 signals); **`fsmgen --strict` success / 0 diagnostics**; `validate` no stage-staleness (31-vs-31), 52/100 ADEQUATE. Residuals: bit-fields + field-resets unlowered; 1 `isf_temporal_unrepresentable` (honest, not emitted). **Coverage 25 of 57. 23/25 strict-clean** (2 FAIL = DTI Lever A, LPI Lever C).
- **PHASE FINDING (#22–#25):** "already current-binary-equivalent" is NOT uniform — a doc's stale evidence sits at whatever binary last rebuilt it. #22 Cortex-A76 was fully current (byte-identical confirmation); the 3 CoreSight SoC-600 versions (#23/#24/#25) predated two later gates → uniformly genuine consolidation+transaction refreshes (the #17 Avalon class). So the remaining tail MIXES pure confirmation (#21/#22) and consolidation/recognition refreshes (#17/#23/#24/#25) + bundle restoration; marquee table-family jumps now the exception.
- next_action (PNT): **continue `CORPUS-COVERAGE.2`** — next re-ingest slice: Intel VT-d (`5_0_…`, 103 regs, 12MB — distinct vendor; bigger so watch RAM), then JEDEC eMMC/HBM (`JESD84-B50`/`JESD235*`), then the thin OpenCAPI/PHY/guide tail. Check RAM first; one doc per slice; commit per COMMIT.md. **Alternatively** the most substantive non-gated CODE lever is **Lever D** (non-AMBA `Signal|Direction|Width` table-shape + Wishbone `*_O`/`*_I` signal-recall, surfaced #18/#19) — needs probe-first design + its own owned `PDF-VARIANT-DIGESTION` leaf (kin to parked `.9.10`; mind owner no-denylist steer). `KG-ISF-COMPLETENESS`/`KG-ISF-TRANSACTIONS` FSMGen-parked.
- in_flight_uncommitted: the **`CORPUS-COVERAGE.2 #25`** staged set (docs/tasks/CORPUS-COVERAGE.md, CHANGES.md, DEVELOPMENT_NOTES.md, LIVE_ACHIEVEMENT_STATUS.md, MEMORY.md). blockers: none. **Repo HANDOFF-READY** after commit. (generated/ is git-ignored — the `.2` log table is the durable trace.)
- **OPERATIONAL REALITY:** host RAM monitor before any Docling/14B work — kill ≥85% used ([[feedback_ram_ceiling_monitor]]). This session ran FOUR CPU Docling re-ingests (#22–#25; RAM stayed 66–83% free, no abort); no Rust build (binary current at HEAD). A transient opus-4-8[1m] Bash-classifier outage briefly blocked the #23 cascade (re-ran cleanly on recovery). Artifact-cleanup directive standing (≥24h) — re-ingest restored four `normalized/` bundles (intended); no stray build artifacts.
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
