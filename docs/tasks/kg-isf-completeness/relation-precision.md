# KG-ISF-COMPLETENESS — relation precision

- Part ID: `relation-precision`
- State: `active`

## Post-migration record

- ID: `KG-ISF-COMPLETENESS.1d` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by
  `.5.xiii.a.5.a`) · Goal: **a phrase is not an actor.** The actor-identity gate (`.1a`) admits prose
  fragments as relation subjects: Wishbone B4, once its names became matchable, publishes 9 relations
  whose actors are `interfaces so`, `similar manner`, `same operation`, `bus cycle`, `DAT_I signal`,
  `cases SLAVE`; ADIv6 publishes `details reads TDO` from *"For details about the Read TDO (RTDO) bit,
  see …"*. This is the class `.1c.ii` measured on eMMC (109 multi-word actors) and deferred as having no
  clean within-document structural gate — **its trigger is met**: it now reaches a rebuildable document.
  Measure corpus-wide which actor strings are neither a declared actor, a role noun the document uses as
  a subject elsewhere, nor a component the document names, and read a sample before choosing a gate.
  Verification: census over all 78 chains; replay over the 27 with a per-relation diff; WIRE-BASED-100
  with both binaries; the Wishbone 41 re-read.
  **MEASURED `2026-09-23` — the grounding test the leaf proposed is refuted, and the class has causes, not a
  shape.** The persisted artifacts cannot answer it: 51 of 78 chains were written by older binaries, and their
  actors include `of ordered` and `set of`, which `.1a` already refuses. So the current extractor was replayed
  over every chain's persisted statements (a throwaway `#[ignore]` probe, removed byte-exactly): **1,342
  relations, 794 document-actors, 409 of them two-word** (no current actor is longer). Grounding does not
  separate them: a declared actor matches **3** of 1,053 persisted actors (22 of 78 documents declare any), and
  mention counts point the wrong way — junk `bus cycle` occurs 75 times and in headings, real `watchdog timer`
  nowhere else. A seeded sample of 60 two-word actors read by hand: **about 10 real agents** (`Avalon-MM
  master`, `snooped Manager`, `MASTER interface`, `I/O device`), **about 9 real agents that swallowed their own
  verb** (`HBM responds`, `Manager sees`, `Completer guarantees`), **about 40 not agents at all**. Reading how
  each was minted gave the causes: (1) a relation verb used as a NOUN — the vocabulary holds `read`, `access`,
  `monitor`, … and *"the read pointer"* licenses a relation whose subject is whatever phrase precedes it:
  **124 relations, owned and landed as `.1d.i`**; (2) the subject cut takes two words after the nearest
  determiner even when the second is a finite verb — **about 20 actors, `.1d.ii`**; (3) clauses with no agent
  at all — a copula (*"MUST be the same operation"*), an imperative (*"Design … interfaces so that"*), a
  leading prepositional phrase fused onto the subject (*"In these cases SLAVE circuitry"*) — and a signal
  standing as actor (`DAT_I signal`), which stay here. After `.1d.i`: 377 two-word document-actors, 545
  relations.
  Commit: measurement recorded with `KG-ISF-COMPLETENESS.1d.i`.

- ID: `KG-ISF-COMPLETENESS.1d.i` · Status: `done` (`2026-09-23`; CODE; opened by `.1d`'s measurement) ·
  Goal: **a verb after an article is a noun.** A relation is licensed by any occurrence of a vocabulary verb
  whose object holds the signal, and the vocabulary carries base forms (`read`, `access`, `monitor`, …) that
  English uses as nouns and modifiers: *"the read pointer"*, *"a read transaction"*, *"the access requests"*.
  Acceptance: an occurrence directly preceded by an article or determiner no longer licenses a relation;
  capital `A` stays a possible label (*"Master A asserts"*); every removed relation is read; per-relation replay
  over the 27 and the persisted-statement replay over all 78 with both binaries; WIRE-BASED-100 with both.
  **LANDED.** `is_nominal_verb_occurrence` refuses an occurrence whose preceding word is `the`, `an`, lowercase
  `a`, `this`, `these`, `those`, `each`, `every`, `any`, `no` or `another`, at all four active licensing
  sites. `that` stays out (the relative pronoun of *"the Manager that drives HTRANS"*), as do `its` (the GIC
  `ITS` agent, `.1a`: *"the ITS forwards any operations"*) and `all`/`both`/`some`, which can precede a verb.
  Capital `A` stays a label because the measurement found one: *"1. Master A asserts address"* keeps `Master
  drives ADDRESS`, and a case-blind `a` fails that guard.
  **Reach, persisted statements of all 78 chains:** 1,342 -> 1,218 relations, **124 removed across 16
  documents, 0 added**, and every removal read by hand (`read` 84, `access` 27, `writes` 11, `accesses` 1, `activates` 1):
  the relation verb was a noun in each. **Rebuild of the 27 replayable chains with both binaries:** **25
  byte-identical**; AHB 75 -> 68 and ADIv6 109 -> 99, only `actor_signal_relations` and the surfaces derived
  from it moving. The 17: 15 wrong — among them ADIv6 `details reads TDO` (named in `.1d`), eight `… reads In`
  from *"a read or write packet request"*, AHB `Subordinate reads Manager` from *"the read data bus moves
  data from a Subordinate to the Manager"* — and **2 true but never evidenced**: AHB `Subordinate reads
  HREADY` was read out of *"drives the read data bus"*, and `Subordinate reads HREADYCHK` inherits from it (a
  check signal takes its protected signal's relation). HREADY is a Subordinate input in AHB5, so that is a
  recall residual with a wrong source, owned as `.1d.i.a`, not a fact to keep on a misread sentence.
  **WIRE-BASED-100 unmoved:** no removed relation's statement carries any gold item, and all seven seeds
  (`ahb`, `apb`, `axi`, `swd`, three temporal) score identically with each binary on its own build — only
  temp paths and the AHB/ADIv6 relation counts differ in the full output.
  **The two moved chains are rebuilt in this commit**, because the proof binds the derivation topology
  (`[[producer-change-and-corpus-rebuild-are-one-transaction]]`): the first commit attempt was refused by
  `PROOF-SEAL-CURRENCY` for exactly AHB and ADIv6. Rebuilt evidence -> semantic -> validate -> intent -> validate
  -> adapt, then rebuilt again with the HEAD binary for comparison: `adapter.json` differs only in
  `proof_ledger` hashes — the lowered `isf`, `lowering_status` and `residual_decisions` are identical.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1d.i — a verb after an article is a noun`

- ID: `KG-ISF-COMPLETENESS.1d.i.a` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by `.1d.i`) ·
  Goal: **recover AHB `Subordinate reads HREADY` from a sentence or table that states it.** `.1d.i` removed it
  because its only support was *"the Subordinate drives the read data bus"*; HREADY is a Subordinate input in
  AHB5, and `Subordinate reads HREADYCHK` follows from it. Measure where the specification states the
  direction (a port table, a signal description) and why no relation path reads it today.
  **MEASURED `2026-09-23` — the fact is stated in a table that ingest ROTATED.** AHB Table 2-5 *Multiplexor
  signals* gives HREADY's destination as *"Manager and Subordinate"*, but the normalized page text carries
  every row shifted by one column: under `Name | Destination | Width | Description` the row reads `Manager
  and Subordinate | 1 | When HIGH, the HREADY signal … | HREADY`. No name sits in the Name column, so the table
  never became a structured signal table (SourceIR has AHB tables 2-3 and 2-4, not 2-5), and the
  Destination-column relation path that would read it never runs. Prose does not state it as a relation:
  *"it must also monitor the status of HREADY"* is a table cell, and *"sample … when HREADY is HIGH"* puts
  HREADY in a condition. **Corpus census** (every normalized document; a `Name`/`Signal` first column with
  no identifiers while another column is mostly identifiers): **5 tables in 3 documents, the first three or four
  rows of each read, all the same defect — the first column's value moved to the LAST column and every other cell shifted left**:
  AHB Table 2-5 and the Global signals table (`Clock source | 1 | … | HCLK`), AHB's check-signal property
  table (`1 | - | - | C | C | HTRANSCHK`), APB's check-signal table (`… | N | PADDRCHK`), and ADIv6's
  power-control table (`Bit[28] of the CTRL/STAT register | CDBGPWRUPREQ`). AHB and APB are gold-scored.
  The repair is table normalization, not relation extraction: `.1d.i.a.i`.
  **CORRECTED `2026-09-24` (`.1d.i.a.ii.a`):** Table 2-5 DID become a structured signal table (`table_0009`,
  `#/tables/8`, `signal_description`), and EvidenceIR declares its four signals from it by content (`Signal
  HREADY is input width 1`). What the rotation breaks is the column read: under `Destination` HREADY's row holds
  `1` (its width), so the table cannot supply the destination. Which relation path reads it once repaired is
  `.ii.c`'s to measure. The repair is still table normalization.

