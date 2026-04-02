use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::cli::{EnrichArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::source::{DiagramKind, SourceIr, VisualAsset};

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
                VlmProviderArg::Ollama => "llava:13b".to_string(),
                VlmProviderArg::OpenAi => "gpt-4o".to_string(),
                VlmProviderArg::LmStudio => "loaded_model".to_string(),
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

            if !args.dry_run {
                // Write updated SourceIR with enriched visual_assets.
                source_ir.visual_assets = enriched.updated_assets;
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
    updated_assets: Vec<VisualAsset>,
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
    let mut updated_assets = assets.to_vec();
    let mut calls_made = 0usize;
    let mut timing_enriched = 0usize;
    let mut state_machine_enriched = 0usize;
    let mut errors = 0usize;

    for asset in &mut updated_assets {
        match asset.diagram_kind {
            DiagramKind::TimingDiagram => {
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
                        asset.note = Some(format!(
                            "vlm_timing_diagram_extraction: {}",
                            &observation_text[..observation_text.len().min(120)]
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
                        asset.note = Some(format!(
                            "vlm_state_machine_extraction: {}",
                            &observation_text[..observation_text.len().min(120)]
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
        updated_assets,
        calls_made,
        timing_enriched,
        state_machine_enriched,
        errors,
    })
}

/// Build the structured VLM prompt for a given diagram type.
fn build_vlm_prompt(diagram_type: &str, caption: &str) -> String {
    match diagram_type {
        "timing_diagram" => format!(
            "This is a timing diagram from a chip protocol specification. \
             Caption: \"{caption}\"\n\n\
             Please describe the diagram precisely:\n\
             1. List every signal shown (one per line) with its name.\n\
             2. For each signal, describe its state (HIGH/LOW/X/Z/VALID/UNKNOWN) at each labeled clock cycle.\n\
             3. List any timing annotations (setup time, hold time, cycle labels, arrow annotations).\n\
             Respond in structured JSON: \
             {{\"signals\": [{{\"name\": str, \"values\": [{{\"cycle\": str, \"state\": str}}]}}], \
             \"annotations\": [str]}}"
        ),
        "state_machine" => format!(
            "This is a state machine diagram from a chip protocol specification. \
             Caption: \"{caption}\"\n\n\
             Please describe the diagram precisely:\n\
             1. List all states (one per line) with their names and whether they are the initial state.\n\
             2. For each transition arrow, give: source state, target state, guard condition label (if any).\n\
             Respond in structured JSON: \
             {{\"states\": [{{\"name\": str, \"is_initial\": bool}}], \
             \"transitions\": [{{\"from\": str, \"to\": str, \"guard\": str}}]}}"
        ),
        _ => format!("Describe this chip specification diagram. Caption: \"{caption}\""),
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
        let output = Command::new(&helper_path)
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

    // Encode image as base64.
    let image_bytes = fs::read(image_path).map_err(|e| AppError::Io(e))?;
    let image_b64 = base64_encode(&image_bytes);

    // Build the OpenAI-compatible chat completions request.
    // All supported providers (Ollama, OpenAI, LM Studio) use this format.
    let request_body = build_chat_request(model, &prompt, &image_b64);

    // Determine extra headers (OpenAI requires Authorization).
    let mut cmd = Command::new("curl");
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

    // Write request body to a temp file to avoid shell quoting issues.
    let tempdir = tempfile::tempdir()?;
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

    let response_text = String::from_utf8_lossy(&output.stdout).to_string();
    extract_vlm_content(&response_text)
}

fn build_chat_request(model: &str, prompt: &str, image_b64: &str) -> String {
    // Escape JSON special chars in the prompt.
    let prompt_escaped = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");

    format!(
        r#"{{"model": "{model}", "messages": [{{"role": "user", "content": [{{"type": "text", "text": "{prompt_escaped}"}}, {{"type": "image_url", "image_url": {{"url": "data:image/png;base64,{image_b64}"}}}}]}}], "max_tokens": 1024}}"#
    )
}

/// Extract the assistant message content from an OpenAI-compatible chat response.
fn extract_vlm_content(response_json: &str) -> Result<String> {
    // Parse just enough to find choices[0].message.content.
    if let Some(content_start) = response_json.find("\"content\":") {
        let after = &response_json[content_start + 10..].trim_start();
        if after.starts_with('"') {
            // Simple string content.
            let end = after[1..].find('"').unwrap_or(after.len() - 1);
            return Ok(after[1..end + 1].to_string());
        }
    }
    // Fall back to returning the raw response.
    Ok(response_json.chars().take(512).collect())
}

/// Simple base64 encoder without external dependencies.
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() * 4 + 2) / 3);
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
