# Root rolling-ledger pressure census and transaction plan

Owning leaf: `ROOT-ROLLING-LEDGER-PRESSURE.0` (PROBE/DOC). This report pins the clean opening boundary and
selects the `.1` transaction. It removes no ledger record and changes no archive member, manifest, index,
executable checker, product artifact, threshold, or ceiling; only the required `.0` continuity records may prepend
after the pinned boundary.

## Opening boundary and exact roots

The boundary is commit `4d24b13cbd69b152e166cac85db502002553838e`. The only untracked pre-existing
path is the user-owned `.claude/settings.json`; it is outside this work.

| Ledger | Git blob | SHA-256 | Records | Lines | Bytes | Max line |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| changes | `b85ad2ff5f8ca1ecb489a0d3c6bac4438e88dadc` | `41226ed611ab42c359ee8ff0b07b1ffc99dffe6c94e7ce3d118863b89f91027c` | 114 | 1,645 | 222,725 | 1,629 |
| development-notes | `40acf325a04a7a19bb5cc70885efaafddaaa34d9` | `5d5846fdcfb0371db21b8f19e9adcfcb8808e93bc586e4097ff910893193ed3f` | 84 | 1,697 | 208,089 | 1,401 |
| live-achievement-status | `c4a0d4405451698bb9625b3643ac2f09b03b94a9` | `cf237f9a2fef23c6303549250f17c5d0a11c98325971a48912a269b584a91283` | 69 | 111 | 92,847 | 4,824 |
| rust-codebase-analysis | `e476ce3d2d9395745e2f0f83ff47322441906c22` | `cffefb9e0a5ef95389331b088b89aa2af59a4906bc1df65fec03ec505096ba06` | 62 | 1,187 | 100,701 | 369 |

The existing parser reconstructs every root and all four immutable capsules byte-for-byte. Each live root ends
in the exact planned migration suffix: 75 records for changes, 50 for development, 40 for status, and 50 for
Rust. The post-migration prefixes are therefore exactly 39, 34, 29, and 12 records.

## Pressure and the enforcement split

