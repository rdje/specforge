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
- latest_commit: **`286b2275` (`WIRE-BASED-100.3b`), 36 ahead of `origin/main`, NOT pushed** (owner hold; push ~200 or on ask). CI green = **1304 tests** (full `scripts/run_ci.sh` green). KM 21 facts. `subs/fsmgen` pinned. Local models: `qwen2.5:14b-instruct` (text/NLI) + `qwen2.5vl:7b` (vision); `ollama serve` may be needed.
- active_work: **`WIRE-BASED-100`** (owner NON-NEGOTIABLE bar: 100% on every WIRE-BASED spec APB/AHB/AXI/SWD — literal bus wires, unlike packet CHI). **APB gold-100% ACHIEVED** (`eval-extraction seed_apb.json --provider skip` → the `source-tolerant + filtered` block = constraints + relations P=R=F1=1.000, per-fact, no faking; `.1`/`.1b`/`.6`/`.6c`/`.7`/`.7c`). Harness thesis: LLM judges, Rust grounds, the document decides — `ir/entity_typing` + `ir/condition_extract` + `ir/constraint_extract_llm` (CHI constraint precision 17%→64%) + `ir/extraction_filters`. Conformal/GriTS UNBLOCKED (NLI-oracle; cross-tool consensus + agent-adjudication). CHI = packet/flit: `DBID`/`TxnID` are FIELDS not signals.
- **`.3a`+`.3b` DONE (`.3b` commit pending):** completeness-gauge accuracy. `.3a` (`07e995a1`): APB CATALOG is **already 35/35** (incl. all 14 `*CHK`, from `table_0004`/`0005`/`0014`); `table_0016/0017/0018` are docling-mangled DUPLICATE views → over-count, fixed via `signal_table_covered_by_inventory` (content-based densest-distinct col, rotation-proof, strict) → misses 5→3. `.3b`: a `normative_statement` whose obligation a typed record already cites is NOT a residual (`uncaptured_normative_statement_ids`) → misses 3→**2** (both genuine: `table_0018` docling-garbage + `statement_0370` honest non-wire EDC). **`.2` was a FALSE alarm** (field-name bug `signal_name` vs `subject_signal`): `statement_0223` PSTRB-LOW IS captured (`dyn_sigcon_0015`, dynamic path); gold-100% already includes it. KM card `apb-signal-catalog-fully-extracted`.
- next_action: **`WIRE-BASED-100.4`** — temporal-rule completeness (MEASURED + scoped, ground truth): `eval-extraction seed_apb_temporal.json --provider skip` = `temporal_rule P=R=F1=0.333` (tp=1 fp=2 fn=2; gold=3). tp = PSTRB-LOW (0221). The 2 fn+fp = antecedent capture: PNSE (0285) drops the `when PSEL is asserted` antecedent (→ empty); PBUSER (0339) drops the **leading** PSEL from `when PSEL, PENABLE, and PREADY are asserted` (→ only PENABLE+PREADY). Fix in `ir/semantic.rs` antecedent capture; **CARE: many kg-bench temporal fixtures — keep kg-bench green**; do as its own measure→fix→remeasure cycle. APB otherwise: gold-100% (constraints+relations), catalog-100%, completeness gauge ACCURATE (2 genuine candidates). Then `.5` AHB→AXI→SWD. Genuine out-of-code residuals: `table_0018` (docling garbled), `statement_0370` (non-wire req — leave). ROOT open: tier-overlap degeneracy (Pattern×Nlp DISJOINT). Owner (NON-NEGOTIABLE): every score objectively measured + demonstrated PER-FACT — no fake/gamed scoring (`feedback_scoring_rigor`).
- paused_work: none. in_flight_uncommitted: none. blockers: none.
