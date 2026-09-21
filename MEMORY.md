# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.vi.a`** (CODE — the caption-label identity gate; unblocked by
  `.5.vii.a`). The time-critical `.5.viii` is LANDED, so `CORPUS-COVERAGE`'s next refresh
  (`opencapi_3_0_transaction_layer_28jan2020`) will import the document's real codes, not 129 fakes.
- **`.5.viii` CLOSED `2026-09-21` — the fix it was HANDED was wrong.** `.5.vii` pinned *"a bare
  `'1110'` already parses as binary"*; it does not — the reader tries `parse::<u32>()` FIRST, so that
  arm is dead below 11 digits. Wrong on **314 of 357 (88%)**, worse than the defect on **57**.
  [[quoted-literal-notation-carries-the-radix]]
- **A COUNT CANNOT VERIFY A VALUE.** That leaf's handed-down bar ("quote-wrapped goes to 0, fabricated
  falls by 357") is passed identically by the correct repair and the 88%-wrong one. When a slice
  changes what a value IS, the oracle must compare VALUES. Second bar correction in this programme,
  after `.5.vii.a`'s: **check the stratum AND the dimension before implementing.**
- **RE-DERIVE A PINNED HAZARD; IT MAY BE TOO NARROW** — `.5.viii`'s named one caller; a second one
  MINTED. **And cross-check an instrument's mirror against the product** (7,151 cells → `.5.xii`).
- **A CENSUS OVER THE PERSISTED CORPUS DESCRIBES ARTIFACTS, NOT THE READER**: the producer mints
  **15 enums / 79 members**; **530 / 21,459 (99.6%) are legacy**. A TIMING fact, not a ceiling.
- **Replay the producer; do not read its output's SHAPE** — **578 slots (3.0%) ambiguous**, so the
  census totals are FLOORS.
- **REBUILD-THEN-VALIDATE, PER STAGE, IN ORDER** repairs a stale proof seal (restored 27/27 and made
  `eval-extraction` run where it refused).
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT.** Next `CLAIM-VERIFICATION-ADOPTION.20`
  tranche: a chapter whose claims are NOT corpus-derived totals. **Execute a claim's trigger; never
  guess a key. Run the negative control.**
- **Claim-digest bill on every governed edit** — `repin_claim_regions.py` is TRANSACTIONAL. A REFUSED
  region may mean the CLAIM'S VALUE moved, not just its line (`fact-card-catalog-count` 340 -> 341
  needed a new digest AND value, then `claims.jsonl` durability hashes re-derived).
- **DERIVE a contract's JSON encoder every time** — `CONTRACT-ENCODING` gates it.
- `cargo test -p specforge --lib` reports **0 passed, 474 filtered out** for an `ir/` change, exit 0.
  Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- A new production FUNCTION fails `PRODUCTION-GENERICITY`: re-derive `flow_census.json` under the
  `aggregate_change` idiom (roll baseline+delta forward, name the owner), never edit the number.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **After every push, watch the hosted run to a verdict**; a stalled run is not a pass.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- Corpus 27/27 current at all 4 stages. A slow gate is **measured**: `probe_exec_assessment_latency.sh`.
- **READ A TREE'S GOAL, NOT ITS TITLE, BEFORE CITING IT AS A CONSTRAINT** (`.5.ix`). A refresh bakes in
  whatever gates exist when it runs, so `.5` gates must land AHEAD of `CORPUS-COVERAGE`'s refreshes.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
