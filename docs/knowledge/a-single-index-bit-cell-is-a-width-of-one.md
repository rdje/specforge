---
id: a-single-index-bit-cell-is-a-width-of-one
title: A bit-allocation column writes single-bit wires as `[n]`, not `[n:n]` — so a census or rule that reads only `[hi:lo]` recovers the wide buses and silently drops every handshake and qualifier signal beside them
answers:
  - "does a Bits column write a single-bit signal as [n] or [n:n]"
  - "why did the bit-range census miss lavalid and lrvalid"
  - "what width is the cell [0]"
  - "which notations must a bit-range width rule read"
  - "why does reading only [hi:lo] bias a recall census toward wide buses"
  - "how was the bit-range census error caught"
  - "how many bit cells are in signal_description tables corpus-wide"
date: 2026-09-15
status: current
tags: [evidence-ir, tables, width, notation, census-method, signal-declaration-row-drop, measurement]
evidence: scripts/measure_bit_range_width_cells.py (BIT_RANGE, bit_range_width); generated/source_ir/101542_0102_08_2023_07_05_arm_mmu_700_technical_reference_manual/source_ir.json (table_0259, table_0260); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2f, .4d)
reverify: "python3 scripts/measure_bit_range_width_cells.py --self-test (12/12) then --json: expect 279 bit cells over 15 signal_description tables in 4 documents; SIGNAL scope 9 tables / 241 rows / 146 declared / 95 not (19 of them `Unused`). Restricting BIT_RANGE to the ranged form alone drops these to 167 / 130 / 81 / 49 — the difference is every 1-bit wire."
---

MMU-700's LTI observation-interface tables (`table_0259`, `table_0260`) allocate each signal a slice
of a 128-bit group under a `Bits` column. A multi-bit wire is written as a range and a **one-bit wire
is written as a bare index**:

```text
SIGNALGRP<n> | Bits      | Signal name | SIGQUAL<n> 4'b{MSB..LSB} | Number of cycles of delay
0            | [64:1]    | laaddr      | 3'b000 , lavalid         | 1
0            | [0]       | lavalid     | 3'b000 , lavalid         | 1
1            | [122]     | laogv       | 3'b000 , lavalid         | 1
2            | [37]      | lrctag      | 3'b000 , lrvalid         | 1
```

`[0]` is a width of **1**, and so is `[122]`. A rule or census written to the spelling
*"`[hi:lo]` is `hi - lo + 1`"* matches none of them.

**The bias this creates is not random, which is what makes it dangerous.** The rows it drops are
exactly the one-bit wires — `lavalid`, `lrvalid`, `laogv`, `lassidv`, `lasecsid`, `lrctag`,
`lcctag_*` — the valid/qualifier/handshake signals. A recall census restricted to the ranged form
therefore reports the address and data buses and misses the protocol's own handshake, while looking
complete.

**Measured corpus-wide over `signal_description` tables**: reading both forms finds **279 bit cells
in 15 tables across 4 documents** (SIGNAL scope: 9 tables, 241 rows, 146 already declared, 95 not).
Reading only the ranged form finds **167 / 130 / 81 / 49**. The gap — 112 rows — is entirely 1-bit
wires.

**How it was caught, because the method generalizes.** The first cut of the census published a trade
for `SIGNAL-DECLARATION-ROW-DROP.2f` built on the ranged form. The check that broke it was a
composition question, not a re-reading: `.4d` had handed `.2f` 47 rows on the claim that a bit rule
would give them a readable width, so **are those rows actually in bit-cell rows?** On the ranged-only
reading only **20 of 48** were — a contradiction between two numbers that had to agree. With `[n]`
read, **46 of 48** are. *Asking whether two published numbers compose is a cheaper falsification than
re-deriving either one, and it is the one that finds a notation gap.*

See `[[a-width-cell-that-is-a-sentence-is-not-a-width]]`,
`[[a-dropped-declaration-row-is-usually-not-a-signal]]`,
`[[declaration-replay-reads-the-legacy-stratum]]`.
