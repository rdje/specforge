# EXTRACTION-QUALITY-GAUGE — acceptance checklists untyped

- Part ID: `acceptance-checklists-untyped`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:checklists-untyped-family:start -->
### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2k`

- [x] **REPRODUCE / MEASURE** — the class demonstrated with the REAL producer on real corpus grammar:
  `extract_signal_constraints` over NVMe `statement_7397`'s own obligation clause returns
  `Some("must_be_value:UNIQUE")`, and over OpenCAPI `statement_0613`'s
  `Some("must_be_value:COMPATIBLE")` — both captured as the observed-RED assertion output, not
  predicted. Actionable population over the persisted corpus: **0**. The published deterministic
  `must_be_value` census is `0`/`1`/`5`/`12`/`0B01`/`0B11`/`VALID`/`LOW`/`NO`/`SET`/`PACKED`/`INVALID`/
  `INVALIDATED`/`UPDATED` and carries no relational value, so this leaf ships on the `.3k.1` footing
  `.3k.2e`/`.3k.2f` also used: the class is live and demonstrable through the reader even where the
  corpus has not yet published it. What makes it urgent rather than speculative is `.3k.3`'s addition
  measurement, which ran the narrowed producer over all 78 artifacts and watched NVMe gain exactly the
  two `must_be_value` records the container's ordering rationale predicted.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`'s terminal `generic_value` arm via `is_admissible_state_value`.
  `.3k.2b` gated that slot on ONE shape — the passive participle — and an adjective wears no participle
  ending, so `unique`, `compatible` and `greater` walk straight through a gate built to stop
  `invalidated`. The slot asks "is this word a value?" and had no way to notice that the predicate is
  not satisfied by the subject alone.
- [x] **ADDRESSED (verified)** — `value_slot_states_a_relation` refuses a predicate whose next word
  introduces a second operand or a scope, reached only from `is_admissible_state_value`'s final branch
  so the declared/level/numeric routes still win first; `protocol_state_value_and_complement` returns
  the value and that next word from ONE scan, with `extract_protocol_state_value` delegating to it.
  **Six controls in `mod extraction_quality_gauge_3k_2k`, two of them observed RED with the rule
  removed** — `must_be_value:UNIQUE` and `must_be_value:COMPATIBLE` — and the file restored
  byte-identically afterwards. The other four are the line the rule must not cross: a state complete at
  the word still binds, `by` is not a relation marker, all three admissibility routes win first, and
  the complement comes from the binder the value came from.
- [x] **NO REGRESSION** — `replay-constraints --evidence-root generated/evidence_ir` is **identical
  across the change: 183 persisted / 263 replayed / 137 reproduced / 126 unpersisted, over all 77
  loadable documents**, so no persisted artifact moves and no chain rebuild is owed. `kg-bench`
  **156/156**; `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings` and the
  whole workspace suite green (`specforge-core` lib 1,468 → **1,474**). One existing control failed and
  was CORRECTED rather than accommodated — see `.3k.1`'s amendment. `doctrine/production_genericity/
  flow_census.json` re-derived and attributed: `analyzed_functions` +2, `helper_edges` +34.
- [x] **GENERICITY (ADR 0006)** — universal English clause grammar: a predicate's complement marker.
  No adjective list, no value vocabulary, no document, protocol or vendor name. The one lexical set is
  a closed list of English prepositions, the same footing as `.3d`'s comparative markers, and `by` is
  excluded by its grammatical ROLE rather than by taste.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` gains *"A predicate that names a scope or
  another operand is not a value"* beside `.3k.2b`'s section, since both are about the same slot. The
  chapter's existing *"A bound stated against another operand is not a value"* table asserted
  `The value of PRANGE must be greater than 0` was **kept**; that is the sentence whose record this
  leaf refuses, so the row and its paragraph are corrected in the same edit — the
  `BOOK-METHOD-DOC`/`BOOK-BEHAVIOUR-CURRENCY` case of a changed rule leaving standing book text. KM card
  `[[a-relational-predicate-is-not-a-value]]`.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2j`

- [x] **REPRODUCE / MEASURE** — the check's own history re-derived per revision with that revision's own
  scanner over that revision's own `evidence.rs`: green at `1ada364a` (`.3k.1`), RED at `24a605e8`
  (`.3k.2a`) and still RED at `a81d70ab` (`.3k.2c`). And the sibling census, classified rather than
  counted: 86 script-shaped files, 28 unreachable from any driver/hook/workflow/doctrine registry, **2**
  of those claiming `--check` semantics.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_doctrines.sh` is the registry and the script was
  never added to it; `scripts/run_ci.sh` and `.githooks/pre-commit` both delegate to that registry, so
  one omission removed all three enforcement legs at once. The script itself was correct and simply
  never executed.
