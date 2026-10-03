# What SpecForge still has to do, from PDF to `.isf`

Owner: `EXTRACTION-REPRESENTATION-GAPS.36`. Compiled on `2026-09-26` from the open leaves of every task tree, the
per-category census (`EXTRACTION-REPRESENTATION-GAPS.3`), the category contract
(`doctrine/spec_to_intent_category_contract.json`) and the book's
[ledger](../book/src/reference/representation-gaps.md). Each item names what is missing, the evidence, and the leaf
that owns it; the detail stays in that leaf. Process and repository-maintenance work (document containment, commit
gates, claim verification) is not listed: it is not a missing piece of the product.

The roads are the document categories of the [category page](../book/src/document-categories.md). The stages are
the pipeline's: **source** (PDF to SourceIR), **evidence** (reading), **semantic**/**intent** (typing and carrying),
**adapter** (lowering to `.isf`).

## Across every road

| # | Missing piece | Stage | Evidence | Owner |
| --- | --- | --- | --- | --- |
| S1 | Most documents of the register and platform roads, and every CPU-ISA one, cannot be replayed on current code | source | of 31 replayable chains, 5 are register-or-platform (RISC-V Debug, IOMMU and TMC made replayable by `.3.a`; OpenCAPI discovery reclassified after `EXTRACTION-GAP-FIX.6`) and none is CPU-ISA; 23 of 28 register-or-platform documents are legacy | `EXTRACTION-REPRESENTATION-GAPS.3` |
| S2 | A per-category census of intent the documents state and no IR carrier holds, and of records captured but not understood | all | measured `2026-09-26` over the 31 replayable chains ([census](../research/category-family-census.md)): 2,793 captured regions no record cites, 167 of 187 assumptions not understood; each class routed to an owner (`.3.b`, `.3.c.iv`-`.3.c.viii`, `.3.d`, `.3.f.ii`, `.3.f.iii` and `.3.g.iii` open) | `EXTRACTION-REPRESENTATION-GAPS.3` |
| S3 | A phrase or a section title is taken for an actor; the initiator is not grounded where no signal table names it | evidence | *SYSCON Module* beside *SYSCON*; CoreSight TMC emits as `table_a_3_atb_slave_interface`, a caption; reset-less documents wait on it | `KG-ISF-COMPLETENESS.1` |
| S4 | An invariant id names one invariant | semantic | 6 ids occur twice in 4 chains | `EXTRACTION-REPRESENTATION-GAPS.18` |

## Wire-level bus and interconnect protocols

