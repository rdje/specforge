//! CORPUS-PATTERN-REUSE.2 — derived vendor/layout fingerprint + unsupervised clustering.
//!
//! The owner's aim (`2026-06-09`): recognize that a new chip-spec PDF "is like ones we've seen" — especially
//! within a vendor/brand whose documents share organization — so extraction patterns can be reused. The
//! ADR-0006-safe way to exploit that WITHOUT hardcoding any vendor name is to cluster documents by a
//! **derived structural fingerprint**, never by a baked-in "ARM"/"NXP" string. The cluster key is the shared
//! structure itself.
//!
//! The fingerprint is built from the document's OWN [`EvidenceIr`]:
//! - **structural shape** — a coarse count bucket per typed surface (registers, protocol states, signal
//!   constraints, relations, actors, serial-frame fields, conditional rules). A spec that is register-heavy
//!   and FSM-less fingerprints differently from a constraint-heavy protocol. Available on every document.
//! - **behavioral shape** — which extractor strategies actually FIRED, from the
//!   [`crate::ir::extractor::ExtractionManifest`] (`EXTRACTOR-ARCHITECTURE.8`). Two specs whose facts came
//!   from the same strategies (e.g. register fields from `field_table`, signal meaning from `prose`) tend to
//!   be organised the same way. Present on documents (re)built since `.8`.
//!
//! Each feature is a short token; a document's fingerprint is the SET of its tokens. Similarity is the
//! Jaccard overlap of two token sets, and clustering is a deterministic greedy agglomeration. Nothing here is
//! a vendor name — the emergent clusters are *defined by* shared structure, which is the honest, generic way
//! to capture "documents from the same source tend to look alike".
//!
//! This module is the clustering CAPABILITY; consuming a cluster's learned `ExtractionProfile` as advisory
//! priors (`CORPUS-PATTERN-REUSE.3`) and an offline LLM cluster-miner (`.4`) build on top of it.

use crate::ir::evidence::EvidenceIr;
use std::collections::BTreeSet;

/// A document's derived fingerprint: an ADR-0006-safe SET of structural + behavioral feature tokens. Two
/// documents are similar when their token sets overlap (Jaccard). No vendor/chip names — only shape.
pub type DocumentFingerprint = BTreeSet<String>;

/// Coarse, order-of-magnitude bucket for a surface's record count, so "44 registers" and "60 registers"
/// fingerprint alike (both "lots") while "0" and "3" stay distinct. Buckets, not raw counts, keep the
/// fingerprint robust to incidental count differences between same-family documents.
fn count_bucket(n: usize) -> &'static str {
    match n {
        0 => "b0",
        1..=9 => "b1",
        10..=99 => "b2",
        _ => "b3",
    }
}

/// Derive a document's fingerprint from its own `EvidenceIr` — structural count-buckets per surface plus the
/// extractors that fired (`EXTRACTOR-ARCHITECTURE.8` manifest). Pure + deterministic; no vendor names.
pub fn document_fingerprint(ir: &EvidenceIr) -> DocumentFingerprint {
    let mut features = BTreeSet::new();
    // Structural shape — coarse count bucket per typed surface (available on every document).
    let shape: [(&str, usize); 7] = [
        ("registers", ir.register_records.len()),
        ("protocol_states", ir.protocol_states.len()),
        ("signal_constraints", ir.signal_constraints.len()),
        ("relations", ir.actor_signal_relations.len()),
        ("actors", ir.protocol_actors.len()),
        ("serial_frame", ir.serial_frame_fields.len()),
        ("conditional_rules", ir.conditional_rules.len()),
    ];
    for (name, count) in shape {
        features.insert(format!("shape:{name}:{}", count_bucket(count)));
    }
    // Behavioral shape — which extractor strategies actually fired (produced ≥1 kept record). Present on
    // documents (re)built since `.8`; absent (silently) on older artifacts, which then cluster on shape alone.
    for surface in &ir.extraction_manifest.surfaces {
        for entry in &surface.entries {
            if entry.eligible && entry.kept > 0 {
                features.insert(format!("fired:{}", entry.name));
            }
        }
    }
    features
}

/// Jaccard similarity of two fingerprints: `|a ∩ b| / |a ∪ b|`, in `[0.0, 1.0]`. Two empty fingerprints are
/// defined as `1.0` (identical), matching the set-theoretic identity for empty sets.
pub fn fingerprint_similarity(a: &DocumentFingerprint, b: &DocumentFingerprint) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let intersection = a.intersection(b).count();
    let union = a.union(b).count();
    if union == 0 {
        return 0.0;
    }
    intersection as f64 / union as f64
}

/// An emergent cluster of documents with a similar fingerprint, plus the feature tokens they all share (the
/// cluster's structural "signature" — what makes these documents alike, in vendor-name-free terms).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentCluster {
    /// Member document keys, in the order they were attached (the first is the cluster representative).
    pub members: Vec<String>,
    /// The intersection of all members' fingerprints — the features every member shares.
    pub shared_features: DocumentFingerprint,
}

