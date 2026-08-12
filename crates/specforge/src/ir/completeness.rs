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
    // PDF-VARIANT-DIGESTION.12a — an `unknown`-kind continuation fragment is accounted under its
    // captioned chain head's kind (caption parent-reference + exact header signature), so a
    // page-split intent-bearing table cannot hide from this accounting just because Docling
    // dropped a fragment's classification.
    let inherited_heads = crate::ir::evidence::continuation_inherited_table_heads(tables);
    for table in tables {
        let marker = format!("{}_", table.table_id);
        let effective_kind = inherited_heads
            .get(&table.table_id)
            .map(|&head_index| tables[head_index].table_kind)
            .unwrap_or(table.table_kind);
        let (kind_str, covered) = match effective_kind {
            TableKind::SignalDescription => (
                "signal_description",
                signal_provenance
                    .iter()
                    .any(|p| p.table_id == table.table_id)
                    || signal_table_covered_by_inventory(table, declared_signal_names)
                    || signal_presence_capture_covers(table),
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

/// PDF-VARIANT-DIGESTION.12b — is this table a presence matrix whose rows the signal-presence
/// surface captures COMPLETELY? Runs the exact shared gate/capture definition
/// ([`crate::ir::evidence::capture_signal_presence_rows`]) at validate time, so the coverage
/// reaches promoted artifacts without any rebuild (the `.12a` precedent). STRICT by
/// construction: at least one captured row AND zero refused rows — a matrix with even one
/// fused/garbled row the capture refused stays flagged, so partial capture never hides a
/// genuine miss (`WIRE-BASED-100.3a`).
fn signal_presence_capture_covers(table: &StructuredTableRecord) -> bool {
    let capture = crate::ir::evidence::capture_signal_presence_rows(table);
    !capture.records.is_empty() && capture.refused_rows == 0
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
    // PDF-VARIANT-DIGESTION.12a — the data rows Docling trapped in `header_rows` (the shared
    // `.9.11` structural rule) carry signals too: a presence matrix whose every row-label cell is
    // header-marked has NO body rows, so without them a fully-redundant duplicate presentation
    // could never be recognized as covered. Same strictness applies — one unknown signal anywhere
    // in the densest column keeps the table flagged.
    let data_rows: Vec<&Vec<crate::ir::source::StructuredTableCellRecord>> = table
        .body_rows
        .iter()
        .chain(crate::ir::evidence::recovered_trapped_data_rows(table))
        .collect();
    let col_count = data_rows.iter().map(|row| row.len()).max().unwrap_or(0);
    let mut best: Vec<String> = Vec::new();
    for col in 0..col_count {
        let mut tokens: Vec<String> = Vec::new();
        for row in &data_rows {
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

/// What a document's OWN front-matter (title / table-of-contents / early first-chapter
/// headings) declares itself to be — a self-description signal that complements the
/// structural census (`PDF-VARIANT-DIGESTION.5c`). The owner's observation: a chip-spec
/// PDF usually states its type in plain words in the early pages. Recognized via generic
/// document-TYPE vocabulary only (guide / specification / architecture / standard / …) —
/// never a chip/vendor/protocol-instance name (ADR 0006), the same agnostic-grammar spirit
/// as the normative vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeclaredDocType {
    /// Self-declares a guide / overview / tutorial / application note / "learn the …".
    Guide,
    /// Self-declares a specification / architecture / protocol / standard / reference
    /// manual / datasheet — a real design contract document.
    Specification,
    /// No recognizable document-type word in the front-matter (or no front-matter available).
    #[default]
    Unknown,
}

impl DeclaredDocType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Guide => "guide",
            Self::Specification => "specification",
            Self::Unknown => "unknown",
        }
    }
}

/// Multi-word GUIDE phrases. Checked (and ranked) BEFORE the spec vocabulary because some
/// guide framings legitimately contain a spec word — Arm's "Learn the architecture …"
/// series and an "architecture overview" are guides, not architecture specs — so the guide
/// reading must win. All are generic doc-type phrasings, no chip/vendor names (ADR 0006).
const DECLARED_GUIDE_PHRASES: &[&str] = &[
    "learn the architecture",
    "optimization guide",
    "programming guide",
    "programmer's guide",
    "programmers guide",
    "user guide",
    "user's guide",
    "getting started",
    "application note",
    "white paper",
    "reference design",
    "quick start",
    "usage model",
];
/// Single GUIDE words (matched as whole tokens so "design guidelines" does not trip "guide").
/// Deliberately EXCLUDES "overview" / "introduction" — every specification has an
/// introduction/overview chapter, so those do not declare the document a guide.
const DECLARED_GUIDE_WORDS: &[&str] = &[
    "guide", "guides", "tutorial", "primer", "handbook", "cookbook",
];
/// Multi-word SPECIFICATION phrases.
const DECLARED_SPEC_PHRASES: &[&str] = &[
    "reference manual",
    "technical reference",
    "register map",
    "programmer's model",
    "programmers model",
    "data sheet",
    "instruction set",
];
/// Single SPECIFICATION words (whole tokens).
const DECLARED_SPEC_WORDS: &[&str] = &[
    "specification",
    "specifications",
    "architecture",
    "protocol",
    "standard",
    "datasheet",
];

/// Infer a document's self-declared type from its front-matter text (title + early
/// section headings). Pure, agnostic grammar: only generic document-type vocabulary,
/// no chip/vendor/protocol-instance names (ADR 0006). Guide framings win over spec
/// framings on overlap (e.g. "Learn the architecture", "architecture overview"), so the
/// guide vocabulary is checked first.
pub fn front_matter_doc_type_hint(front_matter: &str) -> DeclaredDocType {
    let lower = front_matter.to_ascii_lowercase();
    if DECLARED_GUIDE_PHRASES.iter().any(|p| lower.contains(p))
        || DECLARED_GUIDE_WORDS
            .iter()
            .any(|w| front_matter_has_word(&lower, w))
    {
        DeclaredDocType::Guide
    } else if DECLARED_SPEC_PHRASES.iter().any(|p| lower.contains(p))
        || DECLARED_SPEC_WORDS
            .iter()
            .any(|w| front_matter_has_word(&lower, w))
    {
        DeclaredDocType::Specification
    } else {
        DeclaredDocType::Unknown
    }
}

/// Whole-token match of `word` (already lowercase) anywhere in `lower` (already lowercase),
/// splitting on any non-alphanumeric boundary so e.g. "guidelines" does not trip "guide".
/// Shared by every front-matter vocabulary probe so the tokenization cannot drift.
fn front_matter_has_word(lower: &str, word: &str) -> bool {
    lower
        .split(|c: char| !c.is_ascii_alphanumeric())
        .any(|tok| tok == word)
}

/// Multi-word CPU-ISA framings (`DOC-INTENT-TAXONOMY.3`). Generic instruction-set /
/// privileged-architecture doc-type vocabulary only — never a chip/vendor/ISA-instance name
/// (ADR 0006), the same agnostic-grammar spirit as `DECLARED_*`. `.1` proved the ISA category
/// has no structural signature, so a front-matter self-declaration is its only positive cue.
const DECLARED_ISA_PHRASES: &[&str] = &[
    "instruction set",
    "instruction-set architecture",
    "privileged architecture",
    "unprivileged architecture",
    "privileged specification",
    "privileged spec",
];
/// Single CPU-ISA words (whole tokens).
const DECLARED_ISA_WORDS: &[&str] = &["isa"];

/// Multi-word PHYSICAL/ELECTRICAL/LINK-layer framings (`DOC-INTENT-TAXONOMY.3`). Generic
/// physical-layer doc-type vocabulary only — never a chip/vendor name (ADR 0006). `.1` proved
/// cat 5 (PHY) and cat 6 (guide) are both near-empty and structurally indistinguishable, so a
/// front-matter self-declaration is the only positive PHY cue.
const DECLARED_PHY_PHRASES: &[&str] = &[
    "physical layer",
    "physical signaling",
    "phy specification",
    "link layer",
    "link training",
    "signal integrity",
    "electrical characteristics",
    "electrical specification",
];
/// Single PHYSICAL-layer words (whole tokens). Kept deliberately specific (`phy` / `serdes` /
/// `transceiver`) to avoid tripping on a protocol document's incidental "signaling" prose.
const DECLARED_PHY_WORDS: &[&str] = &["phy", "serdes", "transceiver"];

/// Whether the document's front-matter self-declares a CPU instruction-set / privileged
/// architecture. Pure, agnostic grammar (generic ISA doc-type vocabulary, no instance names —
/// ADR 0006). Used only by the purpose recognizer (`classify_document_intent_category`); it is
/// independent of `front_matter_doc_type_hint` (an ISA volume also reads as a `Specification`
/// there, and both calls are made — the recognizer just needs the sharper ISA signal).
pub fn front_matter_declares_isa(front_matter: &str) -> bool {
    let lower = front_matter.to_ascii_lowercase();
    DECLARED_ISA_PHRASES.iter().any(|p| lower.contains(p))
        || DECLARED_ISA_WORDS
            .iter()
            .any(|w| front_matter_has_word(&lower, w))
}

/// Whether the document's front-matter self-declares a physical / electrical / link layer.
/// Pure, agnostic grammar (generic PHY doc-type vocabulary, no instance names — ADR 0006).
/// Used only by the purpose recognizer (`classify_document_intent_category`).
pub fn front_matter_declares_phy(front_matter: &str) -> bool {
    let lower = front_matter.to_ascii_lowercase();
    DECLARED_PHY_PHRASES.iter().any(|p| lower.contains(p))
        || DECLARED_PHY_WORDS
            .iter()
            .any(|w| front_matter_has_word(&lower, w))
}

/// The structural census the document-class decision reads — one count per typed
/// intent surface the staged extraction produces, plus the document's self-declared
/// type from its front-matter (`.5c`). Every count is observational (read off
/// already-built IR, no fabrication); the classifier is a pure function of this census.
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
    /// The document's self-declared type from its own front-matter (`.5c`). Defaults to
    /// `Unknown` when no front-matter is available (e.g. the SourceIR was reclaimed).
    pub declared_type: DeclaredDocType,
    /// Typed message/structure-field records (`PDF-VARIANT-DIGESTION.10/.11`). NOT used by
    /// `classify_document` (the 4-way structural class is unchanged); consumed by the richer
    /// purpose recognizer `classify_document_intent_category` (`DOC-INTENT-TAXONOMY.3`), where
    /// it is the packet/flit cue (cat 1) when no register map is present and the in-memory
    /// structure cue (cat 2/3) when one is. Kept on this one census so the validate site can
    /// drive both classifiers from a single object.
    pub message_field_records: usize,
    /// Typed signal-presence matrix rows (`PDF-VARIANT-DIGESTION.12b`) — a wire-protocol
    /// configuration surface that corroborates cat 1. Recognizer-only; ignored by
    /// `classify_document`.
    pub signal_presence_records: usize,
    /// The document's own front-matter self-declares a CPU instruction-set / privileged
    /// architecture (`DOC-INTENT-TAXONOMY.3`) — generic ISA doc-type vocabulary only, never a
    /// chip/vendor name (ADR 0006). `.1` proved CPU-ISA has NO distinct structural signature,
    /// so this self-declaration is the only positive cat-4 cue. Recognizer-only.
    pub front_matter_isa: bool,
    /// The document's own front-matter self-declares a physical / electrical / link layer
    /// (`DOC-INTENT-TAXONOMY.3`) — generic PHY/electrical doc-type vocabulary only, never a
    /// chip/vendor name (ADR 0006). `.1` proved cat 5 (PHY) and cat 6 (guide) are both
    /// near-empty and structurally indistinguishable, so this is the only positive cat-5 cue.
    /// Recognizer-only.
    pub front_matter_phy: bool,
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
    /// The document's self-declared type from its front-matter (`.5c`).
    pub declared_type: DeclaredDocType,
    /// `true` when the structural class is `Guide` (no reliable intent surface) BUT the
    /// document's own front-matter self-declares a specification/architecture/standard.
    /// That combination is NOT a true low-intent guide — it is a real design document we
    /// UNDER-EXTRACTED (typically image/table-heavy), so it routes to the VLM frontier
    /// instead of being quietly dismissed as a guide.
    pub under_extracted_spec: bool,
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

    // Front-matter corroboration (`.5c`): a structurally low-yield doc whose OWN front-matter
    // self-declares a specification/architecture/standard is not a true guide — it is a spec we
    // under-extracted (image/table-heavy), to be routed to the VLM frontier, not dismissed.
    let under_extracted_spec =
        class == DocumentClass::Guide && census.declared_type == DeclaredDocType::Specification;

    let declared_suffix = match (class, census.declared_type) {
        // Guide cases carry their own bespoke phrasing below; non-guide classes get a short note.
        (DocumentClass::Guide, _) => String::new(),
        (_, DeclaredDocType::Unknown) => String::new(),
        (_, declared) => format!("; self-declared: {}", declared.as_str()),
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
            let body = format!(
                "low structured design-intent ({} signals, {} registers, {} relations, {} signal constraints{narrative}{visual})",
                census.declared_signals,
                census.registers,
                census.actor_signal_relations,
                census.signal_constraints,
            );
            match census.declared_type {
                DeclaredDocType::Specification => format!(
                    "{body} — BUT the document's own front-matter self-declares a specification/architecture: likely an UNDER-EXTRACTED spec (image/table-heavy), not a true guide → route to VLM rescan, do not dismiss"
                ),
                DeclaredDocType::Guide => format!(
                    "{body} — corroborated by the document's own guide/overview framing; reported honestly, not a silent miss"
                ),
                DeclaredDocType::Unknown => {
                    format!("{body} — reported honestly, not a silent miss")
                }
            }
        }
        DocumentClass::Register => format!(
            "register-dominated ({} registers / {} fields; {} signal constraints, {interface_surface} connectivity edges){declared_suffix}",
            census.registers, census.register_fields, census.signal_constraints,
        ),
        DocumentClass::Protocol => format!(
            "behavioral protocol ({} signal constraints, {} FSM states, {} frame fields, {} actors, {} relations){declared_suffix}",
            census.signal_constraints,
            census.fsm_states,
            census.serial_frame_fields,
            census.protocol_actors,
            census.actor_signal_relations,
        ),
        DocumentClass::Interface => format!(
            "signal interface ({} signals, {} actor-signal relations; no behavioral obligations){declared_suffix}",
            census.declared_signals, census.actor_signal_relations,
        ),
    };

    DocumentClassification {
        class,
        rationale,
        census,
        declared_type: census.declared_type,
        under_extracted_spec,
    }
}

