# IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf
Per-document detail for `ihi0024_e_2023_02_amba_5_apb_protocol_specification`, routed from [VALIDATION_SNAPSHOT.md](../../VALIDATION_SNAPSHOT.md). Refreshed only by `specforge project-validation` after the review gate.

## Targeted Rescan Recommendations
### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
- finding_id: `intent_connectivity_missing_producer_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with producer-side connectivity evidence instead of remaining producerless
- related_ids: `LEVEL`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`

### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
- finding_id: `intent_interface_signal_conflict_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related interface conflict ids survive with consistent direction/width declarations instead of unresolved interface-shape ambiguity
- related_ids: `interface_signal_conflict_0001`, `interface_signal_conflict_0002`, `interface_signal_conflict_0003`, `interface_signal_conflict_0004`, `interface_signal_conflict_0005`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`

### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- replay_inputs: `source_ir:generated/source_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/source_ir.json`, `evidence_ir:generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
- finding_id: `intent_negative_knowledge_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence
- related_ids: `interface_signal_conflict_0001`, `interface_signal_conflict_0002`, `interface_signal_conflict_0003`, `interface_signal_conflict_0004`, `interface_signal_conflict_0005`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_evidence_ir`: `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/source_ir.json`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`

### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- replay_inputs: `evidence_ir:generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`, `semantic_ir:generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
- finding_id: `intent_prior_guided_semantic_consensus_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with stronger current-document semantic-role consensus instead of prior-guided carry-through
- related_ids: `PREADY`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `nlp_enrich_evidence_ir`: `cargo run --manifest-path Cargo.toml -- nlp-enrich generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json --vlm-provider ollama`
  - `rebuild_semantic_ir`: `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json`
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`

### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- replay_inputs: `semantic_ir:generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_handshake_completion_gap_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: rebuild IntentIR after targeted semantic/evidence rescans and keep the related canonical ids explicit until corroborated
- related_ids: `PAUSER`, `PBUSER`, `PNSE`, `PREADY`, `PRUSER`, `PSTRB`, `PWUSER`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`

### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- replay_inputs: `semantic_ir:generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
- finding_id: `intent_temporal_multi_predicate_antecedents_surface_rescan_guidance`
- extractor_lane: `intent_ir_canonical_surface_corroboration`
- corroboration_policy: `stronger_local_corroboration_required_before_canonical_promotion`
- recommended_action: rebuild IntentIR after targeted semantic/evidence rescans and keep the related canonical ids explicit until corroborated
- related_ids: `temporal_signal_constraint_llm_sigcon_0010`, `temporal_signal_constraint_llm_sigcon_0017`, `temporal_signal_constraint_llm_sigcon_0018`
- automation_status: `planned_not_executed`
- recommended_commands:
  - `rebuild_intent_ir`: `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json`
  - `validate_current_artifact`: `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`

## Projected Artifacts
### IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf (intent_ir)
- document_key: `ihi0024_e_2023_02_amba_5_apb_protocol_specification`
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- artifact_fingerprint: `14af5b5627fc08fd`
- score: `74/100 GOOD`
- summary: IntentIR validation for IHI0024_E_2023-02_AMBA_5_APB_Protocol_Specification.pdf with 15 finding(s)
- findings:
  - [warning:interface_signal_conflicts] 5 interface signal conflict(s) detected; conflicting direction/width evidence is still unresolved in the carried interface surface
  - [warning:quality_score] IntentIR quality score is 74/100 (GOOD)
  - [warning:signal_connectivity] 1 signal(s) in IntentIR connectivity have no resolved producer actor
  - [info:compatibility_surface] 2 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist
  - [info:negative_knowledge] 5 carried conflict/residual pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current intent evidence
  - [info:rescan_guidance] 1 signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current connectivity graph still lacks resolved producer-side connectivity evidence
  - [info:rescan_guidance] 5 interface conflict id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because the current canonical interface surface still carries unresolved direction/width disagreement
  - [info:rescan_guidance] 5 negative-knowledge prior match(es) in IntentIR should be routed to targeted rescans and require stronger local corroboration before canonical promotion; prior memory did not suppress or rewrite current evidence
  - [info:rescan_guidance] 1 prior-guided semantic-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning was strengthened by learned modality-reliability priors
  - [info:rescan_guidance] 7 handshake-role signal id(s) in IntentIR should trigger targeted NLP rescans plus downstream rebuild because handshake semantic roles exist but no typed temporal rule currently expresses a HandshakeComplete predicate
  - [info:rescan_guidance] 3 temporal rule id(s) in IntentIR carry multi-predicate antecedents; compound condition extraction may benefit from further review
  - [info:semantic_role_consensus] 12 resolved semantic role(s) now carry explicit consensus that was strengthened by learned modality-reliability priors
  - [info:system_contract] 2 infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface
  - [info:temporal_grounding] 7 declared signal(s) carry handshake semantic roles (ValidLike/ReadyLike) but no typed temporal rule currently expresses a HandshakeComplete predicate
  - [info:temporal_grounding] 3 typed temporal rule(s) carry multi-predicate antecedents; compound condition extraction may benefit from further review
