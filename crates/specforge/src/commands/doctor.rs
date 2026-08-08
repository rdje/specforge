use std::fs;
use std::process::Command;

use serde::Deserialize;

use crate::cli::{DoctorArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::source::{
    DEFAULT_DOCLING_BOOTSTRAP_SCRIPT, DEFAULT_DOCLING_VENV_DIR, DOCLING_PYTHON_ENV,
    inspect_docling_runtime,
};

pub(crate) const DEFAULT_LOCAL_MODEL: &str = "qwen2.5vl:7b";
const OLLAMA_TAGS_URL: &str = "http://localhost:11434/api/tags";
const OLLAMA_CHAT_URL: &str = "http://localhost:11434/v1/chat/completions";
const LMSTUDIO_MODELS_URL: &str = "http://localhost:1234/v1/models";
const LMSTUDIO_CHAT_URL: &str = "http://localhost:1234/v1/chat/completions";
const LOCAL_GET_PROBE_TIMEOUT_SECONDS: u64 = 2;
const LOCAL_CHAT_PROBE_TIMEOUT_SECONDS: u64 = 30;

pub fn run(args: DoctorArgs) -> Result<()> {
    let docling = inspect_docling_runtime()?;
    let ollama = inspect_ollama_runtime()?;
    let lmstudio = inspect_lmstudio_runtime()?;

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
    println!("ollama_default_model: {DEFAULT_LOCAL_MODEL}");
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

    println!(
        "lmstudio_fallback_loopback_ready: {}",
        yes_no(lmstudio.is_ready())
    );
    println!("lmstudio_models_url: {LMSTUDIO_MODELS_URL}");
    println!("lmstudio_chat_url: {LMSTUDIO_CHAT_URL}");
    println!("lmstudio_default_model: {DEFAULT_LOCAL_MODEL}");
    println!(
        "lmstudio_models_reachable: {}",
        yes_no(lmstudio.models_reachable)
    );
    println!(
        "lmstudio_chat_reachable: {}",
        yes_no(lmstudio.chat_reachable)
    );
    println!(
        "lmstudio_default_model_present: {}",
        yes_no(lmstudio.default_model_present)
    );
    println!("lmstudio_visible_models: {}", lmstudio.visible_models);
    if let Some(detail) = lmstudio.models_detail.as_deref() {
        println!("lmstudio_models_detail: {detail}");
    }
    if let Some(detail) = lmstudio.chat_detail.as_deref() {
        println!("lmstudio_chat_detail: {detail}");
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
    if !lmstudio.is_ready() {
        let resolution = lmstudio.resolution();
        println!("lmstudio_resolution: {resolution}");
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
                "the default Ollama model `{DEFAULT_LOCAL_MODEL}` is not visible at `{OLLAMA_TAGS_URL}`; run `ollama pull {DEFAULT_LOCAL_MODEL}` or override the converge/enrich/nlp model explicitly"
            );
        }

        if !self.chat_reachable {
            return format!(
                "the Ollama chat-completions endpoint at `{OLLAMA_CHAT_URL}` is not usable for `{DEFAULT_LOCAL_MODEL}`: {}; make sure `ollama serve` exposes the OpenAI-compatible endpoint and local loopback access is permitted",
                self.chat_detail
                    .as_deref()
                    .unwrap_or("no additional detail captured")
            );
        }

        "ollama default loopback is ready".to_string()
    }
}

