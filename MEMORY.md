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
- latest_commit (baseline): **`ac2a8e29`** `CORPUS-COVERAGE.2 — re-ingest #17: Avalon Interface Spec (683091, 63pp) current-binary refresh`. **15 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2` re-ingest #18 slice is the next commit (→ 16 ahead). Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`).
- **✅ `CORPUS-COVERAGE.2` re-ingest #18 — Wishbone B4 (`wbspec_b4`, 128pp) DONE, committed.** Docling CPU re-ingest (128pp / 356 visual / 0 residuals; RAM steady 72–78% free, `.4a` guard armed, Ollama idle) → deterministic cascade (evidence 2041 statements). **Honest extraction-gap finding (not a regression):** fresh evidence has **0 interfaces / 0 signal_records → 0 relations** (the lone stale relation dropped by `.1a`; stale was also ~1); IntentIR still carries 215 free-text constraints + 2 transactions + 8 actors. Root cause (probe): Wishbone's `SIGNAL_O()`/`SIGNAL_I()` suffix-notation + prose signal-list style isn't caught by the current signal-table/prose extractors → surfaced **Lever D** (upstream signal-recall, `PDF-VARIANT-DIGESTION` family, kin to parked `.9.10`), NOT an emitter lever, NOT fixed in-slice. `.isf` renders THIN (`arbiter.isf`, 1 signal) + `fsmgen --strict` success/0 diagnostics; `validate` no stage-staleness (0-vs-0). 16/18 re-ingested docs strict-clean. (#17 Avalon + `ISF-VALUE-WIDTH-EMIT` CLOSE are in git history.)
- next_action (PNT): **`CORPUS-COVERAGE.2` re-ingest #19** — next protocol spec from the long tail. Recommended = an **OpenCAPI Transaction-Layer** doc (`opencapi_3_0_transaction_layer_28jan2020` / `opencapi_4_0_transactionlayer_arch_16jun2020` — protocol-rich; the OpenCAPI PHY/mechanical/certified-test docs are low-yield, take them later or skip), then JEDEC (HBM/HBM2/eMMC)/USB/RISC-V system-IP/VT-d/Cortex-A76/GIC-400/overview/guides. Method (unchanged): rebuild release if any `.rs` is newer than the binary, `DOCLING_DEVICE=cpu`, `.4a` RAM guard active (clean abort ≥85% used), Ollama idle, monitor RAM+swap (`[[feedback_ram_ceiling_monitor]]`); cascade `evidence → semantic → intent → adapt --target isf` + `validate`, record before/after deltas in the `.2` log, commit per `COMMIT.md`. **39 normalized-missing docs remain** (`readme` excluded as noise). Other active trees have no freely-pickable light leaf → re-ingest IS the substantive frontier.
- in_flight_uncommitted: **none after the #18 commit** (the `.2` log + all live docs are in that commit; `generated/` is git-ignored — durable trace is the `.2` table). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready.
- **OPERATIONAL REALITY:** host RAM healthy at ~72–78% free (`memory_pressure`), swap 1.0G/2.0G. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama server up but **no model loaded** (idle — serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest = re-run of existing deterministic extractors → WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** orthogonal by construction (the 4 gold docs are not re-ingested). `scripts/run_ci.sh` green before committing CODE (not needed for a re-ingest data slice). Push ~every 200 (**16 ahead after this commit → HOLD**).
