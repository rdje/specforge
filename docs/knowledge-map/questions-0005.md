# Knowledge questions — shard 0005

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what ISF abstractions does FSMGen need next (field-structured storage / register-with-fields, packet/structure layouts, topology)
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > what ISF form does SpecForge use for a bounded-eventually contract
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > what ISF idiom describes states and input-driven transitions
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > what already lowers for category-3 platform docs (the register half — register maps + bit-fields via .4a.ii e.g. CoreSight SoC-600 ~3,250 fields, infrastructure signals, actor ports; cat-3's register intent is the same road as cat-2 and is not the gap)
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what architecture contains an oversized active task tree
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > what are AGENT_CLASS_NOUNS / the parenthetical-strip / sentence-boundary / no-preposition guards
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what are APB's remaining completeness candidate misses
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > what are NON_ACTOR_LEADING_FUNCTION_WORDS and NON_ACTOR_LEADING_VERBS for
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > what are examples of false prose relations in Introducing CoreSight
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > what are scripts/pdf_text.py and scripts/decrypt_pdf.py
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what are the 3 gaps G1 G2 G3 in specforge transaction capture
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > what are the 6 chip-spec document intent categories / purpose taxonomy
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > what are the 6 purpose categories (wire-protocol, register-or-platform, cpu-isa, physical-link, methodology-guide, unresolved)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what are the AXI B1.x channel-signal tables and how do channels map to phases
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > what are the SWD packet phases and per-phase SWDIO direction
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > what are the agent-surface precision and completeness defects (KG-ISF-COMPLETENESS.1)
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > what are the canonical SWD protocol surface counts
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > what are the current AArch64 External Debug artifact hashes
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > what are the current CoreSight Base System artifact hashes
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > what are the current Introducing CoreSight artifact hashes
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > what are the current OpenCAPI AFU address note artifact hashes
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > what are the current OpenCAPI Certified Definition artifact hashes
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > what are the current OpenCAPI Certified artifact hashes
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > what are the current OpenCAPI Ready artifact hashes
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > what are the current USB4 Connection Manager artifact hashes
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > what are the current USB4 Inter-Domain artifact hashes
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > what are the current corpus KB live-document size metrics
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what are the deeper enum member-quality residual classes after .5.ii / what did the .5.iii measurement find (measured 2026-06-24 read-only, reproducer scripts/measure_enum_width_leak.py: of the 5 deferred classes — glossary SEE…, front-matter/ToC, section-caption B2_3_1_…, _WIDTH parameter leaks, value-restart-of-clean — most are SUBSUMED by .5.i (47/54 _WIDTH members
  > and the bulk of 319 section-caption survivors sit in generic-named enums .5.i drops whole), EXCEPT the _WIDTH leak which reaches the AXI wire-gold .isf and is materially damaging)
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > what are the extraction_quality_* validate metrics and when do they read n/a
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > what are the isf_protocol residual packet prefixes
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
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > what changed between stale and current OpenCAPI Certified Definition artifacts
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
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > what did statement 0114 say in the stale OpenCAPI Ready evidence
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did the .2i Rule-A per-phase grouping measurement find (clean only on AHB, empty on APB/AXI/SWD)
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > what did the OpenCAPI AFU address note prove about legal boilerplate
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what did the active PDF task containment census find
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
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > what does AdapterArtifact write_to_disk reconcile
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > what does CORPUS-COVERAGE.1 add
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > what does DOCLING_DEVICE do
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > what does KG-ISF-TRANSACTIONS.2k add
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > what does LIVE DOCUMENT SIZE CONTAINMENT ADOPTION 8 implement
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
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > what does source_literal mean in the active task evidence contract
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > what does synthesize_signal_declarations do when the body is rotated
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what does the TASK-ACCEPTANCE check verify / why was my commit blocked
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what does the active task evidence checker verify
- [active-task-migration-transaction](../knowledge/active-task-migration-transaction.md)
  > what does the active task migration roll back after a validation failure
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
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > what grounded heuristic only interface evidence is preserved
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > what happened to the AArch64 External Debug Guide agent.isf
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > what happened to the CoreSight Base System agent.isf
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > what happened to the USB4 Connection Manager device_also.isf
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > what happened to the USB4 Inter-Domain channel.isf
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > what happened to the four signal two enum Introducing CoreSight adapter
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > what happened to the original Introducing CoreSight rollback
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > what happens to a page sidecar path when page images are not persisted
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > what happens to a register-worded caption that grounds no identifier
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what happens to the extraction-quality gauge when the constraint surface is replaced
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > what happens when SemanticIR has no authoritative signal declarations
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > what happens when SemanticIR has no authoritative signal names
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > what happens when the NLI provider is down
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > what identifies the canonical inputs to generated Knowledge Map shards
- [corpus-kb-managed-currentness](../knowledge/corpus-kb-managed-currentness.md)
  > what inputs define corpus KB managed block currentness
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is ADR 0019
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > what is AppError::IngestAbortedForDisk
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what is AppError::IngestAbortedForMemory
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > what is CORPUS-COVERAGE.2.33d dense-prose adapter trust repair
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
