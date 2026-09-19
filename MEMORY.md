# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`GATE-FIXTURE-EXEC-STALL.2`** — the last open leaf of that tree, and `.1` narrowed it
  to one file. `scripts/test_live_document_size.pl` writes **226** of the **233** fresh executable
  paths a gate run creates, for **2** distinct contents (`exit 0`, `exit 1`). Two shared scripts would
  replace 226 first-exec assessments with 2.
- Next action: read `docs/tasks/GATE-FIXTURE-EXEC-STALL.md`. The saving is settled (≈23 s of every gate
  run at the measured quiet-host delta of 99 ms per fresh exec); **the design question is isolation** —
  fixtures must not be able to influence one another through a shared script, and nothing
  fixture-specific may ever be written into the shared pair. The 113 declared checks and their RED
  behaviour must be unchanged.
- Gate cost, measured `2026-09-19` and worth not re-deriving: quiet host **5m49.7s wall / 3m59.2s user
  CPU**, ALL 16 executed PASS; during an assessment episode the same day **18m07s / 3m55s** — identical
  work, 12m18s of waiting. `--fast` 51.8 s. A slow gate must be MEASURED, not attributed: run
  `scripts/probe_exec_assessment_latency.sh`, which has already excluded the host once and localised a
  regression to the driver instead.
- **`CORPUS-CHAIN-CURRENCY` is closed for now.** AXI/APB/AHB are **undeclared, not unrebuildable**.
  `.10a` refused the re-ingest `.10` decided; `.10b` shipped
  `scripts/probe_held_out_bundle_replay.sh` and found **six** held-out bundles, not three — `.9x` and
  `.10` each hold a copy, byte-identical markdown per document, all six replaying **CONTENT SAME**.
  Claim `wire-gold-bundles-are-held-out-not-lost` is `verified`.
- **Do not re-ingest those three.** The remedy is `RETAINED-BUNDLE-POPULATION-FROZEN.3` (widened to all
  three), blocked on that tree's `.2` then `.1`, never on Docling. When it lands the bundles stop being
  held out, so that claim must then be marked **superseded**, not repaired. Rollback is redundant: two
  bundle copies per document plus `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` (21 files,
  193,457,740 bytes, digest `5a5dffa2865f67ad`). `generated/` is git-ignored; these are the only ways back.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
