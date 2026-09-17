# EXTRACTION-QUALITY-GAUGE — acceptance checklists span

- Part ID: `acceptance-checklists-span`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:checklists-span-family:start -->
### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.8`

- [x] **REPRODUCE / MEASURE** — `replay-constraints --json` over all 77 loadable documents, grouped by
  the producer's own identity minus provenance: **82 extra records across 12 documents**, of which
  **9, in one document, are the cross-producer class** — every one a `sigcon_*`/`row_sigcon_*` pair
  citing the same `supporting_statement_ids`, with the row reader's clause contained in the statement
  reader's row. Every other group was read and comes from a different sentence.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`:
  `dedup_appended_signal_constraints` keys on `signal_constraint_merge_key`, whose last field is
  `source_text`. The statement path cites the serialized row (`.3k.3`'s deliberate invariant — it is
  what `supporting_statement_ids` cites) and `extract_signal_description_row_constraints` cites the
  clause, so the key sees two facts. Observed RED against the file at `HEAD`: the dedup keeps both
  records of a pair whose provenances differ only by containment.
- [x] **ADDRESSED (verified)** — `signal_constraint_assertion_key` (the merge key without the
  provenance; the merge key is built FROM it, pinned by a test) plus a containment test applied only
  to APPENDED records. **Corpus replay 201/128 → 192/119, and APB-E is the only document that moves.**
  Its chain rebuilt from the restored held-out bundle — `evidence → validate → semantic → validate
  → intent → validate → adapt` — EvidenceIR **27 → 18**, SemanticIR/IntentIR **27 → 18**,
  `fact_provenance` 91 → 82, emitted `.isf` **56 → 38 rules with 12 distinct rule bodies before and
  after and not one body lost**. Bundle `diff -rq` byte-identical before removal; retention back to
  **24**.
- [x] **NO REGRESSION** — wire golds `signal_constraint P=R=F1=1.000` with **fp=0** on APB, AHB and
  AXI and `temporal_rule 1.000` on AXI; `kg-bench` **156/156**; `cargo fmt --all --check` and
  `cargo clippy --offline --all-targets -D warnings` clean; the whole workspace suite green
  (`specforge-core` lib 1,516 → **1,521** passing, `specforge` 472, conformance 168,
  production-graph 8/8); `flow_census.json` re-derived and attributed (+1 function, +4 decision sites,
  +4 helper edges, +1 semantic macro). The replay identity is byte-for-byte unchanged — no persisted
  record stops reproducing for a provenance reason.
- [x] **GENERICITY (ADR 0006)** — string containment between two records' own provenance fields. No
  document, protocol, vendor or signal vocabulary, and no producer is named.
- [x] **LOCKSTEP** — the book's obligation-reading chapter gains *"One obligation, read by two
  readers, is still one obligation"* under the table-cell section that owns the row reader; the
  existing `PSTRB` paragraph — two paths reaching two DIFFERENT places and agreeing — is exactly the
  case containment keeps, and is referenced rather than rewritten. No production rule was deleted. No
  KM card: the durable fact is the book's.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.10`

- [x] **REPRODUCE / MEASURE** — `replay-constraints --json` captured per document for all 77 loadable
  artifacts, then re-captured against **three** prototypes and diffed on the producer's own record
  identity. Baseline **200 persisted / 127 reproduced**. Prototype A (cut at the condition's finite
  verb): −4 fabrications, **+4 fabrications** — rejected on the spot, each of the four read against
  source. Prototype B (boundary comma, verb as fallback): −4, +0, but AXI-L's rebuild showed it costs
  the one true `BCOMP` record. Shipped (B + the pronoun antecedent): **−4 fabrications, +6 true
  records**, every one of the ten adjudicated against its own sentence.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `text_before_condition_marker` matches `" when "`/`" if "`/… **with a leading space**. The sentence
  splitter leaves that space in front of every sentence of a statement except the FIRST, so a
  statement that opens with its condition is read as though the condition were part of the subject.
  Observed RED against the restored pre-change file: the AXI sentence yields
  `["ZETAVALID", "ZETAADDR", "ZETAPROT", "ZETASNOOP"]` where the document constrains the last three.
  The committed `.3k.3` control `a_pronoun_subject_does_not_borrow_a_sibling_clauses_signals` asserted
  that same wrong record as expected output and is updated here, which is the second RED.
