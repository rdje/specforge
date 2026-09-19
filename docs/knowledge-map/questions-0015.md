# Knowledge questions — shard 0015

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

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
- [behavioral-text-projection-boundary](../knowledge/behavioral-text-projection-boundary.md)
  > which behavioral genericity transforms cover rich PDF capture
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > which binary does check_proof_seal_currency.sh probe with
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > which boot-volume Rust directories are allowed
- [canonical-collection-catalogs](../knowledge/canonical-collection-catalogs.md)
  > which canonical Markdown collections still rely only on git query
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
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > which collection order changes may the behavioral comparator normalize
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > which column does synthesize_signal_declarations read names from, and when does content overrule the header (a distinct-hardware-token score per column, overruling the header only on a lead of at least two tokens)
- [doctrine-driver-runs-no-cargo-gate](../knowledge/doctrine-driver-runs-no-cargo-gate.md)
  > which command does CI use for clippy
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > which command produces the document_class metric
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > which conditional rules survive when a document declares no signals
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > which converter items earn no residual
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > which corpus PDFs are password/permission protected
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > which corpus docs declare fields with a Field-titled column
- [corpus-coverage-sweep](../knowledge/corpus-coverage-sweep.md)
  > which corpus docs still yield nothing (the VLM frontier)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > which corpus documents are category 4 CPU-ISA (exactly 2: 1_0_risc_v_debug_specification and 1_0_2025_03_12_risc_v_advanced_interrupt_architecture; the RISC-V IOMMU doc is category 2)
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > which corpus documents are real signal-catalog capture misses
- [a-split-name-cell-does-not-say-which-join-it-wants](../knowledge/a-split-name-cell-does-not-say-which-join-it-wants.md)
  > which corpus documents declare from a subscript-split name cell
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > which corpus stages are measurable without re-ingesting a document
- [required-residual-actionability-denominator](../knowledge/required-residual-actionability-denominator.md)
  > which crate owns the source-to-intent vertical evaluator tests
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > which current claim census counts are stable and which ones move
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > which current claim census counts may be published and which must be read from the report
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > which docs exhibit the dense-prose actor explosion (is it AMBA or non-AMBA)
- [corpus-coverage-sweep](../knowledge/corpus-coverage-sweep.md)
  > which docs fail to ingest (giants / timeouts)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > which docs gain a register reset in the .isf (only the 3 CoreSight SoC-600 TRMs — 199/127/120 V>0 resets; the register-heavy non-wire docs)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > which docs have the ISF value-width defect (4 docs / 13 clauses: DTI ATST ×3 [mis-attribution], AXI+ACE ARTAGOP/BTAGMATCH ×6 [width-2 under-emitted, masked by (port expr)], AXI-gold AWCMO ×1 [parametric AWCMO_WIDTH], trace-bus ATID ×3 [width-7 under-emitted — the clean lever])
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > which docs need a re-ingest (Docling + source PDF) vs a cheap stage rebuild
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > which doctrine JSON contracts must never be re-encoded at all (fsmgen_feedback.json, roadmap_projection.json, spec_to_intent_vertical_eval_schema.json and trajectory_controller_input_schema.json are hand-authored and keep small objects inline on one line, which no encoder emits; re-encoding to change one field expands them 9-27% and destroys that layout)
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > which doctrine checks the persisted proof seal on every commit
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > which doctrines are registered (MEMORY-ARCH, KNOWLEDGE-MAP, TASK-ACCEPTANCE)
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > which doctrines are registered in the SpecForge doctrine driver
- [a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest](../knowledge/a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest.md)
  > which doctrines does --fast skip
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > which document categories are mature vs partial vs thin for ISF synthesis (cat1 mature, cat2/3 partial, cat4 thin, cat5/6 non-target)
- [source-to-intent-reviewed-population](../knowledge/source-to-intent-reviewed-population.md)
  > which documents are in the first source-to-IntentIR vertical evaluation population
- [wire-golds-held-out-not-lost](../knowledge/wire-golds-held-out-not-lost.md)
  > which documents are undeclared rather than unrebuildable
- [legacy-source-classifications-are-neutralized-on-load](../knowledge/legacy-source-classifications-are-neutralized-on-load.md)
  > which documents can a classification-keyed extractor actually see
