# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.viii`** (read the quote-wrapped literal; widen
  `parse_encoding_numeric_literal` ONLY — widening `looks_like_encoding_literal` moves `value_col`
  and therefore the members). **TIME-CRITICAL, decided by `.5.ix`:** `CORPUS-COVERAGE`'s next named
  refresh is `opencapi_3_0_transaction_layer_28jan2020`, which carries **129 of this leaf's 357
  values**; landing after it bakes them into a fresh chain and costs a second re-ingest.
  Then `.5.vi.a` (unblocked by `.5.vii.a`).
- **`.5.vii`/`.5.vii.a`/`.5.vii.b` CLOSED `2026-09-21`**: the `row_idx` fallback is GONE. Landing
  changed **54 statements in 4 docs, 0 added, 0 non-Enum**. [[minted-enum-values-are-mostly-row-indices]]
- **CHECK WHICH STRATUM A DEFECT LIVES IN BEFORE WRITING ITS FIX BAR** — a 10,189 bar was
  unverifiable by construction (99.5% legacy); corrected before implementing.
- **REBUILD-THEN-VALIDATE, PER STAGE, IN ORDER** repairs a stale proof seal. Restored chain-currency
  27/27 AND made `eval-extraction` run where it refused for `.5.vi.a`.
- **A CENSUS OVER THE PERSISTED CORPUS DESCRIBES ARTIFACTS, NOT THE READER** (`.5.vii.b`, rule from
  `CORPUS-CHAIN-CURRENCY.11`): the producer mints **15 enums / 79 members**; **530 / 21,459 (99.6%)
  are legacy**. That surface is a TIMING fact — it grows with each refresh — not a ceiling.
- **Replay the producer; do not read its output's SHAPE** — `.5.v`'s test saw 160 enums, the replay
  10,189. It under-counts: **578 slots (3.0%) are ambiguous**, false-positive rate **2 of 79**.
- **`CLAIM-VERIFICATION-ADOPTION.20` tranche 1: 4 regions re-derived, 2 STALE.** Population 167->163.
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT.** Both first-pass verdicts were confidently
  wrong, oppositely: `82->8` was over 33 REBUILDABLE docs; `6,570` counted fields reaching an emitted
  `.isf` (0 today). Next tranche: a chapter whose claims are NOT corpus-derived totals.
- **Execute a claim's trigger; never guess a key** (`bit_fields` -> 0; it is
  `register_records[].fields`). **Run the negative control.**
- **Claim-digest bill on every governed edit** — `repin_claim_regions.py` is TRANSACTIONAL; re-pin
  BEFORE registering new candidates or you create duplicates.
- **DERIVE a contract's JSON encoder every time** — `CONTRACT-ENCODING` gates it (it caught indent=1
  against a declared indent=2 this session).
- `cargo test -p specforge --lib` reports **0 passed, 474 filtered out** for an `ir/` change, exit 0.
  Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **After every push, watch the hosted run to a verdict**; a stalled run is not a pass.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- Corpus 27/27 current at all 4 stages. A slow gate is **measured**: `probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- **READ A TREE'S GOAL, NOT ITS TITLE, BEFORE CITING IT AS A CONSTRAINT** — `.5.ix` was opened on
  "RETAINED-BUNDLE-POPULATION-FROZEN froze the set"; that tree REMOVED the freeze (24 -> 27 retained),
  which reversed the decision. A refresh bakes in whatever gates exist when it runs, so `.5` gates
  must land AHEAD of `CORPUS-COVERAGE`'s refreshes, never after.
- Blockers: none.
