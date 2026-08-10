# Knowledge questions — shard 0004

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > how much memory did the guarded USB4 Connection Manager ingest use
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > how much memory did the guarded USB4 Inter-Domain ingest use
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > how much page content escapes both the structured-element path and the region-crop path
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > how should Rust version copies be verified across Cargo README book and CI
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > how should a behavior-preserving evidence refactor be verified given the non-determinism
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
- [message-field-constraints-surface](../knowledge/message-field-constraints-surface.md)
  > how was the CHI field-constraint routing measured without re-ingesting the PDF
- [agent-identity-structural-gate](../knowledge/agent-identity-structural-gate.md)
  > how was the agent-identity gate proven not to drop real agents (>=8-port proxy, WIRE-BASED-100)
- [eval-gold-interannotator-kappa](../knowledge/eval-gold-interannotator-kappa.md)
  > how was the eval gold checked for idiosyncrasy
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > how was the trailing aux/prep strip proven safe for WIRE-BASED-100 and real agents
- [agent-trailing-fragment-consolidation](../knowledge/agent-trailing-fragment-consolidation.md)
  > how was the trailing-fragment consolidation proven not to regress real agents (WIRE-BASED-100)
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > how were I2S timing_constraints recovered (clock period / clock HIGH / set-up / hold)
- [knowledge-map-shard-contract](../knowledge/knowledge-map-shard-contract.md)
  > how will the million-byte Knowledge Map be sharded without losing question retrieval
- [roadmap-current-history-boundary](../knowledge/roadmap-current-history-boundary.md)
  > how will the oversized SpecForge roadmap be made bounded without losing its history
