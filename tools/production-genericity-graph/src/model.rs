use serde::Serialize;

pub const GRAPH_SCHEMA_VERSION: u32 = 1;

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
