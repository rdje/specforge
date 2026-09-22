# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.xi.a`** — land `.5.xi`'s notational-witness rule at the member-VALUE
  seam. Adjudicated: **198 of 336 legacy member values corrected, 0 replayable moved**, so the oracle is a
  per-VALUE diff and a byte-identical replay is the PREDICTED result, not a no-op.
- **COUNT THE POPULATION THE SEAM READS, NOT THE ONE THE CORPUS HOLDS** (`.5.xi`, `2026-09-22`). The leaf
  was opened on 1,173 corpus cells / 158 replayable and concluded it reaches the current producer. Counts
  re-derive; conclusion FALSE — a cell publishes only from the `value_col` of a table that MINTS. Seam
  census **336 slots, 0 replayable**. Replay the producer; never read its output's SHAPE — 578 slots
  (3.0%) are ambiguous, so its totals are a FLOOR. [[decimal-before-binary-needs-a-notational-witness]]
- **RE-PINNING A MOVED CENSUS NEEDS A SEARCH, NOT AN INTENTION** (`.5.vi.a.2`). `.5.vi.a.1` stated the
  re-pin rule and still left **four** present-tense surfaces on the pre-`.5.vi.a` figure — the tree root,
  two KM cards, one research table. Re-derived: **22 enums / 119 members** replayable, 530 / 21,459 legacy,
  552 / 21,578 total, 99.45% legacy. **Grep the old number across every live surface, never recall it.**
- **A PARTITION MUST BOUND THE GROWABLE FRACTION, NOT THE PART** (`KG-ISF-TASK-EVIDENCE-CONTAINMENT.2`):
  `enum-surface-recent-leaves.md` hit 89.5% of its line target in ONE slice because 567 of its 687 lines
  were sealed. Split there — active part now **118 lines (15.4%)**, same write destination.
- **A RADIX change cannot reach the enum-IDENTITY seams and that is provable**: parse-success is invariant
  under the reorder (0 divergences over 71,001 distinct corpus cells). A NAME change is the opposite —
  `infer_encoding_column_indices` takes the enum name, so a name decides `value_col`, hence the members.
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
