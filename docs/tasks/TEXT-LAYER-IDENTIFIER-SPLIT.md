# TEXT-LAYER-IDENTIFIER-SPLIT: a name cell holds one identifier and the reader sees two words

## Metadata

- Tree ID: `TEXT-LAYER-IDENTIFIER-SPLIT`
- Status: `active` (`2026-09-14`; `.0` open)
- Roadmap lane: `R2` (extraction correctness / wire recall), with an ingest leg in `R15`
- Created: `2026-09-14`
- Last updated: `2026-09-14`
- Owner: repo-local workflow

## Goal

A specification writes `c_opcode`. Its PDF typesets the underscore, the text layer does not carry it,
and SpecForge reads the cell `c opcode` — two tokens, of which the reader keeps the first. The wire is
not dropped and it is not refused; it is **renamed to one letter**, and every row of the table is
renamed to the same letter, so the column offers one distinct name and loses the table to whichever
neighbour offers more.

Establish how many identifiers the corpus loses this way, whether the evidence to rejoin them exists
anywhere SpecForge can reach, and — only then — whether a rejoin can be grounded in the document rather
than in a vocabulary.

## How it was found

`PROSE-NAME-CELL-DECLARATION.3` refused 47 prose width cells corpus-wide, and **43 of them were
TileLink** — rows whose declared name is `C`, `D`, `V` or `R` from a `Type` column. `.5` was opened on
the natural reading, that `.2`'s whole-cell column score had zeroed a real name column of two-token
names, and the census **falsified it**:

| TileLink `table_0012` column | whole-cell score (`.2`) | leading-token score (before `.2`) |
| --- | ---: | ---: |
| 0 `Signal` — `c opcode`, `c param`, `c valid`, … | 0 | **1** |
| 1 `Type` — `C`, `D`, `V`, `R` | **4** | **4** |

`Type` clears the override margin against column 0 under **both** scores. The rotation predates `.2` and
survives its removal. What actually costs TileLink its column is that `c opcode` is *one* identifier:
every row's leading token is `c`, so the column can offer exactly one name however it is scored.

## The evidence exists in the document and does not survive ingest

The TileLink 1.8.0 PDF's own text layer carries **36 underscores on 5 pages** — `a_opcode`, `a_valid`,
`a_ready`, `a_size`, `a_source`, `d_opcode`, `d_valid`, `d_ready`, `d_size`, `d_source`, all inside code
listings — while the prose and every table cell spell the same names with a space (101 spaced
occurrences of the same forms in the first 40 pages alone).

SpecForge's persisted SourceIR for that document carries **zero** underscore characters: none in
`content_elements`, none in a table cell. So the one place the document states the joining is lost
before any reader sees it. Four of 78 documents carry no underscore at all
(`den0068`, both TileLink revisions, `um11732` I2S); the next-lowest carries two.

This is a **different** defect from `[[evidence-statement-markdown-escape-truncates-identifiers]]`, and
the difference decides where a fix can live: there the underscore is present and *escaped* (`CYC\_O`), so
the identifier is truncated at a backslash; here the underscore is **absent**, so there is nothing to
truncate and nothing to unescape.

## Non-Goals

- Do not add a chip, vendor, protocol or English vocabulary, and do not add a list of known signal names
  (ADR 0006). A rejoin must be grounded in the document that states it.
- Do not join two tokens because they *could* be an identifier. `Clock source` would join to
  `Clock_source`; the whole of `PROSE-NAME-CELL-DECLARATION` exists because that cell is prose.
- Do not treat this as the escape defect above; do not widen that fix to cover it.
- Do not repair the corpus artifacts before the class is measured. The two documents found so far are
  legacy and unrebuildable, so a measurement that depends on rebuilding them measures nothing.

## Acceptance Criteria

- The population is measured **per document** with a reproducible instrument: how many name cells in the
  corpus are a leading token plus a continuation, how many of them reach a declaration, and how many
  identifiers that costs.
- The ingest leg is answered as a fact rather than a suspicion: for a document whose PDF text layer
  carries underscores, whether SourceIR keeps them, and where in the ingest they go.
- Any rejoin rule ships with a corpus-wide count of what it newly joins **and an adjudicated sample**,
  and every legitimate multi-token name shape (`AWSIZE, ARSIZE`, `HSELx a`, `ARMPAM [10:0]`) is shown to
  survive it unchanged.
