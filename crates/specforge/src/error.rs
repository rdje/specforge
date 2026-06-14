use std::fmt;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingPath(PathBuf),
    FeatureNotYetImplemented(&'static str),
    MissingRuntimeDependency {
        dependency: &'static str,
        resolution: String,
    },
    ExternalCommandFailed {
        program: String,
        exit_code: Option<i32>,
        stderr: String,
    },
    InvalidBackendOutput(String),
    InvalidStageArtifact(String),
    IngestAbortedForMemory {
        program: String,
        used_percent: f64,
        ceiling_percent: f64,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "i/o error: {error}"),
            Self::Json(error) => write!(f, "json error: {error}"),
            Self::MissingPath(path) => write!(f, "path does not exist: {}", path.display()),
            Self::FeatureNotYetImplemented(feature) => {
                write!(f, "feature not implemented yet: {feature}")
            }
            Self::MissingRuntimeDependency {
                dependency,
                resolution,
            } => {
                write!(
                    f,
                    "missing runtime dependency: {dependency}; resolution: {resolution}"
                )
            }
            Self::ExternalCommandFailed {
                program,
                exit_code,
                stderr,
            } => {
                write!(
                    f,
                    "external command failed: {program} (exit code: {}): {stderr}",
                    exit_code
                        .map(|code| code.to_string())
                        .unwrap_or_else(|| "terminated by signal".to_string())
                )
            }
            Self::InvalidBackendOutput(message) => write!(f, "invalid backend output: {message}"),
            Self::InvalidStageArtifact(message) => write!(f, "invalid stage artifact: {message}"),
            Self::IngestAbortedForMemory {
                program,
                used_percent,
                ceiling_percent,
            } => {
                write!(
                    f,
                    "ingest aborted to protect the host: system memory was {used_percent:.0}% used, \
                     at or above the {ceiling_percent:.0}% safety ceiling, while running {program}. \
                     The host was preserved and the previous normalized bundle is intact. \
                     Free memory and retry, raise the ceiling with \
                     SPECFORGE_INGEST_RAM_ABORT_PERCENT=<percent>, or disable the guard with \
                     SPECFORGE_INGEST_RAM_ABORT_PERCENT=off."
                )
            }
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}
