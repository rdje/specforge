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
- latest_commit: **`07e995a1` (`WIRE-BASED-100.3a`), 34 ahead of `origin/main`, NOT pushed** (owner hold; push ~200 or on ask). CI green = **1301 tests** (full `scripts/run_ci.sh` green). KM 21 facts. `subs/fsmgen` pinned. Local models: `qwen2.5:14b-instruct` (text/NLI) + `qwen2.5vl:7b` (vision); `ollama serve` may be needed.
- active_work: **`WIRE-BASED-100`** (owner NON-NEGOTIABLE bar: 100% on every WIRE-BASED spec APB/AHB/AXI/SWD — literal bus wires, unlike packet CHI). **APB gold-100% ACHIEVED** (`eval-extraction seed_apb.json --provider skip` → the `source-tolerant + filtered` block = constraints + relations P=R=F1=1.000, per-fact, no faking; `.1`/`.1b`/`.6`/`.6c`/`.7`/`.7c`). Harness thesis: LLM judges, Rust grounds, the document decides — `ir/entity_typing` + `ir/condition_extract` + `ir/constraint_extract_llm` (CHI constraint precision 17%→64%) + `ir/extraction_filters`. Conformal/GriTS UNBLOCKED (NLI-oracle; cross-tool consensus + agent-adjudication). CHI = packet/flit: `DBID`/`TxnID` are FIELDS not signals.
- **`.3a` JUST DONE (commit pending):** re-diagnosed `.3` — the APB signal CATALOG is **already 35/35 extracted** (incl. all 14 `*CHK`, from `table_0004`/`0005`/`0014`); `table_0016/0017/0018` are docling-mangled DUPLICATE views (Signal col rotated to last) → flagging them was a completeness-gauge OVER-COUNT, not a miss. Fixed `completeness::unexplained_intent_bearing_tables` (+`signal_table_covered_by_inventory`, content-based densest-distinct signal column, rotation-proof, strict) → APB `validate` candidate_misses **5 → 3** (0016/0017 covered; `0018` honestly stays flagged — docling trapped its signals in header rows; 2 prose residuals remain). No faking. KM card `apb-signal-catalog-fully-extracted`.
- next_action: **`WIRE-BASED-100.2`** — value-constraint recall (scoped + owned). APB's remaining gauge misses are 2 prose residuals: `statement_0223` "the Requester must drive all bits of PSTRB LOW" is a REAL gap — `is_signal_value_constraint` only matches passive `SIGNAL must be VALUE`, not active `<actor> must drive <signal> <LOGIC_LEVEL>` → no PSTRB constraint. Fix in `ir/evidence.rs` (logic-level vocab already in `normative_vocab`, ADR-0006-safe). `statement_0370` (EDC end-to-end) = honest non-wire residual, LEAVE as-is (no fabrication). SENSITIVE: capturing PSTRB-LOW adds a fact not in `seed_apb.json` gold → MUST complete+annotate the gold and re-run `eval-extraction --provider skip` to confirm APB gold-100% holds. Then `.4` temporal, `.5` AHB→AXI→SWD. Future: rotation-proof column detection in the EXTRACTOR (`.5`); `table_0018` header-row recovery / docling re-ingest. ROOT open: tier-overlap degeneracy (Pattern×Nlp DISJOINT) blocks conformal + capture-recapture. Owner (NON-NEGOTIABLE): every score objectively measured + demonstrated PER-FACT — no fake/gamed scoring (`feedback_scoring_rigor`).
- paused_work: none. in_flight_uncommitted: none. blockers: none.
