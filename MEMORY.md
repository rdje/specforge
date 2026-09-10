# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `WIRE-BASED-100.9` — re-ingest the legacy wire golds so this tree's numbers can be re-derived.
  `.9a`/`.9b` (APB)/`.9c` (AHB) COMPLETE; `.9d` AXI `pending` and it CLOSES `.9`. `.8` CLOSED. Opened by
  `.9b`: `WIRE-BASED-100.4a` (temporal antecedent identity) and the tree
  `RETAINED-BUNDLE-POPULATION-FROZEN` (`.1`/`.2`/`.3` pending). Also open: `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **APB and AHB are scoreable again; one published `1.000` was withdrawn and four survived.**
  The corpus is **26 measurable (33.3%) / 52 legacy** and **4 of 7** gold documents can be scored. APB
  (`.9b`): constraints `1.000` (tp=6 fp=0 fn=0) and relations `1.000` (tp=5 fp=0 fn=0) hold, doc-level 6/6 and
  6/6; **temporal withdrawn `1.000` → `0.333`** (tp=1 fp=2 fn=2) because `f88d463d` deleted `.4`'s
  `resolve_indexed_signal_family` when identifiers became opaque, making the gold's `PSELX` antecedent
  unproducible — and the PRESERVED pre-rebuild SemanticIR already carried the defect, so it was invisible for
  four weeks. AHB (`.9c`): all four numbers re-derive exactly, because its temporal gold's only antecedent is
  `HREADY`, declared with the prose spelling.
- Next action: `WIRE-BASED-100.9d` — same route on AXI `ihi0022_l` (2.0 MB, a 14.5 MB persisted chain;
  preserve FIRST, watch RAM). Check the AXI temporal gold's antecedents against the declared catalog BEFORE
  predicting a verdict — that check, not the protocol, decided `.9b` vs `.9c`. It also answers
  `KG-ISF-COMPLETENESS.5.iv.a`'s open `AWATOP` enum prediction. Expect: canonical 3/3/2/2, the adapter moving
  `renderable` → `blocked`, identifiers losing their uppercasing, and the bundle HELD OUT.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **the retained normalized-bundle set can neither grow nor
  shrink** — a size literal in two contract validators, a `reclamations != []` freeze, and a set-equality join
  to a frozen behavioral qualification; the APB and AHB bundles are HELD under
  `generated/preserved/WIRE-BASED-100.9b|9c/`, so their EvidenceIR replays read UNMEASURABLE by design
  (`[[retained-bundle-population-is-frozen]]`). **A re-ingest can return a DIFFERENT number from the one a
  tree carries** — publish a verdict per aspect and withdraw, never carry; and **do not predict the verdict
  from the protocol** (`.9b` predicted `.9c` would fail; it did not). **A legacy chain cannot be scored, so a
  retired fix costs a published number silently**; that missing canonical-currency gate is still unowned. Read
  a gate's cohort rule before treating its ratio as coverage
  (`[[corpus-canonical-currency-and-ownership]]`). Never infer ownership from a mention; read the named
  owner's contract, and search CLOSED leaves before publishing "nobody owns X". Attribute a regression from
  producer history (`git log -S`), never a diff. A probe is not a port; a rescue number is not a rescue. A
  re-ingest destroys evidence no rebuild can restore — preserve first. The derived-state refresh chain an edit
  to `CHANGES.md`, the book, a card or `surfaces.jsonl` sets off is
  `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines. `LIVE_ACHIEVEMENT_STATUS.md` and
  `CHANGES.md` are both past their 80% warning band — `STATUS-LEDGER-ROLLOVER.2` and
  `CHANGES-LEDGER-ROLLOVER.4` own the rollovers. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`;
  `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture
  suite with the locality gate. `durability.stale_check` is never executed by any gate (`.18`), and no gate
  runs `scripts/validate_canonical_recovery_contract.py`.
