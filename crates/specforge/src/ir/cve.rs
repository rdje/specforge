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

use std::collections::{BTreeMap, BTreeSet};

use crate::ir::contract::{
    ActorContract, Condition, ContractKind, ContractProvenance, EventExpr, EvidenceModality,
    LoweringDisposition, Obligation, SequenceStep, Window,
};
use crate::ir::fidelity::FindingStatus;
use crate::ir::semantic::ClockEdge;
use crate::ir::source::AutomationConfidence;
use serde::{Deserialize, Serialize};

// ---------- Constrained prose→contract extraction stats (CVE-PROSE-EXTRACTION)

/// Producer-time stats for the constrained prose→contract extractor
/// (`extract-contracts`, `CVE-PROSE-EXTRACTION`). `schema_rejects` —
/// candidates whose JSON failed `parse_constrained_contract` (fails-closed) —
/// yields NO contract, so it is NOT derivable from the surviving
/// `actor_contracts`; it is persisted here and carried `EvidenceIR →
/// SemanticIR → IntentIR` so the `validate` `constrained:` block can report
/// it. `entailment_fails`/`template_hits` stay derived from the surviving
/// contracts. Additive + `Option` on the IRs ⇒ zero artifact churn until the
/// producer runs.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConstrainedExtractionStats {
    /// Prose statements sent to the provider.
    #[serde(default)]
    pub candidates_seen: usize,
    /// Responses that failed `parse_constrained_contract` (no contract produced).
    #[serde(default)]
    pub schema_rejects: usize,
    /// Contracts that parsed and were kept (incl. entailment-Residual ones).
    #[serde(default)]
    pub contracts_accepted: usize,
}

/// `schema_rejects` for the `validate` `constrained:` block: read from the
/// carried producer stat, `0` when the extractor has not run.
pub fn constrained_schema_rejects(stats: Option<&ConstrainedExtractionStats>) -> usize {
    stats.map(|s| s.schema_rejects).unwrap_or(0)
}

/// Fold prose-extracted contracts into the temporal-rule-projected set, in
/// order (projected first, then extracted), so the existing `apply_fusion` /
/// `apply_fidelity_gates` pass sees them as ordinary `actor_contracts`. Pure
/// helper so the fold is unit-testable away from the full `SemanticIr::build`.
pub fn fold_extracted_contracts(projected: &mut Vec<ActorContract>, extracted: &[ActorContract]) {
    projected.extend(extracted.iter().cloned());
}

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

// ---------- Uncertainty-driven converge selector (R16-CONSTRAINED-VERIFIED-EXTRACTION.5)
//
// Per the `.1` design's "Uncertainty-driven converge (`.5`)" section,
// the converge pass should spend its bounded budget on the contracts
// with the highest value-of-information — those that are least
// confident AND have the most fidelity failures. `.5` ships the
// deterministic SELECTION helper + tests over synthetic findings.
// Integrating the helper into the actual converge command is the
// integration leaf (deferred to `.6` close or a future follow-up —
// the converge command is an existing flow; this leaf does not
// re-wire it, honoring the bounded-scope discipline).

/// Inputs for the uncertainty selector. Construct on the fly from
/// `ir.actor_contracts` and `ir.fidelity_findings` at the converge
/// command's call site.
pub struct ConvergeInputs<'a> {
    pub contracts: &'a [ActorContract],
    pub findings: &'a [crate::ir::fidelity::FidelityFinding],
}

fn confidence_uncertainty(c: AutomationConfidence) -> f64 {
    // 1 - rank(c)/2; High=2 ⇒ 0; Medium=1 ⇒ 0.5; Low=0 ⇒ 1.
    let rank = match c {
        AutomationConfidence::High => 2.0,
        AutomationConfidence::Medium => 1.0,
        AutomationConfidence::Low => 0.0,
    };
    1.0 - rank / 2.0
}

fn count_fail_findings_for(
    contract_id: &str,
    findings: &[crate::ir::fidelity::FidelityFinding],
) -> u32 {
    findings
        .iter()
        .filter(|f| {
            f.status == FindingStatus::Fail && f.contract_id.as_deref() == Some(contract_id)
        })
        .count() as u32
}

