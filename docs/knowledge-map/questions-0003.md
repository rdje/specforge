# Knowledge questions — shard 0003

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > does .5.i change anything besides enums (yes, beneficially — dropped Enum statements leave discovered_values, so off-gold junk value-constraints derived from junk-enum members also disappear, e.g. AXI ACTIVATEACK A -> grounded ACTIVATEACK 1; distinct constraint facts identical, WIRE-BASED-100 unaffected)
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > does 24 measurable mean 24 documents produce a score
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > does AXI mint a signal from a prose name cell
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
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > does I2C extraction remain invariant under opaque signal renaming
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
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > does SpecForge declare a signal for a `<name> _in` template row
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > does SpecForge delete non-applicable physical timing records
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > does SpecForge detect implementation-defined or TBD or and/or
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does SpecForge emit register bit-fields to ISF now (YES — DOC-INTENT-TAXONOMY.4a.ii: the storage var carries a (fields (field …)) block; 6,570 fields / 2,531 registers / 24 docs, was 0; 4 wire golds byte-identical; 0 new fsmgen --strict diagnostics)
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > does SpecForge ingest lose content from the PDF
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > does SpecForge lower register reset values into the .isf (YES as of ISF-REGISTER-RESET-EMIT.2/.3 — composed from per-field reset_value and emitted at the true register width; it was dropped at the emit boundary before)
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > does SpecForge model-check temporal properties
- [cat3-topology-fsmgen-actor-network-reassessment](../knowledge/cat3-topology-fsmgen-actor-network-reassessment.md)
  > does SpecForge need to file a topology feature request now (not before .4c.ii measures the current contract and carrier fit)
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > does SpecForge preserve certification workflow source evidence
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > does SpecForge read `Master -> Slave` as a direction
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > does SpecForge read an enumerated width set like 8, 16, 32, 64
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > does SpecForge read the Slave -> Master arrow direction form
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > does SpecForge reject a page metadata staging traversal or symlink escape
- [contested-priors](../knowledge/contested-priors.md)
  > does SpecForge revise or decay priors
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > does SpecForge uppercase declared signal names
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
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > does a figure label appear in content_elements
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > does a figure that produced waveform contracts get a residual
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > does a live-document size warning always mean the surface is growing (no — an extremal dimension such as line_bytes_each can sit at rollover on a file that has not changed size at all)
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > does a metadata rewrite failure preserve the previous normalized bundle
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > does a replaced constraint surface get polarity refinement (apply_persisted_polarity_to_constraints)
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > does a research record with a live writer need a rollover instead of a partition
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > does a valid claim registry record prove the assertion true
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does adapter output reconciliation delete unrelated files or directories
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > does adaptive batch sizing change the ingest output / break byte-identity
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > does adding a sub-field to extraction_manifest un-seal the persisted corpus
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > does agreement between sources boost confidence
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > does an actor signal relation alone authorize a SemanticIR interface signal
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > does an empty declared-signal catalog disable the SemanticIR grounding filter
- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > does an explicit timing row unit override a caption unit
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > does any command report the live record count of a rolling ledger
- [book-behaviour-currency-instrument](../knowledge/book-behaviour-currency-instrument.md)
  > does any gate catch a book paragraph describing deleted behaviour
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > does any gate fail when a persisted chain falls below the current canonical schema
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > does canonical SWD EvidenceIR contain interface edge timing
- [project-data-locality-enforcement](../knowledge/project-data-locality-enforcement.md)
  > does cargo test need TMPDIR set manually
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > does changing a source path mean a corpus document was refreshed
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > does check_chain_currency re-run ingest
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > does claim verification replace task trees or doctrine enforcement
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
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > does current Docling lose captions
- [cat3-topology-fsmgen-actor-network-reassessment](../knowledge/cat3-topology-fsmgen-actor-network-reassessment.md)
  > does current FSMGen have any static actor instance or group construct (yes at a51dcdad0 — bounded actor instances/groups plus transaction-scoped actor/pin handoffs)
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > does docs/research have a file-count ceiling
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > does emitting (input) signals break fsmgen --strict (no — 0 new diagnostics; drives are suppressed for inputs)
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > does encryption block the VLM from reading tables (no)
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > does enrich / audit-extraction / recover-register-bits read full-page images or region images
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > does eval-extraction rebuild evidence or load the persisted file
- [task-tree-node-forms](../knowledge/task-tree-node-forms.md)
  > does every Frontier -> .x mention in a task tree resolve to a real leaf
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > does every SWD protocol record receive an ISF adapter disposition
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > does every temporal_rule reach the .isf or a residual
- [protocol-evidence-is-generic-and-document-derived](../decisions/0035-protocol-evidence-is-generic-and-document-derived.md)
  > does generic protocol evidence still project losslessly to IntentIR
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
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does platform/system-IP (category 3) topology intent need a new ISF construct or map onto an existing one (ISF has NO declarative static-topology/connectivity construct — composition is transaction-level only; decision deferred to a capture-recall measurement .4c.i before any FR)
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > does raising max_facts alone create fact-card headroom
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > does re-ingesting lose three paragraphs
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > does reading the arrow form fix the four documents that lose every row
- [flow-arrow-direction-grammar](../knowledge/flow-arrow-direction-grammar.md)
  > does reading the arrow form recover Avalon's eight signals
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > does refreshed in the corpus frontier census mean the document is at the current schema
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > does refusing a phrase name cell recover the wire the row was hiding
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > does register extraction require the table_kind register classification
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > does relation-derived direction synthesis independently invent signal names
- [semantic-section-phases-require-heading-authority](../knowledge/semantic-section-phases-require-heading-authority.md)
  > does removing sentence fallback remove address phase recognition
- [identifiers-are-opaque-and-one-way-grounded](../decisions/0037-identifiers-are-opaque-and-one-way-grounded.md)
  > does renaming a signal change semantic extraction
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
- [inference-antecedent-local-grounding-stops-at-semantic-layer-d](../knowledge/inference-antecedent-local-grounding-stops-at-semantic-layer-d.md)
  > does the Layer D grounding filter need to be weakened
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > does the NLI verifier actually catch real extraction errors
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
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > does the co-reference rule resurrect resolve_indexed_signal_family
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > does the converge NLI pass measure quality or demote unsupported intent
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > does the current FSMGen ISF support named bit-fields inside a storage var (NO — opaque (var NAME (width N)) only on pin 030f8c273; set-field/extract are runtime ops not a declaration)
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > does the current SWD chain come from the tracked ADI PDF
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > does the current SpecForge binary still fabricate AIA TOC timing constraints
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > does the drop model agree with docling iterate_items
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
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > does the legal statement classifier use a vendor or document denylist
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > does the live-document registry reject unknown fields oversized arrays or oversized scalars
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > does the move portability repair change PathBuf JSON fields
