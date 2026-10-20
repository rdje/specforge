# EXTRACTION-REPRESENTATION-GAPS — default and assumption leaves

- Part ID: `default-and-assumption-leaves`
- State: `active`

The `.3.d` family continues here, after `unread-intent-leaves` reached its mandatory rollover (`2026-10-19`): the stated
defaults and the assume-sentences no typed record yet understands, and the readers that will. `.3.d.ii.b.iii` and its
children moved here whole, with their acceptance checklists.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii`
  Status: `done` (`2026-10-19`; MEASUREMENT, re-scoped from CODE; found by `.3.d.ii.b`; children `.3.d.ii.b.iii.a`-`.3.d.ii.b.iii.c`)
  Goal: **a stated default of an attribute, flow or field reaches its reader.** Among the 25 `stated_default` records,
  AXI states what a request's domain is when `AxDOMAIN` is absent (*Non-cacheable requests are assumed to be in the
  System domain*; *CMO transactions are assumed to be Non-shareable*), which SMMU flow holds when none is indicated
  (*a Stall flow is assumed*) and what the interface state is without credit control (*assumed to be running when not
  in reset*); USB4 states a lane default (*assume single-Lane operation*); IOMMU a field value under a fault (*the DTF
  value assumed … is 0*). **Reproduce:** shape `stated_default`. **Impact:** a default an absent signal implies is
  carried only when the sentence writes a literal (`.3.d.ii.a`), so a default written as a label (*Non-shareable*, a
  member of `AxDOMAIN`'s own encoding) reaches no record. Acceptance: let an absent value be a member name the
  document's enum for that signal defines, route a flow or state default to the reader that owns the attribute, and
  measure every chain.
  **Measured `2026-10-19`** (`python3 scripts/measure_unread_intent_records.py --json`, shape `stated_default`, every
  record read in its statement's neighbours). After `.3.d.ii.b.i` filed four analysis premises as apparatus, **21**
  remain, in 7 chains, and they need three different remedies and one existing owner:
  | group | records | what they say | owner |
  | --- | ---: | --- | --- |
  | a labeled default for a class of requests | 3 | AXI `statement_2359`/`2360`, bullets under *If AxDOMAIN signals are not present:* (`statement_2358`): *Non-cacheable requests are assumed to be in the System domain*, *Cacheable requests … in the Shareable domain*; `statement_2701` *If AxDOMAIN is omitted from the interface, CMO transactions are assumed to be Non-shareable* | `.3.d.ii.b.iii.a` |
  | an *assume* word that states no abstraction | 13 | premises: OpenCAPI's airflow *must be assumed to be approximately 5 ⁰C above ambient*, *One ×8 cable is assumed for this guideline*, *The moving average smoothing procedure assumes a window size of 1 GHz*; USB 3.2's *the conceptual clocking architecture shown in Figure E-15 is assumed* and *The reference JTF curves in Figure E-17 assume a second order filter*. *Assume* as *take on*: IOMMU's *the bits cqb.LOG2SZ-1:0 in cqt assume a valid but otherwise UNSPECIFIED value* (and `fqb`, `pqb`), USB 3.2's SetConfiguration *causes the device to assume the described configuration* and the hub that *shall ignore the route string and assume all packets are routed directly to the hub*. A negation: AXI's *the missing input cannot be assumed to take a fixed value*. A field value under a fault: IOMMU's *the PRPR value assumed is 0* and *the DTF value assumed for reporting such faults is 0* | `.3.d.ii.b.iii.b` |
  | a default whose subject is no presence-record signal | 4 | AXI `statement_3715` *If no flow is indicated, a Stall flow is assumed*; `statement_3903` *If credit control signals are not included, the interface is assumed to be running when not in reset*; `statement_5203` *ACSNOOP, all requests on the AC channel can be assumed to be DVM messages*; `statement_2337` *if an interface does not have Domain signaling then Non-cacheable requests are assumed to be in the System Domain and Cacheable requests … in the Non-shareable Domain* | `.3.d.ii.b.iii.c` |
  | a software procedure | 1 | USB4 inter-domain: the Local Domain with the higher UUID *shall* … *If bonding does not complete, assume single-Lane operation* | `.3.d.i` |
  **Two findings change the leaf's premise.** AXI states every domain default for a **class of requests**, not for
  the signal: binding *Non-shareable* as `AWDOMAIN`'s one absent value would assign it to non-CMO requests too, which
  the document contradicts two sections earlier, so the carrier must keep the class the value holds for. And **the
  document disagrees with itself**: `statement_2337` puts cacheable requests in the Non-shareable domain when
  domain signaling is absent, and `statement_2360` puts them in the Shareable domain. A label can be resolved where
  the absent value is bound, because EvidenceIR already states each enum member as a statement citing its table
  (`Enum AxDOMAIN SYSTEM_DOMAIN = 3.`, `table_0111`, whose rows carry the label and meaning cells *System* and *System
  domain*). No code change here, so no acceptance checklist applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.a`
  Status: `done` (`2026-10-19`; CODE; found by `.3.d.ii.b.iii`; checklist below; opened `TEXT-LAYER-IDENTIFIER-SPLIT.5`)
  Goal: **a stated absent value may be a label of the signal's own encoding, and may hold for a class of requests
  only.** AXI states `AxDOMAIN`'s defaults per request class, in a list under a lead-in that carries the absence
  clause (*If AxDOMAIN signals are not present:* / *- Non-cacheable requests are assumed to be in the System domain.*)
  and in one sentence (*If AxDOMAIN is omitted from the interface, CMO transactions are assumed to be
  Non-shareable*). **Reproduce:** the 3 records of `.3.d.ii.b.iii`'s first group; `AWDOMAIN` (`presence_0033`) and
  `ARDOMAIN` (`presence_0112`) carry no absent value. **Impact:** the domain a request takes when the interface has
  no domain signals reaches no record. Acceptance: read a label value only when it is a cell of a row of the table
  the document's enum for that signal (or its collective name) cites, naming the member; keep the class a value holds
  for as written and never fold a class-scoped value into the signal's one absent value; read a list lead-in's
  absence clause into the bullets it introduces; and measure every chain.
  **Decided `2026-10-19`: a class-scoped value is a qualified value on the presence record, and a label carries the
  code its encoding row gives it.** `SignalPresenceRecord` gains `qualified_absent_values` (serialised only when
  non-empty), and `StatedAbsentValue` gains `applies_to` (the class as written) and `encoding: StatedValueEncoding {
  table_id, codes }`; the records still cross SemanticIR and IntentIR unchanged. **A label** is the clause's words
  after *assumed to be* without a leading *in the*; it binds only when a row of a table the document's `Enum`
  statements for the signal, or for a collective name it is a member of, cite carries it as a cell, and every such
  row's code is kept as written (*Shareable domain* is `0b01` and `0b10`). The convergent loop writes those `Enum`
  statements, so `bind_stated_absent_values` now runs after it. **A class** is the subject of the value's clause
  when the absence is stated in another clause or in the list's lead-in and the subject is not the signal (a pronoun,
  a final noun for a signal or its value, a name, or a subject itself said to be absent). **A list**: a bullet
  stating no absence takes the absence of the statement that introduces its list, when that statement ends with a
  colon and states one, with only bullets between; both are cited. Agreement is settled per class. A literal still
  wins wherever it occurs in a statement, so no existing binding can move. **A table row is not read**: the first
  measurement bound a fourth AXI class from the change history's *| Cache maintenance operations | Clarification |
  It is clarified that if the AxDOMAIN signals are missing, a CMOis assumed to be Non-shareable. |*, whose fused
  *CMOis* became the class *a CMOis*; a row belongs to its table's reader, as for the other prose readers that skip
  rows. The adapter keeps one `isf_protocol_signal_qualified_absent_value_<presence>_<n>` residual per class-scoped
  value, and `validate` reports `signal_presence_qualified_absent_values`.
  **Measured over all 31 chains:** evidence dry-run with the HEAD and the new binary, 30 identical; AXI moves only in
  `signal_presence_records`: `AWDOMAIN` (`presence_0033`) and `ARDOMAIN` (`presence_0112`) each take *System domain*
  for *Non-cacheable requests* (`0b11`, `statement_2358` + `statement_2359`), *Shareable domain* for *Cacheable
  requests* (`0b01`, `0b10`, `statement_2358` + `statement_2360`) and *Non-shareable* for *CMO transactions* (`0b00`,
  `statement_2701`), all through `AxDOMAIN` and `table_0111`. The rebuilt chain moves only that field at SemanticIR
  and IntentIR; the adapter gains six residuals, and `manager.isf` is byte-identical.
  **Issues uncovered, owned:** the text layer fuses an identifier with the word after it (`CMOis`, `IOMMUto`,
  `AWMMUFLOWis`): 52 tokens in 8 chains by a lower-bound census, `TEXT-LAYER-IDENTIFIER-SPLIT.5`. The producer-graph
  analyser hides a value captured inside a format string (`format!("…{presence}")`; one controlled edit moves
  `semantic_macros` by exactly 1), so this slice passes such values as explicit arguments and
  `PRODUCTION-GRAPH-CENSUS-PIN.5` owns the analyser; its name fallback (`.4`) attached 374 false edges to a first
  draft's `AbsentValueEncodings::new`, which is therefore named `read`. The workspace test run also exposed `.48`'s
  regression in the trajectory controller, fixed first as `.48.a`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.b`
  Status: `done` (`2026-10-19`; CODE; found by `.3.d.ii.b.iii`; checklist below)
  Goal: **an *assume* word that states no abstraction does not make the sentence one.** 13 records: five premises
  `.3.d.ii.b.i`'s non-contract test does not see (a figure named inside the subject, *for this guideline*, a
  procedure as subject, a design-guidance *must be assumed*), five sentences where *assume* means *take on* (a
  register's bits, a device's configuration, a hub's routing), one negation (*cannot be assumed to take a fixed
  value*), and two field values under a fault condition (IOMMU's PRPR and DTF). **Reproduce:** the 13 records of
  `.3.d.ii.b.iii`'s second group. **Impact:** a hardware behavior and two conditional field values are filed as
  assumptions no reader lowers. Acceptance: file the premises as apparatus, let the other sentences take the class
  they would take without the word, measure what every reader then mints from them (`.3.d.ii.b.i` measured that a
  fall-through can mint rules from non-contract prose), and keep all 11 seeds.
  **Decided `2026-10-19`: a sentence keeps its own class when every assume word in it states no assumption, and
  three more premises are apparatus.** `abstraction_class` now asks `every_assume_word_states_no_abstraction` before
  returning `ExplicitAbstraction` (a *treated as* sentence never asks): each whole-word *assume*/*assumes*/*assumed*
  must be negated in its clause (*cannot be*, *should not*, …), an active verb whose object ends in a value, state,
  configuration, mode, role or responsibility (*take on*), an active verb governed by *shall* in its clause (the
  device's obligation), or *assumed* right after *value*; a modelling phrase or one ordinary use keeps the sentence
  an abstraction. `is_non_contract_assumption` gains `premise_of_a_figure_a_guideline_or_an_analysis` (a figure named
  in the assume clause, a document noun after *this* in the sentence, *guideline* included, an analysis noun in the
  clause) and the premise phrases *assumed to be approximately* and *examples*. **Measured over all 31 chains**
  (evidence dry-run, `fa37925ef`-equivalent binary against the new one): 24 identical; in 7, `extracted_statements` is
  the only section that moves, 19 statements changing class. The 13 targets: IOMMU's PRPR, DTF, `cqb`, `fqb` and
  `pqb` statements -> `source_fact`; AXI `statement_4784` -> `derived_rule`; USB 3.2's hub obligation -> `normative_statement`
  and its SetConfiguration sentence -> `timing_constraint` (the timing arm's misfire `.3.d.iii` owns); OpenCAPI's thermal,
  cable and ILD premises and USB 3.2's two figure premises -> `apparatus`. Six more, each read: RISC-V debug's *All
  examples assume …* (2) and Wishbone's two legal sentences (*The user of this document assumes all responsibility*)
  -> `apparatus`; Wishbone's *Three state buffers can assume a logic low state* -> `source_fact`; USB 3.2's *An endpoint
  and its client software cannot assume a specific rate of service* -> `normative_statement`. USB 3.2's two *should not
  assume* sentences stay abstractions: each also says *can assume only that it will receive a transaction*. Rebuilt
  stage by stage: SemanticIR abstractions fall in 6 chains (IOMMU 11 -> 6, USB 24 -> 19, RISC-V debug 4 -> 2, OpenCAPI
  25G 3 -> 1, OpenCAPI 32G 1 -> 0, AXI 22 -> 21); AXI gains 1 invariant and USB 2 — the reclassified rules themselves
  (*the CHK signal cannot be used reliably*, the hub's routing obligation, *client software cannot assume ordering*),
  carried as constraints and temporal invariants as every rule is; adapters and the two emitted `.isf` texts
  byte-identical; Wishbone moves only in EvidenceIR. Unread records 361 -> 345 (15 assumptions and the hub's behavior
  record), `assumed_by_word` 50 -> 37, `stated_default` 18 -> 5.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.c`
  Status: `done` (`2026-10-19`; DESIGN, re-scoped from DESIGN + CODE; found by `.3.d.ii.b.iii`; opened `.49`)
  Goal: **a default whose subject is no presence-record signal reaches the record that owns it.** 4 AXI records: the
  SMMU flow default (the statement names `MMUFLOW_Present`, a term of the presence conditions of `AWMMUFLOW`,
  `ARMMUFLOW`, `AWMMUATST` and `ARMMUATST`, and *Stall* labels code 0 of `AxMMUFLOW`'s encoding), the interface
  state without credit control (*running when not in reset*; the protocol states read are `DEACTIVATE`, `STOP` and
  `ACTIVATE`), `ACSNOOP`'s removal rationale (a signal of earlier issues with no presence row), and *Domain
  signaling*'s defaults, which name the `AxDOMAIN` signals by a phrase and disagree with `statement_2360`.
  **Reproduce:** the 4 records of `.3.d.ii.b.iii`'s third group. **Impact:** a flow and an interface-state default
  stay assumptions, and the document's disagreement over a cacheable request's domain is invisible. Acceptance:
  decide each carrier (a default reached through the property that gates the signal, an interface-state default, a
  removed signal's rationale as non-contract), surface the disagreement as a conflict record, and measure every
  chain.
  **Decided `2026-10-19`: no reader for any of the four, because none of their shapes recurs, and the one defect
  underneath is owned on its own.** Measured over the 31 chains: *If no … is indicated, … is assumed* occurs once
  (`statement_3715`), *the interface is assumed to be* once (`statement_3903`), a list of signals *no longer required*
  once (`statement_5202`, the lead-in of `statement_5203`), and *<word> signaling* 252 times, almost all of them
  section titles and contents rows, so it is no handle on a group of signals. A reader for any of them would read
  one AXI sentence, which ADR 0006's genericity forbids in all but name. Dispositions, each recorded where a reader
  of the census will find it:
  - **The Stall flow** (`statement_3715`) stays an IntentIR assumption. Binding *Stall* to `AWMMUFLOW`/`ARMMUFLOW`
    would need two inferences no structure grounds: that *no flow is indicated* means the signals are absent, and
    that *flow* names them (only the second sentence's `MMUFLOW_Present`, a term of their presence condition, hints
    at it). A label alone would resolve, through `AxMMUFLOW`'s encoding (`table_0198`), if a second document wrote
    the shape.
  - **The interface state without credit control** (`statement_3903`, *running when not in reset*) waits on a carrier
    that does not exist: AXI enumerates four credit-control states (*STOP, ACTIV ATE, RUN, DEACTIVATE*,
    `statement_3926`) and writes *the interface moves into the RUN state* (`statement_3939`), but `protocol_states`
    holds only DEACTIVATE, STOP and ACTIVATE. That recall defect is `.49`, which also decides whether the default
    binds to RUN.
  - **`ACSNOOP`'s rationale** (`statement_5203`) stays an IntentIR assumption: a bullet under *the following signals
    described in earlier issues … are no longer required*, about a signal this issue neither declares nor lists in a
    presence matrix.
  - **The disagreement** (`statement_2337` puts a cacheable request in the Non-shareable domain without domain
    signaling, `statement_2360` in the Shareable domain) is not a conflict record: `statement_2337` names no signal,
    only *Domain signaling*, so no binder reads it and there is nothing structural to conflict with. It is surfaced in
    the book (`pipeline/evidence-rule-chronology.md`, the label and class section) and in
    [[assume-sentences-fall-into-six-shapes]]; the class-scoped value `.3.d.ii.b.iii.a` bound is `statement_2360`'s.
  No code change, so no acceptance checklist applies.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.b`
  Status: `done` (`2026-10-20`; CODE; found by `.3.d.iii`; checklist below; opened `.50` and `SCRATCH-RESIDUE-CONTAINMENT.15`)
  Goal: **a statement is a timing constraint only when it states a time.** `classify_statement`'s timing arm matches
  substrings: ` cycle`, `cycles` and `at least` in its first list and `cycle`, `ns`, `ps`, `time` and `edge` in its
  second, so a sentence that mentions bus cycles, a quantity *at least* some size, or a word containing `ns` is filed
  `timing_constraint`. **Reproduce:** 645 of the 813 `timing_constraint` statements on the 31 chains carry no time
  expression (Wishbone 268, USB 3.2 72, ADIv6 62, AXI 60), among them the 17 unread behaviors `.3.d.iii` found.
  **Impact:** every reader of the class takes prose that states no time, and the behaviors it lifts are mislabelled.
  Acceptance: decide what a time expression is in the document's own terms (a number with a time or cycle unit, a
  named timing parameter, a clock edge or period, a latency), match the words as whole words, measure what every chain
  and every reader of the class loses and gains, and keep all 11 seeds, the temporal golds included.
  **Decided `2026-10-20`: the timing words admit a sentence only when it also states a time.** The arm keeps its two
  lists and now also asks `states_a_time`, which reads whole words (an underscore stays in a token, a possessive *'s*
  is dropped): a number and its unit, apart or fused (*clock* only before *cycle(s)*/*period(s)*, *bit* only as *bit
  time*, a fused *s* never); a count of cycles, clocks or seconds (*one*-*ten*, *single*, *N*) and of a named clock's
  cycles (*50 SWCLKTCK cycles*); an event placed at a cycle (*next*, *previous*, *same*, *following*, *subsequent*,
  *first*, *last*; *cycle N*); the eight timing abbreviations or a token written as a lowercase *t* before a capital
  (*tSU*, *tPortConfiguration*: all 98 such tokens on the 31 chains are USB 3.2, USB4, I2C and I2S timing parameters);
  *setup time*, *hold time*, *clock period*, *propagation delay*, *clock edge*; and an edge word with *edge* at most
  one word later. *The current cycle* (Wishbone's bus cycle under way) and a latency with no value state none. A gate
  that only narrows was chosen over the test alone: a first draft that used it alone also pulled 529 statements into
  the class (timers, table rows, data rates) for a label only. **Measured over all 31 chains** (evidence dry-run, `2d61dc479`'s
  artifacts against the new binary's): 8 identical; 534 of 813 statements leave the class (374 -> `source_fact`, 109 ->
  `normative_statement`, 51 -> `conditional_rule`; Wishbone 249, USB 3.2 52, ADIv6 41, OpenCAPI DL 23) and 279 keep it,
  every one attributed to a rule by a Python port of the test. The other readers add 47 conditional rules in 11 chains
  (Wishbone 18, USB 3.2 9, AXI, ADIv6 and RISC-V debug 4 each) and 9 prose hints in TMC, AHB and Wishbone, and remove
  nothing; `extract_interface_edge_timings`, the class's own reader, is unchanged. Rebuilt stage by stage (23 chains),
  SemanticIR and IntentIR move only `conditional_rules` and, in those three, the hinted signals' semantic candidates;
  Wishbone's adapter gains one `isf_obligations_not_lowered` residual (5 new obligations: no level to drive, or met by
  another actor), AXI's and ADIv6's residual texts renumber rule ids, and every `.isf` text is byte-identical. Six of
  the 9 new hints give a role the sentence does not (`ATREADYM`/`ATREADYS`, `HREADY`, `RTY_I`/`RTY_O`, `DAT_O`): the
  prose hint reader's precision, owned by `.50`. Unread behaviors stay 176 (their timing-classed members 17 -> 4;
  `.3.d.iii.a` owns the permissions). 728 statements state a time with none of the admitting words and keep their
  class (273 of them table rows), deliberately: the class has no reader they would reach.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a`
  Status: `open` (`2026-10-19`; DESIGN + CODE; found by `.3.d.iii`)
  Goal: **a stated permission is a typed record.** 154 unread behaviors grant something (*A MASTER MAY signal Classic
  Cycle indefinitely*; *The IO bridge may provide a completion to the device*; *TRX-ST-RJ may be adjusted to meet the
  target value*). **Reproduce:** modality `permission`. **Impact:** optional behavior has no carrier, so a document's
  permissions are indistinguishable from its unread descriptions. Acceptance: decide the carrier (the agent or
  subject, the permitted action, a condition, as written, the way `.3.d.ii.b.ii` carries an equivalence), keep a
  software agent's permission with `.3.d.i`, read the 154 into it citing their statements, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.c`
  Status: `open` (`2026-10-19`; CODE; found by `.3.d.iii`)
  Goal: **an obligation stated with *assume that* is no apparatus, and the remaining behaviors have a disposition.**
  `.3.d.ii.b.i`'s premise phrase *assume that* filed 7 obligations as apparatus (*A device shall not assume that
  transactions occur at the same location within each service interval*; *the debugger must not assume that the TAR
  wraps*; *A channel must meet the common-mode requirements*), and 14 behaviors are recommendations (4), capabilities
  (3) and descriptions (7). **Reproduce:** modality `obligation` with class `apparatus`; the other three modalities.
  **Impact:** an obligation is read by no obligation reader. Acceptance: let a sentence whose *assume* is governed by
  *shall* or *must* (negated or not) keep its normative class, decide the remainder's dispositions, route software
  agents to `.3.d.i`, and measure every chain.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.a` (CODE) — DONE `2026-10-19`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_unread_intent_records.py`: `assumed_by_word` 53, `stated_default`
  21, among them AXI `statement_2359`, `statement_2360` and `statement_2701`; `validate` on AXI's EvidenceIR:
  `signal_presence_absent_values` 4 and no value on `AWDOMAIN` (`presence_0033`) or `ARDOMAIN` (`presence_0112`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `stated_absent_value_literal` read one token
  after *assumed to be* and required a literal (`is_absent_value_literal`), required the absence predicate in the same
  sentence, and had no carrier for a class: *Non-shareable* is a label, *Non-cacheable requests* a class, and the
  bullets' absence is in their lead-in (`statement_2358`). The labels' encoding is `table_0111`, which only the
  convergent loop's `Enum AxDOMAIN …` statements name. RED, with the file restored byte-identically after each (6
  mutations, each failing its named test): a label bound without the encoding (`a_label_no_encoding_of_the_signal_names_binds_nothing`,
  `a_qualified_or_prose_absent_value_is_not_read`), a class folded into the signal's own value
  (`a_value_stated_for_a_class_of_requests_is_qualified`), no list lead-in
  (`a_list_lead_in_states_the_absence_for_its_bullets`), table rows read (`a_table_row_is_not_read_as_a_sentence`),
  a self noun taken as a class (`a_subject_that_is_the_signal_is_no_class`), agreement across classes
  (`classes_settle_agreement_apart`).
- [x] **ADDRESSED (verified)** — Evidence dry-run of all 31 chains, HEAD binary against new: 30 identical; AXI moves
  only in `signal_presence_records`, `AWDOMAIN` and `ARDOMAIN` 0 -> 3 class-scoped values each (`validate`:
  `signal_presence_qualified_absent_values` 6, `signal_presence_absent_values` 4 unchanged). Rebuilt stage by stage,
  each validated once, the adapter last: SemanticIR and IntentIR move only `signal_presence_records`; the adapter
  gains 6 `isf_protocol_signal_qualified_absent_value_*` residuals and removes none; `manager.isf` byte-identical.
  Unread records 364 -> 361, `assumed_by_word` 53 -> 50, `stated_default` 21 -> 18.
- [x] **NO REGRESSION** — `check_chain_currency.sh`: 31 of 31 current at every stage, the retained set exact (the
  binder is outside the EvidenceIR seal closure; the new fields are nested and serialised only when present).
  WIRE-BASED-100: all 11 seeds byte-identical (temp paths masked), the HEAD binary scored on the HEAD artifacts and the
  new one on the rebuilt chain. `kg-bench` 158/158 (`fixtures_failed: 0`). `cargo fmt --check` clean; `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 171 /
  1,841 passed, 0 failed, on the final binary, after which `check_chain_currency.sh` reported 31 of 31 current again.
  `flow_census.json` rolled forward (+8 functions, +88 decision sites, +129 helper edges, +1 semantic macro); boundary
  counts unchanged.
- [x] **GENERICITY (ADR 0006)** — the absence predicates, English auxiliaries, pronouns and nouns for a signal or its
  value, a leading preposition and article, a colon closing a lead-in, and a bullet marker; a label resolves only
  through the document's own `Enum` statements and encoding table. No document, signal or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*A label of the signal's own encoding, and a value that
  holds for one class of requests*), `pipeline/semanticir.md`, `pipeline/intentir.md`, `pipeline/isf-adapter.md`,
  `reference/representation-gaps.md` (the *Signal absent by default* row); KM
  `an-absent-value-may-be-a-label-and-hold-for-a-class` (new),
  `a-signals-stated-absent-value-rides-its-presence-record` and `assume-sentences-fall-into-six-shapes` (amended). No
  production rule was deleted or replaced: the literal reader is kept and wins wherever a literal occurs; the book
  text that said a qualified value is not read still holds for a qualification after the value.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii.b` (CODE) — DONE `2026-10-19`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_unread_intent_records.py`: `assumed_by_word` 50, `stated_default`
  18, 13 of them premises, *take on* uses, a negation and two field values (`.3.d.ii.b.iii`'s second group); 97
  `explicit_abstraction` statements on the 31 chains, 40 of them *treated as*.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `abstraction_class` returned `ExplicitAbstraction` for
  every sentence using an assume word that `is_non_contract_assumption` did not call apparatus, and that test saw a
  figure, a document or an analysis only as the sentence's opening subject and *example* only in the singular. RED,
  the file restored byte-identically after each (9 mutations, each failing its named test): no figure/guideline/analysis
  premise, no approximation premise, no plural *examples*
  (`a_premise_of_a_figure_a_guideline_or_an_analysis_is_not_an_abstraction`); the fall-through disabled, no negation,
  no *take on*, no obligation, no stated value (`an_assume_word_that_states_no_abstraction_leaves_the_sentence_its_own_class`);
  *any* instead of *every* (`one_ordinary_assume_keeps_the_sentence_an_abstraction`).
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains, 24 identical; 19 statements in 7 change class and no
  other evidence section moves. Rebuilt stage by stage, each validated once, the adapter last: SemanticIR abstractions
  65 -> 49 across the six chains, IntentIR assumptions 15 fewer, 3 invariants and constraints gained (the rules
  themselves); adapters and `.isf` byte-identical. Unread records 361 -> 345, `assumed_by_word` 50 -> 37,
  `stated_default` 18 -> 5.
- [x] **NO REGRESSION** — the classifier is outside the EvidenceIR seal closure: `check_chain_currency.sh` 31 of 31
  current at every stage after the rebuilds. WIRE-BASED-100: all 11 seeds byte-identical to `fa37925ef`'s reports
  (temp paths masked), each side on its own binary. `kg-bench` 158/158 (`fixtures_failed: 0`). `cargo fmt --check` clean;
  `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 171 / 1,844 passed, 0 failed. `flow_census.json` rolled forward (+5 functions,
  +23 decision sites, +26 helper edges); boundary counts unchanged.
- [x] **GENERICITY (ADR 0006)** — English negations and modals, nouns for what is taken on, nouns of a document or an
  analysis, *figure*, and premise phrases; no document, signal or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*An assume word that states no assumption*),
  `pipeline/intentir.md` (the unread-assumption passage); KM `an-assume-sentence-about-the-document-is-apparatus` and
  `assume-sentences-fall-into-six-shapes` (amended). No production rule was deleted or replaced: the abstraction arm is
  narrowed and the apparatus test widened, and the book's `.3.d.ii.b.i` text still holds.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.b` (CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — `.3.d.iii`'s census: 813 `timing_constraint` statements on the 31 chains, 645 with no
  time expression by the probe (`.project-data/tmp/erg-3diiib/probe.py`), Wishbone 268, among them 17 unread behaviors.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `classify_statement`'s timing arm matched both
  word lists as substrings (`cycles` satisfies both, `ns` matches *transactions*, `at least` a size). RED, the file
  restored byte-identically after each: the gate forced open fails
  `the_timing_words_admit_a_sentence_only_when_it_states_a_time`; `states_a_time` cut short after the parameter test
  fails that test and `a_number_and_its_unit_a_counted_or_placed_cycle_a_parameter_or_an_edge_states_a_time`.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 534 statements leave the class and none joins; 47
  conditional rules and 9 hints added, none removed. The 23 changed chains rebuilt stage by stage, each artifact
  validated once, the adapter last; `.isf` texts byte-identical. `python3 scripts/measure_unread_intent_records.py`:
  176 unread behaviors, timing-classed 17 -> 4.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current after the rebuilds (no new field, no re-seal).
  WIRE-BASED-100: the 11 seeds' scores identical to `2d61dc479`'s reports, each side on its own binary; the three
  temporal golds, I2C, NVMe, RISC-V debug registers and SWD derivation byte-identical; AHB, APB, AXI and SWD differ
  only in the candidate pool (71 -> 82, 18 -> 20, 545 -> 551, 306 -> 317) and existing-rule counts (AXI 150 -> 154, SWD
  101 -> 105). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D
  warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 171 / 1,854 passed, 0
  failed. `flow_census.json` rolled forward (+1 function, +18 decision sites, +6 helper edges, +2 macros); boundary
  counts unchanged.
- [x] **GENERICITY (ADR 0006)** — time units, English number words, relative-order words and the typographic
  convention for timing parameters; no document, signal or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*A timing constraint states a time*); KM
  `a-timing-constraint-states-a-time` (new) and `unread-behaviors-are-permissions-and-the-timing-class-misfires`
  (amended). No production rule deleted: the arm is narrowed.
