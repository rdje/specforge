# Rolling-ledger archive-route containment design

Owning leaf: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.9a` (PROBE/DOC). This report measures and selects the
`.9b` migration contract. It changes no archive member, manifest consumer, executable checker, threshold, or
ceiling.

## Boundary and measurements

The shared `docs/archive/rolling-ledgers/INDEX.md` is 81 lines / 5,311 bytes with a 121-byte maximum content
line. Its health target is 96 lines / 6,144 bytes and its ceiling is 128 / 8,192; mandatory byte rollover starts
at 5,530 bytes. Only 218 bytes remain. The last live-status segment route occupies 3 lines / 301 bytes, so another
same-shaped route would cross the boundary by at least 83 bytes. The route must be contained before another
segment is admitted.

Current per-ledger blocks in the shared index are:

| Ledger | Lines | Bytes |
| --- | ---: | ---: |
| changes | 17 | 1,124 |
| development-notes | 18 | 1,153 |
| live-achievement-status | 20 | 1,529 |
| rust-codebase-analysis | 16 | 1,162 |

The shared JSONL manifest is 10 lines / 7,673 bytes: one 136-byte control record and nine data records. Its
largest current record is 983 bytes. The unchanged controls allow 32 data records, 32,768 total bytes, 2,048
bytes per record, and 768 bytes per scalar.

| Ledger | Data records | Data bytes | Capsule row | Largest segment row |
| --- | ---: | ---: | ---: | ---: |
| changes | 2 | 1,563 | 658 | 905 |
| development-notes | 2 | 1,606 | 705 | 901 |
| live-achievement-status | 3 | 2,652 | 755 | 972 |
| rust-codebase-analysis | 2 | 1,716 | 732 | 984 |

Existing segment-surface ceilings allow at most 28 segment files for each ledger. Projecting one capsule and 28
segments at each ledger's largest measured row gives 26,134, 26,069, 28,107, and 28,420 bytes respectively.
Each projection is 29 data records and stays below both unchanged manifest controls. A concise per-ledger index
at that boundary projects to about 40 lines and 6,776, 7,056, 7,224, and 7,196 bytes respectively.

The live-status root currently contains 64 records / 107 lines / 89,011 bytes with a 4,824-byte maximum line.
Its next record rollover is at 72. The route migration therefore precedes any next sealed segment.

## Consumer and topology census

The exact shared index path has eight current occurrences across four files: four ledger registry references,
one live-document surface target, the mdBook, and this protocol's existing fact card. The exact shared manifest
path has 17 occurrences across three files: four ledger registry references, 12 archive-surface `archive_manifest`
references, and the fact card. The checker follows registry data and does not hardcode either shared path.

The manifest already requires scalar `predecessor` and `successor` fields, but
`scripts/check_rolling_ledger_protocol.pl` validates only their presence. It does not prove reciprocal edges, a
single complete chain, acyclicity, or absence of disconnected segments. `validate_archive_index` likewise
searches for selected strings rather than proving exact complete membership and chain order. Existing chronology
claims are therefore stronger than executable enforcement. `.9b` must repair this as part of the route migration.

## Selected topology

ADR 0017 selects one fixed bounded landing and four per-ledger authority partitions:

1. `docs/archive/rolling-ledgers/INDEX.md` becomes a stable landing with exactly four ledger routes. It does not
   list capsules or segments and therefore does not grow when one is added.
2. Each existing ledger directory gains `INDEX.md` and `manifest.jsonl`. Its index directly links the live root,
   every segment, the capsule, and its manifest in verified newest-to-oldest order. Its manifest repeats the
   current bounded control row and contains only byte-identical records for that ledger.
3. Each registry entry gains a `landing` path; its `index` and `manifest` paths move to its own directory. The 12
   archive surfaces consume the matching per-ledger manifest.
4. The landing retains its current 96-line / 6,144-byte health targets and 128 / 8,192 ceilings. The four indexes
   form a `partitioned_canonical` surface using the landing as `external_membership`.

The new per-ledger index surface has these measured bounds:

| Dimension | Health | Ceiling |
| --- | ---: | ---: |
| Files | 4 | 4 |
| Lines per file | 64 | 96 |
| Bytes per file | 8,192 | 12,288 |
| Maximum line bytes per file | 512 | 1,024 |
| Aggregate lines | 256 | 384 |
| Aggregate bytes | 32,768 | 49,152 |

No existing target or ceiling widens. The maximum ordinary read is two bounded hops: landing → ledger index →
member. Immutable history remains optional rather than a mandatory bootstrap or current-read dependency.

## Atomic `.9b` migration and acceptance

The implementation must:

1. copy the shared control row and exact data lines into four per-ledger manifests;
2. prove the four-part data-line union equals the retired manifest's data lines byte-for-byte, grouped without
   normalization or member rewriting;
3. create the four complete per-ledger indexes and the fixed landing;
4. switch all four registry records, all 12 archive-manifest consumers, and current documentation together;
5. require one complete simple chain from live source through every segment to the capsule, with reciprocal
   predecessor/successor agreement and no duplicate, missing, cyclic, or disconnected member;
6. require every member path exactly once in the per-ledger index and in verified chain order, plus every ledger,
   live source, index, and manifest route in the landing;
7. verify all identities and direct retrieval, then delete only the exact retired shared manifest and prove no
   current consumer or filesystem residue remains.

Fail-closed fixtures must cover a foreign-ledger manifest record, missing and duplicate edges, a broken
successor, a cycle, a disconnected segment, a missing landing route, a missing or duplicate member link, wrong
index order, and index/manifest size overflow. Capsules and segments remain byte-identical. Future exhaustion of
one ledger partition requires a separately task-owned partition decision rather than a ceiling increase.

## Acceptance disposition

- **Reproduce/measure:** exact shared-route dimensions, growth, per-ledger blocks, manifest rows, reader paths,
  and future capacity are recorded above.
- **Root cause:** segment growth from four independent ledgers is multiplexed into one append-growing route; the
  verifier also treats declared chronology as unvalidated prose-shaped metadata.
- **Addressed design:** stable landing plus per-ledger bounded authority partitions and complete chain/index
  validation remove both failure modes without losing direct retrieval.
- **No regression:** this leaf changes only planning/current-truth documents. `.9b` owns every executable and
  archive mutation, exact migration proof, focused mutation, and resulting-tree gate.

## Adjacent pressure surfaced by the resulting tree

The composed report identifies `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md` as the largest
`task_evidence` member at 2,420 lines / 229,064 bytes. Its 278,528-byte health target gives a 222,823-byte warning
and a 250,676-byte rollover threshold; it is 6,241 bytes past warning with 21,611 bytes of pre-rollover headroom. This is not
an archive-route implementation concern, so `.9a` does not fold in a second migration. It opens `.10a` to design
a lossless bounded current/history task-tree topology and `.10b` to implement it. No trimming or ceiling increase
is authorized, and `.9b` must keep its own task-tree update within the remaining rollover margin.
