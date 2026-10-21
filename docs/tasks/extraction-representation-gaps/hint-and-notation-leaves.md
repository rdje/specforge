# EXTRACTION-REPRESENTATION-GAPS — hint and notation leaves

- Part ID: `hint-and-notation-leaves`
- State: `active`

Open ERG leaves continue here, after `register-binding-leaves` neared its rollover (`2026-10-20`): the roles a prose
hint gives a signal, and the sentences that define the document's own notation. `.50` and `.51` moved here whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.50`
  Status: `done` (`2026-10-21`; CODE; found by `.3.d.iii.b`; checklist below; opened `.53`, `.54`)
  Goal: **a prose hint gives a signal the role its sentence gives it.** The prose hint reader
  (`synthesize_signal_semantic_hints_from_prose`, source facts only) tags a signal from the words near it. When
  `.3.d.iii.b` returned 534 misfiled timing statements to their own classes it read them for the first time, and of
  its 9 new hints 6 give a role the sentence does not: TMC's `ATREADYM`/`ATREADYS` -> `handshake_valid_like` from *If
  there is valid data, that is, ATVALID HIGH, the data was accepted this cycle* (the valid signal is another named
  one), AHB's `HREADY` -> `handshake_valid_like` from *provide valid data in the final cycle of the transfer, as
  indicated by HREADY HIGH*, Wishbone's `RTY_I`/`RTY_O` -> `handshake_ready_like` from *indicates that the interface is
  not ready to accept or send data* (a negated readiness: a retry), and Wishbone's `DAT_O` (already tagged from another
  sentence) from *presents valid data on its [DAT_O()] lines* (the data the validity qualifies). **Reproduce:**
  `signal_semantic_hints` of those three chains, `statement_1727`/`1751`, `statement_0733`, `statement_0398`/`0412`,
  `statement_1232`. **Impact:** interfaces' signal records carry a handshake role the document does not give (TMC's
  table tags were already wrong and are now `cross_modality`); no transaction, temporal record or `.isf` text moves
  today. Acceptance: tag a signal valid- or ready-like only when the sentence makes that signal the bearer of the
  validity or readiness (not the data it qualifies, not another named signal, not a negated readiness), and measure
  every chain's hints and interfaces.
  **Found `2026-10-21`: the six were one rule's misreadings, in every modality.** HEAD's 136 hints over the 31 chains
  (prose 85, tables 47, captions 4) came from one word test, `contains_any(" valid", "valid ", …)`, on the text with
  signal names stripped: a role inside a longer word counted (*CleanInvalid*, *validity*, *already*, *AFVALID*), and
  any role word tagged whichever signal stood near it. And AXI declares a generic `VALID`, which *Valid* case-folds
  to, so the stripping removed every *valid* from AXI's descriptions: its valid indicators came from the local corpus
  memory alone (126 hints without it). **Decided:** `semantic_role_phrase_spans` matches role phrases as whole words,
  and `retain_borne_semantic_roles` keeps a role, for all three readers, only when the text makes the signal its
  bearer — the agent of an indicating predicate whose complement holds it (*STB_O indicates a valid data transfer
  cycle*; *keeps ACK_I asserted to indicate that it is ready*; *… by asserting ACK_I*; *asserted when … valid*), the
  subject of a verb-phrase role (*XREADY can accept the transfer*), or a description's label (*Write address valid.*,
  *Ready indicator*; a table row naming one signal is its description). Not the data a validity qualifies (*presents
  valid data on its DAT_O lines*; *PNSE must be valid when PSEL is asserted*), not a signal beside the bearer (*that
  is, ATVALID HIGH*; *in response to STB_O*), not a denied role (*not ready*, *unable to complete the transfer*), not
  a choice (*indicates which byte lanes …*, *the read data chunks that are valid*). A role recalled from memory has
  no position, so it holds only where the text has no words of that role and the signal indicates or opens the
  sentence as subject. `is_role_label_word`: *Valid* is the word, only the spelling `VALID` the signal. Test fixtures
  that wrote a role as *XREQ valid timing* (a signal followed by its role word, which the corpus never writes: 0
  occurrences) state XREQ's validity and now read *XREQ indicates a valid request* (9 captions in Rust tests, 4
  kg-bench fixtures).
  **Measured over all 31 chains** (evidence dry-run against `b98ddf42f`'s): 24 identical; 7 move only
  `signal_semantic_hints`, their conflicts and the extraction manifest: 136 -> 97 hints (67 kept, 68 withdrawn, 29
  gained), conflicts 4 -> 1 (Wishbone `ACK_I`, which the document gives both roles). The six this leaf named are
  withdrawn, with TMC's `ATBYTES*`/`AFREADY*`/`FLUSHIN`, AXI's enum and descriptor tags (`ARSNOOP`, `AWCMO`, `ARPBHA`,
  `xMMUATST`, `xTAG`, `ASKSTOP`, `BROADCASTSHAREABLE`), APB5's `PxUSER`/`PNSE`/`PSTRB` (*must be valid when*), AHB's
  `HWSTRB`/`HADDR`/`HREADYOUT`, ADIv6's `CDBGPWRUPACK`/`TDO`, I2C's `SCL`, Wishbone's `ADR_O`/`DAT_I`; AXI `VALID` keeps
  valid-like only. Gained: AXI's indicators from their own words (`xVALID`, `xACTV`, `xMMUSSIDV`, `AWSTASHxIDEN`,
  `RCHUNKV`) and APB5 `PREADY` from *Ready.*. Without memory 126 -> 96. Rebuilt stage by stage: 36 signals lose a
  resolved role the text does not give, 6 AXI field-valid flags gain one; SemanticIR and IntentIR move only interface
  records and conflicts, two adapters drop their `semantic_signal_role_unresolved` residual (TMC 13 -> 12, AXI 73 -> 72), every `.isf` text
  byte-identical. **Declared residuals:** AXI `Y`, a key table's code read as a signal, keeps ready-like from *| Ready
  | Y |* (`.53`); AXI `AWTAGOP` gains ready-like from the stale memory alone (`.54`); AHB `HREADYOUT` has no hint left,
  since its only sentence denies the readiness.