- [x] **ADDRESSED (verified)** — `scripts/check_constraint_part_span.sh` registered as
  `CONSTRAINT-PART-SPAN|gate`; the driver reports **PASS** and the count moves 13 → 14 executed
  doctrines. **Two observed-RED controls, both run through the DRIVER rather than the script, because
  what failed here was the wiring and not the check:** (a) making the row reader call the untyped
  classifier directly fails `CONSTRAINT-PART-SPAN` in the doctrine report; (b) adding a third producer
  (`zeta_probe_third_producer`) fails it naming the found set against the expected one — the exact drift
  `.3k` built the script for. `crates/` restored byte-identically after both (`git diff --stat` empty).
- [x] **NO REGRESSION** — no production code changes in this slice; `crates/` is byte-identical to the
  previous commit. `scripts/check_doctrines.sh` reports **all 14 gate-tier doctrines PASS**, the meta-check
  that every registered enforcer exists and is executable included. `cargo fmt --all --check`, `cargo
  clippy --offline --all-targets -D warnings` and the whole workspace suite stay green; `kg-bench`
  **156/156**.
- [x] **GENERICITY (ADR 0006)** — enforcement wiring only; no extraction rule, no vocabulary, no
  document, protocol or vendor name.
- [x] **LOCKSTEP** — the adapter, the registry line, and the `DOCTRINE_ENFORCEMENT.md` §10 row are the
  three surfaces that must agree and they are written together; the §10 row records how the gap was
  found and the sibling enumeration, so the next reader inherits the census rather than the conclusion.
  `TOOLBOX.md` needs no change: it catalogs diagnostic tools, and this script's entry is its doctrine
  row. `RETAINED-BUNDLE-POPULATION-FROZEN` keeps the one sibling finding; it is named here, not moved.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2e`

- [x] **REPRODUCE / MEASURE** — derived with the producer, after `.3k.2g` made that possible. Over the
  26 documents whose table classifications the current loader accepts, the row reader mints **12
  records and not one more**, and every one is typed by an arm the document wrote — so the untyped
  fallback's actionable population is **0**, not the 17 `.3k.2d` published from a mirror over the
  persisted `table_kind`. Baseline `replay-constraints --evidence-root generated/evidence_ir`: 183
  persisted / 137 reproduced / 46 not-reproduced.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_signal_description_row_constraints`: it called `classify_signal_constraint_kind`, whose
  terminal arm returns `MustBeStable` for a clause no phrase matched. `.3k.2a` left that deliberately,
  and its justification is what expired: the four APB clauses it reasoned from were typed by `.3k.2c`.
  Demonstrated on the real reader — a MATCH, an ALIGNMENT and a PRESENCE claim each publish
  `must_be_stable`, the last negated.
- [x] **ADDRESSED (verified)** — the row reader goes through `classify_signal_constraint_kind_typed`
  and refuses on `None`, so no producer can publish the terminal arm. **Observed RED:** restoring the
  untyped call republishes exactly those three records (`ZETAREADY`/`OMEGABURST` `must_be_stable`,
  `SIGMASTRB must_be_stable, negated: true`) while the over-kill guard stays green — the pair is the
  property, because a refusal that also dropped a stated obligation would be a worse trade than the
  fabrication. Corpus re-measured: **183 / 137 / 46, unchanged** — 0 added, 0 removed, 0 retyped, no
  persisted artifact moves.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the whole workspace suite green; `specforge-core` lib **1,466 → 1,468**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD. All 13 gate-tier doctrines
  PASS; `flow_census.json` unmoved by this slice (the refusal replaces an assignment rather than adding
  a branch the census counts). The over-kill guard asserts all four arms the live records use still
  publish from a row: header-supplied validity, `.3k.2c`'s no-change spelling, a negative polarity
  form, and a plain value binding.