fn inspect_ollama_runtime() -> Result<OllamaRuntimeDiagnosis> {
    let (tags_reachable, default_model_present, visible_models, tags_detail) =
        match run_ollama_tags_probe()? {
            Ok(tags_response) => match parse_ollama_tags_response(&tags_response) {
                Ok(summary) => (
                    true,
                    summary.default_model_present,
                    summary.visible_models,
                    None,
                ),
                Err(detail) => (false, false, 0usize, Some(detail)),
            },
            Err(detail) => (false, false, 0usize, Some(detail)),
        };

    let (chat_reachable, chat_detail) = if tags_reachable && default_model_present {
        match run_ollama_chat_probe()? {
            Ok(response) => (
                true,
                Some(format!(
                    "chat probe succeeded for `{DEFAULT_LOCAL_MODEL}`: {}",
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
                "chat probe skipped because `{DEFAULT_LOCAL_MODEL}` is not visible in /api/tags"
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct LmStudioRuntimeDiagnosis {
    models_reachable: bool,
    chat_reachable: bool,
    default_model_present: bool,
    visible_models: usize,
    models_detail: Option<String>,
    chat_detail: Option<String>,
}

impl LmStudioRuntimeDiagnosis {
    fn is_ready(&self) -> bool {
        self.models_reachable && self.chat_reachable && self.default_model_present
    }

    fn resolution(&self) -> String {
        if !self.models_reachable {
            return format!(
                "the LM Studio models endpoint at `{LMSTUDIO_MODELS_URL}` is not reachable; start the LM Studio local server and make sure local loopback access to port 1234 is available"
            );
        }

        if !self.default_model_present {
            return format!(
                "the default LM Studio model `{DEFAULT_LOCAL_MODEL}` is not visible at `{LMSTUDIO_MODELS_URL}`; load `{DEFAULT_LOCAL_MODEL}` in LM Studio or override the converge/enrich/nlp model explicitly when using `--vlm-provider lmstudio` / `--nlp-provider lmstudio`"
            );
        }

        if !self.chat_reachable {
            return format!(
                "the LM Studio chat-completions endpoint at `{LMSTUDIO_CHAT_URL}` is not usable for `{DEFAULT_LOCAL_MODEL}`: {}; make sure the LM Studio local server exposes the OpenAI-compatible endpoint and local loopback access is permitted",
                self.chat_detail
                    .as_deref()
                    .unwrap_or("no additional detail captured")
            );
        }

        "lmstudio fallback loopback is ready".to_string()
    }
}

fn inspect_lmstudio_runtime() -> Result<LmStudioRuntimeDiagnosis> {
    let (models_reachable, default_model_present, visible_models, models_detail) =
        match run_get_probe(LMSTUDIO_MODELS_URL)? {
            Ok(models_response) => {
                match parse_openai_models_response(&models_response, DEFAULT_LOCAL_MODEL) {
                    Ok(summary) => (
                        true,
                        summary.default_model_present,
                        summary.visible_models,
                        None,
                    ),
                    Err(detail) => (false, false, 0usize, Some(detail)),
                }
            }
            Err(detail) => (false, false, 0usize, Some(detail)),
        };

    let (chat_reachable, chat_detail) = if models_reachable && default_model_present {
        match run_openai_chat_probe(
            LMSTUDIO_CHAT_URL,
            DEFAULT_LOCAL_MODEL,
            "lmstudio_chat_probe",
        )? {
            Ok(response) => (
                true,
                Some(format!(
                    "chat probe succeeded for `{DEFAULT_LOCAL_MODEL}`: {}",
                    truncate_for_display(&response, 80)
                )),
            ),
            Err(detail) => (false, Some(detail)),
        }
    } else if !models_reachable {
        (
            false,
            Some("chat probe skipped because /v1/models is unavailable".to_string()),
        )
    } else {
        (
            false,
            Some(format!(
                "chat probe skipped because `{DEFAULT_LOCAL_MODEL}` is not visible in /v1/models"
            )),
        )
    };

    Ok(LmStudioRuntimeDiagnosis {
        models_reachable,
        chat_reachable,
        default_model_present,
        visible_models,
        models_detail,
        chat_detail,
    })
}

pub(crate) fn local_vlm_default_model_present(provider: VlmProviderArg) -> bool {
    match provider {
        VlmProviderArg::Ollama => run_ollama_tags_probe()
            .ok()
            .and_then(|response| response.ok())
            .and_then(|response| parse_ollama_tags_response(&response).ok())
            .is_some_and(|summary| summary.default_model_present),
        VlmProviderArg::LmStudio => run_get_probe(LMSTUDIO_MODELS_URL)
            .ok()
            .and_then(|response| response.ok())
            .and_then(|response| parse_openai_models_response(&response, DEFAULT_LOCAL_MODEL).ok())
            .is_some_and(|summary| summary.default_model_present),
        VlmProviderArg::OpenAi | VlmProviderArg::Skip => false,
    }
}

fn run_ollama_tags_probe() -> Result<std::result::Result<String, String>> {
    run_get_probe(OLLAMA_TAGS_URL)
}

fn run_get_probe(url: &str) -> Result<std::result::Result<String, String>> {
    let mut command = Command::new("curl");
    crate::project_data::configure_command(&mut command)?;
    let output = command
        .arg("-s")
        .arg("--max-time")
        .arg(LOCAL_GET_PROBE_TIMEOUT_SECONDS.to_string())
        .arg(url)
        .output()?;
    if !output.status.success() {
        return Ok(Err(format_curl_probe_failure(
            &output.stderr,
            output.status.code(),
            LOCAL_GET_PROBE_TIMEOUT_SECONDS,
            url,
        )));
    }
    Ok(Ok(String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string()))
}

fn run_ollama_chat_probe() -> Result<std::result::Result<String, String>> {
    run_openai_chat_probe(OLLAMA_CHAT_URL, DEFAULT_LOCAL_MODEL, "ollama_chat_probe")
}

fn run_openai_chat_probe(
    api_url: &str,
    model: &str,
    request_stem: &str,
) -> Result<std::result::Result<String, String>> {
    let request_body = format!(
        r#"{{"model":"{model}","messages":[{{"role":"user","content":"Reply with the single token ok."}}],"max_tokens":8}}"#
    );
    let tempdir = crate::project_data::tempdir()?;
    let request_path = tempdir.path().join(format!("{request_stem}.json"));
    let response_path = tempdir.path().join(format!("{request_stem}.out"));
    fs::write(&request_path, request_body)?;

    let mut command = Command::new("curl");
    crate::project_data::configure_command(&mut command)?;
    let output = command
        .arg("-s")
        .arg("--max-time")
        .arg(LOCAL_CHAT_PROBE_TIMEOUT_SECONDS.to_string())
        .arg("-o")
        .arg(&response_path)
        .arg("-w")
        .arg("%{http_code}")
        .arg("-X")
        .arg("POST")
        .arg(api_url)
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(format!("@{}", request_path.display()))
        .output()?;

    if !output.status.success() {
        return Ok(Err(format_curl_probe_failure(
            &output.stderr,
            output.status.code(),
            LOCAL_CHAT_PROBE_TIMEOUT_SECONDS,
            api_url,
        )));
    }

    let http_code = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let response_body = fs::read_to_string(&response_path).unwrap_or_default();
    if http_code != "200" {
        return Ok(Err(format!(
            "HTTP {http_code}: {}",
            truncate_for_display(&response_body, 120)
        )));
    }

    match parse_openai_chat_response(&response_body) {
        Ok(summary) => Ok(Ok(summary.assistant_content)),
        Err(detail) => Ok(Err(detail)),
    }
}

fn format_curl_probe_failure(
    stderr: &[u8],
    status_code: Option<i32>,
    timeout_seconds: u64,
    api_url: &str,
) -> String {
    let stderr = String::from_utf8_lossy(stderr).trim().to_string();
    if !stderr.is_empty() {
        return stderr;
    }

    if status_code == Some(28) {
        return format!("curl timed out after {timeout_seconds}s while probing `{api_url}`");
    }

    format!("curl exited with {status_code:?} while probing `{api_url}`")
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
        .any(|model| model.name == DEFAULT_LOCAL_MODEL);

    Ok(OllamaTagsSummary {
        default_model_present,
        visible_models: parsed.models.len(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenAiModelsSummary {
    default_model_present: bool,
    visible_models: usize,
}

fn parse_openai_models_response(
    response: &str,
    default_model: &str,
) -> std::result::Result<OpenAiModelsSummary, String> {
    #[derive(Deserialize)]
    struct ModelsResponse {
        #[serde(default)]
        data: Vec<ModelRecord>,
    }

    #[derive(Deserialize)]
    struct ModelRecord {
        id: String,
    }

    if response.trim().is_empty() {
        return Err("empty response from /v1/models".to_string());
    }

    let parsed = serde_json::from_str::<ModelsResponse>(response)
        .map_err(|error| format!("invalid /v1/models response: {error}"))?;
    let default_model_present = parsed.data.iter().any(|model| model.id == default_model);

    Ok(OpenAiModelsSummary {
        default_model_present,
        visible_models: parsed.data.len(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OpenAiChatSummary {
    assistant_content: String,
}

fn parse_openai_chat_response(response: &str) -> std::result::Result<OpenAiChatSummary, String> {
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

    Ok(OpenAiChatSummary { assistant_content })
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
        DEFAULT_LOCAL_MODEL, LOCAL_CHAT_PROBE_TIMEOUT_SECONDS, format_curl_probe_failure,
        parse_ollama_tags_response, parse_openai_chat_response, parse_openai_models_response,
        truncate_for_display, yes_no,
    };

    #[test]
    fn parse_ollama_tags_response_detects_default_model() {
        let response = format!(
            r#"{{"models":[{{"name":"{DEFAULT_LOCAL_MODEL}"}},{{"name":"other:model"}}]}}"#
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
    fn parse_openai_models_response_detects_default_model() {
        let response =
            format!(r#"{{"data":[{{"id":"{DEFAULT_LOCAL_MODEL}"}},{{"id":"other:model"}}]}}"#);

        let summary =
            parse_openai_models_response(&response, DEFAULT_LOCAL_MODEL).expect("valid model list");

        assert!(summary.default_model_present);
        assert_eq!(summary.visible_models, 2);
    }

    #[test]
    fn parse_openai_models_response_reports_missing_default_model() {
        let response = r#"{"data":[{"id":"other:model"}]}"#;

        let summary =
            parse_openai_models_response(response, DEFAULT_LOCAL_MODEL).expect("valid model list");

        assert!(!summary.default_model_present);
        assert_eq!(summary.visible_models, 1);
    }

    #[test]
    fn parse_openai_chat_response_accepts_openai_compatible_string_content() {
        let response = r#"{"choices":[{"message":{"content":"ok"}}]}"#;

        let summary = parse_openai_chat_response(response).expect("valid chat response");

        assert_eq!(summary.assistant_content, "ok");
    }

    #[test]
    fn parse_openai_chat_response_accepts_array_content() {
        let response = r#"{"choices":[{"message":{"content":[{"type":"text","text":"ok"}]}}]}"#;

        let summary = parse_openai_chat_response(response).expect("valid array content");

        assert_eq!(summary.assistant_content, "ok");
    }

    #[test]
    fn truncate_for_display_appends_ellipsis_when_needed() {
        assert_eq!(truncate_for_display("abcdef", 3), "abc…");
        assert_eq!(truncate_for_display("abc", 3), "abc");
    }

    #[test]
    fn format_curl_probe_failure_reports_timeout_context() {
        let detail = format_curl_probe_failure(
            b"",
            Some(28),
            LOCAL_CHAT_PROBE_TIMEOUT_SECONDS,
            "http://localhost:11434/v1/chat/completions",
        );

        assert!(detail.contains("timed out after 30s"));
        assert!(detail.contains("http://localhost:11434/v1/chat/completions"));
    }

    #[test]
    fn format_curl_probe_failure_preserves_stderr() {
        let detail = format_curl_probe_failure(
            b"connection refused",
            Some(7),
            LOCAL_CHAT_PROBE_TIMEOUT_SECONDS,
            "http://localhost:11434/api/tags",
        );

        assert_eq!(detail, "connection refused");
    }

    // --- yes_no ---

    #[test]
    fn yes_no_returns_yes_for_true() {
        assert_eq!(yes_no(true), "yes");
    }

    #[test]
    fn yes_no_returns_no_for_false() {
        assert_eq!(yes_no(false), "no");
    }

    // --- truncate_for_display edge cases ---

    #[test]
    fn truncate_for_display_empty_string() {
        assert_eq!(truncate_for_display("", 5), "");
    }

    #[test]
    fn truncate_for_display_zero_max_chars() {
        assert_eq!(truncate_for_display("hello", 0), "…");
    }

    #[test]
    fn truncate_for_display_exact_max_chars() {
        assert_eq!(truncate_for_display("hello", 5), "hello");
    }

    #[test]
    fn truncate_for_display_shorter_than_max() {
        assert_eq!(truncate_for_display("hi", 10), "hi");
    }

    // --- parse_ollama_tags_response error cases ---

    #[test]
    fn parse_ollama_tags_response_rejects_invalid_json() {
        let result = parse_ollama_tags_response("not json");
        assert!(result.is_err());
    }

    // --- parse_openai_models_response error cases ---

    #[test]
    fn parse_openai_models_response_rejects_invalid_json() {
        let result = parse_openai_models_response("not json", DEFAULT_LOCAL_MODEL);
        assert!(result.is_err());
    }

    #[test]
    fn parse_openai_chat_response_rejects_invalid_json() {
        let result = parse_openai_chat_response("not json");
        assert!(result.is_err());
    }

    #[test]
    fn parse_openai_chat_response_rejects_missing_choices() {
        let result = parse_openai_chat_response(r#"{"other": "field"}"#);
        assert!(result.is_err());
    }
}
