---
id: rolling-ledger-record-grammars
title: The four root rolling ledgers use three explicit whole-record grammars
answers:
  - "how are CHANGES DEVELOPMENT_NOTES LIVE_ACHIEVEMENT_STATUS and RUST_CODEBASE_ANALYSIS split into records"
  - "what is the rolling ledger archive protocol"
  - "why are two containment records at the bottom of CHANGES"
  - "what protects the validation projection when LIVE_ACHIEVEMENT_STATUS rolls over"
  - "where is the first post-migration LIVE_ACHIEVEMENT_STATUS rollover segment"
  - "where is the second LIVE_ACHIEVEMENT_STATUS rollover segment and what blocks the next one"
  - "where is the first post-migration DEVELOPMENT_NOTES rollover segment"
  - "how will the shared rolling ledger archive index be partitioned"
  - "does the rolling ledger verifier validate predecessor successor chronology"
  - "which root rolling ledger is currently above its rollover signal"
  - "was the warning-safe rolling ledger transaction independently audited from a clean clone"
date: 2026-08-08
status: current
tags: [documentation, rolling-ledger, archive, continuity, validation]
evidence: doctrine/live_document_size/rolling_ledgers.jsonl; docs/archive/rolling-ledgers/INDEX.md; docs/archive/rolling-ledgers/changes/manifest.jsonl; docs/archive/rolling-ledgers/development-notes/manifest.jsonl; docs/archive/rolling-ledgers/live-achievement-status/manifest.jsonl; docs/archive/rolling-ledgers/rust-codebase-analysis/manifest.jsonl; docs/research/root-rolling-ledger-pressure-plan.jsonl; docs/research/root-rolling-ledger-pressure-audit.md
reverify: perl scripts/check_rolling_ledger_protocol.pl --report
---

`CHANGES.md` is a mixed grammar: leading modern `###` records, then legacy `##` records, plus the
exact `.0` and `.1` containment headings detached below the 2026-03-31 tail. `DEVELOPMENT_NOTES.md`
uses every H2 after its H1 as a record. `RUST_CODEBASE_ANALYSIS.md` keeps `## Purpose` in its
prologue and treats later H2 sections as records. `LIVE_ACHIEVEMENT_STATUS.md` treats each top-level
bullet between `## Current snapshot` and `## Highest-priority remaining gap` as one record; its
trailer includes the generated validation projection.

The bounded registry pins each grammar, source identity, planned newest-first whole-record window,
local limits, reader/writer paths, and repository-relative archive route. The checker reconstructs
every source byte-for-byte and verifies the planned live view. After migration it switches identity
authority to an immutable exact source capsule, verifies its manifest/index, enforces the bounded
root, and requires the retained migration records to remain an exact suffix behind any newly
prepended records. The validation writer's root path and managed markers are required literals.

`CHANGES.md` completed the first migration in `.4b`. Its immutable 1,798-record capsule is
`docs/archive/rolling-ledgers/changes/source-through-2026-08-08.md` with SHA-256
`d89809322857aab3c506dde1cc6caaf57e22d0349655b37ddaab1f7bf22ba994`; the root is the bounded current
view and `docs/archive/rolling-ledgers/INDEX.md` is the retrieval route.
At clean commit `4d24b13c`, the owned pressure census measured this root at 114 records / 1,645 lines / 222,725
bytes, or 91.4% of line health. Development was 89.3% of line health, status was 80.7% of byte health, and Rust
was 87.9% of line health. The focused checker incorrectly applied milestones to enforcement ceilings. `.1` binds
the generic health authority and has materialized exact 29/24/12/8-record cuts through the existing per-ledger
index/manifest topology. The changes segment is `segment-0002-2026-08-09.md`: 29 records / 428 lines / 37,189
bytes / SHA-256 `bc87665975d80078697384e89b2127e67164ae0e523a8076b93f81eb5582fbd8`; the current root is 87 records /
1,246 lines / 188,183 bytes after `.1`. The closure record makes the current root 88 records / 1,259 lines /
189,197 bytes.

`DEVELOPMENT_NOTES.md` completed the second migration in `.4c`. Its immutable 1,601-record capsule is
`docs/archive/rolling-ledgers/development-notes/source-through-2026-08-08.md` with SHA-256
`76b51a3f450cdb1e764922dc366cf6ff55529cb95f7a6f410bfba1f1f378fedc`. Its first post-migration rollover is
`docs/archive/rolling-ledgers/development-notes/segment-0001-2026-08-08.md`: 27 whole records / 467 lines /
36,125 bytes / SHA-256 `b07a73670863e283f359911629db691859f41d99021a1902db3adfeb83788b91`.
That root contained the H1 prologue, 10 newest post-capsule records, then the exact 50-record retained migration
suffix. The next rollover seals 24 records in `segment-0002-2026-08-09.md`: 421 lines / 34,258 bytes / SHA-256
`0576c44b7b95c40bc4669128ce07ebdc6a99b1ebbe9322cd1aca3acad8c04cbd`. The current root contains two later
task records, the ten newest opening records, and the exact 50-record suffix: 62 records / 1,294 lines / 175,215
bytes after `.1`. The closure record makes the current root 63 records / 1,306 lines / 176,145 bytes. Segment
0001's final former-successor separator remains represented by the protocol's canonical terminal newline.

