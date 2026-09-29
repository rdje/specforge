# EXTRACTION-REPRESENTATION-GAPS: one ledger of what a specification states, what SpecForge extracts, and what `.isf` can say — each gap owned on its own side

## Metadata

- Tree ID: `EXTRACTION-REPRESENTATION-GAPS`
- Status: `active`
- Roadmap lane: the program objective (ADR 0033, amended by ADR 0053) and `R0` downstream contract hygiene
- Created: `2026-09-25`
- Last updated: `2026-09-29`
- Owner: repo-local workflow
- Decision: ADR 0053 (`docs/decisions/0053-extraction-and-isf-requests-advance-in-parallel.md`)

## Goal

Every piece of design intent a chip specification states is **extracted** by SpecForge into canonical IR and
is **representable** in `.isf`. Where either fails, the gap is named, attributed to the side that owns it, and
owned by an open leaf: a SpecForge extraction leaf, an FSMGen request (through `FSMGEN-REQUESTS`), or both.
The owner's directive (`2026-09-25`): *SpecForge does its part and FSMGen does its part.*

The ledger is `docs/gaps/`, routed from the book's `reference/representation-gaps.md`, so the owner can see every gap
and who holds it without reading code. Non-goals, acceptance criteria, every leaf contract, the censuses, the
decisions, the enforced acceptance checklists and the dated record live in the task-evidence parts; this root
carries the bounded current summary and the owner registry only.

## Current Program State

- **The ledger** is `docs/gaps/` (`.36`): SpecForge's items by road and stage, and FSMGen's 28-rung ladder, each
  with an owner the `OWNERSHIP-CITATIONS` gate holds open (`.1`). `.36.a` triages every open leaf into it.
- **The censuses.** `.2` measured IntentIR against the adapter; five chains now emit `.isf` (AXI, APB-E, AHB,
  Wishbone, and CoreSight TMC since `.7.f`). `.3` measures what each road states against the category contract on current code: `.3.a` made 31
  chains replayable, and the per-family census found 2,793 captured regions and 167 of 187 assumptions that reach
  no carrier (`docs/research/category-family-census.md`); `.3.b`-`.3.f` own what it found.
- **FSMGen requests** `.4`-`.6`, `.15`, `.16`, `.20` and the question `.19` were forwarded by the owner on
  `2026-09-25`; each closes when FSMGen answers and SpecForge integrates. `.21` lands with
  `FSMGEN-REFRESH-INTEGRATE-10`.
- **Lowering** leaves `.7`-`.14`, `.17`, `.18` and `.37` carry the extraction and lowering gaps the wire road
  found; `.22`-`.35` are closed.

## Current Frontier

`.3.b.iii` — a value, timer or constraint table the ingest left untyped is read by the reader its header names.

Then, in order: `.41.e` (a field-to-signal bit map), `.3.d`, `.3.g.iii` (multi-column encodings), `.3.c.iv` (per-bit
encodings), `.3.c.v` (requirement matrices), `.3.c.vi` (today's typed tables), `.3.c.viii` (a converter-shifted signal
table), `.3.f.ii` and `.3.f.iii` (single-label and unnamed grouped timing tables); `.3.c`, `.3.e`, `.3.e.i`, `.3.c.iii`,
`.39`, `.3.c.ii.a`, `.3.c.ii.b`, `.42`, `.3.c.ii`, `.3.c.vii`, `.41.c`, `.43.a`, `.3.b`, `.3.b.i` and `.3.b.ii` are done
(every typed table of `2026-09-26` has a reason or an owner; a symbol cites the table it was read from; a word in a
signal table's name column is not a signal; a field's value table fills the field's values; a condition marker that
opens a sentence opens that sentence's condition; the object before *and then* is no subject; an actor phrase ends at
its punctuation; a character reference's semicolon is no clause boundary; a signal named from one actor to another is
declared; a width operator the converter dropped stays a residual; a layout table sizes the register its heading names;
a descriptive consequent's signal is the one it describes; every untyped uncited table has a family, an owner or a
reader; an untyped signal table with a direction column declares its signals; every untyped uncited table is
non-contract or owned). After those: `.7.e.i` once `KG-ISF-COMPLETENESS.1` grounds the initiators; `.36.a`, `.37`,
`.17.a`, `.11`, `.9`, `.10`, `.12`, `.13`, `.14`, `.18`; `.10.g.iii` waits for `.4`, and `.41.d` for a legacy
register-map document's re-ingest.

