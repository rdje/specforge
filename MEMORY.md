# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`CORPUS-CHAIN-CURRENCY.10a`** — execute the re-ingest `.10` decided: **APB first**, then
  AHB, then AXI, one at a time. Docling is present and all three PDFs are under `corpus/`.
- Next action: **APB's gold result is a decision point, not a step.** Re-ingest APB, rebuild its cascade
  (evidence 0.5s, semantic 0.8s), then re-verify `WIRE-BASED-100` **before** starting AHB. If APB's scores
  move off `1.000`, **STOP and adjudicate** — AXI must not be touched. Update
  `doctrine/chain_currency/retained_bundles.json` (`retained: 24` → 25/26/27) in the same commit or
  `CHAIN-CURRENCY` fails closed both ways, and re-verify the measured stratum is still 27.
- Rollback, and it is the only one: `generated/` is git-ignored, so the snapshot at
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` is the sole way back — **21 files, 193,457,740
  bytes, digest `5a5dffa2865f67ad`**, verified byte-identical. Named in `.10` so the residue sweep spares
  it. Re-verify that census before relying on it.
- Why: `.5a` proved by A/B that composing a reader into a registered evidence derivation invalidates
  proof-carrying artifacts, and AXI/APB/AHB retain **no normalized bundle** so they cannot be rebuilt —
  three of the four wire-based golds would be lost permanently by any future evidence-producer change.
  `.10` closed (c) as impossible and (b) as foreclosing the producer work `.3j.3` says is the bottleneck.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none is a decision. `EXTRACTION-GAP-FIX.5b` waits on `.10a`; `.3j.4.a` wants a model provider.
