# SourceIR reproducibility census

Owning leaf: `SOURCE-IR-REPRODUCIBILITY.1` (MEASURE).

## The question

`scripts/check_chain_currency.sh` proves that every persisted corpus artifact is what the current
binary reproduces from its **persisted input**. EvidenceIR replays from the persisted
`source_ir.json`, SemanticIR from the persisted `evidence_ir.json`, and so on. The oracle therefore
starts one stage downstream of ingest and never re-runs it: a 24/24 current chain is a true
statement about EvidenceIR through the ISF adapter and says nothing about the artifact all four
stages descend from.

`SOURCE-IR-REPRODUCIBILITY.0` showed that this blind spot is not theoretical. Replaying the Cortex-A76
software optimization guide moved its reviewed prose from `elem_00219` to `elem_00230`, and the change
was localized to Docling's own promoted markdown rather than to anything in this repository. What `.0`
could not say is **how much** of the persisted corpus stands on a SourceIR the current toolchain no
longer produces. That is this census.

## Frame, strata, and selection rule

The frame is every persisted `generated/source_ir/<key>/source_ir.json`. Each document lands in
exactly one stratum, and the rule is mechanical rather than editorial:

- **live** — the artifact is SourceIR schema 3, the schema the current toolchain emits, carrying a
  verified proof ledger.
- **legacy** — the artifact predates schema 3. No ingest can reproduce it, because the current
  toolchain does not emit that artifact at all. The honest disposition is *unmeasurable*, not
  *failing*.
- Within the live stratum, a document is **measured** when its exact recorded source resolves on the
  repository volume with the byte count the persisted artifact records, and **unmeasurable**
  otherwise. There is no discretionary selection inside the measured set: everything resolvable is
  measured.

The live stratum coincides exactly with the chain-currency retained-bundle declaration in
`doctrine/chain_currency/retained_bundles.json` — same members, no exception either way. The census
frame is therefore precisely the population whose chains report current today, which is what makes
its result actionable rather than academic.

Repository sources are used in place. A source recorded at a host-local path is supplied through an
untracked runtime map below `.project-data/tmp`, the same protocol
`scripts/replay_source_to_intent_population.py` uses: the authority must be on the repository
volume, its basename must equal the portable id, and the census stages a verified copy into its own
scratch root before ingesting. No machine-specific path enters the tracked tree, and running without
a map simply reports those documents unmeasurable.

## What "reproduces" means here

Each measured document is re-ingested through the isolated `source_to_intent_replay` example into a
fresh `.project-data/tmp` root, and the replayed `source_ir.json` is compared with the persisted one.
Nothing under `generated/` is written or removed; the census is read-only with respect to both the
tracked tree and the corpus.

Three normalizations, each declared and each exactly as wide as it claims:

1. **Root rewriting.** `SourceIR` embeds its own output paths in `artifact_layout`,
   `normalization_plan`, and every page and visual record, so two replays into differently named
   roots differ everywhere by construction. The replay root is rewritten to the persisted root before
   parsing. That is the root, not drift.
2. **Post-build validation.** `validation_reports`, `proof_context`, and `proof_ledger` are excluded
   — the same three sections `check_chain_currency.sh` excludes, for the same reason: `specforge
   validate` back-annotates them after the stage ran, and the SourceIR capture digest is the hash of
   the whole premise map, so one extra premise re-scopes every claim. Excluding them costs no
   authority: every premise mirrors an artifact field that is compared directly, and the ruleset
   digest, the premise-key set, and the claim-address set are re-checked separately.
3. **The legacy-migration note.** Every live artifact carries one note emitted by
   `neutralize_legacy_source_classifications`, recording that it was migrated from schema 1 rather
   than freshly ingested. It describes how the persisted artifact came to be; a fresh ingest has no
   reason to carry it. Exactly that one note is removed, and its removal is reported per document.

The input's location is excluded (`requested_path`, `canonical_path`, `path_origin`) because it is
equal by construction for a repository source and necessarily different for an external source staged
into the repository. Input identity is carried instead by `source.size_bytes`, which is compared, and
by the source SHA-256, which is measured and reported. Everything else — `content_elements`,
`structured_tables`, `page_artifacts`, `visual_assets`, `document_sections`, `document_profile`,
`artifact_layout`, `document_identity`, `residual_decisions` — is compared exactly.

## The controls

