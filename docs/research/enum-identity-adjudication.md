# Enum identity — adjudicating the cross-table conflations (`KG-ISF-COMPLETENESS.5.vi`)

Read-only, `2026-09-21`, over all 78 persisted chains. Reproducer
`scripts/adjudicate_conflated_enum_identity.py`. Prior census:
`scripts/measure_enum_member_value_fidelity.py` (`.5.v`, KM
`[[minted-enums-are-mostly-cross-table-conflations]]`).

`.5.v` counted **231 of 550 minted enums with no single source table** and stopped there on
purpose, handing `.5.vi` one instruction: before writing a rule, measure how many of them are the
legitimate case — **a specification that splits one code table across a page break** — and how
many are a name colliding across unrelated `Field | Description` tables. Only then choose between
refusing the enum and splitting it per table.

This record answers that. The legitimate case is real, and it is **2 enums of 550**; both rules
the leaf offered are nevertheless wrong, for a reason neither anticipated.

## The instrument

Attribution replays the producer instead of pattern-matching its output. For each minted enum and
each table, `infer_encoding_column_indices` and the member/value synthesis of
`synthesize_encoding_declarations_for_enum` are re-run to give the `(member, value)` sequence that
table would mint under that name. The minted members are then segmented into consecutive runs,
each an ordered subsequence of one table's replay, with the tables in increasing order — the walk
the producer actually performs.

Two instrument decisions are load-bearing:

- **The `.5.ii`/`.5.iii` per-member gates are deliberately not replicated.** They only ever
  *remove* members, so every genuinely minted run is a subsequence of the ungated replay. Matching
  against the ungated sequence is therefore sufficient, and it cannot drift when those gates change.
- **A convergence pass may restart the table walk.** `scan_encoding_tables_by_signal_anchor` runs
  once per pass with a growing `known_signals`, appending only new statement texts, so a strictly
  monotone table order is not guaranteed. The reconstruction allows up to three restarts and falls
  back to a greedy set cover. Of 550 enums, **498 reconstruct in one monotone walk, 45 need a
  restart, and 7 fall back to set cover** — the producer's own order for 90% of the population and
  order-consistent for 99%.

## Adjudication of the 550

| verdict | enums | members | members >60 chars | promoted |
| --- | ---: | ---: | ---: | ---: |
| `single_table` | **316** | 2,130 | 308 | 283 |
| **`stated_continuation`** | **2** | 10 | 2 | 2 |
| `adjacent_same_shape` | 22 | 244 | 73 | 22 |
| `distinct_tables` | **204** | 18,602 | 2,917 | 199 |
| `unattributed` | 6 | 606 | 167 | 6 |

Cross-tabulated against `.5.v`, the two independently built instruments partition the same
population the same way. Every one of its **231** no-source-table enums lands in a multi-table or
unattributed row — 202 `distinct_tables`, 21 `adjacent_same_shape`, 2 `stated_continuation`,
6 `unattributed` — and **316 of its 319** located ones land in `single_table`, the other 3 being
enums whose single matched column was not the producer's actual source.

### The legitimate case is two enums, and both are in the AXI gold

`stated_continuation` requires the document's **own** statement that two fragments are one table:
the `Table B2.2 Continued from previous page` marker that `caption_table_ref` reads and
`PDF-VARIANT-DIGESTION.12a` already trusts in production, plus agreement on which table is being
continued. Corpus-wide exactly two minted enums qualify, both in `ihi0022_l` (the AXI wire gold):

| enum | head | continuation |
| --- | --- | --- |
| `ARDOMAIN` | `Table A7.4: ARSNOOP encodings` | `Table A7.4 Continued from previous page` |
| `SYSCOACK` | `Table A15.26: Coherency Connection signaling states` | `Table A15.26 Continued from previous page` |

**The test is not vacuous, which is the first thing to check about a near-zero — and checking it
once was not enough.** Publishers write the marker in two dialects, and the detector initially read
only one:

| dialect | captions | documents |
| --- | ---: | ---: |
| `Table B2.2 Continued from previous page` (after the label) | 209 | 7 |
| `Table 2-12 Configurable options … (continued)` (at the end) | 112 | 20 |
| both on one caption | 1 | — |

With **both** read — 320 of 5,531 captions, spanning far more of the corpus than either alone —
the answer is still **2**. The number is small because fused enums are rarely continuations, not
because the evidence is missing.

