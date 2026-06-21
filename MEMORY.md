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
- latest_commit (baseline): **`fa964832`** `CORPUS-COVERAGE.2 — re-ingest #19: OpenCAPI 4.0 TL Arch (240pp) honest table-recognition-gap finding`. **17 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2` re-ingest #20 slice is the next commit (→ 18 ahead). Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`).
- **✅ `CORPUS-COVERAGE.2` re-ingest #17–#20 DONE, committed.** #17 Avalon (clean); #18 Wishbone + #19 OpenCAPI-TL = honest **non-AMBA table-recognition recall gaps** (thin 1-signal `.isf`, **REGRESSION RULED OUT** — DTI 159 / MMU-700 63 / AHB 66 hold → upstream **Lever D**, not emitter bugs); #20 GIC-400 TRM = **healthy register/TRM-phase refresh** (5 `signal_description` tables → 16-signal AXI-slave-interface `.isf` + 5 txns + 4 storage(reset) + 6 relations, strict-clean; registers 3→4 modest). 18/20 re-ingested docs strict-clean (2 FAIL = DTI Lever A width-align, LPI Lever C rule-conflict).
- next_action (PNT): **`CORPUS-COVERAGE.2` re-ingest #21 — continue the register/TRM/ISA phase.** Recommended #21 = **RISC-V IOMMU arch** (`1_0_1_2026_02_22_risc_v_iommu_architecture_specification`, 2MB — register-dense arch, may fire `.10c`/`.10g`) or a **CoreSight SoC-600 TRM** (`100806_0701_17_..._coresight_soc_600_technical_reference_manual`, 6MB — the `.10c` family lifted CoreSight TMC 2→30) or **JEDEC eMMC** (`jesd84_b50_2013_09_emmc_5_0`, 6MB — EXT_CSD register-heavy). Method (unchanged): rebuild release if any `.rs` newer than binary, `DOCLING_DEVICE=cpu`, `.4a` RAM guard, Ollama idle, monitor RAM; cascade + `validate`, record deltas in the `.2` log, commit per `COMMIT.md`. **37 normalized-missing docs remain** (`readme` + the OpenCAPI PHY/mechanical/TL-variants + guides tail are expected-thin, lower priority). Re-ingest IS the substantive frontier.
- in_flight_uncommitted: **none after the #20 commit** (the `.2` log + all live docs are in that commit; `generated/` is git-ignored — durable trace is the `.2` table). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready.
- **OPERATIONAL REALITY:** host RAM healthy at ~72–78% free (`memory_pressure`), swap 1.0G/2.0G. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama server up but **no model loaded** (idle — serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest = re-run of existing deterministic extractors → WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** orthogonal by construction (the 4 gold docs are not re-ingested). `scripts/run_ci.sh` green before committing CODE (not needed for a re-ingest data slice). Push ~every 200 (**18 ahead after this commit → HOLD**).
