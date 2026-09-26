# EXTRACTION-REPRESENTATION-GAPS: one ledger of what a specification states, what SpecForge extracts, and what `.isf` can say — each gap owned on its own side

## Metadata

- Tree ID: `EXTRACTION-REPRESENTATION-GAPS`
- Status: `active`
- Roadmap lane: the program objective (ADR 0033, amended by ADR 0053) and `R0` downstream contract hygiene
- Created: `2026-09-25`
- Last updated: `2026-09-26`
- Owner: repo-local workflow
- Decision: [ADR 0053](../decisions/0053-extraction-and-isf-requests-advance-in-parallel.md)

## Goal

Every piece of design intent a chip specification states is **extracted** by SpecForge into canonical IR and
is **representable** in `.isf`. Where either fails, the gap is named, attributed to the side that owns it, and
owned by an open leaf: a SpecForge extraction leaf, an FSMGen request (through `FSMGEN-REQUESTS`), or both.
The owner's directive (`2026-09-25`): *SpecForge does its part and FSMGen does its part.*

The ledger is a book chapter, [`reference/representation-gaps.md`](../book/src/reference/representation-gaps.md),
so the owner can see every gap and who holds it without reading code.

## Non-Goals

- Filing a request without a reproduced gap. ADR 0053 keeps `PROCESS.md`'s evidence bar: cited
  specification text, and FSMGen's refusal reproduced on the pin and on upstream.
- Forwarding requests. The owner forwards; `FSMGEN-REQUESTS` records it.
- Fixing a SpecForge extraction gap inside this tree when an existing tree already owns that category. The
  ledger names the owner; the work stays where it belongs.
- Inspecting or changing `subs/fsmgen`. FSMGen is read only through its published book and CLI.

## Acceptance Criteria

- The ledger lists every gap the two censuses (`.2`, `.3`) find, with evidence, the owner of each half and a
  status, and a check refuses a row whose owner is not an open work unit (`.1`).
- Every FSMGen-owned row has a request bundle signed off under `PROCESS.md`, or a written reason it does not
  need one (already in FSMGen's published contract at the pin, or withdrawn).
- Every SpecForge-owned row has an open leaf in the tree that owns its category.
- The ledger is re-checked on every FSMGen pin refresh; the refresh leaf records the re-check.

## Task Tree

- ID: `EXTRACTION-REPRESENTATION-GAPS`
  Status: `active` (`2026-09-25`)
  Children: `.0`-`.28`

- ID: `EXTRACTION-REPRESENTATION-GAPS.0`
  Status: `done` (`2026-09-25`; DECISION + DOC)
  Goal: **open the lane.** Record the owner's directive as ADR 0053 (amending ADR 0033 item 4: an FSMGen
  request may precede the IntentIR carrier when an open SpecForge leaf owns the extraction); publish the
  ledger chapter seeded only with rows verified in `KG-ISF-COMPLETENESS.2a.vii.a`-`.c.iii` and against FSMGen's
  published book at the pin (13a *Interface*, 13k feature matrix rows *Actor report metadata and params*,
  *Interface ports*, *Reusable ISF libraries*); state its update rule; route it from the book summary, the
  FSMGen requests chapter and the ISF adapter chapter; add the lane to the roadmap's owner region.

- ID: `EXTRACTION-REPRESENTATION-GAPS.1`
  Status: `done` (`2026-09-25`; CHECK)
  Goal: **a ledger row cannot cite a closed or missing owner.** Declare the ledger's owner column as an
  `OWNERSHIP-CITATIONS` region. First extend `scripts/check_ownership_citations.pl`: it resolves a leaf's
  status only from the two-line `- ID:` / `Status:` form in a tree's root file, so a leaf of a partitioned tree
  (`KG-ISF-COMPLETENESS.2a.vii.c.iv`, declared as one line `- ID: … · Status: …` in a part under
  `docs/tasks/<tree>/`) reads as nonexistent. Resolve the one-line form in the root and in the tree's parts,
  with a self-test case for each form and a refused case for a closed leaf.
  **Landed.** `resolve_leaf_status` reads both grammars over the root and every part file under
  `docs/tasks/<tree in lower case>/` (`tree_part_texts`), and fails closed when a leaf's declarations disagree;
  the self-test calls it directly instead of re-stating its regex (the old leaf-shape case tested a copy), and
  gains five cases (one-line in a root, in a part, a closed one-line leaf, disagreeing declarations, a longer id
  sharing the prefix): 16/16. The ledger's open-gaps table is the `gap_owners` region, with its 10 owners
  classified `current_owner`. The limit mattered beyond this ledger: the one-line form is how every partitioned
  tree and many roots declare leaves, so any such citation in a declared region would have been refused as
  nonexistent.

