---
id: a-cheap-structural-rule-overfires-until-you-read-its-selection
title: A cheap structural rule over-fires until someone reads WHAT it selects — a count cannot tell a port sense from a property value, and three measured instances say so
answers:
  - "why must I adjudicate a census selection instead of trusting its count (three measured instances: a suffix rule read a naming template as a catalogue, an enumerated-width rule selected a bus-mode matrix, and a literal-direction rule admitted 18 rows on O meaning Optional)"
  - "why did the literal-direction-column census report 124 rows and then 106 (18 were admitted on the single letter O, which AMBA LTI table_0081 and AXI-Stream table_0015 use for Optional beside N for not-present and C for conditional - protocol-VERSION presence matrices, not direction columns)"
  - "is it safe to put single-letter abbreviations in a notation census vocabulary (no - measured 0 true positives and 18 false ones corpus-wide for i/o/io/in/out; an abbreviation is not a notation until a document is shown to use it as one)"
  - "what is the difference between a census that OVER-fires and one that UNDER-reads (over-firing selects things that are not what you think, and the remedy is to read the selection; under-reading sees one spelling of a notation and misses the others, and the remedy is to census the spellings - both are 'a count is not an adjudication' but they fail in opposite directions)"
  - "how do I know a structural rule's selection is clean (print every distinct selected form verbatim with its count and read them; when the population is small enough the adjudicable sample IS the population, as with the 83 flow-arrow cells in 13 forms)"
  - "can adding recall activate a bug that was previously harmless (yes - SIGNAL-DECLARATION-ROW-DROP.2h.1: the name-column override had already mis-picked HBM2 table_0076's Status column, but those rows were being dropped for having no direction; giving them one turned the inert bad guess into four phantom signals named X, V and Active)"
  - "why does a presence matrix look identical to a signal table to a structural rule (both are a name column beside a short-token column; N/O/C for not-present/optional/conditional occupies exactly the shape a Type column occupies, so only the meaning of the tokens separates them and only reading them recovers it)"
date: 2026-09-15
status: current
tags: [census, adjudication, false-positive, adr-0006, signal-tables, wire-based-100, signal-declaration-row-drop, method]
evidence: docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.2b, .2c, .2h.0); docs/tasks/WIRE-BASED-100.md (.10b); scripts/measure_declaration_row_notations.py (LITERAL_DIRECTION_WORDS); generated/source_ir/ihi0089_d_2025_08_amba_lti_protocol_specification/source_ir.json (table_0081); generated/source_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/source_ir.json (table_0015); generated/source_ir/jesd84_b50_2013_09_emmc_5_0/source_ir.json (table_0020)
reverify: "python3 scripts/measure_declaration_row_notations.py — expect literal-direction-column: 106 cells across 13 tables, every row admitted on the full word input or output and none on an abbreviation"
---

This repository's most-repeated extraction lesson, with three measured instances and one control.
A structural rule is cheap to write and its count is cheap to read; **what it selects is neither**, and
the gap between them is where the false positives live.

## The three instances

**A suffix rule read a naming template as a catalogue.** AMBA AXI's *Credited channel signals* table
lists `VALID`, `PENDING`, `RP`, `CRDT`, `CRDTSH`, `SHAREDCRD` — the pattern, written once, instantiated
as `AWVALID`, `ARVALID`, `WCRDT`. Read as a catalogue it put six ports into the manager's emitted
interface *beside their own instantiations*, carrying 9 signal constraints and 23 ISF rules addressed to
wires that do not exist (`[[base-name-template-table-is-not-a-catalogue]]`).

**An enumerated-width rule selected a bus-mode matrix.** `SIGNAL-DECLARATION-ROW-DROP.2c` censused 7
cells listing legal widths. **Four were not signal widths at all**: the `Bus Width` column of eMMC
`table_0020`, a mode matrix whose name column holds mode names. Teaching the reader that notation would
have declared `Backwards`, `High`, `High` and `HS200` as signals — one phantom becoming five. The leaf
was **deferred on its own adjudication**, and the reason was worth more than the leaf.