- [rolling-ledger-record-grammars](../knowledge/rolling-ledger-record-grammars.md)
  > how will the shared rolling ledger archive index be partitioned
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is 'X, which connects to Y, drives Z and W' clause distribution handled
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is (sample input as name) value-free in ISF (yes; (drive input) is rejected — drives exist only for outputs)
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > is AArch64 External Debug a methodology guide or under-extracted architecture
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > is AXI/SWD per-signal phase membership recoverable from timing diagrams via a VLM
- [corpus-reuse-serial-prose-lever-not-cluster-scopable](../knowledge/corpus-reuse-serial-prose-lever-not-cluster-scopable.md)
  > is CORPUS-PATTERN-REUSE.3b.3a a go or no-go
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > is DOC-INTENT-TAXONOMY.4a.ii buildable now (DONE 2026-06-22 — implemented once FSMGen shipped the construct; superseded .4a.i; Gap B packet/flit still deferred)
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > is ISF a single-actor or multi-actor format (per-actor — one .isf describes one actor/module; SpecForge's emit collapses to one initiator via select_initiator_actor; lowering cross-component topology would need a multi-actor emit, an architectural change not an emitter tweak)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is KG-ISF-TRANSACTIONS.2i unparked / what is the .2i decision (yes — FSMGen confirmed option a: ship the grounded per-phase membership grouping as IntentIR metadata, .isf byte-identical)
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > is RISC-V Debug register bit-position recovery a deterministic text-table parse or a VLM-image read (VLM-image — 53/56 diagrams are images, the 7 flattened tables are garbled/symbolic; deterministic parse would fabricate)
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > is SPECFORGE_INGEST_BATCH_PAGES a fixed size or a ceiling
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > is SWD at 100% and on what metric
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > is SpecForge specification mining
- [nli-gate-real-apb-validation](../knowledge/nli-gate-real-apb-validation.md)
  > is SpecForge's constraint extraction over-generating
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > is VALIDATION_SNAPSHOT the latest local artifact validation or the last reviewed projection
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > is a better VLM needed to read register bit-layout diagrams
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is a name-only gate enough to fix the generic enum (no — 271 real-named enums like COMMAND/DWORD_MISR/AMBA are themselves fragment-heavy/dup-heavy; the load-bearing signal is member quality)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is a transaction's phase ORDER recoverable from prose (no — first-occurrence wrong on SWD, cues sparse, precedence conflicting)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is a value-less drive valid in ISF / can an output participate in a transaction body without a value (no — every (drive …) needs a concrete actual; raised to FSMGEN)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is an ISF transaction body ordered (yes — totally ordered, one clause ≈ one cycle; same-cycle concurrency only via a multi-pair drive block)
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > is bar #5 (no silent drop of behavior/temporal) still honest after CORPUS-COVERAGE.0
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > is bar #6 (ISF round-trip, no silent drop) a large faithful-lowering gap
- [evidence-signal-declaration-utf8-boundary-panic](../knowledge/evidence-signal-declaration-utf8-boundary-panic.md)
  > is collect_known_signal_names UTF-8 safe
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > is dropping a zero-evidence actor safe across AXI vs AXI-Stream
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > is intent-bearing content slipping through because nothing reads the full page
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > is it a strict error to drive a signal declared (input ...) in an .isf rule
- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > is it safe to extend the .1b.i trailing-fragment strip to trailing prepositions and auxiliaries (host has -> host, advantage of -> advantage)
- [canonical-promotion-output-path-artifact-layout](../knowledge/canonical-promotion-output-path-artifact-layout.md)
  > is it safe to measure a backup copy of an evidence_ir with nli-verify
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > is mdbook test part of the canonical SpecForge CI gate
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > is qwen2.5:14b-instruct good enough for NLI
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > is relation-completeness the right bar dimension for register / command / coherency protocols
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is removing the generic enums WIRE-BASED-100-safe (scores ORTHOGONAL/SAFE — generic enums are in no scored gold; but the .isf BYTES change on all 4 wire golds — APB/AHB/AXI/SWP each emit a junk TABLE; AHB's TABLE fuses HTRANS+HSIZE which already have correct enums — a strict improvement needing a deliberate snapshot refresh, NOT byte-identical)
- [prior-phrase-utf8-byte-as-char](../knowledge/prior-phrase-utf8-byte-as-char.md)
  > is replace_term_with_placeholder safe on non-ASCII signal/actor names
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > is signal direction faithful to the document now or still defaulted to output
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > is source_ir.json bounded in size for very large PDFs
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is subject coordination (Actor1 and Actor2 drive X) handled / worth building
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > is the .5.ii enum member-quality gate landed (yes, LANDED 2026-06-24: is_prose_fragment_member_name + PROSE_SENTENCE_SPINE_WORDS in ir/evidence.rs gate the member loop in synthesize_encoding_declarations_for_enum, one seam for both call paths; AXI manager.isf now emits (BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7)) recovering
  > codes from the 16-member prose-fused enum; WIRE-BASED-100 1.000 before==after across all 10 seeds, FSMGen --strict success on AXI+APB, kg-bench 156/156, run_ci GREEN lib 1716 +4 tests. .5 enum-surface fidelity now built)
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > is the .isf affected by transaction membership (no — emitter lowers steps, not ports/phase_membership)
- [aarch64-external-debug-guide-refresh-is-authority-empty](../knowledge/aarch64-external-debug-guide-refresh-is-authority-empty.md)
  > is the AArch64 External Debug Guide current-binary refresh complete
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > is the APB PSTRB must be LOW constraint extracted
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > is the APB signal catalog extracted
- [behavior-temporal-lowering-broader-corpus](../knowledge/behavior-temporal-lowering-broader-corpus.md)
  > is the ATP ihi0082 ARVALID/RVALID/RREADY drop a lowering gap
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > is the CoreSight Base System Architecture current-binary refresh complete
- [coresight-base-system-refresh-is-authority-empty](../knowledge/coresight-base-system-refresh-is-authority-empty.md)
  > is the CoreSight Base System Architecture fully extracted
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > is the DOC-INTENT-TAXONOMY .2 per-category scorecard measurement complete (YES after .4e — Gap A lowered .4a.ii, Gap B carrier .4b gated, cat-3 .4c->.4c.i, cat-4 .4d->.4d.i, conditional rules .4e honest residual; remaining work is CODE not measurement)
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > is the DTI ATST 0B01 constraint a real obligation (no — mis-attribution: the source text 'ATTR_OVR.SHCFG must be 0b01' binds SHCFG's value to ATST, which is a value of the FLOW field; an upstream extraction bug spun out of ISF-VALUE-WIDTH-EMIT)
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > is the EvidenceIR build reproducible / deterministic
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > is the ISF storage-var width the register width (no — ir/isf_ir.rs uses max single-field extent, a latent bug; the true width is size_bits or max(bits_high)+1)
- [introducing-coresight-guide-refresh-is-authority-empty](../knowledge/introducing-coresight-guide-refresh-is-authority-empty.md)
  > is the Introducing CoreSight Debug and Trace refresh complete
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > is the LLM-primary promotion a recall improvement
- [canonical-promotion-no-reingest-protocol](../knowledge/canonical-promotion-no-reingest-protocol.md)
  > is the LLM-primary promotion gauge improvement reproducible on canonical artifacts (not just /tmp copies)
- [llm-vlm-provider-default](../knowledge/llm-vlm-provider-default.md)
  > is the LLM/VLM provider missing or not wired up
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is the NLP-SHALLOW-PARSE build frontier exhausted
- [opencapi-certified-definition-refresh](../knowledge/opencapi-certified-definition-refresh.md)
  > is the OpenCAPI 3.0 Certified Definition refresh complete
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > is the OpenCAPI 3.0 Certified Test Resources engineering note refresh complete
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > is the OpenCAPI 3.0 Ready Test Resources engineering note refresh complete
- [opencapi-afu-address-note-refresh](../knowledge/opencapi-afu-address-note-refresh.md)
  > is the OpenCAPI AFU Address Space Usage refresh complete
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > is the PDF-VARIANT-DIGESTION current frontier internally consistent
- [swd-intent-is-the-fsm-driving-swdio](../knowledge/swd-intent-is-the-fsm-driving-swdio.md)
  > is the SWD FSM the same as the JTAG TAP DBGTAPSM (no)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is the SemanticIR phases surface the protocol transaction phases (no — it is section/chapter-derived)
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > is the SpecForge extraction robust and can it grow to a vast set of chip-spec PDFs
- [usb4-connection-manager-refresh-is-authority-empty](../knowledge/usb4-connection-manager-refresh-is-authority-empty.md)
  > is the USB4 Connection Manager current-binary refresh complete
- [usb4-inter-domain-refresh-is-portable-and-authority-empty](../knowledge/usb4-inter-domain-refresh-is-portable-and-authority-empty.md)
  > is the USB4 Inter-Domain Service corpus refresh complete
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
- [opencapi-ready-note-refresh-rejects-dl-signal](../knowledge/opencapi-ready-note-refresh-rejects-dl-signal.md)
  > was DL a signal declaration in the OpenCAPI Ready note
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > was any USB4 pipeline artifact changed by the source locality probe
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > was the (contract ... eventually ...) ISF clause removed
- [swd-canonical-protocol-artifact-is-current](../knowledge/swd-canonical-protocol-artifact-is-current.md)
  > was the fresh SWD 29 of 29 artifact promoted
