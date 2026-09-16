# COMMIT-GATE-SINGLE-RUN: every slice pays the full doctrine gate twice

## Metadata

- Tree ID: `COMMIT-GATE-SINGLE-RUN`
- Status: `active` (`2026-09-15`; `.0` open)
- Roadmap lane: process / continuity (commit workflow)
- Created: `2026-09-15`
- Last updated: `2026-09-17`
- Owner: repo-local workflow

## Goal

`COMMIT.md` step 8 tells the agent to run `scripts/check_doctrines.sh` before committing, and
`.githooks/pre-commit` ends by running the same driver. Both are correct in isolation and the
combination is redundant: **every slice pays the complete gate twice**, and the second run is the one
that can block, so the first buys only an earlier failure signal.

Make the cost paid once without weakening the gate. The un-bypassable leg is the hook (a local run can
simply be skipped), so whatever changes, the hook keeps running the full driver.

## Why this tree exists at all

The director approved a change on `2026-08-31`, in response to a steer to stop burning time on
administrative work: *for a slice that touches no Rust, run only a focused subset by hand and let the
hook run the full driver once at commit.* Their words: *"Move on with your proposition … we will amend
later if need be."*

**It was never implemented, because no tree owned it.** The obvious candidates were already closed
(`DOCTRINE-ENFORCEMENT-ADOPT` is `done`), and doctrine forbids a change without an owning leaf, so the
approval sat in an agent memory note across sessions instead of in the repository. That is the failure
this tree fixes first: an approved change with no owner is not tracked, it is remembered.

## Measured cost — and the earlier figures were wrong

**Re-measured `2026-09-15` over one long session (10 commits), wall clock from task-file birth to
mtime:**

| run | count | mean | range |
| --- | ---: | ---: | --- |
| standalone `check_doctrines.sh` (the `COMMIT.md` step-8 run) | 11 | **5m57s** | 5m14s – 7m13s |
| `git commit` (hook gate + commit) | 10 | **5m49s** | 5m06s – 8m02s |

**The redundant half cost this session about 65 minutes.** At ~6 minutes a run it is roughly one hour
per ten-commit session, which is the scale that justifies the work — not per-commit urgency.

**Two earlier figures are withdrawn, and how they were wrong matters more than the numbers.**
`2026-08-31` recorded *"LIVE-DOC-SIZE alone ran over 8 minutes and was still going"* and a session
asserted *"~15 min each / ~30 min per commit"*. Both were invented from the fact that a run exceeded a
2-minute foreground timeout, never derived. A `2026-09-01` measurement (n=4) already put the gate at
~5 minutes; this session's n=21 confirms it. **Do not seed any policy from a figure that was not
measured** — the whole point of the focused subset is that it is chosen by per-doctrine timing.

## Non-Goals

- Weakening the hook. It is the un-bypassable leg; it keeps running the complete driver.
- Composing a "focused subset" from a guess. No doctrine enters or leaves the fast set without its own
  measured cost — the same bar this repository applies to every other rule.
- Removing the agent's ability to run the gate early. An early signal is worth having when it is
  *chosen*, not when it is mandatory.

## Acceptance Criteria

- A slice that touches no Rust pays the full gate **once**, not twice.
- The hook still runs every registered doctrine, and a non-compliant change still cannot land.
- Whatever `COMMIT.md` ends up instructing is derived from per-doctrine timings, with the measurement
  committed.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `COMMIT-GATE-SINGLE-RUN` · Status: `active` (`2026-09-15`) · Children: `.0`, `.1`

