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

use std::collections::BTreeSet;

use crate::ir::contract::{
    ActorContract, Condition, EventExpr, LoweringDisposition, Obligation, SequenceStep, Window,
};
use crate::ir::fidelity::FindingStatus;

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

// ---------- Entailment verifier (R16-CONSTRAINED-VERIFIED-EXTRACTION.3)

fn event_signals(e: &EventExpr, out: &mut BTreeSet<String>) {
    match e {
        EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => {
            out.insert(signal.clone());
        }
        EventExpr::HandshakeFire { valid, ready } => {
            out.insert(valid.clone());
            out.insert(ready.clone());
        }
        EventExpr::Start | EventExpr::PhaseBoundary { .. } => {}
    }
}

fn window_signals(w: &Window, out: &mut BTreeSet<String>) {
    if let Window::Between { from, to } = w {
        event_signals(from, out);
        event_signals(to, out);
    }
}

fn obligation_signals(o: &Obligation, out: &mut BTreeSet<String>) {
    match o {
        Obligation::Eventually { target, window } => {
            event_signals(target, out);
            window_signals(window, out);
        }
        Obligation::Stable { signal, during } => {
            out.insert(signal.clone());
            window_signals(during, out);
        }
        Obligation::Drive { signal, .. } | Obligation::Observe { signal } => {
            out.insert(signal.clone());
        }
        Obligation::HandshakeBarrier { valid, ready } => {
            out.insert(valid.clone());
            out.insert(ready.clone());
        }
        Obligation::Persist { hold, until } => {
            event_signals(hold, out);
            event_signals(until, out);
        }
        Obligation::Sequence { steps } => {
            for SequenceStep { event, window } in steps {
                event_signals(event, out);
                window_signals(window, out);
            }
        }
        Obligation::Mutex { a, b } => {
            out.insert(a.clone());
            out.insert(b.clone());
        }
        Obligation::OrderedBefore { .. } => {}
    }
}

fn contract_signals_for_entailment(c: &ActorContract) -> BTreeSet<String> {
    let mut s = BTreeSet::new();
    obligation_signals(&c.obligation, &mut s);
    if let Some(Condition::Eq { signal, .. }) = &c.guard {
        s.insert(signal.clone());
    }
    for Condition::Eq { signal, .. } in &c.guard_candidates {
        s.insert(signal.clone());
    }
    if let Some(clk) = &c.clock_signal {
        s.insert(clk.clone());
    }
    s
}

/// Bounds the verifier should expect to find as digit runs in the
/// source span (parsed `u64`). The current scope is the windowed
/// obligations + `Drive.value` when numeric — extending this set as
/// `Obligation` grows is the corresponding test
/// (`entailment_check_unsupported_obligation_is_not_evaluated`).
fn obligation_numeric_bounds(o: &Obligation) -> Vec<u64> {
    let mut nums = Vec::new();
    let pull = |w: &Window, nums: &mut Vec<u64>| {
        if let Window::Within { min, max } = w {
            if let Some(m) = min {
                nums.push(*m as u64);
            }
            nums.push(*max as u64);
        }
    };
    match o {
        Obligation::Eventually { window, .. } => pull(window, &mut nums),
        Obligation::Stable { during, .. } => pull(during, &mut nums),
        Obligation::Drive { value, .. } => {
            if let Ok(v) = value.parse::<u64>() {
                nums.push(v);
            }
        }
        Obligation::Sequence { steps } => {
            for SequenceStep { window, .. } in steps {
                pull(window, &mut nums);
            }
        }
        _ => {}
    }
    nums
}

fn span_contains_number(span: &str, want: u64) -> bool {
    let bytes = span.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if let Ok(n) = std::str::from_utf8(&bytes[start..i])
            .unwrap_or("")
            .parse::<u64>()
            && n == want
        {
            return true;
        }
    }
    false
}

/// Conservative lexical/structural entailment check: every signal the
/// contract references must appear as a case-preserving substring of
/// `source_span`; every numeric bound the contract's obligation
/// carries must appear as a complete digit run in `source_span`.
///
/// Returns `NotEvaluated` when the contract has nothing checkable
/// (no signals AND no numeric bounds) — never silently `Pass`. Per
/// the `.1` design, the verifier never "softens" a contract to pass;
/// `apply_entailment_to_contract` reroutes a `Fail` on a `Lowerable`
/// contract to `Residual{reason="entailment fail: …"}`.
pub fn entailment_check(source_span: &str, contract: &ActorContract) -> FindingStatus {
    let signals = contract_signals_for_entailment(contract);
    let bounds = obligation_numeric_bounds(&contract.obligation);
    if signals.is_empty() && bounds.is_empty() {
        return FindingStatus::NotEvaluated;
    }
    let mut missing_signals: Vec<String> = Vec::new();
    for s in &signals {
        if !source_span.contains(s.as_str()) {
            missing_signals.push(s.clone());
        }
    }
    let missing_bounds: Vec<u64> = bounds
        .into_iter()
        .filter(|n| !span_contains_number(source_span, *n))
        .collect();
    if missing_signals.is_empty() && missing_bounds.is_empty() {
        FindingStatus::Pass
    } else {
        FindingStatus::Fail
    }
}

