# Knowledge questions — shard 0003

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > how is a Reg.Field cross-reference distinguished from a real constraint subject
- [nli-entailment-verifier](../knowledge/nli-entailment-verifier.md)
  > how is a claim's grounding checked beyond a string match
- [section-header-register-field-extraction](../knowledge/section-header-register-field-extraction.md)
  > how is a duplicate register mnemonic (AUTHSTATUS/CSW/IDR reused per access-port block) handled
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how is a fabricated mega-register / over-count avoided when recovering reused register mnemonics
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how is a frame field's width kept honest (why is ARBITRATION FIELD width None not 11)
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > how is a fused Fields(Continued) caption handled
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > how is a fused contract's automation_confidence computed
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how is a guide reported so it is not a silent 0-yield extraction miss
- [message-field-records-surface](../knowledge/message-field-records-surface.md)
  > how is a message field's width kept honest (per-variant widths stay None)
- [llm-primary-promotion-stage](../knowledge/llm-primary-promotion-stage.md)
  > how is a promoted constraint surface visible in the extraction manifest
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > how is a register mnemonic reused across access-port blocks (AUTHSTATUS/CSW/IDR/DEVARCH/CLAIMSET) recovered instead of dropped
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how is a register name recovered from a section heading (RISC-V dmstatus/dmcontrol)
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > how is a register-field mnemonic recovered when the name column is a bit-range (NVMe)
- [docling-metadata-sidecar-paths-are-portable](../knowledge/docling-metadata-sidecar-paths-are-portable.md)
  > how is an external PDF labeled in Docling metadata
- [document-class-from-structure](../knowledge/document-class-from-structure.md)
  > how is an under-extracted spec distinguished from a true guide (evidence_document_underextracted_spec)
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > how is an unless/except exception clause handled in a temporal condition
- [source-pdf-registry-authority](../knowledge/source-pdf-registry-authority.md)
  > how is corpus SOURCE_PDF_REGISTRY currentness checked
- [corpus-kb-managed-currentness](../knowledge/corpus-kb-managed-currentness.md)
  > how is corpus_kb currentness checked
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > how is docs TASK_TREE kept complete without mirroring task history
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > how is docs knowledge INDEX kept complete
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how is prose signal over-capture prevented (no garbage)
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > how is register-field extraction quality measured / scored
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > how is the .5.ii member-quality gate designed / what did the .5.ii calibration find (measured 2026-06-24 read-only over 78 docs/561 enums/12509 members: the gate is PER-MEMBER not per-enum — a whole-enum drop destroys AXI BRESP's real codes OKAY/EXOKAY/SLVERR/DECERR which are FUSED with prose fragments in one conflated enum; value-restart is NOT a junk signal — AHB HPROT
  > restarts but every member is a clean identifier. The load-bearing signal is per-member NAME shape: an English sentence-SPINE token marks a prose fragment. Land a per-member sentence-spine fragment drop at synthesize_encoding_declarations_for_enum)
- [extractor-path-architecture](../knowledge/extractor-path-architecture.md)
  > how is the EvidenceIR extractor path / extraction layer structured and wired
- [fsmgen-feedback-channel](../knowledge/fsmgen-feedback-channel.md)
  > how is the FSMGen feedback channel kept bounded without losing old requests and responses
- [nli-intent-gate](../knowledge/nli-intent-gate.md)
  > how is the NLI gate tested without Ollama
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how is the SWD FSM/frame derivation scored (not constraints/relations/temporal)
- [swd-derivation-scored-100](../knowledge/swd-derivation-scored-100.md)
  > how is the SWDIO sampling and drive-change edge scored
- [prose-signal-capture](../knowledge/prose-signal-capture.md)
  > how is the agent-definition grammar kept garbage-free without a fragile noun denylist
- [can-composition-frame-fields](../knowledge/can-composition-frame-fields.md)
  > how is the composition-frame grammar kept free of corpus false positives
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > how is the corpus distributed across the document intent categories
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > how is the ingest DISK footprint bounded for very large PDFs
- [live-document-containment-fixture-gate](../knowledge/live-document-containment-fixture-gate.md)
  > how is the live-document containment checker tested
