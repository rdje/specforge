use std::env;
use std::io::{self, Write};
use std::path::Path;

use specforge::ir::trajectory::{evaluate_trajectory, load_controller_input};

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let input_path = arguments.next().ok_or_else(|| {
        invalid_input(
            "usage: cargo run -p specforge --example trajectory_controller -- <repository-relative-controller-input>",
        )
    })?;
    if let Some(unexpected) = arguments.next() {
        return Err(invalid_input(format!(
            "unexpected argument '{}'; the controller accepts exactly one repository-relative input path",
            unexpected.to_string_lossy()
        ))
        .into());
    }

    let input = load_controller_input(Path::new(&input_path))?;
    let report = evaluate_trajectory(&input).map_err(|problems| {
        invalid_input(format!(
            "trajectory evaluation failed: {}",
            problems.join("; ")
        ))
    })?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer_pretty(&mut output, &report)?;
    output.write_all(b"\n")?;
    Ok(())
}
