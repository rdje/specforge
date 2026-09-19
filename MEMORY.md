# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`GATE-FIXTURE-EXEC-STALL.3`** — make the gate's stall self-identifying. A doctrine step
  that blocks at zero CPU with no output is indistinguishable from a hang; that ambiguity cost a session
  40 minutes, an aborted commit and a wrong first diagnosis (a pipe deadlock). Emit a per-doctrine
  progress marker and, past a generous threshold, name the host condition and how to confirm it.
- Next action: read `docs/tasks/GATE-FIXTURE-EXEC-STALL.md` — its frontier leads with `.3` deliberately,
  ahead of the larger `.1`/`.2` (census the fresh-executable multiplier, then remove it).
- **`CORPUS-CHAIN-CURRENCY` is closed for now and its corpus question is settled the opposite way from
  how `.10` framed it.** AXI/APB/AHB are **undeclared, not unrebuildable**. `.10a` refused the re-ingest
  `.10` decided; `.10b` shipped `scripts/probe_held_out_bundle_replay.sh` and found **six** held-out
  bundles, not three — `WIRE-BASED-100.9b`/`.9c`/`.9d` hold copies alongside `.10`, byte-identical
  markdown per document. All six replay **CONTENT SAME**. Claim
  `wire-gold-bundles-are-held-out-not-lost` is `verified`.
- **Do not re-ingest these three.** The remedy is `RETAINED-BUNDLE-POPULATION-FROZEN.3` (widened to all
  three), blocked on that tree's `.2` then `.1`, never on Docling. It can pre-flight itself with
  `probe_held_out_bundle_replay.sh` rather than learn the answer by performing the restore, and when it
  lands the bundles stop being held out — so that claim must then be marked **superseded**, not repaired.
- Rollback for that restore is now redundant: two independent bundle copies per document, plus
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` (21 files, 193,457,740 bytes, digest
  `5a5dffa2865f67ad`). `generated/` is git-ignored, so these are the only ways back. Keep them.
- Host hazard, and it will recur: macOS Gatekeeper assesses each newly created executable on first exec.
  A full gate measured **18m07s wall against 3m55s CPU**; a fresh script's first exec took 127 s against
  8 ms once assessed. A gate sitting at zero CPU is probably this, not a deadlock — check
  `XprotectService`/`syspolicyd` CPU before diagnosing the repository.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
