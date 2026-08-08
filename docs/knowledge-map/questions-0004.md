# Knowledge questions — shard 0004

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what are the two dominant ISF-lowering completeness gaps (register bit-fields, message-field structures)
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > what are the undeclared-named-subject rule drops at ISF lowering (conditional_rules / signal_constraints / temporal_invariants)
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what blocks a Rust code change from committing in specforge
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what carries register bit-fields in SpecForge (RegisterFieldRecord in source.rs:414; IntentIr.register_records clone at intent.rs:193 — full metadata survives to IntentIR)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what carries transaction membership faithfully instead of the body (IntentIR metadata: ports / phase_membership / channel_membership)
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > what causes actor_signal_relations / extracted_statements to differ run-to-run
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > what confidence axis correlates with extracted-constraint correctness
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what debug/diagnostic tools does specforge have (TOOLBOX.md)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did FSMGEN answer about transaction phase membership (don't fabricate value or order; keep value-less participation + unordered membership as IntentIR metadata/residual not body steps; checked phase-group metadata is the future ISF shape on its own FSMGen tree; .isf stays source of truth, no .val)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what did KG-ISF-TRANSACTIONS.2l measure / decide
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what did KG-ISF-TRANSACTIONS.2n measure / decide
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > what did running nli-verify on a real spec find
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did the .2i Rule-A per-phase grouping measurement find (clean only on AHB, empty on APB/AXI/SWD)
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what did the corpus promotion sweep measure (gauge deltas per doc)
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > what distinguishes a PURE-INFERRED phantom from a PROSE-GROUNDED or SECTION+INFERRED 0/0 actor
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > what do leading_section_number and is_descendant_section_number do
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > what document class is a chip-spec PDF (protocol / register / interface / guide)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what does (on start (sample S as s)) assert in FSMGen semantics (an entry-cycle D-input capture, cycle N port && can_accept)
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > what does .10h do that .10g did not (block-qualified register-mnemonic recovery)
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > what does .10i do that .10h did not (block-qualified recovery of the disjoint register class)
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > what does CORPUS-COVERAGE.1 add
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > what does DOCLING_DEVICE do
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > what does KG-ISF-TRANSACTIONS.2k add
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > what does MessageFieldRecord.bit_range mean and when is it set
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > what does MessageFieldRecord.byte_offset mean
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > what does SPECFORGE_INGEST_ADAPTIVE_BATCH do
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > what does SPECFORGE_INGEST_MIN_FREE_DISK_MB do
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what does SPECFORGE_INGEST_RAM_ABORT_PERCENT do
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what does SPECFORGE_INGEST_RAM_SAMPLE_SECS do
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > what does SPECFORGE_INGEST_SAVE_PAGE_IMAGES do
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what does SWD Figure B4-1 show (single SWDIO wire packet, bit-field time-phases, Host/Target/Host driver)
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what does SpecForge take from a grounded author
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > what does SpecForge take from the spec-mining literature and what does it leave out
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > what does a fast category recognizer need beyond surface counts (wire-relation shape, front-matter/self-declared type, topology cue)
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > what does byte_offset mean on a message field record when bit_range is None
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > what does bytes[index] as char do to non-ASCII UTF-8 text in prior_memory.rs
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what does converge --promote-constraints-llm do and when does it run
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > what does drop_unconditional_overlap_conflicts / unconditional_overlap_residual_packet do in ir/isf_ir.rs
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > what does drop_unrenderable_rule_values / unrenderable_rule_value_residual_packet do in ir/isf_ir.rs
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > what does intent --nli-verify do
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > what does is_dotted_cross_reference_subject do in evidence.rs
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > what does isf_enum_value_is_emittable_literal / isf_enum_is_emittable do in ir/isf_ir.rs
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > what does lead_binds_value do in evidence.rs
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > what does resolve_indexed_signal_family do
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > what does sanitize_isf_name do (allowlist [A-Za-z0-9_] -> everything else becomes _)
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what does signal_table_covered_by_inventory do
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > what does synthesize_signal_declarations do when the body is rotated
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what does the TASK-ACCEPTANCE check verify / why was my commit blocked
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > what does the bounded current ROADMAP contain
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what does uncaptured_normative_statement_ids do
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > what dominates the source_ir.json size (content_elements? page_artifacts?)
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > what enum member values does FSMGen reject (a bare token of only 0/1 digits with length >= 4 — an un-qualified binary literal; verified by value sweep: 1000/1010/1111/10000 fail, 999/1020/69152 and 0/1/111 and 4'b1000/16'd1000 pass)
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > what eval-extraction tasks score the SWD surfaces
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > what extraction approach does SWD/ADI need
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > what falls outside Docling's segmented bounding boxes on a page
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what field holds the constrained signal name (signal_name vs subject_signal)
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what fraction of a chip-spec PDF's intent reaches the emitted .isf, per purpose category
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what fsmgen pin carries the phase-membership answer (030f8c273, FSMGEN-REFRESH-INTEGRATE-3, ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.1/.2)
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > what fsmgen pin does SpecForge target for temporal properties
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > what gates protect the leading-identifier mnemonic form from bleed
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > what happens to a register-worded caption that grounds no identifier
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what happens to the extraction-quality gauge when the constraint surface is replaced
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > what happens when the NLI provider is down
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > what identifies the canonical inputs to generated Knowledge Map shards
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > what is AppError::IngestAbortedForDisk
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what is AppError::IngestAbortedForMemory
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > what is DOC-INTENT-TAXONOMY .2 Result 3 verdict (closed by .4e: the rule-lowering shortfall is dominated by conditional_rules that are honest residual; signal_constraints + temporal_rules lower well; not an ISF-completeness gap)
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > what is EvidenceIr.extraction_quality_gauge and who writes it
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > what is KG-ISF-COMPLETENESS.2a.iii (ISF module-name HDL-sanitization)
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > what is KG-ISF-COMPLETENESS.2a.iv (ISF enum value-literal emit gate / Lever F)
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > what is KG-ISF-COMPLETENESS.2a.v (ISF unconditional-rule-overlap conflict residual / Lever C)
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > what is KG-ISF-COMPLETENESS.2a.vi (ISF rule-drive-value validity gate)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what is KG-ISF-COMPLETENESS.5 (the generic-enum-conflation measurement + decision packet)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > what is KG-ISF-TRANSACTIONS.2m / the channel-membership lever
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > what is Lever E / KG-ISF-COMPLETENESS.1c agent-identity precision for the dense-prose doc class
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > what is MessageFieldConstraintRecord / ground_constraint_typed / GroundedConstraint
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > what is MessageFieldRecord / message_field_surface / message_fields manifest entry
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > what is NON_ACTOR_TRAILING_DISCOURSE_MARKERS and why is it a subset of the leading function-word list
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > what is NON_ACTOR_TRAILING_FUNCTION_WORDS and why does it exclude conjunctions
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > what is PDF-VARIANT-DIGESTION.12a
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > what is PDF-VARIANT-DIGESTION.12b
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > what is PDF-VARIANT-DIGESTION.3 prose entity capture
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > what is ProtocolStateRecord / protocol_states / DBGTAPSM
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > what is SWD's actual intent / protocol (from the spec)
- [swd-serial-frame-surface](../knowledge/swd-serial-frame-surface.md)
  > what is SerialFrameField / serial_frame_fields / SerialFramePhase
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > what is SpecForge doing in academic or research terms
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > what is TransactionIntent.channel_membership and where is it built (mint_named_transaction)
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > what is a ClusterExtractionProfile / derive_extraction_profiles (the per-cluster extraction profile)
- [contested-priors](../knowledge/contested-priors.md)
  > what is a contested prior
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > what is audit-extraction / PDF-VARIANT-DIGESTION.4b
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > what is capture_signal_presence_rows and who shares it
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > what is consolidate_register_field_fragments / EXTRACTION-GAP-FIX.4c
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > what is content-based name-column detection / rotation offset remapping
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > what is continuation_inherited_table_heads and what grounds the join
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > what is corpus_cluster / document_fingerprint / cluster_documents / DocumentCluster
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > what is declared_signal_complete_gold_precision
- [llm-primary-constraint-dedup](../knowledge/llm-primary-constraint-dedup.md)
  > what is dedup_constraints / its canonical key
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > what is definitional_signal_names / the copula + glossary-colon prose-signal grammar
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > what is document_completeness_gauge / document_completeness_gaps / why is a guide not penalized
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > what is document_intent_category / the 6-category purpose recognizer
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > what is document_type_declared / front_matter_doc_type_hint
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > what is extract_composition_frame_fields / is_frame_field_name / stated_frame_field_bit_width / parse_count_word
- [agnostic-quoted-mode-fsm](../knowledge/agnostic-quoted-mode-fsm.md)
  > what is extract_quoted_mode_states / quoted_mode_states_in / mode_state_ ids
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > what is extract_transition_bound_states / transition_bound_state_names_in / is_bare_state_name / named_state_ ids
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > what is in seed_swd_derivation.json
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > what is index-family signal canonicalization
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
