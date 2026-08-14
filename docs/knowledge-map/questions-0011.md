# Knowledge questions — shard 0011

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > which tree owns the empty-catalog grounding filter defect
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > which units mark a timing record as decibel domain
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > which validate metrics/finding surface channel membership
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > who reads and writes docs tasks CORPUS-COVERAGE md
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > who reads and writes the SPEC-TO-INTENT-ALIGNMENT task tree
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > who reads or writes the active PDF task tree
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > why a parallel field-constraint surface instead of a subject-kind discriminator
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why an allowlist instead of a denylist for .isf identifier sanitization (a denylist can't enumerate every bad char — it missed the arrow →)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > why are 169 composable registers not yet emittable (their composed reset needs more bits than the current storage-var width which is max-field-extent not register width — e.g. CoreSight DPIDR V=0x1c013477 at width 11 over-width; var-width reconciliation spun to ISF-REGISTER-RESET-EMIT.3)
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > why are 58 corpus documents not currency-measurable
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
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > why are extracted signal names truncated at the underscore
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > why are held-out alpha failures not yet production name-coupling evidence
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
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > why are some retained CCIX decibel timing records still canonical
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > why are some section-heading registers held as a residual instead of emitted
- [stable-obligation-phase-scoped-residual](../knowledge/stable-obligation-phase-scoped-residual.md)
  > why are stability obligations residuals
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why are table-row sources exempt from the pre-bind constraint subject rule
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
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > why are there 82 retained phase records but 101 on a current rule replay
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > why are two containment records at the bottom of CHANGES
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > why can CORPUS-COVERAGE not accept refresh 49 yet
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > why can SWD protocol records not be lowered directly to ISF
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > why can SWD score 100 percent while its protocol is absent downstream
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > why can a shipped standalone extractor fail to improve the default end-to-end result
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > why can an active task legacy route use a shorthand source literal
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > why can an extra blank line fail harmless layout comparison
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why can an uppercase token inside a longer word like OpenCAPI become a signal constraint subject
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > why can the fact-card maximum not simply be raised from 198
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
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > why did 64 AArch64 External Debug interfaces disappear
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > why did 88 CoreSight Base System interfaces disappear
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > why did AFU BAR CFG GB ID MEM MMIO and PASID disappear as signals
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did APB unexplained_intent_bearing_tables go to zero
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > why did AXI atomic_transaction / prefetch / writezero / writedeferrable go from 0 to a real signal set
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > why did AXI unexplained tables go 32 to 31 and ACE 36 to 35 and LTI 6 to 4
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did AXI unexplained tables go 39 to 32 and the denominator 94 to 98
- [register-record-access-and-table-provenance](../knowledge/register-record-access-and-table-provenance.md)
  > why did Arm Debug register access disappear before EvidenceIR
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > why did Cortex-A76 instruction performance tables produce 151 timing constraints with no min typ max values
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > why did DL disappear from the OpenCAPI Certified Definition
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why did FSMGen pin a51dcdad0 reject SpecForge manager.isf with isf_ambiguous_rule_transaction_drive_priority on AWSNOOP
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why did GIC-600's .isf fail fsmgen strict with 'Malformed top-level FSM source ?fsm:redistributor→…'
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > why did HBM2's .isf fail fsmgen strict with enum member 'TABLE.REPAIR_LANE_8' value token '1000'
- [identifiers-are-opaque-and-one-way-grounded](../decisions/0037-identifiers-are-opaque-and-one-way-grounded.md)
  > why did ISF renderability fall from 44 to 17
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did LTI unexplained tables go UP from 5 to 6
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > why did OpenCAPI data link semantic phases gates and contracts drop to zero
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > why did OpenCAPI email review become an IntentIR behavior
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why did OpenCAPI produce constraints on CAPI OCDE and DLX that the document never constrains
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why did SpecForge remove every generated (priority RULE over TRANSACTION) line
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > why did TL disappear from the OpenCAPI Ready Definition
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why did USB 3.2 produce 918 interfaces and 556 adapter signals
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > why did a fresh ingest expose normalized staging in persisted metadata
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > why did an EXTRACTOR-ARCHITECTURE byte-identical proof fail on SWD/ADI but pass on other docs
- [timing-scalar-rows-require-independent-cell-geometry](../knowledge/timing-scalar-rows-require-independent-cell-geometry.md)
  > why did an OpenCAPI Notes footer become a timing constraint with parameter min typ max and unit
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > why did an OpenCAPI functional test become an IntentIR behavior
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > why did an OpenCAPI permissions paragraph become an IntentIR behavior
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > why did channel.isf and setportfeature_port_over_current.isf coexist after rebuilding USB 3.2
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > why did eight Introducing CoreSight ports and four connectivity edges disappear
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > why did generated artifact scanning miss FigureRegion raw_image_path
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > why did ingest stop with 'ingest aborted before launching'
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > why did ingest stop with 'ingest aborted to protect the host'
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > why did max_facts become 379
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why did normalize_prior_phrase grow a string exponentially
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > why did removing false signals ADD conditional rules to SemanticIR
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > why did rendered_image.path contain normalized.staging
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > why did retiring generic phases remove pure inferred actors
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > why did running nli-verify on a .prepromote.bak overwrite the real evidence_ir.json
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > why did seed_axi_temporal fail after constraint promotion and how was it fixed
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > why did six Introducing CoreSight interfaces disappear
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > why did the .2a direction deferral get reopened (explicit owner steer 2026-06-17 — Build it, initiator perspective)
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why did the ACE evidence build run out of memory / get SIGKILLed (exit 137)
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > why did the AIA table of contents have 19 timing false positives
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
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > why did the OpenCAPI Certified DL signal disappear
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > why did the OpenCAPI Ready DL signal disappear
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > why did the canonical SWD relation count change from 25 to 21
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > why did the corpus KB size warning appear and how was it removed
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > why did the corpus task-evidence index verify the PDF contract
- [corpus-wide-interface-authority-rebuild](../knowledge/corpus-wide-interface-authority-rebuild.md)
  > why did the emitted ISF count drop from 57 to 44
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > why did the host actor disappear from AArch64 External Debug
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > why did the initial prospective behavioral held-out run fail
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > why did the moved Python virtual environments still access the old repository
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > why did the promoted surface lose the AXI reset temporal rules (DEASSERTED vs LOW)
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > why did the register-at-offset placement-map hypothesis get overturned
- [no-collection-may-declare-an-aggregate-below-its-own-legal-maximum](../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
  > why did the task_evidence aggregate ceiling become 480000
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > why did transaction_phases contain called edge or positive
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > why did validating a copied rollback backannotate the canonical chain
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
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why do I2C SDA USDA and SDAH remain signals
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > why do I2C/CCIX/USB4 have 0 table signals and how are they recovered
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > why do NVMe/CCIX/DTI register-field obligations not lower to the .isf
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > why do conditional_rules lower to .isf only partially
- [repo-local-temp-docling-test-collision](../knowledge/repo-local-temp-docling-test-collision.md)
  > why do environment-lock tests cascade with PoisonError
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
