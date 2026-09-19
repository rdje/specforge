# Knowledge questions — shard 0005

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > how are the three FSM grammars (SWD hyphen, quoted-mode, single-word) distinguished
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > how are two genuinely-different registers sharing a mnemonic (MEM-AP CSW vs JTAG-AP CSW) recovered instead of dropped
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > how are two-column bits | description tables extracted
- [vlm-table-strategy](../knowledge/vlm-table-strategy.md)
  > how are unknown tables reclassified by the VLM
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > how big does source_ir.json get / how does it scale with page count
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > how big is the header-sourced naming population really (994 candidates / 285 accepted in 9 documents, NOT the 134 in 10 that .5.iv reported. .5.iv censused table_kind == encoding only, but scan_encoding_tables_by_signal_anchor skips just signal-description/register-map/timing-parameter and table_looks_like_encoding then admits any name+value header, so unknown-kind tables are
  > in scope — and they carry the glossary/notation/abbreviation junk class .5.iv never saw. Reproducer scripts/measure_header_sourced_enum_naming.py)
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > how big may one LIVE_ACHIEVEMENT_STATUS record be
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > how can I browse every SpecForge knowledge fact card by id or title
- [trajectory-steering-is-a-reviewable-multimetric-control-loop](../decisions/0034-trajectory-steering-is-a-reviewable-multimetric-control-loop.md)
  > how can SpecForge automatically choose the next task without gaming metrics
- [canonical-collection-catalogs](../knowledge/canonical-collection-catalogs.md)
  > how can a collection use a membership index outside its own surface
- [task-plane-cardinality-is-removed-behind-a-declared-exemption](../decisions/0045-task-plane-cardinality-is-removed-behind-a-declared-exemption.md)
  > how can a live surface null a size dimension
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > how can a prose signal spelling bind to a declared one without a suffix inference
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > how can a script ask the SpecForge canonical loader without mutating the artifact
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > how can one compound sentence contain two independent signal facts
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how complete is a document's extracted intent / what is the per-doc completeness gauge
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > how complete is register-IP / platform-IP / CPU-ISA ISF lowering
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > how dense is SpecForge's captured component topology on cat-3 docs vs cat-1 wire docs (cat-3 = 0.355 edges/actor + 24% both-endpoint; cat-1 wire baseline = 4.108 edges/actor + 85% both-endpoint — the SAME signal_connectivity surface is ~12x denser and fully-connected on wire docs, so the surface is capable; the shortfall is capture-recall on platform TRMs)
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > how did APB temporal reach 100% (WIRE-BASED-100.4)
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > how did the AHB decoder become connected (Subordinate and decoder read HADDR)
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > how do Continued from previous page fragments find their home
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > how do I add a new enforced doctrine / doctrine check
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > how do I add a reviewed residual gold key
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > how do I add fact-card capacity
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how do I address exactly one converter item
- [constraint-record-producer-strata](../knowledge/constraint-record-producer-strata.md)
  > how do I census which span a published constraint read its parts from
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > how do I change the reviewed source-to-intent fixture projection
- [book-behaviour-currency-instrument](../knowledge/book-behaviour-currency-instrument.md)
  > how do I check the mdBook for stale behavioural claims
- [wire-golds-held-out-not-lost](../knowledge/wire-golds-held-out-not-lost.md)
  > how do I check whether a held-out bundle still rebuilds its document
- [task-tree-node-forms](../knowledge/task-tree-node-forms.md)
  > how do I check whether a named frontier has an owning task-tree leaf
- [persisted-table-kind-is-a-classifier-generation-artefact](../knowledge/persisted-table-kind-is-a-classifier-generation-artefact.md)
  > how do I check whether a persisted artifact field describes current behaviour
- [proof-seal-currency-gate](../knowledge/proof-seal-currency-gate.md)
  > how do I check whether the persisted corpus seal is stale
