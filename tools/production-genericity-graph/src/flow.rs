use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

use quote::ToTokens;
use syn::visit::{self, Visit};
use syn::{BinOp, Block, Expr, FnArg, ImplItem, Item, Pat, Signature, Stmt, TraitItem, Type};

use crate::analyzer::{
    AnalyzedProgram, ModuleUnit, analyze_program, normalized_tokens, short_hash,
};
use crate::model::{FLOW_SCHEMA_VERSION, InformationFlowReport};

const BOUNDARY_PATH: &str = "doctrine/production_genericity/information_flow_boundary.tsv";
const RULE_PATH: &str = "doctrine/production_genericity/rule_family_inventory.tsv";
const CLAIM_PATH: &str = "doctrine/production_genericity/claim_family_inventory.tsv";
const BYPASS_PATH: &str = "doctrine/production_genericity/conformance_bypass_inventory.tsv";
const BOUNDARY_HEADER: &str = "boundary_id\trole\tdata_class\tpath\tcontract";
const RULE_HEADER: &str = "family_id\tstage\trule_stem\tcurrent_producer_entrypoints\tcurrent_mutator_entrypoints\tpremise_kinds\tsymbol_capability\talpha_obligation\tcompatibility\ttarget_module\tschema_owner\tcanonical_seams\tmigration_leaf";
const CLAIM_HEADER: &str =
    "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane";
const BYPASS_HEADER: &str =
    "entrypoint\tstage\tcurrent_mutation\tcurrent_sink\ttarget_disposition\tmigration_leaf";

#[derive(Debug, Clone)]
struct BoundaryRow {
    id: String,
    role: String,
    data_class: String,
    path: String,
    contract: String,
}

#[derive(Debug, Default)]
struct Boundary {
    rows: Vec<BoundaryRow>,
    source_types: BTreeMap<String, String>,
    source_fields: BTreeMap<(String, String), String>,
    source_parameters: BTreeMap<(String, usize), String>,
    source_returns: BTreeMap<String, String>,
    declassifiers: BTreeSet<String>,
    declassifier_classes: BTreeMap<String, String>,
    proof_gates: BTreeSet<String>,
    proof_gate_stages: BTreeMap<String, String>,
    proof_value_returns: BTreeSet<String>,
    proof_value_stages: BTreeMap<String, String>,
    proof_value_sinks: BTreeMap<String, usize>,
    trusted_roots: BTreeSet<String>,
    trusted_modules: BTreeSet<String>,
    non_authority_roots: BTreeSet<String>,
    protected_types: BTreeMap<String, String>,
    rule_roots: BTreeSet<String>,
    grammar_roots: BTreeSet<String>,
    canonical_seams: BTreeSet<String>,
    seam_stages: BTreeMap<String, BTreeSet<String>>,
    bypass_roots: BTreeSet<String>,
    canonical_fields: BTreeSet<String>,
    derived_joins: BTreeSet<String>,
}

#[derive(Clone)]
struct FunctionModel {
    id: String,
    module: String,
    plane: String,
    owner: Option<String>,
    owner_terminal: Option<String>,
    signature: Signature,
    block: Block,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Summary {
    return_params: BTreeSet<usize>,
    return_direct: BTreeSet<String>,
    sink_params: BTreeSet<usize>,
    sink_direct: BTreeSet<String>,
}

#[derive(Debug, Clone, Default)]
struct Dependency {
    params: BTreeSet<usize>,
    direct: BTreeSet<String>,
    canonical_fields: BTreeSet<String>,
    proof_value: bool,
}

impl Dependency {
    fn parameter(index: usize) -> Self {
        Self {
            params: BTreeSet::from([index]),
            ..Self::default()
        }
    }

    fn direct(class: impl Into<String>) -> Self {
        Self {
            direct: BTreeSet::from([class.into()]),
            ..Self::default()
        }
    }

    fn union(&mut self, other: Self) {
        self.params.extend(other.params);
        self.direct.extend(other.direct);
        self.canonical_fields.extend(other.canonical_fields);
        self.proof_value |= other.proof_value;
    }

    fn combined(values: impl IntoIterator<Item = Self>) -> Self {
        let mut result = Self::default();
        for value in values {
            result.union(value);
        }
        // Combining values is a transformation, not an exact verified-serialization move.
        result.proof_value = false;
        result
    }

    fn is_sensitive(&self) -> bool {
        !self.params.is_empty() || !self.direct.is_empty()
    }
}

#[derive(Default)]
struct Inspection {
    summary: Summary,
    decision_sites: usize,
    protected_constructions: usize,
    protected_calls: usize,
    semantic_macros: usize,
    failures: Vec<String>,
}

pub(crate) fn analyze(root: &Path) -> Result<InformationFlowReport, String> {
    let program = analyze_program(root)?;
    let mut boundary = load_boundary(root, &program)?;
    load_rule_joins(root, &program, &mut boundary)?;
    load_claim_fields(root, &mut boundary)?;
    load_bypass_roots(root, &program, &mut boundary)?;
    let functions = collect_functions(&program, &boundary)?;
    let edges = local_edges(&program, &functions);
    let rule_closure = closure(&boundary.rule_roots, &edges);
    let grammar_closure = closure(&boundary.grammar_roots, &edges);
    let mut proof_roots = boundary.proof_gates.clone();
    proof_roots.extend(boundary.proof_value_returns.iter().cloned());
    proof_roots.extend(boundary.proof_value_sinks.keys().cloned());
    let proof_closure = closure(&proof_roots, &edges);
    let declassifier_closures = declassifier_closures(&boundary, &edges);
    let declassifier_closure: BTreeSet<_> = declassifier_closures
        .values()
        .flat_map(|items| items.iter().cloned())
        .collect();
    let mut trusted_seeds = boundary.trusted_roots.clone();
    for module in &boundary.trusted_modules {
        trusted_seeds.extend(
            functions
                .values()
                .filter(|function| &function.module == module)
                .map(|function| function.id.clone()),
        );
    }
    let trusted_closure = closure(&trusted_seeds, &edges);
    let bypass_closure = closure(&boundary.bypass_roots, &edges);
    let non_authority_closure = closure(&boundary.non_authority_roots, &edges);

    verify_registry_regions(
        &boundary,
        &rule_closure,
        &grammar_closure,
        &trusted_closure,
        &proof_closure,
        &non_authority_closure,
        &declassifier_closure,
    )?;
    verify_canonical_seams(&boundary, &edges)?;

    let mut summaries: BTreeMap<String, Summary> = functions
        .keys()
        .map(|id| (id.clone(), Summary::default()))
        .collect();
    let mut stabilized = false;
    for _ in 0..=functions.len() {
        let previous = summaries.clone();
        for function in functions.values() {
            summaries.insert(
                function.id.clone(),
                inspect_function(
                    function,
                    &program,
                    &functions,
                    &previous,
                    &boundary,
                    &rule_closure,
                    &grammar_closure,
                    &proof_closure,
                    &trusted_closure,
                    &bypass_closure,
                    &non_authority_closure,
                    &declassifier_closures,
                    false,
                )
                .summary,
            );
        }
        if summaries == previous {
            stabilized = true;
            break;
        }
    }
    if !stabilized {
        return Err("information-flow summaries did not reach a fixed point".to_owned());
    }

    let mut report = InformationFlowReport {
        schema_version: FLOW_SCHEMA_VERSION,
        boundary_rows: boundary.rows.len(),
        source_types: boundary.source_types.len(),
        source_fields: boundary.source_fields.len(),
        source_parameters: boundary.source_parameters.len(),
        source_returns: boundary.source_returns.len(),
        rule_roots: boundary.rule_roots.len(),
        grammar_declassifiers: boundary.grammar_roots.len() + boundary.declassifiers.len(),
        canonical_seams: boundary.canonical_seams.len(),
        proof_gates: boundary.proof_gates.len(),
        trusted_regions: boundary.trusted_roots.len() + boundary.trusted_modules.len(),
        non_authoritative_regions: boundary.non_authority_roots.len(),
        protected_types: boundary.protected_types.len(),
        analyzed_functions: functions.len(),
        helper_edges: edges.values().map(BTreeSet::len).sum(),
        decision_sites: 0,
        protected_constructions: 0,
        protected_calls: 0,
        semantic_macros: 0,
    };
    let mut failures = Vec::new();
    for function in functions.values() {
        let inspection = inspect_function(
            function,
            &program,
            &functions,
            &summaries,
            &boundary,
            &rule_closure,
            &grammar_closure,
            &proof_closure,
            &trusted_closure,
            &bypass_closure,
            &non_authority_closure,
            &declassifier_closures,
            true,
        );
        report.decision_sites += inspection.decision_sites;
        report.protected_constructions += inspection.protected_constructions;
        report.protected_calls += inspection.protected_calls;
        report.semantic_macros += inspection.semantic_macros;
        failures.extend(inspection.failures);
    }
    failures.sort();
    failures.dedup();
    if !failures.is_empty() {
        return Err(format!(
            "information-flow boundary rejected {} site(s):\n{}",
            failures.len(),
            failures.join("\n")
        ));
    }
    Ok(report)
}

fn load_boundary(root: &Path, program: &AnalyzedProgram) -> Result<Boundary, String> {
    let text = fs::read_to_string(root.join(BOUNDARY_PATH))
        .map_err(|error| format!("cannot read {BOUNDARY_PATH}: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(BOUNDARY_HEADER) {
        return Err(format!("{BOUNDARY_PATH} has an unsupported header"));
    }
    let mut boundary = Boundary::default();
    let mut ids = BTreeSet::new();
    for (offset, line) in lines.enumerate() {
        let line_number = offset + 2;
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 5 || fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "{BOUNDARY_PATH}:{line_number}: expected five non-empty tab-separated fields"
            ));
        }
        let row = BoundaryRow {
            id: fields[0].to_owned(),
            role: fields[1].to_owned(),
            data_class: fields[2].to_owned(),
            path: fields[3].to_owned(),
            contract: fields[4].to_owned(),
        };
        if !ids.insert(row.id.clone()) {
            return Err(format!(
                "{BOUNDARY_PATH}:{line_number}: duplicate boundary id '{}'",
                row.id
            ));
        }
        validate_boundary_row(&row, program, &mut boundary)
            .map_err(|error| format!("{BOUNDARY_PATH}:{line_number}: {error}"))?;
        boundary.rows.push(row);
    }
    if boundary.rows.is_empty() {
        return Err(format!("{BOUNDARY_PATH} has no data rows"));
    }
    let required_joins = BTreeSet::from([
        "registered_rule_entrypoints".to_owned(),
        "registered_grammar_declassifiers".to_owned(),
        "proof_only_sinks".to_owned(),
    ]);
    if boundary.derived_joins != required_joins {
        return Err(format!(
            "{BOUNDARY_PATH} must declare exactly the three closed derived joins"
        ));
    }
    if boundary.source_types.is_empty()
        || boundary.proof_gates.is_empty()
        || boundary.protected_types.is_empty()
    {
        return Err(format!(
            "{BOUNDARY_PATH} leaves a required boundary class empty"
        ));
    }
    Ok(boundary)
}

