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
  Status: `open` (`2026-10-20`; CODE; found by `.3.d.iii.d`)
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

- ID: `EXTRACTION-REPRESENTATION-GAPS.53`
  Status: `open` (`2026-10-21`; CODE; found by `.50`)
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

- ID: `EXTRACTION-REPRESENTATION-GAPS.54`
  Status: `open` (`2026-10-21`; DESIGN; found by `.50`)
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
