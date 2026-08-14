# Knowledge questions — shard 0009

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > what is the difference between phases and transaction_phases
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > what is the difference between validate persistence and stage write_to_disk
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > what is the exact CORPUS-COVERAGE task evidence boundary
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is the exact SPEC-TO-INTENT-ALIGNMENT task evidence baseline
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is the exact active PDF task evidence baseline
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > what is the exact pre-containment FSMGEN_FEEDBACK source identity
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > what is the exact pre-containment ROADMAP source identity
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > what is the executable currentness authority for VALIDATION_SNAPSHOT
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > what is the faithful fix for an over-width ISF value literal (1: recover the signal's grounded width across ALL interface signal_records + actor_ports; 2: re-render the literal as a width-cast W'<radix><digits> when value<2^W, else residualize — never truncate; ADR-0006 numeric only)
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > what is the first SpecForge trajectory snapshot
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > what is the first opt-in extractor for CORPUS-PATTERN-REUSE.3b.3 / activate-only consume
- [source-to-intent-first-reviewed-result](../knowledge/source-to-intent-first-reviewed-result.md)
  > what is the first reviewed source-to-IntentIR evaluation result
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > what is the genuine lever for RISC-V CSR bit recall (a sharper VLM read for the existing recover-register-bits / register_bits.rs path — stronger/cloud model, upscaling, voting, tighter prompt — owned OUTSIDE the .4 ISF-lowering program; bound purely by VLM accuracy)
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > what is the header_rows trapped-data-row recovery in synthesize_timing_constraints
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > what is the ingest disk pre-flight check
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > what is the initiator-perspective direction emission / KG-ISF-COMPLETENESS.2a.ii
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > what is the intent_stale_relations_dropped / semantic_stale_relations_dropped finding
- [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md)
  > what is the inter-annotator agreement of the eval gold
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > what is the largest faithful-lowering gap in the IntentIR -> .isf round trip
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is the live-document coverage authority
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > what is the message_field_catalog_dump measurement harness
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > what is the message_fields.section_header_field strategy
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > what is the model-misspelled-subject / phantom-subject defect class
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > what is the must_be_value + VALID typed convention
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > what is the next big PDF-variant digestion lever after the serial class
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > what is the north-star bar #2 relation-completeness finding (KG-ISF-COMPLETENESS.3)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what is the only VLM-unique signal in AXI timing diagrams (phase ORDER, the .2h residual)
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > what is the only improvement path for conditional rules (upstream EXTRACTION — extract-constraints-llm / EXTRACTION-QUALITY-GAUGE recovering the concrete obligation from the conditional's source_text for the deontic-modal bucket; then it lowers via the existing (rule) path with no new ISF construct — lower-leverage than register/structure/topology)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what is the owner directive on transaction recognition / membership / step-by-step / fast / minimum
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > what is the parenthetical noun-phrase head rule / EXTRACTION-GAP-FIX.1
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > what is the per-document fingerprint made of (structural shape + extraction_manifest fired set)
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > what is the permission-vs-obligation gate / frame error class
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > what is the presence-matrix structural gate and how many tables fire it
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > what is the present blocking point on the specification-to-executable-intent path
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > what is the proposed Extractor framework (registry / driver / SurfacePolicy / run manifest)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what is the recommended fix (.5.i extraction-side fallback name-gate: derive_encoding_enum_name must return None unless the candidate token is a declared signal -> no enum minted; + emitter orphan-type fix isf_ir.rs:403-409 gate types block by emitted_enums(); .5.ii member-quality gate for the 271 real-named junk enums, calibration-gated)
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > what is the register-field eval surface (EvalTask::RegisterField)
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > what is the registers.section_header_field strategy
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > what is the rolling ledger archive protocol
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > what is the semantic_ungrounded_records_not_promoted residual packet
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what is the shipped ISF field-structured storage grammar ((storage (var NAME (width N) [(reset V)] (fields (field NAME (bits HI LO) [(access ...)] [(reset V)] [(enum ...)]))))) — metadata-only/schedule-safe, report key inferred_storage[].fields[])
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > what is the size-immunity binding constraint for source_ir.json at extreme page counts
- [spec-to-intent-category-contract](../knowledge/spec-to-intent-category-contract.md)
  > what is the source-to-IntentIR completeness contract
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > what is the standing per-doc quality report wired into converge/CI
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > what is the table-kind precision estimate and the flagged-mismatch list
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > what is the transaction phase qualifier authority rule
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what is the validate intent transaction phase-membership surface (transactions_with_phase_membership + transaction_phase_groups metrics + intent_transaction_phase_membership finding)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what is the validate transaction inventory surface (intent_transaction_inventory metrics + finding)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what is the validate transaction-phase inventory (semantic_transaction_phase_inventory metric + finding)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > what is the vendor/layout fingerprint, the ExtractionProfile, and the offline corpus pattern miner
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > what keeps VALIDATION_SNAPSHOT current without mutating generated artifacts
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > what kind of extractor actually needs the cross-document cluster mechanism
- [corpus-task-bounded-active-root-and-evidence-parts](../decisions/0024-corpus-task-bounded-active-root-and-evidence-parts.md)
  > what limits govern the corpus task root index parts and capsule
