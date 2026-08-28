# STATUS-LEDGER-ROLLOVER: roll the status ledger before its next product record is refused

## Metadata

- Tree ID: `STATUS-LEDGER-ROLLOVER`
- Status: `active`
- Roadmap lane: repository durability and portability
- Created: `2026-08-11`
- Last updated: `2026-08-11`
- Owner: repo-local workflow

## Goal

`LIVE_ACHIEVEMENT_STATUS.md` is a `rolling_ledger` whose `records` dimension reached 88.8% of its health
target when `CORPUS-COVERAGE.2.51` landed. The 90% milestone is 72 records, so the *next* genuine
product-status record is refused unless the same change performs the declared rollover. Perform it now, as
its own transaction, so the next product slice is not forced to bundle a control-plane migration into a
corpus refresh.

Size the cut for the records that will follow it, not for the committed root alone —
`CHANGES-LEDGER-ROLLOVER.1` had to correct exactly that mistake one day earlier.

## Non-Goals

- Do not widen a limit, milestone, or ceiling to postpone the rollover.
- Do not edit, reorder, or reflow a status record. Records are sealed evidence; the transaction moves whole
  records and nothing else.
- Do not cut into the retained migration suffix. Those 40 records are pinned live by the migrated-state
  contract and are already byte-exact in `source-through-2026-08-08.md`. Changing that pin is `.1`, not `.0`.
- Do not write a `LIVE_ACHIEVEMENT_STATUS.md` record for this transaction. A containment migration changes no
  product status, so under `COMMIT.md`'s routing contract it has nothing to say on that surface.

## Measured boundary (`2026-08-11`)

Committed at `5c2fe11f`, `LIVE_ACHIEVEMENT_STATUS.md` is 71 records / 113 lines / 97,617 bytes with a
4,824-byte widest line, SHA-256 `5034867ccee7db8a757d605b57f42a6dee9d584fadc8602575d50b4482ce698a`. Of those
71 records, 40 are the retained migration suffix and 31 were appended since `2026-08-08`. Pressure against
the `achievement_status` health targets and the ledger's `live_limits`:

| Dimension | Actual | Health target | Warning (80%) | Rollover (90%) |
| --- | ---: | ---: | ---: | ---: |
| Records | 71 | 80 | 64 | 72 |
| Bytes | 97,617 | 115,000 | 92,000 | 103,500 |
| Lines | 113 | 560 | 448 | 504 |
| Max line bytes | 4,824 | 6,200 | 4,960 | 5,580 |

Records and bytes are both past warning; records are one entry from the mandatory rollover. Lines are not a
pressure axis here, because one status record is one line.

## Acceptance Criteria

- A task-owned, repository-relative JSONL plan pins the committed opening blob and an exact whole-record cut.
- The dry run passes before the applied run, and only the exact green plan is applied.
- The resulting root is below the 80% warning on every dimension **and keeps enough headroom that ordinary
  product records do not re-cross the warning within a few slices**.
- The sealed segment, manifest, index, and chronology chain validate; older archive members stay
  byte-identical.
- `scripts/check_doctrines.sh` passes; no limit, milestone, or ceiling moves.

## Task Tree

- ID: `STATUS-LEDGER-ROLLOVER`
  Status: `active`
  Goal: the status ledger is back inside its warning band through its declared transaction, with no record
  edited and no bound moved, and the structural reason it keeps returning is measured
  Children: `STATUS-LEDGER-ROLLOVER.0`, `.1`, `.2`

