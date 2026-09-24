# FSMGEN-REFRESH-INTEGRATE-9: refresh the FSMGen pin to upstream and audit the contract delta

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-9`
- Status: `done`
- Roadmap lane: `R6` (× `R15`/`R16` — FSMGen strict is the executable-intent boundary)
- Created: `2026-09-24`
- Last updated: `2026-09-24`
- Owner: repo-local workflow
- Owner directive (`2026-09-24`): "you can update FSGEM submodule if you want."

## Goal

Advance the read-only `subs/fsmgen` gitlink from `c0d8b668db2527108d1c23c20d184400c51efea6` to the fetched
`origin/main` tip, audit the intervening change to FSMGen's **published** downstream contract, verify
SpecForge's strict executable-intent boundary against the exact new binary, and integrate only the product or
documentation truth that audit establishes.

## Non-Goals

- Modifying or committing FSMGen source from this repository.
- Reading FSMGen's implementation. Unlike cycle 8, this audit is confined to published contract surfaces
  (ISF specs, public-interface contract, downstream integration spec, the book's ISF chapters, the capability
  manifest, FSMGen's reply ledger to SpecForge) plus black-box runs of its CLI (standing doctrine: submodules
  are black boxes).
- Weakening a strict canary or expectation to hide an incompatible change.
- Implementing new FSMGen capabilities SpecForge could adopt; each becomes its own owned leaf.

## Acceptance Criteria

- The fetched tip, ancestry, commit count and the changed published-contract files are recorded.
- `subs/fsmgen` is detached, clean, exactly at the tip; only the parent gitlink changes.
- Every changed published-contract surface relevant to what SpecForge emits is read, each with an adopt /
  incompatible / no-action conclusion.
- The Rust strict canaries pass on the new pin; every emitted `.isf` (the 14 written adapters and the 27 live
  `source_text`s) gives the same strict verdict as on the old pin, or each difference is explained and owned.
- The three `FSMGEN-REQUESTS` bundles are re-run on the new pin and their conclusions still hold, or are updated.
- Every declared pin copy (feedback channel, derived-state authority, Knowledge Map, mdBook, live docs) agrees
  with the gitlink.
- Full CI passes; committed through `COMMIT.md`.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-9` · Status: `done` · Goal: refresh cycle 9 — pin, audit, verify, integrate,
  close. Children: `.1`.
- ID: `FSMGEN-REFRESH-INTEGRATE-9.1` · Status: `done` (`2026-09-24`) · Goal: fetch and audit the exact
  upstream range, bump the gitlink, run the strict canaries and the emitted-`.isf` census on both pins,
  reconcile pin copies, close.

## Acceptance Checklist (enforced) — `FSMGEN-REFRESH-INTEGRATE-9.1`

- [x] **REPRODUCE / MEASURE** — pruned fetch and an independent `ls-remote --symref` agree on `origin/main`
  `54ac1c22d5314943027494f4c270f9c612e62704` (2026-09-22); the entry pin is its ancestor across 818 commits.
