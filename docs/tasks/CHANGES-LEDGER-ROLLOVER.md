# CHANGES-LEDGER-ROLLOVER: roll the change ledger before its next append is refused

## Metadata

- Tree ID: `CHANGES-LEDGER-ROLLOVER`
- Status: `active` (`.0`/`.1`/`.2`/`.3`/`.5`/`.6` done; `.4` owns the standing limit every rollover keeps hitting)
- Roadmap lane: repository durability and portability
- Created: `2026-08-11`
- Last updated: `2026-08-30`
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
  Children: `CHANGES-LEDGER-ROLLOVER.0`, `.1`, `.2`, `.3`, `.4`, `.5`, `.6`

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

- ID: `CHANGES-LEDGER-ROLLOVER.3`
  Status: `done` (`2026-08-29`)
  Goal: seal twelve records so the ledger accepts `CLAIM-VERIFICATION-ADOPTION.6b`'s append
  Acceptance: the committed root at `40acadb2` is 1,619 lines — 89.9% of the 1,800-line health target — and the
  next ordinary record crosses the mandatory 90% signal, so by this tree's own rule the leaf whose entry trips
  the threshold performs the rollover and both land in one commit. A plan pins boundary commit `40acadb2` and
  its exact opening blob; the dry run is green before the applied run; no record is edited, reordered, or
  reflowed; no limit, milestone, or ceiling moves; the retained 75-record migration suffix is untouched; and the
  survivor is below the 80% warning on every dimension with the segment, manifest, index, and chronology chain
  validating
  Evidence: sealed 12 records / 504 lines / 45,619 bytes into
  `docs/archive/rolling-ledgers/changes/segment-0015-2026-08-29.md`, leaving a 1,114-line (61.9%) /
  176,223-byte (69.1%) root that is 1,152 lines (64.0%) after this slice's record. Dry run reported
  `exact and warning-safe` before the applied run; the applied run reports all 4 ledgers satisfying the
  lossless live-window/archive protocol
  Rig note worth keeping: a plan's `first_record_sha256` is the digest of the record's **raw** byte slice,
  which includes the blank line that separates it from the next record — while the segment's own digest is
  taken **after** the writer collapses the trailing blank to a single newline. Computing both the same way is
  the natural mistake, and the dry run reports the computed digest next to the expected one so it is a
  one-iteration fix rather than an opaque identity failure
  Plan: [`docs/research/claim-verification-adoption-6b-changes-rollover-plan.jsonl`](../research/claim-verification-adoption-6b-changes-rollover-plan.jsonl)
  Commit: `CLAIM-VERIFICATION-ADOPTION.6b / CHANGES-LEDGER-ROLLOVER.3 — sweep the two surfaces .6 and .6a left`

- ID: `CHANGES-LEDGER-ROLLOVER.4`
  Status: `pending` (tracking-only)
  Goal: decide the lifecycle of the pinned 75-record migration suffix, which is why every rollover only buys a
  dozen slices
  Acceptance: measured `2026-08-29`, the retained migration suffix alone is **1,054 lines and 170,695 bytes** —
  **58.6%** of the 1,800-line health target and **66.9%** of the 255,000-byte target — before a single current
  record exists. So a rollover can only ever recycle the remaining third, and at the measured ~2,500-byte
  record mean the ledger returns to its 90% signal after roughly a dozen ordinary slices. Three rollovers
  (`.0`/`.1`, `.2`, `.3`) have now each bought about that much, which is the signature of a standing limit
  rather than a run of coincidences. The decision this leaf owns is whether that suffix belongs in the live
  window at all: `.2` declined to seal it into a segment on the correct ground that its bytes are **already
  byte-exact in the source capsule**, and that same fact means retiring it from the live view duplicates
  nothing — it is a `planned_live` registry change plus a decision record, not a segment. Acceptance must show
  every reader route still resolves, the capsule still reconstructs every source byte, and the resulting live
  window is a window over *current* history rather than mostly frozen migration history
  Prerequisite: none; it blocks nothing today, and `.3` bought roughly eleven records of headroom

