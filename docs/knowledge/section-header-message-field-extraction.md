---
id: section-header-message-field-extraction
title: DTI-class message fields live in SECTION HEADINGS (`<NAME>, bits [hi:lo]`), not tables — read them into message_field_records with container-decides routing
answers:
  - "why does DTI (ihi0088) have zero message_field_records"
  - "how are message fields written as section headings extracted"
  - "where do DTI message field obligations leak (signal_constraints) and how is it fixed"
  - "how does the section-heading field reader tell a message container from a register container"
  - "what is the message_fields.section_header_field strategy"
  - "how are <NAME>, bit [N] / <NAME>, bits [hi:lo] section-heading field defs parsed"
  - "why are GIC/SMMU/CoreSight section-heading fields NOT message fields"
  - "how are spacing-artifact field names and the unit word Bits handled"
date: 2026-06-17
tags: [message-fields, sections, prose, grammar, digestion, dti, container-routing, probe-first]
evidence: crates/specforge/src/ir/evidence.rs (extract_section_header_message_fields, parse_dotted_container_heading, parse_section_header_field, normalize_section_field_name, is_section_header_field_name, is_message_container_name, is_field_descriptions_anchor, caption_names_register, is_register_attribute_heading, SectionHeaderMessageFieldExtractor, message_field_surface); docs/tasks/PDF-VARIANT-DIGESTION.md (.10f)
reverify: cargo test -p specforge --lib section_header_message_field -- --nocapture ; cargo test -p specforge --lib section_header_message_field_corpus_sweep -- --ignored --nocapture (DTI 159 fields / 17 containers, only DTI fires)
---

`PDF-VARIANT-DIGESTION.10f` (2026-06-17), spun from [[behavior-temporal-lowering-broader-corpus]].
AMBA DTI (`ihi0088`) is a message protocol (`DTI_TBU_TRANS_REQ`, `DTI_TBU_CONDIS_REQ`, …) but
carried **0 `message_field_records`** — so field obligations (`the MMUV field must be 0`) leaked
into `signal_constraints` with dotted/undeclared subjects (`DTI_TBU_TRANS_REQ.MMUV`). The
`.10a`–`.10e` readers all read TABLES and saw nothing, because **DTI's field layout is not in
tables at all**.

**Scoping correction (measurement-first).** The spun-out note guessed the fields lived in prose
`list_item`s of the form `"<Field>, bit [N]"`. Re-measuring the persisted `source_ir.json`
showed they do NOT: DTI gives each field **its own section HEADING** — `STAGES, bits [27:26]`,
`SPD, bit [25]`, `M_MSG_TYPE, bits [3:0]`, `IMPLEMENTATION DEFINED, bit [7]` — grouped under a
dotted-numbered message container (`3.1.1 DTI_TBU_CONDIS_REQ`) with a `Field descriptions` anchor
sub-heading. `list_item` matches are ~0; the `body_text` matches are descriptive prose
(`Messages with bits [3:0] equal to 0xE …`) and cross-refs — the noise the section-heading
restriction excludes. A heading is a cleaner anchor than free prose.

**Container-decides routing (the [[byte-location-structure-field-extraction]] / `.10c` rule —
register iff register-attribute vocabulary).** The SAME `<NAME>, bits [hi:lo]` heading shape
appears in register manuals (GIC 612 / SMMU 434 / CoreSight 42 / ACC 5 / ARM-Debug-v6 138; 1396
corpus-wide across 8 docs). A field's container = the nearest preceding dotted-numbered heading;
it is a REGISTER (skip — deferred to `.10g`) iff its caption uses the whole word `register` or
its sub-heading run carries an `Attributes` / `Accessing …` block, else a MESSAGE → emit. DTI's
message sections carry `Source`/`Usage constraints`/`Flow control result` (no register markers),
so they route to `message_field_records`. ADR-0006: universal section grammar, no chip-name list.

**Gate (each measured per-item):** (1) heading matches `<IDENT>, bit[s] [range]`, `^…$`-anchored,
identifier-shaped name (admits value-slice `TOK_TRANS_REQ[11:8]` + multi-word `IMPLEMENTATION
DEFINED`); (2) container not-register; (3) `Field descriptions` anchor present; (4) ≥2 DISTINCT
fields. Two name cleanups: a Docling spacing artifact `<ident> [slice]` normalizes to
`<ident>[slice]` (merges duplicate tokenizations); a name that is exactly the unit word
`bit`/`bits` (`Bits, bit [5:4]`) is an unnamed reserved range → dropped (honest residual). Bit
overlaps are KEPT — DTI documents Manager/Subordinate views of one position (`M_MSG_TYPE[3:0]`
vs `S_MSG_TYPE[3:0]`), both real.

**Measured:** DTI `message_field_records` **0 → 159 / 17 containers**; **only DTI fires (1/79)**.
Old-vs-new `evidence --dry-run` parity over the wire gold + table-message-field gold docs is
byte-identical except the run manifest gaining `message_fields.section_header_field` (produced 0);
NVMe 216 / AMD 217 ([[bit-position-structure-field-extraction]]) unchanged; ARM-Debug-v6's 138
headings → 0 message fields (register-routed). WIRE-BASED-100 orthogonal; `kg-bench` 156/156;
`run_ci.sh` green (lib 1672). Closing the recognition gap lets the LLM-primary constraint reader
type a DTI field obligation into `message_field_constraints` (the deterministic Pattern surface
does not consult the catalog, so the persisted leak is corrected on the next live-NLP DTI rebuild
— its normalized bundle is host-local-blocked). The register-routed section-heading form
(~1230 fields → the register surface) is the deferred sibling lever `.10g`.
