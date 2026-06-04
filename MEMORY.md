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
- latest_commit: `99c97779` — "COMMIT-DOCTRINE — raise push threshold 30 -> 200" (this `PRIOR-DECAY.2` close commit pending → ~31 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200** (raised from 30, user directive 2026-06-04; `COMMIT.md` authoritative) — no push due until ~200 ahead or explicit `push`. CI green baseline = **1233 tests** (was 1229; +4 PRIOR-DECAY). KM live (11 facts after this commit).
- active_work_unit: none — `PRIOR-DECAY` **CLOSED `2026-06-04`** (read-only `CorpusMemory::contested_priors()` detects same-key/conflicting-value priors within a protocol family over ActorTaxonomy/SemanticPhrase/TableShape; advisory hint = strongest-supported value; surfaced in `learn-priors`; additive — no harvest/merge/consultation change; 4 tests; book + KM card `contested-priors`; CI green 1233). Consultation down-weight + time-staleness deferred.
- next_action: PNT — **`DEMPSTER-FUSION-COMBINER`** (user's 2nd pick). Investigate `ir/fusion.rs` (where modality/evidence confidences combine — currently `min()`/some rule). Design a Dempster-Shafer belief-mass combiner with explicit **conflict mass K**, guarding the high-conflict **Zadeh pathology** (K→1 normalize-by-near-zero → counterintuitive certainty → fall back / surface conflict as residual rather than fabricate agreement). **Mind fixture churn** (changes confidence values → may shift `converge`/eval snapshots); if churn is large, scope to an ADDITIVE `dempster_combine()` helper + tests first, don't flip the live combiner until verified. Own + design (.1) before code. Reminder: card durable facts; grep `KNOWLEDGE_MAP.md` first; **no push until ~200 ahead or explicit ask.**
- paused_work: `TEMPORAL-RULE-SVA-RENDER` (deferred, awaiting user decision vs FSMGen-native LTL/MTL).
- in_flight_uncommitted: none.
- blockers: none for Dempster (but mind fixture churn if flipping the live combiner).