fn validate_boundary_row(
    row: &BoundaryRow,
    program: &AnalyzedProgram,
    boundary: &mut Boundary,
) -> Result<(), String> {
    validate_boundary_contract(row)?;
    match row.role.as_str() {
        "source_type" => {
            let name = terminal_symbol(&row.path)?;
            require_item_kind(program, &row.path, &["struct", "enum"])?;
            insert_unique(
                &mut boundary.source_types,
                name,
                &row.data_class,
                "source type",
            )?;
        }
        "source_field" => {
            let (owner, field) = split_owner_field(&row.path)?;
            require_struct_field(program, &row.path, &owner, &field)?;
            if boundary
                .source_fields
                .insert((owner.clone(), field.clone()), row.data_class.clone())
                .is_some()
            {
                return Err(format!("duplicate source field '{owner}::{field}'"));
            }
        }
        "source_return" => {
            for item in resolve_function_path(program, &row.path)? {
                insert_unique(
                    &mut boundary.source_returns,
                    item,
                    &row.data_class,
                    "source return",
                )?;
            }
        }
        "source_parameter" => {
            let index = argument_index(&row.contract)?;
            for item in resolve_function_path(program, &row.path)? {
                if boundary
                    .source_parameters
                    .insert((item.clone(), index), row.data_class.clone())
                    .is_some()
                {
                    return Err(format!(
                        "duplicate source parameter '{item}' argument {index}"
                    ));
                }
            }
        }
        "rule_root" => {
            boundary
                .rule_roots
                .extend(resolve_function_path(program, &row.path)?);
        }
        "grammar_root" => {
            let roots = resolve_function_path(program, &row.path)?;
            boundary.rule_roots.extend(roots.iter().cloned());
            boundary.grammar_roots.extend(roots);
        }
        "non_authoritative_root" => {
            boundary
                .non_authority_roots
                .extend(resolve_function_path(program, &row.path)?);
        }
        "declassifier" => {
            for item in resolve_function_path(program, &row.path)? {
                boundary.declassifiers.insert(item.clone());
                insert_unique(
                    &mut boundary.declassifier_classes,
                    item,
                    &row.data_class,
                    "declassifier function",
                )?;
            }
        }
        "proof_gate" => {
            let stage = contract_stage(&row.contract)?;
            for item in resolve_function_path(program, &row.path)? {
                boundary.proof_gates.insert(item.clone());
                insert_unique(
                    &mut boundary.proof_gate_stages,
                    item,
                    &stage,
                    "proof gate stage",
                )?;
            }
        }
        "proof_value_return" => {
            let stage = contract_stage(&row.contract)?;
            for item in resolve_function_path(program, &row.path)? {
                boundary.proof_value_returns.insert(item.clone());
                insert_unique(
                    &mut boundary.proof_value_stages,
                    item,
                    &stage,
                    "proof value stage",
                )?;
            }
        }
        "proof_value_sink" => {
            let index = row
                .contract
                .strip_prefix("verified_argument:")
                .ok_or_else(|| {
                    "proof value sink contract must name verified_argument:N".to_owned()
                })?
                .parse::<usize>()
                .map_err(|_| "proof value sink argument index is not an integer".to_owned())?;
            for item in resolve_function_path(program, &row.path)? {
                if boundary
                    .proof_value_sinks
                    .insert(item.clone(), index)
                    .is_some()
                {
                    return Err(format!("duplicate proof value sink '{item}'"));
                }
            }
        }
        "trusted_region" if row.path.ends_with("#module") => {
            boundary
                .trusted_modules
                .extend(resolve_module_path(program, &row.path)?);
        }
        "trusted_region" => {
            boundary
                .trusted_roots
                .extend(resolve_function_path(program, &row.path)?);
        }
        "protected_type" => {
            let name = terminal_symbol(&row.path)?;
            require_item_kind(program, &row.path, &["struct", "enum"])?;
            insert_unique(
                &mut boundary.protected_types,
                name,
                &row.contract,
                "protected type",
            )?;
        }
        "derived_join" => {
            if !boundary.derived_joins.insert(row.contract.clone()) {
                return Err(format!("duplicate derived join '{}'", row.contract));
            }
        }
        other => return Err(format!("unknown boundary role '{other}'")),
    }
    Ok(())
}

fn validate_boundary_contract(row: &BoundaryRow) -> Result<(), String> {
    if !row.id.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_')
    }) {
        return Err(format!("boundary id '{}' has an invalid shape", row.id));
    }
    let valid_stage_contract = |prefix: &str| {
        row.contract
            .strip_prefix(prefix)
            .is_some_and(is_artifact_stage)
    };
    let valid = match row.role.as_str() {
        "source_type" => is_source_class(&row.data_class) && row.contract == "aggregate_input",
        "source_field" => is_source_class(&row.data_class) && row.contract == "field_projection",
        "source_return" => is_source_class(&row.data_class) && row.contract == "return_value",
        "source_parameter" => {
            is_source_class(&row.data_class)
                && row
                    .contract
                    .strip_prefix("argument:")
                    .is_some_and(|value| value.parse::<usize>().is_ok())
        }
        "rule_root" => row.data_class == "semantic_control" && row.contract == "registered_rule",
        "grammar_root" => {
            matches!(
                row.data_class.as_str(),
                "raw_evidence" | "grounded_proposal"
            ) && matches!(
                row.contract.as_str(),
                "evaluation_only"
                    | "grounded_proposal_only"
                    | "proof_gated_aggregation"
                    | "proposal_only"
                    | "universal_filter"
                    | "validated_prior_output"
                    | "validated_structural_prior"
                    | "validated_prior_input"
                    | "proof_gated_orchestration"
                    | "demotion_only"
            )
        }
        "non_authoritative_root" => {
            is_source_class(&row.data_class)
                && matches!(row.contract.as_str(), "lifecycle_only" | "diagnostics_only")
        }
        "declassifier" => {
            matches!(
                row.data_class.as_str(),
                "document_identity" | "symbol_identity"
            ) && matches!(
                row.contract.as_str(),
                "exact_identity_only"
                    | "transparent_persistence"
                    | "presentation_only"
                    | "target_lowering"
            )
        }
        "proof_gate" => {
            row.data_class == "proof_authority"
                && (valid_stage_contract("verified_authority:")
                    || valid_stage_contract("verified_artifact:"))
        }
        "proof_value_return" => {
            row.data_class == "proof_authority" && valid_stage_contract("verified_serialization:")
        }
        "proof_value_sink" => {
            row.data_class == "proof_authority"
                && row
                    .contract
                    .strip_prefix("verified_argument:")
                    .is_some_and(|value| value.parse::<usize>().is_ok())
        }
        "trusted_region" => {
            row.data_class == "proof_authority"
                && matches!(
                    row.contract.as_str(),
                    "kernel_implementation" | "proof_construction"
                )
        }
        "protected_type" => {
            matches!(
                row.data_class.as_str(),
                "document_identity" | "symbol_identity" | "proof_authority" | "canonical_authority"
            ) && matches!(
                row.contract.as_str(),
                "kernel_only_construct" | "registered_rule_construct"
            )
        }
        "derived_join" => {
            let (expected_path, expected_class) = match row.contract.as_str() {
                "registered_rule_entrypoints" => (
                    "doctrine/production_genericity/rule_family_inventory.tsv#entrypoints",
                    "semantic_control",
                ),
                "registered_grammar_declassifiers" => (
                    "doctrine/production_genericity/rule_family_inventory.tsv#grammar_introduces",
                    "raw_evidence",
                ),
                "proof_only_sinks" => (
                    "doctrine/production_genericity/rule_family_inventory.tsv#canonical_seams",
                    "canonical_authority",
                ),
                _ => ("", ""),
            };
            !expected_path.is_empty()
                && row.path == expected_path
                && row.data_class == expected_class
        }
        _ => false,
    };
    if !valid {
        return Err(format!(
            "role/data-class/path/contract combination is outside schema 1: {} / {} / {} / {}",
            row.role, row.data_class, row.path, row.contract
        ));
    }
    Ok(())
}

