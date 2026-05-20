//! Multimodal contract fusion (R16-MULTIMODAL-CONTRACT-FUSION).
//!
//! Per the `R16-MULTIMODAL-CONTRACT-FUSION.1` design (see
//! `docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md`): a deterministic
//! fusion phase that clusters multimodal `ActorContract` candidates by
//! a typed `FusionKey` and merges them into one. Agreement merges
//! preserve provenance (union of `supporting_statement_ids`, `Mixed`
//! modality, delimited `source_text`, minimum `automation_confidence`).
//! **Disagreement** (incompatible obligation / guard / kind) sets the
//! fused contract's `lowering = Residual{reason="disagreement: …"}` —
//! the honesty doctrine, mechanically enforced (never a silent pick;
//! parallel to `R16-CAPTURE-FIDELITY-GATES.3` Fail-to-Residual
//! routing).
//!
//! `.2` scope (this module): the typed model + `merge_cluster`
//! primitive + unit tests on agreement / disagreement / size-1
//! identity paths. The producer that runs `merge_cluster` in
//! `SemanticIr::build` (before the fidelity gate) is `.3`. The
//! `validate fusion:` count surface is `.4`.

use crate::ir::contract::{
    ActorContract, EventExpr, EvidenceModality, LoweringDisposition, Obligation,
};
use crate::ir::source::AutomationConfidence;

/// The cluster key. Two contracts with equal `FusionKey` are candidates
/// for merging. `channel`/`phase` are populated by the extraction
/// trees (`R16-KG-PROTOCOL-ONTOLOGY` / `#4`/`#6`); until then most
/// contracts cluster at size 1 ⇒ fusion is identity (zero artifact
/// churn).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FusionKey {
    pub actor: Option<String>,
    pub channel: Option<String>,
    pub phase: Option<String>,
    pub obligation_kind: &'static str,
    pub primary_signal: Option<String>,
}

/// Discriminator name for the contract's obligation — used as part of
/// `FusionKey` so two contracts with different obligation shapes never
/// cluster (an `Eventually` is not the same protocol observation as a
/// `Stable`, even if every other field aligned).
pub fn obligation_kind(o: &Obligation) -> &'static str {
    match o {
        Obligation::Eventually { .. } => "eventually",
        Obligation::Stable { .. } => "stable",
        Obligation::Drive { .. } => "drive",
        Obligation::HandshakeBarrier { .. } => "handshake_barrier",
        Obligation::Persist { .. } => "persist",
        Obligation::Sequence { .. } => "sequence",
        Obligation::Mutex { .. } => "mutex",
        Obligation::OrderedBefore { .. } => "ordered_before",
        Obligation::Observe { .. } => "observe",
    }
}

/// The "subject" signal of an obligation. Mirrors fidelity.rs's helper
/// (intentional duplication — the helper is small and module
/// independence avoids a fidelity↔fusion coupling for a one-liner).
fn obligation_primary_signal(o: &Obligation) -> Option<String> {
    match o {
        Obligation::Eventually { target, .. } | Obligation::Persist { hold: target, .. } => {
            match target {
                EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => {
                    Some(signal.clone())
                }
                EventExpr::HandshakeFire { valid, .. } => Some(valid.clone()),
                EventExpr::Start | EventExpr::PhaseBoundary { .. } => None,
            }
        }
        Obligation::Stable { signal, .. }
        | Obligation::Drive { signal, .. }
        | Obligation::Observe { signal } => Some(signal.clone()),
        Obligation::HandshakeBarrier { valid, .. } => Some(valid.clone()),
        Obligation::Sequence { steps } => steps.first().and_then(|s| match &s.event {
            EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => {
                Some(signal.clone())
            }
            EventExpr::HandshakeFire { valid, .. } => Some(valid.clone()),
            EventExpr::Start | EventExpr::PhaseBoundary { .. } => None,
        }),
        Obligation::Mutex { a, .. } => Some(a.clone()),
        Obligation::OrderedBefore { .. } => None,
    }
}

