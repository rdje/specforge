use std::fs;
use std::process::Command;

use crate::cli::{EnrichArgs, VlmProviderArg};
use crate::error::{AppError, Result};
#[cfg(test)]
use crate::ir::source::timing_table_has_structural_authority;
use crate::ir::source::{DiagramKind, SourceIr, StructuredTableRecord, TableKind, VisualAsset};

/// Environment variable overriding the VLM helper script (for unit testing).
const VLM_HELPER_ENV: &str = "SPECFORGE_VLM_HELPER";

/// Enrich a SourceIR artifact with VLM-derived visual observations.
///
/// Architecture:
///   Step A (always): classify each `VisualAsset` as `timing_diagram`,
///   `state_machine_diagram`, `block_diagram`, etc. from caption text.
///   This is already done by the Python ingest helper — this step is a no-op
///   if the field is already set, and provides a re-classification opportunity.
///
///   Step B (requires VLM): for each asset with `diagram_kind == TimingDiagram`
///   or `StateMachineDiagram`, call the configured VLM with a structured prompt
///   and parse the response into typed `VisualObservation` records.
pub fn run(args: EnrichArgs) -> Result<()> {
    let source_ir_path = if args.source_ir.exists() {
        args.source_ir.clone()
    } else {
        return Err(AppError::MissingPath(args.source_ir));
    };

    let mut source_ir = SourceIr::load_from_path(&source_ir_path)?;

    // Count assets by diagram kind.
    let timing_count = source_ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::TimingDiagram))
        .count();
    let state_machine_count = source_ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::StateMachineDiagram))
        .count();
    let unknown_count = source_ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::Unknown))
        .count();

    println!("command: enrich");
    println!("mode: {}", if args.dry_run { "dry-run" } else { "execute" });
    println!("source_ir_path: {}", source_ir_path.display());
    println!("document_key: {}", source_ir.document_identity.document_key);
    println!("visual_assets_total: {}", source_ir.visual_assets.len());
    println!("timing_diagrams: {timing_count}");
    println!("state_machine_diagrams: {state_machine_count}");
    println!("diagram_kind_unknown: {unknown_count}");

    if args.classify_only {
        println!("classify_only: true (no VLM calls)");
        return Ok(());
    }

    match args.vlm_provider {
        VlmProviderArg::Skip => {
            println!("vlm_provider: skip");
            println!("vlm_enrichment: skipped");
            println!(
                "hint: re-run with --vlm-provider ollama or --vlm-provider openai to enrich timing diagrams and state machines"
            );
            return Ok(());
        }
        provider => {
            let provider_name = match provider {
                VlmProviderArg::Ollama => "ollama",
                VlmProviderArg::OpenAi => "openai",
                VlmProviderArg::LmStudio => "lmstudio",
                VlmProviderArg::Skip => unreachable!(),
            };
            println!("vlm_provider: {provider_name}");

            let model = args.vlm_model.unwrap_or_else(|| match provider {
                // qwen2.5vl:7b is the recommended open-source VLM for diagram extraction.
                // It outperforms GPT-4o-mini on document/diagram understanding tasks.
                VlmProviderArg::Ollama => "qwen2.5vl:7b".to_string(),
                VlmProviderArg::OpenAi => "gpt-4o".to_string(),
                VlmProviderArg::LmStudio => "qwen2.5vl:7b".to_string(),
                VlmProviderArg::Skip => unreachable!(),
            });
            println!("vlm_model: {model}");

            let api_url = match provider {
                VlmProviderArg::Ollama => "http://localhost:11434/v1/chat/completions".to_string(),
                VlmProviderArg::OpenAi => "https://api.openai.com/v1/chat/completions".to_string(),
                VlmProviderArg::LmStudio => "http://localhost:1234/v1/chat/completions".to_string(),
                VlmProviderArg::Skip => unreachable!(),
            };

            let enriched = enrich_visual_assets(
                &source_ir.visual_assets,
                provider,
                &model,
                &api_url,
                args.dry_run,
            )?;

            println!("vlm_calls_made: {}", enriched.calls_made);
            println!("timing_diagrams_enriched: {}", enriched.timing_enriched);
            println!(
                "state_machines_enriched: {}",
                enriched.state_machine_enriched
            );
            println!("vlm_errors: {}", enriched.errors);

            for (asset_id, diagram_kind, exact_response) in enriched.observations {
                source_ir.apply_visual_observation(&asset_id, diagram_kind, exact_response)?;
            }
            // PDF-VARIANT-DIGESTION.2d — count non-data noise tables (TOC / revision / index) that the VLM
            // passes skip, so they don't waste calls and the unknown count is reported honestly.
            let noise_tables = source_ir
                .structured_tables
                .iter()
                .filter(|t| {
                    t.table_kind == TableKind::Unknown && crate::ir::evidence::table_is_noise(t)
                })
                .count();
            println!("tables_skipped_as_noise: {noise_tables}");
            // PDF-VARIANT-DIGESTION.2b' — VLM grid repair: re-extract degenerate `unknown` tables (Docling
            // failed to structure them) from their images so the deterministic extractors can run.
            let (tables_repaired, repair_errors) = repair_degenerate_tables_via_vlm(
                &mut source_ir,
                provider,
                &model,
                &api_url,
                args.dry_run,
            );
            println!("tables_grid_repaired_by_vlm: {tables_repaired}");
            println!("table_repair_errors: {repair_errors}");
            // PDF-VARIANT-DIGESTION.2b — VLM table strategy: reclassify the remaining `unknown` tables from
            // their rendered images (best-wins; confident deterministic kinds are untouched).
            let (tables_reclassified, table_errors) = classify_unknown_tables_via_vlm(
                &mut source_ir,
                provider,
                &model,
                &api_url,
                args.dry_run,
            );
            println!("tables_reclassified_by_vlm: {tables_reclassified}");
            println!("table_vlm_errors: {table_errors}");

            if !args.dry_run {
                // Write updated SourceIR with enriched visual_assets + VLM table classifications.
                source_ir.write_to_disk()?;
                println!(
                    "enriched_source_ir_path: {}",
                    source_ir.artifact_layout.source_ir_path.display()
                );
                println!("next_stage: specforge evidence (re-run to pick up VLM observations)");
            }
        }
    }

    Ok(())
}