- [x] **ADDRESSED (verified)** — `main_clause_of_fronted_conditional` reads where the fronted clause
  ends (a comma that is not part of a coordinated list, else the condition's own finite verb) and
  `fronted_condition_subject` reads the antecedent of a pronoun that heads the main clause.
  **Corpus: 200/127 → 201/128.** AXI-L's chain rebuilt from its restored held-out bundle —
  `evidence → validate → semantic → validate → intent → validate → adapt` — **54 → 56** signal
  constraints with **nothing removed**: two recovered `AWAKEUP` requirements the document states and
  the `BCOMP` record kept. SemanticIR and IntentIR **53 → 55**, and the emitted `.isf` **130 → 135**
  rules. Bundle `diff -rq` byte-identical before removal; retained population back to **24**.
- [x] **NO REGRESSION** — wire golds `signal_constraint P=R=F1=1.000` with **fp=0** on AXI, APB and
  AHB, `temporal_rule 1.000` on AXI, document-level recall 1.000; `kg-bench` **156/156**;
  `cargo fmt --all --check` and `cargo clippy --offline --all-targets -D warnings` clean; the whole
  workspace suite green (`specforge-core` lib 1,508 → **1,516** passing, `specforge` 472, conformance
  168, production-graph 8/8); `flow_census.json` re-derived and attributed (+2 functions,
  +25 decision sites, +11 helper edges). AHB and APB-E do not move — they still LOAD, which is the
  standing proof-staling signal used as the per-document currency check.
- [x] **GENERICITY (ADR 0006)** — English clause structure only: the condition markers the infix
  reader already uses (held to one set by a test), the two coordinators, the finite forms of the
  copulas the head walk already skips, and the two subject pronouns. No document, protocol, vendor or
  signal vocabulary.
- [x] **LOCKSTEP** — the book's *"An obligation in a table cell belongs to whatever precedes its
  modal"* section states the pronoun refusal as unconditional (*"Resolving that is anaphora, not a
  rule, so SpecForge promotes neither"*); that is now true of only one of its two examples and is
  rewritten in the same edit, with the fronted-conditional reading given its own section in the
  chapter that owns this concern. No production rule was deleted. No KM card: the durable fact is the
  book's, in the same shape `.3k.5` recorded.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.7`

- [x] **REPRODUCE / MEASURE** — `replay-constraints --json` captured per document for all 77 loadable
  artifacts, then re-captured against a prototype of the WIDEST version of this rule and diffed on the
  producer's own record identity. Baseline **200 persisted / 127 reproduced**; the wide rule removes
  **12 records across 7 documents** and moves AXI-L's content (it then refuses to LOAD, which is the
  standing hazard used as a signal). Every one of the twelve was read against its source and
  adjudicated; the node's "three reproduced persisted records" is **2** — MMU-700 `dyn_sigcon_0008`
  already fails to reproduce for an unrelated reason, which the verdict's `refused_by` confirms.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `obligation_head_is_a_foreign_identifier`: it reads `content_head` alone, so a clause whose subject
  is a common noun premodified by an identifier (`WTAG bits`) presents a head that is not an
  identifier, the `INVARIANT-SHAPE-ADMISSION.5` narrowing declines, gate 2 exempts the row, and gate 4
  never runs. Observed RED on the pre-change producer with the two controls installed verbatim against
  the restored original file: `is_post_passive_binding_only_subject_in` returns `false` for `OMEGASTRB`,
  and `extract_signal_constraints` mints `sigcon_0003 OMEGASTRB MustBeValue { value: "VALID" }` from the
  row. Live instance: AXI-L `sigcon_0027`, AXI-H `sigcon_0043`.
