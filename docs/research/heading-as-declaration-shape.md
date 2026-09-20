# A section heading that IS a wire name — what may license it as a declaration

Owning leaf: `SIGNAL-CATALOG-CAPTURE-GAP.2` (DESIGN + MEASURE; no rule, no production change).
Producer: `python3 scripts/measure_heading_declaration_shape.py`.

## The question this census exists to answer

`.1` classified all 33 empty-catalog documents and found exactly one capture miss, with its
declarations located: Wishbone declares its interface as a **heading-as-declaration convention** —
sections whose title *is* the wire name (`CLK_I`, `RST_O`, `DAT_I()`, …), each followed by prose
giving direction and meaning. SpecForge reads declarations from tables and from a formal prose
predicate; this document uses neither.

`.1` also proved the title **shape alone cannot be the rule**, and named the documents that prove
it. So `.2`'s job is not to find the shape — it is to find what, beside the shape, may license a
title as a declaration, and to measure the selection over all 78 documents *before* any code.

## Shape, and then a discriminator

| level | predicate | selects |
| --- | --- | ---: |
| SHAPE | `.1`'s probe B — a section title that is a bare identifier token | **277** headings in **33** documents |
| COMPOUND | …and it has an underscore or an array/call suffix, which is a wire name's form | **65** in **11** documents |
| **LICENSED** | …**and the section's own body states a DIRECTION** | **27** in **1** document |

The direction test uses the closed vocabulary the declaration reader already owns — `input`,
`output`, `inout`, full words only. No abbreviation: `SIGNAL-DECLARATION-ROW-DROP.2h.0` measured
`i`/`o`/`in`/`out` at 0 true positives and 18 false ones, and `.2j` re-refused them corpus-wide. The
body is the heading's **own** content elements in reading order, so a long section is not truncated
and a short one does not borrow its neighbour's prose.

## What the discriminator refuses, document by document

Every false positive `.1` named is refused, and the refusal costs nothing anywhere:

| document | compound headings | licensed | what they actually are |
| --- | ---: | ---: | --- |
| USB 3.2 | 13 | **0** | hub port-feature selectors |
| CoreSight TMC | 4 | **0** | — |
| ARM SMMU software guide | 3 | **0** | register names |
| ADIv6 | 3 | **0** | `IMPLEMENTATION_DEFINED` boilerplate |
| AMBA DTI | 3 | **0** | message names |
| CoreSight architecture | 2 | **0** | — |
| ADIv6 ACC | 2 | **0** | — |
| AMBA LPI / GIC / SMMU | 1 each | **0** | — |

**0 licensed headings outside Wishbone.** A rule keyed on the shape alone would have declared all 33
of those a wire.

## Wishbone's own 32, adjudicated one at a time

**27 admitted, and every one is a real Wishbone wire whose body states its sense in so many words** —
`CLK_O`, `RST_O`, `CLK_I`, `DAT_I()`, `DAT_O()`, `RST_I`, `TGD_I()`, `TGD_O()`, `ACK_I`, `ADR_O()`,
`CYC_O`, `STALL_I`, `ERR_I`, `LOCK_O`, `RTY_I`, `SEL_O()`, `STB_O`, `WE_O`, `ACK_O`, `ADR_I()`,
`CYC_I`, `ERR_O`, `LOCK_I`, `RTY_O`, `SEL_I()`, `STB_I`, `WE_I`. *"The clock input [CLK_I] coordinates
all activities…"*, *"The data output array [DAT_O()] is used to pass binary data…"*. **Precision on
the admitted set: 27/27, zero fabrications.**

**5 refused, and all five are real wires** — this is a recall miss, not a precision win, and the
cause is one thing:

| refused | its body opens | why it fails the test |
| --- | --- | --- |
| `TGA_O()` | *"Address tag type [TGA_O()] contains information associated with address lines…"* | names the **type**, never the sense |
| `TGC_O()` | *"Cycle tag type [TGC_O()] contains information associated with bus cycles…"* | same |
| `TGA_I` | *"Address tag type [TGA_I()]…"* | same |
| `TGC_I()` | *"Cycle tag type [TGC_I()]…"* | same |
| `STALL_O` | *"The pipeline stall **signal** [STALL_O] indicates…"* | says *signal* where its 27 siblings say *input* or *output* |

So the measured result is **27 of 32 (84%) recall on the one capture miss, at 100% precision, with 0
admitted anywhere else in the corpus**.

## The one design question this hands to `.3`, stated rather than assumed

All five misses carry their direction in the name's `_I`/`_O` suffix, which is exactly the convention
the document declares. Reading it would take recall to 32/32 — and it is **not obviously admissible**:
a suffix-to-direction mapping is a naming convention, and ADR 0006 forbids document, vendor and
protocol identity in a production decision. Whether `_I`/`_O` is *universal digital-design grammar*
(like a bit range or a direction column) or *this document's own convention* is the judgement `.3`
must make and defend, not a detail to slip in beside the rule. **This census deliberately measures
without it**, so the number `.3` inherits is the one the direction-word discriminator alone earns.

## Re-derivation

```bash
python3 scripts/measure_heading_declaration_shape.py            # the census
python3 scripts/measure_heading_declaration_shape.py --titles   # every selected heading, with its body
python3 scripts/measure_heading_declaration_shape.py --self-test
```

