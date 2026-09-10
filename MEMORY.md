# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`WIRE-BASED-100.9` CLOSED `2026-09-10`** — all three wire golds re-ingested and re-derived.
  Open in this tree: `.4a` (temporal antecedent identity, opened by `.9b`) and `.10` (AXI's 115 lost typed
  declarations, opened by `.9d`); `.2`/`.3`/`.4`/`.5` remain. Also opened by `.9b`:
  `RETAINED-BUNDLE-POPULATION-FROZEN` (`.1`/`.2`/`.3` pending). Other open trees: `KG-ISF-COMPLETENESS`
  beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION`
  `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: **APB, AHB and AXI are scoreable again. Twelve carried numbers re-derived: eleven held, one
  was withdrawn.** The corpus is **27 measurable (34.6%) / 51 legacy** and **5 of 7** gold documents score;
  the two that do not are the ones `.9a` routed out (RISC-V Debug, NVMe). Withdrawn: APB temporal
  `1.000 → 0.333` — `f88d463d` deleted `.4`'s `resolve_indexed_signal_family`, so the un-indexed prose `PSEL`
  no longer binds to declared `PSELx` and `PNSE`/`PBUSER` each lose an antecedent. **Case is NOT a second
  cause** — `eval::temporal_predicate_key` (`eval.rs:458`) uppercases every name; `.9b` claimed two routes and
  `.9d` disproved it. **The largest defect `.9` found was invisible to all twelve numbers:** AXI's declared
  inventory fell `289 → 159` with all 110 `*CHK` signals gone (`.10`).
- Next action: `WIRE-BASED-100.10` — bisect `2026-08-12..HEAD` with `git show <rev>:crates/specforge/src/ir/
  evidence.rs` to name the revision that stopped synthesizing `Signal <X> is width <n>.` from
  `Name | Signals covered | Width | Check enable` tables, then decide regression-vs-retirement (APB/AHB still
  carry their `*CHK` signals, so it is not a blanket ADR 0006 retirement) and fix or publish the retirement
  with its count. Alternatively `.4a`, or `RETAINED-BUNDLE-POPULATION-FROZEN.1`.
- In-flight uncommitted: none after this commit.
- Blockers: none that stop work. Standing hazards: **a green score is evidence only about the facts its gold
  names** — AXI kept all six numbers while losing 115 typed declarations. **The retained normalized-bundle set
  can neither grow nor shrink**; the APB/AHB/AXI bundles are HELD under `generated/preserved/
  WIRE-BASED-100.9b|9c|9d/`, so their EvidenceIR replays read UNMEASURABLE by design
  (`[[retained-bundle-population-is-frozen]]`). **Do not predict a re-derivation's verdict** — `.9b` predicted
  `.9c` would fail (it did not) and `.9d` predicted two outcomes and got both wrong; measure, then publish.
  **A legacy chain cannot be scored, so a retired fix costs a published number silently**; that missing
  canonical-currency gate is still unowned. Read a gate's cohort rule before treating its ratio as coverage
  (`[[corpus-canonical-currency-and-ownership]]`). Never infer ownership from a mention; read the named
  owner's contract, and search CLOSED leaves before publishing "nobody owns X". Attribute a regression from
  producer history (`git log -S`, `git show <rev>:<path>`), never a diff. A probe is not a port. A re-ingest
  destroys evidence no rebuild can restore — preserve first. The derived-state refresh chain an edit to
  `CHANGES.md`, the book, a card or `surfaces.jsonl` sets off is `[[live-surface-edit-bookkeeping-chain]]`;
  this file's cap is 50 lines. `LIVE_ACHIEVEMENT_STATUS.md` and `CHANGES.md` are both past their 80% warning
  band — `STATUS-LEDGER-ROLLOVER.2` and `CHANGES-LEDGER-ROLLOVER.4` own the rollovers. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — never run the fixture suite with the locality gate.
  `durability.stale_check` is never executed by any gate (`.18`), and no gate runs
  `scripts/validate_canonical_recovery_contract.py`.