- [x] **GENERICITY (ADR 0006)** — no new rule and no vocabulary: one caller now uses the typed
  gateway the other already used. Controls use invented names (`ZETAREADY`, `OMEGABURST`,
  `ALPHACHUNK`, `SIGMASTRB`).
- [x] **LOCKSTEP** — code, this leaf, `.3k.2a` (whose asymmetry is superseded in place, with the
  reasoning kept and its population named as what changed), the book's EvidenceIR chapter, and the
  resume pointer agree before commit. **A production rule IS replaced here** — the row path's fallback
  — and the book text that described it is repaired rather than left standing: the "An obligation that
  names no kind states no constraint" section ended with a subsection asserting the table-row reader
  keeps the fallback and why, which is no longer true. That section now lives in
  `docs/book/src/pipeline/obligation-reading.md`: this leaf's own book edit pushed the EvidenceIR
  chapter 299 bytes past its ceiling and `shipped_behavior` blocked the commit, which
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.19` resolved by splitting the chapter rather than by trimming the
  paragraph — the remedy `.3` had already adjudicated for this exact surface. `scripts/measure_constraint_part_span.py` is
  re-derived, not re-pinned: its topology now tracks the typed gateway and adds the untyped-caller
  invariant, and its self-test gains a case for the `#[cfg(test)]` scope bug the re-derivation exposed
  (9 → 10 declared cases).

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2f`

- [x] **REPRODUCE / MEASURE** — a set claim, carried with its enumeration in both directions.
  `is_relational_equality_constraint` and `is_reference_magnitude_constraint` have exactly one caller
  each, `extract_signal_constraints`; the row reader calls neither. Population over the corpus, derived
  with the real producer now that the row stratum is judged (`.3k.2g`): of the row clauses the 26
  judgeable documents admit, **0** match either predicate. Baseline `replay-constraints
  --evidence-root generated/evidence_ir`: 183 persisted / 137 reproduced / 46 not-reproduced.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_signal_description_row_constraints`: the clause goes from `obligation_subject` straight to
  `classify_signal_constraint_kind`, with no vocabulary-slot gate between them. Demonstrated through
  the real reader, not read off a diff: a signal-description table whose cell states either relation
  publishes `ZETARANGE must_be_stable, negated: true` and `OMEGABURST must_be_value VALUE`.
- [x] **ADDRESSED (verified)** — both predicates evaluated over the clause, before classification.
  `a_clause_with_no_vocabulary_slot_is_refused_in_the_row_path_too` goes from two fabricated records to
  none. **Observed RED:** removing the guard restores exactly those two records, with the kinds quoted
  above; restored green. Corpus re-measured after the change: **183 / 137 / 46, unchanged** — 0 added,
  0 removed, 0 retyped, and no persisted artifact moves.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the whole workspace suite green; `specforge-core` lib **1,464 → 1,466**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD and `temporal_rule 1.000` on
  APB/AHB/AXI. All 13 gate-tier doctrines PASS. `flow_census.json` re-derived and attributed to this
  leaf (+1 decision site, +2 helper edges). The over-kill guard is its own control: an ordinary row
  obligation still extracts, and both predicates are asserted to keep NOT firing on a magnitude against
  a literal (*"must be greater than 0"*) or on `.3k.2c`'s *"the same value IN …"*.
- [x] **GENERICITY (ADR 0006)** — no new rule; two existing predicates reach a second caller. Both are
  keyed on ordinary English comparatives and reference leads, and the controls use invented names
  (`ZETARANGE`, `ZETAOAS`, `OMEGABURST`, `ZETAREADY`).
