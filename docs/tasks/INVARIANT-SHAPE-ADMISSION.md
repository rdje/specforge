# INVARIANT-SHAPE-ADMISSION: 739 captions are a precision defect; the table rows are an extraction gap, and they are not the same problem

## Metadata

- Tree ID: `INVARIANT-SHAPE-ADMISSION`
- Status: `active` (`2026-09-19`; `.0`-`.3`, `.5` and `.6a` done; `.4` is a program, not a slice; `.6b` is the production change)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-12`
- Last updated: `2026-09-19`
- Owner: repo-local workflow

## Goal

An `InvariantRecord` is meant to be a **normative constraint**, and it flows to IntentIR
`constraints` — the product boundary a downstream consumer is asked to enforce.

Measured over the proof-carrying stratum, **1,528 of 5,927 published IntentIR constraints (25.8%) are
not statements at all**: 769 are serialized markdown table rows and 759 are figure captions or
cross-references, up to and including a constraint whose entire text is `Figure 1.` That 25.8% is the
symptom, not the defect — see `.0`, which found two different causes underneath it.

Stop publishing markup as a requirement, without losing a requirement that happens to live in a table.

**`.0` measured the two halves and they are not one defect.** The 759 captions carry no requirement and
should never have been admitted. The 769 table rows mostly carry requirements found nowhere else — only
70 duplicate an existing declaration — so refusing them would be a recall loss, not a precision gain.
The tree therefore splits: `.1` refuses captions, `.2` reads table rows.

## How it was found

`ACTOR-NOUN-RELATION-DECLARATION.1` removed ADIv6's phantom signal `In` and 13 invariants vanished.
`ANCHORLESS-INVARIANT-DROP.0` went looking for an anchor bug and found an **admission gate** instead
(`is_invariant_like`, `crates/specforge/src/ir/semantic.rs`). Three routes admit a statement:

| route | condition | current stratum | of which not a statement |
| --- | --- | ---: | ---: |
| `r1` | a modal phrase — `must`, `shall`, `always`, `until`, … | 4,397 | 432 (412 table rows) |
| `r2` | **mentions a declared signal** *and* a weak phrase — `handshake`, `asserted`, `deasserted`, `transition`, `state`, `timing`, `observed` | 304 | 180 |
| `r3` | related visual evidence with a Normative or Ambiguous role | 1,155 | **910 (635 captions)** |

`r3` is the worst by a wide margin: **only 245 of its 1,155 admissions are prose.** A figure's caption
sits next to normative visual evidence by construction, so the route that trusts visual evidence
admits the caption along with it.

The adjudication is not a judgement call. Of the 759 published captions, **20** contain a modal verb;
the rest are `Figure 2-1: External debugger`, `Figure 6-1: Breakpoint event`, `Figure 1.` Of the 769
table rows, 414 do contain a modal — but the sample shows what they are:

```text
| AWVALID | 1 | - | Asserted high to indicate that the signals on the AWchannel are valid. |
| AWREADY | 1 | - | Asserted high to indicate that a transfer on the AWchannel can be accepted. |
```

Those are AXI **signal-description rows**, the same rows `synthesize_signal_declarations` already turns
into `Signal AWVALID is …` — published twice, once as a declaration and once as raw markup.

**`.0` measured how representative that sample is, and it is not.** Only **70** of the 769 table rows
have a first cell naming an already-declared signal. The other 699 carry content that exists nowhere
else in the artifact, which is why this tree splits rather than shipping one rule.

## Non-Goals

- Do not delete a requirement because it lives in a table. A specification states real obligations in
  tables, and a row's *content* may be normative even when its *serialization* is not. The defect is
  publishing the pipe-delimited row; recovering its content is a separate, larger question.
- Do not touch `r1`. A statement carrying `must` or `shall` is normative by the document's own grammar,
  and its 4,397 admissions are the tree's baseline of correct behaviour.
- Do not narrow the weak-phrase list as the fix. `state` admits 105 of `r2` alone, but `r2` is 304 of
  5,856 — the shape problem is four times larger in `r3` and would survive any phrase change.
- Do not read the two halves as one number. `.0`'s finding is that 739 captions and 699 table rows are
  a precision defect and an extraction gap respectively; summing them reads as one 25.8% problem and
  invites one rule, which would delete 699 real requirements.

## Acceptance Criteria

- The refusal is on **shape**, adjudicated corpus-wide, with every distinct form it removes sampled
  and no prose statement affected.
- The count of published constraints that are prose does not fall. This is a precision change and must
  cost no recall among real statements.
- Whatever a table row's content is worth, its disposition is **stated** — kept, transformed, or
  recorded as a residual — not silently dropped. `[[ANCHORLESS-INVARIANT-DROP]]` is the standing
  reminder that a silent drop is its own defect.
- Golds re-scored, not assumed; the chain rebuilt for every document whose artifacts move.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `INVARIANT-SHAPE-ADMISSION` · Status: `active` (`2026-09-19`) · Children: `.0`-`.6` (`.6a`, `.6b`)

- ID: `INVARIANT-SHAPE-ADMISSION.0` · Status: `done` (`2026-09-12`) · Goal: **decide where the shape is
  refused, and what happens to a table row's content.** Both decided; the second answer split the tree.
  Producer: `python3 scripts/measure_invariant_admission_shape.py`.
  Commit: `INVARIANT-SHAPE-ADMISSION.0`

- ID: `INVARIANT-SHAPE-ADMISSION.1` · Status: `done` (`2026-09-12`) · Goal: **a figure or table caption is not
  admitted by `r2` or `r3`.** The level is `is_invariant_like`, matching the idiom three other passes
  already use for a table row. The rule needs no second condition, because `r1` is tested first: the
  **20** captions that carry a modal verb are admitted by `r1` and are untouched, and the **739** that
  do not are exactly the population `r2` and `r3` admit.
  Population removed, measured: 104 (`r2`) + 635 (`r3`) = 739 of 5,856 current SemanticIR invariants.
  Prerequisite: `.0`. Verification: observed RED on a bare caption and GREEN on a modal-bearing one;
  the removed set sampled and adjudicated; no prose statement affected; golds re-scored; the chain
  rebuilt for every document whose artifacts move — which will be most of them, so budget for it.
  Shipped as `statement_is_a_caption`, refused between the modal route and the two weaker ones.
  Commit: `INVARIANT-SHAPE-ADMISSION.1`

- ID: `INVARIANT-SHAPE-ADMISSION.2` · Status: `done` (`2026-09-12`) · Goal: **read a table row instead of publishing
  it.** 699 of the 769 table-row constraints carry content found nowhere else, and the sample is
  normative: `| Secure | Must be zero |`, `| True | | When Burst type is WRAP, the transaction must be
  cache line sized and Modifiable. |`, `| BRESP_WIDTH | 0, 2, 3 | 2 | Width of BRESP in bits. Must be 3
  if … |`. This is an **extraction gap wearing a precision defect's clothes**: the requirement is real
  and the serialization is wrong.
  Census first — what shapes do these rows take, how many are a condition/value pair, a parameter
  constraint, a response-code definition? Only 70 duplicate a declaration the reader already made, so
  subtraction is not the answer for the other 699.
  Prerequisite: `.1` (it removes the caption noise this census would otherwise have to filter).
  **Answered: they cannot be read where they are published, and the readable part is small.** Result
  below; the work splits into `.3` (bounded) and `.4` (a program).
  Commit: `INVARIANT-SHAPE-ADMISSION.2`

- ID: `INVARIANT-SHAPE-ADMISSION.3` · Status: `done` (`2026-09-12`) · Goal: **a signal-description row whose
  description cell states an obligation about the signal that row declares is a `signal_constraint`.**
  The 17 rows reproduced exactly (APB 7, AXI 6, AHB 4) under `is_invariant_like`'s wide modal list; under
  the constraint extractor's own `must`/`shall` idiom they are 15 rows carrying **20 obligation clauses**,
  and all 20 were adjudicated individually rather than sampled.
  **Two of this leaf's premises were false and the result below says so**: the rows are not an untouched
  gap (9 of the 17 already produce a constraint, 3 of them fabricated), and *"no existing
  `signal_constraint` changes"* could not be the bar, because the same reading that admits the good rows
  is what names the bad ones. That half is `.5`.
  Producer: `python3 scripts/measure_signal_row_obligation_subject.py`.
  Prerequisite: `.2`. Verification: all 20 clauses adjudicated; observed RED; the three AMBA chains
  rebuilt from the evidence stage with their held-out bundles restored — **+12 constraints, −0, and the
  prose families unmoved**.
  Commit: `INVARIANT-SHAPE-ADMISSION.3`

- ID: `INVARIANT-SHAPE-ADMISSION.4` · Status: `pending` · Goal: **read a requirement matrix.** The
  remaining ~380 obligation-bearing rows are permission matrices, truth tables and parameter tables —
  `| Read* | Shareable | Permitted to hit | Must hit | … |`, `| 0 | 0 | Non-secure | Non-secure | 0 |
  Non-trusted request that must target a Non-secure PAS. |`. **This is a program, not a slice**, and
  the leaf exists so it is tracked rather than implied. Do not start it as a slice; scope it first,
  and consider whether an existing table-semantics tree should own it.
  Prerequisite: `.3`. Verification: scoping is the deliverable.

- ID: `INVARIANT-SHAPE-ADMISSION.6` · Status: `active` (opened `2026-09-19` by
  `BOUNDED-DECISION-PROVIDER.1a.1`) · Children: `.6a` (census, done), `.6b` (ship, open)
  · Goal: **ship the caption-admission repair that a rejected provider evaluation produced.**
  `BOUNDED-DECISION-PROVIDER` set out to buy a decision model for two defects, one of them this
  tree's; it measured a deterministic local repair instead and rejected the provider (ADR 0051). The
  repair is the deliverable, and it lands here because this tree owns `is_invariant_like`.
  **Three rules, all sentence shape, none a vocabulary list on its own.** **R1** a caption with no
  finite main clause is a title, so a deontic word inside it qualifies a noun. **R2** a sentence that
  OPENS with a figure/table label plus a reporting verb reports its referent — anchored to the
  opening, not to word order, because *"The bit combinations that Table 3-7 does not show, are not
  permitted"* is a real prohibition whose reporting verb sits in a relative clause. **R3**
  `(is|are) not permitted` and `no … (is|are) allowed` are deontic and route `r1` carries neither.
  Prerequisite: `.1` (it removed the caption noise these rules are measured against).

- ID: `INVARIANT-SHAPE-ADMISSION.6a` · Status: `done` (`2026-09-19`) · Goal: **adjudicate the
  corpus-wide selection before a line of production changes**, per this tree's standing rule that a
  cheap structural rule over-fires until someone reads what it selects.
  Producer: `python3 scripts/measure_caption_admission_repair.py`. Full record:
  `docs/research/caption-admission-repair-census.md`; fact card
  `[[caption-repair-corpus-selection]]`.
  **Over all 78 documents — 261,508 statements, 13,136 caption-shaped: 71 removals and 176
  additions**, every one printed rather than sampled.
  **The precision half costs no requirement.** 69 of the 71 removals carry none — a label, or a
  sentence whose main verb is `shows`/`lists`/`summarizes`. **Two do**: TileLink `1.7.1`/`1.8.0`
  caption *"Figure 3.1: Valid must be driven LOW for at least 100 cycles during reset"*, a finite
  clause R1's no-terminator proxy reads as a title. **Both documents state the same rule in prose**
  (*"Before deasserting reset, a valid, c valid, and e valid must be driven LOW by the master…"*),
  which route `r1` admits on `must`, so the requirement survives in both. The blind spot is recorded
  as a known limit of R1, not repaired: one distinct sentence in two editions is not a grammar.
  **The corpus narrowed R3 three times, and four documents could not have found any of them.**
  `was`/`were` dropped — its only two corpus rows are *"Prior to Issue G, … were not permitted"*,
  a superseded edition's rule. The negated-existential window tightened from `[^.]{0,80}` to
  `[^.,;:]{0,60}` — the wide form matched across a clause break in SMMU's *"No\_snoop == 1 flag, it
  indicates that the transaction **is allowed** to 'opt-out'"*, where the `no` is part of a signal
  name and the permission is GRANTED. And 38 of the 176 are serialized table rows, reported as their
  own stratum rather than refused: route `r1` already admits a table row carrying `must`, and `.0`
  measured 769 of them and kept them because 699 carry content found nowhere else.
  **Two residual classes are named rather than fixed**, because no shape rule separates them without
  refusing real requirements: ~4 revision-history entries (*"| Correction: Use of SnpDVMOp is not
  permitted |"*) and subjectless bullet continuations, whose subject is in the preceding bullet —
  this tree's separate subject question.
  **`BOUNDED-DECISION-PROVIDER.1a.1`'s producer is deliberately left untouched**: it is pinned
  evidence for a published score, and a score is not rewritten because a later census improved the
  rule it measured. A RED case asserts both rows its frozen set depends on still admit under the
  narrowed forms.
  Non-goal: any production change; that is `.6b`.
  Prerequisite: `.6`.
  Verification: see the `.6a` acceptance checklist below.
  Commit: `INVARIANT-SHAPE-ADMISSION.6a — read the whole corpus before changing the rule`

- ID: `INVARIANT-SHAPE-ADMISSION.6b` · Status: `pending` · Goal: **ship R1–R3 into
  `is_invariant_like`.** `.6a` adjudicated the selection; this is the production change and it is
  deliberately a separate slice because its cost is not the rule, it is the cascade.
  **What it owes, and none of it is optional:** the Rust change with its evidence-backed acceptance
  checklist; the cargo oracles the doctrine gate never runs; **the chain rebuilt for every document
  whose artifacts move — which `.1` warns will be most of them, so budget for it**; the wire golds
  re-scored rather than assumed; and the book updated where the admission contract is described.
  **Size the recall change before running it:** 176 admissions is the census figure, 138 of it prose
  and 38 serialized table rows entering on the existing footing — expect the published constraint
  count to move on most documents, and attribute the delta before the cascade, per ADR 0025.
  Prerequisite: `.6a`.
  Verification: `pending`
  Commit: `pending`

- ID: `INVARIANT-SHAPE-ADMISSION.5` · Status: `done` (`2026-09-12`) · Goal: **a serialized row's obligation must not be
  attributed to the row's name-cell signal when the clause binds to a different nominal.** Opened by `.3`'s
  adjudication, which found the statement paths already reading these rows — and getting three of them
  wrong. `is_post_passive_binding_only_subject` is the predicate that would refuse exactly this, and its
  **gate 2 exempts a table row** on the reasoning that *"a table row supplies subject context from its
  other cells"*. That is true for `RRESP` and false for `HBURST`, whose cell constrains `HBURST_WIDTH`.
  Measured population, adjudicated in full: **3 clauses** — `HBURST_WIDTH must be 0 or 3` →
  `dyn_sigcon_0013` `HBURST must_be_value 0`; `HPROT_WIDTH must be 0, 4, or 7` → `dyn_sigcon_0014`
  `HPROT must_be_value 0`; `Indicates which tags must be written to memory` → `WTAGUPDATE`. The first two
  are fabricated twice over: wrong subject, and a value that is one alternative of a set the document
  writes as `0 or 3`.
  `.3` already ships the predicate this needs (`obligation_subject`), so the leaf is a placement decision,
  not a new rule.
  **Shipped as a one-condition narrowing of gate 2, not a new predicate**, and the population is 4 rather
  than 3: the census found `WTAG` taking `WTAGUPDATE must be deasserted` twice in AXI-H, where the scan
  lifted a shorter declared name out of a longer identifier. AHB rebuilt: `signal_constraints` 16 → 14,
  and a **false temporal conflict went with them**.
  Producer: `python3 scripts/measure_table_row_foreign_subject.py`.
  Prerequisite: `.3`. Verification: all 4 adjudicated; observed RED; each removed record named
  individually, per this tree's standing residual rule.
  Commit: `INVARIANT-SHAPE-ADMISSION.5`

## `.5` — result (`2026-09-12`)

### The exemption was right for the shape it was written for, and wrong for one other

`is_post_passive_binding_only_subject` says in its own doc-comment that English binds a passive
obligation to a subject that PRECEDES the modal — which is `.3`'s rule, stated a year earlier for
prose. Its **gate 2** then exempts a table row outright:

```rust
// (2) a table row supplies subject context from its other cells → out of scope.
if text.trim_start().starts_with('|') { return false; }
```

That is true for `| RLAST | … | Must be HIGH |`, where the clause has no subject at all and only the
row's other cells can supply one. It is false for
`| HBURST | Subordinate | HBURST_WIDTH | … HBURST_WIDTH must be 0 or 3. |`, where the clause names a
subject and it is not the row's signal — and there the exemption hands the obligation to `HBURST`.

### Every one of the 351 persisted constraints is accounted for

`python3 scripts/measure_table_row_foreign_subject.py`, over all 78 documents:

| verdict | records | disposition |
| --- | ---: | --- |
| `not_a_table_row` | 240 | gate 2 never applied |
| `subject_precedes_the_lead` | 34 | gate 4 already keeps these |
| `common_noun_head` | 32 | `the LASECSID signal must be 0`, `This field shall be 0h` — a descriptor, not a subject |
| `active_obligation` | 19 | `Controller must set PREQ LOW` — gate 3 already keeps these |
| `not_plain_identifier` | 18 | gate 1 |
| `subjectless_clause` | **4** | **the exemption's real purpose — preserved** |
| `foreign_identifier_head` | **4** | **refused** |

The two bold rows are the whole change, and the first of them is why this is a narrowing rather than
a removal: `| RLAST | … | Must be HIGH |` and Intel VT-d's three `Must be 0` rows keep the exemption
they were written for.

### The first rule I wrote was wrong, and the census is what said so

The obvious rule — *refuse when the subject heads none of the row's obligation clauses* — refuses
**45** records. Adjudicating them showed it conflates three grammars the existing gates already
handle:

```text
the LASECSID signal must be 0     head `signal`      a descriptor; the identifier is adjacent
Controller must set PREQ LOW      head `Controller`  an ACTIVE obligation — gate 3 keeps it
This field shall be 0h            head `field`       the NVMe field-cell class (`.3e`)
```

Requiring the head to be an **uppercase-run identifier** — the same `[A-Z0-9_]` tokenization
`collect_subject_signal_tokens` uses, so the gate sees the spelling the extractor actually lifted —
keeps all three and refuses only the real mis-subjects. 45 → 4.

### Corpus effect, and a falsification the pipeline supplied on its own

AHB rebuilt from the evidence stage with its held-out bundle restored:

| | before | after |
| --- | ---: | ---: |
| EvidenceIR `signal_constraints` | 16 | **14** |
| IntentIR `signal_constraints` / `temporal_rules` | 16 | 14 |
| IntentIR `temporal_invariants` | 191 | 189 |
| IntentIR `actor_contracts` | 14 | 13 |
| lowered `.isf` rules | 37 | 35 |
| **IntentIR `temporal_conflicts`** | **1** | **0** |
| records added, any stage | | **0** |

The last two rows were not designed for. `temporal_conflict_0001` reported `HPROT` taking both `HIGH`
and `LOW` at the same `HCLK` rising edge, from `temporal_signal_constraint_dyn_sigcon_0014` — the
fabricated `HPROT must_be_value 0` — against `dyn_sigcon_0015`, the document's real
*"a Manager sets HPROT[0] HIGH, to indicate a data access"*. **The mis-subjected record was making AHB
look internally inconsistent with itself.** That is a dimensionally different falsification from the
one this leaf was built on: the census says the subject is wrong by grammar, and the temporal layer
says it is wrong by contradiction, independently.

### The two AXI-H records are NOT removed, and why

`llm_sigcon_0025`/`0027` sit in a schema-2 (legacy) artifact, which is inspection-only and is not
rebuilt. They are also `llm_sigcon_*`: `crates/specforge/src/ir/constraint_extract_llm.rs` applies
**none** of the `.2.50a`/`.3e`/`.3g`/`.3h` positional subject gates — it has its own catalog-grounding
gates only. So this producer change does not reach them by either route. Tracked as
`EXTRACTION-QUALITY-GAUGE.3j`; stated here rather than left to look like an unexplained miss.

### The dropped obligation's disposition, stated

`[[ANCHORLESS-INVARIANT-DROP]]` requires it. Nothing else in AHB cited those two rows, so removing the
records leaves their content carried only by the serialized rows themselves, which stay published as
invariants. That is correct rather than lossy: `HBURST_WIDTH must be 0 or 3` is a **parameter**
constraint, and `SignalConstraintKind` has no slot for one — an honest residual, not a lost
requirement. Reading it properly is `.4`'s matrix programme.

## Acceptance Checklist (enforced) — `.5`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_table_row_foreign_subject.py`: of 351
  persisted constraints, 4 head a foreign identifier and 4 are subjectless clauses the exemption
  exists for; every remaining record lands in a named verdict (240 not a table row, 34 subject before
  the lead, 32 common-noun head, 19 active, 18 non-identifier subject).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`
  `is_post_passive_binding_only_subject` gate 2: `if text.trim_start().starts_with('|') { return false; }`.
  The predicate's own doc-comment states `.3`'s rule for prose; gate 2 exempts every table row from it,
  so `extract_dynamic_signal_constraints`' whole-statement scan attributes
  `HBURST_WIDTH must be 0 or 3` to `HBURST` (`dyn_sigcon_0013`, persisted).
- [x] **ADDRESSED (verified)** — `obligation_head_is_a_foreign_identifier`, and gate 3 moved above
  gate 2 so the lead is computed once. AHB rebuilt: `signal_constraints` 16 → 14, `.isf` rules 37 → 35,
  `temporal_conflicts` 1 → 0, **0 records added at any stage**. The two removed are exactly the two the
  census named. **Observed RED**: with gate 2 restored to its blanket form,
  `a_row_whose_obligation_names_a_width_parameter_does_not_constrain_the_signal` fails — and only that
  test, so the four protected shapes are held by controls that do not depend on the refusal.
- [x] **NO REGRESSION** — `cargo test` green including the four new controls;
  `a_row_keeps_its_subject_context_for_every_shape_the_exemption_was_written_for` pins the subjectless,
  descriptor-head, field-cell and active-obligation shapes, and
  `prose_behaviour_is_unchanged_by_the_row_narrowing` pins both prose directions, since gate 2 never
  applied to prose. `cargo fmt --check` and `cargo clippy --all-targets -D warnings` green;
  `scripts/check_doctrines.sh` green. Retention stays at the declared 24: the AHB bundle was restored,
  `diff -r`-verified unchanged by the rebuild, and removed.
- [x] **GENERICITY (ADR 0006)** — the added condition is a tokenization, not a vocabulary: the head must
  be a single maximal `[A-Z0-9_]` run spanning the whole token, which is exactly what
  `collect_subject_signal_tokens` lifts. No document, protocol, vendor, or signal name appears in the
  rule, and the controls use invented names (`OMEGABURST`, `ZETAPROT`, `SIGMATAG`, `ZETASECSID`).
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md`'s "Currently only half-applied" callout,
  added by `.3`, is now false and is replaced by what actually shipped.
  `[[table-row-obligation-binds-to-the-token-before-its-modal]]` updated: the gate-2 narrowing is no
  longer "not yet applied", and the false temporal conflict is recorded as the independent
  confirmation. No production rule was deleted; gate 2 was narrowed, and the book says so.

