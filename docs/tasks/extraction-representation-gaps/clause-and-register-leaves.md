# EXTRACTION-REPRESENTATION-GAPS — clause and register leaves

- Part ID: `clause-and-register-leaves`
- State: `active`

The leaves `.39` to `.43`: how a clause, a condition and a register field are read. They continue here, and so does
every other new leaf outside the `.3` family, after `recent-leaves` reached its mandatory rollover (`2026-09-29`).

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.39`
  Status: `done` (`2026-09-29`; CODE; SpecForge defect; found by `.3.c.ii`; checklist below)
  Goal: **a condition marker that opens a sentence opens that sentence's condition.** `split_conditional_sentence`
  reads a statement that does not itself open with a condition as *Y when Z*, split at its first ` when `. In a
  statement of several sentences that ` when ` can open a later sentence. ADIv6's *CSYSPWRUPREQ is the signal from the
  debug interface to the power controller. … When CSYSPWRUPREQ is asserted, the power controller … then asserts
  CSYSPWRUPACK …* gives the earlier sentences as the consequent and the whole *When* sentence as the antecedent. So
  the rule's consequent signal is the first known signal of the wrong sentence: today `CSYSPWRUPACK`, right only
  because it is the one known signal there, and `CSYSPWRUPREQ` once `.3.c.ii` makes that wire known. Measured on the
  persisted IR of the 31 current chains: 108 of 1,130 conditional rules, in 12 documents (USB 3.2 41, RISC-V IOMMU 11,
  RISC-V Debug 10, ADIv6 8, AXI 5, Wishbone 5, …). Acceptance: a marker that follows a sentence end is read as its
  sentence's leading condition; replay every chain and read every rule that moves, and what it lowers to.
  **Done.** In `split_conditional_sentence`'s embedded path, a marker whose preceding text ends in `.`, `:` or `;`
  opens a sentence: that sentence is split as a statement-initial one would be. A sentence the leading path cannot
  split is refused, as it is at the start of a statement. **Measured:** EvidenceIR dry-run of the 31 chains with the
  HEAD and new binaries: 10 identical, 21 move and only in `conditional_rules`, 105 rule sources. 93 keep their signal
  and action and their antecedent narrows to a strict prefix, the condition as written. 8 take their consequent
  signal from their own sentence or have none (AXI `BROADCASTATOMIC`, `AWMMUVALID` and Wishbone `RST_O` had borrowed
  one; ADIv6 `nSRST`, I2C `SCLH`, Wishbone `CLK_I`, AXI `RCHUNKCHK` gain one). 1 takes its action from its own sentence
  (RISC-V IOMMU *must be 0*). 6 are refused (TMC's two *When asserted:* list heads, Wishbone's two glossary
  sentences, USB 3.2's two comma-less conditions). The 21 chains rebuilt stage by stage: SemanticIR and IntentIR move
  only in the carried `conditional_rules`; no `.isf` text or lowering status moves; five adapters' listed
  obligations follow the rules (TMC's junk `CSYSACK`/`CSYSREQ` gone, AXI 120 → 118, ADIv6 and I2C one more listed,
  Wishbone renumbered). A consequent still runs to the end of its statement, so ADIv6's `nSRST` comes from a later
  sentence: `.39.a`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.39 — a condition marker that opens a sentence opens that sentence's condition`

- ID: `EXTRACTION-REPRESENTATION-GAPS.39.a`
  Status: `done` (`2026-09-29`; CODE; SpecForge defect; found by `.39`; checklist below)
  Goal: **a rule's consequent ends at its sentence.** `split_conditional_sentence` gives a fronted condition's
  consequent as the rest of the statement, and the consequent signal is the first known signal in it. ADIv6's *When
  nSRST is asserted, the entire system must be reset, including the debug domain. However, … Only the non-debug domain
  is held in reset while nSRST is asserted* takes `nSRST` from the third sentence. Measured on the 31 chains after
  `.39`: 254 of 1,124 rules have a consequent that runs past its sentence, and about 9 would change consequent signal
  if it were bounded (AHB `HTRANS` 3, AXI `BCOMP`, AHB `HRDATA`, I2C `SCL`, `SDA`, ADIv6 `nSRST`, Wishbone `STALL_I`).
  Acceptance: bound the consequent at its sentence end, replay every chain, read every rule that moves, and score the
  AHB gold.
  **Done.** `split_conditional_sentence`'s fronted path reads one sentence: the condition's boundary (the modal's, or
  the first comma, or *then*) and the consequent both end at the sentence end. `sentence_end_in` finds that end, and a
  period right after a word holding `_` is a register's field separator, not a sentence end: USB4's *If
  ADP_USB3_GT_CS_0. Gen X Adapter Coupled bit is set …* lost its rule to the first version of this leaf. **Measured:**
  EvidenceIR dry-run of the 31 chains with the HEAD and new binaries: 22 identical, 9 move only in `conditional_rules`,
  15 rules, all read. 9 consequent signals borrowed from a later sentence become none (AHB `HTRANS` ×3 and `HRDATA`,
  AXI `BCOMP`, I2C `SDA` and `SCL`, ADIv6 `nSRST`, Wishbone `STALL_I`). 3 antecedents end with their sentence (RISC-V
  IOMMU's *then* split, two USB 3.2). 3 rules whose sentence has no boundary are refused, as the same sentence alone
  is. Rebuilt stage by stage: only the carried rules and the adapters' listed obligations move (AHB 23 → 19, I2C 2 →
  1, AXI 118 → 117, ADIv6 2 → 1, Wishbone 1 → 0); no `.isf` text moves.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.39.a — a rule's consequent ends at its sentence`

