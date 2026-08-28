# Ingest content-loss adjudication

Owning leaf: `SOURCE-IR-REPRODUCIBILITY.5` (MEASURE).

## The question

`SOURCE-IR-REPRODUCIBILITY.1` re-ingested the whole live corpus and reported that a current ingest
drops 31 persisted content elements: 28 re-segmentation, and **three** that "the current toolchain
emits nowhere" — one paragraph each in the USB4 Connection Manager guide, USB 3.2, and Wishbone.
Those three were the only measured evidence that ingest loses source content outright, and they are
what made the tree's preservation question urgent.

But that verdict rests on one test. The census asks whether the persisted element's text appears as
a substring of the replayed element stream joined with `" \n"`. That test is exactly right for
detecting a merge or a split at the SourceIR level, and it is blind to two things a converter does
routinely without losing a word:

- it can split a paragraph across two items, so the joined string carries a separator the persisted
  text does not have;
- it can splice a newly detected fragment **into** a sentence, so the persisted characters are no
  longer contiguous anywhere.

Either one makes a fully preserved paragraph read as absent. So "three lost paragraphs" was an upper
bound on loss, not a measurement of it, and `.5` exists to convert it into an answer with named
remedies: text absent from the converter's own document is an upstream boundary to declare, and text
present there but dropped by SpecForge is a defect this repository can fix.

## Method

`scripts/measure_ingest_content_loss.py` re-ingests each named document through the same isolated
`source_to_intent_replay` example the `.1` census uses, into a fresh `.project-data/tmp` root, and
then reads **two** artifacts from that one run: the `SourceIR` it produced, and the converter's own
document beside it (`normalized/<key>.backend.json`). Nothing under `generated/` is written or
removed, and the producer refuses to run with a modified `crates/` tree.

It imports the `.1` census rather than restating it — the same frame, staging protocol, root
normalization, and drift shape — and it asserts that the set of elements it adjudicates has exactly
the size the census's own absent count reports. The two producers cannot drift into a second opinion
about what "absent" means.

### The alignment

For each flagged element the producer answers "where did these words go" **token by token**, not as
a string:

1. **whole item** — some converter text item equals the persisted text after whitespace collapse.
2. **within item** — the persisted text is a substring of one converter text item.
3. **aligned** — anchor the persisted tokens in the converter's body-layer token stream, then run an
   ordered alignment inside a bounded window. Every persisted token accounted for, in order, is
   preservation however much was inserted around it. A persisted token with no match is loss.

The anchor is a shared 6-token gram where one exists, and the text's **rarest** token otherwise —
because a single spliced-in fragment can break every 6-gram of a short paragraph, and a truncated
text can be shorter than the gram itself.

The window is what keeps the alignment honest. It reaches one text-length before the anchor and two
after; without it, a long document would "recover" any paragraph from tokens scattered across
unrelated pages. When the candidate anchors have to be truncated, the result is reported
`unresolved_anchor_truncated` rather than absent: a search that ran out of budget is not evidence
that a text is gone.

Insertions are counted only **between** the first and last matched token, so the surrounding window
is never mistaken for interleaved content.

### Reachability

An item "reached SourceIR" when the `(source_ref, text)` pair matches a `content_elements` or
`document_sections` record, or when its text is carried as the bound `caption_text` of a table or
visual asset, or when it appears in a residual decision.

The pair, not the ref. A bounded-memory ingest converts page ranges and writes one converter
document per range, and `self_ref` is per-range: the Arm Debug guide's 6,784 content elements share
only 2,252 distinct `source_ref` values, 1,883 of them used more than once. USB 3.2 in this
measurement is nine batches. Keying on the ref alone would credit an item in one range with a record
belonging to another.

### The controls

Eighteen controls run under `--self-test`, and six perturbations were observed going RED:

| Perturbation | Observed |
| --- | ---: |
| drop the token alignment, leaving the `.1` census's whole-string test | 11/18 |
| count only `(source_ref, text)`, dropping the bound-caption leg | 17/18 |
| test only the direct parent for figure interiority, not the ancestor chain | 17/18 |
| absorb an unrecognized converter bundle as an empty document | 17/18 |
| count insertions across the whole window instead of the matched span | 17/18 |
| remove the rarest-token fallback anchor | 16/18 |

The first is the decisive one: it is the previous producer's own test, and it turns this
measurement's result back into `.1`'s.

