# EXTRACTION-QUALITY-GAUGE — untyped default family

- Part ID: `untyped-default-family`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:untyped-default-and-generic-value-leaves:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2` · Status: `active` (split `2026-09-12` after `.3k.6` re-sized it;
  re-split `2026-09-12` by `.3k.2d`'s census) · Children: `.3k.2a`-`.3k.2j` · Goal: **what a clause that types nothing may
  publish.**
  Two arms of `classify_signal_constraint_kind` emit a fact the document did not state: the terminal
  `untyped_default` publishes `MustBeStable` for any obligation no phrase matched, and the
  `generic_value` arm lifts whatever word follows `must be `/`shall be ` as a typed value with no gate
  at all. Re-sized with `replay-constraints` before splitting, exactly as the container's amendment
  requires: the published counts were 26 and 11; the REPRODUCED counts — the population a change can
  move — are **17 and 4**. Reading all 21 separates them cleanly, and the split follows the reading
  rather than the arm: **17 of 17 untyped-default records are wrong** and the arm must be refused,
  while **2 of 4 `generic_value` records are right** (`AWTAGOP must be Invalid`, `CKE must be held
  LOW`) so that arm needs a GATE, not a refusal. A third finding came out of the same reading: the
  ROW path's 4 untyped-default records are *correct* and merely under-typed, which is why the refusal
  is asymmetric.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2a` · Status: `done` (`2026-09-12`, CODE) · Goal: **the terminal
  `MustBeStable` is not a default, it is a fabrication — refuse it in the statement path.**
  **All 17 reproduced records adjudicated, 17 of 17 wrong:** two CoreSight `APB must_be_stable` from a
  barrier-transaction description; four AXI records from `| Manager: False | ARCHUNKEN is not present.
  …` (a presence cell); one from `Manager RCHUNKV input is tied low` (a level, not stability); two APB
  `PSLVERR` from *"It is recommended, but not required…"* and *"Completers are not required to
  support PSLVERR"*; one AHB `HRESP` from a two-cycle response description; six waveform narrations
  (`- T1 FREADY signal remains HIGH`, `At T3 … QDENY remains LOW`, `AERR is driven HIGH for 1 tCK`).
  Not one of them says anything about stability.
  **The asymmetry is the design.** The refusal is in the STATEMENT path only. The row path
  (`extract_signal_description_row_constraints`) keeps the fallback because it has already proved,
  via `obligation_subject`, that its clause binds to its row's own signal — so an untyped obligation
  there is a real obligation with a spelling the table lacks. Its 4 records are
  `PAUSER`/`PWUSER must have the same value in the Setup and Access phase`, which IS a stability
  obligation; typing it properly is `.3k.2c`.
  Shipped: `classify_signal_constraint_kind_typed`, which returns `None` exactly when the terminal arm
  is reached with no stability/validity phrase anywhere in the clause; `extract_signal_constraints`
  refuses on `None`. `classify_signal_constraint_kind` is unchanged, so the row path is untouched.
  **Superseded in part (`2026-09-13`, by `.3k.2e`): the asymmetry is gone.** It rested on the four APB
  `must have the same value` clauses, which `.3k.2c` then typed, so nothing correct reaches the row
  path's fallback any more and every producer goes through the typed gateway. The reasoning stands for
  its own population; the population is what changed.
  **Chain rebuilt for all three documents whose artifacts move — APB, AHB and AXI-L — because the
  change stales their proofs and all three have a held-out bundle.** `evidence → validate → semantic →
  validate → intent → validate → adapt` each: **8 records removed, 0 added, 0 retyped** (AXI 45→40,
  APB 25→23, AHB 14→13); every removal is one of the 17. The other 9 stay in documents with no bundle
  to rebuild from, and `replay-constraints` now reports them as not-reproduced rather than hiding them.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2a`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2b` · Status: `done` (`2026-09-12`, CODE) · Goal: **a passive
  obligation puts its VERB in the value slot.** `extract_protocol_state_value` lifts the first
  non-filler word after `must be `/`shall be `/`must remain `/`shall remain ` and says nothing about
  what it is. **Population: 4 reproduced `sigcon_*`, and it is 2-2** — right: AXI
  `AWTAGOP must_be_value INVALID` from *"AWTAGOP must be Invalid"* (an adjective) and HBM2
  `CKE must_be_value LOW` from *"CKE must be held LOW"* (a logic level); wrong: RISC-V IOMMU
  `GSCID` and DTI `DO_NOT_CACHE`, both `INVALIDATED` lifted out of *"must be invalidated"* — a past
  participle, which is what happens TO the thing, not what it equals.
  **The first design was too wide, was measured, and was reverted before it shipped.** Gating every
  value arm against the document's discovered enum values retyped **13 correct APB and 4 correct AHB
  records** from `must_be_value VALID` to `must_be_stable`: the `must be valid` arm's value is the
  validity CONVENTION `.8` established, not a word admitted on position, and `VALID` is in no
  document's enum set. The validity arm is therefore explicitly not gated, and the code says why.
  Gating only the generic arm against discovered values then still cost the one correct record whose
  enum table the discovery pass does not read (`AWTAGOP … Invalid`).
  **What shipped is exactly as wide as the evidence**: `is_admissible_state_value` admits a value the
  document declares, a logic level, or a numeric literal, and otherwise refuses only the PAST
  PARTICIPLE shape. So `Invalid` binds, `INVALIDATED` and `UPDATED` do not, and a participle-shaped
  enum member the specification does declare (`Shared`) is admitted through the document route — the
  pair that makes the override meaningful rather than decorative.
  **Measured effect: 2 fabricated records refused, 0 correct records lost, 0 artifacts changed.**
  Both instances live in documents with no normalized bundle, so `replay-constraints` reports them as
  not-reproduced (125 of 171, up from 127 of 171 by exactly these two) and nothing in `generated/`
  moves — the leaf is its own illustration of why the published and actionable populations differ.
  **Residual named, not absorbed:** AXI `WTAGUPDATE must_be_value UPDATED` survives, because its cell
  reaches the UNGATED validity arm on a `must be valid` later in the same cell while
  `extract_protocol_state_value` binds from the first `must be ` in the text. That is a span defect,
  and it belongs to `.3k.3`.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2b`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2d` · Status: `done` (`2026-09-12`, CODE) · Goal: **one modal
  vocabulary for the whole record.** Three functions read an obligation clause for its modal and each
  carried its own vocabulary: `obligation_is_negated` accepts `must not`/`shall not`/`must never`/
  `shall never`/`cannot`/`will not`; `constraint_bearing_sentence` looked only for `must`/`shall`; and
  every phrase in `classify_signal_constraint_kind` is spelled `must`/`shall`. So an obligation a
  document states with `cannot` was flagged NEGATED by the first, given no sentence of its own by the
  second, and typed as nothing by the third. This is the remaining half of `.3i`'s second finding
  (*"every phrase in the table is affirmative"*) one level up: every phrase in the table is also
  MODAL-specific, and `.3i` answered that with three more literals rather than with the rule.
  **The leaf opened on a different mechanism and its own census refuted it.** It said
  `extract_protocol_state_value` has no negated binder, citing *"The DV operand must not be 1 for
  IODIR"*. That sentence is a TEST STRING, not a corpus record: the document's own statement is
  RISC-V IOMMU `statement_1033`, class `conditional_rule`, and its records are `dyn_sigcon_0008`/
  `0009` — the DYNAMIC path, which never reaches this classifier at all. The rule was written from a
  description of the producer instead of from the producer (`CLAIM_VERIFICATION.md` §3 Leg 2).
  **The census, over all 78 persisted artifacts, enumerating the full cross product of the six modals
  `obligation_is_negated` accepts with the binder verbs `extract_protocol_state_value` reads.**
  Statement path: **31** `signal_value_constraint` statements carry a negated binder — 13 reach
  `must_not_change`, 6 reach `must_be_deasserted`, **12 reach the generic/untyped arm**. All 12 read
  against source: **9** are *"cannot be changed"* / *"will not be changed"* (eMMC ×2, USB4 ×5, SMBus,
  Wishbone) — a no-change obligation whose modal the table lacks; **2** are DTI reference magnitudes,
  correctly refused by `.3k.1`; **1** is AMBA LPI *"QREQn cannot be driven HIGH until the handshake is
  completed"* — a level obligation the table ALSO already owns (`must be driven high`), blocked by the
  same modal. Row path: 50 admitted obligation clauses, **0** carrying one of these modals.
  **So the value binder's own population is zero.** Not one of the 12 needs a negated binder; ten of
  them need the modal, and the vocabulary slot `.3k.2b` named (`MustBeValue` + `negated`) is reached
  by no corpus clause in either caller. The gap is real as a capability and empty as a population.
  Shipped: `normalize_obligation_modal` reduces the equivalent negative modals to the `must not` form
  the table is written in, and `sentence_states_an_obligation` gives the obligation-sentence scan that
  same vocabulary. **Measured over all 78 artifacts, record by record: 0 added, 0 removed, 0 retyped**
  — `replay-constraints` is identical before and after (171 persisted / 125 reproduced / 46 not / 126
  unpersisted), and APB rebuilt to the same 23 records with the same ids.
  **The zero is the result, not the absence of one, and the intermediate measurement proves the two
  halves cannot ship apart.** With only the classifier half, the replay gains exactly 2 records —
  eMMC `NOTE must_not_change` twice, from `| NOTE 1 | … A Device … will not change its state to the
  rcv state. … |`, whose subject is the serialized row's own NOTE marker. Teaching the classifier a
  modal the sentence scan cannot find moves the record's span to the whole row; teaching both leaves
  the obligation with a sentence of its own, and the marker is not in it.
  **Four findings routed rather than absorbed:** `.3k.2e` (the row path's untyped fallback), `.3k.2f`
  (the row path applies neither vocabulary-slot refusal), `.3k.2g` (`replay-constraints` does not judge
  the `row_sigcon_*` stratum at all), `.3k.2h` (`obligation_subject` is the next function still reading
  `must`/`shall` only, and a continuation row's empty name cell drops every obligation in it), `.3k.2i`
  (the row path's generic-value arm).
  **Correction (`2026-09-13`, by `.3k.2g`) — the row-path populations this leaf routed were measured
  with a MIRROR and three of them are withdrawn.** The census selected tables on the persisted
  `table_kind`; the producer selects on the loaded one, and a legacy artifact has every classification
  neutralized to `Unknown`. Re-derived with the real producer, the row path mints 12 records over the
  whole corpus, all already published: `.3k.2e`'s "17 of 17 wrong" is **0 actionable**, and `.3k.2i`'s
  "4 right / 5 wrong" is likewise unmeasurable rather than current. Each node carries its own
  re-derivation; the readings stand as estimates for the 51 legacy documents, never as counts.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2d`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2e` · Status: `done` (`2026-09-13`, CODE) · Goal: **proving the
  SUBJECT does not make the KIND readable — the row path's untyped fallback is refused too.**
  `.3k.2a` refused the terminal `MustBeStable` in the statement path and kept it in the table-row
  reader, on the reasoning that this reader has already proved via `obligation_subject` that its clause
  binds to its row's own signal, so an untyped obligation there must be a real obligation with a
  spelling the table lacks. **That reasoning was sound for the population it was made about and is no
  longer about any population.** It rested on four APB `must have the same value` clauses; `.3k.2c`
  gave the table that spelling and they stopped reaching the fallback at all.
  What reaches it now is a different thing: a clause whose obligation the constraint vocabulary cannot
  express. Both callers of the kind classifier now go through
  `classify_signal_constraint_kind_typed`, so **the terminal arm is unreachable as a published kind by
  any producer**, and `scripts/measure_constraint_part_span.py --check` pins that as its own invariant
  alongside the two producers' spans.
  **Measured effect: zero, and it is stated as a class rather than a count.** `replay-constraints` with
  the row stratum judged is unchanged at 183 persisted / 137 reproduced / 46 not-reproduced; all 12
  published `row_sigcon_*` records are typed by an arm the document wrote, so the refusal costs nothing
  measurable. The class is demonstrated through the REAL row reader: a MATCH (*"ZETAREADY must match
  OMEGABURST"*), an ALIGNMENT (*"Must be aligned to a burst size"*) and a PRESENCE claim (*"Must not be
  present"*) each publish `must_be_stable` without the refusal — the last one negated, i.e. *"SIGMASTRB
  must not be stable"* — and all three vanish with it. The over-kill guard is the half that decides
  whether the refusal is safe, and it is asserted as the four shapes the live records are made of:
  header-supplied validity, `.3k.2c`'s no-change spelling, a negative polarity form, and a plain value
  binding all still publish.
  **`.3k.2d`'s "17 of 17 wrong" stays withdrawn** (`.3k.2g`): that census selected tables on the
  persisted `table_kind` and the producer selects on the loaded one. The reading stands as an estimate
  for the 51 documents whose classifications are neutralized on load
  (`[[legacy-source-classifications-are-neutralized-on-load]]`), never as a current count.
  **Found in passing, and owned rather than noted:** `scripts/measure_constraint_part_span.py --check`
  — `.3k`'s own declared verification — has been RED since `.3k.2a`, and `.3k.2a`/`.3k.2b`/`.3k.2c`
  each shipped over it because nothing runs it. Re-derived per revision with each revision's own
  scanner, not by reading. `.3k.2j` owns wiring it into a driver.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2e`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2j` · Status: `done` (`2026-09-13`, DOCTRINE) · Goal: **a fail-closed
  check that no driver runs is not a check — writing one is not adopting it.**
  `scripts/measure_constraint_part_span.py` was built by `.3k` precisely so that a change to the kind
  classifier's call-site topology would fail closed rather than silently restratify the census that
  sized this whole container, and `.3k` names `--check` and `--self-test` in its verification. It was in
  no driver: not `scripts/check_doctrines.sh`, not `scripts/run_ci.sh`, not `.githooks/`.
  **Attributed by re-derivation, not by reading.** Running each revision's own scanner over that
  revision's own `evidence.rs` (`git show <rev>:<path>`): green at `.3k.1`; RED from `.3k.2a`, which
  introduced `classify_signal_constraint_kind_typed` so the bare classifier's only production caller
  became the wrapper; still RED at `.3k.2c`. Three leaves shipped over it, each reporting a fully green
  doctrine gate — because the gate never included it.
  Shipped: `scripts/check_constraint_part_span.sh` (self-test then check, the order `CORPUS-FRONTIER`
  states — a gate is not trusted on a day its own negatives have not been re-proven), registered
  `CONSTRAINT-PART-SPAN|gate` with its `DOCTRINE_ENFORCEMENT.md` §10 row. Gate tier is earned rather
  than assumed: `--check` is read-only, offline and sub-second, and touches the persisted corpus only
  on the census path, never on `--check`.
  **The set claim, with its enumeration, because "nothing else is unrun" is refuted by one
  counterexample.** Of **86** script-shaped files under `scripts/`, `knowledge-map/scripts/` and
  `tools/`, **28** are unreachable from any driver, git hook, CI workflow, or doctrine registry — and a
  raw reachability count is a population, not a defect count, so it is classified before it is
  published. Exactly **2** of the 28 offer a `--check` mode, i.e. claim gate semantics while nothing
  executes them: this leaf's subject, and `scripts/validate_canonical_recovery_contract.py`. The second
  is **already adjudicated here** — `RETAINED-BUNDLE-POPULATION-FROZEN` carries *"Does
  `scripts/validate_canonical_recovery_contract.py` still have an owner?"* as an open question — so it
  is named and left to its owner rather than re-opened. The remaining 26 are one-shot measurement probes
  (`measure_*.py`, each of which produced a number for one leaf) and PDF utilities, none of which claims
  a gate mode. The first pass of this census said 33 and was wrong: it missed that a doctrine registry
  invokes a verifier by `argv` rather than by a shell reference, which is the same
  derive-from-the-producer failure this container keeps finding.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2j`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2f` · Status: `done` (`2026-09-13`, CODE) · Goal: **a refusal that
  says the constraint VOCABULARY has no slot for what a clause states belongs to the rule, not to one
  caller.** `.3d`'s `is_relational_equality_constraint` and `.3k.1`'s `is_reference_magnitude_constraint`
  make the same argument — *the vocabulary can say "this signal must be `HIGH`" and cannot say "this
  operand is bounded by that one", so refuse rather than fabricate* — and both were evaluated only
  inside `extract_signal_constraints`. The table-row reader called neither, so one sentence was refused
  as a statement and published as a row.
  Shipped: both predicates evaluated over the CLAUSE the row reader is already holding, which is this
  producer's own unit and is also the scope `.3k.5` is moving the statement path's copies toward.
  **Measured corpus effect: zero. That is the honest result and it is stated as a class, not as a
  count.** With the row stratum judged (`.3k.2g`), `replay-constraints` is unchanged at 183 persisted /
  137 reproduced / 46 not-reproduced, and no admitted row clause in the 26 judgeable documents matches
  either predicate. The leaf ships on `.3k.1`'s footing — a live, demonstrable class with an empty
  published population — and the demonstration is the observed-RED control, run through the REAL row
  reader rather than a mirror:
  *"ZETARANGE must not be greater than the size indicated by the ZETAOAS field"* → `ZETARANGE
  must_be_stable, negated: true`, i.e. **"ZETARANGE must not be stable"** — the exact AMBA DTI
  fabrication `.3k.1` closed in the statement path — and *"OMEGABURST must be equal to the value of
  ZETAREADY"* → `OMEGABURST must_be_value VALUE`, the value lifted out of the phrase *the value of*.
  Both vanish with the refusal wired and both return when it is removed.
  **Honest limit, inherited not absorbed:** 51 of 78 documents have their table classifications
  neutralized on load (`[[legacy-source-classifications-are-neutralized-on-load]]`), so "no admitted row
  clause matches" is a statement about the 26 the producer can see. It is not a clean bill for the rest.
  Prerequisite: `.3k.2e` — **waived**, deliberately. `.3k.2e` refuses a clause whose kind is UNTYPED;
  this one refuses a clause whose kind types perfectly well and whose MEANING has no slot, which is why
  the `must_be_value VALUE` shape above is invisible to `.3k.2e` and survives it. The two are
  independent, and ordering them was an assumption `.3k.2g`'s re-sizing removed.
  Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2f`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2g` · Status: `done` (`2026-09-13`, CODE) · Goal: **the replay
  instrument judged two of the three deterministic producers and said nothing about the third.**
  `.3k.6` shipped `replay-constraints` so a change could be sized against what today's extractor
  actually mints, and `.3k.2a`–`.3k.2d` each used it — while it filtered the judged set to `sigcon_*`
  and `dyn_sigcon_*`. So `.3k.2c` had to state in prose that its retype *"is inside the row stratum,
  which the replay does not judge"*, and `.3k.2d` had to restore a held-out bundle and rebuild a whole
  document chain to prove that same stratum unmoved. An instrument a family sizes itself with cannot
  have a producer-shaped hole in it.
  Shipped: `replay_persisted_signal_constraints` takes the document's own `SourceIr` — which the
  artifact already names, repository-root-relative — and composes the table-row pass exactly as the
  build does (append after the statement paths, refine polarity BEFORE the dedup, then dedup against
  the established count), with the build's own catalog (statement declarations ∪ the
  signal-description tables' names). `row_sigcon_*` joins the judged set when and only when that pass
  actually ran.
  **The second half had to be measured rather than assumed, and it is the leaf's main result.** A
  LEGACY `SourceIr` loads, and `neutralize_legacy_source_classifications` sets every `table_kind` to
  `Unknown` — correctly, because only the current schema plus a verified proof ledger carries
  classification authority. The row producer selects tables by `TableKind::SignalDescription`, so over
  such an artifact it selects NONE and returns an empty result **indistinguishable from "this document
  states no row obligation"**. Measured on AMBA LTI: its persisted SourceIR marks 25 tables
  `signal_description`, and after a legacy load **0 of its 88 tables pass the producer's own gate**.
  Reporting that as a judged stratum would publish a silent zero — the exact failure `.3k.6` exists to
  retire. `SourceIr::carries_canonical_source_classifications` gates it, and the report publishes both
  halves: **26 documents judged, 51 not** (51 legacy SourceIRs; one document is the named EvidenceIR
  skip).
  **Corpus result: 171 → 183 persisted deterministic records, 125 → 137 reproduced, `not_reproduced`
  unchanged at 46.** All 12 published `row_sigcon_*` records reproduce — which re-derives, without a
  rebuild, exactly what `.3k.2d` had to rebuild APB to establish, and extends it to AHB and AXI-L.
  **This leaf's first act was to falsify a population `.3k.2d` published one commit earlier.**
  `.3k.2e` was opened saying the row path's untyped fallback is *"17 of 17 wrong"*, from a Python
  census that selected tables on the persisted `table_kind` field. The real producer selects on the
  LOADED one, and over the 26 judgeable documents it mints **12 row records and not one more** — zero
  on the untyped fallback, zero unpersisted. `.3k.2e` is re-sized in place; see its node.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2g`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2h` · Status: `pending` (opened `2026-09-12` by `.3k.2d`) · Goal:
  **the row path's two remaining readers of a narrower vocabulary.** (a) `obligation_subject` decides
  whether a description clause states an obligation by looking for `must`/`shall` only, so the modal
  gap `.3k.2d` closed for the statement path is still open one function along: a row cell stating
  *"X cannot be asserted while Y is high"* is `NotAnObligation` and the clause is dropped. (b) A
  signal-description table split across pages emits continuation rows whose NAME cell is empty, and
  `resolve_declared_signal_identifier("")` fails, so every obligation in the continuation is dropped —
  AMBA LTI `table_0014` (*"Table B4.1 Continued from previous page"*) loses two, including
  *"When LAMMUV is 1 and LAPM is 1, LAFLOW must not be Stall"*, the corpus's only genuine NEGATED VALUE
  binding and therefore the only clause that would have given `.3k.2d`'s original mechanism a
  population. Both are recall, both are in the row reader, and (b) decides whether the vocabulary slot
  `MustBeValue` + `negated` is ever reached at all.
  **SIZED `2026-09-13` against the real gate, and BOTH halves measure zero — so this leaf is blocked on
  re-ingest, not on a decision.** Over all **27** current-schema documents and the **102** tables that
  pass `should_treat_table_as_top_level_signal_description`, the probe finds **0** rows with an empty
  name cell and **0** description clauses stating an obligation with `cannot`/`will not`/`never` and no
  `must`/`shall`. The LTI `table_0014` instance that motivated (b) is real and is in a document whose
  `SourceIr` is schema 1, so the row producer cannot see it at all
  (`[[legacy-source-classifications-are-neutralized-on-load]]`). Neither half is refuted; both are
  **unmeasurable** until re-ingest reaches those documents, and shipping either now would be a rule with
  no reachable instance and no demonstrated class — weaker footing than `.3k.1`'s, which had one.
  Prerequisite: the owning document's re-ingest (`CORPUS-CHAIN-CURRENCY`). Verification: re-size both
  halves with the same probe once the judged-document count moves; each half adjudicated; observed RED
  on a demonstrated instance, not an invented one.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2i` · Status: `pending` (opened `2026-09-12` by `.3k.2d`) · Goal:
  **the row path's generic-value arm, read.** The same census that sized `.3k.2e` found 9 admitted row
  clauses reaching the `generic_value` arm, read as 4 right / 5 wrong. **Both numbers are withdrawn as
  CURRENT by `.3k.2g`** for the same reason `.3k.2e`'s are: the census selected tables on the persisted
  `table_kind` and the producer selects on the loaded one, so none of the 9 is reachable today. They are
  the reading for 51 legacy documents whose classifications this build has not re-derived, and the leaf
  must re-derive its own population with `replay-constraints` (row stratum judged) before it ships. Right: `LAPM must be 0`,
  `LAPRIV must be 0`, `LRHWATTR must be 0`, `LRMECID must be 0`. Wrong, and each for its own reason:
  *"LAPAS must be Non-secure or Secure"* publishes `NON` — the value binder splits at the hyphen, and
  the clause is a DISJUNCTION the slot cannot hold either (×2); *"LRATTR must be Snoopable Write-Back"*
  publishes `SNOOPABLE`, the first word of a two-word value; *"One write response must be sent for each
  write command"* publishes `SENT`, a passive verb `.3k.2b`'s participle rule misses because `sent` does
  not end in `ed`; *"… LRATTR must match LAATTR, with the exception of the allocation hint which must be
  Allocate …"* publishes `ALLOCATE`, a value lifted from a different clause of the same sentence
  (`.3k.3`'s span defect, in the row path).
  **Re-derived `2026-09-13`: the row reader mints 12 records over the whole judgeable corpus and every
  one is already published, so this arm's actionable population is 0 as well.** The nine readings stand
  as the estimate for the 51 legacy documents, which is where all nine live; the leaf is blocked on the
  same re-ingest `.3k.2h` names, not on a decision.
  Prerequisite: the owning documents' re-ingest (`CORPUS-CHAIN-CURRENCY`). Verification: each admitted
  clause adjudicated; observed RED per rule; the corpus delta measured with the row stratum judged
  (`.3k.2g`).
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2c` · Status: `done` (`2026-09-12`, CODE) · Goal: **the spelling
  this corpus uses for a no-change obligation.** APB writes it as *"PAUSER must have the same value in
  the Setup and Access phase of a transfer"* and *"… in every cycle during the Access phase"*; the
  phrase table knew `must be stable`, `must remain stable`, `must hold` and nothing of this form, so
  4 `row_sigcon_*` records reached the untyped fallback and were published as `must_be_stable` by
  accident. They are the only reason `.3k.2a` had to leave the row path's fallback in place.
  **Typed as `MustNotChange`**, because that is what the sentence says: the value is the SAME across
  two phases, or across every cycle of one — it does not change. The table already reads *"must
  remain stable"* (the same obligation over time) that way.
  **The placement is the load-bearing decision, and it was measured.** A serialized signal-description
  cell routinely carries BOTH obligations — *"• PAUSER must be valid when PSELx is asserted. • PAUSER
  must have the same value …"* — and the first arm to match types the whole record. Ahead of the
  validity arm, this phrase retyped APB `sigcon_0009`/`0010` from `must_be_value VALID` to
  `must_not_change`, losing a fact the document states. Behind it, it fires exactly where nothing else
  matched. Both the phrase and its position carry their own RED control.
  **Measured: 4 records retyped, 0 added, 0 removed, one document** — APB `row_sigcon_0018`/`0019`/
  `0021`/`0022`, `must_be_stable` → `must_not_change`; `sigcon_0009`/`0010` keep `must_be_value VALID`.
  The 56 corpus statements containing *"have the same value"* are almost all descriptive
  (*"implementations that have the same value"*, *"It does not have the same value"*) and are untouched
  because the phrases carry their modal.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2c`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2k` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.3`'s addition measurement) · Goal: **a predicate that states a RELATION is not a value.**
  `.3k.2b` asked the neighbouring question about the same slot — value or the obligation's VERB — and
  refused the passive participle. This is the other way the slot fills with something that is not a
  value: a predicate ADJECTIVE whose truth is not about the subject alone. NVMe states *"The ANA Group
  Identifier (ANAGRPID) for each ANA Group shall be unique within the NVM subsystem"*; the generic arm
  lifts `unique` and publishes `must_be_value UNIQUE`, and there is no state `UNIQUE` a signal equals.
  It is `.3d`'s and `.3k.1`'s class one relation along — an inter-operand EQUALITY and a comparative
  MAGNITUDE against a reference operand — and the constraint vocabulary has no slot for any of the
  three. An honest residual keeps the statement counted as an uncaptured normative statement; a
  fabricated value does not.
  **Why it is opened here rather than inside `.3k.3`, and why now.** The container's ordering rationale
  says `.3k.2` lands before `.3k.3` *"because narrowing the kind's span moves 3 of its 4 records onto
  the ungated `generic_value` arm (NVMe would publish `ANAGRPID must_be_value UNIQUE`)"*. `.3k.3`'s
  addition measurement ran the real producer under the narrowed span and that prediction came true
  exactly: NVMe gains `ANA must_be_value` and `ANAGRPID must_be_value`. `.3k.2b` shipped the gate that
  was supposed to stop this and its rule — refuse a PAST PARTICIPLE — does not reach an adjective. So
  the arm is not finished, and `.3k.3` cannot land on top of it.
  **Actionable population today: ZERO, and the leaf says so rather than implying coverage.** Over all
  78 persisted artifacts, no judged `must_be_value` record carries a relational value: the census of
  every published deterministic `must_be_value` value is `0`/`1`/`5`/`12`/`0B01`/`0B11`/`VALID`/`LOW`/
  `NO`/`SET`/`PACKED`/`INVALID`/`INVALIDATED`/`UPDATED`. The class is nevertheless live and
  demonstrable through the REAL producer on a REAL corpus sentence, which is the `.3k.1` footing
  `.3k.2e`/`.3k.2f` also shipped on: hand `extract_normative_signal_constraints` NVMe
  `statement_7397`'s own first obligation clause and today's code returns `must_be_value UNIQUE`.
  **The discriminator is positional, not lexical (ADR 0006).** English marks the difference in the
  grammar rather than in the word: a state is complete at the predicate (`Invalid`, `LOW`, `0b01`),
  while a relation must name its second operand or its scope, and it does so with a preposition
  IMMEDIATELY after the predicate — `unique within <scope>`, `compatible with <other>`, `less than
  <other>`. No adjective list and no document vocabulary. `by` is deliberately NOT in the set: it
  marks an AGENT, not an operand, and the participles it follows are already `.3k.2b`'s.
  **Ordered after the three admissibility routes that `.3k.2b` measured**, so a document-declared
  value, a logic level and a numeric literal are untouched: the test can only ever fire in the final
  `everything else is admissible` branch `.3k.2b` left open.
  **It caught a fabrication one leaf back.** `.3k.1`'s control
  `a_magnitude_against_a_literal_still_yields_its_constraint` went RED, and it was right to: the record
  it pinned is `ZETARANGE must_be_value GREATER`, the comparative lifted into the value slot, with the
  literal `0` the sentence names nowhere in it. `.3k.1` described it as *"a value binding"*; it is not
  one, and the vocabulary has no `at least` kind either, so a magnitude against a literal has no more
  of a slot than a magnitude against a reference. The control now pins the property `.3k.1` actually
  owns — its own gate's verdict on a literal operand — and the amendment is written into `.3k.1`.
  **A control that asserts a record EXISTS pins whatever that record says**, fabrication included,
  which is how this one survived a leaf written to remove fabrications.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2k`
<!-- extraction-quality-gauge-task-source-region:untyped-default-and-generic-value-leaves:end -->
