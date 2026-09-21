# Enum values — where they come from, and how often the document never stated one (`KG-ISF-COMPLETENESS.5.vii`)

Read-only, `2026-09-21`, over all 78 persisted chains. Reproducer
`scripts/adjudicate_positional_enum_values.py`. Prior census
`scripts/measure_enum_member_value_fidelity.py` (`.5.v`, KM
`[[minted-enums-are-mostly-cross-table-conflations]]`); attribution reused from
`scripts/adjudicate_conflated_enum_identity.py` (`.5.vi`).

`.5.v` flagged **160 minted enums whose values are exactly `0..n-1` and appear in no column of
their source table**, and gave `.5.vii` one instruction: decide between refusing the value and
keeping it on a measurement rather than an intuition, because *"a positional gate firing on a real
code table would be worse than the residual"*.

This record answers that. The fallback is not a fringe behaviour to be gated — it produces **more
than half of every enum value SpecForge publishes**, and the direction the leaf was told to test is
settled against it by the document's own rows.

## The producer

`synthesize_encoding_declarations_for_enum` (`crates/specforge/src/ir/evidence.rs`):

```rust
let value: u32 = row
    .get(value_col)
    .and_then(|cell| parse_encoding_numeric_literal(&cell.text))
    .unwrap_or(row_idx as u32);
```

A cell the parser cannot read becomes the row's **ordinal**, silently and per row. The statement
`Enum <name> <member> = <value>.` has no way to say *"the document did not state a value"*, so the
ordinal is published in the same grammar as a real code, and everything downstream —
`build_symbol_definitions`, `IntentIR`, the `.isf` adapter — reads it as one.

## The instrument, and why it replaces `.5.v`'s shape test

`.5.v` inferred the fallback from the **shape** of the finished value list. That test is wrong in
both directions:

- it **misses** every enum that mixes stated and fabricated values, which no shape can separate,
  and every enum whose ordinals have gaps — `row_idx` indexes `table.body_rows`, while the `.5.ii`
  spine gate and the `.5.iii` `_WIDTH` gate **skip** rows, so a gated table's ordinals are not
  contiguous;
- it can **flag** a genuine `0,1,2,…` code table whose document really does state those codes.

This instrument does not infer. It **replays the producer** per table, carrying one extra bit per
member — whether `parse_encoding_numeric_literal` returned a value or the row index was substituted
— and attributes each minted member to the table the producer walked, so provenance is read off
rather than guessed.

**Cross-instrument agreement, checked rather than assumed.** Attribution is `.5.vi`'s, and it
partitions the population identically: 543 enums attributed per member (`.5.vi`: 498 ordered + 45
multipass), 7 declined (`.5.vi`: exactly 7 set-cover). A set cover names the contributing tables but
not which table produced **which** member, and this question is per member, so those 7 are declined
rather than guessed. The promoted-symbol total re-derives at **512** across 50 documents, matching
`.5.v`'s independently built census.

## Result

**10,189 of 19,023 attributed minted values — 53.6% — were never stated by the document.**

| per enum | enums | members | fabricated | promoted |
| --- | ---: | ---: | ---: | ---: |
| every value fabricated | **228** | 2,694 | 2,694 | 207 |
| some values fabricated | **218** | 15,282 | 7,495 | 214 |
| every value document-stated | **97** | 1,047 | 0 | 84 |
| declined (set-cover only) | 7 | 2,569 | — | 7 |

**Only 84 of the 512 `symbol_definitions` that reach `IntentIR` — 16% — come from an enum whose
every value the document actually states.** The 218 mixed enums are the class `.5.v` could not see
at all: a member the document does state sits beside one it does not, in one enum, in one grammar,
with nothing marking which is which.

### Why the cell did not parse

| the column was chosen because | fabricated values | |
| --- | ---: | ---: |
| a header word matched (`value`, `encoding`, `code`, `bit`, …) | **6,244** | 61.3% |
| `unwrap_or(0)` — nothing matched at all | 3,124 | 30.7% |
| column 0 sniffed as an encoding literal | 821 | 8.1% |

The majority is **not** a table without a value column. It is a column the header calls a value
column, holding something that is not a single literal:

