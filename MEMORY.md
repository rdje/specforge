# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`WIRE-BASED-100.10` CLOSED `2026-09-11`** — AXI's lost typed declarations attributed and
  recovered. Open in this tree: `.10a` (30 declarations still missing), `.10b` (AXI's 11 prose ISF members,
  now proven independent), `.4a`, `.2`/`.3`/`.4`/`.5`. Also open:
  `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`/`.2`/`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **The 115-declaration loss was in the SourceIR table classifier, not `evidence.rs`.**
  `dee0740f` (`2026-08-12`, `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`) narrowed role matching to whole-label
  equality, so `Name | Signals covered | Width | Check enable` fell to `unknown` — and an `unknown` table
  never reaches declaration synthesis (`evidence.rs:4041`), so `.9d`'s `_ => continue` seam and its
  "SourceIR structurally identical" exoneration are both **corrected where they were published**. The fix
  reads a generic interface noun as a whole word in a column label; over all 11,033 persisted tables it
  moves **exactly 7**. AXI provenance `265 → 388` / distinct `170 → 293` / `*CHK` `0 → 115`; declared
  inventory and ISF ports both `159 → 277`. **All six wire numbers re-derive unchanged**; census holds at
  **27 measurable (34.6%) / 51 legacy**, 5 of 7 golds.
- Next action: `WIRE-BASED-100.10a` — the 30 AXI declarations still missing, in two disjoint groups:
  `table_0251`/`table_0255` (22) are `Name | Width | Source | Description` continuation pages that lost
  `Source`-as-direction and have no signal-naming caption; `table_0059`/`table_0187`/`table_0259` (8) are
  already `signal_description` and lose rows inside `synthesize_signal_declarations` — the only place
  `.9d`'s `_ => continue` hypothesis can still hold. Or `.10b`, `.4a`, `RETAINED-BUNDLE-POPULATION-FROZEN.1`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **a green score is evidence only about the facts its gold names** —
  AXI kept all six numbers while losing 115 declarations AND while regaining 118. **A SourceIR production change
  stales every persisted SourceIR proof at once**, so budget the refresh into the same slice; **equal artifact counts
  do not prove an identical artifact**; and **measure a classifier change over every persisted SourceIR before
  shipping it** — all three, with commands, are `[[qualified-role-header-proves-no-role]]`. **The retained
  normalized-bundle set can neither grow nor shrink**; APB/AHB/AXI bundles are HELD under
  `generated/preserved/WIRE-BASED-100.9b|9c|10/`, so their replays read UNMEASURABLE by design
  (`[[retained-bundle-population-is-frozen]]`). **Do not predict a re-derivation's verdict** — `.9b`, `.9d` and
  `.10`'s own junk-actor hypothesis were all wrong; measure, then publish. Read a gate's cohort rule before treating
  its ratio as coverage (`[[corpus-canonical-currency-and-ownership]]`). Never infer ownership from a mention.
  Attribute a regression from producer history (`git log -S`, `git show <rev>:<path>`), never a diff. A probe is not a
  port. A re-ingest destroys evidence no rebuild can restore — preserve first. A live-surface edit sets off
  `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines. `docs/tasks/WIRE-BASED-100.md` is at
  **91.5%** of its 272 KB ceiling — past its 90% rollover milestone and unowned;
  `LIVE_ACHIEVEMENT_STATUS.md`/`CHANGES.md` are past 80% under `STATUS-LEDGER-ROLLOVER.2` and
  `CHANGES-LEDGER-ROLLOVER.4`. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`;
  `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite with
  the locality gate. `durability.stale_check` is never executed by any gate (`.18`), and no gate runs
  `scripts/validate_canonical_recovery_contract.py`.