- [x] **ADDRESSED (verified)** — the head test reads the premodifier when the head carries no
  identifier, through one new `content_head_with_premodifier` that `content_head` now delegates to (so
  the two readings cannot drift) and one new `whole_token_identifier` shared by both slots.
  **Corpus: 200/127 → 199/126 over 77 documents, a single record removed, and it is this leaf's.**
  AXI-L's chain rebuilt from its restored held-out bundle — `evidence → validate → semantic →
  validate → intent → validate → adapt` — **55 → 54** signal constraints, `fact_provenance`
  312 → 311 losing exactly `WSTRB|MustBeValue { value: "VALID" }|`, SemanticIR/IntentIR
  `signal_constraints` unchanged at 53, `.isf` identical but for rule numbering. Bundle `diff -rq`
  byte-identical before removal; retained population back to **24**.
- [x] **NO REGRESSION** — wire golds `signal_constraint P=R=F1=1.000` with **fp=0** on AXI, APB and
  AHB, and `temporal_rule 1.000` on AXI; `kg-bench` **156/156**; `cargo fmt --all --check` and
  `cargo clippy --offline --all-targets -D warnings` clean; the whole workspace suite green
  (`specforge-core` lib 1,503 → **1,508** passing, `specforge` 472, conformance 168, production-graph 8/8);
  `flow_census.json` re-derived and attributed (+2 functions, +4 decision sites, +3 helper edges).
  The 12 records the wide rule destroyed all survive, verified by diffing the shipped snapshot against
  the baseline: the only difference corpus-wide is AXI-H `sigcon_0043`.
- [x] **GENERICITY (ADR 0006)** — noun-phrase premodification plus an adjacency test that is pure
  punctuation structure. No document, protocol, vendor, value or token list; the closed helper-word
  list it walks is the one `content_head` already had.
- [x] **LOCKSTEP** — the book's *"An obligation in a table cell belongs to whatever precedes its
  modal"* section carried a disposition table whose "a different nominal" row was true only of a bare
  identifier head; the descriptor row and the adjacency rule are added there, in the chapter that owns
  this concern. No production rule was deleted or replaced, so no standing book text describes gone
  behaviour. The KM card this slice adds is for the SemanticIR finding, not for this rule — the
  durable fact here is the book's.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.5`

- [x] **REPRODUCE / MEASURE** — `replay-constraints` over all 77 loadable documents: **302 replayed
  and 127 reproduced, before and after — nothing moves.** The four statements the node was sized from
  were counted with a Python mirror before `.3k.3` and `.3k.2k` existed, and neither survives as this
  leaf's population: the record it expected to recover is refused by `.3e`'s descriptive-field-cell
  gate, and the record it expected to expose is refused by `.3k.2k`. The class is nevertheless
  demonstrable through the real reader, which is the `.3k.1` footing `.3k.2e`/`.3k.2f`/`.3k.2k`
  shipped on.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. Both
  `is_relational_equality_constraint` (`.3d`) and `is_reference_magnitude_constraint` (`.3k.1`) were
  evaluated over `&statement.text` in both deterministic statement-level producers, while the records
  they suppress became clause-scoped in `.3k.3` and `.3k.4`. So a relation mentioned anywhere in a
  statement refused every obligation that statement states — the container's own defect, on the
  refusal side.
- [x] **ADDRESSED (verified)** — the two guard clauses moved inside the loops that already hold the
  clause: the obligation in `extract_signal_constraints`, the binding in
  `extract_dynamic_signal_constraints` (both binders). **Three controls in
  `mod extraction_quality_gauge_3k_5`, two observed RED with the refusals put back on the statement**
  and the file restored byte-identically: a relation in another clause no longer refuses this
  obligation, the clause that STATES the relation is still refused, and a statement stating both keeps
  only what the vocabulary can hold.
- [x] **NO REGRESSION** — corpus replay identical (302/127), so no persisted artifact moves and no
  chain rebuild is owed; all 77 documents still load. Wire golds hold: `signal_constraint
  P=R=F1=1.000` with **fp=0** on APB, AHB and AXI. `kg-bench` **156/156**; `cargo fmt --all --check`,
  `cargo clippy --offline --all-targets -D warnings` and the whole workspace suite green
  (`specforge-core` lib 1,500 → **1,503**). `flow_census.json` re-derived and attributed: one decision
  site, no new function.
- [x] **GENERICITY (ADR 0006)** — a scope change to two existing rules; no new vocabulary. The three
  leads that WOULD have been vocabulary are explicitly not added, with the measurement recorded.
- [x] **LOCKSTEP** — the book's *"A bound stated against another operand is not a value"* section says
  *"Both readers now refuse, for the same reason, on the clause each is holding"* — which this leaf is
  what finally makes true of all three producers, so the sentence stands and is now accurate rather
  than aspirational. `is_reference_magnitude_constraint`'s doc comment asserted the gate *"is evaluated
  over the whole statement"*; that is now false and is rewritten in the same edit, together with the
  measurement that keeps the leads out. No KM card: the durable fact is
  `[[one-record-per-obligation-clause]]`'s, and this leaf is its refusal-side completion rather than a
  new mechanism.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.13`

- [x] **REPRODUCE / MEASURE** — the population is **2 records**, surfaced by `.3k.11`'s own
  adjudication of all 22 reproduced logic-level records and confirmed with `replay-constraints`:
  corpus replayed **292 → 291** over the 76 comparable documents, and AHB 12 → 11 on rebuild. AHB
  `dyn_sigcon_0012` is a RECOMMENDATION; AMBA LPI `dyn_sigcon_0009` is a PERMISSION in a figure
  caption. Nothing else in the corpus moves.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_dynamic_signal_constraints`. This producer types a record from its VALUE BINDER and never
  from a modal — correctly, because a flat binding is a real invariant — so it had no modality gate at
  all. `EXTRACTION-QUALITY-GAUGE.3k.2a` refuses the same shape in the statement path, but that refusal
  rides the kind classifier, which this producer never reaches.
