# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey. History lives in
> `git log`; work state lives in the task-trees (`docs/tasks/`); durable facts/decisions live in
> `docs/decisions/`. Do **not** append session narration — overwrite the "Current state" block.

## How to resume (any AI, any harness)
- Derive the current revision on read with `git rev-parse HEAD`; never store a latest-commit shadow.
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (doctrines are mechanically
  gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`); follow `COMMIT.md`
  after every slice (unit id in the commit subject).
- Non-negotiable doctrine: `docs/decisions/0003-task-tree-and-commit-doctrine.md` (no code change without
  an owning task-tree first; signoff quality; zero ROADMAP↔code↔mdBook drift; push ~every 200 commits;
  artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh`; hooks + CI run it too. Retrieval starts at bounded
  `KNOWLEDGE_MAP.md`, then its linked question shards.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v` is complete, fully qualified, and pending commit.
  `.6d.ii.e.iv.iv`, `.6d.ii.e.iv.iii`, `.6d.ii.e.iv.ii`, `.6d.ii.e.iv.i`, `.6d.ii.e.iii`,
  `.6d.ii.e.ii`, `.6d.ii.e.i`, `.6d.ii.d.iv`, `.6d.ii.d.iii`, `.6d.ii.d.ii`, `.6d.ii.d.i`,
  `.6d.ii.c`, `FSMGEN-REFRESH-INTEGRATE-8.1`, `.6d.ii.b`, `.6d.ii.a`, and
  `.6d.i` are committed complete. Tracking-only: `STATUS-LEDGER-ROLLOVER.2`
  and `TASK-PART-SEAL-REACHABILITY.0`.
- Current state: SourceIR, EvidenceIR, SemanticIR, and IntentIR have executable cumulative authority. IntentIR
  schema 2 proves 49 public fields across nine homogeneous families, directly binds 30 exact carries to immediate
  SemanticIR, separates filtered actors and NLI-filterable contracts into projection authority, and closes
  validation plus one-to-one order-preserving NLI demotion. Exactly 24 reachable IntentIRs migrated with zero
  pre-existing public-field delta; 54 remain proof-unmeasurable behind legacy SemanticIR. Final chain replay is
  24 current / zero stale at every proof-bearing stage; full CI and 156/156 KG fixtures pass. `.e.iv.vii` owns
  whole-module digest over-invalidation.
- Next action: commit `.e.iv.v` with its required subject, verify a clean boundary and zero-byte commit brief,
  then activate adapter proof leaf `.e.iv.vi`.
- In-flight uncommitted: the fully implemented and qualified IntentIR proof slice, exact migration/adapter
  rebuild, enforcement adjustments, public/live/book/retrieval alignment, and task evidence. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