- ID: `KG-ISF-COMPLETENESS.1d.i.a.i` · Status: `done` (`2026-09-24`; CODE; opened by `.1d.i.a`) · Goal: **a
  table cell Docling indexed into the wrong column is re-homed to the column its geometry puts it in.** Locate the ingest
  stage that rotates (Docling output vs SpecForge normalization) before choosing where to repair; the repair
  must be structural (a name column with no identifiers whose values reappear, row by row, as identifiers in
  the last column), never a per-document rule. Verification: the 5 tables re-read; a census over all 78
  that no unrotated table matches; AHB and ADIv6 chains rebuilt both ways; WIRE-BASED-100 both binaries
  (AHB and APB are seeds); `Subordinate reads HREADY` recovered from Table 2-5.
  **STAGE LOCATED `2026-09-23`: Docling's cell COLUMN INDICES are wrong; its GEOMETRY is right.** In the
  Docling output (`normalized/*.backend.json`) AHB Table 2-5's `HREADY` cell spans x=116-155, directly under
  the `Name` header (x=116-140), yet carries `start_col_offset_idx` 3; every body cell is indexed one column
  off in the same cyclic way, and our normalization reproduces the indices faithfully. So the repair re-derives
  a cell's column from the header cell it sits under, and only where index and geometry disagree. **Census not
  yet calibrated:** a crude left-edge assignment over all 477 Docling tables flags 68, but only AHB/APB
  disagree on every body cell; many OpenCAPI/USB4 hits disagree on exactly half (3/6, 4/8), the signature of
  spanned or grouped headers the crude rule misreads. Calibrate the rule (a whole-row cyclic shift) and read a
  sample before counting — 68 is NOT a finding.
  **CALIBRATED `2026-09-24`.** Each body cell assigned to the header cell its bbox overlaps MOST, over every
  Docling table: 201 of 477 have no single full header row (not assessable by this rule), **266 consistent,
  7 whole-table shifts by one, 3 partial**; the crude 68 was the instrument. Reading the non-consistent 10:
  **6 are true shifts** (AHB tables 3/8/32/33/34 — Table 2-5 is 8 — and APB table 16); APB table 15 is the
  same shift with MERGED cells (`'Y Y'`) and APB table 17 merges across columns (`'DATA_WIDTH/8 Check_Type'`,
  text geometry cannot split); **2 are false**: USB4 table 40 is a bit-field diagram whose "header" is `PDF | 0
  | Hop ID`, and Wishbone table 0 is a table of contents whose two header boxes OVERLAP (x 56-214, 123-538).
  ADIv6's rotated table does not enter the rule (no full single header row) and stays a named residual.
  **Repair spec (from these readings):** at `extract_table_grid` (the embedded ingest script in
  `crates/specforge/src/ir/source/docling_backend.rs`, which today drops the bbox), re-home a body cell to the
  header column its bbox dominantly overlaps, PER CELL (a whole-row rotation is wrong for APB 15's merged
  cells), only when the header cells are horizontally DISJOINT (excludes Wishbone's ToC) and the table's
  disagreeing cells share one consistent non-zero shift (excludes the USB4 diagram); merged text is never
  split. **Cost:** there is no reload path from the persisted `*.backend.json`, so the affected documents
  (AHB, APB) need a Docling re-run, then chain rebuilds and WIRE-BASED-100 both ways (both are seeds).
  **DONE `2026-09-24` — the ingest repair; the corpus refresh moves to `.1d.i.a.ii`.** `geometric_column_homes`
  + a re-homing `extract_table_grid` in the embedded helper, per the spec above plus one rule the full population
  forced (below). **The calibration's population was a third of the corpus:** it read top-level `tables`, and the
  three page-batched bundles (AXI, ADIv6, USB 3.2) keep theirs under `documents[]` — **1,256 tables, not 477**
  (fact card `docling-tables-live-under-documents-in-a-batched-bundle`). Replayed over all 1,256 by loading every
  bundle part through `DoclingDocument.model_validate` and running HEAD's and the new `extract_table_grid` on the
  same `TableItem`s: the spec as written changed **9** — the 8 expected (AHB 3/8/32/33/34, APB 15/16/17) and AXI
  table 93, a burst-address table whose only 2 disagreeing cells are merged `S S` pairs spanning headers `4` and
  `3`, where dominant overlap merely moves the merge from one to the other. **Added rule: a shift must be
  WITNESSED by a disagreeing cell under exactly one header**; merged text may follow a witnessed shift but not
  establish one. With it: **8 changed, 1,248 byte-identical**. Read in full: AHB 8 now `HREADY | Manager and
  Subordinate | 1 | …`; APB 17's `PSLVERRCHK`/`PAUSERCHK` return to `Signal` (merged `PSTRBCHK 1` stays merged,
  `.1d.i.a.iii`); ADIv6's rotated table is untouched (no single full header row, `.1d.i.a.iii`).
  **Why the corpus leg moved.** The script is hashed into no artifact, so this commit stales nothing; but a
  Docling re-run of AHB/APB would carry the measured re-ingest drift (figure text, ordinal ids —
  `pipeline/sourceir.md`) into two gold chains alongside the table fix, and WIRE-BASED-100 could not attribute
  a move to either. The replay `.1d.i.a.ii` builds changes only the 8 tables, so the rebuild, WIRE-BASED-100 both
  ways and the `Subordinate reads HREADY` recovery go there. Scratch probes lived under
  `.project-data/tmp/kg-1diai/` and were removed after the census.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1d.i.a.i — a table cell lands in the column it is drawn under`

- ID: `KG-ISF-COMPLETENESS.1d.i.a.ii` · Status: `open` (`2026-09-24`; CODE; opened by `.1d.i.a.i`) · Goal:
  **a table repair reaches the corpus without re-running Docling's models.** Tables are built once, inside the
  Docling run, from its column-indexed grid; the persisted `normalized/*.backend.json` keeps every cell's bbox,
  but no path re-derives `structured_tables` from it, so any ingest-time table fix costs a full re-run per
  document. Build that replay (load the persisted Docling document, re-run only table extraction), proved
  equal to the current tables on every document the repair does not touch. Scheduled with `.1d.i.a.i`.
  **Re-scoped `2026-09-24` by `.1d.i.a.i`: this leaf now also CARRIES that repair into the corpus** — replay the
  AHB and APB tables (the only 8 that change), rebuild both chains stage by stage through the adapter,
  WIRE-BASED-100 with each binary on its own build (both are seeds), and recover `Subordinate reads HREADY`
  from AHB Table 2-5. The load path is proven: every one of the 45 bundle parts validates as a
  `DoclingDocument` and yields the same rows HEAD's ingest built for 1,248 of 1,256 tables.
  **Split `2026-09-24`** into `.ii.a` (the premise measurement), `.ii.b` (the recapture path) and `.ii.c` (the
  corpus refresh), because the persisted table rows are a proof PREMISE, not a derivation (below).

- ID: `KG-ISF-COMPLETENESS.1d.i.a.ii.a` · Status: `done` (`2026-09-24`; MEASUREMENT; opened by `.1d.i.a.ii`) ·
  Goal: **is a persisted table premise a deterministic function of the retained converter document?**
  `source_proof_migrate` cannot carry a table repair: for a current-schema SourceIR it re-executes the retained
  `proof_context` and never recaptures, and `proof_context.field_premises.structured_tables` holds the helper's
  `header_rows`/`body_rows` themselves. A repair therefore needs a RECAPTURE of that premise, which is sound only
  if the premise is exactly what the helper derives from the retained `backend.json`.
  **MEASURED: yes, for every table.** For each of the 27 SourceIR artifacts with a retained bundle, every
  premise table record was located in its Docling document (by `self_ref` within `source_batch`, an integer batch
  index; for the two pre-coordinate batched artifacts, ADIv6 and USB 3.2, by position, accepted only when the
  premise count equals the concatenated batch tables AND every position's `self_ref` agrees) and re-derived with
  the pre-repair helper (`a2f42cbc`): **1,256 of 1,256 premise rows equal**, zero unlocated. With the current
  helper exactly the 8 `.1d.i.a.i` tables differ. **CORRECTED `2026-09-24`:** this record first said the 8 are
  persisted as `table_kind: unknown` and that this is why Table 2-5 never became a signal table. That read the
  PREMISE, and the premise is built from a neutralised copy (`SourceIr` sets every `table_kind` to `Unknown`
  before taking premises), so `unknown` there is true of every table by construction. The classified
  `structured_tables` field has all 8 as `signal_description`. What the rotation actually breaks is downstream
  of classification, with no name under `Name`, and `.ii.c` must measure it rather than assume it. The row
  equality above compared rows only and stands. 51 SourceIR artifacts retain no bundle and are out of reach
  of any replay; AHB and APB are not among them.
  Verification: the census above (scratch under `.project-data/tmp/kg-1diaii/`, removed).
  Commit: `KG-ISF-COMPLETENESS.1d.i.a.ii.a — a persisted table premise is the helper's output, exactly`

- ID: `KG-ISF-COMPLETENESS.1d.i.a.ii.b` · Status: `done` (`2026-09-24`; CODE; opened by `.1d.i.a.ii`) · Goal:
  **recapture a SourceIR's table premise from its retained converter document.** A helper mode that loads a
  retained `backend.json` (each `documents[]` part of a batched bundle) and emits every table's
  `header_rows`/`body_rows`/`table_kind`, located as `.ii.a` located them; a feature-gated
  (`source-proof-migration`) SourceIR maintenance path that replaces only the tables whose recaptured rows
  differ, refuses any unlocated or ambiguous table, then refreshes the proof from the amended context the way
  `rebuild_from_retained_capture` does. Record the recapture as a decision (a premise re-derived from retained
  model output is a recapture, not a re-ingest). Acceptance: a dry run over all 27 changes exactly the 8 tables
  and leaves the other 25 documents byte-identical; tests pin the locate/refuse rules.
  **DONE `2026-09-24`.** `recapture_retained_tables` (`docling_backend.rs`) appends a recapture entry point to the
  helper's own function prefix and runs it with the ingest Python over the retained `backend.json`, with no model.
  `SourceIr::recapture_tables_from_retained_converter` (`source.rs`) locates every premise table
  (`locate_recaptured_tables`), amends only the differing tables' `header_rows`/`body_rows`/`row_count` in both the
  root and the indexed premise keys, and refreshes through `refresh_from_retained_context`, which is factored out
  of `rebuild_from_retained_capture` unchanged (that path now shares `load_retained_capture` and
  `retained_proof_context`). `examples/source_table_recapture.rs` dry-runs by default. **Dry run over all 27
  retained documents: would-change AHB `table_0004,0009,0033,0034,0035` and APB `table_0016,0017,0018`; the other
  25 unchanged**, including the three batched bundles and the two positionally located ones; both refreshes
  re-proved without error; nothing written. Decision: ADR 0052. **Defect found and owned:** no gate compiles the
  `source-proof-migration` feature, so this path and `source_proof_migrate` can rot with every gate green; that is
  `COMMIT-GATE-SINGLE-RUN.18`.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1d.i.a.ii.b — recapture a table premise from the retained converter document`

- ID: `KG-ISF-COMPLETENESS.1d.i.a.ii.c` · Status: `open` (`2026-09-24`; CODE then MEASUREMENT; opened by
  `.1d.i.a.ii`) · Goal: **carry the repair into the AHB and APB chains and recover `Subordinate reads HREADY`.**
  Apply `.ii.b` to both, rebuild evidence -> validate -> semantic -> validate -> intent -> validate -> adapt,
  score WIRE-BASED-100 with each binary on its own build (both are seeds), and read every relation, constraint
  and `.isf` port that moves.
  **PARTIAL `2026-09-24` — APB carried; AHB measured and HELD on `.ii.d`.** Pre-rebuild copies of both chains
  are under `generated/preserved/KG-ISF-COMPLETENESS.1d.i.a.ii.c/`. `source_table_recapture --write` changed
  exactly the 8 tables: the SourceIR diff is `structured_tables`, the same-index premise keys and the proof, and
  every `table_kind` is unchanged (`signal_description`). Both chains were then rebuilt and validated stage by
  stage (evidence, semantic, intent) and adapted. All 11 WIRE-BASED-100 seeds scored identically before and after
  (same binary, since no code changed); only AHB's relation count moved.
  **AHB EvidenceIR, every moved relation read (68 -> 67):** 11 ADDED, all correct: `Subordinate reads HREADY` (the
  goal), `Subordinate reads HREADYCHK`, `Manager reads HRDATA / HREADY / HRESP / HEXOKAY` and their `CHK` twins
  (Table 2-5's Destination column, now read as destinations), and `Clock source drives HCLK` / `Reset controller
  drives HRESETn` (Table 2-3's Source column). 12 REMOVED, all wrong, and each had **`Manager` as the signal**
  (`Subordinate drives Manager`, `2.5 Multiplexor drives Manager`, `decoder reads Manager`, …). The rotation put
  destination text under `Name`, so `Manager` was minted as a signal name, and prose about a Manager became
  relations. Statements: `+HRESETn … width 1`, `HREADYOUT … output` gains `width 1`. **APB:** relations
  unchanged; `PADDRCHK … width ceil(ADDR_WIDTH/8) a` (a cell borrowed from another row) becomes `ADDR_WIDTH/8`;
  the `.isf` differs only in proof.
  **AHB's `.isf` regressed in one place: the lowered actor is renamed `manager` -> `two_cycle_response`** (body
  byte-identical otherwise). Traced and re-derived: `derive_isf_actor_name` (`adapters.rs`) takes
  `select_initiator_actor` (`isf_ir.rs`), which keeps only NET PRODUCERS (Output ports > Input ports) and breaks
  exact ties by the lexicographically last name. Manager's ports went from (6 out, 0 in, 2 inout) to (6, **7**, 2)
  because the recovered facts are Manager READS relations, so Manager is filtered out and four phrase actors tie at
  (2, 0). The refresh is correct and the rule is wrong: an initiator reads its responses. **So AHB was restored
  byte-exactly** (9 of 9 files `cmp`-identical to the preserved copies; `seed_ahb` re-scored identical to the
  baseline), and its refresh waits on `.ii.d`. APB stays refreshed. Remaining for this leaf: re-apply AHB after
  `.ii.d`, then close.

- ID: `KG-ISF-COMPLETENESS.1d.i.a.ii.d` · Status: `done` (`2026-09-24`; MEASUREMENT; opened by
  `.1d.i.a.ii.c`) · Goal: **the lowered actor is the interface's initiator even when it correctly reads its
  responses.** `select_initiator_actor` (`crates/specforge/src/ir/isf_ir.rs`) requires Output > Input and breaks
  ties by the lexicographically last name. Correct AHB facts (`Manager reads HRDATA/HREADY/HRESP` and their
  `CHK` twins) disqualify the Manager and hand the name to `two_cycle_response`. **Repro:** apply `.ii.c`'s AHB
  refresh (`source_table_recapture --write` on AHB, rebuild) and read `adapters/isf/<ahb>/adapter.json`
  `isf.actor_name`. **Impact:** a gold `.isf` actor is named from a prose phrase, and the interface
  perspective used for port directions comes from that phrase actor. Measure first, over every lowered adapter:
  which actor the rule picks, the port counts behind the pick, and what a candidate rule would pick. Candidates
  include largest Output count without the net-producer filter, and a tie-break that is not alphabetical. Read every
  changed name, then change the rule with no name lists (ADR 0006). Also note that Manager has only 6 Output ports,
  while AHB's Manager drives far more (HADDR, HWRITE, HSIZE, …). That is a drives-recall gap, and `.ii.d`
  should measure it and open a leaf for it rather than fold it in. Then `.ii.c` re-applies AHB.
  **MEASURED `2026-09-24`: the rule is not the cause, and no reordering of it is a fix.** Over the 39 IntentIR
  artifacts with `actor_ports`, three rules were compared: today's (net producer, max `(out, in)`), "most Output
  ports", and "most driven incl. InOut". They disagree on 11. "Most outputs" names AHB's actor **`Subordinate`
  even today** (Subordinate 9 out / 21 in vs Manager 6 / 0), and on the refreshed AHB "out minus in" still picks
  the phrase actor. The counts are skewed upstream. Subordinate's 21 Input ports (HADDR, HWRITE, HSIZE, HBURST,
  HTRANS, HWDATA, …) all come from AHB **Table 2-2 "Manager signals"**, whose `Destination` column names the
  readers. Its caption names the **source**, and no path reads it, so Manager has 6 Output ports where the
  specification gives it about 20. Tables 2-4 "Decoder signals" and 2-5 "Multiplexor signals" have the same shape.
  With Table 2-2's source read, the refreshed Manager would have far more outputs than its 7 inputs, and today's
  rule picks it. **So the fix is extraction: `.ii.e`.** The rule's alphabetical tie-break among phrase actors is
  arbitrary but only decides between wrong answers. It is phrase-actor precision (`.1d`), not a naming defect.
  Verification: the census above (scratch removed); Table 2-2 and the port lists re-read from the persisted AHB
  SourceIR and IntentIR.
  Commit: `KG-ISF-COMPLETENESS.1d.i.a.ii.d — the initiator rule is right; the Manager's drives were never read`

- ID: `KG-ISF-COMPLETENESS.1d.i.a.ii.e` · Status: `open` (`2026-09-24`; CODE; opened by `.1d.i.a.ii.d`) · Goal:
  **a signal table whose caption names its owner and whose rows name their destinations yields `<owner> drives
  <signal>`.** AHB Table 2-2 "Manager signals" (`Name | Destination | Width | Description`) produces only
  `Subordinate/decoder reads …`, never `Manager drives …`. Rule shape, to be measured before it is fixed: the
  caption is `<actor> signals` for an actor the document already names (no name list, ADR 0006), and a
  `Destination` column marks the rows as the owner's outputs. A `Source` column marks the reverse. Measure first,
  over every signal table in the corpus: which captions name an owner, which owners are known actors, and every
  relation the rule would add. Read them all. The rule is a producer change inside the evidence derivation, so the
  corpus rebuild ships in the same commit (`[[producer-change-and-corpus-rebuild-are-one-transaction]]`). Then
  `.ii.c` re-applies AHB and checks the lowered actor is `manager` again.

- ID: `KG-ISF-COMPLETENESS.1d.i.a.iii` · Status: `open` (`2026-09-24`; MEASUREMENT then CODE; opened by
  `.1d.i.a.i`) · Goal: **the column shifts the header-geometry rule cannot see are measured and owned.** The
  rule assesses only tables with one full single-cell header row: 201 of 477 are outside it (**that 477 is the
  non-batched third of the corpus; the population is 1,256 — re-measure on it**), among them ADIv6's
  rotated power-control table (`Bit[28] of the CTRL/STAT register | CDBGPWRUPREQ`). APB table 17 also merges
  text across columns (`'DATA_WIDTH/8 Check_Type'`), which geometry re-homing cannot split. Extend the census
  to multi-row and spanned headers, and measure merged cells against a geometric witness (pdfplumber,
  `GRITS-CROSS-TOOL`) before choosing a repair. Scheduled after `.1d.i.a.i`.

- ID: `KG-ISF-COMPLETENESS.1d.ii` · Status: `done` (`2026-09-23`; MEASUREMENT then CODE; opened by `.1d`) ·
  Goal: **a subject phrase ends before its own verb.** `extract_subject_phrase` takes up to two words after the
  nearest determiner, so *"the HBM responds following …"* yields `HBM responds`. Of 40 two-word actors whose
  second word is a lowercase `-s` word, about 20 are this (`Manager sees`, `host tries`, `specification
  permits`) and the rest plural nouns (`Distributor registers`, `snoop transactions`); measure a cue that
  separates the two before choosing a rule.
  **MEASURED `2026-09-23` at `306878ab` — trimming is refuted; the defect is governance, not the phrase.**
  The current extractor replayed over all 78 chains (1,218 relations) gives **40** two-word actors whose second
  word is a lowercase `-s` word, every one read in its sentence: **21 swallowed a finite verb**, 19 end in a
  plural noun (first published as 41 / 22 / 20 from a count of a printed list; re-derived from the records on
  `2026-09-23`, per `CLAIM_VERIFICATION.md` §6.1 a named count is exact). Of the 21, trimming to the head would be right in only **2** — a coordinated relation verb
  (*"The Manager sees that LMOPENACK is asserted and … asserts LRCREDIT"*) and a catenative with a
  to-infinitive (*"the component continues to receive more DVM … operations"*). In the other **19** the relation
  verb sits inside the swallowed verb's COMPLEMENT and is not governed by the subject at all: trimming would
  turn `Arm recommends drives SPI`, `IOMMU offers reads DEVICE`, `chapter dictates reads DATA` into
  plausible-looking `Arm drives SPI`, `IOMMU reads DEVICE` — laundering junk into facts. **Nor does the next
  word separate verb from plural noun**: `that` follows both (*"recommends that sources"* / *"encodings that
  are"*). So the rule to measure is a GOVERNANCE test: when the subject phrase ends in a finite verb V, the
  relation is licensed only if its verb is coordinated with V (`.1e`'s shared subject) or is V's to-infinitive
  complement; otherwise it is refused, not re-attributed. Measure that test's reach both ways before coding.
  **Caveat found on re-verification:** *"the host wants to access a partition"* and *"… tries to access …"* also
  have the catenative + to-infinitive shape, so that exception alone would keep `host reads PARTITION`, where
  `PARTITION` is catalog noise rather than a signal — the governance test must be measured together with the
  signal side, not instead of it.
  **LANDED `2026-09-23`.** `swallowed_verb_governance` (`crates/specforge/src/ir/evidence.rs`) judges a
  two-word subject phrase whose second word is finite-verb-shaped (lowercase, `-s`, not `-ss`/`-us`/`-is`)
  when clause material — anything but a modal or a short adverb — stands between it and the relation verb.
  Both subject paths apply it, reading the gap from the clause BEFORE the relative-marker cut, because
  that cut is what hid *"that sources are programmed to"* from the backward path. A refused phrase ends
  the search: continuing to an earlier determiner re-attributes (*"… defined in this chapter dictates"*
  would publish `TL-C standard reads DATA`). The head is kept in two shapes: a to-infinitive complement
  (*"needs to drive"*) and coordination, which licenses only a word MARKED as a verb. Unmarked, the rule
  was measured and refuted: it added 4 wrong relations for 1 right one, because `-s` cannot tell `sees`
  from the noun `registers` (*"the Distributor registers … and provides"* gave `Distributor drives GIC`).
  The two markers are a closed-class auxiliary (`has`/`does`/`was`: without it P-Channel's `device drives
  PACCEPT` from *"has sampled … and sets PACCEPT"* is lost) and a complementizer `that` followed by the
  clause's own subject (*"sees that LMOPENACK is"*, not the relative *"encodings that are"*). The
  to-infinitive exception is reach-neutral here (`host`/`component` already hold those facts from other
  sentences) and is kept because it is the correct grammar. **Known boundary:** a two-word plural subject
  that a relative clause separates from its verb (*"the Subordinate interfaces, which are optional, drive
  HREADYOUT"*) is refused as well — no true relation of that shape is among the 50 read removals.
  **Reach, persisted statements of all 78 chains:** 1,218 -> 1,169. **50 removed, all read**: 41 refused
  wrong relations (the 21 swallowed verbs, the plural-noun phrases, and one-word actors normalised from a
  swallowed phrase — `register` from *"contains information that enables test"*, AXI `Subordinate reads
  RVALID` from *"has only one source of read data"*) and 9 collapsed onto a key already held from another
  sentence. **1 added**, the one re-attribution in the corpus: LTI `Manager drives LRCREDIT`, correct.
  **Rebuild of the 27 replayable chains with both binaries:** 25 byte-identical; ADIv6 99 -> 98 (`fault
  conditions drives In`, from *"two fault conditions that are not included in …"*) and I2C 12 -> 10 (`wiring
  patterns drives SDA`/`SCL`), all wrong.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1d.ii — a subject phrase ends before its own verb`


- ID: `KG-ISF-COMPLETENESS.1d.ii.a` · Status: `open` (`2026-09-24`; MEASUREMENT then CODE; opened by `.1d.ii`) ·
  Goal: **a plural subject behind a relative clause keeps its relation.** `.1d.ii` refuses *"the Subordinate
  interfaces, which are optional, drive HREADYOUT"*, because behind a relative clause the `-s` word cannot be
  told from a verb (pinned by `a_plural_subject_behind_a_relative_clause_is_refused`). No true relation of that
  shape is in the corpus today, so the recall cost is zero now and grows with the corpus. Measure a cue that
  separates the two (the relation verb's agreement: plural subjects take the base form, *drive*, while a
  swallowed finite verb leaves the relation verb inside its complement) before changing the rule. Scheduled
  after `.1f`.

- ID: `KG-ISF-COMPLETENESS.1e` · Status: `done` (`2026-09-23`; CODE; opened by `.5.xiii.a.5.a`) · Goal: **a
  relation's direction comes from the verb that governs the signal, not from co-occurrence.**
  **SIX OF WISHBONE'S SEVEN WRONG DIRECTIONS WERE ONE DEFECT: AN ACTIVE VERB'S OBJECT RAN TO THE END OF ITS
  CLAUSE.** `active_object_contains_signal` now also ends the object at a **coordinated third-person relation
  verb** (`coordinated_relation_verb_start`: *"latches [DAT_I()] and negates its [STB_O]"*, *"monitors the
  MASTER's [STB_O] signal, and asserts [ERR_I]"*) and at the causal adjunct **`in response to`** (*"asserts
  [ACK_I] … in response to every assertion of the [STB_O]"*). Only an `-s` verb opens a verb phrase:
  coordinated NOUN objects stay the object (`4cb4b535`), and a base form after `and` is routinely a noun or a
  signal name.
  **THE CUT NEEDED ITS PARTNER, MEASURED.** Alone it lost 3 correct facts (Avalon `source interface drives
  VALID`, LTI `Subordinate drives LACREDIT`/`LCCREDIT`): the coordinated verb's subject is ellipted.
  `active_verb_subject` gives a coordinated FINITE verb the subject of the verb it is coordinated with
  (`last_third_person_relation_verb`), at all five active subject sites. Two further corrections, each from
  a measured record: the governing verb may end right before `and` (without it, both TileLink documents
  published `agents drives DATA`), and a base form never shares a subject (without it, Avalon's *"…, and read
  after the rising edge of clk"* published `Master reads CLK`).
  **Reach.** 27 replayable chains: **26 byte-identical**; Wishbone 41 -> 35, exactly the six wrong
  relations (`fact_provenance` -6, manifest and proof ledger with them). Persisted statements of all 78
  chains, replayed both ways: **34 facts removed, all wrong** (the six, LTI `Subordinate reads LACREDIT`,
  TileLink `receiver drives DATA`, HBM `HBM device reads DBI`, and junk actors or signals: `possible
  mismatch`, `NOT`, `ANY`, `AN`); 7 re-attributed, all on non-signal catalog names (`AN`, `NOT`, `BUS`,
  `RCA`, `CLASS`); AHB `Manager drives HTRANS`, LTI's two credits and Avalon `VALID` verified present.
  The seventh wrong direction (`MASTER reads STB_I`, a pronoun subject) is a different class: `.1f`.
  Verification: acceptance checklist below.
  Commit: `KG-ISF-COMPLETENESS.1e — a relation's object ends where its verb phrase ends`

- ID: `KG-ISF-COMPLETENESS.1f` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by `.1e`) ·
  Goal: **a pronoun subject resolves to its antecedent, not to the nearest role noun.** Wishbone publishes
  `MASTER reads STB_I` from *"the MASTER will assert the select signals [SEL_O()] during every bus cycle, but
  a particular slave is only accessed when it monitors that its [STB_I] input is asserted"*: the subject of
  `monitors` is `it`, whose antecedent is *a particular slave*. Measure how many active relations take their
  actor across a pronoun, then read a sample before choosing a rule. Verification: per-relation replay diff
  over the 27; the persisted-statement replay over all 78.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1d.i.a.ii.b` (CODE) — DONE `2026-09-24`

- [x] **REPRODUCE / MEASURE** — `.1d.i.a.ii.a`: every persisted table premise equals the pre-repair helper's output from its retained converter document (1,256 of 1,256), and exactly 8 differ under the current helper.
- [x] **ROOT CAUSE (WHY + WHERE)** — `SourceIr::rebuild_from_retained_capture` (`crates/specforge/src/ir/source.rs`) re-executes `proof_context` and never recaptures; the table rows live in `field_premises.structured_tables` as captured helper output, so no existing path could carry `.1d.i.a.i` into a persisted artifact short of a Docling re-run (which brings measured drift).
- [x] **ADDRESSED (verified)** — `recapture_tables_from_retained_converter` + `locate_recaptured_tables` + `recapture_retained_tables` + example `source_table_recapture`; dry run over the 27 retained documents: would-change exactly AHB (5 tables) and APB (3), 25 unchanged, both proofs re-derived without error. Five feature-gated tests in `table_recapture_location_tests` (single-document by `source_ref` in any order; batched by `(source_batch, source_ref)`; pre-coordinate positional with its two refusals; absent / ambiguous / unreferenced / mixed / coordinate-without-batches refused; injectivity); RED: dropping the batch from the lookup key fails exactly the batched test.
- [x] **NO REGRESSION** — `rebuild_from_retained_capture` is the same statements, moved into `load_retained_capture` / `retained_proof_context` / `refresh_from_retained_context`; no persisted artifact written; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean and the same with `-p specforge -p specforge-core --features source-proof-migration`; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,657, unchanged: the new tests are feature-gated); `kg-bench` not re-run (no producer on the default build changed); the gated tests run with `--features source-proof-migration` (5 / 5).
- [x] **GENERICITY (ADR 0006)** — locating by converter coordinates and re-reading with the ingest helper; no document, name or protocol identity.
- [x] **LOCKSTEP** — ADR 0052; book `reference/doctrine-enforcement.md` (the recapture path beside the proof-only path); `COMMIT-GATE-SINGLE-RUN.18` opened; tree frontier (-> `.ii.c`); `MEMORY.md`. No rule deleted.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1d.i.a.i` (CODE) — DONE `2026-09-24`

- [x] **REPRODUCE / MEASURE** — the calibration census re-run as a pure function over every persisted grid: 8 tables re-home (AHB 3/8/32/33/34, APB 15/16/17), matching `.1d.i.a.i`'s calibrated 6 true shifts + APB 15/17; then the population corrected to all 1,256 tables (45 bundle parts) — the calibration had read only the 477 top-level tables.
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_table_grid` (`crates/specforge/src/ir/source/docling_backend.rs`, embedded helper) copied Docling's `start_col_offset_idx` ordering and dropped the bbox, so AHB Table 2-5's `HREADY` (bbox x=116-155, under `Name` x=116-140) was emitted at index 3 (`Description`); the persisted `backend.json` shows every body cell of the 8 tables one index off cyclically.
- [x] **ADDRESSED (verified)** — `geometric_column_homes` + re-homing `extract_table_grid`; the replay over all 1,256 real `TableItem`s changes exactly the 8 tables, each read in full (AHB 8 `HREADY | Manager and Subordinate | 1`; APB 17 `PSLVERRCHK`, `PAUSERCHK` back under `Signal`); unit test `table_cells_are_rehomed_to_the_header_column_their_geometry_sits_under` drives the helper's own Python through the rotation and seven refusals (overlapping headers, two shifts, spanned body cell, collision, no geometry, straddling-only, empty placeholder); RED: `return {}` in place of `return homes` fails it on the rotated case.
- [x] **NO REGRESSION** — replay: 1,248 of 1,256 tables byte-identical to HEAD's `extract_table_grid`; the unwitnessed AXI table 93 refused (would have moved merged `S S` between headers). No persisted artifact changes in this commit, so WIRE-BASED-100 and proof currency are unmoved by construction (the helper is hashed into no artifact). `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,657, +1).
- [x] **GENERICITY (ADR 0006)** — page geometry only (header bbox overlap, one cyclic shift, span and collision refusals); no name, document or protocol identity; the unit test uses neutral identifiers.
- [x] **LOCKSTEP** — book `pipeline/sourceir.md` (new subsection *Table cells land in the column they are drawn under*, in the section on what ingest carries over); fact card `docling-tables-live-under-documents-in-a-batched-bundle`; `.1d.i.a.ii` re-scoped and `.1d.i.a.iii`'s population corrected; tree root frontier; `MEMORY.md`. No rule deleted.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1d.ii` (CODE) — DONE `2026-09-23`

- [x] **REPRODUCE / MEASURE** — the persisted-statement replay (throwaway `#[ignore]` probe, removed and the file's tail compared byte-exactly with its backup) reproduces the recorded 1,218 relations at HEAD, so it is the same instrument; the 21 swallowed-verb actors of the `.1d.ii` measurement re-read in their sentences.
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_subject_phrase` (`crates/specforge/src/ir/evidence.rs`) takes two words after the nearest determiner, or the last two before the verb, with no test of whether the second is the subject's own verb heading a complement; the relative-marker cut then hides that complement from the backward path (*"Arm recommends [that sources are programmed to] write SPI"*).
- [x] **ADDRESSED (verified)** — `swallowed_verb_governance` at both subject paths; eight seam tests in `kg_isf_completeness_1d_ii`, RED-verified by neutralising each rule: the whole rule off fails 5 (complement refusal, stop-the-search, plural-noun coordination, coordinated shared subject, to-infinitive); unmarked coordination fails exactly the plural-noun test; no auxiliaries fails exactly the P-Channel test; continuing past a refusal fails the stop-the-search test (`TL-C standard reads DATA`). The modal test is a labelled regression guard, the relative-clause test a labelled known boundary.
- [x] **NO REGRESSION** — persisted-statement replay over all 78: 50 removed (41 refused wrong, 9 collapsed onto keys held from other sentences), 1 added (`Manager drives LRCREDIT`, correct), every one read; the two refuted variants measured the same way (unmarked coordination +4 wrong; complement-only marker lost `device drives PACCEPT`). `evidence --dry-run` over the 27 with both binaries: 25 byte-identical, ADIv6 99 -> 98 and I2C 12 -> 10, all wrong. WIRE-BASED-100: `ahb`, `apb`, `axi`, three temporal, `nvme_registers`, `riscv_debug_registers` score identically with each binary on its own build; `swd`, `swd_derivation` and `i2c_signals` read the two moved chains, which the new binary refuses until rebuilt (`topology is stale`); after the rebuild all three score identically too, so all 11 seeds are unmoved. **Both moved chains rebuilt** evidence -> validate -> semantic -> validate -> intent -> validate -> adapt, pre-rebuild copies under `generated/preserved/KG-ISF-COMPLETENESS.1d.ii/`: ADIv6 `adapter.json` differs only in `proof_ledger`; I2C's lowered `.isf` differs only in the actor's NAME, `wiring_patterns` -> `optional_open_drain` (body byte-identical), and a HEAD-binary rebuild still names it `wiring_patterns`, so the move is this slice's. Both names are phrase actors: I2C's lowered actor is still named from prose, which stays with `.1d`. `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,656, +8).
- [x] **GENERICITY (ADR 0006)** — English clause structure (modals, auxiliaries, complementizer vs relative `that`, to-infinitive, coordination); no name, document or protocol identity. Flow census +1 function / +18 decision sites / +36 helper edges / +0 macros under `aggregate_change`; boundary unchanged.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (relation-precision section, with the rule's known boundary); fact card `subject-phrase-that-swallowed-its-verb-is-refused`; tree root frontier; `MEMORY.md`. No rule deleted.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1d.i` (CODE) — DONE `2026-09-23`

- [x] **REPRODUCE / MEASURE** — the current extractor replayed over all 78 chains' persisted statements (throwaway `#[ignore]` probe, removed byte-exactly): 1,342 relations; attribution through the extractor's own `active_object_contains_signal` + `active_verb_subject` found 125 licensed only by a determiner-preceded verb occurrence, and a 40-relation sample read by hand was wrong in every case.
- [x] **ROOT CAUSE (WHY + WHERE)** — `extract_actor_signal_relations` (`crates/specforge/src/ir/evidence.rs`) licensed a relation at any spelling of a vocabulary verb, and `ACTIVE_READS_VERBS`/`ACTIVE_DRIVES_VERBS` hold forms English uses as nouns (`read`, `access`, `writes`), so *"the read pointer"* minted `value reads APB`.
- [x] **ADDRESSED (verified)** — `is_nominal_verb_occurrence` at the four active licensing sites; five seam tests in `kg_isf_completeness_1d_i`: neutralising the helper fails exactly the three RED tests, and a case-blind `a` fails exactly the `Master A asserts` label guard.
- [x] **NO REGRESSION** — persisted-statement replay over all 78: 124 removed, 0 added, every removal read; `evidence --dry-run` over the 27 with both binaries: 25 byte-identical, AHB 75 -> 68 and ADIv6 109 -> 99 (15 wrong, 2 true-but-unevidenced, owned as `.1d.i.a`); both chains rebuilt through the adapter, `adapter.json` differing from a HEAD-binary rebuild only in `proof_ledger` hashes; WIRE-BASED-100 seeds `ahb`/`apb`/`axi`/`swd` and three temporal score identically with each binary on its own build (source-tolerant relations 1.000, document-level recall 1.000); `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,648, +5).
- [x] **GENERICITY (ADR 0006)** — English determiner grammar; no name, document or protocol identity. Flow census +1 function / +3 decision sites / +1 helper edge / +0 macros under `aggregate_change`; boundary unchanged.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (the relation-precision section); tree root frontier; `MEMORY.md`. No rule deleted.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1e` (CODE) — DONE `2026-09-23`

- [x] **REPRODUCE / MEASURE** — Wishbone's 41 relations read by hand (`.5.xiii.a.5.a`): 7 wrong directions; 6 trace to one object span (the verb's object ran to its clause end).
- [x] **ROOT CAUSE (WHY + WHERE)** — `active_object_contains_signal` (`crates/specforge/src/ir/evidence.rs`) cut the object only at `.`/`;` and condition markers, so a coordinated second verb's object and a causal adjunct's trigger were read as the first verb's object.
- [x] **ADDRESSED (verified)** — six seam tests in `kg_isf_completeness_1e`, each RED-verified by neutralising the rule it pins: coordination cut (2 fail), `in response to` (2 fail), shared subject (1 fails), finite agreement (Avalon test fails), governing verb before `and` (TileLink test fails); the AHB test is a labelled regression guard.
- [x] **NO REGRESSION** — `evidence --dry-run` over all 78 chains with both binaries: 27 replayed, 26 byte-identical, Wishbone −6 wrong relations only; persisted-statement replay over all 78: 34 wrong facts removed, 0 correct lost. WIRE-BASED-100 unmoved by construction (no scorable gold is Wishbone, and every other replayable chain is byte-identical); `kg-bench` 158/158; `cargo fmt` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` green (473 / 168 / 1,643, +6). Wishbone chain rebuilt through the adapter (still no `.isf`, as before).
- [x] **GENERICITY (ADR 0006)** — English clause structure plus the existing relation-verb vocabulary; no name, document or protocol identity. Flow census +3 functions / +9 decision sites / +4 helper edges / +1 macro under `aggregate_change`; boundary unchanged.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (the Wishbone relation-precision paragraph); tree root frontier; `MEMORY.md`. No rule deleted.
