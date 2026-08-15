# Reviewed-population clean replay — structural-carrier diagnostic

- Date: `2026-08-15`
- Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.a`
- Production revision: `0d2181167d81d9418e140b2d09c1c5aecbfd3197`
- Reviewed authority: 12 documents / 14 cells / 48 isolated stages
- Diagnostic root: `.project-data/tmp/spec-to-intent-f-iv-population-r1`
- Runtime source map: `.project-data/tmp/spec-to-intent-f-iv-source-map.json`

## Why this result is diagnostic, not current authority

The first `.f.iv` replay completed all 12 hash-equal sources and all 48 stages from a clean production revision.
At discovery time it was retained below repository-local scratch and did not replace the tracked `b977a51f`
result, controller input, report, or portable replay manifest. That diagnostic was later consumed and deleted;
`.f.iv.b` now publishes a separate clean post-repair replay. The values below remain historical diagnosis.

The replay manifest, not the bounded fixture projection, is stage-identity authority. The fixture's
`original_sha256` fields retain the frozen review-era hashes so cells stay comparable across revisions. Actual
fresh stage hashes live in `replay_manifest.json` and differ from the tracked `.6d.ii.a` manifest. The initial
appearance of “same stages, different score” was therefore a projection-identity interpretation error, not an
evaluator contradiction.

## Exact diagnostic result

The current dataset is 143,371 bytes with SHA-256
`226d5a2dfdb3e72b898f94500ed2c1046408ec156e808f0b40859da9742b4484`. The result is 100,338 bytes with SHA-256
`d02e988c46a060034156a056c4c11c6888587172e40b667a4f7dfa299f6339a8`. Compared with the pinned 24/2/16
TP/FP/FN authority, exactly three reviewed cells change:

| Cell | Pinned result | Clean diagnostic | Disposition |
| --- | --- | --- | --- |
| AMD IOMMU packed PTE | one false register; capture present | no canonical register; capture absent | fabrication removal is correct; required residual remains missing |
| GIC-400 register summary | one generic false register and 15 misses | 15 source-named rows, all missing access, and 15 misses | name/offset carrier improves; access carrier is incomplete |
| Arm Debug register summary | 12 exact register/access facts | capture absent and 12 misses | unacceptable truth loss |

The diagnostic aggregate is 12/15/28 IntentIR TP/FP/FN, 30/30 canonical provenance, 36/64 conservation, 4/24
actionable residuals, 15 fabricated facts, and 28 unexplained drops. Physical-link remains supported; the other
five reviewed categories remain incomplete. These values describe the blocked diagnostic only.

## Root cause

All three target tables retain their reviewed rows. The behavior changes at structural classification and
register-carrier construction:

- The AMD 4×21 packed bit layout moves from the stale retained `register_map` label to `unknown`. It has neither
  a register-name role nor access semantics, so removing the false register is the intended generic outcome.
- The GIC-400 16×5 table moves from stale retained `encoding` to fresh `register_map`. Its `Offset | Name | Type |
  Reset | Description` shape yields 15 correctly named and offset registers. EvidenceIR does not carry `Type` as
  the access column even though the same register-field grammar and every body value establish access semantics.
- The Arm Debug 14×6 table remains `unknown`. Its exact structural roles are `Register`, `Access`, and
  `Address (A a, SELECT.DPBANKSEL)`. Closed exact header-role matching rejects the qualified address label, so the
  otherwise complete register/access table never reaches the register synthesizer.

The repair boundary is generic. A structural role may accept a parenthesized qualifier only when the complete
unqualified head is already a closed role. Within an already-classified register map, an explicit `Type` access
role or an unambiguous column of access literals may carry access. Neither rule may inspect a document, vendor,
protocol, filename, symbol, expected score, or reviewed result.

## `.f.iv.a` repair and target result

The production repair implements that exact boundary in both SourceIR classifiers and EvidenceIR. A header may
be a closed role followed by one nonempty parenthesized qualifier; arbitrary suffixes remain unknown. Register
access comes from an explicit access/RW header or exactly one non-identity column whose every body row supplies a
closed access literal. The unique-column rule excludes names, offsets, resets, descriptions, and bit positions;
blank or missing values do not authorize the column.

Direct fresh-PDF qualification now gives the intended three-cell result:

- Arm Debug `table_0044` is `register_map` and yields all 12 reviewed register/access facts with exact table
  provenance;
- GIC-400 yields all 15 source-named registers with their exact offsets and RW/RO/WO access values; and
- AMD's packed layout remains `unknown`, so the former false register cannot return.

The feature-gated SourceIR migration seam also needed correction. Its schema-3 proof context already retained
the exact neutral capture, but the old implementation refreshed proof around stale classified fields and stale
validation. It now replays table/visual/section classification from those premises, reapplies grounded proposals,
and recomputes current validation without rerunning Docling or trusting the prior conclusion.

ADR 0025 reconciliation used an exact 144-file / 964,185,725-byte same-volume snapshot at SHA-256
`6e27f458f11afd5fd6090fc13de6daf7b07715121d7ca5b305b869c47b80cf76`. Only Arm SourceIR changes outside
proof/validation (`table_0044`: `unknown`→`register_map`). EvidenceIR, SemanticIR, and IntentIR change only Arm
and OpenCAPI Discovery: Arm replaces four section-derived placeholder records with 12 table-derived records
(74→82 total), while OpenCAPI keeps one register and gains its real `Attributes` access values. Only Arm's
blocked adapter payload changes (74→82 storage records); the clock/reset blocker and no-file state remain exact.
The independent chain oracle is 24/24 current and zero stale at all four replayed stages.

## Replay-driver follow-up owned by `.f.iv.b`

Reinspection against current source, parent revision `e125aac7^`, and the Rust example contract disproves the
earlier five-versus-four claim. The orchestrator already passes exactly four values after `--`: source, replay
root, prior memory, and `-` for no observed table. The earlier note counted replay root twice; any optional
`env KEY=value` entries precede `cargo` and are not replay positionals. `.f.iv.b` preserves the command bytes
through a named builder and pins the exact separator suffix plus environment prefix in a focused test. The clean
publication replay then completes 12/12 sources and 48/48 stages at `e125aac7`, publishing 39/0/1 TP/FP/FN,
42/42 provenance, 117/118 conservation, zero fabrication, and one APB unexplained drop.

## Storage and cleanup state

The diagnostic root contained exactly 3,913 files and occupied 1,458,132 KiB by `du -sk`. Eight external PDF
authorities were discovered once, verified hash-equal, and copied below the repository-volume root. After the
complete gate consumed the evidence, `.f.iv.a` removed the diagnostic root, source map, two target roots, and
rollback/comparison root: 5,112 files / 3,614,756 KiB across the five exact paths. The residue census finds all
five absent. The rollback copy is no longer recoverable; current generated artifacts remain reproducible and
currency-gated. `.f.iv.b` subsequently removed its own three attempt roots—5,616 files / 2,038,580 KiB—and runtime
source map with no residue after promoting the successful 3,094-file / 1,147,260-KiB authority root.