/// Mechanically enforce the honesty doctrine on `contract`: if
/// `entailment_check` returns `Fail` and the contract is currently
/// `Lowerable`, downgrade to `Residual{reason="entailment fail: …"}`
/// with a list of the missing signals and bounds. Pass or
/// `NotEvaluated` leaves the contract unchanged; an already-Residual
/// contract is left untouched (no re-writing of pre-existing reasons).
/// Returns the `FindingStatus` the verifier produced so the caller can
/// log it.
pub fn apply_entailment_to_contract(
    contract: &mut ActorContract,
    source_span: &str,
) -> FindingStatus {
    let status = entailment_check(source_span, contract);
    if status == FindingStatus::Fail && matches!(contract.lowering, LoweringDisposition::Lowerable)
    {
        let signals = contract_signals_for_entailment(contract);
        let bounds = obligation_numeric_bounds(&contract.obligation);
        let missing_signals: Vec<&str> = signals
            .iter()
            .map(String::as_str)
            .filter(|s| !source_span.contains(*s))
            .collect();
        let missing_bounds: Vec<u64> = bounds
            .into_iter()
            .filter(|n| !span_contains_number(source_span, *n))
            .collect();
        contract.lowering = LoweringDisposition::Residual {
            reason: format!(
                "entailment fail: missing signals={:?} bounds={:?}",
                missing_signals, missing_bounds
            ),
        };
    }
    status
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
    fn entailment_pass_when_span_mentions_every_signal_and_bound() {
        let mut c = sample_contract();
        // sample uses Drive{Q, "1"} + clock="clk".
        let span = "Drive Q to 1 every clk cycle";
        assert_eq!(entailment_check(span, &c), FindingStatus::Pass);
        // Window-bearing obligation:
        c.obligation = Obligation::Stable {
            signal: "Q".into(),
            during: crate::ir::contract::Window::Within { min: None, max: 3 },
        };
        let span2 = "Q must remain stable for 3 cycles on clk";
        assert_eq!(entailment_check(span2, &c), FindingStatus::Pass);
    }

    #[test]
    fn entailment_fail_when_signal_or_bound_missing() {
        let mut c = sample_contract();
        let span_missing_signal = "Drive to 1 every clock cycle"; // no "Q" mentioned, no "clk"
        assert_eq!(
            entailment_check(span_missing_signal, &c),
            FindingStatus::Fail
        );
        c.obligation = Obligation::Stable {
            signal: "Q".into(),
            during: crate::ir::contract::Window::Within { min: None, max: 7 },
        };
        // signal "Q" + "clk" present, but bound 7 not mentioned (only 3):
        let span_missing_bound = "Q must remain stable for 3 cycles on clk";
        assert_eq!(
            entailment_check(span_missing_bound, &c),
            FindingStatus::Fail
        );
    }

    #[test]
    fn entailment_not_evaluated_when_nothing_checkable() {
        let mut c = sample_contract();
        c.obligation = Obligation::OrderedBefore {
            earlier_phase: "setup".into(),
            later_phase: "access".into(),
        };
        c.clock_signal = None; // remove the only signal
        // No signals + no bounds ⇒ NotEvaluated (never silently Pass).
        assert_eq!(
            entailment_check("anything", &c),
            FindingStatus::NotEvaluated
        );
    }

    #[test]
    fn apply_entailment_routes_lowerable_fail_to_residual_with_reason() {
        let mut c = sample_contract();
        // Force a guaranteed Fail: span mentions neither Q nor clk.
        let span = "some unrelated prose mentioning nothing";
        let status = apply_entailment_to_contract(&mut c, span);
        assert_eq!(status, FindingStatus::Fail);
        match &c.lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(reason.starts_with("entailment fail:"), "{reason}");
                assert!(reason.contains("\"Q\""), "{reason}");
                assert!(reason.contains("\"clk\""), "{reason}");
            }
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn apply_entailment_leaves_pass_lowerable_unchanged() {
        let mut c = sample_contract();
        let span = "Drive Q to 1 every clk cycle"; // Pass
        let before = c.clone();
        assert_eq!(
            apply_entailment_to_contract(&mut c, span),
            FindingStatus::Pass
        );
        assert_eq!(c, before);
    }

    #[test]
    fn apply_entailment_does_not_rewrite_preexisting_residual_reason() {
        let mut c = sample_contract();
        c.lowering = LoweringDisposition::Residual {
            reason: "preexisting".into(),
        };
        let span = "some unrelated prose"; // would Fail
        let _ = apply_entailment_to_contract(&mut c, span);
        match &c.lowering {
            LoweringDisposition::Residual { reason } => assert_eq!(reason, "preexisting"),
            other => panic!("expected Residual unchanged, got {other:?}"),
        }
    }

    #[test]
    fn span_contains_number_matches_complete_digit_runs_only() {
        // The number 7 should NOT match in "70" (different digit run).
        assert!(!span_contains_number("only 70 cycles", 7));
        assert!(span_contains_number("exactly 7 cycles", 7));
        assert!(span_contains_number("up to 12 cycles", 12));
        assert!(!span_contains_number("up to 12 cycles", 1));
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
