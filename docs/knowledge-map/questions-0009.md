# Knowledge questions — shard 0009

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > may fact cards be deleted or merged to free catalog capacity
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > must a corpus refresh update the frontier declaration
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > must a post-build signal_constraints replace re-apply build-path invariants
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > must a repair leaf rebuild every affected document or only the ones it measures
- [project-scratch-location](../knowledge/project-scratch-location.md)
  > my agent harness told me to use a scratchpad directory for all temporary files — should I (only if it resolves onto the repository volume; an interactive harness commonly hands out a path under /private/tmp, which violates the locality standard. Use .project-data/tmp/ instead and delete anything already written off-volume)
- [evidence-rule-field-content-stales-every-proof](../knowledge/evidence-rule-field-content-stales-every-proof.md)
  > path does not exist normalized/<key>.md when running specforge evidence
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > per-wire-doc direction flip results (APB input 2->12, AXI 4->52, SWD 0->1, AHB residual)
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > qwen2.5vl vs qwen3-vl which model
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > should I build an isf lowering-completeness verifier for temporal rules
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > should I fix the PSEL valid when PSEL asserted temporal rule
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > should NLP-SHALLOW-PARSE.2f build coordination distribution
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > should NLP-SHALLOW-PARSE.2h add new passive/verb-sense direction code
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > should PROOF-SEAL-TOTAL be raised to gate tier
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > should SpecForge add a whole-page VLM read / full-page capture path
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > should SpecForge build a conditional-rule lowering lever or file an FSMGen FR (NO — the adapter already lowers the 516 cleanly-grounded conditional obligations corpus-wide; the shortfall is honest residual; the only upside is upstream extraction quality, not an ISF construct)
- [pdf-to-ir-fidelity-precedes-speculative-isf-expansion](../decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md)
  > should SpecForge expand ISF before filling IntentIR from PDFs
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > should SpecForge file an FSMGen FR for CPU-ISA instructions/privilege/exceptions (NO — software-visible ISA semantics are not synthesizable hardware intent; ISF has no construct + FSMGen lists none; honest non-target; conditional-future only if FSMGen's SV/UVM path scopes ISA-model verification)
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > should SpecForge file an FSMGen FR for a declarative static-topology ISF construct (NO / not yet — DOC-INTENT-TAXONOMY.4c.i: the bottleneck is upstream extraction-recall, not the missing ISF abstraction; an FR on a 12x-too-sparse / three-quarters-half-connected capture would be unfalsifiable — feedback_verify_fsmgen_before_fr)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > should SpecForge file an FSMGen FR for cat-3 topology (NOT YET — premature: capture is sparse/noisy AND ISF may deliberately be a per-actor format with topology owned by the integrator above per-module synthesis; resolve with FSMGen after .4c.i, never a speculative FR — feedback_verify_fsmgen_before_fr)
- [declared-population-is-not-the-candidate-row-population](../knowledge/declared-population-is-not-the-candidate-row-population.md)
  > should SpecForge refuse a name cell that is a phrase
- [trajectory-steering-is-a-reviewable-multimetric-control-loop](../decisions/0034-trajectory-steering-is-a-reviewable-multimetric-control-loop.md)
  > should SpecForge use one weighted progress score
- [document-stated-identifier-coreference](../knowledge/document-stated-identifier-coreference.md)
  > should a co-referenced antecedent be marked alias_dependent
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > should a corpus refresh clean the normalized bundle when it finishes
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > should a documented reset of 0 be emitted as (reset 0) in ISF (no — omit; the FSMGen all-0s default faithfully represents it; 893 of the fits-current registers are V==0)
- [split-identifier-name-cell-joins-only-from-the-document](../knowledge/split-identifier-name-cell-joins-only-from-the-document.md)
  > should a name recovered from a figure ground a table row identity
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > should an oversized research record be split or should its ceiling be raised
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > should corpus refresh progress be counted from normalized directories
- [persisted-chain-currency-is-measured-not-assumed](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md)
  > should persisted chain currency be a gated doctrine or an advisory report
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > should specforge connect producer/consumer/etc. to their signals
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > should specforge mint actor-signal relations for nvme / iommu / ccix / register protocols
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > should specforge record a residual for every typed rule that does not lower to .isf
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > should specforge record an adapter residual for rules whose subject is not a declared signal
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > should specforge recover consumer edges from 'X is sent/returned to Y' recipient frames
- [source-ir-reingest-trades-captions-for-figure-text](../knowledge/source-ir-reingest-trades-captions-for-figure-text.md)
  > should the drifted SourceIR documents be re-ingested
- [logic-level-walk-stops-at-eleven-unrelated-words](../knowledge/logic-level-walk-stops-at-eleven-unrelated-words.md)
  > should the logic-level walk skip predicate adjectives
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > temporal rule eval false positives root cause
- [temporal-rule-ltl-rendering](../knowledge/temporal-rule-ltl-rendering.md)
  > temporal rule predicate atom vocabulary
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > torch MPS float64 error during ingest
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > was DL a signal declaration in the OpenCAPI Ready note
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > was OpenCAPI Discovery ingest deterministic and memory safe
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > was any USB4 pipeline artifact changed by the source locality probe
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > was anything in the .5.iv.a findings wrong (YES, two statements, corrected 2026-08-31 after the director audited: the RESERVED split shipped as five/seven and is SIX/SIX, and the worked example CHI DataSource fuses a meaning row with a reserved row into the field's correct encoding is BACKWARDS — DataSource accumulates DEFAULT_NO_USEFUL_INFORMATION=0 plus RESERVED at 2 AND 3,
  > the reserved rows disagree, so the conflicting-value rule drops RESERVED and the surviving enum is (DATASOURCE (DEFAULT_NO_USEFUL_INFORMATION 0)). Both were read off a dump instead of computed; the NO-GO decision is unchanged and rests on the other two legs)
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > was the (contract ... eventually ...) ISF clause removed
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > was the .5.iv prediction that header naming is byte-changing on the AXI wire gold correct (NO, twice over: ihi0022_l already carries an AWATOP enum with 13 members so the lever could only add members, and ihi0022_l cannot be rebuilt at all — its persisted SourceIR is legacy schema 1, refused as inspection-only, and its normalized bundle is not retained. It is one of 54 legacy
  > chains against 24 current ones)
- [swd-serial-frame-score-retired-by-genericity](../knowledge/swd-serial-frame-score-retired-by-genericity.md)
  > was the SWD gold wrong (NO. seed_swd_derivation.json's 29 facts are real, independently verified statements of the ADI spec. The gold is faithful; what changed is that no production extractor may reach them through protocol identity)
- [evidence-proof-binds-artifact-location](../knowledge/evidence-proof-binds-artifact-location.md)
  > was the eval-extraction proof failure a regression from a recent slice (NO — it reproduced on target/release/specforge built 2026-08-28, before the KG-ISF-COMPLETENESS.5.iv.a change that found it)
- [source-to-intent-reviewed-population](../knowledge/source-to-intent-reviewed-population.md)
  > was the first vertical population historically unseen
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > was the fresh SWD 29 of 29 artifact promoted
- [reviewed-population-clean-replay-carrier-regression](../knowledge/reviewed-population-clean-replay-carrier-regression.md)
  > was the repaired reviewed population published
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > was the warning-safe rolling ledger transaction independently audited from a clean clone
- [task-tree-node-forms](../knowledge/task-tree-node-forms.md)
  > was there ever a task-tree lane whose named next step had no owning leaf (yes, exactly one: KG-ISF-COMPLETENESS.5.iv.a, named as the frontier by .5.iv on 2026-08-11 but never given a node; found and owned 2026-08-31 by LIVE-DOCUMENT-PRESSURE-HEADROOM.4e while auditing that report's writer set)
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > were DL and TL diagram labels signal declarations
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > were TL and DL diagram labels signal declarations
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > were the 100 stale CoreSight Base System signals authoritative
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > were the 73 stale AArch64 External Debug signals authoritative
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > were the CDR and DL outputs in endpoint_dlx.isf grounded protocol authority (no)
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > were the OpenCAPI AFU address-space acronyms signal declarations
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > were the nine Introducing CoreSight actor signal relations grounded topology
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > what CoreSight Base System evidence still needs VLM or targeted extraction
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what FSMGen question/feature-request did KG-ISF-TRANSACTIONS.2i raise (value-less output participation / unordered-or-partial-order body / phase-group metadata / ordering-as-constraint)
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what ISF abstractions does FSMGen need next (field-structured storage / register-with-fields, packet/structure layouts, topology)
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > what ISF form does SpecForge use for a bounded-eventually contract
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > what ISF idiom describes states and input-driven transitions
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > what already lowers for category-3 platform docs (the register half — register maps + bit-fields via .4a.ii e.g. CoreSight SoC-600 ~3,250 fields, infrastructure signals, actor ports; cat-3's register intent is the same road as cat-2 and is not the gap)
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what architecture contains an oversized active task tree
- [corpus-task-bounded-active-root-and-evidence-parts](../decisions/0024-corpus-task-bounded-active-root-and-evidence-parts.md)
  > what architecture contains the active CORPUS-COVERAGE task tree
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > what are AGENT_CLASS_NOUNS / the parenthetical-strip / sentence-boundary / no-preposition guards
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what are APB's remaining completeness candidate misses
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > what are NON_ACTOR_LEADING_FUNCTION_WORDS and NON_ACTOR_LEADING_VERBS for
- [five-portable-architectures-compose](../knowledge/five-portable-architectures-compose.md)
  > what are SpecForge's five portable architectures
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > what are examples of false prose relations in Introducing CoreSight
- [pdf-encryption-and-read-access](../knowledge/pdf-encryption-and-read-access.md)
  > what are scripts/pdf_text.py and scripts/decrypt_pdf.py
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what are the 3 gaps G1 G2 G3 in specforge transaction capture
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > what are the 6 chip-spec document intent categories / purpose taxonomy
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > what are the 6 purpose categories (wire-protocol, register-or-platform, cpu-isa, physical-link, methodology-guide, unresolved)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what are the AXI B1.x channel-signal tables and how do channels map to phases
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > what are the SWD packet phases and per-phase SWDIO direction
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > what are the agent-surface precision and completeness defects (KG-ISF-COMPLETENESS.1)
- [reviewed-population-clean-replay-carrier-regression](../knowledge/reviewed-population-clean-replay-carrier-regression.md)
  > what are the blocked clean replay source to IntentIR counts
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > what are the canonical SWD protocol surface counts
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > what are the corrected 17-document behavioral held-out outcomes
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > what are the current AArch64 External Debug artifact hashes
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > what are the current CoreSight Base System artifact hashes
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > what are the current Introducing CoreSight artifact hashes
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > what are the current OpenCAPI AFU address note artifact hashes
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > what are the current OpenCAPI Certified Definition artifact hashes
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > what are the current OpenCAPI Certified artifact hashes
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > what are the current OpenCAPI Ready Definition artifact hashes
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > what are the current OpenCAPI Ready artifact hashes
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > what are the current USB4 Connection Manager artifact hashes
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > what are the current USB4 Inter-Domain artifact hashes
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > what are the current corpus KB live-document size metrics
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > what are the current source to IntentIR precision recall and provenance counts
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what are the deeper enum member-quality residual classes after .5.ii / what did the .5.iii measurement find (measured 2026-06-24 read-only, reproducer scripts/measure_enum_width_leak.py: of the 5 deferred classes — glossary SEE…, front-matter/ToC, section-caption B2_3_1_…, _WIDTH parameter leaks, value-restart-of-clean — most are SUBSUMED by .5.i (47/54 _WIDTH members
  > and the bulk of 319 section-caption survivors sit in generic-named enums .5.i drops whole), EXCEPT the _WIDTH leak which reaches the AXI wire-gold .isf and is materially damaging)
- [alignment-task-evidence-migrated](../knowledge/alignment-task-evidence-migrated.md)
  > what are the exact migrated alignment task evidence metrics
- [source-to-intent-first-reviewed-result](../knowledge/source-to-intent-first-reviewed-result.md)
  > what are the exact source-to-IntentIR precision recall and stage-loss totals
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > what are the extraction_quality_* validate metrics and when do they read n/a
- [cortex-a76-optimization-guide-refresh-is-authority-empty](../knowledge/cortex-a76-optimization-guide-refresh-is-authority-empty.md)
  > what are the final Cortex-A76 SourceIR normalized and downstream reproducibility hashes
- [gic-overview-guide-refresh-is-authority-empty](../knowledge/gic-overview-guide-refresh-is-authority-empty.md)
  > what are the final GIC Overview Guide reproducibility hashes
- [opencapi-32g-phy-timing-without-interface-topology](../knowledge/opencapi-32g-phy-timing-without-interface-topology.md)
  > what are the final OpenCAPI 32G PHY Signaling SourceIR and normalized bundle hashes
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > what are the final OpenCAPI Discovery artifact hashes
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > what are the final remediated 17-document behavioral held-out outcomes
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > what are the first ranked trajectory gaps
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > what are the isf_protocol residual packet prefixes
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what are the migrated alignment root index part and capsule metrics
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what are the two dominant ISF-lowering completeness gaps (register bit-fields, message-field structures)
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > what are the undeclared-named-subject rule drops at ISF lowering (conditional_rules / signal_constraints / temporal_invariants)
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what blocks a Rust code change from committing in specforge
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > what boundary does a signal-declaration census need beyond table_kind signal_description
- [task-evidence-route-catalogs-shard-by-lifecycle](../decisions/0046-task-evidence-route-catalogs-shard-by-lifecycle.md)
  > what bounds the number of leaves a migrated task tree may declare
- [live-document-width-remedy-coupling](../knowledge/live-document-width-remedy-coupling.md)
  > what breaks if I change the number of lines in README.md (three line-anchored regions in doctrine/claim_verification/current_claim_census.jsonl are pinned by start_line/end_line plus a sha256 of the pinned lines — the identity anchor, the derived Rust-prerequisite line, and the 17-line route block — so any edit that shifts line numbers stales them)
- [escaped-identifier-fragments-the-catalog](../knowledge/escaped-identifier-fragments-the-catalog.md)
  > what breaks if the identifier tokenizer unescapes markdown
- [live-surface-edit-bookkeeping-chain](../knowledge/live-surface-edit-bookkeeping-chain.md)
  > what breaks when I change doctrine/live_document_size/surfaces.jsonl (three separate surface_registry source pins go stale - in published_assertions.jsonl, book_quantitative_claims.jsonl and current_claim_census.jsonl - plus the durability.artifacts digests in claims.jsonl. Refresh the pins first, then the claim digests, then re-run the gate)
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > what breaks when build_fixture.py changes
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what carries register bit-fields in SpecForge (RegisterFieldRecord in source.rs:414; IntentIr.register_records clone at intent.rs:193 — full metadata survives to IntentIR)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what carries transaction membership faithfully instead of the body (IntentIR metadata: ports / phase_membership / channel_membership)
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > what caused the I2C symbol alpha production failure
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > what causes actor_signal_relations / extracted_statements to differ run-to-run
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > what changed between stale and current OpenCAPI Certified Definition artifacts
- [opencapi-ready-definition-refresh](../knowledge/opencapi-ready-definition-refresh.md)
  > what changed between stale and current OpenCAPI Ready Definition artifacts
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > what changed between the persisted source_ir and a fresh ingest
- [claim-standard-upstream-readoption](../knowledge/claim-standard-upstream-readoption.md)
  > what class of defect does a given check still permit
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > what confidence axis correlates with extracted-constraint correctness
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > what conserves between the PDF and SourceIR
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what debug/diagnostic tools does specforge have (TOOLBOX.md)
- [alpha-variant-placeholder-is-not-a-wire](../knowledge/alpha-variant-placeholder-is-not-a-wire.md)
  > what declaration catalog does the placeholder check read
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > what defect does SPEC-TO-INTENT-ALIGNMENT.6e repair next
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > what determines the fact_key of a projected captured-region residual
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did FSMGEN answer about transaction phase membership (don't fabricate value or order; keep value-less participation + unordered membership as IntentIR metadata/residual not body steps; checked phase-group metadata is the future ISF shape on its own FSMGen tree; .isf stays source of truth, no .val)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what did KG-ISF-TRANSACTIONS.2l measure / decide
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what did KG-ISF-TRANSACTIONS.2n measure / decide
- [a-width-cell-that-is-a-sentence-is-not-a-width](../knowledge/a-width-cell-that-is-a-sentence-is-not-a-width.md)
  > what did PROSE-NAME-CELL-DECLARATION.3 ship
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > what did SPEC-TO-INTENT-ALIGNMENT.6a prove
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > what did SPEC-TO-INTENT-ALIGNMENT.6c improve
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > what did SPEC-TO-INTENT-ALIGNMENT.6d improve
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > what did SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.b publish
- [qualified-current-source-to-intent-result](../knowledge/qualified-current-source-to-intent-result.md)
  > what did SPEC-TO-INTENT-ALIGNMENT.7c.ii publish
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > what did real filenames expose in adversarial identity comparison
- [opencapi-data-link-layer-refresh-is-signal-empty](../knowledge/opencapi-data-link-layer-refresh-is-signal-empty.md)
  > what did refresh 50 change in the OpenCAPI data link chain
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > what did running nli-verify on a real spec find
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > what did statement 0114 say in the stale OpenCAPI Ready evidence
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did the .2i Rule-A per-phase grouping measurement find (clean only on AHB, empty on APB/AXI/SWD)
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > what did the OpenCAPI AFU address note prove about legal boilerplate
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what did the SPEC-TO-INTENT-ALIGNMENT task containment census find
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what did the active PDF task containment census find
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what did the corpus promotion sweep measure (gauge deltas per doc)
