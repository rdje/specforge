use std::env;
use std::io;

use specforge::ir::trajectory_snapshot::{
    check_current_trajectory_artifacts, write_current_trajectory_artifacts,
};

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let mode = arguments.next().ok_or_else(|| {
        invalid_input(
            "usage: cargo run -p specforge --example trajectory_snapshot -- <--check|--write>",
        )
    })?;
    if let Some(unexpected) = arguments.next() {
        return Err(invalid_input(format!(
            "unexpected argument '{}'; the snapshot tool accepts one mode",
            unexpected.to_string_lossy()
        ))
        .into());
    }
    match mode.to_string_lossy().as_ref() {
        "--check" => check_current_trajectory_artifacts()?,
        "--write" => write_current_trajectory_artifacts()?,
        unexpected => {
            return Err(invalid_input(format!(
                "unexpected mode '{unexpected}'; expected --check or --write"
            ))
            .into());
        }
    }
    Ok(())
}
