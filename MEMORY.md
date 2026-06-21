# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002); retrieval index: `KNOWLEDGE_MAP.md` (grep before re-deriving).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit (baseline): **`7aee143b`** `CORPUS-COVERAGE.2 — continuity checkpoint: pin re-ingest #17 as the next PNT leaf`. **14 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2` re-ingest #17 slice is the next commit (→ 15 ahead). Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`).
- **✅ `CORPUS-COVERAGE.2` re-ingest #17 — Avalon Interface Spec (`683091`, 63pp) DONE, committed.** Fresh-session PNT slice. Rebuilt release first (binary was stale — `isf_ir.rs` newer; `CARGO_BUILD_JOBS=2`, 1m30s) so the cascade ran HEAD. Docling CPU re-ingest (63pp / 192 visual / 0 residuals; RAM steady 77–78% free, `.4a` guard armed, Ollama idle) → deterministic `evidence`→`semantic`→`intent`→`adapt --target isf`. Refresh: relations **126→111** / actors **64→50** (current `.1a`/`.1b`/`.1b.iv` consolidation gates fold fragment/phantom actors the stale Jun-7 evidence carried — cleaner KG), registers 8 / transactions 5 held, **honest absence of message-field/presence surfaces** (Avalon has no such tables). `.isf` renderable (`source.isf`, 26 signals) + real `fsmgen --strict --check` **success / 0 diagnostics**; `validate` no stage-staleness. 15/17 re-ingested docs strict-clean. Prior `ISF-VALUE-WIDTH-EMIT` CLOSED (`.2`) is in git history.
- next_action (PNT): **`CORPUS-COVERAGE.2` re-ingest #18** — the next protocol spec from the long tail. Recommended pick = **Wishbone B4** (`wbspec_b4_wishbone_b4_specification`, 2MB — a complete bus protocol, AHB/APB-class, high-value) or an **OpenCAPI Transaction-Layer** doc; then JEDEC/USB/RISC-V system-IP/VT-d/Cortex-A76/GIC-400/overview/guides. Method (unchanged): rebuild release if any `.rs` is newer than the binary, `DOCLING_DEVICE=cpu`, `.4a` RAM guard active (clean abort ≥85% used), Ollama idle, monitor RAM+swap (`[[feedback_ram_ceiling_monitor]]`); cascade `evidence → semantic → intent → adapt --target isf` + `validate`, record before/after deltas in the `.2` log, commit per `COMMIT.md`. 41 normalized-missing docs remain (40 after #18; `readme` excluded as noise). Other active trees have no freely-pickable light leaf → re-ingest IS the substantive frontier.
- in_flight_uncommitted: **none after the #17 commit** (the `.2` log + all live docs are in that commit; `generated/` is git-ignored — durable trace is the `.2` table). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready.
- **OPERATIONAL REALITY:** host RAM healthy at ~77–78% free (`memory_pressure`), swap 1.0G/2.0G. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama server up but **no model loaded** (idle — serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest = re-run of existing deterministic extractors → WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** orthogonal by construction (the 4 gold docs are not re-ingested). `scripts/run_ci.sh` green before committing CODE (not needed for a re-ingest data slice). Push ~every 200 (**15 ahead after this commit → HOLD**).
