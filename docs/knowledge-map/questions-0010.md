# Knowledge questions — shard 0010

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > how much page content escapes both the structured-element path and the region-crop path
- [bounded-task-root-accretes-in-its-frontier](../knowledge/bounded-task-root-accretes-in-its-frontier.md)
  > how much should one closure cost a bounded task root
- [a-sealed-region-cannot-move-out-of-an-active-part-alone](../knowledge/a-sealed-region-cannot-move-out-of-an-active-part-alone.md)
  > how much writable budget does relocating a sealed region actually return (the active part stops owning any region, so its whole health target becomes spendable rather than the target minus the sealed payload)
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > how should Rust version copies be verified across Cargo README book and CI
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > how should a behavior-preserving evidence refactor be verified given the non-determinism
- [llm-primary-recall-ceiling](../knowledge/llm-primary-recall-ceiling.md)
  > how should a precision figure from the LLM-primary path be read (as a statement about 18% of the population — every .3j result, the 3/7 gate precision, the 36 ungrounded subjects, the 16 carried names, the 7 row-keyed records, measures model behaviour on what the deterministic extractors already found)
- [claim-standard-upstream-readoption](../knowledge/claim-standard-upstream-readoption.md)
  > how should a red check be attributed to a change
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > how should an AXI gold or extraction be structured (per channel)
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > how should the agent-identity / actor noise gate stay agnostic (no name list, ADR 0006)
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > how should the current FSMGen gitlink in documentation be verified
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > how to compose a register-level ISF reset from per-field reset_value (LSB-tiling: V = OR(parse_int(reset_i) << bits_low_i), mirroring ir/register_bits.rs; only when every field is located + parseable non-neg int fitting its field width + no overlap)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how to exploit that same-vendor / same-brand PDFs share organization without hardcoding vendor names
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > how to express the JTAG TAP / SWD FSM in .isf
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > how to get a fresh eval-extraction baseline for a spec
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how to re-measure the message-field corpus yield
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how to re-score SWD derivation
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > how to read a chip-spec PDF when the Read tool refuses it
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > how to recover register field bit positions that live in the layout graphic, not the table
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > how was AHB HREADY recovered for the temporal antecedent
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > how was AXI constraint precision fixed
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > how was the 400 page Docling SIGKILL reproduced
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > how was the CHI field-constraint routing measured without re-ingesting the PDF
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > how was the agent-identity gate proven not to drop real agents (>=8-port proxy, WIRE-BASED-100)
- [a-single-index-bit-cell-is-a-width-of-one](../knowledge/a-single-index-bit-cell-is-a-width-of-one.md)
  > how was the bit-range census error caught
- [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md)
  > how was the eval gold checked for idiosyncrasy
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > how was the migrated alignment task evidence independently audited
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > how was the trailing aux/prep strip proven safe for WIRE-BASED-100 and real agents
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > how was the trailing-fragment consolidation proven not to regress real agents (WIRE-BASED-100)
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > how were I2S timing_constraints recovered (clock period / clock HIGH / set-up / hold)
- [trajectory-steering-is-a-reviewable-multimetric-control-loop](../decisions/0034-trajectory-steering-is-a-reviewable-multimetric-control-loop.md)
  > how will SpecForge automatically detect convergence or divergence
- [a-tie-off-is-not-a-drive](../knowledge/a-tie-off-is-not-a-drive.md)
  > how will a tie-off reach .isf (FSMGen absent-input value, EXTRACTION-REPRESENTATION-GAPS.4)
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > how will the million-byte Knowledge Map be sharded without losing question retrieval
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > how will the oversized SpecForge roadmap be made bounded without losing its history
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > how will the shared rolling ledger archive index be partitioned
- [retained-chain-rebuild-order](../knowledge/retained-chain-rebuild-order.md)
  > in what order do I rebuild a retained corpus chain
- [partitioning-a-task-tree-has-a-fixed-registration-price](../knowledge/partitioning-a-task-tree-has-a-fixed-registration-price.md)
  > in what order do I run a task-tree partition (write the closing record, commit, correct every node Status, commit, lock, migrate)
