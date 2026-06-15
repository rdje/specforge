---
id: value-binder-alphabetic-whole-word
title: The deterministic value binder matches an alphabetic enum value as a whole word (no substring fabrication)
answers:
  - "how does extract_discovered_state_value_from_text match a constraint value"
  - "why was NVMe SANICAP must_be_value NO removed / where did the bogus NO come from"
  - "why does an alphabetic constraint value require a word boundary but a numeric value does not"
  - "what does lead_binds_value do in evidence.rs"
  - "why does shall be 0h still bind the value 0 but shall be non-zero does not bind NO"
  - "EXTRACTION-QUALITY-GAUGE.3f what is the alphabetic-value word-boundary gate"
date: 2026-06-15
tags: [extraction-quality, constraints, value-binding, evidence-ir, adr-0006, anti-fabrication]
evidence: crates/specforge/src/ir/evidence.rs (extract_discovered_state_value_from_text, lead_binds_value); crates/specforge/src/ir/evidence.rs tests (alphabetic_value_is_not_lifted_from_a_longer_word, alphabetic_value_binds_as_a_whole_word, numeric_value_still_binds_before_a_radix_suffix); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3f)
reverify: cargo test -p specforge --lib "ir::evidence::tests::nlp_classification::alphabetic_value" 2>&1 | tail -4
---

`extract_discovered_state_value_from_text` (the deterministic value binder used by
`extract_dynamic_signal_constraints`, `dyn_sigcon_*`) recovers a constraint's bound value by
looking for a value the document itself used, sitting behind a normative lead phrase
(`must be` / `shall be` / `must remain` / `shall remain` `<value>`, plus the `is <value> when`
form). The match goes through `lead_binds_value`, which enforces a **whole-word boundary after
the value only when the value ends in a letter**:

- An **alphabetic** enum value (`NO`, `YES`, `VALID`) must match a whole word — the character
  immediately after it cannot continue an identifier. This is the `EXTRACTION-QUALITY-GAUGE.3f`
  fix: the old plain-substring match lifted `NO` out of *"this field shall be **no**n-zero"* and
  minted the fabricated fact `SANICAP must_be_value NO`. The real obligations there are
  *"shall be non-zero"* / *"shall be cleared to 0h"* — neither is `must_be_value NO`.
- A **numeric** value stays lenient (no trailing boundary required), so a radix-suffixed literal
  like *"shall be **0**h"* (NVMe `ELEN`/`RECFMT`, genuine) still binds the value `0`. A blanket
  after-boundary rule was measured and rejected precisely because it dropped those genuine `0h`
  obligations.

Why letter-only and not "always require a boundary": measurement over all 78 persisted evidence
docs showed the blanket rule removed 10 records, several genuine (`ELEN`/`RECFMT` "shall be 0h"),
while the letter-only rule removes exactly **1** record (the NVMe `SANICAP NO` fabrication) and
**0** genuine facts. It is universal grammar — no signal/value name lists (ADR 0006) — and a
no-op on the wire specs (APB/AHB/AXI/SWD): a fresh Pattern rebuild of all four has zero alphabetic
`must_be_value` records the gate would alter, so the wire-doc 1.000 gold bar is structurally safe.

Sibling residuals left as honest gaps (different structural shapes, each a separate measured
slice, no denylist): the wrong-*subject* prose class (`NVM`/`FFFF`/`LBA` lifted from descriptive
command sentences) and the spurious-*digit*-value class on descriptive field cells (`HMDLLA 1`,
`MPS 0`). See [[extraction-quality-gauge-standing]].
