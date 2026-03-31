use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceKind {
    Pdf,
    Markdown,
    Directory,
    Unknown,
}

impl SourceKind {
    pub fn detect(path: &Path) -> Self {
        if path.is_dir() {
            return Self::Directory;
        }

        match normalized_extension(path).as_deref() {
            Some("pdf") => Self::Pdf,
            Some("md") | Some("markdown") => Self::Markdown,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Markdown => "markdown",
            Self::Directory => "directory",
            Self::Unknown => "unknown",
        }
    }

    pub fn requires_markdown_conversion(self) -> bool {
        matches!(self, Self::Pdf)
    }
}

fn normalized_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::SourceKind;

    #[test]
    fn detects_pdf_extension_case_insensitively() {
        assert_eq!(SourceKind::detect(Path::new("spec.PDF")), SourceKind::Pdf);
    }

    #[test]
    fn detects_markdown_extensions() {
        assert_eq!(SourceKind::detect(Path::new("spec.md")), SourceKind::Markdown);
        assert_eq!(
            SourceKind::detect(Path::new("spec.markdown")),
            SourceKind::Markdown
        );
    }

    #[test]
    fn reports_unknown_for_other_extensions() {
        assert_eq!(SourceKind::detect(Path::new("spec.txt")), SourceKind::Unknown);
    }
}