- [self-test-coverage-guard-is-in-the-exit-path](../knowledge/self-test-coverage-guard-is-in-the-exit-path.md)
  > is $passed/$total a coverage guard
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is 'X, which connects to Y, drives Z and W' clause distribution handled
- [an-assume-sentence-about-the-document-is-apparatus](../knowledge/an-assume-sentence-about-the-document-is-apparatus.md)
  > is 'treated as' still an abstraction after .3.d.ii.b.i (yes, whatever the sentence says)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is (sample input as name) value-free in ISF (yes; (drive input) is rejected — drives exist only for outputs)
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > is 64 the status ledger record count
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > is AArch64 External Debug a methodology guide or under-extracted architecture
- [persisted-llm-constraint-corpus-predates-catalog-grounding](../knowledge/persisted-llm-constraint-corpus-predates-catalog-grounding.md)
  > is APB's PSEL a declared signal as far as entity typing is concerned (no — APB's catalog holds PSELX and PSELXCHK because the document declares the parameterised template PSELx, and resolve_unique_document_identifier is exact-then-case-fold, so the bare PSEL cannot resolve; a refusal of PSEL is the resolver working, not a broken census)
- [declared-spelling-is-the-document-spelling](../knowledge/declared-spelling-is-the-document-spelling.md)
  > is ARESETN in a persisted artifact a current defect
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > is AXI/SWD per-signal phase membership recoverable from timing diagrams via a VLM
- [a-dropped-declaration-row-is-usually-not-a-signal](../knowledge/a-dropped-declaration-row-is-usually-not-a-signal.md)
  > is AxLEN a declared AXI signal
