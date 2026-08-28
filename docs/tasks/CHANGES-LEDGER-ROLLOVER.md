# CHANGES-LEDGER-ROLLOVER: roll the change ledger before its next append is refused

## Metadata

- Tree ID: `CHANGES-LEDGER-ROLLOVER`
- Status: `active` (`.0`/`.1` done; `.2` rolls the ledger again at the same signal)
- Roadmap lane: repository durability and portability
- Created: `2026-08-11`
- Last updated: `2026-08-28`
- Owner: repo-local workflow

## Goal

`CHANGES.md` is a `rolling_ledger`, and the four `LIVE-DOC-STOP-RISK` entries took it from 81.4% to 89.1% of
its 1,800-line health target. The 90% milestone is 1,620 lines: the next ordinary append is refused unless the
same change performs the declared rollover. Perform it now, as its own transaction, rather than bundling a
control-plane migration into the next product slice.

## Non-Goals

- Do not widen a limit, milestone, or ceiling to postpone the rollover.
- Do not edit, reorder, or reflow a change record. Records are sealed evidence; the transaction moves whole
  records and nothing else.
- Do not cut into the retained migration suffix. Those 75 records are already preserved byte-exact in
  `source-through-2026-08-08.md`, so sealing them again would archive the same bytes twice.

## Measured boundary (`2026-08-11`)

Committed at `9dc39c61`, `CHANGES.md` is 107 records / 1,603 lines / 218,950 bytes, SHA-256
`6c2c56847de483393bc2f2367329ee96dfba43be00dcc4016360848b4d010ff3`. Of those 107 records, 75 are the retained
migration suffix and 32 were appended since. Pressure against the generic surface health targets:

| Dimension | Actual | Health target | Warning (80%) | Rollover (90%) |
| --- | ---: | ---: | ---: | ---: |
| Lines | 1,603 | 1,800 | 1,440 | 1,620 |
| Bytes | 218,950 | 255,000 | 204,000 | 229,500 |
| Records | 107 | 128 | 102 | 115 |

Lines and records are both past warning; lines are 17 from the mandatory rollover.

## Acceptance Criteria

- A task-owned, repository-relative JSONL plan pins the committed opening blob and an exact whole-record cut.
- The dry run passes before the applied run, and only the exact green plan is applied.
- The resulting root is below the 80% warning on every dimension, and the sealed segment, manifest, index, and
  chronology chain validate; older archive members stay byte-identical.
- `scripts/check_doctrines.sh` passes; no limit, milestone, or ceiling moves.

## Task Tree

- ID: `CHANGES-LEDGER-ROLLOVER`
  Status: `done`
  Goal: the change ledger is back inside its warning band through its declared transaction, with no record
  edited and no bound moved
  Children: `CHANGES-LEDGER-ROLLOVER.0`, `.1`

- ID: `CHANGES-LEDGER-ROLLOVER.0`
  Status: `done`
  Goal: seal the oldest post-capsule change records into `segment-0005` and leave a warning-safe root.
  Acceptance: `the plan pins commit 9dc39c61 and SHA-256 6c2c5684…0ff3; the dry run is green before the applied run; the root is below every 80% warning; segment/manifest/index/chain validate and older members are byte-identical; the gate passes`
  Verification: `plan docs/research/changes-ledger-rollover-2026-08-11-plan.jsonl pins commit 9dc39c61 and opening SHA-256 6c2c5684…0ff3; the dry run reported "exact and warning-safe" before the applied run; the root is 95 records / 1,423 lines / 203,398 bytes = 79.1% of the line health target, 79.8% of bytes, 74.2% of records — every dimension below its 80% warning; segment-0005-2026-08-11.md holds 12 records / 179 lines / 15,551 bytes at SHA-256 355afe2b…9af6; git diff proves all four older segments and the source capsule byte-identical; scripts/check_doctrines.sh 6/6`
  Commit: `CHANGES-LEDGER-ROLLOVER.0 — seal segment-0005 and return the change ledger to its warning band`

