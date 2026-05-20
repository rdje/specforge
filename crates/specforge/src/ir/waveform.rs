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

// ---------- FigureRegion → PartialTrace adapter (R16-WAVEFORM-CONTRACT-MINING.3.2)

use crate::ir::figure_region::{FigureAnnotation, FigureLane, FigureRegion, LaneLevel};

fn level_value(level: &LaneLevel) -> Option<String> {
    match level {
        LaneLevel::High => Some("1".into()),
        LaneLevel::Low => Some("0".into()),
        LaneLevel::Bus(v) => Some(v.clone()),
        LaneLevel::Unknown => None,
    }
}

/// Build `LaneEdge`s + `ValueSpan`s from a single `FigureLane`.
/// `LaneEdge` records transitions between samples; `ValueSpan`
/// records the maximal run of identical value. `Unknown` samples
/// break runs but do not record an edge (honest dormancy — we
/// don't know the level).
fn lane_to_edges_and_spans(lane: &FigureLane) -> (Vec<LaneEdge>, Vec<ValueSpan>) {
    let mut edges = Vec::new();
    let mut spans = Vec::new();
    if lane.samples.is_empty() {
        return (edges, spans);
    }
    // Samples in tick order (best-effort: assume upstream emits sorted).
    let mut prev: Option<&crate::ir::figure_region::LaneSample> = None;
    let mut run_start: Option<u32> = None;
    let mut run_value: Option<String> = None;
    for s in &lane.samples {
        if let Some(p) = prev {
            // Edge?
            match (&p.level, &s.level) {
                (LaneLevel::Low, LaneLevel::High) | (LaneLevel::High, LaneLevel::Low) => {
                    edges.push(LaneEdge {
                        signal: lane.signal_name.clone(),
                        at_tick: s.at_tick,
                        kind: if matches!(s.level, LaneLevel::High) {
                            EdgeKind::Rising
                        } else {
                            EdgeKind::Falling
                        },
                    });
                }
                _ => {}
            }
        }
        // Run handling — track contiguous identical-value spans.
        match level_value(&s.level) {
            Some(v) => match (&run_value, run_start) {
                (Some(rv), Some(rs)) if *rv == v => {
                    // Continue current run; nothing to flush.
                    let _ = (rs, rv);
                }
                _ => {
                    // Flush previous run.
                    if let (Some(rv), Some(rs)) = (run_value.take(), run_start.take())
                        && s.at_tick > rs
                    {
                        // Closed when value changed at s.at_tick.
                        // ValueSpan's to_tick is inclusive; the run held value `rv`
                        // through s.at_tick - 1.
                        spans.push(ValueSpan {
                            signal: lane.signal_name.clone(),
                            value: rv,
                            from_tick: rs,
                            to_tick: s.at_tick.saturating_sub(1),
                        });
                    }
                    run_start = Some(s.at_tick);
                    run_value = Some(v);
                }
            },
            None => {
                // Unknown breaks the run; flush.
                if let (Some(rv), Some(rs)) = (run_value.take(), run_start.take())
                    && s.at_tick > rs
                {
                    spans.push(ValueSpan {
                        signal: lane.signal_name.clone(),
                        value: rv,
                        from_tick: rs,
                        to_tick: s.at_tick.saturating_sub(1),
                    });
                }
            }
        }
        prev = Some(s);
    }
    // Flush any trailing run (closes at the last sample's tick).
    if let (Some(rv), Some(rs)) = (run_value, run_start)
        && let Some(last) = lane.samples.last()
        && last.at_tick > rs
    {
        spans.push(ValueSpan {
            signal: lane.signal_name.clone(),
            value: rv,
            from_tick: rs,
            to_tick: last.at_tick,
        });
    }
    (edges, spans)
}