fn is_source_class(value: &str) -> bool {
    matches!(
        value,
        "raw_evidence" | "grounded_proposal" | "document_identity" | "symbol_identity"
    )
}

fn is_artifact_stage(value: &str) -> bool {
    matches!(
        value,
        "source_ir" | "evidence_ir" | "semantic_ir" | "intent_ir" | "isf_adapter"
    )
}

fn contract_stage(contract: &str) -> Result<String, String> {
    contract
        .split_once(':')
        .map(|(_, stage)| stage.to_owned())
        .filter(|stage| is_artifact_stage(stage))
        .ok_or_else(|| format!("authority contract '{contract}' has no valid artifact stage"))
}

fn argument_index(contract: &str) -> Result<usize, String> {
    contract
        .strip_prefix("argument:")
        .ok_or_else(|| format!("parameter contract '{contract}' has no argument index"))?
        .parse::<usize>()
        .map_err(|_| format!("parameter contract '{contract}' has an invalid argument index"))
}

fn load_rule_joins(
    root: &Path,
    program: &AnalyzedProgram,
    boundary: &mut Boundary,
) -> Result<(), String> {
    let text = fs::read_to_string(root.join(RULE_PATH))
        .map_err(|error| format!("cannot read {RULE_PATH}: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(RULE_HEADER) {
        return Err(format!("{RULE_PATH} has an unsupported header"));
    }
    for (offset, line) in lines.enumerate() {
        let line_number = offset + 2;
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 13 || fields.iter().any(|field| field.is_empty()) {
            return Err(format!("{RULE_PATH}:{line_number}: malformed schema-1 row"));
        }
        let stage = fields[1];
        let capability = fields[6];
        for cell in [fields[3], fields[4]] {
            for path in split_paths(cell) {
                let resolved = resolve_function_path(program, path)
                    .map_err(|error| format!("{RULE_PATH}:{line_number}: {error}"))?;
                boundary.rule_roots.extend(resolved.iter().cloned());
                if capability == "grammar_introduces" {
                    boundary.grammar_roots.extend(resolved);
                }
            }
        }
        for path in split_paths(fields[11]) {
            for seam in resolve_function_path(program, path)
                .map_err(|error| format!("{RULE_PATH}:{line_number}: {error}"))?
            {
                boundary.canonical_seams.insert(seam.clone());
                boundary
                    .seam_stages
                    .entry(seam)
                    .or_default()
                    .insert(stage.to_owned());
            }
        }
    }
    if boundary.rule_roots.is_empty()
        || boundary.grammar_roots.is_empty()
        || boundary.canonical_seams.is_empty()
    {
        return Err("rule inventory produced an empty structural join".to_owned());
    }
    Ok(())
}

fn load_claim_fields(root: &Path, boundary: &mut Boundary) -> Result<(), String> {
    let text = fs::read_to_string(root.join(CLAIM_PATH))
        .map_err(|error| format!("cannot read {CLAIM_PATH}: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(CLAIM_HEADER) {
        return Err(format!("{CLAIM_PATH} has an unsupported header"));
    }
    for (offset, line) in lines.enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "{CLAIM_PATH}:{}: malformed schema-1 row",
                offset + 2
            ));
        }
        for field in fields[2].split(',') {
            if !boundary.canonical_fields.insert(field.to_owned()) {
                // Repeated field names across artifact stages are expected; one name still denotes one
                // syntax-level mutation surface.
            }
        }
    }
    if boundary.canonical_fields.is_empty() {
        return Err("claim inventory yielded no canonical field names".to_owned());
    }
    Ok(())
}

fn load_bypass_roots(
    root: &Path,
    program: &AnalyzedProgram,
    boundary: &mut Boundary,
) -> Result<(), String> {
    let text = fs::read_to_string(root.join(BYPASS_PATH))
        .map_err(|error| format!("cannot read {BYPASS_PATH}: {error}"))?;
    let mut lines = text.lines();
    if lines.next() != Some(BYPASS_HEADER) {
        return Err(format!("{BYPASS_PATH} has an unsupported header"));
    }
    for (offset, line) in lines.enumerate() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "{BYPASS_PATH}:{}: malformed schema-1 row",
                offset + 2
            ));
        }
        boundary.bypass_roots.extend(
            resolve_function_path(program, fields[0])
                .map_err(|error| format!("{BYPASS_PATH}:{}: {error}", offset + 2))?,
        );
    }
    Ok(())
}

fn collect_functions(
    program: &AnalyzedProgram,
    boundary: &Boundary,
) -> Result<BTreeMap<String, FunctionModel>, String> {
    let planes: BTreeMap<_, _> = program
        .graph
        .inventory_files
        .iter()
        .map(|row| (row.path.as_str(), row.target_plane.as_str()))
        .collect();
    let mut functions = BTreeMap::new();
    for unit in program.modules.values() {
        let plane = planes
            .get(unit.source_path.as_str())
            .ok_or_else(|| format!("module '{}' has no inventory plane", unit.id))?;
        collect_unit_functions(unit, plane, &mut functions)?;
    }
    for item in boundary
        .rule_roots
        .iter()
        .chain(&boundary.grammar_roots)
        .chain(&boundary.canonical_seams)
        .chain(&boundary.proof_gates)
        .chain(&boundary.proof_value_returns)
        .chain(boundary.proof_value_sinks.keys())
        .chain(
            boundary
                .source_parameters
                .keys()
                .map(|(function, _)| function),
        )
        .chain(&boundary.trusted_roots)
        .chain(&boundary.declassifiers)
    {
        if !functions.contains_key(item) {
            return Err(format!("boundary function '{item}' has no analyzable body"));
        }
    }
    for (item, index) in boundary.source_parameters.keys() {
        let inputs = functions
            .get(item)
            .expect("source parameter function was checked above")
            .signature
            .inputs
            .len();
        if *index >= inputs {
            return Err(format!(
                "boundary source parameter '{item}' argument {index} is outside its {inputs}-argument signature"
            ));
        }
    }
    Ok(functions)
}

