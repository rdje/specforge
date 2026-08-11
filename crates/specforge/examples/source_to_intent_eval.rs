use std::env;
use std::io::{self, Write};
use std::path::Path;

use specforge::ir::source_to_intent_eval::{evaluate_dataset, load_dataset};

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let dataset_path = arguments.next().ok_or_else(|| {
        invalid_input(
            "usage: cargo run -p specforge --example source_to_intent_eval -- <repository-relative-dataset>",
        )
    })?;
    if let Some(unexpected) = arguments.next() {
        return Err(invalid_input(format!(
            "unexpected argument '{}'; the evaluator accepts exactly one repository-relative dataset path",
            unexpected.to_string_lossy()
        ))
        .into());
    }

    let dataset = load_dataset(Path::new(&dataset_path))?;
    let report = evaluate_dataset(&dataset)
        .map_err(|problems| invalid_input(format!("evaluation failed: {}", problems.join("; "))))?;

    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer_pretty(&mut output, &report)?;
    output.write_all(b"\n")?;
    Ok(())
}
