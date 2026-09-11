# Task tree catalog part 0002

> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with
> `perl scripts/check_task_tree_catalog.pl --write`.

Complete membership for this range. The bounded landing is
[`docs/TASK_TREE.md`](../TASK_TREE.md); it carries the open trees and routes here for the rest.

| Tree | Status | Purpose | File |
| --- | --- | --- | --- |
| `FULL-PAGE-INTENT-CAPTURE` | `done` | use the full scope of a page's visual information | [open](../tasks/FULL-PAGE-INTENT-CAPTURE.md) |
| `GRITS-CROSS-TOOL` | `done` | a table-structure gold from independent-witness agreement | [open](../tasks/GRITS-CROSS-TOOL.md) |
| `INTENT-COMPLETENESS-RESEARCH` | `done` | theory + design for detecting & bounding intent-capture misses | [open](../tasks/INTENT-COMPLETENESS-RESEARCH.md) |
| `ISF-HANDSHAKE-STAGE-LOWERING` | `superseded` | lower HandshakeComplete temporal_rules to `(stage …)` | [open](../tasks/ISF-HANDSHAKE-STAGE-LOWERING.md) |
| `ISF-ONLY-CONSOLIDATION` | `done` | Drop HDL + `.fsm` adapters; SpecForge emits only `.isf` | [open](../tasks/ISF-ONLY-CONSOLIDATION.md) |
| `ISF-ONLY-IR-PRUNE` | `done` | remove genuinely `.fsm`-era orphaned IR surfaces | [open](../tasks/ISF-ONLY-IR-PRUNE.md) |
| `ISF-REGISTER-RESET-EMIT` | `done` | lower extracted register reset values into the ISF `(storage (var … (reset V)))` surface | [open](../tasks/ISF-REGISTER-RESET-EMIT.md) |
| `ISF-RULE-CONFLICT-RESIDUAL` | `done` | surface dropped value-conflicting rules as explicit residuals (not silent loss) | [open](../tasks/ISF-RULE-CONFLICT-RESIDUAL.md) |
| `ISF-SYMBOL-COUNT-EMITTED` | `done` | make the adapter `constant_count`/`enum_count` reflect emitted content | [open](../tasks/ISF-SYMBOL-COUNT-EMITTED.md) |
| `ISF-SYMBOL-SURFACE-EMIT` | `done` | emit the built-but-discarded `(constants)`/`(types)`/`(enums)` ISF surface | [open](../tasks/ISF-SYMBOL-SURFACE-EMIT.md) |
| `ISF-TEMPORAL-LOWERING` | `done` | lower IntentIR.temporal_rules into the `.isf` adapter | [open](../tasks/ISF-TEMPORAL-LOWERING.md) |
| `ISF-TXN-GRAMMAR-FIX` | `done` | correct SpecForge's `.isf` transaction-step emitter grammar | [open](../tasks/ISF-TXN-GRAMMAR-FIX.md) |
| `ISF-VALUE-WIDTH-EMIT` | `done` | width-align ISF value literals to the declared signal width | [open](../tasks/ISF-VALUE-WIDTH-EMIT.md) |
| `KG-ISF-COMPLETENESS` | `active` | the KG/IntentIR must be COMPLETE enough to lower faithfully to ISF | [open](../tasks/KG-ISF-COMPLETENESS.md) |
| `KG-ISF-TRANSACTIONS` | `active` | every supported protocol transaction + its signals, fully captured & ISF-ready | [open](../tasks/KG-ISF-TRANSACTIONS.md) |
| `KNOWLEDGE-MAP-ADOPTION` | `done` | adopt the portable Knowledge Map retrieval layer | [open](../tasks/KNOWLEDGE-MAP-ADOPTION.md) |
| `LITERATURE-GROUNDING` | `done` | ground every SpecForge aspect in published research | [open](../tasks/LITERATURE-GROUNDING.md) |
| `LIVE-DOC-STOP-RISK` | `done` | find the live-document stops that no current signal makes actionable | [open](../tasks/LIVE-DOC-STOP-RISK.md) |
| `LIVE-DOCUMENT-PRESSURE-HEADROOM` | `active` | keep current-facing canonical surfaces writable | [open](../tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md) |
| `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION` | `done` | bounded live docs, stable README, and same-volume project data | [open](../tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md) |
| `LLM-EXTRACTION-EVAL` | `done` | a labeled precision/recall eval set for the LLM extraction passes | [open](../tasks/LLM-EXTRACTION-EVAL.md) |
| `LLM-PRIMARY-PROMOTION` | `done` | promote the LLM-primary constraint surface into the canonical pipeline | [open](../tasks/LLM-PRIMARY-PROMOTION.md) |
| `LLM-TEXT-TRANSPORT-DEDUP` | `done` | consolidate the duplicated text chat transport (extract-contracts + signal-resolve) | [open](../tasks/LLM-TEXT-TRANSPORT-DEDUP.md) |
| `LOGIC-LEVEL-BOUNDARY` | `done` | resolve HIGH/LOW as universal "how"; centralize the vocabulary | [open](../tasks/LOGIC-LEVEL-BOUNDARY.md) |
| `MDBOOK-DOCTEST-HYGIENE` | `done` | classify examples and make the live book doctest-safe | [open](../tasks/MDBOOK-DOCTEST-HYGIENE.md) |
| `MEASUREMENT-PLANE-CONVERGENCE-RISK` | `superseded` | can we tell whether SpecForge is converging? | [open](../tasks/MEASUREMENT-PLANE-CONVERGENCE-RISK.md) |
| `MEMORY-ARCHITECTURE-DOC` | `done` | author a portable, harness-agnostic durable-memory standard | [open](../tasks/MEMORY-ARCHITECTURE-DOC.md) |
| `MEMORY-BOUNDED-INGEST` | `active` | bounded-memory ingestion of very large PDFs | [open](../tasks/MEMORY-BOUNDED-INGEST.md) |
| `MEMORY-RESUME-POINTER-BYTE-CAP` | `done` | set the durable resume pointer's one-read ceiling | [open](../tasks/MEMORY-RESUME-POINTER-BYTE-CAP.md) |
| `NLI-CLAIM-CONDITION` | `done` | carry a constraint's condition into its NLI claim (precision fix) | [open](../tasks/NLI-CLAIM-CONDITION.md) |
| `NLI-CLAIM-CONNECTOR` | `done` | join the condition clause with a connective so claims read as English | [open](../tasks/NLI-CLAIM-CONNECTOR.md) |
| `NLI-ENTAILMENT-VERIFIER` | `done` | a semantic "does the source actually say this?" gate | [open](../tasks/NLI-ENTAILMENT-VERIFIER.md) |
| `NLI-GATE-METRIC` | `done` | surface the NLI gate's demotions as a validate metric | [open](../tasks/NLI-GATE-METRIC.md) |
| `NLI-INTENT-GATE` | `done` | make the NLI verifier an active IntentIR gate (demote NotEntailed → residual) | [open](../tasks/NLI-INTENT-GATE.md) |
| `NLP-ENRICH-TRANSPORT-DEDUP` | `done` | complete the text-transport consolidation (route nlp_enrich through llm_text) | [open](../tasks/NLP-ENRICH-TRANSPORT-DEDUP.md) |
| `NLP-SHALLOW-PARSE` | `active` | a deterministic in-Rust shallow-parse tier (subject–verb–object understanding) | [open](../tasks/NLP-SHALLOW-PARSE.md) |
| `PDF-AGNOSTIC-EXTRACTION` | `done` | remove all hardcoded chip-spec vocabulary; derive from the document | [open](../tasks/PDF-AGNOSTIC-EXTRACTION.md) |
| `PDF-VARIANT-DIGESTION` | `active` | make SpecForge digest as many chip-spec PDF variants as possible | [open](../tasks/PDF-VARIANT-DIGESTION.md) |
| `PER-EXTRACTOR-FACT-TAGGING` | `done` | record which extractor found each fact (recall-gauge precondition) | [open](../tasks/PER-EXTRACTOR-FACT-TAGGING.md) |
| `PRIOR-DECAY` | `done` | detect & flag contested priors (revision-on-contradiction for CorpusMemory) | [open](../tasks/PRIOR-DECAY.md) |
| `PROVENANCE-HARDENING` | `done` | Test Assertion Coverage For Provenance-Like IR Fields | [open](../tasks/PROVENANCE-HARDENING.md) |
| `PROVIDER-MODEL-STORE-LOCALITY` | `active` | decide and gate where the VLM/NLP model store lives | [open](../tasks/PROVIDER-MODEL-STORE-LOCALITY.md) |
| `PURE-NLP-INTENT-EXTRACTION` | `active` | model-based intent extraction (ACTIVE — first increment) | [open](../tasks/PURE-NLP-INTENT-EXTRACTION.md) |
| `R1-R5-FOUNDATION-BACKFILL` | `done` | own + audit the foundational pipeline milestones (delivered pre-task-tree-system) | [open](../tasks/R1-R5-FOUNDATION-BACKFILL.md) |
| `R14-SIGNAL-RESOLVE` | `done` | Tier-3 LLM `signal_relation` extraction (the `signal-resolve` command) | [open](../tasks/R14-SIGNAL-RESOLVE.md) |
| `R15-GRAPH-DIRECTION-MIGRATION` | `done` | Complete actor-relative graph direction migration | [open](../tasks/R15-GRAPH-DIRECTION-MIGRATION.md) |
| `R15C-CONVERGENCE-REPORT` | `done` | make the convergent extraction loop first-class + inspectable | [open](../tasks/R15C-CONVERGENCE-REPORT.md) |
| `R15C-R15G-LEARNING-PLANE-BACKFILL` | `active` | own + audit the learning / eval / corpus lanes (in-progress) | [open](../tasks/R15C-R15G-LEARNING-PLANE-BACKFILL.md) |
| `R16-CAPTURE-FIDELITY-GATES` | `done` | objective capture-fidelity metric (point #5) | [open](../tasks/R16-CAPTURE-FIDELITY-GATES.md) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` | `done` | schema-constrained + verified extraction (point #6 — the crux, continuous) | [open](../tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md) |
| `R16-CONTRACT-IR` | `done` | typed timed-contract IR layer (point #1) | [open](../tasks/R16-CONTRACT-IR.md) |
| `R16-INTENT-CAPTURE` | `done` | SOTA design-intent capture (program umbrella) | [open](../tasks/R16-INTENT-CAPTURE.md) |
| `R16-KG-PROTOCOL-ONTOLOGY` | `done` | protocol-structured knowledge graph (point #2) | [open](../tasks/R16-KG-PROTOCOL-ONTOLOGY.md) |
| `R16-MODULE-HARDENING` | `done` | unit-test signoff hardening for the R16 IR modules | [open](../tasks/R16-MODULE-HARDENING.md) |
| `R16-MULTIMODAL-CONTRACT-FUSION` | `done` | cross-modal evidence → one contract (point #3) | [open](../tasks/R16-MULTIMODAL-CONTRACT-FUSION.md) |
| `R16-WAVEFORM-CONTRACT-MINING` | `done` | timing diagram → contract (point #4 — the crux) | [open](../tasks/R16-WAVEFORM-CONTRACT-MINING.md) |

