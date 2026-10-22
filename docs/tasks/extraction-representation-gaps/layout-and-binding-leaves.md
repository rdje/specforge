# EXTRACTION-REPRESENTATION-GAPS — layout and binding leaves

- Part ID: `layout-and-binding-leaves`
- State: `active`

Open ERG leaves continue here, after `encoding-and-state-leaves` reached its rollover (`2026-10-22`): a signal's
sub-field layouts, a signal laid out per message kind, a register field's sub-fields, and every later ERG leaf.
`.3.c.iv.b.i`, `.3.c.iv.b.ii`, `.3.c.iv.c` and `.60` moved here whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.i`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.iv.b`; checklist below)
  Goal: **a table naming a signal's bit sub-fields is typed.** **Reproduce:** AHB *3-6*, APB-E `table_0007`, AXI-L
  *A12.3*-*A12.6* reach EvidenceIR as raw rows only. Acceptance: the `signal_layouts` surface, its shape and level
  reading as `.3.c.iv.b` decided; every other family of the 23 tables untouched; measure the 31 chains; amend the
  book's *captured region* sentence.
  **Done `2026-10-22`.**
  - **Reader.** `read_signal_layouts` reads `.3.c.iv.b`'s shape (`parse_name_slice`: a slice of one name; the name
    column the first other column of distinct cells of at most five words, no number). A number column must restate
    each slice's width; another column holding a slice or a binary literal refuses the table. One `SignalLayoutRecord`
    per table and its continuations, with `SignalSubField` rows (slice, high and low bit, name, other cells as
    written) and, for a one-bit sub-field, `SubFieldLevel`s (`sub_field_levels`, `sentence_level`).
  - **Manifest.** `signal_layout_surface` records the `signal_layouts` entry only when a table had the slice column,
    so a document without one keeps its manifest (as `.3.b.iii.i` did for transition states).
  - **Measured.** Dry-run of the 31 chains: 27 identical. AHB gains 1 layout (*3-6*: 7 sub-fields, 12 levels;
    `HPROT[2]` none, under its condition), APB5 1 (3 sub-fields, 6 levels from the `LOWindicates`/`HIGH indicates`
    bullets), AXI 4 (*A12.3*-*A12.6*: `MPAM_NS`/`MPAM_SP`, `PARTID`, `PMG` onto `AxMPAM`), and ADIv6 counts its
    caption-less `HPROT` map as a refused candidate (manifest 1 produced, 0 kept). No statement moved.
  - **Rebuilt.** AHB, APB5, AXI and ADIv6 rebuilt stage by stage: every later stage and `.isf` text byte-identical,
    the adapter's residuals unchanged (AHB 41, APB5 41, AXI 83, ADIv6 30). The new field moved the EvidenceIR ruleset
    hash, so `rebuild_stage_cascade.sh --write` re-sealed all 31 chains, content-identical at every stage.
  - **Part rollover.** With this record `encoding-and-state-leaves` passed its 90% rollover (93.3% of its 65,536-byte
    target): new part `layout-and-binding-leaves` (contract and manifest part, leaf routes, index, route catalog,
    parts surface, routing paragraph, `MEMORY.md`); `.3.c.iv.b.i`, `.3.c.iv.b.ii`, `.3.c.iv.c` and `.60` moved whole,
    and the old part is at 84.5%.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.i` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — `.3.c.iv.b`'s census (`scan2.py`, 23 tables, five families); the persisted EvidenceIR
  held AHB *3-6*, APB5 `table_0007` and AXI *A12.3*-*A12.6* as raw row statements only.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` had no reader for a table keyed by one
  signal's slices; the value-combination and single-column readers key on titles or codes. RED 4/4, the file restored
  byte-identically: levels read under a condition, a number column not checked against the widths, overlapping slices
  kept, a level word needing its space; each fails `a_table_naming_a_signals_sub_fields_is_its_layout`.
- [x] **ADDRESSED (verified)** — the six layouts above in the dry-run and the rebuilt chains.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports and the
  temporal one byte-identical to `f5ceee7cb`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo
  clippy --offline --workspace --all-targets -- -D warnings` clean (an `if let` collapsed). `cargo test --workspace
  --lib --exclude specforge-production-graph` 475 / 171 / 1,922 passed; the proof test counts 50 EvidenceIR rule
  fields and the inventory 206 rules (`signal_layouts` added to `evidence.protocol_structure` in both inventories).
  `flow_census.json` rolled (+6 functions, +57 decision sites, +94 helper edges, +2 macros) and its boundary moved for
  the new producer (rule roots 134, grammar declassifiers 75).
- [x] **GENERICITY (ADR 0006)** — the shape is read from cells (a slice of a declared name, a number restating a
  width, a binary literal, a level sentence's opening words); no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A signal's bit sub-fields*; the *captured region* sentence
  amended, as `.3.c.iv.b` named); KM card `a-table-naming-a-signals-bits-is-its-layout`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii`
  Status: `open` (`2026-10-22`; CODE; opened by `.3.c.iv.b`)
  Goal: **a signal's sub-field layout reaches IntentIR, and the adapter names it.** Acceptance: carry `signal_layouts`
  through SemanticIR to IntentIR; an adapter residual per layout, naming any sub-field past its width parameter's
  default; `.isf` texts unchanged and FSMGen-strict-clean; rebuild what moves. Then re-evaluate ADR 0033 §4 for
  `docs/gaps/fsmgen-ladder.md` F29: file the drafted request (`.project-data/tmp/req-port-fields/packet`) only if an
  executable consumer needs the layout.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.c`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`)
  Goal: **a signal laid out per message kind is read.** **Reproduce:** AXI-L *A15.20*-*A15.22* and their continuations
  give each `ACADDR` bit a constant or a source field (`VA[56]`) per TLBI/BPI/VICI/PICI message part, and no reader
  takes them. Acceptance: decide the record (a layout per message kind, with constant and source-field bits), how it
  joins the message kinds' encodings and `DOC-INTENT-TAXONOMY.4b`'s layouts, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.60`
  Status: `open` (`2026-10-22`; DESIGN; opened by `.3.c.iv.b`)
  Goal: **a register field's sub-field table types the field's sub-fields.** **Reproduce:** ADIv6 *B3-10* lays
  `IDCODE`'s `DESIGNER` (`bits[11:1]`) out as *Continuation code* `Bits[11:8]` and *Identity code* `Bits[7:1]`, with
  ARM's values; `RegisterFieldRecord` has no sub-fields. Acceptance: decide the carrier (as `MessageSubField` did for
  message fields, `.55.b.iv.b.i`), how the register's bit positions map onto the field's, and measure every chain.
