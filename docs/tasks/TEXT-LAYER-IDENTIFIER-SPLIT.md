# TEXT-LAYER-IDENTIFIER-SPLIT: a name cell holds one identifier and the reader sees two words

## Metadata

- Tree ID: `TEXT-LAYER-IDENTIFIER-SPLIT`
- Status: `active` (`2026-09-14`; `.0` done — no rule; `.1`/`.2` open)
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

- ID: `TEXT-LAYER-IDENTIFIER-SPLIT.0` · Status: `done` (`2026-09-14`, PROBE/DOC) · **Measured; no rule,
  and the measurement is why.** Both legs ran.
  **The column signature is not a discriminator.** "A leading token repeated down the column with
  distinct continuations" selects **120 columns** corpus-wide (10 current, 110 legacy), and in the
  current stratum **not one of them is a name column**: all ten are *description* columns whose
  sentences happen to open with the same word — `Transaction identifier` / `Transaction address`,
  `Global clock` / `Global reset`, `User request` / `User write`. Nine legacy columns are chosen as a
  name column and five of those are not split identifiers either (`Write address` names an AXI
  *channel*; `Redundant Data` is a function). The signature is the shape; it is not the fact.
  **The in-document join is precise and repairs nothing.** Asking whether the document's own text
  anywhere spells `<lead>_<continuation>` selects **3 of the 120**, all AMD IOMMU field tables that
  also write `iommu_info` and `iommu_attributes` — 3 correct, 0 wrong. But all three are in tables
  that declare nothing, so a rule keyed on it would change no declaration in the corpus.
  **The case that opened the tree has no such evidence to join against.** TileLink's 36 underscored
  spellings are vector text inside three timing-diagram figures (`Figure 4.1 Ready-Valid Signaling`,
  4.3, 4.4). The ingest captures those as images — SpecForge holds `picture-0007/0009/0010.png` — and
  the only observations recorded on them are the source document's own captions. So the joining is
  stated in the document, is not in any text SpecForge reads, and **is** in a surface SpecForge already
  carries. That is `.1`.
  **A second notation was found and it is not an underscore.** eMMC `table_0221` writes `t PERIOD` and
  `t TLH , t THL` for `tPERIOD` and `tTLH`/`tTHL` — a *subscript* split with no separator — and it
  **declares**. A join rule keyed on `_` would not find it. That is `.2`.
  Verification: `python3 scripts/measure_split_identifier_name_cells.py`, read-only; the reader mirror
  is imported from `measure_parametric_width_cell_shapes.py`. PDF text-layer counts by `pypdf` over the
  repository-volume copy; figure-asset observations read from the persisted EvidenceIR.
  Commit: `TEXT-LAYER-IDENTIFIER-SPLIT.0`

- ID: `TEXT-LAYER-IDENTIFIER-SPLIT.1` · Status: `pending` (opened `2026-09-14` by `.0`) · Goal: **the
  only reachable spelling of TileLink's names is inside a figure SpecForge already stores.** Three
  timing diagrams carry `a_opcode`, `a_valid`, `a_ready`, `a_size`, `a_source`, `d_opcode`, `d_valid`,
  `d_ready`, `d_size`, `d_source` as vector text; the assets are persisted and their only observations
  are captions. Establish whether a VLM read of those three images recovers the ten names, and — this
  is the load-bearing half — whether a name recovered from a **figure** may ground a table row's
  identity at all, or whether it can only corroborate one. The doctrine that table declarations are
  authoritative was written when the alternative was prose, not an image.
  **Cost is stated because it decides the sequencing**: this needs a live VLM provider over three
  images, and TileLink cannot be rebuilt (no retained normalized bundle), so the recovery must be
  demonstrated on the assets directly rather than through a chain rebuild.
  Prerequisite: none. Verification: the three figures read with the provider named and pinned; the ten
  names compared against the PDF text layer as ground truth; a stated decision on figure-grounded
  identity before any rule.
  Commit: pending

- ID: `TEXT-LAYER-IDENTIFIER-SPLIT.2` · Status: `pending` (opened `2026-09-14` by `.0`) · Goal: **a
  subscript split has no separator, so nothing can be joined on.** eMMC `table_0221` declares from
  `t PERIOD`, `t TLH , t THL`; the document writes `tPERIOD`, `tTLH`, `tTHL`. The join is
  concatenation, not `<a>_<b>`, and the same cell also carries a comma family — so the one cell needs
  two readings at once. Census the corpus population of a single-letter lead followed by an
  upper-case continuation before proposing anything; a rule that concatenates two words is strictly
  more dangerous than one that inserts an underscore, because it leaves no mark of having guessed.
  Prerequisite: none. Verification: the population measured with the real reader and adjudicated;
  observed RED; every legitimate multi-token name shape shown to survive.
  Commit: pending

## Current Frontier

1. `TEXT-LAYER-IDENTIFIER-SPLIT.1` — read the three TileLink timing diagrams and decide whether a name
   recovered from a figure may ground a table row's identity. The identity question is the leaf, not the
   VLM call.
