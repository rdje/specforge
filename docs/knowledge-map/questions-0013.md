# Knowledge questions — shard 0013

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [identifiers-are-opaque-and-one-way-grounded](../decisions/0037-identifiers-are-opaque-and-one-way-grounded.md)
  > what is one-way grounding from EvidenceIR to ISF
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > what is promotion_status not_promoted_review_required and where does the canonical mutation live
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > what is recover-register-bits / how does the recover-register-bits command work
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > what is recovered_trapped_data_rows and who shares it
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > what is register_field_name_recall / register_field_completeness / register_bit_structure_recall
- [declaration-replay-reads-the-legacy-stratum](../knowledge/declaration-replay-reads-the-legacy-stratum.md)
  > what is replay-declarations
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > what is replay_bytes and what depends on it
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > what is residual_gold_law
- [task-evidence-route-catalogs-shard-by-lifecycle](../decisions/0046-task-evidence-route-catalogs-shard-by-lifecycle.md)
  > what is route_catalog_state
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what is scripts/check_doctrines.sh / the doctrine driver
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > what is select_initiator_actor / initiator_perspective_directions
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > what is signal_presence_records and what does a SignalPresenceRecord hold
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > what is snap_subject_to_sentence_token and when does it fire
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > what is source_batch in SourceIR
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > what is synthesize_register_field_tables
- [prose-pin-appositive-signal-capture](../knowledge/prose-pin-appositive-signal-capture.md)
  > what is synthesize_signal_declarations_from_prose / the pin-appositive pattern
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > what is table_states_a_property_rather_than_a_signal_inventory
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
- [llm-primary-recall-ceiling](../knowledge/llm-primary-recall-ceiling.md)
  > what is the LLM recall ceiling per document (AXI ihi0022_l_2025_08 209 to 37 = 17.7%, AHB ihi0033_c 57 to 10 = 17.5%, I2C um10204 27 to 3 = 11.1%, APB ihi0024_e 21 to 9 = 42.9%, ADIv6 ihi0074_a 12 to 1 = 8.3%)
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
- [opencapi-32g-phy-timing-without-interface-topology](../knowledge/opencapi-32g-phy-timing-without-interface-topology.md)
  > what is the OpenCAPI 32G PHY Signaling visual and NLP capture frontier
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
- [trajectory-steering-is-a-reviewable-multimetric-control-loop](../decisions/0034-trajectory-steering-is-a-reviewable-multimetric-control-loop.md)
  > what is the SpecForge trajectory controller
- [caption-repair-corpus-selection](../knowledge/caption-repair-corpus-selection.md)
  > what is the TileLink Figure 3.1 caption removal
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > what is the VLM table strategy / PDF-VARIANT-DIGESTION.2b
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > what is the `<role> channel signals` caption cue and how is it parsed
- [book-quantitative-candidate-vocabulary](../knowledge/book-quantitative-candidate-vocabulary.md)
  > what is the admission bar for a new candidate unit noun (measured precision: a noun is admitted only when EVERY line it newly admits is a real published quantity, read by hand, with the marginal count taken over the already-admitted set rather than the old baseline because overlap flatters a noun)
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > what is the ambiguous_statements metric in validate
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > what is the authoritative membership set for corpus SOURCE_PDF_REGISTRY
- [behavioral-text-projection-boundary](../knowledge/behavioral-text-projection-boundary.md)
  > what is the behavioral genericity held-out population