**A literal-direction rule admitted eighteen rows on `O`.** `SIGNAL-DECLARATION-ROW-DROP.2d` published
**124 rows in 15 tables** whose direction is stated outright in a column the header scan never reads.
`.2h.0` read all 15. **Eighteen were admitted on the single letter `O`** — AMBA LTI `table_0081`
("Summary of parity signal presence of each LTI version") writes `LAVALIDCHK | N | O`, and AXI-Stream
`table_0015` writes `TREADY … | O | O` beside a `property` column of `O`/`C`. These are
protocol-**version presence matrices**: `N` not-present, `O` optional, `C` conditional. Reading `O` as
`Output` would have **declared `TREADY` an output because a presence matrix called it optional**.
Corrected population: **106 rows, 13 tables, 4 documents**, every row admitted on the full word.

**A fourth instance, and the only one where the over-firing rule was the AUTHOR'S OWN, not a census.**
`SIGNAL-DECLARATION-ROW-DROP.2h.1` taught the reader to use a column whose cells are the literal
direction words. Run unscoped against HBM2 `table_0076` — a table whose header row is itself data — it
produced **`Signal X is input.`, `Signal V is output.`, `Signal Active is input.`**, read out of the
`Status` column, where it had produced **nothing**. The mechanism is worth more than the instance:
the pre-existing content name-column override had *already* guessed that column wrongly, and the guess
was inert only because those rows were being DROPPED for having neither a direction nor a width.
Supplying a direction woke a latent bad guess. **Two guesses do not compose** — the fix scopes the new
rule to tables whose name column the header designates, at a measured cost of six genuine wires in one
other table. The lesson generalises past censuses: adding recall to a pipeline can activate an upstream
error that was previously harmless, and a rule's selection must be read *after* it is wired in, not
only where it is defined.

## Why a presence matrix is indistinguishable by shape

A structural rule sees *a name column beside a column of short repeated tokens*. A `Type` column holding
`Input`/`Output` and a version column holding `N`/`O`/`C` are **the same shape**. Only the meaning of the
tokens separates them, and meaning is exactly what a count does not carry. This is why the remedy is
never a better threshold — it is printing every distinct selected form verbatim and reading it.

**The corollary, and it is the cheap rule:** an abbreviation is not a notation until a document is shown
to use it as one. `i`, `o`, `io`, `in`, `out` were in the census vocabulary for one revision on the
assumption that a `Type` column abbreviates. Measured across all 78 stored artifacts: **0 true positives,
18 false.** They were removed rather than special-cased, on the same bar `.2b` applied when it refused
to read a leftward arrow as a flow — a rule with no population behind it is disqualified, whichever
direction the error runs.

## Over-firing is not the only failure, and the remedies differ

Both failures are "a count is not an adjudication", but they run in opposite directions and a session
that confuses them will apply the wrong fix:

| | what goes wrong | the remedy |
| --- | --- | --- |
| **over-fires** | the rule selects things that are not what you think | read the selection, form by form |
| **under-reads** | the rule sees one spelling and misses the others | census the spellings before trusting the denominator |

The under-reading twin is `[[a-single-index-bit-cell-is-a-width-of-one]]`: a bit-cell census that read
`[hi:lo]` and missed the single-index `[n]`, which is how every one-bit wire is written. Its bias was not
random — the dropped rows were exactly the valid/qualifier/handshake signals — so it *looked* complete.

## The control: what a clean selection looks like

`.2b`'s flow-arrow grammar found **83 cells in 2 documents and 13 distinct forms** — few enough that the
adjudicable sample **is** the population, which is why its in-crate control enumerates all thirteen. All
18 admitted cells were genuine direction statements: **zero false positives**. That is the standard the
other three failed to meet on the first try, and it is reachable whenever the population is small enough
to print.

Links: [[flow-arrow-direction-grammar]], [[declaration-reader-drops-uninterpretable-rows]],
[[actor-taxonomy-grows-in-pairs-not-terms]].
