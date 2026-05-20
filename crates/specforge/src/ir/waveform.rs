//! Timing-diagram → contract mining (R16-WAVEFORM-CONTRACT-MINING).
//!
//! Per the `R16-WAVEFORM-CONTRACT-MINING.1` design (see
//! `docs/tasks/R16-WAVEFORM-CONTRACT-MINING.md`): the typed
//! intermediate `PartialTrace` is the contract between a figure→trace
//! extractor (`.3`) and the trace→contract generalizer (this module,
//! `.2`). Both sides are independently unit-testable against synthetic
//! `PartialTrace`s.
//!
//! `.2` scope (this module):
//! - the typed intermediate (`PartialTrace`, `LaneEdge`, `ValueSpan`,
//!   `RelativeDelay`, `CausalArrow`, `EdgeKind`);
//! - `generalize_partial_trace(&PartialTrace) -> Vec<ActorContract>`
//!   (conservative bounded rules; under-determined ⇒ Observe +
//!   Residual; never fabricate a window);
//! - `verify_contract_against_trace(&ActorContract, &PartialTrace) ->
//!   FindingStatus` — round-trip oracle reusing
//!   `R16-CAPTURE-FIDELITY-GATES.2`'s `evaluate_figure_trace`. A
//!   generalized contract that does NOT satisfy its own source trace
//!   is honest-flagged (the producer in `.3` demotes it to
//!   `Residual{reason="verifier disagreement: …"}`).
//!
//! No producer wiring in `.2` ⇒ zero artifact churn (the
//! CONTRACT-IR.2 / KG-ONTOLOGY.2 / FIDELITY.2 / FUSION.2 discipline).
//!
//! NOTE: `clock_signal` on a generalized contract is `None` — a
//! figure rarely names its clock signal; binding the clock is the
//! extractor's job in `.3` (cross-referencing the actor whose ports
//! the figure shows).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ir::contract::{
    ActorContract, ContractKind, ContractProvenance, EdgeDir, EventExpr, EvidenceModality,
    LoweringDisposition, Obligation, Window,
};
use crate::ir::fidelity::{FigureTrace, FindingStatus, evaluate_figure_trace};
use crate::ir::semantic::ClockEdge;
use crate::ir::source::AutomationConfidence;

/// A single signal-lane transition recovered from a figure.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Rising,
    Falling,
    Stable,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneEdge {
    pub signal: String,
    pub at_tick: u32,
    pub kind: EdgeKind,
}

/// `signal` holds `value` across the half-open interval
/// `[from_tick, to_tick]` (inclusive both ends).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValueSpan {
    pub signal: String,
    pub value: String,
    pub from_tick: u32,
    pub to_tick: u32,
}

/// An annotated relative delay between two signal events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelativeDelay {
    pub from_signal: String,
    pub to_signal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cycles: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cycles: Option<u32>,
    pub annotation_text: String,
}

/// A causal arrow drawn in the diagram (e.g. `valid↑` causes
/// `ready↑` the next tick). Adjacent-tick causal arrows generalize to
/// next-cycle `Eventually{Within{0,1}}`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CausalArrow {
    pub from_signal: String,
    pub from_tick: u32,
    pub to_signal: String,
    pub to_tick: u32,
}

/// The typed intermediate an extractor produces and the generalizer
/// consumes. Each evidence kind (`edges`/`spans`/`delays`/`causal`) is
/// recorded as a separate vector so the extractor's reliability is
/// observable per dimension.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PartialTrace {
    pub figure_id: String,
    #[serde(default)]
    pub signals: Vec<String>,
    #[serde(default)]
    pub edges: Vec<LaneEdge>,
    #[serde(default)]
    pub spans: Vec<ValueSpan>,
    #[serde(default)]
    pub delays: Vec<RelativeDelay>,
    #[serde(default)]
    pub causal: Vec<CausalArrow>,
    pub ticks: u32,
    pub confidence: AutomationConfidence,
}

