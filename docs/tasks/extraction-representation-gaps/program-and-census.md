# EXTRACTION-REPRESENTATION-GAPS — program and census

- Part ID: `program-and-census`
- State: `active`

<!-- extraction-representation-gaps-task-source-region:program-and-census:start -->
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

<!-- extraction-representation-gaps-task-source-region:program-and-census:end -->

<!-- extraction-representation-gaps-task-source-region:isf-census-and-decisions:start -->
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

<!-- extraction-representation-gaps-task-source-region:isf-census-and-decisions:end -->

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c`
  Status: `open` (`2026-09-26`; MEASUREMENT; continued after the partition)
  Goal: unchanged (see the sealed record above). **`2026-09-26`, after `.38`:** of the 24 tables left, 10 hold every
  row in their header rows (`.3.g`) and 4 are encodings SourceIR typed `signal_description` (`.3.h`). **Left here: 10**
  — the 3 CHI C2C encodings minted and withdrawn later in the build (`Opcode2`, `Message credit`, `ActTrigger`), the
  USB 3.2 compliance pattern whose rows are all refused, and 6 tables still unread (AXI credited-channel signals and
  legal MMU combinations, ADIv6 programmers'-model mapping, USB hub port timers and power states, OpenCAPI DVSEC
  applicability). **After `.3.h`** (`2026-09-26`): 3 of its 4 tables return here — AHB `HPROT` (one column per
  bit: flags, not one enum), ADIv6 `DbgSwEnable` (*Value of DbgSwEnable* heads its code column) and USB 3.2's
  port-status type codes (no signal named) — so **13 are left**.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g`
  Status: `open` (`2026-09-26`; MEASUREMENT then CODE; found by `.3.c`)
  Goal: **a table whose every row is marked header keeps its rows.** 133 tables in the 31 replayable chains carry
  their whole content as header rows (3 or more header rows, no body row: 1,884 rows): Cortex-A76 guide 37, AXI
  26, USB 3.2 23, ADIv6 10, IOMMU 7, CoreSight BSA 6, others. 121 stay untyped, so this also feeds `.3.b`. Among the
  typed ones, AXI's presence summaries (B2.2, B2.3 and their continuations), *Signals in each version* (A13.3) and
  *Physical address space encodings* (A4.7), and USB 3.2's transmitter eye, LFPS timing and port-type tables reach
  no record. `recovered_trapped_data_rows` lifts only rows whose other cells are not headers, so these stay trapped.
  Acceptance: measure where the marking comes from (the converter's cell flags or SourceIR's reading of them),
  decide structurally which trailing header rows are data, recover them where the table's own shape says so, and
  measure the recovered records and every current chain's delta.
  **Measured `2026-09-28` (the MEASUREMENT half; the CODE half waits on `.3.g.i` and `.3.g.ii`).** The marking is the
  converter's: Docling flags a row-header column's cells as headers, and SourceIR files any row with one flagged cell
  among `header_rows`. Of the rows after each table's first header row, 1,589 flag only their first cell, 97 their
  first two, 11 every cell. The structural rule is the one `recovered_trapped_data_rows` already applies for four
  readers: in a table with no body row, a later row whose only flagged cell is the first is data. Lifted at the
  evidence stage's entry (`build_unproved_from_source_ir`, so every reader and the proof replay see it; patch kept at
  `.project-data/tmp/erg3g/row-header-lift.patch`), it moves 1,593 rows of 131 tables in 16 current chains, and
  `check_chain_currency.sh` finds only 2 chains changed: RISC-V Debug (`dmstatus` gains its continuation table) and
  AXI IHI0022 L. On AXI it removes three junk semantic hints and one junk conflict (presence-table rows read as prose:
  `Y` and `AxSNOOP` "valid-like"), and adds four `Enum AxPROT …` statements and a polarity conflict on `AxMMUATST`.
  Both additions are reader defects the lift exposes, not rows it misreads, and AXI is a wire gold, so the lift waits:
  `.3.g.i` and `.3.g.ii`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.i`
  Status: `done` (`2026-09-28`; CODE; found by `.3.g`; checklist below)
  Goal: **`X is deasserted LOW` does not make X active-low.** AXI IHI0022 L states *When AxMMUATST is deasserted LOW,
  the Stall flow is used* and *When AxMMUATST is asserted HIGH, the ATST flow is used*; once `.3.g` makes `AxMMUATST`
  a known signal, the prose polarity pass reads the first as `active_low` and the second as `active_high` and records
  a conflict. Deasserting a signal to its low level is what an active-high signal does. **Reproduce:** apply `.3.g`'s
  patch and dry-run AXI's evidence (`signal_polarity_conflicts` 0 -> 1). Acceptance: read the level a sentence states
  together with its verb (asserted, deasserted) so the pair above agrees on `active_high`; census every chain's
  polarity observations for the same shape; no wire gold's polarity changes except toward the stated one.
  **Done.** The cause is a substring: `detect_signal_polarity` matched `asserted low` inside `deasserted low`.
  Its phrases now must start a word (`contains_phrase_at_word_start`; the end stays open, because the text layer
  fuses `active LOWand` and `Active LOWJTAG`), and `deasserted high`/`deasserted low` state active-low/active-high.
  Over every statement of the 31 current chains exactly one detection changes, the AxMMUATST sentence; bounding both
  ends would also have lost AHB's `active LOWand` and ADIv6's `Active LOWJTAG`, measured and refused. With `.3.g`'s
  patch applied on top, AXI has no polarity conflict and `AxMMUATST` is `active_high`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.g.i — a signal deasserted LOW is active-high, and asserted is not read inside deasserted`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.ii`
  Status: `open` (`2026-09-28`; CODE; found by `.3.g`)
  Goal: **an encoding keyed by two signals is not minted as one signal's enum.** AXI's *Physical address space
  encodings* (A4.7) keys each space on `AxNSE` and `AxPROT[1]` together; once `.3.g` gives the table a body, the
  encoding path mints `AxPROT` members `SECURE = 0`, `ROOT = 0`, `NON_SECURE__NS = 1`, `REALM = 1`, two names per value
  and neither true of `AxPROT` alone. **Reproduce:** apply `.3.g`'s patch and dry-run AXI's evidence (four `Enum AxPROT`
  statements). Acceptance: decide from the table's shape when an encoding is keyed by more than one column, and then
  mint no single-signal enum (or a keyed one where the IR can carry it); census every chain's encoding tables with two
  value columns.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.h`
  Status: `done` (`2026-09-26`; CODE; found by `.3.c`; checklist below)
  Goal: **a signal's encoding table is read as an encoding.** SourceIR types some encoding tables
  `signal_description`, and neither enum path reads that kind: the encoding-table path runs on `encoding` only and
  the signal-anchor scan skips `signal_description`. Measured: AHB *Table 3-4 Burst signal encoding*
  (`HBURST[2:0] | Type | Description`: `SINGLE`, `INCR`, `WRAP4`, …) and *Table 3-5 Protection signal encoding*
  (`HPROT`), ADIv6 *Table C2-5 DbgSwEnable*, USB 3.2 *Table 10-12 Port Status Type Codes*. AHB carries `HTRANS` and
  `HRESP` enums and no `HBURST` or `HPROT`. Acceptance: decide from the table's shape (a code column keyed by one
  named signal) whether a `signal_description` table is an encoding, read it as one, and measure AHB's symbols, the
  wire golds and every current chain's delta.
  **Done.** `signal_table_encodes_its_first_header_signal` (`ir/evidence.rs`) admits a `signal_description` table to
  the signal-anchor encoding scan when its first header is the anchor signal (a bit range may follow), most rows open
  with an encoding literal, and no other column is one of literals. Measured before writing it over all 78 stored
  documents: two tables have that shape, AHB's Table 3-4 and a permitted-combinations table in AXI H.c (legacy) that
  the third condition refuses. `check_chain_currency.sh` with the new binary moves AHB alone (+8 statements, the eight
  `HBURST` members; 30 chains current at every stage). Rebuilt with each stage validated, AHB's SemanticIR and
  IntentIR differ, id-free, only in `symbol_definitions` (9 -> 10), and `manager.isf` gains
  `(HBURST (SINGLE 0) (INCR 1) (WRAP4 2) (INCR4 3) (WRAP8 4) (INCR8 5) (WRAP16 6) (INCR16 7))`, strict-clean. The port
  stays `(width 1)`: its width is the parameter `HBURST_WIDTH`, which may be 0 (`.4`, `KG-ISF-COMPLETENESS.2a.vii`).
  The other three tables stay with `.3.c`: `HPROT`'s gives each bit its own column (flags, not one enum),
  `DbgSwEnable`'s first header is *Value of DbgSwEnable*, and USB's port-status type codes name no signal.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.h — AHB's burst encoding is read: HBURST reaches the .isf`

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.h` (CODE) — DONE `2026-09-26`

- [x] **REPRODUCE / MEASURE** — AHB IntentIR `symbol_definitions`: `HTRANS`, `HRESP` and parameters, no `HBURST`;
  `table_0014` (`HBURST[2:0] | Type | Description`, typed `signal_description`) in `captured_region_residuals`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `scan_encoding_tables_by_signal_anchor`
  skipped every `TableKind::SignalDescription` table, and the encoding-table path runs only on `TableKind::Encoding`.
- [x] **ADDRESSED (verified)** — AHB rebuilt: +8 `Enum HBURST …` statements, `symbol_definitions` 9 -> 10, the `.isf`
  gains the `HBURST` type and members; FSMGen `--strict --check --json` success, 0 diagnostics.
- [x] **NO REGRESSION** — `check_chain_currency.sh` (new binary): AHB alone moves; 30 chains current at every stage.
  AHB's SemanticIR/IntentIR differ id-free only in `symbol_definitions`. WIRE-BASED-100: all 11 seeds score
  identically with the HEAD binary before and the new binary after (temporary paths masked). `kg-bench` 158/158.
  `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test
  --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,747 passed (+1). `flow_census.json` +1
  function, +8 decision sites, +2 helper edges, +1 macro.