/// Cluster documents by fingerprint similarity, unsupervised. Deterministic greedy agglomeration: documents
/// are processed in key order (so the result never depends on input order or hash iteration); each is
/// attached to the FIRST existing cluster whose representative (its first member) is at least `threshold`
/// similar, otherwise it opens a new cluster. `shared_features` is recomputed as the running intersection.
///
/// Greedy single-link-to-representative clustering is intentionally simple + explainable (no opaque ML, no
/// dependency): the goal is to surface that same-organisation documents fall together, which a human can
/// then read off the shared signature. `threshold` in `[0.0, 1.0]` (e.g. `0.6`) tunes how alike members must be.
pub fn cluster_documents(
    documents: &[(String, DocumentFingerprint)],
    threshold: f64,
) -> Vec<DocumentCluster> {
    // Process in key order for determinism (independent of caller order / hash iteration).
    let mut ordered: Vec<&(String, DocumentFingerprint)> = documents.iter().collect();
    ordered.sort_by(|a, b| a.0.cmp(&b.0));

    let mut clusters: Vec<DocumentCluster> = Vec::new();
    let mut representatives: Vec<DocumentFingerprint> = Vec::new();
    for (key, fingerprint) in ordered {
        let mut attached = false;
        for (idx, rep) in representatives.iter().enumerate() {
            if fingerprint_similarity(rep, fingerprint) >= threshold {
                let cluster = &mut clusters[idx];
                cluster.members.push(key.clone());
                cluster.shared_features = cluster
                    .shared_features
                    .intersection(fingerprint)
                    .cloned()
                    .collect();
                attached = true;
                break;
            }
        }
        if !attached {
            representatives.push(fingerprint.clone());
            clusters.push(DocumentCluster {
                members: vec![key.clone()],
                shared_features: fingerprint.clone(),
            });
        }
    }
    clusters
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fp(tokens: &[&str]) -> DocumentFingerprint {
        tokens.iter().map(|t| t.to_string()).collect()
    }

    #[test]
    fn count_buckets_are_order_of_magnitude() {
        assert_eq!(count_bucket(0), "b0");
        assert_eq!(count_bucket(3), "b1");
        assert_eq!(count_bucket(44), "b2");
        assert_eq!(count_bucket(250), "b3");
    }

    #[test]
    fn jaccard_similarity_is_intersection_over_union() {
        let a = fp(&["x", "y", "z"]);
        let b = fp(&["y", "z", "w"]);
        // |∩| = 2 (y,z), |∪| = 4 (w,x,y,z) → 0.5
        assert!((fingerprint_similarity(&a, &b) - 0.5).abs() < 1e-9);
        assert!((fingerprint_similarity(&a, &a) - 1.0).abs() < 1e-9);
        assert!((fingerprint_similarity(&a, &fp(&["p", "q"]))).abs() < 1e-9);
        // two empty fingerprints are identical, not undefined
        assert!((fingerprint_similarity(&fp(&[]), &fp(&[])) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn similar_documents_cluster_and_dissimilar_split() {
        // Two register-heavy, field-table docs (alike) + one constraint-heavy protocol (different).
        let reg_a = fp(&[
            "shape:registers:b2",
            "shape:protocol_states:b0",
            "fired:registers.field_table",
        ]);
        let reg_b = fp(&[
            "shape:registers:b2",
            "shape:protocol_states:b0",
            "fired:registers.field_table",
        ]);
        let proto = fp(&[
            "shape:registers:b0",
            "shape:signal_constraints:b2",
            "fired:semantic_hints.prose",
        ]);
        let docs = vec![
            ("doc_reg_a".to_string(), reg_a),
            ("doc_proto".to_string(), proto),
            ("doc_reg_b".to_string(), reg_b),
        ];
        let clusters = cluster_documents(&docs, 0.6);
        assert_eq!(
            clusters.len(),
            2,
            "the two register docs cluster; the protocol stands alone"
        );
        // The register cluster has both register docs and a non-empty shared signature.
        let reg_cluster = clusters
            .iter()
            .find(|c| c.members.contains(&"doc_reg_a".to_string()))
            .unwrap();
        assert!(reg_cluster.members.contains(&"doc_reg_b".to_string()));
        assert!(
            reg_cluster
                .shared_features
                .contains("fired:registers.field_table")
        );
    }

    #[test]
    fn clustering_is_deterministic_regardless_of_input_order() {
        let x = fp(&["shape:registers:b2", "fired:registers.field_table"]);
        let y = fp(&["shape:registers:b2", "fired:registers.field_table"]);
        let z = fp(&["shape:signal_constraints:b2"]);
        let order1 = vec![
            ("a".to_string(), x.clone()),
            ("b".to_string(), y.clone()),
            ("c".to_string(), z.clone()),
        ];
        let order2 = vec![
            ("c".to_string(), z),
            ("b".to_string(), y),
            ("a".to_string(), x),
        ];
        assert_eq!(
            cluster_documents(&order1, 0.6),
            cluster_documents(&order2, 0.6),
            "key-sorted greedy clustering is independent of input order"
        );
    }
}