- ID: `CHANGES-LEDGER-ROLLOVER.5`
  Status: `done` (`2026-08-30`)
  Goal: roll the change ledger, which `STATUS-LEDGER-ROLLOVER.5`'s own record pushed past its line signal
  Acceptance: measured `2026-08-30`, `CHANGES.md` reached **1,639 lines = 91.1%** of its 1,800-line health
  target, crossing the mandatory 90% signal. It cannot be dodged by shortening the entry that crossed it: the
  entry would have to fall to five lines, and the ledger would still sit at the threshold. This rollover is
  therefore performed **inside the same transaction** as `STATUS-LEDGER-ROLLOVER.5` rather than after it,
  because the status rollover's own ledger record is what crossed the line and a commit cannot be made legal
  by deferring the gate that blocks it. That is this repository's existing pattern for a blocking pair, not a
  new one — `2b9e8899` carries `CLAIM-VERIFICATION-ADOPTION.6b / CHANGES-LEDGER-ROLLOVER.3` for the same
  reason. The task-tree pivot rule is respected in substance: no new work is started, and the tree is brought
  to a clean, committed state at the first point where that is possible.
  **Sizing, stated before the plan is written.** The pinned 75-record migration suffix is 1,053 lines and
  170,695 bytes before a single current record exists — 58.5% of the line target and 66.9% of the byte
  target — which is the standing limit `.4` owns and no cut can move. Of the reachable third, keeping three of the
  committed boundary leaves the root at 71.5% of bytes and 65.5% of lines with about eight records of
  headroom at the measured 2,466-byte mean. Three is chosen for a stated reason: they are exactly the records
  that close `CLAIM-VERIFICATION-ADOPTION.7`, the same three the status ledger kept in this commit, so the two
  ledgers open on the same story; this transaction's own two records ride over the cut as future prepends. `CHANGES-LEDGER-ROLLOVER.1`
  established that the record a rollover itself writes must be inside the kept prefix, and both are.
  **A plan must pin the COMMITTED boundary, not the working tree — the first attempt was refused for exactly
  that.** Because this rollover runs in the same transaction as the ledger records that triggered it,
  `CHANGES.md` was already dirty, and a plan whose `opening_sha256` was the working-tree digest failed with
  `committed opening blob differs from reconstructed boundary`. The checker's design is better than that
  workaround: it subtracts records added since `boundary_commit` as `future_prepends`, validates the
  remaining opening view against the committed blob, and carries the prepends over the cut untouched. So the
  plan pins 91 committed records and keeps three of them, and this transaction's own two records ride on top
  — a live root of five current records, reached without pretending the tree was clean.
  Prerequisite: none; it blocks `STATUS-LEDGER-ROLLOVER.5`'s commit
  Verification: `plan docs/research/changes-ledger-rollover-2026-08-30-plan.jsonl pins boundary commit
  3ff9e363 and committed opening SHA-256 eba47525…4522 across 91 records with 2 future prepends; the dry run
  reported "exact and warning-safe" before the applied run, and the applied run installed root-last. Root
  93 -> 80 records / 1,655 -> 1,179 lines / 226,925 -> 182,298 bytes = 71.5% of the byte target and 65.5% of
  the 1,800-line target, back under the 80% warning on every dimension with about eight records of headroom
  at the measured 2,466-byte mean. Segment segment-0016-2026-08-30.md holds 13 records / 474 lines /
  45,871 bytes at SHA-256 cdf66321…d208; git diff proves every older segment and the source capsule
  byte-identical, with only the root, the new segment, the manifest and the index changed. No record was
  edited, reordered or reflowed and no limit, milestone or ceiling moved`
  Commit: `STATUS-LEDGER-ROLLOVER.5 / CHANGES-LEDGER-ROLLOVER.5 — roll both root ledgers in one transaction`

