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

<!-- task_catalog:start -->
## Task Tree Catalog

This derived catalog is navigation, not execution history. Read `MEMORY.md` for the single
resume pointer, then open the owning tree for its frontier, decisions, evidence, and commits.
The author template is linked separately and is never classified as active work.

| Tree | Status | Purpose | File |
| --- | --- | --- | --- |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT` | `done` | keep active task history bounded and resumable | [open](tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md) |
| `AMBIGUITY-PHRASE-DETECTOR` | `done` | flag vague / under-specified spec prose for review | [open](tasks/AMBIGUITY-PHRASE-DETECTOR.md) |
| `ARTIFACT-PATH-PORTABILITY` | `done` | repository-relative IR provenance and move-safe generated artifacts | [open](tasks/ARTIFACT-PATH-PORTABILITY.md) |
| `AUDIT-DOC-RECONCILE` | `done` | fix doc drift found by the post-ISF-ONLY audit | [open](tasks/AUDIT-DOC-RECONCILE.md) |
| `AUDIT-PROVIDER-FRAMING-RECONCILE` | `done` | reconcile live-doc framing that the LLM/VLM provider "doesn't exist" / R16 CVE crux is "upstream-blocked" | [open](tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md) |
| `BOOK-COMMAND-COVERAGE` | `done` | mdBook command-surface drift reconciliation | [open](tasks/BOOK-COMMAND-COVERAGE.md) |
| `BOOK-METHOD-DOC` | `done` | per-task-tree implementation & verification, in the book | [open](tasks/BOOK-METHOD-DOC.md) |
| `BOOK-USER-FRIENDLY-BACKFILL` | `done` | upgrade existing book subsections to the user-friendly standard | [open](tasks/BOOK-USER-FRIENDLY-BACKFILL.md) |
| `CANONICAL-PROMOTION-SWEEP` | `done` | land the default LLM-primary constraint promotion across the corpus's canonical artifacts | [open](tasks/CANONICAL-PROMOTION-SWEEP.md) |
| `COMPLETENESS-CLOSURE-INVARIANTS` | `done` | first completeness miss-detectors (symbol closure + register tiling) | [open](tasks/COMPLETENESS-CLOSURE-INVARIANTS.md) |
| `COMPLETENESS-RECALL-GAUGE` | `done` | a calibrated capture–recapture recall estimate | [open](tasks/COMPLETENESS-RECALL-GAUGE.md) |
| `COMPLETENESS-RECALL-RELATIONS` | `done` | extend per-extractor tagging + recall gauge to actor-signal relations | [open](tasks/COMPLETENESS-RECALL-RELATIONS.md) |
| `COMPLETENESS-REGION-ACCOUNTING` | `done` | surface intent-bearing source regions that produced no fact | [open](tasks/COMPLETENESS-REGION-ACCOUNTING.md) |
| `COMPLETENESS-REPORT-SURFACE` | `done` | one honest completeness headline over the detectors | [open](tasks/COMPLETENESS-REPORT-SURFACE.md) |
| `CONSTRAINT-CONDITION-SUBJECT` | `done` | never let a condition-clause signal become a constraint subject | [open](tasks/CONSTRAINT-CONDITION-SUBJECT.md) |
| `CONSTRAINT-DRIVE-LEVEL-RECALL` | `done` | extract "drive <signal> LOW/HIGH" as a value constraint | [open](tasks/CONSTRAINT-DRIVE-LEVEL-RECALL.md) |
| `CONSTRAINT-EXTRACTION-V2` | `done` | fix the 3 constraint bug classes REEXTRACTION-REMEASURE found | [open](tasks/CONSTRAINT-EXTRACTION-V2.md) |
| `CONSTRAINT-SUBJECT-PRECISION` | `done` | stop the constraint extractor minting non-subject signals | [open](tasks/CONSTRAINT-SUBJECT-PRECISION.md) |
| `CORPUS-COVERAGE` | `active` | build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale | [open](tasks/CORPUS-COVERAGE.md) |
| `CORPUS-HARDENING` | `active` | harden SpecForge against the real chip-doc corpus (AMBA core first) | [open](tasks/CORPUS-HARDENING.md) |
| `CORPUS-PATTERN-REUSE` | `active` | reuse extraction patterns across PDFs, clustered by derived vendor/layout fingerprint | [open](tasks/CORPUS-PATTERN-REUSE.md) |
| `CVE-PROSE-EXTRACTION` | `done` | wire a live prose→ActorContract extractor into the R16 constrained-verified surface | [open](tasks/CVE-PROSE-EXTRACTION.md) |
| `DEMPSTER-FUSION-COMBINER` | `done` | corroboration-boosting confidence fusion (Dempster's rule) | [open](tasks/DEMPSTER-FUSION-COMBINER.md) |
| `DOC-INTENT-TAXONOMY` | `active` | chip-spec document intent taxonomy → per-category complete ISF synthesis | [open](tasks/DOC-INTENT-TAXONOMY.md) |
| `DOCLING-DEVICE-CPU-DEFAULT` | `done` | make Docling ingest avoid the broken MPS auto-device | [open](tasks/DOCLING-DEVICE-CPU-DEFAULT.md) |
| `DOCTRINE-ENFORCEMENT-ADOPT` | `done` | adopt the portable Doctrine-Enforcement architecture (4th standard) | [open](tasks/DOCTRINE-ENFORCEMENT-ADOPT.md) |
| `EVAL-DOCUMENT-RECALL` | `done` | attribution-agnostic fact recall (the per-statement scorer under-counts) | [open](tasks/EVAL-DOCUMENT-RECALL.md) |
| `EVAL-GOLD-INTERANNOTATOR-AGREEMENT` | `done` | is the eval answer-key trustworthy? (Cohen's κ) | [open](tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md) |
| `EVAL-RELATION-GRANULARITY` | `done` | per-relation-kind P/R/F1 + MUC near-miss diagnostic | [open](tasks/EVAL-RELATION-GRANULARITY.md) |
| `EVIDENCE-DETERMINISM` | `done` | make the EvidenceIR build reproducible (no content-level non-determinism) | [open](tasks/EVIDENCE-DETERMINISM.md) |
| `EVIDENCE-MATERIALIZE-IDEMPOTENCY` | `done` | don't accumulate stale facts on re-build | [open](tasks/EVIDENCE-MATERIALIZE-IDEMPOTENCY.md) |
| `EXTRACTION-GAP-FIX` | `active` | close the extraction gaps PDF-VARIANT-DIGESTION.4 quantified | [open](tasks/EXTRACTION-GAP-FIX.md) |
| `EXTRACTION-QUALITY-GAUGE` | `active` | measure the extraction-quality gap — and CHI's is large | [open](tasks/EXTRACTION-QUALITY-GAUGE.md) |
| `EXTRACTOR-ARCHITECTURE` | `done` | make the EvidenceIR extractor path a coherent whole | [open](tasks/EXTRACTOR-ARCHITECTURE.md) |
| `FACT-CARD-CATALOG-CONTAINMENT` | `done` | keep fact-card browsing bounded before capacity fails | [open](tasks/FACT-CARD-CATALOG-CONTAINMENT.md) |
| `FSMGEN-ASSERT-LOWERING` | `done` | lower stable / antecedent→consequent / min>1 obligations into the ISF verification family | [open](tasks/FSMGEN-ASSERT-LOWERING.md) |
| `FSMGEN-ASSERT-MIGRATE` | `done` | re-pin to 43b29f5c + migrate `(contract … eventually …)` → `(assert (monitor …))` | [open](tasks/FSMGEN-ASSERT-MIGRATE.md) |
| `FSMGEN-ISSUE-REPORTING` | `done` | File the FSMGen doc-vs-strict findings via the bundle protocol | [open](tasks/FSMGEN-ISSUE-REPORTING.md) |
| `FSMGEN-LTL-MTL-SUGGESTION` | `done` | suggest first-class LTL/MTL temporal properties in ISF | [open](tasks/FSMGEN-LTL-MTL-SUGGESTION.md) |
| `FSMGEN-MIN-WINDOW-CONFIRM` | `done` | answer FSMGen's `min > 1` window question | [open](tasks/FSMGEN-MIN-WINDOW-CONFIRM.md) |
| `FSMGEN-REFRESH-INTEGRATE-2` | `done` | refresh the FSMGen submodule (2026-06) + re-assess adoptable ISF features | [open](tasks/FSMGEN-REFRESH-INTEGRATE-2.md) |
| `FSMGEN-REFRESH-INTEGRATE-3` | `done` | bump FSMGen submodule to the phase-membership-response tip | [open](tasks/FSMGEN-REFRESH-INTEGRATE-3.md) |
| `FSMGEN-REFRESH-INTEGRATE-4` | `done` | refresh the FSMGen pin + integrate FSMGen's answer to the field-structured-storage FR | [open](tasks/FSMGEN-REFRESH-INTEGRATE-4.md) |
| `FSMGEN-REFRESH-INTEGRATE-5` | `done` | refresh the FSMGen pin to the SHIPPED declarative storage fields + un-gate DOC-INTENT-TAXONOMY.4a.ii | [open](tasks/FSMGEN-REFRESH-INTEGRATE-5.md) |
| `FSMGEN-REFRESH-INTEGRATE` | `done` | refresh the FSMGen submodule + assess adoptable ISF features | [open](tasks/FSMGEN-REFRESH-INTEGRATE.md) |
| `FSMGEN-SUBMODULE-BUMP` | `done` | pin to upstream that fixed both reported findings | [open](tasks/FSMGEN-SUBMODULE-BUMP.md) |
| `FULL-PAGE-INTENT-CAPTURE` | `done` | use the full scope of a page's visual information | [open](tasks/FULL-PAGE-INTENT-CAPTURE.md) |
| `GRITS-CROSS-TOOL` | `done` | a table-structure gold from independent-witness agreement | [open](tasks/GRITS-CROSS-TOOL.md) |
| `INTENT-COMPLETENESS-RESEARCH` | `done` | theory + design for detecting & bounding intent-capture misses | [open](tasks/INTENT-COMPLETENESS-RESEARCH.md) |
| `ISF-HANDSHAKE-STAGE-LOWERING` | `superseded` | lower HandshakeComplete temporal_rules to `(stage …)` | [open](tasks/ISF-HANDSHAKE-STAGE-LOWERING.md) |
| `ISF-ONLY-CONSOLIDATION` | `done` | Drop HDL + `.fsm` adapters; SpecForge emits only `.isf` | [open](tasks/ISF-ONLY-CONSOLIDATION.md) |
| `ISF-ONLY-IR-PRUNE` | `done` | remove genuinely `.fsm`-era orphaned IR surfaces | [open](tasks/ISF-ONLY-IR-PRUNE.md) |
| `ISF-REGISTER-RESET-EMIT` | `done` | lower extracted register reset values into the ISF `(storage (var … (reset V)))` surface | [open](tasks/ISF-REGISTER-RESET-EMIT.md) |
| `ISF-RULE-CONFLICT-RESIDUAL` | `done` | surface dropped value-conflicting rules as explicit residuals (not silent loss) | [open](tasks/ISF-RULE-CONFLICT-RESIDUAL.md) |
| `ISF-SYMBOL-COUNT-EMITTED` | `done` | make the adapter `constant_count`/`enum_count` reflect emitted content | [open](tasks/ISF-SYMBOL-COUNT-EMITTED.md) |
| `ISF-SYMBOL-SURFACE-EMIT` | `done` | emit the built-but-discarded `(constants)`/`(types)`/`(enums)` ISF surface | [open](tasks/ISF-SYMBOL-SURFACE-EMIT.md) |
| `ISF-TEMPORAL-LOWERING` | `done` | lower IntentIR.temporal_rules into the `.isf` adapter | [open](tasks/ISF-TEMPORAL-LOWERING.md) |
| `ISF-TXN-GRAMMAR-FIX` | `done` | correct SpecForge's `.isf` transaction-step emitter grammar | [open](tasks/ISF-TXN-GRAMMAR-FIX.md) |
| `ISF-VALUE-WIDTH-EMIT` | `done` | width-align ISF value literals to the declared signal width | [open](tasks/ISF-VALUE-WIDTH-EMIT.md) |
| `KG-ISF-COMPLETENESS` | `active` | the KG/IntentIR must be COMPLETE enough to lower faithfully to ISF | [open](tasks/KG-ISF-COMPLETENESS.md) |
| `KG-ISF-TRANSACTIONS` | `active` | every supported protocol transaction + its signals, fully captured & ISF-ready | [open](tasks/KG-ISF-TRANSACTIONS.md) |
| `KNOWLEDGE-MAP-ADOPTION` | `done` | adopt the portable Knowledge Map retrieval layer | [open](tasks/KNOWLEDGE-MAP-ADOPTION.md) |
| `LITERATURE-GROUNDING` | `done` | ground every SpecForge aspect in published research | [open](tasks/LITERATURE-GROUNDING.md) |
| `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION` | `done` | bounded live docs, stable README, and same-volume project data | [open](tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md) |
| `LLM-EXTRACTION-EVAL` | `done` | a labeled precision/recall eval set for the LLM extraction passes | [open](tasks/LLM-EXTRACTION-EVAL.md) |
| `LLM-PRIMARY-PROMOTION` | `done` | promote the LLM-primary constraint surface into the canonical pipeline | [open](tasks/LLM-PRIMARY-PROMOTION.md) |
| `LLM-TEXT-TRANSPORT-DEDUP` | `done` | consolidate the duplicated text chat transport (extract-contracts + signal-resolve) | [open](tasks/LLM-TEXT-TRANSPORT-DEDUP.md) |
| `LOGIC-LEVEL-BOUNDARY` | `done` | resolve HIGH/LOW as universal "how"; centralize the vocabulary | [open](tasks/LOGIC-LEVEL-BOUNDARY.md) |
| `MDBOOK-DOCTEST-HYGIENE` | `done` | classify examples and make the live book doctest-safe | [open](tasks/MDBOOK-DOCTEST-HYGIENE.md) |
| `MEMORY-ARCHITECTURE-DOC` | `done` | author a portable, harness-agnostic durable-memory standard | [open](tasks/MEMORY-ARCHITECTURE-DOC.md) |
| `MEMORY-BOUNDED-INGEST` | `active` | bounded-memory ingestion of very large PDFs | [open](tasks/MEMORY-BOUNDED-INGEST.md) |
| `NLI-CLAIM-CONDITION` | `done` | carry a constraint's condition into its NLI claim (precision fix) | [open](tasks/NLI-CLAIM-CONDITION.md) |
| `NLI-CLAIM-CONNECTOR` | `done` | join the condition clause with a connective so claims read as English | [open](tasks/NLI-CLAIM-CONNECTOR.md) |
| `NLI-ENTAILMENT-VERIFIER` | `done` | a semantic "does the source actually say this?" gate | [open](tasks/NLI-ENTAILMENT-VERIFIER.md) |
| `NLI-GATE-METRIC` | `done` | surface the NLI gate's demotions as a validate metric | [open](tasks/NLI-GATE-METRIC.md) |
| `NLI-INTENT-GATE` | `done` | make the NLI verifier an active IntentIR gate (demote NotEntailed → residual) | [open](tasks/NLI-INTENT-GATE.md) |
| `NLP-ENRICH-TRANSPORT-DEDUP` | `done` | complete the text-transport consolidation (route nlp_enrich through llm_text) | [open](tasks/NLP-ENRICH-TRANSPORT-DEDUP.md) |
| `NLP-SHALLOW-PARSE` | `active` | a deterministic in-Rust shallow-parse tier (subject–verb–object understanding) | [open](tasks/NLP-SHALLOW-PARSE.md) |
| `PDF-AGNOSTIC-EXTRACTION` | `done` | remove all hardcoded chip-spec vocabulary; derive from the document | [open](tasks/PDF-AGNOSTIC-EXTRACTION.md) |
| `PDF-VARIANT-DIGESTION` | `active` | make SpecForge digest as many chip-spec PDF variants as possible | [open](tasks/PDF-VARIANT-DIGESTION.md) |
| `PER-EXTRACTOR-FACT-TAGGING` | `done` | record which extractor found each fact (recall-gauge precondition) | [open](tasks/PER-EXTRACTOR-FACT-TAGGING.md) |
| `PRIOR-DECAY` | `done` | detect & flag contested priors (revision-on-contradiction for CorpusMemory) | [open](tasks/PRIOR-DECAY.md) |
| `PROVENANCE-HARDENING` | `done` | Test Assertion Coverage For Provenance-Like IR Fields | [open](tasks/PROVENANCE-HARDENING.md) |
| `PURE-NLP-INTENT-EXTRACTION` | `active` | model-based intent extraction (ACTIVE — first increment) | [open](tasks/PURE-NLP-INTENT-EXTRACTION.md) |
| `R1-R5-FOUNDATION-BACKFILL` | `done` | own + audit the foundational pipeline milestones (delivered pre-task-tree-system) | [open](tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| `R14-SIGNAL-RESOLVE` | `done` | Tier-3 LLM `signal_relation` extraction (the `signal-resolve` command) | [open](tasks/R14-SIGNAL-RESOLVE.md) |
| `R15-GRAPH-DIRECTION-MIGRATION` | `done` | Complete actor-relative graph direction migration | [open](tasks/R15-GRAPH-DIRECTION-MIGRATION.md) |
| `R15C-CONVERGENCE-REPORT` | `done` | make the convergent extraction loop first-class + inspectable | [open](tasks/R15C-CONVERGENCE-REPORT.md) |
| `R15C-R15G-LEARNING-PLANE-BACKFILL` | `active` | own + audit the learning / eval / corpus lanes (in-progress) | [open](tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| `R16-CAPTURE-FIDELITY-GATES` | `done` | objective capture-fidelity metric (point #5) | [open](tasks/R16-CAPTURE-FIDELITY-GATES.md) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` | `done` | schema-constrained + verified extraction (point #6 — the crux, continuous) | [open](tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md) |
| `R16-CONTRACT-IR` | `done` | typed timed-contract IR layer (point #1) | [open](tasks/R16-CONTRACT-IR.md) |
| `R16-INTENT-CAPTURE` | `done` | SOTA design-intent capture (program umbrella) | [open](tasks/R16-INTENT-CAPTURE.md) |
| `R16-KG-PROTOCOL-ONTOLOGY` | `done` | protocol-structured knowledge graph (point #2) | [open](tasks/R16-KG-PROTOCOL-ONTOLOGY.md) |
| `R16-MODULE-HARDENING` | `done` | unit-test signoff hardening for the R16 IR modules | [open](tasks/R16-MODULE-HARDENING.md) |
| `R16-MULTIMODAL-CONTRACT-FUSION` | `done` | cross-modal evidence → one contract (point #3) | [open](tasks/R16-MULTIMODAL-CONTRACT-FUSION.md) |
| `R16-WAVEFORM-CONTRACT-MINING` | `done` | timing diagram → contract (point #4 — the crux) | [open](tasks/R16-WAVEFORM-CONTRACT-MINING.md) |
| `R6-CONVERGE-HARDENING` | `done` | Converge Module Test Assertion Hardening | [open](tasks/R6-CONVERGE-HARDENING.md) |
| `R6-EVIDENCE-HARDENING` | `done` | Evidence + Adapter Module Test Assertion Hardening | [open](tasks/R6-EVIDENCE-HARDENING.md) |
| `R6-FSM-ADAPTER` | `superseded` | `.fsm` Adapter Hardening | [open](tasks/R6-FSM-ADAPTER.md) |
| `R6-INTENT-HARDENING` | `done` | Intent Module Test Assertion Hardening | [open](tasks/R6-INTENT-HARDENING.md) |
| `R6-ISF-ADAPTER` | `done` | ISF (.isf) adapter ownership backfill + hardening | [open](tasks/R6-ISF-ADAPTER.md) |
| `R6-PRIOR-MEMORY-HARDENING` | `done` | Prior Memory Module Test Assertion Hardening | [open](tasks/R6-PRIOR-MEMORY-HARDENING.md) |
| `R6-SEMANTIC-HARDENING` | `done` | Semantic Module Test Assertion Hardening | [open](tasks/R6-SEMANTIC-HARDENING.md) |
| `R6-SOURCE-HARDENING` | `done` | SourceIR Field Test Assertion Hardening | [open](tasks/R6-SOURCE-HARDENING.md) |
| `R7-VALIDATION` | `done` | R7 Validation and Back-Annotation Hardening | [open](tasks/R7-VALIDATION.md) |
| `R8-R13-EXTRACTION-BACKFILL` | `done` | own + audit the extraction-SOTA milestones (delivered pre-task-tree-system) | [open](tasks/R8-R13-EXTRACTION-BACKFILL.md) |
| `RECALL-CHAO-ESTIMATOR` | `done` | a heterogeneity-robust second recall estimate (Chao) alongside Lincoln–Petersen | [open](tasks/RECALL-CHAO-ESTIMATOR.md) |
| `REEXTRACTION-REMEASURE` | `done` | re-ingest + re-extract the APB spec to measure current quality | [open](tasks/REEXTRACTION-REMEASURE.md) |
| `REGISTER-CLASSIFIER-ENCODING-FP` | `done` | stop DVM/encoding cross-reference tables being mis-read as register maps | [open](tasks/REGISTER-CLASSIFIER-ENCODING-FP.md) |
| `REGISTER-MAP-CLASSIFIER-PRECISION` | `done` | stop the over-eager `register_map` table classification | [open](tasks/REGISTER-MAP-CLASSIFIER-PRECISION.md) |
| `ROADMAP-TASKTREE-COVERAGE` | `done` | every roadmap milestone task-tree-owned + audited + locked to code & mdBook | [open](tasks/ROADMAP-TASKTREE-COVERAGE.md) |
| `ROOT-ROLLING-LEDGER-PRESSURE` | `done` | restore bounded headroom in current continuity ledgers | [open](tasks/ROOT-ROLLING-LEDGER-PRESSURE.md) |
| `SIGNAL-TABLE-COLUMNLESS-RECALL` | `done` | capture signals from column-less Signal\|Description tables | [open](tasks/SIGNAL-TABLE-COLUMNLESS-RECALL.md) |
| `SIGNOFF-REMEDIATION` | `done` | Restore signoff quality at HEAD | [open](tasks/SIGNOFF-REMEDIATION.md) |
| `SPEC-MINING-PROVENANCE` | `done` | name the discipline + a per-author adopt/defer ledger | [open](tasks/SPEC-MINING-PROVENANCE.md) |
| `SWD-SERIAL-EXTRACTION` | `done` | serial-protocol/architecture extraction for SWD/ADI → WIRE-BASED-100 100% | [open](tasks/SWD-SERIAL-EXTRACTION.md) |
| `SYMBOL-CLOSURE-CORPUS-VALIDATION` | `done` | corpus-validate (and settle) the descoped symbol-closure detector | [open](tasks/SYMBOL-CLOSURE-CORPUS-VALIDATION.md) |
| `TABLE-GRITS-CONFORMAL` | `done` | GriTS table-structure metric + split-conformal calibration | [open](tasks/TABLE-GRITS-CONFORMAL.md) |
| `TEMPORAL-ANTECEDENT-RECALL` | `done` | distribute a shared assertion-value across coordinated condition signals | [open](tasks/TEMPORAL-ANTECEDENT-RECALL.md) |
| `TEMPORAL-RULE-EVAL` | `done` | supervised precision/recall/F1 for mined temporal rules | [open](tasks/TEMPORAL-RULE-EVAL.md) |
| `TEMPORAL-RULE-LTL-RENDER` | `done` | render mined temporal rules in standard LTL/MTL notation | [open](tasks/TEMPORAL-RULE-LTL-RENDER.md) |
| `TEMPORAL-RULE-SVA-RENDER` | `superseded` | render mined temporal rules as SystemVerilog Assertions (SVA) | [open](tasks/TEMPORAL-RULE-SVA-RENDER.md) |
| `TRACE-SEVERITY-GATING-AUDIT` | `done` | warnings/errors/fatals must never be masked by a trace/verbosity level | [open](tasks/TRACE-SEVERITY-GATING-AUDIT.md) |
| `VERB-COVERAGE-CORPUS` | `done` | mine a comprehensive normative-verb vocabulary from a real chip-spec corpus | [open](tasks/VERB-COVERAGE-CORPUS.md) |
| `WIRE-BASED-100` | `active` | max every score (100%) on wire-based interface specs (APB/AHB/AXI/SWD/…) | [open](tasks/WIRE-BASED-100.md) |

Authoring template: [`docs/tasks/TEMPLATE.md`](tasks/TEMPLATE.md).

<!-- task_catalog:end -->

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
- The canonical documentation surfaces named by `COMMIT.md` are reviewed. Update each only when
  the truth it owns changed; no-op or ceremonial co-staging is not completion evidence.
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
