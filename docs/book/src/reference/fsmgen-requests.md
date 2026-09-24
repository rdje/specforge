# Requests To FSMGen

SpecForge lowers its intent to `.isf`, and FSMGen turns `.isf` into state machines and HDL. When SpecForge
needs FSMGen to change — because FSMGen breaks its own published contract, or because the contract lacks
something a specification needs — SpecForge prepares a **request bundle** that FSMGen can reproduce on its
own machine with one command. The owner forwards the bundle; SpecForge tracks it locally until the change
is in the FSMGen revision SpecForge uses, or the request is closed another way.

The full procedure is [`docs/fsmgen-issues/PROCESS.md`](../../../fsmgen-issues/PROCESS.md). This page
explains it and lists the requests SpecForge is tracking.

## Bug, feature, or our own defect?

Every finding is classified against FSMGen's current documentation before anything is built:

| What the evidence shows | What SpecForge files |
| --- | --- |
| FSMGen rejects what its documentation says is supported, accepts what it says is an error, or produces wrong output | a **bug report** |
| FSMGen does what its documentation says, but SpecForge needs something it does not offer | a **feature request** |
| FSMGen is right and SpecForge produced a wrong `.isf` | nothing to FSMGen — a SpecForge task fixes it |
| The documentation does not say | a **question** in `docs/FSMGEN_FEEDBACK.md` |

A finding can be two things. When SpecForge wrote the prose word `VALID` as a rule value, that was
SpecForge's defect; FSMGen *accepting* it, against its own rule that undeclared operands are errors, was
FSMGen's. Each half has its own owner.

## What makes a request ready

A request is signed off only when all nine points of its `SIGNOFF.md` hold, and a check that runs on every
SpecForge commit refuses the commit otherwise:

1. **Classified**, with the FSMGen documentation text that decides it quoted.
2. **Reproduced twice**: on the FSMGen revision SpecForge is pinned to, *and* on FSMGen's current
   upstream revision, because a request against an old revision may ask for something that already
   exists.
3. **Standalone**: its `commands.sh` runs from FSMGen's root using only files in the bundle, and every
   captured output came from that script.
4. **Minimal**: the inputs are small hand-written examples, each differing from its companion in one
   place, with the accepted counterpart included.
5. **Testable expectation**: `expected/expected-notes.md` says what a regression test should assert.
6. **Not a duplicate** of an earlier request or of an answer FSMGen already gave.
7. **Portable**: no path from this machine appears anywhere in it.
8. **Owned** by a task leaf and by an open record in `docs/FSMGEN_FEEDBACK.md`.
9. **SpecForge's own part separated**, with its own owner, so FSMGen is never asked to fix SpecForge.

## Lifecycle

`drafted` → `signed_off` (ready to forward) → `forwarded` → `acknowledged` → `resolved_upstream` →
`integrated`; or `rejected` / `withdrawn` with the reason written down. Each bundle's `request.json`
holds the current state, and `docs/FSMGEN_FEEDBACK.md` mirrors it. A request is closed only after
SpecForge has moved to an FSMGen revision containing the change and re-run the bundle against it.

**When you forward one:** send its directory, then record the date and channel in its `request.json`
`forwarded` field and set its `status` to `forwarded`. The check then requires those details, and
the feedback record moves to `awaiting_response` in the same commit.

## Current requests

All three were forwarded and acknowledged by FSMGen on `2026-09-24`, and FSMGen is working on them.
Each closes when a published FSMGen revision containing its change is pinned and its bundle re-run.

| Bundle | Kind | Status | What it asks |
| --- | --- | --- | --- |
| [`sf-isf-derived-width-parameter`](../../../fsmgen-issues/sf-isf-derived-width-parameter/README.md) | feature request | acknowledged | Let an `.isf` port width be derived from a parameter (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`, `clog2(N)`) and stay generic when the block is instantiated. FSMGen already evaluates such expressions one level down, in `.fsm`. |
| [`sf-isf-undeclared-rule-operand`](../../../fsmgen-issues/sf-isf-undeclared-rule-operand/README.md) | bug report | acknowledged | Strict mode accepts a rule that reads a name nobody declared, and the generated module gains an input port for it. |
| [`sf-tool-issue-bundle-foreign-git-head`](../../../fsmgen-issues/sf-tool-issue-bundle-foreign-git-head/README.md) | bug report | acknowledged | FSMGen's own bundle tool records the wrong project's revision when run from an exported copy of FSMGen. |

Until the first one ships, every derived-width signal is emitted at `(width 1)` and listed, with its
expression, in the adapter residual `isf_signal_width_unlowered` (see
[How wide is each signal?](../pipeline/isf-adapter.md#how-wide-is-each-signal)).

Authoritative tracking: `docs/tasks/FSMGEN-REQUESTS.md`.
