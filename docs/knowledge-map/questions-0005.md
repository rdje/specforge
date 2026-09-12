# Knowledge questions — shard 0005

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > how do I verify every task tree is linked exactly once
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > how do I waive or range-scope the task-acceptance check
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how do Published-claims ids resolve
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how do byte-granular page fragments chain (offset plus size adjacency)
- [register-record-access-and-table-provenance](../knowledge/register-record-access-and-table-provenance.md)
  > how do canonical register records retain source table provenance
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how do continuation tables (Table B2.2 Continued) merge into one container
- [decision-capacity-is-rederived-without-moving-stable-records](../decisions/0041-decision-capacity-is-rederived-without-moving-stable-records.md)
  > how do decision slots change Knowledge Map capacity
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how do dword-relative page fragments chain
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > how do packet/flit protocols (CHI-class) declare message fields vs signals
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > how do task trees memory the Knowledge Map doctrine enforcement and claim verification fit together
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how do you audit registers/signals against the table image with the VLM
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how does .10g differ from .10f (register vs message routing)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how does / will SpecForge reuse extraction patterns across different PDFs
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > how does CORPUS-COVERAGE 2 33d ii prevent weak signal names from reentering through relations
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > how does FSMGen decide a value literal's width (by notation digit count — 0x7D=8 bits, 0b00=2 bits — NOT by value; it requires an exact width-cast W'… match, no implicit truncation/extension; a bare decimal is unsized and fits any width)
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > how does FSMGen decide two rule data-writes conflict (same target, different value, NOT compatible/disjoint/priority/resource resolved) and when is a guard proven disjoint (_condition_terms_prove_disjoint: shared eq: signal with different values; an absent/empty condition is NEVER proven disjoint)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > how does SemanticIR preserve VALID READY without formal signal declarations
- [source-proof-migration-replays-classification-context](../knowledge/source-proof-migration-replays-classification-context.md)
  > how does SourceIR proof migration handle a classifier implementation change
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how does SpecForge avoid chip/vendor name lists in document classification (structural typed-surface counts + generic front-matter doc-type vocabulary only; ADR 0006)
- [protocol-state-machine-binding](../knowledge/protocol-state-machine-binding.md)
  > how does SpecForge bind a protocol state to its state machine
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how does SpecForge capture protocol actors/agents defined in prose (.3b/.8 extract_protocol_actors)
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how does SpecForge capture signals that are in prose not tables (I2C SDA/SCL)
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how does SpecForge cluster chip-spec PDFs by vendor/layout without hardcoding vendor names
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > how does SpecForge combine confidence across modalities or sources
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > how does SpecForge decide signal direction (input/output) in the emitted .isf
- [contested-priors](../knowledge/contested-priors.md)
  > how does SpecForge detect contradicting or conflicting priors
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > how does SpecForge detect two surfaces disagreeing about one quantity
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how does SpecForge determine what a chip-spec PDF is about / its purpose category
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > how does SpecForge distinguish a timing table category from a scalar min typ max layout
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > how does SpecForge distinguish legal conditions from protocol conditions
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > how does SpecForge distinguish protocol requests from product listing requests
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > how does SpecForge emit temporal rules or a bounded-eventually into .isf
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how does SpecForge extract register fields from tables
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > how does SpecForge flag vague or ambiguous spec language
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does SpecForge group a transaction's signals by channel
- [timing-scalar-rows-require-independent-cell-geometry](../knowledge/timing-scalar-rows-require-independent-cell-geometry.md)
  > how does SpecForge handle Docling clones of a table cell with col_span greater than one
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > how does SpecForge handle one transaction and one rule writing the same named-drive target
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > how does SpecForge measure source-to-IntentIR stage loss
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > how does SpecForge prevent analog dB limits from becoming digital timing intent
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > how does SpecForge prove a cited self-test contains a known-bad RED case
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > how does SpecForge prove a persisted corpus artifact is still what the current binary produces
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > how does SpecForge prove a reviewed paraphrase is parser equivalent
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does SpecForge read a PDF's front-matter / title / ToC to know its doc type
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > how does SpecForge recover a subject the model misspelled
- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > how does SpecForge recover a timing unit from a table caption
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > how does SpecForge relate to GoldMine Texada Pnueli Ammons
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > how does SpecForge resolve PSEL to PSELx in a temporal condition
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > how does SpecForge resolve a persisted path after the repository moves
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > how does SpecForge serialize repository owned paths
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > how does SpecForge stop a published count going stale under a green gate
- [required-residual-actionability-denominator](../knowledge/required-residual-actionability-denominator.md)
  > how does SpecForge stop a published gap reproduction from silently running no test
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > how does SpecForge tell a generic channel-signal template from a signal catalogue
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does SpecForge tell a guide from a real spec / report low-yield docs honestly
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > how does SpecForge tell a property table from a signal table
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > how does SpecForge verify an extracted claim semantically / catch hallucination
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > how does a VLM timing note become a FigureRegion
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > how does a deliberate normalized-bundle reclamation get authorized
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > how does a future SpecForge session retrieve and re-run a current claim
- [no-collection-may-declare-an-aggregate-below-its-own-legal-maximum](../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
  > how does a heterogeneous collection declare its legal maximum
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > how does a repair prove its change is isolated if it also rebuilds drifted documents
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how does a section-heading register avoid double-counting an existing register record
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > how does a timing diagram become an ActorContract in SemanticIR and IntentIR
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > how does a transaction absorb signals from its subsections (3.1.1 / 3.1.2)
- [table-row-obligation-binds-to-the-token-before-its-modal](../knowledge/table-row-obligation-binds-to-the-token-before-its-modal.md)
  > how does a width parameter's obligation become a signal constraint
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > how does an unknown-kind Continued from previous page table fragment get a kind
- [trajectory-controller-engine](../knowledge/trajectory-controller-engine.md)
  > how does automatic task ranking keep hard failures ahead of breadth work
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > how does behavioral genericity normalize renamed stable ids safely
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > how does build_transaction_phases distinguish a named phase from phase error or phase tolerance
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > how does chain currency distinguish a checked blocked adapter from an emitted ISF file
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how does claim evidence become stale
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how does collapse_section_header_register_identity decide same-register vs different-register
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > how does converge report integrated scheduled and omitted production capabilities
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > how does converge report per-document extraction quality after stabilization
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > how does de-fragmentation enable the recover-register-bits gate (b)
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > how does derive_isf_actor_name produce a valid HDL identifier ([A-Za-z_]\\w*)
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > how does derive_register_block_name parse a block out of a register-descriptions section heading
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > how does each active task-evidence index invoke its own contract
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > how does extract_discovered_state_value_from_text match a constraint value
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > how does ground_constraint recover a value the model did not echo
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > how does held out full capture refresh preserve alpha eligibility authority
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > how does ingest avoid being RAM-guard-aborted on a small/restricted machine
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > how does ingest avoid crashing the host when memory runs out
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > how does ingest avoid filling the disk on a very large PDF
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how does learn-priors harvest extraction-profile priors (multi-member clusters only, schema v6)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how does pattern reuse stay agnostic (ADR 0006) and honest (no fabrication / no overfitting)
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > how does production recover explicit inference antecedent state
- [register-record-access-and-table-provenance](../knowledge/register-record-access-and-table-provenance.md)
  > how does register access differ from register field access
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > how does retain_authoritative_interface_candidate_signals behave with an empty authority set
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > how does specforge capture a signal that is defined in prose not a signal table (SWP S1/S2)
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > how does specforge consolidate a Class-B agent fragment like Subordinate extends onto Subordinate
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > how does specforge consolidate a dense-prose agent fragment like host has or host to onto host
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > how does specforge currently capture transactions and why is it thin
- [table-row-obligation-binds-to-the-token-before-its-modal](../knowledge/table-row-obligation-binds-to-the-token-before-its-modal.md)
  > how does specforge decide whether a description cell constrains the row's signal
- [prose-pin-appositive-signal-capture](../knowledge/prose-pin-appositive-signal-capture.md)
  > how does specforge declare a signal mentioned only in prose
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > how does specforge drop a zero-evidence phantom actor like controller or agent
- [agnostic-quoted-mode-fsm](../knowledge/agnostic-quoted-mode-fsm.md)
  > how does specforge extract a CAN-style error-state FSM (error active / error passive / bus off)
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > how does specforge extract a single-word ALL-CAPS state machine (SWP ACTIVATED / DEACTIVATED / SUSPENDED)
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > how does specforge fold an X interface relation subject onto the bare agent X
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > how does specforge group a transaction's signals by phase (.2i: TransactionIntent.phase_membership, built in mint_named_transaction by intersecting anchor.signal_set with each TransactionPhaseRecord.signal_set; metadata, not .isf steps)
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > how does specforge handle PSEL vs PSELx (or HSEL vs HSELx)
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > how does specforge handle a signal table whose name column is not first
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > how does specforge model the JTAG TAP / SWD state machine (FSM)
- [swd-serial-frame-surface](../knowledge/swd-serial-frame-surface.md)
  > how does specforge model the SWD serial frame / packet
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > how does specforge read free disk space without a new dependency
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > how does specforge read system memory without a new dependency
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > how does specforge read total physical RAM without a new dependency
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > how does specforge recognise transaction phases (the <qualifier> phase prose gate, build_transaction_phases/derive_phase_name)
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how does specforge recover CAN's frame fields (SOF/Arbitration/Control/Data/CRC/ACK/EOF)
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > how does specforge reject non-signal constraint subjects (LICENSEE, AXI, RME, MPAM)
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > how does specforge reject prose-fragment non-agents like For / Then it / is recommended / ensures
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > how does specforge split a coordinated X and Y relation subject into both agents
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > how does specforge tell a trapped data row from a genuine multi-row column header without a list or case
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how does the EXTRACTOR-ARCHITECTURE run manifest enable cross-document clustering
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > how does the LLM-primary extractor avoid extracting a when/if/unless clause subject as an obligation
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > how does the LLM-primary extractor handle 'It is recommended' / 'It is permitted' / 'would be' frames
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > how does the SemanticIR Phase-2 role-term scan (build_actors ACTOR_TERMS) mint Class-C actors
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > how does the VLM understand tables / can a VLM read PDF tables
- [active-task-migration-transaction](../knowledge/active-task-migration-transaction.md)
  > how does the active task migration preserve non-ASCII legacy bytes
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > how does the adversarial PDF identity test preserve source bytes
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does the ambiguity gate keep channel membership boundary-precise (bar #3)
- [behavioral-semantic-negative-sensitivity](../knowledge/behavioral-semantic-negative-sensitivity.md)
  > how does the behavioral genericity gate prove negative-control sensitivity
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > how does the claim gate find ignored or untracked scratch producers
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how does the claim gate reject ignored or untracked scratch producers
- [trajectory-controller-engine](../knowledge/trajectory-controller-engine.md)
  > how does the controller prove a proposed task is task-tree owned
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > how does the corpus KB aggregate fixture page stay bounded as fixtures grow
- [corpus-task-bounded-active-root-and-evidence-parts](../decisions/0024-corpus-task-bounded-active-root-and-evidence-parts.md)
  > how does the corpus task preserve all evidence after containment
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > how does the current claim census prove no produced candidate is silent
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > how does the emitter decide a rule drive value is renderable (is_safe_isf_scalar_value — non-empty, whitespace-free; a prose value like 'the value that was presented on the ARLOOP signal' fails)
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > how does the extract-constraints-llm prompt express a validity requirement
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > how does the extractor avoid minting a constraint about a cross-referenced register field
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > how does the fused two-label column ACE5-Lite ACE5-LiteACP split
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > how does the held-out evaluator detect omission fabrication provenance loss and silent drops
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > how does the measurable corpus population grow
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > how does the no-re-ingest canonical promotion protocol stay in place
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > how does the published-assertion gate decide which surfaces are watched
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > how does the register reader recover a field name with no name column
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > how does the section-heading field reader tell a message container from a register container
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does the table-number grammar handle both B1.1 colon and A2-2 dash forms
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > how does the tiling gate keep register-bit recovery honest (no fabrication)
- [trajectory-controller-engine](../knowledge/trajectory-controller-engine.md)
  > how does the trajectory controller classify converging diverging stalled mixed and unmeasurable
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does validate classify a document / what is document_class
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > how does validate know an intent_ir is stale relative to its semantic_ir
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > how does validate reach the upstream artifact (carried semantic_ir_path / evidence_ir_path)
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does validate report registers_without_fields / registers_unresolved_width / signals_without_direction / unexplained_intent_bearing_tables
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > how erroneous are the canonical Pattern constraint surfaces on the persisted corpus
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > how expensive is reading the proof seal from the whole persisted corpus
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how flexible is the register model / what register-table shapes are handled
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > how good is prose signal capture / .3a quality