- No gold score moves down; `scripts/check_doctrines.sh` green; no ceiling, milestone or contract widened.

## Task Tree

- ID: `TEXT-LAYER-IDENTIFIER-SPLIT` · Status: `active` (`2026-09-14`) · Children: `.0`

- ID: `TEXT-LAYER-IDENTIFIER-SPLIT.0` · Status: `pending` (opened `2026-09-14`) · Goal: **measure the
  class before proposing anything.** Two independent measurements, neither of which changes code:
  (a) over the persisted corpus, every `signal_description` name cell whose shape is `<identifier>
  <continuation>`, classified by whether the whole column shares the leading token — the signature that
  separates a split identifier from a phrase, since `c opcode`/`c param`/`c valid` all lead with `c`
  while `Clock source`/`Reset controller` do not; and (b) over the source PDFs SpecForge can still reach
  on the repository volume, whether the text layer carries underscores the persisted SourceIR does not.
  Two documents are already known to carry the class — TileLink (both revisions) and eMMC `table_0221`
  (`t PERIOD`, `t TLH , t THL`) — and eMMC's **declares**, so the class is not one publisher's
  typesetting.
  Non-goal: any code change; any rejoin rule. The leaf is allowed to conclude that the evidence needed
  to rejoin is not reachable and that the honest outcome is an accounted refusal rather than a repair.
  Prerequisite: none. Verification: read-only over persisted artifacts and repository-volume PDFs; no
  artifact written or mutated.
  Commit: pending

## Current Frontier

1. `TEXT-LAYER-IDENTIFIER-SPLIT.0` — the two censuses above. Run (b) first if it is cheaper: if the
   underscore survives ingest for documents that have it, the ingest leg is closed and the whole tree is
   about the cells that never had one.

## Decisions

- `2026-09-14` — **opened as its own tree rather than as a leaf of `PROSE-NAME-CELL-DECLARATION`.** That
  tree is about what a name cell *says*; this is about a cell whose text is already wrong when it arrives.
  They meet at one function and nowhere else, and `PROSE-NAME-CELL-DECLARATION` has no eligible frontier
  left — folding a new question into it would reopen a closed tree to host an unrelated one.
- `2026-09-14` — **the first leaf is a census with no code change.** Every rule this reader has been given
  over-fired until its selection was inspected, and this one would join two words into an identifier,
  which is the most expensive kind of mistake available here.

## Open Questions

- Does the underscore survive ingest for a document whose text layer has it? The TileLink evidence says
  no for that document, but 36 underscores on 5 code-listing pages may simply be pages the ingest did not
  capture at all — which is a different finding and a different fix.
- Is "every cell in the column leads with the same token" a sufficient signature, or does a real catalog
  ever share a leading token across a whole column? An AXI channel table's names share a *prefix*
  (`AWADDR`, `AWLEN`) but not a whitespace-delimited leading token, which is the distinction to measure.
- Can the continuation token ever be trusted alone? `t TLH , t THL` is two split identifiers and a comma
  in one cell, and no rule proposed so far reads it.

## Blockers

None.

## Verification Log

- `2026-09-14` — opened. Evidence carried over from `PROSE-NAME-CELL-DECLARATION.5`:
  `python3 scripts/measure_name_column_whole_cell_score.py` (11 zeroed name columns corpus-wide, all
  prose, TileLink not among them), the column profile computed with that script's own `column_profile`
  over TileLink `table_0012`/`table_0013`, and a `pypdf` read of
  `.cache/local-references/chipdoc/risc-v/interfaces/tilelink/current/TileLink-1.8.0_Specification.pdf`
  (repository-volume, read-only): 36 underscores on 5 pages, 10 distinct underscored names, against 0 in
  the persisted SourceIR.

## Commit Log

- Opened in the commit that closed `PROSE-NAME-CELL-DECLARATION.5`.

## Changelog

- `2026-09-14` — created from `PROSE-NAME-CELL-DECLARATION.5`'s census, which falsified that leaf's own
  premise: TileLink's rotation is not the whole-cell column score's doing under either score, and the
  cause is a name cell holding one identifier the text layer split in two.