`LIVE_ACHIEVEMENT_STATUS.md` completed the third migration in `.4d`. Its immutable 1,920-record
capsule is
`docs/archive/rolling-ledgers/live-achievement-status/source-through-2026-08-08.md` with SHA-256
`b00ff5f5c4a29554a20eea9d901a95848d54a644749eba74ad9618798dd9bd6a`; its root keeps the H1/current
heading, 50 retained snapshot bullets, the complete gap/validation trailer, and later prepends. The
real project-validation writer test proves that both managed markers still work at the stable root.
The first post-migration rollover sealed the 12 oldest new prepends byte-for-byte in
`docs/archive/rolling-ledgers/live-achievement-status/segment-0001-2026-08-08.md`, linked the newer live root
to the older source capsule through the manifest/index, and reduced the live window from 72 to 60 records
without touching the complete trailer or the exact 40-record migration suffix.
The second rollover sealed the next 12 records in
`docs/archive/rolling-ledgers/live-achievement-status/segment-0002-2026-08-08.md`: 12 lines / 10,894 bytes /
SHA-256 `11e058831b33b71bbd7cb0a080cbf6024032ed96587b1a7d7c340945a58328df`. Direct extraction from the
pre-removal Git content proves byte identity; the manifest chain orders live root → segment 0002 → segment
0001 → capsule, and the root again retains 20 new records plus the exact 40-record suffix. The added route
makes the former shared index 81 lines / 5,311 bytes, only 218 bytes before its 90% byte rollover; `.9a` designed
and `.9b` landed the bounded partitioned authority before another segment.
The third rollover seals 12 records in `segment-0003-2026-08-09.md`: 12 lines / 9,181 bytes / SHA-256
`9bb1607d89fcc67ea2c1824a9c84e21bd95d56f7f0f49a1af43fee9d3b865edb`. The fourth seals 12 more in
`segment-0004-2026-08-09.md`: 12 lines / 11,420 bytes / SHA-256
`6d1c07596313d0c98c04afde63dd23d7df61fda291bd50d2cbc57a0568b924eb`. The current status root has 19 new
records, the exact 40-record suffix, and its complete trailer: 59 records / 101 lines / 82,836 bytes.
The closure bullet makes the current status root 60 records / 102 lines / 83,464 bytes.
`RUST_CODEBASE_ANALYSIS.md` completed the fourth migration in `.4e`. Its immutable 1,350-record
capsule is
`docs/archive/rolling-ledgers/rust-codebase-analysis/source-through-2026-08-08.md` with SHA-256
`95e1665628b615498e94f67d6dc6e0083d4d104a6ca8a815b4c23cf2f97cc7ce`; its root is exactly the
H1/Purpose prologue plus newest 55 H2 records. The lifecycle-only `.4e` slice added no architecture record, but
the generic material checker in `.1` is a material checker architecture change. Its second segment seals eight
records in `segment-0002-2026-08-09.md`: 134 lines / 12,009 bytes / SHA-256
`2444822e5f98f04516a20a9a54468e57f99721afcf869c37a12da0bc538bacb6`. The current root is 55 records /
1,064 lines / 89,706 bytes.
All four `migrated` registry entries now share only the fixed landing; each points to a distinct bounded index and
manifest inside its ledger directory. The landing is 34 lines / 1,512 bytes. The four complete indexes are 42
lines / 1,750 bytes in aggregate, with the 12-line / 579-byte status index as the largest part. Existing per-ledger manifest
capacity remains sufficient through every declared 28-segment surface ceiling without widening a control;
direct retrieval is at most two bounded hops.

The four manifests repeat the unchanged bounded control row and now carry 3, 3, 5, and 3 data records. Every old
capsule and segment remains byte-identical. The schema-v2 registry declares the retired path, and the checker
rejects any residue.

The checker now proves reciprocal edges, one complete acyclic live→segments→capsule chain, and absence of foreign
or disconnected members. Each per-ledger index must link every chain member exactly once in verified order plus
its manifest, while the landing must expose exactly the four registry-ordered ledger routes. Thirty-five focused
parser/control/chain/route/transaction mutations and 55 generic lifecycle/overflow cases pass. The transaction
authenticates the committed boundary and exact range, stages on the repository volume, installs the root last,
and restores exact prior bytes under an injected failure without residue.

The `.2` closure independently clones commit `10d182ff` without local hardlinks beneath `generated/` on the same
device, validates Git objects and all four chains, and passes all six doctrines after populating the clone's
normally uninitialized `subs/fsmgen` gitlink at exact commit `d327129b`. One temporary valid record through each
grammar leaves 88/63/60/56 records and 1,250/1,298/102/1,068 lines, all below warning. The audit removes those
fixtures exactly, proves a clean clone and zero transaction residue, then deletes the workspace.

For an H2 live cut, one blank line immediately before the removed successor is boundary structure,
not retained record content. The capsule preserves it exactly; the root renderer omits exactly that
one separator at EOF, and the suffix verifier permits no other difference. This prevents a
blank-at-EOF artifact without weakening content identity or normalizing archive bytes.