- ID: `EXTRACTION-REPRESENTATION-GAPS.2`
  Status: `done` (`2026-09-25`; MEASUREMENT)
  Goal: **the IntentIR-to-`.isf` census.** For every IntentIR family, over every live chain: how many records
  the IR carries, how many the adapter lowers, and, for the rest, why — the IR lacks an operand (SpecForge), or
  FSMGen's published contract cannot state it (FSMGen; cite the section), or both. The adapter's residual packets
  are the starting evidence. Each FSMGen-owned result becomes a request leaf here; each SpecForge-owned result a
  leaf in the tree that owns the family. Every row lands in the ledger with its evidence command.
  **Measured** (`python3 scripts/measure_intent_to_isf_reach.py`, read-only; result below under *`.2` — the
  IntentIR-to-`.isf` census*): the adapter reads 18 of IntentIR's families and never reads ~20; **every one of
  the 27 live adapters is `blocked`, all 27 for want of a clock/reset contract**, so no current chain hands FSMGen an
  `.isf` file; 150 residual packets fall into 8 attributable classes. New leaves `.7`-`.16`; `.3` re-anchored.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3`
  Status: `in_progress` (`2026-09-26`; MEASUREMENT; per-family census on current code recorded; `.3.b`-`.3.d` open)
  Goal: **the PDF-to-IR census: extend the closed ontology.** `docs/research/intent-ontology-coverage.md`
  (`2026-05-31`) is the recall denominator, and it has no category for configuration parameters, presence
  properties or interface-class profiles, which is why the `.2a.vii.c` gaps were found by hand. Measure, per
  document category, which kinds of design intent the live corpus states that no IR carrier holds; extend the
  ontology with each, and give each an owning SpecForge leaf. The ledger gains a row per category.
  **Re-anchored by `.2`**: the per-category denominator already exists as
  `doctrine/spec_to_intent_category_contract.json` (six categories, each with semantic families mapped to IntentIR
  surfaces; `platform-system-ip`'s `component_topology_and_domains` family maps to none), so measure against it
  and extend it rather than the older ontology. It also owns the **7,477 untyped records** `.2` found
  (`constraints` 5,229, `behaviors` 2,091, `assumptions` 157: a free-text `statement` plus provenance): count how
  many state something no typed record holds, which is captured-but-not-understood intent.
  **First census `2026-09-26`** (`.project-data/tmp/erg3/census.py`, read-only, over all 78 persisted chains; the 51
  not replayable are from older binaries and only indicative). By each chain's own `document_intent_category`:
  wire-protocol 21 documents (6 replayable), register-or-platform 28 (1 replayable, ADIv6), physical-link 6 (5),
  methodology-guide 8 (6), unresolved 15 (9); no document reports cpu-isa (RISC-V lands in register-or-platform at
  low confidence). register-or-platform holds 1,261 of the 1,527 EvidenceIR message/structure field records and
  3,393 of the IntentIR register records, and the contract maps four families to **no** IntentIR surface: wire
  `message_and_flit_structure`, register `programming_structures`, platform `component_topology_and_domains`, ISA
  `software_architecture_semantics`. So the category with the largest known carrier gap has one replayable
  document. Untyped records whose statements no typed record cites: register-or-platform 247 of 16,004
  constraints, 908 of 5,210 behaviors, 524 of 541 assumptions; wire-protocol 107 / 10,070, 440 / 5,107, 170 / 212.
  **Per-family census on current code `2026-09-26`** ([record](../research/category-family-census.md); reproduce with
  `python3 scripts/measure_category_family_census.py`), over the 31 replayable chains: wire-protocol 7 (C2C among
  them), register-or-platform 5, methodology-guide 6, physical-link 5, unresolved 8, cpu-isa 0. **2,793 captured
  regions no canonical record cites**: 1,414 unclassified figures, 1,161 untyped tables (507 header shapes, 277
  headerless), 134 typed tables (91 encoding, 32 signal description, 8 timing, 2 feature matrix, 1 register map), 30
  state-machine figures, 19 message-field tables. Untyped records not understood: constraints 60 of 5,838, behaviors
  288 of 2,265, assumptions **167 of 187**. Every class falls in a family the contract already names, so the contract
  gains no family; each is routed to an owner in the record: `.3.b`, `.3.c`, `.3.d` and `EXTRACTION-GAP-FIX.9` are new.
  The census found its own first input wrong: the categories `.3.a` recorded came from legacy reports (corrected
  there). The five chains rebuilt that day were then validated stage by stage and stay current; the script refuses a
  report older than its artifact.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.a`
  Status: `done` (`2026-09-26`; DATA re-ingest; SpecForge; found by `.3`)
  Goal: **one replayable document per under-covered road.** Re-ingest, on the current binary and under the
  `MEMORY-BOUNDED-INGEST` bounds, RISC-V Debug (119 pages; CSR bits drawn in figures), RISC-V IOMMU (108; register
  IP with in-memory structures), AMBA CHI C2C issue A (108; 149 message/flit fields) and the CoreSight TMC TRM (116;
  a platform component), and add each retained bundle to `doctrine/chain_currency/retained_bundles.json`, so chain
  currency measures them. Record each document's category and the families `.3` then measures.
  **Done.** Each chain was snapshotted, then rebuilt `ingest` -> `evidence` -> `semantic` -> `intent` ->
  `adapt --target isf` (`DOCLING_DEVICE=cpu`, `SPECFORGE_INGEST_RAM_ABORT_PERCENT=85`, no model server). A host-library
  source is ingested by its resolved same-volume path: the persisted-path contract refuses the
  `.cache/local-references` symlink, as it did the first attempt. Every chain is now canonical SourceIR 3 /
  EvidenceIR 3 / SemanticIR 2 / IntentIR 2, and the currency oracle, run over the four, reports 4 replayed / 4
  current / 0 stale at every stage; all four adapters are `blocked` on the clock/reset contract, as `.2` found for
  every chain. Retained 27 -> 31.

  | document | pages | ingest | peak RSS | bundle | category |
  | --- | ---: | ---: | ---: | ---: | --- |
  | RISC-V Debug 1.0 | 119 | 93 s | 5.6 GiB | 475 files / 141 MiB | register-or-platform, low |
  | RISC-V IOMMU 1.0.1 | 108 | 61 s | 5.9 GiB | 417 / 126 MiB | register-or-platform, low |
  | CHI C2C issue A | 108 | 57 s | 6.2 GiB | 309 / 76 MiB | wire-protocol, high (corrected, see below) |
  | CoreSight TMC r0p1 | 116 | 57 s | 8.6 GiB | 392 / 84 MiB | register-or-platform, low |

  **Corrected by `.3` the same day: the categories first recorded here were the legacy binary's.** `evidence` does not
  write `validation_report.json`, this route skipped `validate`, and the four reports read were from `2026-08-12`.
  Validated stage by stage, CHI C2C is **wire-protocol (high)**: 149 message fields and 3 protocol states make a wire
  cue. The other three stay register-or-platform (low), which is the classifier's design: categories 2 and 3 are
  combined because counts cannot tell them apart, and category 4 needs an ISA self-declaration.
  **Deltas against the stale chains, each read to its cause:**
  - TMC's register surface is now right where it was wrong. The old chain filed the 50-row *TMC summary* table as
    one pseudo-register holding 30 "fields" that are registers; the current one makes each row a register (30 -> 50
    records, all 50 with an offset). Its 31 timing constraints are gone correctly: the old "timing table" was
    *Table 3-52 Permitted register writes*.
  - RISC-V Debug keeps 44 registers / 179 fields; its gold now scores again (it was legacy and refused).
  - **IOMMU's register surface is wrong, and so is the already-current OpenCAPI discovery chain's.** The current
    classifier types IOMMU's 18 `Bits | Field | Attribute | Description` tables `register_map`, and
    `synthesize_register_records` attaches each field row to the last register built from **any** earlier table.
    So one record named `version` (the first field of `table_0033`) holds the 98 fields of 18 registers while
    citing only `table_0033`, and OpenCAPI discovery's `Device ID` holds 206 fields from a 29-row table. Owned by
    `EXTRACTION-GAP-FIX.6`.
  - Access types are lost the same way: IOMMU 147 -> 71 fields carrying one (`RW` and `RW1C` gone entirely), TMC's
    30-row summary table none. `Attribute` and `Type` are not read as access headers, and the fallback wants every
    cell in a closed list that has no `WARL`/`WPRI`/`RW1S` and refuses a footnoted `RO,RW a`. Owned by
    `EXTRACTION-GAP-FIX.7`. The register-map path also reads a register's `Reset` column and discards it: a
    `RegisterRecord` has no carrier for it. Owned by `EXTRACTION-GAP-FIX.8`.
  - The chains lost phrase actors and symbols the current precision rules refuse (RISC-V Debug actors 17 -> 6,
    symbols 20 -> 0; TMC actors 26 -> 21); the rules predate this leaf and were adjudicated where they shipped.
  **Found while declaring the bundles:** the three AMBA golds' qualification debt named
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`, which was already `done` when `RETAINED-BUNDLE-POPULATION-FROZEN.3` declared
  it, and the behavioral gate accepts any non-empty `owing_leaf`. All seven entries now name
  `CORPUS-CHAIN-CURRENCY.12`, which owns executing those relations and making the gate refuse a closed owner.
  **Also corrected:** the book's RISC-V Debug eval block still showed register-name association `59/60`;
  `EXTRACTION-GAP-FIX.4c` had moved it to `43/44` in June by merging 16 fragments. Re-derived today: field-name
  recall 20/34, register-name 43/44, bit-structure 0/34, bit-extent 0/179.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.b`
  Status: `open` (`2026-09-26`; MEASUREMENT; found by `.3`)
  Goal: **say what the 1,161 untyped tables no record cites state.** They reach EvidenceIR with SourceIR kind
  `unknown` in all 31 replayable chains: 507 header shapes and 277 tables with no header row. The largest shapes
  already show several families: descriptor and structure layouts (USB 3.2 `Width (bits) | Offset (DW:bit) |
  Description` 29, USB4 `DW | Bits | Description` 20, C2C `Field Name | Width (bits) | Value` 6), USB request
  encodings (24), signal presence matrices (AXI 6), register field tables (TMC 23, owned by `EXTRACTION-GAP-FIX.9`),
  and non-contract tables (Cortex-A76 instruction latencies 44, RISC-V Debug program examples 20, revision
  histories, glossaries). Adjudicate the shapes covering most of the population into contract families or a stated
  non-contract disposition, route each family to its owner (opening one where none exists), and record the residue.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c`
  Status: `open` (`2026-09-26`; MEASUREMENT; found by `.3`)
  Goal: **a typed table no record cites is a refusal with a reason or a miss with an owner.** 134 tables that
  SourceIR typed reach no canonical record: 91 `encoding` (8 documents; USB 3.2 35, AXI 24, IOMMU 11, C2C 11), 32
  `signal_description` (6; AXI 11, USB 3.2 11), 8 `timing_parameter` (USB 3.2 5, I2C 3), 2 `feature_matrix`, 1
  `register_map` (IOMMU `table_0077`, a one-row table whose field is named `0`, dropped by the bit-layout-grid
  filter). Read each against its extractor: refused by a rule (name it), a classification error, or a recall miss
  (open or name the owning leaf).
  **Encoding tables adjudicated `2026-09-26`** (`python3 scripts/measure_uncarried_encoding_tables.py`, attribution by
  replay with `located_tables`): 15 minted an enum that reaches IntentIR (AXI 13, C2C 2) and are listed only because a
  symbol definition cites no table (`.3.e`); 3 minted an enum withdrawn before IntentIR (`AxSIZE`, `BTAGMATCH`,
  `HSIZE`; `KG-ISF-COMPLETENESS.5.xiii.a.6` owns `BTAGMATCH`, `KG-ISF-COMPLETENESS.5` the other two); 73 minted
  nothing (USB 3.2 35, IOMMU 11, C2C 9, AXI 9, USB4 3, ADIv6 3, RISC-V Debug 2, AHB 1). **Why, read by a throwaway
  replay of both production paths:** 66 get no enum name because `KG-ISF-COMPLETENESS.5.i`'s gate refuses a caption
  candidate that is neither a declared signal nor a column (24 encode a structure or message field and 22 are USB
  descriptor layouts, both `DOC-INTENT-TAXONOMY.4b`; 14 captionless continuations; 6 name the encoded quantity, the
  recall the gate gives up by design); 2 are multi-key (`KG-ISF-COMPLETENESS.5.xiii`); 3 C2C tables mint and are
  withdrawn later, 1 has every row refused, 1 holds its body in header rows.
  **The other 43, first read** (table by table in the [record](../research/category-family-census.md)): 3 I2C timing
  tables in operating-mode column groups (`.3.f`), 3 TMC two-row signal tables (`SIGNAL-DECLARATION-ROW-DROP.6`), 5
  property tables (`KG-ISF-COMPLETENESS.2a.vii.c`), 7 structure layouts and status words (`DOC-INTENT-TAXONOMY.4b`),
  5 physical pinouts, legends and one grid-filtered table (no owner: not intent the contract lowers).
  **Remaining here:** the 5 encoding leftovers, 9 header fragments whose body is elsewhere, 3 encodings under the
  signal-description label, and 7 unread tables. A first read of the
  signal-description class finds mostly other kinds under that label (AXI property and presence tables, AHB `HBURST`
  and `HPROT` encodings, USB structure layouts and port-status bit fields, connector pinouts) and three genuine TMC
  signal tables; five USB timing tables hold their body in header rows.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.e`
  Status: `open` (`2026-09-26`; CODE; found by `.3.c`)
  Goal: **an encoding table that minted a symbol counts as carried.** The captured-region accounting
  (`SemanticIr::cited_table_ids`) reads table ids from register, timing, polarity and interface-signal records. A
  symbol definition carries none: its members cite synthetic `Enum X M = v.` statements with no span. So 15
  encoding tables whose enum reaches IntentIR (AXI 13, CHI C2C 2) are listed as
  `no_canonical_carrier_for_captured_region`, and the census over-counts uncarried intent. Acceptance: the minting
  table's id travels with the symbol (record or member provenance) and the accounting reads it; exactly those 15
  residuals go; the new provenance is added through the proof-carrying record contract, not beside it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d`
  Status: `open` (`2026-09-26`; MEASUREMENT; found by `.3`)
  Goal: **decide what the unread assumptions and behaviors are.** 167 of 187 IntentIR assumptions and 288 of 2,265
  behaviors cite no statement a typed record cites. A sample mixes software and debugger obligations (*the debugger
  must poll `dmcontrol`…*, *an entry marked not present must be skipped*), treatment rules (*a WriteDeferrable
  transaction must be treated as 64-byte atomic*), environment statements and conventions (*the latencies assume an
  L1 hit*, *shaded areas are undefined*). Classify them; route hardware intent to a carrier's owner, software intent
  to the contract families that name it (`software_architecture_semantics`, `programming_model_and_behavior`, which
  have no IntentIR surface), and record the rest as non-contract.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f`
  Status: `open` (`2026-09-26`; DESIGN + CODE; found by `.3.c`)
  Goal: **a timing limit keeps the operating mode it holds for.** A table that gives MIN/MAX per operating mode
  (I2C UM10204 rev 7: *Standard-mode / Fast-mode / Fast-mode Plus*, and *Cb = 100 pF / 400 pF* for Hs-mode) stays
  classified as timing and reaches no record, because `TimingConstraintRecord` is scalar and flattening would lose
  which limit belongs to which mode. `CORPUS-COVERAGE.2.47a` documented this (the book's *scalar timing records
  require scalar values*) and named no owner. Measured: I2C `table_0014` (11 rows, electrical), `table_0015` (17
  rows: `fSCL`, `tHD;STA`, `tLOW`, `tHIGH`, … — the protocol's core timing), `table_0017` (17 rows, Hs-mode by bus
  load); the single-mode `table_0016` already yields 17 constraints. Acceptance: a dimension-preserving timing record
  (the mode as the table's column-group header states it), carried through SemanticIR and IntentIR; the adapter's
  use or non-use stated; I2C's gold and WIRE-BASED-100 re-scored.

- ID: `EXTRACTION-REPRESENTATION-GAPS.4`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **FSMGen can state an optional signal.** AXI sets 14 width parameters to `0` by default (re-derived for the
  bundle; `.0` wrote 20, which counted parameters with no default too) (*the signal
  is absent unless enabled*) and gates signals on yes/no properties (`PROT_Present`). FSMGen's published
  contract refuses a zero-valued parameter as a width (13a, 13k *Interface ports*) and has no optional or
  conditional port. Draft, reproduce and sign off one `feature_request` covering a zero-width (absent) port, a
  port present only when a parameter enables it, and the value an absent input is tied to (AXI: *an attached
  Subordinate must have its AWSNOOP input tied LOW* when the Manager omits it; found by `.2`).
  **Bundle signed off `2026-09-25`**: [`sf-isf-optional-signal`](../upstream/fsmgen/sf-isf-optional-signal/README.md),
  forwarded by the owner `2026-09-25`. Reproduced on the pin `54ac1c22` and upstream `354844fe1` (FSMGen's feature
  matrix is byte-identical at both, so nothing has shipped since the pin). Requests R1 zero width = absent, R2
  `(present P)`, R3 `(absent-value V)`, R4 (optional) presence kept across instantiation. Measured need: AXI 14
  width parameters default to 0 (21 signals); 18 tie-off statements across AXI/AHB/APB. **Found while
  building it**: strict mode accepts an unknown port clause and ignores it (`(present PROT_Present)` and
  `(bogus-clause xyz)` both pass and generate the same SystemVerilog as no clause); owned as `.19`. This leaf
  closes when FSMGen ships and SPECFORGE integrates, or the request is rejected or withdrawn. SpecForge halves: `KG-ISF-COMPLETENESS.2a.vii.c.vi` (read the
  presence properties); the adapter's positive-width substitution is replaced once the feature ships.

- ID: `EXTRACTION-REPRESENTATION-GAPS.5`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **FSMGen can carry a parameter's legal values and recommended range.** SpecForge now carries
  `legal_values` and `recommended_values` on 36 parameter symbols (`KG-ISF-COMPLETENESS.2a.vii.c.ii`/`.c.iii`);
  FSMGen's `(params (P default))` holds a default only (13k *Actor report metadata and params*), so the rest
  stops at IntentIR, and nothing checks that an override is legal. Draft, reproduce and sign off a
  `feature_request` for a declared value set / range on an actor parameter, checked on every default and
  override.
  **Bundle signed off `2026-09-25`**: [`sf-isf-parameter-value-set`](../upstream/fsmgen/sf-isf-parameter-value-set/README.md),
  forwarded by the owner `2026-09-25`. Reproduced on `54ac1c22` and `354844fe1`: every spelling is refused with
  *params entries require '(NAME value)'* (13k *Actor report metadata and params*). R1 `(values …)`/`(range …)`, R2
  checked on overrides, R3 reported in `actor_params[]`, R4 (optional) advisory `(recommended …)`, R5 (optional) an
  HDL elaboration check. Measured need: 36 parameters (AXI 28, AHB 7, APB 5 1).

- ID: `EXTRACTION-REPRESENTATION-GAPS.6`
  Status: `open` (`2026-09-25`; REQUEST)
  Goal: **a generated block stays generic in its width parameters.** FSMGen gives ports *resolved positive
  integer widths* and lists *parameter-driven shape elaboration* as backlog (13k *Interface ports*, *Reusable ISF
  libraries*), so the HDL is fixed at the declared default and a different width means regenerating.
  Specifications describe generic blocks (`HRDATA` is `DATA_WIDTH` bits). Draft, reproduce and sign off a
  `feature_request` for HDL whose port widths remain module parameters, citing FSMGen's own backlog entry and
  checking FSMGen's feedback records for an earlier request first.
  **Bundle signed off `2026-09-25`**: [`sf-isf-generic-port-widths`](../upstream/fsmgen/sf-isf-generic-port-widths/README.md),
  forwarded by the owner `2026-09-25`. Sharper than expected: the generated module DECLARES `parameter DATA_WIDTH =
  32` but sizes the port `output reg [31:0] WDATA` (register and reset too), and a default of 64 gives `[63:0]`;
  the width follows the default, never the parameter, so an instantiation override silently does nothing.
  R1 ports follow the parameter, R2 overrides re-specialize, R3 (if R1 is declined) no misleading parameter, R4
  (optional) `+size` records the dependency. The base case `sf-isf-derived-width-parameter` presumes. 62 ports.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a real specification's clock and reset reach the system contract.** 0 of 27 live chains carry a
  `system_contract`, so `assess_isf_renderability` blocks all 27 and no current chain emits an `.isf` file; the
  carrier the adapter reads for clock/reset distribution (`infrastructure_signals`) is empty on all 27 too.
  `build_system_contract` (`ir/semantic.rs`) reads only canonical `Clock X.` / `Reset X …` statements and needs all
  seven fields (clock, reset, kind, polarity, assertion and release timing, target); no rule derives them from a
  signal table (`ACLK | Clock source | Global clock signal`, `ARESETn | … | active-LOW`). FSMGen can state it
  (13a *Clock and reset*: `(reset (rst_n async active_low))`). Measure what the 7 signal-bearing live documents
  state, decide which fields the adapter truly needs, and derive the contract from the document's own grammar.
  Closes when `.7.a`-`.7.e` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.a`
  Status: `done` (`2026-09-25`; MEASUREMENT; SpecForge)
  Goal: what each signal-bearing live document states about its clock and reset (persisted EvidenceIR, HEAD
  `01ed8d80`). The adapter needs four fields: clock, reset, kind (`sync`/`async`) and polarity. FSMGen's
  clause (13a *Reset*) has no fifth, and the other three contract fields follow from the kind.

  | Document | Clock | Reset | Polarity stated | Kind stated | Contract today |
  |---|---|---|---|---|---|
  | AXI | `ACLK` (table) | `ARESETn` (table, prose) | active-LOW (prose `0549`) | asserted asynchronously, deasserted synchronously (`0549`) | none: the table cell states no kind |
  | AHB | `HCLK` (table) | `HRESETn` (table, prose) | active LOW (table `0246`, prose `0855`) | the same (`0855`) | none, as AXI |
  | APB-E | `PCLK` (table) | `PRESETn` (table) | active-LOW (table `0164`) | **not stated** | none |
  | Wishbone | `CLK_I` (prose `0385`) | `RST_I` (prose `0388`) | only as a rule over every interface signal (`0380`) | "purely synchronous" (`0433`) | none: no signal table |
  | ADIv6 | `TCK` and `SWCLK`, one per port | none for the interface; *the SWD interface does not include a reset signal* (`1957`) | — | — | none |
  | I2C | `SCL`, `SCLH`, `USCL`, one per mode | none | — | — | none |
  | I2S | `SCK` | none | — | — | none |

  Only a table cell that states a kind yields a `Reset` statement, and the grammar wants the kind and the polarity
  in one statement. The four AMBA-style documents split them across a table row and a prose sentence.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.b`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **AXI and AHB reach a contract from their own sentences.** The reset grammar accepts a statement
  that gives the kind alone or the polarity alone, and `build_system_contract` merges them, refusing any
  disagreement. The table rule emits the polarity it reads. A prose reader emits the kind and polarity of the
  one declared signal the sentence calls *the reset signal*. It reads *asserted asynchronously* as `async` and
  *asserted synchronously* as `sync`. It refuses an asynchronous deassertion (the model releases on the clock),
  two candidate resets, or both polarities. APB then carries a polarity and no kind, so it is still refused.
  **Landed**: AXI and AHB emit `manager.isf` with `(reset (ARESETn async active_low))` and `(reset (HRESETn
  async active_low))`, strict-clean. Emitting them exposed `.22` (rules drop or invent their condition).

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.c`
  Status: `done` (`2026-09-26`; CODE; SpecForge; checklist below)
  Goal: Wishbone's clock and reset come from prose, since it has no signal table: *The clock input [CLK_I]*,
  *The reset input [RST_I]*, *a purely synchronous protocol*. Its polarity is only stated as a rule over every
  interface signal (*All WISHBONE interface signals MUST use active high logic*). Reading that needs the rule's
  scope (which signals are *WISHBONE interface signals*) to be grounded, not assumed.
  **Landed.** Three readers, each from a sentence the document states: the clock from *The clock input [CLK_I] …*
  (only a clock *input*: I2C's *serial clock signal SCL* is a line its initiator drives, `.7.e`); the kind from *the
  WISHBONE reset [RST_I] signal that uses a purely synchronous protocol*; the polarity from the class rule, which
  reaches `RST_I` only through *the SLAVE interface MUST include the following signals: … [RST_I]* and *WISHBONE
  MASTER and SLAVE interfaces*. A heading (*CLOCK GENERATION*) had also parsed as a second clock and refused the
  contract; a clock is now declared only by a sentence. Wishbone's contract is `CLK_I`, `RST_I` synchronous
  active-high, and it emits `slave.isf`, strict-clean. That file's port set is `.32`'s: *The SLAVE asserts either the
  [ACK_I], [ERR_I] or [RTY_I] signals* grounds the MASTER's port names on the SLAVE.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.d`
  Status: `done` (`2026-09-26`; DECISION + CODE; SpecForge; checklist below)
  Goal: APB-E states its reset's polarity but not its kind. FSMGen's clause has no *kind not stated*: a flat
  `(reset PRESETn)` means `sync`. Decide: an implementation choice recorded as a residual, like the parameter
  defaults of `KG-ISF-COMPLETENESS.2a.vii.c.iii`, or a request for an unstated kind.
  **Decision (`2026-09-26`): an implementation choice, listed**, as `.4` already does for a width the document
  leaves open. The contract records the kind as `not_stated` (its timing and target too); the adapter emits the kind
  that keeps the stated polarity at the pinned FSMGen, where `async` always comes out active-low and `sync`
  active-high (`.21`), so `async` for APB-E's active-LOW `PRESETn`, and lists `isf_reset_kind_not_stated`. A request
  for an unstated kind is not raised now: once `.21`'s fix is published either kind keeps the polarity, and the
  residual already says the kind is not the document's. **Landed.** APB-E emits `requester.isf` for the first time,
  strict-clean. Emitting it exposed one lowering defect, fixed here: a windowed event lowered as `(monitor (within
  SIGNAL N))` ignored its value, so APB's *PADDR, PWRITE, and PWDATA must be valid* became *PADDR is true within a
  cycle*; a windowed event now lowers only when its value resolves to `1`. The three defects behind those rules are
  `.29`-`.31`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e`
  Status: `done` (`2026-09-26`; DECISION; SpecForge; implementation `.7.e.i`)
  Goal: the documents with no system reset (I2C, I2S, ADIv6's SWD port) and those with one clock per port or
  mode (ADIv6, I2C). FSMGen gives an actor that omits `(reset ...)` the reset `rst_n async active_low`
  (13a *Actor Declaration*): SPECFORGE must never omit the clause and let that default stand for the document.
  13a also says *each domain owns zero or one reset* under `(clock-domains ...)`, which may state a reset-less
  clock without inventing one. Decide the route, and whether FSMGen needs a single-clock reset-less form.
  **Decision (`2026-09-26`).** Measured on the persisted IR: I2C, I2S and ADIv6 yield no system contract, and their
  clock-like signals are protocol wires the initiator drives (I2C *the controller's SCL output*; ADIv6 *Signal SWCLK
  is output*; I2S relates no actor to `SCK`). (1) **No FSMGen request.** FSMGen's published contract already states a
  clock with no reset: a single-domain `(clock-domains (domain … (clock C) :default))` block, whose *clock-only no-reset
  domains* lower through the single-clock path (13a *Multi-Clock Domains*). SpecForge emits that block for a document
  that states no reset, and never omits `(reset …)` to let `rst_n` default in. (2) **The initiator's own clock is not
  the protocol clock it drives.** No document states it, so it is an implementation clock, chosen and listed, as `.7.d`
  lists a chosen reset kind; a target takes the protocol clock as its clock only where the document says it samples
  on it. (3) **Implementation waits on the actor graph.** Emitting these documents now would state their current actor
  graphs: I2C's initiator is the phrase *optional open-drain*, and I2S relates no actor to its clock
  (`KG-ISF-COMPLETENESS.1`, agent-surface fidelity). The route is `.7.e.i`, after their initiators are grounded.

- ID: `EXTRACTION-REPRESENTATION-GAPS.7.e.i`
  Status: `open` (`2026-09-26`; CODE; SpecForge; after `KG-ISF-COMPLETENESS.1` grounds these documents' initiators)
  Goal: **a document with no reset, and an initiator whose clock the document does not name, lower as `.7.e` decided.**
  Record in the system contract a clock with no reset (I2S `SCK`, SWD's `SWCLK`), and which actor drives the clock;
  emit the single-domain `(clock-domains …)` block with no reset; for an initiator that drives the protocol clock,
  choose an implementation clock and list it. One clock per port or mode (ADIv6's `TCK` and `SWCLK`, I2C's `SCL`,
  `SCLH`, `USCL`) is one actor per port or mode, each with its own domain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.8`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect, ACTIVE; found by `.2`, widened the same day)
  Goal: **a drive is lowered only from a level the document states, resolved through the signal's polarity.**
  Found as a latent case: `isf_literal_value` (`ir/isf_ir.rs`) maps `asserted` to `1` for every signal and the
  adapter never reads `signal_polarities` (51 records), so an active-low rule would lower inverted (0 live rules
  reach it). **Widened on reading the lowering loops: the defect is active and broad.** Of the 250 drive rules in
  the live adapters' `.isf` text (6 documents), the signal-constraint loop ignores `constraint_kind` and defaults a
  missing value to `1`, the invariant loop does the same, and IntentIR's
  `signal_constraint_to_temporal_invariant_kind` folds `MustBeHigh`, `MustBeLow`, `MustBeAsserted`,
  `MustBeDeasserted` and `MustBeValue` into one `MustBeAsserted` without carrying the level or `negated`; the
  conditional-rule loop drives `1` unless the action text contains `not`. Measured: AXI *AWLOCK must be
  deasserted* and *AWSTASHLPIDEN … must be LOW* are emitted as `(AWLOCK 1)`, `(AWSTASHLPIDEN 1)`; APB *PADDR must
  remain stable* and *PAUSER must be valid* as `(PADDR 1)`, `(PAUSER 1)`; AXI *AWCMO must be 0b000* as `1`; 46 of 53
  lowered conditional rules have the action text `must` alone. FSMGen strict accepts all of it (valid syntax), so
  only reading the values shows it; no `.isf` file is emitted today (`.7`), which is why this must land before `.7`.
  Fix: IntentIR invariants carry the stated level and negation in `target_value`; every adapter path derives a
  drive's value from a stated level (`1`/`0`, `asserted`/`deasserted` through polarity, a stated literal) and
  otherwise records the obligation in one counted residual, never a default; an unresolvable polarity in a guard
  residualizes the rule rather than dropping the guard.
  **Landed.** IntentIR: `signal_constraint_level` (`ir/intent.rs`) puts the stated level, negation applied, in
  each signal-constraint invariant's `target_value` (`0`, `1`, `asserted`, `deasserted`, or the stated value).
  Adapter (`ir/isf_ir.rs`): the three rule loops became `lower_obligations`, whose value comes from
  `signal_constraint_level`, the invariant's `target_value`, or `action_stated_level` (the action's own `be <level>`),
  resolved by `resolve_level` through `signal_polarity_map`; `guard_from_candidates` and `drive_level` do the
  same for temporal contracts in both `classify_actor_contract` and its test oracle; `isf_literal_value` is gone.
  What is refused lands in `isf_obligations_not_lowered`. **Result** on the 6 chains whose IntentIR changed (their
  sealed IntentIR no longer verified under the new binary, so each was rebuilt: `intent` → validate → `adapt` →
  validate): drive rules **250 → 95**; AXI 149 → 82 with **44 values corrected** (`AWLOCK`, `AWSTASHLPIDEN`,
  `AWSTASHNIDEN`, `AWSTASHNID`, `AWIDUNQ` 1 → 0; `AWCMO` 1 → `2'd0`) and 18 rules that now lower because their
  stated value no longer collides with an invented one (`SYSCOACK 0`); APB 38 → 5 (`PSTRB` 1 → 0); AHB 32, ADIv6 5
  and Wishbone 10 → 0, every one of them invented; I2C 11 → 3. 166 obligations are listed instead (53 for an
  unstated polarity, 63 with no level, 61 naming a word); conflicting drives fell 38 → 20 and rule/transaction
  overlaps 7 → 3. EvidenceIR was already right: it refines `asserted` only through a resolved polarity
  (`apply_signal_polarity_to_constraints`); the adapter was the only stage that guessed. **Found while landing**:
  6 SemanticIR invariant ids name two records each (`.18`), and recovering the 53 polarity refusals needs polarity
  extraction (`.17`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.9`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **a value word lowers to the value it names.** Five rule values are residualized as *not an ISF literal*
  although FSMGen accepts them: `ZERO` (2; the number 0) and the enum members `INCR` (2) and `SHAREABLE` (1)
  (13k *Enum member values*: enum members are legal rule and drive values). Map number words to numbers and a
  value naming a member of the target signal's own enum to that member, declaring the enum. `ARDOMAIN` *Must be
  Shareable (0b01 or 0b10)* names a set of two encodings, so it is a membership assertion (`.11`), not a drive.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`; `.10.a`-`.10.f` and `.10.h` done, `.10.g` open)
  Goal: **a normative constraint keeps its condition, and is lowered as what it is.** 38 rules are dropped as
  conflicting drives under an empty guard (`AWSNOOP` 0 vs 1, `AWBURST` INCR vs WRAP) and 7 more overlap a
  transaction's drive: the source states each value under a condition that the rule lost. Read, not assumed:
  `AWSNOOP` 0 (`dyn_sigcon_0068`) comes from *"A Manager that only uses Opcodes where AWSNOOP is LOW can set
  AWSNOOP_WIDTH to 0 which omits the AWSNOOP output … An attached Subordinate must have its AWSNOOP input tied
  LOW"*: a configuration condition and a tie-off for an absent signal (`.4`), not a drive. Classify every dropped
  rule's lost condition (configuration, transaction, phase) before designing the fix. Recover the conditions; and decide, from the lowered actor's perspective, whether a constraint on a
  signal it drives is a drive, and one on a signal it reads an `assume`, rather than lowering every constraint as
  a drive. **Measured `2026-09-25` (`.27.f`)**: 18 of the 80 rules in AXI's `manager.isf` drive a port the Manager
  declares as an input (`ACTIVATEACK`, `ACWAKEUP`, `ASKSTOP`, `BCOMP`, `SYSCOACK`: another actor's obligation, e.g.
  *ASKSTOP must be LOW when ACTIVATEACK is LOW*, which the Subordinate meets); 14 did before `.27.f` corrected
  `ASKSTOP`'s direction. FSMGen `--strict --check` accepts all of them, so only reading the ports against the rules
  shows it. Closes when `.10.a`-`.10.h` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.a`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **an obligation on a signal the actor reads is not lowered as its drive.** Such an obligation is met by the
  actor that drives the signal; from the lowered actor's side it is an assumption, which `.isf` states only inside a
  transaction until FSMGen answers `.15`. **Landed**: `lower_obligations` and the temporal-contract rule path refuse an
  obligation whose target the actor declares as an input (`DrivesAnInput`), listed with the other obligations not
  lowered. AXI's `manager.isf` drops exactly its 18 such rules (80 → 62), none of which the Manager meets; AHB's has no
  rules; APB-E, AHB and Wishbone only re-classify obligations already listed. FSMGen `--strict --check` 0 diagnostics.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.b`
  Status: `done` (`2026-09-25`; MEASUREMENT; SpecForge)
  Goal: **the rest of `.10`: conflicting drives and overlaps, and the conditions they lost.** Re-measure `.2`'s 38
  conflicting drives under an empty guard and 7 rule/transaction overlaps after `.8`, `.22`, `.25`, `.26` and `.10.a`;
  classify each remaining one's lost condition (configuration, transaction, phase), recover the conditions the
  document states, and list the rest.
  **Measured** (the persisted adapters at `c5292610`): only AXI has any, 18 rule conflicts and 3 rule/transaction
  overlaps (from 45). Every one pairs obligations the sentence conditions, read with no condition:

  | Class | Constraints | Example | Owner |
  |---|---:|---|---|
  | a transaction or presence condition not captured | 8 | *If present, AWIDUNQ must be asserted for Prefetch transactions* (6); per-transaction table rows *ARSNOOP \| Must be 0b1110* and *ARLOCK \| Must be 0b0* (2) | `.10.c` |
  | the condition's signal taken as the subject | 2 | *If AWIDUNQ is deasserted for an Atomic request, the corresponding RIDUNQ signal must be deasserted* — recorded on `AWIDUNQ` | `.10.d` |
  | a configuration tie-off read as a drive | 3 | *AWLOCK and ARLOCK are tied LOW* (a compatibility row); *an attached Subordinate must have its AWSNOOP input tied LOW* | `.10.e` (with `.4`) |

  Separately, `ARLOCK` `0B0` against `0` is counted as a conflict although both are the level LOW (`.10.f`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.c`
  Status: `done` (`2026-09-26`; CODE; SpecForge; checklist below)
  Goal: **a constraint keeps a transaction or presence condition its sentence states.** *… must be asserted for Prefetch
  transactions* and *If present, …* carry `condition_text: None`, so the obligation lowers unconditionally and collides
  with the ones for other transactions. Capture the stated condition (a transaction type, a presence clause, the
  transaction a table row belongs to) as the constraint's condition, so lowering lists it (`ConditionNotLowered`)
  instead of driving it always; reading it as a guard (the opcode a transaction type names) is later work (`.10.g`).
  **Landed.** `extract_condition_clause` reads a condition the sentence fronts whole (an inner *when* no longer takes
  the obligation with it), and `stated_condition_clauses` adds a fronted *For <operation>,*, an infix *if*, *for as
  long as* and a trailing *for* phrase that names an operation, never one that counts (*for one response transfer*);
  an infix condition ends at the comma before a later obligation. A two-cell keyed row takes the operation its
  table's caption names (*Table A15.23: DVM Complete request constraints*), whichever producer reads it. AXI's 16 rule
  conflicts fall to 0, and 30 drives true only of a DVM Complete request or a stated transaction leave its
  `manager.isf`; the 3 `AWSNOOP` overlaps are `.10.e`'s.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.d`
  Status: `done` (`2026-09-26`; CODE; SpecForge; `CONSTRAINT-SUBJECT-PRECISION` is closed; landed with `.10.c`)
  Goal: **in *If X is V …, the corresponding Y must be W*, the subject is Y and the condition X is V.** AXI's two
  `RIDUNQ` sentences are recorded as obligations on `AWIDUNQ` with no condition.
  **Landed.** The sentences are list items (*- -If AWIDUNQ is deasserted …*), and the markers hid the fronted *If*
  from `main_clause_of_fronted_conditional` and `fronted_condition_subject`, so the subject search fell back to the
  first signal. `without_list_markers` strips them; the two `AWIDUNQ` records leave, and the `RIDUNQ` ones keep the
  condition.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.e`
  Status: `done` (`2026-09-26`; CODE; SpecForge; with `.4` and `KG-ISF-COMPLETENESS.2a.vii.c.vi`; checklist below)
  Goal: **a stated tie-off is not a drive.** *AWLOCK and ARLOCK are tied LOW* (a row of a property-compatibility table)
  and *an attached Subordinate must have its AWSNOOP input tied LOW* state what an input is tied to under a
  configuration, not what an actor drives. Record them as tie-offs with their configuration, and never as unconditional
  drive obligations. **Measured `2026-09-26` (`.10.c`)**: with the DVM Complete rows conditioned, AXI's `manager.isf`
  now emits the tie-off readings `ARLOCK` 0 (*AWLOCK and ARLOCK are tied LOW*) and `ARSNOOP` 0 (the `ARSNOOP_WIDTH`
  omission) as unconditional drives, 2 rules each, and `AWSNOOP` 0 still overlaps `persistent_cmo_pcmo_transaction`
  (3 residuals).
  **Landed.** A level bound by *tied* in a clause that states no condition is retyped `TiedTo`, whichever producer
  read it (`type_tie_offs`); a *tied* level with a stated scope (*tied HIGH for the entire burst*) stays a hold. No
  stage lowers it: semantic gives it no rule, IntentIR no invariant, and the adapter lists it (*is a tie-off*). Its
  configuration is the signal's own absence on the driver, which the signal's presence condition states
  (`AWSNOOP_WIDTH` 0, `Exclusive_Accesses` False) and `KG-ISF-COMPLETENESS.2a.vii.c.vi` reads; `.isf` states the tied
  value once FSMGen ships `.4`'s absent-input value. AXI's 5 tie-offs (`ARLOCK`, `AWLOCK`, `ARSNOOP`, `AWSNOOP`,
  `WSTRB`) leave `manager.isf` (9 rules) and its 3 rule/transaction overlaps fall to 0. `AWLOCK` 0 is still driven (3
  rules, one constraint), from *AWLOCK must be deasserted, not exclusive access*, a list item whose condition is its
  lead-in's (`.10.h`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.f`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **the same level in two spellings is one drive.** The rule-conflict check compares drive values as text, so
  `ARLOCK` `0B0` and `0` are reported as conflicting; compare the resolved level.
  **Landed.** A first attempt emitted both spellings and FSMGen strict refused the pair
  (`isf_conflicting_rule_writes`: it compares writes as written). So `drive_agreement` separates the same text (both
  emitted, as before), the same number in another spelling (emitted once, the restatement listed as
  `isf_rule_same_level_<name>`), and a different value (a conflict). Both the same-guard and the unconditional-overlap
  passes use it. AXI's `manager.isf` is byte-identical and strict-clean; its two `ARLOCK` conflicts become same-level
  entries (16 real conflicts remain, owned by `.10.c`-`.10.e`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g`
  Status: `open` (`2026-09-26`; MEASUREMENT + DECISION done, CODE in `.10.g.ii`/`.10.g.iii`; SpecForge; found by `.10.c`)
  Goal: **a transaction or presence condition is read as the guard or scope it names.** `.10.c` keeps the condition, so
  the obligation is listed instead of driven always; it is still not lowered. AXI's `manager.isf` lists 20 obligations
  whose condition is not `SIGNAL is LEVEL` (3 before `.10.c`): *DVM Complete request* (a table's rows), *For
  non-stash transactions*, *present on the interface, and for Prefetch transactions*. A transaction condition belongs
  inside the transaction the document names (the opcode that identifies it), a presence condition to the configuration
  that includes the signal (with `.4`). Measure which conditions name a transaction the IntentIR already carries, then
  lower those and list the rest.
  **Measured `2026-09-26`** over the 27 live adapters: 19 obligations are listed for a condition that is not
  `SIGNAL is LEVEL`, AXI 17 and APB-E 2. None can be lowered inside a transaction: AXI's IntentIR carries 14 named
  transactions (from section headings), 4 of which the conditions name (Atomic, Prefetch, WriteZero,
  WriteDeferrable), but none carries the value that identifies it or a body, and `manager.isf` emits none. What
  identifies the transaction is the document's own statement of it, of two kinds:
  - **a signal's level states the transfer kind**: APB's *PWRITE indicates an APB write access when HIGH and an APB
    read access when LOW* (`0170`) makes *For read transfers* and *a read transfer* (`PSTRB` must be LOW,
    `constraint_15`, `_17`) the guard `PWRITE` LOW. Owned by `.10.g.ii`.
  - **an opcode table states it**: AXI's `AWSNOOP` table row `| 0b00111 | NSH, SH, SYS | WriteZero |
    WriteZero_Transaction | …` (`2227`) identifies WriteZero by `AWSNOOP` *and* `AWDOMAIN`, so one signal's value is
    not the key; `InvalidateHint`, `UnstashTranslation`, `Prefetch`, *non-stash*, Atomic (`AWATOP`) and *DVM
    Complete* (`ARSNOOP`) are the same kind. Owned by `.10.g.iii`. 7 of AXI's 17 also carry a presence clause (*If
    present*), which waits for `.4`; and *DVM Complete request*'s 7 rows state the request's own attributes, an
    implication between outputs the Manager chooses together (`.11` lowers it as an assertion, not a drive).
  **Decision.** A condition is lowered only through the document's statement of what identifies the transfer; the
  transaction's name alone lowers nothing. `.10.g` closes when `.10.g.ii` and `.10.g.iii` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.ii`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.10.g`; checklist below)
  Goal: **a transfer kind a signal's level states is a guard.** Read *<SIGNAL> indicates a <kind> <operation> when
  <LEVEL> [and a <kind> <operation> when <LEVEL>]* (APB `PWRITE`, `0170`) as a typed record (kind → signal, level,
  statement), and read a condition that names that kind of operation (*For read transfers*, *a read transfer*) as
  the guard `SIGNAL is LEVEL`, alike in every reader (temporal rule, invariant, constraint lowering). APB-E's two
  `PSTRB` obligations then lower in `requester.isf`.
  **Landed.** SemanticIR's `operation_kinds` (`read` → `PWRITE` LOW, `write` → `PWRITE` HIGH, from `0170`), carried
  by IntentIR; `operation_kind_condition` rewrites a condition naming only that kind of operation before the temporal,
  invariant and constraint readers parse it. APB-E's `requester.isf` gains 5 rules `(== PWRITE 0)` → `(PSTRB 0)`
  (two constraints, two invariants, one temporal rule), strict-clean, and its 2 listed conditions go; the sentence
  states no transfer scope, so none is added. AHB's *When HIGH this signal indicates a write transferandwhenLOWa read
  transfer* (`0265`, fronted, *this signal*, glued text layer) is not read; no AHB obligation names a read or write
  transfer. Registered: inventory 173 → 175 rules, `rule_roots` 122 → 123.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.g.iii`
  Status: `blocked` (`2026-09-26`; MEASUREMENT done; CODE waits for `.4`; SpecForge; found by `.10.g`)
  Goal: **a transaction an opcode table identifies is a guard.** Type the rows of a table that names each transaction
  by the values of declared signals (AXI's `AWSNOOP`/`AWDOMAIN` table `2227`, `ARSNOOP`, `AWATOP`), then read a
  condition naming one of those transactions as the conjunction the row states. Measure first which tables key a
  transaction by one signal and which by several; a presence clause still waits for `.4`.
  **Measured `2026-09-26`**: of AXI's 17 listed conditions, none lowers through opcode identification alone. The 6
  on `AWIDUNQ` (*If present, … for a WriteZero / WriteDeferrable / Prefetch transaction, an InvalidateHint /
  UnstashTranslation request, a write with tag operations …*) also need presence, which waits for `.4`; the 7 rows
  of *DVM Complete request constraints* state the request's own attributes, implications between outputs the Manager
  chooses together (`.11`, as assertions); *For Atomic transactions* (`AWLOCK`) and *For non-stash transactions*
  (`AWSTASHNIDEN`, `AWSTASHLPIDEN`) name a class the document gives only as its complement (*AWATOP is 0b000000
  (Non-atomic operation)*), which reading would take from a word's shape; and *When AWSNOOP is not CMO, WritePtlCMO or
  WriteFullCMO* (`rule_70`, `AWCMO`) is read only as far as its first comma (`.35`). The opcode table is typed
  when an obligation can use it: after `.4`, the 6 `AWIDUNQ` conditions.

- ID: `EXTRACTION-REPRESENTATION-GAPS.10.h`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.10.e`; checklist below)
  Goal: **a list item keeps the condition its lead-in states.** AXI's *For Atomic transactions, the following
  restrictions apply for request attributes:* introduces *- AWLOCK must be deasserted, not exclusive access.*; read
  alone, the item is an unconditional drive of `AWLOCK` 0 (3 rules in `manager.isf`). Measured `2026-09-26` over the
  live chains: 3 constraints whose list lead-in opens with *For*/*When*/*If* and ends in a colon (AXI `AWLOCK` and
  `WTAGUPDATE`, AHB `HSIZE` under *For all Device memory, … the required behavior is:*). Carry the lead-in's
  condition to its items, as `.10.c` carries a table caption's operation to its rows.
  **Landed.** `condition_list_items_on_their_lead_in` gives every list-marked statement after an introducing one the
  condition that introducing sentence fronts, joined to the item's own. Only the sentence ending in the colon counts
  (*It has the following rules:* conditions nothing, so `WTAGUPDATE`'s items, one of which states the opposite scope,
  stay as written), and only a *For* phrase naming an operation or quantifying over *all*/*each*/*every*, or a fronted
  *If*/*When* clause (*For example, …:* is none). Read, not assumed: AXI's two other lists, *When using a Valid-Ready
  transport, the following rules apply:* and *When transporting tags, …:*, condition 4 items that already had a
  condition. AXI's `manager.isf` loses its 3 `AWLOCK` 0 rules; AHB's `HSIZE` keeps *For all Device memory*.

- ID: `EXTRACTION-REPRESENTATION-GAPS.11`
  Status: `open` (`2026-09-25`; CODE; SpecForge; found by `.2`)
  Goal: **stability and exclusion obligations lower as assertions.** 7 rules are residualized as *bare stability
  across tick phases has no supported .isf construct* (`HSIZE must not be changed`) and 3 same-cycle exclusions as a
  0-cycle window (`HEXOKAY must not be asserted in the same cycle as HRESP`). The first claim is stale: FSMGen's
  matrix (13k *Immediate verification checks*) has `(assert …)` with `(stable SIG)`, `(=> A B)` and plain boolean
  exclusion. Lower them as assertions; an actor with no transaction to host one needs `.15`. First make
  `classify_invariant_text` (`ir/intent.rs`) stop labelling unmatched text `MustRemainStable` ("default for chip
  invariants"): lowered as an assertion, that default would state a stability the document never did.

- ID: `EXTRACTION-REPRESENTATION-GAPS.11.a`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.11`; checklist below)
  Goal: **an invariant whose text states no kind says so.** Measured `2026-09-26` over the 27 live chains' IntentIR:
  of 5,162 invariants lifted from SemanticIR's statement invariants, 5,089 match none of `classify_invariant_text`'s
  phrases and are labelled `must_remain_stable` by its default (*The PE cannot service interrupts in Debug state.*,
  *Each access port can drive a different memory space.*); 73 match a phrase. None has a subject signal, so none
  reaches an `.isf`, but each is a stability claim the document never made, and `.11` would lower it as one. Add a
  `NotStated` kind and make it the default.
  **Landed.** `TemporalInvariantKind::NotStated` (`not_stated`) is `classify_invariant_text`'s default; exactly the
  5,089 flip, the 73 that name a kind keep it, and no semantic record, adapter or `.isf` moves.

- ID: `EXTRACTION-REPRESENTATION-GAPS.12`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **a protocol state machine is typed.** 51 protocol records (45 states, 5 operations, 1 edge timing) are
  residualized because they carry no typed transitions, guards, initial state, encoding or actions, and the typed
  state-machine families (`regular_states`, `state_transitions`) are empty on all 27 live chains. FSMGen can state
  state machines and transactions; SpecForge must extract their parts. No open leaf owned this after
  `SWD-SERIAL-EXTRACTION.7d` introduced the residuals.

- ID: `EXTRACTION-REPRESENTATION-GAPS.13`
  Status: `open` (`2026-09-25`; MEASUREMENT; SpecForge, possibly FSMGen; found by `.2`)
  Goal: **every register field and reset value is accounted for.** 158 register fields are not lowered (no bit
  range, a shared name, or overlap) and 71 registers keep a documented reset that is not integer-lowerable
  (symbolic or implementation-defined, partial coverage, over-wide, overlapping). Break both down by cause; the
  capture causes are SpecForge's, and a symbolic or implementation-defined reset value that FSMGen's `(reset V)`
  cannot state becomes an FSMGen request here if the breakdown shows it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.14`
  Status: `open` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; found by `.2`)
  Goal: **platform topology lowers to composition.** `signal_connectivity` (235 records in 6 documents) is never
  read by the adapter, which emits one flat actor; FSMGen composes actors (13f). Measure which documents state a
  multi-component topology and lower it to FSMGen composition where the IR carries it.

- ID: `EXTRACTION-REPRESENTATION-GAPS.15`
  Status: `open` (`2026-09-25`; REQUEST; found by `.2`)
  Goal: **FSMGen can state an always-on assertion outside a transaction.** `(assert …)`, `(assume …)` and `(cover …)`
  are transaction-level clauses (13d, 13k *Immediate verification checks*), although each lowers to an always-on
  clocked property. A protocol invariant (`HSIZE must not change during a burst`) belongs to no transaction. Draft,
  reproduce and sign off a `feature_request` for actor-level assert/assume/cover.
  **Corrected `2026-09-25`**: `.2` wrote that most SPECFORGE actors have no transaction to host one; measured, **1
  of the 7** signal-bearing live documents (I2S) has none, and the emitted property is always-on with no
  transaction-state gating, so elsewhere hosting is placement only. The need is that one actor class plus not
  misattributing an invariant to its host. **Bundle signed off `2026-09-25`**:
  [`sf-isf-actor-level-assertion`](../upstream/fsmgen/sf-isf-actor-level-assertion/README.md), forwarded
  by the owner `2026-09-25`; reproduced on `54ac1c22` and `354844fe1` (*unknown actor clause 'assert'*, also in an actor with no
  transaction). FSMGen's own guidance supports it: obligations belong in the verification family, not a
  transaction body. R1 actor-level placement, R2 documented placement semantics, R3 (optional) a clock domain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.16`
  Status: `open` (`2026-09-25`; REQUEST; found by `.2`)
  Goal: **FSMGen can say "X holds a known value when Y".** Specifications state validity obligations (`PADDR,
  PWRITE and PWDATA must be valid` when `PSEL` is asserted; 11 residualized records, and many more captured as
  constraints). FSMGen's property predicates are `stable`, `changed`, `rose`, `fell` and `past` (13k); there is no
  known-value (`$isunknown`-style) predicate. Draft, reproduce and sign off a `feature_request`. SpecForge's half,
  never emitting `VALID` as a value, is `KG-ISF-COMPLETENESS.2a.vii.e`.
  **Bundle signed off `2026-09-25`**: [`sf-isf-known-value-predicate`](../upstream/fsmgen/sf-isf-known-value-predicate/README.md),
  forwarded by the owner `2026-09-25`; `(known PADDR)` and `(! (isunknown PADDR))` refused on `54ac1c22` and
  `354844fe1`, `(stable PADDR)` accepted in the same position. R1 `(known SIG)`, R2 (optional) `(isunknown SIG)`, R3
  (optional) several signals. Measured need: 28 *must be valid* statements, 17 typed obligations, 11 held-back
  temporal rules.

- ID: `EXTRACTION-REPRESENTATION-GAPS.17`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.8`; checklist below)
  Goal: **every signal's polarity the document states reaches the IR.** After `.8`, `asserted`/`deasserted` lower
  only through a stated polarity, and the 27 live chains carry 51 polarity records across 4 documents (5 of them
  active-low). Measure where the documents state polarity (an `active-LOW`/`active-HIGH` cell or phrase per
  signal; a document-wide statement such as *signals are active-HIGH unless stated*) and read it: a stated
  convention is document grammar, not spelling (ADR 0037 still forbids inferring it from a `…n` suffix).
  Found by `.7.c` (`2026-09-26`): Wishbone's *All WISHBONE interface signals MUST use active high logic*, grounded
  through its role sections, states every interface signal's polarity; only the reset reads it today, so
  `master.isf` still lists *MASTER interfaces MUST assert [CYC_O]* as a polarity unknown.
  **Measured `2026-09-26`** over the 27 live adapters: held back because *the document never states the signal's
  polarity* are 19 obligations (AXI 17, Wishbone 2), 16 temporal rules (AXI 14, Wishbone 1, AHB 1) and 1 AXI guard.
  Over all 78 persisted evidence chains, five documents state polarity for a scope of signals, not one:
  Wishbone's class rule above; AHB *The reset signal, HRESETn, is the only active LOW signal in the protocol*
  (an exclusive statement: every other signal of the protocol is active-HIGH); ADIv6 *All signals that are
  described in this section are active-high* (B2.3.2; a section rule); Avalon *In the following table, all
  signal roles are active high*; GIC-400 *All PPI signals are active-LOW level-sensitive* (GIC-600 states it
  only as a default that parameters invert). The last three are not among the 27 live chains. Wishbone's class membership cannot come from its minimum lists
  alone: *As a minimum, the MASTER interface MUST include … [CYC_O] …* is split across two statements, so the
  list reads only `ACK_I`; the role sections `.32` reads place every MASTER and SLAVE port. AXI states no scope
  rule: its only convention is *Lowercase n … denotes an active-LOW signal*, a spelling rule ADR 0037 forbids
  reading, so its 17 obligations, 14 temporal rules and guard stay held back (honest residuals). This slice reads the class rule (members from the
  minimum lists and the role sections), the exclusive statement (over the declared catalog) and the section
  rule (over the signals the section's statements name). Avalon's table rule and GIC's PPI class name members
  this reader does not ground, and are owned by `.17.a`.
  **Landed.** `scope_rule_polarity_observations` joins the prose polarity strategy: the class rule reaches 23
  Wishbone signals (members from the minimum lists and the MASTER and SLAVE role sections, not SYSCON's), the
  exclusive statement the other 40 AHB signals, the section rule ADIv6's `CSYSPWRUPACK` (B2.3.2's other three wires
  are rows the catalog does not declare, `SIGNAL-DECLARATION-ROW-DROP.1e`). Live records 51 → 115 in 5 documents,
  active-low 5 → 5, no conflict. The reset's class-rule statement now follows the resolved record, so a contested
  reset gets none. AHB's `HEXOKAY` and Wishbone's polarity refusals clear; the 17 AXI obligations, 14 temporal rules
  and 1 guard stay. **Found here and fixed**: Wishbone's `CYC_O` constraint was *[CYC_O] MUST be asserted no later
  than the rising [CLK_I] edge that qualifies the assertion of [STB_O]*, an order between events read as a level;
  with its polarity read it lowered as three always-HIGH `(CYC_O 1)` rules. A level bounded *no later/earlier
  than* an event now mints no constraint (`states_a_level_bounded_by_an_event`); `master.isf` is byte-identical to
  before, and reading the order is `.33`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.17.a`
  Status: `open` (`2026-09-26`; MEASUREMENT then CODE; SpecForge; found by `.17`)
  Goal: **a polarity stated over a table's rows or a named class of signals reaches each member.** Avalon's *In
  the following table, all signal roles are active high* scopes the rows of the table after it, and GIC-400's
  *All PPI signals are active-LOW level-sensitive* scopes a class the document lists in *Table 2-3*. Neither is
  among the 27 live chains, and their persisted artifacts predate the current readers (Avalon's holds a polarity
  record for a catalog entry `ROLES`, read from that very sentence), so first rebuild each from its source, then
  measure which declared signals each scope names and read the scope only where a table or list names its members.
  GIC-600's *PPI signals are active-LOW by default* is a configurable default, not a fixed polarity: it stays
  unread.

- ID: `EXTRACTION-REPRESENTATION-GAPS.18`
  Status: `open` (`2026-09-25`; CODE; SpecForge integrity; found by `.8`)
  Goal: **an invariant id names one invariant.** 6 SemanticIR invariant ids occur twice in 4 live chains (AXI
  `invariant_completion_0b0_completion_not_required` from `statement_4186` and `statement_4289`; ADIv6 two, each
  a list item and its table cell restating one sentence), and IntentIR carries both records under one
  `tinv_…` id. An id that names two records breaks every consumer that keys by id (found when a comparison
  keyed by it compared the wrong pair). Mint distinct ids, or merge restatements into one record with both
  provenances, and gate uniqueness at validation.

- ID: `EXTRACTION-REPRESENTATION-GAPS.19`
  Status: `open` (`2026-09-25`; QUESTION; found by `.4`)
  Goal: **learn whether FSMGen's strict mode should reject an unknown interface-port clause.** Building `.4`,
  `(output AWPROT (width 3) (present PROT_Present))` and `(… (bogus-clause xyz))` both passed `--strict --check`
  with no diagnostic, and the generated SystemVerilog was identical to the port without the clause (pin
  `54ac1c22` and upstream `354844fe1`). FSMGen's book says strict mode narrows *"toward the supported intentional
  surface"* (10) and lists the port grammar (13k) but does not say an unknown clause is an error, so under
  `PROCESS.md` §1 this is a question, not a bug: file it in `docs/FSMGEN_FEEDBACK.md`. It matters to SPECFORGE
  because a strict pass cannot then prove that a clause SPECFORGE emits was understood.
  **Filed `2026-09-25`** as the open question `strict-unknown-port-clause` in `docs/FSMGEN_FEEDBACK.md`, forwarded
  by the owner `2026-09-25`. **Also here**: the ledger's contract capped open records at 8, which the five requests
  had just reached; raised to 12 (`doctrine/live_document_size/fsmgen_feedback.json`), because ADR 0053 expects
  more concurrent requests and the file sits at 173 of 256 lines and 9,674 of 32,768 bytes of its health target
  (an open record is about 9 lines, so 12 records stay under the line target). Records leave the open list when a
  request is integrated, rejected or withdrawn. This leaf closes when FSMGen answers and the answer is recorded.
  **Packet since `2026-09-25`** (ADR 0054): [`sf-isf-strict-unknown-port-clause`](../upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md),
  which adds the `bogus-clause` probe and shows each generated module equal to the clause-free one.

- ID: `EXTRACTION-REPRESENTATION-GAPS.20`
  Status: `open` (`2026-09-25`; REQUEST; found reading FSMGen's 2026-06-16 answer while building `.15`)
  Goal: **FSMGen can carry a transaction's phase membership.** SPECFORGE extracts, per transaction, which signals
  each protocol phase involves (`phase_membership`, `KG-ISF-TRANSACTIONS.2i`): 18 transactions, 14 phase groups, 105
  port memberships over five documents; only 2 of the 18 have grounded steps, and the adapter emits a transaction
  only with steps, so the membership of the other 16 reaches no `.isf`. `.2`'s census missed it because the
  `transactions` family is one the adapter reads. On `2026-06-16` FSMGen answered SPECFORGE that *"checked
  transaction phase-group metadata"* is *"the right future ISF shape"* *"if this becomes load-bearing for
  `.isf`"*; this measures that it is, and files the request.
  **Bundle signed off `2026-09-25`**: [`sf-isf-transaction-phase-groups`](../upstream/fsmgen/sf-isf-transaction-phase-groups/README.md),
  forwarded by the owner `2026-09-25`; `(phases …)` refused as an unsupported body clause and the actor-level spelling
  as an unknown actor clause on `54ac1c22` and `354844fe1`. R1-R3 are FSMGen's own first slice (checked metadata,
  validation, reporting), R4 (optional) a transaction whose only content is its phases. **Also here**: the
  `fsmgen_issue_packets` collection (`docs/upstream/fsmgen/**/*.md`) allowed 32 files and this bundle made 35 (each
  bundle adds a README, a SIGNOFF and expected notes); raised to 96 files with its byte and line totals raised in
  step (96 × the per-file limits, ADR 0032), because ADR 0053 expects many more requests and the collection used
  1,930 of 16,384 lines and 131 KB of 1 MiB, so only the file count bound. Noted for `.19`: unknown
  transaction and actor clauses ARE refused; only port clauses are silently ignored.

- ID: `EXTRACTION-REPRESENTATION-GAPS.21`
  Status: `open` (`2026-09-25`; fixed in FSMGen's unpublished development line; closes with `FSMGEN-REFRESH-INTEGRATE-10`)
  Goal: **an `.isf` reset keeps the polarity it states, whatever its name.** At the pin `54ac1c22` an `async`
  reset always comes out active low: `(rst_n async active_high)` passes `--strict` as active low, and without
  `--strict` so does `(rst async active_high)`. `--strict` also refuses `(ARST async active_low)`, `(nSRST async
  active_low)`, `(rst_b async active_low)`, `(rst async active_high)` and `(rst_n sync active_low)` as a *legacy
  or misleading* spelling. **Not filed**: building the bundle showed upstream HEAD `354844fe1` honours all eight
  probe cases under `--strict`, and exports place the fix at `f16130a4f` (`2026-09-25`, *preserve ISF reset policy
  through lowering*; `07f7e8333` alone did not fix the `.isf` path). A name cannot carry a polarity anyway: the
  owner knows `X_n`, `X_b`, `Xn`, `Xb`, and the corpus adds ADIv6's `nSRST`/`TRST*` and Wishbone's `[#RST_I]`,
  `[/RST_I]`, `[N_RST_I]` (`0381`). AXI's and AHB's resets are unaffected. Closes when the pin includes `f16130a4f`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.22`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect; found when `.7.b` made AXI emit `manager.isf`; checklist below)
  Goal: **a rule keeps the condition its sentence states, or is not lowered.** `sanitize_rule_condition`
  (`ir/isf_ir.rs`) turns a multi-word condition into no guard, so the rule drives unconditionally. It turns a
  one-word condition into `(== <word lowercased> 1)` without checking that the word is a declared signal or that
  `1` is its stated level. Upstream, `synthesize_temporal_invariants` (`ir/intent.rs`) builds an invariant
  without its constraint's condition (24 of 81 constraint-derived invariants, in AXI, APB, AHB and I2C). Over the
  live chains 16 emitted rules drop a stated condition, 8 by each path (AXI `rule_70` *AWSNOOP is not CMO*). AXI
  *SYSCOREQ … must be deasserted when ARESETn is asserted* is emitted twice: guarded `(== ARESETn 0)` from its
  temporal rule, and unguarded from its invariant. 3 rules are guarded on an undeclared name. AXI *ACTIVATEREQ,
  ACTIVATEACK and ASKSTOP must be LOW during reset* became `(rule constraint_45 (== reset 1) (ACTIVATEREQ 0))`,
  and FSMGen then adds an input port `reset` (`.23`). A condition becomes a guard only on a declared signal at a
  stated level (`resolve_level`); *during reset* names the contract's reset (`ARESETn` at its active level);
  anything else is listed with the obligations not lowered.
  **Design (`2026-09-25`, measured)**: of 80 conditions on lowerable records, most are prose (*it is driving the
  final read transfer*, *PSEL and PWRITE are asserted*). One reader, `condition_guard`, turns a whole `SIGNAL is
  LEVEL` or *(in/during) reset* into a guard and refuses the rest (`ConditionNotLowered`). An invariant's
  `condition_signal` keeps its `condition_value`, and a constraint whose condition cannot be read yields no invariant
  (an invariant holds unconditionally). A temporal rule's guard candidates are a conjunction: all are kept, as
  FSMGen's `(& …)`, or the rule is refused. Today `guard_from_candidates` keeps the first usable one; none of the
  24 emitted temporal rules loses one yet, but 4 contracts carry 2-3 candidates.
  **Corrected `2026-09-25` (`.25.a`)**: the claim below missed the temporal path. A temporal rule whose condition
  gave no antecedent carried no trace of it, so 4 emitted rules still dropped theirs (AXI's three *… LOW during
  reset*, I2C's *the Acknowledge cycle*); `.25.a` lists them. The measurement had read the temporal rules' guard
  candidates, not their source constraints' conditions.
  **Landed**: no emitted rule drops or invents a condition (16 and 3 before); AXI 89 → 80 rules, 12 guarded; the
  conjunction exposed a fused contract with contradictory conditions, refused and owned by `.25`; APB's *PSEL is
  asserted* against the declared `PSELx` family is `.24`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.23`
  Status: `done` (`2026-09-25`; no addendum needed — fixed in FSMGen's development line)
  Goal: **the acknowledged `sf-isf-undeclared-rule-operand` fix covers a rule's guard too.** At the pin
  `54ac1c22`, `(rule r1 (== bogus_name 1) (flag 0))` passes `--strict --check` and FSMGen adds `input bogus_name`.
  Upstream HEAD `354844fe1` refuses it (*operand 'bogus_name' read by rule 'r1' guard*), and a binary search over
  exports puts the refusal of both the guard and the report's own value case at `c6b8bc783` (`2026-09-24`). The fix
  already covers guards, so no addendum is sent; it lands with `FSMGEN-REFRESH-INTEGRATE-10`, which also closes
  `FSMGEN-REQUESTS.2`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.24`
  Status: `done` (`2026-09-26`; DECISION `.24.a` and CODE `.24.b` done; SpecForge; found by `.22`)
  Goal: **a signal family with one instance per target is read as one.** APB states *The Requester generates a
  PSELx signal for each Completer* (`0168`); AHB states *Each Subordinate has its own select signal HSELx* (`0277`)
  and *An address decoder provides a select signal, HSELx, for each Subordinate* (`0658`). Per the owner, `PSEL`
  is the generic name and every instance (`PSELA`, `PSELB`, …) follows `PSEL`'s contract, so APB's *PSEL is
  asserted* is a condition on each instance. Today SpecForge declares `PSELx` as one 1-bit port and leaves
  conditions naming `PSEL` unread (`.22`: APB's *PSEL is asserted* yields no invariant). Decide how the generic
  name is tied to its family: the document's own *for each* statement grounds the family, and the identity must
  not come from the trailing `x` (ADR 0037). Then read it, and decide whether FSMGen can state a per-target port
  family or needs a request. Closes when `.24.a` and `.24.b` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.24.a`
  Status: `done` (`2026-09-25`; MEASUREMENT + DECISION; SpecForge)
  Goal: what the live documents state about a per-target family and its generic name, what each stage reads
  today, and the route. **Measured** over the persisted IR of the 27 live chains
  (`.project-data/tmp/erg24/family_census.py`, `partial_conditions.py`, scratch):
  - Per-target statements: APB `0168` (*The Requester generates a PSELx signal for each Completer*), AHB `0277`
    (*Each Subordinate has its own select signal HSELx*) and `0658` (*… a select signal, HSELx, for each
    Subordinate*). No other live document states one (AXI `0715`, *one PENDING signal per channel*, names a set of
    channel signals, not one per target).
  - The generic name: APB writes `PSEL` in 26 statements and `PSELx` in 11; AHB `HSEL` in 5 and `HSELx` in 12.
  - The identity: APB states it, writing the same role phrase before both (*Select signal, PSELx*, *Select
    signal, PSEL*), which `document_signal_coreferences` has read since `WIRE-BASED-100.4a`. AHB never does: its
    `HSEL` sentences (*HSEL asserted if present*, *an HSEL output*, *HSEL to be tied HIGH on the Subordinates*)
    share no role phrase with `HSELx`, and no AHB constraint names `HSEL`.
  - What is read: all 4 APB constraints conditioned on `PSEL` (`sigcon_0007`, `0012`, `0015`, `0016`) reach IntentIR
    as contracts guarded on `PSELx`. `.22`'s *PSEL is asserted yields no invariant* is the invariant reader
    (`extract_condition_from_text`, exact or case-folded only); the contract carries the same condition, so no
    intent is lost, only the two readers disagree. `sigcon_0015` keeps 2 of its 4 predicates, a defect of the
    condition reader, not of the family (`.26`). All 4 state *must be valid*, so none lowers yet
    (`KG-ISF-COMPLETENESS.2a.vii.e`, `.16`).
  **Decision.** (1) A generic name is tied to its family only by what the document states, today a shared role
  phrase. A per-target sentence grounds that one declared signal has an instance per target, but it names one
  identifier, so it ties nothing. ADR 0037 stands: nothing is read from the trailing `x`, and AHB's `HSEL` stays
  unread. (2) The family lowers one instance at a time, each under the generic contract, as the owner stated. An
  actor that reads it (a Completer, a Subordinate) has one instance, under the declared name. An actor that drives
  it (the Requester, a decoder) drives one per target, in a number neither document states; a single-actor `.isf`
  states one, and must say so. (3) **No FSMGen request.** FSMGen's published contract already has the shape: a
  requester declares one `(output PSEL)` (13a's APB example), a multi-peripheral interconnect fans out a decoded
  `PSEL` to each completer child (14h), and AHB decoding emits one `HSEL_*` per mapped window (`HSEL_REGS`, 14i).
  The fan-out is composition, `.14`.
  **Found while measuring**: of the 16 temporal rules whose stated condition yields a guard, 5 keep only part of it
  (`.26`); both emitted `.isf` files declare ports whose direction is a default rather than grounded on the lowered
  actor (`.27`); the ownership-citation registry the ledger uses is at 60 of 64 records (`.28`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.24.b`
  Status: `done` (`2026-09-26`; CODE; SpecForge; checklist below)
  Goal: **a per-target family is typed intent, and every reader resolves the generic name alike.** Record a
  per-target family from the document's own sentence (signal, target role, statements) through IntentIR; persist
  the co-reference once in SemanticIR so IntentIR's invariant reader resolves `PSEL` as the contract path does; and
  have the adapter list a family lowered as one instance on an actor that drives it. Generic: the grammar is *for
  each ROLE*, *each ROLE has its own … SIGNAL*, with ROLE a declared actor; no signal name is read.
  **Design `2026-09-26`.** SemanticIR persists `signal_coreferences` (name, declared signal, the shared role
  phrase, statements) and `signal_families` (signal, the role it has one instance per, statements), computed once;
  `build_temporal_rules` takes the co-references instead of recomputing them. IntentIR carries `signal_families`,
  and its invariant reader resolves a condition's signal through the co-references (APB `sigcon_0007`, *PSEL is
  asserted*, is the one live condition it changes). The adapter lists a family whose signal the lowered actor
  declares as an output. Both fields are skipped when empty, so only APB-E and AHB carry them.
  **Landed.** APB-E: `PSEL` → `PSELx` (role phrase *select signal*, statements 0203, 0211, 0243), family `PSELx`
  per `Completer` (0168), a new invariant `tinv_sc_sigcon_0007` (*PNSE must be valid when PSEL is asserted*), and
  `requester.isf` lists `isf_family_driven_once`; AHB: family `HSELx` per `Subordinate` (0277, 0658), no
  co-reference, no residual (its Manager does not drive `HSELx`). No `.isf` changes. The two SemanticIR fields and
  the IntentIR carry are registered rules (inventory 170 → 173; `rule_roots` 120 → 122), so every live chain's
  proof digest moved and the 27 were regenerated.

- ID: `EXTRACTION-REPRESENTATION-GAPS.25`
  Status: `done` (`2026-09-26`; CODE; SpecForge defect; found by `.22`; `.25.a`-`.25.c` done)
  Goal: **a contract keeps one sentence's conditions, and an *until* clause is not a condition.** AXI's contract
  `fused:contract_temporal_signal_constraint_sigcon_0043+…_0044` joins *When AWAKEUP is asserted with SYSCOREQ
  asserted and SYSCOACK deasserted, it must remain asserted until SYSCOACK is asserted* with its mirror sentence
  (*SYSCOREQ deasserted and SYSCOACK asserted*). Its guard candidates are `SYSCOREQ ASSERTED`, `SYSCOACK
  DEASSERTED` and `SYSCOACK ASSERTED`, the last being the *until* release, and it drives `AWAKEUP HIGH` although
  the sentence says *must remain asserted*, a stability obligation. Until `.22` the adapter kept the first candidate
  and emitted `(== SYSCOREQ 1)`; it now refuses the contradiction. Fix the fusion and the antecedent reading.
  Closes when `.25.a`-`.25.c` close. Measured (SemanticIR): `sigcon_0043`'s condition swallows its obligation
  (*…, it must remain asserted until SYSCOACK is asserted*) and its kind is `must_be_high`; its temporal rule's
  antecedents are `SYSCOREQ ASSERTED`, `SYSCOACK DEASSERTED`, `SYSCOACK ASSERTED`; `sigcon_0044`'s are `SYSCOREQ
  ASSERTED`, `SYSCOACK ASSERTED`, although its text says *SYSCOREQ deasserted*.

- ID: `EXTRACTION-REPRESENTATION-GAPS.25.a`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below; `.25.b` landed with it)
  Goal: **contracts with different stated conditions never fuse.** `merge_cluster` (`ir/fusion.rs`) compares kind,
  obligation and guard, and unions `guard_candidates` without comparing them; `fusion_key` omits the conditions.
  Put the sorted conditions in the key, so fusion merges only one obligation seen twice.
  **Landed**, with what splitting the fusion exposed: a temporal rule whose stated condition gave no antecedent is
  listed by the adapter instead of lowered unguarded (the IR keeps it: a phase condition has no signal predicate,
  as the WIRE-BASED-100 gold records); a clause naming several signals splits at each mention and each signal
  takes its own value (`.25.b`). AXI's AWAKEUP rules become holds, `(& (== AWAKEUP 1) (== SYSCOREQ 1) (== SYSCOACK
  0)) (AWAKEUP 1)` and its mirror; its three *during reset* rules and I2C's one are listed. Every emitted rule on the
  live chains keeps its stated condition as its guard (14) or states none (69).

- ID: `EXTRACTION-REPRESENTATION-GAPS.25.b`
  Status: `done` (`2026-09-25`; CODE; SpecForge; landed with `.25.a`, WIRE-BASED-100 6 of 6 identical)
  Goal: **an *until* clause is not a condition, and each signal takes its own value.**
  `parse_temporal_condition_predicates` (`ir/semantic.rs`) cuts *unless*/*except* but not *until*, and a clause
  naming several signals gives each the first value word (*with SYSCOREQ deasserted and SYSCOACK asserted* read
  SYSCOREQ as asserted).

- ID: `EXTRACTION-REPRESENTATION-GAPS.25.c`
  Status: `done` (`2026-09-26`; CODE; SpecForge; with `.11`; checklist below)
  Goal: **an obligation does not leak into its condition.** *When AWAKEUP is asserted with …, it must remain
  asserted until SYSCOACK is asserted* is a persistence obligation (remain asserted, released by SYSCOACK). The
  constraint keeps it all as the condition, with kind `must_be_high`. Since `.25.a` the lowering is a faithful hold
  (the guard includes `(== AWAKEUP 1)`); the constraint's kind and condition text are still wrong in the IR.
  **Landed.** The condition half landed with `.10.c` (*AWAKEUP is asserted with SYSCOREQ asserted and SYSCOACK
  deasserted*). The kind: 9 constraints on the live chains read *must remain LEVEL until RELEASE* as a constant level
  (8 AXI, 1 APB), and in 5 the release was nowhere in the IR, so lowered they would never release. `type_holds`
  retypes them `MustRemainUntil { value, release }`; semantic reads a release that states a value into the rule's
  exceptions; IntentIR gives a hold no invariant; the adapter refuses it as a constant level, lists a hold whose
  release was not read, and drops an exception the guard already excludes, so AXI's four `AWAKEUP` holds and its
  `manager.isf` are unchanged. The APB WIRE-BASED-100 gold for *PWAKEUP must remain asserted until PREADY is
  asserted …* moves from `must_be_asserted` to `must_remain_until`, as the sentence states.

- ID: `EXTRACTION-REPRESENTATION-GAPS.26`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect; found by `.24.a`; checklist below)
  Goal: **a temporal rule's condition is read whole, or the rule says it was not.** `parse_temporal_condition_predicates`
  (`ir/semantic.rs`) silently keeps part of a condition in 5 of the 16 temporal rules on the live chains whose stated
  condition yields a guard (`partial_conditions.py`). APB `sigcon_0015`, *PSEL, PENABLE, and PREADY are asserted, and PWRITE is
  deasserted*, keeps `PREADY` and `PWRITE`: the shared value is distributed only when the whole condition has one
  value, so the list members before *and PREADY are asserted* are dropped. AHB `sigcon_0007`, `0009`, `0010`, `0011`,
  *when HREADY is LOW, unless HRESP signal is ERROR*, keep `HREADY` alone: the exception is cut (`WIRE-BASED-100.5h`,
  right not to conjoin it as a positive condition, wrong to lose it). Each weakened guard applies the obligation in
  cycles the document exempts; `.11` would lower AHB's two stability obligations with it. None is emitted today.
  Fix: a value-less list member takes the value of the clause that ends its list; an exception becomes a negated
  predicate (FSMGen guards take `(! …)`, 13g) or the rule is marked as not read whole, so the adapter lists it.
  The AHB WIRE-BASED-100 gold labels the `HREADY` antecedent alone (*unless … is an EXCEPTION, not the
  antecedent*), so the gold's intent is read before its key is changed.
  **Landed.** `parse_temporal_condition` reads a condition whole or returns nothing: a list member takes the value
  of the clause that closes its list; an `unless`/`except` clause becomes `TemporalRuleRecord::exceptions` (still
  never an antecedent); a signal left without a value, or an exception naming none, empties the condition, and the
  adapter lists such a rule, now for conditional rules too (`unread_stated_condition`). An exception reaches the
  contract as `Condition::Ne` (so fusion keeps it apart) and the adapter's guard as `(! (== S L))`; an exception at
  the required level is a contradiction and one whose value is no level refuses the rule. The eval key and the LTL
  rendering carry exceptions; the two AHB temporal golds were labelled with theirs, as their notes already said.
  **Corrected `2026-09-25` (`.27.b`)**: this commit left test code that fails `cargo fmt --check` (an edit in
  `adapters.rs` after the last format) and `cargo clippy -D warnings` (a tuple return type in a `semantic.rs` test
  helper, added after the last clippy run). The pre-commit gate runs neither, so only push CI would have caught them;
  `.27.b` fixes both. The checklist's *`cargo fmt` and `cargo clippy -D warnings` clean* was true of the code before
  the last test edits, not of the commit.
  **Result**: all 16 guarded temporal rules keep their whole condition (APB `sigcon_0015` gains `PSELx`, `PENABLE`;
  AHB's four gain `HRESP ERROR`). Nothing emitted changes, because those obligations are refused for their own
  reasons (`VALID`, `.16`; stability, `.11`), and `ERROR` becomes lowerable only with `.9`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.26.b`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.25.c`; checklist below)
  Goal: **a condition's symbolic value is one the clause states.** `temporal_clause_value` (`ir/semantic.rs`) falls
  back to any capitalized word after the signal, so *the associated ACVALID / ACREADY handshake … of the DVM
  transaction* reads as `ACVALID` = `DVM`. `.25.c` found it when reading a hold's release, and reads a release only
  when it states a value (a copula). Measured `2026-09-26` on the live SemanticIR: of the 6 non-level predicates, the
  4 AHB `HRESP` = `ERROR` exceptions are stated (*HRESP signal is ERROR*) and none other remains, so no condition is
  misread today; the fallback stays able to invent one. Take a symbolic value only where the clause states it of the
  signal (a copula or an equality), and re-measure.
  **Landed.** `symbolic_value_is_stated` keeps a symbolic value only after a copula or an equality. SemanticIR
  replay: 27 of 27 identical, as measured; the reader can no longer invent a value.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27`
  Status: `done` (`2026-09-25`; MEASUREMENT then CODE; SpecForge defect; found by `.24.a`; `.27.a`-`.27.f` done)
  Goal: **every port of an emitted actor has the direction the document grounds on that actor, or is listed.** The
  adapter lowers every interface signal into the initiator's actor; a signal the initiator neither drives nor reads
  takes `(output)` and a named drive, and no residual lists it (`initiator_perspective_directions`, `ir/isf_ir.rs`;
  `KG-ISF-COMPLETENESS.2a.ii` called the default an honest residual). It is not neutral: AHB `manager.isf` declares
  as Manager outputs `HSELx` (driven by the decoder, per IntentIR), `HRUSER`, `HBUSER` and `HREADYOUT` (the
  Subordinate). Measured on the two emitted files (`port_grounding.py`): AHB 11 of 39 ports (8 grounded only on
  other actors, 2 the Manager both drives and reads, 1 unrelated), AXI 172 of 294 (169 with no actor relation).
  Measure why (extraction recall versus a signal that belongs to another actor), then emit only grounded ports and
  list the rest. Closes when `.27.a`-`.27.e` close.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.a`
  Status: `done` (`2026-09-25`; MEASUREMENT; SpecForge)
  Goal: why each of the 183 emitted ports lacks a direction grounded on the Manager, from the row that declares it
  (`.project-data/tmp/erg27/classify.py`, scratch: the declaring table's kind, header and `Source` cell, and the
  signal's actor relations in IntentIR).

  | Class | AXI | AHB | What the row states | Owner |
  |---|---:|---:|---|---|
  | a check signal whose check table has no `Source` column | 122 | 0 | the signals it covers (AXI Table A16.4) | `.27.d` |
  | a `Source: Manager` row on a `Continued from previous page` fragment | 42 | 0 | `Manager`, never read | `.27.b` |
  | a `Source: Tie-off` row | 5 | 0 | a tie-off, not an actor | `.27.c` |
  | a relation only on another actor | 2 | 8 | e.g. AHB `HSELx` (address decoder), `HRUSER`, `HBUSER`, `HREADYOUT` (Subordinate) | `.27.e` |
  | a Manager relation that is both input and output | 1 | 2 | AXI `ASKSTOP`, AHB `HTRANS`, `HTRANSCHK` | `.27.e` |
  | no `Source` cell and no relation | 0 | 1 | AHB `HRESET`, a Manager/Subordinate presence-matrix row | `.27.e` |

  The 42 are the clearest: AXI's B1.1 and B1.4 continue on a second page (`table_0251`, `table_0255`), whose header
  is the head's exactly and whose rows state `Source: Manager` (`AWSTASHLPID | 5 | Manager | Stash Logical Processor
  ID`). `continuation_inherited_table_heads` already gives such a fragment its head's kind for the declaration
  gap-fill and the table accounting (card `header-trapped-signal-table-recovery`), but the actor-relation reader does
  not consult it, so the rows declare their signals and relate them to no actor.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.b`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **a continuation page states its source like the page it continues.** The table-source actor-relation reader
  consumes `continuation_inherited_table_heads`, so a fragment with its head's header signature grounds its rows'
  `Source` cells (AXI 42). Measure the corpus-wide reach first; the emitted direction does not change for these
  (they already default to `(output)`), their grounding does.
  **Landed.** Reach (`continuation_reach.py`): only AXI on the live chains, 2 fragments with a relation column
  (`table_0251` 24 rows, `table_0255` 19); the other inheriting fragments are encodings or header-trapped presence
  matrices with no body rows. `extract_relations_from_signal_tables_with_prior_guidance` reads such a fragment in its
  head's context (kind gate, caption, section), citing the fragment. AXI gains 43 `Manager` drives and 43
  complementary `Subordinate` reads, as its head pages already had; the Manager's grounded ports rise 122 → 165 of 294.
  Downstream the rules and contracts on those signals gain their actor (`Manager` drives `AWIDUNQ`; 10 contracts,
  some now `lowerable`), and one real document conflict surfaces: `ARCHUNKEN` has two producers, the Manager (Table
  B1.4) and an interconnect (`1854`). The emitted `.isf` is byte-identical.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.c`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **a signal stated as an actor's input is read as that actor's input.** AXI B1.13's `BROADCAST*` rows state
  `Source: Tie-off`, and each signal's own table says whose input it is (A8.12 *BROADCASTSHAREABLE | 1 | 0b1 | Manager
  tie-off input, used to control the issuing of Shareable transactions*). **Corrected `2026-09-25`**: `.27.a` framed
  these as signals the Manager's interface should stop declaring; the document says the opposite, that they are
  Manager inputs tied at integration, which no actor drives. The emitted `.isf` got four of them right only through
  the flat `input` hint, and declared `BROADCASTCACHEMAINT` and `BROADCASTSHAREABLE` as driven Manager outputs (the
  second on a noise actor `Shareable`, read from *an optional Manager input signal, BROADCASTSHAREABLE*).
  **Landed.** `actor_relation_stated_by_description`: when a signal row's relation column names no actor, a
  description that opens by naming the signal an input or output of an actor the document already names
  (`<actor> [tie-off] input|output`) states that relation, before the section-heading fallback. Reach (AXI only,
  `tieoff_census.py`): 5 `Manager` reads; A14.1's *Manager output, asserted HIGH …* now also states `AWAKEUP` and
  `ACWAKEUP`, which B1.10's `Source` column already gave, so only their citation moves to the earlier page. The
  Manager's grounded ports rise 286 → 291 of 294; `manager.isf` makes `BROADCASTCACHEMAINT` and `BROADCASTSHAREABLE`
  inputs and drops their drives. `BROADCASTATOMIC`'s own table (A6.11) is classed `unknown`, so it keeps only its
  flat hint (`.27.e`). The tie-off's stated value (`Default 0b1`) is `.4`'s concern.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.d`
  Status: `done` (`2026-09-25`; CODE; SpecForge; checklist below)
  Goal: **a check table is read by its covered-signals column, whatever it calls its name column.** AXI Table A16.4
  lists, for each check signal, the signals it covers, and the document says check signals *are added to groups of
  signals on the interface and driven such that there is always an odd number of asserted bits in that group*
  (`4766`). **Corrected `2026-09-25`**: `.27.a` framed this as a decision still to make; it was made long ago.
  `augment_check_signal_relations_from_tables` (`ir/evidence.rs`, test
  `check_signal_tables_inherit_relations_from_covered_signals`) gives a check signal the relations of the signals it
  covers. It never fires on AXI because it finds the check column by a header containing *check signal*, and A16.4
  heads it `Name` (`Name | Signals covered | Width | Check enable`, on its uncaptioned head `table_0245` and four
  continuation pages, 122 rows). Recognise a check table by its covered-signals column, with the row's subject in the
  name column; the direction then comes from the stated coverage, never from the `CHK` spelling (ADR 0037).
  **Landed.** A table with a covered-signals column and no *check signal* header takes the row's signal from its first
  column. Reach: AXI only (123 rows; APB's and AHB's check tables already head the column *Check signal*). AXI gains
  254 inherited relations (Manager drives 82, Subordinate reads 81, Manager reads 42, Subordinate drives 42, and 7 on
  other actors the covered signals were already related to: `interconnect` 3, and descriptor noise such as `Note
  reads` and `exclusive reads`, owned by actor precision in `KG-ISF-COMPLETENESS.1`) and 123 direction declarations; the Manager's grounded ports rise 165 → 286 of 294. The
  emitted `manager.isf` changes as it should: 40 checks of Subordinate-driven signals (`WREADYCHK`, `RVALIDCHK`, …)
  become inputs and lose the drive the default gave them; FSMGen `--strict --check` accepts it with 0 diagnostics.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.f`
  Status: `done` (`2026-09-25`; CODE; SpecForge defect; found by `.27.e`'s re-measurement; checklist below)
  Goal: **an active relation verb's object stops where another agent's clause begins.** The in/out relations `.27.a`
  left were prose misreadings. AHB *The two-cycle response provides sufficient time for the Manager to cancel this next
  access and drive HTRANS[1:0] to IDLE* gave `two-cycle response drives HTRANS` (`provides`'s object ran into the
  Manager's infinitive) and `Manager reads HTRANS` (`access`, a noun after *this next*, read as a verb). AXI *The
  Manager controls the move from RUN to DEACTIVATE, but the Subordinate can use the ASKSTOP signal …* gave `Manager
  drives ASKSTOP`. `two-cycle response` is the Class-A noise `KG-ISF-COMPLETENESS.1` catalogued; no open leaf held
  these instances. **Landed**: the object ends at `but` and at `for <noun phrase of at most three words> to`; a verb
  form after a determiner and a closed-class sequence modifier (`next`, `same`, `subsequent`, …) is a noun. Over the
  27 live chains exactly 8 relations leave, all false (AXI `Manager drives ASKSTOP` and its inherited check; AHB the
  two above and their checks; ADIv6 `following drives In`, `previous reads In`), none is added. The Manager's `HTRANS`
  is a grounded output and AXI's `ASKSTOP`/`ASKSTOPCHK` become inputs.

- ID: `EXTRACTION-REPRESENTATION-GAPS.27.e`
  Status: `done` (`2026-09-25`; MEASUREMENT then CODE; SpecForge; checklist below)
  Goal: **the adapter declares only ports grounded on the lowered actor, and lists the rest.** After `.27.b`-`.27.d`,
  re-measure what remains (AHB's 8 other-actor ports; the in/out relations, where AHB's Manager both drives and reads
  `HTRANS` although its own transaction drives it, so the relation is wrong upstream; the unrelated `HRESET`), then
  emit only grounded ports, list the rest, and keep every transaction and rule that drives a port consistent with it.
  **Re-measured `2026-09-25`** after `.27.b`-`.27.d` and `.27.f`: AXI 293 of 294 ports grounded (`BROADCASTATOMIC`
  keeps only its flat hint: its own table, A6.11, is classed `unknown`); AHB 30 of 39 (8 grounded only on the
  Subordinate or the address decoder: `HSELx`, `HRUSER`, `HBUSER`, `HREADYOUT` and their checks; `HRESET`, a
  presence-matrix row, unrelated). The in/out class is gone. Dropping an ungrounded port can lose one the Manager does
  have but the document never relates to it (AHB states only that the Subordinate is `HRUSER`'s source, not who reads
  it), so the listing must say which ports the document gives to another actor and which it relates to none; a rule
  that drives an input is `.10`'s question.
  **Landed.** With an initiator, the adapter declares a port only with a direction grounded on it; a signal no actor is
  related to keeps its declared direction, read across every interface record that states one
  (`declared_signal_directions`: the first record alone often states none); every other signal is left out and
  listed in `isf_ports_not_on_the_lowered_actor` by reason. AXI's `manager.isf` is unchanged (its one ungrounded port,
  `BROADCASTATOMIC`, keeps its declared input). AHB's loses 9 ports, their 9 drives and `USER_RESP_WIDTH`, which sized
  only `HBUSER`. The blocked chains' adapter text changes too: APB-E leaves out `PCLK`/`PRESETn` (related only to their
  source actors until `.7.d` gives APB a clock/reset contract), and ADIv6's `TCK`/`DBGTMS` and Wishbone's six `…_I`
  signals flip from a defaulted output to their declared input. Their listings expose the actor graph: I2C's
  initiator is the phrase `optional open-drain` and Wishbone's is `SLAVE`, both `KG-ISF-COMPLETENESS.1`'s actor
  precision.

- ID: `EXTRACTION-REPRESENTATION-GAPS.29`
  Status: `done` (`2026-09-26`; MEASUREMENT + DECISION + CODE; SpecForge; found by `.7.d`; checklist below)
  Goal: **a timing-diagram position is a window only against the event it counts from.** All 6 windowed temporal
  rules on the live chains are APB-E's, and each takes its window from a diagram time label: *The Setup phase of the
  write transfer occurs at T1 in Figure 3-1* gives `[1, 1]`, *… shown at T2 in Figure 3-1* gives `[2, 2]`, with no
  antecedent to count from, so lowered they would count from the start of the run. The reader does this on purpose
  (`parse_named_generic_edge_diagram_position`, kg-bench `unit_first_diagram_position_timing_gold`). Decide what a
  position counts from, and keep a window only where the document grounds that event.
  **Decision (`2026-09-26`).** The IR keeps a position as the document states it (the kg-bench fixtures pin `[3, 3]`
  for *at tick T3 of HCLK*); what a window may not do is count from an event no sentence names. FSMGen's book (13,
  *Intent scheduling*) arms `(monitor (within S N))` when its transaction's `(on start)` fires, so the adapter's
  unguarded monitor counted every such window from the start of the run. A windowed rule now lowers only with an
  antecedent, as `(=> g (within s [min] max))`; without one it is listed. **Landed.** No live artifact moves: APB-E's
  six windowed rules were already refused for other reasons (*must be valid*, *remain stable*, `PSEL` undeclared).

- ID: `EXTRACTION-REPRESENTATION-GAPS.30`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.7.d`; checklist below)
  Goal: **contracts fuse only when their obligations agree.** `.25.a` fuses contracts whose conditions agree, but APB-E
  `fused:contract_temporal_signal_constraint_sigcon_0002+…_0005` joins *PADDR … must be valid* (window 1) with
  *PADDR must remain stable* (window 2) and keeps only the first obligation; `0003`+`0006` likewise. Measure every
  fused contract on the live chains and fuse only equal obligations. Since `.31` those two no longer fuse (their
  conditions differ); the rule that fused them stands.
  **Measured, and the goal corrected (`2026-09-26`).** Fusing disagreeing obligations is by design: `merge_cluster`
  marks the fused contract `Residual { disagreement: … }` (R16-MULTIMODAL-CONTRACT-FUSION's *no source silently
  overwrites another*). The defect is downstream: `classify_actor_contract` lowers a `Drive` or windowed contract
  through its obligation arm, reading `lowering` only in its fallback, so a disagreeing fusion became its first
  member's drive. On the live chains 9 contracts are fused, 3 of them disagreements, all AXI's (`AWBURST` INCR and
  WRAP, `BCOMP` HIGH and LOW, `WTAG` ZERO and VALID), none emitted only because each value was refused for another
  reason. **Landed.** A fused contract marked residual is listed as a disagreement. Only a fusion: the converter
  also marks a handshake barrier residual, which R16-CONTRACT-IR.4 lowers as a stage on purpose.

- ID: `EXTRACTION-REPRESENTATION-GAPS.31`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.7.d`; checklist below)
  Goal: ***X is asserted, which means that Y must …* keeps X as Y's condition.** APB-E *The select signal, PSEL, is
  asserted, which means that PADDR, PWRITE, and PWDATA must be valid* yields three constraints with no condition,
  and a fourth that `PSEL` is asserted; the obligation holds when `PSEL` is. Read the *which means that* clause as
  the consequence of the one before it.
  **Landed.** The pattern path gives an inferred obligation its antecedent clause as condition (`inference_antecedent`),
  unless the antecedent names the subject itself, which restates the fact the antecedent producer records; the
  semantic reader closes an appositive member with the bare predicate after it (*The select signal, PSEL , is
  asserted* reads `PSELx = ASSERTED`). With the three rules guarded, `.30`'s APB fusions stop, and the two stability
  rules they had hidden (`0005`, `0006`, *must remain stable* with a two-cycle window) surfaced as `(monitor (within
  PADDR 2))`, *PADDR becomes true*, because a windowed consequent with no value took the level `1`. Such a consequent
  is now a `Stable` or `Observe` obligation, listed, in the conversion and the classifier alike. APB-E's
  `requester.isf` is unchanged.

- ID: `EXTRACTION-REPRESENTATION-GAPS.32`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.7.c`; checklist below)
  Goal: **a port is named from its own actor's side of the wire.** Wishbone names each interface's ports by direction
  (`ACK_O` on the SLAVE, `ACK_I` on the MASTER, joined by the interconnect), and its MASTER section says *The SLAVE
  asserts either the [ACK_I], [ERR_I] or [RTY_I] signals in response to every assertion of the [STB_O] signal*
  (statement `0401`). Read as *SLAVE drives ACK_I*, it puts the MASTER's inputs on the SLAVE: Wishbone's `slave.isf`
  declares `ACK_I`, `ERR_I`, `RTY_I` as outputs beside `ACK_O`, `ERR_O`, `RTY_O`, while the SLAVE's own `STB_I`,
  which *indicates that the SLAVE is selected*, is related to no actor and left out. Decide from the document how a
  wire's two port names relate (the interconnect *connects the [CLK_O] output to the [CLK_I] input*), and ground each
  port on the actor whose interface names it.
  **Measured and landed.** The document assigns every port to a role in its own sections: *2.2.3 MASTER Signals*
  (inputs `ACK_I`, `ERR_I`, `RTY_I`, `STALL_I`; outputs `ADR_O`, `CYC_O`, `LOCK_O`, `SEL_O`, `STB_O`, `WE_O`), *2.2.4
  SLAVE Signals* (the mirror), *2.2.2 Signals Common to MASTER and SLAVE Interfaces* (`CLK_I`, `RST_I`, `DAT_I`,
  `DAT_O`), *2.2.1 SYSCON Module Signals*. `extract_relations_from_role_signal_sections` reads an *input*/*output*
  naming a bracketed signal under such a heading; `supersede_relations_by_role_sections` drops a prose relation that
  puts a signal on a role whose section does not name it (the four other-side relations, including *MASTER reads
  STB_I*). With the SLAVE no longer holding the MASTER's names, no actor was a net producer: the contract's clock and
  reset, inputs of every actor, tied the MASTER at 7 and 7. `select_initiator_for` counts without them, and Wishbone
  emits `master.isf` with exactly the MASTER's 12 ports, strict-clean. The section's role is named as written, so
  *SYSCON Module* stands beside the prose's `SYSCON` (`KG-ISF-COMPLETENESS.1`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.33`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.17`; checklist below)
  Goal: **an obligation bounded in time keeps its bound.** Wishbone's statement `0438` states three: *MASTER
  interfaces MUST assert [CYC_O] for the duration of SINGLE READ / WRITE, BLOCK and RMW cycles* (a scope over
  operations, in the active voice no constraint reader takes), *[CYC_O] MUST be asserted no later than the rising
  [CLK_I] edge that qualifies the assertion of [STB_O]* and *… MUST be negated no earlier than the rising [CLK_I] edge
  that qualifies the negation of [STB_O]* (an order between events; since `.17` minted as nothing, not as a level).
  Its `RST_I` *MUST be asserted for at least one complete clock cycle* is still read as `must_be_high` with the
  minimum duration lost (an input and the reset, so nothing lowers it). Measure these shapes across the live chains
  (*no later than*, *no earlier than*, *for the duration of*, *for at least*) and type each with its bound, or list it.
  **Measured `2026-09-26`** over the 27 live chains: 20 normative statements carry one of the shapes (*no later
  than* 7, *no earlier than* 1, *for the duration of* 5, *for at least* 5, *at least N cycles* 3; USB 3.2 holds 8,
  Wishbone 6). Only one yields a signal record: Wishbone's `RST_I` `must_be_high`. The USB rows are system
  behaviour (*U0 entry shall be initiated no later than tDownLinkStateChange*) with no signal, and the rest mint
  nothing. **Found here**: *nothing* is not *counted* — Wishbone's `0438` is classed `signal_value_constraint`, and
  the completeness summary counts only uncited `normative_statement`s, so it is listed nowhere (`.34`). **The bound is not lost at SemanticIR**: the rule
  semantic derives from `RST_I`'s constraint carries `cycle_window {min_cycles: 1}`, and the adapter lists it as an
  unguarded window (`.29`). **It is lost at IntentIR**: `synthesize_temporal_invariants` turns the same
  constraint into `tinv_sc_sigcon_0002`, *RST_I is 1* at every cycle. The same holds for every constraint whose
  own rule is windowed, 5 on the live chains: `RST_I`; APB-E's `PADDR`, `PWDATA` and `PSEL`, placed at a figure's
  *T1*/*T2*; AHB's `HEXOKAY` *in the same cycle as HRESP is asserted*, whose exclusion `.11` owns. None reaches an
  emitted `.isf` (each is an input's or already listed), so the defect is five false IntentIR facts. The code:
  an invariant holds at every cycle, so a constraint whose own rule states a window is not one.
  **Landed.** `synthesize_temporal_invariants` skips a constraint whose rule, found by `signal_constraint_rule_id`
  (the one link SemanticIR mints), carries a cycle window; the rule keeps the bound. IntentIR loses exactly the 5
  invariants, and no semantic record and no emitted `.isf` moves. `CYC_O`'s three clauses stay unread: an order
  between events and a scope over Wishbone's bus cycles have no IR shape, and one statement on the live chains
  states them. Counting that statement is `.34`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.34`
  Status: `done` (`2026-09-26`; MEASUREMENT + CODE; SpecForge; found by `.33`; checklist below)
  Goal: **an obligation no typed record cites is counted, whatever its class.** `validate`'s completeness summary
  counts the uncited statements of class `normative_statement` only (`uncaptured_normative_statement_ids`,
  `WIRE-BASED-100.3b`). On the 27 live chains, 96 of 165 `signal_value_constraint` statements and 34 of 1,007
  `conditional_rule` statements are cited by no signal constraint or conditional rule, and none is counted: Wishbone's
  `0438` (*[CYC_O] MUST be asserted no later than …*), which `.17` stopped reading as a level, is listed nowhere.
  Several refusals say their statement *stays counted as an uncaptured normative statement*
  (`ir/evidence.rs`, EXTRACTION-QUALITY-GAUGE.3k.2a and siblings; `pipeline/obligation-reading.md`); that holds only
  for a statement of that class. Measure which of the uncited statements another family cites (temporal rules,
  timing constraints, protocol records, relations), then count the rest as candidate misses, and correct the claims.
  **Measured `2026-09-26`** over the 27 live chains' EvidenceIR, counting a statement as cited when any other
  record family names it: of the statements no constraint or rule cites, 7 of 96 `signal_value_constraint` and 0 of
  34 `conditional_rule` are cited elsewhere (protocol states, relations), and 107 of 3,008 `normative_statement`
  (93 by protocol states, whose records carry no typed obligation, `.12`). The refusal comments never held: the
  passive constraint path reads only `signal_value_constraint` statements. `timing_constraint` (705 cited by none)
  and `derived_rule` (181) are topic classes, not obligation classes (*… typically has a data width of 32 bits*,
  *Figure 4-6 shows a configuration …*), so they are not counted. The finding's statement list was also not its
  count: `evidence_normative_residual_statement_ids` listed every normative statement, cited or not (3,026 against
  3,008). The code: an uncited `signal_value_constraint` or `conditional_rule` statement is a candidate miss, with
  its own metric and finding, and the normative finding lists exactly the statements it counts. Cited keeps its
  meaning (a constraint or a rule names it), so no count falls.
  **Landed.** `uncaptured_classified_obligation_statement_ids` (`ir/completeness.rs`) and one citation set in
  `validate` (`evidence_captured_statement_ids`) give `classified_obligation_residuals`, which joins
  `completeness_candidate_misses`, and `evidence_classified_obligation_residuals_remaining`, which lists them: 130 on
  the live chains (AXI 30, USB 3.2 22, ADIv6 17, Wishbone 15 with `0438`, …), 17 chains' totals rise by that much and
  none falls. The refusal comments in `ir/evidence.rs` and the book's *stays counted* sentences now name the count
  that holds them.

- ID: `EXTRACTION-REPRESENTATION-GAPS.35`
  Status: `done` (`2026-09-26`; CODE; SpecForge; found by `.10.g.iii`; checklist below)
  Goal: **a fronted condition keeps every member of the list it names.** AXI's *When AWSNOOP is not CMO, WritePtlCMO
  or WriteFullCMO, AWCMO must be 0b000* is `condrule_0152` with the antecedent *AWSNOOP is not CMO*: the fronted
  split (`split_conditional_sentence`) stops at the first comma, and the constraint path's
  `main_clause_of_fronted_conditional` recognizes a list only by an Oxford comma (*A, B, and C,*). Read, a guard would
  exempt only `CMO`. Measured over the 27 live chains: 3 conditional rules end their antecedent before a *, X or Y,*
  list (this one; USB 3.2 and USB4's *If there is a conflict between text, figures, and tables, …*), and no signal
  constraint does. End a fronted condition at the comma after the list's last member.
  **Landed.** `fronted_condition_end` serves both readers. It keeps the documented Oxford rule (a comma followed by an
  *and*/*or* segment is a list comma, and the condition ends at the comma after the list or else at its verb). It
  closes a list without the Oxford comma at the comma after its coordinating member, only where the main clause keeps
  a subject before the modal. The conditional-rule reader uses it only with a *must*/*shall* in the same sentence,
  else the first comma. A verbless all-list condition ends at the first comma. On the 27 live chains, 16 conditional
  rules change, every one a fix: AXI `condrule_0152` *AWSNOOP is not CMO, WritePtlCMO or WriteFullCMO*, ADIv6's
  *…, and the power controller receives a conflicting request …*, USB's *text, figures, and tables*. 8 AXI
  constraints regain *present* (*If present, AWSTASHNID, AWSTASHNIDEN, AWSTASHLPID, and AWSTASHLPIDEN must be
  LOW*), so `manager.isf` loses 18 rules that drove them LOW in every configuration; they are now listed under a
  presence condition (AXI's listed conditions 17 → 25, waiting for `.4`), strict-clean. **Found in replay and
  fixed before landing**: a first cut read lists over whole multi-sentence statements (USB 3.2's *…, it may
  receive …* swallowed its main clause), and broke the Oxford rule's `ACADDR, ACPROT, and ACSNOOP` subject; the
  three existing boundary tests caught the second.

- ID: `EXTRACTION-REPRESENTATION-GAPS.36`
  Status: `done` (`2026-09-26`; DOCUMENTATION + CHECK; SpecForge; the owner's request)
  Goal: **every missing piece from PDF to `.isf` is indexed in one place, with its owner.** The owner, `2026-09-26`:
  *catalog, categorize, list, describe, specify everything not addressed yet … explicitly indexed, task-tree owned,
  tracked … for FSMGen's items, an incremental list of all the things SPECFORGE needs FSMGen to support to go from
  PDF to ISF.* Gap knowledge is spread over this tree's leaves, the book's ledger, `docs/FSMGEN_FEEDBACK.md`, the 13
  packets under `docs/upstream/fsmgen/`, other trees' leaves and the category contract's carrier-less families.
  Build `docs/gaps/`: `INDEX.md` (every item, one row: side, road, stage, status, owner), `fsmgen-ladder.md` (the
  incremental list along the PDF-to-`.isf` path, each rung classified from FSMGen's published book at the pin as
  supported, fixed upstream and unpublished, requested, anticipated or a question) and `specforge.md` (SpecForge's
  items by road and stage). The index's owner column is an ownership region, so every cited unit must be classified
  and every current owner open (`OWNERSHIP-CITATIONS`); the collection is a registered live-document surface. The
  ladder is re-checked on every FSMGen pin refresh. An anticipated rung is not a request: `PROCESS.md`'s evidence
  bar and ADR 0033 still decide what is filed.
  **Landed.** [`docs/gaps/`](../gaps/INDEX.md): 25 SpecForge items (S1–S25) by road and stage, and FSMGen's ladder
  of 28 rungs classified from its book at `54ac1c22` (15 supported, 1 fixed upstream, 9 requested, 1 question, 2
  anticipated, F26's multi-bit crossings anticipated too); the index lists the 25 and the 14 rungs FSMGen does not
  yet provide, each with an open owner. The index table is the `gap_catalog_owners` ownership region (44 owners
  classified; the registry holds 104 of 128 records), and `docs/gaps/*.md` is the registered `gap_catalog` surface.
  Opened `.37` for the one need with no owner (temporal properties beyond *eventually*). **Found building it**: the
  handshake-to-stage lowering first listed as missing was delivered in May by `R16-CONTRACT-IR.4`, so it is not an
  item; the SpecForge list was drawn from the census, the category page and the owner leaves checked, not from every
  open leaf, so `.36.a` triages the rest.

- ID: `EXTRACTION-REPRESENTATION-GAPS.36.a`
  Status: `open` (`2026-09-26`; TRIAGE; SpecForge; found by `.36`)
  Goal: **every non-closed leaf is either a catalogued missing piece, process work, or closed.** On `2026-09-26`, 244
  leaves in 64 trees are not closed; many are process or repository work, and several trees are `active` with no
  open child. Classify each: a missing product piece gets a row in `docs/gaps/` under its owner; process work is
  named as such; a root whose children are all closed is closed or given its next leaf. Record the counts.

- ID: `EXTRACTION-REPRESENTATION-GAPS.37`
  Status: `open` (`2026-09-26`; MEASUREMENT then CODE, then a request if FSMGen lacks it; SpecForge + FSMGen; found by `.36`)
  Goal: **a temporal obligation beyond *eventually within N* is typed, and lowered or requested.** FSMGen's matrix
  (13k, *Explicit Non-Claims*) does not ship *runtime-signal or expression windows, global implication forms,
  dynamic bounds, expression operands, and multiple outstanding obligations*. The live chains state them: a window
  counted from an event (`.29` lists unguarded ones), a minimum duration (Wishbone *[RST_I] MUST be asserted for at
  least one complete clock cycle*, kept only as a SemanticIR `cycle_window`, `.33`), an order between two events
  (Wishbone *[CYC_O] MUST be asserted no later than the rising [CLK_I] edge that qualifies the assertion of [STB_O]*,
  counted, not read, `.34`). Measure how many live obligations take each form, type them in IntentIR, lower what
  `(assert …)` inside a transaction can state, and file a request for the rest under `PROCESS.md`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.38`
  Status: `open` (`2026-09-26`; DOC; found by `.3.c`'s commit)
  Goal: **partition this tree before its root warns.** The root file is 79.6% of the `task_evidence` 278,528-byte
  ceiling (warning 80%, rollover 90%) after 38 leaves in two days. Migrate it to a partitioned tree with
  `scripts/check_active_task_evidence.pl --migrate`, parts by reader concern (the census and roads, the FSMGen
  requests, the lowering leaves, the checklists), as the other large trees are; every leaf keeps its id and text.

- ID: `EXTRACTION-REPRESENTATION-GAPS.28`
  Status: `done` (`2026-09-25`; CHECK capacity; SpecForge; found by `.24.a`)
  Goal: **the ledger's owner citations have room to grow.** `doctrine/ownership_citations/citations.jsonl` holds one
  record per cited unit and allows 64; `.24.a` leaves it at 62, and the ledger holds most of them (ADR 0053 expects
  more rows). Raise the record bound under a recorded authority, keeping the byte bound able to fund it, as `.19`
  and `.20` did for their collections, and re-derive `claim-registry-capacity-is-coherent`, which reads this registry.
  **Landed.** `max_records` 64 → 128 under a registry authority in `ceiling_increase_authorities.jsonl` (retired by
  the next commit, as the protocol requires); `max_bytes` stays 32,768, which funds about 200 records at the real
  mean of 160 bytes, so the census reports no byte bound biting. The registry stays incoherent in the class sense
  (`max_records` × `max_record_bytes` exceeds `max_bytes`), which is `LIVE-DOCUMENT-PRESSURE-HEADROOM.36d`'s question,
  so the census's class shape (2 coherent, 8 not) is unchanged. **Found here and fixed**: that claim's result says
  *no registry is at or above a rollover milestone*, which was false while this registry sat at 93.8% and 96.9%,
  and nothing it ran tested the sentence. The census's summary line now carries `at rollover N` and the claim checks
  `… incoherent 8   at rollover 0`: restoring 64 prints `at rollover 1` and the claim's command fails (RED).

## `.2` — the IntentIR-to-`.isf` census (`2026-09-25`)

Re-derive: `python3 scripts/measure_intent_to_isf_reach.py` (read-only, over the 27 live chains).

**Emission.** All 27 live adapters are `blocked`: 27 have no clock/reset contract, 20 also have no interface
signals, 4 also no behaviour. The 7 with signals (AXI, APB 5, AHB, ADIv6, I2C, I2S, Wishbone) are blocked by the
contract alone (`.7`). Every number below is what the adapter *would* lower; none of it reaches FSMGen today.

**Families the adapter never reads**, and why that is or is not a gap:

| Family | Records (docs) | Verdict |
| --- | --- | --- |
| `constraints`, `behaviors`, `assumptions` | 5,229 (26), 2,091 (24), 157 (22) | untyped prose (`statement` + ids): unlowerable by construction; how many say what no typed record does is `.3` |
| `timing_constraints` | 237 (5) | physical and electrical parameters (ns, V, dB, Ω, mA, UI) from PHY/electrical tables: not an `.isf` gap by decision (below) |
| `signal_polarities` | 51 (4) | read nowhere, which makes the `asserted` lowering polarity-blind: `.8` |
| `signal_connectivity` | 235 (6) | topology; FSMGen composes actors, SpecForge emits one flat actor: `.14` |
| `actor_signal_relations`, `actor_drive_relations`, `actor_sample_relations` | 546, 273, 273 (6) | inputs to `actor_ports` directions, which the adapter reads: not a gap |
| `*_conflicts`, `fidelity_findings`, `captured_region_residuals`, proofs, identities | — | accounting and provenance, not design intent: not a gap |
| `regular_states`, `state_transitions`, `explicit_modules`, `explicit_tops`, `actor_trigger_relations` | 0 | empty on every live chain: an extraction gap where the document states them (`.12`, `.14`) |

**Residual packets** (150 in 28 families over the 27 adapters), attributed:

| Class | Packets | Owner | Why |
| --- | --- | --- | --- |
| protocol state / operation / edge timing | 51 | SpecForge `.12` | the records lack typed transitions, guards, initial state, encoding, actions |
| conflicting drives under an empty guard, rule/transaction overlap | 45 | SpecForge `.10` | the source's condition was lost; constraints lowered as drives |
| value `VALID`/`INVALID` | 11 | SpecForge `KG-ISF-COMPLETENESS.2a.vii.e`; FSMGen `.16` | validity is not a value; no known-value predicate |
| bare stability, 0-cycle exclusion | 10 | SpecForge `.11`; FSMGen `.15` for placement | `(stable …)` exists; the adapter's claim that it does not is stale |
| value words `ZERO`, `INCR`, `SHAREABLE` | 5 | SpecForge `.9` | FSMGen accepts enum members and numbers |
| register fields / resets not lowered | 5 (229 items) | SpecForge `.13`, FSMGen maybe | capture causes; symbolic resets to be measured |
| signal-width parameters | 9 | FSMGen `FSMGEN-REQUESTS.1`, `.4`-`.6`; SpecForge `KG-ISF-COMPLETENESS.2a.vii.c.iv`-`.c.vi` | the ledger's parameter rows |
| upstream canonicalization, grouping, role, unreadable width, guard disagreement, missing interface signal | 14 | SpecForge (their producing stages) | IR-side, before the adapter |

**Decision — physical and electrical parameters are not requested of FSMGen.** `timing_constraints` holds
tables of nanoseconds, volts, decibels, ohms and unit intervals, from documents the category contract classes
`physical-link` (`executable_target: honest_non_target_by_default`). ADR 0033 §6, kept by ADR 0053, keeps such
knowledge typed in IntentIR and forbids forcing it into executable constructs. Requesting an FSMGen carrier for
them would reverse that; it is the owner's call and is raised to them, not assumed.

## Current Frontier

1. `.3` — the PDF-to-IR census against the category contract, first. Re-prioritized `2026-09-26`: 44 of this tree's
   59 commits since `2026-09-24` served the wire-protocol road the category page rates most mature, while ADR 0033
   makes per-category PDF-to-IR population the program constraint. `.3.a` made four more documents replayable (31
   chains) and `EXTRACTION-GAP-FIX.6` corrected the register surface they exposed. `.3`'s per-family census is
   recorded: 2,793 captured regions and 167 of 187 assumptions reach no carrier. Next, in order: `.3.c` (134 typed
   tables no record cites: 24 left to read) after `.38` (partition this root first), `SIGNAL-DECLARATION-ROW-DROP.6` (TMC's `CLK`), `EXTRACTION-GAP-FIX.9`
   (TMC's field tables), `.3.f` (I2C's multi-mode timing),
   `.3.e` (a symbol's minting table), `.3.b` (1,161 untyped tables), `.3.d`.
2. `.7.e.i` — the reset-less documents, after `KG-ISF-COMPLETENESS.1` grounds their initiators.
3. `.36.a` (triage every open leaf into `docs/gaps/`), `.37`, `.17.a`, `.11`, `.9`, `.10`, `.12`, `.13`, `.14`, `.18` —
   the SpecForge extraction and lowering gaps; `.10.g.iii` waits for `.4`.
   (`.4`-`.6`, `.15`, `.16`, `.20` are requests and `.19` a question, forwarded by the owner `2026-09-25`; each
   closes when FSMGen answers and SPECFORGE integrates. `.21` lands with `FSMGEN-REFRESH-INTEGRATE-10` once FSMGen
   publishes its fix.)

## Decisions

- `2026-09-26` — `.3.a`: `.3` measures families per replayable document against all six categories, not per
  classifier bucket; the bucket merges categories 2 and 3 by design and reports 4 only on a self-declaration.
  (First written with CHI C2C in the register bucket, from a legacy report; it is wire-protocol, see `.3`.)
- `2026-09-25` — `.24.a`: a generic name is tied to its family only by the document's own statement (APB's shared
  role phrase; AHB states none, so `HSEL` stays unread); a family lowers one instance at a time under the generic
  contract; FSMGen already states one select per target, so no request, and the fan-out is `.14`'s composition.
- `2026-09-25` — ADR 0053: extraction and FSMGen requests advance in parallel; the evidence bar is unchanged;
  one ledger in the book.
- `2026-09-25` — the owner, on reset semantics (`.7`): *an asynchronous reset, active high or active low (the
  default), asserts asynchronously and deasserts synchronously with respect to the domain clock.* So a document's
  *asserted asynchronously, deasserted synchronously* is FSMGen's `async`, and an asynchronous deassertion has no
  `.isf` form and is refused. Since FSMGen defaults the polarity, SPECFORGE always states it and never omits it.
- `2026-09-25` — the owner, on reset names: an active-low reset *usually* ends in `_n`, `_b` or a bare `n`
  (`ARESETn`), *just a convention* that helps a human read RTL. It stays a convention: no stage reads a polarity
  from a name (ADR 0037). FSMGen at the pin does not honour a stated polarity; it keys the polarity on the name
  (probe in `.21`).
- `2026-09-25` — the owner, on `PSELx`: *the `x` discriminates several selectable APB targets (target A has
  `PSELA`, target B `PSELB`, …); `PSEL` without it is the generic name, and any instance must behave according to
  `PSEL`'s contract.* Owned by `.24`.
- `2026-09-25` — the ledger is seeded only with rows re-verified today. The legacy-document reach was left
  out: chain currency reports 51 documents unmeasurable, while `CORPUS-COVERAGE`'s census counts refreshed
  documents differently; `.2` re-derives the reach before it is published.

## Open Questions

None.

## Blockers

None.

## Verification Log

- `2026-09-25` — `.0`: every seeded row re-derived in this session. FSMGen facts read at the pin from its
  published book (`subs/fsmgen/docs/book/src/13a-actor-interface.md` *Interface*;
  `13k-isf-feature-support-matrix.md` rows *Actor report metadata and params*, *Interface ports*, *Reusable ISF
  libraries*); SpecForge facts from the persisted IntentIR of the three live AMBA chains (36 `Param` symbols: AXI
  28, AHB 7, APB 5 1) and the `.2a.vii.c.iii` record.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.1` (CHECK) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — with the ledger declared as a region, `perl scripts/check_ownership_citations.pl --check` at `869a2b62` would read `KG-ISF-COMPLETENESS.2a.vii.c.iv` (declared one-line in `docs/tasks/kg-isf-completeness/isf-parameter-widths.md`) and `FSMGEN-REQUESTS.1` (declared one-line in its root) as nonexistent: `unit_status` matched only `- ID:` + newline + `Status:` in the root file.
- [x] **ROOT CAUSE (WHY + WHERE)** — `unit_status` (`scripts/check_ownership_citations.pl`) knew one of the two leaf grammars and never opened a partitioned tree's parts.
- [x] **ADDRESSED (verified)** — `resolve_leaf_status` + `tree_part_texts`; live check: 38 cited units across 3 regions resolve. End to end: pointing one ledger row at the closed `KG-ISF-COMPLETENESS.2a.vii.c.iii` is refused by name (`its own Status is 'done'`), then restored.
- [x] **NO REGRESSION** — self-test 16/16 (11 prior cases unchanged in intent); `ROADMAP.md` and the pressure-headroom region still resolve (28 units before the ledger region, 38 with it); RED: deleting the one-line grammar fails *a one-line leaf in a root did not resolve*, and resolving disagreeing declarations to the first fails *disagreeing declarations were resolved to one status*.
- [x] **GENERICITY** — the rule reads the task-tree grammars and the repository's `docs/tasks/<tree>/` layout; no tree is named.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md` `OWNERSHIP-CITATIONS` row; the ledger chapter's update rule states the check; this tree's frontier.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.8` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — persisted adapter text at `e2de9962`: AXI `(rule constraint_8 (AWLOCK 1))` from *AWLOCK must be deasserted* (`must_be_low`), `(AWSTASHLPIDEN 1)` from *… must be LOW*, APB `(PADDR 1)` from *must remain stable*, AXI `rule_70 (AWCMO 1)` from *must be 0b000*; 250 drive rules over 6 documents, 46 of 53 lowered conditional rules with action text `must` alone.
- [x] **ROOT CAUSE (WHY + WHERE)** — `IsfIr::from_intent_ir` (`ir/isf_ir.rs`): the signal-constraint loop ignored `constraint_kind` and did `target_value.unwrap_or("1")`, the invariant loop the same, the conditional-rule loop drove `1` unless the action contained `not`, and `isf_literal_value` mapped `asserted` to `1` regardless of polarity; upstream, `signal_constraint_to_temporal_invariant_kind` (`ir/intent.rs`) folded five level kinds into `MustBeAsserted` and `synthesize_temporal_invariants` copied neither the level nor `negated`.
- [x] **ADDRESSED (verified)** — rebuilt 6 chains: 250 → 95 rules, 44 AXI values corrected, 166 obligations listed in `isf_obligations_not_lowered`; spot check of emitted AXI constraints against their source sentences (`AWSTASHNIDEN … must be LOW` → `0`, `ARMMUATST Must be 0b0` → `0B0`, `SYSCOACK must be deasserted` → `0`). Tests `a_signal_constraint_states_its_level_with_negation_applied`, `a_level_resolves_only_through_the_signals_stated_polarity`, `an_action_states_a_level_only_when_it_names_one`, `an_obligation_drives_only_the_level_the_document_states`, `a_guard_on_an_unknown_polarity_refuses_the_rule_instead_of_vanishing`; three RED-verified (restoring the default `1`, resolving `asserted` without polarity, skipping an unknown-polarity guard). No test pinned the old values, which is how an inversion survived.
- [x] **NO REGRESSION** — `intent`/`adapt --dry-run` over the 27 live chains: 21 identical, the 6 changed only in `temporal_invariants.target_value` (IntentIR) and in their rules/residuals (adapter); FSMGen `--strict --check` 0 diagnostics on all 6 rebuilt texts; WIRE-BASED-100 11 of 11 seeds identical (HEAD and new binaries; EvidenceIR unchanged); `check_chain_currency.sh` 27 of 27 current at evidence, semantic, intent and isf-adapter; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,684); flow census +8 / +45 / +130 / 0, boundary counts unchanged.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — levels come from the constraint's typed kind, the action's own words and the signal's stated polarity; no signal name, suffix or protocol is consulted.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *What value does a rule drive?* (the rule, the table, the dated correction) and step 10; ledger chapter (polarity row now `.17`, conflict count, *already flows* entry); KM card `isf-emission-blocked-by-clock-reset-contract`; new leaves `.17`, `.18`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.7.b` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `01ed8d80` 0 of 27 live chains carried a `system_contract`. `.7.a`: AXI and AHB state their reset's polarity and kind only in prose (`0549`, `0855`), APB-E its polarity only in a table cell (`0164`), and the table rule needed both in one cell. A prototype of the reader over all 78 EvidenceIR documents fired on exactly AXI, AHB and APB-E.
- [x] **ROOT CAUSE (WHY + WHERE)** — `parse_system_reset_descriptor` (`ir/semantic.rs`) required a kind in every `Reset` statement and `build_system_contract` merged whole declarations, so a kind and a polarity stated apart never met; nothing read either from prose (`synthesize_system_contract_from_table_descriptions`, `ir/evidence.rs`, reads one table cell).
- [x] **ADDRESSED (verified)** — `synthesize_system_reset_prose_statements`, a `Reset` grammar that takes the kind or the polarity alone, and a merge that refuses disagreement. AXI `ACLK`/`ARESETn` and AHB `HCLK`/`HRESETn`, asynchronous, active low, confidence high; both chains `renderable`; both `manager.isf` pass FSMGen `--strict --check` at `54ac1c22` with 0 diagnostics. Tests `a_reset_sentence_states_its_kind_and_polarity`, `a_reset_sentence_is_refused_when_it_is_ambiguous_or_unrepresentable`, `a_contract_merges_a_kind_and_a_polarity_stated_apart`, `a_contract_is_refused_without_a_kind_or_when_its_statements_disagree`. Found in verification and fixed here: the reset left the interface, so `guard_from_candidates` skipped *when ARESETn is asserted* and AXI SYSCOREQ/SYSCOACK drove `0` unconditionally. The adapter now tests the contract's reset at its stated level (`with_reset_polarity`, rule signals) and never drives it: `a_guard_on_the_contract_reset_survives_the_reset_leaving_the_interface`, `the_contract_states_the_reset_polarity_unless_a_record_disagrees`, both halves RED-verified.
- [x] **NO REGRESSION** — EvidenceIR replay over the 27 live chains: 24 identical; AXI, APB and AHB gain one appended statement each (earlier statements an exact prefix, source span carried). Rebuilt: APB's IntentIR and adapter identical; AXI and AHB gain the contract, 2 infrastructure signals, clock/reset input ports on every protocol actor (18/16, some on descriptor-noun actors owned by `KG-ISF-COMPLETENESS.1`), and AHB 2 direction conflicts (the table's driver-side `output` against the contract's `input`). Their clock and reset are no longer emitted as driven outputs. AXI residuals 49 → 47 (census 145/25 → 143/25). WIRE-BASED-100: 6 of 6 AMBA seeds identical between the HEAD and new binaries (constraints 6/6·6/6·3/3, relations 5/5·6/6·6/6, temporal 3/3·4/4·3/3). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. `cargo fmt` and `cargo clippy -D warnings` clean; lib tests 473 / 168 / 1,690; flow census +8 / +50 / +30 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the reader keys on the words *reset signal*, *active low/high* and *asserted (a)synchronously*, and on the document's declared signals; no signal name, suffix or protocol is consulted (the renamed-signal case in the refusal test).
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *Where do the clock and reset come from?*; ledger intro, *Clock and reset*, new rows *A reset's stated polarity* (`.21`) and *A rule's stated condition* (`.22`, `.23`), *already flows*; KM card `isf-emission-blocked-by-clock-reset-contract` (and its stale `.8` reverify line); leaves `.7.a`-`.7.e`, `.21`-`.23`; the owner's reset statements under Decisions.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.22` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `1746276a`, over the live chains: 16 emitted rules dropped a stated condition (AXI `rule_70` *AWSNOOP is not CMO*, 8 through invariants built without their constraint's condition) and 3 AXI rules were guarded `(== reset 1)` on a signal that does not exist, which FSMGen at the pin turns into an input port. Of 80 conditions on lowerable records most are prose; 24 of 81 constraint-derived invariants had lost their condition.
- [x] **ROOT CAUSE (WHY + WHERE)** — `sanitize_rule_condition` (`ir/isf_ir.rs`) returned no guard for a condition of several words and `(== <word lowercased> 1)` for one word; the invariant loop guarded on `condition_signal` alone at an invented `1`; `extract_condition_from_text` (`ir/intent.rs`) read only a condition still containing *when* and kept no value; `guard_from_candidates` kept the first usable candidate and skipped the rest.
- [x] **ADDRESSED (verified)** — `condition_guard` (whole `SIGNAL is LEVEL`, or *(in/during) reset* as the contract's reset), candidates joined by `(& …)` with undeclared, levelless or contradictory ones refusing the rule, `condition_value` carried and an unreadable condition yielding no invariant, refusal `ConditionNotLowered`. AXI 89 → 80 rules (12 guarded: nine `(== ARESETn 0)`, three `(== ACTIVATEACK 0)`), APB 5 → 3, I2C 3 → 1; AXI `manager.isf` strict-clean. Tests `a_condition_is_a_guard_on_a_declared_signal_or_it_is_refused`, `an_obligation_keeps_its_stated_condition_or_is_refused`, `a_constraint_condition_is_read_whole_or_the_invariant_is_not_made`, the updated `a_guard_on_an_unknown_polarity_refuses_the_rule_instead_of_vanishing`; three RED-verified (a condition dropped to no guard, first candidate only, contradiction accepted).
- [x] **NO REGRESSION** — IntentIR replay over 27 chains: 23 identical, AXI/APB/AHB/I2C change only in `temporal_invariants` (22 removed, all with an unread condition; 5 gain their condition); adapters: all 27 persisted equal the final binary (ADIv6 and Wishbone change only the listed-obligations wording). WIRE-BASED-100: 6 of 6 AMBA seeds identical (EvidenceIR unchanged). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. `cargo fmt`, `cargo clippy -D warnings` clean; lib tests 473 / 168 / 1,692; flow census 0 / +5 / +4 / −2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the grammar is the words *is*, the level vocabulary and *reset*; signals come from the declared set and levels from stated polarities; no name or protocol is consulted.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` *And under which condition?*; ledger row *A rule's stated condition*; KM card; leaves `.24`, `.25`; the owner's `PSELx` statement under Decisions.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.25.a` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `8f6c177c`, AXI's contract `fused:…sigcon_0043+…_0044` joined two mirror sentences with opposite conditions (candidates `SYSCOREQ ASSERTED`, `SYSCOACK DEASSERTED`, `SYSCOACK ASSERTED`); `sigcon_0044`'s antecedents read *SYSCOREQ deasserted* as `ASSERTED`; 4 emitted temporal rules dropped a stated condition (AXI `sigcon_0047`-`0049` *during reset*, I2C `dyn_sigcon_0004`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `fusion_key` (`ir/fusion.rs`) omitted the conditions and `merge_cluster` unioned them; `temporal_clause_value` (`ir/semantic.rs`) took the clause's first value word for whichever signal the clause yielded, and a clause yielded one signal; *until* was not cut; the adapter lowered a temporal rule with no antecedent as unconditional although its constraint stated a condition.
- [x] **ADDRESSED (verified)** — conditions in the fusion key; `split_clause_at_signal_mentions` plus `own_mention_tail` give each signal its own value; `until` cut with `unless`/`except`; `unread_stated_condition` makes the adapter list such a rule. AXI: the AWAKEUP rules become strict-clean holds, three *during reset* rules listed (the constraint loop keeps them guarded `(== ARESETn 0)`); I2C's one listed. Tests `contracts_with_different_stated_conditions_do_not_fuse`, `temporal_condition_reads_each_signals_own_value_and_stops_at_until`, `a_temporal_rule_whose_stated_condition_was_not_read_is_listed_not_emitted`, and `derives_actor_grounded_stability_event_from_stable_constraint` (now declares `HREADY` and asserts the antecedent); five RED-verified (key without conditions, no `until` cut, first value word, no per-mention split, unread condition lowered).
- [x] **NO REGRESSION** — first attempt, dropping such rules in SemanticIR, lost 2 of 4 AHB WIRE-BASED-100 temporal golds (*HAUSER must be valid during the address phase*: the gold keeps the rule with no antecedent), so the refusal moved to the adapter. Final: SemanticIR replay 26 of 27 identical, AXI changing only the AWAKEUP contracts and rules; AHB un-fuses one pair (its adapter text unchanged); every adapter equals the final binary; WIRE-BASED-100 6 of 6 identical; `check_chain_currency.sh` 27 of 27 current at every stage (2 emitted `.isf` files); lib tests 473 / 168 / 1,695; `cargo fmt`, `cargo clippy -D warnings` clean; flow census +6 / +26 / +89 / +2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — mentions are the document's declared signals and co-references; values are the level words; no name or protocol is consulted.
- [x] **LOCKSTEP** — book *And under which condition?* (temporal path, dated correction of `.22`); ledger condition row; KM card; `.22`'s record corrected; `.25.b` closed with it; `.25.c` updated.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.26` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `3d1651cb`, over the 27 live chains (`partial_conditions_all.py`, scratch): of the 16 temporal rules whose stated condition yields a guard, 5 keep part of it. APB `sigcon_0015` *PSEL , PENABLE , and PREADY are asserted, and PWRITE is deasserted* has antecedents `PREADY`, `PWRITE` only; AHB `sigcon_0007`, `0009`, `0010`, `0011` *… when HREADY is LOW/HIGH, unless HRESP signal is ERROR* keep `HREADY` and lose the exception. No conditional-rule-sourced rule is partial; AXI's two *until* cuts are releases (`.25.c`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `parse_temporal_condition_predicates` (`ir/semantic.rs`) distributed a list's value only when the whole condition stated exactly one value, and silently dropped value-less members otherwise; it cut `unless`/`except` with `until` and kept nothing of them (`WIRE-BASED-100.5h`); `TemporalRuleRecord` and `Condition` had nowhere to put an exception.
- [x] **ADDRESSED (verified)** — `parse_temporal_condition` + `read_condition_clauses` + `TemporalRuleRecord::exceptions` + `Condition::Ne` + the negated guard term; SemanticIR replay: APB `sigcon_0015` now `PSELx`, `PENABLE`, `PREADY` asserted and `PWRITE` deasserted, AHB's four carry `HRESP ERROR`, and those 5 contracts carry the same as guard candidates. FSMGen `--strict --check` accepts `(rule r (& (== READY 0) (! (== RESP 1))) (HOLD 0))` at `54ac1c22` with 0 diagnostics (HDL not inspected: FSMGen writes it only inside its repository). Tests `a_list_member_takes_the_value_of_the_clause_that_closes_its_list`, `an_exception_is_kept_apart_from_the_condition_and_an_until_is_not_one`, `a_condition_read_only_in_part_yields_nothing`, `a_rules_exceptions_join_its_guard_negated`, `contracts_that_differ_only_in_an_exception_do_not_fuse`, `a_rule_keeps_its_exception_as_a_negated_guard_or_is_listed`, `a_temporal_key_carries_exceptions_only_when_the_rule_has_them`, `an_exception_renders_as_a_negated_antecedent_atom`; nine RED-verified (no backward fill, exceptions dropped, partial read kept, negated term omitted, conditional rules not listed, `Ne` minted as `Eq`, no `|X:` key segment, no negated LTL atom, `Ne` left out of the fusion key).
- [x] **NO REGRESSION** — new-binary dry-runs against the persisted artifacts: 25 of 27 chains identical at SemanticIR, IntentIR and adapter (75 of 81 stage replays); APB-E and AHB change only in `temporal_rules` and `actor_contracts`, and after the rebuild their adapters are identical and AHB's `manager.isf` byte-identical. WIRE-BASED-100: 6 of 6 AMBA seeds identical, each side scored with its own binary (HEAD on the old gold, new on the new gold); the new binary on the old AHB gold scores 2/4, so the key sees exceptions. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,702; `cargo fmt` and `cargo clippy -D warnings` clean; flow census +4 / +54 / +54 / +3, boundary counts unchanged.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the grammar is list punctuation, the level vocabulary and the words *unless*/*except*/*until*; signals come from the declared set and the document's co-references; the tests use renamed signals.
- [x] **LOCKSTEP** — book `domain/temporal-semantics.md` *Reading a condition whole* (and the conflict fields), `pipeline/isf-adapter.md` *And under which condition?* (table rows, dated correction); ledger condition row; KM card `temporal-condition-read-whole`; the two AHB golds; `.28`'s authority retired.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.27.b` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `0519ec79`: AXI's `table_0251` (*Table B1.1 Continued from previous page*, kind `unknown`, header `Name | Width | Source | Description`) and `table_0255` (B1.4) state `Source: Manager` for 43 rows (`AWSTASHLPID | 5 | Manager`), and no actor relation carries any of them; `continuation_reach.py` finds no other live chain with such a fragment.
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_relations_from_signal_tables_with_prior_guidance` (`ir/evidence.rs`) gates each table on `should_treat_table_as_top_level_signal_description`, whose kind for a fragment is `unknown`; `continuation_inherited_table_heads` resolves the fragment to its head but served only the declaration gap-fill, the table accounting and validate.
- [x] **ADDRESSED (verified)** — the reader judges a fragment by its head (kind gate, caption owner, section context) and cites the fragment. EvidenceIR replay: AXI `actor_signal_relations` 257 → 343, every old relation kept (id-free), the 86 new ones 43 `Manager` drives and 43 `Subordinate` reads on `table_0251`/`table_0255`; the Manager's grounded ports 122 → 165 of 294. Test `a_continuation_page_states_its_sources_like_the_page_it_continues` (a fragment relates its rows and cites itself; a fragment with a different header relates nothing), RED-verified by reading the fragment in its own context.
- [x] **NO REGRESSION** — EvidenceIR replay: 26 of 27 live chains identical. AXI rebuilt (evidence → validate → semantic → intent → adapt, each validated): its adapter is content-identical and `manager.isf` byte-identical; SemanticIR/IntentIR gain the relations, 86 ports, 42 connectivity records, actor grounding on 20 temporal rules and 10 contracts, and one connectivity conflict (`ARCHUNKEN`: Manager in Table B1.4, interconnect in `1854`). WIRE-BASED-100: 6 of 6 AMBA seeds identical, each side scored with its own binary on its own AXI EvidenceIR (HEAD's regenerated for the HEAD run, then the new one, content-identical to the rebuilt, and every downstream stage rebuilt from it: a first currency run failed at the adapter on the stale ledger prefix, recorded in card `retained-chain-rebuild-order`). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,703; `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean after the last edit (which also fixes `.26`'s two misses); flow census 0 / +1 / +1 / 0, boundary counts unchanged.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the join is the caption's table number and the exact first header row; no table, signal or protocol is named.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (*Where graph facts come from*), `pipeline/evidence-rule-chronology.md` (the continuation passage); KM card `header-trapped-signal-table-recovery`; `.27.d` corrected; `.26` corrected.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.27.d` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `927f2712`, AXI's 122 check signals (Table A16.4: uncaptioned head `table_0245` and four continuation pages, header `Name | Signals covered | Width | Check enable`) relate to no actor, and the emitted `manager.isf` declares all of them as Manager outputs by default. Only AXI heads its check column otherwise (APB and AHB: *Check signal*).
- [x] **ROOT CAUSE (WHY + WHERE)** — `augment_check_signal_relations_from_tables` (`ir/evidence.rs`) found the check column only by a header containing *check signal* and skipped every table without one.
- [x] **ADDRESSED (verified)** — a covered-signals column makes the table a check table and the row's signal is its first column. AXI EvidenceIR: 343 → 597 relations (every old one kept), +123 direction declarations appended (52 later synthesized statement ids renumber; 6,476 keep their text); the Manager's grounded ports 165 → 286 of 294; `manager.isf`: 40 outputs become inputs and their 40 drive blocks go, the rest unchanged; FSMGen `--strict --check` 0 diagnostics. Test `a_check_table_is_read_by_its_covered_column_whatever_it_calls_its_name_column` (checks inherit drivers on both sides; a table without the column infers nothing), RED-verified by removing the fallback.
- [x] **NO REGRESSION** — EvidenceIR replay: 26 of 27 live chains identical. AXI rebuilt from EvidenceIR in order. WIRE-BASED-100: 6 of 6 AMBA seeds identical, HEAD scored first on HEAD's persisted artifacts with HEAD's binary, then the new binary on the rebuilt ones (no ledger detour). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,704; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census unchanged.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the rule reads the header's covered-signals column and the row's first cell; the direction comes from the covered signals' relations; no name suffix, table or protocol is consulted.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (*Where graph facts come from*, the check-table paragraph); ledger direction row unchanged (`.27` still open).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.27.c` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `55b94339`, AXI's six `BROADCAST*` signals relate to no Manager although A8.12, A9.2, A9.13, A9.16 and A9.19 each describe theirs as *Manager tie-off input* (A6.11 too, on a table classed `unknown`); `manager.isf` declares `BROADCASTCACHEMAINT` and `BROADCASTSHAREABLE` as driven outputs. No other live document states an actor's input this way (`tieoff_census.py`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_relations_from_signal_tables_with_prior_guidance` (`ir/evidence.rs`) reads an actor only from a `Source`/`Destination` column or a signal-section heading; a description cell that names the owner was never read.
- [x] **ADDRESSED (verified)** — `actor_relation_stated_by_description`, tried after the relation column and before the heading. AXI: +5 `Manager` reads, 4 `WAKEUP` relations re-cited to A14.1 (unchanged otherwise), 0 removed; grounded ports 286 → 291 of 294; `manager.isf` differs only in the two ports becoming inputs and their two drives going; FSMGen `--strict --check` 0 diagnostics. Test `a_description_that_names_an_actors_input_states_the_relation` (a stated input relates; a mid-cell mention or a missing input/output does not), RED-verified by removing the reader.
- [x] **NO REGRESSION** — EvidenceIR replay: 26 of 27 live chains identical; AXI rebuilt in order. WIRE-BASED-100: 6 of 6 AMBA seeds identical (HEAD scored first on HEAD's artifacts). `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,705; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +1 / +13 / +35 / 0, boundary counts unchanged.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the actor must be one the document names in its own signal tables or headings; the grammar is the words *tie-off*, *input*, *output* at the start of the cell; no signal, table or protocol is named.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (*Where graph facts come from*); `.27.c`'s goal corrected; ledger direction row unchanged (`.27` open).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.27.f` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `4ffb0795`, a probe of `extract_actor_signal_relations` on the two sentences returns `two-cycle response Drives HTRANS`, `Manager Drives HTRANS`, `Manager Reads HTRANS` (AHB `0710`) and `Manager Drives ASKSTOP` (AXI `3950`); B1.9 states `ASKSTOP | 1 | Subordinate`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `active_object_contains_signal` (`ir/evidence.rs`) ended a verb's object only at adverbial markers and coordinated relation verbs, so `provides` and `controls` reached signals in a later clause with its own subject; `is_nominal_verb_occurrence` looked one word back, so *this next access* passed as a verb.
- [x] **ADDRESSED (verified)** — `but` and `infinitive_with_own_subject_start` end the object; a determiner plus a sequence modifier makes the verb form a noun. The probe then returns only `Manager Drives HTRANS`. EvidenceIR replay: 8 relations leave (AXI 2, AHB 4, ADIv6 2), each read and each false, none added. Tests `an_infinitive_with_its_own_subject_ends_the_object_and_a_modified_noun_is_no_verb`, `but_opens_a_clause_that_the_first_verb_does_not_govern`, `an_object_before_the_new_clause_is_still_the_verbs`; three RED-verified (no `but`, no infinitive rule, no modifier rule).
- [x] **NO REGRESSION** — EvidenceIR replay: 24 of 27 live chains identical; AXI, AHB and ADIv6 rebuilt in order. AXI `manager.isf`: `ASKSTOP`/`ASKSTOPCHK` become inputs and lose their drives; AHB's is byte-identical; ADIv6's adapter identical; FSMGen `--strict --check` 0 diagnostics on both files. WIRE-BASED-100: 6 of 6 identical after correcting one gold item: `seed_axi_temporal` `statement_3954` (*ASKSTOP must be LOW when ACTIVATEACK is LOW*) had been drafted without the consequent's driver, which the extractor could not ground while the false `Manager drives ASKSTOP` competed with the table's Subordinate; the gold now carries `Subordinate drives ASKSTOP`, citing B1.9 and A14.4 and following its sibling item `4690`. HEAD's binary on the old gold and the new binary on the new gold both score 3/3, and the new binary on the old gold scores 2/3. `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,708; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +1 / +7 / +1 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the rules are grammar: a coordinating conjunction, a `for … to` infinitive, determiners and a closed class of sequence modifiers; no actor, signal or protocol is named.
- [x] **LOCKSTEP** — book `domain/actor-connectivity.md` (active prose); KM card `relation-verb-after-an-article-is-a-noun`; `.27.e` re-measured; `.10` measured; the AXI temporal gold.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.27.e` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `e96f373b`, AHB's `manager.isf` declares 9 ports the document does not ground on the Manager (`HSELx` on the address decoder; `HRUSER`, `HBUSER`, `HREADYOUT` and their checks on the Subordinate; `HRESET` on none) as Manager outputs with drives, and nothing lists them (`port_grounding.py`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `IsfIr::from_intent_ir` (`ir/isf_ir.rs`) put every interface signal in the initiator's actor and gave one it did not drive or read the first interface record's direction hint, else `(output)`; the drive block followed.
- [x] **ADDRESSED (verified)** — the grounded-or-listed rule with `declared_signal_directions` and the `isf_ports_not_on_the_lowered_actor` residual. Adapter replay: 22 of 27 identical; APB-E, AHB, ADIv6, I2C and Wishbone change as recorded above, none leaves a dangling reference to a left-out port; AHB `manager.isf` loses exactly 9 ports, 9 drives and one parameter and FSMGen `--strict --check` gives 0 diagnostics; AXI's is byte-identical. Test `a_port_the_document_does_not_ground_on_the_lowered_actor_is_listed_not_declared`, RED-verified twice (no exclusion; no declared-direction exception).
- [x] **NO REGRESSION** — EvidenceIR, SemanticIR and IntentIR untouched (adapter-only); WIRE-BASED-100 unchanged on all 6 AMBA seeds; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,709; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +4 / +13 / +8 / +1.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the rule reads the actor-port graph and the declared directions; no actor, signal or protocol is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (*Which way does each signal point?*: the table and the dated history); ledger direction row moved to *What already flows end to end* and its citation removed; KM card `isf-initiator-perspective-direction`; `scripts/check_book_current_truth.sh`'s pinned code literal realigned (the initiator is now passed by reference; the contract it guards is unchanged, reverified); `.27` closed.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.10.a` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `ee4df2ec`, 18 of the 80 rules in AXI's `manager.isf` drive a port the Manager declares as an input (`ACTIVATEACK`, `ACWAKEUP`, `ASKSTOP`, `BCOMP`, `SYSCOACK`; `rules_on_inputs.py`), e.g. *ASKSTOP must be LOW when ACTIVATEACK is LOW*, which the Subordinate meets.
- [x] **ROOT CAUSE (WHY + WHERE)** — `lower_obligations` (`ir/isf_ir.rs`) and the temporal-contract loop lowered every obligation on a declared signal as the actor's `(rule …)` drive, whatever the signal's direction on the actor.
- [x] **ADDRESSED (verified)** — `LevelRefusal::DrivesAnInput` in `lower_obligations` and a matching arm on the temporal path. AXI 80 → 62 rules, the 18 removed exactly those driving inputs, 0 remaining; FSMGen `--strict --check` 0 diagnostics on both emitted files. Tests `an_obligation_on_the_actors_own_input_is_refused_not_driven`, `a_temporal_rule_on_the_actors_own_input_is_listed_not_driven`, both RED-verified.
- [x] **NO REGRESSION** — adapter replay: 23 of 27 identical; APB-E, AHB, Wishbone change only the wording of the obligations residual (the same counts, re-classified). Extraction untouched, so WIRE-BASED-100 cannot move; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,711; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census 0 / +5 / 0 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the rule reads the actor's declared port directions; no signal or protocol is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (*What value does a rule drive?* and the direction section's closing sentence); `.10` split.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.10.f` (CODE) — DONE `2026-09-25`

- [x] **REPRODUCE / MEASURE** — at `8eee3f92`, AXI's adapter reports `isf_rule_conflict_constraint_61` and `…_tinv_sc_dyn_sigcon_0063`: `ARLOCK` `0` against `0B0`, the same level (*tied LOW*, *Must be 0b0*).
- [x] **ROOT CAUSE (WHY + WHERE)** — `dedup_conflicting_rules` and `drop_unconditional_overlap_conflicts` (`ir/isf_ir.rs`) compared drive values as text.
- [x] **ADDRESSED (verified)** — `drive_agreement` and `same_level_duplicate_residual_packet`, used by both passes. Emitting both spellings was tried first and refused by FSMGen strict (`isf_conflicting_rule_writes` on `ARLOCK`), which is why the restatement is listed rather than emitted. AXI: `manager.isf` byte-identical, strict 0 diagnostics, 2 conflicts become same-level entries. Test `the_same_level_in_two_spellings_is_not_a_conflict` (both passes; a different number and a word still conflict; identical text is kept twice), RED-verified by classing the same number as different.
- [x] **NO REGRESSION** — adapter replay 27 of 27 identical after rebuilding AXI's adapter; extraction untouched; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,712; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +2 / +3 / +6 / +1.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — numbers are read by the shared numeric notation parser; no signal is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the conflict passes).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.10.c` and `.10.d` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `01c1ceb7`, AXI's adapter reports 16 rule conflicts (`AWIDUNQ` 0 against 1 ×14, `ARSNOOP` `0B1110` against 0 ×2) and 3 rule/transaction overlaps; its `manager.isf` drives `AWIDUNQ`, `AWSTASHLPIDEN`, `AWSTASHNIDEN` 0 and seven rows of *Table A15.23: DVM Complete request constraints* unconditionally (3 rules each); `.10.b` measured `condition_text: None` on *If present, AWIDUNQ must be asserted for Prefetch transactions* and *For non-stash transactions, …*, and the `RIDUNQ` sentences recorded on `AWIDUNQ`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_condition_clause` (`ir/evidence.rs`) read only infix *when/while/during/unless/provided* and took everything after the marker, so a fronted *If …, then …* with an inner *when* kept the obligation; *for* and *if* conditions and a table's caption were never read; list markers (*- -If*) hid a fronted condition from `main_clause_of_fronted_conditional` and `fronted_condition_subject`.
- [x] **ADDRESSED (verified)** — fronted conditions read whole, `stated_condition_clauses`, `names_an_operation`/`counts_operations`, `without_a_later_obligation`, `without_list_markers`, `condition_keyed_rows_on_their_tables_operation`. EvidenceIR against HEAD's binary: 23 of 27 identical; AXI 70 → 80 constraints (every added, removed and changed record read); AHB 11 → 10 (a table cell's condition had swallowed its obligation; corrected, the row reader's unconditional copy of the clause merges into it and stays listed); APB-E keeps *For read transfers* and *… if PWAKEUP and PSELx are HIGH in the same cycle*, ADIv6 *for as long as CSYSPWRUPREQ is asserted*. AXI adapter: rule conflicts 16 → 0, `manager.isf` loses 30 conditioned drives and gains the two `AWAKEUP` holds their sentences state; 17 more obligations listed under an unread condition (`.10.g`). Tests `a_constraint_keeps_the_condition_its_sentence_states`, `a_keyed_row_takes_the_operation_its_tables_caption_names`, `a_list_items_markers_do_not_hide_its_fronted_condition`; five RED-verified (no fronted-first read, no count refusal, no caption pass, no infix bound, no list-marker strip in either fronted reader).
- [x] **NO REGRESSION** — AXI, AHB, APB-E and ADIv6 rebuilt in order; AHB `manager.isf` byte-identical; FSMGen `--strict --check` 0 diagnostics on every emitted file. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds, identical to HEAD's binary; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,715; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +8 / +35 / +98 / +1.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — conditions are read by clause grammar (fronted markers, modals, commas, list markers) and a closed class of bus-operation nouns; a caption is found by its visual-asset link; no signal, actor or protocol is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the kept condition); `.10.e` re-measured; `.10.g` opened; KM card `a-condition-never-contains-its-obligation`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.10.e` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `ba3e82d4`, AXI's `manager.isf` drives `ARLOCK` 0, `AWLOCK` 0 and `ARSNOOP` 0 (2 rules each) and `WSTRB` 1 (3), and its adapter reports 3 `isf_rule_transaction_conflict_*` for `AWSNOOP` 0; the five constraints (`dyn_sigcon_0072`-`0074`, `0078`, `0079`) are read from *… tied LOW/HIGH* sentences and carry `must_be_low`/`must_be_high` with no condition. No other live chain has a *tied* constraint.
- [x] **ROOT CAUSE (WHY + WHERE)** — `logic_level_bindings` (`ir/evidence.rs`) counts *tied* as a binding verb like *drive* and mints a level obligation; nothing downstream can tell a tie-off from a drive, so `lower_obligations` (`ir/isf_ir.rs`) and the constraint-derived invariants and temporal rules lower it as the Manager's drive.
- [x] **ADDRESSED (verified)** — `SignalConstraintKind::TiedTo`, minted by `type_tie_offs` through `tie_off_bindings`; no consequent in `temporal_consequents_from_signal_constraint`, no invariant (`signal_constraint_to_temporal_invariant_kind` returns `None`), `ConstraintLevel::TieOff` refused as `LevelRefusal::TieOff`. EvidenceIR replay: 26 of 27 identical; AXI retypes exactly the five records, nothing else moves. AXI `manager.isf` loses exactly the 9 tie-off rules; the tie-off class lists 5; rule/transaction overlaps 3 → 0. Tests `a_tied_level_is_a_tie_off_unless_its_clause_states_a_scope`, `a_tie_off_is_listed_not_driven`, `a_tie_off_is_no_rules_consequent`, and `a_signal_constraint_states_its_level_with_negation_applied` extended; three RED-verified (no retyping pass, no scope guard, a tie-off lowered as a drive).
- [x] **NO REGRESSION** — AXI rebuilt in order; AHB `manager.isf` untouched; FSMGen `--strict --check` 0 diagnostics on both emitted files. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds, identical to HEAD's binary; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,718; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +3 / +9 / +19 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the rule is grammar: the binding verb *tied*, the adjacency walk the level reader already uses, and the condition reader; no signal, actor or protocol is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the value table and the conflict passage); the gap ledger's condition row; `.10.h` opened; KM card `a-tie-off-is-not-a-drive`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.10.h` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `2f9eaabf`, AXI's `manager.isf` drives `AWLOCK` 0 in 3 rules from *- AWLOCK must be deasserted, not exclusive access.*, an item of the list *For Atomic transactions, the following restrictions apply for request attributes:*; a census over the live chains finds 3 unconditioned constraints under a lead-in that fronts a condition and ends in a colon (AXI `AWLOCK`, `WTAGUPDATE`; AHB `HSIZE`).
- [x] **ROOT CAUSE (WHY + WHERE)** — each list item is its own statement, and `extract_condition_clause` (`ir/evidence.rs`) reads only the item's text, so the condition stated by the sentence introducing the list never reaches it.
- [x] **ADDRESSED (verified)** — `condition_list_items_on_their_lead_in` and `list_lead_in_condition`, composed after the caption pass. EvidenceIR replay: 25 of 27 identical; AXI changes exactly 5 records (`AWLOCK` under *For Atomic transactions*, and 4 items of two *When …, the following rules apply:* lists that gain the lead-in's condition, each read against the document), AHB 1 (`HSIZE`, *For all Device memory*); `WTAGUPDATE`'s items stay as written because their list is introduced by *It has the following rules:*. AXI `manager.isf` loses exactly the 3 `AWLOCK` rules. Test `a_list_item_keeps_the_condition_its_lead_in_states`, three RED-verified (no pass, the whole statement read instead of its colon sentence, no universal *For all*).
- [x] **NO REGRESSION** — AXI and AHB rebuilt in order; AHB `manager.isf` byte-identical (its obligation list loses only the unreadable-condition invariant copy of `HSIZE`, which its constraint still lists); FSMGen `--strict --check` 0 diagnostics on both. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds, identical to HEAD's binary; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,719; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +2 / +11 / +34 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — list markers, the colon that introduces a list, fronted condition markers, universal quantifiers and the closed operation-noun class; no signal, actor or protocol is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the kept-condition row and paragraph); the gap ledger's condition row; KM card `a-condition-never-contains-its-obligation` extended.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.25.c` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `17f0de12`, 9 constraints on the live chains state *must remain LEVEL until RELEASE* (AXI `AWAKEUP` ×4, `ACWAKEUP`, `ASKSTOP`, `ACVALID`, `CRVALID`; APB `PWAKEUP`) and are recorded as `must_be_high`/`must_be_asserted`; in 5 of them the release appears nowhere in the IR (`ACWAKEUP` with no condition at all), so the constraint path would lower a constant drive and the temporal path a hold that never releases. None is emitted today (Manager inputs, or an unread condition).
- [x] **ROOT CAUSE (WHY + WHERE)** — `classify_signal_constraint_kind` (`ir/evidence.rs`) folds *must remain HIGH* into `MustBeHigh`, and `parse_temporal_condition` (`ir/semantic.rs`) deliberately reads no *until* (`.25.a`), so the release had no carrier at any stage.
- [x] **ADDRESSED (verified)** — `SignalConstraintKind::MustRemainUntil` minted by `type_holds`/`hold_release` (the release cut only at a marker that opens a condition, so *until the rising clock edge after the Manager asserts …* stays whole) and refined through polarity; the release read into the rule's exceptions only when it states a value; `ConstraintLevel::Hold`/`LevelRefusal::Hold`; `unread_hold_release`; the implied exception dropped in `guard_from_candidates`. EvidenceIR replay: 25 of 27 identical; AXI and APB retype exactly the 9 records. AXI `manager.isf` byte-identical; its and APB's adapters change only the obligation residual (5 and 1 holds). Tests `a_remain_until_obligation_is_a_hold_with_its_release`, `a_hold_is_lowered_with_its_release_or_listed`, `a_signal_constraint_states_its_level_with_negation_applied` extended; `the_statement_class_and_not_the_grammar_decides_what_the_constraint_path_reads` now expects the hold. Seven RED-verified (no retyping, release not read, no guard simplification, no unread-release arm, no trailing-condition cut, *after* ending a release, a release without a stated value read).
- [x] **NO REGRESSION** — AXI and APB rebuilt in order; FSMGen `--strict --check` 0 diagnostics. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds after one gold refinement, `seed_apb` `statement_0322` (*PWAKEUP must remain asserted until PREADY is asserted …*) from `must_be_asserted` to `must_remain_until`, as the sentence states: HEAD's binary on HEAD's evidence scores 6/6 on the old gold and 5/6 on the new, the new binary 5/6 and 6/6; the other 5 seeds are identical. `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,721; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +3 / +27 / +67 / +2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — *must/shall remain/stay LEVEL until*, condition openers and a copula; no signal, actor or protocol is named.
- [x] **LOCKSTEP** — book `domain/temporal-semantics.md` (*A hold keeps its release*) and `pipeline/isf-adapter.md` (the value table); the APB gold's label note; KM card `a-hold-keeps-its-release`; `.25` closed; `.26.b` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.26.b` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — while `.25.c` was built, before its release reader required a stated value, reading AXI's `ACWAKEUP` release (*until the associated ACVALID / ACREADY handshake to ensure progress of the DVM transaction*) produced the predicates `ACVALID = DVM` and `ACREADY = DVM`. At `d97ffca3` the live SemanticIR's 4 non-level predicates (AHB `HRESP = ERROR`) are all stated with a copula, so no live condition is misread.
- [x] **ROOT CAUSE (WHY + WHERE)** — `temporal_clause_value` (`ir/semantic.rs`) falls back to `extract_symbolic_value`, the first capitalized word after the signal that is not a level keyword or a known signal, whatever the clause says of it.
- [x] **ADDRESSED (verified)** — `symbolic_value_is_stated`: a copula (*is*, *are*, *was*, *were*, *be*, *equals*) or an equality before the value. Test `a_symbolic_value_is_one_the_clause_states` (stated, equality, level, and the unstated *… of the ZETA transaction*), RED-verified by removing the filter (`Some("ZETA")`).
- [x] **NO REGRESSION** — SemanticIR replay 27 of 27 identical (evidence untouched); WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; `check_chain_currency.sh`: 27 of 27 current at every stage, 2 emitted `.isf` files. Lib tests 473 / 169 / 1,722; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +1 / +3 / +3 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — copulas and an equality sign; no signal or protocol is named.
- [x] **LOCKSTEP** — book `domain/temporal-semantics.md` (*Reading a condition whole*); the gap ledger's condition row; KM card `a-hold-keeps-its-release`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.7.d` (DECISION + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `e382d2e5`, APB-E's adapter is blocked with *no source-grounded system clock/reset contract*; a probe of `build_system_contract` reads `PCLK`, `PRESETn` and active-low from statements `0580`/`0607` and no kind, so the contract is refused. Of the 27 live chains only APB-E has that shape (SemanticIR replay: 26 identical).
- [x] **ROOT CAUSE (WHY + WHERE)** — `build_system_contract` (`ir/semantic.rs`) required a stated kind (`reset_kind?`), and `SystemResetKind` had no value for a kind the document does not state; the windowed-event arm of `classify_actor_contract` (`ir/isf_ir.rs`) matched `EventExpr::Level { signal, .. }` and dropped the value.
- [x] **ADDRESSED (verified)** — `SystemResetKind::NotStated` (with `NotStated` timing and target); the adapter's kind choice and `reset_kind_not_stated_residual_packet`; `windowed_event_refusal` in both classifiers. APB-E: SemanticIR gains the contract (and the clock and reset as inputs of each actor, with 2 direction conflicts against its source-side declarations recorded), and the rebuilt chain emits `requester.isf` with `(reset (PRESETn async active_low))`, strict-clean; its 3 windowed *must be valid* rules are listed instead of lowered. Tests `an_unstated_reset_kind_is_recorded_as_not_stated`, `a_contract_is_refused_when_its_statements_disagree` (the APB-E case moved out of it), `an_unstated_reset_kind_is_chosen_to_keep_the_polarity_and_listed`, `a_windowed_event_lowers_only_a_signal_becoming_true`; four RED-verified (kind required again, no polarity branch, not listed, the value dropped).
- [x] **NO REGRESSION** — SemanticIR replay 26 of 27 identical; APB-E rebuilt in order; AXI's and AHB's `manager.isf` untouched; FSMGen `--strict --check` 0 diagnostics on all three emitted files. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, now 3 emitted `.isf` files. Lib tests 473 / 169 / 1,725; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +2 / +4 / +3 / +2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the contract's statements and the stated polarity decide; no reset name is read (`PRESETn` is not read as active-low from its `n`).
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (*Where do the clock and reset come from?*, the Contract disposition, the direction section); the gap ledger's clock/reset row; `.29`-`.31` opened; KM card `an-unstated-reset-kind-is-a-listed-choice`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.31` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `54ea4d8b`, APB-E's `sigcon_0002`-`0004` (*… PSEL , is asserted, which means that PADDR , PWRITE , and PWDATA must be valid*) carry no condition; on the live chains this is the one inference with a modal after it that yields constraints (AXI's and ADIv6's yield none).
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_signal_constraints` (`ir/evidence.rs`) read the condition from the consequent only (`consequent_after_inference_marker`), so the antecedent was dropped; `read_condition_clauses` (`ir/semantic.rs`) dropped a comma-split predicate with no signal of its own; `contract_from_temporal_rule` (`ir/contract.rs`) gave a windowed consequent with no value the level `1`, which `.7.d`'s value check could not see.
- [x] **ADDRESSED (verified)** — `inference_split`/`inference_antecedent` with the own-subject exception; `appositive_predicate_level`; the value-less windowed branch in the conversion and the classifier. EvidenceIR replay: APB-E's three records gain *The select signal, PSEL , is asserted*, 26 of 27 identical; SemanticIR replay: only APB-E's contracts move (`0005`/`0006` become `Stable`, listed). APB-E's `requester.isf` byte-identical to HEAD. Tests `an_inferred_obligation_holds_under_its_antecedent`, `an_appositive_predicate_closes_its_member`, `a_windowed_stability_is_kept_stable_and_not_lowered` (it replaces the parity-finding test that pinned the default `1`), `a_windowed_event_lowers_only_a_signal_becoming_true` extended; four RED-verified (no antecedent, no appositive closing, no value-less branch, no own-subject exception: the frozen inference matrix and the new test both fail).
- [x] **NO REGRESSION** — APB-E rebuilt in order; FSMGen `--strict --check` 0 diagnostics on all three emitted files. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; the frozen inference-antecedent matrix passes. `check_chain_currency.sh`: 27 of 27 current at every stage, 3 emitted `.isf` files. Lib tests 473 / 169 / 1,727; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +3 / +2 / +20 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — inference markers, appositive punctuation, copulas and predicate kinds; no signal or protocol is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the condition table and the Contract disposition); `.30` re-measured; KM card `a-condition-never-contains-its-obligation` extended.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.30` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `594d4a04`, the live IntentIR carries 9 fused contracts; 3 (AXI `dyn_sigcon_0075`+`0077` `AWBURST` INCR/WRAP, `sigcon_0002`+… `BCOMP` HIGH/LOW, `sigcon_0026`+… `WTAG` ZERO/VALID) are `Residual { disagreement: obligation }`, and the adapter lists them under the first member's refusal (*not an ISF literal*, *an input of the actor*), as a lone `Drive` would be.
- [x] **ROOT CAUSE (WHY + WHERE)** — `classify_actor_contract` (`ir/isf_ir.rs`) matches the obligation first and reads `contract.lowering` only in the fallback arm, so a fused `Drive` or windowed contract bypasses its disagreement marker.
- [x] **ADDRESSED (verified)** — a fused contract marked residual returns a residual naming the disagreement. Adapter replay: 26 of 27 identical; AXI changes exactly the three disagreement reasons, its `manager.isf` byte-identical. Test `a_disagreeing_fusion_is_listed_not_lowered_as_its_first_member` (a disagreeing fusion listed; an agreeing one still a rule), RED-verified by removing the check (the first member's drive is lowered).
- [x] **NO REGRESSION** — AXI's adapter rebuilt; extraction untouched, so WIRE-BASED-100 cannot move; `kg-bench` 158/158; the contract parity tests pass (a handshake barrier still lowers as a stage). `check_chain_currency.sh`: 27 of 27 current at every stage, 3 emitted `.isf` files. Lib tests 473 / 169 / 1,728; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census 0 / +3 / 0 / 0.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the contract's fusion marker decides; no signal is named.
- [x] **LOCKSTEP** — book `direction/temporal-intent-capture.md` (the fusion guarantee, corrected); `.30`'s goal corrected in the tree.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.29` (DECISION + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `c5109e41`, all 6 windowed temporal rules on the live chains are APB-E's, each windowed by a diagram label (*at T1 in Figure 3-1*, *at T2*) with no antecedent; the adapter's unguarded branch rendered such a rule as `(monitor (within S N))` in an `(on start)` transaction (test `classify_windowed_signalvalue_is_contract` pinned it).
- [x] **ROOT CAUSE (WHY + WHERE)** — `windowed_eventual_prop` (`ir/isf_ir.rs`) gave an unguarded window the monitor form, which FSMGen arms at the transaction's `(on start)`, so the window counted from the start of the run, an anchor the document never states.
- [x] **ADDRESSED (verified)** — `windowed_eventual_prop` returns a `Result`, refusing a window with no antecedent (and, as before, a guarded 0 lower bound); both classifiers take its reason. Test `an_unguarded_window_has_no_event_to_count_from` (both classifiers; it replaces the test that pinned the monitor), and `a_windowed_event_lowers_only_a_signal_becoming_true` now states its antecedent; RED-verified by restoring the monitor branch.
- [x] **NO REGRESSION** — adapter replay 27 of 27 identical (no live rule reached the branch); WIRE-BASED-100 temporal seeds 1.000; `kg-bench` 158/158 (the diagram-position fixtures untouched). `check_chain_currency.sh`: 27 of 27 current at every stage, 3 emitted `.isf` files. Lib tests 473 / 169 / 1,728; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census 0 / 0 / 0 / -3.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — a structural rule over the rule's antecedents; no signal is named.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the Contract disposition's unguarded case).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.7.c` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `6d254409`, Wishbone's adapter is blocked (*no source-grounded system clock/reset contract*) and its EvidenceIR carries no `Clock`/`Reset` statement, although its prose states the clock input (`0385`), the reset input (`0388`), the reset's kind (`0433`) and a polarity rule over its interface signals (`0380`).
- [x] **ROOT CAUSE (WHY + WHERE)** — the clock was read only from signal tables (Wishbone has none); `stated_reset_descriptor` (`ir/evidence.rs`) did not call *reset [X] signal* the reset nor read a kind from a relative clause; nothing read a polarity stated over a class of signals; and `build_system_contract` (`ir/semantic.rs`) took the heading *CLOCK GENERATION* for a second clock, refusing the contract.
- [x] **ADDRESSED (verified)** — `synthesize_system_clock_prose_statements`, `relative_clause_protocol_kind`, `synthesize_reset_class_rule_polarity_statements`, and the sentence check in `build_system_contract`. EvidenceIR replay: 26 of 27 identical, Wishbone gains exactly `Clock CLK_I.`, `Reset RST_I is synchronous.`, `Reset RST_I is active high.` (the clock reader first also caught I2C's *serial clock signal SCL* and restated AXI/AHB/APB clocks; narrowed to a clock input the document does not already declare, those left). SemanticIR replay: only Wishbone gains a contract. Wishbone emits `slave.isf` with `(reset (RST_I sync active_high))`, FSMGen `--strict --check` 0 diagnostics. Tests `a_reset_kind_stated_in_a_relative_clause_on_the_signal`, `a_clock_input_named_in_prose_is_the_clock_unless_the_document_declares_one`, `a_reset_polarity_stated_over_a_class_the_document_places_it_in` (each link needed), `a_heading_is_no_second_clock`; four RED-verified (no relative kind, no declared-clock skip, no class link, no sentence check; a first draft that required a declared signal instead failed six adapter tests whose markdown declares its clock with `Clock clk.`, and was replaced).
- [x] **NO REGRESSION** — AXI's, AHB's and APB-E's artifacts untouched; FSMGen strict 0 diagnostics on all four emitted files. WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158. `check_chain_currency.sh`: 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 473 / 169 / 1,732; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit; flow census +4 / +42 / +70 / +1.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — clause grammar (*clock input*, *reset … signal*, a relative clause, *All Q interface signals*, *the R interface MUST include*, *Q R and R2 interfaces*); no signal name is read for its role or polarity.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (the clock/reset table); the gap ledger's clock/reset row; `.32` opened; KM card `wishbone-clock-and-reset-come-from-prose`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.32` (MEASUREMENT + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `40d8dda9`, Wishbone's `slave.isf` declares `ACK_I`, `ERR_I`, `RTY_I` as SLAVE outputs (relations `asr_0006`-`0008`, from statement `0401`, *The SLAVE asserts either the [ACK_I], [ERR_I] or [RTY_I] signals*) and leaves out the SLAVE's `STB_I`; a numbered-section census of its SourceIR finds every port assigned to a role, with a direction, in sections 2.2.1-2.2.4.
- [x] **ROOT CAUSE (WHY + WHERE)** — relations came from tables and prose only, so a document that assigns ports by section had its ports grounded by prose that names the other interface's port; and `select_initiator_actor` (`ir/isf_ir.rs`) counted the contract's clock and reset, which every actor reads, so correcting the SLAVE left the MASTER tied and no initiator chosen (the module then declared both sides of every wire).
- [x] **ADDRESSED (verified)** — `extract_relations_from_role_signal_sections` and its helpers, `supersede_relations_by_role_sections`, `select_initiator_for`. EvidenceIR replay: 26 of 27 identical; Wishbone's relations lose exactly the 4 other-side relations and gain the 17 its sections state. Adapter replay: 26 of 27 identical (AXI, AHB, APB-E keep their initiators). Wishbone emits `master.isf` with exactly the MASTER's 12 ports, FSMGen strict 0 diagnostics. Tests `a_role_signal_section_grounds_its_ports_and_supersedes_the_other_side`, `the_initiator_is_chosen_without_the_system_clock_and_reset`; three RED-verified (no supersession, no shared section, the clock counted).
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; FSMGen strict 0 diagnostics on all four emitted files. `check_chain_currency.sh`: 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 473 / 169 / 1,734; `cargo fmt --check` and `cargo clippy -D warnings` clean after the last edit (a lint attribute the insertion had displaced was moved back to its function); flow census +6 / +75 / +134 / +2.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — numbered headings, *Signals* / *Signals Common to … Interfaces* titles, *input*/*output* before a bracketed signal; no port direction is read from a `_I`/`_O` suffix.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (*Which way does each signal point?*); `scripts/check_book_current_truth.sh`'s pinned initiator literal realigned (the initiator is still selected from `actor_ports`, reverified); `.17` notes Wishbone's class rule; KM card `wishbone-clock-and-reset-come-from-prose` extended.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.17` (MEASUREMENT + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `00ebcf0d`, the 27 live adapters hold back 19 obligations, 16 temporal rules and 1 guard as *the document never states the signal's polarity* (Wishbone's *MASTER interfaces MUST assert [CYC_O]* among them), while Wishbone states *All WISHBONE interface signals MUST use active high logic* and carries 0 polarity records, AHB states *HRESETn … is the only active LOW signal in the protocol* and carries 1, and ADIv6's B2.3.2 states *All signals that are described in this section are active-high*. A census over all 78 persisted evidence chains finds five documents with a scope rule (the leaf above).
- [x] **ROOT CAUSE (WHY + WHERE)** — the prose polarity strategy (`extract_signal_polarity_from_prose`) reads a polarity only for a signal a sentence names, or for two or more a collective subject names; a rule whose subject is a class, *all but one*, or a section names none, and `.7.c`'s class-rule reader (`synthesize_reset_class_rule_polarity_statements`) served the reset alone. Its members also came only from the minimum lists, one of which Wishbone splits across two statements.
- [x] **ADDRESSED (verified)** — `scope_rule_polarity_observations` joins the prose strategy with `interface_class_polarity_rule`, `interface_class_members` (minimum lists and `.32`'s role sections), the exclusive statement over the declared catalog, and the section rule over `statement_section_indices`; the reset statement now follows the resolved record the rule gives. RED on each of four mutations (role sections ignored, complement dropped, section unscoped, reset ignoring its record) in `a_class_rule_reaches_every_signal_the_document_places_in_the_class`, `the_only_signal_of_one_polarity_gives_every_other_signal_the_other`, `a_section_rule_reaches_the_signals_its_own_section_names`, `a_reset_polarity_stated_over_a_class_the_document_places_it_in`. EvidenceIR replay: 24 of 27 identical; polarity records AHB 1 → 41, ADIv6 4 → 5, Wishbone 0 → 23, no conflict; live total 51 → 115 in 5 documents, active-low 5 → 5. Held back after: AXI's 17 obligations, 14 temporal rules and 1 guard only. **Found here and fixed**: Wishbone's `CYC_O` constraint was an order between events read as a level and lowered as three always-HIGH `(CYC_O 1)` rules once its polarity was read; `states_a_level_bounded_by_an_event` refuses it (`a_level_bounded_by_an_event_is_not_a_level_obligation`, RED with the refusal disabled). Wishbone's `master.isf` is byte-identical to `00ebcf0d`'s, AHB's `manager.isf` unchanged.
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; FSMGen strict 0 diagnostics on the emitted files. `check_chain_currency.sh`: 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 473 / 169 / 1,738 passed (4 new), `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the grammars are *All `<class>` interface signals must use active `<level>` logic*, *`<X>` is the only active `<level>` signal in the protocol* and *All signals [that are] described in this section are active `<level>`*; members are named exactly as declared and never through an inversion mark; tests use renamed signals. AXI's *Lowercase n … denotes an active-LOW signal* is a spelling rule and stays unread.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (scope rules), `pipeline/obligation-reading.md` (event-bounded level), `pipeline/isf-adapter.md` (polarity sources), `reference/representation-gaps.md` (polarity row); KM card `a-polarity-rule-reaches-its-stated-scope`; `.17.a` and `.33` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.33` (MEASUREMENT + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `fc04dd39`, Wishbone's IntentIR holds `tinv_sc_sigcon_0002`, *RST_I* `must_be_asserted` `1` with no condition, from *[RST_I] MUST be asserted for at least one complete clock cycle*, while SemanticIR's `temporal_signal_constraint_sigcon_0002` carries `cycle_window {min_cycles: 1}`. Over the 27 live chains, 5 IntentIR invariants come from a constraint whose own rule is windowed; 20 normative statements carry a time-bound shape, and only `RST_I`'s yields a signal record (the leaf above).
- [x] **ROOT CAUSE (WHY + WHERE)** — `synthesize_temporal_invariants` (`ir/intent.rs`) makes an invariant from every level or stability constraint whose condition it can read (`.22`), and never asks whether the statement places the constraint at a cycle or a window; SemanticIR reads the window into the constraint's rule, and IntentIR ignored it.
- [x] **ADDRESSED (verified)** — `signal_constraint_rule_id` (`ir/semantic.rs`) names the rule a constraint yields, used where SemanticIR mints it and where IntentIR looks it up; the invariant is skipped when that rule has a window. `a_constraint_placed_at_a_window_is_not_an_invariant` (end to end, renamed signals) is RED with the skip disabled. Rebuilding the 27 chains from SemanticIR: IntentIR loses exactly `tinv_sc_sigcon_0002` (Wishbone), `0005`/`0006`/`0018` (APB-E) and `0004` (AHB); every semantic artifact is unchanged (compared as JSON) and every emitted `.isf` byte-identical.
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; `check_chain_currency.sh` 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 473 / 169 / 1,739 passed (1 new); `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the decision reads a typed record (a rule's `cycle_window`) through the stage's own derivation id; no text or name is read at IntentIR.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (a constraint placed at a window yields no invariant) and `pipeline/obligation-reading.md` (`.17`'s claim corrected: an event-bounded statement is counted nowhere, `.34`); `ir/evidence.rs`'s `.17` comment corrected the same way; KM card `an-invariant-holds-at-every-cycle`; `.34` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.34` (MEASUREMENT + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `77295ea2`, Wishbone's statement `0438` (classed `signal_value_constraint`, cited by no record since `.17`) appears in no residual count and no finding; over the 27 live chains 96 constraint-classed and 34 rule-classed statements are cited by no signal constraint or conditional rule, and the normative finding lists 3,026 statement ids for a count of 3,008 (the leaf above).
- [x] **ROOT CAUSE (WHY + WHERE)** — `uncaptured_normative_statement_ids` (`ir/completeness.rs`) filters on `NormativeStatement` only, while the passive constraint path it was meant to account for reads only `SignalValueConstraint` statements; and `evidence_normative_residual_statement_ids` (`commands/validate.rs`) filtered by class without the citation set the count used.
- [x] **ADDRESSED (verified)** — `uncaptured_classified_obligation_statement_ids`, `evidence_captured_statement_ids`, `evidence_classified_obligation_residual_statement_ids`; metric `classified_obligation_residuals` inside `completeness_candidate_misses`; finding `evidence_classified_obligation_residuals_remaining`. `an_uncited_classified_obligation_is_a_candidate_miss` (end to end, renamed signals) and `an_uncited_classified_obligation_is_a_residual` are RED under four mutations (count left out of the total, normative list ignoring citations, rule class dropped, finding never pushed). Re-validating the 27 chains: 130 statements newly counted on 17 chains, no count falls; semantic, intent and adapter artifacts move only in proof and validation records, no `.isf` moves.
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; `check_chain_currency.sh` 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 474 / 170 / 1,739 passed (2 new); `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the count reads statement classes and record citations only; no document text or name.
- [x] **LOCKSTEP** — book `quality/validation.md` (the new residual), `pipeline/evidence-rule-chronology.md` (`candidate_misses` components), `pipeline/obligation-reading.md` (which count holds a refused statement); `ir/evidence.rs`'s four *stays counted* comments corrected; KM card `an-uncited-obligation-is-a-candidate-miss`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.24.b` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `2155d8b1`, APB-E's `sigcon_0007` (*PNSE must be valid when PSEL is asserted*) has a contract guarded on `PSELx` but no IntentIR invariant, because `extract_condition_from_text` reads only declared or case-folded names while the temporal reader resolves `PSEL` through `document_signal_coreferences`; no IR records that `PSELx` has one instance per Completer or `HSELx` one per Subordinate, and `requester.isf` declares one `PSELx` without saying so (`.24.a`'s measurement).
- [x] **ROOT CAUSE (WHY + WHERE)** — the co-reference was computed inside `build_temporal_rules` (`ir/semantic.rs`) and never persisted, so IntentIR had no way to read it; no reader typed the document's *for each ROLE* / *Each ROLE has its own* sentence.
- [x] **ADDRESSED (verified)** — `SignalCoreferenceRecord`/`SignalFamilyRecord`, `document_signal_coreference_records`, `document_signal_family_records`, `signal_coreference_map` (passed into `build_temporal_rules`), IntentIR's carried `signal_families` and co-reference-aware `extract_condition_from_text`, the adapter's `family_driven_once_residual_packet`. Tests `a_family_is_one_declared_signal_per_declared_actor`, `a_co_reference_record_keeps_its_role_phrase_and_statements`, `a_family_the_actor_drives_is_declared_once_and_listed`, and `a_constraint_condition_is_read_whole_or_the_invariant_is_not_made` (extended) are RED under four mutations (own-instance clause ignored, two-signal sentences read, co-reference lookup disabled, input families listed). Regenerating the 27 chains: content moves only in APB-E (co-reference, family, `tinv_sc_sigcon_0007`, the adapter residual) and AHB (family); no `.isf` moves.
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; `check_chain_currency.sh` 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 474 / 170 / 1,742 passed (3 new); `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean; the rule inventory (`claim_family_inventory.tsv`, `rule_family_inventory.tsv`, 170 → 173 rules) and the pinned `rule_roots` (120 → 122, reason recorded) move together.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the family grammar is *for each ROLE* or *Each ROLE has its own …* with ROLE a declared actor and exactly one declared signal in the sentence; the co-reference is the document's repeated role phrase; nothing is read from a trailing `x`; tests use renamed signals and actors.
- [x] **LOCKSTEP** — book `domain/temporal-semantics.md` (the persisted co-reference and the family), `pipeline/isf-adapter.md` (a driven family is listed), `reference/representation-gaps.md` (the family row read; ownership citation of `.24` made historical); KM card `a-per-target-family-is-typed-intent`; `.24` closes.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.10.g.ii` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `b4e3b4b3`, APB-E's `requester.isf` lists `constraint_15` and `constraint_17` (`PSTRB` must be LOW, conditions *For read transfers*, *a read transfer*) as *states a condition that is not `SIGNAL is LEVEL`*, while statement `0170` states *PWRITE indicates an APB write access when HIGH and an APB read access when LOW*; `.10.g`'s measurement above covers the other 17 (AXI).
- [x] **ROOT CAUSE (WHY + WHERE)** — no reader typed a sentence that states which level of a signal means a kind of operation, so every condition reader (`parse_temporal_condition` in `ir/semantic.rs`, `extract_condition_from_text` in `ir/intent.rs`, `condition_guard` in `ir/isf_ir.rs`) saw *read transfers* as words with no signal.
- [x] **ADDRESSED (verified)** — `OperationKindRecord`, `document_operation_kind_records`, `operation_kind_condition` (applied before each of the three readers), IntentIR's carried `operation_kinds`. `a_signal_level_that_states_an_operation_kind_is_its_guard` and `a_condition_naming_an_operation_kind_is_the_level_that_states_it` (end to end, renamed signals, each reader's rule asserted by name) are RED under four mutations (adapter, invariant and temporal reader each disabled; a contested kind chosen). Regenerating the 27 chains: content moves only in APB-E, whose `requester.isf` gains 5 rules `(== PWRITE 0)` → `(PSTRB 0)`, FSMGen strict 0 diagnostics.
- [x] **NO REGRESSION** — **found and corrected**: WIRE-BASED-100 `seed_apb_temporal` fell 1.000 → 0.667, because its gold for statement `0221` held no antecedent, with the note *the 'read transfer' guard has no signal-value predicate*. The document states one (`0170`), so the gold now carries `PWRITE` LOW with a note citing it and this leaf (`test_data/llm_eval/seed_apb_temporal.json`, README); re-scored 1.000 (3/3), and the other 5 AMBA seeds stay 1.000. `kg-bench` 158/158; `check_chain_currency.sh` 27 of 27 current at every stage. Lib tests 474 / 170 / 1,744 passed (2 new); `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean (`lower_obligations` carries a reasoned `#[expect(too_many_arguments)]`); inventory 173 → 175 rules, `rule_roots` 122 → 123 (reason recorded).
- [x] **GENERICITY (ADR 0006, ADR 0037)** — the grammar is *<clause naming one declared signal> indicates … <kind> <operation> when HIGH|LOW*, the operation a closed grammatical class; the kind is the document's word, matched only against itself; tests use renamed signals.
- [x] **LOCKSTEP** — book `pipeline/isf-adapter.md` (a named kind of operation is the level stating it; opcode tables route to `.10.g.iii`), `reference/representation-gaps.md` (the condition row); KM card `an-operation-kind-a-level-states-is-a-guard`; `.10.g` measured and decided, `.10.g.iii` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.35` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `35b9ce2d`, AXI's `condrule_0152` reads *AWSNOOP is not CMO* from *When AWSNOOP is not CMO, WritePtlCMO or WriteFullCMO, AWCMO must be 0b000*, and `sigcon_0038`-`_0046` read no condition from *If present, AWSTASHNID, AWSTASHNIDEN, AWSTASHLPID, and AWSTASHLPIDEN must be LOW*, so `manager.isf` drives the four LOW always; 3 conditional rules on the live chains end before a comma list (the leaf above).
- [x] **ROOT CAUSE (WHY + WHERE)** — `split_conditional_sentence` (`ir/evidence.rs`) cut a fronted condition at its first comma, and `main_clause_of_fronted_conditional` knew a list only by an Oxford comma and read nothing when every comma was a list comma and the condition had no verb.
- [x] **ADDRESSED (verified)** — `fronted_condition_end` shared by both readers (Oxford rule unchanged; a non-Oxford list closes where the main clause keeps a subject; a same-sentence modal bounds the conditional-rule reader), and a verbless all-list condition ends at the first comma. `a_fronted_condition_keeps_every_member_of_its_list` (renamed AXI, USB and USB4 sentences) is RED under four mutations (first comma always, non-Oxford list ignored, subject guard dropped, verbless fallback dropped); the pre-existing boundary tests (`the_boundary_is_a_comma_that_is_not_part_of_a_list` and two siblings) stay green, and caught a first cut that broke them. EvidenceIR replay against the pre-`.35` evidence: 16 conditional rules and 8 constraints change, each read against its source as a fix; `manager.isf` loses 18 `AWSTASH*` rules, adds none, FSMGen strict 0 diagnostics.
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; `check_chain_currency.sh` 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 474 / 170 / 1,745 passed (1 new); `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — commas, *and*/*or*, *must*/*shall* and a sentence end; no signal or document word.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` (two boundary rows and the dated correction); KM card `a-fronted-condition-ends-after-its-list`; `.10.g.iii` measured and blocked on `.4`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.11.a` (MEASUREMENT + CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — at `7d0b0b00`, 5,089 of the 5,162 IntentIR invariants lifted from SemanticIR's statement invariants on the 27 live chains carry `must_remain_stable` although their text names no stability (*The PE cannot service interrupts in Debug state.*); 73 match one of `classify_invariant_text`'s six phrases.
- [x] **ROOT CAUSE (WHY + WHERE)** — `classify_invariant_text` (`ir/intent.rs`) returned `MustRemainStable` for any text it could not classify (*default for chip invariants*), and `TemporalInvariantKind` had no kind for text that states none.
- [x] **ADDRESSED (verified)** — `TemporalInvariantKind::NotStated`, the classifier's default; `temporal_invariant_kind_classification` extended, RED with the old default restored. Regenerating the 27 chains: exactly 5,089 invariants flip `must_remain_stable` → `not_stated`, and no semantic record, adapter or `.isf` changes.
- [x] **NO REGRESSION** — WIRE-BASED-100 = 1.000 on all 6 AMBA seeds; `kg-bench` 158/158; `check_chain_currency.sh` 27 of 27 current at every stage, 4 emitted `.isf` files. Lib tests 474 / 170 / 1,745 passed; `cargo fmt --check` and `cargo clippy --workspace --all-targets -D warnings` clean; the flow census does not move.
- [x] **GENERICITY (ADR 0006, ADR 0037)** — no reading added: text that matches no phrase is recorded as stating no kind.
- [x] **LOCKSTEP** — book `pipeline/intentir.md` (the kind an invariant carries), `reference/representation-gaps.md` (the stability row); KM card `an-invariant-that-states-no-kind-is-not-stated`; `.11` stays open for lowering the stated kinds as assertions.

## Commit Log

- `.0` — `EXTRACTION-REPRESENTATION-GAPS.0 — one ledger of extraction and representation gaps, owned per side`.
- `.1` — `EXTRACTION-REPRESENTATION-GAPS.1 — the gap ledger's owners are checked, in either leaf grammar`.
- `.2` — `EXTRACTION-REPRESENTATION-GAPS.2 — census: every live adapter is blocked, and 150 residuals attributed`.
- `.8` — `EXTRACTION-REPRESENTATION-GAPS.8 — a rule drives only the level the document states`.
- `.7.b` — `EXTRACTION-REPRESENTATION-GAPS.7.b — a reset's kind and polarity come from the document's sentences`.
- `.22` — `EXTRACTION-REPRESENTATION-GAPS.22 — a rule keeps the condition it states, or is not emitted`.
- `.25.a` — `EXTRACTION-REPRESENTATION-GAPS.25.a — conditions fuse only when they agree, and each signal keeps its own`.
- `.24.a` — `EXTRACTION-REPRESENTATION-GAPS.24.a — the PSELx family: read through the document's own words, no FSMGen request`.
- `.28` — `EXTRACTION-REPRESENTATION-GAPS.28 — room for the ledger's owner citations, and a claim that checks its own sentence`.
- `.26` — `EXTRACTION-REPRESENTATION-GAPS.26 — a temporal condition is read whole, and an unless clause is kept as an exception`.
- `.27.a` — `EXTRACTION-REPRESENTATION-GAPS.27.a — why 183 emitted ports have no grounded direction, class by class`.
- `.27.b` — `EXTRACTION-REPRESENTATION-GAPS.27.b — a continuation page states its sources like the page it continues`.
- `.27.d` — `EXTRACTION-REPRESENTATION-GAPS.27.d — a check table is read by the signals it covers, whatever it calls its name column`.
- `.27.c` — `EXTRACTION-REPRESENTATION-GAPS.27.c — a signal stated as an actor's input is read as that actor's input`.
- `.27.f` — `EXTRACTION-REPRESENTATION-GAPS.27.f — a relation verb's object stops where another agent's clause begins`.
- `.27.e` — `EXTRACTION-REPRESENTATION-GAPS.27.e — an emitted actor declares only the ports the document grounds on it`.
- `.10.a` — `EXTRACTION-REPRESENTATION-GAPS.10.a — an obligation on a signal the actor reads is not lowered as its drive`.
- `.10.b` — `EXTRACTION-REPRESENTATION-GAPS.10.b — the 21 remaining conflicts, each a condition the sentence states`.
- `.10.f` — `EXTRACTION-REPRESENTATION-GAPS.10.f — one level in two spellings is a restatement, not a conflict`.
- `.10.c` — `EXTRACTION-REPRESENTATION-GAPS.10.c — a constraint keeps the condition its sentence or its table states`.
- `.10.e` — `EXTRACTION-REPRESENTATION-GAPS.10.e — a tie-off is recorded as one, and no stage lowers it as a drive`.
- `.10.h` — `EXTRACTION-REPRESENTATION-GAPS.10.h — a list item keeps the condition the sentence introducing it states`.
- `.25.c` — `EXTRACTION-REPRESENTATION-GAPS.25.c — a hold keeps its release, and is never a constant drive`.
- `.26.b` — `EXTRACTION-REPRESENTATION-GAPS.26.b — a condition's symbolic value is one its clause states`.
- `.7.d` — `EXTRACTION-REPRESENTATION-GAPS.7.d — an unstated reset kind is SpecForge's listed choice, and APB-E emits`.
- `.31` — `EXTRACTION-REPRESENTATION-GAPS.31 — an inferred obligation holds under the clause it is inferred from`.
- `.30` — `EXTRACTION-REPRESENTATION-GAPS.30 — a fusion whose sentences disagree is listed, not lowered as its first`.
- `.29` — `EXTRACTION-REPRESENTATION-GAPS.29 — a window lowers only against the event it counts from`.
- `.7.e` — `EXTRACTION-REPRESENTATION-GAPS.7.e — a reset-less document needs no FSMGen request; its route is .7.e.i`.
- `.7.c` — `EXTRACTION-REPRESENTATION-GAPS.7.c — Wishbone's clock and reset come from its prose, and it emits`.
- `.32` — `EXTRACTION-REPRESENTATION-GAPS.32 — a port is grounded on the role whose section names it`.
- `.17` — `EXTRACTION-REPRESENTATION-GAPS.17 — a polarity stated for a scope reaches the signals it places there`.
- `.33` — `EXTRACTION-REPRESENTATION-GAPS.33 — a constraint placed at a window is not an every-cycle invariant`.
- `.34` — `EXTRACTION-REPRESENTATION-GAPS.34 — an obligation no record cites is a candidate miss, whatever its class`.
- `.24.b` — `EXTRACTION-REPRESENTATION-GAPS.24.b — a per-target family is typed intent, and every reader resolves the generic name alike`.
- `.10.g.ii` — `EXTRACTION-REPRESENTATION-GAPS.10.g.ii — a kind of operation a signal's level states is the guard a condition naming it reads as`.
- `.35` — `EXTRACTION-REPRESENTATION-GAPS.35 — a fronted condition keeps every member of the list it names`.
- `.11.a` — `EXTRACTION-REPRESENTATION-GAPS.11.a — an invariant whose text states no kind says so`.
- `.3` — `EXTRACTION-REPRESENTATION-GAPS.3 — the per-category census comes first; one replayable document per under-covered road`.
- `.36` — `EXTRACTION-REPRESENTATION-GAPS.36 — every missing piece from PDF to .isf is indexed in docs/gaps/, with its owner`.
- `.3.a` — `EXTRACTION-REPRESENTATION-GAPS.3.a — four more roads replay on current code, and the register-map path merges registers across tables`.
- `.3` census — `EXTRACTION-REPRESENTATION-GAPS.3 — the per-family census on current code: 2,793 captured regions reach no carrier`.
- `.3.c` encoding — `EXTRACTION-REPRESENTATION-GAPS.3.c — 15 of 91 uncarried encoding tables are carried and miscounted`.
- `.3.c` causes — `EXTRACTION-REPRESENTATION-GAPS.3.c — 66 of 73 encoding tables are refused a name by design`.
- `.3.c` first read — `EXTRACTION-REPRESENTATION-GAPS.3.c — the other 43 typed tables, first read: I2C's core timing table and TMC's CLK reach no record`.

## Changelog

- `2026-09-25` — tree opened on the owner's directive; `.0` done (ADR 0053, ledger chapter); `.1`-`.6` open.
- `2026-09-25` — `.1` done: the ledger's owner column is an enforced region, and the checker reads one-line and
  partitioned-tree leaves.
- `2026-09-25` — `.2` done: all 27 live adapters blocked for want of a clock/reset contract; 150 residuals and
  ~20 unread families attributed; `.7`-`.16` opened (8 SpecForge, 2 FSMGen); `.3` re-anchored on the category
  contract; physical/electrical parameters not requested, by decision, and raised to the owner.
- `2026-09-25` — `.8` done: the adapter gave value-less obligations `1` and inverted some stated ones; rules 250 → 95
  on 6 chains, 44 AXI values corrected, 166 obligations listed; `.17` (polarity extraction) and `.18` (duplicate
  invariant ids) opened.
- `2026-09-25` — `.7.a` measured, `.7.b` done: AXI and AHB carry a clock/reset contract and emit strict-clean `.isf`;
  `.7.c`-`.7.e` (Wishbone, APB-E's unstated kind, reset-less documents), `.21` (FSMGen drops a stated reset polarity),
  `.22` (rules drop or invent conditions) and `.23` (guard addendum) opened.
- `2026-09-25` — `.21` and `.23`: both defects found fixed in FSMGen's development line (`f16130a4f`, `c6b8bc783`,
  unpublished) before a report
  went out; `.23` done, `.21` waits for `FSMGEN-REFRESH-INTEGRATE-10`, opened for it.
- `2026-09-25` — the owner forwarded `.4`, `.5`, `.6`, `.15`, `.16`, `.20` and question `.19` to FSMGen.
- `2026-09-25` — `.22` done: no emitted rule drops or invents a condition; `.24` (the `PSELx` family) and `.25` (a
  fused contract's mixed conditions) opened.
- `2026-09-25` — `.25.a`/`.25.b` done; `.22`'s claim corrected (4 temporal-path rules had still dropped a condition).
- `2026-09-25` — `.24.a` done (decision: no FSMGen request; `HSEL` unread); `.24.b`, `.26`, `.27`, `.28` opened.
- `2026-09-25` — `.28` done: the citation registry allows 128 records; the coherence claim now checks its rollover sentence.
- `2026-09-25` — `.26` done: 5 partly read temporal conditions read whole; exceptions typed and lowered negated; `.28`'s authority retired.
- `2026-09-25` — `.27.a` measured: 183 ungrounded ports in six classes; `.27.b`-`.27.e` opened.
- `2026-09-25` — `.27.b` done: continuation pages' `Source` cells read (AXI +43 relations); `.27.d` corrected to a code fix; `.26`'s fmt/clippy miss fixed.
- `2026-09-25` — `.27.d` done: AXI's 122 check signals take their covered signals' relations; 40 become Manager inputs in the emitted `.isf`.
- `2026-09-25` — `.27.c` done (goal corrected: tie-off inputs are the Manager's inputs); 5 `Manager` reads; 2 more emitted ports become inputs.
- `2026-09-25` — `.27.f` opened and done: 8 false prose relations removed; `.27.e` re-measured (AXI 1, AHB 9 ungrounded); `.10` measured (18 AXI rules drive a Manager input).
- `2026-09-25` — `.27.e` done, and with it `.27`: only grounded ports are declared, the rest listed; the ledger's direction row moves to *What already flows*.
- `2026-09-25` — `.10` split: `.10.a` done (18 AXI rules on Manager inputs listed, not driven); `.10.b` open.
- `2026-09-25` — `.10.b` measured: AXI's 21 remaining conflicts in three classes; `.10.c`-`.10.f` opened.
- `2026-09-25` — `.10.f` done: the same level in two spellings is listed as a restatement, not a conflict.
- `2026-09-26` — `.10.c` and `.10.d` done: stated and table conditions kept, AXI's 16 rule conflicts to 0; `.10.g` opened.
- `2026-09-26` — `.10.e` done: 5 AXI tie-offs typed and listed, 9 drives and 3 overlaps gone; `.10.h` opened.
- `2026-09-26` — `.10.h` done: list items take their lead-in's condition; AXI's last `AWLOCK` drives leave.
- `2026-09-26` — `.25.c` done, and with it `.25`: 9 holds typed with their release; `.26.b` opened.
- `2026-09-26` — `.26.b` done: a symbolic value only where the clause states it; no live artifact moves.
- `2026-09-26` — `.7.d` decided and done: APB-E emits `requester.isf`, strict-clean; `.29`-`.31` opened.
- `2026-09-26` — `.31` done: *which means that* keeps its antecedent; a value-less windowed consequent is no eventuality.
- `2026-09-26` — `.30` done (goal corrected): a disagreeing fusion is listed; 3 AXI fusions were lowerable as their first member.
- `2026-09-26` — `.29` decided and done: an unguarded window is listed, not counted from the start of the run.
- `2026-09-26` — `.7.e` decided: no FSMGen request (a clock-only domain exists); `.7.e.i` opened, after `KG-ISF-COMPLETENESS.1`.
- `2026-09-26` — `.7.c` done: Wishbone's contract from prose; it emits `slave.isf`, strict-clean; `.32` opened.
- `2026-09-26` — `.32` done: Wishbone's ports from its role sections; it emits `master.isf` with the MASTER's ports.
- `2026-09-26` — `.17` done: polarity stated for a scope reaches its members (115 records in 5 documents); an event-bounded level mints nothing; `.17.a`, `.33` opened.
- `2026-09-26` — `.33` done: a constraint placed at a window is no invariant (5 false IntentIR facts gone); `.34` opened.
- `2026-09-26` — `.34` done: 130 uncited constraint- and rule-classed statements are candidate misses; the normative finding lists what it counts.
- `2026-09-26` — `.24.b` done, and with it `.24`: co-references and per-actor families are typed records; a driven family is listed.
- `2026-09-26` — `.10.g` measured and decided (`.10.g.ii`, `.10.g.iii` opened); `.10.g.ii` done: *For read transfers* lowers under `PWRITE` LOW.
- `2026-09-26` — `.10.g.iii` measured and blocked on `.4`; `.35` opened and done: a fronted condition keeps its whole list (16 rules, 8 constraints).
- `2026-09-26` — `.11.a` opened and done: 5,089 invariants whose text states no kind are `not_stated`, not a stability.
- `2026-09-26` — frontier re-prioritized to `.3`; first census: register-or-platform has 28 documents, 1 replayable, and holds 83% of the message/structure fields; `.3.a` opened.
- `2026-09-26` — `.36` done: `docs/gaps/` indexes 25 SpecForge items and FSMGen's 28-rung ladder, owners enforced; `.36.a`, `.37` opened.
- `2026-09-26` — `.3.a` done: RISC-V Debug, RISC-V IOMMU, CHI C2C and CoreSight TMC replay on current code (retained 31); `EXTRACTION-GAP-FIX.6`-`.8` and `CORPUS-CHAIN-CURRENCY.12` opened.
- `2026-09-26` — `.3` census recorded (`docs/research/category-family-census.md`); `.3.a`'s categories corrected (C2C is wire-protocol); `.3.b`-`.3.d` and `EXTRACTION-GAP-FIX.9` opened.
- `2026-09-26` — `.3.c`: the 91 encoding tables adjudicated (15 carried but uncounted, 3 withdrawn, 73 mint nothing); `.3.e` opened.
- `2026-09-26` — `.3.c`: why 73 encoding tables mint nothing (66 refused a name by `KG-ISF-COMPLETENESS.5.i`'s gate; 7 left).
- `2026-09-26` — `.3.c`: first read of the other 43 typed tables; `.3.f` and `SIGNAL-DECLARATION-ROW-DROP.6` opened.
