//! Shared OpenAI-compatible **text** chat transport for the prose-extraction
//! commands (`extract-contracts`, `signal-resolve`).
//!
//! De-duplicates the curl + `SPECFORGE_VLM_HELPER` test hook + request/response
//! shape that both commands previously copied (originally from `nlp_enrich`).
//! Only the transport plumbing lives here — each command keeps its own prompt
//! builder and its own pure response classifier.
//!
//! NOTE: `nlp_enrich` keeps its own inline copy for now (it has a different
//! result type and is the command the running converge jobs exercise);
//! routing it onto this helper too is a tracked follow-up
//! (`LLM-TEXT-TRANSPORT-DEDUP` Non-Goals).

use std::fs;
use std::process::Command;

use crate::cli::VlmProviderArg;
use crate::error::{AppError, Result};

/// Env var overriding the LLM helper with a test script (covers `enrich`,
/// `nlp-enrich`, `extract-contracts`, `signal-resolve`).
pub(crate) const VLM_HELPER_ENV: &str = "SPECFORGE_VLM_HELPER";

pub(crate) fn provider_name(provider: VlmProviderArg) -> &'static str {
    match provider {
        VlmProviderArg::Ollama => "ollama",
        VlmProviderArg::OpenAi => "openai",
        VlmProviderArg::LmStudio => "lmstudio",
        VlmProviderArg::Skip => "skip",
    }
}

pub(crate) fn default_model(provider: VlmProviderArg) -> String {
    match provider {
        VlmProviderArg::Ollama | VlmProviderArg::LmStudio => "qwen2.5vl:7b".to_string(),
        VlmProviderArg::OpenAi => "gpt-4o".to_string(),
        VlmProviderArg::Skip => String::new(),
    }
}

pub(crate) fn api_url(provider: VlmProviderArg) -> &'static str {
    match provider {
        VlmProviderArg::Ollama => "http://localhost:11434/v1/chat/completions",
        VlmProviderArg::OpenAi => "https://api.openai.com/v1/chat/completions",
        VlmProviderArg::LmStudio => "http://localhost:1234/v1/chat/completions",
        VlmProviderArg::Skip => "",
    }
}

/// OpenAI-compatible text-only chat request body. `max_tokens` is caller-chosen
/// so each command keeps its own response budget (e.g. `nlp-enrich` uses a
/// smaller cap because it scans every normative statement).
fn build_text_chat_request(model: &str, prompt: &str, max_tokens: usize) -> String {
    let prompt_escaped = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!(
        r#"{{"model": "{model}", "messages": [{{"role": "user", "content": "{prompt_escaped}"}}], "max_tokens": {max_tokens}, "temperature": 0}}"#
    )
}

/// Extract the assistant message content from an OpenAI-compatible chat response.
fn extract_chat_content(response_json: &str) -> Result<String> {
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
            "invalid LLM response: {e}\nraw: {}",
            &response_json[..response_json.len().min(256)]
        ))
    })?;
    let content = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| AppError::InvalidStageArtifact("LLM response has no choices".to_string()))?
        .message
        .content;
    match content {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Array(parts) => {
            for part in &parts {
                if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                    return Ok(text.to_string());
                }
            }
            Err(AppError::InvalidStageArtifact(
                "LLM response content array has no text part".to_string(),
            ))
        }
        other => Ok(other.to_string()),
    }
}

/// Send `prompt` to the provider for one statement and return the raw assistant
/// text. Honors the `SPECFORGE_VLM_HELPER` test override (passed
/// `--statement-id` / `--sentence`, matching `nlp_enrich`'s hook shape so one
/// mock covers all the text commands).
pub(crate) fn call_text_provider(
    provider: VlmProviderArg,
    model: &str,
    api_url: &str,
    statement_id: &str,
    sentence: &str,
    prompt: &str,
    max_tokens: usize,
) -> Result<String> {
    if let Some(helper_path) = std::env::var_os(VLM_HELPER_ENV) {
        let output = Command::new(&helper_path)
            .arg("--statement-id")
            .arg(statement_id)
            .arg("--sentence")
            .arg(sentence)
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
    let request_body = build_text_chat_request(model, prompt, max_tokens);
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
                resolution: "Set OPENAI_API_KEY environment variable".to_string(),
            })?;
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {api_key}"));
    }
    let tempdir = tempfile::tempdir()?;
    let request_path = tempdir.path().join("text_request.json");
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
    extract_chat_content(&String::from_utf8_lossy(&output.stdout))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_body_is_valid_json_with_escaped_prompt() {
        let body = build_text_chat_request("qwen2.5vl:7b", "say \"hi\"\nnow", 512);
        let v: serde_json::Value = serde_json::from_str(&body).expect("valid request JSON");
        assert_eq!(v["model"], "qwen2.5vl:7b");
        assert_eq!(v["messages"][0]["role"], "user");
        assert_eq!(v["messages"][0]["content"], "say \"hi\"\nnow");
        assert_eq!(v["temperature"], 0);
    }

    #[test]
    fn request_body_honors_caller_max_tokens() {
        let small = build_text_chat_request("m", "p", 256);
        let v: serde_json::Value = serde_json::from_str(&small).expect("valid request JSON");
        assert_eq!(v["max_tokens"], 256);
        let large = build_text_chat_request("m", "p", 512);
        let v2: serde_json::Value = serde_json::from_str(&large).expect("valid request JSON");
        assert_eq!(v2["max_tokens"], 512);
    }

    #[test]
    fn extract_content_handles_string_and_array_shapes() {
        let s = r#"{"choices":[{"message":{"content":"hello"}}]}"#;
        assert_eq!(extract_chat_content(s).unwrap(), "hello");
        let a = r#"{"choices":[{"message":{"content":[{"type":"text","text":"hi"}]}}]}"#;
        assert_eq!(extract_chat_content(a).unwrap(), "hi");
    }

    #[test]
    fn extract_content_fails_closed_on_garbage_and_no_choices() {
        assert!(extract_chat_content("not json").is_err());
        assert!(extract_chat_content(r#"{"choices":[]}"#).is_err());
    }
}
