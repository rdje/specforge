# LIVE-DOCUMENT-PRESSURE-HEADROOM — contract encoding

- Part ID: `contract-encoding`
- State: `active`

## Derived-state file hygiene

A doctrine contract's bytes are written by whichever session touched it last, and its encoder
is not a property of the file. These leaves measure that and gate it. Split out of
[current and open work](current-and-open-work.md) on `2026-09-21` because that part had reached
its mandatory byte rollover and the alternative was deleting evidence to fit, which `.25` calls
a policy defect rather than an author problem.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.39`
  Status: `done` (`2026-09-21`; MEASUREMENT, no code; opens `.39a`)
  Goal: **a contract's JSON encoder is the last writer's reading, not a property of the file.** `.23`
  published one per file and `.24a` and the fact card `[[live-surface-edit-bookkeeping-chain]]` repeat
  it, so every session is sent to a literal before editing a contract.
  Acceptance: decide whether a per-file encoder is stable enough to carry; if not, remove the literal
  rather than refresh it (`CLAIM_VERIFICATION.md` §3 — derived or gated, never carried).
  **MEASURED `2026-09-21`: it is not.** Replaying every revision of every contract and deriving the
  encoder at each — **31 flips across 12 of the 23 contracts with history**. `fact_card_catalog.json`
  alone flipped **8 times in 217 revisions** and is `indent=2` today; the card's `indent=1` was **true
  when written on `2026-09-14` and false from `2026-09-20`**, so this is drift, not an authoring error.
  **It went stale by doing what the card exists to prevent**: `65e2b4d2` rewrote that file with **286
  changed lines to carry 12 changed fields** (first published as 288: a `grep -cE '^[+-]'` counts the
  patch headers too — read churn from `--numstat`).
  **Correction to this leaf's own first reading.** `HEAD`, `HEAD~5`, `HEAD~20` and the six oldest
  revisions all returned `indent=2`, and the first conclusion was that the card had never been right.
  All nine samples fall outside the one week in which it was. **Nine agreeing samples are not a
  census**, and "always wrong" versus "stale since a named commit" is the difference between fixing a
  card and gating a file.
  **Landed in this active part**: the first attempt edited `knowledge-and-fact-plane.md` inside
  its marked region and the contract check refused, so `.23`/`.24a` stand as dated readings.
  Non-goal: reformatting contracts back; a repository-wide encoder (`.23` found none).
  Prerequisite: none; surfaced paying the bookkeeping chain for `KG-ISF-COMPLETENESS.5.vi`.
  Verification: `card and node carry no per-file literal; flip census re-runnable from git log plus a derived encoder; drift commit 65e2b4d2 and its 286-lines/12-fields ratio re-derived from --numstat; knowledge-map OK; all gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.39 — a contract's encoder is the last writer's reading, not the file's property`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.39a`
  Status: `done` (`2026-09-21`; CODE; LANDED as the `CONTRACT-ENCODING` doctrine)
  Goal: **refuse a silent reformat of a doctrine JSON contract** — 31 flips, each a commit whose real
  change was buried in a re-encode. An advisory card cannot stop this; `DOCTRINE_ENFORCEMENT.md` says a
  doctrine worth stating is worth gating.
  Acceptance: derive each changed `doctrine/**/*.json`'s encoder at `HEAD` and in the worktree and
  **refuse when they differ**, so a re-encode is declared rather than slipped in. Measure against the
  31 historical flips FIRST — how many it catches, how many were declared migrations, what the escape
  hatch costs. **A gate refusing most of its population on arrival is a policy defect** (`.25`), so the
  hatch is part of the acceptance.
  Non-goal: normalising contracts to one encoder; the hand-authored four (`.23`: re-encoding expands
  them 9-27% and destroys their layout).
  Prerequisite: `.39`.
  **MEASURED FIRST, AND THE OBVIOUS DESIGN IS REFUTED.** Replayed against the 31 historical flips,
  the diff-based rule fires on **31 of 31** and **13 of them declare a re-encode** in their commit
  message — `.25`'s policy defect, on a proxy (a keyword scan) nothing can gate on. Independently, a
  diff is meaningless on a fresh CI checkout, so it would be inert where it is un-bypassable.
  **LANDED as an ABSOLUTE invariant**: `doctrine/contract_encoding/encoders.jsonl` declares each
  contract's encoder and `scripts/check_contract_encoding.py` derives it from the bytes and refuses a
  disagreement. A reformat fails; a deliberate migration edits one registry line, visible in the diff.
  **The escape hatch costs a line, not an authority record.** It also retires a carried constant
  (`CLAIM_VERIFICATION.md` §3): the registry is now the answer to "which encoder wrote this file",
  and the gate keeps it true. 30 contracts declared, **4 hand-authored** — the same four `.23` named,
  re-derived independently. Full design rationale is the script's docstring, not repeated here.
  **Still permits** a rewrite under the same encoder: this gates encoding, not churn (`--report`
  prints churn-per-field for that).
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.39a — declare the encoder, and a silent re-encode cannot pass`