- [x] **LOCKSTEP** — code, this leaf, and the book's EvidenceIR chapter agree before commit: the "A
  bound stated against another operand is not a value" section said the deterministic paths refuse
  these sentences, which was true of one path and is now true of both, so the chapter's claim is
  repaired rather than extended. No production rule is deleted or replaced. The resume pointer's next
  action moves on.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2g`

- [x] **REPRODUCE / MEASURE** — the instrument's own blind spot, stated as a census rather than an
  impression: `replay_persisted_signal_constraints` filtered the judged set to `sigcon_*` and
  `dyn_sigcon_*`, so the 12 published `row_sigcon_*` records (APB 10, AXI-L 1, AHB 1) were invisible to
  every corpus figure `.3k.6` and `.3k.2a`–`.3k.2d` quoted. Baseline
  `replay-constraints --evidence-root generated/evidence_ir`: 171 persisted / 125 reproduced / 46 not.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `replay_persisted_signal_constraints`: the `deterministic` filter names two id prefixes, and the
  replay never calls `extract_signal_description_row_constraints` because that producer needs a
  `SourceIr` the function was not given. The second cause was found by probing the real producer, not
  by reading: with LTI's persisted `SourceIr` loaded through `load_for_inspection`,
  `should_treat_table_as_top_level_signal_description` passes **0 of 88** tables — because
  `neutralize_legacy_source_classifications` (`crates/specforge/src/ir/source.rs`) sets every
  `table_kind` to `Unknown` for a legacy artifact, while that document's persisted JSON marks 25 tables
  `signal_description`.
- [x] **ADDRESSED (verified)** — the row pass is composed exactly as the build composes it and the
  stratum is gated on `SourceIr::carries_canonical_source_classifications`. Corpus: **183 persisted /
  137 reproduced / 46 not-reproduced**, i.e. +12 persisted and +12 reproduced — **all 12 row records
  reproduce**, which re-derives without a rebuild what `.3k.2d` had to rebuild APB to establish, and
  extends it to AHB and AXI-L. The statement and dynamic strata are byte-identical to the baseline,
  compared record by record. **Three observed-RED controls, one per decision:** (a) suppressing the row
  pass while still judging the stratum makes
  `the_row_stratum_is_judged_only_when_its_producer_can_run` fail, reporting a live record as lost;
  (b) removing the schema gate makes the corpus report claim **77 documents judged instead of 26**,
  including LTI, whose producer sees nothing — the silent zero itself; (c) neutralizing `table_kind` on
  an otherwise identical `SourceIr` empties the row producer's output in
  `a_legacy_source_ir_is_not_a_classification_authority`. All restored green.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the whole workspace suite green; `specforge-core` lib **1,461 → 1,464**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD, `temporal_rule 1.000` on
  APB/AHB/AXI. `scripts/check_doctrines.sh` all 13 gate-tier PASS. `flow_census.json` re-derived and
  attributed to this leaf (+2 analyzed functions, +9 decision sites, +6 helper edges, +1 semantic
  macro); the workspace's own `current_repository_flow_is_complete_local_and_deterministic` caught the
  drift before the driver did, which is the second gate working.
- [x] **GENERICITY (ADR 0006)** — no document, protocol, vendor or signal name enters the production
  path; the gate is a schema comparison and the composition mirrors the build's. The controls build
  their own `SourceIr` with invented names (`ZETAREADY`, `ZETASELX`). AMBA LTI is named only in a
  comment and in this leaf, as the measured instance.
- [x] **LOCKSTEP** — code, this leaf, `.3k.2e`/`.3k.2i` (whose populations this leaf withdraws), the
  book's EvidenceIR chapter, `TOOLBOX.md` §5.5, and the resume pointer agree before commit. No
  production rule is deleted or replaced. Knowledge Map card
  `[[legacy-source-classifications-are-neutralized-on-load]]` records the mechanism, because a pass
  keyed on any typed source classification hits it and the empty result looks like an answer.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2d`

- [x] **REPRODUCE / MEASURE** — the population enumerated in both directions over all 78 persisted
  artifacts, as the full cross product of the six modals `obligation_is_negated` accepts with the
  binder verbs `extract_protocol_state_value` reads, not the four spellings the leaf opened with
  (which gave 2 and was wrong). Statement path: **31** `signal_value_constraint` statements carry a
  negated binder — 13 `must_not_change`, 6 `must_be_deasserted`, **12 generic/untyped**. All 12 read
  against source: 9 `cannot/will not be changed`, 2 DTI reference magnitudes (`.3k.1` refuses them),
  1 AMBA LPI *"QREQn cannot be driven HIGH until the handshake is completed"*. Row path: 50 admitted
  obligation clauses, 0 carrying one of these modals. Baseline `replay-constraints --evidence-root
  generated/evidence_ir`: 171 persisted / 125 reproduced / 46 not / 126 unpersisted, captured per
  record before the change.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. Three functions read one
  clause for its modal with three vocabularies: `obligation_is_negated` (six modals),
  `constraint_bearing_sentence` (`must`/`shall` only, so an obligation stated with `cannot` gets no
  sentence and the span silently falls back to the whole statement), and
  `classify_signal_constraint_kind` (every phrase spelled `must`/`shall`, so the same clause types as
  nothing and — since `.3k.2a` — publishes nothing). Not the mechanism the leaf named: the record it
  cited, RISC-V IOMMU `dyn_sigcon_0008`/`0009`, comes from `statement_1033`, class `conditional_rule`,
  through the dynamic path, which does not call this classifier. Verified by locating the statement
  and its records in the artifact, not by reading the leaf.