fn collect_unit_functions(
    unit: &ModuleUnit,
    plane: &str,
    functions: &mut BTreeMap<String, FunctionModel>,
) -> Result<(), String> {
    for item in &unit.items {
        match item {
            Item::Fn(function) => insert_function(
                functions,
                FunctionModel {
                    id: format!("{}::fn:{}", unit.id, function.sig.ident),
                    module: unit.id.clone(),
                    plane: plane.to_owned(),
                    owner: None,
                    owner_terminal: None,
                    signature: function.sig.clone(),
                    block: (*function.block).clone(),
                },
            )?,
            Item::Impl(item_impl) => {
                let owner = normalized_tokens(&item_impl.self_ty);
                let owner_terminal = terminal_type_identifier(&item_impl.self_ty);
                let mut header = item_impl.clone();
                header.items.clear();
                let scope = format!("impl:{}", short_hash(&normalized_tokens(&header)));
                for member in &item_impl.items {
                    if let ImplItem::Fn(function) = member {
                        insert_function(
                            functions,
                            FunctionModel {
                                id: format!("{}::{scope}::impl_fn:{}", unit.id, function.sig.ident),
                                module: unit.id.clone(),
                                plane: plane.to_owned(),
                                owner: Some(owner.clone()),
                                owner_terminal: owner_terminal.clone(),
                                signature: function.sig.clone(),
                                block: function.block.clone(),
                            },
                        )?;
                    }
                }
                let _ = owner;
            }
            Item::Trait(item_trait) => {
                let owner = item_trait.ident.to_string();
                for member in &item_trait.items {
                    if let TraitItem::Fn(function) = member
                        && let Some(block) = &function.default
                    {
                        insert_function(
                            functions,
                            FunctionModel {
                                id: format!(
                                    "{}::trait:{owner}::trait_fn:{}",
                                    unit.id, function.sig.ident
                                ),
                                module: unit.id.clone(),
                                plane: plane.to_owned(),
                                owner: Some(owner.clone()),
                                owner_terminal: None,
                                signature: function.sig.clone(),
                                block: block.clone(),
                            },
                        )?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn insert_function(
    functions: &mut BTreeMap<String, FunctionModel>,
    function: FunctionModel,
) -> Result<(), String> {
    if functions.insert(function.id.clone(), function).is_some() {
        return Err("duplicate analyzable function identity".to_owned());
    }
    Ok(())
}

fn local_edges(
    program: &AnalyzedProgram,
    functions: &BTreeMap<String, FunctionModel>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut edges: BTreeMap<String, BTreeSet<String>> = functions
        .keys()
        .map(|id| (id.clone(), BTreeSet::new()))
        .collect();
    for call in &program.graph.calls {
        if let Some(targets) = edges.get_mut(&call.caller) {
            let mut resolved: BTreeSet<_> = call
                .targets
                .iter()
                .filter(|target| functions.contains_key(*target))
                .cloned()
                .collect();
            if resolved.is_empty() {
                let name = call.syntax.trim_start_matches('.');
                let name = name.split("::").last().unwrap_or(name);
                let candidates: Vec<_> = functions
                    .values()
                    .filter(|function| function.signature.ident == name)
                    .map(|function| function.id.clone())
                    .collect();
                if candidates.len() == 1 {
                    resolved.extend(candidates);
                }
            }
            targets.extend(resolved);
        }
    }
    for function in functions.values() {
        let bound = bound_names(function);
        let references = function_references(&function.block, functions, &bound);
        edges
            .entry(function.id.clone())
            .or_default()
            .extend(references);
        let macro_references = macro_function_references(&function.block, functions, &bound);
        edges
            .entry(function.id.clone())
            .or_default()
            .extend(macro_references);
    }
    edges
}

fn macro_function_references(
    block: &Block,
    functions: &BTreeMap<String, FunctionModel>,
    bound: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut collector = MacroFunctionReferenceCollector {
        names: BTreeSet::new(),
    };
    collector.visit_block(block);
    collector
        .names
        .into_iter()
        .filter(|name| !bound.contains(name))
        .filter_map(|name| {
            let mut candidates = functions
                .values()
                .filter(|function| function.signature.ident == name)
                .map(|function| function.id.clone());
            let first = candidates.next()?;
            candidates.next().is_none().then_some(first)
        })
        .collect()
}

struct MacroFunctionReferenceCollector {
    names: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for MacroFunctionReferenceCollector {
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        collect_token_calls(mac.tokens.clone(), &mut self.names);
        visit::visit_macro(self, mac);
    }
}

fn collect_token_calls(stream: proc_macro2::TokenStream, names: &mut BTreeSet<String>) {
    let tokens: Vec<_> = stream.into_iter().collect();
    for (index, token) in tokens.iter().enumerate() {
        if let proc_macro2::TokenTree::Ident(identifier) = token
            && tokens
                .get(index + 1)
                .is_some_and(|next| matches!(next, proc_macro2::TokenTree::Group(group) if group.delimiter() == proc_macro2::Delimiter::Parenthesis))
        {
            names.insert(identifier.to_string());
        }
        if let proc_macro2::TokenTree::Group(group) = token {
            collect_token_calls(group.stream(), names);
        }
    }
}

fn function_references(
    block: &Block,
    functions: &BTreeMap<String, FunctionModel>,
    bound: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut collector = FunctionReferenceCollector {
        names: BTreeSet::new(),
    };
    collector.visit_block(block);
    let by_name: BTreeMap<_, Vec<_>> = collector
        .names
        .into_iter()
        .filter(|name| !bound.contains(name))
        .map(|name| {
            let matches = functions
                .values()
                .filter(|function| function.signature.ident == name)
                .map(|function| function.id.clone())
                .collect();
            (name, matches)
        })
        .collect();
    by_name
        .into_values()
        .filter(|matches| matches.len() == 1)
        .flatten()
        .collect()
}

fn bound_names(function: &FunctionModel) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for argument in &function.signature.inputs {
        match argument {
            FnArg::Receiver(_) => {
                names.insert("self".to_owned());
            }
            FnArg::Typed(argument) => names.extend(pattern_names(&argument.pat)),
        }
    }
    let mut collector = PatternNameCollector::default();
    collector.visit_block(&function.block);
    names.extend(collector.names);
    names
}

struct FunctionReferenceCollector {
    names: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for FunctionReferenceCollector {
    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if let Some(name) = path.path.segments.last() {
            self.names.insert(name.ident.to_string());
        }
        visit::visit_expr_path(self, path);
    }
}

fn closure(
    roots: &BTreeSet<String>,
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let mut reached = roots.clone();
    let mut pending: VecDeque<_> = roots.iter().cloned().collect();
    while let Some(item) = pending.pop_front() {
        if let Some(targets) = edges.get(&item) {
            for target in targets {
                if reached.insert(target.clone()) {
                    pending.push_back(target.clone());
                }
            }
        }
    }
    reached
}

fn declassifier_closures(
    boundary: &Boundary,
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut roots: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (function, class) in &boundary.declassifier_classes {
        roots
            .entry(class.clone())
            .or_default()
            .insert(function.clone());
    }
    roots
        .into_iter()
        .map(|(class, roots)| (class, closure(&roots, edges)))
        .collect()
}

fn verify_registry_regions(
    boundary: &Boundary,
    rule_closure: &BTreeSet<String>,
    grammar_closure: &BTreeSet<String>,
    trusted_closure: &BTreeSet<String>,
    proof_closure: &BTreeSet<String>,
    non_authority_closure: &BTreeSet<String>,
    declassifier_closure: &BTreeSet<String>,
) -> Result<(), String> {
    if !grammar_closure.is_subset(rule_closure) {
        return Err("registered grammar helper closure escapes the rule closure".to_owned());
    }
    if boundary
        .canonical_seams
        .iter()
        .any(|seam| trusted_closure.contains(seam) && !proof_closure.contains(seam))
    {
        return Err(
            "canonical seam enters a trusted construction region without a proof gate".to_owned(),
        );
    }
    if boundary
        .canonical_seams
        .iter()
        .any(|seam| declassifier_closure.contains(seam))
    {
        return Err("declassifier helper closure reaches a canonical sink".to_owned());
    }
    if boundary
        .canonical_seams
        .iter()
        .any(|seam| non_authority_closure.contains(seam))
        || boundary
            .proof_value_sinks
            .keys()
            .any(|sink| non_authority_closure.contains(sink))
    {
        return Err(
            "non-authoritative presentation/lifecycle/diagnostic closure reaches a canonical sink"
                .to_owned(),
        );
    }
    Ok(())
}

fn verify_canonical_seams(
    boundary: &Boundary,
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    let value_sinks: BTreeSet<_> = boundary.proof_value_sinks.keys().cloned().collect();
    for seam in &boundary.canonical_seams {
        if value_sinks.contains(seam) {
            continue;
        }
        for stage in boundary.seam_stages.get(seam).into_iter().flatten() {
            let stage_targets: BTreeSet<_> = boundary
                .proof_gate_stages
                .iter()
                .chain(&boundary.proof_value_stages)
                .filter(|(_, candidate)| *candidate == stage)
                .map(|(function, _)| function.clone())
                .collect();
            let proof_reachable = reverse_reachable(&stage_targets, edges);
            if !proof_reachable.contains(seam) {
                return Err(format!(
                    "canonical seam '{seam}' for stage '{stage}' has no graph path to a stage-matched proof gate or verified-value boundary"
                ));
            }
        }
    }
    Ok(())
}

fn reverse_reachable(
    targets: &BTreeSet<String>,
    edges: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeSet<String> {
    let mut reverse: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (caller, callees) in edges {
        for callee in callees {
            reverse
                .entry(callee.clone())
                .or_default()
                .insert(caller.clone());
        }
    }
    closure(targets, &reverse)
}

#[allow(clippy::too_many_arguments)]
fn inspect_function(
    function: &FunctionModel,
    program: &AnalyzedProgram,
    functions: &BTreeMap<String, FunctionModel>,
    summaries: &BTreeMap<String, Summary>,
    boundary: &Boundary,
    rule_closure: &BTreeSet<String>,
    grammar_closure: &BTreeSet<String>,
    proof_closure: &BTreeSet<String>,
    trusted_closure: &BTreeSet<String>,
    bypass_closure: &BTreeSet<String>,
    non_authority_closure: &BTreeSet<String>,
    declassifier_closures: &BTreeMap<String, BTreeSet<String>>,
    emit_failures: bool,
) -> Inspection {
    let mut analyzer = FunctionAnalyzer {
        function,
        program,
        functions,
        summaries,
        boundary,
        rule_closure,
        grammar_closure,
        proof_closure,
        trusted_closure,
        bypass_closure,
        non_authority_closure,
        declassifier_closures,
        emit_failures,
        env: BTreeMap::new(),
        env_types: BTreeMap::new(),
        param_classes: BTreeMap::new(),
        return_value: Dependency::default(),
        sink_value: Dependency::default(),
        decision_sites: 0,
        protected_constructions: 0,
        protected_calls: 0,
        semantic_macros: 0,
        failures: Vec::new(),
        call_ordinal: 0,
    };
    analyzer.seed_parameters();
    let tail = analyzer.eval_block(&function.block);
    analyzer.return_value.union(tail);
    analyzer.finish()
}

struct FunctionAnalyzer<'a> {
    function: &'a FunctionModel,
    program: &'a AnalyzedProgram,
    functions: &'a BTreeMap<String, FunctionModel>,
    summaries: &'a BTreeMap<String, Summary>,
    boundary: &'a Boundary,
    rule_closure: &'a BTreeSet<String>,
    grammar_closure: &'a BTreeSet<String>,
    proof_closure: &'a BTreeSet<String>,
    trusted_closure: &'a BTreeSet<String>,
    bypass_closure: &'a BTreeSet<String>,
    non_authority_closure: &'a BTreeSet<String>,
    declassifier_closures: &'a BTreeMap<String, BTreeSet<String>>,
    emit_failures: bool,
    env: BTreeMap<String, Dependency>,
    env_types: BTreeMap<String, String>,
    param_classes: BTreeMap<usize, String>,
    return_value: Dependency,
    sink_value: Dependency,
    decision_sites: usize,
    protected_constructions: usize,
    protected_calls: usize,
    semantic_macros: usize,
    failures: Vec<String>,
    call_ordinal: usize,
}

impl FunctionAnalyzer<'_> {
    fn seed_parameters(&mut self) {
        for (index, argument) in self.function.signature.inputs.iter().enumerate() {
            let (names, ty) = match argument {
                FnArg::Receiver(_) => (vec!["self".to_owned()], None),
                FnArg::Typed(argument) => {
                    (pattern_names(&argument.pat), Some(argument.ty.as_ref()))
                }
            };
            let dependency = Dependency::parameter(index);
            for name in names {
                self.env.insert(name, dependency.clone());
                if let Some(owner) = &self.function.owner
                    && self.function.signature.inputs[index]
                        .to_token_stream()
                        .to_string()
                        .contains("self")
                {
                    self.env_types.insert("self".to_owned(), owner.clone());
                }
            }
            if let Some(ty) = ty {
                let identifiers = type_identifiers(ty);
                if let Some(class) = identifiers
                    .iter()
                    .find_map(|identifier| self.boundary.source_types.get(identifier))
                {
                    self.param_classes.insert(index, class.clone());
                } else if identifiers
                    .iter()
                    .any(|identifier| matches!(identifier.as_str(), "str" | "String"))
                {
                    // Until every historical raw string carrier is replaced by a sealed newtype,
                    // string parameters in the generic core are conservatively source-sensitive.
                    // Registered grammar/proof/presentation regions declassify their exact use;
                    // an unregistered helper does not gain authority merely by accepting `&str`.
                    self.param_classes.insert(index, "raw_evidence".to_owned());
                }
                if let Some(owner) = identifiers.iter().rev().find(|identifier| {
                    self.program.graph.items.iter().any(|item| {
                        item.owner.as_deref().is_some_and(|candidate| {
                            candidate == identifier.as_str()
                                || candidate
                                    .strip_prefix(identifier.as_str())
                                    .is_some_and(|suffix| suffix.trim_start().starts_with('<'))
                        })
                    })
                }) {
                    for name in pattern_names(match &self.function.signature.inputs[index] {
                        FnArg::Typed(argument) => &argument.pat,
                        FnArg::Receiver(_) => continue,
                    }) {
                        self.env_types.insert(name, owner.clone());
                    }
                }
            }
            if let Some(class) = self
                .boundary
                .source_parameters
                .get(&(self.function.id.clone(), index))
            {
                self.param_classes.insert(index, class.clone());
            }
        }
    }

    fn finish(mut self) -> Inspection {
        self.record_sink_policy();
        // Any value returned by a function that branches on sensitive input is control-dependent on
        // that input, even when both branches are literals or the result was assigned through a loop.
        // Propagating the complete sink dependency closes those implicit-flow laundering paths.
        self.return_value.union(self.sink_value.clone());
        Inspection {
            summary: Summary {
                return_params: self.return_value.params,
                return_direct: self.return_value.direct,
                sink_params: self.sink_value.params,
                sink_direct: self.sink_value.direct,
            },
            decision_sites: self.decision_sites,
            protected_constructions: self.protected_constructions,
            protected_calls: self.protected_calls,
            semantic_macros: self.semantic_macros,
            failures: self.failures,
        }
    }

    fn record_sink_policy(&mut self) {
        if !self.emit_failures || !self.sink_value.is_sensitive() {
            return;
        }
        for class in self.dependency_classes(&self.sink_value) {
            if !self.class_allowed_for(&self.function.id, &self.function.plane, &class) {
                self.fail(format!(
                    "{}: {class} reaches semantic control outside its registered region",
                    self.function.id
                ));
            }
        }
    }

    fn dependency_classes(&self, dependency: &Dependency) -> BTreeSet<String> {
        let mut classes = dependency.direct.clone();
        for param in &dependency.params {
            if let Some(class) = self.param_classes.get(param) {
                classes.insert(class.clone());
            }
        }
        classes
    }

    fn class_allowed_for(&self, function: &str, plane: &str, class: &str) -> bool {
        let class_region = match class {
            "raw_evidence" => self.grammar_closure.contains(function),
            "grounded_proposal" => self.rule_closure.contains(function),
            "document_identity" | "symbol_identity" => {
                self.rule_closure.contains(function)
                    || self.proof_closure.contains(function)
                    || self.trusted_closure.contains(function)
            }
            _ => false,
        };
        class_region
            || plane.contains("conformance")
            || self.non_authority_closure.contains(function)
            || self
                .declassifier_closures
                .get(class)
                .is_some_and(|closure| closure.contains(function))
            || self.trusted_closure.contains(function)
            || self.proof_closure.contains(function)
    }

    fn record_called_sink(&mut self, target: &str, dependency: &Dependency) {
        if !dependency.is_sensitive() {
            return;
        }
        let classes = self.dependency_classes(dependency);
        let plane = self
            .functions
            .get(target)
            .map(|function| function.plane.as_str())
            .unwrap_or_default();
        if classes.is_empty()
            || classes
                .iter()
                .any(|class| !self.class_allowed_for(target, plane, class))
        {
            self.record_sink(dependency);
        }
    }

    fn eval_block(&mut self, block: &Block) -> Dependency {
        let saved = self.env.clone();
        let mut tail = Dependency::default();
        for statement in &block.stmts {
            match statement {
                Stmt::Local(local) => {
                    let value = local
                        .init
                        .as_ref()
                        .map(|init| self.eval_expr(&init.expr))
                        .unwrap_or_default();
                    for name in pattern_names(&local.pat) {
                        self.env.insert(name, value.clone());
                    }
                    if let Some(init) = &local.init
                        && let Some((_, diverge)) = &init.diverge
                    {
                        self.eval_expr(diverge);
                    }
                }
                Stmt::Item(_) => {}
                Stmt::Expr(expression, semicolon) => {
                    let value = self.eval_expr(expression);
                    if semicolon.is_none() {
                        tail = value;
                    }
                }
                Stmt::Macro(statement) => {
                    self.eval_macro(&statement.mac);
                }
            }
        }
        self.env = saved;
        tail
    }

    fn eval_expr(&mut self, expression: &Expr) -> Dependency {
        match expression {
            Expr::Array(value) => {
                Dependency::combined(value.elems.iter().map(|item| self.eval_expr(item)))
            }
            Expr::Assign(value) => {
                let right = self.eval_expr(&value.right);
                self.check_canonical_assignment(&value.left, &right);
                self.assign_pattern_expr(&value.left, right.clone());
                right
            }
            Expr::Async(value) => self.eval_block(&value.block),
            Expr::Await(value) => self.eval_expr(&value.base),
            Expr::Binary(value) => {
                let dependency = Dependency::combined([
                    self.eval_expr(&value.left),
                    self.eval_expr(&value.right),
                ]);
                if is_assignment_binary(&value.op) {
                    self.check_canonical_assignment(&value.left, &dependency);
                    self.assign_pattern_expr(&value.left, dependency.clone());
                }
                if is_decision_binary(&value.op) {
                    self.record_sink(&dependency);
                }
                dependency
            }
            Expr::Block(value) => self.eval_block(&value.block),
            Expr::Break(value) => value
                .expr
                .as_ref()
                .map(|expr| self.eval_expr(expr))
                .unwrap_or_default(),
            Expr::Call(value) => self.eval_call(value),
            Expr::Cast(value) => {
                let mut dependency = self.eval_expr(&value.expr);
                dependency.proof_value = false;
                dependency
            }
            Expr::Closure(value) => {
                let saved = self.env.clone();
                for input in &value.inputs {
                    for name in pattern_names(input) {
                        self.env.insert(name, Dependency::default());
                    }
                }
                let result = self.eval_expr(&value.body);
                self.env = saved;
                result
            }
            Expr::Const(value) => self.eval_block(&value.block),
            Expr::Continue(_) | Expr::Infer(_) | Expr::Lit(_) => Dependency::default(),
            Expr::Field(value) => {
                let mut dependency = self.eval_expr(&value.base);
                let field = value.member.to_token_stream().to_string();
                if let Some(variable) = receiver_variable(&value.base)
                    && let Some(owner) = self.env_types.get(&variable)
                    && let Some(class) = self
                        .boundary
                        .source_fields
                        .get(&(owner.clone(), field.clone()))
                {
                    dependency.union(Dependency::direct(class.clone()));
                } else {
                    let classes: BTreeSet<_> = self
                        .boundary
                        .source_fields
                        .iter()
                        .filter(|((_, candidate), _)| candidate == &field)
                        .map(|(_, class)| class.clone())
                        .collect();
                    if classes.len() == 1 {
                        dependency.union(Dependency::direct(
                            classes.into_iter().next().expect("one source-field class"),
                        ));
                    }
                }
                dependency
            }
            Expr::ForLoop(value) => {
                let iterator = self.eval_expr(&value.expr);
                self.record_sink(&iterator);
                let saved = self.env.clone();
                for name in pattern_names(&value.pat) {
                    self.env.insert(name, iterator.clone());
                }
                self.eval_block(&value.body);
                self.env = saved;
                Dependency::default()
            }
            Expr::Group(value) => self.eval_expr(&value.expr),
            Expr::If(value) => {
                let condition = self.eval_expr(&value.cond);
                self.record_sink(&condition);
                let mut output = self.eval_block(&value.then_branch);
                if let Some((_, branch)) = &value.else_branch {
                    output.union(self.eval_expr(branch));
                }
                output.proof_value = false;
                output
            }
            Expr::Index(value) => {
                let dependency = Dependency::combined([
                    self.eval_expr(&value.expr),
                    self.eval_expr(&value.index),
                ]);
                self.record_sink(&dependency);
                dependency
            }
            Expr::Let(value) => {
                let dependency = self.eval_expr(&value.expr);
                self.record_sink(&dependency);
                dependency
            }
            Expr::Loop(value) => self.eval_block(&value.body),
            Expr::Macro(value) => self.eval_macro(&value.mac),
            Expr::Match(value) => {
                let discriminant = self.eval_expr(&value.expr);
                self.record_sink(&discriminant);
                let mut output = Dependency::default();
                for arm in &value.arms {
                    if let Some((_, guard)) = &arm.guard {
                        let guard = self.eval_expr(guard);
                        self.record_sink(&guard);
                    }
                    output.union(self.eval_expr(&arm.body));
                }
                output.proof_value = false;
                output
            }
            Expr::MethodCall(value) => self.eval_method_call(value),
            Expr::Paren(value) => self.eval_expr(&value.expr),
            Expr::Path(value) => value
                .path
                .get_ident()
                .and_then(|ident| self.env.get(&ident.to_string()))
                .cloned()
                .unwrap_or_default(),
            Expr::Range(value) => Dependency::combined(
                value
                    .start
                    .iter()
                    .chain(value.end.iter())
                    .map(|expr| self.eval_expr(expr)),
            ),
            Expr::RawAddr(value) => self.eval_expr(&value.expr),
            Expr::Reference(value) => {
                let mut dependency = self.eval_expr(&value.expr);
                if value.mutability.is_some()
                    && let Some(field) = terminal_field(&value.expr)
                    && self.boundary.canonical_fields.contains(&field)
                {
                    dependency.canonical_fields.insert(field);
                }
                dependency
            }
            Expr::Repeat(value) => {
                Dependency::combined([self.eval_expr(&value.expr), self.eval_expr(&value.len)])
            }
            Expr::Return(value) => {
                let dependency = value
                    .expr
                    .as_ref()
                    .map(|expr| self.eval_expr(expr))
                    .unwrap_or_default();
                self.return_value.union(dependency.clone());
                dependency
            }
            Expr::Struct(value) => {
                let terminal = value
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string());
                self.check_protected_construction(terminal.clone());
                let mut dependency = Dependency::default();
                for field in &value.fields {
                    dependency.union(self.eval_expr(&field.expr));
                }
                if let Some(rest) = &value.rest {
                    dependency.union(self.eval_expr(rest));
                }
                if let Some(class) = terminal
                    .as_ref()
                    .and_then(|name| self.boundary.source_types.get(name))
                {
                    dependency.union(Dependency::direct(class.clone()));
                }
                dependency.proof_value = false;
                dependency
            }
            Expr::Try(value) => self.eval_expr(&value.expr),
            Expr::TryBlock(value) => self.eval_block(&value.block),
            Expr::Tuple(value) => {
                Dependency::combined(value.elems.iter().map(|item| self.eval_expr(item)))
            }
            Expr::Unary(value) => {
                let mut dependency = self.eval_expr(&value.expr);
                dependency.proof_value = false;
                dependency
            }
            Expr::Unsafe(value) => self.eval_block(&value.block),
            Expr::While(value) => {
                let condition = self.eval_expr(&value.cond);
                self.record_sink(&condition);
                self.eval_block(&value.body);
                Dependency::default()
            }
            Expr::Yield(value) => value
                .expr
                .as_ref()
                .map(|expr| self.eval_expr(expr))
                .unwrap_or_default(),
            Expr::Verbatim(_) => {
                self.fail(format!("{}: opaque verbatim expression", self.function.id));
                Dependency::default()
            }
            _ => {
                self.fail(format!(
                    "{}: unsupported expression survives information-flow parsing: {}",
                    self.function.id,
                    normalized_tokens(expression)
                ));
                Dependency::default()
            }
        }
    }

    fn eval_call(&mut self, call: &syn::ExprCall) -> Dependency {
        let ordinal = self.call_ordinal;
        self.call_ordinal += 1;
        let arguments: Vec<_> = call
            .args
            .iter()
            .map(|argument| self.eval_expr(argument))
            .collect();
        let targets = self.call_targets(ordinal);
        let terminal = callable_terminal(&call.func);
        self.check_protected_construction(terminal.clone());
        self.check_protected_call(terminal.as_deref(), &targets);
        let mut output = Dependency::default();
        if targets.is_empty() {
            output = Dependency::combined(arguments.iter().cloned());
        }
        for target in &targets {
            if let Some(summary) = self.summaries.get(target) {
                if !self.boundary.proof_gates.contains(target) {
                    output.union(apply_summary_dependencies(
                        &summary.return_params,
                        &summary.return_direct,
                        &arguments,
                    ));
                }
                let sink =
                    apply_summary_dependencies(&summary.sink_params, &BTreeSet::new(), &arguments);
                self.record_called_sink(target, &sink);
            }
            if let Some(class) = self.boundary.source_returns.get(target) {
                output.union(Dependency::direct(class.clone()));
            }
            if self.boundary.proof_value_returns.contains(target) {
                output.proof_value = true;
            }
            if let Some(index) = self.boundary.proof_value_sinks.get(target) {
                self.check_proof_value_argument(*index, &arguments);
            }
        }
        output
    }

    fn eval_method_call(&mut self, call: &syn::ExprMethodCall) -> Dependency {
        let ordinal = self.call_ordinal;
        self.call_ordinal += 1;
        let mut arguments = vec![self.eval_expr(&call.receiver)];
        arguments.extend(call.args.iter().map(|argument| self.eval_expr(argument)));
        let mut targets = self.call_targets(ordinal);
        let method = call.method.to_string();
        targets.extend(self.typed_method_targets(&method, &call.receiver));
        targets.sort();
        targets.dedup();
        self.check_protected_call(Some(&method), &targets);
        let mut output = Dependency::combined(arguments.iter().cloned());
        for target in &targets {
            if let Some(summary) = self.summaries.get(target) {
                if !self.boundary.proof_gates.contains(target) {
                    output.union(apply_summary_dependencies(
                        &summary.return_params,
                        &summary.return_direct,
                        &arguments,
                    ));
                }
                let sink =
                    apply_summary_dependencies(&summary.sink_params, &BTreeSet::new(), &arguments);
                self.record_called_sink(target, &sink);
            }
            if let Some(class) = self.boundary.source_returns.get(target) {
                output.union(Dependency::direct(class.clone()));
            }
            if self.boundary.proof_value_returns.contains(target) {
                output.proof_value = true;
            }
            if let Some(index) = self.boundary.proof_value_sinks.get(target) {
                self.check_proof_value_argument(*index, &arguments);
            }
        }
        if is_decision_method(&method) {
            self.record_sink(&output);
        }
        if is_canonical_mutator(&method) && !self.in_registered_mutation_region() {
            let mut fields = arguments[0].canonical_fields.clone();
            if let Some(field) = terminal_field(&call.receiver)
                && self.boundary.canonical_fields.contains(&field)
            {
                fields.insert(field);
            }
            for field in fields {
                self.fail(format!(
                    "{}: canonical field '{field}' is mutated outside a registered rule or conformance overlay",
                    self.function.id
                ));
            }
        }
        output.proof_value = targets
            .iter()
            .any(|target| self.boundary.proof_value_returns.contains(target));
        output
    }

    fn eval_macro(&mut self, mac: &syn::Macro) -> Dependency {
        let mut dependencies = macro_dependencies(&mac.tokens, &self.env);
        dependencies.proof_value = false;
        if !dependencies.is_sensitive() {
            return dependencies;
        }
        self.semantic_macros += 1;
        let name = mac
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default();
        if is_decision_macro(&name) {
            self.record_sink(&dependencies);
        } else if !is_transparent_macro(&name) {
            self.fail(format!(
                "{}: sensitive value enters unresolved semantic macro '{}!'",
                self.function.id, name
            ));
        }
        dependencies
    }

    fn call_targets(&self, ordinal: usize) -> Vec<String> {
        let id = format!("{}::call:{ordinal}", self.function.id);
        let Some(call) = self.program.graph.calls.iter().find(|call| call.id == id) else {
            return Vec::new();
        };
        let mut targets: Vec<_> = call
            .targets
            .iter()
            .filter(|target| self.functions.contains_key(*target))
            .cloned()
            .collect();
        if targets.is_empty() && !call.syntax.starts_with('.') {
            let name = call.syntax.split("::").last().unwrap_or(&call.syntax);
            let mut candidates = self
                .functions
                .values()
                .filter(|function| function.signature.ident == name)
                .map(|function| function.id.clone());
            if let Some(first) = candidates.next()
                && candidates.next().is_none()
            {
                targets.push(first);
            }
        }
        targets
    }

    fn record_sink(&mut self, dependency: &Dependency) {
        if dependency.is_sensitive() {
            self.decision_sites += 1;
            self.sink_value.union(dependency.clone());
        }
    }

    fn check_protected_construction(&mut self, terminal: Option<String>) {
        let Some(mut name) = terminal else { return };
        if name == "Self"
            && let Some(owner) = &self.function.owner_terminal
        {
            name.clone_from(owner);
        }
        let Some(contract) = self.boundary.protected_types.get(&name) else {
            return;
        };
        self.protected_constructions += 1;
        let allowed = match contract.as_str() {
            "kernel_only_construct" => self.trusted_closure.contains(&self.function.id),
            "registered_rule_construct" => {
                self.rule_closure.contains(&self.function.id)
                    || self.bypass_closure.contains(&self.function.id)
            }
            _ => false,
        };
        if !allowed {
            self.fail(format!(
                "{}: protected type '{name}' is constructed outside its {contract} region",
                self.function.id
            ));
        }
    }

    fn check_protected_call(&mut self, terminal: Option<&str>, targets: &[String]) {
        let Some(name) = terminal else { return };
        if !matches!(
            name,
            "grammar_capability"
                | "persistence_capability"
                | "presentation_capability"
                | "lowering_capability"
                | "promote"
                | "seal"
                | "finish"
                | "verify_persisted"
        ) {
            return;
        }
        if !targets.iter().any(|target| {
            target.contains("::crate::ir::derivation::") && target.rsplit(':').next() == Some(name)
        }) {
            return;
        }
        self.protected_calls += 1;
        if !self.trusted_closure.contains(&self.function.id) {
            self.fail(format!(
                "{}: protected proof/capability call '{name}' occurs outside a trusted region",
                self.function.id
            ));
        }
    }

    fn check_proof_value_argument(&mut self, index: usize, arguments: &[Dependency]) {
        if !arguments
            .get(index)
            .is_some_and(|argument| argument.proof_value)
        {
            self.fail(format!(
                "{}: verified-value sink argument {index} is not derived from a registered proof serialization",
                self.function.id
            ));
        }
    }

    fn check_canonical_assignment(&mut self, left: &Expr, _right: &Dependency) {
        if let Some(field) = terminal_field(left)
            && self.boundary.canonical_fields.contains(&field)
            && !self.in_registered_mutation_region()
        {
            self.fail(format!(
                "{}: canonical field '{field}' is assigned outside a registered rule or conformance overlay",
                self.function.id
            ));
        }
    }

    fn assign_pattern_expr(&mut self, left: &Expr, dependency: Dependency) {
        if let Expr::Path(path) = left
            && let Some(name) = path.path.get_ident()
        {
            self.env.insert(name.to_string(), dependency);
        }
    }

    fn in_registered_mutation_region(&self) -> bool {
        self.rule_closure.contains(&self.function.id)
            || self.trusted_closure.contains(&self.function.id)
            || self.bypass_closure.contains(&self.function.id)
            || self.function.plane.contains("conformance")
    }

    fn typed_method_targets(&self, method: &str, receiver: &Expr) -> Vec<String> {
        let Some(variable) = receiver_variable(receiver) else {
            return Vec::new();
        };
        let Some(owner) = self.env_types.get(&variable) else {
            return Vec::new();
        };
        self.program
            .graph
            .items
            .iter()
            .filter(|item| {
                item.name == method
                    && matches!(item.kind.as_str(), "impl_fn" | "trait_fn")
                    && item.owner.as_deref().is_some_and(|candidate| {
                        candidate == owner
                            || candidate
                                .strip_prefix(owner)
                                .is_some_and(|suffix| suffix.trim_start().starts_with('<'))
                    })
            })
            .map(|item| item.id.clone())
            .collect()
    }

    fn fail(&mut self, message: String) {
        if self.emit_failures {
            self.failures.push(message);
        }
    }
}

fn type_identifiers(ty: &Type) -> Vec<String> {
    let mut identifiers = TypeIdentifierCollector::default();
    identifiers.visit_type(ty);
    identifiers.identifiers.into_iter().collect()
}

fn terminal_type_identifier(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        Type::Reference(reference) => terminal_type_identifier(&reference.elem),
        Type::Paren(paren) => terminal_type_identifier(&paren.elem),
        Type::Group(group) => terminal_type_identifier(&group.elem),
        _ => None,
    }
}

#[derive(Default)]
struct TypeIdentifierCollector {
    identifiers: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for TypeIdentifierCollector {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        self.identifiers.extend(
            path.segments
                .iter()
                .map(|segment| segment.ident.to_string()),
        );
        visit::visit_path(self, path);
    }
}

fn pattern_names(pattern: &Pat) -> Vec<String> {
    let mut collector = PatternNameCollector::default();
    collector.visit_pat(pattern);
    collector.names.into_iter().collect()
}

#[derive(Default)]
struct PatternNameCollector {
    names: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for PatternNameCollector {
    fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
        self.names.insert(pattern.ident.to_string());
        visit::visit_pat_ident(self, pattern);
    }
}

fn apply_summary_dependencies(
    params: &BTreeSet<usize>,
    direct: &BTreeSet<String>,
    arguments: &[Dependency],
) -> Dependency {
    let mut result = Dependency::default();
    for index in params {
        if let Some(argument) = arguments.get(*index) {
            result.union(argument.clone());
        }
    }
    result.direct.extend(direct.iter().cloned());
    result
}

fn macro_dependencies(
    tokens: &proc_macro2::TokenStream,
    env: &BTreeMap<String, Dependency>,
) -> Dependency {
    let mut result = Dependency::default();
    fn walk(
        stream: proc_macro2::TokenStream,
        env: &BTreeMap<String, Dependency>,
        result: &mut Dependency,
    ) {
        for token in stream {
            match token {
                proc_macro2::TokenTree::Ident(identifier) => {
                    if let Some(value) = env.get(&identifier.to_string()) {
                        result.union(value.clone());
                    }
                }
                proc_macro2::TokenTree::Group(group) => walk(group.stream(), env, result),
                _ => {}
            }
        }
    }
    walk(tokens.clone(), env, &mut result);
    result
}

fn is_decision_binary(operator: &BinOp) -> bool {
    matches!(
        operator,
        BinOp::Eq(_) | BinOp::Lt(_) | BinOp::Le(_) | BinOp::Ne(_) | BinOp::Ge(_) | BinOp::Gt(_)
    )
}

fn is_assignment_binary(operator: &BinOp) -> bool {
    matches!(
        operator,
        BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_)
    )
}

fn is_decision_method(name: &str) -> bool {
    matches!(
        name,
        "contains"
            | "starts_with"
            | "ends_with"
            | "eq_ignore_ascii_case"
            | "find"
            | "rfind"
            | "is_match"
            | "position"
            | "rposition"
            | "any"
            | "all"
            | "find_map"
            | "binary_search"
            | "binary_search_by"
            | "binary_search_by_key"
            | "cmp"
            | "partial_cmp"
            | "sort"
            | "sort_by"
            | "sort_by_key"
            | "sort_unstable"
            | "sort_unstable_by"
            | "sort_unstable_by_key"
            | "hash"
    )
}

fn is_canonical_mutator(name: &str) -> bool {
    matches!(
        name,
        "push"
            | "extend"
            | "extend_from_slice"
            | "insert"
            | "append"
            | "clear"
            | "retain"
            | "remove"
            | "swap_remove"
            | "truncate"
            | "dedup"
            | "dedup_by"
            | "dedup_by_key"
    )
}

fn is_decision_macro(name: &str) -> bool {
    matches!(
        name,
        "matches"
            | "assert"
            | "assert_eq"
            | "assert_ne"
            | "debug_assert"
            | "debug_assert_eq"
            | "debug_assert_ne"
            | "ensure"
    )
}

fn is_transparent_macro(name: &str) -> bool {
    matches!(
        name,
        "format"
            | "format_args"
            | "vec"
            | "json"
            | "write"
            | "writeln"
            | "print"
            | "println"
            | "eprint"
            | "eprintln"
            | "anyhow"
            | "bail"
            | "debug"
            | "trace"
            | "info"
            | "warn"
            | "error"
    )
}

fn callable_terminal(expression: &Expr) -> Option<String> {
    match expression {
        Expr::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        Expr::Paren(value) => callable_terminal(&value.expr),
        Expr::Group(value) => callable_terminal(&value.expr),
        _ => None,
    }
}

fn receiver_variable(expression: &Expr) -> Option<String> {
    match expression {
        Expr::Path(path) => path.path.get_ident().map(ToString::to_string),
        Expr::Paren(value) => receiver_variable(&value.expr),
        Expr::Group(value) => receiver_variable(&value.expr),
        Expr::Reference(value) => receiver_variable(&value.expr),
        _ => None,
    }
}

fn terminal_field(expression: &Expr) -> Option<String> {
    match expression {
        Expr::Field(field) => Some(field.member.to_token_stream().to_string()),
        Expr::Paren(value) => terminal_field(&value.expr),
        Expr::Reference(value) => terminal_field(&value.expr),
        _ => None,
    }
}

fn split_paths(cell: &str) -> impl Iterator<Item = &str> {
    cell.split(';').filter(|value| *value != "-")
}

fn terminal_symbol(path: &str) -> Result<String, String> {
    let (_, symbol) = split_registry_path(path)?;
    symbol
        .split("::")
        .last()
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("registry path '{path}' has no terminal symbol"))
}

