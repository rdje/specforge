# SOURCE-IR-REPRODUCIBILITY: make SourceIR ingest reproducible, and gate it

## Metadata

- Tree ID: `SOURCE-IR-REPRODUCIBILITY`
- Status: `active` (`.0`–`.1`, `.5`, `.11` measured; `.2`–`.4`, `.6`–`.10`, `.12` pending)
- Roadmap lane: repository durability and portability (sibling of `CORPUS-CHAIN-CURRENCY`)
- Created: `2026-08-27`
- Last updated: `2026-08-28`
- Owner: repo-local workflow

## Goal

Make "the persisted `SourceIR` reflects what the current toolchain produces from the same PDF" a **measured,
gated** property, and stop reviewed fixtures from anchoring on identifiers that a segmentation change silently
invalidates.

**Ingest must not drop information that is present in the source PDF** (owner directive, `2026-08-28`). This is
not a new constraint: the roadmap already treats text, layout, figures, captions, tables, and charts as
first-class evidence rather than decoration, the doctrine already requires an unresolved thing to become an
explicit residual instead of disappearing, and the programme goal is zero remaining defect. A caption binding
that vanishes with no residual and no gate breaches all three. `.1` briefly framed the consequence as a
re-ingest-versus-retain *decision*; that framing is **withdrawn**. Both of its branches are defective — one
retains artifacts the toolchain cannot reproduce, the other drops source content — and neither is a choice
this tree may offer. Note also that the persisted corpus is not the clean baseline it appeared to be: it is
missing the 1,804 figure-interior elements a current ingest recovers. Neither capture is complete, so the
target is to capture both.

`.5` then measured the loss side and found it is **not where this tree thought it was**. None of the three
paragraphs `.1` reported as emitted nowhere is lost: all three are present word for word in the converter's
own document, interrupted by a spliced-in figure fragment. The corpus-wide content loss from a re-ingest is
**zero elements**. What `.5` found instead is a standing, undrifted gap at the same boundary — **46% of the
text items the converter emits reach no `SourceIR` record and earn no residual**, 5,896 of them because
SpecForge never traverses inside a figure — and a faithfulness failure the preservation question does not
cover: the spliced fragments make sentences the specification never wrote. `.8`, `.9`, and `.10` own what
that opened.

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
`Ideal Clock`, `requency (GHZ)`, `MSb LSD`). Of 31 dropped elements, `.1` classified 28 as re-segmentation
with the text still present and three as content the current toolchain emits nowhere; `.5` re-measured those
three token by token and all 31 are re-segmentation — the three "lost" paragraphs are present in full,
interrupted by inserted figure text that the whole-string test could not see. No collection other than
`content_elements` changes cardinality anywhere, and `proof_ledger.ruleset_sha256` is identical for all 24.

