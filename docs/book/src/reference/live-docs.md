# Live Docs And Continuity

The book is the canonical user-facing documentation surface.
The repo root still contains the live operational and continuity documentation needed to keep work recoverable across long sessions, crashes, and handoffs.

These are separate documentation planes.

- the book explains the product to the outside world
- the root live docs preserve the current engineering state

Both matter, but they are not interchangeable.

## Containment and project-data locality are fully enforced

SpecForge completed the bounded-live-document and repository-volume-data program under
`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`. The purpose is not to reduce durable
information or user documentation. It is to separate bounded current views,
maintained reference prose, generated projections, rolling chronology, and
retrievable history so no mandatory read grows forever.

The project-owned authority is `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`. A complete JSONL registry
classifies every parent-tracked Markdown path exactly once, and the `LIVE-DOC-SIZE` doctrine check
re-derives the resulting-tree metrics on every commit and CI run. The pinned FSMGen submodule is a
separate Git authority and is not swept into SpecForge's registry.

The control plane is bounded too: schema version, allowed fields, record count, registry and record
bytes, array cardinality, and scalar bytes all fail closed. A 55-case fixture suite exercises every
local lifecycle plus index, freshness, currency, routing, transition-baseline, and ceiling-history
path. Another 44 cases exercise the neutral field classifier and the two local authority adapters.
All disposable workspaces live only under repository-local `generated/`.

Because the mdBook is a maintained reference, its current-state contract is executable as well.
`scripts/check_book_current_truth.sh` binds actor-direction and constrained-extraction claims to code,
requires this program's completed status and closing verification section, and rejects their stale
forms; `LIVE-DOC-SIZE` runs that currency check on every commit and CI build.

#### Newer derived-state contract review (`2026-08-09`)

The donor's newer portable revision adds a field-level truth boundary beyond surface size and freshness. An exact
fact about *now* must be derived when read or retained only as a verified copy of a named authority. Authored
intent—such as the selected next action—and revision-bound task evidence are different classes and must not be
forced into an equality oracle.

The `.8a` SpecForge-local probe found that the existing surface plane remains sound: 41 surfaces are governed,
eight currency contracts and 15 non-budget lifecycle verifiers execute, and bounded generated projections are
fresh. It also found zero field declarations and three concrete seams. Old corpus/cache counts in `MEMORY.md`
were misplaced current copies; Rust `1.95.0` is published in README/book/CI without comparison to workspace
`rust-version`; and the current FSMGen hash is checked against a JSON copy but not derived from the mode-160000
Git-index entry.

The owned `.8b` follow-up is now implemented. Fourteen bounded contracts classify one derive-on-read field, two
authored-intent regions, one immutable-evidence region, and ten verified copies. The neutral checker executes
eight existing projection/currentness authorities without project field names. The separate local adapter proves
that workspace `rust-version`—normalized to patch form—agrees with README, mdBook, and CI, and that both retained
FSMGen pin copies agree with the stage-zero mode-160000 Git-index object. `MEMORY.md` exposes
`git rev-parse HEAD` at the reader boundary and forbids a stored latest-commit shadow. Existing generators remain
the authority; no donor value, threshold, or ceiling is copied. `.8c` was assigned to audit the result before the
reopened program could close.

The independent `.8c` audit did not permit closure. All 14 primary declarations and current values pass, but
three secondary copies are still enumerated only in adapter source: the mdBook and CI Rust prerequisites, plus
the feedback JSON gitlink pin. This means executable comparisons are correct while the data-only registry is not
yet a complete copy inventory. `.8d` moves those paths and exact markers into bounded declarations and neutral
validation; `.8e` repeats the audit before the program may close.

`.8d` repairs that inventory boundary without inventing three more field contracts. The two adapter-backed
primary records now carry three bounded `secondary_copies` entries: mdBook Rust is a governed `surface` member;
CI Rust and the feedback JSON pin are repository-local `control` members. Each declaration names a closed role,
safe path, and exact once-only marker. The neutral checker validates role/schema uniqueness, same-volume regular
files, surface membership versus non-Markdown control ownership, and marker cardinality. The adapter consumes
the required roles and contains none of the three path literals; alternate-path fixtures prove it cannot fall
back. The focused plane is now 47 neutral plus 25 adapter cases.

The `.8e` cold audit confirms all 14 primary and three secondary declarations, their once-only markers, and all
72 focused mutations, but it does not close the program. A repository-wide current-value scan finds the live
FSMGen gitlink embedded in the older feedback-protocol self-test renderer. That synthetic root is validated
against the live contract, so the value is update-coupled executable state rather than an independent fixture.
`.8f` derives every synthetic required literal from contract data and adds a changed-pin proof; `.8g` then
repeats the independent source/data audit before `.8` may close.

That `.8f` repair is now implemented without teaching the renderer what either literal means. It expands the
complete `current_root.required_literals` array into the synthetic section; an eleventh protocol self-test
replaces both members with unrelated fixture strings before rendering and validation. The live hash is absent
from executable source, the real protocol output is unchanged, and all 11 protocol plus 72 derived-state cases
pass. `.8g` retains independent closure authority.

The `.8g` audit now closes the derived-state program. It independently confirms 14 primary plus three secondary
members, 17 once-only markers, both Cargo/Rust and Git-index/FSMGen authority groups, and zero live FSMGen values
in executable source. Historical records remain immutable evidence; Rust constants in the adapter suite are
self-contained temporary contracts rather than repository-current shadows. All 83 focused cases and full CI
pass. The next containment frontier is the archive route itself.

Writing that audit result also exercised the status ledger's second post-migration rollover. Twelve exact records
were sealed in segment 0002 (10,894 bytes; SHA-256 `11e058…28df`) and the live root returned to 60 records without
touching its trailer or 40-record migration suffix. The added direct route moves the shared archive index to
81 lines / 5,311 bytes, only 218 bytes before mandatory rollover. The last route entry used 301 bytes, so another
same-shaped append cannot be admitted. `.9a` measured all readers and accepted ADR 0017; `.9b` then landed its
bounded archive-route topology before any third segment. No history, route, threshold, or ceiling was silently
discarded or widened.

#### Bounded archive-route design

The shared file is now a stable 34-line / 1,512-byte landing that names exactly the four ledgers and never lists a
capsule or segment. Each existing ledger archive directory owns a bounded complete `INDEX.md` and `manifest.jsonl`.
The ledger index links its live root, all sealed segments, immutable capsule, and manifest in verified
newest-to-oldest order; the manifest contains only that ledger's records. Reader navigation remains bounded at
landing → ledger index → selected member, and immutable history is not part of ordinary bootstrap reads.

The migration retains the landing and manifest controls. Existing archive surfaces allow no more than 28
segments per ledger; one capsule plus all 28 projects to at most 29 records / 28,420 bytes, below the unchanged
32-record / 32,768-byte manifest limits. The four ledger indexes receive independent per-file and aggregate
bounds rather than borrowing headroom from a wider shared ceiling.

The design audit also corrected the acceptance contract. The checker now rejects missing or duplicate edges,
broken successors, cycles, disconnected or foreign members, and any index whose exact membership/order differs
from the verified chain. The four manifests preserve all nine former data rows byte-for-byte under unchanged
controls; every consumer now uses its ledger authority, and the schema-v2 registry rejects residue at the exact
retired shared-manifest path. Twenty-four focused protocol cases plus the 55-case lifecycle/overflow suite cover
the route, graph, bounds, and cleanup failures.

The `.9a` resulting-tree report also catches pressure in the containment program's own task record: 229,064
bytes, 82.2% of its health target, with 21,611 bytes of pre-rollover headroom. Separate `.10a`/`.10b` leaves own
its lossless current/history topology. The archive-route implementation stayed within that measured margin;
neither concern authorizes trimming history or widening a ceiling.

#### Terminal task-tree boundary

The `.10a` follow-up measures the task source after archive-route completion at 2,451 lines / 232,649 bytes,
with 18,026 bytes of pre-rollover headroom. Across 38 commits it accumulated four roles: program/current
navigation, decisions, slice-specific contracts, and verification/commit/changelog evidence. The last nine
updates averaged 49.7 lines / 5,370 bytes. This is a finite program approaching closure, not a rolling ledger;
mechanical heading shards would split one task authority, while trimming would lose revision-bound evidence.

[ADR 0018](../../../decisions/0018-terminal-task-tree-current-history-boundary.md) therefore uses a two-commit
terminal boundary. `.10b.i` has now committed the complete still-live source contract plus its neutral identity
checker. In `source_locked`, the current task must match its exact SHA-256/metrics/markers and every declared
archive path must be absent. Fifteen same-volume cases exercise both source-locked and future migrated failures;
the checker runs unconditionally through `LIVE-DOC-SIZE`. `.10b.ii` has now copied that durable source byte-for-byte
into a 2,538-line / 242,172-byte immutable capsule at SHA-256 `f8e10e…96b68`, then left a 119-line /
7,978-byte bounded closed summary at the stable task path. A 22-line / 812-byte index and exact manifest route
history in two bounded hops. The task catalog continues reading
only the stable H1 and metadata status; ordinary startup never reads the capsule. The compact root retains the
final verification marker/table required by the derived-state contract, while all historical rows remain exact
in the capsule.

