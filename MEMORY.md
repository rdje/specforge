# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`CLAIM-VERIFICATION-ADOPTION`** — `.18` closed today; `.16` and `.17` remain open and
  both are actionable in-session. `.17` is the nearer one and now has two recorded instances.
- Next action: `.17` — gate the census evidence-id convention or declare it decorative. The suffix is the
  first 12 hex of the region's content digest, 68 of 68 hold it, and **nothing checks it**; it has now
  broken twice, both times from automation repairing one region and disturbing a neighbour. Measure the
  population first, then pick: one comparison in `check_current_claim_census.pl` with a RED case, or
  correct `[[live-surface-edit-bookkeeping-chain]]` and stop relying on it.
- Current state: `EXTRACTION-QUALITY-GAUGE` is exhausted for in-session work and declares no eligible
  frontier — every open leaf needs a provider, a legacy re-ingest, or a detached full-CI run. Eight slices
  closed today. The programme-level finding is `.3j.3`: the LLM path is shown **60 of 326** obligations
  about declared signals — an **18.4%** ceiling — so the bottleneck is deterministic recall, not the model.
  `.18` widened the book claim gate's unit vocabulary by ten of the product's own nouns, 483 → 539.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none actionable. Resources the director could supply: a local model provider (unblocks
  `EXTRACTION-QUALITY-GAUGE.3j.4.a`, 62 calls) and a detached full-CI window (unblocks `.3k.9`).
