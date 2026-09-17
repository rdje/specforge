# EXTRACTION-QUALITY-GAUGE — kind span successors

- Part ID: `kind-span-successors`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:kind-span-successor-leaves:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.3k.3` · Status: `done` (`2026-09-13`, CODE) · Goal: **the kind reads its own obligation
  clause** — the original `.3k` goal, at its true size, which turned out to be **one record per OBLIGATION**.
  `classify_signal_constraint_kind(&text.to_ascii_lowercase())` becomes
  `classify_signal_constraint_kind(&constraint_bearing_sentence(text).to_ascii_lowercase())` in
  `extract_signal_constraints`, joining the subject, the condition and (since `.3i`) the negation,
  which all already come from that span. **Population: 4 `sigcon_*` records.** AHB `sigcon_0002` is
  the mechanism `INVARIANT-SHAPE-ADMISSION.3` reported without explaining: its clause is *"When the
  Subordinate is initially selected, it must also monitor the status of HREADY…"*, which contains no
  kind phrase at all, and the published `must_be_asserted` comes from the NEXT sentence, *"HSELx must
  be asserted in the same cycle…"* — kind from one clause, condition from another. NVMe
  `sigcon_0005`/`0006`/`0007` take `must_not_change` from a sentence two clauses later whose own
  obligation is conditional on a capability bit.
  **Inherited from `.3k.2b` (`2026-09-12`) — the same defect in the VALUE slot.** AXI
  `WTAGUPDATE must_be_value UPDATED` survives every gate this family has built because its table cell
  matches the `must be valid` arm on a phrase LATER in the cell while `extract_protocol_state_value`
  binds from the FIRST `must be ` in the text, which is *"the tags in memory must be updated"*. The
  arm matched on one span and its value came from another, so narrowing the classifier's span fixes
  the value binder at the same time — re-measure the value slot here, not only the kind.
  **RE-DERIVED `2026-09-13` with `replay-constraints` (row stratum judged): the population is 3, not 4.**
  AHB `sigcon_0002` and NVMe `sigcon_0005`/`0006` still reproduce; NVMe `sigcon_0007` (`NVM
  must_not_change`) does not — `CORPUS-COVERAGE.2.50a` refuses its subject, which is a message-name
  fragment rather than a signal. Read against source, the other three are exactly this leaf's defect:
  NVMe's statement's first modal sentence is *"The ANA Group Identifier (ANAGRPID) … shall be unique
  within the NVM subsystem"* — uniqueness, not no-change — while the published `must_not_change` comes
  from the NEXT sentence, whose own obligation is conditional on a capability bit that the record drops;
  and two of its three subjects (`ANA`, `NVM`) are fragments.
  **A trap this leaf must not walk into, found while re-deriving.** AHB `sigcon_0002` is CORRECT today,
  by accident: its clause is the first modal sentence (*"When the Subordinate is initially selected, it
  must also monitor the status of HREADY…"*), which states no kind, and the published
  `HSELx must_be_asserted` is lifted from the THIRD sentence of the same serialized row, where the
  document does say it. Narrowing the classifier's span as this node describes would type that clause as
  nothing and `.3k.2a` would then refuse it — **losing a record the document supports**. So the span
  narrowing alone is not the fix. The real defect underneath is that `extract_signal_constraints` takes
  only the FIRST modal sentence of a statement and drops obligations 2..n, which is the collapse
  `INVARIANT-SHAPE-ADMISSION.3` named for the row path (*"a cell stating three obligations yields
  three — the serialized statement collapses them into one and keeps only the first"*). One record per
  clause, the way the row reader already works, is the shape to size — and that is recall, so it must be
  measured as an ADDITION before it is shipped as a narrowing.
  **ADDITION MEASURED `2026-09-13`, with the real producer, before any of it shipped** — the node asked
  for exactly this and it changed the design twice. A prototype was built (one record per obligation
  clause, every part read from that clause), `replay-constraints --json` was captured per document
  before and after, and the two runs were diffed over the **74 documents comparable in both** (AHB,
  AXI-L and APB-E drop out: the prototype moves their content, which stales their proofs and they then
  refuse to LOAD — the standing hazard, and its first useful use as a signal).
  **First result: +19 records, and −2, and the −2 are this leaf's own population.** NVMe `sigcon_0005`
  and `0006` stop reproducing, which is the fabrication this leaf exists to remove; **not one other
  persisted record is lost corpus-wide**. AHB `sigcon_0002` is not lost either — its third clause mints
  the same subject and kind with the CONDITION its own clause states, so the trap recorded above is
  avoided by reading every clause rather than by narrowing to the first.
  **Second result, and the reason the first design was wrong: the whole-statement subject FALLBACK is
  a cross-clause leak, and per-clause reading multiplies it.** When a clause's subject part yields no
  signal the reader scans the entire statement, so every clause's kind is handed the same
  statement-wide subject set. AXI `When the ACVALID signal is asserted the snoop address and control
  signals on ACADDR, ACPROT, and ACSNOOP must not change… When ACVALID is asserted, it must remain
  asserted until ACREADY is asserted` minted `ACADDR/ACPROT/ACSNOOP must_be_asserted`: the second
  clause's obligation is about ACVALID, its own subject is the pronoun `it`, and the statement-wide
  scan supplied the first clause's three signals. LTI minted `signal must_be_low` and
  `LAFLOW must_be_low` the same way.
  **Third result, and the missing piece: a FRONTED condition leaves the subject part empty.**
  `text_before_condition_marker` cuts at the EARLIEST marker, so `When <cond>, <subject> must <kind>`
  cuts at offset 0 — which is WHY the fallback fires so often, and why AHB `sigcon_0002` took its
  subject from a different sentence than its condition in the first place. The main clause of a
  fronted conditional begins after its comma. With that read and the fallback bounded by the
  obligation, the fabrications above disappear, `LAOGV must_be_low` replaces the fragment
  `LAOG must_be_low`, and the additions become **+22 on the same 74 documents**: AXI's five
  `*VALID must remain asserted` handshake invariants, AXI's four `AWSTASH* must be driven LOW`,
  LTI `LMOPENREQ must be asserted`, HBM2 `CKE must be driven LOW`, and the rest.
  **Fourth result — the container's ordering rationale is confirmed, not assumed, and it blocks.**
  NVMe's clause 1 (*"… shall be unique within the NVM subsystem"*) lands on the ungated `generic_value`
  arm exactly as `.3k` predicted, so the prototype trades two fabrications for two others.
  `EXTRACTION-QUALITY-GAUGE.3k.2k` was opened and landed for that, and this leaf resumes on top of it.
  **Two open questions for the implementation, both raised by the measurement and neither yet decided:**
  (a) APB's four `PAUSER`/`PWUSER must have the same value` clauses are then minted by BOTH the
  statement path and the row path, with the same subject/kind/condition and different `source_text`, so
  the merge key does not collapse them — decide whether that is a duplication to fix here or a
  cross-producer question of its own; (b) eMMC mints `PARTITION must_not_change` from
  `PARTITION\_ACCESS`, a fragment produced by the normalizer's escaped underscore, which is a
  tokenization defect rather than a span defect.
  **SHIPPED `2026-09-13`.** `constraint_bearing_sentences` yields every obligation clause in document
  order (the singular helper is now its first element, so the two cannot drift); the per-statement body
  became a per-obligation loop reading kind, value, negation, subject and condition from the ONE clause
  that mints the record; `obligation_subject_part` reads the main clause of a fronted conditional;
  the subject fallback is bounded by the obligation; and `is_post_passive_binding_only_subject_in`
  judges the obligation the record came from rather than the statement's FIRST one. Records restating
  one obligation dedupe on the producer's own merge identity. `source_text` deliberately stays the
  STATEMENT: it is what `supporting_statement_ids` cites and what the replay's merge identity keys on.
  **Measured on the three rebuilt documents, and the container's two inherited residuals both closed.**
  AHB 13 → 13 with `HSELx must_be_asserted`'s condition CORRECTED from the previous clause's to its
  own (*"a Subordinate is selected for a non-IDLE transfer"*) — the trap avoided rather than walked
  into. AXI-L 40 → 53: five `*VALID must remain asserted` handshake invariants, four
  `AWSTASH* must be driven LOW`, `WTAGUPDATE must_be_deasserted`, four `WTAG` value records — and
  `WTAGUPDATE must_be_value UPDATED` REMOVED, which is `.3k.2b`'s named residual closed exactly as it
  predicted ("narrowing the classifier's span fixes the value binder at the same time"). APB-E 23 → 27.
  Corpus-wide over the 74 replayable documents: **187 → 211 replayed, and the only persisted records
  that stop reproducing are NVMe `sigcon_0005`/`0006`, this leaf's own population.**
  **A new temporal conflict is a RESULT, not a regression.** AXI-L gains `WTAG post_tick VALID vs
  ZERO`: the document states both, conditional on `WTAGOP`'s enum row, and the condition lives in the
  row's value cell rather than in a `when` clause. The pipeline surfaces the ambiguity instead of
  silently keeping one — which is the roadmap's own contract for undecided evidence.
  **Four residuals found by the measurement, each given its own leaf rather than absorbed:** `.3k.7`
  (AXI `WSTRB must_be_value VALID`, a subject inside `enabled by WSTRB` that the table-row exemption
  admits), `.3k.8` (the statement path and the row path now publish APB's six `PAUSER`/`PWUSER`
  obligations twice, differing only in `source_text`), `.3k.9` (eMMC `PARTITION` from
  `PARTITION\_ACCESS`, an escaped-underscore tokenization fragment), `.3k.10` (a fronted condition
  that opens the STATEMENT carries no leading space, so its marker is never seen).
  Prerequisite: `.3k.2` (see the container's ordering rationale) — **satisfied `2026-09-13` by
  `.3k.2k`**. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.3`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.7` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.3`) · Goal:
  **a table row's subject exemption survives a clause that has its own subject.**
  `is_post_passive_binding_only_subject` exempts a serialized table row from its pre-lead subject
  authority, because a row's other cells legitimately name the subject an obligation cell constrains.
  `INVARIANT-SHAPE-ADMISSION.5` withdraws that exemption when the clause HEADS with a different
  identifier. It does not withdraw it when the identifier sits one descriptor back, and AXI
  `| Match | 0b11 | … WTAG bits must be valid for byte lanes that are enabled by WSTRB. |` is exactly
  that: head `bits`, a common noun, so the exemption stands and `WSTRB` — reachable only inside the
  trailing `enabled by` phrase — is published as a co-subject of an obligation about `WTAG`.
  **The obvious rule was tried in `.3k.3` and MEASURED, and it is too wide:** withdrawing the
  exemption whenever the clause names any identifier before its lead costs reproduced persisted
  records and removes AXI `AWSIZE`/`AWLEN`/`AWCMO` and LTI `LASSID`/`LRMECID` value records — and it
  reaches the DYNAMIC path, which `.3k.4` owns, through the two-argument wrapper.
  **RE-DERIVED `2026-09-13` by building that widest rule as a prototype and diffing
  `replay-constraints` per document, and the node's own sizing was wrong in both directions.** The
  wide rule removes **12 records over the 76 comparable documents**, not eight, and MMU-700
  `dyn_sigcon_0008` is **not** among the reproduced ones — it stopped reproducing before this leaf
  (its published condition comes from a different clause), so the cost in *reproduced persisted*
  records is **2** (RISC-V IOMMU `dyn_sigcon_0007` `PDT`, NVMe `dyn_sigcon_0015` `BADD`). The two the
  node did not name are OpenCAPI `sigcon_0003` `TLX` and AXI-H `sigcon_0043` `WSTRB` — and the second
  is the defect itself, in the older specification, which is the class evidence this leaf needed.
  **The shipped rule is one token wider than the head, and the corpus decided where it stops.** The
  first bullet of `obligation_head_is_a_foreign_identifier`'s own doc comment already said what a
  common-noun head IS — *"a DESCRIPTOR standing in for an identifier the cell names right beside
  it"* — so when that identifier is right beside it the clause DOES name its own subject. The head
  test now reads the PREMODIFIER when the head carries no identifier of its own, and the
  premodifier counts only when the preceding token is the identifier **raw**. That single condition
  is the whole difference between the two rules: it separates `WTAG bits` from LTI's
  *"When LASSIDV is LOW, this signal must be 0"*, where the nearest identifier-shaped token is
  `LOW,` closing the fronted condition rather than opening the noun phrase. A head that carries an
  identifier FRAGMENT but is not one (`LRPROT[0`, `11:00`) is left to the row exemption unchanged,
  because that spelling is not the tokenization the extractor lifts.
  **Measured: the shipped rule removes exactly ONE record over the 76 comparable documents**
  (AXI-H `sigcon_0043`), and all twelve the wide rule destroyed survive. Corpus replay
  **200/127 → 199/126**, `not_reproduced` unchanged at 73. AXI-L rebuilt: **55 → 54**, the delta
  being `sigcon_0027 WSTRB must_be_value VALID` and one `fact_provenance` entry; everything else in
  the artifact is id renumbering, and the emitted `.isf` differs only in `tinv_sc_sigcon_*` numbering.
  **A finding the rebuild produced, routed rather than absorbed: the fabrication never reached
  SemanticIR, because `WSTRB` is not in AXI's declared catalog.** Both WSTRB records — the
  fabrication and the REAL one (*"An attached Subordinate must have its WSTRB input tied HIGH"*) —
  are demoted to residuals by the semantic grounding filter. `WSTRB` is declared in EvidenceIR
  (`Signal WSTRB is output width DATA_WIDTH / 8.`) and dropped by
  `parse_explicit_signal_declaration`, whose `index != tokens.len()` refusal discards the WHOLE
  declaration — direction included — when it cannot finish the width expression. Nine AXI-L signals
  are lost this way and every one has an arithmetic width. Owned by
  `SIGNAL-DECLARATION-ROW-DROP.4`; fact card `[[arithmetic-width-drops-the-declaration]]`.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.7`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.8` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.3`) · Goal: **one obligation, two producers, two records.** APB-E publishes `PAUSER must_be_value VALID`,
  `PAUSER must_not_change` ×2 and the three `PWUSER` equivalents **twice** — once as `sigcon_*` from
  the statement path reading the serialized row, once as `row_sigcon_*` from the table reader reading
  the same cell. They differ only in `source_text`: the statement path cites the whole row, the row
  path cites the clause. `dedup_appended_signal_constraints` keys on
  `signal_constraint_merge_key`, which INCLUDES `source_text`, so it cannot see them as the same fact.
  **Pre-existing but amplified: it was 2 records before `.3k.3` and is 6 after**, because the
  statement path now reads every clause of the row the row reader already reads.
  **The decision is which provenance survives, and it is not obvious**: the row path's `source_text`
  is strictly better (the obligation's own words), but the dedup is deliberately one-directional so
  the established pattern/dynamic surface stays byte-for-byte. Changing the merge key also moves
  `replay-constraints`' reproduction identity for the whole corpus, so the population must be measured
  before and after with that in mind.
  **RE-DERIVED `2026-09-13` over all 77 loadable documents with `replay-constraints`, and the census
  is bigger and narrower than the node's framing.** The corpus carries **82 extra records in 12
  documents** that assert the same subject/kind/condition as another — and all but nine come from a
  genuinely DIFFERENT sentence, so they are not duplicates at all. The cross-producer class this leaf
  is about is **nine records in one document** (APB-E; the node said six, which predated `.3k.3`
  reading every clause of the row), and in every one the row reader's clause is literally CONTAINED in
  the statement reader's serialized row.
  **That containment is the shipped test, and it makes the merge-key question go away.** The merge key
  is split: `signal_constraint_assertion_key` is every identity field EXCEPT the provenance, and
  `signal_constraint_merge_key` is now built from it plus `source_text`, so the two cannot drift and
  the replay's reproduction identity is byte-for-byte unchanged — the blast radius the node worried
  about is not incurred. `dedup_appended_signal_constraints` drops an APPENDED record when an
  established record asserts the same thing AND contains its provenance.
  **Which provenance survives is answered by an invariant rather than by preference.** The established
  record survives, and its `source_text` is the serialized row — which is what its own
  `supporting_statement_ids` cites, the invariant `.3k.3` fixed for the statement path. The
  alternative (rewrite the row reader's `source_text` to the serialized row, letting the existing key
  collapse the pairs) was rejected: it moves the provenance of EVERY `row_sigcon_*` record, including
  the ones no other producer reaches, to serve nine.
  **Measured: corpus replay 201/128 → 192/119, and APB-E is the only document that moves.** Its chain
  rebuilt: EvidenceIR **27 → 18**, SemanticIR and IntentIR **27 → 18**, and the emitted `.isf`
  **56 → 38 rules** — with the count of DISTINCT rule bodies unchanged at 12 and not one body lost.
  FSMGen was being handed eighteen identical rules where the specification states nine obligations.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.8`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.9` · Status: `pending` — **re-derived and re-owned `2026-09-13`;
  the premise it was opened on is wrong and the disposition is DO NOT SHIP YET, on evidence** (opened
  `2026-09-13` by `.3k.3`) · Goal: **a Markdown escape fragments an identifier, and the fragment is
  then DECLARED as a signal.**
  `.3k.3` opened this as a tokenization nuisance: the normalizer emits `PARTITION\_ACCESS`,
  `collect_subject_signal_tokens` splits on any character that is not alphanumeric-or-underscore, the
  backslash ends the token, and eMMC mints `PARTITION must_not_change` about a signal the
  specification does not have. `EXTRACTION-QUALITY-GAUGE.3k.1`'s own test comment records the same
  mechanism producing `ZETADTI` out of `ZETADTI\_TBU\_CONDIS\_ACK`.
  **Where the escape comes from, measured rather than assumed.** It is not SpecForge's: it is
  Docling's, and it is CORRECT Markdown — an underscore inside an identifier must be escaped. The
  persisted `SourceIr` carries **zero** occurrences (it stores structured table cells); the normalized
  bundle carries them (`CONTEXTIDR\_EL1`); and EvidenceIR carries 2,908 in eMMC alone, because the
  evidence stage reads the bundle's text. **So "fix it upstream" is not available**: re-ingest
  reproduces it by construction, and both contaminated documents are frozen anyway, so an ingest-side
  fix would reach neither of them. The fix has to be at the reader, which is also the only place that
  reaches the frozen stratum — the same property that makes `replay-constraints` work.
  **THE FINDING THAT MATTERS, and it is why this leaf must not be shipped from its opening premise.**
  The fragmentation does not merely produce a bad subject: it produces a bad **DECLARATION**. eMMC's
  statement set contains `Signal PARTITION is width 1.` and `Enum PARTITION NOT_DEFINED = 0.` — the
  catalog holds the fragment, so every subject gate, polarity pass and relation reader downstream
  treats it as authority. **And that defeats the repository's standard discriminator.** `.3g`/`.3h`/
  `.3k.11` all use *standalone wins* — a candidate that occurs even once outside the suspect position
  is never touched — and here the synthesized declarations ARE those standalone occurrences. **The
  contamination manufactures its own evidence of innocence.** A census that does not exclude the
  synthesized `Signal …`/`Enum …` forms reports this class as empty; the first cut of this
  re-derivation did exactly that.
  **The measured populations, three of them, and they are not the same size**
  (`python3 scripts/measure_escaped_identifier_fragments.py`, shipped with this re-derivation,
  `--self-test` 6/6):
  * TEXT — **67 of 78 documents** carry an escaped identifier. Wide, and mostly provenance: a register
    name in a section title is not intent.
  * CATALOG — **18 declared names across 2 documents** exist ONLY as the head of an escaped compound:
    17 in eMMC (`PARTITION` ← `PARTITION_ACCESS`, `PARTITIONING` ← `PARTITIONING_EN`, `POWER` ←
    `POWER_CLASS`, `TAG` ← `TAG_UNIT_SIZE`, …) and 1 in GIC-600 (`REQUEST` ← `REQUEST_COMPLETE`).
  * RECORDS — **0** published constraint subjects. Not one. The eMMC record `.3k.3` mints is not
    persisted, so this becomes 1 only when that document's artifact is next refreshed.
  **THE CALL: do not ship it now, and the reason is the measurement rather than the calendar.** The
  only fix that reaches the two contaminated documents is a change to the shared identifier
  tokenization — the seam the catalog, every subject reader, the polarity pass and the relation reader
  all sit on — and it would move the identity layer of the **67** documents that carry the escape in
  order to correct **18 names in 2** of them, with a published constraint effect of **zero**. This
  family's own rule, applied twice already today, is that a rule nothing exercises does not ship
  (`.3k.5`'s magnitude leads, and `.3k.2k`'s zero population shipped only because its class was
  demonstrable through the real reader on a real sentence — this one's is not, in the records).
  Shipping it would also be unmeasurable at handoff: `CHAIN-CURRENCY` re-executes the whole pipeline
  and does not finish inside a session.
  **What has to be true before it ships**, in order: (a) the corpus effect of unescaping at the
  tokenization seam measured with `replay-constraints` AND with a full `scripts/check_doctrines.sh
  --all`, run detached, because this moves declarations and not only constraints; (b) every current-schema
  document that moves rebuilt and diffed (AXI-L carries `AWSNOOP\_WIDTH` and `WSTRB\_Present`, so it
  will move); (c) the 18 names adjudicated individually — several are real English words (`POWER`,
  `USER`, `CLASS`, `NUMBER`) whose removal from a catalog may withdraw records that are correct for
  unrelated reasons; (d) a control for the CIRCULARITY above, so the next reader cannot re-derive this
  as empty.
  Prerequisite: none. Verification: the three populations re-derived with the shipped census; all 18
  names adjudicated individually; observed RED; the chain rebuilt for every document whose artifacts
  move; `--all` doctrines green.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.10` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.3`) · Goal:
  **a fronted condition that opens the STATEMENT carries no leading space.**
  `text_before_condition_marker` matches `" when "`, `" if "`, … with a leading space, so a condition
  fronting the first clause of a statement is invisible to it and the condition's own signals stay in
  the subject part. AXI `When the ACVALID signal is asserted the snoop address and control signals on
  ACADDR, ACPROT, and ACSNOOP must not change, …` therefore publishes `ACVALID must_not_change`
  alongside the three real subjects. `.3k.3` pinned the shape in
  `a_pronoun_subject_does_not_borrow_a_sibling_clauses_signals` rather than fixing it, because the
  cheap repair is wrong here: this sentence's first comma is a LIST separator, not the condition's
  boundary, so taking the text after it would also drop `ACADDR`. The clause boundary has to be found,
  not guessed. `split_conditional_sentence` already carries a leading-marker list for the same
  question and is the place to start.
  **The asymmetry is narrower than "statement-initial", and finding that decided the design.** The
  sentence splitter leaves a leading space in front of every sentence of a statement EXCEPT the
  first, so `text_before_condition_marker` sees a fronted marker everywhere but there — and where it
  does see one, `obligation_subject_part` already handled it, by taking the text after the FIRST
  comma. So the defect is two defects: the first sentence is not read at all, and the comma rule is
  wrong wherever that comma opens a list.
  **The boundary is read, in two structural steps, and the corpus chose both.** (1) A comma inside a
  coordinated list is followed — at some later segment before the modal — by the coordinator that
  closes the list; the boundary is the first comma that is NOT such a comma. (2) When every comma
  belongs to a list, or there is none, the condition is a FINITE clause and ends at its verb: the cut
  is after the last finite copula before the modal. Step (2) is what recovers the sentence this leaf
  opened on, whose main clause begins with no punctuation at all.
  **A second leg was forced by the measurement, and it is the better half of the result.** The
  boundary reading ALONE removes four fabrications and costs one true record — AXI
  `If BCOMP is present, it must be asserted …`, which the old code got right only by accident, by
  scanning the condition it was meant to strip. The main clause of a fronted conditional very often
  opens with a PRONOUN, and there the antecedent is not a guess: it is the condition's own subject,
  the single nominal the sentence puts before it. With that leg the same four fabrications go and
  **six** true records arrive, including that one and AXI's `*VALID must remain asserted` handshake
  invariants. **The two ship together because the first makes the second decidable** — only once the
  condition's extent is known can the main clause's subject be recognised as a pronoun at all.
  **The book's standing pronoun refusal is refined, not weakened, and a control proves it**: AHB's
  `When the Subordinate is initially selected, it must also monitor the status of HREADY` resolves
  `it` to *the Subordinate*, which is not a declared signal, so that row still mints nothing.
  **An intermediate design was built and measured and is recorded because it failed**: cutting at the
  condition's finite verb INSTEAD of at its boundary comma leaves an adjunct phrase's signals
  (`When AWAKEUP is asserted with SYSCOREQ asserted and SYSCOACK deasserted, it must …`) in the
  subject part and publishes two requirements the sentence does not state. That is why the comma leg
  is tried first and the verb leg is the fallback rather than the rule.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.10`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.4` · Status: `done` (`2026-09-13`, CODE) · Goal: **the dynamic path's span
  discipline.** After `.3i` it reads its negation from `constraint_bearing_sentence` while its
  subject (`text_before_condition_marker(&statement.text)`), its value binder
  (`extract_discovered_state_value_from_text` over the whole lowered statement) and its condition
  (`extract_condition_clause(&statement.text)`) all read the WHOLE statement — so it is now the one
  producer whose parts are provably drawn from two different spans. The fix is not to call
  `constraint_bearing_sentence`: this path's records are minted by a value binding that need not be
  modal, so the clause it needs is the BINDING-bearing clause. Define it, then apply it to all four
  parts at once. **SHIPPED `2026-09-13`, and the shape that shipped is narrower than the node
  described, because the wider one was built and measured first.** `binding_bearing_clause` FINDS the
  binding exactly as the statement-wide reader found it — same value, same kind, bit for bit — and
  then LOCATES the first clause that reproduces it; the condition and the negation are read there.
  **Running the binders per clause instead was measured and rejected twice.** Narrowing the SUBJECT
  to that clause costs ten NVMe records whose subject is the cell's leading MNEMONIC while the binding
  is in the descriptive body (`| 17:16 | Record Format (RECFMT): … shall be 0h. |`) — the same
  row asymmetry `is_post_passive_binding_only_subject` gate (2) already encodes, and 30 reproduced
  records corpus-wide. Searching per clause then ADDS six records in AMBA LPI alone, and three of the
  six are `PREQ`/`PACCEPT must_be_high` off state-table rows that set those signals LOW, because
  `logic_level_binding_kind_from_text` pairs a level with the BIND VERB rather than with a signal
  (`.3k.11`). A span leaf must not ship recall through a pairing that is still wrong — the ordering
  `.3k` imposed on `.3k.2` before `.3k.3`, applied again.
  **Measured, corpus-wide over all 77 loadable documents: `replayed_total` is UNCHANGED in every
  single document (304 → 304). Zero records added, zero removed, 19 corrected**, and all three
  current-schema documents still load, so nothing is rebuilt. All 19 adjudicated individually with
  the instrument this leaf extended: **8 had a condition taken from a clause the record does not come
  from** (MMU-700 ×6, where `LRPROT`/`LAPROT must be 0` carried *"When LRRESP is FaultAbort … this
  signal is not valid"* — a clause that CONTRADICTS the record; CoreSight ×1; GIC-600 ×1, a trailing
  cell delimiter), **5 had a condition that ran past its clause into the next sentence** (HBM2, where
  `AERR/DERR must_be_low when parity check is suspended during power-down` swallowed *"Signals are
  shown with tPARAC=0 …"*, and `DBI must_be_high` swallowed its own `otherwise` branch), **5 carried a
  negation from a clause two sentences away** (GIC-600 ×2, AMBA LPI ×3 — `PREQ`/`PACCEPT
  must_be_high` **negated** by a `cannot` about assuming properties of a previous power state), and
  **1 had a run-on condition spanning a duplicated cell** (NVMe `ELEN`).
  **The instrument was extended because the verdict could not be adjudicated.**
  `replay-constraints` reported *"the kind, condition or negation moved"* and printed none of them;
  `ConstraintReplayVerdict` now carries `condition_text`, `negated` and a bounded `source_text`
  excerpt, which is what made all 19 readable from the report rather than from a re-derivation. That
  is an amendment to `.3k.6`'s surface, owned here because this leaf is what needed it.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.4`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.11` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.4`) · Goal: **the logic-level binder pairs a level with a VERB, not with a SIGNAL.**
  `logic_level_binding_kind_from_text` finds the first bind verb (`drive`/`set`/`tied`/…) and then
  takes the LAST logic value within six words of it, and the caller pairs that kind with every
  declared signal the statement names. So `| P_ACCEPT | … | Controller must set PREQ LOWand PREQCHK
  HIGH. |` publishes **`PREQ must_be_high`** — the level belongs to `PREQCHK`, one token later, and
  the record says the opposite of what the row states. Two such records are already published in AMBA
  LPI, and `.3k.4` measured that searching the binder per clause would add three more.
  **Size it against the real binder before changing it**, and note the normalizer artifact in the
  same population: the same rows read `LOWand`/`HIGHafter` with the space lost, so a fix that assumes
  clean word boundaries will behave differently on the corpus than on a test string
  (`[[one-modal-vocabulary-per-constraint-record]]`'s lesson, one layer down). The obvious rule — pair
  the level with the nearest preceding identifier — must be measured against every `must_be_high`/
  `must_be_low` record the dynamic path currently publishes, because that path is 77 of the corpus's
  deterministic records.
  **SHIPPED `2026-09-13`, and the obvious rule was wrong in three separate ways the corpus showed.**
  (a) *Nearest PRECEDING* is wrong: AMBA LPI writes *"a controller with an absent or tied LOW QDENY
  signal"*, so the walk goes backward first and forward only when backward finds nothing. (b) *Shape*
  is wrong: an identifier cannot be recognised by its CASE, because
  `WIRE-BASED-100.5i`'s alpha-invariance control feeds this path
  `signal_alias_000001_ready_000000006d11fd13` and requires identical behaviour — so the DOCUMENT'S OWN
  CATALOG decides what an identifier is, which is the repository's idiom everywhere else and the only
  ADR-0006-safe answer. That control went RED on the first implementation and is the reason this leaf
  has a catalog parameter at all. (c) *Clean word boundaries* are wrong: the normalizer loses the space
  in `LOW and` / `HIGH after`, so `token_logic_level` reads a token's leading uppercase RUN as well as
  the whole token — and that run is also what stops `PREQCHK HIGH` reaching back past `LOWand` to
  `PREQ`. A SUBSCRIPT is skipped rather than treated as a boundary (`sets HPROT[0] HIGH`), which the
  AHB rebuild proved necessary: without it a correct record is lost alongside the fabricated one.
  `logic_level_binding_kind_from_text` is RETIRED — it answered "is there a level after a binding
  verb" and nothing about what the level belonged to.
  **Measured, all 22 reproduced logic-level records adjudicated individually, plus every record the
  change adds or removes. Corpus replayed is 304 before and 304 after**, and the composition is the
  result: **11 fabrications removed** — GIC-600 `PMU`/`GIC must_be_high` (the HIGH belongs to the
  lowercase tie-off `gicp_allow_ns`) and `MBIST must_be_high` (it belongs to the row's own
  `nmbistreset`); CoreSight `ATB must_be_low` (it belongs to `araddr_m`/`awaddr_m`, while `ATB` comes
  from `ATB_DATA_WIDTH` a sentence later); AXI-ACE `WVALID must_be_low` from *"When WVALID is LOW, the
  write strobes can take any value"*, a CONDITION; AHB `HTRANS must_be_high` (it belongs to `HSEL`);
  LPI `PREQ`/`PACCEPT must_be_high` (they belong to `PREQCHK`/`PACCEPTCHK`); NVMe `NVM`/`LBA
  must_be_low` from *"used to low level format the NVM media"*; HBM2 `DM must_be_high` (*"DM output is
  not affected by the DBIac function"*). **15 correct records added** — ten AMBA LPI P-Channel
  state-table rows (`Controller has set PREQ LOW…`, `Device must set PACCEPT LOW`), AXI-L
  `AWSNOOP`/`ARSNOOP must_be_low` from *"An attached Subordinate must have its AWSNOOP input tied
  LOW"*, HBM2 `DBI must_be_low` from the `otherwise` branch it used to swallow and `CKE must_be_low`,
  and CoreSight TMC `FULL must_be_high` from *"the FULL output is pulled HIGH"*.
  **One correct record is LOST and is named rather than absorbed:** LPI `QDENY must_be_low` from
  *"…with the QDENY output absent or tied low"*. The walk stops at `absent`, which is a predicate
  adjective rather than scaffolding, and the sibling sentence one figure earlier (*"an absent or tied
  LOW QDENY signal"*, signal AFTER the level) still binds. Widening the skip list to fit this one
  sentence would be fitting the rule to an instance; the residual is `.3k.12`.
  **`.3k.4`'s control is superseded in part** and says so in place: it compared the located clause's
  kind against `logic_level_binding_kind_from_text`, and that function no longer exists. Its reasoning
  stands for the DISCOVERED-VALUE binder, which is what it now asserts.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.11`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.12` · Status: `done` (`2026-09-14`, PROBE/DOC) · **Sized, and the
  class it named has a population of ONE. No rule.** The leaf asked whether PREDICATE ADJECTIVES are a
  class the level walk should cross, after AMBA LPI's *"with the QDENY output absent or tied low"*
  failed to bind while its twin one figure away bound.
  Measured over all 261,858 persisted statements and constraint source texts
  (`scripts/measure_logic_level_walk_blockers.py`), the population of "a word that stops the backward
  walk while a declared signal sits within three words beyond it" is **14 distinct cases across 11
  different blocking words**, and they are not one class:

  | blocker | n | what it is | crossing it |
  | --- | ---: | --- | --- |
  | `always`, `again`, `therefore` | 5 | adverb | right — `AERR is always driven LOW` |
  | `remain`, `remains` | 2 | verb | right — `HRESP remains driven HIGH` |
  | `can` | 2 | modal | **wrong** — `TVALID can be driven HIGH` is a PERMISSION, which `.3k.13` just gated |
  | `cannot` | 1 | negation | **wrong** — `TLAST cannot be tied LOW` would publish its opposite |
  | `this` | 1 | determiner opening a new sentence | crosses a clause boundary |
  | `absent` | **1** | **predicate adjective** | the leaf's whole named class |
  | `PDENY` | 1 | an UNDECLARED SIGNAL | not the walk's defect at all |
  | `write` | 1 | a garbage eMMC row | — |

  **The named class is one sentence in one document**, which is exactly the mirror `.3k.11` refused to
  build, and no wider rule is available: any list long enough to admit the adverbs also admits `cannot`
  (which inverts the fact) or `can` (which states a permission as a requirement). The walk's stop is
  correct; what it stops at is thirteen unrelated things.
  **One real finding came out of it and it belongs elsewhere**: `PDENY` blocks only because AMBA LPI's
  catalog does not contain it — that document declares 5 signals (`PACCEPT`, `PREQ`, `QACCEPTN`,
  `QDENY`, `QREQN`) and the P-Channel's `PDENY`, `PACTIVE` and `PSTATE` are missing. Routed to
  `SIGNAL-CATALOG-CAPTURE-GAP.6`.
  Verification: `python3 scripts/measure_logic_level_walk_blockers.py` — read-only over every persisted
  EvidenceIR's `extracted_statements` and `signal_constraints[].source_text`, mirroring
  `logic_level_bindings`' `BIND_VERBS`, `MAX_GAP`, `DESCRIPTORS` and `token_logic_level`; every one of
  the 14 windows printed verbatim and adjudicated above. It ships as a tracked reproducer rather than as
  numbers in this file, per this repository's standing finding that an untracked probe's figure cannot
  be re-derived one session later. No artifact written or mutated.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.12`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.13` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.11`) · Goal: **the dynamic path has no MODALITY gate.** AHB `dyn_sigcon_0012` publishes `HPROT must_be_high` from
  *"It is **recommended** that a Manager sets HPROT[0] HIGH"*. The pairing is right and the level is
  right; what is wrong is that a RECOMMENDATION is published as a hard constraint.
  `EXTRACTION-QUALITY-GAUGE.3k.2a` refused exactly this shape in the statement path — *"It is
  recommended, but not required, that PSLVERR is driven LOW"* was one of its seventeen — but that
  refusal rides the kind classifier, which this path never reaches: it types a record from its VALUE
  BINDER. So the class is live here and nowhere gated.
  **SHIPPED `2026-09-13`. Actionable population: 2, both adjudicated, and the leaf is as small as its
  population.** AHB `dyn_sigcon_0012` (*"It is **recommended** that a Manager sets HPROT[0] HIGH"*) and
  AMBA LPI `dyn_sigcon_0009` (*"Figure 2-16 shows how a device **can** be interfaced directly to a
  controller with an absent or tied LOW QDENY signal"* — a permitted configuration, in a figure
  caption). Corpus replayed **292 → 291 over the 76 comparable documents**, and the AHB rebuild takes
  it 12 → 11; nothing else in the corpus moves. LPI's real requirement survives untouched, because the
  document states it in its own mandatory clauses (`dyn_sigcon_0007`/`0008`, *"QDENY must be tied
  LOW"*) — which is the shape of evidence that makes the refusal safe rather than merely defensible.
  **The control that defines the gate's limit is the one that keeps this producer alive.** A
  specification binds a signal FLATLY all the time — *"the FULL output is pulled HIGH"*, *"AERR, DERR
  are driven LOW"* — and those are real invariants with no modal anywhere. So the refusal needs an
  EXPLICIT non-mandatory marker, and a mandatory modal in the same clause outranks it; a permission
  granted in one sentence does not suppress the requirement stated in the next
  (*"A Manager … can set the width to 0. An attached Subordinate must have its AWSNOOP input tied
  LOW."*). A modality gate written any wider on a path that reads BINDINGS rather than OBLIGATIONS
  would refuse almost everything it exists to capture.
  **Two `.3k.11` controls were retargeted, not weakened.** Both were written from corpus sentences
  that this leaf now refuses, so each now asserts its property on `logic_level_bindings` directly —
  one rule, one control. Their reasoning is unchanged and is recorded in place.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.13`

<!-- extraction-quality-gauge-task-source-region:kind-span-successor-leaves:end -->
