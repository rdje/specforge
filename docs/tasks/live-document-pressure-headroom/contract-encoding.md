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

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.40`
  Status: `done` (`2026-09-23`; CODE + DOC; opened by the startup re-read of the containment precedent)
  Goal: **re-read the FSMGen adoption guide, which moved after `.8a` read it, and own what it exposes.**
  `.8a` recorded SHA-256 `6f10b950…` (upstream `70ba62f35`, `2026-08-09`); the guide is now `8f77fa39…` at
  `727e0d086` (`2026-09-05`). `git diff 70ba62f35 727e0d086` over the guide adds exactly two rules.
  **Rule A — bound task narrative separately; count markup; reject string-encoded numeric evidence.** The
  bounding half was already implemented: each active-task-evidence contract routes every leaf, pins the
  pre-migration source capsule by commit, blob and SHA-256, requires the root to link its index exactly once
  (`check_active_task_evidence.pl`, *"bounded active root must link index … exactly once"*), and measures parts
  in raw bytes. **The type half was a real gap.** A probe set one part's `"bytes" : 42768` to `"42768"` in
  `claim_verification_task_evidence.json`, and a surface transition's `bytes_total` to `"1643"`: both
  checkers passed. Every loader reads a digit regex or compares numerically, and both accept a quoted value.
  **Rule B — a delegating test must not copy the live registry's cardinality.** Already held: the only exact
  surface count in any test is `test_live_document_size.pl`'s `11 governed surfaces`, and that is its own
  synthetic fixture, which the rule explicitly permits.
  **LANDED: one pass in `check_live_document_size.pl`, over every registry beside the surface registry**
  (19 today; the fixture's `control/` in tests, so both run the same code). It decodes each afresh, because
  JSON::PP marks a number `IOK`/`NOK` and a string `POK` and the first use of either sets the other flag — so
  no downstream reader can recover the type, and one pass is the only place it can be checked. Two rules,
  both total: a decimal-numeral string of at most 15 integer digits is refused under any key (every count is
  below 10^15; the shortest identity held is a 16-hex fingerprint, so an all-decimal identity is never
  mistaken for one), and a key that is a JSON number anywhere must be one everywhere (which also refuses
  `"1,643"`). **Measured before gating: 0 of 19 registries violate either rule**, so it refuses nothing on
  arrival. Residual, stated: an integer too large for a native integer decodes as a string and is refused
  as one; no bound comes near it.
  Non-goal: the claim registries (`published_assertions` quotes a value from prose as a string, legitimately);
  a numeric-field schema per registry, which the population-derived rule makes unnecessary.
  Verification: `test_live_document_size.pl` **120/120** (declared count re-derived 116 -> 120: three RED —
  quoted numeral in the surface registry, in a sibling, and a non-numeral string in a numeric field — and one
  positive for an all-decimal identity beside ordinary text). Attributed by neutralising the pass: exactly
  the three RED cases fail. Both live probes now refuse with the field and file named; the live tree is green.
  Doctrine: `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` gains both rules as neutral text and the dated reading.
  **Book, and a stale contract number found on the way.** `reference/live-docs.md`'s control-plane paragraph
  owns what the registries refuse, so it gains the type refusal. The same paragraph said, in the present
  tense, *"A 55-case fixture suite … Another 44 cases …"*; re-derived, the suites declare **120**, **47** and
  **25**. Every suite already gate-holds its own total, so the book now names the coverage and drops the
  counts instead of carrying three new ones. The line-29 census region left with its quantity: its
  `incomplete` record is removed and the book denominator is rolled **600 -> 599** (re-derived by
  `check_book_quantitative_claims.pl --check`, not edited to match). **Surfaced, owned next:** the census
  grammar never saw `55-case` (line 27, and line 118's dated one) — a hyphenated count is invisible to it — and
  `[[book-quantitative-candidate-vocabulary]]`'s `reverify` still expects `42 … 539`; both go to
  `CLAIM-VERIFICATION-ADOPTION.24`. Fact card `[[containment-numbers-are-type-checked-at-decode]]`; catalog
  346 -> 347 via `--accept-value`.
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.40 — a containment number is a JSON number`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.41`
  Status: `done` (`2026-09-23`; CODE, one line; found re-verifying a published finding)
  Goal: **a self-test's failure message carried the boundary it no longer tests.** `check_fact_card_catalog.pl`'s
  overflow case renders `max_cards + 1` cards — **393** today (`max_cards` 392 = the canonical surface's 394
  files minus README and INDEX) — but its `die` message said *"337-card ceiling did not fail closed"*, the
  boundary of the 336-card profile it replaced. The test was right; the diagnostic would have named the wrong
  boundary on the day it fired. The message now derives its number from the same limit the test uses.
  **Ordering, stated:** the one-line edit was made before this leaf was written, in the same working session.
  Non-goal: `reference/containment-adoption.md`'s *"A full 336-card render … 337th must fail closed … 58 focused
  cases"* — dated landing narrative (its neighbour region is adjudicated `dated_boundary_evidence`), correct as of
  its landing.
  Verification: `perl scripts/check_fact_card_catalog.pl --self-test` 60/60; `rg '337-card' scripts` returns
  nothing.
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.41 — the overflow message names the boundary it tests`