## Result — the three paragraphs are not lost

Measured at production revision `22f4b329` with `crates/` unmodified.

| Document | Element | Page | Verdict | Tokens missing | Tokens inserted |
| --- | --- | ---: | --- | ---: | ---: |
| `usb4_connection_manager_guide_v2_0_2025_11` | `elem_00659` | 51 | retained | 0 | 6 |
| `usb_3_2_revision_1_0_2017_09` | `elem_03577` | 291 | retained | 0 | 5 |
| `wbspec_b4_wishbone_b4_specification` | `elem_01967` | 126 | retained | 0 | 35 |

**All three are `aligned_with_insertions`: every persisted word is present, in order, and every
converter item covering them carries a `SourceIR` record.** What changed is that the current run
interleaves something new into the middle of each sentence:

- **USB4 Connection Manager guide.** `#/texts/1381` now reads `… by reading the` **`USB4 Host
  Enhanced SS Host Controller`** `ROUTER_CS_6. Gen T Full Connectivity Support field …` — six words
  lifted out of the adjacent figure and dropped into the sentence. The paragraph is one item in both
  runs; only its contents grew.
- **USB 3.2.** `batch4:#/texts/842` takes **`Disabled Stall, Error, or SetFeature`** — state labels
  from the neighbouring stream state machine — into the middle of a note about DP/ACK ordering.
- **Wishbone.** The persisted paragraph is split across `#/texts/4075` and `#/texts/4078`, with
  footnote 14 (`The logic analyzer samples at 500 Mhz …`) and the plot annotation `Sampled · 4.0g`
  sitting between the two halves. Both halves reach `SourceIR` as `elem_02470` and `elem_02474`.

So the answer to `.5` is unanimous and it is not the expected one: **no source content is
unrecoverably lost at this boundary in any of the three documents.** `.1`'s "three paragraphs emitted
nowhere" is withdrawn — it measured the test, not the documents. The corpus-wide loss figure for the
`.1` re-ingest is therefore **zero elements of content**, with 31 elements re-segmented rather than
28.

That is a better result than the tree assumed, and it is also a worse one, because the sentences
these fragments land in are now wrong. `Before setting up a USB3 Gen T path … by reading the USB4
Host Enhanced SS Host Controller ROUTER_CS_6. Gen T Full Connectivity Support field` is not a
sentence the specification contains. Preservation is satisfied; **faithfulness is not**, and nothing
in the pipeline currently observes the difference.

## The result the question did not ask for

Loading the converter's own document to answer that question makes a second measurement almost free:
how much of what the converter emitted reaches `SourceIR` at all. The pipeline conserves downstream —
stage conservation is 120/120 — but nothing measures the PDF boundary, because there is no upstream
artifact to conserve against.

| Document | Converter text items | Reach SourceIR | Reach nothing |
| --- | ---: | ---: | ---: |
| `usb4_connection_manager_guide_v2_0_2025_11` | 2,456 | 1,509 | 947 |
| `usb_3_2_revision_1_0_2017_09` (9 batches) | 12,317 | 6,587 | 5,730 |
| `wbspec_b4_wishbone_b4_specification` | 4,097 | 2,126 | 1,971 |
| **Total** | **18,870** | **10,222** | **8,648** |

**Forty-six percent of the text items the converter emits reach no `SourceIR` record and earn no
residual.** Every one resolves to a named predicate — the producer's `unexplained` bucket is empty
across all three documents, so this is a complete account rather than a partial one:

| Reason | Items | What it is |
| --- | ---: | --- |
| `picture_interior_not_traversed` | 5,896 | text the converter placed inside a figure |
| `content_layer_excluded` | 2,722 | running headers and footers, on the `furniture` layer |
| `empty_text` | 30 | `formula` items whose text is empty |

Two of the three are correct behaviour. A running header is furniture by the converter's own
classification and is deliberately excluded; an empty formula has no text to carry.

**The first is not.** `docling_backend.rs` iterates with `traverse_pictures=False`, so
`DoclingDocument.iterate_items` skips every child of a `PictureItem` except the refs in that
picture's own `captions` list — and the skip is at the boundary, so everything below a blocked child
goes with it. A list group inside a figure takes its list items down too, which is why the ancestor
chain has to be walked rather than the direct parent: in the persisted I2C bundle (below) nine list
items sit under `#/groups/41`–`44` and `#/groups/65`–`69`, whose parents are `#/pictures/61` and
`#/pictures/82`. A direct-parent test reports those nine as unexplained.

