# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`ANCHORLESS-INVARIANT-DROP.0` CLOSED `2026-09-12` by disproving its own tree, which is now `done` with no code change.**
  `related_interface_ids` does not gate publication — **560 of ADIv6's 593 invariants carry no anchor** and publish fine. The phantom `In` was supplying
  an **admission gate**, not an anchor: `is_invariant_like` admits a statement that *mentions a declared signal* plus a weak phrase such as `state`.
  Open: **`INVARIANT-SHAPE-ADMISSION.0`** (new); `PROSE-NAME-CELL-DECLARATION.3` (unblocked); `PRODUCTION-GRAPH-CENSUS-PIN` `.1`/`.2`;
  `SIGNAL-DECLARATION-ROW-DROP` `.2c`(unblocked)/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100`
  `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **the census found a defect four times larger than the one it was looking for.** At the product boundary, **1,528 of 5,927 current
  IntentIR constraints (25.8%) are not statements** — 769 serialized markdown table rows and 759 figure captions, one reading `Figure 1.` in full. The
  `r3` visual-evidence route is the worst: **910 non-statements against 245 prose**. Many table rows are AXI signal-description rows already extracted
  correctly as declarations, so they are published twice. Also corrected: the 13 ADIv6 records `ACTOR-NOUN-RELATION-DECLARATION.1` recorded as 'real
  JTAG requirements' lost contain **no modal verb** — descriptive, not normative, so their removal was a precision gain. **15 registered doctrines**, 13
  at gate tier. Census 27 measurable / 51 legacy.
- Next action: `INVARIANT-SHAPE-ADMISSION.0` — **decide the level, not the rule**, with no code change. The shape test is trivial (a leading `|`, or
  `Figure`/`Table` + a number); what is undecided is where it belongs: at `is_invariant_like` (cheap, covers all three routes), at the statement source
  (a caption should arguably never be an invariant candidate for any pass), or at the SourceIR/EvidenceIR boundary where a table row becomes a statement
  at all — the only level that stops the double-publication. Measure each level's blast radius before choosing; statement text is read by more than one
  pass and this repository has twice measured such a radius to be surprising.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **a phantom declaration widens every gate that asks 'does this statement mention a declared signal?'** — it does not
  merely add phantom records. **Disprove your own tree's premise before building on it**: two trees this session were opened on a reading of an artifact
  and corrected by their own first leaf. **A correction upstream can expose a silent drop downstream** — only an item-by-item diff of a rebuild finds
  it; a count says 614 → 601 and nothing else. **Measure what a path is WORTH before measuring what a rule costs.** **Ask which producer wrote a
  persisted field before treating it as a defect** (`[[persisted-table-kind-is-a-classifier-generation-artefact]]`,
  `[[declared-spelling-is-the-document-spelling]]`). **A number that justifies a rule must be counted over the rule's whole population.** **A comma is
  an author enumerating; a space is the default separator between any two words.** **A check that prints a number is not a check**
  (`PRODUCTION-GRAPH-CENSUS-PIN`). **A mirror of a producer must move with the producer.** **Do not A/B a gold across a producer change**: the seal
  refuses the pre-change artifact by design. **A green score is evidence only about the facts its gold names.** A producer change stales the seals of
  the documents it touches; restore the bundle from `generated/preserved/`, rebuild in the interleaved order with exactly one validate per artifact,
  return it byte-identical (`[[retained-chain-rebuild-order]]`) — a rollback is a rebuild too. **The doctrine driver runs no cargo gate**
  (`[[doctrine-driver-runs-no-cargo-gate]]`). **Re-anchor a digest-bound region by its TEXT.** Never run the fixture suite with the locality gate
  (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
