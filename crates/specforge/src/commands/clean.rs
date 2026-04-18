use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::{CleanArgs, CleanScopeArg};
use crate::error::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanupTarget {
    path: PathBuf,
    bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CleanupPlan {
    targets: Vec<CleanupTarget>,
    total_bytes: u64,
}

pub fn run(args: CleanArgs) -> Result<()> {
    let plan = build_cleanup_plan(
        &args.generated_root,
        args.scope,
        args.document_key.as_deref(),
    )?;

    println!("command: clean");
    println!("mode: {}", if args.execute { "execute" } else { "dry-run" });
    println!("generated_root: {}", args.generated_root.display());
    println!("scope: {}", clean_scope_label(args.scope));
    if let Some(document_key) = &args.document_key {
        println!("document_key: {document_key}");
    }
    println!("candidate_count: {}", plan.targets.len());
    println!(
        "reclaimable_bytes: {} ({})",
        plan.total_bytes,
        human_bytes(plan.total_bytes)
    );

    if plan.targets.is_empty() {
        return Ok(());
    }

    println!("candidates:");
    for target in &plan.targets {
        println!(
            "- {} [{}]",
            target.path.display(),
            human_bytes(target.bytes)
        );
    }

    if args.execute {
        execute_cleanup_plan(&plan)?;
        println!("deleted_count: {}", plan.targets.len());
    } else {
        println!("next_action: rerun with --execute to delete these artifacts");
    }

    Ok(())
}

fn clean_scope_label(scope: CleanScopeArg) -> &'static str {
    match scope {
        CleanScopeArg::SourceNormalized => "source-normalized",
        CleanScopeArg::Document => "document",
    }
}

fn build_cleanup_plan(
    generated_root: &Path,
    scope: CleanScopeArg,
    document_key: Option<&str>,
) -> Result<CleanupPlan> {
    let mut candidate_paths = BTreeSet::new();
    match scope {
        CleanScopeArg::SourceNormalized => collect_source_normalized_candidates(
            generated_root,
            document_key,
            &mut candidate_paths,
        )?,
        CleanScopeArg::Document => {
            collect_document_candidates(generated_root, document_key, &mut candidate_paths)?
        }
    }

    let mut targets = Vec::with_capacity(candidate_paths.len());
    for path in candidate_paths {
        let bytes = path_size(&path)?;
        targets.push(CleanupTarget { path, bytes });
    }
    let total_bytes = targets.iter().map(|target| target.bytes).sum();

    Ok(CleanupPlan {
        targets,
        total_bytes,
    })
}

fn collect_source_normalized_candidates(
    generated_root: &Path,
    document_key: Option<&str>,
    candidate_paths: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    let source_ir_root = generated_root.join("source_ir");
    if !source_ir_root.exists() {
        return Ok(());
    }

    for document_root in iter_document_roots(&source_ir_root, document_key)? {
        for child_name in ["normalized", "normalized.staging"] {
            let candidate = document_root.join(child_name);
            if candidate.exists() {
                candidate_paths.insert(candidate);
            }
        }
    }

    Ok(())
}

fn collect_document_candidates(
    generated_root: &Path,
    document_key: Option<&str>,
    candidate_paths: &mut BTreeSet<PathBuf>,
) -> Result<()> {
    for stage in ["source_ir", "evidence_ir", "semantic_ir", "intent_ir"] {
        let stage_root = generated_root.join(stage);
        if !stage_root.exists() {
            continue;
        }

        for document_root in iter_document_roots(&stage_root, document_key)? {
            candidate_paths.insert(document_root);
        }
    }

    let adapters_root = generated_root.join("adapters");
    if adapters_root.exists() {
        for adapter_root in iter_document_roots(&adapters_root, None)? {
            for document_root in iter_document_roots(&adapter_root, document_key)? {
                candidate_paths.insert(document_root);
            }
        }
    }

    Ok(())
}

fn iter_document_roots(root: &Path, document_key: Option<&str>) -> Result<Vec<PathBuf>> {
    if !root.exists() {
        return Ok(Vec::new());
    }

    if let Some(document_key) = document_key {
        let document_root = root.join(document_key);
        return Ok(document_root
            .is_dir()
            .then_some(document_root)
            .into_iter()
            .collect());
    }

    let mut paths = Vec::new();
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn path_size(path: &Path) -> Result<u64> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.is_file() {
        return Ok(metadata.len());
    }

    if metadata.is_dir() {
        let mut total = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            total += path_size(&entry.path())?;
        }
        return Ok(total);
    }

    Ok(metadata.len())
}