2. `TEXT-LAYER-IDENTIFIER-SPLIT.2` — the subscript notation (`t PERIOD` for `tPERIOD`), which declares
   and which no underscore-keyed rule can reach. Census first.

## Decisions

- `2026-09-14` — **opened as its own tree rather than as a leaf of `PROSE-NAME-CELL-DECLARATION`.** That
  tree is about what a name cell *says*; this is about a cell whose text is already wrong when it arrives.
  They meet at one function and nowhere else, and `PROSE-NAME-CELL-DECLARATION` has no eligible frontier
  left — folding a new question into it would reopen a closed tree to host an unrelated one.
- `2026-09-14` — **the first leaf is a census with no code change.** Every rule this reader has been given
  over-fired until its selection was inspected, and this one would join two words into an identifier,
  which is the most expensive kind of mistake available here.
- `2026-09-14` — **`.0` shipped no rule even though one of its two tests is precise.** The in-document
  join selects 3 of 120 and is right about all three, which is the best selectivity any rule in this
  area has shown. It still does not ship, because all three are in tables that declare nothing: a rule
  whose entire measured effect is zero declarations is a rule with no evidence behind it, and the next
  document to exercise it would be its first test rather than its hundredth.
- `2026-09-14` — **the ingest is not at fault and the tree says so.** The underscore was never in the
  text stream: it is vector text inside figures, which the ingest correctly captures as images. Blaming
  the ingest would have sent the repair to the wrong stage.

## Open Questions

- ~~Does the underscore survive ingest for a document whose text layer has it?~~ **Answered by `.0`, and
  the premise was wrong.** The underscore is not on code-listing pages and is not lost by the ingest: it
  is vector text inside three timing-diagram figures, which the ingest captures as images by design. The
  text stream never had it.
- ~~Is "every cell in the column leads with the same token" a sufficient signature?~~ **No, measured.**
  It selects 120 columns corpus-wide and the current stratum's ten are all description columns. A real
  catalog shares a *prefix* (`AWADDR`, `AWLEN`) and not a whitespace-delimited leading token — but so
  does a column of sentences that open with the same word, and the second population is far larger.
- May a name read from a FIGURE ground a table row's identity, or only corroborate one? `.1`. The
  authority doctrine was written when the alternative to a table was prose.
- Can the continuation token ever be trusted alone? `t TLH , t THL` is two split identifiers and a comma
  family in one cell, and no rule proposed so far reads it (`.2`).

## Blockers

None.

## Verification Log

- `2026-09-14` — `.0`. `python3 scripts/measure_split_identifier_name_cells.py` over all 573 boundary
  tables: 120 columns carry the signature (10 current, 110 legacy), each printed verbatim with its
  header, its cells, its role (header-designated / chosen / declaring) and any in-document join. Of the
  9 legacy columns the reader CHOOSES, 3 carry a join and all 3 are AMD IOMMU field tables; the current
  stratum's 10 are all description columns and none is chosen. Read-only: no artifact written, rebuilt
  or mutated. Figure evidence: `generated/evidence_ir/tilelink_1_8_0_specification/evidence_ir.json`
  visual evidence `visual_0021`/`visual_0023`/`visual_0024` (assets `picture_0007`/`0009`/`0010`, pages
  27/33/34) exist with caption-only observations and no VLM read; `pypdf` over the repository-volume PDF
  puts all 36 underscores on exactly those pages.
- `2026-09-14` — opened. Evidence carried over from `PROSE-NAME-CELL-DECLARATION.5`:
  `python3 scripts/measure_name_column_whole_cell_score.py` (11 zeroed name columns corpus-wide, all
  prose, TileLink not among them), the column profile computed with that script's own `column_profile`
  over TileLink `table_0012`/`table_0013`, and a `pypdf` read of
  `.cache/local-references/chipdoc/risc-v/interfaces/tilelink/current/TileLink-1.8.0_Specification.pdf`
  (repository-volume, read-only): 36 underscores on 5 pages, 10 distinct underscored names, against 0 in
  the persisted SourceIR.

## Commit Log

- Opened in the commit that closed `PROSE-NAME-CELL-DECLARATION.5`.
- `.0` — `TEXT-LAYER-IDENTIFIER-SPLIT.0`.

## Changelog

- `2026-09-14` — `.0` closed with **no rule**. The column signature selects 120 columns and the current
  stratum's ten are all description columns; the in-document join selects 3 of 120 and is right about
  all three, but all three are inert. TileLink's joining spelling is vector text inside three figures
  the ingest correctly stores as images — so the ingest is not at fault and the evidence is reachable
  only through the VLM path (`.1`). eMMC's `t PERIOD` is a subscript split with no separator (`.2`).
- `2026-09-14` — created from `PROSE-NAME-CELL-DECLARATION.5`'s census, which falsified that leaf's own
  premise: TileLink's rotation is not the whole-cell column score's doing under either score, and the
  cause is a name cell holding one identifier the text layer split in two.
