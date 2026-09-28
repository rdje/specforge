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
  - "how much of the minted enum surface does the current reader actually produce"
  - "can a census over the persisted corpus describe the current producer"
date: 2026-09-21
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, census, fidelity]
evidence: scripts/adjudicate_positional_enum_values.py; docs/research/enum-value-provenance.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.vii); crates/specforge/src/ir/evidence.rs (synthesize_encoding_declarations_for_enum, infer_encoding_column_indices, parse_encoding_numeric_literal)
reverify: "python3 scripts/adjudicate_positional_enum_values.py — expect TODAY (re-derived 2026-09-29, EXTRACTION-REPRESENTATION-GAPS.3.c.i) 522 minted enums across 48 documents, 514 attributed per member and 8 declined, 17,998 attributed members of which 9,588 fabricated, 203 every-value-fabricated / 199 mixed / 112 fully stated, and the stratum 32 enums / 179 members replayable (2 fabricated) against 490 / 20,392 legacy. `.3.c.i` made the replay read values as the producer does (the .5.xi.a radix witness and the .5.viii quoted literal), which attributes CHI C2C MsgType, Opcode2 and ActTrigger (+3 attributed, +16 members); the rest of the movement from the 2026-09-22 figures (555 / 548 / 7 / 19,026 / 10,139 / 224 / 214 / 110) is legacy-stratum drift not decomposed here. `.5.xiii.a` moved those from 552 / 545 / 19,009 / 107 by naming AXI's members from the document's own column and recovering three encodings (RRESP, ARTAGOP, AWTAGOP); `fabricated` did not move. Two earlier sets of figures in this card are dated MEASUREMENTS and do NOT re-derive: the pre-`.5.vii.a` 550 / 543 / 19,023 / 10,189 / 228 / 218 / 97 reproduce only at bfe5cb23^, and the post-`.5.vii.a` 545 / 538 / 18,969 / 100 only between bfe5cb23 and 0d828e7e. `.5.vi.a` then ADDED 7 enums and 40 members by recovering encodings a caption label had hidden, which is why the totals grew while `fabricated` did not move. The residual is legacy: 10,137 of the 10,189 were in the 51 chains unrebuildable at the time."
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

## The totals below are facts about the CORPUS, not about the reader

`CORPUS-CHAIN-CURRENCY.11`: a persisted legacy `evidence_ir.json` is evidence about ITSELF. **The
current producer mints 25 enums / 136 members across the 27 replayable documents (re-derived
`2026-09-22` after `.5.xiii.a`); the other 530 enums
/ 21,459 members (99.4%) are legacy artifacts** from binaries that no longer exist. The defect is
still real independently of them — 52 fabricated values in 20 enums on the replayable stratum alone —
but the SCALE below describes the corpus on disk. [[legacy-artifact-declaration-drift]]

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
  repair is to READ the value. **The repair prescribed here was wrong** and `.5.viii` landed the
  correct one: stripping the wrapper discards the RADIX, because the shared reader tries decimal
  first, so `'1110'` would have become 1110 rather than 14 — wrong on 314 of the 357.
  [[quoted-literal-notation-carries-the-radix]] owns that question and its seam analysis.

## Landed `2026-09-21` — and why the landing is 54, not 10,189

`.5.vii.a` removed the fallback. **99.5% of the defect is unreachable**: 10,137 of the 10,189 values
sit in the 51 legacy chains that cannot be rebuilt without re-ingest, so a replay can only observe
**54 statements across 4 documents** (AXI 29, AHB 11, ADIv6 10, Wishbone 4) — 0 added, 0 non-`Enum`.
An acceptance bar written from the on-disk total would have been unverifiable by construction.
**Check which stratum a defect lives in before writing the bar for fixing it.**

**The census under-counts, and knows by how much.** Two of the 54 were invisible to it: AHB
`HREADYOUT`'s members are the bare digits `0` and `1`, so its `(member, value)` pairs are producible
by seven tables and the ordered walk picked one where the cells parse. **578 slots (3.0%) in 152
enums** are in that ambiguity class; the direction is an under-count, so the totals are floors.

**A stale proof seal is repaired by rebuild-then-validate, per stage, in order.** Validation
back-annotates the artifact, so building a stage before its upstream is validated fails with
`cumulative proof ledger does not retain the exact verified upstream prefix`. Doing it in order
restores `check_chain_currency.sh` to 27/27 at every stage AND makes `eval-extraction` run again where
it refused during `.5.vi.a`'s trial — which is the concrete protocol that leaf lacked.
[[evidence-rule-field-content-stales-every-proof]]

Links: [[minted-enums-are-mostly-cross-table-conflations]], [[caption-label-is-not-table-identity]],
[[generic-enum-conflation]].
