# EXTRACTION-QUALITY-GAUGE — acceptance checklists foundation

- Part ID: `acceptance-checklists-foundation`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:checklists-founding-gates:start -->
### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.6`

- [x] **REPRODUCE / MEASURE** — `cargo run -- replay-constraints --evidence-root generated/evidence_ir`:
  `documents_scanned: 78`, `documents_skipped: 1`, `persisted_deterministic_records: 179`,
  `reproduced: 144`, `not_reproduced: 35`, `granted_declarations: 67`. Ten partial documents and one
  fully frozen (`ihi0079_b … amba_cxs 0/2`). By refusing gate: 17 none, 12 `CORPUS-COVERAGE.2.50a`,
  4 `.3k.1`+`.2.50a`, 1 `.3h` value-position, 1 `.3g` dotted-cross-reference.
- [x] **ROOT CAUSE (WHY + WHERE)** — the question had no instrument. `crates/specforge/src/ir/evidence.rs`
  exposed no way to ask a persisted record whether it still comes out, and `generated/` cannot answer
  it: 54 of 78 documents have no normalized bundle, so their artifacts cannot be rebuilt and freeze at
  the generation that wrote them. `.3k.1` had to establish that by hand, one probe per record.
- [x] **ADDRESSED (verified)** — `replay_persisted_signal_constraints` in `ir/evidence.rs` plus the
  `replay-constraints` command. **Calibrated in both directions against artifacts whose generation is
  known:** APB 15/15 and AHB 13/13 — both rebuilt by `.3i`, so a current artifact reproduces
  completely — and AMBA CXS 0/2, whose two records are exactly the `must not be asserted` shape `.3i`
  retyped, so a known-stale artifact reproduces nothing. Two independent confirmations that the
  instrument measures generation drift rather than noise. Fidelity control run separately: every
  persisted record's `source_text` and `supporting_statement_ids` resolve inside its own artifact
  (0 orphans corpus-wide), so a NOT-REPRODUCED verdict is never a missing-statement artifact.
  **Observed RED, and it changed the published number:** the first widening seeded the catalog
  `HashSet` the producer takes, which only the inference-antecedent sibling reads;
  `a_subject_no_statement_declares_is_still_granted_its_trial` failed, and fixing it to insert
  declaration STATEMENTS moved the corpus figure 120/179 → 144/179.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`, and
  `cargo test` green; `specforge-core` lib 1,441 → 1,445, no existing expectation changed.
  `scripts/check_doctrines.sh` green. Read-only by construction: the command never writes, and it
  loads through `load_for_inspection`, so no artifact's proof context moves.
- [x] **GENERICITY (ADR 0006)** — no document, protocol, vendor or signal name in the producer or the
  command; the strata are selected by `constraint_id` prefix and the controls use invented names
  (`ZETASTRB`, `ZETAOAS`, `ZETADTI`, `ZETAKEEP`).
- [x] **LOCKSTEP** — code, this leaf, `[[persisted-census-measures-published-not-current]]` (whose
  `reverify` is now this command), the book's command pages, `TOOLBOX.md` §5.5 and the chooser row,
  and the resume pointer agree before commit.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.1`

