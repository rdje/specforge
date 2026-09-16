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

- ID: `COMMIT-GATE-SINGLE-RUN` · Status: `active` (`2026-09-15`) · Children: `.0`, `.1`, `.2`

- ID: `COMMIT-GATE-SINGLE-RUN.0` · Status: `done` (`2026-09-17`) · Goal: **time each doctrine
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
  **Measured `2026-09-17` at `474de7f2`, alone on an idle machine**, by
  `scripts/measure_doctrine_cost.sh`, which DERIVES its population by parsing the driver's `DOCTRINES=(...)`
  registry rather than listing it, so a doctrine added there cannot be missing from the table. Gate tier,
  **16 of 18 registered** (two are CI-tier), every one PASS:

| Doctrine | Wall clock | Share | Enforcer |
| --- | ---: | ---: | --- |
| `LIVE-DOC-SIZE` | 1m25.9s | 25.6% | `scripts/check_live_document_size.sh` |
| `PROJECT-DATA-LOCALITY` | 1m22.8s | 24.7% | `scripts/check_project_data_locality.sh` |
| `PROOF-SEAL-CURRENCY` | 1m12.0s | 21.5% | `scripts/check_proof_seal_currency.sh` |
| `PRODUCTION-GENERICITY` | 0m50.8s | 15.1% | `scripts/check_production_genericity.sh` |
| `CLAIM-VERIFICATION` | 0m31.8s | 9.5% | `scripts/check_claim_verification.pl` |
| `KNOWLEDGE-MAP` | 0m09.9s | 2.9% | `knowledge-map/scripts/check_knowledge_map.sh` |
| `PUBLISHED-ASSERTIONS` | 0m00.6s | 0.1% | `scripts/check_published_assertions.pl` |
| `README-POLICY` | 0m00.2s | 0.0% | `scripts/check_readme_policy.sh` |
| `SECTION-ANCHORS` | 0m00.1s | 0.0% | `scripts/check_section_anchors.pl` |
| `TASK-NODE-RETENTION` | 0m00.1s | 0.0% | `scripts/check_task_node_retention.py` |
| `CONSTRAINT-PART-SPAN` | 0m00.1s | 0.0% | `scripts/check_constraint_part_span.sh` |
| `CORPUS-FRONTIER` | 0m00.1s | 0.0% | `scripts/check_corpus_frontier.sh` |
| `MEMORY-ARCH` | 0m00.0s | 0.0% | `scripts/check_memory_architecture.sh` |
| `TASK-ACCEPTANCE` | 0m00.0s | 0.0% | `scripts/check_task_acceptance.sh` |
| `RESIDUAL-ACTIONABILITY` | 0m00.0s | 0.0% | `scripts/validate_residual_actionability_contract.py` |
| `OWNERSHIP-CITATIONS` | 0m00.0s | 0.0% | `scripts/check_ownership_citations.pl` |
| **total** | **5m35.0s** | **100.0%** | — |

  **The distribution is extremely skewed, and that is the finding.** Four doctrines carry **86.9%**; five
  carry **96.4%**; the remaining **ten together cost 1.2 seconds, 0.4%**. The tree-level mean of 5m57s is
  confirmed independently at **5m35s** summed per doctrine.
  **The first table was thrown away, and how it was wrong is the durable part.** That run recorded
  `CLAIM-VERIFICATION` at **0m00.8s**; the true figure is **0m31.8s**, a **40x** understatement, because the
  checker exited early on an unrelated failure — `scripts/measure_doctrine_cost.sh` was itself untracked at
  the time, which `CLAIM-VERIFICATION` refuses as an *untracked producer-shaped path under governed source
  roots* and `PROJECT-DATA-LOCALITY` refuses for missing its `project_data_env.sh` contract text. Both
  refusals were correct and caught a real omission. **A cost measured on a failing tree is not that check's
  cost, it is the cost of its first error**, so a timing table is only valid when every row reads PASS.
  **What the table decides, and it does not decide it the way the leaf guessed.** A cheap subset does exist
  arithmetically — the ten free doctrines cost 1.2s — but it would be worthless: of the **four** doctrines
  that actually blocked a commit in this session, `LIVE-DOC-SIZE`, `PROJECT-DATA-LOCALITY`,
  `CLAIM-VERIFICATION` and `PUBLISHED-ASSERTIONS`, only the last is in the free ten. The useful cut is
  different and the numbers name it: **everything except the four heaviest costs 42.6s, 12.7% of the gate**,
  and it contains three of this session's four real blocks. The four excluded are structural checks over the
  corpus and the package boundary, which a docs-only slice does not move.
  Verification: `2026-09-17` — `bash scripts/measure_doctrine_cost.sh` run twice, the second on a tree where
  all 16 report PASS; population derived from the driver registry and cross-checked at 18 entries / 16 gate
  tier; the 5m35s sum agrees with the tree's independently measured 5m57s wall-clock mean
  Commit: `COMMIT-GATE-SINGLE-RUN.0 — measure what the doctrine gate actually costs, per doctrine`

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

