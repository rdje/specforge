# SOURCE-IR-REPRODUCIBILITY: make SourceIR ingest reproducible, and gate it

## Metadata

- Tree ID: `SOURCE-IR-REPRODUCIBILITY`
- Status: `active` (`.0`–`.2`, `.5`, `.8`, `.9`, `.11`–`.12` done; `.3`, `.4`, `.6`, `.7`, `.9a`, `.10`, `.13`–`.16` pending)
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

**The chain is not 24/24 today.** Measured `2026-08-28` at HEAD `3833ad10`, before any of this tree's
current work: 0 current / 24 stale at all four stages, every one refusing at
`proof ledger ruleset hash is stale`. That is a seal problem, not a content problem, and `.14` owns it.
Where this file reasons from a current chain it is reasoning from a state that has since lapsed.

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
  State: `done` (`2026-08-28`)
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
  Design (measured `2026-08-28`, before implementing): the obvious rule — "find the element containing the
  reviewed excerpt" — is **worse than today**, and the measurement says so. Against the persisted artifacts it
  resolves 8 of 12 reviewed regions uniquely and leaves **4 ambiguous**, all tables, because the excerpt is
  matched against `json.dumps(record)`: `all values in ns` hits three I2S tables, `Channel requirements` hits
  three OpenCAPI tables, `CPU interface register summary` hits three GIC tables. Failing closed on a third of
  the population would convert four working anchors into scoring failures. Two further measurements fix it:
  most false matches carry the phrase only in a table *body* (a contents or index table that merely mentions
  it) while the reviewed table carries it in its **caption**; and two reviewed anchors are the reverse, with
  their excerpt in the body and nothing in the caption. So the rule is a **precedence over the region's own
  natural-language surface, never its serialization**: prose matches element text; a table or figure matches
  its caption first and, only if that is not decisive, its caption plus cell text; zero or multiple matches at
  the deciding tier fail closed. Two excerpts also have to be **strengthened** — `all values in ns` to
  `Target receiver with data rate of 2.5 MHz` and `CPU interface register summary` to
  `Table 3-6 CPU interface register summary`, each taken from the same reviewed region's own caption, which
  tightens an identity rather than re-pointing an anchor and must be shown to select the same region. Measured
  result: **all 12 resolve uniquely to exactly the region the ordinal selects today** — 10 at the caption/text
  tier and 2 at the caption-plus-body tier. Attribution must use one replay and two projections (frozen builder
  as control, new builder as treatment) so the resolver's effect is not confounded with the ingest drift this
  tree measured.
  Evidence: `source_record` now resolves through `resolve_region`, a precedence over the region's own
  natural-language surface. **Attribution is one replay, two projections of the same artifacts**: the
  control leg (frozen builder) reproduces the published defect exactly at **13/14 exact source regions**,
  the treatment leg reaches **14/14**, and every other global metric is byte-identical between them —
  10/14 disposition, 8/12 modality, 45/45 provenance, 120/120 conservation, 8/16 residual actionability,
  0 fabricated, 0 unexplained drops. **Exactly one cell changes**, and only by dropping
  `source_region_missing_or_ambiguous`; its `actual_keys` moves `[] -> ["elem_00219"]`, the reviewed
  anchor unchanged, and its two other hard failures still stand, so nothing unrelated was papered over.
  No reviewed fact, predicate, or `expected_keys` value moved. Controls: `build_fixture.py --self-test`
  is **11/11** with **six observed RED perturbations** — resolving ambiguity by taking the first match,
  matching a figure against its serialization, dropping the caption tier, making the body tier disjoint
  from the caption tier, admitting an ordinal id into a region surface, and removing the frozen-build
  assertion. Two controls were rewritten after a perturbation left them green: an ambiguous-tier
  fall-through is provably a no-op because a later tier is a superset, and the first figure fixture could
  not distinguish caption matching from serialization matching
  Lockstep discharged: the change moved **eight** pinned surfaces, not the two the fact card names —
  `REPLAY_PROJECTION_SHA256`, `POPULATION_REPLAY_PRODUCTION_REVISION`, `REPLAY_VERTICAL_RESULT_SHA256`,
  `POPULATION_REPLAY_EVIDENCE_SHA256`, the cleanup census, `current_dataset.byte_count`,
  `current_result.byte_count`, and the compiled `exact_source_regions != 13` literal, plus the replay
  authority identity and the published `current_result_snapshot.json`. `.8d`'s record was **not**
  re-stamped: a published record must keep the digest of the tool that produced it, so `.2` publishes its
  own replay `source-ir-repro-2-population-r1` at production revision `5fe81128`, under a distinct
  current-replay dataset id `source-to-intent-reviewed-current-r6` rather than reusing the review-locked
  dataset's own id. Scratch removed exactly: 3,604 files / 1,331,419 KiB with an empty residue census

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
  State: `done` (`2026-08-28`)
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
  Design (measured `2026-08-28`, before implementing): the carrier is a **nested** field on the figure's own
  `visual_assets` record, `interior_texts: Vec<FigureInteriorText>`, not a new top-level `SourceIR`
  collection. A new top-level field must be registered in `SOURCE_RULE_FIELDS`, and
  `SourceIr::validate_proof_context` requires the persisted `proof_context.field_premises` to carry an
  entry for **every** registered field — an artifact written before the field exists carries none, so
  registering one makes all 24 live artifacts fail `load_from_path` with "SourceIR proof context lacks
  field". Nesting under `#[serde(default, skip_serializing_if = "Vec::is_empty")]` is the rule `.9`
  already established: an artifact with no interior text serializes exactly the bytes it serialized
  before, so the capture digest and every persisted proof ledger survive the change. It is also the
  right owner — the text is interior to that figure, and keeping it on the figure's record is what
  structurally prevents the promotion into prose `.10` forbids.
  The membership rule is **the library's own traversal differenced against itself**, never a
  reimplemented predicate: production already iterates `doc.iterate_items()`, and the interior set is
  what `doc.iterate_items(traverse_pictures=True)` additionally yields. The two calls differ only in
  picture traversal, so the difference *is* the picture-interior population by construction, and
  `.11`'s oracle keeps the census's model honest against the same library. Attribution walks the item's
  own `parent` chain to the enclosing `#/pictures/N`, because a list group inside a figure puts its
  list items two levels down.
  Population (`2026-08-28`, all 24 retained converter bundles, read-only): **13,506** non-empty
  figure-interior text items — 13,416 `text`, 52 `caption`, 12 `section_header`, 10 `list_item`,
  9 `footnote`, 6 `checkbox_unselected`, 1 `code`. This is the *persisted-bundle* population and is
  deliberately not the same number as `.5`'s 5,896, which is what a **current re-ingest** of three
  documents discards; the two agree where they overlap, and the probe reproduces `.5`'s published
  per-document persisted figures exactly (I2C 1,372, I2S 255). Every one of the 13,506 attributes to a
  body-layer picture that becomes a `VisualAsset` — **0 orphans**, so the nested carrier loses nothing
  the flat one would have kept. Scope is complete for content: the only non-text nodes blocked inside a
  picture anywhere in the corpus are **10 `groups`**, which are containers carrying no text of their own
  and whose list items are already in the count; **no picture and no table is nested inside a picture**
  in any of the 24 bundles. One interior item has empty normalized text and is excluded by the same rule
  the helper already applies to `content_elements`, which `.12` classified as a correct exclusion.
  Evidence: `VisualAsset` gains `interior_texts: Vec<FigureInteriorText>`
  (`#[serde(default, skip_serializing_if = "Vec::is_empty")]`), and `docling_backend.rs` fills it
  from `collect_figure_interior_texts`, which differences `doc.iterate_items(traverse_pictures=True)`
  against the traversal production already runs. An interior item that resolves to no enclosing
  figure raises rather than being dropped — the one thing this leaf exists to stop.
  **Measured end to end on a real re-ingest** of the 14-page I2S bus specification, through the
  census producer's own `ConverterDocument` / `SourceIrIndex` / `conservation_census`: 464 converter
  text items, **115 reaching a record before and 370 after**; the `picture_interior_not_traversed`
  bucket goes **255 → 0**, and the 94 that remain are entirely `content_layer_excluded` (71
  `page_footer`, 23 `page_header`) — exclusions ingest is right to make. The persisted leg of the
  same comparison reproduces `.5`'s published figures for this document exactly (349 of 464 with no
  record, 255 figure interior). `content_elements` is **115 before and 115 after**, so `.10`'s
  constraint is measured rather than asserted: 255 diagram labels — `TRANSMITTER`, `clock SCK`,
  `word select WS` — were recovered without one word entering the prose stream. 8 of the document's
  27 figures carry the field and the other 19 serialize exactly as before, as do `content_elements`
  (115), `document_sections` (24), `structured_tables` (7), and `page_artifacts` (14).
  Landability control: the change had to be provably invisible to every artifact already on disk,
  because a persisted `proof_ledger` is sealed over the capture premise. Both legs are measured —
  the focused Rust test asserts an empty carrier is byte-indistinguishable from no carrier, and
  `source_proof_migrate` (dry run, no `--write`) re-derives all **24/24** live artifacts from their
  own retained capture and reports `verified` under the new schema. Noted while running it: those
  artifacts already fail `specforge validate` with `proof ledger ruleset hash is stale`, and a
  stash-and-rebuild control confirms that is **pre-existing at HEAD**, not caused here — recorded as
  `.14`.
  Controls: focused Rust test
  `figure_interior_text_reaches_a_carrier_without_disturbing_artifacts_already_on_disk` proving four
  properties together, with **three observed RED perturbations** — dropping `skip_serializing_if`
  (an empty carrier gains a key), dropping `default` (a pre-carrier artifact stops loading), and
  making `replay_source_classifications` clear the carrier (a production artifact carrying interior
  text would fail to verify, silently and only in production). Producer self-test **33 → 37** with
  **four observed RED perturbations** — the carrier never indexed, the carrier matched by its text
  instead of its own ref, the carrier given a constant batch instead of its figure's, and the
  published figure counting carrying figures instead of carried texts. Two of those four first ran
  GREEN and the controls were rewritten until they discriminated: the text-keyed perturbation
  crashed on an index built later in `__init__` (fixed by declaring every index up front), and the
  count control could not tell one figure from one text (fixed by giving the fixture one figure with
  two interior texts).
  What this does **not** do: it cannot repair an artifact already on disk. The 24 live artifacts keep
  their figure-interior gap until they are re-ingested; the census reports
  `carried_figure_interior_texts` beside the `picture_interior_not_traversed` bucket so the two
  states are told apart rather than conflated.