## `.3` — result (`2026-09-12`)

### The 17 reproduce, and reading them clause by clause is what makes them adjudicable

`.2` recorded **17 rows** across APB (7), AXI (6) and AHB (4). That reproduces exactly — but only under
`is_invariant_like`'s **wide** modal list, which is an *admission* list, not an obligation list. Two of
the 17 are admitted by `required` alone and state no obligation at all:

```text
| AWLOCK, ARLOCK | 1 | 0b0 | Asserted high to indicate that an exclusive access is required. |
| PSELx | Requester | 1 | Select. … PSELx indicates that the Completer is selected and that a data
                          transfer is required. |
```

Under the constraint extractor's own idiom (`constraint_bearing_sentence`: `must`/`shall`) the
population is **15 rows carrying 20 obligation clauses**. Clauses, not rows, is the unit that can be
adjudicated, because a cell states up to three obligations and the serialized statement keeps only the
first — which is why APB's three-bullet `PAUSER`/`PWUSER` rows published one constraint each before this
leaf and three after.

### All 20, by what heads the obligation

`python3 scripts/measure_signal_row_obligation_subject.py`, over 567 declared signal-description rows in
the 27 proof-carrying documents:

| head | clauses | verdict |
| --- | ---: | --- |
| `absent` — the modal opens the clause | 1 | **admit**: only the row's header can supply the subject |
| `self` — the row's own declared signal | 11 | **admit** |
| `pronoun` — `it` / `that` | 5 | refuse: the referent needs anaphora, not a rule |
| `other` — a different nominal | 3 | refuse: the obligation is about something else |

