# Knowledge questions — shard 0004

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > does header-sourced enum naming re-create the merge-by-name conflation (NO — 28 of 28 collision groups agree on every shared value, 0 conflicts. Structural, not lucky: a caption keyword like Table is shared by unrelated tables, but a header names the actual field and a field encodes the same way throughout a document. Worked example SMMU SH: 11 tables in ihi0070_e_a, every
  > shared value identical 0b00=NON_SHAREABLE/0b10=OUTER_SHAREABLE/0b11=INNER_SHAREABLE/0b01=RESERVED — the merge IS the correct encoding)
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > does known_signals fold a signal name's case
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
- [a-relational-predicate-is-not-a-value](../knowledge/a-relational-predicate-is-not-a-value.md)
  > does must_be_value GREATER exist in the corpus
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > does name_cell_is_read_whole cost any recall
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > does parse_explicit_signal_declaration keep the direction when it cannot read the width
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does platform/system-IP (category 3) topology intent need a new ISF construct or map onto an existing one (ISF has NO declarative static-topology/connectivity construct — composition is transaction-level only; decision deferred to a capture-recall measurement .4c.i before any FR)
- [measured-stratum-promotion-population](../knowledge/measured-stratum-promotion-population.md)
  > does promoting a measured-stratum document destroy a persisted extraction quality gauge (no — zero of the 27 measured documents carry one; the only seven artifacts in generated/ that carry a gauge are exactly the seven historical documents that are already promoted, because nli-verify was only ever run where the promotion had been)
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > does raising max_facts alone create fact-card headroom
- [legacy-reclassification-is-not-the-binding-constraint](../knowledge/legacy-reclassification-is-not-the-binding-constraint.md)
  > does re-deriving a table label grant canonical authority
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > does re-ingesting lose three paragraphs
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > does reading the arrow form fix the four documents that lose every row
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > does reading the arrow form recover Avalon's eight signals
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > does refreshed in the corpus frontier census mean the document is at the current schema
- [caption-repair-corpus-selection](../knowledge/caption-repair-corpus-selection.md)
  > does refusing a caption title lose a real requirement
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > does refusing a phrase name cell recover the wire the row was hiding
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > does refusing a prose width lose a signal
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > does register extraction require the table_kind register classification
- [a-remote-decision-provider-is-measured-against-the-best-local-arm](../decisions/0051-a-remote-decision-provider-is-measured-against-the-best-local-arm.md)
  > does rejecting a provider mean models are unwelcome in SpecForge
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > does relation-derived direction synthesis independently invent signal names
- [escaped-identifier-fragment-adjudication](../knowledge/escaped-identifier-fragment-adjudication.md)
  > does removing an English-word catalog name like POWER or USER risk withdrawing a correct record (no, and the evidence runs the other way: eMMC writes the word power 361 times in lowercase prose and the identifier POWER never, so declaring POWER a signal is the defect; the published record population for all 18 is 0, so nothing is withdrawn today)
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > does removing sentence fallback remove address phase recognition
- [identifiers-are-opaque-and-one-way-grounded](../decisions/0037-identifiers-are-opaque-and-one-way-grounded.md)
  > does renaming a signal change semantic extraction
- [the-binding-bearing-clause](../knowledge/the-binding-bearing-clause.md)
  > does replay-constraints show what moved when a record is not reproduced
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > does retiring gates remove conditional rules or temporal rules
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > does sanitizing the module name break initiator port matching (no — from_intent_ir re-derives the initiator raw; actor_name is only the label)
- [doctrine-driver-runs-no-cargo-gate](../knowledge/doctrine-driver-runs-no-cargo-gate.md)
  > does scripts/check_doctrines.sh run cargo clippy
- [doctrine-driver-runs-no-cargo-gate](../knowledge/doctrine-driver-runs-no-cargo-gate.md)
  > does scripts/check_doctrines.sh run cargo test or cargo fmt
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > does select_initiator_actor choose the first or last equal maximum
- [source-proof-migration-replays-classification-context](../knowledge/source-proof-migration-replays-classification-context.md)
  > does source proof migration rerun Docling
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > does source_ref identify one Docling item
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does specforge adapt remove an old actor isf when actor selection changes
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > does specforge handle coordinated drive/read objects (X drives A and B)
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > does specforge handle passive voice for actor-signal relations (X is driven by Y)
- [retained-chain-rebuild-order](../knowledge/retained-chain-rebuild-order.md)
  > does specforge validate change the artifact it validates
