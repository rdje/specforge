//! EXTRACTION-GAP-FIX.4a — recover register-field bit positions from a register-layout diagram
//! by cumulative LSB tiling, gated so that nothing is ever fabricated.
//!
//! # Why this module exists
//!
//! Some specs place a register's bit positions ONLY in a rendered layout GRAPHIC (a horizontal
//! bit-field strip), not in the field-definition table. The deterministic table path then recovers
//! the field NAMES (and access/reset/description) but leaves every `bits_high`/`bits_low` empty —
//! the bit extents live in a modality the table parser cannot read (the honesty guardrail: you can
//! only extract a fact that is present in some readable modality). The bits are in the image.
//!
//! # Why we do NOT trust the image's absolute positions
//!
//! A local VLM reads field NAMES + ORDER + per-field WIDTHS off the layout graphic reliably, but it
//! misreads the ABSOLUTE bit positions of WIDE fields (it grabs the wrong one of the two edge labels
//! a wide cell prints, cascading an off-by-N). So we DISCARD the VLM's absolute positions and
//! reconstruct them from order + widths using a structural law the VLM cannot violate:
//!
//! > **A register's fields tile it from MSB to LSB with no gaps and no overlap.**
//!
//! # The two-gate anti-fabrication policy
//!
//! A reconstruction is ACCEPTED only when it is provably grounded:
//!
//! - **Gate (a) — standard-width tiling.** The proposed widths must sum to a STANDARD register
//!   width (8/16/32/64/128). Registers come in standard widths by universal digital-design
//!   convention (this is structural "how", like logic-level semantics — ADR 0006 — never a chip
//!   name). A misread width breaks the sum and is rejected.
//! - **Gate (b) — name agreement with the document.** The proposed field-name set must match the
//!   register's OWN field-definition table (derived from the document, never hardcoded). A
//!   hallucinated or dropped field breaks the match and is rejected.
//!
//! When either gate fails, the bits stay an honest residual — they are NEVER fabricated. This is
//! exactly why `dmstatus` (whose layout has reserved gaps the 20-field table does not name, and
//! whose wide reserved field the VLM misreads) yields a residual, while `dmcontrol` (whose 14 named
//! fields tile all 32 bits with no reserved gaps) recovers cleanly.
//!
//! The module is PURE: it performs no I/O and knows nothing about images, providers, or the VLM
//! transport. The command layer (`commands::recover_register_bits`) supplies the proposals from the
//! diagram reader and persists the result.

use std::collections::BTreeMap;

use crate::ir::source::RegisterRecord;

/// Standard register widths, in bits. A tiling whose field widths do not sum to one of these is not
/// trusted (gate (a)). These are universal digital-design register sizes — structural "how", not
/// document-specific vocabulary (ADR 0006), in the same spirit as the universal logic levels.
pub const STANDARD_REGISTER_WIDTHS: [u32; 5] = [8, 16, 32, 64, 128];

/// One field as read off the register-layout diagram: a name and a width, in MSB→LSB order.
///
/// The diagram reader (a VLM, in production) emits these left-to-right across the bit strip, i.e.
/// most-significant field first. The width is the number of bits the field's cell spans; absolute
/// positions are intentionally NOT carried here because the reader misreads them for wide fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterDiagramFieldProposal {
    /// Field identifier as labeled in the diagram (compared against the field-definition table).
    pub field_name: String,
    /// Field width in bits, as read from the diagram cell (≥ 1).
    pub width: u32,
}

/// A field's bit extent reconstructed by tiling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveredFieldBits {
    pub field_name: String,
    pub bits_high: u32,
    pub bits_low: u32,
    pub bit_width: u32,
}

/// The outcome of attempting bit recovery for one register — every non-recovery is an explicit,
/// honest reason (no silent fabrication, no silent skip).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterBitRecoveryOutcome {
    /// Bits attached to `count` fields (all of them) from the tiled reconstruction.
    Recovered { count: usize },
    /// The register already carried bit positions; recovery was not attempted.
    AlreadyComplete,
    /// The register has no fields to attach bits to.
    NoFields,
    /// The diagram reader proposed nothing.
    NoProposals,
    /// Gate (a) failed: the proposed widths do not tile a standard register width.
    ResidualNonStandardWidth { proposed_total: u32 },
    /// Gate (b) failed: the proposed field names do not match the field-definition table.
    ResidualNameMismatch,
}