- [x] **ADDRESSED (verified)** — `normalize_obligation_modal` reduces `cannot`/`can not`/`will not`/
  `must never`/`shall never` to `must not`; `sentence_states_an_obligation` gives the sentence scan the
  same set. **Corpus effect measured record by record over all 78 artifacts: 0 added, 0 removed, 0
  retyped** — the per-record `replay-constraints` dump is byte-identical to the baseline. APB rebuilt
  (`evidence → validate → semantic → validate → intent → validate → adapt`): 23 constraint records,
  identical identities AND identical ids, all 10 `row_sigcon_*` included; only `proof_context`,
  `proof_ledger` and `validation_reports` differ, and the restored bundle was `diff -r` byte-identical
  before removal (retention back at 24). **Two observed-RED controls, one per decision, each isolating
  the other:** disabling `normalize_obligation_modal` fails
  `the_equivalent_negative_modals_reach_the_same_kinds` with `MustBeStable` where `MustNotChange` is
  correct (and `a_self_negating_kind_still_refuses_the_flag_under_any_modal` with it); narrowing
  `sentence_states_an_obligation` back to `must`/`shall` fails
  `the_obligation_sentence_is_found_by_the_same_modal_the_kind_is`, returning the whole serialized row
  where the obligation's own sentence is correct. Both restored green.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the full suite green; `specforge-core` lib **1,456 → 1,461**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD and `temporal_rule
  P=R=F1=1.000` on APB/AHB/AXI. `scripts/check_doctrines.sh` all 13 gate-tier PASS after
  `doctrine/production_genericity/flow_census.json` was re-derived and attributed to this leaf
  (`analyzed_functions` +2, `helper_edges` +3, `decision_sites` −1 — the inline disjunction
  `constraint_bearing_sentence` spelled becomes one `contains_any` call). Corpus replay unchanged at
  171 / 125 / 46 / 126.
- [x] **GENERICITY (ADR 0006)** — universal English modal equivalence. Five modal strings and one
  canonical form, no document, protocol, vendor or signal name; the set is exactly
  `obligation_is_negated`'s, so the two halves of a record cannot again recognise different modals.
  Controls use invented names (`ZETALEN`, `ZETAOKAY`, `ZETARESP`, `ZETASEL`, `ZETADATA`, `ZETASTRB`,
  `ZETANOTE`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter and the resume pointer agree before
  commit. No production rule is deleted or replaced, so no book text describes behaviour that is now
  gone; the chapter gains the modal rule beside `.3i`'s negation rule and `.3k.2c`'s phrase rule.
  Knowledge Map card `[[one-modal-vocabulary-per-constraint-record]]` records the measured zero so the
  next session does not re-derive it.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2c`

