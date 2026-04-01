use std::fmt;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingPath(PathBuf),
    FeatureNotYetImplemented(&'static str),
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