- ID: `SOURCE-IR-REPRODUCIBILITY.9`
  State: `done` (`2026-08-28`)
  Goal: make `source_ref` identify one converter item under bounded-memory ingest
  Acceptance: a batched ingest converts page ranges and writes one converter document per range, and Docling's
  `self_ref` restarts at zero in each range, so the `source_ref` SpecForge records is ambiguous across batches.
  Measured `2026-08-28`: the Arm Debug guide's 6,784 content elements carry only 2,252 distinct `source_ref`
  values with 1,883 used more than once; USB 3.2 is nine batches. Provenance must resolve to exactly one
  converter item — a batch-qualified ref, or an equivalent — and a RED control proves a colliding ref is not
  silently credited to the wrong item. `.7`'s gate depends on this: a conservation check joins converter items
  to `SourceIR` records, and today that join is ambiguous for every batched document
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.5`
  Design (`2026-08-28`): add the missing **coordinate**, do not redefine the existing field.
  `source_ref` is Docling's `self_ref` and is documented as such; overloading it into
  `batch2:#/texts/7` would change the meaning of a value already carried through EvidenceIR,
  SemanticIR, and IntentIR, and would make every persisted artifact disagree with a re-ingest for a
  reason unrelated to ingest. Instead each of the four record kinds that carries `source_ref`
  (`content_elements`, `document_sections`, `structured_tables`, `visual_assets`) gains
  `source_batch: Option<u32>`, which addresses `documents[i]` in the batched raw-backend envelope
  SpecForge already writes. It is emitted **only when the run is actually batched**: a single-pass run
  produces one converter document where `source_ref` is already unique, and writing a constant zero
  there would change every unbatched artifact for no gained identity. `#[serde(default,
  skip_serializing_if = "Option::is_none")]` therefore keeps an unbatched serialization byte-identical
  and lets every artifact already on disk deserialize with `None` rather than defaulting to batch zero,
  which would itself be a claim.
  Population (`2026-08-28`, all 78 persisted artifacts, read-only): **14** carry a bare `source_ref`
  that does not identify one content element; 64 do not. Across those 14, 111,861 content elements
  resolve to only **28,599** distinct refs — 22,467 refs are used more than once, and **83,262** records
  (74.4%) cannot be addressed by a bare ref. The Arm Debug guide reproduces this tree's published figure
  exactly: 6,784 elements, 2,252 distinct refs, 1,883 used more than once.
  Falsification (`2026-08-28`): the competing hypothesis is that the reuse is a producer defect emitting
  duplicate refs rather than batching. It is separated by the retained converter bundles, an independent
  artifact: of the 24 artifacts whose bundle survives, **22 are unambiguous and unbatched and 2 are
  ambiguous and batched** (7 and 9 converter documents), with **zero disagreements in either direction**.
  The remaining 54 have no retained bundle, so their batching is not confirmable from the artifact and is
  not asserted. A correction belongs here too: pooling `content_elements` with `document_sections` makes
  all 78 look ambiguous, because a section header is recorded in both collections under the same ref —
  1,011 of 1,011 for the Arm Debug guide. That is a legitimate shared ref, not a collision, and the
  population above counts `content_elements` alone.
  Evidence: `crates/specforge/src/ir/source.rs` gains the field on four records with one focused test
  (`source_batch_addresses_one_converter_item_without_disturbing_unbatched_artifacts`) proving all three
  properties together — a pre-coordinate record deserializes to `None`, an unbatched record serializes
  without the key, and a coordinate-bearing record round-trips and is distinct from its colliding twin.
  Two observed RED perturbations: removing `skip_serializing_if` makes an unbatched record gain
  `"source_batch":null`, and removing `default` makes a pre-coordinate record fail to deserialize.
  `docling_backend.rs` sets the coordinate from `_IngestAccumulator.batch_ref()`, which returns `None`
  unless `len(page_batches) > 1`. `scripts/measure_ingest_content_loss.py` consumes it: a
  coordinate-bearing record is matched **exclusively** by `batch{n}:{ref}` and never also by the
  `(source_ref, text)` pair, or the fallback would re-admit the collision it fixes. Self-test 32 -> 33
  with **seven observed RED perturbations** — the coordinate record also entering the pair index, the
  exact address never consulted, a boolean accepted as a coordinate, a mixed artifact reported as exact,
  the batch not passed through the production join, the ref-reuse count reported as zero, and the
  addresses not batch-qualified. The conservation census now publishes `source_ref_identity`,
  `converter_refs`, `converter_distinct_refs`, `converter_refs_reused`, and
  `converter_distinct_addresses`, so the ambiguity is reported rather than worked around silently.
  Confirmed on real data after the change landed (`2026-08-28`, `2172ad9e`, producer `--persisted`, which
  needs a clean tree): the acceptance is "provenance must resolve to exactly one converter item", and on
  the converter side the Arm Debug guide's 7-document bundle yields **12,144** text items addressed by
  only **2,486** distinct `self_ref` values — 9,658 reused — and by **12,144 distinct batch-qualified
  addresses**, one per item. The unbatched I2C specification is 2,507 items / 2,507 refs / 0 reused /
  2,507 addresses, so an unbatched run needs no coordinate and gains none. Both report
  `source_ref_identity: ref_text_pair`, correctly: they are artifacts written before the coordinate
  existed, and the consumer says so rather than implying an exactness they cannot support.
  What this does **not** do: it cannot repair an artifact already on disk. The 14 ambiguous artifacts
  stay ambiguous until they are re-ingested; the coordinate is recorded from this revision forward, and
  the consumer states which key each artifact supports rather than treating both as exact.

