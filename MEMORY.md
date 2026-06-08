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
- latest_commit: **`a5bc5e7a` ("PDF-VARIANT-DIGESTION.4a.1 — register-field per-fact eval surface"); ~4 local commits ahead of `origin/main` after `.4a.2`** (`625a02ea` reconcile, `2a07e87e` split, `a5bc5e7a` .4a.1, then `.4a.2`). Push ~every 200 commits or on ask (`feedback_push_cadence`). CI green = **1366 tests** (`cargo test -p specforge --lib`); kg-bench 151/151. KM 38 facts — **systematically write a KM card per durable fact** (`feedback_systematic_km_cards`). `subs/fsmgen` pinned (`c0b7eaa7`/`d31b0b91`). Local models: `qwen2.5:14b-instruct` (text/NLI) + `qwen2.5vl:7b` (vision); `ollama serve` may be needed. Disk reclaimed ~31.8 GiB `2026-06-07`. **DOCTRINE: SpecForge RUNTIME code is PDF-AGNOSTIC (ADR-0006) — concrete chip names live ONLY in gold answer-keys (`test_data/llm_eval/seed_*.json`) + `#[cfg(test)]` fixtures, NEVER in runtime logic (`feedback_no_hardcoded_chip_spec_names`).**
- corpus: **8 git-tracked source PDFs in `corpus/`** (5 wire-based APB/AHB/AXI/AXI-Stream/SWD + NVMe + I2C(nxp) + RISC-V Debug) + `corpus/SOURCE_PDF_REGISTRY.md` (registry lists the 5 wire-based; NVMe/I2C/RISC-V rows pending). Owner's larger library is host-local — provided on request, **do NOT log its path** (`feedback_source_pdfs_in_repo`). Re-ingest needs `DOCLING_DEVICE=cpu` (`project_docling_mps_cpu`).
- prior_work (DONE, all pushed): **WIRE-BASED-100** = APB/AHB/AXI 100% on all 3 aspects (constraints/relations/temporal, per-fact source-tolerant, no faking) + **SWD-SERIAL-EXTRACTION** 100% (frame/operations/FSM). Detail lives in `docs/tasks/WIRE-BASED-100.md`/`SWD-SERIAL-EXTRACTION.md` + git, not here. GOTCHA: the eval scores PERSISTED evidence — re-ingest before measuring (`eval-scores-persisted-evidence`). `clean --scope source-normalized` breaks evidence rebuilds → re-ingest.
- active_work: **`PDF-VARIANT-DIGESTION`** (HIGH-PRIORITY owner program — digest ANY chip-spec PDF; `project_pdf_variant_digestion`). Lever A (table-kind classify: deterministic register-field `.2`/`.2c`/`.2d` + VLM classify/grid-repair `.2b`/`.2b'`) + Lever B (prose entity capture: signals `.3a` → I2C 0→10, actors `.3b`) both LANDED. Whole-corpus sweep: 75/82 ingest OK, aggregate 1,908 signals / 2,953 registers / 10,632 fields / 3,077 relations — BREADTH proven, but only 4/74 (APB/AHB/AXI/SWD) have VERIFIED precision. Frontier now = make breadth TRUSTWORTHY (`.4`–`.8`). Guardrails: genericity (`feedback_genericity_guardrail`), multi-strategy best-wins (`feedback_multi_strategy_best_wins`), ADR 0006 (no hardcoded chip names), scoring rigor (`feedback_scoring_rigor`). ISF lowering DEFERRED.
- next_action: **`PDF-VARIANT-DIGESTION.4a.3`** — NVMe register-field gold. `.4a.1` (eval surface) + `.4a.2` DONE (RISC-V Debug measure&surface: field-name recall 20/34=0.588, register-name 0/60 + bit-extent 0/179 gaps surfaced honestly; the strict 0.000 reflects synthetic names + no bit positions). `.4a.3`: NVMe DID capture `bit_width` (`.2c`: 44 regs/199 fields), so a source-verified gold from the `corpus/` NVMe PDF (FRESH re-ingest, `DOCLING_DEVICE=cpu`) should give a MEANINGFUL strict per-fact P/R/F1 (unlike RISC-V Debug). Then `.4a.4` declared-signal eval surface, `.4a.5` I2C prose-signal gold. Two RISC-V Debug fix leaves identified (register-name heading association; bit-layout-graphic parsing) — future tree, NOT measurement leaves.
- paused_work: none. in_flight_uncommitted: `.4a.2` code (`eval.rs`+`eval_extraction.rs`) + gold seed + book + live-docs staged for the `PDF-VARIANT-DIGESTION.4a.2` commit (this MEMORY refresh reflects pre-commit baseline `a5bc5e7a`). blockers: none.