This topology applies only when every leaf is complete and reopening is forbidden. The same census finds the
active `PDF-VARIANT-DIGESTION` task tree at 222,616 bytes, 207 bytes below warning. Its next append requires a
separately owned active-tree partition design; it cannot borrow the terminal rule or a wider ceiling.
`ACTIVE-TASK-EVIDENCE-CONTAINMENT.0` now supplies that ownership and pins the untouched 2,393-line source at
SHA-256 `9284dce4…a19d4`. The opening read also finds that the legacy frontier paragraph and several node-status
headers disagree with completion evidence. `.1.1` accounts for all source bytes in 13 semantic regions and finds
that the nominal frontier is a 1,392-line / 124,201-byte `.9`–`.13` activity store. It classifies every stable-path,
identifier, generic-tool, and writer dependency; no consumer uses a fragment and no executable writes the target.
Because verified completion and pending/blocked claims conflict, [ADR 0019](../../../decisions/0019-bounded-active-task-root-and-semantic-evidence-parts.md)
establishes explicit current-state precedence rather than silently turning stale text into current truth. It keeps
the stable path as a bounded active root over seven semantic legacy parts and exact provenance. The initial root
has no eligible leaf; future work creates a new activity part and atomically updates root + part, with semantic
split-before-rollover and fixed route/aggregate bounds.

The `.2.1` source lock is executable. `.2.2` advances it to complete inputs against boundary commit `f04db37a`,
Git blob `7d89ea4f…b555`, the stage-zero index entry, SHA-256, and exact metrics. All 15 source ranges carry exact
hashes and line/byte/max-line metrics; all 52 IDs derived from path commit subjects have one primary semantic
route; and all seven planned payloads remain below warning. Historical source spelling is deliberately mixed:
six canonical IDs occur only as `.10p`/`.12a`-style shorthand, while four more use shorthand in their primary
payload. Each legacy route therefore binds its canonical ID to a `source_literal` constrained to the full ID or
its exact tree-relative suffix and token-matched in that payload. Thirty-two pre-migration cases reject arbitrary
aliases, missing payload evidence, Git/source drift, premature output, pressure, manifest/frontier errors, and
seal drift.

`.3.1` performs the accepted migration through the contract-driven writer. The stable path is now a 106-line /
5,295-byte active current root; its sole detail route reaches a 79-line / 5,053-byte index, seven bounded semantic
parts totaling 2,473 lines / 225,132 bytes, and all 52 canonical leaf routes. The exact 2,393-line / 222,616-byte
source remains independently retrievable from the archive terminal at SHA-256 `9284dce4…a19d4`; every one of its
15 regions also appears byte-identically between markers in the owning semantic part. The current root reports
`No eligible frontier.` under ADR 0019's conservative precedence rather than reviving stale legacy status text.

The writer validates the complete locked state and reviewed root template before it creates anything. It writes
the capsule, parts, index, manifest, and migrated contract before replacing the stable root, then validates the
whole resulting tree. A failed write or final check restores the source and contract and removes only destination
directories whose prior absence established ownership. The first guarded production attempt exercised that
rollback when Unicode scaffold concatenation upgraded raw UTF-8 legacy bytes and changed their payload identity.
Generated scaffolds are now encoded before raw source slices are appended, and a non-ASCII legacy fixture keeps
the boundary covered. All 34 focused cases pass. Three separate live-document surfaces enforce index, part, and
archive-terminal bounds without widening a threshold or ceiling.

`.3.2` independently clones committed migration `06eb3941` into a repository-local workspace and repeats the
Git object/capsule comparison, marker and route counts, root/manifest/contract reconciliation, live-surface
checks, and doctrine gate from a clean index. It also adds the missing positive continuation case: a new active
semantic part, eligible frontier, `post_migration` route, root-required literal, index, manifest, and contract
update pass together. The 35-case suite therefore proves both rejection and reachability of the future writer
contract, and the containment program is closed.

#### Corpus task migration landed losslessly

The same architecture was measured independently for active `CORPUS-COVERAGE`. Its pre-migration source was
2,308 lines / 277,636 bytes / 4,746 maximum content-line bytes, with only 892 bytes left at the direct
task-evidence ceiling. The guarded root-last writer has replaced that monolith with a 65-line / 2,819-byte current
root and a 75-line / 4,349-byte index. Seven semantic parts total 2,357 lines / 279,157 bytes; every one remains
below warning. They reconstruct all seven source regions byte-for-byte and route all 48 formal ids: 41
completion-subject ids are `legacy` routes and seven formal containers are source-backed `structural` routes.

The exact 2,308-line / 277,636-byte source remains independently retrievable at SHA-256 `5d7acb0…123f` from the
archive capsule. `doctrine/live_document_size/corpus_task_evidence.json` now binds the migrated root, 488-line
manifest, index, parts, capsule, Git/source identity, fixed limits, and migration metadata. Three dedicated
live-document surfaces enforce index, part-collection, and exact archive bounds; all 51 resulting surfaces pass.
The repository-local template was removed after the transaction. Refresh #49 remains a separately owned product
slice. An independent same-SSD no-hardlink clone authenticated `67e726f1`, the source capsule, seven regions,
48 routes, index, manifest, and all doctrines. A temporary eighth active part then made `CORPUS-COVERAGE.2.49`
an eligible `post_migration` route; the focused contract and composed 720-file live-document gate passed without
warnings from the corpus surfaces. The clone returned to the exact committed tree and was removed, so containment
is closed. The first separate post-migration leaf, `CORPUS-COVERAGE.2.49`, refreshes the 45-page Generic Interrupt
Controller Overview Guide with two deterministic guarded CPU ingests. Source structure holds at 430 elements and
the normalized bundle is restored at 139 files / 30,731,394 bytes. The guide's GIC-version/CPU-family table is a
feature matrix, not three timing constraints; current generic authority also removes 83 heuristic interfaces,
26 section phases, 54 prose gates, and the unsupported `controller.isf`. Grounded invariants, contracts, four
intent behaviors, and 88 constraints remain. The separate `.2.50` leaf then refreshed the 57-page / 527-element
OpenCAPI Data Link Layer v2.0 specification, the smallest remaining source. Its first cascade safely blocked the
stale target but exposed child `.2.50a`: four post-binding uppercase tokens became false signal constraints and
one temporal conflict, so both deterministic extractors now bind a passive obligation only to a subject the
document names before its `must/shall be|remain` lead. Two guarded CPU ingests then reproduced 57 pages / 64
visuals / 53 tables / 111 sections / 527 elements and a 184-file / 52,570,034-byte normalized bundle, and two full
cascades reproduced every downstream hash. Current authority removes four stale interfaces, ports, and relations,
54 phases, 71 gates, 38 contracts, and the synthetic `endpoint_dlx.isf`; six semantic actors, 87 invariants, four
assertions, 24 intent behaviors, and 87 constraints remain, with lowering blocking honestly on no declared
interface signals. Corpus coverage is 50/56 with six refreshes remaining.

`CORPUS-CHAIN-CURRENCY` then replaced the assumption that those persisted artifacts still match the code with a
measurement. Its registered `CHAIN-CURRENCY` oracle replays every stage from the persisted artifact one stage
upstream, and its first full run found real standing drift: only 14 of 79 SemanticIRs and 15 of 79 IntentIRs were
what the current binary produces, because each earlier repair had rebuilt only the documents it measured. The
closing leaf rebuilt every downstream chain from its unchanged EvidenceIR — no re-ingest, since only the evidence
stage reads a normalized markdown bundle — and dropped two non-corpus scratch chains. Stages are now
**22/78/78/78 measured-current across 78 documents, and all 44 retained emitted targets pass FSMGen strict
validation**. That count fell from 57 for a measured reason: 14 documents' heuristic interfaces collapsed to zero
under current authority, so their lowering now blocks honestly on having no declared signals. Fifty-six documents
remain unmeasurable at the evidence stage alone until their bundles are backfilled, which the check reports as a
count rather than implying full coverage.

The closing leaf then made that backfill inevitable rather than hoped for. Reclaiming a document's normalized
bundle used to be routine cleanup, which is why only 22 documents could be replayed from the start of the chain
at all. A refresh now keeps its bundle, the retained set is declared in
`doctrine/chain_currency/retained_bundles.json`, and the same oracle compares that declaration with what is on
disk — failing closed on a bundle that disappeared and on a bundle no leaf recorded. The measurable population
therefore grows by exactly one document per refresh, and a reclamation has to name the leaf that authorized it.
Verified by the full gate reporting `22/78/78/78` current stages plus an exact retention match, by sixteen
fail-closed self-test cases, and by three live negatives run against the real declaration.

Post-migration sealing is Git-backed: a part's final content commit lands first, then a following state-only
transaction records that ancestor commit/blob before any later part accepts work. The migrated legacy payloads
need no such self-reference because exact source-region markers already make their bytes independently immutable.

All four root-ledger migrations have landed. `CHANGES.md`, `DEVELOPMENT_NOTES.md`,
`LIVE_ACHIEVEMENT_STATUS.md`, and `RUST_CODEBASE_ANALYSIS.md` are bounded current views, while their
exact source capsules are the historical authority.
The same clean-clone audit finds root-ledger pressure. The owned follow-up pins clean boundary `4d24b13c` and
measures changes at 114 records / 1,645 lines / 222,725 bytes, development at 84 / 1,697 / 208,089, status at
69 / 111 / 92,847, and Rust at 62 / 1,187 / 100,701. Changes is at 91.4% of its line health target;
development, status, and Rust are at warning.

