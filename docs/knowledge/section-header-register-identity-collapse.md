---
id: section-header-register-identity-collapse
title: A register mnemonic reused across blocks (ARM-Debug/CoreSight AUTHSTATUS/IDR/CSW) is resolved by FIELD-SET CONTAINMENT — identical/nested views collapse to one record, disjoint registers stay residual (.10h)
answers:
  - "how is a register mnemonic reused across access-port blocks (AUTHSTATUS/CSW/IDR/DEVARCH/CLAIMSET) recovered instead of dropped"
  - "what does .10h do that .10g did not (block-qualified register-mnemonic recovery)"
  - "how does collapse_section_header_register_identity decide same-register vs different-register"
  - "why do nested register views (AUTHSTATUS) collapse but disjoint ones (MEM-AP CSW vs JTAG-AP CSW) stay a residual"
  - "why is the block name not used to qualify the duplicate registers (heading levels flattened)"
  - "how is a fabricated mega-register / over-count avoided when recovering reused register mnemonics"
date: 2026-06-24
tags: [registers, sections, prose, grammar, digestion, arm-debug, coresight, recall, residual, probe-first]
evidence: crates/specforge/src/ir/evidence.rs (extract_section_header_registers, collapse_section_header_register_identity, section_header_register_block_probe_local_measurement; builds on [[section-header-register-field-extraction]] .10g, shares scan_section_header_field_containers / distinct_section_header_fields); docs/tasks/PDF-VARIANT-DIGESTION.md (.10h)
reverify: cargo test -p specforge --lib section_header_registers -- --nocapture ; cargo test -p specforge --lib section_header_register_corpus_sweep -- --ignored --nocapture (ARM-Debug 15/69, CoreSight 6/29; GIC 73/468, SMMU 88/381, ACC 2/4 unchanged)
---

`PDF-VARIANT-DIGESTION.10h` (2026-06-24), the follow-on to
[[section-header-register-field-extraction]] (`.10g`). `.10g` reads register fields written as
section headings (`<NAME>, bits [hi:lo]`) into `register_records`, but DROPPED every register
mnemonic reused across ≥2 register-routed containers (ARM-Debug `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/
`DEVARCH`; CoreSight `AUTHSTATUS`) because the dotted heading carries only the short name, and a
blind merge would either conflate two genuinely-different registers or over-count identical
cross-references. `.10h` recovers the SAFE half of that residual.

**The resolver (field-set containment, not the block name).** A probe
(`section_header_register_block_probe_local_measurement`) established two facts over the persisted
`source_ir`: (1) the reused mnemonics live in EXACTLY 2 docs (ARM-Debug `ihi0074`, CoreSight
`ihi0029`); (2) the heading levels are FLATTENED to L1 by the PDF backend, so there is NO
ancestor-block heading to qualify with — only the dotted-number hierarchy and the parent section
title (`C2.6 MEM-AP register descriptions`) survive. So `.10h` disambiguates by the one structural
signal that is always present and faithful: the FIELD SET. `collapse_section_header_register_identity`
returns the single MAXIMAL occurrence (largest field set, earliest in document order among ties)
IFF every occurrence's field set — compared by uppercased field name — is a subset of it; otherwise
`None`.

**The three sub-classes (per-occurrence field-set audit).**
- **Identical cross-reference** (`IDR` C1.4 ≡ C2.6, `DEVARCH` C1.4 ≡ C3.5) → one cluster → collapse
  to ONE record (no over-count).
- **Nested views** (`AUTHSTATUS` {NSID,NSNID} ⊂ {NSID,NSNID,SID} ⊂ {…,SNID} ⊂ {HID,…}) → one
  cluster with a single maximum → collapse to ONE record carrying the FULLEST occurrence's REAL
  layout (never a fabricated union — the emitted fields are an actual document occurrence).
- **Genuinely different** (MEM-AP `CSW` 11 fields vs JTAG-AP `CSW` 7 fields — DISJOINT; `CLAIMSET`
  claim-tag pair vs a 4-field variant) → no common superset → `None` → honest residual (collapsing
  would conflate; the flattened hierarchy gives no clean per-occurrence block qualifier).

**Measured.** `section_header_register_corpus_sweep`: ARM-Debug 12 → **15** registers / 57 → **69**
fields (+`AUTHSTATUS`/`DEVARCH`/`IDR`), CoreSight 5 → **6** / 24 → **29** (+`AUTHSTATUS`); GIC 73,
SMMU 88, ACC 2 byte-identical (no reused names). Full `evidence --dry-run` register_records:
ARM-Debug 37 → **40**, CoreSight 26 → **27**. A `git stash` baseline-vs-change full-`evidence` diff
over 8 golds (CCIX r1.0, NVMe register golds; AXI, AHB wire golds; GIC, SMMU, ACC section-header
no-dup; DTI `.10f` message-field) is BYTE-IDENTICAL; the only 2 changed docs ADD records with ZERO
removals and every baseline record byte-identically preserved → pure recall gain, no conflation.
`kg-bench` 156/156; `run_ci.sh` green (lib 1718 → 1721). ADR-0006 (universal field-set containment,
no name list).

**Update (`.10i`, 2026-06-24):** the "no clean per-occurrence block qualifier" remark above was too
pessimistic — see [[section-header-register-block-qualification]]. The block name IS recoverable, not
from a *nested* ancestor heading (those are flattened), but from the parent SECTION TITLE reached via
the dotted-parent number (`C2.6.7` → `C2.6 MEM-AP register descriptions` → `MEM-AP`). `.10i` uses it
to block-qualify the genuinely-different class this card left residual (`CSW@MEM-AP`/`CSW@JTAG-AP`,
`CLAIMSET@AP`/`@MEM-AP`/`@JTAG-AP`), so ARM-Debug rises 15 → 20 registers / 69 → 93 fields.
