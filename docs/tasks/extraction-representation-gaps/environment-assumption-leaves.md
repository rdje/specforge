# EXTRACTION-REPRESENTATION-GAPS — environment assumption leaves

- Part ID: `environment-assumption-leaves`
- State: `active`

The `.3.d.iii` family continues here, after `default-and-assumption-leaves` neared its rollover (`2026-10-20`): what a
component, an interface or a protocol assumes of its environment, told from the premises of the document, its
examples and its reader, and the unread-record census that measures them. `.3.d.iii.e` moved here whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.e`
  Status: `done` (`2026-10-20`; CODE; found by `.3.d.iii.c`; checklist below; opened `.3.d.iii.f`)
  Goal: **what an implementation or interface assumes about its environment is an assumption, not apparatus.**
  `.3.d.ii.b.i`'s premise phrase *assumes that* files a sentence as apparatus whatever its subject, which is right for
  an example's or a table's premise (*Table 9 assumes that the Hs devices are isolated*; *The circuit assumes that the
  RAM primitive is the FASM*) but not when the subject is the hardware: Wishbone's *The WISHBONE interface logic
  assumes that a low-skew clock distribution scheme is used on the target device, and that the clock-skew shall be
  low enough …*, AXI's *This MEC implementation assumes that Managers and caches do not perform …*, and I2C's *SMBus
  protocol just assumes that if something takes too long … all devices must reset*. **Reproduce:** apparatus
  statements saying *assumes that* whose subject is no document noun, example or analysis (5 on the 31 chains; these
  3, the table premise and the example circuit). **Impact:** the environment contract a component states is read by
  no reader, and SMBus's *Target devices are not then allowed to hold the clock LOW too long* with it. Acceptance:
  tell an environment assumption from a premise by the subject the document gives it (a component, an interface, an
  implementation, a protocol), keep the premise shapes apparatus, and measure every chain.
  **Decided `2026-10-20`: *assume that* and *assumes that* make a premise only when every use of them has a premise's
  subject.** `states_only_premises_with_assume_that` replaces the two phrases in `is_non_contract_assumption`: a use is a
  premise when the words of its sentence before it (a table cell counts as a sentence) hold a document noun, an
  analysis noun or a `PREMISE_SUBJECT_WORDS` word (*rules*, *limitation*, *descriptions*, *figures*, *shown*, *we*,
  *we'll*, *you*, *reader*), or nothing but connectives (*Assume that a target or line error has occurred*); one use
  with any other subject makes the sentence an assumption. The subject is told from the document's own words, never
  from a list of components (ADR 0006). `names_an_example_before_its_assumptions` replaces the bare *example* test: an
  example after every assume word illustrates the assumption (AXI's *This MEC implementation assumes that … For
  example, …*); a sentence with no assume word keeps the old test. *Assumes the* is not changed: its three uses with
  no premise subject are a *take on* and two software conclusions, none an environment assumption, and are
  `.3.d.iii.f`. Of the two premises the leaf named, *Table 9 assumes* stays apparatus by its subject; Wishbone's *The
  circuit assumes that the RAM primitive is the FASM* does not, because its statement also holds *the WISHBONE
  interface assumes that output data is valid at the rising [CLK_I] edge following the assertion of the [ACK_O]
  output*, an interface's assumption, and one such use makes the statement an assumption.
  **Measured over all 31 chains** (evidence dry-run against `791b74d27`'s): 27 identical; 5 statements in 4 change
  class, all `apparatus` -> `explicit_abstraction` (Wishbone 2, AXI, I2C's SMBus, RISC-V debug's *Debuggers may assume
  that a hardware platform has no harts with indexes higher than the first nonexistent one*, which `permission_rules`
  also reads: RISC-V debug 62 -> 63); no other evidence record moves. A first draft read a table row's cells as the
  subject (ADIv6's *| invalid ACK | Assume that a target or line error has occurred*); the cell break fixed it.
  Rebuilt stage by stage: SemanticIR abstractions +5 (Wishbone 0 -> 2, AXI 19 -> 20, I2C 0 -> 1,
  RISC-V debug 2 -> 3), IntentIR assumptions +5, RISC-V debug's permission carried; adapters unchanged but RISC-V
  debug's permission residual, which recounts; `.isf` byte-identical. Unread IntentIR records 139 -> 142: the five
  are now IntentIR assumptions, and three of them (AXI's MEC, SMBus, Wishbone's clock) are cited by no typed record.
  **Declared residual:** an environment assumption is an IntentIR assumption by design, and no typed record carries
  its subject and what it assumes; the census gives the two of `assumed_by_word` a new shape, `environment`, ahead of
  `narrative`, whose *assume that* would call them an example's premise. AXI's MEC is filed by the census as
  `apparatus` for its opening *Note that*, with 9 behaviors the same way: **opened `.3.d.iii.g`**.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.f`
  Status: `done` (`2026-10-20`; CODE; found by `.3.d.iii.e`; checklist below)
  Goal: **an *assumes the* whose subject is no premise's states what its subject does.** `.3.d.ii.b.i`'s premise
  phrase *assumes the* files a sentence as apparatus whatever its subject; `.3.d.iii.e` left it so, because the three
  uses whose subject is no document noun, figure, example, rule or reader are no environment assumption a record
  could carry: USB 3.2's *a host assumes the initial transmit or receive sequence number and burst size for bulk pipes
  after it has successfully completed the appropriate control transfer* (`statement_1706`) means *takes on*, and the
  USB4 connection-manager guide's *else Connection Manager assumes the configuration of Asymmetric Link failed*
  (`statement_1079`, and `statement_1087` for the symmetric link) is a software step's conclusion. **Reproduce:**
  apparatus statements on the 31 chains saying *assumes the* with none of those subjects before it. **Impact:** a
  host's reset of its bulk sequence state is read by no reader, and the guide's steps never reach `.3.d.i`. Acceptance:
  read a *take on* whose object is a value phrase (*the initial … sequence number and burst size*) the way
  `.3.d.ii.b.iii.b` reads one ending in *value* or *state*, route the software conclusion to `.3.d.i`, keep the
  premise shapes apparatus, and measure every chain.
  **Decided `2026-10-20`: *assumes the* takes `.3.d.iii.e`'s subject test, and a take-on is also read from its
  object's head.** `states_only_premises_with_a_premise_phrase` (renamed from `.3.d.iii.e`'s
  `states_only_premises_with_assume_that`) now counts *assumes the* among its phrases, matched as the old substring
  was (so *assumes these* and *assumes there* too), and `is_non_contract_assumption` no longer lists it. In
  `every_assume_word_states_no_abstraction` an active *assume* takes on when the clause's last word **or** its
  object's head (`object_head`: the last word before the first of `OBJECT_ENDS`, English prepositions and
  subordinators) is one of `TAKE_ON_NOUNS`, which gains *number(s)* and *size(s)*. *Of* is not an object end, so
  *assumes the configuration of Asymmetric Link failed* has the head *failed*: a conclusion, which stays an
  abstraction, and the census files it under `software_agent` for `.3.d.i`. **Measured over all 31 chains**
  (evidence dry-run against `61d5b3929`'s): 28 identical; 5 statements in 3 change class: USB 3.2's `statement_1706`
  `apparatus` -> `source_fact`, the connection-manager guide's `statement_1079` and `statement_1087` `apparatus` ->
  `explicit_abstraction`, and two Wishbone statements: the patents chapter's *… designers assume all
  responsibility for determining if their WISHBONE design infringes on the rights of others* (`statement_1140`,
  `explicit_abstraction` -> `source_fact`, the legal take-on `.3.d.ii.b.iii.b` meant and missed because the clause
  ends in *others*), and an example circuit's datasheet row, which the prose probe skipped, *| Special requirements: |
  Circuit assumes the use of synchronous RAM primitives with asynchronous read capability. |* (`statement_1339`,
  `apparatus` -> `explicit_abstraction`: the circuit's requirement on its environment, as `.3.d.iii.e` reads its prose
  twin, and a table row, which the census leaves to `.3.b`). No other evidence record moves. Rebuilt stage by stage: the
  guide's SemanticIR abstractions 1 -> 3 and IntentIR assumptions 2 -> 4, Wishbone's 2 -> 3 and 3 -> 4 (its patent
  disclaimer was never lifted by SemanticIR), USB 3.2 unchanged past EvidenceIR; adapters unchanged, `.isf`
  byte-identical. Unread IntentIR records 142 -> 145: `software_agent` 9 -> 11 (the two conclusions, `.3.d.i`'s) and
  `table_row` 39 -> 40 (the circuit's row, `.3.b`'s).

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.g`
  Status: `done` (`2026-10-20`; CODE, measurement only; found by `.3.d.iii.e`)
  Goal: **the census files a note as apparatus only when the note is about the document.** The unread-record census
  (`scripts/measure_unread_intent_records.py`) puts a record in its `apparatus` family, routed non-contract, when the
  cited statement opens with *Note* (`DOCUMENT_REFERENCE`), and in these documents a note states protocol behavior:
  USB 3.2's *Note: If the port enters this state because far end receiver terminations are not detected and VBUS is
  present, it may immediately transition to USPORT.Powered*, *Note: A device will receive an isochronous timestamp if
  its link is in U0 around a bus interval boundary*, the connection-manager guide's *Note: If Wake Events are not
  enabled … the Connection Manager may set the PORT_CS_19.USB4 Port is Configured bit to 1b*, and since `.3.d.iii.e`
  AXI's *Note that the MEC architecture specification [3] details several implementation options … This MEC
  implementation assumes that …*. **Reproduce:** census records of family `apparatus` whose statement opens with
  *Note* (10 on the 31 chains: 9 behaviors and 1 assumption). **Impact:** the census reports as non-contract
  records that state behavior, so the `permission_or_description` and `assumed_by_word` counts and their owners miss
  them. Acceptance: let *Note* open a document reference only when what follows speaks about the document (a figure,
  a table, a section, this specification), re-measure every family, and route each moved record to its owner.
  **Decided `2026-10-20`: a note label is read past, not taken as a reference.** `DOCUMENT_REFERENCE` no longer holds
  `note`; `without_note_label` strips a leading `Note:`, `Note 1:`, `Note -` or `Note that` before that test and
  before `subject_region`, so a note is apparatus only when what follows refers to the document (*Note: See …*), and a
  note's subject is found past its label. No product code changed; the census reads the same IntentIR. **Measured on
  the 31 chains** (`scripts/measure_unread_intent_records.py`, against `ff5dda775`'s run): unread records 145, as
  before; exactly the 10 records move and no other: `apparatus` 22 -> 12; `assumed_by_word` 9 -> 11 (AXI's MEC,
  shape `environment` 2 -> 3; AXI's *Note that the burst type is assumed to be INCR*, `stated_default` 4 -> 5);
  `software_agent` 11 -> 12 (the guide's *Note: If Wake Events are not enabled … the Connection Manager may set …*,
  found past its label, `.3.d.i`'s); `permission_or_description` 46 -> 53 (permission 32 -> 38, obligation 3 -> 4:
  the burst note's *this must be taken into account*). **The six permissions are `.3.d.iii.a`'s declared refusals**,
  replayed through its grammar probe: three pronoun subjects (*It may optionally be enabled*; *it may immediately
  transition*), two consequences (*may* before a verb of what can happen), a relative clause, and a subject split across
  commas (*the terminations, and therefore ability for Rx.Detect to complete, may be disabled due to …*, also a
  consequence); none is refused for its label.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.e` (CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — 21 apparatus statements on the 31 chains say *assume that* or *assumes that*
  (`.project-data/tmp/erg-3diiie/probe.py`); 5 have a subject that is no document noun, example, rule, figure,
  analysis or reader, and so do 3 *assumes the* uses, routed to `.3.d.iii.f`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `is_non_contract_assumption` listed *assume
  that* and *assumes that* among its premise phrases whatever came before them, and its *example* test fired on an
  example anywhere in the statement. New `states_only_premises_with_assume_that`, `names_an_example_before_its_assumptions`
  and `PREMISE_SUBJECT_WORDS`. RED, the file restored byte-identically after each (7 mutations, each failing its named
  test): the blanket phrases back, one premise use enough and an example anywhere
  (`an_environment_assumption_stated_with_assume_that_is_an_abstraction`); no premise words (also
  `a_premise_of_an_example_or_analysis_is_not_an_abstraction`), no document nouns, a subjectless use not a premise
  and the table cell joined to the row (`a_premise_stated_with_assume_that_stays_apparatus`). The subjectless mutant
  first survived because it disabled the word test inside `all`, which an empty word list never calls; disabling the
  whole conjunct killed it.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 27 identical, 5 statements in 4 change class, 1
  permission added, nothing else. The 4 chains rebuilt stage by stage, each artifact validated once, the
  adapter last; only abstractions, assumptions and RISC-V debug's permission move; `.isf` byte-identical. The
  classifier is outside the stage-seal closure, so no chain re-seals.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical
  to `791b74d27`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 171 / 1,865 passed, 0 failed. `flow_census.json` rolled (+2 functions, +15 decision sites, +8 helper edges);
  boundary unchanged. Census `scripts/measure_unread_intent_records.py`: unread 139 -> 142, shape `environment` 2.
- [x] **GENERICITY (ADR 0006)** — document, analysis and reader nouns, pronouns and connectives of English; no document,
  component, signal or protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*What the hardware assumes of its environment*),
  `pipeline/intentir.md`; KM `an-assume-sentence-about-the-document-is-apparatus` and
  `an-obligation-stated-with-assume-keeps-its-class` amended. A new active part, `environment-assumption-leaves`,
  holds `.3.d.iii.e` and `.3.d.iii.f` (`default-and-assumption-leaves` at 81.4% of its bytes): manifest, contract,
  route catalog, index and parts surface. No production rule deleted; two premise phrases replaced by the narrower test.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.f` (CODE) — DONE `2026-10-20`

- [x] **REPRODUCE / MEASURE** — apparatus statements on the 31 chains saying *assumes the* with no premise subject: 3
  (`.project-data/tmp/erg-3diiie/probe.py`); the object-head probe (`.project-data/tmp/erg-3diiif/probe.py`, prose
  only) listed the two a head reading changes: USB 3.2's host sentence and Wishbone's patent disclaimer.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `is_non_contract_assumption` listed
  *assumes the* whatever its subject, and `every_assume_word_states_no_abstraction` read a take-on only from the
  clause's last word. Changed: `states_only_premises_with_a_premise_phrase`, `object_head`, `OBJECT_ENDS`,
  `TAKE_ON_NOUNS`. RED, the file restored byte-identically after each (5 mutations, each failing
  `assumes_the_is_a_premise_only_with_a_premise_subject`): the blanket phrase back, *assumes the* dropped from the
  subject test, no object head, *of* ending the object, and no size nouns.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 28 identical, 5 statements in 3 change class,
  nothing else. The 3 chains rebuilt stage by stage, each artifact validated once, the
  adapter last; only abstractions and assumptions move; `.isf` byte-identical. The classifier is outside the
  stage-seal closure, so no chain re-seals.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical
  to `61d5b3929`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 171 / 1,866 passed, 0 failed. `flow_census.json` rolled (+1 function, +2 decision sites, +2 helper edges);
  boundary unchanged. Census: unread 142 -> 145, `software_agent` 11, `table_row` 40.
- [x] **GENERICITY (ADR 0006)** — English prepositions, subordinators and value nouns; no document, component, signal or
  protocol name.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (*What a subject takes on*); KM
  `an-assume-sentence-about-the-document-is-apparatus` amended (the rename too). No production rule deleted; one
  premise phrase replaced by the subject test.