Admissible: **12**. The rule is one line of English — *an obligation binds to the nominal immediately
preceding its modal* — and it is the sharpened form of a principle the repository already states, in
`is_post_passive_binding_only_subject`'s own doc-comment.

The pronoun refusal is not timidity. **Four of the five mean the signal and one does not**, and nothing
in the clause separates them:

```text
| HWRITE | … It has the same timing as the address signals, however, it must remain constant … |   → HWRITE
| HSELx  | … When the Subordinate is initially selected, it must also monitor the status of HREADY … | → the Subordinate
```

Both cells are signal-description rows, both heads are `it`, and only the document's meaning separates
them — no property of the clause does. Admitting all five would buy four real obligations at the price of
one fabricated one, and this leaf's stated bar is precision over a population small enough to adjudicate.
So all five stay residual: the rows remain published (`.2`), their text is intact, and `.4`'s
table-semantics programme is where anaphora would belong if it is ever worth resolving.

### The premise that was false: these rows are already being read, and three readings are wrong

`.3` was scoped as a recall gap. Measured, **9 of the 17 rows already produce a `signal_constraint`** —
the statement paths reach the serialized row and scan the whole of it for a declared-signal subject.
Five of those readings are right *by accident of the cell repeating its own name* (`PNSE must be valid
when PSEL is asserted`). Three are wrong:

