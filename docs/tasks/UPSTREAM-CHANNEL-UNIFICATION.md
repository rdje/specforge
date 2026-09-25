# UPSTREAM-CHANNEL-UNIFICATION: one channel for questions, bug reports and feature requests to upstream projects

## Metadata

- Tree ID: `UPSTREAM-CHANNEL-UNIFICATION`
- Status: `active`
- Roadmap lane: `R6` (the FSMGen boundary), applies to any upstream project
- Created: `2026-09-25`
- Last updated: `2026-09-25`
- Owner: repo-local workflow
- Owner directive (`2026-09-25`): *"bugs and features go through the same channel, which is one directory under
  docs/fsmgen-issues, but apparently questions go in docs/FSMGEN_FEEDBACK.md. Would it be possible to unify all 3
  categories, and to plan an unification of question, bug and feature requests to upstream projects?"*

## Goal

Every outgoing item to an upstream project (a question, a bug report or a feature request) travels the same way:
one packet directory with the same `request.json` lifecycle and sign-off, one ledger per project that indexes the
packets and records the answers, and a doctrine check that the two always agree.

## Non-Goals

- Changing how FSMGen receives or answers: its inbox and reply ledger are its own.
- Rewriting closed history: `docs/archive/fsmgen-feedback/` stays as it is.
- Inventing a second upstream before one exists; the layout only has to admit one.

## Acceptance Criteria

- An ADR fixes the packet layout, the three kinds, the shared lifecycle and the ledger's role.
- No outgoing ledger record exists without exactly one packet, and no packet without its record; a check refuses
  either, and the lifecycle state agrees on both sides (`PROCESS.md` §6).
- The open question `strict-unknown-port-clause` (`EXTRACTION-REPRESENTATION-GAPS.19`) is a packet like any other.
- Packets live under a project-keyed path; every link, catalog, citation and book route follows the move.
- The book explains the one channel for any upstream project.

## Task Tree

- ID: `UPSTREAM-CHANNEL-UNIFICATION.0`
  Status: `done` (`2026-09-25`; MEASUREMENT)
  Goal: the channel as it is. FSMGen is the only upstream (the one submodule). `docs/FSMGEN_FEEDBACK.md` holds 10
  SpecForge → FSMGen records. The 9 bug reports and feature requests each have a packet
  `docs/fsmgen-issues/<id>/` (`request.json` lifecycle `drafted` → … → `integrated`, `SIGNOFF.md`, a reproduction
  made with FSMGen's `bin/fsmgen-issue-bundle`), checked by `scripts/check_fsmgen_request_bundles.pl`. The one
  question (`strict-unknown-port-clause`, `.19`) is only a ledger record whose evidence borrows the
  `sf-isf-optional-signal` README: it has no `request.json` and no sign-off, so its forwarding date lives only in
  prose and no check sees it. Every path and name is FSMGen's.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.1`
  Status: `done` (`2026-09-25`; DECISION — ADR 0054)
  Goal: an ADR. One packet per outgoing item, any kind, under `docs/upstream/<project>/<id>/`. `request.json`
  `kind` is `question`, `bug_report` or `feature_request`, with one lifecycle for all three (for a question, FSMGen's
  answer is its `resolved_upstream`). A question packet carries its probe when one exists, otherwise the reasoning
  and citations that make it a question (`PROCESS.md` §1). The per-project ledger is the index of packets plus
  incoming answers. The reproduction specifics (FSMGen's bundle helper) stay in a per-project process page.
  `decision_records` is 3 files from its rollover ceiling, so this may need the ADR rollover first.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.2`
  Status: `open` (`2026-09-25`; CHECK)
  Goal: generalise `check_fsmgen_request_bundles.pl` to any project and add the packet ↔ ledger-record bijection,
  with the §6 status correspondence, as a doctrine check with RED cases.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.3`
  Status: `done` (`2026-09-25`; MIGRATION; landed before `.2` so the bijection check passes when it lands)
  Goal: `strict-unknown-port-clause` becomes the first question packet, `forwarded` `2026-09-25`, carrying its
  two probes (`present`, `bogus-clause`) at the pin and upstream.
  **Landed**: [`sf-isf-strict-unknown-port-clause`](../fsmgen-issues/sf-isf-strict-unknown-port-clause/README.md),
  `kind: question`, built with FSMGen's bundle helper (failure class `unknown`, which FSMGen's reporting protocol
  allows when undecided). Three 13-line inputs pass `--strict` on `54ac1c22` and `354844fe1`, and both clauses
  generate a module equal to the clause-free one apart from its date line. The validator accepts `question`
  (self-test 12 cases); the ledger record points to the packet; the book lists it with the requests.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.4`
  Status: `open` (`2026-09-25`; MIGRATION)
  Goal: move `docs/fsmgen-issues/` to `docs/upstream/fsmgen/` with every link, catalog, citation, collection
  ceiling and book route following, and nothing left pointing at the old path.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.5`
  Status: `open` (`2026-09-25`; DOCS)
  Goal: the book's *Requests To FSMGen* becomes *Requests to upstream projects*, describing the one channel; the
  process pages and the gap ledger's links follow.

## Current Frontier

1. `.2` — the check (`.1` decided, `.3` landed first so the check passes); then `.4`, `.5`.

## Decisions

- `2026-09-25` — ADR 0054: one packet per outgoing item (`kind` question, bug report or feature request), one
  lifecycle, the ledger as index plus incoming answers, common process plus a per-project reproduction page.

## Open Questions

None.

## Blockers

None.

## Verification Log

- `2026-09-25` — `.0`: record kinds read from `docs/FSMGEN_FEEDBACK.md`; packets listed from `docs/fsmgen-issues/`;
  `git submodule status` names one submodule, `subs/fsmgen`.

## Commit Log

- `.0` — `UPSTREAM-CHANNEL-UNIFICATION.0 — plan one channel for questions, bug reports and feature requests`.
- `.1` — `UPSTREAM-CHANNEL-UNIFICATION.1 — ADR 0054: one kind of packet for every outgoing item`.
- `.3` — `UPSTREAM-CHANNEL-UNIFICATION.3 — the open question becomes the first question packet`.

## Changelog

- `2026-09-25` — tree opened on the owner's question; `.0` measured.
- `2026-09-25` — `.1` done: ADR 0054; `PROCESS.md` §1 now routes a question to a packet.
- `2026-09-25` — `.3` done (ahead of `.2`): `strict-unknown-port-clause` is the first question packet.
