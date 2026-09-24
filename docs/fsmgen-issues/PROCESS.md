# Requests to FSMGen: bug reports and feature requests

This is SPECFORGE's process for anything it asks FSMGen to change. It builds on FSMGen's own published
protocol, `subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md` (bundle layout, `bin/fsmgen-issue-bundle`,
triage contract), and adds four things that protocol leaves to the reporter: a classification rule, a
verification standard, a feature-request shape, and a tracked lifecycle with a mechanically enforced
signoff. Ownership: `FSMGEN-REQUESTS` (`docs/tasks/FSMGEN-REQUESTS.md`).

Every request is a directory `docs/fsmgen-issues/<id>/` (the id starts `sf-`), owned by a task-tree leaf and
by an open record in [`docs/FSMGEN_FEEDBACK.md`](../FSMGEN_FEEDBACK.md). The owner forwards the directory
to FSMGen; SPECFORGE keeps tracking it until the change is integrated or the request is closed otherwise.

## 1. Classify before building anything

Read FSMGen's **current** public contract (its mdBook under `docs/book/src/` and
`docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md`) for the construct, then pick exactly one:

| The evidence shows | Kind | Where it goes |
| --- | --- | --- |
| FSMGen rejects what its docs say is supported, accepts what they say is an error, or emits wrong artifacts or diagnostics | `bug_report` | a bundle here |
| FSMGen does what its docs say, and SPECFORGE needs something they do not offer | `feature_request` | a bundle here |
| FSMGen is right and SPECFORGE produced a wrong artifact | SPECFORGE defect | a leaf in the owning SPECFORGE tree; **no** FSMGen request |
| The docs are ambiguous or silent and SPECFORGE needs an answer before it can tell | `question` / `clarity_request` | a record in `docs/FSMGEN_FEEDBACK.md` |

Cite the contract text that decides it (file and section) in the README. A finding can be two things at
once — SPECFORGE emitting a bad artifact *and* FSMGen accepting it when its contract says it must not —
and then each half gets its own owner (see `sf-isf-undeclared-rule-operand`).

## 2. Verify before filing

A request is filed only after it reproduces on:

1. **SPECFORGE's pinned FSMGen** (`subs/fsmgen`, read-only), and
2. **FSMGen's current upstream HEAD.** Export it read-only onto the repository volume —
   `git -C <FSMGEN_CHECKOUT> archive HEAD | tar -x -C generated/tmp/fsmgen-upstream-<sha>` — and run its
   `bin/fsmgen` there. Never modify `subs/fsmgen` or its pin to do this.

Also re-read the relevant upstream book sections at that HEAD (a capability may have shipped since the
pin), and search `docs/fsmgen-issues/`, `docs/FSMGEN_FEEDBACK.md`, its archive, and FSMGen's
`docs/SPECFORGE_FEEDBACK_RESPONSE.md` for an earlier record. `sf-isf-explicit-fsm-declaration` is the
standing example of what skipping this costs: a feature request withdrawn the same day because the pin was
312 commits stale and the capability already existed.

## 3. Build the bundle

1. Write **standalone, minimal** inputs by hand. Each companion input differs from the primary in one
   stated place; include the known-good counterpart. Use SPECFORGE corpus content only when the defect
   cannot be shown without it, and say why.
2. Run `bin/fsmgen-issue-bundle` from the upstream export (it writes the bundle there, not into
   `subs/fsmgen`), passing `--failure-class`, a concrete `--expected` and `--observed`, and every companion
   as `--extra-source`.
3. Append the companion probes to `commands.sh`, then **re-run `commands.sh`** from the FSMGen root, so
   everything under `observed/` is the script's own output.
