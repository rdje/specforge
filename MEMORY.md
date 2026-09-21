# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight**.
- Next action: **`KG-ISF-COMPLETENESS.5.vi.a`** — its blocker is GONE. `.5.vii.a` landed, so the enums
  that rule adds can no longer carry fabricated values, and the rebuild-then-validate protocol below
  is the gold-refresh step its node called "the remaining work". `.5.viii` (quoted literal, 357 values,
  3 OpenCAPI docs) is independent but **wholly legacy — 0 measurable effect today**.
- **`.5.vii`/`.5.vii.a`/`.5.vii.b` CLOSED `2026-09-21`**: the `row_idx` fallback is GONE. Landing
  changed **54 statements in 4 docs, 0 added, 0 non-Enum**. [[minted-enum-values-are-mostly-row-indices]]
- **CHECK WHICH STRATUM A DEFECT LIVES IN BEFORE WRITING ITS FIX BAR** — the committed acceptance
  said 10,189; unverifiable by construction (99.5% legacy), corrected before implementing.
- **REBUILD-THEN-VALIDATE, PER STAGE, IN ORDER** repairs a stale proof seal (out of order ->
  `cumulative proof ledger does not retain the exact verified upstream prefix`). Restored
  chain-currency 27/27 AND made `eval-extraction` run where it refused for `.5.vi.a`.
- **A CENSUS OVER THE PERSISTED CORPUS DESCRIBES ARTIFACTS, NOT THE READER** (`.5.vii.b`, rule from
  `CORPUS-CHAIN-CURRENCY.11`): the current producer mints **15 enums / 79 members**; the other
  **530 / 21,459 (99.6%) are legacy** from binaries that no longer exist. `.5.v`/`.5.vi`/`.5.vii`
  headlines all have that denominator. The census now prints the split above every total.
- **Replay the producer; do not read its output's SHAPE** — `.5.v`'s `0..n-1` test saw 160 enums, the
  replay 10,189. It under-counts too: **578 slots (3.0%) have >=2 candidate tables disagreeing**, and
  the post-landing control puts its false-positive rate at **2 of 79 (2.5%)**. Totals are FLOORS.
- **`CLAIM-VERIFICATION-ADOPTION.20` tranche 1: 4 regions re-derived in `isf-adapter.md`, 2 STALE —
  rate 2 in 4.** Population 167 -> 163; chapter 24 -> 20.
- **RECOVER A CLAIM'S DENOMINATOR BEFORE COMPARING TO IT.** Both first-pass verdicts were confidently
  wrong, oppositely: `82->8` was over 33 REBUILDABLE docs (0 today of 27); `6,570` counted fields
  reaching an emitted `.isf` (0 today), not the 12,602 in `IntentIR`. Next tranche: a chapter whose
  claims are NOT corpus-derived totals.
- **Execute a claim's trigger; never guess a key** (`bit_fields` returned 0; it is
  `register_records[].fields`). **Run the negative control.**
- **Every governed edit pays a claim-digest bill** — `repin_claim_regions.py` is TRANSACTIONAL;
  hand-resolve refusals, and re-pin BEFORE registering new candidates or you create duplicates.
- **DERIVE a contract's JSON encoder every time** — `CONTRACT-ENCODING` gates it (it caught indent=1
  against a declared indent=2 this session).
- `cargo test -p specforge --lib` reports **0 passed, 474 filtered out** for an `ir/` change, exit 0.
  Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- **After every push, watch the hosted run to a verdict**; a stalled run is not a pass.
- **A parked leaf keeps its blocker until re-derived. Do not restate a handed-down number.**
- Corpus 27/27 current at all 4 stages. A slow gate is **measured**: `probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. **`.5.ix` is OPEN and awaits the director**: the `.5` programme's verification
  surface is 15 enums; continuing / pausing / re-ingesting per-leaf are all defensible. Not blocking.
