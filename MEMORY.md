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
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.7b` owns production recovery and exact retained-chain reconciliation.
  `.7a` has frozen the design; `.7c` remains the complete reviewed replay and publication slice.
  Tracking-only: `STATUS-LEDGER-ROLLOVER.2`, `TASK-PART-SEAL-REACHABILITY.0`,
  `CLAIM-VERIFICATION-ADOPTION.1a`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`.
- Current state: current replay authority reports one missing APB canonical fact at the SourceIR-to-EvidenceIR
  boundary. The compound sentence explicitly says `PSEL` is asserted and then infers that three other signals
  must be valid. The existing extractor correctly prevents `PSEL` from borrowing the consequence's `VALID`
  value, but no independent producer preserves the antecedent's own asserted state. The `.7a` machine contract
  now binds a source-grounded sibling producer, closed asserted/deasserted and polarity grammar, positive and
  refusal controls, every proof-affected retained chain, and the complete population replay obligation. The
  exact source-local `PSEL` identity is grounded by its same-clause signal appositive; distinct declared `PSELX`
  remains opaque and an explicit refusal proves suffix spelling cannot alias it. Because the reviewed evidence
  establishes no polarity, the corrected missing key is polarity-neutral `PSEL|must_be_asserted|<missing>`;
  HIGH, LOW, and borrowed VALID are forbidden.
  Existing current-result and consequence-precision baselines remain unchanged; `.7b` is the sole frontier.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: execute `.7b`: implement the frozen sibling producer, run the complete conformance matrix and
  genericity controls, rebuild all affected retained chains, and require zero stale measurable stages.
- In-flight uncommitted: `.7a` contract/checker and synchronized task/live/book/retrieval records await the
  commit workflow; no production Rust change or background job.
- Blockers: none. The user-owned `.claude/settings.json` is untouched.
