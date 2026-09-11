---
id: alpha-variant-placeholder-is-not-a-wire
title: A token with one interior lower-case position that wildcards onto two declared names is the document's placeholder for a signal family, and a relation must not declare it
answers:
  - "why did AxLEN become an AXI interface port"
  - "is there an AXLEN signal in the AMBA AXI specification"
  - "what is an alpha-variant placeholder"
  - "why does SpecForge only treat an INTERIOR lower-case position as a placeholder"
  - "can an actor-signal relation declare a signal name no table declares"
  - "why is nRESET not treated as a metavariable"
  - "why is PSELx not treated as a metavariable"
  - "how many placeholder tokens exist corpus-wide"
  - "what declaration catalog does the placeholder check read"
date: 2026-09-11
status: current
tags: [evidence-ir, declarations, actor-graph, adr-0006, false-positive, wire-based-100, axi]
evidence: crates/specforge/src/ir/evidence.rs (is_alpha_variant_placeholder; synthesize_directions_from_relations; an_alpha_variant_placeholder_is_not_a_wire); docs/tasks/WIRE-BASED-100.md (.10b); generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json (elem_00503, elem_00823, table_0020)
reverify: "cargo test -p specforge-core --lib an_alpha_variant_placeholder_is_not_a_wire; then rebuild the AXI chain and confirm no AxLEN declaration statement and no AxLEN interface port, while AWLEN and ARLEN both remain declared."
---

AMBA AXI says, in its own words: *"Length is communicated using the AWLEN and ARLEN signals … In this
specification, **AxLEN** indicates AWLEN and ARLEN."* It then uses the placeholder in ordinary normative
prose — *"A Manager that only issues requests of Length 1 can omit the AxLEN outputs from its interface"* —
which the relation extractor reads correctly as `Manager drives AxLEN`.

**A relation then became a declaration.** `synthesize_directions_from_relations` promotes a Drives triple
into a formal `Signal X is output.` statement, and a formal declaration is exactly what authorizes an
interface signal. That is the one path in EvidenceIR that can mint a name the document's tables never
declared, and it is how `AxLEN` reached the emitted AXI interface. `AXLEN` does **not** exist anywhere in
the specification — not in a table, not in a prose element — so any published claim that "the declared
AXLEN does not reach the interface" is false and withdrawn.

## The test

A token is a placeholder when the document itself supplies the family:

- it is **not** in the table declaration catalog (a declared name is a wire, whatever its spelling);
- some **interior** position holds a lower-case ASCII letter;
- at least two upper-case letters sit elsewhere in the token;
- wildcarding exactly that position matches **at least two declared names of the same length**.

`AxLEN` → `A?LEN` → `AWLEN`, `ARLEN`. Corpus-wide the rule selects **exactly one token**.

## Interior is load-bearing

The two edge positions already mean something else in hardware naming, and neither is a metavariable:

- a **leading** lower-case letter is the active-LOW convention — `nRESET` beside a declared
  `NRESET`/`PRESET` pair would otherwise qualify and be deleted;
- a **trailing** one is the indexed-family convention (`PSELx`), whose recovery `WIRE-BASED-100.4a` owns.

Only a substitution *inside* the identifier is a stand-in for a varying character.

## Read the declaration catalog, not the name-column universe

The first implementation consulted `collect_signal_names_from_tables`, which scoops the first column of
every gated table whether or not the row declares anything. AXI's `Table A6.5: Signals that should be the
same in an exclusive sequence` is a bare 5-column grid of `Ax*` metavariables, so `AxLEN` was "known" there
and the check refused itself. The predicate must read `table_signal_declaration_provenance` — the names a
table actually declared.

Links: [[base-name-template-table-is-not-a-catalogue]], [[semantic-interface-authority-empty-fallback]],
[[dense-prose-false-signal-loop-reaches-isf]].
