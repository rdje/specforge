# CHANGES-LEDGER-ROLLOVER: roll the change ledger before its next append is refused

## Metadata

- Tree ID: `CHANGES-LEDGER-ROLLOVER`
- Status: `done`
- Roadmap lane: repository durability and portability
- Created: `2026-08-11`
- Last updated: `2026-08-11`
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
  Children: `CHANGES-LEDGER-ROLLOVER.0`

- ID: `CHANGES-LEDGER-ROLLOVER.0`
  Status: `done`
  Goal: seal the oldest post-capsule change records into `segment-0005` and leave a warning-safe root.
  Acceptance: `the plan pins commit 9dc39c61 and SHA-256 6c2c5684…0ff3; the dry run is green before the applied run; the root is below every 80% warning; segment/manifest/index/chain validate and older members are byte-identical; the gate passes`
  Verification: `plan docs/research/changes-ledger-rollover-2026-08-11-plan.jsonl pins commit 9dc39c61 and opening SHA-256 6c2c5684…0ff3; the dry run reported "exact and warning-safe" before the applied run; the root is 95 records / 1,423 lines / 203,398 bytes = 79.1% of the line health target, 79.8% of bytes, 74.2% of records — every dimension below its 80% warning; segment-0005-2026-08-11.md holds 12 records / 179 lines / 15,551 bytes at SHA-256 355afe2b…9af6; git diff proves all four older segments and the source capsule byte-identical; scripts/check_doctrines.sh 6/6`
  Commit: `CHANGES-LEDGER-ROLLOVER.0 — seal segment-0005 and return the change ledger to its warning band`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CHANGES-LEDGER-ROLLOVER.0` | `done` | landed; the ledger is back inside its warning band |

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CHANGES-LEDGER-ROLLOVER.0` | `CHANGES-LEDGER-ROLLOVER.0 — seal segment-0005 and return the change ledger to its warning band` | 12 records sealed; older members byte-identical |

## Changelog

- `2026-08-11`: Created because `LIVE-DOC-STOP-RISK`'s four entries moved the ledger from 81.4% to 89.1% of its
  line health target, leaving the next ordinary append one entry from refusal.
- `2026-08-11`: `.0` closed and with it the tree. The declared transaction ran unmodified; the only code change
  was to make an exact-identity failure report its computed value, which is what made the plan derivable by
  hand at all.