Most of that 5,896 is diagram furniture — `Tx_0`, `Router A`, `Back to TOC`. But not all of it:

| Label | Items dropped as figure interior |
| --- | ---: |
| `text` | 5,875 |
| `caption` | 9 |
| `footnote` | 8 |
| `section_header` | 4 |

Nine captions the converter labelled `caption` and attached to no figure are discarded silently.
That is the same unattached-caption population `SOURCE-IR-REPRODUCIBILITY.6` exists to rebind, and it
is larger than the caption-binding delta `.1` measured, because `.1` could only see bindings that
changed between two runs.

**This gap is not drift.** The same census runs against the persisted artifacts and their retained
bundles, with no ingest at all (`--persisted`):

| Persisted document | Converter items | Reach nothing | Figure interior | Furniture | `.1` verdict |
| --- | ---: | ---: | ---: | ---: | --- |
| `um10204_rev7_0_2021_i2c_bus_specification` | 2,507 | 1,814 | 1,372 | 437 | reproduces exactly |
| `wbspec_b4_wishbone_b4_specification` | 3,517 | 1,895 | 1,434 | 457 | drifted |
| `usb4_connection_manager_guide_v2_0_2025_11` | 2,183 | 870 | 681 | 187 | drifted |
| `102196_…_aarch64_external_debug_guide` | 373 | 107 | 0 | 107 | drifted |

The I2C specification reproduces **byte-for-byte** under the `.1` census and still stands on an
ingest that discarded 1,372 figure-interior items — including **39 captions**. Reproducibility and
conservation are independent properties, and until now only one of them was measured.

The last row is the honest counterpoint: the Arm external-debug guide loses nothing to figures at
all, because its converter document places no text inside them. The gap is document-dependent, not
universal, which is why it needs a gate rather than an assumption in either direction.

A fourth re-ingest makes the same point without needing an external source at all. The I2S bus
specification is a repository-owned PDF, and the `.1` census scores it as reproducing **exactly**
(115 → 115 content elements). Re-ingesting it now yields 464 converter text items of which **349 —
75% — reach no `SourceIR` record**: 255 figure interior (including four captions and one list item)
and 94 furniture-layer headers and footers. The most reproducible document in the corpus is among
the least conservative:

```bash
python3 scripts/measure_ingest_content_loss.py \
  --output-root .project-data/tmp/<census-id> --census-id <census-id> \
  --owner SOURCE-IR-REPRODUCIBILITY.5 \
  --document um11732_v3_2022_02_17_i2s_bus_specification
```

## What this does not say

It does not extend the conservation figure to the corpus. Four re-ingested documents and four
persisted bundles are what was measured; the whole-population number belongs to
`SOURCE-IR-REPRODUCIBILITY.7`'s gate, not to this report.

It does not say the figure-interior text should become `content_elements`. Diagram labels are not
prose and promoting them wholesale is how the interleaving above happened in the first place. What it
says is narrower and harder to argue with: the pipeline's own doctrine requires an unresolved thing
to become an explicit residual rather than disappear, and 5,896 items disappear.

It does not measure faithfulness. Every alignment here confirms that words survive; none of them
confirms that the sentence a word ends up in is the sentence the specification wrote. The three
interpolations above are direct evidence that survival and faithfulness have come apart, and nothing
currently gates the difference.

Reproduce with:

```bash
python3 scripts/measure_ingest_content_loss.py --self-test
python3 scripts/measure_ingest_content_loss.py \
  --output-root .project-data/tmp/<census-id> --census-id <census-id> \
  --owner SOURCE-IR-REPRODUCIBILITY.5 \
  --external-source-map .project-data/tmp/<untracked-runtime-map>.json --plan-only
```

Dropping `--plan-only` runs it; `--compare-only` re-derives the adjudication from an existing root's
retained replays without re-ingesting, and `--document <key>` adjudicates one document instead of the
declared three. `--persisted` censuses conservation on the persisted artifacts and their retained
bundles, with no ingest and no source PDF:

```bash
python3 scripts/measure_ingest_content_loss.py \
  --output-root .project-data/tmp/<census-id> --census-id <census-id> \
  --owner SOURCE-IR-REPRODUCIBILITY.5 --persisted \
  --document um10204_rev7_0_2021_i2c_bus_specification
```