- ID: `EXTRACTION-REPRESENTATION-GAPS.40`
  Status: `done` (`2026-09-29`; MEASUREMENT + DECISION; found by `.3.c.ii.a`)
  Goal: **`seed_swd`'s one gold constraint scores 0 where it once scored 1.000.** The gold labels ADIv6's
  *CSYSPWRUPACK must be asserted for as long as CSYSPWRUPREQ is asserted* `must_be_asserted`. SpecForge records
  `must_be_high`, because the section states *All signals that are described in this section are active-high, so assert
  denotes taking the signal HIGH* and polarity refinement applies it. So at `be58b45b` the seed scores `signal_constraint`
  tp=0 fp=1 fn=1, where `WIRE-BASED-100` and `LLM-PRIMARY-PROMOTION` recorded 1.000. `.3.c.ii.a` removes the prediction
  (fp 1 → 0) until `.3.c.ii` restores it. Acceptance: find the commit that moved it, and decide between refining
  the gold to the level the document states (the `.25.c` precedent) and scoring a refined kind as its unrefined form;
  record which, with the seed's score on each side.
  **Measured.** `.17` (`fc04dd39`) moved it. Built from `fc04dd39^` and from `fc04dd39` in a scratch worktree, each
  binary dry-run on ADIv6's current SourceIR gives `CSYSPWRUPACK` no polarity and `must_be_asserted`, then `active_high`
  and `must_be_high`. Neither old binary can run `eval-extraction` on today's persisted EvidenceIR, whose proof
  topology has moved on, so the kind is read from their EvidenceIR. `.17` scored the six AMBA seeds, not `seed_swd`.
  **Decision: refine the gold**, as `.25.c` did for APB's hold. The document states the level: *All signals that are
  described in this section are active-high, so assert denotes taking the signal HIGH* (`statement_1252`). The
  scorer's key stays an exact identity, because accepting a refined kind as its unrefined form would need the scorer
  to replay polarity. HEAD's binary (`ea4fac4b`) scores `signal_constraint` tp=0 fp=1 fn=1 on the old gold and
  1.000 (tp=1) on the new one; the other ten seeds are untouched. The gold's `label_note` cites the section and this
  leaf.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.40 — seed_swd's gold states the level the document states`

