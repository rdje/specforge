# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`RETAINED-BUNDLE-POPULATION-FROZEN.2`** — the last open leaf of that tree and the only
  freeze still standing: the behavioral population **set-equality join** in
  `scripts/check_behavioral_genericity_contract.py` (~line 1392), which requires the behavioral row set
  to equal `retained`, so a newly retained key must also arrive as a fully qualified held-out row.
- Next action: read `docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md`, where `.4` scoped `.2` without
  starting it. **Not blocked on an owner** — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v` is closed. The
  candidate shape is right and is **ADR 0049's class of error**: `held_out_policy` defines the
  population *"at the selection boundary"* and pins `selection_boundary_commit`, so it is a snapshot
  wired as a live equality invariant; the remedy is subset-plus-report, not retirement, because the
  frozen selection still carries real evidence and only its equality is wrong. **But it is not one
  comparison**: `population_assertions.current_documents: 24` and `frozen_census` are frozen boundary
  values too, and each needs adjudicating — which stay exact as release evidence, which become
  subset-or-reported. That adjudication plus a decision record is the slice.
- `.1` and `.4` are done. Mechanisms 1 and 2 (`len(retained_ids) != 24`, `reclamations != []`) are
  retired; the count/digest binding that replaces them is **stronger** — a digest catches a same-size
  substitution a size check cannot. `--self-test` 40/40 RED + 1/1 admissible (a 25-key set carrying a
  recorded reclamation is now ACCEPTED).
- `.4` retired `scripts/validate_canonical_recovery_contract.py` (**ADR 0049**). It was red **because
  the repair it froze had landed** — the live tree matches the contract's `expected_canonical_keys`,
  not its pre-repair `current_canonical_keys`. Never wire a pre-repair freeze and never regenerate its
  witness; its matrix already runs as 6 Rust tests in `evidence.rs`.
- `.3` (install and declare all three gold bundles) still needs `.2`. When it lands the claim
  `wire-gold-bundles-are-held-out-not-lost` must be marked **superseded**, not repaired.
- **Do not re-ingest AXI/APB/AHB.** Undeclared, not unrebuildable: six held-out bundles under
  `generated/preserved/WIRE-BASED-100.{9b,9c,9d,10}/`, all CONTENT SAME. Check with
  `scripts/probe_held_out_bundle_replay.sh --census` (0.1 s).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`.
  `GATE-FIXTURE-EXEC-STALL` is done (226 fresh executable inodes per run → 2).
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
