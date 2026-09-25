---
id: questions-bugs-and-features-to-upstream-travel-as-one-kind-of-packet
title: A question, a bug report and a feature request to an upstream project each travel as one kind of packet, indexed by one ledger per project
date: 2026-09-25
status: accepted
scope: fsmgen, upstream, requests, questions, process, doctrine
evidence: docs/tasks/UPSTREAM-CHANNEL-UNIFICATION.md (.0 measurement); docs/fsmgen-issues/PROCESS.md; docs/FSMGEN_FEEDBACK.md; scripts/check_fsmgen_request_bundles.pl; docs/decisions/0053-extraction-and-isf-requests-advance-in-parallel.md
answers:
  - "how does SpecForge send a question to FSMGen"
  - "where does a question to an upstream project go"
  - "why do questions and bug reports to FSMGen follow the same process"
  - "what is an upstream packet"
  - "how is a question to FSMGen tracked until it is answered"
  - "can SpecForge report to an upstream project other than FSMGen"
---

# ADR 0054: A question, a bug report and a feature request to an upstream project each travel as one kind of packet

## Context

SpecForge asks FSMGen for three kinds of thing: a bug report, when FSMGen breaks its own published contract; a
feature request, when the contract lacks something SpecForge needs; and a question, when the contract is silent
or ambiguous. On `2026-09-25` the owner noticed that they do not travel the same way and asked for one channel
for questions, bug reports and feature requests to upstream projects.

Measured (`UPSTREAM-CHANNEL-UNIFICATION.0`): the 9 bug reports and feature requests are packets,
`docs/fsmgen-issues/<id>/`. Each has a `request.json` lifecycle (`drafted` → `signed_off` → `forwarded` → … →
`integrated`), a completed `SIGNOFF.md`, and a reproduction built with FSMGen's `bin/fsmgen-issue-bundle`, all
checked by `scripts/check_fsmgen_request_bundles.pl`. `PROCESS.md` §1 sends a question to a record in
`docs/FSMGEN_FEEDBACK.md` instead. The one question, `strict-unknown-port-clause`, therefore has no lifecycle
file and no sign-off. Its evidence borrows another packet's README, and the date it was forwarded lives only in
prose that no check reads. A question often carries a probe too: that one rests on two `--strict` runs.

## Decision

1. **Every outgoing item is a packet**, whatever its kind: `docs/upstream/<project>/<id>/`, with `fsmgen` the only
   project today. A packet holds a `README.md` (the item, and its reproduction or reasoning), a `request.json`
   and a `SIGNOFF.md`. When the item rests on a probe, which a bug report always does and a question often does,
   it also holds the reproduction (`sources/`, `commands.sh`, `observed/`, `expected/`).
2. **One `request.json` for three kinds.** `kind` is `bug_report`, `feature_request` or `question` (a clarity
   request is a question). Fields and lifecycle are shared. For a question, `resolved_upstream` means the
   project answered, and `integrated` means SpecForge recorded the answer and acted on it (filing the bug or
   feature request the answer calls for, or closing it).
3. **The ledger is an index, not a second channel.** Each project keeps one ledger (`docs/FSMGEN_FEEDBACK.md` for
   FSMGen). Every outgoing record in it points to exactly one packet, every packet has exactly one outgoing
   record, and the two agree on status by the table in `PROCESS.md` §6. Answers coming *from* the project stay
   ledger records; they are not outgoing, so they need no packet.
4. **What is common is written once.** Classification, verification, sign-off, lifecycle and the ledger contract
   live in one process for every project. How to reproduce against a given project (for FSMGen, its bundle
   helper and the pin/upstream comparison) lives in that project's own process page.
5. **A doctrine enforces it** (`UPSTREAM-CHANNEL-UNIFICATION.2`): the packet ↔ record bijection, the status
   correspondence and each packet's lifecycle, with RED cases.

`PROCESS.md` §1's row for *the docs are ambiguous or silent* changes from "a record in `docs/FSMGEN_FEEDBACK.md`"
to "a packet here, `kind: question`, with its ledger record". Forwarding stays the owner's act (`PROCESS.md` §8,
ADR 0053).

## Consequences

- A question gets a sign-off and a checked forwarding date, like any request; the one open question becomes a
  packet (`.3`).
- Packets move from `docs/fsmgen-issues/` to `docs/upstream/fsmgen/` (`.4`). Links, catalogs, ownership
  citations, collection ceilings and the book's routes move with them.
- Closed history (`docs/archive/fsmgen-feedback/`) is not rewritten.
- A second upstream project needs only its own directory, ledger and reproduction page.

## Alternatives considered

- **Keep questions in the ledger and add lifecycle fields to its records.** Rejected: two formats for one
  lifecycle, and a question's probe would have nowhere to live.
- **Move bug reports and feature requests into the ledger.** Rejected: a reproduction (inputs, commands,
  captured outputs) does not fit a record.

## Links

- Task tree: [`UPSTREAM-CHANNEL-UNIFICATION`](../tasks/UPSTREAM-CHANNEL-UNIFICATION.md)
- Process: [`docs/fsmgen-issues/PROCESS.md`](../fsmgen-issues/PROCESS.md)
- Related: [ADR 0053](0053-extraction-and-isf-requests-advance-in-parallel.md)
