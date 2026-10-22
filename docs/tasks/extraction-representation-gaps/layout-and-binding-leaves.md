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
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports (the three
  temporal ones among them; first written "and the temporal one", corrected by `.3.c.iv.b.ii`) byte-identical to
  `f5ceee7cb`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean (an `if let` collapsed). `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,922 passed; the proof test counts 50 EvidenceIR rule fields and the
  inventory 206 rules (`signal_layouts` added to `evidence.protocol_structure` in both inventories).
  `flow_census.json` rolled (+6 functions, +57 decision sites, +94 helper edges, +2 macros) and its boundary moved for
  the new producer (rule roots 134, grammar declassifiers 75).
- [x] **GENERICITY (ADR 0006)** — the shape is read from cells (a slice of a declared name, a number restating a
  width, a binary literal, a level sentence's opening words); no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A signal's bit sub-fields*; the *captured region* sentence
  amended, as `.3.c.iv.b` named); KM card `a-table-naming-a-signals-bits-is-its-layout`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii`
  Status: `done` (`2026-10-22`; CODE; opened by `.3.c.iv.b`; checklist below)
  Goal: **a signal's sub-field layout reaches IntentIR, and the adapter names it.** Acceptance: carry `signal_layouts`
  through SemanticIR to IntentIR; an adapter residual per layout, naming any sub-field past its width parameter's
  default; `.isf` texts unchanged and FSMGen-strict-clean; rebuild what moves. Then re-evaluate ADR 0033 §4 for
  `docs/gaps/fsmgen-ladder.md` F29: file the drafted request (`.project-data/tmp/req-port-fields/packet`) only if an
  executable consumer needs the layout.
  **Done `2026-10-22`.**
  - **Carried.** `signal_layouts` is a field of SemanticIR (`semantic.carried_evidence`) and IntentIR
    (`intent.carried_semantics`), cloned as it is; both claim-family inventories name it.
  - **Residual.** `signal_layout_residuals` states one `isf_protocol_signal_layout_*` packet per layout: no `.isf`
    port form (ladder F29), and per port the layout names (the signal, or each member of its collective) every
    sub-field whose high bit reaches the port's elaborated width (`IsfIr::signal_width`), with the parameter's
    default. A layout whose signal is no port of the actor says so.
  - **Measured.** AHB's residual names `Lookup [4]`, `Allocate [5]` and `Shareable [6]` past `HPROT_WIDTH`'s default
    4. APB5's `PPROT` fits its width 3. AXI's four name each layout's fields past `MPAM_WIDTH`'s default 1 (a
    placeholder default, `KG-ISF-COMPLETENESS.2a.vii`) on `ARMPAM` and `AWMPAM`. Residuals: AHB 41 -> 42, APB5 41 ->
    42, AXI 83 -> 87; only `signal_layouts` moved in SemanticIR and IntentIR, and every `.isf` text is byte-identical.
  - **ADR 0033 §4, re-evaluated.** No rule, guard or drive names a sub-field, and a field changes no generated HDL, so
    no executable consumer needs the layout: no request is filed, and F29 stays *anticipated*, now owned by
    `.3.c.iv.b.iii` with its drafted bundle and trigger.
  - **Rebuilt.** AHB, APB5, AXI and ADIv6 rebuilt stage by stage (ADIv6 unchanged at every stage). The new fields
    moved the SemanticIR and IntentIR ruleset hashes, so `rebuild_stage_cascade.sh --write` re-sealed all 31 chains,
    content-identical at every stage.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii` (CODE) — DONE `2026-10-22`

- [x] **REPRODUCE / MEASURE** — after `.3.c.iv.b.i` the six layouts stopped at EvidenceIR; the adapters' residual
  packets per chain; the ports the layouts name and their widths in each `.isf`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` and `intent.rs` declared no carrier, and
  `crates/specforge/src/ir/adapters.rs` stated no residual. New carried fields, `signal_layout_residuals` and
  `IsfIr::signal_width` (`isf_ir.rs`). RED 5/5, each file restored byte-identically: SemanticIR dropping the layouts,
  IntentIR dropping them, the adapter stating no residual, no field listed past the width, a collective not expanded;
  each fails its test (`projects_protocol_surfaces_exactly_across_all_three_stages`,
  `isf_adapter_residualizes_a_signal_layout_naming_fields_past_the_width`).
- [x] **ADDRESSED (verified)** — the residuals above on the rebuilt chains; `.isf` texts byte-identical, so their
  FSMGen strict verdicts are unchanged.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `86ea02e05`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude
  specforge-production-graph` 475 / 171 / 1,923 passed; the proof tests count 64 SemanticIR and 63 IntentIR rule
  fields (43 carried) and the inventory 208 rules. `flow_census.json` rolled (+2 functions, +8 decision sites, +19
  helper edges, +1 macro); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the residual reads the record, the collective's members and the port's own width;
  no document, signal or parameter name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A signal's bit sub-fields*: carriage and residual); KM card
  `a-table-naming-a-signals-bits-is-its-layout`; ladder F29 re-evaluated, unchanged.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.iii`
  Status: `open` (`2026-10-22`; REQUEST, waits for a consumer; opened by `.3.c.iv.b.ii`)
  Goal: **a port's sub-field layout reaches FSMGen once something executable names a sub-field.** FSMGen ladder F29 is
  *anticipated*: no `.isf` port takes a field, and ADR 0033 §4 files a request only when executable intent needs one.
  Trigger: a rule, guard or drive SPECFORGE lowers names a sub-field (`HPROT` *Privileged*, `AxMPAM` *PARTID*).
  Acceptance: on the trigger, refresh the drafted bundle (`.project-data/tmp/req-port-fields/packet`, reproduced on
  the pin `54ac1c22` and upstream `c51e22712`) against the current pin and upstream, sign it off as an
  `FSMGEN-REQUESTS` leaf and its ledger record, and move F29 to *requested*.

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