- [x] **REPRODUCE / MEASURE** — 56 persisted statements contain *"have/has the same value"*; only 6
  deterministic records come from one: APB `sigcon_0009`/`0010` (typed by the validity arm, correct)
  and `row_sigcon_0018`/`0019`/`0021`/`0022` (the untyped fallback, published as `must_be_stable`).
  The 4 row records were read against source: each states that a signal holds one value across two
  phases or across every cycle of one, which is a no-change obligation.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`: the phrase table carries `must not change`, `must remain stable`,
  `must be stable` and `must hold`, and no form of *"must have the same value"*. The obligation is
  real and the reader had no spelling for it, so it fell through to the terminal arm.
- [x] **ADDRESSED (verified)** — a new arm for `must have the same value` / `shall have the same
  value` → `MustNotChange`, placed AFTER the validity arm. APB rebuilt
  (`evidence → validate → semantic → validate → intent → validate → adapt`): **4 records retyped, 0
  added, 0 removed**, and `sigcon_0009`/`0010` keep `must_be_value VALID`. **Two observed-RED
  controls, one per decision:** removing the phrase makes
  `the_same_value_spelling_types_as_no_change` fail with `MustBeStable`, and moving the arm ahead of
  the validity arm makes `a_cell_stating_both_obligations_keeps_its_validity_kind` fail with
  `MustNotChange` where `MustBeValue { value: "VALID" }` is correct. Both pass restored.
- [x] **NO REGRESSION** — `kg-bench` **156/156**; WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000`
  on APB, AHB, AXI and SWD. `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`,
  `cargo test` green; `specforge-core` lib 1,453 → 1,456. `replay-constraints` corpus unchanged at
  171 persisted / **125** reproduced / 46 not-reproduced / 1 named skip — the retype is inside the
  row stratum, which the replay does not judge, and it moved nothing in the two it does. Retention
  back at **24**, APB's bundle `diff -r`-verified byte-identical before removal.
