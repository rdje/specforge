use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::model::InventoryFile;

const INVENTORY_PATH: &str = "doctrine/production_genericity/module_inventory.tsv";
const HEADER: &str = "path\tcurrent_plane\ttarget_plane\tprimary_lane\tdisposition";

#[derive(Debug, Clone)]
pub(crate) struct InventoryRow {
    pub path: String,
    pub current_plane: String,
    pub target_plane: String,
}

pub(crate) fn read_inventory(root: &Path) -> Result<BTreeMap<String, InventoryRow>, String> {
    let text = fs::read_to_string(root.join(INVENTORY_PATH))
        .map_err(|error| format!("cannot read {INVENTORY_PATH}: {error}"))?;
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| format!("{INVENTORY_PATH} is empty"))?;
    if header != HEADER {
        return Err(format!("{INVENTORY_PATH} has an unsupported header"));
    }

    let mut rows = BTreeMap::new();
    for (offset, line) in lines.enumerate() {
        let line_number = offset + 2;
        if line.is_empty() {
            return Err(format!("{INVENTORY_PATH}:{line_number}: blank row"));
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 5 || fields.iter().any(|field| field.is_empty()) {
            return Err(format!(
                "{INVENTORY_PATH}:{line_number}: expected five non-empty tab-separated fields"
            ));
        }
        let path = fields[0];
        if Path::new(path).is_absolute()
            || path.split('/').any(|component| component == "..")
            || !path.ends_with(".rs")
        {
            return Err(format!(
                "{INVENTORY_PATH}:{line_number}: unsafe Rust path '{path}'"
            ));
        }
        let row = InventoryRow {
            path: path.to_owned(),
            current_plane: fields[1].to_owned(),
            target_plane: fields[2].to_owned(),
        };
        if rows.insert(path.to_owned(), row).is_some() {
            return Err(format!(
                "{INVENTORY_PATH}:{line_number}: duplicate path '{path}'"
            ));
        }
    }
    if rows.is_empty() {
        return Err(format!("{INVENTORY_PATH} has no data rows"));
    }
    Ok(rows)
}

impl From<&InventoryRow> for InventoryFile {
    fn from(row: &InventoryRow) -> Self {
        Self {
            path: row.path.clone(),
            current_plane: row.current_plane.clone(),
            target_plane: row.target_plane.clone(),
        }
    }
}
