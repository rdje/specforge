# Knowledge questions — shard 0006

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > how is a claim's grounding checked beyond a string match
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > how is a completed oversized task tree contained without losing evidence
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how is a duplicate register mnemonic (AUTHSTATUS/CSW/IDR reused per access-port block) handled
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how is a fabricated mega-register / over-count avoided when recovering reused register mnemonics
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how is a frame field's width kept honest (why is ARBITRATION FIELD width None not 11)
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how is a fused Fields(Continued) caption handled
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > how is a fused contract's automation_confidence computed
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how is a guide reported so it is not a silent 0-yield extraction miss
- [task-evidence-route-catalogs-shard-by-lifecycle](../decisions/0046-task-evidence-route-catalogs-shard-by-lifecycle.md)
  > how is a leaf route lifecycle verified
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how is a message field's width kept honest (per-variant widths stay None)
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > how is a promoted constraint surface visible in the extraction manifest
- [claim-standard-upstream-readoption](../knowledge/claim-standard-upstream-readoption.md)
  > how is a published mechanism or causal account verified in SpecForge
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how is a register mnemonic reused across access-port blocks (AUTHSTATUS/CSW/IDR/DEVARCH/CLAIMSET) recovered instead of dropped
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how is a register name recovered from a section heading (RISC-V dmstatus/dmcontrol)
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how is a register-field mnemonic recovered when the name column is a bit-range (NVMe)
- [a-bounded-snapshot-needs-a-declared-repeatable-rollover](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
  > how is a roadmap rollover capsule verified
- [protocol-state-machine-binding](../knowledge/protocol-state-machine-binding.md)
  > how is a state machine identifier introduced generically
- [task-tree-node-forms](../knowledge/task-tree-node-forms.md)
  > how is a task-tree leaf written in docs/tasks (two forms: the absolute `- ID: `TREE.x` · Status: ...` line used by most trees, and a nested relative ` - `.x` · Status: ...` line used for children written inline under their parent, e.g. DOC-INTENT-TAXONOMY .3b/.3c under .3)
- [active-task-migration-transaction](../knowledge/active-task-migration-transaction.md)
  > how is a valid post-migration active task continuation verified
- [active-task-migration-transaction](../knowledge/active-task-migration-transaction.md)
  > how is an active task evidence migration written atomically
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > how is an active task source locked before migration
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > how is an external PDF labeled in Docling metadata
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how is an under-extracted spec distinguished from a true guide (evidence_document_underextracted_spec)
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > how is an unless/except exception clause handled in a temporal condition
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > how is copied artifact validation path containment tested
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > how is corpus SOURCE_PDF_REGISTRY currentness checked
- [corpus-kb-managed-currentness](../knowledge/corpus-kb-managed-currentness.md)
  > how is corpus_kb currentness checked
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > how is docs TASK_TREE kept complete without mirroring task history
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > how is docs knowledge INDEX kept complete
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > how is fact-plane capacity derived
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > how is max_cards derived from the knowledge_cards surface
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how is prose signal over-capture prevented (no garbage)
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how is published claim provenance mechanically gated
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > how is register-field extraction quality measured / scored
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > how is the .5.ii member-quality gate designed / what did the .5.ii calibration find (measured 2026-06-24 read-only over 78 docs/561 enums/12509 members: the gate is PER-MEMBER not per-enum — a whole-enum drop destroys AXI BRESP's real codes OKAY/EXOKAY/SLVERR/DECERR which are FUSED with prose fragments in one conflated enum; value-restart is NOT a junk signal — AHB HPROT
  > restarts but every member is a clean identifier. The load-bearing signal is per-member NAME shape: an English sentence-SPINE token marks a prose fragment. Land a per-member sentence-spine fragment drop at synthesize_encoding_declarations_for_enum)
- [prior-memory-is-identity-independent](../decisions/0036-prior-memory-is-identity-independent.md)
  > how is the CorpusMemory feedback loop made reproducible
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > how is the EvidenceIR extractor path / extraction layer structured and wired
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > how is the FSMGen feedback channel kept bounded without losing old requests and responses
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > how is the NLI gate tested without Ollama
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how is the SWD FSM/frame derivation scored (not constraints/relations/temporal) — four deterministic eval-extraction tasks read the EvidenceIR surfaces directly: serial_frame_field, protocol_operation (once named swd_operation), protocol_state, interface_edge_timing
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how is the SWDIO sampling and drive-change edge scored
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how is the agent-definition grammar kept garbage-free without a fragile noun denylist
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how is the composition-frame grammar kept free of corpus false positives
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > how is the corpus distributed across the document intent categories
- [decision-capacity-is-rederived-without-moving-stable-records](../decisions/0041-decision-capacity-is-rederived-without-moving-stable-records.md)
  > how is the decision-record file bound derived
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > how is the default SPECFORGE_INGEST_BATCH_THRESHOLD selected
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how is the figure-interior drop mechanism verified
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > how is the ingest DISK footprint bounded for very large PDFs
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > how is the live-document containment checker tested
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > how is the page-range batch size chosen / adapted
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how is the precision of the broadened (non-gold) extraction measured / estimated
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > how is the proof seal read from a large stage artifact
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > how is the prose definitional signal grammar kept garbage-free without a denylist (ADR 0006)
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > how is the protocol initiator actor identified structurally without a name list
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > how is the published current result kept in agreement with the reviewed gold
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
- [workflow-standard-capacity-is-rederived-from-explicit-member-growth](../decisions/0043-workflow-standard-capacity-is-rederived-from-explicit-member-growth.md)
  > how is workflow-standard file capacity derived
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > how large is the average docs/research record
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
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > how many alignment task owners and routes survive migration
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > how many arrow-form direction cells exist corpus-wide
- [book-behaviour-currency-instrument](../knowledge/book-behaviour-currency-instrument.md)
  > how many book claims describe behaviour the code no longer has
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > how many bracketed placeholder name cells exist corpus-wide
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > how many caption bindings does the corpus lose on re-ingest
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how many captions does ingest discard
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > how many captured figure regions does the retained corpus hold
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > how many conditional_rules are concretely lowerable (only ~3 of 603 across 9 representative docs carry a concrete value/level cue; 161/164 declared-consequent candidates are bare modals shall/must/shall not)
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how many converter text items does iterate_items yield across the corpus
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
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > how many corpus/-sourced documents are there and how many are legacy
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > how many current claim census candidates join exact evidence or registered annotations
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > how many current claim evidence units are derived registered incomplete or excluded
- [behavioral-text-projection-boundary](../knowledge/behavioral-text-projection-boundary.md)
  > how many current documents are behaviorally measurable
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > how many current emitted ISFs pass FSMGen after retiring fabricated priorities (44 of 44, zero diagnostics at pin a51dcdad0)
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > how many current governed Markdown surfaces are in the claim census
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > how many declared names are spelled in a case the document never uses
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > how many digests move when the reviewed projection changes
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
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > how many enumerated width cells exist corpus-wide
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > how many evidence artifacts have no validation report
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > how many fact cards can SpecForge hold
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > how many fact cards can SpecForge hold now
- [timing-scalar-rows-require-independent-cell-geometry](../knowledge/timing-scalar-rows-require-independent-cell-geometry.md)
  > how many false timing records were removed by the independent-cell geometry boundary (23; 608 to 585)
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > how many falsification controls are cited by verified SpecForge claims
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how many figure interior texts does the corpus hold
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > how many generated SourceIR records still name the old livework checkout
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > how many generated artifacts still mention the deleted boot-volume repository
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > how many governed claim producers are tracked
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > how many identifier co-references exist corpus-wide
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > how many lines may MEMORY.md be (50; the active_resume surface caps lines_each and lines_total at 50 and the memory-arch check fails the commit above it, so durable procedure belongs in a fact card or a task tree rather than in the resume pointer)
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > how many page objects does a /Type/Page regex report vs pdfinfo
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > how many pages elements and normalized files does the OpenCAPI data link ingest produce
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > how many persisted SourceIR artifacts are stale
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how many persisted artifacts have an ambiguous source_ref
- [alpha-variant-placeholder-is-not-a-wire](../knowledge/alpha-variant-placeholder-is-not-a-wire.md)
  > how many placeholder tokens exist corpus-wide
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > how many production rules does the registry declare
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > how many property tables are admitted as signal tables corpus-wide
- [protocol-state-machine-binding](../knowledge/protocol-state-machine-binding.md)
  > how many protocol states carry a machine name corpus-wide
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > how many public fields do SemanticIR and IntentIR declare
- [mdbook-quantitative-census-freeze](../knowledge/mdbook-quantitative-census-freeze.md)
  > how many quantitative prose candidates are in the SpecForge book
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > how many records does LIVE_ACHIEVEMENT_STATUS.md hold
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