- ID: `EXTRACTION-REPRESENTATION-GAPS.41`
  Status: `done` (`2026-09-29`; DESIGN + CODE; found by `KG-ISF-COMPLETENESS.1g`; checklist below)
  Goal: **a register field bound to a wire carries the binding.** ADIv6 states which programmers'-model bit is which
  wire: *This field drives the nSRSTOUT signal* (`SRST_OUT`), *This bit controls the CSYSPWRUPREQ signal* (CTRL/STAT
  bit 30), *This read-only field returns the logical AND of the PORTCONNECTED signals from all ports*, and *Table
  B2-7 Debug Port programmers' model* maps CTRL/STAT bits 28-31 to the four power-up wires. The relation reader made
  the sentences' `field` and `bit` into actors (`KG-ISF-COMPLETENESS.1g` refuses them), and no record carries the
  binding. Acceptance: decide the carrier (the field's bound signal and its direction: the field drives the wire, or
  the wire sets the field), read it from the register the sentence describes, and measure every chain.
  **Decision.** The carrier is `RegisterFieldRecord.signal_bindings`: the signal as printed, and whether the field
  `drives` it or `reflects` it (omitted when empty). The binding sentences are not in the fields' description cells.
  They follow each field's own `.10g` heading (`SRST_OUT, bit[0]`), so `fill_field_signal_bindings` reads the body
  after such a heading, up to the next heading, into the one field with that name and bit range. The phrasings are:
  *drives*/*controls* for a field that drives the wire; *indicates the status of*/*returns the logical AND of*/*returns
  the value of* for one that reflects it; the wire is the identifier before *signal(s)*. Table B2-7's
  bit-to-wire mapping is converter-swapped and is not read here. **Measured:** EvidenceIR dry-run of the 31 chains: 30
  identical. ADIv6 gains 10 bindings, only in `register_records`: `CTRL/STAT` `CSYSPWRUPREQ`, `CDBGPWRUPREQ`,
  `CDBGRSTREQ` drive and their three acknowledges reflect (after `.41.a`); `CSW@JTAG-AP` `TRST_OUT` drives `TRST*`,
  `SRST_OUT` drives `nSRSTOUT`, and `PORTCONNECTED` and `SRSTCONNECTED` reflect theirs. Rebuilt stage by stage, only the
  carried records move; no `.isf` or residual moves.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.41 — a register field bound to a wire carries the binding`

- ID: `EXTRACTION-REPRESENTATION-GAPS.42`
  Status: `done` (`2026-09-29`; CODE; SpecForge defect; found by `KG-ISF-COMPLETENESS.1g`; checklist below)
  Goal: **a character reference's semicolon is no clause boundary.** The converter writes `<`, `>`, `&` and a
  cell's `|` as `&lt;`, `&gt;`, `&amp;`, `&#124;`, and at least twelve readers in `ir/evidence.rs` end a clause or a
  sentence at `;`. CoreSight TMC's *When the space in the FIFO is less than or equal to this value, that is, Fill
  level &gt;= MEM_SIZE - BUFWM, the FULL output is pulled HIGH* is split after `&gt`, so the level binding *the FULL
  output is pulled HIGH* loses its condition. The unconditional constraint is listed today only because junk actors
  hold `FULL`; once `.1g` removes them it lowers into TMC's emitted `.isf` as `(FULL 1)`. 281 statements on the 31
  current chains hold a reference (ADIv6 173, USB 3.2 47). Acceptance: one shared boundary rule that keeps a
  reference whole, used by every clause and sentence splitter; replay every chain and read every fact that moves.
  **Done.** `closes_character_reference` recognises the `;` that ends `&name;` or `&#digits;` (the converter escapes
  every literal `&`, so no other `&…;` survives conversion), and `split_clauses`, `find_clause_boundary` and
  `rfind_clause_boundary` end a clause at a delimiter only when it is not such a `;`. All twelve splitters use them,
  and so does `.39`'s sentence-end test. **Measured:** EvidenceIR dry-run of the 31 chains with the HEAD and new
  binaries: 30 identical. TMC loses the one wrong fact, the unconditional `FULL` must-be-HIGH constraint, and with it
  an unconditional temporal rule on the junk actor `DEVID register`, an invariant and an adapter residual line. No
  `.isf` text moves. The sentence now yields no constraint at all, because the level-binding path reads a binding
  only before a condition marker: `.42.a`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.42 — a character reference's semicolon is no clause boundary`

- ID: `EXTRACTION-REPRESENTATION-GAPS.42.a`
  Status: `done` (`2026-09-29`; CODE; found by `.42`; checklist below)
  Goal: **a level binding after a fronted condition is read with that condition.** The level-binding path
  (`logic_level_bindings` over `text_before_condition_marker(clause)`) reads a binding only before its condition
  marker, which serves *X is HIGH when Y*. CoreSight TMC's *When the space in the FIFO is less than or equal to this
  value, …, the FULL output is pulled HIGH* has its binding after a fronted condition, so since `.42` it yields
  nothing: not a constraint, and not a conditional rule, since the statement is not classified as one. Acceptance:
  read a fronted condition's main clause through `main_clause_of_fronted_conditional`, keep the condition on the
  record, census how many statements the path now reads, and read what each lowers to.
  **Done.** `fronted_condition_split` splits a clause that opens with a condition into (condition, main clause): with a
  modal, at `main_clause_of_fronted_conditional`'s boundary; without one, at the first comma, as
  `split_conditional_sentence` does. `extract_dynamic_signal_constraints` reads the level binding from the main clause
  and gives the record that condition, never the whole clause. **Measured:** EvidenceIR dry-run of the 31 chains with the
  HEAD and new binaries: 30 identical; TMC gains `FULL` must be HIGH under *the space in the FIFO is less than or equal to
  this value*. Rebuilt stage by stage: SemanticIR carries it as a rule whose condition reads no guard, and the adapter
  lists it (*the rule's stated condition … was not read into a guard*) rather than lowering it. No `.isf` text moves, and
  FSMGen `--strict --check` still reports 0 diagnostics.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.42.a — a level binding after a fronted condition keeps that condition`

- ID: `EXTRACTION-REPRESENTATION-GAPS.43`
  Status: `done` (`2026-09-29`; CODE; SpecForge defect; found by `.39.a`; checklist below)
  Goal: **a rule's consequent signal is its obligation's subject, not the first signal its consequent names.**
  `extract_conditional_rules` takes the first declared signal anywhere in the consequent. I2C's *When both SCLH and
  SCL become HIGH …, transistor TR2 opens to separate the SCLH and SCL lines* gets `SCLH`, a signal the consequent
  names only as an object, and AXI's *…, the RCHUNKCHK signal covers both of these signals, so RCHUNKNUM and
  RCHUNKSTRB must be the same width* gets `RCHUNKCHK` rather than the two signals the obligation constrains.
  Acceptance: read the subject of the consequent's obligation (the phrase before its modal or finite verb), keep none
  where the subject is not a signal, replay every chain and read every rule that moves, and what each lowers to.
  **Done, for obligations.** When the consequent has a modal, `extract_conditional_rules` takes the first declared
  signal of the clause before it (after that clause's last comma, so a coordinated subject keeps its first member).
  With none there, the subject is an agent, and it takes the first declared signal after the modal: *the Manager must
  assert HREADY* gives `HREADY`. A consequent without a modal keeps the first signal it names, which is usually the
  one its verb acts on (*… and then asserts CSYSPWRUPACK*); I2C's *transistor TR2 opens to separate the SCLH and SCL
  lines* is not an obligation and still gives `SCLH`: `.43.a`. **Measured:** EvidenceIR dry-run of the 31 chains: 30
  identical; AXI moves two rules. *…, so RCHUNKNUM and RCHUNKSTRB must be the same width* gives `RCHUNKNUM`, not
  `RCHUNKCHK`. *… can use any value for ARRP, but all DVM Complete messages on an interface must use the same RP*
  gives none, since the obligation binds messages and `RP`, which is no declared signal; `ARRP` came from the
  permission clause. A first version that took the last signal before the modal gave `BRESP` for *AWSNOOP and BRESP
  must be wide enough*, arbitrarily; it was not landed. Rebuilt stage by stage: AXI lists 116 obligations instead
  of 117, `.isf` text unchanged and FSMGen-strict-clean.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.43 — a rule's consequent signal is its obligation's subject`

- ID: `EXTRACTION-REPRESENTATION-GAPS.43.a`
  Status: `done` (`2026-09-29`, CODE; found by `.43`; checklist below)
  Goal: **a descriptive consequent's signal is the one its verb acts on.** A consequent without a modal takes the first
  declared signal it names. That is right when the signal is the verb's object (*the power controller … asserts
  CSYSPWRUPACK*) and wrong when the signal sits inside another phrase (I2C *transistor TR2 opens to separate the SCLH
  and SCL lines* gives `SCLH`). Acceptance: read the object of the consequent's relation verb, keep none where no verb
  acts on a signal, replay every chain, and read every rule that moves.
  **Measured and shipped (`2026-09-29`).** 44 of the 1,121 rules on the 31 chains name a consequent signal. A first
  reading (a signal counts only at its clause's start or after a signal verb) moved 9, and 3 of those losses were wrong:
  *It is recommended PWAKEUP be deasserted*, *The strobe input [STB_I]*, *the SLAVE's [ACK_O] signal MAY be tied*. A
  signal now counts where no verb-shaped word comes before it in its clause, where a *be* or modal follows it, or after
  a verb acting on a level. No signal counts inside a clause a condition marker opens, which also clears AXI's two
  *If AWIDUNQ was asserted, the ID can be reused* rules. Six rules move, each read and each correct.

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.a`
  Status: `done` (`2026-09-29`; CODE; SpecForge defect; found by `.41`; checklist below)
  Goal: **ADIv6's `CTRL/STAT` fields belong to `CTRL/STAT`, not to `ABORT`.** The document lays its fields out as `.10g`
  headings (`CSYSPWRUPACK, bit[31]` … `ORUNDETECT, bit[0]`) under *B2.2.3 CTRL/STAT, Control/Status register*. The
  reader does not take that heading as a container, so all 16 fields are read into the register before it: `ABORT`
  holds 21 fields, its own 5 plus `CTRL/STAT`'s 16, two of them at bit 4. The `CTRL/STAT` record, named by
  `table_0044`, has none. `.41`'s first replay attached the power-up bindings to `ABORT` for that reason. Acceptance:
  find why the container heading is refused (the `/` in its name is the lead), let the reader take it by its own
  rule, fill `CTRL/STAT`'s record as `.10g` fills a register named elsewhere, and replay every chain.
  **Done.** `is_message_container_name` refused a name holding `/`, so the container heading was missed. It now
  accepts `/` joining the parts of one name; the rest of the rule (letters all upper case, or an `_`) is unchanged.
  **Measured:** EvidenceIR dry-run of the 31 chains: 30 identical. ADIv6's `ABORT` keeps its 5 fields; `CTRL/STAT`
  (`table_0044`'s record) gains its 16 by `.10g`'s fill-in rule; the other heading registers' ids shift by one, since one
  more container exists. Rebuilt stage by stage: only `register_records` move, and the blocked adapter's text gains
  `ctrl_stat`'s 16 fields and `abort`'s 5. That had failed closed on two fields at bit 4. `abort`'s width goes from 32
  to 5, because `register_size_from_fields` takes the highest field's bit plus one and the 32 had come from
  `CTRL/STAT`'s fields: `.41.b`.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.41.a — CTRL/STAT's fields belong to CTRL/STAT`

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.b`
  Status: `done` (`2026-09-29`; MEASUREMENT + DECISION + CODE; found by `.41.a`; checklist below)
  Goal: **a register's width is its stated width, not its highest field.** `register_size_from_fields` sets
  `size_bits` to the highest field bit plus one for heading and table registers, so a register whose top bits are
  reserved reads narrower than it is: ADIv6's `ABORT` (fields to bit 4) is emitted as `(var abort (width 5))`.
  Measure how many records take their width this way where the document states another width, or none, and
  decide between the stated width, none (the adapter's default), and the field extent kept as a lower bound.
  **Measured (`2026-09-29`).** On the 31 current chains, 201 registers have located fields and every one's width is
  its field extent: `size_bits` is either that inference or unset, and none states a different width. 13 are not a
  whole number of bytes (ADIv6 `ABORT` 5, `DEVID` 5, `AUTHSTATUS` 10, `CFG` 20, `CLAIMSET` 2, …; RISC-V IOMMU 26, 4,
  17, 18, 9). RISC-V IOMMU states its registers' sizes in *Table 15. IOMMU Memory-mapped register layout* (`size` in
  bytes), but its field-table registers are unnamed (`register_table_0033`), so nothing joins them. **Decision:**
  `size_bits` is a width the document states, never one inferred from the fields: `PDF-VARIANT-DIGESTION.2c`'s
  fill-in is withdrawn at its five sites. An unset width is what `registers_with_unresolved_width` reports as a
  completeness gap. The adapter still needs a width, so it keeps the field extent, and it lists that choice as a
  residual, as `.7.d` lists a chosen reset kind.
  **Done.** The five fill-in sites in `ir/evidence.rs` (the bit-table, heading and field-table readers, the
  register-map fill-in and the fragment merge) record no width, and `register_size_from_fields` is gone. The adapter
  lists every register whose width it chose (`isf_register_width_not_stated`). **Measured:** EvidenceIR dry-run of the
  31 chains: 27 identical; ADIv6, CoreSight TMC, OpenCAPI discovery and RISC-V IOMMU lose exactly 201 inferred widths
  and nothing else. Rebuilt stage by stage: register records carry no width, and the four adapters and RISC-V
  Debug's list their chosen widths. RISC-V Debug's 44 registers had none before, so its EvidenceIR does not move. No
  `.isf` text moves: the adapter computes the same widths.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.41.b — a register's width is its stated width, not its highest field`

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.c`
  Status: `done` (`2026-09-29`, CODE; found by `.41.b`)
  Goal: **a register the document sizes in its layout table takes that size.** RISC-V IOMMU's *Table 15. IOMMU
  Memory-mapped register layout* (`offset | name | size | description`) states each register's size in bytes
  (`capabilities` 8, `fctl` 4, …). Its field tables become registers named `register_table_0033` and so on, so nothing
  joins them to that table, and since `.41.b` they carry no width. Acceptance: find how each field table names its
  register (a heading, a caption), name it, join the layout's size by name and offset, and measure every chain.
  **Measured (`2026-09-29`).** Each field table follows a heading that names its register in a bare parenthetical
  (`6.6. Command-queue base ( cqb )`). `EXTRACTION-GAP-FIX.3` names a register from a heading only when the
  parenthetical also holds a hex address, so none is named. Placing a table by page is wrong: `ddtp`'s field table sits
  on page 70 below `cqb`'s heading, and `cqh` and `cqt` share page 71. SourceIR records no reading order for a table,
  but the ingest numbers every item it yields in one sequence and records the number only on text, so each figure and
  table leaves a gap. On this document the 196 gaps are exactly the 196 visual assets, in order, each between pages that
  bracket its own. The layout table's `Size` states no unit. Its offsets advance by the size (0+8, 8+4, 12+4, …), so
  the unit is the offset's, bytes. Across the corpus 161 tables have an offset, a name and a size or width column, in 13
  documents; only this one is a current chain, and its 33 register records join no layout row by name.
  **Decision.** A register takes a layout row's name and width only when two witnesses agree: the heading nearest before
  its table in reading order names it in a bare parenthetical that the layout table lists, and its fields (with a split
  table's fragments together) end exactly at the width the row states. The unit is the table's own (a `bit`/`byte` word
  in the header, `N-bit` in the cell), or else whichever of bytes or bits most of its offsets advance by. `msi_cfg_tbl`
  shows why the extent is required: its row is a 256-byte table of entries, and the field table under its heading is one
  64-bit entry. A register another reader already named is not joined: `.41.d`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.41.d`
  Status: `open` (`2026-09-29`; blocked on re-ingest; found by `.41.c`)
  Goal: **a register already named takes the width its summary table states.** GIC-600, CoreSight SDC-600, MMU-700 and
  ACC list their registers in summary tables (`Offset | Name | Type | Width | …`, `Width` `32` or `32-bit`), and 68 of
  their register records carry exactly a listed name (GIC-600 15, SDC-600 4, MMU-700 48, ACC 1). `.41.c` joins only a
  field table its heading names, because all four documents are legacy: their SourceIR is inspection-only
  (`specforge evidence --dry-run` refuses it), so this join has no case that can be run. Acceptance: once any of them is
  re-ingested, join a named register to its row by exact name under `.41.c`'s unit and extent tests, and measure the
  document before and after.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.39` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 `statement_1242` (*CSYSPWRUPREQ is the signal from … . When CSYSPWRUPREQ is
  asserted, the power controller … asserts CSYSPWRUPACK …*) holds a rule whose antecedent is the whole *When* sentence
  and whose consequent is the sentences before it; on the persisted IR of the 31 current chains 108 of 1,130 rules are
  split at a marker that follows a sentence end.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `split_conditional_sentence` tries a fronted
  marker only at the start of the statement, and its embedded search (` when `, ` while `, ` during `, ` unless `)
  also matches a marker that opens a later sentence. RED, with the file restored byte-identically: dropping the new
  clause fails `a_condition_marker_after_a_sentence_end_opens_its_own_sentence`.
- [x] **ADDRESSED (verified)** — the 21 moved chains rebuilt stage by stage, each stage validated, the adapter last;
  ADIv6's rule reads antecedent *CSYSPWRUPREQ is asserted*, consequent signal `CSYSPWRUPACK`.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains with the HEAD and new binaries: 10 identical, 21 move
  only in `conditional_rules` (105 sources: 93 antecedents narrowed to a strict prefix, checked mechanically; 8
  consequent signals, 1 action and 6 refusals, each read). SemanticIR and IntentIR move only in the carried rules; no
  `.isf` text or lowering status moves; the five adapters whose listed obligations move were read.
  `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds. WIRE-BASED-100: all 11 seeds
  identical; `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D
  warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,778 passed, 0
  failed. `flow_census.json` rolled forward under this leaf (+4 decision sites, +1 helper edge).
- [x] **GENERICITY (ADR 0006)** — sentence punctuation only (`.`, `:`, `;` before the marker); no document, signal or
  phrase list.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` (*Corrected `2026-09-29`*); KM
  `a-condition-marker-after-a-sentence-end-is-fronted` (new); gap ledger S7 routes `.39.a`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.42` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — CoreSight TMC EvidenceIR holds an unconditional `FULL` must-be-HIGH constraint from *When
  the space in the FIFO is … Fill level &gt;= MEM_SIZE - BUFWM, the FULL output is pulled HIGH*; with
  `KG-ISF-COMPLETENESS.1g`'s gate it lowers into TMC's emitted `.isf` as `(FULL 1)`. 281 statements on the 31 current
  chains hold a character reference.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: twelve clause and sentence splitters end a
  clause at `;`, including the `;` of a converter character reference, so the level binding began after `&gt;` without
  its *When*. RED, with the file restored byte-identically: dropping the reference test in `is_clause_boundary` fails
  `a_character_reference_semicolon_ends_no_clause`.
- [x] **ADDRESSED (verified)** — TMC rebuilt stage by stage, each stage validated, the adapter last: no `FULL`
  constraint, no unconditional temporal rule on `DEVID register`.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains with the HEAD and new binaries: 30 identical, TMC loses
  one constraint and what hung off it downstream (a temporal rule, an invariant, an adapter residual line); no `.isf`
  text moves. `check_chain_currency.sh` 31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical;
  `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,781 passed, 0 failed.
  `flow_census.json` rolled forward under this leaf (+5 functions, +16 decision sites, +33 helper edges).
- [x] **GENERICITY (ADR 0006)** — the shape of a character reference (`&name;`, `&#digits;`); no document or signal.
- [x] **LOCKSTEP** — book `pipeline/evidence-failure-modes.md` (*A semicolon that belongs to a `>`*); KM
  `a-character-reference-semicolon-ends-no-clause` (new); `.42.a` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.42.a` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — after `.42`, CoreSight TMC's *When the space in the FIFO is less than or equal to this
  value, …, the FULL output is pulled HIGH* yields no constraint and no conditional rule.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `extract_dynamic_signal_constraints` read a
  level binding only from `text_before_condition_marker(clause)`, empty for a clause that opens with its condition, and
  `main_clause_of_fronted_conditional` needs a modal. RED, with the file restored byte-identically after each: skipping
  the split, or dropping the carried condition, fails `a_level_binding_after_a_fronted_condition_keeps_that_condition`.
- [x] **ADDRESSED (verified)** — TMC rebuilt stage by stage, each stage validated, the adapter last: the conditional
  constraint present, listed by the adapter, `.isf` unchanged and FSMGen-strict-clean.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical, TMC one addition.
  `check_chain_currency.sh` 31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical; `kg-bench`
  158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo
  test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,786 passed, 0 failed. `flow_census.json`
  rolled forward under this leaf (+1 function, +6 decision sites, +4 helper edges).
- [x] **GENERICITY (ADR 0006)** — the fronted-condition markers and a comma; no document or signal.
- [x] **LOCKSTEP** — book `pipeline/evidence-failure-modes.md` (the `.42` section's last paragraph); KM
  `a-character-reference-semicolon-ends-no-clause`; gap ledger S7.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.39.a` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — 254 of 1,124 conditional rules on the 31 current chains have a consequent that runs past
  its sentence, and about 9 take their consequent signal from a later one (ADIv6 *When nSRST is asserted, the entire
  system must be reset …* names `nSRST` only in its third sentence).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `split_conditional_sentence` bounded only its
  modal search by the sentence; the boundary comma and the consequent ran through the statement. RED, with the file
  restored byte-identically after each: reading the whole rest, or ending a sentence at a register's spaced field
  separator, fails `a_fronted_rule_ends_at_its_sentence`.
- [x] **ADDRESSED (verified)** — the 9 moved chains rebuilt stage by stage, each stage validated, the adapter last.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 22 identical, 15 rules read; the first version's one loss
  (USB4's register-dot-field rule) is kept by `sentence_end_in`. `.35`'s test expected a consequent running into the next
  sentence and now expects it to end. `check_chain_currency.sh` 31 of 31 current at every stage. WIRE-BASED-100: all
  11 seeds identical (AHB's gold included); `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 170 / 1,787 passed, 0 failed. `flow_census.json` rolled forward under this leaf (+1 function, +5 decision
  sites, +2 helper edges).
- [x] **GENERICITY (ADR 0006)** — punctuation and an identifier shape; no document or signal.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` (the `.39` correction's last paragraph); KM
  `a-condition-marker-after-a-sentence-end-is-fronted`; gap ledger S7.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41.a` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6's `ABORT` record holds 21 fields, its own 5 and `CTRL/STAT`'s 16 (two at bit 4),
  and `CTRL/STAT`'s record none, though *B2.2.3 CTRL/STAT, Control/Status register* heads those 16 field headings.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `is_message_container_name` refused a name
  holding `/`, so `parse_dotted_container_heading` missed the container. RED, with the file restored byte-identically:
  refusing `/` again fails `parse_dotted_container_heading_forms`.
- [x] **ADDRESSED (verified)** — ADIv6 rebuilt stage by stage, each stage validated, the adapter last: `ABORT` 5 fields,
  `CTRL/STAT` 16; both field blocks lowered in the blocked adapter's text.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical; ADIv6 moves only in `register_records`
  (the fields and a one-step id shift of the later heading registers). `check_chain_currency.sh` 31 of 31 current at
  every stage. WIRE-BASED-100: all 11 seeds identical; `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy
  --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 170 / 1,787 passed, 0 failed. `flow_census.json` unchanged. `abort`'s width
  becomes 5 (`.41.b`).
- [x] **GENERICITY (ADR 0006)** — one more character in an identifier shape; no document or register name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (after `.10g`); KM `section-header-register-block-qualification`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — ADIv6 states 10 register-field bindings to wires under the fields' own headings
  (*This bit controls the CSYSPWRUPREQ signal*, *This field drives the nSRSTOUT signal*, …) and no record carried them;
  the relation reader had read their `field`/`bit` as actors until `KG-ISF-COMPLETENESS.1g`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs` `RegisterFieldRecord` had no slot for a
  binding, and no reader in `crates/specforge/src/ir/evidence.rs` read a field heading's body. RED, with the file
  restored byte-identically after each: binding a field two registers share, or reading *indicates the status of* as a
  drive, fails `a_field_heading_body_binds_the_field_to_its_wire`.
- [x] **ADDRESSED (verified)** — ADIv6 rebuilt stage by stage, each stage validated, the adapter last: 10 fields carry
  their binding in EvidenceIR, SemanticIR and IntentIR register records.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical, ADIv6 only `register_records`; no `.isf` or
  residual moves. `check_chain_currency.sh` 31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical;
  `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,788 passed, 0 failed.
  `flow_census.json` rolled forward under this leaf (+2 functions, +14 decision sites, +20 helper edges, +1 macro).
- [x] **GENERICITY (ADR 0006)** — the `.10g` heading shape and closed verb phrases; no document, register or signal.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field bound to a wire*); KM
  `a-register-field-binding-to-a-wire` (new).

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41.b` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — all 201 registers with located fields on the 31 current chains take `size_bits` from their
  fields' extent and none states another; ADIv6's `ABORT` reads 5 bits wide; 13 widths are not a whole byte.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` filled `size_bits` from
  `register_size_from_fields` at five reader sites (`PDF-VARIANT-DIGESTION.2c`). RED, with each file restored
  byte-identically: inferring the heading register's width again fails
  `section_header_registers_capture_register_container`; dropping the residual's tail fails
  `a_register_width_the_document_does_not_state_is_listed_as_the_adapters_choice`.
- [x] **ADDRESSED (verified)** — the four moved chains and RISC-V Debug rebuilt stage by stage, each stage validated, the
  adapter last: no inferred width, `isf_register_width_not_stated` listed, `.isf` text unchanged.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 27 identical, exactly 201 widths removed and nothing else.
  `check_chain_currency.sh` 31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical; `kg-bench`
  158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo
  test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,789 passed, 0 failed. Three tests that
  pinned the inferred width now expect none. `flow_census.json` rolled forward (+1 decision site, +10 helper edges, +1
  macro; one function added, one removed).
- [x] **GENERICITY (ADR 0006)** — removes an inference; no document or register name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` and `pipeline/isf-adapter.md`; KM
  `a-register-width-is-stated-never-inferred` (new), `register-field-table-extraction`,
  `section-header-register-block-qualification`; `.41.c` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.43` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — AXI's *…, so RCHUNKNUM and RCHUNKSTRB must be the same width* rule names `RCHUNKCHK`, the
  first signal of its consequent, which is not the obligation's subject.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `extract_conditional_rules` took the first
  declared signal anywhere in the consequent. RED, with the file restored byte-identically after each: reverting to the
  first signal, or reading the whole text before the modal as the subject, fails
  `a_rules_consequent_signal_is_its_obligations_subject`.
- [x] **ADDRESSED (verified)** — AXI rebuilt stage by stage, each stage validated, the adapter last: `RCHUNKNUM`; `.isf`
  unchanged and FSMGen `--strict --check` 0 diagnostics.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical, AXI 2 rules, both read. `check_chain_currency.sh`
  31 of 31 current at every stage. WIRE-BASED-100: all 11 seeds identical; `kg-bench` 158/158. `cargo fmt --check`
  clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib
  --exclude specforge-production-graph` 474 / 170 / 1,790 passed, 0 failed. `flow_census.json` rolled forward (+4
  decision sites, +2 helper edges).
- [x] **GENERICITY (ADR 0006)** — the modal and a comma; no document or signal.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md`; KM `a-condition-marker-after-a-sentence-end-is-fronted`;
  `.43.a` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.41.c` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — RISC-V IOMMU's 33 register records are field tables named `register_table_00NN` with no
  width, while *Table 15. IOMMU Memory-mapped register layout* lists each register's offset and size and a heading
  names each in parentheses (`6.6. Command-queue base ( cqb )`). Corpus-wide, 161 tables carry an offset, a name and a
  size or width column, in 13 documents; IOMMU is the only current chain among them.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `register_name_from_heading` names a register
  only from a parenthetical holding a hex address, no reader reads a layout table, and `nearest_section_title_original`
  places a table by page. RED, with the file restored byte-identically after each: dropping the extent test or the
  gap-count guard fails `a_field_table_takes_the_register_its_heading_names_when_the_layout_sizes_it`; reading every
  table as bytes fails `a_layout_tables_size_is_read_in_the_unit_it_states_or_its_offsets_advance_by`.
- [x] **ADDRESSED (verified)** — IOMMU rebuilt stage by stage, each stage validated, the adapter last, then again with
  the final binary (identical): 26 records named, four split pairs merged, 29 records, 22 registers emitted under their
  names at the stated width; `isf_register_width_not_stated` lists 7, down from 33; FSMGen `--strict --check` 0
  diagnostics.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 30 identical, IOMMU moves only `register_records`.
  SemanticIR and IntentIR move only in register records and the two captured-region residuals of the layout tables,
  which the registers now cite; `.isf` text moves only in register names. `check_chain_currency.sh` 31 of 31 current.
  WIRE-BASED-100: all 11 seeds identical; `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph`
  474 / 170 / 1,792 passed, 0 failed. `flow_census.json` rolled forward (+4 functions, +65 decision sites, +80 helper
  edges, +1 macro).
- [x] **GENERICITY (ADR 0006)** — header words (`offset`, `name`, `size`, `width`, `bit`, `byte`), the table's own
  offsets, a heading's closing parenthetical and the ingest's own sequence; no document, register or vendor name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md`; KM `a-register-layout-table-sizes-the-register-its-heading-names`
  and `a-tables-reading-order-is-the-gap-it-left` (new), `a-register-width-is-stated-never-inferred`; `.41.d` opened.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.43.a` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — I2C's *transistor TR2 opens to separate the SCLH and SCL lines* rule names `SCLH`, the
  first signal of a consequent that is about a transistor; 44 of 1,121 rules on the 31 chains name a consequent signal.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` `extract_conditional_rules` took a modal-less
  consequent's first declared signal. RED, with the file restored byte-identically after each: reverting to the first
  signal, or dropping the condition-clause exclusion, the following-*be* subject or the verb object, fails
  `a_descriptive_consequents_signal_is_its_subject_or_the_one_its_verb_acts_on`.
- [x] **ADDRESSED (verified)** — I2C, Wishbone and AXI rebuilt stage by stage, each stage validated, the adapter last:
  `SCLH`, `CLK_I`, `BID`, `AWIDUNQ` ×2 and `BPERSIST` no longer bound; I2C's `isf_obligations_not_lowered` packet goes
  (its one entry was that rule) and AXI's falls from 116 to 112; `.isf` text unchanged.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 chains: 28 identical, six rules moved, all read. SemanticIR and
  IntentIR move only in those rules. `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seeds score
  identically (the three rebuilt documents scored by the HEAD binary on its own snapshot); `kg-bench` 158/158. `cargo
  fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo test --workspace
  --lib --exclude specforge-production-graph` 474 / 170 / 1,793 passed, 0 failed. `flow_census.json` rolled forward
  (+1 function, +8 decision sites, +47 helper edges, +1 macro).
- [x] **GENERICITY (ADR 0006)** — clause punctuation, determiners, condition markers, *be* and modal forms, and verbs of
  a signal's level; no document or signal name.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md`; KM `a-condition-marker-after-a-sentence-end-is-fronted`.
