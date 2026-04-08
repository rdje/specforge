use std::fs;
use std::process::Command;

use serde::Deserialize;
use tempfile::tempdir;

use crate::cli::DoctorArgs;
use crate::error::{AppError, Result};
use crate::ir::source::{
    DEFAULT_DOCLING_BOOTSTRAP_SCRIPT, DEFAULT_DOCLING_VENV_DIR, DOCLING_PYTHON_ENV,
    inspect_docling_runtime,
};

const OLLAMA_TAGS_URL: &str = "http://localhost:11434/api/tags";
const OLLAMA_CHAT_URL: &str = "http://localhost:11434/v1/chat/completions";
const OLLAMA_DEFAULT_MODEL: &str = "qwen2.5vl:7b";

pub fn run(args: DoctorArgs) -> Result<()> {
    let docling = inspect_docling_runtime()?;
    let ollama = inspect_ollama_runtime()?;

    println!("command: doctor");
    println!("docling_runtime_ready: {}", yes_no(docling.is_ready()));
    println!("repo_local_docling_venv: {DEFAULT_DOCLING_VENV_DIR}");
    println!("bootstrap_script: {DEFAULT_DOCLING_BOOTSTRAP_SCRIPT}");
    println!("env_override: {DOCLING_PYTHON_ENV}");

    match docling.selected_python.as_ref() {
        Some(path) => println!("selected_python: {}", path.display()),
        None => println!("selected_python: none"),
    }

    println!(
        "selected_source: {}",
        docling
            .selected_source
            .map(|source| source.as_str())
            .unwrap_or("none")
    );
    println!(
        "selected_label: {}",
        docling.selected_label.as_deref().unwrap_or("none")
    );
    println!(
        "selected_python_version: {}",
        docling
            .selected_python_version
            .as_deref()
            .unwrap_or("unknown")
    );
    println!(
        "selected_docling_version: {}",
        docling
            .selected_docling_version
            .as_deref()
            .unwrap_or("unknown")
    );
    println!("candidates: {}", docling.candidates.len());

    for (index, candidate) in docling.candidates.iter().enumerate() {
        println!("candidate[{index}].label: {}", candidate.label);
        println!("candidate[{index}].path: {}", candidate.path.display());
        println!("candidate[{index}].source: {}", candidate.source.as_str());
        println!("candidate[{index}].status: {}", candidate.status.as_str());
        println!(
            "candidate[{index}].selected: {}",
            yes_no(candidate.selected)
        );
        println!(
            "candidate[{index}].python_version: {}",
            candidate.python_version.as_deref().unwrap_or("unknown")
        );
        println!(
            "candidate[{index}].docling_version: {}",
            candidate.docling_version.as_deref().unwrap_or("unknown")
        );
        if let Some(detail) = candidate.detail.as_deref() {
            println!("candidate[{index}].detail: {detail}");
        }
    }

    println!(
        "ollama_default_loopback_ready: {}",
        yes_no(ollama.is_ready())
    );
    println!("ollama_tags_url: {OLLAMA_TAGS_URL}");
    println!("ollama_chat_url: {OLLAMA_CHAT_URL}");
    println!("ollama_default_model: {OLLAMA_DEFAULT_MODEL}");
    println!("ollama_tags_reachable: {}", yes_no(ollama.tags_reachable));
    println!("ollama_chat_reachable: {}", yes_no(ollama.chat_reachable));
    println!(
        "ollama_default_model_present: {}",
        yes_no(ollama.default_model_present)
    );
    println!("ollama_visible_models: {}", ollama.visible_models);
    if let Some(detail) = ollama.tags_detail.as_deref() {
        println!("ollama_tags_detail: {detail}");
    }
    if let Some(detail) = ollama.chat_detail.as_deref() {
        println!("ollama_chat_detail: {detail}");
    }

    let mut missing = Vec::new();
    if !docling.is_ready() {
        let resolution = docling.resolution();
        println!("docling_resolution: {resolution}");
        missing.push(format!("docling: {resolution}"));
    }
    if !ollama.is_ready() {
        let resolution = ollama.resolution();
        println!("ollama_resolution: {resolution}");
        missing.push(format!("ollama_default_loopback: {resolution}"));
    }

    if args.strict && !missing.is_empty() {
        return Err(AppError::MissingRuntimeDependency {
            dependency: "runtime readiness",
            resolution: missing.join(" | "),
        });
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OllamaRuntimeDiagnosis {
    tags_reachable: bool,
    chat_reachable: bool,
    default_model_present: bool,
    visible_models: usize,
    tags_detail: Option<String>,
    chat_detail: Option<String>,
}

impl OllamaRuntimeDiagnosis {
    fn is_ready(&self) -> bool {
        self.tags_reachable && self.chat_reachable && self.default_model_present
    }

    fn resolution(&self) -> String {
        if !self.tags_reachable {
            return format!(
                "the Ollama tags endpoint at `{OLLAMA_TAGS_URL}` is not reachable; start `ollama serve` and make sure local loopback access to port 11434 is available"
            );
        }

        if !self.default_model_present {
            return format!(
                "the default Ollama model `{OLLAMA_DEFAULT_MODEL}` is not visible at `{OLLAMA_TAGS_URL}`; run `ollama pull {OLLAMA_DEFAULT_MODEL}` or override the converge/enrich/nlp model explicitly"
            );
        }

        if !self.chat_reachable {
            return format!(
                "the Ollama chat-completions endpoint at `{OLLAMA_CHAT_URL}` is not usable for `{OLLAMA_DEFAULT_MODEL}`: {}; make sure `ollama serve` exposes the OpenAI-compatible endpoint and local loopback access is permitted",
                self.chat_detail
                    .as_deref()
                    .unwrap_or("no additional detail captured")
            );
        }

        "ollama default loopback is ready".to_string()
    }
}

fn inspect_ollama_runtime() -> Result<OllamaRuntimeDiagnosis> {
    let tags_response = run_ollama_tags_probe()?;
    let (tags_reachable, default_model_present, visible_models, tags_detail) =
        match parse_ollama_tags_response(&tags_response) {
            Ok(summary) => (
                true,
                summary.default_model_present,
                summary.visible_models,
                None,
            ),
            Err(detail) => (false, false, 0usize, Some(detail)),
        };

    let (chat_reachable, chat_detail) = if tags_reachable && default_model_present {
        match run_ollama_chat_probe()? {
            Ok(response) => (
                true,
                Some(format!(
                    "chat probe succeeded for `{OLLAMA_DEFAULT_MODEL}`: {}",
                    truncate_for_display(&response, 80)
                )),
            ),
            Err(detail) => (false, Some(detail)),
        }
    } else if !tags_reachable {
        (
            false,
            Some("chat probe skipped because /api/tags is unavailable".to_string()),
        )
    } else {
        (
            false,
            Some(format!(
                "chat probe skipped because `{OLLAMA_DEFAULT_MODEL}` is not visible in /api/tags"
            )),
        )
    };

    Ok(OllamaRuntimeDiagnosis {
        tags_reachable,
        chat_reachable,
        default_model_present,
        visible_models,
        tags_detail,
        chat_detail,
    })
}

fn run_ollama_tags_probe() -> Result<String> {
    let output = Command::new("curl")
        .arg("-s")
        .arg(OLLAMA_TAGS_URL)
        .output()?;
    if !output.status.success() {
        return Ok(String::new());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn run_ollama_chat_probe() -> Result<std::result::Result<String, String>> {
    let request_body = format!(
        r#"{{"model":"{OLLAMA_DEFAULT_MODEL}","messages":[{{"role":"user","content":"Reply with the single token ok."}}],"max_tokens":8}}"#
    );
    let tempdir = tempdir()?;
    let request_path = tempdir.path().join("ollama_chat_probe.json");
    let response_path = tempdir.path().join("ollama_chat_probe.out");
    fs::write(&request_path, request_body)?;

    let output = Command::new("curl")
        .arg("-s")
        .arg("-o")
        .arg(&response_path)
        .arg("-w")
        .arg("%{http_code}")
        .arg("-X")
        .arg("POST")
        .arg(OLLAMA_CHAT_URL)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(format!("@{}", request_path.display()))
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Ok(Err(if stderr.is_empty() {
            format!("curl exited with {:?}", output.status.code())
        } else {
            stderr
        }));
    }

    let http_code = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let response_body = fs::read_to_string(&response_path).unwrap_or_default();
    if http_code != "200" {
        return Ok(Err(format!(
            "HTTP {http_code}: {}",
            truncate_for_display(&response_body, 120)
        )));
    }

    match parse_ollama_chat_response(&response_body) {
        Ok(summary) => Ok(Ok(summary.assistant_content)),
        Err(detail) => Ok(Err(detail)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OllamaTagsSummary {
    default_model_present: bool,
    visible_models: usize,
}

fn parse_ollama_tags_response(response: &str) -> std::result::Result<OllamaTagsSummary, String> {
    #[derive(Deserialize)]
    struct TagsResponse {
        #[serde(default)]
        models: Vec<TagsModel>,
    }

    #[derive(Deserialize)]
    struct TagsModel {
        name: String,
    }

    if response.trim().is_empty() {
        return Err("empty response from /api/tags".to_string());
    }

    let parsed = serde_json::from_str::<TagsResponse>(response)
        .map_err(|error| format!("invalid /api/tags response: {error}"))?;
    let default_model_present = parsed
        .models
        .iter()
        .any(|model| model.name == OLLAMA_DEFAULT_MODEL);

    Ok(OllamaTagsSummary {
        default_model_present,
        visible_models: parsed.models.len(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OllamaChatSummary {
    assistant_content: String,
}

fn parse_ollama_chat_response(response: &str) -> std::result::Result<OllamaChatSummary, String> {
    #[derive(Deserialize)]
    struct ChatResponse {
        choices: Vec<ChatChoice>,
    }

    #[derive(Deserialize)]
    struct ChatChoice {
        message: ChatMessage,
    }

    #[derive(Deserialize)]
    struct ChatMessage {
        content: serde_json::Value,
    }

    if response.trim().is_empty() {
        return Err("empty response from /v1/chat/completions".to_string());
    }

    let parsed = serde_json::from_str::<ChatResponse>(response)
        .map_err(|error| format!("invalid /v1/chat/completions response: {error}"))?;
    let choice = parsed
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| "no choices returned from /v1/chat/completions".to_string())?;

    let assistant_content = match choice.message.content {
        serde_json::Value::String(text) => text,
        serde_json::Value::Array(parts) => parts
            .into_iter()
            .filter_map(|part| {
                part.get("text")
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
            })
            .collect::<Vec<_>>()
            .join("\n"),
        other => {
            return Err(format!(
                "unexpected chat response content shape: {}",
                truncate_for_display(&other.to_string(), 120)
            ));
        }
    };

    Ok(OllamaChatSummary { assistant_content })
}

fn truncate_for_display(text: &str, max_chars: usize) -> String {
    let mut truncated = String::new();
    for (index, ch) in text.chars().enumerate() {
        if index >= max_chars {
            truncated.push('…');
            return truncated;
        }
        truncated.push(ch);
    }
    truncated
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::{
        OLLAMA_DEFAULT_MODEL, parse_ollama_chat_response, parse_ollama_tags_response,
        truncate_for_display,
    };

    #[test]
    fn parse_ollama_tags_response_detects_default_model() {
        let response = format!(
            r#"{{"models":[{{"name":"{OLLAMA_DEFAULT_MODEL}"}},{{"name":"other:model"}}]}}"#
        );

        let summary = parse_ollama_tags_response(&response).expect("valid tags response");

        assert!(summary.default_model_present);
        assert_eq!(summary.visible_models, 2);
    }

    #[test]
    fn parse_ollama_tags_response_reports_missing_default_model() {
        let response = r#"{"models":[{"name":"other:model"}]}"#;

        let summary = parse_ollama_tags_response(response).expect("valid tags response");

        assert!(!summary.default_model_present);
        assert_eq!(summary.visible_models, 1);
    }

    #[test]
    fn parse_ollama_chat_response_accepts_openai_compatible_string_content() {
        let response = r#"{"choices":[{"message":{"content":"ok"}}]}"#;

        let summary = parse_ollama_chat_response(response).expect("valid chat response");

        assert_eq!(summary.assistant_content, "ok");
    }

    #[test]
    fn parse_ollama_chat_response_accepts_array_content() {
        let response = r#"{"choices":[{"message":{"content":[{"type":"text","text":"ok"}]}}]}"#;

        let summary = parse_ollama_chat_response(response).expect("valid array content");

        assert_eq!(summary.assistant_content, "ok");
    }

    #[test]
    fn truncate_for_display_appends_ellipsis_when_needed() {
        assert_eq!(truncate_for_display("abcdef", 3), "abc…");
        assert_eq!(truncate_for_display("abc", 3), "abc");
    }
}
