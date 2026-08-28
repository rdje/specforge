# Knowledge questions — shard 0014

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [agent-identity-prose-class-measurement](../knowledge/agent-identity-prose-class-measurement.md)
  > why does the eMMC (JEDEC) IntentIR explode to 153 actors while HBM2 consolidates to 38
- [agent-trailing-function-word-consolidation](../knowledge/agent-trailing-function-word-consolidation.md)
  > why does the eMMC actor count drop 153 to 138 after .1c.i
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why does the emitted .isf default ~98% of signals to output and width 1
- [isf-module-name-hdl-sanitization](../knowledge/isf-module-name-hdl-sanitization.md)
  > why does the emitted .isf module name get sanitized / how is the (actor <name>) label derived
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > why does the fact-card file count differ from the Knowledge Map fact count
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > why does the fact-card landing have one line per card
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > why does the feedback protocol self test block derived state closure
- [no-collection-may-declare-an-aggregate-below-its-own-legal-maximum](../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
  > why does the live-document report show lines below the ceiling
- [doctrine-enforcement-adoption](../knowledge/doctrine-enforcement-adoption.md)
  > why does the pre-commit hook run check_doctrines.sh
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why does the project README get spurious ahb_transfer/apb_transfer transactions
- [cortex-a76-optimization-guide-refresh-is-authority-empty](../knowledge/cortex-a76-optimization-guide-refresh-is-authority-empty.md)
  > why does the refreshed Cortex-A76 Software Optimization Guide emit no ISF
- [gic-overview-guide-refresh-is-authority-empty](../knowledge/gic-overview-guide-refresh-is-authority-empty.md)
  > why does the refreshed Generic Interrupt Controller Overview Guide emit no ISF
- [opencapi-32g-phy-timing-without-interface-topology](../knowledge/opencapi-32g-phy-timing-without-interface-topology.md)
  > why does the refreshed OpenCAPI 4.0 32 Gbps PHY Signaling specification emit no ISF
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > why does the reviewed prose sit at elem_00230 instead of elem_00219
- [a-bounded-snapshot-needs-a-declared-repeatable-rollover](../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md)
  > why does the roadmap archive hold more than one capsule
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why does the same token AXI interface merge in one doc but not another
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why does the same-guard dedup_conflicting_rules miss a conflict between an unconditional rule and a guarded rule
- [validation-snapshot-reviewed-boundary](../knowledge/validation-snapshot-reviewed-boundary.md)
  > why does the tracked validation snapshot differ from current generated IntentIR reports
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > why does the trajectory snapshot reject my fixture builder change
- [relation-completeness-staleness-vs-absence](../knowledge/relation-completeness-staleness-vs-absence.md)
  > why does tilelink have 39 relations in evidence_ir but 0 in intent_ir
- [retained-chain-rebuild-order](../knowledge/retained-chain-rebuild-order.md)
  > why does validate fail with cumulative proof ledger does not retain the exact verified upstream prefix
- [workflow-standard-capacity-is-rederived-from-explicit-member-growth](../decisions/0043-workflow-standard-capacity-is-rederived-from-explicit-member-growth.md)
  > why does workflow-standard capacity become 21
- [fsmgen-temporal-isf-form](../knowledge/fsmgen-temporal-isf-form.md)
  > why doesn't SpecForge emit (contract eventually) anymore
- [stable-obligation-phase-scoped-residual](../knowledge/stable-obligation-phase-scoped-residual.md)
  > why doesn't SpecForge lower stability obligations to (assert (stable sig))
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > why doesn't SpecForge use TLA+
- [dempster-fusion](../knowledge/dempster-fusion.md)
  > why doesn't fusion use the minimum confidence
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > why doesn't the NLI-oracle conformal calibration produce a threshold
- [ingest-disk-preflight](../knowledge/ingest-disk-preflight.md)
  > why doesn't the disk estimate use the page count
- [temporal-eval-residual-fps-are-stale](../knowledge/temporal-eval-residual-fps-are-stale.md)
  > why doesn't the temporal_rule eval reach precision 1.0
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why don't RISC-V Debug register bit-fields reach .isf (all 179 fields are UNLOCATED — field_name/access/reset/description captured but 0 carry bits_high/bits_low/bit_width; the field_table strategy did not parse the bit-layout column)
- [swd-serial-frame-surface](../knowledge/swd-serial-frame-surface.md)
  > why don't parallel buses get serial_frame_fields
- [isf-enum-value-literal-emit-gate](../knowledge/isf-enum-value-literal-emit-gate.md)
  > why drop the enum instead of width/radix-qualifying it (the value is a binary code mis-read as a decimal — the emitter can't recover the radix without fabricating; honest residual over fabrication)
- [fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound](../decisions/0027-fact-plane-capacity-requires-reshaping-the-projection-not-raising-a-bound.md)
  > why is 198 exactly the largest fact-card maximum the current projection allows
- [decision-capacity-is-rederived-without-moving-stable-records](../decisions/0041-decision-capacity-is-rederived-without-moving-stable-records.md)
  > why is ADR 0038 not split or rewritten
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > why is APB PSEL asserted missing from the current reviewed population
- [isf-value-width-operand-contract](../knowledge/isf-value-width-operand-contract.md)
  > why is ATID emitted (width 1) when the IntentIR grounds width 7 (the emitter's first-seen signal dedup isf_ir.rs:696-700 takes the first signal_records entry (w=None→1) and skips the later w=7 record; the .2a.i recovery only falls back to actor_ports, and ATID has none)
- [agent-pure-inferred-phantom-drop](../knowledge/agent-pure-inferred-phantom-drop.md)
  > why is AXI transmitter / SWD host / GIC arbiter kept but APB controller / AHB agent dropped
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is AXI/SWD per-signal phase membership empty (document absence — AXI 0/4, SWD 0/63 signal↔phase co-occurrence; phases described abstractly/by-packet, not per declared wire — not an extraction gap)
- [corpus-reuse-activate-only-no-current-consumer](../knowledge/corpus-reuse-activate-only-no-current-consumer.md)
  > why is CORPUS-PATTERN-REUSE.3b.3b (activate-only consume) deferred / not built
- [document-intent-category-recognizer](../knowledge/document-intent-category-recognizer.md)
  > why is CPU-ISA / PHY only recognized from front-matter (no distinct structural signature; cat 5 vs cat 6 indistinguishable by structure)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why is DTI DOWNSTREAM not preserved as a heuristic only wire
- [agent-surface-defect-taxonomy](../knowledge/agent-surface-defect-taxonomy.md)
  > why is For / Then it / is recommended / next / HPROT bit / TREADY input minted as an actor
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is KG-ISF-TRANSACTIONS.2i parked/blocked (waiting on FSMGEN to say how to lower phase membership without fabricating drive values or step order)
- [source-ir-size-scaling](../knowledge/source-ir-size-scaling.md)
  > why is MEMORY-BOUNDED-INGEST.5 summary streaming deferred
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why is NVMe register field_name a bit-range and how is the mnemonic found in the description
- [legacy-generic-section-phases-are-audit-only](../knowledge/legacy-generic-section-phases-are-audit-only.md)
  > why is Reset value not a semantic phase
- [transaction-phase-membership-vlm-vs-channel](../knowledge/transaction-phase-membership-vlm-vs-channel.md)
  > why is SWD per-signal phase membership degenerate / empty
- [swd-adi-not-signal-table-spec](../knowledge/swd-adi-not-signal-table-spec.md)
  > why is SWD/ADI hard / different from APB AHB AXI
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is SWIO not captured as a third SWP signal
- [spec-mining-framing](../knowledge/spec-mining-framing.md)
  > why is SpecForge called forward specification mining
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is Subordinate interface merged to Subordinate but GIC CPU interface kept intact
- [task-tree-catalog](../knowledge/task-tree-catalog.md)
  > why is TEMPLATE excluded from the task catalog
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why is Vital Product Data (VPD) not a signal
- [parenthetical-data-head-requires-wire-qualifier](../knowledge/parenthetical-data-head-requires-wire-qualifier.md)
  > why is Wishbone memory output data (DO) not a port
- [section-header-register-block-qualification](../knowledge/section-header-register-block-qualification.md)
  > why is a CSW@MEM-AP / CLAIMSET@AP qualified register name emitted, and is it .isf-safe
- [semantic-grounding-filter-is-catalog-independent](../knowledge/semantic-grounding-filter-is-catalog-independent.md)
  > why is a conditional rule in EvidenceIR but not in SemanticIR
- [evidence-statement-markdown-escape-truncates-identifiers](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md)
  > why is a constraint subject CLK when the document says CLK_I
- [fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy](../decisions/0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
  > why is a fact-card aggregate ceiling the file bound times the per-file bound
- [bit-location-register-field-vocabulary](../knowledge/bit-location-register-field-vocabulary.md)
  > why is a field name accepted or rejected from a description cell
- [captured-region-residual-carrier](../knowledge/captured-region-residual-carrier.md)
  > why is a figure caption not coverage for the figure
- [a-bounded-snapshot-bounds-its-sections-not-just-its-file](../decisions/0031-a-bounded-snapshot-bounds-its-sections-not-just-its-file.md)
  > why is a file-level line ceiling not enough for a bounded snapshot
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > why is a gerund after while/when not a condition (action coordination)
- [transaction-body-emission-faithfully-complete](../knowledge/transaction-body-emission-faithfully-complete.md)
  > why is a membership-derived (sample) body NOT faithful even though it is FSMGen-accepted
- [isf-rule-transaction-priority-authority](../knowledge/isf-rule-transaction-priority-authority.md)
  > why is a multi-caller named drive kept without actor priority
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > why is a pure-hex-literal subject filter unsafe (CBA, BADD)
- [normalized-bundle-retention-is-declared](../knowledge/normalized-bundle-retention-is-declared.md)
  > why is a retained bundle count not the same as corpus refresh progress
- [legacy-generic-gates-are-audit-only](../knowledge/legacy-generic-gates-are-audit-only.md)
  > why is a sentence containing when not necessarily a semantic gate
- [timing-observation-to-verified-figure-contract](../knowledge/timing-observation-to-verified-figure-contract.md)
  > why is a stable span after tick zero residual
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > why is an ambiguous derived id pairing a fail rather than an invalid alpha transform
- [corpus-wide-interface-authority-rebuild](../knowledge/corpus-wide-interface-authority-rebuild.md)
  > why is an emitted-ISF count from an earlier refresh slice not the current number
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > why is an exact archive alone insufficient for the active corpus task
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why is bus not a valid parenthetical single-wire head
- [cat3-topology-isf-lowering-decision](../knowledge/cat3-topology-isf-lowering-decision.md)
  > why is category 3 (platform/system-IP) topology not lowered to .isf (ISF has no static-topology construct + the emit is single-initiator-actor; cross-component topology is structurally absent from the emit by design — KG-ISF-COMPLETENESS.2a.ii)
- [cat4-isa-csr-lowering-decision](../knowledge/cat4-isa-csr-lowering-decision.md)
  > why is category 4 (CPU ISA) ISF-thin (the only lowerable cat-4 intent is CSRs, which are registers; their bit-fields are UNLOCATED so they don't reach .isf — an extraction-recall gap, not a missing ISF construct)
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > why is channel membership metadata-only and not lowered to .isf
- [conformal-tier-agreement-degenerate](../knowledge/conformal-tier-agreement-degenerate.md)
  > why is conformal calibration still blocked at CHI scale
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > why is consequent_signal NOTICE or PDF or IMPLEMENTATION or MUST in SemanticIR
- [host-library-route-remains-on-boot-volume](../knowledge/host-library-route-remains-on-boot-volume.md)
  > why is corpus coverage refresh 34 blocked before ingest
- [retrospective-baseline-current-replay-boundary](../knowledge/retrospective-baseline-current-replay-boundary.md)
  > why is current binary replay coverage 1 of 12
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is descriptor apposition (signal NAME / NAME signal) NOT used to capture signal names
- [source-ir-ingest-not-reproducible](../knowledge/source-ir-ingest-not-reproducible.md)
  > why is exact source region capture 13 of 14
- [passive-binding-subject-authority](../knowledge/passive-binding-subject-authority.md)
  > why is must have its WSTRB input tied HIGH still extracted after the pre-bind subject repair
- [retained-chain-rebuild-order](../knowledge/retained-chain-rebuild-order.md)
  > why is my rebuilt chain stale even though the content matches
- [page-image-disk-bounding](../knowledge/page-image-disk-bounding.md)
  > why is page_image_path null / None for a large document
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is plural-rejection unsafe for phase names (access ends in ss)
- [actionable-published-claims-require-three-dimensionally-different-legs](../decisions/0042-actionable-published-claims-require-three-dimensionally-different-legs.md)
  > why is repeating the same check not independent verification
- [required-residual-actionability-denominator](../knowledge/required-residual-actionability-denominator.md)
  > why is residual actionability 8 of 16 in the current reviewed result
- [legal-administrative-prose-is-not-semantic-authority](../knowledge/legal-administrative-prose-is-not-semantic-authority.md)
  > why is section-title boilerplate filtering insufficient
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why is signal direction hard to lower faithfully to a single .isf module
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > why is source_ref ambiguous
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > why is statement_0223 still a normative statement
- [ingest-drops-figure-interior-text](../knowledge/ingest-drops-figure-interior-text.md)
  > why is text inside a diagram missing from SourceIR
- [fsmgen-ignores-signal-direction](../knowledge/fsmgen-ignores-signal-direction.md)
  > why is the .isf direction default (output) FSMGen-neutral / not a faithful-lowering gap
- [isf-initiator-perspective-direction](../knowledge/isf-initiator-perspective-direction.md)
  > why is the .isf module named after the initiator (manager / requester / debugger) instead of actors.first()
- [eval-scores-persisted-evidence](../knowledge/eval-scores-persisted-evidence.md)
  > why is the AHB eval baseline wrong or stale
- [register-diagram-bit-recovery-via-tiling](../knowledge/register-diagram-bit-recovery-via-tiling.md)
  > why is the Docling table capture of a register diagram unreliable
- [swd-protocol-fsm-surface](../knowledge/swd-protocol-fsm-surface.md)
  > why is the FSM important for SWD/JTAG
- [local-llm-for-text-reasoning](../knowledge/local-llm-for-text-reasoning.md)
  > why is the NLI framing better than free-form labeling
- [indexed-signal-family-canonicalization](../knowledge/indexed-signal-family-canonicalization.md)
  > why is the PSEL antecedent dropped in a temporal rule
- [fact-card-catalog](../knowledge/fact-card-catalog.md)
  > why is the SpecForge fact-card catalog almost out of capacity
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why is the USB 3.2 adapter syntactically valid but semantically untrustworthy
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is the X interface strip gated on X being a connected agent in this doc
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why is the _WIDTH enum-member leak a real fidelity defect (.5.iii: 7 _WIDTH members in real-signal-named enums in AXI gold ihi0022_l reach manager.isf — (BRESP (BRESP_WIDTH 0)(OKAY 0)…) duplicates value 0, (RRESP (RRESP_WIDTH 0)) REPLACES the real RRESP codes, (AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1)) pure junk; a width PARAMETER 'Enum BRESP BRESP_WIDTH = 0.' mis-read as
  > an encoding VALUE — a false bar-#6 fact, unscored by WIRE-BASED-100 since enums are emitter-orthogonal)
- [definitional-signal-capture](../knowledge/definitional-signal-capture.md)
  > why is the abbreviation-table I/O-expansion not used to capture signals (MMIO/DMA/IOVA garbage)
- [offset-suffixed-dword-relative-bit-cells](../knowledge/offset-suffixed-dword-relative-bit-cells.md)
  > why is the absolute bit position never derived from offset*8+bit
- [section-header-register-identity-collapse](../knowledge/section-header-register-identity-collapse.md)
  > why is the block name not used to qualify the duplicate registers (heading levels flattened)
- [active-task-migration-transaction](../knowledge/active-task-migration-transaction.md)
  > why is the bounded active task root written last
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > why is the channel role kept verbatim instead of mapped to address/data/response phases
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > why is the coordination split safe for WIRE-BASED-100 (AHB relation gold)
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > why is the default ingest threshold 131 pages on a 24 GiB host
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > why is the extraction-profile prior family not scoped by ProtocolFamily
- [fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards](../decisions/0026-fact-plane-capacity-is-derived-and-the-landing-stops-scaling-with-cards.md)
  > why is the fact-card catalog about to run out of capacity
- [inference-antecedent-local-grounding-stops-at-semantic-layer-d](../knowledge/inference-antecedent-local-grounding-stops-at-semantic-layer-d.md)
  > why is the first SPEC-TO-INTENT-ALIGNMENT.7c replay not publishable
- [first-reviewed-trajectory-snapshot](../knowledge/first-reviewed-trajectory-snapshot.md)
  > why is the first trajectory snapshot diverging with insufficient history
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > why is the in-repo corpus tree outside the host-library refresh cohort
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is the interface consolidation safe for WIRE-BASED-100 (gold docs have no interface actor)
- [reviewed-population-clean-replay-carrier-regression](../knowledge/reviewed-population-clean-replay-carrier-regression.md)
  > why is the latest reviewed population replay not published
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > why is the nested cross-tab timing table (TRANSMITTER/RECEIVER) left an honest residual
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is the ordered multi-phase transaction body the hard deferred part (no structural name bridge AXI handshake to named transaction)
- [semantic-interface-authority-empty-fallback](../knowledge/semantic-interface-authority-empty-fallback.md)
  > why is the repaired USB 3.2 ISF adapter blocked
- [reviewed-fixture-projection-digest-lockstep](../knowledge/reviewed-fixture-projection-digest-lockstep.md)
  > why is the reviewed dataset review-locked
- [model-misspelled-subject-snap](../knowledge/model-misspelled-subject-snap.md)
  > why is the snap trigger absence-from-sentence and not typing failure
- [stage-staleness-validate-detector](../knowledge/stage-staleness-validate-detector.md)
  > why is the stage-staleness check zero-versus-some and not a count comparison
- [register-field-eval-measure-and-surface](../knowledge/register-field-eval-measure-and-surface.md)
  > why is the strict register-field per-fact score 0 on RISC-V Debug / NVMe
- [document-intent-category-census](../knowledge/document-intent-category-census.md)
  > why is the structural document_class (protocol/register/interface/guide) too coarse for the purpose taxonomy
- [message-field-validate-integration](../knowledge/message-field-validate-integration.md)
  > why is there no fields-without-positions completeness gap
- [inference-antecedent-state-loss](../knowledge/inference-antecedent-state-loss.md)
  > why must PSEL not inherit VALID from the which means consequence
- [no-collection-may-declare-an-aggregate-below-its-own-legal-maximum](../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
  > why must a collection aggregate be at least files times per-file
- [current-claim-census-freeze](../knowledge/current-claim-census-freeze.md)
  > why must a current claim census candidate key include the semantic view
- [behavioral-held-out-oracle-defects](../knowledge/behavioral-held-out-oracle-defects.md)
  > why must opaque identifiers remain one grammar token
- [repository-local-scratch](../knowledge/repository-local-scratch.md)
  > why must read-only census commands avoid /tmp and /private/tmp
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > why must source-library paths not determine whether a corpus document was refreshed
- [inference-antecedent-local-grounding-stops-at-semantic-layer-d](../knowledge/inference-antecedent-local-grounding-stops-at-semantic-layer-d.md)
  > why must source-local PSEL not become a global interface signal
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > why must terminal task tree containment use two commits
- [live-document-coverage-authority](../knowledge/live-document-coverage-authority.md)
  > why must the bounded SPEC-TO-INTENT-ALIGNMENT root retain every task id
- [behavioral-semantic-negative-sensitivity](../knowledge/behavioral-semantic-negative-sensitivity.md)
  > why must the invariant comparator reject a semantic negative first
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > why not CTL for temporal behavior
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > why not create a new corpus coverage continuation tree
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why not emit a (priority A over B) to resolve a rule/rule conflict instead of dropping (the unconditional minority would conflict with EVERY same-value unconditional rule → an ungrounded precedence over each = fabrication; tested: priority rule_5 over _0012 cleared one pair then rule_6 conflicted next)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why not gate the whole enum on value-restart for .5.ii (DISPROVEN false-positive: AHB HPROT has value restarts=2 from 3 fused sub-encodings but all 15 members are clean identifiers DATA_INST/PRIVILEGED/BUFFERABLE/...; dropping it loses real intent. Restart correlates with conflation but conflation-of-clean-tables is all-real-members, so restart cannot gate a drop — keep it,
  > sub-enum splitting deferred)
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > why not recover (port ARLOOP) from the loopback prose (the temporal 'was presented' loopback is not the current (port ARLOOP); recovering one would fabricate the timing — honest residual over fabrication)
- [live-document-derived-state-contract-gap](../knowledge/live-document-derived-state-contract-gap.md)
  > why should corpus counts leave MEMORY md
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > why was APB write_transfer / read_transfer membership only PCLK
- [dense-prose-false-signal-loop-reaches-isf](../knowledge/dense-prose-false-signal-loop-reaches-isf.md)
  > why was CORPUS-COVERAGE 2 33d iii closed without another convergence or adapter filter
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > why was FULL-PAGE-INTENT-CAPTURE.2 not built / closed NO-GO
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > why was HPROT[0] / HSEL / HTRANS IDLE extracted as a constraint (and how was it fixed)
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > why was NVMe MPS must_be_value 0 removed / where did the bogus MPS subject come from