That census also catches an enforcement split: the generic surface contract correctly applies warning and
rollover milestones to reviewed health targets, while the focused rolling-ledger checker applied them to the
larger quarantine ceilings. The focused checker now binds those authorities, and the guarded transaction has
materialized exact whole-record cuts of 29 changes, 24 development, 12 status, and eight Rust records. It
authenticates the committed boundary and selected range, renders into a repository-derived same-volume stage,
installs segment then manifest/index then root, and restores exact prior bytes after any failure. Injected-failure
tests prove rollback and residue cleanup. The resulting roots are changes 87 records / 1,246 lines / 188,183
bytes, development 62 / 1,294 / 175,215, status 59 / 101 / 82,836, and Rust 55 / 1,064 / 89,706: every health
dimension is below warning, and no threshold or ceiling is widened.

The closure audit reproduces commit `10d182ff` from a no-hardlink clone on the repository volume. The clone's
only initial doctrine failure is the standard uninitialized `subs/fsmgen` gitlink; populating exact commit
`d327129b` from the existing same-SSD checkout makes all six doctrines pass. A temporary valid record through
each of the four grammars proves the next concise append remains reachable at 88/63/60/56 records and
1,250/1,298/102/1,068 lines, still below warning. After exact fixture restoration, clean Git/object checks, zero
transaction residue, and workspace deletion, the real closure records leave the current roots at changes 88
records / 1,259 lines / 189,197 bytes, development 63 / 1,306 / 176,145, status 60 / 102 / 83,464, and Rust 55 /
1,064 / 89,706. The pressure tree is closed.
No historical content is removed before its identity, replacement route, consumers, and retrieval
procedure are proved.
The top-level README remains a first-class landing page; changing detail and
chronology will route to controlled canonical destinations.

### Lossless rolling-ledger protocol

The four large root ledgers have an executable migration contract. `.4a` moved no record; `.4b` then
used that contract for the first atomic transition. Their record boundaries are not interchangeable:

| Live root | Whole-record boundary | Stable content outside records |
| --- | --- | --- |
| `CHANGES.md` | modern H3 records, then legacy H2 records, plus two named detached compatibility records | none |
| `DEVELOPMENT_NOTES.md` | every H2 after the H1 title | H1 prologue |
| `LIVE_ACHIEVEMENT_STATUS.md` | each top-level bullet in `Current snapshot` | H1/current heading plus the gap and generated validation trailer |
| `RUST_CODEBASE_ANALYSIS.md` | every H2 after `Purpose` | H1 and `Purpose` prologue |

`doctrine/live_document_size/rolling_ledgers.jsonl` pins the measured source identity, reviewed live
window, local record/line/byte/width limits, consumers, and future archive route for each root.
`scripts/check_rolling_ledger_protocol.pl` parses and reconstructs every byte, then derives the
planned live view from whole records and checks each pressure axis independently. Its grammar tests
also reject non-bullet content inside the status record region.

A pinned plan is first proved without writes and then applied explicitly:

```sh
perl scripts/check_rolling_ledger_protocol.pl --rollover-plan docs/research/<plan>.jsonl
perl scripts/check_rolling_ledger_protocol.pl --rollover-plan docs/research/<plan>.jsonl --apply-rollover
```

The plan closes over the committed root blob, contiguous record range, segment digest and endpoints, current
newest successor, and warning-safe survivor. The checker rejects boundary drift, segment reuse, a stale
successor, surface-authority disagreement, or any survivor at warning. Thirty-five focused cases include a
failure injected after segment installation and prove byte-exact rollback with no transaction residue.

Each initial migration copied the exact pre-migration file into an immutable, repository-local source
capsule before shortening the stable root. The capsule manifest records its digest and dimensions; a
bounded index links both the current root and historical capsule; the checker retrieves and revalidates
both. Each capsule deliberately overlaps the retained current window so complete-source identity
remains independently reproducible. Future rotation seals only newly aged-out whole records, not
another full copy.

This matters especially for `LIVE_ACHIEVEMENT_STATUS.md`: project validation replaces a managed block
at its existing root path. The protocol therefore requires both validation projection markers and
keeps the entire generated trailer outside the rolled bullet region.

The two containment records initially written after the old 2026-03-31 `CHANGES.md` tail are not
silently reordered in historical evidence. The bounded live view promotes those exact records
after its newest prefix, while the source capsule retains their measured original ordinals and bytes.

#### `CHANGES.md` migration landed

At the `.4b` migration boundary, the root change ledger became 1,368 lines / 199,851 bytes instead of
32,682 lines / 2,629,033 bytes.
Its initial capsule at `docs/archive/rolling-ledgers/changes/source-through-2026-08-08.md` retains all
1,798 pre-migration records byte-for-byte under SHA-256
`d89809322857aab3c506dde1cc6caaf57e22d0349655b37ddaab1f7bf22ba994`. The live root holds the newest
85 capsule records, then the exact `.1`/`.0` compatibility records, with `.4b` as the first
post-capsule prepend. `docs/archive/rolling-ledgers/INDEX.md` is the bounded current/history route.

The main surface registry now classifies the root as a normal rolling ledger and the capsule as an
archive terminal. The generic checker skips archive pressure warnings—the capsule is immutable and
outside mandatory reads—but still enforces its exact ceiling; the rolling-ledger checker independently
reopens the file, verifies every metric and digest, checks the manifest/index, and proves the retained
root suffix. This is the retrieval contract, not a reliance on Git history alone.

The next rollover seals 29 more whole records at
`docs/archive/rolling-ledgers/changes/segment-0002-2026-08-09.md` (428 lines / 37,189 bytes / SHA-256
`bc87665975d80078697384e89b2127e67164ae0e523a8076b93f81eb5582fbd8`). The current root contains the two
task records prepended after the pinned boundary, the ten newest opening records, and the exact 75-record
migration suffix: 87 records / 1,246 lines / 188,183 bytes.

#### `DEVELOPMENT_NOTES.md` migration landed

At the `.4c` migration boundary, the engineering-rationale root became 1,480 lines / 194,412 bytes
instead of 20,921 lines / 2,170,230 bytes. Its initial capsule at
`docs/archive/rolling-ledgers/development-notes/source-through-2026-08-08.md` retains all 1,601
pre-migration records under SHA-256
`76b51a3f450cdb1e764922dc366cf6ff55529cb95f7a6f410bfba1f1f378fedc`. The migrated root began with its H1
prologue, 60 capsule records, and `.4c` as the first post-capsule prepend.

The first post-migration rollover landed when a later rationale prepend reached the mandatory 90% record
threshold. It seals the 27 oldest post-capsule records content-identically and in order at
`docs/archive/rolling-ledgers/development-notes/segment-0001-2026-08-08.md` (467 lines / 36,125 bytes / SHA-256
`b07a73670863e283f359911629db691859f41d99021a1902db3adfeb83788b91`). That bounded root carried the H1
prologue, 10 newest post-capsule records, and the exact 50-record retained migration suffix. The manifest places
the sealed segment between that root and the immutable source capsule.

The next rollover seals 24 more whole records at
`docs/archive/rolling-ledgers/development-notes/segment-0002-2026-08-09.md` (421 lines / 34,258 bytes / SHA-256
`0576c44b7b95c40bc4669128ce07ebdc6a99b1ebbe9322cd1aca3acad8c04cbd`). The current root contains the H1
prologue, two later task records, ten newest opening records, and the exact retained 50-record suffix: 62 records /
1,294 lines / 175,215 bytes.

The retained-record check is byte-sensitive beyond visible prose. During `.4c`, an ordinary edit at
the top of the derived root exposed an ownership ambiguity for the blank line between records 60 and
61. The complete-source capsule keeps that byte exactly, but a bounded view ending at record 60 must
not turn the removed successor's separator into a blank-at-EOF artifact. ADR 0008 and the renderer now
define one such terminal separator as boundary structure: it is omitted only at live EOF, while every
record-content byte must match. “Looks the same” remains insufficient; the exception is exact and
mechanical rather than a whitespace normalization.

#### `LIVE_ACHIEVEMENT_STATUS.md` migration landed

At the `.4d` migration boundary, the current-status root became 90 lines / 88,414 bytes instead of
1,960 lines / 581,239 bytes. Its initial capsule at
`docs/archive/rolling-ledgers/live-achievement-status/source-through-2026-08-08.md` retains all 1,920
pre-migration records under SHA-256
`b00ff5f5c4a29554a20eea9d901a95848d54a644749eba74ad9618798dd9bd6a`. The live root keeps its
H1/current heading and newest 50 capsule bullets; `.4d` is the first post-capsule prepend.

Status rotation does not own the material below `## Highest-priority remaining gap`. The complete gap
section, `## Validation Projection`, and both managed markers remain in the root outside the bullet
record region. The repository's focused project-validation test runs the real Rust writer with
temporary storage forced under `generated/tmp`, proving the managed block can still be replaced at the
stable path after migration. Capsule identity, suffix identity, marker literals, pressure, manifest,
and index routing are all rechecked by the unconditional doctrine gate.

The second rollover seals the next 12 aged-out records at
`docs/archive/rolling-ledgers/live-achievement-status/segment-0002-2026-08-08.md`; manifest predecessor/successor
links order it between the live root and segment 0001. Direct comparison with the pre-removal Git content proves
the segment's exact 10,894-byte identity, while the root again contains 20 newer records plus the exact retained
40-record suffix.

