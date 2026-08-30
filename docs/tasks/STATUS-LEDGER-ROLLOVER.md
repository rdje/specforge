# STATUS-LEDGER-ROLLOVER: roll the status ledger before its next product record is refused

## Metadata

- Tree ID: `STATUS-LEDGER-ROLLOVER`
- Status: `active` (`.0`/`.1`/`.3`/`.4`/`.4a`/`.5` done; `.2` tracking-only)
- Roadmap lane: repository durability and portability
- Created: `2026-08-11`
- Last updated: `2026-08-30`
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
  Children: `STATUS-LEDGER-ROLLOVER.0`, `.1`, `.2`, `.3`, `.4`, `.4a`, `.5`

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
  Status: `done` (`2026-08-28`)
  Goal: roll the status ledger before the next product record, and record how its plan is actually built
  Acceptance: measured `2026-08-28` at `3b3ea863` (`.4a`; the root is unchanged since `245b3b60`),
  `LIVE_ACHIEVEMENT_STATUS.md` is **102,748 bytes = 89.35%** of its 115,000-byte health target across
  **70 records = 87.5%** of its 80-record window. Only a record of 751 bytes or less keeps the root under the
  103,500-byte mandatory signal, and no record that size has been written in months, so the next ordinary
  record crosses it and the protocol refuses the append unless the same change performs the declared
  rollover. (`.4a` corrected this row: the revision was `5fe81128`, where the root was 101,547 bytes across
  69 records, and the byte figure belongs to `245b3b60`.) Two things must be
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
  Evidence (`2026-08-28`): plan `docs/research/status-ledger-rollover-2026-08-28-plan.jsonl` pins boundary
  commit `024202dd` and opening SHA-256 `3055b508…cfc6`; the dry run reported **exact and warning-safe**
  before the applied run, and the applied run installed root-last. Root **70 -> 44 records / 112 -> 86 lines
  / 102,748 -> 76,758 bytes** = 66.75% of the byte target, 55.00% of the record window, 15.36% of lines, and
  an unchanged 77.8% widest line (the 4,824-byte line is record 61, inside the pinned suffix, so no cut can
  reach it). Segment `segment-0011-2026-08-28.md` holds 26 records / 26 lines / 25,990 bytes at SHA-256
  `a13be8e8…7a15`; `git diff` proves every older segment and the source capsule byte-identical, with only the
  root, the new segment, the manifest, and the index changed. No limit, milestone, or ceiling moved, and no
  record was edited, reordered, or reflowed.
  How the plan was actually built (this is the part `.3` exists to record): the metrics were **harvested,
  not computed**. A first pass wrote `1` for every metric and 64 zero-hex for every digest; the checker
  reports each mismatch as `actual X, expected Y`, so one dry run yields the segment's records/lines/bytes/
  line_bytes and SHA-256, its first- and last-record digests, and the resulting root's four metrics and
  digest. The second dry run with those values is exact. Only two figures can be modelled by hand and both
  were, as a cross-check that the harvest is not self-fulfilling: `keep + records == post_migration_records`
  (4 + 26 == 30) and the resulting root bytes, predicted at 76,758 from the record-size table before the
  first dry run and reported as 76,758 by the checker.
  Sizing (why four, not the minimum or the maximum): the pinned 40-record migration suffix is live forever
  under `validate_retained_suffix` and alone spends 66,720 of the 92,000-byte warning budget, so the whole
  reachable headroom is about 25,000 bytes — roughly 18 records at `.4a`'s 1,351.6-byte budget — and only a
  cut that removes **every** post-migration record reaches it, leaving the reader a snapshot whose newest
  entry is `2026-08-08`. Keeping the four newest is the natural boundary because those four are exactly the
  `2026-08-28` records, so the snapshot still opens on the current day, and it leaves 15,242 bytes ≈ eleven
  budget-sized records. The live-window record mean after the cut is **1,588.3 bytes**, still above budget:
  the suffix averages 1,496.2 and the four kept records average 2,509.5. A rollover cannot fix that, which is
  `.4`'s point.