- ID: `CHANGES-LEDGER-ROLLOVER.6`
  Status: `done` (`2026-08-31`)
  Goal: roll the change ledger, which `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c`'s own record pushed past its line
  signal
  Acceptance: measured `2026-08-31`, `CHANGES.md` reached **1,636 lines = 90.9%** of its 1,800-line health
  target — the mandatory 90% signal — and 225,378 bytes = 88.4%, already past the 80% byte warning as well.
  Shortening the entry that crossed it is refused on two grounds: the committed root was 1,606 lines, so the
  entry would have to fall to 13 lines to stay under 1,620, and trimming evidence to dodge a declared milestone
  is precisely the Non-Goal `LIVE-DOCUMENT-PRESSURE-HEADROOM` exists to enforce. So the rollover is performed
  **inside the same transaction** as `.4c`, which is this repository's established pattern for a blocking pair
  (`2b9e8899` carries `CLAIM-VERIFICATION-ADOPTION.6b / CHANGES-LEDGER-ROLLOVER.3`; `.5` carries
  `STATUS-LEDGER-ROLLOVER.5`). The pivot rule is respected in substance: no new work is started, and the tree is
  brought to a clean committed state at the first point where that is possible
  **Sizing, stated before the plan is written.** The pinned 75-record migration suffix is 1,053 lines and
  170,695 bytes before any current record exists — the standing limit `.4` owns — so only the remaining third is
  reachable. The cut is taken at a **semantic** boundary rather than at the minimum that clears the warning:
  the committed root's newest seven records are exactly the current `LIVE-DOCUMENT-PRESSURE-HEADROOM` story
  (`.15`, `.4b`, `.4a`, `.14a`, and three `.7` records), and the thirteen below them are the closed
  `SPEC-TO-INTENT-ALIGNMENT.9` and `CLAIM-VERIFICATION-ADOPTION.7` block. Keeping seven opens the ledger on the
  story this transaction continues — `.5`'s stated reason, applied to this tree. The minimum cut would have kept
  eleven and landed at 78.9% of bytes, roughly one record below re-warning; keeping seven lands at 74.0% with
  about six records before the warning and seventeen before the next stop, so the semantic cut is also the one
  that does not re-enter this transaction two slices later
  Prerequisite: none; it blocks `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c`'s commit
  Verification: `plan docs/research/changes-ledger-rollover-2026-08-31-plan.jsonl pins boundary commit d5b4016b
  and committed opening SHA-256 19c6b9c7...3155 across 95 records with 1 future prepend; the dry run reported
  exact and warning-safe before the applied run, and the applied run installed root-last. Root 95 -> 82 committed
  records / 1,606 -> 1,216 lines / 219,676 -> 185,835 bytes, and with .4c's own 30-line record riding over the
  cut the live root is 83 records / 1,246 lines (69.2% of the 1,800-line target) / 188,654 bytes (74.0% of the
  255,000-byte target) — under the 80% warning on every dimension. Segment segment-0017-2026-08-31.md holds
  13 records / 390 lines / 36,724 bytes at SHA-256 3b8783c1...87b2; git diff proves every older segment and the
  source capsule byte-identical, with only the root, the new segment, the manifest and the index changed. No
  record was edited, reordered or reflowed and no limit, milestone or ceiling moved`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c / CHANGES-LEDGER-ROLLOVER.6 — partition the composite genericity audit and roll the ledger it filled`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CHANGES-LEDGER-ROLLOVER.0` | `done` | landed, but one line short of its own acceptance |
| 2 | `CHANGES-LEDGER-ROLLOVER.1` | `done` | corrected the cut to account for the record the rollover itself writes |
| 3 | `CHANGES-LEDGER-ROLLOVER.2` | `done` | sealed 18 records so `.6`'s append was legal |
| 4 | `CHANGES-LEDGER-ROLLOVER.3` | `done` | sealed 12 records so `.6b`'s append was legal; root 61.9% / 69.1% |
| 5 | `CHANGES-LEDGER-ROLLOVER.5` | `done` | sealed 13 records so `STATUS-LEDGER-ROLLOVER.5`'s own record was legal; kept the five this transaction and `.7`'s closure wrote |
| 6 | `CHANGES-LEDGER-ROLLOVER.6` | `done` | sealed 13 records so `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c`'s own record was legal; cut at the semantic boundary, root 69.2% / 74.0% |
| 7 | `CHANGES-LEDGER-ROLLOVER.4` | `pending` | the frozen migration suffix occupies two thirds of the byte budget, so every rollover buys only a dozen slices |

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