- ID: `STATUS-LEDGER-ROLLOVER.0`
  Status: `done`
  Goal: seal the 21 oldest post-migration status records into `segment-0007` and leave a root with real
  headroom, not a minimal one-entry margin.
  Acceptance: `the plan pins commit 5c2fe11f and SHA-256 5034867c…698a; the dry run is green before the applied run; the root is below every 80% warning with at least ten ordinary records of headroom; segment/manifest/index/chain validate and older members are byte-identical; the gate passes`
  Verification: `plan docs/research/status-ledger-rollover-2026-08-11-plan.jsonl pins commit 5c2fe11f and opening SHA-256 5034867c…698a; the dry run reported "exact and warning-safe" before the applied run; the root is 50 records / 92 lines / 80,586 bytes = 62.5% of the record target, 70.1% of bytes, 16.4% of lines, 77.8% of max-line bytes — every dimension below its 80% warning; segment-0007-2026-08-11.md holds 21 records / 21 lines / 17,031 bytes at SHA-256 b1a7bcce…61b9; git diff proves all six older segments and the source capsule byte-identical; scripts/check_doctrines.sh 6/6`
  Commit: `STATUS-LEDGER-ROLLOVER.0 — seal segment-0007 and give the status ledger real headroom`

- ID: `STATUS-LEDGER-ROLLOVER.1`
  Status: `done`
  Goal: repair what `.0`'s mandatory alignment review found. The mdBook's rolling-ledger chapter stores a
  "current root is N records / L lines / B bytes" sentence for each of the four ledgers, and all four were
  false — a stored copy of a value that every commit invalidates is the derived-state anti-pattern the
  containment doctrine names, not a stale paragraph.
  Acceptance: `no book sentence asserts a current live-root size; the immutable per-segment identities stay; the status chapter states rollover seven; the reader gets a derive-on-read command and the archive index instead; mdbook build and the book-currency verifier pass; the aggregate-change authority is fresh and exact`
  Verification: `four stale claims removed — changes "87 records / 1,246 lines / 188,183 bytes" (actual 1,453 lines), development-notes "62 records / 1,294 lines / 175,215 bytes" (actual 1,523), live-achievement-status "59 records / 101 lines / 82,836 bytes" (actual 71 records at entry), rust-codebase-analysis "is now 1,064 lines / 89,706 bytes" (actual 1,189) — each replaced by the repeating structural shape plus perl scripts/check_rolling_ledger_protocol.pl --report and the per-ledger archive INDEX; the rust sentence keeps its number as an explicit migration-boundary capture; segments 0001-0006 independently re-counted at exactly 12 records each before asserting it; mdbook build exit 0; book-current-truth verifier green; authority STATUS-LEDGER-ROLLOVER.1-BOOK baseline 36/14,136/881,426 delta 0/+4/+383`
  Commit: `STATUS-LEDGER-ROLLOVER.0 — seal segment-0007 and give the status ledger real headroom`

- ID: `STATUS-LEDGER-ROLLOVER.2`
  Status: `pending`
  Goal: decide whether the pinned `2026-08-08` migration suffix should shrink. `.0` measured the reason this
  ledger returns to its threshold so quickly: `validate_retained_suffix` pins the migration window live
  forever, so a maximal cut can never release it, and it is the dominant consumer on all four root ledgers.
  Acceptance: `an exact per-ledger measurement of pinned-suffix share and post-cut capacity, a decision record for or against lowering planned_live.prefix_records, and — if lowered — a lossless transaction proving every released record still resolves in the source capsule`
  Verification: `pending`
  Commit: `pending`