// ── Document intent-category recognizer (DOC-INTENT-TAXONOMY.3) ─────────────────

/// The 6-category PURPOSE taxonomy of a chip-spec PDF (`DOC-INTENT-TAXONOMY`) — what the
/// document is *about*. A richer SEMANTIC lens than the 4-way structural [`DocumentClass`]:
/// it folds cat 1+5 out of "protocol", cat 2+3 out of "register"/"interface", and has a slot
/// for cat 4 (CPU-ISA) that the structural class lacks. It is ADDITIVE — built ON the same
/// structural census (consumed as one input), never replacing the class.
///
/// `.1` measured the hard truth this enum encodes honestly: typed-surface counts CANNOT
/// separate cat 2 (register-IP) from cat 3 (platform/system-IP), cat 4 (ISA) has no structural
/// signature, and cat 5 (PHY) vs cat 6 (guide) are indistinguishable — so this taxonomy does
/// NOT pretend to split those, it carries a combined [`Self::RegisterOrPlatform`] and an
/// [`Self::Unresolved`] residual rather than fabricating a guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentIntentCategory {
    /// Category 1 — wire-level bus / interconnect protocol (signals, transactions, handshake /
    /// temporal rules, actor↔signal relations). ISF-mature; the only category the recognizer
    /// claims at HIGH confidence from structure alone.
    WireProtocol,
    /// Categories 2 OR 3 — register/memory-mapped IP vs platform/system-IP. `.1` measured these
    /// are NOT separable by typed-surface counts (both register/structure-dominant); the
    /// topology cue that would split them is deferred to `.3c` rather than guessed (honest
    /// residual). Also the honest home for a register-heavy protocol (CCIX) whose wire shape is
    /// outweighed by its register/structure surface — flagged in the residual.
    RegisterOrPlatform,
    /// Category 4 — CPU ISA / privileged architecture. `.1` measured NO structural signature,
    /// so this is reported ONLY on a front-matter ISA self-declaration; always LOW confidence.
    CpuIsa,
    /// Category 5 — physical / electrical / link layer. Behaviorally near-empty by nature (a
    /// thin `.isf` is CORRECT, not a gap); reported on a front-matter PHY self-declaration.
    PhysicalLink,
    /// Category 6 — methodology / language / EDA standard / guide. A non-target: recognized so
    /// chip intent is never forced out of it. HIGH confidence on a front-matter guide framing.
    MethodologyGuide,
    /// Structure + front-matter could not decide (e.g. a near-empty doc that is PHY-or-guide
    /// with no decisive self-declaration, or a self-declared specification we under-extracted).
    /// Honest residual, never a forced guess.
    Unresolved,
}

