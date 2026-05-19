//! ContractIR — typed timed-contract model (R16-CONTRACT-IR).
//!
//! Per the `R16-CONTRACT-IR.1` design (see
//! `docs/tasks/R16-CONTRACT-IR.md`): a per-actor `assume`/`guarantee`
//! contract over boundary signals, expressed in a small, **closed**
//! operator algebra so it is realizability-checkable, mechanically
//! lowerable, and has a crisp residual boundary (`lowering:
//! Residual{reason}` is explicit — never silent loss).
//!
//! `.2` scope (this module): the typed model + serde + a lossless
//! `contract_from_temporal_rule` conversion + unit tests. The field is
//! added additively to `SemanticIr`/`IntentIr` but is **not yet
//! populated and `.isf` lowering is unchanged** — re-pointing producers
//! and the adapter onto ContractIR with a corpus CI-parity gate is
//! `R16-CONTRACT-IR.3`.

use serde::{Deserialize, Serialize};

use crate::ir::semantic::{ClockEdge, TemporalPredicateRecord, TemporalRuleRecord, TickPhase};
use crate::ir::source::AutomationConfidence;

/// Edge direction of a signal transition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EdgeDir {
    Rose,
    Fell,
}

/// Where on a phase an event is anchored.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PhaseAt {
    Enter,
    Exit,
}

/// A point in time at the actor boundary.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum EventExpr {
    /// A signal transition.
    Edge { signal: String, dir: EdgeDir },
    /// A signal holding a value (a state, used as a level/condition).
    Level { signal: String, value: String },
    /// The cycle a ready/valid handshake fires (the transfer point).
    HandshakeFire { valid: String, ready: String },
    /// Transaction / phase start.
    Start,
    /// Enter/exit of a named phase (incl. tick phases `pre_tick`/`post_tick`).
    PhaseBoundary { phase: String, at: PhaseAt },
}

/// A bounded interval.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Window {
    /// Bounded `(within N)` — `max` cycles, `max >= 1` (FSMGen strict
    /// rejects `(within 0)`; a 0-cycle obligation is `SameCycle`).
    Within {
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<u32>,
        max: u32,
    },
    /// Interval delimited by two boundary events (stability / throughout).
    Between { from: EventExpr, to: EventExpr },
    /// 0-cycle / combinational — explicitly distinct so `(within 0)` is
    /// never emitted; lowers residual, but the intent is modelled.
    SameCycle,
}

/// A bounded `.isf`-safe guard (the existing rule condition form).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Condition {
    /// `(== signal value)`.
    Eq { signal: String, value: String },
}

/// One step of an ordered sequence obligation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SequenceStep {
    pub event: EventExpr,
    pub window: Window,
}

/// The guaranteed/assumed behavior.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Obligation {
    /// `target` must occur within `window` (FSMGen `bounded_eventually`).
    Eventually { target: EventExpr, window: Window },
    /// `signal` is stable across `during`.
    Stable { signal: String, during: Window },
    /// `signal` is driven to `value` (an actor `(rule …)` drive).
    Drive { signal: String, value: String },
    /// Ready/valid barrier (FSMGen `(stage p (ready r)(valid v))`).
    HandshakeBarrier { valid: String, ready: String },
    /// `hold` persists until `until` is observed.
    Persist { hold: EventExpr, until: EventExpr },
    /// Ordered sequence of boundary events with per-step windows.
    Sequence { steps: Vec<SequenceStep> },
    /// Mutual exclusion of two signals.
    Mutex { a: String, b: String },
    /// Phase ordering.
    OrderedBefore {
        earlier_phase: String,
        later_phase: String,
    },
    /// Weak observational fact: a boundary signal participates but no
    /// concrete value/window is licensed by the source. Captured in the
    /// typed KG; lowers residual (never fabricated into a value).
    Observe { signal: String },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractKind {
    Assume,
    Guarantee,
}

