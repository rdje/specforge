# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`CORPUS-CHAIN-CURRENCY.10b`** — ship `scripts/probe_held_out_bundle_replay.sh` as a tracked
  producer and lift the claim `wire-gold-bundles-are-held-out-not-lost` from `incomplete` to `verified`.
- **`.10a` refused the re-ingest `.10` decided, and that refusal is the result.** `.10` believed AXI, APB and
  AHB "cannot be re-derived at all". False. All three were re-ingested by `WIRE-BASED-100.9b`/`.9c`/`.9d`
  (`2026-09-10`) and again by `.10` (`2026-09-11`), and each run **held the bundle out** under
  `generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/` instead of declaring it,
  because the retention declaration is frozen. Measured `2026-09-19`: each replays `evidence --dry-run`
  **CONTENT SAME** against its persisted EvidenceIR — APB 0.30 s, AHB 0.67 s, AXI 3.28 s, 4.25 s for all
  three. They are **undeclared, not unrebuildable**, and no corpus mutation was performed.
- Next action: write the probe (copy bundle in → replay → `compare_stage_artifact` → remove, restoring
  pre-state on **every** exit path including failure), give it a `--self-test` RED matrix (missing bundle,
  content difference, already-populated normalized root, failing replay), then upgrade the registry record.
- Do **not** re-ingest these three. The remedy is `RETAINED-BUNDLE-POPULATION-FROZEN.3` (widened `2026-09-19`
  from APB-only to all three), which installs the held bundles and declares them; it is blocked on that
  tree's `.2` (what a newly retained key owes the frozen behavioral population) and `.1` (retire the
  redundant `24` literal), never on Docling.
- Rollback, still the only one: `generated/` is git-ignored, so
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` — **21 files, 193,457,740 bytes, digest
  `5a5dffa2865f67ad`**, re-censused `2026-09-19` — is the sole way back, and it is now the rollback for
  `RETAINED-BUNDLE-POPULATION-FROZEN.3` rather than for a re-ingest. Keep it; the residue sweep spares it
  because `.10`/`.10a` name it.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` still waits, now on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` still wants a model provider.