- ID: `COMMIT-GATE-SINGLE-RUN.2` · Status: `pending` (opened `2026-09-17`) · Goal: **make a slice pay the
  gate once**, now that `.0` has measured what it costs. The approval this tree exists to carry is the
  director's `2026-08-31` one — *for a slice that touches no Rust, run only a focused subset by hand and
  let the hook run the full driver once at commit* — and `.0`'s table finally says what that subset is.
  **The candidate, with its own numbers**: keep the manual pass at everything except `LIVE-DOC-SIZE`,
  `PROJECT-DATA-LOCALITY`, `PROOF-SEAL-CURRENCY` and `PRODUCTION-GENERICITY` — **42.6s against 5m35.0s,
  12.7%** — which still covers three of the four doctrines that blocked a commit in the `2026-09-17`
  session. The alternative `.0` was asked to weigh, deleting step 8's manual run outright, is cheaper
  still and loses the early signal entirely; the hook blocks either way, so this is a question about
  iteration cost, not about enforcement.
  **Do not implement either by editing `COMMIT.md` prose alone.** A subset that lives only in an
  instruction is a subset each session re-derives from memory, which is how `.1`'s wrong-flag class
  happens; whatever is chosen needs an executable entry point that names its own membership, and that
  entry point must derive from the driver registry the way `scripts/measure_doctrine_cost.sh` does, so a
  newly registered doctrine is never silently outside the fast set.
  **Sequenced after `.1`**, deliberately: `.1` decides whether a hand-run check can be trusted at all, and
  a fast set that can silently no-op is worse than no fast set.
  Prerequisite: `.0` (done); `.1` should land first.
  Verification: pending
  Commit: pending

## Current Frontier

Ordered; PNT selects the first eligible leaf.

0. `COMMIT-GATE-SINGLE-RUN.1` — a hand-run check that can silently be a no-op. First, because a fast set
   that can silently pass is worse than none, and `.0`'s table is what makes a fast set worth having.
1. `COMMIT-GATE-SINGLE-RUN.2` — spend the measurement: one executable entry point for the 12.7% subset,
   or delete step 8's manual run. Blocked on `.1` by the reasoning above, not by evidence.

`COMMIT-GATE-SINGLE-RUN.0` is `done` (`2026-09-17`): four doctrines carry 86.9% of a 5m35.0s gate and the
cheapest ten carry 0.4%, so the subset question has an answer and it is not the one the leaf guessed.

## Decisions

- `2026-09-15` (tree opened): **the hook is the leg that must not move.** `MEMORY_ARCHITECTURE.md` §9
  layers discovery / self-check / git hook / CI, and notes the hosted CI is manual-only. With CI not
  firing automatically, the pre-commit hook is the only un-bypassable enforcement in practice, so the
  redundancy must be removed from the manual side.
- `2026-09-15`: **this is background hygiene, not feature work.** Measured at ~1 hour per ten-commit
  session it pays off across a long run and not within one slice; schedule it accordingly rather than
  ahead of product work.
