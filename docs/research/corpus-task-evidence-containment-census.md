# Corpus task-evidence containment census

Owning leaf: `CORPUS-TASK-EVIDENCE-CONTAINMENT.2` (DOC/MEASURE/DESIGN).

## Boundary and pressure

The immutable source boundary is `docs/tasks/CORPUS-COVERAGE.md` at commit
`d78d842e4e7fc8d1fd902937cf6902f07d96b68e`, Git blob
`d7ac9aa2c07723cb4a7a8f3a15332ef84fe09f07`, and SHA-256
`5d7acb0c973a75d821c9d5a963aac5c3899da19261dcf6427274cb123804123f`. It has 2,308 lines / 277,636 bytes /
4,746 maximum content-line bytes. The width follows the live-document doctrine: it excludes LF and an optional
preceding CR. The `.1` diagnostic included LF and therefore reported 4,747; that off-by-one notation is corrected
in the owning tree without changing the locked source.

The registered `task_evidence` ceiling is 278,528 bytes, leaving 892 bytes. Across the fourteen latest
path-touching intervals, the source grew by 57,886 bytes: mean 4,134.7, median 3,684.5, minimum 1,215, and maximum
8,949 bytes per interval. Even the smallest recent interval exceeds the remaining margin. Refresh #49 cannot be
honestly owned in the monolith.

The source has 94 path-touching commits through the boundary. Its formal task surface has 48 distinct `- ID:`
declarations, 40 acceptance/parent-exit headings, 48 refresh-ledger rows, and 65 dated changelog records. Git
history has 41 distinct ID-bearing subjects; all are included in the 48 formal ids, so unlike the first active
task migration there are no commit-only ids or shorthand-only routes.

## Exact source-role partition

Seven natural regions account for every byte exactly. Each boundary is a task/activity, ledger, or chronology
boundary rather than an arbitrary size cut.

| Region / planned part | Source lines | Lines | Bytes | Max content-line bytes | SHA-256 |
| --- | ---: | ---: | ---: | ---: | --- |
| `program-and-latest-refreshes` | 1–469 | 469 | 43,000 | 147 | `33b9e61f33081fc558638128ea1a0c728c4d401d3ee6c92d3f2054496f53593f` |
| `refreshes-33-40` | 470–954 | 485 | 46,968 | 152 | `7d06b66249b529c7628bf1426890d1ab7424d2179ea0b77b388e62772b3e0007` |
| `refreshes-41-43` | 955–1,372 | 418 | 38,409 | 124 | `8f0caf40f64cd1a3ec393d252ebaf30bcc6d1299d3903ff7baa195a243913eef` |
| `repair-acceptance-and-frontier` | 1,373–1,660 | 288 | 26,217 | 150 | `2a5769cf9d79036c1fca9174dcce49a20a2de704e61cb6e91e744c61bdcd51dd` |
| `refresh-ledger` | 1,661–1,750 | 90 | 59,259 | 4,746 | `934fa91d9a6d9b837c99b10bfa8bcf84a15275db7ddd2fea4a127db1d32a1de9` |
| `recent-chronology` | 1,751–2,080 | 330 | 37,780 | 1,459 | `20bf16fe43797d44c61a3de39c8dc40c2977b0b3d3f0bcb50e873120b1f0065d` |
| `legacy-chronology` | 2,081–2,308 | 228 | 26,003 | 180 | `4e3162ba6220e44b3c17a3cc25a1f1b6fbdc1658b61b1b3a83d29f95acc91e27` |
| **Total** | **1–2,308** | **2,308** | **277,636** | **4,746** | source identity above |

The first four regions own all 48 formal routes: 12, 26, nine, and one respectively. Forty-one are `legacy`
routes authenticated by both completion-subject history and their source literal; the seven formal container ids
absent from completion subjects are source-backed `structural` routes. The ledger and chronology regions retain
complete cross-cutting evidence but need no primary leaf route. Marker scaffolds make each payload independently
hashable; concatenation in source order reconstructs the exact source.

## Current-state reconciliation

The source's final evidence is coherent, but its current-facing structure is not:

- metadata still says 47 refreshes and names `.2.47` as latest;
- the actual current boundary is recorded by row 48, the cumulative statement after that table, the newest
  changelog record, current roadmap/status/book surfaces, and the committed `.2.48` result;
- the containment prerequisite appears as a `Frontier:` bullet inside the `.3` material, not a required bounded
  `## Current Frontier` section; and
- the same paragraph deliberately retains older phase narration such as 39 done / 17 remaining. That remains
  valid history but must not be presented as current state in the root.

Current authority is therefore: top-level `CORPUS-COVERAGE` active; `.0`, `.1`, and `.3` done; `.2` active; 48 of
56 real chip-spec refreshes done and eight remain; stage census 80 SourceIR / 20 normalized / 80 EvidenceIR / 79
downstream chains; 59/59 emitted ISFs strict-clean; no blocker. Containment deliberately does not select #49, so
the migrated root must say there is no eligible product leaf and name the next action: create and own
`CORPUS-COVERAGE.2.49` from the clean containment commit.

Literal stale statements remain unchanged in the exact capsule and legacy payloads. The bounded root is an
explicit current projection, not a rewrite of history.

## Reader inventory

At committed census input `3a40b152`, 42 tracked files outside the source contain its exact stable path:

- 33 Knowledge Map fact cards;
- three mdBook chapters (`pipeline/sourceir`, `pipeline/evidenceir`, and `quality/validation`);
- three root live documents (`ROADMAP.md`, `CHANGES.md`, and `DEVELOPMENT_NOTES.md`);
- the containment owner; and
- two immutable rolling-ledger source capsules.

