//! Multimodal contract fusion (R16-MULTIMODAL-CONTRACT-FUSION).
//!
//! Per the `R16-MULTIMODAL-CONTRACT-FUSION.1` design (see
//! `docs/tasks/R16-MULTIMODAL-CONTRACT-FUSION.md`): a deterministic
//! fusion phase that clusters multimodal `ActorContract` candidates by
//! a typed `FusionKey` and merges them into one. Agreement merges
//! preserve provenance (union of `supporting_statement_ids`, `Mixed`
//! modality, delimited `source_text`) and **corroborate** the
//! `automation_confidence` via Dempster's rule (`DEMPSTER-FUSION-COMBINER`:
//! independent agreement raises confidence rather than capping at the weakest).
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

/// Map an ordinal `AutomationConfidence` to a Dempster-Shafer belief mass in the
/// supported proposition (the remainder is uncertainty mass on the frame).
fn confidence_belief_mass(c: AutomationConfidence) -> f64 {
    match c {
        AutomationConfidence::High => 0.9,
        AutomationConfidence::Medium => 0.7,
        AutomationConfidence::Low => 0.5,
    }
}

/// Map a combined belief mass back onto the ordinal confidence scale.
fn belief_mass_confidence(mass: f64) -> AutomationConfidence {
    const EPS: f64 = 1e-9;
    if mass >= 0.9 - EPS {
        AutomationConfidence::High
    } else if mass >= 0.7 - EPS {
        AutomationConfidence::Medium
    } else {
        AutomationConfidence::Low
    }
}

/// Dempster's rule of combination (`DEMPSTER-FUSION-COMBINER`) for INDEPENDENT
/// sources that all support the SAME proposition — the agreement case in
/// `merge_cluster`. Disagreement is routed to a residual upstream, so the
/// conflict mass `K` is 0 on this path.
///
/// Combining agreeing belief masses CORROBORATES: the combined belief
/// `1 - ∏(1 - mᵢ)` is at least the strongest single source, so independent
/// agreement raises confidence instead of capping it at the weakest (the prior
/// `min` rule). E.g. Medium+Medium → `1 - 0.3·0.3 = 0.91` → High; Low+Low →
/// `0.75` → Medium; a single source is unchanged. Graded-conflict DS (`K > 0`
/// plus the Zadeh high-conflict guard) does not arise here and is a documented
/// future extension.
fn dempster_corroborate_confidence<I>(confidences: I) -> AutomationConfidence
where
    I: IntoIterator<Item = AutomationConfidence>,
{
    let mut uncertainty = 1.0_f64;
    let mut any = false;
    for c in confidences {
        any = true;
        uncertainty *= 1.0 - confidence_belief_mass(c);
    }
    if !any {
        return AutomationConfidence::Low;
    }
    belief_mass_confidence(1.0 - uncertainty)
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
        // Disagreeing sources route to a residual; keep the conservative `min`
        // confidence folded above — conflict must not be "corroborated".
    } else {
        // DEMPSTER-FUSION-COMBINER: independent sources that AGREE corroborate.
        // Dempster's rule raises the combined belief above the weakest source,
        // instead of the prior `min` cap.
        base.automation_confidence =
            dempster_corroborate_confidence(cluster.iter().map(|c| c.automation_confidence));
    }

    // Synthetic stable id for the merged contract: "fused:<ids>".
    let ids: Vec<&str> = cluster.iter().map(|c| c.contract_id.as_str()).collect();
    base.contract_id = format!("fused:{}", ids.join("+"));
    base
}