- [validate-explicit-artifact-path-contained-backannotation](../knowledge/validate-explicit-artifact-path-contained-backannotation.md)
  > does specforge validate modify the artifact passed on the command line
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > does specforge write to the input path I pass or to a canonical generated path
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > does suffix spelling authorize a PSEL to PSELX alias
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > does the .5.iv.a header lever change any persisted artifact (NO — it is inert on the whole measurable stratum, proven not assumed: check_chain_currency.sh replays evidence/semantic/intent/isf-adapter for all 24 rebuildable documents against the patched binary and every persisted artifact is byte-identical. Exactly 1 of the 285 accepted tables sits in a rebuildable document
  > — the SMMU guide Table 3-1 that opened .5.iv — and it mints nothing because .5.ii drops its sentence members. The other 8 documents are legacy schema-1 chains the current binary refuses for canonical use until re-ingest)
- [a-dropped-declaration-row-is-usually-not-a-signal](../knowledge/a-dropped-declaration-row-is-usually-not-a-signal.md)
  > does the Ax metavariable name a wire
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
- [an-llm-constraint-record-cannot-be-judged-by-a-positional-gate](../knowledge/an-llm-constraint-record-cannot-be-judged-by-a-positional-gate.md)
  > does the LLM constraint path apply positional subject gates
- [constraint-record-producer-strata](../knowledge/constraint-record-producer-strata.md)
  > does the LLM constraint path use classify_signal_constraint_kind
- [inference-antecedent-local-grounding-stops-at-semantic-layer-d](../knowledge/inference-antecedent-local-grounding-stops-at-semantic-layer-d.md)
  > does the Layer D grounding filter need to be weakened
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > does the NLI verifier actually catch real extraction errors
- [persisted-table-kind-is-a-classifier-generation-artefact](../knowledge/persisted-table-kind-is-a-classifier-generation-artefact.md)
  > does the SourceIR classifier type a bus-mode matrix as a signal table
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > does the USB4 Inter-Domain adapter still emit a USB4 signal
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > does the administrative classifier name OpenCAPI or a vendor
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > does the agent-identity gate keep Class-B fragments like Subordinate extends
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > does the alpha harness expose transform recipes to production core
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > does the authority empty repair preserve formal and system contract interfaces
- [caption-repair-corpus-selection](../knowledge/caption-repair-corpus-selection.md)
  > does the caption repair admit serialized table rows
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > does the co-reference rule resurrect resolve_indexed_signal_family
- [one-record-per-obligation-clause](../knowledge/one-record-per-obligation-clause.md)
  > does the constraint record source_text become the clause
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does the converge NLI pass measure quality or demote unsupported intent
- [one-modal-vocabulary-per-constraint-record](../knowledge/one-modal-vocabulary-per-constraint-record.md)
  > does the corpus contain a negated value binding the extractor cannot read
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does the current FSMGen ISF support named bit-fields inside a storage var (NO — opaque (var NAME (width N)) only on pin 030f8c273; set-field/extract are runtime ops not a declaration)
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > does the current SWD chain come from the tracked ADI PDF
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > does the current SpecForge binary still fabricate AIA TOC timing constraints
- [bounded-decision-frozen-baseline](../knowledge/bounded-decision-frozen-baseline.md)
  > does the declaration reader publish any false positive declaration
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > does the drop model agree with docling iterate_items
- [constraint-record-producer-strata](../knowledge/constraint-record-producer-strata.md)
  > does the dynamic constraint path use classify_signal_constraint_kind
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > does the emitted .isf signal direction affect FSMGen downstream correctness
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > does the explicit-direction signal catalog repeat the UTF-8 boundary bug
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > does the external SSD project directory contain the USB4 Inter-Domain Service PDF
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > does the extractor cut identifiers at the underscore character
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > does the figure carrier change artifacts already on disk
- [gate-and-replay-costs-are-affordable](../knowledge/gate-and-replay-costs-are-affordable.md)
  > does the full CI-tier doctrine gate pass
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > does the ingest lose underscores
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > does the legal statement classifier use a vendor or document denylist
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > does the live-document registry reject unknown fields oversized arrays or oversized scalars
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > does the move portability repair change PathBuf JSON fields
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > does the path portability contract cover FigureRegion raw images
- [per-row-column-drift-rule](../knowledge/per-row-column-drift-rule.md)
  > does the per-row drift rule change consistent tables
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > does the persisted path gate cover dormant schemas
- [legacy-source-classifications-are-neutralized-on-load](../knowledge/legacy-source-classifications-are-neutralized-on-load.md)
  > does the persisted table_kind field carry authority
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > does the placeholder rule use a list of placeholder words
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
- [workflow-standard-capacity-is-rederived-from-explicit-member-growth](../decisions/0043-workflow-standard-capacity-is-rederived-from-explicit-member-growth.md)
  > does the workflow catalog fit the full 21-file profile
