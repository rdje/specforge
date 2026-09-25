# FSMGEN-REFRESH-INTEGRATE-10: refresh the FSMGen pin to land two fixes SpecForge verified upstream

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-10`
- Status: `active`
- Roadmap lane: `R6` (× `R15`/`R16` — FSMGen strict is the executable-intent boundary)
- Created: `2026-09-25`
- Last updated: `2026-09-25`
- Owner: repo-local workflow
- Owner directive (`2026-09-24`): "you can update FSGEM submodule if you want."

## Goal

Advance the read-only `subs/fsmgen` gitlink from `54ac1c22d5314943027494f4c270f9c612e62704` to the published
`origin/main` tip. Two FSMGen fixes that SpecForge needs sit after the pin, each verified on complete
`git archive` exports of FSMGen's history (the checkout is never touched):

- `f16130a4f` (*preserve ISF reset policy through lowering*, `2026-09-25`): an `.isf` reset keeps the polarity
  its clause states. Before it, including at `07f7e8333`, an `async` reset always came out active low and `--strict`
  keyed the polarity on the name (`EXTRACTION-REPRESENTATION-GAPS.21`).
- `c6b8bc783` (*preserve authored lowering failure context*, `2026-09-24`): `--strict` refuses an undeclared rule
  read, in a value (`FSMGEN-REQUESTS.2`) or in a guard (*operand 'bogus_name' read by rule 'r1' guard*,
  `EXTRACTION-REPRESENTATION-GAPS.23`). `be25e7205` (*reject undeclared ISF rule reads*) alone did not yet refuse.

Audit the published-contract delta as cycle 9 did, and integrate only what that audit establishes.

## Non-Goals

- Modifying or committing FSMGen source from this repository, or reading FSMGen's implementation: published
  contract surfaces, commit subjects and black-box CLI runs only.
- Pinning the sibling checkout's HEAD when it is ahead of `origin`: a pin must be fetchable by anyone.
- Weakening a strict canary or expectation to hide an incompatible change.

## Acceptance Criteria

- The fetched tip, ancestry, commit count and changed published-contract files are recorded, and the tip contains
  `f16130a4f` and `c6b8bc783`.
- `subs/fsmgen` is detached, clean, exactly at the tip; only the parent gitlink changes.
- Every changed published-contract surface relevant to what SpecForge emits is read, each with an adopt /
  incompatible / no-action conclusion.
- The Rust strict canaries pass on the new pin; every emitted `.isf` (the 2 written files and the 27 live
  `source_text`s) gives the same strict verdict as on the old pin, or each difference is explained and owned.
- Every open request bundle is re-run on the new pin; `FSMGEN-REQUESTS.2` and `EXTRACTION-REPRESENTATION-GAPS.21`
  close on it, and any other request the new pin answers is updated.
- Every declared pin copy (feedback channel, derived-state authority, Knowledge Map, mdBook, live docs) agrees
  with the gitlink.
- Full CI passes; committed through `COMMIT.md`.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-10` · Status: `active` · Goal: refresh cycle 10 — pin, audit, verify, integrate,
  close. Children: `.1`.
- ID: `FSMGEN-REFRESH-INTEGRATE-10.1` · Status: `open` (`2026-09-25`) · Goal: fetch and audit the exact
  upstream range, bump the gitlink, run the strict canaries, the emitted-`.isf` census and every open request
  bundle on both pins, reconcile pin copies, close `.21` and `FSMGEN-REQUESTS.2`.

## Current Frontier

1. `.1`.

## Decisions

- `2026-09-25` — no bug report for either defect: both were found on SpecForge's pin and verified fixed upstream
  before a bundle was forwarded, so the remedy is this refresh, not a report.

## Open Questions

None.

## Blockers

None.

## Verification Log

- `2026-09-25` — exports: at `07f7e8333~1` and `07f7e8333`, `(reset (rst_n async active_high))` passes `--strict`
  with `reset_active_level` 0 and four other stated combinations are refused; at `f16130a4f` all eight probe cases
  pass `--strict` with the stated level. At `be25e7205~1` and `be25e7205` the undeclared guard and value cases pass
  `--strict`; binary search over exports puts the first refusal at `c6b8bc783` for both.

## Commit Log

None yet.

## Changelog

- `2026-09-25` — tree opened by `EXTRACTION-REPRESENTATION-GAPS.21`/`.23`.