- [ingest-adaptive-batch-sizing](../knowledge/ingest-adaptive-batch-sizing.md)
  > how is the page-range batch size chosen / adapted
- [extraction-audit-vlm](../knowledge/extraction-audit-vlm.md)
  > how is the precision of the broadened (non-gold) extraction measured / estimated
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > how is the prose definitional signal grammar kept garbage-free without a denylist (ADR 0006)
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > how is the protocol initiator actor identified structurally without a name list
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > how is the purpose category different from document_class (richer 6-way semantic taxonomy vs coarse 4-way structural proxy; consumes document_class as one input, never replaces it)
- [agnostic-quoted-mode-fsm](../knowledge/agnostic-quoted-mode-fsm.md)
  > how is the quoted-mode FSM extractor kept agnostic and false-positive-free (ADR 0006)
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > how is the required free disk for an ingest estimated
- [transition-bound-state-fsm](../knowledge/transition-bound-state-fsm.md)
  > how is the single-word `<NAME> state` grammar kept false-positive-free without a keyword doc-gate (ADR 0006)
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > how is transaction membership kept boundary-precise across read vs write (bar #3)
- [corpus-refresh-completion-vs-normalized-retention](../knowledge/corpus-refresh-completion-vs-normalized-retention.md)
  > how many CORPUS-COVERAGE re-ingests remain after normalized cleanup
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > how many RISC-V Debug register bit diagrams are images vs flattened tables (53/56 images, 34 field tables, only 7 flattened diagram tables; bits live in the image modality)
- [mdbook-doctest-gap](../knowledge/mdbook-doctest-gap.md)
  > how many SpecForge mdBook doctests currently fail
- [conditional-rule-lowering-triage](../knowledge/conditional-rule-lowering-triage.md)
  > how many conditional_rules are concretely lowerable (only ~3 of 603 across 9 representative docs carry a concrete value/level cue; 161/164 declared-consequent candidates are bare modals shall/must/shall not)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > how many docs / enums are affected (56/78 docs carry a generic-named enum; 96 generic vs 493 real; but a name-only gate misses 271 real-named-but-junk fragment/dup enums — the real defect is member quality)
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > how many docs are wire-protocol vs register-IP vs platform vs ISA vs PHY vs guide
- [corpus-coverage-buildout](../knowledge/corpus-coverage-buildout.md)
  > how many docs reach evidence vs semantic vs intent vs isf
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > how many generated artifacts still mention the deleted boot-volume repository
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > how many register bit-fields fail to lower to .isf (12,638 fields across 32 docs — the largest measurable intent-loss; DOC-INTENT-TAXONOMY.2)
- [register-reset-isf-emit](../knowledge/register-reset-isf-emit.md)
  > how many registers can SpecForge compose a faithful ISF reset for (1508 strictly composable corpus-wide; 1339 fit the current emit width; 446 have V>0 — the real .isf diff)
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > how many signals does each AXI channel have
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > how many transactions does each persisted IntentIR doc have (AXI=9, AHB=3, APB=3)
- [corpus-register-table-shape-gap](../knowledge/corpus-register-table-shape-gap.md)
  > how many unknown-kind tables does the corpus carry
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > how much RAM did the qwen2.5vl:7b VLM use on a timing-diagram crop (13 GB; host hit 87% used, across the 85% kill threshold)
- [corpus-coverage-sweep](../knowledge/corpus-coverage-sweep.md)
  > how much intent does SpecForge extract across the whole corpus
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > how much page content escapes both the structured-element path and the region-crop path
- [evidence-build-nondeterminism](../knowledge/evidence-build-nondeterminism.md)
  > how should a behavior-preserving evidence refactor be verified given the non-determinism
- [axi-channel-structure](../knowledge/axi-channel-structure.md)
  > how should an AXI gold or extraction be structured (per channel)
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > how should the agent-identity / actor noise gate stay agnostic (no name list, ADR 0006)
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
- [nlp-coordination-already-handled](../knowledge/nlp-coordination-already-handled.md)
  > is 'X, which connects to Y, drives Z and W' clause distribution handled
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > is (sample input as name) value-free in ISF (yes; (drive input) is rejected — drives exist only for outputs)
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
