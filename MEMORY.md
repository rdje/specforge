# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`BOUNDED-DECISION-PROVIDER`** — evaluating TypeSafe's **Jev** for bounded use.
  **`.1` and `.1a.1` closed `2026-09-19`, and together they answer the tree.**
- Next action: **`BOUNDED-DECISION-PROVIDER.6`** — record the decision, which is now reachable ON
  MEASUREMENT: under the pre-registered bar arm C cannot clear either decision, so `.6` writes a
  REJECTION and states the only two things that would change it (run arm C anyway; revisit the
  `0.05` margin) — both the director's. Needs a decision record, `ROADMAP.md` and the mdBook.
- **The result, do not re-derive it** — read `docs/research/bounded-decision-arm-b.md` and the tree.
  Arm A / arm B1 on `.1`'s frozen 1,257-row set: `caption_admission` **0.65014 → 0.95413** (9 of 9
  errors corrected, no new false positive); `declaration_row` 0.90715 → 0.91077. C1 therefore needs
  arm C at **≥ 1.00413** on captions — impossible — and 13 of 21 rows on declarations whose evidence
  is not in the row and whose recovery C3 forbids. Zero egress, no key, no model run.
- **The key is no longer a prerequisite for deciding.** Buying `TYPESAFE_API_KEY` is now a choice to
  run arm C despite the bar, not a block on `.6`. `.4`/`.5` stay blocked on it; `.2`'s roadmap
  amendment is likely moot; `.3` (the contract) is still worth writing as the standard the next
  provider is measured against.
- Shipping B1 is NOT done and is routed, not implied: R1–R3 to `INVARIANT-SHAPE-ADMISSION`, R4 to
  `SIGNAL-DECLARATION-ROW-DROP.2j`, each needing the corpus-wide adjudication over all 78 documents.
- **Latent stop:** `claims.jsonl` is at its byte ceiling with no archive path, owned by
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.36`; register no new full claim record before it is resolved.
- The corpus is 27/27 current (`check_chain_currency.sh`).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none for `.6`. `EXTRACTION-GAP-FIX.5b` is unblocked; `EXTRACTION-QUALITY-GAUGE.3j.4.a`
  wants a provider; `BOUNDED-DECISION-PROVIDER.1a.2` (arm B2) is open and not decision-relevant.
