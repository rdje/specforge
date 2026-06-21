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
- latest_commit (baseline): **`5858140a`** `CORPUS-COVERAGE.2 — re-ingest #18: Wishbone B4 (wbspec_b4, 128pp) honest signal-recall-gap finding`. **16 ahead** of `origin/main` (push at ~200 → HOLD). This `CORPUS-COVERAGE.2` re-ingest #19 slice is the next commit (→ 17 ahead). Untracked (stays): `.claude/settings.json`.
- **MODE: PNT loop** (fresh session `2026-06-22`).
- **✅ `CORPUS-COVERAGE.2` re-ingest #17/#18/#19 DONE, committed.** #17 Avalon (clean, FSMGen 0 diags); #18 Wishbone B4 + #19 OpenCAPI-4.0-TL Arch = honest **doc-style table-recognition recall gaps** (Wishbone 1 `signal_description`/25 tables, OpenCAPI **0/246** — 219 `unknown`; relations→0, thin 1-signal `.isf`, FSMGen strict-clean; IntentIR still carries the free-text constraints/transactions/actors). **REGRESSION RULED OUT** (current binary still yields DTI 159 msg-fields / MMU-700 63 registers / AHB 66 relations) → these are the upstream **Lever D** (non-AMBA table/signal recognition, `PDF-VARIANT-DIGESTION` family, kin to parked `.9.10`), NOT emitter bugs, NOT fixed in-slice. 17/19 re-ingested docs strict-clean (2 FAIL = DTI Lever A width-align, LPI Lever C rule-conflict).
- next_action (PNT): **`CORPUS-COVERAGE.2` re-ingest #20 — PIVOT to the register/TRM/ISA docs** (the high-value AMBA-style protocol specs are exhausted; #17–#19 proved the remaining non-AMBA protocol docs are thin until Lever D is built). The `.10c`/`.10g` register families demonstrably fire on TRM/arch docs (CoreSight TMC 2→30, GIC-600 15→33, MMU-700 13→63, SMMU-arch 1→89). Recommended #20 = **GIC-400 TRM** (`ddi0471_a_2011_06_23_gic_400_technical_reference_manual`, 1MB — ARM TRM, `.10c` `bits|name|description` upside, small/fast) or **RISC-V IOMMU arch** (`1_0_1_..._risc_v_iommu_architecture_specification`, 2MB) or a **CoreSight SoC-600 TRM** (`100806_*`). Method (unchanged): rebuild release if any `.rs` newer than binary, `DOCLING_DEVICE=cpu`, `.4a` RAM guard, Ollama idle, monitor RAM; cascade + `validate`, record deltas in the `.2` log, commit per `COMMIT.md`. **38 normalized-missing docs remain** (`readme` + the OpenCAPI PHY/mechanical/TL-variants tail are expected-thin, lower priority). Re-ingest IS the substantive frontier.
- in_flight_uncommitted: **none after the #19 commit** (the `.2` log + all live docs are in that commit; `generated/` is git-ignored — durable trace is the `.2` table). Untracked (stays): `.claude/settings.json`. blockers: none. Repo handoff-ready.
- **OPERATIONAL REALITY:** host RAM healthy at ~72–78% free (`memory_pressure`), swap 1.0G/2.0G. Builds run `CARGO_BUILD_JOBS≤2`, RAM monitored, kill ≥85% used (`[[feedback_ram_ceiling_monitor]]`). ollama server up but **no model loaded** (idle — serialize vs Docling).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication. Re-ingest = re-run of existing deterministic extractors → WIRE-BASED-100 + register/wire golds + `kg-bench` **156/156** orthogonal by construction (the 4 gold docs are not re-ingested). `scripts/run_ci.sh` green before committing CODE (not needed for a re-ingest data slice). Push ~every 200 (**17 ahead after this commit → HOLD**).
