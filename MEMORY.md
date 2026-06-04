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
- latest_commit: `6628d1d4` — "FSMGEN-LTL-MTL-SUGGESTION — suggest LTL/MTL-in-ISF …; close" (this `isf-temporal-lowering-no-silent-drop` KM-card commit pending → becomes 28 ahead of pushed `d92a73e3`; **PUSH due ~now (≈28, threshold ~30) — recommend pushing to checkpoint the marathon run**). CI green baseline = 1229 tests. KM live (10 facts after this commit).
- active_work_unit: none. **Clean/bounded/ungated backlog effectively EXHAUSTED this run** (the user's "roll to exhaustion" reached for the clean tier). Investigated the next candidate (`.isf` temporal lowering-completeness verifier) → **NO-BUILD**: already guaranteed (ISF-TEMPORAL-LOWERING.2.2/.2.3 — each temporal_rule → contract|rule|residual, tested by `isf_temporal_rules_reach_isf_end_to_end`); recorded as KM card `isf-temporal-lowering-no-silent-drop`. Prior next candidate `TEMPORAL-RULE-SVA-RENDER` is user-DEFERRED.
- next_action: PNT — remaining backlog is the **big / gated / core-subsystem tier**, needs a user steer or unblocking: **big** (prior-decay for CorpusMemory [Parisi; touches the learning plane]; NLI entailment verifier [needs a model]; Dempster fusion combiner [fixture risk]; TEDS/GriTS table metric [needs table gold]; the deferred SVA export); **gated** (conformal calibration [needs a held-out calibration set]; κ/α agreement [needs a 2nd gold annotator]; SVA-vs-FSMGen-native [awaits user decision + FSMGen response]); **already-covered** (lowering-completeness). No clean bounded ungated win remains. Recommend: PUSH now (~28 ahead) + user picks the next big investment. Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` first.
- paused_work: `TEMPORAL-RULE-SVA-RENDER` (deferred, awaiting user decision vs FSMGen-native LTL/MTL).
- in_flight_uncommitted: none.
- blockers: the remaining trees are gated (data / annotator / user-decision) or big core-subsystem changes.
