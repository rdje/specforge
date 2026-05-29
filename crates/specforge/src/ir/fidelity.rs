//! Objective capture-fidelity gates (R16-CAPTURE-FIDELITY-GATES).
//!
//! Per the `R16-CAPTURE-FIDELITY-GATES.1` design (see
//! `docs/tasks/R16-CAPTURE-FIDELITY-GATES.md`): a typed set of gates
//! over `ActorContract` that produces a per-document fidelity score and
//! routes failures to residual. The doctrine — *unverifiable temporal
//! intent → explicit residual, never fabricated* — becomes
//! **mechanically enforced** in `.3`, not only authorial.
//!
//! `.2` scope (this module): the typed model + per-gate evaluators over
//! `actor_contracts` + the figure-trace replay primitive + unit tests.
//! The producer that runs the evaluators in `SemanticIr::build` and
//! routes `Fail` on `Lowerable` to `Residual` is `.3`. The corpus
//! `validate` report is `.4`. The `fidelity_findings` field is added
//! additively to `SemanticIr`/`IntentIr` and is **empty until `.3`**
//! (serde-skipped while empty ⇒ zero artifact churn — the
//! `R16-CONTRACT-IR.2` / `R16-KG-PROTOCOL-ONTOLOGY.2` discipline).
//!
//! Honest three-valued status: `NotEvaluated` is **never** silently
//! treated as `Pass`. The per-document score excludes `NotEvaluated`
//! from its denominator and counts it separately.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::ir::contract::{
    ActorContract, Condition, ContractKind, EventExpr, LoweringDisposition, Obligation,
    SequenceStep, Window,
};

/// The typed fidelity gate set (see the `.1` design).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FidelityGate {
    /// Every signal referenced by the contract (guard / obligation /
    /// clock) is declared on the actor's boundary.
    RealizableBoundary,
    /// The primary obligation signal direction is consistent with
    /// `ContractKind` (Assume↔input, Guarantee↔output).
    RealizableDirection,
    /// `HandshakeBarrier`: `ready ∈ inputs ∧ valid ∈ outputs`.
    RealizableHandshake,
    /// `Residual{reason}` must carry a non-empty reason; a `Lowerable`
    /// contract must not bear a shape with no `.isf` form (e.g.,
    /// `Observe`).
    ResidualHonesty,
    /// The contract, when lowered, would not produce FSMGen-strict-
    /// invalid syntax (bounded: `Lowerable` + (`Observe` |
    /// `OrderedBefore`) ⇒ `Fail`).
    NoStrictInvalid,
    /// A figure-derived trace assigned to this contract satisfies it.
    /// `NotEvaluated` corpus-wide until `R16-WAVEFORM-CONTRACT-MINING`
    /// (#4) populates `FigureTrace`s from PDF figures.
    FigureConformance,
}

/// Honest three-valued result. `NotEvaluated` is **not** `Pass`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FindingStatus {
    Pass,
    Fail,
    NotEvaluated,
}

/// One gate evaluation against one contract.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FidelityFinding {
    pub gate: FidelityGate,
    pub status: FindingStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    pub message: String,
}

/// A cycle-accurate sample sequence: one `u64` per signal per tick.
/// Recovery from PDF figures is the extraction trees' job
/// (`R16-WAVEFORM-CONTRACT-MINING` / #4); this module ships the
/// primitive a producer can drive once traces exist.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FigureTrace {
    pub signals: BTreeMap<String, Vec<u64>>,
    pub ticks: u32,
}

// ---------- signal-extraction helpers (referenced by RealizableBoundary)

fn event_expr_signals(e: &EventExpr) -> BTreeSet<String> {
    let mut s = BTreeSet::new();
    match e {
        EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => {
            s.insert(signal.clone());
        }
        EventExpr::HandshakeFire { valid, ready } => {
            s.insert(valid.clone());
            s.insert(ready.clone());
        }
        EventExpr::Start | EventExpr::PhaseBoundary { .. } => {}
    }
    s
}

fn window_signals(w: &Window) -> BTreeSet<String> {
    match w {
        Window::SameCycle | Window::Within { .. } => BTreeSet::new(),
        Window::Between { from, to } => {
            let mut s = event_expr_signals(from);
            s.extend(event_expr_signals(to));
            s
        }
    }
}

