# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`BOUNDED-DECISION-PROVIDER`** — evaluating TypeSafe's **Jev** (`jev-1.13.0`) for bounded
  use. Verdict: **yes, narrowly** — it may only RANK among candidate spans the deterministic extractors
  already found. **`.1` closed `2026-09-19`.**
- Next action: **`BOUNDED-DECISION-PROVIDER.1a`** — build arm B on `.1`'s frozen set. B1 (the repaired
  rule) now has a sized target: `.1` names a deterministic repair for **all 9** `caption_admission`
  errors. Zero egress; `.3` (the contract, design-only) is the other eligible leaf.
- **`.1`'s result, and it is evidence AGAINST arm C before arm C runs.** Frozen set
  `docs/research/bounded-decision-adjudication.jsonl` (sha `d3c5898f…`, 1,257 labelled rows), scored by
  `scripts/build_bounded_decision_baseline.py`. Arm A: `declaration_row` macro-F1 **0.90715**, precision
  **1.0000**, 22 errors; `caption_admission` macro-F1 **0.65014**, 9 errors. The decision is a **table
  property in 101/101 tables**, and **0 of 31** disagreements are genuine ambiguity. The bar is
  pre-registered as **C1–C4**; `.6` is held to it. Do not re-derive — read the tree and
  `docs/research/bounded-decision-baseline.md`.
- **Director's bar (`2026-09-19`): integration must PROVE new capability.** Three-arm comparison — A
  status quo, **B best local alternative**, C Jev — and C must beat **both** by the pre-registered margin.
  Four disqualifiers stay fatal (identity dependence, numeric reading, no offline replay, egress refused).
- **`.2` is a ROADMAP AMENDMENT**, not a preference: `ROADMAP.md:37` mandates bounded *local* generators
  and Jev is API-only. **No network call before `.2` closes.**
- **BLOCKED ON PROCUREMENT (director, `2026-09-19`): `TYPESAFE_API_KEY` must be bought before ANY Jev
  test.** Hosted-only, bearer auth, keys from `console.typesafe.ai/keys`. **The block is arm C only
  (`.4`, `.5`).** `.1a` and `.3` are keyless and proceed now. Key is env-only; `.gitignore` closed
  against `/.env*`.
- The corpus is 27/27 current (`check_chain_currency.sh`); `RETAINED-BUNDLE-POPULATION-FROZEN` closed.
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: arm C on the key (above); nothing else. `EXTRACTION-GAP-FIX.5b` is unblocked;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a provider; `SIGNAL-DECLARATION-ROW-DROP.2j` opened by `.1`.