| persisted record | the cell actually says | the obligation is about |
| --- | --- | --- |
| `dyn_sigcon_0013` `HBURST must_be_value 0` | `HBURST_WIDTH must be 0 or 3` | the width parameter |
| `dyn_sigcon_0014` `HPROT must_be_value 0` | `HPROT_WIDTH must be 0, 4, or 7` | the width parameter |
| `sigcon_0002` `HSELx must_be_asserted` | condition lifted from the *preceding* sentence | right kind, wrong condition |

The first two are fabricated twice over: wrong subject, and a value that is one alternative of a set.

**So the leaf's stated bar — "no existing `signal_constraint` changes" — could not survive contact with
the population.** The same reading that admits `RRESP` is what identifies `HBURST` as mis-subjected.
Splitting rather than widening: `.3` ships the admission, `.5` owns the refusal.

### And the loss the same blindness causes

`| RRESP | RRESP_WIDTH | 0b000 (OKAY) | … Must be valid when RVALID is asserted. |` is classified
`signal_value_constraint`, reaches the extractor, and yields **nothing**. After narrowing to the
obligation and cutting the condition, the subject scan is left with `Must be valid` — whose three words
all pass the permissive `is_hardware_signal_token` identifier test, so `subject_signals` is non-empty and
the full-text fallback never runs; then all three are dropped as undeclared. A subjectless obligation
defeats a subject scan by having no subject to find, which is `.2`'s thesis in mechanical form.

### Corpus effect: +12 constraints, −0, exactly the 12 the census predicted

The three documents were rebuilt from the evidence stage down. The **12 admissible clauses became 12
records and nothing else moved** — the prediction was made from the census before the rebuild and the
rebuild confirmed it rather than being used to find it.

