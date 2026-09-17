# EXTRACTION-QUALITY-GAUGE — kind span scoping

- Part ID: `kind-span-scoping`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:kind-span-scoping-and-direct-leaves:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.3k` · Status: `active` (opened `2026-09-12` by `.3i`; **scoped +
  split** `2026-09-12`) · Children: `.3k.1`–`.3k.6` (`.3k.2` carries `.3k.2a`–`.3k.2j`) · Goal: **every part of a
  published constraint must be read from the span that produced the record.** `.3i` established that
  for the negation; this container owns the rest. Its first result is that **both numbers `.3i`
  handed it were measured over the wrong population**, so the leaf is split around the populations
  that actually exist rather than around the two it inherited.
  **The scoping `.3k` demanded, done** — `scripts/measure_constraint_part_span.py`, read-only over
  all 78 persisted artifacts / 349 signal-constraint records, stratified by PRODUCER:
  `classify_signal_constraint_kind` has exactly **two** callers.
  `extract_signal_description_row_constraints` (`row_sigcon_*`, 12 records) already hands it ONE
  clause and is the reference implementation for this whole container — its census rows are 0/0/0.
  `extract_signal_constraints` (`sigcon_*`, 111 records) is the only caller that reads the whole
  statement. The other two producers of the same record type **never reach the classifier**: the
  dynamic path types a record from its VALUE BINDER
  (`extract_discovered_state_value_from_text` / `logic_level_binding_kind_from_text`) and the LLM
  path parses the kind the model NAMED (`parse_kind`).
  **Correction 1 — the kind-span population is 4, not 18.** Running the clause-vs-whole comparison
  over all 349 records gives **19** (not 18), of which **12 are `llm_sigcon_*` and 3 are
  `dyn_sigcon_*` — strata whose kind never passes through the function the change would edit**. The
  classifier's own population is **4 `sigcon_*` records**: AHB `sigcon_0002` and NVMe
  `sigcon_0005`/`0006`/`0007`.
  **Correction 2 — the "7 negated records on an untyped default" do not exist.** All 7 are
  `dyn_sigcon_*`, and that path **never publishes the untyped default**: every one of its 77 records
  carries a kind its own binder typed (`must_be_high` 15 / `must_be_low` 23 / `must_be_value` 39).
  The 7 was read off a classifier that does not run on them. The real population of "a negation
  stacked on a kind the document never typed" is **4 records — DTI `sigcon_0002`–`0005` — and they
  are the SAME 4 records as the relational magnitudes**, not a separate 7 plus 4. DTI publishes
  `OAS must_be_stable, negated` and `DTI must_be_stable, negated` — *"OAS must not be stable"* — from
  *"The range given by this field must not be greater than the size indicated by the OAS field …"*,
  a sentence that names no stability, whose subject is *"this field"*, and in which `OAS` is the
  right operand and `DTI` a message-name prefix. One refusal removes all three defects at once.
  **The decision the leaf asked for — what a clause is, per producer.** A record's parts must come
  from the span that PRODUCED the record, and that span is not the same construct for every producer:
  the pattern path is minted by a MODAL OBLIGATION, so its clause is `constraint_bearing_sentence`
  (already its subject's, condition's and negation's span — the kind is the only part still outside);
  the row path is minted by ONE clause of a description cell and already uses it throughout; the
  dynamic path is minted by a VALUE BINDING that **need not be modal at all** (*"X is tied HIGH"*),
  so `constraint_bearing_sentence` is the WRONG narrowing for it — it locates a modal the record may
  not have, and would silently move the record's span to an unrelated sentence. The dynamic path
  needs a BINDING-bearing clause, which no helper computes today; that is why `.3k.4` is separate
  rather than "call the same helper in both paths". The LLM path is out of scope here: a model names
  a subject and a kind deliberately rather than scanning a span, so the span question is a different
  question (`.3j`).
  **Ordering rationale.** `.3k.1` is strictly subtractive and landed first. `.3k.2` must land before
  `.3k.3`, because narrowing the kind's span moves 3 of its 4 records onto the ungated `generic_value`
  arm (NVMe would publish `ANAGRPID must_be_value UNIQUE`, a value lifted off the adjective following
  `shall be`); fixing the span before the arm would trade one fabricated fact for another. `.3k.6`
  (opened by `.3k.1`) should land before `.3k.2` is sized, for the reason `.3k.1` discovered.
  **Amendment (`2026-09-12`, from `.3k.1`) — every population in this node is a PUBLISHED population,
  not an actionable one.** Only 24 of the 78 documents keep a normalized bundle, so the other 54
  evidence artifacts are frozen at whatever generation wrote them and can carry records the current
  extractor would not mint. `.3k.1`'s four DTI records turned out to be exactly that: published, and
  reproducible by nothing. Every remaining child must re-derive its population by running the real
  producer on each record's own `source_text` before sizing its change
  (`[[persisted-census-measures-published-not-current]]`); `.3k.6` shipped the instrument that makes
  that mechanical, and its corpus answer is **144 of 179 reproduce, 35 do not** — so roughly one
  published deterministic constraint in five is not what this code would produce today.
  **Amendment (`2026-09-13`, from `.3k.2e`-`.3k.2j`) — the ROW path's populations are bounded by what its
  producer can SEE, and that is 27 of 78 documents.** `extract_signal_description_row_constraints` selects
  tables by `TableKind::SignalDescription`, and a legacy `SourceIr` is loaded with every classification
  neutralized to `Unknown`, so 51 documents are invisible to it
  (`[[legacy-source-classifications-are-neutralized-on-load]]`). Measured over the 27 it can see and the
  102 tables that pass its own gate: it mints **12 records, all already published**, and `.3k.2e`,
  `.3k.2f`, `.3k.2h` and `.3k.2i` each measure an actionable population of **zero**. Two of them shipped
  anyway, on `.3k.1`'s footing, because their class is demonstrable through the real reader; two did not,
  because theirs is not. **The remaining row-path work is therefore blocked on re-ingest rather than on
  analysis**, and the measurable frontier moves back to the statement and dynamic paths — `.3k.3`,
  `.3k.4`, `.3k.5` — which see all 78.
  Verification: `python3 scripts/measure_constraint_part_span.py --self-test` (9/9) and `--check`
  (`kind-classifier call sites unchanged (2 callers, 1 reading the whole statement)`) both green; the
  census above re-derived from the persisted corpus, every listed record adjudicated against its own
  source text in this node. **Two observed-RED controls, because this census exists to stop a number
  being published from something nothing exercised:** (a) a third caller injected into `evidence.rs`
  (`zeta_probe_caller`) makes `--check` exit 1 naming the found set against the expected one, and the
  file was restored byte-identical (`git status --short crates/` clean); (b) deleting one self-test
  case makes `--self-test` exit 1 with `ran 8 cases, expected 9` — the total is a literal declared
  independently of the case list, the `PRODUCTION-GRAPH-CENSUS-PIN.3` property.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k`