"Reproduced" must not be the answer a broken comparator gives by default, so the producer carries
thirteen controlled cases under `--self-test`. They prove that a changed element text is reported at
its exact ordinal, that an inserted element moves both the count and the first divergence, that a
re-classified table drifts without contaminating a neighbouring section, that a lost element is never
classified as additive, that changed input bytes drift, and that each declared exclusion is exactly as
wide as stated: the migration note is exempt but notes as a class are not, an externally staged source
path is exempt but a changed ruleset digest is still surfaced on the proof surface.

The controls are observed going RED. Widening the note exemption from the single migration note to
notes as a class — a plausible simplification of `strip_legacy_migration_note` — takes the suite to
12/13 with `note handling must exempt the migration note only, never notes as a class`.

## Result

Measured at production revision `085582c0` with `crates/` unmodified, Docling `2.84.0`
(`docling-core 2.78.0`, `docling-ibm-models 3.13.2`, `docling-parse 5.11.0`), one Hugging Face
snapshot revision per model.

| Stratum | Documents | Disposition |
| --- | ---: | --- |
| live, measured | 24 | re-ingested and compared |
| live, unmeasurable | 0 | every recorded source resolved on the repository volume |
| legacy (pre-schema-3) | 54 | structurally unmeasurable; the current toolchain emits no such artifact |
| **Frame** | **78** | every persisted `generated/source_ir/*` artifact |

**Eleven of the twenty-four live documents reproduce exactly. Thirteen do not.** Those thirteen hold
11,379 of the live population's 22,088 persisted content elements, so slightly over half the persisted
SourceIR content stands on an ingest the current toolchain no longer reproduces — while all twenty-four
chains report current at every stage the chain-currency oracle can see.

| Document | Elements persisted → replayed | Added | Removed | Sections that differ |
| --- | ---: | ---: | ---: | --- |
| `102196_0100_01_2022_05_05_aarch64_external_debug_guide` | 266 → 347 | 81 | 0 | content elements, document profile, document sections, structured tables, visual assets |
| `102520_0101_01_2025_09_15_introducing_coresight_debug_and_trace` | 250 → 333 | 89 | 6 | content elements, document profile, document sections, visual assets |
| `opencapi_25gbps_phy_signaling_spec_1_0` | 224 → 231 | 8 | 1 | content elements, document profile, document sections, visual assets |
| `opencapi_3_0_certified_definition_v1_1` | 151 → 155 | 4 | 0 | content elements, document profile, document sections |
| `opencapi_3_0_ready_definition_v1_1` | 169 → 171 | 2 | 0 | content elements, document profile, document sections |
| `opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020` | 318 → 376 | 59 | 1 | content elements, document profile, document sections, visual assets |
| `opencapi_4_0_32gbps_phy_mech_spec_v10_17mar2021` | 212 → 262 | 52 | 2 | content elements, document profile, document sections |
| `opencapi_discovery_configuration_v201` | 172 → 181 | 9 | 0 | content elements, document profile, document sections, visual assets |
| `pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide` | 249 → 260 | 11 | 0 | content elements, document profile, document sections |
| `usb4_connection_manager_guide_v2_0_2025_11` | 1,313 → 1,509 | 200 | 4 | content elements, document profile, document sections, visual assets |
| `usb4_inter_domain_service_specification_v2_0_2025_11` | 603 → 615 | 12 | 0 | content elements, document profile, document sections |
| `usb_3_2_revision_1_0_2017_09` | 5,830 → 6,586 | 771 | 15 | content elements, document profile, document sections, structured tables, visual assets |
| `wbspec_b4_wishbone_b4_specification` | 1,622 → 2,126 | 506 | 2 | content elements, document profile, document sections, visual assets |
| `109242_0100_01_2023_09_04_arm_smmu_software_guide` | 600 → 600 | 0 | 0 | — exact |
| `198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide` | 430 → 430 | 0 | 0 | — exact |
| `den0068_2018_07_23_coresight_base_system_architecture` | 404 → 404 | 0 | 0 | — exact |
| `ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification` | 6,784 → 6,784 | 0 | 0 | — exact |
| `opencapi_25gbps_phy_mechanical_spec_v10` | 760 → 760 | 0 | 0 | — exact |
| `opencapi_3_0_certified_test_resources_engineering_note_v1_0` | 173 → 173 | 0 | 0 | — exact |
| `opencapi_3_0_ready_test_resources_engineering_note_v1_0` | 105 → 105 | 0 | 0 | — exact |
| `opencapi_afu_address_space_usage` | 118 → 118 | 0 | 0 | — exact |
| `opencapi_data_link_layer_v20_09jul2020` | 527 → 527 | 0 | 0 | — exact |
| `um10204_rev7_0_2021_i2c_bus_specification` | 693 → 693 | 0 | 0 | — exact |
| `um11732_v3_2022_02_17_i2s_bus_specification` | 115 → 115 | 0 | 0 | — exact |