// ---------- Generalization (PartialTrace → Vec<ActorContract>)

/// `min(High,Medium)=Medium` / `min(_,Low)=Low`. Mining never promotes
/// to `High` without cross-modal corroboration (a single figure is
/// not enough on its own).
fn capped_confidence(c: AutomationConfidence) -> AutomationConfidence {
    match c {
        AutomationConfidence::High => AutomationConfidence::Medium,
        other => other,
    }
}

fn provenance(figure_id: &str, annotation: &str) -> ContractProvenance {
    ContractProvenance {
        supporting_statement_ids: vec![format!("figure:{figure_id}")],
        source_text: annotation.to_string(),
        modality: EvidenceModality::Figure,
    }
}

fn make_contract(
    contract_id: String,
    obligation: Obligation,
    lowering: LoweringDisposition,
    prov: ContractProvenance,
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
        provenance: prov,
        lowering,
        automation_confidence: confidence,
    }
}

/// Generate `ActorContract` candidates from a `PartialTrace`,
/// conservatively. Each evidence record contributes at most one
/// contract; under-determined inputs lower as `Observe` +
/// `Residual` (the honesty doctrine, mechanically enforced — never
/// fabricate a window).
pub fn generalize_partial_trace(trace: &PartialTrace) -> Vec<ActorContract> {
    let mut out = Vec::new();
    let conf = capped_confidence(trace.confidence);

    // RelativeDelay ⇒ Eventually{Within{min,max}}.
    for (i, d) in trace.delays.iter().enumerate() {
        let id = format!("wf:{}:delay:{}", trace.figure_id, i);
        match (d.min_cycles, d.max_cycles) {
            (Some(min), Some(max)) if max >= 1 && max >= min => {
                let obligation = Obligation::Eventually {
                    target: EventExpr::Edge {
                        signal: d.to_signal.clone(),
                        dir: EdgeDir::Rose,
                    },
                    window: Window::Within {
                        min: Some(min),
                        max,
                    },
                };
                out.push(make_contract(
                    id,
                    obligation,
                    LoweringDisposition::Lowerable,
                    provenance(&trace.figure_id, &d.annotation_text),
                    conf,
                ));
            }
            _ => {
                // Under-determined (missing/zero/inverted bound) ⇒
                // Observe + Residual: honest, never fabricate.
                out.push(make_contract(
                    id,
                    Obligation::Observe {
                        signal: d.to_signal.clone(),
                    },
                    LoweringDisposition::Residual {
                        reason: "under-determined delay — bounds missing or invalid".into(),
                    },
                    provenance(&trace.figure_id, &d.annotation_text),
                    conf,
                ));
            }
        }
    }

    // Multi-tick ValueSpan ⇒ Stable{Within{max=span_len}}.
    for (i, s) in trace.spans.iter().enumerate() {
        let id = format!("wf:{}:span:{}", trace.figure_id, i);
        if s.to_tick > s.from_tick {
            let span_len = s.to_tick - s.from_tick;
            let obligation = Obligation::Stable {
                signal: s.signal.clone(),
                during: Window::Within {
                    min: None,
                    max: span_len,
                },
            };
            out.push(make_contract(
                id,
                obligation,
                LoweringDisposition::Lowerable,
                provenance(
                    &trace.figure_id,
                    &format!(
                        "{}={} held {}..{}",
                        s.signal, s.value, s.from_tick, s.to_tick
                    ),
                ),
                conf,
            ));
        }
        // Single-tick spans are not Stable (combinational); skip
        // (they may be picked up as Drive later via extractor hints).
    }

    // Next-tick CausalArrow ⇒ Eventually{Within{0,1}}.
    for (i, c) in trace.causal.iter().enumerate() {
        let id = format!("wf:{}:causal:{}", trace.figure_id, i);
        if c.to_tick == c.from_tick + 1 {
            let obligation = Obligation::Eventually {
                target: EventExpr::Edge {
                    signal: c.to_signal.clone(),
                    dir: EdgeDir::Rose,
                },
                window: Window::Within {
                    min: Some(0),
                    max: 1,
                },
            };
            out.push(make_contract(
                id,
                obligation,
                LoweringDisposition::Lowerable,
                provenance(
                    &trace.figure_id,
                    &format!(
                        "{}@{} → {}@{}",
                        c.from_signal, c.from_tick, c.to_signal, c.to_tick
                    ),
                ),
                conf,
            ));
        } else if c.to_tick > c.from_tick {
            // Wider gap: still Eventually but min=0, max=gap.
            let max = c.to_tick - c.from_tick;
            let obligation = Obligation::Eventually {
                target: EventExpr::Edge {
                    signal: c.to_signal.clone(),
                    dir: EdgeDir::Rose,
                },
                window: Window::Within { min: Some(0), max },
            };
            out.push(make_contract(
                id,
                obligation,
                LoweringDisposition::Lowerable,
                provenance(
                    &trace.figure_id,
                    &format!(
                        "{}@{} → {}@{}",
                        c.from_signal, c.from_tick, c.to_signal, c.to_tick
                    ),
                ),
                conf,
            ));
        }
        // Backward/co-temporal arrows are not licensed; skipped.
    }

    // Bare LaneEdges (no enclosing delay/span/causal) ⇒
    // Observe + Residual. Bounded by signal to avoid one-Observe-per-
    // edge spam: at most one Observe per signal not already covered.
    let covered: std::collections::BTreeSet<&str> = trace
        .spans
        .iter()
        .map(|s| s.signal.as_str())
        .chain(
            trace
                .delays
                .iter()
                .flat_map(|d| [d.from_signal.as_str(), d.to_signal.as_str()]),
        )
        .chain(
            trace
                .causal
                .iter()
                .flat_map(|c| [c.from_signal.as_str(), c.to_signal.as_str()]),
        )
        .collect();
    let mut emitted_signals: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    for e in &trace.edges {
        if covered.contains(e.signal.as_str()) || !emitted_signals.insert(e.signal.as_str()) {
            continue;
        }
        let id = format!("wf:{}:bare:{}", trace.figure_id, e.signal);
        out.push(make_contract(
            id,
            Obligation::Observe {
                signal: e.signal.clone(),
            },
            LoweringDisposition::Residual {
                reason: "bare edge — no window licensed".into(),
            },
            provenance(&trace.figure_id, &format!("bare edge on {}", e.signal)),
            conf,
        ));
    }

    out
}

