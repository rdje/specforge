# EXTRACTION-REPRESENTATION-GAPS — unread intent leaves

- Part ID: `unread-intent-leaves`
- State: `active`

The `.3.d` family continues here, after `table-census-leaves` reached its mandatory rollover (`2026-09-30`): the
IntentIR assumptions and behaviors no typed record understands, and the readers that will.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d`
  Status: `done` (`2026-09-30`; MEASUREMENT; the sealed record is in `program-and-census`; children `.3.d.i`-`.3.d.iii`)
  Goal: unchanged: decide what the unread IntentIR assumptions and behaviors are, and route each class.
  **Measured `2026-09-30`** (`python3 scripts/measure_unread_intent_records.py --samples 7`, new and read-only; it
  imports `measure_category_family_census.py`'s predicate). 443 records cite no statement a typed record cites: 166
  assumptions, 277 behaviors (167 and 288 on `2026-09-26`). No IR field says whether an agent is software (SemanticIR
  actors are role nouns), so the script declares a closed list of software-agent words; every other test is
  structural. Families, first match wins, seven seeded members of each read:
  - **pass assumption, 17:** IntentIR's own modeling choices, which cite no statement (`intent.rs` writes them).
    By design; not document intent.
  - **table row, 50:** Markdown table rows lifted as prose (7 of 7). Their tables' family owners (`.3.b`) own them.
  - **apparatus, 40:** document self-reference, figure examples, informative notes (5 of 7; 2 notes carry an
    obligation). Non-contract.
  - **software agent, 47:** software obligations (6 of 7; 34 of the 47 are in the USB4 connection-manager guide);
    one is I2C's *output drivers*, hardware. `.3.d.i`.
  - **assumed by word, 111:** SemanticIR lifts a sentence to an abstraction by the words *assume* or *treated as*;
    4 of 7 are hardware rules or defaults, 2 conventions, 1 a fragment (*60% Cu Assumer*). `.3.d.ii`.
  - **permission or description, 178:** 2 of 7 obligations no reader reads, 2 permissions, 3 descriptions; 17 carry
    the `timing_constraint` class though none states a time. `.3.d.iii`.
  **Decided:** assumptions are not a class of intent, and the behaviors are four kinds of content; each open kind
  has an owner. The census record is in [the category-family census](../../research/category-family-census.md).
  No code change, so no acceptance checklist applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.i`
  Status: `open` (`2026-09-30`; DESIGN + CODE; found by `.3.d`)
  Goal: **a software agent's obligation is recorded as software intent, not as an untyped behavior.** 47 unread
  records oblige or permit a software agent (*The Connection Manager may enable CLx 50ms after TMU has been
  activated*; *The firmware is responsible for creating the stage 2 translation tables for the untrusted OS*); 34 are
  in the USB4 connection-manager guide. The category
  contract calls such content non-applicable to executable IntentIR and requires an explicit disposition.
  **Reproduce:** `scripts/measure_unread_intent_records.py`, family `software_agent`. **Impact:** a document's
  software procedure is indistinguishable from unread hardware behavior. Acceptance: decide how an agent is known to
  be software from the document (no IR field says so today), give the obligation a typed non-executable disposition,
  and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii`
  Status: `done` (`2026-09-30`; MEASUREMENT, re-scoped from CODE; found by `.3.d`; children `.3.d.ii.a`, `.3.d.ii.b`)
  Goal: **an assumption is what a document states it assumes.** SemanticIR lifts any sentence saying *assume* or
  *treated as* to an abstraction, and IntentIR carries it as an assumption: 111 such records are hardware rules or
  defaults (*If a fault condition prevents locating a valid device context then the DTF value assumed for reporting
  such faults is 0*; *the missing signals are assumed to be LOW*), conventions (*the latencies shown assume an L1
  hit*) and fragments. **Reproduce:** family `assumed_by_word`. **Impact:** a default value the hardware must use is
  filed as an assumption no reader lowers. Acceptance: read a stated default (`X is assumed to be V`) into the typed
  record its subject takes, keep a convention as a non-contract assumption, and measure every chain.
  **Measured `2026-09-30`** (the family's 111 records, by sentence shape): 5 give an absent signal's value (AXI's
  *If the BRESP_WIDTH property is 0, it is not present and assumed to be 0b000 (OKAY)*, RRESP alike, *Any AxSNOOP bits
  not driven by an interface are assumed to be LOW*, and APB-E's and AHB's *the missing signals are assumed to be
  LOW*); 9 more state a default (AXI's domain defaults when `AxDOMAIN` is omitted, IOMMU's *the DTF value assumed …
  is 0*); 31 are *treated as* rules (*LPIs are always treated as Non-secure Group 1 interrupts*); 14 are conventions
  (*This guide assumes that you are familiar with …*); 52 mix the rest.
  **Why this is not one code slice:** the carrier an absent value belongs on, EvidenceIR's `signal_presence_records`
  (`BRESP_WIDTH > 0`), stops at EvidenceIR — no SemanticIR or IntentIR field carries presence. Carrying it is a new
  top-level field in two stages and re-seals every chain; FSMGen already accepted the matching `.isf` request
  (`sf-isf-optional-signal`, its `(absent-value V)`). So the measurement closes here and the code is two children.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a`
  Status: `done` (`2026-10-03`; CODE; found by `.3.d.ii`; checklist below; opened `.3.d.ii.a.i`, `.3.d.ii.a.ii` and
  `.45`)
  Goal: **a signal's stated absent value is carried with its presence to IntentIR.** Carry
  `signal_presence_records` through SemanticIR and IntentIR, and read a sentence that gives an absent or undriven
  signal a value (*not present and assumed to be 0b000*, *not driven … assumed to be LOW*, *the missing signals are
  assumed to be LOW*) into that signal's record as its absent value, citing the statement. **Reproduce:** the 5
  records of `.3.d.ii`'s first shape. **Impact:** a width-0 `BRESP` has no value in IntentIR, so no consumer can tie
  it off. Acceptance: the carrier and the reader, the re-seal handled as `.3.b.iii.g`'s was, and every chain measured.
  **Decided `2026-10-03`: the value lives on the presence record, literal and cited.** `SignalPresenceRecord` gains
  `absent_value: Option<StatedAbsentValue { value, supporting_statement_ids }>`, bound after the matrix capture
  inside the one registered `signal_presence` surface (no new extractor entry, so no other document's manifest
  moves), and `signal_presence_records` cross SemanticIR (`semantic.carried_evidence`) and IntentIR
  (`intent.carried_semantics`) unchanged. The value is the literal as written and is never interpreted against the
  signal's polarity: the sentence states a literal, and the consumer that ties the signal off decides its meaning on
  the wire. **The reader:** in one sentence an absence predicate (*not present*, *not driven*, *omitted*, *missing*)
  precedes *assumed to be*, and a literal follows — a binary, hexadecimal or decimal number, a word of two or more
  capitals, or *asserted*/*deasserted* — closed by the end of the sentence, a clause break or one parenthetical label
  (*0b000 (OKAY)*). The signal is the one presence-record name the whole statement mentions, so the second
  sentence's *it* resolves to `BRESP` while `BRESP_WIDTH`, a property, is no candidate. A qualified value (*assumed
  to be 0b0 for a DVMComplete request*) is not read; a statement naming no presence record or two binds nothing;
  statements that disagree bind nothing, and repeats are all cited. **Of the 5 target records, 2 bind** (`BRESP`,
  `RRESP`); the other 3 are the two shapes the reader cannot resolve without a carrier, each now owned:
  `.3.d.ii.a.i` (the class *signals covered by a check signal*, whose members APB5's *Table 5-1 Check signal
  descriptions* and AHB5's *Table 12-1 Parity check signals* state in a `Check signal | Signals covered` table) and
  `.3.d.ii.a.ii` (the generic `AxSNOOP`, whose rows are `AWSNOOP`/`ARSNOOP`; AXI's alias map is empty). The `.isf`
  adapter keeps one `isf_protocol_signal_absent_value_<presence_id>` residual per stated value (`.isf` has no
  `(absent-value V)` at the pin; `sf-isf-optional-signal` R3 is acknowledged).
  **Measured over all 31 chains:** evidence dry-run with the HEAD and the new binary, 30 identical; AXI moves only in
  `signal_presence_records` (`presence_0082` `BRESP` and `presence_0143` `RRESP` gain `0b000`, citing
  `statement_1089` and `statement_1118`). The unread `assumed_by_word` family: 111 -> 109 (443 ->
  441 records). The three presence-bearing chains (AXI, APB5, AHB5) change content at SemanticIR and
  IntentIR (the carried field) and were rebuilt stage by stage first; the cascade then re-sealed all 31
  content-identical at every stage.
  **Issue uncovered, owned:** APB5 `table_0018`'s presence rows (`PWDATACHK`..`PBUSERCHK`, `presence_0020`-`0023`)
  carry variant labels `OC`/`N`/`N`/`N` instead of `APB5`..`APB2`: the converter fused one row's `N N` cells and left
  one empty, and the capture remapped the header (`.45`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.i`
  Status: `done` (`2026-10-03`; CODE; found by `.3.d.ii.a`; checklist below; opened `.46`)
  Goal: **a class-scoped absent value binds each signal the check-coverage table lists.** APB5 and AHB5 state *If
  some of the signals covered by a check signal are not present on an interface, then the missing signals are
  assumed to be LOW* (`ihi0024_e` `statement_0388`, `ihi0033_c` `statement_1047`; `ihi0022_h_c` and `ihi0024_d` say
  it too). The class's members are stated by a table whose headers name the relation (`Check signal | Signals
  covered`: APB5 *Table 5-1*, `table_0014`, 15 rows; AHB5 *Table 12-1*, `table_0031`, 17 rows; a cell lists several
  covered signals, `PPROT, PWRITE, PNSE` or `HBURST HMASTLOCK HWRITE HSIZE HNONSEC`). **Reproduce:** the two
  statements bind nothing under `.3.d.ii.a`'s reader (`validate` metric `signal_presence_absent_values` 0 on both).
  **Impact:** the tie-off rule for every parity-covered signal stays an unread assumption. Acceptance: read the
  coverage table into a typed relation (a check signal and the signals it covers), resolve the class phrase to that
  relation when the sentence's words name the table's two headers, bind each covered signal's presence record, and
  measure every chain.
  **Decided `2026-10-03`: a typed relation, and the class resolves through it.** EvidenceIR gains
  `check_signal_records` (`CheckSignalRecord { check_id, check_signal, covered_signals, width, granularity,
  check_enable, table_id }`, `evidence.declaration`), carried unchanged by SemanticIR and IntentIR. The gate is a
  header cell whose words are exactly *signals covered* and a preceding column headed by *signal* or *name* (AXI's
  *Table A16.4*); width, granularity and enable columns are found by their header word and kept verbatim. A name
  cell's trailing footnote letter is dropped (`PSELxCHK b`); a covered cell splits on commas and spaces; a row whose
  covered cell holds anything but identifiers is refused and keeps its position out of the ids. One reading
  (`check_coverage_tables`) feeds both the records and the binder, so they cannot disagree. **The class sentence**
  binds when its absence clause carries every word of the table's two headers (`sentence_names_relation`:
  `check`, `signal`, `signals`, `covered`); each covered signal's presence record takes the literal, citing the
  statement and the table in a new `supporting_table_ids`. A sentence naming the signal itself outranks the class.
  No check signal is lowered: `.isf` has `(^ A B)` but no parity form, and the decision is `.46` (ledger S25).
  **Measured over all 31 chains:** 28 identical at EvidenceIR. AXI: 123 records over `table_0245`-`0249` (166
  distinct covered signals), no absent value (its class sentence states *the default for that signal*). APB5: 14
  records, 15 presence rows take `LOW` citing `statement_0388` and `table_0014` (16 covered; `PADDR`'s matrix row is
  refused for a fused cell, `.45`). AHB5: 16 records, 22 rows take `LOW` citing `statement_1047` and `table_0031`.
  Unread records 441 -> 439, `assumed_by_word` 109 -> 107.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.ii`
  Status: `open` (`2026-10-03`; DESIGN + CODE; found by `.3.d.ii.a`)
  Goal: **a generic channel-prefix name binds the signals it abbreviates.** *Any AxSNOOP bits not driven by an
  interface are assumed to be LOW* (AXI `statement_2207`) names `AxSNOOP`; the presence rows are `AWSNOOP`
  (`presence_0034`) and `ARSNOOP` (`presence_0113`), AXI's `signal_alias_map` is empty, and SpecForge keeps generic
  names literal everywhere (`AxBURST` enums, `AxMMUATST`). **Reproduce:** the statement binds nothing under
  `.3.d.ii.a`'s reader. **Impact:** every rule stated on a generic name reaches neither concrete signal. Acceptance:
  decide the carrier for the document's own prefix convention (the sentence that defines `Ax` as `AW` and `AR`, or
  the matrix rows that share the suffix), bind through it and nothing else (ADR 0006: no prefix list), and measure
  every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b`
  Status: `open` (`2026-09-30`; MEASUREMENT then CODE; found by `.3.d.ii`)
  Goal: **a stated default or equivalence reaches its reader, and a convention is non-contract.** Of `.3.d.ii`'s
  family, 9 state a default of something other than an absent signal, 31 are *treated as* rules and 14 are
  conventions. **Reproduce:** `scripts/measure_unread_intent_records.py`, family `assumed_by_word`. **Impact:** a
  classification rule (*LPIs are always treated as Non-secure Group 1*) is filed as an assumption. Acceptance: route
  each shape to the reader it belongs to (an enum member's default, a classification) or record it non-contract, and
  measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii`
  Status: `open` (`2026-09-30`; MEASUREMENT then CODE; found by `.3.d`)
  Goal: **a stated permission is typed intent, and an unread obligation reaches its reader.** 178 unread behaviors
  mix permissions (*A MASTER MAY use End-Of-Burst to indicate a single access*), obligations no reader reads (*The
  host shall schedule isochronous transactions such that …*) and descriptions; 17 carry EvidenceIR's
  `timing_constraint` class though none states a time (Wishbone's *MASTER and SLAVE interfaces MAY be designed …*).
  **Reproduce:** family `permission_or_description`. **Impact:** optional behavior has no carrier, and obligations
  stay untyped. Acceptance: split the family by modality, route each obligation class to the reader that should take
  it, decide a carrier for a permission, find why those 17 are classed as timing, and measure every chain.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a` (CODE) — DONE `2026-10-03`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_unread_intent_records.py`: 443 unread records, family
  `assumed_by_word` 111, among them AXI `statement_1089` (*BRESP … not present and assumed to be 0b000 (OKAY)*),
  `statement_1118` (RRESP) and `statement_2207` (*Any AxSNOOP bits not driven … assumed to be LOW*), APB5
  `statement_0388` and AHB5 `statement_1047` (*the missing signals are assumed to be LOW*). `validate` on AXI's
  EvidenceIR: `signal_presence_records` 306, no record carries a value; SemanticIR and IntentIR carry no presence
  record at all.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `SignalPresenceRecord` had no field for a
  value, `signal_presence_surface` read tables only and never the statements, and `crates/specforge/src/ir/semantic.rs`
  / `intent.rs` listed `signal_presence_records` in no rule-field table, so the surface stopped at EvidenceIR
  (`.3.d.ii`'s finding). RED, with the file restored byte-identically after each (5 mutations): dropping the
  absence predicate, the sentence-closure rule, the one-name rule or the disagreement rule fails
  `a_signals_stated_absent_value_rides_its_presence_record`, `a_qualified_or_prose_absent_value_is_not_read`,
  `a_class_of_signals_or_two_signals_binds_nothing` or
  `repeated_statements_are_cited_and_disagreeing_ones_bind_nothing`.
- [x] **ADDRESSED (verified)** — Evidence dry-run of all 31 chains with the HEAD binary (`513610ddc`'s build) and the new
  one: 30 identical; AXI moves only in `signal_presence_records`, `BRESP` and `RRESP` gaining `absent_value` `0b000`
  with their statements. AXI rebuilt stage by stage (evidence, semantic, intent, each validated once, the adapter
  last): `signal_presence_absent_values` 2; SemanticIR and IntentIR `signal_presence_records` 0 -> 306; adapter
  residuals 55 -> 57 (two `isf_protocol_signal_absent_value_*`), lowering status and ISF text unchanged.
  APB5 and AHB5 rebuilt from semantic down: `signal_presence_records` 0 -> 24 and 0 -> 40, no other section moved.
  Unread records 443 -> 441, `assumed_by_word` 111 -> 109.
- [x] **NO REGRESSION** — The new rule fields re-seal every chain: after the three hand rebuilds,
  `rebuild_stage_cascade.sh --write` rebuilt all 31 content-identical at every stage; `check_chain_currency.sh` 31 of
  31 current at evidence, semantic, intent and isf-adapter. WIRE-BASED-100:
  all 11 seeds identical to the reports of `513610ddc`'s binary (temp paths masked; `seed_swd`'s constraint 1.000). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets --
  -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,817 passed, 0 failed.
  Claim-family inventory 178 -> 180 rules (one carried field per stage); `flow_census.json` rolled forward
  (+7 functions, +44 decision sites, +50 helper edges, +6 macros); boundary counts unchanged, since the binder is a helper of the registered `signal_presence_surface`
  producer and no new root or grammar producer was registered.
- [x] **GENERICITY (ADR 0006)** — English absence predicates and the lead *assumed to be*; a literal's shape (a
  number, a capitalised word, *asserted*); the document's own presence-record names; no signal, property, document or
  protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (the `.12b` presence section extended),
  `pipeline/semanticir.md`, `pipeline/intentir.md`, `pipeline/isf-adapter.md`, `reference/intentir-contract.md`,
  `source-to-intent-contract.md`, `reference/representation-gaps.md` (*Signal absent by default*),
  `quality/validation.md` (the fifth presence metric); `docs/gaps/fsmgen-ladder.md` (F7); KM
  `a-signals-stated-absent-value-rides-its-presence-record` (new), `signal-presence-matrix-capture` (amended);
  category contract (`interface_inventory_and_shape` names the surface) and census script carry it. No production
  rule was deleted or replaced.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.i` (CODE) — DONE `2026-10-03`

- [x] **REPRODUCE / MEASURE** — `validate` on APB5's and AHB5's EvidenceIR after `.3.d.ii.a`: `signal_presence_absent_values`
  0 on both, while `statement_0388` and `statement_1047` (*the missing signals are assumed to be LOW*) sit in the unread
  `assumed_by_word` family (109 records). No IR field held a check signal's covered signals: APB5 `table_0014`
  (14 rows), AHB5 `table_0031` (16), AXI `table_0245`-`0249` (123) reached only the declaration reader.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `stated_absent_values` required exactly one
  presence-record name in the statement, and nothing read the `Check signal | Signals covered` relation that defines
  the class. RED, with the file restored byte-identically after each (5 mutations): dropping the header-word
  match, the covered-cell identifier refusal, the name-cell mark rule or the specific-over-class precedence fails
  `the_missing_signals_covered_by_a_check_signal_take_the_stated_value`,
  `a_name_headed_check_table_is_read_and_a_garbled_row_is_refused`,
  `a_check_coverage_table_declares_each_check_signal_and_what_it_covers` or
  `a_sentence_naming_the_signal_outranks_the_class`.
- [x] **ADDRESSED (verified)** — Evidence dry-run of all 31 chains against the persisted (`d7c0def32`-current) artifacts:
  28 identical; AXI moves only in `check_signal_records` (0 -> 123), APB5 in `check_signal_records` (0 -> 14) and
  `signal_presence_records` (15 rows gain `LOW`), AHB5 likewise (0 -> 16; 22 rows). The three rebuilt stage by stage,
  each validated once, the adapter last: SemanticIR and IntentIR carry the records (`check_signal_records` 123, 14 and 16 at both stages);
  adapter residuals and ISF texts AXI unchanged at 57 and its ISF text byte-identical; APB5 22 -> 37 and AHB5 15 -> 37 (one `isf_protocol_signal_absent_value_*` per newly valued row), both ISF texts byte-identical. Unread records 441 -> 439, `assumed_by_word` 109 -> 107.
- [x] **NO REGRESSION** — The new evidence rule field re-seals every chain: after the three hand rebuilds,
  `rebuild_stage_cascade.sh --write` rebuilt all 31 content-identical at every stage; `check_chain_currency.sh`
  31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical to the reports of `d7c0def32`'s binary (temp paths masked; `seed_swd`'s constraint 1.000). `kg-bench` 158/158. `cargo fmt --check`
  clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib
  --exclude specforge-production-graph` 474 / 170 / 1,823 passed, 0 failed. Claim-family inventory 180 -> 183 rules;
  `flow_census.json` rolled forward (+8 functions, +29 decision sites, +127 helper edges, +4 macros); `check_signal_surface` is registered as a producer of the
  `evidence.declaration` family, so `rule_roots` 124 -> 125 and `grammar_declassifiers` 65 -> 66 move by one for that one cause.
- [x] **GENERICITY (ADR 0006)** — header words (*signals covered*, *signal*, *name*, *width*, *granularity*, *enable*),
  identifier shape, a footnote letter's shape, and the sentence's own words against the table's own headers; no
  document, signal, table or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (the `.12b` presence section extended),
  `pipeline/semanticir.md`, `pipeline/intentir.md`, `pipeline/isf-adapter.md`, `reference/intentir-contract.md`,
  `source-to-intent-contract.md`, `reference/representation-gaps.md` (*Signal absent by default* amended, *Parity check
  signals* added), `quality/validation.md`; `docs/gaps/specforge.md` (S25); KM
  `a-check-coverage-table-binds-the-class-of-covered-signals` (new), `a-signals-stated-absent-value-rides-its-presence-record`
  (amended); category contract and inventories carry the surface. No production rule was deleted or replaced.
