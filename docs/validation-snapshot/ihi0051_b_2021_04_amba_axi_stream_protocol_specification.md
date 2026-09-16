# IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf
Per-document detail for `ihi0051_b_2021_04_amba_axi_stream_protocol_specification`, routed from [VALIDATION_SNAPSHOT.md](../../VALIDATION_SNAPSHOT.md). Refreshed only by `specforge project-validation` after the review gate.

## Targeted Rescan Recommendations
### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
- finding_id: `intent_interface_signal_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related interface conflict ids survive with consistent direction/width declarations instead of unresolved interface-shape ambiguity
- related_ids: `interface_signal_conflict_0001`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- replay_inputs: `source_ir:generated/source_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/source_ir.json`, `evidence_ir:generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
- finding_id: `intent_negative_knowledge_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence
- related_ids: `interface_signal_conflict_0001`, `signal_connectivity_conflict_0001`, `signal_connectivity_conflict_0002`, `signal_connectivity_conflict_0003`, `signal_connectivity_conflict_0004`, `signal_connectivity_conflict_0005`, `signal_connectivity_conflict_0006`, `temporal_conflict_0001`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_evidence_ir`: `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/source_ir.json`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
- finding_id: `intent_prior_guided_semantic_consensus_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with stronger current-document semantic-role consensus instead of prior-guided carry-through
- related_ids: `TREADY`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
- finding_id: `intent_signal_connectivity_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related connectivity conflict ids survive with single-producer connectivity instead of unresolved producer ambiguity
- related_ids: `signal_connectivity_conflict_0001`, `signal_connectivity_conflict_0002`, `signal_connectivity_conflict_0003`, `signal_connectivity_conflict_0004`, `signal_connectivity_conflict_0005`, `signal_connectivity_conflict_0006`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal conflict ids survive with a single locally corroborated timing obligation instead of contradictory value obligations
- related_ids: `temporal_conflict_0001`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`

### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- replay_inputs: `semantic_ir:generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_handshake_completion_gap_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: rebuild IntentIR after targeted semantic/evidence rescans and keep the related canonical ids explicit until corroborated
- related_ids: `TKEEP`, `TREADY`, `TVALID`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`

## Projected Artifacts
### IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0051_b_2021_04_amba_axi_stream_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- artifact_fingerprint: `7cce2a1cef094239`
- score: `67/100 ADEQUATE`
- summary: IntentIR validation for IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf with 14 finding(s)
- findings:
  - [warning:interface_signal_conflicts] 1 interface signal conflict(s) detected; conflicting direction/width evidence is still unresolved in the carried interface surface
  - [warning:quality_score] IntentIR quality score is 67/100 (ADEQUATE)
  - [warning:signal_connectivity] 6 signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity
  - [warning:temporal_conflicts] 1 typed temporal conflict(s) detected across contradictory value obligations
  - [info:negative_knowledge] 8 carried conflict/residual pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current intent evidence
  - [info:rescan_guidance] 1 interface conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current canonical interface surface still carries unresolved direction/width disagreement
  - [info:rescan_guidance] 8 negative-knowledge prior match(es) in IntentIR should be routed to targeted rescans and require stronger local corroboration before canonical promotion; prior memory did not suppress or rewrite current evidence
  - [info:rescan_guidance] 1 prior-guided semantic-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning was strengthened by learned modality-reliability priors
  - [info:rescan_guidance] 6 connectivity conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current structural graph still carries unresolved producer ambiguity
  - [info:rescan_guidance] 1 temporal conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current typed timing surface still carries contradictory value obligations
  - [info:rescan_guidance] 3 handshake-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because handshake semantic roles exist but no typed temporal rule currently expresses a HandshakeComplete predicate
  - [info:semantic_role_consensus] 6 resolved semantic role(s) now carry explicit consensus that was strengthened by learned modality-reliability priors
  - [info:system_contract] 1 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface
  - [info:temporal_grounding] 3 declared signal(s) carry handshake semantic roles (ValidLike/ReadyLike) but no typed temporal rule currently expresses a HandshakeComplete predicate
