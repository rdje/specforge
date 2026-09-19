# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`GATE-FIXTURE-EXEC-STALL.1`** — census the fresh-executable multiplier. Count the
  distinct executable PATHS the gate creates and execs in one run against the distinct CONTENTS behind
  them, per producing check. `test_live_document_size.pl` is believed to be ~113 paths for 3 contents,
  and it is unlikely to be the only producer; a fix that lands on one of several will not move the
  wall clock.
- Next action: read `docs/tasks/GATE-FIXTURE-EXEC-STALL.md`. `.1` also owes a **frequency**
  measurement, not just a magnitude: `.3` found the host quiet (134 ms fresh vs 13 ms cached) while the
  gate was slow, so the 127 s first-exec figure that opened this tree is real but **intermittent**.
  State how often, not how bad.
- `.3` is done: `check_doctrines.sh` names each doctrine on stderr before running it and, past
  `SPECFORGE_DOCTRINE_STALL_SECONDS` (default 120), says which one is still going and that ~0% CPU
  means waiting, not hanging. `scripts/probe_exec_assessment_latency.sh` settles it either way.
  **That probe already excluded a wrong hypothesis** — it is the reason a self-inflicted regression in
  the watchdog (a command substitution that held its own stdout pipe, adding the full notice interval
  before every doctrine) was caught before it shipped. `--fast` is 17.3 s.
- **`CORPUS-CHAIN-CURRENCY` is closed for now.** AXI/APB/AHB are **undeclared, not unrebuildable**.
  `.10a` refused the re-ingest `.10` decided; `.10b` shipped
  `scripts/probe_held_out_bundle_replay.sh` and found **six** held-out bundles, not three — the `.9x`
  and `.10` preservation points each hold a copy, byte-identical markdown per document, all six
  replaying **CONTENT SAME**. Claim `wire-gold-bundles-are-held-out-not-lost` is `verified`.
- **Do not re-ingest those three.** The remedy is `RETAINED-BUNDLE-POPULATION-FROZEN.3` (widened to all
  three), blocked on that tree's `.2` then `.1`, never on Docling. It can pre-flight itself with the
  probe. When it lands the bundles stop being held out, so that claim must then be marked
  **superseded**, not repaired.
- Rollback for that restore is redundant: two bundle copies per document, plus
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` (21 files, 193,457,740 bytes, digest
  `5a5dffa2865f67ad`). `generated/` is git-ignored, so these are the only ways back.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