- [evidence-proof-binds-artifact-location](../knowledge/evidence-proof-binds-artifact-location.md)
  > does this break repository portability (not for a repository move — artifact_layout stores repository-root-relative paths and tools derive absolute paths at runtime. It breaks relocation WITHIN the repository, which is what a read-only evaluation on a temp copy needs, which is why the relocation seam exists)
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
- [node-status-has-two-declaration-shapes](../knowledge/node-status-has-two-declaration-shapes.md)
  > does widening declared_node_statuses to the inline form change any existing migrated tree (no - claim_verification 0, pressure_headroom 0, spec_to_intent 1, before and after)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > fresh empirical reconfirmation of the .2i body-emission parking on the current 030f8c273 binary
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > has SPEC-TO-INTENT-ALIGNMENT task evidence migrated
- [claim-verification-task-evidence-migrated](../knowledge/claim-verification-task-evidence-migrated.md)
  > has the CLAIM-VERIFICATION-ADOPTION task evidence migration completed
- [row-keyed-matrix-obligations](../knowledge/row-keyed-matrix-obligations.md)
  > has the LLM-primary path ever carried a table row key into a constraint condition (no — 7 of 7 lose it; llm_sigcon_0017 is the only one with a condition_text and it holds the sentence's own predicate, connected to Manager LRMPAM .MPAM_NS input, not the row key)
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > has the SPEC-TO-INTENT-ALIGNMENT task evidence migration completed
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
- [mdbook-quantitative-census-freeze](../knowledge/mdbook-quantitative-census-freeze.md)
  > how are authored examples identity literals and dated book measurements excluded
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > how are bit location | register description | attributes tables extracted
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > how are bits | name | function tables extracted
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how are byte location | size | register description tables extracted
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > how are caption-less page fragments of a split table stitched together
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how are cited falsification controls bound to known-bad RED evidence
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how are continuation table fragments (B1.1 Continued from previous page) chained to a channel role
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > how are different health and ceiling file counts declared
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
- [behavioral-semantic-negative-sensitivity](../knowledge/behavioral-semantic-negative-sensitivity.md)
  > how are missing stale vacuous ambiguous or partial behavioral attempts classified
- [identifiers-are-opaque-and-one-way-grounded](../decisions/0037-identifiers-are-opaque-and-one-way-grounded.md)
  > how are model proposed signal names grounded
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how are multi-word field names like Validation Bits or FRU ID recovered
- [book-quantitative-candidate-vocabulary](../knowledge/book-quantitative-candidate-vocabulary.md)
  > how are new book quantitative regions adjudicated (excluded with scope_reason dated_boundary_evidence when the sentence or its lead-in anchors the figure to a completed measurement or a superseded state — measured, found, removed, restored, the old IntentIR, a before-and-after; incomplete with all three legs otherwise, which is the conservative default because it declares the
  > legs are missing rather than excusing the figure)
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how are offset-suffixed bit cells like 31:28 +04 extracted
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > how are per-instance indexed signals (PSELx HSELx) referenced in prose handled
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > how are per-state actions captured
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > how are project rescan working directories kept portable
