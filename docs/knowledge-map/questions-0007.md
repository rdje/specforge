# Knowledge questions — shard 0007

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > which SourceIR and EvidenceIR paths serialize repository relative
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > which active task tree is next at the live document warning
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > which boot-volume Rust directories are allowed
- [canonical-collection-catalogs](../knowledge/canonical-collection-catalogs.md)
  > which canonical Markdown collections still rely only on git query
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > which corpus PDFs are password/permission protected
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > which corpus docs declare fields with a Field-titled column
- [corpus-coverage-sweep](../knowledge/corpus-coverage-sweep.md)
  > which corpus docs still yield nothing (the VLM frontier)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > which corpus documents are category 4 CPU-ISA (exactly 2: 1_0_risc_v_debug_specification and 1_0_2025_03_12_risc_v_advanced_interrupt_architecture; the RISC-V IOMMU doc is category 2)
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
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > which doctrines are registered (MEMORY-ARCH, KNOWLEDGE-MAP, TASK-ACCEPTANCE)
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > which document categories are mature vs partial vs thin for ISF synthesis (cat1 mature, cat2/3 partial, cat4 thin, cat5/6 non-target)
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > which exact current state copies are not yet independently verified in SpecForge
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > which formal signal declaration predicates does the dense prose authority gate accept
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > which guides over-extract spurious .isf wire intent (cortex-a76 sw-opt 537 signals, readme, smmu software guide, gic overview, aarch64 debug guide)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > which header signatures are unrecovered register tables
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > which local model should SpecForge use for NLI or entailment verification
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > which local models are pulled and what are they for
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > which persisted evidence docs carry message_field_records
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > which persisted paths must exist and which may be historical references
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > which project rescan string fields are treated as filesystem paths
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > which protocols look like register IPs structurally (CCIX, AXI, CHI, DTI, CHI-C2C — the register-heavy-protocol trap)
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > which register-table family stays residual (byte location size tables)
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > which root rolling ledger is currently above its rollover signal
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > which signal grounds the ordered multi-phase transaction body (the document's own <qualifier> phase structure)
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > which stale roadmap statuses were found before migration
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > which strategy is message_fields.byte_location_table
- [repo-local-temp-docling-test-collision](../knowledge/repo-local-temp-docling-test-collision.md)
  > which task owns deterministic Rust tests under repository-local TMPDIR
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > which task owns fact-card catalog containment
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > which task owns mdBook fence classification and doctest enforcement
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > which task tree crossed its live-document byte warning
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > which validate metrics/finding surface channel membership
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > who reads or writes the active PDF task tree
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > why a parallel field-constraint surface instead of a subject-kind discriminator
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why an allowlist instead of a denylist for .isf identifier sanitization (a denylist can't enumerate every bad char — it missed the arrow →)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > why are 169 composable registers not yet emittable (their composed reset needs more bits than the current storage-var width which is max-field-extent not register width — e.g. CoreSight DPIDR V=0x1c013477 at width 11 over-width; var-width reconciliation spun to ISF-REGISTER-RESET-EMIT.3)
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > why are GIC/SMMU/CoreSight section-heading fields NOT message fields
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > why are SWD recognized phases signal_set empty and transactions ports empty
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > why are USER_GUIDE.md and the root architecture documents only pointers
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why are behaviors and constraints (22k each) not lowered to .isf — is that a gap
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > why are conditional_rules excluded from the document-class decision
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > why are determiners (All Managers) NOT rejected by the .1a agent gate
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why are many distinct value-tables merged into one enum (build_symbol_definitions accumulates members by enum_name key, semantic.rs:2782-2789 — every 'TABLE'-named table fuses into one SymbolDefinitionRecord)
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > why are message fields not part of the document_class decision
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > why are modal verbs must shall should may not flagged as ambiguous
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > why are prior candidate fixture names not joined into one long Markdown line
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why are register-IP (cat 2) and platform-IP (cat 3) reported as one combined register-or-platform category (counts cannot separate them — DOC-INTENT-TAXONOMY.1)
- [live-document-containment-and-data-locality](../decisions/0007-live-document-containment-and-data-locality.md)
  > why are rustup and cargo allowed on the boot volume
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > why are sbaddress3 / custom0 / a garbled sizelo register NOT merged
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why are section-caption / value-restart enum residuals NO-GO (.5.iii: section-caption/table-ref has no FP-free gate — leading [A-Z]?digit token collides with real codes D1/D2/L2 e.g. DEBUG:D1_1; restart-of-clean has no fidelity defect — .5.ii proved restart is not junk, all members real, mostly .5.i-dropped; glossary SEE…/front-matter are tiny + name-ish -> honest
  > residuals)
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > why are some section-heading registers held as a residual instead of emitted
- [stable-obligation-phase-scoped-residual](../knowledge/stable-obligation-phase-scoped-residual.md)
  > why are stability obligations residuals
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why are temporal_rules not usable to order a transaction body (they are per-signal stability/value constraints not phase edges)
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > why are the AHB gold negatives for statements 0561 and 0678 there
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why are the IntentIR transactions[] entries not real transactions (handshakes/behavior blobs)
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > why are the corpus-cluster extraction profiles mostly empty / 'none recorded yet'
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > why are the fired: behavioral features mostly empty in the clustering today
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > why are two containment records at the bottom of CHANGES
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > why can SWD protocol records not be lowered directly to ISF
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > why can SWD score 100 percent while its protocol is absent downstream
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > why can an active task legacy route use a shorthand source literal
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > why can the roadmap not be split safely at Markdown headings
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > why can't I just run converge to land the LLM-primary promotion on a canonical artifact
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > why can't RISC-V AIA registers be captured by .4d.i (its normalized bundle is ABSENT — re-ingest RAM/Docling-gated under CORPUS-COVERAGE — and its CSR intent is prose conditional_rules, not register tables)
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > why can't cluster-scoping replace the forbidden supply-rail (VDD/VSS) denylist
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > why can't extract-constraints-llm run inside a converge pass
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > why can't specforge evidence rebuild the evidence (normalized missing)
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > why can't specforge just drop single-noun or multiword actors to fix the prose phantom explosion
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > why did .1b.i NOT strip trailing prepositions and what changed in .1c.i
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did APB unexplained_intent_bearing_tables go to zero
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > why did AXI atomic_transaction / prefetch / writezero / writedeferrable go from 0 to a real signal set
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > why did AXI unexplained tables go 32 to 31 and ACE 36 to 35 and LTI 6 to 4
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did AXI unexplained tables go 39 to 32 and the denominator 94 to 98
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why did GIC-600's .isf fail fsmgen strict with 'Malformed top-level FSM source ?fsm:redistributor→…'
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > why did HBM2's .isf fail fsmgen strict with enum member 'TABLE.REPAIR_LANE_8' value token '1000'
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did LTI unexplained tables go UP from 5 to 6
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why did USB 3.2 produce 918 interfaces and 556 adapter signals
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > why did a fresh ingest expose normalized staging in persisted metadata
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > why did an EXTRACTOR-ARCHITECTURE byte-identical proof fail on SWD/ADI but pass on other docs
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > why did channel.isf and setportfeature_port_over_current.isf coexist after rebuilding USB 3.2
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > why did generated artifact scanning miss FigureRegion raw_image_path
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > why did ingest stop with 'ingest aborted before launching'
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > why did ingest stop with 'ingest aborted to protect the host'
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why did normalize_prior_phrase grow a string exponentially
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > why did running nli-verify on a .prepromote.bak overwrite the real evidence_ir.json
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > why did seed_axi_temporal fail after constraint promotion and how was it fixed
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > why did the .2a direction deferral get reopened (explicit owner steer 2026-06-17 — Build it, initiator perspective)
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why did the ACE evidence build run out of memory / get SIGKILLed (exit 137)
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > why did the AMBA AXI+ACE ihi0022_h_c manager.isf fail fsmgen strict with 'rule constraint_48 assignment actions require (port expr)'
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why did the AMBA LPI controller.isf fail fsmgen strict with isf_conflicting_rule_writes on PREQ/PACCEPT
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why did the AXI/AHB/AXI-Stream wire-gold .isf fail fsmgen --strict on a fresh re-emit (isf_conflicting_rule_writes), and what fixed them
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > why did the Knowledge Map shard simulator and generator report different canonical input hashes
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > why did the LLM-primary extractor lose SYSCOREQ from a coordinated-subject sentence
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > why did the LLM-primary extractor miss PBUSER / PNSE / HAUSER must_be_value VALID
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > why did the canonical SWD relation count change from 25 to 21
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > why did the corpus KB size warning appear and how was it removed
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > why did the moved Python virtual environments still access the old repository
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > why did the promoted surface lose the AXI reset temporal rules (DEASSERTED vs LOW)
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > why did the register-at-offset placement-map hypothesis get overturned
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > why do .1a and .1b.iv not catch the eMMC phantom actors like advantage of / basic bus / actual sector
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why do 8 generic-named enums survive .5.i (they are document-evidenced — the token IS a declared signal or a column header in that doc, e.g. CCIX 'Table of Contents' header cells keep a 'TABLE' enum; the structural gate correctly cannot drop them without a forbidden name-list — honest .5.ii member-quality residuals)
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > why do AMD DTE / NVMe command dword tables go to message fields and not registers
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > why do APB tables 0016 0017 0018 produce no signal records
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > why do CCIX-class docs extract hundreds of register fields now
- [repo-local-temp-docling-test-collision](../knowledge/repo-local-temp-docling-test-collision.md)
  > why do Docling source tests fail when TMPDIR is inside the repository
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > why do FPs appear in eval that the current code does not produce
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > why do GIC/SMMU/CoreSight architecture-spec registers gain fields from section headings
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > why do I2C/CCIX/USB4 have 0 table signals and how are they recovered
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > why do NVMe/CCIX/DTI register-field obligations not lower to the .isf
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > why do conditional_rules lower to .isf only partially
- [repo-local-temp-docling-test-collision](../knowledge/repo-local-temp-docling-test-collision.md)
  > why do environment-lock tests cascade with PoisonError
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > why do generated IR files contain the old SpecForge repository path
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > why do i / its stay out of the non-actor function-word list (GIC ITS, the letter I)
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > why do many conditional_rules not lower to an ISF (rule) (they name no signal obligation, an undeclared signal, a placeholder action, or only a bare modal shall/must with no concrete value/level — lowering would fabricate the obligation)
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
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > why does a rebuild drop the extraction-quality gauge
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why does a register count NOT veto a wire protocol (wire-vs-structure weight dominance; AXI wire 401 >= struct 229)
