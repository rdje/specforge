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

- ID: `EXTRACTION-REPRESENTATION-GAPS.45`
  Status: `open` (`2026-10-03`; CODE; found by `.3.d.ii.a`)
  Goal: **a presence row whose converter fused two code cells keeps the header's variant labels.** APB5
  `table_0018` (*Table B-3*, header `Signal | Width | Property | APB5 | APB4 | APB3 | APB2`) has a row the converter
  rendered as `PWAKEUPCHK | 1 | Check_Type &Wakeup_Signal | C | N N |  | N` — two codes fused into one cell and the next
  cell empty. The capture's rotation remap then relabelled the whole fragment: `presence_0020`-`presence_0023`
  (`PWDATACHK`, `PSTRBCHK`, `PREADYCHK`, `PBUSERCHK`) carry variant labels `OC`/`N`/`N`/`N` and `PWDATACHK`'s
  condition reads `DATA_WIDTH/8 Check_Type`, while the fused row itself is refused. **Reproduce:** `python3 -c
  "import json; e=json.load(open('generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json')); print([(r['presence_id'], [v['variant_label'] for v in r['variant_presence']]) for r in e['signal_presence_records'] if r['table_id']=='table_0018'])"`.
  The same converter fault hits `table_0016`: `PRESETn | 1 | - | - | Y Y |  | Y | Y` and `PADDR | ADDR_WIDTH | - | - |
  Y | Y | Y Y | ` are refused whole, so APB5 has no presence row for `PADDR` or `PRESETn` and `.3.d.ii.a.i`'s class
  value reaches 15 of the 16 covered signals. **Impact:** four check signals state presence under labels that are
  codes, not versions; a consumer reading `APB5` finds nothing for them, and two always-present signals have no row. Acceptance: find why one fused row moves the fragment's header remap (the
  `.5h` rotation votes, or the fused-pair split), keep the header labels when the header row is a proper
  header, refuse only the fused row, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.46`
  Status: `open` (`2026-10-03`; DESIGN then CODE; found by `.3.d.ii.a.i`)
  Goal: **a parity check signal lowers from the signals it covers and its enable, or the gap is requested.** IntentIR
  now carries 153 `check_signal_records` in 3 documents (AXI 123, AHB5 16, APB5 14: a check signal, its covered
  signals, width, granularity and enable, all as written). `.isf` has `(^ A B)` and `(set …)` but no parity or
  reduction form, so a byte-granular check (`ceil(ADDR_WIDTH/8)` wide, granularity `1-8`) would be a chain of XOR
  rules per byte that the adapter does not emit; a 1-bit check over several signals (`PCTRLCHK` over `PPROT,
  PWRITE, PNSE`) is one XOR chain under its enable (`PSEL`). **Reproduce:** `python3 scripts/measure_intent_to_isf_reach.py
  residuals` lists no check signal; `validate` on AXI's IntentIR reports `check_signal_records: 123`. **Impact:** the
  parity structure every AMBA5 interface declares reaches `.isf` as plain ports with no drive. Acceptance: decide,
  per record shape, whether the adapter emits a combinational rule (and how a parameterised byte count is expressed
  in `.isf`'s width vocabulary) or whether a parity form is a `FSMGEN-REQUESTS` packet; then emit or request, and
  measure every emitting chain under FSMGen `--strict --check`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.47`
  Status: `open` (`2026-10-03`; CODE; found by `.3.d.ii.a.ii`)
  Goal: **a rule stated on a collective name reaches each member.** `.3.d.ii.a.ii` carries the collective names
  AXI defines (`collective_signal_names`) and resolves them for one reader, the absent-value binder. Every other
  reader still takes `AxVALID`, `AxADDR` or `AxMMUATST` as a name of its own: AXI uses a collective name in 245 of
  its statements (*When AxMMUATST is deasserted LOW, the Stall flow is used*; *AxBURST* encodings), so each such
  constraint, condition, enum and relation reaches neither `AW*` nor `AR*`. **Reproduce:** `python3 -c "import json,re;
  e=json.load(open('generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json'));
  print(sum(1 for s in e['extracted_statements'] if re.search(r'\\bAx[A-Z]', s['text'])))"`. **Impact:** the largest
  AMBA document's generic rules are typed on a signal no interface has. Acceptance: decide where resolution lives
  (a SemanticIR projection that duplicates a record per member, or each reader consulting the records), apply it to
  the readers that resolve a condition's or constraint's signal, keep enum definitions on the collective name where
  the document defines them there, and measure every chain and the 11 seeds.

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
