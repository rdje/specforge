# Empty signal catalogs: honest absence vs capture miss

Owning leaf: `SIGNAL-CATALOG-CAPTURE-GAP.1` · measured `2026-08-11` · read-only over the 78 persisted
`SourceIR`/`EvidenceIR`/`SemanticIR` artifacts. No artifact was rebuilt or mutated.

Thirty-three of the 78 corpus documents carry zero interface signal records of any confidence.
`SIGNAL-CATALOG-CAPTURE-GAP.0` established the count and crossed it with each document's
`document_class`. This leaf decides, per document, whether the empty catalog is the honest answer.

## The four probes

Each probe asks whether a **declaration-bearing modality exists at all**. None references a document,
vendor, or protocol name (ADR 0006); all are shape predicates over persisted artifacts.

| Probe | Modality | Predicate |
| --- | --- | --- |
| **A** | table | a `SourceIR` table classified `signal_description`, split by whether its header carries a compact identity column (`signal`/`name`/`symbol`/`pin`) or a field/register/format shape |
| **B** | heading | a `document_sections` title that is a bare identifier token — `^[A-Z][A-Z0-9_]{1,23}(\(\)\|\[…\])?$` — counted separately for compound titles (containing `_` or an array/call suffix) |
| **C** | prose | an `EvidenceIR` statement matching the formal grammar `Signal <ID> is <input\|output\|inout\|internal\|local\|width>` |
| **D** | *loss* | an `EvidenceIR` statement carrying a markdown-escaped `\_` between identifier characters — not a modality, but the measured cause of subject truncation |

Probes A–C are witnesses of presence; D explains why a present declaration can still be read wrongly.

## Headline results

- **Probe A is empty on all 33.** Not one empty-catalog document has a `signal_description` table with an
  identity header. All 41 `signal_description` classifications that do exist across the 33 were read
  individually; they reduce to 18 distinct header shapes, every one a field, register, encoding, status, or
  table-of-contents shape (§Robustness). For calibration, catalog-bearing documents carry up to 19
  identity-header tables each. **The empty catalogs are not caused by a failing table-authority gate — the
  table modality genuinely is not there.**
- **Probe C is zero on all 33**, as expected: the formal prose grammar is what builds the catalog, so its
  absence is the same event seen from the other side, not an independent witness.
- **Probe B separates exactly one document from the other 32.** `wbspec_b4_wishbone_b4_specification` carries
  32 compound identifier headings, every one a wire name. The next highest is 13, and those are software
  feature selectors, not wires.
- **Probe D is pervasive and orthogonal**: 67 of 78 documents (27 of the 33) carry escaped identifiers, and
  it costs recall on documents that already have catalogs too. See
  [`evidence-statement-markdown-escape-truncates-identifiers`](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md).

## The single capture miss

**`wbspec_b4_wishbone_b4_specification` — CAPTURE MISS.**