fn execute_cleanup_plan(plan: &CleanupPlan) -> Result<()> {
    for target in &plan.targets {
        if !target.path.exists() {
            continue;
        }

        let metadata = fs::symlink_metadata(&target.path)?;
        if metadata.is_dir() {
            fs::remove_dir_all(&target.path)?;
        } else {
            fs::remove_file(&target.path)?;
        }
    }

    Ok(())
}

fn human_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit_index = 0usize;
    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{value:.1} {}", UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::{CleanupPlan, build_cleanup_plan, execute_cleanup_plan};
    use crate::cli::CleanScopeArg;
    use crate::error::Result;

    #[test]
    fn source_normalized_scope_collects_normalized_and_staging_roots() -> Result<()> {
        let tempdir = tempdir()?;
        let generated_root = tempdir.path().join("generated");
        let doc_a_normalized = generated_root
            .join("source_ir")
            .join("doc_a")
            .join("normalized");
        let doc_b_staging = generated_root
            .join("source_ir")
            .join("doc_b")
            .join("normalized.staging");
        fs::create_dir_all(&doc_a_normalized)?;
        fs::create_dir_all(&doc_b_staging)?;
        fs::write(doc_a_normalized.join("page.png"), b"a")?;
        fs::write(doc_b_staging.join("page.png"), b"b")?;

        let plan = build_cleanup_plan(&generated_root, CleanScopeArg::SourceNormalized, None)?;
        let collected_paths: Vec<_> = plan.targets.into_iter().map(|target| target.path).collect();

        assert_eq!(
            collected_paths,
            vec![
                generated_root
                    .join("source_ir")
                    .join("doc_a")
                    .join("normalized"),
                generated_root
                    .join("source_ir")
                    .join("doc_b")
                    .join("normalized.staging"),
            ]
        );

        Ok(())
    }

    #[test]
    fn document_scope_collects_all_document_stage_roots() -> Result<()> {
        let tempdir = tempdir()?;
        let generated_root = tempdir.path().join("generated");
        let doc_key = "axi_doc";
        for path in [
            generated_root.join("source_ir").join(doc_key),
            generated_root.join("evidence_ir").join(doc_key),
            generated_root.join("semantic_ir").join(doc_key),
            generated_root.join("intent_ir").join(doc_key),
            generated_root.join("adapters").join("fsm").join(doc_key),
        ] {
            fs::create_dir_all(&path)?;
            fs::write(path.join("marker.txt"), b"x")?;
        }
        fs::create_dir_all(generated_root.join("source_ir").join("other_doc"))?;

        let plan = build_cleanup_plan(&generated_root, CleanScopeArg::Document, Some(doc_key))?;
        let collected_paths: Vec<_> = plan.targets.into_iter().map(|target| target.path).collect();

        assert_eq!(
            collected_paths,
            vec![
                generated_root.join("adapters").join("fsm").join(doc_key),
                generated_root.join("evidence_ir").join(doc_key),
                generated_root.join("intent_ir").join(doc_key),
                generated_root.join("semantic_ir").join(doc_key),
                generated_root.join("source_ir").join(doc_key),
            ]
        );

        Ok(())
    }

    #[test]
    fn execute_cleanup_plan_removes_source_normalized_only() -> Result<()> {
        let tempdir = tempdir()?;
        let generated_root = tempdir.path().join("generated");
        let source_document_root = generated_root.join("source_ir").join("doc");
        let normalized_root = source_document_root.join("normalized");
        fs::create_dir_all(&normalized_root)?;
        fs::write(source_document_root.join("source_ir.json"), b"{}")?;
        fs::write(normalized_root.join("page.png"), b"artifact")?;

        let plan = build_cleanup_plan(
            &generated_root,
            CleanScopeArg::SourceNormalized,
            Some("doc"),
        )?;
        execute_cleanup_plan(&CleanupPlan {
            targets: plan.targets,
            total_bytes: plan.total_bytes,
        })?;

        assert!(!normalized_root.exists());
        assert!(source_document_root.join("source_ir.json").exists());

        Ok(())
    }
}
