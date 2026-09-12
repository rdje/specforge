//! `PRODUCTION-GRAPH-CENSUS-PIN.1` — the declared census the flow check compares against.

use std::collections::BTreeMap;
use std::path::Path;

use serde::Deserialize;

use crate::model::InformationFlowReport;

/// `PRODUCTION-GRAPH-CENSUS-PIN.1` — the declaration `check_production_genericity_flow.sh` compares
/// its derived census against, so a drift fails at COMMIT cadence.
///
/// The census has two kinds of number in it and the contract says which is which, because the
/// distinction is the whole design:
///
/// * **boundary** — which types are sources, which roots are rules, which regions are trusted. These
///   describe the genericity boundary itself and ordinary feature work does not move them (measured
///   unchanged across the six consecutive slices that each moved the volume counts). An exact pin is
///   right here: a change is a real change to the product's authority structure.
/// * **volume** — analyzed functions, helper edges, decision sites, semantic macros. These are sizes.
///   They move whenever production code is added, so an exact pin on them measures the commit rate
///   rather than the boundary. Declared as `baseline + delta` with an owning leaf, the same
///   `aggregate_change` idiom `doctrine/live_document_size/surfaces.jsonl` uses for the book's byte
///   total: the per-commit edit is not removed, it is converted from re-pinning a number nothing
///   compares into ATTRIBUTING a census change to the leaf that caused it.
///
/// Before this contract the only comparison lived in a `cargo test`, which this repository's CI policy
/// runs before a push and not per commit — so the pins drifted by +18 functions / +179 helper edges /
/// +226 decision sites over 29 commits while every commit reported a fully green doctrine gate
/// (`PRODUCTION-GRAPH-CENSUS-PIN.0`).
pub const FLOW_CENSUS_CONTRACT: &str = "doctrine/production_genericity/flow_census.json";

#[derive(Debug, Deserialize)]
struct FlowCensusContract {
    boundary: FlowCensusBoundary,
    volume: FlowCensusVolume,
}

#[derive(Debug, Deserialize)]
struct FlowCensusBoundary {
    pinned: BTreeMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct FlowCensusVolume {
    aggregate_change: FlowCensusAggregateChange,
}

#[derive(Debug, Deserialize)]
struct FlowCensusAggregateChange {
    owner: String,
    baseline: BTreeMap<String, usize>,
    delta: BTreeMap<String, i64>,
}

/// Read the census contract at `root`.
pub fn load_flow_census(root: &Path) -> Result<FlowCensus, String> {
    let path = root.join(FLOW_CENSUS_CONTRACT);
    let raw = std::fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let contract: FlowCensusContract = serde_json::from_str(&raw)
        .map_err(|error| format!("cannot parse {}: {error}", path.display()))?;
    if contract.volume.aggregate_change.owner.trim().is_empty() {
        return Err(format!(
            "{}: volume.aggregate_change.owner names no leaf; a census change must be attributed",
            path.display()
        ));
    }
    let mut expected = contract.boundary.pinned;
    for (field, baseline) in &contract.volume.aggregate_change.baseline {
        let delta = contract
            .volume
            .aggregate_change
            .delta
            .get(field)
            .copied()
            .ok_or_else(|| format!("{}: volume delta has no '{field}'", path.display()))?;
        let total = i64::try_from(*baseline)
            .map_err(|_| format!("{}: baseline '{field}' does not fit", path.display()))?
            + delta;
        let total = usize::try_from(total)
            .map_err(|_| format!("{}: '{field}' baseline + delta is negative", path.display()))?;
        if expected.insert(field.clone(), total).is_some() {
            return Err(format!(
                "{}: '{field}' is declared both as a boundary pin and as a volume count",
                path.display()
            ));
        }
    }
    Ok(FlowCensus { expected })
}

/// The eighteen expected counts, boundary pins and volume totals resolved together.
#[derive(Debug)]
pub struct FlowCensus {
    expected: BTreeMap<String, usize>,
}

impl FlowCensus {
    /// Every field the contract declares that disagrees with `report`, named with both values.
    /// A field the report carries but the contract does not is also a breach: a new census field
    /// must be declared, or the contract stops covering the surface it claims to.
    pub fn disagreements(&self, report: &InformationFlowReport) -> Vec<String> {
        let derived = report.census_fields();
        let mut breaches = Vec::new();
        for (field, expected) in &self.expected {
            match derived.get(field.as_str()) {
                Some(actual) if actual == expected => {}
                Some(actual) => breaches.push(format!(
                    "production-genericity-flow: '{field}' is {actual}, contract declares {expected}"
                )),
                None => breaches.push(format!(
                    "production-genericity-flow: contract declares '{field}', which the census does not report"
                )),
            }
        }
        for field in derived.keys() {
            if !self.expected.contains_key(*field) {
                breaches.push(format!(
                    "production-genericity-flow: census reports '{field}', which the contract does not declare"
                ));
            }
        }
        breaches.sort();
        breaches
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn census(pairs: &[(&str, usize)]) -> FlowCensus {
        FlowCensus {
            expected: pairs
                .iter()
                .map(|(field, value)| ((*field).to_owned(), *value))
                .collect(),
        }
    }

    fn report() -> InformationFlowReport {
        InformationFlowReport {
            rule_roots: 120,
            analyzed_functions: 2_409,
            ..InformationFlowReport::default()
        }
    }

    /// The comparison names the field and BOTH values, so the remedy is obvious from the failure
    /// alone — `PRODUCTION-GRAPH-CENSUS-PIN.1`'s whole point is that a census which reports without
    /// comparing permits every drift.
    #[test]
    fn a_disagreement_names_the_field_and_both_values() {
        let breaches = census(&[("rule_roots", 121)]).disagreements(&report());
        assert!(
            breaches
                .iter()
                .any(|b| b.contains("'rule_roots' is 120, contract declares 121")),
            "{breaches:?}"
        );
    }

    /// A census field the contract does not declare is a breach in its own right. Without this the
    /// contract would silently stop covering the surface it claims to the first time a field is added
    /// to the report — which is exactly how the original pins went stale for 29 commits.
    #[test]
    fn a_census_field_the_contract_does_not_declare_is_a_breach() {
        let breaches = census(&[("rule_roots", 120)]).disagreements(&report());
        assert!(
            breaches.iter().any(|b| b.contains(
                "census reports 'analyzed_functions', which the contract does not declare"
            )),
            "{breaches:?}"
        );
    }

    /// And the other direction: a contract that declares a field the census stopped reporting is also
    /// a breach, not a silent pass.
    #[test]
    fn a_declared_field_the_census_does_not_report_is_a_breach() {
        let mut expected: Vec<(&str, usize)> = report().census_fields().into_iter().collect();
        expected.push(("a_field_that_no_longer_exists", 1));
        let breaches = census(&expected).disagreements(&report());
        assert_eq!(breaches.len(), 1, "{breaches:?}");
        assert!(
            breaches[0].contains("contract declares 'a_field_that_no_longer_exists'"),
            "{breaches:?}"
        );
    }

    /// The full report against its own fields agrees with itself — the base case, so the three
    /// breach tests above are measuring a real difference rather than a broken comparison.
    #[test]
    fn a_contract_matching_the_report_has_no_disagreement() {
        let expected: Vec<(&str, usize)> = report().census_fields().into_iter().collect();
        assert_eq!(
            census(&expected).disagreements(&report()),
            Vec::<String>::new()
        );
    }
}
