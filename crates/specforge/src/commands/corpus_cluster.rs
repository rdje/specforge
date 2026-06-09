//! CORPUS-PATTERN-REUSE.3a — the `corpus-cluster` command.
//!
//! Surfaces the `CORPUS-PATTERN-REUSE.2` clustering capability ([`crate::ir::corpus_cluster`]) as a
//! first-class, user-facing command. It walks the persisted `generated/evidence_ir/<doc_key>/evidence_ir.json`
//! corpus, derives each document's ADR-0006-safe structural+behavioral [`DocumentFingerprint`], clusters the
//! documents by fingerprint similarity, and reports the emergent families — their members and the shared
//! structural signature that makes them alike — so the owner can SEE the corpus structure the cross-document
//! reuse plane (`.3b`/`.4`) will exploit.
//!
//! This is deliberately READ-ONLY and additive: it neither rebuilds nor mutates any IR, and it changes no
//! extraction behavior. The cluster key is the shared *structure*, never a baked-in vendor name (ADR 0006);
//! the emergent families are defined by what the documents themselves look like.

use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::CorpusClusterArgs;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::corpus_cluster::{
    ClusterExtractionProfile, DocumentCluster, DocumentFingerprint, cluster_documents,
    derive_extraction_profiles,
};
use crate::ir::evidence::EvidenceIr;
use std::collections::BTreeMap;

/// An evidence artifact that was found but could not contribute a fingerprint, with the honest reason.
#[derive(Debug, Clone, PartialEq, Eq)]
struct SkippedArtifact {
    artifact_path: PathBuf,
    reason: String,
}

/// The fingerprinted corpus plus any artifacts skipped while loading.
#[derive(Debug, Clone)]
struct LoadedCorpus {
    documents: Vec<(String, DocumentFingerprint)>,
    skipped: Vec<SkippedArtifact>,
}

/// The full, inspectable outcome of clustering the persisted corpus. Pure data so the formatting and the
/// summary counts are testable without touching the filesystem.
#[derive(Debug, Clone, PartialEq)]
struct CorpusClusterReport {
    threshold: f64,
    documents_loaded: usize,
    skipped: Vec<SkippedArtifact>,
    /// Clusters ordered for display: largest first, ties broken by representative key (deterministic).
    clusters: Vec<DocumentCluster>,
    /// Advisory per-cluster extraction profiles (`.3b.1`), keyed for lookup by cluster representative.
    profiles: Vec<ClusterExtractionProfile>,
}

impl CorpusClusterReport {
    fn multi_document_clusters(&self) -> usize {
        self.clusters
            .iter()
            .filter(|cluster| cluster.members.len() > 1)
            .count()
    }

    fn single_document_clusters(&self) -> usize {
        self.clusters
            .iter()
            .filter(|cluster| cluster.members.len() == 1)
            .count()
    }
}

pub fn run(args: CorpusClusterArgs) -> Result<()> {
    let loaded = collect_corpus_documents(&args.evidence_root)?;
    let report = build_corpus_cluster_report(loaded, args.threshold);

    print!("{}", render_report(&report, &args.evidence_root));
    Ok(())
}

/// Walk `<evidence_root>/<doc_key>/evidence_ir.json`, loading every readable EvidenceIR and deriving its
/// fingerprint. Directories without an `evidence_ir.json` are simply not corpus members (not skipped); an
/// `evidence_ir.json` that fails to load or is the wrong stage is recorded as an honest skip with its reason.
fn collect_corpus_documents(evidence_root: &Path) -> Result<LoadedCorpus> {
    if !evidence_root.exists() {
        return Err(AppError::MissingPath(evidence_root.to_path_buf()));
    }

    // Sort directory entries so the skip list (and any tie-breaking) is independent of filesystem order.
    let mut entries: Vec<PathBuf> = fs::read_dir(evidence_root)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect();
    entries.sort();

    let mut documents = Vec::new();
    let mut skipped = Vec::new();
    for dir in entries {
        let artifact_path = dir.join("evidence_ir.json");
        if !artifact_path.exists() {
            continue;
        }

        match EvidenceIr::load_from_path(&artifact_path) {
            Ok(evidence_ir) if matches!(evidence_ir.stage, IrStage::EvidenceIr) => {
                let fingerprint = crate::ir::corpus_cluster::document_fingerprint(&evidence_ir);
                documents.push((
                    evidence_ir.document_identity.document_key.clone(),
                    fingerprint,
                ));
            }
            Ok(evidence_ir) => skipped.push(SkippedArtifact {
                artifact_path,
                reason: format!(
                    "not an evidence_ir artifact (stage: {})",
                    evidence_ir.stage.as_str()
                ),
            }),
            Err(error) => skipped.push(SkippedArtifact {
                artifact_path,
                reason: format!("failed to load: {error}"),
            }),
        }
    }

    Ok(LoadedCorpus { documents, skipped })
}