- [a-gate-doctrine-has-no-stable-wall-clock-on-this-host](../knowledge/a-gate-doctrine-has-no-stable-wall-clock-on-this-host.md)
  > is COMMIT-GATE-SINGLE-RUN.0's per-doctrine cost table reliable
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > is CORPUS-PATTERN-REUSE.3b.3a a go or no-go
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > is DOC-INTENT-TAXONOMY.4a.ii buildable now (DONE 2026-06-22 — implemented once FSMGen shipped the construct; superseded .4a.i; Gap B packet/flit still deferred)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > is ISF a single-actor or multi-actor format (per-actor — one .isf describes one actor/module; SpecForge's emit collapses to one initiator via select_initiator_actor; lowering cross-component topology would need a multi-actor emit, an architectural change not an emitter tweak)
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > is ISF the current SpecForge bottleneck
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is KG-ISF-TRANSACTIONS.2i unparked / what is the .2i decision (yes — FSMGen confirmed option a: ship the grounded per-phase membership grouping as IntentIR metadata, .isf byte-identical)
- [one-modal-vocabulary-per-constraint-record](../knowledge/one-modal-vocabulary-per-constraint-record.md)
  > is MustBeValue plus negated ever produced by a deterministic path
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > is PDF to SourceIR conservation measured
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > is PSEL is asserted read in IntentIR (yes, as a contract guarded on PSELx; only the invariant reader ignores the co-reference)
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > is Perl or Python the right language for editing a doctrine JSON contract (neither - language is the wrong axis. The bytes are decided by indent width, key order, ensure_ascii, space_before and the trailing newline, and both languages reproduce the same bytes once those are matched)
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > is RISC-V Debug register bit-position recovery a deterministic text-table parse or a VLM-image read (VLM-image — 53/56 diagrams are images, the 7 flattened tables are garbled/symbolic; deterministic parse would fabricate)
- [persisted-table-kind-is-a-classifier-generation-artefact](../knowledge/persisted-table-kind-is-a-classifier-generation-artefact.md)
  > is SIGNAL-DECLARATION-ROW-DROP.2c still blocked on a guard
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > is SPEC-TO-INTENT-ALIGNMENT.6d.ii.f eligible after containment
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > is SPECFORGE_INGEST_BATCH_PAGES a fixed size or a ceiling
- [swd-serial-frame-score-retired-by-genericity](../knowledge/swd-serial-frame-score-retired-by-genericity.md)
  > is SWD at 100% / does the SWD derivation gold still score 29/29 (NO. Re-derived 2026-09-01 with the restored oracle: serial_frame_field 0/11 and protocol_state 0/13, protocol_operation 4/4 and interface_edge_timing 1/1 — document-level 5/29. The 29/29 is retired, not current)
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > is SWD at 100% and on what metric (NO, NOT ANY MORE — this card records the 2026-06-07 result and is SUPERSEDED. Re-derived 2026-09-01 it is 5/29: operations 4/4 and edge timing 1/1 hold, frame fields 0/11 and states 0/13 do not. See swd-serial-frame-score-retired-by-genericity)
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > is SourceIR covered by the chain currency doctrine
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > is SpecForge PDF ingest deterministic
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > is SpecForge converging diverging stalled mixed or unmeasurable
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > is SpecForge specification mining
- [source-library-authority-is-ssd-local](../knowledge/source-library-authority-is-ssd-local.md)
  > is SpecForge still allowed to read chipdoc from the boot volume
- [claim-standard-upstream-readoption](../knowledge/claim-standard-upstream-readoption.md)
  > is SpecForge's CLAIM_VERIFICATION.md a copy of the upstream standard
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > is SpecForge's constraint extraction over-generating
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > is TCU_NODE_CTRL n a footnote or an array index
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > is VALIDATION_SNAPSHOT the latest local artifact validation or the last reviewed projection
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > is a Default column a register reset
- [decimal-before-binary-needs-a-notational-witness](../knowledge/decimal-before-binary-needs-a-notational-witness.md)
  > is a bare 10 in an encoding table binary or decimal
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > is a better VLM needed to read register bit-layout diagrams
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > is a candidate-row count the same as a declaration count
- [document-class-is-decided-at-build-but-written-by-validate](../knowledge/document-class-is-decided-at-build-but-written-by-validate.md)
  > is a census that returns zero a finding
- [an-untyped-signal-table-is-read-by-its-direction-cells](../knowledge/an-untyped-signal-table-is-read-by-its-direction-cells.md)
  > is a clock enable a clock
- [deterministic-constraint-recall-is-bounded-by-classification](../knowledge/deterministic-constraint-recall-is-bounded-by-classification.md)
  > is a constraint the signal_constraints surface drops actually lost (not necessarily — of the 319 dropped, 63 are held by the conditional-rule surface and 9 by actor-signal relations, leaving 247 held by no persisted surface at all; reporting 319 as losses overstates it)
- [quoted-literal-notation-carries-the-radix](../knowledge/quoted-literal-notation-carries-the-radix.md)
  > is a count-based acceptance bar enough for a parser fix
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > is a defaulted (output) port in the emitted .isf an honest residual (no — it states the initiator drives the signal; EXTRACTION-REPRESENTATION-GAPS.27)
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > is a doctrine JSON contract's encoder stable (NO - 31 encoder flips across 12 of the 23 contracts with history, measured 2026-09-21 by LIVE-DOCUMENT-PRESSURE-HEADROOM.39; a per-file encoder is a reading of the last writer, not a property of the file)
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > is a fresh ingest better than the persisted SourceIR
- [gate-and-replay-costs-are-affordable](../knowledge/gate-and-replay-costs-are-affordable.md)
  > is a full doctrine run too slow to do in a session
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > is a greater than 500 page threshold sufficient for bounded PDF ingestion
- [a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest](../knowledge/a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest.md)
  > is a green --fast a green doctrine gate
- [enum-member-values-have-no-oracle](../knowledge/enum-member-values-have-no-oracle.md)
  > is a green WIRE-BASED-100 evidence that enums are correct
- [a-register-layout-table-sizes-the-register-its-heading-names](../knowledge/a-register-layout-table-sizes-the-register-its-heading-names.md)
  > is a layout table's Size column in bytes or bits
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > is a line_bytes_each warning the same kind of pressure as a lines_each or bytes_each warning (NO, but not because one is a maximum and the others are not — check_live_document_size.pl computes ALL THREE _each dimensions as per-surface maxima. What differs is the quantity being maximized: lines_each and bytes_each maximize a per-FILE quantity that accumulates as content is
  > added, while line_bytes_each maximizes a per-LINE quantity that does not, so it has no growth driver and is freely reducible at any time)
- [row-keyed-matrix-obligations](../knowledge/row-keyed-matrix-obligations.md)
  > is a lost table row key a source-assembly gap or a proposal-shape defect (both, and which one depends on the table — for 104 of 109 rows the binding is inside the span and a span-local rule can reach it; for 5 the column header binds the other axis and lives in a DIFFERENT statement, so the span cannot express the scope even in principle)
- [wire-golds-held-out-not-lost](../knowledge/wire-golds-held-out-not-lost.md)
  > is a missing normalized bundle the same as a lost normalized bundle
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is a name-only gate enough to fix the generic enum (no — 271 real-named enums like COMMAND/DWORD_MISR/AMBA are themselves fragment-heavy/dup-heavy; the load-bearing signal is member quality)
- [a-check-coverage-table-binds-the-class-of-covered-signals](../knowledge/a-check-coverage-table-binds-the-class-of-covered-signals.md)
  > is a parity check signal lowered to isf (no: EXTRACTION-REPRESENTATION-GAPS.46 decides)
- [docling-tables-live-under-documents-in-a-batched-bundle](../knowledge/docling-tables-live-under-documents-in-a-batched-bundle.md)
  > is a persisted SourceIR table premise a deterministic function of the retained backend.json (yes: 1,256 of 1,256 equal)
- [legacy-artifact-declaration-drift](../knowledge/legacy-artifact-declaration-drift.md)
  > is a persisted legacy evidence_ir what the current binary produces
- [llm-path-lacks-the-local-appositive-declaration](../knowledge/llm-path-lacks-the-local-appositive-declaration.md)
  > is a persisted llm_sigcon_ record with a subject the catalog refuses necessarily wrong (no — APB llm_sigcon_0000 PSEL|must_be_asserted is the canonical fact SPEC-TO-INTENT-ALIGNMENT.7a exists to recover, refused by an asymmetry between the two paths rather than by ADR 0037's principle)
- [persisted-table-kind-is-a-classifier-generation-artefact](../knowledge/persisted-table-kind-is-a-classifier-generation-artefact.md)
  > is a persisted table_kind evidence about the current classifier
- [a-frozen-pre-repair-contract-is-retired-not-regenerated](../decisions/0049-a-frozen-pre-repair-contract-is-retired-not-regenerated.md)
  > is a red validator that no gate runs a defect
- [an-uncited-obligation-is-a-candidate-miss](../knowledge/an-uncited-obligation-is-a-candidate-miss.md)
  > is a refused constraint reading still visible somewhere
- [local-repair-closes-the-caption-decision](../knowledge/local-repair-closes-the-caption-decision.md)
  > is a remote decision provider still worth running after arm B
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > is a repeated leading token enough to tell a split identifier from a phrase
- [arithmetic-width-drops-the-declaration](../knowledge/arithmetic-width-drops-the-declaration.md)
  > is a sentence opening with the word signal and no attribute a lost declaration
- [a-split-name-cell-does-not-say-which-join-it-wants](../knowledge/a-split-name-cell-does-not-say-which-join-it-wants.md)
  > is a subscript split the same defect as an underscore split
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is a transaction's phase ORDER recoverable from prose (no — first-occurrence wrong on SWD, cues sparse, precedence conflicting)
- [an-equivalence-rule-is-a-typed-record](../knowledge/an-equivalence-rule-is-a-typed-record.md)
  > is a treated-as sentence still an explicit abstraction (yes; the typed record cites it)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is a value-less drive valid in ISF / can an output participate in a transaction body without a value (no — every (drive …) needs a concrete actual; raised to FSMGEN)
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > is a width-only declaration with a fabricated width worth keeping
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is an ISF transaction body ordered (yes — totally ordered, one clause ≈ one cycle; same-cycle concurrency only via a multi-pair drive block)
- [direction-suffix-is-one-documents-convention](../knowledge/direction-suffix-is-one-documents-convention.md)
  > is an _IN/_OUT suffix universal digital-design grammar
- [a-signals-stated-absent-value-rides-its-presence-record](../knowledge/a-signals-stated-absent-value-rides-its-presence-record.md)
  > is an absent value interpreted against the signal's polarity
- [legacy-source-classifications-are-neutralized-on-load](../knowledge/legacy-source-classifications-are-neutralized-on-load.md)
  > is an empty result from a table-kind-keyed pass an answer or a blind spot
- [temporal-condition-read-whole](../knowledge/temporal-condition-read-whole.md)
  > is an unless clause an antecedent (no — an exception, kept apart and lowered negated)
- [empty-signal-catalog-is-mostly-honest-absence](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md)
  > is any corpus document ingested at fewer pages than its source PDF
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > is avoiding a new EVIDENCE_RULE_FIELDS entry enough to keep persisted proofs valid
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > is bar #5 (no silent drop of behavior/temporal) still honest after CORPUS-COVERAGE.0
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > is bar #6 (ISF round-trip, no silent drop) a large faithful-lowering gap
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > is canonical recovery complete in the current reviewed population
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > is captured_region_residuals an exact IntentIR carry
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > is collect_known_signal_names UTF-8 safe
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > is converge the complete production capability path
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > is dropping a zero-evidence actor safe across AXI vs AXI-Stream
- [escaped-identifier-fragment-adjudication](../knowledge/escaped-identifier-fragment-adjudication.md)
  > is escape fragmentation the only way an ordinary word becomes a declared signal (no — ACTOR-NOUN-RELATION-DECLARATION owns the same outcome reached by an inferred declaration, so a fix at the tokenization seam addresses only this route)
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > is first-header-appears-in-caption enough to detect a property table
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is header-sourced enum naming landed / what did KG-ISF-COMPLETENESS.5.iv.a do (LANDED 2026-08-31: derive_header_sourced_enum_name in ir/evidence.rs runs LAST, after the signal-match loop and the .5.i-gated caption fallback, so it is strictly additive. Five clauses: one header row of two cells; the right cell names a description role; the left cell carries no POSITIONAL role and
  > leaves exactly one identifier after structure/column-role words are stripped; no value cell is a positional range; at least one value cell parses as an encoding literal. 285 tables accepted in 9 documents. chain-currency 24/24 current at every stage, kg-bench 156/156, +8 tests)
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > is identifier truncation a separate extractor defect or the same empty-catalog gap
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > is intent-bearing content slipping through because nothing reads the full page
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > is it a strict error to drive a signal declared (input ...) in an .isf rule
- [a-table-premise-is-recaptured-from-retained-converter-output-not-re-ingested](../decisions/0052-a-table-premise-is-recaptured-from-retained-converter-output-not-re-ingested.md)
  > is it legitimate to change a SourceIR proof premise
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > is it safe to extend the .1b.i trailing-fragment strip to trailing prepositions and auxiliaries (host has -> host, advantage of -> advantage)
- [wire-golds-held-out-not-lost](../knowledge/wire-golds-held-out-not-lost.md)
  > is it safe to install a held-out normalized bundle
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > is it safe to measure a backup copy of an evidence_ir with nli-verify
- [a-cheap-structural-rule-overfires-until-you-read-its-selection](../knowledge/a-cheap-structural-rule-overfires-until-you-read-its-selection.md)
  > is it safe to put single-letter abbreviations in a notation census vocabulary (no - measured 0 true positives and 18 false ones corpus-wide for i/o/io/in/out; an abbreviation is not a notation until a document is shown to use it as one)
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > is losslessness enough when partitioning a canonical record (no — .4e proved a byte-exact partition can still break 14 section anchors while every doctrine passes)
- [protocol-evidence-is-generic-and-document-derived](../decisions/0035-protocol-evidence-is-generic-and-document-derived.md)
  > is lower recall acceptable when removing protocol-specific shortcuts
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > is mdbook test part of the canonical SpecForge CI gate
- [legacy-reclassification-is-not-the-binding-constraint](../knowledge/legacy-reclassification-is-not-the-binding-constraint.md)
  > is neutralized classification the reason legacy documents extract nothing
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > is one canonical probe per distinct seal a sample
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > is planned_live the current live window
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > is proof currency the same thing as normalized bundle retention
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > is qwen2.5:14b-instruct good enough for NLI
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > is relation-completeness the right bar dimension for register / command / coherency protocols
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is removing the generic enums WIRE-BASED-100-safe (scores ORTHOGONAL/SAFE — generic enums are in no scored gold; but the .isf BYTES change on all 4 wire golds — APB/AHB/AXI/SWP each emit a junk TABLE; AHB's TABLE fuses HTRANS+HSIZE which already have correct enums — a strict improvement needing a deliberate snapshot refresh, NOT byte-identical)
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > is replace_term_with_placeholder safe on non-ASCII signal/actor names
- [direction-column-drift](../knowledge/direction-column-drift.md)
  > is rows minus declarations a loss count
- [measured-stratum-promotion-population](../knowledge/measured-stratum-promotion-population.md)
  > is running the LLM promotion on a document with no constraints a no-op (no — with zero sentences the surface replace changes nothing, but promote_constraints still records a constraints.llm_primary surface manifest and authorize_mutation still appends a ConstraintPromotion record to the proof ledger, so the artifact claims a surface the model never saw)
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > is signal direction faithful to the document now or still defaulted to output
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > is source_ir.json bounded in size for very large PDFs
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is subject coordination (Actor1 and Actor2 drive X) handled / worth building