- ID: `SOURCE-IR-REPRODUCIBILITY.9a`
  State: `pending` (tracking-only)
  Goal: scope the producer's clean-tree guard to the mode it governs
  Acceptance: `measure_ingest_content_loss.py` refuses to run when `git diff --quiet -- crates` fails,
  with the message "production Rust sources must be unmodified while this producer **ingests**". The
  guard is correct for the re-ingest path and is exempted for `--compare-only`, but it also fires for
  `--persisted`, which performs no ingest at all — it reads a persisted artifact and its retained
  converter bundle. Found `2026-08-28` while `.9` needed the standing-ambiguity population and could not
  take it from a working tree that necessarily had `crates/` modified. The measurement was taken by a
  direct read of the artifacts instead, so this blocked nothing, but the guard should state and enforce
  exactly one rule: what the drop model shares with production, and which modes depend on it. Note that
  `--persisted` is not simply guard-free — its drop-reason model mirrors production's traversal — so the
  fix is to scope the guard, not to remove it
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.9`

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
  State: `done` (`2026-08-28`)
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
  Evidence: the defect rate now leads on every current-facing surface — **5,896 of 18,870 (31%) dropped as
  a defect**, with the raw 8,648 (46%) following as decomposed context (2,722 furniture-layer headers and
  footers plus 30 empty formulas are exclusions ingest is right to make). Changed in the research report,
  the book's SourceIR chapter, `LIVE_ACHIEVEMENT_STATUS.md`, and the fact card — whose **title** carried
  the 46% as well. Decision: `CHANGES.md` keeps `.5`'s entry byte-exact and states the correction in `.12`'s
  own entry instead, because that ledger is append-only and rewriting a past slice's record to match a later
  presentation decision is a worse defect than the one being fixed. No measurement changed and no number was
  withdrawn

## Acceptance Checklist (enforced) — `SOURCE-IR-REPRODUCIBILITY.8`

- [x] **REPRODUCE / MEASURE** — a read-only pass over all 24 retained converter bundles finds **13,506**
  non-empty figure-interior text items that reach no `SourceIR` record and earn no residual: 13,416
  `text`, 52 `caption`, 12 `section_header`, 10 `list_item`, 9 `footnote`, 6 `checkbox_unselected`,
  1 `code`. It reproduces `.5`'s published per-document persisted figures exactly (I2C 1,372, I2S 255)
  and is deliberately a different number from `.5`'s 5,896, which is what a *current re-ingest* of
  three documents discards. Scope measured, not assumed: all 13,506 attribute to a body-layer picture
  that becomes a `VisualAsset` (**0 orphans**), the only non-text nodes blocked inside a figure anywhere
  in the corpus are 10 `groups` carrying no text of their own, and **no picture and no table is nested
  inside a picture** in any of the 24 bundles.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source/docling_backend.rs` builds every
  record from a single `for element, level in doc.iterate_items()` inside `process_converted_document`.
  docling-core's default is `traverse_pictures=False`, so `iterate_items` skips every child of a
  `PictureItem` except the refs in that picture's own `captions` list, and the skip is at the boundary —
  every descendant of a blocked child goes with it, which is why a list group inside a figure takes its
  list items down too. There was no second traversal, no residual, and no counter: the items simply had
  no path into the artifact. `.11`'s oracle already confirmed this mechanism against docling-core itself
  across all 24 bundles with 0 disagreements.