- [table-row-obligation-binds-to-the-token-before-its-modal](../knowledge/table-row-obligation-binds-to-the-token-before-its-modal.md)
  > which documents carry obligation-bearing signal-description rows
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > which documents declare signals from tables in the current stratum
- [persisted-census-measures-published-not-current](../knowledge/persisted-census-measures-published-not-current.md)
  > which documents have a retained normalized bundle
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > which documents in the corpus are currently stale
- [the-persisted-corpus-has-a-measured-and-a-historical-stratum](../decisions/0048-the-persisted-corpus-has-a-measured-and-a-historical-stratum.md)
  > which documents may a new measurement population be drawn from
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > which documents no longer reproduce their persisted SourceIR
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > which encoder wrote doctrine/live_document_size/fact_card_catalog.json (python3 json.dumps indent=1 sort_keys=True ensure_ascii=True plus a trailing newline - byte-identical to Perl JSON::PP canonical with indent(1) and space_after but WITHOUT space_before. JSON::PP pretty is wrong for it, because pretty also sets space_before and emits key-space-colon)
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > which exact current state copies are not yet independently verified in SpecForge
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > which extraction commands does converge run directly
- [the-binding-bearing-clause](../knowledge/the-binding-bearing-clause.md)
  > which fields does ConstraintReplayVerdict carry
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > which fields may reviewed text normalization change
- [partitioning-a-task-tree-has-a-fixed-registration-price](../knowledge/partitioning-a-task-tree-has-a-fixed-registration-price.md)
  > which file must the parts surface's census evidence record name (the collection's ALPHABETICALLY first part file, not the first in contract order)
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > which files pin line-anchored claim regions that a mid-file insert will shift (book_quantitative_claims.jsonl, published_assertions.jsonl and current_claim_census.jsonl - published_assertions is the one that gets forgotten)
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > which five current claim census gaps does CLAIM-VERIFICATION-ADOPTION.3b repair
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > which formal signal declaration predicates does the dense prose authority gate accept
- [a-frozen-qualification-population-is-a-subset-floor-not-an-equality](../decisions/0050-a-frozen-qualification-population-is-a-subset-floor-not-an-equality.md)
  > which frozen boundary values in behavioral_qualification.json stay exact
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > which frozen fabrication defects still reproduce in current SpecForge
- [constraint-record-producer-strata](../knowledge/constraint-record-producer-strata.md)
  > which functions call classify_signal_constraint_kind
- [one-modal-vocabulary-per-constraint-record](../knowledge/one-modal-vocabulary-per-constraint-record.md)
  > which functions decide whether a clause states an obligation
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > which guides over-extract spurious .isf wire intent (cortex-a76 sw-opt 537 signals, readme, smmu software guide, gic overview, aarch64 debug guide)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > which header signatures are unrecovered register tables
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > which held-out document has an eligible opaque alpha catalog
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > which is cheaper, a seal check or a chain-currency replay
- [persisted-table-kind-is-a-classifier-generation-artefact](../knowledge/persisted-table-kind-is-a-classifier-generation-artefact.md)
  > which legacy declarations come from tables that are not signal tables
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > which limit binds first when adding a fact card
- [toolbox-catalog-is-a-routed-landing](../knowledge/toolbox-catalog-is-a-routed-landing.md)
  > which line must a new included census collection surface pin its evidence record to (the first non-blank line of its ALPHABETICALLY FIRST member path - produce_candidates sorts the surface's paths and emits one surface_review candidate from the first, so pinning any other member fails with 'lacks exact evidence')
- [refresh-completion-is-not-artifact-currency](../knowledge/refresh-completion-is-not-artifact-currency.md)
  > which loadable documents are outside the CORPUS-FRONTIER refresh cohort (6 — ihi0022_l_2025_08, ihi0024_e, ihi0033_c, ihi0074_a, um10204, um11732; the cohort rule selects on source.requested_path NOT beginning with corpus/, and these were ingested from corpus/ already, so they never needed a refresh)
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > which local model should SpecForge use for NLI or entailment verification
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > which local models are pulled and what are they for
- [a-level-belongs-to-a-signal](../knowledge/a-level-belongs-to-a-signal.md)
  > which logic level records did the pairing remove
- [mdbook-quantitative-census-freeze](../knowledge/mdbook-quantitative-census-freeze.md)
  > which mdBook quantitative assertions still lack claim authority