New records of the `.3` family go to the `table-census-leaves` part (`program-and-census` and `census-leaves` are at
their size warning); every other new leaf and dated entry goes to the `clause-and-register-leaves` part (`recent-leaves`
reached its rollover). The five are the active parts.

## Detailed task evidence

[Open the task-evidence index](extraction-representation-gaps/INDEX.md) for every detailed task contract, census,
decision, acceptance checklist, commit record, semantic evidence part, and the exact pre-migration source capsule.

## Executable Owner Registry

These compact declarations preserve stable-path task-owner lookup. The task-evidence index is the primary
detail-routing authority, and the route catalog carries every leaf with its lifecycle and its detail part.

- ID: `EXTRACTION-REPRESENTATION-GAPS`
- ID: `EXTRACTION-REPRESENTATION-GAPS.0`
- ID: `EXTRACTION-REPRESENTATION-GAPS.1`
- ID: `EXTRACTION-REPRESENTATION-GAPS.2`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.ii.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.vi`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.vii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.viii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.e`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.e.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.h`
- ID: `EXTRACTION-REPRESENTATION-GAPS.4`
- ID: `EXTRACTION-REPRESENTATION-GAPS.5`
- ID: `EXTRACTION-REPRESENTATION-GAPS.6`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.c`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.d`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.f`
- ID: `EXTRACTION-REPRESENTATION-GAPS.8`
- ID: `EXTRACTION-REPRESENTATION-GAPS.9`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.c`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.d`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.e`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.f`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.h`
- ID: `EXTRACTION-REPRESENTATION-GAPS.11`
- ID: `EXTRACTION-REPRESENTATION-GAPS.11.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.12`
- ID: `EXTRACTION-REPRESENTATION-GAPS.13`
- ID: `EXTRACTION-REPRESENTATION-GAPS.14`
- ID: `EXTRACTION-REPRESENTATION-GAPS.15`
- ID: `EXTRACTION-REPRESENTATION-GAPS.16`
- ID: `EXTRACTION-REPRESENTATION-GAPS.17`
- ID: `EXTRACTION-REPRESENTATION-GAPS.17.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.18`
- ID: `EXTRACTION-REPRESENTATION-GAPS.19`
- ID: `EXTRACTION-REPRESENTATION-GAPS.20`
- ID: `EXTRACTION-REPRESENTATION-GAPS.21`
- ID: `EXTRACTION-REPRESENTATION-GAPS.22`
- ID: `EXTRACTION-REPRESENTATION-GAPS.23`
- ID: `EXTRACTION-REPRESENTATION-GAPS.24`
- ID: `EXTRACTION-REPRESENTATION-GAPS.24.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.24.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.25`
- ID: `EXTRACTION-REPRESENTATION-GAPS.25.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.25.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.25.c`
- ID: `EXTRACTION-REPRESENTATION-GAPS.26`
- ID: `EXTRACTION-REPRESENTATION-GAPS.26.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27.c`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27.d`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27.f`
- ID: `EXTRACTION-REPRESENTATION-GAPS.27.e`
- ID: `EXTRACTION-REPRESENTATION-GAPS.29`
- ID: `EXTRACTION-REPRESENTATION-GAPS.30`
- ID: `EXTRACTION-REPRESENTATION-GAPS.31`
- ID: `EXTRACTION-REPRESENTATION-GAPS.32`
- ID: `EXTRACTION-REPRESENTATION-GAPS.33`
- ID: `EXTRACTION-REPRESENTATION-GAPS.34`
- ID: `EXTRACTION-REPRESENTATION-GAPS.35`
- ID: `EXTRACTION-REPRESENTATION-GAPS.36`
- ID: `EXTRACTION-REPRESENTATION-GAPS.36.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.37`
- ID: `EXTRACTION-REPRESENTATION-GAPS.38`
- ID: `EXTRACTION-REPRESENTATION-GAPS.39`
- ID: `EXTRACTION-REPRESENTATION-GAPS.39.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.40`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41.b`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41.c`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41.d`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41.e`
- ID: `EXTRACTION-REPRESENTATION-GAPS.42`
- ID: `EXTRACTION-REPRESENTATION-GAPS.42.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.43`
- ID: `EXTRACTION-REPRESENTATION-GAPS.43.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.28`

## Changelog

The complete dated changelog up to the partition is in the `changelog` part, sealed against the archived capsule.
New dated entries belong in the active part beside the leaf that earns them.