**The decisive result is the captions.** Caption bindings on tables and visual assets fall from 1,191 to
1,152; seven documents lose bindings and one gains two — `structured_tables[1].caption_text` is
`Figure 5-1: External debugger and core handshake sequence` persisted and `null` replayed. What is lost is the
**binding, not the text**: in the persisted Arm external-debug artifact all eight bound captions also exist as
standalone `content_elements`, so a caption is carried twice and a re-ingest drops only the association.
The mechanism is measured, not inferred: SpecForge calls Docling's own `element.caption_text(doc)`, and
comparing both Docling documents shows the layout model still labels the same seven texts `caption` while the
count of items *carrying* a caption reference falls seven to five. `texts` rises 373 to 469 in the same
comparison, all of it `label: text` figure-interior fragments, and Docling assigns captions by proximity — so
**the added elements and the lost bindings are one cause, not two**. Re-ingesting is therefore a trade whose
two sides may be coupled, which inverts the obvious remedy and is why `.5` exists.

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
  Constraint (measured `2026-08-27`, before implementation): this is a three-way lockstep, not a one-file edit.
  `build_fixture.py`'s SHA-256 is pinned by `REPLAY_PROJECTION_SHA256` in
  `crates/specforge/src/test_support/trajectory_snapshot.rs` and by `tools.projection.sha256` in
  `crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json`, and the
  snapshot validator rejects any mismatch. The reviewed dataset is a separate, review-locked digest whose
  `source_region` predicate values must not move — so the change belongs in how `source_record` *resolves* a
  region (match the element carrying the reviewed excerpt; fail closed on zero or multiple matches), never in
  the recorded anchor. See [[reviewed-fixture-projection-digest-lockstep]].

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
  State: `done` (`2026-08-28`)
  Goal: settle whether any source content is unrecoverably lost, or only unlinked
  Acceptance: for each of the three content elements the census found absent from a re-ingest — one each in the
  USB4 Connection Manager guide, USB 3.2, and Wishbone — state whether the text is absent from Docling's own
  document or present there and dropped by SpecForge's element construction, with the deciding field named in
  both artifacts. This is the gating measurement for the whole preservation question: SpecForge-dropped is
  repairable here, Docling-absent is an upstream boundary that must be declared rather than assumed away
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.1`
  Evidence: `scripts/measure_ingest_content_loss.py` re-ingests all three, reads the `SourceIR` and the
  converter document from the same run, and aligns each flagged text token by token. **All three are
  `retained`**: zero persisted tokens missing, with 6, 5, and 35 tokens inserted between them
  (`USB4 Host Enhanced SS Host Controller`; `Disabled Stall, Error, or SetFeature`; footnote 14 plus
  `Sampled · 4.0g`), and every covering converter item carries a `SourceIR` record. Neither branch of the
  acceptance applies, because the premise was wrong: `.1`'s whole-string containment test cannot see a
  sentence interrupted by an insertion. Self-test 18/18 with **six observed RED perturbations**, the first
  being `.1`'s own test, which reproduces `.1`'s answer. The same run censuses conservation: 18,870 converter
  text items, 8,648 reaching no `SourceIR` record, with an empty `unexplained` bucket
  Report: [`docs/research/ingest-content-loss-adjudication.md`](../research/ingest-content-loss-adjudication.md)

- ID: `SOURCE-IR-REPRODUCIBILITY.6`
  State: `pending`
  Goal: recover a caption binding the converter emitted but did not attach
  Acceptance: `.1` measured that no caption text is lost — both runs label the same seven texts `caption`, and
  only the figure-to-caption reference disappears — so the information needed to rebuild the link is already in
  the Docling document SpecForge receives. A deterministic, document-neutral rule re-associates an unreferenced
  `caption`-labelled text with an unbound figure or table using page and geometry only, never document
  vocabulary; it fails closed and emits a typed residual when the association is ambiguous rather than guessing;
  and a RED control proves an ambiguous pair is refused instead of bound
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5` (met)
  Scope correction (`.5`, `2026-08-28`): the unattached-caption population is larger than the binding delta `.1`
  could see, because `.1` compares two runs and only observes bindings that *changed*. An unreferenced
  `caption`-labelled text that was never bound in either run is invisible to it and is discarded silently today:
  the persisted I2C specification alone drops 39 of them, and the three re-ingested documents drop nine. `.6`
  must be scoped to unbound captions as a population, not to the seven the drift exposed

