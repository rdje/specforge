# KG Fixture Prior Candidates

This page records review-only prior candidates derived from tracked KG fixture patterns.
It is an explicit candidate surface, not a `CorpusMemory` artifact and not a promotion approval.

## Human Synthesis

Use this section for curated notes about which candidate prior families should become typed harvesters or consumers next.
Every machine-usable promotion must still pass through an explicit schema, KG-bench coverage, validation, and local-grounding review.

## Managed Prior Candidate Projection

<!-- corpus_kb_prior_candidates:start -->
<!-- This block is refreshed by `specforge corpus-kb`. -->

- source: `kg-bench fixtures`
- review_scope: `capability_surface_not_individual_prior`
- promotion_status: `candidate_not_promoted_review_required`
- canonical_mutation_allowed: `false`
- corpus_memory_mutation_allowed: `false`

| candidate_kind | target_schema | supporting | prior-present | control | required_gates |
| --- | --- | ---: | ---: | ---: | --- |
| `actor_taxonomy_prior` | `CorpusMemory.actor_taxonomy_priors` | `54` | `2` | `15` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding consumer |
| `negative_knowledge_prior` | `CorpusMemory.negative_knowledge_priors` | `7` | `7` | `7` | typed CorpusMemory schema; caution-only validation consumer; paired KG-bench conflict/residual coverage; rescan guidance review gate |
| `semantic_modality_reliability_prior` | `CorpusMemory.semantic_modality_reliability_priors` | `64` | `5` | `35` | typed CorpusMemory schema; paired KG-bench conflict coverage; validated IntentIR harvest input; local-grounded arbitration consumer |
| `semantic_phrase_prior` | `CorpusMemory.semantic_phrase_priors` | `64` | `9` | `35` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding semantic consumer |
| `table_shape_prior` | `CorpusMemory.table_shape_priors` | `65` | `3` | `24` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated SourceIR/IntentIR harvest chain; local table-kind consumer |
| `temporal_phrase_prior` | `CorpusMemory.temporal_phrase_priors` | `79` | `2` | `23` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding temporal consumer |
| `visual_motif_prior` | `CorpusMemory.visual_motif_priors` | `26` | `2` | `23` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; VLM/multimodal corroboration gate |

### Fixture Evidence
Each evidence fixture is listed on its own line so growth remains reviewable and bounded.

#### `actor_taxonomy_prior`
- prior_present_surfaces:
  - `actor_taxonomy_prior_guided_section_direction_gold`
  - `actor_taxonomy_prior_term_mismatch_negative`
- control_surfaces:
  - `actor_ports_gold`
  - `actor_taxonomy_prior_guided_section_direction_without_prior_negative`
  - `actor_taxonomy_prior_term_mismatch_negative`
  - `compat_direction_hints_graph_conflict_incomplete_negative`
  - `compat_direction_hints_incomplete_negative`
  - `compat_direction_hints_mixed_lag_incomplete_negative`
  - `connectivity_endpoint_gaps_negative`
  - `graph_direction_coverage_incomplete_negative`
  - `graph_direction_same_actor_conflict_negative`
  - `multi_producer_conflict_negative`
  - `negative_knowledge_prior_guided_connectivity_conflict_caution_gold`
  - `relative_clause_actor_noise_negative`
  - `table_shape_prior_header_mismatch_negative`
  - `temporal_cycle_window_grounded_gold`
  - `temporal_cycle_window_surface_negative`

#### `negative_knowledge_prior`
- prior_present_surfaces:
  - `negative_knowledge_prior_guided_connectivity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_interface_conflict_caution_gold`
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_residual_caution_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_guided_temporal_conflict_caution_gold`
  - `negative_knowledge_prior_pattern_mismatch_negative`
- control_surfaces:
  - `negative_knowledge_prior_guided_connectivity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_interface_conflict_caution_gold`
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_residual_caution_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_guided_temporal_conflict_caution_gold`
  - `negative_knowledge_prior_pattern_mismatch_negative`

#### `semantic_modality_reliability_prior`
- prior_present_surfaces:
  - `semantic_modality_reliability_global_prior_gold`
  - `semantic_modality_reliability_prior_guided_conflict_gold`
  - `semantic_modality_reliability_role_mismatch_negative`
  - `semantic_modality_reliability_source_kind_mismatch_negative`
  - `semantic_modality_reliability_weak_prior_negative`
