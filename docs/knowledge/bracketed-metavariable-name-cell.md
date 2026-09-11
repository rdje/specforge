---
id: bracketed-metavariable-name-cell
title: Trimming a name cell's leading token before judging it is what lets a bracketed metavariable become a declared signal
answers:
  - "why did a signal named `name` appear in the Avalon interface specification"
  - "why did a signal named `any` appear in an EvidenceIR artifact"
  - "does SpecForge declare a signal for a `<name> _in` template row"
  - "what is a bracketed metavariable name cell"
  - "why does trimming a name token hide a placeholder"
  - "what does the drop reason name_is_a_placeholder mean"
  - "why is the metavariable test run after the identifier test"
  - "how many bracketed placeholder name cells exist corpus-wide"
  - "does the placeholder rule use a list of placeholder words"
  - "how do I re-derive the declaration-row notation populations"
date: 2026-09-11
status: current
tags: [evidence-ir, declarations, signal-tables, adr-0006, false-positive, signal-declaration-row-drop]
evidence: crates/specforge/src/ir/evidence.rs (leading_name_token_is_placeholder; signal_names_in_name_cell; synthesize_signal_declarations; a_bracketed_metavariable_name_cell_declares_no_signal; a_bracketed_non_identifier_keeps_its_original_drop_reason; the_corpus_template_rows_declare_no_signal); scripts/measure_declaration_row_notations.py; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2a); generated/source_ir/683091_18_1_2021_12_25_avalon_interface_specifications/source_ir.json (table_0030, table_0031)
reverify: "python3 scripts/measure_declaration_row_notations.py — expect 12 bracketed leading name tokens, 5 of which pass the identifier test; then cargo test -p specforge-core --lib the_corpus_template_rows_declare_no_signal"
---

A specification routinely documents a *family* of signals with one row about a stand-in name. A
tristate conduit is written as three rows — `<name> _in`, `<name> _out`, `<name> _outen`, described as
*"the input signal of a logical tristate signal"* — where `<name>` is whatever the integrator calls
it. That is a template the reader expands, not a wire the document declares.

`synthesize_signal_declarations` declared it anyway, and the mechanism is worth stating exactly
because it is invisible at the call site. `signal_names_in_name_cell` takes the cell's first
whitespace token and `trim_matches`-es every non-identifier character off **both ends** before
anything judges it. That trim exists for footnote markers (`HSELx a`), and it is not the defect. The
defect is that `is_hardware_signal_token` then sees `name`, an entirely ordinary identifier, with no
trace of the `<` and `>` that were the document's only marker that the token is a metavariable. **A
wrapper that has to be removed before a token looks like a name is evidence that it is not one.**

The rule is therefore to judge the leading token *as written*: wrapped in a matched bracket pair
(`<>`, `()`, `[]`, `{}`) around a non-empty body, it is a metavariable and the row declares nothing,
recorded as `name_is_a_placeholder` with the cell text kept verbatim. Shape only — the delimiters
decide, never the word between them, so there is no placeholder vocabulary to maintain (ADR 0006).

**The test runs after the identifier test, and that ordering is load-bearing.** Corpus-wide there are
**12** bracketed leading name tokens but only **5** that pass the identifier test and can therefore
reach a declaration: `<name> _in`/`_out`/`_outen`, `<any>`, and `(varies)`. The other seven — a bit
range `[15:8]` under a `Bits` header, `[<domain>_]mbistaddr[variable:0]` — were never declaration
candidates. Running the metavariable test first would relabel all seven in the accounting while
changing no row's fate, making the reason vocabulary noisier for nothing.

Measured through the reader on the corpus rows (guard removed, then restored), **two phantom
declarations disappear**: `name` from `table_0031` and `any` from `table_0030`. Both documents are
legacy proofless, so nothing in the rebuildable corpus moves — all 24 re-runnable specifications
produce a byte-identical `EvidenceIR` before and after.

This is the third distinct way one document has tried to hand SpecForge a name that is not a wire, and
they share no mechanism: `[[alpha-variant-placeholder-is-not-a-wire]]` is an interior lower-case
position wildcarding onto two declared names, `[[base-name-template-table-is-not-a-catalogue]]` is a
whole table the document instantiates with a shared prefix, and this one is explicit BNF-style
bracketing in a single cell. Expect a fourth; do not expect a single test to cover them.
`[[declaration-reader-drops-uninterpretable-rows]]` owns the loss this was found while repairing.