- ID: `SOURCE-IR-REPRODUCIBILITY.7`
  State: `pending`
  Goal: gate PDF-to-SourceIR fidelity, the one boundary with no conservation check
  Acceptance: the pipeline already conserves downstream — stage conservation is 120/120 — but nothing measures
  what ingest carries over from the PDF, because there is no upstream artifact to conserve against, which is why
  a lossy ingest passes every green gate. A check compares the converter's own document against the `SourceIR`
  built from it and fails when a text item, caption label, or caption association present in the converter
  output reaches no `SourceIR` record and earns no residual; a RED control proves a dropped item is observed
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5` (met)
  Measured baseline (`.5`, `2026-08-28`): the comparison exists and runs —
  `measure_ingest_content_loss.py`'s conservation census, in both re-ingest and `--persisted` mode. Across the
  three externally sourced re-ingested documents 8,648 of 18,870 converter text items (46%) reach no record and earn no residual,
  every one attributable to a named predicate with an empty `unexplained` bucket. Turning that census into a
  gate is what remains, and it must land after `.8` and `.9`, because a gate at today's numbers would fail
  closed on every document

- ID: `SOURCE-IR-REPRODUCIBILITY.8`
  State: `pending`
  Goal: stop discarding the text the converter found inside a figure
  Acceptance: `.5` measured that SpecForge iterates with `traverse_pictures=False`, so
  `DoclingDocument.iterate_items` skips every child of a `PictureItem` except the refs in that picture's own
  `captions` list, and the skip takes every descendant of the blocked child with it. Across the three
  re-ingested documents that discards 5,896 text items with no record and no residual — 5,875 `text`, 9
  `caption`, 8 `footnote`, 4 `section_header` — and the persisted artifacts carry the same gap, so it is a
  standing defect rather than drift. Figure-interior text must reach a typed carrier or an explicit residual;
  it must **not** be promoted into `content_elements` as prose, because `.10` shows that is how figure labels
  end up spliced into sentences. A RED control proves a discarded interior item is observed
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5`

- ID: `SOURCE-IR-REPRODUCIBILITY.9`
  State: `pending`
  Goal: make `source_ref` identify one converter item under bounded-memory ingest
  Acceptance: a batched ingest converts page ranges and writes one converter document per range, and Docling's
  `self_ref` restarts at zero in each range, so the `source_ref` SpecForge records is ambiguous across batches.
  Measured `2026-08-28`: the Arm Debug guide's 6,784 content elements carry only 2,252 distinct `source_ref`
  values with 1,883 used more than once; USB 3.2 is nine batches. Provenance must resolve to exactly one
  converter item — a batch-qualified ref, or an equivalent — and a RED control proves a colliding ref is not
  silently credited to the wrong item. `.7`'s gate depends on this: a conservation check joins converter items
  to `SourceIR` records, and today that join is ambiguous for every batched document
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5`

- ID: `SOURCE-IR-REPRODUCIBILITY.10`
  State: `pending`
  Goal: observe a sentence the converter corrupted, not merely one it preserved
  Acceptance: `.5` proved preservation and, in the same measurement, disproved faithfulness. All three
  adjudicated paragraphs survive word for word *because* the words are all still there — but each now contains
  an inserted figure fragment, so `SourceIR` carries `… by reading the USB4 Host Enhanced SS Host Controller
  ROUTER_CS_6. Gen T Full Connectivity Support field …`, which is not a sentence the specification contains.
  A conservation gate cannot see this: nothing was lost. Ingest must detect a body-text element assembled from
  disjoint source regions and either keep them separate or mark the element, with the decision made on layout
  geometry rather than document vocabulary; a RED control proves an interleaved element is observed
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5`