- control_surfaces:
  - `alias_dependent_handshake_completion_caveat`
  - `contested_handshake_name_fallback_negative`
  - `cross_modality_semantic_conflict_negative`
  - `cross_modality_semantic_grounding_gold`
  - `interface_signal_conflict_negative`
  - `name_only_semantic_noise_negative`
  - `negative_knowledge_prior_guided_interface_conflict_caution_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_pattern_mismatch_negative`
  - `semantic_modality_reliability_global_prior_gold`
  - `semantic_modality_reliability_prior_guided_conflict_gold`
  - `semantic_modality_reliability_prior_guided_conflict_without_prior_negative`
  - `semantic_modality_reliability_role_mismatch_negative`
  - `semantic_modality_reliability_source_kind_mismatch_negative`
  - `semantic_modality_reliability_weak_prior_negative`
  - `semantic_prior_broad_phrase_negative`
  - `semantic_prior_conflicting_roles_negative`
  - `semantic_prior_guided_phrase_gold`
  - `semantic_prior_guided_phrase_without_prior_negative`
  - `semantic_prior_normalized_phrase_mismatch_negative`
  - `semantic_prior_phrase_mismatch_negative`
  - `semantic_prior_source_kind_mismatch_negative`
  - `semantic_ready_sink_prior_guided_phrase_gold`
  - `semantic_ready_sink_prior_guided_phrase_without_prior_negative`
  - `semantic_valid_prior_guided_phrase_gold`
  - `semantic_valid_prior_guided_phrase_without_prior_negative`
  - `table_misclassification_field_table_negative`
  - `visual_motif_prior_caption_mismatch_negative`
  - `visual_motif_prior_guided_diagram_classification_gold`
  - `visual_motif_prior_guided_diagram_classification_without_prior_negative`
  - `visual_semantic_prior_guided_caption_gold`
  - `visual_semantic_prior_guided_caption_without_prior_negative`
  - `visual_sources_semantic_conflict_negative`
  - `vlm_timing_name_only_semantic_noise_negative`
  - `vlm_timing_semantic_grounding_gold`

#### `semantic_phrase_prior`
- prior_present_surfaces:
  - `semantic_prior_broad_phrase_negative`
  - `semantic_prior_conflicting_roles_negative`
  - `semantic_prior_guided_phrase_gold`
  - `semantic_prior_normalized_phrase_mismatch_negative`
  - `semantic_prior_phrase_mismatch_negative`
  - `semantic_prior_source_kind_mismatch_negative`
  - `semantic_ready_sink_prior_guided_phrase_gold`
  - `semantic_valid_prior_guided_phrase_gold`
  - `visual_semantic_prior_guided_caption_gold`
- control_surfaces:
  - `alias_dependent_handshake_completion_caveat`
  - `contested_handshake_name_fallback_negative`
  - `cross_modality_semantic_conflict_negative`
  - `cross_modality_semantic_grounding_gold`
  - `interface_signal_conflict_negative`
  - `name_only_semantic_noise_negative`
  - `negative_knowledge_prior_guided_interface_conflict_caution_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_pattern_mismatch_negative`
  - `semantic_modality_reliability_global_prior_gold`
  - `semantic_modality_reliability_prior_guided_conflict_gold`
  - `semantic_modality_reliability_prior_guided_conflict_without_prior_negative`
  - `semantic_modality_reliability_role_mismatch_negative`
  - `semantic_modality_reliability_source_kind_mismatch_negative`
  - `semantic_modality_reliability_weak_prior_negative`
  - `semantic_prior_broad_phrase_negative`
  - `semantic_prior_conflicting_roles_negative`
  - `semantic_prior_guided_phrase_gold`
  - `semantic_prior_guided_phrase_without_prior_negative`
  - `semantic_prior_normalized_phrase_mismatch_negative`
  - `semantic_prior_phrase_mismatch_negative`
  - `semantic_prior_source_kind_mismatch_negative`
  - `semantic_ready_sink_prior_guided_phrase_gold`
  - `semantic_ready_sink_prior_guided_phrase_without_prior_negative`
  - `semantic_valid_prior_guided_phrase_gold`
  - `semantic_valid_prior_guided_phrase_without_prior_negative`
  - `table_misclassification_field_table_negative`
  - `visual_motif_prior_caption_mismatch_negative`
  - `visual_motif_prior_guided_diagram_classification_gold`
  - `visual_motif_prior_guided_diagram_classification_without_prior_negative`
  - `visual_semantic_prior_guided_caption_gold`
  - `visual_semantic_prior_guided_caption_without_prior_negative`
  - `visual_sources_semantic_conflict_negative`
  - `vlm_timing_name_only_semantic_noise_negative`
  - `vlm_timing_semantic_grounding_gold`

