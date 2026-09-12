# INVARIANT-SHAPE-ADMISSION: a quarter of published constraints are figure captions and table rows

## Metadata

- Tree ID: `INVARIANT-SHAPE-ADMISSION`
- Status: `active` (`2026-09-12`; opened by `ANCHORLESS-INVARIANT-DROP.0`, `.0` open)
- Roadmap lane: `R2` (extraction correctness / false-positive control)
- Created: `2026-09-12`
- Last updated: `2026-09-12`
- Owner: repo-local workflow

## Goal

An `InvariantRecord` is meant to be a **normative constraint**, and it flows to IntentIR
`constraints` — the product boundary a downstream consumer is asked to enforce.

Measured over the proof-carrying stratum, **1,528 of 5,927 published IntentIR constraints (25.8%) are
not statements at all**: 769 are serialized markdown table rows and 759 are figure captions or
cross-references, up to and including a constraint whose entire text is `Figure 1.`

Stop publishing markup as a requirement, without losing a requirement that happens to live in a table.

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
into `Signal AWVALID is …`. They are published twice: once correctly as a declaration and once as raw
markup in the constraint set.

## Non-Goals

- Do not delete a requirement because it lives in a table. A specification states real obligations in
  tables, and a row's *content* may be normative even when its *serialization* is not. The defect is
  publishing the pipe-delimited row; recovering its content is a separate, larger question.
- Do not touch `r1`. A statement carrying `must` or `shall` is normative by the document's own grammar,
  and its 4,397 admissions are the tree's baseline of correct behaviour.
- Do not narrow the weak-phrase list as the fix. `state` admits 105 of `r2` alone, but `r2` is 304 of
  5,856 — the shape problem is four times larger in `r3` and would survive any phrase change.

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

- ID: `INVARIANT-SHAPE-ADMISSION` · Status: `active` (`2026-09-12`) · Children: `.0`

- ID: `INVARIANT-SHAPE-ADMISSION.0` · Status: `pending` · Goal: **decide where the shape is refused, and
  what happens to a table row's content.** The population is already measured
  (`scripts/measure_invariant_admission_shape.py`); what is not decided is the level. Three candidates,
  and the leaf must choose on evidence rather than taste:
  1. at `is_invariant_like` — cheapest, refuses the shape for every route at once;
  2. at the statement source — a caption and a table row arguably should not be `StatementClass`
     candidates for an invariant at all, which would also stop them reaching other passes;
  3. at the SourceIR/EvidenceIR boundary, where a table row is serialized into a statement in the first
     place — the most upstream, and the only one that could stop the double-publication.
  Measure what each level would also affect: a statement text is read by more than one pass, so a
  refusal at (2) or (3) has a blast radius that (1) does not, and this repository has twice measured
  such a radius to be surprising.
  Non-goal: any code change.
  Prerequisite: none. Verification: read-only; no artifact written or mutated.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `INVARIANT-SHAPE-ADMISSION.0` — where to refuse, and what a table row's content is owed.

## Decisions

- `2026-09-12` — **opened rather than folded into `ANCHORLESS-INVARIANT-DROP`.** That tree asked how a
  dropped invariant should be accounted for and its premise was wrong; this is a distinct, larger, and
  correctly-premised defect found while disproving it.
- `2026-09-12` — **the first leaf decides a level, not a rule.** The shape test is trivial — a leading
  `|`, or a leading `Figure`/`Table` label plus a number. What is not trivial is which of three places
  should carry it, and the repository's recent history is that the level decides whether a fix repairs
  a cause or merely changes how a defect looks.

## Open Questions

- Is `r3` defensible at all? A statement admitted purely because it sits near a normative figure yields
  245 prose invariants and 910 non-statements. If the prose 245 are themselves weak, the route is worth
  re-deriving rather than filtering.
- What is a signal-description table row worth once the declaration reader has already consumed it? If
  the answer is "nothing", the double-publication is pure noise and the fix is subtraction.

## Blockers

None.

## Verification Log

Pending: `.0` is a read-only census.

## Commit Log

Opened in the commit that closed `ANCHORLESS-INVARIANT-DROP.0`.

## Changelog

- `2026-09-12` — tree created. 1,528 of 5,927 published IntentIR constraints in the proof-carrying
  stratum are figure captions or serialized table rows; the `r3` visual-evidence route admits 910
  non-statements against 245 prose.