- ID: `SOURCE-IR-REPRODUCIBILITY.11`
  State: `done` (`2026-08-28`)
  Goal: make the traversal drop model a re-runnable control instead of a reading of upstream source
  Acceptance: `.5`'s largest published number — 5,896 converter text items discarded because
  `iterate_items(traverse_pictures=False)` never yields them — rests on a drop model that *reimplements* two
  docling-core predicates (`content_layer` membership, and the picture-boundary skip with its descendant
  closure). Those predicates were inferred by reading that library's source, so the claim is one careful
  reading away from being wrong, and every downstream leaf cites it. An `--oracle` mode must load each
  persisted converter bundle through `DoclingDocument.model_validate` in the project's own docling
  environment, call `doc.iterate_items()` with production's arguments, and compare the set of text items the
  library actually yields against the set the model predicts — disagreements reported in both directions,
  never a single agreement count that hides an offsetting pair. The census and the oracle must share **one**
  traversal predicate, so the oracle cannot end up validating a copy of the model. The residue between the
  yielded set and the artifact's own `content_elements` must be attributed to the backend helper's remaining
  filters with an explicit unexplained bucket. A RED control proves a disagreement is reported rather than
  absorbed
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5`
  Evidence: `--oracle` loads each retained converter document through `DoclingDocument.model_validate`
  in `.venv-docling` and calls `doc.iterate_items()` with no arguments, as the embedded backend helper
  does. Frame: **all 24** persisted artifacts whose converter document was retained, no sampling.
  43,614 converter text items; **22,127 yielded by the library, 22,127 predicted by the model, 0
  disagreements** in both directions, per document and per batch — including the two batched bundles
  (Arm Debug 7 ranges, USB 3.2 9) where `self_ref` restarts in each. The census and the oracle share
  one predicate, `Batch.traversal_exclusion`. Two checks close the account rather than leaving it at a
  matching count: every document round-trips through its own `export_to_dict`, so the oracle observes
  the document ingest traversed; and the residue is 39 empty `formula` items plus exactly the 22,088
  content elements the live population holds, with `unexplained` 0 for every document. `--self-test`
  **27/27** with **six observed RED perturbations** — netting the two directions, dropping the batch
  qualifier, accepting a probe covering fewer batches, confirming without the round-trip proof,
  dropping the empty-text attribution, and confirming while a document was skipped. The run exits
  non-zero unless every document in the frame was measured, agreed, and round-tripped
  Report: [`docs/research/ingest-content-loss-adjudication.md`](../research/ingest-content-loss-adjudication.md)

- ID: `SOURCE-IR-REPRODUCIBILITY.12`
  State: `pending`
  Goal: lead the published conservation figure with the defect, not the raw non-carry rate
  Acceptance: `.5` headlines "8,648 of 18,870 converter text items — 46% — reach no `SourceIR` record and
  earn no residual". The statement is true and every surface decomposes it in the next sentence, but the
  headline bundles ~2,722 *intended* exclusions (running headers and footers on the `furniture` layer, which
  ingest is right to skip) and 30 empty formulas together with the 5,896 items that are the actual defect —
  and the headline number is the one that propagates into other surfaces and later leaves. The defect rate
  must lead and the raw non-carry rate must follow as context, on every surface that publishes it:
  `docs/research/ingest-content-loss-adjudication.md`, `docs/book/src/pipeline/sourceir.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, `CHANGES.md`, and the fact card
  [[ingest-drops-figure-interior-text]]. No measurement changes and no number is withdrawn; this is a
  presentation defect in a published claim, not a correction of one
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5`

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
- **Is "across time" even the right name?** Host run conditions — thread count, parallelism, machine load —
  were not controlled between the corpus refresh that built the persisted bundles and the `.1` census, and
  cannot be recovered after the fact. Every comparison in this tree is *persisted-then* against *replayed-now*,
  so the temporal and run-condition axes are confounded, and the repeat control cannot separate them because it
  re-ran under identical conditions. The cheapest discriminator is a forward experiment on one drifted
  document: re-ingest it under two different thread counts at the same revision. If the output moves, the
  drift is environmental rather than temporal, and `.4`'s fingerprint must record host conditions rather than
  only versions. The toolchain exclusion is likewise mtime-based, not a content digest of the installed
  packages, because none was recorded when the bundles were built.

## Blockers

- None. `.2`–`.4`, `.6`–`.10`, and `.12` are all runnable. `.5` is complete, so `.6`–`.10` are unblocked; `.7`'s gate
  should land after `.8` and `.9`, because a conservation gate at today's numbers fails closed everywhere and
  its join is ambiguous for every batched document.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-28` | `.11` traversal oracle | the drop model that carries `.5`'s largest number is confirmed against docling-core's own `iterate_items` on **all 24** persisted artifacts whose converter document was retained, with no sampling: 43,614 converter text items, **22,127 yielded and 22,127 predicted, 0 disagreements** in both directions, per document and per batch, across unbatched and 7-/9-range batched bundles. Every document round-trips through its own `export_to_dict`, so the oracle observes the document ingest traversed rather than a re-derived one. The residue closes: 39 empty `formula` items plus exactly the 22,088 content elements the live population holds, `unexplained` 0 everywhere. `--self-test` 27/27 with six observed RED perturbations; the run exits non-zero unless every document was measured, agreed, and round-tripped |
