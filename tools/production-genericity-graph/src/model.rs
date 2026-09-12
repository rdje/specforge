use serde::Serialize;

pub const GRAPH_SCHEMA_VERSION: u32 = 1;
pub const FLOW_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct ProductionGraph {
    pub schema_version: u32,
    pub inventory_files: Vec<InventoryFile>,
    pub reachable_files: Vec<String>,
    pub excluded_test_support_files: Vec<String>,
    pub targets: Vec<TargetNode>,
    pub modules: Vec<ModuleNode>,
    pub items: Vec<ItemNode>,
    pub imports: Vec<ImportEdge>,
    pub calls: Vec<CallEdge>,
    pub macros: Vec<MacroNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InventoryFile {
    pub path: String,
    pub current_plane: String,
    pub target_plane: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TargetNode {
    pub id: String,
    pub package: String,
    pub target: String,
    pub kind: String,
    pub root: String,
    pub enabled_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ModuleNode {
    pub id: String,
    pub target: String,
    pub module_path: String,
    pub source_path: String,
    pub inline: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ItemNode {
    pub id: String,
    pub module: String,
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportEdge {
    pub id: String,
    pub module: String,
    pub source_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding: Option<String>,
    pub glob: bool,
    pub resolution: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CallEdge {
    pub id: String,
    pub caller: String,
    pub syntax: String,
    pub resolution: String,
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MacroNode {
    pub id: String,
    pub module: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containing_item: Option<String>,
    pub name: String,
    pub kind: String,
    pub resolution: String,
    pub targets: Vec<String>,
}

impl ProductionGraph {
    pub fn summary(&self) -> String {
        let definitions = self
            .macros
            .iter()
            .filter(|node| node.kind == "definition")
            .count();
        let invocations = self.macros.len() - definitions;
        format!(
            "production-genericity-graph: {} inventoried files; {} reachable; {} test-support-only; {} targets; {} modules; {} items; {} imports; {} calls; {} macro definitions; {} macro invocations",
            self.inventory_files.len(),
            self.reachable_files.len(),
            self.excluded_test_support_files.len(),
            self.targets.len(),
            self.modules.len(),
            self.items.len(),
            self.imports.len(),
            self.calls.len(),
            definitions,
            invocations,
        )
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct InformationFlowReport {
    pub schema_version: u32,
    pub boundary_rows: usize,
    pub source_types: usize,
    pub source_fields: usize,
    pub source_parameters: usize,
    pub source_returns: usize,
    pub rule_roots: usize,
    pub grammar_declassifiers: usize,
    pub canonical_seams: usize,
    pub proof_gates: usize,
    pub trusted_regions: usize,
    pub non_authoritative_regions: usize,
    pub protected_types: usize,
    pub analyzed_functions: usize,
    pub helper_edges: usize,
    pub decision_sites: usize,
    pub protected_constructions: usize,
    pub protected_calls: usize,
    pub semantic_macros: usize,
}

impl InformationFlowReport {
    /// The census this report publishes, by field name — the surface
    /// `doctrine/production_genericity/flow_census.json` declares and
    /// `check_production_genericity_flow.sh` compares (`PRODUCTION-GRAPH-CENSUS-PIN.1`).
    ///
    /// Derived from the struct's own fields rather than restated, so a field added to the report and
    /// forgotten here cannot silently escape the contract: the comparison reports any field present on
    /// one side and absent on the other.
    pub fn census_fields(&self) -> std::collections::BTreeMap<&'static str, usize> {
        std::collections::BTreeMap::from([
            ("boundary_rows", self.boundary_rows),
            ("source_types", self.source_types),
            ("source_fields", self.source_fields),
            ("source_parameters", self.source_parameters),
            ("source_returns", self.source_returns),
            ("rule_roots", self.rule_roots),
            ("grammar_declassifiers", self.grammar_declassifiers),
            ("canonical_seams", self.canonical_seams),
            ("proof_gates", self.proof_gates),
            ("trusted_regions", self.trusted_regions),
            ("non_authoritative_regions", self.non_authoritative_regions),
            ("protected_types", self.protected_types),
            ("analyzed_functions", self.analyzed_functions),
            ("helper_edges", self.helper_edges),
            ("decision_sites", self.decision_sites),
            ("protected_constructions", self.protected_constructions),
            ("protected_calls", self.protected_calls),
            ("semantic_macros", self.semantic_macros),
        ])
    }

    pub fn summary(&self) -> String {
        format!(
            "production-genericity-flow: {} boundary rows; {} source types; {} source fields; {} source parameters; {} source returns; {} rule roots; {} grammar declassifiers; {} canonical seams; {} proof gates; {} trusted regions; {} non-authoritative regions; {} protected types; {} functions; {} helper edges; {} decision sites; {} protected constructions; {} protected calls; {} semantic macros",
            self.boundary_rows,
            self.source_types,
            self.source_fields,
            self.source_parameters,
            self.source_returns,
            self.rule_roots,
            self.grammar_declassifiers,
            self.canonical_seams,
            self.proof_gates,
            self.trusted_regions,
            self.non_authoritative_regions,
            self.protected_types,
            self.analyzed_functions,
            self.helper_edges,
            self.decision_sites,
            self.protected_constructions,
            self.protected_calls,
            self.semantic_macros,
        )
    }
}