fn split_owner_field(path: &str) -> Result<(String, String), String> {
    let (_, symbol) = split_registry_path(path)?;
    let (owner, field) = symbol
        .split_once("::")
        .ok_or_else(|| format!("source field path '{path}' must name Type::field"))?;
    if owner.is_empty() || field.is_empty() || field.contains("::") {
        return Err(format!("source field path '{path}' is malformed"));
    }
    Ok((owner.to_owned(), field.to_owned()))
}

fn split_registry_path(path: &str) -> Result<(&str, &str), String> {
    let (source, symbol) = path
        .split_once('#')
        .ok_or_else(|| format!("registry path '{path}' lacks '#'"))?;
    if Path::new(source).is_absolute()
        || source.split('/').any(|component| component == "..")
        || !source.ends_with(".rs")
        || symbol.is_empty()
    {
        return Err(format!("registry path '{path}' is unsafe or malformed"));
    }
    Ok((source, symbol))
}

fn resolve_module_path(program: &AnalyzedProgram, path: &str) -> Result<Vec<String>, String> {
    let (source, symbol) = split_registry_path(path)?;
    if symbol != "module" {
        return Err(format!("module boundary '{path}' must end in #module"));
    }
    let modules: Vec<_> = program
        .graph
        .modules
        .iter()
        .filter(|module| module.source_path == source)
        .map(|module| module.id.clone())
        .collect();
    if modules.is_empty() {
        return Err(format!(
            "registry module path '{path}' has no compiled module"
        ));
    }
    Ok(modules)
}

