# Knowledge questions — shard 0020

> **AUTO-GENERATED — DO NOT EDIT.** Canonical questions live in fact front matter.

- [toolbox-catalog-is-a-routed-landing](../knowledge/toolbox-catalog-is-a-routed-landing.md)
  > why was TOOLBOX.md partitioned at sections 5-7 rather than anywhere else (measured over its 43 revisions, sections 5, 6 and 7 added 254 of the 370 lines and 100% of the last nine revisions', while sections 1-4 added exactly ONE line in three months - the cut follows the writer, not the line count)
- [axi-constraint-subject-must-be-declared](../knowledge/axi-constraint-subject-must-be-declared.md)
  > why was a property like RME_Support or MPAM_WIDTH extracted as a signal constraint
- [rotated-signal-table-extraction](../knowledge/rotated-signal-table-extraction.md)
  > why was a signal not extracted from a signal table (e.g. AHB HREADY)
- [register-bit-field-isf-lowering-gap](../knowledge/register-bit-field-isf-lowering-gap.md)
  > why was an emitter-only fix for register bit-fields rejected (per-field vars fabricate/lose grouping; set-field/extract fabricate runtime behavior; comments are not intent — feedback_isf_no_hacks)
- [corpus-task-evidence-containment-design](../knowledge/corpus-task-evidence-containment-design.md)
  > why was corpus task maximum line width corrected from 4747 to 4746
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > why was elem_00017|informational_non_contract changed
- [a-sealed-region-cannot-move-out-of-an-active-part-alone](../knowledge/a-sealed-region-cannot-move-out-of-an-active-part-alone.md)
  > why was moving only the region the wrong remedy even though LIVE-DOCUMENT-PRESSURE-HEADROOM.30b did exactly that (the region .30b moved superseded nothing, so it never met this)
- [bit-assignment-register-table-extraction](../knowledge/bit-assignment-register-table-extraction.md)
  > why was nearest-heading anchoring rejected for capless table adoption
- [corpus-refresh-frontier-derivation](../knowledge/corpus-refresh-frontier-derivation.md)
  > why was nvme_base_specification_2_0a_2021_07_26 missing from the corpus refresh frontier (the old count was decremented rather than re-derived, so an expired denominator adjustment silently removed it for twenty-two slices)
- [reviewed-residual-gold-key-law](../knowledge/reviewed-residual-gold-key-law.md)
  > why was table_0004|toc_non_contract changed
- [cat4-csr-bit-position-recovery-not-deterministic](../knowledge/cat4-csr-bit-position-recovery-not-deterministic.md)
  > why was the .4d.i pre-investigation 'deterministically tractable' verdict overturned (gold check: dmstatus flattened table off-by-8 + dropped 7-field band; dmcontrol image-only no table; tdata1 symbolic XLEN-relative positions)
- [one-record-per-obligation-clause](../knowledge/one-record-per-obligation-clause.md)
  > why was the AXI write-address handshake invariant never extracted
- [bounded-ingest-resource-risk-below-page-threshold](../knowledge/bounded-ingest-resource-risk-below-page-threshold.md)
  > why was the Arm Debug replay killed at 400 pages
- [isf-fsm-via-switch-select](../knowledge/isf-fsm-via-switch-select.md)
  > why was the ISF explicit-FSM feature request withdrawn
- [behavioral-reviewed-recipe-boundary](../knowledge/behavioral-reviewed-recipe-boundary.md)
  > why was the at least timing paraphrase rejected
- [evidence-proof-binds-artifact-location](../knowledge/evidence-proof-binds-artifact-location.md)
  > why was the deeper fix of unbinding artifact_layout from the replay topology not taken (it would change the frozen 38-family/170-field producer graph AND invalidate all 24 sealed chains at once, since every sealed proof was taken over a replay that already contains the location. Re-proving on relocation is bounded, keeps every existing seal valid, and cannot persist tampered
  > content because write_to_disk re-verifies against an independent rebuild)
- [research-record-size-profile](../knowledge/research-record-size-profile.md)
  > why was the production-genericity pipeline audit partitioned
- [actor-signal-direction-passive-active-handled](../knowledge/actor-signal-direction-passive-active-handled.md)
  > why was the spike's 'manager Reads ARID' direction error not in production
- [book-behaviour-currency-instrument](../knowledge/book-behaviour-currency-instrument.md)
  > why was the task-tree count in live-docs.md wrong
- [prose-signal-capture-i2c-precision](../knowledge/prose-signal-capture-i2c-precision.md)
  > why were ACK / NACK / DDC / SDR extracted as I2C signals (and how was it fixed)
- [opencapi-discovery-configuration-refresh](../knowledge/opencapi-discovery-configuration-refresh.md)
  > why were BDF DL and VPD removed from OpenCAPI Discovery
- [packet-field-table-declaration](../knowledge/packet-field-table-declaration.md)
  > why were CHI fields mis-typed as signals (the .gauge spurious-subject class)
- [semantic-empty-catalog-disables-grounding-filter](../knowledge/semantic-empty-catalog-disables-grounding-filter.md)
  > why were PWR and OPEN promoted as conditional-rule consequent signals
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why were RISC-V/TRM register tables unextracted (unknown table_kind)
- [property-table-is-not-a-signal-inventory](../knowledge/property-table-is-not-a-signal-inventory.md)
  > why were True and False known signal names in AXI
- [register-field-table-extraction](../knowledge/register-field-table-extraction.md)
  > why were register names synthetic register_table_NNNN and how is the heading association done
- [default-converge-production-boundary](../knowledge/default-converge-production-boundary.md)
  > will a new standalone extraction command fail if converge does not account for it
- [cross-stage-artifact-paths-are-absolute](../knowledge/cross-stage-artifact-paths-are-absolute.md)
  > will generated stage artifacts survive moving the repository
- [swd-frame-phase-binding-lives-in-the-figure](../knowledge/swd-frame-phase-binding-lives-in-the-figure.md)
  > would a MORE PERMISSIVE phase detector rescue the SWD frame fields (NO, and the attempt is a documented trap: a proximity rule that accepts the phase word within four tokens of phase/phases appears to rescue 5 of 11, but every hit is a false positive - statement_1678 names both phases and assigns neither yet is claimed for seven fields, statement_1798 is about a FAULT response,
  > and statement_0813 is 702 statements away. Run the adversarial control in scripts/measure_swd_frame_phase_scope.py)
- [transaction-capture-census](../knowledge/transaction-capture-census.md)
  > would a table-column phase cue move AXI/SWD off empty per-phase grouping (no — .2j NO-GO; the cue is already captured where present and genuinely absent on AXI/SWD)
- [one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample](../knowledge/one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample.md)
  > would a topology-bearing seal key separate a refusing document
- [generic-enum-conflation](../knowledge/generic-enum-conflation.md)
  > would header-sourced naming have recovered the Arm SMMU guide's SEC_SID enum (NO — honest correction recorded at .5.iv: that table's members are whole description sentences, so the .5.ii spine gate drops them all and the enum empties however it is named. The lever is real but does not help the document that surfaced it)
