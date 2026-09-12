# ANCHORLESS-INVARIANT-DROP: an invariant that loses its last interface anchor is deleted, not residualised

## Metadata

- Tree ID: `ANCHORLESS-INVARIANT-DROP`
- Status: `active` (`2026-09-12`; opened by `ACTOR-NOUN-RELATION-DECLARATION.1`, `.0` open)
- Roadmap lane: `R2` (extraction fidelity / residual accounting)
- Created: `2026-09-12`
- Last updated: `2026-09-12`
- Owner: repo-local workflow

## Goal

A SemanticIR invariant carries `related_interface_ids`. When every one of those references disappears,
the invariant **disappears with them** — it is not emitted, and nothing records that it was dropped.
`residual_decisions` does not grow; `fidelity_findings` does not grow. The requirement simply is not in
the artifact.

Make an invariant that loses its last anchor **accountable**: either it keeps a usable form, or its
removal is a typed residual a reader can act on. Silence is the defect, not the removal.

## How it was found

`ACTOR-NOUN-RELATION-DECLARATION.1` stopped an inferred declaration minting an ordinary word as a wire,
which removed ADIv6's phantom signal `In`. Rebuilding that document's chain:

| | before | after |
| --- | ---: | ---: |
| IntentIR interfaces | 21 | 14 — the 7 lost are all `…in…` groupings built on the phantom |
| IntentIR constraints | 614 | 601 |
| SemanticIR invariants | 606 | **593** |

Of the 57 constraints that moved, **44 keep their id** and simply shed the phantom-bearing entries from
`related_interface_ids`, retaining their real ones. That part is exactly right. The other **13 vanish**,
and their ids say why they were vulnerable:

```text
constraint_invariant_in_the_capture_dr_state_a_logic_0_is_loaded_into_this_register
constraint_invariant_in_the_update_dr_state_nothing_happens_the_shifted_in_data_is_ignored
constraint_invariant_in_the_shift_dr_state_this_data_is_shifted_out_least_significant_bit_first
…
```

These are real JTAG requirements whose sentences **begin with the word "In"** — which is precisely why
the phantom was minted from them in the first place. They were anchored to nothing but that phantom, so
when it went, they went.

**This contradicts a published claim.** `SPEC-TO-INTENT-ALIGNMENT`'s signoff states "zero fabrication and
zero unexplained drops". Thirteen unexplained drops is what a single upstream correction produced, so the
claim is scoped to a reviewed population rather than to the mechanism, and the mechanism has no accounting
at all. `ACTOR-NOUN-RELATION-DECLARATION.1` shipped anyway and said so: an invariant anchored to a wire
that does not exist was never correctly anchored, and publishing it is not better than not publishing it.
But it should not leave silently.

## Non-Goals

- Do not keep an invariant by re-attaching it to a phantom. The upstream correction is right.
- Do not widen `related_interface_ids` to admit an anchor that is not a declared signal. The anchor
  contract is what makes an invariant actionable downstream.
- Do not treat this as an ADIv6 defect. ADIv6 is where it was observed; the mechanism is general and the
  census must say how general.

## Acceptance Criteria

- The population is measured: across the proof-carrying stratum, how many invariants would be dropped if
  their anchors were removed, and how many are today anchored **solely** to a single interface — the
  fragile set.
- Whatever is shipped makes the drop **visible** in the artifact, and the form is chosen against the
  existing residual/fidelity carriers rather than by inventing a new one.
- No invariant that has a real anchor changes.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `ANCHORLESS-INVARIANT-DROP` · Status: `active` (`2026-09-12`) · Children: `.0`

- ID: `ANCHORLESS-INVARIANT-DROP.0` · Status: `pending` · Goal: **measure the fragile set, and find where
  the drop happens.** Two parts, both read-only:
  1. census — per document and per stratum, how many SemanticIR invariants carry exactly one
     `related_interface_id`, and how many carry only ids that would vanish with one signal;
  2. locate the code path that discards an invariant with no surviving anchor, and say whether it
     discards silently by construction or by omission. The difference decides whether `.1` is a carrier
     change or a one-line accounting fix.
  Non-goal: any code change.
  Prerequisite: none. Verification: read-only; no artifact written or mutated.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `ANCHORLESS-INVARIANT-DROP.0` — the census. Nothing else in this tree may start before it.

## Decisions

- `2026-09-12` — **the upstream correction was shipped before this was fixed.** The alternative was to
  keep a phantom wire in a published IntentIR so that thirteen invariants would keep a false anchor.
  A downstream consumer asked to synthesise a port called `In` is a worse outcome than a recorded gap,
  and the gap is now recorded — here.
- `2026-09-12` — **opened as its own tree rather than a leaf of `SPEC-TO-INTENT-ALIGNMENT`.** That tree's
  frontier is its own product programme and its parts are sharded; this is one mechanism with one
  measurement, and folding it in would bury it.

## Open Questions

- Is the right carrier a residual, a fidelity finding, or an invariant that keeps its text with an empty
  anchor set and a stated reason? `.0` must answer this against what the carriers already express, not
  by preference.
- How many of the fragile set are fragile because their only anchor is itself weak? If most single-anchor
  invariants are anchored to an inference rather than a table declaration, the accounting question is
  larger than one phantom.

## Blockers

None.

## Verification Log

Pending: `.0` is a read-only census.

## Commit Log

Opened in the commit that shipped `ACTOR-NOUN-RELATION-DECLARATION.1`.

## Changelog

- `2026-09-12` — tree created. Removing one phantom signal from ADIv6 deleted 13 real JTAG invariants
  with no record of the deletion, while 44 sibling constraints correctly kept their id and shed only the
  phantom anchor.
