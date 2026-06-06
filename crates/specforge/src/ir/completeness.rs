//! Completeness miss-detectors (`INTENT-COMPLETENESS-RESEARCH`).
//!
//! Ground-truth-free checks that surface a *candidate miss* as an explicit
//! residual instead of letting it pass silently. Detectors only FLAG; they never
//! mutate the IR and never fabricate facts. See
//! `docs/research/miss-detectors-catalog.md` for the full catalog; this module
//! starts with the EXACT structural-closure detectors
//! (`COMPLETENESS-CLOSURE-INVARIANTS`).

use crate::ir::source::RegisterRecord;

/// The kind of register bit-field tiling problem detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterTilingKind {
    /// Two or more fields claim the same bit — a real contradiction (the
    /// extraction or the spec disagrees with itself).
    Overlap,
    /// A bit *between* the lowest and highest documented field is covered by no
    /// field — a likely **missed field**. Bits above the highest documented
    /// field are deliberately NOT flagged: the IR carries no register width, so
    /// flagging them would speculate.
    InteriorGap,
}

impl RegisterTilingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Overlap => "overlap",
            Self::InteriorGap => "interior_gap",
        }
    }
}

/// One register-tiling completeness residual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterTilingResidual {
    pub register_name: String,
    pub kind: RegisterTilingKind,
    /// Human-readable bit range(s), e.g. `bits [3:2]`.
    pub detail: String,
}

/// Upper bound on the bit span a single register is checked over. Real registers
/// are ≤ 64 bits wide; a larger span indicates a malformed/garbage extraction
/// (e.g. a parsed bit index in the thousands) which we skip rather than allocate
/// for or false-flag.
const MAX_REGISTER_BIT_SPAN: usize = 4096;

/// Detect bit-field **overlaps** and **interior gaps** in each register.
///
/// Conservative by construction: only fields with BOTH `bits_high` and
/// `bits_low` participate; only the span `[min_low, max_high]` (lowest to
/// highest documented bit) is examined, so an unknown register width never
/// induces a false positive. A register with fewer than two bit-bounded fields
/// has neither an overlap nor an *interior* gap to report and is skipped.
pub fn register_tiling_residuals(registers: &[RegisterRecord]) -> Vec<RegisterTilingResidual> {
    let mut residuals = Vec::new();
    for register in registers {
        let mut ranges: Vec<(u32, u32)> = Vec::new();
        for field in &register.fields {
            if let (Some(high), Some(low)) = (field.bits_high, field.bits_low) {
                let (lo, hi) = if low <= high {
                    (low, high)
                } else {
                    (high, low)
                };
                ranges.push((lo, hi));
            }
        }
        if ranges.len() < 2 {
            continue;
        }
        let min_low = ranges.iter().map(|r| r.0).min().unwrap();
        let max_high = ranges.iter().map(|r| r.1).max().unwrap();
        let span = (max_high - min_low + 1) as usize;
        if span > MAX_REGISTER_BIT_SPAN {
            continue;
        }

        let mut cover = vec![0u32; span];
        for &(lo, hi) in &ranges {
            for bit in lo..=hi {
                cover[(bit - min_low) as usize] += 1;
            }
        }

        // Collapse consecutive bits with the same predicate into compact ranges.
        for residual in collect_runs(&cover, min_low, |c| c > 1)
            .into_iter()
            .map(|(lo, hi)| RegisterTilingResidual {
                register_name: register.register_name.clone(),
                kind: RegisterTilingKind::Overlap,
                detail: format_bit_range(lo, hi),
            })
        {
            residuals.push(residual);
        }
        for residual in collect_runs(&cover, min_low, |c| c == 0)
            .into_iter()
            .map(|(lo, hi)| RegisterTilingResidual {
                register_name: register.register_name.clone(),
                kind: RegisterTilingKind::InteriorGap,
                detail: format_bit_range(lo, hi),
            })
        {
            residuals.push(residual);
        }
    }
    residuals
}

