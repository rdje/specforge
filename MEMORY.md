# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`RETAINED-BUNDLE-POPULATION-FROZEN.4`** — decide the owner and disposition of
  `scripts/validate_canonical_recovery_contract.py`. `.1` measured it at **10 errors** on the live
  tree, byte-identical before and after its change, and **nothing in `check_doctrines.sh` or
  `run_ci.sh` invokes it** — so no gate has ever surfaced them. The decision is stale-by-design
  (retire with a decision record) versus unregistered-doctrine (wire it and make it green); it is not
  "fix it", and picking needs the canonical-recovery boundary's owner.
- Next action: read `docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md`. `.4` is independent of `.2`/`.3`
  and cheap to decide. `.2` (what a newly retained key owes the frozen behavioral population) is the
  substantive blocker and needs `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`'s owner plus a decision record.
- **`.1` is done: two of the three freeze mechanisms are retired.** The `len(retained_ids) != 24`
  literal and the `reclamations != []` freeze are gone from both validators; the count/digest binding
  they hid behind remains and is **stronger** than the literal — a digest catches a same-size
  substitution a size check never could. Proved by perturbation, not argued.
  `--self-test` is **40/40 RED + 1/1 admissible**: a 25-key set carrying a recorded reclamation is now
  ACCEPTED, and restoring either mechanism rejects it. **Mechanism 3 — the behavioral population
  set-equality join in `check_behavioral_genericity_contract.py` — is the only freeze still standing**
  between this tree and `.3`.
- `.3` (install and declare all three gold bundles) still needs `.2`. When it lands, the claim
  `wire-gold-bundles-are-held-out-not-lost` must be marked **superseded**, not repaired.
- **Do not re-ingest AXI/APB/AHB.** They are undeclared, not unrebuildable: six held-out bundles under
  `generated/preserved/WIRE-BASED-100.{9b,9c,9d,10}/`, all replaying CONTENT SAME. Check with
  `scripts/probe_held_out_bundle_replay.sh --census` (0.1 s) or the bare probe (~1 min).
- A slow gate must be **measured, not attributed**: `scripts/probe_exec_assessment_latency.sh`. The host
  assesses new executables intermittently; `GATE-FIXTURE-EXEC-STALL` is done and took the gate's own
  multiplier from 226 fresh inodes per run to 2.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none. `EXTRACTION-GAP-FIX.5b` waits on `RETAINED-BUNDLE-POPULATION-FROZEN.3`;
  `EXTRACTION-QUALITY-GAUGE.3j.4.a` wants a model provider.
