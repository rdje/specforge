---
id: base-name-template-table-is-not-a-catalogue
title: A signal table whose names the document instantiates with a shared prefix is a naming template, and only the mirror test separates it from a real two-level port list
answers:
  - "why did VALID PENDING CRDT CRDTSH SHAREDCRD RP become AXI interface ports"
  - "what is a base-name template table"
  - "how does SpecForge tell a generic channel-signal template from a signal catalogue"
  - "why is a suffix-of-other-declared-names rule not enough to detect a naming template"
  - "what is the mirror test for signal tables"
  - "why are CoreSight SDC-600 TX_VALID and EXT_TX_VALID both real declarations"
  - "where does SpecForge withhold template declarations"
  - "how many tables corpus-wide are base-name templates"
  - "why can a base-name template rule not live in the SourceIR table classifier"
  - "what happens to a constraint whose subject is a withheld template base name"
date: 2026-09-11
status: current
tags: [evidence-ir, declarations, signal-tables, adr-0006, false-positive, wire-based-100, axi]
evidence: crates/specforge/src/ir/evidence.rs (base_name_template_tables; shared_instantiation_prefixes; withhold_base_name_template_declarations; a_base_name_table_the_document_instantiates_is_a_template; a_mirrored_port_list_is_two_real_levels_not_a_template); docs/tasks/WIRE-BASED-100.md (.10b); generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json (table_0011); generated/source_ir/101130_0002_02_2018_11_30_coresight_sdc_600_technical_reference_manual/source_ir.json (table_0037, table_0063)
reverify: "cargo test -p specforge-core --lib base_name_template; then rebuild the AXI chain (evidence -> validate -> semantic -> validate -> intent -> validate -> adapt) and confirm the emitted interface carries no VALID/PENDING/RP/CRDT/CRDTSH/SHAREDCRD port and that table_0011 contributes zero table_signal_declaration_provenance records."
---

A specification that defines a repeated per-channel signal pattern writes the pattern **once**, with its
members spelled as base names, and instantiates each member with the channel's prefix. AMBA AXI's
`Table A2.3: Credited channel signals` lists `VALID`, `PENDING`, `RP`, `CRDT`, `CRDTSH` and `SHAREDCRD`;
the wires are `AWVALID`, `ARVALID`, `WCRDT`, and so on. Read as a catalogue, the table put six ports into
the manager's emitted interface **beside their own instantiations**, and carried 9 signal constraints and
23 ISF rules addressed to wires that do not exist.

## The obvious rule is wrong, and the corpus says so

The cheap shape rule — *a declared name that is a proper suffix of at least two other declared names is a
base name* — selects, over all 1,669 table declarations in the 26 persisted documents that have any,
**10 tables worth 42 declarations. Nine of those tables are real.** CoreSight SDC-600 declares `TX_VALID`,
`TX_READY`, `TX_LINKEST`, `TX_LINKUP` as one component's own ports and `EXT_TX_VALID` / `INT_TX_VALID` as
the two wrappers' ports. Both levels are wires. Shipping the shape rule alone would have deleted 36 real
declarations to remove 6 false ones.

## The mirror test is what separates them

Hierarchical qualification **re-declares the same port list one level up**, so the document contains a
table (`table_0063`) whose entire declared set *is* the prefixed copy of the base table. A naming pattern
has no such mirror: its members are scattered through the larger, heterogeneous per-channel inventories
(`AWVALID` sits in `Table B1.1: Write request channel signals` next to `AWADDR` and `AWID`).

A table is therefore a template when **all four** hold:

1. it declares at least 3 distinct names;
2. every one is a proper suffix of at least 2 names declared by **other** tables;
3. at least 2 prefixes instantiate **every** member — one shared family, not unrelated affix overlap;
4. **no** other table's whole declared set is contained in one prefix's instantiation of this table.

Corpus-wide that selects **exactly one table and six declarations**: AXI `table_0011`. Structural only —
affix relations among the document's own declared names, no vocabulary and no document identity (ADR 0006).

## Where it lives, and why not earlier

The rule needs the document's **complete** declared inventory, so it runs last in
`synthesize_signal_declaration_seed`, after every table producer. It cannot live in the SourceIR table
classifier for the reason `[[sourceir-classification-is-per-record]]` records: `classified_table_kind` is
replayed one record at a time and may not read neighbours.

Withholding the declarations is not enough on its own. `synthesize_directions_from_relations` promotes a
Drives triple into a formal `Signal X is output.` declaration, so the same rows come straight back through
the relation path unless the template's table-sourced relations are dropped with them. The convergence loop
therefore takes the whole `TableDeclarationCatalog` — declared names, template table ids, withheld names —
and filters the known-signal set, the width map and the table relations together.

## What withholding costs, honestly

The template's own normative prose goes with it. *"VALID signals must be LOW during reset"* is a real rule
about every channel's VALID, and with the base name withheld it is dropped rather than expanded over the
instantiating prefixes. Nine AXI constraints and 23 ISF rules left this way, every one of them addressed to
a base name. Expanding them into per-channel obligations is a **capability the template detection now makes
possible** and nothing implements — tracked as `WIRE-BASED-100.10d`.

Links: [[sourceir-classification-is-per-record]], [[alpha-variant-placeholder-is-not-a-wire]],
[[qualified-role-header-proves-no-role]].