/// Collapse the bits whose coverage count satisfies `pred` into contiguous
/// `(low_bit, high_bit)` runs (absolute bit indices, offset by `min_low`).
fn collect_runs(cover: &[u32], min_low: u32, pred: impl Fn(u32) -> bool) -> Vec<(u32, u32)> {
    let mut runs = Vec::new();
    let mut run_start: Option<usize> = None;
    for (idx, &count) in cover.iter().enumerate() {
        if pred(count) {
            run_start.get_or_insert(idx);
        } else if let Some(start) = run_start.take() {
            runs.push((min_low + start as u32, min_low + (idx - 1) as u32));
        }
    }
    if let Some(start) = run_start {
        runs.push((min_low + start as u32, min_low + (cover.len() - 1) as u32));
    }
    runs
}

/// Format an inclusive bit range as `bits [hi:lo]` (or `bit [n]` for a single bit).
fn format_bit_range(low: u32, high: u32) -> String {
    if low == high {
        format!("bit [{low}]")
    } else {
        format!("bits [{high}:{low}]")
    }
}

// ── Region accounting (COMPLETENESS-REGION-ACCOUNTING) ─────────────────────────

use crate::ir::evidence::{
    ExtractorTier, FactKind, FactProvenanceRecord, TableSignalDeclarationProvenanceRecord,
    is_hardware_signal_token, is_signal_synthesis_non_signal,
};
use crate::ir::source::{StructuredTableRecord, TableKind, TimingConstraintRecord};
use std::collections::HashSet;

/// An intent-bearing table — one the classifier recognized as a register /
/// signal / timing table — that produced **no** corresponding extracted record.
/// The extraction recognized the table's purpose but got nothing typed out of it,
/// so it is a candidate miss (an unexplained source region).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnexplainedTableResidual {
    pub table_id: String,
    pub table_kind: &'static str,
    pub caption: Option<String>,
}

/// Region accounting at the table level (the input-side miss reframe): every
/// intent-bearing `SignalDescription` / `RegisterMap` / `TimingParameter` table
/// must produce at least one corresponding record, or it is flagged as
/// unexplained.
///
/// Coverage is resolved through the existing provenance, no fabrication:
/// - SignalDescription → a `TableSignalDeclarationProvenanceRecord` citing the
///   table's `table_id` directly, OR every hardware-signal token it carries is
///   already in the declared-signal inventory (a redundant/duplicate presentation
///   of signals captured from another table — see
///   `signal_table_covered_by_inventory`);
/// - RegisterMap → a `register_record` whose `register_id` embeds the `table_id`
///   (`reg_table_0026_000` ⊃ `table_0026`);
/// - TimingParameter → a `timing_constraint` whose `constraint_id` embeds it
///   (`timing_table_0022_000`).
///
/// The `"{table_id}_"` marker (with the trailing underscore) is used for the
/// embedded-id check so fixed-width ids like `table_0002` cannot spuriously match
/// `table_0020`. Other table kinds are not accounted here.
///
/// `declared_signal_names` is the document's declared-signal inventory (uppercased
/// names, e.g. from `collect_known_signal_names`); it lets the detector tell a
/// genuine catalog miss from a duplicate-content table without fabricating any
/// record (`WIRE-BASED-100.3a`).
pub fn unexplained_intent_bearing_tables(
    tables: &[StructuredTableRecord],
    signal_provenance: &[TableSignalDeclarationProvenanceRecord],
    register_records: &[RegisterRecord],
    timing_constraints: &[TimingConstraintRecord],
    declared_signal_names: &HashSet<String>,
) -> Vec<UnexplainedTableResidual> {
    let mut residuals = Vec::new();
    for table in tables {
        let marker = format!("{}_", table.table_id);
        let (kind_str, covered) = match table.table_kind {
            TableKind::SignalDescription => (
                "signal_description",
                signal_provenance
                    .iter()
                    .any(|p| p.table_id == table.table_id)
                    || signal_table_covered_by_inventory(table, declared_signal_names),
            ),
            TableKind::RegisterMap => (
                "register_map",
                register_records
                    .iter()
                    .any(|r| r.register_id.contains(&marker)),
            ),
            TableKind::TimingParameter => (
                "timing_parameter",
                timing_constraints
                    .iter()
                    .any(|c| c.constraint_id.contains(&marker)),
            ),
            // Encoding / FeatureMatrix / Unknown are not record-producing
            // register/signal/timing tables; not accounted in this slice.
            _ => continue,
        };
        if !covered {
            residuals.push(UnexplainedTableResidual {
                table_id: table.table_id.clone(),
                table_kind: kind_str,
                caption: table.caption_text.clone(),
            });
        }
    }
    residuals
}

