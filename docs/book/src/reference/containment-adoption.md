# Containment Adoption, Surface By Surface

Live-document containment did not arrive as one change. It was adopted surface by surface, in a
dependency order the census chose, and each surface taught something the next one needed. This
chapter is that record: what was closed, in what order, and what each closure turned out to cost.

It is history, kept because the reasoning is reusable — a bound that had no remedy, an aggregate a
collection could exceed, a landing that grew with the project rather than with the work in flight.
The doctrine those closures produced, and the state of the surfaces today, are in
[Live Docs And Continuity](live-docs.md).

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

`docs/TASK_TREE.md` derives one concise row per real task tree, with `TEMPLATE.md` linked
separately for authors. The live count is derived on read, not stored here — `perl
scripts/check_task_tree_catalog.pl --check` prints it, the same way
`STATUS-LEDGER-ROLLOVER.1` replaced this chapter's stored ledger sizes: a copy of a value that every
new tree invalidates is the derived-state anti-pattern the containment doctrine names.

The workflow contract remains outside the generated section. At the migration boundary the index moved
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

At that boundary every fact-plane rollover warning was gone: 195 of 338 collection files (57.7%), 30 of 44 decision records
(68.2%), 2,976 of 12,384 projection lines (24.0%), and 199 of 379 facts. No card and no decision record was
edited. One warning survives deliberately — `knowledge_cards` `lines_each` at 81.0%, because one 243-line card
sits against a 300-line per-card bound. That is a per-card quality signal with a local remedy, not capacity
pressure. The portable bundle's own hard caps (512 facts, 4,096 question keys, 64 shards) became the visible
architectural ceiling: that profile used 74% of the fact cap, and growing past roughly 470 cards would have had
to raise the bundle's caps first.

#### Decision growth re-derived the coupled profile

ADR 0041 applies the same equation to changed inputs rather than treating the ADR 0029 numbers as permanent.
The deciding record brings the collection to 43 files and Git still shows a nine-record peak active day.
Fifty-eight is the smallest new file bound for which 43 is below the 80% warning and 52 is below the 90%
rollover. The per-file line, byte, and width limits do not move; aggregate line and byte bounds remain exactly
`files × per-file bound`.

This changes both downstream capacities as one transaction. Fifty-eight decision files contain at most 57
answers-bearing records, so the fixed 336 card slots yield `336 + 57 = 393` facts. Eight question keys per fact
then require 3,144 keys, rounded up to the registry's 512-key quantum: 3,584. The catalog checker recomputes both
identities and its controlled mutations make a fact-only, decision-only, or question-key-only change fail.
Existing decision paths, the 32-shard topology, and the portable 512-fact/4,096-key hard caps stay fixed.

#### Seven parts, and what the key budget actually funds

`LIVE-DOCUMENT-PRESSURE-HEADROOM.24a` applied the same equation once more: `max_parts` 6 → 7 carries
`max_cards` to 392, `knowledge_cards.files` to 394, `max_facts` to `392 + 57 = 449`, and the derived key
budget to 4,096 — exactly the portable bundle's hard cap.

Landing on the cap looked like the cap becoming the binding authority, and `.28` re-derived that and found it
false. `max_question_keys` is computed as `max_facts × 8` rounded up to a 512-key quantum, and **eight keys
per fact is below what a fact costs**: at the `.28` boundary the measured population was 2,767 keys over 327
facts, about 8.46 each. So the declaration is funded by the quantum's rounding slack, not by the ratio it
names. In funded terms that key budget carried `int(4096 × 327 / 2767)` = **484 facts**, with `max_facts` 449
inside it — the repository's own fact cap binds first, and the plane's nearest real stop is the card plane.

The same arithmetic still refuses an eighth title part, for the funded reason rather than the declared one: it
would raise `max_facts` to `448 + 57 = 505`, above what the budget funds. Nothing in the derivation refuses it
— `505 × 8` rounds to 4,096 as well, and 505 is under the portable 512-fact cap — so the contract would accept
the raise and the projection would refuse later. The lever that makes an eighth part reachable is the
keys-per-fact ratio falling far enough to fund it, never a cap raise.

`.28` also closed the gap that let this go unnoticed. `max_facts` and `max_question_keys` were the only
capacity authorities in the repository that refused with no warning band: a file, line, or byte dimension is
banded by the surface registry, and a bounded-registry record count by the central discovery over
`doctrine/**/*.jsonl`, but a fact and a question key are neither. Both now carry the same 80/90 band every
surface uses, and `perl scripts/check_knowledge_map_shard_contract.pl --report` publishes the funded count, so
a raise into the region the rounding hides is visible in the commit that declares it. `max_shards` stays
unbanded on purpose: the `knowledge_map` surface already bounds the projection at 33 files — one landing plus
32 shards — so a second band would report the same state twice.

#### Nine parts, once a fact is funded at what it costs