The self-test pins nine RED cases, including the four that a count cannot give: the shape refuses a
sentence and a lower-case title, COMPOUND is what separates `CLK_I` from a bare word, the direction
test is **whole-word** (a section about *"the outputs"* states no wire's sense), and a body is its
**own** section's elements so a heading borrows nothing from its neighbour.

---

# `.3` — landing it, and the one place `.2`'s discriminator borrows a neighbour's direction

Owning leaf: `SIGNAL-CATALOG-CAPTURE-GAP.3` (LAND + RE-MEASURE). Producer, unchanged:
`python3 scripts/measure_heading_declaration_shape.py`; the landed rule is re-measured from the
**shipped artifact** with `--landed`.

## Why the shipped rule is not `.2`'s discriminator verbatim

`.2` measured a **selection**: which titles a direction-bearing body licenses. `.3` has to emit a
**declaration**, and a declaration needs a *sense*. Asking which direction the body states is what
exposed the difference between the two readings.

`.2`'s discriminator asks whether the section's body states a direction **anywhere**. Two of its 27
reach one only inside a phrase about a *different* signal:

| heading | the only direction word in its body | whose direction it is |
| --- | --- | --- |
| `TGD_I()` | *"…associated with the data **input** array `[DAT_I()]`"* | `DAT_I`'s |
| `TGD_O()` | *"…associated with the data **output** array `[DAT_O()]`"* | `DAT_O`'s |

Both are real Wishbone wires, so as an *existence* test `.2`'s 27/27 precision stands exactly as it
was measured. But as a *declaration* rule the mechanism is wrong: it would attach `DAT_I`'s sense to
`TGD_I`. It happens to agree — `TGD_I` is an input — and that agreement comes from the `_I` suffix,
not from the evidence, which is precisely what makes it unsafe to keep.

This is the boundary `EXTRACTION-GAP-FIX.1` already measured at the parenthetical-abbreviation seam:
a preceding **window** admits candidates whose noun-phrase head is not a wire (a pulse, a channel, a
rate), and the repair was to require the **head** itself to carry the authority. The same repair
applies here, so the shipped rule binds the direction to the name:

> the body states the direction **immediately before it names the signal**, with at most one
> intervening array noun — `the clock input [CLK_I]`, `the data output array [DAT_O()]`.

## What the shipped rule selects, over all 78 documents

| level | predicate | selects |
| --- | --- | ---: |
| SHAPE | a section title that is a bare identifier token | 277 in 33 documents |
| COMPOUND | …with an underscore or an array/call suffix | 65 in 11 documents |
| `.2` LICENSED | …and the body states a direction **anywhere** | 27 in 1 document |
| **`.3` SHIPPED** | …and the direction is **bound to the name** | **25 in 1 document** |

**0 admitted outside Wishbone at every level below SHAPE**, and all 25 directions are correct.

A free control comes with it: the `_I`/`_O` suffix, which the rule never reads, agrees with the
body-derived direction in **25 of 25**. Two independent surfaces, one answer.

## Recall, and the five-plus-two it honestly misses

25 of 32 (78%). The 7 refusals are all real wires, and each is refused for a stated reason:

- `TGA_O()`, `TGC_O()`, `TGA_I`, `TGC_I` — their bodies name the **type** (*"Address tag type
  `[TGA_O()]` contains information associated with address lines `[ADR_O()]`"*) and state no sense at
  all, for this wire or any other. `.2` refused these too.
- `STALL_O` — *"The pipeline stall **signal** `[STALL_O]`"*, where its 25 siblings say *input* or
  *output*.
- `TGD_I()`, `TGD_O()` — the two above, refused by `.3` and admitted by `.2`.

## The `_I`/`_O` judgement `.2` handed to `.3`: REFUSED, on corpus evidence

Reading the `_I`/`_O` suffix as a direction would take recall to 32/32. `.2` deliberately left the
call to `.3` because ADR 0006 forbids a production decision that depends on one document's
convention, and a suffix-to-direction mapping is a convention unless it is universal grammar.

It is decidable, and it was decided by measurement rather than by argument. Across the persisted
corpus, **1,202 signals in 30 documents** carry a direction declared by an independent authority —
a signal-description table — and of those, **0 carry an `_I`, `_O`, `_IN` or `_OUT` suffix.** The
convention appears in **no other document in the corpus**: GIC-600 writes `SAMPLE_REQ` for an input
and `SAMPLE_ACK` for an output; AXI, MMU-700 and CoreSight SDC-600 name none of their 900-odd
directed signals with a direction suffix.

Contrast the one naming-shaped thing ADR 0006 does admit — logic levels — which the ADR admits
because they are *"assumed by every digital specification and defined by none."* `_I`/`_O` is
assumed by one document in 78 and corroborated by none of the 1,202 independent declarations.
**Decided: the suffix is not read.** The recall the rule earns is the recall the document's own
prose states, and the remaining 7 stay honest misses.

## Re-derivation

```bash
python3 scripts/measure_heading_declaration_shape.py             # the census (277 / 65 / 27)
python3 scripts/measure_heading_declaration_shape.py --landed    # what the SHIPPED rule produced
python3 scripts/measure_heading_declaration_shape.py --self-test # the RED cases
```

`--landed` does not re-implement the rule. It reads each document's persisted `EvidenceIR` and
reports the declarations whose name is a compound identifier heading and which carry **no** table
provenance — the product's own output, joined to the census, so the number cannot drift from the
producer that made it.
