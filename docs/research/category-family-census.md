# What each road states that no IntentIR record carries — the per-family census on current code

Owner: `EXTRACTION-REPRESENTATION-GAPS.3`. Measured `2026-09-26` at `48ddcedc` plus the validation cascade below.
Reproduce (read-only): `python3 scripts/measure_category_family_census.py [--json PATH]`.

The denominator is the category contract, `doctrine/spec_to_intent_category_contract.json`: six document
categories, each a list of semantic families mapped to the IntentIR surfaces that carry them. The population is
every chain whose normalized bundle is retained and whose EvidenceIR is at the current schema: **31 chains**, so
every number is what the current binary produces. Legacy chains (47) are not counted.

## The documents, by the category they report

| category (the classifier's) | documents | confidence |
| --- | ---: | --- |
| wire-protocol | 7: AXI, APB-E, AHB, CHI C2C issue A, I2C, USB 3.2, Wishbone | high |
| register-or-platform | 5: RISC-V Debug, RISC-V IOMMU, CoreSight TMC, ADIv6, OpenCAPI discovery | low |
| methodology-guide | 6 | high |
| physical-link | 5 (OpenCAPI PHY and data-link) | low |
| unresolved | 8 | low |
| cpu-isa | 0 | — |

The classifier merges categories 2 and 3 by design and reports category 4 only on a front-matter ISA
self-declaration (`classify_document_intent_category`), so no replayable document is a CPU ISA and the register and
platform roads share one bucket. Families are therefore measured per document against **all six** categories'
families; the classifier's bucket is only a label.

**Correction to `.3.a`.** `.3.a` recorded all four documents it re-ingested as register-or-platform. Those values
came from `validation_report.json` files written on `2026-08-12` by the legacy binary: `specforge evidence`
rewrites the artifact and leaves the previous build's report in place; only `specforge validate` writes it, and the
refresh route skipped it. After validating each stage once, upstream to downstream (all five chains rebuilt that
day stay current at every stage), CHI C2C reports **wire-protocol (high)**, since its 149 message fields now count
as a wire cue beside its 3 protocol states, and OpenCAPI discovery moves from unresolved to register-or-platform
after `EXTRACTION-GAP-FIX.6`. The census script now refuses a report older than its artifact.

## Families with no IntentIR carrier

Six contract families map to no IntentIR surface. On current code:

| family | what the replayable chains show | owner |
| --- | --- | --- |
| wire `message_and_flit_structure` | CHI C2C: 149 EvidenceIR message fields in 13 containers, 19 tables no record cites | `DOC-INTENT-TAXONOMY.4b` (gap S18) |
| register `programming_structures` | RISC-V IOMMU's in-memory structures (device contexts, queues, page tables) are not captured at all: 0 message-field records; USB 3.2 and USB4 descriptor tables (`Width (bits) | Offset (DW:bit) | Description`, `DW | Bits | Description`: 49 tables) reach no record | `DOC-INTENT-TAXONOMY.4b` (gap S18) |
| platform `component_topology_and_domains` | no surface to measure; connectivity is thin where it exists | `EXTRACTION-REPRESENTATION-GAPS.14` (gap S22) |
| ISA `software_architecture_semantics` | no replayable document; the assumptions below include software and debugger obligations | `EXTRACTION-REPRESENTATION-GAPS.3.d` |
| physical `physical_electrical_and_mechanical_content` | out of scope by the contract (only the digital boundary is lowered) | — |
| methodology `embedded_hardware_contract` | read through the other categories' surfaces | — |

## Captured source regions no canonical record cites

IntentIR's `captured_region_residuals` list every table and figure that reached EvidenceIR and that no SemanticIR
record cites: **2,793** over the 31 chains.

| category | regions | tables | figures and diagrams |
| --- | ---: | ---: | ---: |
| wire-protocol | 1,389 | 621 | 768 |
| register-or-platform | 859 | 398 | 461 |
| methodology-guide | 226 | 113 | 113 |
| unresolved | 158 | 83 | 75 |
| physical-link | 161 | 99 | 62 |

Attributed by the kind SourceIR gave each region, or by EvidenceIR's message fields citing it:

| class | regions | documents | owner |
| --- | ---: | ---: | --- |
| figure, diagram kind `unknown` | 1,414 | 31 | `FULL-PAGE-INTENT-CAPTURE.2` (gap S25) |
| table, kind `unknown` (507 header shapes, 277 with no header row) | 1,161 | 31 | `EXTRACTION-REPRESENTATION-GAPS.3.b` |
| table, `encoding` | 91 | 8 | `EXTRACTION-REPRESENTATION-GAPS.3.c`; 15 are carried and miscounted (`.3.e`), below |
| table, `signal_description` | 32 | 6 | `EXTRACTION-REPRESENTATION-GAPS.3.c` |
| figure or diagram, `state_machine_diagram` | 30 | 5 | `EXTRACTION-REPRESENTATION-GAPS.12` (gap S10) |
| table, message/structure fields (EvidenceIR has them) | 19 | 1 | `DOC-INTENT-TAXONOMY.4b` (gap S18) |
| diagram, `unknown` / `block_diagram` / `timing_diagram` / `flow_chart` | 35 | 9 | `FULL-PAGE-INTENT-CAPTURE.2` (gap S25) |
| table, `timing_parameter` | 8 | 2 | `EXTRACTION-REPRESENTATION-GAPS.3.c` |
| table, `feature_matrix` | 2 | 2 | `EXTRACTION-REPRESENTATION-GAPS.3.c` |
| table, `register_map` | 1 | 1 | none: RISC-V IOMMU `table_0077`, one row whose field is named `0` ("Fixed to 0"), dropped by the bit-layout-grid filter before and after `EXTRACTION-GAP-FIX.6` |

The most frequent untyped header shapes (grouped by their first three columns) show what the 1,161 tables hold, and that it is not one thing:
descriptor and structure layouts (`Width (bits) | Offset (DW:bit) | Description` 29, `DW | Bits | Description` 20,
`Field Name | Width (bits) | Value` 6), USB request encodings (`bmRequestType | bRequest | wValue …` 24),
register field tables with neither an access nor a reset column (`Bits | Name | Function` 23, all CoreSight TMC,
read by neither the old chain nor the current one: `EXTRACTION-GAP-FIX.9`), signal presence matrices
(`Signal | Presence | AXI5 | ACE5-Lite` 6), and non-contract tables: instruction latencies (44, the Cortex-A76 guide),
debug program examples (`Op | Address | Value | Comment` 20), revision histories and glossaries.

## The 91 encoding tables no record cites (`.3.c`)

Reproduce: `python3 scripts/measure_uncarried_encoding_tables.py`. Each table is attributed by replay to the enum it
minted, with the locator the enum instruments already calibrate (`located_tables`).

| verdict | tables | where | owner |
| --- | ---: | --- | --- |
| minted an enum that reaches IntentIR | 15 | AXI 13 (`AxBURST`, `RRESP`, `ARCACHE`, `AWATOP` ×2, `AxDOMAIN`, `AWSNOOP`, `ARSNOOP`, `ARTAGOP`, `AWTAGOP`, `AxMMUSECSID`, `AxMMUFLOW`, `Security`), CHI C2C 2 (`MsgType`, `ChunkValid`) | `EXTRACTION-REPRESENTATION-GAPS.3.e` |
| minted an enum withdrawn before IntentIR | 3 | AXI `AxSIZE`, `BTAGMATCH`; AHB `HSIZE` | `KG-ISF-COMPLETENESS.5.xiii.a.6` (`BTAGMATCH`); `KG-ISF-COMPLETENESS.5` |
| minted nothing | 73 | USB 3.2 35, RISC-V IOMMU 11, CHI C2C 9, AXI 9, USB4 3, ADIv6 3, RISC-V Debug 2, AHB 1 | by cause, below |

### Why 73 encoding tables mint nothing

Read with a throwaway replay of both production paths per table (the encoding-table path, which names the enum
from the caption, section or header; the signal-anchor scan, which names it after a declared signal), with the
build's own prior-memory guidance; `ir/evidence.rs` was restored byte-identical afterwards.

| cause | tables | examples | disposition |
| --- | ---: | --- | --- |
| no enum name: the caption's candidate is neither a declared signal nor a column of the table, so `KG-ISF-COMPLETENESS.5.i`'s gate refuses it | 66 | see below | refused by design |
| minted in this replay, withdrawn later in the build | 3 | CHI C2C `Opcode2`, `Message credit`, `ActTrigger` | cause not yet read: `.3.c` |
| keyed by more than its value column | 2 | AXI `AWSNOOP`/`ARSNOOP` Tables A7.3 and A7.4 | `KG-ISF-COMPLETENESS.5.xiii` |
| every row refused by the member gates | 1 | USB 3.2 *Compliance Pattern Sequences* | `.3.c` |
| body held in the header rows | 1 | AXI *Physical address space encodings* | `EXTRACTION-REPRESENTATION-GAPS.3.g` |

The 66 the naming gate refuses are of four kinds, and the gate is right about most of them:

- **24 encode a field of a structure or message**: IOMMU `iohgatp.MODE`, `iosatp.MODE`, `pdtp.MODE`, `msiptp.MODE`,
  fault-record `CAUSE` and `TTYP`, command opcodes; AXI DVM message fixed fields (6); CHI C2C `MiscOp`,
  `ConnectOp`, `FlitFormat`, `LinkPowerState`, DVM response opcodes; ADIv6 TMS packets; USB4 packet types. The
  field they encode has no IntentIR carrier, so neither does its encoding: `DOC-INTENT-TAXONOMY.4b` (gap S18).
- **22 are USB 3.2 descriptor layouts and sequences** (19 descriptors such as *Standard Device Descriptor*, *BOS
  Descriptor*; *Format of Setup Data*; 2 ordered sets and a compliance pattern) that SourceIR labels `encoding`.
  They are structure layouts: `DOC-INTENT-TAXONOMY.4b`, and `.3.b` for the label.
- **14 have no caption**: continuation pages of the tables above.
- **6 name the quantity they encode** (*U1 Timeout Value Encoding*, *Privilege Mode and Virtualization Mode
  Encoding*, *Device Capability Type Codes*): the recall the gate gives up so that a caption word never becomes an
  enum. No owner: reopening it needs a positively evidenced name source.

**The first row is an accounting defect, not an extraction gap.** The accounting counts a table as carried when a
record names it in `supporting_table_ids` (`SemanticIr::cited_table_ids`: register, timing, polarity and
interface-signal records). A symbol definition names no table: its members cite synthetic `Enum X M = v.`
statements that carry no span. So an encoding table whose enum reaches IntentIR is still listed as having no
carrier.

## The other 43 typed tables no record cites (`.3.c`, first read)

Read table by table against the extractor for their kind. Where a row says *unread*, the cause is `.3.c`'s to find.

| what the table is | tables | owner |
| --- | ---: | --- |
| timing limits in several operating-mode column groups (I2C *Standard / Fast / Fast-mode Plus* and *Cb = 100 / 400 pF*, including the core `fSCL`, `tHD;STA`, `tLOW` table): the scalar timing record has no mode dimension, a limit `CORPUS-COVERAGE.2.47a` documented and nobody owned | 3 | `EXTRACTION-REPRESENTATION-GAPS.3.f` |
| real signal tables of two rows (CoreSight TMC *Clock and reset*, *Synchronization request*, *Test interface*); `CLK` was absent from TMC's interfaces | 3 | `SIGNAL-DECLARATION-ROW-DROP.6`, done `2026-09-26`; TMC's clock sentence is `EXTRACTION-REPRESENTATION-GAPS.7.f` |
| every row held in the header rows, none in the body (AXI presence summaries B2.2/B2.3 and A13.3, USB 3.2 LFPS and eye timing, *Port Type Selection*); 133 such tables corpus-wide | 9 | `EXTRACTION-REPRESENTATION-GAPS.3.g` |
| interface or class property tables (AXI `Trace_Signals`, `Loopback_Signals`, `Wakeup_Signals`, `Coherency_Connection_Signals`; CHI C2C uniform properties) | 5 | `KG-ISF-COMPLETENESS.2a.vii.c` |
| encodings under the signal-description label (AHB `HBURST`, `HPROT`; ADIv6 `DbgSwEnable`; USB port status type codes): neither enum path read that kind | 4 | `EXTRACTION-REPRESENTATION-GAPS.3.h` read `HBURST` (`2026-09-26`); `HPROT` (per-bit flags), `DbgSwEnable` and USB's codes stay with `.3.c` |
| structure layouts and status words under that label (USB LMP formats; port status and change fields) | 7 | `DOC-INTENT-TAXONOMY.4b`; the four status-word tables are field tables like `EXTRACTION-GAP-FIX.9`'s |
| connector pinouts and pin routing (RISC-V Debug MIPI connectors, ADIv6 SWJ-DP) | 3 | none: physical content, outside the digital boundary the contract lowers |
| keys and legends (AXI *Key to signals table*, *Signals that should be the same*) | 2 | none: navigation, not intent |
| unread: AXI credited-channel signals and legal MMU combinations, ADIv6 programmers'-model mapping, USB hub port timers and power states, OpenCAPI DVSEC applicability | 6 | `.3.c` |
| RISC-V IOMMU `table_0077`, one row whose field is named `0` | 1 | none: the bit-layout-grid filter drops it, before and after `EXTRACTION-GAP-FIX.6` |

## The 1,165 untyped tables no record cites (`.3.b`)

Measured `2026-09-29` with `python3 scripts/measure_untyped_table_families.py` (read-only; `--samples N` prints N
members per family). A table is placed by its first header row's words, or with no header row by its body's shape;
the first matching family wins. Precision was read on seven random members of each family: parameter definitions 7
of 7, software and performance 6 of 6, structure layouts about 5 of 6 (one USB response matrix), presence and
compliance about 6 of 7 (one glossary row); the two register families are mixed, which is why they are read table by
table rather than given an owner.

| family | tables | owner or disposition |
| --- | ---: | --- |
| structure and message layouts (USB 3.2 86, USB4 31, C2C 23) | 207 | `DOC-INTENT-TAXONOMY.4b` |
| document apparatus (revisions, glossaries, contents) | 180 | non-contract |
| figures the converter read as tables (ADIv6 62) | 106 | non-contract |
| parameter definitions (AXI 76) | 78 | `KG-ISF-COMPLETENESS.2a.vii.c` |
| instruction timings and programming examples | 73 | non-contract |
| presence, compliance and applicability matrices | 71 | `EXTRACTION-REPRESENTATION-GAPS.3.c.v` |
| line coding, lanes, electrical parameters | 70 | non-contract (outside the digital boundary) |
| register indexes and register-figure pieces (ADIv6 39) | 56 | `.3.b.ii` |
| encodings keyed by several register fields (ADIv6) | 15 | `.3.g.iii` |
| empty tables | 15 | non-contract |
| register field tables the field-table reader missed | 12 | `.3.b.ii` |
| behaviour keyed by several signals' values (AXI 8) | 12 | `.3.g.iii` |
| waveform narrations (AHB 6) | 7 | non-contract |
| signal tables left untyped (CoreSight TMC) | 6 | `.3.b.i` |
| residue: 182 header shapes, and 48 tables with no header row | 257 | `.3.b.ii` |

451 tables (38.7%) state nothing the contract lowers, 389 (33.4%) are contract content with an owner, and 325
(27.9%) are read by `.3.b.ii`. The one real loss found is TMC's: six `Signal | Type | Description` tables (ATB slave,
APB debug, memory and AXI master interfaces) declare 55 signals, none of which reaches the catalog, because SourceIR
typed them `unknown` and no reader visits an untyped table. AXI's snoop-channel lists (`Name | Width | Source |
Presence`) are also uncited, but every signal in them is declared by another table.

## Records captured but not understood

An untyped IntentIR record (a free-text `statement` with provenance) is *not understood* when no statement it
cites is cited by a typed record.

| category | constraints | behaviors | assumptions |
| --- | ---: | ---: | ---: |
| wire-protocol | 30 of 3,969 | 137 of 1,405 | 74 of 92 |
| register-or-platform | 20 of 1,053 | 65 of 519 | 47 of 49 |
| methodology-guide | 5 of 283 | 56 of 150 | 26 of 26 |
| unresolved | 5 of 333 | 22 of 132 | 6 of 6 |
| physical-link | 0 of 200 | 8 of 59 | 14 of 14 |
| **all** | **60 of 5,838** | **288 of 2,265** | **167 of 187** |

Constraints are almost all understood; assumptions almost none. A sample of the assumptions mixes software and
debugger obligations (*the debugger must poll `dmcontrol`…*, *an entry marked not present must be skipped*),
treatment rules (*a WriteDeferrable transaction must be treated as 64-byte atomic*), environment statements
(*all bulk transfers contend for the same bus time*) and conventions of measurement or drawing (*the latencies
assume an L1 hit*, *shaded areas are undefined*). Which of those is hardware intent, which is software intent the
contract names (`software_architecture_semantics`, `programming_model_and_behavior`) and which is no contract at
all is `EXTRACTION-REPRESENTATION-GAPS.3.d`.

## What this changes

- The contract needs no new family: every class above falls in a family it already names, and the gaps are
  carriers and extraction, each owned in the table.
- Nine owners are new: `.3.b` (the untyped tables), `.3.c` (typed tables no record cites), `.3.d` (the unread
  assumptions and behaviors), `.3.e` (a symbol's minting table), `.3.f` (a timing limit's operating mode), `.3.g`
  (tables held in header rows), `.3.h` (encodings typed as signal descriptions), `EXTRACTION-GAP-FIX.9` (TMC's field
  tables) and `SIGNAL-DECLARATION-ROW-DROP.6` (TMC's signal tables).
