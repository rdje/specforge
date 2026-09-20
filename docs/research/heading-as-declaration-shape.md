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
