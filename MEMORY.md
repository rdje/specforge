# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-TASK-EVIDENCE-CONTAINMENT.1`** — author
  `doctrine/live_document_size/kg_isf_completeness_task_evidence.json` + a bounded root template and run
  `check_active_task_evidence.pl --migrate` (atomic, rolls back). **It BLOCKS every further KG-ISF slice:**
  `docs/tasks/KG-ISF-COMPLETENESS.md` is at **260,478 of 278,528 bytes (93.5%)** with ~18KB left and the
  last two slices cost 12.3KB and 13.1KB. Then `KG-ISF-COMPLETENESS.5.xi`.
- **`.5.viii` + `.5.vi.a` CLOSED `2026-09-21`; re-derivation REVERSED what both were handed** — a
  prescribed fix wrong on 314/357, and a trial-revert blocker `.5.vii.a` had already removed.
  [[quoted-literal-notation-carries-the-radix]] [[caption-label-is-not-table-identity]]
- **A COUNT CANNOT VERIFY A VALUE** — `.5.viii`'s handed bar ("quote-wrapped goes to 0") passes the
  correct repair and the 88%-wrong one identically. **Check the stratum AND the dimension of a bar
  before implementing.**
- **A NAME GATE CAN ADD MEMBERS.** `infer_encoding_column_indices` takes the enum NAME as an argument,
  so the name decides `value_col`, hence `name_col`, hence the members. `.5.vi.a` removed a name and
  gained 40 statements. **Never reason about a naming change as subtractive — replay it.**
- **RE-DERIVE A PINNED HAZARD; IT MAY BE TOO NARROW** (`.5.viii`'s named one caller; a second MINTED),
  and **cross-check an instrument's mirror against the product** (7,151 cells → `.5.xii`).
- **REBUILD-THEN-VALIDATE, PER STAGE, IN ORDER — AND THE CASCADE ENDS AT `adapt`, NOT `intent`** (chain
  currency failed on `isf-adapter` until `adapt --target isf` ran).
- **A CENSUS OVER THE PERSISTED CORPUS DESCRIBES ARTIFACTS, NOT THE READER**: the producer mints
  **15 enums / 79 members**; **530 / 21,459 (99.6%) legacy**. TIMING, not a ceiling.
- **Replay the producer; never read its output's SHAPE** — 578 slots (3.0%) ambiguous, totals FLOOR.
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT.** Next `CLAIM-VERIFICATION-ADOPTION.20`
  tranche: a chapter whose claims are NOT corpus-derived totals. **Execute a claim's trigger, never
  guess a key; run the negative control.**
- **Claim-digest bill on every governed edit** — `repin_claim_regions.py` is TRANSACTIONAL. A REFUSED
  region may mean the CLAIM'S VALUE moved, not just its line; then `claims.jsonl` durability hashes
  need re-deriving, iteratively, to a fixpoint.
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
