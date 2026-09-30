# EXTRACTION-REPRESENTATION-GAPS — register binding leaves

- Part ID: `register-binding-leaves`
- State: `active`

New leaves outside the `.3` family continue here, from `.41.g` on, after `clause-and-register-leaves` reached its
mandatory rollover (`2026-09-30`): the register-field-to-signal binding readers and their residuals.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.g`
  Status: `done` (`2026-09-30`; CODE; found by `.41.e`; checklist below; opened `.41.h`)
  Goal: **the AHB appendix's HPROT bit map binds `CSW.Prot`.** ADIv6 *E1.4 AMBA AHB3* states *The CSW.Prot field
  drives the AHB HPROT signals*, then maps bits 27-24 to `HPROT[3:0]` in a table whose caption the converter lost
  (`table_0205`: `Bit | HPROT signal | Description | Description when not implemented …`), and binds bit 28 in prose
  under a sub-field heading (*HPROT[4], Allocate, bit[28]*: *Drives HPROT[4]*). **Reproduce:** `CSW@MEM-AP.Prot`
  has no `HPROT` binding. **Impact:** an AHB MEM-AP's protection bits reach no wire. Acceptance: decide whether the
  sentence naming the field (*The CSW.Prot field drives the AHB HPROT signals*) locates the field for the captionless
  table under it and for the sub-field headings of that section, and measure every chain.
  **Measured (`2026-09-30`), and it overturns a premise of `.41.e`.** The appendix states HPROT bindings in two
  prose shapes: AHB3 heads *HPROT[4], Allocate, bit[28]* and *HPROT[3:0], bits[27:24]*, each followed by *Drives
  HPROT[…]*; AHB5 lists *Bit[27] drives HPROT[6], HPROT[4], and HPROT[3].* So **HPROT[4] is driven by bit 28 in AHB3
  and by bit 27 in AHB5**: one signal, two bit maps, one merged `CSW@MEM-AP` record. `.41.e` reasoned that a bus
  variant is implied by its signals and merged twin tables without a scope; that holds for the AXI twins, whose
  content is identical, and fails here. SourceIR's heading levels are all 1 in this appendix, but the numbered
  titles carry the nesting (`E1.4.1 …` under `E1.4 AMBA AHB3`). APB4 writes *Drives PPROT[2:0].* far from its field
  heading once converted, so no adjacency grammar reads it.
  **Design:** a binding records the numbered heading path it is stated under (`scopes`), for tables (`.41.e`) and
  prose alike; *Bit[N] drives SIG[n], …* and a *SIG[h:l], bits[H:L]* heading followed by *Drives SIG[h:l]* are
  read under the nearest *<REG> register implementation* heading, into the one field of that register holding the
  bits. APB4's PPROT map stays a stated residual.
  **Measured after:** the first run bound AHB5's bit 27 only. `CSW@JTAG-AP` shares the name and holds bits 30:28
  (`WFIFOCNT`) and 26:24 (`RFIFOCNT`), so a single bit named two fields and the reader failed closed. The section's
  own field headings (*Prot, bits[30:24]*, *SDeviceEn, bit[23]*) name fields only `CSW@MEM-AP` has, so they pick
  the register when several share the name. EvidenceIR dry-run of the 31 chains with the HEAD and the new binary:
  30 identical; ADIv6 moves only `register_records`. `CSW@MEM-AP.Prot` gains 7 `HPROT` bindings (bit 28 ->
  `HPROT[4]` in AHB3; bits 27-24 -> `HPROT[3:0]` in both; bit 27 -> `HPROT[6]` and `HPROT[4]` in AHB5), and every
  binding in the document, prose or table, gains its scope (the 14 AXI bindings name both AXI sections; `.41`'s 10
  name their register-description sections).

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.h`
  Status: `open` (`2026-09-30`; CODE; found by `.41.g`)
  Goal: **APB4's PPROT map binds `CSW.Prot[2:0]`.** ADIv6 *E1.7 AMBA APB4* heads *Prot[2:0], bits[30:28]* and says
  *Drives PPROT[2:0].*, but the conversion puts the sentence 13 elements after its heading, as a section header,
  and puts field headings for later bits in between; *Bit[29], Non-secure, corresponds to Prot[1] … PPROT[1] is
  HIGH* follows it. **Reproduce:** `CSW@MEM-AP.Prot` has no `PPROT` binding. **Impact:** an APB4 MEM-AP's
  protection bits reach no wire. Acceptance: decide whether the heading's field selection (`Prot[2:0]`) with a
  later *Drives SIG[2:0]* of the same width in the same register section binds, without pairing a sentence with
  the wrong heading, and measure every chain.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41.g` (CODE) — DONE `2026-09-30`

- [x] **REPRODUCE / MEASURE** — ADIv6's AHB3 and AHB5 sections map `CSW.Prot` bits 28-24 to `HPROT` in prose; no
  binding carried them, and nothing recorded which implementation stated a binding.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `fill_field_signal_bindings` reads whole
  bindings under a field's own heading only, and `RegisterFieldSignalBinding` had no scope. RED, with the file restored
  byte-identically after each (8 mutations): no scope, no parent chain, a numbered heading keeping the register, a
  heading bound without its *Drives* sentence, or no section narrowing fails
  `an_implementation_section_binds_field_bits_in_prose_with_its_scope`; no merge also fails the `.41.e` test; accepting
  an ordinary word as a register fails `a_prose_bit_map_is_read_by_its_grammar_only`; an unwired reader fails
  `the_register_surface_binds_a_field_a_bit_map_names`.
- [x] **ADDRESSED (verified)** — `fill_field_signal_prose_bit_bindings`, `merge_signal_binding`,
  `numbered_heading_path`, `heading_section_number`, `register_heading_name`, `field_bits_heading`,
  `bit_drives_selects`, `signal_bits_heading`, `drives_selection`; `RegisterFieldSignalBinding.scopes`. ADIv6 rebuilt
  stage by stage, each stage validated once, the adapter last: 21 bindings on `CSW@MEM-AP.Prot`, all scoped, in
  EvidenceIR, SemanticIR and IntentIR.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical.
  `check_chain_currency.sh` 31 of 31 current (no seal moved). WIRE-BASED-100: all 11 seeds identical to the reports of
  `02ee6421b`'s binary, unchanged in Rust since (temp paths aside); `kg-bench` 158/158. `cargo fmt --check` clean;
  `cargo clippy --offline --workspace --all-targets -- -D warnings` clean (a nested tuple became `SignalBitsHeading`);
  `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,810 passed, 0 failed.
  `flow_census.json` rolled forward (+10 functions, +68 decision sites, +114 helper edges, +3 macros).
- [x] **GENERICITY (ADR 0006)** — section numbering, heading and sentence shapes, field headings against the register
  records' own fields; no document, register or signal name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field bound to a wire*); KM
  `a-register-field-binding-to-a-wire` (amended); `.41.e`'s record corrected; `.41.h` opened.
