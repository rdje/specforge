---
id: property-table-is-not-a-signal-inventory
title: A configurable-property table whose option name contains "signals" satisfies the signal-caption gate, and only the caption's own word "property" separates it
answers:
  - "why were True and False known signal names in AXI"
  - "why did the emitted AXI isf carry a type named False"
  - "what is table_states_a_property_rather_than_a_signal_inventory"
  - "how does SpecForge tell a property table from a signal table"
  - "is first-header-appears-in-caption enough to detect a property table"
  - "why is APB Check signal descriptions not refused as a property table"
  - "how many property tables are admitted as signal tables corpus-wide"
  - "why did a junk polarity record appear after WIRE-BASED-100.10b"
date: 2026-09-11
status: current
tags: [evidence-ir, signal-tables, adr-0006, false-positive, wire-based-100, axi]
evidence: crates/specforge/src/ir/evidence.rs (table_states_a_property_rather_than_a_signal_inventory; should_treat_table_as_top_level_signal_description; a_property_table_is_not_a_signal_inventory; a_role_phrase_that_repeats_in_its_caption_is_still_a_role_phrase); docs/tasks/WIRE-BASED-100.md (.10e); generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json (table_0179, table_0181, table_0203, table_0237)
reverify: "cargo test --offline -p specforge-core --lib property && rebuild the AXI chain and confirm the emitted .isf carries no (type False …) and signal_polarities has no True record"
---

A specification with configurable options writes one small table per option. AXI's reads
`Table A12.16: Trace_Signals property`, with the header `Trace_Signals`, a `Default` column, and `True` /
`False` in the name column. Because the **option's own name** contains `Signals`, the caption satisfied the
ordinary signal-inventory gate, so `True` and `False` became known signal names for the whole document.

They mint no declaration — a value row states no width and no direction — which is exactly why this stayed
invisible. It surfaced only when `WIRE-BASED-100.10b` withheld a base-name template and the prose polarity
pass, re-reading the same table row, attributed `active_high` to `True`. The polarity record was the
symptom; the larger damage was an **enum literally named `False`**, which reached the emitted `.isf` as
`(type False (bits 2))` beside its enum block.

## Two conditions, because the shape test alone is wrong

The tempting structural test is *the first header occurs verbatim in its own caption* — a property table
heads itself with its subject, a signal table heads itself with a column role. Measured over all **602**
admitted `signal_description` tables in the persisted corpus, that selects **17 tables and most of them are
real**:

- APB's `Table 5-1 Check signal descriptions` is headed `Check signal` and declares `PADDRCHK`,
  `PCTRLCHK`, `PSELxCHK`. Refusing it would break a wire gold.
- CoreSight's `SPIDEN`/`HIDEN` and CHI's `BTI` tables are encodings headed by the signal they encode.

A role phrase that happens to repeat in its own caption is still a role phrase. Adding the document's own
word for what the table is — the caption's **last** word must be `property` — takes the selection to
**exactly 4, and all four satisfy the shape test as well**. Two independent conditions agreeing, with no
false positive left to measure. `property` is universal document grammar in the same class as the
`signal` / `port` / `pin` / `name` words this gate already reads; it is not a document, vendor, protocol or
symbol identity (ADR 0006).

## Where it lives

Unlike `[[base-name-template-table-is-not-a-catalogue]]`, this test is a pure function of one table and its
caption — it needs no document-wide inventory — so it sits directly in
`should_treat_table_as_top_level_signal_description`, the gate every concrete-signal consumer already
shares, and no threading was required.

## Measured effect

AXI: `signal_polarities` 45 → 44, statements 6,454 → 6,451 (exactly the three `Enum False …` facts), and the
emitted `.isf` drops `(type False (bits 2))` and its enum block, 12 → 11 types. Provenance, relations,
constraints, conditional rules, presence, channel memberships, hints, ports and rule count are byte-stable,
and nothing moves in the other 26 proof-carrying chains.

Links: [[base-name-template-table-is-not-a-catalogue]], [[alpha-variant-placeholder-is-not-a-wire]],
[[qualified-role-header-proves-no-role]].
