# Knowledge questions — shard 0003

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > do SemanticIR and IntentIR preserve timing units and table provenance
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > do SourceIR and EvidenceIR keep absolute paths in memory
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > do all corpus docs build through the pipeline without failure
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > do any chip-spec PDFs need a real password (no)
- [swd-protocol-convergence-snapshots-are-exact](../knowledge/swd-protocol-convergence-snapshots-are-exact.md)
  > do convergence fact counts include serial frame fields
- [constraint-record-producer-strata](../knowledge/constraint-record-producer-strata.md)
  > do dyn_sigcon records ever carry the untyped MustBeStable default
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > do equal SourceIR counts prove an identical artifact (no — 320/333/286/3652/527 all held while table_kind moved on 67 of 286 tables, section_kind on 56 of 527 and diagram_kind on 20 of 333)
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > do evidence/semantic/intent stream source_ir.json or load it all into memory
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > do field constraints pass the same grounding gates as signal constraints
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > do host-local source libraries define tracked corpus membership
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > do old SemanticIR gates create IntentIR behaviors or constraints
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > do old SemanticIR phases create IntentIR behaviors
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
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > do the register-classed empty-catalog documents deserve the same bar as protocol-classed ones
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > do the register_bits.rs tiling gates validate field order (NO — only width-sum + name-multiset; a row-jumbled flattened table could pass both gates with WRONG bits, so a deterministic-table reader is strictly more dangerous than the VLM front-end)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > do the wire docs (APB/AHB/AXI/SWD) change when register reset is lowered to ISF (no — ZERO composable resets, .isf byte-identical, WIRE-BASED-100 holds trivially)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > do the wire-gold .isf carry over-width value literals (only AXI ihi0022_l has one — AWCMO; and AXI already fails strict on the orthogonal (port expr) error; APB/AHB/SWD have none; WIRE-BASED-100 measures extraction F1 not .isf bytes so it is orthogonal)
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > do transactions lower to .isf (only those with composed steps; signal-set/channel/phase membership is recognised-but-unlowered)
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > do unfiltered SemanticIR conditional rules reach an emitted .isf
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > does (sample S) work for an interface OUTPUT signal too (yes — FSMGen does not gate sample on direction)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > does .2i per-phase grouping change the emitted .isf (no — phase_membership is IntentIR metadata, the emitter lowers steps not it; byte-identical on all 4 wire docs)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > does .2m change the emitted .isf or the WIRE-BASED-100 surfaces (no — provably orthogonal)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > does .5.i change anything besides enums (yes, beneficially — dropped Enum statements leave discovered_values, so off-gold junk value-constraints derived from junk-enum members also disappear, e.g. AXI ACTIVATEACK A -> grounded ACTIVATEACK 1; distinct constraint facts identical, WIRE-BASED-100 unaffected)
- [corpus-canonical-currency-and-ownership](../knowledge/corpus-canonical-currency-and-ownership.md)
  > does 24 measurable mean 24 documents produce a score
- [subject-resolution-admits-a-width-verified-full-width-slice](../decisions/0047-subject-resolution-admits-a-width-verified-full-width-slice.md)
  > does ADR 0047 weaken ADR 0037's opaque-identifier rule
- [logic-level-walk-stops-at-eleven-unrelated-words](../knowledge/logic-level-walk-stops-at-eleven-unrelated-words.md)
  > does AERR is always driven LOW produce a constraint
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > does AXI mint a signal from a prose name cell
- [refresh-completion-is-not-artifact-currency](../knowledge/refresh-completion-is-not-artifact-currency.md)
  > does CHAIN-CURRENCY reporting 78 chains current contradict the canonical loader refusing 51 (no — CHAIN-CURRENCY measures stage-local reproducibility, that each stage still replays from its persisted upstream; canonical loadability is a different property and ADR 0048 governs it)
- [a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest](../knowledge/a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest.md)
  > does COMMIT.md still require a full manual check_doctrines.sh run
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
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > does SemanticIR record a declaration it refused for an unreadable width
- [escaped-identifier-fragments-the-catalog](../knowledge/escaped-identifier-fragments-the-catalog.md)
  > does SourceIR carry the markdown escape
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
- [actor-taxonomy-grows-in-pairs-not-terms](../knowledge/actor-taxonomy-grows-in-pairs-not-terms.md)
  > does SpecForge read a direction cell containing an arrow as an actor name (no - .2g guards it: infer_signal_direction_from_actor_text declines any cell carrying a FLOW_ARROW_FORMS or FLOW_ARROW_DISQUALIFIERS marker, ahead of even the literal input/output substring readings, so the flow reader judges it)
