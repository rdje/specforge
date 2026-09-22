---
id: decimal-before-binary-needs-a-notational-witness
title: An unquoted all-`0`/`1` encoding cell reads as DECIMAL, wrong on 198 of 336 minted member values — and the radix is decided by a zero-padded witness in the same column width or the same ROW, never by the header word
answers:
  - "which radix does an unquoted encoding cell use"
  - "why is 0010 read as ten"
  - "how does SpecForge decide the radix of an unquoted encoding cell"
  - "what is the notational witness rule"
  - "is a bare 10 in an encoding table binary or decimal"
  - "how many minted enum values are mis-radixed"
  - "does the decimal-before-binary defect reach the current producer"
  - "why is a corpus cell census not a measure of a producer defect"
  - "can reordering the encoding parser arms mint a new enum"
  - "is the decimal-before-binary repair enum-identity-additive"
  - "what discriminates a bit-position column from a binary encoding column"
  - "why must the radix test be stratified by cell width"
  - "which bit-position columns must a radix repair leave alone"
  - "how do I re-derive the .5.xi population"
date: 2026-09-22
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, parser, fidelity, notation, measurement]
evidence: crates/specforge/src/ir/evidence.rs (parse_encoding_numeric_literal, parse_encoding_value_literal, infer_encoding_column_indices, derive_header_sourced_enum_name); scripts/adjudicate_decimal_before_binary_encoding_values.py; docs/research/decimal-before-binary-encoding-values.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.xi)
reverify: "python3 scripts/adjudicate_decimal_before_binary_encoding_values.py --residual — expect corpus cells 230 leading-zero / 1,010 bare trimmed (1,173 untrimmed, the figure the leaf opened on), 336 minted member slots with 0 replayable, adjudication 198 binary / 133 decimal / 0 zero-padded-decimal / 5 undecidable, and 0 of 71,001 distinct cells whose parse SUCCESS changes under the reordering."
---

`parse_encoding_numeric_literal` tries `parse::<u32>()` before its all-`0`/`1` arm, so an unquoted
`0010` reads as **10** and `1110` reads as **1110**. The binary arm runs only when the decimal parse
overflows `u32`, i.e. never below eleven digits.

## A cell census is not a producer defect

`.5.xi` opened on **1,173 corpus cells, 158 of them replayable**, and concluded the defect reaches the
current producer in quantity. Both counts re-derive. The conclusion does not follow: a cell is
published only if it lands in the `value_col` of a table that MINTS an enum, and nearly every
all-`0`/`1` cell in this corpus is a bit position, bit range or slot number. Replaying the producer and
attributing each minted member to its row gives **336 minted member values, 0 of them replayable** —
the attribution-free upper bound is 0 too. The replayable corpus states its codes as `0b`/`0x`
literals (94 + 4 of 126 matches) or single digits (28), which are radix-invariant.

**Count the population the SEAM reads, not the population the corpus holds.**

The producer also trims `[` and `]` before parsing, which the opening census did not: `[10]` is 67
further cells, and they are bit POSITIONS — the decimal-intending class a repair must not disturb.

## The radix is decided by a witness, and the witness may be in the ROW

| test | shape |
| --- | --- |
| notational witness | an all-`0`/`1` digit cell, length ≥ 2, starting `0`, in the same column **at the same width** or anywhere in the same **row** |
| decimal refutation | a digit cell in the same column at the same width carrying a digit ≥ 2 (that is zero-padded decimal) — outranks the witness |
| decimal fallback | absent a witness, any digit ≥ 2 in the column: a run `0 1 2 … 11` is decimal and its `10`/`11` are all-`0`/`1` by coincidence |

Verdicts over the 336: **198 binary, 133 decimal, 5 undecidable** (all 5 genuinely decimal, so the
residual costs nothing).

**Width stratification is load-bearing**: `opencapi_4_0` `table_0092` fuses a bit-index table
(`3 2 1 0`, width 1) with an encode table (`0010`, `0011`, width 4) in ONE column.

**The row witness is 17 of the 22 otherwise-undecidable slots**: Cortex-A76 `op0 | op1 | CRn | CRm |
op2` = `11 | 011 | 0100 | 0100 | 000` puts the zero-padding in the siblings, and `op0`'s own column
holds one cell. CHI `Opcode[5:3]` = `101` is decided the same way, by `Opcode[2:0]` = `000`.

## Unlike `.5.viii`, this repair cannot touch the identity seams

For an all-`0`/`1` string the decimal arm succeeds only at ten digits or fewer, where the binary
reading is at most 1023 and cannot overflow; above ten digits the decimal arm already fails and the
binary arm already runs. **The set of cells that parse is identical under either ordering; only the
value changes.** Verified over 71,001 distinct trimmed corpus cells: 0 divergences. So
`derive_header_sourced_enum_name` clause 5, `infer_encoding_column_indices` and
`looks_like_encoding_literal` all see exactly what they see today, and no enum can be minted, dropped
or renamed — the trap `.5.viii` had to be confined away from does not exist here.

Related: [[quoted-literal-notation-carries-the-radix]], [[minted-enum-values-are-mostly-row-indices]],
[[caption-label-is-not-table-identity]].
