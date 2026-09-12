# Task tree catalog part 0001

> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with
> `perl scripts/check_task_tree_catalog.pl --write`.

Complete membership for this range. The bounded landing is
[`docs/TASK_TREE.md`](../TASK_TREE.md); it carries the open trees and routes here for the rest.

| Tree | Status | Purpose | File |
| --- | --- | --- | --- |
| `ACTIVE-TASK-EVIDENCE-CONTAINMENT` | `done` | keep active task history bounded and resumable | [open](../tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md) |
| `ACTOR-NOUN-RELATION-DECLARATION` | `active` | the relation path declares an actor role as a wire | [open](../tasks/ACTOR-NOUN-RELATION-DECLARATION.md) |
| `AMBIGUITY-PHRASE-DETECTOR` | `done` | flag vague / under-specified spec prose for review | [open](../tasks/AMBIGUITY-PHRASE-DETECTOR.md) |
| `ARTIFACT-PATH-PORTABILITY` | `done` | repository-relative IR provenance and move-safe generated artifacts | [open](../tasks/ARTIFACT-PATH-PORTABILITY.md) |
| `AUDIT-DOC-RECONCILE` | `done` | fix doc drift found by the post-ISF-ONLY audit | [open](../tasks/AUDIT-DOC-RECONCILE.md) |
| `AUDIT-PROVIDER-FRAMING-RECONCILE` | `done` | reconcile live-doc framing that the LLM/VLM provider "doesn't exist" / R16 CVE crux is "upstream-blocked" | [open](../tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md) |
| `BOOK-BEHAVIOUR-CURRENCY` | `done` | the book can describe behaviour the code no longer has, and no gate sees it | [open](../tasks/BOOK-BEHAVIOUR-CURRENCY.md) |
| `BOOK-COMMAND-COVERAGE` | `done` | mdBook command-surface drift reconciliation | [open](../tasks/BOOK-COMMAND-COVERAGE.md) |
| `BOOK-METHOD-DOC` | `done` | per-task-tree implementation & verification, in the book | [open](../tasks/BOOK-METHOD-DOC.md) |
| `BOOK-USER-FRIENDLY-BACKFILL` | `done` | upgrade existing book subsections to the user-friendly standard | [open](../tasks/BOOK-USER-FRIENDLY-BACKFILL.md) |
| `CANONICAL-PROMOTION-SWEEP` | `done` | land the default LLM-primary constraint promotion across the corpus's canonical artifacts | [open](../tasks/CANONICAL-PROMOTION-SWEEP.md) |
| `CHANGES-LEDGER-ROLLOVER` | `active` | roll the change ledger before its next append is refused | [open](../tasks/CHANGES-LEDGER-ROLLOVER.md) |
| `CLAIM-VERIFICATION-ADOPTION` | `active` | adopt three-leg verification for published claims | [open](../tasks/CLAIM-VERIFICATION-ADOPTION.md) |
| `COMPLETENESS-CLOSURE-INVARIANTS` | `done` | first completeness miss-detectors (symbol closure + register tiling) | [open](../tasks/COMPLETENESS-CLOSURE-INVARIANTS.md) |
| `COMPLETENESS-RECALL-GAUGE` | `done` | a calibrated capture–recapture recall estimate | [open](../tasks/COMPLETENESS-RECALL-GAUGE.md) |
| `COMPLETENESS-RECALL-RELATIONS` | `done` | extend per-extractor tagging + recall gauge to actor-signal relations | [open](../tasks/COMPLETENESS-RECALL-RELATIONS.md) |
| `COMPLETENESS-REGION-ACCOUNTING` | `done` | surface intent-bearing source regions that produced no fact | [open](../tasks/COMPLETENESS-REGION-ACCOUNTING.md) |
| `COMPLETENESS-REPORT-SURFACE` | `done` | one honest completeness headline over the detectors | [open](../tasks/COMPLETENESS-REPORT-SURFACE.md) |
| `CONSTRAINT-CONDITION-SUBJECT` | `done` | never let a condition-clause signal become a constraint subject | [open](../tasks/CONSTRAINT-CONDITION-SUBJECT.md) |
| `CONSTRAINT-DRIVE-LEVEL-RECALL` | `done` | extract "drive <signal> LOW/HIGH" as a value constraint | [open](../tasks/CONSTRAINT-DRIVE-LEVEL-RECALL.md) |
| `CONSTRAINT-EXTRACTION-V2` | `done` | fix the 3 constraint bug classes REEXTRACTION-REMEASURE found | [open](../tasks/CONSTRAINT-EXTRACTION-V2.md) |
| `CONSTRAINT-SUBJECT-PRECISION` | `done` | stop the constraint extractor minting non-subject signals | [open](../tasks/CONSTRAINT-SUBJECT-PRECISION.md) |
| `CORPUS-CHAIN-CURRENCY` | `done` | prove, not assume, that every persisted chain matches the current binary | [open](../tasks/CORPUS-CHAIN-CURRENCY.md) |
| `CORPUS-COVERAGE` | `active` | build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale | [open](../tasks/CORPUS-COVERAGE.md) |
| `CORPUS-HARDENING` | `active` | harden SpecForge against the real chip-doc corpus (AMBA core first) | [open](../tasks/CORPUS-HARDENING.md) |
| `CORPUS-PATTERN-REUSE` | `active` | reuse extraction patterns across PDFs, clustered by derived vendor/layout fingerprint | [open](../tasks/CORPUS-PATTERN-REUSE.md) |
| `CORPUS-TASK-EVIDENCE-CONTAINMENT` | `done` | keep the active corpus task bounded and lossless | [open](../tasks/CORPUS-TASK-EVIDENCE-CONTAINMENT.md) |
| `CVE-PROSE-EXTRACTION` | `done` | wire a live prose→ActorContract extractor into the R16 constrained-verified surface | [open](../tasks/CVE-PROSE-EXTRACTION.md) |
| `DECISION-RECORD-CAPACITY-HEADROOM` | `done` | restore room for durable architecture decisions | [open](../tasks/DECISION-RECORD-CAPACITY-HEADROOM.md) |
| `DEMPSTER-FUSION-COMBINER` | `done` | corroboration-boosting confidence fusion (Dempster's rule) | [open](../tasks/DEMPSTER-FUSION-COMBINER.md) |
| `DOC-INTENT-TAXONOMY` | `active` | chip-spec document intent taxonomy → per-category complete ISF synthesis | [open](../tasks/DOC-INTENT-TAXONOMY.md) |
| `DOCLING-DEVICE-CPU-DEFAULT` | `done` | make Docling ingest avoid the broken MPS auto-device | [open](../tasks/DOCLING-DEVICE-CPU-DEFAULT.md) |
| `DOCTRINE-ENFORCEMENT-ADOPT` | `done` | adopt the portable Doctrine-Enforcement architecture (4th standard) | [open](../tasks/DOCTRINE-ENFORCEMENT-ADOPT.md) |
| `EVAL-DOCUMENT-RECALL` | `done` | attribution-agnostic fact recall (the per-statement scorer under-counts) | [open](../tasks/EVAL-DOCUMENT-RECALL.md) |
| `EVAL-GOLD-INTERANNOTATOR-AGREEMENT` | `done` | is the eval answer-key trustworthy? (Cohen's κ) | [open](../tasks/EVAL-GOLD-INTERANNOTATOR-AGREEMENT.md) |
| `EVAL-RELATION-GRANULARITY` | `done` | per-relation-kind P/R/F1 + MUC near-miss diagnostic | [open](../tasks/EVAL-RELATION-GRANULARITY.md) |
| `EVIDENCE-DETERMINISM` | `done` | make the EvidenceIR build reproducible (no content-level non-determinism) | [open](../tasks/EVIDENCE-DETERMINISM.md) |
| `EVIDENCE-MATERIALIZE-IDEMPOTENCY` | `done` | don't accumulate stale facts on re-build | [open](../tasks/EVIDENCE-MATERIALIZE-IDEMPOTENCY.md) |
| `EXTRACTION-GAP-FIX` | `active` | close the extraction gaps PDF-VARIANT-DIGESTION.4 quantified | [open](../tasks/EXTRACTION-GAP-FIX.md) |
| `EXTRACTION-QUALITY-GAUGE` | `active` | measure the extraction-quality gap — and CHI's is large | [open](../tasks/EXTRACTION-QUALITY-GAUGE.md) |
| `EXTRACTOR-ARCHITECTURE` | `done` | make the EvidenceIR extractor path a coherent whole | [open](../tasks/EXTRACTOR-ARCHITECTURE.md) |
| `FACT-CARD-CAPACITY-HEADROOM` | `done` | restore headroom before the fact plane refuses new knowledge | [open](../tasks/FACT-CARD-CAPACITY-HEADROOM.md) |
| `FACT-CARD-CATALOG-CONTAINMENT` | `done` | keep fact-card browsing bounded before capacity fails | [open](../tasks/FACT-CARD-CATALOG-CONTAINMENT.md) |
| `FSMGEN-ASSERT-LOWERING` | `done` | lower stable / antecedent→consequent / min>1 obligations into the ISF verification family | [open](../tasks/FSMGEN-ASSERT-LOWERING.md) |
| `FSMGEN-ASSERT-MIGRATE` | `done` | re-pin to 43b29f5c + migrate `(contract … eventually …)` → `(assert (monitor …))` | [open](../tasks/FSMGEN-ASSERT-MIGRATE.md) |
| `FSMGEN-ISSUE-REPORTING` | `done` | File the FSMGen doc-vs-strict findings via the bundle protocol | [open](../tasks/FSMGEN-ISSUE-REPORTING.md) |
| `FSMGEN-LTL-MTL-SUGGESTION` | `done` | suggest first-class LTL/MTL temporal properties in ISF | [open](../tasks/FSMGEN-LTL-MTL-SUGGESTION.md) |
| `FSMGEN-MIN-WINDOW-CONFIRM` | `done` | answer FSMGen's `min > 1` window question | [open](../tasks/FSMGEN-MIN-WINDOW-CONFIRM.md) |
| `FSMGEN-REFRESH-INTEGRATE-2` | `done` | refresh the FSMGen submodule (2026-06) + re-assess adoptable ISF features | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-2.md) |
| `FSMGEN-REFRESH-INTEGRATE-3` | `done` | bump FSMGen submodule to the phase-membership-response tip | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-3.md) |
| `FSMGEN-REFRESH-INTEGRATE-4` | `done` | refresh the FSMGen pin + integrate FSMGen's answer to the field-structured-storage FR | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-4.md) |
| `FSMGEN-REFRESH-INTEGRATE-5` | `done` | refresh the FSMGen pin to the SHIPPED declarative storage fields + un-gate DOC-INTENT-TAXONOMY.4a.ii | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-5.md) |
| `FSMGEN-REFRESH-INTEGRATE-6` | `done` | refresh the FSMGen pin and integrate the current upstream contract | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-6.md) |
| `FSMGEN-REFRESH-INTEGRATE-7` | `done` | refresh the FSMGen pin and integrate the latest upstream contract | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-7.md) |
| `FSMGEN-REFRESH-INTEGRATE-8` | `done` | refresh the FSMGen pin and audit the new upstream delta | [open](../tasks/FSMGEN-REFRESH-INTEGRATE-8.md) |
| `FSMGEN-REFRESH-INTEGRATE` | `done` | refresh the FSMGen submodule + assess adoptable ISF features | [open](../tasks/FSMGEN-REFRESH-INTEGRATE.md) |

