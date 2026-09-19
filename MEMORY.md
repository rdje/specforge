# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`BOUNDED-DECISION-PROVIDER`** — **DECIDED `2026-09-19`: the provider is REJECTED**
  on measurement, keyless and zero-egress, without ever running arm C. **ADR 0051** is the record.
- Next action: **`BOUNDED-DECISION-PROVIDER.3`** — write the bounded-use contract anyway. A rejection
  must keep it as the standard the NEXT provider is measured against, so the next proposal is
  measured rather than re-argued. Design only, no network, no key.
- **Why it was rejected, do not re-derive it** — read ADR 0051 and
  `docs/research/bounded-decision-arm-b.md`. C1 asks a provider to beat the best LOCAL arm by 0.05
  macro-F1; arm B1 reached **0.95413** on `caption_admission`, so arm C needs **≥ 1.00413** and a
  perfect arm scores 1.00000. On `declaration_row` it needs 13 of 21 rows whose evidence is not in
  the row, reachable only by admitting identity with no attribute (24% precise), which C3 forbids.
- **Do not buy `TYPESAFE_API_KEY` for this evaluation.** `.4`/`.5` are reachable only if the director
  elects to run arm C anyway; `.2`'s roadmap amendment is moot (`ROADMAP.md:37` stands unchanged).
  Reopening needs one of: run arm C, revisit the 0.05 margin, or a different decision whose evidence
  IS in the candidate.
- **Shipping arm B1 is NOT done** and is owned elsewhere: the caption rules (title / cross-reference
  opening / negated permission) go to `INVARIANT-SHAPE-ADMISSION`, the direction-abbreviation rule to
  `SIGNAL-DECLARATION-ROW-DROP.2j`. Each needs corpus-wide adjudication over all 78 documents.
- **Latent stop:** `claims.jsonl` is at 94.7% of its byte ceiling with no archive path, owned by
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.36`; resolve it before registering another full claim record.
- The corpus is 27/27 current (`check_chain_currency.sh`).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` is unblocked; `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a
  provider and this decision does not give it one.
