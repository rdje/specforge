---
id: a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest
title: A doctrine subset is only honest while another leg pays for the rest, so it asserts that leg
answers:
  - "how do I run a cheaper doctrine gate before committing"
  - "does COMMIT.md still require a full manual check_doctrines.sh run"
  - "why does check_doctrines.sh --fast refuse to run"
  - "is a green --fast a green doctrine gate"
  - "which doctrines does --fast skip"
  - "where is the fast doctrine subset declared"
  - "why is the fast doctrine subset an exclusion list rather than an inclusion list"
  - "what happens to the fast subset when a new doctrine is registered"
date: 2026-09-17
status: current
tags: [doctrine-enforcement, commit-workflow, gates, subset]
evidence: scripts/check_doctrines.sh; COMMIT.md; .githooks/pre-commit; docs/tasks/COMMIT-GATE-SINGLE-RUN.md
reverify: bash scripts/check_doctrines.sh --fast
---

`COMMIT.md` step 8 and `.githooks/pre-commit` both ran the complete driver, so every slice paid the
doctrine gate **twice** and only the second run could block. The redundancy came off the manual side:
step 8 now runs `scripts/check_doctrines.sh --fast` for a slice that changes only Markdown, task files,
or doctrine records, and the hook runs the complete driver once at commit.

**The subset is declared as an exclusion, and the direction is the whole point.** `FAST_EXCLUDE=(...)`
sits beside `DOCTRINES=(...)` in the driver and names the four doctrines `--fast` leaves out, so a
doctrine added to the registry is in the fast set *automatically*. An inclusion list inverts the failure:
every newly registered doctrine would sit silently outside the fast set, and nothing would say so. The
exclusion is meta-checked against the registry on **every** run, not only under `--fast`, so renaming a
doctrine without renaming it here refuses at the next gate instead of quietly widening the subset.

**A subset asserts the leg that pays for the rest.** `--fast` is safe only because the pre-commit hook
runs the complete driver afterwards, and that hook is a per-clone opt-in
(`git config core.hooksPath .githooks`). In a clone where it was never wired, a subset run would be the
only doctrine enforcement that ever happened **and it would report PASS**. So the driver resolves
`core.hooksPath`, requires an executable `pre-commit` under it, and requires that file to invoke
`check_doctrines.sh`; otherwise `--fast` exits 2 rather than producing a green line.

**A green `--fast` is an early signal, never a verdict.** Of the five doctrines evidenced blocking a
commit on `2026-09-17` it runs `CLAIM-VERIFICATION`, `PUBLISHED-ASSERTIONS` and `KNOWLEDGE-MAP`, and
omits `LIVE-DOC-SIZE` and `PROJECT-DATA-LOCALITY` — the two that actually fired. That is a real trade,
not a free win, which is why `COMMIT.md` routes any slice touching Rust, a registered enforcer, or
anything under `scripts/` to the full driver, and why the run prints both the `FAST SUBSET … this is NOT
the gate` banner and a second line naming every doctrine it did not run: the caveat has to travel with
any line copied into a task record.

Membership is `COMMIT-GATE-SINGLE-RUN.0`'s one surviving finding — the same four doctrines were costliest
in all three of its runs. Its **shares are withdrawn** (measured at load average 12.95, and three runs of
one tree totalled 335s / 429s / 524s), so no share is quoted in the driver, in `COMMIT.md`, or in the
book. `COMMIT-GATE-SINGLE-RUN.0a` re-measures on a machine proved idle and can move the membership by
editing four lines and nothing else.
