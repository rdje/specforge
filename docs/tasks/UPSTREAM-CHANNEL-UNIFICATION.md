# UPSTREAM-CHANNEL-UNIFICATION: one channel for questions, bug reports and feature requests to upstream projects

## Metadata

- Tree ID: `UPSTREAM-CHANNEL-UNIFICATION`
- Status: `done`
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
  Status: `done` (`2026-09-25`; CHECK)
  Goal: generalise `check_fsmgen_request_bundles.pl` to any project and add the packet ↔ ledger-record bijection,
  with the §6 status correspondence, as a doctrine check with RED cases.
  **Landed** in `check_fsmgen_request_bundles.pl`, which LIVE-DOC-SIZE runs with its self-test: an outgoing open
  record (`SpecForge → FSMGen`, kind bug report, feature request, question or clarity request) needs exactly one
  packet; a packet's record must cite its README and agree on kind and on status (§6: `signed_off` ↔ `open`,
  `forwarded` ↔ `awaiting_response`, `acknowledged` ↔ `accepted`, `resolved_upstream` ↔ `shipped`/`resolved`/
  `answered`). Making it generic across projects waits for `.4`, which moves the packets under
  `docs/upstream/<project>/`. Self-test 16 cases, four new ones refusing each new rule; the 10 live packets pass.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.3`
  Status: `done` (`2026-09-25`; MIGRATION; landed before `.2` so the bijection check passes when it lands)
  Goal: `strict-unknown-port-clause` becomes the first question packet, `forwarded` `2026-09-25`, carrying its
  two probes (`present`, `bogus-clause`) at the pin and upstream.
  **Landed**: [`sf-isf-strict-unknown-port-clause`](../upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md),
  `kind: question`, built with FSMGen's bundle helper (failure class `unknown`, which FSMGen's reporting protocol
  allows when undecided). Three 13-line inputs pass `--strict` on `54ac1c22` and `354844fe1`, and both clauses
  generate a module equal to the clause-free one apart from its date line. The validator accepts `question`
  (self-test 12 cases); the ledger record points to the packet; the book lists it with the requests.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.4`
  Status: `done` (`2026-09-25`; MIGRATION)
  Goal: move `docs/fsmgen-issues/` to `docs/upstream/fsmgen/` with every link, catalog, citation, collection
  ceiling and book route following, and nothing left pointing at the old path.
  **Landed**: `git mv` of all 13 packets and `PROCESS.md`; 24 live files re-pointed (ledger, book, catalogs, KM cards,
  task trees, ADRs 0053/0054, the packet collection's targets, the feedback contract, the claim census and the
  checker). History keeps its old paths by design: the archives, closed trees `FSMGEN-ISSUE-REPORTING` and
  `SWD-SERIAL-EXTRACTION`, ADR 0011, and the two legacy bundles' captured command logs and `env.txt`. The checker
  walks `docs/upstream/<project>/` against a project-to-ledger map and refuses an unregistered project (self-test 17
  cases). No redirect file at the old path: every tracked Markdown file must belong to a registered surface.

- ID: `UPSTREAM-CHANNEL-UNIFICATION.5`
  Status: `done` (`2026-09-25`; DOCS)
  Goal: the book's *Requests To FSMGen* becomes *Requests to upstream projects*, describing the one channel; the
  process pages and the gap ledger's links follow.
  **Landed**: the page is now `reference/upstream-requests.md`, *Requests To Upstream Projects*: three kinds, one
  packet under `docs/upstream/<project>/<id>/`, one lifecycle (for a question, `resolved_upstream` is the answer),
  the ledger as a checked index, and a dated note on the change; SUMMARY, the gap ledger's seven links and the
  claim registry follow the rename.

## Current Frontier

Complete.

## Decisions

- `2026-09-25` — ADR 0054: one packet per outgoing item (`kind` question, bug report or feature request), one
  lifecycle, the ledger as index plus incoming answers, common process plus a per-project reproduction page.

## Open Questions

None.

## Blockers

None.

## Verification Log

- `2026-09-25` — `.0`: record kinds read from `docs/FSMGEN_FEEDBACK.md`; packets listed from `docs/upstream/fsmgen/`;
  `git submodule status` names one submodule, `subs/fsmgen`.

## Commit Log

- `.0` — `UPSTREAM-CHANNEL-UNIFICATION.0 — plan one channel for questions, bug reports and feature requests`.
- `.1` — `UPSTREAM-CHANNEL-UNIFICATION.1 — ADR 0054: one kind of packet for every outgoing item`.
- `.3` — `UPSTREAM-CHANNEL-UNIFICATION.3 — the open question becomes the first question packet`.
- `.2` — `UPSTREAM-CHANNEL-UNIFICATION.2 — every outgoing record has exactly one packet, and they agree`.
- `.4` — `UPSTREAM-CHANNEL-UNIFICATION.4 — packets move to docs/upstream/fsmgen, and the check covers any project`.
- `.5` — `UPSTREAM-CHANNEL-UNIFICATION.5 — the book describes one channel to any upstream project`.

## Changelog

- `2026-09-25` — tree opened on the owner's question; `.0` measured.
- `2026-09-25` — `.1` done: ADR 0054; `PROCESS.md` §1 now routes a question to a packet.
- `2026-09-25` — `.3` done (ahead of `.2`): `strict-unknown-port-clause` is the first question packet.
- `2026-09-25` — `.2` done: the record/packet bijection, evidence and status agreement are checked.
- `2026-09-25` — `.4` done: packets live under `docs/upstream/fsmgen/`; the checker is project-generic.
- `2026-09-25` — `.5` done; tree complete: questions, bug reports and feature requests travel as one kind of packet.
