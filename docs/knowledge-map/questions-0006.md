# Knowledge questions — shard 0006

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [bounded-decision-frozen-baseline](../knowledge/bounded-decision-frozen-baseline.md)
  > how do I re-derive the bounded decision baseline
- [bracketed-metavariable-name-cell](../knowledge/bracketed-metavariable-name-cell.md)
  > how do I re-derive the declaration-row notation populations
- [direction-column-drift](../knowledge/direction-column-drift.md)
  > how do I re-derive the direction column drift census
- [measured-stratum-promotion-population](../knowledge/measured-stratum-promotion-population.md)
  > how do I re-derive the measured-stratum promotion population (cargo test -p specforge --lib measured_stratum_promotion_population -- --ignored --nocapture; the crate is specforge, not specforge-core, because commands/** does not #[path] into core)
- [refresh-completion-is-not-artifact-currency](../knowledge/refresh-completion-is-not-artifact-currency.md)
  > how do I re-derive the refresh-versus-currency partition (read doctrine/corpus_frontier/census.json for the refreshed and remaining sets, then join it against each generated/source_ir/<doc>/source_ir.json proof ledger and each generated/evidence_ir/<doc>/evidence_ir.json schema_version)
- [registry-capacity-bounds-are-incoherent](../knowledge/registry-capacity-bounds-are-incoherent.md)
  > how do I re-derive the registry capacity census
- [claim-registries-pin-two-digest-families](../knowledge/claim-registries-pin-two-digest-families.md)
  > how do I re-pin a region whose published value moved
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > how do I re-pin claim regions after editing a governed file (python3 scripts/repin_claim_regions.py --check then --apply; it resolves by content across all three registries and REFUSES ambiguity rather than taking the first match, which matters because a blank-line region matches every blank line in the file)
- [flow-census-is-derived-against-a-declared-target](../knowledge/flow-census-is-derived-against-a-declared-target.md)
  > how do I re-pin decision_sites or helper_edges
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
- [legacy-artifact-declaration-drift](../knowledge/legacy-artifact-declaration-drift.md)
  > how do I replay the table declaration pass on a legacy document
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
- [book-chapter-growth-is-one-accreting-section](../knowledge/book-chapter-growth-is-one-accreting-section.md)
  > how do I stop a live document becoming an append-only log
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
- [enum-member-values-have-no-oracle](../knowledge/enum-member-values-have-no-oracle.md)
  > how do I verify a change to enum member values
- [quoted-literal-notation-carries-the-radix](../knowledge/quoted-literal-notation-carries-the-radix.md)
  > how do I verify a value-correctness repair
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
- [containment-numbers-are-type-checked-at-decode](../knowledge/containment-numbers-are-type-checked-at-decode.md)
  > how does JSON::PP mark a decoded number versus a string
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
- [decimal-before-binary-needs-a-notational-witness](../knowledge/decimal-before-binary-needs-a-notational-witness.md)
  > how does SpecForge decide the radix of an unquoted encoding cell
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
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > how does SpecForge prove a persisted corpus artifact is still what the current binary produces
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > how does SpecForge prove a reviewed paraphrase is parser equivalent
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does SpecForge read a PDF's front-matter / title / ToC to know its doc type
- [one-record-per-obligation-clause](../knowledge/one-record-per-obligation-clause.md)
  > how does SpecForge read a fronted condition like When asserted, X must remain asserted
- [quoted-literal-notation-carries-the-radix](../knowledge/quoted-literal-notation-carries-the-radix.md)
  > how does SpecForge read a quoted encoding value
- [per-row-column-drift-rule](../knowledge/per-row-column-drift-rule.md)
  > how does SpecForge read a table whose rows disagree about their column order
- [temporal-condition-read-whole](../knowledge/temporal-condition-read-whole.md)
  > how does SpecForge read a temporal condition with an unless or except clause
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > how does SpecForge recover a subject the model misspelled
- [timing-caption-unit-and-table-provenance](../knowledge/timing-caption-unit-and-table-provenance.md)
  > how does SpecForge recover a timing unit from a table caption
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > how does SpecForge relate to GoldMine Texada Pnueli Ammons
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > how does SpecForge resolve PSEL to PSELx in a temporal condition
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > how does SpecForge resolve a persisted path after the repository moves
- [questions-bugs-and-features-to-upstream-travel-as-one-kind-of-packet](../decisions/0054-questions-bugs-and-features-to-upstream-travel-as-one-kind-of-packet.md)
  > how does SpecForge send a question to FSMGen
- [persisted-path-origin-and-rebase-contract](../knowledge/persisted-path-origin-and-rebase-contract.md)
  > how does SpecForge serialize repository owned paths
- [published-assertion-gate](../knowledge/published-assertion-gate.md)
  > how does SpecForge stop a published count going stale under a green gate
- [required-residual-actionability-denominator](../knowledge/required-residual-actionability-denominator.md)
  > how does SpecForge stop a published gap reproduction from silently running no test
- [base-name-template-table-is-not-a-catalogue](../knowledge/base-name-template-table-is-not-a-catalogue.md)
  > how does SpecForge tell a generic channel-signal template from a signal catalogue
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how does SpecForge tell a guide from a real spec / report low-yield docs honestly
- [llm-path-lacks-the-local-appositive-declaration](../knowledge/llm-path-lacks-the-local-appositive-declaration.md)
  > how does SpecForge tell a parameterised declaration template from a truncation (it does not, and deliberately: ADR 0037 forbids reading the suffix at all, so PSELx and ATBYTES are both simply declared names that a shorter proposal does not match)
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > how does SpecForge tell a parametric width from a description sentence
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > how does SpecForge tell a property table from a signal table
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > how does SpecForge verify an extracted claim semantically / catch hallucination
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > how does a VLM timing note become a FigureRegion
- [a-table-premise-is-recaptured-from-retained-converter-output-not-re-ingested](../decisions/0052-a-table-premise-is-recaptured-from-retained-converter-output-not-re-ingested.md)
  > how does a change to how ingest reads tables reach a persisted SourceIR
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > how does a deliberate normalized-bundle reclamation get authorized
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > how does a future SpecForge session retrieve and re-run a current claim
- [no-collection-may-declare-an-aggregate-below-its-own-legal-maximum](../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
  > how does a heterogeneous collection declare its legal maximum
- [a-level-belongs-to-a-signal](../knowledge/a-level-belongs-to-a-signal.md)
  > how does a list of signals share one logic level
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > how does a repair prove its change is isolated if it also rebuilds drifted documents
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how does a section-heading register avoid double-counting an existing register record
- [caption-label-is-not-table-identity](../knowledge/caption-label-is-not-table-identity.md)
  > how does a spec mark a table continued across a page break
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > how does a timing diagram become an ActorContract in SemanticIR and IntentIR
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > how does a transaction absorb signals from its subsections (3.1.1 / 3.1.2)
- [table-row-obligation-binds-to-the-token-before-its-modal](../knowledge/table-row-obligation-binds-to-the-token-before-its-modal.md)
  > how does a width parameter's obligation become a signal constraint
- [header-trapped-signal-table-recovery](../knowledge/header-trapped-signal-table-recovery.md)
  > how does an unknown-kind Continued from previous page table fragment get a kind
- [trajectory-controller-engine](../knowledge/trajectory-controller-engine.md)
  > how does automatic task ranking keep hard failures ahead of breadth work
- [behavioral-identity-alpha-harness](../knowledge/behavioral-identity-alpha-harness.md)
  > how does behavioral genericity normalize renamed stable ids safely
- [transaction-phase-qualifier-requires-positive-authority](../knowledge/transaction-phase-qualifier-requires-positive-authority.md)
  > how does build_transaction_phases distinguish a named phase from phase error or phase tolerance
- [chain-currency-doctrine](../knowledge/chain-currency-doctrine.md)
  > how does chain currency distinguish a checked blocked adapter from an emitted ISF file
- [claim-provenance-is-a-bounded-executable-evidence-join](../decisions/0044-claim-provenance-is-a-bounded-executable-evidence-join.md)
  > how does claim evidence become stale
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how does collapse_section_header_register_identity decide same-register vs different-register
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > how does converge report integrated scheduled and omitted production capabilities
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > how does converge report per-document extraction quality after stabilization
- [register-field-table-defragmentation](../knowledge/register-field-table-defragmentation.md)
  > how does de-fragmentation enable the recover-register-bits gate (b)
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > how does derive_isf_actor_name produce a valid HDL identifier ([A-Za-z_]\\w*)
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > how does derive_register_block_name parse a block out of a register-descriptions section heading
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > how does each active task-evidence index invoke its own contract
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > how does extract_discovered_state_value_from_text match a constraint value
- [llm-primary-must-be-value-recall](../knowledge/llm-primary-must-be-value-recall.md)
  > how does ground_constraint recover a value the model did not echo
