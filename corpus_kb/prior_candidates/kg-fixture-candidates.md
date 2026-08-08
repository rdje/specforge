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
- review_scope: `family_surface_not_individual_prior`
- promotion_status: `candidate_not_promoted_review_required`
- canonical_mutation_allowed: `false`
- corpus_memory_mutation_allowed: `false`

| candidate_kind | target_schema | supporting | positive | guard | required_gates |
| --- | --- | ---: | ---: | ---: | --- |
| `actor_taxonomy_prior` | `CorpusMemory.actor_taxonomy_priors` | `3` | `1` | `2` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding consumer |
| `negative_knowledge_prior` | `CorpusMemory.negative_knowledge_priors` | `7` | `6` | `7` | typed CorpusMemory schema; caution-only validation consumer; paired KG-bench conflict/residual coverage; rescan guidance review gate |
| `semantic_modality_reliability_prior` | `CorpusMemory.semantic_modality_reliability_priors` | `2` | `1` | `1` | typed CorpusMemory schema; paired KG-bench conflict coverage; validated IntentIR harvest input; local-grounded arbitration consumer |
| `semantic_phrase_prior` | `CorpusMemory.semantic_phrase_priors` | `9` | `2` | `7` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding semantic consumer |
| `table_shape_prior` | `CorpusMemory.table_shape_priors` | `5` | `2` | `3` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated SourceIR/IntentIR harvest chain; local table-kind consumer |
| `temporal_phrase_prior` | `CorpusMemory.temporal_phrase_priors` | `3` | `1` | `2` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; local-grounding temporal consumer |
| `visual_motif_prior` | `CorpusMemory.visual_motif_priors` | `3` | `1` | `2` | typed CorpusMemory schema; paired KG-bench gold/negative coverage; validated IntentIR harvest input; VLM/multimodal corroboration gate |

### Fixture Evidence
Each evidence fixture is listed on its own line so growth remains reviewable and bounded.

#### `actor_taxonomy_prior`
- positive_gates:
  - `actor_taxonomy_prior_guided_section_direction_gold`
- guard_gates:
  - `actor_taxonomy_prior_guided_section_direction_without_prior_negative`
  - `actor_taxonomy_prior_protocol_family_mismatch_negative`

#### `negative_knowledge_prior`
- positive_gates:
  - `negative_knowledge_prior_guided_connectivity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_interface_conflict_caution_gold`
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_residual_caution_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_guided_temporal_conflict_caution_gold`
- guard_gates:
  - `negative_knowledge_prior_guided_connectivity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_interface_conflict_caution_gold`
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold`
  - `negative_knowledge_prior_guided_residual_caution_gold`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `negative_knowledge_prior_guided_temporal_conflict_caution_gold`
  - `negative_knowledge_prior_protocol_family_mismatch_negative`

#### `semantic_modality_reliability_prior`
- positive_gates:
  - `semantic_modality_reliability_prior_guided_conflict_gold`
- guard_gates:
  - `semantic_modality_reliability_prior_guided_conflict_without_prior_negative`

#### `semantic_phrase_prior`
- positive_gates:
  - `semantic_prior_guided_phrase_gold`
  - `visual_semantic_prior_guided_caption_gold`
- guard_gates:
  - `semantic_prior_broad_phrase_negative`
  - `semantic_prior_conflicting_roles_negative`
  - `semantic_prior_guided_phrase_without_prior_negative`
  - `semantic_prior_phrase_mismatch_negative`
  - `semantic_prior_protocol_family_mismatch_negative`
  - `semantic_prior_source_kind_mismatch_negative`
  - `visual_semantic_prior_guided_caption_without_prior_negative`

#### `table_shape_prior`
- positive_gates:
  - `table_shape_prior_guided_signal_table_gold`
  - `table_shape_prior_guided_timing_table_gold`
- guard_gates:
  - `table_shape_prior_guided_signal_table_without_prior_negative`
  - `table_shape_prior_guided_timing_table_without_prior_negative`
  - `table_shape_prior_protocol_family_mismatch_negative`

#### `temporal_phrase_prior`
- positive_gates:
  - `temporal_prior_guided_cycle_window_gold`
- guard_gates:
  - `temporal_prior_guided_cycle_window_without_prior_negative`
  - `temporal_prior_protocol_family_mismatch_negative`

#### `visual_motif_prior`
- positive_gates:
  - `visual_motif_prior_guided_diagram_classification_gold`
- guard_gates:
  - `visual_motif_prior_guided_diagram_classification_without_prior_negative`
  - `visual_motif_prior_protocol_family_mismatch_negative`

### Readiness Summary
Readiness is fixture-surface readiness only. It is not promotion approval and does not allow `CorpusMemory` or canonical IR mutation.

| candidate_kind | readiness | supporting | positive | guard | promotion_boundary |
| --- | --- | ---: | ---: | ---: | --- |
| `actor_taxonomy_prior` | `fixture_paired_review_ready` | `3` | `1` | `2` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `negative_knowledge_prior` | `caution_surface_review_ready` | `7` | `6` | `7` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `semantic_modality_reliability_prior` | `fixture_paired_review_ready` | `2` | `1` | `1` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `semantic_phrase_prior` | `fixture_paired_review_ready` | `9` | `2` | `7` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `table_shape_prior` | `fixture_paired_review_ready` | `5` | `2` | `3` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `temporal_phrase_prior` | `fixture_paired_review_ready` | `3` | `1` | `2` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |
| `visual_motif_prior` | `fixture_paired_review_ready` | `3` | `1` | `2` | `review_only_no_corpus_memory_or_canonical_ir_mutation` |

### Promotion Gate Review Matrix
These gates describe the family-level implementation surface already visible to review. They are not approval records and do not grant mutation authority.

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