**27 fused enums do sweep continuation fragments in, and that confirms the collision rather than
softening it.** CHI C2C `ihi0098_b`'s `TABLE` spans 114 tables including **29 continuation
fragments belonging to 20 different parent tables**; CHI `ihi0050_g`'s spans 149 including 26
fragments from 25 parents. A continuation chain has one parent. These have twenty-five.

### And the layout cannot stand in for the statement

`adjacent_same_shape` is the weaker test — identical first header row, compatible captions,
adjacent pages, consecutive table ids — and all 22 were adjudicated by hand. The class **mixes**:

- AXI+ACE `ihi0022_h_c` `AWSNOOP` (header `Code | Transaction type | …`, first codes `0b0000` then
  `0b1101`) and `ACSNOOP` (`ACSNOOP[3:0] | Transaction`) **are** one code table continued — the
  second fragment resumes the first's code sequence — but this document states nothing.
- RISC-V `1_0_risc_v_debug_specification` `ABSTRACT`, `CONFIGURATION` and `SYSTEM` are **not**.
  Each pairs two `Field | Description | Access | Reset` tables describing *different registers*
  (`progbufsize…` with `cmderr…`; `cmdtype…` with `autoexecprogbuf…`). A register chapter emits
  that header once per register, so the shape repeats across unrelated tables — which is exactly
  the `Field | Description` name collision the leaf named, wearing the continuation's clothes.

So outside the two stated cases **no available oracle separates them**, and that disposes of both
offered rules:

- **Refusing an enum whose members span tables** fires on all 228 multi-table enums, including the
  two the document itself vouches for and the AXI+ACE pair no oracle can vouch for.
- **Splitting it per table** turns one `TABLE` enum in `coresight_soc_600` into **491 symbols**.

Neither can be made false-positive-free, because both act on the **span**, and the span is not
where the defect is.

## Where the defect is: the name, not the span

`derive_encoding_enum_name` builds `identity_context = caption + section_title + header` and names
a table after any declared signal referenced anywhere in that blob. Resolving where each
contributing table's name actually came from separates four parts, of which only two are facts
about the table itself:

| name source | `single_table` | `stated_cont.` | `adjacent_same_shape` | `distinct_tables` |
| --- | ---: | ---: | ---: | ---: |
| `section` only | 227 | 0 | 39 | 1,214 |
| **`caption_label` only** | **6** | **0** | **0** | **1,634** |
| `caption_body` only | 11 | 0 | 0 | 58 |
| `header` (± `section`) | 40 | 4 | 10 | 177 |
| other combinations | 13 | 0 | 3 | 189 |
| none located | 19 | 0 | 0 | 53 |
| (contributing tables) | 316 | 4 | 52 | 3,325 |

`caption_label` is the publisher's numbering furniture — the `Table 2-1` / `Figure 8.3` /
`Table A7.4` prefix that opens *every* caption in the document. It names **1,634 of the 3,325
tables** behind `distinct_tables` (49%), **42 of the 100** behind the six unattributed enums,
**6 of 316** single-table enums, and **none at all** of the 56 behind the continuations. Nothing
else in the table separates the two populations that sharply.

The section heading is the opposite: it names 227 of 316 perfectly good single-table enums. It is
the single most common naming source in the corpus, and a rule against it would be a catastrophe.

## Three candidate rules, simulated

Each rule removes a part of `identity_context`. A table whose name the instrument located nowhere
is unaffected by construction and counts as surviving; those 72 tables of 3,797 are the
instrument's residual, not a cost of any rule. A rule is judged by what it costs the populations
it must **not** touch.

| rule — a name may come from … | refuses `single_table` | refuses `stated_cont.` | refuses `adjacent` | refuses `distinct` | tables cut from `distinct` |
| --- | ---: | ---: | ---: | ---: | ---: |
| its own caption text or header | **233** (215 promoted) | 0 | 15 | 104 | 2,896 |
| anything but the **caption label** | **6** | **0** | **0** | 13 | **1,634** |
| anything but the section heading | **227** (210 promoted) | 0 | 15 | 78 | 1,214 |

Two of the three destroy most of the healthy population. The third does not, and the per-item
adjudication says why.

### Every enum the caption-label rule touches, by name

Across all 550 enums the rule reaches **54**, and they carry exactly three names:

| name | enums affected |
| --- | ---: |
| `TABLE` | 41 |
| `FIGURE` | 11 |
| `CHAPTER` | 2 |