- [x] **ADDRESSED (verified)** — `binding_is_non_mandatory`, applied to the clause each binder bound
  in, for both binders. **Five controls in `mod extraction_quality_gauge_3k_13`, two observed RED with
  the predicate stubbed to `false`** and the file restored byte-identically. The other three are the
  gate's limit: a flat binding with no modal at all still mints, a mandatory modal outranks a
  permission in its own clause, and a permission in another clause does not suppress the requirement.
  AHB rebuilt (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated
  exactly once, upstream-first; bundle restored, `diff -rq` clean, removed, retention back to **24**).
- [x] **NO REGRESSION** — the wire golds hold: `signal_constraint P=R=F1=1.000` with **fp=0** on APB,
  AHB and AXI, document-level fact recall **1.000**. `kg-bench` **156/156**; `cargo fmt --all --check`,
  `cargo clippy --offline --all-targets -D warnings` and the whole workspace suite green
  (`specforge-core` lib 1,495 → **1,500**). `flow_census.json` re-derived and attributed. Only AHB's
  artifacts move, and only by the one record.
- [x] **GENERICITY (ADR 0006)** — universal English deontic modality, the RFC-2119 distinction every
  specification in this corpus is written against. No document, protocol or vendor vocabulary; every
  test identifier alpha-renamed.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` gains the distinction beside the dynamic
  path's own section, since that section is what explains why this producer reads bindings rather than
  obligations. KM card: `[[a-level-belongs-to-a-signal]]` is amended rather than duplicated — it
  already carries this residual as the one the adjudication surfaced, and it is the same reader.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.11`