- [x] **GENERICITY (ADR 0006)** — two phrase strings of ordinary English, each carrying its modal so a
  descriptive *"implementations that have the same value"* is untouched. No document, protocol, vendor
  or signal name; the controls use invented names (`ZETAUSER`, `ZETASELX`, `ZETAWUSER`, `ZETAPMCR`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter, and the resume pointer agree
  before commit. `.3d`'s equality refusal is asserted to keep NOT firing on *"the same value IN …"*
  while still firing on *"the same value AS …"*, so the two rules stay distinguishable.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2b`

- [x] **REPRODUCE / MEASURE** — the reproduced `generic_value` population is 4 `sigcon_*` records, all
  read against source: AXI `AWTAGOP must_be_value INVALID` (*"- AWTAGOP must be Invalid."*, correct),
  HBM2 `CKE must_be_value LOW` (*"CKE must be held LOW"*, correct), RISC-V IOMMU `GSCID` and DTI
  `DO_NOT_CACHE` (both `INVALIDATED` from *"must be invalidated"*, fabricated). A fifth instance,
  AXI `WTAGUPDATE must_be_value UPDATED` from *"the tags in memory must be updated"*, surfaced during
  the rebuild and is recorded as this leaf's residual rather than absorbed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`'s generic arm: `extract_protocol_state_value` returns the first
  non-filler word after a binding lead and the arm publishes it as a typed value with no test of what
  the word is. In a passive obligation that word is the obligation's verb.
- [x] **ADDRESSED (verified)** — `is_admissible_state_value(value, discovered_values)`: the document's
  own declared enum members first, then logic levels, then numeric literals, and otherwise a refusal
  of the past-participle shape only. Both deterministic call sites now derive the document's value
  vocabulary with `collect_discovered_enum_values`, the same set the dynamic path already binds
  against. **Two measurements steered the design and both are recorded above**: gating every value arm
  retyped 13 APB + 4 AHB correct records (reverted), and gating the generic arm against discovered
  values alone lost `AWTAGOP … Invalid` (narrowed). **Observed RED** with the admissibility test
  removed: `a_passive_participle_in_the_value_slot_is_not_a_value` emits
  `Some("must_be_value:INVALIDATED")` — the live `GSCID` defect on invented names — and
  `a_participle_the_document_declares_as_a_value_is_admitted` fails on its negative half; both pass
  restored.
- [x] **NO REGRESSION** — the chain was rebuilt for all three documents with a held-out bundle (APB,
  AHB, AXI-L): **0 records removed, 0 added, 0 retyped** in the final shape, and each bundle
  `diff -r`-verified byte-identical before removal, retention back at **24**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB, AHB, AXI and SWD.
  `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`, `cargo test` green;
  `specforge-core` lib 1,449 → 1,454. `replay-constraints` corpus: 171 persisted, **125** reproduced
  (was 127 — exactly the two fabrications), 46 not-reproduced, 1 named skip.
- [x] **GENERICITY (ADR 0006)** — universal English participle grammar plus the document's own
  declared vocabulary. No value list, no protocol, vendor, document or signal name; the controls use
  invented names (`ZETAGSCID`, `ZETATAGOP`, `ZETASTATE`, `ZETACKE`, `ZETAUSER`, `ZETASEL`) and an
  invented enum (`ZETASTATES Shared = 0`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter, and the resume pointer agree
  before commit; the `WTAGUPDATE` residual is written into `.3k.3` rather than left in a comment.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2a`

- [x] **REPRODUCE / MEASURE** — sized with `replay-constraints` FIRST, per the container's amendment.
  The published untyped-default population is 26 `sigcon_*`; the REPRODUCED population is **17**, and
  all 17 were read against their own source text. 17 of 17 are wrong: a barrier-transaction
  description (CoreSight ×2), a presence table cell (AXI ×4), a tied-low level statement (AXI ×1),
  two explicitly non-required recommendations (APB ×2), a response-duration description (AHB ×1), and
  six waveform narrations (LPI ×2, GFB ×3, HBM2 ×2 — `- T1 FREADY signal remains HIGH`,
  `At T3 … QDENY remains LOW`, `AERR is driven HIGH for 1 tCK`). The same census over the ROW stratum
  is the control that shaped the fix: its 4 untyped-default records are `PAUSER`/`PWUSER must have the
  same value in the Setup and Access phase`, which are CORRECT and merely under-typed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`'s terminal `else` arm: when no phrase matches and no value binds,
  it returned `SignalConstraintKind::MustBeStable`. In the statement path that arm fires on any
  sentence the classifier happened to class `SignalValueConstraint`, so a presence cell or a waveform
  step became an assertion that a signal must be stable. The row path reaches the same arm only after
  `obligation_subject` has proved the clause binds to its row's signal, which is why the same default
  is honest there and a fabrication here.
- [x] **ADDRESSED (verified)** — `classify_signal_constraint_kind_typed` returns `None` exactly for
  that arm (no stability or validity phrase anywhere in the clause) and `extract_signal_constraints`
  refuses on `None`; `classify_signal_constraint_kind` is byte-unchanged, so the row path is untouched.
  **Observed RED** with the call reverted: `a_statement_that_names_no_kind_yields_no_constraint` emits
  two `MustBeStable` records from `| Manager: False | ZETACHUNKEN is not present. …`, and
  `a_recommendation_that_names_no_kind_yields_no_constraint` emits one from
  `It is recommended, but not required, that ZETASLVERR is driven LOW …`; both pass restored.
  Controls hold: `must be stable`, `must be asserted` and `must not change` still type, and
  `classify_signal_constraint_kind` still answers `MustBeStable` for the row path's `must have the
  same value …` while the typed variant answers `None`.
- [x] **NO REGRESSION** — **chain rebuilt for all three documents whose proofs the change stales**
  (APB, AHB, AXI-L; each has a held-out bundle):
  `evidence → validate → semantic → validate → intent → validate → adapt --target isf`, zero failures,
  **8 records removed, 0 added, 0 retyped** — AXI 45→40, APB 25→23, AHB 14→13 — and every removal is
  one of the 17 adjudicated records. `kg-bench` **156/156**. WIRE-BASED-100 golds unchanged at
  `signal_constraint P=R=F1=1.000` for APB, AHB, AXI and SWD, and `actor_signal_relation` 1.000 on all
  four filtered. `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`, `cargo test`
  green; `specforge-core` lib 1,445 → 1,449. Retention back at the declared **24** bundles, each
  `diff -r`-verified byte-identical against its held-out copy before removal; corpus frontier
  `57 = 52 + 5` unchanged. Pre-rebuild snapshot at
  `generated/preserved/EXTRACTION-QUALITY-GAUGE.3k.2a/pre-rebuild/`.
- [x] **GENERICITY (ADR 0006)** — one negative predicate over the existing phrase table. No document,
  protocol, vendor or signal name; controls use invented names (`ZETACHUNKEN`, `ZETACHUNKV`,
  `ZETASLVERR`, `ZETASEL`, `ZETAADDR`, `ZETASTRB`, `ZETAUSER`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter, and the resume pointer agree
  before commit. The two findings the reading produced are not absorbed: the `generic_value` arm is
  2-2 and owned by `.3k.2b`, the row path's missing spelling by `.3k.2c`.

<!-- extraction-quality-gauge-task-source-region:checklists-untyped-family:end -->
