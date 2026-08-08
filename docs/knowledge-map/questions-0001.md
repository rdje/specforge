# Knowledge questions — shard 0001

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > Cannot convert a MPS Tensor to float64
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > EXTRACTION-QUALITY-GAUGE.3f what is the alphabetic-value word-boundary gate
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > EXTRACTION-QUALITY-GAUGE.3g what is the dotted-cross-reference spurious-subject gate
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > are APB tables 0016 0017 0018 a real catalog miss
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > are SWCLK and SWDIO extracted / declared
- [stable-obligation-phase-scoped-residual](../knowledge/stable-obligation-phase-scoped-residual.md)
  > are SignalStable obligations representable in .isf
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > are any SpecForge requests to FSMGen currently open
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > are behavior or temporal rules silently dropped to the .isf on the broader 78-doc corpus
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > are byte-location placement tables registers or structures
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > are currently-strict-clean .isf affected by the overlap drop (no — a clean doc cannot contain an unconditional-overlap config or FSMGen would already reject it → byte-identical by construction; 100/107 emitted .isf unchanged)
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > are description bracket slices like Store Data[63:32] positions or values
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > are other docs affected by the rule-drive-value gate (no — only ihi0022_h_c carries a prose-valued rule corpus-wide; every other emit is byte-identical)
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > are presence codes like Y N O C OC ever interpreted
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > are temporal rules silently dropped when lowering IntentIR to .isf
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > are temporal_invariants / conditional_rules / signal_constraints silently dropped at ISF lowering
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > are the 'unknown diagram_kind' visual assets a capture gap
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > are the APB signals PCLK PADDR PWDATA the parity-check PADDRCHK extracted
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > are the AXI/SWD phase-header tables a signal→phase cue (no — AXI table_0150 is a coherency-sequence table, SWD table_0057 is an ACK-response table; neither maps declared signals to phases)
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > are the degenerate PSEL-header or WIDTH-subject temporal rules a live bug
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > are the disconnected/unconnected KG agents a recoverable relation gap or false positives
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > are the per-page full-res page images read by any downstream consumer
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > are the register-bit-recovery plumbing gaps (unknown diagrams, fragmented field tables) fixed
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > are the wire-gold .isf affected by the module-name sanitization (no — byte-identical; their names are pure alphanumeric, so the allowlist is a no-op)
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > are the wire-gold / other .isf affected by the enum emit gate (no — byte-identical; only HBM2 hbm.isf changes corpus-wide; the criterion never flags a legit decimal)
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > can I build semantic/intent without the normalized/ bundle (yes — only evidence_ir.json is needed)
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > can ISF model an explicit state machine / FSM (proven)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > can SpecForge lower a transaction's signal-set membership into the ISF transaction BODY
- [stable-obligation-phase-scoped-residual](../knowledge/stable-obligation-phase-scoped-residual.md)
  > can SpecForge use FSMGen's (stable …) sampled-value predicate
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > can WIRE-BASED-100 reach 100% on SWD the same way as the parallel buses
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > can specforge emit a symbolic signal width like (width ADDR_WIDTH) to the .isf
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > can the Docling-flattened register diagram table be parsed into bits_high/bits_low (no — garbled: wrong explicit positions, dropped field bands, doubled cells, two stacked halves, or symbolic XLEN-relative positions)
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > can the prose bus-line signal lever (PDF-VARIANT-DIGESTION.9.10) be cluster-scoped as a CORPUS-PATTERN-REUSE opt-in extractor
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > can the subject snap fabricate or rewrite a signal name
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > can typed-surface counts alone determine a PDF's purpose category (no — 2 vs 3 not separable, ISA has no signature, 5 vs 6 indistinguishable)
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > could a huge PDF OOM the downstream stages even though ingest is bounded
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > default model for the ollama provider
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > did DOC-INTENT-TAXONOMY.4d.i build a deterministic RISC-V CSR bit-position parser (NO — measured non-viable: bits live in the image, flattened tables garbled/XLEN-symbolic, ~0 correct recovery + fabrication risk; honest residual, no Rust code, no FR)
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > did FSMGen accept the field-structured-storage FR (YES 2026-06-22 — accepted then SHIPPED it via ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1/.2, pin d327129b7; FSMGEN-REFRESH-INTEGRATE-4 accepted, -5 shipped)
- [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md)
  > did a second annotator validate seed_apb.json
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > did doubling the corpus (36->78) introduce a new ISF lowering silent-drop gap
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > did the qwen2.5vl VLM recover a groundable AXI signal-to-phase mapping (no — contradictory, redundant, hallucinated signal semantics)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > do RISC-V CSRs map onto the ISF register/storage abstraction (YES — RISC-V Debug captures its 44 CSRs as register_records; FSMGen titles (storage (var … (fields …))) the register-map/CSR construct, 13a-actor-interface.md:419/:468)
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > do SMBus / I2S / I2C cluster into a derived serial-bus family
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > do all corpus docs build through the pipeline without failure
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > do any chip-spec PDFs need a real password (no)
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > do evidence/semantic/intent stream source_ir.json or load it all into memory
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > do field constraints pass the same grounding gates as signal constraints
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > do host-local source libraries define tracked corpus membership
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > do signal presence records mint signals or declarations
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > do specforge extractors override applies_to / are any self-disabled opt-in
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > do text-reasoning tasks need a vision model
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > do the Pattern and Nlp extraction tiers find the same constraints
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > do the emergent clusters actually track real vendor/layout families
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > do the register_bits.rs tiling gates validate field order (NO — only width-sum + name-multiset; a row-jumbled flattened table could pass both gates with WRONG bits, so a deterministic-table reader is strictly more dangerous than the VLM front-end)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > do the wire docs (APB/AHB/AXI/SWD) change when register reset is lowered to ISF (no — ZERO composable resets, .isf byte-identical, WIRE-BASED-100 holds trivially)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > do the wire-gold .isf carry over-width value literals (only AXI ihi0022_l has one — AWCMO; and AXI already fails strict on the orthogonal (port expr) error; APB/AHB/SWD have none; WIRE-BASED-100 measures extraction F1 not .isf bytes so it is orthogonal)
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > do transactions lower to .isf (only those with composed steps; signal-set/channel/phase membership is recognised-but-unlowered)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > does (sample S) work for an interface OUTPUT signal too (yes — FSMGen does not gate sample on direction)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > does .2i per-phase grouping change the emitted .isf (no — phase_membership is IntentIR metadata, the emitter lowers steps not it; byte-identical on all 4 wire docs)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > does .2m change the emitted .isf or the WIRE-BASED-100 surfaces (no — provably orthogonal)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > does .5.i change anything besides enums (yes, beneficially — dropped Enum statements leave discovered_values, so off-gold junk value-constraints derived from junk-enum members also disappear, e.g. AXI ACTIVATEACK A -> grounded ACTIVATEACK 1; distinct constraint facts identical, WIRE-BASED-100 unaffected)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > does CPU-ISA (category 4) intent need a new ISF construct or does it map onto the existing register/storage abstraction (CSRs map onto storage/register — no new construct; non-register ISA semantics are honest non-targets)
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > does ExtractionContext carry a cross-document cluster profile
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does FSMGen --strict --check use or validate signal direction (input vs output)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > does FSMGen 030f8c273 accept a value-free (sample S as s) transaction body
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does FSMGen accept a symbolic (width PARAM) or only a concrete integer width
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > does FSMGen support a register reset value in storage (yes — (storage (var NAME (width N) [(reset V)])) is shipped per 13k:42 + 13m:48-68; optional, in-width non-negative int, omission = all-0s byte-identical, over-width/non-integer fails closed)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does FSMGen's multi-actor ATL frontier provide a home for a connectivity netlist (NO — the ATL backlog wires children GENERATED from transaction composition spawn/do; it is behavioral orchestration, not a declarative static IP-interconnect netlist; verified 14-feature-backlog.md)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does SpecForge capture component topology / connectivity for platform docs (YES — a typed signal_connectivity producer->consumer graph + infrastructure_signals clock/reset distribution; correcting the .2 'hint-level' to 'captured-but-sparse-and-unlowered')
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > does SpecForge classify Markdown inside the FSMGen submodule
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > does SpecForge cycle-schedule the FSM (no — FSMGen does)
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > does SpecForge detect implementation-defined or TBD or and/or
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does SpecForge emit register bit-fields to ISF now (YES — DOC-INTENT-TAXONOMY.4a.ii: the storage var carries a (fields (field …)) block; 6,570 fields / 2,531 registers / 24 docs, was 0; 4 wire golds byte-identical; 0 new fsmgen --strict diagnostics)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > does SpecForge lower register reset values into the .isf (YES as of ISF-REGISTER-RESET-EMIT.2/.3 — composed from per-field reset_value and emitted at the true register width; it was dropped at the emit boundary before)
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > does SpecForge model-check temporal properties
- [contested-priors](../knowledge/contested-priors.md)
  > does SpecForge revise or decay priors
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > does SpecForge use LTL CTL or TLA+
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > does SpecForge use the full scope of a page's visual information
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > does a deterministic semantic->intent rebuild recover lost actor_signal_relations
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > does a replaced constraint surface get polarity refinement (apply_persisted_polarity_to_constraints)
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > does adaptive batch sizing change the ingest output / break byte-identity
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > does agreement between sources boost confidence
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > does constrained contract extraction ship code
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > does converge re-ingest the PDF every run
- [contested-priors](../knowledge/contested-priors.md)
  > does corpus prior memory only accrete
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > does emitting (input) signals break fsmgen --strict (no — 0 new diagnostics; drives are suppressed for inputs)
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > does encryption block the VLM from reading tables (no)
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > does enrich / audit-extraction / recover-register-bits read full-page images or region images
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > does eval-extraction rebuild evidence or load the persisted file
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > does every temporal_rule reach the .isf or a residual
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > does lowering cat-3 topology need only an ISF construct or also a multi-actor emit (also a multi-actor emit — ISF is per-actor / one .isf = one FSMGen module and SpecForge's emit is single-initiator-actor; a declarative cross-component netlist is an architectural change, decided WITH FSMGen only after capture-recall clears the bar — not today)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does platform/system-IP (category 3) topology intent need a new ISF construct or map onto an existing one (ISF has NO declarative static-topology/connectivity construct — composition is transaction-level only; decision deferred to a capture-recall measurement .4c.i before any FR)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > does register extraction require the table_kind register classification
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > does sanitizing the module name break initiator port matching (no — from_intent_ir re-derives the initiator raw; actor_name is only the label)
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > does specforge handle coordinated drive/read objects (X drives A and B)
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > does specforge handle passive voice for actor-signal relations (X is driven by Y)
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > does specforge write to the input path I pass or to a canonical generated path
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > does the ISF adapter lower actor-relative direction
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > does the NLI verifier actually catch real extraction errors
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > does the agent-identity gate keep Class-B fragments like Subordinate extends
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does the current FSMGen ISF support named bit-fields inside a storage var (NO — opaque (var NAME (width N)) only on pin 030f8c273; set-field/extract are runtime ops not a declaration)
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does the emitted .isf signal direction affect FSMGen downstream correctness
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > does the live-document registry reject unknown fields oversized arrays or oversized scalars
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > does the stage-staleness detector false-fire on register/command docs with 0 relations (no — both empty)
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > does the trapped-row gap-fill mint duplicate signal declarations
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > does validate detect a stale downstream artifact that silently dropped relations
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > does validate report message_field_records
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > fresh empirical reconfirmation of the .2i body-emission parking on the current 030f8c273 binary
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > how are <NAME>, bit [N] / <NAME>, bits [hi:lo] section-heading field defs parsed
- [swd-serial-frame-surface](../knowledge/swd-serial-frame-surface.md)
  > how are ACK WDATA RDATA DATAIN bit-widths extracted
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > how are CHANGES DEVELOPMENT_NOTES LIVE_ACHIEVEMENT_STATUS and RUST_CODEBASE_ANALYSIS split into records
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > how are Docling-fragmented register field tables de-fragmented without fabricating a field set
- [agnostic-quoted-mode-fsm](../knowledge/agnostic-quoted-mode-fsm.md)
  > how are FSM states recovered when the protocol quotes them as node modes instead of <Name> state
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > how are GICD_CHIPR<n> / TCU_NODE_CTRL n array registers named
- [swd-serial-frame-surface](../knowledge/swd-serial-frame-surface.md)
  > how are NAME[hi:lo] bit-ranges parsed into frame fields
- [prose-pin-appositive-signal-capture](../knowledge/prose-pin-appositive-signal-capture.md)
  > how are SWCLK and SWDIO captured if they are not in a signal table
- [live-document-containment-and-data-locality](../decisions/0007-live-document-containment-and-data-locality.md)
  > how are SpecForge live documents kept bounded
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > how are TAP states (Shift-DR, Run-Test/Idle, Test-Logic-Reset) extracted
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > how are TRM register bit assignments tables without access/reset columns handled
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > how are bit location | register description | attributes tables extracted
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > how are bits | name | function tables extracted
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how are byte location | size | register description tables extracted
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > how are caption-less page fragments of a split table stitched together
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how are continuation table fragments (B1.1 Continued from previous page) chained to a channel role
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > how are doctrines enforced in specforge
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > how are document keys in the source PDF registry derived from filenames
- [llm-primary-constraint-dedup](../knowledge/llm-primary-constraint-dedup.md)
  > how are duplicate signal constraints deduplicated in the LLM-primary extractor
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how are extraction-profile priors looked up (extraction_profile_priors_for signature-subset match)
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > how are live-document ceiling increases and immutable debt baselines tested
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > how are message fields written as section headings extracted
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how are message-field tables distinguished from register-field tables
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how are multi-word field names like Validation Bits or FRU ID recovered
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how are offset-suffixed bit cells like 31:28 +04 extracted
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > how are per-instance indexed signals (PSELx HSELx) referenced in prose handled
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > how are per-state actions captured
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > how are register bit-fields admitted to the ISF (fields …) block (structural fail-closed: located fields only; drop a sanitized-name collision group; non-overlapping survivors else whole-register fail-closed; access normalized to FSMGen's 10-token set else omit; field reset = parent reset slice; enum members that fit the width — ADR-0006, no name list)
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how are register fields written as section headings extracted
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > how are rotated version matrices remapped for presence capture
- [prose-pin-appositive-signal-capture](../knowledge/prose-pin-appositive-signal-capture.md)
  > how are serial/architecture spec interface signals added to the catalog
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > how are signal presence matrices captured into typed records
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > how are signal presence matrices with signals trapped in header rows handled
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > how are spacing-artifact field names and the unit word Bits handled
- [temporal-rule-ltl-rendering](../knowledge/temporal-rule-ltl-rendering.md)
  > how are temporal rules expressed as LTL or MTL
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > how are the three FSM grammars (SWD hyphen, quoted-mode, single-word) distinguished
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > how are two genuinely-different registers sharing a mnemonic (MEM-AP CSW vs JTAG-AP CSW) recovered instead of dropped
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > how are two-column bits | description tables extracted
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > how are unknown tables reclassified by the VLM
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > how big does source_ir.json get / how does it scale with page count
