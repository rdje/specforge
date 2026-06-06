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
- latest_commit: **`11a52cfb`, 32 ahead of `origin/main`, NOT pushed** (owner hold; push ~200 or on ask). CI green = **1298 tests**. KM 20 facts. `subs/fsmgen` pinned. Local models: `qwen2.5:14b-instruct` (text/NLI) + `qwen2.5vl:7b` (vision); `ollama serve` may be needed.
- active_work: **`WIRE-BASED-100`** (owner NON-NEGOTIABLE bar: 100% on every WIRE-BASED spec APB/AHB/AXI/SWD — design signals are literal bus wires, unlike packet CHI). **APB gold-100% ACHIEVED + demonstrated**: `specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider skip` → the `source-tolerant + filtered` block = constraints + relations P=R=F1=1.000, per-fact, no faking. Fixes: `.1` content-anchored scoring (re-ingest-proof), `.1b` source-tolerant recall, `.6`/`.6c` actor discrimination, `.7`/`.7c` normative gate. `.6c`/`.7c` = AUTOMATIC LLM detection (`ir/entity_typing` + NLI) behind the cheap heuristic fast-path.
- the session's thesis (the harness — LLM judges, Rust grounds, the document decides): `ir/entity_typing` + `ir/condition_extract` + `ir/constraint_extract_llm` (LLM-primary extractor; CHI constraint precision 17%→64%) + `ir/extraction_filters`. **Conformal UNBLOCKED** via the NLI-oracle (`TABLE-GRITS-CONFORMAL`); **GriTS UNBLOCKED** via cross-tool consensus + the agent-adjudication loop (`GRITS-CROSS-TOOL`, `scripts/grits_cross_tool.py`/`grits_adjudicate.py`, gitignored `.venv-eval` for pdfplumber). CHI DOMAIN CORRECTION: packet/flit protocol — `DBID`/`TxnID` are FIELDS not signals (B2.4); `EXTRACTION-QUALITY-GAUGE` (NLI-oracle per-doc quality gauge; CHI ~80% erroneous).
- next_action: **`WIRE-BASED-100.3a`** — signal-table → signal-record extractor (capture the APB signal CATALOG: `table_0016/0017/0018` ~35 signals produce ZERO records → APB is gold-100% but NOT spec-100%) → then `.5` roll to AHB→AXI→SWD. Owed: the `.6c/.7c` LLM-path end-to-end APB demo (`--provider ollama`). ROOT worth fixing: **tier-overlap degeneracy** (Pattern×Nlp extract DISJOINT facts, zero overlap) blocks BOTH conformal calibration AND capture-recapture completeness. Owner principle (NON-NEGOTIABLE): every score objectively measured + logically explained + demonstrated PER-FACT — no fake/gamed scoring (`feedback_scoring_rigor`).
- paused_work: none. in_flight_uncommitted: none (only 2 untracked `.claude/` local-config files). blockers: none.
