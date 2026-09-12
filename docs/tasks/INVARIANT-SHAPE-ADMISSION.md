# INVARIANT-SHAPE-ADMISSION: 739 captions are a precision defect; the table rows are an extraction gap, and they are not the same problem

## Metadata

- Tree ID: `INVARIANT-SHAPE-ADMISSION`
- Status: `active` (`2026-09-12`; `.0`-`.2` done; `.3` open — one bounded increment; `.4` is a program, not a slice)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-12`
- Last updated: `2026-09-12`
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

- ID: `INVARIANT-SHAPE-ADMISSION` · Status: `active` (`2026-09-12`) · Children: `.0`-`.2`

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

- ID: `INVARIANT-SHAPE-ADMISSION.3` · Status: `pending` · Goal: **a signal-description row whose
  description cell states an obligation about the signal that row declares is a `signal_constraint`.**
  Bounded and well-typed: the reader already has the table, already identifies the signal, and already
  emits `signal_constraints` (84 in the current stratum). Measured population: **17 rows** across APB
  (7), AXI (6) and AHB (4) — e.g. `RRESP` / *"Must be valid when RVALID is asserted"*, `AWLOOP` /
  *"A user-defined value that must be reflected from a write request to response transfers"*.
  Small, so the bar is precision: 17 is a population that can be adjudicated in full rather than
  sampled, and it should be.
  Prerequisite: `.2`. Verification: all 17 adjudicated individually; no existing `signal_constraint`
  changes; chain rebuilt; observed RED.

- ID: `INVARIANT-SHAPE-ADMISSION.4` · Status: `pending` · Goal: **read a requirement matrix.** The
  remaining ~380 obligation-bearing rows are permission matrices, truth tables and parameter tables —
  `| Read* | Shareable | Permitted to hit | Must hit | … |`, `| 0 | 0 | Non-secure | Non-secure | 0 |
  Non-trusted request that must target a Non-secure PAS. |`. **This is a program, not a slice**, and
  the leaf exists so it is tracked rather than implied. Do not start it as a slice; scope it first,
  and consider whether an existing table-semantics tree should own it.
  Prerequisite: `.3`. Verification: scoping is the deliverable.

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

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `INVARIANT-SHAPE-ADMISSION.3` — the 17-row signal-constraint increment. Bounded, fully adjudicable.
2. `INVARIANT-SHAPE-ADMISSION.4` — scope the matrix reader. **Not a slice.**

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
- `.2` — `INVARIANT-SHAPE-ADMISSION.2`.

## Changelog

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