- ID: `COMMIT-GATE-SINGLE-RUN.0` · Status: `pending` (opened `2026-09-15`) · Goal: **time each doctrine
  individually before any policy is written.** The tree-level totals above are the case for doing the
  work; they cannot choose the fast set. Produce a per-doctrine wall-clock table from the registry in
  `scripts/check_doctrines.sh`, on an idle machine, and commit it.
  **Run it alone.** A standing hazard in this repository is that a heavy suite run concurrently with the
  locality gate distorts both; this session's own outlier (13m34s against a 5m57s mean) is what a
  contended measurement looks like.
  **What the table must decide, and neither answer is assumed:** whether a cheap subset exists that
  catches the ordinary docs-only failure early (this session's two real blocks were LIVE-DOC-SIZE and
  PRODUCTION-GENERICITY, both of which the hook would have caught anyway), or whether the honest change
  is simply to delete step 8's manual run and let the hook be the only gate.
  Non-goal: writing the `COMMIT.md` change in this leaf. Measure first.
  Prerequisite: none.
  Verification: pending
  Commit: pending

- ID: `COMMIT-GATE-SINGLE-RUN.1` · Status: `pending` (opened `2026-09-17`) · Goal: **make a hand-run
  focused check impossible to get silently wrong.** This tree's whole remedy is *run a focused subset by
  hand*, and hand-running is exactly where a check turns into a no-op. Censused `2026-09-17` over the 42
  `scripts/check_*.pl|sh` gates:
  **(a) there is no flag convention — 21 accept `--check` and 21 do not**, so the correct invocation is
  per-script knowledge with nothing to check it against.
  **(b) the two families fail differently, and one of them does not fail.** Given an unsupported
  `--check`, the Perl gates die with usage and a non-zero status (`check_live_document_size.pl` 255,
  `check_derived_state_contracts.pl` 255, `check_rolling_ledger_protocol.pl` 25) — loud. But
  `check_readme_policy.sh --check` and `check_memory_architecture.sh --check` **ignore the unknown
  argument, run, and exit 0**: a wrong flag is accepted rather than refused, so a near-miss on a script
  that has a meaningful mode (`--apply`, `--all`, `--apply-rollover`, `--execute-stale-gates`) selects the
  wrong mode silently.
  **(c) the caller amplifies it.** `cmd | grep -i warning | head` reports `$?` from `head`, so a
  non-zero gate reads as success, and usage text on stderr looks nothing like a failure to a reader
  grepping for findings. Observed **twice in one session** (`2026-09-17`): a "no live-document warnings"
  reading that was a usage error hiding **26** warnings, and a commit-completion check that matched the
  PREVIOUS commit's subject and led to the message file being cleared mid-commit.
  **Decide between**: a single focused-subset entry point that owns every script's flags and asserts each
  exit status, so no caller picks flags at all; or normalizing `--check` as an accepted alias across all
  42 plus unknown-argument refusal in the shell gates. The first changes the shape; the second changes 42
  numbers. Do not seed the fast set from this leaf — that is `.0`'s job and it must measure first.
  Prerequisite: none; found while closing `LIVE-DOCUMENT-PRESSURE-HEADROOM.28`.
  Verification: pending
  Commit: pending

## Current Frontier

Ordered; PNT selects the first eligible leaf.

0. `COMMIT-GATE-SINGLE-RUN.0` — per-doctrine timing on an idle machine. Everything else in this tree is
   blocked on it, deliberately: the approved remedy is a *focused subset*, and a subset chosen without
   timings is the same guess that produced the two withdrawn figures above.
1. `COMMIT-GATE-SINGLE-RUN.1` — a hand-run check that can silently be a no-op. Not blocked on `.0`: it
   decides whether a focused subset is *safe* to hand-run, while `.0` decides what would be *in* one.

## Decisions

- `2026-09-15` (tree opened): **the hook is the leg that must not move.** `MEMORY_ARCHITECTURE.md` §9
  layers discovery / self-check / git hook / CI, and notes the hosted CI is manual-only. With CI not
  firing automatically, the pre-commit hook is the only un-bypassable enforcement in practice, so the
  redundancy must be removed from the manual side.
- `2026-09-15`: **this is background hygiene, not feature work.** Measured at ~1 hour per ten-commit
  session it pays off across a long run and not within one slice; schedule it accordingly rather than
  ahead of product work.