- [x] **ADDRESSED (verified)** — `VisualAsset` gains `interior_texts`, filled by
  `collect_figure_interior_texts`, which differences `doc.iterate_items(traverse_pictures=True)` against
  the traversal production already runs, so the membership rule is the library's own rather than a
  reimplementation of it. Measured on a real re-ingest of the I2S bus specification through the census
  producer's own `ConverterDocument` / `SourceIrIndex` / `conservation_census`: 464 converter text items,
  **115 reaching a record before → 370 after**, `picture_interior_not_traversed` **255 → 0**, and the 94
  that remain entirely `content_layer_excluded` (71 `page_footer`, 23 `page_header`). `content_elements`
  is **115 → 115**, so `.10`'s constraint is measured rather than asserted — 255 diagram labels
  (`TRANSMITTER`, `clock SCK`, `word select WS`) recovered with no word entering the prose stream. 8 of
  27 figures carry the field; `document_sections` (24), `structured_tables` (7), and `page_artifacts`
  (14) are unchanged. Running the same probe under the persisted artifact reproduces the pre-change
  state exactly, so the before/after is one comparison rather than two readings.
- [x] **NO REGRESSION** — `cargo test --workspace` **470 / 168 / 1,371 / 4 / 5 passed, 0 failed, 9
  ignored** (`specforge` lib 470, conformance 168, core 1,371 — was 1,370 — plus 4 integration and 5
  doctests);
  `cargo clippy --workspace --all-targets -- -D warnings` clean;
  `measure_ingest_content_loss.py --self-test` **37/37** (was 33/33) with four observed RED
  perturbations; the focused Rust test observed RED three times; `scripts/check_doctrines.sh`
  **9/9 executed gate-tier doctrines PASS** (CHAIN-CURRENCY deferred as CI-tier); `mdbook build`
  exit 0. `cargo fmt --all --check` reports every file this slice owns as formatted and fails only on
  two it does not — `src/ir/source_to_intent_eval.rs` and `src/test_support/trajectory_snapshot.rs`,
  both unmodified here and already failing at HEAD. They are left byte-identical rather than folded
  into an unrelated leaf, and they block `scripts/run_ci.sh` until a leaf owns them. The landability leg is measured, not argued: `source_proof_migrate` (dry run)
  re-derives all **24/24** live artifacts from their own retained capture and reports `verified` under
  the new schema, and the Rust test asserts an empty carrier is byte-indistinguishable from no carrier —
  which is what keeps the capture premise every persisted `proof_ledger` was sealed over unchanged.
  Stated rather than deferred: `check_chain_currency.sh` **fails**, 0 current / 24 stale at all four
  stages. It is not this change — measured at HEAD `3833ad10` with none of this work in the tree,
  `specforge validate` is already verified 0/24 and ruleset-stale 24/24, and every chain-currency
  stage enters through that same loader. `.14` owns it. This box does not claim that gate green.
- [x] **GENERICITY (ADR 0006)** — the rule is the converter's own traversal differenced against itself
  plus a parent-chain walk. No document, vendor, or protocol vocabulary participates, no caption or
  label text is matched, and nothing depends on the content of any specification.
- [x] **LOCKSTEP** — the book's SourceIR chapter gains a dedicated `interior_texts` section anchored to
  the measured re-ingest; the research report gains the `.8` section; `[[ingest-drops-figure-interior-text]]`
  is retitled and rewritten from "they disappear" to what now carries them; `.7`'s remaining prerequisite
  is discharged and its Blockers note records what its gate must now distinguish. `.14` is opened for the
  pre-existing out-of-seal corpus this leaf's control surfaced, and `CLAIM-VERIFICATION-ADOPTION.9` for
  the book quantitative census's candidate vocabulary, which this slice's own numbers demonstrated it
  cannot see.

## Acceptance Checklist (enforced) — `SOURCE-IR-REPRODUCIBILITY.9`

- [x] **REPRODUCE / MEASURE** — a direct read of all 78 persisted `generated/source_ir/*/source_ir.json`
  artifacts: **14** carry a bare `source_ref` that does not identify one content element, 64 do not.
  Across those 14, 111,861 content elements resolve to 28,599 distinct refs; 22,467 refs are used more
  than once and 83,262 records (74.4%) cannot be addressed by a bare ref. The Arm Debug guide reproduces
  this tree's published figure exactly — 6,784 / 2,252 / 1,883.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source/docling_backend.rs` records
  `source_ref = getattr(element, "self_ref", None)` inside `process_converted_document`, which runs once
  per converted document. Bounded-memory ingest calls `converter.convert(..., page_range=...)` once per
  page range and Docling's `self_ref` restarts at zero in each, so the recorded ref is unique only within
  its batch. The falsification leg separates this from a duplicate-emitting producer bug: of the 24
  artifacts whose converter bundle is retained, 22 are unambiguous and unbatched and 2 are ambiguous and
  batched, with zero disagreements in either direction.
