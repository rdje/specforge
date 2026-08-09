# Knowledge questions — shard 0008

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is plural-rejection unsafe for phase names (access ends in ss)
- [isf-lowering-fidelity-gauge](../knowledge/isf-lowering-fidelity-gauge.md)
  > why is signal direction hard to lower faithfully to a single .isf module
- [apb-signal-catalog-fully-extracted](../knowledge/apb-signal-catalog-fully-extracted.md)
  > why is statement_0223 still a normative statement
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
- [transaction-channel-membership](../knowledge/transaction-channel-membership.md)
  > why is the channel role kept verbatim instead of mapped to address/data/response phases
- [agent-coordinated-subject-split](../knowledge/agent-coordinated-subject-split.md)
  > why is the coordination split safe for WIRE-BASED-100 (AHB relation gold)
- [corpus-cluster-fingerprint](../knowledge/corpus-cluster-fingerprint.md)
  > why is the extraction-profile prior family not scoped by ProtocolFamily
- [agent-interface-block-consolidation](../knowledge/agent-interface-block-consolidation.md)
  > why is the interface consolidation safe for WIRE-BASED-100 (gold docs have no interface actor)
- [timing-table-trapped-row-recovery](../knowledge/timing-table-trapped-row-recovery.md)
  > why is the nested cross-tab timing table (TRANSMITTER/RECEIVER) left an honest residual
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > why is the ordered multi-phase transaction body the hard deferred part (no structural name bridge AXI handshake to named transaction)
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
- [repository-local-scratch](../knowledge/repository-local-scratch.md)
  > why must read-only census commands avoid /tmp and /private/tmp
- [temporal-logic-choice](../knowledge/temporal-logic-choice.md)
  > why not CTL for temporal behavior
- [isf-unconditional-rule-overlap-conflict](../knowledge/isf-unconditional-rule-overlap-conflict.md)
  > why not emit a (priority A over B) to resolve a rule/rule conflict instead of dropping (the unconditional minority would conflict with EVERY same-value unconditional rule → an ungrounded precedence over each = fabrication; tested: priority rule_5 over _0012 cleared one pair then rule_6 conflicted next)
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > why not gate the whole enum on value-restart for .5.ii (DISPROVEN false-positive: AHB HPROT has value restarts=2 from 3 fused sub-encodings but all 15 members are clean identifiers DATA_INST/PRIVILEGED/BUFFERABLE/...; dropping it loses real intent. Restart correlates with conflation but conflation-of-clean-tables is all-real-members, so restart cannot gate a drop — keep it,
  > sub-enum splitting deferred)
- [isf-unrenderable-rule-value-residual](../knowledge/isf-unrenderable-rule-value-residual.md)
  > why not recover (port ARLOOP) from the loopback prose (the temporal 'was presented' loopback is not the current (port ARLOOP); recovering one would fabricate the timing — honest residual over fabrication)
- [transaction-membership-subsection-scope](../knowledge/transaction-membership-subsection-scope.md)
  > why was APB write_transfer / read_transfer membership only PCLK
- [full-page-capture-gap](../knowledge/full-page-capture-gap.md)
  > why was FULL-PAGE-INTENT-CAPTURE.2 not built / closed NO-GO
- [llm-primary-permissive-frame-gate](../knowledge/llm-primary-permissive-frame-gate.md)
  > why was HPROT[0] / HSEL / HTRANS IDLE extracted as a constraint (and how was it fixed)
- [dotted-cross-reference-subject-gate](../knowledge/dotted-cross-reference-subject-gate.md)
  > why was NVMe MPS must_be_value 0 removed / where did the bogus MPS subject come from
- [value-binder-alphabetic-whole-word](../knowledge/value-binder-alphabetic-whole-word.md)
  > why was NVMe SANICAP must_be_value NO removed / where did the bogus NO come from
- [llm-primary-condition-subject-gate](../knowledge/llm-primary-condition-subject-gate.md)
  > why was PSELx / HRESP ERROR / ACTIVATEACK LOW extracted as a constraint (and how was it fixed)
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > why was a property like RME_Support or MPAM_WIDTH extracted as a signal constraint
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > why was a signal not extracted from a signal table (e.g. AHB HREADY)
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > why was an emitter-only fix for register bit-fields rejected (per-field vars fabricate/lose grouping; set-field/extract fabricate runtime behavior; comments are not intent — feedback_isf_no_hacks)
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > why was nearest-heading anchoring rejected for capless table adoption
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > why was the .4d.i pre-investigation 'deterministically tractable' verdict overturned (gold check: dmstatus flattened table off-by-8 + dropped 7-field band; dmcontrol image-only no table; tdata1 symbolic XLEN-relative positions)
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > why was the ISF explicit-FSM feature request withdrawn
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > why was the spike's 'manager Reads ARID' direction error not in production
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > why were ACK / NACK / DDC / SDR extracted as I2C signals (and how was it fixed)
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > why were CHI fields mis-typed as signals (the .gauge spurious-subject class)
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why were RISC-V/TRM register tables unextracted (unknown table_kind)
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why were register names synthetic register_table_NNNN and how is the heading association done
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > will generated stage artifacts survive moving the repository
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > would a table-column phase cue move AXI/SWD off empty per-phase grouping (no — .2j NO-GO; the cue is already captured where present and genuinely absent on AXI/SWD)
