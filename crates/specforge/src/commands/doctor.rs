use crate::cli::DoctorArgs;
use crate::error::{AppError, Result};
use crate::ir::source::{
    DEFAULT_DOCLING_BOOTSTRAP_SCRIPT, DEFAULT_DOCLING_VENV_DIR, DOCLING_PYTHON_ENV,
    inspect_docling_runtime,
};

pub fn run(args: DoctorArgs) -> Result<()> {
    let diagnosis = inspect_docling_runtime()?;

    println!("command: doctor");
    println!("docling_runtime_ready: {}", yes_no(diagnosis.is_ready()));
    println!("repo_local_docling_venv: {DEFAULT_DOCLING_VENV_DIR}");
    println!("bootstrap_script: {DEFAULT_DOCLING_BOOTSTRAP_SCRIPT}");
    println!("env_override: {DOCLING_PYTHON_ENV}");

    match diagnosis.selected_python.as_ref() {
        Some(path) => println!("selected_python: {}", path.display()),
        None => println!("selected_python: none"),
    }

    println!(
        "selected_source: {}",
        diagnosis
            .selected_source
            .map(|source| source.as_str())
            .unwrap_or("none")
    );
    println!(
        "selected_label: {}",
        diagnosis.selected_label.as_deref().unwrap_or("none")
    );
    println!(
        "selected_python_version: {}",
        diagnosis
            .selected_python_version
            .as_deref()
            .unwrap_or("unknown")
    );
    println!(
        "selected_docling_version: {}",
        diagnosis
            .selected_docling_version
            .as_deref()
            .unwrap_or("unknown")
    );
    println!("candidates: {}", diagnosis.candidates.len());

    for (index, candidate) in diagnosis.candidates.iter().enumerate() {
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

    if !diagnosis.is_ready() {
        let resolution = diagnosis.resolution();
        println!("resolution: {resolution}");
        if args.strict {
            return Err(AppError::MissingRuntimeDependency {
                dependency: "docling",
                resolution,
            });
        }
    }

    Ok(())
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}