/// Producer wiring (R16-MULTIMODAL-CONTRACT-FUSION.3): cluster
/// `contracts` by `fusion_key` and replace each cluster of size > 1
/// with a single `merge_cluster` result. Insertion order of the
/// resulting vec is preserved (the merged contract takes the slot of
/// the first cluster member; the trailing members are dropped).
/// Single-element clusters are unchanged. Idempotent on already-fused
/// input (a `merge_cluster` of a single contract is identity).
///
/// Designed to run in `SemanticIr::build` BEFORE
/// `apply_fidelity_gates` so the fidelity gates evaluate the fused
/// contracts (not the pre-fusion duplicates).
pub fn apply_fusion(contracts: &mut Vec<ActorContract>) {
    if contracts.len() < 2 {
        return;
    }
    use std::collections::HashMap;
    let mut groups: HashMap<FusionKey, Vec<usize>> = HashMap::new();
    let mut order: Vec<FusionKey> = Vec::new();
    for (i, c) in contracts.iter().enumerate() {
        let k = fusion_key(c);
        if !groups.contains_key(&k) {
            order.push(k.clone());
        }
        groups.entry(k).or_default().push(i);
    }
    let has_multi = order.iter().any(|k| groups[k].len() > 1);
    if !has_multi {
        return;
    }
    let mut replacements: HashMap<usize, ActorContract> = HashMap::new();
    let mut to_skip: std::collections::HashSet<usize> = std::collections::HashSet::new();
    for k in &order {
        let idxs = &groups[k];
        if idxs.len() < 2 {
            continue;
        }
        let cluster: Vec<ActorContract> = idxs.iter().map(|&i| contracts[i].clone()).collect();
        let merged = merge_cluster(&cluster);
        replacements.insert(idxs[0], merged);
        for &i in &idxs[1..] {
            to_skip.insert(i);
        }
    }
    let mut new_contracts = Vec::with_capacity(contracts.len() - to_skip.len());
    for (i, c) in contracts.iter().enumerate() {
        if let Some(m) = replacements.remove(&i) {
            new_contracts.push(m);
        } else if !to_skip.contains(&i) {
            new_contracts.push(c.clone());
        }
    }
    *contracts = new_contracts;
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
        // DEMPSTER-FUSION-COMBINER: two independent agreeing sources corroborate —
        // High + Medium combine to High (1 - 0.1·0.3 = 0.97), not the old `min` (Medium).
        assert_eq!(merged.automation_confidence, AutomationConfidence::High);
        assert_eq!(merged.contract_id, "fused:c1+c2");
    }

    // DEMPSTER-FUSION-COMBINER: corroboration-boosting confidence fusion

    #[test]
    fn dempster_corroborate_two_medium_is_high() {
        // 1 - (1-0.7)(1-0.7) = 0.91 -> High
        assert_eq!(
            dempster_corroborate_confidence([
                AutomationConfidence::Medium,
                AutomationConfidence::Medium,
            ]),
            AutomationConfidence::High
        );
    }

    #[test]
    fn dempster_corroborate_two_low_is_medium() {
        // 1 - (0.5)(0.5) = 0.75 -> Medium
        assert_eq!(
            dempster_corroborate_confidence(
                [AutomationConfidence::Low, AutomationConfidence::Low,]
            ),
            AutomationConfidence::Medium
        );
    }

    #[test]
    fn dempster_corroborate_single_source_is_identity() {
        for c in [
            AutomationConfidence::High,
            AutomationConfidence::Medium,
            AutomationConfidence::Low,
        ] {
            assert_eq!(dempster_corroborate_confidence([c]), c);
        }
    }

    #[test]
    fn dempster_corroborate_high_caps_and_is_order_independent() {
        assert_eq!(
            dempster_corroborate_confidence([
                AutomationConfidence::High,
                AutomationConfidence::Low,
            ]),
            AutomationConfidence::High
        );
        assert_eq!(
            dempster_corroborate_confidence([
                AutomationConfidence::Low,
                AutomationConfidence::High,
            ]),
            AutomationConfidence::High
        );
    }

    #[test]
    fn merge_agreement_corroborates_two_medium_to_high() {
        let make = |id: &str, conf| {
            contract(
                id,
                Some("A"),
                Obligation::Drive {
                    signal: "Q".into(),
                    value: "1".into(),
                },
                ContractKind::Guarantee,
                id,
                EvidenceModality::Prose,
                id,
                LoweringDisposition::Lowerable,
                conf,
            )
        };
        // Two independent Medium sources that AGREE -> corroborate to High.
        let merged = merge_cluster(&[
            make("c1", AutomationConfidence::Medium),
            make("c2", AutomationConfidence::Medium),
        ]);
        assert!(matches!(merged.lowering, LoweringDisposition::Lowerable));
        assert_eq!(merged.automation_confidence, AutomationConfidence::High);
    }

    #[test]
    fn merge_disagreement_keeps_conservative_confidence() {
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
            Obligation::Drive {
                signal: "Q".into(),
                value: "0".into(), // disagrees on value
            },
            ContractKind::Guarantee,
            "table",
            EvidenceModality::Table,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Low,
        );
        let merged = merge_cluster(&[c1, c2]);
        // Disagreement -> Residual, conservative `min` confidence (NOT corroborated).
        assert!(matches!(
            merged.lowering,
            LoweringDisposition::Residual { .. }
        ));
        assert_eq!(merged.automation_confidence, AutomationConfidence::Low);
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

    #[test]
    fn apply_fusion_size_one_input_is_unchanged() {
        let mut cs = vec![contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "s",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::High,
        )];
        let before = cs.clone();
        apply_fusion(&mut cs);
        assert_eq!(cs, before);
    }

    #[test]
    fn apply_fusion_merges_a_multi_cluster_and_keeps_order_of_singletons() {
        // Three contracts: c1 & c3 cluster (same key); c2 alone.
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
            Some("B"), // different actor ⇒ different key
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose",
            EvidenceModality::Prose,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        let c3 = contract(
            "c3",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "table",
            EvidenceModality::Table,
            "s3",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        let mut cs = vec![c1, c2.clone(), c3];
        apply_fusion(&mut cs);
        assert_eq!(cs.len(), 2, "c1+c3 should fuse to one; c2 stays");
        // Merged contract takes c1's slot; c2 stays at index 1.
        assert_eq!(cs[0].contract_id, "fused:c1+c3");
        assert_eq!(cs[0].provenance.modality, EvidenceModality::Mixed);
        assert_eq!(cs[1], c2);
    }

    #[test]
    fn apply_fusion_is_idempotent_on_already_fused() {
        let mut cs = vec![contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "prose | table",
            EvidenceModality::Mixed,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        )];
        let before = cs.clone();
        apply_fusion(&mut cs);
        apply_fusion(&mut cs);
        assert_eq!(cs, before);
    }

    #[test]
    fn merge_cluster_unions_and_dedups_guard_candidates() {
        // Agreement merge unions guard_candidates across the cluster while
        // de-duplicating shared candidates (no double-count).
        let shared = Condition::Eq {
            signal: "g_shared".into(),
            value: "1".into(),
        };
        let only1 = Condition::Eq {
            signal: "g_only1".into(),
            value: "1".into(),
        };
        let only2 = Condition::Eq {
            signal: "g_only2".into(),
            value: "1".into(),
        };
        let mut c1 = contract(
            "c1",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "p1",
            EvidenceModality::Prose,
            "s1",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        c1.guard_candidates = vec![shared.clone(), only1.clone()];
        let mut c2 = contract(
            "c2",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            "p2",
            EvidenceModality::Prose,
            "s2",
            LoweringDisposition::Lowerable,
            AutomationConfidence::Medium,
        );
        c2.guard_candidates = vec![shared.clone(), only2.clone()];
        let merged = merge_cluster(&[c1, c2]);
        // Union, deduped, order-preserved: `shared` appears exactly once.
        assert_eq!(merged.guard_candidates, vec![shared, only1, only2]);
    }

    #[test]
    fn apply_fusion_leaves_distinct_key_contracts_unchanged() {
        // No cluster of size > 1 (all distinct fusion keys) -> the has_multi
        // early-exit returns the input untouched (no "fused:" merge).
        let mk = |id: &str, actor: &str, sig: &str| {
            contract(
                id,
                Some(actor),
                Obligation::Drive {
                    signal: sig.into(),
                    value: "1".into(),
                },
                ContractKind::Guarantee,
                "p",
                EvidenceModality::Prose,
                "s",
                LoweringDisposition::Lowerable,
                AutomationConfidence::Medium,
            )
        };
        let c1 = mk("c1", "A", "P");
        let c2 = mk("c2", "B", "Q");
        let c3 = mk("c3", "C", "R");
        let mut v = vec![c1.clone(), c2.clone(), c3.clone()];
        apply_fusion(&mut v);
        assert_eq!(v, vec![c1, c2, c3]);
        assert!(v.iter().all(|c| !c.contract_id.starts_with("fused:")));
    }
}