- [project-scratch-location](../knowledge/project-scratch-location.md)
  > how do I clean up scratch after a slice
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > how do I clear a live-document line_bytes_each warning (reflow the few widest lines; do not partition, roll over, or raise the ceiling — nothing is running out)
- [status-ledger-record-budget-and-count](../knowledge/status-ledger-record-budget-and-count.md)
  > how do I count the records in the status ledger
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > how do I decide between partitioning a research record and declaring a rollover for it
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > how do I declare a newly retained normalized bundle
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > how do I derive how many corpus refreshes remain (run scripts/check_corpus_frontier.sh; it derives the SourceIR cohort and requires every member in exactly one explicit refreshed or remaining set)
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > how do I edit one field of a doctrine JSON contract without reformatting the whole file (derive that FILE's encoder by round-tripping candidates against its exact bytes - there is no repository-wide style: of 27 parseable contracts under doctrine/, 20 reproduce under json.dumps with separators comma-space-colon and a trailing newline, 3 under Perl JSON::PP canonical+pretty, and
  > 4 are hand-authored and reproduce under neither)
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > how do I file an FSMGen bug report or feature request
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > how do I find out which documents are chain-stale
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > how do I get the clean LLM-primary constraint surface onto the canonical artifacts
- [a-cheap-structural-rule-overfires-until-you-read-its-selection](../knowledge/a-cheap-structural-rule-overfires-until-you-read-its-selection.md)
  > how do I know a structural rule's selection is clean (print every distinct selected form verbatim with its count and read them; when the population is small enough the adjudicable sample IS the population, as with the 83 flow-arrow cells in 13 forms)
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > how do I know my census script still mirrors the reader
- [sourceir-classification-is-per-record](../knowledge/sourceir-classification-is-per-record.md)
  > how do I let a Continued from previous page table contribute what its first page contributes
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > how do I make the NLI verifier actively change extraction / demote claims
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > how do I measure a classifier change's blast radius before shipping it
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > how do I measure a declaration population through the reader rather than through a filter
- [worktree-doctrine-measurement-gitlink](../knowledge/worktree-doctrine-measurement-gitlink.md)
  > how do I measure a doctrine checker across many revisions
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > how do I measure the extraction-quality gauge before and after a canonical promotion
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > how do I measure the parametric width cell population
- [book-quantitative-candidate-vocabulary](../knowledge/book-quantitative-candidate-vocabulary.md)
  > how do I measure what a candidate-grammar change would cost (drive the checker's own discover_candidates over its own derive_book_members — the baseline must reproduce --check exactly, 483 candidates across 26 files with 0 errors — then diff the candidate list with the noun added alone; --produce cannot be used because validate_contract runs first and aborts on the changed
  > denominator)
- [logic-level-walk-stops-at-eleven-unrelated-words](../knowledge/logic-level-walk-stops-at-eleven-unrelated-words.md)
  > how do I measure what a skip-list widening would newly admit
- [claim-verification-task-evidence-migrated](../knowledge/claim-verification-task-evidence-migrated.md)
  > how do I partition an oversized task tree
- [worktree-doctrine-measurement-gitlink](../knowledge/worktree-doctrine-measurement-gitlink.md)
  > how do I populate the fsmgen gitlink in a detached measurement worktree
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > how do I promote a document's constraint surface on canonical without re-ingesting the PDF
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > how do I prove a live-document partition is lossless
- [claim-verification-task-evidence-migrated](../knowledge/claim-verification-task-evidence-migrated.md)
  > how do I prove a task-evidence partition is lossless without trusting the writer
- [worktree-doctrine-measurement-gitlink](../knowledge/worktree-doctrine-measurement-gitlink.md)
  > how do I re-derive a published count per revision instead of at two endpoints
