---
id: minted-enum-values-are-mostly-row-indices
title: 10,189 of 19,023 minted enum values (53.6%) are the table's ROW INDEX, not a value the document stated — and only 84 of the 512 symbol_definitions reaching IntentIR carry no fabricated value
answers:
  - "where does a minted enum value come from"
  - "why does an enum get values 0 1 2 3 when the document states none"
  - "what is the positional value fallback in evidence.rs"
  - "how many minted enum values are fabricated"
  - "is the enum value fallback ever correct"
  - "how do I census enum value provenance"
  - "why do two enum members have the same value"
  - "can a minted enum value be trusted"
  - "what does row_idx do in synthesize_encoding_declarations_for_enum"
  - "why does a Bits column become an enum value column"
  - "are the stated_continuation enums clean"
  - "why is a quoted binary literal not parsed"
date: 2026-09-21
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, census, fidelity]
evidence: scripts/adjudicate_positional_enum_values.py; docs/research/enum-value-provenance.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.vii); crates/specforge/src/ir/evidence.rs (synthesize_encoding_declarations_for_enum, infer_encoding_column_indices, parse_encoding_numeric_literal)
reverify: "python3 scripts/adjudicate_positional_enum_values.py — expect 550 minted enums, 543 attributed per member and 7 declined, 19,023 attributed members of which 10,189 fabricated; per-enum 228 every-value-fabricated / 218 mixed / 97 fully stated; 5,688 colliding slots in 143 enums; 357 quote-wrapped recoverable; 94.5% uncorroborated by their own row"
---

`synthesize_encoding_declarations_for_enum` (`crates/specforge/src/ir/evidence.rs`) mints
`Enum <name> <member> = <value>.` with

```rust
.and_then(|cell| parse_encoding_numeric_literal(&cell.text))
.unwrap_or(row_idx as u32)
```

so a cell the parser cannot read becomes the row's **ordinal**, silently and per row. The statement
grammar cannot say *"the document stated no value"*, so the ordinal is published exactly like a real
code.

## Measured `2026-09-21` over all 78 persisted chains

| per enum | enums | members | fabricated | promoted |
| --- | ---: | ---: | ---: | ---: |
| every value fabricated | **228** | 2,694 | 2,694 | 207 |
| some values fabricated | **218** | 15,282 | 7,495 | 214 |
| every value document-stated | **97** | 1,047 | 0 | 84 |

**Only 84 of the 512 `symbol_definitions` reaching `IntentIR` (16%) come from an enum whose every
value the document states.** The 218 MIXED enums are invisible to a shape test: a stated value sits
beside a fabricated one in one enum with nothing marking which is which — which is why `.5.v`'s
`0..n-1` heuristic counted 160 where replaying the producer counts 10,189 members.

## The fallback is never evidenced

- **94.5%** of fabricated values appear **nowhere in their own source row**.
- **0.6%** (66) are the one defensible case — a bracketed index equal to the row ordinal
  (`HPROT[3]` at row 3) — and even there the number is right while the KIND is wrong: `BUFFERABLE`
  is bit 2 of `HPROT`, not a value of it.
- **5,688 slots in 143 enums are provably wrong**: the ordinal equals a value the same enum states
  elsewhere, and no encoding assigns one code twice. `row_idx` is **table-local** while an enum can
  be cross-table, so fusing fragments restarts the count — 5,654 of those slots are multi-table.

## The biggest cause is a header word, not a missing column

61.3% of fabrications come from a column a header word **matched**. `infer_encoding_column_indices`
accepts the substring `bit`, which matches `Bits` — the `31:16` column of a register field-LAYOUT
table. Those cells never parse, so every field name in such a table is published as an enum member
valued by its ordinal. Re-pointing `value_col` is **not** a safe repair: it moves `name_col` too,
and therefore the members.

## Two corrections this produced

- **`.5.vi`'s two protected `stated_continuation` enums are both corrupt.** `SYSCOACK` is
  `DISABLED=0 CONNECT=0 ENABLED=1 DISCONNECT=1` — every value duplicated, because each fragment was
  numbered from zero. A page-break continuation is the one case where the fallback **cannot** be
  right. Identity being legitimate says nothing about content.
- **357 fabrications (3.5%, 3 OpenCAPI documents) are not residuals at all** — the document states
  `'1110'` and `parse_encoding_numeric_literal` trims only `[` and `]`, so the quotes defeat it. The
  repair is to READ the value, and it must widen the parser **only**: widening
  `looks_like_encoding_literal` would move `value_col`, hence the members.

Links: [[minted-enums-are-mostly-cross-table-conflations]], [[caption-label-is-not-table-identity]],
[[generic-enum-conflation]].
