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
}
