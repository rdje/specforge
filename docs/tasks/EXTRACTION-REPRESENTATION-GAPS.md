# EXTRACTION-REPRESENTATION-GAPS: one ledger of what a specification states, what SpecForge extracts, and what `.isf` can say — each gap owned on its own side

## Metadata

- Tree ID: `EXTRACTION-REPRESENTATION-GAPS`
- Status: `active`
- Roadmap lane: the program objective (ADR 0033, amended by ADR 0053) and `R0` downstream contract hygiene
- Created: `2026-09-25`
- Last updated: `2026-10-19`
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

`.54.a.ii` — the corpus memory's refresh runs as ADR 0036 declares it, under `.54.a.i`'s rule.

It re-pins an equivalent store and re-runs the 24-document behavioral qualification. Then, in order: `.46.c` waits for
a FSMGen pin carrying `sf-isf-byte-group-reduction` (`FSMGEN-REQUESTS.12`), `.3.g.iii` (multi-column encodings),
`.3.c.iv` (per-bit encodings), `.3.c.v` (requirement matrices), `.3.c.vi` (today's typed tables), `.3.c.viii` (a
converter-shifted signal table), `.3.f.ii` and `.3.f.iii` (single-label and unnamed grouped timing tables), `.44`
(Symbol-font glyphs, a re-seal of every chain). After those: `.7.e.i` once `KG-ISF-COMPLETENESS.1` grounds the
initiators; `.36.a`, `.37`, `.17.a`, `.11`, `.9`, `.10`, `.12`, `.13`, `.14`, `.18`; `.10.g.iii` waits for `.4`, and
`.41.d` for a legacy register-map document's re-ingest. Closed leaves are read from the route catalog's lifecycles,
not restated here; the list this paragraph carried until `2026-10-19` is in the `register-binding-leaves` part.

Every new leaf and dated entry goes to the `packet-and-memory-leaves` part. Since `2026-10-21` (`.56.a`) a part whose
every leaf is closed is archived, byte-exact, under `docs/archive/tasks/extraction-representation-gaps/` and takes no
writes: `table-census-leaves`, `hint-and-notation-leaves`, `unread-intent-leaves`, `default-and-assumption-leaves`,
`environment-assumption-leaves`, both `acceptance-checklists` halves and the `changelog`. A sealed leaf is amended by
a new record in a live part. Of the nine live parts, `program-and-census`, `census-leaves`, `recent-leaves`,
`clause-and-register-leaves` and `register-binding-leaves` are at or past their size warning, and the three legacy
parts hold records for their own open leaves only.

## Detailed task evidence

[Open the task-evidence index](extraction-representation-gaps/INDEX.md) for every detailed task contract, census,
decision, acceptance checklist, commit record, semantic evidence part, and the exact pre-migration source capsule.

## Executable Owner Registry

These compact declarations preserve stable-path task-owner lookup for every leaf that is not closed, and for
a closed leaf whose part records no status of its own. A closed leaf leaves this list: the route catalog
carries every leaf with its lifecycle and its detail part (ADR 0057), and the task-evidence index is the
primary detail-routing authority.

- ID: `EXTRACTION-REPRESENTATION-GAPS`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.v`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.vi`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.viii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.4`
- ID: `EXTRACTION-REPRESENTATION-GAPS.5`
- ID: `EXTRACTION-REPRESENTATION-GAPS.6`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7`
- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e.i`
- ID: `EXTRACTION-REPRESENTATION-GAPS.9`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g`
- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.iii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.11`
- ID: `EXTRACTION-REPRESENTATION-GAPS.12`
- ID: `EXTRACTION-REPRESENTATION-GAPS.13`
- ID: `EXTRACTION-REPRESENTATION-GAPS.14`
- ID: `EXTRACTION-REPRESENTATION-GAPS.15`
- ID: `EXTRACTION-REPRESENTATION-GAPS.16`
- ID: `EXTRACTION-REPRESENTATION-GAPS.17.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.18`
- ID: `EXTRACTION-REPRESENTATION-GAPS.19`
- ID: `EXTRACTION-REPRESENTATION-GAPS.20`
- ID: `EXTRACTION-REPRESENTATION-GAPS.21`
- ID: `EXTRACTION-REPRESENTATION-GAPS.36.a`
- ID: `EXTRACTION-REPRESENTATION-GAPS.37`
- ID: `EXTRACTION-REPRESENTATION-GAPS.41.d`
- ID: `EXTRACTION-REPRESENTATION-GAPS.44`
- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a.ii`
- ID: `EXTRACTION-REPRESENTATION-GAPS.46.c`

## Changelog

The complete dated changelog up to the partition is in the `changelog` part, sealed against the archived capsule.
New dated entries belong in the active part beside the leaf that earns them.
