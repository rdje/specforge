use std::env;
use std::path::PathBuf;

use specforge_production_graph::{analyze_information_flow, analyze_repository, load_flow_census};

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
    let mut check_census = false;
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
            Some("--check-census") => {
                flow = true;
                check_census = true;
            }
            Some("--help") | Some("-h") => {
                println!(
                    "Usage: specforge-production-graph [--root REPOSITORY_ROOT] [--json] [--flow] [--check-census]"
                );
                return Ok(());
            }
            Some(other) => return Err(format!("unknown argument '{other}'")),
            None => return Err("arguments must be valid UTF-8".to_owned()),
        }
    }
    if flow {
        let report = analyze_information_flow(&root)?;
        // PRODUCTION-GRAPH-CENSUS-PIN.1 — a census that reports is not a check. The analysis is
        // already paid for above; this adds the comparison, not a run.
        if check_census {
            let breaches = load_flow_census(&root)?.disagreements(&report);
            if !breaches.is_empty() {
                for breach in &breaches {
                    eprintln!("{breach}");
                }
                return Err(format!(
                    "the derived census disagrees with {} in {} field(s) — re-derive the contract and \
                     name the owning leaf, do not edit the number alone",
                    specforge_production_graph::FLOW_CENSUS_CONTRACT,
                    breaches.len()
                ));
            }
        }
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