| header-matched cell that does not parse | cells | |
| --- | ---: | ---: |
| a bit RANGE (`3:0`, `[31:16]`) | **2,579** | 41.3% |
| words (`RES0`, `P`, `False or Not specified`, `'1110'`) | 2,396 | 38.4% |
| several values in one cell (`NSH, SH`) | 488 | 7.8% |
| empty | 444 | 7.1% |
| a literal RANGE (`0b00-0b11`) | 118 | 1.9% |
| `-` / `N/A` / `None` | 104 | 1.7% |
| prose over 60 characters | 101 | 1.6% |
| a don't-care pattern (`0b1x`) | 14 | 0.2% |

**The largest single class is a register field-LAYOUT table.** `infer_encoding_column_indices`
accepts a header containing the substring `bit`, which matches `Bits` — the column holding
`31:16`. Such a table is not an encoding table at all: it lists a register's fields, and the
producer publishes each field NAME as an enum member whose value is the row's ordinal.

## The fairness test `.5.v` demanded

If the ordinal were what the document meant, the row should say so somewhere. It does not:

- **9,631 of 10,189 (94.5%)** — the fabricated number appears **nowhere in its own source row**.
- 558 (5.5%) — the number appears somewhere in the row, which is corroboration only in the weakest
  sense: a `4` in a width column does not make `4` the code.
- **66 (0.6%)** are the one structurally defensible case: a name cell carrying a bracketed index
  equal to the row ordinal (`HPROT[3]` at row 3), where the fallback is right because the table is
  ordered by bit position. Even there it is right about the *number* and wrong about the *kind* —
  `Enum HPROT BUFFERABLE = 2.` says `BUFFERABLE` is a value of `HPROT`, when it is bit 2 of it.

**So the fallback is never evidenced and is at best coincidentally correct.** `.5.v`'s warning —
that a gate might fire on a real code table — does not apply to removing it: a real code table
states its codes, they parse, and nothing about them changes.

## The signal that needs no interpretation

No encoding assigns one code to two members. A fabricated value equal to one the same enum states
elsewhere is therefore **wrong on the document's own evidence**, with no judgement call:

- **5,688 member slots in 143 enums** (141 promoted).
- **34 slots in 22 enums** are single-table — this leaf's defect alone.
- 5,654 slots in 121 enums are multi-table, where `.5.vi`'s conflation compounds it: `row_idx` is
  **table-local** while the enum is cross-table, so fusing fragments restarts the count.

### Which makes `.5.vi`'s "legitimate case" a correction, not a footnote

`.5.vi` measured the legitimate cross-table case as exactly **two** enums — `ARDOMAIN` and
`SYSCOACK`, both in the AXI gold `ihi0022_l`, both vouched for by the document's own
`Table … Continued from previous page` marker — and protected them. Their identity is legitimate.
**Their content is not, and the continuation is precisely what guarantees it:**

```text
SYSCOACK:  DISABLED = 0   CONNECT = 0   ENABLED = 1   DISCONNECT = 1
```

Every value duplicated, because each fragment's rows were numbered from zero independently.
`ARDOMAIN` is worse — `value_col` is the `ARDOMAIN 1` column holding `NSH, SH` (multi-value, never
parses) and `name_col` is therefore the *Description* column, so its members are whole sentences
(`READ_FROM_A_NON_SHAREABLE_OR_SYSTEM_LOCATION = 0`) with ordinal values. **A page-break
continuation is the one case where the positional fallback cannot be right**, and `.5.vi` selected
those two enums as the population to protect without looking at what they contained.

## Not every fabrication is a residual

**357 of the 10,189 (3.5%), across 3 OpenCAPI documents, are values the document DOES state** — as
`'1110'`, a quoted binary literal. `parse_encoding_numeric_literal` trims `[` and `]` and nothing
else, so the quotes defeat it and a stated code becomes an ordinal.

These are a **parser gap, not a fallback defect**, and the repair is to read the value, never to
drop the member. It carries one hazard worth pinning before anyone writes it: the fix must widen
`parse_encoding_numeric_literal` **only**. Widening `looks_like_encoding_literal` would move
`value_col`, hence `name_col`, hence the members themselves — the non-subtractive trap `.5.vi.a`
was reverted for. Owned by `KG-ISF-COMPLETENESS.5.viii`.

## Per-item, on the scored golds

