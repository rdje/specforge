# Knowledge questions — shard 0015

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > who reads or writes the active PDF task tree
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > why a parallel field-constraint surface instead of a subject-kind discriminator
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why an allowlist instead of a denylist for .isf identifier sanitization (a denylist can't enumerate every bad char — it missed the arrow →)
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > why are 15 of Avalon's 26 declarations width-only
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > why are 169 composable registers not yet emittable (their composed reset needs more bits than the current storage-var width which is max-field-extent not register width — e.g. CoreSight DPIDR V=0x1c013477 at width 11 over-width; var-width reconciliation spun to ISF-REGISTER-RESET-EMIT.3)
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > why are 58 corpus documents not currency-measurable
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > why are CoreSight SDC-600 TX_VALID and EXT_TX_VALID both real declarations
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > why are GIC/SMMU/CoreSight section-heading fields NOT message fields
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > why are SWD recognized phases signal_set empty and transactions ports empty
- [root-reference-mdbook-authority](../knowledge/root-reference-mdbook-authority.md)
  > why are USER_GUIDE.md and the root architecture documents only pointers
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why are behaviors and constraints (22k each) not lowered to .isf — is that a gap
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > why are claim evidence commands argv arrays
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > why are conditional_rules excluded from the document-class decision
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > why are determiners (All Managers) NOT rejected by the .1a agent gate
- [decision-capacity-is-rederived-without-moving-stable-records](../decisions/0041-decision-capacity-is-rederived-without-moving-stable-records.md)
  > why are existing decision records not moved into partitions
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
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > why are reviewed fixture anchors fragile
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
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > why are table regions no longer excluded from captured-region residuals
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why are table-row sources exempt from the pre-bind constraint subject rule
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why are temporal_rules not usable to order a transaction body (they are per-signal stability/value constraints not phase edges)
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > why are the AHB gold negatives for statements 0561 and 0678 there
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > why are the APB AHB AXI wire golds not in the corpus refresh frontier
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
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > why can I not record a reclamation in retained_bundles.json
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > why can SWD protocol records not be lowered directly to ISF
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > why can SWD score 100 percent while its protocol is absent downstream
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > why can a base-name template rule not live in the SourceIR table classifier
- [required-residual-actionability-denominator](../knowledge/required-residual-actionability-denominator.md)
  > why can a canonical cell never satisfy its residual actionability observations
- [sourceir-classification-is-per-record](../knowledge/sourceir-classification-is-per-record.md)
  > why can a per-table classifier not inherit a kind from a parent table
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > why can a shipped standalone extractor fail to improve the default end-to-end result
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > why can an active task legacy route use a shorthand source literal
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > why can an extra blank line fail harmless layout comparison
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why can an uppercase token inside a longer word like OpenCAPI become a signal constraint subject
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > why can eval-extraction score only SWD/ADI and I2C
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > why can the 80-record status window never be reached
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > why can the APB, AHB and AXI SourceIR proofs not be migrated from a retained bundle
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
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > why did AXI have 134 actors when APB has 8 and AHB has 25
- [signal-presence-matrix-capture](../knowledge/signal-presence-matrix-capture.md)
  > why did AXI unexplained tables go 32 to 31 and ACE 36 to 35 and LTI 6 to 4
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did AXI unexplained tables go 39 to 32 and the denominator 94 to 98
- [reviewed-population-clean-replay-carrier-regression](../knowledge/reviewed-population-clean-replay-carrier-regression.md)
  > why did Arm Debug lose twelve reviewed register facts
- [register-record-access-and-table-provenance](../knowledge/register-record-access-and-table-provenance.md)
  > why did Arm Debug register access disappear before EvidenceIR
- [alpha-variant-placeholder-is-not-a-wire](../knowledge/alpha-variant-placeholder-is-not-a-wire.md)
  > why did AxLEN become an AXI interface port
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > why did Cortex-A76 instruction performance tables produce 151 timing constraints with no min typ max values
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > why did DL disappear from the OpenCAPI Certified Definition
- [persisted-census-measures-published-not-current](../knowledge/persisted-census-measures-published-not-current.md)
  > why did EXTRACTION-QUALITY-GAUGE.3k.1 have zero currently-reproducible instances
- [logic-level-walk-stops-at-eleven-unrelated-words](../knowledge/logic-level-walk-stops-at-eleven-unrelated-words.md)
  > why did EXTRACTION-QUALITY-GAUGE.3k.12 ship no rule
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why did FSMGen pin a51dcdad0 reject SpecForge manager.isf with isf_ambiguous_rule_transaction_drive_priority on AWSNOOP
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why did GIC-600's .isf fail fsmgen strict with 'Malformed top-level FSM source ?fsm:redistributor→…'
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > why did HBM2's .isf fail fsmgen strict with enum member 'TABLE.REPAIR_LANE_8' value token '1000'
- [one-record-per-obligation-clause](../knowledge/one-record-per-obligation-clause.md)
  > why did HSELx must_be_asserted carry the wrong condition
- [identifiers-are-opaque-and-one-way-grounded](../decisions/0037-identifiers-are-opaque-and-one-way-grounded.md)
  > why did ISF renderability fall from 44 to 17
- [retained-chain-rebuild-order](../knowledge/retained-chain-rebuild-order.md)
  > why did IntentIR proof verification fail after I validated SemanticIR
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > why did LIVE-DOCUMENT-PRESSURE-HEADROOM.4d reflow one bullet instead of rewrapping README.md
- [the-binding-bearing-clause](../knowledge/the-binding-bearing-clause.md)
  > why did LRPROT must be 0 carry the condition When LRRESP is FaultAbort
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > why did LTI unexplained tables go UP from 5 to 6
- [a-level-belongs-to-a-signal](../knowledge/a-level-belongs-to-a-signal.md)
  > why did NVM must_be_low come from low level format
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > why did OpenCAPI data link semantic phases gates and contracts drop to zero
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > why did OpenCAPI email review become an IntentIR behavior
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why did OpenCAPI produce constraints on CAPI OCDE and DLX that the document never constrains
- [a-level-belongs-to-a-signal](../knowledge/a-level-belongs-to-a-signal.md)
  > why did PREQ publish must_be_high when the row sets PREQ LOW
- [self-test-coverage-guard-is-in-the-exit-path](../knowledge/self-test-coverage-guard-is-in-the-exit-path.md)
  > why did PRODUCTION-GRAPH-CENSUS-PIN.2's census have to be corrected twice
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why did RESERVED-only NOT ship as a header-naming exclusion (build_symbol_definitions keys members by NAME per document and drops any member whose value conflicts, so HALF the RESERVED-only cases are eliminated downstream with no name-side gate — 6 of 12 in the .5.iv census frame, 4 of 10 in the shipped-predicate frame, the SAME six survivors either way: SMMU
  > STALL_MODEL/TTENDIAN/HTTU/PGS and CHI-C2C CONTFORMAT/EVENTTYPE. Those survivors are structurally indistinguishable from 31 legitimate single-distinct-member tables (TTL 0b00=NO_LEVEL_HINT_INFORMATION, CD2L, S1P, PRI, GRAN4K...), leaving only the word RESERVED itself as a discriminator — exactly the spec-assigned value vocabulary ADR 0006 forbids. Re-derive:
  > scripts/measure_header_sourced_enum_naming.py --reserved-split)
- [protocol-state-machine-binding](../knowledge/protocol-state-machine-binding.md)
  > why did SWD protocol_state score 0/13 when the state names were right
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why did SpecForge remove every generated (priority RULE over TRANSACTION) line
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > why did TL disappear from the OpenCAPI Ready Definition
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why did USB 3.2 produce 918 interfaces and 556 adapter signals
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > why did VALID PENDING CRDT CRDTSH SHAREDCRD RP become AXI interface ports
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > why did a Name | Signals covered | Width | Check enable table classify as unknown
- [legacy-source-classifications-are-neutralized-on-load](../knowledge/legacy-source-classifications-are-neutralized-on-load.md)
  > why did a Python census over persisted table_kind over-count the row extractor's population
- [sourceir-classification-is-per-record](../knowledge/sourceir-classification-is-per-record.md)
  > why did a SourceIR classification change fail with 'classification is not the registered capture/proposal replay'
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > why did a current claim census count change without the producer changing
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > why did a figure caption become null after re-ingest
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > why did a fresh ingest expose normalized staging in persisted metadata
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > why did a junk polarity record appear after WIRE-BASED-100.10b
- [one-modal-vocabulary-per-constraint-record](../knowledge/one-modal-vocabulary-per-constraint-record.md)
  > why did a must not be changed obligation type while cannot be changed did not
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > why did a paragraph gain words after re-ingest
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > why did a signal declared in EvidenceIR never reach the semantic interface records
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > why did a signal named `any` appear in an EvidenceIR artifact
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > why did a signal named `name` appear in the Avalon interface specification
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > why did a signal table declare English words like Secure, Stream, Asserted or The (its name column scored zero because the column scorer did not strip a cell's punctuation the way the row loop does, so a paired name cell like AWMMUSECSID, ARMMUSECSID lost, and the Description column won the override)
- [table-row-obligation-binds-to-the-token-before-its-modal](../knowledge/table-row-obligation-binds-to-the-token-before-its-modal.md)
  > why did a temporal conflict disappear when a signal constraint was removed
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
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > why did check_doctrines.sh --all not finish in 50 minutes
- [doctrine-driver-runs-no-cargo-gate](../knowledge/doctrine-driver-runs-no-cargo-gate.md)
  > why did check_doctrines.sh pass while the build did not lint
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > why did check_proof_seal_currency.sh pass while four artifacts would not load
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
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > why did my declaration census join at 92.8 percent
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > why did my rolling-ledger rollover transaction fail with staged output identity drift for manifest.jsonl (a non-ASCII byte in the plan's reason: the manifest writer emits without a UTF-8 layer, so one em dash breaks the staged manifest's identity check and the whole transaction rolls back to exact preflight bytes. A plan reason must be pure ASCII)
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why did normalize_prior_phrase grow a string exponentially
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > why did only four documents refuse to load after an EvidenceIR producer change
- [source-proof-migration-replays-classification-context](../knowledge/source-proof-migration-replays-classification-context.md)
  > why did proof-only SourceIR refresh fail after a table classifier change
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
- [one-modal-vocabulary-per-constraint-record](../knowledge/one-modal-vocabulary-per-constraint-record.md)
  > why did teaching the classifier a new modal publish a NOTE constraint
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > why did the .2a direction deferral get reopened (explicit owner steer 2026-06-17 — Build it, initiator perspective)
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > why did the 345-row phrase approximation not re-derive
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > why did the ACE evidence build run out of memory / get SIGKILLed (exit 137)
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > why did the AIA table of contents have 19 timing false positives
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > why did the AMBA AXI+ACE ihi0022_h_c manager.isf fail fsmgen strict with 'rule constraint_48 assignment actions require (port expr)'
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why did the AMBA LPI controller.isf fail fsmgen strict with isf_conflicting_rule_writes on PREQ/PACCEPT
- [reviewed-population-clean-replay-carrier-regression](../knowledge/reviewed-population-clean-replay-carrier-regression.md)
  > why did the AMD IOMMU packed layout false register disappear
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > why did the APB AHB AXI chains go legacy when CORPUS-PATTERN-REUSE.3c already re-ingested them
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why did the AXI/AHB/AXI-Stream wire-gold .isf fail fsmgen --strict on a fresh re-emit (isf_conflicting_rule_writes), and what fixed them
