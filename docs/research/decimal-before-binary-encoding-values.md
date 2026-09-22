# Decimal before binary — what the unquoted ordering actually costs (`KG-ISF-COMPLETENESS.5.xi`)

`parse_encoding_numeric_literal` (`crates/specforge/src/ir/evidence.rs`) tries `parse::<u32>()`
**before** its all-`0`/`1` arm. The binary arm is therefore reachable only when the decimal parse
OVERFLOWS `u32` — never below eleven digits — so the radix of an unquoted cell is a function of the
string's LENGTH, with a discontinuity at 11 characters. `.5.viii` closed the quoted half of this
(`'1110'` is 14, `x'0A00'` is 2560) and handed the unquoted half here.

Reproduce every number below with:

```bash
python3 scripts/adjudicate_decimal_before_binary_encoding_values.py --residual
```

## The leaf's opening premise was wrong, and correcting it is the result

`.5.xi` was opened on a corpus-wide CELL census — **230 cells with a leading zero (2 replayable) and
943 without (156 replayable)** — and concluded from it that the defect *"reaches the current producer
in quantity"* and is *"the only one of the four open enum-reader defects"* that does. Both halves
re-derive; the conclusion does not follow from them.

A table cell becomes a published value only when it lands in the `value_col` that
`infer_encoding_column_indices` picks **for a table that actually mints an enum**. Almost every
all-`0`/`1` cell in this corpus is a bit position, a bit range or a slot number in a column no enum is
minted from. Replaying the producer per table and attributing each minted member to the row it came
from (`.5.vi`'s walk, reused unchanged) gives the number that is a fact about the product:

| census | leading zero | bare | total | replayable |
| --- | ---: | ---: | ---: | ---: |
| corpus cells, as the opening census counted them | 230 | 943 | 1,173 | 158 |
| corpus cells, trimmed as the PRODUCER trims | 230 | 1,010 | 1,240 | 158 |
| **minted member values** — the defect | **92** | **244** | **336** | **0** |

**Zero in the replayable stratum**, and not by attribution luck: the attribution-free upper bound —
every minted pair any table could produce from such a cell — is also 0. The reason is visible in the
value cells the current producer does read: of 126 parsing replay matches across the 22 replayable
enums, 94 are `0b`-prefixed, 4 are `0x`-prefixed, and the remaining 28 are single digits, which read
the same in either radix. **The replayable corpus states its codes in explicit notation.** The 158
"replayable cells" of the opening census are real cells in real tables; none of them is an encoding
table's value column.

### The producer trims brackets, and the opening census did not

`parse_encoding_numeric_literal` trims `[` and `]` before parsing, so `[10]` reaches the same decimal
branch as `10`. That is **67 cells** the opening census could not see. They matter in the direction
opposite to the one a larger population suggests: a bracketed cell is a bit POSITION, and bit
positions are exactly the decimal-intending population a repair must leave alone.

## The discriminator, and why a column is not enough

A leading zero is a fact about NOTATION — no publisher writes a decimal ten as `0010`. A bare `10`,
`11` or `101` is genuinely ambiguous in isolation, so the cell's column and its row are asked for a
witness. All three tests are content SHAPE; none reads a header word or a document key (ADR 0006).

- **Notational witness** — a digit-string cell, all-`0`/`1`, length ≥ 2, beginning with `0`, in the
  same column **at the same width** or anywhere in the same **row**. Zero-padding to a fixed width is
  what binary notation looks like.
- **Decimal refutation** — a digit-string cell in the same column at the same width carrying a digit
  ≥ 2. That is what zero-padded DECIMAL looks like (`01 … 09 10 11 12`), and it outranks the witness in
  both branches, or the repair would corrupt such a column.
- **Decimal fallback** — absent any witness, a digit ≥ 2 anywhere in the column. A column running
  `0 1 2 … 11` is a decimal RUN in which `10` and `11` are all-`0`/`1` by coincidence. This is the
  single largest decimal-intending class.

**Width stratification is load-bearing.** `opencapi_4_0` `table_0092` fuses a bit-position table
(`cmd_flag bit` = `3 2 1 0`) with an encode table (`0010`, `0011`) in one column. An unstratified
`digit ≥ 2` test refutes the binary reading of a cell the document plainly states in binary; the strata
are width 1 and width 4 and they disagree, which is the document's own layout, not noise.

**The row witness is not a refinement, it is 17 of the 22 residual slots.** A row like
`op0 | op1 | CRn | CRm | op2` = `11 | 011 | 0100 | 0100 | 000` (Cortex-A76 system-register encodings)
puts the zero-padded evidence in the *siblings*, never in `op0`'s own column, which holds one cell.
The same shape decides CHI `Opcode[5:3]` = `101` against its `Opcode[2:0]` sibling `000`.

### Result

| verdict | leading zero | bare | total |
| --- | ---: | ---: | ---: |
| binary — the published value is wrong today | 92 | 106 | **198** (58.9%) |
| decimal — the published value is right today | 0 | 133 | 133 (39.6%) |
| zero-padded decimal | 0 | 0 | 0 |
| undecidable — honest residual | 0 | 5 | 5 (1.5%) |

Of the 198, the leading-zero class (92) needs no witness at all. Of the 106 bare cells carried to a
binary verdict, **89 have a same-width column sibling** (3 of those also have a row sibling) and **17
rest on a row sibling alone**.

## Every contestable call, adjudicated by hand against the document

Exactly one binary verdict sits in a column that also holds a digit ≥ 2, and exactly 17 rest on a row
witness alone. All 18 were read against the source document:

| document / table | cell | today | stated | why |
| --- | --- | ---: | ---: | --- |
| `opencapi_4_0` `table_0092` `cmd_flag bit` | `0010`, `0011` | 10, 11 | 2, 3 | two tables fused; `3 2 1 0` are bit indices at width 1, the encodes are width 4 |
| `cortex_a76` `table_0147` and 7 siblings, `op0` | `11` | 11 | 3 | AArch64 sysreg encoding; `op1`/`CRn`/`CRm`/`op2` are `011`/`0100`/`0100`/`000` |
| `ihi0050_g` `table_0101` | `10`, `11` | 10, 11 | 2, 3 | CHI `RespErr`: `10`=DERR, `11`=NDERR, sibling `000` |
| `ihi0050_g` `table_0250` `Opcode[5:3]` | `101` | 101 | 5 | AtomicStore sub-opcode; sibling column `Opcode[2:0]` runs `000`…`111` |

The five residual slots were read the same way, and all five are genuinely DECIMAL — `Bits`/`Bit
Location`/`CCC Bit` columns holding a lone `11` or `10 11`, with no second cell to witness anything
(`gic_600` `table_0102`, `48882` `table_0276`, `ccix_rev_2_0` `table_0070` ×2, `jesd84_b50`
`table_0095`, whose own row says *"class 11"*). Leaving them unread as binary is therefore the correct
outcome, not merely the safe one: the residual costs nothing here.

## The repair cannot be enum-identity-additive, and that is checked

`.5.viii` had to be confined to the member-value seam because widening
`parse_encoding_numeric_literal` would have moved `derive_header_sourced_enum_name` clause 5 — a table
that starts PARSING starts NAMING an enum — the non-subtractive trap `.5.vi.a` was reverted for. This
repair has no such exposure, and the reason is structural rather than careful placement:

> For an all-`0`/`1` digit string, the decimal arm succeeds only at ten digits or fewer, where the
> binary reading is at most 1023 and cannot overflow. Above ten digits the decimal arm already fails
> and the binary arm already runs. **The set of cells that parse is identical under either ordering;
> only the returned value changes.**

Checked over all **71,001** distinct trimmed cells in the persisted corpus: **0** cells whose parse
SUCCESS would change. So `infer_encoding_column_indices`, `looks_like_encoding_literal` and clause 5
all see exactly what they see today, and no enum can appear, disappear, or be renamed.

## Sequencing, and what this does NOT argue

`.5.ix`'s rule — a gate landed now is inherited free by every future `CORPUS-COVERAGE` refresh — still
holds, but it is much weaker here than it was for `.5.viii`. `.5.viii` landed ahead of the refresh of
`opencapi_3_0_transaction_layer_28jan2020` because that one document carried **129** of its 357
recoverable values. The same document carries **1** affected member slot for this leaf. The legacy
mass is elsewhere: `ccix_base_specification_revision1_1_version1_0` (55), the three CHI C2C
specifications (80 between them), `jesd235a_2015_11_hbm2_dram` (28) and `cortex_a76` (25).

**WIRE-BASED-100 exposure is 0 slots.** The APB/AHB/AXI/AXI-Stream golds state their codes in `0b`/
`2'b` notation, which is why a movement there would be an alarm rather than a result.

So this repair is justified by correctness on 198 measured member values — `SNOOP_REQUEST = 10` where
`ccix` states `MsgType[3:0]` = `0010`, i.e. 2 — and not by a refresh deadline. **It is worse than a
fabricated value and that is the argument**: `.5.vii`'s census flags a fabricated ordinal, whereas a
mis-radixed cell publishes a number that LOOKS document-stated and no census questions.

## Authoritative tracking

`docs/tasks/KG-ISF-COMPLETENESS.md` — `.5.xi` (this measurement), `.5.xi.a` (the reader change).
Instrument: `scripts/adjudicate_decimal_before_binary_encoding_values.py`.
Predecessor measurement: `docs/research/enum-value-provenance.md` (`.5.vii`/`.5.viii`).
Knowledge Map: `[[decimal-before-binary-needs-a-notational-witness]]`,
`[[quoted-literal-notation-carries-the-radix]]`, `[[minted-enum-values-are-mostly-row-indices]]`.