fn obligation_signals(o: &Obligation) -> BTreeSet<String> {
    let mut s = BTreeSet::new();
    match o {
        Obligation::Eventually { target, window } => {
            s.extend(event_expr_signals(target));
            s.extend(window_signals(window));
        }
        Obligation::Stable { signal, during } => {
            s.insert(signal.clone());
            s.extend(window_signals(during));
        }
        Obligation::Drive { signal, .. } => {
            s.insert(signal.clone());
        }
        Obligation::HandshakeBarrier { valid, ready } => {
            s.insert(valid.clone());
            s.insert(ready.clone());
        }
        Obligation::Persist { hold, until } => {
            s.extend(event_expr_signals(hold));
            s.extend(event_expr_signals(until));
        }
        Obligation::Sequence { steps } => {
            for SequenceStep { event, window } in steps {
                s.extend(event_expr_signals(event));
                s.extend(window_signals(window));
            }
        }
        Obligation::Mutex { a, b } => {
            s.insert(a.clone());
            s.insert(b.clone());
        }
        Obligation::OrderedBefore { .. } => {}
        Obligation::Observe { signal } => {
            s.insert(signal.clone());
        }
    }
    s
}

fn contract_signals(c: &ActorContract) -> BTreeSet<String> {
    let mut s = obligation_signals(&c.obligation);
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

/// The "subject" signal of an obligation, used by `RealizableDirection`.
/// `None` ⇒ no single primary signal (e.g., `OrderedBefore` over phases).
fn obligation_primary_signal(o: &Obligation) -> Option<&str> {
    match o {
        Obligation::Eventually { target, .. } => match target {
            EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => Some(signal),
            EventExpr::HandshakeFire { valid, .. } => Some(valid),
            EventExpr::Start | EventExpr::PhaseBoundary { .. } => None,
        },
        Obligation::Stable { signal, .. } => Some(signal),
        Obligation::Drive { signal, .. } => Some(signal),
        Obligation::HandshakeBarrier { valid, .. } => Some(valid),
        Obligation::Persist { hold, .. } => match hold {
            EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => Some(signal),
            EventExpr::HandshakeFire { valid, .. } => Some(valid),
            EventExpr::Start | EventExpr::PhaseBoundary { .. } => None,
        },
        Obligation::Sequence { steps } => steps.first().and_then(|s| match &s.event {
            EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => {
                Some(signal.as_str())
            }
            EventExpr::HandshakeFire { valid, .. } => Some(valid.as_str()),
            EventExpr::Start | EventExpr::PhaseBoundary { .. } => None,
        }),
        Obligation::Mutex { a, .. } => Some(a),
        Obligation::OrderedBefore { .. } => None,
        Obligation::Observe { signal } => Some(signal),
    }
}

// ---------- per-gate evaluators

fn finding(
    gate: FidelityGate,
    status: FindingStatus,
    c: &ActorContract,
    msg: String,
) -> FidelityFinding {
    FidelityFinding {
        gate,
        status,
        contract_id: Some(c.contract_id.clone()),
        message: msg,
    }
}

pub fn evaluate_realizable_boundary(
    c: &ActorContract,
    declared: &BTreeSet<String>,
) -> FidelityFinding {
    let refs = contract_signals(c);
    let missing: Vec<String> = refs.difference(declared).cloned().collect();
    if missing.is_empty() {
        finding(
            FidelityGate::RealizableBoundary,
            FindingStatus::Pass,
            c,
            String::new(),
        )
    } else {
        finding(
            FidelityGate::RealizableBoundary,
            FindingStatus::Fail,
            c,
            format!("signals not on actor boundary: {missing:?}"),
        )
    }
}

pub fn evaluate_realizable_direction(
    c: &ActorContract,
    inputs: &BTreeSet<String>,
    outputs: &BTreeSet<String>,
) -> FidelityFinding {
    let Some(signal) = obligation_primary_signal(&c.obligation) else {
        return finding(
            FidelityGate::RealizableDirection,
            FindingStatus::NotEvaluated,
            c,
            "obligation has no primary signal".into(),
        );
    };
    let in_inputs = inputs.contains(signal);
    let in_outputs = outputs.contains(signal);
    if !in_inputs && !in_outputs {
        return finding(
            FidelityGate::RealizableDirection,
            FindingStatus::NotEvaluated,
            c,
            format!("direction of '{signal}' is unknown to this actor"),
        );
    }
    let ok = match c.kind {
        ContractKind::Assume => in_inputs,
        ContractKind::Guarantee => in_outputs,
    };
    if ok {
        finding(
            FidelityGate::RealizableDirection,
            FindingStatus::Pass,
            c,
            String::new(),
        )
    } else {
        finding(
            FidelityGate::RealizableDirection,
            FindingStatus::Fail,
            c,
            format!(
                "{:?} on '{signal}' contradicts its direction (input={in_inputs}, output={in_outputs})",
                c.kind
            ),
        )
    }
}

pub fn evaluate_realizable_handshake(
    c: &ActorContract,
    inputs: &BTreeSet<String>,
    outputs: &BTreeSet<String>,
) -> FidelityFinding {
    let Obligation::HandshakeBarrier { valid, ready } = &c.obligation else {
        return finding(
            FidelityGate::RealizableHandshake,
            FindingStatus::NotEvaluated,
            c,
            "not a HandshakeBarrier".into(),
        );
    };
    let ready_ok = inputs.contains(ready);
    let valid_ok = outputs.contains(valid);
    if ready_ok && valid_ok {
        finding(
            FidelityGate::RealizableHandshake,
            FindingStatus::Pass,
            c,
            String::new(),
        )
    } else {
        finding(
            FidelityGate::RealizableHandshake,
            FindingStatus::Fail,
            c,
            format!(
                "handshake direction mismatch: ready='{ready}' input={ready_ok}, valid='{valid}' output={valid_ok}"
            ),
        )
    }
}

pub fn evaluate_residual_honesty(c: &ActorContract) -> FidelityFinding {
    match (&c.lowering, &c.obligation) {
        (LoweringDisposition::Residual { reason }, _) if reason.trim().is_empty() => finding(
            FidelityGate::ResidualHonesty,
            FindingStatus::Fail,
            c,
            "Residual disposition has empty reason".into(),
        ),
        (LoweringDisposition::Lowerable, Obligation::Observe { signal }) => finding(
            FidelityGate::ResidualHonesty,
            FindingStatus::Fail,
            c,
            format!("Observe('{signal}') has no .isf form — must be Residual"),
        ),
        _ => finding(
            FidelityGate::ResidualHonesty,
            FindingStatus::Pass,
            c,
            String::new(),
        ),
    }
}

pub fn evaluate_no_strict_invalid(c: &ActorContract) -> FidelityFinding {
    match (&c.lowering, &c.obligation) {
        (LoweringDisposition::Residual { .. }, _) => finding(
            FidelityGate::NoStrictInvalid,
            FindingStatus::NotEvaluated,
            c,
            "not lowered".into(),
        ),
        (LoweringDisposition::Lowerable, Obligation::Observe { .. })
        | (LoweringDisposition::Lowerable, Obligation::OrderedBefore { .. }) => finding(
            FidelityGate::NoStrictInvalid,
            FindingStatus::Fail,
            c,
            "obligation shape has no FSMGen-strict-valid .isf form".into(),
        ),
        _ => finding(
            FidelityGate::NoStrictInvalid,
            FindingStatus::Pass,
            c,
            String::new(),
        ),
    }
}

pub fn evaluate_figure_conformance(
    c: &ActorContract,
    trace: Option<&FigureTrace>,
) -> FidelityFinding {
    let Some(trace) = trace else {
        return finding(
            FidelityGate::FigureConformance,
            FindingStatus::NotEvaluated,
            c,
            "no FigureTrace assigned".into(),
        );
    };
    match evaluate_figure_trace(c, trace) {
        FindingStatus::Pass => finding(
            FidelityGate::FigureConformance,
            FindingStatus::Pass,
            c,
            String::new(),
        ),
        FindingStatus::Fail => finding(
            FidelityGate::FigureConformance,
            FindingStatus::Fail,
            c,
            "FigureTrace does not satisfy obligation".into(),
        ),
        FindingStatus::NotEvaluated => finding(
            FidelityGate::FigureConformance,
            FindingStatus::NotEvaluated,
            c,
            "obligation shape unsupported by trace primitive (.2 scope)".into(),
        ),
    }
}

/// Bounded structural trace-replay primitive (`.2` scope: `Stable` and
/// `Drive` only; other obligations return `NotEvaluated`). Honest
/// dormancy: missing signal in trace ⇒ `NotEvaluated`, not `Pass`.
pub fn evaluate_figure_trace(c: &ActorContract, trace: &FigureTrace) -> FindingStatus {
    let ticks = trace.ticks as usize;
    match &c.obligation {
        Obligation::Stable { signal, during } => {
            let Some(samples) = trace.signals.get(signal) else {
                return FindingStatus::NotEvaluated;
            };
            if samples.is_empty() {
                return FindingStatus::NotEvaluated;
            }
            let end = match during {
                Window::SameCycle => 1usize,
                Window::Within { max, .. } => (*max as usize).min(ticks).min(samples.len()),
                Window::Between { .. } => return FindingStatus::NotEvaluated,
            };
            let first = samples[0];
            if samples[..end].iter().all(|v| *v == first) {
                FindingStatus::Pass
            } else {
                FindingStatus::Fail
            }
        }
        Obligation::Drive { signal, value } => {
            let Some(samples) = trace.signals.get(signal) else {
                return FindingStatus::NotEvaluated;
            };
            let Ok(want) = value.parse::<u64>() else {
                return FindingStatus::NotEvaluated;
            };
            if samples.contains(&want) {
                FindingStatus::Pass
            } else {
                FindingStatus::Fail
            }
        }
        _ => FindingStatus::NotEvaluated,
    }
}

/// Per-document fidelity score = `pass / (pass + fail)` over
/// **evaluated** gates (`NotEvaluated` excluded from the denominator
/// and counted separately). Returns `None` when there are no evaluated
/// gates (no honest score is defined). Default threshold check is in
/// `score_meets_threshold`.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FidelitySummary {
    pub pass: u32,
    pub fail: u32,
    pub not_evaluated: u32,
}

