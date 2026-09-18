# EXTRACTION-QUALITY-GAUGE — llm path family

- Part ID: `llm-path-family`
- State: `active`

## Post-migration work

Leaves declared after the `2026-09-17` containment migration. The leaves that migration SEALED, and the
records that close them, live in [llm path sealed](llm-path-sealed.md)
(`LIVE-DOCUMENT-PRESSURE-HEADROOM.14d`).


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