| | before | after | delta |
| --- | ---: | ---: | ---: |
| EvidenceIR `signal_constraints` (APB 15→25, AXI 44→45, AHB 15→16) | 74 | **86** | **+12** |
| IntentIR `signal_constraints` | 73 | 85 | +12 |
| IntentIR `temporal_invariants` | 1,171 | 1,183 | +12 |
| IntentIR `actor_contracts` | 49 | 53 | +4 |
| lowered `.isf` rules (APB 32→51, AXI 110→112, AHB 35→37) | 177 | **200** | +23 |
| SemanticIR `invariants` — **the prose family** | 1,098 | **1,098** | **0** |
| IntentIR `constraints` — **the prose family** | 1,117 | **1,117** | **0** |
| records removed, any stage | | | **0** |

The two zero rows are this tree's acceptance criterion discharged: *"the count of published constraints
that are prose does not fall"*. Nothing was deleted anywhere, which is the other one.

All 12 records carry the document's own words as `source_text` and cite the serialized row statement
they came from, so the residual-honesty accounting counts that statement as captured rather than as an
uncaptured normative miss.

### Three limits, each stated rather than implied

**1. No new `.isf` FILE is emitted, and none was before.** All three documents stay
`is_renderable: false` with the same single blocking reason they already had — *"no source-grounded
system clock/reset contract"* — so the +23 rules are inside the adapter's lowered text and its residual
decisions, not in an emitted artifact. That is unchanged behaviour, not a new block.

**2. One of the 12 carries an imprecise KIND.** `PSTRB must not be active during a read transfer`
classifies as `must_be_stable` with `negated: true`, which reads as *"PSTRB must not be stable"* — not
what the document says. The cause is the shared classifier, not this leaf's rule: its generic arm
returns `MustBeStable` as a **default when nothing matched**, and the independently-computed negation
is then stacked on that default. It is a **pre-existing corpus shape** — 9 `must_be_stable + negated`
records existed before this change, including AHB's own `HSIZE must not be changed` — so this is one
more instance of a known class, not a new one. Keeping it is deliberate: the obligation is real, its
`source_text` states it verbatim, and dropping it would be the silent loss
`[[ANCHORLESS-INVARIANT-DROP]]` warns against. Owned by `EXTRACTION-QUALITY-GAUGE.3i`, opened here with
the population measured (9 records, adjudicable in full).

**3. The AMBA chains needed their held-out bundles restored, and this leaf nearly concluded they could
not be rebuilt at all.** APB, AXI and AHB are the three proof-carrying documents with no
`normalized/` directory, and `specforge evidence` reports only
`path does not exist: …/normalized/<key>.md` — which names the missing input, not the place it is kept.
The bundles are in `generated/preserved/WIRE-BASED-100.10/`, the procedure is
`[[evidence-rule-field-content-stales-every-proof]]`, and **the Knowledge Map already answered this
under a question this leaf did not think to ask.** Restored, rebuilt, `diff -r`-verified byte-identical
and removed again; the declared retained population stays at exactly 24.

## Acceptance Checklist (enforced) — `.3`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_signal_row_obligation_subject.py`: 20 obligation
  clauses over 567 declared signal-description rows in the 27 proof-carrying documents — `absent` 1,
  `self` 11, `pronoun` 5, `other` 3. `.2`'s 17 rows reproduce exactly under the wide modal list
  (APB 7 / AXI 6 / AHB 4) and are 15 rows under `must`/`shall`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. Two mechanisms, both located:
  `extract_signal_constraints` finds a subject by scanning the clause, so a **subjectless** obligation
  (`Must be valid when RVALID is asserted`) yields `Must`/`be`/`valid` — all identifiers under
  `is_hardware_signal_token` — which suppresses the full-text fallback and then drops all three as
  undeclared (RRESP: 0 records from a statement classed `signal_value_constraint`). And
  `is_post_passive_binding_only_subject` gate 2 exempts a table row, so the whole-statement scan in
  `extract_dynamic_signal_constraints` attributes `HBURST_WIDTH must be 0 or 3` to `HBURST`
  (`dyn_sigcon_0013`, persisted).
- [x] **ADDRESSED (verified)** — `obligation_subject` + `extract_signal_description_row_constraints`.
  Control `a_row_states_a_constraint_only_when_its_clause_binds_to_that_row_signal` admits the
  subjectless and self-named rows and refuses the `_WIDTH` and pronoun ones;
  `each_obligation_clause_in_a_cell_becomes_its_own_record` pins one record per obligation (2 from a
  two-bullet cell, where the serialized statement yields 1);
  `an_undeclared_name_cell_yields_no_constraint` pins the catalog gate.
  **Observed RED**: with `ObligationSubject::Head(_) => continue` removed, the first control fails and
  emits `OMEGABURST must_be_value "0"` from `OMEGABURST_WIDTH must be 0 or 3` — the fixture reproduces
  `dyn_sigcon_0013` exactly, so the control is measuring the live defect and not a toy.
  **Corpus, rebuilt**: the three AMBA chains rebuilt `evidence → validate → semantic → validate → intent
  → validate → adapt`, zero failures. EvidenceIR `signal_constraints` 74 → **86** (+12, −0), IntentIR
  `signal_constraints` 73 → 85, `temporal_invariants` 1,171 → 1,183, `actor_contracts` 49 → 53, lowered
  `.isf` rules 177 → 200. **SemanticIR `invariants` 1,098 → 1,098 and IntentIR `constraints` 1,117 →
  1,117** — the prose families did not move, which is this tree's precision criterion. The 12 records are
  exactly the 12 clauses the census called admissible, predicted before the rebuild.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1427** / 4 green (1423 → 1427 = the four new
  controls), after re-pinning the production-graph census (`PRODUCTION-GRAPH-CENSUS-PIN`: analyzed
  functions 2,401 → 2,407, helper edges 14,951 → 14,983, decision sites 12,951 → 12,980, semantic macros
  1,469 → 1,471 — the **fourth consecutive** slice to move it by one predicate). `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` green. `scripts/check_chain_currency.sh`: the evidence stage
  replayed for the **24 replayable documents, 24 current, 0 stale** — the new pass emits nothing on any
  document outside the three, exactly as the census predicted. The three AMBA chains were rebuilt rather
  than replayed, and they **loaded cleanly at HEAD before this change** (probed per document with the
  HEAD binary), so their staleness is this change adding content to those three artifacts and nothing
  else. Held-out bundles restored, `diff -r`-verified unchanged by the rebuild, and removed: retention
  stays at the declared 24. Every new fidelity finding is the standard per-contract gate row; the one
  failing gate (`realizable_boundary`, the clock not being on the actor boundary) already failed for
  **every** pre-existing contract in these documents (APB 12/12, AXI 23/23), so the new contracts join a
  universal pre-existing disposition rather than introducing a failure.
  The classifier extraction (`classify_signal_constraint_kind` / `obligation_is_negated` lifted verbatim
  out of `extract_signal_constraints`) is behaviour-preserving and is covered by the frozen
  `extract_normative_signal_constraints` contract cases, which are unchanged.