- [bounded-decision-frozen-baseline](../knowledge/bounded-decision-frozen-baseline.md)
  > how do I re-derive the bounded decision baseline
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > how do I re-derive the declaration-row notation populations
- [measured-stratum-promotion-population](../knowledge/measured-stratum-promotion-population.md)
  > how do I re-derive the measured-stratum promotion population (cargo test -p specforge --lib measured_stratum_promotion_population -- --ignored --nocapture; the crate is specforge, not specforge-core, because commands/** does not #[path] into core)
- [refresh-completion-is-not-artifact-currency](../knowledge/refresh-completion-is-not-artifact-currency.md)
  > how do I re-derive the refresh-versus-currency partition (read doctrine/corpus_frontier/census.json for the refreshed and remaining sets, then join it against each generated/source_ir/<doc>/source_ir.json proof ledger and each generated/evidence_ir/<doc>/evidence_ir.json schema_version)
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > how do I re-pin claim regions after editing a governed file (python3 scripts/repin_claim_regions.py --check then --apply; it resolves by content across all three registries and REFUSES ambiguity rather than taking the first match, which matters because a blank-line region matches every blank line in the file)
- [llm-primary-recall-ceiling](../knowledge/llm-primary-recall-ceiling.md)
  > how do I re-run the LLM recall ceiling measurement (cargo test -p specforge-core --lib llm_recall_ceiling -- --ignored --nocapture)
- [deterministic-constraint-recall-is-bounded-by-classification](../knowledge/deterministic-constraint-recall-is-bounded-by-classification.md)
  > how do I re-run the deterministic constraint recall measurement (cargo test -p specforge-core --lib constraint_recall_gap -- --ignored --nocapture)
- [escaped-identifier-fragment-adjudication](../knowledge/escaped-identifier-fragment-adjudication.md)
  > how do I re-run the escaped-identifier fragment census (python3 scripts/measure_escaped_identifier_fragments.py, and --self-test for its 8 cases)
- [persisted-llm-constraint-corpus-predates-catalog-grounding](../knowledge/persisted-llm-constraint-corpus-predates-catalog-grounding.md)
  > how do I re-run the llm_sigcon_* subject grounding census (cargo test -p specforge-core --lib llm_constraint_subject_grounding_census -- --ignored --nocapture; the crate is specforge-core because crates/specforge/src/ir/** compiles into it by #[path])
- [row-keyed-matrix-obligations](../knowledge/row-keyed-matrix-obligations.md)
  > how do I re-run the row-keyed obligation census (cargo test -p specforge-core --lib row_keyed_obligation_population -- --ignored --nocapture; the crate is specforge-core because crates/specforge/src/ir/** compiles into it by #[path])
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > how do I re-score the WIRE-BASED-100 golds
- [chomp-is-a-no-op-under-a-callers-slurp](../knowledge/chomp-is-a-no-op-under-a-callers-slurp.md)
  > how do I read a file line by line inside a function whose caller slurps
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > how do I read the labels inside a diagram
- [retained-chain-rebuild-order](../knowledge/retained-chain-rebuild-order.md)
  > how do I rebuild every retained chain after a production change
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > how do I rebuild the AXI APB AHB chains when their normalized bundles are held out
- [retained-bundle-population-is-frozen](../knowledge/retained-bundle-population-is-frozen.md)
  > how do I record a reclamation in retained_bundles.json
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > how do I recover the source row behind a table_signal_declaration_provenance entry
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > how do I refresh a stale SourceIR proof without a Docling re-ingest (source_proof_migrate --retained-manifest doctrine/chain_currency/retained_bundles.json --write, then re-run evidence/semantic/intent/adapt)
- [task-evidence-route-catalogs-shard-by-lifecycle](../decisions/0046-task-evidence-route-catalogs-shard-by-lifecycle.md)
  > how do I regenerate a task-evidence index
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > how do I regenerate controller_input.json and trajectory_report.json
- [a-sealed-region-cannot-move-out-of-an-active-part-alone](../knowledge/a-sealed-region-cannot-move-out-of-an-active-part-alone.md)
  > how do I relocate a sealed region to give an active part its budget back without breaking the gate (move the region AND every post-migration record that supersedes a declaration inside it)
- [evidence-proof-binds-artifact-location](../knowledge/evidence-proof-binds-artifact-location.md)
  > how do I relocate a verified EvidenceIR without breaking its proof (EvidenceIr::load_relocated_to_artifact_base_root — it verifies the artifact where it is, moves it to <base>/<document_key>/evidence_ir.json, and re-derives the proof for the new location from the same verified SourceIR prefix and the same sealed proof context; an unsealed artifact_layout rewrite is still
  > refused)
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > how do I replay a source through SourceIR EvidenceIR SemanticIR and IntentIR without overwriting generated artifacts
- [evidence-proof-binds-artifact-location](../knowledge/evidence-proof-binds-artifact-location.md)
  > how do I reproduce the artifact-relocation proof failure read-only (copy an evidence_ir.json, rewrite only its artifact_layout artifact_root and evidence_ir_path to the new directory, and run specforge entity-type on it: it fails. Run the same command on a byte-identical copy that keeps the original layout: it succeeds. Keeping the <base>/<document_key> convention does not help
  > — relocation as such is what fails)
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > how do I resolve a reviewed region by content
- [a-frozen-qualification-population-is-a-subset-floor-not-an-equality](../decisions/0050-a-frozen-qualification-population-is-a-subset-floor-not-an-equality.md)
  > how do I retain a new normalized bundle without qualifying it behaviorally
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > how do I reverify the frozen current claim census
- [mdbook-quantitative-census-freeze](../knowledge/mdbook-quantitative-census-freeze.md)
  > how do I reverify the frozen mdBook quantitative census
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > how do I reverify the published-assertion gate
- [a-bounded-snapshot-needs-a-declared-repeatable-rollover](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
  > how do I roll ROADMAP.md when it approaches its ceiling
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > how do I run a Docling ingest or re-ingest on this machine
- [a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest](../knowledge/a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest.md)
  > how do I run a cheaper doctrine gate before committing
- [toolbox-catalog-is-a-routed-landing](../knowledge/toolbox-catalog-is-a-routed-landing.md)
  > how do I search every SpecForge tool entry at once (rg -i 'term' TOOLBOX.md docs/toolbox)
- [declaration-replay-reads-the-legacy-stratum](../knowledge/declaration-replay-reads-the-legacy-stratum.md)
  > how do I see what today's SemanticIR declaration reader does with a legacy document
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how do I see which ingested PDFs form structural families (the corpus-cluster command)
- [persisted-census-measures-published-not-current](../knowledge/persisted-census-measures-published-not-current.md)
  > how do I size the population of an extractor change
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > how do I tell Clock source from c opcode
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > how do I tell a comma family from a footnote marker from a phrase without a word list
- [a-split-name-cell-does-not-say-which-join-it-wants](../knowledge/a-split-name-cell-does-not-say-which-join-it-wants.md)
  > how do I tell a concatenation join from an underscore join
- [a-frozen-pre-repair-contract-is-retired-not-regenerated](../decisions/0049-a-frozen-pre-repair-contract-is-retired-not-regenerated.md)
  > how do I tell a stale contract from a broken one
- [evidence-proof-binds-artifact-location](../knowledge/evidence-proof-binds-artifact-location.md)
  > how do I tell an artifact-relocation proof failure from proof-seal staleness (they are different: proof-seal staleness is a ruleset-hash mismatch that check_proof_seal_currency.sh reports and source_proof_migrate re-seals. Relocation passes the seal check, passes chain currency, and passes specforge semantic --dry-run; it fails only when the artifact is read from a different
  > location than the one recorded in its artifact_layout)
- [self-test-coverage-guard-is-in-the-exit-path](../knowledge/self-test-coverage-guard-is-in-the-exit-path.md)
  > how do I tell whether a doctrine check guards its own self-test coverage
- [qualified-role-header-proves-no-role](../knowledge/qualified-role-header-proves-no-role.md)
  > how do I tell whether a persisted SourceIR was produced before or after the classification narrowing
- [persisted-census-measures-published-not-current](../knowledge/persisted-census-measures-published-not-current.md)
  > how do I tell whether a persisted constraint record is still reproducible
- [producer-change-and-corpus-rebuild-are-one-transaction](../knowledge/producer-change-and-corpus-rebuild-are-one-transaction.md)
  > how do I tell whether a producer change invalidated the measured stratum (A/B the composition: remove the call edge and re-load the artifact — with it removed AXI loads 6,451 statements, with it restored the canonical loader refuses; PROOF-SEAL-CURRENCY in the gate tier catches it at commit either way)
- [a-sealed-region-cannot-move-out-of-an-active-part-alone](../knowledge/a-sealed-region-cannot-move-out-of-an-active-part-alone.md)
  > how do I tell which post-migration records have to move with a sealed region (enumerate both strata for every route on that part; any leaf with a declaration in each is superseded and must travel)
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > how do I verify every task tree is linked exactly once
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > how do I waive or range-scope the task-acceptance check
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how do Published-claims ids resolve
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > how do byte-granular page fragments chain (offset plus size adjacency)
- [register-record-access-and-table-provenance](../knowledge/register-record-access-and-table-provenance.md)
  > how do canonical register records retain source table provenance
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how do continuation tables (Table B2.2 Continued) merge into one container
- [decision-capacity-is-rederived-without-moving-stable-records](../decisions/0041-decision-capacity-is-rederived-without-moving-stable-records.md)
  > how do decision slots change Knowledge Map capacity
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how do dword-relative page fragments chain
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > how do packet/flit protocols (CHI-class) declare message fields vs signals
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > how do task trees memory the Knowledge Map doctrine enforcement and claim verification fit together
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how do you audit registers/signals against the table image with the VLM
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how does .10g differ from .10f (register vs message routing)
- [corpus-pattern-reuse](../knowledge/corpus-pattern-reuse.md)
  > how does / will SpecForge reuse extraction patterns across different PDFs
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > how does CORPUS-COVERAGE 2 33d ii prevent weak signal names from reentering through relations
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > how does FSMGen decide a value literal's width (by notation digit count — 0x7D=8 bits, 0b00=2 bits — NOT by value; it requires an exact width-cast W'… match, no implicit truncation/extension; a bare decimal is unsized and fits any width)
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > how does FSMGen decide two rule data-writes conflict (same target, different value, NOT compatible/disjoint/priority/resource resolved) and when is a guard proven disjoint (_condition_terms_prove_disjoint: shared eq: signal with different values; an absent/empty condition is NEVER proven disjoint)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > how does SemanticIR preserve VALID READY without formal signal declarations
- [source-proof-migration-replays-classification-context](../knowledge/source-proof-migration-replays-classification-context.md)
  > how does SourceIR proof migration handle a classifier implementation change
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how does SpecForge avoid chip/vendor name lists in document classification (structural typed-surface counts + generic front-matter doc-type vocabulary only; ADR 0006)
- [protocol-state-machine-binding](../knowledge/protocol-state-machine-binding.md)
  > how does SpecForge bind a protocol state to its state machine
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how does SpecForge capture protocol actors/agents defined in prose (.3b/.8 extract_protocol_actors)
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how does SpecForge capture signals that are in prose not tables (I2C SDA/SCL)
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > how does SpecForge cluster chip-spec PDFs by vendor/layout without hardcoding vendor names
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > how does SpecForge combine confidence across modalities or sources
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > how does SpecForge decide signal direction (input/output) in the emitted .isf
- [a-level-belongs-to-a-signal](../knowledge/a-level-belongs-to-a-signal.md)
  > how does SpecForge decide which signal a logic level belongs to
- [contested-priors](../knowledge/contested-priors.md)
  > how does SpecForge detect contradicting or conflicting priors
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > how does SpecForge detect two surfaces disagreeing about one quantity
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how does SpecForge determine what a chip-spec PDF is about / its purpose category
- [timing-table-structural-authority](../knowledge/timing-table-structural-authority.md)
  > how does SpecForge distinguish a timing table category from a scalar min typ max layout
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > how does SpecForge distinguish legal conditions from protocol conditions
- [administrative-workflow-is-not-semantic-authority](../knowledge/administrative-workflow-is-not-semantic-authority.md)
  > how does SpecForge distinguish protocol requests from product listing requests
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > how does SpecForge emit temporal rules or a bounded-eventually into .isf
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how does SpecForge extract register fields from tables
- [ambiguity-weak-phrase-detector](../knowledge/ambiguity-weak-phrase-detector.md)
  > how does SpecForge flag vague or ambiguous spec language
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > how does SpecForge group a transaction's signals by channel
- [timing-scalar-rows-require-independent-cell-geometry](../knowledge/timing-scalar-rows-require-independent-cell-geometry.md)
  > how does SpecForge handle Docling clones of a table cell with col_span greater than one
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > how does SpecForge handle one transaction and one rule writing the same named-drive target
- [source-to-intent-vertical-evaluator](../knowledge/source-to-intent-vertical-evaluator.md)
  > how does SpecForge measure source-to-IntentIR stage loss
- [decibel-domain-timing-intent-disposition](../knowledge/decibel-domain-timing-intent-disposition.md)
  > how does SpecForge prevent analog dB limits from becoming digital timing intent
- [claim-control-audit-closure](../knowledge/claim-control-audit-closure.md)
  > how does SpecForge prove a cited self-test contains a known-bad RED case