/// Is every hardware signal this `SignalDescription` table carries already in the
/// declared-signal inventory? If so the table is a redundant/duplicate
/// presentation of signals captured elsewhere — covered, not a catalog miss —
/// even when it produced no direct provenance record of its own.
///
/// The signal-name column is found by CONTENT, not header position: the body
/// column with the most signal-shaped tokens. Real bus-spec tables are routinely
/// column-mangled by the PDF backend (e.g. APB's AMBA-version matrix, where docling
/// cyclically rotates the body so the `Signal` column lands last), so trusting the
/// leftmost/header column would miss the names entirely. Strict by construction:
/// returns `true` only when the table carries ≥1 signal token AND **every** token
/// in that densest column is in the inventory — a table with even one signal absent
/// from the inventory stays flagged, so a genuine miss is never hidden
/// (`WIRE-BASED-100.3a`).
fn signal_table_covered_by_inventory(
    table: &StructuredTableRecord,
    declared_signal_names: &HashSet<String>,
) -> bool {
    if declared_signal_names.is_empty() {
        return false;
    }
    let tokens = densest_signal_name_column_tokens(table);
    !tokens.is_empty() && tokens.iter().all(|t| declared_signal_names.contains(t))
}

/// The uppercased hardware-signal tokens in the body column carrying the most
/// **distinct** signal-shaped tokens (the de-facto signal-name column, robust to
/// PDF column misalignment). Returns that single column's distinct tokens.
///
/// Distinct count — not raw count — is the discriminator: a real signal-name
/// column lists one distinct name per row, whereas a property/category column
/// (e.g. APB's `Property` column repeating `Check_Type`) repeats a single value,
/// so it loses even when it ties on raw cell count.
fn densest_signal_name_column_tokens(table: &StructuredTableRecord) -> Vec<String> {
    let col_count = table
        .body_rows
        .iter()
        .map(|row| row.len())
        .max()
        .unwrap_or(0);
    let mut best: Vec<String> = Vec::new();
    for col in 0..col_count {
        let mut tokens: Vec<String> = Vec::new();
        for row in &table.body_rows {
            let Some(cell) = row.get(col) else { continue };
            let token = cell
                .text
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_uppercase();
            if is_hardware_signal_token(&token) && !is_signal_synthesis_non_signal(&token) {
                tokens.push(token);
            }
        }
        tokens.sort();
        tokens.dedup();
        if tokens.len() > best.len() {
            best = tokens;
        }
    }
    best
}

// ── Recall estimate (COMPLETENESS-RECALL-GAUGE) ────────────────────────────────