- ID: `EXTRACTION-QUALITY-GAUGE.3k.1` · Status: `done` (`2026-09-12`, CODE) · Goal: **refuse a
  comparative MAGNITUDE whose right operand is a REFERENCE.** `.3d` already refuses an inter-operand
  EQUALITY (*"X must be equal to the value of Y"*) because the constraint vocabulary has no slot for
  it; *"must not be greater than the size indicated by the OAS field"* is the same shape one relation
  along, and the vocabulary has no slot for it either. A magnitude against a LITERAL (*"must be
  greater than 0"*) must stay untouched — it is a value binding, and `.3d`'s own line between "the
  value of <other>" and a literal is the line this reuses.
  **The population statement this leaf opened with was wrong, and finding out why is its main
  result.** It said *"4 records, all `sigcon_*`, all DTI"*. Those four records are published, but
  **today's extractor reproduces none of them**: every candidate subject in DTI's sentence is named
  only AFTER the obligation's lead, so `CORPUS-COVERAGE.2.50a`'s pre-lead subject authority
  (`is_post_passive_binding_only_subject`) reaches it first and `extract_signal_constraints` returns
  an empty vector. Verified by running the real producer on the live sentence with `OAS`/`DTI`
  declared — `records=[]`, `post_passive OAS=true DTI=true`. The four records predate that gate and
  the document has no retained normalized bundle, so the artifact is frozen where it is.
  **The CLASS is nevertheless live, and that is why this shipped rather than closing as covered.**
  The same grammar with the constrained signal named BEFORE the lead still mints the fabricated pair:
  *"ZETARANGE must not be greater than the size indicated by the ZETAOAS field"* → `MustBeStable` +
  `negated: true`, i.e. **"ZETARANGE must not be stable"** — exactly what DTI published. The dynamic
  path is reachable too: *"The controller drives ZETARANGE LOW whenever the requested span is larger
  than the number of entries the ZETAOAS field reports"* → `ZETARANGE must_be_low` AND
  `ZETAOAS must_be_low`, the right operand minted as a second subject.
  Shipped: pure `is_reference_magnitude_constraint` — a comparative marker IMMEDIATELY followed by a
  phrase naming another operand's attribute — wired beside `.3d`'s refusal in BOTH deterministic
  paths. +6 tests, `specforge-core` lib 1,435 → 1,440.
  **Amended `2026-09-13` by `.3k.2k`: one of this leaf's controls pinned a fabrication.**
  `a_magnitude_against_a_literal_still_yields_its_constraint` asserted that *"The value of ZETARANGE
  must be greater than 0"* still yields a record and called that record a value binding. It is
  `ZETARANGE must_be_value GREATER` — the comparative in the value slot, with the literal `0` the
  sentence names nowhere in it. The line this leaf drew is real and stands: a LITERAL right operand is
  not a REFERENCE magnitude and `is_reference_magnitude_constraint` correctly returns false for it. But
  that is a difference between two REFUSALS, not between a refusal and a capture — the vocabulary has
  no `at least` kind, so the literal shape ends as a residual too, reached by `.3k.2k`'s value-slot
  rule instead. The control now pins this leaf's own gate verdict, which is what it was reaching for.
  **Honest limits, both named rather than absorbed:** (a) the four DTI records stay in the persisted
  artifact until that document is re-ingested, exactly as `.3h`'s NVMe `FFFF` record does; (b) the
  lead list is the measured one — extending it with `that supported by` / `the maximum` / `the
  minimum` would refuse RISC-V IOMMU `dyn_sigcon_0008`/`0009`, whose own obligation clause states no
  relation at all, which would be right by accident and is owned by `.3k.5`.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.1`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.5` · Status: `done` (`2026-09-13`, CODE; opened `2026-09-12` by
  `.3k.1`) · Goal:
  **the refusal gates are statement-scoped while the records they suppress are clause-scoped.**
  `is_relational_equality_constraint` (`.3d`) and `is_reference_magnitude_constraint` (`.3k.1`) are
  both evaluated over the WHOLE statement, so a relation stated in one sentence refuses an obligation
  minted from another — the same span defect this container is about, one level up, on the refusal
  side. **Measured, read-only over 261,508 persisted statements:** the equality phrase appears in 181
  statements and lies OUTSIDE the obligation clause in **4** of them; clause-scoping would admit
  those 4, which recovers one real constraint (NVMe `statement_4474`, *"all bytes of this field shall
  be cleared to 0h"*, currently refused because the cell's descriptive body says *"contains the same
  value as reported in …"*) and exposes one fabricated one (NVMe `statement_5826`, *"The Port
  Identifier … shall be unique"* → `must_be_value UNIQUE`, which is `.3k.2`'s class). Two more are
  unadjudicated. The magnitude leads this leaf owns (`that supported by`, `the maximum`, `the
  minimum`) can only be added once the span is decided: statement-scoped they refuse RISC-V IOMMU
  `dyn_sigcon_0008`/`0009` for a relation in a later sentence. **SHIPPED `2026-09-13` — the SPAN. The LEADS are deliberately NOT shipped, and
  finding out why is this leaf's second result.** Both refusals now read the OBLIGATION in the
  statement path and the BINDING in the dynamic path; the row reader has read them that way since
  `.3k.2f`, so all three producers finally agree. **Corpus: replayed 302 → 302 and reproduced 127 →
  127 across all 77 loadable documents — nothing moves, nothing is rebuilt.**
  **The four statements this leaf was sized from no longer describe the code.** They were counted with
  a Python mirror in `.3k.1`'s day; since then `.3k.3` made the statement path read per obligation and
  `.3k.2k` refused a relational predicate in the value slot. The one record the node expected to
  RECOVER (NVMe `statement_4474`) is not recovered, and the reason is worth keeping: it is refused by
  `EXTRACTION-QUALITY-GAUGE.3e`'s descriptive-field-cell gate — a THIRD statement-scoped gate this
  leaf does not own. The one it expected to EXPOSE (NVMe `statement_5826` → `must_be_value UNIQUE`)
  cannot appear, because `.3k.2k` refuses it. **The prerequisite did its job in a way the node could
  not have predicted: it emptied the admitted set instead of cleaning it.**
  **The leads (`that supported by`, `the maximum`, `the minimum`) are unblocked and still unshipped.**
  `.3k.1` could not add them because statement-scoped they refused RISC-V IOMMU `dyn_sigcon_0008`/
  `0009` for a relation in a later sentence; clause-scoped that objection is gone, and the shapes are
  live — `the maximum` follows a comparative **28** times in the corpus, `the minimum` **17**, `that
  supported by` **6**, in sentences like *"a TID value that is greater than the maximum supported
  TID"*. They are not added because **nothing reaches them**: in the statement path `.3k.2a` refuses a
  clause that types no kind and `.3k.2k` refuses a value slot holding the comparative itself, so every
  candidate is gone before this gate is asked, and I could not construct a reachable case. Adding
  vocabulary no case exercises is what this family refuses to do. The reason is written into the
  gate's own doc comment so the next reader inherits the measurement rather than the plan.
  Prerequisite: `.3k.2` (so the admitted-set is not a fabrication set) — **satisfied**. Verification:
  see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.5`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.6` · Status: `done` (`2026-09-12`, CODE) · Goal: **an instrument
  that answers "does today's extractor still produce this persisted record".** `.3k` sized its
  children from the persisted corpus and `.3k.1` then discovered the corpus is not one code
  generation: only 24 of 78 documents keep a normalized bundle (plus APB/AHB/AXI held out under
  `generated/preserved/WIRE-BASED-100.10/`), so the other 54 artifacts are frozen at whatever
  generation wrote them. Hand-writing a unit test per record does not scale to `.3k.2`'s population.
  **Shipped: `specforge replay-constraints <evidence-ir>` / `--evidence-root <root>`.** It re-runs the
  REAL producer (`extract_normative_signal_constraints`) over an artifact's own `extracted_statements`
  and compares by the producer's own merge identity — subject, kind, value, condition, negation and
  source text, never the ids. It works for the frozen 54 because the deterministic constraint surface
  is a function of the STATEMENTS, not of the PDF, and it reads the legacy/proofless stratum through
  `load_for_inspection` rather than the canonical loader that refuses it.
  **The corpus answer: 144 of 179 published deterministic records still reproduce; 35 do not.** By
  cause: 17 have no positional gate against them (their kind, condition or negation moved), 12 are
  refused by `CORPUS-COVERAGE.2.50a`, 4 by `.3k.1`+`.2.50a` together (the DTI class), 1 by `.3h`, 1 by
  `.3g`. Ten documents are partial and one — AMBA CXS, 0/2 — is fully frozen. **One artifact is a
  named skip**, not a silent omission: I2C is current-schema with a stale proof, which
  `load_for_inspection` still verifies.
  **Two properties make the verdict usable.** (a) A published subject the artifact's statements no
  longer declare is GRANTED a synthetic `Signal <name> is …` declaration, so "not reproduced" can
  never quietly mean "the catalog shrank" — 67 subjects corpus-wide needed one, which is itself a
  finding. (b) The verdict is ASYMMETRIC and the command says so: "not reproduced" is sound because a
  widened catalog can only admit more subjects, while `unpersisted_replay_records` is not a drift
  measure, because the build applies convergence stages the replay does not.
  **The first version of the catalog widening was wrong and a control caught it**: it inserted names
  into the `HashSet` passed to `extract_normative_signal_constraints`, which only the
  inference-antecedent sibling reads — both deterministic paths derive their own catalog from the
  statements. The corpus figure moved 120/179 → 144/179 once the widening was done with declaration
  STATEMENTS instead. That is the same failure shape as the rest of this family, caught this time by a
  control written before the number was published.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.6`
<!-- extraction-quality-gauge-task-source-region:kind-span-scoping-and-direct-leaves:end -->