struct EnrichmentResult {
    observations: Vec<(String, DiagramKind, String)>,
    calls_made: usize,
    timing_enriched: usize,
    state_machine_enriched: usize,
    errors: usize,
}

fn enrich_visual_assets(
    assets: &[VisualAsset],
    provider: VlmProviderArg,
    model: &str,
    api_url: &str,
    dry_run: bool,
) -> Result<EnrichmentResult> {
    let mut observations = Vec::new();
    let mut calls_made = 0usize;
    let mut timing_enriched = 0usize;
    let mut state_machine_enriched = 0usize;
    let mut errors = 0usize;

    for asset in assets {
        match asset.diagram_kind {
            DiagramKind::TimingDiagram => {
                if asset_already_has_vlm_note(asset, "timing_diagram") {
                    continue;
                }
                if dry_run {
                    println!(
                        "  [dry-run] would enrich timing diagram: {} ({})",
                        asset.asset_id,
                        asset.caption_text.as_deref().unwrap_or("no caption")
                    );
                    continue;
                }
                calls_made += 1;
                match call_vlm_for_asset(asset, "timing_diagram", model, api_url, provider) {
                    Ok(observation_text) => {
                        observations.push((
                            asset.asset_id.clone(),
                            DiagramKind::TimingDiagram,
                            observation_text,
                        ));
                        timing_enriched += 1;
                    }
                    Err(e) => {
                        eprintln!(
                            "warning: VLM call failed for timing diagram {}: {e}",
                            asset.asset_id
                        );
                        errors += 1;
                    }
                }
            }
            DiagramKind::StateMachineDiagram => {
                if asset_already_has_vlm_note(asset, "state_machine") {
                    continue;
                }
                if dry_run {
                    println!(
                        "  [dry-run] would enrich state machine: {} ({})",
                        asset.asset_id,
                        asset.caption_text.as_deref().unwrap_or("no caption")
                    );
                    continue;
                }
                calls_made += 1;
                match call_vlm_for_asset(asset, "state_machine", model, api_url, provider) {
                    Ok(observation_text) => {
                        observations.push((
                            asset.asset_id.clone(),
                            DiagramKind::StateMachineDiagram,
                            observation_text,
                        ));
                        state_machine_enriched += 1;
                    }
                    Err(e) => {
                        eprintln!(
                            "warning: VLM call failed for state machine {}: {e}",
                            asset.asset_id
                        );
                        errors += 1;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(EnrichmentResult {
        observations,
        calls_made,
        timing_enriched,
        state_machine_enriched,
        errors,
    })
}

fn asset_already_has_vlm_note(asset: &VisualAsset, diagram_type: &str) -> bool {
    match (diagram_type, asset.note.as_deref()) {
        ("timing_diagram", Some(note)) => note.starts_with("vlm_timing_diagram_extraction:"),
        ("state_machine", Some(note)) => note.starts_with("vlm_state_machine_extraction:"),
        _ => false,
    }
}

/// Build the structured VLM prompt for a given diagram type.
fn build_vlm_prompt(diagram_type: &str, caption: &str) -> String {
    match diagram_type {
        "timing_diagram" => format!(
            "This is a timing diagram from a digital-hardware specification. \
             Caption: \"{caption}\"\n\n\
             Treat every visible label as opaque: copy it exactly and never infer meaning from a familiar name.\n\
             Please describe the diagram precisely:\n\
             1. List every signal shown (one per line) with its name.\n\
             2. For each signal, describe its state (HIGH/LOW/X/Z/VALID/UNKNOWN) only at clock positions tied to a visible clock grid/edge or a printed cycle label. The cycle field must be a zero-based integer or a printed T<n> label; never infer time from response-array position or an unclocked visual position.\n\
             3. List any timing annotations (setup time, hold time, cycle labels, arrow annotations).\n\
             Respond in structured JSON: \
             {{\"signals\": [{{\"name\": str, \"values\": [{{\"cycle\": str, \"state\": str}}]}}], \
             \"annotations\": [str]}}"
        ),
        "state_machine" => format!(
            "This is a state-machine diagram from a digital-hardware specification. \
             Caption: \"{caption}\"\n\n\
             Treat every visible label as opaque: copy it exactly and never infer meaning from a familiar name.\n\
             Please describe the diagram precisely:\n\
             1. List all states (one per line) with their names and whether they are the initial state.\n\
             2. For each transition arrow, give: source state, target state, guard condition label (if any).\n\
             Respond in structured JSON: \
             {{\"states\": [{{\"name\": str, \"is_initial\": bool}}], \
             \"transitions\": [{{\"from\": str, \"to\": str, \"guard\": str}}]}}"
        ),
        _ => format!(
            "Describe this digital-hardware specification diagram using only visible structure and labels. Treat labels as opaque. Caption: \"{caption}\""
        ),
    }
}

fn call_vlm_for_asset(
    asset: &VisualAsset,
    diagram_type: &str,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
) -> Result<String> {
    // Allow test override via SPECFORGE_VLM_HELPER env var.
    if let Some(helper_path) = std::env::var_os(VLM_HELPER_ENV) {
        let mut command = Command::new(&helper_path);
        crate::project_data::configure_command(&mut command)?;
        let output = command
            .arg("--asset-id")
            .arg(&asset.asset_id)
            .arg("--diagram-type")
            .arg(diagram_type)
            .output()?;
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
        return Err(AppError::ExternalCommandFailed {
            program: helper_path.display().to_string(),
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    let Some(image_path) = &asset.image_path else {
        return Err(AppError::InvalidStageArtifact(format!(
            "visual asset {} has no image_path for VLM enrichment",
            asset.asset_id
        )));
    };

    let caption = asset.caption_text.as_deref().unwrap_or("");
    let prompt = build_vlm_prompt(diagram_type, caption);
    vlm_image_query(image_path, &prompt, model, api_url, provider)
}

/// PDF-VARIANT-DIGESTION.2b — VLM table-classification prompt (asks for STRICT JSON `{"kind": …}`).
fn build_table_classify_prompt() -> String {
    "This is a table image from a digital-hardware specification. Classify its structural role from visible headers and layout only. Treat document-owned labels as opaque. Reply with STRICT JSON \
only, no prose: {\"kind\": <one of \"signal_description\", \"register_field\", \"register_map\", \
\"encoding\", \"timing_parameter\", \"feature_matrix\", \"table_of_contents\", \"other\">}."
        .to_string()
}

/// Parse the VLM's `{"kind": "..."}` reply into a [`TableKind`] to APPLY. Returns `None` for kinds we do
/// not reclassify on: `register_field` (the deterministic header-grammar path already recovers these from
/// `unknown`), and `table_of_contents`/`other` (correctly left unextracted). Tolerates ```json fences and
/// surrounding prose. PDF-VARIANT-DIGESTION.2b.
#[cfg(test)]
fn parse_vlm_table_kind(content: &str) -> Option<TableKind> {
    let lower = content.to_ascii_lowercase();
    let key = lower.find("\"kind\"")?;
    let after = &lower[key + 6..];
    let colon = after.find(':')?;
    let q1 = after[colon + 1..].find('"')?;
    let rest = &after[colon + 1 + q1 + 1..];
    let q2 = rest.find('"')?;
    map_vlm_kind(&rest[..q2])
}

/// Map a VLM `kind` label to a [`TableKind`] to APPLY. `None` for `register_field` (the deterministic
/// grammar path recovers these from `unknown`) and `table_of_contents`/`other` (left unextracted).
#[cfg(test)]
fn map_vlm_kind(s: &str) -> Option<TableKind> {
    match s.trim() {
        "signal_description" => Some(TableKind::SignalDescription),
        "encoding" => Some(TableKind::Encoding),
        "timing_parameter" => Some(TableKind::TimingParameter),
        "feature_matrix" => Some(TableKind::FeatureMatrix),
        "register_map" => Some(TableKind::RegisterMap),
        _ => None,
    }
}

/// PDF-VARIANT-DIGESTION.2b' — VLM table-EXTRACTION prompt: transcribe the table image to a JSON grid.
fn build_table_extract_prompt() -> String {
    "This is a table image from a digital-hardware specification. Transcribe visible structure and cell text without inferring from familiar names. Reply as STRICT JSON only, no prose: \
{\"kind\": <one of \"signal_description\",\"register_field\",\"register_map\",\"encoding\",\
\"timing_parameter\",\"feature_matrix\",\"table_of_contents\",\"other\">, \"columns\": [<column header \
strings>], \"rows\": [[<cell strings, one per column>], ...]}. Preserve cell text verbatim."
        .to_string()
}

/// Parse a VLM grid transcription `{"kind","columns":[..],"rows":[[..]]}` (tolerating ```json fences /
/// prose) into `(kind, columns, rows)`. Requires ≥2 columns. PDF-VARIANT-DIGESTION.2b'.
#[cfg(test)]
fn parse_vlm_grid(content: &str) -> Option<(String, Vec<String>, Vec<Vec<String>>)> {
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    if end < start {
        return None;
    }
    let v: serde_json::Value = serde_json::from_str(&content[start..=end]).ok()?;
    let kind = v
        .get("kind")
        .and_then(|k| k.as_str())
        .unwrap_or("other")
        .to_string();
    let cell_str = |c: &serde_json::Value| match c {
        serde_json::Value::String(s) => s.trim().to_string(),
        serde_json::Value::Null => String::new(),
        other => other.to_string(),
    };
    let columns: Vec<String> = v.get("columns")?.as_array()?.iter().map(cell_str).collect();
    if columns.len() < 2 {
        return None;
    }
    let rows: Vec<Vec<String>> = v
        .get("rows")
        .and_then(|r| r.as_array())
        .map(|rows| {
            rows.iter()
                .filter_map(|r| r.as_array())
                .map(|cells| cells.iter().map(cell_str).collect())
                .collect()
        })
        .unwrap_or_default();
    Some((kind, columns, rows))
}

/// A table Docling failed to structure: ≤1 column, or every body row has ≤1 cell.
fn table_is_degenerate(table: &StructuredTableRecord) -> bool {
    let max_cells = table.body_rows.iter().map(|r| r.len()).max().unwrap_or(0);
    table.col_count <= 1 || max_cells <= 1
}

/// PDF-VARIANT-DIGESTION.2b — VERIFY a VLM-proposed table kind against the table's actual STRUCTURE before
/// applying it (the VLM proposes, structure disposes). The VLM over-classifies — e.g. it labels register-
/// field tables (`Field|…|Access|Reset`) or operation/example tables (`Op|Address|Value`) as
/// `signal_description`, which then yields garbage "signals". A reclassification is applied only when the
/// table header is consistent with the proposed kind. Header GRAMMAR (ADR 0006); no chip names.
#[cfg(test)]
fn vlm_kind_structurally_consistent(table: &StructuredTableRecord, kind: TableKind) -> bool {
    let header: Vec<String> = table
        .header_rows
        .first()
        .or_else(|| table.body_rows.first())
        .map(|r| {
            r.iter()
                .map(|c| c.text.trim().to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default();
    let has = |kws: &[&str]| header.iter().any(|h| kws.iter().any(|k| h.contains(k)));
    match kind {
        // a signal table NAMES signals AND has a width/direction/source column — and is NOT a
        // register-field table (field + access/reset), which the register grammar path owns.
        TableKind::SignalDescription => {
            has(&["signal", "name", "pin", "port"])
                && has(&["width", "bits", "direction", "source", "destination"])
                && !(has(&["field"]) && has(&["access", "reset"]))
        }
        TableKind::RegisterMap => {
            has(&["offset", "address", "bits", "field"])
                && has(&["access", "reset", "type", "attribut", "bits"])
        }
        TableKind::Encoding => has(&["value", "encoding", "code", "binary", "hex"]),
        TableKind::TimingParameter => timing_table_has_structural_authority(table),
        TableKind::FeatureMatrix => has(&[
            "feature",
            "property",
            "capability",
            "mandatory",
            "optional",
            "support",
        ]),
        _ => true,
    }
}

/// PDF-VARIANT-DIGESTION.2b — VLM table strategy (best-wins-per-PDF). For each `unknown`-kind table that
/// has a rendered image, ask the VLM to classify its role and APPLY a recognized data kind, so the
/// downstream deterministic extractors fire on tables the deterministic CLASSIFIER missed. Confident
/// deterministic kinds are never overridden (only `unknown` tables are touched). Returns
/// `(reclassified, errors)`. Mutates `source_ir.structured_tables` in place.
fn classify_unknown_tables_via_vlm(
    source_ir: &mut SourceIr,
    provider: VlmProviderArg,
    model: &str,
    api_url: &str,
    dry_run: bool,
) -> (usize, usize) {
    let image_by_asset: std::collections::HashMap<&str, &std::path::Path> = source_ir
        .visual_assets
        .iter()
        .filter_map(|a| a.image_path.as_deref().map(|p| (a.asset_id.as_str(), p)))
        .collect();
    let prompt = build_table_classify_prompt();
    let mut reclassified = 0usize;
    let mut errors = 0usize;
    // Resolve images up front (immutable borrow) before mutating the tables.
    let plan: Vec<(String, std::path::PathBuf)> = source_ir
        .structured_tables
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            t.table_kind == TableKind::Unknown && !crate::ir::evidence::table_is_noise(t)
        })
        .filter_map(|(_, t)| {
            image_by_asset
                .get(t.asset_id.as_str())
                .map(|p| (t.table_id.clone(), p.to_path_buf()))
        })
        .collect();
    for (table_id, image_path) in plan {
        if dry_run {
            continue;
        }
        match vlm_image_query(&image_path, &prompt, model, api_url, provider) {
            Ok(content) => match source_ir.apply_table_classification(&table_id, content) {
                Ok(true) => reclassified += 1,
                Ok(false) => {}
                Err(_) => errors += 1,
            },
            Err(_) => errors += 1,
        }
    }
    (reclassified, errors)
}

/// PDF-VARIANT-DIGESTION.2b' — VLM table-EXTRACTION (grid repair). For each `unknown` table that Docling
/// failed to STRUCTURE (`table_is_degenerate`) and that has a rendered image, ask the VLM to transcribe the
/// grid from the image, then REPLACE the degenerate `header_rows`/`body_rows` with the VLM grid (≥2 columns)
/// so the downstream deterministic extractors can run. Best-wins at the STRUCTURE level (Docling grid vs VLM
/// grid). The kind is set only when structurally consistent (same gate as `.2b`). Returns `(repaired, errors)`.
fn repair_degenerate_tables_via_vlm(
    source_ir: &mut SourceIr,
    provider: VlmProviderArg,
    model: &str,
    api_url: &str,
    dry_run: bool,
) -> (usize, usize) {
    let image_by_asset: std::collections::HashMap<&str, &std::path::Path> = source_ir
        .visual_assets
        .iter()
        .filter_map(|a| a.image_path.as_deref().map(|p| (a.asset_id.as_str(), p)))
        .collect();
    let prompt = build_table_extract_prompt();
    let plan: Vec<(String, std::path::PathBuf)> = source_ir
        .structured_tables
        .iter()
        .enumerate()
        .filter(|(_, t)| {
            t.table_kind == TableKind::Unknown
                && table_is_degenerate(t)
                && !crate::ir::evidence::table_is_noise(t)
        })
        .filter_map(|(_, t)| {
            image_by_asset
                .get(t.asset_id.as_str())
                .map(|p| (t.table_id.clone(), p.to_path_buf()))
        })
        .collect();
    let mut repaired = 0usize;
    let mut errors = 0usize;
    for (table_id, image_path) in plan {
        if dry_run {
            continue;
        }
        match vlm_image_query(&image_path, &prompt, model, api_url, provider) {
            Ok(content) => match source_ir.apply_table_grid_repair(&table_id, content) {
                Ok(true) => repaired += 1,
                Ok(false) => {}
                Err(_) => errors += 1,
            },
            Err(_) => errors += 1,
        }
    }
    (repaired, errors)
}

/// POST a single image + text prompt to an OpenAI-compatible VLM endpoint (Ollama / OpenAI / LM Studio)
/// and return the assistant's message content. Shared by diagram enrichment, table classification
/// (PDF-VARIANT-DIGESTION.2b), and the extraction audit (PDF-VARIANT-DIGESTION.4b.1).
pub(crate) fn vlm_image_query(
    image_path: &std::path::Path,
    prompt: &str,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
) -> Result<String> {
    let image_bytes = fs::read(image_path).map_err(AppError::Io)?;
    let image_b64 = base64_encode(&image_bytes);
    let request_body = build_chat_request(model, prompt, &image_b64);

    let mut cmd = Command::new("curl");
    crate::project_data::configure_command(&mut cmd)?;
    cmd.arg("-s")
        .arg("-X")
        .arg("POST")
        .arg(api_url)
        .arg("-H")
        .arg("Content-Type: application/json");
    if matches!(provider, VlmProviderArg::OpenAi) {
        let api_key =
            std::env::var("OPENAI_API_KEY").map_err(|_| AppError::MissingRuntimeDependency {
                dependency: "OPENAI_API_KEY",
                resolution: "Set OPENAI_API_KEY environment variable to your OpenAI API key"
                    .to_string(),
            })?;
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {api_key}"));
    }
    let tempdir = crate::project_data::tempdir()?;
    let request_path = tempdir.path().join("vlm_request.json");
    fs::write(&request_path, &request_body)?;
    cmd.arg("-d").arg(format!("@{}", request_path.display()));

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppError::ExternalCommandFailed {
            program: "curl".to_string(),
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    extract_vlm_content(&String::from_utf8_lossy(&output.stdout))
}

fn build_chat_request(model: &str, prompt: &str, image_b64: &str) -> String {
    // Escape JSON special chars in the prompt.
    let prompt_escaped = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");

    format!(
        r#"{{"model": "{model}", "messages": [{{"role": "user", "content": [{{"type": "text", "text": "{prompt_escaped}"}}, {{"type": "image_url", "image_url": {{"url": "data:image/png;base64,{image_b64}"}}}}]}}], "max_tokens": 2048}}"#
    )
}

/// Extract the assistant message content from an OpenAI-compatible chat response.
///
/// Parses the standard structure:
/// `{"choices": [{"message": {"content": "..."}}]}`
///
/// `content` may be a plain string or an array of content parts (some VLM providers).
fn extract_vlm_content(response_json: &str) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct ChatResponse {
        choices: Vec<ChatChoice>,
    }
    #[derive(serde::Deserialize)]
    struct ChatChoice {
        message: ChatMessage,
    }
    #[derive(serde::Deserialize)]
    struct ChatMessage {
        content: serde_json::Value,
    }

    let response: ChatResponse = serde_json::from_str(response_json).map_err(|e| {
        AppError::InvalidStageArtifact(format!(
            "invalid VLM response (not OpenAI-compatible JSON): {e}\nraw: {}",
            &response_json[..response_json.len().min(256)]
        ))
    })?;

    let content = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| {
            AppError::InvalidStageArtifact("VLM response has no choices (empty array)".to_string())
        })?
        .message
        .content;

    // Content can be a plain string or an array of {type, text} parts.
    match content {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Array(parts) => {
            for part in parts {
                if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                    return Ok(text.to_string());
                }
            }
            Err(AppError::InvalidStageArtifact(
                "VLM response content array has no text part".to_string(),
            ))
        }
        other => Ok(other.to_string()),
    }
}

/// Simple base64 encoder without external dependencies.
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() * 4).div_ceil(3));
    let mut i = 0;
    while i < data.len() {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < data.len() {
            data[i + 1] as u32
        } else {
            0
        };
        let b2 = if i + 2 < data.len() {
            data[i + 2] as u32
        } else {
            0
        };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 63) as usize] as char);
        result.push(CHARS[((triple >> 12) & 63) as usize] as char);
        if i + 1 < data.len() {
            result.push(CHARS[((triple >> 6) & 63) as usize] as char);
        } else {
            result.push('=');
        }
        if i + 2 < data.len() {
            result.push(CHARS[(triple & 63) as usize] as char);
        } else {
            result.push('=');
        }
        i += 3;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vlm_prompts_are_identity_neutral_and_visually_grounded() {
        let first =
            build_vlm_prompt("timing_diagram", "orchid timing").replace("orchid", "<identity>");
        let renamed =
            build_vlm_prompt("timing_diagram", "juniper timing").replace("juniper", "<identity>");
        assert_eq!(first, renamed);
        assert!(first.contains("Treat every visible label as opaque"));
        assert!(!first.contains("protocol specification"));

        for prompt in [build_table_classify_prompt(), build_table_extract_prompt()] {
            assert!(prompt.contains("digital-hardware specification"));
            assert!(prompt.contains("opaque") || prompt.contains("without inferring"));
            assert!(!prompt.contains("chip-specification"));
        }
    }

    #[test]
    fn vlm_table_kind_parsing_maps_recognized_and_filters_the_rest() {
        // recognized data kinds map (fenced json + surrounding prose tolerated)
        assert_eq!(
            parse_vlm_table_kind("```json\n{\"kind\": \"signal_description\"}\n```"),
            Some(TableKind::SignalDescription)
        );
        assert_eq!(
            parse_vlm_table_kind(r#"{"kind":"register_map"}"#),
            Some(TableKind::RegisterMap)
        );
        assert_eq!(
            parse_vlm_table_kind(r#"the answer is {"kind": "encoding"}."#),
            Some(TableKind::Encoding)
        );
        assert_eq!(
            parse_vlm_table_kind(r#"{"kind": "timing_parameter"}"#),
            Some(TableKind::TimingParameter)
        );
        assert_eq!(
            parse_vlm_table_kind(r#"{"kind": "feature_matrix"}"#),
            Some(TableKind::FeatureMatrix)
        );
        // register_field is recovered by the deterministic grammar path → not reclassified here
        assert_eq!(parse_vlm_table_kind(r#"{"kind": "register_field"}"#), None);
        // noise kinds are left unextracted
        assert_eq!(
            parse_vlm_table_kind(r#"{"kind": "table_of_contents"}"#),
            None
        );
        assert_eq!(parse_vlm_table_kind(r#"{"kind": "other"}"#), None);
        assert_eq!(parse_vlm_table_kind("no json at all"), None);
    }

    #[test]
    fn vlm_kind_verification_rejects_overclassification() {
        use crate::ir::source::StructuredTableCellRecord;
        let cell = |t: &str| StructuredTableCellRecord {
            text: t.to_string(),
            row_span: 1,
            col_span: 1,
            is_header: true,
        };
        let table = |hdr: Vec<&str>| StructuredTableRecord {
            table_id: "t".to_string(),
            asset_id: "a".to_string(),
            page_id: None,
            caption_text: None,
            source_ref: None,
            source_batch: None,
            table_kind: TableKind::Unknown,
            header_rows: vec![hdr.iter().map(|h| cell(h)).collect()],
            body_rows: vec![],
            row_count: 1,
            col_count: hdr.len() as u32,
        };
        // a genuine signal table is accepted
        assert!(vlm_kind_structurally_consistent(
            &table(vec!["Signal", "Width", "Source", "Description"]),
            TableKind::SignalDescription
        ));
        // a register-field table mislabeled signal_description is REJECTED (no garbage signals)
        assert!(!vlm_kind_structurally_consistent(
            &table(vec!["Field", "Description", "Access", "Reset"]),
            TableKind::SignalDescription
        ));
        // an operation/example table mislabeled signal_description is REJECTED
        assert!(!vlm_kind_structurally_consistent(
            &table(vec!["Op", "Address", "Value", "Comment"]),
            TableKind::SignalDescription
        ));
        // a real register-map table is accepted
        assert!(vlm_kind_structurally_consistent(
            &table(vec!["Offset", "Bits", "Field name", "Attributes"]),
            TableKind::RegisterMap
        ));
    }

    #[test]
    fn vlm_grid_parsing_extracts_columns_and_rows() {
        let content = "```json\n{\"kind\":\"register_field\",\"columns\":[\"Field\",\"Bits\",\
\"Access\"],\"rows\":[[\"EN\",\"0\",\"RW\"],[\"MODE\",\"2:1\",\"RW\"]]}\n```";
        let (kind, cols, rows) = parse_vlm_grid(content).unwrap();
        assert_eq!(kind, "register_field");
        assert_eq!(cols, vec!["Field", "Bits", "Access"]);
        assert_eq!(rows, vec![vec!["EN", "0", "RW"], vec!["MODE", "2:1", "RW"]]);
        // a single-column transcription is rejected (not a real grid)
        assert!(parse_vlm_grid(r#"{"kind":"other","columns":["X"],"rows":[]}"#).is_none());
        assert!(parse_vlm_grid("no json here").is_none());
    }
}