- ID: `STATUS-LEDGER-ROLLOVER.5`
  Status: `done` (`2026-08-30`)
  Goal: roll the status ledger again, sized so the next few slices do not re-cross the signal
  Acceptance: measured `2026-08-30` at `3ff9e363`, `LIVE_ACHIEVEMENT_STATUS.md` is **103,382 bytes = 89.9%**
  of its 115,000-byte health target across 56 records. It crossed the 90% mandatory signal earlier in that
  same commit and was brought back under only by tightening that slice's own entry, which is a reprieve and
  not a fix: the next ordinary append crosses it again. Perform the declared transaction — a task-owned
  repository-relative JSONL plan pinning the committed opening blob and an exact whole-record cut, a green
  dry run before the applied run, root installed last — with no record edited and no bound moved.
  **Sizing, stated before the plan is written so the choice is reviewable rather than retrofitted.** The
  pinned 40-record migration suffix is live forever under `validate_retained_suffix` and alone spends 66,720
  of the 92,000-byte warning budget, so the whole reachable headroom is about 25,000 bytes. Of the 16
  post-migration records, seven are `2026-08-30`, five are `2026-08-29` and four are `2026-08-28`. Measured
  from the committed root: keeping all seven same-day records leaves 79,580 bytes and only about **seven**
  records of headroom, which is the "buys one or two slices" outcome `.3` warns against; keeping **three**
  leaves 71,626 bytes = 62.3% of the target with about **eleven** records, matching what `.3` achieved.
  Three is also a principled boundary rather than a count: they are exactly the records that close
  `CLAIM-VERIFICATION-ADOPTION.7`, and they are the newest three, so the snapshot still opens on the current
  day. Keeping zero would leave the most headroom and a snapshot whose newest entry is `2026-08-08`, which is
  the reader cost `.3` already declined to pay
  **Build the plan by harvesting, not by computing** (`.3`'s recorded lesson): this ledger's grammar is
  `current_snapshot_bullets_v1`, so the checker re-renders a canonical live view instead of slicing raw
  bytes, and every hand-computed field was wrong last time. Write placeholder metrics, read each
  `actual X, expected Y` from the dry run, rewrite, and re-run until exact
  Prerequisite: none; it blocks the next status-bearing commit
  Verification: `plan docs/research/status-ledger-rollover-2026-08-30-plan.jsonl pins boundary commit
  3ff9e363 and opening SHA-256 8ed68c76…451d; the dry run reported "exact and warning-safe" before the
  applied run, and the applied run installed root-last. Root 56 -> 43 records / 98 -> 85 lines / 103,382 ->
  71,626 bytes = 62.28% of the byte target, 53.75% of the 80-record window, 15.18% of lines, and an unchanged
  4,824-byte widest line (that line is inside the pinned suffix, so no cut can reach it). Segment
  segment-0012-2026-08-30.md holds 13 records / 13 lines / 31,756 bytes at SHA-256 d4c62d84…6e58; git diff
  proves every older segment and the source capsule byte-identical, with only the root, the new segment, the
  manifest and the index changed. No limit, milestone, or ceiling moved and no record was edited, reordered,
  or reflowed. The harvest was cross-checked against one independently predicted figure before the first dry
  run, exactly as .3 prescribes: the resulting root was predicted at 71,626 bytes from the per-record table
  and reported as 71,626 by the checker, so the harvest is not self-fulfilling. The mdBook rolling-ledger
  chapter is deliberately unchanged — .1 removed its stored root sizes and .3 already recorded the harvest
  procedure, so there is no truth on that surface for this transaction to move`
  Commit: `STATUS-LEDGER-ROLLOVER.5 / CHANGES-LEDGER-ROLLOVER.5 — roll both root ledgers in one transaction`

- ID: `STATUS-LEDGER-ROLLOVER.4`
  Status: `done` (`2026-08-28`)
  Goal: gate the per-record budget this surface's own limits already imply
  Decision (`2026-08-28`, owner-delegated): a status record states the **delivered status delta** — which
  metric moved, to what value, and whether the product claim changed — and **not the method**. Method,
  controls, attribution, and per-leg evidence belong to `CHANGES.md` and the owning task leaf, which are
  already their canonical homes; a status record that narrates them is duplicating a surface, not
  projecting status.
  This is not a style preference, and the measurement is what settles it. The ledger declares an
  **80-record** live window and a **115,000-byte** health target. The live view is not only records: a
  46-byte prologue and a 6,825-byte validation-projection trailer are charged to the same budget, so the
  honest per-record budget is `(115,000 - 6,871) / 80` = **1,351.6 bytes**, not the naive `115,000 / 80`
  = 1,437.5. Re-derived `2026-08-28` by `.4a` at `3b3ea863`: the live window is 102,748 bytes across
  **70** records — 95,877 record bytes, a **1,369.7-byte record mean** — so capacity is **78 records** and
  the declared 80-record window **cannot be reached**. Records near 2.6 KiB bind it at **40**. Two declared
  limits are therefore mutually unsatisfiable at current record sizes, which is the real cause of the
  rollover treadmill; a rollover alone only resets the clock.
  The corrected margin is narrower than the one this leaf opened with (78 against 80, not 71), and the
  driver is sharper: the **newest ten** records average **2,085.6 bytes**, 1.54x the budget, which alone
  caps the window at **51**. The oldest 40 — the pinned migration suffix — average 1,496.2. Record size is
  growing, and it is the recent records that spend the budget.
  Acceptance: the derived budget (`health_bytes / live_limits.records`) is checked rather than remembered —
  a record exceeding it, or a live-window mean exceeding it, is reported against the ledger the same way a
  size ceiling is, with a RED control proving an oversized record is observed. The check must derive the
  budget from the registry rather than carrying a literal, so it cannot go stale the way the counts
  `CLAIM-VERIFICATION-ADOPTION.6` corrected did. `.4a` also measured *why* the wrong count survived
  publication: **no tracked producer reports this root's live record count.**
  `perl scripts/check_live_document_size.pl --report` emits bytes, lines, and line bytes only, and
  `perl scripts/check_rolling_ledger_protocol.pl --report` emits the registry's frozen `planned_live`
  migration boundary, not the live window. The record dimension is bounded, is at 87.5% of its bound, and
  is unreported — so this gate must publish the count as well as the budget
  Prerequisite: `STATUS-LEDGER-ROLLOVER.3`, `STATUS-LEDGER-ROLLOVER.4a`
  Reported as pressure, not as corruption (decided `2026-08-28`, on measurement): the records already on
  every surface are sealed evidence that no compliant change may shrink, and all four ledgers carry records
  above their derived budget today — 62/91, 38/83, 20/44, 17/56. A `problem()` would therefore be a stop
  with no exit, which is the anti-pattern `LIVE-DOC-STOP-RISK` exists to prevent. The check reports the way
  the generic live-size gate reports an approaching ceiling: a named, quantified, non-fatal line, with the
  derivation shown inline so a reader can recompute it.
  Evidence (`2026-08-28`): `scripts/check_rolling_ledger_protocol.pl` gains `measure_record_budget` and
  `validate_record_budget`. The budget is
  `int((health_targets.bytes_each - live-view overhead) / live_limits.records)`, read from
  `doctrine/live_document_size/surfaces.jsonl` and `rolling_ledgers.jsonl` with no literal anywhere, and the
  overhead is measured as `live bytes - sum(record bytes)` rather than assumed zero. `--report` now emits a
  `live` object per migrated ledger — records, declared_records, record_bytes, overhead_bytes, health_bytes,
  record_budget_bytes, record_mean_bytes, max_record_bytes, max_record_ordinal, oversized_records,
  reachable_records — which is the first producer to publish a rolling ledger's live record count at all.
  Measured across all four ledgers, and the finding generalizes beyond this one: **two of four declared
  windows are unreachable** — `changes` holds 109 records against a declared 128 at its 2,328-byte mean, and
  `live-achievement-status` holds 68 against 80 at 1,588. `development-notes` (101 vs 96) and
  `rust-codebase-analysis` (109 vs 96) are reachable. Self-test 35 -> **41**, with **seven observed RED
  perturbations**, each isolating one leg: overhead not charged, budget carried as a literal, oversized
  records never reported, unreachable window never reported, the size comparison loosened, the declared
  window ignored, and the health target ignored. A sixth control proves a live view whose overhead exceeds
  its own health target is a hard failure rather than a negative budget.

- ID: `STATUS-LEDGER-ROLLOVER.4a`
  Status: `done`
  Goal: re-derive the ledger measurement `.3` and `.4` were sized on, after it failed an independent count
  Acceptance: `every figure .3 and .4 publish about the live root is re-derived from the tracked root by an exact repository-relative command; a dimensionally different oracle confirms or refutes the record count; each corrected figure is restated with the conclusion that survives it; and the reason the error was publishable is named rather than treated as a slip`
  Verification: `re-derivation at 3b3ea863 - LC_ALL=C awk '/^## Current snapshot$/{f=1;next} /^## Highest-priority remaining gap$/{f=0} f' LIVE_ACHIEVEMENT_STATUS.md | LC_ALL=C awk '{n++; b+=length($0)+1} END{printf "%d records, %d record bytes, mean %.1f\n", n, b, b/n}' reports 70 records / 95,877 record bytes / 1,369.7 mean, against wc -c = 102,748; falsification by the tracked checker's own current_snapshot_bullets_v1 parser, probed through rollover-plan boundary arithmetic - opening_records 71 goes RED with "has fewer records than its opening boundary" and 70 does not, so parse_snapshot and the marker extraction agree at exactly 70; root cause: 64 is this surface's record WARNING THRESHOLD (80 x 80%) and was published as an observed count, after which 102,748/64 produced the 1,605-byte mean and the 71-record bound; durability leg MISSING - no tracked producer reports the live record count, which is what STATUS-LEDGER-ROLLOVER.4 must add`
  Commit: `STATUS-LEDGER-ROLLOVER.4a — re-derive the status-ledger measurement .3 and .4 were sized on`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `STATUS-LEDGER-ROLLOVER.0` | `done` | the mandatory transaction; it unblocks the next product-status record |
| 2 | `STATUS-LEDGER-ROLLOVER.1` | `done` | its alignment review was a blocker, so it landed in the same commit |
| 3 | `STATUS-LEDGER-ROLLOVER.4a` | `done` | `.3` and `.4` were both sized on a wrong record count; nothing downstream could be built until it was re-derived |
| 4 | `STATUS-LEDGER-ROLLOVER.3` | `done` | the mandatory transaction; the root is back to 66.75% bytes / 55.00% records |
| 5 | `STATUS-LEDGER-ROLLOVER.4` | `done` | the budget and the record count are derived, published, and controlled |
| 6 | `STATUS-LEDGER-ROLLOVER.2` | `pending` | tracking only; it blocks nothing, and no ledger is at a stop today |

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
| `2026-08-28` | `.4a` re-derivation | counted the live window from the tracked root with its registered grammar markers, and measured the fixed prologue/trailer overhead the byte budget also pays | 70 records / 95,877 record bytes / 6,871 bytes of non-record overhead against 102,748 total; record mean 1,369.7, whole-window mean 1,467.8; the published `64 records` and `1,605-byte mean` are both wrong |
| `2026-08-28` | `.4a` falsification | probed the tracked checker's own `current_snapshot_bullets_v1` parser through rollover-plan boundary arithmetic, an independent producer from the marker extraction | `opening_records: 71` RED (`has fewer records than its opening boundary`); `opening_records: 70` not RED — both producers agree at exactly 70, so the competing hypothesis that continuation bullets inflate the shell count is refuted |
| `2026-08-28` | `.4a` root cause | asked where `64` could have come from, and which producer should have caught it | `64` is this surface's record warning threshold (80 x 80%), published as an observed count; `102,748 / 64` then yields the 1,605-byte mean and the 71-record bound. No tracked producer reports the live record count — `check_live_document_size.pl --report` emits bytes/lines/line_bytes and `check_rolling_ledger_protocol.pl --report` emits the frozen `planned_live` boundary — so nothing could contradict it |
| `2026-08-28` | `.3` transaction | `--rollover-plan` dry run, then `--apply-rollover`; `git diff` over every prior archive member; live-size gate | dry run "exact and warning-safe" on the second attempt (the first carried deliberate placeholders to harvest actuals); applied root-last; all ten older segments and the source capsule byte-identical; the `achievement_status` warning lines are gone from the pressure report |
| `2026-08-28` | `.3` cross-check | predicted the resulting root bytes by hand from the per-record size table before running the checker | 76,758 predicted, 76,758 reported — the two figures a planner can model (`keep + records == post_migration_records`, and root bytes) both agree, so the harvested metrics are not merely self-consistent |
| `2026-08-28` | `.3` record-count corroboration | read the dry run's own `resulting_live.records` against the plan's cut | 44 = 70 - 26, an independent third confirmation of `.4a`'s corrected 70-record count, this time from the checker's transaction path rather than its boundary arithmetic |
| `2026-08-28` | `.4` derivation | read the budget from the registry pair rather than a literal, and charged the live view's prologue/trailer to it | `int((bytes_each - overhead) / live_limits.records)`; four ledgers derive 1,992 / 2,603 / 1,351 / 1,872 bytes with 0 / 20 / 6,871 / 269 bytes of overhead |
| `2026-08-28` | `.4` population | ran the new report over all four ledgers | two declared windows are unreachable — `changes` 109 of 128, `live-achievement-status` 68 of 80 — and every ledger carries records above budget (62/91, 38/83, 20/44, 17/56) |
| `2026-08-28` | `.4` controls | `--self-test`, then seven targeted perturbations of the production code, each restored after observation | 35 -> 41 checks; every perturbation observed RED and named the specific leg it broke; no perturbation left the suite green |
| `2026-08-11` | `.2` finding | measured the pinned migration suffix against each ledger's health target | it is immovable under the current registry and dominant everywhere: changes 170,695 B (66.9%), development-notes 155,662 B (62.3%), live-achievement-status 68,133 B (59.2%), rust-codebase-analysis 84,380 B (46.9%). After a maximal cut the usable live capacity left is 387 / 464 / 24-records / 79 lines respectively |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `STATUS-LEDGER-ROLLOVER.0` | `STATUS-LEDGER-ROLLOVER.0 — seal segment-0007 and give the status ledger real headroom` | 21 records sealed; older members byte-identical; root at 62.5% records / 70.1% bytes |
| `STATUS-LEDGER-ROLLOVER.1` | same commit | four false current-root claims removed from the book; a known current-facing contradiction is a `COMMIT.md` blocker, so it could not wait for its own slice |
| `STATUS-LEDGER-ROLLOVER.4a` | `STATUS-LEDGER-ROLLOVER.4a — re-derive the status-ledger measurement .3 and .4 were sized on` | 64 records -> 70; 1,605-byte mean -> 1,369.7; binds at 71 -> 78; the unsatisfiable-limits conclusion survives with a narrower margin |
| `STATUS-LEDGER-ROLLOVER.3` | `STATUS-LEDGER-ROLLOVER.3 — roll the status ledger, and record how its plan is actually built` | 26 records sealed as segment-0011; root 70 -> 44 records / 102,748 -> 76,758 bytes; older members byte-identical |
| `STATUS-LEDGER-ROLLOVER.4` | `STATUS-LEDGER-ROLLOVER.4 — derive, publish, and control the per-record budget` | budget derived from the registry; live record count published for the first time; self-test 35 -> 41 with seven observed RED perturbations |

## Changelog

- `2026-08-11`: Created because `CORPUS-COVERAGE.2.51`'s record took the status ledger to 88.8% of its record
  target, leaving the next product-status entry one record from refusal.
- `2026-08-11`: `.0` closed. The declared transaction ran unmodified and no code changed. Sizing the cut for
  future records rather than for the committed root is the `CHANGES-LEDGER-ROLLOVER.1` lesson applied before
  it could repeat.
- `2026-08-11`: `.1` added and closed. `.0`'s alignment review found the book asserting four current live-root
  sizes, all false. The repair is not "refresh the numbers" — a number that every commit invalidates must be
  derived on read, so the book now states the shape and names the command.
- `2026-08-28`: `.4a` added and closed. `.4`'s own decision paragraph published `64 records` and a
  `1,605-byte mean` for a root that holds 70 records at a 1,369.7-byte mean; `64` is this surface's record
  warning threshold, not a count, and every figure derived by dividing by it was wrong. The conclusion
  survives — the 80-record window and the 115,000-byte target remain mutually unsatisfiable — but by 2
  records, not 9, and the honest budget is 1,351.6 bytes once the 6,871-byte prologue/trailer overhead is
  charged. The finding that matters for `.4` is not the arithmetic: the record dimension is bounded, sits at
  87.5% of its bound, and **no producer reports it**, so nothing in the repository could have contradicted
  the published number.
- `2026-08-28`: `.4` closed, and its finding is wider than the leaf that opened it. The per-record budget
  the declared limits imply is now derived from the registry and checked, the live record count is published
  for the first time by any producer, and the same measurement over all four root ledgers shows **two**
  mutually unsatisfiable windows, not one: `changes` reaches 109 of a declared 128. Reported as pressure
  rather than as a violation, because sealed records cannot be shrunk and a stop with no compliant exit is
  the failure mode `LIVE-DOC-STOP-RISK` owns. The tree's remaining leaf is `.2`, which is tracking-only.
- `2026-08-28`: `.3` closed. The transaction ran as declared and no code changed. Two things are now durable
  that were not: the plan is built by harvesting the dry run's `actual` values rather than hand-modelling a
  grammar the checker re-renders, and the cut is sized against `.4a`'s overhead-net per-record budget instead
  of against the committed root. The measurement that matters for `.4` is the one the cut could not change:
  the post-cut live-window record mean is 1,588.3 bytes against a 1,351.6-byte budget, because the pinned
  suffix averages 1,496.2 on its own. A rollover resets the clock; it cannot make the declared limits
  satisfiable.
- `2026-08-11`: `.2` opened as tracking-only. Root-causing why this ledger returns to its threshold so fast
  found a structural answer that is not specific to this ledger, so it is measured and recorded rather than
  fixed inside a rollover slice.
