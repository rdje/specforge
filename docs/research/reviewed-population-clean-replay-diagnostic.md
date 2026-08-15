# Reviewed-population clean replay — structural-carrier diagnostic

- Date: `2026-08-15`
- Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv.a`
- Production revision: `0d2181167d81d9418e140b2d09c1c5aecbfd3197`
- Reviewed authority: 12 documents / 14 cells / 48 isolated stages
- Diagnostic root: `.project-data/tmp/spec-to-intent-f-iv-population-r1`
- Runtime source map: `.project-data/tmp/spec-to-intent-f-iv-source-map.json`

## Why this result is diagnostic, not current authority

The first `.f.iv` replay completed all 12 hash-equal sources and all 48 stages from a clean production revision.
The output is intentionally retained below repository-local scratch while its regressions are repaired. It has
not replaced the tracked `b977a51f` result, controller input, report, or portable replay manifest.

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

## Storage and cleanup state

The diagnostic root contains exactly 3,913 files and occupies 1,458,132 KiB by `du -sk`. Eight external PDF
authorities were discovered once, verified hash-equal, and copied below the repository-volume root. Both runtime
paths are untracked and remain in flight until `.f.iv.a` verification consumes them. `.f.iv.b` will run from the
clean repair revision, publish only portable identities, then delete the exact diagnostic/replay roots and source
map with a residue census.
