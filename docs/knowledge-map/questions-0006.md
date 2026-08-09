# Knowledge questions — shard 0006

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > what is is_condition_only_subject / conditional_clause_spans
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > what is is_permissive_only_subject_frame and why is it sentence-scoped not block-scoped
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > what is promotion_status not_promoted_review_required and where does the canonical mutation live
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > what is recover-register-bits / how does the recover-register-bits command work
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > what is recovered_trapped_data_rows and who shares it
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > what is register_field_name_recall / register_field_completeness / register_bit_structure_recall
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what is scripts/check_doctrines.sh / the doctrine driver
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > what is select_initiator_actor / initiator_perspective_directions
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > what is signal_presence_records and what does a SignalPresenceRecord hold
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > what is snap_subject_to_sentence_token and when does it fire
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > what is synthesize_register_field_tables
- [prose-pin-appositive-signal-capture](../knowledge/prose-pin-appositive-signal-capture.md)
  > what is synthesize_signal_declarations_from_prose / the pin-appositive pattern
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what is the .2m candidate (deterministic AXI-family channel-membership lever)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what is the .5.ii sentence-spine member-fragment predicate (a synthesized enum member_name is a prose fragment if any _-token is an English sentence-spine word — copula/aux/modal IS/ARE/BE/HAS/MUST/SHALL, article/demonstrative THE/THIS/THAT, relativizer/subordinator WHICH/WHEN/IF/BECAUSE — EXCLUDING the .1a collisions A/I/ITS/CAN/MAY/AM. Precision 1.000 (0/115 clean-anchor
  > flagged), recall 1.000 (269/269 junk-anchor caught), 30.2% of members drop; universal grammar ADR-0006, no name list)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what is the .5.iii _WIDTH parameter-leak gate / is it landed / is it ADR-0006 safe (LANDED 2026-06-24: is_width_parameter_leak_member + a continue-skip in synthesize_encoding_declarations_for_enum after the .5.ii spine gate, known_signals threaded from the signal-match caller. Drops a synthesized encoding member named <X>_WIDTH iff X is a declared signal OR the enum's own name
  > — document-grounded like .5.i, NOT a name list; corpus FP set EMPTY: no legit FULL_WIDTH/HALF_WIDTH value exists and the declared-signal arm never catches one since FULL/HALF are not signals; per-member not per-enum so BRESP keeps its codes and RRESP/AXSNOOP empty to honest residuals. AXI manager.isf now (BRESP (OKAY 0)(EXOKAY 1)...) + (AWCMO (CLEAN_AND_INVALIDATE
  > 0)(CLEAN_ONLY 1)); false RRESP/AXSNOOP/RCHUNK* _WIDTH enums gone; FSMGen --strict success/0; WIRE-BASED-100 1.000 before==after; kg-bench 156/156; run_ci GREEN lib 1718 +2)
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what is the 4th portable architecture (doctrine enforcement)
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > what is the AXI signal naming convention (channel prefix)
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > what is the DOC-INTENT-TAXONOMY.1 corpus census
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what is the DOC-INTENT-TAXONOMY.2 per-category ISF-completeness gauge
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > what is the Dempster combiner in fusion
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > what is the EXTRACTION-QUALITY-GAUGE.FIELD design
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > what is the ExtractionProfilePriorRecord 8th prior family / extraction_profile_priors in CorpusMemory
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what is the FSMGen feature request for field-structured storage (declarative (var NAME (width N) (fields (field NAME (bits hi lo) (access ..) (reset ..) (enum ..)))); docs/FSMGEN_FEEDBACK.md 2026-06-22)
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > what is the FSMGen issue bundle protocol
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > what is the I2C declared-signal recall / precision
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > what is the KG-ISF-COMPLETENESS.1a agent-identity / actor precision gate
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > what is the KG-ISF-COMPLETENESS.1b.i trailing-fragment consolidation
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > what is the KG-ISF-COMPLETENESS.1b.ii named-interface consolidation
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > what is the KG-ISF-COMPLETENESS.1b.iii coordinated-subject split
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > what is the KG-ISF-COMPLETENESS.1b.iv pure-inferred phantom drop
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > what is the KG-ISF-COMPLETENESS.1c.i trailing preposition/auxiliary strip
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what is the KG-ISF-TRANSACTIONS census / transaction-capture baseline
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > what is the LLM-primary extractor's measured precision on APB / AHB / AXI gold
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > what is the LLM-primary extractor's measured recall on APB / AHB / AXI gold
- [temporal-rule-ltl-rendering](../knowledge/temporal-rule-ltl-rendering.md)
  > what is the LTL form of a temporal_rule
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > what is the NLI entailment verifier
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > what is the NVMe register-field recall / precision
- [corpus-coverage-sweep](../knowledge/corpus-coverage-sweep.md)
  > what is the PDF-VARIANT-DIGESTION whole-corpus coverage / re-triage
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what is the PHASE_NAME_STOPWORDS gate and why is it stronger than the anchor gate (prose is noisier)
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > what is the RAM-safe per-doc protocol for the CANONICAL-PROMOTION-SWEEP
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > what is the RISC-V Debug register-field recall / precision
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > what is the SWD line state machine (reset/operating/protocol-error/lockout)
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > what is the VLM table strategy / PDF-VARIANT-DIGESTION.2b
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > what is the `<role> channel signals` caption cue and how is it parsed
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > what is the ambiguous_statements metric in validate
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > what is the authoritative membership set for corpus SOURCE_PDF_REGISTRY
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > what is the bit-exact adjacency chain rule
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > what is the buildable category-4 lever (.4d.i — recover RISC-V CSR field bit positions + a RISC-V-shaped register recogniser for AIA; once located, fields auto-lower via .4a.ii, no emitter change)
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > what is the buildable lever for cat-3 topology if pursued (upstream EXTRACTION-RECALL owned OUTSIDE the .4 ISF-lowering program — denser+fully-connected signal_connectivity capture from TRM integration prose/diagrams + clock/reset source resolution; mirrors .4d.i cat-4 CSR recovery and the cat-2 structure-recall frontier; recorded as a cross-reference, NOT a .4 gap)
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what is the built-in RAM guard / autonomous memory safeguard during ingest
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > what is the canonical declared signal inventory key on SemanticIR
- [knowledge-map-architecture-location](../knowledge/knowledge-map-architecture-location.md)
  > what is the canonical knowledge-map architecture path
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what is the completeness gauge over-counting on APB
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > what is the condition-read-as-obligation error class and its gate
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > what is the constraint-subject-must-be-declared filter
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > what is the corpus coverage build-out (CORPUS-COVERAGE.0)
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > what is the corpus distribution of purpose categories (21 wire / 8 guide high; 28 register-or-platform / 16 unresolved / 5 physical-link low; 0 high-confidence false positives)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > what is the corpus impact of the authority empty interface fallback
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > what is the declared-signal eval surface / EvalTask::DeclaredSignal
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > what is the difference between a refreshed EvidenceIR and a retained normalized bundle
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > what is the difference between validate persistence and stage write_to_disk
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
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > what is the first opt-in extractor for CORPUS-PATTERN-REUSE.3b.3 / activate-only consume
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
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what is the shipped ISF field-structured storage grammar ((storage (var NAME (width N) [(reset V)] (fields (field NAME (bits HI LO) [(access ...)] [(reset V)] [(enum ...)]))))) — metadata-only/schedule-safe, report key inferred_storage[].fields[])
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > what is the size-immunity binding constraint for source_ir.json at extreme page counts
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > what is the standing per-doc quality report wired into converge/CI
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > what is the table-kind precision estimate and the flagged-mismatch list
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
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > what prevents one Knowledge Map question from pointing to multiple fact cards
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > what prevents root documentation from regrowing into a second manual
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > what prevents the actor direction and extract-contracts book claims from drifting
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > what prevents the word while in a license notice from becoming a gate
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > what protects the validation projection when LIVE_ACHIEVEMENT_STATUS rolls over
- [repository-local-scratch](../knowledge/repository-local-scratch.md)
  > what proved that analysis commands can violate project data locality
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what research did SpecForge leave out and why
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > what reset_value shapes are composable vs residual (numeric dec/0x/0b/…h compose; UNKNOWN, IMPLEMENTATION DEFINED, 0x-------- partial-unknown, -, X, Impl Spec, Configuration dependent, enum-annotated are honest residuals — ADR-0006, no name list)
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > what signals belong to which AXI channel
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
- [contested-priors](../knowledge/contested-priors.md)
  > where are cross-document prior contradictions surfaced
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > where are register bit-fields dropped on the way to .isf (isf_ir.rs:852, IsfStorageVar { name, width, reset } — field metadata discarded)