fn resolve_function_path(program: &AnalyzedProgram, path: &str) -> Result<Vec<String>, String> {
    let (source, symbol) = split_registry_path(path)?;
    let function = symbol.split("::").last().unwrap_or_default();
    let owner = symbol
        .split_once("::")
        .map(|(owner, _)| owner)
        .filter(|owner| !owner.is_empty());
    let modules: BTreeSet<_> = program
        .graph
        .modules
        .iter()
        .filter(|module| module.source_path == source)
        .map(|module| module.id.as_str())
        .collect();
    let mut items: Vec<_> = program
        .graph
        .items
        .iter()
        .filter(|item| {
            modules.contains(item.module.as_str())
                && matches!(item.kind.as_str(), "fn" | "impl_fn" | "trait_fn")
                && item.name == function
                && owner.is_none_or(|owner| {
                    item.owner.as_deref().is_some_and(|candidate| {
                        candidate == owner
                            || candidate
                                .strip_prefix(owner)
                                .is_some_and(|suffix| suffix.trim_start().starts_with('<'))
                    })
                })
        })
        .map(|item| item.id.clone())
        .collect();
    items.sort();
    items.dedup();
    if items.is_empty() {
        return Err(format!(
            "registry function path '{path}' has no compiled item"
        ));
    }
    Ok(items)
}

