# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.xi`** (decimal-before-binary for UNQUOTED cells — 1,173 cells,
  158 replayable; leading-zero class unambiguous, bare class needs a column discriminator first).
- **KG-ISF task evidence is now PARTITIONED** (`KG-ISF-TASK-EVIDENCE-CONTAINMENT`, closed `2026-09-21`):
  the root is 7,423 bytes and **new KG-ISF evidence goes to
  `docs/tasks/kg-isf-completeness/enum-surface-recent-leaves.md`**, the one `active` part — NOT the root.
  Migration lesson: `lifecycle` is `closed` only for `done`/`superseded`; `active`/`deferred` are OPEN.
- **`.5.viii` + `.5.vi.a` CLOSED `2026-09-21`; re-derivation REVERSED what both were handed** — a
  prescribed fix wrong on 314/357, and a trial-revert blocker `.5.vii.a` had already removed.
  [[quoted-literal-notation-carries-the-radix]] [[caption-label-is-not-table-identity]]
- **A COUNT CANNOT VERIFY A VALUE** — `.5.viii`'s handed bar ("quote-wrapped goes to 0") passes the
  correct repair and the 88%-wrong one identically. **Check the stratum AND the dimension of a bar
  before implementing.**
- **A NAME GATE CAN ADD MEMBERS** — `infer_encoding_column_indices` takes the enum NAME as an argument,
  so a name decides `value_col`, hence the members. Never reason a naming change subtractive; replay it.
- **A CHANGE THAT MOVES A CENSUS MUST RE-PIN EVERY SURFACE THAT PUBLISHES IT.** `.5.vi.a` passed all
  its own oracles and still falsified the book (15/79 vs 22/119), a KM `reverify` (4 of 8 values) and a
  research table (450 vs 455). A slice's oracles ask *did I break the product*, never *did I falsify
  someone else's number*. Fixed by `.5.vi.a.1`. [[caption-label-is-not-table-identity]]
- **RE-DERIVE A PINNED HAZARD; IT MAY BE TOO NARROW**, and cross-check a mirror against the product.
- **REBUILD-THEN-VALIDATE, PER STAGE, IN ORDER — THE CASCADE ENDS AT `adapt`, NOT `intent`.**
- **A CENSUS OVER THE PERSISTED CORPUS DESCRIBES ARTIFACTS, NOT THE READER**: the producer mints
  **22 enums / 119 members**; **530 / 21,459 legacy**. TIMING, not a ceiling.
- **Replay the producer; never read its output's SHAPE** — 578 slots (3.0%) ambiguous, totals FLOOR.
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT** (next `CLAIM-VERIFICATION-ADOPTION.20`
  tranche: a chapter whose claims are NOT corpus-derived totals). Execute a trigger, never guess a key.
- **Claim-digest bill on every governed edit** — `repin_claim_regions.py` is TRANSACTIONAL; a REFUSED
  region may mean the VALUE moved, then `claims.jsonl` hashes re-derive iteratively to a fixpoint.
- **DERIVE a contract's JSON encoder every time** (`CONTRACT-ENCODING` gates it). `cargo test -p
  specforge --lib` says **0 passed, 474 filtered** for an `ir/` change, exit 0 — use
  `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- A new production FUNCTION fails `PRODUCTION-GENERICITY`: re-derive `flow_census.json` under the
  `aggregate_change` idiom (roll baseline+delta forward, name the owner), never edit the number.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **After every push, watch the hosted run to a verdict**; a stalled run is not a pass.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- **READ A TREE'S GOAL, NOT ITS TITLE, BEFORE CITING IT AS A CONSTRAINT** (`.5.ix`). A refresh bakes in
  whatever gates exist when it runs, so `.5` gates must land AHEAD of `CORPUS-COVERAGE`'s refreshes.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
