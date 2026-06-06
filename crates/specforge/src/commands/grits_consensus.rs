//! `grits-consensus` — score docling's table extraction against a CROSS-TOOL CONSENSUS gold.
//!
//! GriTS was "gated on data" (no table-structure gold). The unblock (GRITS-CROSS-TOOL): build the
//! gold from INDEPENDENT WITNESSES whose errors are uncorrelated with docling's — pdfplumber
//! (geometric / content-stream) + qwen2.5vl (vision). A cell ≥`min_agree` witnesses agree on is
//! silver gold; a cell they SPLIT on is flagged for ADJUDICATION — resolved against the rendered
//! source by an evidence-grounded agent (or a human), never by a correlated vote. docling (the
//! system under test) is then scored against that gold with `grits_content`.
//! You never grade a tool against itself — docling is the *prediction*, never a witness.
//!
//! Input is the witness JSON emitted by `scripts/grits_cross_tool.py` (which runs the witnesses +
//! aligns/matches tables). This command owns only the metric: consensus + GriTS + the flag count.

use crate::cli::GritsConsensusArgs;
use crate::error::{AppError, Result};
use crate::eval::{
    Scorecard, gold_vs_prediction_mismatches, grits_against_consensus, witness_consensus,
};
use serde::{Deserialize, Serialize};

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

/// One disputed cell for the adjudication queue: where docling and the consensus gold disagree.
#[derive(Serialize)]
struct AdjudicationCell {
    table_id: String,
    page: Option<u32>,
    row: usize,
    col: usize,
    /// The consensus-gold value (`None` = the witnesses had no cell here → docling spurious).
    consensus_gold: Option<String>,
    /// docling's value (`None` = docling missed a gold cell).
    docling: Option<String>,
}

#[derive(Serialize, Default)]
struct AdjudicationQueue {
    cells: Vec<AdjudicationCell>,
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
    let mut queue = AdjudicationQueue::default();
    for t in &doc.tables {
        let consensus = witness_consensus(&t.witnesses, args.min_agree);
        let card = grits_against_consensus(&consensus.gold, &t.prediction);
        human_flags += consensus.disagreements.len();
        // Queue the cells where docling disagrees with the consensus gold — candidate docling
        // errors for the evidence-grounded adjudicator (rendered + ruled on downstream).
        if args.adjudicate_out.is_some() {
            for (row, col, consensus_gold, docling) in
                gold_vs_prediction_mismatches(&consensus.gold, &t.prediction)
            {
                queue.cells.push(AdjudicationCell {
                    table_id: t.table_id.clone(),
                    page: t.page,
                    row,
                    col,
                    consensus_gold,
                    docling,
                });
            }
        }
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
        // The adjudication queue: the cells the witnesses split on, with the competing values so an
        // evidence-grounded agent (or human) can resolve them (capped per table for readability).
        for ((r, c), competing) in consensus.disagreements.iter().take(6) {
            let values: Vec<String> = competing
                .iter()
                .map(|(text, n)| format!("{text:?}×{n}"))
                .collect();
            println!("      adjudicate (r{r},c{c}): {}", values.join("  vs  "));
        }
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

    if let Some(path) = &args.adjudicate_out {
        let json = serde_json::to_string_pretty(&queue)
            .map_err(|e| AppError::InvalidStageArtifact(format!("adjudication queue: {e}")))?;
        std::fs::write(path, json)?;
        println!(
            "adjudication queue: {} disputed cells → {}",
            queue.cells.len(),
            path.display()
        );
    }
    Ok(())
}