/// Coarse, interface-independent lowering hint. The precise,
/// interface-aware `.isf` decision stays in the adapter
/// (`R16-CONTRACT-IR.3`); this only records "can a supported construct
/// represent this shape at all?".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum LoweringDisposition {
    Lowerable,
    /// No representable supported `.isf` construct — explicit residual;
    /// syntax is never fabricated.
    Residual {
        reason: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceModality {
    Prose,
    Table,
    Figure,
    StateDiagram,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractProvenance {
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    pub source_text: String,
    pub modality: EvidenceModality,
}

/// A per-actor timed contract over boundary signals.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorContract {
    pub contract_id: String,
    /// The originating `TemporalRuleRecord.rule_id` (provenance). Lets
    /// the adapter reproduce the exact pre-ContractIR `.isf` naming /
    /// disposition during the `R16-CONTRACT-IR.3` parity re-point.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_rule_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name: Option<String>,
    pub kind: ContractKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<Condition>,
    pub obligation: Obligation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_signal: Option<String>,
    pub edge: ClockEdge,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    pub provenance: ContractProvenance,
    pub lowering: LoweringDisposition,
    pub automation_confidence: AutomationConfidence,
}

fn tick_phase_event(phase: TickPhase) -> EventExpr {
    let name = match phase {
        TickPhase::PreTick => "pre_tick",
        TickPhase::PostTick => "post_tick",
    };
    EventExpr::PhaseBoundary {
        phase: name.to_string(),
        at: PhaseAt::Enter,
    }
}

/// First antecedent that is a `SignalValue` → an `Eq` guard.
fn guard_from_antecedents(rule: &TemporalRuleRecord) -> Option<Condition> {
    rule.antecedents.iter().find_map(|p| match p {
        TemporalPredicateRecord::SignalValue {
            signal_name, value, ..
        } => Some(Condition::Eq {
            signal: signal_name.clone(),
            value: value.clone(),
        }),
        _ => None,
    })
}

/// First consequent that names a single signal — mirrors the adapter's
/// `temporal_consequent_signal` EXACTLY (SignalValue / ActorDrivesSignal /
/// ActorMaintainsSignalStable / SignalStable / ActorSamplesSignal /
/// SignalSampled → the signal; HandshakeComplete → None). This parity with
/// the classifier is what lets `R16-CONTRACT-IR.3` reproduce the current
/// windowed→Contract decision for *every* signal-bearing consequent (not
/// only `SignalValue`).
fn consequent_signal(rule: &TemporalRuleRecord) -> Option<String> {
    rule.consequents.iter().find_map(|p| match p {
        TemporalPredicateRecord::SignalValue { signal_name, .. }
        | TemporalPredicateRecord::ActorDrivesSignal { signal_name, .. }
        | TemporalPredicateRecord::ActorMaintainsSignalStable { signal_name, .. }
        | TemporalPredicateRecord::SignalStable { signal_name, .. }
        | TemporalPredicateRecord::ActorSamplesSignal { signal_name, .. }
        | TemporalPredicateRecord::SignalSampled { signal_name, .. } => Some(signal_name.clone()),
        TemporalPredicateRecord::HandshakeComplete { .. } => None,
    })
}

/// First `SignalValue` consequent (signal, value) — mirrors the adapter's
/// `temporal_drive_consequent`.
fn drive_consequent(rule: &TemporalRuleRecord) -> Option<(String, String)> {
    rule.consequents.iter().find_map(|p| match p {
        TemporalPredicateRecord::SignalValue {
            signal_name, value, ..
        } => Some((signal_name.clone(), value.clone())),
        _ => None,
    })
}

/// The level value to anchor a windowed `Eventually` target on: the
/// `SignalValue` value when the first consequent carries one, else
/// `"1"` (asserted). Irrelevant to emitted `.isf` (the `bounded_eventually`
/// contract is `(eventually <signal> (within N))` — no value), kept only
/// so the typed obligation is well-formed.
fn consequent_level_value(rule: &TemporalRuleRecord) -> String {
    drive_consequent(rule)
        .map(|(_, v)| v)
        .unwrap_or_else(|| "1".to_string())
}

/// Actor name carried by the first consequent that has one.
fn actor_from_consequents(rule: &TemporalRuleRecord) -> Option<String> {
    rule.consequents.iter().find_map(|p| match p {
        TemporalPredicateRecord::ActorDrivesSignal { actor_name, .. }
        | TemporalPredicateRecord::ActorMaintainsSignalStable { actor_name, .. }
        | TemporalPredicateRecord::ActorSamplesSignal { actor_name, .. } => {
            Some(actor_name.clone())
        }
        _ => None,
    })
}

/// Lossless `TemporalRuleRecord` → `ActorContract` per the
/// `R16-CONTRACT-IR.1` migration map. Representable shapes get a precise
/// obligation + `Lowerable`; everything else is captured with an
/// explicit `Residual{reason}` (the intent is modelled in the typed KG
/// even when `.isf` lowering stays residual — the R16 thesis). Never
/// fabricates a value/window the source does not license.
pub fn contract_from_temporal_rule(rule: &TemporalRuleRecord) -> ActorContract {
    let usable_max = rule
        .cycle_window
        .as_ref()
        .and_then(|w| w.max_cycles)
        .filter(|&m| m >= 1);
    let min_cycles = rule.cycle_window.as_ref().and_then(|w| w.min_cycles);
    let has_window = rule.cycle_window.is_some();

    let first = rule.consequents.first();

    // Window-first, structurally PARALLEL to the adapter's
    // `classify_temporal_rule` (R16-CONTRACT-IR.3 parity): a windowed
    // rule whose first signal-bearing consequent is declared lowers to a
    // bounded_eventually Contract for EVERY signal-bearing predicate kind
    // (not only `SignalValue`) — `consequent_signal` mirrors the
    // classifier's `temporal_consequent_signal` exactly.
    let (kind, obligation, lowering) = if has_window {
        match (usable_max, consequent_signal(rule)) {
            // Positive bound + single-signal consequent → bounded_eventually.
            (Some(max), Some(signal)) => (
                ContractKind::Guarantee,
                Obligation::Eventually {
                    target: EventExpr::Level {
                        signal,
                        value: consequent_level_value(rule),
                    },
                    window: Window::Within {
                        min: min_cycles,
                        max,
                    },
                },
                LoweringDisposition::Lowerable,
            ),
            // Positive bound but no single-signal consequent (e.g.
            // HandshakeComplete) — classifier residual; keep
            // HandshakeBarrier for `.4`, else Observe.
            (Some(_), None) => {
                let ob = match first {
                    Some(TemporalPredicateRecord::HandshakeComplete {
                        valid_signal,
                        ready_signal,
                        ..
                    }) => Obligation::HandshakeBarrier {
                        valid: valid_signal.clone(),
                        ready: ready_signal.clone(),
                    },
                    _ => Obligation::Observe {
                        signal: String::new(),
                    },
                };
                (
                    ContractKind::Guarantee,
                    ob,
                    LoweringDisposition::Residual {
                        reason: "windowed temporal rule has no single-signal \
                                 consequent (e.g. HandshakeComplete) — not a \
                                 bounded_eventually"
                            .to_string(),
                    },
                )
            }
            // Window present but unusable (0 / no max) — classifier residual.
            (None, sig_opt) => {
                let ob = match (first, sig_opt) {
                    (
                        Some(TemporalPredicateRecord::HandshakeComplete {
                            valid_signal,
                            ready_signal,
                            ..
                        }),
                        _,
                    ) => Obligation::HandshakeBarrier {
                        valid: valid_signal.clone(),
                        ready: ready_signal.clone(),
                    },
                    (_, Some(signal)) => Obligation::Eventually {
                        target: EventExpr::Level {
                            signal,
                            value: consequent_level_value(rule),
                        },
                        window: Window::SameCycle,
                    },
                    (_, None) => Obligation::Observe {
                        signal: String::new(),
                    },
                };
                (
                    ContractKind::Guarantee,
                    ob,
                    LoweringDisposition::Residual {
                        reason: "windowed temporal rule has a 0/none-cycle bound; \
                                 FSMGen strict rejects `(within 0)` — not a \
                                 bounded_eventually"
                            .to_string(),
                    },
                )
            }
        }
    } else {
        // Non-windowed (classifier branches 2 & 3).
        match drive_consequent(rule) {
            // Non-windowed `SignalValue` → actor `(rule …)` drive.
            Some((signal, value)) => (
                ContractKind::Guarantee,
                Obligation::Drive { signal, value },
                LoweringDisposition::Lowerable,
            ),
            None => match first {
                Some(TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    ..
                }) => (
                    ContractKind::Guarantee,
                    Obligation::HandshakeBarrier {
                        valid: valid_signal.clone(),
                        ready: ready_signal.clone(),
                    },
                    LoweringDisposition::Residual {
                        reason: "non-windowed handshake completion — no \
                                 representable supported .isf construct \
                                 (HandshakeBarrier; (stage …) enabled in .4)"
                            .to_string(),
                    },
                ),
                Some(
                    TemporalPredicateRecord::SignalStable {
                        signal_name,
                        from_phase,
                        to_phase,
                    }
                    | TemporalPredicateRecord::ActorMaintainsSignalStable {
                        signal_name,
                        from_phase,
                        to_phase,
                        ..
                    },
                ) => (
                    ContractKind::Guarantee,
                    Obligation::Stable {
                        signal: signal_name.clone(),
                        during: Window::Between {
                            from: tick_phase_event(*from_phase),
                            to: tick_phase_event(*to_phase),
                        },
                    },
                    LoweringDisposition::Residual {
                        reason: "bare stability across tick phases has no \
                                 supported .isf construct"
                            .to_string(),
                    },
                ),
                Some(TemporalPredicateRecord::ActorDrivesSignal { signal_name, .. }) => (
                    ContractKind::Guarantee,
                    Obligation::Observe {
                        signal: signal_name.clone(),
                    },
                    LoweringDisposition::Residual {
                        reason: "actor-drives predicate names a signal but \
                                 carries no concrete value/window"
                            .to_string(),
                    },
                ),
                Some(
                    TemporalPredicateRecord::ActorSamplesSignal { signal_name, .. }
                    | TemporalPredicateRecord::SignalSampled { signal_name, .. },
                ) => (
                    ContractKind::Assume,
                    Obligation::Observe {
                        signal: signal_name.clone(),
                    },
                    LoweringDisposition::Residual {
                        reason: "sample predicate names a signal but carries no \
                                 concrete value/window"
                            .to_string(),
                    },
                ),
                // `SignalValue` is covered by `drive_consequent` above;
                // `None` = producer emitted no consequents (defensive).
                _ => (
                    ContractKind::Guarantee,
                    Obligation::Observe {
                        signal: String::new(),
                    },
                    LoweringDisposition::Residual {
                        reason: "temporal rule has no consequents".to_string(),
                    },
                ),
            },
        }
    };

    ActorContract {
        contract_id: format!("contract_{}", rule.rule_id),
        source_rule_id: Some(rule.rule_id.clone()),
        actor_name: actor_from_consequents(rule),
        kind,
        guard: guard_from_antecedents(rule),
        obligation,
        clock_signal: rule.clock_signal.clone(),
        edge: rule.edge,
        channel: None,
        phase: None,
        provenance: ContractProvenance {
            supporting_statement_ids: rule.supporting_statement_ids.clone(),
            source_text: rule.source_text.clone(),
            modality: EvidenceModality::Prose,
        },
        lowering,
        automation_confidence: rule.automation_confidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::semantic::CycleWindowRecord;

    fn rule(
        consequents: Vec<TemporalPredicateRecord>,
        antecedents: Vec<TemporalPredicateRecord>,
        cycle_window: Option<CycleWindowRecord>,
    ) -> TemporalRuleRecord {
        TemporalRuleRecord {
            rule_id: "r1".to_string(),
            clock_signal: Some("clk".to_string()),
            edge: ClockEdge::Rising,
            antecedents,
            consequents,
            cycle_window,
            source_text: "src".to_string(),
            supporting_statement_ids: vec!["s1".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn sigval(name: &str, value: &str) -> TemporalPredicateRecord {
        TemporalPredicateRecord::SignalValue {
            signal_name: name.to_string(),
            value: value.to_string(),
            phase: TickPhase::PostTick,
        }
    }

    #[test]
    fn windowed_signalvalue_is_eventually_within_lowerable() {
        let c = contract_from_temporal_rule(&rule(
            vec![sigval("ACK", "1")],
            vec![],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(4),
            }),
        ));
        assert_eq!(
            c.obligation,
            Obligation::Eventually {
                target: EventExpr::Level {
                    signal: "ACK".into(),
                    value: "1".into()
                },
                window: Window::Within { min: None, max: 4 },
            }
        );
        assert_eq!(c.lowering, LoweringDisposition::Lowerable);
        assert_eq!(c.kind, ContractKind::Guarantee);
        assert_eq!(c.contract_id, "contract_r1");
    }

    #[test]
    fn zero_cycle_window_is_samecycle_residual_never_within_zero() {
        let c = contract_from_temporal_rule(&rule(
            vec![sigval("ACK", "1")],
            vec![],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(0),
            }),
        ));
        match c.obligation {
            Obligation::Eventually { window, .. } => assert_eq!(window, Window::SameCycle),
            other => panic!("expected Eventually/SameCycle, got {other:?}"),
        }
        assert!(matches!(c.lowering, LoweringDisposition::Residual { .. }));
    }

    #[test]
    fn non_windowed_signalvalue_with_antecedent_is_drive_with_guard() {
        let c = contract_from_temporal_rule(&rule(
            vec![sigval("GRANT", "1")],
            vec![sigval("SEL", "1")],
            None,
        ));
        assert_eq!(
            c.obligation,
            Obligation::Drive {
                signal: "GRANT".into(),
                value: "1".into()
            }
        );
        assert_eq!(
            c.guard,
            Some(Condition::Eq {
                signal: "SEL".into(),
                value: "1".into()
            })
        );
        assert_eq!(c.lowering, LoweringDisposition::Lowerable);
    }

    #[test]
    fn non_windowed_handshake_complete_is_barrier_residual_until_dot4() {
        // Parity (`.3`): non-windowed HandshakeComplete is residual today;
        // the obligation is `HandshakeBarrier` so `.4` can enable
        // `(stage …)` without re-shaping the contract.
        let c = contract_from_temporal_rule(&rule(
            vec![TemporalPredicateRecord::HandshakeComplete {
                valid_signal: "AWVALID".into(),
                ready_signal: "AWREADY".into(),
                phase: TickPhase::PostTick,
            }],
            vec![],
            None,
        ));
        assert_eq!(
            c.obligation,
            Obligation::HandshakeBarrier {
                valid: "AWVALID".into(),
                ready: "AWREADY".into()
            }
        );
        assert!(matches!(c.lowering, LoweringDisposition::Residual { .. }));
    }

    #[test]
    fn windowed_signalstable_is_eventually_lowerable_parity_finding() {
        // R16-CONTRACT-IR.3 parity finding: the classifier's
        // `temporal_consequent_signal` treats `SignalStable` as
        // signal-bearing, so a *windowed* SignalStable currently lowers to
        // a bounded_eventually Contract. The conversion must reproduce that
        // (was wrongly mapped to Stable+Residual in the `.2` draft).
        let c = contract_from_temporal_rule(&rule(
            vec![TemporalPredicateRecord::SignalStable {
                signal_name: "ADDR".into(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }],
            vec![],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(4),
            }),
        ));
        assert_eq!(
            c.obligation,
            Obligation::Eventually {
                target: EventExpr::Level {
                    signal: "ADDR".into(),
                    value: "1".into()
                },
                window: Window::Within { min: None, max: 4 },
            }
        );
        assert_eq!(c.lowering, LoweringDisposition::Lowerable);
    }

    #[test]
    fn signal_stable_is_stable_between_residual_modelled() {
        let c = contract_from_temporal_rule(&rule(
            vec![TemporalPredicateRecord::SignalStable {
                signal_name: "ADDR".into(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }],
            vec![],
            None,
        ));
        assert_eq!(
            c.obligation,
            Obligation::Stable {
                signal: "ADDR".into(),
                during: Window::Between {
                    from: EventExpr::PhaseBoundary {
                        phase: "pre_tick".into(),
                        at: PhaseAt::Enter
                    },
                    to: EventExpr::PhaseBoundary {
                        phase: "post_tick".into(),
                        at: PhaseAt::Enter
                    },
                },
            }
        );
        assert!(matches!(c.lowering, LoweringDisposition::Residual { .. }));
    }

    #[test]
    fn actor_drives_no_value_is_observe_residual_with_actor() {
        let c = contract_from_temporal_rule(&rule(
            vec![TemporalPredicateRecord::ActorDrivesSignal {
                actor_name: "Manager".into(),
                signal_name: "GRANT".into(),
                phase: TickPhase::PostTick,
            }],
            vec![],
            None,
        ));
        assert_eq!(
            c.obligation,
            Obligation::Observe {
                signal: "GRANT".into()
            }
        );
        assert_eq!(c.actor_name, Some("Manager".to_string()));
        assert!(matches!(c.lowering, LoweringDisposition::Residual { .. }));
    }

    #[test]
    fn serde_round_trips() {
        let c = contract_from_temporal_rule(&rule(
            vec![sigval("ACK", "1")],
            vec![sigval("REQ", "1")],
            Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(4),
            }),
        ));
        let json = serde_json::to_string(&c).expect("serialize");
        let back: ActorContract = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(c, back);
    }
}