The Cortex-A76 row is `SOURCE-IR-REPRODUCIBILITY.0` reproduced independently: 249 → 260, the same
eleven elements, with the same figure-interior samples (`IN ORDER`, `Rename,`).

## The shape of the drift

Across the thirteen drifted documents, ingest adds 1,804 content elements and drops 31. Every added
element but two is `body_text`; the exceptions are one `section_header` and one `caption`. The added
text is unambiguously **figure interior** — `Core`, `External Debugger +`, `APB`, `Referenced to`,
`Ideal Clock`, `requency (GHZ)` (a truncated axis label), `LE`/`BE`, `MSb LSD`, `OpenCores`. Current
Docling reads text out of diagram and plot regions that the earlier run left alone.

Of the 31 dropped elements, 28 are **re-segmentation**: the text still appears in the replayed
element stream, split or merged differently. Exactly **three** are content the current toolchain does
not emit anywhere — one paragraph each in the USB4 Connection Manager guide, USB 3.2, and Wishbone.
That distinction matters, and it is what separates "ingest rearranged the document" from "ingest lost
part of it".

No collection other than `content_elements` changes cardinality anywhere: `structured_tables`,
`page_artifacts`, `visual_assets`, and `document_sections` keep their exact counts in all twenty-four
documents. `proof_ledger.ruleset_sha256` is identical across all twenty-four, so the proof rules are
not part of this.

**But the captions are not stable, and they move the wrong way.** Caption bindings on tables and
visual assets fall from 1,191 to 1,152 across the live population. Seven documents lose bindings and
one gains two; the Arm external-debug guide alone drops from eight to five, USB 3.2 from 482 to 458.
A concrete instance: `structured_tables[1].caption_text` is
`Figure 5-1: External debugger and core handshake sequence` in the persisted artifact and `null` in
the replay.

What is lost is the **binding**, not the text, and the mechanism is measured on both sides. SpecForge
computes no captions: `docling_backend.rs` calls Docling's own `element.caption_text(doc)`, which
resolves the item's `captions` list of references into the document's `texts`, and `normalize_text`
maps the empty string to `None` — so an empty `captions` list *is* the `null`.

Comparing the persisted and re-ingested Docling documents for the Arm external-debug guide, the
layout model is not mislabelling anything: both runs label the same seven texts `caption`, none lost
and none gained. The **assignment** is what breaks — items carrying a caption reference fall from
seven to five, with `pictures/4` losing `Figure 3-1: Debug state entry and exit` and `tables/1`
losing `Figure 5-1: External debugger and core handshake sequence`, while both captions still exist
and are still labelled `caption`.

The trigger sits in the same comparison: `texts` rises 373 → 469 and the entire +96 lands in
`label: text`, the figure-interior fragments. Docling assigns captions by proximity and containment,
so extra text regions detected around a figure can displace that assignment.

**The census's two results are therefore one cause with two consequences**, not two findings. More
figure-interior regions detected produces both the added elements and the lost bindings — which is
why `.5` may be choosing between coupled outcomes rather than picking the best of each.

This is the census's most consequential result, and it inverts the obvious remedy. Re-ingesting the
corpus would not simply refresh a stale artifact: it would trade thirty-nine caption bindings — which
this pipeline treats as first-class evidence, not decoration — for figure-interior fragments, and lose
three paragraphs. Newer is not better here. `SOURCE-IR-REPRODUCIBILITY.5` owns that decision, and it
is a decision, not a cleanup.

## This is not run-to-run noise

A census that re-measures once cannot tell drift from a coin flip, so the Arm external-debug guide —
the document with the largest proportional drift — was ingested twice more into two further scratch
roots. All three replays produce the same 347 content elements and the same `+81 / -0` difference
against the persisted artifact, and the three artifacts are identical to each other in every section
except `proof_ledger`. Ingest is stable *within* this environment; what is not stable is this
environment against the one that produced the persisted bundles.

