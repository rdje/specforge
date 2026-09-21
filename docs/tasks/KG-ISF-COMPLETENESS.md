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
  cannot be replayed, and they hold 99.6% of the minted member surface. The current producer mints 15
  enums / 79 members. That is a statement about TIMING: the replayable stratum grows with each
  `CORPUS-COVERAGE` refresh, so a `.5` gate landed now is inherited free by every future refresh, and a
  gate NOT landed is a defect re-imported at full cost (`.5.ix`).
- Two standing rules this tree earned by being wrong first. **A count cannot verify a value**: `.5.viii`
  was handed a bar ("the quote-wrapped count must go to 0") that the correct repair and an 88%-wrong one
  satisfy identically. **A naming change is not subtractive**: `infer_encoding_column_indices` takes the
  enum name as an argument, so removing a name moves the value column — `.5.vi.a` removed one and gained
  40 statements.
- The open work is four measured defects at the same reader, none of them a new programme: `.5.x`
  (the header-sourced naming clause, 3 tables), `.5.xi` (decimal-before-binary for unquoted cells, 1,173
  cells), `.5.xii` (grouped `0b`/`0x` literals refused, 77 cells), `.5.xiii` (a prose `Description` column
  outranking the symbolic one, 262 tables upper bound). `.4` closed on `2026-06-17` as a read-only
  measurement of bars #5/#6, so behaviour/temporal lowering carries no open leaf in this tree today; the
  still-open umbrellas are `.1`, `.1c`, `.2`, `.5` and the deferred-with-trigger `.1c.ii`, `.2a`, `.2b`.
- **Hard gate, unchanged:** the wire docs (APB/AHB/AXI/SWD) stay at WIRE-BASED-100 through every change.
  Universal grammar only, no name lists (ADR 0006).

## Current Frontier

`.5.xi` — the decimal-before-binary ordering for UNQUOTED encoding cells.

It is the largest of the four open enum-reader defects and the only one that reaches the current
producer in quantity: a bare `0010` reads as ten because `parse::<u32>()` is tried before the all-`0`/`1`
arm. The leading-zero class is unambiguous and landable; the bare class needs a column-level
discriminator and its own measurement first, which is why the leaf is MEASUREMENT then CODE.

Eligible leaves, in the order they should be taken: `.5.xi`, `.5.xiii`, `.5.xii`, `.5.x` — the two
measurement-first leaves before the two whose populations they may re-scope. Every other open node
is an umbrella or is deferred behind a stated trigger.

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
- ID: `KG-ISF-COMPLETENESS.5.vii`
- ID: `KG-ISF-COMPLETENESS.5.vii.a`
- ID: `KG-ISF-COMPLETENESS.5.vii.b`
- ID: `KG-ISF-COMPLETENESS.5.viii`
- ID: `KG-ISF-COMPLETENESS.5.x`
- ID: `KG-ISF-COMPLETENESS.5.xi`
- ID: `KG-ISF-COMPLETENESS.5.xii`
- ID: `KG-ISF-COMPLETENESS.5.xiii`
- ID: `KG-ISF-COMPLETENESS.5.ix`

## Changelog

The complete dated changelog is in the changelog part, which is sealed against the archived capsule. New
dated entries belong in the active part beside the leaf that earns them.