The third rollover seals the next 12 aged-out records at
`docs/archive/rolling-ledgers/live-achievement-status/segment-0003-2026-08-09.md` (12 records / 9,181 bytes /
SHA-256 `9bb1607d89fcc67ea2c1824a9c84e21bd95d56f7f0f49a1af43fee9d3b865edb`). The root had reached 72 records;
exact pre-removal reconstruction proves the segment, then the bounded root returns to 20 newer records plus the
reviewed 40-record suffix. The manifest chain is root → segment 0003 → segment 0002 → segment 0001 → capsule.

The fourth rollover seals the next 12 records at
`docs/archive/rolling-ledgers/live-achievement-status/segment-0004-2026-08-09.md` (12 lines / 11,420 bytes /
SHA-256 `6d1c07596313d0c98c04afde63dd23d7df61fda291bd50d2cbc57a0568b924eb`). The current root contains 19 newer
records plus the exact retained 40-record suffix and complete trailer: 59 records / 101 lines / 82,836 bytes.

#### `RUST_CODEBASE_ANALYSIS.md` migration landed

The Rust architecture root is now 1,064 lines / 89,706 bytes instead of 9,039 lines / 1,046,679
bytes. Its initial capsule at
`docs/archive/rolling-ledgers/rust-codebase-analysis/source-through-2026-08-08.md` retains all 1,350
pre-migration records under SHA-256
`95e1665628b615498e94f67d6dc6e0083d4d104a6ca8a815b4c23cf2f97cc7ce`. The live root keeps the H1,
complete Purpose prologue, and newest 55 whole H2 records.

No `.4e` record was added to that root because the initial migration changed documentation lifecycle rather than
Rust architecture. The generic materializer is different: it adds a reusable committed-boundary, same-volume,
root-last, exact-rollback subsystem to the checker, so `.1` has a proper architecture record. Its first later
segment is `docs/archive/rolling-ledgers/rust-codebase-analysis/segment-0002-2026-08-09.md` (eight records / 134
lines / 12,009 bytes / SHA-256 `2444822e5f98f04516a20a9a54468e57f99721afcf869c37a12da0bc538bacb6`).

### Lifecycle controls

Every surface declares one of seven lifecycles:

- a `bounded_snapshot` is overwritten or reviewed in place under independent line, byte, and
  line-width ceilings;
- a `rolling_ledger` must roll whole records into indexed durable history before its live window
  becomes unbounded;
- a `partitioned_canonical` collection keeps bounded parts, count/aggregate controls, and an index or
  deterministic query contract;
- a `generated_projection` has both size ceilings and a freshness verifier against canonical inputs;
- a `frozen_legacy` record is hash-locked, while an `archive_terminal` is governed by an immutable
  manifest and is never an author-overflow destination;
- a `maintained_reference`, such as this mdBook, keeps bounded directly indexed semantic parts while
  exact task-owned aggregate-change authority permits legitimate product-scope evolution.

### Collection and projection closure order

The post-ledger census found that collection pressure was not one problem. The decision index and
mdBook membership/current-truth controls were already complete and remained in place. The remaining
surfaces were closed in this dependency order:

1. rebuild the task catalog as a concise index of every real task tree;
2. close direct fact, decision, research, and remaining canonical-collection navigation;
3. shard the generated Knowledge Map without losing question-key retrieval or derive-and-diff;
4. separate current roadmap direction and FSMGen requests from their historical chronology;
5. replace validation, source-registry, and corpus-KB currency labels with non-mutating producer oracles;
6. audit root maintained references against this book, then close aggregate membership/currentness.

At entry, `docs/TASK_TREE.md` links only 117 of 122 tracked task files. Four real trees are missing;
the fifth difference is the author template and must not appear as active work. Its widest row is
18,932 bytes because it mirrors execution history instead of acting as an index. No collection content
moves in the planning slice; each migration has its own source/retrieval proof.

#### Task catalog migration landed

`docs/TASK_TREE.md` now derives one concise row for all 121 real task trees, with `TEMPLATE.md` linked
separately for authors. The workflow contract remains outside the generated section. The index moved
from 167,583 bytes and an 18,932-byte widest row to 27,228 bytes and a 207-byte widest row while adding
the four missing real routes.

The catalog intentionally omits execution narrative. Open the selected tree for its frontier,
decisions, evidence, and commits; use `MEMORY.md` for the single resume pointer. The generator/checker
validates filename/H1 identity, status vocabulary, ordering, escaping, membership, and independent
title/row/section/count bounds. Its derive-and-diff and seven self-tests run on every live-document
doctrine gate.

#### Decision and fact-card navigation landed

The decision collection keeps its bounded index: every current ADR file is linked exactly once, and
the generic membership gate continues to enforce that route. Fact cards have a different,
complementary route at `docs/knowledge/INDEX.md`. At `.5c.ii`, its 153-line / 28,899-byte generated
catalog listed all 139 immediate cards by id, establishment date, status, and a bounded title preview.
Long historical titles are shortened only in the index; the linked card remains canonical and
unchanged.

The catalog count deliberately excludes `docs/knowledge/README.md` and the generated index itself.
The root question map then reported 140 facts because it additionally scans ADR 0007, which has
Knowledge Map front matter. Thus directory files, fact cards, and cross-layer generated facts are
three explicit counts rather than one overloaded metric.

The focused generator/checker requires filename/id identity, constrained front matter, questions,
date, valid status (with the architecture-defined `current` default), evidence or reverify, stable
ordering, and independent field/row/count/index bounds. Its current two-state checker runs 41 positive/fail-
closed cases plus the real derive-and-diff check through the live-document doctrine. The root README now offers both routes:
use the bounded catalog to browse known ids/titles, and the generated Knowledge Map to search by a
question. The bounded question-shard migration is described below.

The later `47e91540` boundary exposed a new pressure asymmetry before data loss or a failed commit. The
collection now has 158 cards plus its README and index—160 files, exactly the generic surface's 80% file
warning—but the monolithic 32,634-byte browse index has only 134 bytes below its separate 32,768-byte limit.
Its focused 160-card cap leaves two cards even though the generic surface appears to leave 40 files and the
question projection permits 200 facts. Recording the finding added three retrieval keys and moved that already-
sharded projection to 209,962 aggregate bytes, or 80.1% of its health target, still below rollover.

ADR 0020 keeps the stable browse path as a compact direct-ID landing and moves the unchanged detailed rows into
deterministic count-packed title parts under `docs/knowledge-catalog/`. The 198-card maximum derives from the unchanged
200-file collection minus its README and index. At most four title parts may exist; combined landing and parts
are capped at five files / 512 lines / 122,880 bytes. ID retrieval remains one hop; title browsing remains two
bounded hops. `.2.1` first locked the monolith and rejected every destination; `.2.2` then migrated the output.
Canonical facts and question semantics stay separate; neither deleting evidence nor widening a surface limit is
an accepted repair.

ADR 0021 corrects one route-level assumption before code. A legacy `(card-id.md)` link copied byte-for-byte into
the sibling title-part directory would resolve to the wrong place. The monolith therefore remains exact byte
provenance, while each migrated row preserves its ID/date/status/title and resolved canonical destination and
rewrites only the relative target to `../knowledge/card-id.md`.

ADR 0022 records the executable pressure correction: the first 64-card template was 73/80 health lines, already
past mandatory rollover, and even its minimal form was inside warning. Packing 56 cards with a seven-line
scaffold produces 63-line full parts while still fitting 198 cards in the accepted four-part maximum. ADR 0023
then applies pressure to the root at that maximum: the verbose scaffold would produce 218/224 health lines, so
the landing kept only H1, generated/navigation, and title-route scaffold lines. It was 201/224 lines at 198
cards, below mandatory rollover without a limit change. At that boundary the migrated output was a 161-line /
11,984-byte landing plus three parts totaling 179 lines / 34,906 bytes; the complete 340-line / 46,890-byte
projection had no warning. The schema-closed contract retains the committed legacy Git blob,
SHA-256, raw metrics, 158 source cards, and ordered-row provenance while its current `migrated` state enforces
exact output membership, content, routes, residue, and pressure. It required the part directory and
`fact_card_titles` surface absent in legacy state; it now requires that surface's exact generated-projection
lifecycle, inputs, limits, landing, and freshness route.

The closing migration followed the pre-enforced write transaction: register the exact surface and switch state,
render and verify in a repository-local workspace, write all parts, remove only stale generated-prefix files, and
write the stable landing last. The first real check also exposed an ASCII-only fixture gap—decoded non-ASCII row
text was being passed directly to a byte digest. Row unions are now explicitly encoded as UTF-8, and an em-dash
fixture locks the boundary. A separate exact-capacity fixture keeps 198 below rollover and rejects card 199.
Forty-one focused cases, direct link resolution, both generic surface memberships, full
doctrine enforcement, and the mdBook build verify the delivered method. Canonical facts remain the only fact
authority; the generated projection contains no unique evidence and therefore needs no archive copy.

#### The fact-card landing stopped scaling with cards

That landing listed one bare ID line per card, so it grew as `lines = cards + 3` and its 224-line health target
was the tightest of the four authorities bounding the fact plane. Because the checker treats 90% of a health
target as a mandatory-rollover error, 198 cards (201 lines, 89.7%) was not a chosen maximum but the exact edge
the shape permitted, and no bound could be raised while the landing stayed O(cards). ADR 0026 and ADR 0027
record that measurement and its correction.

