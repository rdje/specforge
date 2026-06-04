//! Render mined temporal rules in standard LTL/MTL notation (`TEMPORAL-RULE-LTL-RENDER`).
//!
//! SpecForge's [`TemporalRuleRecord`]s are the `G(antecedent → consequent)`-with-holes
//! property template the spec-mining literature (Pnueli's LTL; GoldMine; Texada) formalized,
//! but they are stored in an ad-hoc typed shape. This module is a **pure, derived** rendering
//! of a rule into that notation — read-only, never persisted to the IR (so it adds no fixture
//! churn). It is the standard-vocabulary foundation a downstream `.isf` → PSL/SVA export will
//! consume; see `docs/tasks/TEMPORAL-RULE-LTL-RENDER.md`.

use crate::ir::semantic::{CycleWindowRecord, TemporalPredicateRecord, TemporalRuleRecord};

/// One temporal predicate as a readable LTL atom. The tick phase is carried by the rule's
/// `G(ante → X/F cons)` structure (pre-tick guard, post-tick result), so individual atoms
/// render their signal/actor identity, not the phase.
fn predicate_atom(pred: &TemporalPredicateRecord) -> String {
    match pred {
        TemporalPredicateRecord::SignalValue {
            signal_name, value, ..
        } => format!("{signal_name}=={value}"),
        TemporalPredicateRecord::ActorDrivesSignal {
            actor_name,
            signal_name,
            ..
        } => format!("drive({actor_name},{signal_name})"),
        TemporalPredicateRecord::ActorMaintainsSignalStable {
            actor_name,
            signal_name,
            ..
        } => format!("stable({actor_name},{signal_name})"),
        TemporalPredicateRecord::SignalStable { signal_name, .. } => {
            format!("stable({signal_name})")
        }
        TemporalPredicateRecord::ActorSamplesSignal {
            actor_name,
            signal_name,
            ..
        } => format!("sample({actor_name},{signal_name})"),
        TemporalPredicateRecord::SignalSampled { signal_name, .. } => {
            format!("sample({signal_name})")
        }
        TemporalPredicateRecord::HandshakeComplete {
            valid_signal,
            ready_signal,
            ..
        } => format!("handshake({valid_signal},{ready_signal})"),
    }
}

/// Conjoin predicate atoms with ` & ` (LTL `&` binds tighter than `->`, so no outer parens).
fn join_and(preds: &[TemporalPredicateRecord]) -> String {
    preds
        .iter()
        .map(predicate_atom)
        .collect::<Vec<_>>()
        .join(" & ")
}

/// The temporal operator applied to the consequent: `X` (next tick) with no cycle window, else
/// the bounded-eventually `F[min,max]` of Metric Temporal Logic (an open bound prints as `∞`).
fn consequent_operator(window: Option<&CycleWindowRecord>) -> String {
    match window {
        Some(w) => {
            let lo = w
                .min_cycles
                .map(|n| n.to_string())
                .unwrap_or_else(|| "0".to_string());
            let hi = w
                .max_cycles
                .map(|n| n.to_string())
                .unwrap_or_else(|| "∞".to_string());
            format!("F[{lo},{hi}]")
        }
        None => "X".to_string(),
    }
}

/// Render a mined [`TemporalRuleRecord`] as a standard LTL/MTL formula string.
///
/// Form: `G( <ante> -> <op> <cons> )`, where `<op>` is `X` (next tick) or `F[min,max]` when a
/// cycle window is present. An empty antecedent yields the invariant `G( <cons> )`. A
/// multi-atom consequent is parenthesized so the temporal operator scopes the whole
/// conjunction. The clock edge / signal (the domain `G` ranges over) is rule metadata, not
/// part of the formula.
pub fn temporal_rule_to_ltl(rule: &TemporalRuleRecord) -> String {
    let cons_body = join_and(&rule.consequents);
    let cons_body = if cons_body.is_empty() {
        "true".to_string()
    } else {
        cons_body
    };

    if rule.antecedents.is_empty() {
        return format!("G( {cons_body} )");
    }

    let ante = join_and(&rule.antecedents);
    let op = consequent_operator(rule.cycle_window.as_ref());
    // Parenthesize a multi-atom consequent so the temporal operator scopes the conjunction.
    let cons = if rule.consequents.len() > 1 {
        format!("({cons_body})")
    } else {
        cons_body
    };
    format!("G( {ante} -> {op} {cons} )")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::semantic::{ClockEdge, TickPhase};
    use crate::ir::source::AutomationConfidence;

    fn rule(
        antecedents: Vec<TemporalPredicateRecord>,
        consequents: Vec<TemporalPredicateRecord>,
        cycle_window: Option<CycleWindowRecord>,
    ) -> TemporalRuleRecord {
        TemporalRuleRecord {
            rule_id: "t".to_string(),
            clock_signal: Some("PCLK".to_string()),
            edge: ClockEdge::Rising,
            antecedents,
            consequents,
            cycle_window,
            source_text: String::new(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn sv(signal: &str, value: &str) -> TemporalPredicateRecord {
        TemporalPredicateRecord::SignalValue {
            signal_name: signal.to_string(),
            value: value.to_string(),
            phase: TickPhase::PreTick,
        }
    }

    fn drive(actor: &str, signal: &str) -> TemporalPredicateRecord {
        TemporalPredicateRecord::ActorDrivesSignal {
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            phase: TickPhase::PostTick,
        }
    }

    #[test]
    fn renders_multi_condition_validity_rule() {
        // PBUSER valid when PSEL, PENABLE, and PREADY are asserted (the real APB rule).
        let r = rule(
            vec![
                sv("PSEL", "ASSERTED"),
                sv("PENABLE", "ASSERTED"),
                sv("PREADY", "ASSERTED"),
            ],
            vec![drive("Completer", "PBUSER"), sv("PBUSER", "VALID")],
            None,
        );
        assert_eq!(
            temporal_rule_to_ltl(&r),
            "G( PSEL==ASSERTED & PENABLE==ASSERTED & PREADY==ASSERTED -> X (drive(Completer,PBUSER) & PBUSER==VALID) )"
        );
    }

    #[test]
    fn renders_single_condition_validity_rule() {
        // PNSE valid when PSEL asserted.
        let r = rule(
            vec![sv("PSEL", "ASSERTED")],
            vec![drive("Requester", "PNSE"), sv("PNSE", "VALID")],
            None,
        );
        assert_eq!(
            temporal_rule_to_ltl(&r),
            "G( PSEL==ASSERTED -> X (drive(Requester,PNSE) & PNSE==VALID) )"
        );
    }

    #[test]
    fn empty_antecedent_renders_as_an_invariant() {
        let r = rule(
            vec![],
            vec![TemporalPredicateRecord::SignalStable {
                signal_name: "PSTRB".to_string(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }],
            None,
        );
        assert_eq!(temporal_rule_to_ltl(&r), "G( stable(PSTRB) )");
    }

    #[test]
    fn cycle_window_renders_as_bounded_eventually() {
        let r = rule(
            vec![sv("PSEL", "ASSERTED")],
            vec![sv("PRDATA", "VALID")],
            Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(2),
            }),
        );
        // single-atom consequent → no parens; window → F[min,max].
        assert_eq!(
            temporal_rule_to_ltl(&r),
            "G( PSEL==ASSERTED -> F[1,2] PRDATA==VALID )"
        );
    }
}
