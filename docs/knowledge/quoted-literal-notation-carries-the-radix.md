---
id: quoted-literal-notation-carries-the-radix
title: A single-quoted value cell states its RADIX, not just its digits — stripping the wrapper and re-parsing is wrong on 314 of 357 slots because `parse_encoding_numeric_literal` tries decimal first
answers:
  - "why is a quoted binary literal not parsed"
  - "what does parse_encoding_value_literal do"
  - "why is '1110' fourteen and not one thousand one hundred ten"
  - "how does SpecForge read a quoted encoding value"
  - "what radix does a single-quoted cell use"
  - "what does x'0A00' mean in an encoding table"
  - "why does stripping quotes give the wrong enum value"
  - "is the all-zero-one binary arm of the encoding parser reachable"
  - "why does a bare binary cell read as decimal"
  - "which seam reads the quoted encoding notation"
  - "can widening the encoding parser mint a new enum"
  - "why is the quoted notation not given to parse_encoding_numeric_literal"
  - "how do I verify a value-correctness repair"
  - "is a count-based acceptance bar enough for a parser fix"
date: 2026-09-21
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, parser, fidelity, notation]
evidence: crates/specforge/src/ir/evidence.rs (parse_encoding_value_literal, parse_encoding_numeric_literal, synthesize_encoding_declarations_for_enum, derive_header_sourced_enum_name); docs/research/enum-value-provenance.md (§`.5.viii`); docs/tasks/KG-ISF-COMPLETENESS.md (.5.viii); scripts/adjudicate_positional_enum_values.py
reverify: "cargo test --offline -p specforge-core --lib parse_encoding_value_literal (5 tests) and quoted_notation_stays_out_of_the_enum_identity_seams (the drift guard). For the population: python3 scripts/adjudicate_positional_enum_values.py — expect 357 recoverable of 10,139 fabricated in 3 documents. The 357 are all LEGACY, so `evidence --dry-run` over every generated/source_ir/*/source_ir.json still replays 27 documents byte-identically and refuses 51 as schema-1; that is the expected result, not a no-op."
---

`parse_encoding_numeric_literal` (`crates/specforge/src/ir/evidence.rs`) tries `parse::<u32>()`
**before** its all-`0`/`1` binary arm. A digit string of only `0` and `1` therefore always parses as
**decimal**, and the binary arm is reachable only when the decimal parse OVERFLOWS `u32` — that is,
**never below eleven digits**:

| cell | the function returns | the document means |
| --- | ---: | ---: |
| `0010` | 10 | 2 |
| `1110` | 1110 | 14 |
| `11111111111` | 2047 | 2047 |

The radix is thus a function of the string's LENGTH, with a discontinuity at 11 characters.

## The wrapper is the radix

The three OpenCAPI transaction-layer specifications state the convention in their own notation
section: *"Binary values in sentences are shown in single quotation marks. For example '1010'."* and
*"Hexadecimal values are preceded by an x and enclosed in single quotation marks. For example
x'0A00'."*

So `'1110'` is **14** and `x'0A00'` is **2560**. Stripping the wrapper does not merely fail to help —
it DISCARDS the one token that says how to read the digits. `KG-ISF-COMPLETENESS.5.vii` prescribed
exactly that repair on the reasoning that *"a bare `'1110'` already parses as binary under the
all-`0`/`1` arm"*; it does not, and the repair would have been **wrong on 314 of 357 slots (88.0%)**
and strictly WORSE than the fabricated ordinal it replaced on **57** of them.

## A count-based bar cannot verify a value repair

`.5.vii` also wrote the acceptance bar as *"the quote-wrapped count must go to 0 and the fabricated
total fall by exactly 357"*. **Both the correct repair and the 88%-wrong one satisfy it identically.**
When a slice changes what a value IS, the oracle must compare VALUES; a population count only proves
that something was read, never that it was read correctly.

## Read it at the member seam only

`parse_encoding_value_literal` is called from `synthesize_encoding_declarations_for_enum` and nowhere
else, deliberately. The shared `parse_encoding_numeric_literal` has two other callers and one of them
MINTS: `derive_header_sourced_enum_name` clause 5 asks whether any value cell parses, so a table that
starts parsing starts NAMING an enum — enum-IDENTITY-additive, the non-subtractive trap `.5.vi.a` was
reverted for. `looks_like_encoding_literal` feeds the column-0 sniff in
`infer_encoding_column_indices`, so widening it would move `value_col`, hence `name_col`, hence the
members. `quoted_notation_stays_out_of_the_enum_identity_seams` pins both against drift.

Related: [[minted-enum-values-are-mostly-row-indices]],
[[minted-enums-are-mostly-cross-table-conflations]], [[caption-label-is-not-table-identity]].
