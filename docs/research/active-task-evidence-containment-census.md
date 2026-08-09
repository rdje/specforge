# Active task-evidence containment census

- Date: `2026-08-09`
- Owner: `ACTIVE-TASK-EVIDENCE-CONTAINMENT.1.1`
- Target: `docs/tasks/PDF-VARIANT-DIGESTION.md`
- Mode: read-only target census; no migration or target correction

## Result

The target is not one coherent current-state document. It is an active task identity plus a small program
contract, several semantic activity ledgers, a stale current-frontier narrative, historical verification and
changelog evidence, and durable links from product documentation and facts. Its 1,392-line `Current frontier`
section alone contains 124,201 bytes (55.8% of the complete source) because later activity specifications were
appended beneath that heading rather than represented as a bounded current view.

The source also has no internally consistent eligible frontier. Durable commits and inline verification close
work whose older node headers remain active or in progress; two formally pending nodes are described elsewhere
as blocked; and the formal `Blockers` section says `None`. A safe migration must preserve the complete literal
source while establishing one separately explicit current-state authority. Mechanical heading sharding cannot
resolve that contradiction.

The measurements favor a hybrid for `.1.2` to decide: a bounded active root and semantic activity route for
ongoing writes, backed by an exact pre-migration source capsule and manifest. The capsule would preserve every
legacy byte but would not make the active root terminal; future work would continue through bounded live
partitions with a declared update/rotation protocol.

## Exact source boundary

The `.0` commit owns the unchanged source boundary:

| Property | Value |
| --- | --- |
| Repository boundary | `684079b1d8c30b2c94a2d8efb2d2b8efdd3f3195` |
| Last target-changing commit | `1abfb49c19f48baa8525349c382c1a7d7b6ef93a` (`2026-06-24`) |
| Path-touching commits | 70 |
| Lines / bytes / maximum content-line bytes | 2,393 / 222,616 / 191 |
| SHA-256 | `9284dce40ad896c3de3811e95c3fdd347132b1849083499e9c543fc9026a19d4` |
| Git blob | `7d89ea4fe59e53e55614b5730bded40d99b4b555` |
| Existing byte warning distance | 207 bytes |

`git diff --exit-code -- docs/tasks/PDF-VARIANT-DIGESTION.md` remains empty throughout this census.

## Semantic source census

The table is an exhaustive, non-overlapping partition of all 2,393 lines and 222,616 bytes. The subactivity
boundaries inside the oversized frontier are semantic labels already present in the source, not arbitrary size
cuts.

| Lines | Count | Bytes | Role |
| --- | ---: | ---: | --- |
| 1–42 | 42 | 2,524 | stable identity, metadata, goal, non-goals, acceptance, task-tree heading |
| 43–374 | 332 | 35,309 | early formal node declarations, dominated by `.9` plus the `.1` node |
| 375–421 | 47 | 3,283 | `.1` triage measurements and whole-corpus refinement |
| 422–494 | 73 | 7,740 | legacy frontier narrative through the parked serial work |
| 495–1325 | 831 | 73,432 | `.10` register/message-field shape recovery activities |
| 1326–1383 | 58 | 4,763 | `.11` validation-surface integration |
| 1384–1585 | 202 | 17,174 | `.12` header-trapped signal/presence work |
| 1586–1813 | 228 | 21,092 | `.13` canonical corpus refresh work |
| 1814–2047 | 234 | 24,227 | planned/implemented `.2`–`.8` quality and breadth work |
| 2048–2064 | 17 | 731 | global decisions, open questions, and blockers |
| 2065–2260 | 196 | 19,383 | historical verification evidence |
| 2261–2383 | 123 | 12,243 | historical changelog |
| 2384–2393 | 10 | 715 | durable PDF tooling/multi-strategy note |

Every candidate semantic live part can remain below 75 KiB before navigation overhead if `.1.2` groups these
regions by task activity rather than retaining the present 124 KiB frontier section. That is capacity evidence,
not a chosen topology or license to rewrite the source.

## Current-state contradiction census

The target metadata establishes only that the top-level program is active. It does not establish a compliant
current frontier:

- The `Current frontier` heading names `.9.3` as the active leaf, while the same narrative records `.9.3a` and
  `.9.3b` done.
- `.9.3b` is declared `in_progress` at line 98, records `done` at line 114, and has completion commit
  `4c9c8866`.
- `.9.7` is declared `in_progress` at line 276, records `done` at line 300, and has completion commit
  `411e889f`.
- `.10` remains textually `in_progress`, although `.10a` through `.10i` and the `.10p` probe have ID-bearing
  commits; `.10g`, `.10h`, and `.10i` are explicitly done in their later node declarations.
