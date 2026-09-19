# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`BOUNDED-DECISION-PROVIDER`** — created `2026-09-19` on the director's greenlight to
  evaluate TypeSafe's **Jev** (`jev-1.13.0`, a System One decision model) for bounded use. Verdict: **yes,
  narrowly** — it may only RANK among candidate spans the deterministic extractors already found.
- Next action: **`BOUNDED-DECISION-PROVIDER.1`** — the frozen per-row baseline for
  `SIGNAL-DECLARATION-ROW-DROP` and `INVARIANT-SHAPE-ADMISSION`, plus the pre-registered margin. Then
  `.1a` builds arm B. **Both are zero-egress**, and both are worth doing even if Jev is rejected.
- **Director's bar (`2026-09-19`): integration must PROVE it brings something SpecForge lacks today.**
  Encoded as a three-arm comparison — A status quo, **B best local alternative**, C Jev — and C must beat
  **both** by a margin fixed before any model runs. Beating A alone means shipping B instead. Four
  disqualifiers stay fatal (identity dependence, numeric reading, no offline replay, egress refused).
  The tree owns the detail; do not re-derive it.
- **`.2` is the director's call and is a ROADMAP AMENDMENT**, not a preference: `ROADMAP.md:37` mandates
  bounded *local* generators and Jev is API-only. **No network call before `.2` closes.**
- The corpus is 27/27 current (`check_chain_currency.sh`), `RETAINED-BUNDLE-POPULATION-FROZEN` closed.
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` is unblocked; `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a provider.
