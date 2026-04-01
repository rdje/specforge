use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde::Deserialize;
use tempfile::tempdir;

use crate::error::{AppError, Result};

use super::{PageArtifact, PlaceholderBinding, SourceArtifactLayout, VisualAsset};

const DOCLING_HELPER_ENV: &str = "SPECFORGE_DOCLING_HELPER";
const DOCLING_PYTHON_ENV: &str = "SPECFORGE_DOCLING_PYTHON";
const PYTHON_CANDIDATES: &[&str] = &["python3", "python"];
const DOCLING_HELPER_SCRIPT: &str = r###"
import argparse
import json
import os
import sys
from importlib import metadata
from pathlib import Path


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    parser.add_argument("--markdown", required=True)
    parser.add_argument("--page-image-root", required=True)
    parser.add_argument("--visual-asset-root", required=True)
    parser.add_argument("--backend-raw-output", required=True)
    parser.add_argument("--metadata-output", required=True)
    parser.add_argument("--summary-output", required=True)
    parser.add_argument("--document-key", required=True)
    return parser.parse_args()


def normalize_text(value):
    if value is None:
        return None
    text = str(value).strip()
    return text or None


def as_posix(path):
    return Path(path).as_posix()


def relative_uri(from_dir, target):
    return Path(os.path.relpath(target, from_dir)).as_posix()


def picture_asset_kind(caption_text):
    lowered = (caption_text or "").lower()
    if "screenshot" in lowered or "screen shot" in lowered:
        return "screenshot"
    if "diagram" in lowered or "block diagram" in lowered:
        return "diagram"
    if "chart" in lowered or "plot" in lowered or "graph" in lowered:
        return "chart"
    return "figure"


def save_json(path, payload):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, ensure_ascii=False, indent=2), encoding="utf-8")