- [corpus-wide-interface-authority-rebuild](../knowledge/corpus-wide-interface-authority-rebuild.md)
  > what made 14 corpus documents stop emitting an .isf
- [behavioral-text-projection-boundary](../knowledge/behavioral-text-projection-boundary.md)
  > what makes a behavioral genericity run invalid unmeasurable or failed
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > what makes a corpus document unmeasurable for chain currency
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > what makes a source-to-Intent residual actionable
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > what makes a valid activate-only opt-in extractor candidate
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > what makes legacy absolute path rebasing safe and unambiguous
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > what message field metrics does validate emit
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > what model do converge / enrich / nlp-enrich use by default
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > what model does the NLI verifier use
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what must .5.iv.a exclude before header-sourced naming can land (four measured junk classes among the 134: OFFSET-headed register-offset tables where the header names a column concept not a field (3, CoreSight SDC-600); *_WIDTH self-named pseudo-enums whose only member is LEGAL_VALUES (the .5.iii honest residual, reappearing from the header side); garbled members (AXADDR ->
  > VA_40/NUM_2_0_A); and 12 RESERVED-only enums carrying no intent. It is byte-changing on the AXI wire gold ihi0022_l (a new AWATOP enum) so it needs the full before/after WIRE-BASED-100 protocol)
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > what must SpecForge derive to fully capture SWD; what are the gaps
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > what must be rebuilt after moving the SpecForge repository
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > what must happen before the USB4 inter-domain corpus refresh can run
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what owns containment of the live document adoption task history
- [live-document-containment-and-data-locality](../decisions/0007-live-document-containment-and-data-locality.md)
  > what owns live-document size limits and transition debt
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > what owns the SWD EvidenceIR to IntentIR projection gap
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what parts of the literature are deferred or flagged as future work
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what phases does each wire doc recognise (APB setup/access, AHB/AXI address/data, SWD address/data/response/turnaround)
- [spec-to-intent-category-contract](../knowledge/spec-to-intent-category-contract.md)
  > what precision and recall floors must a supported document category meet
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > what prevents one Knowledge Map question from pointing to multiple fact cards
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > what prevents root documentation from regrowing into a second manual
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > what prevents the actor direction and extract-contracts book claims from drifting
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > what prevents the word while in a license notice from becoming a gate
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > what produces the backslash in an EvidenceIR statement text
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > what protects the validation projection when LIVE_ACHIEVEMENT_STATUS rolls over
- [repository-local-scratch](../knowledge/repository-local-scratch.md)
  > what proved that analysis commands can violate project data locality
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > what real PDF proves the FigureRegion vertical path
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > what remains under-extracted in OpenCAPI Discovery Configuration
- [prior-memory-is-identity-independent](../decisions/0036-prior-memory-is-identity-independent.md)
  > what replaced ProtocolFamily in CorpusMemory schema 7