| document | enum | n | fabricated | effect |
| --- | --- | ---: | ---: | --- |
| AXI `ihi0022_l` | `AWBURST` | 8 | 8 | emptied |
| AXI `ihi0022_l` | `AWDOMAIN` | 7 | 7 | emptied |
| AXI `ihi0022_l` | `ARDOMAIN` | 6 | 6 | emptied |
| AXI `ihi0022_l` | `AWSNOOP` | 5 | 4 | 1 member left |
| AXI `ihi0022_l` | `AWATOP` | 13 | 2 | 11 left; both removed slots collide |
| AXI `ihi0022_l` | `ARCACHE` | 12 | 2 | 10 left |
| AHB `ihi0033_c` | `HPROT` | 17 | 9 | 8 left; 4 removed slots collide |
| APB `ihi0024_e`, I2C `um10204` | — | 4 / 0 | 0 | unchanged |

`AWBURST` is the case to read first. Its table is a burst-address **example** (`AWADDR | AWSIZE |
AWLEN | AWBURST | 7 | 6 | … | 0`); `value_col` matched the column literally headed `AWBURST`, whose
cells are `INCR`/`WRAP` and never parse, which forced `name_col` to the `AWADDR` column. The minted
enum is

```text
Enum AWBURST 0X00 = 0.  0X01 = 1.  0X04 = 2.  0X06 = 3.  0X00 = 4.  0X04 = 5.  0X00 = 6.  0X08 = 9.
```

— members named after **addresses**, values that are **ordinals**, `0X00` appearing three times with
three different values, and the document's real `AWBURST` codes (`FIXED`, `INCR`, `WRAP`) nowhere in
it. It is not promoted, so it stops at `EvidenceIR`; it is in the AXI wire gold's evidence today.

**The negative control, run because `.5.vi` was blind behind a passing vacuity check.** On the
same two gold documents the instrument reports **zero** fabricated values for `BRESP` (8), `ARSNOOP`
(4), `AWCMO` (2), `BTAGMATCH` (9), `HTRANS` (4), `HSIZE` (6), `HRESP` (2) and APB `PPROT` (4) —
including the enums `.5.ii` and `.5.iii` were written to repair. An instrument that flagged 53.6% of
the corpus and could not clear a clean code table would be measuring something else; these clear.

`AWATOP` shows the rule working correctly: `0b01exxx` and `0b10exxx` are parametric patterns, not
numbers, and the fallback makes them `1` and `2` — while the same document's `Table A6.9` states
`ATOMICSTORE` at `0b010000` (16). Removing the fallback removes exactly those two contradictions and
keeps `ATOMICSWAP = 48`, `ATOMICCOMPARE = 49` and all eight of `Table A6.9`'s clean codes.

## Decision

**GO on removing the positional fallback**: a member whose value does not parse is not minted.

The alternative the leaf was opened to weigh — keep the fallback because it is right often enough —
is refused by its own number: the ordinal is uncorroborated by its own row 94.5% of the time,
structurally defensible 0.6% of the time, and provably wrong in 5,688 slots.

**Predicted effect, to be proved by replay and not trusted from here:** 10,189 enum statements
removed, 228 enums emptied (207 of them names that reach `IntentIR` today), 218 narrowed, 97
untouched. The rule is **purely subtractive at the member seam** — it can only `continue` past a
row, so it cannot rename an enum, change which declared signal wins, or mint one. That argument is
why it is safe to attempt; it is **not** evidence, and `.5.vi.a` is the standing reason to say so:
that leaf's rule was also reasoned subtractive, was measured additive, and was reverted.

### Deliberately not bundled

- **Re-pointing `value_col`** away from a `bit`-headed layout column. Real, and the largest cause
  at 41.3%, but changing `value_col` changes `name_col` and therefore the members — non-subtractive,
  and it must be measured on its own. Removing the fallback subsumes its **symptom** meanwhile,
  because a bit-range cell never parses.
- **Reading the quoted literal** (357 values) — `KG-ISF-COMPLETENESS.5.viii`.
- **The conflation seam** — `.5.vi` / `.5.vi.a`, still sequenced after this leaf.

## Authoritative tracking

`docs/tasks/KG-ISF-COMPLETENESS.md` — `.5.vii` (this measurement), `.5.vii.a` (the code slice),
`.5.viii` (the quoted-literal parser gap). Knowledge Map:
`[[minted-enum-values-are-mostly-row-indices]]`,
`[[minted-enums-are-mostly-cross-table-conflations]]`, `[[caption-label-is-not-table-identity]]`,
`[[generic-enum-conflation]]`.
