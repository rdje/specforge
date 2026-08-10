# Knowledge questions — shard 0009

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > why do register bit-fields not appear in the emitted .isf (registers lower as opaque width-only storage vars)
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > why do some docs reach intent but not isf (honest block: no behavioral content to lower)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > why do the CCIX specs extract almost no register fields
- [llm-primary-constraint-dedup](../knowledge/llm-primary-constraint-dedup.md)
  > why do two constraints with different conditions not merge
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > why does .1b.i NOT strip a trailing conjunction (and/or) or X interface
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why does .2i not emit a universal address/data/response phase order (ordering is an honest residual per .2h)
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > why does ACE table_0275 stay flagged after presence capture
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > why does AHB not flip any signal to input under Manager perspective (sparse stale grounding — Manager's only graph inputs are HCLK/HRESETN, excluded as clock/reset)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > why does AMD IOMMU extract no register fields
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why does APB phase grouping stay minimal despite phase prose naming signals (.2c membership thinness — read/write_transfer carry only PCLK; a .2c-breadth lever, not a phase-cue one)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > why does AXI have 71 registers but zero composable resets (AXI register_records are encoding pseudo-tables — Valid and Ready signals / Resource plane number properties — fields with no bit positions and symbolic resets like -, False, AxPROT[1])
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > why does CAN serial_frame_fields use SerialFrameField with phase None and order = composition index
- [agnostic-quoted-mode-fsm](../knowledge/agnostic-quoted-mode-fsm.md)
  > why does CAN yield 0 from extract_protocol_states but 3 from the quoted-mode path
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > why does CORPUS-COVERAGE still say 32 refreshes when only one normalized bundle exists
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > why does DTI (ihi0088) have zero message_field_records
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > why does Docling re-ingest fail on Apple Silicon
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why does I2S SD remain a signal
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > why does Instruction group not count as an ns unit and why does SMIN not count as min
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > why does OpenCAPI Discovery Configuration no longer emit ISF
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why does RISC-V AIA capture 0 registers (its IMSIC/APLIC CSR intent is in prose conditional_rules/behaviors; no current register strategy matches RISC-V's CSR layout — the .10g <NAME>, bits [hi:lo] section-heading family fires only on ARM ihiXXXX arch specs)
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > why does SWP yield 0 from extract_protocol_states and extract_quoted_mode_states but 4 from the transition-bound path
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > why does USB 3.2 EvidenceIR panic on start byte index is not a char boundary
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why does USB 3.2 emit AT ENHANCED NO and USB as ISF signals
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why does USB 3.2 still emit hundreds of low confidence ISF outputs after the four false signals are removed
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > why does a PDF-VARIANT-DIGESTION commit subject id not appear fully qualified in the task source
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > why does a new SemanticIR have an empty gates array
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > why does a new SemanticIR have an empty phases array
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > why does a rebuild drop the extraction-quality gauge
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why does a register count NOT veto a wire protocol (wire-vs-structure weight dominance; AXI wire 401 >= struct 229)
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > why does a register doc captioned 'message fields' yield zero message fields
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > why does a sentence-period caption label yield nothing
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > why does a symbolic bit cell reject the whole table
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > why does a temporal antecedent use PSELX not PSEL
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > why does a timing_parameter table produce 0 timing_constraints when it clearly has rows (I2S table_0004, SMBus table_0012)
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > why does adding a new extractor feel fragile / erratic (god-orchestrator + inline dedup loops)
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > why does adding an extractor change every doc's extraction manifest
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > why does an .isf enum get dropped / held out of the emitted .isf
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why does an .isf rule get dropped when it conflicts with an unconditional rule on the same signal
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > why does an .isf rule get dropped when its drive value is prose / not a (port expr)
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > why does an alphabetic constraint value require a word boundary but a numeric value does not
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why does an evidence/converge build OOM on a doc with many multi-word actor names
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > why does an in-body (drive NAME) need a top-level named-drive definition (drive 'X' not defined)
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > why does an incidental 'can' in a source block not drop its other constraints
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > why does condition_text matter for the NLI claim
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > why does constraint promotion run outside the convergence loop
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > why does decoder go from 0/0 to connected (decoder also consolidated)
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > why does dropping phantom actors leave the .isf and WIRE-BASED-100 unchanged
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > why does ingest still generate page images if it does not save them
- [repo-local-temp-docling-test-collision](../knowledge/repo-local-temp-docling-test-collision.md)
  > why does inspect_docling_runtime PATH probe select RepoLocalVenv in tests
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > why does mdbook test interpret ISF and console examples as Rust
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > why does one register appear as several RegisterRecords / how are split register-field tables merged
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > why does rebuilding the same SourceIR give a different evidence_ir.json (non-determinism)
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > why does recover-register-bits still recover 0 bits after the plumbing is complete
- [knowledge-map-architecture-location](../knowledge/knowledge-map-architecture-location.md)
  > why does root KNOWLEDGE_MAP_ARCHITECTURE.md not exist
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why does setportfeature port over current become the USB 3.2 adapter actor
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > why does shall be 0h still bind the value 0 but shall be non-zero does not bind NO
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > why does slicing idx minus 2 before signal panic
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > why does specforge split on 'and' but not 'or' for a coordinated actor subject
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why does the .isf emit a generic (type TABLE (bits N)) enum / what is the TABLE mega-enum
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > why does the AArch64 External Debug Guide emit no ISF
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > why does the ADI spec produce so few signals and so much garbage
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > why does the Claude Read tool refuse some PDFs / report password-protected
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > why does the CoreSight Base System Architecture emit no ISF
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > why does the D4.5 CLAIMSET (no block token) stay a residual under .10i
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > why does the DTI / trace-bus .isf fail FSMGen --strict --check (OperandContract: a value literal wider than the declared signal width)
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > why does the Introducing CoreSight guide emit no ISF
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > why does the MPAM 'must be included' sentence extract nothing
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > why does the OpenCAPI AFU Address Space Usage note emit no ISF
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > why does the OpenCAPI Certified Definition emit no ISF
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > why does the OpenCAPI Certified engineering note emit no ISF
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > why does the OpenCAPI Ready Definition emit no ISF
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > why does the OpenCAPI Ready engineering note emit no ISF
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > why does the USB4 Connection Manager Guide emit no ISF
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > why does the VLM misread register-diagram bit positions and how is it fixed
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > why does the batch size depend on total RAM instead of free memory
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > why does the eMMC (JEDEC) IntentIR explode to 153 actors while HBM2 consolidates to 38
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > why does the eMMC actor count drop 153 to 138 after .1c.i
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why does the emitted .isf default ~98% of signals to output and width 1
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why does the emitted .isf module name get sanitized / how is the (actor <name>) label derived
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > why does the fact-card file count differ from the Knowledge Map fact count
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > why does the feedback protocol self test block derived state closure
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > why does the pre-commit hook run check_doctrines.sh
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why does the project README get spurious ahb_transfer/apb_transfer transactions
- [cortex-a76-optimization-guide-refresh-is-authority-empty](../knowledge/cortex-a76-optimization-guide-refresh-is-authority-empty.md)
  > why does the refreshed Cortex-A76 Software Optimization Guide emit no ISF
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why does the same token AXI interface merge in one doc but not another
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why does the same-guard dedup_conflicting_rules miss a conflict between an unconditional rule and a guarded rule
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > why does the tracked validation snapshot differ from current generated IntentIR reports
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > why does tilelink have 39 relations in evidence_ir but 0 in intent_ir
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > why doesn't SpecForge emit (contract eventually) anymore
- [stable-obligation-phase-scoped-residual](../knowledge/stable-obligation-phase-scoped-residual.md)
  > why doesn't SpecForge lower stability obligations to (assert (stable sig))
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > why doesn't SpecForge use TLA+
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > why doesn't fusion use the minimum confidence
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > why doesn't the NLI-oracle conformal calibration produce a threshold
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > why doesn't the disk estimate use the page count
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > why doesn't the temporal_rule eval reach precision 1.0
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why don't RISC-V Debug register bit-fields reach .isf (all 179 fields are UNLOCATED — field_name/access/reset/description captured but 0 carry bits_high/bits_low/bit_width; the field_table strategy did not parse the bit-layout column)
- [swd-serial-frame-surface](../knowledge/swd-serial-frame-surface.md)
  > why don't parallel buses get serial_frame_fields
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > why drop the enum instead of width/radix-qualifying it (the value is a binary code mis-read as a decimal — the emitter can't recover the radix without fabricating; honest residual over fabrication)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > why is ATID emitted (width 1) when the IntentIR grounds width 7 (the emitter's first-seen signal dedup isf_ir.rs:696-700 takes the first signal_records entry (w=None→1) and skips the later w=7 record; the .2a.i recovery only falls back to actor_ports, and ATID has none)
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > why is AXI transmitter / SWD host / GIC arbiter kept but APB controller / AHB agent dropped
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is AXI/SWD per-signal phase membership empty (document absence — AXI 0/4, SWD 0/63 signal↔phase co-occurrence; phases described abstractly/by-packet, not per declared wire — not an extraction gap)
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > why is CORPUS-PATTERN-REUSE.3b.3b (activate-only consume) deferred / not built
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why is CPU-ISA / PHY only recognized from front-matter (no distinct structural signature; cat 5 vs cat 6 indistinguishable by structure)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why is DTI DOWNSTREAM not preserved as a heuristic only wire
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > why is For / Then it / is recommended / next / HPROT bit / TREADY input minted as an actor
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is KG-ISF-TRANSACTIONS.2i parked/blocked (waiting on FSMGEN to say how to lower phase membership without fabricating drive values or step order)
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > why is MEMORY-BOUNDED-INGEST.5 summary streaming deferred
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why is NVMe register field_name a bit-range and how is the mnemonic found in the description
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > why is Reset value not a semantic phase
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > why is SWD per-signal phase membership degenerate / empty
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > why is SWD/ADI hard / different from APB AHB AXI
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is SWIO not captured as a third SWP signal
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > why is SpecForge called forward specification mining
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is Subordinate interface merged to Subordinate but GIC CPU interface kept intact
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > why is TEMPLATE excluded from the task catalog
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why is Vital Product Data (VPD) not a signal
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why is Wishbone memory output data (DO) not a port
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > why is a CSW@MEM-AP / CLAIMSET@AP qualified register name emitted, and is it .isf-safe
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > why is a field name accepted or rejected from a description cell
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > why is a gerund after while/when not a condition (action coordination)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > why is a membership-derived (sample) body NOT faithful even though it is FSMGen-accepted
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > why is a pure-hex-literal subject filter unsafe (CBA, BADD)
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > why is a sentence containing when not necessarily a semantic gate
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why is bus not a valid parenthetical single-wire head
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > why is category 3 (platform/system-IP) topology not lowered to .isf (ISF has no static-topology construct + the emit is single-initiator-actor; cross-component topology is structurally absent from the emit by design — KG-ISF-COMPLETENESS.2a.ii)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why is category 4 (CPU ISA) ISF-thin (the only lowerable cat-4 intent is CSRs, which are registers; their bit-fields are UNLOCATED so they don't reach .isf — an extraction-recall gap, not a missing ISF construct)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > why is channel membership metadata-only and not lowered to .isf
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > why is conformal calibration still blocked at CHI scale
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > why is corpus coverage refresh 34 blocked before ingest
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is descriptor apposition (signal NAME / NAME signal) NOT used to capture signal names
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > why is page_image_path null / None for a large document
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is plural-rejection unsafe for phase names (access ends in ss)
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > why is section-title boilerplate filtering insufficient
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why is signal direction hard to lower faithfully to a single .isf module
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > why is statement_0223 still a normative statement
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > why is the .isf direction default (output) FSMGen-neutral / not a faithful-lowering gap
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > why is the .isf module named after the initiator (manager / requester / debugger) instead of actors.first()
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > why is the AHB eval baseline wrong or stale
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > why is the Docling table capture of a register diagram unreliable
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > why is the FSM important for SWD/JTAG
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > why is the NLI framing better than free-form labeling
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > why is the PSEL antecedent dropped in a temporal rule
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > why is the SpecForge fact-card catalog almost out of capacity
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why is the USB 3.2 adapter syntactically valid but semantically untrustworthy
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is the X interface strip gated on X being a connected agent in this doc
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why is the _WIDTH enum-member leak a real fidelity defect (.5.iii: 7 _WIDTH members in real-signal-named enums in AXI gold ihi0022_l reach manager.isf — (BRESP (BRESP_WIDTH 0)(OKAY 0)…) duplicates value 0, (RRESP (RRESP_WIDTH 0)) REPLACES the real RRESP codes, (AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1)) pure junk; a width PARAMETER 'Enum BRESP BRESP_WIDTH = 0.' mis-read as
  > an encoding VALUE — a false bar-#6 fact, unscored by WIRE-BASED-100 since enums are emitter-orthogonal)
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is the abbreviation-table I/O-expansion not used to capture signals (MMIO/DMA/IOVA garbage)