There is no fourth. The six it refuses outright in a population it must not touch are
`FIGURE` ×4 (`48882_pub_3_10`, `ddi0471_a`, `ihi0098_b`, `smbus_3_3_1`) and `TABLE` ×2
(`den0034_a`, `ihi0068_d`) — document furniture in every case, so refusing them is the rule
working, not a cost. It refuses **zero** of the two stated continuations and **zero** of the 22
layout-plausible ones.

`CHAPTER` matters out of proportion to its two instances: it is what makes this a rule of document
grammar rather than a two-word denylist. The label prefix is `<word> <number>` whatever word the
publisher chose, so the gate needs no vocabulary and ADR 0006 holds.

### Reach

The rule cuts 1,634 of the 3,325 tables behind the `distinct_tables` class (49%) and dissolves the
largest conflations:

| document | enum | tables before | after |
| --- | --- | ---: | ---: |
| `100806_0100_…coresight_soc_600` | `TABLE` | 491 | 187 |
| `100806_0200_…coresight_soc_600` | `TABLE` | 193 | 76 |
| `100806_0701_…coresight_soc_600` | `TABLE` | 195 | 32 |
| `ihi0050_g_…amba_chi` | `TABLE` | 149 | 3 |
| `ihi0098_b_…amba_chi_c2c` | `TABLE` | 114 | 7 |
| `nvme_base_specification_2_0a` | `FIGURE` | 60 | 1 |

It is not the whole of `.5`'s remaining defect — 204 enums span distinct tables and most survive
this rule under a name that is genuinely theirs. It is the part that is measured, FP-free, and
grounded in document structure, and it removes the class that carries the pathology.

## Instrument corrections, recorded because they changed the headline

Both were found by following a single suspicious item rather than by re-reading the code, and both
moved a published number.

1. **The caption-label pattern required the numbering token to begin with a digit.** Arm captions
   number by volume — `Table A7.4`, `Table B13.12` — so the label went unrecognised and its
   `Table` was scored as caption *prose*. Corrected, `caption_label`-only tables in the
   `distinct_tables` class rose **1,217 → 1,634** and the rule's reach **43 → 54** enums.
2. **A shared caption reference was accepted as a statement of continuation.** Three CHI C2C
   register tables captioned `Chapter B15. …` share a chapter, and a chapter is not a table; the
   detector called them one continued table. Requiring the `Continued` marker itself — the actual
   statement — dropped that false positive and, with correction 1, admitted the two real AXI-gold
   pairs a punctuation mismatch in the reference had been hiding (`Table A7.4:` against
   `Table A7.4`). `stated_continuation` went **0 → 1 → 2**.
3. **Only one of the two continuation dialects was being read.** The trailing `(continued)` form
   covers **112 captions in 20 documents** against the prefix form's 209 in 7, and they overlap on
   a single caption — so the detector was blind on most of the corpus while its vacuity check
   looked healthy. Reading both left the verdict at **2** and raised the swept-in-fragment count
   **7 → 27**.

The first version of this adjudication published `stated_continuation = 0` and concluded that the
legitimate case had no population at all. That conclusion was wrong, and it was wrong in the
direction that flattered the rule being proposed. **Each correction was found by following one
odd-looking item, never by re-reading the code**: a suspicious `Chapter B15.` verdict, an AXI pair
that obviously *was* a continuation, and a caption sampled at random that ended in `(continued)`.
The rule survives all three — it still refuses neither of the two — but it now survives on
evidence rather than on the absence of it.

## What this does not settle

- **The 204 `distinct_tables` enums with a real name.** A signal token in a section heading names
  every table under that heading, and merge-by-name then fuses them. The heading is a legitimate
  naming source — 227 healthy enums depend on it — so the fix there is not to refuse the name but
  to stop *merging* on it, which is a `build_symbol_definitions` question and not this leaf's.
- **The 22 `adjacent_same_shape` enums**, which need an oracle that does not exist yet; the AXI+ACE
  `AWSNOOP`/`ACSNOOP` pair is genuine and the document says nothing.
- **The 6 unattributed enums** (606 members, 6 promoted) whose members no table replay reproduces.
- **`.5.vii`'s positional values**, whose denominator this rule moves and which is why it is
  sequenced after it.

## Authoritative tracking

`docs/tasks/KG-ISF-COMPLETENESS.md` — `.5.vi` (this adjudication) and `.5.vi.a` (the code slice it
opens). Knowledge Map: `[[caption-label-is-not-table-identity]]`,
`[[minted-enums-are-mostly-cross-table-conflations]]`, `[[generic-enum-conflation]]`.
