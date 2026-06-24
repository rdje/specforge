---
id: section-header-register-block-qualification
title: A genuinely-different register reused under one mnemonic (MEM-AP CSW vs JTAG-AP CSW) is recovered by BLOCK-QUALIFYING it with the parent section title's block name (.10i)
answers:
  - "how are two genuinely-different registers sharing a mnemonic (MEM-AP CSW vs JTAG-AP CSW) recovered instead of dropped"
  - "what does .10i do that .10h did not (block-qualified recovery of the disjoint register class)"
  - "where does the block name come from when the PDF backend flattens heading levels (parent section title via dotted-parent)"
  - "how does derive_register_block_name parse a block out of a register-descriptions section heading"
  - "why is a CSW@MEM-AP / CLAIMSET@AP qualified register name emitted, and is it .isf-safe"
  - "why does the D4.5 CLAIMSET (no block token) stay a residual under .10i"
date: 2026-06-24
tags: [registers, sections, prose, grammar, digestion, arm-debug, recall, residual, probe-first, isf]
evidence: crates/specforge/src/ir/evidence.rs (extract_section_header_registers, derive_register_block_name, section_dotted_number, dotted_parent, block_qualify_register_occurrences, push_section_header_register; builds on [[section-header-register-identity-collapse]] .10h and [[section-header-register-field-extraction]] .10g, shares scan_section_header_field_containers / distinct_section_header_fields); docs/tasks/PDF-VARIANT-DIGESTION.md (.10i)
reverify: cargo test -p specforge --lib section_header_registers -- --nocapture ; cargo test -p specforge --lib derive_register_block_name -- --nocapture ; cargo test -p specforge --lib section_header_register_corpus_sweep -- --ignored --nocapture (ARM-Debug 20/93; CoreSight 6/29, GIC 73/468, SMMU 88/381, ACC 2/4 unchanged)
---

`PDF-VARIANT-DIGESTION.10i` (2026-06-24), the follow-on to
[[section-header-register-identity-collapse]] (`.10h`). `.10h` recovered the *collapsible* half of the
`.10g` reused-mnemonic residual (identical/nested views of ONE register → one record), but left the
GENUINELY-DIFFERENT half — disjoint field sets under one mnemonic — fully dropped. `.10i` recovers it.

**Why `.10h` couldn't, and why `.10i` can.** `.10h` noted the PDF backend flattens every heading to
L1, so there is no ancestor-block heading *nested above* a register to qualify it. The `.10i`
measurement (the same `#[ignore]` block probe, read-only over persisted `source_ir`) refines that:
the block name is still printed in the document — as the **parent section TITLE**, a sibling line in
the flat list — and the **dotted-parent number** leads to it. Every reused register lives under a
heading of the universal form `<dotted-num> <BLOCK> register descriptions`:
`C2.6 MEM-AP register descriptions` → `MEM-AP`, `C3.5 JTAG-AP register descriptions` → `JTAG-AP`,
`C1.4 AP Register Descriptions` → `AP`. A parent with no block token (`D4.5 Register descriptions`)
honestly yields none.

**The mechanism.** `extract_section_header_registers` builds a `by_number` map (dotted section number
→ heading title) over `document_sections`, threads each container's own dotted number onto
`SectionHeaderFieldContainer.dotted` (additive — `.10f` ignores it), and tags each candidate with its
parent block via `dotted_parent` + `derive_register_block_name`. `derive_register_block_name` strips
the leading dotted number (`section_dotted_number`), then requires the remainder to be exactly
`<single-token BLOCK> register description(s)` (case-insensitive tail) — a multi-word lead-in or a
missing tail yields `None`. When `.10h` containment returns `None` (the genuinely-different class),
`block_qualify_register_occurrences` groups the occurrences by block and emits each block-named one as
`<NAME>@<BLOCK>`; a no-block occurrence stays residual, and ≥2 still-disjoint occurrences sharing one
block re-run `.10h` containment within the block (else residual — never conflated). Universal section
grammar over the document's own block headings; NO chip-name list (ADR 0006).

**`.isf`-safe by sanitization.** A qualified name reaches the `.isf` storage lowering, which applies
`sanitize_isf_name(register_name.to_lowercase())` → a valid identifier: `CSW@MEM-AP` → `csw_mem_ap`,
kept distinct from `csw_jtag_ap`. The ARM-Debug `debugger.isf` gains 5 distinct storage vars and
passes FSMGen `--strict --check` with 0 diagnostics.

**Measured.** The genuinely-different class lives in EXACTLY 1 doc — ARM-Debug `ihi0074`:
`CSW@MEM-AP`{11} + `CSW@JTAG-AP`{7} (disjoint) and `CLAIMSET@AP`/`@MEM-AP`/`@JTAG-AP`{2} (the fourth,
under the block-less `D4.5`, stays residual). `section_header_register_corpus_sweep`: ARM-Debug
15 → **20** registers / 69 → **93** fields; CoreSight 6/29, GIC 73, SMMU 88, ACC 2 byte-identical
(CoreSight's lone reused `AUTHSTATUS` is `.10h`-collapsible → never reaches `.10i`). Full `evidence`
register_records ARM-Debug 40 → **45**. A `git stash` baseline-vs-change full-`evidence` diff over the
8 `.10h` golds + CoreSight `ihi0029` is BYTE-IDENTICAL; only `ihi0074` changes — ADDing 5 records with
ZERO removals (all 40 baseline records preserved). `kg-bench` 156/156; `run_ci.sh` green. ADR-0006
(universal `<NUM> <BLOCK> register descriptions` grammar, no name list).
