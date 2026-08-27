# SOURCE-IR-REPRODUCIBILITY: make SourceIR ingest reproducible, and gate it

## Metadata

- Tree ID: `SOURCE-IR-REPRODUCIBILITY`
- Status: `active` (`.0`–`.1` measured; `.2`–`.5` pending)
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

## Reproduction and measurement (`.0` localization, `.1` census; `2026-08-27`, both complete)

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

**The change is in Docling's own output, not this repository's code.** The decisive comparison is at the
ingest boundary: the persisted `2026-08-10` promoted markdown is 168,210 bytes / 1,302 lines; a fresh run of the
same PDF through the same installed Docling produces 168,359 bytes / 1,324 lines. The 22 added lines are text
fragments recovered from *inside a block diagram* (`IN ORDER`, `Rename,`/`Dispatch`, `Branch`,
`Integer Single-Cycle O`/`1`, `FP/ASIMD O`) — current Docling extracts text from a figure region the earlier run
left alone, and those 22 lines become the 11 additional content elements. SpecForge's code never sees a
difference it could have caused.

Every remaining suspect is excluded by measurement:

- **Input.** The PDF is byte-identical throughout: SHA-256 `8358c5ae…3a22`, unchanged on disk since
  `2026-04-02`, and the replay orchestrator asserts source digest equality before ingesting.
- **Revision.** `483e525d` and `3d04bde0` — the commit *before* the only revision in range that touched
  `crates/specforge/src/ir/source/docling_backend.rs` — both produce 260. Between the published `.7c.ii`
  revision and `483e525d` the only change under `crates/specforge/src/ir/source*` is additive (a new record
  type), and no file outside `crates/`, `docs/`, and `doctrine/` changed at all.
- **Toolchain.** Docling `2.84.0`, `docling-core 2.78.0`, `docling-ibm-models 3.13.2`, `docling-parse 5.11.0`,
  unchanged on disk since `2026-08-08`; no model blob under `.cache/huggingface` is newer than `2026-08-09`.
- **Batching.** Batched (`SPECFORGE_INGEST_BATCH_PAGES=16`) and unbatched ingest both produce 260.
- **Compute device.** The CPU default landed `2026-06-02`, over two months before the persisted bundle, so both
  runs take the same device path.
- **Run-to-run noise.** Two back-to-back runs are byte-identical after normalizing the replay root, so ingest is
  stable *within* a session. Artifact digests always differ across roots because `SourceIR` embeds its own
  output paths; that is expected and is not drift.

What remains is that the same PDF, the same installed Docling, and the same model blobs produced one markdown
rendering when the persisted bundle was built and a different one now. `SourceIR` ingest is therefore not
reproducible across time in this environment, the instability originates upstream of SpecForge, and no doctrine
can currently observe it. Why Docling's figure handling changed without a version change is the open question
`.1` must answer; the normalized bundle records no device, thread, or model fingerprint that would settle it
after the fact.

### Population census (`.1`)

`scripts/measure_source_ir_reproducibility.py` partitions every persisted
`generated/source_ir/<key>/source_ir.json` into exactly one stratum, re-ingests each measurable document
through the isolated `source_to_intent_replay` example into `.project-data/tmp` scratch, and compares. It
writes nothing under `generated/` and states every unmeasurable document with its reason.

| Stratum | Documents |
| --- | ---: |
| live (schema 3), measured | 24 |
| live, unmeasurable | 0 |
| legacy (pre-schema-3), structurally unmeasurable | 54 |
| **Frame** | **78** |

The live stratum is exactly the membership of `doctrine/chain_currency/retained_bundles.json`, so the census
frame is precisely the population whose chains report current today. Every recorded source resolved on the
repository volume, so the whole live population is measured with no sampling.

**Eleven of twenty-four reproduce exactly; thirteen do not.** The thirteen hold 11,379 of the live
population's 22,088 persisted content elements. Ingest adds 1,804 elements across them — all but two are
`body_text`, and the text is unambiguously figure interior (`Core`, `External Debugger +`, `Referenced to`,
`Ideal Clock`, `requency (GHZ)`, `MSb LSD`). Of 31 dropped elements, 28 are re-segmentation with the text
still present and exactly three are content the current toolchain emits nowhere. No collection other than
`content_elements` changes cardinality anywhere, and `proof_ledger.ruleset_sha256` is identical for all 24.

**The decisive result is the captions.** Caption bindings on tables and visual assets fall from 1,191 to
1,152; seven documents lose bindings and one gains two. A captured caption becomes a loose interior text
element — `structured_tables[1].caption_text` is `Figure 5-1: External debugger and core handshake sequence`
persisted and `null` replayed. Re-ingesting is therefore not a refresh but a trade: newer figure-interior
text for 39 caption bindings and three paragraphs. That inverts the obvious remedy and is why `.5` exists.

The result is not run-to-run noise. The largest proportional drift was ingested twice more into separate
roots; all three replays produce the same 347 elements and the same `+81 / -0`, and differ from each other
only in `proof_ledger`. Comparing two same-input replays isolates that difference to each claim's `scope`
and `conclusion_sha256`, with the ruleset digest and all 448 claim addresses equal — the capture digest
hashes a premise map that quotes the artifact's own output paths, so two replays into different roots
cannot share a ledger. Excluding the proof surface from the byte comparison is structurally necessary, not
merely conventional.

Full result, method, controls, and per-document table:
[`docs/research/source-ir-reproducibility-census.md`](../research/source-ir-reproducibility-census.md).

