# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`CLAIM-VERIFICATION-ADOPTION.16`** — the tree's last open leaf. It governs the
  decision-bearing `N of M` population: 285 lines across 43 files, growing ~1.2 per commit, with its own
  registry and lifecycle rather than an extension of the census (`.13` sized the alternative at 4,987
  records against a sibling bound of 512).
- Next action: read `.16`'s contract in `docs/tasks/claim-verification-adoption/` before designing
  anything — `.13` already did the sizing, so the open question is the registry's shape and lifecycle,
  not whether to build it. `.17` and `.18` closed today and neither blocks it.
- Current state: `EXTRACTION-QUALITY-GAUGE` is exhausted for in-session work and declares no eligible
  frontier — every open leaf needs a provider, a legacy re-ingest, or a detached full-CI run. Eight slices
  closed today. The programme-level finding is `.3j.3`: the LLM path is shown **60 of 326** obligations
  about declared signals — an **18.4%** ceiling — so the bottleneck is deterministic recall, not the model.
  `.18` widened the book claim gate's unit vocabulary by ten of the product's own nouns, 483 → 539, and
  `.17` gated the census evidence-id suffix — the unique witness for a re-pin that lands on the wrong line.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none actionable. Resources the director could supply: a local model provider (unblocks
  `EXTRACTION-QUALITY-GAUGE.3j.4.a`, 62 calls) and a detached full-CI window (unblocks `.3k.9`).