fn require_item_kind(program: &AnalyzedProgram, path: &str, kinds: &[&str]) -> Result<(), String> {
    let (source, symbol) = split_registry_path(path)?;
    let name = symbol.split("::").last().unwrap_or_default();
    let modules: BTreeSet<_> = program
        .graph
        .modules
        .iter()
        .filter(|module| module.source_path == source)
        .map(|module| module.id.as_str())
        .collect();
    if !program.graph.items.iter().any(|item| {
        modules.contains(item.module.as_str())
            && kinds.contains(&item.kind.as_str())
            && item.name == name
    }) {
        return Err(format!(
            "registry item path '{path}' has no compiled {} item",
            kinds.join("/")
        ));
    }
    Ok(())
}

fn require_struct_field(
    program: &AnalyzedProgram,
    path: &str,
    owner: &str,
    field: &str,
) -> Result<(), String> {
    let (source, _) = split_registry_path(path)?;
    let found = program
        .modules
        .values()
        .filter(|module| module.source_path == source)
        .flat_map(|module| &module.items)
        .any(|item| {
            let Item::Struct(item) = item else {
                return false;
            };
            item.ident == owner
                && item
                    .fields
                    .iter()
                    .any(|candidate| candidate.ident.as_ref().is_some_and(|name| name == field))
        });
    if !found {
        return Err(format!(
            "registry source field path '{path}' has no compiled struct field"
        ));
    }
    Ok(())
}

fn insert_unique(
    map: &mut BTreeMap<String, String>,
    key: String,
    value: &str,
    label: &str,
) -> Result<(), String> {
    if map.insert(key.clone(), value.to_owned()).is_some() {
        return Err(format!("duplicate {label} terminal '{key}'"));
    }
    Ok(())
}