- ID: `STATUS-LEDGER-ROLLOVER.3`
  Status: `pending`
  Goal: roll the status ledger before the next product record, and record how its plan is actually built
  Acceptance: measured `2026-08-28` at `5fe81128`, `LIVE_ACHIEVEMENT_STATUS.md` is **102,748 bytes = 89.3%**
  of its 115,000-byte health target, so the next ordinary record crosses the mandatory 90% signal and the
  protocol refuses the append unless the same change performs the declared rollover. Two things must be
  carried into that plan, both learned the hard way in `SOURCE-IR-REPRODUCIBILITY.2`:
  (1) **this ledger's plan cannot be hand-modelled the way `CHANGES.md`'s can.** Its grammar is
  `current_snapshot_bullets_v1` with `## Current snapshot` / `## Highest-priority remaining gap` markers and
  a validation-projection trailer, so the checker parses records and **re-renders a canonical live view**
  instead of slicing raw bytes. A hand-computed cut was wrong on every field — segment 17,997 bytes against
  an actual 14,741, root 83,550 against an actual 70,133. Build the plan by harvesting the `actual` values
  the dry run reports, then re-run it.
  (2) **record size is the real driver.** Status records had grown to roughly 2.6 KiB each, which returns
  this ledger to its signal about every two slices; keeping a record near 1.2 KiB is what let `.2` land
  without a rollover at all. A cut that does not also address record size buys one or two slices
  Prerequisite: none; it blocks the next status-bearing commit

- ID: `STATUS-LEDGER-ROLLOVER.4`
  Status: `pending`
  Goal: gate the per-record budget this surface's own limits already imply
  Decision (`2026-08-28`, owner-delegated): a status record states the **delivered status delta** — which
  metric moved, to what value, and whether the product claim changed — and **not the method**. Method,
  controls, attribution, and per-leg evidence belong to `CHANGES.md` and the owning task leaf, which are
  already their canonical homes; a status record that narrates them is duplicating a surface, not
  projecting status.
  This is not a style preference, and the measurement is what settles it. The ledger declares an
  **80-record** live window and a **115,000-byte** health target, which together imply a mean of
  **1,437 bytes per record** (1,638 at the 131,072-byte ceiling). Measured `2026-08-28`: the live window is
  102,748 bytes across 64 records — a **1,605-byte mean**, so the byte dimension binds at **71 records** and
  the declared 80-record window **can never be reached**. Records near 2.6 KiB bind it at roughly 44. Two
  declared limits are therefore mutually unsatisfiable at current record sizes, which is the real cause of
  the rollover treadmill; a rollover alone only resets the clock.
  Acceptance: the derived budget (`health_bytes / live_limits.records`) is checked rather than remembered —
  a record exceeding it, or a live-window mean exceeding it, is reported against the ledger the same way a
  size ceiling is, with a RED control proving an oversized record is observed. The check must derive the
  budget from the registry rather than carrying a literal, so it cannot go stale the way the counts
  `CLAIM-VERIFICATION-ADOPTION.6` corrected did
  Prerequisite: `STATUS-LEDGER-ROLLOVER.3`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `STATUS-LEDGER-ROLLOVER.0` | `done` | the mandatory transaction; it unblocks the next product-status record |
| 2 | `STATUS-LEDGER-ROLLOVER.1` | `done` | its alignment review was a blocker, so it landed in the same commit |
| 3 | `STATUS-LEDGER-ROLLOVER.2` | `pending` | tracking only; it blocks nothing, and no ledger is at a stop today |

## Decisions

- `2026-08-11`: run this as its own transaction rather than bundling it into the next corpus refresh. Same
  reasoning as `CHANGES-LEDGER-ROLLOVER`: pairing a control-plane migration with a product slice makes one
  commit that is hard to review and hard to roll back independently.
- `2026-08-11`: cut 21 records, not the minimal 8. Eight records is the smallest cut that satisfies the
  checker (63 records / 91,787 bytes), but the very next product record would return the ledger to 64 records
  — exactly at the warning — so the minimal cut buys one slice. Twenty-one leaves 14 records and 11,414 bytes
  of headroom; at the measured 946-byte post-migration record average that is about twelve ordinary records
  before the next warning and about twenty-two before the next mandatory rollover.
- `2026-08-11`: cut only from the post-migration records, and only upward from the boundary. The checker
  requires `keep_opening_prefix_records + records == post_migration_records`, so a cut always consumes the
  oldest post-migration records; the ten newest stay live and the reader keeps a current view.