| # | Missing piece | Stage | Evidence | Owner |
| --- | --- | --- | --- | --- |
| S5 | A clock and reset contract for every document that states one, and a decided lowering for those with none | evidence, adapter | 5 of 31 chains emit `.isf` | `EXTRACTION-REPRESENTATION-GAPS.7`, `EXTRACTION-REPRESENTATION-GAPS.7.e.i` |
| S6 | A value word (`VALID`, an enum member) lowers to the value it names | adapter | value words are listed, not driven | `EXTRACTION-REPRESENTATION-GAPS.9` |
| S7 | A normative constraint keeps its condition and lowers as what it is | evidence, adapter | conditions not `SIGNAL is LEVEL` are listed | `EXTRACTION-REPRESENTATION-GAPS.10` |
| S8 | A transaction named by an opcode table is a guard (`AWSNOOP` with `AWDOMAIN`) | evidence | 17 of AXI's listed conditions name a transaction or request type; 6 also need presence (F7), and the 7 *DVM Complete* rows are implications (F12) | `EXTRACTION-REPRESENTATION-GAPS.10.g`, `EXTRACTION-REPRESENTATION-GAPS.10.g.iii` |
| S9 | Stability and same-cycle exclusion lower as assertions | adapter | 10 rules; needs F12 for an actor with no transaction | `EXTRACTION-REPRESENTATION-GAPS.11` |
| S10 | A protocol state machine is typed (states, transitions, guards, initial state) | semantic | 51 protocol records carry no transitions | `EXTRACTION-REPRESENTATION-GAPS.12` |
| S11 | A polarity stated over a table's rows or a named class of signals reaches each member | evidence | Avalon, GIC-400 (legacy chains) | `EXTRACTION-REPRESENTATION-GAPS.17.a` |
| S12 | A window from an event, a minimum duration and an order between events are typed | evidence, semantic | Wishbone `RST_I`, `CYC_O`; needs F15 to lower | `EXTRACTION-REPRESENTATION-GAPS.37` |
| S13 | A recognised template's prose is expanded over the prefixes that instantiate it | evidence | `Ax…` templates | `WIRE-BASED-100.10d` |
| S14 | A relation does not become a declaration in a table-rich document | evidence | junk declarations | `WIRE-BASED-100.10f` |
| S15 | A transaction's body, where the document gives its steps | semantic, intent | most named transactions carry ports only | `KG-ISF-TRANSACTIONS` |
| S16 | Serial frame fields, a prose bus-line grammar, single-word state names | evidence | CAN, SMBus | `PDF-VARIANT-DIGESTION.9.3b`, `PDF-VARIANT-DIGESTION.9.10`, `PDF-VARIANT-DIGESTION.9.7` |
| S25 | A parity check signal lowers from the signals it covers and its enable | adapter | 153 `check_signal_records` in 3 documents (AXI 123, AHB5 16, APB5 14) since `2026-10-03`; `.isf` has `(^ A B)` but no parity form, and the adapter emits none | `EXTRACTION-REPRESENTATION-GAPS.46` |

## Programmable register and memory-mapped IP

| # | Missing piece | Stage | Evidence | Owner |
| --- | --- | --- | --- | --- |
| S17 | Every register field and reset value is accounted for, and a dropped field is listed | adapter | the field drop is silent today | `EXTRACTION-REPRESENTATION-GAPS.13`, `DOC-INTENT-TAXONOMY.4a.i` |
| S18 | Message, flit and in-memory structure fields reach IntentIR (descriptors, queues, page tables) | intent | 1,527 EvidenceIR field records, no IntentIR carrier; then F23 | `DOC-INTENT-TAXONOMY.4b` |
| S19 | A width written as a parameter lowers to it; a class's property table and a negative recommendation are read | evidence, adapter | parameter tables | `KG-ISF-COMPLETENESS.2a.vii`, `KG-ISF-COMPLETENESS.2a.vii.c.iv`, `KG-ISF-COMPLETENESS.2a.vii.c.v` |
| S20 | A bit range is a width | evidence | MMU-700's observation interface | `SIGNAL-DECLARATION-ROW-DROP.2f` |
| S21 | Enum surface fidelity | adapter | junk and conflated enums | `KG-ISF-COMPLETENESS.5` |

## Platform and system-IP topology

| # | Missing piece | Stage | Evidence | Owner |
| --- | --- | --- | --- | --- |
| S22 | Topology (components, connectivity, clock and reset trees) lowers to composition | evidence, adapter | 0.355 connectivity edges per component; single-actor `.isf`; needs F25 | `EXTRACTION-REPRESENTATION-GAPS.14`, `DOC-INTENT-TAXONOMY.4c.ii` |

## CPU instruction-set and privileged architecture

| # | Missing piece | Stage | Evidence | Owner |
| --- | --- | --- | --- | --- |
| S23 | CSR bit positions drawn in a register's layout figure | source, evidence | 53 of 56 RISC-V Debug register layouts are images; decided as bounded by the local vision model | `DOC-INTENT-TAXONOMY.4` |

## Intent carried only by figures and page layout

| # | Missing piece | Stage | Evidence | Owner |
| --- | --- | --- | --- | --- |
| S24 | Identifiers whose only spelling is inside a stored figure | source | TileLink | `TEXT-LAYER-IDENTIFIER-SPLIT.1` |
| S25 | Grounded whole-page capture of what the layout states | source | gated on a measured go | `FULL-PAGE-INTENT-CAPTURE.2` |