- [x] **REPRODUCE / MEASURE** — two censuses, and the second one overturned the first.
  (1) `python3 scripts/measure_constraint_part_span.py` over the 78 persisted artifacts: the
  `reference-operand magnitude` row is **4** in the `sigcon_*` stratum and **0** in `dyn_sigcon_*`,
  `row_sigcon_*` and `llm_sigcon_*` — AMBA DTI `sigcon_0002`–`0005`, publishing
  `OAS must_be_stable, negated` and `DTI must_be_stable, negated`, i.e. *"OAS must not be stable"*.
  The same four are the ENTIRE `negation on untyped default` population, so the two halves `.3k`
  inherited as "7 plus 4" are one set of 4.
  (2) **Running the real producer on that exact sentence yields `records=[]`.** A probe calling
  `extract_signal_constraints` with `OAS`/`DTI` declared reported
  `PROBE records=[] … post_passive OAS=true DTI=true`: `CORPUS-COVERAGE.2.50a` already refuses both
  subjects because the sentence names them only after `must not be`. The four records predate that
  gate, and DTI has no retained normalized bundle, so nothing has rewritten them
  (`[[persisted-census-measures-published-not-current]]`).
  (3) The class is reachable anyway — a third probe on the same grammar with the subject named BEFORE
  the lead returned `ZETARANGE MustBeStable negated: true`, and on the dynamic path
  `ZETARANGE MustBeLow` plus `ZETAOAS MustBeLow`. That is what this leaf gates.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`.
  `is_relational_equality_constraint` (`.3d`) expresses the refusal for an inter-operand EQUALITY and
  nothing for a comparative MAGNITUDE, so a bound stated against another operand reaches the kind
  classifier, matches no phrase in its table, falls to the terminal `MustBeStable` default, and then
  carries the sentence's `must not` on top — a record asserting that a signal must not be stable,
  which no sentence of this shape says. The right operand is additionally minted as a second subject
  (`ZETAOAS must_be_low` above; `OAS` in the live artifact).
- [x] **ADDRESSED (verified)** — pure `is_reference_magnitude_constraint(text)`: a comparative marker
  (`greater than `, `less than `, `larger than `, … each carrying its trailing space) IMMEDIATELY
  followed by a phrase naming another operand's attribute (`the value of`, `the size indicated by`,
  `the number of`, `that indicated by`, …). Wired beside the `.3d` refusal in BOTH deterministic
  extractors. **Observed RED with both call sites reverted:**
  `a_magnitude_against_a_referenced_operand_yields_no_constraint` fails emitting
  `MustBeStable, negated: true` from *"ZETARANGE must not be greater than the size indicated by the
  ZETAOAS field"*, and `…_yields_no_dynamic_constraint` fails emitting two `MustBeLow` records; both
  pass restored. The literal controls hold in both directions: *"The value of ZETARANGE must be
  greater than 0"* still extracts, and the HBM2 shape *"sets ZETADBI HIGH when the number of
  transitioning data bits within a byte is greater than 4"* still yields exactly `must_be_high`.
- [x] **NO REGRESSION** — `cargo fmt --all --check` green; `cargo clippy --all-targets -D warnings`
  green; `cargo test` green with `specforge-core` lib **1,435 → 1,440** and no existing expectation
  changed. `scripts/check_doctrines.sh` green (13/13 executed, 2 CI-tier deferred). No persisted
  artifact moves: the only corpus instance is in a document with no normalized bundle, so there is
  nothing to rebuild and nothing to diff — stated as a limit, not as coverage, exactly as `.3h` did
  for NVMe `FFFF`.
- [x] **GENERICITY (ADR 0006)** — two phrase lists of ordinary English comparatives and reference
  leads. No document, protocol, vendor, register or signal name appears in the rule; every control
  uses invented names (`ZETARANGE`, `ZETAOAS`, `ZETADTI`, `ZETADBI`).
- [x] **LOCKSTEP** — code, this leaf, the two Knowledge Map cards, the book's EvidenceIR gate
  narrative, and the resume pointer agree before commit. The refusal's own span defect is not
  silently inherited: it is written down and owned by `.3k.5`.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3i`

- [x] **REPRODUCE / MEASURE** — two read-only censuses over the 78-document persisted corpus. 20 records
  carry `negated`; **4** have no negator in their own obligation clause (RISC-V IOMMU
  `dyn_sigcon_0008`/`0009` from `The DV operand must be 1 for IODIR`, NVMe `dyn_sigcon_0018`, I2C
  `dyn_sigcon_0008`); **20 of 20** sit on a kind no phrase matched, because every phrase in the table is
  affirmative and a negated obligation never contains one. The second census — phrase match over the
  clause vs over the whole text — differs on **18 of 349**.
  **CORRECTED by `.3k`'s scoping (`2026-09-12`): both of this leaf's censuses were run over the whole
  349-record table, and `classify_signal_constraint_kind` has only 111 of them in its reach.** The
  second census re-derives to **19** over 349 — of which 12 are `llm_sigcon_*` and 3 `dyn_sigcon_*`,
  strata whose kind never passes through that function — and to **4** over the classifier's own
  `sigcon_*` population, which is what `.3k.3` inherits. "20 of 20 sit on a kind no phrase matched" is
  true of the `sigcon_*` stratum and vacuous for the `dyn_sigcon_*` one, whose kind comes from its
  value binder. The fix this leaf shipped is unaffected — a negation read from another sentence is a
  defect at any population size — but the numbers it published are not the ones it measured.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. `extract_signal_constraints`
  computed `let lowered = text.to_ascii_lowercase()` over the WHOLE statement and used it for both kind
  and negation, while the subject used `constraint_bearing_sentence(text)` and the condition said so in
  its own comment (*"from the SAME bounded obligation the subject came from"*,
  `CONSTRAINT-EXTRACTION-V2.2`). `extract_dynamic_signal_constraints` did the same with its own inline
  copy of the negator list. The record's parts were drawn from different spans of one statement.
