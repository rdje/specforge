# IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf
Per-document detail for `ihi0022_l_2025_08_amba_axi_protocol_specification`, routed from [VALIDATION_SNAPSHOT.md](../../VALIDATION_SNAPSHOT.md). Refreshed only by `specforge project-validation` after the review gate.

## Targeted Rescan Recommendations
### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_connectivity_missing_consumer_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with consumer-side connectivity evidence instead of remaining consumerless
- related_ids: `AXLEN`, `BROADCASTSHAREABLE`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_connectivity_missing_producer_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with producer-side connectivity evidence instead of remaining producerless
- related_ids: `AXPROT`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_graph_direction_coverage_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with actor-relative graph direction coverage instead of remaining graph-uncovered
- related_ids: `ACADDRCHK`, `ACCRDTCHK`, `ACPENDINGCHK`, `ACREADYCHK`, `ACTIVATEACKCHK`, `ACTIVATEACKDCHK`, `ACTIVATEREQCHK`, `ACTIVATEREQDCHK`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `source_ir:generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json`, `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_negative_knowledge_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence
- related_ids: `signal_connectivity_conflict_0001`, `signal_connectivity_conflict_0002`, `signal_connectivity_conflict_0003`, `signal_connectivity_conflict_0004`, `temporal_conflict_0003`, `temporal_conflict_0005`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_evidence_ir`: `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_prior_guided_semantic_consensus_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with stronger current-document semantic-role consensus instead of prior-guided carry-through
- related_ids: `ACREADY`, `ACVALID`, `ARACT`, `ARACTV`, `ARMMUATST`, `ARMMUSSIDV`, `ARPBHA`, `ARREADY`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_signal_connectivity_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related connectivity conflict ids survive with single-producer connectivity instead of unresolved producer ambiguity
- related_ids: `signal_connectivity_conflict_0001`, `signal_connectivity_conflict_0002`, `signal_connectivity_conflict_0003`, `signal_connectivity_conflict_0004`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal conflict ids survive with a single locally corroborated timing obligation instead of contradictory value obligations
- related_ids: `temporal_conflict_0001`, `temporal_conflict_0002`, `temporal_conflict_0003`, `temporal_conflict_0004`, `temporal_conflict_0005`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_cycle_window_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with explicit cycle-window bounds instead of remaining unbounded
- related_ids: `temporal_signal_constraint_llm_sigcon_0000`, `temporal_signal_constraint_llm_sigcon_0002`, `temporal_signal_constraint_llm_sigcon_0003`, `temporal_signal_constraint_llm_sigcon_0004`, `temporal_signal_constraint_llm_sigcon_0006`, `temporal_signal_constraint_llm_sigcon_0007`, `temporal_signal_constraint_llm_sigcon_0008`, `temporal_signal_constraint_llm_sigcon_0009`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- replay_inputs: `semantic_ir:generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_handshake_completion_gap_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: rebuild IntentIR after targeted semantic/evidence rescans and keep the related canonical ids explicit until corroborated
- related_ids: `ACREADY`, `ACVALID`, `ARACT`, `ARACTV`, `ARMMUATST`, `ARMMUSSIDV`, `ARPBHA`, `ARREADY`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`

## Projected Artifacts
### IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0022_l_2025_08_amba_axi_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- artifact_fingerprint: `2a4daf2cba8a708c`
- score: `71/100 GOOD`
- summary: IntentIR validation for IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf with 21 finding(s)
- findings:
  - [warning:quality_score] IntentIR quality score is 71/100 (GOOD)
  - [warning:signal_connectivity] 2 signal(s) in IntentIR connectivity have no resolved consumer actor
  - [warning:signal_connectivity] 1 signal(s) in IntentIR connectivity have no resolved producer actor
  - [warning:signal_connectivity] 4 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [warning:temporal_conflicts] 5 typed temporal conflict(s) detected across contradictory value obligations
  - [info:compatibility_surface] 118 declared signal record(s) still lack flat compatibility direction hints and actor-relative graph coverage
  - [info:knowledge_graph] 123 declared signal record(s) still lack graph-derived direction coverage
  - [info:negative_knowledge] 6 carried conflict/residual pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current intent evidence
  - [info:rescan_guidance] 2 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current connectivity graph still lacks resolved consumer-side connectivity evidence
  - [info:rescan_guidance] 1 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current connectivity graph still lacks resolved producer-side connectivity evidence
  - [info:rescan_guidance] 8 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current canonical surface still lacks actor-relative graph direction coverage
  - [info:rescan_guidance] 6 negative-knowledge prior match(es) in IntentIR should be routed to targeted rescans and require stronger local corroboration before canonical promotion; prior memory did not suppress or rewrite current evidence
  - [info:rescan_guidance] 8 prior-guided semantic-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning was strengthened by learned modality-reliability priors
  - [info:rescan_guidance] 4 connectivity conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current structural graph still carries unresolved producer ambiguity
  - [info:rescan_guidance] 5 temporal conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current typed timing surface still carries contradictory value obligations
  - [info:rescan_guidance] 8 typed temporal rule id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current rules still lack explicit cycle-window bounds
  - [info:rescan_guidance] 8 handshake-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because handshake semantic roles exist but no typed temporal rule currently expresses a HandshakeComplete predicate
  - [info:semantic_role_consensus] 273 resolved semantic role(s) now carry explicit consensus that was strengthened by learned modality-reliability priors
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface
  - [info:temporal_grounding] 36 declared signal(s) carry handshake semantic roles (ValidLike/ReadyLike) but no typed temporal rule currently expresses a HandshakeComplete predicate
  - [info:temporal_grounding] typed temporal rules exist, but none currently carry explicit cycle-window bounds