impl FidelitySummary {
    pub fn from_findings(fs: &[FidelityFinding]) -> Self {
        let mut s = Self::default();
        for f in fs {
            match f.status {
                FindingStatus::Pass => s.pass += 1,
                FindingStatus::Fail => s.fail += 1,
                FindingStatus::NotEvaluated => s.not_evaluated += 1,
            }
        }
        s
    }

    /// `pass / (pass + fail)` over evaluated gates; `None` if none.
    pub fn score(&self) -> Option<f64> {
        let evaluated = self.pass + self.fail;
        if evaluated == 0 {
            None
        } else {
            Some(self.pass as f64 / evaluated as f64)
        }
    }

    /// Default discipline: a document meets `threshold` iff
    /// `score ≥ threshold` AND `fail == 0`. Threshold `1.0` ⇒ any
    /// `Fail` = below-threshold (the honest default).
    pub fn meets_threshold(&self, threshold: f64) -> bool {
        match self.score() {
            Some(s) => s >= threshold && self.fail == 0,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::contract::{ContractProvenance, EvidenceModality};
    use crate::ir::semantic::ClockEdge;
    use crate::ir::source::AutomationConfidence;

    fn declared(s: &[&str]) -> BTreeSet<String> {
        s.iter().map(|x| (*x).to_string()).collect()
    }

    fn lowerable_contract(id: &str, obligation: Obligation, kind: ContractKind) -> ActorContract {
        ActorContract {
            contract_id: id.into(),
            source_rule_id: Some(id.into()),
            actor_name: None,
            kind,
            guard: None,
            guard_candidates: vec![],
            obligation,
            clock_signal: Some("clk".into()),
            edge: ClockEdge::Rising,
            channel: None,
            phase: None,
            provenance: ContractProvenance {
                supporting_statement_ids: vec![],
                source_text: "src".into(),
                modality: EvidenceModality::Prose,
            },
            lowering: LoweringDisposition::Lowerable,
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn realizable_boundary_pass_and_fail() {
        let c = lowerable_contract(
            "c1",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        let pass = evaluate_realizable_boundary(&c, &declared(&["clk", "Q"]));
        assert_eq!(pass.status, FindingStatus::Pass);

        let fail = evaluate_realizable_boundary(&c, &declared(&["clk"]));
        assert_eq!(fail.status, FindingStatus::Fail);
        assert!(fail.message.contains("\"Q\""), "{}", fail.message);
    }

    #[test]
    fn realizable_direction_assume_input_guarantee_output_unknown_not_evaluated() {
        let g = lowerable_contract(
            "g",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        assert_eq!(
            evaluate_realizable_direction(&g, &declared(&[]), &declared(&["Q"])).status,
            FindingStatus::Pass,
        );
        // Guarantee on an input is a direction violation.
        assert_eq!(
            evaluate_realizable_direction(&g, &declared(&["Q"]), &declared(&[])).status,
            FindingStatus::Fail,
        );
        // Unknown direction ⇒ NotEvaluated (never silently Pass).
        assert_eq!(
            evaluate_realizable_direction(&g, &declared(&[]), &declared(&[])).status,
            FindingStatus::NotEvaluated,
        );

        let a = lowerable_contract(
            "a",
            Obligation::Stable {
                signal: "D".into(),
                during: Window::SameCycle,
            },
            ContractKind::Assume,
        );
        assert_eq!(
            evaluate_realizable_direction(&a, &declared(&["D"]), &declared(&[])).status,
            FindingStatus::Pass,
        );
    }

    #[test]
    fn realizable_handshake_matches_contract_ir_4_gate() {
        let hs = lowerable_contract(
            "hs",
            Obligation::HandshakeBarrier {
                valid: "AWVALID".into(),
                ready: "AWREADY".into(),
            },
            ContractKind::Guarantee,
        );
        assert_eq!(
            evaluate_realizable_handshake(&hs, &declared(&["AWREADY"]), &declared(&["AWVALID"]))
                .status,
            FindingStatus::Pass,
        );
        // ready not in inputs ⇒ Fail (FSMGen-strict would reject).
        assert_eq!(
            evaluate_realizable_handshake(&hs, &declared(&[]), &declared(&["AWVALID"])).status,
            FindingStatus::Fail,
        );
        // Not a HandshakeBarrier ⇒ NotEvaluated.
        let nb = lowerable_contract(
            "nb",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        assert_eq!(
            evaluate_realizable_handshake(&nb, &declared(&[]), &declared(&[])).status,
            FindingStatus::NotEvaluated,
        );
    }

    #[test]
    fn residual_honesty_catches_empty_reason_and_observe_lowerable() {
        let mut c = lowerable_contract(
            "r",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        c.lowering = LoweringDisposition::Residual {
            reason: "   ".into(),
        };
        assert_eq!(evaluate_residual_honesty(&c).status, FindingStatus::Fail);

        let observe = lowerable_contract(
            "o",
            Obligation::Observe { signal: "X".into() },
            ContractKind::Guarantee,
        );
        assert_eq!(
            evaluate_residual_honesty(&observe).status,
            FindingStatus::Fail
        );

        let ok = lowerable_contract(
            "ok",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        assert_eq!(evaluate_residual_honesty(&ok).status, FindingStatus::Pass);
    }

    #[test]
    fn no_strict_invalid_observe_lowerable_fails_residual_not_evaluated() {
        let observe_lowerable = lowerable_contract(
            "o",
            Obligation::Observe { signal: "X".into() },
            ContractKind::Guarantee,
        );
        assert_eq!(
            evaluate_no_strict_invalid(&observe_lowerable).status,
            FindingStatus::Fail,
        );
        let mut residual = observe_lowerable.clone();
        residual.lowering = LoweringDisposition::Residual {
            reason: "no .isf form".into(),
        };
        assert_eq!(
            evaluate_no_strict_invalid(&residual).status,
            FindingStatus::NotEvaluated,
        );
        let ok = lowerable_contract(
            "ok",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        assert_eq!(evaluate_no_strict_invalid(&ok).status, FindingStatus::Pass);
    }

    #[test]
    fn figure_conformance_not_evaluated_without_trace() {
        let c = lowerable_contract(
            "c",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        assert_eq!(
            evaluate_figure_conformance(&c, None).status,
            FindingStatus::NotEvaluated,
        );
    }

    #[test]
    fn figure_conformance_with_trace_returns_pass_fail_and_unsupported() {
        // The trace=Some glue paths (Pass / Fail / obligation-unsupported
        // NotEvaluated), distinct from the trace=None path above.
        let stable_contract = lowerable_contract(
            "s",
            Obligation::Stable {
                signal: "D".into(),
                during: Window::Within { min: None, max: 3 },
            },
            ContractKind::Assume,
        );
        let stable_trace = FigureTrace {
            signals: BTreeMap::from([("D".into(), vec![5, 5, 5, 5])]),
            ticks: 4,
        };
        let pass = evaluate_figure_conformance(&stable_contract, Some(&stable_trace));
        assert_eq!(pass.gate, FidelityGate::FigureConformance);
        assert_eq!(pass.status, FindingStatus::Pass);

        let wobbly_trace = FigureTrace {
            signals: BTreeMap::from([("D".into(), vec![5, 5, 6, 5])]),
            ticks: 4,
        };
        let fail = evaluate_figure_conformance(&stable_contract, Some(&wobbly_trace));
        assert_eq!(fail.status, FindingStatus::Fail);
        assert!(
            !fail.message.is_empty(),
            "a Fail finding must carry a diagnostic message"
        );

        // An obligation the trace primitive does not support must stay
        // NotEvaluated (never silently Pass), even with a trace present.
        let observe_contract = lowerable_contract(
            "obs",
            Obligation::Observe { signal: "X".into() },
            ContractKind::Guarantee,
        );
        let any_trace = FigureTrace {
            signals: BTreeMap::from([("X".into(), vec![1, 1])]),
            ticks: 2,
        };
        assert_eq!(
            evaluate_figure_conformance(&observe_contract, Some(&any_trace)).status,
            FindingStatus::NotEvaluated,
        );
    }

    #[test]
    fn figure_trace_drive_pass_fail_not_evaluated() {
        let drive = lowerable_contract(
            "d",
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
        );
        let t_ok = FigureTrace {
            signals: BTreeMap::from([("Q".into(), vec![0, 0, 1])]),
            ticks: 3,
        };
        assert_eq!(evaluate_figure_trace(&drive, &t_ok), FindingStatus::Pass);

        let t_no = FigureTrace {
            signals: BTreeMap::from([("Q".into(), vec![0, 0, 0])]),
            ticks: 3,
        };
        assert_eq!(evaluate_figure_trace(&drive, &t_no), FindingStatus::Fail);

        let t_missing = FigureTrace::default();
        assert_eq!(
            evaluate_figure_trace(&drive, &t_missing),
            FindingStatus::NotEvaluated
        );
    }

    #[test]
    fn figure_trace_stable_within_window() {
        let c = lowerable_contract(
            "s",
            Obligation::Stable {
                signal: "D".into(),
                during: Window::Within { min: None, max: 3 },
            },
            ContractKind::Assume,
        );
        let stable = FigureTrace {
            signals: BTreeMap::from([("D".into(), vec![5, 5, 5, 5])]),
            ticks: 4,
        };
        assert_eq!(evaluate_figure_trace(&c, &stable), FindingStatus::Pass);
        let wobbly = FigureTrace {
            signals: BTreeMap::from([("D".into(), vec![5, 5, 6, 5])]),
            ticks: 4,
        };
        assert_eq!(evaluate_figure_trace(&c, &wobbly), FindingStatus::Fail);
    }

    #[test]
    fn summary_score_excludes_not_evaluated_and_threshold_default_is_strict() {
        let pass = FidelityFinding {
            gate: FidelityGate::ResidualHonesty,
            status: FindingStatus::Pass,
            contract_id: None,
            message: String::new(),
        };
        let fail = FidelityFinding {
            status: FindingStatus::Fail,
            ..pass.clone()
        };
        let ne = FidelityFinding {
            status: FindingStatus::NotEvaluated,
            ..pass.clone()
        };

        let mixed =
            FidelitySummary::from_findings(&[pass.clone(), pass.clone(), fail.clone(), ne.clone()]);
        assert_eq!(
            mixed,
            FidelitySummary {
                pass: 2,
                fail: 1,
                not_evaluated: 1
            }
        );
        assert!((mixed.score().unwrap() - (2.0 / 3.0)).abs() < 1e-9);
        // Threshold 1.0 default rejects any Fail even at perfect denominator.
        assert!(!mixed.meets_threshold(1.0));
        let perfect = FidelitySummary::from_findings(&[pass.clone(), pass.clone()]);
        assert!(perfect.meets_threshold(1.0));
        // No evaluated gates ⇒ no honest score ⇒ does not meet threshold.
        let only_ne = FidelitySummary::from_findings(&[ne]);
        assert!(only_ne.score().is_none());
        assert!(!only_ne.meets_threshold(1.0));
    }
}