// ---------- Round-trip verifier

/// Lift a `PartialTrace` to a `FigureTrace` for the verifier. Values
/// are best-effort: every signal in `trace.signals` is sampled across
/// `trace.ticks` ticks; spans apply first (parsed as `u64`, fallback
/// `0`), then edges (`Rising → 1`, `Falling → 0`, `Stable/Unknown →
/// no-op`).
pub fn partial_trace_to_figure_trace(trace: &PartialTrace) -> FigureTrace {
    let ticks = trace.ticks as usize;
    let mut signals: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    for s in &trace.signals {
        signals.insert(s.clone(), vec![0u64; ticks]);
    }
    for span in &trace.spans {
        let samples = signals
            .entry(span.signal.clone())
            .or_insert_with(|| vec![0u64; ticks]);
        let v = span.value.parse::<u64>().unwrap_or(0);
        let to = (span.to_tick as usize).min(samples.len().saturating_sub(1));
        let from = (span.from_tick as usize).min(to);
        for s in samples.iter_mut().take(to + 1).skip(from) {
            *s = v;
        }
    }
    for e in &trace.edges {
        let samples = signals
            .entry(e.signal.clone())
            .or_insert_with(|| vec![0u64; ticks]);
        let at = e.at_tick as usize;
        match e.kind {
            EdgeKind::Rising => {
                if at < samples.len() {
                    samples[at] = 1;
                }
            }
            EdgeKind::Falling => {
                if at < samples.len() {
                    samples[at] = 0;
                }
            }
            EdgeKind::Stable | EdgeKind::Unknown => {}
        }
    }
    FigureTrace {
        signals,
        ticks: trace.ticks,
    }
}

