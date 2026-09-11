# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)


- Active unit: **`WIRE-BASED-100.10c` CLOSED `2026-09-11`** — AXI's AR side is declared for the first
  time and no real signal is missing against the legacy chain. Open in this tree: `.10b` (7 ISF prose
  members, 6 from one generic-channel-template table), `.4a`, `.2`/`.3`/`.4`/`.5`. Also open:
  `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`/`.2`/`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **Three defects in three slices, none of which any score could see.** `.10` a SourceIR
  classifier that dropped 115 typed AXI declarations; `.10a` a name-column scorer that disagreed with its
  own row loop and minted prose as signals (actors `134 → 25`); `.10c` two readers that dropped the whole
  AR side — a `Continued from previous page` fragment is now judged through its chain head, and a family
  cell (`AWPROT, ARPROT`) declares every member. AXI provenance `388 → 468` / distinct `288 → 303`,
  declared inventory and ISF ports both `280 → 295`; the residual vs legacy is `8`, all legacy junk.
  **Six scored numbers unchanged throughout.** Census 27 measurable (34.6%) / 51 legacy.
- Next action: `WIRE-BASED-100.10b` — the 7 prose members left in AXI's ISF interface. Six (`VALID`,
  `PENDING`, `RP`, `CRDT`, `CRDTSH`, `SHAREDCRD`) come from `table_0011` "Credited channel signals", whose
  Name column genuinely holds those tokens: it is a **generic channel template** describing the per-channel
  pattern (`AWVALID`, `AWCRDT`), not concrete wires, so the fix is semantic, not a column heuristic. The
  seventh, `AxLEN`, reaches the interface from prose while the declared `AXLEN` does not. `.6c`'s
  `ir/entity_typing` is the candidate fallback. Or `.4a`, `RETAINED-BUNDLE-POPULATION-FROZEN.1`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **a green score is evidence only about the facts its gold names** —
  AXI's six held while it lost 115 declarations, regained them, moved 134 → 25 actors and gained its whole AR side.
  **A SourceIR classification rule may not read its neighbours**: the proof kernel replays one field record at a time,
  so a rule needing document order belongs in EvidenceIR (`[[sourceir-classification-is-per-record]]`). **Two readers
  of the same data must tokenize identically.** **A SourceIR production change stales every persisted SourceIR proof
  at once** (an EvidenceIR change stales only EvidenceIR and below, no re-ingest); **equal counts do not prove an
  identical artifact**; **measure a producer change over every persisted SourceIR before shipping** — with commands,
  `[[qualified-role-header-proves-no-role]]`. A stale SourceIR proof is repaired by `source_proof_migrate`, not a
  re-ingest, including for the three wire golds: restore the held-out bundle for the duration of the chain and return
  it byte-identical (`[[retained-bundle-population-is-frozen]]`). **Do not predict a re-derivation's verdict** —
  `.9b`, `.9d`, `.10`'s junk-actor hypothesis, `.10a`'s own stated mechanism and `.10c`'s first design were all wrong.
  Read a gate's cohort rule before treating its ratio as coverage (`[[corpus-canonical-currency-and-ownership]]`).
  Never infer ownership from a mention; attribute a regression from producer history, never a diff; preserve before a
  re-ingest; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit
  sets off `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines. `durability.stale_check` is never
  executed by any gate (`.18`), and no gate runs `scripts/validate_canonical_recovery_contract.py`.
