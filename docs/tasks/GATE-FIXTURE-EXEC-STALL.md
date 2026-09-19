# GATE-FIXTURE-EXEC-STALL: the commit gate's runtime is hostage to the host's executable-assessment daemon

## Metadata

- Tree ID: `GATE-FIXTURE-EXEC-STALL`
- Status: `active`
- Roadmap lane: `R16` (doctrine enforcement / developer throughput)
- Created: `2026-09-19`
- Last updated: `2026-09-19`
- Owner: repo-local workflow

## Goal

Make the doctrine gate's wall-clock cost a function of the work it does, not of how many **new**
executable files it asks the operating system to assess. The gate must stay honest — no check is
weakened, skipped, or made conditional — while ceasing to pay a per-fixture security-assessment toll
that has no relationship to what it is proving.

## Non-Goals

- Changing the host's security posture. Excluding a path from Gatekeeper/XProtect is the machine
  owner's decision, is not repository-portable, and would hide the cost rather than remove it.
- Weakening `test_live_document_size.pl`. Its 113 checks are the reason `LIVE-DOC-SIZE` is trusted;
  this tree must not trade coverage for speed.
- Making the gate conditional on the environment. A gate that runs a different set of checks on a
  slow host is not the same gate.

## The finding (measured `2026-09-19` during `CORPUS-CHAIN-CURRENCY.10a`)

A `git commit` sat in `.githooks/pre-commit` for **41 minutes** and was aborted. The whole tree —
`git commit` → `pre-commit` → `check_doctrines.sh` → `test_live_document_size.pl` →
`check_live_document_size.pl` → `bash <fixture>/scripts/currency-ok.sh` — was sleeping at a combined
**0.03 s of CPU**. The innermost blocked process was a fixture script whose entire body is
`#!/usr/bin/env bash` + `exit 0`.

**It is not a deadlock, and it is not this repository's logic.** Measured directly:

| exec | latency |
| --- | ---: |
| first exec of a brand-new script | **127,254 ms** |
| next brand-new script | 223 ms |
| next brand-new script | 177 ms |
| re-exec of an already-assessed script | **8 ms** |

Concurrently `XprotectService` held 75.1% CPU and `syspolicyd` 47.3% (152 hours accumulated). macOS
assesses an executable on first exec; when that daemon is backed up, the assessment blocks the caller
for minutes. The repository's processes were waiting, not working.

**The cost is quantified by the gate's own numbers.** A full `scripts/check_doctrines.sh` run on the
same tree, immediately afterwards: **18m07s wall against 3m55s user CPU**. All 16 executed doctrines
PASS. Roughly 14 of those 18 minutes bought nothing.

**Why the repository owns a share of it.** `new_fixture` builds each fixture with
`tempdir('live-document-size-tests.XXXXXX', DIR => $generated, CLEANUP => 1)` and writes its verifier
scripts inside. Every fixture therefore presents the operating system with a **new** executable at a
**new** path — about 113 of them per run — so the gate requests ~113 first-exec assessments per run
for three distinct scripts whose content never varies. That multiplier is ours, and it is the part we
can remove without touching anyone's security settings.

**Second-order damage, and it is the reason this is not merely slow.** The stall is
indistinguishable from a hang: zero CPU, no output, no progress marker. It cost this session an
aborted commit, a killed process tree, and a wrong first hypothesis (a pipe deadlock). It is also
intermittent — once the daemon drains, the same command finishes in 1m37s — so it will be
re-diagnosed from scratch by the next session that meets it unless this tree exists.

## Acceptance Criteria

- Fixture verifier scripts are presented to the operating system as a bounded, reused set rather than
  one new executable per fixture, with no change to what any check asserts.
- `test_live_document_size.pl` still reports its full declared check count and still goes RED on every
  case it goes RED on today.
- The gain is measured the way the loss was: wall clock **and** user CPU, cold and warm, before and
  after, on this host.
- The stall is made legible where a future session will meet it, so the next occurrence is recognised
  in seconds rather than re-derived over 40 minutes.

## Task Tree

- ID: `GATE-FIXTURE-EXEC-STALL`
  Status: `active`
  Goal: make gate wall-clock a function of gate work
  Children: `.1`, `.2`, `.3`

- ID: `GATE-FIXTURE-EXEC-STALL.1`
  Status: `pending`
  Goal: **measure the multiplier before changing anything.** Count the distinct executable paths the
  gate creates and execs in one full run, and the distinct *contents* behind them. The finding above
  asserts ~113 paths for 3 contents from one test; confirm it, and enumerate every other gate-tier
  check that writes-then-execs a fixture, because `test_live_document_size.pl` is unlikely to be the
  only one and a fix that lands on one of several is a fix that does not show up in the wall clock.
  Acceptance: an exact path/content census per producing check, plus a cold/warm wall-clock and user-
  CPU baseline for the full driver on this host.
  Verification: `pending`
  Commit: `pending`

- ID: `GATE-FIXTURE-EXEC-STALL.2`
  Status: `pending`
  Goal: **present a bounded set of verifier executables instead of one per fixture**, and prove the
  checks are unchanged. The shape to evaluate first is a stable per-run script directory the fixtures
  reference, so identical content is written once and assessed once; the constraint is that fixtures
  must stay isolated from each other, which is why this is a leaf and not an edit.
  Acceptance: identical declared check count, identical RED behaviour on every self-test case, and a
  measured drop in first-exec assessments; the before/after numbers recorded here.
  Prerequisite: `.1`.
  Verification: `pending`
  Commit: `pending`

- ID: `GATE-FIXTURE-EXEC-STALL.3`
  Status: `pending`
  Goal: **make the stall self-identifying.** A gate that blocks at zero CPU with no output is
  indistinguishable from a hang, and that ambiguity is what cost this session 40 minutes. Emit a
  progress marker per doctrine and, when a step exceeds a generous threshold, name the host condition
  and how to confirm it (`XprotectService`/`syspolicyd` CPU, the fresh-vs-cached exec probe) so the
  next session reaches the answer in seconds. Route the diagnostic into `TOOLBOX.md`.
  Acceptance: an induced stall produces a message that names the cause; the toolbox entry reproduces
  the exec-latency probe.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `GATE-FIXTURE-EXEC-STALL.3` | `pending` | cheapest, and it is the leaf that stops the next session paying the 40-minute diagnosis; it needs nothing from `.1` |
| 2 | `GATE-FIXTURE-EXEC-STALL.1` | `pending` | the census that tells `.2` whether one producer or several own the multiplier |
| 3 | `GATE-FIXTURE-EXEC-STALL.2` | `pending` | the actual remedy; it must not start before `.1` says where the cost lives |

## Blockers

- None. The host condition is not a blocker: the repository-side multiplier is removable without it.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-19` | finding | exec-latency probe (fresh vs cached), `ps` CPU accounting on the blocked tree, full `check_doctrines.sh` timed | fresh first exec **127,254 ms** vs cached **8 ms**; blocked tree at **0.03 s CPU / 41 min**; full gate **18m07s wall / 3m55s CPU**, all 16 executed doctrines PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| finding | `CORPUS-CHAIN-CURRENCY.10a` | found while committing that leaf; the gate stall is not that tree's subject |

## Changelog

- `2026-09-19`: Created. A 41-minute pre-commit stall was root-caused to host executable assessment,
  not to repository logic — and to a repository-owned multiplier of ~113 fresh executables per gate
  run for three unchanging script contents.
