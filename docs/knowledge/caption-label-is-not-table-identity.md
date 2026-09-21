---
id: caption-label-is-not-table-identity
title: Only 2 of 550 minted enums are document-stated page-break continuations, so a span rule cannot be made FP-free; the discriminator is the caption's `Table N-M` label prefix, which names 1,634 conflated tables and 6 healthy ones
answers:
  - "are conflated enums really split code tables"
  - "how many enums are page-break continuations"
  - "should I refuse an enum whose members span tables"
  - "should I split a conflated enum per table"
  - "where does a minted enum's name actually come from"
  - "can a section heading name a table"
  - "why is an enum called TABLE or FIGURE"
  - "what is the caption label prefix"
  - "how do I adjudicate cross-table enum conflation"
  - "which part of identity_context may name an encoding table"
  - "how many tables declare themselves a continuation"
  - "how do I match an Arm caption label like Table A7.4"
  - "how does a spec mark a table continued across a page break"
date: 2026-09-21
status: current
tags: [intentir, enums, extraction-quality, kg-isf-completeness, census, identity, adr-0006]
evidence: scripts/adjudicate_conflated_enum_identity.py; docs/research/enum-identity-adjudication.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.vi); crates/specforge/src/ir/evidence.rs (derive_encoding_enum_name, caption_table_ref)
reverify: "python3 scripts/adjudicate_conflated_enum_identity.py — expect 550 minted enums / 316 single_table, 2 stated_continuation, 22 adjacent_same_shape, 204 distinct_tables, 6 unattributed; the caption-label rule refusing 6 single_table (all FIGURE/TABLE), 0 stated_continuation, 0 adjacent_same_shape, 13 distinct_tables, and cutting 1,634 contributing tables; affected enum names exactly TABLE 41 / FIGURE 11 / CHAPTER 2"
---

`KG-ISF-COMPLETENESS.5.v` left `.5.vi` a warning: before refusing an enum whose members span
tables, measure the legitimate case — **a specification that splits one code table across a page
break**. Measured, that case is **2 enums of 550**, and both rules the leaf offered still fail.

## The adjudication, `2026-09-21`, all 78 persisted chains

| verdict | enums | members | promoted |
| --- | ---: | ---: | ---: |
| `single_table` | 316 | 2,130 | 283 |
| **`stated_continuation`** | **2** | 10 | 2 |
| `adjacent_same_shape` | 22 | 244 | 22 |
| `distinct_tables` | 204 | 18,602 | 199 |
| `unattributed` | 6 | 606 | 6 |

`stated_continuation` means the document itself says so — the `Table B2.2 Continued from previous
page` marker `caption_table_ref` reads and `PDF-VARIANT-DIGESTION.12a` already trusts — plus
agreement on which table is continued. Exactly two qualify, both in the AXI gold `ihi0022_l`:
`ARDOMAIN` (`Table A7.4`) and `SYSCOACK` (`Table A15.26`).

**Check that a near-zero is not vacuous — and check it against BOTH dialects.** Publishers write
the marker two ways: after the label (`Table B2.2 Continued from previous page`, **209 captions in
7 documents**) or at the end (`Table 2-12 … (continued)`, **112 in 20 documents**, overlapping on
one caption). Reading only the first leaves the detector blind on most of the corpus while its
vacuity check looks healthy; reading both, over 320 of 5,531 captions, still gives **2**. And
**27 fused enums sweep continuation fragments in** without being continuations, which confirms the
collision — CHI C2C `ihi0098_b`'s `TABLE` spans 114 tables including 29 fragments from **20
different parents**. A continuation chain has one parent.

**The layout cannot substitute for the statement.** All 22 `adjacent_same_shape` items were
adjudicated by hand and the class mixes: AXI+ACE `AWSNOOP`/`ACSNOOP` genuinely resume one code
sequence, while RISC-V `ABSTRACT`/`CONFIGURATION`/`SYSTEM` each pair two
`Field | Description | Access | Reset` tables for **different registers** — a register chapter
emits that header once per register. So **both rules `.5.vi` was offered are unusable**: refusing
a span fires on the genuine cases, and splitting per table turns one `TABLE` enum into **491
symbols**.

## The defect is the NAME, and the discriminator is the caption LABEL

`derive_encoding_enum_name` names a table after any declared signal in
`caption + section_title + header`. Resolving which part actually carried the name, per
contributing table:

| name source | single_table | stated_cont. | adjacent_same_shape | distinct_tables |
| --- | ---: | ---: | ---: | ---: |
| `section` only | 227 | 0 | 39 | 1,214 |
| **`caption_label` only** | **6** | **0** | **0** | **1,634** |
| `caption_body` only | 11 | 0 | 0 | 58 |
| `header` (± section) | 40 | 4 | 10 | 177 |
| other / none | 32 | 0 | 3 | 242 |
| total tables | 316 | 4 | 52 | 3,325 |

The **caption label** is the publisher's numbering furniture — the `Table 2-1` / `Table A7.4`
prefix every caption opens with. A rule forbidding it as a name source touches **54 of 550 enums
corpus-wide, carrying exactly three names: `TABLE` (41), `FIGURE` (11), `CHAPTER` (2)**. There is
no fourth. It refuses **0** continuations of either kind and 6 single-table enums, all six named
`FIGURE` or `TABLE`.

`CHAPTER` is why this is document grammar and not a two-word denylist: the label is
`<word> <number>` whatever word the publisher chose (ADR 0006).

**The section heading is the opposite and must not be touched**: it names 227 of 316 perfectly
healthy single-table enums, and a rule against it refuses 227 (210 promoted). Naming from the
heading is legitimate; **merging** unrelated tables on the resulting name is the remaining defect,
and that lives in `build_symbol_definitions`, not at the naming seam.

## Three instrument traps this measurement fell into first

- **An Arm caption label does not start with a digit.** `Table A7.4`, `Table B13.12` — a pattern
  anchored on `[0-9]` misses every volume-numbered caption and scores its `Table` as caption prose
  instead. Fixing it moved `caption_label` tables **1,217 → 1,634** and the rule's reach
  **43 → 54**.
- **A shared caption reference is not a statement of continuation.** Three tables captioned
  `Chapter B15. …` share a chapter, not a table. Only the `Continued` marker is the statement.
  And compare references built from the captured *groups*: the matched slice keeps trailing
  punctuation, so `Table A7.4:` and `Table A7.4` compare unequal and the real pair is missed.
- **A marker has dialects.** Reading only the prefix form missed the trailing `(continued)` form
  that covers 20 documents to the prefix form's 7.

Together these took `stated_continuation` **0 → 2** — and the first published version of the
finding, "the legitimate case has no population at all", was wrong in the direction that flattered
the proposed rule. **All three were found by following one odd-looking item, never by re-reading
the code.**

Links: [[minted-enums-are-mostly-cross-table-conflations]], [[generic-enum-conflation]].
