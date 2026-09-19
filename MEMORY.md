# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`RETAINED-BUNDLE-POPULATION-FROZEN.1`** — retire the redundant `len(retained_ids) != 24`
  literal and the `reclamations != []` freeze in `scripts/validate_residual_actionability_contract.py`
  (line ~592) and `scripts/validate_canonical_recovery_contract.py` (line ~455), keeping
  `affected_chain_ids_sha256` as the real binding. It is the mechanical half of the wall that blocks
  the gold-bundle restore.
- Next action: read `docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md`. `.1` does not need `.2`; `.3`
  (install and declare all three gold bundles) needs both. An open question that tree records and `.1`
  must not silently answer: `validate_canonical_recovery_contract.py` may have no owner — nothing in
  `check_doctrines.sh` or `run_ci.sh` invokes it, and it reports four unrelated failures.
- **`GATE-FIXTURE-EXEC-STALL` is DONE** (`.1`/`.2`/`.3`). Assessment is keyed to the inode, not the
  path, so the two fixture verifier contents are written once per run and hard-linked into each
  fixture: **226 new executable inodes → 2**, `test_live_document_size.pl` **30.4 s → 9.0 s at
  identical user CPU**, 113/113 unchanged. The isolation rule that cost one wrong attempt is
  **replace the path, never modify it** — a `chmod` follows a hard link and reached every fixture.
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`. The
  host condition is intermittent (an episode gave 18m07s wall / 3m55s CPU where quiet gives 5m49.7s /
  3m59.2s) and the probe has already excluded it once, localising a regression to the driver instead.
- **`CORPUS-CHAIN-CURRENCY` is closed for now.** AXI/APB/AHB are **undeclared, not unrebuildable**:
  `.10a` refused the re-ingest `.10` decided, `.10b` shipped
  `scripts/probe_held_out_bundle_replay.sh` and found **six** held-out bundles, all replaying CONTENT
  SAME. **Do not re-ingest them.** The remedy is `RETAINED-BUNDLE-POPULATION-FROZEN.3`, and when it
  lands the claim `wire-gold-bundles-are-held-out-not-lost` must be marked **superseded**, not repaired.
  Rollback is redundant: two bundle copies per document plus
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` (21 files, 193,457,740 bytes, digest
  `5a5dffa2865f67ad`). `generated/` is git-ignored; these are the only ways back.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
