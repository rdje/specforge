# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `WIRE-BASED-100.9` — re-ingest the legacy wire golds so this tree's numbers can be re-derived.
  `.9a` (measurement + ownership) COMPLETE; `.9b` APB / `.9c` AHB / `.9d` AXI `pending`. `.8` CLOSED. Also
  open: `KG-ISF-COMPLETENESS` beyond `.5` (all `.5` children done);
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **only 24 of 78 persisted documents (30.8%) pass the scorer's schema gate, and just 2 carry
  a gold**, and no gate published that until `.9a` derived it
  (`scripts/measure_corpus_canonical_currency.py`). `check_chain_currency.sh` reads `24/24 current` over the
  rebuildable stratum alone; `check_corpus_frontier.sh` reads `52 refreshed + 5 remaining` over a
  host-library sweep — and **31 of those 52 are still legacy**, because nothing ties `refreshed` to a schema.
  **`.8`'s hand-off of the re-ingest to that frontier is disproven**: its cohort rule is
  `excluded_source_prefixes: ["corpus/"]`, so all 18 legacy in-repo gold/eval documents are outside it by
  construction. **7 gold documents, 2 measurable** (SWD/ADI, I2C). **Cause: `CORPUS-PATTERN-REUSE.3c`
  refreshed APB/AHB/AXI on `2026-06-09` at 1.000; the `2026-08-12` schema bump made it legacy, unreported.**
  SWD is 5/29, not 29/29 (`.8c`).
- Next action: `WIRE-BASED-100.9b` — preserve the persisted APB chain on the repo volume FIRST (a legacy
  artifact cannot be regenerated), then `ingest`/`evidence`/`semantic` on tracked
  `corpus/arm/amba/core/apb/current/IHI0024_E_*.pdf`, then `eval-extraction --provider skip` on `seed_apb`
  + `seed_apb_temporal`, publishing the re-derived per-fact table against the carried 6/6 · 6/6 · 3/3.
  Routed OUT, still unowned: RISC-V Debug (`PDF-VARIANT-DIGESTION` register class); the 31
  refreshed-but-legacy cohort documents (what `refreshed` should mean); and **no gate fails when a chain
  falls below the canonical schema** — the defect that let a completed refresh rot unreported.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **read a gate's cohort rule before treating its ratio as
  coverage of anything** — the denominator a gate publishes is the population it was built for, not the one
  you are asking about (`[[corpus-canonical-currency-and-ownership]]`). **Never infer ownership from a
  mention or a hand-off sentence: read the named owner's own contract.** **The absence of an OPEN owner is
  not the absence of an owner — search CLOSED leaves before publishing "nobody owns X"** (`.9a` published
  that and withdrew it; a closed leaf held the causal story). Prove the oracle runs BEFORE trusting a green
  result; re-derive a published score before citing it; measure a proposed fix before building it (`.8d`'s
  scope rule is wrong on all 11 SWD frame fields). When a change retires a producer, the stale surfaces
  publish its NUMBER, not its ARTIFACT. Never name a stratum by one schema number — the legacy version
  differs per stage (1/2/1/1 against 3/3/2/2 current). **A probe is not a port** — publish from a committed
  derivation, never an ad-hoc script (`.8e`). **A rescue number is not a rescue** — `.8f`'s permissive
  detector "recovers" 5 of 11 SWD fields and all five are false positives. A re-ingest destroys evidence no
  rebuild can restore. The full derived-state refresh chain an edit to `CHANGES.md`, the book, a
  fact card or `surfaces.jsonl` sets off — including the ASCII-only rollover `reason` and this file's
  50-line cap — is `[[live-surface-edit-bookkeeping-chain]]`. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; and
  `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite with the locality gate. `durability.stale_check` is never executed by any gate (`.18`).
