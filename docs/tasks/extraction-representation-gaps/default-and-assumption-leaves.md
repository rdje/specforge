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
  Status: `open` (`2026-10-19`; DESIGN + CODE; found by `.3.d.ii.b.iii`)
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
