# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even while only the early IR stages are implemented

## Session update (2026-05-07 KG fixture for no-prior modality VLM exclusion)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 0` alongside aggregate, table, prose, alias-grounded prose, and visual-caption hint metrics.
- This locks the local table/prose conflict as free of VLM timing-annotation evidence when modality memory is absent.

## Session update (2026-05-07 KG fixture for no-prior modality visual exclusion)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_visual_captions = 0` alongside aggregate, table, prose, and alias-grounded prose hint metrics.
- This locks the local table/prose conflict as free of visual-caption evidence when modality memory is absent.

## Session update (2026-05-07 KG fixture for no-prior modality alias exclusion)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside aggregate, table, and prose hint metrics.
- This locks the local table/prose conflict as free of alias-grounded prose when modality memory is absent.

## Session update (2026-05-07 KG fixture for no-prior modality prose split)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 1` alongside aggregate and table hint metrics.
- This locks the local prose-side observation as counted EvidenceIR when modality memory is absent.

## Session update (2026-05-07 KG fixture for no-prior modality table split)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 1` alongside the aggregate hint metric.
- This locks the local table-side observation as counted EvidenceIR when modality memory is absent.

## Session update (2026-05-07 KG fixture for no-prior modality hint totals)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints = 2` for the no-prior semantic-modality mirror.
- This locks the local table/prose conflict as two counted EvidenceIR semantic hints when modality memory is absent.

## Session update (2026-05-07 KG fixture for weak modality-prior timing-extraction absence)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `timing_diagram_extractions = 0` alongside its full semantic-hint source split.
- This locks the local table/prose conflict as free of timing-diagram extraction support while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for weak modality-prior VLM exclusion)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 0` alongside aggregate, table, prose, alias-grounded prose, and visual-caption hint metrics.
- This locks the local table/prose conflict as free of VLM timing-annotation evidence while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for weak modality-prior visual exclusion)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_visual_captions = 0` alongside aggregate, table, prose, and alias-grounded prose hint metrics.
- This locks the local table/prose conflict as free of visual-caption evidence while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for weak modality-prior alias exclusion)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside aggregate, table, and prose hint metrics.
- This locks the local table/prose conflict as free of alias-grounded prose while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for weak modality-prior prose split)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 1` alongside aggregate and table hint metrics.
- This locks the local prose-side observation as counted EvidenceIR while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for weak modality-prior table split)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 1` alongside the aggregate hint metric.
- This locks the local table-side observation as counted EvidenceIR while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for weak modality-prior hint totals)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints = 2` for the underpowered modality-prior case.
- This locks the local table/prose conflict as two counted EvidenceIR semantic hints while weak modality memory stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch timing-extraction absence)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `timing_diagram_extractions = 0` alongside its full semantic-hint source split.
- This locks the local table/prose conflict as free of timing-diagram extraction support while the visual-caption-only modality prior stays non-authoritative for non-visual evidence.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch VLM exclusion)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 0` alongside aggregate, table, prose, alias-grounded prose, and visual-caption hint metrics.
- This locks the local table/prose conflict as free of VLM timing-annotation evidence while the visual-caption-only modality prior stays non-authoritative for non-visual evidence.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch visual exclusion)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_visual_captions = 0` alongside aggregate, table, prose, and alias-grounded prose hint metrics.
- This locks the local table/prose conflict as free of visual-caption evidence while the visual-caption-only modality prior stays non-authoritative for non-visual evidence.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch alias exclusion)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside aggregate, table, and prose hint metrics.
- This locks the local table/prose conflict as free of alias-grounded prose while the visual-caption-only modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch prose split)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 1` alongside aggregate and table hint metrics.
- This locks the local prose-side observation as counted EvidenceIR while the visual-caption-only modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch table split)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 1` alongside the aggregate hint metric.
- This locks the local table-side observation as counted EvidenceIR while the visual-caption-only modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch hint totals)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints = 2` for the visual-prior source-kind mismatch case.
- This locks the local table/prose conflict as two counted EvidenceIR semantic hints while the visual-caption-only modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch timing-extraction absence)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `timing_diagram_extractions = 0` alongside its full semantic-hint source split.
- This locks the AXI-local conflict as free of timing-diagram extraction support while the unrelated APB modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch VLM exclusion)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 0` alongside aggregate, table, prose, alias-grounded prose, and visual-caption hint metrics.
- This locks the AXI-local conflict as free of VLM timing-annotation evidence while the unrelated APB modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch visual exclusion)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_visual_captions = 0` alongside aggregate, table, and prose hint metrics.
- This locks the AXI-local conflict as free of visual-caption evidence while the unrelated APB modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch alias exclusion)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside aggregate, table, and prose hint metrics.
- This locks the AXI-local conflict as free of alias-grounded prose while the unrelated APB modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch prose split)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 1` alongside aggregate and table hint metrics.
- This locks the AXI-local prose observation as counted evidence while the unrelated APB modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch table split)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 1` alongside the aggregate hint metric.
- This locks the AXI-local table observation as counted evidence while the unrelated APB modality prior stays non-authoritative.

## Session update (2026-05-07 KG fixture for semantic-modality family-mismatch hint totals)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints = 2` for the scoped-out APB modality-prior mismatch case.
- This locks the AXI-local table/prose semantic observations as counted evidence while keeping unrelated prior memory non-authoritative.

## Session update (2026-05-07 KG fixture for negative-knowledge caution alias exclusion)
- Tightened `negative_knowledge_prior_guided_semantic_conflict_caution_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside its visual source semantic-hint metrics.
- This locks the caution-guided semantic conflict as free of alias-grounded prose evidence while preserving negative-knowledge guidance.

## Session update (2026-05-07 KG fixture for negative-knowledge caution prose exclusion)
- Tightened `negative_knowledge_prior_guided_semantic_conflict_caution_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 0` alongside its visual source semantic-hint metrics.
- This locks the caution-guided semantic conflict as free of ordinary prose evidence while preserving negative-knowledge guidance.

## Session update (2026-05-07 KG fixture for negative-knowledge caution table exclusion)
- Tightened `negative_knowledge_prior_guided_semantic_conflict_caution_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 0` alongside its visual source semantic-hint metrics.
- This locks the caution-guided semantic conflict as visual evidence only while preserving negative-knowledge guidance.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch timing extraction)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `timing_diagram_extractions = 1` alongside its semantic-hint metrics.
- This locks the family-mismatch VLM timing semantic hint as extraction-backed while keeping unrelated APB negative-knowledge memory scoped out.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch alias exclusion)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside aggregate and visual source semantic-hint metrics.
- This locks the semantic conflict as free of alias-grounded prose evidence while keeping unrelated APB negative-knowledge memory scoped out.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch prose exclusion)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 0` alongside aggregate and visual source semantic-hint metrics.
- This locks the semantic conflict as free of ordinary prose evidence while keeping unrelated APB negative-knowledge memory scoped out.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch table exclusion)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 0` alongside aggregate, visual-caption, and VLM timing semantic-hint metrics.
- This locks the semantic conflict as visual evidence only while keeping unrelated APB negative-knowledge memory scoped out.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch VLM split)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 1` alongside aggregate and visual-caption semantic-hint metrics.
- This locks the AXI-local VLM timing side of the semantic conflict while keeping unrelated APB negative-knowledge memory scoped out.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch visual split)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_visual_captions = 1` alongside the aggregate semantic-hint metric.
- This locks the AXI-local visual-caption side of the semantic conflict while keeping unrelated APB negative-knowledge memory scoped out.

## Session update (2026-05-07 KG fixture for cross-modality grounding VLM exclusion)
- Tightened `cross_modality_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 0` alongside the aggregate and other source semantic-hint metrics.
- This locks decisive `XREQ` cross-modality semantic grounding as free of VLM timing-note semantic-hint participation.

## Session update (2026-05-07 KG fixture for cross-modality grounding alias exclusion)
- Tightened `cross_modality_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside the aggregate and direct source semantic-hint metrics.
- This locks decisive `XREQ` cross-modality semantic grounding as free of alias-grounded prose semantic-hint participation.

## Session update (2026-05-07 KG fixture for cross-modality grounding prose exclusion)
- Tightened `cross_modality_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 0` alongside the aggregate, table, and visual-caption semantic-hint metrics.
- This locks decisive `XREQ` cross-modality semantic grounding as free of ordinary prose semantic-hint participation.

## Session update (2026-05-07 KG fixture for cross-modality grounding visual split)
- Tightened `cross_modality_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_visual_captions = 1` alongside the aggregate and table semantic-hint metrics.
- This locks the visual-caption half of decisive `XREQ` cross-modality semantic grounding as a source-specific EvidenceIR metric.

## Session update (2026-05-07 KG fixture for cross-modality grounding table split)
- Tightened `cross_modality_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_tables = 1` alongside the aggregate semantic-hint metric.
- This locks the table half of decisive `XREQ` cross-modality semantic grounding as a source-specific EvidenceIR metric.

## Session update (2026-05-07 KG fixture for cross-modality grounding hint totals)
- Tightened `cross_modality_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints = 2` for the positive table plus visual-caption grounding path.
- This locks decisive `XREQ` cross-modality semantic grounding to the aggregate EvidenceIR semantic-hint metric.

## Session update (2026-05-07 KG fixture for cross-modality semantic-conflict VLM split)
- Tightened `cross_modality_semantic_conflict_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_vlm_timing_annotations = 0` alongside all other source-split hint metrics.
- This locks the unresolved `XCTRL` semantic-role conflict as caption/table evidence, not VLM timing-note evidence.

## Session update (2026-05-07 KG fixture for cross-modality semantic-conflict alias split)
- Tightened `cross_modality_semantic_conflict_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_alias_grounded_prose = 0` alongside total, table, prose, and visual-caption hint metrics.
- This locks the unresolved `XCTRL` semantic-role conflict as free of alias-grounded prose participation.

## Session update (2026-05-07 KG fixture for cross-modality semantic-conflict prose split)
- Tightened `cross_modality_semantic_conflict_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints_from_prose = 0` alongside the total, table, and visual-caption hint metrics.
- This locks the unresolved `XCTRL` semantic-role conflict as table-plus-visual only, not prose-contaminated.

## Session update (2026-05-07 KG fixture for cross-modality semantic-conflict hint totals)
- Tightened `cross_modality_semantic_conflict_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts evidence-stage `signal_semantic_hints = 2` alongside the table and visual-caption source splits.
- This locks aggregate semantic-hint metrics to the two conflicting observations used by the unresolved `XCTRL` semantic-role conflict.

## Session update (2026-05-07 KG fixture for waveform-motion VLM timing total hint exclusion)
- Tightened `vlm_timing_waveform_motion_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks waveform motion states as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for motion-only VLM timing total hint exclusion)
- Tightened `vlm_timing_motion_annotation_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks motion-only timing annotations as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for cycle-qualified VLM timing total hint exclusion)
- Tightened `vlm_timing_cycle_qualified_signal_value_annotation_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks cycle-qualified signal-value labels as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for indexed VLM timing total hint exclusion)
- Tightened `vlm_timing_indexed_signal_value_annotation_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks indexed signal-value labels as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for spurious VLM timing total hint exclusion)
- Tightened `vlm_timing_spurious_annotation_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks low-value timing labels and value-like annotations as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for active-low VLM deassertion total hint exclusion)
- Tightened `vlm_timing_active_low_deassertion_equivalence_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks active-low reset-release timing evidence as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for active-low VLM assertion total hint exclusion)
- Tightened `vlm_timing_active_low_assertion_equivalence_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts total evidence-stage `signal_semantic_hints = 0` in addition to the VLM timing-annotation source split.
- This locks active-low reset assertion timing evidence as semantic-role silent across all evidence hint sources.

## Session update (2026-05-07 KG fixture for active-low VLM deassertion hint exclusion)
- Tightened `vlm_timing_active_low_deassertion_equivalence_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts zero VLM timing-annotation semantic hints while keeping the timing extraction and polarity-aware temporal rules intact.
- This locks reset-release annotations as temporal/polarity evidence only, not accidental handshake-role recovery.

## Session update (2026-05-07 KG fixture for active-low VLM assertion hint exclusion)
- Tightened `vlm_timing_active_low_assertion_equivalence_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts zero VLM timing-annotation semantic hints while keeping the timing extraction and polarity-aware temporal rules intact.
- This locks reset assertion annotations as temporal/polarity evidence only, not accidental handshake-role recovery.

## Session update (2026-05-07 KG fixture for VLM timing semantic-grounding extraction metric)
- Tightened `vlm_timing_semantic_grounding_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts the evidence-stage `timing_diagram_extractions = 1` metric alongside VLM timing-annotation semantic hints.
- This locks VLM timing semantic recovery as extraction-backed evidence rather than a detached downstream role hint.

## Session update (2026-05-07 KG fixture for duplicate-initial warning exclusion)
- Tightened `vlm_state_machine_duplicate_initial_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now explicitly excludes semantic and intent state-machine initial-cardinality findings while keeping the merged canonical initial count at `1`.
- This locks duplicate-state merging as both shape-correct and validation-clean.

## Session update (2026-05-07 KG fixture for state-machine label-noise initial shape)
- Tightened `vlm_state_machine_label_noise_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts `IDLE` remains the only canonical initial state and `initial_regular_states` remains `1` after prose/OCR-like VLM labels are filtered.
- This locks label-noise filtering as non-destructive to declared state truth at both canonical stages.

## Session update (2026-05-07 KG fixture for undeclared-transition initial state shape)
- Tightened `vlm_state_machine_undeclared_transition_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts `IDLE` remains the only canonical initial state and `initial_regular_states` remains `1` after undeclared transition endpoints are filtered.
- This locks endpoint filtering as non-destructive to declared state truth at both canonical stages.

## Session update (2026-05-07 KG fixture for multiple-initial state-machine finding payloads)
- Tightened `vlm_state_machine_multiple_initial_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact semantic and intent validation warning payloads for a VLM state-machine extraction with more than one canonical initial state.
- This locks the multiple-initial state-machine guard as inspectable graph preservation plus warning-level validation details tied to `IDLE` and `BUSY`.

## Session update (2026-05-07 KG fixture for missing-initial state-machine finding payloads)
- Tightened `vlm_state_machine_missing_initial_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact semantic and intent validation warning payloads for a VLM state-machine extraction with no canonical initial state.
- This locks the missing-initial state-machine guard as inspectable graph preservation plus warning-level validation details tied to `IDLE` and `BUSY`.

## Session update (2026-05-07 KG fixture for visual-motif corroboration finding payload)
- Tightened `visual_motif_prior_guided_diagram_classification_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts the exact evidence-stage corroboration guidance payload for prior-classified visual evidence, including related id `visual_0001`.
- This locks visual-motif memory as a diagram-classification aid that preserves targeted multimodal corroboration debt instead of silently promoting semantic facts.

## Session update (2026-05-07 KG fixture for weak modality-prior finding payloads)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact non-decisive arbitration plus role/conflict rescan-guidance validation payloads for `XCTRL` and `semantic_conflict_0001` at both canonical stages when modality-prior support is underpowered.
- This locks the weak semantic-modality prior guard as actionable arbitration debt while keeping the prior-guided resolution findings absent.

## Session update (2026-05-07 KG fixture for modality source-kind mismatch finding payloads)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact non-decisive arbitration plus role/conflict rescan-guidance validation payloads for `XCTRL` and `semantic_conflict_0001` at both canonical stages when only a visual-caption modality prior is staged.
- This locks the semantic-modality source-kind mismatch guard as actionable arbitration debt while keeping the prior-guided resolution findings absent.

## Session update (2026-05-07 KG fixture for modality family-mismatch finding payloads)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact non-decisive arbitration plus role/conflict rescan-guidance validation payloads for `XCTRL` and `semantic_conflict_0001` at both canonical stages when only an unrelated APB modality prior is staged.
- This locks the semantic-modality protocol-family mismatch guard as actionable arbitration debt while keeping the prior-guided resolution findings absent.

## Session update (2026-05-07 KG fixture for no-prior modality finding payloads)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact non-decisive arbitration plus role/conflict rescan-guidance validation payloads for `XCTRL` and `semantic_conflict_0001` at both canonical stages.
- This locks the no-prior semantic-modality mirror as actionable arbitration debt, not merely a conflict count or finding-id presence check.

## Session update (2026-05-07 KG fixture for AMBA-generic modality finding payloads)
- Tightened `semantic_modality_reliability_amba_generic_fallback_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact prior-guided arbitration, consensus, and rescan-guidance validation payloads for AMBA-generic modality fallback at both canonical stages.
- This locks generic AMBA modality memory as validation-detail-equivalent to exact-family memory without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for negative-knowledge family-mismatch arbitration)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts `XCTRL` enters semantic arbitration but remains explicitly non-decisive at both canonical stages when only an unrelated APB-scoped negative-knowledge prior is staged.
- This locks the negative-knowledge protocol-family mismatch path as prior-silent and arbitration-preserving without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for negative-knowledge semantic conflict shape)
- Tightened `negative_knowledge_prior_guided_semantic_conflict_caution_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `XCTRL` semantic-conflict observations at both canonical stages: visual-caption valid-like evidence remains paired with VLM timing-annotation ready-like evidence.
- This locks negative-knowledge caution memory as conflict-preserving guidance without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided actor-taxonomy graph directions)
- Tightened `actor_taxonomy_prior_guided_section_direction_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts per-signal graph-backed direction coverage for `XADDR`, `XCMD`, and `XRESP` at both canonical stages when learned `Issuer` and `Acceptor` actor priors are staged.
- This locks the actor-taxonomy positive path as exact graph-direction coverage without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for actor-taxonomy family-mismatch graph directions)
- Tightened `actor_taxonomy_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts per-signal graph-backed direction absence for `XADDR` at both canonical stages when an APB-scoped actor-taxonomy prior is staged against an AXI-local `Issuer signals` heading.
- This locks the actor-taxonomy protocol-family mismatch guard as exact graph-direction absence without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior actor-taxonomy graph directions)
- Tightened `actor_taxonomy_prior_guided_section_direction_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts per-signal graph-backed direction absence for `XADDR`, `XCMD`, and `XRESP` at both canonical stages while preserving the local signal inventory.
- This locks the no-prior actor-taxonomy section-heading mirror as exact graph-direction absence without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for table-shape family-mismatch provenance)
- Tightened `table_shape_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts zero table-signal declaration provenance at `EvidenceIR` when an APB-scoped table-shape prior is staged against an AXI-local unknown signal-table shape.
- This locks the protocol-family mismatch guard at the earliest table-shape recovery boundary without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided table-shape signal provenance)
- Tightened `table_shape_prior_guided_signal_table_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `XREQ`/`XACK` table-signal declaration provenance from `table_0001` at `EvidenceIR` and canonical table-support carry-through at both later stages.
- This locks the prior-guided table-shape signal-table path as exact provenance and support preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior table-shape signal inventory)
- Tightened `table_shape_prior_guided_signal_table_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts zero table-signal declaration provenance at `EvidenceIR` and excludes `XREQ`/`XACK` from canonical signal inventory at both later stages when no learned table-shape prior is staged.
- This locks the no-prior table-shape signal-table path as exact provenance and inventory absence without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided visual-motif signal shape)
- Tightened `visual_motif_prior_guided_diagram_classification_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact table-derived `XREQ` output direction at both canonical stages while prior-guided motif classification makes the local diagram normative without creating semantic role evidence.
- This locks visual-motif positive recovery as exact local signal-shape preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior visual-motif signal shape)
- Tightened `visual_motif_prior_guided_diagram_classification_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact table-derived `XREQ` output direction at both canonical stages while the absent-prior visual motif remains ambiguous and non-semantic.
- This locks the visual-motif no-prior mirror as exact local signal-shape preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided visual semantic signal shape)
- Tightened `visual_semantic_prior_guided_caption_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XACK` input direction at both canonical stages while visual-caption prior-guided recovery resolves `handshake_ready_like` with `single_source` grounding.
- This locks the positive visual-caption semantic phrase path as both role-exact and signal-shape exact without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior visual semantic signal shape)
- Tightened `visual_semantic_prior_guided_caption_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XACK` input direction at both canonical stages while the absent-prior visual-caption path stays semantically unresolved.
- This locks the visual-caption no-prior mirror as exact local signal-shape preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided receive signal shape)
- Tightened `semantic_prior_guided_phrase_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XACK` input direction at both canonical stages while prior-guided recovery resolves `handshake_ready_like` with `single_source` grounding.
- This locks the positive `can receive the transfer` path as both role-exact and signal-shape exact without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided ready-sink signal shape)
- Tightened `semantic_ready_sink_prior_guided_phrase_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XACK` input direction at both canonical stages while prior-guided recovery resolves `handshake_ready_like` with `single_source` grounding.
- This locks the positive `can sink the transfer` path as both role-exact and signal-shape exact without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for prior-guided valid-like signal shape)
- Tightened `semantic_valid_prior_guided_phrase_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XREQ` output direction at both canonical stages while prior-guided recovery resolves `handshake_valid_like` with `single_source` grounding.
- This locks the positive `can publish the beat` path as both role-exact and signal-shape exact without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior valid-like signal shape)
- Tightened `semantic_valid_prior_guided_phrase_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XREQ` output direction at both canonical stages while the absent-prior valid-like path stays semantically unresolved.
- This locks the `can publish the beat` no-prior mirror as exact local signal-shape preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior ready-sink signal shape)
- Tightened `semantic_ready_sink_prior_guided_phrase_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XACK` input direction at both canonical stages while the absent-prior ready-sink path stays semantically unresolved.
- This locks the `can sink the transfer` no-prior mirror as exact local signal-shape preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for no-prior semantic phrase signal shape)
- Tightened `semantic_prior_guided_phrase_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XACK` input direction at both canonical stages while the absent-prior path stays semantically unresolved.
- This locks the no-prior mirror for semantic phrase recovery as exact local signal-shape preservation without changing production prior lookup behavior.

## Session update (2026-05-07 KG fixture for conflicting semantic phrase signal shape)
- Tightened `semantic_prior_conflicting_roles_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XCTRL` input direction at both canonical stages while conflicting valid-like and ready-like semantic phrase priors stay silent.
- This locks conflicting-prior fail-closed behavior as exact local signal-shape preservation without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for broad semantic phrase signal shape)
- Tightened `semantic_prior_broad_phrase_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XFLOW` output direction at both canonical stages while the one-token broad semantic phrase prior stays silent.
- This locks broad-prior rejection as exact local signal-shape preservation without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for semantic phrase mismatch signal shape)
- Tightened `semantic_prior_phrase_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XFLOW` output direction at both canonical stages while the unmatched learned semantic phrase prior stays silent.
- This keeps exact-phrase matching conservative without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for semantic phrase source-kind signal shape)
- Tightened `semantic_prior_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XSTAGE` output direction at both canonical stages while the visual-caption-only semantic phrase prior stays silent.
- This keeps source-kind-scoped semantic phrase memory honest without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for semantic phrase family-mismatch signal shape)
- Tightened `semantic_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact local `XREQ` output direction at both canonical stages while the unrelated APB semantic phrase prior stays silent.
- Slice 25 also reran the broader local CI gate; formatting, Clippy warning-deny, `614` Rust tests, rustdoc warning-deny, and mdBook passed.

## Session update (2026-05-07 KG fixture for semantic phrase resolved shape)
- Tightened `semantic_prior_guided_phrase_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `XACK` role and grounding at both canonical stages: `handshake_ready_like` and `single_source`.
- This brings the older `can receive the transfer` semantic phrase prior gold up to the exact role/grounding expectation standard.

## Session update (2026-05-07 KG fixture for AMBA-generic modality fallback shape)
- Tightened `semantic_modality_reliability_amba_generic_fallback_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact AMBA-generic prior-guided `XCTRL` resolution: `handshake_valid_like`, `single_source` grounding strength, and explicit table/prose semantic-conflict observations at both canonical stages.
- This keeps generic prior fallback precise while preserving the current-document grounding boundary.

## Session update (2026-05-07 KG fixture for prior-guided modality resolved shape)
- Tightened `semantic_modality_reliability_prior_guided_conflict_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact prior-guided `XCTRL` resolution: `handshake_valid_like`, `single_source` grounding strength, and explicit table/prose semantic-conflict observations at both canonical stages.
- This documents the intended separation between prior-guided arbitration and current-document grounding strength.

## Session update (2026-05-07 KG fixture for no-prior modality conflict shape)
- Tightened `semantic_modality_reliability_prior_guided_conflict_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The no-prior mirror fixture now asserts exact semantic-conflict observations for `XCTRL`: table-derived valid-like evidence and prose-derived ready-like evidence remain contested at both canonical stages.
- This makes the before side of modality-prior arbitration exact before the prior-guided gold path is hardened further.

## Session update (2026-05-07 KG fixture for weak modality-prior conflict shape)
- Tightened `semantic_modality_reliability_weak_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact semantic-conflict observations for `XCTRL`: table-derived valid-like evidence and prose-derived ready-like evidence remain contested at both canonical stages.
- This locks weak semantic-modality priors as non-decisive exact conflict-shape coverage without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for semantic-modality source-kind mismatch shape)
- Tightened `semantic_modality_reliability_source_kind_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact semantic-conflict observations for `XCTRL`: table-derived valid-like evidence and prose-derived ready-like evidence remain contested at both canonical stages.
- This locks the wrong-source-kind prior guard as exact conflict-shape coverage without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for semantic-modality mismatch conflict shape)
- Tightened `semantic_modality_reliability_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact semantic-conflict observations for `XCTRL`: table-derived valid-like evidence and prose-derived ready-like evidence remain contested at both canonical stages.
- This makes the unrelated APB modality-reliability prior guard exact without changing production prior lookup or arbitration behavior.

## Session update (2026-05-07 KG fixture for negative-knowledge mismatch conflict shape)
- Tightened `negative_knowledge_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact semantic-conflict observations for `XCTRL`: visual-caption valid-like evidence and VLM timing-annotation ready-like evidence remain contested at both canonical stages.
- This keeps the unrelated APB negative-knowledge prior silent while making the local conflict surface exact instead of count-only.

## Session update (2026-05-07 KG fixture for visual-motif family-mismatch shape)
- Tightened `visual_motif_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts table-backed `XREQ` direction/support at both canonical stages while excluding semantic candidates and semantic consensus for the same signal.
- This keeps the APB visual-motif family-mismatch guard exact: unrelated visual memory stays silent without disturbing current-document table truth.

## Session update (2026-05-07 KG fixture for table-shape family-mismatch inventory guard)
- Tightened `table_shape_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts that `XREQ` and `XACK` stay out of canonical signal inventory and graph-backed direction coverage when an AXI-local unknown table is paired with an unrelated APB table-shape prior.
- This is fixture-level false-positive hardening with no production code change; it strengthens the existing direction-count guard into explicit no-inventory/no-graph expectations.

## Session update (2026-05-07 KG fixture for temporal-prior family-mismatch rule shape)
- Tightened `temporal_prior_protocol_family_mismatch_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `temporal_rules_include` shape for the AXI-local `XREADY` family-mismatch guard, including local clock grounding, rising edge, asserted-value predicate, and supporting statement id while the unrelated APB prior still cannot add a cycle window.
- It also excludes clock-grounding replay findings, so the remaining validation guidance is specifically cycle-window debt; this is fixture-level prior-consumption guard hardening with no production code change.

## Session update (2026-05-07 KG fixture for no-prior temporal clock replay separation)
- Tightened `temporal_prior_guided_cycle_window_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now explicitly excludes clock-grounding replay findings while preserving cycle-window replay guidance for the locally clocked no-prior `one beat later` rule.
- This is fixture-level prior-consumption guard hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for prior-guided temporal no-replay)
- Tightened `temporal_prior_guided_cycle_window_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now explicitly excludes cycle-window and clock-grounding replay findings after the prior recovers the one-cycle window and local clock grounding.
- This is fixture-level prior-consumption validator hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for no-prior temporal rule shape)
- Tightened `temporal_prior_guided_cycle_window_without_prior_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `temporal_rules_include` shape for the no-prior `one beat later` negative path, including local clock grounding, rising edge, asserted-value predicate, and supporting statement id while validation keeps cycle-window replay visible.
- This is fixture-level prior-consumption guard hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for prior-guided temporal rule shape)
- Tightened `temporal_prior_guided_cycle_window_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `temporal_rules_include` shape for the prior-guided `one beat later` recovery, including local clock grounding, rising edge, one-cycle window, asserted-value predicate, and supporting statement id.
- This is fixture-level prior-consumption hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for fully grounded temporal no-replay)
- Tightened `temporal_cycle_window_grounded_gold` in `crates/specforge/test_data/kg_quality/`.
- The fixture now explicitly excludes cycle-window, actor-grounding, and clock-grounding replay findings for the fully grounded bounded rule at both canonical validation stages.
- This is fixture-level validator-boundary hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for unbounded temporal rule shape)
- Tightened `temporal_cycle_window_surface_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `temporal_rules_include` shape for the unbounded asserted-value rule, including `PCLK` rising-edge grounding, actor-drive predicate, asserted-value predicate, and supporting statement id while validation proves the missing cycle window remains visible.
- This is fixture-level truthfulness hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for actorless temporal rule shape)
- Tightened `temporal_actor_grounding_surface_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `temporal_rules_include` shape for the actorless bounded asserted-value rule, including `clk` rising-edge grounding, max-cycle window, asserted-value predicate, and supporting statement id.
- This is fixture-level truthfulness hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for clockless temporal rule shape)
- Tightened `temporal_clock_grounding_surface_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now asserts exact `temporal_rules_include` shape for the clockless bounded stability rule, including unknown edge, max-cycle window, actor-stability predicate, signal-stability predicate, and supporting statement id.
- This is fixture-level truthfulness hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for temporal clock-grounding replay lane separation)
- Tightened `temporal_clock_grounding_surface_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now proves a bounded rule with actor grounding emits only clock-grounding replay guidance, not cycle-window or actor-grounding replay findings.
- This completes the current three-way temporal replay lane split at the fixture level with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for temporal actor-grounding replay lane separation)
- Tightened `temporal_actor_grounding_surface_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now proves a bounded rule with clock grounding emits only actor-grounding replay guidance, not cycle-window or clock-grounding replay findings.
- This mirrors the cycle-window lane-separation fixture and remains validator expectation hardening with no production code change; `kg-bench`, corpus-KB projection, and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for temporal grounding replay lane separation)
- Tightened `temporal_cycle_window_surface_negative` in `crates/specforge/test_data/kg_quality/`.
- The fixture now proves an unbounded rule with existing actor and clock grounding emits only cycle-window replay guidance, not actor-grounding or clock-grounding replay findings.
- This is validator expectation hardening with no production code change; `kg-bench` and docs CI passed with `150/150` tracked fixtures.

## Session update (2026-05-07 KG fixture for bounded temporal cycle-window no-rescan)
- Added `temporal_cycle_window_grounded_gold` under `crates/specforge/test_data/kg_quality/`.
- The fixture proves a `within 2 cycles` temporal rule with explicit clock and actor grounding carries its cycle-window and actor-drive predicate through `SemanticIR` and `IntentIR` without emitting cycle-window replay guidance.
- This complements the unbounded-rule negative fixture by locking both sides of the validator boundary; `kg-bench`, corpus-KB fixture projection, and docs CI passed.

## Session update (2026-05-07 KG fixture for temporal cycle-window rescan guidance)
- Added `temporal_cycle_window_surface_negative` under `crates/specforge/test_data/kg_quality/`.
- The fixture proves clock-grounded and actor-grounded temporal rules that still lack explicit cycle-window bounds remain visible through `semantic_temporal_cycle_window_surface_rescan_guidance` and `intent_temporal_cycle_window_surface_rescan_guidance`.
- This is a test-surface hardening slice rather than a production architecture change; `kg-bench`, corpus-KB fixture projection, and docs CI passed, and `cargo sweep --time 1` remains deferred while `target/release/tool_matrix` is active.

## Session update (2026-05-06 `.fsm` duplicate top-port direction renderable block)
- Tightened `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves duplicate/conflicting top-port direction blockers leave no renderable top root and no aggregate `.fsm` source document.
- This locks duplicate public top-port direction evidence against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` recovered top-port renderable block)
- Tightened `top_composition_preserves_recovered_top_port_direction_when_still_blocked` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves recovered top-port direction evidence leaves no renderable top root and no aggregate `.fsm` source document while the child module is still missing.
- This locks topology-recovered top-port direction evidence against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` actor-port parametric-width renderable block)
- Tightened `standalone_dt_blocks_parametric_actor_port_width_with_diagnostic` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed actor-port parametric-width blockers leave no renderable module and no aggregate `.fsm` source document.
- This locks symbolic actor-port width evidence against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` canonical parametric-width renderable block)
- Tightened `standalone_dt_blocks_parametric_signal_width_with_diagnostic` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves canonical parametric signal-width blockers leave no renderable module and no aggregate `.fsm` source document.
- This locks symbolic interface width evidence against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` selector-predicate renderable block)
- Tightened `keeps_selector_based_dt_blocked_when_branch_predicate_is_not_relative_to_selector` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves unsupported selector-branch predicates leave no renderable module and no aggregate `.fsm` source document.
- This locks selector/test-node predicate mismatches against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` reset-polarity renderable block)
- Tightened `keeps_reset_polarity_blocked_when_signal_name_cannot_preserve_it` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves reset-polarity blockers leave no renderable module and no aggregate `.fsm` source document.
- This locks reset-name polarity mismatches against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` missing system-contract renderable block)
- Tightened `keeps_standalone_sequential_dt_blocked_without_system_contract` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves missing standalone sequential system contracts leave no renderable module and no aggregate `.fsm` source document.
- This locks absent clock/reset system surfaces against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` system-contract width renderable block)
- Tightened `standalone_sequential_dt_blocks_conflicting_system_contract_signal_width` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves standalone sequential clock/reset width conflicts leave no renderable module and no aggregate `.fsm` source document.
- This locks conflicting system-contract width evidence against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` system-contract direction renderable block)
- Tightened `standalone_sequential_dt_blocks_conflicting_system_contract_signal_direction` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves standalone sequential clock/reset direction conflicts leave no renderable module and no aggregate `.fsm` source document.
- This locks conflicting system-contract direction evidence against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` system-contract flat/graph renderable block)
- Tightened `standalone_sequential_dt_blocks_system_contract_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves standalone sequential clock/reset flat/graph direction disagreements leave no renderable module and no aggregate `.fsm` source document.
- This locks system-contract clock/reset direction disagreements against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct flat/graph renderable block)
- Tightened `standalone_dt_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves direct DT flat/graph direction disagreements leave no renderable module and no aggregate `.fsm` source document.
- This locks canonical-vs-graph direction disagreements against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct flat-conflict renderable block)
- Tightened `standalone_dt_blocks_conflicting_flat_direction_even_with_actor_graph` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves direct DT flat-direction conflicts leave no renderable module and no aggregate `.fsm` source document even when actor graph evidence exists.
- This locks conflicting canonical direction evidence against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct width-conflict renderable block)
- Tightened `standalone_dt_blocks_conflicting_control_input_actor_port_widths` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves direct DT actor-port width conflicts leave no renderable module and no aggregate `.fsm` source document.
- This locks conflicting control-input width evidence against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` blocked DT-centric renderable block)
- Tightened `builds_blocked_dt_centric_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves baseline blocked DT-centric adapters leave no renderable module and no aggregate `.fsm` source document.
- This locks unresolved render-critical signal roles against stale renderable DT output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` structured undriven-output renderable block)
- Tightened `structured_fsm_blocks_graph_backed_undriven_output_inventory` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed structured-FSM undriven output blockers leave the aggregate `.fsm` artifact without a renderable source document.
- This locks graph-backed structured-FSM undriven outputs against stale renderable FSM output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` structured flat/graph renderable block)
- Tightened `structured_fsm_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves structured-FSM flat/graph direction disagreements leave the aggregate `.fsm` artifact without a renderable source document.
- This locks structured-FSM flat/graph direction disagreements against stale renderable FSM output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module flat/graph renderable block)
- Tightened `standalone_explicit_module_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves explicit-module flat/graph direction disagreements leave the module without a renderable module and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks explicit-module flat/graph direction disagreements against stale renderable module output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module control-read direction renderable block)
- Tightened `standalone_explicit_module_blocks_conflicting_module_control_read_direction` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed module control-read direction conflicts leave the module without a renderable module and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks explicit-module control-read direction conflicts against stale renderable module output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module width conflict renderable block)
- Tightened `standalone_explicit_module_blocks_conflicting_control_input_actor_port_widths` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed actor-port width conflicts in standalone explicit modules leave the module without a renderable module and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks explicit-module width conflicts against stale renderable module output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undeclared target renderable block)
- Tightened `keeps_structured_fsm_blocked_when_transition_target_is_undeclared` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves structured FSMs with a transition targeting an undeclared state leave the aggregate `.fsm` artifact without a renderable source document.
- This locks undeclared-target state-graph blockers against stale renderable FSM output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` missing initial state renderable block)
- Tightened `keeps_structured_fsm_blocked_without_exactly_one_initial_state` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves structured FSMs without exactly one explicit initial state leave the aggregate `.fsm` artifact without a renderable source document.
- This locks missing-initial state-graph blockers against stale renderable FSM output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` missing child module renderable block)
- Tightened `keeps_top_composition_blocked_when_child_module_is_missing` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves top compositions that reference an undeclared child module leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks missing child-module references against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child actor-port direction renderable block)
- Tightened `top_composition_blocks_conflicting_actor_port_directions` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed child actor-port direction conflicts leave the child module without a renderable module and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks child actor-port direction conflicts against stale child module output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child-link direction renderable block)
- Tightened `top_composition_blocks_conflicting_child_link_topology_directions` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves conflicting child-link topology direction evidence leaves the child module without a renderable module and leaves the aggregate `.fsm` artifact without a renderable source document.
- This locks child-link direction conflicts against stale child module output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-link width mismatch renderable block)
- Tightened `top_composition_blocks_width_mismatched_top_link_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves first-slice top links with incompatible endpoint widths leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks width-mismatched top-boundary links against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top actor direction renderable block)
- Tightened `top_composition_blocks_conflicting_top_actor_port_direction` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed top actor-port direction conflicts leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks conflicting top-boundary actor-port direction evidence against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` dedicated child role renderable block)
- Tightened the dedicated `top_composition_blocks_child_source_direction_role_guidance` and `top_composition_blocks_child_target_direction_role_guidance` tests in `crates/specforge/src/ir/adapters.rs`.
- These tests now prove child-side source and target endpoint direction-role mismatches leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This anchors the child-side role blocker against stale renderable top output in the child endpoint tests themselves; focused child-source and child-target coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child target role renderable block)
- Tightened `top_composition_blocks_child_target_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves child target endpoint direction-role mismatches leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the child-target direction-role blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child source role renderable block)
- Tightened `top_composition_blocks_child_source_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves child source endpoint direction-role mismatches leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the child-source direction-role blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` unemitted child target renderable block)
- Tightened `top_composition_blocks_link_to_unemitted_child_target_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves target-side top links to child endpoints absent from emitted child modules leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the target-side unemitted child endpoint blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` source endpoint renderable block)
- Tightened `top_composition_blocks_link_from_undeclared_top_source_guidance` and `top_composition_blocks_link_from_unemitted_child_source_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The tests now prove source-side top-link endpoint blockers leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks undeclared top-boundary source ports and unemitted child source ports against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undeclared top source renderable block)
- Tightened `top_composition_blocks_link_from_undeclared_top_source_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves top links originating from undeclared top-boundary ports leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the missing source endpoint blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undeclared top target renderable block)
- Tightened `top_composition_blocks_link_to_undeclared_top_target_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves top links targeting undeclared top-boundary ports leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the missing target endpoint blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` duplicate child renderable block)
- Tightened `top_composition_blocks_duplicate_child_instance_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves duplicate explicit child-instance records leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the duplicate-child composition blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-without-port renderable block)
- Tightened `top_composition_blocks_top_without_port_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves explicit tops with child-module references but no top-port records leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the missing-top-port composition blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-without-child renderable block)
- Tightened `top_composition_blocks_top_without_child_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves explicit tops with ports but no child-module references leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the missing-child composition blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` multi-child no-link renderable block)
- Tightened `top_composition_blocks_multi_child_without_links_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves multi-child tops without explicit link records leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the no-link composition blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` widthless top-port renderable block)
- Tightened `top_composition_blocks_widthless_top_port_without_width_recovery` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves public top ports without numeric width recovery leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the widthless top-boundary blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` parametric top-width renderable block)
- Tightened `top_composition_blocks_parametric_top_port_width_for_fsm_public_io` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves unresolved parametric public top-port widths leave the selected top without a renderable top root and leave the aggregate `.fsm` artifact without a renderable source document.
- This locks the parametric top-boundary blocker against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top width-conflict renderable-block anchor)
- Tightened `top_composition_blocks_conflicting_top_port_widths_from_child_links` in `crates/specforge/src/ir/adapters.rs`.
- The child-link top-width conflict test now directly proves blocked public top-port width disagreement leaves no renderable top root and no aggregate renderable source document.
- The live analysis also records the actor-port top-width conflict renderable-block assertion already present in the adapter suite; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top child-link width conflict renderable block)
- Tightened `top_composition_blocks_conflicting_top_port_widths_from_child_links` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves conflicting child-link width evidence for a public top port leaves the selected top without a renderable top root and leaves the aggregate `.fsm` artifact without a renderable source document.
- This locks the blocked top child-link width path against stale renderable top output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child topology-width conflict renderable block)
- Tightened `top_composition_blocks_conflicting_child_topology_widths` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves conflicting direct child/topology width evidence leaves the producer child without a renderable module and leaves the aggregate `.fsm` artifact without a renderable source document.
- This locks the blocked child topology-width path against stale renderable child output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` sibling child-width conflict renderable block)
- Tightened `top_composition_blocks_conflicting_sibling_child_link_widths` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves conflicting sibling child-link width evidence leaves the consumer child without a renderable module and leaves the aggregate `.fsm` artifact without a renderable source document.
- This locks the blocked child-width path against stale renderable child output; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` transitive child-width size-entry lock)
- Tightened `top_composition_recovers_child_width_through_transitive_topology` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves producer and consumer child renderable `.fsm` size entries consume graph-recovered widths propagated through top-link and sibling-link topology while retaining output/input directions.
- This locks the downstream child size-entry consumers for transitive topology width recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` source sibling child-width size-entry lock)
- Tightened `top_composition_recovers_source_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves the producer child renderable `.fsm` size entry for `output_data` consumes graph-recovered sibling child-link width while retaining output direction.
- This locks the downstream child size-entry consumer for source-side sibling child-link width recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` sibling child-width size-entry lock)
- Tightened `top_composition_recovers_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves the consumer child renderable `.fsm` size entry for `input_data` consumes graph-recovered sibling child-link width while retaining input direction.
- This locks the downstream child size-entry consumer for sibling child-link width recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-link child-width size-entry lock)
- Tightened `top_composition_recovers_child_width_from_top_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves the producer child renderable `.fsm` size entry for `output_data` consumes graph-recovered top-link width while retaining output direction.
- This locks the downstream child size-entry consumer for top-link width recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top child topology direction size-entry lock)
- Tightened `top_composition_recovers_child_directions_from_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves child renderable `.fsm` size entries consume graph-recovered link-topology directions for producer output and consumer input/output ports when flat child-module `direction_hint` values are absent.
- This locks the downstream child size-entry consumer for top-composition link-topology direction recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top child actor direction size-entry lock)
- Tightened `top_composition_recovers_child_directions_from_actor_ports` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves child renderable `.fsm` size entries consume graph-recovered actor-port directions for producer output and consumer input/output ports when flat child-module `direction_hint` values are absent.
- This locks the downstream child size-entry consumer for top-composition actor-port direction recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module control-input width size-entry lock)
- Tightened `standalone_explicit_module_recovers_control_input_width_from_actor_port_graph` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves the explicit-module renderable `.fsm` size entry for `DATA_IN` consumes graph-recovered actor-port width together with module-control input direction when flat module-local `direction_hint` is absent.
- This locks the downstream size-entry consumer for explicit-module control-input width recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct control-input width size-entry lock)
- Tightened `standalone_dt_recovers_control_input_width_from_actor_port_graph` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves the direct-DT renderable `.fsm` size entry for `DATA_IN` consumes graph-recovered actor-port width together with target-actor input direction when flat direct-interface `direction_hint` is absent.
- This locks the downstream size-entry consumer for direct control-input width recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct control graph size-entry lock)
- Tightened `standalone_dt_derives_target_inputs_from_control_reads_after_output_actor_selection` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves the direct-DT renderable `.fsm` size entry for `DATA_IN` consumes graph-recovered target-actor input direction when flat direct-interface `direction_hint` is absent.
- This locks the direct-DT downstream size-entry consumer of graph-backed control-read recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module graph size-entry lock)
- Tightened `standalone_explicit_module_recovers_inputs_from_module_control_reads` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-recovered module-local input directions are consumed by renderable `.fsm` size entries for `DATA_IN`, `GO`, and `DONE` when flat module-local `direction_hint` values are absent.
- This locks the explicit-module downstream size-entry consumer of graph-backed module-control-read recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` graph-backed FSM size-entry direction lock)
- Tightened `structured_fsm_derives_guard_inputs_from_control_reads_after_output_actor_selection` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-recovered target-actor input directions are consumed by renderable `.fsm` size entries for `DATA_IN`, `GO`, and `DONE` when flat direct-interface `direction_hint` values are absent.
- This locks the structured-FSM downstream size-entry consumer of graph-backed control-read recovery; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` graph-backed DT size-entry direction lock)
- Tightened `standalone_dt_recovers_directions_from_unambiguous_actor_ports` in `crates/specforge/src/ir/adapters.rs`.
- The test now proves graph-backed actor-port directions are consumed by renderable `.fsm` size entries for `DATA_IN`, `DATA_OUT`, and `ZERO_FLAG` when flat direct-interface `direction_hint` values are absent.
- This locks the downstream size-entry consumer of `preferred_signal_direction_hint(...)` in the standalone DT path; focused adapter coverage, full adapter coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 NLP alias source-label prefix filter)
- Added a bounded source-layout label guard to `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs`.
- Form 2 alias learning now rejects subjects prefixed by labels such as `Table 3:`, `Figure 4.2`, and `Section 3.1`, preventing source-region layout text from entering `signal_alias_map`.
- Added `extract_alias_phrase_rejects_source_label_prefixes`, bringing focused alias parser coverage to `10` tests and NLP-enrich module coverage to `22` tests; focused parser coverage, module coverage, fmt, docs CI, KG bench, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` unemitted child source guidance residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_link_from_unemitted_child_source_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Source-side child endpoints absent from emitted child modules now prove source-endpoint enrichment guidance, top-port/child/link provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair context stay visible together.
- The live tracker and roadmap now mark unemitted child source guidance residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child actor-port direction residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_actor_port_directions` in `crates/specforge/src/ir/adapters.rs`.
- Child actor-port direction conflicts now prove actor-relative graph-direction repair guidance, explicit child signal and graph actor-port support IDs, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair context stay visible together.
- The live tracker and roadmap now mark child actor-port direction residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child topology direction residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_child_link_topology_directions` in `crates/specforge/src/ir/adapters.rs`.
- Child link-topology direction conflicts now prove actor-relative graph-direction repair guidance, contradictory topology-link support IDs, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair context stay visible together.
- The live tracker and roadmap now mark child topology direction residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-link width mismatch residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_width_mismatched_top_link_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top-link endpoint width mismatches now prove width-compatible top-link repair guidance, mismatched topology-link support IDs, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair context stay visible together.
- The live tracker and roadmap now mark top-link width mismatch residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top child-link width residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_top_port_widths_from_child_links` in `crates/specforge/src/ir/adapters.rs`.
- Top child-link width conflicts now prove top-boundary width-conflict repair guidance, both conflicting topology-link support-ID sets, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair context stay visible together.
- The live tracker and roadmap now mark top child-link width residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` sibling child-link width residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_sibling_child_link_widths` in `crates/specforge/src/ir/adapters.rs`.
- Sibling child-link width conflicts now prove canonical width-conflict repair guidance, both conflicting topology-link support-ID sets, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair context stay visible together.
- The live tracker and roadmap now mark sibling child-link width residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child topology width residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_child_topology_widths` in `crates/specforge/src/ir/adapters.rs`.
- Child topology width conflicts now prove width-conflict repair guidance, signal-declaration and topology-link support IDs, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair context stay visible together.
- The live tracker and roadmap now mark child topology width residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undeclared top-source residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_link_from_undeclared_top_source_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top links originating from undeclared top-boundary ports now prove source-endpoint enrichment guidance, top-port/child/link provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair context stay visible together.
- The live tracker and roadmap now mark undeclared top-source residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undeclared top-target residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_link_to_undeclared_top_target_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top links targeting undeclared top-boundary ports now prove target-endpoint enrichment guidance, top-port/child/link provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair context stay visible together.
- The live tracker and roadmap now mark undeclared top-target residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` duplicate child-instance residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_duplicate_child_instance_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Duplicate child-instance top blockers now prove deduplication guidance, both child declaration support-ID sets, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and child-module reference context stay visible together.
- The live tracker and roadmap now mark duplicate child-instance residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` no-top-port residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_top_without_port_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top roots without explicit top-port records now prove top-port enrichment guidance, child declaration provenance, resolved child root kind, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair guidance stay visible together.
- The live tracker and roadmap now mark no-top-port residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` no-child top residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_top_without_child_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top roots without child-module references now prove child-module enrichment guidance, declared top-port provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and child-module reference repair guidance stay visible together.
- The live tracker and roadmap now mark no-child top residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` multi-child no-link residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_multi_child_without_links_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Multi-child top roots without explicit top-link records now prove top-link enrichment guidance, child declaration provenance, resolved child root kinds, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and width-compatible-link repair guidance stay visible together.
- The live tracker and roadmap now mark multi-child no-link residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` duplicate top-width residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_keeps_duplicate_top_port_width_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Duplicate top-boundary width blockers now prove duplicate declaration blocking, top-boundary width-conflict guidance, duplicate support-ID sets, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair guidance stay visible together.
- The live tracker and roadmap now mark duplicate top-width residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and the user-requested safe `cargo sweep --time 1` passed.

## Session update (2026-05-06 `.fsm` duplicate top-direction residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Duplicate top-boundary direction blockers now prove duplicate declaration blocking, top-boundary direction-conflict guidance, duplicate support-ID sets, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair guidance stay visible together.
- The live tracker and roadmap now mark duplicate top-direction residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` missing-child residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `keeps_top_composition_blocked_when_child_module_is_missing` in `crates/specforge/src/ir/adapters.rs`.
- Primary missing-child top-composition blockers now prove child-source declaration guidance, missing-module blocker text, top-port/child provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and child-module-reference repair guidance stay visible together.
- The live tracker and roadmap now mark missing-child residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child target role residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_child_target_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Target-side child endpoint direction-role blockers now prove child-module flat/graph direction-role guidance, top and aggregate endpoint-role guidance, top-port/child/link provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair guidance stay visible together.
- The live tracker and roadmap now mark target-side child role residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` child source role residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_child_source_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Source-side child endpoint direction-role blockers now prove child-module graph-direction role guidance, top and aggregate endpoint-role guidance, top-port/child/link provenance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair guidance stay visible together.
- The live tracker and roadmap now mark source-side child role residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-target role residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_top_target_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Public top targets with conflicting boundary direction evidence now prove boundary direction-conflict guidance, endpoint-role guidance, selected top signal-inventory evidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair guidance stay visible together.
- The live tracker and roadmap now mark top-target role residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top-link direction residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_keeps_conflicting_top_port_direction_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Top-link-derived top-boundary direction conflicts now prove boundary direction-conflict guidance, endpoint-role guidance, top-port/topology-link support and confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and both repair interpretations stay visible together.
- The live tracker and roadmap now mark top-link direction residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top actor width residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_top_actor_port_width` in `crates/specforge/src/ir/adapters.rs`.
- Top actor-port width conflicts now prove top-boundary width-conflict guidance, top-port and graph actor-port support/confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair guidance stay visible together.
- The live tracker and roadmap now mark top actor-port width residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` top actor direction residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_conflicting_top_actor_port_direction` in `crates/specforge/src/ir/adapters.rs`.
- Top actor-port direction conflicts now prove top-boundary direction-conflict guidance, top-port and graph actor-port support/confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-port repair guidance stay visible together.
- The live tracker and roadmap now mark top actor-port direction residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` recovered top-port direction residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_preserves_recovered_top_port_direction_when_still_blocked` in `crates/specforge/src/ir/adapters.rs`.
- Blocked recovered top-port direction cases now prove topology-recovered direction provenance, child-source declaration guidance, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and child-module repair guidance stay visible together.
- The live tracker and roadmap now mark recovered top-port direction residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` target unemitted child residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_link_to_unemitted_child_target_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Target-side unemitted child endpoint blockers now prove target-endpoint repair guidance, preserved top-link support/confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair guidance stay visible together.
- The live tracker and roadmap now mark target-side unemitted child residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` source unemitted child residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_link_from_unemitted_child_source_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Source-side unemitted child endpoint blockers now prove source-endpoint repair guidance, preserved top-link support/confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and renderable-child repair guidance stay visible together.
- The live tracker and roadmap now mark source-side unemitted child residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` parametric top-port residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_parametric_top_port_width_for_fsm_public_io` in `crates/specforge/src/ir/adapters.rs`.
- Parametric public top-port blockers now prove symbolic-width blocker text, top-boundary width resolution guidance, preserved parametric width support/confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-ports repair guidance stay visible together.
- The live tracker and roadmap now mark parametric top-port residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` widthless top-port residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_composition_blocks_widthless_top_port_without_width_recovery` in `crates/specforge/src/ir/adapters.rs`.
- Widthless top-port blockers now prove missing-width blocker text, top-boundary width recovery guidance, preserved top-port support/confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and explicit-top-ports repair guidance stay visible together.
- The live tracker and roadmap now mark widthless top-port residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` recovered top-root residual diagnostics)
- Added composition residual confidence and blocker-text assertions to `top_root_kind_confidence_follows_recovered_top_port_evidence` in `crates/specforge/src/ir/adapters.rs`.
- Recovered top-port evidence now proves graph-backed support, high selected top-root confidence, low-confidence `fsm_adapter_composition_topology` diagnostics, topology-detail blocker text, and child-module repair guidance stay visible together.
- The live tracker and roadmap now mark recovered top-root residual diagnostics as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` blocked DT signal-inventory guidance)
- Added signal-inventory residual-guidance assertions to `builds_blocked_dt_centric_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs`.
- Baseline blocked DT-centric adapters now prove blocked lowering status, absent target emission, DT root-kind selection, selected signal inventory, low-confidence `fsm_adapter_signal_inventory` guidance, and the upstream interface-inventory enrichment candidate stay visible together.
- The live tracker and roadmap now mark blocked DT-centric signal-inventory residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` reset-polarity residual guidance)
- Added system-contract residual-guidance assertions to `keeps_reset_polarity_blocked_when_signal_name_cannot_preserve_it` in `crates/specforge/src/ir/adapters.rs`.
- Reset-polarity blockers now prove reset signal name, active-low polarity, system-contract support IDs, high system confidence, exact reset-name blocker text, low-confidence `fsm_adapter_system_contract` guidance, and the upstream system-surface enrichment candidate stay visible together.
- The live tracker and roadmap now mark reset-polarity residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` missing system-contract residual guidance)
- Added system-contract residual-guidance assertions to `keeps_standalone_sequential_dt_blocked_without_system_contract` in `crates/specforge/src/ir/adapters.rs`.
- Missing clock/reset system contracts now prove DT control-fragment support IDs, referenced signal inventory, high DT candidate confidence, missing-contract blocker text, low-confidence `fsm_adapter_system_contract` guidance, and the upstream system-surface enrichment candidate stay visible together.
- The live tracker and roadmap now mark missing-system-contract residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` system-contract flat/graph residual guidance)
- Added system-contract residual-guidance assertions to `standalone_sequential_dt_blocks_system_contract_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- Clock flat/graph direction disagreements now prove conflicting system/actor-port direction evidence, graph support IDs, high signal confidence, exact flat/graph conflict text, low-confidence `fsm_adapter_system_contract` guidance, and the upstream system-surface enrichment candidate stay visible together.
- The live tracker and roadmap now mark system-contract flat/graph residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` system-contract width residual guidance)
- Added system-contract residual-guidance assertions to `standalone_sequential_dt_blocks_conflicting_system_contract_signal_width` in `crates/specforge/src/ir/adapters.rs`.
- Clock width conflicts now prove system-contract support IDs, high signal confidence, exact clock-width conflict text, low-confidence `fsm_adapter_system_contract` guidance, and the upstream system-surface enrichment candidate stay visible together.
- The live tracker and roadmap now mark system-contract width residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` system-contract direction residual guidance)
- Added system-contract residual-guidance assertions to `standalone_sequential_dt_blocks_conflicting_system_contract_signal_direction` in `crates/specforge/src/ir/adapters.rs`.
- Clock direction conflicts now prove system-contract support IDs, high signal confidence, exact clock-direction conflict text, low-confidence `fsm_adapter_system_contract` guidance, and the upstream system-surface enrichment candidate stay visible together.
- The live tracker and roadmap now mark system-contract direction residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module width residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_explicit_module_blocks_conflicting_control_input_actor_port_widths` in `crates/specforge/src/ir/adapters.rs`.
- Explicit-module actor-port width conflicts now prove both graph-backed width support IDs, high automation confidence, blocked module/root renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark explicit-module width-conflict residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module control-read residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_explicit_module_blocks_conflicting_module_control_read_direction` in `crates/specforge/src/ir/adapters.rs`.
- Explicit-module control-read direction conflicts now prove actor-port/control-read provenance categories, unresolved conflicting graph direction, blocked module/root renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark explicit-module control-read conflict residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` explicit-module flat/graph residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_explicit_module_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- Explicit-module flat/graph direction disagreements now prove module interface direction, graph-backed actor direction, graph support provenance, high inventory confidence, blocked renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark explicit-module flat/graph residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` structured flat/graph residual guidance)
- Added signal-inventory residual-guidance assertions to `structured_fsm_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- Structured-FSM flat/graph direction disagreements now prove interface direction, graph-backed actor direction, graph support provenance, high inventory confidence, blocked renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark structured-FSM flat/graph residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undeclared-transition residual guidance)
- Added state-graph residual-guidance assertions to `keeps_structured_fsm_blocked_when_transition_target_is_undeclared` in `crates/specforge/src/ir/adapters.rs`.
- Undeclared-target structured-FSM blockers now prove state support IDs, rejected-transition support IDs, high candidate confidence, exact undeclared-target blocking text, low-confidence `fsm_adapter_state_graph` guidance, and the upstream state-graph enrichment candidate stay visible together.
- The live tracker and roadmap now mark undeclared-transition residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` missing-initial residual guidance)
- Added state-graph residual-guidance assertions to `keeps_structured_fsm_blocked_without_exactly_one_initial_state` in `crates/specforge/src/ir/adapters.rs`.
- Missing-initial structured-FSM blockers now prove state/transition support IDs, high candidate confidence, exact missing-initial blocking text, low-confidence `fsm_adapter_state_graph` guidance, and the upstream state-graph enrichment candidate stay visible together.
- The live tracker and roadmap now mark missing-initial residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` structured undriven residual guidance)
- Added signal-inventory residual-guidance assertions to `structured_fsm_blocks_graph_backed_undriven_output_inventory` in `crates/specforge/src/ir/adapters.rs`.
- Structured-FSM graph-backed undriven output blockers now prove selected actor support IDs, high inventory confidence, exact FSM-state undriven-output blocking text, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark structured-FSM undriven-output residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` undriven output residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_dt_blocks_graph_backed_undriven_output_inventory` in `crates/specforge/src/ir/adapters.rs`.
- Standalone graph-backed undriven output blockers now prove selected actor support IDs, high inventory confidence, exact undriven-output blocking text, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark standalone undriven-output residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` flat-graph residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_dt_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs`.
- Direct-root flat/graph direction disagreements now prove interface direction, graph-backed actor direction, graph support provenance, high inventory confidence, blocked renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible together.
- The live tracker and roadmap now mark direct-root flat/graph disagreement residual guidance as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` conflict residual guidance alignment)
- Aligned blocked direct-root conflict residual guidance in `crates/specforge/src/ir/adapters.rs`.
- `standalone_dt_keeps_conflicting_actor_port_width_unresolved` now carries the width-conflict `fsm_adapter_signal_inventory` guidance assertion directly, while `standalone_dt_blocks_conflicting_flat_direction_even_with_actor_graph` keeps an accurately named flat-direction conflict guidance assertion.
- The live tracker and roadmap now mark direct-root flat direction conflict residual guidance as closed; focused flat-direction and width-conflict coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` actor width conflict residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_dt_keeps_conflicting_actor_port_width_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Same-signal actor-port width conflicts now prove width remains unresolved while conflicting width support IDs, high inventory confidence, blocked renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible.
- The live tracker and roadmap now mark the actor-port width conflict residual-guidance guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` actor direction conflict residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_dt_keeps_conflicting_actor_port_direction_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Same-signal actor-port direction conflicts now prove graph direction remains unresolved while conflicting support IDs, high inventory confidence, blocked renderability, and low-confidence `fsm_adapter_signal_inventory` repair guidance stay visible.
- The live tracker and roadmap now mark the actor-port direction conflict residual-guidance guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` ambiguous actor residual guidance)
- Added signal-inventory residual-guidance assertions to `standalone_dt_ignores_ambiguous_actor_port_context` in `crates/specforge/src/ir/adapters.rs`.
- Ambiguous direct actor-port graph context now proves graph direction/category/support evidence stays unselected while the blocked adapter retains low-confidence `fsm_adapter_signal_inventory` repair guidance and the upstream interface-inventory enrichment candidate.
- The live tracker and roadmap now mark the ambiguous actor-port residual-guidance guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` unrelated actor context residual cleanliness)
- Added a signal-inventory residual-clean assertion to `standalone_dt_ignores_unrelated_actor_ports_for_graph_context` in `crates/specforge/src/ir/adapters.rs`.
- Unrelated actor-port graph context now proves side-band graph evidence stays excluded without leaving a stale `fsm_adapter_signal_inventory` packet once the standalone DT root is renderable.
- The live tracker and roadmap now mark the unrelated actor-port context residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct actor-width residual cleanliness)
- Added a signal-inventory residual-clean assertion to `standalone_dt_recovers_control_input_width_from_actor_port_graph` in `crates/specforge/src/ir/adapters.rs`.
- Direct actor-port width recovery now proves recovered graph-backed control-input width evidence leaves no stale `fsm_adapter_signal_inventory` packet once the standalone DT root is renderable.
- The live tracker and roadmap now mark the direct actor-port width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct control-read residual cleanliness)
- Added a signal-inventory residual-clean assertion to `standalone_dt_derives_target_inputs_from_control_reads_after_output_actor_selection` in `crates/specforge/src/ir/adapters.rs`.
- Direct control-read input recovery now proves recovered graph-backed read inputs leave no stale `fsm_adapter_signal_inventory` packet once the standalone DT root is renderable.
- The live tracker and roadmap now mark the direct control-read input recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` output actor selection residual cleanliness)
- Added a signal-inventory residual-clean assertion to `standalone_dt_selects_output_actor_when_external_actors_share_signals` in `crates/specforge/src/ir/adapters.rs`.
- Target-actor-backed direct output selection now proves selected output graph evidence leaves no stale `fsm_adapter_signal_inventory` packet once the standalone DT root is renderable.
- The live tracker and roadmap now mark the output actor selection residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` direct actor direction residual cleanliness)
- Added a signal-inventory residual-clean assertion to `standalone_dt_recovers_directions_from_unambiguous_actor_ports` in `crates/specforge/src/ir/adapters.rs`.
- Direct-root actor-port direction recovery now proves graph-backed direct signal directions leave no stale `fsm_adapter_signal_inventory` packet once the standalone DT root is renderable.
- The live tracker and roadmap now mark the direct actor-port direction recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` transitive child-width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_child_width_through_transitive_topology` in `crates/specforge/src/ir/adapters.rs`.
- Transitive child-width recovery now proves topology-backed producer and consumer child signal width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the transitive child-width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, and full CI passed, with `cargo sweep --time 1` deferred until no target-tree process is active.

## Session update (2026-05-06 `.fsm` source-side sibling width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_source_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- Source-side sibling child-width recovery now proves topology-backed producer child signal width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the source-side sibling child-width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` sibling-link child-input width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- Sibling-link child-input width recovery now proves topology-backed consumer child signal width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the sibling-link child-input width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top-link child-width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_child_width_from_top_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- Top-link child-width recovery now proves topology-backed child signal width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the top-link child-width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child link-direction residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_child_directions_from_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- Child link-topology direction recovery now proves topology-backed child signal direction recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the child link-topology direction recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child actor direction residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_child_directions_from_actor_ports` in `crates/specforge/src/ir/adapters.rs`.
- Child actor-port direction recovery now proves graph-backed child signal direction recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the child actor-port direction recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child system-contract width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_top_system_port_widths_from_child_system_contract` in `crates/specforge/src/ir/adapters.rs`.
- Child system-contract top-width recovery now proves clock/reset public top-port width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the child system-contract top-width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child-link top-width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_top_port_width_from_child_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- Child-link top-width recovery now proves topology-backed public top-port width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the child-link top-width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` actor-port width residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_top_port_width_from_actor_ports` in `crates/specforge/src/ir/adapters.rs`.
- Actor-port width recovery now proves graph-backed public top-port width recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the actor-port width recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` actor-port direction residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_top_port_direction_from_actor_ports` in `crates/specforge/src/ir/adapters.rs`.
- Actor-port direction recovery now proves graph-backed public top-port direction recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the actor-port direction recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top-link direction residual cleanliness)
- Added a residual-clean assertion to `top_composition_recovers_top_port_direction_from_link_topology` in `crates/specforge/src/ir/adapters.rs`.
- Top-link direction recovery now proves topology-backed public top-port direction recovery leaves no stale `fsm_adapter_composition_topology` packet once the top is renderable.
- The live tracker and roadmap now mark the top-link direction recovery residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` mixed-child residual cleanliness)
- Added a residual-clean assertion to `renderable_top_document_preserves_mixed_child_root_order_and_kind` in `crates/specforge/src/ir/adapters.rs`.
- Mixed DT/FSM child top documents now prove the combined `?dtc` / `?fsmc` source-document path leaves no stale `fsm_adapter_composition_topology` packet once renderable.
- The live tracker and roadmap now mark the mixed-child residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` reused-FSM-child residual cleanliness)
- Added a residual-clean assertion to `renderable_top_document_deduplicates_reused_fsm_child_roots` in `crates/specforge/src/ir/adapters.rs`.
- Reused-FSM-child top documents now prove the shared FSM direct-root de-duplication path leaves no stale `fsm_adapter_composition_topology` packet once the source document is renderable.
- The live tracker and roadmap now mark the reused-FSM-child residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` single-FSM-child residual cleanliness)
- Added a residual-clean assertion to `renderable_top_document_preserves_fsm_child_root_kind` in `crates/specforge/src/ir/adapters.rs`.
- Single-FSM-child top documents now prove the typed `?fsmc` child-reference path leaves no stale `fsm_adapter_composition_topology` packet once the source document is renderable.
- The live tracker and roadmap now mark the single-FSM-child residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top-before-child residual cleanliness)
- Added a residual-clean assertion to `renderable_top_document_emits_top_before_child_direct_roots` in `crates/specforge/src/ir/adapters.rs`.
- Top-first source-document emission now proves its renderable `?top` before child-root output path leaves no stale `fsm_adapter_composition_topology` packet.
- The live tracker and roadmap now mark the top-before-child residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` reused-child residual cleanliness)
- Added a residual-clean assertion to `renderable_top_document_deduplicates_reused_child_module_roots` in `crates/specforge/src/ir/adapters.rs`.
- Reused-child top documents now prove the shared child-root de-duplication path leaves no stale `fsm_adapter_composition_topology` packet once the `?top` document is renderable.
- The live tracker and roadmap now mark the reused-child residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top-link confidence residual cleanliness)
- Added aggregate renderability and residual-clean assertions to `top_root_kind_confidence_follows_top_link_evidence` in `crates/specforge/src/ir/adapters.rs`.
- Top-link evidence now raises selected top-root confidence without leaving `fsm_adapter_composition_topology` behind when the linked top is renderable.
- The live tracker and roadmap now mark the top-link confidence residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child-confidence residual cleanliness)
- Added renderability and residual-clean assertions to `top_root_kind_confidence_follows_child_declaration_evidence` in `crates/specforge/src/ir/adapters.rs`.
- Child-declaration evidence now raises selected top-root confidence without leaving `fsm_adapter_composition_topology` behind when the single-child top is renderable.
- The live tracker and roadmap now mark the child-declaration confidence residual-clean guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` recovered top-root confidence residual lock)
- Added a composition residual interpretation assertion to `top_root_kind_confidence_follows_recovered_top_port_evidence` in `crates/specforge/src/ir/adapters.rs`.
- Recovered top-port evidence now keeps child-module repair visible in `fsm_adapter_composition_topology` while raising selected top-root confidence.
- The live tracker and roadmap now mark the recovered top-root confidence residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` blocked recovered top residual lock)
- Added a composition residual interpretation assertion to `top_composition_preserves_recovered_top_port_direction_when_still_blocked` in `crates/specforge/src/ir/adapters.rs`.
- Blocked recovered top-port direction cases now keep child-module repair visible in `fsm_adapter_composition_topology` while retaining topology-recovered direction provenance.
- The live tracker and roadmap now mark the blocked recovered top-port residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` missing-child residual lock)
- Added a composition residual interpretation assertion to `keeps_top_composition_blocked_when_child_module_is_missing` in `crates/specforge/src/ir/adapters.rs`.
- Missing child-module reference blockers now keep child-module repair visible in `fsm_adapter_composition_topology` while retaining existing child-source declaration guidance.
- The live tracker and roadmap now mark the missing-child residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` unemitted child endpoint guidance alignment)
- Renamed the legacy unemitted child-port composition regression in `crates/specforge/src/ir/adapters.rs` so its source-side endpoint blocker is explicit in the test name.
- Added a top-root selection assertion to keep that regression anchored to explicit top composition.
- The live tracker, roadmap, and mdBook now describe unemitted child endpoint blockers as preserving source- and target-side repair guidance; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` parametric top-port residual lock)
- Added a composition residual assertion to `top_composition_blocks_parametric_top_port_width_for_fsm_public_io` in `crates/specforge/src/ir/adapters.rs`.
- Parametric public top-port blockers now keep explicit-top-port repair visible in `fsm_adapter_composition_topology` while retaining existing top-boundary width-resolution guidance.
- The live tracker and roadmap now mark the parametric top-port residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` widthless top-port residual lock)
- Added a composition residual assertion to `top_composition_blocks_widthless_top_port_without_width_recovery` in `crates/specforge/src/ir/adapters.rs`.
- Widthless public top-port blockers now keep explicit-top-port repair visible in `fsm_adapter_composition_topology` while retaining existing top-boundary width-recovery guidance.
- The live tracker and roadmap now mark the widthless top-port residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child actor-port direction residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_actor_port_directions` in `crates/specforge/src/ir/adapters.rs`.
- Child actor-port direction conflicts now keep renderable-child repair visible in `fsm_adapter_composition_topology` while retaining existing child-module graph-direction conflict enrichment guidance.
- The live tracker and roadmap now mark the child actor-port direction residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child topology direction residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_child_link_topology_directions` in `crates/specforge/src/ir/adapters.rs`.
- Child topology direction conflicts now keep renderable-child repair visible in `fsm_adapter_composition_topology` while retaining existing child-module graph-direction conflict enrichment guidance.
- The live tracker and roadmap now mark the child topology direction residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` child topology width residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_child_topology_widths` in `crates/specforge/src/ir/adapters.rs`.
- Child topology width conflicts now keep renderable-child repair visible in `fsm_adapter_composition_topology` while retaining existing child-module canonical width-conflict enrichment guidance.
- The live tracker and roadmap now mark the child topology width residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` sibling child-link width residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_sibling_child_link_widths` in `crates/specforge/src/ir/adapters.rs`.
- Sibling child-link width conflicts now keep renderable-child repair visible in `fsm_adapter_composition_topology` while retaining existing child-module canonical width-conflict enrichment guidance.
- The live tracker and roadmap now mark the sibling child-link width residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top child-link width residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_top_port_widths_from_child_links` in `crates/specforge/src/ir/adapters.rs`.
- Top child-link width conflicts now keep explicit top-port repair visible in `fsm_adapter_composition_topology` while retaining existing top-boundary width-conflict enrichment guidance.
- The live tracker and roadmap now mark the top child-link width residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top-link direction residual alignment)
- Extended `top_composition_keeps_conflicting_top_port_direction_unresolved` in `crates/specforge/src/ir/adapters.rs` so the existing composition residual also checks explicit top-port repair.
- Top-link direction conflicts now keep both renderable-child repair and explicit-top-port repair visible in `fsm_adapter_composition_topology` while retaining existing direction-conflict and endpoint-role guidance.
- The live tracker and roadmap now mark the top-link direction conflict residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top actor-width residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_top_actor_port_width` in `crates/specforge/src/ir/adapters.rs`.
- Top actor-port width conflicts now keep explicit top-port repair visible in `fsm_adapter_composition_topology` while retaining existing top-boundary width-conflict enrichment guidance.
- The live tracker and roadmap now mark the top actor-port width residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top actor-direction residual lock)
- Added a composition residual assertion to `top_composition_blocks_conflicting_top_actor_port_direction` in `crates/specforge/src/ir/adapters.rs`.
- Top actor-port direction conflicts now keep explicit top-port repair visible in `fsm_adapter_composition_topology` while retaining existing top-boundary direction-conflict enrichment guidance.
- The live tracker and roadmap now mark the top actor-port direction residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` duplicate top-width residual lock)
- Added a composition residual assertion to `top_composition_keeps_duplicate_top_port_width_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Duplicate top-port width blockers now keep explicit top-port repair visible in `fsm_adapter_composition_topology` while retaining existing deduplication and width-conflict enrichment guidance.
- The live tracker and roadmap now mark the duplicate top-width residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` duplicate top-direction residual lock)
- Added a composition residual assertion to `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs`.
- Duplicate top-port direction blockers now keep explicit top-port repair visible in `fsm_adapter_composition_topology` while retaining existing deduplication and direction-conflict enrichment guidance.
- The live tracker and roadmap now mark the duplicate top-direction residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` duplicate child-instance residual lock)
- Added a composition residual assertion to `top_composition_blocks_duplicate_child_instance_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Duplicate top child-instance blockers now keep child-module reference context visible in `fsm_adapter_composition_topology` while retaining existing deduplication enrichment guidance.
- The live tracker and roadmap now mark the duplicate child-instance residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` no-top-port residual lock)
- Added a composition residual assertion to `top_composition_blocks_top_without_port_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top roots that omit explicit top-port records now keep explicit top-port repair visible in `fsm_adapter_composition_topology` while retaining existing renderability enrichment guidance.
- The live tracker and roadmap now mark the no-top-port residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` no-child residual lock)
- Added a composition residual assertion to `top_composition_blocks_top_without_child_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Top roots that omit explicit child-module references now keep child-module reference repair visible in `fsm_adapter_composition_topology` while retaining existing renderability enrichment guidance.
- The live tracker and roadmap now mark the no-child top residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` multi-child no-link residual lock)
- Added a composition residual assertion to `top_composition_blocks_multi_child_without_links_guidance` in `crates/specforge/src/ir/adapters.rs`.
- Multi-child top roots that omit explicit top-link records now keep width-compatible top-link repair visible in `fsm_adapter_composition_topology` while retaining existing renderability enrichment guidance.
- The live tracker and roadmap now mark the multi-child no-link residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` top-source residual alignment)
- Added composition residual assertions to `top_composition_blocks_link_from_undeclared_top_source_guidance` and `top_composition_blocks_link_from_unemitted_child_source_guidance` in `crates/specforge/src/ir/adapters.rs`.
- The undeclared top-source blocker now checks explicit-top-port repair, and the unemitted child-source blocker now checks renderable-child repair in its own dedicated regression.
- The live tracker and roadmap now mark the undeclared top-source residual guarantee as closed while the child-source residual guarantee is backed by the named source regression; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` undeclared top-target residual alignment)
- Aligned `top_composition_blocks_link_to_undeclared_top_target_guidance` in `crates/specforge/src/ir/adapters.rs` so its composition residual assertion names explicit-top-port repair instead of child-module repair.
- Added the missing dedicated residual assertion to `top_composition_blocks_link_to_unemitted_child_target_guidance`, where renderable-child repair is the correct interpretation.
- The live tracker and roadmap now mark the undeclared top-target residual guarantee as closed while the child-target residual guarantee is backed by its own regression; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` unemitted child-source residual lock)
- Tightened `top_composition_blocks_link_from_unemitted_child_source_guidance` in `crates/specforge/src/ir/adapters.rs` so source-side links from child endpoints absent from emitted child modules explicitly preserve `fsm_adapter_composition_topology`.
- This mirrors the target-side residual lock and keeps renderable-child repair visible alongside source-endpoint guidance, support IDs, and high automation confidence.
- The live tracker and roadmap now mark the source-side unemitted child residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` unemitted child-target residual lock)
- Tightened `top_composition_blocks_link_to_unemitted_child_target_guidance` in `crates/specforge/src/ir/adapters.rs` so target-side links to child endpoints absent from emitted child modules explicitly preserve `fsm_adapter_composition_topology`.
- The assertion keeps repair tied to renderable child modules while the same blocked top-link artifact also preserves target-endpoint guidance, support IDs, and high automation confidence.
- The live tracker and roadmap now mark the target-side unemitted child residual guarantee as closed; focused adapter coverage, full adapter coverage, KG bench, docs, full CI, and `cargo sweep --time 1` all passed.

## Session update (2026-05-06 `.fsm` unemitted child-target confidence alignment)
- Tightened `top_composition_blocks_link_to_unemitted_child_target_guidance` in `crates/specforge/src/ir/adapters.rs` so target-side links to child endpoints absent from emitted child modules explicitly preserve high automation confidence.
- This matches the source-side unemitted child endpoint regression and keeps the live tracker's support/confidence claim backed on both source and target topology blockers.
- The slice is regression-only: blocked renderability behavior and emitted `.fsm` output remain unchanged; after rebuilding the stale target cache, focused adapter coverage, full adapter coverage, KG bench, docs, and full CI all passed.

## Session update (2026-05-06 `.fsm` child role residual assertion alignment)
- Added the missing `fsm_adapter_composition_topology` assertions to `top_composition_blocks_child_source_direction_role_guidance` and `top_composition_blocks_child_target_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs`.
- This keeps the dedicated child endpoint role regressions aligned with the live tracker rows for source/target composition residual coverage.
- The public top-boundary source/target residual assertions remain in place, so both public-boundary and child-endpoint role blockers now carry structured composition diagnostics.

## Session update (2026-05-06 `.fsm` top-link target-role residual lock)
- Tightened `top_composition_blocks_child_target_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs` so target-side top-link direction-role blockers also prove `fsm_adapter_composition_topology` is emitted.
- The regression mirrors the source-side residual lock and checks that the structured interpretation keeps renderable-child repair visible while endpoint-role enrichment stays on child, top, and aggregate renderability.
- This completes symmetric source/target residual coverage for top-link endpoint role blockers.

## Session update (2026-05-06 `.fsm` top-link role residual lock)
- Tightened `top_composition_blocks_child_source_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs` so source-side top-link direction-role blockers also prove `fsm_adapter_composition_topology` is emitted.
- The regression now checks the residual interpretation that keeps upstream repair tied to renderable child modules while endpoint-role enrichment remains visible on child, top, and aggregate renderability.
- This keeps top-link role mismatches visible through both renderability enrichments and structured residual decisions.

## Session update (2026-05-06 `.fsm` top-link width residual lock)
- Tightened `top_composition_blocks_width_mismatched_top_link_guidance` in `crates/specforge/src/ir/adapters.rs` so width-mismatched top links also prove the adapter emits `fsm_adapter_composition_topology`.
- The regression now checks the residual interpretation that tells upstream canonicalization to carry width-compatible top links with explicit top ports, child-module references, and renderable children.
- This keeps the width-compatible link blocker visible through both renderability enrichment and the structured residual-decision channel.

## Session update (2026-05-06 `.fsm` top-link width guidance lock)
- Added `top_composition_blocks_width_mismatched_top_link_guidance` in `crates/specforge/src/ir/adapters.rs` so explicit top links with incompatible endpoint widths preserve width-compatible repair guidance.
- The regression proves the blocked top candidate and aggregate `.fsm` renderability carry `keep first-slice top-link endpoints width-compatible` while retaining topology-link support IDs and high automation confidence.
- This separates the explicit top-link compatibility gate from child/top canonical width-conflict blockers and keeps R6 focused on honest `?top:name` emission boundaries.

## Session update (2026-05-06 `.fsm` top-target boundary role guidance lock)
- Added `top_composition_blocks_top_target_direction_role_guidance` in `crates/specforge/src/ir/adapters.rs` so a top-boundary target declared as input, while topology needs output, preserves endpoint-role repair guidance.
- The regression pairs the top-target conflict with the top-source boundary lock and keeps direction-conflict guidance on both blocked top and aggregate `.fsm` renderability.
- Top-port declaration provenance, topology-link support IDs, and selected top signal-inventory evidence remain visible for review.

## Session update (2026-05-06 `.fsm` top-boundary role guidance lock)
- Extended top-port direction conflict handling in `crates/specforge/src/ir/adapters.rs` so conflicts caused by explicit top-link topology also preserve endpoint-role repair guidance.
- Tightened `top_composition_keeps_conflicting_top_port_direction_unresolved` to prove the new role guidance survives beside top-boundary direction-conflict guidance on blocked top and aggregate `.fsm` renderability.
- The regression continues to keep top-port declaration provenance, topology-link support IDs, and selected top signal-inventory evidence visible.

## Session update (2026-05-06 `.fsm` top-link target-role guidance lock)
- Extended topology-backed flat/graph direction disagreement handling in `crates/specforge/src/ir/adapters.rs` so top-link endpoint-role guidance survives when explicit child direction and link topology disagree.
- Added `top_composition_blocks_child_target_direction_role_guidance` to prove a child target declared as an output, while used as a top-link target, keeps role repair guidance on the blocked child module, blocked top candidate, and aggregate `.fsm` renderability.
- The regression also keeps declared top-port provenance, child declaration provenance, and topology-link support IDs visible for review.

## Session update (2026-05-06 `.fsm` top-link direction-role guidance)
- Added shared top-link endpoint direction-role guidance in `crates/specforge/src/ir/adapters.rs` so source/target role mismatches now produce canonical repair advice instead of only blocking text.
- Added `top_composition_blocks_child_source_direction_role_guidance` to prove a child source declared as an input, while used as a top-link source, keeps the role repair guidance on the blocked child module, blocked top candidate, and aggregate `.fsm` renderability.
- The regression also keeps declared top-port provenance, child declaration provenance, and topology-link support IDs visible for review.

## Session update (2026-05-06 `.fsm` unemitted child-source guidance lock)
- Added `top_composition_blocks_link_from_unemitted_child_source_guidance` in `crates/specforge/src/ir/adapters.rs` so top links originating from child endpoints absent from renderable child modules retain source-endpoint enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves declared top-port provenance, producer child declaration provenance, high-confidence topology-link state, and top-link support IDs survive while the producer module resolves to a renderable DT root.
- This gives the source-side child endpoint blocker the same provenance contract as the child-target blocker.

## Session update (2026-05-06 `.fsm` unemitted child-target guidance lock)
- Added `top_composition_blocks_link_to_unemitted_child_target_guidance` in `crates/specforge/src/ir/adapters.rs` so top links targeting child endpoints absent from renderable child modules retain target-endpoint enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves declared top-port provenance, consumer child declaration provenance, and top-link support IDs survive while the child module resolves to a renderable DT root.
- This pairs target-side child endpoint validation with the existing child-source endpoint blocker.

## Session update (2026-05-06 `.fsm` undeclared top-source guidance lock)
- Added `top_composition_blocks_link_from_undeclared_top_source_guidance` in `crates/specforge/src/ir/adapters.rs` so top links originating from undeclared top-boundary ports retain source-endpoint enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves declared top-port provenance, child declaration provenance, and top-link support IDs survive while the child module resolves to a renderable DT root.
- This pairs the undeclared top-source and top-target blockers so both public-boundary endpoint directions require canonical top-port declarations.

## Session update (2026-05-06 `.fsm` undeclared top-target guidance lock)
- Added `top_composition_blocks_link_to_undeclared_top_target_guidance` in `crates/specforge/src/ir/adapters.rs` so top links targeting undeclared top-boundary ports retain target-endpoint enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves declared top-port provenance, child declaration provenance, and top-link support IDs survive while the child module resolves to a renderable DT root.
- This closes another composition wiring blocker where the adapter must require canonical top-boundary declarations instead of creating public IO from a link alone.

## Session update (2026-05-06 `.fsm` no-top-port guidance lock)
- Added `top_composition_blocks_top_without_port_guidance` in `crates/specforge/src/ir/adapters.rs` so explicit top roots without top-port records retain top-port enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves the declared child remains visible with support IDs, high automation confidence, and a resolved DT root kind while no top port is invented.
- This completes the basic explicit-top shape guidance trio: no children, no top ports, and multi-child/no-link composition blockers.

## Session update (2026-05-06 `.fsm` no-child top guidance lock)
- Added `top_composition_blocks_top_without_child_guidance` in `crates/specforge/src/ir/adapters.rs` so explicit top roots without child-module references retain child-module enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves the surviving top port keeps direction, numeric width, support IDs, and high automation confidence while no child candidate is fabricated.
- This complements the multi-child no-link blocker by locking both missing-composition-shape gates before `.fsm` `?top:name` emission.

## Session update (2026-05-06 `.fsm` multi-child top-link guidance lock)
- Added `top_composition_blocks_multi_child_without_links_guidance` in `crates/specforge/src/ir/adapters.rs` so multi-child top roots with no explicit top-link records retain top-link enrichment guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves producer/consumer child declaration support IDs survive, both child modules resolve to DT roots, and the adapter blocks rather than inferring composition wiring.
- This locks the composition-topology blocker that keeps first-slice `?top:name` lowering honest for multi-child designs.

## Session update (2026-05-06 `.fsm` duplicate top-width dedup guidance lock)
- Tightened `top_composition_keeps_duplicate_top_port_width_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs` so duplicate top-port width blockers retain explicit top-port deduplication guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover width-conflict repair guidance, both duplicate declaration support-ID sets, unresolved recovered width, selected signal-inventory width conflict state, and high automation confidence.
- This completes the duplicate top-port deduplication guidance pair across direction and width blockers.

## Session update (2026-05-06 `.fsm` duplicate top-direction dedup guidance lock)
- Tightened `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs` so duplicate top-port direction blockers retain explicit top-port deduplication guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover direction-conflict repair guidance, both duplicate declaration support-ID sets, unresolved recovered direction, selected signal-inventory provenance, and high automation confidence.
- This keeps the duplicate top-port direction blocker actionable as both a duplicate-declaration repair lane and a direction-conflict repair lane.

## Session update (2026-05-06 `.fsm` duplicate child-instance guidance lock)
- Added `top_composition_blocks_duplicate_child_instance_guidance` in `crates/specforge/src/ir/adapters.rs` so duplicate top child instance blockers retain child-instance deduplication guidance on the blocked top candidate and aggregate `.fsm` renderability.
- The regression also proves both duplicate child declarations keep distinct support IDs while the resolved child root kind and source module remain visible.
- This extends the blocked top-composition guidance surface from top-port and missing-child blockers into duplicate child-instance declarations.

## Session update (2026-05-06 `.fsm` primary missing-child guidance lock)
- Tightened `keeps_top_composition_blocked_when_child_module_is_missing` in `crates/specforge/src/ir/adapters.rs` so the primary missing child module blocker retains child-source declaration guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover declared `result_data` top-port provenance, missing child declaration support IDs, high automation confidence, and the precise missing-module blocker.
- This pairs the dedicated missing-child composition regression with the recovered-top-direction missing-child guidance lock.

## Session update (2026-05-06 `.fsm` missing-child guidance lock)
- Tightened `top_composition_preserves_recovered_top_port_direction_when_still_blocked` in `crates/specforge/src/ir/adapters.rs` so missing child module blockers retain child-source declaration guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover recovered `result_data` top-boundary direction, topology-link support IDs, signal-inventory graph provenance, high automation confidence, and the precise missing-module blocker.
- This keeps recovered top-boundary evidence reviewable while the separate missing-child declaration lane remains actionable.

## Session update (2026-05-06 `.fsm` unemitted child-link aggregate guidance lock)
- Tightened `top_composition_blocks_link_to_unemitted_child_port` in `crates/specforge/src/ir/adapters.rs` so top links to child endpoints that are not emitted retain source-endpoint repair guidance on aggregate `.fsm` renderability.
- Existing assertions still cover the blocked top candidate guidance, explicit top-link support IDs, high automation confidence, and the precise unemitted-child-port diagnostic.
- This keeps the aggregate blocked artifact actionable when the referenced endpoint is present in topology evidence but absent from the emitted child module surface.

## Session update (2026-05-06 `.fsm` parametric top-port guidance lock)
- Tightened `top_composition_blocks_parametric_top_port_width_for_fsm_public_io` in `crates/specforge/src/ir/adapters.rs` so symbolic public IO width blockers retain parametric-width resolution guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover explicit top-port support IDs, high automation confidence, and selected top signal-inventory `parametric_width_hint`.
- This keeps symbolic top-width blockers actionable without confusing them with missing numeric width or width-conflict repair lanes.

## Session update (2026-05-06 `.fsm` widthless top-port guidance lock)
- Tightened `top_composition_blocks_widthless_top_port_without_width_recovery` in `crates/specforge/src/ir/adapters.rs` so missing numeric public IO width blockers retain top-boundary width recovery guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover explicit top-port support IDs, high automation confidence, and selected top signal-inventory missing-width state.
- This keeps widthless public top-port blockers actionable without confusing them with parametric-width or width-conflict repair lanes.

## Session update (2026-05-06 `.fsm` duplicate top-direction blocker guidance lock)
- Tightened `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs` so duplicate top-port direction conflicts retain top-boundary direction repair guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover both duplicate declaration support-ID sets and high automation confidence on duplicate recovered top ports and selected top signal-inventory entries.
- This completes the top-boundary direction guidance trio for actor-port graph conflicts, top-link topology conflicts, and duplicate declaration conflicts.

## Session update (2026-05-06 `.fsm` top-link direction blocker guidance lock)
- Tightened `top_composition_keeps_conflicting_top_port_direction_unresolved` in `crates/specforge/src/ir/adapters.rs` so top-link direction conflicts retain top-boundary direction repair guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover explicit top-port support IDs, topology-link support IDs, high automation confidence, and selected top signal-inventory graph-direction disagreement.
- This proves topology-derived top-boundary direction conflicts expose the same actionable repair guidance as graph actor-port conflicts.

## Session update (2026-05-06 `.fsm` top actor-direction blocker guidance lock)
- Tightened `top_composition_blocks_conflicting_top_actor_port_direction` in `crates/specforge/src/ir/adapters.rs` so actor-port graph direction conflicts retain top-boundary direction repair guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover explicit top-port support IDs, graph actor-port support ID, high automation confidence, and selected top signal-inventory direction disagreement.
- This begins the top-boundary direction guidance lane by proving graph actor-port conflicts preserve actionable repair guidance alongside their evidence trail.

## Session update (2026-05-06 `.fsm` top-boundary width blocker guidance lock)
- Tightened `top_composition_keeps_duplicate_top_port_width_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs` so duplicate top-port width conflicts retain top-boundary width repair guidance on the blocked top candidate and aggregate `.fsm` renderability.
- Tightened `top_composition_blocks_conflicting_top_port_widths_from_child_links` so child-link-derived top-port width conflicts carry the same repair guidance, and rechecked the top actor-port width blocker still covers that guidance surface.
- This aligns the top-boundary width blocker family: actor-port graph evidence, duplicate declarations, and child-link topology evidence now all preserve actionable repair guidance alongside their provenance and width-conflict inventory state.

## Session update (2026-05-06 `.fsm` top child-width blocker guidance lock)
- Tightened `top_composition_blocks_conflicting_top_port_widths_from_child_links` in `crates/specforge/src/ir/adapters.rs` so blocked top-boundary child-link width conflicts retain top-port width repair guidance on the selected top candidate and aggregate `.fsm` renderability.
- Existing assertions still cover both conflicting child-link support-ID sets, high automation confidence, and selected top signal-inventory width-conflict state.
- This closes the current top-boundary sibling of the child-module width guidance locks.

## Session update (2026-05-06 `.fsm` sibling child-width blocker guidance lock)
- Tightened `top_composition_blocks_conflicting_sibling_child_link_widths` in `crates/specforge/src/ir/adapters.rs` so blocked sibling child-link width conflicts retain canonical width-conflict enrichment guidance on the blocked module candidate and aggregate `.fsm` renderability.
- Existing assertions still cover both conflicting topology-link support-ID sets, high automation confidence, and width-conflict inventory state.
- This extends aggregate child-module guidance coverage to sibling-link-propagated child width blockers.

## Session update (2026-05-06 `.fsm` child topology-width blocker guidance lock)
- Tightened `top_composition_blocks_conflicting_child_topology_widths` in `crates/specforge/src/ir/adapters.rs` so blocked child topology width conflicts retain canonical width-conflict enrichment guidance on the blocked module candidate and aggregate `.fsm` renderability.
- Existing assertions still cover child declaration support IDs, topology-link support IDs, high automation confidence, and width-conflict inventory state.
- This proves the aggregate child-module guidance propagation also covers width blockers, not only graph-direction blockers.

## Session update (2026-05-06 `.fsm` child topology-direction blocker guidance lock)
- Tightened `top_composition_blocks_conflicting_child_link_topology_directions` in `crates/specforge/src/ir/adapters.rs` so blocked child topology-link direction conflicts retain actor-relative graph-direction enrichment guidance on the blocked module candidate and aggregate `.fsm` renderability.
- Existing assertions still cover child declaration support IDs, contradictory topology-link support IDs, high automation confidence, and conflicted graph-direction inventory state.
- This mirrors the actor-port conflict guidance lock for topology-derived child graph-direction conflicts.

## Session update (2026-05-06 `.fsm` child actor-direction blocker guidance lock)
- `analyze_top_renderability(...)` now propagates required-enrichment guidance from blocked explicit child modules into the aggregate top renderability surface.
- Tightened `top_composition_blocks_conflicting_actor_port_directions` in `crates/specforge/src/ir/adapters.rs` so blocked child actor-port direction conflicts retain actor-relative graph-direction enrichment guidance on the blocked module candidate and aggregate `.fsm` renderability.
- Existing assertions still cover child declaration support IDs, graph actor-port support IDs, high automation confidence, and conflicted graph-direction inventory state.
- This complements the child actor-direction provenance locks by proving the blocked artifact also preserves actionable repair guidance.

## Session update (2026-05-06 `.fsm` child actor-direction provenance lock)
- Tightened `top_composition_recovers_child_directions_from_actor_ports` in `crates/specforge/src/ir/adapters.rs` so actor-port-recovered child signal directions keep original child signal declaration support IDs in selected child module inventories.
- Existing assertions still cover actor-port support IDs, high automation confidence, graph-backed child signal-inventory provenance, and emitted topology text.
- This pairs with child link-topology direction recovery so both child direction recovery sources preserve declaration provenance.

## Session update (2026-05-06 `.fsm` child topology-direction provenance lock)
- Tightened `top_composition_recovers_child_directions_from_link_topology` in `crates/specforge/src/ir/adapters.rs` so topology-recovered child signal directions keep original child signal declaration support IDs in selected child module inventories.
- Existing assertions still cover topology-link support IDs, high automation confidence, topology-backed child signal-inventory provenance, and emitted topology text.
- This aligns child link-topology direction recovery with the child-width declaration-provenance locks.

## Session update (2026-05-06 `.fsm` transitive child-width provenance lock)
- Tightened `top_composition_recovers_child_width_through_transitive_topology` in `crates/specforge/src/ir/adapters.rs` so transitive topology-recovered child signal widths keep the original producer and consumer child signal declaration support IDs in selected child module inventories.
- Existing assertions still cover both contributing topology-link support IDs, high automation confidence, topology-backed child signal-inventory provenance, and emitted `(output_data 8)` / `(input_data 8)` text.
- This closes the direct and transitive child-width declaration-provenance set.

## Session update (2026-05-06 `.fsm` source child-width provenance lock)
- Tightened `top_composition_recovers_source_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs` so source-side sibling-link-recovered child signal width keeps the original child signal declaration support IDs in the selected child module inventory.
- Existing assertions still cover topology-link support IDs, high automation confidence, topology-backed child signal-inventory provenance, and emitted `(output_data 8)` text.
- This completes direct sibling-link child-width declaration coverage for both recovered input and output child signals.

## Session update (2026-05-06 `.fsm` sibling child-width provenance lock)
- Tightened `top_composition_recovers_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs` so sibling-link-recovered child signal width keeps the original child signal declaration support IDs in the selected child module inventory.
- Existing assertions still cover topology-link support IDs, high automation confidence, topology-backed child signal-inventory provenance, and emitted `(input_data 8)` text.
- This pairs with top-link child-width recovery so direct topology width recovery paths preserve child declaration provenance.

## Session update (2026-05-06 `.fsm` top-link child-width provenance lock)
- Tightened `top_composition_recovers_child_width_from_top_link_topology` in `crates/specforge/src/ir/adapters.rs` so top-link-recovered child signal width keeps the original child signal declaration support IDs in the selected child module inventory.
- Existing assertions still cover topology-link support IDs, high automation confidence, topology-backed child signal-inventory provenance, and emitted `(output_data 8)` text.
- This extends declaration-provenance coverage from public top ports into recovered child module width evidence.

## Session update (2026-05-06 `.fsm` child-system width provenance lock)
- Tightened `top_composition_recovers_top_system_port_widths_from_child_system_contract` in `crates/specforge/src/ir/adapters.rs` so child-system-contract-recovered public clock/reset top-port widths keep the original top-port declaration support IDs in selected and renderable records.
- Existing assertions still cover topology-link support IDs, high automation confidence, child system-contract signal provenance, and emitted clock/reset topology text.
- This complements the graph actor-port and child-link topology width provenance locks for public top ports.

## Session update (2026-05-06 `.fsm` child-link width provenance lock)
- Tightened `top_composition_recovers_top_port_width_from_child_link_topology` in `crates/specforge/src/ir/adapters.rs` so topology-recovered public top-port width keeps the original top-port declaration support IDs in selected and renderable records.
- Existing assertions still cover topology-link width support IDs, high automation confidence, topology-backed signal-inventory provenance, and emitted `result_data>8` text.
- This mirrors the top-link direction recovery provenance lock for the topology-backed public-top width path.

## Session update (2026-05-06 `.fsm` actor-port width provenance lock)
- Tightened `top_composition_recovers_top_port_width_from_actor_ports` in `crates/specforge/src/ir/adapters.rs` so actor-port-recovered public top-port width keeps the original top-port declaration support IDs in selected and renderable records.
- Existing assertions still cover graph actor-port width support IDs, high automation confidence, actor-port-width signal-inventory provenance, and emitted `ext_data>8` text.
- This mirrors the actor-port direction recovery provenance lock for the graph-backed public-top width path.

## Session update (2026-05-06 `.fsm` actor-port direction provenance lock)
- Tightened `top_composition_recovers_top_port_direction_from_actor_ports` in `crates/specforge/src/ir/adapters.rs` so actor-port-recovered public top-port direction keeps the original top-port declaration support IDs in selected and renderable records.
- Existing assertions still cover graph actor-port support IDs, high automation confidence, actor-port signal-inventory provenance, and emitted `ext_data>8` text.
- This mirrors the topology direction recovery provenance lock for the graph-backed public-top direction path.

## Session update (2026-05-06 `.fsm` top-link direction provenance lock)
- Tightened `top_composition_recovers_top_port_direction_from_link_topology` in `crates/specforge/src/ir/adapters.rs` so topology-recovered public top-port direction keeps the original top-port declaration support IDs in selected and renderable records.
- Existing assertions still cover topology-link support IDs, high automation confidence, graph-backed signal-inventory provenance, and emitted `result_data>8` text.
- This locks the recovery contract that topology evidence augments public IO declarations instead of erasing their declaration provenance.

## Session update (2026-05-06 `.fsm` mixed-child top provenance lock)
- Tightened `renderable_top_document_preserves_mixed_child_root_order_and_kind` in `crates/specforge/src/ir/adapters.rs` so mixed DT/FSM top documents keep the public `ACC` top-port support IDs in selected and renderable top records.
- The renderable top root now also proves producer-to-controller data-link and controller-to-top `ACC` link support IDs survive while mixed child root kinds and direct-root order remain unchanged.
- This rounds out the current top-document provenance lane across reused DT children, top-before-child ordering, single FSM children, reused FSM children, and mixed DT/FSM children.

## Session update (2026-05-06 `.fsm` reused-FSM-child top provenance lock)
- Tightened `renderable_top_document_deduplicates_reused_fsm_child_roots` in `crates/specforge/src/ir/adapters.rs` so reused FSM child top documents keep `ACC_A` and `ACC_B` top-port support IDs in selected and renderable top records.
- The renderable top root now also proves both first/second child `ACC` topology-link support-ID sets survive while the shared `controller_core` FSM direct root is still emitted once.
- This mirrors the reused-DT-child top-boundary provenance lock across the reused FSM child-root path.

## Session update (2026-05-06 `.fsm` single-FSM-child top provenance lock)
- Tightened `renderable_top_document_preserves_fsm_child_root_kind` in `crates/specforge/src/ir/adapters.rs` so the single-FSM-child top-document path keeps `ACC` top-port support IDs in both selected and renderable top records.
- The renderable top root now also proves the `controller.ACC -> ACC` topology-link support IDs survive while the FSM child root-kind and direct-root checks remain unchanged.
- This closes the same top-boundary provenance gap for the single-FSM-child source-document fixture after the top-before-child ordering fixture.

## Session update (2026-05-06 `.fsm` top-before-child provenance lock)
- Tightened `renderable_top_document_emits_top_before_child_direct_roots` in `crates/specforge/src/ir/adapters.rs` so top-before-child source-document emission keeps public top-port support IDs in the selected top candidate.
- The renderable top root now also proves both explicit topology-link support-ID sets survive while the existing child support-ID and top-first order guarantees remain intact.
- This keeps source-document ordering coverage tied to the same provenance surface as the other renderable top-composition paths.

## Session update (2026-05-06 `.fsm` reused-child top provenance lock)
- Tightened `renderable_top_document_deduplicates_reused_child_module_roots` in `crates/specforge/src/ir/adapters.rs` so reused-child top composition preserves the public `result_data` top-port shape and support IDs.
- The test now also proves both topology-link support-ID sets survive while the shared `stage_core` direct root is still emitted exactly once.
- This starts the `N=200` batch by expanding renderable top-composition provenance from child deduplication to the top boundary and wiring surface.

## Session update (2026-05-06 `.fsm` renderable top-port support lock)
- Tightened `builds_renderable_top_composition_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable composition artifact retains explicit `result_data` top-port shape and confidence.
- The test now proves output direction, numeric width, high automation confidence, and the renderable top root survive alongside existing child and topology support-ID checks.
- This pairs the blocked missing-child top-port lock with the renderable top-composition path, keeping the public top boundary auditable in both outcomes.

## Session update (2026-05-06 `.fsm` missing-child top-port support lock)
- Tightened `keeps_top_composition_blocked_when_child_module_is_missing` in `crates/specforge/src/ir/adapters.rs` so the blocked composition artifact retains the declared `result_data` top-port surface.
- The test now proves the top port keeps output direction, numeric width, supporting statement IDs, and high automation confidence alongside the unresolved child candidate.
- This makes missing-child composition diagnostics preserve the usable top boundary while still blocking `.fsm` document emission.

## Session update (2026-05-06 `.fsm` undeclared-target state support lock)
- Tightened `keeps_structured_fsm_blocked_when_transition_target_is_undeclared` in `crates/specforge/src/ir/adapters.rs` so the blocked undeclared-target graph retains declared state-candidate provenance for `idle` and `busy`.
- The test now proves state support IDs, initial-state flags, and high automation confidence survive alongside the existing missing-target transition candidate.
- This makes undeclared-transition diagnostics explain both the rejected edge and the declared graph surface that caused it to be rejected.

## Session update (2026-05-06 `.fsm` missing-initial graph support lock)
- Tightened `keeps_structured_fsm_blocked_without_exactly_one_initial_state` in `crates/specforge/src/ir/adapters.rs` so the blocked state graph retains transition-candidate provenance for both `idle -> busy` and `busy -> idle`.
- The test now proves transition support IDs and high automation confidence survive alongside the existing state-candidate provenance and `fsm_adapter_state_graph` residual decision.
- This strengthens blocked structured-FSM diagnostics before any emitted `.fsm` target exists.

## Session update (2026-05-06 `.fsm` reset-block renderable-module support lock)
- Tightened `builds_renderable_structured_fsm_with_reset_blocks` in `crates/specforge/src/ir/adapters.rs` so the reset-block structured-FSM artifact retains auditable renderable-module state coverage for `idle` and `busy`.
- The renderable state bodies now explicitly prove transition actions back and forth while module-level reset blocks preserve both `ResetSynchronous` and `ResetAsynchronous` roles.
- This records the important adapter distinction that this reset-block fixture is audited through its renderable module rather than through top-level state candidates.

## Session update (2026-05-06 `.fsm` structured FSM candidate support lock)
- Tightened `builds_renderable_structured_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable structured-FSM artifact retains named `idle` and `busy` state candidates.
- The transition candidate list now explicitly proves `idle -> busy` and `busy -> idle` survive in the adapter artifact while emitted system, state, transition, and action text remains unchanged.
- This moves the baseline structured-FSM test from count-only candidate checks to named graph coverage.

## Session update (2026-05-06 `.fsm` standalone sequential DT support lock)
- Tightened `builds_renderable_standalone_sequential_dt_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable sequential DT artifact retains the canonical `dt_primary_intent_cone` candidate id.
- The selected signal inventory now explicitly covers `clk`, `rst_n`, `DATA_IN`, and `ACC` while system-contract, init-assignment, size, and sequential assignment emission stay unchanged.
- This gives the clock/reset-backed standalone DT path the same adapter-artifact audit surface as the combinational baseline path.

## Session update (2026-05-06 `.fsm` standalone DT inventory support lock)
- Tightened `builds_renderable_standalone_dt_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the baseline renderable DT artifact retains the canonical `dt_primary_intent_cone` candidate id.
- The selected signal inventory now explicitly covers `DATA_IN`, `DATA_OUT`, and `ZERO_FLAG` while the emitted `.fsm` size and control-block text stays unchanged.
- This anchors the narrower renderable symbolic, selector, computed-selector, and compound-update coverage against the baseline standalone DT path.

## Session update (2026-05-06 `.fsm` symbolic DT support lock)
- Tightened `builds_renderable_symbolic_dt_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable symbolic DT artifact retains one visible decision-tree candidate.
- The selected signal inventory now proves the selector plus symbolic assignment outputs remain visible while constants, defines, params, enums, and symbolic assignments still render unchanged.
- This pins the symbol-definition lowering path to an auditable adapter artifact rather than only checking emitted text.

## Session update (2026-05-06 `.fsm` compound-update support lock)
- Tightened `builds_renderable_compound_update_dt_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable compound-update DT artifact retains a visible decision-tree candidate at low confidence.
- The fixture is renderable through fallback lowering and does not carry canonical control-fragment support IDs, so the regression keeps the confidence contract explicit rather than overstating provenance.
- The selected signal inventory now proves `clk`, `rst_n`, and `ACC` remain visible while emitted `.fsm` reset/init and compound-update shorthand stay unchanged.

## Session update (2026-05-06 `.fsm` computed-selector support lock)
- Tightened `builds_renderable_computed_selector_dt_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable computed-selector DT artifact retains a visible decision-tree candidate at medium confidence.
- The fixture still has no canonical control-fragment support records, so the regression intentionally locks the honest computed-selector shape rather than implying high-confidence support provenance.
- The selected signal inventory now proves the computed selector inputs and action outputs remain visible while emitted `.fsm` text stays renderable.

## Session update (2026-05-06 `.fsm` renderable selector support lock)
- Tightened `builds_renderable_selector_based_dt_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the renderable selector-based DT artifact retains its decision-tree candidate, control-fragment support IDs, single branch block, and high confidence.
- The adapter already emitted selector/test-node `.fsm` text for the supported `MODE == mode_t.idle` shape; this regression now proves the same renderable path keeps its selector and output signal inventory visible.
- This complements the blocked selector-predicate coverage by pinning both renderable and blocked selector outcomes to auditable DT provenance.

## Session update (2026-05-06 `.fsm` selector-branch blocker support lock)
- Tightened `keeps_selector_based_dt_blocked_when_branch_predicate_is_not_relative_to_selector` in `crates/specforge/src/ir/adapters.rs` so the blocked selector-based DT artifact retains its decision-tree candidate, control-fragment support IDs, referenced guard/action signals, and high confidence.
- The adapter already blocked `.fsm` emission when a branch predicate could not be rendered relative to the selector token; this regression now proves the selected control cone remains auditable.
- The selected signal inventory also keeps the selector head and branch guard/action signals visible, covering the unsupported selector-predicate branch without fabricating target syntax.

## Session update (2026-05-06 `.fsm` missing-system-contract DT support lock)
- Tightened `keeps_standalone_sequential_dt_blocked_without_system_contract` in `crates/specforge/src/ir/adapters.rs` so the blocked sequential-DT artifact retains its decision-tree candidate, control-fragment support IDs, referenced signals, and high confidence.
- The adapter already blocked `.fsm` emission when clock/reset/init evidence was missing; this regression now proves the explicit control cone remains auditable.
- This complements system-contract blocker coverage by showing the missing contract does not hide the otherwise usable DT evidence.

## Session update (2026-05-06 `.fsm` reset-polarity blocker support lock)
- Tightened `keeps_reset_polarity_blocked_when_signal_name_cannot_preserve_it` in `crates/specforge/src/ir/adapters.rs` so the blocked sequential-DT artifact retains system-contract support IDs, reset signal name, active-low polarity, and high confidence.
- The adapter already blocked `.fsm` emission when active-low polarity could not be preserved through the reset signal name; this regression now proves the rejected system contract remains auditable.
- This complements the system-contract direction/width blocker locks with the reset-semantics blocker branch.

## Session update (2026-05-06 `.fsm` structured-FSM undeclared-transition support lock)
- Tightened `keeps_structured_fsm_blocked_when_transition_target_is_undeclared` in `crates/specforge/src/ir/adapters.rs` so the blocked FSM-root transition candidate retains source/target state names, declaration support IDs, and high confidence.
- The adapter already blocked `.fsm` emission when a transition target was undeclared; this regression now proves the transition remains visible for review.
- This pairs with the missing-initial blocker lock to cover structured-FSM state-graph cardinality and endpoint blockers.

## Session update (2026-05-06 `.fsm` structured-FSM missing-initial support lock)
- Tightened `keeps_structured_fsm_blocked_without_exactly_one_initial_state` in `crates/specforge/src/ir/adapters.rs` so blocked FSM-root state candidates retain declaration support IDs and high confidence.
- The adapter already blocked `.fsm` emission when the explicit state graph had no initial state; this regression now proves those state declarations remain visible for review.
- This complements structured-FSM control/read and blocker provenance coverage with the state-graph cardinality branch.

## Session update (2026-05-06 `.fsm` missing-child top support lock)
- Tightened `keeps_top_composition_blocked_when_child_module_is_missing` in `crates/specforge/src/ir/adapters.rs` so the blocked top candidate retains the unresolved child declaration support IDs and high confidence.
- The adapter already blocked `.fsm` emission when a top child referenced `missing_module`; this regression now proves the child candidate remains inspectable with no resolved root kind.
- This complements renderable child-declaration support locks by covering the missing-module blocker branch.

## Session update (2026-05-06 `.fsm` child actor-port direction conflict support lock)
- Tightened `top_composition_blocks_conflicting_actor_port_directions` in `crates/specforge/src/ir/adapters.rs` so the blocked producer `output_data` inventory entry retains local child signal declaration support and both conflicting graph actor-port support IDs.
- The adapter already blocked `.fsm` emission when actor-port direction evidence for the child output disagreed; this regression now proves the graph conflict and declaration evidence remain inspectable together.
- This complements the child actor-port direction recovery lock with the blocked conflict path.

## Session update (2026-05-06 `.fsm` child topology direction conflict support lock)
- Tightened `top_composition_blocks_conflicting_child_link_topology_directions` in `crates/specforge/src/ir/adapters.rs` so the blocked producer `output_data` inventory entry retains both contradictory topology-link support-ID sets and the local signal declaration support.
- The adapter already blocked graph-backed direction conflict after module-local direction hints were cleared; this regression now proves the competing topology evidence and declaration evidence remain inspectable.
- This pairs with the child topology width conflict lock so both topology conflict dimensions preserve evidence trails.

## Session update (2026-05-06 `.fsm` child topology width conflict support lock)
- Updated adapter signal-inventory registration in `crates/specforge/src/ir/adapters.rs` so interface signal records contribute their own supporting statement IDs in addition to the enclosing interface id.
- Tightened `top_composition_blocks_conflicting_child_topology_widths` so the blocked producer `output_data` inventory entry retains both explicit signal declaration support and topology-link support when width evidence conflicts.
- This fixes a real provenance gap exposed by the regression: topology conflict overlays must not erase the local declaration evidence they conflict with.

## Session update (2026-05-06 `.fsm` top child-link width conflict support lock)
- Tightened `top_composition_blocks_conflicting_top_port_widths_from_child_links` in `crates/specforge/src/ir/adapters.rs` so the blocked recovered top port and selected `result_data` inventory entry retain both conflicting child-link support-ID sets.
- The adapter already left incompatible child-to-top width evidence unresolved and blocked top renderability; this regression now proves both link evidence records remain auditable.
- This complements the sibling child-link width conflict lock with the public top-port conflict path.

## Session update (2026-05-06 `.fsm` sibling child-link width conflict support lock)
- Tightened `top_composition_blocks_conflicting_sibling_child_link_widths` in `crates/specforge/src/ir/adapters.rs` so the blocked consumer `input_data` inventory entry retains both conflicting topology-link support-ID sets.
- The adapter already left incompatible sibling-link width evidence unresolved and blocked renderability; this regression now proves both conflicting link records remain auditable.
- This starts a blocked topology-width conflict provenance lane after the renderable top-composition child support locks.

## Session update (2026-05-06 `.fsm` top-before-child support lock)
- Tightened `renderable_top_document_emits_top_before_child_direct_roots` in `crates/specforge/src/ir/adapters.rs` so producer and consumer child declarations retain support IDs in the selected top candidate.
- The adapter already emitted renderable source documents with the top root before child direct roots; this regression now proves the same order fixture keeps child evidence auditable.
- This closes a narrow provenance gap in the ordering test without changing adapter lowering behavior.

## Session update (2026-05-06 `.fsm` single FSM-child top support lock)
- Tightened `renderable_top_document_preserves_fsm_child_root_kind` in `crates/specforge/src/ir/adapters.rs` so a single FSM child declaration retains support IDs in the selected top candidate.
- The adapter already preserved explicit top-first emission and FSM child root kind; this regression now proves the top child evidence remains auditable on that single-child path.
- This complements reused-FSM and mixed-child top provenance coverage with the simpler one-FSM-child composition fixture.

## Session update (2026-05-05 `.fsm` mixed-child top support lock)
- Tightened `renderable_top_document_preserves_mixed_child_root_order_and_kind` in `crates/specforge/src/ir/adapters.rs` so mixed producer DT and controller FSM child declarations retain support IDs in the selected top candidate.
- The adapter already preserved mixed child root order and kind; this regression now proves both child declarations remain auditable while the emitted document order stays top-first.
- This extends top child provenance coverage beyond reused-child deduplication into mixed-kind top documents.

## Session update (2026-05-05 `.fsm` reused-FSM-child top support lock)
- Tightened `renderable_top_document_deduplicates_reused_fsm_child_roots` in `crates/specforge/src/ir/adapters.rs` so reused FSM top child declarations retain distinct support IDs in the selected top candidate.
- The adapter already deduplicated the shared FSM child module root while preserving two top child instances; this regression now proves instance-level evidence stays auditable in the FSM-root variant as well.
- This mirrors the reused direct-child support lock across the reused FSM-child document path.

## Session update (2026-05-05 `.fsm` reused-child top support lock)
- Tightened `renderable_top_document_deduplicates_reused_child_module_roots` in `crates/specforge/src/ir/adapters.rs` so reused top child declarations retain distinct support IDs in the selected top candidate.
- The adapter already deduplicated the shared child module root while preserving two top child instances; this regression now proves instance-level child evidence stays auditable.
- This extends baseline top-composition provenance coverage into the reused-child deduplication path.

## Session update (2026-05-05 `.fsm` baseline top-composition provenance lock)
- Tightened `builds_renderable_top_composition_fsm_adapter_artifact` in `crates/specforge/src/ir/adapters.rs` so the baseline renderable top-composition path retains support IDs for top ports, child declarations, and topology links.
- The adapter already rendered the canonical producer/consumer datapath composition; this regression now proves the selected top candidate keeps the core composition evidence auditable.
- This anchors the narrower top-composition recovery and conflict provenance locks against the baseline renderable fixture.

## Session update (2026-05-05 `.fsm` child-declaration top-root confidence provenance lock)
- Tightened `top_root_kind_confidence_follows_child_declaration_evidence` in `crates/specforge/src/ir/adapters.rs` so recovered top children retain child-declaration support IDs while top-root confidence rises to high.
- The adapter already folded child declaration confidence into top-root selection; this regression now proves the recovered child candidate keeps the declaration support record that explains that confidence.
- This complements the recovered top-port root-kind confidence lock with the child-declaration confidence path.

## Session update (2026-05-05 `.fsm` recovered top-root confidence provenance lock)
- Tightened `top_root_kind_confidence_follows_recovered_top_port_evidence` in `crates/specforge/src/ir/adapters.rs` so graph-backed recovered top-port evidence retains support IDs while top-root confidence rises to high.
- The adapter already used recovered top-boundary evidence in top-root confidence folding; this regression now proves the recovered port keeps the actor-port support record that explains that confidence.
- This starts the root-kind confidence provenance lane after the child-topology width coverage.

## Session update (2026-05-05 `.fsm` transitive child-width provenance lock)
- Tightened `top_composition_recovers_child_width_through_transitive_topology` in `crates/specforge/src/ir/adapters.rs` so transitive child width recovery retains support IDs and automation confidence across producer and consumer inventories.
- The adapter already rendered top compositions when producer width recovered from a top link flowed through a sibling link into a consumer input; this regression now proves both contributing topology links remain inspectable.
- This extends the child topology width provenance lane from direct sibling and top-link recovery into multi-hop recovery.

## Session update (2026-05-05 `.fsm` child sibling-link source width provenance lock)
- Tightened `top_composition_recovers_source_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs` so source-side child width recovery from a sibling child link retains support IDs and automation confidence.
- The adapter already rendered top compositions when a producer child output width could be recovered from a consumer sibling link; this regression now proves the producer module inventory keeps that topology evidence inspectable.
- This completes the immediate sibling child-link width recovery provenance pair by covering both target-side and source-side recovery paths.

## Session update (2026-05-05 `.fsm` child sibling-link target width provenance lock)
- Tightened `top_composition_recovers_child_width_from_sibling_child_link_topology` in `crates/specforge/src/ir/adapters.rs` so target-side child width recovery from a sibling child link retains support IDs and automation confidence.
- The adapter already rendered top compositions when a consumer child input width could be recovered from a producer sibling link; this regression now proves the consumer module inventory keeps that topology evidence inspectable.
- This complements the top-link child width recovery lock with the sibling-link target-side width path.

## Session update (2026-05-05 `.fsm` child top-link width provenance lock)
- Tightened `top_composition_recovers_child_width_from_top_link_topology` in `crates/specforge/src/ir/adapters.rs` so child width recovery from public top-link topology retains support IDs and automation confidence.
- The adapter already rendered top compositions when a child output width could be recovered from the linked public top port; this regression now proves the producer module inventory keeps that top-link evidence inspectable.
- This extends renderable child-topology provenance from direction recovery into width recovery.

## Session update (2026-05-05 `.fsm` child link-direction provenance lock)
- Tightened `top_composition_recovers_child_directions_from_link_topology` in `crates/specforge/src/ir/adapters.rs` so child module direction recovery from explicit top-link topology retains support IDs and automation confidence.
- The adapter already rendered top compositions when child module directions could be recovered from topology links; this regression now proves producer-to-consumer and consumer-to-top link evidence stays inspectable in child inventories.
- This complements the child actor-port direction recovery lock with the parallel link-topology recovery path.

## Session update (2026-05-05 `.fsm` child actor-port direction provenance lock)
- Tightened `top_composition_recovers_child_directions_from_actor_ports` in `crates/specforge/src/ir/adapters.rs` so child module direction recovery from actor-port graph evidence retains support IDs and automation confidence.
- The adapter already rendered top compositions when child module directions could be recovered from actor-port graph hints; this regression now proves producer and consumer child inventories keep that evidence inspectable.
- This starts the renderable child-topology recovery provenance lane after the blocked top-port conflict locks.

## Session update (2026-05-05 `.fsm` duplicate top-port width provenance lock)
- Tightened `top_composition_keeps_duplicate_top_port_width_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs` so duplicate top-port width declarations retain both support-ID sets, width-conflict inventory state, and automation confidence.
- The adapter already blocked top lowering when duplicate public top-port declarations disagreed on width; this regression now proves both duplicate declaration records remain inspectable on recovered top ports and selected signal inventory entries.
- This completes the immediate duplicate top-port declaration conflict pair by covering both direction and width disagreements.

## Session update (2026-05-05 `.fsm` duplicate top-port direction provenance lock)
- Tightened `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` in `crates/specforge/src/ir/adapters.rs` so duplicate top-port direction declarations retain both support-ID sets and automation confidence.
- The adapter already blocked top lowering when duplicate public top-port declarations disagreed on direction; this regression now proves both duplicate declaration records remain inspectable on recovered top ports and selected signal inventory entries.
- This extends top-composition conflict provenance coverage from graph-vs-flat disagreement to duplicate explicit declaration disagreement.

## Session update (2026-05-05 `.fsm` top-link direction conflict provenance lock)
- Tightened `top_composition_keeps_conflicting_top_port_direction_unresolved` in `crates/specforge/src/ir/adapters.rs` so blocked top-link direction conflicts retain explicit top-port support IDs, topology-link support IDs, graph-direction inventory state, and automation confidence.
- The adapter already blocked top lowering when a module-topology link contradicted an explicit top-port direction; this regression now proves both evidence families remain inspectable on the recovered top port and selected signal inventory.
- This extends the top-composition conflict locks beyond actor-port graph evidence into link-topology graph evidence.

## Session update (2026-05-05 `.fsm` top actor-width conflict provenance lock)
- Tightened `top_composition_blocks_conflicting_top_actor_port_width` in `crates/specforge/src/ir/adapters.rs` so blocked top actor-port width conflicts retain explicit top-port support IDs, graph actor-port support ID, width-conflict inventory state, and automation confidence.
- The adapter already blocked top lowering when flat top-port width and graph actor-port width disagreed; this regression now proves both evidence families remain inspectable on the recovered top port and selected signal inventory.
- This completes the immediate top actor-port graph-vs-flat conflict pair by covering both direction and width disagreements.

## Session update (2026-05-05 `.fsm` top actor-direction conflict provenance lock)
- Tightened `top_composition_blocks_conflicting_top_actor_port_direction` in `crates/specforge/src/ir/adapters.rs` so blocked top actor-port direction conflicts retain explicit top-port support IDs, graph actor-port support ID, and automation confidence.
- The adapter already blocked top lowering when flat top-port direction and graph actor-port direction disagreed; this regression now proves both evidence families remain inspectable on the recovered top port and selected signal inventory.
- This extends the top-composition provenance locks from successful recovery and missing-width blockers into graph-vs-flat conflict handling.

## Session update (2026-05-05 `.fsm` blocked recovered top-port provenance lock)
- Tightened `top_composition_preserves_recovered_top_port_direction_when_still_blocked` in `crates/specforge/src/ir/adapters.rs` so top-link-recovered top-port direction retains support IDs and automation confidence when another composition gate still blocks the top.
- The adapter already preserved recovered top-boundary direction in the selected signal inventory while a missing child module blocked renderability; this regression now proves the top candidate port and inventory both keep high-confidence evidence.
- This complements the explicit top-link blocker lock by covering successful top-link direction recovery on a blocked top.

## Session update (2026-05-05 `.fsm` unemitted child-link provenance lock)
- Tightened `top_composition_blocks_link_to_unemitted_child_port` in `crates/specforge/src/ir/adapters.rs` so blocked top links to child endpoints that are not emitted retain explicit top-link support IDs and automation confidence.
- The adapter already blocked top lowering when a link endpoint could not resolve to an emitted child port; this regression now proves the blocked link remains inspectable and keeps the source-endpoint enrichment guidance precise.
- This complements the public top-port blocker locks by covering a blocked top-link endpoint path.

## Session update (2026-05-05 `.fsm` parametric top-port provenance lock)
- Tightened `top_composition_blocks_parametric_top_port_width_for_fsm_public_io` in `crates/specforge/src/ir/adapters.rs` so blocked symbolic-width top-composition public IO retains explicit top-port support IDs and automation confidence.
- The adapter already blocked top roots when a public port had parametric width evidence but no numeric width evidence; this regression now proves that symbolic-width evidence remains inspectable in the top candidate and signal inventory.
- This complements the widthless top-port provenance lock by covering the unresolved-parametric public IO blocker.

## Session update (2026-05-05 `.fsm` widthless top-port provenance lock)
- Tightened `top_composition_blocks_widthless_top_port_without_width_recovery` in `crates/specforge/src/ir/adapters.rs` so blocked top-composition public IO retains explicit top-port support IDs and automation confidence.
- The adapter already blocked top roots when a public port had direction evidence but no numeric width evidence; this regression now proves that blocked top-boundary evidence remains inspectable in the top candidate and signal inventory.
- This complements the existing parametric top-port and recovered top-port provenance coverage by covering the missing-width public IO blocker.

## Session update (2026-05-05 `.fsm` standalone undriven-output provenance lock)
- Tightened `standalone_dt_blocks_graph_backed_undriven_output_inventory` in `crates/specforge/src/ir/adapters.rs` so blocked graph-backed undriven output inventory retains selected actor support ID and automation confidence.
- The adapter already blocked standalone-DT renderability when an output had graph-backed direction but no typed control action; this regression now proves the evidence behind that blocked output remains inspectable.
- This complements the structured-FSM undriven-output provenance lock by covering the direct-DT branch of the shared graph-backed output-drive validation path.

## Session update (2026-05-05 `.fsm` structured undriven-output provenance lock)
- Tightened `structured_fsm_blocks_graph_backed_undriven_output_inventory` in `crates/specforge/src/ir/adapters.rs` so blocked graph-backed undriven output inventory retains selected actor support ID and automation confidence.
- The adapter already blocked structured-FSM renderability when an output had graph-backed direction but no typed FSM-state drive action; this regression now proves the evidence behind that blocked output remains inspectable.
- This complements the structured flat/graph provenance lock by covering another blocked structured-FSM signal-inventory path.

## Session update (2026-05-05 `.fsm` structured-FSM flat/graph provenance lock)
- Tightened `structured_fsm_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs` so blocked FSM-root flat-vs-graph disagreement retains interface/actor-port categories, graph support ID, and automation confidence.
- The adapter already blocked structured-FSM renderability when canonical FSM-root direction disagreed with graph-backed actor direction; this regression now proves both evidence families remain inspectable.
- This completes the immediate direct-root, explicit-module, structured-FSM, and system-contract flat/graph provenance coverage set.

## Session update (2026-05-05 `.fsm` explicit-module flat/graph provenance lock)
- Tightened `standalone_explicit_module_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs` so blocked module-local flat-vs-graph disagreement retains interface/actor-port categories, graph support ID, and automation confidence.
- The adapter already blocked explicit-module renderability when canonical module direction disagreed with graph-backed actor direction; this regression now proves both evidence families remain inspectable.
- This brings explicit-module disagreement provenance in line with the direct-root and system-contract flat/graph locks.

## Session update (2026-05-05 `.fsm` system width-conflict provenance lock)
- Tightened `standalone_sequential_dt_blocks_conflicting_system_contract_signal_width` in `crates/specforge/src/ir/adapters.rs` so blocked system-contract width conflicts retain support IDs and automation confidence.
- The adapter already kept contradictory canonical clock width evidence unresolved; this regression now proves the blocked inventory preserves the system-contract evidence trail.
- This completes the immediate standalone sequential system-contract direction/width conflict provenance pair.

## Session update (2026-05-05 `.fsm` system direction-conflict provenance lock)
- Tightened `standalone_sequential_dt_blocks_conflicting_system_contract_signal_direction` in `crates/specforge/src/ir/adapters.rs` so blocked system-contract direction conflicts retain support IDs and automation confidence.
- The adapter already kept contradictory canonical clock direction evidence unresolved; this regression now proves the blocked inventory preserves the system-contract evidence trail.
- This pairs with the flat/graph disagreement coverage to make both system direction-conflict branches auditable.

## Session update (2026-05-05 `.fsm` system flat/graph disagreement provenance lock)
- Tightened `standalone_sequential_dt_blocks_system_contract_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs` so blocked clock flat-vs-graph disagreement retains actor-port support ID and automation confidence.
- The adapter already blocked standalone sequential DT renderability when canonical system-contract direction disagreed with graph-backed actor direction; this regression now proves the contradictory graph evidence remains inspectable.
- This complements the renderable system actor-port direction provenance lock by covering the blocked disagreement branch.

## Session update (2026-05-05 `.fsm` system actor-port direction provenance lock)
- Tightened `standalone_sequential_dt_recovers_system_directions_from_actor_ports` in `crates/specforge/src/ir/adapters.rs` so clock/reset graph-backed actor-port recovery retains support IDs and automation confidence.
- The adapter already used actor-port graph evidence to satisfy standalone sequential DT system-contract direction needs when flat direct-interface hints lagged; this regression now proves that recovered evidence stays visible in the signal inventory.
- This brings the renderable system-contract actor-port direction path in line with the direct-root actor-port provenance locks.

## Session update (2026-05-05 `.fsm` same-actor width-conflict provenance lock)
- Tightened `standalone_dt_keeps_conflicting_actor_port_width_unresolved` in `crates/specforge/src/ir/adapters.rs` so same-actor actor-port width conflicts retain both conflicting support IDs and automation confidence.
- The adapter already kept repeated graph-backed width disagreement unresolved; this regression now proves the blocked inventory preserves enough provenance to audit both sides.
- This completes the immediate same-actor conflict provenance pair with the existing direction-conflict lock.

## Session update (2026-05-05 `.fsm` direct flat-conflict provenance lock)
- Tightened `standalone_dt_blocks_conflicting_flat_direction_even_with_actor_graph` in `crates/specforge/src/ir/adapters.rs` so canonical flat direction conflicts remain provenance-auditable alongside actor graph evidence.
- The adapter already blocked renderability when flat canonical declarations conflicted; this regression now proves graph-backed actor evidence remains visible without overriding the flat conflict.
- This complements the flat-vs-graph disagreement provenance lock by covering flat-only conflict plus graph evidence.

## Session update (2026-05-05 `.fsm` direct flat/graph disagreement provenance lock)
- Tightened `standalone_dt_blocks_flat_graph_direction_disagreement` in `crates/specforge/src/ir/adapters.rs` so flat canonical and graph-backed direction disagreement remains provenance-auditable.
- The adapter already blocked renderability when canonical flat direction evidence disagreed with actor graph direction evidence; this regression now proves interface and actor-port evidence remain visible together.
- This keeps the direct flat/graph disagreement surface aligned with same-actor graph direction conflict provenance.

## Session update (2026-05-05 `.fsm` direct actor-port direction-conflict provenance lock)
- Tightened `standalone_dt_keeps_conflicting_actor_port_direction_unresolved` in `crates/specforge/src/ir/adapters.rs` so same-actor direction conflicts retain both conflicting support IDs and automation confidence.
- The adapter already kept contradictory graph-backed direction evidence unresolved; this regression now proves the blocked inventory preserves enough provenance to audit both sides.
- This mirrors the direct-root width-conflict provenance lock for graph direction evidence.

## Session update (2026-05-05 `.fsm` ambiguous actor-port exclusion lock)
- Tightened `standalone_dt_ignores_ambiguous_actor_port_context` in `crates/specforge/src/ir/adapters.rs` so ambiguous actor-port context must leave graph-backed direction and support evidence unselected.
- The adapter already blocked direct-root lowering when producer/consumer context could not identify a single target actor; this regression now proves the blocked inventory does not carry misleading actor-port provenance.
- This complements the unambiguous and unrelated-context locks by making the negative selection path auditable too.

## Session update (2026-05-05 `.fsm` unrelated actor-port provenance lock)
- Tightened `standalone_dt_ignores_unrelated_actor_ports_for_graph_context` in `crates/specforge/src/ir/adapters.rs` so unrelated actor-port context cannot contaminate selected output provenance.
- The adapter already excluded unrelated `SIDE_BAND` actor-port signals from direct-root inventories; this regression now also proves selected controller output support IDs and automation confidence remain visible.
- This strengthens the direct actor-selection review surface around both positive selected evidence and negative unrelated-evidence exclusion.

## Session update (2026-05-05 `.fsm` unambiguous direct actor provenance lock)
- Tightened `standalone_dt_recovers_directions_from_unambiguous_actor_ports` in `crates/specforge/src/ir/adapters.rs` so unambiguous direct actor-port direction recovery retains support IDs and automation confidence.
- The adapter already recovered graph-backed directions when only one actor context applied; this regression now proves the renderable signal inventory keeps that graph evidence visible for both direct inputs and outputs.
- This anchors the basic direct-root graph recovery path before the more selective shared-output actor and control-read recovery paths.

## Session update (2026-05-05 `.fsm` direct output actor provenance lock)
- Tightened `standalone_dt_selects_output_actor_when_external_actors_share_signals` in `crates/specforge/src/ir/adapters.rs` so selected direct-root output actor evidence retains support IDs and automation confidence.
- The adapter already selected the controller as the target output actor when external monitor actors shared output signal names; this regression now proves the renderable signal inventory keeps the controller graph evidence visible.
- This complements the direct control-read and width-recovery provenance locks by keeping both selected-output and recovered-input evidence auditable on the renderable direct-DT path.

## Session update (2026-05-05 `.fsm` direct-root width-conflict provenance lock)
- Tightened `standalone_dt_blocks_conflicting_control_input_actor_port_widths` in `crates/specforge/src/ir/adapters.rs` so blocked direct-root actor-port width conflicts retain both graph support IDs and automation confidence.
- The adapter already blocked contradictory external actor-port widths; this regression now proves the blocked direct inventory keeps both evidence sides visible.
- This makes the direct-root width-conflict review surface match the explicit-module conflict coverage.

## Session update (2026-05-05 `.fsm` explicit-module width-conflict provenance lock)
- Tightened `standalone_explicit_module_blocks_conflicting_control_input_actor_port_widths` in `crates/specforge/src/ir/adapters.rs` so blocked explicit-module actor-port width conflicts retain both graph support IDs and automation confidence.
- The adapter already blocked contradictory external actor-port widths; this regression now proves the blocked module inventory keeps both evidence sides visible.
- This makes the width-conflict review surface as auditable as the renderable explicit-module width recovery path.

## Session update (2026-05-05 `.fsm` explicit-module actor-port width provenance lock)
- Tightened `standalone_explicit_module_recovers_control_input_width_from_actor_port_graph` in `crates/specforge/src/ir/adapters.rs` so recovered explicit-module actor-port width evidence retains support ID and automation confidence.
- The adapter already recovered missing module-local control-input widths from actor-port numeric shape without importing external actor direction; this regression now proves the renderable module inventory keeps that graph-backed width trail visible.
- This mirrors the direct-root actor-port width provenance lock for explicit-module lowering.

## Session update (2026-05-05 `.fsm` explicit-module control-read provenance lock)
- Tightened `standalone_explicit_module_recovers_inputs_from_module_control_reads` in `crates/specforge/src/ir/adapters.rs` so recovered module-control inputs retain canonical control/transition support and automation confidence.
- The adapter already derived module-local target inputs from explicit-module state-body actions and transition guards; this regression now proves the renderable module inventory keeps that high-confidence evidence trail visible.
- This mirrors the direct-DT and structured-FSM control-read provenance locks for the explicit-module root path.

## Session update (2026-05-05 `.fsm` structured-FSM control-read provenance lock)
- Tightened `structured_fsm_derives_guard_inputs_from_control_reads_after_output_actor_selection` in `crates/specforge/src/ir/adapters.rs` so recovered structured-FSM control-read inputs retain canonical control/transition support and automation confidence.
- The adapter already derived target-actor inputs from FSM state-body actions and transition guards; this regression now proves the renderable inventory keeps that high-confidence evidence trail visible.
- This mirrors the direct-DT control-read provenance lock for the structured-FSM root path.

## Session update (2026-05-05 `.fsm` direct control-read provenance lock)
- Tightened `standalone_dt_derives_target_inputs_from_control_reads_after_output_actor_selection` in `crates/specforge/src/ir/adapters.rs` so recovered direct control-input graph evidence retains canonical control-branch support and automation confidence.
- The adapter already derived `DATA_IN` from typed control reads after selecting the target actor; this regression now proves the renderable inventory keeps that high-confidence direct-read evidence trail visible.
- This complements the direct actor-port width provenance lock by keeping both graph-backed recovery families auditable without importing external actor direction.

## Session update (2026-05-05 `.fsm` direct actor-port width provenance lock)
- Tightened `standalone_dt_recovers_control_input_width_from_actor_port_graph` in `crates/specforge/src/ir/adapters.rs` so recovered direct-root actor-port width evidence retains support ID and automation confidence.
- The adapter already recovered missing direct control-input widths from actor-port numeric shape without importing external actor direction; this regression now proves the renderable inventory keeps that graph-backed width trail visible.
- This keeps direct-root `.fsm` width recovery auditable on both blocked and renderable paths.

## Session update (2026-05-05 `.fsm` numeric-over-symbolic width precedence lock)
- Tightened `standalone_dt_keeps_explicit_numeric_width_over_actor_parametric_width` in `crates/specforge/src/ir/adapters.rs` so the renderable numeric-width precedence path retains interface provenance, supporting IDs, and automation confidence.
- The adapter already preferred explicit numeric canonical width over actor-port symbolic width; this regression now proves the renderable inventory stays auditable and does not carry a stale `parametric_width_hint`.
- This keeps the `.fsm` numeric-width emission path honest while future adapters may still use symbolic widths.

## Session update (2026-05-05 `.fsm` canonical parametric width provenance lock)
- Tightened `standalone_dt_blocks_parametric_signal_width_with_diagnostic` in `crates/specforge/src/ir/adapters.rs` so canonical parametric width blockers retain interface provenance, supporting IDs, and automation confidence.
- The adapter already blocked symbolic canonical width lowering; this regression now proves the blocked `fsm.signal_inventory` entry keeps the canonical evidence trail visible.
- This complements the actor-port symbolic width provenance lock and keeps both symbolic-width source families independently reviewable.

## Session update (2026-05-05 `.fsm` actor-port parametric width provenance lock)
- Tightened `standalone_dt_blocks_parametric_actor_port_width_with_diagnostic` in `crates/specforge/src/ir/adapters.rs` so graph-backed actor-port parametric width blockers must retain provenance category, support ID, and automation confidence.
- The adapter already blocked symbolic actor-port width lowering; this regression now proves the blocked `fsm.signal_inventory` entry keeps the actor-port evidence trail visible.
- This keeps canonical parametric width provenance and graph-backed actor-port parametric width provenance auditable as separate sources.

## Session update (2026-05-05 `.fsm` top parametric width provenance lock)
- Tightened `top_composition_blocks_parametric_top_port_width_for_fsm_public_io` in `crates/specforge/src/ir/adapters.rs` so selected top signal inventory entries must preserve the symbolic `parametric_width_hint` for blocked public IO widths.
- The adapter already blocked parametric public IO lowering; this regression now also proves the blocked artifact keeps the symbolic width provenance visible in `fsm.signal_inventory`.
- This keeps parametric width evidence distinct from missing numeric width evidence and from true width conflicts at the top-root renderability boundary.

## Session update (2026-05-05 NLP alias reference-link normalization)
- Extended `normalize_alias_subject_markup(...)` in `crates/specforge/src/commands/nlp_enrich.rs` so reference-style markdown links in candidate alias subjects collapse to the visible label.
- Added `extract_alias_phrase_uses_reference_link_labels`, bringing focused alias parser coverage to `9` tests and NLP-enrich module coverage to `21` tests.
- This keeps markdown reference ids out of `signal_alias_map` while preserving the useful linked prose phrase.

## Session update (2026-05-05 NLP alias markdown-link normalization)
- Hardened `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs` so markdown links in candidate alias subjects are normalized to their visible labels before alias word cleanup.
- Added `extract_alias_phrase_uses_markdown_link_labels`, bringing focused alias parser coverage to `8` tests and NLP-enrich module coverage to `20` tests.
- This prevents link target syntax from entering `signal_alias_map` while keeping linked prose labels usable as implicit aliases.

## Session update (2026-05-05 NLP alias punctuation normalization)
- Hardened `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs` so Form 2 alias learning trims surrounding ASCII punctuation from each candidate alias word after article stripping.
- Added `extract_alias_phrase_trims_wrapping_punctuation_from_alias_words`, bringing focused alias parser coverage to `7` tests and NLP-enrich module coverage to `19` tests.
- This keeps inline-code wrappers and trailing modal-boundary punctuation out of `signal_alias_map` without dropping useful implicit noun-phrase aliases.

## Session update (2026-05-05 NLP alias marker filter hardening)
- Hardened `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs` so Form 2 alias learning rejects numeric outline markers, parenthesized list markers, and lettered list markers before noun-phrase normalization.
- Added `extract_alias_phrase_rejects_outline_and_lettered_marker_prefixes`, bringing the focused alias parser coverage to `6` tests and the NLP-enrich module coverage to `18` tests.
- This reduces alias-map pollution from markdown/list structure while preserving the useful implicit noun-phrase alias path.

## Session update (2026-05-05 mdBook adapter provenance docs)
- Updated mdBook user-facing documentation for the `.fsm` adapter provenance behavior in the actor-connectivity, pipeline-command, and generated-artifact reference pages.
- The book now distinguishes blocked renderability role collapse from signal-inventory provenance preservation: users can inspect flat `direction_hint` and graph-backed `graph_direction_hint` even when the renderable top port is unresolved.
- This documents the slice-22 top-root inventory contract without changing Rust behavior.

## Session update (2026-05-05 `.fsm` top-port flat/graph inventory provenance)
- Hardened top-root signal inventory projection in `crates/specforge/src/ir/adapters.rs` so flat top-port directions and graph-backed actor/topology directions remain separately inspectable when they disagree.
- `build_top_signal_inventory(...)` now reserves `graph_direction_hint_conflicted` for true graph-side conflicts, instead of setting it for flat-vs-graph disagreement.
- Updated the top actor-port and top-link disagreement regressions to lock visible `direction_hint` plus `graph_direction_hint` pairs while top renderability remains blocked.

## Session update (2026-05-05 `.fsm` system-contract flat/graph disagreement guard)
- Added standalone sequential-DT system-contract coverage for the shared flat-vs-graph direction disagreement rule in `crates/specforge/src/ir/adapters.rs`.
- `standalone_sequential_dt_blocks_system_contract_flat_graph_direction_disagreement` proves canonical system-contract input evidence and graph-backed actor-port output evidence remain visible together while `.fsm` lowering stays blocked.
- This locks the `validate_system_signal_renderability(...)` branch added with the direct guard, complementing direct DT, explicit-module, and structured-FSM coverage.

## Session update (2026-05-05 `.fsm` structured-FSM flat/graph disagreement guard)
- Added structured-FSM coverage for the shared flat-vs-graph direction disagreement rule in `crates/specforge/src/ir/adapters.rs`.
- `structured_fsm_blocks_flat_graph_direction_disagreement` proves FSM-state action lowering preserves both contradictory direction sources in the adapter artifact and refuses to render until upstream canonical shape is corrected.
- This completes immediate path coverage for the shared guard across direct DT, explicit-module FSM, and direct structured-FSM roots.

## Session update (2026-05-05 `.fsm` explicit-module flat/graph disagreement guard)
- Added explicit-module coverage for the shared flat-vs-graph direction disagreement rule in `crates/specforge/src/ir/adapters.rs`.
- `standalone_explicit_module_blocks_flat_graph_direction_disagreement` proves module-local canonical shape and actor-relative graph shape are both preserved in the artifact when they disagree, while lowering remains blocked.
- This closes the immediate coverage gap after the direct-root production fix: both direct-root and explicit-module inventories now lock the same graph-first but contradiction-safe behavior.

## Session update (2026-05-05 `.fsm` flat/graph direction disagreement guard)
- Hardened `crates/specforge/src/ir/adapters.rs` so `preferred_signal_direction_hint(...)` returns unresolved when unconflicted flat canonical direction evidence disagrees with graph-backed actor-relative direction evidence.
- Added `standalone_dt_blocks_flat_graph_direction_disagreement`, proving a single contradictory flat declaration is now blocking even when actor-port graph evidence could otherwise render the direct DT root.
- Renderability diagnostics now distinguish flat-vs-graph disagreement from flat-only conflict, graph-only conflict, and genuinely missing direction evidence.

## Session update (2026-05-05 actor-taxonomy prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so actor-taxonomy role lookup now shares the exact-family plus AMBA-generic search policy used by strict prior consumers.
- Added `actor_taxonomy_prior_protocol_family_mismatch_negative`, which proved the previous broad fallback could let an APB actor-taxonomy prior infer AXI-local section-heading direction, graph direction, actor relations, and actor ports.
- Updated the evidence-layer actor-taxonomy unit tests to infer AMBA for AMBA-generic prior fallback, keeping the positive path explicit while closing unknown-family fallback leakage.

## Session update (2026-05-05 negative-knowledge prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so negative-knowledge pattern matching now shares the exact-family plus AMBA-generic search policy used by strict prior consumers.
- Added `negative_knowledge_prior_protocol_family_mismatch_negative`, which proved the previous broad fallback could let an APB negative-knowledge prior match an AXI-local semantic conflict and emit caution/rescan findings.
- The change keeps negative knowledge advisory and scoped: it can still surface matching-family caution records, but unrelated concrete protocol families no longer create prior-memory matches.

## Session update (2026-05-05 semantic phrase prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so semantic phrase role lookup now shares the exact-family plus AMBA-generic search policy used by the other strict prior consumers.
- Added `semantic_prior_protocol_family_mismatch_negative`, which proved the previous broad fallback could let an APB semantic phrase prior resolve an AXI-local signal role.
- Renamed the AMBA-generic positive semantic-prior fixture source files to AMBA-scoped names, keeping fallback coverage intentional while removing hidden dependence on unknown-family fallback.

## Session update (2026-05-05 temporal prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so temporal phrase cycle-window lookup now shares the exact-family plus AMBA-generic search policy used by semantic modality, table-shape, and visual-motif priors.
- Added `temporal_prior_protocol_family_mismatch_negative`, which proved the previous broad fallback could let an APB temporal phrase prior add a cycle window to an AXI-local timing rule.
- This keeps learned temporal phrase priors advisory and protocol-scoped: unrelated concrete families cannot fill timing bounds when current-document evidence still lacks that cycle-window grounding.

## Session update (2026-05-05 visual-motif prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so visual motif caption lookup now shares the exact-family plus AMBA-generic search policy used by table-shape and semantic modality-reliability priors.
- Added `visual_motif_prior_protocol_family_mismatch_negative`, which proved the prior broad fallback could let an APB caption motif classify an AXI-local unknown visual asset and emit visual-motif corroboration.
- Updated the evidence-layer visual motif unit test source key to infer AXI, preserving intentional AMBA-generic fallback coverage while closing unknown-family fallback leakage.

## Session update (2026-05-05 table-shape prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so table-shape prior lookup now shares the exact-family plus AMBA-generic search policy with semantic modality-reliability priors.
- Added `table_shape_prior_protocol_family_mismatch_negative`, which proved the previous broad fallback could let an APB table-shape prior classify an AXI-local unknown table and mint resolved directions.
- The helper is now intentionally general for prior consumers that must stay exact-family scoped while preserving AMBA-generic reuse; unrelated concrete protocol families remain advisory-silent.

## Session update (2026-05-05 semantic modality-prior AMBA-generic fallback fixture)
- Added `semantic_modality_reliability_amba_generic_fallback_gold` to the KG quality corpus.
- This test-only slice locks the intended positive policy after protocol-family hardening: AMBA-generic modality-reliability priors can still guide AXI-family documents, while unrelated concrete families remain blocked.
- The fixture keeps the semantic conflict visible while proving prior-guided arbitration and consensus are still available for a generic AMBA prior.

## Session update (2026-05-05 semantic modality-prior protocol-family guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so semantic modality-reliability priors use exact protocol-family lookup plus AMBA-generic fallback, without the broad any-family fallback used by other prior lookups.
- Added `semantic_modality_reliability_protocol_family_mismatch_negative`, which proved the previous behavior could let an APB prior resolve an AXI-local semantic conflict.
- The narrower lookup preserves the intended strong-prior gold path while keeping family-mismatched priors advisory-silent.

## Session update (2026-05-05 semantic modality-prior source-kind guard fixture)
- Added `semantic_modality_reliability_source_kind_mismatch_negative` to the KG quality corpus.
- The fixture proves semantic modality-reliability prior bonuses remain source-kind scoped: a visual-caption reliability prior cannot decide a prose/table semantic conflict.
- This is a test-only guard around the existing exact source-kind filter in `CorpusMemory::semantic_modality_reliability_bonus`.

## Session update (2026-05-05 semantic modality-prior weak-margin guard)
- Hardened `crates/specforge/src/ir/semantic.rs` so semantic modality-reliability priors are applied once per source kind while building a role candidate.
- Prior-guided semantic arbitration now requires decisive-strength prior adjustment before learned modality reliability can resolve a conflict, preventing weak priors from using existing evidence asymmetry to force consensus.
- Added `semantic_modality_reliability_weak_prior_negative` to prove weak prior memory preserves non-decisive arbitration while the existing strong-prior gold still resolves.

## Session update (2026-05-05 semantic prior broad-phrase guard)
- Hardened `crates/specforge/src/ir/prior_memory.rs` so loaded semantic phrase priors are rechecked for meaningful phrase shape during lookup.
- `is_meaningful_prior_phrase` now requires at least two non-placeholder terms, which blocks one-token records like `transfer` from matching arbitrary local prose that happens to contain that word.
- Added `semantic_prior_broad_phrase_negative` to prove the false positive is closed through the full `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` KG fixture path while representative positive prior fixtures still pass.

## Session update (2026-05-05 KG semantic-prior phrase-match guard fixture)
- Added `semantic_prior_phrase_mismatch_negative` to the KG quality corpus.
- The fixture proves semantic phrase priors remain local-phrase grounded by staging an unrelated prior and expecting the current prose phrase to stay unresolved.
- The corpus-KB prior-candidate manifest now surfaces this as another semantic-phrase guard fixture, strengthening the boundary between prior availability and local evidence matching.

## Session update (2026-05-05 KG semantic-prior source-kind guard fixture)
- Added `semantic_prior_source_kind_mismatch_negative` to the KG quality corpus.
- The fixture proves semantic phrase priors remain source-kind scoped by staging a visual-caption prior while presenting the same normalized phrase only as prose.
- The corpus-KB prior-candidate manifest now surfaces this as another semantic-phrase guard fixture, strengthening the boundary between local evidence modality and advisory prior interpretation.

## Session update (2026-05-05 KG semantic-prior conflict guard fixture)
- Added `semantic_prior_conflicting_roles_negative` to the KG quality corpus.
- The fixture stages conflicting semantic phrase priors for the same normalized local phrase and proves the EvidenceIR/SemanticIR/IntentIR pipeline does not resolve a role or emit semantic consensus from ambiguous prior memory.
- The corpus-KB prior-candidate manifest now treats this as an additional semantic-phrase guard fixture, strengthening the review-only non-mutation boundary around semantic prior promotion.

## Session update (2026-05-05 KG ready-like prior-guided sink phrase fixtures)
- Added paired KG quality fixtures for ready-like semantic phrase recovery through typed prior memory.
- The gold fixture proves `<signal> can sink the transfer` can map to `handshake_ready_like` when the prior exists, while the negative fixture proves the same source text does not become semantic truth by heuristic fallback.
- This complements the existing receive-transfer and valid-like publish fixtures with another prior-dependent ready-like wording and keeps the corpus-KB prior-memory projection current.

## Session update (2026-05-05 KG valid-like prior-guided semantic phrase fixtures)
- Added paired KG quality fixtures under `crates/specforge/test_data/kg_quality` for valid-like semantic phrase recovery.
- The gold fixture exercises typed prior-memory arbitration for an otherwise ambiguous prose phrase, while the negative fixture verifies the same phrase remains unresolved without that prior.
- The refreshed corpus-KB projection surfaces the pair in benchmark, semantic/truthfulness pattern, and prior-memory views, preserving reviewability for the broader KG fixture corpus.

## Session update (2026-05-05 rescan-plan unsupported NLP argument coverage)
- Extended `crates/specforge/src/commands/rescan_plan.rs` malformed provider tests to cover unsupported trailing args for NLP enrichment hints.
- NLP enrichment provider parsing now has explicit regression coverage for duplicate providers, missing values, flag-shaped values, unsupported providers, and unsupported args.
- This completes the current batch with symmetrical provider hardening coverage across source and NLP enrichment replay hints.

## Session update (2026-05-05 rescan-plan duplicate NLP provider coverage)
- Extended `crates/specforge/src/commands/rescan_plan.rs` malformed provider tests to cover duplicate `--vlm-provider` flags for NLP enrichment hints.
- NLP enrichment provider parsing already rejected duplicates through the same replacement guard as source enrichment.
- The regression keeps provider-option hardening symmetrical across both replay enrichment lanes.

## Session update (2026-05-05 rescan-plan extra intent argument coverage)
- Extended `crates/specforge/src/commands/rescan_plan.rs` extra-argument coverage to the intent rebuild command hint.
- The execution parser remains exact-arity for direct rebuild and validation hints.
- This keeps unsupported trailing flags out of the intent replay path before dispatch.

## Session update (2026-05-05 rescan-plan lane mismatch coverage)
- Extended `crates/specforge/src/commands/rescan_plan.rs` command-hint mismatch coverage across source, enrichment, NLP enrichment, and intent lanes.
- The execution parser remains fail-closed unless the structured command intent and SpecForge subcommand agree.
- This further constrains replay dispatch to the command-hint contract rather than trusting display strings or mismatched args.

## Session update (2026-05-05 rescan-plan missing path coverage)
- Extended the malformed command-hint tests in `crates/specforge/src/commands/rescan_plan.rs` to cover missing intent and NLP enrichment replay paths.
- The parser behavior was already fail-closed through exact argument-shape matching and provider parsing.
- The coverage now spans all replay command families that accept source or artifact path arguments.

## Session update (2026-05-05 rescan-plan current-directory replay path rejection)
- Hardened `crates/specforge/src/commands/rescan_plan.rs` command-hint path parsing against the current-directory token `.`.
- `parse_rescan_command_path` now rejects placeholders that name the execution root itself before accepting a replay path.
- This keeps structured replay paths non-empty, non-directory-root, relative, and non-traversing before execution dispatch.

## Session update (2026-05-05 rescan-plan empty replay path rejection)
- Hardened `crates/specforge/src/commands/rescan_plan.rs` command-hint path parsing against explicit empty path tokens.
- `parse_rescan_command_path` now rejects missing-by-value paths before checking absolute or parent-traversal forms.
- This keeps structured replay paths non-empty, relative, and non-traversing before execution dispatch.

## Session update (2026-05-05 rescan-plan parent traversal path rejection)
- Hardened `crates/specforge/src/commands/rescan_plan.rs` command-hint path parsing against `..` parent traversal components.
- `parse_rescan_command_path` now rejects both absolute paths and upward traversal before constructing replay invocations.
- Execution dispatch now resolves parsed relative replay paths against a captured execution root, avoiding accidental dependence on process-global current-directory changes during tests or nested execution.
- This keeps structured replay paths repo-relative in practice rather than merely syntactically relative.

## Session update (2026-05-05 rescan-plan absolute replay path rejection)
- Hardened `crates/specforge/src/commands/rescan_plan.rs` command-hint parsing so replay source/artifact path tokens must be relative paths.
- `parse_rescan_command_path` now rejects absolute paths before constructing `RescanInvocation` variants for ingest, enrich, nlp-enrich, stage rebuild, or validate lanes.
- This closes a trust-boundary gap left after executable and working-directory checks: structured hints still have to stay inside repo-relative replay vocabulary before execution dispatch.

## Session update (2026-05-05 rescan-plan source-enrich skip classify-only)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around source-only classification replay.
- No production parser code changed; the new regression proves `--vlm-provider skip --classify-only` parses as source enrichment with classification-only enabled.
- This keeps source-stage classification replay explicit while the evidence NLP lane continues rejecting classify-only flags.

## Session update (2026-05-05 rescan-plan source-enrich skip provider)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around source-enrichment local provider variants.
- No production parser code changed; the new regression proves `--vlm-provider skip` parses into a source-enrichment replay invocation with `VlmProviderArg::Skip`.
- This keeps source-enrichment provider coverage aligned with evidence NLP replay coverage.

## Session update (2026-05-05 rescan-plan open-ai alias rejection)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around rejected remote-provider aliases.
- No production parser code changed; the new regression proves the hyphenated `open-ai` value is rejected for both source enrichment and evidence NLP replay hints.
- This keeps executable rescan-plan replay constrained to local providers instead of silently accepting remote OpenAI spellings.

## Session update (2026-05-05 rescan-plan lmstudio alias)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around accepted local LM Studio provider spellings.
- No production parser code changed; the new regression proves the unhyphenated `lmstudio` value still maps to `VlmProviderArg::LmStudio`.
- This keeps local replay provider aliases explicit and covered.

## Session update (2026-05-05 rescan-plan nlp classify-only rejection)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around `nlp-enrich` replay flags.
- No production parser code changed; the new regression proves `--classify-only` remains source-enrich-only and is rejected for evidence NLP replay hints.
- This keeps local replay lanes explicit instead of sharing flags accidentally.

## Session update (2026-05-05 rescan-plan command trust boundary)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around command-hint executable and working-directory gates.
- No production parser code changed; the new regression proves non-`cargo` executables and non-repository working directories are rejected before replay invocation construction.
- This keeps rescan execution constrained to repository-local cargo stage commands.

## Session update (2026-05-05 rescan-plan exact pending-status selection)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` selection tests around pending automation status matching.
- No production selection code changed; the new regression proves only exact `planned_not_executed` entries are selected.
- This keeps malformed or review-adjusted status strings from slipping into local rescan execution.

## Session update (2026-05-05 rescan-plan exact document-key filter)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` selection tests around scoped document-key matching.
- No production selection code changed; the new regression proves `--document-key` selection is exact and does not catch prefix-like sibling keys.
- This reduces risk when rescan plans contain multiple related generated document keys.

## Session update (2026-05-05 rescan-plan grade-removal execution status)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` execution-status tests around optional grade removal.
- No production status code changed; the new regression proves validation snapshot changes from grade disappearance are counted as executed validation changes.
- This keeps execution reporting sensitive to neutral-review grade availability drift.

## Session update (2026-05-05 rescan-plan score-removal execution status)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` execution-status tests around optional score removal.
- No production status code changed; the new regression proves validation snapshot changes from score disappearance are counted as executed validation changes.
- This keeps execution reporting sensitive to neutral-review score availability drift.

## Session update (2026-05-05 rescan-plan grade-removal arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around optional grade removal.
- No production arbitration code changed; the new regression proves grade disappearance stays neutral review when score and finding direction do not move.
- This keeps qualitative grade availability drift separated from score/finding regressions.

## Session update (2026-05-05 rescan-plan score-removal arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around optional score removal.
- No production arbitration code changed; the new regression proves `Some(score) -> None` leaves `score_delta` unavailable and therefore routes to neutral review absent directional finding/count evidence.
- This keeps score availability drift distinct from true numeric score decreases.

## Session update (2026-05-05 rescan-plan grade-only execution status)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` execution-status tests around grade-only validation changes.
- No production status code changed; the new regression proves qualitative grade drift still counts as `executed_validated_changed` even when arbitration stays neutral review.
- This keeps report counters honest for non-promotional validation changes.

## Session update (2026-05-05 rescan-plan grade-only arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around grade-only validation changes.
- No production arbitration code changed; the new regression proves grade-label drift alone produces neutral review rather than regression or improvement.
- This keeps qualitative validation changes review-required while preserving the stricter regression/improvement signals for score and finding movement.

## Session update (2026-05-05 rescan-plan execute document-key scope)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` execute-mode tests around document-key scoped queue execution.
- No production execution code changed; the new regression proves `--execute --document-key` updates only matching pending recommendations and leaves other document work pending without summaries.
- This keeps targeted local replay safe for multi-document plans.

## Session update (2026-05-05 rescan-plan execute limit)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` execute-mode tests around bounded queue execution.
- No production execution code changed; the new regression proves `--execute --limit 1` updates only selected work and leaves unselected pending recommendations without execution summaries.
- This keeps local replay batches bounded and auditable.

## Session update (2026-05-05 rescan-plan execute report no-change)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` execute-mode run-report tests around validated-no-change summaries.
- No production execution code changed; the new regression proves returned reports increment no-change counters, avoid review-required totals, and carry the no-change promotion summary.
- This keeps the in-memory execution report aligned with the persisted plan mutation.

## Session update (2026-05-05 rescan-plan dry-run report counts)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` dry-run run-report tests around scoped selection counters.
- No production report code changed; the new regression proves non-executing reports retain the document-key filter, scoped pending count, selected count, and zero execution counters.
- This keeps dry-run replay summaries deterministic before any local rescan mutation is allowed.

## Session update (2026-05-05 rescan-plan replay-input rendering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` dry-run renderer tests around replay-input compact formatting.
- No production renderer code changed; the new regression proves empty replay inputs render as `none` and non-empty inputs stay ordered as `kind:path` pairs.
- This keeps replay-scope review output deterministic for local rescan operators.

## Session update (2026-05-05 rescan-plan list rendering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` dry-run renderer tests around compact string-list rendering.
- No production renderer code changed; the new regression proves empty list fields render as `none` and non-empty values stay comma-separated in source order.
- This keeps review-facing dry-run queue fields stable without broadening CLI behavior.

## Session update (2026-05-05 rescan-plan verdict-count summary)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` run-report tests around exact arbitration verdict counting.
- No production report code changed; the new regression proves each verdict is counted by exact match instead of review-required suffix grouping.
- This keeps CLI/report summary consumers aligned with the distinct arbitration classes.

## Session update (2026-05-05 rescan-plan review-count summary)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` run-report tests around review-required summary counting.
- No production report code changed; the new regression proves `validated_no_change` summaries stay out of `review_required_count()` while review-required arbitration verdicts count.
- This keeps report summaries aligned with the no-promotion review taxonomy.

## Session update (2026-05-05 rescan-plan finding-count improvement)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around validation finding-count decreases.
- No production arbitration code changed; the new regression proves a negative finding-count delta alone produces possible-improvement review.
- This keeps aggregate validation improvements review-required rather than promoting them automatically.

## Session update (2026-05-05 rescan-plan finding-count regression)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around validation finding-count increases.
- No production arbitration code changed; the new regression proves a positive finding-count delta alone produces regression review.
- This protects aggregate validation regressions even when precise finding identity changes are absent.

## Session update (2026-05-05 rescan-plan score-rise arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around validation score increases.
- No production arbitration code changed; the new regression proves a positive score delta alone produces possible-improvement review.
- This keeps favorable score movement review-required rather than conflating it with canonical truth promotion.

## Session update (2026-05-05 rescan-plan score-drop arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around validation score decreases.
- No production arbitration code changed; the new regression proves a negative score delta alone produces regression review.
- This keeps score-driven regressions explicit even when finding identities and counts do not change.

## Session update (2026-05-05 rescan-plan added-finding arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around newly added validation findings.
- No production arbitration code changed; the new regression proves added findings alone produce regression review even when score and finding-count metadata are flat.
- This keeps finding-identity regressions visible instead of relying on aggregate validation counters.

## Session update (2026-05-05 rescan-plan removed-finding arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests around removed validation findings.
- No production arbitration code changed; the new regression proves removed findings alone produce possible-improvement review rather than neutral drift.
- This keeps the review verdict boundary explicit for rescan executions that remove a finding without changing score or finding-count metadata.

## Session update (2026-05-05 rescan-plan command-hint render order)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` dry-run renderer tests around multi-command recommendations.
- No production renderer code changed; the new regression proves command hints are printed in stored plan order within each selected recommendation.
- This keeps generated replay sequences reviewable without hidden command reordering.

## Session update (2026-05-05 rescan-plan unscoped executed-skip limits)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` queue-selection tests around executed entries inside unscoped positive limits.
- No production selection code changed; the new regression proves positive limits are applied after pending-status filtering even without a document-key filter.
- This pairs with the scoped executed-skip regression so automatic replay batches do not spend limit slots on already-executed recommendations in either selection mode.

## Session update (2026-05-05 rescan-plan scoped executed-skip limits)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` queue-selection tests around executed entries inside document-scoped positive limits.
- No production selection code changed; the new regression proves positive limits are applied after pending-status and document-key filtering.
- This keeps scoped automatic replay batches from spending limit slots on already-executed recommendations.

## Session update (2026-05-05 rescan-plan no-change promotion gate)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` policy tests around `validated_no_change` promotion gates.
- No production policy code changed; the new regression proves no-change rescans remain not promoted, blocked by no-delta policy, and not reviewable for canonical mutation.
- This keeps promotion semantics inspectable without depending only on the integration execution test.

## Session update (2026-05-05 rescan-plan fingerprint delta)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` delta tests around fingerprint-only changes.
- No production delta code changed; the new regression proves artifact fingerprint changes remain visible even when validation score, grade, and finding sets are unchanged.
- This keeps rescan arbitration honest about byte-level artifact changes that still require neutral human review.

## Session update (2026-05-05 rescan-plan missing validation report)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` reader tests around absent validation-report sidecars.
- No production reader code changed; the new regression proves the missing `validation_report.json` path is reported directly.
- This keeps rescan execution dependent on real validator output rather than implicit or fabricated snapshots.

## Session update (2026-05-05 rescan-plan validation report sidecar)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` helper tests around validation report sidecar path derivation.
- No production helper code changed; the new regression proves sidecars stay colocated with the validated artifact directory.
- This keeps validation snapshot loading aligned with the `validate` command's deterministic `validation_report.json` placement.

## Session update (2026-05-05 rescan-plan schema version)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` loader tests around schema-version gating.
- No production loader code changed; unsupported schema versions now have direct regression coverage.
- This keeps preview and execution paths pinned to schema-v2 rescan plans.

## Session update (2026-05-05 rescan-plan replay-input rendering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` render tests around `replay_inputs`.
- No production renderer code changed; dry-run output now has direct coverage for preserving the plan's replay-input order.
- This keeps the displayed replay chain stable and faithful to the stored recommendation.

## Session update (2026-05-05 rescan-plan related-id rendering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` render tests around `related_ids`.
- No production renderer code changed; dry-run output now has direct coverage for preserving the plan's related-id order.
- This keeps review-facing replay scope text stable and faithful to the stored recommendation.

## Session update (2026-05-05 rescan-plan selected-order rendering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` render tests around selected queue order.
- No production renderer code changed; dry-run output now has direct coverage for preserving caller-provided selected index order.
- This keeps preview row order deterministic after queue filtering and limiting.

## Session update (2026-05-05 rescan-plan selected-index rendering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` render tests around selected queue indices.
- No production renderer code changed; dry-run output now has direct coverage for excluding unselected recommendations.
- This keeps limited and document-scoped previews aligned with the actual execution selection.

## Session update (2026-05-05 rescan-plan empty dry-run queue)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` render tests around empty dry-run selections.
- No production render code changed; empty selected queues now have direct coverage for rendering only the `rescan_queue:` header.
- This keeps no-op queue previews explicit without fabricating recommendation rows.

## Session update (2026-05-05 rescan-plan scoped unlimited queue selection)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` queue-selection tests around `limit == 0` with document scoping.
- No production selection code changed; the new regression proves unlimited mode still applies the document filter before collecting pending indices.
- This protects multi-document rescan plans when operators ask to process all pending work for one document.

## Session update (2026-05-05 rescan-plan large-limit queue selection)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` queue-selection tests around oversized positive limits.
- No production selection code changed; pending recommendations now have direct coverage for all-pending preservation when `limit` is larger than the queue.
- This protects automatic batch execution from accidental pending-work loss when callers choose a generous limit.

## Session update (2026-05-05 rescan-plan dry-run command display)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` render tests around command-hint display text.
- No production renderer code changed; dry-run output is now directly covered as printing `ProjectRescanCommandHint.display` for human review.
- This pairs with the parser trust-boundary test by keeping display text visible without using it as executable input.

## Session update (2026-05-05 rescan-plan display-string trust)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around the display-string trust boundary.
- No production parser code changed; execution parsing is now directly covered as structured-field driven even when `display` is misleading.
- This keeps review-facing shell text separate from executable replay input.

## Session update (2026-05-05 rescan-plan cargo prefix tokens)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around the exact cargo command prefix.
- No production parser code changed; wrong cargo subcommands and missing `--` separators are now directly covered.
- This protects the executor from accepting cargo-shaped hints unless they preserve the exact repository-local run prefix.

## Session update (2026-05-05 rescan-plan missing stage paths)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around required stage-command paths.
- No production parser code changed; missing path/artifact args are now directly covered for `ingest`, `evidence`, `semantic`, and `validate`.
- This completes the exact-arity lock around non-enrichment replay lanes after extra args were covered.

## Session update (2026-05-05 rescan-plan extra stage args)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around exact stage-command arity.
- No production parser code changed; extra args are now directly covered for `ingest`, `evidence`, `semantic`, and `validate` command hints.
- This protects the executor from accepting optional CLI switches through stage rebuild lanes that are intended to be exact replay shapes.

## Session update (2026-05-05 rescan-plan stage command parsing)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around whitelisted non-enrichment stage commands.
- No production parser code changed; accepted `ingest`, `evidence`, `semantic`, and `validate` command hints are now directly covered.
- This pairs with the recent rejection tests so both the allowed and denied sides of the structured executor boundary are explicit.

## Session update (2026-05-05 rescan-plan unknown command intents)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around the structured command-intent allowlist.
- No production parser code changed; unknown validation-shaped and adapter-rebuild-shaped intents are now directly covered.
- This keeps replay execution tied to explicit executor lanes instead of accepting valid-looking subcommands under unsupported intent names.

## Session update (2026-05-05 rescan-plan intent/subcommand mismatches)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around structured intent and subcommand pairing.
- No production parser code changed; evidence-vs-semantic and validate-vs-intent mismatches are now directly covered.
- This protects the executor from treating a plausible SpecForge subcommand as valid when it is attached to the wrong rescan-plan intent.

## Session update (2026-05-05 rescan-plan unknown provider values)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around unsupported local provider names.
- No production parser code changed; unknown provider labels are now directly covered for both enrich and NLP-enrich command hints.
- This keeps the executable replay surface pinned to the explicit local provider allowlist.

## Session update (2026-05-05 rescan-plan flag-shaped provider values)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around local provider values.
- No production parser code changed; option-token provider values are now directly covered for both enrich and NLP-enrich command hints.
- This complements the model-value fix by locking both replay option value positions against flag swallowing.

## Session update (2026-05-05 rescan-plan flag-shaped model values)
- Tightened `crates/specforge/src/commands/rescan_plan.rs` replay command parsing for local model overrides.
- `parse_vlm_model_hint_value(...)` now rejects model values that begin with `--`, preventing a following option from being swallowed as the model name.
- The regression covers both enrich and NLP-enrich hints, closing the malformed-input gap left after missing end-of-list model values were covered.

## Session update (2026-05-05 rescan-plan explicit provider hints)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around local replay provider policy.
- No production parser code changed; missing explicit `--vlm-provider` is now directly covered for both enrich and NLP-enrich command hints.
- This keeps replay execution local-provider explicit instead of letting hidden defaults leak into generated rescan plans.

## Session update (2026-05-05 rescan-plan missing model values)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around truncated local replay model overrides.
- No production parser code changed; missing `--vlm-model` values are now directly covered for both enrich and NLP-enrich command hints.
- This complements the duplicate-flag lock by covering the other local model-override malformed-input shape.

## Session update (2026-05-05 rescan-plan duplicate replay flags)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` parser tests around local replay command hints.
- No production parser code changed; duplicate `--vlm-model` and duplicate `--classify-only` rejection is now directly covered.
- This keeps replay invocation parsing deterministic before execution reaches local enrichment or NLP-enrichment lanes.

## Session update (2026-05-05 rescan-plan scoped limit ordering)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` queue-selection tests.
- No production selection code changed; the new regression proves `selected_pending_indices(...)` filters by pending status and document key before applying `limit`.
- This protects multi-document queues where earlier pending recommendations from other documents should not consume a scoped document's execution budget.

## Session update (2026-05-05 rescan-plan score labels)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` presentation tests around validation score labels.
- No production formatting changed; `score_label(...)` now has direct coverage for complete, score-only, grade-only, and absent metadata.
- This protects review-facing execution-summary text for partial validation reports without changing the rescan executor contract.

## Session update (2026-05-05 rescan-plan mixed finding arbitration)
- Strengthened `crates/specforge/src/commands/rescan_plan.rs` arbitration tests for mixed finding exchanges.
- No production verdict logic changed; the new regression locks the existing conservative order where added findings are checked before removed findings.
- This matters because an execution can remove one finding and introduce another while keeping score and finding count flat, and that state must still require regression review rather than possible-improvement review.

## Session update (2026-05-05 graph replay command-lane assertions)
- Strengthened the combined graph-direction replay tests in `crates/specforge/src/commands/project_validation.rs`.
- No production replay logic changed; the tests now assert that both concurrent recommendations keep their command lanes:
  - SemanticIR coverage and conflict recommendations use NLP enrichment, SemanticIR rebuild, then validation
  - IntentIR coverage and conflict recommendations use NLP enrichment, SemanticIR rebuild, IntentIR rebuild, then validation
- This complements the related-id and replay-input assertions by guarding the actionable command-hint side of schema-v2 rescan plans.

## Session update (2026-05-05 flat-hint graph-conflict guidance fixture)
- Strengthened `graph_direction_same_actor_conflict_negative` in the tracked KG fixture set.
- No Rust production code changed; the flat-hint-present graph conflict fixture now asserts the exact rescan-guidance payloads for both SemanticIR and IntentIR:
  - `PREADY` remains the graph-coverage replay target
  - `graph_direction_conflict:actor_completer:PREADY` remains the actor-aware disagreement replay target
- This aligns the flat-hint-present and flat-hint-missing graph-conflict fixtures while preserving their different compatibility-surface expectations.

## Session update (2026-05-05 validator graph-conflict guidance assertions)
- Strengthened `crates/specforge/src/commands/validate.rs` tests for same-actor graph-direction conflicts.
- No production semantics changed; the validator tests now assert that the same report contains both replayable guidance payloads:
  - coverage guidance related to the signal id `PREADY`
  - conflict guidance related to `graph_direction_conflict:actor_completer:PREADY`
- This closes the unit-test layer beneath the project-validation replay preservation tests and the KG fixture assertion, reducing risk that future validator edits drop the coverage guidance while preserving only the conflict-specific guidance.

## Session update (2026-05-05 graph-conflict guidance fixture)
- Strengthened `compat_direction_hints_graph_conflict_incomplete_negative` in the tracked KG fixture set.
- No Rust production code changed; the fixture now asserts both rescan-guidance findings that should accompany a flat-hint-missing graph conflict.
- The fixture now ties three layers together: validation findings, metric counts, and replay guidance related-id payloads.
- This complements the project-validation unit tests by proving the staged pipeline fixture emits the exact guidance that the replay planner is expected to preserve.

## Session update (2026-05-05 IntentIR graph-conflict replay split)
- Added the IntentIR-stage sibling regression in `crates/specforge/src/commands/project_validation.rs`.
- The new test exercises the real upstream replay-input derivation path for IntentIR recommendations: `project-validation` starts from a SemanticIR replay input, loads that artifact, derives the EvidenceIR input, and emits both replay inputs for each recommendation.
- It proves concurrent `intent_graph_direction_coverage_surface_rescan_guidance` and `intent_graph_direction_conflict_surface_rescan_guidance` findings stay as two recommendations with distinct related-id payloads.
- No production code changed, but test coverage now protects both SemanticIR and IntentIR replay-planning surfaces after graph conflicts became graph-coverage debt.

## Session update (2026-05-05 graph-conflict replay split)
- Added a focused regression in `crates/specforge/src/commands/project_validation.rs` for the combined graph-direction conflict plus coverage guidance state.
- No production code changed; the test protects `collect_rescan_recommendations(...)` from collapsing two valid replay recommendations that share one artifact and one replay-input lane.
- The regression matters because graph conflicts now also report coverage debt: operators need both the signal-id replay target and the actor-aware conflict-id replay target to survive into `generated/validation/rescan_plan.json`.
- This strengthens the R15c replay-planning boundary after the R15e fixture work made graph-direction conflicts count as unresolved coverage.

## Session update (2026-05-05 flat-hint graph conflicts)
- Strengthened the tracked expectations for `graph_direction_same_actor_conflict_negative`.
- No Rust production code changed in this slice; the fixture now explicitly asserts that flat compatibility hints remain a separate coverage surface when graph evidence is conflicted.
- The expected validation shape is conflict plus graph-coverage debt, with `semantic_compat_direction_hints_*` / `intent_compat_direction_hints_*` findings excluded because the flat hints are present.
- The metric expectations also lock the mixed state: two resolved directions through graph-or-compat fallback, one graph-covered signal, two compatibility-hinted signals, and one graph conflict.

## Session update (2026-05-05 conflicted graph direction gaps)
- Tightened `crates/specforge/src/commands/validate.rs` so same-actor graph-direction conflicts are not treated as resolved coverage but also no longer disappear from graph-coverage debt.
- `missing_graph_direction_signal_names(...)` now includes every declared/interface signal absent from the resolved graph-direction set, including conflicted signals.
- `unresolved_missing_compat_direction_signal_names(...)` now reports flat-hint-missing conflicted signals as unresolved compatibility gaps instead of suppressing them behind the conflict-only surface.
- The existing conflict-specific findings and `*_graph_direction_conflict_surface_rescan_guidance` remain the actor-aware diagnostic path, with related ids such as `graph_direction_conflict:actor_completer:PREADY`.
- Added `compat_direction_hints_graph_conflict_incomplete_negative` so the tracked KG suite proves this behavior through the staged pipeline, not only through validator unit tests.
- Updated `graph_direction_same_actor_conflict_negative` to keep the flat-hint-present conflict path aligned: conflict evidence still produces graph-coverage debt, but no unresolved compatibility-direction debt is expected while the flat hint exists.

## Session update (2026-05-05 KG fixture for mixed direction-gap states)
- Added `compat_direction_hints_mixed_lag_incomplete_negative` to the tracked KG-quality suite.
- The fixture covers a mixed validation state: `PREADY` is graph-backed but flat-hint-missing, while `PSEL` is flat-hint-missing and graph-uncovered.
- This does not change Rust source behavior; it raises executable truthfulness coverage by proving `SemanticIR` and `IntentIR` keep `*_compat_direction_hints_lag_graph`, `*_compat_direction_hints_incomplete`, and graph-direction coverage related ids scoped to the correct signals in the same artifact.
- The R15 graph-first migration now has fixture coverage for the pure graph-lag case, pure unresolved case, and mixed case.

## Session update (2026-05-05 KG fixture for unresolved direction gaps)
- Added a tracked KG-quality fixture, `compat_direction_hints_incomplete_negative`, for the no-flat-direction/no-actor-graph validation state.
- This is intentionally fixture coverage, not a production-code change: `validate_semantic_ir(...)` and `validate_intent_ir(...)` already expose the split, and the benchmark now locks it in the same staged-pipeline harness used for graph truthfulness.
- The fixture preserves a declared width-bearing signal while clearing its flat compatibility direction hint; with no actor ports, validation must report the `*_compat_direction_hints_incomplete` finding ids and must not report the graph-backed `*_compat_direction_hints_lag_graph` ids.
- This closes a coverage gap left by the previous unit-test-only IntentIR slice and keeps the R15 graph-first migration honest at the executable fixture layer.

## Session update (2026-05-05 FSMGEN submodule machine-contract baseline)
- Fast-forwarded `subs/fsmgen` from `955f2bb` to `32aa318` and kept it read-only from the SPECFORGE parent.
- No SPECFORGE Rust source changed, but the adapter reference surface changed materially: the pinned FSMGEN baseline now has bounded capability-manifest, check-JSON, stable diagnostic-code, normalized semantic JSON, support-accounting/report contract, generated-SystemVerilog validation, embedding, and `HDLGenerator` contract surfaces.
- The `.fsm` adapter design should still remain downstream of canonical `IntentIR`; the new FSMGEN surfaces are validation/reference inputs for target-language truth, not a reason to push `.fsm` semantics backward into `SemanticIR` or `IntentIR`.
- The near-term codebase priority remains the R15 graph-first truthfulness work, but future `.fsm` adapter validation can now be planned against concrete FSMGEN CLI/report surfaces instead of only mdBook/support-accounting prose.

## Session update (2026-05-05 IntentIR validation direction-gap split)
- Continued the R15 graph-first validation cleanup in `crates/specforge/src/commands/validate.rs`.
- `SemanticIR` validation already distinguished graph-backed flat `direction_hint` lag from declared signals that lack both flat direction and actor-relative graph coverage; `IntentIR` now mirrors that split.
- `validate_intent_ir(...)` computes `unresolved_missing_compat_direction_signal_names` for declared canonical signals and emits `intent_compat_direction_hints_incomplete` with signal-level `related_ids` when no flat compatibility hint and no graph direction are available.
- Existing `intent_compat_direction_hints_lag_graph` semantics remain graph-backed only, so unresolved missing graph coverage is no longer collapsed into a graph-lag compatibility finding.
- The focused regression `validate_intent_ir_keeps_incomplete_direction_finding_without_graph_coverage` locks the no-graph/no-flat path; broader verification passed with `521` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` tracked KG fixtures.

## Session update (2026-05-05 `.fsm` FSM undriven graph output lock)
- Continued from commit `a2882b4` by locking the structured-FSM branch of the graph-backed output-drive validation path.
- The production seam was already moved from size-entry-only checks to full `FsmSignalCandidate` inventory checks; this slice adds `structured_fsm_blocks_graph_backed_undriven_output_inventory` so true-FSM lowering cannot regress while the DT branch stays covered separately.
- The fixture clears flat direction hints, adds width-only `UNUSED_TRACE`, recovers its output role from `IntentIR.actor_ports`, and proves the `.fsm` adapter blocks with the typed FSM-state undriven-output diagnostic because no state-body action drives it.
- No production behavior changed; adapter coverage is now `76` tests, and full local verification passed with `520` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` tracked KG fixtures.

## Session update (2026-05-05 `.fsm` graph-backed undriven output inventory)
- Continued from commit `706118e` by moving another `.fsm` renderability consumer onto the graph-first inventory surface.
- Root cause: DT/FSM renderability checked undriven outputs only by walking `FsmRenderableSizeEntry` values that had already been registered through control expressions/actions, so an explicit-but-unreferenced output whose direction came from `IntentIR.actor_ports` could stay out of emitted `+size` entries without producing the intended "not driven" blocker.
- `validate_output_inventory_is_driven(...)` now validates every `FsmSignalCandidate` whose `preferred_signal_direction_hint(...)` resolves to `Output`, covering both flat compatibility directions and actor-relative graph recovery.
- Top-composition child endpoints marked by `module_topology_link` stay composition-scoped: if a top link references a child port that the child module did not emit, the top renderability diagnostic remains the precise "unemitted child port" blocker rather than falsely making the child module non-renderable.
- The new regression `standalone_dt_blocks_graph_backed_undriven_output_inventory` locks the graph-backed standalone case; adapter coverage is now `75` tests, and full local CI passed with `519` Rust tests plus the `127/127` tracked KG fixture suite.

## Session update (2026-05-05 README/COMMIT bootstrap refresh)
- Re-executed `README.md` and `SESSION_BOOTSTRAP.md` for the current session, including the referenced root markdown docs, mdBook chapters, corpus-KB pages, FSMGEN feedback, `COMMIT.md`, and a direct Rust module survey.
- The codebase still matches the documented architecture: one Rust workspace member, `specforge` CLI, Rust `1.95` MSRV, staged `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`, and downstream adapters with `.fsm` as the only active lowering implementation.
- No Rust source behavior changed in this slice; the survey confirms the highest-priority implementation seam remains the existing R15 graph-first transition away from compatibility `direction_hint` consumers, not adapter-family expansion.
- The measured Rust surface is currently `31` source files and `80,456` Rust source lines under `crates/specforge/src`; full validation for this continuity slice passed with `518` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.
- This slice repairs the continuity docs from the stale pre-`9b425e9` handoff state before any next roadmap implementation slice starts.

## Session update (2026-04-30 `.fsm` mixed child root order)
- Continued from commit `67a4d84` by locking mixed DT/FSM child ordering in renderable top source documents.
- The adapter already traverses top children in declaration order and clones each renderable child module into `FsmRenderableSourceDocument.direct_roots`; the new regression proves that this order survives when the first child is `FsmRootKind::Dt` and the second is `FsmRootKind::Fsm`.
- The emitted text assertion covers `(?dtc:producer producer_core)`, `(?fsmc:controller controller_core)`, top-before-direct-root emission, and direct-root order `(?dt:producer_core` before `(?fsm:controller_core`.
- This is a regression-only artifact-boundary slice: no renderability, topology recovery, or source-document modeling behavior changed.
- Adapter coverage increases to `74` tests; full local verification for this slice: `518` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` reused FSM child roots)
- Continued from commit `f2e714d` by locking `renderable_modules_for_top(...)` de-duplication for structured FSM child modules, not only DT child modules.
- The new regression instantiates the same `controller_core` FSM module twice under one top and proves both renderable children remain `FsmRootKind::Fsm` while `FsmRenderableSourceDocument.direct_roots` emits one shared `controller_core` `?fsm:name` root.
- The emitted text assertion counts exactly one `(?fsm:controller_core` root while preserving both `(?fsmc:first controller_core)` and `(?fsmc:second controller_core)` child instances.
- This is a regression-only artifact-boundary slice: no renderability, topology recovery, or source-document modeling behavior changed.
- Adapter coverage increases to `73` tests; full local verification for this slice: `517` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` top documents with FSM children)
- Continued from commit `17435fd` by locking the mixed child-root-kind branch in renderable top source documents.
- The adapter already allowed top children to resolve as either `FsmRootKind::Dt` or `FsmRootKind::Fsm`; the new regression proves that a structured `controller_core` FSM child remains `Fsm` in `FsmTopChildCandidate`, `FsmRenderableTopRoot.children`, and `FsmRenderableSourceDocument.direct_roots`.
- The emitted text assertion covers both `(?fsmc:controller controller_core)` and the child `(?fsm:controller_core ...)` direct root, including system/state content, so top composition is no longer implicitly DT-only in regression coverage.
- This is a regression-only artifact-boundary slice: no topology recovery, renderability, or source-document modeling behavior changed.
- Adapter coverage increases to `72` tests; full local verification for this slice: `516` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` top source-document root order)
- Continued from commit `ec4f991` by locking the final text renderer after the renderable source-document direct-root model was covered.
- `render_fsm_source_document(...)` emits `FsmRenderableSourceDocument.top_root` first, then each child direct root in document order; the new regression proves the final `.fsm` text follows that contract for explicit top composition.
- The fixture checks `(?top:datapath` precedes `(?dt:producer_core`, which precedes `(?dt:consumer_core`, so model order and emitted root order stay aligned.
- This is a regression-only artifact-boundary slice: no renderability, topology recovery, or source-document modeling behavior changed.
- Adapter coverage increases to `71` tests; full local verification for this slice: `515` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` reused child direct-root de-duplication)
- Continued from commit `80c81c5` by locking the reused-child branch in the final renderable source-document direct-root model.
- `renderable_modules_for_top(...)` intentionally de-duplicates child source modules after top renderability succeeds; this prevents repeated child instances from duplicating identical direct module roots in emitted source documents.
- The new regression instantiates `stage_core` twice, proves both renderable top children remain present, and proves `direct_roots` emits one shared `stage_core` `?dt:name` root with the expected size-entry surface.
- The emitted text assertion counts exactly one `(?dt:stage_core` root while preserving both `(?dtc:first stage_core)` and `(?dtc:second stage_core)` child instances.
- Adapter coverage increases to `70` tests; full local verification for this slice: `514` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` renderable top direct roots)
- Continued from commit `68c54a0` by locking the final source-document direct-root list produced for explicit top composition.
- `renderable_modules_for_top(...)` already traverses top children, de-duplicates source modules, and clones each renderable child module into `FsmRenderableSourceDocument.direct_roots`.
- The main renderable top-composition regression now asserts the direct roots are `producer_core` then `consumer_core`, both with `FsmRootKind::Dt`, and that their renderable size-entry surfaces include the child ports consumed by top links.
- This is a regression-only artifact-boundary slice: renderability, child endpoint validation, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `69` tests; full local verification for this slice: `513` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` renderable top-link provenance)
- Continued from commit `4f2bf49` by locking the final renderable top-root link surface for explicit topology evidence.
- `FsmRenderableTopRoot.links` already clones the explicit top links when top renderability succeeds; this slice proves the clone preserves link support IDs and high automation confidence.
- The assertion extends the isolated top-link confidence fixture, so public top-port and child declaration confidence remain low while the child-to-child link carries the high-confidence evidence.
- This is a regression-only artifact-boundary slice: renderability, endpoint validation, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `69` tests; full local verification for this slice: `513` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` top-link root-kind confidence)
- Continued from commit `9795e3e` by locking the explicit topology-link confidence path in the top-root decision fold.
- `build_top_root_kind_decision(...)` already folds `FsmTopCandidate.links[*].automation_confidence`; this slice adds a regression where the only high-confidence evidence is a child-to-child top link.
- The fixture downgrades the public top port and child declarations to low confidence, and the link does not touch the top boundary, so recovered top-port provenance cannot mask the link path.
- `fsm.root_kind_decision.automation_confidence == High` now directly covers the third confidence lane after recovered ports and child declarations.
- Adapter coverage increases to `69` tests; full local verification for this slice: `513` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` top child root-kind confidence)
- Continued from commit `4053491` by locking the top-root decision confidence path for explicit child declarations.
- `build_top_root_kind_decision(...)` already folds `FsmTopCandidate.children[*].automation_confidence`; this slice adds a regression where the only high-confidence evidence is the child declaration.
- The fixture keeps the public top port low confidence and leaves links empty, so `fsm.root_kind_decision.automation_confidence == High` proves the decision followed `FsmTopChildCandidate` evidence rather than recovered top-boundary or topology evidence.
- The child resolves to `?dt:controller_core`, so the confidence source is tied to a real renderable child declaration.
- Adapter coverage increases to `68` tests; full local verification for this slice: `512` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` top child declaration provenance)
- Continued from commit `db40fdf` by locking the top-child declaration artifact surface inside the system-contract distribution fixture.
- The production path already copied `ExplicitTopChildRecord` support IDs and confidence into `FsmTopChildCandidate`; this slice proves the `controller` child keeps that evidence and resolves to `?dt:controller_core`.
- The same assertion checks `FsmRenderableTopRoot.children`, so the final renderable top model preserves the child root kind used for `(?dtc:controller controller_core)` emission.
- This is a regression-only artifact-boundary slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` child renderable system-contract provenance)
- Continued from commit `240b5cc` by locking the top-composition child renderable module system-contract surface.
- The production path already cloned child `SystemContractRecord` values into explicit module renderable modules; this slice proves the child `controller_core` renderable module keeps support IDs and high confidence alongside the child clock/reset signal inventory.
- The same assertion now covers `renderable_document.direct_roots[*].module`, so the final source-document boundary used for emitted child roots preserves the child system-block evidence.
- This is a regression-only artifact-boundary slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` standalone renderable system-contract provenance)
- Continued from commit `5c715d6` by locking the final renderable standalone system-contract surfaces.
- The production path already cloned `SystemContractRecord` into renderable modules; this slice proves that support IDs and high confidence survive in both `fsm.renderable_module` and `renderable_document.direct_roots[*].module`.
- The assertion covers direct sequential shape recovery, direct sequential materialization from system-contract-only clock/reset facts, and standalone explicit-module recovery.
- This is a regression-only artifact-boundary slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` standalone system signal provenance)
- Continued from commit `2a1114e` by locking the direct standalone system-contract signal recovery surfaces.
- The production path already copied `SystemContractRecord` support IDs and confidence into recovered/materialized standalone clock/reset `FsmSignalCandidate` entries; this slice proves those values stay visible on direct sequential `clk` / `rst_n` candidates when shape hints are cleared or flat signal records are absent.
- The same invariant is now asserted for standalone explicit-module lowering, so `controller.clk` and `controller.rst_n` keep contract provenance even when module-local interface shape hints are cleared.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` child system endpoint provenance)
- Continued from commit `57ac516` by locking the child module endpoint side of the system-contract provenance chain.
- The production path already copied `SystemContractRecord` support IDs and confidence into materialized child clock/reset `FsmSignalCandidate` entries; this slice proves those values stay visible on `controller_core.clk` and `controller_core.rst_n`.
- The focused assertion extends the existing top system-contract distribution test, so it now covers child endpoint provenance before resolved top ports, selected top inventory, and the renderable `?top:soc` document model consume it.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` resolved top system-port width provenance)
- Continued from commit `8beb670` by locking the resolved top-candidate port sibling of the child-system-contract width provenance surface.
- The production path already merged top width evidence into `FsmTopCandidate.ports`; this slice proves that child-system-contract endpoint recovery leaves top-link support IDs and high confidence visible on resolved top `clk` / `rst_n`.
- The focused assertion extends the existing top system-contract distribution test, so it covers resolved top ports, selected top inventory, and the renderable `?top:soc` document model together.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` selected top system-port width provenance)
- Continued from commit `ade21cc` by locking the selected top inventory sibling of the child-system-contract width provenance surface.
- The production path already merged top width evidence into `FsmSignalCandidate` entries; this slice proves that child-system-contract endpoint recovery leaves `module_topology_link`, top-link support IDs, and high confidence visible on selected top `clk` / `rst_n`.
- The focused assertion extends the existing top system-contract distribution test, so it covers selected top inventory and the renderable `?top:soc` document model together.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` renderable top-root system-port width provenance)
- Continued from commit `661f019` by locking the child-system-contract endpoint sibling of the renderable top-root width provenance surface.
- The production path already cloned resolved top ports into `FsmRenderableTopRoot.ports`; this slice proves that top-link recovered clock/reset width, support IDs, and confidence survive into `renderable_document.top_root.ports`.
- The focused assertion extends the existing top system-contract distribution test, so it covers the final `?top:soc` top-root document model consumed by `.fsm` text rendering.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` renderable top-root actor-port width provenance)
- Continued from commit `b9bd0a9` by locking the actor-port width sibling of the renderable top-root provenance surface.
- The production path already cloned resolved top ports into `FsmRenderableTopRoot.ports`; this slice proves that actor-port-recovered width, support IDs, and confidence survive into `renderable_document.top_root.ports`.
- The focused assertion extends the existing top actor-port width recovery test, so it covers the final top-root document model consumed by `.fsm` text rendering.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` renderable top-root top-link width provenance)
- Continued from commit `fb4f050` by locking the width sibling of the explicit top-link topology renderable provenance surface.
- The production path already cloned resolved top ports into `FsmRenderableTopRoot.ports`; this slice proves that topology-recovered width, support IDs, and confidence survive into `renderable_document.top_root.ports`.
- The focused assertion extends the existing child-link top-port width recovery test, so it covers the final top-root document model consumed by `.fsm` text rendering.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-30 `.fsm` renderable top-root top-link provenance)
- Continued from commit `e705a95` by locking the explicit top-link topology sibling of the renderable top-root provenance surface.
- The production path already cloned resolved top ports into `FsmRenderableTopRoot.ports`; this slice proves that topology-recovered direction, support IDs, and confidence survive into `renderable_document.top_root.ports`.
- The focused assertion extends the existing explicit top-link recovery test, so it covers the final top-root document model consumed by `.fsm` text rendering.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` renderable top-root port provenance)
- Continued from commit `78ed684` by locking the renderable-document consumer of recovered top-port provenance.
- The production path already cloned resolved top ports into `FsmRenderableTopRoot.ports`; this slice proves that the recovered actor-port direction, support ID, and confidence survive into `renderable_document.top_root.ports`.
- The focused assertion extends the existing renderable actor-port recovery test, so it covers the final top-root document model consumed by `.fsm` text rendering.
- This is a regression-only artifact-surface slice: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Adapter coverage remains at `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` top-root confidence regression)
- Continued from commit `224e908` by locking the root-kind decision consumer of recovered top-port confidence.
- The production path was already corrected by resolved top-port provenance merging; this slice adds a minimal top-only regression so future refactors cannot accidentally make `build_top_root_kind_decision(...)` fold raw top-port confidence again.
- The fixture keeps children and links absent, sets the raw explicit top port to low confidence, and lets high-confidence actor-port recovery supply the direction evidence.
- `fsm.root_kind_decision.automation_confidence == High` now proves the selected `?top:name` decision follows recovered top-port evidence through `FsmTopCandidate.ports`.
- Adapter coverage increased to `67` tests; full local verification for this slice: `511` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` resolved top-port provenance)
- Continued from commit `30e6a50` by tightening the resolved top-port artifact surface after selected top inventory began carrying recovered confidence.
- The root cause was that `analysis.resolved_ports` cloned raw explicit top ports, replaced direction/width hints with recovered evidence, but did not merge recovered supporting IDs or automation confidence into the `ExplicitTopPortRecord`.
- Added `merge_resolved_top_port_provenance(...)`, which merges graph-direction and width-evidence supporting IDs and max-folds their confidence into the resolved port.
- The same resolved vector feeds `FsmTopCandidate.ports` and `renderable_top.ports`, so both artifact surfaces now match the selected inventory provenance story.
- The change is artifact-shape only: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- The actor-port direction assertion failed before the fix because the recovered top port did not carry `graph_wrapper_ext_data`; focused sibling lanes and the full adapter suite now pass with `66` tests.
- Full local verification for this slice: `510` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` selected top confidence provenance)
- Continued from commit `75b580a` by tightening selected top automation-confidence projection after recovered categories and support IDs were preserved.
- The root cause was that selected top inventory still used `resolved_port.automation_confidence` after merging recovered direction/width provenance from actor ports and top links.
- `TopPortDirectionEvidence` and `TopPortWidthEvidence` now carry `AutomationConfidence` through the same ledgers that carry recovered/conflicted state, source categories, and supporting canonical IDs.
- Actor-port top direction/width recovery contributes actor-port confidence; top-link direction and child-endpoint width recovery contribute explicit top-link confidence.
- `build_top_signal_inventory(...)` now max-folds explicit/declaration, graph-direction, and width-evidence confidence into the selected `FsmSignalCandidate`.
- The change is artifact-shape only: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- The actor-port direction confidence assertion failed before the fix and now passes; sibling topology direction and actor/topology width assertions passed after the implementation. The full adapter suite passed with `66` tests.
- Full local verification for this slice: `510` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` selected top support-ID provenance)
- Continued from commit `0d2c9df` by tightening selected top support-ID projection after the recovered category lanes landed.
- The root cause was that selected top inventory merged recovered direction/width source categories but left `supporting_canonical_ids` tied to `resolved_port.supporting_statement_ids` only.
- `TopPortDirectionEvidence` and `TopPortWidthEvidence` now carry support IDs in the same evidence ledgers that carry recovered/conflicted state and mention categories.
- Actor-port top direction/width recovery contributes actor-port support IDs; top-link direction and child-endpoint width recovery contribute explicit top-link support IDs with a link-id fallback.
- `build_top_signal_inventory(...)` now merges explicit top-port support IDs, graph-direction evidence support IDs, and width evidence support IDs into selected `FsmSignalCandidate`.
- The change is artifact-shape only: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Actor-port direction and child-link width support-ID assertions failed before the fix and now pass; focused top-composition coverage passed with `27` tests, and the full adapter suite passed with `66` tests.
- Full local verification for this slice: `510` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` selected top direction provenance)
- Continued from commit `8783350` by tightening selected top direction provenance after the width-source category fix.
- The root cause was that `TopPortDirectionEvidence` tracked resolved/conflicted graph-backed direction state but no source category, so selected top inventory could expose `graph_direction_hint` without carrying whether the graph evidence came from top actor ports or explicit top-link topology.
- `TopPortDirectionEvidence` now carries `mention_categories` for graph-side recovery evidence.
- Top actor-port direction recovery records `actor_port`, top-link direction recovery records `module_topology_link`, and `build_top_signal_inventory(...)` projects those categories into selected `FsmSignalCandidate` entries.
- The fix is artifact-shape only: renderability, conflict behavior, and emitted `.fsm` text remain unchanged.
- Actor-port and blocked top-link topology provenance assertions failed before the fix and now pass; focused top-composition coverage passed with `27` tests, and the full adapter suite passed with `66` tests.
- Full local verification for this slice: `510` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` selected top width provenance)
- Continued from commit `e92fe35` by tightening artifact provenance for recovered selected top widths.
- The root cause was that `TopPortWidthEvidence` carried resolved/conflicted width state but no source category, and `build_top_signal_inventory(...)` always rebuilt selected top `mention_categories` as plain `top_port`.
- `TopPortWidthEvidence` now carries mention categories through its merge path, tagging duplicate top declarations as `top_port`, actor-port width evidence as `actor_port_width`, and child/top-link topology evidence as `module_topology_link`.
- Selected `FsmSignalCandidate` entries now merge those categories, so recovered top-boundary width evidence remains visible in adapter artifacts without changing renderability or emitted `.fsm` text.
- The actor-port provenance assertion failed before the fix and now passes; a sibling child-link topology assertion locks the other recovered-width lane.
- Focused top-composition coverage passed with `27` tests, and the full adapter suite passed with `66` tests.
- Full local verification for this slice: `510` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` top system-contract endpoint coverage)
- Continued from commit `4846128` by locking the top-composition consumer of materialized child system-contract ports.
- The new regression proves a child module with only `SystemContractRecord` clock/reset facts still exposes `controller.clk` and `controller.rst_n` as emitted child endpoints for top-link analysis.
- Widthless public top `clk` / `rst_n` inputs now have explicit coverage for recovering 1-bit widths from those child endpoints and rendering as bare 1-bit input public IO.
- This is a coverage-hardening slice: production behavior from the previous system-contract materialization change already made the path work, and the new test prevents future endpoint or top-width recovery refactors from silently breaking it.
- Focused top-composition coverage passed with `27` tests, and the full adapter suite passed with `66` tests.
- Full local verification for this slice: `510` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` system-contract signal materialization)
- Continued from commit `7bab72c` by tightening the system-contract-to-signal-inventory boundary.
- The root cause was that `overlay_system_contract_signal(...)` returned early unless the clock/reset signal already existed in inventory, despite `SystemContractRecord` being canonical evidence for those signal roles.
- The overlay now always calls the conflict-aware canonical registration helper, so absent system clock/reset signals are materialized as input, 1-bit `system_contract_signal` entries and existing contradictory evidence still blocks through the existing merge logic.
- The new regression removes flat `clk` / `rst_n` signal records from a sequential `IntentIR` while retaining the system contract, proving the adapter can still emit `(+system ...)` without inventing any non-contract facts.
- Focused system-contract recovery/conflict tests and the full adapter suite passed; adapter coverage now has `65` tests.
- Full local verification for this slice: `509` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` actor-port parametric width provenance)
- Continued from commit `d22ae1f` by applying the parametric-width provenance split to actor-port graph overlays.
- The root cause was that actor-port inventory overlays still converted width evidence through `WidthHint::as_numeric()`, dropping symbolic graph widths such as `DATA_WIDTH` before `FsmSignalCandidate` projection.
- The adapter now uses a separate recovered-parametric-width registration path for actor ports: symbolic actor-port widths are preserved only when the inventory has no numeric width and no width conflict, while canonical parametric signal declarations keep their stricter always-visible behavior.
- Actor-port overlays now register numeric width evidence before recovered symbolic width evidence, which keeps numeric recovery authoritative and avoids order-dependent symbolic blockers.
- Renderability now reports actor-port parametric width evidence as a parametric-width blocker instead of a missing-width fallback.
- Focused coverage locks the blocked symbolic actor-port case, the explicit-numeric-over-symbolic actor-port guard, existing numeric actor-port recovery lanes, and the full adapter suite now has `64` tests.
- Full local verification for this slice: `508` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` parametric signal width provenance)
- Continued from commit `c52f07a` by tightening width provenance for direct/module signal inventories.
- The root cause was that non-top `InterfaceSignalRecord.width_hint` values were collapsed through `WidthHint::as_numeric()`, so symbolic widths such as `DATA_WIDTH` were indistinguishable from absent width evidence by the time `.fsm` renderability ran.
- `SignalInventoryEvidence` and `FsmSignalCandidate` now carry `parametric_width_hint` alongside numeric width and width-conflict state.
- `register_renderable_signal(...)` now blocks symbolic signal widths with a parametric-width diagnostic before emitting missing-width guidance, and system-contract width checks use the same priority.
- `build_top_signal_inventory(...)` now preserves parametric top width text in selected artifacts too, matching the already-conservative top-public-IO blocker.
- A new regression locks the direct-root case where `DATA_IN` is declared with `DATA_WIDTH`; adapter coverage now has `62` tests.
- Full local verification for this slice: `506` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` flat direction conflict provenance)
- Continued from commit `a9a322e` by tightening another renderability boundary in the adapter signal inventory.
- The root cause was that flat canonical direction conflicts existed in `SignalInventoryEvidence.direction_hint_conflicted` but were dropped when projecting `FsmSignalCandidate`.
- Because `preferred_signal_direction_hint(...)` only knew about graph-direction conflicts, an unambiguous actor-port graph could still provide a renderable direction for a signal whose explicit canonical declarations disagreed.
- `FsmSignalCandidate` now preserves `direction_hint_conflicted`, and renderability checks block conflicting flat direction evidence before accepting graph recovery or reporting generic missing-direction diagnostics.
- System-contract diagnostics now also distinguish conflicting canonical direction evidence from missing clock/reset direction evidence.
- A new regression locks the unsafe case: duplicate flat `DATA_OUT` directions plus an unambiguous `controller.DATA_OUT` actor-port graph now stays blocked instead of rendering. Adapter coverage now has `61` tests.
- Full local verification for this slice: `505` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` top-link child endpoint emission gate)
- Continued from commit `b9f9c27` by tightening the child side of top-composition renderability.
- The root cause was a mismatch between analysis and rendering: top-link resolution used `FsmSignalCandidate` inventory entries, but `render_fsm_module(...)` only emits actual child signal surfaces from `FsmRenderableModule.size_entries` plus system-contract ports.
- `renderable_ports_for_module_candidate(...)` now derives child-link-visible ports from the emitted child module surface instead of advisory inventory.
- Missing top-link endpoint diagnostics now distinguish explicit top-port absence from child endpoints that do not resolve to emitted ports on renderable child modules.
- A new regression proves a top link to a declared-but-unemitted child signal stays blocked; top-composition coverage now has `26` tests, and the full adapter suite now has `60` tests.
- Full local verification for this slice: `504` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` top public IO width renderability gate)
- Continued from commit `af3962a` by tightening a top-composition safety seam adjacent to the width-conflict work.
- The root cause was that top renderability validated direction recovery but did not require a numeric public IO width before `render_top_port_token(...)` ran. Because that renderer prints widthless top inputs/outputs as implicit 1-bit ports, a direction-only or parametric top port could emit misleading `.fsm` text.
- `analyze_top_renderability(...)` now calls `validate_top_port_width_renderability(...)` after top actor-port and child-link width recovery, so valid recovery paths still complete before the gate fires.
- The new gate blocks missing, conflicted, and parametric top-boundary width evidence for public IO emission while preserving explicit numeric, top actor-port, and child-link topology recovery paths.
- Two focused regressions first reproduced the unsafe renderability result, then passed after the validator landed. Top-composition coverage now has `25` tests, and the full adapter suite now has `59` tests.
- Full local verification for this slice: `503` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` width-conflict artifact provenance and diagnostics)
- Continued from commit `b6255cf` by applying the same conflict-vs-missing renderability split to width evidence.
- The root cause was a projection gap: `SignalInventoryEvidence` carried `width_hint_conflicted`, but `FsmSignalCandidate` only serialized the resolved numeric `width_hint`. Once a width conflict collapsed to `None`, downstream diagnostics could not distinguish conflict from absence.
- `FsmSignalCandidate` now has a defaulted `width_hint_conflicted` flag. Direct/module inventory projection copies it from `SignalInventoryEvidence`, and selected top inventory projects it from `TopPortWidthEvidence`.
- `register_renderable_signal(...)` and `validate_system_signal_renderability(...)` now emit conflicting-width blockers before the generic missing-width guidance, while still refusing to render any conflicted width.
- A new system-contract width conflict regression complements existing direct, explicit-module, top-boundary, and child-topology width conflict coverage.
- Focused width-conflict tests plus the full adapter test slice passed for this change.
- Full local verification for this slice: `501` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` graph-conflict renderability diagnostics)
- Continued from commit `72d47f0` by cleaning up a diagnostic consumer rather than changing adapter renderability semantics.
- The root cause was that `preferred_signal_direction_hint(...)` intentionally returns `None` for graph-conflicted signals, but downstream renderability checks treated every `None` as a missing direction hint.
- `register_renderable_signal(...)` now emits a graph-conflict-specific blocker when `FsmSignalCandidate.graph_direction_hint_conflicted` is set, preserving the generic missing-direction message only for genuinely absent direction evidence.
- `validate_system_signal_renderability(...)` mirrors that split for system-contract clock/reset checks, and top renderability now identifies conflicted top-boundary direction evidence separately from missing top-port recovery.
- Regression assertions now confirm graph-conflicted direct, module, and child-topology inventory entries retain the sticky conflict bit and expose conflict wording in blocking reasons.
- Focused conflict tests plus the full adapter test slice passed for this change.
- Full local verification for this slice: `500` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` selected top inventory graph-conflict stickiness)
- Continued from commit `f2618b4` by tightening the selected top inventory projection that had just stopped flattening recovered graph/topology directions into compatibility hints.
- The root cause was that selected top inventory only compared raw and resolved top-port directions. When explicit flat top direction evidence conflicted with graph/topology evidence, renderability correctly collapsed the resolved top port to `None`, but `fsm.signal_inventory` did not preserve the explicit declaration side or mark the graph conflict.
- `analyze_top_renderability(...)` now keeps declared top-port direction evidence separate from graph/topology top-boundary direction evidence while still feeding combined evidence into the renderability blocker.
- `merge_top_port_direction_hint(...)` centralizes the direction-evidence mutation so the graph-only ledger and combined ledger share the same sticky conflict behavior without double-emitting diagnostics.
- `build_top_signal_inventory(...)` now projects selected top signals like module/direct signals: explicit top declarations stay in `direction_hint`, graph-only recovery stays in `graph_direction_hint`, and graph-vs-explicit disagreement sets `graph_direction_hint_conflicted` while leaving rendering blocked.
- Focused graph-vs-flat conflict tests, full top-composition coverage, and the full adapter test slice passed for this change.
- Full local verification for this slice: `500` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 `.fsm` selected top inventory graph-backed direction provenance)
- Continued from commit `65fcf4b` with another graph-first consumer cleanup, this time in the `.fsm` adapter top-surface projection.
- The root cause was that `select_fsm_surface(...)` rebuilt selected top `signal_inventory` from already-resolved top ports. Directions recovered from top-link topology or top actor ports therefore appeared as flat compatibility `direction_hint` values in `fsm.signal_inventory`.
- `FsmTopCandidate` now carries the top-surface `signal_inventory` computed during `analyze_top_renderability(...)`, while both raw top declarations and resolved top ports are available.
- `build_top_signal_inventory(...)` now keeps explicit raw top-port directions as flat compatibility hints, but stores directions recovered for width-only raw top ports in `graph_direction_hint`.
- The emitted/renderable top root still uses resolved top ports, so `.fsm` text remains renderable; the artifact inventory simply stops misclassifying recovered graph/topology evidence as an original flat declaration.
- Focused top-link, top actor-port, blocked top, full top-composition, and full adapter tests passed for this slice.
- Full local verification for this slice: `500` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 semantic compat-direction validation split)
- Continued from commit `ce6d875` by tightening a validation-only graph-first seam rather than changing IR construction or adapter lowering.
- The root cause was a semantic-stage diagnostic mismatch: `validate_semantic_ir(...)` already counted actor-relative graph directions as resolved direction coverage, but still reported graph-resolved flat-hint gaps under `semantic_compat_direction_hints_incomplete`.
- `crates/specforge/src/commands/validate.rs` now separates graph-backed flat-hint lag from genuinely unresolved direction evidence:
  - `semantic_compat_direction_hints_lag_graph` is emitted only when non-conflicted actor-relative graph coverage exists and the flat compatibility `direction_hint` is missing
  - `semantic_compat_direction_hints_incomplete` is retained for signals with no flat hint and no non-conflicted graph coverage
  - graph-conflicted signals stay on the graph-direction conflict surface instead of being reclassified as ordinary compatibility lag
- `crates/specforge/src/commands/kg_bench.rs` and `crates/specforge/test_data/kg_quality/compat_direction_hints_lag_graph_negative/fixture.json` were updated so the tracked fixture locks the new semantic-stage finding ID.
- This does not change the live protocol validation projection, but it narrows one remaining direct compatibility-hint consumer in validation and keeps future graph-first work from chasing a misleading incomplete-direction finding.
- Full local verification for this slice: `500` Rust tests, warning-deny Clippy/rustdoc, mdBook validation, and `127/127` KG fixtures.

## Session update (2026-04-29 README/bootstrap restart and corpus-KB projection refresh)
- Re-executed the README handoff path through `SESSION_BOOTSTRAP.md`, the root continuity docs, the canonical mdBook, corpus-KB pages, FSMGEN feedback, and the active Rust crate layout.
- The codebase survey still matches the documented architecture: one Rust workspace member, `specforge` CLI, staged `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`, downstream adapters, and active truthfulness surfaces in `semantic.rs`, `validate.rs`, `adapters.rs`, `project_validation.rs`, and `kg_bench.rs`.
- The top `.fsm` adapter code confirms the latest committed topology slice: explicit top endpoint widths are seeded from declared top ports and child module signals, propagated through links to a fixed point, and kept conservative by locking declared widths and collapsing propagated conflicts.
- The restart uncovered documentation drift rather than production-code drift:
  - `MEMORY.md` still pointed at commit `3598999` and described the transitive topology slice as in flight even though `d4f53bb` already landed it.
  - the corpus-KB benchmark projection was stale at `92/92` fixtures while the executable `kg-bench` command now proves `127/127`.
- The managed corpus-KB fixture pages were refreshed through `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality`, keeping review projections tied to executable fixture output instead of manual edits.
- Current implementation size after the restart: `31` Rust source files and `77,934` lines under `crates/specforge/src`.
- Current local verification baseline: `499` Rust tests, `127/127` KG fixtures, warning-deny Clippy/rustdoc, and mdBook validation.
- No new architectural pivot is needed from this restart; the next roadmap work should continue the existing graph-first/actor-relative adapter and validation direction rather than widening backend scope.

## Session update (2026-04-29 `.fsm` transitive top-link width recovery)
- Continued from commit `3598999` by turning the adjacent one-hop topology-width recoveries into a bounded explicit-top endpoint-width closure.
- The root cause was that `collect_module_topology_port_directions(...)` recovered child widths from directly connected top or sibling child endpoints, but did not make newly recovered endpoint widths visible to other links in the same top composition.
- `crates/specforge/src/ir/adapters.rs` now models top ports and child module signals as endpoint-width keys, seeds declared numeric widths, and iterates links to a fixed point before emitting module topology overlays.
- Declared endpoint widths remain locked; propagated conflicts collapse inferred width evidence instead of choosing a convenient side.
- Module inventories still receive the connected peer's resolved width as link-compatibility evidence, preserving the previous blocking behavior for contradictory child/top declarations.
- Focused coverage now proves the formerly blocked transitive recovery path, and the topology suite proves sibling/top conflicts still block honestly.
- Current adapter implementation size after this slice: `9,143` lines in `crates/specforge/src/ir/adapters.rs`.
- Full local CI baseline for this slice: `499` Rust tests plus warning-deny Clippy/rustdoc and the mdBook build.

## Session update (2026-04-29 `.fsm` sibling child-link source-width coverage and SourceIR test isolation)
- Continued from commit `b7274bb` with a focused coverage-hardening pass instead of changing production topology behavior.
- The previous slice made child-to-child width propagation symmetric in code, but only the target-child recovery direction had positive coverage.
- Added a mirror adapter regression where a widthless source child output recovers width from a connected target child input.
- This locks the intended bidirectional behavior for future refactors of `collect_module_topology_port_directions(...)`.
- Full CI also exposed a SourceIR/Docling test isolation bug: SourceIR PDF tests used a private environment mutex while Docling runtime tests used the shared `test_support::env_var_lock()`, so parallel PATH mutation could hide standard shell tools from the stub backend helper.
- `ir::source::tests` now uses the same shared lock, aligning the safety comments with the actual cross-module serialization boundary.

## Session update (2026-04-29 `.fsm` sibling child-link width recovery)
- Continued from commit `b462600` by closing the remaining numeric-width hole in first-slice explicit top links.
- The root cause was that `collect_module_topology_port_directions(...)` only used top-boundary endpoint widths when attaching width evidence to child module ports.
- Child-to-child links therefore recovered direction, but a widthless child input linked to a sibling child output with numeric width still blocked as widthless.
- The topology collector now builds a numeric width map from explicit module signal records, then uses the opposite child endpoint as width evidence for child-to-child links.
- Focused adapter coverage proves positive child-to-child width recovery and conflicting sibling-derived width blocking.

## Session update (2026-04-29 `.fsm` top-link top-boundary width recovery)
- Continued from commit `e9a91dd` by closing the mirror of the previous child-width topology recovery.
- The root cause was a top-boundary projection asymmetry: explicit top links could recover top port direction, and the connected child endpoint could carry numeric width, but the top boundary width evidence stayed unresolved and rendered as a widthless one-bit-looking port.
- `analyze_top_renderability(...)` now merges numeric child endpoint widths back into existing top-port width evidence after child modules resolve and before final top-link validation.
- The merge only considers top-boundary-to-child links, refreshes the selected top signal inventory before projection, and keeps contradictory child endpoint widths blocking through the existing top-port width conflict path.
- Focused adapter coverage proves width recovery for a widthless top port and conflict blocking when two child endpoints imply different widths for the same top boundary port.

## Session update (2026-04-29 `.fsm` explicit module actor-port width recovery)
- Continued from commit `7b266d2` by applying the same width-vs-direction separation to explicit module candidates.
- The root cause matched the direct-root slice: module-control reads could recover input direction, but missing input width stayed unresolved when only an external actor-port graph record carried the numeric width.
- `build_module_candidate(...)` now applies the width-only actor-port overlay before module actor-port direction evidence and module-control input recovery are analyzed.
- The overlay still skips non-inventoried signals, records `actor_port_width` provenance, ignores actor-port direction, and keeps contradictory graph widths blocking through the existing width-conflict collapse path.
- Focused adapter coverage proves explicit-module control-input width recovery and conflicting external-width blocking.

## Session update (2026-04-29 `.fsm` direct actor-port width recovery)
- Continued from commit `abc3405`, still working the adapter shape boundary rather than widening backend targets.
- The root cause was a direct-root evidence asymmetry: control reads could recover target-actor input direction, but missing input width stayed unresolved even when actor-port graph evidence already carried the numeric signal width.
- Direction is actor-relative, so external actor ports must not define the selected root perspective; width is signal shape, so direct roots can safely consume it independently for already-inventoried signals.
- `build_signal_inventory(...)` now applies a width-only actor-port overlay before direct-root renderability analysis.
- The overlay records `actor_port_width` provenance, does not merge actor-port direction, skips non-inventoried signals, and reuses the existing width-conflict collapse behavior for contradictory graph widths.
- Focused adapter coverage proves the positive control-input width recovery path and the contradictory external-width blocking path.

## Session update (2026-04-29 `.fsm` child topology width recovery)
- Continued the graph/topology-first adapter thread from commit `bba3a59`.
- The root cause was another shape-evidence asymmetry: explicit top-link topology could recover child module port direction, and the opposite top-boundary endpoint could already carry numeric width, but that width was not fed into the child module signal inventory.
- A widthless child output linked to an 8-bit top output therefore stayed blocked as widthless even though the composition graph already had enough numeric shape to render the child module honestly.
- `ModuleTopologyPortDirection` now carries optional numeric width evidence, sourced only from the opposite top-boundary endpoint on a top link.
- Child-child links still do not invent widths, parametric top widths remain deferred, and contradictory explicit child width versus topology width collapses to unresolved through the existing signal-inventory conflict behavior.
- Focused adapter coverage now proves topology-backed child-width recovery and conflicting topology-width blocking.

## Session update (2026-04-29 `.fsm` top boundary actor-port width recovery)
- Continued the graph-first top-boundary adapter thread from commit `096438d`.
- The root cause was a shape/provenance mismatch: top actor-port graph evidence could now recover direction, but matching actor-port width was still ignored by explicit top-root analysis.
- That meant a top boundary port declared with direction but no width could render as an implicit 1-bit `.fsm` public IO even when the matching `IntentIR.actor_ports` record already carried a numeric width.
- `merge_top_port_evidence_from_actor_ports(...)` now merges both direction and width from actor ports whose actor matches the explicit top name and whose signal is already a declared top port.
- The same bounded conflict behavior applies to width: explicit top width versus top actor-port width disagreement collapses the resolved width to `None` and keeps `.fsm` lowering blocked.
- Focused adapter coverage now proves graph-backed width recovery and contradictory graph width blocking.

## Session update (2026-04-29 `.fsm` top boundary actor-port direction recovery)
- Continued the live `R15` graph-first adapter work instead of widening backend scope.
- The targeted remaining consumer was explicit top-root boundary direction analysis in `crates/specforge/src/ir/adapters.rs`.
- Before this slice:
  - explicit module ports could consume matching module actor ports
  - top boundary ports could consume explicit top-link topology
  - but width-only top ports could not consume actor-port graph facts for the explicit top actor itself
- `build_top_candidates(...)` now carries `IntentIR.actor_ports` into top renderability analysis, and top analysis merges only actor ports whose `actor_name` matches the `ExplicitTopRecord.top_name` and whose signal already exists as a declared top port.
- Conflicts stay honest: if the explicit top-port direction says one role and the top actor graph says the opposite, the resolved top port direction collapses to `None` and `.fsm` lowering remains blocked.
- Focused adapter coverage now proves both the positive recovery path and the contradictory graph path.

## Session update (2026-04-29 runtime doctor cold-load tolerance)
- Picked the next bounded task from the README execution findings rather than widening adapter scope: `specforge doctor --strict` could false-negative a healthy local Ollama when the first chat probe canceled during cold model load.
- The root cause was in `crates/specforge/src/commands/doctor.rs`:
  - tag/model-list probes were healthy
  - the OpenAI-compatible chat probe remained the correct proof of real provider usability
  - but the previous 5-second chat timeout was shorter than a realistic first `qwen2.5vl:7b` local runner load
- The fix keeps the readiness boundary strict:
  - local chat probes still require a real `/v1/chat/completions` response
  - the timeout is now cold-load tolerant
  - curl timeout/connection failures are surfaced directly instead of being flattened into empty-response parse diagnostics
- This does not change the IR architecture, provider defaults, or canonical artifact shape.
- It improves the project’s local-first operator seam so README/bootstrap runs can rely on `doctor --strict` without manual model prewarming.
- Focused doctor-unit coverage now locks the cold-load timeout margin and curl failure-detail formatting; the full local CI baseline now reports `483` passing Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-29 cycle-qualified VLM timing-value label hardening)
- Continued from commit `b175d83`, still hardening the graph/temporal evaluation surface rather than widening adapters or semantic schemas.
- The relevant Rust seam remains `parse_timing_diagram_observation(...)` in `crates/specforge/src/ir/semantic.rs`, where VLM timing annotations are promoted into `TimingConstraintRecord`s unless one of the low-value label filters rejects them first.
- The prior slices had already covered three adjacent noise classes:
  - bare timing/sample/index labels
  - indexed/ranged signal-value labels
  - motion-only waveform prose and waveform-motion states
- But the signal-value filter still had a precise parser-shape blind spot:
  - it treated bare `SIGNAL VALUE` annotations as low-value noise
  - but once the same label carried only a trailing cycle marker like `at T1`, `on T1`, or `during T0`, the filter stopped matching and the annotation could leak through as a fake timing constraint
- This slice closes that parser-gap at the root by extending `is_signal_value_annotation_label(...)` so it still recognizes low-value signal-value annotations when the only trailing tokens form a cycle marker label:
  - `T1`
  - `at T1`
  - `on T1`
  - `during T0`
  - `in T0`
- The safety profile stays aligned with the project’s broader design:
  - the change is local to VLM timing-annotation rejection
  - it does not invent new canonical truth
  - it preserves the concrete waveform samples as `SignalConstraintRecord`s
  - it only prevents low-value VLM labels from being mis-upgraded into `TimingConstraintRecord`s
- Proof coverage now exists in both lanes that matter:
  - a direct semantic regression for the parser/filter seam
  - a tracked KG negative fixture for end-to-end validation
- The tracked KG-quality suite rises by one fixture because this is another new evaluation-family edge case, not just a direct-lane hardening pass.
- Focused semantic regression coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline rises to `481` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite now stands at `127` fixtures.

## Session update (2026-04-29 indexed VLM timing-value label hardening)
- Continued from the current graph/temporal/eval hardening track rather than widening adapters or semantic schemas.
- The relevant Rust seam lives in `crates/specforge/src/ir/semantic.rs` inside `parse_timing_diagram_observation(...)`, where VLM timing annotations are promoted into `TimingConstraintRecord`s unless one of the low-value label filters rejects them first.
- The existing filter stack already handled three neighboring classes well:
  - bare timing/sample/index labels (`T0`, `Addr 1`, `XREQ[0]`)
  - bare signal-value labels (`XREQ HIGH`, `XREQ asserted`)
  - motion-only waveform prose and waveform-motion states
- But it still left a narrow parser-gap between the first two classes:
  - indexed/ranged signal-value labels like `XREQ[0] HIGH` or `XREQ[3:0] asserted`
  - those strings are not real timing constraints, but the old filter path treated the indexed token as an unknown identifier and let the whole annotation survive as a fake timing constraint description
- This slice closes that parser-gap at the root by teaching the signal-value label filter to normalize indexed/ranged signal tokens back to their known base signal when the bracket/range payload is itself just a compact waveform index.
- The safety profile stays aligned with the project’s broader design:
  - the change is local to VLM timing-annotation rejection
  - it does not invent new canonical truth
  - it preserves the concrete waveform samples as `SignalConstraintRecord`s
  - it only prevents low-value VLM labels from being mis-upgraded into `TimingConstraintRecord`s
- Proof coverage now exists in both lanes that matter:
  - a direct semantic regression for the parser/filter seam
  - a tracked KG negative fixture for end-to-end validation
- The tracked KG-quality suite rises by one fixture because this is a new evaluation-family edge case, not just a direct-lane hardening pass.
## Session update (2026-04-22 generic next-cycle direct lexical hardening)
- Continued from commit `cbb4036`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic next-cycle family was already part of the parser and extraction design surface:
  - `next cycle`
  - `next clock cycle`
  - `following cycle`
  - `subsequent cycle`
- The tracked KG-quality benchmark corpus already locked those spellings inside `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/`.
- But the direct proof lane still only locked the canonical extractor spelling and had no semantic or validator regression for the full lexical quartet.
- This slice expands the direct extractor regression, semantic regression, and validator regression so the supported generic next-cycle lexical surface is protected before and alongside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a direct-lane hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused extractor coverage, focused semantic coverage, focused validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline rises to `479` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 next-tick direct lexical hardening)
- Continued from commit `ec0c155`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The next-tick family was already part of the parser and extraction design surface:
  - `next tick`
  - `following tick`
  - `subsequent tick`
- The tracked KG-quality benchmark corpus already locked those spellings inside `crates/specforge/test_data/kg_quality/next_tick_timing_gold/`.
- But the direct proof lane still only locked the canonical semantic path and had no validator regression for the full lexical trio.
- This slice expands the direct extractor regression, semantic regression, and validator regression so the supported next-tick lexical surface is protected before and alongside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a direct-lane hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused extractor coverage, focused semantic coverage, focused validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline rises to `477` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 clock-edge-of-clock signal-leading exact hardening)
- Continued from commit `65943a0`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The `clock edge(s) of <clock>` family was already part of the parser and extraction design surface:
  - `clock edge T4 of HCLK`
  - `within 2 clock edges of HCLK`
  - `HCLK clock edge T5`
- But the tracked KG-quality benchmark corpus and the direct clock-edge-of-clock tests still only locked the trailing exact spelling and the trailing bounded spelling.
- This slice deepens `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/` so the existing fixture family now proves the signal-leading exact `clock edge` spelling through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- It also expands the direct semantic and validator regressions so clock-edge-of-clock variants are protected outside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `476` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 named quantified signal-leading ordinal hardening)
- Continued from commit `9f46e35`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named quantified and ordinal generic-edge family was already part of the parser and extraction design surface:
  - `within 2 HCLK edges`
  - `third edge of HCLK`
  - `third HCLK edge`
- But the tracked KG-quality benchmark corpus and the direct named quantified tests still only locked the signal-leading bounded form and the trailing ordinal spelling.
- This slice deepens `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/` so the existing fixture family now proves the signal-leading ordinal named-edge spelling through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- It also expands the direct semantic and validator regressions so named quantified-edge variants are protected outside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `476` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 named diagram-edge hardening)
- Continued from commit `1629861`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named generic-edge diagram family was already part of the parser and extraction design surface:
  - `edge T3 of HCLK`
  - `HCLK edge T4`
- But the tracked KG-quality benchmark corpus and the direct named diagram-edge tests still only locked the trailing `edge ... of HCLK` spelling.
- This slice deepens `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/` so the existing fixture family now proves the signal-leading named generic-edge spelling through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- It also expands the direct semantic and validator regressions so named diagram-edge variants are protected outside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `476` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 unit-first diagram-position hardening)
- Continued from commit `3185061`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The unit-first local-clock diagram family was already part of the parser and extraction design surface:
  - `tick T3 of HCLK`
  - `posedge T4 of HCLK`
  - `rising edge T5 of HCLK`
- But the tracked KG-quality benchmark corpus and the direct unit-first tests still only locked the canonical token form.
- This slice deepens `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/` so the existing fixture family now proves the remaining `tick` and edge-word unit-first spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- It also expands the direct semantic and validator regressions so unit-first diagram-position variants are protected outside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `476` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 named one-cycle edge hardening)
- Continued from commit `1277f37`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named one-cycle family was already part of the parser/extraction design surface:
  - `next ACLK cycle`
  - `following HCLK edge`
  - `subsequent HCLK rising edge`
  - `next HCLK clock edge`
  - `following HCLK falling edge`
- But the tracked KG-quality benchmark corpus and the direct named one-cycle tests still only locked the cycle/generic-edge subset plus one rising-edge form.
- This slice deepens `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/` so the existing fixture family now proves the remaining named `clock edge` and falling-edge one-cycle spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 5` and `temporal_rules_missing_clock_grounding = 0`.
- It also adds direct semantic and validator regressions so named one-cycle edge variants are protected outside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline rises to `476` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 named local zero-cycle edge hardening)
- Continued from commit `8f4378b`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named local zero-cycle family was already part of the parser/extraction design surface:
  - `same ACLK cycle`
  - `this HCLK tick`
  - `current HCLK edge`
  - `current HCLK clock edge`
  - `current HCLK falling edge`
- But the tracked KG-quality benchmark corpus and the direct named zero-cycle tests still only locked the cycle/tick subset plus one rising-edge form.
- This slice deepens `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/` so the existing fixture family now proves the remaining named generic-edge and falling-edge zero-cycle spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 5` and `temporal_rules_missing_clock_grounding = 0`.
- It also adds direct semantic and validator regressions so named zero-cycle falling-edge grounding is protected outside the benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, validator coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 default-clock zero-cycle edge hardening)
- Continued from commit `4e2dc71`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock zero-cycle family was already part of the parser/extraction design surface:
  - `same cycle`
  - `same tick`
  - `this tick`
  - `current clock edge`
  - `current rising edge`
  - `current falling edge`
- But the tracked KG-quality benchmark corpus and the direct zero-cycle extraction test still only locked the cycle/tick subset plus one rising-edge form.
- This slice deepens `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/` so the existing fixture family now proves the remaining generic-edge and falling-edge zero-cycle spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 6` and `temporal_rules_missing_clock_grounding = 0`.
- It also extends the direct semantic extraction regression in `semantic.rs` so `current clock edge` and `current falling edge` stay covered at the parser layer instead of only through the end-to-end benchmark harness.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused semantic regression coverage, tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 default-clock zero-cycle lexical benchmark hardening)
- Continued from commit `755055a`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock zero-cycle family was already part of the parser/extraction design surface:
  - `same cycle`
  - `same tick`
  - `this tick`
  - `current rising edge`
- But the tracked KG-quality benchmark corpus only locked two canonical spellings inside that family.
- This slice deepens `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/` so the existing fixture family now proves all four supported zero-cycle spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 4` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 MSRV/toolchain alignment)
- Continued from commit `4719d62`, this slice does not change Rust architecture or the staged IR model, but it does update the repo’s current toolchain contract.
- The workspace `rust-version`, the hosted CI toolchain install, and the user-facing getting-started docs had still been pinned to Rust `1.89` even though the active compiler baseline had moved to `1.95`.
- This slice raises the declared workspace MSRV to `1.95`, moves `.github/workflows/ci.yml` to Rust `1.95.0`, and updates the mdBook getting-started docs to the same version so the project advertises one consistent Rust floor.
- Local `run_ci.sh` and whitespace checks passed after the alignment update.

## Session update (2026-04-22 named local zero-cycle lexical benchmark hardening)
- Continued from commit `7ad8e6d`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named local zero-cycle family was already part of the parser/extraction design surface:
  - `same ACLK cycle`
  - `this HCLK tick`
  - `current HCLK edge`
- But the tracked KG-quality benchmark corpus only locked the canonical `same ACLK cycle` spelling.
- This slice deepens `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/` so the existing fixture family now proves all three supported zero-cycle lexical spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- While validating that benchmark hardening, the current toolchain also surfaced repo-wide clippy failures outside the fixture lane. This slice clears those root-cause failures in `validate.rs`, `evidence.rs`, `semantic.rs`, and `source.rs` so the full standard `run_ci.sh` gate stays authoritative.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 named one-cycle lexical benchmark hardening)
- Continued from commit `0175a30`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named one-cycle local-clock family was already part of the parser/extraction design surface:
  - `next ACLK cycle`
  - `following HCLK edge`
  - `subsequent HCLK rising edge`
- But the tracked KG-quality benchmark corpus only locked canonical `next` spellings across that family.
- This slice deepens `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/` so the existing fixture family now proves all three supported one-cycle lexical spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 default-clock explicit edge lexical benchmark hardening)
- Continued from commit `3663e53`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock explicit edge family was already part of the parser/extraction design surface:
  - `next rising edge`
  - `next falling edge`
  - `following rising edge`
  - `following falling edge`
  - `subsequent rising edge`
  - `subsequent falling edge`
- But the tracked KG-quality benchmark corpus only locked the canonical `next` spellings.
- This slice deepens `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/` so the existing fixture family now proves all six supported spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 6` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 generic clock-edge lexical benchmark hardening)
- Continued from commit `65f7748`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic clock-edge family was already part of the parser/extraction design surface:
  - `next clock edge`
  - `following clock edge`
  - `subsequent clock edge`
  - `within N clock edges`
- But the tracked KG-quality benchmark corpus only locked the canonical `next clock edge` spelling plus the bounded variant.
- This slice deepens `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/` so the existing fixture family now proves all three supported one-cycle spellings plus the bounded form through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 4` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 shorthand next-edge lexical benchmark hardening)
- Continued from commit `efbebe8`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The shorthand next-edge family was already part of the parser/extraction design surface:
  - `next posedge`
  - `next negedge`
  - `following posedge`
  - `following negedge`
  - `subsequent posedge`
  - `subsequent negedge`
- But the tracked KG-quality benchmark corpus only locked the canonical `next` spellings.
- This slice deepens `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/` so the existing fixture family now proves all six supported spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 6` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 next-tick lexical benchmark hardening)
- Continued from commit `f8626f1`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The idiomatic next-tick family was already part of the parser/extraction design surface:
  - `next tick`
  - `following tick`
  - `subsequent tick`
- But the tracked KG-quality benchmark corpus only locked the canonical `next tick` phrase.
- This slice deepens `crates/specforge/test_data/kg_quality/next_tick_timing_gold/` so the existing fixture family now proves all three supported spellings through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite size stays flat because this is a lexical hardening pass inside an already-landed benchmark family, not a new corpus family.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite remains `125` fixtures.

## Session update (2026-04-22 default-clock later-edge benchmark lock)
- Continued from commit `b640021`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock later-edge family was already part of the parser/extraction design surface:
  - `after N rising edges`
  - `after N falling edges`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/default_clock_later_edge_timing_gold/`, a compact tracked fixture that locks the default-clock later-edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `125` fixtures.

## Session update (2026-04-22 default-clock quantified-edge benchmark lock)
- Continued from commit `081299b`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock quantified and ordinal edge family was already part of the parser/extraction design surface:
  - `within N rising edges`
  - `within N falling edges`
  - `third rising edge`
  - `third falling edge`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/default_clock_quantified_edge_timing_gold/`, a compact tracked fixture that locks the default-clock quantified and ordinal edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 4` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `124` fixtures.

## Session update (2026-04-22 default-clock explicit next-edge benchmark lock)
- Continued from commit `6509fb9`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock explicit next-edge family was already part of the parser/extraction design surface:
  - `next rising edge`
  - `next falling edge`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/`, a compact tracked fixture that locks the default-clock explicit next-edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `123` fixtures.

## Session update (2026-04-22 generic exact-cycle benchmark lock)
- Continued from commit `dd61578`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic exact-cycle family was already part of the parser/extraction design surface:
  - `after N cycles`
  - `for N cycles`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/generic_exact_cycle_timing_gold/`, a compact tracked fixture that locks the generic exact-cycle family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `122` fixtures.

## Session update (2026-04-22 generic range-cycle benchmark lock)
- Continued from commit `2b6d848`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic range and one-sided cycle family was already part of the parser/extraction design surface:
  - `at least N cycles`
  - `at most N cycles`
  - `between N and M cycles`
  - `no more than N cycles`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/`, a compact tracked fixture that locks the generic range-cycle family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 4` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `121` fixtures.

## Session update (2026-04-22 generic bounded-cycle benchmark lock)
- Continued from commit `5316b79`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic bounded-cycle family was already directly proved in semantic coverage:
  - `within 2 cycles` recovered the expected bounded window
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/generic_bounded_cycle_timing_gold/`, a compact tracked fixture that locks the generic bounded-cycle family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 1` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `120` fixtures.

## Session update (2026-04-22 generic next-cycle benchmark lock)
- Continued from commit `5410478`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic next-cycle family was already directly proved in semantic coverage:
  - `next cycle` recovered the expected one-cycle window
  - `next clock cycle` recovered the expected one-cycle window
  - the same built-in timing lane also recognized `following cycle` and `subsequent cycle`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/`, a compact tracked fixture that locks the generic next-cycle family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 4` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `119` fixtures.

## Session update (2026-04-22 default-clock zero-cycle benchmark lock)
- Continued from commit `ac2eea8`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock zero-cycle family was already directly proved in semantic coverage:
  - `same tick` recovered the expected zero-cycle window
  - `current rising edge` recovered the expected zero-cycle window
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/`, a compact tracked fixture that locks the default-clock zero-cycle family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `118` fixtures.

## Session update (2026-04-22 default-clock generic clock-edge benchmark lock)
- Continued from commit `fee7c02`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The default-clock generic clock-edge family was already directly proved in parser coverage:
  - `next clock edge` recovered the expected one-cycle window
  - `within 2 clock edges` recovered the expected bounded window
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/`, a compact tracked fixture that locks the default-clock generic clock-edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `117` fixtures.

## Session update (2026-04-22 next-tick benchmark lock)
- Continued from commit `eaee035`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The idiomatic bare next-tick family was already directly proved in unit coverage:
  - `next tick` recovered `cycle_window = 1..1`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/next_tick_timing_gold/`, a compact tracked fixture that locks the idiomatic next-tick family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 1` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `116` fixtures.

## Session update (2026-04-22 later-phrase benchmark lock)
- Continued from commit `55df3a3`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The explicit later-phrase family was already directly proved in unit coverage:
  - `two cycles later` recovered `cycle_window = 2..2`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/`, a compact tracked fixture that locks the explicit later-phrase family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 1` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `115` fixtures.

## Session update (2026-04-22 shorthand next-edge benchmark lock)
- Continued from commit `2804882`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The bare shorthand next-edge family was already directly proved in unit coverage:
  - `next posedge` recovered `edge = rising` with `cycle_window = 1..1`
  - `next negedge` recovered `edge = falling` with `cycle_window = 1..1`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/`, a compact tracked fixture that locks the shorthand next-edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `114` fixtures.

## Session update (2026-04-22 tick-unit benchmark lock)
- Continued from commit `7efb8ce`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The generic tick-unit family was already directly proved in unit coverage:
  - `within 2 ticks` recovered the expected bounded `cycle_window`
  - `tick T3` recovered the expected exact `cycle_window`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/tick_unit_timing_gold/`, a compact tracked fixture that locks the generic tick-unit family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `113` fixtures.

## Session update (2026-04-22 plural edge-of-clock benchmark lock)
- Continued from commit `fd6c434`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The plural edge-of-clock family was already directly proved in unit coverage:
  - `within 2 edges of HCLK` preserved `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/plural_edge_of_clock_timing_gold/`, a compact tracked fixture that locks the plural edge-of-clock family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 1` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `112` fixtures.

## Session update (2026-04-22 unit-first diagram-position benchmark lock)
- Continued from commit `2fd726a`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The unit-first diagram-position family was already directly proved in unit coverage:
  - `posedge T4 of HCLK` preserved `clock_signal = HCLK`, `edge = rising`, and `cycle_window = 4..4`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/`, a compact tracked fixture that locks the unit-first diagram-position family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 1` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `111` fixtures.

## Session update (2026-04-22 named diagram-edge benchmark lock)
- Continued from commit `b40ccbb`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named diagram-edge family was already directly proved in unit coverage:
  - `edge T3 of HCLK` preserved `clock_signal = HCLK`, `edge = rising`, and `cycle_window = 3..3`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/`, a compact tracked fixture that locks the named diagram-edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 1` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `110` fixtures.

## Session update (2026-04-22 named quantified-edge benchmark lock)
- Continued from commit `93ef907`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named quantified and ordinal edge family was already directly proved in unit coverage:
  - `within 2 HCLK edges` preserved `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
  - `the third edge of HCLK` preserved `clock_signal = HCLK`, `edge = rising`, and `cycle_window = 3..3`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/`, a compact tracked fixture that locks the named bounded and exact edge family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 2` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `109` fixtures.

## Session update (2026-04-22 named one-cycle benchmark lock)
- Continued from commit `8bbead3`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named one-cycle family was already directly proved in unit coverage:
  - `next ACLK cycle` preserved `clock_signal = ACLK`, `edge = rising`, and `cycle_window = 1..1`
  - `next HCLK edge` preserved `clock_signal = HCLK`, `edge = rising`, and `cycle_window = 1..1`
  - `next HCLK rising edge` already stayed out of the missing-cycle-window validation path
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/`, a compact tracked fixture that locks the named one-cycle local-clock family through both `SemanticIR` and `IntentIR`, with `temporal_rules_with_cycle_window = 3` and `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `108` fixtures.

## Session update (2026-04-22 named local cycle benchmark lock)
- Continued from commit `1867e39`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The named local cycle family was already directly proved in unit coverage:
  - `same ACLK cycle` preserved `clock_signal = ACLK`
  - the same rule preserved `edge = rising`
  - the same rule preserved `cycle_window = 0..0`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/`, a compact tracked fixture that locks `TVALID must be asserted in the same ACLK cycle.` through both `SemanticIR` and `IntentIR`, with `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `107` fixtures.

## Session update (2026-04-22 signal-leading clock benchmark lock)
- Continued from commit `c55b858`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The signal-leading clock family was already directly proved in unit coverage after the previous slices:
  - word-form `HCLK rising edge` / `HCLK falling edge`
  - token-form `HCLK posedge` / `HCLK negedge`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/`, a compact tracked fixture that locks all four signal-leading local-clock forms through both `SemanticIR` and `IntentIR`, with `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not just another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `106` fixtures.

## Session update (2026-04-22 signal-leading clock lexical hardening)
- Continued from commit `d237f12`, still tightening the temporal proof surface rather than widening the parser, semantic model, or tracked fixture corpus.
- The signal-leading clock family was already symmetric in edge direction after the previous slice:
  - `HCLK rising edge` and `HCLK falling edge` at the semantic layer
  - `HCLK posedge` and `HCLK negedge` at the validator layer
- But the direct proof was still lexically split across those two lanes.
- This slice makes the family lexically self-contained in both places:
  - `crates/specforge/src/ir/semantic.rs` now also proves token-form `HCLK posedge` / `HCLK negedge` beside the existing word-form pair
  - `crates/specforge/src/commands/validate.rs` now also proves word-form `HCLK rising edge` / `HCLK falling edge` beside the existing token-form pair
- The tracked KG-quality suite stays flat because this is reliability hardening inside an existing temporal family, not a new benchmark family or capability row.
- Focused semantic and validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 signal-leading clock symmetry hardening)
- Continued from commit `235197f`, still tightening the temporal proof surface rather than widening the parser, semantic model, or tracked fixture corpus.
- The signal-leading clock family already had direct proof on the rising side:
  - `HCLK rising edge` at the semantic layer
  - `HCLK posedge` at the validator layer
- But the falling-side twins were still only implied by shared parser logic.
- This slice makes the family internally symmetric in its direct proof lanes:
  - `crates/specforge/src/ir/semantic.rs` now proves `PWAKEUP must be asserted on HCLK falling edge.` preserves `clock_signal = HCLK` and `edge = falling` beside the existing rising-side rule
  - `crates/specforge/src/commands/validate.rs` now proves `PWAKEUP must be asserted on HCLK negedge.` stays fully grounded beside the existing `HCLK posedge` rule
- The tracked KG-quality suite stays flat because this is reliability hardening inside an existing temporal family, not a new benchmark family or capability row.
- Focused semantic and validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Executive summary
- the repository now contains a single active `specforge` crate and CLI with an executable surface of:
  - `inspect`
  - `doctor`
  - `converge`
  - `ingest`
  - `evidence`
  - `semantic`
  - `intent`
  - `adapt`
  - `enrich`
  - `validate`
  - `kg-bench`
  - `project-validation`
  - `rescan-plan`
  - `learn-priors`
  - `corpus-kb`
  - `clean`
  - `nlp-enrich`
- the canonical product boundary remains `IntentIR`, not `.fsm`
- the staged pipeline is operational through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- a whole-pipeline fixed-point entrypoint now exists via `specforge converge`, which reuses persisted artifacts, defaults to Ollama VLM + NLP Level 3, and stops when the cross-stage knowledge snapshot is stable
- `SourceIR` now captures structured Docling output, typed content elements, structured tables, visual assets, and document-profile metadata, and its PDF normalization path stages into `normalized.staging/` before atomically replacing `normalized/` so stale source-side leftovers do not accumulate across reruns
- `EvidenceIR` now synthesizes typed declarations and records from tables, preserves typed NLP outputs, persists alias-learning state, extracts actor-signal relation triples from prose and signal-description tables, runs a monotone convergence loop so discovered enum facts and explicit active-level polarity prose can unlock additional signal constraints without hardcoded protocol-specific value lists, keeps polarity disagreement explicit through typed conflict records instead of only via a neutralized fallback, preserves provenance from table-synthesized signal declarations back to originating `SourceIR` table ids, and exposes that bridge through the `table_signal_declaration_provenance` validation metric
- `SemanticIR` now lifts that evidence into interfaces, canonical signal records with table-support ids, explicit interface-signal conflict records for conflicting direction/width evidence, actor-relative port/connectivity records, explicit signal-connectivity conflict records for unresolved multi-producer ambiguity, system/reset/init records, control/state records, timing/register records, and filtered NLP constraints, with VLM observations merged into the semantic surface
- `IntentIR` now carries forward the canonical signal/control/system/state/register/timing surface plus the actor-relative KG needed for honest downstream lowering
- `specforge validate` now reports `table_signal_declaration_provenance` for `EvidenceIR` plus `with_table_support` for `SemanticIR` and `IntentIR`, while `specforge kg-bench` can assert exact EvidenceIR table-signal provenance records, direct EvidenceIR provenance counts, exact canonical table support, validation finding payloads, validation finding related IDs, direct graph-direction conflicted signal-name sets, actor-level graph-direction conflict provenance records, actor-aware graph-direction conflict related ids, direct graph-direction coverage-gap related ids, and direct compat-direction lag related ids via a narrow flat-hint-clearing patch lane, making both evidence-stage table-declaration provenance and validation-backed caution/rescan surfaces visible without replacing exact per-signal provenance checks
- `kg-bench` can now also assert `validation.source` expectations directly, so tracked fixtures can lock SourceIR validation findings and metrics instead of leaving source-stage replay surfaces unit-test-only
- source-stage missing VLM enrichment now joins the replay plane too: `validate` still reports timing/state diagram asset ids when `SourceIR` carries no VLM enrichment, but now also emits source-stage VLM-gap-specific rescan guidance so `project-validation` can route those exact asset ids through a narrower `enrich -> validate` loop on the current `SourceIR` artifact instead of leaving them as passive ingest-side debt
- actor-port gaps now join that canonical replay plane too: `validate` still reports relation-only actor graph remnants when `SemanticIR` or `IntentIR` carries `actor_signal_relations` but no `actor_ports`, but now also emits actor-port-gap-specific rescan guidance so `project-validation` can route those exact `asr_*` relation ids through the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` lane instead of leaving them as passive graph debt
- graph-direction coverage in validation and benchmark expectations is now a little stricter too: same-actor self-conflicts no longer earn graph-direction credit merely because non-`unknown` actor ports exist somewhere for that signal, validation reports those self-conflicts explicitly instead of only showing a silent coverage drop, and `project-validation` now routes those preserved actor-aware contradictions into the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` replay lane instead of leaving them as passive warnings
- evidence-stage missing VLM observations now join the replay plane too: `validate` still reports visual evidence ids when `EvidenceIR` carries no timing/state observations, but now also emits VLM-gap-specific rescan guidance so `project-validation` can route those exact visual ids through a source-side `enrich -> evidence -> validate` loop instead of leaving them as passive enrichment debt
- evidence-stage structural-KG gaps now join the replay plane too: `validate` still reports stranded behavioral ids when `EvidenceIR` carries signal constraints or conditional rules without any actor-signal graph, but now also emits structural-gap-specific rescan guidance so `project-validation` can route those exact behavioral ids through a narrower `nlp-enrich -> validate` loop on the current `EvidenceIR` artifact instead of leaving them as passive graph debt
- evidence-stage normative residuals now join the replay plane too: `validate` still reports partially structured normative statement ids as an honesty surface, but now also emits residual-specific rescan guidance so `project-validation` can route those exact statement ids through a narrower `nlp-enrich -> validate` loop on the current `EvidenceIR` artifact instead of leaving them as passive extraction debt
- evidence-stage signal-polarity conflicts now join the replay plane too: `validate` still reports the warning-level active-level disagreement, but now also emits conflict-id-specific rescan guidance so `project-validation` can route preserved `polarity_conflict_*` ids through a narrower `nlp-enrich -> validate` loop on the current `EvidenceIR` artifact instead of leaving them as passive upstream warnings
- evidence-stage negative-knowledge caution now joins the replay plane too: `validate` still reports prior-match caution at `EvidenceIR`, but `project-validation` now routes the exact caution-target ids through a bounded `SourceIR -> EvidenceIR -> validate` lane instead of leaving evidence-stage caution as a planner dead end or pretending it already requires canonical-stage rebuilds
- carried signal-semantic conflicts now join that same replay plane too: `validate` still reports the warning-level semantic-role disagreement, but now also emits conflict-id-specific rescan guidance so `project-validation` can route preserved `semantic_conflict_*` ids through the same bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` loop instead of relying only on the broader signal-name arbitration surface
- the current `.fsm` adapter slice is real and intentionally narrow: it can emit honest `?dt:name`, `?fsm:name`, and `?top:name` outputs when the canonical facts are explicit enough
- the enrichment, convergence, validation, benchmark, targeted-rescan, first prior-learning, and first corpus-knowledge toolchain is also real: `specforge enrich`, `specforge nlp-enrich`, `specforge converge`, `specforge validate`, `specforge project-validation`, `specforge rescan-plan`, `specforge kg-bench`, `specforge learn-priors`, and `specforge corpus-kb` are wired into the CLI and exercised by the workspace tests
- generated artifact hygiene is now part of the executable surface too: `specforge clean` dry-runs rebuildable heavyweight `generated/source_ir/*/normalized` bundles by default, can execute those deletions explicitly, can remove full per-document generated stage trees, and now also supports a first-class `--scope all-generated` sweep for intentionally discarding the entire local `generated/` root
- `project-validation` and `rescan-plan` now form a schema-v2 targeted-rescan loop: recommendations carry typed replay inputs, structured local command hints, dry-run-by-default execution, before/after validation snapshots, execution summaries, promotion-gate descriptors, and an explicit no-canonical-mutation boundary
- the `rescan-plan` dry-run renderer regression now locks the review-critical pending recommendation fields, including artifact path, extractor lane, replay inputs, action text, related ids, automation status, and command hints
- sparse dry-run recommendations are covered too: empty replay inputs, related ids, and command hints must remain visible as explicit `none` values
- document-scoped rescan selection now has regression coverage that prevents already executed recommendations or absent document keys from re-entering the pending dry-run queue
- command-hint parsing now has direct regression coverage for rejecting non-repository working directories, non-standard cargo manifests, and cargo hints without SpecForge args after the required separator
- accepted local provider command hints are locked too: LM Studio aliases, source-enrich `--classify-only`, and skip-mode NLP enrichment remain valid while OpenAI stays rejected
- malformed local provider hints are locked as rejections too, including repeated provider flags, missing values, and unsupported args
- execution-summary validation deltas now have direct regression coverage that sorts and deduplicates added/removed finding-id lists before review persistence
- score/grade presence-only validation changes are regression-locked as neutral review-required drift rather than automatic improvement/regression because they lack a comparable score delta
- that replay surface now spans both evidence-local and canonical-local loops: evidence-stage normative residuals, signal-polarity conflicts, or signal-semantic conflicts can drive a direct `nlp-enrich -> validate` plan on the current `EvidenceIR`, while canonical-stage gaps still use the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` rebuild lanes
- negative-knowledge caution now spans stage-appropriate replay scopes too: evidence-stage caution uses `SourceIR -> EvidenceIR -> validate`, while semantic/intent caution still use the deeper downstream rebuild lanes
- `CorpusMemory` schema v5 now carries actor-taxonomy, semantic-phrase, semantic-modality-reliability, temporal-phrase, table-shape, visual-motif, and negative-knowledge prior families; current consumers remain advisory and locally grounded rather than fact-authoring, and the negative-knowledge surface now includes signal-polarity-conflict archetypes beside semantic, temporal, interface, connectivity, and residual caution patterns
- the first `R15g` corpus knowledge-base plane now exists too: tracked `corpus_kb/` pages can preserve reviewable cross-document synthesis, and `specforge corpus-kb` refreshes managed validation-finding, KG fixture-result, dedicated semantic/truthfulness pattern, typed-prior-memory, table/visual/state-machine/timing/infrastructure/protocol-family pages, and review-only prior-candidate pages plus a schema-versioned readiness manifest from validation report sidecars / benchmark fixture outcomes without mutating canonical IR or typed priors; the KG fixture-result path now uses quiet fixture-local validation and projects fixture-family summaries so coverage is visible beyond aggregate pass/fail counts
- the tracked KG-quality benchmark surface currently contains 105 fixtures, including signal-table inventory authority and table-provenance coverage, direct signal-inventory exclusion coverage for document/integration vocabulary false positives, direct signal-connectivity conflict shape coverage for multi-producer graph conflicts, direct signal-semantic conflict shape coverage for multimodal disagreement, direct interface-signal conflict shape coverage for direction/width disagreement, direct signal-polarity conflict shape coverage for active-level prose/table disagreement, direct source-stage missing-VLM-enrichment replay-guidance coverage, direct actor-port-gap replay-guidance coverage, direct evidence-stage missing-VLM replay-guidance coverage, direct evidence-stage structural-KG replay-guidance coverage, direct polarity-conflict replay-guidance coverage, direct evidence-stage normative-residual replay-guidance coverage, direct prior-guided polarity-conflict caution coverage, direct resolved signal-polarity shape coverage for active-high/active-low recovery, direct resolved semantic-role shape coverage for valid-like/ready-like recovery, direct semantic-grounding strength coverage for single-source/cross-modality evidence quality, direct temporal-conflict shape coverage for the negative-knowledge caution path, direct temporal-rule shape coverage for representative AXI/APB/AHB timing fixtures including APB address/protection wait-state stability, APB write-control stability, APB response stability, AHB control stability, AHB transfer/lock stability, AHB exclusive/security stability, AHB response stability, AHB write-data stability, AXI write response, write-response ID stability, address/response USER sideband stability, read address, read-address ID stability, address QoS/region sideband stability, data USER sideband stability, read-address control sideband stability, write-address ID stability, write-address control sideband stability, read-address sideband stability, read data, read-data ID stability, read-data response stability, read-data last stability, write data, write-data last stability, sideband stability, write-address sideband stability, generic `clock edge(s) of <clock>` timing, and trailing shorthand-edge timing, direct relative-clause actor-noise rejection and coordinated active-drive object recovery, direct clock/reset infrastructure-topology coverage, protocol-PDF clock/reset contract-scope negative coverage, generic clock/reset advice negative coverage, same-actor graph-direction self-conflict coverage, compat-direction lag despite graph recovery coverage, VLM timing spurious-annotation/sample-index-label rejection, motion-only timing-annotation rejection, waveform-motion state rejection, VLM state-machine label-noise, undeclared-transition, duplicate-initial, multiple-initial, and missing-initial coverage, active-low VLM timing polarity-equivalence coverage, and collective, mixed clause-local, and detached mixed-polarity negative non-reset control polarity coverage

## Session update (2026-04-21 unit-first diagram-position clock grounding)
- Continued from commit `d3ea3a0`, extending the explicit clock-tick model into the remaining unit-first diagram-position grounding gap without widening timing guesses.
- `explicit_clock_signal_from_text()` now recognizes unit-first diagram-position forms with trailing local clock names such as `tick T3 of HCLK`, `posedge T4 of HCLK`, and `rising edge T5 of HCLK`, preserving the local `clock_signal` instead of leaving those rules half grounded.
- The exact bounded `cycle_window` path was already present; this slice completes the grounding story without changing the window semantics.
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks all passed for this slice.
- The current local CI baseline is `461` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 generic clock-edge-of-clock regression lock)
- Continued from commit `35960c7`, tightening a quality seam without widening the temporal model.
- Generic `clock edge(s) of <clock>` phrasing such as `clock edge T4 of HCLK` and `within 2 clock edges of HCLK` was already supported by the bounded parser helpers and already described in the docs, but it was not yet regression-locked through the semantic and validation layers the way neighboring temporal phrasing families were.
- This slice keeps the model unchanged and makes the contract explicit:
  - parser coverage now proves those forms still recover the expected bounded `cycle_window`
  - semantic coverage now proves `clock_signal = HCLK`, `edge = rising`, and the bounded window survive together into typed temporal rules
  - validator coverage now proves that same phrasing no longer relies on undocumented helper behavior and stays out of both missing-clock and missing-window warning paths
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks passed for this slice.
- The current local CI baseline is `467` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 generic clock-edge-of-clock tracked fixture coverage)
- Continued from commit `3fc7e1a`, raising the same temporal family into the tracked benchmark corpus instead of leaving it only in unit coverage.
- Added `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/`, a minimal tracked fixture that locks both:
  - exact diagram-style generic clock-edge timing: `clock edge T4 of HCLK`
  - bounded generic clock-edge-of-clock timing: `within 2 clock edges of HCLK`
- That fixture now proves, at the KG benchmark level, that both `SemanticIR` and `IntentIR` preserve:
  - `clock_signal = HCLK`
  - `edge = rising`
  - the expected exact or bounded `cycle_window`
- The validation expectations in the same fixture also prove the grounded shape directly instead of only relying on rule presence.
- Focused benchmark coverage and focused `clock_edge_of_clock` unit coverage passed for this slice.
- The current local CI baseline remains `467` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `104` fixtures.

## Session update (2026-04-22 trailing edge-word timing symmetry lock)
- Continued from commit `9db08cf`, tightening the same trailing shorthand-edge benchmark family again without widening the temporal model or adding a new fixture family.
- The prior slice already benchmark-locked token shorthand symmetry:
  - `within 2 posedges of HCLK`
  - `within 2 negedges of HCLK`
- But the closely parallel word-based rising-edge form was still only implied by local parser and semantic logic:
  - `within 2 rising edges of HCLK`
- This slice makes that neighboring form explicit at all three useful levels:
  - direct semantic regression now proves `PSEL must be asserted within 2 rising edges of HCLK.` preserves `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
  - direct validator regression now proves the resulting intent rule stays fully grounded instead of surfacing missing-clock or missing-window debt
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now carries the same bounded word-edge rule so the phrase is benchmark-locked end to end through `SemanticIR`, `IntentIR`, and validation
- The tracked benchmark surface size remains `105` fixtures because this is a hardening pass inside an existing family, not a new family.
- Focused temporal tests, tracked KG fixtures, docs CI, full local CI, and whitespace checks all passed for this slice.
- The current full local CI baseline is now `471` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 trailing falling edge-word symmetry lock)
- Continued from commit `3859295`, keeping the same trailing shorthand-edge family honest without widening truth or creating a new fixture family.
- The prior slice benchmark-locked the word-based rising-side phrase:
  - `within 2 rising edges of HCLK`
- But the falling-side twin was still only implied by generic parser helpers and neighboring token-form coverage:
  - `within 2 falling edges of HCLK`
- This slice makes that twin explicit at the same useful levels:
  - the explicit local-clock extraction regression for trailing `of <clock>` edge phrasing now covers `within 2 falling edges of HCLK`
  - direct semantic coverage now proves `PWRITE must be asserted within 2 falling edges of HCLK.` preserves `clock_signal = HCLK`, `edge = falling`, and `cycle_window.max_cycles = 2`
- the existing validator regression for trailing word-edge timing now proves both rising and falling word-edge rules stay fully grounded together
- the existing tracked fixture `trailing_shorthand_edge_timing_gold` now also carries the falling-side word-edge rule, so both `SemanticIR` and `IntentIR` lock the full five-rule temporal family end to end
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside an existing family, not a new family.
- Focused temporal tests, tracked KG fixtures, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline is now `472` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 ordinal falling edge-word symmetry lock)
- Continued from commit `9cf8a28`, still tightening the same trailing shorthand-edge family rather than widening the temporal model or creating a new fixture family.
- The prior slice closed the bounded falling-side word-edge gap:
  - `within 2 falling edges of HCLK`
- But the exact ordinal family was still asymmetric:
  - `the third rising edge of HCLK` already had direct semantic and validator proof
  - `the third falling edge of HCLK` did not
- This slice makes that exact falling-side twin explicit at the same useful levels:
  - the trailing `of <clock>` local-clock extraction regression now covers `the third falling edge of HCLK`
  - the existing ordinal semantic regression now proves both `third rising edge` and `third falling edge` rules survive typed lowering with `cycle_window = 3..3`
  - the existing explicit-clock validator regression now proves both exact ordinal word-edge rules stay fully grounded together
- the existing tracked fixture `trailing_shorthand_edge_timing_gold` now also carries `PWAKEUP must be asserted on the third falling edge of HCLK.`, so both `SemanticIR` and `IntentIR` lock the full six-rule temporal family end to end
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside the same fixture family.
- Focused extraction, semantic, validator, tracked KG fixture checks, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `472` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 word-edge validator family completion)
- Continued from commit `7ec2155`, still hardening the temporal proof surface rather than widening the temporal model or adding benchmark corpus.
- The spelled-out trailing word-edge family was already supported and benchmark-locked, but its direct validator proof was still split:
  - exact `third rising edge` / `third falling edge` lived in the broader `explicit_clock_text` regression
  - bounded `within 2 rising edges` / `within 2 falling edges` lived in the `trailing_of_word_edge` regression
- This slice makes that validator story self-contained:
  - `validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text` now also carries the exact rising/falling word-edge pair
  - the same direct validator lane now proves all four trailing word-edge corners together: exact rising, exact falling, bounded rising, and bounded falling
- The tracked benchmark surface size remains `105` fixtures because this is validator hardening inside an existing temporal family, not a new corpus addition.
- Focused validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 exact shorthand-token validator symmetry lock)
- Continued from commit `2f20610`, still hardening the same trailing shorthand-edge family rather than widening the temporal model or adding new benchmark corpus.
- The previous slice closed the benchmark and semantic asymmetry for the exact shorthand-token pair:
  - `the third posedge of HCLK`
  - `the third negedge of HCLK`
- After that, the direct intent-stage validator lane was still slightly uneven:
  - bounded `posedges` / `negedges` were both covered
  - exact `negedge` was covered
  - exact `posedge` was only indirectly protected by the tracked fixture and nearby semantic regression
- This slice closes that remaining validator asymmetry:
  - the existing `validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text` regression now also carries `PGRANT must be asserted on the third posedge of HCLK.`
  - the same direct validator lane now proves all four shorthand-token corners together: bounded rising, bounded falling, exact rising, and exact falling
- The tracked benchmark surface size remains `105` fixtures because this is validator hardening inside an existing temporal family, not a new corpus addition.
- Focused validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 exact falling shorthand-token benchmark lock)
- Continued from commit `628c124`, still deepening the existing trailing shorthand-edge family rather than widening the temporal model or creating a new benchmark family.
- The previous slice benchmark-locked the exact ordinal word-edge pair:
  - `the third rising edge of HCLK`
  - `the third falling edge of HCLK`
- After that, one exact shorthand-token asymmetry remained:
  - `the third posedge of HCLK` was already fixture-locked
  - `the third negedge of HCLK` was still only implied by nearby parser and semantic logic
- This slice closes that last exact shorthand-token gap:
  - the trailing `of <clock>` local-clock extraction regression now also covers `the third negedge of HCLK`
  - a new direct semantic regression now proves `PLOCK must be asserted on the third negedge of HCLK.` preserves `clock_signal = HCLK`, `edge = falling`, and `cycle_window = 3..3`
  - the existing trailing shorthand-edge validator regression now proves the exact falling token form stays grounded alongside the already-locked bounded token pair
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now carries `PLOCK must be asserted on the third negedge of HCLK.`
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside the same fixture family.
- Focused extraction, semantic, validator, tracked KG fixture checks, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline is now `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 ordinal rising edge-word benchmark lock)
- Continued from commit `cb24e3b`, still tightening the same trailing shorthand-edge family rather than widening the temporal model or spawning a new benchmark family.
- The prior slice benchmark-locked the exact falling-side ordinal word-edge form:
  - `the third falling edge of HCLK`
- After that, the exact word-based ordinal pair was symmetric in direct semantic and validator proof, but still asymmetric at the tracked fixture level:
  - `the third rising edge of HCLK` remained unit-locked
  - `the third falling edge of HCLK` was fixture-locked
- This slice closes that last benchmark asymmetry:
  - the trailing `of <clock>` local-clock extraction regression now also covers `the third rising edge of HCLK`
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now carries `PGRANT must be asserted on the third rising edge of HCLK.`
  - both `SemanticIR` and `IntentIR` now lock the full seven-rule temporal family end to end, including exact ordinal word-edge rules on both sides
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside the same fixture family.
- Focused extraction, ordinal semantic/validator checks, tracked KG fixture checks, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `472` Rust tests plus warning-deny rustdoc and the mdBook build.
## Session update (2026-04-21 trailing shorthand-edge tracked fixture coverage)
- Continued from commit `f7f9921`, raising the neighboring trailing shorthand-edge family into the tracked benchmark corpus instead of leaving it only in unit coverage.
- The first benchmark pass exposed a real semantic bug rather than just a missing fixture:
  - `within 2 negedges of HCLK` already preserved the local `clock_signal` and bounded `cycle_window`
  - but `explicit_clock_edge_from_text()` still only recognized singular `posedge` / `negedge`
  - so plural shorthand-edge timing could silently fall back to `edge = rising`
- `crates/specforge/src/ir/semantic.rs` now fixes that boundary directly by recognizing plural `posedges` and `negedges`, and a focused regression now proves `within 2 negedges of HCLK` keeps `ClockEdge::Falling`.
- Added `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/`, a minimal tracked fixture that locks both:
  - exact shorthand-edge timing: `the third posedge of HCLK`
  - bounded shorthand-edge timing: `within 2 negedges of HCLK`
- That fixture now proves, at the KG benchmark level, that both `SemanticIR` and `IntentIR` preserve:
  - `clock_signal = HCLK`
  - the explicit edge kind for both rules
  - the expected exact or bounded `cycle_window`
- The validation expectations in the same fixture also prove the grounded shape directly instead of only relying on rule presence.
- Focused benchmark coverage, focused `trailing_of_shorthand_edge` unit coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current local CI baseline is now `468` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `105` fixtures.

## Session update (2026-04-21 plural shorthand-edge symmetry lock)
- Continued from commit `58d0333`, tightening the same temporal family without widening the model or adding a new benchmark family.
- The prior slice fixed plural shorthand-edge semantics in code and directly proved the falling-side plural form, but the rising-side plural twin was still only implied by shared detector logic.
- This slice makes that symmetry explicit:
  - `trailing_shorthand_edge_timing_gold` now also carries `PENABLE must be asserted within 2 posedges of HCLK.`
  - focused semantic regression now proves plural rising shorthand preserves `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
  - validator coverage now proves plural rising and plural falling shorthand rules stay grounded together in one intent-stage report
- The tracked benchmark surface size is unchanged at `105` fixtures because this is a hardening pass inside an existing fixture family, not a new benchmark family.
- The current local CI baseline is now `469` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 trailing-`of <clock>` shorthand-edge grounding)
- Continued from commit `6477b08`, closing another half-grounded temporal corner without widening truth.
- `extract_cycle_window_from_text()` and `explicit_clock_edge_from_text()` already knew how to recover bounded windows and explicit edge kind from phrases like `the third posedge of HCLK` or `within 2 negedges of HCLK`, but `explicit_clock_signal_from_text()` could still drop the locally named clock unless the phrase used the diagram-position `... T4 of HCLK` family.
- This slice keeps the fix narrow:
  - only explicit trailing `of <known clock>` shorthand-edge phrasing is added
  - the widening is limited to edge families already accepted as typed timing language
  - there is still no arbitrary clock guessing from free-form shorthand edge text
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks passed for this slice.
- The current local CI baseline is `464` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 named diagram-style generic-edge positions)
- Continued from commit `d7557f8`, extending the explicit clock-tick model into the remaining named diagram-style generic-edge gap without widening arbitrary edge guessing.
- `extract_cycle_window_from_text_with_known_signals()` now recognizes exact diagram-position forms such as `HCLK edge T3`, `edge T3 of HCLK`, and `clock edge T4 of HCLK`, recovering `cycle_window = N..N` through a bounded known-signal-aware helper instead of leaving those phrases as half-parsed prose.
- `explicit_clock_signal_from_text()` now reuses that same bounded helper, so named diagram-style generic-edge phrases preserve the local `clock_signal` instead of only recovering a numeric window.
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks all passed for this slice.
- The current local CI baseline is `458` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 evidence-stage negative-knowledge replay planning)
- Continued from commit `29d97ef`, closing a planner-consumption gap without widening truth.
- `validate` already emitted `evidence_negative_knowledge_rescan_guidance` for learned caution matches at `EvidenceIR`; this slice makes `project-validation` consume that finding instead of silently dropping it.
- The bounded replay contract stays upstream and honest about stage ownership:
  - replay inputs are `source_ir` plus `evidence_ir`
  - command hints are `rebuild_evidence_ir` then `validate_current_artifact`
  - the action text now says "related evidence conflict or residual ids" so caution does not imply downstream canonical rebuilds that the current stage does not justify
- Focused planner coverage, full tracked KG-bench validation, docs CI, full local CI, and whitespace checking passed for this slice.
- The current local CI baseline is `429` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 actor-port-gap replay guidance)
- Continued from commit `5505790`, extending the canonical replay surface to another explicit graph-completeness gap rather than widening truth.
- `validate` still reports `semantic_actor_ports_missing` / `intent_actor_ports_missing` as explicit knowledge-graph errors, but now also emits actor-port-gap-specific rescan guidance with the exact `asr_*` relation ids that still lack actor-relative port synthesis.
- `project-validation` now treats that gap as a first-class replay target and routes it through the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` lane instead of leaving relation-only graph remnants as passive canonical debt.
- `kg-bench` now accepts `semantic_ir_patch.clear_actor_ports`, and the new tracked fixture `actor_port_gap_surface_negative` locks the replay surface against a relation-present, actor-port-missing canonical artifact.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, full local CI, and whitespace checking passed for this slice.
- The current local CI baseline is `428` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `103` fixtures.

## Session update (2026-04-21 source-stage VLM replay guidance)
- Continued from commit `f18d02a`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `source_vlm_enrichment_missing` as an explicit honesty warning, but now also emits `source_vlm_enrichment_missing_surface_rescan_guidance` with the exact timing/state diagram asset ids that still lack VLM enrichment.
- `project-validation` now treats that source-stage finding as a first-class replay target and emits a SourceIR-local `enrich -> validate` plan against the current `SourceIR`, rather than leaving it as passive visual-enrichment debt or forcing a downstream symptom-oriented replay.
- `kg-bench` now accepts `validation.source` expectations too, and the new tracked fixture `source_vlm_enrichment_surface_negative` locks that source-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, full local CI, and whitespace checking passed for this slice.
- The current local CI baseline is `426` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `102` fixtures.

## Session update (2026-04-21 evidence-stage missing VLM replay guidance)
- Continued from commit `fca8c73`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `evidence_missing_vlm_observations` as an explicit honesty warning, but now also emits `evidence_missing_vlm_observations_surface_rescan_guidance` with the exact visual evidence ids that still lack timing/state extraction.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a source-side `enrich -> evidence -> validate` plan against the current `SourceIR`, rather than forcing an NLP-only lane when the missing information is visual rather than textual.
- The new tracked fixture `evidence_missing_vlm_observations_surface_negative` now locks that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `424` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 evidence-stage structural-KG replay guidance)
- Continued from commit `cb3c08b`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `evidence_structural_kg_missing` as an explicit honesty warning, but now also emits `evidence_structural_kg_missing_surface_rescan_guidance` with the exact stranded behavioral ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the missing graph grounding has not yet left the evidence stage.
- The new tracked fixture `evidence_structural_kg_missing_surface_negative` now locks that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `422` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 evidence-stage normative residual replay guidance)
- Continued from commit `02f862e`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `EvidenceIR` normative residual statement ids as explicit honesty signals, but now also emits `evidence_normative_residual_surface_rescan_guidance` with those exact residual statement ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the under-structuring has not yet left the evidence stage.
- The new tracked fixture `evidence_normative_residual_surface_negative` now locks that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `421` Rust tests plus warning-deny rustdoc and the mdBook build.
- active drive/read actor extraction now trims relative clauses before selecting subject actors, scans the active verb's current object clause for coordinated signals, and shared actor-term hygiene rejects descriptive phrases such as `mixture of`; the staged KG fixture locks `interconnect` as producer and `Manager` as consumer for both `ARCHUNKEN` and `RCHUNKV` while preserving zero connectivity conflicts
- direct `.fsm` graph-backed direction recovery is now target-actor-aware: standalone direct roots can select one actor that graph-drives every render-critical output target, so external actors that drive inputs or read outputs no longer make an otherwise clear target actor unusable; after that target actor is selected, explicit control reads can recover target-actor input directions for local inventory signals across DT and true FSM roots without mutating canonical `IntentIR`; standalone explicit module roots now apply the same bounded read-side recovery inside module-local inventories and preserve contradictory graph/control-read evidence as unresolved; top-composition analysis also preserves top-link-recovered boundary directions in blocked adapter artifacts, collapses contradictory top-boundary plus duplicate top-port direction/width evidence to unresolved state, and can recover existing child-module input/output port directions from explicit child-link endpoints before module renderability analysis while preserving contradictory topology as unresolved; direct and explicit-module roots can also recover existing clock/reset signal input/1-bit shape from canonical system-contract facts while preserving contradictory local shape as unresolved
- adapter-local signal inventories now also separate graph-derived direction from compatibility/system-contract direction explicitly: `graph_direction_hint` plus sticky graph-conflict tracking feeds renderability first, compatibility direction only fills when graph evidence is absent, and explicit graph disagreement remains blocking instead of silently falling back to flat hints

## Session update (2026-04-21 evidence-stage polarity conflict replay guidance)
- Continued from commit `50f3ada`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `EvidenceIR` `signal_polarity_conflicts` as explicit honesty warnings, but now also emits `evidence_signal_polarity_conflict_surface_rescan_guidance` with the exact `polarity_conflict_*` ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the disagreement has not yet left the evidence stage.
- The tracked fixtures `control_polarity_conflict_negative` and `negative_knowledge_prior_guided_polarity_conflict_caution_gold` now lock that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `420` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 evidence-stage semantic conflict replay guidance)
- Continued from commit `dbce48e`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `EvidenceIR` `signal_semantic_conflicts` as explicit honesty warnings, but now also emits `evidence_signal_semantic_conflict_surface_rescan_guidance` with the exact `semantic_conflict_*` ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the disagreement has not yet left the evidence stage.
- The tracked fixtures `cross_modality_semantic_conflict_negative` and `negative_knowledge_prior_guided_semantic_conflict_caution_gold` now lock that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, and full local CI passed for this slice; the current local CI baseline is `419` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 signal-semantic conflict replay guidance)
- Continued from commit `3687576`, extending the schema-v2 replay surface rather than widening canonical truth.
- `validate` still reports carried `signal_semantic_conflicts` as explicit honesty warnings, but now also emits `semantic_signal_semantic_conflict_surface_rescan_guidance` / `intent_signal_semantic_conflict_surface_rescan_guidance` with the exact `semantic_conflict_*` ids.
- `project-validation` now treats those findings as first-class evidence-local replay targets, reusing the existing bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` plan shape and action summaries instead of leaving typed semantic-role contradiction as passive review debt or relying only on the higher-level arbitration-by-signal lane.
- The tracked fixtures `cross_modality_semantic_conflict_negative` and `negative_knowledge_prior_guided_semantic_conflict_caution_gold` now lock that replay surface with and without prior-memory caution.
- Focused validator tests, focused planner tests, full tracked KG-bench validation, and full local CI passed for this slice; the current local CI baseline is `418` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 graph-direction self-conflict replay guidance)
- Continued from commit `a94c73f`, extending the schema-v2 replay surface rather than widening canonical truth.
- `validate` still reports same-actor `graph_direction_conflicts` as explicit honesty warnings, but now also emits `semantic_graph_direction_conflict_surface_rescan_guidance` / `intent_graph_direction_conflict_surface_rescan_guidance` with the exact actor-aware conflict ids.
- `project-validation` now treats those findings as first-class evidence-local replay targets, reusing the existing bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` plan shape and action summaries instead of leaving graph-direction contradiction as passive review debt.
- The tracked fixture `graph_direction_same_actor_conflict_negative` now locks that replay surface end to end, so the benchmark proves both the honesty boundary and the follow-up contract.
- Focused planner tests, focused validator tests, full tracked KG-bench validation, and full local CI passed for this slice; the current local CI baseline is `416` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-18 KG-bench graph-direction conflict fixture lane)
- Continued from commit `99808e9`, turning the new validation honesty surface into a tracked benchmark contract.
- `kg-bench` now accepts a narrow `semantic_ir_patch.actor_ports_append` surface, so fixtures can append canonical actor-port records after normal semantic build and then still flow through the standard `SemanticIR -> IntentIR -> validate` path.
- The new tracked fixture `graph_direction_same_actor_conflict_negative` uses that patch lane to append a conflicting `Completer -> PREADY` input port and proves:
  - graph-direction signal inclusion/exclusion stays `PADDR` yes / `PREADY` no
  - `SemanticIR` and `IntentIR` both report `graph_direction_conflicts: 1`
  - both stages emit the graph-direction conflict warning with `PREADY` as a related id
- This is intentionally a narrow benchmark escape hatch, not a general semantic patching framework. The point is to express canonical honesty regressions that normal source-only synthesis does not naturally preserve, while still exercising the ordinary stage builders and validators.
- Focused harness-unit validation, focused tracked-fixture validation, full `90/90` tracked KG-bench validation, and full local CI with `357` Rust tests plus rustdoc and the mdBook build passed for this slice.

## Session update (2026-04-19 compat-direction lag related IDs)
- Continued from commit `792c60e`, tightening the sibling compatibility-surface warning now that graph-direction conflict vs coverage semantics are cleaner.
- Validation already knew when graph evidence had recovered signal direction while flat compatibility `direction_hint` lagged behind; this slice makes that surface reviewable instead of count-only.
- `semantic_compat_direction_hints_incomplete` now emits missing flat-hint signal names directly in `related_ids`.
- `intent_compat_direction_hints_lag_graph` now emits `related_ids` only for graph-backed declared signals with missing flat hints, which keeps the finding aligned with its actual meaning instead of counting every missing compat hint on the page.
- `kg-bench` now supports a narrow `semantic_ir_patch.clear_signal_direction_hints` patch lane so tracked fixtures can express "graph truth survived, flat compat lagged" without mutating actor ports or general semantic structure.
- The new tracked fixture `compat_direction_hints_lag_graph_negative` locks that graph-first truth boundary end to end, and the corpus-KB projection now reports `92/92` tracked fixtures with `51/51` semantic/truthfulness fixtures.
- Focused validator coverage, focused harness coverage, the tracked fixture run, full local CI with `361` Rust tests plus rustdoc and the mdBook build, and `corpus-kb --kg-fixtures-root` refresh all passed for this slice.

## Session update (2026-04-18 corpus-KB benchmark refresh to 90 fixtures)
- Continued from commit `1ae25f8`, synchronizing the review-facing corpus knowledge-base plane with the newly expanded tracked benchmark suite.
- Re-ran `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality`, which refreshed the managed benchmark and pattern pages without changing canonical IR, validation behavior, or benchmark execution semantics.
- The benchmark projection now reports `90/90` passing fixtures, and the pattern page now reports `49` fixtures across the semantic/truthfulness families.
- `graph_direction_same_actor_conflict_negative` now appears explicitly in those managed review surfaces under:
  - `actor connectivity`
  - `truthfulness negatives and cautions`
- This matters as continuity infrastructure, not only as vanity bookkeeping: future sessions and human reviewers now see the real tracked suite size and the new graph-direction conflict coverage in the persistent corpus-KB plane instead of an outdated `89/89` snapshot.

## Session update (2026-04-18 graph-direction conflict visibility)
- Continued from commit `4fa5e31`, keeping the stricter graph-direction truth boundary but improving observability.
- The validator already refused to credit same-actor self-conflicts as graph-resolved direction; this slice makes that decision explicit through a shared summary helper that returns both resolved signal names and conflicted signal names.
- `SemanticIR` and `IntentIR` validation now both expose:
  - a `graph_direction_conflicts` metric
  - a printed coverage-line conflict count
  - a warning finding that names the conflicted signal ids and explains why graph coverage remains unresolved
- This stays on the reporting side of the boundary:
  - no canonical `SemanticIR` or `IntentIR` facts are rewritten
  - no conflict auto-resolution is attempted
  - the goal is truthful diagnostics, not silent healing
- Focused graph-direction validation tests, the existing `kg-bench` conflict guard regression, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 graph-direction coverage conflict guard)
- Continued from commit `2e7a49b`, tightening the graph-first evaluation surface rather than widening adapters again.
- `validate::graph_direction_signal_names()` now excludes signals when the same actor claims contradictory directions for that same signal, so graph-direction coverage no longer over-credits locally self-conflicted actor-port evidence.
- `kg-bench` now calls that same validator helper, keeping benchmark expectations aligned with validation semantics instead of letting fixture coverage drift onto a looser interpretation.
- This is deliberately narrower than forcing the actor-relative model back into one flat direction bit:
  - different actors can still contribute different directions for the same signal
  - structural multi-producer ambiguity is still surfaced separately through connectivity conflicts
  - only same-actor self-contradiction loses graph-direction credit here
- Focused `validate` tests, focused `kg-bench` tests, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter graph-backed direction surface split)
- Continued from commit `42b5adc`, landing the next concrete `R15` adapter consumer split rather than another analysis-only refresh.
- `FsmSignalCandidate` now carries `graph_direction_hint` and `graph_direction_hint_conflicted`, so adapter-local graph overlays no longer have to reuse the compatibility-facing `direction_hint` field.
- Actor-port, child-link topology, module-control-read, and direct-control-read overlays now all register through that graph-backed surface, while flat interface/system-contract shape remains available as compatibility evidence.
- Renderability now reads direction with an explicit three-state rule:
  - unambiguous graph direction wins
  - absent graph evidence may fall back to compatibility direction
  - conflicted graph evidence stays unresolved and blocks lowering even if a flat compatibility hint exists
- That last behavior is deliberate honesty, not pessimism: if the actor/topology/control-read graph disagrees internally, the adapter should not let a flat hint erase the conflict and pretend the canonical world model is already coherent.
- Focused adapter validation, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter duplicate top-port width conflict collapse)
- Continued from commit `6282875`, hardening duplicate explicit top-port width handling.
- Duplicate top-port declarations still block lowering, but their width hints now merge through sticky `TopPortWidthEvidence` instead of overwriting earlier width evidence in blocked artifacts.
- Added `top_composition_keeps_duplicate_top_port_width_conflict_unresolved`, where `drive_data` is declared twice as a top output with widths `8` and `16`.
- Expected behavior is blocked `.fsm` lowering, no emitted target text, resolved output direction, and unresolved width in both the top candidate and selected top signal inventory.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.
- the local runtime boundary is now operationally stronger too: `specforge doctor` reports Docling readiness, the default Ollama loopback readiness, and LM Studio fallback readiness directly, repo-local `.venv-docling` auto-discovery is supported, and the backend now probes versioned Python candidates like `python3.11` before giving up on fresh ingest
- GitHub Actions CI is part of the repo baseline and still runs `cargo fmt --all --check`, warning-deny Clippy, warning-deny Rust tests, warning-deny rustdoc, and the mdBook build when launched manually, but automatic `push` / `pull_request` triggers are temporarily paused to conserve account Actions minutes
- that CI path still has a single checked-in entrypoint at `scripts/run_ci.sh`, and the GitHub workflow calls that script directly so local and hosted Rust validation do not drift apart
- the remaining dominant gaps are semantic-truthfulness gaps: finishing the remaining graph-first consumers, deepening the temporal-rule layer into richer actor-relative and contradiction-aware clocked semantics, KG-guided rescans, evidence arbitration, benchmark-quality evaluation, and deepening the now-started `R15g` corpus knowledge base beside the already-live typed prior-memory plane; adapter expansion is now horizon work
- the workspace currently validates through `bash scripts/run_ci.sh`, which runs Rust formatting, Clippy with `-D warnings`, Rust tests with `RUSTFLAGS="-D warnings"`, rustdoc with `RUSTDOCFLAGS="-D warnings"`, and the mdBook docs build; after the signal-polarity conflict replay/caution slice the full local CI path reports clean formatting, clean Clippy, `414` passing Rust tests, clean Rust API docs, and a successful mdBook build

## Session update (2026-04-21 polarity-conflict replay and caution)
- Continued from commit `5501cd9`, extending the replay-oriented validation lane to the remaining carried active-level disagreement surface instead of leaving polarity as a warning-only outlier.
- `validate` now emits `semantic_signal_polarity_conflict_surface_rescan_guidance` / `intent_signal_polarity_conflict_surface_rescan_guidance` alongside the existing carried polarity-conflict warnings, using the preserved `polarity_conflict_*` ids directly.
- `project-validation` now classifies those findings into the same bounded local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane used by the nearby conflict and evidence-strength surfaces, with action text aimed at collapsing the related conflict ids toward one locally corroborated active-level interpretation.
- Negative-knowledge caution learned to cover this family too:
  - `NegativeKnowledgeKind` now includes `signal_polarity_conflict`
  - `learn-priors` harvests the normalized polarity-disagreement shape from validated `IntentIR`
  - `validate` matches that learned caution in `EvidenceIR`, `SemanticIR`, and `IntentIR` without mutating current-document truth
- The tracked benchmark surface now locks both sides:
  - `control_polarity_conflict_negative` requires the semantic/intent replay-guidance findings
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold` proves evidence-stage caution plus semantic/intent replay guidance and negative-knowledge findings for the same preserved conflict id
- Focused polarity-validation coverage, focused polarity prior-learning coverage, focused project-validation coverage, full tracked `kg-bench`, docs CI, full local CI with `414` Rust tests plus rustdoc and mdBook, and `git diff --check` passed for this slice.

## Session update (2026-04-18 adapter duplicate top-port direction conflict collapse)
- Continued from commit `283215f`, hardening duplicate explicit top-port declaration handling.
- Duplicate top-port declarations still block lowering, but their direction hints now merge through sticky `TopPortDirectionEvidence` instead of overwriting earlier evidence in the direction map.
- Added `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved`, where `drive_data` is declared as both top output and top input.
- Expected behavior is blocked `.fsm` lowering, no emitted target text, and unresolved `drive_data` direction in both the top candidate and selected top signal inventory.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter top-port direction conflict collapse)
- Continued from commit `3f99161`, hardening explicit top-boundary direction analysis.
- Added sticky `TopPortDirectionEvidence` so contradictory top declaration/topology evidence blocks lowering and collapses the resolved top port direction to `None` instead of leaving a stale earlier hint visible.
- Added `top_composition_keeps_conflicting_top_port_direction_unresolved`, where `drive_data` is declared as a top output but used as a top-link source, implying top input.
- Expected behavior is blocked `.fsm` lowering, no emitted target text, and unresolved `drive_data` direction in both the top candidate and selected top signal inventory.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 graph-direction conflict finding provenance)
- Continued from commit `08de26a`, aligning validation payloads with the sharper graph-conflict truth surface already present in the benchmark harness.
- Same-actor graph-direction conflict findings now emit stable actor-aware related ids such as `graph_direction_conflict:actor_completer:PREADY` instead of only the raw conflicted signal name.
- The signal-count metric stays intentionally unchanged: `graph_direction_conflicts` still counts conflicted signals, while the warning payload now carries the actor-level provenance needed for targeted review and rescan planning.
- Updated the tracked `graph_direction_same_actor_conflict_negative` fixture and focused validation regressions so both `SemanticIR` and `IntentIR` lock the actor-aware related-id payload directly.
- Formatting, three focused regressions, the focused tracked fixture run, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 kg-bench graph-direction conflict provenance)
- Continued from commit `c17dda7`, tightening the tracked graph-direction truth contract without changing canonical behavior.
- Added `graph_direction_conflicts_include` to canonical-stage `kg-bench` expectations so fixtures can assert same-actor self-conflicts as typed actor-plus-signal records.
- The harness derives those records from the same graph-direction coverage summary used for resolved/conflicted signal-name coverage, keeping the conflict detector centralized instead of duplicating logic in the benchmark harness.
- Updated `graph_direction_same_actor_conflict_negative` to lock the specific culprit explicitly: `Completer` self-conflicts on `PREADY`.
- Formatting, two focused regressions, the focused tracked fixture run, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 kg-bench graph-direction conflict expectations)
- Continued from commit `f8359fa`, tightening the tracked graph-direction honesty contract without changing canonical semantics.
- Added `graph_direction_conflicted_signal_names_include` and `graph_direction_conflicted_signal_names_exclude` to canonical-stage `kg-bench` expectations.
- The harness computes that set from the same graph-direction coverage summary used by validation, so fixtures can assert exactly which signals were withheld from resolved graph-direction credit because of same-actor self-conflicts.
- Updated `graph_direction_same_actor_conflict_negative` to prove the split explicitly: `PADDR` remains resolved while `PREADY` appears only in the conflicted set.
- Formatting, the focused harness test, the focused tracked fixture run, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter system-contract signal conflict guard)
- Continued from commit `3d417ea`, adding adversarial regression coverage for clock/reset signal recovery from canonical system-contract facts.
- The new test marks `clk` as a direct interface output while the canonical system contract still says `clk` is the clock.
- Expected behavior is sticky unresolved direction: `clk` keeps `system_contract_signal` provenance, `direction_hint` collapses to `None`, the adapter emits the existing system-contract residual, and no `.fsm` text is emitted.
- This protects the system-contract recovery boundary: clock/reset facts may fill absent adapter shape, but contradictory local signal shape still requires upstream correction.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter system-contract signal recovery)
- Continued from commit `2bd0034`, moving clock/reset adapter renderability away from flat interface shape hints when canonical system-contract facts are already present.
- Added `overlay_system_contract_signal_inventory`, which applies to direct roots and explicit module roots and overlays existing local clock/reset inventory entries as input, 1-bit `system_contract_signal` evidence.
- The overlay is bounded: it cannot create undeclared clock/reset ports, does not mutate canonical `IntentIR`, and still relies on sticky merge behavior to keep contradictory evidence unresolved.
- Added focused regressions for standalone sequential DT and standalone explicit module roots where flat `clk` / `rst_n` direction and width hints are cleared but the system contract remains intact.
- Formatting, both focused system-contract recovery tests, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter child-link topology conflict guard)
- Continued from commit `6d05173`, adding adversarial regression coverage for child-module direction recovery from explicit top-link topology.
- The new test clears flat module-local directions, then makes topology evidence disagree about `producer_core.output_data`: one link uses it as a child source, another as a child target.
- Expected behavior is sticky unresolved direction: `producer_core.output_data` keeps `module_topology_link` provenance, `direction_hint` collapses to `None`, the producer module blocks with the existing missing-direction diagnostic, and no `.fsm` text is emitted.
- This protects the topology recovery boundary: explicit links may fill absent child port roles, but contradictory topology still requires upstream correction.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter child-link topology direction recovery)
- Continued from commit `e03d5f3`, moving another top-composition consumer away from flat module-local `direction_hint` dependence.
- Added a topology-local child-port direction projection from explicit top links: child link sources imply module outputs, and child link targets imply module inputs.
- The overlay is bounded to already-inventory module signals and uses the sticky merge path, so it cannot invent ports, cannot mutate canonical `IntentIR`, and cannot force a winner when flat, actor-port, control-read, or topology evidence conflicts.
- Added `top_composition_recovers_child_directions_from_link_topology`, which clears module-local direction hints and supplies no actor ports; explicit topology alone recovers `producer_core.output_data`, `consumer_core.input_data`, and `consumer_core.result_data` well enough to emit the top.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter explicit-module read conflict guard)
- Continued from commit `e7b41e4`, adding adversarial regression coverage for the new explicit-module `module_control_input` recovery.
- The regression clears flat module-local directions, then makes `controller` actor-port evidence claim `DATA_IN` is an output while the module body reads `DATA_IN` in state-body assignments.
- Expected behavior is sticky unresolved direction: `DATA_IN` keeps both `actor_port` and `module_control_input` evidence categories, `direction_hint` collapses to `None`, the module renderability blocks with the existing missing-direction diagnostic, and no `.fsm` text is emitted.
- This protects the recovery boundary: module-local reads may fill absent input roles, but contradictory evidence still requires upstream correction.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter explicit-module control-read recovery)
- Continued from commit `ee2f1b4`, moving from coverage-only true-FSM direct-root protection into a production adapter improvement for explicit module roots.
- Refactored output-target and control-read collection into reusable slice helpers shared by direct roots and explicit module candidates.
- Added `overlay_module_control_input_inventory`, which marks already-inventory module read signals as `module_control_input` inputs when they are not output/init targets.
- This keeps the recovery bounded: it does not add new ports, does not choose external actor perspectives, and does not mutate canonical `IntentIR`.
- Added a standalone explicit `controller` module regression where flat module-local directions are cleared, the module actor only provides clock/reset inputs plus `ACC` / `TRACE` outputs, and external actors drive `DATA_IN`, `GO`, and `DONE`.
- The selected root renders as `?fsm:controller`, with `DATA_IN`, `GO`, and `DONE` recovered from state-body assignments, transition guards, and standalone control blocks.
- Formatting, the focused new adapter test, the full adapter test module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter structured-FSM graph-read coverage)
- Continued from commit `b888297`, adding regression coverage for an existing true-FSM target-actor/control-read recovery path.
- The new structured-FSM test clears flat direct-interface direction hints and provides graph evidence where `controller` owns `ACC` / `TRACE`, reads `clk` / `rst_n`, external `environment` drives `DATA_IN` / `GO` / `DONE`, and `monitor` reads the outputs.
- The adapter must select `controller` from produced output targets, then recover `DATA_IN`, `GO`, and `DONE` as `direct_control_input` from state-body assignments, transition guards, and standalone control blocks.
- The rendered root remains a true `?fsm:explicit_fsm`; the regression checks emitted sequential assignment, guard, and standalone trace syntax.
- This is intentionally coverage-only: production already had the recovery path, but the true-FSM branch was not directly protected against future drift back to flat `direction_hint` dependence.
- Formatting, the focused new adapter test, the full adapter test module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter blocked-top direction retention)
- Continued from commit `4c79c0b`, targeting the remaining adapter-side `direction_hint` cleanup work rather than widening backend scope.
- `analyze_top_renderability` now returns recovered top ports separately from the optional renderable top root, so link-topology direction recovery can survive even when top composition remains blocked for another reason.
- This closes a small but important artifact-quality gap: a width-only top port whose role is recoverable from `source -> top_port` or `top_port -> target` topology now stays recovered in `FsmTopCandidate.ports` and selected top signal inventory even if no `.fsm` target text is emitted.
- Added a regression with `consumer.result_data -> result_data` and an intentionally missing child module. The adapter stays blocked on `missing_module`, but the recovered `result_data` output direction remains visible for downstream review.
- Focused validation, the full adapter test module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 KG validation-finding payload expectations)
- Continued from commit `9e83612`, adding exact validation-finding payload expectations to the `kg-bench` fixture schema.
- `validation.<stage>.findings_include` can now match finding id, severity, category, summary substring, related ids that must be present, and related ids that must be absent.
- The five negative-knowledge prior-guided caution fixtures now prove their validation prior-match and rescan-guidance findings point at the exact local conflict/residual IDs that triggered the learned caution: `semantic_conflict_0001`, `temporal_conflict_0001`, `semantic_actor_boundary_inference`, `signal_connectivity_conflict_0001`, and `interface_signal_conflict_0001` / `interface_signal_conflict_0002`.
- Added a focused harness self-test for missing validation finding related IDs.
- Focused negative-knowledge fixture validation, `cargo test -p specforge kg_bench`, full tracked `kg-bench`, corpus-KB refresh, docs CI, full local CI, and `git diff --check` passed for this slice.

## Session update (2026-04-18 post-push continuity baseline sync)
- Continued from pushed commit `4dfb6b9`, updating live continuity state rather than Rust code.
- The post-push baseline is clean and aligned with `origin/main`; the previous KG table-support diagnostic batch is no longer in-flight.
- No architecture, extractor, validator, fixture, adapter, or mdBook behavior changed in this slice.
- The full local CI result from the pushed baseline remains the current behavior proof: formatting, warning-deny Clippy, `328` passing Rust tests, warning-deny rustdoc, and mdBook build.
- Docs-only validation for the continuity sync passed through docs CI, whitespace checking, and the README sentinel check.

## Session update (2026-04-18 KG IntentIR table-support missing-signal diagnostic coverage)
- Continued from commit `849e14a`, adding the IntentIR mirror for absent-signal `signal_supporting_table_ids_include` diagnostics.
- `kg_bench_reports_missing_intent_table_support_failure` expects `MISSING_INTENT_SIGNAL` table support in a one-row signal-table fixture that actually recovers `XREQ`.
- The focused `table_support_failure` filter now runs all four canonical table-support diagnostic cases: wrong SemanticIR support id, wrong IntentIR support id, missing SemanticIR signal, and missing IntentIR signal.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG canonical table-support missing-signal diagnostic coverage)
- Continued from commit `5cdc36e`, tightening the canonical `signal_supporting_table_ids_include` missing-signal diagnostic.
- The diagnostic now reports the exact `signal_supporting_table_ids_include[<signal>]` field and the actual canonical signal set when a fixture expects table support for a signal absent from the canonical inventory.
- Added `kg_bench_reports_missing_canonical_table_support_failure`, which expects `MISSING_SIGNAL` table support in a one-row signal-table fixture that actually recovers `XREQ`.
- The focused `table_support_failure` filter now runs the wrong SemanticIR support-id diagnostic, wrong IntentIR support-id diagnostic, and missing canonical signal diagnostic together.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG IntentIR table-support diagnostic coverage)
- Continued from commit `c2bbd8f`, adding focused negative diagnostic coverage for canonical `signal_supporting_table_ids_include` expectations at the `IntentIR` stage.
- `kg_bench_reports_intent_table_support_failure` reuses the structured one-row signal-table fixture helper, expects `XREQ` to carry `missing_intent_signal_table`, and confirms the failure reports the fixture, `intent` stage, `signal_supporting_table_ids_include[XREQ]`, missing table id, and actual `table_protocol_signal_description` support set.
- This complements the previous SemanticIR diagnostic test so the canonical table-provenance carry-through path is now guarded at both canonical stages.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG canonical table-support diagnostic coverage)
- Continued from commit `ae4ea54`, adding focused negative diagnostic coverage for canonical `signal_supporting_table_ids_include` expectations.
- `kg_bench_reports_canonical_table_support_failure` reuses the structured one-row signal-table fixture helper, expects `XREQ` to carry `missing_signal_table`, and confirms the failure reports the fixture, `signal_supporting_table_ids_include[XREQ]`, the missing table id, and the actual `table_protocol_signal_description` support set.
- The one-row signal-table fixture helper now accepts full expectation objects so EvidenceIR and canonical-stage diagnostic tests share the same realistic `SourceIR` table patch without duplicating setup.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG EvidenceIR table-provenance count diagnostic coverage)
- Continued from commit `69351aa`, adding focused negative diagnostic coverage for the EvidenceIR table-provenance count expectation.
- `kg_bench_reports_evidence_table_provenance_count_failure` uses the shared one-row signal-table fixture helper, expects `table_signal_declaration_provenance_count: 0`, and confirms the resulting failure reports the fixture, expectation field, expected count, and actual count.
- This completes the immediate EvidenceIR table-provenance expectation diagnostic trio: count mismatch, missing signal/table provenance record, and mismatched synthesized statement text.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG EvidenceIR table-provenance count expectation)
- Continued from commit `cc7e533`, adding a direct EvidenceIR provenance count expectation to `specforge kg-bench`.
- `EvidenceStageExpectations.table_signal_declaration_provenance_count` checks `EvidenceIr.table_signal_declaration_provenance.len()` directly.
- `table_misclassification_field_table_negative` now asserts count `0`, proving the deliberately misclassified `Bits | Name | Description` table does not create fake table-backed signal declarations or provenance.
- The fixture also asserts the persisted EvidenceIR validation metric `table_signal_declaration_provenance: 0`, keeping exact staged expectations and validator output aligned.
- Focused validation passed for the strengthened field-table negative fixture; full tracked `kg-bench`, `corpus-kb`, docs CI, full local CI, and `git diff --check` also passed for this slice.

## Session update (2026-04-17 KG EvidenceIR missing-provenance diagnostic coverage)
- Continued from commit `5ae15c4`, adding the sibling negative branch for the EvidenceIR table-provenance expectation path.
- `kg_bench_reports_missing_evidence_table_provenance_failure` builds a temporary fixture with a real structured signal table, then expects `XREQ` provenance from a deliberately wrong table id.
- The test proves missing `table_signal_declaration_provenance_include` records report the fixture name, expectation field, missing expected table id, and actual EvidenceIR table id.
- The two EvidenceIR provenance diagnostic tests now share a one-row signal-table fixture writer, reducing duplicated JSON patch setup while preserving realistic staged fixture behavior.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG EvidenceIR table-provenance diagnostic coverage)
- Continued from commit `487389e`, adding focused negative coverage for the new EvidenceIR table-provenance fixture expectation path.
- `kg_bench_reports_evidence_table_provenance_statement_failure` builds a temporary fixture with a real structured signal table and deliberately expects the wrong synthesized statement text for `XREQ`.
- The first focused run exposed that statement-text mismatch diagnostics did not name `table_signal_declaration_provenance_include`.
- The diagnostic now names that field for statement-text mismatches as well as missing provenance records, so fixture failures point at the relevant schema field.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 EvidenceIR table provenance KG expectations)
- Continued from commit `43653a1`, adding exact EvidenceIR table-provenance fixture expectations rather than another aggregate metric.
- `specforge kg-bench` now accepts an `evidence` expectation section with `table_signal_declaration_provenance_include` entries.
- Each entry checks a required `signal_name` / `table_id` pair and can additionally verify the synthesized `statement_text` reached by the provenance record's `statement_id`.
- `signal_table_inventory_authority_negative` now proves that `XREQ`, `XACK`, and `PAYLOAD` each carry exact EvidenceIR provenance from `table_protocol_signal_description` before the canonical `supporting_table_ids` checks run.
- The focused fixture initially caught an incorrect `PAYLOAD` expectation of width `32`; the real synthesized statement is `Signal PAYLOAD is output width DATA_WIDTH.`, reflecting the table patch.
- Focused and full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 EvidenceIR table-signal provenance validation metric)
- Continued from commit `7d8e6e2`, adding EvidenceIR-side validation visibility for the table-signal provenance bridge.
- `specforge validate` now prints and persists `table_signal_declaration_provenance` for `EvidenceIR` by counting `EvidenceIr.table_signal_declaration_provenance`.
- The new focused validator regression builds markdown plus a structured `Signal | Direction | Width | Description` table and proves `XREQ`, `XACK`, and `PAYLOAD` each produce a provenance record linked to `table_protocol_signal_description`.
- `signal_table_inventory_authority_negative` now asserts the EvidenceIR validation metric value `3`, complementing the existing SemanticIR / IntentIR `with_table_support` metrics and exact canonical table-support expectations.
- This remains a visibility and regression metric only: it does not author truth, repair missing provenance, or promote facts.
- Focused validation passed for the new unit test and focused KG fixture; full tracked `kg-bench`, `corpus-kb`, docs CI, full local CI, and `git diff --check` also passed.

## Session update (2026-04-17 validator table-support unit coverage)
- Continued from commit `8d447e3`, adding direct unit coverage for the table-support validation metric rather than widening production behavior.
- `validate_semantic_and_intent_ir_count_signal_table_support` builds a real staged pipeline from a structured `Signal | Direction | Width | Description` table and checks that `SemanticIR` and `IntentIR` validation both report `with_table_support: 3`.
- This test complements `signal_table_inventory_authority_negative`: the fixture locks exact per-signal provenance and false-positive exclusion, while the unit test locks the validator metric over the canonical stages.
- Focused validation and full local CI passed; the suite now reports `320` Rust tests plus the mdBook build.

## Session update (2026-04-17 signal-table support validation metric)
- Continued from commit `2014352`, adding a narrow validator coverage metric over the already-committed table-provenance surface.
- `SemanticIR` validation now reports `with_table_support` by counting interface signal records with non-empty `supporting_table_ids`.
- `IntentIR` validation now reports `with_table_support` over declared non-`Low` signal records, matching the declared-signal coverage family.
- `signal_table_inventory_authority_negative` now asserts `with_table_support: 3` for both canonical stages while still requiring exact `signal_supporting_table_ids_include` expectations for `XREQ`, `XACK`, and `PAYLOAD`.
- Focused KG-bench validation, full `89/89` KG-bench validation, corpus-KB refresh, docs CI, full local CI, and `git diff --check` passed for this slice.

## Session update (2026-04-17 signal-table provenance carry-through)
- Continued from commit `e96ebe1`, adding a small IR provenance bridge plus KG expectation support.
- `EvidenceIR` now records `table_signal_declaration_provenance` when a structured signal-description table synthesizes a formal signal declaration statement.
- `SemanticIR` now resolves those synthesized statement ids back to table ids and stores them on `InterfaceSignalRecord.supporting_table_ids`; `IntentIR` carries the same canonical interface signal records forward.
- `specforge kg-bench` now supports `signal_supporting_table_ids_include` expectations for semantic and intent stages.
- `signal_table_inventory_authority_negative` now proves that `XREQ`, `XACK`, and `PAYLOAD` are not only recovered from the structured table with direction hints, but also retain `table_protocol_signal_description` as table support.
- Focused KG-bench validation, full `89/89` KG-bench validation, corpus-KB refresh, test compilation, docs CI, full local CI, and `git diff --check` passed for this slice.

## Session update (2026-04-17 signal-table inventory authority fixture)
- Continued from commit `664f2a5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative`.
- The fixture encodes the protocol-PDF reality that real signal inventory usually comes from dedicated signal/interface tables.
- Its markdown source contains only document/integration context terms, while a structured `Signal | Direction | Width | Description` table supplies `XREQ`, `XACK`, and `PAYLOAD`.
- The fixture requires all three table signals to survive through `SemanticIR` and `IntentIR` with table-derived directions, and excludes `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, `SoC`, and `SOC` from canonical signal inventory.
- Focused and full KG-bench validation passed with `89/89` fixtures; `corpus-kb` refreshed table extraction/hygiene coverage to `9/9` and truthfulness-negative/caution coverage to `36/36`; full local CI passed with `319` Rust tests plus the mdBook build.

## Session update (2026-04-17 KG signal-inventory exclusion expectations)
- Continued from commit `4aa4183`, adding a narrow test-harness feature in `crates/specforge/src/commands/kg_bench.rs`.
- `CanonicalStageExpectations` now supports `signal_names_exclude`, letting fixtures assert that specific terms are absent from the canonical interface signal inventory at `SemanticIR` and `IntentIR`.
- This complements the existing `signal_names_include` check and makes negative signal-inventory truthfulness executable instead of relying on absence-by-inspection.
- The first hardened fixture is `clock_reset_contract_scope_negative`, which now excludes `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `PLLs`, `DFT`, `SoC`, and `SOC` while still requiring `ACLK` and `ARESETN`.
- Focused and full KG-bench validation passed for the hardened fixture path, `corpus-kb` refreshed the tracked `88/88` fixture projections with `0` failures, and full local CI passed with `319` Rust tests plus the mdBook build.

## Session update (2026-04-17 clock/reset contract-scope KG fixture)
- Continued from commit `5d49cdb`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/clock_reset_contract_scope_negative`.
- The fixture makes the protocol-document scope boundary executable: the current document can define `ACLK` / `ARESETN` contract semantics for RTL and verification IP while explicitly saying final physical clock/reset tree construction belongs to the integrating SoC team.
- It preserves `ACLK` and active-low asynchronous `ARESETN` as first-class infrastructure signals, while requiring zero ordinary actor ports and zero concrete topology records through both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `88/88`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed benchmark, infrastructure, and semantic/truthfulness pattern projections from the `88`-fixture suite with `0` failures; infrastructure semantics coverage is `5/5`, infrastructure/polarity page coverage is `10/10`, and truthfulness-negative/caution coverage is `35/35`.
- Full local CI passed through formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 clock/reset protocol-document scope)
- Continued from commit `988e9bc`, with no Rust production-code change.
- The user clarified the intended domain boundary: AMBA, Intel, and similar protocol/chip-interface PDFs are contract documents for RTL designers and VIP creators, not complete sources for physical clock/reset tree construction.
- SPECFORGE should recover interface-visible clock/reset contract semantics from those PDFs: clock/reset identity, polarity, reset kind, asynchronous assertion / synchronous release discipline, and protocol-boundary timing obligations.
- Physical clock/reset trees remain team-specific SoC integration artifacts that depend on local clock generators, reset controllers, power domains, CDC/RDC policy, DFT/scan constraints, CTS, floorplan, and methodology.
- Future implementation should treat complete clock/reset tree synthesis as out of scope for protocol PDFs. Explicit topology phrases can remain bounded intent-level `infrastructure_topology` hints, but they are not signoff-quality physical tree recipes.
- Docs CI, full local CI with `319` Rust tests, and `git diff --check` passed for this documentation-only scope slice.

## Session update (2026-04-17 clock/reset generic-advice negative KG fixture)
- Continued from commit `4912ad7`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/clock_reset_generic_advice_negative`.
- The fixture preserves `ACLK` and active-low asynchronous `ARESETN` as first-class infrastructure signals while proving generic clock/reset doctrine does not create concrete `infrastructure_topology` records.
- The negative evidence covers glitch-avoidance clock-gate policy, no-glue reset-tree advice, possible synchronizer usage, and asynchronous assertion / synchronous release discipline.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `87/87`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed benchmark, infrastructure, and semantic/truthfulness pattern projections from the `87`-fixture suite with `0` failures; infrastructure semantics coverage is `4/4`, infrastructure/polarity page coverage is `9/9`, and truthfulness-negative/caution coverage is `34/34`.
- Full local CI passed through formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 FSMGEN response captured)
- FSMGEN responded to SPECFORGE's feedback in `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, observed at FSMGEN commit `7475f07`.
- SPECFORGE captured that response in `docs/FSMGEN_FEEDBACK.md` and `ROADMAP.md` so `.fsm` adapter planning can rely on the accepted sync contract.
- The concrete adapter posture is now explicit: target strict-mode canonical `.fsm`, block compatibility syntax by default, use FSMGEN's mdBook and regression corpus/support-accounting sources until machine-readable surfaces exist, and plan future adapter validation around capability manifests, stable diagnostic codes, check-only JSON, and normalized semantic JSON export.
- Longer-term `.fsm` language features remain valuable only when FSMGEN can parse, validate, normalize, document, support-account, and honestly lower or preserve them as checked metadata.

## Session update (2026-04-17 FSMGEN IntentIR-aligned feedback)
- `docs/FSMGEN_FEEDBACK.md` now frames SPECFORGE's feedback as `.fsm` language co-evolution, not only adapter validation tooling.
- The same handoff now introduces SPECFORGE explicitly as a Rust staged-IR toolchain whose product boundary is backend-independent `IntentIR`, so FSMGEN can read the feedback without prior project context.
- The added feature suggestions target natural `IntentIR` lowering: first-class system contracts, actor-relative ports, interface/channel grouping, temporal/stability contracts, semantic signal roles, generated assumptions/residual/provenance metadata, explicit direct-module root shape, and contract-aware composition.
- The support/tooling requests remain, but are now secondary to the language goal: capability manifest, JSON diagnostics, normalized AST/IR export, stable diagnostic codes, and adapter-facing examples make the richer `.fsm` contract executable.
- This preserves the SPECFORGE boundary: `IntentIR` remains canonical, but FSMGEN can evolve `.fsm` so fewer justified canonical facts are lost or blocked at adapter time.

## Session update (2026-04-17 FSMGEN reference sync)
- `subs/fsmgen` is now fast-forwarded from `57f00e5` to `955f2bb` for `.fsm` adapter reconnaissance.
- The updated FSMGEN reference includes its own live mdBook at `subs/fsmgen/docs/book/`, so SPECFORGE can consult both code and progressive user-facing documentation when widening `.fsm` lowering.
- The adapter-relevant FSMGEN direction is strict-mode/support-accounting, typed diagnostics, richer aggregate/type/package semantics, composition/toplink typing, and structural forward-IR separation.
- The SPECFORGE implementation stance stays unchanged: `IntentIR` remains canonical, `.fsm` is a downstream target, and FSMGEN is a read-only contextual reference from this repository.
- `docs/FSMGEN_FEEDBACK.md` is now the tracked handoff document for FSMGEN-facing feedback from SPECFORGE.
- Recommended FSMGEN feedback for future adapter leverage: machine-readable capability manifest, JSON check/diagnostic mode, normalized AST/IR export, richer reset/clock metadata, adapter-facing example corpus, and continued strict-mode-first support accounting.
- A local FSMGEN `./bin/ci-regression` run was intentionally stopped after the scope was clarified; it had reached `t/274-package-aggregate-values.t` with all reported tests green, so this sync is not recorded as a full FSMGEN validation pass.

## Session update (2026-04-17 direct `.fsm` control-read input recovery)
- Continued from commit `1a8c424`, completing the immediate pair to target-actor selection: after a direct root selects a target actor from graph-owned outputs, explicit control reads can now recover that target actor's input directions.
- `crates/specforge/src/ir/adapters.rs` now collects read references from DT guards/actions, rich control selectors/predicates/actions, and state-transition guards, then overlays `direct_control_input` only for local inventory signals that are not direct output targets.
- The new regression proves `DATA_IN` can be recovered as a target input from `DATA_OUT = DATA_IN` and a `DATA_IN` guard even when the structural KG only says an external `environment` drives `DATA_IN`.
- Existing standalone direct ambiguity, conflict, unrelated-actor, and output-owner tests remain green, keeping the recovery adapter-local and bounded.
- Full local CI passed through formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 direct `.fsm` target-actor selection)
- Continued from commit `14d6496`, deepening the `R15` graph-first adapter path rather than adding another flat direction-hint workaround.
- `crates/specforge/src/ir/adapters.rs` now selects a standalone direct target actor from graph-backed output-target ownership before falling back to the older one-actor direct context rule.
- The new regression clears flat direct-interface directions and proves the adapter still lowers when `controller` drives all direct outputs while an external `environment` drives an input and `monitor` reads the outputs.
- Existing standalone direct ambiguity, direction-conflict, width-conflict, unrelated-actor, and unambiguous-context tests remain green, keeping the widening bounded.
- Full local CI passed through formatting, warning-deny Clippy, `318` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 coordinated active-read coverage)
- Continued from commit `4cb4b53`, strengthening the read/sample side of the active-object extraction path.
- `relative_clause_actor_noise_negative` now uses one coordinated consumer sentence: `The Manager samples ARCHUNKEN and RCHUNKV.`
- Added focused regressions proving coordinated active-read clauses recover every sampled object and do not cross guard markers such as `when RVALID is HIGH`.
- Focused KG-bench validation still passes for `relative_clause_actor_noise_negative`, proving the staged `SemanticIR` / `IntentIR` graph carries both `Manager reads ARCHUNKEN` and `Manager reads RCHUNKV`.
- Full tracked KG-bench validation reports `86/86`, and full local CI passed through formatting, warning-deny Clippy, `317` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 relative-clause actor-noise KG fixture)
- Continued from commit `019f536`, with a production-code follow-on in `crates/specforge/src/ir/evidence.rs` plus a new tracked KG fixture under `crates/specforge/test_data/kg_quality/relative_clause_actor_noise_negative`.
- `relative_clause_actor_noise_negative` locks AXI-style chunking prose where an `interconnect` with a relative clause can drive both `ARCHUNKEN` and `RCHUNKV`.
- The fixture asserts graph direction, actor-signal relations, actor ports, zero signal-connectivity conflicts, and validation metrics through both `SemanticIR` and `IntentIR`.
- The first fixture run exposed that coordinated active-drive objects after the same verb only recovered the first signal; active object parsing now scans the verb's current clause so both `ARCHUNKEN` and `RCHUNKV` inherit the same head actor.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `86/86`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed benchmark and semantic/truthfulness corpus-KB pages from the `86`-fixture suite with `0` failures; actor-connectivity coverage is `12/12` and truthfulness-negative/caution coverage is `33/33`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `315` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 relative-clause actor extraction hygiene)
- Continued from commit `02d4be1`, with production-code changes in `crates/specforge/src/ir/evidence.rs` and `crates/specforge/src/ir/prior_memory.rs`.
- `extract_subject_phrase()` now limits active-drive subject parsing to the current sentence and strips relative clauses before choosing the candidate actor.
- `is_meaningful_actor_term()` rejects `mixture` / `mixture of`, so descriptive support phrases cannot become protocol actors or learned actor-taxonomy terms.
- Added a regression for AXI chunking prose proving `interconnect` remains the `ARCHUNKEN` actor and `mixture of` is not promoted.
- Rebuilt local AXI `EvidenceIR`, `SemanticIR`, and `IntentIR` from the existing generated `SourceIR` and validated the result; the live score remains `85/100 GOOD`, the fake `mixture of` producer is gone, and the remaining `ARCHUNKEN` conflict is `Manager` versus `interconnect`.
- Focused actor-extraction tests passed, and full local CI passed through formatting, warning-deny Clippy, `314` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AHB exclusive/security wait-state stability KG fixture)
- Continued from commit `264be3e`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_exclusive_security_stability_gold`.
- `ahb_exclusive_security_stability_gold` locks AHB wait-state exclusive/security stability for `HEXCL`, `HNONSEC`, and `HEXOKAY`.
- The fixture uses `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables, proving `HEXCL`, `HNONSEC`, `HSEL`, `HREADY`, and `HEXOKAY` become graph-backed Manager/Subordinate actor relations and actor-relative ports.
- `HEXCL` and `HNONSEC` stability are Manager-grounded while `HEXOKAY` stability is Subordinate-grounded, all under `HREADY LOW` and `HSEL HIGH`, with no handshake-completion predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `85/85`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `85`-fixture suite with `0` failures; AMBA-family coverage is `36/36` and temporal-family coverage is `42/42`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AHB transfer/lock wait-state stability KG fixture)
- Continued from commit `64bc735`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_transfer_lock_stability_gold`.
- `ahb_transfer_lock_stability_gold` locks AHB wait-state transfer/lock stability for `HTRANS` and `HMASTLOCK`.
- The fixture uses `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables, proving `HTRANS`, `HMASTLOCK`, `HSEL`, and `HREADY` become graph-backed Manager/Subordinate actor relations and actor-relative ports.
- `HTRANS` and `HMASTLOCK` stability are Manager-grounded under `HREADY LOW` and `HSEL HIGH`, with no handshake-completion predicate, so stalled transfer/control obligations stay distinct from completed transfers.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `84/84`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `84`-fixture suite with `0` failures; AMBA-family coverage is `35/35` and temporal-family coverage is `41/41`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 APB address/protection wait-state stability KG fixture)
- Continued from commit `62fdca5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/apb_address_protection_stability_gold`.
- `apb_address_protection_stability_gold` locks APB wait-state address/protection stability for `PADDR` and `PPROT`.
- The fixture uses a `Signal | Source | Width | Description` table plus prose actor relations, proving `PSEL`, `PENABLE`, `PREADY`, `PADDR`, and `PPROT` become graph-backed Requester/Completer actor relations and actor-relative ports.
- `PSEL` and `PREADY` resolve to valid-like / ready-like roles from local table descriptions; `PADDR` and `PPROT` stability are Requester-grounded under `PSEL HIGH`, `PENABLE HIGH`, and `PREADY LOW`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `83/83`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `83`-fixture suite with `0` failures; AMBA-family coverage is `34/34` and temporal-family coverage is `40/40`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AXI address/response USER sideband stability KG fixture)
- Continued from commit `ac0e91b`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_address_response_user_sideband_stability_gold`.
- `axi_address_response_user_sideband_stability_gold` locks AXI address-channel and write-response `USER` sideband stability for `AWUSER`, `ARUSER`, and `BUSER`.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, `AWUSER`, `ARVALID`, `ARREADY`, `ARUSER`, `BVALID`, `BREADY`, and `BUSER` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` / `AWREADY`, `ARVALID` / `ARREADY`, and `BVALID` / `BREADY` resolve to valid-like / ready-like roles from local table descriptions; `AWUSER` and `ARUSER` stability are Manager-grounded, while `BUSER` stability is Subordinate-grounded.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `82/82`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `82`-fixture suite with `0` failures; AMBA-family coverage is `33/33` and temporal-family coverage is `39/39`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AXI data USER sideband stability KG fixture)
- Continued from commit `672c3b2`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_data_user_sideband_stability_gold`.
- `axi_data_user_sideband_stability_gold` locks AXI data-channel `USER` sideband stability for `WUSER` and `RUSER`.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `WVALID`, `WREADY`, `WUSER`, `RVALID`, `RREADY`, and `RUSER` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `WVALID` / `WREADY` and `RVALID` / `RREADY` resolve to valid-like / ready-like roles from local table descriptions; `WUSER` stability is Manager-grounded, while `RUSER` stability is Subordinate-grounded.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `81/81`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `81`-fixture suite with `0` failures; AMBA-family coverage is `32/32` and temporal-family coverage is `38/38`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI address QoS/region sideband stability KG fixture)
- Continued from commit `59881b3`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_address_qos_region_sideband_stability_gold`.
- `axi_address_qos_region_sideband_stability_gold` locks paired AXI address-channel QoS/region stability for `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION`.
- The fixture uses one width-only `Name | Width | Description` table plus prose actor relations, proving both write-address and read-address sideband sets become graph-backed Manager/Subordinate actor relations and actor-relative ports without direction columns.
- `AWVALID` / `AWREADY` and `ARVALID` / `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and each QoS/region stability rule requires the matching `HandshakeComplete(...)` predicate plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `80/80`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `80`-fixture suite with `0` failures; AMBA-family coverage is `31/31` and temporal-family coverage is `37/37`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address control sideband stability KG fixture)
- Continued from commit `997da1a`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_control_sideband_stability_gold`.
- `axi_read_address_control_sideband_stability_gold` locks AXI read-address control sideband stability for `ARPROT`, `ARCACHE`, and `ARLOCK` when `ARVALID` and `ARREADY` complete the read-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `ARVALID`, `ARREADY`, `ARPROT`, `ARCACHE`, and `ARLOCK` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `ARVALID` and `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and each control-sideband stability rule requires `HandshakeComplete(ARVALID, ARREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `79/79`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `79`-fixture suite with `0` failures; AMBA-family coverage is `30/30` and temporal-family coverage is `36/36`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-address control sideband stability KG fixture)
- Continued from commit `b3c0820`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_address_control_sideband_stability_gold`.
- `axi_write_address_control_sideband_stability_gold` locks AXI write-address control sideband stability for `AWPROT`, `AWCACHE`, and `AWLOCK` when `AWVALID` and `AWREADY` complete the write-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, `AWPROT`, `AWCACHE`, and `AWLOCK` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` and `AWREADY` resolve to valid-like / ready-like roles from local table descriptions, and each control-sideband stability rule requires `HandshakeComplete(AWVALID, AWREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `78/78`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `78`-fixture suite with `0` failures; AMBA-family coverage is `29/29` and temporal-family coverage is `35/35`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address ID stability KG fixture)
- Continued from commit `b9f69d5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_id_stability_gold`.
- `axi_read_address_id_stability_gold` locks AXI read-address ID stability for `ARID` when `ARVALID` and `ARREADY` complete the read-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `ARVALID`, `ARREADY`, and `ARID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `ARVALID` and `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `ARID` stability rule requires `HandshakeComplete(ARVALID, ARREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `77/77`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `77`-fixture suite with `0` failures; AMBA-family coverage is `28/28` and temporal-family coverage is `34/34`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-address ID stability KG fixture)
- Continued from commit `883366f`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_address_id_stability_gold`.
- `axi_write_address_id_stability_gold` locks AXI write-address ID stability for `AWID` when `AWVALID` and `AWREADY` complete the write-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, and `AWID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` and `AWREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `AWID` stability rule requires `HandshakeComplete(AWVALID, AWREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `76/76`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `76`-fixture suite with `0` failures; AMBA-family coverage is `27/27` and temporal-family coverage is `33/33`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data ID stability KG fixture)
- Continued from commit `cf2fc42`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_id_stability_gold`.
- `axi_read_data_id_stability_gold` locks AXI read-data ID stability for `RID` when `RVALID` and `RREADY` complete the read-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `RVALID`, `RREADY`, and `RID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `RVALID` and `RREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `RID` stability rule requires `HandshakeComplete(RVALID, RREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `75/75`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `75`-fixture suite with `0` failures; AMBA-family coverage is `26/26` and temporal-family coverage is `32/32`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-response ID stability KG fixture)
- Continued from commit `11592f5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_response_id_stability_gold`.
- `axi_write_response_id_stability_gold` locks AXI write-response ID stability for `BID` when `BVALID` and `BREADY` complete the write-response channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `BVALID`, `BREADY`, and `BID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `BVALID` and `BREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `BID` stability rule requires `HandshakeComplete(BVALID, BREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `74/74`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `74`-fixture suite with `0` failures; AMBA-family coverage is `25/25` and temporal-family coverage is `31/31`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data response stability KG fixture)
- Continued from commit `14e53ed`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold`.
- `axi_read_data_response_stability_gold` locks AXI read-data response stability for `RRESP` when `RVALID` and `RREADY` complete the read-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `RVALID`, `RREADY`, and `RRESP` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `RVALID` and `RREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `RRESP` stability rule requires `HandshakeComplete(RVALID, RREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `73/73`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `73`-fixture suite with `0` failures; AMBA-family coverage is `24/24` and temporal-family coverage is `30/30`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address sideband stability KG fixture)
- Continued from commit `fec0137`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_sideband_stability_gold`.
- `axi_read_address_sideband_stability_gold` locks AXI read-address sideband stability for `ARSIZE` and `ARBURST` when `ARVALID` and `ARREADY` complete the read-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `ARVALID`, `ARREADY`, `ARSIZE`, and `ARBURST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `ARVALID` and `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and both sideband stability rules require `HandshakeComplete(ARVALID, ARREADY)` plus Manager-grounded `actor_maintains_signal_stable` consequents.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `72/72`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `72`-fixture suite with `0` failures; AMBA-family coverage is `23/23` and temporal-family coverage is `29/29`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-data last stability KG fixture)
- Continued from commit `d21252a`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold`.
- `axi_write_data_last_stability_gold` locks AXI write-data last-beat stability for `WLAST` when `WVALID` and `WREADY` complete the write-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `WVALID`, `WREADY`, and `WLAST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `WVALID` and `WREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `WLAST` stability rule requires `HandshakeComplete(WVALID, WREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `71/71`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `71`-fixture suite with `0` failures; AMBA-family coverage is `22/22` and temporal-family coverage is `28/28`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data last stability KG fixture)
- Continued from commit `b0b20c1`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold`.
- `axi_read_data_last_stability_gold` locks AXI read-data last-beat stability for `RLAST` when `RVALID` and `RREADY` complete the read-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `RVALID`, `RREADY`, and `RLAST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `RVALID` and `RREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `RLAST` stability rule requires `HandshakeComplete(RVALID, RREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `70/70`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `70`-fixture suite with `0` failures; AMBA-family coverage is `21/21` and temporal-family coverage is `27/27`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-address sideband stability KG fixture)
- Continued from commit `746bf5d`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold`.
- `axi_write_address_sideband_stability_gold` locks AXI write-address sideband stability for `AWLEN`, `AWSIZE`, and `AWBURST` when `AWVALID` and `AWREADY` complete the channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, `AWLEN`, `AWSIZE`, and `AWBURST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` and `AWREADY` resolve to valid-like / ready-like roles from local table descriptions, and all three sideband stability rules require `HandshakeComplete(AWVALID, AWREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `69/69`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `69`-fixture suite with `0` failures; AMBA-family coverage is `20/20` and temporal-family coverage is `26/26`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AHB write-data stability KG fixture)
- Continued from commit `598ed3f`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold`.
- `ahb_write_data_stability_gold` locks AHB wait-state write-data stability for `HWDATA` when `HREADY` is low, `HSEL` is high, and `HWRITE` is high.
- The fixture asserts `HSEL`, `HWRITE`, `HWDATA`, and `HREADY` as graph-backed Manager/Subordinate actor relations and actor-relative ports using `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables.
- The temporal rule is actor-grounded to a `Manager` stability obligation and intentionally asserts `temporal_rules_with_handshake_completion = 0`, proving three-predicate write wait-state stability does not become a false completed-handshake predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `68/68`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `68`-fixture suite with `0` failures; AMBA-family coverage is `19/19` and temporal-family coverage is `25/25`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AHB response stability KG fixture)
- Continued from commit `bbde6db`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_response_stability_gold`.
- `ahb_response_stability_gold` locks AHB wait-state response stability for `HRDATA` and `HRESP` when `HREADY` is low and `HSEL` is high.
- The fixture asserts `HSEL`, `HREADY`, `HRDATA`, and `HRESP` as graph-backed Manager/Subordinate actor relations and actor-relative ports using `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables.
- The two temporal rules are actor-grounded to `Subordinate` stability obligations and intentionally assert `temporal_rules_with_handshake_completion = 0`, proving response-side wait-state stability does not become a false completed-handshake predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `67/67`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `67`-fixture suite with `0` failures; AMBA-family coverage is `18/18` and temporal-family coverage is `24/24`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AHB control stability KG fixture)
- Continued from commit `6801c19`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_control_stability_gold`.
- `ahb_control_stability_gold` locks AHB wait-state address/control stability for `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` when `HREADY` is low and `HSEL` is high.
- The fixture asserts `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, `HPROT`, `HSEL`, and `HREADY` as graph-backed Manager/Subordinate actor relations and actor-relative ports using `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables.
- The five temporal rules are actor-grounded to `Manager` stability obligations and intentionally assert `temporal_rules_with_handshake_completion = 0`, proving wait-state stability does not become a false completed-handshake predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `66/66`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `66`-fixture suite with `0` failures; AMBA-family coverage is `17/17` and temporal-family coverage is `23/23`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 APB response stability KG fixture)
- Continued from commit `e9310c8`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/apb_response_stability_gold`.
- `apb_response_stability_gold` locks APB completed-access response stability for `PRDATA` and `PSLVERR` when `PSEL`, `PENABLE`, and `PREADY` are high.
- The fixture asserts `PSEL`, `PENABLE`, `PREADY`, `PRDATA`, and `PSLVERR` as graph-backed actor relations and actor-relative ports, with `PSEL` / `PREADY` resolved to valid-like / ready-like roles from table descriptions.
- The two temporal rules are actor-grounded to `Completer` stability obligations and require `HandshakeComplete(PSEL, PREADY)`, giving a contrast to the prior `PREADY LOW` wait-state fixture that intentionally had zero handshake-completion predicates.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `65/65`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `65`-fixture suite with `0` failures; AMBA-family coverage is `16/16` and temporal-family coverage is `22/22`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 APB write-control stability KG fixture)
- Continued from commit `599e577`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold`.
- `apb_write_control_stability_gold` locks APB wait-state stability for `PWRITE`, `PWDATA`, and `PSTRB` when `PSEL` and `PENABLE` are high and `PREADY` is low.
- The fixture asserts `PSEL`, `PENABLE`, `PREADY`, `PWRITE`, `PWDATA`, and `PSTRB` as graph-backed actor relations and actor-relative ports, with `PSEL` / `PREADY` resolved to valid-like / ready-like roles from table descriptions.
- The three temporal rules are actor-grounded to `Requester` stability obligations and intentionally assert `temporal_rules_with_handshake_completion = 0`, proving the wait-state guard does not become a false completed handshake.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `64/64`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `64`-fixture suite with `0` failures; AMBA-family coverage is `15/15` and temporal-family coverage is `21/21`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI sideband stability KG fixture)
- Continued from commit `ccfd99e`, with `30` Rust source files and `58,564` lines under `crates/specforge/src` after tightening the corpus-KB fixture-family classifier.
- `axi_sideband_stability_gold` locks sideband stability across two AXI channel fragments: `ARLEN` must remain stable when `ARVALID` / `ARREADY` complete, and `WSTRB` must remain stable when `WVALID` / `WREADY` complete.
- The fixture asserts graph-backed actor relations and actor ports for `ARVALID`, `ARREADY`, `ARLEN`, `WVALID`, `WREADY`, and `WSTRB`, resolved valid-like / ready-like semantic roles for the controlling handshakes, single-source semantic grounding, and actor-grounded stability consequents for `Manager` at both `SemanticIR` and `IntentIR`.
- `corpus_kb` fixture-family labeling now treats `stability` fixture names as temporal semantics, so sideband hold coverage appears on both the AMBA protocol page and the timing motif page.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `63/63`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `63`-fixture suite with `0` failures; AMBA-family coverage is `14/14` and temporal-family coverage is `20/20`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address timing KG fixture)
- Continued from commit `242caca`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold`.
- `axi_read_address_timing_gold` locks an AXI read-address channel with a width-only signal table plus prose actor relations: `ARVALID`, `ARADDR`, and `ARLEN` are driven by `Manager`, `ARREADY` is driven by `Subordinate`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `ARREADY` assertion, and actor-grounded `ARADDR` stability under `ARVALID` / `ARREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `62/62`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `62`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-data timing KG fixture)
- Continued from commit `f8c4faa`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold`.
- `axi_write_data_timing_gold` locks an AXI write-data channel with a width-only signal table plus prose actor relations: `WVALID`, `WDATA`, and `WSTRB` are driven by `Manager`, `WREADY` is driven by `Subordinate`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `WREADY` assertion, and actor-grounded `WDATA` stability under `WVALID` / `WREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `61/61`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `61`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data timing KG fixture)
- Continued from commit `3414a6d`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold`.
- `axi_read_data_timing_gold` locks an AXI read-data channel with a width-only signal table plus prose actor relations: `RVALID`, `RDATA`, and `RRESP` are driven by `Subordinate`, `RREADY` is driven by `Manager`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `RVALID` assertion, and actor-grounded `RDATA` stability under `RVALID` / `RREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `60/60`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `60`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-15 AXI write-response timing KG fixture)
- Continued from commit `bc110c3`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold`.
- `axi_write_response_timing_gold` locks an AXI write-response channel with a width-only signal table plus prose actor relations: `BVALID` and `BRESP` are driven by `Subordinate`, `BREADY` is driven by `Manager`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `BVALID` assertion, and actor-grounded `BRESP` stability under `BVALID` / `BREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `59/59`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `59`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 GitHub Actions manual-only cost control)
- Continued from commit `6cd5170` after the KG-bench semantic-grounding strength expectation slice.
- `.github/workflows/ci.yml` now exposes only `workflow_dispatch`, temporarily removing automatic `push` / `pull_request` triggers to conserve account GitHub Actions minutes.
- The hosted workflow still delegates to `./scripts/run_ci.sh`, so manual GitHub runs and local validation continue to exercise the same formatting, Clippy, warning-deny Rust test, warning-deny rustdoc, and mdBook gate.
- Trigger inspection confirms the workflow has no active automatic `push` or `pull_request` trigger keys.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench semantic-grounding strength expectations)
- Continued from commit `ac7e3f4`, with `30` Rust source files and `58,559` lines under `crates/specforge/src` after extending KG-bench semantic-grounding strength expectations.
- `kg-bench` can now assert canonical `InterfaceSignalRecord.semantic_grounding_strength` directly at the `SemanticIR` and `IntentIR` stages through `semantic_grounding_strengths_include`.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so `XREQ`, `XACK`, `PSEL`, and `PREADY` lock exact single-source / cross-modality canonical grounding strength rather than only aggregate validation counters.
- Focused KG-bench validation passed for all four strengthened semantic-grounding fixtures; the full tracked KG suite reports `58/58` passing fixtures.
- Focused `kg_bench` Rust target passed with the tracked fixture suite reporting `58/58`, and full local CI passed through `bash scripts/run_ci.sh` with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench resolved semantic-role shape expectations)
- Continued from commit `d85a061`, with `30` Rust source files and `58,530` lines under `crates/specforge/src` after extending KG-bench resolved semantic-role expectations.
- `kg-bench` can now assert canonical `InterfaceSignalRecord.resolved_semantic_role` directly at the `SemanticIR` and `IntentIR` stages through `resolved_semantic_roles_include`.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so `XREQ`, `XACK`, `PSEL`, and `PREADY` lock exact valid-like / ready-like canonical role shape rather than only aggregate role-presence metrics.
- Focused KG-bench validation passed for all four strengthened semantic-role fixtures; the full tracked KG suite reports `58/58` passing fixtures.
- Focused `kg_bench` Rust target passed with the tracked fixture suite reporting `58/58`, and full local CI passed through `bash scripts/run_ci.sh` with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench resolved signal-polarity shape expectations)
- Continued from commit `9cb41c8`, with `30` Rust source files and `58,500` lines under `crates/specforge/src` after extending KG-bench resolved signal-polarity expectations.
- `kg-bench` can now assert canonical `InterfaceSignalRecord.resolved_polarity` directly at the `SemanticIR` and `IntentIR` stages through `signal_polarities_include`.
- Strengthened `non_reset_control_polarity_gold`, `multi_control_polarity_gold`, and `mixed_control_polarity_gold` so `CS_N`, `WE_N`, and `ENABLE` lock exact active-low / active-high canonical polarity rather than only aggregate `with_resolved_polarity` metrics.
- Focused KG-bench validation passed for all three strengthened polarity fixtures; the full tracked KG suite reports `58/58` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench signal-polarity conflict shape expectations)
- Continued from commit `010d9e2`, with `30` Rust source files and `58,471` lines under `crates/specforge/src` after extending KG-bench signal-polarity conflict expectations.
- `kg-bench` can now assert canonical `signal_polarity_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, observation polarity, observation source kind, supporting statement ids, and supporting table ids.
- Added `control_polarity_conflict_negative`, proving `PRESETN` active-high prose evidence and active-low signal-description-table evidence remain visible as a carried polarity conflict rather than forcing a winner.
- Focused KG-bench validation passed for the new polarity-conflict fixture; the full tracked KG suite reports `58/58` passing fixtures.
- The corpus-KB benchmark, pattern, and infrastructure/polarity fixture pages refreshed from the `58`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench interface-signal conflict shape expectations)
- Continued from commit `e1e629e`, with `30` Rust source files and `58,377` lines under `crates/specforge/src` after extending KG-bench interface-signal conflict expectations.
- `kg-bench` can now assert canonical `interface_signal_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, conflict kind, and included observation values plus supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_interface_conflict_caution_gold` so the `DATA` conflict is asserted as a `direction_mismatch` over `input` / `output` and a `width_mismatch` over `8` / `16`, including under prior-memory caution.
- Focused KG-bench validation passed for the strengthened interface-conflict fixture; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench signal-connectivity conflict shape expectations)
- Continued from commit `5b56ce3`, with `30` Rust source files and `58,291` lines under `crates/specforge/src` after extending KG-bench signal-connectivity conflict expectations.
- `kg-bench` can now assert canonical `signal_connectivity_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, conflict kind, conflicting actor ids/names, and supporting statement ids.
- Strengthened `multi_producer_conflict_negative` and `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` so the `PREADY` graph conflict is asserted as a `multiple_producers` conflict involving `Completer` and `Monitor`, including under prior-memory caution.
- Focused KG-bench validation passed for both strengthened connectivity-conflict fixtures; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench signal-semantic conflict shape expectations)
- Continued from commit `92344fa`, with `30` Rust source files and `58,215` lines under `crates/specforge/src` after extending KG-bench signal-semantic conflict expectations.
- `kg-bench` can now assert canonical `signal_semantic_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, and included observations by semantic tags, source kind, source text, and supporting statement/table/visual evidence ids.
- Strengthened `visual_sources_semantic_conflict_negative` so the `XCTRL` multimodal disagreement is asserted as visual-caption `handshake_valid_like` evidence versus VLM timing-diagram annotation `handshake_ready_like` evidence, with arbitration still non-decisive.
- Focused KG-bench validation passed for the strengthened visual semantic-conflict fixture; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench temporal-conflict shape expectations)
- Continued from commit `7490cb5`, with `30` Rust source files and `58,104` lines under `crates/specforge/src` after extending KG-bench temporal-conflict expectations.
- `kg-bench` can now assert canonical `temporal_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over signal name, phase, clock signal, edge, cycle window, antecedents, conflicting values, supporting rule ids, and supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_temporal_conflict_caution_gold` so the current-document `PREADY` post-tick `HIGH` / `LOW` contradiction under `HREADY LOW` stays asserted as typed conflict shape even when a matching negative-knowledge prior adds caution/rescan/corroboration guidance.
- Focused KG-bench validation passed for the strengthened temporal-conflict fixture; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench temporal-rule shape expectations)
- Continued from commit `f99956e`, with `30` Rust source files and `58,010` lines under `crates/specforge/src` after extending KG-bench temporal-rule expectations.
- `kg-bench` can now assert canonical `temporal_rules` directly at the `SemanticIR` and `IntentIR` stages using partial matches over source text, clock signal, edge, cycle window, supporting statement ids, antecedents, and consequents.
- Strengthened `axi_next_cycle_timing_gold`, `apb_setup_access_timing_gold`, and `ahb_wait_state_timing_gold` so representative AXI/APB/AHB timing fixtures now lock typed actor-grounded consequents, compound antecedents, handshake-completion predicates, and one-cycle windows directly.
- Focused KG-bench validation passed for all three strengthened timing fixtures; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench clock/reset infrastructure topology)
- Continued from commit `37cd89a`, with `30` Rust source files and `57,931` lines under `crates/specforge/src` after extending KG-bench infrastructure expectations.
- `kg-bench` can now assert canonical `infrastructure_signals` and `infrastructure_topology` records directly at the `SemanticIR` and `IntentIR` stages.
- Added `clock_reset_topology_gold`, proving explicit current-document clock-gate, reset-synchronizer, and reset-tree topology survives canonically while generic topology advice stays non-authoring and ordinary actor ports remain absent.
- The tracked KG-quality suite now reports `57/57` passing fixtures, and the corpus-KB benchmark plus infrastructure/polarity pages were refreshed from that suite.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 VLM timing motion-only annotation filtering)
- Continued from commit `7ce78ea`, with `30` Rust source files and `57,712` lines under `crates/specforge/src` after adding the VLM timing annotation filter.
- `SemanticIR` now treats non-quantitative waveform-motion prose in VLM timing `annotations[]` as annotation markup rather than timing law when no timing/constraint indicators or numeric/cycle anchors are present.
- The new `vlm_timing_motion_annotation_negative` KG fixture failed before the parser fix with two unexpected timing constraints, and now passes with zero timing constraints while preserving one concrete signal constraint / temporal rule from the document-grounded `HIGH` sample.
- Positive VLM annotation coverage still passes: fenced setup/hold-style timing annotations and direct VLM timing-note semantic grounding remain accepted.
- The tracked KG-quality suite now reports `56/56` passing fixtures, and the corpus-KB benchmark/timing/visual/pattern pages were refreshed from that suite.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 R15g prior-candidate readiness manifest)
- Continued from commit `c2e6084`, with `30` Rust source files and `57,614` lines under `crates/specforge/src` after adding the prior-candidate readiness manifest path.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes a tracked schema-versioned `corpus_kb/prior_candidates/kg-fixture-candidates.json` manifest beside the existing Markdown prior-candidate page.
- The manifest records candidate kind, target `CorpusMemory` schema, readiness, fixture counts, supporting/positive/guard fixture names, structured gate identifiers, and the explicit `review_only_no_corpus_memory_or_canonical_ir_mutation` promotion boundary.
- The Markdown prior-candidate page now mirrors this with a `Readiness Summary`, including `fixture_paired_review_ready` for paired prior fixture families and `caution_surface_review_ready` for caution-only negative-knowledge coverage.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, docs CI passed, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g typed prior-memory corpus KB page)
- Continued from commit `fd716eb`, with `30` Rust source files and `57,453` lines under `crates/specforge/src` after adding the typed prior-memory corpus-KB page family.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_memory/kg-fixtures.md` from the existing `typed prior memory` fixture-family label.
- The current tracked refresh projects `21/21` passing typed prior-memory fixtures spanning actor-taxonomy, semantic phrase, semantic modality-reliability, temporal phrase, table-shape, visual semantic, visual-motif, and caution-only negative-knowledge behavior.
- The page is review-only corpus synthesis. It preserves fixture provenance and cannot write `generated/prior_memory/corpus_memory.json`, mutate `CorpusMemory`, promote canonical IR, or approve prior records.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, docs CI passed, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g state-machine corpus KB page)
- Continued from commit `91492cc`, with `30` Rust source files and `57,433` lines under `crates/specforge/src` after adding the state-machine corpus-KB page family.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/state_machines/kg-fixtures.md` from the existing `VLM state machines` fixture-family label.
- The current tracked refresh projects `5/5` passing state-machine fixtures: duplicate-initial merge, label-noise filtering, missing-initial warning, multiple-initial warning, and undeclared-transition endpoint filtering.
- The page is review-only corpus synthesis. It preserves fixture provenance and cannot promote VLM state-machine hypotheses into canonical IR, typed prior memory, validation scoring, or adapter lowering.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, docs CI passed, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g prior-candidate gate matrix)
- Continued from commit `2286f86`, with `30` Rust source files and `57,395` lines under `crates/specforge/src` after adding the prior-candidate gate review matrix.
- `specforge corpus-kb --kg-fixtures-root ...` now emits `review_scope: family_surface_not_individual_prior` in `corpus_kb/prior_candidates/kg-fixture-candidates.md`.
- The managed prior-candidate projection now includes a `Promotion Gate Review Matrix` with family-level schema, fixture, harvest, consumer, and non-mutation boundary gates for all seven `CorpusMemory` prior families.
- The matrix is deliberately review-only: it records visible implementation surfaces and still ends every row with `review_only_no_corpus_memory_or_canonical_ir_mutation`.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g semantic/truthfulness corpus KB patterns)
- Continued from commit `b3c05f5`, with `30` Rust source files and `57,326` lines under `crates/specforge/src` after adding the semantic/truthfulness pattern-page projection.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/patterns/kg-fixtures.md` from the same tracked KG fixture run, selecting actor/connectivity, semantic role arbitration, negative-knowledge, truthfulness-negative/caution, and residual/caveat fixture labels.
- The current live refresh projects `42/42` passing semantic/truthfulness pattern fixtures with explicit fixture-path provenance while preserving human synthesis outside the managed block.
- The page is corpus synthesis only: it can guide review, debugging, benchmark design, and arbitration work, but it does not affect fixture execution, validation scoring, `CorpusMemory`, or canonical IR.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g prior-candidate corpus KB projection)
- Continued from commit `0f3166f`, with `30` Rust source files and `57,283` lines under `crates/specforge/src` after adding the prior-candidate projection path.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_candidates/kg-fixture-candidates.md` from KG fixtures that already encode prior-guided gold/negative/caution behavior.
- The managed projection groups candidate kinds by target `CorpusMemory` schema surface, supporting fixture count, positive fixtures, guard/caution fixtures, and required gates while keeping `promotion_status: candidate_not_promoted_review_required`, `canonical_mutation_allowed: false`, and `corpus_memory_mutation_allowed: false`.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g dedicated corpus KB fixture-family pages)
- Continued from commit `8681b13`, with `30` Rust source files and `57,076` lines under `crates/specforge/src` after the multi-page corpus-KB refresh update.
- The KG fixture refresh now writes the aggregate benchmark page plus dedicated family pages under `corpus_kb/tables/`, `corpus_kb/visuals/`, `corpus_kb/timing/`, `corpus_kb/infra/`, and `corpus_kb/protocols/`.
- The current tracked refresh projects table fixtures `8/8`, visual fixtures `18/18`, timing fixtures `14/14`, infrastructure/polarity fixtures `6/6`, and AMBA-family fixtures `9/9`, all with fixture-path provenance and all remaining review-only corpus synthesis.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g KG fixture-family corpus KB summary)
- Continued from commit `62be333`, with `30` Rust source files and `56,860` lines under `crates/specforge/src` after the corpus-KB family-summary projection update.
- `specforge corpus-kb --kg-fixtures-root ...` now renders a managed fixture-family summary table before the per-fixture KG benchmark list, so the corpus KB exposes coverage across VLM timing/state-machine, actor connectivity, multimodal visual grounding, negative knowledge, polarity, AMBA-family protocol, semantic arbitration, table hygiene, temporal, truthfulness-negative, and typed-prior-memory families.
- The family summary is review-facing synthesis only. It can help humans and future sessions see what the tracked benchmark suite covers, but it does not change `kg-bench` execution, validation scoring, canonical IR, or `CorpusMemory`.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g KG fixture-result corpus KB projection)
- Continued from the README/PNT flow after commit `1229426`, with `30` Rust source files and `56,601` lines under `crates/specforge/src` after the CLI/benchmark seam update.
- `specforge corpus-kb` now accepts optional `--kg-fixtures-root <fixture-root>` plus repeated `--kg-fixture <fixture>` selectors, so R15g can refresh a tracked benchmark-result page without requiring validation report inputs.
- The new `corpus_kb/benchmarks/kg-fixtures.md` managed page records the current tracked KG fixture run: `55` total fixtures, `55` passed, `0` failed, with fixture paths preserved as provenance and human synthesis kept outside the managed block.
- `kg_bench` now exposes an internal fixture-outcome collection seam for this projection while preserving the standalone `specforge kg-bench` command behavior.
- The former noisy-output caveat on the KG fixture-result projection is now closed by the quiet fixture-local validation path; future cleanup can still improve fixture-family summarization, but projection output no longer dumps full validation reports.
- Focused `corpus_kb` and `kg_bench` tests passed, the KG fixture-result refresh command passed with `55/55` fixtures, and the full local CI wrapper passed with `311` Rust tests.

## Session update (2026-04-13 R15g quiet KG fixture validation path)
- Continued from commit `84500f5`, with `30` Rust source files and `56,671` lines under `crates/specforge/src` after the validator-output guard update.
- `validate::run_quiet()` now wraps the existing user-facing validator in a thread-local output guard, so internal callers can persist validation sidecars/backannotations without printing the full validation report.
- `kg-bench` now uses that quiet path for fixture-local validation expectations. This keeps `specforge kg-bench` and `specforge corpus-kb --kg-fixtures-root ...` focused on fixture outcomes while preserving the same validation semantics and artifacts.
- The output guard restores its previous state through `Drop`; focused validation passed for the guard, fixture-failure output, corpus-KB benchmark refresh, the live `55/55` corpus-KB refresh command, the full `kg_bench` test target, and the full local CI wrapper with `312` Rust tests.

## Session update (2026-04-13 R15g corpus knowledge-base bootstrap)
- Executed the README handoff through `SESSION_BOOTSTRAP.md`, reviewed the referenced live docs, and rechecked the active Rust surface: `30` Rust source files and `56,332` lines under `crates/specforge/src` after adding the new command module.
- `specforge corpus-kb <validation-report>...` is now wired into the CLI to refresh tracked corpus knowledge-base pages under `corpus_kb/` from reviewable validation report sidecars.
- The first `R15g` page family is `corpus_kb/failures/validation-findings.md`; its managed block is auto-refreshed from the four live AMBA validation reports while the human synthesis section remains outside the managed block.
- The corpus KB boundary is explicit in code and docs: it can inform humans, future LLM sessions, benchmark ideas, rescan design, and typed-prior candidates, but it cannot directly mutate canonical IR or become a prior without a separate validation-gated path.
- Focused `corpus_kb` tests passed, the refresh command passed on the four live validation report sidecars, and the full local CI wrapper passed with `307` Rust tests.

## Session update (2026-04-12 VLM timing signal bit-select annotation filtering)
- Recovered the interrupted README-bootstrap/PNT state after the crash, re-read the referenced continuity docs, and rechecked the active Rust surface: `29` Rust source files and `56,040` lines under `crates/specforge/src` before documentation edits.
- `SemanticIR` now extends the VLM timing spurious-annotation filter to standalone signal bit-select/range labels such as `XREQ[0]`, `XREQ<1>`, and `XREQ[3:0]`.
- The change is deliberately scoped to VLM timing `annotations[]` label noise. Grounded `signals[].values[]` observations for the same document signal still become typed `SignalConstraintRecord`s and temporal `SignalValue` predicates when the sampled value is concrete.
- The direct semantic regression and `vlm_timing_spurious_annotation_negative` KG fixture now cover this edge case while keeping the tracked fixture count at `55`; focused validation, targeted `kg-bench`, and the full local CI wrapper passed with `305` Rust tests.

## Session update (2026-04-12 README bootstrap refresh)
- Executed the README handoff through `SESSION_BOOTSTRAP.md`, re-read the referenced live docs, and compared the current Rust implementation surface against the documentation map.
- The active Rust surface now contains `29` Rust source files and `56,024` lines under `crates/specforge/src`, including the full command set wired through `cli.rs`, `commands/mod.rs`, and `lib.rs`: `inspect`, `doctor`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, `rescan-plan`, `kg-bench`, `learn-priors`, and `nlp-enrich`.
- No Rust architecture drift requiring code changes was found, but the README implementation-path map and this analysis file's current testing subsection lagged the codebase; this refresh aligns them with the `76274f6` baseline, `55` tracked KG-quality fixtures, and the latest full local CI result of `305` passing Rust tests.

## Session update (2026-04-12 NLP alias markdown/list marker filtering)
- `extract_alias_phrase()` now rejects broader Form 2 alias subjects that begin with markdown bullets, block quotes, and ordered-list markers such as `*`, `+`, `>`, `1.`, and `2)` in addition to the existing `-`, `|`, and `#` guards.
- This keeps list/table formatting out of `signal_alias_map` while preserving ordinary prose alias learning for noun phrases like `address bus`.
- Focused validation, warning-deny Clippy, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 VLM timing bracketed sample label filtering)
- `SemanticIR` now filters bracketed VLM timing annotation labels such as `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` as low-value waveform/sample markup instead of allowing them to become `TimingConstraintRecord`s.
- The existing `vlm_timing_spurious_annotation_negative` fixture and direct semantic regression now cover that bracketed label family while preserving real signal-value observations as typed temporal evidence.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 VLM timing compact sample label filtering)
- `SemanticIR` now filters compact VLM timing annotation labels such as `D0`, `A1`, `DATA0`, and `0xAA` as low-value waveform/sample markup instead of allowing them to become `TimingConstraintRecord`s.
- The existing `vlm_timing_spurious_annotation_negative` fixture and direct semantic regression now cover that compact label family while preserving real signal-value observations as typed temporal evidence.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 missing initial FSM state cardinality warning)
- Added `vlm_state_machine_missing_initial_negative`, proving VLM-authored `IDLE` / `BUSY` state evidence with no initial marker keeps the state graph and transition visible while validation reports unsafe initial-state cardinality at both semantic and intent stages.
- Added a direct explicit-state validation regression for the same zero-initial path and shared the staged IR build helper with the multiple-initial regression.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 initial FSM state cardinality warning)
- `specforge validate` now emits `semantic_state_machine_initial_cardinality` and `intent_state_machine_initial_cardinality` warnings when a canonical state graph exists but has zero or multiple initial states.
- Added `vlm_state_machine_multiple_initial_negative`, proving VLM-authored `IDLE` and `BUSY` multiple-initial evidence keeps the graph visible while validation flags the unsafe cardinality at both semantic and intent stages.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with 304 Rust tests for this slice.

## Session update (2026-04-12 initial FSM state validation metric)
- `specforge validate` now reports `initial_regular_states` for both `SemanticIR` and `IntentIR`, making FSM initial-state cardinality visible beside total state and transition counts.
- The tracked `vlm_state_machine_duplicate_initial_gold` fixture now asserts `initial_regular_states = 1` at both semantic and intent validation stages, so the duplicate-label merge path proves exactly one canonical initial state survives.
- Focused semantic validation, targeted `kg-bench` validation, and the full local CI gate passed with 303 Rust tests for this slice.

## Session update (2026-04-12 VLM state-machine duplicate initial markers)
- `SemanticIR` now merges duplicate VLM state-machine state labels by state name before adding canonical `RegularStateRecord`s, preserving `is_initial` if any duplicate carries it.
- `kg-bench` can now assert canonical initial-state names directly, and the tracked `vlm_state_machine_duplicate_initial_gold` fixture proves `IDLE` remains the single initial state when a duplicate later marks it as initial while `BUSY` stays non-initial.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_state_machine_duplicate_initial_gold`, and the full local CI gate passed with 303 Rust tests.

## Session update (2026-04-11 VLM state-machine undeclared-transition filtering)
- `SemanticIR` now requires VLM state-machine transition `from` / `to` endpoints to reference state names accepted from the same `vlm_state_machine_extraction` observation before adding `StateTransitionRecord` entries.
- Added `vlm_state_machine_observation_rejects_undeclared_transition_endpoints` plus the tracked `vlm_state_machine_undeclared_transition_negative` fixture, proving `IDLE` / `BUSY` and `IDLE->BUSY` survive while undeclared identifier-shaped endpoints such as `DONE` and `RESET` are filtered.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_state_machine_undeclared_transition_negative`, and the full local CI gate passed with 302 Rust tests.

## Session update (2026-04-11 VLM state-machine label-noise filtering)
- `SemanticIR` now parses VLM state-machine `states[].name` labels and transition `from` / `to` endpoints through the canonical identifier parser before adding `RegularStateRecord` or `StateTransitionRecord` entries, so prose labels such as `IDLE state` and `ACCESS phase` stay out of canonical FSM state.
- `kg-bench` can now assert canonical state names and state-transition endpoint keys directly, and the tracked `vlm_state_machine_label_noise_negative` fixture locks that clean `IDLE` / `BUSY` evidence survives while noisy prose labels are filtered.
- Focused validation passed for the semantic regression and targeted `kg-bench` validation passed for `vlm_state_machine_label_noise_negative`; the full local CI gate passed with 301 Rust tests for this slice.

## Session update (2026-04-11 VLM timing compact edge spelling variants)
- `SemanticIR` now rejects abbreviated and compact VLM timing waveform-motion spellings such as `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` before the generic symbolic-value fallback can promote them into false signal values.
- Strengthened `vlm_timing_diagram_observation_rejects_waveform_motion_states` and `vlm_timing_waveform_motion_negative` so the mixed waveform still produces exactly one VLM-authored signal constraint and temporal rule: the concrete `HIGH` sample.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_timing_waveform_motion_negative`, and the full local CI gate passed with 300 Rust tests for this slice.

## Session update (2026-04-11 VLM timing waveform motion spelling variants)
- `SemanticIR` now normalizes VLM timing motion states across spaces, underscores, and hyphens before applying the waveform-motion filter, so identifier-shaped values such as `RISING_EDGE`, `LOW_TO_HIGH`, and `HIGH_TO_LOW` do not become false symbolic signal values.
- Strengthened `vlm_timing_diagram_observation_rejects_waveform_motion_states` and `vlm_timing_waveform_motion_negative` to cover those separator variants while keeping the concrete `HIGH` sample as the only surviving signal constraint and temporal rule.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_timing_waveform_motion_negative`, and the full local CI gate passed with 300 Rust tests.

## Session update (2026-04-11 VLM timing waveform motion filtering)
- `SemanticIR` VLM timing signal-value lifting now filters waveform motion descriptors such as `rising`, `falling`, `stable`, `steady`, `unchanged`, and `toggle` before the generic symbolic-value fallback can promote them into false `MustBeValue` facts.
- Concrete VLM timing values still flow through the bounded path: `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` remain eligible for typed signal constraints and temporal predicates when the signal name is document-grounded.
- Added `vlm_timing_diagram_observation_rejects_waveform_motion_states` plus tracked KG fixture `vlm_timing_waveform_motion_negative`, proving a mixed `rising` / `HIGH` / `stable` / `falling` / `UNCHANGED` waveform creates exactly one concrete signal constraint and temporal rule.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_timing_waveform_motion_negative`, and the full local CI gate passed with 300 Rust tests.

## Session update (2026-04-11 SemanticIR sticky interface conflicts)
- `InterfaceSignalAccumulator` now tracks whether direction and width hints already conflicted, so `None` no longer ambiguously means both unknown and conflict-collapsed inside the canonical interface builder.
- Strengthened `surfaces_interface_signal_conflicts_for_conflicting_explicit_declarations` with an input/output/input width 8/16/8 declaration sequence; `DATA.direction_hint` and `DATA.width_hint` remain unresolved instead of self-healing to the repeated original value.
- Focused validation passed for the strengthened regression, the full `ir::semantic::tests` suite passed with 65 semantic tests, and the full local CI gate passed with 299 Rust tests.

## Session update (2026-04-11 actor-port width conflict stickiness)
- Added `standalone_dt_keeps_conflicting_actor_port_width_unresolved`, proving sticky adapter inventory conflicts cover numeric width as well as direction roles.
- The regression starts with canonical flat `DATA_OUT` width `8`, overlays graph-backed `DATA_OUT` width `16`, then repeats graph-backed width `8`; lowering must remain blocked with `DATA_OUT.width_hint == None` instead of resurrecting the original width.
- Focused validation passed for the new regression, the full `ir::adapters::tests` suite passed with 26 adapter tests, and the full local CI gate passed with 299 Rust tests.

## Session update (2026-04-11 actor-port conflict stickiness)
- `FsmSignalCandidate` inventory evidence now tracks whether a direction or width hint already conflicted, so `None` no longer ambiguously means both unknown and conflict-collapsed.
- This prevents later repeated graph/interface evidence from resurrecting a signal role after disagreement; once a direct actor-port overlay sees incompatible `DATA_OUT` directions, the candidate stays unresolved and lowering blocks.
- Added `standalone_dt_keeps_conflicting_actor_port_direction_unresolved`, proving repeated same-actor `DATA_OUT` evidence in the order `Output -> Input -> Output` remains blocked instead of self-healing to `Output`.
- Focused validation passed for the new regression, the full `ir::adapters::tests` suite passed with 25 adapter tests, and the full local CI gate passed with 298 Rust tests.

## Session update (2026-04-11 graph-backed sequential system directions)
- Added an adapter regression proving standalone sequential DT lowering remains renderable after clearing flat direct-interface direction hints when a single `controller` actor-port graph supplies directions for `clk`, `rst_n`, `DATA_IN`, and `ACC`.
- This locks the system-contract renderability path, not just ordinary data-port rendering: `clk` and `rst_n` are recovered as graph-backed inputs, `(+system ...)` still emits, and `fsm_adapter_system_contract` is not produced.
- Focused validation passed for `standalone_sequential_dt_recovers_system_directions_from_actor_ports`, the full `ir::adapters::tests` suite passed with 24 adapter tests, and the full local CI gate passed with 297 Rust tests.

## Session update (2026-04-11 mixed clause-local control polarity)
- `EvidenceIR` now has a bounded fallback that can split mixed active-level prose such as `CS_N is active LOW and ENABLE is active HIGH` into clause-local polarity observations.
- The fallback is intentionally strict: every mentioned known signal must be recovered, each recovered polarity clause must name exactly one known signal, and detached polarity wording such as `CS_N is active LOW and active HIGH` stays unresolved instead of borrowing an implicit subject.
- The whole-statement detector still returns no polarity for text containing both active-low and active-high cues; clause-local recovery is a separate guarded path rather than a broad global guess.
- `mixed_control_polarity_gold` locks the path through the KG benchmark surface with two declared canonical signal records, zero heuristic duplicates, two resolved polarities, and zero polarity/temporal conflicts.
- Focused validation passed for the mixed-clause positive case, the detached-polarity negative case, the detector guard, targeted `kg-bench mixed_control_polarity_gold`, the full 48-fixture KG benchmark suite, and the full local CI gate with 296 Rust tests.

## Session update (2026-04-11 detached mixed polarity negative fixture)
- `detached_mixed_control_polarity_negative` now locks the conservative mirror case for `CS_N is active LOW and active HIGH` through the tracked KG benchmark surface.
- The fixture proves the signal remains declared and constrained, but `with_resolved_polarity` stays `0` through `SemanticIR` and `IntentIR`, with zero heuristic duplicates, zero polarity conflicts, and zero temporal conflicts.
- Targeted `kg-bench detached_mixed_control_polarity_negative` passed, the full `kg_bench` test passed with 49 tracked fixtures, and the full local CI gate passed with 296 Rust tests.

## Session update (2026-04-11 collective control polarity)
- `EvidenceIR` now recovers unambiguous collective active-level prose such as `CS_N and WE_N are active LOW signals`, producing polarity observations for both declared controls and refining asserted/deasserted constraints after polarity is grounded.
- Mixed low/high compound prose remained unresolved in that slice until clause-local parsing could safely bind each signal to exactly one polarity phrase.
- `SemanticIR` now treats polarity-only co-mentions of already declared signals as enrichment of the authoritative records rather than low-confidence heuristic interface groups, preventing duplicate canonical signal records and inflated polarity metrics.
- `multi_control_polarity_gold` locks the path through the KG benchmark surface with two resolved polarities and zero polarity/temporal conflicts.
- Focused validation passed for the new evidence regressions, semantic duplicate-suppression regression, targeted `kg-bench multi_control_polarity_gold`, the full 47-fixture KG benchmark suite, and the full local CI gate with 295 Rust tests.

## Session update (2026-04-11 asserted-when-level control polarity)
- `EvidenceIR` polarity detection now recognizes explicit local phrases such as `asserted when LOW`, `LOW when asserted`, `asserted by driving LOW`, and their active-high mirrors
- the new path is intentionally evidence-grounded: it does not infer active-low from `_N` suffixes alone
- focused regressions prove the detector accepts asserted-when-level wording and that a non-reset control signal `CS_N` refines asserted/deasserted constraints to LOW/HIGH only after local prose says it is asserted when LOW
- `SemanticIR` now suppresses redundant one-signal heuristic interface candidates when that signal is already explicitly declared, preventing local polarity prose from double-counting the same canonical signal
- focused `cargo test --manifest-path Cargo.toml polarity -- --nocapture` passed with 21 polarity tests, `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` passed with all 46 KG fixtures, and `bash scripts/run_ci.sh` passed with 292 Rust tests plus docs for that slice

## Session update (2026-04-11 KG-bench graph direction expectations)
- `specforge kg-bench` canonical stage expectations now include `graph_direction_signal_names_include` and `graph_direction_signal_names_exclude`
- those expectations inspect canonical `actor_ports` for non-`unknown` direction evidence by signal name, which lets fixtures test graph-backed direction coverage directly without overloading flat `InterfaceSignalRecord.direction_hint`
- the existing `signal_directions_include` expectation remains compatibility-specific; this matters because actor-relative producer/consumer roles should not be flattened into a fake global direction when a fixture really means flat interface perspective
- `actor_ports_gold` now exercises the new surface for both `SemanticIR` and `IntentIR`
- the full local CI path passed with 288 Rust tests, warning-deny Clippy, warning-deny rustdoc, and mdBook build

## Session update (2026-04-11 bootstrap refresh)
- executed the README terminal instruction by reading `SESSION_BOOTSTRAP.md`, which expands the task into reading the referenced live docs, analyzing the Rust codebase, updating this analysis if necessary, and continuing from the roadmap
- reviewed the high-signal live docs (`README.md`, `SESSION_BOOTSTRAP.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `VALIDATION_SNAPSHOT.md`, `INTENTIR_SPEC.md`, `MEMORY.md`, `COMMIT.md`) and compared their claims against the current CLI/module surface
- observed the current Rust implementation under `crates/specforge/src`: 28 Rust files and about 53,858 lines across the active single-crate implementation
- corrected stale analysis claims around the CLI command list, `rescan-plan`, schema-v2 rescan execution, promotion-review boundaries, `CorpusMemory` schema v5 prior families, KG fixture count, and current test count
- re-ran the README bootstrap sanity pass after `35a8372` and found no new Rust architecture drift beyond continuity/doc hygiene: `MEMORY.md` needed the latest committed baseline and `USER_GUIDE.md` had stray example bullets under its root-doc list
- the codebase remains a real end-to-end staged IR pipeline with a growing learning/rescan plane, but it is not yet a universal chip-spec-PDF oracle; the honest next pressure remains graph-first semantics, temporal/clock-reset truthfulness, multimodal arbitration, KG-quality expansion, and corpus-level learning without cross-document fact leakage

## Session update (2026-04-11 dead-code warning cleanup)
- removed the last five dead-code warning sources instead of suppressing them
- the deleted adapter helpers were stale legacy paths for `DecisionTreeFragmentRecord` block/action validation and direct `DecisionTreeActionRecord` rendering; the active `.fsm` lowering path now validates and renders `ControlBlockRecord` / `ControlActionRecord` instead
- the deleted semantic helpers were empty register/timing builder stubs with no call sites; `SemanticIR` already carries register records from `EvidenceIR` and extends timing records with VLM timing extraction directly in `SemanticIr::build()`
- targeted validation with `cargo test --manifest-path Cargo.toml --lib` now reports 282 passing tests and no dead-code warning output

## Session update (2026-04-11 warning-deny CI)
- `scripts/run_ci.sh` now runs `cargo test --manifest-path Cargo.toml` with `RUSTFLAGS="-D warnings"`
- the warning-deny gate lives in the shared script, so local pre-push validation and manually launched GitHub Actions runs enforce the same baseline
- this turns the cleaned warning baseline into a regression guard rather than a one-time cleanup

## Session update (2026-04-11 Clippy gate)
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` is now clean
- `scripts/run_ci.sh` now runs Clippy before the Rust test suite, and GitHub Actions installs the `clippy` component before calling the shared script
- mechanical Clippy findings were fixed directly; intentional broad IR plumbing remains localized behind `#[expect(...)]` attributes with reasons rather than global allows

## Session update (2026-04-11 rustdoc gate)
- `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` is now clean
- fixed a broken intra-doc link interpretation in the `ActorSignalRelation` docs by changing bracket-shaped `output_of[...]` / `input_of[...]` wording into code-formatted prose
- `scripts/run_ci.sh` now runs rustdoc before the mdBook build and composes caller-provided `RUSTDOCFLAGS` with `-D warnings`

## Session update (2026-04-11 active-low VLM deassertion fixture)
- added `vlm_timing_active_low_deassertion_equivalence_gold` to lock the reset-release mirror of the existing active-low VLM timing polarity fixture
- the fixture proves active-low `ARESETN` observed as both `deasserted` and `HIGH` creates two typed temporal rules and zero temporal/polarity conflicts
- targeted `kg-bench` validation for the new fixture passed

## Session update (2026-04-11 graph-backed `.fsm` module directions)
- moved the first `.fsm` adapter consumer onto target-actor-relative graph evidence: explicit module candidates now overlay matching `IntentIR.actor_ports` before renderability analysis
- this lets an explicit top composition lower when a child module's flat module-local `direction_hint` values lag but the actor-relative graph already resolves the module actor's input/output port roles
- the slice stays conservative: `Input` / `Output` graph directions can fill `.fsm` module port roles, but `InOut` / `Unknown` do not become fake directions, and conflicts with flat hints still collapse to unresolved state
- added regressions that clear flat child-module directions to prove graph-backed `producer_core` / `consumer_core` actor ports recover the needed child port directions, and that inject a conflicting graph direction to prove lowering stays blocked instead of silently overriding local evidence

## Session update (2026-04-11 graph-backed `.fsm` direct-root directions)
- moved the next `.fsm` adapter consumer onto bounded actor-relative graph evidence: standalone direct roots can now recover missing local signal directions from `IntentIR.actor_ports` when the actor-port graph relevant to the direct local signal inventory has exactly one renderable actor context
- the direct-root path is intentionally stricter than explicit module lowering because no module name exists to identify the target actor; mixed producer/consumer graph contexts stay blocked rather than guessing which perspective should define direct `.fsm` input/output roles
- the overlay only strengthens signals already present in the direct signal inventory, unlike explicit-module lowering where graph-only module ports can be useful child-module interface evidence; unrelated graph-only actor ports are ignored by the direct-root actor-context gate
- actor-port provenance merging now reconciles direction once per actor-port record, preventing a conflicting graph direction with multiple supporting ids from accidentally reintroducing a resolved direction after conflict collapse
- focused tests now prove unambiguous standalone recovery, unrelated graph-only actor-port ignoring, ambiguous mixed-actor blocking, and duplicate-provenance conflict blocking for top-composition graph evidence
- the full local CI path passed with 287 Rust tests, warning-deny Clippy, warning-deny rustdoc, and mdBook build

## Session update (2026-04-11 top-link-backed `.fsm` boundary directions)
- explicit top ports now preserve width-only declarations by making `ExplicitTopPortRecord.direction_hint` optional
- `.fsm` top-composition renderability can recover a missing top boundary direction from explicit link topology: top link source means top input, and top link target means top output
- the recovery is not actor-graph inference and does not invent boundary roles; unresolved top directions and conflicts between explicit direction and link topology still block
- focused tests prove width-only `result_data` survives into canonical top composition and renders as `result_data>8` only through the explicit `consumer.result_data -> result_data` link
- the full local CI path passed with 288 Rust tests, warning-deny Clippy, warning-deny rustdoc, and mdBook build

## Session update (2026-04-04)
- `specforge converge` now drives the persisted `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters` path as a fixed-point loop and stops when the materialized knowledge snapshot is stable
- `EvidenceIr::build()` now preserves compatible alias-learning state, NLP-upgraded statement classes, and structured NLP records across rebuilds so pass `N+1` does not forget pass `N`
- `EvidenceIr::build()` now uses `converge_evidence_extractions()` instead of a one-shot extraction tail, allowing new enum facts and polarity facts to feed later passes in the same build
- signal-anchored encoding rescans recover weakly labeled encoding tables without introducing a new hardcoded APB/AHB/AXI value list
- `SemanticIR` now tolerates raw JSON, fenced JSON, and prose-wrapped JSON in VLM timing/state observations, restoring timing/state lift from real Ollama outputs
- `SemanticIR` / `IntentIR` now preserve the structural KG downstream via `actor_signal_relations`, `actor_ports`, and `signal_connectivity`, so actor-aware evidence is no longer trapped in `EvidenceIR`
- `specforge validate` now scores semantic and intent direction coverage from the actor-relative graph first, exposing flat `direction_hint` lag separately instead of treating compatibility-hint absence as semantic failure
- `SemanticIR` / `IntentIR` now also carry a first typed `temporal_rules` layer derived from constraints and timing observations, and validation now reports missing clock/edge grounding for that surface
- the temporal-rule layer now also recovers bounded `cycle_window` latency from structured prose/timing text, including idiomatic one-cycle phrases like `next cycle`, `next tick`, `next clock edge`, `next rising edge`, `next falling edge`, `next posedge`, and `next negedge`, plus quantified tick/edge forms like `within 2 ticks`, `within 2 clock edges`, `within 2 HCLK edges`, `within 2 edges of HCLK`, and `after 3 falling edges`, unit-first diagram labels like `tick T3` / `posedge T4`, named diagram-style generic-edge labels like `HCLK edge T3` / `edge T3 of HCLK`, unit-first local-clock forms like `tick T3 of HCLK` / `posedge T4 of HCLK`, trailing-`of <clock>` shorthand-edge forms like `the third posedge of HCLK` / `within 2 negedges of HCLK`, and explicit later/ordinal forms like `two cycles later`, `on the third rising edge`, and `on the third edge of HCLK`; explicit shorthand edge phrases now also preserve the named edge itself for signal/conditional/timing temporal rules instead of silently falling back to the default clock edge, explicit local timing text like `rising edge of HCLK`, `HCLK rising edge`, `HCLK posedge`, `next HCLK edge`, `the third edge of HCLK`, `within 2 edges of HCLK`, `edge T3 of HCLK`, `tick T3 of HCLK`, `posedge T4 of HCLK`, `third posedge of HCLK`, or `2 negedges of HCLK` can now override the document default `clock_signal`, named local cycle/tick phrasing like `same ACLK cycle` now inherits the same bounded rising-edge temporal anchor already used for default-clock timing, and known-signal-aware named edge phrasing like `next HCLK edge`, `within 2 HCLK edges`, `within 2 edges of HCLK`, `edge T3 of HCLK`, `third posedge of HCLK`, or `on the third edge of HCLK` now recovers the same bounded windows as the more explicit edge families instead of getting stranded behind the inserted clock token
- the temporal-rule layer now also compresses grounded ready/valid completion guards into typed `HandshakeComplete` predicates, so protocol-native transfer events are no longer represented only as separate scalar value clauses
- `EvidenceIR` now also persists typed `signal_semantic_hints` mined from signal-description text, direct prose descriptions, and alias-grounded prose descriptions, and `SemanticIR` / `IntentIR` now carry those roles as per-signal `semantic_tags`, so handshake-role inference can use meaning-grounded descriptions before falling back to literal `VALID` / `READY` spellings
- that semantic-hint synthesis is now stricter too: prose/visual hint extraction strips explicit signal identifiers before tag inference, so declarations and captions no longer create semantic role consensus from the signal name token alone
- semantic-hint synthesis is now also less lossy for multi-signal regions: prose statements and visual captions can be decomposed into clause-local per-signal context windows, so one region can ground valid-like meaning for one signal and ready-like meaning for another without forcing whole-text single-target resolution
- that same targeting path is now also stricter about aliases: if a sentence/caption already names the signal explicitly, alias-grounding is suppressed for that same signal so one source region cannot inflate semantic support just because both the alias and the canonical signal name appear
- the canonical role surface is now more inspectable too: semantic candidates and consensus summaries carry an explicit `alias_dependent` flag, and validation reports alias-dependent resolved roles instead of forcing readers to infer that dependency from raw source-kind lists
- that honesty surface now reaches the temporal layer too: validation reports when typed `HandshakeComplete` predicates depend on alias-dependent semantic consensus, so temporal progress semantics no longer look equally grounded when they still rely only on alias mapping
- that weaker temporal grounding is now canonical too: `SemanticIR` emits an explicit residual packet for alias-dependent handshake completion, and `IntentIR` mirrors it as an assumption so the warning survives even without running the validator
- contradictory polarity now survives downstream as well: `signal_polarity_conflicts` are no longer trapped in `EvidenceIR`, and validation now reports them for `SemanticIR` and `IntentIR` too
- `EvidenceIR` now also mines `signal_semantic_hints` from grounded visual captions and VLM timing-diagram annotations, using the same robust fenced/prose-wrapped JSON recovery path as the semantic VLM lift and preserving explicit visual-evidence provenance on those hints
- `SemanticIR` / `IntentIR` no longer collapse that richer role evidence entirely into `semantic_tags`; they now also carry per-signal `semantic_observations`, preserving source kind and provenance in the canonical layers
- `SemanticIR` / `IntentIR` now also carry per-signal `semantic_candidates`, keeping competing role hypotheses visible in the canonical layers instead of flattening them into only winner-or-none state
- `SemanticIR` / `IntentIR` now also carry per-signal `semantic_arbitration`, preserving the current lead role, runner-up, evidence margin, and decisive-vs-contested state so arbitration remains inspectable without forcing unsafe winner selection
- the temporal handshake layer now respects that arbitration state too: contested semantic-role evidence blocks literal `VALID` / `READY` name fallback instead of being silently overridden by it
- that blocked heuristic path is now surfaced explicitly as semantic residual state and validation metrics/findings, so withheld handshake promotion is inspectable rather than only implicit
- `SemanticIR` / `IntentIR` now also resolve canonical per-signal semantic roles plus modality-aware `semantic_grounding_strength` from those observations, so downstream consumers can prefer provenance-backed role consensus and validation can distinguish single-source grounding, same-modality repetition, and cross-modality reinforcement
- fallback-only resolved semantic roles are no longer only validator-visible drift: `SemanticIR` now emits a dedicated residual packet when a role still lacks observation-backed consensus, and `IntentIR` carries the same provisional state as an explicit assumption
- handshake-role consumers are now stricter as well: typed handshake recovery only trusts observation-backed semantic consensus, and handshake-shaped signals with fallback-only provisional roles now block raw name fallback instead of quietly regaining protocol meaning through spelling alone
- `SemanticIR` / `IntentIR` now also carry explicit `semantic_consensus` summaries for observation-backed role meaning, including supporting source kinds, supporting observation count, and strongest supporting automation confidence, and validation now flags any resolved role still lacking that richer profile
- `specforge nlp-enrich` now refreshes `signal_semantic_hints` before writing updated `EvidenceIR`, so alias learning can immediately feed downstream semantic-role inference instead of waiting for a later rebuild path
- `EvidenceIR` now also persists typed `signal_semantic_conflicts` when the same signal accumulates incompatible valid-like and ready-like role evidence, and `specforge validate` reports that disagreement explicitly instead of hiding it inside a dual-tag ambiguity
- `SemanticIR` / `IntentIR` now also carry those `signal_semantic_conflicts`, and `specforge validate` now reports them there too, so semantic-role disagreement does not disappear once the pipeline leaves the evidence stage
- the temporal-rule layer now reuses unique KG producers to emit actor-grounded drive predicates for value-oriented rules, reconnecting temporal semantics back to the structural graph
- the temporal-rule layer now also emits actor-grounded stability predicates for stable/hold rules when the KG resolves a unique producer, so producer obligations are no longer flattened into signal-only invariants
- the temporal-rule layer now preserves compound `and` guards as multiple typed antecedents when each clause anchors to a known signal, so conjunctive protocol preconditions survive into `IntentIR`
- the temporal layer now also materializes typed `temporal_conflicts` records when contradictory value obligations target the same signal/phase under the same grounded context, which keeps disagreement explicit instead of silently flattening it away
- the convergent evidence loop now also extracts active-high/active-low signal polarity from `SignalDescription` tables and merges that with prose polarity conservatively before refining asserted/deasserted constraints
- `EvidenceIR` now also persists `signal_polarity_conflicts` when prose/table polarity disagree, and `specforge validate` reports those conflicts explicitly instead of leaving the disagreement visible only as a neutralized constraint kind
- `SemanticIR` / `IntentIR` now also persist `interface_signal_conflicts` when conflicting declarations disagree on signal direction or width, and `specforge validate` reports that interface-shape disagreement explicitly instead of only degrading the canonical hint to `None`
- `SemanticIR` / `IntentIR` now also persist `signal_connectivity_conflicts` when the structural KG resolves multiple producers for the same signal, and `specforge validate` reports that producer ambiguity explicitly instead of leaving it implicit in connectivity vectors
- `specforge validate` now writes deterministic stage-local `validation_report.json` sidecars and backannotates the current report into `validation_reports` on `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
- `specforge project-validation <artifact>...` now refreshes `VALIDATION_SNAPSHOT.md` and the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md` from persisted IR validation reports
- `specforge learn-priors <intent_ir>...` now builds the first local typed `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`, harvesting only from validated `IntentIR` artifacts and keeping the learning plane advisory-only
- the first `R15f` slice currently learns:
  - actor-taxonomy priors from decisive actor-grounded handshake-role evidence plus conservative self-identifying actor vocabulary
  - semantic-role phrase priors from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text
  - semantic modality-reliability priors from decisive, non-alias-dependent canonical semantic consensus plus supporting source kinds
  - temporal-language phrase priors from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`
  - table-shape priors from validated structured-table header signatures chained through validated artifacts
- the latest live four-document AMBA prior-memory run across AXI/APB/AHB/AXI-Stream `IntentIR` artifacts currently yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors
- the learning plane now also rejects bogus payload/event actor terms like `control information` during both prior harvesting and prior lookup, so stale non-actor vocabulary cannot keep biasing future extraction after the underlying document-local bug is repaired
- that matters because the first unseen protocol run has now moved the semantic side of the learning plane from “architecturally landed but empty on real artifacts” to “materially populated by validated real-document consensus”
- the first bounded prior-consumption path is now landed too: `EvidenceIR` can consult the local `CorpusMemory` during `evidence` / `converge` and use actor-taxonomy priors to interpret explicit local actor labels in section headings and `Source` / `Destination` columns, and width-only section-guided signal tables can now recover structural `ActorSignalRelation::Drives` edges instead of only flat compatibility directions
- that matters because it replaces another brittle hardcoded-vocabulary heuristic with a typed reusable prior while still keeping canonical truth local; the prior can widen the meaning of a seen actor term like `Producer`, but it cannot invent a new actor, signal, or relation
- the second bounded prior-consumption path is now landed too: `EvidenceIR` can use semantic phrase priors to recover local signal-role hints from non-hardcoded grounded phrases, and that guidance now survives later `refresh_signal_semantic_hints()` calls because the consulted `prior_memory_path` is persisted into `EvidenceIR`
- the third bounded prior-consumption path is now landed too: `SemanticIR` can use temporal phrase priors to advisory-recover cycle windows from local timing text when the built-in parser cannot recover that timing window directly
- the fourth bounded prior-consumption path is now landed too: `SemanticIR` can use semantic modality-reliability priors to advisory-adjust local semantic arbitration when the current PDF already contains multiple competing locally grounded role candidates, while keeping the underlying semantic conflict explicit
- the benchmark surface now locks all four first bounded prior families:
  - actor-taxonomy priors: width-only `Issuer signals` / `Acceptor signals` sections stay directionless and graph-empty without prior memory and gain structural KG edges, actor ports, and canonical directions only when the matching actor-taxonomy priors are staged into the fixture
  - semantic phrase priors: `XACK can receive the transfer` stays semantically unresolved without prior memory and gains ready-like recovery only with a matching semantic prior
  - temporal phrase priors: `PREADY must be asserted one beat later` stays temporally unbounded without prior memory and gains a one-cycle `cycle_window` only with a matching temporal prior
  - semantic modality-reliability priors: locally conflicted role evidence like `XCTRL` stays contested without prior memory and becomes decisively resolved only when the matching modality-reliability prior is staged into the fixture
- the same semantic prior family is now benchmarked across another modality too: the unseen local visual-caption phrase `XACK can sink the transfer` stays unresolved without prior memory and gains ready-like recovery only when a matching `visual_caption` semantic prior is staged into the fixture
- the fourth bounded prior-consumption path is now landed too: `EvidenceIR` can advisory-recover a local table kind from a learned table-shape prior, but only when the current table is still `unknown`; explicit local `SourceIR` table kinds still win outright
- the benchmark surface now locks the first table-shape before/after truthfulness pairs too:
  - a locally `unknown` `Name | Direction | Width` table stays inert without prior memory and gains signal-description recovery only when a matching learned table-shape prior is staged into the fixture
  - a locally `unknown` `Parameter | Min | Max | Unit` table stays inert without prior memory and gains timing-parameter recovery only when a matching learned table-shape prior is staged into the fixture
- that matters because the learning plane can now strengthen local temporal understanding on unseen phrase shapes without letting prior memory create timing rules that are not already grounded in the current PDF
- the KG benchmark harness can now also stage a fixture-owned `CorpusMemory`, and the first tracked gold/negative pair proves that unseen local timing language only gains a `cycle_window` when a matching learned temporal prior is present
- the KG benchmark harness now also locks the same before/after truthfulness pattern for semantic priors: unseen local role language only gains canonical semantic recovery when a matching learned semantic prior is present
- the latest unseen protocol stress run is now AXI-Stream: it converges in `2` full pipeline iterations, validates at `90/100 EXCELLENT`, and the latest truthfulness fixes now recover both parity-check ownership and parity-check widths from the local `Check Signal / Signals Covered` semantics, bringing declared graph-direction and width coverage to `22/22`; `ACLK` / `ARESETN` now classify as infrastructure connectivity with canonical sourcing left in the system-contract surface, same-cycle timing language now lands as six explicit `0`-cycle temporal windows, assertion-vs-level temporal comparison is now polarity-aware, and the last carried interface-grouping residual is gone after heuristic grouping stopped treating width/table metadata as interface signals and explicit interfaces began subsuming smaller statement fragments
- resolved signal polarity now also survives directly on canonical `InterfaceSignalRecord`s and is visible in validator metrics as `with_resolved_polarity`; after the latest AHB infrastructure-interface fix, AXI/APB/AHB/AXI-Stream now all report `1`, and non-reset control coverage now includes asserted-when-level prose, unambiguous collective active-level prose, and safe mixed clause-local prose rather than only reset carry-through plumbing
- `specforge converge` now excludes downstream adapter residual work from `knowledge_fact_count`, so fewer adapter residual decisions do not falsely trip the monotone-knowledge guard
- `generated/` is now intentionally git-ignored and untracked, so local validation snapshots must be recorded in the live docs instead of relying on versioned artifacts
- latest local validation snapshot is now AXI `85/100 GOOD`, APB `90/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`; this refresh replaced a stale optimistic snapshot, the current tracked four-artifact projection now uses APB `IHI0024_E`, AHB remains in the excellent lane, and the latest AXI truthfulness fixes removed the last blocked-handshake residual, the false `AWAKEUP` / `CRVALID` semantic-role conflicts, the false `ACLK` / `ARESETN` interface-direction conflicts, and the bogus `Tie-off`-driven `BROADCAST*` missing-consumer warning while leaving AXI as the main remaining live quality outlier
- APB `IHI0024_D` was re-run from the original PDF through full `specforge converge` with Ollama VLM + NLP Level 3 and converged in 2 passes
- AXI `IHI0022_L` was re-run from the original PDF through full `specforge converge` with Ollama VLM + NLP Level 3, converged in 2 passes, and recovered timing to reach 94/100 EXCELLENT
- `extract_alias_phrase()` now rejects markdown/table/list marker prefixes such as `-`, `|`, `#`, `*`, `+`, `>`, `1.`, and `2)`, closing the last small R12 cleanup in the NLP alias-learning loop
- the documented README staged flow was re-executed on `README.md` through `inspect -> ingest -> evidence -> semantic -> intent -> adapt --dry-run`, confirming the current entry path still runs end-to-end
- the roadmap now explicitly treats adapter expansion as horizon work; the next structural gaps are making the actor-relative graph primary, broadening the new table/prose/alias-grounded role inference into richer multimodal grounding, adding deeper explicit temporal semantics, and hardening KG quality/evaluation
- the current architecture is still intentionally document-local, which is correct for truthfulness, but the next strategic expansion after the current semantic-truthfulness work should be a separate cross-document learning plane that stores reusable extraction priors rather than cross-document facts
- the local Rust test suite is now at `288/288` passing in the latest full CI run after the KG-bench graph-direction expectation surface landed
- `specforge kg-bench` now provides the first tracked KG-quality fixture harness under `crates/specforge/test_data/kg_quality`, including:
  - a gold actor-port recovery fixture
  - a negative name-only semantic noise fixture
  - a negative multi-producer conflict fixture that exercises validation findings
  - an actor-boundary residual-quality fixture
  - a stage-patched contested-handshake fixture that proves contested `XVALID`/`XACK` meaning blocks typed `HandshakeComplete` recovery
  - a stage-patched alias-dependent caveat fixture that proves typed handshake recovery can remain canonical while keeping alias-dependent residual and assumption state explicit
  - a stage-patched cross-modality grounding gold fixture that proves one role can be reinforced jointly by table evidence and visual-caption evidence
  - a stage-patched cross-modality conflict fixture that proves table-versus-visual disagreement stays canonically contested instead of collapsing into false cross-modality consensus
  - a stage-patched VLM timing-note gold fixture that proves a semantic hint can be grounded directly by `vlm_timing_diagram_extraction` and verified at the evidence stage instead of being inferred only from downstream visual-grounding side effects
  - a stage-patched VLM timing-note negative fixture that proves waveform/tick descriptions around `XVALID`-style names still produce timing extraction while semantic-role inference stays at zero
  - a stage-patched visual-source conflict fixture that proves caption semantics and VLM timing-note semantics can disagree while remaining visibly grounded and canonically contested
  - a first representative AMBA-style gold fixture that proves `Source`-column signal-description tables can recover driver-side actor ports plus semantic handshake meaning strongly enough to derive a typed `HandshakeComplete` guard from one constraint
  - a representative AMBA-style `Destination`-column gold fixture that proves receiver-side table rows survive canonically as `Reads` relations and actor-relative input ports
  - a representative APB-style gold fixture that proves `Requester` / `Completer` source roles recover driver-side actor relations, actor-relative ports, request/accept semantics, and typed handshake completion end-to-end
  - a representative APB-style timing gold fixture that proves `Requester` / `Completer` source roles can also recover setup/access timing, actor-grounded temporal predicates, multi-predicate guards, bounded next-cycle latency, and handshake completion together
  - a representative APB address/protection stability fixture that proves `PADDR` and `PPROT` stay actor-grounded stable Requester outputs through wait-state constraints without false handshake completion
  - a representative APB write-control stability fixture that proves `PWRITE`, `PWDATA`, and `PSTRB` stay actor-grounded stable requester outputs through wait-state constraints without false handshake completion
  - a representative APB response stability fixture that proves `PRDATA` and `PSLVERR` stay actor-grounded stable completer outputs through completed-access constraints with real handshake completion
  - a representative AHB control stability fixture that proves `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` stay actor-grounded stable manager outputs through wait-state constraints without false handshake completion
  - a representative AHB transfer/lock stability fixture that proves `HTRANS` and `HMASTLOCK` stay actor-grounded stable manager outputs through wait-state constraints without false handshake completion
  - a representative AHB exclusive/security stability fixture that proves `HEXCL` and `HNONSEC` stay actor-grounded stable manager outputs while `HEXOKAY` stays an actor-grounded stable subordinate output through wait-state constraints without false handshake completion
  - a representative AHB response stability fixture that proves `HRDATA` and `HRESP` stay actor-grounded stable subordinate outputs through wait-state constraints without false handshake completion
  - a representative AHB write-data stability fixture that proves `HWDATA` stays actor-grounded stable as a manager output through three-predicate write wait-state constraints without false handshake completion
  - a representative AXI-style gold fixture that proves width-only channel tables plus prose drive/sample relations recover actor-relative direction, signal inventory, request/accept semantics, and typed handshake completion without any table direction column
  - a representative AXI-style timing gold fixture that proves width-only channel tables plus prose actor relations can also recover next-cycle timing, actor-grounded temporal predicates, and handshake completion together
  - a representative AXI write-response timing fixture that proves the same path on `BVALID` / `BREADY` / `BRESP`, including response-valid timing and response-payload stability
  - a representative AXI write-response ID stability fixture that proves `BID` remains an actor-grounded subordinate transaction-ID obligation under the controlling `BVALID` / `BREADY` handshake
  - a representative AXI read-address timing fixture that proves the same path on `ARVALID` / `ARREADY` / `ARADDR` / `ARLEN`, including read-address-ready timing and read-address payload stability
  - a representative AXI read-address ID stability fixture that proves `ARID` remains an actor-grounded manager transaction-ID obligation under the controlling `ARVALID` / `ARREADY` handshake
  - a representative AXI write-address ID stability fixture that proves `AWID` remains an actor-grounded manager transaction-ID obligation under the controlling `AWVALID` / `AWREADY` handshake
  - a representative AXI write-address control sideband stability fixture that proves `AWPROT`, `AWCACHE`, and `AWLOCK` remain actor-grounded manager control-sideband obligations under the controlling `AWVALID` / `AWREADY` handshake
  - a representative AXI read-address control sideband stability fixture that proves `ARPROT`, `ARCACHE`, and `ARLOCK` remain actor-grounded manager control-sideband obligations under the controlling `ARVALID` / `ARREADY` handshake
  - a representative AXI address QoS/region sideband stability fixture that proves `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION` remain actor-grounded manager sideband obligations under their matching address-channel handshakes
  - a representative AXI read-address sideband stability fixture that proves `ARSIZE` and `ARBURST` remain actor-grounded manager sideband obligations under the controlling `ARVALID` / `ARREADY` handshake
  - a representative AXI read-data timing fixture that proves the same path on `RVALID` / `RREADY` / `RDATA` / `RRESP`, including read-data-valid timing and read-data payload stability
  - a representative AXI read-data ID stability fixture that proves `RID` remains an actor-grounded subordinate transaction-ID obligation under the controlling `RVALID` / `RREADY` handshake
  - a representative AXI read-data response stability fixture that proves `RRESP` remains an actor-grounded subordinate response obligation under the controlling `RVALID` / `RREADY` handshake
  - a representative AXI read-data last stability fixture that proves `RLAST` remains an actor-grounded subordinate sideband obligation under the controlling `RVALID` / `RREADY` handshake
  - a representative AXI address/response USER sideband stability fixture that proves `AWUSER` and `ARUSER` remain actor-grounded manager sideband obligations while `BUSER` remains an actor-grounded subordinate sideband obligation under their matching channel handshakes
  - a representative AXI data USER sideband stability fixture that proves `WUSER` remains an actor-grounded manager sideband obligation and `RUSER` remains an actor-grounded subordinate sideband obligation under their matching data-channel handshakes
  - a representative AXI write-data timing fixture that proves the same path on `WVALID` / `WREADY` / `WDATA` / `WSTRB`, including write-data-ready timing and write-data payload stability
  - a representative AXI write-data last stability fixture that proves `WLAST` remains an actor-grounded manager sideband obligation under the controlling `WVALID` / `WREADY` handshake
  - a representative AXI sideband stability fixture that proves `ARLEN` and `WSTRB` remain actor-grounded sideband obligations under their controlling ready/valid handshakes
  - a representative AXI write-address sideband stability fixture that proves `AWLEN`, `AWSIZE`, and `AWBURST` remain actor-grounded sideband obligations under the controlling `AWVALID` / `AWREADY` handshake
  - a representative AHB-style gold fixture that proves `Manager signals` / `Subordinate signals` section context recovers per-signal direction and width correctly from staged `SourceIR`
  - a representative AHB-style timing gold fixture that proves the same section-heading and destination-column context can also recover wait-state timing, actor-grounded temporal predicates, bounded next-cycle latency, and multi-predicate guards together
  - the canonical parser now also accepts width-only synthesized declarations like `Signal AWVALID is width 1.`, which closes the specific AXI gap where table-grounded widths previously stopped at actor relations and never became real interface signal records
  - a bogus-actor-attribution negative fixture that proves `Clock` / `Reset` infrastructure rows in AMBA-style `Source` columns do not become protocol actors while the true requester/subordinate rows still survive canonically
  - a field-table misclassification negative fixture that proves a misclassified `Bits | Name | Description` table does not synthesize fake top-level signals or semantic roles from field names that merely look signal-like
  - a spurious-timing negative fixture that proves low-value VLM timing-diagram labels like `T0`, `Addr 1`, `Cycle 2`, `D0`, `A1`, `DATA0`, `0xAA`, `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` still count as timing-diagram extraction at the evidence stage but do not survive into `TimingConstraintRecord` or `TemporalRuleRecord`
  - a VLM waveform-motion negative fixture that proves `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `HIGH_TO_LOW`, `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` timing states do not become symbolic signal values while a concrete `HIGH` sample still survives
  - a VLM state-machine label-noise negative fixture that proves prose/OCR labels like `IDLE state` and `ACCESS phase` do not become canonical FSM state names or transition endpoints while clean identifier evidence still survives
  - a VLM state-machine undeclared-transition negative fixture that proves identifier-shaped but undeclared endpoints such as `DONE` and `RESET` do not become canonical transition graph facts
  - a VLM state-machine duplicate-initial gold fixture that proves duplicate state labels merge and preserve an initial marker if any duplicate carries it
- the harness can now also assert canonical actor-signal relations directly, so tracked gold fixtures can lock `Drives` versus `Reads` truth instead of checking only actor-port projections or relation counts
- the harness can now also patch `SourceIR.document_sections` and assert per-signal canonical direction directly, which is important for protocol families like AHB where section-heading context still carries real directionality
- the harness now also asserts canonical semantic candidates and decisive-vs-contested semantic arbitration directly, which is a better `R15e` truthfulness check than inferring arbitration quality only from blocked fallback or validation side effects
- the harness now also asserts canonical state names, initial-state names, and transition endpoints directly, so VLM state-machine truthfulness can be locked in tracked fixtures instead of only in unit tests
- the harness now also asserts persisted validation metric values directly at the evidence, semantic, and intent stages and can patch `SourceIR` visual assets, which makes tracked cross-modality grounding and VLM-note provenance checks practical instead of leaving them to ad hoc unit tests
- table-based relation extraction itself is also less lossy now:
  - `Source` / `Driver` columns become `Drives`
  - `Destination` columns become `Reads`
  - direction/infrastructure placeholders like `input`, `Clock`, and `Reset` are filtered instead of being promoted into bogus actor names
- table-driven top-level signal synthesis is also less lossy now:
  - field-like `Bits | Name | Description` layouts are rejected even if they were misclassified upstream as `signal_description`
  - that guard now protects fake-signal leakage across declarations, semantic hints, and related table-driven inference paths
- timing-diagram lifting is also less noisy now:
  - label-only VLM annotations such as `T0`, `Addr 1`, and `Cycle 2` are treated as waveform labels, not as timing semantics
  - VLM timing signal-value states such as `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `HIGH_TO_LOW`, `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` are treated as waveform motion descriptors, not as concrete symbolic signal values
  - the semantic lift now keeps the timing extraction visible upstream while refusing to fabricate canonical timing constraints from those low-value labels alone
- that negative multimodal fixture also locks an important nuance in the validation surface:
  - conflicting multimodal evidence should still count as visual grounding
  - but it must not count as resolved cross-modality grounding
- that means benchmark hardening is no longer purely roadmap text; the repo now has a real seed harness for false-positive control and residual-quality regression across AMBA, APB, AHB, and expanding APB/AHB/AXI protocol-grade gold suites, even though broader AXI and deeper structured-constraint suites are still ahead
- the harness is also more realistic now than a pure markdown fixture runner because tracked fixtures can patch `SourceIR` and `EvidenceIR` surfaces directly, which is a practical way to benchmark richer multimodal/structured semantics without needing a heavyweight source document for every regression

## Observed current state
### Repository contents directly observed
- `.git/`
- `.gitmodules`
- live documentation surface
- `INTENTIR_SPEC.md`
- `Cargo.toml`
- `Cargo.lock`
- `crates/specforge/Cargo.toml`
- `crates/specforge/src/main.rs`
- `crates/specforge/src/lib.rs`
- `crates/specforge/src/cli.rs`
- `crates/specforge/src/error.rs`
- `crates/specforge/src/commands/inspect.rs`
- `crates/specforge/src/commands/converge.rs`
- `crates/specforge/src/commands/ingest.rs`
- `crates/specforge/src/commands/evidence.rs`
- `crates/specforge/src/commands/semantic.rs`
- `crates/specforge/src/commands/intent.rs`
- `crates/specforge/src/commands/adapt.rs`
- `crates/specforge/src/commands/enrich.rs`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/commands/project_validation.rs`
- `crates/specforge/src/commands/rescan_plan.rs`
- `crates/specforge/src/commands/kg_bench.rs`
- `crates/specforge/src/commands/learn_priors.rs`
- `crates/specforge/src/commands/nlp_enrich.rs`
- `crates/specforge/src/test_support.rs`
- `crates/specforge/src/ir/mod.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/src/ir/source/docling_backend.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/adapters.rs`
- `crates/specforge/src/ir/prior_memory.rs`
- `subs/fsmgen/`

### Rust-specific contents still absent
- no dedicated `specforge-source` crate
- no dedicated `specforge-evidence` crate
- no dedicated `specforge-semantic` crate
- no dedicated `specforge-intent` crate
- no dedicated `specforge-adapters` crate
- no dedicated validation crate
- no separate Rust integration-test crate; the tracked KG-quality fixture harness lives inside the active crate under `crates/specforge/test_data/kg_quality`
- no additional real builders beyond the current `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR` slices and the first `.fsm` adapter slice

### Immediate implication
- the codebase is no longer mostly scaffolding; the main open problem is semantic truthfulness across the four IR layers, especially graph primacy, temporal semantics, multimodal rescans, and evidence arbitration
- the practical risk is now split across two partial migrations:
  - `SemanticIR` and `IntentIR` still expose both graph-native actor-relative records and legacy flat `direction_hint` fields; validation/scoring is now graph-first and explicit-module plus unambiguous standalone direct `.fsm` paths have graph-backed direction overlays, but some downstream compatibility and consumer paths still consult the flat hints directly
  - the new temporal-rule layer is real and now includes bounded cycle windows, compound antecedents, typed temporal conflicts, and first actor-grounded drive/stability predicates, but it still covers only a narrow slice of possible temporal/actor semantics and does not yet arbitrate broader cross-rule or cross-modality contradictions
- the continuity risk around untracked generated artifacts is lower now that validation snapshots can be re-projected into tracked docs deterministically, but the docs still depend on someone running the projection flow after meaningful validation runs

## What the tool needs to do
- build `SourceIR` from raw specifications and normalized artifacts
- build `EvidenceIR` from normalized markdown, page assets, figures, captions, and evidence extraction
- build `SemanticIR` from actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- build canonical `IntentIR` as a backend-independent intent model
- lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- validate stage outputs and adapter outputs and back-annotate findings

## Current implemented architecture
### Root workspace
- `Cargo.toml`
  - workspace root
- `Cargo.lock`
  - dependency lockfile

### Active crate
- `crates/specforge`
  - single user-facing CLI crate and binary for the current slice

### Implemented module boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch and public module exports
- `src/cli.rs`
  - clap-based command model using the `specforge` binary name
- `src/error.rs`
  - typed error/result boundary
- `src/commands/inspect.rs`
  - deterministic source/path inspection command
- `src/commands/converge.rs`
  - fixed-point orchestration command over persisted `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR`/adapter artifacts
- `src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `src/commands/adapt.rs`
  - `.fsm` adapter preview/materialization command
- `src/commands/enrich.rs`
  - VLM-backed visual enrichment command for `SourceIR`
- `src/commands/validate.rs`
  - stage-aware artifact validation and quality-scoring command
- `src/commands/project_validation.rs`
  - project-level validation snapshot projection and schema-v2 targeted rescan recommendation generation
- `src/commands/rescan_plan.rs`
  - dry-run-first consumer and whitelisted local executor for schema-v2 targeted rescan plans
- `src/commands/kg_bench.rs`
  - tracked KG-quality benchmark harness over staged IR artifacts and persisted validation findings
- `src/commands/learn_priors.rs`
  - local typed `CorpusMemory` harvesting command for advisory cross-document extraction priors
- `src/commands/nlp_enrich.rs`
  - LLM-backed NLP Level 3 enrichment command for `EvidenceIR`
- `src/test_support.rs`
  - shared process-global test synchronization utilities for env-var-mutating CLI tests
- `src/ir/mod.rs`
  - stage identifiers for `source_ir`, `evidence_ir`, `semantic_ir`, and `intent_ir`
- `src/ir/source.rs`
  - concrete `SourceIR` implementation, structured source types, `WidthHint`, and actor-signal relation type definitions
- `src/ir/source/docling_backend.rs`
  - external backend discovery, Docling command orchestration, and the embedded Python helper for structured PDF normalization
- `src/ir/evidence.rs`
  - concrete `EvidenceIR` builder, markdown parsing, caption/reference linking, table synthesis, typed NLP extraction, alias persistence, and actor-signal relation extraction
- `src/ir/semantic.rs`
  - concrete `SemanticIR` builder, semantic lifting heuristics, VLM merge logic, and residual-decision generation
- `src/ir/intent.rs`
  - concrete `IntentIR` builder, canonicalization heuristics, and residual-decision preservation
- `src/ir/adapters.rs`
  - typed adapter artifacts, `.fsm` lowering logic, renderability gating, and adapter-side residual-decision generation
- `src/ir/prior_memory.rs`
  - typed local prior-memory schema and helper logic for advisory cross-document extraction priors

## Assessment of current structure
### What is good
- the crate/binary identity now matches the repo direction
- the code no longer hardcodes `.fsm` as the conceptual endpoint
- a real typed `SourceIR` artifact exists instead of a handwritten ingest plan
- a real structured PDF normalization path now exists inside `SourceIR`, so the first stage is operational for both Markdown and PDF inputs
- a real typed `EvidenceIR` artifact now exists, so the staged pipeline is operational beyond raw source normalization
- a real typed `SemanticIR` artifact now exists, so the staged pipeline now reaches a backend-neutral semantic layer before the final canonicalization stage
- a real typed `IntentIR` artifact now exists, so the end-to-end source-to-intent pipeline is operational before adapter lowering
- the later stages have typed names and module homes, which reduces the risk of accidental backend-first growth
- adapter lowering is separated from the canonical IR stages
- the IR surface now carries page and visual manifests plus backend source references that later stages can ground against
- the repository now also contains a pinned local `fsmgen` checkout, which gives the next `.fsm` adapter slice a nearby reference implementation without changing the canonical `IntentIR` boundary
- that `fsmgen` checkout is now explicitly contextual and read-only from the `specforge` side; any observed upstream misbehavior should be captured as a local `FSMGEN-BUG-####` report instead of a submodule edit
- the first `.fsm` adapter slice already enforces honest renderability boundaries instead of fabricating target text from under-specified intent
- the canonical model now preserves typed signal inventory and backend-neutral guarded/action control fragments before the adapter boundary
- the canonical model now also preserves backend-neutral system contract and init-assignment records for explicit standalone sequential control, including first-class reset polarity/assertion/release/target semantics
- the canonical model now also preserves explicit regular-state and transition records for stateful lowering
- the canonical model now also preserves canonical symbol-definition sections and structured control blocks, including dedicated synchronous-reset and asynchronous-reset control roles
- the `.fsm` adapter can now emit a real standalone `?dt:name` file for explicit combinational and sequential DT cases, canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, compound-update shorthand, and a real structured `?fsm:name` file for explicit state-graph cases when those canonical facts are explicit enough

### What is still insufficient
- only the `.fsm` adapter is implemented today; SystemVerilog, Verilog, and VHDL adapters are still absent
- validation now backannotates persisted IR artifacts, writes stage-local sidecars, and can project the latest staged snapshot back into tracked docs
- the actor-signal relation graph now survives into `SemanticIR` / `IntentIR`, and the first explicit-module `.fsm` composition consumer can use matching `actor_ports` to recover child-module port directions, but legacy interface records still flatten some other downstream consumers onto actor-agnostic `direction_hint` values
- the previous dead-code warning baseline has been cleaned by deleting stale helpers rather than suppressing them
- the current renderable `.fsm` slices are intentionally narrow: they handle explicit standalone combinational/sequential DT control, canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, compound-update shorthand, explicit structured FSM-root cases, and explicit top-root composition, while broader unsupported selector/predicate shapes and non-FSM backends stay deferred

## Architectural recommendation
### Core architectural stance
- keep `IntentIR` as the canonical endpoint
- keep adapters downstream of `IntentIR`
- keep the internal system of record typed and stage-specific
- keep residual decisions explicit at every stage
- do not let convenience around one backend contaminate the stage-neutral model
- use structured parsing first and selective multimodal enrichment second, rather than collapsing the problem into markdown-only OCR or ungrounded VLM generation
- keep actor/signal relations first-class long enough that downstream adapter work does not have to rediscover them from flattened `input` / `output` hints

### Recommended growth path from the current codebase
#### Keep in the current crate for the next slices
- extend the current validation projection flow beyond staged IR artifacts into downstream adapter artifacts
- finish moving the downstream signal-direction model from compatibility flat hints to actor-relative semantics before serious SystemVerilog adapter work
- keep compatibility-level `?mod:name` / `?module:name` spellings outside the adapter root-kind model until a real backend-neutral direct-module distinction exists
- keep any new composition/control enrichment backend-neutral so the canonical model boundary stays intact
- keep the latest projected four-artifact snapshot visible as follow-on work lands: AXI `85/100 GOOD`, APB `90/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`, with AXI still the main quality outlier

#### Split into dedicated crates when pressure becomes real
- `specforge-source`
  - source registration, normalization, converter orchestration
- `specforge-evidence`
  - section anchors, evidence spans, statement extraction, relation extraction, and provenance
- `specforge-semantic`
  - actor and semantic lifting
- `specforge-intent`
  - canonical intent model and versioned serialization
- `specforge-adapters`
  - target-specific lowerings
- `specforge-validate`
  - validation, diagnostics, and back-annotation

## Mapping from staged architecture to the current modules
### SourceIR
- current primary ownership:
  - `src/commands/ingest.rs`
  - `src/ir/source.rs`

### EvidenceIR
- current declared ownership:
  - `src/commands/evidence.rs`
  - `src/ir/evidence.rs`
- current executable behavior:
  - builds `EvidenceIR` from ready `SourceIR`, promoted markdown, visual-asset manifests, and structured tables
  - synthesizes signal, enum, register, and timing evidence from structured tables
  - extracts structured signal constraints and conditional rules from classified sentences
  - extracts actor-signal relation triples from prose verb patterns and signal-description table role columns
  - re-enters a monotone convergence loop so signal anchors, discovered enum members, dynamic prose constraints, polarity refinement, and KG-derived direction synthesis can reinforce one another before hand-off to `SemanticIR`
  - persists signal-alias state so `specforge nlp-enrich` can tighten the evidence iteratively across passes

### SemanticIR
- current declared ownership:
  - `src/commands/semantic.rs`
  - `src/ir/semantic.rs`
- dependency:
  - requires real `EvidenceIR`
- current executable behavior:
  - builds `SemanticIR` from persisted `EvidenceIR`, deriving actors, actor-relative port/connectivity records, interfaces, typed signal records, backend-neutral control fragments, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions
  - merges VLM timing/state observations and filters NLP outputs through the declared-signal gate
  - now preserves `actor_signal_relations`, `actor_ports`, and `signal_connectivity`, but still keeps compatibility-level actor-agnostic `direction_hint` values on interface records

### IntentIR
- current declared ownership:
  - `src/commands/intent.rs`
  - `src/ir/intent.rs`
- dependency:
  - requires real `SemanticIR`
- current executable behavior:
  - builds `IntentIR` from persisted `SemanticIR`, deriving intent identity, actor responsibilities, interface inventory, backend-neutral control fragments, behaviors, constraints, assumptions, and residual decisions

### Adapters
- current declared ownership:
  - `src/commands/adapt.rs` — `.fsm` adapter preview/materialization command
  - `src/commands/enrich.rs` — VLM diagram enrichment command (Ollama/OpenAI/LM Studio)
  - `src/commands/validate.rs` — artifact health validation command with quality score
  - `src/commands/nlp_enrich.rs` — NLP Level 3 evidence-enrichment command
  - `src/ir/mod.rs`
- dependency:
  - requires stable `IntentIR`
- nearby reference implementation:
  - `subs/fsmgen/`
- local workflow rule:
  - treat `subs/fsmgen` as read-only contextual input
  - if upstream behavior looks wrong, file a local tracked bug report under `FSMGEN-BUG-####` rather than patching the submodule here
- current executable behavior:
  - builds a typed `.fsm` adapter artifact from persisted `IntentIR`
  - chooses `?dt:name` for explicit standalone DT cases, `?fsm:name` when explicit regular-state and transition records are present, and `?top:name` when explicit module/top composition facts are present
  - consumes canonical signal inventory, backend-neutral system/init records, backend-neutral control fragments, explicit regular-state/transition records, and explicit module/top composition facts when present
  - overlays explicit module signal inventory with matching `IntentIR.actor_ports` so graph-backed module actor directions can fill missing child-module port hints during `?top:name` renderability analysis
  - overlays standalone direct signal inventory with graph-backed actor-port directions only when all renderable actor-port evidence for signals already present in the local direct inventory points at one unambiguous actor
  - recovers missing top boundary port directions from explicit top-link source/target topology before rendering `?top:name`
  - emits a real standalone `?dt:name` file only when widths, directions, guarded/action blocks, and any required standalone sequential system/init facts are explicit enough to avoid semantic invention
  - emits a real structured `?fsm:name` file only when the state graph, transition targets, and state-body control are explicit enough to avoid semantic invention
  - emits a real explicit `?top:name` source document only when the top ports, child modules, and links are explicit enough to avoid semantic invention
- next real implementation target:
  - complete the transition to graph-first actor-relative direction semantics, then extend the same validation/reporting discipline into downstream non-FSM adapter work

## Major risks
### Risk: backend leakage into IntentIR
- if `.fsm` or RTL-specific assumptions creep back into the canonical model, the pivot fails even if the names remain correct
### Risk: actor-agnostic direction collapse
- if the current actor-signal relation graph is flattened too early into one-size-fits-all `input` / `output` hints, downstream adapters will encode the wrong actor perspective and hide the real structural knowledge the pipeline already extracted

### Risk: incomplete validation/back-annotation
- if validation findings never flow back into persisted artifacts and live docs, the pipeline will remain executable but harder to trust, compare, and iterate on

### Risk: markdown-only drift for PDFs
- if the real builder treats markdown as the only normalized representation, the system will silently lose figure, chart, and layout semantics before `EvidenceIR`

### Risk: ungrounded visual descriptions
- if multimodal descriptions are generated without stable links back to page regions, captions, and source references, later stages will be vulnerable to hallucinated evidence
### Risk: mixed Rust/Python backend seam
- the SourceIR PDF path now depends on a Rust-to-Python orchestration boundary and an external Docling runtime
- interpreter discovery, package installation, and first-run model downloads are operational concerns that must stay explicit in docs and tests
- this is acceptable temporarily, but should be closed soon so the first stage is truly operational for PDFs

## Testing implications
- current full local CI path: `bash scripts/run_ci.sh`
- current active Rust surface after the README/bootstrap refresh: `31` Rust source files and `80,456` Rust source lines under `crates/specforge/src`
- current Rust test count observed through the canonical local CI path after the latest slice: `518` Rust tests, all passing under warning-deny Clippy/rustdoc plus the mdBook build
- current tracked KG-quality fixture count: 127
- current tests cover:
  - source-kind detection
  - deterministic source key naming
  - `SourceIR` JSON materialization
  - directory residual decision emission
  - PDF normalization planning
  - PDF materialization through a stubbed backend override
  - Docling table-kind and diagram-kind classification
  - markdown-backed `EvidenceIR` construction
  - table-synthesized signal, enum, register, and timing evidence
  - anchored encoding-table rescans and dynamic value-constraint extraction
  - polarity refinement from active-low / active-high prose, including explicit asserted-when-level, collective active-level, safe mixed clause-local control wording, and detached mixed-polarity rejection
  - caption and figure-reference grounding into visual evidence
  - VLM observation injection (TimingDiagramExtraction, StateMachineExtraction from VisualAsset.note)
  - handshake-driven `SemanticIR` actor/interface/invariant extraction
  - structured-table signal direction+width extraction through EvidenceIR → SemanticIR
  - sticky SemanticIR interface direction/width conflict collapse when conflicting declarations repeat
  - parametric-width handling through the IR pipeline
  - VLM timing diagram annotation → TimingConstraintRecord in SemanticIR
  - VLM timing signal-value filtering for waveform motion states, separator variants, and compact edge spellings such as rising/stable/falling/RISING_EDGE/LOW_TO_HIGH/POS_EDGE/LOW2HIGH
  - VLM state machine extraction → duplicate-merged identifier-bounded RegularStateRecord plus same-observation declared-endpoint-gated StateTransitionRecord in SemanticIR
  - ambiguous visual-grounding residual decisions in `SemanticIR`
  - handshake-driven `IntentIR` identity/behavior/constraint/assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
  - NLP Level 3 extraction, backannotation, and alias learning
  - markdown/table/list marker alias rejection for Form 2 alias learning
  - actor-signal relation extraction from prose and table roles
  - AMBA `Source` / `Driver` / `Destination` signal-direction handling
  - `.fsm` adapter renderability, including graph-backed top-composition child direction recovery, bidirectional top-link boundary/child and sibling-child width recovery, unambiguous standalone direct direction recovery, direct and explicit-module control-input width recovery from actor-port shape, sticky actor-port direction/width conflict blocking, graph-backed sequential system-contract direction recovery, graph-only direct-context filtering, top-link boundary direction recovery, top actor-port boundary direction/width recovery, and conflict/ambiguity blocking
  - SourceIR PDF materialization tests sharing one process-global environment lock with Docling runtime tests
  - `specforge validate` for all four IR stages
  - project-level validation projection and schema-v2 rescan-plan generation
  - schema-v2 `rescan-plan` normalization, whitelisted execution, execution summaries, and no-promotion review gates
  - bounded prior-family harvest/consumption for actor taxonomy, semantic phrases, temporal phrases, table shapes, semantic modality reliability, visual motifs, and negative knowledge
  - KG-quality fixture execution across gold, negative, conflict, multimodal, prior-guided, infrastructure, and active-low VLM timing polarity-equivalence cases
- next tests should cover:
  - richer APB and AXI end-to-end fixtures for relation-driven direction coverage
  - actor-relative direction modeling once it lands in `SemanticIR` / `IntentIR`
  - wider `.fsm` renderability coverage and snapshot stability on protocol-heavy fixtures
  - future adapter targets beyond the current `.fsm` slice

## Latest validation completed in this refresh
- `cargo fmt --manifest-path Cargo.toml -- --check`
  - passed
- `bash scripts/run_docs_ci.sh`
  - passed and rebuilt the mdBook into `generated/mdbook/specforge`
- `bash scripts/run_ci.sh`
  - passed with `518` Rust tests, `0` failures, warning-deny Clippy/rustdoc, and a successful mdBook build
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench`
  - passed with `127` fixtures and `0` failures
- `git diff --check`
  - passed

## Session update (2026-04-19 README bootstrap analysis refresh)
- Re-executed the README handoff path through `SESSION_BOOTSTRAP.md`, reread the linked continuity and user-facing markdown surfaces, and resurveyed the active Rust crate layout directly from disk.
- The current Rust implementation now spans `31` source files and `77,759` lines under `crates/specforge/src`, with the tracked KG fixture suite at `127` and the canonical local CI path at `498` passing Rust tests plus warning-deny rustdoc and mdBook validation.
- The bootstrap pass did not reveal a new architectural pivot or an unlogged product-surface drift; the recent graph-direction validation work is already represented in the live docs.
- The meaningful action from this refresh is simply keeping the bootstrap analysis truthful, so future resumed sessions start from current numbers instead of stale ones.

## Session update (2026-04-18 README bootstrap analysis refresh)
- Re-executed the README handoff path through `SESSION_BOOTSTRAP.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `MEMORY.md`, and `COMMIT.md`, then resurveyed the active Rust crate layout directly from disk.
- The current Rust implementation now spans `31` source files and `62,017` lines under `crates/specforge/src`, with the tracked KG fixture suite at `89` and the canonical local CI path at `351` passing Rust tests plus warning-deny rustdoc and mdBook validation.
- The bootstrap pass did not reveal a new architectural pivot, but it did reaffirm the most important remaining codebase-level gap from the roadmap: `R15` is still incomplete because several downstream adapter and validation seams continue to consume compatibility `direction_hint` rather than purely actor-relative graph semantics.
- That means the next meaningful implementation work should favor graph-first downstream direction consumers over more adapter-family breadth or superficial scoring tweaks.

## Earlier validation trail
- `cargo run --manifest-path Cargo.toml -p specforge -- --help`
  - passed and confirmed the current CLI surface includes `enrich`, `validate`, and `nlp-enrich`
- `cargo test --manifest-path Cargo.toml`
  - passed with 99 tests
- `cargo run -p specforge -- inspect README.md`
  - passed; confirmed the repo entry source is detected as markdown
- `cargo run -p specforge -- ingest README.md --dry-run`
  - passed; confirmed `SourceIR` planning for the README entrypoint
- `cargo run -p specforge -- ingest README.md`
  - passed; materialized `generated/source_ir/readme/source_ir.json`
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - passed; confirmed README-backed `EvidenceIR` preview
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - passed; materialized `generated/evidence_ir/readme/evidence_ir.json` with 15 section anchors and 190 extracted statements
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - passed; confirmed README-backed `SemanticIR` preview
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - passed; materialized `generated/semantic_ir/readme/semantic_ir.json` with 2 actors, 6 phases, 5 invariants, and 8 gates
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - passed; confirmed README-backed `IntentIR` preview
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - passed; materialized `generated/intent_ir/readme/intent_ir.json` with 2 actors, 14 behaviors, 6 constraints, and 1 assumption
- `cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run`
  - passed; produced the expected honest blocked `.fsm` adapter plan for the README-derived intent surface

## Current recommendation
- keep the current single-crate workspace for one more slice
- keep `IntentIR` canonical and resist any temptation to make `.fsm` the hidden endpoint again
- take the alias-marker cleanup as complete and treat it as evidence that the current multi-spec extraction stack is ready for the next slice
- finish promoting the signal model from compatibility hints to actor-relative direction semantics next, then extend the same validation/reporting discipline into downstream RTL adapter work
- do not treat NLP Level 3 as the missing piece anymore; the pipeline now has both Level 3 enrichment and convergent typed EvidenceIR reuse

## Session update (2026-04-18 VLM timing bus-label annotation rejection)
- Continued from commit `d7f7281`, hardening VLM timing truthfulness rather than broadening extraction authority.
- `is_generic_waveform_label_token()` now recognizes short bus/waveform labels such as `Burst`, `Packet`, `Frame`, `Transaction`, and `Txn` as the same low-value annotation family as `Addr`, `Cycle`, `D0`, `DATA[3]`, and `XREQ[3:0]`.
- This closes a realistic chip-spec PDF failure mode where timing-diagram annotation gutters label burst/transaction phases, but those labels are descriptive figure markup rather than timing constraints that should survive into canonical IR.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock the expanded annotation-noise set while preserving the grounded `signals[].values[]` temporal evidence path.

## Session update (2026-04-18 Compact VLM timing bus-label rejection)
- Continued from commit `619a773`, tightening the same timing truthfulness boundary for compact diagram layout.
- `is_compact_waveform_sample_label()` now recognizes `Burst1`, `Packet2`, `Frame3`, `Transaction4`, and `Txn5` as the compact equivalents of the already-filtered spaced bus labels.
- This closes the space-constrained waveform-gutter variant common in chip-spec PDFs, where bus-phase labels are squeezed into one token but still do not encode protocol timing law.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock both spaced and compact bus-label annotation noise while preserving the grounded temporal sample path.

## Session update (2026-04-18 Compact VLM phase/transfer label rejection)
- Continued from commit `867455e`, tightening the same compact-layout timing truthfulness boundary one step further.
- `is_compact_waveform_sample_label()` now recognizes `Phase1` and `Transfer2` as compact waveform-gutter labels rather than timing constraints.
- This closes the remaining compact-layout gap inside the existing generic waveform label vocabulary: `phase` and `transfer` were already filtered when tokenized, but not when layout collapsed them into a single token.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock compact phase/transfer labels alongside the existing spaced and compact annotation-noise families.

## Session update (2026-04-18 Multi-token VLM waveform-label rejection)
- Continued from commit `5996029`, tightening the same annotation-noise boundary for short multi-token waveform-gutter labels.
- `is_spurious_timing_annotation_label()` now rejects all-generic annotation groups up to four tokens instead of three.
- This closes a real leak where labels such as `Channel 1 Phase 2` or `Lane 0 Slot 1` could previously survive as `TimingConstraintRecord`s even though every token was already in the generic waveform-label vocabulary.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock these four-token markup labels alongside the shorter spaced and compact label-noise families.

## Session update (2026-04-18 Name-only VLM timing label rejection)
- Continued from commit `74a74de`, tightening the same timing truthfulness boundary for bare known-signal labels.
- `parse_timing_diagram_observation()` now rejects annotations that are only a known signal name, using the document-grounded `known_signal_names` set rather than generic waveform-label heuristics.
- This closes a separate leak where labels such as `XREQ` could previously survive as `TimingConstraintRecord`s simply because they were neither generic markup nor sentence-shaped timing statements.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock bare known-signal labels alongside the existing generic spaced, compact, and multi-token annotation-noise families.

## Session update (2026-04-18 Signal-value VLM timing label rejection)
- Continued from commit `53f5953`, tightening the same timing truthfulness boundary for known-signal sampled-value lane labels.
- `parse_timing_diagram_observation()` now rejects annotations that are exactly a known signal plus one simple sampled-value token such as `HIGH`, `LOW`, `asserted`, or `deasserted`.
- This closes the follow-on leak where labels such as `XREQ HIGH` could previously survive as `TimingConstraintRecord`s even though the structured `signals[].values[]` path already represents sampled VLM timing values more honestly.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock these two-token known-signal label variants alongside the existing name-only, generic spaced, compact, and multi-token annotation-noise families.
## Session update (2026-04-19 graph-direction conflict vs coverage-gap split)
- Continued from commit `f8164d0`, tightening the warning boundary inside validation rather than adding new canonical data.
- Same-actor graph-direction conflicts no longer also emit the generic `*_graph_direction_coverage_incomplete` finding.
- The split is now cleaner:
  - conflict findings mean graph evidence exists but disagrees
  - coverage-gap findings mean the graph still provides no usable direction for those signals
- Updated the tracked `graph_direction_same_actor_conflict_negative` fixture to assert `finding_ids_exclude` for the generic coverage-gap finding, so the benchmark surface now locks that non-overlap directly.
- Formatting, four focused validator regressions, two focused tracked fixture runs, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-19 graph-direction coverage finding related IDs)
- Continued from commit `eeb67a8`, tightening the graph-direction validation surface rather than widening extractor or adapter scope.
- `semantic_graph_direction_coverage_incomplete` and `intent_graph_direction_coverage_incomplete` now emit the missing signal names in `related_ids` instead of remaining count-only findings.
- The split with the previous conflict work stays deliberate:
  - coverage-gap findings remain signal-level because they answer which canonical signals still lack graph-derived direction
  - same-actor contradiction findings remain actor-aware because they answer which actor-signal pair caused the unresolved conflict
- Added focused validator regressions for both canonical stages plus a tracked `graph_direction_coverage_incomplete_negative` fixture.
- Refreshed the managed corpus-KB benchmark and pattern pages; the tracked truthfulness suite now reports `91/91` fixtures and the semantic/truthfulness pattern page reports `50` fixtures.
- Formatting, two focused validator regressions, the focused tracked fixture run, full local CI, corpus-KB refresh, and whitespace checking passed for this slice.