- [x] **REPRODUCE / MEASURE** — the actionable population derived with `replay-constraints`: of the
  dynamic path's judged records, **22 reproduce with a logic-level kind**, and all 22 were adjudicated
  individually against their own source before anything was written. Six were wrong — AHB `HTRANS`
  (the level belongs to `HSEL`), LPI `PREQ must_be_high` ×2 (it belongs to `PREQCHK`), NVMe
  `NVM`/`LBA must_be_low` (*"low level format"*), and AHB `HPROT` (right pairing, wrong MODALITY —
  routed to `.3k.13`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `logic_level_binding_kind_from_text` (now retired). It returned the LAST logic level within six
  words of a binding verb and said nothing about what that level belonged to; the caller then attached
  that one kind to EVERY declared signal the statement named. Two independent errors in one reader, and
  both are visible in one row: `| P_ACCEPT | … | Controller must set PREQ LOWand PREQCHK HIGH. |`
  publishes `PREQ must_be_high`.
- [x] **ADDRESSED (verified)** — `logic_level_bindings` pairs each level with the signals ADJACENT to
  it, walking backward first and forward when backward finds nothing, stopping at another level, and
  reading identity through the DOCUMENT'S OWN CATALOG. **Eight controls in
  `mod extraction_quality_gauge_3k_11`, three observed RED by removing exactly one rule each** — the
  level delimiter (`ZETAREQ must_be_high` reappears), the per-level subjects (`ZETATRANS must_be_high`
  reappears), and the forward walk — with the file restored byte-identically each time.
  **Corpus replayed is 304 before and 304 after, and the composition is the result: 11 fabrications
  removed, 15 correct records added, 1 correct record lost and named** (`.3k.12`). Every one of the 27
  is listed with its sentence in the node. **Two documents rebuilt** (AHB 13 → 12, AXI-L 53 → 55), the
  bundles restored from `generated/preserved/WIRE-BASED-100.10/`, `diff -rq` clean, removed again,
  retention back to **24**.
- [x] **NO REGRESSION** — the wire golds hold: `signal_constraint P=R=F1=1.000` with **fp=0** on APB,
  AHB and AXI, `temporal_rule 1.000`, document-level fact recall **1.000** for constraints and
  relations. `WIRE-BASED-100.5i`'s alpha-invariance control passes and is the control that shaped the
  design. `kg-bench` **156/156**; `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D
  warnings` and the whole workspace suite green (`specforge-core` lib 1,487 → **1,495**).
  `flow_census.json` re-derived and attributed.
- [x] **GENERICITY (ADR 0006)** — universal English adjacency plus the logic-level vocabulary the
  repository already carries, with identity read only through the document's own declaration catalog.
  The alpha-invariance control is the proof rather than the claim: an opaque signal alias binds exactly
  as a conventional name does, and the same sentence with an undeclared name binds nothing.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md`: `.3k.4`'s section said a level *"is
  currently paired with the verb that sets it"* and that *"recall waits for that to be fixed"* — that
  is now false and is rewritten in the same edit, which is the `BOOK-BEHAVIOUR-CURRENCY` case of a
  changed rule leaving standing book text. KM card `[[a-level-belongs-to-a-signal]]`. Two residuals
  routed to `.3k.12` and `.3k.13` rather than left in prose.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.4`

- [x] **REPRODUCE / MEASURE** — built as a prototype and measured with `replay-constraints` before the
  design was settled, which rejected TWO wider shapes. (a) Narrowing the SUBJECT to the binding clause:
  **99 → 69 reproduced**, ten NVMe records lost because a register row names its subject in the cell
  mnemonic and binds in the body. (b) Searching the binders per clause: **+6 records in AMBA LPI**,
  three of them `must_be_high` off rows that set the signal LOW. The shipped shape measures
  **304 → 304 replayed in every one of the 77 loadable documents — zero added, zero removed — and 19
  corrected**, each adjudicated individually below.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_dynamic_signal_constraints`. `.3i` gave this path `constraint_bearing_sentence` for its
  negation, which locates an obligation MODAL — and this producer's record is minted by a VALUE
  BINDING that need not be modal at all. So the negation came from whichever clause happened to carry
  a `must`/`cannot` while the condition came from the whole statement, and neither had to be the
  clause that bound the value. Evidence: AMBA LPI `dyn_sigcon_0014`/`0015` published `must_be_high`
  **negated** from a `cannot` two sentences away; MMU-700 `dyn_sigcon_0007`/`0008` carried a condition
  that says the signal is not valid.
- [x] **ADDRESSED (verified)** — `binding_bearing_clause` finds the binding statement-wide exactly as
  before and then locates the first clause that reproduces it, failing OPEN to the whole statement
  when none does; `condition_text` and `negated` read that clause. The kind and value are unchanged by
  construction and a control asserts it. **Five controls in `mod extraction_quality_gauge_3k_4`, two
  observed RED with the two reads reverted** — the condition control reporting the exact corpus
  string `"ZETARESP is FaultAbort, this signal is not valid. Width is 3-bit. |"` — and the file
  restored byte-identically. All 19 corrections adjudicated: 8 conditions from a foreign clause, 5
  conditions running past their clause, 5 negations from a clause two sentences away, 1 run-on
  condition across a duplicated cell.
- [x] **NO REGRESSION** — `replay-constraints --evidence-root generated/evidence_ir`:
  **304 replayed before and after, in every document**, so no persisted artifact moves, all three
  current-schema documents still load, and no chain rebuild is owed. `kg-bench` **156/156**;
  `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings` and the whole
  workspace suite green (`specforge-core` lib 1,482 → **1,487**). `flow_census.json` re-derived and
  attributed: `analyzed_functions` +2, `decision_sites` +5, `helper_edges` +3, `semantic_macros` +1.
- [x] **GENERICITY (ADR 0006)** — the clause split is the repository's existing punctuation split and
  the binding is located by asking the PRODUCER's own binder, so no second rule, no vocabulary, no
  document, protocol or vendor name. Every test identifier is alpha-renamed.
- [x] **LOCKSTEP** — book `commands/quality-and-learning.md` documents the three fields
  `replay-constraints` verdicts gained, because a reader is told a record moved and must be able to
  see what moved; `pipeline/obligation-reading.md` gains the dynamic path's own span rule beside the
  statement path's. KM card `[[the-binding-bearing-clause]]`. The LPI pairing defect is routed to
  `.3k.11` rather than left in prose.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.3`

- [x] **REPRODUCE / MEASURE** — the change was built as a PROTOTYPE and measured with the real
  producer before any of it shipped, which the node required and which changed the design twice.
  `replay-constraints --json` captured per document before and after, diffed over the 74 documents
  comparable in both (AHB/AXI-L/APB-E drop out — the change moves their content, which stales their
  proofs so they refuse to LOAD, and that is itself the signal that they need rebuilding). Final:
  **187 → 211 replayed; 61 → 59 reproduced, and the only two lost are NVMe `sigcon_0005`/`0006`, this
  leaf's own population**. The first prototype measured **+19 with three fabrications**
  (`ACADDR/ACPROT/ACSNOOP must_be_asserted` from a clause whose subject is the pronoun `it`;
  LTI `signal`/`LAFLOW must_be_low`), and the second confirmed the container's ordering rationale by
  publishing NVMe `must_be_value UNIQUE` — which is why `.3k.2k` was opened and landed first.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_signal_constraints`. The loop ran ONCE per statement over
  `constraint_bearing_sentence(text)`, the FIRST clause carrying a modal, and classified the kind over
  the WHOLE statement. So obligations 2..n were dropped, and the one record that survived could take
  its kind from one clause and its condition from another — AHB `| HSELx a | … |` did exactly that.
  Three readings underneath it were wrong for the same reason: a FRONTED condition cuts
  `text_before_condition_marker` at offset 0 and leaves no subject part; the subject fallback then
  scanned the whole STATEMENT; and `is_post_passive_binding_only_subject` re-derived the obligation as
  the statement's FIRST clause, so it judged the Nth record against the 1st record's clause.
- [x] **ADDRESSED (verified)** — `constraint_bearing_sentences` (all obligation clauses in document
  order, with the singular helper as its first element so the two cannot drift), a per-obligation
  loop, `obligation_subject_part` (the main clause of a fronted conditional), an obligation-bounded
  subject fallback, `is_post_passive_binding_only_subject_in` (told which obligation it judges), and
  an in-statement dedup on the producer's own merge identity. **Eight controls in
  `mod extraction_quality_gauge_3k_3`, each part observed RED by disabling exactly that part and the
  file restored byte-identically each time:** all-clauses (2 RED), the fronted-condition subject
  (2 RED), the obligation-bounded fallback (1 RED — `ZETASEL must_be_stable`, a second contradictory
  kind for the signal the first clause constrains), the dedup (1 RED). **Three documents rebuilt**
  (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated exactly
  once, upstream-first, bundles restored from `generated/preserved/WIRE-BASED-100.10/` and removed
  again with `diff -rq` clean, retention back to **24**): AHB 13 → 13 with its condition corrected,
  APB-E 23 → 27, AXI-L 40 → 53 — and `WTAGUPDATE must_be_value UPDATED` removed, closing `.3k.2b`'s
  named residual. `.3i`'s inherited claim is closed too: every part of a `sigcon_*` record now comes
  from one span.
- [x] **NO REGRESSION** — the wire golds hold at the bar: `signal_constraint P=R=F1=1.000` with
  **fp=0** on APB, AHB and AXI, `temporal_rule 1.000` on all three, and document-level fact recall
  **1.000** for constraints and relations. Section-by-section the rebuilt EvidenceIRs move ONLY
  `signal_constraints`, their derived `fact_provenance`, and `conditional_rules` **by id alone**
  (content byte-identical, ids shifted by the shared counter) — `actor_signal_relations` is untouched,
  which is why the golds' `drives` attribution numbers are unchanged and pre-existing. AHB's IntentIR
  and `.isf` are unchanged entirely. `kg-bench` **156/156**; `cargo fmt --all --check`, `cargo clippy
  --offline --all-targets -D warnings` and the whole workspace suite green (`specforge-core` lib
  1,474 → **1,482**). `flow_census.json` re-derived and attributed: `analyzed_functions` +3,
  `decision_sites` +6, `helper_edges` +37, `semantic_macros` +1. AXI-L gains one temporal conflict
  (`WTAG` VALID vs ZERO) — a correct surfacing of an ambiguity the document states across two enum
  rows, not a regression.
- [x] **GENERICITY (ADR 0006)** — universal English clause structure only: a statement decomposes into
  clauses at the punctuation the row reader already uses, an obligation is a clause carrying one of
  the modals `.3k.2d` fixed, and a fronted conditional's main clause follows its comma. No document,
  protocol, vendor or token list; every test identifier is alpha-renamed.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` gains *"A statement that states three
  obligations yields three records"* as the chapter's first rule, since every other rule in it now
  operates per obligation. No book text described the old one-record-per-statement behaviour, so
  nothing is deleted. KM card `[[one-record-per-obligation-clause]]`. Four residuals routed to
  `.3k.7`–`.3k.10` rather than left in prose.

<!-- extraction-quality-gauge-task-source-region:checklists-span-family:end -->