def main():
    args = parse_args()

    try:
        from docling.datamodel.base_models import InputFormat
        from docling.datamodel.pipeline_options import PdfPipelineOptions
        from docling.document_converter import DocumentConverter, PdfFormatOption
        from docling_core.types.doc import ImageRefMode, PictureItem, TableItem
    except ImportError as exc:
        print(f"docling import failed: {exc}", file=sys.stderr)
        return 2

    input_path = Path(args.input)
    markdown_path = Path(args.markdown)
    page_image_root = Path(args.page_image_root)
    visual_asset_root = Path(args.visual_asset_root)
    backend_raw_output_path = Path(args.backend_raw_output)
    metadata_output_path = Path(args.metadata_output)
    summary_output_path = Path(args.summary_output)

    markdown_path.parent.mkdir(parents=True, exist_ok=True)
    page_image_root.mkdir(parents=True, exist_ok=True)
    visual_asset_root.mkdir(parents=True, exist_ok=True)
    backend_raw_output_path.parent.mkdir(parents=True, exist_ok=True)
    metadata_output_path.parent.mkdir(parents=True, exist_ok=True)
    summary_output_path.parent.mkdir(parents=True, exist_ok=True)

    pipeline_options = PdfPipelineOptions()
    pipeline_options.images_scale = 2.0
    pipeline_options.generate_page_images = True
    pipeline_options.generate_picture_images = True

    converter = DocumentConverter(
        format_options={
            InputFormat.PDF: PdfFormatOption(pipeline_options=pipeline_options)
        }
    )
    result = converter.convert(str(input_path))
    doc = result.document

    page_metadata_records = {}
    page_artifacts = []
    for page in doc.pages.values():
        page_number = int(page.page_no)
        page_id = f"page_{page_number:04d}"
        page_image_path = page_image_root / f"page-{page_number:04d}.png"
        page_metadata_path = page_image_root / f"page-{page_number:04d}.json"

        page.image.pil_image.save(page_image_path, format="PNG")

        page_record = {
            "page_id": page_id,
            "page_number": page_number,
            "size_points": {
                "width": getattr(page.size, "width", None),
                "height": getattr(page.size, "height", None),
            },
            "rendered_image": {
                "path": as_posix(page_image_path),
                "width_px": int(round(page.image.size.width)),
                "height_px": int(round(page.image.size.height)),
                "dpi": int(round(page.image.dpi)) if page.image.dpi is not None else None,
            },
            "picture_refs": [],
            "table_refs": [],
        }
        page_metadata_records[page_number] = {
            "record": page_record,
            "metadata_path": page_metadata_path,
        }
        page_artifacts.append(
            {
                "page_id": page_id,
                "page_number": page_number,
                "page_image_path": as_posix(page_image_path),
                "layout_metadata_path": as_posix(page_metadata_path),
                "width_px": int(round(page.image.size.width)),
                "height_px": int(round(page.image.size.height)),
            }
        )

    visual_assets = []
    picture_counter = 0
    table_counter = 0
    for element, _level in doc.iterate_items():
        if isinstance(element, PictureItem):
            picture_counter += 1
            page_number = int(element.prov[0].page_no) if element.prov else None
            page_id = f"page_{page_number:04d}" if page_number is not None else None
            asset_id = f"picture_{picture_counter:04d}"
            asset_path = visual_asset_root / f"picture-{picture_counter:04d}.png"
            element.get_image(doc).save(asset_path, "PNG")
            if element.image is not None:
                element.image.uri = relative_uri(markdown_path.parent, asset_path)
            caption_text = normalize_text(element.caption_text(doc))
            if page_number in page_metadata_records:
                page_metadata_records[page_number]["record"]["picture_refs"].append(
                    element.self_ref
                )
            visual_assets.append(
                {
                    "asset_id": asset_id,
                    "asset_kind": picture_asset_kind(caption_text),
                    "page_id": page_id,
                    "image_path": as_posix(asset_path),
                    "caption_text": caption_text,
                    "caption_source_path": as_posix(backend_raw_output_path),
                    "source_ref": element.self_ref,
                    "placeholder_text": None,
                    "note": None,
                }
            )
        elif isinstance(element, TableItem):
            table_counter += 1
            page_number = int(element.prov[0].page_no) if element.prov else None
            page_id = f"page_{page_number:04d}" if page_number is not None else None
            asset_id = f"table_{table_counter:04d}"
            asset_path = visual_asset_root / f"table-{table_counter:04d}.png"
            element.get_image(doc).save(asset_path, "PNG")
            caption_text = normalize_text(element.caption_text(doc))
            if page_number in page_metadata_records:
                page_metadata_records[page_number]["record"]["table_refs"].append(
                    element.self_ref
                )
            visual_assets.append(
                {
                    "asset_id": asset_id,
                    "asset_kind": "table_region",
                    "page_id": page_id,
                    "image_path": as_posix(asset_path),
                    "caption_text": caption_text,
                    "caption_source_path": as_posix(backend_raw_output_path),
                    "source_ref": element.self_ref,
                    "placeholder_text": None,
                    "note": None,
                }
            )

    for page_number in sorted(page_metadata_records):
        entry = page_metadata_records[page_number]
        save_json(entry["metadata_path"], entry["record"])

    markdown_text = doc.export_to_markdown(image_mode=ImageRefMode.REFERENCED)
    markdown_path.write_text(markdown_text, encoding="utf-8")
    save_json(backend_raw_output_path, doc.export_to_dict())

    docling_version = None
    try:
        docling_version = metadata.version("docling")
    except metadata.PackageNotFoundError:
        docling_version = None

    save_json(
        metadata_output_path,
        {
            "backend_name": "docling",
            "backend_version": docling_version,
            "document_key": args.document_key,
            "input_path": as_posix(input_path),
            "promoted_markdown_path": as_posix(markdown_path),
            "page_count": len(page_artifacts),
            "picture_count": picture_counter,
            "table_count": table_counter,
            "images_scale": 2.0,
            "generate_page_images": True,
            "generate_picture_images": True,
        },
    )
    save_json(
        summary_output_path,
        {
            "backend_version": docling_version,
            "page_artifacts": page_artifacts,
            "visual_assets": visual_assets,
            "placeholder_bindings": [],
            "metadata": {
                "page_count": len(page_artifacts),
                "picture_count": picture_counter,
                "table_count": table_counter,
            },
        },
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
"###;

#[derive(Debug, Deserialize)]
pub struct DoclingBackendSummary {
    pub backend_version: Option<String>,
    pub page_artifacts: Vec<PageArtifact>,
    pub visual_assets: Vec<VisualAsset>,
    #[serde(default)]
    pub placeholder_bindings: Vec<PlaceholderBinding>,
    pub metadata: DoclingDocumentMetadata,
}

#[derive(Debug, Deserialize)]
pub struct DoclingDocumentMetadata {
    pub page_count: usize,
    pub picture_count: usize,
    pub table_count: usize,
}

struct BackendCommand {
    display_name: String,
    command: Command,
}

enum PythonProbe {
    Ready,
    MissingCommand,
    ImportFailed(String),
}

pub fn materialize_pdf(
    source_path: &Path,
    promoted_markdown_path: &Path,
    metadata_output_path: &Path,
    artifact_layout: &SourceArtifactLayout,
    document_key: &str,
) -> Result<DoclingBackendSummary> {
    fs::create_dir_all(&artifact_layout.normalized_root)?;
    fs::create_dir_all(&artifact_layout.page_image_root)?;
    fs::create_dir_all(&artifact_layout.visual_asset_root)?;

    let tempdir = tempdir()?;
    let summary_output_path = tempdir.path().join("docling_summary.json");
    let mut backend_command = build_backend_command(tempdir.path())?;
    backend_command
        .command
        .arg("--input")
        .arg(source_path)
        .arg("--markdown")
        .arg(promoted_markdown_path)
        .arg("--page-image-root")
        .arg(&artifact_layout.page_image_root)
        .arg("--visual-asset-root")
        .arg(&artifact_layout.visual_asset_root)
        .arg("--backend-raw-output")
        .arg(&artifact_layout.backend_raw_output_path)
        .arg("--metadata-output")
        .arg(metadata_output_path)
        .arg("--summary-output")
        .arg(&summary_output_path)
        .arg("--document-key")
        .arg(document_key);

    let output = backend_command.command.output()?;
    if !output.status.success() {
        return Err(AppError::ExternalCommandFailed {
            program: backend_command.display_name,
            exit_code: output.status.code(),
            stderr: render_command_output(&output),
        });
    }

    let summary_text = fs::read_to_string(&summary_output_path).map_err(|error| {
        AppError::InvalidBackendOutput(format!(
            "docling backend completed without writing a summary manifest at {}: {error}",
            summary_output_path.display()
        ))
    })?;
    let summary =
        serde_json::from_str::<DoclingBackendSummary>(&summary_text).map_err(|error| {
            AppError::InvalidBackendOutput(format!(
                "failed to parse docling backend summary at {}: {error}",
                summary_output_path.display()
            ))
        })?;

    Ok(summary)
}

fn build_backend_command(tempdir: &Path) -> Result<BackendCommand> {
    if let Some(helper_override) = env::var_os(DOCLING_HELPER_ENV) {
        let helper_path = PathBuf::from(helper_override);
        return Ok(BackendCommand {
            display_name: helper_path.display().to_string(),
            command: Command::new(helper_path),
        });
    }

    let python = resolve_docling_python()?;
    let helper_script_path = tempdir.join("specforge_docling_backend.py");
    fs::write(&helper_script_path, DOCLING_HELPER_SCRIPT)?;

    let mut command = Command::new(&python);
    command.arg(&helper_script_path);

    Ok(BackendCommand {
        display_name: format!("{} {}", python.display(), helper_script_path.display()),
        command,
    })
}

fn resolve_docling_python() -> Result<PathBuf> {
    if let Some(runtime_override) = env::var_os(DOCLING_PYTHON_ENV) {
        let candidate = PathBuf::from(runtime_override);
        return match probe_docling_python(&candidate)? {
            PythonProbe::Ready => Ok(candidate),
            PythonProbe::MissingCommand => Err(AppError::MissingRuntimeDependency {
                dependency: "docling python runtime",
                resolution: format!(
                    "`{DOCLING_PYTHON_ENV}` points at `{}` but that command is not available; set it to a working Python interpreter or unset it",
                    candidate.display()
                ),
            }),
            PythonProbe::ImportFailed(detail) => Err(AppError::MissingRuntimeDependency {
                dependency: "docling",
                resolution: format!(
                    "`{DOCLING_PYTHON_ENV}` points at `{}` but `import docling` failed there: {}; install `docling` into that interpreter or unset `{DOCLING_PYTHON_ENV}`",
                    candidate.display(),
                    detail
                ),
            }),
        };
    }

    for candidate in PYTHON_CANDIDATES {
        let candidate_path = Path::new(candidate);
        if matches!(probe_docling_python(candidate_path)?, PythonProbe::Ready) {
            return Ok(candidate_path.to_path_buf());
        }
    }

    Err(AppError::MissingRuntimeDependency {
        dependency: "docling",
        resolution: format!(
            "install `docling` into a Python interpreter available as `python3` or `python`, or set `{DOCLING_PYTHON_ENV}` to an interpreter where `import docling` succeeds"
        ),
    })
}

fn probe_docling_python(candidate: &Path) -> Result<PythonProbe> {
    match Command::new(candidate)
        .args(["-c", "import docling"])
        .output()
    {
        Ok(output) if output.status.success() => Ok(PythonProbe::Ready),
        Ok(output) => Ok(PythonProbe::ImportFailed(render_command_output(&output))),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(PythonProbe::MissingCommand),
        Err(error) => Err(error.into()),
    }
}

fn render_command_output(output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    match (stdout.is_empty(), stderr.is_empty()) {
        (false, false) => format!("stdout: {stdout}; stderr: {stderr}"),
        (false, true) => format!("stdout: {stdout}"),
        (true, false) => format!("stderr: {stderr}"),
        (true, true) => "no stdout or stderr captured".to_string(),
    }
}