| `2026-08-28` | `.5` content-loss adjudication | all three elements `.1` reported as emitted nowhere are **retained**: zero persisted tokens missing, 6 / 5 / 35 tokens inserted between them, and every covering converter item carries a `SourceIR` record — so re-ingest content loss is zero and all 31 dropped elements are re-segmentation. Conservation censused in the same run: 18,870 converter text items across the three, 8,648 (46%) reaching no record and earning no residual — 5,896 figure interior (5,875 `text`, 9 `caption`, 8 `footnote`, 4 `section_header`), 2,722 furniture layer, 30 empty formulas, `unexplained` empty. `--persisted` mode shows the same gap without any ingest: I2C 1,372 figure-interior items discarded (39 of them captions) on an artifact `.1` scores as reproducing exactly, while the Arm external-debug guide discards none. A fourth re-ingest of the repository-owned I2S bus specification — which `.1` scores as reproducing exactly, 115 → 115 — reaches 349 of 464 converter items with no record (75%), 255 figure interior. Producer self-test 18/18 with six observed RED perturbations, the first being `.1`'s own whole-string test, which reproduces `.1`'s answer |
| `2026-08-27` | `.1` population census | 78 persisted artifacts partition into 24 live-measured / 0 live-unmeasurable / 54 legacy-unmeasurable; the live stratum equals the chain-currency retained-bundle declaration exactly; 11 reproduce and 13 drift, adding 1,804 content elements (1,802 `body_text`, all figure-interior text) and dropping 31, of which 28 are re-segmentation and three are content emitted nowhere; caption bindings fall 1,191 to 1,152; no collection but `content_elements` changes cardinality and `proof_ledger.ruleset_sha256` is identical for all 24; two further ingests of the largest proportional drift reproduce it exactly, and a two-replay comparison isolates the cross-root ledger difference to `scope`/`conclusion_sha256` with all 448 addresses equal; producer self-test 14/14 with two observed RED perturbations |
| `2026-08-27` | `.0` reproduction and exclusion | the reviewed Cortex-A76 prose moves `elem_00219` to `elem_00230` (249 to 260 content elements, identical table/visual/page counts); the drift is localized to Docling's own promoted markdown (168,210 to 168,359 bytes; 22 added lines, all text from inside a block diagram) with input digest, production revision, installed Docling and model versions, batching, device, and run-to-run noise each excluded by direct measurement; `check_chain_currency.sh` is shown to start from the persisted `source_ir.json`, so the ingest boundary is outside its oracle |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.11` | `SOURCE-IR-REPRODUCIBILITY.11 — measure the traversal the census had only read` | turn `.5`'s inferred drop mechanism into a re-runnable control: 24/24 documents, 0 disagreements, residue closed, six observed RED perturbations |
| `.5` | `SOURCE-IR-REPRODUCIBILITY.5 — adjudicate the three absent elements, and census what ingest never carries` | withdraw the three-paragraph loss finding, publish the 46% PDF-to-SourceIR conservation gap, and open `.8`/`.9`/`.10` |
| `.0` | `SPEC-TO-INTENT-ALIGNMENT.8d — publish the reviewed population residual closure` | route the SourceIR reproducibility gap and reviewed-anchor fragility surfaced by the `.8d` replay into an owning tree |
| `.1` | `SOURCE-IR-REPRODUCIBILITY.1 — census the standing SourceIR drift across the live corpus` | measure the whole live population, publish 11 reproduced / 13 drifted with the caption-binding loss, correct the book's reproducibility claim, and open `.4`/`.5` |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A measured
census updates the reproduction section rather than appending a second one.