- `.12` remains textually `in_progress`, although `.12a` and `.12b` have completion commits.
- `.13` remains textually `in_progress`, while `.13c` states the sweep is complete and `.13a`–`.13d` have
  ID-bearing commits (including the `.13b.1` repair).
- `.6` and `.7` are formal `pending` nodes, but the frontier narrative calls them blocked on host-local PDFs;
  the formal `Blockers` section nevertheless says `None`.
- The source contains 37 canonical indented or top-level `- ID:` declarations but Git history contains 52 unique ID-bearing commit
  subjects, showing that much of the later topology lives in prose-shaped activity blocks rather than the
  repository's current node form.

Therefore `.1.1` cannot truthfully nominate a current eligible PDF leaf. It establishes the narrower authority
facts needed by `.1.2`: completed commits and verification evidence must not be demoted by stale headers;
parked/blocked work is not eligible; and any normalized current-state table must be an explicit, reviewable
reconciliation while the capsule retains every literal legacy statement.

## Reader inventory

### Stable-route consumers

At committed census input `f1183e64` (before this slice's own outputs), 32 tracked files outside the target contain
its exact path. Three are immutable rolling-ledger source capsules, leaving 29 current input files. No consumer
uses a `#fragment` or query suffix, so retaining the stable root path plus a complete leaf-to-part route avoids
forced link rewrites.

Current direct-route consumers are:

- `ROADMAP.md`.
- Two mdBook chapters with 18 total citations: `docs/book/src/pipeline/evidenceir.md` and
  `docs/book/src/quality/extraction-eval.md`.
- Twenty-four Knowledge Map fact cards:
  `agnostic-quoted-mode-fsm`, `bit-assignment-register-table-extraction`,
  `bit-location-register-field-vocabulary`, `bit-position-structure-field-extraction`,
  `byte-location-structure-field-extraction`, `can-composition-frame-fields`,
  `corpus-register-table-shape-gap`, `corpus-reuse-serial-prose-lever-not-cluster-scopable`,
  `definitional-signal-capture`, `document-class-from-structure`, `header-trapped-signal-table-recovery`,
  `message-field-validate-integration`, `offset-suffixed-dword-relative-bit-cells`,
  `prior-phrase-utf8-byte-as-char`, `prose-signal-capture-i2c-precision`,
  `register-field-eval-measure-and-surface`, `register-field-table-extraction`,
  `section-header-message-field-extraction`, `section-header-register-block-qualification`,
  `section-header-register-field-extraction`, `section-header-register-identity-collapse`,
  `signal-presence-matrix-capture`, `timing-table-trapped-row-recovery`, and
  `transition-bound-state-fsm`.
- `docs/research/task-evidence-terminal-containment-design.md` and the owning
  `docs/tasks/ACTIVE-TASK-EVIDENCE-CONTAINMENT.md`.

The three immutable exact-path occurrences live in the source capsules for `CHANGES.md`,
`DEVELOPMENT_NOTES.md`, and `LIVE_ACHIEVEMENT_STATUS.md`; they are evidence, not live consumers, and must remain
unchanged.

### Generic structural readers

- Startup/PNT reads `MEMORY.md`, then the derived catalog, then the stable task root's `Current Frontier`.
- `scripts/check_task_tree_catalog.pl` scans only direct Markdown children of `docs/tasks/`, requires the
  filename/H1 id match, and parses the first metadata `Status`. The stable root must remain a direct child.
- `scripts/check_roadmap_projection_contract.pl` requires roadmap task owners to exist in that complete catalog.
- `scripts/check_task_acceptance.sh` treats any staged `docs/tasks/.*.md` as possible Rust-change ownership and
  scans the staged task text for evidence-backed checklist fields. The active write contract must say which root
  and/or part is staged for a completed product leaf.
- `scripts/check_live_document_size.pl` currently classifies only `docs/tasks/*.md` as `task_evidence`. Any
  nested or differently rooted live partitions need their own exactly-once surface declaration before landing.
- `COMMIT.md` requires the owning task file's node status, verification, commit log, and frontier to change with
  each completed slice. A partition design must preserve that single-slice atomicity.

### Identity references that do not read the task Markdown

At the same committed input, 89 tracked files outside the source contain `PDF-VARIANT-DIGESTION` somewhere. The
exact-path consumers above are the route-sensitive subset. Eleven Rust/Python files use leaf IDs only in source
comments or help text; 16 other task trees use the ID for cross-task dependencies; the remaining occurrences are
ledgers, research, facts, derived question shards, corpus records, and analysis. No script, hook, or CI file
contains the exact target path, and no executable opens or rewrites it. These identifier references require stable
leaf identity, not retention of the monolithic layout.

## Writer inventory

There is no automated writer for the target. Its 70 path-touching commits follow the human/agent task workflow:

1. Select or split a leaf and update its status/frontier.
2. Implement and verify the slice.
3. Add node acceptance/results, verification evidence, commit subject, decisions, and changelog context.
4. Stage the owning task evidence with live-doc impacts and commit atomically.
5. Regenerate `docs/TASK_TREE.md` only if the root H1/status/catalog membership changes.

The new architecture therefore needs a mechanically declared writer target per information role. A future agent
must not choose among root, activity part, event segment, or capsule by convention alone.

## Reconstruction and migration invariants

`.1.2` must preserve or explicitly enforce all of these:

1. The source boundary's 2,393 lines / 222,616 bytes / SHA-256 / Git blob remain independently retrievable.
2. The stable path retains the exact tree id, active metadata, goal/non-goals/acceptance, and a truthful bounded
   current frontier or explicit no-eligible-frontier state.
3. Every legacy leaf id, owner direction, decision, measurement, verification record, commit subject, changelog
   entry, and tooling note remains directly reachable from the stable root in bounded hops.
4. Literal historical contradictions remain in immutable evidence; the current root never presents them as
   current truth.
5. A complete leaf-to-part index covers all known ids, including prose-shaped later activities and unusual legacy
   identifiers, without renumbering.
6. Every live part has fixed line/byte/width bounds, collection/count bounds, and a declared rotation or closure
   rule; an exact capsule alone does not govern future active growth.
7. The task catalog, roadmap projection, task-acceptance gate, live-document coverage, mdBook citations, Knowledge
   Map evidence routes, and COMMIT workflow continue to resolve without heuristic fallback.
8. All project-owned migration data, temporary workspaces, manifests, and generated checks stay on the repository
   volume with repository-relative persisted paths.
9. Migration changes task-document architecture only; it does not implement a PDF feature, silently alter a leaf
   status, or claim an unverified product frontier.

## Candidate topologies for `.1.2`

### A. Hybrid active root + semantic live parts + exact source capsule

Keep a bounded active root for identity/current frontier and a bounded index/manifest. Preserve the `.0` source
exactly as a historical capsule. Route completed activity detail into semantic parts; write future leaf contracts
and evidence to the owning bounded activity part while atomically refreshing the root frontier. Rotate or close a
part by declared semantic/activity rules. This best fits the measured roles but needs a new neutral active-tree
contract; it is not ADR 0018's terminal root.

### B. Semantic live parts only, with exact reconstruction

Partition every source byte among semantic canonical parts and prove concatenated reconstruction through a
manifest. This avoids a duplicate capsule but makes the legacy noncontiguous activity layout and contradictory
current text part of the live canonical collection. It has higher migration and writer complexity and a larger
mandatory aggregate.

### C. Rolling chronological segments

Seal commit/event records into time segments and keep a current window. This fits the verification/changelog
tail but not the dependency graph, decisions, or nonchronological activity specifications. It would need a second
semantic authority and is therefore not sufficient alone.

The census recommends that `.1.2` evaluate A as the primary candidate, use chronological segmentation only for
future append-heavy evidence if needed, and reject any design that treats the exact capsule as the active writer
destination.

## Decision disposition

`.1.2` accepted candidate A in
[ADR 0019](../decisions/0019-bounded-active-task-root-and-semantic-evidence-parts.md). The stable root remains
active current authority; seven bounded semantic parts provide routine historical/detail reads; and the exact
committed source becomes immutable provenance. Migrated legacy payloads are sealed. Future work creates a new
bounded activity part and atomically updates root + owning part, splitting only at a child/container boundary
before rollover. The normalized root initially has no eligible PDF frontier; `.6`, `.7`, and `.9.10` require
explicit revalidation rather than silent resumption.

## Reverification

Run from the repository root:

```sh
shasum -a 256 docs/tasks/PDF-VARIANT-DIGESTION.md
git hash-object docs/tasks/PDF-VARIANT-DIGESTION.md
wc -l -c docs/tasks/PDF-VARIANT-DIGESTION.md
git diff --exit-code -- docs/tasks/PDF-VARIANT-DIGESTION.md
git log --format='%H' -- docs/tasks/PDF-VARIANT-DIGESTION.md | wc -l
git grep -l -F 'docs/tasks/PDF-VARIANT-DIGESTION.md' f1183e64 -- ':!docs/tasks/PDF-VARIANT-DIGESTION.md'
git grep -l -F 'PDF-VARIANT-DIGESTION' f1183e64 -- ':!docs/tasks/PDF-VARIANT-DIGESTION.md'
```
