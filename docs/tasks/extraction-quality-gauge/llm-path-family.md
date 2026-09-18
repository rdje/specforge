# EXTRACTION-QUALITY-GAUGE — llm path family

- Part ID: `llm-path-family`
- State: `active`

<!-- extraction-quality-gauge-task-source-region:llm-path-leaves:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.3j` · Status: `done` (`2026-09-17`, NO; opened `2026-09-12` by
  `INVARIANT-SHAPE-ADMISSION.5`) · Goal: **the LLM-primary constraint path applies none of the
  positional spurious-subject gates this family built.** `.2.50a`, `.3e`, `.3g`, `.3h` and now
  `INVARIANT-SHAPE-ADMISSION.5` are all wired as `subject_signals.retain(…)` in the two DETERMINISTIC
  extractors only. `crates/specforge/src/ir/constraint_extract_llm.rs` has its own gates —
  `ground_constraint` / `ground_constraint_typed`, which check catalog membership, drop invented
  subjects and drop condition-only subjects — but nothing that reads WHERE in the sentence the subject
  sits. So a model proposal is grounded on "is this a declared signal?" and never on "is this the
  thing the sentence constrains?".
  **Measured instance:** AXI-H `llm_sigcon_0025`/`0027` attribute `WTAGUPDATE must be deasserted` to
  `WTAG` — the scan lifted a shorter declared name out of a longer identifier — and
  `INVARIANT-SHAPE-ADMISSION.5`'s narrowing cannot reach them because they are not produced by the path
  it gates. The census (`python3 scripts/measure_table_row_foreign_subject.py`) names them.
  **Scope this before implementing.** The question is not "call the five retains from the LLM path
  too": it is whether a grounded model proposal SHOULD be judged by a gate designed for a pattern
  path's full-text scan, since the model is not scanning — it names a subject deliberately, and a gate
  built to catch scanning artefacts may refuse a correct proposal the deterministic path could never
  have made. Measure how many current `llm_sigcon_*` records each of the five would refuse before
  wiring any of them.
  Prerequisite: none. Verification: the per-gate refusal count over the persisted `llm_sigcon_*`
  population, adjudicated individually; observed RED for whichever gates are wired; the chain rebuilt
  for every document whose artifacts move.
  **Done `2026-09-17` — measured, adjudicated, and the answer is NO: do not wire any of the five.**
  The leaf asked for the refusal count before wiring; the count is small and mostly WRONG.
  **Census** (`python3 scripts/measure_llm_subject_gate_refusals.py`, read-only, over the **149**
  `llm_sigcon_*` records in **7** documents):

| gate | predicate | would refuse |
| --- | --- | ---: |
| `CORPUS-COVERAGE.2.50a` | `is_post_passive_binding_only_subject` | **7** (4.7%) |
| `EXTRACTION-QUALITY-GAUGE.3e` | `is_descriptive_field_cell_spurious_subject` | 0 |
| `EXTRACTION-QUALITY-GAUGE.3g` | `is_dotted_cross_reference_subject` | 0 |
| `EXTRACTION-QUALITY-GAUGE.3h` | `is_value_position_subject` | 0 |
| `INVARIANT-SHAPE-ADMISSION.5` | `obligation_head_is_a_foreign_identifier` | 2 of `.2.50a`'s 7 |

  Three of the five have **no population at all** on this path, so wiring them is unjustifiable in
  either direction: nothing to gain and an unmeasured rule to maintain.
  **All seven `.2.50a` refusals adjudicated individually, and FOUR ARE WRONG.** The gate would drop
  correct records: AXI `llm_sigcon_0025` `WTAG must_be_value zero` against the row's own
  `WTAG must be zero`; `llm_sigcon_0027` `WTAG must_be_value VALID` against `WTAG bits must be valid…`;
  ATB `llm_sigcon_0004` `AFVALID must_be_low` and `llm_sigcon_0006` `ATVALID must_be_low`, each against
  `… and <SUBJECT> must be driven LOW` in the very same sentence. Three refusals are right, and for a
  reason the gate is not aimed at: APB `llm_sigcon_0004`/`0005` read the DESCRIPTIVE
  `where PENABLE is asserted` / `PREADY is asserted by the Completer` as `must_be_asserted`, and LTI
  `llm_sigcon_0034` has the bare common noun `signal` as its subject. **Precision 3/7 = 43%.**
  **THE LEAF'S OWN MEASURED INSTANCE WAS WRONG.** It recorded AXI `llm_sigcon_0025`/`0027` as attributing
  `WTAGUPDATE must be deasserted` to `WTAG`. Their kinds are `must_be_value zero` / `must_be_value VALID`,
  matching two real `WTAG` obligations in the same row: the extractor was right, the GATE flagged them, and
  the premise was formed from the subject spelling without reading the record's `constraint_kind`.
  **Root cause = a SPAN MISMATCH.** `is_post_passive_binding_only_subject` narrows to
  `constraint_bearing_sentence`, the FIRST modal clause, while a record can be minted from any obligation
  in the statement — the `.3k.3` defect one level out. The deterministic remedy
  (`is_post_passive_binding_only_subject_in`) is unusable here because the proposal carries no clause;
  `.3j.1` scoped it as a `RawConstraint` change, not an IR schema change. Full mechanism:
  **[[an-llm-constraint-record-cannot-be-judged-by-a-positional-gate]]**.
  Verification: `2026-09-17` — census over 149 records / 7 documents; all 7 refusals adjudicated by
  reading the record's `constraint_kind` against its `source_text`; 0-population confirmed for three
  gates. No production code changed, so no oracle moved: this leaf's whole deliverable is the decision.
  **CORRECTION `2026-09-17`, on the director's challenge to re-verify. The DECISION stands; two published
  numbers gain a caveat they should have carried.** (a) **The population is not current.** All seven
  `llm_sigcon_*` documents sit outside the refreshed cohort of `doctrine/corpus_frontier/census.json` —
  five are outside the cohort rule, `opencapi_3_0`/`3_1` are listed `remaining` — so 149/7/0/0/0 describes
  the PERSISTED corpus, not necessarily what the current binary emits. The decision rests on the
  MECHANISM (first-clause narrowing; `RawConstraint` carries no clause), which is a property of the code
  and is unaffected. (b) **The instrument shipped without a self-test**, so three gates reading 0 was
  indistinguishable from a mirror that never fires — the one thing that would have made the census
  worthless. `--self-test` now proves each gate fires AND declines on its own doc-comment example,
  **11/11**, and the census is unchanged, so the zero readings are real populations.
  **A third check was attempted and is INVALID; recorded so nobody repeats it.** Running the mirror over
  deterministic records that survived the real gates reported 21 of 195 "collisions", 15 in refreshed docs —
  meaningless: those paths gate `statement.text`, while a persisted record's `source_text` is the narrower
  minting clause (`.3k.4`). For `llm_sigcon_*` the same feed IS correct (universe built from `source_text`).
  Commit: `EXTRACTION-QUALITY-GAUGE.3j — NO: the positional gates would refuse 7 of 149 and be wrong about 4`

- ID: `EXTRACTION-QUALITY-GAUGE.3j.1` · Status: `pending` (opened `2026-09-17` by `.3j`) · Goal: **an
  `llm_sigcon_*` record does not carry the obligation clause it was minted from, so no positional
  subject gate can judge it.** `.3j` measured what happens if one tries: `is_post_passive_binding_only_
  subject` refuses 7 of 149 records and **4 of the 7 are correct records**, every one because the gate
  narrows to `constraint_bearing_sentence` (the FIRST modal clause) while the record came from a later
  one. The deterministic paths were given `is_post_passive_binding_only_subject_in` for precisely this
  (`.3k.3`); the LLM path has nothing to pass it, because the record cites only
  `supporting_statement_ids` — the whole statement.
  **The leaf is the span, not the gate.** Give the LLM constraint record the clause it was minted from,
  the way the deterministic record already effectively has one, and the five gates become answerable
  questions rather than unanswerable ones. Then, and only then, re-run `.3j`'s census against the
  three-argument form and adjudicate again.
  **Do not shortcut it by re-deriving the clause at gate time** from the record's kind/value: that is a
  second reader of the same statement and it will disagree with the first exactly where it matters.
  Prerequisite: none. Blocks: wiring any of the five gates into `constraint_extract_llm.rs`.
  **SCOPED `2026-09-17`: the remedy is a PROPOSAL-shape change, not an IR schema change.** `source_text`
  **is** the span the model was shown (`commands/extract_constraints_llm.rs:60-70` builds the universe from
  the DISTINCT `source_text` of existing constraints; `constraint_extract_llm.rs:379`/`:391` write it back).
  What is missing is WHICH obligation inside it was read, because `RawConstraint` is
  `{subject, kind, condition, value}`. So add the clause to `RawConstraint` and the prompt — no EvidenceIR
  field, no re-seal, no rebuild — and make it CHECKABLE: the clause must be a literal substring of
  `source_text`, refused otherwise, the shape `LLM-PRIMARY-PROMOTION.3a` already uses.
  Split: `.3j.1.a` carries the clause and refuses a non-substring; `.3j.1.b` re-runs `.3j`'s census against
  `is_post_passive_binding_only_subject_in` and re-adjudicates. The wiring decision is `.3j.1.b`'s and must
  not be folded into `.3j.1.a` — a clause carried but never re-measured proves nothing.
  Verification: split; see `.3j.1.a`/`.3j.1.b`
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3j.1.a` · Status: `pending` (opened `2026-09-17` by `.3j.1`) · Goal: **make
  the model name the obligation it read, and refuse it when it did not.** Add the clause to `RawConstraint`
  and the prompt; at grounding require a literal substring of the record's `source_text`, else drop the
  proposal. Consumed at grounding time, not persisted, unless `.3j.1.b` shows otherwise.
  Acceptance: a proposal whose clause is absent from its span is REFUSED, observed RED; per-document
  constraint counts unmoved where every proposal cites a real substring. Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3j.1.b` · Status: `pending` (opened `2026-09-17` by `.3j.1`) · Goal:
  **re-run `.3j`'s census with the clause and re-adjudicate.** `.3j` measured 7 refusals of 149 with **4
  wrong**, each because the gate read the FIRST modal clause. Wire only if precision beats 3/7, and
  re-measure on a REFRESHED population (see `.3j`'s correction). Prerequisite: `.3j.1.a`.
  Verification: pending
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3j.3` · Status: `pending` (opened `2026-09-17` by `.3j.1`'s scoping) ·
  Goal: **the LLM pass has a hard recall ceiling nothing states.** Its universe is the DISTINCT
  `source_text` of constraints the DETERMINISTIC paths already emitted, so it can never see a span those
  paths missed. Measure it (obligation-bearing statements vs distinct spans visited) before widening
  anything. Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2` · Status: `pending` (opened `2026-09-17` by `.3j`) · Goal: **one
  `llm_sigcon_*` subject is a bare common noun, and the catalog grounding did not refuse it.** LTI
  `llm_sigcon_0034` carries `subject_signal: "signal"` with `must_be_value 0`, from *"the following
  signals must be 0:"*. `signal` appears in **none** of that document's seven signal-bearing EvidenceIR
  surfaces (`table_signal_declaration_provenance` 70 names, `signal_presence_records` 50,
  `signal_polarities`, `signal_semantic_hints`, `actor_signal_relations`, `signal_alias_map`,
  `signal_semantic_conflicts`) — checked directly.
  **Size it against the real grounding function, not against a proxy, and that caveat is the leaf.** A
  proxy census over those same surfaces flagged 36 of 149 subjects as undeclared, and reducing
  bit-slice/qualified spellings to a base name still left 20 — but the survivors include APB's `PSEL`,
  which is unquestionably a declared APB signal. **So the proxy is wrong and its numbers are not
  findings**; they are recorded here only so the next session does not re-derive the same dead end. The
  measurement has to run `ground_constraint`/`ground_constraint_typed`'s own membership test.
  Prerequisite: none.
  Verification: pending
  Commit: pending

<!-- extraction-quality-gauge-task-source-region:llm-path-leaves:end -->


## Post-migration work

Declared after the `2026-09-17` containment migration. These nodes live outside the marked legacy region,
which is what the active part is for; the payload above is immutable.

- ID: `EXTRACTION-QUALITY-GAUGE.3j.1.a`
  Status: `done` (`2026-09-17`, CODE)
  Goal: **make the model name the obligation it read, and refuse it when it did not.**
  Acceptance: a proposal whose clause is absent from its span is REFUSED, observed RED; per-document
  constraint counts unmoved where every proposal cites a real substring.
  **Shipped exactly per `.3j.1`'s scoping — a PROPOSAL-shape change, not an IR schema change.**
  `RawConstraint` gains `clause: Option<String>` (`#[serde(default)]`), `extraction_prompt` asks for *the
  exact words of the obligation you read, copied verbatim from the sentence*, and
  `ground_constraint_typed` drops the whole proposal when the clause is present and is **not** a literal
  quote of the span. No EvidenceIR field, no re-seal, no rebuild.
  **The check is a quote test, not a similarity test.** Pure `clause_is_quoted_from` normalizes whitespace
  on BOTH sides — a model that re-wraps a long clause has still quoted it — and relaxes nothing else. It
  never re-derives the clause from the record's own kind or value: `.3j.1` is explicit that this would be a
  second reader of the same statement, disagreeing with the first exactly where it matters.
  **Silence is not a refusal, and that is a decision rather than an omission.** `.3j.1`'s split says this
  leaf "carries the clause and refuses a non-substring". A missing clause therefore passes: whether it
  should also refuse is `.3j.1.b`'s call, after it re-measures. Refusing silence here would have made the
  behaviour depend on whether the model happens to answer a brand-new field — a change to extraction
  volume disguised as a gate.
  **Corpus effect is ZERO BY CONSTRUCTION, not by measurement, and the distinction matters.** The field
  defaults to `None`, the refusal fires only on a present non-quoting clause, and no production site sets
  one — the only `clause: Some(...)` in the tree are the two in this leaf's own controls. Nothing
  re-grounds a persisted record, and `replay-constraints` composes the three DETERMINISTIC producers, so a
  replay would prove nothing about this path either way. The live half — what a real model answers, and
  whether the five gates then adjudicate better than 3/7 — is `.3j.1.b`'s, and it needs a model that is
  not currently up.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3j.1.a`

- [x] **REPRODUCE / MEASURE** — `.3j`'s census: `is_post_passive_binding_only_subject` refuses **7 of the
  149** persisted `llm_sigcon_*` records across 7 documents, and **4 of the 7 refusals are WRONG**, each
  because the gate narrows to `constraint_bearing_sentence` — the FIRST modal clause — while the record was
  minted from a later one. Precision 3/7 = 43%.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/constraint_extract_llm.rs:19`:
  `RawConstraint` is `{subject, kind, condition, value}`, and the record it grounds into cites only
  `supporting_statement_ids` — the whole statement. The deterministic paths were given
  `is_post_passive_binding_only_subject_in` for exactly this (`.3k.3`); the LLM path has nothing to pass
  it. The span is not missing — `source_text` **is** the span the model was shown
  (`commands/extract_constraints_llm.rs:60-70` builds the universe from DISTINCT `source_text`) — what is
  missing is WHICH obligation inside it was read.
- [x] **ADDRESSED (verified)** — the model is asked, and the answer is checked. RED observed by A/B: with
  only the four-line guard removed and everything else byte-identical,
  `a_clause_the_model_composed_refuses_the_whole_proposal` FAILS; restored, it passes. The same test pins
  the other direction in one function — the identical proposal carrying the clause the document DOES state
  is kept — so the refusal is demonstrably the clause's doing and not the subject's.
- [x] **NO REGRESSION** — `cargo test -p specforge-core --lib` **1,546 passed / 5 ignored (1,547 -> 1,551
  declared)**, `cargo test -p specforge --lib` 473 passed, `cargo fmt --all -- --check` clean,
  `cargo clippy --workspace --all-targets` clean. `PRODUCTION-GENERICITY` re-derived rather than edited:
  `analyzed_functions` 2,461 -> 2,462, `decision_sites` 13,200 -> 13,203, `helper_edges` 15,441; **every
  boundary count unmoved**, which is the statement that a predicate over a proposal and its own span is not
  new authority over any artifact.
- [x] **GENERICITY (ADR 0006)** — a substring test over a proposal and the span it was shown. No document,
  protocol, vendor or signal name appears in the rule; the controls use AXI's real `WTAG`/`WTAGUPDATE` pair
  only because it is `.3j`'s own measured instance, and the predicate never reads either name.
- [x] **LOCKSTEP** — `docs/book/src/commands/quality-and-learning.md` stated the proposal shape as
  `(subject, kind, condition, value)` and is now current, with the measurement that motivates the field and
  the rule that a proposal naming no clause is not refused. No production rule was deleted, so no book text
  describes behaviour that has gone.
  Verification: the A/B above, plus the four new unit tests; census re-derivation is `.3j.1.b`'s
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.1.a — carry the obligation clause, and refuse one the span never stated`
  Prerequisite: none. Blocks: `.3j.1.b`, which must re-measure before any gate is wired

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2`
  Status: `done` (`2026-09-18`, MEASUREMENT + CODE)
  Goal: **one `llm_sigcon_*` subject is a bare common noun, and the catalog grounding did not refuse it.**
  **Sized against the real membership test — and the sizing retires the premise.** The production
  authority is not the seven signal-bearing EvidenceIR surfaces: `promote_constraints` types a subject
  through `resolve_unique_document_identifier` against `declared_signal_catalog(ir)` first and the
  `message_field_records` names second (`commands/extract_constraints_llm.rs:110-128`), and
  `declared_signal_catalog` is `collect_known_signal_names(extracted_statements)` ∪
  `table_signal_declaration_provenance` (`ir/entity_typing.rs:50-73`). Re-running **that** closure,
  rebuilt verbatim, over all **149** persisted records in the 7 documents:
  **111 exact-signal / 2 case-folded-signal / 0 field / 36 ungrounded.**
  So the answer to the leaf's own question is NO: `signal` **is** refused by the catalog grounding, and
  so are 35 other persisted subjects. Per document (records / ungrounded): AXI 65/11, LTI 35/9, APB 19/1,
  ATB 8/1, AXI-Stream 8/0, OpenCAPI-3.0 7/7, OpenCAPI-3.1 7/7.
  **The 36 split into three classes, and only one of them is what the leaf was opened for.**
  **16 CARRY a declared name** and are refused for their spelling — `RRESP[3]`, `ARCACHE[3:0]`,
  `WSTRB bits`, `WTAG bits`, `Subordinate LAPM`, `Manager LRMPAM .PARTID[11:9]` — a bit slice or an
  actor/field qualifier the exact/case-fold resolver cannot see past. **2 TRUNCATE one**: `PSEL`, whose
  catalog declares `PSELX` and `PSELXCHK`, and `ATB`, whose catalog declares `ATBYTES`. **18 have no
  declared relative at all**, but 14 of those are the two OpenCAPI documents' `dPart`/`dLength`/`AFUTag`
  — message fields on artifacts that predate `message_field_records` (empty on every schema-2 artifact),
  so they are the `.FIELD` story, not this one. Of the remaining four, `Write strobes` is AXI's prose
  name for the declared `WSTRB` and `Subordinate LASECSID[1]` names a token absent from LTI's persisted
  catalog. **That leaves exactly one true bare common noun in 149 records: LTI `llm_sigcon_0034`,
  `signal`.** The leaf's premise about its size was right; its premise about its survival was not.
  **CORRECTION — the proxy was not wrong, the adjudication that condemned it was.** `.3j.2` recorded
  that the proxy census "flagged 36 of 149" and "still left 20" after reducing bit-slice/qualified
  spellings, then discarded both numbers because the survivors "include APB's `PSEL`, which is
  unquestionably declared". The real membership test returns **36**, and 36 − 16 qualified/sliced
  spellings = **20** — the proxy's two numbers reproduce exactly. It also refuses `PSEL`, and the reason
  is now visible rather than assumed: APB's declared spelling is `PSELx`, a **parameterised declaration
  template**, and the catalog holds `PSELX`/`PSELXCHK`, never the bare `PSEL`. "Unquestionably declared"
  was a human reading of the protocol, not a check of the document's catalog. The recorded numbers are
  restored as findings; the dismissal is retracted.
  **ROOT CAUSE — `signal` was admitted by a grounding rule that no longer exists.** All seven artifacts
  were written in one batch at `2026-08-12 17:44`, when HEAD was `4b8895d6` (`15:03`). There,
  `promote_constraints` built `type_subject` as
  `classify_entity(gather_entity_evidence(s, &ir, …), |_| EntityType::Signal)` — an LLM judgment with the
  model stubbed to answer `Signal`, so every token the document did not positively contradict became a
  signal, and **no catalog was consulted anywhere on that path**: `propose_constraints_llm(sentence,
  provider, model)` took no carrier list either. `declared_signal_catalog` did not exist until
  `9c38b569` (`19:02`), 78 minutes after the corpus was minted, and the schema bump that makes these
  artifacts inspection-only landed at `1aa7f95d` (`2026-08-13 01:32`), so nothing has rewritten them
  since. The defect this leaf names is therefore **already closed in the producer** — closed by the
  `.1`-grounding rewrite, not by this leaf.
  **CONSEQUENCE — the 149-record population measures a superseded producer.** Every census over it,
  `.3j`'s 7-of-149 refusals included, is a measurement of the pre-catalog grounding rule. `.3j.1.b`
  already requires re-measuring on a REFRESHED population; this leaf supplies the size of the staleness:
  **24% of the persisted subjects (36/149) would not survive today's grounding at all.** The one
  observation that needs a fresh build to adjudicate — `LASECSID` absent from LTI's *persisted* catalog —
  is routed there rather than asserted here, because this census rebuilds the persisted catalog, not a
  current one. The two classes that are properties of the **resolver** rather than of catalog
  completeness reproduce on any catalog that declares the base name, and are opened as `.3j.2.a` and
  `.3j.2.b`.
  Prerequisite: none. Blocks: nothing; `.3j.1.b` consumes the staleness size.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3j.2`

- [x] **REPRODUCE / MEASURE** — `cargo test -p specforge-core --lib llm_constraint_subject_grounding_census
  -- --ignored --nocapture`: **149 records / 111 exact-signal / 2 case-folded-signal / 0 field / 36
  ungrounded**, and **36 = 16 carries a declared name / 2 truncates one / 18 has no declared relative**.
  Read-only over persisted artifacts; no provider, no rebuild, no mutation.
- [x] **ROOT CAUSE (WHY + WHERE)** — `commands/extract_constraints_llm.rs:110-128` is the production
  membership test today; at `4b8895d6:crates/specforge/src/commands/extract_constraints_llm.rs` (the
  mint-time HEAD) the same lines read `classify_entity(gather_entity_evidence(…), |_| EntityType::Signal)`
  and consulted no catalog. `git log -L '/^pub fn declared_signal_catalog/,/^}/:crates/specforge/src/ir/
  entity_typing.rs'` dates the catalog to `9c38b569`, 78 minutes after the artifacts' mtime.
- [x] **ADDRESSED (verified)** — the current closure refuses the token, pinned in both directions by
  `a_subject_the_catalog_does_not_declare_is_refused`. **RED observed by A/B**: replacing only the
  resolver body with the mint-time stub (`|_| EntityType::Signal`), everything else byte-identical, fails
  the test at `constraint_extract_llm.rs:1614` with *"a common noun the catalog does not declare must not
  ground"*; restored, it passes. The GREEN half lives in the same function — the identical proposal on a
  DECLARED subject still grounds — so the refusal is the catalog's doing, not the proposal shape's.
- [x] **NO REGRESSION** — `cargo test --workspace --lib --exclude specforge-production-graph`
  **2,188 passed / 10 ignored / 0 failed** (specforge 473, specforge-conformance 168, specforge-core
  1,547 — up from 1,546 by this leaf's one control, with its one `--ignored` harness added to the 5).
  `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets` byte-identical to the
  pre-slice baseline (the one pre-existing `too_many_arguments` warning, nothing new — verified by
  stashing the slice and re-running).
- [x] **GENERICITY (ADR 0006)** — the slice adds no production rule. The control's catalog is a single
  opaque token the rule never reads, and the harness reads every document's own catalog out of its own
  artifact; the protocol names in the findings above are measured instances, not inputs to any predicate.
- [x] **LOCKSTEP** — no user-visible behaviour changed and no production rule was deleted, so no book text
  describes behaviour that has gone; the book's statement of the proposal shape stays current. A fact card
  carries the durable causal finding (the persisted LLM-constraint corpus predates catalog grounding).
  Verification: the census above, the A/B, and the workspace oracle
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.2 — the catalog already refuses it; the corpus predates the catalog`

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.a`
  Status: `done` (`2026-09-18`, ADJUDICATION + MEASUREMENT)
  Goal: **a subject that CARRIES a declared name is dropped whole for its spelling — decide whether the
  resolver should see past it.** Opened by `.3j.2` on the observation that 16 of the 36 ungrounded
  subjects contain a declared signal plus a bit slice (`RRESP[3]`, `ARCACHE[3:0]`) or an actor/field
  qualifier (`WSTRB bits`, `Subordinate LAPM`, `Manager LRMPAM .PARTID[11:9]`), that
  `resolve_unique_document_identifier` is exact-then-case-fold so none of them resolves and the whole
  obligation is lost, and that this is a property of the **resolver** rather than of the persisted
  catalog — so unlike the rest of `.3j.2`'s census it needs no refreshed population. Adjudicated before
  wiring, per `.3j`'s standing rule: a widened resolver also admits a subject whose slice contradicts the
  obligation, and that cost had to be measured rather than assumed.
  **ANSWER: NO to any general widening; YES to exactly one narrow rule.** The 16 were first classified
  MECHANICALLY, by the census harness rather than by hand, against each carried name's **stated width**
  read out of the same synthesised `Signal <name> is width <n>.` sentence the catalog is built from:
  **6 qualifier-only / 3 full-width alias / 1 proper sub-slice / 6 slice whose signal states no width.**
  Then each record was read against its source, which is the half no classifier can do.
  **FULL-WIDTH-ALIAS — 3 of 3 correct, and correct by denotation rather than by tally.** `AWCMO[1:0]`
  (stated width 2), `ARLEN[7:0]` (8), `ARCACHE[3:0]` (4): each spans the whole signal from bit 0, so
  `X[w-1:0]` *is* `X` and resolving loses nothing. Their sources state plain obligations — *"this signal
  must be 0b00"*, *"ARLEN[7:0] must be 0x00"*, *"ARCACHE[3:0] must be 0b0010"*. This is the one rule the
  evidence supports, and its justification does not rest on the population: it is an identity, not a
  statistic. What the population licenses is only that the class is real and non-empty.
  **QUALIFIER-ONLY — 1 of 6. NO, and its failure mode is the worst available.** `WTAG bits` → `WTAG` is
  correct. The other five are not: `snoop response` carries the declared token `SNOOP`, but the sentence's
  real subject is the snoop **response** payload, so resolving would **invent a subject the sentence never
  names**; `WSTRB bits` twice obligates the *Subordinate* (*"The Subordinate must only write those bytes
  indicated by the relevant WSTRB bits"*, *"A write with no strobes asserted must be supported"*), not
  WSTRB; `Subordinate LAPM` and `Subordinate LAPASUNKNOWN` do resolve cleanly but their records drop the
  scenario that scopes them, which is `.3j.2.c`'s defect and not the resolver's.
  **PROPER-SUB-SLICE — must stay refused, and this is the decisive case.** `AWSNOOP[3]` with stated width
  4 is **one bit of four**, from *"AWSNOOP[3] must be tied LOW"*. Resolving it to `AWSNOOP` yields
  *"AWSNOOP must be LOW"* — a **strictly stronger obligation the document never stated**, fabricated
  silently and trusted by every gate downstream. A widened resolver with no width test does exactly this.
  **SLICE-WIDTH-UNKNOWN — must stay refused, because the question cannot be answered.** `RRESP[3]`,
  `LAPAS[2:1]`, `LRPAS[2:1]`, and the three `LRMPAM .MPAM_SP[…]`/`.PARTID[11:9]` sub-field spellings carry
  a declared name whose document states no width — the catalog also admits names through
  `table_signal_declaration_provenance`, which carries none. Across the 7 documents only **209 of 353**
  declaration-grammar names (59.2%) state a width at all, so this third answer is structural, not rare.
  Four of these six are wrong records independently: `RRESP[3]`'s span is an **encoding table row**
  (*"| RRESP[3] | Interconnect | IsShared | HIGH |"*) describing what the bit MEANS when HIGH, not an
  obligation that it be HIGH; `LRPAS[2:1]` and `LRMPAM .MPAM_SP[1]` are minted `must_not_change` from
  *"output is unconnected"*; `LRMPAM .MPAM_SP[0]` is minted `must_hold_data` from a **connectivity**
  statement.
  **The arithmetic that decides it.** General widening would make 4 of 16 records correct — **25%**, below
  the 3/7 = 43% that `.3j` already answered NO to — while silently strengthening one obligation and
  inventing one subject. Restricted to a full-width slice it is 3 of 3 with no known false admission.
  Prerequisite: none. Blocks: `.3j.2.a.i` wires the one rule; `.3j.2.b` must not absorb it.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3j.2.a`

- [x] **REPRODUCE / MEASURE** — `cargo test -p specforge-core --lib llm_constraint_subject_grounding_census
  -- --ignored --nocapture` now prints **`CARRIES-DECLARED 16 = 6 qualifier-only / 3 full-width alias /
  1 proper sub-slice / 6 slice whose signal states no width`**, each subject named with the declared token
  it carries. The totals above it are unmoved (149 / 111 / 2 / 0 / 36).
- [x] **ROOT CAUSE (WHY + WHERE)** — `ir/entity_typing.rs:26-47`: `resolve_unique_document_identifier` is
  exact-then-case-fold over opaque names, so `ARLEN[7:0]`, `WTAG bits` and `Subordinate LAPM` are simply
  different strings from `ARLEN`, `WTAG`, `LAPM`. `commands/extract_constraints_llm.rs:110-128` types
  through it and `ir/constraint_extract_llm.rs:358-362` drops the whole proposal on `EntityType::Unknown`.
  The width that separates a lossless slice from a lossy one is present for 209 of 353 names, in the
  `Signal <name> is width <n>.` sentences the catalog is derived from.
- [x] **ADDRESSED (verified)** — this leaf is an adjudication, and what it delivers is the decision plus a
  **re-derivable** classification: the sub-class of every one of the 16 now falls out of the harness rather
  than out of a reading. The classifier is pinned in both directions by
  `a_full_width_slice_is_an_alias_and_a_partial_slice_is_not`, whose RED half is the case the decision
  turns on — `AWSNOOP[3]` against stated width 4 must NOT classify as an alias.
- [x] **NO REGRESSION** — `cargo test --workspace --lib --exclude specforge-production-graph`
  **2,189 passed / 10 ignored / 0 failed** (specforge-core 1,548, up from 1,547 by this leaf's one
  control). `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets` unchanged from the
  pre-slice baseline. No production rule was added or changed: both new functions are `#[cfg(test)]`, and
  `PRODUCTION-GENERICITY` re-derives with every count unmoved.
- [x] **GENERICITY (ADR 0006)** — the rule reads a bracket span and a stated width. No document, vendor,
  protocol or signal name enters any predicate; the names above are the measured instances the decision was
  made on, and the classifier never reads one.
- [x] **LOCKSTEP** — no user-visible behaviour changed and no production rule was deleted, so no book text
  describes behaviour that has gone. The durable finding — a slice is only an alias against a stated width,
  and 41% of declared names state none — is carried by the `.3j.2` fact card's neighbourhood and by this
  leaf; wiring is `.3j.2.a.i`'s, and the book changes there if the contract does.
  Verification: the harness sub-classification, the RED/GREEN classifier control, and the workspace oracle
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.2.a — a slice is an alias only against a stated width`

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.c` · Status: `pending` (opened `2026-09-18` by `.3j.2.a`) · Goal:
  **an obligation minted from a multi-cell table row loses the row key that scopes it, and the key is
  inside the span.** Six of LTI's nine ungrounded records — `llm_sigcon_0013`-`0018` — come from ONE cell
  of ONE compatibility-matrix row, and every one is minted as an unconditional obligation although the
  row's first cell reads `LTI_MMU = True LTI_GPC = False`. That key is present in the record's own
  `source_text`, so this is not a source-assembly gap and not `.3j.1`'s question (WHICH obligation was
  read): it is WHAT scopes the obligation that was read. `Subordinate LAPM is tied LOW` is true only in
  that configuration, and the record asserts it always. Measure the population of row-keyed obligations
  before proposing a remedy, and do it on a refreshed corpus — unlike `.3j.2.a`'s classes this one is a
  property of the proposal, not of the resolver. Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.a.i`
  Status: `done` (`2026-09-18`, CODE)
  Goal: **wire the one rule `.3j.2.a` adjudicated: a full-width slice resolves to its signal, and
  nothing else does.** Opened by `.3j.2.a` with its placement already decided — in the grounding closure
  where `.3j.1.a` shipped its clause check, never inside `resolve_unique_document_identifier`, whose
  contract is opaque identity and which other surfaces depend on.
  **Shipped as three functions and one call-site rewrite.**
  `evidence::stated_signal_widths` reads the width each signal's own declaration states, from the
  canonical `Signal <name> is width <n>.` grammar under **exactly** the admission rule
  `collect_known_signal_names` uses, so the catalog and the widths cannot disagree about what a
  declaration is. `entity_typing::resolve_full_width_slice_alias` resolves `X[w-1:0]` against that width,
  and its private `full_width_candidate` splits the spelling. `promote_constraints` rewrites
  `raw.subject` **before** typing, and only when the catalog does not already declare the subject as
  spelled.
  **What it refuses is the whole design, and both guards are load-bearing.** A **proper sub-slice** is
  excluded by `low == 0`; a slice whose signal states **no width** is excluded by the comparison itself;
  a bare **qualifier** never enters, because it produces no bracket span. The two guards do **not**
  subsume one another, and the A/B proved it rather than assuming it: with only the width comparison
  removed the composition control still passes, because `XQRSNP[3]` is caught by `low == 0`; with only
  `low == 0` removed **both** controls fail, because a top-bit slice `X[w-1]` satisfies `high + 1 == w`
  on its own. A width test alone would therefore have admitted exactly the case the adjudication was
  about.
  **A disagreement yields no width rather than a choice.** A name the document declares with two
  different widths resolves to none. Measured `2026-09-18`: 209 of 353 declaration-grammar names state a
  width and **none states two**, so this guard has no population — it is there because the alternative is
  choosing arbitrarily, not because a document has been seen to need it.
  **Corpus effect is zero by construction, and the reach is measured anyway.** Nothing re-grounds a
  persisted record, so no artifact moves. What the shipped function would do is re-derived through the
  production function itself rather than through the harness's own classifier, and the two instruments
  agree: `SHIPPED full-width alias (.3j.2.a.i) resolves 3 of the 16 carried-name subjects`, the same
  three the independent width classifier calls `FULL-WIDTH-ALIAS`.
  Prerequisite: `.3j.2.a`. Blocks: nothing.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3j.2.a.i`

- [x] **REPRODUCE / MEASURE** — `.3j.2.a`'s census: of the 36 ungrounded subjects, **16 carry a declared
  name** and `3` of those are a full-width slice. After the wiring, the same harness re-derives the reach
  through the SHIPPED function: **`SHIPPED full-width alias (.3j.2.a.i) resolves 3 of the 16`**.
- [x] **ROOT CAUSE (WHY + WHERE)** — `ir/entity_typing.rs:26-47`:
  `resolve_unique_document_identifier` is exact-then-case-fold over opaque names, so `ARLEN[7:0]` and
  `ARLEN` are different strings; `commands/extract_constraints_llm.rs` types through it and
  `ir/constraint_extract_llm.rs` drops the whole proposal on `EntityType::Unknown`. The width that
  separates a lossless slice from a lossy one was present but unread — `SignalDeclarationPredicate::Width`
  carried no numeral.
- [x] **ADDRESSED (verified)** — **two A/Bs, each against a different guard, both observed RED.**
  (1) Width comparison removed, everything else byte-identical:
  `a_full_width_slice_resolves_and_nothing_else_does` FAILS at `entity_typing.rs:575` — the same spelling
  must refuse once its width is unknown. (2) `low == 0` removed: that control **and**
  `the_production_composition_records_the_signal_a_full_width_slice_names` both FAIL —
  *"a proper sub-slice must not reach the record surface at all"*. Restored, all pass. The composition
  control is the one that proves the RECORD carries `XQRLEN`, not the slice spelling, because a rule that
  resolves correctly and a record that carries the resolved name are two claims.
  `stated_signal_widths_reads_the_declared_numeral_and_refuses_a_disagreement` pins the accessor,
  including the two-width refusal and the non-numeral width.
- [x] **NO REGRESSION** — `cargo test --workspace --lib --exclude specforge-production-graph`
  **2,192 passed / 10 ignored / 0 failed** (specforge-core 1,551, up from 1,548 by this leaf's three
  controls). `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets` unchanged from
  the pre-slice baseline. `PRODUCTION-GENERICITY` **re-derived, not edited**: `aggregate_change` rolls its
  previous total into the baseline and attributes `analyzed_functions +3`, `decision_sites +17`,
  `helper_edges +17`, `semantic_macros +0` to this leaf. **Every boundary count is unmoved** — reading a
  width a document states and comparing a slice against it adds no source, no rule root and no
  declassifier, so it is not new authority over any artifact.
- [x] **GENERICITY (ADR 0006)** — the rule reads a bracket span and a stated width. No document, vendor,
  protocol or signal name enters any predicate, and the controls use opaque `XQR*` tokens the rule never
  reads; the real spellings appear only in prose as `.3j.2.a`'s measured instances.
- [x] **LOCKSTEP** — `docs/book/src/commands/quality-and-learning.md` stated that a subject the catalog
  does not declare is dropped, full stop, which is no longer the whole truth. It now states the one
  spelling that survives, both guards that keep it narrow, the measured 4-of-16 that ruled out widening,
  and why a top-bit slice is excluded although it satisfies the width comparison. No production rule was
  deleted, so no book text describes behaviour that has gone.
  Verification: the two A/Bs, the three new controls, the shipped-reach re-derivation, the workspace oracle
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.2.a.i — resolve the slice that is the whole signal, and only that one`

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.b`
  Status: `done` (`2026-09-18`, ADJUDICATION — the answer was already doctrine)
  Goal: **the catalog can hold a parameterised declaration template, and identity resolution has no
  notion of one.** Opened by `.3j.2` on APB's `PSEL`: the document declares `PSELx`, the catalog holds
  `PSELX`/`PSELXCHK`, and the bare family name cannot resolve. ATB's `ATB` → `ATBYTES` was noted as the
  same shape from the other side and told apart from it.
  **ANSWER: NO, and it was decided before this leaf was opened — `ADR 0037` §1 and §3.** Identity
  resolution has no notion of a declaration template **by decision**, not by omission: *"Its length, case,
  prefix, suffix, substring, resemblance to a conventional name … carry no semantic authority"* (§1), and
  *"Model proposals must resolve exactly to the current document's declared catalog … Case-folded
  resolution is permitted only when it yields exactly one current-document identity"* (§3). The
  Knowledge Map already carried the answer under the exact question this leaf asks —
  `[[inference-antecedent-state-loss]]`, *"does suffix spelling authorize a PSEL to PSELX alias"* — and
  this leaf found it there rather than re-deriving it. **`.3j.2`'s census refusing `PSEL` is ADR 0037
  working, not a defect.**
  **The measurement that would have been needed anyway, and it agrees.** Before the decision record was
  found, the one mechanically defensible rule was costed: resolve a proposal to the unique declared name
  it extends by exactly one character — which separates `PSEL` → `PSELX` from `ATB` → `ATBYTES` (four
  characters) rather than conflating them, as the leaf required. Its admission surface is the catalog
  minus one character: across five documents **372 one-character stems** exist that are not themselves
  declared, so the rule would admit 372 strings to recover **one** real name, and four of them
  (`ARCTLCHK` → `ARCTLCHK0..3`, `A` → `AC`/`AR`/`AW`) are ambiguous in AXI alone. Cost and doctrine agree.
  **The real defect is one layer down, and this leaf found it by reading the record rather than the
  spelling.** APB `llm_sigcon_0000` is `PSEL | must_be_asserted`, minted from *"The select signal, PSEL,
  is asserted, which means that PADDR, PWRITE and PWDATA must be valid."* — **the exact sentence
  `SPEC-TO-INTENT-ALIGNMENT.7a` built its remedy for.** ADR 0037's own answer to `PSEL` is not an alias:
  it is that a **same-clause appositive is a local declaration of opaque `PSEL`**, with `PSELX` retained
  as distinct and a negative control proving bare `PSEL` with only `PSELX` declared emits nothing. So the
  record is **CORRECT**, and it is refused only because that remedy is **deterministic-path-only**:
  `is_same_clause_signal_appositive` has exactly one call site,
  `parse_inference_antecedent_signal_constraint` (`ir/evidence.rs:10107`), and the LLM path types against
  the global catalog alone. The LLM path is therefore applying §3 against an **incomplete notion of
  "declared"** — one that omits a declaration form this repository has already ruled is a declaration.
  That is `.3j.2.b.i`, and it is a different defect from the one this leaf was opened for.
  Prerequisite: none. Blocks: nothing; `.3j.2.b.i` owns the successor.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3j.2.b`

- [x] **REPRODUCE / MEASURE** — APB's catalog holds `PSELX`/`PSELXCHK` and not `PSEL`, while the
  document's own prose uses **both** spellings (*"The select signal, PSEL , is asserted"* beside
  *"- Select signal, PSELx"* and the declaration row *"The Requester generates a PSELx signal for each
  Completer"*). The one-character-extension cost is **372 stems across five documents, 4 of them
  ambiguous**, against one real recovery.
- [x] **ROOT CAUSE (WHY + WHERE)** — `docs/decisions/0037-identifiers-are-opaque-and-one-way-grounded.md`
  §1 and §3; the recovery route is `is_same_clause_signal_appositive` at `ir/evidence.rs:10107`, whose
  single call site is `parse_inference_antecedent_signal_constraint`, reachable only from the
  deterministic constraint extractor. `grep -rn is_same_clause_signal_appositive crates/` returns that one
  use.
- [x] **ADDRESSED (verified)** — no code changed, and that **is** the verified outcome: the behaviour the
  leaf suspected of being a gap is the behaviour an accepted decision record requires, and the existing
  negative control `exact_witness_establishes_local_psel_without_aliasing_pselx`
  (`ir/evidence.rs:31806`) already pins it — it asserts `PSEL` is recovered, `PSELX` stays distinct, and
  the two are never aliased. Changing anything here would have broken a control that already passes.
- [x] **NO REGRESSION** — no production change; the workspace oracle is unmoved from `.3j.2.a.i`'s
  **2,192 passed / 10 ignored / 0 failed**.
- [x] **GENERICITY (ADR 0006)** — the leaf's outcome is to add no spelling rule at all, which is the
  strongest possible compliance. The 372-stem measurement is over opaque tokens and names none of them as
  authority.
- [x] **LOCKSTEP** — no user-visible behaviour changed, so the book is unmoved. The durable finding gets a
  fact card (`[[llm-path-lacks-the-local-appositive-declaration]]`), linked to the existing
  `[[inference-antecedent-state-loss]]` rather than duplicating it — that card already answers the
  template question and is what closed this leaf.
  Verification: the decision record, the existing negative control, and the one-character-extension census
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.2.b — the suffix question was already answered; the appositive one was not`

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.b.i` · Status: `pending` (opened `2026-09-18` by `.3j.2.b`) · Goal:
  **the LLM path applies ADR 0037 §3 against an incomplete notion of "declared".** A same-clause
  appositive is a local declaration of an opaque identifier — that is `SPEC-TO-INTENT-ALIGNMENT.7a`'s
  ruling, and `is_same_clause_signal_appositive` implements it at one call site inside the deterministic
  antecedent parser. The LLM path types against the global catalog alone, so it refuses APB
  `llm_sigcon_0000` (`PSEL | must_be_asserted`) although that is the canonical fact `.7a` exists to
  recover and the deterministic path does recover it. Decide whether the local declaration belongs in the
  catalog the LLM path types against, **scoped to the sentence it was read in** — a global widening would
  let one sentence's appositive validate a subject in every other sentence, which is exactly the identity
  minting ADR 0037 §3 forbids. Adjudicate before wiring, on a refreshed population: the persisted records
  predate catalog grounding entirely (`.3j.2`). Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.a.ii`
  Status: `done` (`2026-09-18`, DECISION RECORD)
  Goal: **`.3j.2.a.i` shipped a resolution mode no accepted decision record authorizes — get the authority
  or withdraw the rule.** Opened by `.3j.2.b` while reading ADR 0037 for a different question, with the
  case on both sides already stated: for, that §3's concern is identity **minting** and the mode lands on a
  declared name warranted by a typed stated width rather than the resemblance §1 forbids, and that it is
  alpha-equivariant under §7; against, that §3 reads as an exhaustive permission and an accepted ADR is not
  extended by a leaf's reasoning.
  **RESOLVED BY EXTENDING, NOT BY ASSERTING.** `ADR 0047` is accepted: subject resolution has exactly
  **three** modes — exact, unique case-fold, and a full-width slice verified against a width the document
  states — and any fourth needs its own record. The extension is argued on ADR 0037's own terms rather
  than around them. §1 is untouched because the warrant is a **typed width declaration**, which §2 already
  calls bounded definitional grammar, and not the resemblance, prefix, suffix or substring §1 forbids; §3's
  actual concern is identity **minting**, and the mode lands on a name the document already declares,
  cannot add an external identity and fails closed on ambiguity; §7 holds because renaming a declaration
  renames its width statement and every slice spelling with it.
  **Both guards are written into the decision as non-redundant, because the A/B proved they are.** `lo == 0`
  is not implied by the width comparison: a top-bit slice `X[w-1]` satisfies `hi + 1 == w` on its own, so
  a width test alone would admit exactly the one-bit case the decision exists to refuse. ADR 0047 §4 says
  so explicitly, so a future reader cannot drop one as redundant.
  **The rejected candidates are recorded with their measured precision**, so the next proposal starts from
  evidence: a general widening 4 of 16, a qualifier 1 of 6, a proper sub-slice never, and a slice without a
  stated width never — with 209 of 353 declared names stating a width at all.
  **A second defect was found and fixed while writing it, and it is named rather than silent.** ADR 0037's
  own `reverify` ran `cargo test -p specforge --lib alpha_`, which executes **3** alpha-equivariance
  controls; the same filter against `specforge-core` — the crate `crates/specforge/src/ir/**` compiles
  into by `#[path]` — executes **11**. The decision that DEFINES alpha-equivariance was verifying it with a
  command reaching a fifth of its controls. All 14 pass, so nothing regressed; the command was corrected
  and both the correction and the reason are recorded in ADR 0037's own `## Extensions` section and in
  ADR 0047's consequences. This is `COMMIT-GATE-SINGLE-RUN.5`'s defect in a third location.
  Prerequisite: none. Blocks: nothing; subject resolution is unblocked for further work under ADR 0047.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3j.2.a.ii`

- [x] **REPRODUCE / MEASURE** — ADR 0037 §3 enumerates two resolution modes; `.3j.2.a.i` ships three.
  `cargo test -p specforge --lib alpha_` runs **3 tests / 470 filtered out**; `cargo test -p specforge-core
  --lib alpha_` runs **11 tests / 1,546 filtered out**. Both green, so the gap is in verification reach,
  not in behaviour.
- [x] **ROOT CAUSE (WHY + WHERE)** —
  `docs/decisions/0037-identifiers-are-opaque-and-one-way-grounded.md` §3 (the enumeration) and its
  `reverify:` front-matter line (the crate name). The shipped third mode is
  `entity_typing::resolve_full_width_slice_alias`, wired at
  `commands/extract_constraints_llm.rs` and warranted by `evidence::stated_signal_widths`.
- [x] **ADDRESSED (verified)** — `docs/decisions/0047-…md` accepted and indexed; ADR 0037 carries an
  `## Extensions` section naming the extension and the command correction, with **no decision text
  changed** (the record's Append/supersede rule is respected — a verification command is not a decision).
  The 14 alpha-equivariance controls pass in both crates **with the third mode shipped**, which is the
  direct evidence for ADR 0047 §9.
- [x] **NO REGRESSION** — `cargo test --workspace --lib --exclude specforge-production-graph`
  **2,192 passed / 10 ignored / 0 failed**, unmoved: this slice changes no Rust. The Knowledge Map
  re-derives to **336 facts / 2,831 question keys** and its gate passes; `LIVE-DOC-SIZE` passes with the
  new decision record inside its `files` budget.
- [x] **GENERICITY (ADR 0006)** — the decision adds no vocabulary and authorizes no spelling inference; it
  bounds one resolution mode to a typed width the document states.
- [x] **LOCKSTEP** — the book already states this behaviour and both its guards (`.3j.2.a.i` shipped that
  text); the decision record is the layer that was missing, not the user-facing description. No production
  rule was added or deleted here, so no book text describes behaviour that has gone.
  Verification: ADR 0047's own `reverify` chain — the alpha controls in both crates plus the three
  full-width-slice controls
  Commit: `EXTRACTION-QUALITY-GAUGE.3j.2.a.ii — ADR 0047: three resolution modes, and the third one names its guards`