#### `table_shape_prior`
- prior_present_surfaces:
  - `table_shape_prior_guided_signal_table_gold`
  - `table_shape_prior_guided_timing_table_gold`
  - `table_shape_prior_header_mismatch_negative`
- control_surfaces:
  - `actor_taxonomy_prior_guided_section_direction_without_prior_negative`
  - `actor_taxonomy_prior_term_mismatch_negative`
  - `contested_handshake_name_fallback_negative`
  - `control_polarity_conflict_negative`
  - `cross_modality_semantic_conflict_negative`
  - `cross_modality_semantic_grounding_gold`
  - `message_field_register_table_negative`
  - `message_field_table_gold`
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold`
  - `semantic_modality_reliability_global_prior_gold`
  - `semantic_modality_reliability_prior_guided_conflict_gold`
  - `semantic_modality_reliability_prior_guided_conflict_without_prior_negative`
  - `semantic_modality_reliability_role_mismatch_negative`
  - `semantic_modality_reliability_source_kind_mismatch_negative`
  - `semantic_modality_reliability_weak_prior_negative`
  - `signal_presence_malformed_refusal_negative`
  - `signal_presence_matrix_gold`
  - `signal_table_inventory_authority_negative`
  - `table_misclassification_field_table_negative`
  - `table_shape_prior_guided_signal_table_without_prior_negative`
  - `table_shape_prior_header_mismatch_negative`
  - `visual_motif_prior_caption_mismatch_negative`
  - `visual_motif_prior_guided_diagram_classification_gold`
  - `visual_motif_prior_guided_diagram_classification_without_prior_negative`

#### `temporal_phrase_prior`
- prior_present_surfaces:
  - `temporal_prior_guided_cycle_window_gold`
  - `temporal_prior_phrase_mismatch_negative`
- control_surfaces:
  - `alias_dependent_handshake_completion_caveat`
  - `contested_handshake_name_fallback_negative`
  - `detached_mixed_control_polarity_negative`
  - `mixed_control_polarity_gold`
  - `multi_control_polarity_gold`
  - `negative_knowledge_prior_guided_temporal_conflict_caution_gold`
  - `non_reset_control_polarity_gold`
  - `table_misclassification_field_table_negative`
  - `temporal_actor_grounding_surface_negative`
  - `temporal_clock_grounding_surface_negative`
  - `temporal_conflict_negative`
  - `temporal_cycle_window_grounded_gold`
  - `temporal_cycle_window_surface_negative`
  - `temporal_prior_guided_cycle_window_gold`
  - `temporal_prior_guided_cycle_window_without_prior_negative`
  - `temporal_prior_phrase_mismatch_negative`
  - `vlm_timing_active_low_assertion_equivalence_gold`
  - `vlm_timing_active_low_deassertion_equivalence_gold`
  - `vlm_timing_cycle_qualified_signal_value_annotation_negative`
  - `vlm_timing_indexed_signal_value_annotation_negative`
  - `vlm_timing_motion_annotation_negative`
  - `vlm_timing_spurious_annotation_negative`
  - `vlm_timing_waveform_motion_negative`

#### `visual_motif_prior`
- prior_present_surfaces:
  - `visual_motif_prior_caption_mismatch_negative`
  - `visual_motif_prior_guided_diagram_classification_gold`
- control_surfaces:
  - `cross_modality_semantic_conflict_negative`
  - `cross_modality_semantic_grounding_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_pattern_mismatch_negative`
  - `visual_motif_prior_caption_mismatch_negative`
  - `visual_motif_prior_guided_diagram_classification_gold`
  - `visual_motif_prior_guided_diagram_classification_without_prior_negative`
  - `visual_semantic_prior_guided_caption_gold`
  - `visual_semantic_prior_guided_caption_without_prior_negative`
  - `visual_sources_semantic_conflict_negative`
  - `vlm_state_machine_duplicate_initial_gold`
  - `vlm_state_machine_label_noise_negative`
  - `vlm_state_machine_missing_initial_negative`
  - `vlm_state_machine_undeclared_transition_negative`
  - `vlm_timing_active_low_assertion_equivalence_gold`
  - `vlm_timing_active_low_deassertion_equivalence_gold`
  - `vlm_timing_cycle_qualified_signal_value_annotation_negative`
  - `vlm_timing_indexed_signal_value_annotation_negative`
  - `vlm_timing_motion_annotation_negative`
  - `vlm_timing_name_only_semantic_noise_negative`
  - `vlm_timing_semantic_grounding_gold`
  - `vlm_timing_spurious_annotation_negative`
  - `vlm_timing_waveform_motion_negative`

### Readiness Summary
Readiness is fixture-surface readiness only. It is not promotion approval and does not allow `CorpusMemory` or canonical IR mutation.

| candidate_kind | readiness | supporting | prior-present | control | promotion_boundary |
| --- | --- | ---: | ---: | ---: | --- |
| `actor_taxonomy_prior` | `prior_and_control_surfaces_present` | `54` | `2` | `15` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `negative_knowledge_prior` | `caution_surface_review_ready` | `7` | `7` | `7` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `semantic_modality_reliability_prior` | `prior_and_control_surfaces_present` | `64` | `5` | `35` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `semantic_phrase_prior` | `prior_and_control_surfaces_present` | `64` | `9` | `35` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `table_shape_prior` | `prior_and_control_surfaces_present` | `65` | `3` | `24` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `temporal_phrase_prior` | `prior_and_control_surfaces_present` | `79` | `2` | `23` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `visual_motif_prior` | `prior_and_control_surfaces_present` | `26` | `2` | `23` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |

### Promotion Gate Review Matrix
These gates describe the capability-level implementation surface already visible to review. They are not approval records and do not grant mutation authority.

| candidate_kind | schema_gate | fixture_gate | harvest_gate | consumer_gate | promotion_boundary |
| --- | --- | --- | --- | --- | --- |
| `actor_taxonomy_prior` | `present_schema_v5` | `paired_gold_without_prior_fixture` | `learn_priors_actor_taxonomy_harvester_present` | `evidence_actor_taxonomy_local_grounding_consumer_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `negative_knowledge_prior` | `present_schema_v5` | `caution_fixtures_across_conflict_residual_families_present` | `learn_priors_conflict_residual_negative_knowledge_harvester_present` | `validation_caution_and_rescan_guidance_consumer_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `semantic_modality_reliability_prior` | `present_schema_v5` | `paired_conflict_gold_without_prior_fixture` | `learn_priors_modality_reliability_harvester_present` | `semantic_arbitration_reliability_consumer_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `semantic_phrase_prior` | `present_schema_v5` | `paired_text_and_visual_gold_without_prior_fixtures` | `learn_priors_semantic_phrase_harvester_present` | `evidence_semantic_phrase_local_grounding_consumer_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `table_shape_prior` | `present_schema_v5` | `paired_signal_and_timing_table_gold_without_prior_fixtures` | `learn_priors_source_ir_table_shape_harvester_present` | `evidence_table_kind_local_shape_consumer_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `temporal_phrase_prior` | `present_schema_v5` | `paired_gold_without_prior_fixture` | `learn_priors_temporal_phrase_harvester_present` | `semantic_temporal_phrase_cycle_window_consumer_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `visual_motif_prior` | `present_schema_v5` | `paired_visual_motif_gold_without_prior_fixture` | `learn_priors_source_ir_visual_motif_harvester_present` | `evidence_visual_caption_motif_consumer_with_corroboration_gate_present` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |

<!-- corpus_kb_prior_candidates:end -->