- [protocol-evidence-is-generic-and-document-derived](../decisions/0035-protocol-evidence-is-generic-and-document-derived.md)
  > what replaced the SWD-specific EvidenceIR carrier
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what research did SpecForge leave out and why
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > what reset_value shapes are composable vs residual (numeric dec/0x/0b/…h compose; UNKNOWN, IMPLEMENTATION DEFINED, 0x-------- partial-unknown, -, X, Impl Spec, Configuration dependent, enum-annotated are honest residuals — ADR-0006, no name list)
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > what retained chains changed in SPEC-TO-INTENT-ALIGNMENT.6d.i
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > what reviewed paraphrase and layout calibrations currently pass
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > what signals belong to which AXI channel
- [a-bounded-snapshot-needs-a-declared-repeatable-rollover](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
  > what stops a bounded snapshot from accreting chronology
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > what stops a later sentence or trailing agent phrase from supplying a passive constraint subject
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > what structural cue separates a message-field table from a register-field table
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what structural cues recognize transactions universally (section anchors + enumeration tables)
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > what structural surfaces discriminate document class
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > what table structure is required before port or pin vocabulary grants signal authority
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > what temporal logic backs temporal_rules
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > what validate metrics/findings carry the purpose category (document_intent_category, document_intent_category_confidence, evidence_document_intent_category finding)
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > what was the HBM2 canonical promotion pilot result
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > what was the must_be_value recall gap and how was it closed
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > when does a caption ground a register name
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > when does data (ACRONYM) declare a one-bit signal
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > when does evidence_extraction_quality_majority_not_entailed or _gauge_stale fire
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > when does presence capture refuse a row vs the whole table
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > when is a bracket-slice leading token a field name
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > when is a single letter a field name
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > when is the purpose category HIGH vs LOW confidence (only clean wire shape + self-declared guide are HIGH; everything else LOW + explicit residual)
- [spec-to-intent-category-contract](../knowledge/spec-to-intent-category-contract.md)
  > when may a semantic family or source modality be marked non-applicable
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > when should SpecForge request a new ISF or FSMGen construct
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > when should a live-document capacity bound be raised
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > when should message fields join the document class census
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > when should the activate-only ExtractionProfile consume contract be built
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > where are Docling models stored for SpecForge
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > where are Docling page sidecar paths normalized
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > where are administrative workflows filtered
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > where are behavioral paraphrase and harmless layout recipes registered
- [contested-priors](../knowledge/contested-priors.md)
  > where are cross-document prior contradictions surfaced
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > where are register bit-fields dropped on the way to .isf (isf_ir.rs:852, IsfStorageVar { name, width, reset } — field metadata discarded)
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > where are section-derived semantic phases built
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > where are the persisted trajectory controller input and report
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > where did SpecForge suggest LTL/MTL support in ISF
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > where did my signal constraint go between EvidenceIR and SemanticIR
- [llm-primary-constraint-dedup](../knowledge/llm-primary-constraint-dedup.md)
  > where did the AXI AWIDUNQ / WTAGUPDATE duplicate records go
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > where did the root architecture catalog go
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > where do CCIX PER error structure fields land in the IR
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > where do DBID / TxnID / ReturnNID style names come from in CHI
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > where do DTI message field obligations leak (signal_constraints) and how is it fixed
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > where do I log feedback or a suggestion to FSMGen
- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > where do TimingConstraintRecord table provenance ids live
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > where do in-memory structure layouts (queue entries, table entries, dwords) live in EvidenceIR
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > where do interface_edge_timings stop in the pipeline
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > where do live-document checker test fixtures create temporary files
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > where do message-field / structure / flit / descriptor layouts go in the pipeline (EvidenceIR only — no IntentIR carrier, never lowered)
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > where do obligations on message fields (TagOp must be 0) live in EvidenceIR
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > where do packet/flit message fields (TxnID / DBID / Opcode) live in EvidenceIR
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > where do the 9 new ACE wires AWBAR AWDOMAIN AWSNOOP come from and when do they land
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > where do the APB signal declarations come from (which table)
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > where do the signal / FSM / register / constraint / actor extractors live and how are they merged
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where do transaction phases live (SemanticIr.transaction_phases) and do they reach the .isf (no — SemanticIR-only, recognition only)
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > where do zero-port actors come from in the IntentIR actor surface
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > where does .cache/local-references/chipdoc resolve
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > where does .isf record dropped temporal obligations
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > where does EvidenceIR statement text come from
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > where does SpecForge canonicalize upstream artifact paths before serialization
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > where does SpecForge store temporary files and caches
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where does TransactionPhaseRecord.signal_set come from (build_transaction_phases, ir/semantic.rs — phase-naming statements' signals ∩ declared inventory, the .2c technique)
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > where does a register's byte offset come from when only the caption states it
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > where does nli-verify / extract-constraints-llm / semantic / intent / adapt write their output
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where does recognize_digital_patterns hardcode HTRANS/PSEL/MISO and why is it an ADR-0006 breach