- [x] **ROOT CAUSE (WHY + WHERE)** — published-contract delta read (no implementation read): `docs/ISF_SPEC.md`,
  `ISF_PUBLIC_INTERFACE_CONTRACT.md`, `ISF_DOWNSTREAM_INTEGRATION_SPEC.md`, `SPECFORGE_FEEDBACK_RESPONSE.md`,
  `DOWNSTREAM_ISSUE_REPORTING.md` unchanged; `docs/isf-spec/` gains one test link; of 24 changed book files the
  ISF ones are 13a (a named drive's `{drive}_{param}` payload is documented target-width, not 1 bit), 13b/13h
  (loop timing formulas corrected to `N × (body + 1) + 2`), 10 (FSMGen's own CI/VIAL gate) and the backlog count;
  the rest is HIAL/VIAL verification architecture. Classification: contract-neutral for what SpecForge emits.
- [x] **ADDRESSED (verified)** — `subs/fsmgen` detached, clean, exactly at `54ac1c22`; only the parent gitlink
  changed; the three declared current-pin copies (`docs/FSMGEN_FEEDBACK.md`, the feedback contract's required
  literal, both `fsmgen_gitlink_copies` markers) moved with it.
- [x] **NO REGRESSION** — strict census of every `.isf` SpecForge produces (14 written adapters + 27 live
  `source_text`s) on a complete `git archive` of `c0d8b668` and on the new pin, after a known-good check
  succeeded on both: 41 of 41 identical verdicts (22 pass, 19 fail identically); 7 of 7 strict canaries pass
  (`cargo test --workspace --lib --exclude specforge-production-graph fsmgen_strict`); the three
  `FSMGEN-REQUESTS` bundles reproduce unchanged; full CI (see verification log).
- [x] **GENERICITY (ADR 0006)** — no product code, emitted `.isf`, exception or pin-specific logic added.
- [x] **LOCKSTEP** — book `pipeline/intentir.md` (how the pin moves and how a move is checked), feedback
  channel, derived-state authority, `PROCESS.md` §2 comparison rule, request bundles, `CHANGES.md`, `MEMORY.md`.

## Current Frontier

Complete.

## Decisions

- Entry gitlink: `c0d8b668db2527108d1c23c20d184400c51efea6`. New gitlink: `54ac1c22d5314943027494f4c270f9c612e62704`.
- Pin the **published** tip, not the sibling FSMGen checkout's HEAD (`5a44ad49`/`67c8b36d`), which was ahead of
  `origin` with unpushed commits: a pin must be fetchable by anyone.
- **The first census was invalid setup, not an FSMGen finding.** An old-revision copy made of selected runtime
  directories lacked `specs/pplugin.spec`, so every old run failed ("No spec pplugin") and 41 false differences
  appeared; a second copy that added `specs/` was still partial. FSMGen's guidance (relayed by the owner):
  recreate the complete old revision from that exact revision and confirm a known-good strict check on both
  installations first. Done: complete `git archive c0d8b668` (79 MB), known-good `success:true` with empty stderr
  on both, then 41/41 identical. The rule is now `PROCESS.md` §2.
- **The 19 identical failures are not actionable here.** They are signal-less documents (guides, PHY and
  mechanical specs) whose adapter is already `blocked` and writes no `.isf`; FSMGen's documented strict rule
  rejects the empty `(+size)` their text would lower to. Nothing SpecForge emits fails.

- **Full CI caught two of this slice's own bookkeeping slips, both fixed before commit**: a hand re-pin of the
  pin sentence kept the census evidence id, whose suffix must be the region digest (renamed to
  `…-89d661ed22a1`); and the new `CHANGES.md` entry's heading needed its `dated_rolling_ledger_evidence`
  registration. Neither involved FSMGen.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-24` | `.1` census | complete `git archive` of both revisions; known-good check on each; strict on 41 produced `.isf` | 41/41 identical (22 pass, 19 identical blocked-text failures) |
| `2026-09-24` | `.1` canaries | `cargo test --workspace --lib --exclude specforge-production-graph fsmgen_strict` | 7 passed |
| `2026-09-24` | `.1` full CI, first | `bash scripts/run_ci.sh` | blocked at `CLAIM-VERIFICATION` (the two slips above) |
| `2026-09-24` | `.1` full CI, final | `bash scripts/run_ci.sh` | PASS: 19/19 doctrines, fmt, warning-deny Clippy, tests 473 / 168 / 1,671 / 8 / 5 green, mdBook built, final project-data locality |

## Blockers

- None.

## Changelog

- `2026-09-24`: created before fetching or changing the gitlink, on the owner's permission.
- `2026-09-24`: fetched, audited the published contract delta, pinned `54ac1c22`, verified on complete
  installations of both revisions, reconciled pin copies; closed.
