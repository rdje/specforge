# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `WIRE-BASED-100.9` — re-ingest the legacy wire golds so this tree's numbers can be re-derived.
  `.9a` and `.9b` (APB) COMPLETE; `.9c` AHB / `.9d` AXI `pending`. `.8` CLOSED. Opened by `.9b`:
  `WIRE-BASED-100.4a` (temporal antecedent identity) and the new tree
  `RETAINED-BUNDLE-POPULATION-FROZEN` (`.1`/`.2`/`.3` all pending). Also open:
  `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **APB is scoreable again and one published `1.000` is withdrawn.** The re-ingest
  restored `ihi0024_e` to canonical 3/3/2/2, so the corpus is **25 measurable (32.1%) / 53 legacy** and
  **3 of 7** gold documents can be scored. `seed_apb` re-derives EXACTLY — constraints `1.000`
  (tp=6 fp=0 fn=0), relations `1.000` (tp=5 fp=0 fn=0), doc-level recall 6/6 and 6/6.
  `seed_apb_temporal` does NOT: **`0.333` (tp=1 fp=2 fn=2)** against the carried `1.000`, so the tree's
  "APB 100% on ALL three aspects" headline is withdrawn. Cause: `.4`'s `resolve_indexed_signal_family`
  was deleted by `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` (`f88d463d`) when identifiers became opaque, so
  the gold's `PSELX` antecedent is unproducible. **The preserved pre-rebuild SemanticIR already carries
  both defects** — the loss was persisted and unscoreable, invisible for four weeks.
- Next action: `WIRE-BASED-100.9c` — same route on AHB `ihi0033_c` (preserve the chain first), and
  expect `seed_ahb_temporal` to fail the same way (`.5` reuses the resolver for `HSELx`); record it,
  do not absorb the fix. Alternatively take `RETAINED-BUNDLE-POPULATION-FROZEN.1` first — it unblocks
  the same wall `.9c`/`.9d` will hit.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **the retained normalized-bundle set can neither
  grow nor shrink** — a size literal in two contract validators, a `reclamations != []` freeze, and a
  set-equality join to a frozen behavioral qualification; APB's bundle is HELD at
  `generated/preserved/WIRE-BASED-100.9b/apb-normalized-bundle-held-out/`, so its EvidenceIR replay
  reads UNMEASURABLE by design until that tree's `.3`. **A re-ingest can return a DIFFERENT number
  from the one a tree carries** — publish the re-derivation as a verdict per aspect and withdraw, never
  carry. **A legacy chain cannot be scored, so a retired fix costs a published number silently**; that
  missing canonical-currency gate is still unowned. Read a gate's cohort rule before treating its ratio
  as coverage (`[[corpus-canonical-currency-and-ownership]]`). Never infer ownership from a mention;
  read the named owner's contract, and search CLOSED leaves before publishing "nobody owns X".
  Attribute a regression from producer history (`git log -S`), never by reading a diff. A probe is not
  a port; a rescue number is not a rescue. A re-ingest destroys evidence no rebuild can restore —
  preserve first. The derived-state refresh chain an edit to `CHANGES.md`, the book, a fact card or
  `surfaces.jsonl` sets off is `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
  Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite with the locality gate.
  `durability.stale_check` is never executed by any gate (`.18`), and no gate runs
  `scripts/validate_canonical_recovery_contract.py` — which currently reports four unrelated failures.
