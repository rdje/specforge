---
id: dotted-cross-reference-subject-gate
title: A constraint subject that is only ever a Reg.Field dotted cross-reference is dropped (not the cell's subject)
answers:
  - "why was NVMe MPS must_be_value 0 removed / where did the bogus MPS subject come from"
  - "what does is_dotted_cross_reference_subject do in evidence.rs"
  - "how does the extractor avoid minting a constraint about a cross-referenced register field"
  - "why is a pure-hex-literal subject filter unsafe (CBA, BADD)"
  - "EXTRACTION-QUALITY-GAUGE.3g what is the dotted-cross-reference spurious-subject gate"
  - "how is a Reg.Field cross-reference distinguished from a real constraint subject"
date: 2026-06-15
tags: [extraction-quality, constraints, subject-filtering, evidence-ir, adr-0006, anti-fabrication]
evidence: crates/specforge/src/ir/evidence.rs (is_dotted_cross_reference_subject, both subject_signals.retain sites in extract_dynamic_signal_constraints and extract_signal_constraints); crates/specforge/src/ir/evidence.rs tests (dotted_cross_reference_subject_is_dropped, standalone_or_own_subject_is_kept); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3g)
reverify: cargo test -p specforge --lib "ir::evidence::tests::nlp_classification::dotted_cross_reference_subject_is_dropped" 2>&1 | tail -4
---

`is_dotted_cross_reference_subject(text, subject)` (`ir/evidence.rs`) is a `EXTRACTION-QUALITY-GAUGE.3g`
constraint-precision gate, applied as a `subject_signals.retain(…)` in BOTH value-binding extractors
(`extract_dynamic_signal_constraints` `dyn_sigcon_*` and `extract_signal_constraints` `sigcon_*`), right
after the `.3e` descriptive-cell-body gate.

The problem it fixes: register and structure specs cross-reference *another* register's field with dotted
notation. NVMe's `BADD` cell reads *"Buffer Address (BADD): … aligned to the memory page size
(`CC.MPS`). The least significant bits … shall be 0."* The genuine obligation is about `BADD`
(alignment), but the subject collector also picked up `MPS` — the trailing half of the cross-reference
`CC.MPS` — and minted a fabricated `MPS must_be_value 0`. `.3e` does not catch it because the cell has no
*"This field `<verb>`"* marker (it reads *"(BADD): Indicates …"*).

The gate is structurally decidable and conservative: drop `subject` only if it is a plain identifier AND
**every** whole-word occurrence in the source is immediately preceded by `"<identifier>."` (a dotted
reference). A subject that appears standalone even once — its own declaration/mention — is always kept,
so the cell's real subject (`BADD`, written *"(BADD):"*) is untouched. Universal `Reg.Field`
cross-reference grammar, no name lists (ADR 0006).

A tempting alternative — "drop any all-hex-looking subject as a literal" (to kill the sibling `FFFF`
case) — was **rejected by measurement**: the real fields `CBA` (Controller Base Address) and `BADD`
(Buffer Address) are spelled with only hex letters, so a pure-hex filter would delete genuine
obligations. Measuring the 4 corpus hits (3 genuine) killed that design before any code.

Wire-safe by construction and by measurement: a wire spec's signal never appears only as a dotted
reference, so a fresh Pattern rebuild of all four wire docs has zero records the gate alters (proven),
and every wire-based gold gate stays at 1.000. Net live effect (with `.3f`): NVMe 20 → 18. See
[[value-binder-alphabetic-whole-word]] and [[extraction-quality-gauge-standing]].
