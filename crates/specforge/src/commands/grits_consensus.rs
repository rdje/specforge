//! `grits-consensus` — score docling's table extraction against a CROSS-TOOL CONSENSUS gold.
//!
//! GriTS was "gated on data" (no table-structure gold). The unblock (GRITS-CROSS-TOOL): build the
//! gold from INDEPENDENT WITNESSES whose errors are uncorrelated with docling's — pdfplumber
//! (geometric / content-stream) + qwen2.5vl (vision). A cell ≥`min_agree` witnesses agree on is
//! silver gold; a cell they SPLIT on is flagged for human review (the small set worth a person's
//! time). docling (the system under test) is then scored against that gold with `grits_content`.
//! You never grade a tool against itself — docling is the *prediction*, never a witness.
//!
//! Input is the witness JSON emitted by `scripts/grits_cross_tool.py` (which runs the witnesses +
//! aligns/matches tables). This command owns only the metric: consensus + GriTS + the flag count.

use crate::cli::GritsConsensusArgs;
use crate::error::{AppError, Result};
use crate::eval::{Scorecard, grits_against_consensus, witness_consensus};
use serde::Deserialize;

#[derive(Deserialize)]
struct WitnessTable {
    #[serde(default)]
    table_id: String,
    #[serde(default)]
    page: Option<u32>,
    /// One grid per independent witness (rows of cell text).
    witnesses: Vec<Vec<Vec<String>>>,
    /// docling's grid for the same table (the system under test).
    prediction: Vec<Vec<String>>,
}

#[derive(Deserialize)]
struct WitnessDoc {
    tables: Vec<WitnessTable>,
}

pub fn run(args: GritsConsensusArgs) -> Result<()> {
    let text = std::fs::read_to_string(&args.witnesses_json)?;
    let doc: WitnessDoc = serde_json::from_str(&text).map_err(|e| {
        AppError::InvalidStageArtifact(format!(
            "witness json {}: {e}",
            args.witnesses_json.display()
        ))
    })?;

    println!("command: grits-consensus");
    println!("witnesses_json: {}", args.witnesses_json.display());
    println!(
        "matched tables: {} | min_agree: {}",
        doc.tables.len(),
        args.min_agree
    );

    let mut agg = Scorecard::default();
    let mut human_flags = 0usize;
    for t in &doc.tables {
        let consensus = witness_consensus(&t.witnesses, args.min_agree);
        let card = grits_against_consensus(&consensus.gold, &t.prediction);
        human_flags += consensus.disagreements.len();
        agg.tp += card.tp;
        agg.fp += card.fp;
        agg.fn_count += card.fn_count;
        agg.gold_total += card.gold_total;
        println!(
            "  [{}] page={:?}  grits P={:.3} R={:.3} F1={:.3}  (gold={} tp={} fp={} fn={})  human_flag_cells={}",
            t.table_id,
            t.page,
            card.precision(),
            card.recall(),
            card.f1(),
            card.gold_total,
            card.tp,
            card.fp,
            card.fn_count,
            consensus.disagreements.len()
        );
    }
    println!(
        "=== aggregate (docling vs consensus gold): P={:.3} R={:.3} F1={:.3}  (gold={} tp={} fp={} fn={});  human-flag cells={} ===",
        agg.precision(),
        agg.recall(),
        agg.f1(),
        agg.gold_total,
        agg.tp,
        agg.fp,
        agg.fn_count,
        human_flags
    );
    Ok(())
}
