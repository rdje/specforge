# FSMGEN-REFRESH-INTEGRATE-10: refresh the FSMGen pin to land two fixes SpecForge verified upstream

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-10`
- Status: `active`
- Roadmap lane: `R6` (× `R15`/`R16` — FSMGen strict is the executable-intent boundary)
- Created: `2026-09-25`
- Last updated: `2026-09-26`
- Owner: repo-local workflow
- Owner directive (`2026-09-24`): "you can update FSGEM submodule if you want."

## Goal

Advance the read-only `subs/fsmgen` gitlink from `54ac1c22d5314943027494f4c270f9c612e62704` to the published
`origin/main` tip. Two FSMGen fixes that SpecForge needs sit after the pin in FSMGen's development line, each
verified on complete `git archive` exports of FSMGen's history (the checkout is never touched):

- `f16130a4f` (*preserve ISF reset policy through lowering*, `2026-09-25`): an `.isf` reset keeps the polarity
  its clause states. Before it, including at `07f7e8333`, an `async` reset always came out active low and `--strict`
  keyed the polarity on the name (`EXTRACTION-REPRESENTATION-GAPS.21`).
- `c6b8bc783` (*preserve authored lowering failure context*, `2026-09-24`): `--strict` refuses an undeclared rule
  read, in a value (`FSMGEN-REQUESTS.2`) or in a guard (*operand 'bogus_name' read by rule 'r1' guard*,
  `EXTRACTION-REPRESENTATION-GAPS.23`). `be25e7205` (*reject undeclared ISF rule reads*) alone did not yet refuse.

A third fix found on `2026-09-26` (`FSMGEN-REQUESTS.6`) must be in the tip too:

- `a7bf398ac` (the input-role repair FSMGen cites in its `2026-09-26` response): an interface input that no
  transaction reads stays a port of the generated module. On the pin such an input is dropped. 123 of the 126
  inputs declared by the five emitted `.isf` files are absent from their generated SystemVerilog (`PRDATA`,
  `PREADY`, `HRDATA`, `ACK_I`, …), and every file still passes `--strict`. On a complete export of `5be6a3c08`,
  0 of 126 are absent; outputs are complete (255 of 255) on both.

Audit the published-contract delta as cycle 9 did, and integrate only what that audit establishes.

## Non-Goals

- Modifying or committing FSMGen source from this repository, or reading FSMGen's implementation: published
  contract surfaces, commit subjects and black-box CLI runs only.
- Pinning the sibling checkout's HEAD when it is ahead of `origin`: a pin must be fetchable by anyone.
- Weakening a strict canary or expectation to hide an incompatible change.

## Acceptance Criteria

- The fetched tip, ancestry, commit count and changed published-contract files are recorded, and the tip contains
  `f16130a4f`, `c6b8bc783` and `a7bf398ac`.
- `subs/fsmgen` is detached, clean, exactly at the tip; only the parent gitlink changes.
- Every changed published-contract surface relevant to what SpecForge emits is read, each with an adopt /
  incompatible / no-action conclusion.
- The Rust strict canaries pass on the new pin; every emitted `.isf` (the 2 written files and the 27 live
  `source_text`s) gives the same strict verdict as on the old pin, or each difference is explained and owned.
- Every port an emitted `.isf` declares is a port of its generated module on the new pin (a strict pass does not
  show this; `scripts/measure_isf_realized_ports.py <export>` inventories it), or each absence is owned.
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

1. `.1` — blocked until FSMGen publishes `f16130a4f`, `c6b8bc783` and `a7bf398ac` (see Blockers).

## Decisions

- `2026-09-25` — no bug report for either defect: both were found on SpecForge's pin and verified fixed in FSMGen's
  development line before a bundle was forwarded, so the remedy is this refresh, not a report.

## Open Questions

None.

## Blockers

- `2026-09-25` — **the fixes are not published.** `git fetch --prune origin` and `git ls-remote --symref origin
  HEAD` agree that `origin/main` is `54ac1c22`, SpecForge's pin. `f16130a4f` and `c6b8bc783` are only in the
  sibling checkout, whose HEAD `354844fe1` is 168 commits ahead of `origin/main`, unpushed. Only a published tip
  may be pinned (cycle 9), so `.1` waits for FSMGen to publish them.
- `2026-09-26` — unchanged: `git ls-remote origin refs/heads/main` is `54ac1c22`; the sibling checkout's HEAD is
  `5be6a3c08`, with `f16130a4f`, `c6b8bc783` and `a7bf398ac` among its unpushed commits.

## Verification Log

- `2026-09-25` — exports: at `07f7e8333~1` and `07f7e8333`, `(reset (rst_n async active_high))` passes `--strict`
  with `reset_active_level` 0 and four other stated combinations are refused; at `f16130a4f` all eight probe cases
  pass `--strict` with the stated level. At `be25e7205~1` and `be25e7205` the undeclared guard and value cases pass
  `--strict`; binary search over exports puts the first refusal at `c6b8bc783` for both.

- `2026-09-26` — complete exports of `54ac1c22` (the pin) and `5be6a3c08` (FSMGen's development HEAD), each with
  a known-good strict check (exit 0, no stderr): SystemVerilog generated for the five current chains' emitted
  `.isf` files lacks 123 of 126 declared inputs on the pin and none on `5be6a3c08`; 0 of 255 outputs absent on
  both. `git ls-remote origin refs/heads/main` is still `54ac1c22`.

## Commit Log

None yet.

## Changelog

- `2026-09-25` — tree opened by `EXTRACTION-REPRESENTATION-GAPS.21`/`.23`.
- `2026-09-25` — `.1` blocked: the published tip is still the pin; both fixes are unpushed.
- `2026-09-26` — `a7bf398ac` added to what the tip must contain, and a realized-port acceptance item
  (`FSMGEN-REQUESTS.6`); still blocked: `origin/main` is `54ac1c22`.