- ID: `CHANGES-LEDGER-ROLLOVER.1`
  Status: `done`
  Goal: correct `.0`. Its cut was minimal against the root *as committed*, so the entry the transaction itself
  had to write put the ledger back over the warning — the leaf's own acceptance criterion was false in the tree
  it committed. Size the cut to include the record the rollover must write.
  Acceptance: `the root is below the 80% warning on every dimension after this leaf's own ledger entry lands; older members including segment-0005 stay byte-identical; no limit, milestone, or ceiling moves; the gate passes`
  Verification: `segment-0006-2026-08-11.md seals four more post-capsule records (55 lines / 4,713 bytes, SHA-256 72af06cd…a2f7), leaving the root at 92 records / 1,385 lines / 200,226 bytes; with this leaf's own 14-line entry the ledger is ~77% of its line health target, against 80.1% after .0; dry run exact on the first attempt; git diff proves segment-0005 and every earlier member byte-identical; scripts/check_doctrines.sh 6/6 with no change_history warning`
  Commit: `CHANGES-LEDGER-ROLLOVER.1 — size a rollover cut to include the record it must itself write`

- ID: `CHANGES-LEDGER-ROLLOVER.2`
  Status: `done` (`2026-08-28`)
  Goal: seal eighteen records so the ledger accepts `CLAIM-VERIFICATION-ADOPTION.6`'s append
  Acceptance: the committed root at `fdda3c53` is 1,608 lines — 89.3% of the 1,800-line health target — and
  `.6`'s record takes it to 1,636 (90.9%), past the mandatory 90% signal. The protocol's own rule is that the
  next ordinary append is refused *unless the same change performs the declared rollover*, so this leaf is the
  blocking prerequisite of that commit rather than an independent slice, and both land together with the
  rollover named in the body. A plan pins boundary commit `fdda3c53` and its exact opening blob; the dry run is
  green before the applied run; no record is edited, reordered, or reflowed; no limit, milestone, or ceiling
  moves; the retained 75-record migration suffix is untouched; and the resulting root is below the 80% warning
  on every dimension with the segment, manifest, index, and chronology chain validating
  Evidence: sealed 18 records / 259 lines / 21,230 bytes into
  `docs/archive/rolling-ledgers/changes/segment-0013-2026-08-28.md`, leaving a 1,348-line root (74.9%) that
  is 1,390 lines (**77.2%**) after this slice's two records. Cut sized deliberately, not minimally: two records
  were the minimal line-safe cut and would have returned the ledger to the signal within one ordinary slice.
  Dry run exact and warning-safe before the applied run; `check_rolling_ledger_protocol.pl` reports all 4
  ledgers satisfying the lossless live-window/archive protocol afterwards
  Plan: [`docs/research/claim-verification-adoption-6-changes-rollover-plan.jsonl`](../research/claim-verification-adoption-6-changes-rollover-plan.jsonl)
  Commit: `CLAIM-VERIFICATION-ADOPTION.6 — re-derive the drifted claim-annotated prose counts`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CHANGES-LEDGER-ROLLOVER.0` | `done` | landed, but one line short of its own acceptance |
| 2 | `CHANGES-LEDGER-ROLLOVER.1` | `done` | corrected the cut to account for the record the rollover itself writes |

## Decisions

- `2026-08-11`: run this as its own transaction rather than bundling it into `CORPUS-COVERAGE.2.51`. The
  repository's precedent is that the leaf whose entry trips the threshold performs the rollover, but no product
  leaf is active, and pairing a control-plane migration with a corpus ingest would make one commit that is hard
  to review and hard to roll back independently.
- `2026-08-11`: cut only from the post-capsule records. The 75 retained migration-suffix records sit below them
  and are already byte-exact in the source capsule, so sealing them into a segment would duplicate archived
  bytes rather than preserve anything new.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | ownership | measured `CHANGES.md` against its generic surface health targets and the ledger's `live_limits` | 1,603 lines is 89.1% of the 1,800-line health target and 17 lines from the 90% mandatory rollover; 107 records is 83.6% of 128 |
| `2026-08-11` | `.0` cut search | computed, independently of the checker, the minimal N whose removal puts records, lines, and bytes all below 80% | N=12 (keep 20 of the 32 post-capsule records) → 95 records / 1,423 lines / 203,398 bytes; N=11 still leaves 1,441 lines, one over the 1,440 warning |
| `2026-08-11` | `.0` diagnosability | the first dry run failed on an opaque `segment first-record identity drift` with no actual value, unlike every neighbouring drift message | fixed the two messages to print actual-vs-expected, then hand-derived the digest from exact byte offsets: leading records retain their blank separator (`…\n\n`), only the final record loses it to the canonical single-newline collapse — the earlier line-join attempt dropped that byte |
| `2026-08-11` | `.0` transaction | `--rollover-plan` dry run, then `--apply-rollover`; `git diff` over every prior archive member | dry run "exact and warning-safe"; applied root-last; all four older segments and the source capsule byte-identical; only the root, the new segment, the manifest, and the index changed |
| `2026-08-11` | `.0` gate | `bash scripts/check_doctrines.sh` | 6/6 PASS, `CHAIN-CURRENCY` DEFER as registered |
| `2026-08-11` | `.0` post-commit | read the live-document pressure report after committing `.0` | **`.0` did not meet its own acceptance**: `change_history lines_each at or above warning (80.1%)`. The cut was minimal against the committed root, and the transaction's own 18-line entry then pushed it back over the 1,440-line warning by one line |
| `2026-08-11` | `.1` sizing | recomputed the cut with the entry it would itself require included | N=4 leaves 1,385 lines → ~1,399 after a 14-line entry (77.7%); N=1 would have reproduced the same 80.1% failure |
| `2026-08-11` | `.1` transaction | dry run, then `--apply-rollover`; `git diff` over every earlier archive member | exact on the first dry run using the byte-offset record derivation `.0` established; root 92 records / 1,385 lines / 200,226 bytes; `segment-0005` and all earlier members byte-identical |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CHANGES-LEDGER-ROLLOVER.0` | `CHANGES-LEDGER-ROLLOVER.0 — seal segment-0005 and return the change ledger to its warning band` | 12 records sealed; older members byte-identical; left the root at 80.1% |
| `CHANGES-LEDGER-ROLLOVER.1` | `CHANGES-LEDGER-ROLLOVER.1 — size a rollover cut to include the record it must itself write` | 4 more records sealed into segment-0006; root ~77% |