impl RegisterBitRecoveryOutcome {
    /// A short, stable label for logging/reporting.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Recovered { .. } => "recovered",
            Self::AlreadyComplete => "already_complete",
            Self::NoFields => "no_fields",
            Self::NoProposals => "no_proposals",
            Self::ResidualNonStandardWidth { .. } => "residual_non_standard_width",
            Self::ResidualNameMismatch => "residual_name_mismatch",
        }
    }
}

/// Is `total` one of the standard register widths (gate (a))?
pub fn is_standard_register_width(total: u32) -> bool {
    STANDARD_REGISTER_WIDTHS.contains(&total)
}

/// Normalize a field name for set comparison: trim + ASCII-lowercase. Diagram labels and table
/// names routinely differ only in surrounding whitespace or case.
fn normalize_field_name(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

/// Reconstruct each field's bit extent by cumulative LSB tiling of MSB→LSB-ordered proposals.
///
/// Returns `Some(extents)` (in the same order as `proposals`) only when the widths tile a standard
/// register width exactly (gate (a)); otherwise `None` — an honest residual. A proposal list with a
/// zero width, or an arithmetic overflow, also yields `None`.
///
/// Tiling places the first (MSB) proposal in the top `width` bits and walks down toward bit 0:
/// for a 32-bit register `[w=1, w=1, w=10, w=10, …]` yields `[31:31], [30:30], [25:16], [15:6], …`.
pub fn reconstruct_bits_by_tiling(
    proposals: &[RegisterDiagramFieldProposal],
) -> Option<Vec<RecoveredFieldBits>> {
    if proposals.is_empty() || proposals.iter().any(|p| p.width == 0) {
        return None;
    }
    // Checked sum so an adversarial/garbled width set cannot overflow.
    let mut total: u32 = 0;
    for p in proposals {
        total = total.checked_add(p.width)?;
    }
    if !is_standard_register_width(total) {
        return None;
    }
    // `remaining_top` is the exclusive top of the not-yet-assigned span, counted down from `total`.
    // Each field claims `[remaining_top - width, remaining_top - 1]`; no subtraction can underflow
    // because `width <= remaining_top` holds while the running total is consumed, and the standard
    // -width gate guarantees `remaining_top` lands exactly on 0.
    let mut remaining_top = total;
    let mut recovered = Vec::with_capacity(proposals.len());
    for p in proposals {
        if p.width > remaining_top {
            return None; // defensive: cannot occur once the sum equals `total`
        }
        let bits_low = remaining_top - p.width;
        let bits_high = remaining_top - 1;
        recovered.push(RecoveredFieldBits {
            field_name: p.field_name.clone(),
            bits_high,
            bits_low,
            bit_width: p.width,
        });
        remaining_top = bits_low;
    }
    debug_assert_eq!(
        remaining_top, 0,
        "standard-width tiling must consume every bit"
    );
    Some(recovered)
}

/// Do the proposed field names match the register's field-definition table EXACTLY, as a multiset
/// of normalized names (gate (b))? Exact multiset equality — not subset — so a reader that invents
/// an extra field (e.g. a reserved cell absent from the table) or drops one is rejected. This is
/// what keeps a register with reserved gaps (whose layout names bits the table does not) honest.
pub fn proposal_names_match_fields(
    proposals: &[RegisterDiagramFieldProposal],
    fields: &[crate::ir::source::RegisterFieldRecord],
) -> bool {
    if proposals.is_empty() || fields.is_empty() || proposals.len() != fields.len() {
        return false;
    }
    let mut counts: BTreeMap<String, i64> = BTreeMap::new();
    for p in proposals {
        *counts
            .entry(normalize_field_name(&p.field_name))
            .or_insert(0) += 1;
    }
    for f in fields {
        *counts
            .entry(normalize_field_name(&f.field_name))
            .or_insert(0) -= 1;
    }
    counts.values().all(|&c| c == 0)
}

/// True when EVERY field of the register is missing an absolute bit position — the "bits live in the
/// graphic" shape this recovery targets. If any field already has bits, we do not touch the
/// register (deterministic table extraction is authoritative; we never override it).
fn all_fields_missing_bits(register: &RegisterRecord) -> bool {
    register
        .fields
        .iter()
        .all(|f| f.bits_high.is_none() && f.bits_low.is_none())
}

/// Attach diagram-recovered bits to a register's fields, gated by both anti-fabrication checks.
///
/// This is the pure decision core wired into the command layer: it takes the proposals the diagram
/// reader produced and the register's own field table, and either attaches a fully-tiled, name-
/// agreeing bit layout to every field or leaves the register untouched with an explicit residual
/// reason. It never fabricates a partial or guessed bit position.
pub fn recover_bits_for_register(
    register: &mut RegisterRecord,
    proposals: &[RegisterDiagramFieldProposal],
) -> RegisterBitRecoveryOutcome {
    if register.fields.is_empty() {
        return RegisterBitRecoveryOutcome::NoFields;
    }
    if !all_fields_missing_bits(register) {
        return RegisterBitRecoveryOutcome::AlreadyComplete;
    }
    if proposals.is_empty() {
        return RegisterBitRecoveryOutcome::NoProposals;
    }
    // Gate (a): the widths must tile a standard register width.
    let Some(recovered) = reconstruct_bits_by_tiling(proposals) else {
        let proposed_total: u32 = proposals.iter().map(|p| p.width).sum();
        return RegisterBitRecoveryOutcome::ResidualNonStandardWidth { proposed_total };
    };
    // Gate (b): the proposed names must match the register's own field table.
    if !proposal_names_match_fields(proposals, &register.fields) {
        return RegisterBitRecoveryOutcome::ResidualNameMismatch;
    }
    // Both gates passed — attach by normalized name. The multiset match guarantees a 1:1 pairing,
    // so every field receives exactly its tiled extent.
    let by_name: BTreeMap<String, &RecoveredFieldBits> = recovered
        .iter()
        .map(|r| (normalize_field_name(&r.field_name), r))
        .collect();
    let mut count = 0usize;
    for field in &mut register.fields {
        if let Some(bits) = by_name.get(&normalize_field_name(&field.field_name)) {
            field.bits_high = Some(bits.bits_high);
            field.bits_low = Some(bits.bits_low);
            field.bit_width = Some(bits.bit_width);
            count += 1;
        }
    }
    // Keep the register width consistent with the recovered layout when it was unknown.
    if register.size_bits.is_none() {
        register.size_bits = Some(proposals.iter().map(|p| p.width).sum());
    }
    RegisterBitRecoveryOutcome::Recovered { count }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::{AutomationConfidence, RegisterFieldRecord};

    fn proposal(name: &str, width: u32) -> RegisterDiagramFieldProposal {
        RegisterDiagramFieldProposal {
            field_name: name.to_string(),
            width,
        }
    }

    fn named_field(name: &str) -> RegisterFieldRecord {
        RegisterFieldRecord {
            field_name: name.to_string(),
            bits_high: None,
            bits_low: None,
            bit_width: None,
            access_type: None,
            reset_value: None,
            description: None,
            enumerated_values: Vec::new(),
        }
    }

    fn register(name: &str, fields: Vec<RegisterFieldRecord>) -> RegisterRecord {
        RegisterRecord {
            register_id: format!("regfld_{name}"),
            register_name: name.to_string(),
            offset_address: None,
            size_bits: None,
            fields,
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    /// The real RISC-V Debug `dmcontrol` layout, MSB→LSB (14 named fields tiling all 32 bits, no
    /// reserved gaps). Source: `test_data/llm_eval/seed_riscv_debug_registers.json`.
    fn dmcontrol_proposals_msb_to_lsb() -> Vec<RegisterDiagramFieldProposal> {
        vec![
            proposal("haltreq", 1),
            proposal("resumereq", 1),
            proposal("hartreset", 1),
            proposal("ackhavereset", 1),
            proposal("ackunavail", 1),
            proposal("hasel", 1),
            proposal("hartsello", 10),
            proposal("hartselhi", 10),
            proposal("setkeepalive", 1),
            proposal("clrkeepalive", 1),
            proposal("setresethaltreq", 1),
            proposal("clrresethaltreq", 1),
            proposal("ndmreset", 1),
            proposal("dmactive", 1),
        ]
    }

    fn dmcontrol_register() -> RegisterRecord {
        // The field-definition table carries the same 14 names but no bit positions (they live in
        // the layout graphic). Order is deliberately different from the diagram order.
        register(
            "dmcontrol",
            vec![
                named_field("dmactive"),
                named_field("ndmreset"),
                named_field("clrresethaltreq"),
                named_field("setresethaltreq"),
                named_field("clrkeepalive"),
                named_field("setkeepalive"),
                named_field("hartselhi"),
                named_field("hartsello"),
                named_field("hasel"),
                named_field("ackunavail"),
                named_field("ackhavereset"),
                named_field("hartreset"),
                named_field("resumereq"),
                named_field("haltreq"),
            ],
        )
    }

    #[test]
    fn tiling_reconstructs_dmcontrol_exact_bits() {
        let recovered = reconstruct_bits_by_tiling(&dmcontrol_proposals_msb_to_lsb())
            .expect("dmcontrol widths tile 32 bits");
        // Spot-check the wide fields and the two endpoints against the gold.
        let by_name: BTreeMap<&str, &RecoveredFieldBits> = recovered
            .iter()
            .map(|r| (r.field_name.as_str(), r))
            .collect();
        assert_eq!(
            (by_name["haltreq"].bits_high, by_name["haltreq"].bits_low),
            (31, 31)
        );
        assert_eq!(
            (
                by_name["hartsello"].bits_high,
                by_name["hartsello"].bits_low
            ),
            (25, 16)
        );
        assert_eq!(
            (
                by_name["hartselhi"].bits_high,
                by_name["hartselhi"].bits_low
            ),
            (15, 6)
        );
        assert_eq!(
            (by_name["dmactive"].bits_high, by_name["dmactive"].bits_low),
            (0, 0)
        );
        // No gaps, no overlap: union of all extents is exactly 0..=31.
        let mut covered: Vec<u32> = recovered
            .iter()
            .flat_map(|r| r.bits_low..=r.bits_high)
            .collect();
        covered.sort_unstable();
        assert_eq!(covered, (0..=31).collect::<Vec<_>>());
    }

    #[test]
    fn tiling_rejects_non_standard_width() {
        // dmstatus's reserved field misread (width 7 → 1) drops the sum to a non-power-of-two.
        let proposals = vec![proposal("a", 4), proposal("b", 16), proposal("reserved", 6)];
        assert_eq!(proposals.iter().map(|p| p.width).sum::<u32>(), 26);
        assert!(reconstruct_bits_by_tiling(&proposals).is_none());
    }

    #[test]
    fn tiling_rejects_zero_width_and_empty() {
        assert!(reconstruct_bits_by_tiling(&[]).is_none());
        assert!(reconstruct_bits_by_tiling(&[proposal("x", 0), proposal("y", 32)]).is_none());
    }

    #[test]
    fn tiling_accepts_every_standard_width() {
        for &w in &STANDARD_REGISTER_WIDTHS {
            let one = vec![proposal("whole", w)];
            let extents = reconstruct_bits_by_tiling(&one).expect("single full-width field tiles");
            assert_eq!((extents[0].bits_high, extents[0].bits_low), (w - 1, 0));
        }
    }

    #[test]
    fn name_match_requires_exact_multiset() {
        let fields = vec![named_field("a"), named_field("b"), named_field("c")];
        assert!(proposal_names_match_fields(
            &[proposal("A", 1), proposal("b", 1), proposal("C", 30)],
            &fields
        ));
        // Extra proposed field (reserved cell not in the table) → reject.
        assert!(!proposal_names_match_fields(
            &[
                proposal("a", 1),
                proposal("b", 1),
                proposal("c", 1),
                proposal("reserved", 29)
            ],
            &fields
        ));
        // Dropped field → reject.
        assert!(!proposal_names_match_fields(
            &[proposal("a", 1), proposal("b", 31)],
            &fields
        ));
        // Renamed/hallucinated field → reject.
        assert!(!proposal_names_match_fields(
            &[proposal("a", 1), proposal("b", 1), proposal("zzz", 30)],
            &fields
        ));
    }

    #[test]
    fn recover_attaches_all_dmcontrol_bits() {
        let mut reg = dmcontrol_register();
        let outcome = recover_bits_for_register(&mut reg, &dmcontrol_proposals_msb_to_lsb());
        assert_eq!(outcome, RegisterBitRecoveryOutcome::Recovered { count: 14 });
        // Every field now has bits, and the wide fields match the gold extents.
        assert!(
            reg.fields
                .iter()
                .all(|f| f.bits_high.is_some() && f.bits_low.is_some())
        );
        let by_name: BTreeMap<&str, &RegisterFieldRecord> = reg
            .fields
            .iter()
            .map(|f| (f.field_name.as_str(), f))
            .collect();
        assert_eq!(
            (
                by_name["hartsello"].bits_high,
                by_name["hartsello"].bits_low
            ),
            (Some(25), Some(16))
        );
        assert_eq!(
            (
                by_name["hartselhi"].bits_high,
                by_name["hartselhi"].bits_low
            ),
            (Some(15), Some(6))
        );
        assert_eq!(by_name["hartsello"].bit_width, Some(10));
        assert_eq!(reg.size_bits, Some(32));
    }

    #[test]
    fn recover_residual_on_dmstatus_reserved_width_misread() {
        // dmstatus's field table has 20 named fields; its layout adds reserved cells (bits 20:21
        // and 25:31). The VLM both adds those reserved cells AND misreads the wide one as width 1,
        // so the proposal neither tiles 32 nor matches the 20-field table → honest residual.
        let mut reg = register(
            "dmstatus",
            vec![
                named_field("version"),
                named_field("confstrptrvalid"),
                named_field("hasresethaltreq"),
                named_field("authbusy"),
                named_field("authenticated"),
            ],
        );
        // Proposal includes a reserved cell the table does not name; widths also fail to sum to 32.
        let proposals = vec![
            proposal("version", 4),
            proposal("confstrptrvalid", 1),
            proposal("hasresethaltreq", 1),
            proposal("authbusy", 1),
            proposal("authenticated", 1),
            proposal("reserved", 1), // wide reserved field misread as width 1
        ];
        let outcome = recover_bits_for_register(&mut reg, &proposals);
        assert!(matches!(
            outcome,
            RegisterBitRecoveryOutcome::ResidualNonStandardWidth { .. }
        ));
        // Nothing fabricated.
        assert!(
            reg.fields
                .iter()
                .all(|f| f.bits_high.is_none() && f.bits_low.is_none())
        );
        assert_eq!(reg.size_bits, None);
    }

    #[test]
    fn recover_residual_on_name_mismatch_even_when_widths_tile() {
        // Widths tile 32 exactly (gate a passes) but one proposed name is not in the field table.
        let mut reg = dmcontrol_register();
        let mut proposals = dmcontrol_proposals_msb_to_lsb();
        proposals[0].field_name = "bogusfield".to_string(); // hallucinated MSB name
        assert_eq!(proposals.iter().map(|p| p.width).sum::<u32>(), 32);
        let outcome = recover_bits_for_register(&mut reg, &proposals);
        assert_eq!(outcome, RegisterBitRecoveryOutcome::ResidualNameMismatch);
        assert!(reg.fields.iter().all(|f| f.bits_high.is_none()));
    }

    #[test]
    fn recover_skips_register_that_already_has_bits() {
        let mut reg = dmcontrol_register();
        reg.fields[0].bits_high = Some(0);
        reg.fields[0].bits_low = Some(0);
        let outcome = recover_bits_for_register(&mut reg, &dmcontrol_proposals_msb_to_lsb());
        assert_eq!(outcome, RegisterBitRecoveryOutcome::AlreadyComplete);
    }

    #[test]
    fn recover_handles_no_fields_and_no_proposals() {
        let mut empty = register("empty", Vec::new());
        assert_eq!(
            recover_bits_for_register(&mut empty, &dmcontrol_proposals_msb_to_lsb()),
            RegisterBitRecoveryOutcome::NoFields
        );
        let mut reg = dmcontrol_register();
        assert_eq!(
            recover_bits_for_register(&mut reg, &[]),
            RegisterBitRecoveryOutcome::NoProposals
        );
    }
}