/// A capture–recapture recall estimate for one fact kind. `estimated_remaining_misses`
/// is a LOWER BOUND (capture–recapture under-estimates content; the two tiers
/// share prose input, so the estimate is optimistic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecallEstimate {
    /// Distinct facts found by the Pattern tier.
    pub pattern: usize,
    /// Distinct facts found by the Nlp tier.
    pub nlp: usize,
    /// Facts found by both tiers (the recapture overlap).
    pub overlap: usize,
    /// Distinct facts found by either tier (`|a ∪ b|`).
    pub distinct: usize,
    /// Lincoln–Petersen population estimate `N̂ = |a|·|b| / m`, rounded.
    pub estimated_total: usize,
    /// `max(0, N̂ − distinct)` — a lower bound on facts neither tier found.
    pub estimated_remaining_misses: usize,
    /// `distinct / N̂` as a percentage (an upper-ish bound, given the bias).
    pub estimated_recall_pct: u32,
    /// Chao1 (Chao 1987) lower-bound richness estimate `distinct + f1²/(2·f2)`, where
    /// `f1 = distinct − overlap` (singletons) and `f2 = overlap` (doubletons). Robust to the
    /// unequal catchability of the two tiers (which Lincoln–Petersen assumes away), so it
    /// typically sits *above* the LP estimate — the pair is reported as a range.
    pub chao_estimated_total: usize,
    /// `max(0, N̂_chao − distinct)` — the Chao lower bound on facts neither tier found.
    pub chao_estimated_remaining_misses: usize,
}

/// Capture–recapture (Lincoln–Petersen, 2-extractor) recall estimate for signal
/// constraints, over the per-extractor fact-provenance index.
///
/// Returns `None` when fewer than two tiers tagged finds, or when there is no
/// overlap (`m = 0`, where Lincoln–Petersen is undefined) — i.e. no honest
/// estimate is possible. NEVER fabricates "0 misses" for the no-data case.
pub fn recall_estimate(
    provenance: &[FactProvenanceRecord],
    fact_kind: FactKind,
) -> Option<RecallEstimate> {
    let keys = |tier: ExtractorTier| -> HashSet<&str> {
        provenance
            .iter()
            .filter(|p| p.producer == tier && p.fact_kind == fact_kind)
            .map(|p| p.canonical_key.as_str())
            .collect()
    };
    let a = keys(ExtractorTier::Pattern);
    let b = keys(ExtractorTier::Nlp);
    if a.is_empty() || b.is_empty() {
        return None; // need both independent tiers
    }
    let overlap = a.intersection(&b).count();
    if overlap == 0 {
        return None; // Lincoln–Petersen undefined with no recapture
    }
    let distinct = a.union(&b).count();
    // Lincoln–Petersen N̂ = |a|·|b| / m; ≥ distinct by construction (guarded).
    let n_hat = ((a.len() * b.len()) as f64 / overlap as f64).round() as usize;
    let estimated_total = n_hat.max(distinct);
    // Chao1 lower bound (Chao 1987), robust to unequal catchability: in the 2-source
    // incidence case the singletons f1 = facts seen by exactly one tier (distinct − overlap)
    // and the doubletons f2 = facts seen by both (overlap). f2 > 0 is guaranteed above, so the
    // simple form `distinct + f1²/(2·f2)` applies (no f2=0 bias-correction needed here).
    let f1 = distinct - overlap;
    let f2 = overlap;
    let chao = (distinct as f64 + (f1 * f1) as f64 / (2.0 * f2 as f64)).round() as usize;
    let chao_estimated_total = chao.max(distinct);
    Some(RecallEstimate {
        pattern: a.len(),
        nlp: b.len(),
        overlap,
        distinct,
        estimated_total,
        estimated_remaining_misses: estimated_total.saturating_sub(distinct),
        estimated_recall_pct: ((distinct as f64 / estimated_total as f64) * 100.0).round() as u32,
        chao_estimated_total,
        chao_estimated_remaining_misses: chao_estimated_total.saturating_sub(distinct),
    })
}

