# Knowledge questions — shard 0005

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how is the composition-frame grammar kept free of corpus false positives
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > how is the corpus distributed across the document intent categories
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > how is the default SPECFORGE_INGEST_BATCH_THRESHOLD selected
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > how is the ingest DISK footprint bounded for very large PDFs
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > how is the live-document containment checker tested
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > how is the page-range batch size chosen / adapted
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how is the precision of the broadened (non-gold) extraction measured / estimated
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > how is the prose definitional signal grammar kept garbage-free without a denylist (ADR 0006)
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > how is the protocol initiator actor identified structurally without a name list
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how is the purpose category different from document_class (richer 6-way semantic taxonomy vs coarse 4-way structural proxy; consumes document_class as one input, never replaces it)
- [agnostic-quoted-mode-fsm](../knowledge/agnostic-quoted-mode-fsm.md)
  > how is the quoted-mode FSM extractor kept agnostic and false-positive-free (ADR 0006)
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > how is the required free disk for an ingest estimated
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > how is the single-word `<NAME> state` grammar kept false-positive-free without a keyword doc-gate (ADR 0006)
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > how is the terminal task source archive boundary verified
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > how is transaction membership kept boundary-precise across read vs write (bar #3)
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > how many AArch64 External Debug page sidecars are repository relative
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > how many CORPUS-COVERAGE re-ingests remain after normalized cleanup
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > how many CoreSight Base System page sidecars are repository relative
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > how many Introducing CoreSight page paths are repository relative
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > how many OpenCAPI AFU address note page paths are repository relative
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > how many OpenCAPI Certified Definition paths are repository relative
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > how many OpenCAPI Ready Definition paths are repository relative
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > how many OpenCAPI Ready page paths are repository relative
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > how many RISC-V Debug register bit diagrams are images vs flattened tables (53/56 images, 34 field tables, only 7 flattened diagram tables; bits live in the image modality)
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > how many SpecForge mdBook doctests currently fail
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > how many USB4 Connection Manager page sidecars are repository relative
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > how many USB4 Inter-Domain page sidecars are repository relative
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > how many administrative workflow statements were measured
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > how many conditional_rules are concretely lowerable (only ~3 of 603 across 9 representative docs carry a concrete value/level cue; 161/164 declared-consequent candidates are bare modals shall/must/shall not)
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > how many corpus documents carry markdown-escaped underscores in their statements
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > how many corpus refreshes are done and how many remain (52 of 57 done and five remaining as of 2026-08-11; re-run the gate rather than trusting an older count)
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > how many corpus refreshes remain after AArch64 External Debug
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > how many corpus refreshes remain after CoreSight Base System
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > how many corpus refreshes remain after Introducing CoreSight
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > how many corpus refreshes remain after OpenCAPI AFU Address Space Usage
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > how many corpus refreshes remain after OpenCAPI Certified Definition
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > how many corpus refreshes remain after OpenCAPI Discovery
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > how many corpus refreshes remain after OpenCAPI Ready Definition
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > how many corpus refreshes remain after USB4 Connection Manager
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > how many corpus refreshes remain after USB4 Inter-Domain
- [gic-overview-guide-refresh-is-authority-empty](../knowledge/gic-overview-guide-refresh-is-authority-empty.md)
  > how many corpus refreshes remain after the GIC Overview Guide
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > how many corpus refreshes remain after the OpenCAPI Certified note
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > how many corpus refreshes remain after the OpenCAPI Ready note
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > how many corpus task ids and source regions must containment preserve
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > how many current emitted ISFs pass FSMGen after retiring fabricated priorities (44 of 44, zero diagnostics at pin a51dcdad0)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > how many docs / enums are affected (56/78 docs carry a generic-named enum; 96 generic vs 493 real; but a name-only gate misses 271 real-named-but-junk fragment/dup enums — the real defect is member quality)
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > how many docs are wire-protocol vs register-IP vs platform vs ISA vs PHY vs guide
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > how many docs reach evidence vs semantic vs intent vs isf
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > how many documents carry unfiltered conditional rules and signal constraints
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > how many documents lost promoted records when the empty-catalog special case was deleted
- [corpus-wide-interface-authority-rebuild](../knowledge/corpus-wide-interface-authority-rebuild.md)
  > how many emitted .isf files does the corpus have and are they FSMGen-strict clean
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > how many evidence artifacts have no validation report
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > how many fact cards can SpecForge hold
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > how many fact cards can SpecForge hold now
- [timing-scalar-rows-require-independent-cell-geometry](../knowledge/timing-scalar-rows-require-independent-cell-geometry.md)
  > how many false timing records were removed by the independent-cell geometry boundary (23; 608 to 585)
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > how many generated SourceIR records still name the old livework checkout
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > how many generated artifacts still mention the deleted boot-volume repository
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > how many page objects does a /Type/Page regex report vs pdfinfo
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > how many pages elements and normalized files does the OpenCAPI data link ingest produce
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > how many register bit-fields fail to lower to .isf (12,638 fields across 32 docs — the largest measurable intent-loss; DOC-INTENT-TAXONOMY.2)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > how many registers can SpecForge compose a faithful ISF reset for (1508 strictly composable corpus-wide; 1339 fit the current emit width; 446 have V>0 — the real .isf diff)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > how many retained documents depended entirely on heuristic SemanticIR interfaces
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > how many retained legal semantic gates were measured
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > how many retained phases depended only on sequencing words
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > how many retained typed transaction phases were false positives
- [source-to-intent-reviewed-population](../knowledge/source-to-intent-reviewed-population.md)
  > how many reviewed documents are locked per source category
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > how many reviewed documents have current binary replay evidence
- [source-to-intent-first-reviewed-result](../knowledge/source-to-intent-first-reviewed-result.md)
  > how many reviewed source regions and modalities were found
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > how many sentence-start signal descriptor phrases pollute the retained corpus
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > how many signals does each AXI channel have
- [source-to-intent-reviewed-population](../knowledge/source-to-intent-reviewed-population.md)
  > how many source-to-intent evaluation inputs are repository sources versus external read-only sources
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > how many synthetic behaviors came from generic gates
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > how many synthetic behaviors came from generic section phases
- [opencapi-32g-phy-timing-without-interface-topology](../knowledge/opencapi-32g-phy-timing-without-interface-topology.md)
  > how many timing constraints remain in OpenCAPI 32G PHY Signaling after refresh (60)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > how many transactions does each persisted IntentIR doc have (AXI=9, AHB=3, APB=3)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > how many unknown-kind tables does the corpus carry
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > how much RAM did the qwen2.5vl:7b VLM use on a timing-diagram crop (13 GB; host hit 87% used, across the 85% kill threshold)
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > how much disk do normalized bundles cost across the corpus
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > how much disk do the retained normalized bundles cost
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > how much fact-card catalog index headroom remains
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > how much headroom remains in the corpus coverage task file
- [corpus-coverage-sweep](../knowledge/corpus-coverage-sweep.md)
  > how much intent does SpecForge extract across the whole corpus
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > how much memory did the guarded AArch64 External Debug ingest use
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > how much memory did the guarded CoreSight Base System ingest use
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > how much memory did the guarded Introducing CoreSight ingest use
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > how much memory did the guarded OpenCAPI AFU address note ingest use
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > how much memory did the guarded OpenCAPI Certified Definition ingest use
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > how much memory did the guarded OpenCAPI Ready Definition ingest use
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > how much memory did the guarded OpenCAPI Ready ingest use
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > how much memory did the guarded USB4 Connection Manager ingest use
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > how much memory did the guarded USB4 Inter-Domain ingest use
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > how much of the reviewed trajectory population has current binary replay evidence
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > how much page content escapes both the structured-element path and the region-crop path
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > how should Rust version copies be verified across Cargo README book and CI
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > how should a behavior-preserving evidence refactor be verified given the non-determinism
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > how should an AXI gold or extraction be structured (per channel)
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > how should the agent-identity / actor noise gate stay agnostic (no name list, ADR 0006)
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > how should the current FSMGen gitlink in documentation be verified
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > how to compose a register-level ISF reset from per-field reset_value (LSB-tiling: V = OR(parse_int(reset_i) << bits_low_i), mirroring ir/register_bits.rs; only when every field is located + parseable non-neg int fitting its field width + no overlap)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how to exploit that same-vendor / same-brand PDFs share organization without hardcoding vendor names
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > how to express the JTAG TAP / SWD FSM in .isf
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > how to get a fresh eval-extraction baseline for a spec
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how to re-measure the message-field corpus yield
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how to re-score SWD derivation
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > how to read a chip-spec PDF when the Read tool refuses it
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > how to recover register field bit positions that live in the layout graphic, not the table
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > how was AHB HREADY recovered for the temporal antecedent
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > how was AXI constraint precision fixed
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > how was the 400 page Docling SIGKILL reproduced
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > how was the CHI field-constraint routing measured without re-ingesting the PDF
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > how was the agent-identity gate proven not to drop real agents (>=8-port proxy, WIRE-BASED-100)
- [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md)
  > how was the eval gold checked for idiosyncrasy
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > how was the trailing aux/prep strip proven safe for WIRE-BASED-100 and real agents
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > how was the trailing-fragment consolidation proven not to regress real agents (WIRE-BASED-100)
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > how were I2S timing_constraints recovered (clock period / clock HIGH / set-up / hold)
- [trajectory-steering-is-a-reviewable-multimetric-control-loop](../decisions/0034-trajectory-steering-is-a-reviewable-multimetric-control-loop.md)
  > how will SpecForge automatically detect convergence or divergence
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > how will the million-byte Knowledge Map be sharded without losing question retrieval
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > how will the oversized SpecForge roadmap be made bounded without losing its history
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > how will the shared rolling ledger archive index be partitioned
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is 'X, which connects to Y, drives Z and W' clause distribution handled
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is (sample input as name) value-free in ISF (yes; (drive input) is rejected — drives exist only for outputs)
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > is AArch64 External Debug a methodology guide or under-extracted architecture
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > is AXI/SWD per-signal phase membership recoverable from timing diagrams via a VLM
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > is CORPUS-PATTERN-REUSE.3b.3a a go or no-go
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > is DOC-INTENT-TAXONOMY.4a.ii buildable now (DONE 2026-06-22 — implemented once FSMGen shipped the construct; superseded .4a.i; Gap B packet/flit still deferred)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > is ISF a single-actor or multi-actor format (per-actor — one .isf describes one actor/module; SpecForge's emit collapses to one initiator via select_initiator_actor; lowering cross-component topology would need a multi-actor emit, an architectural change not an emitter tweak)
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > is ISF the current SpecForge bottleneck
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is KG-ISF-TRANSACTIONS.2i unparked / what is the .2i decision (yes — FSMGen confirmed option a: ship the grounded per-phase membership grouping as IntentIR metadata, .isf byte-identical)
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > is RISC-V Debug register bit-position recovery a deterministic text-table parse or a VLM-image read (VLM-image — 53/56 diagrams are images, the 7 flattened tables are garbled/symbolic; deterministic parse would fabricate)
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > is SPECFORGE_INGEST_BATCH_PAGES a fixed size or a ceiling
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > is SWD at 100% and on what metric
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > is SpecForge converging diverging stalled mixed or unmeasurable
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > is SpecForge specification mining
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > is SpecForge still allowed to read chipdoc from the boot volume
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > is SpecForge's constraint extraction over-generating
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > is VALIDATION_SNAPSHOT the latest local artifact validation or the last reviewed projection
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > is a better VLM needed to read register bit-layout diagrams
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > is a greater than 500 page threshold sufficient for bounded PDF ingestion
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is a name-only gate enough to fix the generic enum (no — 271 real-named enums like COMMAND/DWORD_MISR/AMBA are themselves fragment-heavy/dup-heavy; the load-bearing signal is member quality)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is a transaction's phase ORDER recoverable from prose (no — first-occurrence wrong on SWD, cues sparse, precedence conflicting)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is a value-less drive valid in ISF / can an output participate in a transaction body without a value (no — every (drive …) needs a concrete actual; raised to FSMGEN)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is an ISF transaction body ordered (yes — totally ordered, one clause ≈ one cycle; same-cycle concurrency only via a multi-pair drive block)
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > is any corpus document ingested at fewer pages than its source PDF
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > is bar #5 (no silent drop of behavior/temporal) still honest after CORPUS-COVERAGE.0
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > is bar #6 (ISF round-trip, no silent drop) a large faithful-lowering gap
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > is collect_known_signal_names UTF-8 safe
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > is converge the complete production capability path
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > is dropping a zero-evidence actor safe across AXI vs AXI-Stream
