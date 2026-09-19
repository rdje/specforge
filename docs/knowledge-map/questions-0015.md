# Knowledge questions — shard 0015

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

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
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > where does SpecForge record a captured region that produced no fact
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > where does SpecForge store temporary files and caches
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > where does SpecForge withhold template declarations
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > where does TileLink spell a_opcode
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where does TransactionPhaseRecord.signal_set come from (build_transaction_phases, ir/semantic.rs — phase-naming statements' signals ∩ declared inventory, the .2c technique)
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > where does a register's byte offset come from when only the caption states it
- [sourceir-classification-is-per-record](../knowledge/sourceir-classification-is-per-record.md)
  > where does a rule that needs document order belong if SourceIR cannot hold it (EvidenceIR, which reads the whole SourceIR as its input)
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > where does figure interior text go now
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > where does nli-verify / extract-constraints-llm / semantic / intent / adapt write their output
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > where does parse_table_width_hint_text refuse a cell
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where does recognize_digital_patterns hardcode HTRANS/PSEL/MISO and why is it an ADR-0006 breach
- [a-bounded-snapshot-needs-a-declared-repeatable-rollover](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
  > where does retired roadmap direction go
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > where does signal_channel_memberships live (EvidenceIR) and how is it carried (SemanticIR -> IntentIR)
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > where does specforge validate write validation_report.json
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > where does the .isf signal direction/width come from (direction_hint/width_hint, not the actor graph)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where does the <qualifier> phase vocabulary live (extracted_statements prose, not section_anchors)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > where does the ISF emitter lower a rule value literal (ir/isf_ir.rs:1493-1495 render_isf_control_expression → ControlExpressionRecord::Literal{literal}=>literal.clone(); copied verbatim, no width reconciliation at the emit site isf_ir.rs:418-432)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > where does the ISF emitter lower register reset_value (ir/isf_ir.rs: IsfStorageVar.reset + classify_register_reset + register_var_width; render emits (var NAME (width W) (reset V)); before .2/.3 IsfStorageVar had only name+width and dropped it)
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > where does the NLI gate route a not-entailed contract
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > where does the SpecForge chipdoc source route currently resolve
- [escaped-identifier-fragments-the-catalog](../knowledge/escaped-identifier-fragments-the-catalog.md)
  > where does the backslash underscore in EvidenceIR text come from
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > where does the block name come from when the PDF backend flattens heading levels (parent section title via dotted-parent)
- [source-to-intent-first-reviewed-result](../knowledge/source-to-intent-first-reviewed-result.md)
  > where does the first reviewed source-to-IntentIR population fail
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > where does the flow-arrow reading sit in the direction priority chain
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > where does the generic enum name TABLE/FIGURE/DATA come from (derive_encoding_enum_name fallback, evidence.rs:4457-4461 — first caption token passing is_hardware_signal_token at evidence.rs:7106, which accepts 'Table'->'TABLE')
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > where does the nli-verify measurement go / is the extraction-quality gauge persisted
- [direction-column-drift](../knowledge/direction-column-drift.md)
  > where does the phantom signal DATA in the CoreSight TMC come from
- [a-frozen-qualification-population-is-a-subset-floor-not-an-equality](../decisions/0050-a-frozen-qualification-population-is-a-subset-floor-not-an-equality.md)
  > where does the post-boundary retention declaration live and why
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > where does the specforge chipdoc host library symlink currently resolve
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > where does the uppercase spelling in a declaration come from
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > where is FigureRegion stored in EvidenceIR
- [knowledge-map-architecture-location](../knowledge/knowledge-map-architecture-location.md)
  > where is KNOWLEDGE_MAP_ARCHITECTURE.md
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > where is actor-signal relation drive/read direction decided in evidence.rs
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > where is adaptive_batch_pages / BatchSizePolicy in the code
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > where is build_fixture.py digest pinned
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > where is classify_document_intent_category implemented (crates/specforge/src/ir/completeness.rs)
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > where is consolidate_interface_actor_relations and strip_interface_suffix wired
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > where is consolidate_trailing_fragment and how is it ordered against the .1a reject
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > where is coordinated-object actor-signal relation extraction implemented
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > where is current binary replay evidence stored
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > where is is_pure_inferred_phantom_role and the build_intent_actors phantom guard
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > where is legal administrative prose filtered
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > where is normalized staging removed from Docling metadata
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > where is parenthetical single-wire authority enforced
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > where is register bit-field ISF emission implemented (register_storage_fields + normalize_field_access + IsfStorageField + the storage render in crates/specforge/src/ir/isf_ir.rs; residual isf_register_fields_not_lowered)
- [register-record-access-and-table-provenance](../knowledge/register-record-access-and-table-provenance.md)
  > where is register level access stored in RegisterRecord
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > where is register-field reset_value extracted and carried (ir/source.rs:432 RegisterFieldRecord.reset_value; ir/evidence.rs:11509/11838 populate it; ir/intent.rs:193 clones register_records to IntentIR)
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > where is split_coordinated_actor_relations and split_coordinated_actor_subject wired
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > where is the .6d.ii.f behavioral signoff recorded
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > where is the 12 document current replay manifest
- [wire-golds-held-out-not-lost](../knowledge/wire-golds-held-out-not-lost.md)
  > where is the AHB normalized bundle
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > where is the APB normalized bundle after WIRE-BASED-100.9b
- [wire-golds-held-out-not-lost](../knowledge/wire-golds-held-out-not-lost.md)
  > where is the AXI normalized bundle
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > where is the I2C signal gold seed
- [temporal-rule-ltl-rendering](../knowledge/temporal-rule-ltl-rendering.md)
  > where is the LTL renderer for temporal rules
- [swd-frame-phase-binding-lives-in-the-figure](../knowledge/swd-frame-phase-binding-lives-in-the-figure.md)
  > where is the SWD frame's field-to-phase membership actually stated (in Figure B4-1 SWD successful write operation and Figure B4-2 SWD successful read operation. Both are captured as visual assets picture_0038 and picture_0039, but their role is ambiguous and their only observation is the caption, so the diagram content was never read)
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > where is the SpecForge FSMGen feedback or handoff channel
- [trajectory-controller-engine](../knowledge/trajectory-controller-engine.md)
  > where is the SpecForge trajectory controller engine
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > where is the U+F0B7 signal integrity panic in evidence extraction
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > where is the acceptance checklist a code change must satisfy
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > where is the authoritative chipdoc source library now
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > where is the behavioral alpha renaming harness implemented
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > where is the biggest register extraction gap across the corpus
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > where is the canonical SpecForge user guide
- [canonical-collection-catalogs](../knowledge/canonical-collection-catalogs.md)
  > where is the complete index for SpecForge research workflow architecture corpus KB or KG fixtures
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > where is the complete live document containment adoption task history
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > where is the declared-signal gate applied (pattern + dynamic constraint paths)
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > where is the declared-signal set that the semantic grounding filter uses built
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > where is the default-flip decision packet / should promote-constraints-llm become the default
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > where is the dropped enum surfaced (an isf_enum_value_literal_<name> ResidualDecisionPacket via enum_residuals(), wired into adapters.rs residual_decisions)
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > where is the exact historical SpecForge roadmap after containment
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > where is the exact pre-migration alignment task source
- [claim-verification-task-evidence-migrated](../knowledge/claim-verification-task-evidence-migrated.md)
  > where is the exact pre-migration claim-verification task source
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > where is the extraction architecture contract
- [a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest](../knowledge/a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest.md)
  > where is the fast doctrine subset declared
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > where is the first post-migration DEVELOPMENT_NOTES rollover segment
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > where is the first post-migration LIVE_ACHIEVEMENT_STATUS rollover segment
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > where is the gate that stops an unknown-kind table from producing signal declarations (should_treat_table_as_top_level_signal_description, crates/specforge/src/ir/evidence.rs — the _ => continue width/direction arm is never reached for such a table)
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > where is the mdBook current-truth verifier
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > where is the normative IntentIR product contract
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > where is the per-author adopt-vs-defer provenance
- [source-to-intent-first-reviewed-result](../knowledge/source-to-intent-first-reviewed-result.md)
  > where is the persisted source-to-IntentIR result snapshot
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > where is the register-field gold seed
- [required-residual-actionability-denominator](../knowledge/required-residual-actionability-denominator.md)
  > where is the required-residual rule frozen
- [source-to-intent-reviewed-population](../knowledge/source-to-intent-reviewed-population.md)
  > where is the reviewed source-to-intent dataset
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > where is the second LIVE_ACHIEVEMENT_STATUS rollover segment and what blocks the next one
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > where is the set of retained normalized bundles declared
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > where is the shared 0.6 fingerprint clustering threshold defined (DEFAULT_FINGERPRINT_SIMILARITY_THRESHOLD)
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > where is the shared section-heading container-walk that .10f and .10g both use
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > where is the sole source to EvidenceIR canonical loss
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > where is the source-to-Intent vertical evaluation schema
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > where is the spawn+poll+kill memory guard in materialize_pdf
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > where is the structural gate that drops function-word-led and verb-led actor candidates
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > where is the tiling-gated register bit recovery implemented
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > where is the trailing function-word strip in consolidate_trailing_fragment
- [claim-standard-upstream-readoption](../knowledge/claim-standard-upstream-readoption.md)
  > where is the upstream claim-verification source
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > where is the weak-phrase / NASA ARM ambiguity detector
- [repository-local-scratch](../knowledge/repository-local-scratch.md)
  > where may diagnostic scratch files and comparison lists be written
- [live-document-containment-and-data-locality](../decisions/0007-live-document-containment-and-data-locality.md)
  > where must SpecForge project artifacts caches and temporary workspaces live
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > where was build_gates removed
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > where was build_phases removed
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > which 15 docs are category-3 platform/system-IP in the corpus (GIC-600/400 TRMs, CoreSight SoC-600 x3 / SDC-600 / TMC TRMs, MMU-700 TRM, Cortex-A76 TRM, CoreSight Base System Arch, CoreSight/GIC/SMMU/ARM-Debug-v6/Advanced-Comms-Channel architecture specs; the .1 census never persisted the per-doc labels — .4c.i enumerates them in scripts/measure_cat3_topology_recall.py)
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > which ADR 0026 decision was wrong and why
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > which AXI signals belong to which channel (B1.1 write request / B1.2 write data / B1.3 write response / B1.4 read request / B1.5 read data / B1.6 B1.7 snoop)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > which AXI transactions gain channel grouping (atomic/prefetch/writezero/writedeferrable/narrow_transfer)
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > which IR stages have adopted move safe persisted paths
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > which IntentIR surfaces are lowered to the .isf vs silently dropped
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > which LLM or VLM does SpecForge use
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > which Markdown files must the live-document containment registry cover
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > which PDF task history ids exist only as tree-relative shorthand
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > which README.md regions are pinned by line number (the census pins L1 as the document identity anchor, L30 as the rust_prerequisite_copies derived value, and L88-L104 as the entrypoint route block; the line numbers are current as of 2026-08-31 and must be re-derived, not assumed)
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > which SWD protocol facts are safe to lower today
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > which SemanticIR IntentIR adapter and prior memory paths serialize repository relative
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > which SemanticIR fields carry supporting_table_ids
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > which SourceIR and EvidenceIR paths serialize repository relative
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > which active task tree is next at the live document warning
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > which active task tree now requires bounded evidence containment
- [a-relational-predicate-is-not-a-value](../knowledge/a-relational-predicate-is-not-a-value.md)
  > which admissibility routes run before the relational test
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > which architecture owns work and which one verifies published claims
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > which arrow spellings does the declaration reader accept
- [registry-capacity-bounds-are-incoherent](../knowledge/registry-capacity-bounds-are-incoherent.md)
  > which banded registries are coherent
- [behavioral-text-projection-boundary](../knowledge/behavioral-text-projection-boundary.md)
  > which behavioral genericity transforms cover rich PDF capture
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > which binary does check_proof_seal_currency.sh probe with
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > which boot-volume Rust directories are allowed
- [canonical-collection-catalogs](../knowledge/canonical-collection-catalogs.md)
  > which canonical Markdown collections still rely only on git query
- [caption-repair-corpus-selection](../knowledge/caption-repair-corpus-selection.md)
  > which caption additions are table-reading descriptions rather than requirements
- [local-repair-closes-the-caption-decision](../knowledge/local-repair-closes-the-caption-decision.md)
  > which caption does arm B1 lose that arm A admitted
- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > which caption grammar can supply a unit to every timing row
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > which cargo profile do the corpus replay checks build
- [escaped-identifier-fragments-the-catalog](../knowledge/escaped-identifier-fragments-the-catalog.md)
  > which census measures escaped identifier fragments
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > which checks pin the retained-bundle population at 24
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > which claim control needed a known-bad repair in CLAIM-VERIFICATION-ADOPTION.4
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > which claim control-audit fields are gated and which must be read from the report
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > which claim family owns captured_region_residuals
