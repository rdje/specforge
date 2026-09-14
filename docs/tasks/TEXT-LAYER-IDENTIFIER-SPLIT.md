# TEXT-LAYER-IDENTIFIER-SPLIT: a name cell holds one identifier and the reader sees two words

## Metadata

- Tree ID: `TEXT-LAYER-IDENTIFIER-SPLIT`
- Status: `active` (`2026-09-14`; `.0`/`.2` done — no rule from either; **`.1` open** — the VLM/figure leg)
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

- ID: `TEXT-LAYER-IDENTIFIER-SPLIT.2` · Status: `done` (`2026-09-14`, PROBE/DOC) · **Measured; no
  rule — and the census found something the leaf did not go looking for: the SAME CELL SHAPE carries
  TWO DIFFERENT JOINS, and nothing in the cell says which.**
  **The population, over all 573 boundary tables** (`python3 scripts/measure_subscript_split_name_cells.py`,
  read-only). Scope is the column the reader actually reads — header-designated or reader-chosen —
  because no other cell can become a declaration; the 1,677 matches outside it across 38 documents are
  counted as the false-positive surface rather than dropped:

  | signature, in the NAME column | cells | with an in-document concatenation |
  | --- | ---: | ---: |
  | tier A — continuation is UPPER-CASE (`t PERIOD`) | **9** | **2** |
  | tier B — continuation is any word (`t PERIOD` *and* `a opcode`) | **126** | **2** |

  **Tier B is what a naive "join two adjacent tokens" rule would take, and 81 of its 126 are
  TileLink** — `a opcode`, `b param`, `c valid`, … across both revisions' five channel tables. Their
  correct join is an **underscore** (`a_opcode`), never a concatenation (`aopcode`), and `.0` already
  established that. So tier B is not a wider version of tier A: **it merges two classes whose joins
  are different and whose cells are identical.** A rule keyed on the shape would be wrong about 124 of
  126, and silently — concatenation leaves no mark of having guessed, which is exactly the danger this
  leaf was opened to check.
  **What separates them is the document's own spelling, and it separates them perfectly.** Asking
  whether the document anywhere writes the CONCATENATION selects **2 of 126, and both are right**:
  eMMC `table_0221` `t PERIOD` → `tPERIOD` and `t TLH , t THL` → `tTLH`/`tTHL`, the second being the
  comma family and the split in one cell that this leaf was opened on. Every one of TileLink's 81 is
  refused, correctly, because TileLink never writes `aopcode` anywhere — its joining spelling is
  `a_opcode`, and `.0` found even that only as vector text inside three figures. **0 false positives
  against a counter-population of 81 that has the same shape and the opposite answer** is the
  strongest discrimination anything in this area has shown.
  **And it still does not ship, for `.0`'s reason rather than a new one.** The entire correct
  population is **one table in one document, and that document is legacy** — `jesd84_b50` cannot be
  rebuilt, so a rule would change **zero declarations in the current stratum** and the next document
  to exercise it would be its first test rather than its hundredth. `.0` refused a rule that was right
  about 3 of 120 and inert; this one is right about 2 of 126 and inert for the same reason. The
  measurement is the deliverable, and it is now reproducible.
  **Two smaller facts worth keeping.** The `MAX_LEAD_CHARACTERS = 2` bound is what makes every
  legitimate multi-token name shape survive untouched — `AWSIZE, ARSIZE`, `HSELx a`, `ARMPAM [10:0]`,
  `PADDR [31:0]`, `Duty Cycle`, `Clock source` all select **nothing** at either tier, verified
  directly against the reader's own tokenizer. And eMMC's remark column writes `C DEVICE` for
  `CDEVICE`, so the class is not confined to the `t` family — but a remark cell is outside the name
  column and cannot declare, so it is counted and not acted on.
  Prerequisite: none. Verification: `python3 scripts/measure_subscript_split_name_cells.py` over all
  78 persisted SourceIRs, read-only (no artifact written, rebuilt or mutated); every selection printed
  for adjudication; the survival of the legitimate shapes checked against `token_pairs` directly.
  Commit: `TEXT-LAYER-IDENTIFIER-SPLIT.2`

## Current Frontier

1. `TEXT-LAYER-IDENTIFIER-SPLIT.1` — read the three TileLink timing diagrams and decide whether a name
   recovered from a figure may ground a table row's identity. The identity question is the leaf, not the
   VLM call. **`.2` sharpened why it matters**: TileLink's 81 name cells are the counter-population that
   makes every shape-only join rule wrong, and the only thing that refuses them is the absence of an
   in-document spelling — which is exactly the evidence `.1` would go and fetch from the figures.
   It needs a live VLM provider over three images and TileLink cannot be rebuilt (no retained bundle),
   so the recovery has to be demonstrated on the persisted assets directly.

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

- `2026-09-14` — `.2`. `python3 scripts/measure_subscript_split_name_cells.py` over all 78 persisted
  SourceIRs / 573 boundary tables, read-only: name-column tier A **9 cells, 2 reachable**; tier B **126
  cells, 2 reachable**, of which **81 are TileLink and 0 are reachable**; 1,677 tier-B matches outside
  the name column across 38 documents reported as the false-positive surface. The two reachable cells
  are eMMC `table_0221` r0c0 and r1c0, both in a table that DECLARES, both legacy. Legitimate shapes
  (`AWSIZE, ARSIZE`, `HSELx a`, `ARMPAM [10:0]`, `PADDR [31:0]`, `Duty Cycle`, `Clock source`) select
  nothing at either tier, checked directly against `token_pairs`. No artifact written, rebuilt or
  mutated.
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
- `.2` — `TEXT-LAYER-IDENTIFIER-SPLIT.2`.

## Changelog

- `2026-09-14` — `.0` closed with **no rule**. The column signature selects 120 columns and the current
  stratum's ten are all description columns; the in-document join selects 3 of 120 and is right about
  all three, but all three are inert. TileLink's joining spelling is vector text inside three figures
  the ingest correctly stores as images — so the ingest is not at fault and the evidence is reachable
  only through the VLM path (`.1`). eMMC's `t PERIOD` is a subscript split with no separator (`.2`).
- `2026-09-14` — created from `PROSE-NAME-CELL-DECLARATION.5`'s census, which falsified that leaf's own
  premise: TileLink's rotation is not the whole-cell column score's doing under either score, and the
  cause is a name cell holding one identifier the text layer split in two.