By `2026-09-29` a fact cost **8.73 question keys**, not the eight the budget assumed, and the card plane was less
than two days of ordinary work from refusing a card. Merging cards is ruled out (ADR 0026 decision 3), so ADR 0055
takes the step ADR 0029 named for growth of this size. It moves the bundle's fact and key caps, and it funds a fact
at its measured cost, ten keys per fact, stated once in the catalog checker. The laws already in force then
re-derive every other bound together, card slots, decision files, facts, keys and shards, and the ADR carries the
arithmetic. The plane grows to nine title parts and no fact-plane axis warns afterwards. The same record names the
next stop: the projection now declares all but one shard's worth of its byte cap, so the next raise has to shrink
the per-shard byte bound first.

#### A complete decision record is sealed, not warned about

On `2026-10-21` ADR 0038, an accepted record, held 474 of 512 lines and 31,833 of 32,768 bytes. It was past rollover
on both, so `decision_records` reported a rollover on every run for a file that takes no more writes. That warning
was misread three times as "no room for a new decision record", although a new record is a new file. ADR 0058 lets a
collection seal a member that has reached its per-file rollover: the surface lists it with its content's SHA-256, an
edit fails closed, and it is amended by a new member. The per-file milestones then measure the members that can
still grow, while every ceiling, the file count and both aggregates still measure all of them. A member below
rollover cannot be sealed, so the mechanism cannot quiet a file that is still growing.

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

Two collections keep more natural existing routes. At that boundary `docs/TASK_TREE.md` directly covered
all 122 task files—121 real trees plus the separately linked author template—and the portable Knowledge Map
bundle uses its own README to link all four bundle documents. The task index has since become a routed
landing; see "The task landing carries open work, not lifetime" below. In total, the eight former query-only surfaces
now have direct membership proof over 367 canonical files without moving or copying their prose.

#### The task landing carries open work, not lifetime

`docs/TASK_TREE.md` no longer lists every tree. It lists every **open** tree — anything not `done` or
`superseded` — and routes the complete catalog to derived parts under `docs/task-catalog/`, exactly as the
Knowledge Map routes questions to shards and the fact-card catalog routes titles to parts. Both the landing
section and the parts are generated by `perl scripts/check_task_tree_catalog.pl --write` and proved by
derive-and-diff; neither is hand-edited, and every tree appears in exactly one part.

The point is which quantity the bound now measures. A landing that lists every tree grows with project
lifetime, so its line ceiling becomes a cap on how much history the project may accumulate. A landing that
lists open trees grows with concurrent work in flight, which ordinary work reduces by finishing trees. There
is no limit on the number of task-trees (ADR 0045); a finished tree stays as project history in the parts.

The same exemption now covers the research plane. `docs/research/` declares no file count either, and for the
same measured reason its registry entry records: only the count was loaded, while the line and byte aggregates
that bound the resource actually consumed sat far below their targets, and the count had no rollover a
compliant author could take. `docs/catalogs/research-records.md` remains the bounded route, so what stays
bounded is the reader's path rather than the project's history. Every exemption is declared in the registry and
gated — a bare `null` is still refused, and the resource dimensions must stay numeric.

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

#### Workflow-standard headroom is measured, not guessed

Publishing the claim-verification contract and pull-request review template grew the canonical workflow set from
12 to 14 explicit members against a 16-file profile. That 87.5% count is a warning about collection growth, not
large documents: the stable paths, individual files, generated browsing catalog, and registry array all remain
comfortably inside their own bounds.

[ADR 0043](../../../decisions/0043-workflow-standard-capacity-is-rederived-from-explicit-member-growth.md)
therefore repeats the same warning/rollover derivation used for decision records. Git creation history records a
four-member peak active day. Twenty-one is the first capacity where current use is strictly below the 80% warning
and one peak is strictly below 90% rollover:

- `14 / 21 = 66.7%`;
- `(14 + 4) / 21 = 85.7%`;
- `18 / 20 = 90.0%`, so 20 is not safe under the strict boundary.

The collection stays an explicit target list so a Markdown file cannot silently become policy. Its 700-line,
65,536-byte, and 1,024-line-byte per-file limits do not change; reachable aggregate bands become 14,700 lines and
1,376,256 bytes. A full 21-member generated catalog still fits below its own ceilings, and 21 stays below the
registry's independent 32-item portable array cap. Reproduce the boundary with:

```sh
scripts/measure_workflow_standard_capacity.pl --check
```

The generic live-size count warning is the signal to remeasure again. Ordinary additions do not automatically
raise capacity, and the exact 16→21 authority is retired as soon as the profile becomes committed baseline state.

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

`MEMORY.md` has an exact 32,768-byte (`32 * 1024 B`) maximum so an agent can consume the complete pointer in one
read. That outer byte boundary does not replace its stricter content-shape controls: the pointer remains limited
to 50 lines, 160 bytes per content line, and overwrite-only current state. History and durable detail still route
to task trees, decision records, fact cards, and Git rather than using the larger byte ceiling as storage.
