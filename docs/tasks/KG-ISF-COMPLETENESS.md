# KG-ISF-COMPLETENESS: the KG/IntentIR must be COMPLETE enough to lower faithfully to ISF

## Metadata

- Tree ID: `KG-ISF-COMPLETENESS`
- Status: `active`
- Roadmap lane: `R15`/`R16` (extraction quality / design-intent capture, ISF-fidelity lens)
- Created: `2026-06-16`
- Last updated: `2026-09-21`
- Parent context: owner directive (`2026-06-16`): *"Extracting for extracting is not the goal. In fine we
  need the IntentIR to be complete and contain all the necessary information to be lowered to ISF"*,
  *"the KG shall contain everything (actors, agent, relations, constraints) for all of these PDFs."*
- Sequencing (`2026-08-11`, ADR 0033): ISF is not the present blocker. Populate and measure
  source-grounded PDF content through canonical IntentIR first; take ISF/FSMGen work only when that
  content demonstrates a concrete expressiveness gap.

## Goal

Make the KG/IntentIR complete enough to lower FAITHFULLY to `.isf` — every real protocol agent, every
relation, every stated constraint, every behaviour carried or recorded as an explicit residual. *"ISF
ready"* has never meant *"the adapter renders without blocking"*: all 36 built documents already rendered
`.isf` with 0 blocking reasons on `2026-06-16`. The adapter only renders; the fidelity of the `.isf` is
the fidelity of the IntentIR feeding it. The per-document bar, the measured baseline, every per-leaf
contract, each enforced acceptance checklist, and the dated changelog live in the task-evidence parts;
this root carries the bounded current summary and the owner registry only.

## Current Program State

- **Agent identity** (`.1`, `.1c`) is closed on the AMBA/structured class and partially on dense prose:
  `.1c.i` landed the trailing preposition/auxiliary strip; `.1c.ii` stays deferred-with-trigger because a
  rare-but-real agent can also appear in exactly one relation, so a participation gate needs its own
  measurement before it may drop anything.
- **ISF lowering fidelity** (`.2a.i`-`.2a.vi`) is closed: width, direction, HDL-safe module names, the
  enum value-literal emit gate, the unconditional-rule-overlap conflict residual, and the rule-drive-value
  validity gate, reaching **70/70 renderable strict-clean**.
- **Relation completeness** (`.3`) is closed with 0 stale documents corpus-wide and a stage-staleness
  detector shipped as `CORPUS-COVERAGE.1`.
- **The enum surface** (`.5`) is the live programme. `.5.i`-`.5.iv.a` cut the generic-`TABLE` mega-enum
  (82 generic enums to 8, 422 enum records to 105) and gave the header its own naming path. `.5.v`-`.5.ix`
  then measured what those gates could not see: enum IDENTITY is mostly cross-table conflation, and enum
  VALUES were mostly the table's row index. `.5.vii.a` deleted that positional fallback, `.5.viii` taught
  the reader the quoting notation the documents declare, and `.5.vi.a` stopped the caption's numbering
  label from naming — and from mis-pointing — an encoding table.
- **Read every corpus-wide enum total as a fact about the ARTIFACTS, not the reader** — 51 of 78 chains
  cannot be replayed, and they hold 99.4% of the minted member surface. The current producer mints 25
  enums / 136 members (after `.5.xiii.a`). That is a statement about TIMING: the replayable stratum grows with each
  `CORPUS-COVERAGE` refresh, so a `.5` gate landed now is inherited free by every future refresh, and a
  gate NOT landed is a defect re-imported at full cost (`.5.ix`).