That single remaining section also settles why the proof surface is excluded rather than merely
awkward. Comparing the two same-input replays directly, `proof_ledger` agrees on its ruleset digest
and on all 448 claim addresses, and differs **only** in each claim's `scope` and `conclusion_sha256`.
The capture digest hashes the premise map, and the premise map quotes the artifact's own output
paths, so two replays into differently named roots cannot share a ledger no matter how identical
their content is. Root rewriting fixes the path strings; it cannot retro-compute a digest taken over
the original bytes. Excluding the ledger from the byte comparison is therefore structurally
necessary, and the ruleset/address/premise-key checks are what remains available — all three of which
agree everywhere.

## Environment identity at the census boundary

Each suspect is excluded by measurement, not by argument:

- **Toolchain.** The Docling install under `.venv-docling` has not changed since `2026-08-08 20:16`.
  Every persisted bundle was built between `2026-08-09 02:43` and `2026-08-11 13:15` — *after* that
  install. The persisted and replayed runs used the same installed converter.
- **Models.** Each Docling model has exactly one snapshot revision in the repository-local Hugging
  Face cache, both fetched `2026-04-01`, and each `refs` entry resolves to that revision. There is no
  second revision for a later run to have selected.
- **Production revision.** The census refuses to run with a modified `crates/` tree and records the
  revision it measured.
- **Input.** Every source is byte-count-checked against what the persisted artifact recorded, and its
  SHA-256 is measured and reported.
- **Run-to-run noise.** Excluded by the repeat control above.

One suspect is **not** excluded, and it is named here rather than left implicit. Host run conditions —
thread count, parallelism, and machine load — were not controlled between the corpus refresh that
built the persisted bundles and this census, and cannot be recovered after the fact. Every comparison
in this tree is *persisted-then* against *replayed-now*, so the time axis and the run-condition axis
are confounded. If Docling's layout model is sensitive to host parallelism, "not reproducible across
time" would be the wrong name for the same measurements. The repeat control cannot separate them: it
re-ran under identical conditions. The **toolchain** exclusion is also weaker than the others — it
rests on the `.venv-docling` modification time, not on a content digest of the installed packages,
because no digest was recorded when the bundles were built.

What is left is that the same PDF, the same installed Docling, and the same model blobs produced one
result when the persisted bundles were built and a different result now, for thirteen of
twenty-four documents.

## What this census does not say

It does not say *why*. It localizes the change to Docling's figure and caption handling and rules out
input, revision, install, model revision, and run-to-run variation, but the mechanism remains open.
The normalized bundle records no device, thread-count, or model fingerprint, so the question cannot
be answered from the persisted artifacts after the fact — which is itself the argument for
`SOURCE-IR-REPRODUCIBILITY.4`.

It says nothing about the 54 legacy documents. Their artifacts predate schema 3 and the current
toolchain emits no comparable artifact, so their disposition is *unmeasurable*, not *current* and not
*stale*. `CORPUS-CHAIN-CURRENCY` already classifies the same 54 chains as explicitly unmeasurable, so
this census neither improves nor worsens what is known about them.

It is a statement about one environment at one revision. Re-deriving it elsewhere needs the same
external sources, and the producer states every document it could not measure rather than skipping
it.

## What follows

- `SOURCE-IR-REPRODUCIBILITY.2` — content-addressed reviewed anchors. Thirteen documents now have
  moved ordinals, so an anchor pinned to `elem_NNNNN` is a latent scoring failure, not a hypothetical.
- `SOURCE-IR-REPRODUCIBILITY.3` — the chain-currency blind spot. The denominator is now measured: the
  oracle reports twenty-four current chains, and thirteen of them descend from a SourceIR the current
  toolchain does not reproduce.
- `SOURCE-IR-REPRODUCIBILITY.4` — a reproducibility fingerprint in the normalized bundle, so the next
  drift is attributable instead of archaeological.
- `SOURCE-IR-REPRODUCIBILITY.5` — the remediation decision. Re-ingesting is not obviously the repair:
  it costs thirty-nine caption bindings and three paragraphs, and it would move every published
  measurement pinned to the current artifacts.

Reproduce with:

```bash
python3 scripts/measure_source_ir_reproducibility.py --self-test
python3 scripts/measure_source_ir_reproducibility.py \
  --output-root .project-data/tmp/<census-id> --census-id <census-id> \
  --owner SOURCE-IR-REPRODUCIBILITY.1 \
  --external-source-map .project-data/tmp/<untracked-runtime-map>.json --plan-only
```

Dropping `--plan-only` runs the census; `--compare-only` re-derives the comparison from an existing
root's retained replays without re-ingesting.
