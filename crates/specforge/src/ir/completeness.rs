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

/// Names of registers whose mandatory bit-WIDTH is unresolved from this document.
///
/// A register is physical bit-storage, so a width ALWAYS exists — an absent `size_bits` is therefore a
/// completeness GAP (the width is parametric, e.g. XLEN/DXLEN, or defined in another spec), not an
/// optional/"don't care" attribute. Surfacing these is a register-specific completeness residual: every
/// returned register is a known miss whose width must be sourced (parameter, cross-document, or design).
pub fn registers_with_unresolved_width(registers: &[RegisterRecord]) -> Vec<String> {
    registers
        .iter()
        .filter(|r| r.size_bits.is_none())
        .map(|r| r.register_name.clone())
        .collect()
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
    ExtractedStatement, ExtractorTier, FactKind, FactProvenanceRecord, StatementClass,
    TableSignalDeclarationProvenanceRecord, is_hardware_signal_token,
    is_signal_synthesis_non_signal,
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

/// Statement ids of `NormativeStatement`-classed statements whose obligation was
/// NOT captured by any typed record — the genuine "partially-structured normative"
/// residuals.
///
/// A statement can end at class `NormativeStatement` yet still have its obligation
/// captured by a downstream extractor (e.g. the dynamic constraint path emits a
/// `signal_constraint` from "the Requester must drive PSTRB LOW" but leaves the
/// statement's class normative). Counting such a captured statement as a completeness
/// miss is a false positive — `captured_statement_ids` (the union of
/// `supporting_statement_ids` over the typed records) removes them. Strict:
/// a normative statement no typed record cites stays a residual, so a genuine gap is
/// never hidden (`WIRE-BASED-100.3b`; mirrors the duplicate-table coverage fix `.3a`).
pub fn uncaptured_normative_statement_ids<'a>(
    statements: &'a [ExtractedStatement],
    captured_statement_ids: &HashSet<&str>,
) -> Vec<&'a str> {
    statements
        .iter()
        .filter(|stmt| matches!(stmt.class, StatementClass::NormativeStatement))
        .map(|stmt| stmt.statement_id.as_str())
        .filter(|id| !captured_statement_ids.contains(id))
        .collect()
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

// ── Document class (PDF-VARIANT-DIGESTION.5a) ──────────────────────────────────

/// The structural class of a chip-spec document, inferred ONLY from WHICH typed
/// intent surfaces its staged extraction produced — never from a chip/vendor/
/// protocol name (ADR 0006). The class routes class-appropriate reporting and,
/// crucially, lets a document with little structured design-intent (a programming
/// guide, an ISA narrative, an image-only datasheet) be reported HONESTLY as such
/// instead of looking like a silent extraction failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentClass {
    /// Behavioral spec: signal constraints / conditional rules / an explicit FSM /
    /// a serial frame — the document encodes how signals behave over time.
    Protocol,
    /// Register/CSR-dominated: many register records are the document's primary
    /// design-intent (a register map), with no dominant behavioral surface.
    Register,
    /// Signal-interface spec: a signal inventory + actor-signal connectivity, but
    /// no behavioral obligations and not register-dominated.
    Interface,
    /// Low structured design-intent: little or no typed intent of any kind was
    /// recovered (guide / narrative / image-heavy). An HONEST floor — reported as
    /// such, never as a silent miss.
    Guide,
}

impl DocumentClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Protocol => "protocol",
            Self::Register => "register",
            Self::Interface => "interface",
            Self::Guide => "guide",
        }
    }
}

/// The structural census the document-class decision reads — one count per typed
/// intent surface the staged extraction produces. Every count is observational
/// (read off already-built IR, no fabrication); the classifier is a pure function
/// of this census.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DocumentClassCensus {
    pub registers: usize,
    pub register_fields: usize,
    pub declared_signals: usize,
    pub actor_signal_relations: usize,
    pub signal_constraints: usize,
    pub conditional_rules: usize,
    pub protocol_actors: usize,
    pub fsm_states: usize,
    pub serial_frame_fields: usize,
    pub visual_evidence: usize,
}