The generic surface authority measures milestones against reviewed health targets, as required by
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`. The focused rolling-ledger checker currently measures the same milestone
percentages against enforcement ceilings. That is a real split in executable doctrine, not two equivalent
views.

| Ledger | Record pressure in focused registry | Generic health pressure that controls action | Disposition |
| --- | --- | --- | --- |
| changes | 114/128 = 89.1% | lines 91.4%; bytes 87.3%; max line 77.6% | line rollover is already mandatory |
| development-notes | 84/96 = 87.5% | lines 89.3%; bytes 83.2%; max line 77.8% | warning; the two required task records would cross line rollover |
| live-achievement-status | 69/80 = 86.3% | lines 19.8%; bytes 80.7%; max line 77.8% | warning; three task records would also reach the 72-record rollover point |
| rust-codebase-analysis | 62/96 = 64.6% | lines 87.9%; bytes 55.9%; max line 67.1% | warning with only 28 lines before rollover |

This explains why `scripts/check_live_document_size.pl --report` reports the changes rollover while
`scripts/check_rolling_ledger_protocol.pl --report` passes. The focused checker uses 1,800 lines as neither a
target nor a threshold; it applies 90% to the 2,000-line ceiling and would wait until 1,800 lines. `.1` must bind
each ledger to its declared generic surface, require the existing focused line/byte/max-line limits to equal that
surface's enforcement ceilings, and evaluate milestones against its health targets. Record pressure remains a
rolling-ledger-only dimension. No value widens.

## Archive and consumer census

The verified current chains are:

- changes: live root → segment 0001 → source capsule;
- development-notes: live root → segment 0001 → source capsule;
- live-achievement-status: live root → segments 0003 → 0002 → 0001 → source capsule; and
- rust-codebase-analysis: live root → segment 0001 → source capsule.

The manifests have respectively 2, 2, 4, and 2 data records plus one control row. They occupy 1,699, 1,742,
3,753, and 1,852 bytes. Adding one segment to each produces only 3, 3, 5, and 3 data records against the unchanged
32-record / 32,768-byte controls. Segment collections become 2, 2, 4, and 2 files against their 28-file ceilings.
The four indexes currently total 37 lines / 1,378 bytes; one concise chronology row in each remains far below
the 256-line / 32,768-byte aggregate health target.

The registry declares `README.md` and the mdBook live-doc chapter as readers. `COMMIT.md` is the ordinary writer
for changes and development; status also has the real `project_validation.rs` managed-trailer writer, and Rust
also has `SESSION_BOOTSTRAP.md`. Stable roots and writer literals remain unchanged. The manifest/index checker
already proves one complete reciprocal live→segments→capsule chain, exact segment identities, and exact ordered
index membership.

There is no safe post-migration writer. `--emit-planned` intentionally refuses a migrated ledger and can emit
only an initial capsule-derived view. Prior rollovers were assembled within their owning product commits. `.1`
must therefore add a project-neutral transaction mode rather than repeat one-off byte surgery.

## Locked whole-record cuts

The machine-readable companion is
`docs/research/root-rolling-ledger-pressure-plan.jsonl`; its control row pins the boundary, schema, order, and
portable bounds, and its four registry-ordered rows pin every value in this section for `.1` consumption.

All ranges below are identified at the opening commit. Later `.0`/`.1` prepends are outside the ranges and stay
live. Segment rendering preserves selected record bytes and changes only the already accepted terminal successor
separator to one canonical newline.

| Ledger | New segment | Opening records removed | Segment metrics | Segment SHA-256 | Opening root after cut |
| --- | --- | ---: | --- | --- | --- |
| changes | `changes-0002` / `segment-0002-2026-08-09.md` | 29 | 428 lines / 37,189 bytes / max 128 | `bc87665975d80078697384e89b2127e67164ae0e523a8076b93f81eb5582fbd8` | 85 records / 1,216 lines / 185,535 bytes |
| development-notes | `development-notes-0002` / `segment-0002-2026-08-09.md` | 24 | 421 lines / 34,258 bytes / max 120 | `0576c44b7b95c40bc4669128ce07ebdc6a99b1ebbe9322cd1aca3acad8c04cbd` | 60 records / 1,275 lines / 173,830 bytes |
| live-achievement-status | `live-achievement-status-0004` / `segment-0004-2026-08-09.md` | 12 | 12 lines / 11,420 bytes / max 1,231 | `6d1c07596313d0c98c04afde63dd23d7df61fda291bd50d2cbc57a0568b924eb` | 57 records / 99 lines / 81,427 bytes before task prepends |
| rust-codebase-analysis | `rust-codebase-analysis-0002` / `segment-0002-2026-08-09.md` | 8 | 134 lines / 12,009 bytes / max 116 | `2444822e5f98f04516a20a9a54468e57f99721afcf869c37a12da0bc538bacb6` | 54 records / 1,052 lines / 88,691 bytes |

The exact segment endpoint identities are:

| Ledger | First record SHA-256 | Last rendered record SHA-256 |
| --- | --- | --- |
| changes | `2f113a975e3b704e89a783a1156f75a90bd25c7f8fcb8f3d43a01356fa47efa2` | `5b65cd58c4cf91958da66387e8ae9edf017f1c2fae95fc9d1e75ead80dea2487` |
| development-notes | `b099ad1d4d7186c131ae4ecc4c277aaa1b2af4dda8f5912bcbc3d7c135d88cc4` | `d8311c6435ffe8ef87a828b7e970a3d229100404810114964639517b76c6d3ec` |
| live-achievement-status | `5995a99ab15be3eeae63864b7faddc81346110ef22bf066a98831e69d7bdec69` | `22e95f7a667a454ce757f634fb18beabca694ba1ecab9e361117d793cbd781fa` |
| rust-codebase-analysis | `aaff9e64e0c3eb0848e0f3b0cc33acd181f10482a3f6dfcc471cf11ce8eb6ec0` | `a0205e2fcc01b48aaab2c23a89a8fa59bb5fa9b0326fc35370d85201282f3898` |

Changes and development each retain their ten newest opening post-migration records; status retains 17 opening
records plus the two `.0`/`.1` prepends and reaches 20 after the `.2` closure record; Rust retains its four newest
post-migration architecture records. The Rust cut is the smallest whole-record cut that leaves its 1,052-line
root below the 1,080-line warning with room for the largest measured recent 19-line architecture record.

The opening-root SHA-256 values after those exact cuts are `e54a310c…572d8`, `cef23549…56359`,
`99881cb3…04e3`, and `5607b0af…678f`. The first three final root hashes intentionally differ because required
task records prepend before or after materialization; the transaction must rediscover the pinned contiguous
range by count plus segment hash and then validate the actual resulting metrics. Concise task records are capped
so changes/development remain below all generic warnings, and each status bullet in this tree stays at or below
1,024 bytes; the projected final status root is at most 84,499 bytes, below its 92,000-byte warning.

## `.1` transaction contract

The implementation is one data-driven dry-run/apply path over the four registry records:

1. read a repository-relative transaction plan; reject an unknown ledger, unsafe path, reused segment id/path,
   non-current predecessor, ambiguous/non-contiguous range, count/hash/endpoint mismatch, or any plan output at
   or above a health warning;
2. render the four exact segments, roots, manifest rows, reciprocal predecessor edits, and complete indexes in a
   repository-derived same-volume staging directory;
3. validate staged identity, grammar, chain, route, limits, surface/registry agreement, and root reconstruction
   before changing authority;
4. install each segment first, then its manifest and index, and its live root last using same-directory exclusive
   temporaries; preserve exact rollback bytes and restore them on any failure;
5. run the focused protocol, generic live-size gate, all doctrines, docs/full CI, and residue/locality checks; and
6. retain only the committed roots, segments, manifests, indexes, plan evidence, and checker. No transaction
   workspace, backup, temporary file, or off-volume output may survive.

The new segment is each root's successor and the former newest segment's predecessor. The new segment's successor
is that former newest segment; only its manifest predecessor field changes. Capsules and existing segment files
remain byte-identical. Index rows regenerate in verified chain order.

## Acceptance disposition

- **Reproduce/measure:** four exact root identities, grammars, suffixes, pressure dimensions, chains, capacities,
  consumers, and candidate cuts are pinned above.
- **Root cause:** normal post-migration whole-record growth is expected; the actionable defect is that the focused
  milestone denominator drifted from the generic health authority to the quarantine ceiling.
- **Addressed design:** four bounded cuts restore warning-safe roots while a generic root-last transaction and
  surface binding make the next rollover reproducible and fail closed.
- **No regression:** `.0` changes planning/current-truth documentation only. `.1` owns executable enforcement,
  exact materialization, mutations, full gates, and zero residue; `.2` owns the independent clean-tree audit.