- [x] **ADDRESSED (verified)** — the four record kinds that carry `source_ref` gain
  `source_batch: Option<u32>`, set from `_IngestAccumulator.batch_ref()` and emitted only when
  `len(page_batches) > 1`. `measure_ingest_content_loss.py` matches a coordinate-bearing record
  **exclusively** by `batch{n}:{ref}`, so the `(source_ref, text)` fallback cannot re-admit the collision:
  its end-to-end control feeds a two-batch fixture where `#/texts/0` carries the identical text in both
  batches and only batch 1 was recorded, and the census resolves it 1 reached / 1 dropped rather than 2
  reached. The conservation census now publishes `source_ref_identity`, `converter_refs`,
  `converter_distinct_refs`, `converter_refs_reused`, and `converter_distinct_addresses`.
- [x] **NO REGRESSION** — `cargo test --workspace --lib` **470 / 168 / 1,370 passed, 0 failed, 9 ignored**;
  `cargo clippy --workspace --all-targets -- -D warnings` clean; `measure_ingest_content_loss.py
  --self-test` **33/33** (was 32/32) with seven observed RED perturbations; the new focused Rust test
  observed RED twice. Unbatched artifacts are unaffected by construction, and the Rust test proves it: a
  record with no coordinate serializes without the key, so a single-pass ingest writes the bytes it wrote
  before this change.
- [x] **GENERICITY (ADR 0006)** — the coordinate is the index of a converter document within one run. No
  document, vendor, or protocol identity participates, and no value depends on the content of any
  specification.
- [x] **LOCKSTEP** — the SourceIR book chapter documents the field and when it appears; this leaf and
  `[[ingest-drops-figure-interior-text]]` carry the corrected population; `.7`'s prerequisite is
  discharged.

## Acceptance Checklist (enforced) — `SOURCE-IR-REPRODUCIBILITY.2`

- [x] **REPRODUCE / MEASURE** — the control leg of the population replay reproduces the published defect
  exactly: `exact_source_regions` **13/14**, with the Cortex-A76 cell scoring
  `source_region: {"actual_keys": [], "expected_keys": ["elem_00219"], "false_negatives": 1}` and
  `hard_failures` carrying `source_region_missing_or_ambiguous`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `build_fixture.py:source_record` selected the region by
  `item["element_id"] == region_id`, an **ordinal position**. `SOURCE-IR-REPRODUCIBILITY.0` measured that
  Docling moved the same prose from `elem_00219` to `elem_00230` (249 → 260 content elements), so the
  anchor addressed a different paragraph and the fixture failed closed. Measured before implementing: the
  naive replacement — match the reviewed excerpt anywhere in the record — resolves only 8 of 12 reviewed
  regions and leaves 4 ambiguous, because `json.dumps(record)` lets any contents or index table that
  mentions the phrase claim the anchor.
- [x] **ADDRESSED (verified)** — `resolve_region` matches the region's own natural-language surface, most
  specific first, and fails closed on zero or multiple matches at the deciding tier. One replay, two
  projections of the same artifacts: **13/14 → 14/14** exact source regions, every other global metric
  byte-identical (10/14 disposition, 8/12 modality, 45/45 provenance, 120/120 conservation, 8/16 residual
  actionability, 0 fabricated, 0 unexplained drops). Exactly one cell changed, `actual_keys`
  `[] -> ["elem_00219"]`, with its two unrelated hard failures still standing.
- [x] **NO REGRESSION** — `cargo test --workspace` **470 / 168 / 1,369 / 4 passed, 0 failed**;
  `cargo clippy --workspace --all-targets -- -D warnings` clean; `build_fixture.py --self-test` **11/11**
  with six observed RED perturbations. No reviewed fact, predicate, or `expected_keys` value moved.
- [x] **GENERICITY (ADR 0006)** — the rule is structural: element text for prose, caption then caption+cells
  for a table, caption for a figure. No document, vendor, or protocol vocabulary participates. The two
  strengthened excerpts are literals taken from the reviewed regions' own captions, in review data rather
  than in production code.
- [x] **LOCKSTEP** — eight pinned surfaces moved together with the published replay authority and the
  result snapshot; `[[reviewed-fixture-projection-digest-lockstep]]` corrected from two pins to eight; the
  book's SourceIR chapter updated where it told readers to pin by content.

- ID: `SOURCE-IR-REPRODUCIBILITY.13`
  State: `pending`
  Goal: make the frozen reviewed fixture re-derivable, or declare that it is not
  Acceptance: found while verifying `.2`, and pre-existing at `5fe81128` — `build_fixture.py --check` cannot
  run at all. It asserts each document's four stage artifacts against the frozen reviewed stage-hash lock and
  stops on the first mismatch. Measured across the population: **all 12 reviewed documents have drifted**, and
  the drift is total from EvidenceIR onward — `source_ir` still matches for **5/12**, while `evidence_ir`,
  `semantic_ir`, and `intent_ir` match for **0/12**. So it is *not* explained by the ingest instability this
  tree measures: five documents whose SourceIR is byte-exact still fail. `CORPUS-CHAIN-CURRENCY` reports
  24/24 current because it replays each stage from its **persisted input**; "does the persisted output still
  match the reviewed frozen lock" is a different question and nothing asks it. The consequence is a
  durability gap: `result_snapshot.json` is tracked and can be compared, but its inputs no longer exist
  anywhere, so nothing can verify it is what those artifacts project. Either the frozen lock is re-anchored
  to artifacts that exist — which must be a published, attributed re-derivation and not a re-stamp — or the
  un-re-derivability is registered as a declared, measured unmeasurability with its own census, exactly as
  `.3` requires for the ingest boundary
  Prerequisite: none

