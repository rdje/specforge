# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.xiv`** — **9 replayable enums publish one code on two members,
  all in the AMBA wire golds** (`HPROT` 6/8, `AWSNOOP` 5, `BTAGMATCH` 5, `HSIZE` 4, `ARSNOOP` 3).
  `value_col` picks ONE BIT of a split field (`HSIZE[2]|HSIZE[1]|HSIZE[0]|…`). The class `.5.vii.a`
  could not reach: it dropped cells that do not parse, and these DO parse.
- **COUNT THE POPULATION THE SEAM READS, NOT THE ONE THE CORPUS HOLDS** (`.5.xi`): 1,173 corpus cells
  but **336 seam slots, 0 replayable**. Replay the producer; never read its output's SHAPE — 578 slots
  (3.0%) are ambiguous, so its totals are a FLOOR. [[decimal-before-binary-needs-a-notational-witness]]
- **RE-PINNING A MOVED CENSUS NEEDS A SEARCH, NOT AN INTENTION** (`.5.vi.a.2`): `.5.vi.a.1` stated the
  rule and still left four present-tense surfaces stale. Current: **22 enums / 119 members** replayable,
  552 / 21,578 total. **Grep the old number across every live surface, never recall it.**
- **A PARTITION MUST BOUND THE GROWABLE FRACTION, NOT THE PART** (`KG-ISF-TASK-EVIDENCE-CONTAINMENT.2`);
  a FRONTIER leaf must also be declared in the ACTIVE part. [[bounded-task-root-accretes-in-its-frontier]]
- **A RADIX change cannot reach the enum-IDENTITY seams (parse-success is invariant); a NAME change is
  the opposite** — `infer_encoding_column_indices` takes the enum name, so a name decides the members.
- **A CONTENT SCORE IS NOT AN ADJUDICATION** (`.5.xiii`): it picks AxSIZE's `Label` = `1 2 4 … 128`
  (byte COUNTS) over the prose column, and `Enum AxSIZE 1 = 0.` is worse than what ships. Also **one
  minting table in eight (654/5,456) is MULTI-KEY** and cannot be a flat enum at all.
  [[enum-member-name-column-needs-adjudication-not-a-score]]
- **KG-ISF task evidence is PARTITIONED**: new evidence goes to the one `active` part above, NOT the root.
  Its CONTRACT (`doctrine/live_document_size/kg_isf_completeness_task_evidence.json`) is HAND-maintained —
  part metrics/sha256, `current_frontier.literal` and `root_required_literals` must be edited BEFORE
  `check_active_task_evidence.pl --write`, which derives only INDEX/routes/manifest.
- **RE-DERIVE A PINNED HAZARD; IT MAY BE TOO NARROW**; cross-check a mirror against the product; and
  REBUILD-THEN-VALIDATE, PER STAGE, IN ORDER — the cascade ends at `adapt`, NOT `intent`.
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT** (next `CLAIM-VERIFICATION-ADOPTION.20`
  tranche: a chapter whose claims are NOT corpus-derived totals). Execute a trigger, never guess a key.
- **Claim-digest bill on every governed edit** — `repin_claim_regions.py --apply` covers BOTH families
  (844 pins, one pass; `.21`); never hand-roll it. A REFUSED region may mean the VALUE moved.
- **DERIVE a contract's JSON encoder every time** (`CONTRACT-ENCODING` gates it). `cargo test -p
  specforge --lib` says **0 passed, 474 filtered** for an `ir/` change, exit 0 — use
  `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- A new production FUNCTION fails `PRODUCTION-GENERICITY`: re-derive `flow_census.json` under the
  `aggregate_change` idiom (roll baseline+delta forward, name the owner), never edit the number.
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- **READ A TREE'S GOAL, NOT ITS TITLE, BEFORE CITING IT AS A CONSTRAINT** (`.5.ix`). A refresh bakes in
  whatever gates exist when it runs, so `.5` gates must land AHEAD of `CORPUS-COVERAGE`'s refreshes.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
