# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`RETAINED-BUNDLE-POPULATION-FROZEN.3`** — the tree's last leaf, now **unblocked**.
  Restore `generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/` to each
  document's normalized root, declare all three, and re-run the currency gate.
- Next action: read `docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md` `.3`, whose remaining obligation is
  **measured, not assumed**. At 27 retained: the behavioral gate is **green** provided the three keys are
  declared in `doctrine/production_genericity/post_boundary_retention.json` (each naming its owed
  relations + `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`); `validate_residual_actionability_contract.py`
  reports exactly 2 problems, both bookkeeping — `reconciliation.affected_chain_count` → 27 and
  `affected_chain_ids_sha256` recomputed; `check_corpus_frontier_census.pl` is unaffected. Pre-flight the
  restore with `scripts/probe_held_out_bundle_replay.sh --census` (0.1 s); rollback snapshot is
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` (21 files, 193,457,740 bytes, `5a5dffa2865f67ad`).
- **All three freeze mechanisms are now retired.** `.1` took the `24` literal and the `reclamations != []`
  freeze; `.2` took the behavioral set-equality join, replacing it with **ADR 0050**'s subset floor plus
  declared residual. `--self-test` 23/23 RED + 1/1 admissible. When `.3` lands, the claim
  `wire-gold-bundles-are-held-out-not-lost` must be marked **superseded**, not repaired.
- Two `.2` results worth not re-deriving. `.4`'s scoping said the fix was "not one comparison" and that
  `population_assertions.current_documents` and `frozen_census` each needed adjudicating — **it was one
  comparison**; both compare against `len(rows)` from the frozen TSV, never against `retained`, and they
  **stay exact**. And a declaration that grows must not live in a frozen artifact: putting
  `unqualified_keys` in `behavioral_qualification.json` costs **36** digest re-pins per amendment (35 of
  them held-out attempt identities) and recurs on every retention — hence its own live file, zero re-pins.
- **Do not re-ingest AXI/APB/AHB.** Undeclared, not unrebuildable: six held-out bundles under
  `generated/preserved/WIRE-BASED-100.{9b,9c,9d,10}/`, all CONTENT SAME.
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
