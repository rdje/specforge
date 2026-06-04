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
- latest_commit: `ee638480` — "TEMPORAL-RULE-LTL-RENDER.1 — own + design the standard LTL/MTL rendering …" (this `TEMPORAL-RULE-LTL-RENDER.2` close commit pending → becomes 20 ahead of pushed `d92a73e3`; **PUSH due-ish, threshold ~30**). NOTE: CI green baseline = 1223 tests. KM is live (`KNOWLEDGE_MAP.md`, now 4 facts/20 keys) — grep it before re-deriving; write a card for durable facts.
- active_work_unit: none — `TEMPORAL-RULE-LTL-RENDER` **CLOSED `2026-06-04`** (mined `temporal_rules` render to standard LTL/MTL via pure `ir/temporal_ltl.rs::temporal_rule_to_ltl`; 4 tests; book + KM card; derived/no-IR-field → zero fixture churn, zero behavior change; CI green 1223). SIX trees CLOSED this run: LITERATURE-GROUNDING, TEMPORAL-RULE-EVAL, TEMPORAL-ANTECEDENT-RECALL, RECALL-CHAO-ESTIMATOR, KNOWLEDGE-MAP-ADOPTION, TEMPORAL-RULE-LTL-RENDER.
- next_action: PNT — pick the next tree. The explicit downstream consumer of the LTL renderer is **`.isf`→PSL/SVA assertion export** — a NEW tree, but it touches the `.isf` adapter + the **FSMGen handoff contract** (the authority; read `feedback_fsmgen_contract` / the handoff doc FIRST — parser acceptance ≠ support). Other LITERATURE-GROUNDING backlog: conformal LLM-tier calibration (needs more labeled data); NLI-based entailment verifier; Dempster-rule fusion combiner. Gated: column-less residual (~112; needs a precision/recall decision), CHI Class-A, APB/CHI re-ingest (Docling works w/ DOCLING_DEVICE=cpu). **PUSH due-ish (20 ahead; threshold ~30).** Reminder: write a `docs/knowledge/<id>.md` card whenever a durable fact is established or archaeology is caught.
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
