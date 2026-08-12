use clap::ValueEnum;

/// Text/vision provider selection shared by core proposal producers and the application CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum VlmProviderArg {
    /// Local Ollama server at http://localhost:11434.
    Ollama,
    /// OpenAI cloud API. Requires `OPENAI_API_KEY`.
    OpenAi,
    /// Local OpenAI-compatible LM Studio server.
    LmStudio,
    /// Skip model enrichment.
    Skip,
}