- ID: `EXTRACTION-REPRESENTATION-GAPS.51`
  Status: `done` (`2026-10-21`; CODE; found by `.3.d.iii.d`; checklist below; opened `.55`)
  Goal: **a sentence defining the document's own notation is apparatus.** ADIv6's appendix E3 (*Pseudocode
  Definition*) explains the pseudocode language the document uses, and its sentences are read as protocol content:
  `.3.d.iii.d` read *The block of lines consisting of elsif and its indented statements is optional* as an optional
  feature (`option_0006`), and *This statement indicates a special case that replaces the behavior defined by the
  current pseudocode* is classed `normative_statement` five times and `derived_rule` once. **Reproduce:** statements
  on the 31 chains not classed `apparatus` that name *pseudocode*, *elsif*, *indented statements* or *indentation*:
  55, 46 of them ADIv6's (39 `source_fact`, 6 `normative_statement`, 1 `derived_rule`), AXI 6, ACE 2, CHI C2C 1.
  **Impact:** a notation rule becomes a requirement or an option the hardware is held to, and an obligation reader
  can take it. Acceptance: tell a section that defines the document's notation (pseudocode, typographic or
  numbering conventions) from one that uses it, by its heading and its own words rather than a document's name, file
  its sentences as apparatus, keep a pseudocode function that describes hardware behavior (*The pseudocode function
  EnterDormantState describes the function of the JTAG-to-DS sequence detector*) where it is, and measure every chain.
  (The census's *ACE 2* is AHB5's, `ihi0033_c`: its typographical conventions.)
  **Decided `2026-10-21`: the section decides, by its heading and its outline number.** A heading names the
  document's notation (`names_document_notation`) when it holds *convention(s)*, *notation(s)*, *typographic* or
  *typographical*, or names pseudocode as its topic (*Pseudocode Definition*, *About ARM pseudocode*, *Pseudo Code*)
  — not pseudocode put to use, which an *of*/*for* after the word marks (*A3.1.7 Pseudocode description of the
  transfers*). The converter gives every heading level 1, so the outline number decides membership (`outline_number`:
  `E3.2.1`, `1.6.2.3`, a chapter's `2`, an appendix's `E3`): a numbered notation heading covers the headings numbered
  under it and the unnumbered ones among them (*UNDEFINED*, *if … then … else …*), until a heading numbered outside
  it; an unnumbered one covers only itself. `document_notation_sections` marks them and every sentence there is
  `apparatus`, whatever class its words would give. Readers that interpret notation read every class, so the
  collective-name reader still reads *AxCACHE refers to both ARCACHE and AWCACHE* from AXI's conventions.
  **Measured over all 31 chains** (evidence dry-run against `4803a679d`'s): 11 identical; 20 documents move 447
  statements to `apparatus` (416 source facts, 23 normative, 5 conditional, 2 derived, 1 design decision), and 8
  evidence records leave: ADIv6's `elsif` option, its equivalence and conditional rule from *Integer constants …
  Hexadecimal integer constants are treated as positive*, USB 3.2's and USB4 inter-domain's conditional rules from
  their *Precedence* and *Reserved* keyword definitions (2 each), and Wishbone's *Array limits may also be shown within
  the parenthesis* permission. The leaf's 55 statements fall to 14: table-of-contents rows, the protocol sentences
  that use pseudocode (*EnterDormantState*, the claim-tag protocols, a cross-reference), and typographic items the
  converter made unnumbered headings. Rebuilt stage by stage: SemanticIR and IntentIR lose the 8 and the 35
  invariants and 29 constraints the notation sentences minted (ADIv6 20 / 18, USB 3.2 9 / 7, USB4 inter-domain 5 / 3,
  Wishbone 1 / 1: *Each of the following simple statements must be terminated with a semicolon*, *Shall is a keyword
  indicating a mandatory (normative) requirement*, the state-machine legend); ADIv6's adapter drops its equivalence
  residual, its optional and Wishbone's permission residuals count one fewer; every `.isf` text byte-identical.
  **Declared residuals:** an unnumbered conventions heading covers only itself, so a typographic item the converter
  made its own heading (AXI's *monospace*, ADIv6's *Colored text Indicates a link:*, *SMALL CAPITALS*) keeps its
  class; USB4's *1.2 Endian Conventions* is apparatus as a whole, its UUID-to-DWORD layout included, which no reader
  consumed; the obligation the *Reserved* and *N/A* definitions carry (*a reserved bit … shall be set to zero by the
  sender*) reaches no carrier (`.55`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.53`
  Status: `done` (`2026-10-21`; CODE; found by `.50`; checklist below)
  Goal: **a key table's codes are not signal names.** AXI's `table_0265` (*Table B2.1: Key to signals table*, header
  `Code | Manager interfaces | Subordinate interfaces`, rows `Y`, `YM`, `YS`, `O`, `NS`, `N`) is classed
  `signal_description`, so `collect_signal_names_from_tables` admits its codes as signal names; the older AXI edition's
  `table_0270` (*Table G2-1 Key for Signal Matrix*, `Code | Meaning`: `M`, `S`, `I`, `V`, `Y`, `N`, `O`, `OO`, outside
  the 31 chains) is classed the same. **Reproduce:** over `generated/source_ir/*/source_ir.json`, the
  `signal_description` tables whose first header is *code*, *key*, *symbol* or *legend*, or whose caption says *key
  to* or *legend*: the two AXI key tables and USB 3.2's state-machine key (`table_0208`, `Key | Description`, entries
  such as `polling.timeout`); the other two are a genuine signal table and a timing table. **Impact:** the
  hint reader targets the codes: `Y` -> `handshake_ready_like` from *| Ready | Y | AXI channels use READY flow control
  signals. |* (`statement_0562`), and `O` was tagged from presence rows until `.50`; every other reader of
  `collect_signal_names_from_tables` (the hint catalog, the declared-signal set, the visual-motif names) sees them.
  Acceptance: tell a key table (a column of codes that the other columns define) from a signal-description table by
  its own structure, not by its caption words alone, keep every genuine signal table, and measure every chain.
  **Decided `2026-10-21`: a first column headed *Code* or *Key* alone holds codes, not signals.**
  `table_keys_codes_rather_than_signals` joins `table_states_a_property_rather_than_a_signal_inventory` in
  `should_treat_table_as_top_level_signal_description`, so every reader of signal tables (the name catalog, the hint
  reader, the actor columns) passes such a table by; the caption's *signals* no longer makes it an inventory. A
  *Symbol* column stays one: JESD's *Signal List and Description* heads its names so. Over every SourceIR the rule
  matches the two AXI key tables and USB 3.2's state-machine key, and no signal table. **Found while measuring:**
  with the codes gone, AXI's presence rows (*| ARREADY | AXI_Transport == Ready | O | Y | … |*) name one signal each,
  so `.50`'s label rule read *Ready* closing the condition cell as the row's label — six ready-like hints right by
  accident; a value compared in an expression (after `=`, `≠`, `<`, `>`) is now not a label.
  **Measured over all 31 chains** (evidence dry-run against `84d060191`'s): 30 identical; AXI moves only
  `signal_semantic_hints` (57 -> 56, the `Y` hint) and its extraction manifest; USB 3.2's key never reached the
  catalog (its keys are not identifier-shaped). AXI rebuilt stage by stage: SemanticIR, IntentIR, the adapter and
  every `.isf` text byte-identical (`Y` was in no interface).

- ID: `EXTRACTION-REPRESENTATION-GAPS.54`
  Status: `done` (`2026-10-21`; DESIGN; found by `.50`; opened `.54.a`)
  Goal: **a remembered hint role does not outlive the reading it was learned from.** The local corpus memory
  (`generated/prior_memory/corpus_memory.json`, learned `2026-08-12` by `learn-priors`, untracked) holds 83 semantic
  phrase priors harvested from that day's hints, among them the readings `.50` withdraws (*Enum ARSNOOP CLEANINVALID =
  9* -> valid-like; *Indicates that the transaction has already undergone PCIe ATS translation* -> ready-like). `.50`
  gates a recalled role by the current text, which leaves one: AXI `AWTAGOP` -> `handshake_ready_like` from
  `statement_3364` (*… The signal AWTAGOP indicates the tag operation …*), learned from the word *already*. Nothing
  relearns the memory when a reader changes, and every evidence build consults it when present. **Reproduce:**
  dry-run every chain with and without `--prior-memory` (a missing path): 97 and 96 hints, the difference `AWTAGOP`.
  **Impact:** an advisory store re-asserts a role the reader no longer gives, and a checkout without the file reads
  the corpus differently. Acceptance: decide when the memory is relearned (after a reader change, from rebuilt
  IntentIR) or whether a recall must also pass the current reader's own reading of the phrase it was learned from;
  measure every prior family's drift, not only semantic phrases, before relearning.
  **Measured `2026-10-21`.** The store is ADR 0036's fixed point (SHA-256 `a416cc8b…`), and it is reviewed authority:
  `doctrine/production_genericity/behavioral_qualification.json` and the retained holdout evidence pin that hash.
  (1) **Its refresh cannot run.** ADR 0036 decision 4 refreshes the store by validating its declared inputs,
  relearning, replaying every affected chain and relearning until byte-stable; 8 of its 13 inputs (AXI-Stream, CXS,
  ATP, GFB, HBM2, NVMe, both TileLinks) are now legacy schema-1 IntentIR outside the chain set, and `learn-priors`
  refuses them (*IntentIR schema version 1 is legacy/proofless and inspection-only*). (2) **What it does today.** An
  evidence dry-run of all 31 chains without the store differs from one with it in two records: AXI's `AWTAGOP`
  ready-like hint (from the old reader's *already*) and one duplicate AHB5 declaration it suppresses (*Signal HCLK is
  width 1.*, which renumbers 25 later synthesized statements); relations, polarities, constraints and every other
  family are identical. (3) **What a relearn would do.** A store learned from the 31 proof-carrying chains drifts
  in every family (actor taxonomy 29 -> 24, semantic phrases 83 -> 64, temporal 443 -> 1,140, table shapes 0 -> 128,
  visual motifs 1,251 -> 1,430, negative knowledge 11 -> 13, profiles 2 -> 3) and moves four chains' evidence
  (actor relations +27/−24 in one, polarities, table declarations, 8 table hints and 4 constraints in AXI, 821
  statement positions).
  **Decided:** a recalled role is already bounded by the current text (`.50`: it holds only where the text has no
  words of that role and the signal indicates or opens the sentence), so the store is not relearned piecemeal by a
  reader change. It is refreshed by the protocol ADR 0036 declares — re-declared inputs that validate today, the
  convergence loop, and a replay of the qualification evidence that pins it — as one owned transaction, `.54.a`.
  Until then `AWTAGOP` stays the declared residual.

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a`
  Status: `open` (`2026-10-21`; CODE; opened by `.54`)
  Goal: **the corpus memory's refresh runs as ADR 0036 declares it.** The store pinned by the behavioral
  qualification (`a416cc8b…`) was learned from 13 IntentIR inputs, 8 of which are now legacy and refused by
  `learn-priors`, so its declared refresh cannot run and it still recalls readings the current reader withdraws (AXI
  `AWTAGOP` ready-like). **Reproduce:** `specforge learn-priors` over the store's `source_artifacts` fails on the
  first legacy input. **Impact:** the store cannot follow any reader change, and a checkout without it reads two
  records differently. Acceptance: re-declare the learning inputs as the IntentIR that validates today (the 31
  proof-carrying chains, or a stated subset), run ADR 0036's convergence loop to a byte-stable store, replay every
  affected chain and the behavioral holdout evidence, re-pin the hash in `behavioral_qualification.json` and the
  holdout evidence with the measured drift, and decide what triggers the next refresh.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.50` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — every hint of the 31 chains listed with its sentence before any code (136: prose 85,
  tables 47, captions 4); the corpus counted for the phrasings the rule decides (*asserted when … valid*: 0 signal
  statements; a signal followed by *valid*/*ready*: 0); a dry-run without corpus memory for HEAD and the change.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `infer_signal_semantic_tags_from_description`
  matched role phrases as substrings and every reader kept every tag; the stripping folded *Valid* into `VALID`. New
  `semantic_role_phrase_spans`, `is_role_label_word`, `SemanticRoleSubject`, `retain_borne_semantic_roles`,
  `SemanticRoleReading`, wired into the table, prose and visual readers. RED, the file restored byte-identically after
  each (14 mutations, each failing a named test): whole words, the gate, denial, *which*, the relative choice, the
  preposition, the label, the memory guard, the label word, the subject, the means, the clause, *asserted when*, a row
  naming two signals.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 24 identical; the 7 moved chains' hints listed one by
  one (67 kept, 68 withdrawn, 29 gained); the 7 rebuilt stage by stage: interface records, conflicts and two
  `semantic_signal_role_unresolved` residuals move, every `.isf` text byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical
  to `b98ddf42f`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 171 / 1,883 passed, 0 failed. `flow_census.json` rolled (+18 functions, +213 decision sites, +533 helper edges,
  +2 macros); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — English role, indicating, negating and determiner words and the text's own signal
  mentions; no document or signal name. Fixtures that wrote *XREQ valid timing* rewritten to the indicating form (9
  captions, 4 kg-bench fixtures), the corpus never writing the old form.
- [x] **LOCKSTEP** — book `domain/handshake-semantics.md` (*Who bears the role*, and the prior rule); KM
  `a-hint-role-counts-only-for-the-signal-that-bears-it`. Production rules replaced: the substring role test (its
  phrases kept, matched whole) and `contains_ready_like_acknowledgment_phrase` (folded into `READY_ROLE_PHRASES`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.55`
  Status: `open` (`2026-10-21`; DESIGN; found by `.51`)
  Goal: **an obligation a keyword's definition carries reaches a carrier.** USB 3.2 (*1.8.2.6 Reserved*) and the USB4
  inter-domain specification (*1.6.2.6 Reserved*) define the keyword *Reserved* and with it a rule on every field it
  marks: *A reserved bit, byte, word or field shall be set to zero by the sender and shall be ignored by the
  receiver. Reserved field values shall not be sent by the sender and, if received, shall be ignored by the
  receiver*; both define *N/A* with *shall not be checked or used by the recipient*. Until `.51` each Reserved
  definition minted a conditional rule that read nothing of it (antecedent *otherwise stated, shall not be utilized
  …*, consequent *shall not*); since `.51` the definitions are apparatus, as the rest of their conventions section.
  **Reproduce:** `statement_1200`/`1195` (USB 3.2) and `statement_0207`/`0204` (USB4 inter-domain), class
  `apparatus`. **Impact:** a document-wide rule on reserved and not-applicable fields reaches no record, so no field
  a register or packet table marks *Reserved* carries "send zero, ignore on receipt". Acceptance: decide whether a
  keyword definition's obligation binds the fields the document's tables mark with that keyword (and through which
  record: a field property, a constraint, a residual), read it from the definition rather than a document's name,
  and measure every chain's reserved and N/A fields.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.51` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — the leaf's census (55 statements naming pseudocode, *elsif*, *indented statements* or
  *indentation*); every heading of the 31 chains holding *convention*, *notation*, *pseudocode* or *typograph*; each
  such section's statements by class and every evidence record citing one, before any code (448 statements, 40
  records, 32 of them AXI's collective names); the USB keyword definitions read in full.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: statements were classed by their words
  alone, and the only section rule (Layer A) demotes a normative sentence in a boilerplate section to a source fact.
  New `outline_number`, `names_document_notation`, `document_notation_sections`, read in the statement loop. RED, the
  file restored byte-identically after each (7 mutations, each failing a named test): the wiring, the conventions
  words, pseudocode put to use, the numbered family, its unnumbered members, the appendix number, a chapter's number.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 11 identical; 447 statements to `apparatus` in 20,
  8 records gone, each listed; the 20 rebuilt stage by stage: 35 invariants and 29 constraints the notation sentences
  minted leave SemanticIR and IntentIR, three adapter residuals follow, every `.isf` text byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: 10 of 11 seed reports
  byte-identical to `4803a679d`'s (temp paths masked); `seed_swd` reports ADIv6's conditional rules 103 -> 102 and its
  enrichment candidates 318 -> 306, the notation sentences, every score line unchanged. `kg-bench` 158/158. `cargo fmt
  --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace
  --lib --exclude specforge-production-graph` 474 / 171 / 1,886 passed, 0 failed. `flow_census.json` rolled (+3
  functions, +11 decision sites, +8 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — English heading words and the headings' own outline numbers; no document, section
  number or signal name.
- [x] **LOCKSTEP** — book `pipeline/presence-and-assumptions.md` (*A section that defines the document's notation*);
  KM `a-section-defining-the-documents-notation-is-apparatus`. No production rule deleted or replaced.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.53` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — every `signal_description` table over all SourceIRs whose first header is *code*,
  *key*, *symbol*, *legend* or *abbreviation*: the two AXI key tables, USB 3.2's key, two JESD signal and timing
  tables; where AXI's codes surface in its evidence (3 hint targets).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `should_treat_table_as_top_level_signal_description`
  took a caption naming *signals* as an inventory, so `collect_signal_names_from_tables` admitted the key's codes. New
  `table_keys_codes_rather_than_signals`; `labels_role` refuses a compared value. RED, the file restored
  byte-identically after each (4 mutations, each failing a named test): the wiring, the key words, any header word,
  a compared value read as a label.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 30 identical, AXI −1 hint (`Y`); without the
  compared-value refusal the same run gained six presence-row hints, measured before it was added. AXI rebuilt stage
  by stage: every later stage and `.isf` text byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `84d060191`'s (temp paths masked). `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 171 / 1,887 passed, 0 failed. `flow_census.json` rolled (+1 function, +4 decision
  sites, +3 helper edges, +1 macro); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the table's own first header word; no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/evidenceir.md` (a signal inventory by structure) and `domain/handshake-semantics.md`
  (a compared value); KM `a-hint-role-counts-only-for-the-signal-that-bears-it` amended. No production rule deleted
  or replaced.