- [a-dropped-declaration-row-is-usually-not-a-signal](../knowledge/a-dropped-declaration-row-is-usually-not-a-signal.md)
  > does SpecForge read a width written as N bit or N bits
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > does SpecForge read an enumerated width set like 8, 16, 32, 64
- [declaration-reader-drops-uninterpretable-rows](../knowledge/declaration-reader-drops-uninterpretable-rows.md)
  > does SpecForge read the Slave -> Master arrow direction form
- [docling-page-sidecar-paths-are-portable](../knowledge/docling-page-sidecar-paths-are-portable.md)
  > does SpecForge reject a page metadata staging traversal or symlink escape
- [contested-priors](../knowledge/contested-priors.md)
  > does SpecForge revise or decay priors
- [one-modal-vocabulary-per-constraint-record](../knowledge/one-modal-vocabulary-per-constraint-record.md)
  > does SpecForge type an obligation stated with cannot or will not
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > does SpecForge uppercase declared signal names
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > does SpecForge use LTL CTL or TLA+
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > does SpecForge use the full scope of a page's visual information
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > does Wishbone declare its signals in a table
- [a-single-index-bit-cell-is-a-width-of-one](../knowledge/a-single-index-bit-cell-is-a-width-of-one.md)
  > does a Bits column write a single-bit signal as [n] or [n:n]
- [dormant-serialized-paths-require-portability](../knowledge/dormant-serialized-paths-require-portability.md)
  > does a PathBuf need portability handling before it has a producer
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > does a SIGKILL prove that Docling ran out of memory
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does a blocked adapter remove a previously emitted isf
- [self-test-coverage-guard-is-in-the-exit-path](../knowledge/self-test-coverage-guard-is-in-the-exit-path.md)
  > does a check notice when one of its own self-test cases is deleted
- [toolbox-catalog-is-a-routed-landing](../knowledge/toolbox-catalog-is-a-routed-landing.md)
  > does a citation like TOOLBOX.md section 7.7 still resolve after the toolbox partition (yes - section numbers were deliberately preserved and every cited number is still named in the landing, which routes it to its part in one hop; 26 such citations exist and several sit in sealed archive segments whose source end can never be repaired)
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > does a clean checkout or a CI runner pay for the corpus replay build
- [persisted-census-measures-published-not-current](../knowledge/persisted-census-measures-published-not-current.md)
  > does a count over generated/evidence_ir measure current extractor behaviour
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > does a debug build and a release build accept the same persisted artifact
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > does a deterministic semantic->intent rebuild recover lost actor_signal_relations
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > does a document ever spell the joined form of a split name cell
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
- [a-gate-doctrine-has-no-stable-wall-clock-on-this-host](../knowledge/a-gate-doctrine-has-no-stable-wall-clock-on-this-host.md)
  > does a stale tree make the live-document-size gate slower
- [a-level-belongs-to-a-signal](../knowledge/a-level-belongs-to-a-signal.md)
  > does a subscript separate a signal from its level
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > does a valid claim registry record prove the assertion true
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > does adapter output reconciliation delete unrelated files or directories
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > does adaptive batch sizing change the ingest output / break byte-identity
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > does adding a sub-field to extraction_manifest un-seal the persisted corpus
- [producer-change-and-corpus-rebuild-are-one-transaction](../knowledge/producer-change-and-corpus-rebuild-are-one-transaction.md)
  > does adding any Rust production function invalidate the persisted corpus (no — only a change to a REGISTERED evidence derivation does; EXTRACTION-QUALITY-GAUGE.3j.2.c.i added four production functions to the LLM-primary grounding path on the same day and every artifact kept loading, because that path is not part of the evidence derivation)
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
- [a-split-name-cell-does-not-say-which-join-it-wants](../knowledge/a-split-name-cell-does-not-say-which-join-it-wants.md)
  > does eMMC table_0221 declare from t PERIOD
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > does effective_table_kind promote an unknown table
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
- [a-dropped-declaration-row-is-usually-not-a-signal](../knowledge/a-dropped-declaration-row-is-usually-not-a-signal.md)
  > does every column header being a name header separate a signal grid from a legend
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
- [a-relational-predicate-is-not-a-value](../knowledge/a-relational-predicate-is-not-a-value.md)
  > does must_be_value GREATER exist in the corpus
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > does name_cell_is_read_whole cost any recall
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > does parse_explicit_signal_declaration keep the direction when it cannot read the width
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > does platform/system-IP (category 3) topology intent need a new ISF construct or map onto an existing one (ISF has NO declarative static-topology/connectivity construct — composition is transaction-level only; decision deferred to a capture-recall measurement .4c.i before any FR)