No occurrence adds a Markdown fragment or query. No executable under `scripts/`, `crates/`, `.githooks/`,
`.github/`, or `doctrine/` opens the exact target. Keeping the stable root and a complete leaf-to-part index
therefore avoids bulk citation rewrites. In total, 105 tracked files mention the `CORPUS-COVERAGE` identity;
those references need stable ids, not the monolithic layout.

Generic readers impose four additional requirements:

1. startup/PNT reads `MEMORY.md`, the derived task catalog, and the stable root's bounded current frontier;
2. `scripts/check_task_tree_catalog.pl` reads only direct `docs/tasks/*.md` roots and requires stable H1/id/status;
3. roadmap projection expects the stable catalog route; and
4. task acceptance accepts nested `docs/tasks/.*.md` evidence, but the writer transaction must stage the root as
   well so current frontier/checklist state cannot drift from the owning detail part.

## Writer inventory and invariants

There is no automated source writer. The 94 commits were produced by the task-tree/`COMMIT.md` transaction: own
a leaf, append its contract and result, update ledger/changelog/current state, stage the task evidence, run gates,
and commit. The new topology must replace that one ambiguous destination with an explicit atomic transaction.

The migration and every future writer must preserve these invariants:

1. the locked 2,308-line / 277,636-byte source remains independently retrievable at its exact identity;
2. all seven raw regions and all 48 formal routes are covered exactly once;
3. the stable root remains the active current authority and catalog member;
4. completed legacy payloads seal; current normalization never mutates their bytes;
5. root, owning part, index, manifest, and route state agree within one commit;
6. every live/archive surface has exactly one registry classification and an unconditional verifier;
7. all staging, rollback, manifests, and test workspaces remain repository-relative and same-volume; and
8. refresh #49 remains a separate product slice with its own owning leaf and commit.

## Candidate topology disposition

### New top-level continuation only — rejected

A `CORPUS-COVERAGE-CONTINUATION` tree would change the established `.2.<refresh>` namespace, while the stable
root would either leave its `.2` parent falsely active without children or falsely mark an unfinished program
done. It reduces files but weakens continuity and route identity.

### Bounded root plus exact archive only — rejected

An exact capsule is necessary provenance but is an `archive_terminal`, excluded from ordinary task-authority
reads and future writes. Making 48 completed leaf contracts terminal-only would demote browsable canonical
evidence cited by 33 fact cards and three book chapters. An index pointing only at one 277 KiB capsule also fails
the working-set purpose of containment.

### Chronological rolling ledger — rejected as the primary topology

The 65-record changelog and 48-row result table are chronological, but the 48-node dependency/acceptance surface
is semantic. A ledger alone cannot own task hierarchy or current leaf contracts. The two chronology regions may
seal as semantic legacy parts inside the chosen collection.

### Bounded active root + seven semantic parts + exact capsule — selected

This is the smallest topology that preserves stable current authority, direct task browsing, all established
ids, and exact provenance. It reuses the already-neutral `scripts/check_active_task_evidence.pl` with a separate
corpus data contract; no second migration engine or corpus-specific parser is required. The corpus-specific
measurements are simpler than the first target: seven contiguous regions, 48 fully qualified routes, and no
legacy aliases.

## Corpus-specific bounds

Warning remains 80% and rollover remains 90%. These values do not widen an existing surface. Planned legacy
parts are at most 485 source lines / 59,259 bytes / 4,746 content-line bytes before a small marker scaffold.

| Surface | Health targets | Inclusive ceilings |
| --- | --- | --- |
| stable active root | 224 lines / 24,576 bytes / 512 max-line bytes | 320 / 36,864 / 1,024 |
| bounded index | 128 lines / 16,384 bytes / 512 max-line bytes | 192 / 24,576 / 768 |
| semantic parts | 12 files; 640 lines / 76,800 bytes / 6,144 max-line bytes each; 4,096 lines / 524,288 bytes aggregate | 16 files; 896 / 98,304 / 6,400 each; 6,144 / 655,360 aggregate |
| exact source capsule | exact 2,308 lines / 277,636 bytes / 4,746 max-line bytes | exact same values |

The manifest is schema-closed at 65,536 bytes, 1,024 maximum line/scalar bytes, 16 parts, 32 source regions, and
96 leaf routes. Seven sealed legacy parts leave one planned active `refreshes-49-56` part and ample fixed
capacity. Based on the recent mean, all eight remaining product records would add roughly 33 KiB; nevertheless,
the writer measures the owning active part before every slice and splits at a refresh boundary before the next
write would reach rollover. Warning triggers review, not threshold widening.

## Selected writer transaction

After migration, a corpus product leaf changes these authorities atomically:

1. stable root: current counts, one current leaf contract/checklist, frontier, blockers, and a bounded recent
   verification/commit window;
2. exactly one active semantic part: full leaf rationale, measurements, result, verification, and chronology;
3. index and manifest: the new fully qualified leaf route and exact part metrics/state; and
4. ordinary impacted live docs, book, and fact card per `COMMIT.md`.

The root and owning part are always staged. The index/manifest are staged whenever a route or measured part state
changes—which is every new refresh leaf. Completed parts seal; reopening creates a continuation part and link.
The focused corpus contract, all doctrines, and ordinary risk-proportionate gates run before every commit.

## Remaining migration stages

- `.3`: commit a separate `source_locked/complete` corpus contract, add its unconditional invocation, declare all
  seven regions/48 routes/bounds, and prove every destination absent while the source remains byte-identical.
- `.4`: materialize the existing raw-byte/root-last transaction, register the root/index/parts/capsule exactly
  once, verify all routes and reconstruction plus a positive future append, synchronize live docs/book/fact, close
  containment, and return the resume pointer to owning refresh #49.
