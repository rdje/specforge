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
  Status: `done` (`2026-10-20`; DESIGN + CODE; found by `.3.d`; checklist below; opened `.52`)
  Goal: **a software agent's obligation is recorded as software intent, not as an untyped behavior.** 47 unread
  records oblige or permit a software agent (*The Connection Manager may enable CLx 50ms after TMU has been
  activated*; *The firmware is responsible for creating the stage 2 translation tables for the untrusted OS*); 34 are
  in the USB4 connection-manager guide. The category
  contract calls such content non-applicable to executable IntentIR and requires an explicit disposition.
  **Reproduce:** `scripts/measure_unread_intent_records.py`, family `software_agent`. **Impact:** a document's
  software procedure is indistinguishable from unread hardware behavior. Acceptance: decide how an agent is known to
  be software from the document (no IR field says so today), give the obligation a typed non-executable disposition,
  and measure every chain. `.3.d.ii.b.iii` (`2026-10-19`) routes one more record here: USB4 inter-domain's *If
  bonding does not complete, assume single-Lane operation*, a step of the Local Domain's procedure.
  **Decided `2026-10-20`: the document's own words make an agent software, and every statement it is the subject of
  is an EvidenceIR `software_intent` record, carried unchanged by SemanticIR and IntentIR, with one adapter residual
  per document as the category contract's explicit non-applicable disposition.** Three bases, each the document's
  words (`SoftwareAgentBasis`): an English software noun (*software*, *firmware*, *operating system*, *hypervisor*,
  *device driver*, *debugger*; not *driver* alone, which names an I/O pad, nor *user*), a glossary row defining the
  term as software (*| Hypervisor | Software entity that controls virtualization. |*), and *X can be implemented in
  software* (*A Connection Manager can be implemented in software or firmware*). The census's own agent list (`.3.d`)
  is a measurement vocabulary and stays one: it named I2C's *output drivers*, USB 3.2's *Re-driver*, a debug-host
  *user* and *Debug software access*. `SoftwareIntentRecord { intent_id, agent, software_name, basis,
  basis_statement_ids, modality, supporting_statement_ids }`, family `evidence.normative`; modality is the word after
  the subject (obligation, recommendation, permission, capability, description). **Probed first**
  (`.project-data/tmp/erg-3di/probe.py`, then the token-for-token port `grammar.py`): 25 of 25 sampled subjects are
  software. **Measured over all 31 chains** (evidence dry-run against `7a262314d`'s): 20 identical, 11 gain only
  `software_intent`, 342 records (USB4 connection-manager guide 232, ADIv6 44, RISC-V debug 16, IOMMU 15, external
  debug guide 13; 185 descriptions, 61 obligations, 54 capabilities, 36 permissions, 6 recommendations; bases 232
  implemented in software, 104 software nouns, 6 glossary); the Python port yields the same 342. Rebuilt stage by stage:
  only `software_intent` moves at each stage, each of the 11 adapters gains one `isf_software_intent_not_applicable`
  residual, every `.isf` text byte-identical; the new rule field re-sealed every chain (`rebuild_stage_cascade.sh
  --write`, 31 content-identical). Unread IntentIR records 145 -> 138: `software_agent` 12 -> 5.
  **Declared residuals:** of the census's 12 `software_agent` records, 7 are now read; 4 name hardware or a human (the
  list's false positives above), and 1 is software the grammar does not reach: the inter-domain spec's *Higher level
  software running on each Host usually requires unique MAC Addresses* (a participle after the noun). The same spec
  names its Connection Manager only as *the Local Connection Manager software*, a compound that also names hardware's
  software (*Host Controller software*), so its 9 Connection Manager permissions carry no `software_intent`. **Opened `.52`:** the readers that ran before this one minted 137
  IntentIR records from these statements (102 temporal invariants, 25 conditional rules, 10 multi-statement records).

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
  Status: `done` (`2026-10-03`; DESIGN + CODE; found by `.3.d.ii.a`; checklist below; opened `.47`)
  Goal: **a generic channel-prefix name binds the signals it abbreviates.** *Any AxSNOOP bits not driven by an
  interface are assumed to be LOW* (AXI `statement_2207`) names `AxSNOOP`; the presence rows are `AWSNOOP`
  (`presence_0034`) and `ARSNOOP` (`presence_0113`), AXI's `signal_alias_map` is empty, and SpecForge keeps generic
  names literal everywhere (`AxBURST` enums, `AxMMUATST`). **Reproduce:** the statement binds nothing under
  `.3.d.ii.a`'s reader. **Impact:** every rule stated on a generic name reaches neither concrete signal. Acceptance:
  decide the carrier for the document's own prefix convention (the sentence that defines `Ax` as `AW` and `AR`, or
  the matrix rows that share the suffix), bind through it and nothing else (ADR 0006: no prefix list), and measure
  every chain.
  **Decided `2026-10-03`: the carrier is the document's own definition, as a typed record.** EvidenceIR gains
  `collective_signal_names` (`CollectiveSignalNameRecord { collective_id, collective_name, member_names,
  supporting_statement_ids }`, `evidence.declaration`), carried unchanged by SemanticIR and IntentIR, read in two
  shapes. **A sentence that names the members:** an identifier with one run of lowercase letters followed by
  uppercase letters (`Ax`, `VAx`, `Cxxx`), a defining verb (*indicates*, *refers to*, *denotes*, *means*,
  *represents*), then two or more identifiers that are the name with its run replaced by uppercase letters (*AxSIZE
  indicates AWSIZE and ARSIZE*; *the term VAxQOSACCEPT refers collectively to the VAWQOSACCEPT and VARQOSACCEPT
  signals*; ADIv6's *CxxxPWRUPREQ refers to either CSYSPWRUPREQ or CDBGPWRUPREQ*); the members need not be declared.
  **A stated rule:** a statement that says *collective* and carries such an example (*Lowercase x - At the second
  letter of a signal name denotes a collective term for both Read and Write. For example, AxCACHE refers to both
  the ARCACHE and AWCACHE signals*) states that every name with the same run at the same position stands for the
  same replacements; each such name the document uses is derived, in the example's member order, only when all its
  members are known signals (declared, or a presence-matrix row), citing the rule. No rule, no derivation (USB 3.2's
  `RxD`, `TxEQ`). **Binding:** the absent-value reader resolves a statement naming no row but exactly one collective
  name to each member with a row, recording the name as `collective_name` on the value; the closest naming wins
  (the signal itself, then a collective name, then a class). **The shape's trap**, found on Wishbone: *When the gap
  indicates a wait state, the symbols '-WSM-' or '-WSS-' …* read as `When` -> `WSM`, `WSS` until a name was required
  to carry an uppercase letter after its run.
  **Measured over all 31 chains:** evidence dry-run against the persisted artifacts, 29 identical. AXI: 37 records
  (6 from sentences, 31 from the rule; `AxCACHE` cites both `statement_0446` and `statement_1198`), and `AWSNOOP`
  (`presence_0034`) and `ARSNOOP` (`presence_0113`) take `LOW` from `statement_2207` through `AxSNOOP`. ADIv6: 2
  records (`CxxxPWRUPREQ`, `CxxxPWRUPACK`), no absent value. Unread records 439 -> 438, `assumed_by_word`
  107 -> 106. Every other rule stated on a collective name (245 AXI statements) still reaches no member:
  `.47`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b`
  Status: `done` (`2026-10-03`; MEASUREMENT, the code re-scoped to three children `.3.d.ii.b.i`-`.3.d.ii.b.iii`; found by
  `.3.d.ii`)
  Goal: **a stated default or equivalence reaches its reader, and a convention is non-contract.** Of `.3.d.ii`'s
  family, 9 state a default of something other than an absent signal, 31 are *treated as* rules and 14 are
  conventions. **Reproduce:** `scripts/measure_unread_intent_records.py`, family `assumed_by_word`. **Impact:** a
  classification rule (*LPIs are always treated as Non-secure Group 1*) is filed as an assumption. Acceptance: route
  each shape to the reader it belongs to (an enum member's default, a classification) or record it non-contract, and
  measure every chain.
  **Measured `2026-10-03`** (`python3 scripts/measure_unread_intent_records.py`, which now gives every
  `assumed_by_word` record one of six shapes, first match wins; `--json` carries it as `shape`). After `.3.d.ii.a`,
  `.a.i` and `.a.ii` bound the five absent values, **106** remain, and every one of them reads as one of:
  | shape | records | what they say | disposition |
  | --- | ---: | --- | --- |
  | `convention` | 6 | *This guide assumes that you are familiar with …*; *In this example, the following conditions are assumed:*; *Throughout this document we refer to hartsel* | non-contract: the document, example or reader is the subject |
  | `diagram_convention` | 10 | *Shaded bus and signal areas are undefined, so the bus or signal can assume any value*; *The components used in timing diagrams are explained in …* | non-contract: a timing-diagram key, stated once per document |
  | `narrative` | 30 | *This limitation assumes that there is a maximum of 3 logic levels*; *the debugger must not assume that the TAR wraps*; *To keep the examples readable, they all assume that everything succeeds* | non-contract: an example, a modelling premise or an analysis assumption; a few are obligations on software (*must not assume*), which `.3.d.i` owns |
  | `treated_as` | 31 | *LPIs are always treated as Non-secure Group 1 interrupts*; *If the instruction being stepped over would normally stall the hart, then instead the instruction is treated as a nop*; *ClearPortFeature(PORT_RESET) … may be treated as a no-operation request by hubs* | an equivalence rule the hardware applies, often under a condition: typed, not an assumption (`.3.d.ii.b.ii`) |
  | `stated_default` | 25 | AXI's *If AxDOMAIN is omitted from the interface, CMO transactions are assumed to be Non-shareable* and three more domain defaults; *If no flow is indicated, a Stall flow is assumed*; USB4's *If bonding does not complete, assume single-Lane operation*; IOMMU's *the DTF value assumed for reporting such faults is 0*; against physical-link analysis premises (*RJ and BUJ are assumed as Gaussian terms*) and table notes (*The latencies shown assume the memory access hits in the L1*) | the genuine defaults reach a reader (`.3.d.ii.b.iii`); the analysis premises and table notes are non-contract (`.3.d.ii.b.i`) |
  | `other` | 4 | *60% Cu Assumer*; *an abstracted view of a notional OpenCAPI solution*; IPMI's *abstracted, message-based interface* | non-contract: a fragment and three uses of *abstracted* as an adjective |
  **Decided:** the root cause is one classifier. `classify_statement` (`crates/specforge/src/ir/evidence.rs`) files any
  sentence holding *assume*, *assuming*, *modeled as*, *abstracted* or *treated as* as `ExplicitAbstraction` before any
  other class is tried, SemanticIR lifts every such statement to an abstraction, and IntentIR makes each an
  assumption. So a hardware equivalence rule and a guide's prerequisite share one carrier that neither belongs to. The
  three children divide the remedy by what each shape needs: a class that stops the non-contract shapes becoming
  assumptions (`.3.d.ii.b.i`), a carrier for the equivalence rules (`.3.d.ii.b.ii`), and the reader a genuine
  default belongs to (`.3.d.ii.b.iii`). No code change here, so no acceptance checklist applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.i`
  Status: `done` (`2026-10-03`; CODE; found by `.3.d.ii.b`; checklist below)
  Goal: **a convention, a diagram key, an example or an analysis premise is not an abstraction.** 50 of the 106
  `assumed_by_word` records (6 conventions, 10 diagram conventions, 30 narratives, 4 others) and the analysis premises
  among the 25 stated defaults become IntentIR assumptions only because their sentence holds an *assume* word.
  **Reproduce:** `python3 scripts/measure_unread_intent_records.py`, family `assumed_by_word`, shapes `convention`,
  `diagram_convention`, `narrative`, `other`. **Impact:** a document's stated assumptions are diluted by prose that
  states none. Acceptance: decide the class such a sentence takes instead of `ExplicitAbstraction` (its subject is
  the guide, document, example, reader, figure or diagram, or the sentence states a premise of an analysis), keep the
  abstraction class for a sentence that states a modelling abstraction, measure what every chain's SemanticIR and
  IntentIR lose and whether any other reader now takes the reclassified sentences, and keep all 11 seeds.
  **Decided `2026-10-03`: a dedicated class, `apparatus`, that no reader consumes.** `classify_statement`'s first
  arm is now `abstraction_class`: the words are whole words (*Assumer* is none), *abstracted* counts only as a verb,
  a sentence saying *treated as* is an abstraction whatever else it says, and the three measured non-contract shapes
  (`is_non_contract_assumption`: the document, a figure or the reader as subject; a timing-diagram key; the premise
  of an example or an analysis) are `StatementClass::Apparatus`. *Assumed that* is deliberately no premise marker,
  so ADIv6's *it can be assumed that no write occurred* stays an abstraction. **Two cheaper remedies were measured
  and refused.** Letting the sentence fall through sent 33 of 83 into the rule, constraint and timing classes and the
  rule reader minted conditional rules from six of them, a diagram key among them (*Variations, when they occur,
  have clear labels. You must not assume …*); filing it as `SourceFact` kept every other record byte-identical except
  Wishbone's, where the prose hint reader minted `CLK_I` -> `handshake_valid_like` from an example circuit's *assumes
  that output data is valid at the rising [CLK_I] edge*. `is_invariant_like` excludes the new class beside
  `ExplicitAbstraction`; class-filtered readers never select it.
  **Measured over all 31 chains:** 83 statements in 22 chains move `explicit_abstraction` -> `apparatus`, 6 of
  them sentences that used an abstraction word only as a fragment or an adjective (*Assumer*, *an abstracted
  view*); every other evidence section is byte-identical (dry-run against the persisted artifacts). A first cut let
  those 6 fall through as source facts, and the invariant reader minted two constraints from *Figure 2-1 provides
  an abstracted view of a notional OpenCAPI solution*; a sentence that only mentions the word is apparatus too. Unread records 438 -> 364,
  `assumed_by_word` 106 -> 53; IntentIR assumptions 78 fewer across the 22 chains, no other IntentIR section moved, adapters and ISF texts byte-identical.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.ii`
  Status: `done` (`2026-10-19`; DESIGN then CODE; found by `.3.d.ii.b`; checklist below)
  Goal: **an equivalence rule (X is treated as Y) is typed.** 31 records state that under a condition one thing is
  treated as another: an interrupt class (*LPIs are always treated as Non-secure Group 1 interrupts*), an instruction
  (*treated as a nop*), a request (*may be treated as a no-operation request by hubs*), a register field (*CSW.DbgSwEnable
  must be treated as one*), a value (*DEVARCH and DEVTYPE are treated as having a value of 0*). **Reproduce:** shape
  `treated_as`. **Impact:** a conditional rule of the hardware is filed as an assumption no reader lowers. Acceptance:
  decide the carrier (a conditional rule whose consequent is an equivalence, or a typed equivalence record with
  subject, equivalent and condition), read the 31 into it citing their statements, keep the ones whose subject is
  software with `.3.d.i`, and measure every chain.
  **Decided `2026-10-19`: a typed `EquivalenceRuleRecord`, not a conditional rule.** `ConditionalRuleRecord` is
  signal-centric (an antecedent, a consequent signal, an action), and most equivalences have neither a condition nor a
  signal, so filing them there would tell every consumer something the sentence does not say. EvidenceIR gains
  `equivalence_rules` (`EquivalenceRuleRecord { equivalence_id, subject, equivalent, condition, modality,
  supporting_statement_ids }`, family `evidence.normative`), carried unchanged by SemanticIR and IntentIR; every part
  is as written and the sentence keeps its `explicit_abstraction` class. **The grammar** was probed on all 39 *treated
  as* uses before any code (`stated_equivalences`): auxiliaries with a copula before the phrase, the subject from the
  sentence's last clause break, a condition when the text before that break opens like one, the equivalent to its
  clause's end, the modality from the auxiliaries. A coordinated predicate ends its subject at the subject's own verb,
  a parenthesis holds its own subject and a reason clause asserts its equivalence; a contrasted clause (*otherwise*,
  *else*, *in which case*), a hypothetical, a coordinated predicate without a verb and a use with no copula are
  refused (5 of 39). None of the 31 has a software subject, so `.3.d.i` takes none. The adapter keeps one
  `isf_protocol_equivalence_rule_<id>` residual per record, and `validate` counts the records on all three stages.
  **Measured over all 31 chains:** evidence dry-run against `0e69b1170`'s output, 21 identical; in 10 only
  `equivalence_rules` appears — 34 records (USB 3.2 10, GICv3 6, IOMMU 4, ADIv6 4, RISC-V debug 3, AXI 3, SMMU guide,
  TMC, CHI C2C and the USB4 connection-manager guide 1 each). Rebuilt stage by stage, the 10 move only that section at
  SemanticIR and IntentIR and gain one adapter residual per record; both emitted `.isf` texts byte-identical. The new
  rule field moves each stage's rule fields, so every chain re-sealed: `rebuild_stage_cascade.sh --write` rebuilt all 31 at every stage,
  31 content-identical and validated, one seal per stage after. Unread records 345 -> 315 (27 of the 31 *treated as*
  assumptions and 3 behaviors citing the same statements), `assumed_by_word` 37 -> 10, `treated_as` 31 -> 4: the four
  refused contrast and hypothetical sentences (IOMMU's *otherwise* and *else*, USB 3.2's *in which case* and *if …*).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii`
  Status: `done` (`2026-10-19`; MEASUREMENT, the code re-scoped to three children `.3.d.iii.a`-`.3.d.iii.c`; found by `.3.d`)
  Goal: **a stated permission is typed intent, and an unread obligation reaches its reader.** 178 unread behaviors
  mix permissions (*A MASTER MAY use End-Of-Burst to indicate a single access*), obligations no reader reads (*The
  host shall schedule isochronous transactions such that …*) and descriptions; 17 carry EvidenceIR's
  `timing_constraint` class though none states a time (Wishbone's *MASTER and SLAVE interfaces MAY be designed …*).
  **Reproduce:** family `permission_or_description`. **Impact:** optional behavior has no carrier, and obligations
  stay untyped. Acceptance: split the family by modality, route each obligation class to the reader that should take
  it, decide a carrier for a permission, find why those 17 are classed as timing, and measure every chain.
  **Measured `2026-10-19`** (`python3 scripts/measure_unread_intent_records.py`, which now gives every
  `permission_or_description` record one of five modalities, first match wins; `--json` carries it as `modality`).
  **176** remain, and the family is a permission family: permission 154 (*may*, *permitted*, *allowed*, *optional*;
  USB 3.2 51, IOMMU 35, Wishbone 20, the USB4 connection-manager guide 18, I2C 7), obligation 8, description 7,
  recommendation 4, capability 3. **The 17 timing-class records are one symptom of a corpus-wide defect.**
  `classify_statement`'s timing arm matches substrings: its first list holds ` cycle`, `cycles` and `at least`, its
  second `cycle`, `ns`, `ps`, `time` and `edge`, so *MASTER and SLAVE interfaces MAY be designed so that they do not
  support the BLOCK cycles* satisfies both, as do *at least 16 bits of PASID must be supported* and a register named
  `iohpmcycles`. Over all 31 chains, **645 of the 813** `timing_constraint` statements (79%) carry no time expression
  (no number with a time or cycle unit, no setup, hold or propagation time, no clock edge, period or latency) —
  Wishbone 268, USB 3.2 72, ADIv6 62, AXI 60. **The 8 obligations** are 7 statements `.3.d.ii.b.i`'s premise phrase
  *assume that* filed as apparatus although each states a hardware or software obligation (*A device shall not assume
  that transactions occur at the same location within each service interval*; *the debugger must not assume that the
  TAR wraps*) and one refused equivalence. **Decided:** three children, ordered by what each corrupts: the timing class
  first, since 645 statements wear a class they do not state (`.3.d.iii.b`); then a carrier for a permission
  (`.3.d.iii.a`); then the swallowed obligations and the 14-record remainder (`.3.d.iii.c`). No code change here, so no
  acceptance checklist applies.

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

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.a.ii` (CODE) — DONE `2026-10-03`

- [x] **REPRODUCE / MEASURE** — After `.3.d.ii.a.i`, AXI's `statement_2207` (*Any AxSNOOP bits not driven by an interface
  are assumed to be LOW*) sits in the unread `assumed_by_word` family (107 records): `validate` on AXI's EvidenceIR
  reports `signal_presence_absent_values` 2 (`BRESP`, `RRESP`), none on `AWSNOOP` (`presence_0034`) or `ARSNOOP`
  (`presence_0113`); AXI's `signal_alias_map` is empty, and no IR field held `AxCACHE` -> `ARCACHE`, `AWCACHE` though
  the document states it (`statement_0446`, `statement_1198`) and uses a collective name in 245 statements.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `stated_absent_values` matched identifiers
  against presence-row names only, and nothing read the document's own definition of a collective name, in either
  shape. RED, with the file restored byte-identically after each (5 mutations): dropping the known-member requirement
  or the rule's position check fails `a_stated_rule_derives_every_name_of_that_shape_whose_members_are_known`;
  letting a collective name outrank the signal itself, or ignoring collective names in the binder, fails
  `an_absent_value_stated_on_a_collective_name_reaches_each_member`; dropping the uppercase-after-the-run rule fails
  `a_capitalised_word_is_not_a_collective_name_but_a_longer_run_is`.
- [x] **ADDRESSED (verified)** — Evidence dry-run of all 31 chains against the persisted (`9c43a142e`-current) artifacts:
  29 identical; AXI moves in `collective_signal_names` (0 -> 37) and `signal_presence_records` (`AWSNOOP`, `ARSNOOP`
  gain `LOW` with `collective_name` `AxSNOOP`); ADIv6 in `collective_signal_names` (0 -> 2). Both rebuilt stage by
  stage, each validated once, the adapter last: `collective_signal_names` 37 and 2 at both stages; AXI's adapter residuals 57 -> 59 (two `isf_protocol_signal_absent_value_*`) with its ISF text byte-identical, ADIv6's adapter unchanged. Unread records 439 -> 438, `assumed_by_word`
  107 -> 106.
- [x] **NO REGRESSION** — The new evidence rule field re-seals every chain: after the two hand rebuilds,
  `rebuild_stage_cascade.sh --write` rebuilt all 31 content-identical at every stage; `check_chain_currency.sh`
  31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical to the reports of `9c43a142e`'s binary (temp paths masked; `seed_swd`'s constraint 1.000). `kg-bench` 158/158. `cargo fmt --check`
  clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib
  --exclude specforge-production-graph` 474 / 170 / 1,828 passed, 0 failed. Claim-family inventory 183 -> 186 rules;
  `flow_census.json` rolled forward (+4 functions, +41 decision sites, +59 helper edges, +5 macros); `collective_signal_name_surface` is registered as a producer of the
  `evidence.declaration` family, so `rule_roots` 125 -> 126 and `grammar_declassifiers` 66 -> 67 move by one for that one cause.
- [x] **GENERICITY (ADR 0006)** — the shape of a name (one lowercase run, uppercase after it), English defining verbs
  and the word *collective*, the document's own example and its declared or listed signals; no prefix, signal,
  document or protocol list.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (the `.12b` presence section extended),
  `pipeline/semanticir.md`, `pipeline/intentir.md`, `pipeline/isf-adapter.md`, `reference/intentir-contract.md`,
  `source-to-intent-contract.md`, `reference/representation-gaps.md` (*Signal absent by default*),
  `quality/validation.md`; KM `a-document-stated-collective-name-stands-for-its-members` (new),
  `a-signals-stated-absent-value-rides-its-presence-record` (amended); category contract and inventories carry the
  surface. No production rule was deleted or replaced.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.i` (CODE) — DONE `2026-10-03`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_unread_intent_records.py`: `assumed_by_word` 106 with shapes
  `convention` 6, `diagram_convention` 10, `narrative` 30, `other` 4 — every one an IntentIR assumption because its
  sentence holds an *assume* word; over all 180 `explicit_abstraction` statements on the 31 chains, 78 take those
  shapes. `validate` on APB5's EvidenceIR counts its two timing-diagram keys and parity-budget premise as
  `explicit_abstraction`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `classify_statement` tested a substring list
  (`assume`, `assuming`, `modeled as`, `abstracted`, `treated as`) before every other class and returned
  `ExplicitAbstraction`; `semantic.rs` `build_abstractions` lifts that class and `intent.rs` `build_assumptions`
  carries it. RED, with the file restored byte-identically after each (6 mutations): dropping the diagram-key, premise
  or reader-subject shape fails `a_timing_diagram_key_is_not_an_abstraction`,
  `a_premise_of_an_example_or_analysis_is_not_an_abstraction` or
  `a_sentence_about_the_document_or_reader_is_not_an_abstraction`; dropping the *treated as* override fails
  `a_hardware_rule_stated_with_assume_or_treated_as_is_an_abstraction`; matching the words as substrings, or
  treating a fragment or adjective mention as ordinary prose, fails
  `an_abstraction_word_counts_only_as_a_whole_word_or_a_verb`.
- [x] **ADDRESSED (verified)** — Evidence dry-run of all 31 chains against the persisted (`121da63e7`-current) artifacts:
  83 statements in 22 chains move `explicit_abstraction` -> `apparatus`, and no other section of any chain moves (the two refused remedies moved `conditional_rules` in six chains and `signal_semantic_hints` in one).
  The changed chains rebuilt stage by stage, each validated once, the adapter last: IntentIR assumptions 78 fewer across the 22 chains, no other IntentIR section moved; adapters and ISF texts byte-identical. Unread records
  438 -> 364, `assumed_by_word` 106 -> 53.
- [x] **NO REGRESSION** — The classifier is outside the EvidenceIR seal closure: `check_chain_currency.sh` 31 of
  31 current at every stage after the rebuilds, no cascade needed. WIRE-BASED-100: all 11 seeds identical to the reports of `121da63e7`'s binary (temp paths masked; `seed_swd`'s constraint 1.000), the HEAD side scored on its own binary's EvidenceIR for the six measurable seed documents, put in place and restored byte-identically. `kg-bench` 158/158.
  `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test
  --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,833 passed, 0 failed. `flow_census.json` rolled
  forward (+2 functions, +8 decision sites, +9 helper edges); boundary counts unchanged, since no producer or field was added.
- [x] **GENERICITY (ADR 0006)** — English abstraction verbs as whole words, the nouns a sentence takes when it speaks about
  the document, the words of a diagram key, and the lead words of a premise; no document, signal or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*Which assume-sentences are abstractions, and which are
  apparatus*), `pipeline/intentir.md` (the unread-assumption passage); KM
  `an-assume-sentence-about-the-document-is-apparatus` (new), `assume-sentences-fall-into-six-shapes` (amended). No
  production rule was deleted or replaced: the abstraction arm is narrowed, and the book described it nowhere by
  name.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.ii` (DESIGN then CODE) — DONE `2026-10-19`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_unread_intent_records.py`: `assumed_by_word` 37 with shape
  `treated_as` 31, each an IntentIR assumption no typed record cites; 39 *treated as* uses on the 31 chains.
- [x] **ROOT CAUSE (WHY + WHERE)** — no carrier: `crates/specforge/src/ir/evidence.rs` had no surface for an equivalence and
  `ConditionalRuleRecord` (`crates/specforge/src/ir/source.rs`) needs a signal-centric antecedent and consequent. RED, the
  file restored byte-identically after each (8 mutations, each failing its named test): no contrast refusal, no
  hypothetical refusal, no copula required (`a_contrasted_or_hypothetical_equivalence_is_refused`); no coordinated cut,
  no parenthesis subject, no reason clause (`a_coordinated_parenthesised_or_reason_clause_finds_its_own_subject`); no
  merge, table rows read (`the_surface_merges_repeats_and_skips_rows_and_apparatus`).
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 21 identical, 10 gain `equivalence_rules` (34 records)
  and nothing else; rebuilt stage by stage, only that section moves downstream, adapter residuals +34, `.isf`
  byte-identical. Unread records 345 -> 315, `treated_as` 31 -> 4.
- [x] **NO REGRESSION** — after the cascade, `check_chain_currency.sh` 31 of 31 current at every stage. WIRE-BASED-100:
  all 11 seeds byte-identical to `0e69b1170`'s reports (temp paths masked), each side on its own binary. `kg-bench`
  158/158 (`fixtures_failed: 0`). `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D
  warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 171 / 1,851 passed, 0
  failed (the rule-field and inventory counts each move by one per stage, 186 -> 189 rules). `flow_census.json` rolled
  forward (+2 functions, +52 decision sites, +64 helper edges, +4 macros); boundary `rule_roots` 126 -> 127 and
  `grammar_declassifiers` 67 -> 68 for the one registered producer, `equivalence_rule_surface`, as each earlier new
  field's producer moved them.
- [x] **GENERICITY (ADR 0006)** — English auxiliaries, copulas, modals, condition openers, *otherwise*/*else*/*in which
  case*, *since*/*because*, and punctuation; no document, signal or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*An equivalence rule is a typed record*),
  `pipeline/semanticir.md`, `pipeline/intentir.md`, `pipeline/isf-adapter.md`, `quality/validation.md`,
  `reference/intentir-contract.md`, `source-to-intent-contract.md`; KM `an-equivalence-rule-is-a-typed-record` (new),
  `assume-sentences-fall-into-six-shapes` (amended). No production rule was deleted or replaced.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.i` (DESIGN + CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_unread_intent_records.py`: family `software_agent` 12; the
  basis probe and the subject probe (`.project-data/tmp/erg-3di/probe.py`, 256 statements, 25 of 25 sampled correct)
  before any code.
- [x] **ROOT CAUSE (WHY + WHERE)** — no IR field said an agent is software, so nothing told a software procedure from
  hardware behavior. New `software_intent` in `crates/specforge/src/ir/evidence.rs` (`software_intent_surface`,
  `software_subject`, `software_agent`, `glossary_software_name`, `implemented_in_software_names`,
  `without_lead_markers`), carried in `semantic.rs` and `intent.rs`, residual in `adapters.rs`, counted in
  `commands/validate.rs`. RED, the file restored byte-identically after each (12 mutations, each failing its named
  test): the glossary follower, *implemented in software*, condition-word and preposition refusals off
  (`a_software_noun_that_is_not_the_subject_or_a_name_the_document_never_makes_software_is_no_intent`); the condition
  clause, relative clause, note label and list number kept, and *can* read as permission
  (`an_agent_the_document_makes_software_is_read_with_its_basis_and_modality`); the apparatus and table skips and the
  merge off (`the_surface_skips_tables_and_apparatus_and_merges_a_repeated_statement`). The preposition mutant first
  survived because the verb list cut *External accesses from a debugger* at *accesses*; a subject the preposition
  alone refuses (*Data from software is ignored*) killed it.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 20 identical, 11 gain only `software_intent`, 342
  records, equal to the Python port's 342. The 11 chains rebuilt stage by stage, each artifact validated once,
  the adapter last; only `software_intent` moves at each stage, one residual per adapter, `.isf` byte-identical; then
  `rebuild_stage_cascade.sh --write` re-sealed all 31 content-identical. Unread records 145 -> 138.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical
  to `7a262314d`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 171 / 1,869 passed, 0 failed. Rule-field counts: EvidenceIR 46, SemanticIR 61, IntentIR 60 (40 carried),
  qualification 198. `flow_census.json` rolled (+8 functions, +87 decision sites, +171 helper edges, +10 macros);
  boundary `rule_roots` 129 -> 130 and `grammar_declassifiers` 70 -> 71 for the one registered producer.
- [x] **GENERICITY (ADR 0006)** — English software nouns, articles, modals, condition openers, prepositions and
  procedure verbs; a name is software only by the document's own glossary row or *implemented in software* sentence;
  no document, agent, signal or protocol name in code.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*What a software agent does is software intent*),
  `pipeline/semanticir.md`, `pipeline/intentir.md`, `pipeline/isf-adapter.md`, `quality/validation.md`,
  `reference/intentir-contract.md`, `source-to-intent-contract.md`; KM `a-software-agents-statement-is-software-intent`
  (new); `claim_family_inventory.tsv`, `rule_family_inventory.tsv`, `spec_to_intent_category_contract.json`
  (`software_architecture_semantics` and `non_contract_disposition` name the surface) and the census's
  `TYPED_SURFACES`. No production rule deleted or replaced.
