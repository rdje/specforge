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
