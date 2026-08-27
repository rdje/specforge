# SOURCE-IR-REPRODUCIBILITY: make SourceIR ingest reproducible, and gate it

## Metadata

- Tree ID: `SOURCE-IR-REPRODUCIBILITY`
- Status: `active` (`.0` measured; `.1`–`.3` pending)
- Roadmap lane: repository durability and portability (sibling of `CORPUS-CHAIN-CURRENCY`)
- Created: `2026-08-27`
- Last updated: `2026-08-27`
- Owner: repo-local workflow

## Goal

Make "the persisted `SourceIR` reflects what the current toolchain produces from the same PDF" a **measured,
gated** property, and stop reviewed fixtures from anchoring on identifiers that a segmentation change silently
invalidates.

`CORPUS-CHAIN-CURRENCY` proves the rest of the chain: for every persisted artifact, replaying the owning stage
at the **fixed persisted input** must reproduce it. That oracle starts from the persisted `source_ir.json` and
never re-runs ingest, so it is structurally blind to `SourceIR` itself. A 24/24 current chain is a true
statement about EvidenceIR through the adapter and says nothing about the artifact they all descend from.

## Non-Goals

- Do not make Docling itself deterministic; that is an upstream dependency, not a SpecForge surface.
- Do not re-anchor a reviewed fixture cell to whatever ingest currently emits. That would make the fixture agree
  with drift instead of detecting it, which is the failure this tree exists to prevent.
- Do not re-ingest the corpus as a side effect of measurement; measurement must be bounded and explicit.

## Reproduction and measurement (`.0`, `2026-08-27`, complete)

Found by `SPEC-TO-INTENT-ALIGNMENT.8d`'s control leg, not by a gate. Replaying the reviewed population from
clean production made the Cortex-A76 guide's reviewed prose cell fail its source-region query for the first
time, with `source_region_missing_or_ambiguous`, dropping exact source regions from 14/14 to 13/14.

The reviewed cell is anchored on the **ordinal** element id `elem_00219` plus an asserted excerpt. The current
toolchain places that prose at `elem_00230`:

| Ingest | content elements | reviewed prose at | tables | visual assets | pages |
| --- | ---: | --- | ---: | ---: | ---: |
| persisted `generated/` chain (normalized bundle cached `2026-08-10`) | 249 | `elem_00219` | 68 | 71 | 46 |
| current toolchain (this measurement) | 260 | `elem_00230` | 68 | 71 | 46 |

An 11-element shift, with table, visual-asset, and page counts identical. The fixture failed closed — it
refuses an element whose text does not contain the reviewed excerpt rather than matching the wrong one.

**The cause is not this repository's code.** Every usual suspect is excluded by measurement:

- **Input.** The PDF is byte-identical throughout: SHA-256 `8358c5ae…3a22`, unchanged on disk since
  `2026-04-02`, and the replay orchestrator asserts source digest equality before ingesting.
- **Revision.** `483e525d` and `3d04bde0` — the commit *before* the only revision in range that touched
  `crates/specforge/src/ir/source/docling_backend.rs` — both produce 260. Between the published `.7c.ii`
  revision and `483e525d` the only change under `crates/specforge/src/ir/source*` is additive (a new record
  type), and no file outside `crates/`, `docs/`, and `doctrine/` changed at all.
- **Toolchain.** Docling `2.84.0`, `docling-core 2.78.0`, `docling-ibm-models 3.13.2`, `docling-parse 5.11.0`,
  unchanged on disk since `2026-08-08`; no model blob under `.cache/huggingface` is newer than `2026-08-09`.
- **Batching.** Batched (`SPECFORGE_INGEST_BATCH_PAGES=16`) and unbatched ingest both produce 260.
- **Run-to-run noise.** Two back-to-back runs are byte-identical after normalizing the replay root, so ingest is
  stable *within* a session. Artifact digests always differ across roots because `SourceIR` embeds its own
  output paths; that is expected and is not drift.

What remains is that the same PDF, toolchain, and models produced 249 elements when the persisted bundle was
built and produce 260 now. `SourceIR` ingest is therefore not reproducible across time in this environment, and
no doctrine can currently observe it.

### Why this matters beyond one cell

1. Every persisted `generated/source_ir/*` artifact may be stale in a way no gate detects, and every downstream
   stage inherits that staleness while `CHAIN-CURRENCY` reports current.
2. Reviewed fixtures anchor cells on ordinal ids, so any segmentation change moves anchors and converts an
   ingest question into a scoring failure in an unrelated program.

## Planned children

- ID: `SOURCE-IR-REPRODUCIBILITY.1`
  State: `pending`
  Goal: measure the standing SourceIR drift across the persisted corpus
  Acceptance: a bounded, read-only census re-ingests a declared sample and reports, per document, whether the
  current toolchain reproduces the persisted `source_ir.json` content (excluding embedded output paths); the
  sample, its selection rule, and every unmeasurable document are stated rather than implied

- ID: `SOURCE-IR-REPRODUCIBILITY.2`
  State: `pending`
  Goal: make reviewed anchors survive segmentation change
  Acceptance: a reviewed source region resolves by content identity rather than ordinal position, the reviewed
  dataset re-derives without changing any reviewed fact, and a RED control proves an anchor that no longer
  matches its excerpt still fails closed
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.1`

- ID: `SOURCE-IR-REPRODUCIBILITY.3`
  State: `pending`
  Goal: close the chain-currency blind spot at the ingest boundary
  Acceptance: the doctrine states explicitly that `SourceIR` is outside the replay oracle and either extends the
  oracle to the ingest boundary for documents whose source is present, or registers the exclusion as a declared,
  measured unmeasurability with its own census
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.1`

## Open Questions

- What actually changed between the `2026-08-10` bundle and now, given that input, revision, toolchain, and
  models are all excluded? Candidates not yet tested: host-level parallelism or thread-count sensitivity in the
  layout model, and a Docling cache or scratch directory whose state affects segmentation.
- Should ingest record a reproducibility fingerprint (toolchain versions plus model blob digests) in the
  normalized bundle metadata, so a future drift is attributable instead of archaeological?

## Blockers

- None. `.1` is runnable; it needs only a declared sample and bounded scratch.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-27` | `.0` reproduction and exclusion | the reviewed Cortex-A76 prose moves `elem_00219` to `elem_00230` (249 to 260 content elements, identical table/visual/page counts); input digest, production revision, Docling and model versions, batching, and run-to-run noise are each excluded by direct measurement; `check_chain_currency.sh` is shown to start from the persisted `source_ir.json`, so the ingest boundary is outside its oracle |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.0` | `SPEC-TO-INTENT-ALIGNMENT.8d — publish the reviewed population residual closure` | route the SourceIR reproducibility gap and reviewed-anchor fragility surfaced by the `.8d` replay into an owning tree |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A measured
census updates the reproduction section rather than appending a second one.
