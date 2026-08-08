# Validation Finding Patterns

This page is the first auto-refreshable page family in the `R15g` corpus knowledge base.
It records validation-finding patterns from reviewable validation reports without promoting them into canonical document truth.

## Human Synthesis

Use this section for curated notes that explain recurring validation patterns across documents.
Keep provenance explicit, and do not treat this page as an approval artifact.

## Managed Validation Projection

<!-- corpus_kb_validation_findings:start -->
<!-- This reviewed block is refreshed from `VALIDATION_SNAPSHOT.md` by `specforge corpus-kb --validation-snapshot`. -->

### ihi0022_l_2025_08_amba_axi_protocol_specification
- artifact_path: `generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json`
- stage: `intent_ir`
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

### ihi0024_e_2023_02_amba_5_apb_protocol_specification
- artifact_path: `generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json`
- stage: `intent_ir`
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

### ihi0033_c_2021_09_amba_5_ahb_protocol_specification
- artifact_path: `generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json`
- stage: `intent_ir`
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

### ihi0051_b_2021_04_amba_axi_stream_protocol_specification
- artifact_path: `generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json`
- stage: `intent_ir`
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

<!-- corpus_kb_validation_findings:end -->