/// One document-class decision: the class, the census that drove it, and a
/// human-readable rationale (the latter is what `validate` surfaces so a reader
/// sees WHY a doc was classed — especially that a `Guide` is an honest low-intent
/// call, not a silent miss).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentClassification {
    pub class: DocumentClass,
    pub rationale: String,
    pub census: DocumentClassCensus,
}

/// Minimum register records before a document is "register-dominated". One stray
/// register is not a register spec; a couple is the smallest honest threshold.
const DOC_CLASS_REGISTER_MIN: usize = 2;
/// Minimum signal constraints before a document counts as "behavioral" (protocol).
/// On real corpus data behavioral specs carry ≥11 (APB 14, I2C 11, CHI 13) while
/// guides carry 0 — a clean gap, so a small floor of 3 safely separates them
/// without crediting one or two incidental constraints.
const DOC_CLASS_BEHAVIORAL_MIN: usize = 3;
/// Minimum signal-inventory + connectivity presence before a document is an
/// "interface" rather than a guide. A couple of stray signal mentions is not an
/// interface catalog.
const DOC_CLASS_INTERFACE_MIN: usize = 3;

/// Classify a document by the dominant typed intent surface its extraction
/// produced. Pure, deterministic, and agnostic: the only constants are small
/// generic structural floors (no chip/vendor/protocol vocabulary, ADR 0006).
///
/// Only LOW-NOISE surfaces drive the decision: register records, signal
/// constraints, actor-signal relations, the declared-signal inventory, and the
/// FSM / serial-frame surfaces. `conditional_rules` is deliberately EXCLUDED — on
/// real corpus data it is over-produced narrative "if X then Y" prose that fires
/// even on programming guides (GIC overview guide: 12; SMMU software guide: 7) and
/// register docs (RISC-V Debug: 78; NVMe: 250), so it does not discriminate class.
/// It stays in the census for transparency only.
///
/// Decision order (first match wins):
/// 1. **Register** — register-dominated: at least `DOC_CLASS_REGISTER_MIN`
///    register records, and they outnumber BOTH the connectivity surface
///    (relations + declared signals) AND the behavioral surface (signal
///    constraints). So RISC-V Debug (60 regs ≥ 30 connectivity) / NVMe (46 ≥ 26
///    constraints) land here, while AHB/AXI/SWD (which carry registers but are
///    connectivity/behavior-dominant) fall through.
/// 2. **Protocol** — `signal_constraints ≥ DOC_CLASS_BEHAVIORAL_MIN`, or there is
///    an FSM / a serial frame: the document encodes signal behavior over time.
/// 3. **Interface** — a signal inventory + actor-signal connectivity at the floor,
///    with no behavioral obligations and not register-dominated.
/// 4. **Guide** — the HONEST floor: no reliable typed intent surface is present
///    (guide / narrative / image-heavy), reported as such instead of as a silent
///    0-yield miss.
pub fn classify_document(census: DocumentClassCensus) -> DocumentClassification {
    let interface_surface = census.actor_signal_relations + census.declared_signals;
    let has_state_behavior = census.fsm_states > 0 || census.serial_frame_fields > 0;

    let class = if census.registers >= DOC_CLASS_REGISTER_MIN
        && census.registers >= interface_surface
        && census.registers >= census.signal_constraints
    {
        DocumentClass::Register
    } else if census.signal_constraints >= DOC_CLASS_BEHAVIORAL_MIN || has_state_behavior {
        DocumentClass::Protocol
    } else if interface_surface >= DOC_CLASS_INTERFACE_MIN {
        DocumentClass::Interface
    } else {
        DocumentClass::Guide
    };

    let rationale = match class {
        DocumentClass::Guide => {
            let narrative = if census.conditional_rules > 0 {
                format!(
                    ", {} narrative conditional rules (over-produced, not class-determining)",
                    census.conditional_rules
                )
            } else {
                String::new()
            };
            let visual = if census.visual_evidence > 0 {
                format!(", {} visual assets", census.visual_evidence)
            } else {
                String::new()
            };
            format!(
                "low structured design-intent ({} signals, {} registers, {} relations, {} signal constraints{narrative}{visual}) — reported honestly, not a silent miss",
                census.declared_signals,
                census.registers,
                census.actor_signal_relations,
                census.signal_constraints,
            )
        }
        DocumentClass::Register => format!(
            "register-dominated ({} registers / {} fields; {} signal constraints, {interface_surface} connectivity edges)",
            census.registers, census.register_fields, census.signal_constraints,
        ),
        DocumentClass::Protocol => format!(
            "behavioral protocol ({} signal constraints, {} FSM states, {} frame fields, {} actors, {} relations)",
            census.signal_constraints,
            census.fsm_states,
            census.serial_frame_fields,
            census.protocol_actors,
            census.actor_signal_relations,
        ),
        DocumentClass::Interface => format!(
            "signal interface ({} signals, {} actor-signal relations; no behavioral obligations)",
            census.declared_signals, census.actor_signal_relations,
        ),
    };

    DocumentClassification {
        class,
        rationale,
        census,
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
            bit_width: match (high, low) {
                (Some(h), Some(l)) if h >= l => Some(h - l + 1),
                _ => None,
            },
            access_type: None,
            reset_value: None,
            description: None,
            enumerated_values: vec![],
        }
    }

    fn register(name: &str, fields: Vec<RegisterFieldRecord>) -> RegisterRecord {
        RegisterRecord {
            register_id: format!("reg_{name}"),
            register_name: name.to_string(),
            offset_address: None,
            size_bits: None,
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
    fn unresolved_register_width_is_flagged() {
        // A register is physical bit-storage → width is mandatory; an unresolved width is a gap.
        let mut sized = register("CTRL", vec![field("EN", Some(0), Some(0))]);
        sized.size_bits = Some(32);
        let no_width = register("STATUS", vec![field("BUSY", Some(0), Some(0))]); // size_bits None
        let flagged = registers_with_unresolved_width(&[sized, no_width]);
        assert_eq!(flagged, vec!["STATUS".to_string()]);
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
            size_bits: None,
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

    // ── prose residuals (captured-normative coverage) ───────────────────────

    fn normative_stmt(id: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::NormativeStatement,
            modality: crate::ir::evidence::EvidenceModality::Text,
            text: format!("{id} text"),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn captured_normative_statement_is_not_a_residual() {
        // statement_0223 is class NormativeStatement but a typed constraint
        // (dyn_sigcon_0015) cites it → captured, not a residual (WIRE-BASED-100.3b).
        let stmts = vec![
            normative_stmt("statement_0223"),
            normative_stmt("statement_0370"),
        ];
        let captured: HashSet<&str> = ["statement_0223"].into_iter().collect();
        let residuals = uncaptured_normative_statement_ids(&stmts, &captured);
        assert_eq!(residuals, vec!["statement_0370"]); // only the uncaptured one remains
    }

    #[test]
    fn uncaptured_normative_statement_stays_a_residual() {
        // No typed record cites either → both are genuine residuals (no gap hidden).
        let stmts = vec![
            normative_stmt("statement_0223"),
            normative_stmt("statement_0370"),
        ];
        let residuals = uncaptured_normative_statement_ids(&stmts, &HashSet::new());
        assert_eq!(residuals.len(), 2);
    }

    #[test]
    fn non_normative_statements_are_never_residuals() {
        // A SourceFact is not a normative statement, so it is never counted here even
        // when uncaptured.
        let mut s = normative_stmt("statement_0001");
        s.class = StatementClass::SourceFact;
        let stmts = [s];
        let residuals = uncaptured_normative_statement_ids(&stmts, &HashSet::new());
        assert!(residuals.is_empty());
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

    // ── document class (PDF-VARIANT-DIGESTION.5a) ───────────────────────────

    /// A census with everything zero except the named overrides — keeps each
    /// classifier test focused on the one surface it exercises.
    fn census(overrides: impl FnOnce(&mut DocumentClassCensus)) -> DocumentClassCensus {
        let mut c = DocumentClassCensus::default();
        overrides(&mut c);
        c
    }

    #[test]
    fn zero_yield_document_is_an_honest_guide() {
        // The 8 zero-yield corpus docs (guides / ISA narratives / image-heavy):
        // no typed intent of any kind → Guide, NOT a silent miss.
        let r = classify_document(census(|c| c.visual_evidence = 40));
        assert_eq!(r.class, DocumentClass::Guide);
        assert!(
            r.rationale.contains("low structured design-intent")
                && r.rationale.contains("not a silent miss"),
            "a guide must be reported honestly, not as a failure: {}",
            r.rationale
        );
        // image-heavy guides note their visual assets so "guide vs image-heavy" is visible.
        assert!(r.rationale.contains("40 visual assets"));
    }

    #[test]
    fn a_couple_stray_signals_stay_a_guide() {
        // Two stray signal mentions and nothing else is below the interface floor.
        let r = classify_document(census(|c| c.declared_signals = 2));
        assert_eq!(r.class, DocumentClass::Guide);
    }

    #[test]
    fn register_dominated_document_is_register() {
        // NVMe / RISC-V Debug shape: many register records, no behavioral surface.
        let r = classify_document(census(|c| {
            c.registers = 44;
            c.register_fields = 199;
        }));
        assert_eq!(r.class, DocumentClass::Register);
        assert!(r.rationale.contains("44 registers / 199 fields"));
    }

    #[test]
    fn a_few_incidental_constraints_do_not_flip_a_register_doc() {
        // 5 registers with 2 incidental constraints: registers ≥ behavioral → Register.
        let r = classify_document(census(|c| {
            c.registers = 5;
            c.signal_constraints = 2;
        }));
        assert_eq!(r.class, DocumentClass::Register);
    }

    #[test]
    fn registers_outnumbered_by_connectivity_and_behavior_are_protocol() {
        // AHB shape (real data): 21 register records, but 66 relations + 42 signals
        // of connectivity and 15 signal constraints dominate → Protocol, not Register.
        let r = classify_document(census(|c| {
            c.registers = 21;
            c.signal_constraints = 15;
            c.conditional_rules = 39;
            c.actor_signal_relations = 66;
            c.declared_signals = 42;
        }));
        assert_eq!(r.class, DocumentClass::Protocol);
    }

    #[test]
    fn registers_with_an_fsm_are_protocol_not_register() {
        // SWD shape (real data): 33 register records, but a serial frame + an FSM
        // and 100 connectivity edges dominate → Protocol via the state surface.
        let r = classify_document(census(|c| {
            c.registers = 33;
            c.signal_constraints = 2;
            c.actor_signal_relations = 80;
            c.declared_signals = 20;
            c.fsm_states = 13;
            c.serial_frame_fields = 11;
        }));
        assert_eq!(r.class, DocumentClass::Protocol);
    }

    #[test]
    fn over_produced_conditional_rules_do_not_make_a_guide_a_protocol() {
        // THE real-data finding: `conditional_rules` is over-produced narrative prose
        // that fires even on programming guides (GIC overview guide: 12 conditional
        // rules, zero of every reliable surface). It must NOT flip the class — the doc
        // stays an honest Guide, and the rationale explains the noise.
        let r = classify_document(census(|c| {
            c.conditional_rules = 12;
            c.visual_evidence = 44;
        }));
        assert_eq!(r.class, DocumentClass::Guide);
        assert!(
            r.rationale.contains("narrative conditional rules")
                && r.rationale.contains("not class-determining"),
            "the guide rationale must explain the conditional-rule noise: {}",
            r.rationale
        );
    }

    #[test]
    fn behavioral_obligations_make_a_protocol() {
        // APB / AHB / AXI / I2C shape: signal constraints (+ relations/actors).
        let r = classify_document(census(|c| {
            c.declared_signals = 30;
            c.actor_signal_relations = 69;
            c.signal_constraints = 16;
            c.protocol_actors = 2;
        }));
        assert_eq!(r.class, DocumentClass::Protocol);
    }

    #[test]
    fn an_fsm_or_frame_makes_a_protocol_without_constraints() {
        // SWD shape: a serial frame + an FSM, no signal constraints — still Protocol.
        let r = classify_document(census(|c| {
            c.fsm_states = 9;
            c.serial_frame_fields = 7;
        }));
        assert_eq!(r.class, DocumentClass::Protocol);
    }

    #[test]
    fn signals_and_relations_without_behavior_are_an_interface() {
        // Avalon shape: a signal inventory + dense connectivity, zero behavioral
        // obligations, no registers → Interface (not Protocol, not a Guide).
        let r = classify_document(census(|c| {
            c.declared_signals = 34;
            c.actor_signal_relations = 179;
        }));
        assert_eq!(r.class, DocumentClass::Interface);
        assert!(r.rationale.contains("no behavioral obligations"));
    }
}