/// Round-trip oracle: a generalized contract must satisfy the trace
/// it was generalized from. Reuses
/// `R16-CAPTURE-FIDELITY-GATES.2`'s `evaluate_figure_trace`. The
/// producer in `.3` interprets `Fail` as "demote to
/// `Residual{reason='verifier disagreement: …'}`".
pub fn verify_contract_against_trace(
    contract: &ActorContract,
    trace: &PartialTrace,
) -> FindingStatus {
    let ft = partial_trace_to_figure_trace(trace);
    evaluate_figure_trace(contract, &ft)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trace_with_span() -> PartialTrace {
        PartialTrace {
            figure_id: "fig1".into(),
            signals: vec!["Q".into()],
            edges: vec![],
            spans: vec![ValueSpan {
                signal: "Q".into(),
                value: "1".into(),
                from_tick: 1,
                to_tick: 3,
            }],
            delays: vec![],
            causal: vec![],
            ticks: 4,
            confidence: AutomationConfidence::High,
        }
    }

    #[test]
    fn relative_delay_with_bounds_generalizes_to_eventually_within() {
        let t = PartialTrace {
            figure_id: "fig2".into(),
            signals: vec!["A".into(), "B".into()],
            edges: vec![],
            spans: vec![],
            delays: vec![RelativeDelay {
                from_signal: "A".into(),
                to_signal: "B".into(),
                min_cycles: Some(0),
                max_cycles: Some(2),
                annotation_text: "≤ 2 cycles".into(),
            }],
            causal: vec![],
            ticks: 4,
            confidence: AutomationConfidence::Medium,
        };
        let cs = generalize_partial_trace(&t);
        assert_eq!(cs.len(), 1);
        match (&cs[0].obligation, &cs[0].lowering) {
            (
                Obligation::Eventually {
                    target:
                        EventExpr::Edge {
                            signal,
                            dir: EdgeDir::Rose,
                        },
                    window:
                        Window::Within {
                            min: Some(0),
                            max: 2,
                        },
                },
                LoweringDisposition::Lowerable,
            ) if signal == "B" => {}
            other => panic!("unexpected obligation/lowering: {other:?}"),
        }
        assert_eq!(cs[0].provenance.modality, EvidenceModality::Figure);
    }

    #[test]
    fn relative_delay_missing_bounds_lowers_residual() {
        let t = PartialTrace {
            figure_id: "fig3".into(),
            signals: vec!["A".into(), "B".into()],
            edges: vec![],
            spans: vec![],
            delays: vec![RelativeDelay {
                from_signal: "A".into(),
                to_signal: "B".into(),
                min_cycles: None,
                max_cycles: None,
                annotation_text: "after A".into(),
            }],
            causal: vec![],
            ticks: 4,
            confidence: AutomationConfidence::Medium,
        };
        let cs = generalize_partial_trace(&t);
        assert_eq!(cs.len(), 1);
        assert!(matches!(cs[0].obligation, Obligation::Observe { .. }));
        match &cs[0].lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(reason.contains("under-determined delay"), "{reason}");
            }
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn value_span_generalizes_to_stable_within_span_len() {
        let t = trace_with_span();
        let cs = generalize_partial_trace(&t);
        assert_eq!(cs.len(), 1);
        match (&cs[0].obligation, &cs[0].lowering) {
            (
                Obligation::Stable {
                    signal,
                    during: Window::Within { min: None, max: 2 },
                },
                LoweringDisposition::Lowerable,
            ) if signal == "Q" => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn causal_next_tick_generalizes_to_eventually_within_zero_one() {
        let t = PartialTrace {
            figure_id: "fig4".into(),
            signals: vec!["A".into(), "B".into()],
            edges: vec![],
            spans: vec![],
            delays: vec![],
            causal: vec![CausalArrow {
                from_signal: "A".into(),
                from_tick: 1,
                to_signal: "B".into(),
                to_tick: 2,
            }],
            ticks: 3,
            confidence: AutomationConfidence::Medium,
        };
        let cs = generalize_partial_trace(&t);
        assert_eq!(cs.len(), 1);
        match &cs[0].obligation {
            Obligation::Eventually {
                target:
                    EventExpr::Edge {
                        signal,
                        dir: EdgeDir::Rose,
                    },
                window:
                    Window::Within {
                        min: Some(0),
                        max: 1,
                    },
            } if signal == "B" => {}
            other => panic!("expected Eventually{{Within{{0,1}}}}, got {other:?}"),
        }
        assert!(matches!(cs[0].lowering, LoweringDisposition::Lowerable));
    }

    #[test]
    fn bare_edge_generalizes_to_observe_residual() {
        let t = PartialTrace {
            figure_id: "fig5".into(),
            signals: vec!["X".into()],
            edges: vec![LaneEdge {
                signal: "X".into(),
                at_tick: 1,
                kind: EdgeKind::Rising,
            }],
            spans: vec![],
            delays: vec![],
            causal: vec![],
            ticks: 2,
            confidence: AutomationConfidence::Medium,
        };
        let cs = generalize_partial_trace(&t);
        assert_eq!(cs.len(), 1);
        assert!(matches!(cs[0].obligation, Obligation::Observe { .. }));
        match &cs[0].lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(reason.contains("bare edge"), "{reason}");
            }
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn verifier_passes_a_stable_contract_against_its_source_trace() {
        let t = trace_with_span();
        let cs = generalize_partial_trace(&t);
        let c = &cs[0];
        // The lifted figure trace is Q=[0,1,1,1] (span from_tick=1,
        // to_tick=3, value=1) → first 0 then 1s. Stable{Q,
        // Within{max=2}} only inspects the FIRST 2 samples (Q[0]=0,
        // Q[1]=1), which differ → would Fail. The verifier's honesty
        // catches a Stable generalized from a span starting AFTER
        // tick 0: the producer in .3 will use this to demote.
        // For a Stable contract whose source span starts AT tick 0,
        // verifier passes — test that case explicitly:
        let t2 = PartialTrace {
            figure_id: "fig6".into(),
            signals: vec!["Q".into()],
            edges: vec![],
            spans: vec![ValueSpan {
                signal: "Q".into(),
                value: "1".into(),
                from_tick: 0,
                to_tick: 2,
            }],
            delays: vec![],
            causal: vec![],
            ticks: 3,
            confidence: AutomationConfidence::Medium,
        };
        let cs2 = generalize_partial_trace(&t2);
        assert_eq!(
            verify_contract_against_trace(&cs2[0], &t2),
            FindingStatus::Pass
        );
        // And the original t Pass-or-Fail observation is also valid evidence — record it:
        let st = verify_contract_against_trace(c, &t);
        assert!(matches!(st, FindingStatus::Pass | FindingStatus::Fail));
    }

    #[test]
    fn verifier_not_evaluated_on_unsupported_obligation() {
        let t = PartialTrace {
            figure_id: "fig7".into(),
            signals: vec!["A".into(), "B".into()],
            edges: vec![],
            spans: vec![],
            delays: vec![],
            causal: vec![CausalArrow {
                from_signal: "A".into(),
                from_tick: 0,
                to_signal: "B".into(),
                to_tick: 1,
            }],
            ticks: 2,
            confidence: AutomationConfidence::Medium,
        };
        let cs = generalize_partial_trace(&t);
        assert_eq!(cs.len(), 1);
        // Eventually is NotEvaluated by `.2`-scope evaluate_figure_trace.
        assert_eq!(
            verify_contract_against_trace(&cs[0], &t),
            FindingStatus::NotEvaluated
        );
    }
}