- `2026-08-11`: this transaction writes no `LIVE_ACHIEVEMENT_STATUS.md` record. It writes a `CHANGES.md`
  record, because that ledger records changes, and a rollover is one.

## Open Questions

- `.2` owns the only one: should the pinned migration suffix stay pinned?

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | ownership | measured `LIVE_ACHIEVEMENT_STATUS.md` against the `achievement_status` health targets and the ledger `live_limits` | 71 records is 88.8% of the 80-record target and one entry from the 90% mandatory rollover; 97,617 bytes is 84.9% of the 115,000-byte target |
| `2026-08-11` | `.0` cut search | computed, independently of the checker, the minimal N and then the N that survives ordinary appends | N=8 is minimal (63 records / 91,787 bytes) but leaves one slice of headroom; N=21 leaves 50 records / 80,586 bytes ≈ twelve slices |
| `2026-08-11` | `.0` line-width safety | checked that the cut can reach the widest line | it cannot: the 4,824-byte widest line is record 62, inside the pinned migration suffix, so `line_bytes` is 77.8% of health before and after and no cut can lower it |
| `2026-08-11` | `.0` transaction | `--rollover-plan` dry run, then `--apply-rollover`; `git diff` over every prior archive member | dry run "exact and warning-safe" on the first attempt; applied root-last; all six older segments and the source capsule byte-identical; only the root, the new segment, the manifest, and the index changed |
| `2026-08-11` | `.0` gate | `bash scripts/check_doctrines.sh` | 6/6 PASS, `CHAIN-CURRENCY` DEFER as registered; the `achievement_status` warning lines are gone from the pressure report |
| `2026-08-11` | `.1` drift scan | compared every current-state number in the book's rolling-ledger chapter with the live roots | four of four "current root" sentences were false; the migration-boundary sentences were correct, because they name an exact capture boundary |
| `2026-08-11` | `.1` book gate | `mdbook build docs/book`, `scripts/check_book_current_truth.sh` via the doctrine driver, fresh aggregate authority | build exit 0; the five pinned live-docs literals are untouched; aggregate 36 files / 14,140 lines / 881,809 bytes matches baseline + delta exactly |
| `2026-08-11` | `.2` finding | measured the pinned migration suffix against each ledger's health target | it is immovable under the current registry and dominant everywhere: changes 170,695 B (66.9%), development-notes 155,662 B (62.3%), live-achievement-status 68,133 B (59.2%), rust-codebase-analysis 84,380 B (46.9%). After a maximal cut the usable live capacity left is 387 / 464 / 24-records / 79 lines respectively |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `STATUS-LEDGER-ROLLOVER.0` | `STATUS-LEDGER-ROLLOVER.0 — seal segment-0007 and give the status ledger real headroom` | 21 records sealed; older members byte-identical; root at 62.5% records / 70.1% bytes |
| `STATUS-LEDGER-ROLLOVER.1` | same commit | four false current-root claims removed from the book; a known current-facing contradiction is a `COMMIT.md` blocker, so it could not wait for its own slice |

## Changelog

- `2026-08-11`: Created because `CORPUS-COVERAGE.2.51`'s record took the status ledger to 88.8% of its record
  target, leaving the next product-status entry one record from refusal.
- `2026-08-11`: `.0` closed. The declared transaction ran unmodified and no code changed. Sizing the cut for
  future records rather than for the committed root is the `CHANGES-LEDGER-ROLLOVER.1` lesson applied before
  it could repeat.
- `2026-08-11`: `.1` added and closed. `.0`'s alignment review found the book asserting four current live-root
  sizes, all false. The repair is not "refresh the numbers" — a number that every commit invalidates must be
  derived on read, so the book now states the shape and names the command.
- `2026-08-11`: `.2` opened as tracking-only. Root-causing why this ledger returns to its threshold so fast
  found a structural answer that is not specific to this ledger, so it is measured and recorded rather than
  fixed inside a rollover slice.
