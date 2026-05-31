# Repo-Local Task Tree Workflow

This document defines the repo-local task-tree workflow used by SpecForge.
Adapted from the FSMGen task-tree system.

See [`docs/TASK_TREE_README.md`](TASK_TREE_README.md) for the system overview
and the governing doctrine (no code change without an owning tree; every
activity tracked; past work audited into trees; ROADMAP ↔ codebase ↔ mdBook
locked with zero drift). Whole-roadmap task-tree coverage + the past-work audit
is driven by [`docs/tasks/ROADMAP-TASKTREE-COVERAGE.md`](tasks/ROADMAP-TASKTREE-COVERAGE.md).

## Purpose

Use a task tree when a top-level task is too broad to finish safely as one
signoff-level slice, or when a task is expected to discover subtasks and
sub-subtasks over time.

The goal is not to create a second roadmap. The roadmap states the high-level
workstream direction. A task tree owns the recursive breakdown, current
frontier, acceptance criteria, blockers, decisions, validation, and completion
evidence for one top-level task.

## Active Task Trees

| Tree | Status | Roadmap lane | Current frontier | File |
| --- | --- | --- | --- | --- |
| `PROVENANCE-HARDENING` | `done` | `R6` | — | [docs/tasks/PROVENANCE-HARDENING.md](docs/tasks/PROVENANCE-HARDENING.md) |
| `R6-FSM-ADAPTER` | `superseded` | `R6` | — (superseded by `ISF-ONLY-CONSOLIDATION`) | [docs/tasks/R6-FSM-ADAPTER.md](docs/tasks/R6-FSM-ADAPTER.md) |
| `R6-SOURCE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-SOURCE-HARDENING.md](docs/tasks/R6-SOURCE-HARDENING.md) |
| `R6-CONVERGE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-CONVERGE-HARDENING.md](docs/tasks/R6-CONVERGE-HARDENING.md) |
| `R6-EVIDENCE-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-EVIDENCE-HARDENING.md](docs/tasks/R6-EVIDENCE-HARDENING.md) |
| `R6-INTENT-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-INTENT-HARDENING.md](docs/tasks/R6-INTENT-HARDENING.md) |
| `R6-SEMANTIC-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-SEMANTIC-HARDENING.md](docs/tasks/R6-SEMANTIC-HARDENING.md) |
| `R6-PRIOR-MEMORY-HARDENING` | `done` | `R6` | — | [docs/tasks/R6-PRIOR-MEMORY-HARDENING.md](docs/tasks/R6-PRIOR-MEMORY-HARDENING.md) |
| `R15-GRAPH-DIRECTION-MIGRATION` | `done` | `R15` | — | [docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md](docs/tasks/R15-GRAPH-DIRECTION-MIGRATION.md) |
| `R7-VALIDATION` | `done` | `R7` | — (closed `2026-05-20`; `.1`–`.4` implemented `2026-05-16`; `.5` design-deliverable landed `2026-05-20` — tracked-approval-evidence design for canonical IR mutation; implementation pathway stays gated on user-owned decision) | [docs/tasks/R7-VALIDATION.md](docs/tasks/R7-VALIDATION.md) |
| `SIGNOFF-REMEDIATION` | `done` | `R0` | — | [docs/tasks/SIGNOFF-REMEDIATION.md](docs/tasks/SIGNOFF-REMEDIATION.md) |
| `R6-ISF-ADAPTER` | `done` | `R6` | — | [docs/tasks/R6-ISF-ADAPTER.md](docs/tasks/R6-ISF-ADAPTER.md) |
| `ISF-ONLY-CONSOLIDATION` | `done` | `R6` | — | [docs/tasks/ISF-ONLY-CONSOLIDATION.md](docs/tasks/ISF-ONLY-CONSOLIDATION.md) |
| `ISF-TEMPORAL-LOWERING` | `done` | `R15b` | — | [docs/tasks/ISF-TEMPORAL-LOWERING.md](docs/tasks/ISF-TEMPORAL-LOWERING.md) |
| `ISF-ONLY-IR-PRUNE` | `done` | `R6` | — | [docs/tasks/ISF-ONLY-IR-PRUNE.md](docs/tasks/ISF-ONLY-IR-PRUNE.md) |
| `AUDIT-DOC-RECONCILE` | `done` | `R0` | — | [docs/tasks/AUDIT-DOC-RECONCILE.md](docs/tasks/AUDIT-DOC-RECONCILE.md) |
| `FSMGEN-ISSUE-REPORTING` | `done` | `R0` | — | [docs/tasks/FSMGEN-ISSUE-REPORTING.md](docs/tasks/FSMGEN-ISSUE-REPORTING.md) |
| `FSMGEN-SUBMODULE-BUMP` | `done` | `R0` | — | [docs/tasks/FSMGEN-SUBMODULE-BUMP.md](docs/tasks/FSMGEN-SUBMODULE-BUMP.md) |
| `ISF-HANDSHAKE-STAGE-LOWERING` | `superseded` | `R15b` | — (delivered via `R16-CONTRACT-IR.4`) | [docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md](docs/tasks/ISF-HANDSHAKE-STAGE-LOWERING.md) |
| `R16-INTENT-CAPTURE` | `done` | `R16` | — (umbrella CLOSED `2026-05-29`; **R16 program complete** — all 6 sub-trees done at their honest scope boundaries #1/#2/#3/#4/#5/#6; 2 remaining crux items as honestly-deferred FUTURE trees: CVE producer wiring (**NOT** upstream-blocked — the Ollama+Qwen2.5VL prose provider is production-default; needs wiring into `parse_constrained_contract`) + waveform raster/vector `FigureRegion` extraction (the genuinely upstream-absent piece)) | [docs/tasks/R16-INTENT-CAPTURE.md](docs/tasks/R16-INTENT-CAPTURE.md) |
| `R16-MODULE-HARDENING` | `done` | `R16` | — (CLOSED `2026-05-29`; all 7 R16 IR modules at the unit-test signoff bar via `.1`–`.4`; R16 module tests `67 → 86`) | [docs/tasks/R16-MODULE-HARDENING.md](docs/tasks/R16-MODULE-HARDENING.md) |
| `FSMGEN-REFRESH-INTEGRATE` | `done` | `R6` | — (CLOSED `2026-05-29`: bumped `subs/fsmgen` `9bfb9a20` → `88a7af9c` + verified, ISF feature-adoption assessment recorded; adoptable work → proposed trees `ISF-SYMBOL-SURFACE-EMIT` + `ISF-TXN-GRAMMAR-FIX`) | [docs/tasks/FSMGEN-REFRESH-INTEGRATE.md](docs/tasks/FSMGEN-REFRESH-INTEGRATE.md) |
| `ISF-TXN-GRAMMAR-FIX` | `done` | `R6` | — (CLOSED `2026-05-29`: 6 emitter grammar sites corrected to FSMGen-contract-exact forms `shift_left`/`shift_right`/`await_all`/`await_any` + `spawn … as`; book-verified; render-lock test) | [docs/tasks/ISF-TXN-GRAMMAR-FIX.md](docs/tasks/ISF-TXN-GRAMMAR-FIX.md) |
| `ISF-SYMBOL-SURFACE-EMIT` | `done` | `R6` | — (CLOSED `2026-05-29`: ingested FSMGen's enum-type answer (pin `c0b7eaa7`); `render()` now emits `(types)`/`(enums)`/`(constants)`, fsmgen-`--strict`-verified e2e; count-vs-emission honesty gap closed) | [docs/tasks/ISF-SYMBOL-SURFACE-EMIT.md](docs/tasks/ISF-SYMBOL-SURFACE-EMIT.md) |
| `ISF-SYMBOL-COUNT-EMITTED` | `done` | `R6` | — (CLOSED `2026-05-29`: adapter `constant_count`/`enum_count` now count emitted content via DRY-shared `emitted_*` accessors — metric == emitted, mirroring the transaction/rule counts) | [docs/tasks/ISF-SYMBOL-COUNT-EMITTED.md](docs/tasks/ISF-SYMBOL-COUNT-EMITTED.md) |
| `R16-CONTRACT-IR` | `done` | `R16` | — (program point #1 delivered) | [docs/tasks/R16-CONTRACT-IR.md](docs/tasks/R16-CONTRACT-IR.md) |
| `R16-KG-PROTOCOL-ONTOLOGY` | `done` | `R16` | — (program point #2 delivered `2026-05-20`; typed `protocol_graph` + projection + `validate` count surface; kg-bench fixtures deferred to extraction trees) | [docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md](docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md) |
| `R16-CAPTURE-FIDELITY-GATES` | `done` | `R16` | — (program point #5/order-3 delivered `2026-05-20`; typed `fidelity` + producer with honesty-doctrine routing + `validate fidelity:` block; corpus baseline `fail=0 score=1.000`) | [docs/tasks/R16-CAPTURE-FIDELITY-GATES.md](docs/tasks/R16-CAPTURE-FIDELITY-GATES.md) |
| `R16-MULTIMODAL-CONTRACT-FUSION` | `done` | `R16` | — (program point #3 delivered `2026-05-20`; typed `fusion` + producer with disagreement→Residual + `validate fusion:` block; corpus baseline `groups_merged=0 disagreements=0`; load-bearing for extraction) | [docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md](docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md) |
| `R16-WAVEFORM-CONTRACT-MINING` | `done` | `R16` | — (program point #4 fully delivered `2026-05-20`: `.1`+`.2`+`.3` (`.3.1`+`.3.2`)+`.4`; `validate waveform: figure_contracts=0 verifier_fail_residuals=0` corpus baseline; raster/vector handling deferred to a future tree when upstream PDF pipeline produces those bytes) | [docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md](docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` | `done` | `R16` | — (program point #6 delivered `2026-05-20`; fails-closed adapter + entailment verifier + template library + uncertainty VoI selector + `validate constrained:` block; corpus baseline 0/0/0; closes R16 program) | [docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md](docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md) |
| `BOOK-METHOD-DOC` | `done` | `R0` | — (closed `2026-05-20`: convention + placement map + 7+2 backfilled trees + close-rule encoded in workflow doctrine; close-rule now structurally enforced via Completion Rules + Required Commit Workflow) | [docs/tasks/BOOK-METHOD-DOC.md](docs/tasks/BOOK-METHOD-DOC.md) |
| `BOOK-USER-FRIENDLY-BACKFILL` | `done` | `R0` | — (closed `2026-05-20`: 10 concept-introducing subsections upgraded across 5 book chapters via per-subsection sub-leaves; BOOK-METHOD-DOC Decisions cross-referenced — standard fully self-referential) | [docs/tasks/BOOK-USER-FRIENDLY-BACKFILL.md](docs/tasks/BOOK-USER-FRIENDLY-BACKFILL.md) |
| `AUDIT-PROVIDER-FRAMING-RECONCILE` | `done` | `R0` | — (CLOSED `2026-05-30`: reconciled the "prose LLM/VLM provider doesn't exist / R16 CVE crux upstream-blocked" framing across ROADMAP/TASK_TREE/INTENTIR_SPEC/RUST_CODEBASE_ANALYSIS — provider is production-default Ollama+Qwen2.5VL; CVE wiring NOT upstream-blocked) | [docs/tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md](docs/tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md) |
| `CVE-PROSE-EXTRACTION` | `done` | `R16` | — (CLOSED `2026-05-31`: live prose→`ActorContract` extractor wired into the R16 constrained-verified surface — `extract-contracts` command (Qwen via Ollama), fails-closed + entailment-gated, folded into `actor_contracts` before fusion; `validate` `schema_rejects` now real. Live inference dispatched but server-gated; wiring proven by unit tests + skip-mode. lib →1159) | [docs/tasks/CVE-PROSE-EXTRACTION.md](docs/tasks/CVE-PROSE-EXTRACTION.md) |
| `R14-SIGNAL-RESOLVE` | `done` | `R14` | — (CLOSED `2026-05-31`: Tier-3 LLM `signal_relation` extraction delivered as the `signal-resolve` command (Qwen via Ollama) — grounded + deduped `ActorSignalRelation` edges appended to the existing `EvidenceIR.actor_signal_relations` KG field; pure classifier 5-path-tested. Live run server-gated; skip-mode verified. lib →1164) | [docs/tasks/R14-SIGNAL-RESOLVE.md](docs/tasks/R14-SIGNAL-RESOLVE.md) |
| `LLM-TEXT-TRANSPORT-DEDUP` | `done` | `R0` | — (CLOSED `2026-05-31`: consolidated the text chat transport `extract_contracts`+`signal_resolve` duplicated into a shared `commands/llm_text.rs`; 3→2 copies (`nlp_enrich` inline left as follow-up); +3 transport tests; no behavior change; CI green) | [docs/tasks/LLM-TEXT-TRANSPORT-DEDUP.md](docs/tasks/LLM-TEXT-TRANSPORT-DEDUP.md) |
| `ISF-RULE-CONFLICT-RESIDUAL` | `done` | `R6` | — (CLOSED `2026-05-31`: `.isf` emitter dedup now records each dropped value-conflicting rule as an explicit `ResidualDecisionPacket` (→ `residual_decisions`) instead of silently dropping it; emitted `.isf` byte-identical/strict-valid; extracted testable `dedup_conflicting_rules`; +2 tests; book subsection) | [docs/tasks/ISF-RULE-CONFLICT-RESIDUAL.md](docs/tasks/ISF-RULE-CONFLICT-RESIDUAL.md) |
| `ROADMAP-TASKTREE-COVERAGE` | `active` | `R0` | `ROADMAP-TASKTREE-COVERAGE.4` (R1–R5 + R8–R13 done; next R15c–g, then the ROADMAP↔code↔mdBook alignment lock) — turn the WHOLE roadmap into owned, audited task-trees (user doctrine directive `2026-05-31`) | [docs/tasks/ROADMAP-TASKTREE-COVERAGE.md](docs/tasks/ROADMAP-TASKTREE-COVERAGE.md) |
| `R1-R5-FOUNDATION-BACKFILL` | `done` | `R1-R5` | — (CLOSED `2026-05-31`: backfilled ownership + meticulous audit for R1–R5 (CLI/IntentIR pivot, SourceIR, EvidenceIR, SemanticIR, IntentIR) — all delivered, tested (source 12 / evidence 120 / semantic 381 / intent 52), book-covered, ROADMAP criteria met; `ROADMAP-TASKTREE-COVERAGE.2`) | [docs/tasks/R1-R5-FOUNDATION-BACKFILL.md](docs/tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| `R8-R13-EXTRACTION-BACKFILL` | `done` | `R8-R13` | — (CLOSED `2026-05-31`: backfilled ownership + meticulous audit for R8–R13 (SourceIR Tier-1 capture, EvidenceIR Tier-2 typed evidence, Tier-3 VLM visual, NLP-L3 enrichment, multi-spec validation, Tier-2 relation extraction) — all delivered + book-covered, ROADMAP criteria met; R9 = Mostly Done with residual robustness-hardening noted; `ROADMAP-TASKTREE-COVERAGE.3`) | [docs/tasks/R8-R13-EXTRACTION-BACKFILL.md](docs/tasks/R8-R13-EXTRACTION-BACKFILL.md) |

## Directory Layout

```text
docs/TASK_TREE.md
docs/tasks/
  TEMPLATE.md
  <TREE>.md
```

`docs/TASK_TREE.md` is the workflow and active-tree index.
Each top-level task owns one file in `docs/tasks/`.
`docs/tasks/TEMPLATE.md` is copied when creating a new top-level tree.

## Definitions

- Task tree: the recursive decomposition of one top-level task.
- Node: one item in that tree.
- Container node: a node with children. It is not directly executable.
- Leaf node: a node with no children. It is the only unit PNT may implement.
- Current frontier: the ordered set of leaf nodes that are eligible to be
  picked next.
- Slice: one completed leaf task plus its tests, docs, live-doc updates, and
  commit workflow.
- Evidence: the validation output, changed-doc summary, and git commit subject
  that prove a leaf was completed.

## ID Rules

Each task tree has a stable top-level ID.

```text
<TREE>
<TREE>.1
<TREE>.1.1
<TREE>.1.1.1
```

Rules:

- `<TREE>` uses uppercase letters, digits, and hyphens.
- Child IDs append dot-separated positive integers.
- IDs are permanent once published.
- Never renumber closed nodes.
- If a new ordering is needed, add new IDs and mark old nodes `superseded` or
  `deferred` with a reason.
- A commit that completes a task-tree leaf must identify the leaf ID in the
  commit subject or in the first body line.

## Status Vocabulary

Use only these statuses.

| Status | Meaning |
| --- | --- |
| `proposed` | Captured but not yet accepted into the active tree. |
| `active` | The top-level tree is open, or a container has unfinished children. |
| `pending` | Ready to be selected once it reaches the current frontier. |
| `in_progress` | Currently being implemented in the worktree. |
| `blocked` | Cannot proceed without a named blocker and unblock condition. |
| `done` | Completed, validated, documented, and committed. |
| `deferred` | Deliberately postponed with an explicit consequence. |
| `superseded` | Replaced by another node, with the replacement ID named. |

## Required Task File Sections

Every top-level task file must contain:

- Metadata: tree ID, status, roadmap lane, created date, last updated date.
- Goal: the user-visible or project-visible outcome.
- Non-goals: what this tree deliberately does not try to solve.
- Acceptance criteria: concrete conditions that close the top-level task.
- Task tree: all known nodes, with status and short result intent.
- Current frontier: ordered leaf nodes that PNT may select next.
- Decisions: accepted technical decisions and their rationale.
- Open questions: unresolved questions that do not block the whole tree yet.
- Blockers: blockers with unblock conditions.
- Verification log: checks run for completed leaves.
- Commit log: leaf IDs mapped to completion commit subjects.
- Changelog: dated edits to the tree itself.

## Node Rules

Every node must be one of these two shapes.

Container node:

```text
- ID: <TREE>.<n>
  Status: active
  Goal: ...
  Children: <TREE>.<n>.1, <TREE>.<n>.2
```

Leaf node:

```text
- ID: <TREE>.<n>
  Status: pending
  Goal: ...
  Acceptance: ...
  Verification: pending
  Commit: pending
```

A node with children must not be marked `done` until every child is `done`,
`deferred`, or `superseded`, and every non-`done` child has a recorded reason.

## Current Frontier Rules

The current frontier is the only list PNT uses when selecting work from a task
tree.

Rules:

- The frontier contains only leaf nodes.
- The frontier is ordered by intended priority.
- A container never appears in the frontier.
- A blocked node stays out of the frontier until unblocked.
- When a leaf is split, remove that leaf from the frontier, mark it `active`,
  add children, and place the first executable child or children in the
  frontier.
- When a leaf completes, remove it from the frontier and add the next eligible
  leaf or leaves.

## PNT Selection Rules

When PNT is asked to continue and at least one active task tree exists:

1. Read `docs/TASK_TREE.md`.
2. Read the active task file named in the `Active Task Trees` table.
3. Pick the first eligible leaf in that file's `Current Frontier`.
4. Implement only that leaf.
5. If the leaf is too broad, split it before implementation and commit the
   tree update as the leaf's honest outcome.
6. Run the required validation for the leaf.
7. Update the task file, live docs, and roadmap if status changed.
8. Run the full commit workflow before selecting another leaf.

If several active trees exist, choose the first active tree in the table unless
the user names another tree or the roadmap status names a different immediate
lane.

## Splitting Rules

Split a node when any of these are true:

- It cannot be completed to signoff quality in one slice.
- It mixes design, implementation, diagnostics, tests, and docs in ways that
  can be reviewed independently.
- It hides an unresolved policy choice behind implementation wording.
- It would require touching unrelated ownership areas in one commit.
- It discovers a lower-level dependency that should be solved first.

Do not split merely to create vague placeholders. Every child must have a
clear goal and a way to verify completion.

## Completion Rules

A leaf is complete only when all of the following are true:

- Implementation or documentation work for that leaf is finished.
- Focused checks passed, and broader checks ran when warranted.
- The owning task file records the result, validation, and commit subject.
- `MEMORY.md`, `CHANGES.md`, `DEVELOPMENT_NOTES.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, and `ROADMAP.md` are updated when the
  leaf changes project state.
- **Book method-doc close-rule** (`BOOK-METHOD-DOC`): a tree's
  closing leaf (`.N`) MUST add or refresh that tree's
  implementation+verification subsection in the topically-correct
  mdBook chapter (per the placement map in
  `docs/tasks/BOOK-METHOD-DOC.md`). The subsection is the
  human-facing "how + why + how-verified"; the task-tree file
  remains the machine-tracked authority. A close leaf whose book
  section is missing or stale is incomplete — the book / ROADMAP
  language always describes what the code does (the
  `AUDIT-DOC-RECONCILE` doctrine).
- The commit workflow in `COMMIT.md` has completed.
- `git_message_brief.txt` has been cleared after commit.

Commit hashes are intentionally not required inside the same task-file update:
the final hash cannot be known until after the commit exists. The stable
join key is the leaf ID in the commit subject or first body line. Later status
refreshes may backfill hashes if useful.

## Blocker Rules

A blocked node must record:

- the exact blocker,
- why it blocks the node,
- the unblock condition,
- and the next task that should run instead, if any.

Do not leave a node as `blocked` only because it is large or unclear. Large or
unclear work should be split until a real blocker is visible.

## Relationship To Live Docs

The task tree is the detailed execution ledger.

- `ROADMAP.md` remains the canonical high-level workstream status.
- `MEMORY.md` remains the recovery/handoff continuity log.
- `CHANGES.md` remains the chronological technical history.
- `DEVELOPMENT_NOTES.md` remains design rationale.
- `LIVE_ACHIEVEMENT_STATUS.md` remains the latest completed slice summary.
- The mdBook remains user-facing product/language documentation.

Do not duplicate the whole task tree into those files. Link to the task tree
and summarize only the part that changes live project state.

## SpecForge-Specific Defaults

- All R6 `.fsm` adapter hardening and provenance-field hardening work is
  task-tree-managed by default.
- Active PNT cycles select from the first active tree's current frontier
  unless the user names a different lane.
- The canonical validation command is `cargo test -p specforge --lib`.
- Commit messages for task-tree-managed leaves must include the leaf ID
  (e.g. `PROV-HARDEN.4`) in the commit subject or first body line.
