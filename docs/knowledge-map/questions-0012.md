# Knowledge questions — shard 0012

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > why do four corpus documents have no document_class
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > why do generated IR files contain the old SpecForge repository path
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > why do i / its stay out of the non-actor function-word list (GIC ITS, the letter I)
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > why do many conditional_rules not lower to an ISF (rule) (they name no signal obligation, an undeclared signal, a placeholder action, or only a bare modal shall/must with no concrete value/level — lowering would fabricate the obligation)
- [spec-to-intent-category-contract](../knowledge/spec-to-intent-category-contract.md)
  > why do message fields prevent a wire or register category completeness claim
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why do message/flit fields count as wire intent only when there is no register map (CHI/DTI reg=0 vs NVMe/AMD/CCIX reg>0)
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > why do nested register views (AUTHSTATUS) collapse but disjoint ones (MEM-AP CSW vs JTAG-AP CSW) stay a residual
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > why do nvme / tilelink / wbspec / i2c / ccix / vt-d / iommu have zero actor_signal_relations
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > why do only 36 of 79 ingested docs reach IntentIR / .isf
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > why do plain English words like Address or Vector become field names under the bracket frame
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > why do producer/consumer/receiver/transmitter actors carry 0 ports and 0 relations
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > why do register bit-fields not appear in the emitted .isf (registers lower as opaque width-only storage vars)
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > why do some docs reach intent but not isf (honest block: no behavioral content to lower)
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > why do source derived ids need field aware alpha normalization
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > why do the CCIX specs extract almost no register fields
- [llm-primary-constraint-dedup](../knowledge/llm-primary-constraint-dedup.md)
  > why do two constraints with different conditions not merge
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > why do unchanged PDF proof digests differ between isolated scratch roots
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
- [protocol-evidence-is-generic-and-document-derived](../decisions/0035-protocol-evidence-is-generic-and-document-derived.md)
  > why does EvidenceIR schema 2 clear old protocol records
- [cat3-topology-fsmgen-actor-network-reassessment](../knowledge/cat3-topology-fsmgen-actor-network-reassessment.md)
  > why does FSMGen actor-network support not erase the cat3 topology capture-recall gate
- [reviewed-population-clean-replay-carrier-regression](../knowledge/reviewed-population-clean-replay-carrier-regression.md)
  > why does GIC 400 emit fifteen registers with missing access
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why does I2S SD remain a signal
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > why does Instruction group not count as an ns unit and why does SMIN not count as min
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > why does OpenCAPI Discovery Configuration no longer emit ISF
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why does RISC-V AIA capture 0 registers (its IMSIC/APLIC CSR intent is in prose conditional_rules/behaviors; no current register strategy matches RISC-V's CSR layout — the .10g <NAME>, bits [hi:lo] section-heading family fires only on ARM ihiXXXX arch specs)
- [a-bounded-snapshot-bounds-its-sections-not-just-its-file](../decisions/0031-a-bounded-snapshot-bounds-its-sections-not-just-its-file.md)
  > why does ROADMAP.md have per-section line bounds
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > why does SWP yield 0 from extract_protocol_states and extract_quoted_mode_states but 4 from the transition-bound path
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > why does SourceIR carry CLK_I but EvidenceIR carries CLK\\_I
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > why does SpecForge keep normalized bundles instead of reclaiming them
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > why does SpecForge retain normalized bundles instead of reclaiming them after a refresh
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > why does USB 3.2 EvidenceIR panic on start byte index is not a char boundary
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > why does USB 3.2 declare no signals
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why does USB 3.2 emit AT ENHANCED NO and USB as ISF signals
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why does USB 3.2 still emit hundreds of low confidence ISF outputs after the four false signals are removed
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > why does Wishbone declare no signals
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > why does a PDF-VARIANT-DIGESTION commit subject id not appear fully qualified in the task source
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > why does a SemanticIR residual packet list only some undeclared signal names
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > why does a Wishbone constraint name CYC instead of CYC_O
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > why does a document with no declared signals carry more ungrounded rules than one with signals
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
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > why does aggregate_composition support different health and ceiling counts
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
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why does an isf_rule_transaction_conflict_<name> residual appear in adapter.json
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
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > why does the AMBA DTI specification declare no signals
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > why does the Bosch CAN specification declare no signals
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
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > why does the OpenCAPI Data Link Layer specification emit no isf target
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
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > why does the chain-currency check ignore validation_reports
- [corpus-task-bounded-active-root-and-evidence-parts](../decisions/0024-corpus-task-bounded-active-root-and-evidence-parts.md)
  > why does the corpus task reuse the active task evidence checker
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
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > why does the fact-card landing have one line per card
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > why does the feedback protocol self test block derived state closure
- [no-collection-may-declare-an-aggregate-below-its-own-legal-maximum](../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
  > why does the live-document report show lines below the ceiling
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > why does the pre-commit hook run check_doctrines.sh
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why does the project README get spurious ahb_transfer/apb_transfer transactions
- [cortex-a76-optimization-guide-refresh-is-authority-empty](../knowledge/cortex-a76-optimization-guide-refresh-is-authority-empty.md)
  > why does the refreshed Cortex-A76 Software Optimization Guide emit no ISF
- [gic-overview-guide-refresh-is-authority-empty](../knowledge/gic-overview-guide-refresh-is-authority-empty.md)
  > why does the refreshed Generic Interrupt Controller Overview Guide emit no ISF
- [opencapi-32g-phy-timing-without-interface-topology](../knowledge/opencapi-32g-phy-timing-without-interface-topology.md)
  > why does the refreshed OpenCAPI 4.0 32 Gbps PHY Signaling specification emit no ISF
- [a-bounded-snapshot-needs-a-declared-repeatable-rollover](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
  > why does the roadmap archive hold more than one capsule
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
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > why is 198 exactly the largest fact-card maximum the current projection allows
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
