use std::env;
use std::path::PathBuf;

use specforge_production_graph::{analyze_information_flow, analyze_repository};

fn main() {
    if let Err(error) = run() {
        eprintln!("production-genericity-graph: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args_os();
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut emit_json = false;
    let mut flow = false;
    while let Some(argument) = args.next() {
        match argument.to_str() {
            Some("--root") => {
                root = args
                    .next()
                    .map(PathBuf::from)
                    .ok_or_else(|| "--root requires a path".to_owned())?;
            }
            Some("--json") => emit_json = true,
            Some("--flow") => flow = true,
            Some("--help") | Some("-h") => {
                println!(
                    "Usage: specforge-production-graph [--root REPOSITORY_ROOT] [--json] [--flow]"
                );
                return Ok(());
            }
            Some(other) => return Err(format!("unknown argument '{other}'")),
            None => return Err("arguments must be valid UTF-8".to_owned()),
        }
    }
    if flow {
        let report = analyze_information_flow(&root)?;
        if emit_json {
            println!(
                "{}",
                serde_json::to_string_pretty(&report)
                    .map_err(|error| format!("cannot serialize flow report: {error}"))?
            );
        } else {
            println!("{}", report.summary());
        }
    } else {
        let graph = analyze_repository(&root)?;
        if emit_json {
            println!(
                "{}",
                serde_json::to_string_pretty(&graph)
                    .map_err(|error| format!("cannot serialize graph: {error}"))?
            );
        } else {
            println!("{}", graph.summary());
        }
    }
    Ok(())
}
