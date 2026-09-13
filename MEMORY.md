# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`TEXT-LAYER-IDENTIFIER-SPLIT.0` CLOSED `2026-09-14` (PROBE/DOC, no rule)** — a name cell the PDF
  split into two words cannot be rejoined by its SHAPE, and the one test that IS precise repairs nothing.
- **The column signature is not the fact.** "A leading token repeated down the column with distinct
  continuations" selects **120 columns** corpus-wide (10 current, 110 legacy) and in the current stratum **not one
  is a name column** — all ten are DESCRIPTION columns opening with the same word (`Transaction identifier`,
  `Global clock`, `User request`). Five of the nine legacy columns the reader CHOOSES are not splits either.
  Fourth time a cheap structural rule in this area selected mostly prose.
- **The in-document join is precise and ships nothing.** Asking whether the document itself spells
  `<lead>_<continuation>` selects **3 of 120**, all AMD IOMMU (`iommu_info`, `iommu_attributes`), 0 wrong — and all
  three are in tables that DECLARE NOTHING. A rule whose whole measured effect is zero declarations has no
  evidence behind it.
- **TileLink's joined spelling is vector text inside three timing diagrams**, and the ingest is NOT at fault: it
  captures figures as images by design, so all 36 underscores (10 distinct names, pages 27/33/34) are outside every
  text surface SpecForge reads — and inside one it already stores (`picture-0007/0009/0010.png`, caption-only
  observations, no VLM read). `.1` asks the load-bearing question: **may a name read from a FIGURE ground a table
  row's identity, or only corroborate it?**
- **Second notation found**: eMMC `table_0221` writes `t PERIOD` / `t TLH , t THL` for `tPERIOD` / `tTLH` / `tTHL`
  — a SUBSCRIPT split with no separator, which no underscore-keyed rule reaches, in a cell that also carries a
  comma family. `.2`. That table DECLARES.
- Next action: `TEXT-LAYER-IDENTIFIER-SPLIT.1` (the figure-identity decision, not the VLM call),
  `CORPUS-CHAIN-CURRENCY.4`, `EXTRACTION-QUALITY-GAUGE.3k.12`, the unsized `.3j`, or
  `SIGNAL-DECLARATION-ROW-DROP.2d`/`.4b`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit.
- Blockers: none. Push cadence is **400** per director directive `2026-09-13`, FIXED there, so no push is due at 246; directive 16 still gates it on
  full CI. Standing hazards, each with its procedure in a fact card: **an evidence-stage change that MOVES a current-schema artifact's content stales
  its proof and it then refuses to LOAD** — use that refusal as the signal for WHICH documents move; rebuild `evidence → validate → semantic →
  validate → intent → validate → adapt`, each validated once, upstream-first, restoring the bundle from
  `generated/preserved/WIRE-BASED-100.10/{ahb,axi,apb}-normalized-bundle-held-out`, then `diff -rq`, remove, retention back to **24**. **A census
  boundary keyed on a PERSISTED field is a lower bound on what the reader reads** (`[[a-width-cell-that-is-a-sentence-is-not-a-width]]`); the join rate
  is the control that says so. **A new fact card may not reuse a question key another card already answers** — the knowledge-map generator fails closed
  on the collision. A corpus-root tool REFUSES an off-volume path. **A census counts the population of the FUNCTION being changed**
  (`[[constraint-record-producer-strata]]`), **never from a mirror**. **A Rust change moves `flow_census.json`; the re-derivation must ATTRIBUTE the
  delta to the owning leaf**; a new module moves `module_inventory.tsv`; a new command `CLI_SURFACE_REGISTRY`; a new `scripts/` file must be STAGED or
  claim-verification calls it untracked. **Editing a live surface sets off a fixed bookkeeping chain** —
  `[[live-surface-edit-bookkeeping-chain]]`; the book's line-pinned regions re-anchor BY CONTENT, and the claim IDs that move ARE the
  `Published-claims:` declaration. **The doctrine driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`); `evidence.rs` tests live in
  **`-p specforge-core`**; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). This file's cap is 50 lines.