`FACT-CARD-CAPACITY-HEADROOM.2` therefore reshaped the landing into a fixed-size router. It keeps its H1 and
generated/navigation lines, states how many cards route through how many parts, and then carries one table row
per title part: the part link, its card count, and its inclusive first and last id. The unchanged title parts
still carry every card's id, date, status, and title, so any id is reached in one deterministic extra hop —
find the range that contains it, open that part. At 193 cards the landing fell from 196 lines / 15,417 bytes to
**10 lines / 878 bytes**, and its size now follows the part count: seven lines at one part, ten at the four-part
maximum. The four title parts were byte-identical through the change.

The proof moved with the shape. The landing is rejected if it links a card directly instead of routing it, and a
re-read of the rendered table must reproduce the canonical card list exactly: one ordered row per part, counts
summing to the catalog, and boundary ids the card list confirms. Every card must resolve exactly once across the
parts. The canonical card surface declares `routed_membership` through the `fact_card_titles` record, so the
generic membership gate proves the same completeness independently. Inline assertions and focused cases pin the
fixed-size law, the card-free landing, and three range regressions, alongside the exact capacity and
fail-closed boundaries. That leaf changed no card content and moved no limit; re-deriving the whole profile
against the new shape is the next subsection.

#### Fact-plane capacity became one derived profile

With the landing fixed-size, `FACT-CARD-CAPACITY-HEADROOM.3` re-derived every remaining authority in one
transaction. Measuring first exposed a plainer problem than a tight limit: **the declared capacity was not
reachable**. `max_cards` was 198, but 198 cards at the measured mean of 50.5 lines each need 10,026 aggregate
lines against a 10,000-line ceiling, so the plane would have refused its own advertised last card — in
whichever unrelated slice happened to write it. `decision_records` had the same shape (a 4,000-line total
against 32 × 512 legal per-file lines), and the question projection was already **over** its aggregate byte
health target at 267,938 of 262,144.

None of those totals had a legal exit. Cards and decision records are canonical: ADR 0026 forbids deleting or
merging a card to buy capacity, and neither collection rolls over into an archive. A total that
individually-legal files can exceed is therefore a state ordinary compliant writing reaches and compliant
work cannot leave. ADR 0029 fixes the class, not the instance: **pressure belongs on a dimension that has a
remedy.** For a never-deleted collection that dimension is the count, whose remedy is to add capacity, so the
aggregate line and byte bounds become the file bound times the per-file bound. The per-file bounds keep their
warning bands and stay the quality signal; the totals only guarantee that a legal corpus is never refused.
The title parts already satisfied that rule for lines (320 = 4 × 80), which is where it was read off.

Capacity itself is set from measurement against the doctrine's own milestones: a bound is chosen so the
measured population sits below the 80% warning and one measured peak active-day of growth still sits below
the 90% mandatory rollover, then rounded up to the surface's natural quantum. Growth was taken from Git —
cards per active day median 8, 90th percentile 20, peak 25 over 20 active days; decision records peak 9.
The binding dimension turned out to be the part-file count: four rendered parts must stay under 80% of the
permitted parts, which needs six.

Everything else follows from `max_parts`, the only free parameter:

| Quantity | Derivation | Value |
| --- | --- | ---: |
| `cards_per_part` | ADR 0022 — 56 cards + 7 scaffold lines = 63 of an 80-line health target | 56 |
| `max_parts` | four rendered parts below 80%, five below 90% | 6 |
| `max_cards` | `cards_per_part × max_parts` | 336 |
| `knowledge_cards.files` | `max_cards + 2` (README, landing) | 338 |
| Card aggregates | `338 × 300` lines, `338 × 36,864` bytes | 101,400 / 12,460,032 |
| `decision_records.files` | 30 measured, peak 9 → `(30 + 9)/0.90` | 44 |
| `max_facts` | `max_cards` + every record but the index | 379 |
| `max_question_keys` | `max_facts ×` the 8-keys-per-fact ratio the Knowledge Map bundle's hard caps declare | 3,072 |
| Projection aggregates | landing bound + `max_shards ×` shard bound | 12,384 / 1,581,056 |

The derivation is pinned as a derivation. The catalog self-test asserts the identities — capacity is the part
quantum times the part count, each aggregate band equals files times that band's per-file bound, the
projection ceiling equals the landing ceiling plus the part aggregate — so a raise cannot move one literal and
strand another. `max_facts` is no longer a pinned literal at all: the checker derives it from `max_cards` plus
the decision-record file ceiling and rejects any other value, with a focused case for each writer drifting
alone. A full 336-card render must cross no mandatory pressure and a 337th must fail closed, replacing the old
198/199 pair; 58 focused cases pass.

Every fact-plane rollover warning is gone: 195 of 338 collection files (57.7%), 30 of 44 decision records
(68.2%), 2,976 of 12,384 projection lines (24.0%), and 199 of 379 facts. No card and no decision record was
edited. One warning survives deliberately — `knowledge_cards` `lines_each` at 81.0%, because one 243-line card
sits against a 300-line per-card bound. That is a per-card quality signal with a local remedy, not capacity
pressure. The portable bundle's own hard caps (512 facts, 4,096 question keys, 64 shards) are now the visible
architectural ceiling: this profile uses 74% of the fact cap, and growing past roughly 470 cards would have to
raise the bundle's caps first.

#### Remaining canonical collection catalogs landed

At `.5c.iii`, no partitioned canonical Markdown collection relied only on a Git query. The bounded
`docs/catalogs/` plane then contained six generated direct indexes for 241 workflow-standard, root-
architecture, FSMGen-issue, research, corpus-KB, and KG-fixture members. Its 13-line landing index
linked every generated catalog; the whole seven-file plane was 314 lines / 49,305 bytes, and its
largest part was the 167-line / 34,615-byte KG-fixture catalog.

The later root-reference audit proved that the four root architecture/user files are compatibility
pointers, not a partitioned canonical collection. Their generated catalog was therefore retired;
the remaining five catalogs directly cover the five still-applicable external collections. The root
pointers themselves now link their maintained book homes and have independent bounded-snapshot
controls.

Two collections keep more natural existing routes. `docs/TASK_TREE.md` directly covers all 122 task
files—121 real trees plus the separately linked author template—and the portable Knowledge Map bundle
uses its own README to link all four bundle documents. In total, the eight former query-only surfaces
now have direct membership proof over 367 canonical files without moving or copying their prose.

The doctrine now distinguishes internal membership from `external_membership`. An external index must
be one safe repository-relative Markdown file outside the member surface, must itself belong to a
separately classified bounded surface, and must link every member directly. This lets a large fixture
catalog remain outside fixture inputs and lets the task collection preserve one canonical catalog.
Fifty-three lifecycle/control-plane fixtures cover the positive path plus missing, stale, off-root,
and improperly internal external-index failures. The common catalog generator adds independent
path/title/row/member/index bounds and derive-and-diff; no generated catalog contains canonical facts.

Locator honesty became mechanical at the same time. A `file` locator must match exactly one path; a target
matching several is a collection and must say so. That rule closed a real blind spot: the aggregate
`lines_total` and `bytes_total` milestones are suppressed for a one-file surface, where they merely repeat the
per-file measure, and the suppression keyed off the *declared* locator. Four surfaces declared `file` over a
multi-file glob, so their aggregate ceilings kept failing closed while their milestones stayed silent —
`knowledge_cards` sat at 97.7% of a hard 10,000-line aggregate with no warning ever emitted, and before the
landing was reshaped it was at 99.6%, under one average card from a stop nobody could see coming. The exemption
is now decided from the measured file count, the four surfaces are reclassified, and both directions are pinned
by fixtures. ADR 0028 records the finding and its root cause.

A third kind, `routed_membership`, exists for a front door whose direct member list would itself grow
with the collection. A routed index lives inside its surface like an internal membership index, but it
additionally names one declared `route_surface`; it must link every file of that companion surface,
and completeness is then proven from the union of its own links and the companion files' links. The
hop count is fixed at one: the companion may not itself be routed, and `route_surface` is rejected on
any other index kind. Size then follows the companion's file count rather than the member count. The
fixture suite runs 67 cases; eight of them cover the routed positive path plus an unreachable member,
an unrouted companion file, a missing, unregistered, self-referencing, or chained `route_surface`, an
index placed outside its own surface, and `route_surface` smuggled onto a direct membership contract.
Three more pin locator honesty: a `file` locator over several paths is rejected, a collection near its
aggregate line target warns, and a genuinely single-file surface stays exempt from the duplicate warning.

#### Knowledge Map projection migration landed

The committed entry baseline was one 2,231-line / 1,038,010-byte generated file
with 138 facts and 978 question rows. An audit found only 977 unique questions: the same register-bit-
field question appeared in both a broad completeness scorecard and its focused lowering-gap card. The
focused card now owns that key, and the contract rejects any future question with two destinations.

The migrated read path keeps `KNOWLEDGE_MAP.md` stable as a small landing page. It links every
deterministically packed `docs/knowledge-map/questions-NNNN.md` shard and the separate fact-card
catalog. Search all questions with:

```bash
rg -i --glob 'questions-*.md' 'your terms' docs/knowledge-map
```

Each preserved question links directly to one canonical fact card. Question entries sort by UTF-8
bytes, wrap under a fixed physical-line limit, and omit repeated date/reverify fields; those fields
remain in the linked card. The repeated generated fact section also disappears because the bounded
fact catalog already owns id/title browsing.

