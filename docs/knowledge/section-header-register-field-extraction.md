---
id: section-header-register-field-extraction
title: GIC/SMMU/CoreSight/ACC/ARM-Debug register fields live in SECTION HEADINGS (`<NAME>, bits [hi:lo]`), not tables — read them into register_records, with a per-document name-uniqueness residual gate
answers:
  - "how are register fields written as section headings extracted"
  - "why do GIC/SMMU/CoreSight architecture-spec registers gain fields from section headings"
  - "what is the registers.section_header_field strategy"
  - "how does .10g differ from .10f (register vs message routing)"
  - "how is a duplicate register mnemonic (AUTHSTATUS/CSW/IDR reused per access-port block) handled"
  - "why are some section-heading registers held as a residual instead of emitted"
  - "how does a section-heading register avoid double-counting an existing register record"
  - "where is the shared section-heading container-walk that .10f and .10g both use"
date: 2026-06-17
tags: [registers, sections, prose, grammar, digestion, gic, smmu, coresight, arm-debug, container-routing, probe-first]
evidence: crates/specforge/src/ir/evidence.rs (extract_section_header_registers, scan_section_header_field_containers, distinct_section_header_fields, SectionHeaderRegisterExtractor, register_record_surface; shares parse_dotted_container_heading / parse_section_header_field / caption_names_register / is_register_attribute_heading / is_field_descriptions_anchor with .10f); docs/tasks/PDF-VARIANT-DIGESTION.md (.10g)
reverify: cargo test -p specforge --lib section_header_registers -- --nocapture ; cargo test -p specforge --lib section_header_register_corpus_sweep -- --ignored --nocapture (GIC 73/468, SMMU 88/381, CoreSight 5/24, ACC 2/4, ARM-Debug 12/57; only these 5 fire, DTI 0)
---

`PDF-VARIANT-DIGESTION.10g` (2026-06-17), the register-routed twin of
[[section-header-message-field-extraction]]. The same `<NAME>, bits [hi:lo]` section-heading
field layout that DTI uses for MESSAGE fields is also how ARM architecture specs (not TRMs)
write REGISTER fields: GIC (`ihi0069`), SMMU (`ihi0070`), CoreSight (`ihi0029`), ACC (`ihi0076`),
ARM-Debug-v6 (`ihi0074`) give each register a dotted-numbered container heading
(`B2.2.1 ABORT, Abort register` / `6.3.1 SMMU_IDR0`) with a `Field descriptions` anchor and one
section heading per field (`ORUNERRCLR, bit[4]`, `TERM_MODEL, bit [26]`). `.10f` routed these
register containers AWAY (deferred to `.10g`); `.10g` reads them into `register_records`.

**No-drift design (the [[byte-location-structure-field-extraction]] / `.10a` "one matcher" rule).**
`.10f` and `.10g` share ONE container-walk, `scan_section_header_field_containers`, which classifies
every dotted-numbered container ONCE as a register (its caption uses the whole word `register`, OR
it carries an `Attributes`/`Accessing` sub-heading) or a message (neither). `.10f` keeps the
non-register containers; `.10g` keeps the register ones. The field gate
(`distinct_section_header_fields`: dedup by distinct name, ≥2 required) is also shared, so the two
surfaces cannot drift apart. New strategy `registers.section_header_field` runs LAST in
`register_record_surface` (`run_surface_concat`).

**Per-document name-uniqueness residual gate (the decisive precision finding).** A short register
mnemonic is reused across access-port blocks — ARM-Debug reuses `AUTHSTATUS`/`CSW`/`IDR`/`CLAIMSET`/
`DEVARCH`, CoreSight reuses `AUTHSTATUS` — and the dotted heading carries only the short name, not
the block. Those occurrences are a MIX of identical cross-references, subset views, AND genuinely
different registers (`CSW` MEM-AP vs JTAG-AP have disjoint fields). Emitting them would either
over-count (identical/subset dups) or, via the existing all-distinct
`consolidate_register_field_fragments` merge, CONFLATE two different registers into a fabricated
mega-register. So `.10g` emits a register only when its name is UNIQUE within the document; a reused
name is held as an honest residual. Universal grammar over per-document name multiplicity; no
chip-name list (ADR 0006). **`.10h` (2026-06-24) then recovered the SAFE half of that residual —
identical cross-references + nested views collapse to one record by field-set containment, only the
genuinely-different (disjoint) registers stay residual; see [[section-header-register-identity-collapse]].**

**No double-count with the existing surface.** A `.10g` register whose unique name matches an
existing 0-field record (e.g. ARM-Debug `DPIDR`, minted name-only by the map/summary strategies)
MERGES via `consolidate_register_field_fragments` (0 fields are trivially distinct → safe → one
enriched record, the existing identity kept since `.10g` runs last). Live: ARM-Debug
`register_records` 29 → 37, fields 186 → 243, +8 brand-new registers (ABORT/BASE/CFG/DEVID/IDCODE/
MEMTYPE/PRIDR0/TARGETSEL), `DPIDR` enriched to 4 fields, ZERO duplicate names in the final set.

**Measured (`extract_section_header_registers` over the persisted corpus):** GIC **73 / 468**,
SMMU **88 / 381**, CoreSight **5 / 24**, ACC **2 / 4**, ARM-Debug **12 / 57** = **180 registers /
934 fields across exactly 5 docs**; every other doc fires 0, and DTI fires 0 registers (its
containers are message-routed) while keeping its 159 `.10f` message fields. Old-vs-new
`evidence --dry-run` over the wire gold + register golds (NVMe 42 reg / 216 msg, AMD 217 msg,
CCIX 131 reg / 92 msg, RISC-V 44 reg, APB/AHB/AXI/AXI-Stream) is byte-identical except the run
manifest gaining `registers.section_header_field` (produced 0); `.10f` message fields byte-identical
(DTI 159). WIRE-BASED-100 orthogonal; `kg-bench` 156/156; `run_ci.sh` green (lib 1672 → 1677).
access/reset/offset/description are honestly absent (`None`) — a heading states only name + bit range.