### Why this matters beyond one cell

1. Every persisted `generated/source_ir/*` artifact may be stale in a way no gate detects, and every downstream
   stage inherits that staleness while `CHAIN-CURRENCY` reports current.
2. Reviewed fixtures anchor cells on ordinal ids, so any segmentation change moves anchors and converts an
   ingest question into a scoring failure in an unrelated program.

## Planned children

- ID: `SOURCE-IR-REPRODUCIBILITY.1`
  State: `done` (`2026-08-27`)
  Goal: measure the standing SourceIR drift across the persisted corpus
  Acceptance: a bounded, read-only census re-ingests a declared sample and reports, per document, whether the
  current toolchain reproduces the persisted `source_ir.json` content (excluding embedded output paths); the
  sample, its selection rule, and every unmeasurable document are stated rather than implied
  Evidence: `scripts/measure_source_ir_reproducibility.py` partitions all 78 persisted artifacts into
  24 live-measured / 0 live-unmeasurable / 54 legacy-unmeasurable, re-ingests the whole live stratum with no
  sampling inside it, and reports 11 reproduced / 13 drifted. Read-only: nothing under `generated/` is written
  or removed, and the producer refuses to run with a modified `crates/` tree. Every exclusion is declared and
  bounded by a control — `--self-test` is 14/14, observed RED at 12/13 when the migration-note exemption is
  widened to notes as a class and at 13/14 when re-segmentation is counted as content loss

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

- ID: `SOURCE-IR-REPRODUCIBILITY.4`
  State: `pending`
  Goal: make a future ingest drift attributable instead of archaeological
  Acceptance: the normalized bundle records a reproducibility fingerprint — converter and sibling package
  versions, resolved model snapshot revisions, compute device, and batch regime — a re-ingest whose fingerprint
  differs is distinguishable from one whose fingerprint matches, and a RED control proves a changed fingerprint
  is observed rather than absorbed
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.1`

- ID: `SOURCE-IR-REPRODUCIBILITY.5`
  State: `pending`
  Goal: decide what to do about the thirteen documents whose persisted SourceIR no longer reproduces
  Acceptance: a decision record weighs re-ingest against retention on measured evidence, not on the assumption
  that newer is better — re-ingesting costs 39 caption bindings and three paragraphs and moves every published
  measurement pinned to the current artifacts, while retaining leaves a chain that reports current on an
  artifact the toolchain no longer produces; whichever is chosen is executed under an explicit re-measurement
  plan for every affected published claim
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.1`

## Open Questions

- What changed inside Docling between the `2026-08-09`–`2026-08-11` bundles and now? `.1` strengthens the
  exclusion — one snapshot revision per model, both fetched `2026-04-01`, refs resolving to them, and an install
  unchanged since before every persisted bundle — and adds the shape the mechanism must explain: figure-interior
  text appears, and a caption that was bound to its table or figure becomes a loose interior element. Candidates
  not yet tested: host-level parallelism or thread-count sensitivity in the layout model, and a Docling cache or
  scratch directory whose state affects figure handling.
- Why does drift split the population so cleanly? Eleven documents reproduce exactly, including the largest
  measured document (Arm Debug, 6,784 elements / 400 pages), while thirteen change. Whatever changed is
  content-dependent rather than uniform, and identifying the trigger would narrow the mechanism faster than
  another whole-population re-measurement.
- `.4` now owns the fingerprint question that was recorded here.

## Blockers

- None. `.2`–`.5` are all runnable; `.5` needs a decision, not a capability.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-27` | `.1` population census | 78 persisted artifacts partition into 24 live-measured / 0 live-unmeasurable / 54 legacy-unmeasurable; the live stratum equals the chain-currency retained-bundle declaration exactly; 11 reproduce and 13 drift, adding 1,804 content elements (1,802 `body_text`, all figure-interior text) and dropping 31, of which 28 are re-segmentation and three are content emitted nowhere; caption bindings fall 1,191 to 1,152; no collection but `content_elements` changes cardinality and `proof_ledger.ruleset_sha256` is identical for all 24; two further ingests of the largest proportional drift reproduce it exactly, and a two-replay comparison isolates the cross-root ledger difference to `scope`/`conclusion_sha256` with all 448 addresses equal; producer self-test 14/14 with two observed RED perturbations |
| `2026-08-27` | `.0` reproduction and exclusion | the reviewed Cortex-A76 prose moves `elem_00219` to `elem_00230` (249 to 260 content elements, identical table/visual/page counts); the drift is localized to Docling's own promoted markdown (168,210 to 168,359 bytes; 22 added lines, all text from inside a block diagram) with input digest, production revision, installed Docling and model versions, batching, device, and run-to-run noise each excluded by direct measurement; `check_chain_currency.sh` is shown to start from the persisted `source_ir.json`, so the ingest boundary is outside its oracle |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.0` | `SPEC-TO-INTENT-ALIGNMENT.8d — publish the reviewed population residual closure` | route the SourceIR reproducibility gap and reviewed-anchor fragility surfaced by the `.8d` replay into an owning tree |
| `.1` | `SOURCE-IR-REPRODUCIBILITY.1 — census the standing SourceIR drift across the live corpus` | measure the whole live population, publish 11 reproduced / 13 drifted with the caption-binding loss, correct the book's reproducibility claim, and open `.4`/`.5` |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A measured
census updates the reproduction section rather than appending a second one.
