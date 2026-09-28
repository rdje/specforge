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
  - "why did the radix census report 0 replayable member slots"
date: 2026-09-22
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, parser, fidelity, notation, measurement]
evidence: crates/specforge/src/ir/evidence.rs (parse_encoding_numeric_literal, parse_encoding_value_literal, infer_encoding_column_indices, derive_header_sourced_enum_name); scripts/adjudicate_decimal_before_binary_encoding_values.py; docs/research/decimal-before-binary-encoding-values.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.xi)
reverify: "python3 scripts/adjudicate_decimal_before_binary_encoding_values.py --residual — expect (re-derived 2026-09-29, EXTRACTION-REPRESENTATION-GAPS.3.c.i) corpus cells 230 leading-zero / 1,010 bare trimmed (1,173 untrimmed, the figure the leaf opened on), 309 minted member slots, 10 of them replayable (all CHI C2C, all binary), adjudication 182 binary / 122 decimal / 0 zero-padded-decimal / 5 undecidable, and 0 of 70,995 distinct cells whose parse SUCCESS changes under the reordering. For the SHIPPED reader: cargo test --offline --workspace --exclude specforge-production-graph --lib decimal_before_binary (the 12 .5.xi.a tests, including decimal_before_binary_is_parse_success_invariant and the_unquoted_radix_stays_out_of_the_enum_identity_seams)."
---

`parse_encoding_numeric_literal` tries `parse::<u32>()` before its all-`0`/`1` arm, so an unquoted
`0010` reads as **10** and `1110` reads as **1110**. The binary arm runs only when the decimal parse
overflows `u32`, i.e. never below eleven digits.

**SHIPPED `2026-09-22` (`.5.xi.a`)**: `parse_encoding_member_value` now reads the value cell in the
radix the TABLE states, at the member seam only. The rule is the witness rule below; the product replay
is byte-identical because the replayable reach was 0, which is the predicted result and not a no-op.

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

## The replay must read values the way the producer does (`2026-09-29`, `EXTRACTION-REPRESENTATION-GAPS.3.c.i`)

The census attributes minted members with `replay_table_with_provenance`, and that replay kept reading value
cells with `parse_encoding_numeric_literal` after `.5.xi.a` shipped. On the 27 chains replayable on `2026-09-22`
that cost nothing, which is why "0 replayable" was true then. `EXTRACTION-REPRESENTATION-GAPS.3.a` made CHI C2C
replayable on `2026-09-26`, and its producer reads `MsgType`, `Opcode2` and `ActTrigger` codes (`10`, `11`, …) as
binary, as it should; the replay read them as decimal, could not attribute them, and the census kept printing 0.
The replay now uses the producer's own `parse_encoding_member_value` (radix witness, then the quoted literal of
`.5.viii`), single-sourced in `scripts/adjudicate_positional_enum_values.py`: **10 replayable slots, all binary and
all already read correctly by the product.** The legacy stratum is unchanged by the fix (299). The total was 336 on
`2026-09-22` and 299 just before the fix; that movement is in the legacy stratum and was not decomposed here. A mirror that lags its producer by one clause manufactures
the finding it is used to refute ([[stage-seal-closure-membership]] is the same session's other instance of a
check that looked at the wrong thing).

Related: [[quoted-literal-notation-carries-the-radix]], [[minted-enum-values-are-mostly-row-indices]],
[[caption-label-is-not-table-identity]].
