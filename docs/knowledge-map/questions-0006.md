# Knowledge questions — shard 0006

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > where is the spawn+poll+kill memory guard in materialize_pdf
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > where is the structural gate that drops function-word-led and verb-led actor candidates
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > where is the tiling-gated register bit recovery implemented
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > where is the trailing function-word strip in consolidate_trailing_fragment
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > where is the weak-phrase / NASA ARM ambiguity detector
- [repository-local-scratch](../knowledge/repository-local-scratch.md)
  > where may diagnostic scratch files and comparison lists be written
- [live-document-containment-and-data-locality](../decisions/0007-live-document-containment-and-data-locality.md)
  > where must SpecForge project artifacts caches and temporary workspaces live
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > which 15 docs are category-3 platform/system-IP in the corpus (GIC-600/400 TRMs, CoreSight SoC-600 x3 / SDC-600 / TMC TRMs, MMU-700 TRM, Cortex-A76 TRM, CoreSight Base System Arch, CoreSight/GIC/SMMU/ARM-Debug-v6/Advanced-Comms-Channel architecture specs; the .1 census never persisted the per-doc labels — .4c.i enumerates them in scripts/measure_cat3_topology_recall.py)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > which AXI signals belong to which channel (B1.1 write request / B1.2 write data / B1.3 write response / B1.4 read request / B1.5 read data / B1.6 B1.7 snoop)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > which AXI transactions gain channel grouping (atomic/prefetch/writezero/writedeferrable/narrow_transfer)
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > which IntentIR surfaces are lowered to the .isf vs silently dropped
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > which LLM or VLM does SpecForge use
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > which Markdown files must the live-document containment registry cover
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
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > which protocols look like register IPs structurally (CCIX, AXI, CHI, DTI, CHI-C2C — the register-heavy-protocol trap)
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > which register-table family stays residual (byte location size tables)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > which signal grounds the ordered multi-phase transaction body (the document's own <qualifier> phase structure)
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > which stale roadmap statuses were found before migration
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > which strategy is message_fields.byte_location_table
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > which validate metrics/finding surface channel membership
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
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > why did an EXTRACTOR-ARCHITECTURE byte-identical proof fail on SWD/ADI but pass on other docs
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
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > why does DTI (ihi0088) have zero message_field_records
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > why does Docling re-ingest fail on Apple Silicon
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why does RISC-V AIA capture 0 registers (its IMSIC/APLIC CSR intent is in prose conditional_rules/behaviors; no current register strategy matches RISC-V's CSR layout — the .10g <NAME>, bits [hi:lo] section-heading family fires only on ARM ihiXXXX arch specs)
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > why does SWP yield 0 from extract_protocol_states and extract_quoted_mode_states but 4 from the transition-bound path
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
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > why does one register appear as several RegisterRecords / how are split register-field tables merged
