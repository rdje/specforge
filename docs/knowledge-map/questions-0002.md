# Knowledge questions — shard 0002

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does FSMGen --strict --check use or validate signal direction (input vs output)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > does FSMGen 030f8c273 accept a value-free (sample S as s) transaction body
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does FSMGen accept a symbolic (width PARAM) or only a concrete integer width
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > does FSMGen strict success prove that a SpecForge adapter is semantically faithful
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > does FSMGen support a register reset value in storage (yes — (storage (var NAME (width N) [(reset V)])) is shipped per 13k:42 + 13m:48-68; optional, in-width non-negative int, omission = all-0s byte-identical, over-width/non-integer fails closed)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does FSMGen's multi-actor ATL frontier provide a home for a connectivity netlist (NO — the ATL backlog wires children GENERATED from transaction composition spawn/do; it is behavioral orchestration, not a declarative static IP-interconnect netlist; verified 14-feature-backlog.md)
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > does FigureRegion have a production producer or only synthetic tests
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > does GateRecord remain schema compatible
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > does IntentIR preserve SWD protocol provenance and order
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > does PhaseRecord remain schema compatible
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > does SemanticIR keep copyright and license text
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > does SemanticIR preserve SWD protocol provenance and order
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does SpecForge capture component topology / connectivity for platform docs (YES — a typed signal_connectivity producer->consumer graph + infrastructure_signals clock/reset distribution; correcting the .2 'hint-level' to 'captured-but-sparse-and-unlowered')
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > does SpecForge classify Markdown inside the FSMGen submodule
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > does SpecForge cycle-schedule the FSM (no — FSMGen does)
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > does SpecForge delete non-applicable physical timing records
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > does SpecForge detect implementation-defined or TBD or and/or
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does SpecForge emit register bit-fields to ISF now (YES — DOC-INTENT-TAXONOMY.4a.ii: the storage var carries a (fields (field …)) block; 6,570 fields / 2,531 registers / 24 docs, was 0; 4 wire golds byte-identical; 0 new fsmgen --strict diagnostics)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > does SpecForge lower register reset values into the .isf (YES as of ISF-REGISTER-RESET-EMIT.2/.3 — composed from per-field reset_value and emitted at the true register width; it was dropped at the emit boundary before)
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > does SpecForge model-check temporal properties
- [cat3-topology-fsmgen-actor-network-reassessment](../knowledge/cat3-topology-fsmgen-actor-network-reassessment.md)
  > does SpecForge need to file a topology feature request now (not before .4c.ii measures the current contract and carrier fit)
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > does SpecForge preserve certification workflow source evidence
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > does SpecForge reject a page metadata staging traversal or symlink escape
- [contested-priors](../knowledge/contested-priors.md)
  > does SpecForge revise or decay priors
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > does SpecForge use LTL CTL or TLA+
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > does SpecForge use the full scope of a page's visual information
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > does Wishbone declare its signals in a table
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > does a PathBuf need portability handling before it has a producer
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > does a SIGKILL prove that Docling ran out of memory
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does a blocked adapter remove a previously emitted isf
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > does a deterministic semantic->intent rebuild recover lost actor_signal_relations
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > does a metadata rewrite failure preserve the previous normalized bundle
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > does a replaced constraint surface get polarity refinement (apply_persisted_polarity_to_constraints)
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does adapter output reconciliation delete unrelated files or directories
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > does adaptive batch sizing change the ingest output / break byte-identity
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > does agreement between sources boost confidence
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > does an actor signal relation alone authorize a SemanticIR interface signal
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > does an empty declared-signal catalog disable the SemanticIR grounding filter
- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > does an explicit timing row unit override a caption unit
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > does canonical SWD EvidenceIR contain interface edge timing
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > does cargo test need TMPDIR set manually
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > does changing a source path mean a corpus document was refreshed
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > does cleaning normalized bundles undo a completed corpus re-ingest
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > does constrained contract extraction ship code
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does converge apply the IntentIR NLI demotion gate
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does converge automatically run extract-contracts
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does converge automatically run recover-register-bits
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does converge automatically run signal-resolve
- [swd-protocol-convergence-snapshots-are-exact](../knowledge/swd-protocol-convergence-snapshots-are-exact.md)
  > does converge detect a protocol only SWD change
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > does converge re-ingest the PDF every run
- [swd-protocol-convergence-snapshots-are-exact](../knowledge/swd-protocol-convergence-snapshots-are-exact.md)
  > does convergence detect a same count protocol rewrite
- [swd-protocol-convergence-snapshots-are-exact](../knowledge/swd-protocol-convergence-snapshots-are-exact.md)
  > does convergence preserve protocol record order
- [corpus-kb-managed-currentness](../knowledge/corpus-kb-managed-currentness.md)
  > does corpus KB refresh mutate canonical IR or CorpusMemory
- [contested-priors](../knowledge/contested-priors.md)
  > does corpus prior memory only accrete
