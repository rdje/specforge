# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`WIRE-BASED-100.10a` CLOSED `2026-09-11`** — the name-column scorer disagreed with its own
  row loop and was minting prose as declarations. Open in this tree: `.10c` (continuation pages + multi-name
  cells), `.10b` (7 ISF prose members left, 6 from one template table), `.4a`, `.2`/`.3`/`.4`/`.5`. Also
  open: `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`/`.2`/`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **Two defects in two days, both invisible to all six scores in both directions.** `.10`
  fixed a SourceIR classifier that had dropped 115 typed AXI declarations; `.10a` fixed
  `signal_token_distinct` in `synthesize_signal_declarations`, which tokenized differently from the row loop
  beside it — a paired name cell (`AWMMUSECSID, ARMMUSECSID`) scored its column at ZERO, so the rotation
  override handed two tables to their Description column and `Secure`/`Stream`/`Asserted`/`The` became
  declarations, actors and `.isf` ports. Fix = tokenization parity + a ≥2-token override margin; 18 tables
  move and all 18 move back to their header column. AXI declared `277 → 280`, **actors `134 → 25`** (APB 8,
  AHB 25). Census 27 measurable (34.6%) / 51 legacy.
- Next action: `WIRE-BASED-100.10c` — the AR-side declarations two readers still drop. (a) `table_0255`
  (15 real signals) is a `Name | Width | Source | Description` page captioned "Continued from previous
  page": decide whether a self-declared continuation inherits its parent's kind, as document grammar — a
  SourceIR change, so it must ship with its own refresh. (b) a name cell listing two signals declares only
  the first (`AWMMUSECSID` yes, `ARMMUSECSID` no) — EvidenceIR-only, the cheaper half. Or `.10b`, `.4a`,
  `RETAINED-BUNDLE-POPULATION-FROZEN.1`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **a green score is evidence only about the facts its gold names** —
  AXI kept all six while losing 115 declarations, regaining them, and moving 134 → 25 actors. **Two readers of the
  same data must tokenize identically.** **A SourceIR production change stales every persisted SourceIR proof at
  once** (an EvidenceIR change stales only EvidenceIR and below, no re-ingest), so budget the refresh into the same
  slice; **equal counts do not prove an identical artifact**; **measure a producer change over every persisted
  SourceIR before shipping** — all four, with commands, are `[[qualified-role-header-proves-no-role]]`. The retained
  bundle set cannot grow or shrink; APB/AHB/AXI bundles are HELD under `generated/preserved/WIRE-BASED-100.9b|9c|10/`
  and may be restored read-only for a rebuild and returned byte-identical
  (`[[retained-bundle-population-is-frozen]]`). **Do not predict a re-derivation's verdict** — `.9b`, `.9d`, `.10`'s
  junk-actor hypothesis and `.10a`'s own stated mechanism were all wrong. Read a gate's cohort rule before treating
  its ratio as coverage (`[[corpus-canonical-currency-and-ownership]]`). Never infer ownership from a mention.
  Attribute a regression from producer history, never a diff. A probe is not a port. Preserve before a re-ingest. A
  live-surface edit sets off `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
  `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md` is at 91.5% of the 272 KB task ceiling, past its 90% rollover, with
  `scripts/check_active_task_evidence.pl --migrate` as the precedent; `LIVE_ACHIEVEMENT_STATUS.md`/`CHANGES.md` are
  past 80% under `STATUS-LEDGER-ROLLOVER.2`/`CHANGES-LEDGER-ROLLOVER.4`. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.4` —
  never run the fixture suite with the locality gate. `durability.stale_check` is never executed by any gate (`.18`),
  and no gate runs `scripts/validate_canonical_recovery_contract.py`.
