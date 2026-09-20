use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::Command;

use syn::punctuated::Punctuated;
use syn::{Attribute, Expr, Lit, Meta, Token};

#[derive(Debug, Clone)]
pub(crate) struct Configuration {
    flags: BTreeSet<String>,
    values: BTreeMap<String, BTreeSet<String>>,
    features: BTreeSet<String>,
    declared_features: BTreeSet<String>,
}

/// The target the production graph is DEFINED against, so the census measures the source and not
/// the machine that ran it (`COMMIT-GATE-SINGLE-RUN.16`).
///
/// Without this, `rustc --print cfg` answers for the host. The analyzer honours `cfg`, and the
/// product has genuinely platform-conditional code — the memory-pressure guard in
/// `ir/source/docling_backend.rs` compiles a different helper on macOS than on Linux — so the
/// derived graph differed by host: `decision_sites` 13345 on an Apple machine against 13348 on a
/// Linux runner, with `helper_edges` moving the opposite way by the same 3. The declaration was
/// pinned on macOS, so hosted CI could never have agreed with it, and nothing revealed that while
/// hosted CI was not running the gate.
///
/// Linux is chosen because that is where CI runs; the value matters far less than its being FIXED.
/// Changing it re-derives every volume count, so it is a decision with an owning leaf, not a knob.
/// `rustc --print cfg --target` computes cfg without needing that target's std installed, so this
/// costs nothing on a developer machine of any platform.
pub(crate) const ANALYSIS_TARGET: &str = "x86_64-unknown-linux-gnu";

impl Configuration {
    pub(crate) fn from_rustc(
        root: &Path,
        features: BTreeSet<String>,
        declared_features: BTreeSet<String>,
    ) -> Result<Self, String> {
        let output = Command::new("rustc")
            .args(["--print", "cfg", "--target", ANALYSIS_TARGET])
            .current_dir(root)
            .output()
            .map_err(|error| format!("cannot execute rustc --print cfg: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "rustc --print cfg failed with status {}",
                output.status
            ));
        }
        let stdout = String::from_utf8(output.stdout)
            .map_err(|_| "rustc --print cfg emitted non-UTF-8 output".to_owned())?;
        let mut flags = BTreeSet::new();
        let mut values: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for line in stdout.lines() {
            if let Some((key, raw_value)) = line.split_once('=') {
                let value = raw_value
                    .strip_prefix('"')
                    .and_then(|value| value.strip_suffix('"'))
                    .ok_or_else(|| format!("unsupported rustc cfg value '{line}'"))?;
                values
                    .entry(key.to_owned())
                    .or_default()
                    .insert(value.to_owned());
            } else if !line.is_empty() {
                flags.insert(line.to_owned());
            }
        }
        flags.remove("test");
        Ok(Self {
            flags,
            values,
            features,
            declared_features,
        })
    }

    pub(crate) fn attributes_are_active(&self, attrs: &[Attribute]) -> Result<bool, String> {
        for attribute in attrs {
            if attribute.path().is_ident("cfg_attr") {
                return Err("cfg_attr is unsupported in the production graph".to_owned());
            }
            if attribute.path().is_ident("cfg") {
                let meta = attribute
                    .parse_args::<Meta>()
                    .map_err(|error| format!("cannot parse cfg attribute: {error}"))?;
                if !self.evaluate(&meta)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn evaluate(&self, meta: &Meta) -> Result<bool, String> {
        match meta {
            Meta::Path(path) => {
                let name = single_identifier(path)?;
                if name == "test" {
                    Ok(false)
                } else if self.flags.contains(&name) {
                    Ok(true)
                } else {
                    Err(format!("unsupported cfg flag '{name}'"))
                }
            }
            Meta::NameValue(name_value) => {
                let name = single_identifier(&name_value.path)?;
                let Expr::Lit(value) = &name_value.value else {
                    return Err(format!("cfg key '{name}' does not use a literal value"));
                };
                let Lit::Str(value) = &value.lit else {
                    return Err(format!("cfg key '{name}' does not use a string value"));
                };
                if name == "feature" {
                    if !self.declared_features.contains(&value.value()) {
                        return Err(format!(
                            "cfg references undeclared Cargo feature '{}'",
                            value.value()
                        ));
                    }
                    Ok(self.features.contains(&value.value()))
                } else {
                    Ok(self
                        .values
                        .get(&name)
                        .is_some_and(|values| values.contains(&value.value())))
                }
            }
            Meta::List(list) => {
                let operator = single_identifier(&list.path)?;
                let nested = list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .map_err(|error| format!("cannot parse cfg {operator} operands: {error}"))?;
                match operator.as_str() {
                    "all" => {
                        for item in &nested {
                            if !self.evaluate(item)? {
                                return Ok(false);
                            }
                        }
                        Ok(true)
                    }
                    "any" => {
                        for item in &nested {
                            if self.evaluate(item)? {
                                return Ok(true);
                            }
                        }
                        Ok(false)
                    }
                    "not" if nested.len() == 1 => Ok(!self.evaluate(&nested[0])?),
                    "not" => Err("cfg not(...) requires exactly one operand".to_owned()),
                    _ => Err(format!("unsupported cfg operator '{operator}'")),
                }
            }
        }
    }
}

pub(crate) fn path_attribute(attrs: &[Attribute]) -> Result<Option<String>, String> {
    let mut value = None;
    for attribute in attrs {
        if !attribute.path().is_ident("path") {
            continue;
        }
        let Meta::NameValue(name_value) = &attribute.meta else {
            return Err("module path attribute must use #[path = \"...\"]".to_owned());
        };
        let Expr::Lit(literal) = &name_value.value else {
            return Err("module path attribute must use a string literal".to_owned());
        };
        let Lit::Str(literal) = &literal.lit else {
            return Err("module path attribute must use a string literal".to_owned());
        };
        if value.replace(literal.value()).is_some() {
            return Err("module has more than one path attribute".to_owned());
        }
    }
    Ok(value)
}

fn single_identifier(path: &syn::Path) -> Result<String, String> {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return Err(format!(
            "unsupported configuration path '{}'",
            quote::quote!(#path)
        ));
    }
    Ok(path.segments[0].ident.to_string())
}
