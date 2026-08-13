use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

const PRODUCT_PACKAGES: &[&str] = &["specforge", "specforge-conformance", "specforge-core"];

#[derive(Debug, Clone)]
pub(crate) struct TargetContext {
    pub id: String,
    pub package: String,
    pub target: String,
    pub kind: String,
    pub root: String,
    pub features: BTreeSet<String>,
    pub declared_features: BTreeSet<String>,
    pub external_crates: BTreeSet<String>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
    workspace_members: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Package {
    id: String,
    name: String,
    manifest_path: String,
    targets: Vec<Target>,
    dependencies: Vec<Dependency>,
    features: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Target {
    name: String,
    kind: Vec<String>,
    src_path: String,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    name: String,
    rename: Option<String>,
    kind: Option<String>,
    optional: bool,
    uses_default_features: bool,
    features: Vec<String>,
    target: Option<String>,
}

pub(crate) fn load_target_contexts(root: &Path) -> Result<Vec<TargetContext>, String> {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--locked",
            "--offline",
        ])
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot execute cargo metadata: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed with status {}",
            output.status
        ));
    }
    let metadata: Metadata = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("cannot parse cargo metadata: {error}"))?;
    let workspace_members: BTreeSet<_> = metadata.workspace_members.iter().collect();
    let mut packages = BTreeMap::new();
    for package in metadata.packages {
        if PRODUCT_PACKAGES.contains(&package.name.as_str()) {
            if !workspace_members.contains(&package.id) {
                return Err(format!(
                    "product package '{}' is not a workspace member",
                    package.name
                ));
            }
            if packages.insert(package.name.clone(), package).is_some() {
                return Err("cargo metadata contains a duplicate product package".to_owned());
            }
        }
    }
    for expected in PRODUCT_PACKAGES {
        if !packages.contains_key(*expected) {
            return Err(format!(
                "cargo metadata is missing product package '{expected}'"
            ));
        }
    }

    let enabled = enabled_features(&packages)?;
    let mut contexts = Vec::new();
    for package_name in PRODUCT_PACKAGES {
        let package = &packages[*package_name];
        let manifest = repository_relative(root, Path::new(&package.manifest_path))?;
        if !manifest.ends_with("/Cargo.toml") {
            return Err(format!(
                "package '{package_name}' has an unsupported manifest path '{manifest}'"
            ));
        }
        let external_crates = external_crates(package);
        for target in &package.targets {
            let selected_kind = if target.kind.iter().any(|kind| kind == "lib") {
                Some("lib")
            } else if target.kind.iter().any(|kind| kind == "bin") {
                Some("bin")
            } else {
                None
            };
            let Some(kind) = selected_kind else {
                continue;
            };
            let root_path = repository_relative(root, Path::new(&target.src_path))?;
            let mut target_external = external_crates.clone();
            if kind == "bin" {
                target_external.insert(normalize_crate_name(package_name));
            }
            contexts.push(TargetContext {
                id: format!("{package_name}:{}:{kind}", target.name),
                package: (*package_name).to_owned(),
                target: target.name.clone(),
                kind: kind.to_owned(),
                root: root_path,
                features: enabled.get(*package_name).cloned().unwrap_or_default(),
                declared_features: package.features.keys().cloned().collect(),
                external_crates: target_external,
            });
        }
    }
    contexts.sort_by(|left, right| left.id.cmp(&right.id));
    if contexts.is_empty() {
        return Err("cargo metadata exposes no product library or binary targets".to_owned());
    }
    Ok(contexts)
}

fn enabled_features(
    packages: &BTreeMap<String, Package>,
) -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let mut enabled: BTreeMap<String, BTreeSet<String>> = packages
        .keys()
        .map(|name| (name.clone(), BTreeSet::new()))
        .collect();
    for (name, package) in packages {
        enable_feature(name, "default", packages, &mut enabled)?;
        for dependency in &package.dependencies {
            if dependency
                .kind
                .as_deref()
                .is_some_and(|kind| kind != "normal")
            {
                continue;
            }
            if dependency.target.is_some() {
                return Err(format!(
                    "product package '{name}' has an unsupported target-conditional dependency '{}'",
                    dependency.name
                ));
            }
            if dependency.optional {
                continue;
            }
            if !packages.contains_key(&dependency.name) {
                continue;
            }
            if dependency.uses_default_features {
                enable_feature(&dependency.name, "default", packages, &mut enabled)?;
            }
            for feature in &dependency.features {
                enable_feature(&dependency.name, feature, packages, &mut enabled)?;
            }
        }
    }
    for features in enabled.values_mut() {
        features.remove("default");
    }
    Ok(enabled)
}

fn enable_feature(
    package_name: &str,
    requested: &str,
    packages: &BTreeMap<String, Package>,
    enabled: &mut BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    let package = &packages[package_name];
    if requested == "default" && !package.features.contains_key("default") {
        return Ok(());
    }
    if !package.features.contains_key(requested) {
        return Err(format!(
            "dependency requests unknown feature '{requested}' from '{package_name}'"
        ));
    }
    let mut pending = VecDeque::from([requested.to_owned()]);
    while let Some(feature) = pending.pop_front() {
        if !enabled
            .get_mut(package_name)
            .expect("enabled feature package exists")
            .insert(feature.clone())
        {
            continue;
        }
        for member in &package.features[&feature] {
            if member.starts_with("dep:") || member.contains('/') || member.contains('?') {
                continue;
            }
            if !package.features.contains_key(member) {
                return Err(format!(
                    "feature '{package_name}/{feature}' references unknown local feature '{member}'"
                ));
            }
            pending.push_back(member.clone());
        }
    }
    Ok(())
}

fn external_crates(package: &Package) -> BTreeSet<String> {
    let mut names = BTreeSet::from([
        "alloc".to_owned(),
        "core".to_owned(),
        "proc_macro".to_owned(),
        "std".to_owned(),
    ]);
    for dependency in &package.dependencies {
        if dependency.kind.as_deref().is_some_and(|kind| kind == "dev") {
            continue;
        }
        names.insert(normalize_crate_name(
            dependency.rename.as_deref().unwrap_or(&dependency.name),
        ));
    }
    names
}

fn normalize_crate_name(name: &str) -> String {
    name.replace('-', "_")
}

fn repository_relative(root: &Path, path: &Path) -> Result<String, String> {
    let absolute = if path.is_absolute() {
        PathBuf::from(path)
    } else {
        root.join(path)
    };
    let relative = absolute.strip_prefix(root).map_err(|_| {
        "cargo metadata exposes a product path outside the repository root".to_owned()
    })?;
    let value = relative
        .to_str()
        .ok_or_else(|| "cargo metadata exposes a non-UTF-8 product path".to_owned())?
        .replace('\\', "/");
    if value.is_empty() || value.split('/').any(|component| component == "..") {
        return Err("cargo metadata exposes an unsafe product path".to_owned());
    }
    Ok(value)
}