The migration's 139 facts / 981 unique questions occupied seven shards.
The landing is 19 lines / 898 bytes; the largest shard is 300 lines / 30,427 bytes, and the complete
question set is 2,000 lines / 183,756 bytes. Independent limits govern input ids, paths, question
bytes, line width, files, lines, and bytes; explicit 4,096-line / 393,216-byte aggregate caps prevent
unused per-shard capacity from becoming banked growth. A SHA-256 identity covers participating paths
and content. The hook stages every current output and tracked deletion, while the checker regenerates
under repository-local `generated/` and compares exact root/shard membership plus content. Eight
contract cases, eight portable-bundle integration cases, and the common lifecycle suite prove parsing,
collision rejection, ordering, wrapping, rollover, stale cleanup/staging, bounds, drift, and no
temporary residue. A ninth contract case guards raw-byte identity after the initial simulator was
found hashing scalars consumed by checked UTF-8 decoding. Every bootstrap, CI, bundle, doctrine, and mdBook reader moved in the same slice;
the generated surface is now normal rather than transition debt.

At `.5i` closure, the bounded catalog contains 145 fact cards and the generated map contains 146 facts
with 1,017 unique question keys across seven shards. Both counts and every route are derive-and-diff
checked, so later cards can grow the bounded projections without reviving the historical counts above
as current truth.

#### Bounded roadmap with exact historical recovery

The roadmap is a different lifecycle problem from a newest-first change ledger. Its 23 H3 workstream
records mix current goals and remaining scope with delivery chronology inside the same headings. At
the pinned boundary, `ROADMAP.md` is 1,487 lines / 183,445 bytes; `done:` blocks alone account for 722
lines / 129,242 bytes. Splitting on headings would therefore keep both roles mixed, while deleting
the completion blocks in place would make the source impossible to reproduce.

ADR 0010 implements a two-product lifecycle:

- the stable root remains a bounded, human-authored current-direction snapshot;
- an immutable repository-relative capsule preserves every pre-migration byte;
- a bounded archive index and manifest make that capsule directly retrievable and hash-verifiable;
- the task catalog remains the exact execution/status ledger instead of being copied into the root.

The 153-line / 12,033-byte root keeps the objective, canonical pipeline, implementation doctrine, current strategic
priorities, one concise row for each R0–R16/R15b–g workstream, forward order, and direct history/
execution routes. It rejects `done:` blocks and dated progress chronology. For example, closing one
task-tree leaf updates that tree and `CHANGES.md`; it changes `ROADMAP.md` only when the high-level
workstream status, priority, or program direction changes.

The exact capsule retains all 1,487 lines / 183,445 bytes at SHA-256
`20a71e88c15133398879fb11620dbb4c3ed20a6b689f2bcea733ef82b2a47d88`; its bounded
[archive index](../../../archive/roadmap/INDEX.md) and manifest provide direct retrieval and verification.
The migrated gate pins that identity, five exhaustive source regions, all 23
workstream ids and owning task routes, known readers/writer, four stale-current findings, archive
topology, and independent root limits. The live-document doctrine verifies capsule identity plus
bounded-root/index/manifest retrieval on every run.

##### The rollover that makes the bound survivable

A one-time migration bounds a document once. It does not say what to do the next time the bounded root
fills — and this one filled again three days later, at 364 of 384 ceiling lines. Root-causing that growth
found a single shape: 213 of the 364 lines were one bullet under `Current strategic priorities` that had
grown by one sentence per closed task-tree leaf, which is exactly the delivery chronology the same file
disclaims nine lines further down. The contract's `forbidden_literals` guard three legacy chronology
shapes and matched none of it, because this chronology was written as ordinary prose.

[ADR 0030](../../../decisions/0030-a-bounded-snapshot-needs-a-declared-repeatable-rollover.md) turns the
one-time migration into a repeatable operation. A rollover copies the bounded root to a dated capsule
under `docs/archive/roadmap/root-through-<date>.md`, appends one record to the contract, adds one archive
index row, and rewrites the root. The contract record carries the capsule's SHA-256, lines, bytes, maximum
line bytes, seal date, and reason, and the gate re-reads every capsule on every run — so recovering retired
direction never depends on Git object reachability.

Three properties keep the new series from becoming the problem it solves:

- **A capsule must have been legal when it was sealed.** Every capsule is checked against the *current
  root's* enforcement ceilings, so a capsule above them is a gate failure rather than a rescue. The gate's
  silence is the evidence that the surface never actually overflowed.
- **The series is bounded, reported, and remedied.** `rollover_policy` declares 16 capsules maximum,
  warns at 12, and names its own remedy in the same record. This is deliberate: `archive_terminal`
  surfaces are exempt from the live-document milestone report, because immutable history has no per-file
  remedy — so a growing terminal collection would otherwise stay silent until its hard ceiling.
- **The bounds are derived, not chosen.** Per ADR 0029, the collection's aggregates are the file bound
  times the per-file bound (16 × 384 lines, 16 × 49,152 bytes), and the per-file bound is the current
  root's ceiling, because that is the largest thing a rollover can retire.

The first rollover sealed the exact 364-line / 34,938-byte root at SHA-256
`2cc2fa4a39ff7a654ec0c24ec59a715d016bb0625a4846a9e583afc94fb77190` and left a 156-line / 12,394-byte root
— 40.6% of its ceiling — without moving a single health target, ceiling, or milestone. The contract's
focused cases grew from nine to 31: the original nine still fail on duplicate, reordered, or missing source
workstreams, missing or duplicate current rows, forbidden chronology, and unsafe paths, and 22 new ones
cover rollover schema and identity — duplicate ids and capsules, reuse of the pre-migration capsule,
non-chronological seal dates, unsafe and non-Markdown paths, over-cap counts, a capsule above the
current-root ceiling, a rollover declared in `planned` state, and every capsule metric or digest mismatch.

##### Why the file bound was the wrong detector

The rollover is an exit, not a lock, so
[ADR 0031](../../../decisions/0031-a-bounded-snapshot-bounds-its-sections-not-just-its-file.md) closes the
entrance. A file-level bound detects this failure badly: 213 lines grew inside one of seven sections, and
measured against the file that reads as "`ROADMAP.md` is at 142.2% of health" — a number about the whole
document that names no cause and points at no action. Measured against the section, the same growth reads
as "`Current strategic priorities` is at 82% of its 56-line bound; route per-leaf detail to its owning task
tree."

Each H2 of the bounded root therefore declares its own `max_lines` and its own remedy, and the gate proves
three things about the set: the declared sections are exactly the required H2 order, no section exceeds its
bound, and `Σ max_lines + section_count` fits inside the file's health target. That last rule is ADR 0029's
reachability requirement applied inward — a fully legal set of sections must be a healthy document, so the
file ceiling stays a quarantine backstop rather than the operative control. The present bounds sum to 243 of
256.

Bounds follow measured shape and change frequency rather than an even split:

| Section | Lines | Bound | Pressure | Remedy on breach |
| --- | ---: | ---: | ---: | --- |
| Objective | 7 | 14 | 50.0% | it belongs in the book introduction |
| Canonical pipeline | 2 | 6 | 33.3% | it belongs in the pipeline overview |
| Cross-cutting implementation doctrine | 57 | 80 | 71.2% | a rule needing a paragraph is a decision record |
| Current strategic priorities | 34 | 56 | 60.7% | route leaf detail to its task tree, or roll the root |
| Workstream status | 29 | 44 | 65.9% | the workstream set itself needs restructuring |
| Recommended implementation order | 11 | 20 | 55.0% | an order that long is a plan; move it to its tree |
| History and execution | 9 | 16 | 56.2% | a new route replaces an old one |

Replaying the real accretion is what proves the control: the gate warns at 46 of 56 section lines and fails
at 58, with the whole file at 180 of its 384 lines — less than half its ceiling and still inside its health
target. The file-level bound would have stayed silent for roughly another 200 lines. Whether other bounded
snapshots need the same inner bound is a separate measurement, not an assumption.

#### An aggregate a collection can exceed is not a bound

