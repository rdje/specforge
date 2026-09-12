# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`INVARIANT-SHAPE-ADMISSION.0` CLOSED `2026-09-12`, and it split its own tree.** The 25.8% of published constraints that are not
  statements is a **symptom with two causes**: 739 figure captions are admission noise, and 769 table rows are an **extraction gap** — only 70 duplicate
  a declaration, the other 699 carry content found nowhere else and much of it normative (`| Secure | Must be zero |`). One rule over both would have
  deleted 699 real requirements. Open: **`INVARIANT-SHAPE-ADMISSION.1`/`.2`**; `PROSE-NAME-CELL-DECLARATION.3` (unblocked);
  `PRODUCTION-GRAPH-CENSUS-PIN` `.1`/`.2`; `SIGNAL-DECLARATION-ROW-DROP` `.2c`(unblocked)/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`;
  `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`;
  `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **the level is decided and the repository had already decided it** — a statement beginning with `|` is treated as a table row rather
  than prose in three existing places (`extract_actor_signal_relations`, `is_post_passive_binding_only_subject`, `is_standalone_markdown_block`);
  `is_invariant_like` simply does not use the idiom. The two upstream alternatives are refused on measurement: stopping a table row becoming a statement
  would break those three passes, which depend on such statements existing. **`r1` already protects what matters** — it is tested first, so the 20
  captions carrying a modal verb are admitted by `r1` and a rule confined to `r2`/`r3` cannot reach them; 739 removed, 20 kept, no second condition
  needed. **15 registered doctrines**, 13 at gate tier. Census 27 measurable / 51 legacy.
- Next action: `INVARIANT-SHAPE-ADMISSION.1` — a figure/table caption is not admitted by `r2` or `r3`, in `is_invariant_like`. Removes a measured **739
  of 5,856** current SemanticIR invariants (104 via `r2`, 635 via `r3`). Verify with an observed RED on a bare caption and GREEN on a modal-bearing one;
  sample and adjudicate the removed set; show no prose statement is affected; re-score the golds. **Budget for the rebuild** — this touches most
  documents in the proof-carrying stratum, not one or two, so plan the bundle restores (`generated/preserved/WIRE-BASED-100.10/` holds AHB/APB/AXI)
  before starting rather than discovering them mid-slice.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **a symptom that sums two populations invites one rule that is wrong for both** — 739 captions + 699 table rows
  looked like one 25.8% defect. **Look for an existing idiom before inventing a rule**: three passes already skipped `|` rows and the fourth simply did
  not. **A phantom declaration widens every gate that asks 'does this statement mention a declared signal?'** **Disprove your own tree's premise before
  building on it** — two trees this session were opened on a reading of an artifact and corrected by their own first leaf. **A correction upstream can
  expose a silent drop downstream**; only an item-by-item diff of a rebuild finds it. **Measure what a path is WORTH before measuring what a rule
  costs.** **Ask which producer wrote a persisted field before treating it as a defect**
  (`[[persisted-table-kind-is-a-classifier-generation-artefact]]`, `[[declared-spelling-is-the-document-spelling]]`). **A number that justifies a rule
  must be counted over the rule's whole population.** **A check that prints a number is not a check** (`PRODUCTION-GRAPH-CENSUS-PIN`). **A mirror of a
  producer must move with the producer.** **Do not A/B a gold across a producer change**: the seal refuses the pre-change artifact by design. **A green
  score is evidence only about the facts its gold names.** A producer change stales the seals of the documents it touches; restore the bundle from
  `generated/preserved/`, rebuild in the interleaved order with exactly one validate per artifact, return it byte-identical
  (`[[retained-chain-rebuild-order]]`). **The doctrine driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`). **Re-anchor a
  digest-bound region by its TEXT.** Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off
  `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