impl DocumentIntentCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireProtocol => "wire-protocol",
            Self::RegisterOrPlatform => "register-or-platform",
            Self::CpuIsa => "cpu-isa",
            Self::PhysicalLink => "physical-link",
            Self::MethodologyGuide => "methodology-guide",
            Self::Unresolved => "unresolved",
        }
    }
}

/// Recognizer confidence (`DOC-INTENT-TAXONOMY.3a` honest-residual policy): `High` only when a
/// decisive cue drove the call (a clean wire shape, or a front-matter guide self-declaration);
/// `Low` whenever the call rests on a weak/ambiguous cue — always paired with a `residual`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentCategoryConfidence {
    High,
    Low,
}

impl IntentCategoryConfidence {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Low => "low",
        }
    }
}

/// One intent-category decision: the category, the confidence, a human rationale, and an
/// explicit `residual` whenever structure + front-matter could not decide (honest residual
/// over fabrication, `[[feedback_scoring_rigor]]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentIntentClassification {
    pub category: DocumentIntentCategory,
    pub confidence: IntentCategoryConfidence,
    pub rationale: String,
    pub residual: Option<String>,
}

/// Minimum actor↔signal relations for a substantive wire-connectivity cue (category 1). Real
/// wire protocols carry dozens-to-hundreds (AXI 348, AHB 66, Avalon 111); register/structure
/// docs carry ~0 (NVMe 0, CCIX ~0). A floor of 8 admits genuine connectivity while rejecting a
/// register doc's incidental mentions. Measured on the persisted corpus (`DOC-INTENT-TAXONOMY.3b`).
const INTENT_WIRE_RELATION_MIN: usize = 8;
/// Minimum FSM-state + serial-frame-field count for a real state-behavior cue (category 1). A
/// floor of 2 rejects the 1 spurious FSM state several register/platform TRMs carry
/// (CoreSight SoC-600 = 1) while keeping real state machines (SWD/SWP 4, CAN 3+7 frame, CHI 7).
const INTENT_WIRE_STATE_MIN: usize = 2;
/// Minimum message-field records for a packet/flit cue, valid ONLY when the document carries no
/// register map (registers == 0) and some other wire cue is co-present. This is the measured
/// discriminator between cat-1 flit protocols (CHI 106, DTI 159 — zero registers) and cat-2/3
/// in-memory structures (NVMe 216, AMD-IOMMU 217, CCIX 92 — all carry registers).
const INTENT_WIRE_FLIT_MIN: usize = 16;
/// Minimum message/structure-field records for a structure-dominant document (category 2/3)
/// when no wire shape wins.
const INTENT_STRUCTURE_MIN: usize = 8;