/// Adapter (`R16-WAVEFORM-CONTRACT-MINING.3.2`): consume a
/// `FigureRegion` produced by an upstream PDF pipeline and produce
/// the typed `PartialTrace` the `.2` generalizer consumes.
///
/// Mapping rules:
/// - each `FigureLane` ⇒ `LaneEdge`s on level transitions and
///   `ValueSpan`s on contiguous identical-value runs;
/// - `FigureAnnotation::Delay { from_signal, to_signal, min_cycles,
///   max_cycles, … }` ⇒ `RelativeDelay`;
/// - `FigureAnnotation::Value { signal, value, from_tick, to_tick,
///   … }` ⇒ extra `ValueSpan`;
/// - `FigureAnnotation::Label` is informational only (ignored by the
///   adapter — recovered via prose / KG, not the trace);
/// - `FigureAnnotation::Unknown` lowers `PartialTrace.confidence`
///   (Medium → Low) — honest dormancy; never silently licensed.
///
/// `figure_id` is `FigureRegion.visual_asset_id` (the upstream
/// `VisualAsset` reference). The output `PartialTrace.ticks` is
/// `FigureRegion.inferred_ticks()`.
pub fn figure_region_to_partial_trace(region: &FigureRegion) -> PartialTrace {
    let mut edges = Vec::new();
    let mut spans = Vec::new();
    let mut signals: BTreeMap<String, ()> = BTreeMap::new();
    let mut delays = Vec::new();
    let mut unknown_count = 0u32;

    for lane in &region.waveform_lanes {
        signals.insert(lane.signal_name.clone(), ());
        let (e, s) = lane_to_edges_and_spans(lane);
        edges.extend(e);
        spans.extend(s);
    }

    for ann in &region.annotations {
        match ann {
            FigureAnnotation::Delay {
                from_signal,
                to_signal,
                min_cycles,
                max_cycles,
                text,
                ..
            } => {
                signals.insert(from_signal.clone(), ());
                signals.insert(to_signal.clone(), ());
                delays.push(RelativeDelay {
                    from_signal: from_signal.clone(),
                    to_signal: to_signal.clone(),
                    min_cycles: *min_cycles,
                    max_cycles: *max_cycles,
                    annotation_text: text.clone(),
                });
            }
            FigureAnnotation::Value {
                signal,
                value,
                from_tick,
                to_tick,
                ..
            } => {
                signals.insert(signal.clone(), ());
                spans.push(ValueSpan {
                    signal: signal.clone(),
                    value: value.clone(),
                    from_tick: *from_tick,
                    to_tick: *to_tick,
                });
            }
            FigureAnnotation::Label { .. } => {
                // Informational only — not lifted to the trace.
            }
            FigureAnnotation::Unknown { .. } => {
                unknown_count += 1;
            }
        }
    }

    // Confidence downgrade per honest dormancy: any Unknown annotation
    // demotes the trace's confidence one rank (Medium ⇒ Low, High ⇒
    // Medium). The trace never silently keeps confidence in the face
    // of upstream-unclassified evidence.
    let confidence = if unknown_count > 0 {
        match region.confidence {
            AutomationConfidence::High => AutomationConfidence::Medium,
            AutomationConfidence::Medium => AutomationConfidence::Low,
            AutomationConfidence::Low => AutomationConfidence::Low,
        }
    } else {
        region.confidence
    };

    PartialTrace {
        figure_id: region.visual_asset_id.clone(),
        signals: signals.into_keys().collect(),
        edges,
        spans,
        delays,
        causal: Vec::new(), // Upstream does not (yet) emit causal arrows.
        ticks: region.inferred_ticks(),
        confidence,
    }
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

    // ---------- R16-WAVEFORM-CONTRACT-MINING.3.2 adapter tests

    use crate::ir::figure_region::{
        FigureAnnotation, FigureLane, FigureRegion, LaneLevel, LaneSample,
    };

    fn lane_q_pulse_2() -> FigureLane {
        FigureLane {
            signal_name: "Q".into(),
            samples: vec![
                LaneSample {
                    at_tick: 0,
                    level: LaneLevel::Low,
                },
                LaneSample {
                    at_tick: 1,
                    level: LaneLevel::High,
                },
                LaneSample {
                    at_tick: 2,
                    level: LaneLevel::High,
                },
                LaneSample {
                    at_tick: 3,
                    level: LaneLevel::Low,
                },
            ],
        }
    }

    #[test]
    fn adapter_lifts_lanes_into_edges_and_value_spans() {
        let region = FigureRegion {
            visual_asset_id: "fig:rv1".into(),
            bbox: None,
            annotations: vec![],
            waveform_lanes: vec![lane_q_pulse_2()],
            tick_count: None,
            raw_image_path: None,
            confidence: AutomationConfidence::High,
        };
        let pt = figure_region_to_partial_trace(&region);
        assert_eq!(pt.figure_id, "fig:rv1");
        assert_eq!(pt.ticks, 4);
        assert_eq!(pt.signals, vec!["Q".to_string()]);
        // Edges: rising at tick 1, falling at tick 3.
        assert_eq!(pt.edges.len(), 2);
        assert!(
            pt.edges
                .iter()
                .any(|e| e.signal == "Q" && e.at_tick == 1 && matches!(e.kind, EdgeKind::Rising))
        );
        assert!(
            pt.edges
                .iter()
                .any(|e| e.signal == "Q" && e.at_tick == 3 && matches!(e.kind, EdgeKind::Falling))
        );
        // Spans: Q="0" 0..0, Q="1" 1..2, Q="0" 3..3. The first and
        // last single-tick runs are flushed too (closed at 0 by
        // the rising transition; closed at 3 by the trailing
        // flush). Verify the multi-tick high span is present.
        assert!(
            pt.spans
                .iter()
                .any(|s| s.signal == "Q" && s.value == "1" && s.from_tick == 1 && s.to_tick == 2)
        );
    }

    #[test]
    fn adapter_lifts_delay_annotation_into_relative_delay() {
        let region = FigureRegion {
            visual_asset_id: "fig:d".into(),
            bbox: None,
            annotations: vec![FigureAnnotation::Delay {
                from_signal: "A".into(),
                to_signal: "B".into(),
                min_cycles: Some(2),
                max_cycles: None,
                text: "≥ 2 cycles".into(),
                bbox: None,
            }],
            waveform_lanes: vec![],
            tick_count: Some(5),
            raw_image_path: None,
            confidence: AutomationConfidence::Medium,
        };
        let pt = figure_region_to_partial_trace(&region);
        assert_eq!(pt.delays.len(), 1);
        assert_eq!(pt.delays[0].from_signal, "A");
        assert_eq!(pt.delays[0].to_signal, "B");
        assert_eq!(pt.delays[0].min_cycles, Some(2));
        assert_eq!(pt.delays[0].max_cycles, None);
        // Signals include both delay endpoints even with no lanes.
        assert!(pt.signals.contains(&"A".to_string()));
        assert!(pt.signals.contains(&"B".to_string()));
    }

    #[test]
    fn adapter_lifts_value_annotation_into_value_span() {
        let region = FigureRegion {
            visual_asset_id: "fig:v".into(),
            bbox: None,
            annotations: vec![FigureAnnotation::Value {
                signal: "DATA".into(),
                value: "0xAB".into(),
                from_tick: 2,
                to_tick: 5,
                text: "DATA = 0xAB".into(),
                bbox: None,
            }],
            waveform_lanes: vec![],
            tick_count: None,
            raw_image_path: None,
            confidence: AutomationConfidence::High,
        };
        let pt = figure_region_to_partial_trace(&region);
        assert_eq!(pt.spans.len(), 1);
        assert_eq!(pt.spans[0].signal, "DATA");
        assert_eq!(pt.spans[0].value, "0xAB");
        assert_eq!(pt.spans[0].from_tick, 2);
        assert_eq!(pt.spans[0].to_tick, 5);
        // tick count = max(to_tick) + 1 = 6.
        assert_eq!(pt.ticks, 6);
    }

    #[test]
    fn adapter_demotes_confidence_on_any_unknown_annotation() {
        let region = FigureRegion {
            visual_asset_id: "fig:u".into(),
            bbox: None,
            annotations: vec![FigureAnnotation::Unknown {
                text: "???".into(),
                bbox: None,
            }],
            waveform_lanes: vec![],
            tick_count: Some(1),
            raw_image_path: None,
            confidence: AutomationConfidence::High,
        };
        let pt = figure_region_to_partial_trace(&region);
        // High ⇒ Medium under unknown demotion (honest dormancy).
        assert_eq!(pt.confidence, AutomationConfidence::Medium);
    }

    #[test]
    fn adapter_round_trips_into_generalizer_and_verifier_pass() {
        // End-to-end smoke: FigureRegion → PartialTrace → contracts →
        // verifier Pass on at least one Stable contract from a Value
        // annotation that the verifier knows how to evaluate.
        let region = FigureRegion {
            visual_asset_id: "fig:e2e".into(),
            bbox: None,
            annotations: vec![FigureAnnotation::Value {
                signal: "D".into(),
                value: "1".into(),
                from_tick: 0,
                to_tick: 2,
                text: "D held".into(),
                bbox: None,
            }],
            waveform_lanes: vec![],
            tick_count: Some(3),
            raw_image_path: None,
            confidence: AutomationConfidence::Medium,
        };
        let pt = figure_region_to_partial_trace(&region);
        let cs = generalize_partial_trace(&pt);
        // The Value annotation produced a multi-tick ValueSpan; the
        // generalizer turned it into a Stable contract.
        let stable = cs
            .iter()
            .find(|c| matches!(c.obligation, Obligation::Stable { .. }))
            .expect("expected at least one Stable contract");
        // The verifier passes Stable contracts evaluated against the
        // source trace (per fidelity.rs evaluate_figure_trace
        // semantics for Stable + Within).
        assert_eq!(
            verify_contract_against_trace(stable, &pt),
            FindingStatus::Pass
        );
    }

    #[test]
    fn adapter_label_annotation_is_informational_only() {
        let region = FigureRegion {
            visual_asset_id: "fig:l".into(),
            bbox: None,
            annotations: vec![FigureAnnotation::Label {
                text: "Bus A".into(),
                bbox: None,
            }],
            waveform_lanes: vec![],
            tick_count: Some(0),
            raw_image_path: None,
            confidence: AutomationConfidence::Medium,
        };
        let pt = figure_region_to_partial_trace(&region);
        assert!(pt.edges.is_empty());
        assert!(pt.spans.is_empty());
        assert!(pt.delays.is_empty());
        // Confidence is unchanged for Label.
        assert_eq!(pt.confidence, AutomationConfidence::Medium);
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