- [x] **GENERICITY (ADR 0006)** — pure grammar: the token before the modal, a closed helper list, and a
  closed pronoun-refusal (a pronoun is *reported* as a head, never resolved). No document, protocol,
  vendor, signal, or value name appears in the rule or the fixtures — the controls use invented names
  (`ZETAREADY`, `OMEGABURST`, `SIGMASTRB`, `ALPHACHUNK`) so a fixture cannot become a name list.
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md` gains "An obligation in a table cell belongs
  to whatever precedes its modal", with both admissions, both refusals, and the reason the pronoun case
  stays residual. Fact card `[[table-row-obligation-binds-to-the-token-before-its-modal]]` carries the
  rule and the three live mis-attributions;
  `[[evidence-rule-field-content-stales-every-proof]]` gains two question keys so the
  `path does not exist: …/normalized/<key>.md` error leads to the remedy it already documents — the
  discovery failure this leaf actually hit. No production rule was deleted or replaced, so no book text
  became false.

## `.2` — result (`2026-09-12`)

### A table row is a matrix row, and serialization threw away what reads it

Across the 769 table-row constraints, **396 carry a modal verb** and are therefore real obligations.
But almost none is a sentence. The obligation is a **cell**, and its subject comes from the row's other
cells and from the table's header:

```text
| Read*      | Shareable | Permitted to hit | Must hit         | Permitted to hit | Must hit |
| 0 | 0 | Non-secure | Non-secure | 0 | Non-trusted request that must target a Non-secure PAS. |
| BRESP_WIDTH | 0, 2, 3 | 2 | Width of BRESP in bits. Must be 3 if: Untranslated_Transactions = v2 … |
| Non-secure | Must be zero |
```

The codebase already says this in another pass: `is_post_passive_binding_only_subject` skips a row
because *"a table row supplies subject context from its other cells"*. **The published statement does
not carry the header**, so the subject is unrecoverable at the level where these constraints exist. Any
real reading has to happen where the table is still a table — in EvidenceIR from
`StructuredTableRecord`, the same place `synthesize_signal_declarations` already works.

By first cell, the 769 split: 438 phrase, 109 number or range, 100 identifier, 70 an already-declared
signal, 30 an encoded value, 22 empty (continuation rows).

### The readable part is smaller than it looks

The natural first increment is the one shape the reader already understands end to end: a
signal-description row for a signal it has just declared, whose description cell states an obligation
about that signal. **17 rows corpus-wide**, against 84 existing `signal_constraints` — a real but
modest gain, and small enough that all 17 can be adjudicated rather than sampled. That is `.3`.

The other ~380 are matrices, and reading them is a table-semantics programme rather than a rule. `.4`
exists to hold it, explicitly not as a slice.

### Meanwhile, the rows stay published — deliberately

`.1` removed captions because they state nothing. These state something, so removing them would lose
it, and `[[ANCHORLESS-INVARIANT-DROP]]` is the standing reminder that a silent drop is its own defect.
They are also not currently reaching a machine consumer: the ISF adapter reports 27 blocked states and
0 emitted files, so today these records serve audit, where a serialized row is legible to a person.
**Stated rather than left implicit** — that is what this tree's acceptance criteria asked for, and it
is discharged here for the table-row half.

## Acceptance Checklist (enforced) — `.1`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_invariant_admission_shape.py`: 759 of 5,927
  published IntentIR constraints are figure or table captions, 422 of them bare labels.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs` `is_invariant_like`: route 2
  admits a caption for carrying a weak phrase such as `state` beside a declared signal, and route 3
  admits it because a caption sits beside the normative figure it names.
  `cargo test -p specforge-core --lib a_caption_is_not_an_invariant_unless_it_states_an_obligation`
  reproduces route 2 exactly on `"Figure 3-1: HCLK debug state entry and exit"`.
- [x] **ADDRESSED (verified)** — all 27 proof-carrying documents rebuilt from `semantic` down, zero
  failures. SemanticIR invariants 5,856 → 5,117; IntentIR constraints 5,927 → 5,188; **every removed
  record is a caption**, prose unchanged at 4,399, table rows unchanged at 769, nothing added. The 20
  modal-bearing captions survive via route 1, as designed before the rebuild.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1423** / 4 green (the last after re-pinning the
  production-graph census, `PRODUCTION-GRAPH-CENSUS-PIN` — third consecutive slice to move it by one
  predicate, which answers that tree's open question); `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` green; `scripts/check_doctrines.sh` green;
  `scripts/check_chain_currency.sh` current across all four stages, retention at the declared 24. All
  eight wire golds re-scored byte-identically — **stated as corroboration, not proof**: they score
  EvidenceIR facts and this change is at the semantic stage.
- [x] **GENERICITY (ADR 0006)** — `Figure`/`Table` plus a label number is document-structure grammar,
  the same class as the `property` word `WIRE-BASED-100.10e` reads. The control pins seven near-misses
  (`Table`, `Figure `, `Tables are used…`, `TableOfContents`, a markdown row) so the test cannot widen
  into vocabulary.
- [x] **LOCKSTEP** — `docs/book/src/pipeline/semanticir.md` gains "A figure caption is evidence, not a
  requirement", matching that chapter's standing pattern of naming what the stage refuses to promote;
  it states the rule, why placement after the modal route is the design, and the rebuilt numbers. No
  production rule was deleted or replaced, so no book text became false.

## Acceptance Checklist (enforced) — `INVARIANT-SHAPE-ADMISSION.6a`

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_caption_admission_repair.py` over all 78
  persisted documents: **261,508 statements, 13,136 caption-shaped, 71 removals (15 title / 56
  cross-reference), 176 additions (168 `is/are not permitted`, 8 `no … is/are allowed`, 38 of them
  serialized table rows, 138 prose, 27 documents, 129 distinct texts)**. `--rows` prints every moved
  row; nothing is sampled.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs`, `is_invariant_like` route
  `r1`: `contains_any_phrase` over 12 phrases, tested before `statement_is_a_caption`. It fires on a
  deontic **word** wherever it sits and misses a deontic **clause** it does not list. The 71 and the
  176 are the two faces of that one shape, measured corpus-wide.
- [x] **ADDRESSED (verified)** — the selection is adjudicated, not assumed: the precision half costs
  **no requirement** (the two captions that do state one are duplicated in prose the pipeline already
  admits, checked per document), and the recall rule is **narrowed three times on corpus evidence**
  — `was`/`were` dropped, the negated-existential window tightened past a clause break, and table
  rows separated as their own stratum rather than refused.
- [x] **NO REGRESSION** — **no Rust, fixture, artifact, gold, seal or `.isf` is touched**; the slice
  adds one read-only producer, one research record and one fact card, so no score can move.
  `--self-test` **12/12 RED cases**, and one was observed failing on a known-bad input: restoring
  `was|were` trips `past-tense-is-not-a-current-prohibition` plus two census pins, and the producer
  was restored byte-identically. `BOUNDED-DECISION-PROVIDER.1a.1`'s producer is untouched and still
  reports 11/11, so its published scores stand.
- [x] **GENERICITY (ADR 0006)** — R1 and R2 are sentence shape; R3 is a two-form deontic grammar
  whose every corpus admission was read. No document, vendor or protocol name enters any rule; the
  verbatim corpus text lives only in the adjudication evidence, which is where document text belongs.
- [x] **LOCKSTEP** — no user-visible behaviour changes because nothing is shipped, so the book is
  unchanged by the producer sub-clause. The durable surfaces are
  `docs/research/caption-admission-repair-census.md` and `[[caption-repair-corpus-selection]]`. The
  production change is **owned rather than implied**: `.6b`, with the cascade cost stated up front.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `INVARIANT-SHAPE-ADMISSION.6b` — **ship the caption repair.** `.6a` adjudicated the corpus-wide
   selection, so what remains is the production change: the Rust edit, the cargo oracles, the chain
   rebuilt for every document whose artifacts move — `.1` warns that is most of them — the wire golds
   re-scored rather than assumed, and the book. Size the recall change first: 176 admissions, 138 of
   them prose, and attribute the delta before the cascade per ADR 0025.
2. `INVARIANT-SHAPE-ADMISSION.4` — scope the matrix reader. **Not a slice.** What remains in this
   tree besides `.6b` is the ~380-row matrix programme, and the first thing `.4` owes is a decision
   about whether an existing table-semantics tree should own it.

## `.0` — result (`2026-09-12`)

### The level: `is_invariant_like`, and the repository already chose it

A statement beginning with `|` is already treated as a table row rather than prose in **three** places:
`extract_actor_signal_relations` ("Skip synthesized declarations and table rows"),
`is_post_passive_binding_only_subject` ("a table row supplies subject context from its other cells →
out of scope"), and `is_standalone_markdown_block`. The idiom exists; `is_invariant_like` simply does
not use it.

The two more upstream candidates are refused on measurement, not taste. Stopping a table row becoming a
statement at all would break the three passes above, which depend on those statements existing and
handle them deliberately. A `StatementClass` change is a schema change for a filtering problem.

### Captions: refuse, and `r1` already protects what matters

| | captions | admitted by |
| --- | ---: | --- |
| bare labels (≤70 chars, e.g. `Figure 1.`, `Figure 2-1: External debugger`) | 422 | `r2`/`r3` |
| cross-reference sentences with no modal (`Figure 3-4 shows a write transfer with one wait state.`) | 317 | `r2`/`r3` |
| **carrying a modal verb** (`Table A8.2: Opcodes which must be cache line sized and Regular`) | **20** | **`r1`** |

The split is exact and it is not a coincidence: `is_invariant_like` tests the modal route first, so a
caption that states a requirement is already admitted by `r1` and a rule confined to `r2`/`r3` cannot
reach it. **739 removed, 20 kept, no second condition needed.**

### Table rows: do NOT refuse — this half is an extraction gap

The opening framing of this tree said the table rows are "published twice, once as a declaration and
once as markup". **Measured, that is true of 70 of 769.** For the other 699 the first cell is not an
already-declared signal, and the content is real:

```text
| Secure | Must be zero |
| True | | When Burst type is WRAP, the transaction must be cache line sized and Modifiable. |
| BRESP_WIDTH | 0, 2, 3 | 2 | Width of BRESP in bits. Must be 3 if: Untranslated_Transactions = … |
| 0b100 | DEFER | Write was unsuccessful because it cannot be serviced at this time. … |
```

Deleting those is a recall loss. The requirement is real and only its serialization is wrong, so the
work is to **read the row**, which is `.2` and is a different and larger piece of work than `.1`.

**This corrects this tree's own headline.** "A quarter of published constraints are captions and table
rows" is arithmetically right and analytically wrong: it adds a 739-record precision defect to a
699-record extraction opportunity and calls the sum one number.

## `.1` — result (`2026-09-12`)

`statement_is_a_caption` in `is_invariant_like`, placed **between the modal route and the two weaker
routes**. That placement is the rule: a caption stating an obligation is already admitted by `r1` and
never reaches the test, so no second condition is needed to protect it.

### Corpus effect, measured on rebuilt artifacts

All 27 proof-carrying documents were rebuilt from the semantic stage down — `semantic`, `validate`,
`intent`, `validate`, `adapt` per document, in the interleaved order, zero failures.

| | before | after | delta |
| --- | ---: | ---: | ---: |
| SemanticIR invariants | 5,856 | 5,117 | **−739** |
| IntentIR constraints | 5,927 | 5,188 | −739 |
| … of which **prose** | 4,399 | **4,399** | **0** |
| … of which **table rows** | 769 | **769** | **0** |
| … of which captions | 759 | **20** | −739 |

**Every removed record is a caption. No prose statement and no table row moved, and nothing was
added.** The 20 surviving captions are exactly the modal-bearing ones, as the design predicted — that
prediction was made before the rebuild and the rebuild confirmed it rather than being used to find it.

### The limits, stated

- **The wire golds could not have moved.** All eight re-score byte-identically to the pre-change run,
  but they score EvidenceIR-level facts and this change is at the semantic stage. That is corroboration
  that nothing upstream shifted, not evidence about the change itself. The evidence is the artifact
  diff and the chain oracle.
- **The 739 are gone with no residual.** `[[ANCHORLESS-INVARIANT-DROP]]` established that a record
  leaving the artifact unrecorded is its own defect, and this tree's acceptance criteria carry that
  forward. It is not discharged here: a caption states nothing, so there is nothing to residualise —
  but the general requirement stands for `.2`, where real content is at stake.

## Decisions

- `2026-09-12` — **opened rather than folded into `ANCHORLESS-INVARIANT-DROP`.** That tree asked how a
  dropped invariant should be accounted for and its premise was wrong; this is a distinct, larger, and
  correctly-premised defect found while disproving it.
- `2026-09-12` — **the two halves get different treatment, and that is the leaf's main finding.** They
  were opened as one defect because they share a symptom — a published constraint that is not a
  statement. They do not share a cause: one is admission noise, the other is a reader that does not yet
  exist. A single rule over both would have deleted 699 real requirements.
- `2026-09-12` — **the first leaf decides a level, not a rule.** The shape test is trivial — a leading
  `|`, or a leading `Figure`/`Table` label plus a number. What is not trivial is which of three places
  should carry it, and the repository's recent history is that the level decides whether a fix repairs
  a cause or merely changes how a defect looks.

## Open Questions

- ~~What is a signal-description table row worth once the declaration reader has consumed it?~~
  **Answered: that describes only 70 of 769.** The question that matters is `.2`'s.
- Is `r3` defensible at all? A statement admitted purely because it sits near a normative figure yields
  245 prose invariants and 910 non-statements. If the prose 245 are themselves weak, the route is worth
  re-deriving rather than filtering.


## Blockers

None.

## Verification Log

- `2026-09-12` — `.5`. All **4** foreign-identifier records adjudicated individually, and the other 347
  persisted constraints accounted for by named verdict — `python3 scripts/measure_table_row_foreign_subject.py`.
  The first rule drafted refused **45**; the 41 false positives were read by hand and produced the
  uppercase-run-identifier condition, so the shipped rule is the adjudicated one rather than the first
  one that looked right.
  **Observed RED**: with gate 2 restored to its blanket form,
  `a_row_whose_obligation_names_a_width_parameter_does_not_constrain_the_signal` fails on all three of
  its cases while the other three controls stay green.
  `cargo test` green (+4 controls); `cargo fmt --check` and `cargo clippy --all-targets -D warnings`
  green; `scripts/check_doctrines.sh` green.
  Chain: AHB rebuilt `evidence → validate → semantic → validate → intent → validate → adapt`, zero
  failures, bundle restored from `generated/preserved/WIRE-BASED-100.10/`, `diff -r`-verified unchanged
  and removed; retention at the declared 24. Pre-rebuild snapshot at
  `generated/preserved/INVARIANT-SHAPE-ADMISSION.5/pre-rebuild/`. **−2 records, +0 anywhere**, and
  `temporal_conflicts` 1 → 0 — an independent falsification of `dyn_sigcon_0014` the leaf did not design
  for and did not need.

- `2026-09-12` — `.3`. All **20** obligation clauses adjudicated individually, not sampled: 1 `absent`,
  11 `self`, 5 `pronoun`, 3 `other`, each printed in full with its table id, name cell and description
  cell by `scripts/measure_signal_row_obligation_subject.py`. `.2`'s 17 rows re-derived from persisted
  SourceIR joined to `table_signal_declaration_provenance` — they reproduce exactly, and the modal list
  that produces 17 rather than 15 was identified rather than assumed.
  **Observed RED**: removing the `ObligationSubject::Head(_) => continue` arm fails
  `a_row_states_a_constraint_only_when_its_clause_binds_to_that_row_signal` and mints
  `OMEGABURST must_be_value "0"` from a `_WIDTH` obligation — the same shape as the persisted
  `dyn_sigcon_0013`.
  `cargo test` 472 / 168 / **1427** / 4 green; `cargo fmt --check` and
  `cargo clippy --all-targets -D warnings` green; `scripts/check_doctrines.sh` green;
  `scripts/check_chain_currency.sh` green — the evidence stage replayed for all 24 replayable documents
  with the current binary, proving the new pass is inert on every document the corpus can rebuild.
  Chain: the 24 replayable documents replayed at the evidence stage by `check_chain_currency.sh` — 24
  current, 0 stale. The three AMBA chains rebuilt in full after restoring their held-out bundles from
  `generated/preserved/WIRE-BASED-100.10/`; pre-rebuild snapshot at
  `generated/preserved/INVARIANT-SHAPE-ADMISSION.3/pre-rebuild/`; bundles `diff -r`-verified unchanged by
  the rebuild and removed, so the declared retained population stays at exactly 24. Baseline established
  rather than assumed: all three loaded cleanly with the HEAD binary before the change.

- `2026-09-12` — `.2`. Read-only; no artifact written, rebuilt or mutated. Classification by first cell
  and by modal presence over all 769 current table-row constraints, with samples printed per class and
  adjudicated by hand. The 17-row increment was measured from persisted SourceIR
  `signal_description` tables joined to each document's own declared set, not from the serialized
  statements — deliberately, since the point of the leaf is that the statements are unreadable.
- `2026-09-12` — `.1`. Controls, **observed RED**: with the refusal removed,
  `a_caption_is_not_an_invariant_unless_it_states_an_obligation` fails on
  `"Figure 3-1: HCLK debug state entry and exit"`, which route 2 admits for carrying `state` beside a
  declared signal. The same test asserts GREEN on `"Table A8.2: Opcodes which must be cache line sized
  and Regular"` (modal route) and on `"Tables are used throughout this chapter…"` (a label noun that is
  not a label). `a_caption_is_recognised_by_its_label_and_number_alone` pins the shape over the census's
  own forms and over seven near-misses including `Table`, `Figure ` and `TableOfContents`.
  `cargo test` 472 / 168 / **1423** / 4 green; fmt and clippy `-D warnings` green.
  Chain: all 27 proof-carrying documents rebuilt from `semantic` down, zero failures;
  `scripts/check_chain_currency.sh` current; retention untouched at the declared 24 (this change needs
  no bundle — `semantic` reads the persisted EvidenceIR).
