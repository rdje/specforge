# Knowledge questions — shard 0002

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > how are two-column bits | description tables extracted
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > how are unknown tables reclassified by the VLM
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > how big does source_ir.json get / how does it scale with page count
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > how can I browse every SpecForge knowledge fact card by id or title
- [canonical-collection-catalogs](../knowledge/canonical-collection-catalogs.md)
  > how can a collection use a membership index outside its own surface
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how complete is a document's extracted intent / what is the per-doc completeness gauge
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > how complete is register-IP / platform-IP / CPU-ISA ISF lowering
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > how dense is SpecForge's captured component topology on cat-3 docs vs cat-1 wire docs (cat-3 = 0.355 edges/actor + 24% both-endpoint; cat-1 wire baseline = 4.108 edges/actor + 85% both-endpoint — the SAME signal_connectivity surface is ~12x denser and fully-connected on wire docs, so the surface is capable; the shortfall is capture-recall on platform TRMs)
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > how did APB temporal reach 100% (WIRE-BASED-100.4)
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > how did the AHB decoder become connected (Subordinate and decoder read HADDR)
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > how do Continued from previous page fragments find their home
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > how do I add a new enforced doctrine / doctrine check
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > how do I file an FSMGen bug report or feature request
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > how do I get the clean LLM-primary constraint surface onto the canonical artifacts
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > how do I make the NLI verifier actively change extraction / demote claims
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > how do I measure the extraction-quality gauge before and after a canonical promotion
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > how do I promote a document's constraint surface on canonical without re-ingesting the PDF
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > how do I run a Docling ingest or re-ingest on this machine
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how do I see which ingested PDFs form structural families (the corpus-cluster command)
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > how do I verify every task tree is linked exactly once
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > how do I waive or range-scope the task-acceptance check
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how do byte-granular page fragments chain (offset plus size adjacency)
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how do continuation tables (Table B2.2 Continued) merge into one container
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how do dword-relative page fragments chain
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > how do packet/flit protocols (CHI-class) declare message fields vs signals
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how do you audit registers/signals against the table image with the VLM
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how does .10g differ from .10f (register vs message routing)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how does / will SpecForge reuse extraction patterns across different PDFs
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > how does FSMGen decide a value literal's width (by notation digit count — 0x7D=8 bits, 0b00=2 bits — NOT by value; it requires an exact width-cast W'… match, no implicit truncation/extension; a bare decimal is unsized and fits any width)
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > how does FSMGen decide two rule data-writes conflict (same target, different value, NOT compatible/disjoint/priority/resource resolved) and when is a guard proven disjoint (_condition_terms_prove_disjoint: shared eq: signal with different values; an absent/empty condition is NEVER proven disjoint)
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how does SpecForge avoid chip/vendor name lists in document classification (structural typed-surface counts + generic front-matter doc-type vocabulary only; ADR 0006)
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
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how does SpecForge determine what a chip-spec PDF is about / its purpose category
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > how does SpecForge emit temporal rules or a bounded-eventually into .isf
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how does SpecForge extract register fields from tables
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > how does SpecForge flag vague or ambiguous spec language
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does SpecForge group a transaction's signals by channel
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does SpecForge read a PDF's front-matter / title / ToC to know its doc type
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > how does SpecForge recover a subject the model misspelled
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > how does SpecForge relate to GoldMine Texada Pnueli Ammons
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does SpecForge tell a guide from a real spec / report low-yield docs honestly
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > how does SpecForge verify an extracted claim semantically / catch hallucination
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how does a section-heading register avoid double-counting an existing register record
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > how does a transaction absorb signals from its subsections (3.1.1 / 3.1.2)
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > how does an unknown-kind Continued from previous page table fragment get a kind
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how does collapse_section_header_register_identity decide same-register vs different-register
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > how does converge report per-document extraction quality after stabilization
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > how does de-fragmentation enable the recover-register-bits gate (b)
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > how does derive_isf_actor_name produce a valid HDL identifier ([A-Za-z_]\\w*)
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > how does derive_register_block_name parse a block out of a register-descriptions section heading
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > how does extract_discovered_state_value_from_text match a constraint value
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > how does ground_constraint recover a value the model did not echo
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
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > how does specforge capture a signal that is defined in prose not a signal table (SWP S1/S2)
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > how does specforge consolidate a Class-B agent fragment like Subordinate extends onto Subordinate
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > how does specforge consolidate a dense-prose agent fragment like host has or host to onto host
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > how does specforge currently capture transactions and why is it thin
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
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does the ambiguity gate keep channel membership boundary-precise (bar #3)
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > how does the corpus KB aggregate fixture page stay bounded as fixtures grow
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > how does the emitter decide a rule drive value is renderable (is_safe_isf_scalar_value — non-empty, whitespace-free; a prose value like 'the value that was presented on the ARLOOP signal' fails)
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > how does the extract-constraints-llm prompt express a validity requirement
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > how does the extractor avoid minting a constraint about a cross-referenced register field
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > how does the fused two-label column ACE5-Lite ACE5-LiteACP split
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > how does the no-re-ingest canonical promotion protocol stay in place
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > how does the register reader recover a field name with no name column
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > how does the section-heading field reader tell a message container from a register container
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does the table-number grammar handle both B1.1 colon and A2-2 dash forms
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > how does the tiling gate keep register-bit recovery honest (no fabrication)
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
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how flexible is the register model / what register-table shapes are handled
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > how good is prose signal capture / .3a quality
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > how is AXI organized / what are the AXI channels
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how is AXI per-signal channel membership recovered without a VLM
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > how is AXI per-signal phase membership recoverable without a VLM
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > how is Gap A (register bit-fields) related to Gap B (message-field structures) — same missing ISF abstraction (named-field packed layout); Gap B also lacks an Evidence->Intent carrier (no message_field key in intent.rs)
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > how is SourceIr loaded from disk by downstream commands
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > how is a Reg.Field cross-reference distinguished from a real constraint subject
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > how is a claim's grounding checked beyond a string match
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
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how is a message field's width kept honest (per-variant widths stay None)
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > how is a promoted constraint surface visible in the extraction manifest
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how is a register mnemonic reused across access-port blocks (AUTHSTATUS/CSW/IDR/DEVARCH/CLAIMSET) recovered instead of dropped
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how is a register name recovered from a section heading (RISC-V dmstatus/dmcontrol)
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how is a register-field mnemonic recovered when the name column is a bit-range (NVMe)
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how is an under-extracted spec distinguished from a true guide (evidence_document_underextracted_spec)
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > how is an unless/except exception clause handled in a temporal condition
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > how is corpus SOURCE_PDF_REGISTRY currentness checked
- [corpus-kb-managed-currentness](../knowledge/corpus-kb-managed-currentness.md)
  > how is corpus_kb currentness checked
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > how is docs TASK_TREE kept complete without mirroring task history
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > how is docs knowledge INDEX kept complete
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how is prose signal over-capture prevented (no garbage)
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > how is register-field extraction quality measured / scored
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > how is the .5.ii member-quality gate designed / what did the .5.ii calibration find (measured 2026-06-24 read-only over 78 docs/561 enums/12509 members: the gate is PER-MEMBER not per-enum — a whole-enum drop destroys AXI BRESP's real codes OKAY/EXOKAY/SLVERR/DECERR which are FUSED with prose fragments in one conflated enum; value-restart is NOT a junk signal — AHB HPROT
  > restarts but every member is a clean identifier. The load-bearing signal is per-member NAME shape: an English sentence-SPINE token marks a prose fragment. Land a per-member sentence-spine fragment drop at synthesize_encoding_declarations_for_enum)
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > how is the EvidenceIR extractor path / extraction layer structured and wired
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > how is the FSMGen feedback channel kept bounded without losing old requests and responses
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > how is the NLI gate tested without Ollama
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how is the SWD FSM/frame derivation scored (not constraints/relations/temporal)
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how is the agent-definition grammar kept garbage-free without a fragile noun denylist
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how is the composition-frame grammar kept free of corpus false positives
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > how is the corpus distributed across the document intent categories
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > how is the ingest DISK footprint bounded for very large PDFs
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > how is the live-document containment checker tested
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > how is the page-range batch size chosen / adapted
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how is the precision of the broadened (non-gold) extraction measured / estimated
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > how is the prose definitional signal grammar kept garbage-free without a denylist (ADR 0006)
