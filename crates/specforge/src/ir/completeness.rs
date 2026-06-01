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

use crate::ir::evidence::TableSignalDeclarationProvenanceRecord;
use crate::ir::source::{StructuredTableRecord, TableKind, TimingConstraintRecord};

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
///   table's `table_id` directly;
/// - RegisterMap → a `register_record` whose `register_id` embeds the `table_id`
///   (`reg_table_0026_000` ⊃ `table_0026`);
/// - TimingParameter → a `timing_constraint` whose `constraint_id` embeds it
///   (`timing_table_0022_000`).
///
/// The `"{table_id}_"` marker (with the trailing underscore) is used for the
/// embedded-id check so fixed-width ids like `table_0002` cannot spuriously match
/// `table_0020`. Other table kinds are not accounted here.
pub fn unexplained_intent_bearing_tables(
    tables: &[StructuredTableRecord],
    signal_provenance: &[TableSignalDeclarationProvenanceRecord],
    register_records: &[RegisterRecord],
    timing_constraints: &[TimingConstraintRecord],
) -> Vec<UnexplainedTableResidual> {
    let mut residuals = Vec::new();
    for table in tables {
        let marker = format!("{}_", table.table_id);
        let (kind_str, covered) = match table.table_kind {
            TableKind::SignalDescription => (
                "signal_description",
                signal_provenance
                    .iter()
                    .any(|p| p.table_id == table.table_id),
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

    #[test]
    fn signal_table_with_provenance_is_covered() {
        let tables = vec![table("table_0005", TableKind::SignalDescription)];
        let provenance = vec![prov("HADDR", "table_0005")];
        let r = unexplained_intent_bearing_tables(&tables, &provenance, &[], &[]);
        assert!(r.is_empty());
    }

    #[test]
    fn signal_table_without_provenance_is_unexplained() {
        let tables = vec![table("table_0009", TableKind::SignalDescription)];
        let r = unexplained_intent_bearing_tables(&tables, &[], &[], &[]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].table_id, "table_0009");
        assert_eq!(r[0].table_kind, "signal_description");
    }

    #[test]
    fn register_table_covered_by_embedded_id() {
        let tables = vec![table("table_0026", TableKind::RegisterMap)];
        let regs = vec![reg_with_id("reg_table_0026_000")];
        let r = unexplained_intent_bearing_tables(&tables, &[], &regs, &[]);
        assert!(r.is_empty());
    }

    #[test]
    fn register_table_marker_does_not_falsely_match_adjacent_id() {
        // table_0002 must NOT be considered covered by reg_table_0020_000.
        let tables = vec![table("table_0002", TableKind::RegisterMap)];
        let regs = vec![reg_with_id("reg_table_0020_000")];
        let r = unexplained_intent_bearing_tables(&tables, &[], &regs, &[]);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].table_id, "table_0002");
    }

    #[test]
    fn timing_table_covered_by_embedded_id() {
        let tables = vec![table("table_0022", TableKind::TimingParameter)];
        let timing = vec![timing_with_id("timing_table_0022_000")];
        let r = unexplained_intent_bearing_tables(&tables, &[], &[], &timing);
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
        let r = unexplained_intent_bearing_tables(&tables, &[], &[], &[]);
        assert!(r.is_empty());
    }
}