## Changelog

- `2026-08-28`: a third post-migration rollover, executed inside `LIVE-DOCUMENT-PRESSURE-HEADROOM.5`
  because the line dimension crossed its mandatory 90% signal at 1,639 of 1,800 and the protocol refuses
  the append otherwise. Plan `docs/research/changes-ledger-rollover-2026-08-28b-plan.jsonl` pins boundary
  commit `943381c8` and opening blob `84d2daff…2284`; 14 records sealed as `segment-0014-2026-08-28.md`
  (371 lines / 33,716 bytes / SHA-256 `6da102b1…c831`); root 95 -> 81 records / 1,639 -> 1,267 lines
  (70.4%) / 223,990 -> 190,273 bytes (74.6%); all 13 older segments and the capsule byte-identical. Two
  things are worth carrying forward. The plan had to be re-harvested once: `opening_records` must be the
  count in the **committed** blob (94), not the working tree (95), or the boundary check fails with
  "committed opening blob differs from reconstructed boundary" — the uncommitted record is a
  `future_prepend`, not part of the opening. And this rollover is `STATUS-LEDGER-ROLLOVER.4`'s prediction
  arriving on schedule: this ledger's declared 128-record window is unreachable at 108, so a size
  dimension always binds first, and it was lines.

- `2026-08-11`: Created because `LIVE-DOC-STOP-RISK`'s four entries moved the ledger from 81.4% to 89.1% of its
  line health target, leaving the next ordinary append one entry from refusal.
- `2026-08-11`: `.0` closed and with it the tree. The declared transaction ran unmodified; the only code change
  was to make an exact-identity failure report its computed value, which is what made the plan derivable by
  hand at all.
- `2026-08-11`: reopened for `.1`, because `.0`'s closure was premature: the committed tree did not satisfy
  `.0`'s own acceptance criterion. A rollover writes a ledger record like any other change, so a cut that is
  minimal against the committed root stops being minimal the moment the transaction finishes. `.1` re-cut with
  that record included and the tree is closed for real.
