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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` is in progress, owning the measured I2C production
  symbol-spelling/ordering coupling. Tracking-
  only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`, `DECISION-RECORD-CAPACITY-HEADROOM.1`, and
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: four structural production causes are repaired and the production slice's full repository gate
  passes: opaque underscore identifiers remain one grammar
  token; actor-signal and semantic signal collections/ordinals follow first source occurrence; and declared aliases
  cannot grant section-topic semantics. The exact six-signal/352-occurrence diagnostic now preserves equal stage
  record counts and 11,170/11,170 proof claims over 266,194 leaves. Five SemanticIR reference ids, 78 downstream
  set/reference values, and one unordered ISF declaration block remain comparator-owned. Focused controls pass;
  the current compiled-flow census is 2,270 functions / 11,902 edges / 11,355 decisions / 1,443 macros. ADR 0025
  reconciliation rebuilt the exact two affected retained chains; currency is 24/24 current and zero stale through
  EvidenceIR, SemanticIR, IntentIR, and adapters, with unchanged proof counts and blocked/no-file outcomes. All nine
  doctrines, all 11 production-genericity components, 1,976 Rust tests / eight ignored / zero failed, five
  compile-fail doctests, warning-denied Clippy/Rustdoc, mdBook, and final locality pass.
- Next action: commit the verified production remediation, then fulfill the director's new `MEMORY.md` 32,768-byte
  cap directive under its own task-tree leaf before resuming clean-revision comparator replay/publication.
- In-flight uncommitted: the verified production code, graph snapshot, exact retained-chain refresh, and synchronized
  task/live/book/retrieval evidence await the first `.f.iii.a` commit. No background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