- `2026-09-12` — `.0`. Read-only; no artifact written, rebuilt or mutated.
  `python3 scripts/measure_invariant_admission_shape.py`, plus two adjudications the script's samples
  support: every caption checked for a modal verb (20 of 759) and every table-row constraint's first
  cell checked against its own document's table-declared signal set (70 of 769 duplicate). The
  existing `|` idiom was located by reading all three call sites rather than assuming one.

## Commit Log

- Opened in the commit that closed `ANCHORLESS-INVARIANT-DROP.0` (`ec2b5a31`).
- `.0` — `INVARIANT-SHAPE-ADMISSION.0` (`481c2d39`).
- `.1` — `INVARIANT-SHAPE-ADMISSION.1` (`e3d22be0`).
- `.2` — `INVARIANT-SHAPE-ADMISSION.2` (`e30fed04`).
- `.3` — `INVARIANT-SHAPE-ADMISSION.3` (`ecc185c0`).
- `.5` — `INVARIANT-SHAPE-ADMISSION.5`.

## Changelog

- `2026-09-12` — `.5` closed. Gate 2 of `is_post_passive_binding_only_subject` exempted every table row
  from the rule the predicate's own doc-comment states; narrowed to exempt only a row whose obligation
  clause names no subject of its own. 4 records refused corpus-wide, 4 subjectless clauses preserved.
  AHB `signal_constraints` 16 → 14 with **nothing added**, and the fabricated `HPROT must_be_value 0`
  took a false `temporal_conflict` with it — it had been contradicting the document's own
  `HPROT[0] HIGH` rule.
