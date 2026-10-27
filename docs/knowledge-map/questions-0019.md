# Knowledge questions — shard 0019

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

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
- [wishbone-clock-and-reset-come-from-prose](../knowledge/wishbone-clock-and-reset-come-from-prose.md)
  > where does Wishbone's clock and reset come from (it has no signal table)
- [a-fronted-condition-ends-after-its-list](../knowledge/a-fronted-condition-ends-after-its-list.md)
  > where does a fronted condition end when it contains a list
- [minted-enum-values-are-mostly-row-indices](../knowledge/minted-enum-values-are-mostly-row-indices.md)
  > where does a minted enum value come from
- [caption-label-is-not-table-identity](../knowledge/caption-label-is-not-table-identity.md)
  > where does a minted enum's name actually come from
- [questions-bugs-and-features-to-upstream-travel-as-one-kind-of-packet](../decisions/0054-questions-bugs-and-features-to-upstream-travel-as-one-kind-of-packet.md)
  > where does a question to an upstream project go
- [a-storage-or-value-noun-heads-no-agent](../knowledge/a-storage-or-value-noun-heads-no-agent.md)
  > where does a register field bound to a wire go
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > where does a register's byte offset come from when only the caption states it
- [a-table-of-declared-names-is-a-rules-operand-set](../knowledge/a-table-of-declared-names-is-a-rules-operand-set.md)
  > where does a rule that cites a table by number (signals shown in Table A6.5) get its operands
- [sourceir-classification-is-per-record](../knowledge/sourceir-classification-is-per-record.md)
  > where does a rule that needs document order belong if SourceIR cannot hold it (EvidenceIR, which reads the whole SourceIR as its input)
- [a-register-width-is-stated-never-inferred](../knowledge/a-register-width-is-stated-never-inferred.md)
  > where does an .isf register var width come from when the document states none
- [relation-verb-after-an-article-is-a-noun](../knowledge/relation-verb-after-an-article-is-a-noun.md)
  > where does an active relation verb's object end (at but, and at an infinitive with its own subject: for the Manager to drive)
- [an-encoding-members-enabling-property](../knowledge/an-encoding-members-enabling-property.md)
  > where does an enum member's enabling property go (AWCMO CleanSharedPersist needs Persist_CMO)
- [document-class-is-decided-at-build-but-written-by-validate](../knowledge/document-class-is-decided-at-build-but-written-by-validate.md)
  > where does declared_type come from in validate
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
- [a-register-field-binding-to-a-wire](../knowledge/a-register-field-binding-to-a-wire.md)
  > where does specforge record that a register bit controls a signal
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > where does specforge validate write validation_report.json
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > where does the .isf signal direction/width come from (direction_hint/width_hint, not the actor graph)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > where does the <qualifier> phase vocabulary live (extracted_statements prose, not section_anchors)
- [assume-sentences-fall-into-six-shapes](../knowledge/assume-sentences-fall-into-six-shapes.md)
  > where does the ExplicitAbstraction class come from / which words make a statement an abstraction
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
- [isf-emission-blocked-by-clock-reset-contract](../knowledge/isf-emission-blocked-by-clock-reset-contract.md)
  > where does the isf reset polarity come from
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > where does the nli-verify measurement go / is the extraction-quality gauge persisted
- [direction-column-drift](../knowledge/direction-column-drift.md)
  > where does the phantom signal DATA in the CoreSight TMC come from
- [a-frozen-qualification-population-is-a-subset-floor-not-an-equality](../decisions/0050-a-frozen-qualification-population-is-a-subset-floor-not-an-equality.md)
  > where does the post-boundary retention declaration live and why
- [a-layouts-reserved-space-is-a-typed-span-citing-the-keyword](../knowledge/a-layouts-reserved-space-is-a-typed-span-citing-the-keyword.md)
  > where does the reserved space of a packet layout go (Reserved (R) . at 3:19)
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > where does the specforge chipdoc host library symlink currently resolve
- [a-hold-keeps-its-release](../knowledge/a-hold-keeps-its-release.md)
  > where does the until release go (the temporal rule's exceptions, not the condition)
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > where does the uppercase spelling in a declaration come from
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > where is FigureRegion stored in EvidenceIR
- [knowledge-map-architecture-location](../knowledge/knowledge-map-architecture-location.md)
  > where is KNOWLEDGE_MAP_ARCHITECTURE.md
- [a-sharded-task-manifest-carries-no-closed-route](../knowledge/a-sharded-task-manifest-carries-no-closed-route.md)
  > where is a closed leaf's route (routes-000N.md)
- [a-list-items-number-is-the-converters-marker](../knowledge/a-list-items-number-is-the-converters-marker.md)
  > where is a list item's number (marker, enumerated) in the converter output and in SourceIR
- [an-invariant-holds-at-every-cycle](../knowledge/an-invariant-holds-at-every-cycle.md)
  > where is a minimum duration such as for at least one clock cycle kept
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > where is a register summary's Reset column kept (RegisterRecord.reset_value)
- [a-tie-off-is-not-a-drive](../knowledge/a-tie-off-is-not-a-drive.md)
  > where is a tie-off's configuration (the signal's own absence on the driver, its presence condition)
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
- [document-class-is-decided-at-build-but-written-by-validate](../knowledge/document-class-is-decided-at-build-but-written-by-validate.md)
  > where is document_class computed and where is it stored