The roadmap was one instance of a class. A collection declares a file bound, a per-file bound, and an
aggregate; if the aggregate is smaller than `files × per-file`, then a corpus whose every file is legal is
refused by a total no single file can see — a state ordinary compliant writing reaches and no compliant
action leaves.
[ADR 0032](../../../decisions/0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
makes that shape impossible to declare.

Enumerating all 20 multi-file surfaces found eleven tight. The clearest was `task_evidence`, the task-tree
collection: its 40,000-line aggregate bound bit at a mean of 250 lines per tree against a measured 229.7
across 131 trees, while its **file count was already at 81.9% of its warning**. The declared remedy for a
count is to add capacity — but raising it to 200 trees would have produced 45,940 lines of capacity the
aggregate could not accept. The advertised capacity would have been unreachable, exactly as the fact plane's
198 cards were.

The gate now requires `lines_total ≥ files × lines_each` and `bytes_total ≥ files × bytes_each` on both the
health and ceiling bands of every `collection` surface. A `file` locator is exempt, because it holds one
document where the aggregate merely repeats the per-file bound.

One exemption exists, and it must do arithmetic rather than make a claim. A heterogeneous collection may
declare `aggregate_composition` — a rationale plus at least two member roles, each with a count and explicit
health and ceiling bounds — and the gate then requires the counts to sum to the file bound, the products to
sum *exactly* to each total, and the largest member to equal the per-file bound. `fact_index` is its only
user and its reason for existing: the Knowledge Map projection is one 96-line landing plus up to 32 384-line
shards, so its exact legal maximum is 12,384 lines rather than 33 × 384 = 12,672.

Two smaller consequences are worth stating plainly. First, every pressure line now ends with the distance to
the wall — `— 20 below its 384 ceiling` — because a surface past its health target reports a percentage of a
number it already blew, which is the least urgent fact about it. Ranking the report by that distance was
declined: surface-id order lets consecutive runs diff cleanly, and the headroom figure already carries the
urgency. Second, the test harness itself was violating the new rule — `generous_dimensions` declared 16 × 100
lines against a 500-line total — which is the cheapest available evidence that this shape is easy to write by
accident.

#### Bounded FSMGen feedback channel landed

The stable [`docs/FSMGEN_FEEDBACK.md`](../../../FSMGEN_FEEDBACK.md) path is now an 81-line / 5,243-byte
directed current channel. Its explicitly marked open region is empty at the sealed boundary; a concise
six-row register links every closed question, answer, suggestion, feature request, and composite bug
episode to both complete history and independent FSMGen response or local issue-resolution evidence.

Before replacement, the migration froze the exact 936-line / 57,980-byte source at
[`docs/archive/fsmgen-feedback/source-through-2026-08-08.md`](../../../archive/fsmgen-feedback/source-through-2026-08-08.md).
That source contains 359 lines of ordinary directed exchanges, a 104-line resolved two-bug episode, a
15-line scope override, and a 456-line legacy primer. The bounded
[archive index](../../../archive/fsmgen-feedback/INDEX.md) and JSON manifest make its identity and
recovery direct rather than relying on Git history.

The primer also demonstrates a time-layer collision. April prose calls `.fsm` one of SpecForge's
adapter targets, while the May scope override says the only adapter is `.isf`. Its “latest response”
pin remains `030f8c273`, while the tracked FSMGen gitlink advanced to `d327129b7`. The old prose is
useful history, but the word “latest” is not current truth.

[ADR 0011](../../../decisions/0011-bounded-fsmgen-feedback-channel.md) defines two products now in use:

- the stable root remains the bounded current handoff channel;
- an exact immutable capsule retains every old request, response excerpt, bug narrative, and stale
  claim;
- a concise register routes all six closed exchanges to detailed history/evidence;
- an explicitly marked open region contains only active correspondence;
- the upstream response file and reproducible issue-bundle catalog remain the authoritative detailed
  response and bug surfaces.

Future open records declare direction, kind, status, owning task, and evidence. The live set allows at
most eight records, each no larger than 96 lines / 12,288 bytes / 512 bytes per line; detailed designs
and reproductions stay in task/research records or issue bundles. The entire root has independent
256-line / 32,768-byte / 512-byte health targets and 384-line / 49,152-byte / 1,024-byte hard limits.

The migrated executable gate pins the source SHA-256, five exhaustive regions, all six record spans,
status/direction/evidence, both stale-current findings, 26 consumers, current-root schema, and archive
topology. Its ten focused cases cover both lifecycle forms; the live-document doctrine now
authenticates the capsule and independently checks the root, open-record schema, closed register,
manifest, index, direct routes, and pressure ceilings. Feedback transition debt is gone.

#### Validation snapshot means last reviewed

`VALIDATION_SNAPSHOT.md` is not a mirror of whichever git-ignored artifact happens to be newest on one
workstation. It is the tracked **last reviewed** projection. The current boundary remains four IntentIR
reports and 29 rescan recommendations from reviewed commit `a44323d5`; later local promotion results
were deliberately withheld because the approval gate had not promoted them into validated state.

This distinction is executable. `doctrine/live_document_size/validation_snapshot.json` declares the
ordered artifact paths, report fingerprints/scores/grades/finding counts, review evidence, snapshot and
live-block identities, and four exact Rust producer regions. The read-only currentness checker validates
all of that plus summary, recommendation, marker, and queue structure. It never runs the mutating
producer or reads ambient local artifacts, so the same reviewed-state proof works in a fresh clone.

To refresh the tracked projection, run `project-validation` on the intended artifacts, review the
result, then update the declaration, snapshot, and managed live-status block atomically. A local report
delta is evidence to review, not permission to silently rewrite validated state.

#### Source-PDF registry follows Git membership

`corpus/SOURCE_PDF_REGISTRY.md` is a durable source inventory, not a generated-artifact catalog and not
a mirror of the owner's larger local library. Its current membership denominator is the Git-indexed
set of PDFs below `corpus/`. The registry currently maps all 22 tracked PDFs exactly once.

The read-only currentness contract checks safe paths, exact membership, unique keys and paths, derived
parent directories, `%PDF-` signatures, and the real filename-stem-to-`document_key` implementation.
It pins both derivation functions and the three `SourceIR::build` seams that consume them. Thirteen
repository-local mutation cases fail closed and remove their fixtures even when a mutation itself
throws. Adding, removing, or renaming a source PDF therefore requires its registry row in the same
change; host-local and generated-only documents do not silently enter the durable corpus.

#### Root compatibility pointers and one public truth plane

The four former root reference documents no longer carry current behavior beside the mdBook.
`USER_GUIDE.md` had already been declared a compatibility pointer when the book was created, but
feature work appended a command list and current-limitations mirror again. The extraction and
knowledge-graph roots combined durable architecture with April implementation plans, bugs, and
validation state. `INTENTIR_SPEC.md` combined unique product rules with stage/schema examples now
maintained more completely in the book.

The audit preserved the unique pieces in two direct book parts:

- [Extraction Architecture Contract](extraction-architecture.md) owns the six evidence modalities,
  stage obligations, extraction rules, and target-quality boundary;
- [IntentIR Product Contract](intentir-contract.md) owns the canonical product boundary, residual,
  serialization, adapter, and guardrail contract.

The actor graph, temporal semantics, current stage behavior, artifact layout, commands, quality, and
validation already have richer topical chapters. The four root filenames now contain only direct
routes to those homes. `scripts/check_book_current_truth.sh` binds the product claims to the live
`IntentIr` and sole-`Isf` adapter seams, requires every compatibility route, and rejects the stale
root status headings. Independent pointer budgets prevent a second manual from regrowing.

#### Resulting collection and projection closure

The final resulting-tree census covers exactly 604 Git-indexed Markdown files across 37 governed
surfaces. Every file is classified once; all collection indexes and generated projections are direct
and current; all eight currency contracts are enforced; and no surface remains in a transition or
currency-debt state. Five generated member catalogs route 238 canonical files, while the 121-tree task
catalog, decision index, 146-card fact catalog, 147-fact / 1,023-question Knowledge Map, mdBook summary,
and portable bundle keep their dedicated one-hop routes.

The corpus-KB warning found during closure was caused by output shape rather than collection growth.
Its aggregate page repeated a six-line record for every passing fixture, and its prior-candidate table
joined whole fixture sets into single cells. The producer now emits one fixture row plus failure-only
detail and one evidence fixture per candidate bullet. At 156/156 passing fixtures, the 24 Markdown
files total 1,172 lines / 124,679 bytes; the largest file is 201 lines / 25,470 bytes and the longest
content line is 305 bytes. The existing limits were not widened.

Two early pressure warnings remain explicit: the complete fact-card/index plane is at 81.0% of its
per-file line target and 81.9% of its per-file byte target, while the exact owner-reviewed validation
snapshot is at 85.0% of its line target. Neither is debt or at the 90% rollover point. Their named
maintainers must partition before a future addition reaches rollover; evidence is not rewritten merely
to silence a warning, and no ceiling was widened.

Lines, bytes, file count, collection totals, and maximum content-line width are independent axes.
At adoption entry, oversized ledgers were explicit transition debt whose measured baselines could not
move. Every owning migration has now landed: exact capsules retain the original bytes, bounded live
windows carry current records, and the resulting registry contains no transition or currency debt.
A legacy ceiling is never advertised as health.

#### Adoption program closed and verified

The delivered implementation composes four controls: a bounded README and impact-routed continuity
contract; exact archive-backed live windows plus bounded collection/projection indexes; executable
currentness for maintained/generated views; and repository-root-derived temp, cache, Python, model,
and child-process data. The last control is described in
[Project Data Locality](project-data-locality.md). Shared `~/.cargo`, `~/.rustup`, and the ambiguous
home Hugging Face cache remain explicit protected inputs rather than being relabelled project data.

The closing audit reopens every route and identity rather than trusting earlier commits. All six
doctrines pass; the common lifecycle suite passes 55/55; all four rolling capsules, roadmap and FSMGen
feedback archives, task/fact/catalog/Knowledge Map projections, and eight currency contracts retrieve
and validate. Formatting, warning-deny Clippy, 1,730 Rust tests with five intentionally ignored,
rustdoc, and the complete mdBook build pass with caller temp/cache variables absent. The pinned FSMGen
gitlink is unchanged, the old boot-volume repository is absent, the shared caches remain present, and
the repository-local temporary root finishes with only its tracked placeholder.

Documentation synchronization is now impact-based. Every completed slice updates
its owning task-tree leaf. Other surfaces change only when their own truth changes:
the resume pointer when the next action or in-flight state changes, the status page
when product status changes, and the book when user understanding changes. Editing
an unrelated document merely to show that it was reviewed is prohibited.

The resume pointer also does not copy Git's current revision. Run
`git rev-parse HEAD` and `git status --short --branch` when resuming; `MEMORY.md`
owns only the active unit, concise current state, next action, in-flight work, and
blockers.

## Root docs and what they are for

- `README.md`
  - entry point and high-level navigation
- `ROADMAP.md`
  - bounded high-level direction and workstream status
- `LIVE_ACHIEVEMENT_STATUS.md`
  - current status snapshot
- `VALIDATION_SNAPSHOT.md`
  - tracked projection of persisted validation reports
- `RUST_CODEBASE_ANALYSIS.md`
  - architecture and codebase state
- `DEVELOPMENT_NOTES.md`
  - engineering rationale
- `CHANGES.md`
  - detailed current change history
- `MEMORY.md`
  - crash-safe continuity record

## How to think about each surface

The book should be stable enough for a new reader.
It should explain:

- what `specforge` does
- how the pipeline works
- why the project is structured this way
- how to run and inspect the tool
- how validation and learning are supposed to behave

The root live docs are more operational, but they are still governed.
They track:

- current baseline scores
- active roadmap slices
- implementation notes
- handoff context
- recent decisions
- crash recovery breadcrumbs

That means root docs may mention temporary states, current artifacts, local blockers, or implementation
details that would be too specific for the public book. It does not mean one mandatory file may grow
forever; chronology and large evidence move to indexed, query-first storage at lifecycle boundaries.

## Why the split exists

The book is what the world should see.
It should explain the product clearly and transparently.

The root docs stay live and operational because they are part of the project's continuity system for
active development and session recovery. Their working sets remain bounded; durable detail stays
recoverable through the registered task, decision, fact, shard, projection, or Git-history route.

## What belongs in the book

Every durable user-facing concept should eventually land in the book.

Examples:

- runtime setup
- command usage
- generated artifact layout
- the four IR stages
- validation scoring
- KG fixture benchmarking
- cross-document prior memory
- clock/reset infrastructure semantics
- corpus knowledge-base direction
- troubleshooting guidance

If a concept is part of how `specforge` works or why it is built that way, the book should explain it at the right level of depth.

## What belongs in live docs

The root live docs are the right place for:

- current score snapshots
- detailed change chronology
- session handoff notes
- roadmap sequencing
- implementation rationale that may still be in motion
- local runtime findings
- warnings about artifact freshness

Those docs can be more current and operational because their main job is continuity. Detail still has
to obey the registered lifecycle: update a bounded snapshot, append only within a rolling window,
partition canonical evidence, or route exact history to its controlled terminal.

## When to update both

Many changes should touch both surfaces.

For example:

- a new public command should update the book and the live docs
- a new IR concept should update the book and the live docs
- a score-only refresh may update only the live docs
- a docs-architecture clarification may update only the book plus continuity notes

The guiding rule is:

- update the book when the public understanding of `specforge` changes
- update live docs when the repo’s current engineering state changes

## Practical rule

Use the book to understand and evaluate `specforge`.
Use the root docs to understand the repo’s live state and recover work safely.

## Why this matters for long-running work

`specforge` is being built across many small truthfulness and architecture slices.
Some runs are long, and some work depends on local generated artifacts that are intentionally not committed.

The live docs make that workflow survivable.
They let a future session answer:

- what was the last known baseline?
- what changed recently?
- what is the next likely slice?
- which artifacts were refreshed?
- which caveats are still real?

The book, meanwhile, keeps the project understandable to someone who did not live through those sessions.

## The README is a bounded landing page

Root `README.md` is intentionally smaller than either documentation plane. It keeps only the project
purpose and audience, top-level scope, prerequisites, one verified first-use path, stable architecture,
and navigation. Current delivery state belongs in the roadmap and task trees; command and feature
detail belongs in this book; chronology belongs in Git or a governed historical surface.

The repository-owned `README_POLICY.md` makes that boundary explicit. Its unconditional doctrine
check independently enforces 150 lines and 5,800 bytes, values derived from the reviewed 127-line,
4,834-byte survivor rather than copied from another project. A data-only route inventory distinguishes
reader navigation from places authors may put changing detail. Missing, off-repository, duplicated, or
uncontrolled routes fail the same pre-commit and CI doctrine driver even when `README.md` itself was not
changed.

This is containment, not documentation reduction: material removed from the old landing page was
already duplicated by richer canonical homes. The user manual remains the complete maintained product
surface, while task state and rationale remain available through their dedicated continuity layers.

## Closed task trees — how each was implemented and verified

### `MDBOOK-DOCTEST-HYGIENE` — executable examples stay executable

The rendered book had remained healthy while the optional native doctest command reported 26 failures across
four chapters. The bounded repair first measured every opening, then classified non-Rust material as `text` and
ten deliberately incomplete Rust illustrations as `rust,ignore`; three self-contained Rust examples remained
ordinary executable `rust`. No example body changed.

The close leaf made that corrected behavior durable in `scripts/run_docs_ci.sh`: it runs
`mdbook test docs/book` immediately before `mdbook build docs/book`. Full `scripts/run_ci.sh` already consumes
that entrypoint exactly once. The focused docs gate and final full CI both pass, so a future fence-classification
regression fails before the rendered book is accepted. Authoritative evidence lives in
`docs/tasks/MDBOOK-DOCTEST-HYGIENE.md`.

### `SIGNOFF-REMEDIATION` — the doctrine that keeps `main` always-shippable

If you `git pull main` and run `scripts/run_ci.sh`, it should
be green. That sentence is meant to be unconditional. The
`SIGNOFF-REMEDIATION` tree is the work that paid for it, and
the doctrine that keeps it true.

#### The user-facing guarantee

> **`scripts/run_ci.sh` on `main` is always green. If you
> branch from `main`, your starting point is clean —
> formatted, clippy-clean, every test passing. Every leaf
> that lands has to keep it that way; nothing about a leaf is
> "done" until CI is green again.**

That's the doctrine. It's non-negotiable: a leaf isn't
complete until `scripts/run_ci.sh` runs through to green at
the leaf's commit.

#### Why this isn't free either

When `main` drifts to "fails CI but probably works", several
nasty things happen at once:

- **New leaves can't tell whether they broke something.**
  Running CI on a branch and getting failures, the author
  has no way to know which failures are theirs vs which were
  inherited from `main`. The signal-to-noise on every
  subsequent leaf collapses.
- **The bar slips, quietly.** "We'll fix the 3 clippy
  warnings later" becomes "we'll fix the 7", becomes "we'll
  fix the 25." There's no natural moment to stop the slide;
  there's always a more urgent leaf.
- **Idiomatic fixes get replaced by `#[allow]`s.** Under
  pressure to make CI green again, the temptation is to
  silence the lint rather than fix the code. The lint is
  there for a reason; silencing it loses the signal.

`SIGNOFF-REMEDIATION` showed up after the bar had drifted to
25 clippy errors plus formatting failures. The tree
restored CI to green and turned the bar into a doctrine that
prevents the drift from happening again.

#### What this tree fixed (concretely)

When the tree opened, HEAD failed both `cargo fmt --all
--check` and `cargo clippy -- -D warnings` (the second with
25 errors). The remediation followed two rules strictly:

- **Idiomatic fixes only.** Every clippy lint was addressed
  by rewriting the code the way clippy wants it (use
  `Result::ok()` not a match; use `.contains(&x)` not
  `.iter().any(|y| *y == x)`; etc.). No blanket
  `#[allow(clippy::…)]` at module level.
- **Narrow, recorded `#[allow]` only where genuinely
  inapplicable.** When a lint truly doesn't apply (e.g.
  `#[allow(clippy::too_many_arguments)]` on a verbose test
  helper), the allow goes on the function or line — never
  the module — and the reason is recorded in a comment.