/// Cluster the fingerprinted corpus and order the clusters for display (largest first, ties broken by
/// representative key). Pure — no I/O — so ordering and summary counts are testable in isolation.
fn build_corpus_cluster_report(loaded: LoadedCorpus, threshold: f64) -> CorpusClusterReport {
    let documents_loaded = loaded.documents.len();
    let profiles = derive_extraction_profiles(&loaded.documents, threshold);
    let mut clusters = cluster_documents(&loaded.documents, threshold);
    // Display order: most-shared families first; ties by representative (first member) for determinism.
    clusters.sort_by(|a, b| {
        b.members
            .len()
            .cmp(&a.members.len())
            .then_with(|| a.members.first().cmp(&b.members.first()))
    });

    CorpusClusterReport {
        threshold,
        documents_loaded,
        skipped: loaded.skipped,
        clusters,
        profiles,
    }
}

/// Render the report as a deterministic, human-readable summary: header counts, then each multi-document
/// family with its shared structural signature and members, then the single-document (unique-shape) docs.
fn render_report(report: &CorpusClusterReport, evidence_root: &Path) -> String {
    let mut out = String::new();
    out.push_str("command: corpus-cluster\n");
    out.push_str(&format!("evidence_root: {}\n", evidence_root.display()));
    out.push_str(&format!("threshold: {:.2}\n", report.threshold));
    out.push_str(&format!("documents_loaded: {}\n", report.documents_loaded));
    out.push_str(&format!("documents_skipped: {}\n", report.skipped.len()));
    out.push_str(&format!("clusters_total: {}\n", report.clusters.len()));
    out.push_str(&format!(
        "multi_document_clusters: {}\n",
        report.multi_document_clusters()
    ));
    out.push_str(&format!(
        "single_document_clusters: {}\n",
        report.single_document_clusters()
    ));

    // Advisory profile lookup by cluster representative (first member) — `.3b.1`.
    let profile_by_representative: BTreeMap<&str, &ClusterExtractionProfile> = report
        .profiles
        .iter()
        .filter_map(|profile| {
            profile
                .members
                .first()
                .map(|representative| (representative.as_str(), profile))
        })
        .collect();

    let multi: Vec<&DocumentCluster> = report
        .clusters
        .iter()
        .filter(|cluster| cluster.members.len() > 1)
        .collect();
    if !multi.is_empty() {
        out.push_str("\nmulti-document families (shared structural signature):\n");
        for (index, cluster) in multi.iter().enumerate() {
            let signature = if cluster.shared_features.is_empty() {
                "(none)".to_string()
            } else {
                cluster
                    .shared_features
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            out.push_str(&format!(
                "  family {} [{} docs] shared: {signature}\n",
                index + 1,
                cluster.members.len()
            ));
            // Advisory extraction profile: the UNION of extractor strategies that fired across this family's
            // members (with per-member support) — more than the shared signature above. Honestly sparse until
            // the corpus-wide extraction_manifest sweep lands `fired:` tokens on every document.
            let profile_line = cluster
                .members
                .first()
                .and_then(|representative| profile_by_representative.get(representative.as_str()));
            match profile_line {
                Some(profile) if !profile.fired_extractors.is_empty() => {
                    let fired = profile
                        .fired_extractors
                        .iter()
                        .map(|support| {
                            format!("{} ({})", support.extractor_name, support.member_support)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    out.push_str(&format!(
                        "    profile (fired extractors, member support): {fired}\n"
                    ));
                }
                _ => {
                    out.push_str(
                        "    profile (fired extractors): none recorded yet (run after a corpus re-ingest sweep)\n",
                    );
                }
            }
            for member in &cluster.members {
                out.push_str(&format!("    - {member}\n"));
            }
        }
    }

    let singletons: Vec<&DocumentCluster> = report
        .clusters
        .iter()
        .filter(|cluster| cluster.members.len() == 1)
        .collect();
    if !singletons.is_empty() {
        out.push_str("\nsingle-document clusters (unique structural shape):\n");
        for cluster in singletons {
            out.push_str(&format!("  - {}\n", cluster.members[0]));
        }
    }

    if !report.skipped.is_empty() {
        out.push_str("\nskipped artifacts:\n");
        for skipped in &report.skipped {
            out.push_str(&format!(
                "  - {} ({})\n",
                skipped.artifact_path.display(),
                skipped.reason
            ));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn fp(tokens: &[&str]) -> DocumentFingerprint {
        tokens.iter().map(|t| t.to_string()).collect()
    }

    fn report_for(
        documents: Vec<(String, DocumentFingerprint)>,
        threshold: f64,
    ) -> CorpusClusterReport {
        build_corpus_cluster_report(
            LoadedCorpus {
                documents,
                skipped: Vec::new(),
            },
            threshold,
        )
    }

    #[test]
    fn report_orders_clusters_largest_first_then_by_representative() {
        // Two alike register docs (a family) + one unique constraint-heavy doc.
        let reg = fp(&["shape:registers:b2", "fired:registers.field_table"]);
        let documents = vec![
            ("zeta_reg".to_string(), reg.clone()),
            ("alpha_reg".to_string(), reg.clone()),
            (
                "mid_proto".to_string(),
                fp(&["shape:signal_constraints:b2"]),
            ),
        ];
        let report = report_for(documents, 0.6);

        assert_eq!(report.documents_loaded, 3);
        assert_eq!(report.clusters.len(), 2);
        assert_eq!(report.multi_document_clusters(), 1);
        assert_eq!(report.single_document_clusters(), 1);
        // Largest family first; its members stay key-sorted (alpha before zeta).
        assert_eq!(report.clusters[0].members, vec!["alpha_reg", "zeta_reg"]);
        assert!(
            report.clusters[0]
                .shared_features
                .contains("fired:registers.field_table")
        );
        assert_eq!(report.clusters[1].members, vec!["mid_proto"]);
    }

    #[test]
    fn report_is_deterministic_regardless_of_input_order() {
        let reg = fp(&["shape:registers:b2", "fired:registers.field_table"]);
        let proto = fp(&["shape:signal_constraints:b2"]);
        let order1 = vec![
            ("a".to_string(), reg.clone()),
            ("b".to_string(), reg.clone()),
            ("c".to_string(), proto.clone()),
        ];
        let order2 = vec![
            ("c".to_string(), proto),
            ("b".to_string(), reg.clone()),
            ("a".to_string(), reg),
        ];
        assert_eq!(report_for(order1, 0.6), report_for(order2, 0.6));
    }

    #[test]
    fn render_lists_families_singletons_and_counts() {
        let reg = fp(&["shape:registers:b2", "fired:registers.field_table"]);
        let report = report_for(
            vec![
                ("alpha_reg".to_string(), reg.clone()),
                ("beta_reg".to_string(), reg),
                (
                    "gamma_proto".to_string(),
                    fp(&["shape:signal_constraints:b2"]),
                ),
            ],
            0.6,
        );
        let rendered = render_report(&report, Path::new("generated/evidence_ir"));

        assert!(rendered.contains("documents_loaded: 3"));
        assert!(rendered.contains("multi_document_clusters: 1"));
        assert!(rendered.contains("single_document_clusters: 1"));
        assert!(rendered.contains("multi-document families"));
        assert!(rendered.contains("alpha_reg"));
        assert!(rendered.contains("beta_reg"));
        assert!(rendered.contains("single-document clusters"));
        assert!(rendered.contains("gamma_proto"));
    }

    #[test]
    fn render_shows_per_family_profile_fired_extractors() {
        let reg = fp(&["shape:registers:b2", "fired:registers.field_table"]);
        let report = report_for(
            vec![
                ("alpha_reg".to_string(), reg.clone()),
                ("beta_reg".to_string(), reg),
            ],
            0.6,
        );
        let rendered = render_report(&report, Path::new("generated/evidence_ir"));
        assert!(
            rendered
                .contains("profile (fired extractors, member support): registers.field_table (2)"),
            "the family profile shows the union of fired extractors with member support; got:\n{rendered}"
        );
    }

    #[test]
    fn render_marks_profile_sparse_when_no_fired_tokens() {
        // Documents whose fingerprints carry only structural `shape:` tokens (no extraction_manifest) → the
        // profile is honestly reported as not-yet-recorded, never fabricated.
        let shape_only = fp(&["shape:registers:b2", "shape:protocol_states:b0"]);
        let report = report_for(
            vec![
                ("a".to_string(), shape_only.clone()),
                ("b".to_string(), shape_only),
            ],
            0.6,
        );
        let rendered = render_report(&report, Path::new("generated/evidence_ir"));
        assert!(rendered.contains("none recorded yet"));
    }

    #[test]
    fn collect_errors_when_evidence_root_is_missing() {
        let missing = Path::new("definitely/not/here/evidence_ir");
        let result = collect_corpus_documents(missing);
        assert!(matches!(result, Err(AppError::MissingPath(_))));
    }

    #[test]
    fn collect_skips_unloadable_artifact_and_ignores_dirs_without_artifact() {
        let root = tempdir().unwrap();
        // A directory whose evidence_ir.json cannot be loaded as an EvidenceIR (it does not match the
        // schema) → recorded as an honest skip rather than silently dropped or fabricated.
        let bad = root.path().join("doc_bad");
        fs::create_dir_all(&bad).unwrap();
        fs::write(bad.join("evidence_ir.json"), r#"{"stage":"source_ir"}"#).unwrap();
        // A directory with no evidence_ir.json at all → silently not a corpus member (not a skip).
        fs::create_dir_all(root.path().join("doc_empty")).unwrap();

        let loaded = collect_corpus_documents(root.path()).unwrap();
        assert!(loaded.documents.is_empty());
        assert_eq!(loaded.skipped.len(), 1);
        assert!(
            loaded.skipped[0]
                .artifact_path
                .ends_with("evidence_ir.json")
        );
        assert!(loaded.skipped[0].reason.contains("failed to load"));
    }
}