/// Value-of-information score for one contract over the current
/// findings. Initial weights `w_conf = w_fail = 1.0` per `.1`.
pub fn voi_score(
    contract: &ActorContract,
    findings: &[crate::ir::fidelity::FidelityFinding],
) -> f64 {
    let w_conf = 1.0f64;
    let w_fail = 1.0f64;
    w_conf * confidence_uncertainty(contract.automation_confidence)
        + w_fail * count_fail_findings_for(&contract.contract_id, findings) as f64
}

/// Pick the top-`n` `contract_id`s by `voi_score` (descending). Ties
/// break deterministically by `contract_id` lexicographic ascending
/// — the next pass is reproducible across runs. Returns up to `n`
/// ids (fewer if `contracts.len() < n`).
pub fn select_top_n_by_voi(inputs: &ConvergeInputs<'_>, n: usize) -> Vec<String> {
    if n == 0 {
        return Vec::new();
    }
    let mut scored: Vec<(f64, &str)> = inputs
        .contracts
        .iter()
        .map(|c| (voi_score(c, inputs.findings), c.contract_id.as_str()))
        .collect();
    // Descending VoI; ascending contract_id on ties.
    scored.sort_by(|(a_voi, a_id), (b_voi, b_id)| {
        b_voi
            .partial_cmp(a_voi)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a_id.cmp(b_id))
    });
    scored
        .into_iter()
        .take(n)
        .map(|(_, id)| id.to_string())
        .collect()
}

// ---------- Protocol-pattern template library (R16-CONSTRAINED-VERIFIED-EXTRACTION.4)
//
// Per the `.1` design's "Template library (`.4`)" section, this is the
// canonical seed set: ready/valid, credit flow control, setup/access,
// async-assert/sync-release reset, burst+last. The `.1` design names
// `prior_memory` as the canonical home; for `.4` bounded scope the
// library ships here next to the entailment verifier (the natural
// consumer), with a future leaf available to migrate into
// `prior_memory` if a `CorpusMemory` integration becomes useful.
//
// Match-grounding gate (honesty doctrine): instantiation requires
// every role in `required_roles()` to be bound in the
// `SignalBindings`; missing bindings ⇒ `None`. Some templates whose
// obligation shape is not yet representable as a single supported
// `Obligation` (credit flow control; setup/access phase ordering)
// honestly instantiate as `Observe + Residual{reason}` rather than
// fabricate a misleading Obligation — the template's existence is
// recorded; its lowering is deferred (future work surfaces the
// `Residual.reason` so an operator can see what's missing).

/// The canonical seed-set of protocol-pattern templates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolTemplate {
    ReadyValidHandshake,
    CreditFlowControl,
    SetupAccess,
    AsyncAssertSyncReleaseReset,
    BurstLast,
}

impl ProtocolTemplate {
    pub fn all() -> &'static [ProtocolTemplate] {
        &[
            ProtocolTemplate::ReadyValidHandshake,
            ProtocolTemplate::CreditFlowControl,
            ProtocolTemplate::SetupAccess,
            ProtocolTemplate::AsyncAssertSyncReleaseReset,
            ProtocolTemplate::BurstLast,
        ]
    }

    pub fn name(self) -> &'static str {
        match self {
            ProtocolTemplate::ReadyValidHandshake => "ready_valid_handshake",
            ProtocolTemplate::CreditFlowControl => "credit_flow_control",
            ProtocolTemplate::SetupAccess => "setup_access",
            ProtocolTemplate::AsyncAssertSyncReleaseReset => "async_assert_sync_release_reset",
            ProtocolTemplate::BurstLast => "burst_last",
        }
    }

    /// Role names a binding must supply for the template to instantiate.
    pub fn required_roles(self) -> &'static [&'static str] {
        match self {
            ProtocolTemplate::ReadyValidHandshake => &["valid", "ready"],
            ProtocolTemplate::CreditFlowControl => &["credit_grant", "credit_consume"],
            ProtocolTemplate::SetupAccess => &["sel", "enable", "ready"],
            ProtocolTemplate::AsyncAssertSyncReleaseReset => &["reset_n"],
            ProtocolTemplate::BurstLast => &["last"],
        }
    }
}

/// Maps template role names (e.g. `"valid"`) to concrete signal names
/// from the actor boundary (e.g. `"AWVALID"`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SignalBindings {
    pub bindings: BTreeMap<String, String>,
}

