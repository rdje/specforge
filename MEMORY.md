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
- latest_commit: `c26a9453` — "SPEC-MINING-PROVENANCE.1 — own + design …" (this `SPEC-MINING-PROVENANCE.2` commit pending → becomes 24 ahead of pushed `d92a73e3`; **PUSH due — at/near the ~30 threshold soon; consider pushing on the next user turn or at .3 close**). NOTE: CI green baseline = 1229 tests. KM live (`KNOWLEDGE_MAP.md`, 6 facts/30 keys).
- active_work_unit: `SPEC-MINING-PROVENANCE` — `.1` design + `.2` framing & temporal-trio DONE. `.2` = `docs/research/grounding/adopt-defer-ledger.md` (forward-spec-mining framing + Pnueli/GoldMine/Texada Take/Leave-out+why/Instantiated-at) + "forward specification mining" adopted in README + architecture-rationale + KM card `spec-mining-framing` (KM 6/30); CI green 1229. EIGHT trees CLOSED this run.
- next_action: PNT — `SPEC-MINING-PROVENANCE.3`: fold the REMAINING swept authors into `adopt-defer-ledger.md`, each Take/Leave-out+why/Instantiated-at, reusing verified citations from the per-aspect grounding docs (Ammons-method/Daikon → spec-mining; Docling/TableFormer/DocLayNet/PubTables-1M → document-extraction.md; OpenIE/ReVerb/Mintz/Hogan → knowledge-graph-relation-extraction.md; LayoutLM/Donut/Dempster → multimodal-fusion.md; GCD/Outlines/RAG/SNLI/SelfCheckGPT/Garcez-Lamb → neuro-symbolic-bounded-llm.md; Yarowsky/Riloff-Jones/NELL/Snorkel → cross-document-learning.md; van Rijsbergen/MUC/Cohen/Chao → extraction-evaluation.md; Chow/El-Yaniv/Vovk/Guo/Scheirer → uncertainty-residual-honesty.md; PSL/SVA/AssertLLM/HLS → spec-to-hardware.md). Then a short synthesis + CLOSE. Optionally add per-author KM cards for the highest-traffic ones. Then other backlog: `.isf`→PSL/SVA export (FSMGen contract first); conformal; NLI/Dempster. **PUSH due (~24 ahead).**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