- [cat3-topology-fsmgen-actor-network-reassessment](../knowledge/cat3-topology-fsmgen-actor-network-reassessment.md)
  > does current FSMGen have any static actor instance or group construct (yes at a51dcdad0 — bounded actor instances/groups plus transaction-scoped actor/pin handoffs)
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > does emitting (input) signals break fsmgen --strict (no — 0 new diagnostics; drives are suppressed for inputs)
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > does encryption block the VLM from reading tables (no)
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > does enrich / audit-extraction / recover-register-bits read full-page images or region images
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > does eval-extraction rebuild evidence or load the persisted file
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > does every SWD protocol record receive an ISF adapter disposition
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > does every temporal_rule reach the .isf or a residual
- [protocol-evidence-is-generic-and-document-derived](../decisions/0035-protocol-evidence-is-generic-and-document-derived.md)
  > does generic protocol evidence still project losslessly to IntentIR
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > does header-sourced enum naming re-create the merge-by-name conflation (NO — 28 of 28 collision groups agree on every shared value, 0 conflicts. Structural, not lucky: a caption keyword like Table is shared by unrelated tables, but a header names the actual field and a field encodes the same way throughout a document. Worked example SMMU SH: 11 tables in ihi0070_e_a, every
  > shared value identical 0b00=NON_SHAREABLE/0b10=OUTER_SHAREABLE/0b11=INNER_SHAREABLE/0b01=RESERVED — the merge IS the correct encoding)
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > does lowering cat-3 topology need only an ISF construct or also a multi-actor emit (also a multi-actor emit — ISF is per-actor / one .isf = one FSMGen module and SpecForge's emit is single-initiator-actor; a declarative cross-component netlist is an architectural change, decided WITH FSMGen only after capture-recall clears the bar — not today)
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > does malformed page metadata preserve the last good normalized bundle
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > does markdown escaping cost recall on documents that already have a signal catalog
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > does mdbook test pass for the SpecForge book
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > does moving a PDF from the boot volume to SSD complete a current-binary corpus refresh
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does platform/system-IP (category 3) topology intent need a new ISF construct or map onto an existing one (ISF has NO declarative static-topology/connectivity construct — composition is transaction-level only; decision deferred to a capture-recall measurement .4c.i before any FR)
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > does raising max_facts alone create fact-card headroom
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > does register extraction require the table_kind register classification
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > does relation-derived direction synthesis independently invent signal names
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > does removing sentence fallback remove address phase recognition
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > does retiring gates remove conditional rules or temporal rules
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > does sanitizing the module name break initiator port matching (no — from_intent_ir re-derives the initiator raw; actor_name is only the label)
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > does select_initiator_actor choose the first or last equal maximum
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does specforge adapt remove an old actor isf when actor selection changes
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > does specforge handle coordinated drive/read objects (X drives A and B)
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > does specforge handle passive voice for actor-signal relations (X is driven by Y)
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > does specforge validate modify the artifact passed on the command line
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > does specforge write to the input path I pass or to a canonical generated path
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > does the Certified sibling confirm the OpenCAPI Ready glossary result
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > does the Docling metadata sidecar store repository relative paths
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > does the ISF adapter consume swd_operations or protocol_states
- [mdbook-current-truth-drift-lock](../knowledge/mdbook-current-truth-drift-lock.md)
  > does the ISF adapter lower actor-relative direction
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > does the ISF adapter lower generic gate behaviors
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > does the NLI verifier actually catch real extraction errors
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > does the USB4 Inter-Domain adapter still emit a USB4 signal
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > does the administrative classifier name OpenCAPI or a vendor
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > does the agent-identity gate keep Class-B fragments like Subordinate extends
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > does the authority empty repair preserve formal and system contract interfaces
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does the converge NLI pass measure quality or demote unsupported intent
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does the current FSMGen ISF support named bit-fields inside a storage var (NO — opaque (var NAME (width N)) only on pin 030f8c273; set-field/extract are runtime ops not a declaration)
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > does the current SWD chain come from the tracked ADI PDF
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > does the current SpecForge binary still fabricate AIA TOC timing constraints
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does the emitted .isf signal direction affect FSMGen downstream correctness
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > does the explicit-direction signal catalog repeat the UTF-8 boundary bug
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > does the external SSD project directory contain the USB4 Inter-Domain Service PDF
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > does the extractor cut identifiers at the underscore character
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > does the legal statement classifier use a vendor or document denylist
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > does the live-document registry reject unknown fields oversized arrays or oversized scalars
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > does the move portability repair change PathBuf JSON fields
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > does the path portability contract cover FigureRegion raw images
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > does the persisted path gate cover dormant schemas
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > does the reviewed I2S fixture claim a live VLM run
- [source-to-intent-reviewed-population](../knowledge/source-to-intent-reviewed-population.md)
  > does the reviewed population publish product support results
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > does the rolling ledger verifier validate predecessor successor chronology
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > does the stage-staleness detector false-fire on register/command docs with 0 relations (no — both empty)
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > does the trapped-row gap-fill mint duplicate signal declarations
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > does the upstream-first decision weaken the executable-intent objective
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > does the vertical evaluator make a category support claim yet
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > does transaction phase precision change phase membership or ISF output
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > does validate detect a stale downstream artifact that silently dropped relations
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > does validate follow the embedded artifact layout
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > does validate materialize SourceIR normalized manifests
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > does validate report message_field_records
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > fresh empirical reconfirmation of the .2i body-emission parking on the current 030f8c273 binary
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > has the corpus task evidence migration landed
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > have the PDF task migration destinations been created
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
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > how are external held-out PDFs identified without absolute host paths
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > how are external reviewed PDFs replayed without persisting host paths
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how are extraction-profile priors looked up (extraction_profile_priors_for signature-subset match)
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > how are generic SemanticIR phases different from transaction_phases
- [corpus-kb-managed-currentness](../knowledge/corpus-kb-managed-currentness.md)
  > how are human corpus KB notes preserved during refresh
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > how are incomplete gold and product failure distinguished
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > how are live-document ceiling increases and immutable debt baselines tested
- [section-header-message-field-extraction](../knowledge/section-header-message-field-extraction.md)
  > how are message fields written as section headings extracted
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how are message-field tables distinguished from register-field tables
