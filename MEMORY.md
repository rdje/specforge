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
- latest_commit: `10c09366` — "AMBIGUITY-PHRASE-DETECTOR.2 — flag vague spec prose in validate …; close tree" (this `SPEC-MINING-PROVENANCE.1` design commit pending → becomes 23 ahead of pushed `d92a73e3`; **PUSH due-ish, threshold ~30**). NOTE: CI green baseline = 1229 tests. KM live (`KNOWLEDGE_MAP.md`, 5 facts/25 keys).
- active_work_unit: `SPEC-MINING-PROVENANCE` — `.1` design DONE (user directive: name SpecForge "specification mining" [forward vs the literature's backward]; per-author adopt/defer ledger = Take / Leave-out+why / Instantiated-at, in KM + book + `docs/research/grounding/adopt-defer-ledger.md`). EIGHT trees CLOSED this run (LITERATURE-GROUNDING, TEMPORAL-RULE-EVAL, TEMPORAL-ANTECEDENT-RECALL, RECALL-CHAO-ESTIMATOR, KNOWLEDGE-MAP-ADOPTION, TEMPORAL-RULE-LTL-RENDER, AMBIGUITY-PHRASE-DETECTOR + the 6 grounding aspects).
- next_action: PNT — `SPEC-MINING-PROVENANCE.2`: create `docs/research/grounding/adopt-defer-ledger.md` (intro = the "specification mining" framing, forward vs backward, Ammons POPL'02 verified) + the **temporal trio** entries (Pnueli FOCS'77; GoldMine DATE'10; Texada ASE'15 — Take / Leave-out+why / Instantiated-at, reusing verified citations from `protocol-temporal-semantics.md`). Adopt "specification mining" in `README.md` (project objective) + `architecture-rationale.md`. Add KM card(s): a `spec-mining-framing` card (+ ledger pointer). Regenerate KM; full CI (mdBook + KM gate) green; commit. Then `.3` = remaining swept authors into the ledger + synthesis + close. **PUSH due-ish (~23 ahead).**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