- [x] **ADDRESSED (verified)** — `obligation_is_negated(&constraint_bearing_sentence(…).to_ascii_lowercase())`
  in both paths, the dynamic path's duplicated list replaced by the shared predicate `.3` extracted, and
  the two missing spellings added. APB + AHB rebuilt `evidence → validate → semantic → validate → intent
  → validate → adapt`, zero failures: **3 retyped, 0 added, 0 removed**. `PSTRB`
  `must_be_stable`+negated → `must_be_low`, `HSIZE` → `must_not_change`, `HEXOKAY` →
  `must_be_deasserted`. **`PSTRB` is the corroboration**: polarity-refined to `LOW`, it now agrees with
  `dyn_sigcon_0015`, the same document's prose *"For read transfers, the Requester must drive all bits
  of PSTRB LOW"* — two independent extraction paths converging where they previously contradicted, which
  is why APB's `actor_contracts` fall 15 → 14.
  **Observed RED**: with the narrowing reverted,
  `a_negation_in_another_sentence_does_not_negate_this_obligation` fails and emits
  `MustBeValue { value: "1" }, negated: true` from `The ZETADV operand must be 1 for ZETADIR` — the
  fixture reproduces the live `dyn_sigcon_0008` exactly.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1435** / 4 green. One existing control changed
  expectation, and it is **the fix landing rather than a regression**:
  `invariant_shape_admission_3::a_row_states_a_constraint_only_when_its_clause_binds_to_that_row_signal`
  asserted `PSTRB` was `negated`; `INVARIANT-SHAPE-ADMISSION.3`'s own result recorded that record as
  imprecise and named this leaf as its owner. It now asserts `must_be_deasserted` with `negated: false`
  by the `WIRE-BASED-100.5b` guard. `cargo fmt --check` and `cargo clippy --all-targets -D warnings`
  green; `scripts/check_doctrines.sh` green. Retention at the declared 24: both held-out bundles
  restored from `generated/preserved/WIRE-BASED-100.10/`, `diff -r`-verified unchanged by the rebuild,
  and removed. Pre-rebuild snapshot at `generated/preserved/EXTRACTION-QUALITY-GAUGE.3i/pre-rebuild/`.
- [x] **GENERICITY (ADR 0006)** — a span narrowing plus four phrase strings that are the negative
  spellings of forms already in the table. No document, protocol, vendor, or signal name appears in the
  rule; the controls use invented names (`ZETADV`, `ZETASIZE`, `ZETAOKAY`, `ZETASTRB`).
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md` gains "A negation belongs to the obligation
  it modifies". No production rule was deleted; two phrase forms were added and one span narrowed, and
  the book says which.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3h`

- [x] **REPRODUCE / MEASURE** — read-only census over all 248 deterministic constraint records: exactly one
  value-position-only subject (NVMe `dyn_sigcon_0011` `FFFF`), across one document, zero wire-protocol records.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: the pattern path excludes only the
  value it bound itself, and the dynamic path's subject scan admits any uppercase run, so a literal bound by a
  *different* phrase (`set to FFFFh`) reaches the subject slot; `.3d`–`.3g` express no value-position rule.
- [x] **ADDRESSED (verified)** — `is_value_position_subject` guards both paths; the exact NVMe sentence now yields
  no `FFFF` record, and standalone/ordinary value-binding subjects are provably kept.
- [x] **NO REGRESSION** — `kg-bench` 156/156; full `run_ci.sh` GREEN with 1,807 tests / five ignored; an isolated
  replay over all 22 rebuildable documents is unchanged 22/22, so the gate alters zero live records outside NVMe;
  57/57 emitted ISFs stay FSMGen-strict clean.
- [x] **GENERICITY (ADR 0006)** — value-binder preposition grammar plus identifier-boundary occurrence; no name,
  radix, literal-shape, vendor, or document list.
- [x] **LOCKSTEP** — code, this leaf, the book gate narrative, the Knowledge Map card, and the resume pointer
  agree before commit.
<!-- extraction-quality-gauge-task-source-region:checklists-founding-gates:end -->
