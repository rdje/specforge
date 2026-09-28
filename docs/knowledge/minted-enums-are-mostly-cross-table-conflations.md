---
id: minted-enums-are-mostly-cross-table-conflations
title: 231 of 550 minted enums draw their members from no single table, and 226 of the 512 symbol_definitions that reach IntentIR come from them — the member/value-fidelity defect is most of the symbol surface, not a bounded curiosity
answers:
  - "where do minted enum members come from"
  - "how many minted enums are cross-table conflations"
  - "how many symbol_definitions reach IntentIR"
  - "are enum members ever whole sentences"
  - "what fraction of the IntentIR symbol surface is trustworthy"
  - "what is enum member-column drift"
  - "how do I census enum member and value fidelity"
  - "is the enum member-value defect promoted to isf"
date: 2026-09-21
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, census, fidelity]
evidence: scripts/measure_enum_member_value_fidelity.py; docs/tasks/KG-ISF-COMPLETENESS.md (.5.v); crates/specforge/src/ir/evidence.rs (infer_encoding_column_indices, scan_encoding_tables_by_signal_anchor)
reverify: "python3 scripts/measure_enum_member_value_fidelity.py — expect TODAY (re-derived 2026-09-29, EXTRACTION-REPRESENTATION-GAPS.3.c.i) 522 minted enums across 48 documents, 309 with a located source table (30 member-column drift, 164 positional values, 21 both), 213 with no located source table; the 2026-09-22 figures (555 / 51 / 29 / 168 / 229) and the ones this card was written on (550 / 52 / 21 / 160 / 231) are dated measurements and no longer re-derive. The symbol_definitions class count (512 total, 226 conflated / 135 positional / 17 drift / 134 clean) is a dated 2026-09-22 measurement, not re-derived on 2026-09-29"
---

`evidence.rs` mints `Enum <name> <member> = <value>.` from tables. `KG-ISF-COMPLETENESS.5.i` and
`.5.iv.a` gate the **name**. Nothing asks where the members that follow came from, and the value falls
back to `row_idx` when no cell parses.

**The same stratum caveat applies to every number here** (`.5.vii.b`): 530 of the 550 enums and
21,459 of the 21,592 members are LEGACY artifacts, which `CORPUS-CHAIN-CURRENCY.11` says are evidence
about themselves and not about the current reader. The current producer mints 25 enums / 136 members
(re-derived `2026-09-22`, after `.5.xiii.a`).

## Three classes, measured `2026-09-21` over all 78 persisted chains

| class | enums | detection |
| --- | ---: | --- |
| member-column drift | **21** | the name matched one column's header; the members are another column's cells |
| positional values | **160** | values are exactly `0..n-1` and appear in no column of the source table |
| **cross-table conflation** | **231** | **no single table holds the members at all** — 228 have no column anywhere containing every member |
| (total minted) | **550** | across 52 documents |

**The class nobody was looking for is the dominant one.** Those 231 enums carry **19,447 of the corpus's
21,592 enum statements — 90% of every minted member** — and **3,159 of those members exceed 60
characters**: description sentences promoted as member names. The longest is **2,344 characters**, a
changelog paragraph. This is precisely what `.5.ii`'s `is_prose_fragment_member_name` gate exists to
reject, still leaking at scale.

## It is not bounded away from the product

`.5.v` opened believing the defect stayed upstream — *"`symbol_definitions` stays 0 on this document"*.
True of Wishbone, false of the corpus:

| promoted `symbol_definitions` | symbols | members | members >60 chars |
| --- | ---: | ---: | ---: |
| from conflated enums | **226** | 9,999 | 2,782 |
| from positional-value enums | 135 | 617 | 176 |
| from member-column drift | 17 | 69 | 15 |
| clean | **134** | 588 | 97 |
| total | **512** across 50 documents | 11,273 | |

**Only 26% of the IntentIR symbol surface comes from an enum this census finds clean**, and 89% of
promoted members come from conflations.

## Each flag is a necessary condition, not a verdict

A genuine code table may run `0..n-1`; a `name | description` table may legitimately source members from
column 1; and a specification may split one code table across a page break, which reads as conflation.
The **>60-character member count is the independently damning signal**, because no encoding names a
member with a sentence. `KG-ISF-COMPLETENESS.5.vi` (identity) and `.5.vii` (positional values) own the
two rules separately — they fire at different seams and must be measured apart, and `.5.vi` goes first
because conflation dominates the positional population too.

## The instrument needed correcting before its number meant anything

The census first required the minted members to be a **prefix** of a column and left 240 enums
unsourced. The minting loop **skips** members its `.5.ii` and `.5.iii` gates reject, so the minted list
is a *subsequence*; matching that way resolved 9. The remaining 231 are not a matcher limitation — 99%
of them have no single column containing all their members under any matching rule.

**Where the 160 went.** `.5.vii` replayed the producer instead of reading the value SHAPE and
measured the fallback exactly: 10,189 of 19,023 minted values are the row index, and the shape
test could see neither the mixed enums nor the gapped ones — [[minted-enum-values-are-mostly-row-indices]].

Links: [[document-class-is-decided-at-build-but-written-by-validate]],
[[minted-enum-values-are-mostly-row-indices]].