- [x] **GENERICITY (ADR 0006)** — table structure only: the first header names the anchor signal, a literal-majority
  first column, no other literal column. No document, signal or vendor name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` gains *An encoding filed as a signal table*, with AHB's table as the
  example and the two cases it refuses.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.g.i` (CODE) — DONE `2026-09-28`

- [x] **REPRODUCE / MEASURE** — with `.3.g`'s patch applied, AXI IHI0022 L's `signal_polarity_conflicts` 0 -> 1:
  `AxMMUATST` read `active_low` from *When AxMMUATST is deasserted LOW, the Stall flow is used* (`statement_3726`) and
  `active_high` from *… asserted HIGH …* (`statement_3727`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `detect_signal_polarity` tested
  `text.contains("asserted low")`, which is true inside `deasserted low`. RED, each alone with the file restored
  byte-identically: substring matching, dropping the deasserted-level phrases, and bounding both phrase ends each fail
  `a_deasserted_level_states_the_opposite_polarity`.
- [x] **ADDRESSED (verified)** — with `.3.g`'s patch on top of this change, AXI dry-run: no polarity conflict,
  `AxMMUATST` `active_high`. Mirror of old against new detection over every statement of the 31 current chains:
  exactly that one sentence changes.
- [x] **NO REGRESSION** — `check_chain_currency.sh` with the new binary: 31 of 31 current at every stage (nothing
  persisted moves; `AxMMUATST` is not yet a known signal without `.3.g`). WIRE-BASED-100: all 11 seeds identical with
  the HEAD binary and the new one. `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 /
  1,761 passed, 0 failed. `flow_census.json` rolled forward under this leaf (+1 function,
  -14 decision sites: two chains of substring tests became two phrase lists, +1 helper edge; boundary unchanged).
- [x] **GENERICITY (ADR 0006)** — the universal assert/deassert level grammar; no signal or document name.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md`'s polarity rules gain the deasserted level and the word-start
  matching, with the AxMMUATST pair as the example.