Wishbone declares its interface in a **heading-as-declaration convention**: 32 document sections whose title
*is* the wire name — `CLK_I`, `CLK_O`, `RST_I`, `RST_O`, `ACK_I`, `CYC_O`, `STB_O`, `STALL_I`, `DAT_I()`,
`DAT_O()`, `TGD_I()`, `TGD_O()`, … — each followed by prose stating direction and meaning ("The cycle output
[CYC_O], when asserted, indicates that a valid bus cycle is in progress."). 485 of the document's 1,622
`SourceIR` content elements name a `*_I`/`*_O` wire. None of it reaches the declared-signal catalog, because
SpecForge reads declarations from tables and from a formal prose predicate, and this document uses neither.

The document also states its own convention in prose: *"All signal names used in this specification have the
'_I' or '_O' characters attached to them. These indicate if the signals are an input (to the core) or an
output."*

Its 47 demoted records name `CLK`, `CYC`, `STB`, `RST`, `STALL` — truncations, and Probe D explains them
exactly. The `EvidenceIR` statement text is built from the normalized markdown, where the underscore is
escaped (`[CYC\_O]`), and identifier tokenization stops at the backslash. `SourceIR` content elements carry
the same sentences unescaped.

## Honest absence — the other 32, with the reason per group

| Group | Documents | Why the empty catalog is correct |
| --- | ---: | --- |
| `guide`-classed methodology, PHY mechanical/signaling, certification and test-resource notes, software optimization | 17 | No identity-header table, no identifier heading, no formal declaration. The OpenCAPI PHY documents describe lanes, jitter, and mechanical envelopes; their tables are acronym glossaries, revision histories, and contents fragments. `109242_…_arm_smmu_software_guide`'s three identifier headings are register names (`SMMU_CMDQ_BASE`, `SMMU_S_CMDQ_BASE`, `SMMU_R_CMDQ_BASE`), not wires. |
| `register`-classed | 7 | Their `signal_description` tables carry `Bit Location \| Register Description \| Attributes`, `Field name \| Width (bits) \| Value`, and `Offset \| Description` headers — register, flit-field, and fault tables. `ihi0076_a`'s two identifier headings are `IMPLEMENTATION_DEFINED` / `SUBARCHITECTURE_DEFINED` boilerplate. Two are AMBA documents, which answers the tree's second open question: the class does not change the call, the evidence does. |
| `protocol`-classed, other than Wishbone | 4 | See below. |
| unclassified | 4 | See below. |

### The four other `protocol`-classed documents

- **`usb_3_2_revision_1_0_2017_09`** — its 13 compound headings are hub **port-feature selectors**
  (`PORT_POWER`, `PORT_LINK_STATE`, `PORT_OVER_CURRENT`, `C_PORT_CONNECTION`), the arguments to
  `SetPortFeature`/`ClearPortFeature`, not wires. Its 19 `signal_description` tables are LMP packet formats,
  a state-machine legend, and the VBUS requirements matrix — already measured false in
  [`dense-prose-false-signal-loop-reaches-isf`](../knowledge/dense-prose-false-signal-loop-reaches-isf.md),
  where failing closed on this document was the deliberate repair. Its 225 demoted records are the largest in
  the corpus and are dominated by packet and link-command identifiers.
- **`ihi0088_g_2024_06_amba_dti_protocol_specification`** — declares a naming **transformation**, not a
  catalog. Its one `signal_description` table is `Direction | Suffix` (`*_DTI_DN`, `*_DTI_UP`); its three
  identifier headings are message names (`REQ_CONNECT`, `REQ_DISCONNECT`, `ATTR_OVR`). The AXI-Stream signals
  it carries (`TDATA`, `TKEEP`, `TLAST`) are declared in a different specification. Honest absence *for this
  document*; the cross-document case is noted as an open question, not claimed here.
- **`bosch_can_specification_2_0_1991`** — 4 tables, none signal-shaped; its 12 bare headings are section
  words (`TRANSMITTER`, `RECEIVER`, `SYNCHRONIZATION`, `INTERMISSION`). The document specifies frame formats,
  arbitration, and bit timing; the physical wires belong to the companion physical-layer standard.
- **`usb4_connection_manager_guide_v2_0_2025_11`** — zero headings, zero signal-shaped tables. Its own
  `document_intent_category` is `methodology-guide` despite the `protocol` class: a control-plane/software
  document.

### The four unclassified documents

They are unclassified because their `EvidenceIR` carries **zero validation reports**, so no metric surface
exists to read — not because the classifier failed on them. `document_class` is produced by the `validate`
command (`crates/specforge/src/commands/validate.rs`), which back-annotates it into `EvidenceIR`. This is a
corpus-wide condition: **14 of 78** evidence artifacts have no validation report at all, including
`ihi0022_l_…_amba_axi`, `ihi0024_e_…_amba_5_apb`, and `ihi0033_c_…_amba_5_ahb`. That answers the tree's third
open question and is tracked separately.

On the substance, all four are honest absence:

- `ihi0098_a_…` and `ihi0098_a_b_…` (CHI C2C revisions) — `Field name | Width (bits) | Value` flit-field
  tables; bare headings are link states (`RUN`, `STOP`, `ACTIVATE`, `DEACTIVATE`).
- `1_0_2025_03_12_risc_v_advanced_interrupt_architecture` — no signal-shaped table, no identifier heading.
- `jesd235_2013_10_hbm_dram` — honest absence with a **corpus-acquisition caveat**: the retained source is a
  genuine 6-page PDF (`pdfinfo` confirms `Pages: 6`), not the ~125-page HBM standard. SpecForge ingested it
  completely; there is nothing to extract. A full-corpus check confirms ingest is complete everywhere —
  across all 77 available sources, no document's ingested `page_count` is below its PDF page count.

## Per-document probe table

`A` is *identity-header / other-shape* `signal_description` tables; `B` is compound identifier headings;
`D` is escaped statements. Probe `C` is `0` for every row and is omitted.

| Document | class | A | B | D |
| --- | --- | --- | ---: | ---: |
| `wbspec_b4_wishbone_b4_specification` | `protocol` | 0 / 1 | **32** | 455 |
| `usb_3_2_revision_1_0_2017_09` | `protocol` | 0 / 19 | 13 | 348 |
| `ihi0088_g_2024_06_amba_dti_protocol_specification` | `protocol` | 0 / 1 | 3 | 559 |
| `bosch_can_specification_2_0_1991` | `protocol` | 0 / 0 | 0 | 43 |
| `usb4_connection_manager_guide_v2_0_2025_11` | `protocol` | 0 / 0 | 0 | 249 |
| `ihi0098_b_2026_03_23_amba_chi_chip_to_chip_c2c_architecture_specification` | `register` | 0 / 9 | 0 | 383 |
| `ccix_base_specification_r1_0a_v1_0_for_evaluation` | `register` | 0 / 2 | 0 | 41 |
| `5_0_2024_08_intel_virtualization_technology_for_directed_io_specification` | `register` | 0 / 1 | 0 | 155 |
| `ihi0076_a_2018_05_02_advanced_communications_channel_architecture_specification` | `register` | 0 / 1 | 2 | 16 |
| `nvme_base_specification_2_0a_2021_07_26` | `register` | 0 / 1 | 0 | 69 |
| `1_0_1_2026_02_22_risc_v_iommu_architecture_specification` | `register` | 0 / 0 | 0 | 140 |
| `usb4_inter_domain_service_specification_v2_0_2025_11` | `register` | 0 / 0 | 0 | 4 |
| `109242_0100_01_2023_09_04_arm_smmu_software_guide` | `guide` | 0 / 0 | 3 | 69 |
| `198123_0302_03_2025_04_22_generic_interrupt_controller_overview_guide` | `guide` | 0 / 0 | 0 | 54 |
| `opencapi_25gbps_phy_mechanical_spec_v10` | `guide` | 0 / 0 | 0 | 39 |
| `102196_0100_01_2022_05_05_aarch64_external_debug_guide` | `guide` | 0 / 0 | 0 | 16 |
| `opencapi_4_0_32g_phy_signal_spec_1_0_16nov2020` | `guide` | 0 / 0 | 0 | 8 |
| `pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide` | `guide` | 0 / 0 | 0 | 8 |
| `opencapi_discovery_configuration_v201` | `guide` | 0 / 0 | 0 | 7 |
| `opencapi_25gbps_phy_signaling_spec_1_0` | `guide` | 0 / 0 | 0 | 5 |
| `102520_0101_01_2025_09_15_introducing_coresight_debug_and_trace` | `guide` | 0 / 0 | 0 | 2 |
| `opencapi_4_0_32gbps_phy_mech_spec_v10_17mar2021` | `guide` | 0 / 0 | 0 | 1 |
| `opencapi_data_link_layer_v20_09jul2020` | `guide` | 0 / 0 | 0 | 1 |
| `den0068_2018_07_23_coresight_base_system_architecture` | `guide` | 0 / 0 | 0 | 0 |
| `opencapi_3_0_certified_definition_v1_1` | `guide` | 0 / 0 | 0 | 0 |
| `opencapi_3_0_certified_test_resources_engineering_note_v1_0` | `guide` | 0 / 0 | 0 | 0 |
| `opencapi_3_0_ready_definition_v1_1` | `guide` | 0 / 0 | 0 | 0 |
| `opencapi_3_0_ready_test_resources_engineering_note_v1_0` | `guide` | 0 / 0 | 0 | 0 |
| `opencapi_afu_address_space_usage` | `guide` | 0 / 0 | 0 | 0 |
| `ihi0098_a_b_2026_02_03_amba_chi_chip_to_chip_c2c_architecture_specification` | — | 0 / 3 | 0 | 28 |
| `ihi0098_a_2024_02_07_amba_chi_chip_to_chip_c2c_architecture_specification` | — | 0 / 3 | 0 | 25 |
| `1_0_2025_03_12_risc_v_advanced_interrupt_architecture` | — | 0 / 0 | 0 | 21 |
| `jesd235_2013_10_hbm_dram` | — | 0 / 0 | 0 | 4 |

## Reproducer

The census reproducer published in `SIGNAL-CATALOG-CAPTURE-GAP.0` reads
`metric.get("metric_id")`; `EvidenceIR` validation metrics are `{name, value}` pairs and carry no
`metric_id`, so that snippet reports `class=None` for every document. The `.0` breakdown itself
(17 `guide` / 7 `register` / 5 `protocol` / 4 unclassified) is correct and reproduces exactly with the
corrected key. Use:

```bash
python3 - <<'PY'
import json, glob, os
for sp in sorted(glob.glob("generated/semantic_ir/*/semantic_ir.json")):
    key = sp.split(os.sep)[2]
    s = json.load(open(sp))
    recs = [r for i in s.get("interfaces", []) for r in i.get("signal_records", [])]
    if recs:
        continue
    cls = None
    ep = f"generated/evidence_ir/{key}/evidence_ir.json"
    if os.path.exists(ep):
        for rep in json.load(open(ep)).get("validation_reports", []):
            for m in rep.get("metrics", []):
                if m.get("name") == "document_class":
                    cls = m.get("value")
    print(f"{key} class={cls}")
PY
```

Probes A, B, and D are reproduced by the same read-only traversal: count `structured_tables` with
`table_kind == "signal_description"` and test each header row for an identity column; match
`document_sections[].title` against the bare-identifier shape; and search `extracted_statements[].text`
for `[A-Za-z0-9]\_[A-Za-z0-9]`.

## Robustness of the probes

The probes are regexes and a set-membership test. That is fragile as an *oracle* and acceptable as a
*candidate generator whose every hit is then read*. Three checks establish which of the two each probe is,
and one of them changed a number in this document.

**A regex was outright wrong here, and was caught.** The `jesd235` ingest was first judged truncated on a
`/Type/Page` byte-regex over the raw PDF, which reported 125 pages against 6 ingested. `pdfinfo` reports
`Pages: 6`: the regex counts page objects from embedded and superseded revisions, not the page tree. The
corpus-wide completeness result quoted above uses `pdfinfo`, not the regex. Nothing in this document rests on
a byte-level PDF heuristic.

**Probe A was re-run over every table, not a sample.** The first pass recorded at most four header shapes per
document. Re-reading all 41 `signal_description` tables across the 33 yields 18 distinct header shapes:
`field name | width (bits) | value` (×10), 8b/10b encoding tables `data byte name | data byte value (hex) | …`
(×8), `width (bits) | offset (DW:bit) | description` (×3), `bit location | register description | attributes`
(×2), `offset | register name` (×2), `bit | description` (×4 across two variants), `direction | suffix`,
`bits | field | description | access type | reset`, port-status and fault-reason matrices, and two
table-of-contents fragments. None is a signal inventory. The conclusion rests on having read all 18 shapes,
not on the identity-column membership set — which is exact-match and would, for example, miss a header
spelled `Signal name (abbrev)`. The earlier count of 44 in this document was a mis-sum of the per-document
column and is corrected to 41.

**Probe B is deliberately over-broad, and its blind spot was measured.** The shape is uppercase-only, so a
mixed-case declaration convention (`ClkIn`, `n_reset`) would be invisible. Relaxing it to any case across all
33 documents adds 0 wire declarations: the new hits are ordinary section words (`Contents`, `Abstract`,
`Preface`, `Conventions`, `Glossary`, `Feedback`), glossary entries, CamelCase protocol identifiers
(`DVMConnect`, `DVMDisabled`, `CompAck`, `NoStall`, `ContainerID`), and DTI pseudocode function names
(`CombineAllocHints()`, `DefaultMemAttrHints()`). The blind spot is real in principle and empty on this
corpus, so the classification is unchanged.

That same relaxation is the strongest argument that **Probe B must not become the product rule**: on Wishbone
it goes 32 real wires → 52 uppercase hits → 69 any-case hits, and the `()` suffix that legitimately marks
Wishbone's `DAT_I()` array also matches DTI's pseudocode functions. A title shape alone cannot license a
declaration. `SIGNAL-CATALOG-CAPTURE-GAP.2` is therefore a design leaf: the rule it must find is
corroborative — a title that names an identifier *and* a body that states a direction for it — not a
prettier regex.

**Probe D is the one result that does not depend on a heuristic being right**, because it is triangulated
across four surfaces rather than inferred from one: the normalized markdown bytes carry `\_`, the `SourceIR`
content elements and table cells carry 0, the `EvidenceIR` statements carry 455, and the typed
`signal_constraints[].subject_signal` is the truncated stem while that record's own `source_text` preserves
the escaped full name. Any single one of those could be a regex artifact; all four agreeing is a mechanism.

## What this leaf does not claim

The honesty guardrail from `EXTRACTION-GAP-FIX` holds: **you cannot extract what is not there.** This leaf
establishes presence or absence of a declaration-bearing modality; it does not write a rule, and it does not
claim that closing Wishbone's heading convention is safe on the other 45 documents. That is
`SIGNAL-CATALOG-CAPTURE-GAP.2` and `.3`.