- [table-name-cells-seed-known-signals](../knowledge/table-name-cells-seed-known-signals.md)
  > where is inferred_name_is_an_ordinary_word applied
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
- [the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask](../knowledge/the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask.md)
  > where is the API reference (generated/mdbook/specforge/api, reference/api-reference.md)
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
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > where is the chain-currency content-identity predicate defined (cargo xtask check chain-currency)
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
- [extraction-and-isf-requests-advance-in-parallel](../decisions/0053-extraction-and-isf-requests-advance-in-parallel.md)
  > where is the gap table between PDF extraction and isf representation
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > where is the gate that stops an unknown-kind table from producing signal declarations (should_treat_table_as_top_level_signal_description, crates/specforge/src/ir/evidence.rs — the _ => continue width/direction arm is never reached for such a table)
- [missing-pieces-are-indexed-in-docs-gaps](../knowledge/missing-pieces-are-indexed-in-docs-gaps.md)
  > where is the list of everything SpecForge still cannot do from PDF to ISF
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > where is the mdBook current-truth verifier
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > where is the normative IntentIR product contract
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > where is the per-author adopt-vs-defer provenance
- [source-to-intent-first-reviewed-result](../knowledge/source-to-intent-first-reviewed-result.md)
  > where is the persisted source-to-IntentIR result snapshot
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > where is the persisted-artifact path check implemented (cargo xtask check persisted-artifact-paths)
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > where is the proof-seal reader defined (cargo xtask check proof-seal-currency)
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
- [bounded-task-root-accretes-in-its-frontier](../knowledge/bounded-task-root-accretes-in-its-frontier.md)
  > where should a closed leaf's verification and commit rows live after a task tree migration
- [book-chapter-growth-is-one-accreting-section](../knowledge/book-chapter-growth-is-one-accreting-section.md)
  > where should a new extraction rule document itself in the book
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > where was build_gates removed
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > where was build_phases removed
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > which 15 docs are category-3 platform/system-IP in the corpus (GIC-600/400 TRMs, CoreSight SoC-600 x3 / SDC-600 / TMC TRMs, MMU-700 TRM, Cortex-A76 TRM, CoreSight Base System Arch, CoreSight/GIC/SMMU/ARM-Debug-v6/Advanced-Comms-Channel architecture specs; the .1 census never persisted the per-doc labels — .4c.i enumerates them in scripts/measure_cat3_topology_recall.py)
- [a-register-field-binding-to-a-wire](../knowledge/a-register-field-binding-to-a-wire.md)
  > which ADIv6 fields are bound to wires
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > which ADR 0026 decision was wrong and why
- [an-encoding-members-enabling-property](../knowledge/an-encoding-members-enabling-property.md)
  > which AXI enums have property-gated members (AWSNOOP, AWCMO, AxPAS)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > which AXI signals belong to which channel (B1.1 write request / B1.2 write data / B1.3 write response / B1.4 read request / B1.5 read data / B1.6 B1.7 snoop)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > which AXI transactions gain channel grouping (atomic/prefetch/writezero/writedeferrable/narrow_transfer)
- [missing-pieces-are-indexed-in-docs-gaps](../knowledge/missing-pieces-are-indexed-in-docs-gaps.md)
  > which FSMGen capabilities are supported, requested, anticipated or fixed upstream
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > which IR stages have adopted move safe persisted paths
- [unread-intent-records-fall-into-six-families](../knowledge/unread-intent-records-fall-into-six-families.md)
  > which IntentIR assumptions does SpecForge write itself (in this IntentIR pass)
- [isf-emission-blocked-by-clock-reset-contract](../knowledge/isf-emission-blocked-by-clock-reset-contract.md)
  > which IntentIR families does the isf adapter never read
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
