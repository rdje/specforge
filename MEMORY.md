# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`INVARIANT-SHAPE-ADMISSION`** — `LIVE-DOCUMENT-PRESSURE-HEADROOM.36b`/`.36c` closed
  `2026-09-19`: the claim registry's capacity is derived and coherent (`21 x 12,288 = 258,048`),
  13 of 21 records used, and the consumed single-use authority is retired. The latent stop that
  blocked registering a full claim record is GONE, and claim registration is unblocked.
- **Do not restate a handed-down sizing — re-derive it.** `.36a` sized `.36b` at an 8,192 per-record
  ceiling; two commits later the registry held a 10,223-byte record, so applying it would have
  refused a committed `verified` record. Same rule for every number this pointer carries.
- Next action: **`INVARIANT-SHAPE-ADMISSION.6b`** — the production change to `is_invariant_like`,
  now unblocked. **Its cost is the cascade, not the rule**: Rust edit + acceptance checklist, cargo
  oracles, the chain rebuilt for every document whose artifacts move (`.1` warns that is most of
  them), wire golds re-scored not assumed, book updated. Size the recall delta and attribute it
  before the cascade (ADR 0025). Expect the published constraint count to move on most documents.
- **The caption census, do not re-derive it** — `docs/research/caption-admission-repair-census.md`.
  Over 78 documents / 261,508 statements: **71 removals** (15 title, 56 cross-reference) and **176
  additions** (168 `is/are not permitted`, 8 `no … is/are allowed`; 38 serialized table rows, 138
  prose). The precision half costs **no requirement**.
- `BOUNDED-DECISION-PROVIDER` is **DECIDED: provider REJECTED** (ADR 0051), keyless and zero-egress.
  Do not buy `TYPESAFE_API_KEY` for it. `.3` (the bounded-use contract) stays open and worth writing
  as the standard the next provider is measured against; `.1a.2`/`.2`/`.4`/`.5` are conditional.
- `SIGNAL-DECLARATION-ROW-DROP.2j`: the direction-abbreviation census is **done and REFUSES the
  rule** — 837 rows write the full word, exactly 1 usable cell writes an abbreviation, inside the
  garbled ADIv6 `table_0108`. What remains is sizing that table's column garble (9 of 22 lost rows)
  and routing it to the ingest tree.
- `.36d` owns the claim registry's lifecycle and triggers at the 80% record band (**17 of 21**): the
  derivation bought 9 records and then spends the class portable envelope, and nothing returns a slot.
- The corpus is 27/27 current (`check_chain_currency.sh`).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
