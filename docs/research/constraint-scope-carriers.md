# A constraint read from a list item or a table row loses the scope its lead-in or caption states (`KG-ISF-COMPLETENESS.5.xiii.a.3`)

`.5.xiii.a` made three AXI constraints visible — `AWBURST must be INCR`, `AWBURST must be WRAP`,
`ARBURST must be INCR` — that hold only under a list lead-in (*"The rules for atomic transactions are
as follows:"*) or a table caption (*"Table A15.23: DVM Complete request constraints"*) and are
published with no `condition_text`. The same table already published `ARADDR must be zero` the same way.
This measures the class before a repair is chosen. Reproduce with:

```bash
python3 scripts/measure_constraint_scope_carriers.py --sample 40 --seed 13
```

## The census is layout, not vocabulary

Every `signal_constraints` record in all 78 persisted `evidence_ir.json` files is traced to its first
supporting statement. The statement's shape is read off its text (a list marker, a serialized table
row, or prose), and a bullet or row has a **scope carrier** when the document states one before it: a
colon-terminated lead-in, a colon-terminated bullet of the same list (a nested lead-in whose sub-items
the text layer flattened to one level), or a `Table …` caption. No word list decides anything; whether
a carrier restricts is decided by hand below.

| stratum | shape | records | with a scope carrier | lead-in | nested lead-in | caption |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| replayable | bullet | 50 | 46 | 32 | 14 | 0 |
| replayable | table row | 33 | 33 | 0 | 0 | 33 |
| replayable | prose | 20 | 0 | — | — | — |
| legacy | bullet | 62 | 51 | 42 | 9 | 0 |
| legacy | table row | 87 | 56 | 0 | 0 | 56 |
| legacy | prose | 106 | 0 | — | — | — |

**186 of 358 records (52%) sit under a scope carrier** — 79 of the 103 the current producer mints.

## Forty adjudicated by hand: the scope that restricts is lost every time

A seeded sample of 40 scoped records, each carrier read against its document:

| carrier | records | record carries a `condition_text` |
| --- | ---: | ---: |
| **restricting** — *"During reset, the following interface requirements apply:"*, *"The rules for atomic transactions are as follows:"*, *"Table A15.23: DVM Complete request constraints"*, *"When FRESETn is asserted:"*, *"Table D8-2 Barrier response transaction constraints"*, the AXI4-Lite upgrade procedure | 16 | **0** |
| **row-keyed** — the condition is the row's own key column (`AWTAGOP = Match`, `LTI_MMU = True LTI_GPC = False`), not the caption | 8 | 2 |
| **neutral** — *"The User signals protocol rules are:"*, *"Table B5.1 Continued from previous page"*, signal-description captions | 16 | 14 |

Every `condition_text` in the table comes from the item's OWN clause — by construction, since the
extractor reads only the statement that binds — never from the carrier.

So **40% of the sampled scoped records hold only under a scope the record drops, and none of them keeps
it**; another 20% hold only for a row key the record drops. Extrapolated with the sample's width in
mind, that is on the order of 70 records under a restricting carrier and 35 under a row key, across 186.
`During reset … must drive RACK LOW` is published as *"RACK must be LOW"* — true in reset and false in
operation.

## Why the repair cannot be "copy the lead-in into the condition"

Sixteen of the forty carriers are neutral, and a neutral lead-in copied into `condition_text` is a
fabricated condition (*"when the User signals protocol rules are"*). Telling the two apart needs the
meaning of the lead-in, and ADR 0006 forbids deciding it with a word list. What the structure CAN
state, without reading any meaning, is that a scope carrier EXISTS.

## Decision

1. **`.5.xiii.a.3.a` records the carrier, not a reading of it.** A constraint record gains the scope
   carrier's text and kind (lead-in, nested lead-in, caption) as typed provenance, so every consumer can
   see that the document stated a scope the record does not encode. That much is structural and moves
   no published fact.
   **Whether an unconditioned scoped record may still lower is NOT decided here**, and the obvious rule
   — residualize them all — is refuted by this sample: the neutral 40% includes constraints the
   WIRE-BASED-100 golds score (APB `PWUSER`/`PAUSER` under *"User signal descriptions"*, AHB
   `HAUSER`/`HBUSER` under *"The User signals protocol rules are:"*). `.a` must score the golds with
   and without any lowering gate before choosing one.
2. **Row keys are a different shape** (the condition is a sibling column, like `.5.xiii`'s multi-key
   table) and are not folded into `.a`.
3. **Nested lists are flattened** — 23 of the 97 scoped bullets depend on a nested lead-in the text
   layer lost; the carrier search reconstructs it, and `.a` must use the same reconstruction.

## Authoritative tracking

`docs/tasks/KG-ISF-COMPLETENESS.md` — `.5.xiii.a.3` (this measurement) and `.5.xiii.a.3.a` (the repair).
Instrument: `scripts/measure_constraint_scope_carriers.py`. Knowledge Map:
`[[constraint-scope-carrier-is-lost]]`.
