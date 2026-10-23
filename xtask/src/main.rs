//! `cargo xtask <command>`: see the library for the command table.

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let Some(name) = arguments.first() else {
        eprint!("{}", xtask::usage());
        return ExitCode::from(2);
    };
    if matches!(name.as_str(), "help" | "--help" | "-h") {
        print!("{}", xtask::usage());
        return ExitCode::SUCCESS;
    }
    let Some(task) = xtask::find(name) else {
        eprintln!("xtask: unknown command '{name}'");
        eprint!("{}", xtask::usage());
        return ExitCode::from(2);
    };
    match xtask::run(task, &arguments[1..]) {
        Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        Err(error) => {
            eprintln!("xtask: cannot run {}: {error}", task.name);
            ExitCode::from(1)
        }
    }
}