- ID: `SOURCE-IR-REPRODUCIBILITY.14`
  State: `pending`
  Goal: stop the persisted corpus standing out of seal with nothing that says so
  Acceptance: found by `.8`'s landability control, and **pre-existing at HEAD** — a stash-and-rebuild
  at `3833ad10` reproduces it with none of `.8`'s changes in the tree. Every one of the 24 live
  `generated/source_ir/*/source_ir.json` artifacts fails `specforge validate` with
  `SourceIR proof verification failed: proof ledger ruleset hash is stale`, so none of them can be
  loaded through `SourceIr::load_from_path`, and `source_proof_migrate --write` is the tool that
  re-seals.
  **Mechanism corrected (`2026-08-28`):** an earlier draft of this leaf said a rule registration
  "carries `production_semantic_implementation_digest(IrStage::SourceIr)`, so **any** edit to the
  production source module invalidates every persisted ledger". That overstates it and gives the
  design less credit than it earns. The digest is not computed over a file. It is generated at build
  time by `crates/specforge-core/build.rs`, which roots at each stage's rule-registry constructor,
  recursively follows its **stage-local production items** including the verifier it binds, and folds
  in the production trusted-kernel token graph. Rust comments, doc attributes, formatting,
  `cfg(test)` items, and conformance sources are explicitly **not** inputs — it is implementation
  authority, not a whole-file freshness proxy. Measured consequence: of the **54** commits since the
  corpus was sealed on `2026-08-15`, only **5** touched a stage root or `derivation.rs` at all, so
  the blast radius is far narrower than "any edit".
  **Correction (`2026-08-28`, same day):** an earlier draft of this leaf said "no doctrine reports it,
  while `CORPUS-CHAIN-CURRENCY` publishes a current chain". Both halves are wrong and the measurement
  below, taken after that sentence was written, contradicts them. `CHAIN-CURRENCY` reports it loudly,
  refuses to be bypassed, and names the owning remedy (rebuild under `CORPUS-COVERAGE`, ADR 0025
  decision 1); it publishes no current chain at all. What is actually absent is a **gate-tier** signal:
  `CHAIN-CURRENCY` is registered `ci` in `scripts/check_doctrines.sh`, so an ordinary slice never runs
  it and the seal debt is invisible until pre-push. That is a direct consequence of the CI policy that
  deliberately moved full CI to the push boundary, not an unowned hole — so the question this leaf
  carries is a **judgement call for the director**, not a defect to fix unilaterally: is a cheap
  seal-only check worth adding at gate tier, or is discovering the debt at pre-push the intended cost
  of the policy?
  Measured (`2026-08-28`): at HEAD `3833ad10`, with none of `.8`'s changes in the tree,
  `specforge validate` reports **verified 0/24, ruleset-stale 24/24, other failures 0/24** across the
  live stratum. `bash scripts/check_chain_currency.sh` fails closed and loudly — **0 replayed, 0
  current, 24 stale** at *every* one of the four stages (`evidence`, `semantic`, `intent`,
  `isf-adapter`), each with the same `proof ledger ruleset hash is stale`, and it refuses to be
  bypassed. So the oracle *does* enter through the canonical loader it documents; the competing
  hypothesis that it does not is falsified, and no second finding is needed.
  What this does **not** mean: no artifact's *content* is in question. The stale digest is
  `production_semantic_implementation_digest`, over the implementation, so the seal breaks on an
  ordinary source edit while the captured premises stay intact —
  `source_proof_migrate` (dry run) re-derives all 24 from their own retained capture and reports
  `verified`. The cost is that nothing an ordinary slice runs observes the transition: a `source.rs`
  edit un-seals the whole corpus, and the fact only surfaces at a CI-tier gate ordinary slices do not
  run (`DOCTRINE_ENFORCEMENT.md` §4.7). Re-sealing under `--write` is a corpus-wide write and belongs
  to this leaf with its own before/after evidence, not to a slice that happens to touch `source.rs`.
  Practical consequence to record: `scripts/run_ci.sh` runs `check_doctrines.sh --all` **first** under
  `set -euo pipefail`, so this is the first thing that blocks a push — ahead of the formatting drift
  `SIGNOFF-REMEDIATION.3` cleared, which blocked at its third step.
  **Resolved for SourceIR (`2026-08-28`).** `source_proof_migrate --write --retained-manifest` re-sealed
  all 24 live artifacts. The decisive control is the content diff, taken against an 89.0 MB snapshot of
  the 24 `source_ir.json` files captured before the write and compared field by field with
  `proof_context`/`proof_ledger` excluded: **24 proof-only, 0 public content changed, 0 unchanged**. So
  the staleness was a seal, exactly as diagnosed, and nothing about any artifact's content was in
  question. After it, `specforge validate` reports **verified 24/24, failed 0/24**, and the persisted
  ruleset digest is homogeneous across all 24.
  Chain currency moved with it, partly: `evidence` went **0 current / 24 stale -> 24 replayed, 24
  current, 0 stale**. `semantic`, `intent`, and `isf-adapter` remain 0/24 but now fail on a **different**
  error — `EvidenceIR proof verification failed: cumulative proof ledger ruleset hash is stale`. Those
  stages read the *persisted* EvidenceIR, whose own cumulative seal is stale, so the same class of debt
  exists one stage down. No equivalent of `rebuild_from_retained_capture` exists for EvidenceIR,
  SemanticIR, IntentIR, or the adapter — only `SourceIr` has one — so the downstream remedy is a real
  stage-rebuild cascade that writes artifact content, not a proof-only re-seal. That is `.15`
  Cost measured, since it decides where the check belongs: reading the ruleset seal from all 78
  artifacts is **0.18 s**; full canonical verification of all 24 through the product's own loader is
  **7.2 s**; discovering the same fact through `check_chain_currency.sh` took **~20 minutes**. And it went
  undiscovered for **13 days / 54 commits** — sealed `2026-08-15`, with the earliest commit that could
  have broken it on `2026-08-16` (`29dde0ac`), one of only 5 in that window that touched a stage root or
  `derivation.rs`
  Prerequisite: none

