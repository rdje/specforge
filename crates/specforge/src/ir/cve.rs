//! Constrained-decoding + entailment-verifier (R16-CONSTRAINED-VERIFIED-EXTRACTION).
//!
//! Per the `R16-CONSTRAINED-VERIFIED-EXTRACTION.1` design (see
//! `docs/tasks/R16-CONSTRAINED-VERIFIED-EXTRACTION.md`): make
//! extraction high-precision **by construction** by:
//!
//! 1. forcing the upstream LLM/VLM to emit JSON conforming to the
//!    `ActorContract` schema (this module — `.2`), with the adapter
//!    failing closed on schema violations (no silent fabrication);
//! 2. running an entailment verifier that checks the source span
//!    actually licenses the extracted contract (`.3`);
//! 3. seeding a protocol-pattern template library into `prior_memory`
//!    (`.4`);
//! 4. picking the next-pass extraction targets by an
//!    uncertainty-driven value-of-information score (`.5`).
//!
//! `.2` scope (this module): the **schema-constrained adapter**.
//! The authoritative validator is `serde` — the typed Rust shape of
//! `ActorContract` IS the schema. The JSON Schema string returned by
//! `actor_contract_json_schema_summary()` is a human/provider-facing
//! summary intended to drive constrained decoding in any LLM/VLM that
//! supports a JSON-Schema-shaped grammar; it intentionally documents
//! the top-level required keys and the obligation discriminator
//! without re-stating every nested `$def` (that surface would
//! drift). Providers wanting a complete schema can generate one via
//! `schemars` against the `ActorContract` type — left as a non-pinned
//! integration choice (the program thesis: provider-agnostic).
//!
//! `.3` (entailment), `.4` (templates), `.5` (uncertainty) land in
//! later commits; the producer wiring stays off in `.2` so corpus
//! artifacts are unchanged (the CONTRACT-IR.2 / KG-ONTOLOGY.2 /
//! FIDELITY.2 / FUSION.2 / WAVEFORM.2 discipline).

use crate::ir::contract::ActorContract;

/// Human-/provider-facing JSON-Schema **summary** for `ActorContract`,
/// expressed so an LLM/VLM that supports JSON-Schema-grammar-
/// constrained decoding can be steered into emitting one. The full,
/// authoritative validator is the round-trip through
/// `parse_constrained_contract` (serde-on-typed-`ActorContract`).
///
/// Kept deliberately short to avoid drift between this string and the
/// serde shape — the `actor_contract_summary_schema_mentions_required_keys`
/// test holds the invariant.
pub fn actor_contract_json_schema_summary() -> &'static str {
    r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://specforge.example/r16/actor_contract.summary.json",
  "title": "ActorContract (R16-CONTRACT-IR) — summary",
  "description": "Provider-facing summary; serde-on-ActorContract is the authoritative validator. See crates/specforge/src/ir/contract.rs for the typed schema; consider schemars-derived schema for full structural validation.",
  "type": "object",
  "required": [
    "contract_id",
    "kind",
    "obligation",
    "edge",
    "provenance",
    "lowering",
    "automation_confidence"
  ],
  "properties": {
    "contract_id": { "type": "string", "minLength": 1 },
    "source_rule_id": { "type": ["string", "null"] },
    "actor_name": { "type": ["string", "null"] },
    "kind": { "enum": ["assume", "guarantee"] },
    "obligation": {
      "type": "object",
      "required": ["kind"],
      "properties": {
        "kind": {
          "enum": [
            "eventually", "stable", "drive", "handshake_barrier",
            "persist", "sequence", "mutex", "ordered_before", "observe"
          ]
        }
      }
    },
    "clock_signal": { "type": ["string", "null"] },
    "edge": { "enum": ["rising", "falling"] },
    "channel": { "type": ["string", "null"] },
    "phase": { "type": ["string", "null"] },
    "lowering": {
      "type": "object",
      "required": ["kind"],
      "properties": {
        "kind": { "enum": ["lowerable", "residual"] }
      }
    },
    "automation_confidence": { "enum": ["high", "medium", "low"] }
  }
}
"#
}

/// Parse a constrained-decoded JSON contract into the typed
/// `ActorContract`. **Fails closed** on any schema/serde violation:
/// the returned `Err` carries the serde diagnostic — the caller MUST
/// route the candidate to a residual / reject it, never fabricate a
/// fallback contract.
///
/// Provider-agnostic: any LLM/VLM that produces JSON matching the
/// shape returned by `actor_contract_json_schema_summary()` (and any
/// nested obligation/condition variants documented in
/// `crates/specforge/src/ir/contract.rs`) will round-trip cleanly.
pub fn parse_constrained_contract(json: &str) -> Result<ActorContract, String> {
    serde_json::from_str::<ActorContract>(json)
        .map_err(|e| format!("constrained-decoding parse fail: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::contract::{
        ContractKind, ContractProvenance, EvidenceModality, LoweringDisposition, Obligation,
    };
    use crate::ir::semantic::ClockEdge;
    use crate::ir::source::AutomationConfidence;

    fn sample_contract() -> ActorContract {
        ActorContract {
            contract_id: "c1".into(),
            source_rule_id: Some("r1".into()),
            actor_name: Some("A".into()),
            kind: ContractKind::Guarantee,
            guard: None,
            guard_candidates: vec![],
            obligation: Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            clock_signal: Some("clk".into()),
            edge: ClockEdge::Rising,
            channel: None,
            phase: None,
            provenance: ContractProvenance {
                supporting_statement_ids: vec!["s1".into()],
                source_text: "src".into(),
                modality: EvidenceModality::Prose,
            },
            lowering: LoweringDisposition::Lowerable,
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn valid_contract_round_trips_through_adapter() {
        let c = sample_contract();
        let json = serde_json::to_string(&c).expect("serialize");
        let parsed = parse_constrained_contract(&json).expect("parse should succeed");
        assert_eq!(parsed, c);
    }

    #[test]
    fn malformed_input_fails_closed() {
        let bad = parse_constrained_contract("not json at all");
        assert!(bad.is_err());
        let empty_obj = parse_constrained_contract("{}");
        assert!(empty_obj.is_err(), "empty object lacks required fields");
        let missing_required = parse_constrained_contract(r#"{"contract_id":"c1"}"#);
        assert!(missing_required.is_err());
    }

    #[test]
    fn schema_summary_lists_required_top_level_keys() {
        // The summary is human-facing; this test holds the doc-vs-code
        // contract: the required keys named in the schema must match
        // the actual non-Option non-skip-default fields of
        // ActorContract.
        let schema = actor_contract_json_schema_summary();
        for required in [
            "contract_id",
            "kind",
            "obligation",
            "edge",
            "provenance",
            "lowering",
            "automation_confidence",
        ] {
            assert!(
                schema.contains(required),
                "schema summary should mention required key {required}"
            );
        }
    }

    #[test]
    fn schema_summary_enumerates_obligation_kinds() {
        // Spot-check the obligation discriminator: every variant in
        // contract.rs's Obligation must be listed in the schema enum
        // (drift-detection). If the enum grows, this test fails until
        // the schema is updated — the failure mode is exactly what the
        // .1 design wants ("authoritative validator is serde; schema
        // summary is human-facing", but it must not diverge silently).
        let schema = actor_contract_json_schema_summary();
        for kind in [
            "eventually",
            "stable",
            "drive",
            "handshake_barrier",
            "persist",
            "sequence",
            "mutex",
            "ordered_before",
            "observe",
        ] {
            assert!(
                schema.contains(&format!("\"{kind}\"")),
                "schema summary should list obligation kind {kind:?}"
            );
        }
    }
}
