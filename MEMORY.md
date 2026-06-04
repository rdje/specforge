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
- latest_commit: `fe3b19e9` — "AMBIGUITY-PHRASE-DETECTOR.1 — own + design …" (this `AMBIGUITY-PHRASE-DETECTOR.2` close commit pending → becomes 22 ahead of pushed `d92a73e3`; **PUSH due-ish, threshold ~30**). NOTE: CI green baseline = 1229 tests. KM live (`KNOWLEDGE_MAP.md`, 5 facts/25 keys) — grep before re-deriving; card durable facts.
- active_work_unit: none — `AMBIGUITY-PHRASE-DETECTOR` **CLOSED `2026-06-04`** (flag-only weak-phrase detector → `validate` finding+metric; `ir/ambiguity.rs`; 6 tests; book + KM card; extraction-neutral; CI green 1229). SEVEN trees CLOSED this run.
- next_action: PNT — **NEW user directive (2026-06-04, two messages):** SpecForge is *specification mining* (adopt the literature's term) and benefits a lot from Pnueli/GoldMine/Texada (and the other literature-sweep authors). Create a tree to (a) adopt the **"specification mining"** framing across README/book/live-docs, and (b) build a per-author **adopt-vs-defer provenance ledger** — for each leveraged author/work: what abstraction SpecForge TAKES (+ where it's instantiated in code/IR), what it LEAVES OUT for now + WHY — documented in **KM cards + book + task-tree**. Prioritize the temporal trio (Pnueli/GoldMine/Texada — the LTL `G(ante→cons)` template SpecForge's `temporal_rules` + `ir/temporal_ltl.rs` instantiate), then the others (Ammons spec-mining/Daikon; Docling/TableFormer; OpenIE/ReVerb/Mintz; LayoutLM/Dempster; GCD/RAG/NLI; Chao; van Rijsbergen/MUC; Chow/conformal; PSL/SVA). Then other backlog: `.isf`→PSL/SVA export (FSMGen contract first); conformal; NLI/Dempster. **PUSH due-ish (~22 ahead).**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
