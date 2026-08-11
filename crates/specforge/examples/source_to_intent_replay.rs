use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use specforge::ir::source_to_intent_replay::{
    SourceToIntentReplayRequest, replay_source_to_intent,
};

fn invalid_input(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message.into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = env::args_os().skip(1);
    let source = arguments.next().ok_or_else(|| {
        invalid_input(
            "usage: cargo run -p specforge --example source_to_intent_replay -- <repository-relative-source> <.project-data/tmp/output-root> [prior-memory|-] [observed-table-id|-]",
        )
    })?;
    let output_root = arguments
        .next()
        .ok_or_else(|| invalid_input("missing repository-local output root"))?;
    let prior_memory = arguments
        .next()
        .map(PathBuf::from)
        .and_then(|path| (path.as_os_str() != "-").then_some(path));
    let observed_table_id = arguments.next().and_then(|value| {
        let value = value.to_string_lossy().into_owned();
        (value != "-").then_some(value)
    });
    if let Some(unexpected) = arguments.next() {
        return Err(invalid_input(format!(
            "unexpected argument '{}'; replay accepts at most four arguments",
            unexpected.to_string_lossy()
        ))
        .into());
    }

    let report = replay_source_to_intent(&SourceToIntentReplayRequest {
        source: PathBuf::from(source),
        output_root: PathBuf::from(output_root),
        prior_memory,
        observed_table_id,
    })?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer_pretty(&mut output, &report)?;
    output.write_all(b"\n")?;
    Ok(())
}