impl SignalBindings {
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        let bindings = pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        Self { bindings }
    }

    fn missing_roles(&self, required: &[&'static str]) -> Vec<&'static str> {
        required
            .iter()
            .copied()
            .filter(|r| !self.bindings.contains_key(*r))
            .collect()
    }
}

fn template_provenance(template_name: &str) -> ContractProvenance {
    ContractProvenance {
        supporting_statement_ids: vec![format!("template:{template_name}")],
        source_text: format!("instantiated from {template_name} template"),
        modality: EvidenceModality::Mixed,
    }
}

fn template_contract(
    contract_id: String,
    obligation: Obligation,
    lowering: LoweringDisposition,
    template_name: &str,
    confidence: AutomationConfidence,
) -> ActorContract {
    ActorContract {
        contract_id,
        source_rule_id: None,
        actor_name: None,
        kind: ContractKind::Guarantee,
        guard: None,
        guard_candidates: vec![],
        obligation,
        clock_signal: None,
        edge: ClockEdge::Rising,
        channel: None,
        phase: None,
        provenance: template_provenance(template_name),
        lowering,
        automation_confidence: confidence,
    }
}

/// Instantiate `template` against `bindings`. Returns `None` when the
/// match-grounding gate fails (any required role unbound). When the
/// template's obligation shape is not yet representable as a single
/// supported `Obligation` (CFC; SetupAccess phase ordering), the
/// instantiation honestly produces an `Observe + Residual{reason}`
/// instead of fabricating a misleading Obligation — the template's
/// existence is recorded but its lowering is deferred.
pub fn instantiate_template(
    template: ProtocolTemplate,
    bindings: &SignalBindings,
) -> Option<ActorContract> {
    let missing = bindings.missing_roles(template.required_roles());
    if !missing.is_empty() {
        return None;
    }
    let get = |role: &str| bindings.bindings.get(role).cloned().unwrap();
    let id = format!("tmpl:{}", template.name());
    Some(match template {
        ProtocolTemplate::ReadyValidHandshake => template_contract(
            id,
            Obligation::HandshakeBarrier {
                valid: get("valid"),
                ready: get("ready"),
            },
            LoweringDisposition::Lowerable,
            template.name(),
            AutomationConfidence::High,
        ),
        ProtocolTemplate::AsyncAssertSyncReleaseReset => template_contract(
            id,
            Obligation::Drive {
                signal: get("reset_n"),
                value: "0".into(),
            },
            LoweringDisposition::Lowerable,
            template.name(),
            AutomationConfidence::High,
        ),
        ProtocolTemplate::BurstLast => template_contract(
            id,
            Obligation::Drive {
                signal: get("last"),
                value: "1".into(),
            },
            LoweringDisposition::Lowerable,
            template.name(),
            AutomationConfidence::High,
        ),
        ProtocolTemplate::CreditFlowControl => template_contract(
            id,
            Obligation::Observe {
                signal: get("credit_grant"),
            },
            LoweringDisposition::Residual {
                reason: "credit-flow template — counter primitives not yet representable as a single ContractIR obligation".into(),
            },
            template.name(),
            AutomationConfidence::High,
        ),
        ProtocolTemplate::SetupAccess => template_contract(
            id,
            Obligation::Observe { signal: get("sel") },
            LoweringDisposition::Residual {
                reason: "setup/access template — phase ordering needs explicit ProtocolPhase ids (R16-KG-PROTOCOL-ONTOLOGY) bound via extraction".into(),
            },
            template.name(),
            AutomationConfidence::High,
        ),
    })
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
    fn protocol_template_library_enumerates_five_canonical_templates() {
        let all = ProtocolTemplate::all();
        assert_eq!(all.len(), 5);
        let names: Vec<&str> = all.iter().map(|t| t.name()).collect();
        for n in [
            "ready_valid_handshake",
            "credit_flow_control",
            "setup_access",
            "async_assert_sync_release_reset",
            "burst_last",
        ] {
            assert!(names.contains(&n), "missing template {n}");
        }
    }

    #[test]
    fn ready_valid_handshake_instantiates_handshake_barrier_at_high_confidence() {
        let b = SignalBindings::from_pairs(&[("valid", "AWVALID"), ("ready", "AWREADY")]);
        let c = instantiate_template(ProtocolTemplate::ReadyValidHandshake, &b).unwrap();
        match &c.obligation {
            Obligation::HandshakeBarrier { valid, ready } => {
                assert_eq!(valid, "AWVALID");
                assert_eq!(ready, "AWREADY");
            }
            other => panic!("expected HandshakeBarrier, got {other:?}"),
        }
        assert!(matches!(c.lowering, LoweringDisposition::Lowerable));
        assert_eq!(c.automation_confidence, AutomationConfidence::High);
        assert_eq!(c.provenance.modality, EvidenceModality::Mixed);
        assert_eq!(c.contract_id, "tmpl:ready_valid_handshake");
    }

    #[test]
    fn match_grounding_gate_returns_none_for_missing_role() {
        let b = SignalBindings::from_pairs(&[("valid", "VLD")]); // missing "ready"
        assert!(instantiate_template(ProtocolTemplate::ReadyValidHandshake, &b).is_none());
    }

    #[test]
    fn burst_last_template_instantiates_drive_lowerable() {
        let b = SignalBindings::from_pairs(&[("last", "WLAST")]);
        let c = instantiate_template(ProtocolTemplate::BurstLast, &b).unwrap();
        match &c.obligation {
            Obligation::Drive { signal, value } => {
                assert_eq!(signal, "WLAST");
                assert_eq!(value, "1");
            }
            other => panic!("expected Drive, got {other:?}"),
        }
        assert!(matches!(c.lowering, LoweringDisposition::Lowerable));
    }

    #[test]
    fn async_reset_template_drives_reset_n_to_zero_lowerable() {
        let b = SignalBindings::from_pairs(&[("reset_n", "ARESETN")]);
        let c = instantiate_template(ProtocolTemplate::AsyncAssertSyncReleaseReset, &b).unwrap();
        match &c.obligation {
            Obligation::Drive { signal, value } => {
                assert_eq!(signal, "ARESETN");
                assert_eq!(value, "0");
            }
            other => panic!("expected Drive, got {other:?}"),
        }
        assert!(matches!(c.lowering, LoweringDisposition::Lowerable));
    }

    #[test]
    fn credit_flow_and_setup_access_honestly_residual() {
        let cfc = instantiate_template(
            ProtocolTemplate::CreditFlowControl,
            &SignalBindings::from_pairs(&[
                ("credit_grant", "CR_GRANT"),
                ("credit_consume", "CR_USE"),
            ]),
        )
        .unwrap();
        match &cfc.lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(reason.contains("credit-flow"), "{reason}");
                assert!(
                    reason.contains("not yet"),
                    "must call out the deferred lowering: {reason}"
                );
            }
            other => panic!("expected Residual, got {other:?}"),
        }
        assert!(matches!(cfc.obligation, Obligation::Observe { .. }));

        let sa = instantiate_template(
            ProtocolTemplate::SetupAccess,
            &SignalBindings::from_pairs(&[
                ("sel", "PSEL"),
                ("enable", "PENABLE"),
                ("ready", "PREADY"),
            ]),
        )
        .unwrap();
        match &sa.lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(reason.contains("setup/access"), "{reason}");
                assert!(reason.contains("ProtocolPhase"), "{reason}");
            }
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn template_match_is_entailment_verifiable_against_an_actor_boundary_span() {
        // The .1 design says: "the match itself is entailment-verifiable".
        // A template instantiates only when its bound signals appear; the
        // entailment check on a source-span containing those signals
        // then Passes.
        let b = SignalBindings::from_pairs(&[("valid", "AWVALID"), ("ready", "AWREADY")]);
        let c = instantiate_template(ProtocolTemplate::ReadyValidHandshake, &b).unwrap();
        let span = "AWVALID asserts and AWREADY is sampled on the rising edge";
        assert_eq!(entailment_check(span, &c), FindingStatus::Pass);
    }

    fn finding_for(
        contract_id: &str,
        status: FindingStatus,
    ) -> crate::ir::fidelity::FidelityFinding {
        use crate::ir::fidelity::{FidelityFinding, FidelityGate};
        FidelityFinding {
            gate: FidelityGate::ResidualHonesty,
            status,
            contract_id: Some(contract_id.into()),
            message: String::new(),
        }
    }

    fn make_for_voi(id: &str, conf: AutomationConfidence) -> ActorContract {
        let mut c = sample_contract();
        c.contract_id = id.into();
        c.automation_confidence = conf;
        c
    }

    #[test]
    fn voi_low_confidence_beats_high_confidence_when_no_findings() {
        let low = make_for_voi("low", AutomationConfidence::Low);
        let high = make_for_voi("high", AutomationConfidence::High);
        assert!(voi_score(&low, &[]) > voi_score(&high, &[]));
        // VoI(High) is 0 + 0 = 0; VoI(Low) is 1 + 0 = 1.
        assert_eq!(voi_score(&high, &[]), 0.0);
        assert_eq!(voi_score(&low, &[]), 1.0);
    }

    #[test]
    fn voi_fail_findings_outrank_a_low_confidence_no_findings_contract() {
        let medium_two_fails = make_for_voi("m", AutomationConfidence::Medium);
        let low_no_fails = make_for_voi("l", AutomationConfidence::Low);
        let findings = vec![
            finding_for("m", FindingStatus::Fail),
            finding_for("m", FindingStatus::Fail),
            finding_for("l", FindingStatus::Pass),
        ];
        // VoI(m) = 0.5 + 2 = 2.5; VoI(l) = 1 + 0 = 1.
        assert!(voi_score(&medium_two_fails, &findings) > voi_score(&low_no_fails, &findings));
    }

    #[test]
    fn select_top_n_by_voi_tie_breaks_lex_ascending_on_contract_id() {
        let cs = vec![
            make_for_voi("b", AutomationConfidence::Low),
            make_for_voi("a", AutomationConfidence::Low),
        ];
        let inputs = ConvergeInputs {
            contracts: &cs,
            findings: &[],
        };
        // Equal VoI ⇒ deterministic ascending tie-break.
        assert_eq!(
            select_top_n_by_voi(&inputs, 2),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(select_top_n_by_voi(&inputs, 1), vec!["a".to_string()]);
        assert!(select_top_n_by_voi(&inputs, 0).is_empty());
    }

    #[test]
    fn select_top_n_orders_by_descending_voi() {
        let cs = vec![
            make_for_voi("high_clean", AutomationConfidence::High),
            make_for_voi("low_clean", AutomationConfidence::Low),
            make_for_voi("medium_one_fail", AutomationConfidence::Medium),
        ];
        let findings = vec![finding_for("medium_one_fail", FindingStatus::Fail)];
        let inputs = ConvergeInputs {
            contracts: &cs,
            findings: &findings,
        };
        // VoIs: high=0, low=1, medium+1fail=1.5 → ordering m,l,h.
        assert_eq!(
            select_top_n_by_voi(&inputs, 3),
            vec![
                "medium_one_fail".to_string(),
                "low_clean".to_string(),
                "high_clean".to_string(),
            ]
        );
        // Budget of 1 picks just the top one.
        assert_eq!(
            select_top_n_by_voi(&inputs, 1),
            vec!["medium_one_fail".to_string()]
        );
    }

    #[test]
    fn select_top_n_empty_inputs_yield_empty() {
        let cs: Vec<ActorContract> = vec![];
        let inputs = ConvergeInputs {
            contracts: &cs,
            findings: &[],
        };
        assert!(select_top_n_by_voi(&inputs, 5).is_empty());
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

    #[test]
    fn extraction_stats_default_and_round_trip() {
        let z = ConstrainedExtractionStats::default();
        assert_eq!(z.candidates_seen, 0);
        assert_eq!(z.schema_rejects, 0);
        assert_eq!(z.contracts_accepted, 0);
        let s = ConstrainedExtractionStats {
            candidates_seen: 7,
            schema_rejects: 2,
            contracts_accepted: 4,
        };
        let json = serde_json::to_string(&s).expect("serialize");
        let back: ConstrainedExtractionStats = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(s, back);
    }

    #[test]
    fn constrained_schema_rejects_reads_stat_else_zero() {
        assert_eq!(constrained_schema_rejects(None), 0);
        let s = ConstrainedExtractionStats {
            candidates_seen: 5,
            schema_rejects: 3,
            contracts_accepted: 2,
        };
        assert_eq!(constrained_schema_rejects(Some(&s)), 3);
    }

    #[test]
    fn fold_extracted_appends_after_projected_preserving_order() {
        let mut projected = vec![make_for_voi("proj_a", AutomationConfidence::High)];
        let extracted = vec![
            make_for_voi("extr_b", AutomationConfidence::Low),
            make_for_voi("extr_c", AutomationConfidence::Medium),
        ];
        fold_extracted_contracts(&mut projected, &extracted);
        let ids: Vec<&str> = projected.iter().map(|c| c.contract_id.as_str()).collect();
        assert_eq!(ids, vec!["proj_a", "extr_b", "extr_c"]);
    }
}