- Two standing rules this tree earned by being wrong first. **A count cannot verify a value**: `.5.viii`
  was handed a bar ("the quote-wrapped count must go to 0") that the correct repair and an 88%-wrong one
  satisfy identically. **A naming change is not subtractive**: `infer_encoding_column_indices` takes the
  enum name as an argument, so removing a name moves the value column — `.5.vi.a` removed one and gained
  40 statements. **And a third, from `.5.xi`: count the population the SEAM reads, not the population the
  corpus holds.** `.5.xi` was opened on 1,173 corpus cells, 158 of them replayable, and concluded the
  defect reached the current producer; the cells are real and the conclusion is false, because a cell is
  published only if it lands in the `value_col` of a table that MINTS. The seam census is 336 member
  values, **0 replayable**. **A fourth, from `.5.vi.a.2`: re-pinning a moved census needs a SEARCH, not
  an intention** — `.5.vi.a.1` stated the rule and still left four present-tense surfaces, this root
  among them, claiming the pre-`.5.vi.a` 15 enums / 79 members. **And a fifth, from `.5.xiii`: a
  CONTENT SCORE IS NOT AN ADJUDICATION.** Its shape test picks AxSIZE's `Label` column — `1 2 4 8 …
  128`, byte COUNTS — over the prose column, and `Enum AxSIZE 1 = 0.` is strictly worse than what ships
  today. A score cannot separate a member-name column from a quantity column. **And a sixth, from
  `.5.xiv`: AN ORACLE THAT CANNOT FAIL IS NOT VERIFICATION.** 0 of 156 `kg-bench` fixtures mention an
  enum and `eval-extraction` has no enum task, so `kg-bench 156/156` and `WIRE-BASED-100 = 1.000` are
  NECESSARY and NOT SUFFICIENT for any enum change — which is why every `.5` defect was found by a
  census rather than by a test. [[enum-member-values-have-no-oracle]]
- The open work is three measured defects at the same reader, none of them a new programme:
  `.5.xiii.a` (the member-name column, adjudicated per table), `.5.xii` (grouped `0b`/`0x` literals
  refused, 77 cells), `.5.x` (the header-sourced naming clause, 3 tables). Five leaves closed on
  `2026-09-22`: `.5.xi`/`.5.xi.a` read the unquoted radix from the table's own layout, correcting
  **198 of 336** minted member values; `.5.xiii` refuted the naming repair it was opened to specify;
  `.5.xiv`/`.5.xiv.a` withdrew **40 contradictory member values** and landed the repository's first
  enum-member-value fixture, which is the oracle the whole programme lacked.
  `.4` closed on `2026-06-17` as a read-only
  measurement of bars #5/#6, so behaviour/temporal lowering carries no open leaf in this tree today; the
  still-open umbrellas are `.1`, `.1c`, `.2`, `.5` and the deferred-with-trigger `.1c.ii`, `.2a`, `.2b`.
- **Hard gate, unchanged:** the wire docs (APB/AHB/AXI/SWD) stay at WIRE-BASED-100 through every change.
  Universal grammar only, no name lists (ADR 0006).

## Current Frontier

`.5.xiii.a.5.a` — statement text holds the document's characters, not Markdown's escapes.

`.5.xiii.a.3` closed on `2026-09-23` (MEASUREMENT): **186 of 358** constraint records sit under a list
lead-in or table caption, and in a hand-read sample of 40 the **16 restricting scopes were all lost**
(*"During reset …"* publishes `RACK must be LOW`). Both obvious repairs are refuted: copying the lead-in
fabricates conditions from the neutral 40%, and residualizing every scoped record costs gold-scored
constraints. `.a` records the carrier as provenance and measures any lowering gate on the golds.

Earlier the same day: `.5.xiii.a.2` (a number ends where its literal ends), `.5.xiii.a`/`.a.1` (members
named from the document's own column; `RRESP`/`ARTAGOP`/`AWTAGOP` recovered; `kg-bench` 158/158).

Verification pass `2026-09-23`: every numeric finding of `.5.xiii.a`-`.a.3` re-derives exactly; two
recorded claims did not hold and are corrected in the leaves (the EvidenceIR-field cost, and APB user
signals named as gold-scored). Four findings that had no owner now do: `.5.xiii.a.4`-`.a.7`; the
fifth, the typed kg-bench count, was a blind gate and is closed as `.5.xiii.a.8`.

`.5.xiii.a.7` closed: the enum instruments locate by attribution, which found the loose locator had
paired 68% of tables with enums they never minted — every `.5.xiii`/`.5.xiii.a` population figure is
corrected where it is stated.

`.5.xiii.a.5` closed as a measurement: DTI's mis-attributed records are legacy (the current producer
mints none), but the logic behind them is live — escaped `\_` hides every underscore name in prose
(Wishbone: 25 names, 629 mentions, 0 constraints), and a leading condition supplies the subject.

Eligible leaves, in the order they should be taken: `.5.xiii.a.5.a`, `.5.xiii.a.5.b`, `.5.xiii.a.5.c`,
`.5.xiii.a.3.a`, `.5.xiii.a.4`, `.5.xiii.a.6`, `.5.xii`, `.5.x`.
Every other open node is an umbrella or is deferred behind a stated trigger.

## Detailed task evidence

[Open the task-evidence index](kg-isf-completeness/INDEX.md) for every detailed task contract, acceptance
checklist, decision, verification entry, commit record, semantic evidence part, and the exact
pre-migration source capsule.

## Executable Owner Registry

These compact declarations preserve stable-path task-owner lookup. The task-evidence index is the primary
detail-routing authority, and the route catalog carries every leaf with its lifecycle and its detail part.

- ID: `KG-ISF-COMPLETENESS`
- ID: `KG-ISF-COMPLETENESS.1c`
- ID: `KG-ISF-COMPLETENESS.1c.i`
- ID: `KG-ISF-COMPLETENESS.1c.ii`
- ID: `KG-ISF-COMPLETENESS.0`
- ID: `KG-ISF-COMPLETENESS.1`
- ID: `KG-ISF-COMPLETENESS.1a`
- ID: `KG-ISF-COMPLETENESS.1b`
- ID: `KG-ISF-COMPLETENESS.1b.i`
- ID: `KG-ISF-COMPLETENESS.1b.ii`
- ID: `KG-ISF-COMPLETENESS.1b.iii`
- ID: `KG-ISF-COMPLETENESS.1b.iv`
- ID: `KG-ISF-COMPLETENESS.2`
- ID: `KG-ISF-COMPLETENESS.2a`
- ID: `KG-ISF-COMPLETENESS.2a.i`
- ID: `KG-ISF-COMPLETENESS.2a.ii`
- ID: `KG-ISF-COMPLETENESS.2a.iii`
- ID: `KG-ISF-COMPLETENESS.2a.iv`
- ID: `KG-ISF-COMPLETENESS.2a.v`
- ID: `KG-ISF-COMPLETENESS.2a.vi`
- ID: `KG-ISF-COMPLETENESS.2b`
- ID: `KG-ISF-COMPLETENESS.3`
- ID: `KG-ISF-COMPLETENESS.4`
- ID: `KG-ISF-COMPLETENESS.5`
- ID: `KG-ISF-COMPLETENESS.5.i`
- ID: `KG-ISF-COMPLETENESS.5.ii`
- ID: `KG-ISF-COMPLETENESS.5.iii`
- ID: `KG-ISF-COMPLETENESS.5.iv`
- ID: `KG-ISF-COMPLETENESS.5.iv.a`
- ID: `KG-ISF-COMPLETENESS.5.v`
- ID: `KG-ISF-COMPLETENESS.5.vi`
- ID: `KG-ISF-COMPLETENESS.5.vi.a`
- ID: `KG-ISF-COMPLETENESS.5.vi.a.1`
- ID: `KG-ISF-COMPLETENESS.5.vi.a.2`
- ID: `KG-ISF-COMPLETENESS.5.vii`
- ID: `KG-ISF-COMPLETENESS.5.vii.a`
- ID: `KG-ISF-COMPLETENESS.5.vii.b`
- ID: `KG-ISF-COMPLETENESS.5.viii`
- ID: `KG-ISF-COMPLETENESS.5.x`
- ID: `KG-ISF-COMPLETENESS.5.xi`
- ID: `KG-ISF-COMPLETENESS.5.xi.a`
- ID: `KG-ISF-COMPLETENESS.5.xii`
- ID: `KG-ISF-COMPLETENESS.5.xiii`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.1`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.2`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.3`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.3.a`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.4`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5.a`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5.b`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.5.c`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.6`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.7`
- ID: `KG-ISF-COMPLETENESS.5.xiii.a.8`
- ID: `KG-ISF-COMPLETENESS.5.xiv`
- ID: `KG-ISF-COMPLETENESS.5.xiv.a`
- ID: `KG-ISF-COMPLETENESS.5.ix`

## Changelog

The complete dated changelog is in the changelog part, which is sealed against the archived capsule. New
dated entries belong in the active part beside the leaf that earns them.