- `2026-09-12` — `.3` closed, and it closed differently than it opened. The 17 rows are 20 obligation
  clauses; 12 bind to the row's own signal and are now read from the table, where the header still
  exists. The other 8 are refused with a stated reason. Two premises failed on measurement: 9 of the 17
  rows already produce a constraint (3 of them fabricated — `HBURST_WIDTH must be 0 or 3` published as
  `HBURST must_be_value 0`), so "no existing `signal_constraint` changes" could not be the bar, and the
  refusal half became `.5`. Rebuilt across APB, AXI and AHB: EvidenceIR `signal_constraints` 74 → **86**
  (+12, −0), `.isf` rules 177 → 200, and the prose families unmoved at 1,098 invariants / 1,117
  constraints.
- `2026-09-12` — `.2` closed. A table row is a matrix row whose subject lives in its header, and
  serialization discarded the header — so these constraints are unreadable where they are published.
  396 of 769 carry an obligation; the readable increment is 17 rows (`.3`) and the rest is a
  table-semantics programme (`.4`). The rows stay published meanwhile, and the reason is stated.
- `2026-09-12` — `.1` closed. `statement_is_a_caption` removes 739 captions across the rebuilt
  proof-carrying corpus; published constraints 5,927 → 5,188 with **prose and table rows unchanged**.
- `2026-09-12` — `.0` closed and split the tree. Captions (739 admitted by `r2`/`r3`, 20 by `r1`) are a
  precision defect and go to `.1`; table rows are an extraction gap — only 70 of 769 duplicate a
  declaration, 699 carry unique and often normative content — and go to `.2`.
- `2026-09-12` — tree created. 1,528 of 5,927 published IntentIR constraints in the proof-carrying
  stratum are figure captions or serialized table rows; the `r3` visual-evidence route admits 910
  non-statements against 245 prose.
