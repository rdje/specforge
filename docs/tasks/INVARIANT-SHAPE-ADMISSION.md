# INVARIANT-SHAPE-ADMISSION: 739 captions are a precision defect; the table rows are an extraction gap, and they are not the same problem

## Metadata

- Tree ID: `INVARIANT-SHAPE-ADMISSION`
- Status: `active` (`2026-09-12`; `.0` done — it split the population in two; `.1` and `.2` open)
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

- ID: `INVARIANT-SHAPE-ADMISSION.1` · Status: `pending` · Goal: **a figure or table caption is not
  admitted by `r2` or `r3`.** The level is `is_invariant_like`, matching the idiom three other passes
  already use for a table row. The rule needs no second condition, because `r1` is tested first: the
  **20** captions that carry a modal verb are admitted by `r1` and are untouched, and the **739** that
  do not are exactly the population `r2` and `r3` admit.
  Population removed, measured: 104 (`r2`) + 635 (`r3`) = 739 of 5,856 current SemanticIR invariants.
  Prerequisite: `.0`. Verification: observed RED on a bare caption and GREEN on a modal-bearing one;
  the removed set sampled and adjudicated; no prose statement affected; golds re-scored; the chain
  rebuilt for every document whose artifacts move — which will be most of them, so budget for it.

- ID: `INVARIANT-SHAPE-ADMISSION.2` · Status: `pending` · Goal: **read a table row instead of publishing
  it.** 699 of the 769 table-row constraints carry content found nowhere else, and the sample is
  normative: `| Secure | Must be zero |`, `| True | | When Burst type is WRAP, the transaction must be
  cache line sized and Modifiable. |`, `| BRESP_WIDTH | 0, 2, 3 | 2 | Width of BRESP in bits. Must be 3
  if … |`. This is an **extraction gap wearing a precision defect's clothes**: the requirement is real
  and the serialization is wrong.
  Census first — what shapes do these rows take, how many are a condition/value pair, a parameter
  constraint, a response-code definition? Only 70 duplicate a declaration the reader already made, so
  subtraction is not the answer for the other 699.
  Prerequisite: `.1` (it removes the caption noise this census would otherwise have to filter).
  Verification: the census is the deliverable; a count with no adjudication is not.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `INVARIANT-SHAPE-ADMISSION.1` — refuse a caption in `r2`/`r3`. Bounded, measured, and the larger of
   the two precision wins.
2. `INVARIANT-SHAPE-ADMISSION.2` — the table-row census. After `.1`, so its population is clean.

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

- `2026-09-12` — `.0`. Read-only; no artifact written, rebuilt or mutated.
  `python3 scripts/measure_invariant_admission_shape.py`, plus two adjudications the script's samples
  support: every caption checked for a modal verb (20 of 759) and every table-row constraint's first
  cell checked against its own document's table-declared signal set (70 of 769 duplicate). The
  existing `|` idiom was located by reading all three call sites rather than assuming one.

## Commit Log

- Opened in the commit that closed `ANCHORLESS-INVARIANT-DROP.0` (`ec2b5a31`).
- `.0` — `INVARIANT-SHAPE-ADMISSION.0`.

## Changelog

- `2026-09-12` — `.0` closed and split the tree. Captions (739 admitted by `r2`/`r3`, 20 by `r1`) are a
  precision defect and go to `.1`; table rows are an extraction gap — only 70 of 769 duplicate a
  declaration, 699 carry unique and often normative content — and go to `.2`.
- `2026-09-12` — tree created. 1,528 of 5,927 published IntentIR constraints in the proof-carrying
  stratum are figure captions or serialized table rows; the `r3` visual-evidence route admits 910
  non-statements against 245 prose.