4. Copy the bundle to `docs/fsmgen-issues/<id>/` and normalise it: replace machine-local absolute paths by
   `<FSMGEN_ROOT>`, `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>`, `<HOME>`; and, until FSMGen fixes
   `sf-tool-issue-bundle-foreign-git-head`, correct the recorded FSMGen head by hand (the helper records the
   enclosing repository's commit when it runs in an export). List every such change in the README.
5. Add `expected/expected-notes.md` (testable outcomes) and the known-good artifact with its captured
   result.

## 4. Write the README

It opens with the protocol's §1 summary, fields in order and all filled, then states the request kind,
the SPECFORGE owner and the local status. After that:

- **Bug report:** the defect in one paragraph; the reproduction command and a table of inputs, exit codes
  and results; evidence with file and line; the contract text it contradicts; expected behaviour;
  secondary observations kept apart from the defect; impact on SPECFORGE; revision note; redactions.
- **Feature request:** what SPECFORGE needs and why, in the specification's own terms; the measured need,
  with a command that re-derives it; the current contract and any precedent inside FSMGen; reproduction;
  the requested capability as numbered items marked required or optional, each with its failure cases;
  proposed acceptance tests; alternatives SPECFORGE considered and why they are not faithful; what
  SPECFORGE does until it ships; compatibility; revision note; redactions.

The README must stand alone: a maintainer who never saw SPECFORGE can reproduce, judge and test it.

## 5. `request.json`

| Field | Meaning |
| --- | --- |
| `schema_version` | `1` |
| `id` | the directory name |
| `kind` | `bug_report` or `feature_request` |
| `title` | one line |
| `owner` | the owning task leaf, e.g. `FSMGEN-REQUESTS.2` |
| `feedback_record` | the id of its record in `docs/FSMGEN_FEEDBACK.md` |
| `failure_class` | the protocol's failure class |
| `verified_on` | `[{fsmgen_revision, role: pinned\|upstream, result}]` — both roles required from `signed_off` on |
| `specforge_side` | the SPECFORGE leaf that owns any SPECFORGE half, or why there is none |
| `status` | see §6 |
| `signed_off` | the date signoff was completed |
| `forwarded` | `null`, or `{date, channel}` once the owner sends it |
| `upstream_reference` | `null`, or FSMGen's tracking reference (task id, commit, response anchor) |
| `resolution` | `null`, or `{date, fsmgen_revision, summary}` once closed |

## 6. Lifecycle

| `status` | Meaning | `FSMGEN_FEEDBACK.md` record status |
| --- | --- | --- |
| `drafted` | bundle under construction; must not be forwarded | `open` |
| `signed_off` | `SIGNOFF.md` complete; ready for the owner to forward | `open` |
| `forwarded` | sent to FSMGen (`forwarded` filled) | `awaiting_response` |
| `acknowledged` | FSMGen has accepted it (`upstream_reference` filled) | `accepted` |
| `resolved_upstream` | FSMGen shipped a fix or the feature | `shipped` / `resolved` |
| `integrated` | SPECFORGE pinned a revision containing it, re-ran `commands.sh`, recorded `RESOLUTION.md` | moved to the closed register |
| `rejected` / `withdrawn` | closed without a change; the README or `RESOLUTION.md` says why | moved to the closed register |

The owning leaf stays open until `integrated`, `rejected` or `withdrawn`.

## 7. Signoff

`SIGNOFF.md` holds nine boxes — classified, reproduced on the pin and upstream, standalone, minimized,
expected is testable, no duplicate, portable, owned, SPECFORGE side separated — each ticked only with the
evidence it names. `scripts/check_fsmgen_request_bundles.pl` runs inside the `LIVE-DOC-SIZE` doctrine and
refuses the commit when a bundle at `signed_off` or later:

- lacks `README.md` (with every §1 summary field), `commands.sh` (executable), `env.txt`,
  `sources/fsmgen-input/`, `observed/`, `expected/expected-notes.md`, `SIGNOFF.md` or a valid
  `request.json`;
- has an open box in `SIGNOFF.md`;
- lacks a `pinned` and an `upstream` entry in `verified_on`;
- names an owner leaf no task tree declares, or a `feedback_record` that is not an open record of
  `docs/FSMGEN_FEEDBACK.md` while the request is still open;
- contains a machine-local absolute path in any text file.

A bundle at `drafted` is checked for layout and ownership only. The three bundles filed before this
process (`sf-isf-contract-eventually-flat`, `sf-isf-stage-ready-valid`, `sf-isf-explicit-fsm-declaration`)
are closed history and exempt.

## 8. Forwarding and closure

The owner forwards the bundle directory (or an archive of it). Record the date and channel in
`request.json` `forwarded`, set `status` to `forwarded`, and move the feedback record to
`awaiting_response` in the same commit. When FSMGen answers, record `upstream_reference`. When a fix ships,
bump the pin through the FSMGen refresh procedure, re-run `commands.sh` against it, write `RESOLUTION.md`
(what changed, on which revision, what the re-run shows), move the feedback record to the closed register,
and close the leaf.