/// Recognize a chip-spec document's PURPOSE category (`DOC-INTENT-TAXONOMY.3`). Pure,
/// deterministic, agnostic: every cue is a structural typed-surface count/shape or a generic
/// front-matter doc-type self-declaration — NO chip/vendor/protocol-instance name list
/// (ADR 0006). Additive: it consumes the same [`DocumentClassCensus`] as [`classify_document`]
/// and never replaces the structural class.
///
/// The decision order (grounded in the `.1` corpus census + the `.3b` per-document
/// measurement, the latter refining the `.3a` flit clause its own data falsified):
/// 1. **cat 6 guide** — a front-matter guide self-declaration beats spurious surface counts
///    (the `.2` finding: 5/14 guides over-extract). High confidence.
/// 2. **cat 1 wire** — a substantive wire shape (behavioral constraints that are not
///    register-dominated, a real FSM/frame, dense actor↔signal relations, or flit fields with
///    NO register map) whose wire weight is not outweighed by a register/structure surface.
///    A register/field count does NOT veto cat 1 (the AXI rescue: 71 registers, wire still
///    dominates). High confidence.
/// 3. **cat 4 CPU-ISA** — only via a front-matter ISA self-declaration (no structural
///    signature, `.1`). Low confidence + residual. Checked before the register step so a
///    register-bearing ISA volume is not mislabelled register-IP.
/// 4. **cat 2/3 register-or-platform** — registers/structures dominate (or a wire cue is
///    present but outweighed: the register-heavy-protocol case). `.1` proved 2↔3 is invisible
///    to counts → combined category + residual, never a guess. Low confidence.
/// 5. **cat 5 PHY** — only via a front-matter PHY self-declaration (cat 5 vs cat 6 invisible to
///    structure, `.1`). Low confidence + residual.
/// 6. **Unresolved** — near-empty with no decisive cue; a self-declared spec here is an
///    under-extracted document, otherwise PHY-or-guide. Low confidence + residual.
pub fn classify_document_intent_category(
    census: DocumentClassCensus,
) -> DocumentIntentClassification {
    // 1. A self-declared guide overrides any spurious surface counts (`.2` over-extraction).
    if census.declared_type == DeclaredDocType::Guide {
        return DocumentIntentClassification {
            category: DocumentIntentCategory::MethodologyGuide,
            confidence: IntentCategoryConfidence::High,
            rationale:
                "front-matter self-declares a methodology/guide — category 6 (an ISF non-target); a self-declared guide is not overridden by spurious surface counts"
                    .to_string(),
            residual: None,
        };
    }

    // Wire cues. `behavioral` reuses the SAME register-dominance test as `classify_document`, so
    // a register map's incidental constraints (NVMe 20) do not read as a wire shape, while a true
    // protocol's constraints (APB 19, DTI 16) do.
    let interface_surface = census.actor_signal_relations + census.declared_signals;
    let register_dominated = census.registers >= DOC_CLASS_REGISTER_MIN
        && census.registers >= interface_surface
        && census.registers >= census.signal_constraints;
    let behavioral = census.signal_constraints >= DOC_CLASS_BEHAVIORAL_MIN && !register_dominated;
    let has_state = (census.fsm_states + census.serial_frame_fields) >= INTENT_WIRE_STATE_MIN;
    let connected = census.actor_signal_relations >= INTENT_WIRE_RELATION_MIN;
    // Flit fields are a cat-1 cue ONLY without a register map (the measured CHI/DTI vs NVMe/AMD
    // discriminator) and only co-present with another wire cue.
    let flit = census.registers == 0
        && census.message_field_records >= INTENT_WIRE_FLIT_MIN
        && (census.actor_signal_relations >= 1
            || census.signal_constraints >= 1
            || census.fsm_states >= 1);
    let wire_cue = behavioral || has_state || connected || flit;

    // Surface weights for the dominance test. Message fields are wire intent (flit) when there is
    // no register map, otherwise in-memory-structure intent — so they are never double-counted.
    let flit_fields = if census.registers == 0 {
        census.message_field_records
    } else {
        0
    };
    let struct_fields = if census.registers == 0 {
        0
    } else {
        census.message_field_records
    };
    let wire_weight = census.actor_signal_relations
        + census.signal_constraints
        + flit_fields
        + census.fsm_states
        + census.serial_frame_fields
        + census.signal_presence_records;
    let struct_weight = census.registers + census.register_fields + struct_fields;

    // 2. Clean wire shape that is not outweighed by a register/structure surface → category 1.
    //    The register/field count does NOT veto (AXI: 71 regs, wire 401 > struct 229).
    if wire_cue && wire_weight >= struct_weight {
        let mut cues: Vec<String> = Vec::new();
        if behavioral {
            cues.push(format!("{} signal constraints", census.signal_constraints));
        }
        if census.fsm_states + census.serial_frame_fields >= INTENT_WIRE_STATE_MIN {
            cues.push(format!(
                "{} FSM states / {} serial-frame fields",
                census.fsm_states, census.serial_frame_fields
            ));
        }
        if connected {
            cues.push(format!(
                "{} actor-signal relations",
                census.actor_signal_relations
            ));
        }
        if flit {
            cues.push(format!(
                "{} flit/message fields with no register map",
                census.message_field_records
            ));
        }
        if census.signal_presence_records > 0 {
            cues.push(format!(
                "{} signal-presence rows",
                census.signal_presence_records
            ));
        }
        return DocumentIntentClassification {
            category: DocumentIntentCategory::WireProtocol,
            confidence: IntentCategoryConfidence::High,
            rationale: format!(
                "wire-behavioral shape ({}); wire weight {} ≥ register/structure weight {} — register/field count does not veto category 1",
                cues.join(", "),
                wire_weight,
                struct_weight
            ),
            residual: None,
        };
    }

    // 3. A front-matter ISA self-declaration → category 4, BEFORE the register step so a
    //    register-bearing ISA volume is not mislabelled register-IP. No structural signature
    //    exists (`.1`), so this rests on the self-declaration alone → Low confidence + residual.
    if census.front_matter_isa {
        return DocumentIntentClassification {
            category: DocumentIntentCategory::CpuIsa,
            confidence: IntentCategoryConfidence::Low,
            rationale:
                "front-matter self-declares a CPU instruction-set / privileged architecture (category 4); no distinct structural signature exists (DOC-INTENT-TAXONOMY.1)"
                    .to_string(),
            residual: Some(
                "category 4 (CPU-ISA) rests on the front-matter self-declaration alone; instruction / CSR / privilege / exception intent has no dedicated typed surface yet (DOC-INTENT-TAXONOMY.4+)"
                    .to_string(),
            ),
        };
    }

    // 4. Register- or in-memory-structure-dominant, OR a wire cue outweighed by structure (the
    //    register-heavy protocol). `.1` proved cat 2↔3 is invisible to counts → combined category
    //    + residual, never a guess.
    if census.registers >= DOC_CLASS_REGISTER_MIN
        || census.message_field_records >= INTENT_STRUCTURE_MIN
    {
        let wire_note = if wire_cue {
            " — a wire cue is present but the register/structure surface outweighs it, so this may be a register-heavy WIRE protocol (category 1); the splitting topology/relation cue is deferred to .3c"
        } else {
            ""
        };
        return DocumentIntentClassification {
            category: DocumentIntentCategory::RegisterOrPlatform,
            confidence: IntentCategoryConfidence::Low,
            rationale: format!(
                "register/structure-dominant ({} registers / {} fields, {} message/structure fields; structure weight {} vs wire weight {})",
                census.registers,
                census.register_fields,
                census.message_field_records,
                struct_weight,
                wire_weight
            ),
            residual: Some(format!(
                "categories 2 (register/memory-mapped IP) and 3 (platform/system-IP) are not separable by typed-surface counts (DOC-INTENT-TAXONOMY.1); a topology cue to split them is deferred to .3c rather than guessed{wire_note}"
            )),
        };
    }

    // 5. A front-matter PHY self-declaration → category 5 (cat 5 vs cat 6 is invisible to
    //    structure, `.1`). Behaviorally near-empty by nature — a thin `.isf` is correct.
    if census.front_matter_phy {
        return DocumentIntentClassification {
            category: DocumentIntentCategory::PhysicalLink,
            confidence: IntentCategoryConfidence::Low,
            rationale:
                "front-matter self-declares a physical / electrical / link layer (category 5); behaviorally near-empty by nature — a thin .isf is correct, not a gap"
                    .to_string(),
            residual: Some(
                "category 5 (PHY) rests on the front-matter self-declaration; physical / electrical / link intent is an honest ISF non-target"
                    .to_string(),
            ),
        };
    }

    // 6. Near-empty with no decisive cue. A self-declared specification here is a real document
    //    we under-extracted; otherwise the doc is PHY-or-guide (structure cannot split, `.1`).
    let residual = if census.declared_type == DeclaredDocType::Specification {
        "front-matter self-declares a specification but no typed intent surface was recovered — an UNDER-EXTRACTED spec (image/table-heavy); true category indeterminate until re-extraction (see document_class under_extracted_spec)"
    } else {
        "near-empty: physical/electrical (category 5) or methodology/guide (category 6) — structure cannot split them (DOC-INTENT-TAXONOMY.1) and no decisive front-matter cue is present"
    };
    DocumentIntentClassification {
        category: DocumentIntentCategory::Unresolved,
        confidence: IntentCategoryConfidence::Low,
        rationale: "no decisive structural or front-matter cue for a purpose category".to_string(),
        residual: Some(residual.to_string()),
    }
}

// ── Per-document completeness gauge (PDF-VARIANT-DIGESTION.5b) ──────────────────

/// Bounded number of affected item names/ids carried as a review sample per gap.
const GAUGE_SAMPLE_CAP: usize = 8;

/// One completeness dimension's result: how many extracted items of a given kind
/// are MISSING a mandatory attribute, out of how many of that kind the extraction
/// produced. Pure observation — every counted item is one the extraction itself
/// produced but left incomplete; nothing is fabricated. `sample` is a bounded
/// (≤ `GAUGE_SAMPLE_CAP`) list of affected item names/ids for human review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletenessGap {
    /// Stable machine label, e.g. `registers_without_fields`.
    pub kind: &'static str,
    /// Items of this kind missing the mandatory attribute (the numerator).
    pub missing: usize,
    /// Items of this kind the extraction produced (the denominator).
    pub total: usize,
    /// A bounded sample (≤ `GAUGE_SAMPLE_CAP`) of affected item names/ids — never fabricated.
    pub sample: Vec<String>,
}

