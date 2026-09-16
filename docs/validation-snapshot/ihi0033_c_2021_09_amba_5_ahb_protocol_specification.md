# IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf
Per-document detail for `ihi0033_c_2021_09_amba_5_ahb_protocol_specification`, routed from [VALIDATION_SNAPSHOT.md](../../VALIDATION_SNAPSHOT.md). Refreshed only by `specforge project-validation` after the review gate.

## Targeted Rescan Recommendations
### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_connectivity_missing_consumer_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with consumer-side connectivity evidence instead of remaining consumerless
- related_ids: `HAUSER`, `HAUSERCHK`, `HBUSER`, `HBUSERCHK`, `HRUSER`, `HRUSERCHK`, `HWUSER`, `HWUSERCHK`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_connectivity_missing_producer_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with producer-side connectivity evidence instead of remaining producerless
- related_ids: `HADDR`, `HADDRCHK`, `HBURST`, `HCTRLCHK1`, `HCTRLCHK2`, `HEXCL`, `HEXOKAY`, `HMASTER`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_graph_direction_coverage_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with actor-relative graph direction coverage instead of remaining graph-uncovered
- related_ids: `HRESET`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_interface_signal_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related interface conflict ids survive with consistent direction/width declarations instead of unresolved interface-shape ambiguity
- related_ids: `interface_signal_conflict_0001`, `interface_signal_conflict_0002`, `interface_signal_conflict_0003`, `interface_signal_conflict_0004`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `source_ir:generated/source_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/source_ir.json`, `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_negative_knowledge_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence
- related_ids: `interface_signal_conflict_0001`, `interface_signal_conflict_0002`, `interface_signal_conflict_0003`, `interface_signal_conflict_0004`, `signal_connectivity_conflict_0001`, `signal_connectivity_conflict_0002`, `signal_connectivity_conflict_0003`, `signal_connectivity_conflict_0004`, `signal_connectivity_conflict_0005`, `signal_connectivity_conflict_0006`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_evidence_ir`: `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/source_ir.json`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_prior_guided_semantic_consensus_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with stronger current-document semantic-role consensus instead of prior-guided carry-through
- related_ids: `HREADYOUT`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_signal_connectivity_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related connectivity conflict ids survive with single-producer connectivity instead of unresolved producer ambiguity
- related_ids: `signal_connectivity_conflict_0001`, `signal_connectivity_conflict_0002`, `signal_connectivity_conflict_0003`, `signal_connectivity_conflict_0004`, `signal_connectivity_conflict_0005`, `signal_connectivity_conflict_0006`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- replay_inputs: `semantic_ir:generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_handshake_completion_gap_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: rebuild IntentIR after targeted semantic/evidence rescans and keep the related canonical ids explicit until corroborated
- related_ids: `HADDR`, `HREADYOUT`, `HTRANS`, `HWSTRB`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`

## Projected Artifacts
### IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0033_c_2021_09_amba_5_ahb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `f2b3e0591b4a5fbc`
- score: `65/100 ADEQUATE`
- summary: IntentIR validation for IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf with 20 finding(s)
- findings:
  - [warning:interface_signal_conflicts] 4 interface signal conflict(s) detected; conflicting direction/width evidence is still unresolved in the carried interface surface
  - [warning:quality_score] IntentIR quality score is 65/100 (ADEQUATE)
  - [warning:signal_connectivity] 8 signal(s) in IntentIR connectivity have no resolved consumer actor
  - [warning:signal_connectivity] 21 signal(s) in IntentIR connectivity have no resolved producer actor
  - [warning:signal_connectivity] 6 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [info:compatibility_surface] 1 declared signal record(s) still lack flat compatibility direction hints and actor-relative graph coverage
  - [info:compatibility_surface] 10 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist
  - [info:knowledge_graph] 1 declared signal record(s) still lack graph-derived direction coverage
  - [info:negative_knowledge] 10 carried conflict/residual pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current intent evidence
  - [info:rescan_guidance] 8 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current connectivity graph still lacks resolved consumer-side connectivity evidence
  - [info:rescan_guidance] 8 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current connectivity graph still lacks resolved producer-side connectivity evidence
  - [info:rescan_guidance] 1 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current canonical surface still lacks actor-relative graph direction coverage
  - [info:rescan_guidance] 4 interface conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current canonical interface surface still carries unresolved direction/width disagreement
  - [info:rescan_guidance] 10 negative-knowledge prior match(es) in IntentIR should be routed to targeted rescans and require stronger local corroboration before canonical promotion; prior memory did not suppress or rewrite current evidence
  - [info:rescan_guidance] 1 prior-guided semantic-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning was strengthened by learned modality-reliability priors
  - [info:rescan_guidance] 6 connectivity conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current structural graph still carries unresolved producer ambiguity
  - [info:rescan_guidance] 4 handshake-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because handshake semantic roles exist but no typed temporal rule currently expresses a HandshakeComplete predicate
  - [info:semantic_role_consensus] 5 resolved semantic role(s) now carry explicit consensus that was strengthened by learned modality-reliability priors
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface
  - [info:temporal_grounding] 4 declared signal(s) carry handshake semantic roles (ValidLike/ReadyLike) but no typed temporal rule currently expresses a HandshakeComplete predicate
