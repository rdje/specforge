# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **none in flight.** `RETAINED-BUNDLE-POPULATION-FROZEN` is **closed** — all four leaves
  done. The retained-bundle population now moves in both directions ADR 0025 mandates, and the three
  wire-based golds are back in the measurable corpus.
- Next action: pick a new tree. The leaf this work opened is **`CORPUS-CHAIN-CURRENCY.10c`** and it is
  small and well-specified: `probe_held_out_bundle_replay.sh --census` exits **1** when it correctly
  finds **0** held-out bundles, because `run_census` ends in `[ "$found" -gt 0 ] && …`. Zero is now the
  permanent healthy state, so the probe fails on its own success — ADR 0050's pattern in another file.
  No gate runs the probe, so nothing is red; it is wrong in silence.
- **The corpus is 27/27.** `check_chain_currency.sh`: 27 replayed / 27 current / 0 stale at evidence,
  semantic, intent and isf-adapter, with *retention: 27 — exactly the declared retained set*. `retained`
  is 27, `affected_chain_count` 27. The six preserved bundle copies are **gone** (583,434,736 bytes,
  residue census 0); all three source PDFs are present in `corpus/`, so they stayed rebuildable.
- **Retention is not qualification, and the gap is recorded, not implied.** The three golds are declared
  in `doctrine/production_genericity/post_boundary_retention.json` as unqualified residuals owing three
  held-out relations each to `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`. The behavioral gate states it every
  run: `3 retained post-boundary and unqualified`. **ADR 0050** is the rule that makes this sayable — a
  frozen qualification population binds as a subset FLOOR of the live set, never as an equality, and a
  list that grows must not live inside a frozen artifact (it would re-pin 36 digests per change).
- `wire-gold-bundles-are-held-out-not-lost` is **superseded**, not repaired — the state it asserted is
  one `.3` deliberately ended.
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` was waiting on `RETAINED-BUNDLE-POPULATION-FROZEN.3` and is
  **unblocked**; `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
