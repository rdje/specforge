# Knowledge questions — shard 0004

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > is the .isf affected by transaction membership (no — emitter lowers steps, not ports/phase_membership)
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > is the APB PSTRB must be LOW constraint extracted
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > is the APB signal catalog extracted
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > is the ATP ihi0082 ARVALID/RVALID/RREADY drop a lowering gap
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > is the DOC-INTENT-TAXONOMY .2 per-category scorecard measurement complete (YES after .4e — Gap A lowered .4a.ii, Gap B carrier .4b gated, cat-3 .4c->.4c.i, cat-4 .4d->.4d.i, conditional rules .4e honest residual; remaining work is CODE not measurement)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > is the DTI ATST 0B01 constraint a real obligation (no — mis-attribution: the source text 'ATTR_OVR.SHCFG must be 0b01' binds SHCFG's value to ATST, which is a value of the FLOW field; an upstream extraction bug spun out of ISF-VALUE-WIDTH-EMIT)
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > is the EvidenceIR build reproducible / deterministic
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > is the ISF storage-var width the register width (no — ir/isf_ir.rs uses max single-field extent, a latent bug; the true width is size_bits or max(bits_high)+1)
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > is the LLM-primary promotion a recall improvement
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > is the LLM-primary promotion gauge improvement reproducible on canonical artifacts (not just /tmp copies)
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > is the LLM/VLM provider missing or not wired up
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is the NLP-SHALLOW-PARSE build frontier exhausted
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > is the PDF-VARIANT-DIGESTION current frontier internally consistent
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > is the SWD FSM the same as the JTAG TAP DBGTAPSM (no)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is the SemanticIR phases surface the protocol transaction phases (no — it is section/chapter-derived)
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > is the SpecForge extraction robust and can it grow to a vast set of chip-spec PDFs
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > is the VLM the right lever for AXI transaction phase membership (no — a deterministic channel-table cue is)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > is the VLM-tier transaction frontier exhausted (yes — superseded by the deterministic channel cue / honest absence)
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > is the WIRE-BASED-100.5a AHB 0.364 baseline real
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > is the canonical SWD normalized bundle path portable
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > is the canonical intent_ir stale relative to its evidence_ir
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > is the cat-3 platform/system-IP topology capture faithful enough to lower to ISF (NO — measured 0.355 signal_connectivity edges/actor over 380 actors / 15 docs, only 24% of edges have both a producer AND a consumer, 0/10 infrastructure_signals have a resolved source; lowering it would synthesize an unfaithful sliver)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > is the cat-3 topology capture good enough to lower (NO — sparse + noisy: CoreSight SoC-600 has 6 signal_connectivity edges across 60 actors, GIC-600 66 edges / 2 infra; None/escaped actor names; a capture-recall measurement .4c.i must precede any lowering)
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > is the cat-3 topology problem name-noise or sparsity (primarily SPARSITY + half-connectedness + rootless clock/reset, NOT noise — endpoints are 95% clean / only 12 escaped edges corpus-wide; refines the .4c 'sparse and noisy' to 'sparse + half-connected + rootless-infra with minor name noise')
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > is the cat-4 register-field gap an ISF-abstraction gap or an extraction gap (EXTRACTION RECALL — fields are unlocated / AIA registers uncaptured; ISF already expresses register fields via .4a.ii; spun out as .4d.i RISC-V CSR bit-position recovery)
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > is the chipdoc source library on the same SSD volume as specforge
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > is the clock/reset distribution tree captured for platform docs (only partially — infrastructure_signals is a near-fixed 2-per-doc surface; 6/10 carry a fan-out distributed_to_actor_ids list but 0/10 carry a resolved recovered_source_actor_ids root, so the tree has no captured origin)
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > is the conditional_rules ISF-lowering shortfall a real gap or honest residual (HONEST RESIDUAL — 73% prose/undeclared/placeholder; the rest are bare deontic modals with no concrete obligation; no buildable ISF lever, no FR)
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > is the dense-prose phantom-actor explosion a relation-subject seam or an actors[] prose-mint seam problem
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > is the descendant-subsection scope over-broad (no — 0 over-broad corpus-wide)
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > is the drive/read direction correct for passive vs active prose relations
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > is the enum drop a width-overflow rule (no — count-derived width 2a.iv hypothesis was DISPROVEN; GIC-600 emits 69152 strict-clean; FSMGen accepts bare decimals of any magnitude)
- [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md)
  > is the eval gold / answer key trustworthy or reliable
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > is the eval-extraction temporal precision 0.6 a real defect
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > is the extraction audit chip-spec-PDF agnostic (yes)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is the generic-TABLE enum conflation fixed / what did KG-ISF-COMPLETENESS.5.i do (LANDED 2026-06-24: derive_encoding_enum_name fallback keeps the candidate only when independently evidenced — a declared signal OR a column-header reference token of the table — else None; emitter isf_ir.rs gates the (types) block by emitted_enums() so a member-dropped enum leaves no orphan
  > (type ...). Corpus generic enums 82->8 / total enum records 422->105 across 33 rebuildable docs; real signal-match enums byte-identical; WIRE-BASED-100 1.000 before==after; fsmgen --strict 0 diagnostics)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is the generic-enum conflation an emitter bug or an extraction bug (EXTRACTION-born in evidence.rs + semantic.rs; isf_ir.rs:889-912 lowers it faithfully)
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > is the kg-isf-completeness.3 relation-completeness frontier closed / are any docs still stale
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is the orphan (type TABLE) line a separate emitter bug (yes — isf_ir.rs:403-409 emits all self.types unconditionally, so a Lever-F-residualized enum still leaves an orphan (type ...) line; gate by emitted_enums())
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > is the register-bit-field ISF gap (Gap A) a SpecForge bug or a missing ISF abstraction (a missing ISF abstraction — fields reach IntentIR fully; ISF has no field-structured storage)
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > is the relation-incompleteness on the 0-relation docs a recoverable gap or genuine absence
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > is the transaction body faithfully complete (yes — only the grounded enum-selector drive is body-lowerable)
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > is the whole renderable corpus FSMGen --strict clean now (yes — 70/70 current-emit .isf after .2a.vi; the ISF-emit strict-FAIL frontier is closed: Levers A/B/C/F + .2a.vi)
- [temporal-rule-ltl-rendering](../knowledge/temporal-rule-ltl-rendering.md)
  > is there a PSL or SVA export of temporal rules
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > is there a buildable transaction ordered multi-phase body lever beyond .2b
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > is there a first opt-in extractor for the CORPUS-PATTERN-REUSE activate-only consume side
- [isf-temporal-lowering-no-silent-drop](../knowledge/isf-temporal-lowering-no-silent-drop.md)
  > is there a lowering-completeness check for temporal rules
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > is tier-agreement a good confidence axis for conformal calibration
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > must a post-build signal_constraints replace re-apply build-path invariants
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
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > should SpecForge add a whole-page VLM read / full-page capture path
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > should SpecForge build a conditional-rule lowering lever or file an FSMGen FR (NO — the adapter already lowers the 516 cleanly-grounded conditional obligations corpus-wide; the shortfall is honest residual; the only upside is upstream extraction quality, not an ISF construct)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > should SpecForge file an FSMGen FR for CPU-ISA instructions/privilege/exceptions (NO — software-visible ISA semantics are not synthesizable hardware intent; ISF has no construct + FSMGen lists none; honest non-target; conditional-future only if FSMGen's SV/UVM path scopes ISA-model verification)
- [cat3-topology-capture-recall](../knowledge/cat3-topology-capture-recall.md)
  > should SpecForge file an FSMGen FR for a declarative static-topology ISF construct (NO / not yet — DOC-INTENT-TAXONOMY.4c.i: the bottleneck is upstream extraction-recall, not the missing ISF abstraction; an FR on a 12x-too-sparse / three-quarters-half-connected capture would be unfalsifiable — feedback_verify_fsmgen_before_fr)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > should SpecForge file an FSMGen FR for cat-3 topology (NOT YET — premature: capture is sparse/noisy AND ISF may deliberately be a per-actor format with topology owned by the integrator above per-module synthesis; resolve with FSMGen after .4c.i, never a speculative FR — feedback_verify_fsmgen_before_fr)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > should a documented reset of 0 be emitted as (reset 0) in ISF (no — omit; the FSMGen all-0s default faithfully represents it; 893 of the fits-current registers are V==0)
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > should corpus refresh progress be counted from normalized directories
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
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > temporal rule eval false positives root cause
- [temporal-rule-ltl-rendering](../knowledge/temporal-rule-ltl-rendering.md)
  > temporal rule predicate atom vocabulary
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > torch MPS float64 error during ingest
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > was any USB4 pipeline artifact changed by the source locality probe
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > was the (contract ... eventually ...) ISF clause removed
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > was the fresh SWD 29 of 29 artifact promoted
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > was the warning-safe rolling ledger transaction independently audited from a clean clone
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
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > what are AGENT_CLASS_NOUNS / the parenthetical-strip / sentence-boundary / no-preposition guards
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what are APB's remaining completeness candidate misses
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > what are NON_ACTOR_LEADING_FUNCTION_WORDS and NON_ACTOR_LEADING_VERBS for
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
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > what are the canonical SWD protocol surface counts
- [corpus-kb-bounded-projection-shape](../knowledge/corpus-kb-bounded-projection-shape.md)
  > what are the current corpus KB live-document size metrics
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > what are the deeper enum member-quality residual classes after .5.ii / what did the .5.iii measurement find (measured 2026-06-24 read-only, reproducer scripts/measure_enum_width_leak.py: of the 5 deferred classes — glossary SEE…, front-matter/ToC, section-caption B2_3_1_…, _WIDTH parameter leaks, value-restart-of-clean — most are SUBSUMED by .5.i (47/54 _WIDTH members
  > and the bulk of 319 section-caption survivors sit in generic-named enums .5.i drops whole), EXCEPT the _WIDTH leak which reaches the AXI wire-gold .isf and is materially damaging)
- [extraction-quality-gauge-standing](../knowledge/extraction-quality-gauge-standing.md)
  > what are the extraction_quality_* validate metrics and when do they read n/a
- [swd-protocol-surfaces-reach-intentir](../knowledge/swd-protocol-surfaces-reach-intentir.md)
  > what are the isf_protocol residual packet prefixes
- [document-intent-isf-completeness](../knowledge/document-intent-isf-completeness.md)
  > what are the two dominant ISF-lowering completeness gaps (register bit-fields, message-field structures)
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > what are the undeclared-named-subject rule drops at ISF lowering (conditional_rules / signal_constraints / temporal_invariants)
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what blocks a Rust code change from committing in specforge
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > what carries register bit-fields in SpecForge (RegisterFieldRecord in source.rs:414; IntentIr.register_records clone at intent.rs:193 — full metadata survives to IntentIR)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what carries transaction membership faithfully instead of the body (IntentIR metadata: ports / phase_membership / channel_membership)
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > what causes actor_signal_relations / extracted_statements to differ run-to-run
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > what confidence axis correlates with extracted-constraint correctness
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > what debug/diagnostic tools does specforge have (TOOLBOX.md)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did FSMGEN answer about transaction phase membership (don't fabricate value or order; keep value-less participation + unordered membership as IntentIR metadata/residual not body steps; checked phase-group metadata is the future ISF shape on its own FSMGen tree; .isf stays source of truth, no .val)
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what did KG-ISF-TRANSACTIONS.2l measure / decide
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what did KG-ISF-TRANSACTIONS.2n measure / decide
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > what did running nli-verify on a real spec find
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > what did the .2i Rule-A per-phase grouping measurement find (clean only on AHB, empty on APB/AXI/SWD)
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > what did the active PDF task containment census find
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what did the corpus promotion sweep measure (gauge deltas per doc)
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > what distinguishes a PURE-INFERRED phantom from a PROSE-GROUNDED or SECTION+INFERRED 0/0 actor
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > what do leading_section_number and is_descendant_section_number do
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > what document class is a chip-spec PDF (protocol / register / interface / guide)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > what does (on start (sample S as s)) assert in FSMGen semantics (an entry-cycle D-input capture, cycle N port && can_accept)
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > what does .10h do that .10g did not (block-qualified register-mnemonic recovery)
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > what does .10i do that .10h did not (block-qualified recovery of the disjoint register class)
- [adapter-write-reconciles-stale-isf](../knowledge/adapter-write-reconciles-stale-isf.md)
  > what does AdapterArtifact write_to_disk reconcile
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > what does CORPUS-COVERAGE.1 add
- [docling-device-cpu](../knowledge/docling-device-cpu.md)
  > what does DOCLING_DEVICE do
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > what does KG-ISF-TRANSACTIONS.2k add
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > what does LIVE DOCUMENT SIZE CONTAINMENT ADOPTION 8 implement
- [bit-position-structure-field-extraction](../knowledge/bit-position-structure-field-extraction.md)
  > what does MessageFieldRecord.bit_range mean and when is it set
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > what does MessageFieldRecord.byte_offset mean
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > what does SPECFORGE_INGEST_ADAPTIVE_BATCH do
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > what does SPECFORGE_INGEST_MIN_FREE_DISK_MB do
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what does SPECFORGE_INGEST_RAM_ABORT_PERCENT do
- [ingest-ram-guard](../knowledge/ingest-ram-guard.md)
  > what does SPECFORGE_INGEST_RAM_SAMPLE_SECS do
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > what does SPECFORGE_INGEST_SAVE_PAGE_IMAGES do
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > what does SWD Figure B4-1 show (single SWDIO wire packet, bit-field time-phases, Host/Target/Host driver)
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli
- [adopt-defer-ledger](../knowledge/adopt-defer-ledger.md)
  > what does SpecForge take from a grounded author
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > what does SpecForge take from the spec-mining literature and what does it leave out
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > what does a fast category recognizer need beyond surface counts (wire-relation shape, front-matter/self-declared type, topology cue)
- [byte-location-structure-field-extraction](../knowledge/byte-location-structure-field-extraction.md)
  > what does byte_offset mean on a message field record when bit_range is None
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > what does bytes[index] as char do to non-ASCII UTF-8 text in prior_memory.rs
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > what does converge --promote-constraints-llm do and when does it run
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > what does drop_unconditional_overlap_conflicts / unconditional_overlap_residual_packet do in ir/isf_ir.rs
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > what does drop_unrenderable_rule_values / unrenderable_rule_value_residual_packet do in ir/isf_ir.rs
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > what does intent --nli-verify do
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > what does is_dotted_cross_reference_subject do in evidence.rs
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > what does isf_enum_value_is_emittable_literal / isf_enum_is_emittable do in ir/isf_ir.rs
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > what does lead_binds_value do in evidence.rs
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > what does resolve_indexed_signal_family do
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > what does sanitize_isf_name do (allowlist [A-Za-z0-9_] -> everything else becomes _)
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > what does signal_table_covered_by_inventory do
- [active-task-legacy-route-aliases](../knowledge/active-task-legacy-route-aliases.md)
  > what does source_literal mean in the active task evidence contract