- ID: `SOURCE-IR-REPRODUCIBILITY.15`
  State: `pending`
  Goal: carry the seal restoration downstream, where no proof-only path exists
  Acceptance: `.14` re-sealed SourceIR and evidence replay went to 24/24 current, which exposed the same
  debt one stage down: `semantic`, `intent`, and `isf-adapter` fail at `EvidenceIR proof verification
  failed: **cumulative** proof ledger ruleset hash is stale`, because they read the persisted EvidenceIR
  rather than the replayed one. Only `SourceIr` has `rebuild_from_retained_capture`; EvidenceIR,
  SemanticIR, IntentIR, and the adapter have no proof-only re-seal, so the remedy is a **stage-rebuild
  cascade that writes real artifact content**, and it must be treated as such. Two properties have to be
  proved, not assumed: that each rebuilt stage is content-identical to its persisted form — the evidence
  stage already reports 24/24 current, so it is the safe starting point, while `semantic`/`intent`/
  `adapter` currency is **unknown** because their upstream refused to load — and that any content delta
  that does appear is attributed to a named change per ADR 0025 decision 1 rather than absorbed. The
  precedent to respect: the ADR 0025 reconciliation found exactly one real delta across 24 documents
  (`table_0044` becoming `register_map`), so "it will be identical" is a hypothesis, not a given
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.14` (met for SourceIR)

- ID: `SOURCE-IR-REPRODUCIBILITY.16`
  State: `pending`
  Goal: make the seal debt visible at the moment it is created, not at the push boundary
  Acceptance: the director's call on `.14`'s open question, decided `2026-08-28` on the measurements in
  that leaf — 0.18 s to read every seal and 7.2 s to verify all 24 canonically, against ~20 minutes to
  learn the same fact from `check_chain_currency.sh`, and 13 days / 54 commits of actual latency. A
  gate-tier check must report the persisted corpus's seal state per commit. Two design constraints it
  must not violate: it must ask the product's own canonical loader rather than reimplement the digest
  comparison — the `.11` lesson about a second copy of a predicate that can drift — and it must PASS on a
  tree with no persisted corpus, because `generated/` is untracked and a fresh clone has none. The seal is
  homogeneous across all 24 artifacts, so verifying homogeneity plus one sample is decisive and costs
  under a second; verifying all 24 costs 7.2 s and is still gate-affordable. Scope it to the stages whose
  seal is actually current when it lands — SourceIR today — and let `.15` extend it as it re-seals the
  rest, rather than landing a gate that fails on day one
  Prerequisite: `SOURCE-IR-REPRODUCIBILITY.14`

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

- None. `.2`–`.4`, `.6`, `.7`, `.10`, `.13`, and `.14` are all runnable. `.7`'s two prerequisites are now
  discharged — `.8` gives the figure-interior population a carrier and `.9` gives the join an exact key — so
  the gate it owns is next, and it must be built to distinguish an artifact written with the carrier from one
  written before it, because a gate at the persisted corpus's numbers still fails closed everywhere.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-28` | `.14` SourceIR seal restored | `source_proof_migrate --write` re-sealed all 24 live artifacts. Decisive control is the content diff against an 89.0 MB pre-write snapshot of the 24 `source_ir.json` files, compared field by field with `proof_context`/`proof_ledger` excluded: **24 proof-only, 0 public content changed**. After: `specforge validate` **verified 24/24, failed 0/24**, seal homogeneous across all 24. Chain currency `evidence` **0 current / 24 stale -> 24 replayed / 24 current / 0 stale**; `semantic`/`intent`/`isf-adapter` still 0/24 but on a **different** error (persisted EvidenceIR's *cumulative* seal), which is `.15`. Cost that decided `.16`: 0.18 s to read all 78 seals, 7.2 s to verify all 24 canonically, versus **~20 min** for `check_chain_currency.sh` — and **13 days / 54 commits** of actual latency (sealed `2026-08-15`; earliest possible breaker `29dde0ac` `2026-08-16`, one of only 5 commits in that window touching a stage root or `derivation.rs`) |
| `2026-08-28` | `.8` figure-interior carrier | population over all 24 retained converter bundles: **13,506** figure-interior text items reaching no record and earning no residual (13,416 `text`, 52 `caption`, 12 `section_header`, 10 `list_item`, 9 `footnote`, 6 `checkbox_unselected`, 1 `code`), **0 orphans** — every one attributable to a body-layer picture that becomes a `VisualAsset`; the only non-text nodes blocked inside a figure are 10 text-free `groups`, and no picture or table is nested inside a picture anywhere in the corpus. Reproduces `.5`'s persisted per-document figures exactly (I2C 1,372, I2S 255). End-to-end on a real I2S re-ingest: 464 converter items, **115 → 370** reaching a record, `picture_interior_not_traversed` **255 → 0**, the 94 remaining all furniture-layer, and `content_elements` **115 → 115** — the labels are carried without entering the prose stream. Landability measured: `source_proof_migrate` dry run re-derives **24/24** live artifacts as `verified` under the new schema. Producer self-test 33/33 → **37/37** with four observed RED perturbations (two rewritten after first running GREEN); focused Rust test with three observed RED perturbations; `cargo test --workspace` 470 / 172 / 1,376 / 4 passed, 0 failed |
| `2026-08-28` | `.9` batch-qualified provenance | population: 78 persisted artifacts, **14** whose bare `source_ref` does not identify one content element and 64 that do; 111,861 elements over 28,599 distinct refs across the 14, 22,467 refs used more than once, 83,262 records (74.4%) unaddressable — Arm Debug 6,784 / 2,252 / 1,883 reproduces this tree's published figure. Falsified against the retained converter bundles, an independent artifact: 22 unambiguous+unbatched, 2 ambiguous+batched, **0 disagreements in either direction** across the 24 whose bundle survives; the 54 without one are not asserted. A first pass reporting all 78 as ambiguous was wrong and is corrected here — pooling `content_elements` with `document_sections` double-counts a section header, legitimately recorded in both under one ref (1,011 of 1,011 for Arm Debug). Producer self-test 32/32 -> **33/33** with seven observed RED perturbations, plus two on the Rust schema test; `cargo test --workspace --lib` 470 / 168 / 1,370 passed / 0 failed; clippy `-D warnings` clean |
| `2026-08-28` | `.12` published-figure correction | the defect rate leads on every current-facing surface — 5,896 of 18,870 (31%) dropped as a defect, with the raw 8,648 (46%) following as decomposed context; the fact card's title carried the 46% too. No measurement changed; `CHANGES.md` keeps `.5`'s entry byte-exact by decision |
| `2026-08-28` | `.11` traversal oracle | the drop model that carries `.5`'s largest number is confirmed against docling-core's own `iterate_items` on **all 24** persisted artifacts whose converter document was retained, with no sampling: 43,614 converter text items, **22,127 yielded and 22,127 predicted, 0 disagreements** in both directions, per document and per batch, across unbatched and 7-/9-range batched bundles. Every document round-trips through its own `export_to_dict`, so the oracle observes the document ingest traversed rather than a re-derived one. The residue closes: 39 empty `formula` items plus exactly the 22,088 content elements the live population holds, `unexplained` 0 everywhere. `--self-test` 27/27 with six observed RED perturbations; the run exits non-zero unless every document was measured, agreed, and round-tripped |
| `2026-08-28` | `.5` content-loss adjudication | all three elements `.1` reported as emitted nowhere are **retained**: zero persisted tokens missing, 6 / 5 / 35 tokens inserted between them, and every covering converter item carries a `SourceIR` record — so re-ingest content loss is zero and all 31 dropped elements are re-segmentation. Conservation censused in the same run: 18,870 converter text items across the three, 8,648 (46%) reaching no record and earning no residual — 5,896 figure interior (5,875 `text`, 9 `caption`, 8 `footnote`, 4 `section_header`), 2,722 furniture layer, 30 empty formulas, `unexplained` empty. `--persisted` mode shows the same gap without any ingest: I2C 1,372 figure-interior items discarded (39 of them captions) on an artifact `.1` scores as reproducing exactly, while the Arm external-debug guide discards none. A fourth re-ingest of the repository-owned I2S bus specification — which `.1` scores as reproducing exactly, 115 → 115 — reaches 349 of 464 converter items with no record (75%), 255 figure interior. Producer self-test 18/18 with six observed RED perturbations, the first being `.1`'s own whole-string test, which reproduces `.1`'s answer |
| `2026-08-27` | `.1` population census | 78 persisted artifacts partition into 24 live-measured / 0 live-unmeasurable / 54 legacy-unmeasurable; the live stratum equals the chain-currency retained-bundle declaration exactly; 11 reproduce and 13 drift, adding 1,804 content elements (1,802 `body_text`, all figure-interior text) and dropping 31, of which 28 are re-segmentation and three are content emitted nowhere; caption bindings fall 1,191 to 1,152; no collection but `content_elements` changes cardinality and `proof_ledger.ruleset_sha256` is identical for all 24; two further ingests of the largest proportional drift reproduce it exactly, and a two-replay comparison isolates the cross-root ledger difference to `scope`/`conclusion_sha256` with all 448 addresses equal; producer self-test 14/14 with two observed RED perturbations |
| `2026-08-27` | `.0` reproduction and exclusion | the reviewed Cortex-A76 prose moves `elem_00219` to `elem_00230` (249 to 260 content elements, identical table/visual/page counts); the drift is localized to Docling's own promoted markdown (168,210 to 168,359 bytes; 22 added lines, all text from inside a block diagram) with input digest, production revision, installed Docling and model versions, batching, device, and run-to-run noise each excluded by direct measurement; `check_chain_currency.sh` is shown to start from the persisted `source_ir.json`, so the ingest boundary is outside its oracle |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.8` | `SOURCE-IR-REPRODUCIBILITY.8 — give the text inside a figure somewhere to land` | `interior_texts` on the figure that contains it, membership defined by the converter's own traversal differenced against itself; 255 → 0 on a measured re-ingest with `content_elements` unmoved; opens `.14` |
| `.14` | `SOURCE-IR-REPRODUCIBILITY.14 — re-seal the SourceIR corpus and size the check that should have caught it` | proof-only re-seal of all 24, 0 content changed; evidence chain currency restored; `.15`/`.16` opened |
| `.9` | `SOURCE-IR-REPRODUCIBILITY.9 — give provenance the batch coordinate it was missing` | `source_batch` on the four `source_ref`-bearing records, emitted only for a batched run; the consumer keys on it exclusively; the standing ambiguity is measured (14 of 78) and published rather than worked around |
| `.12` | `SOURCE-IR-REPRODUCIBILITY.12 — lead the conservation figure with the defect, not the non-carry rate` | make the published headline the actionable 31%, not the 46% that bundles intended exclusions with it |
| `.11` | `SOURCE-IR-REPRODUCIBILITY.11 — measure the traversal the census had only read` | turn `.5`'s inferred drop mechanism into a re-runnable control: 24/24 documents, 0 disagreements, residue closed, six observed RED perturbations |
| `.5` | `SOURCE-IR-REPRODUCIBILITY.5 — adjudicate the three absent elements, and census what ingest never carries` | withdraw the three-paragraph loss finding, publish the 46% PDF-to-SourceIR conservation gap, and open `.8`/`.9`/`.10` |
| `.0` | `SPEC-TO-INTENT-ALIGNMENT.8d — publish the reviewed population residual closure` | route the SourceIR reproducibility gap and reviewed-anchor fragility surfaced by the `.8d` replay into an owning tree |
| `.1` | `SOURCE-IR-REPRODUCIBILITY.1 — census the standing SourceIR drift across the live corpus` | measure the whole live population, publish 11 reproduced / 13 drifted with the caption-binding loss, correct the book's reproducibility claim, and open `.4`/`.5` |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A measured
census updates the reproduction section rather than appending a second one.
