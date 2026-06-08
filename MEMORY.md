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
  ROADMAP↔code↔mdBook drift; push ~every 30 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: **`46dbead6` = `PDF-VARIANT-DIGESTION.5b` MEMORY-refresh (HEAD before this session's commits); substantive HEAD `61d59ee7` = `EXTRACTION-GAP-FIX.4d`; ~29 ahead of `origin/main`** (session `2026-06-08`). Push ~every 200 commits or on ask (`feedback_push_cadence`). Last verified gate: **FULL `scripts/run_ci.sh` GREEN** + kg-bench 151/151. **GOTCHA: per-slice focused checks (fmt+clippy+`cargo test`) do NOT run rustdoc — run `scripts/run_ci.sh` (or `cargo doc` warning-deny) before declaring a slice green / before handoff.** `subs/fsmgen` pinned (`c0b7eaa7`/`d31b0b91`). **DOCTRINE (non-negotiable): RUNTIME code is PDF-AGNOSTIC (ADR-0006) — concrete chip names ONLY in gold keys (`test_data/llm_eval/seed_*.json`) + `#[cfg(test)]`, NEVER in runtime (`feedback_no_hardcoded_chip_spec_names`); every decision preserves this.**
- this_session (`2026-06-08`): reclaimed **28.6 GiB** (`cargo clean`, ≥24h artifact directive) → 159G free; verified **mdBook + KM (45 cards) in sync** with the recent slices; then the owner unblocked the program (see active_work). `.venv-docling` present; Ollama reachable (`qwen2.5:14b-instruct` text/NLI + `qwen2.5vl:7b` vision; `ollama serve` may be needed). Re-ingest needs `DOCLING_DEVICE=cpu` (`project_docling_mps_cpu`).
- corpus: **12 git-tracked source PDFs in `corpus/`** — 8 prior (APB/AHB/AXI/AXI-Stream/SWD-ADI/RISC-V Debug/NVMe/I2C) + **4 NEW serial specs (`.9.1`, `2026-06-08`): CAN/SWP/SMBus/I2S** (`bosch/can`, `etsi/swp`, `smbus`, `nxp/i2s`) + `corpus/SOURCE_PDF_REGISTRY.md`. Owner's larger library is host-local — provided on request, **do NOT log its path** (`feedback_source_pdfs_in_repo`).
- prior_work (DONE, pushed): **WIRE-BASED-100** (APB/AHB/AXI 100% on constraints/relations/temporal) + **SWD-SERIAL-EXTRACTION** 100% (frame/operations/FSM). GOTCHA: eval scores PERSISTED evidence — re-ingest before measuring (`eval-scores-persisted-evidence`); `clean --scope source-normalized` breaks evidence rebuilds → re-ingest.
- active_work: **`PDF-VARIANT-DIGESTION.9`** (NEW serial-protocol class — owner-directed `2026-06-08`). **`.9.1` (import + git-track 4 serial PDFs + register) DONE.** Frontier → **`.9.2` = CAN 2.0 honest baseline** (`DOCLING_DEVICE=cpu cargo run -p specforge -- ingest corpus/bosch/can/current/Bosch_CAN_Specification_2.0_1991.pdf` → evidence → validate; record document_class + completeness gauge + serial-frame/FSM/signal yield; spin the extraction-lever leaf from the honest gap). Start CAN (richest serial frame + error-state FSM; exercises `SerialFrameField`+`ProtocolStateRecord`), then PNT through SWP/SMBus/I2S. `.6`/`.7` stay blocked on host-local zero-yield/USB-3.2 PDFs.
- **HONESTY GUARDRAIL (owner, non-negotiable): you CANNOT extract what is not there** — read the modality where the fact lives or report an honest RESIDUAL; NEVER fabricate a width/bit-range/field/name. APB/AHB/AXI/SWD stay 100%; genericity (`feedback_genericity_guardrail`), multi-strategy (`feedback_multi_strategy_best_wins`), scoring rigor (`feedback_scoring_rigor`). ISF lowering DEFERRED.
- next_action: **execute `.9.2`** — ingest CAN (cmd above), then `evidence`/`validate` for the honest baseline; record it in the PVD tree (+ KM card if a durable fact emerges); from the gap, own + implement the agnostic extraction lever. Book/live-docs/commit per slice (`COMMIT.md`).
- paused_work: none. in_flight_uncommitted: **none after the `.9.1` commit** (4 serial PDFs + registry + PVD `.9` nodes + CHANGES + LIVE_ACHIEVEMENT_STATUS + this MEMORY). Only untracked: `.claude/settings.json` (local harness config, stays untracked). blockers: none. RESUME: read `README.md`+`MEMORY_ARCHITECTURE.md` then this pointer → `.9.2` CAN baseline.