/// A class-aware per-document completeness gauge (`PDF-VARIANT-DIGESTION.5b`): how
/// complete is the typed intent the extraction DID produce, judged appropriately
/// for the document's class.
///
/// The class (`PDF-VARIANT-DIGESTION.5a`) decides applicability so the gauge never
/// punishes a document for lacking a surface it was never expected to carry: a
/// `Guide` (low structured design-intent) is `applicable == false` — there is no
/// design surface whose completeness to gauge, and the `.5c` under-extracted flag
/// already carries the "actually an under-extracted spec" case. For the three
/// design-document classes each dimension is gauged ONLY when its denominator is
/// non-zero, so a protocol with no registers shows no register gap while a register
/// document IS held to "every register has fields and a resolved width".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentCompletenessGauge {
    pub class: DocumentClass,
    /// `false` for `Guide` — a low structured-design-intent document has no design
    /// surface whose completeness to gauge (honest, not a penalty).
    pub applicable: bool,
    /// Per-dimension gaps, only for dimensions whose denominator (`total`) > 0.
    pub gaps: Vec<CompletenessGap>,
}

impl DocumentCompletenessGauge {
    /// Total incomplete items across every gauged dimension.
    pub fn total_missing(&self) -> usize {
        self.gaps.iter().map(|g| g.missing).sum()
    }

    /// `true` when the gauge applies AND found no incomplete item — a document whose
    /// produced intent is fully attributed. A non-applicable (`Guide`) gauge is never
    /// "complete": there was nothing to measure.
    pub fn is_complete(&self) -> bool {
        self.applicable && self.total_missing() == 0
    }
}

/// Bounded review sample of the first ≤ `GAUGE_SAMPLE_CAP` items from `names`.
fn gauge_sample<I: IntoIterator<Item = String>>(names: I) -> Vec<String> {
    names.into_iter().take(GAUGE_SAMPLE_CAP).collect()
}

