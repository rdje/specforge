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
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: **`66c67b3a`** (about to commit `.3` batch A on top); **133 ahead** of `origin/main` (push at ~200). Session `2026-06-15` = **dedicated `CANONICAL-PROMOTION-SWEEP` sweep**; owner authorized **"run both, .3 then .2"** (the recorded post-`.1` gate question — owner chose to run the whole sweep, non-wire scale-out first, then wire docs).
- **THIS-SESSION:** executing the full `CANONICAL-PROMOTION-SWEEP` (land the default LLM-primary constraint promotion on CANONICAL artifacts, RAM-safe + review-gated). Scope inventoried: 78 evidence bundles → 31 with constraints → **26 non-wire docs in `.3`** (excl. 4 already-promoted: AXI/APB5/AHB/HBM2 + wire-gold SWD `ihi0074_a`→`.2`). **`.3` batch A (13 smallest) DONE:** 12 promoted on canonical + KEPT (aggregate **89.8%→29.7% not-entailed**, entailed 5→26), 1 REVERTED (`soc600_0701`, lost its one entailed constraint → restored to consistent Pattern). `kg-bench` 156/156; RAM **min 43% free** (watchdog). Canonical mutation in git-ignored `generated/`; `*.prepromote.bak` retained; `promotion_status=not_promoted_review_required`.
- **RAM-SAFETY (owner, non-negotiable): 90→93% used RAM = reboot DANGER ZONE; monitor every background job + swap, autonomously kill at ≥85% used; serialize Docling vs the 14B model (`ollama stop` before ingests); QUALITY INVARIANT under restriction — only SPEED flexes** (`feedback_ram_ceiling_monitor` / `project_big_pdf_memory_bounded_ingest`). Push ~every 200 commits or on ask.
- **DOCTRINE (non-negotiable): RUNTIME code is PDF-AGNOSTIC (ADR-0006)** — concrete chip names ONLY in gold keys + `#[cfg(test)]`, never runtime; NO deny/filter LISTS; goal = #1 structured+multimodal+bounded-LLM intent mining w/ honest residuals; honest RESIDUAL over fabrication. **`scripts/run_ci.sh` before declaring any CODE slice green** (per-slice focused checks skip rustdoc/fmt-check/KM derive-and-diff); `cargo test` does NOT rebuild the bin — `cargo build --release` before live measurement. `subs/fsmgen` pinned `c0b7eaa7`.
- **LOCKED SWEEP PROTOCOL (per doc, no re-ingest):** `ollama stop` → ≥40% RAM free → backup `*.prepromote.bak` → BEFORE `nli-verify` → `extract-constraints-llm` (promote in place) → `semantic`/`intent`/`adapt` rebuild → AFTER `nli-verify` → `kg-bench` → 3s RAM watchdog (≤15%-free kill) → `ollama stop`. **KEEP/REVERT:** revert iff `entailed_after<entailed_before` OR not-entailed fraction worsened; else keep. **WRITE-PATH GOTCHA:** commands persist by the artifact's `artifact_layout` (canonical path), NOT the input path → NEVER run a command on a `*.bak` (clobbers canonical); KM card `canonical-promotion-output-path-artifact-layout`. Driver = `/tmp/sweep.sh` (untracked watchdog tooling; defaults `ollama`+`qwen2.5:14b-instruct`).
- next_action: **launch `.3` batch B** (9 medium non-wire docs, 11–20 cons: risc_v_iommu, trace_bus, i2c, apb-orig `ihi0024_d`, axi-stream, ccix_r1_0a, opencapi_4_0, nvme, gfb) under the locked protocol → then **batch C** (4 big: LPI 30 / LTI 41 / AXI+ACE `ihi0022_h_c` 97 / DTI 114) → then **`.2` wire docs** (APB5/AHB/AXI already promoted → re-verify the gold battery via `eval-extraction seed_apb/ahb/axi(_temporal)` + `seed_swd_derivation`; SWD `ihi0074_a` promote+verify, revert-on-regress). Commit per batch.
- in_flight_uncommitted: batch-A record (task-tree + KM card + `CHANGES.md` + this `MEMORY.md`) staged for THIS commit; canonical promotions persisted in git-ignored `generated/`. Untracked (stays): `.claude/settings.json`. blockers: **none**.