- [self-test-coverage-guard-is-in-the-exit-path](../knowledge/self-test-coverage-guard-is-in-the-exit-path.md)
  > what is the behavioural oracle for a coverage guard
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > what is the bit-exact adjacency chain rule
- [actor-taxonomy-grows-in-pairs-not-terms](../knowledge/actor-taxonomy-grows-in-pairs-not-terms.md)
  > what is the blast radius of a new actor-role term (four surfaces: the direction cell of a signal row, a section heading ending in ' signals', a relation-actor name in the by-role map, and the complementary-reader's exactly-one-opposite-name condition, which is NOT monotone)
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
- [claim-standard-upstream-readoption](../knowledge/claim-standard-upstream-readoption.md)
  > what is the cheapest falsification oracle available in this repository
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > what is the claim verification registry schema
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > what is the complete parenthetical data-head declaration census
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
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > what is the corpus pre-bind subject measurement (26 false records across nine documents)
- [corpus-task-bounded-active-root-and-evidence-parts](../decisions/0024-corpus-task-bounded-active-root-and-evidence-parts.md)
  > what is the corpus task evidence writer transaction after migration
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > what is the corpus timing-table authority repair measurement (2144 to 608 across 39 documents)
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > what is the current corpus refresh frontier after refresh 48
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > what is the declared-signal eval surface / EvalTask::DeclaredSignal
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > what is the difference between PROOF-SEAL-CURRENCY and CHAIN-CURRENCY
- [a-cheap-structural-rule-overfires-until-you-read-its-selection](../knowledge/a-cheap-structural-rule-overfires-until-you-read-its-selection.md)
  > what is the difference between a census that OVER-fires and one that UNDER-reads (over-firing selects things that are not what you think, and the remedy is to read the selection; under-reading sees one spelling of a notation and misses the others, and the remedy is to census the spellings - both are 'a count is not an adjudication' but they fail in opposite directions)
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > what is the difference between a refreshed EvidenceIR and a retained normalized bundle
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > what is the difference between ingest batch activation and adaptive batch size
- [a-relational-predicate-is-not-a-value](../knowledge/a-relational-predicate-is-not-a-value.md)
  > what is the difference between is_reference_magnitude_constraint and value_slot_states_a_relation
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > what is the difference between phases and transaction_phases
- [persisted-census-measures-published-not-current](../knowledge/persisted-census-measures-published-not-current.md)
  > what is the difference between the published population and the actionable population
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > what is the difference between validate persistence and stage write_to_disk
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > what is the exact CORPUS-COVERAGE task evidence boundary
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is the exact SPEC-TO-INTENT-ALIGNMENT task evidence baseline
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is the exact active PDF task evidence baseline
- [refresh-completion-is-not-artifact-currency](../knowledge/refresh-completion-is-not-artifact-currency.md)
  > what is the exact partition of the 78 persisted documents by schema and refresh cohort (78 = 21 refreshed-and-loadable + 6 outside-cohort-and-loadable + 31 refreshed-but-legacy + 15 outside-cohort-and-legacy + 5 remaining-and-legacy)
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
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > what is the first task in canonical recovery
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > what is the flow-arrow direction grammar
- [bounded-decision-frozen-baseline](../knowledge/bounded-decision-frozen-baseline.md)
  > what is the frozen adjudication set for the bounded decision provider
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
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > what is the line-size profile of SpecForge research records
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what is the live-document coverage authority
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > what is the longest legitimate parametric width expression in the corpus
- [workflow-standard-capacity-is-rederived-from-explicit-member-growth](../decisions/0043-workflow-standard-capacity-is-rederived-from-explicit-member-growth.md)
  > what is the measured peak day for workflow standards
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > what is the message_field_catalog_dump measurement harness
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > what is the message_fields.section_header_field strategy
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > what is the mirror test for signal tables
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
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > what is the per-record byte budget for a status record
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > what is the permission-vs-obligation gate / frame error class
- [constraint-record-producer-strata](../knowledge/constraint-record-producer-strata.md)
  > what is the population of a kind-classifier change in evidence.rs
- [bounded-decision-frozen-baseline](../knowledge/bounded-decision-frozen-baseline.md)
  > what is the pre-registered adoption bar for a remote decision provider
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > what is the presence-matrix structural gate and how many tables fire it
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > what is the present blocking point on the specification-to-executable-intent path
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > what is the proposed Extractor framework (registry / driver / SurfacePolicy / run manifest)
- [persisted-llm-constraint-corpus-predates-catalog-grounding](../knowledge/persisted-llm-constraint-corpus-predates-catalog-grounding.md)
  > what is the real membership test the LLM constraint path grounds a subject against (resolve_unique_document_identifier over declared_signal_catalog(ir) first and message_field_records names second — commands/extract_constraints_llm.rs:110-128 — NOT the seven signal-bearing EvidenceIR surfaces, which no producer consults)
- [measured-stratum-promotion-population](../knowledge/measured-stratum-promotion-population.md)
  > what is the recall universe of promote_constraints (NOT the document — the distinct source_text of the constraints already persisted, one provider call each; a document with no Pattern constraints gets no LLM proposals at all)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what is the recommended fix (.5.i extraction-side fallback name-gate: derive_encoding_enum_name must return None unless the candidate token is a declared signal -> no enum minted; + emitter orphan-type fix isf_ir.rs:403-409 gate types block by emitted_enums(); .5.ii member-quality gate for the 271 real-named junk enums, calibration-gated)
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > what is the register-field eval surface (EvalTask::RegisterField)
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > what is the registers.section_header_field strategy
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > what is the remedy when a research record reaches its per-file line ceiling
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > what is the rolling ledger archive protocol
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > what is the safe way to narrow an over-wide line in a governed Markdown surface (reflow the enclosing block at the narrowest column that preserves its line count exactly; that keeps the byte count identical too, because the number of separators is unchanged)
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > what is the sample size of the gate-tier seal probe
- [a-sealed-region-cannot-move-out-of-an-active-part-alone](../knowledge/a-sealed-region-cannot-move-out-of-an-active-part-alone.md)
  > what is the scope of the LIVE-DOCUMENT-PRESSURE-HEADROOM.30a two-stratum rule (one part FILE - a leaf's sealed declaration and its post-migration supersession must live in the same file)
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > what is the semantic_ungrounded_records_not_promoted residual packet
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > what is the semantic_unreadable_declaration_width residual packet
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what is the shipped ISF field-structured storage grammar ((storage (var NAME (width N) [(reset V)] (fields (field NAME (bits HI LO) [(access ...)] [(reset V)] [(enum ...)]))))) — metadata-only/schedule-safe, report key inferred_storage[].fields[])
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > what is the size-immunity binding constraint for source_ir.json at extreme page counts
- [spec-to-intent-category-contract](../knowledge/spec-to-intent-category-contract.md)
  > what is the source-to-IntentIR completeness contract