/// Recall estimate for signal constraints — see [`recall_estimate`].
pub fn signal_constraint_recall_estimate(
    provenance: &[FactProvenanceRecord],
) -> Option<RecallEstimate> {
    recall_estimate(provenance, FactKind::SignalConstraint)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::{AutomationConfidence, RegisterFieldRecord, RegisterRecord};

    fn field(name: &str, high: Option<u32>, low: Option<u32>) -> RegisterFieldRecord {
        RegisterFieldRecord {
            field_name: name.to_string(),
            bits_high: high,
            bits_low: low,
            access_type: None,
            reset_value: None,
            description: None,
        }
    }

    fn register(name: &str, fields: Vec<RegisterFieldRecord>) -> RegisterRecord {
        RegisterRecord {
            register_id: format!("reg_{name}"),
            register_name: name.to_string(),
            offset_address: None,
            fields,
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn contiguous_fields_are_clean() {
        // [3:0] + [7:4] tile [0,7] exactly: no overlap, no interior gap.
        let regs = vec![register(
            "CTRL",
            vec![field("LO", Some(3), Some(0)), field("HI", Some(7), Some(4))],
        )];
        assert!(register_tiling_residuals(&regs).is_empty());
    }

    #[test]
    fn overlapping_fields_are_flagged() {
        // [3:0] and [5:2] both claim bits 2 and 3.
        let regs = vec![register(
            "STATUS",
            vec![field("A", Some(3), Some(0)), field("B", Some(5), Some(2))],
        )];
        let r = register_tiling_residuals(&regs);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, RegisterTilingKind::Overlap);
        assert_eq!(r[0].register_name, "STATUS");
        assert_eq!(r[0].detail, "bits [3:2]");
    }

    #[test]
    fn interior_gap_is_flagged_but_top_gap_is_not() {
        // [1:0] + [7:4]: bits 2,3 are an INTERIOR gap (between documented fields).
        // Bits above 7 are never flagged (no declared width).
        let regs = vec![register(
            "CFG",
            vec![
                field("LOW", Some(1), Some(0)),
                field("HIGH", Some(7), Some(4)),
            ],
        )];
        let r = register_tiling_residuals(&regs);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, RegisterTilingKind::InteriorGap);
        assert_eq!(r[0].detail, "bits [3:2]");
    }

    #[test]
    fn single_field_register_is_skipped() {
        let regs = vec![register("ONE", vec![field("ALL", Some(31), Some(0))])];
        assert!(register_tiling_residuals(&regs).is_empty());
    }

    #[test]
    fn fields_without_bit_bounds_are_skipped() {
        let regs = vec![register(
            "UNBOUNDED",
            vec![field("X", None, None), field("Y", None, Some(0))],
        )];
        assert!(register_tiling_residuals(&regs).is_empty());
    }

    #[test]
    fn reversed_bounds_are_normalized() {
        // A field given as low=3, high=0 must normalize to [3:0], not panic.
        let regs = vec![register(
            "REV",
            vec![field("A", Some(0), Some(3)), field("B", Some(7), Some(4))],
        )];
        assert!(register_tiling_residuals(&regs).is_empty());
    }

    #[test]
    fn single_bit_gap_formats_as_bit() {
        // [0:0] + [2:2]: bit 1 is a single-bit interior gap.
        let regs = vec![register(
            "SB",
            vec![field("A", Some(0), Some(0)), field("C", Some(2), Some(2))],
        )];
        let r = register_tiling_residuals(&regs);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].kind, RegisterTilingKind::InteriorGap);
        assert_eq!(r[0].detail, "bit [1]");
    }

    // ── region accounting ──────────────────────────────────────────────────

    use crate::ir::source::{StructuredTableRecord, TableKind};

    fn table(table_id: &str, kind: TableKind) -> StructuredTableRecord {
        StructuredTableRecord {
            table_id: table_id.to_string(),
            asset_id: format!("asset_{table_id}"),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: kind,
            header_rows: vec![],
            body_rows: vec![],
            row_count: 0,
            col_count: 0,
        }
    }
    fn prov(signal: &str, table_id: &str) -> TableSignalDeclarationProvenanceRecord {
        TableSignalDeclarationProvenanceRecord {
            statement_id: format!("stmt_{signal}"),
            signal_name: signal.to_string(),
            table_id: table_id.to_string(),
        }
    }
    fn reg_with_id(id: &str) -> RegisterRecord {
        RegisterRecord {
            register_id: id.to_string(),
            register_name: id.to_string(),
            offset_address: None,
            fields: vec![],
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }
    fn timing_with_id(id: &str) -> TimingConstraintRecord {
        TimingConstraintRecord {
            constraint_id: id.to_string(),
            parameter_name: id.to_string(),
            min_value: None,
            typ_value: None,
            max_value: None,
            unit: None,
            description: None,
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }
    fn cell(text: &str) -> crate::ir::source::StructuredTableCellRecord {
        crate::ir::source::StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header: false,
        }
    }
    /// A SignalDescription table with `body` rows (each row a slice of cell texts)
    /// and no provenance of its own — used to exercise the covered-by-inventory path.
    fn signal_table_with_body(table_id: &str, body: &[&[&str]]) -> StructuredTableRecord {
        let body_rows: Vec<Vec<_>> = body
            .iter()
            .map(|row| row.iter().map(|t| cell(t)).collect())
            .collect();
        let col_count = body_rows.iter().map(|r| r.len()).max().unwrap_or(0) as u32;
        StructuredTableRecord {
            table_id: table_id.to_string(),
            asset_id: format!("asset_{table_id}"),
            page_id: None,
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![],
            row_count: body_rows.len() as u32,
            col_count,
            body_rows,
        }
    }
    fn inventory(names: &[&str]) -> HashSet<String> {
        names.iter().map(|n| n.to_string()).collect()
    }

    #[test]
    fn signal_table_with_provenance_is_covered() {
        let tables = vec![table("table_0005", TableKind::SignalDescription)];
        let provenance = vec![prov("HADDR", "table_0005")];
        let r = unexplained_intent_bearing_tables(&tables, &provenance, &[], &[], &HashSet::new());
        assert!(r.is_empty());
    }

    #[test]
    fn signal_table_without_provenance_is_unexplained() {
        let tables = vec![table("table_0009", TableKind::SignalDescription)];
        let r = unexplained_intent_bearing_tables(&tables, &[], &[], &[], &HashSet::new());
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].table_id, "table_0009");
        assert_eq!(r[0].table_kind, "signal_description");
    }

    #[test]
    fn signal_table_covered_by_inventory_when_all_signals_declared() {
        // A duplicate-content signal table that produced NO provenance record, yet
        // every signal it carries is already in the declared inventory — covered,
        // not a catalog miss (WIRE-BASED-100.3a). The name column is the LAST one
        // (PDF column rotation): the detector finds it by content density, not header.
        let table = signal_table_with_body(
            "table_0016",
            &[
                &["1", "-", "PCLK"],
                &["ADDR_WIDTH", "-", "PADDR"],
                &["DATA_WIDTH", "-", "PWDATA"],
            ],
        );
        let inv = inventory(&["PCLK", "PADDR", "PWDATA", "PREADY"]);
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &inv);
        assert!(r.is_empty(), "all signals are in the inventory → covered");
    }

    #[test]
    fn property_column_does_not_outrank_signal_column_on_tie() {
        // Real APB table_0017 shape: a `Property` column repeats "Check_Type" on every
        // row, tying the (last) signal column on raw cell count. Distinct-count must
        // pick the signal column (4 distinct names vs 1), so the table is covered.
        let table = signal_table_with_body(
            "table_0017",
            &[
                &["ADDR_WIDTH/8", "Check_Type", "C", "PADDRCHK"],
                &["1", "Check_Type", "C", "PCTRLCHK"],
                &["1", "Check_Type", "C", "PSELXCHK"],
                &["1", "Check_Type", "C", "PENABLECHK"],
            ],
        );
        let inv = inventory(&["PADDRCHK", "PCTRLCHK", "PSELXCHK", "PENABLECHK"]);
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &inv);
        assert!(
            r.is_empty(),
            "the distinct signal column must beat the repeated Property column"
        );
    }

    #[test]
    fn signal_table_with_unknown_signal_stays_unexplained() {
        // Strict: even ONE signal absent from the inventory keeps the table flagged,
        // so a genuine catalog miss is never hidden by the covered-by-inventory path.
        let table = signal_table_with_body(
            "table_0042",
            &[&["1", "PCLK"], &["1", "PADDR"], &["1", "PNEWSIG"]],
        );
        let inv = inventory(&["PCLK", "PADDR"]); // PNEWSIG missing
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &inv);
        assert_eq!(
            r.len(),
            1,
            "an uncovered signal must keep the table flagged"
        );
        assert_eq!(r[0].table_id, "table_0042");
    }

    #[test]
    fn register_table_covered_by_embedded_id() {
        let tables = vec![table("table_0026", TableKind::RegisterMap)];
        let regs = vec![reg_with_id("reg_table_0026_000")];
        let r = unexplained_intent_bearing_tables(&tables, &[], &regs, &[], &HashSet::new());
        assert!(r.is_empty());
    }

    #[test]
    fn register_table_marker_does_not_falsely_match_adjacent_id() {
        // table_0002 must NOT be considered covered by reg_table_0020_000.
        let tables = vec![table("table_0002", TableKind::RegisterMap)];
        let regs = vec![reg_with_id("reg_table_0020_000")];
        let r = unexplained_intent_bearing_tables(&tables, &[], &regs, &[], &HashSet::new());
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].table_id, "table_0002");
    }

    #[test]
    fn timing_table_covered_by_embedded_id() {
        let tables = vec![table("table_0022", TableKind::TimingParameter)];
        let timing = vec![timing_with_id("timing_table_0022_000")];
        let r = unexplained_intent_bearing_tables(&tables, &[], &[], &timing, &HashSet::new());
        assert!(r.is_empty());
    }

    #[test]
    fn non_intent_table_kinds_are_skipped() {
        // Encoding / FeatureMatrix / Unknown are not accounted here, even with no records.
        let tables = vec![
            table("table_0001", TableKind::Encoding),
            table("table_0002", TableKind::FeatureMatrix),
            table("table_0003", TableKind::Unknown),
        ];
        let r = unexplained_intent_bearing_tables(&tables, &[], &[], &[], &HashSet::new());
        assert!(r.is_empty());
    }

    // ── recall estimate ────────────────────────────────────────────────────

    fn fact_prov(tier: ExtractorTier, key: &str) -> FactProvenanceRecord {
        FactProvenanceRecord {
            producer: tier,
            fact_kind: FactKind::SignalConstraint,
            canonical_key: key.to_string(),
        }
    }

    #[test]
    fn recall_estimate_lincoln_petersen_from_overlap() {
        // Pattern {A,B,C,D}, Nlp {C,D,E,F}: overlap 2, distinct 6, N̂ = 4·4/2 = 8,
        // remaining misses = 8-6 = 2, recall = 6/8 = 75%.
        let prov_v = vec![
            fact_prov(ExtractorTier::Pattern, "A"),
            fact_prov(ExtractorTier::Pattern, "B"),
            fact_prov(ExtractorTier::Pattern, "C"),
            fact_prov(ExtractorTier::Pattern, "D"),
            fact_prov(ExtractorTier::Nlp, "C"),
            fact_prov(ExtractorTier::Nlp, "D"),
            fact_prov(ExtractorTier::Nlp, "E"),
            fact_prov(ExtractorTier::Nlp, "F"),
        ];
        let est = signal_constraint_recall_estimate(&prov_v).expect("estimate");
        assert_eq!(est.pattern, 4);
        assert_eq!(est.nlp, 4);
        assert_eq!(est.overlap, 2);
        assert_eq!(est.distinct, 6);
        assert_eq!(est.estimated_total, 8);
        assert_eq!(est.estimated_remaining_misses, 2);
        assert_eq!(est.estimated_recall_pct, 75);
        // Chao1: singletons f1 = distinct−overlap = 4 (A,B,E,F), doubletons f2 = overlap = 2
        // (C,D) → N̂_chao = 6 + 4²/(2·2) = 10; remaining misses 10−6 = 4. Chao sits above LP
        // (heterogeneity-aware), so the pair reports a range: total 8–10, misses 2–4.
        assert_eq!(est.chao_estimated_total, 10);
        assert_eq!(est.chao_estimated_remaining_misses, 4);
        assert!(
            est.chao_estimated_total >= est.distinct,
            "Chao is a lower bound on richness (≥ observed)"
        );
    }

    #[test]
    fn recall_estimate_chao_matches_observed_on_full_overlap() {
        // Both tiers found exactly {A,B,C}: zero singletons (f1=0) → Chao adds nothing, so
        // the Chao estimate equals the observed count and predicts no remaining misses.
        let prov = vec![
            fact_prov(ExtractorTier::Pattern, "A"),
            fact_prov(ExtractorTier::Pattern, "B"),
            fact_prov(ExtractorTier::Pattern, "C"),
            fact_prov(ExtractorTier::Nlp, "A"),
            fact_prov(ExtractorTier::Nlp, "B"),
            fact_prov(ExtractorTier::Nlp, "C"),
        ];
        let est = signal_constraint_recall_estimate(&prov).expect("estimate");
        assert_eq!(est.distinct, 3);
        assert_eq!(est.overlap, 3);
        assert_eq!(
            est.chao_estimated_total, 3,
            "no singletons (f1=0) → Chao adds nothing"
        );
        assert_eq!(est.chao_estimated_remaining_misses, 0);
    }

    #[test]
    fn recall_estimate_none_without_two_tiers() {
        // Only Pattern → can't recapture.
        let only_pattern = vec![
            fact_prov(ExtractorTier::Pattern, "A"),
            fact_prov(ExtractorTier::Pattern, "B"),
        ];
        assert!(signal_constraint_recall_estimate(&only_pattern).is_none());
    }

    #[test]
    fn recall_estimate_none_without_overlap() {
        // Both tiers but no shared fact → Lincoln–Petersen undefined.
        let no_overlap = vec![
            fact_prov(ExtractorTier::Pattern, "A"),
            fact_prov(ExtractorTier::Nlp, "B"),
        ];
        assert!(signal_constraint_recall_estimate(&no_overlap).is_none());
    }

    #[test]
    fn recall_estimate_generalizes_to_relations() {
        // The same Lincoln–Petersen math must apply to actor-signal relations
        // when keyed by FactKind::ActorSignalRelation.
        let rel = |tier, key: &str| FactProvenanceRecord {
            producer: tier,
            fact_kind: FactKind::ActorSignalRelation,
            canonical_key: key.to_string(),
        };
        // Pattern captured {A,B}, Nlp captured {B,C}; overlap = {B} = 1.
        // N̂ = 2*2/1 = 4 distinct edges; observed = 3 → 1 remaining miss; recall = 75%.
        let prov = vec![
            rel(ExtractorTier::Pattern, "M|Drives|A"),
            rel(ExtractorTier::Pattern, "M|Drives|B"),
            rel(ExtractorTier::Nlp, "M|Drives|B"),
            rel(ExtractorTier::Nlp, "M|Drives|C"),
        ];
        let est = recall_estimate(&prov, FactKind::ActorSignalRelation).expect("estimate");
        assert_eq!(est.overlap, 1);
        assert_eq!(est.estimated_total, 4);
        assert_eq!(est.estimated_remaining_misses, 1);
        assert_eq!(est.estimated_recall_pct, 75);
        // Cross-kind isolation: a relation-only provenance yields no signal-constraint estimate.
        assert!(recall_estimate(&prov, FactKind::SignalConstraint).is_none());
    }
}