- **Zero production behaviour change.** Every fix had to
  preserve what the code does. The tree was a CI restoration,
  not a refactor.

After this tree, the bar held for every subsequent close —
the long R16 program shipped 40+ leaves with
`scripts/run_ci.sh` green at every single commit, because the
doctrine and the per-leaf signoff discipline kept it green.

#### How the doctrine is enforced now

The doctrine became **structural** through `COMMIT.md`'s
"Required Commit Workflow":

- Every commit's workflow ends with `scripts/run_ci.sh`. The
  step isn't optional; it's part of what a commit means.
- Per the close-rule (and the `BOOK-METHOD-DOC`
  reinforcement), a tree's close leaf has the same CI
  requirement: green or it isn't closed.
- Idiomatic fixes only is recorded as the canonical way to
  land clippy/fmt deltas; `#[allow]` is permitted only at
  line/function level when the lint is genuinely
  inapplicable, with a brief recorded reason.

The result: a contributor who follows the workflow doesn't
have to choose between "ship it" and "keep CI green" — the
workflow makes them the same choice.

#### What this buys you, as a SpecForge user

- **You can `git pull main` and start working immediately.**
  CI is green from your starting commit; failures you see
  while developing are yours, not inherited drift.
- **Every leaf you read about in the book or in
  `CHANGES.md` shipped under a green CI gate.** That's not
  decoration; it's a hard precondition the workflow
  enforces.
- **clippy / fmt deltas are small and idiomatic.** If you
  send a contribution, the precedent is clear: fix the lint
  the idiomatic way; reach for `#[allow]` only when the
  lint genuinely doesn't apply, and only at the narrowest
  scope.

*Authoritative tracking:*
`docs/tasks/SIGNOFF-REMEDIATION.md`. The doctrine is
recorded in `COMMIT.md`'s "Required Commit Workflow" — every
commit closes on `scripts/run_ci.sh` green.
