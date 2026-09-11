# PROSE-NAME-CELL-DECLARATION: a row whose name cell is a phrase declares its first word as a signal

## Metadata

- Tree ID: `PROSE-NAME-CELL-DECLARATION`
- Status: `active` (`2026-09-11`; opened by `SIGNAL-DECLARATION-ROW-DROP.2c`'s census, `.0` open)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-11`
- Last updated: `2026-09-11`
- Owner: repo-local workflow

## Goal

`synthesize_signal_declarations` reads a row's name cell by taking its **first whitespace token**,
trimming the token's non-identifier characters, and asking `is_hardware_signal_token`. That is right
for `HSELx a` (a footnote marker) and for `readdatavali d readdatavali` (a PDF text-layer artefact of
one real signal). It is wrong for a cell that is a **phrase**: `Backwards Compatibility with legacy
MMCcard` declares a signal called `Backwards`, `Row Command/Address` declares `Row`, `OEM ID`
declares `OEM`, `a opcode` declares `a`.

Establish how many declarations in the corpus are minted this way, and separate the phrase cells from
the several legitimate multi-word forms — then decide what, if anything, can be refused without
losing a real wire.

## How it was found

`SIGNAL-DECLARATION-ROW-DROP.2c` was going to teach the reader that a width cell listing legal widths
(`8, 16, 32, 64`) is a width set. Its census found 7 such cells. Adjudicating them — before writing
code, which is this repository's standing rule — showed that **4 of the 7 are not signal widths at
all**: they are the `Bus Width` column of eMMC `table_0020`, a bus-mode matrix
(`Mode Name | Data Rate | IO Voltage | Bus Width | Frequency | Max Data Transfer`) that SourceIR typed
`signal_description`. Its name column holds mode names.

That table already mints a signal today. Its `HS400` row is the one whose Bus Width cell is a single
value (`8`) rather than a list, so it parses, and the persisted EvidenceIR for `jesd84_b50…` carries
**`HS400` in `table_signal_declaration_provenance` for `table_0020`**. The other four rows are held
back only by the width notation `.2c` was about to teach. Reading it would have taken that table from
one phantom signal to five — the exact over-firing this repository keeps re-learning
(`[[base-name-template-table-is-not-a-catalogue]]`, `[[property-table-is-not-a-signal-inventory]]`,
`[[bracketed-metavariable-name-cell]]`).

## Non-Goals

- Do not refuse every multi-word name cell. Several are legitimate and load-bearing: a comma-separated
  family (`AWSIZE, ARSIZE`), a footnote marker (`AWUNIQUE a`), a name plus a bit range
  (`response [1:0]`), and a text-layer split of one identifier
  (`waitrequest waitrequest _ n`). `signal_names_in_name_cell` already handles the first of these.
- Do not solve it at the table level. `WIRE-BASED-100.10e` already refuses a *property* table by its
  caption, and that mechanism does not see a mode matrix. This tree is about the **row**.
- Do not add document, vendor, or protocol vocabulary, and do not add an English word list (ADR 0006).
  A rule that needs to know `Backwards` is a word and `AWSIZE` is not has not been found yet, and the
  first leaf is allowed to conclude that no such rule exists cheaply.

## Acceptance Criteria

- The population is measured **through the reader**, not through an approximation: how many entries in
  the persisted `table_signal_declaration_provenance` come from a name cell that is a phrase, per
  document, with every distinct cell form listed for adjudication.
- Any rule shipped carries a corpus-wide count of what it newly refuses **and an adjudicated sample**,
  and the legitimate multi-word forms above are each shown to survive it.
- No gold score moves down; the wire golds are re-scored, not assumed. AXI is the risk here — 35
  candidate rows, of which the `AWSIZE, ARSIZE` comma families are real.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `PROSE-NAME-CELL-DECLARATION` · Status: `active` (`2026-09-11`) · Children: `.0`

- ID: `PROSE-NAME-CELL-DECLARATION.0` · Status: `pending` · Goal: **measure before proposing anything.**
  A read-only census over persisted SourceIR + EvidenceIR pairs: for every entry in
  `table_signal_declaration_provenance`, recover the source row that produced it and classify its name
  cell — single token / comma family / footnote-marked / bracket-suffixed / text-layer split / phrase.
  Report per document and list every distinct phrase cell, because the adjudication is the deliverable
  and a count is not.
  **Known starting point, already measured:** a first approximation — name cells with two or more
  whitespace tokens that do not share a two-character prefix or suffix, whose first token is an
  identifier — selects **345 rows across 28 documents**. That number is a *candidate* population and is
  known to be contaminated: it includes AXI's comma families, `AWUNIQUE a`, `response [1:0]` and
  `waitrequest waitrequest _ n`, all legitimate. The real number is smaller and must come from the
  reader's own provenance, not from this filter. State both, and state the difference.
  Non-goal: any code change.
  Prerequisite: none.
  Verification: read-only; no artifact written or mutated.
  Commit: pending

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `PROSE-NAME-CELL-DECLARATION.0` — the census. Nothing else in this tree may start before it.

## Decisions

- `2026-09-11` — **opened as its own tree rather than a leaf of `SIGNAL-DECLARATION-ROW-DROP`.** That
  tree is about rows the reader *drops*; this is about rows it *should* drop. They meet at one function
  and nowhere else, and folding a precision question into a recall tree would make both frontiers
  unreadable.
- `2026-09-11` — **the first leaf is a census with no code change.** Three separate rules in this
  reader have now over-fired until their selection was inspected, and the one number available today
  (345) is already known to contain false positives. A rule proposed before the adjudication would be
  the fourth.

## Open Questions

- Is there a shape-only discriminator at all? The distinguishing property of `Backwards Compatibility
  with legacy MMCcard` against `waitrequest waitrequest _ n` may be irreducibly lexical, in which case
  the honest outcome is a residual rather than a rule — and saying so, with the measurement behind it,
  is a complete result.
- Does the answer belong at the row (this tree), the table (`WIRE-BASED-100.10e`'s mechanism), or the
  SourceIR classifier that typed a bus-mode matrix as `signal_description` in the first place?

## Blockers

None.

## Verification Log

Pending: `.0` is a read-only census.

## Commit Log

Opened in the commit that deferred `SIGNAL-DECLARATION-ROW-DROP.2c`.

## Changelog

- `2026-09-11` — tree created from `SIGNAL-DECLARATION-ROW-DROP.2c`'s adjudication, which found 4 of 7
  enumerated-width cells to be a misclassified bus-mode matrix already minting `HS400` as a signal.
