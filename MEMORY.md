# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.xiv.a`** — ship the missing RED case (a `kg_quality` fixture
  asserting an enum member VALUE) **together with** the subtractive hold-out that refuses to publish an
  enum carrying contradictory codes. **8 of 9 colliding enums cross into IntentIR `symbol_definitions`**
  (the README product boundary); none reaches `.isf` (AXI/AHB/APB are `lowering_status: blocked`).
- **Replay the producer; never read its output's SHAPE** — 578 slots ambiguous, totals are a FLOOR.
- **RE-PINNING A MOVED CENSUS NEEDS A SEARCH, NOT AN INTENTION** (`.5.vi.a.2`). Census now: **22 enums
  / 119 members** replayable, 552 / 21,578 total. Grep the old number; never recall it.
- **A PARTITION MUST BOUND THE GROWABLE FRACTION, NOT THE PART** (`KG-ISF-TASK-EVIDENCE-CONTAINMENT.2`);
  a FRONTIER leaf must also be declared in the ACTIVE part. [[bounded-task-root-accretes-in-its-frontier]]
- **A RADIX change cannot reach the enum-IDENTITY seams (parse-success is invariant); a NAME change is
  the opposite** — `infer_encoding_column_indices` takes the enum name, so a name decides the members.
- **AN ORACLE THAT CANNOT FAIL IS NOT VERIFICATION** (`.5.xiv`): **0 of 156 `kg-bench` fixtures mention
  an enum** and `eval-extraction` has no enum task, so `kg-bench 156/156` and `WIRE-BASED-100 = 1.000`
  are NECESSARY-NOT-SUFFICIENT for any enum change. Every `.5` defect was found by a census, never a
  test. [[enum-member-values-have-no-oracle]]
- **A CONTENT SCORE IS NOT AN ADJUDICATION** (`.5.xiii`): it picks AxSIZE's `Label` = `1 2 4 … 128`
  (byte COUNTS) over prose, and `Enum AxSIZE 1 = 0.` is worse than what ships. **One minting table in
  eight (654/5,456) is MULTI-KEY.** [[enum-member-name-column-needs-adjudication-not-a-score]]
- **KG-ISF task evidence is PARTITIONED**: new evidence goes to the one `active` part, NOT the root. Its
  CONTRACT is HAND-maintained — part metrics/sha256, `current_frontier.literal` and
  `root_required_literals` before `--write`, which derives only INDEX/routes/manifest.
- **RE-DERIVE A PINNED HAZARD; IT MAY BE TOO NARROW**; cross-check a mirror against the product;
  REBUILD-THEN-VALIDATE per stage, in order — the cascade ends at `adapt`, NOT `intent`.
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT** (next `CLAIM-VERIFICATION-ADOPTION.20`
  tranche: a chapter whose claims are NOT corpus-derived totals). Execute a trigger, never guess a key.
- **Claim-digest bill on every governed edit**: `repin_claim_regions.py --apply`, plus
  `--accept-value <file>` when a published VALUE moved. Never hand-roll it.
  [[claim-registries-pin-two-digest-families]]
- **DERIVE a contract's JSON encoder every time** (`CONTRACT-ENCODING`). `cargo test -p specforge --lib`
  says **0 passed, 474 filtered** for an `ir/` change — use `--workspace --lib --exclude
  specforge-production-graph` (COMMIT.md step 8).
- A new production FUNCTION fails `PRODUCTION-GENERICITY`: re-derive `flow_census.json` under the
  `aggregate_change` idiom (roll baseline+delta forward, name the owner), never edit the number.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- **READ A TREE'S GOAL, NOT ITS TITLE, BEFORE CITING IT AS A CONSTRAINT** (`.5.ix`). A refresh bakes in
  whatever gates exist when it runs, so `.5` gates must land AHEAD of `CORPUS-COVERAGE`'s refreshes.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