/// Derive the `FusionKey` for a single contract.
pub fn fusion_key(c: &ActorContract) -> FusionKey {
    FusionKey {
        actor: c.actor_name.clone(),
        channel: c.channel.clone(),
        phase: c.phase.clone(),
        obligation_kind: obligation_kind(&c.obligation),
        primary_signal: obligation_primary_signal(&c.obligation),
    }
}

/// `min` over `AutomationConfidence` (High > Medium > Low) — used as
/// the conservative pick when merging multiple sources.
fn min_confidence(a: AutomationConfidence, b: AutomationConfidence) -> AutomationConfidence {
    let rank = |c: AutomationConfidence| match c {
        AutomationConfidence::High => 2u8,
        AutomationConfidence::Medium => 1,
        AutomationConfidence::Low => 0,
    };
    if rank(a) <= rank(b) { a } else { b }
}

/// Deterministically merge a cluster of contracts that share a
/// `FusionKey`. Single-element clusters return unchanged. Agreement
/// merges preserve provenance; disagreement (incompatible obligation
/// / guard / kind) sets `lowering = Residual{reason="disagreement: …"}`.
///
/// Panics if `cluster` is empty — the producer should never call this
/// with an empty group.
pub fn merge_cluster(cluster: &[ActorContract]) -> ActorContract {
    assert!(
        !cluster.is_empty(),
        "merge_cluster called with empty cluster"
    );
    if cluster.len() == 1 {
        return cluster[0].clone();
    }
    let mut base = cluster[0].clone();
    let mut disagreement_fields: Vec<&'static str> = Vec::new();
    for c in &cluster[1..] {
        if c.kind != base.kind {
            disagreement_fields.push("kind");
        }
        if c.obligation != base.obligation {
            disagreement_fields.push("obligation");
        }
        if c.guard != base.guard {
            disagreement_fields.push("guard");
        }

        for cand in &c.guard_candidates {
            if !base.guard_candidates.contains(cand) {
                base.guard_candidates.push(cand.clone());
            }
        }
        for sid in &c.provenance.supporting_statement_ids {
            if !base.provenance.supporting_statement_ids.contains(sid) {
                base.provenance.supporting_statement_ids.push(sid.clone());
            }
        }
        if c.provenance.modality != base.provenance.modality {
            base.provenance.modality = EvidenceModality::Mixed;
        }
        if base.provenance.source_text != c.provenance.source_text
            && !base
                .provenance
                .source_text
                .split(" | ")
                .any(|piece| piece == c.provenance.source_text)
        {
            base.provenance.source_text = format!(
                "{} | {}",
                base.provenance.source_text, c.provenance.source_text
            );
        }
        base.automation_confidence =
            min_confidence(base.automation_confidence, c.automation_confidence);
    }

    if !disagreement_fields.is_empty() {
        disagreement_fields.sort();
        disagreement_fields.dedup();
        base.lowering = LoweringDisposition::Residual {
            reason: format!("disagreement: {}", disagreement_fields.join(",")),
        };
    }

    // Synthetic stable id for the merged contract: "fused:<ids>".
    let ids: Vec<&str> = cluster.iter().map(|c| c.contract_id.as_str()).collect();
    base.contract_id = format!("fused:{}", ids.join("+"));
    base
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::contract::{Condition, ContractKind, ContractProvenance, Window};
    use crate::ir::semantic::ClockEdge;

    #[allow(clippy::too_many_arguments)]
    fn contract(
        id: &str,
        actor: Option<&str>,
        obligation: Obligation,
        kind: ContractKind,
        source_text: &str,
        modality: EvidenceModality,
        statement_id: &str,
        lowering: LoweringDisposition,
        confidence: AutomationConfidence,
    ) -> ActorContract {
        ActorContract {
            contract_id: id.into(),
            source_rule_id: Some(id.into()),
            actor_name: actor.map(|s| s.to_string()),
            kind,
            guard: None,
            guard_candidates: vec![],
            obligation,
            clock_signal: None,
            edge: ClockEdge::Rising,
            channel: None,
            phase: None,
            provenance: ContractProvenance {
                supporting_statement_ids: vec![statement_id.into()],
                source_text: source_text.into(),
                modality,
            },
            lowering,
            automation_confidence: confidence,
        }
    }

    #[test]
    fn fusion_key_groups_by_actor_obligation_kind_primary_signal() {
        let c1 = contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        let c2 = contract(
            "c2",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "table",
            EvidenceModality::Table,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        assert_eq!(fusion_key(&c1), fusion_key(&c2));

        let c3 = contract(
            "c3",
            Some("A"),
            Obligation::Drive {
                signal: "R".into(),
                value: "1".into(),
            }, // different primary signal
            ContractKind::Guarantee,
            "prose",
            EvidenceModality::Prose,
            "s3",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        assert_ne!(fusion_key(&c1), fusion_key(&c3));

        let c4 = contract(
            "c4",
            Some("B"), // different actor
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose",
            EvidenceModality::Prose,
            "s4",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        assert_ne!(fusion_key(&c1), fusion_key(&c4));
    }

    #[test]
    fn merge_size_one_is_identity() {
        let c1 = contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "src",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::High,
        );
        let merged = merge_cluster(std::slice::from_ref(&c1));
        assert_eq!(merged, c1);
    }

    #[test]
    fn merge_agreement_unions_provenance_and_marks_mixed() {
        let c1 = contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose §3.1",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::High,
        );
        let c2 = contract(
            "c2",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "table §3.4",
            EvidenceModality::Table,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        let merged = merge_cluster(&[c1, c2]);
        assert!(matches!(merged.lowering, LoweringDisposition::Lowerable));
        assert_eq!(merged.provenance.modality, EvidenceModality::Mixed);
        assert_eq!(merged.provenance.supporting_statement_ids, vec!["s1", "s2"]);
        assert_eq!(merged.provenance.source_text, "prose §3.1 | table §3.4");
        // Min automation_confidence is the conservative pick.
        assert_eq!(merged.automation_confidence, AutomationConfidence::Medium);
        assert_eq!(merged.contract_id, "fused:c1+c2");
    }

    #[test]
    fn merge_disagreement_routes_to_residual_with_reason() {
        let c1 = contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::High,
        );
        let c2 = contract(
            "c2",
            Some("A"),
            // Same FusionKey (actor=A, obligation_kind=drive, primary=Q)
            // but disagrees on the value:
            Obligation::Drive {
                signal: "Q".into(),
                value: "0".into(),
            },
            ContractKind::Guarantee,
            "table",
            EvidenceModality::Table,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        // They cluster (same key) and disagree on obligation:
        assert_eq!(fusion_key(&c1), fusion_key(&c2));
        let merged = merge_cluster(&[c1, c2]);
        match &merged.lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(reason.contains("disagreement:"), "{reason}");
                assert!(reason.contains("obligation"), "{reason}");
            }
            other => panic!("expected Residual disagreement, got {other:?}"),
        }
        // Provenance is still unioned even on disagreement.
        assert_eq!(merged.provenance.supporting_statement_ids, vec!["s1", "s2"]);
        assert_eq!(merged.provenance.modality, EvidenceModality::Mixed);
    }

    #[test]
    fn merge_disagreement_lists_all_diff_fields_sorted_deduped() {
        let c1 = contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        let mut c2 = contract(
            "c2",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "0".into(),
            }, // obligation diff
            ContractKind::Assume, // kind diff
            "table",
            EvidenceModality::Table,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        c2.guard = Some(Condition::Eq {
            signal: "EN".into(),
            value: "1".into(),
        }); // guard diff
        let merged = merge_cluster(&[c1, c2]);
        match &merged.lowering {
            LoweringDisposition::Residual { reason } => {
                // sorted alphabetically: guard, kind, obligation
                assert_eq!(reason, "disagreement: guard,kind,obligation");
            }
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn unsupported_window_round_trips_through_obligation_kind() {
        let c = contract(
            "c",
            None,
            Obligation::Stable {
                signal: "D".into(),
                during: Window::SameCycle,
            },
            ContractKind::Assume,
            "s",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        assert_eq!(obligation_kind(&c.obligation), "stable");
        assert_eq!(fusion_key(&c).obligation_kind, "stable");
    }
}