/// Build the class-aware per-document completeness gauge. Pure and deterministic:
/// every count is read off already-built IR; the only judgement is the
/// class-applicability gate and the "denominator > 0" inclusion rule — no chip,
/// vendor, or protocol vocabulary (ADR 0006).
///
/// - `registers` — the document's register records (the field/width dimensions);
/// - `declared_signal_count` — the size of the declared-signal inventory (the
///   signal-direction denominator);
/// - `signals_missing_direction` — declared signals with no resolved direction (the
///   caller computes this as the inventory minus the directioned set; the evidence
///   stage already folds relation-derived directions into that set);
/// - `intent_bearing_table_count` — register/signal/timing tables the classifier
///   recognized (the table-coverage denominator);
/// - `unexplained_tables` — those intent-bearing tables that produced no record
///   (from [`unexplained_intent_bearing_tables`]).
pub fn document_completeness_gauge(
    class: DocumentClass,
    registers: &[RegisterRecord],
    declared_signal_count: usize,
    signals_missing_direction: &[String],
    intent_bearing_table_count: usize,
    unexplained_tables: &[UnexplainedTableResidual],
) -> DocumentCompletenessGauge {
    // A guide carries no design surface to gauge — never penalize it for absence.
    if class == DocumentClass::Guide {
        return DocumentCompletenessGauge {
            class,
            applicable: false,
            gaps: Vec::new(),
        };
    }

    let mut gaps = Vec::new();

    if !registers.is_empty() {
        let no_fields: Vec<&RegisterRecord> =
            registers.iter().filter(|r| r.fields.is_empty()).collect();
        gaps.push(CompletenessGap {
            kind: "registers_without_fields",
            missing: no_fields.len(),
            total: registers.len(),
            sample: gauge_sample(no_fields.iter().map(|r| r.register_name.clone())),
        });

        let unresolved_width = registers_with_unresolved_width(registers);
        gaps.push(CompletenessGap {
            kind: "registers_unresolved_width",
            missing: unresolved_width.len(),
            total: registers.len(),
            sample: gauge_sample(unresolved_width),
        });
    }

    if declared_signal_count > 0 {
        gaps.push(CompletenessGap {
            kind: "signals_without_direction",
            missing: signals_missing_direction.len(),
            total: declared_signal_count,
            sample: gauge_sample(signals_missing_direction.iter().cloned()),
        });
    }

    if intent_bearing_table_count > 0 {
        gaps.push(CompletenessGap {
            kind: "unexplained_intent_bearing_tables",
            missing: unexplained_tables.len(),
            total: intent_bearing_table_count,
            sample: gauge_sample(unexplained_tables.iter().map(|t| t.table_id.clone())),
        });
    }

    DocumentCompletenessGauge {
        class,
        applicable: true,
        gaps,
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
            access_type: None,
            offset_address: None,
            size_bits: None,
            fields,
            supporting_table_ids: vec![],
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
            access_type: None,
            offset_address: None,
            size_bits: None,
            fields: vec![],
            supporting_table_ids: vec![],
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
            supporting_table_ids: vec![],
            intent_disposition: Default::default(),
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

    fn hcell(text: &str) -> crate::ir::source::StructuredTableCellRecord {
        crate::ir::source::StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header: true,
        }
    }
    /// PDF-VARIANT-DIGESTION.12a — a SignalDescription table whose data rows Docling trapped in
    /// `header_rows` (label cell header-marked, value cells not), leaving `body_rows` empty.
    fn signal_table_with_trapped_rows(
        table_id: &str,
        caption: Option<&str>,
        kind: TableKind,
        header: &[&str],
        trapped: &[&[&str]],
    ) -> StructuredTableRecord {
        let mut header_rows: Vec<Vec<_>> = vec![header.iter().map(|t| hcell(t)).collect()];
        header_rows.extend(trapped.iter().map(|row| {
            row.iter()
                .enumerate()
                .map(|(i, t)| if i == 0 { hcell(t) } else { cell(t) })
                .collect::<Vec<_>>()
        }));
        StructuredTableRecord {
            table_id: table_id.to_string(),
            asset_id: format!("asset_{table_id}"),
            page_id: None,
            caption_text: caption.map(|c| c.to_string()),
            source_ref: None,
            table_kind: kind,
            header_rows,
            row_count: trapped.len() as u32,
            col_count: header.len() as u32,
            body_rows: vec![],
        }
    }

    #[test]
    fn header_trapped_signal_table_covered_by_inventory() {
        // PDF-VARIANT-DIGESTION.12a — a presence matrix whose every data row is header-trapped has
        // NO body rows; the coverage must read the recovered rows or a fully-redundant duplicate
        // presentation could never be recognized as covered.
        let table = signal_table_with_trapped_rows(
            "table_0266",
            Some("Table B2.2: Summary of signal presence for each interface class"),
            TableKind::SignalDescription,
            &["Signal", "Presence", "AXI5"],
            &[&["PCLK", "-", "Y"], &["PADDR", "-", "Y"]],
        );
        let inv = inventory(&["PCLK", "PADDR"]);
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &inv);
        assert!(r.is_empty(), "all trapped signals declared → covered");
    }

    #[test]
    fn header_trapped_signal_table_with_unknown_signal_stays_flagged() {
        // Strictness is unchanged by the trapped-row recovery: one unknown signal anywhere in the
        // densest column keeps the table flagged, so a genuine miss is never hidden.
        let table = signal_table_with_trapped_rows(
            "table_0078",
            Some("Table C5.1: Summary of signal presence"),
            TableKind::SignalDescription,
            &["Signal", "Presence", "Issue"],
            &[&["PCLK", "-", "Y"], &["PNEWSIG", "-", "O"]],
        );
        let inv = inventory(&["PCLK"]);
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &inv);
        assert_eq!(
            r.len(),
            1,
            "an uncovered trapped signal keeps the table flagged"
        );
    }

    #[test]
    fn presence_captured_matrix_is_explained_without_inventory() {
        // PDF-VARIANT-DIGESTION.12b — a presence matrix whose rows the signal-presence surface
        // captures COMPLETELY is explained even when its signals are absent from the declared
        // inventory (a generic-name family like `AxVALID` has no literal declaration anywhere):
        // the presence capture IS the typed record the accounting was asking for.
        let table = signal_table_with_trapped_rows(
            "table_0188",
            Some("Table A13.3: Signal presence"),
            TableKind::SignalDescription,
            &["Signal", "Presence", "V1", "V2"],
            &[
                &["AxVALID", "-", "Y", "O"],
                &["AxADDR", "XADDR_WIDTH > 0", "O", "N"],
            ],
        );
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &HashSet::new());
        assert!(
            r.is_empty(),
            "a fully presence-captured matrix is explained"
        );
    }

    #[test]
    fn partially_refused_presence_matrix_stays_flagged() {
        // Strictness (WIRE-BASED-100.3a): one refused row (a fused `Y Y` cell the capture
        // honestly refuses) keeps the matrix flagged — partial capture never hides a miss.
        let table = signal_table_with_trapped_rows(
            "table_0016",
            None,
            TableKind::SignalDescription,
            &["Signal", "Presence", "V1", "V2", "V3"],
            &[
                &["XCLK", "-", "Y", "Y", "Y"],
                &["XRST", "-", "Y Y", "", "Y"],
            ],
        );
        let r = unexplained_intent_bearing_tables(&[table], &[], &[], &[], &HashSet::new());
        assert_eq!(
            r.len(),
            1,
            "a refused row keeps the matrix an honest candidate miss"
        );
    }

    #[test]
    fn unknown_continuation_fragment_accounted_under_chain_head_kind() {
        // PDF-VARIANT-DIGESTION.12a — an `unknown` continuation fragment of a signal table joins
        // the accounting under the head's kind (it can no longer hide from the gauge); whether it
        // is covered then follows the normal inventory rule.
        let head = signal_table_with_trapped_rows(
            "table_head",
            Some("Table B2.3: Summary of check signal presence"),
            TableKind::SignalDescription,
            &["Signal", "AXI5"],
            &[&["PADDRCHK", "O"]],
        );
        let frag = signal_table_with_trapped_rows(
            "table_frag",
            Some("Table B2.3 Continued from previous page"),
            TableKind::Unknown,
            &["Signal", "AXI5"],
            &[&["PNEWCHK", "O"]],
        );
        // The fragment carries an undeclared signal: it must now surface as a candidate miss.
        let inv = inventory(&["PADDRCHK"]);
        let r =
            unexplained_intent_bearing_tables(&[head.clone(), frag.clone()], &[], &[], &[], &inv);
        assert_eq!(r.len(), 1, "the fragment is accounted and flagged");
        assert_eq!(r[0].table_id, "table_frag");
        assert_eq!(r[0].table_kind, "signal_description");
        // Once its signal is declared, the same fragment is covered.
        let inv = inventory(&["PADDRCHK", "PNEWCHK"]);
        let r = unexplained_intent_bearing_tables(&[head, frag], &[], &[], &[], &inv);
        assert!(r.is_empty(), "declared fragment signals → covered");
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

    // ── front-matter doc-type hint (PDF-VARIANT-DIGESTION.5c) ────────────────

    #[test]
    fn front_matter_hint_recognizes_real_guide_framings() {
        // Real early-heading strings from the corpus.
        assert_eq!(
            front_matter_doc_type_hint("Arm Cortex-A76 Software Optimization Guide"),
            DeclaredDocType::Guide
        );
        // Arm's "Learn the architecture …" series is a GUIDE even though it contains
        // "architecture" — the guide phrasing must win over the spec word.
        assert_eq!(
            front_matter_doc_type_hint("Learn the architecture - AArch64 external debug"),
            DeclaredDocType::Guide
        );
        assert_eq!(
            front_matter_doc_type_hint("USB4 Connection Manager Guide"),
            DeclaredDocType::Guide
        );
    }

    #[test]
    fn front_matter_hint_recognizes_real_spec_framings() {
        // Real early-heading strings from the corpus.
        assert_eq!(
            front_matter_doc_type_hint("AMBA APB Protocol Specification Release Information"),
            DeclaredDocType::Specification
        );
        assert_eq!(
            front_matter_doc_type_hint("The RISC-V Advanced Interrupt Architecture"),
            DeclaredDocType::Specification
        );
        assert_eq!(
            front_matter_doc_type_hint("JEDEC STANDARD High Bandwidth Memory (HBM) DRAM Scope"),
            DeclaredDocType::Specification
        );
        // A spec's own "Introduction to …" chapter must NOT read as a guide.
        assert_eq!(
            front_matter_doc_type_hint(
                "Avalon Interface Specifications Contents 1. Introduction to the Avalon Interface"
            ),
            DeclaredDocType::Specification
        );
    }

    #[test]
    fn front_matter_hint_is_unknown_without_a_doctype_word() {
        assert_eq!(
            front_matter_doc_type_hint("Release Information Proprietary Notice Change History"),
            DeclaredDocType::Unknown
        );
        // Whole-word match: "guidelines" must NOT trip the "guide" word.
        assert_eq!(
            front_matter_doc_type_hint("Design Guidelines and Coding Rules"),
            DeclaredDocType::Unknown
        );
    }

    #[test]
    fn structural_guide_with_spec_front_matter_is_flagged_under_extracted() {
        // THE .5c value: a structurally low-yield doc whose OWN front-matter self-declares a
        // specification/architecture is NOT a true guide — it is a spec we under-extracted
        // (image/table-heavy), flagged for the VLM frontier, not dismissed.
        let r = classify_document(census(|c| {
            c.visual_evidence = 200; // image-heavy
            c.declared_type = DeclaredDocType::Specification;
        }));
        assert_eq!(r.class, DocumentClass::Guide);
        assert!(r.under_extracted_spec);
        assert!(
            r.rationale.contains("UNDER-EXTRACTED spec") && r.rationale.contains("VLM rescan"),
            "an under-extracted spec must say so: {}",
            r.rationale
        );
    }

    #[test]
    fn structural_guide_with_guide_front_matter_is_a_confirmed_guide() {
        // A true guide: structurally empty AND self-declares a guide → confirmed, not flagged.
        let r = classify_document(census(|c| {
            c.conditional_rules = 12;
            c.declared_type = DeclaredDocType::Guide;
        }));
        assert_eq!(r.class, DocumentClass::Guide);
        assert!(!r.under_extracted_spec);
        assert!(
            r.rationale
                .contains("corroborated by the document's own guide")
        );
    }

    #[test]
    fn non_guide_class_notes_the_declared_type_but_is_not_under_extracted() {
        // A real protocol whose title says "specification" → corroboration only, never flagged.
        let r = classify_document(census(|c| {
            c.signal_constraints = 14;
            c.actor_signal_relations = 69;
            c.declared_type = DeclaredDocType::Specification;
        }));
        assert_eq!(r.class, DocumentClass::Protocol);
        assert!(!r.under_extracted_spec);
        assert!(r.rationale.contains("self-declared: specification"));
    }

    // ── document intent-category recognizer (DOC-INTENT-TAXONOMY.3b) ─────────
    // Every census below is the REAL persisted-corpus measurement for the named document
    // (read off generated/evidence_ir/<key>/evidence_ir.json), so each test demonstrates the
    // recognizer per item (`[[feedback_scoring_rigor]]`), not on invented numbers.

    #[test]
    fn isa_phy_front_matter_helpers_are_generic_and_whole_word() {
        // Generic ISA doc-type vocabulary, no instance names (ADR 0006).
        assert!(front_matter_declares_isa(
            "RISC-V Instruction Set Manual Volume I"
        ));
        assert!(front_matter_declares_isa(
            "The RISC-V Privileged Architecture"
        ));
        assert!(front_matter_declares_isa("Some Title (RISC-V ISA)"));
        // "architecture" alone is NOT ISA (every spec says architecture) — keeps the AIA /
        // AHB "architecture" docs out of cat 4.
        assert!(!front_matter_declares_isa(
            "The RISC-V Advanced Interrupt Architecture"
        ));
        assert!(!front_matter_declares_isa(
            "AMBA 5 AHB Protocol Specification"
        ));
        // Generic PHY vocabulary, no instance names.
        assert!(front_matter_declares_phy(
            "OpenCAPI 25Gbps PHY Signaling Spec"
        ));
        assert!(front_matter_declares_phy(
            "USB4 Physical Layer Specification"
        ));
        assert!(front_matter_declares_phy(
            "25 Gbps Physical Signaling Specification"
        ));
        assert!(front_matter_declares_phy(
            "Link Training and Status State Machine"
        ));
        // A protocol's incidental "signaling" prose must NOT read as PHY.
        assert!(!front_matter_declares_phy(
            "AMBA APB Protocol Specification"
        ));
    }

    #[test]
    fn declared_guide_overrides_spurious_surface_counts() {
        // The `.2` finding: 5/14 guides over-extract (cortex-a76 sw-opt: 537 signals). A
        // self-declared guide must still be category 6, not be flipped by the spurious counts.
        let r = classify_document_intent_category(census(|c| {
            c.declared_signals = 537;
            c.actor_signal_relations = 40;
            c.declared_type = DeclaredDocType::Guide;
        }));
        assert_eq!(r.category, DocumentIntentCategory::MethodologyGuide);
        assert_eq!(r.confidence, IntentCategoryConfidence::High);
        assert!(r.residual.is_none());
    }

    #[test]
    fn axi_is_wire_protocol_despite_its_registers() {
        // AXI ihi0022_l: 71 registers / 158 fields, but 348 relations + 50 constraints. The
        // register count must NOT veto cat 1 (wire weight 401 ≥ struct weight 229).
        let r = classify_document_intent_category(census(|c| {
            c.registers = 71;
            c.register_fields = 158;
            c.actor_signal_relations = 348;
            c.signal_constraints = 50;
            c.fsm_states = 3;
        }));
        assert_eq!(r.category, DocumentIntentCategory::WireProtocol);
        assert_eq!(r.confidence, IntentCategoryConfidence::High);
        assert!(r.residual.is_none());
        assert!(r.rationale.contains("does not veto category 1"));
    }

    #[test]
    fn chi_and_dti_flit_protocols_are_wire_without_a_register_map() {
        // CHI ihi0050: 0 registers, 79 relations, 7 FSM states, 106 flit fields → cat 1.
        let chi = classify_document_intent_category(census(|c| {
            c.actor_signal_relations = 79;
            c.fsm_states = 7;
            c.message_field_records = 106;
        }));
        assert_eq!(chi.category, DocumentIntentCategory::WireProtocol);
        assert_eq!(chi.confidence, IntentCategoryConfidence::High);
        // DTI ihi0088: 0 registers, 1 relation, 16 constraints, 2 states, 159 flit fields → cat 1
        // (behavioral + flit; the flit fields count only because there is no register map).
        let dti = classify_document_intent_category(census(|c| {
            c.actor_signal_relations = 1;
            c.signal_constraints = 16;
            c.fsm_states = 2;
            c.message_field_records = 159;
        }));
        assert_eq!(dti.category, DocumentIntentCategory::WireProtocol);
        assert!(
            dti.rationale
                .contains("flit/message fields with no register map")
        );
    }

    #[test]
    fn fsm_or_frame_only_protocols_are_wire() {
        // SWP/SWD shape (etsi swp): 5 relations, 4 FSM states, no constraints → cat 1 via state.
        let swp = classify_document_intent_category(census(|c| {
            c.registers = 1;
            c.register_fields = 4;
            c.actor_signal_relations = 5;
            c.fsm_states = 4;
        }));
        assert_eq!(swp.category, DocumentIntentCategory::WireProtocol);
        // CAN: 0 relations, 3 FSM states + 7 serial-frame fields → cat 1 via frame.
        let can = classify_document_intent_category(census(|c| {
            c.fsm_states = 3;
            c.serial_frame_fields = 7;
        }));
        assert_eq!(can.category, DocumentIntentCategory::WireProtocol);
    }

    #[test]
    fn nvme_register_map_is_not_wire_despite_incidental_constraints() {
        // NVMe: 42 registers / 201 fields, 0 relations, 20 incidental constraints, 216 STRUCTURE
        // fields. Register-dominated → its constraints are NOT a wire shape; the 216 message
        // fields are in-memory structures (a register map is present) → register-or-platform.
        let r = classify_document_intent_category(census(|c| {
            c.registers = 42;
            c.register_fields = 201;
            c.signal_constraints = 20;
            c.message_field_records = 216;
        }));
        assert_eq!(r.category, DocumentIntentCategory::RegisterOrPlatform);
        assert_eq!(r.confidence, IntentCategoryConfidence::Low);
        assert!(r.residual.as_deref().unwrap().contains("not separable"));
    }

    #[test]
    fn amd_iommu_relations_do_not_make_a_structure_doc_wire() {
        // AMD-IOMMU: 8 registers, 98 relations, 217 STRUCTURE fields. The 98 relations are a
        // wire cue, but the structure weight (8 + 217 = 225) outweighs the wire weight (98) →
        // register-or-platform, with the residual flagging the present-but-outweighed wire cue.
        let r = classify_document_intent_category(census(|c| {
            c.registers = 8;
            c.actor_signal_relations = 98;
            c.message_field_records = 217;
        }));
        assert_eq!(r.category, DocumentIntentCategory::RegisterOrPlatform);
        assert!(
            r.residual
                .as_deref()
                .unwrap()
                .contains("register-heavy WIRE protocol")
        );
    }

    #[test]
    fn register_heavy_protocols_and_trms_are_register_or_platform_with_an_honest_residual() {
        // CCIX rev2: 143 registers / 389 fields, ~0 relations, 45 message fields — a
        // register-heavy interconnect. Structurally register-dominant → register-or-platform.
        let ccix = classify_document_intent_category(census(|c| {
            c.registers = 143;
            c.register_fields = 389;
            c.message_field_records = 45;
        }));
        assert_eq!(ccix.category, DocumentIntentCategory::RegisterOrPlatform);
        // GIC-600 TRM: 33 registers / 293 fields, 101 relations, 7 constraints, 2 states. A wire
        // cue is present (rel 101) but the field surface dominates (struct 326 vs wire 110) → the
        // register/platform home, with the residual noting the present-but-outweighed wire cue.
        let gic = classify_document_intent_category(census(|c| {
            c.registers = 33;
            c.register_fields = 293;
            c.actor_signal_relations = 101;
            c.signal_constraints = 7;
            c.fsm_states = 2;
        }));
        assert_eq!(gic.category, DocumentIntentCategory::RegisterOrPlatform);
        assert!(
            gic.residual
                .as_deref()
                .unwrap()
                .contains("register-heavy WIRE protocol")
        );
    }

    #[test]
    fn isa_self_declaration_is_cpu_isa_even_with_registers() {
        // A register-bearing ISA volume: the ISA front-matter is checked before the register
        // step, so it is category 4, not register-IP — Low confidence (no structural signature).
        let r = classify_document_intent_category(census(|c| {
            c.registers = 30;
            c.register_fields = 120;
            c.front_matter_isa = true;
            c.declared_type = DeclaredDocType::Specification;
        }));
        assert_eq!(r.category, DocumentIntentCategory::CpuIsa);
        assert_eq!(r.confidence, IntentCategoryConfidence::Low);
        assert!(r.residual.as_deref().unwrap().contains("CPU-ISA"));
    }

    #[test]
    fn phy_self_declaration_is_physical_link() {
        // OpenCAPI PHY: behaviorally near-empty + a PHY front-matter self-declaration → cat 5.
        let r = classify_document_intent_category(census(|c| {
            c.front_matter_phy = true;
            c.declared_type = DeclaredDocType::Specification;
        }));
        assert_eq!(r.category, DocumentIntentCategory::PhysicalLink);
        assert_eq!(r.confidence, IntentCategoryConfidence::Low);
        assert!(r.residual.as_deref().unwrap().contains("ISF non-target"));
    }

    #[test]
    fn near_empty_spec_is_an_under_extracted_residual_not_a_forced_category() {
        // RISC-V AIA shape: 0 of every typed surface, but front-matter self-declares an
        // architecture (Specification). Honest: an under-extracted spec, not a forced guess.
        let r = classify_document_intent_category(census(|c| {
            c.conditional_rules = 39; // over-produced narrative, not class-determining
            c.declared_type = DeclaredDocType::Specification;
        }));
        assert_eq!(r.category, DocumentIntentCategory::Unresolved);
        assert!(r.residual.as_deref().unwrap().contains("UNDER-EXTRACTED"));
    }

    #[test]
    fn near_empty_with_no_declaration_is_phy_or_guide_residual() {
        // Truly near-empty, no front-matter cue: structure cannot split cat 5 (PHY) from cat 6
        // (guide) → Unresolved with the honest PHY-or-guide residual, never a forced guess.
        let r = classify_document_intent_category(census(|c| c.visual_evidence = 5));
        assert_eq!(r.category, DocumentIntentCategory::Unresolved);
        assert_eq!(r.confidence, IntentCategoryConfidence::Low);
        assert!(
            r.residual
                .as_deref()
                .unwrap()
                .contains("physical/electrical")
        );
    }

    #[test]
    fn only_wire_and_guide_are_ever_high_confidence() {
        // The signoff guarantee: the recognizer never makes a HIGH-confidence claim outside the
        // two robust cases (a clean wire shape, a self-declared guide). Everything else is Low,
        // always carrying a residual — no high-confidence mislabel.
        let low_cases = [
            classify_document_intent_category(census(|c| {
                c.registers = 42;
                c.message_field_records = 216;
            })),
            classify_document_intent_category(census(|c| c.front_matter_isa = true)),
            classify_document_intent_category(census(|c| c.front_matter_phy = true)),
            classify_document_intent_category(census(|c| c.visual_evidence = 1)),
        ];
        for r in low_cases {
            assert_eq!(
                r.confidence,
                IntentCategoryConfidence::Low,
                "{:?}",
                r.category
            );
            assert!(
                r.residual.is_some(),
                "a Low call must carry a residual: {:?}",
                r.category
            );
        }
    }

    // ── per-document completeness gauge (PDF-VARIANT-DIGESTION.5b) ───────────

    fn unexplained(table_id: &str) -> UnexplainedTableResidual {
        UnexplainedTableResidual {
            table_id: table_id.to_string(),
            table_kind: "register_map",
            caption: None,
        }
    }

    #[test]
    fn gauge_is_not_applicable_for_a_guide() {
        // A guide carries no design surface to gauge — even with stray registers/signals
        // it is NEVER penalized; the honest report is "not applicable", not a 0% score.
        let regs = vec![register("STRAY", vec![])];
        let g = document_completeness_gauge(
            DocumentClass::Guide,
            &regs,
            5,
            &["X".to_string()],
            3,
            &[unexplained("table_0001")],
        );
        assert!(!g.applicable);
        assert!(g.gaps.is_empty());
        assert!(
            !g.is_complete(),
            "a non-applicable gauge is never 'complete'"
        );
    }

    #[test]
    fn register_doc_is_held_to_fields_and_width() {
        // A register document IS held to "every register has fields and a resolved width".
        let mut sized = register("CTRL", vec![field("EN", Some(0), Some(0))]);
        sized.size_bits = Some(32);
        let bare = register("STATUS", vec![]); // no fields, size_bits None
        let regs = vec![sized, bare];
        // No signal inventory and no intent-bearing tables → only register dimensions gauged.
        let g = document_completeness_gauge(DocumentClass::Register, &regs, 0, &[], 0, &[]);
        assert!(g.applicable);
        let kinds: Vec<&str> = g.gaps.iter().map(|x| x.kind).collect();
        assert_eq!(
            kinds,
            vec!["registers_without_fields", "registers_unresolved_width"]
        );
        let no_fields = g
            .gaps
            .iter()
            .find(|x| x.kind == "registers_without_fields")
            .unwrap();
        assert_eq!((no_fields.missing, no_fields.total), (1, 2));
        assert_eq!(no_fields.sample, vec!["STATUS".to_string()]);
        let width = g
            .gaps
            .iter()
            .find(|x| x.kind == "registers_unresolved_width")
            .unwrap();
        assert_eq!((width.missing, width.total), (1, 2));
        assert_eq!(width.sample, vec!["STATUS".to_string()]);
        assert_eq!(g.total_missing(), 2);
    }

    #[test]
    fn protocol_doc_with_no_registers_shows_no_register_gap() {
        // Class-appropriateness: a protocol with a signal inventory but zero registers is
        // gauged ONLY on signal direction + table coverage — never a register gap.
        let g = document_completeness_gauge(
            DocumentClass::Protocol,
            &[],
            10,
            &["PREADY".to_string(), "PSLVERR".to_string()],
            4,
            &[unexplained("table_0007")],
        );
        let kinds: Vec<&str> = g.gaps.iter().map(|x| x.kind).collect();
        assert_eq!(
            kinds,
            vec![
                "signals_without_direction",
                "unexplained_intent_bearing_tables"
            ]
        );
        let dir = g
            .gaps
            .iter()
            .find(|x| x.kind == "signals_without_direction")
            .unwrap();
        assert_eq!((dir.missing, dir.total), (2, 10));
        let tables = g
            .gaps
            .iter()
            .find(|x| x.kind == "unexplained_intent_bearing_tables")
            .unwrap();
        assert_eq!((tables.missing, tables.total), (1, 4));
        assert_eq!(tables.sample, vec!["table_0007".to_string()]);
    }

    #[test]
    fn a_fully_attributed_design_doc_is_complete() {
        // Every register has fields + width, every signal a direction, every table explained.
        let mut sized = register("CTRL", vec![field("EN", Some(0), Some(0))]);
        sized.size_bits = Some(8);
        let g = document_completeness_gauge(DocumentClass::Register, &[sized], 3, &[], 2, &[]);
        assert!(g.applicable);
        assert_eq!(g.total_missing(), 0);
        assert!(g.is_complete());
        // The gauged dimensions are still PRESENT with missing == 0 so completeness is visible,
        // not silently dropped.
        assert!(
            g.gaps
                .iter()
                .any(|x| x.kind == "registers_without_fields" && x.missing == 0)
        );
        assert!(
            g.gaps
                .iter()
                .any(|x| x.kind == "signals_without_direction" && x.missing == 0)
        );
    }

    #[test]
    fn gauge_sample_is_bounded() {
        // 12 field-less registers → the review sample is capped at GAUGE_SAMPLE_CAP (8),
        // while the count itself stays exact.
        let regs: Vec<RegisterRecord> = (0..12)
            .map(|i| register(&format!("R{i}"), vec![]))
            .collect();
        let g = document_completeness_gauge(DocumentClass::Register, &regs, 0, &[], 0, &[]);
        let no_fields = g
            .gaps
            .iter()
            .find(|x| x.kind == "registers_without_fields")
            .unwrap();
        assert_eq!(no_fields.missing, 12);
        assert_eq!(no_fields.sample.len(), GAUGE_SAMPLE_CAP);
    }
}
