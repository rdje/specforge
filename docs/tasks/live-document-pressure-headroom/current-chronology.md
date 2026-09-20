# LIVE-DOCUMENT-PRESSURE-HEADROOM — current chronology

- Part ID: `current-chronology`
- State: `active`

The writable half of this tree's chronology. [Verification and chronology](verification-and-chronology.md)
is sealed `legacy` at the `2026-09-16` migration boundary (`.30`, boundary commit `e39c41d0`) and cannot
take a row, so every closure since then wrote its record into the bounded root instead, where it accreted
until the root had one line of headroom left. This part is that record's writable home. The root now keeps
the shape of both log tables and no rows, so a closure costs it one owner-registry line and nothing else.

**Rollover trigger, stated so it is not discovered by a refusal.** This part accretes by construction: one
verification row and one commit row per closure, a measured **523 bytes** together across the 14 closures
moved here. Its health target is **65,536 bytes / 768 lines**. At the 80% warning band — **52,428 bytes** —
seal this part and open `current-chronology-0002` under `.31`'s partition shape. A chronology splits on the
axis it is already ordered by, so the cut is a date and nothing has to be re-read to choose it.

## Verification Log

Every dated entry since the seal, byte-identical to the root row it was moved from. The pre-seal log is in
[verification and chronology](verification-and-chronology.md), which remains the complete record through
`2026-09-16`.

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-21` | `.34a` | the cut taken at 79.7% on `.34`'s prediction rather than at a refusal; every pinned region re-derived at its new location before the move was kept | `evidenceir.md` **104,450 -> 21,796 bytes (79.7% -> 16.6%)**, chronology chapter 83,519 (63.7%), **29 of 29 regions verified**. **No book file is in a warning band at all** — the maximum is now `isf-adapter.md` at 68.3%, the first time since this tree opened. The relocation is stated: the new chapter is the accreting half and its own trigger is the 80% band at 104,857 bytes, where the next cut goes by reader concern (rule 2), not by another chronology split |
| `2026-09-20` | `.38` | the proposed remedy tested against this tree's own two standing rules before it was adopted | **widening `PUBLISHED-ASSERTIONS` to task-tree prose is REFUSED**: a task tree is dated history, so the check would refuse correctly-dated records — `.25` (a gate refusing most of its population is a policy defect) and `.29c` (not pressure until reachable). Remedy shipped as an authoring rule in `COMMIT.md` instead: read a closing record's before-value at `<this leaf's commit>^` |
| `2026-09-20` | `.34.2` | `live-docs.md` measured at every commit boundary of this session rather than at the two ends of it | **`.34` published a start value that was not its own**: 121,517 (92.7%) is the pre-`.37` state, 124,084 (94.7%) the pre-`.34` state. `.34`'s effect is **124,084 -> 75,698**. `surfaces.jsonl` already said 94.7% in the same commit, so two surfaces disagreed on one field with every gate green — `PUBLISHED-ASSERTIONS` does not govern task-tree prose |
| `2026-09-20` | `.34.1` | every figure `.34` published re-derived by a SECOND method — endpoint differencing instead of per-revision accumulation, and a real-tree-id match instead of a shape regex | **four reproduce exactly, one is method-dependent**: the moved block is `+23,984 of +55,557` against the published `+24,085 of +55,799`, ~0.5% apart, ranking and margin unchanged. Root `146 -> 229` with Current Frontier **+39** matches digit for digit; the heading census returns the same **75 of 239** with zero shape-only false positives. Repair is to quote the method, not to change the number |
| `2026-09-20` | `.34` | per-block growth attributed across the last 40 revisions of the file before a cut was chosen; every pinned region re-derived at its NEW location and required to match before the move was kept | the accreting block is `Collection and projection closure order` — **+24,085 of +55,799 bytes** (per-revision accumulation; endpoint differencing gives +23,984 of +55,557, same ranking) and the largest block at 48,634 — and it is a chronology inside a doctrine chapter. Moved byte-identically, no heading promoted: `live-docs.md` **124,084 -> 75,698 (94.7% -> 57.8%)** (start value corrected by `.34.2`), new chapter 49,293 (37.6%), and `shipped_behavior` **no longer warns on bytes**. 100 regions re-pointed, 12 shifted, 71 untouched, 183 of 183 digests verified; one currency literal followed the move and was repaired |
| `2026-09-20` | `.20` | each acceptance leg checked against the artifact that satisfies it rather than against the leaf's own summary; the manual re-censused for headings still named for a work unit | the deliverable is **three** rules, not one, and it has been applied twice — `SIGNAL-CATALOG-CAPTURE-GAP.3`'s heading lost its leaf id in the rule's own commit, and `.37` extended the section that already owned its concern. **The forward-looking half is complete and the backlog is not**: 75 of 239 `###` headings still lead with a work-unit id, 26 in `evidenceir.md`. `.34` answered rather than left open — `reference/live-docs.md` is the SAME mechanism, largest section **97,007 of 124,084 bytes (78.2%)** against `evidenceir.md`'s 78.8% |
| `2026-09-20` | `.37` | root growth attributed per section across all 19 post-migration revisions before a cut was chosen; every removed row tested for a superset in the sealed part, per row, on content-bearing tokens rather than by eye | **the accreting section was `## Current Frontier`, not the log windows** — +39 lines of the +83 on the binding axis, against +13 and +13. The token test also refused a row that read as a duplicate: the second `.30` row carries `` `pending` ``, which its sealed counterpart does not, so 14 rows moved where reading had cleared 13. Root **229 -> 166 lines (89.5% -> 64.8%)**, bytes **82.4% -> 33.6%**, widest line **80.3% -> 22.1%**; all three axes leave the warning band |
| `2026-09-19` | `.36b.1` | every published finding of `.36b` re-derived on the director's audit challenge | `the envelope is spent` REFUSED by arithmetic — the per-record ceiling only has to ADMIT the largest record, so a re-derivation still reaches more slots than shipped; the trajectory and the slot count also corrected; the residual is DERIVED now, self-test 9/12 -> 12/12, new control observed RED |
| `2026-09-19` | `.36c` | the banked-authority refusal observed at `387009a4` before the record was removed | `FAILED with 1 violation(s)`, green after; emptying the authority registry also cleared the capacity warning `.36b` predicted would be transient, so the observer reads content and not a declaration |
| `2026-09-19` | `.36b` | the handed-down sizing re-derived per revision before use; the new gate attributed by revert-and-re-apply | the 8,192 ceiling was stale by two commits and would have refused a committed record (7,827 at `ade27bc2`, 10,223 at `95c81cd1`); no coherent triple fitted the old envelope, `13 x 10,240` exceeding the compiled 131,072. 94.7% -> 24.0%; suite 113 -> 116, RED matrix 7 -> 9 |
| `2026-09-17` | `.33` | the writable stratum measured separately from the file; both strata of every route on the part enumerated before the region was moved | 95.2% of the writable budget against 87.5% of the file; THREE leaves declared `pending` inside the sealed payload and `done` outside it, so the closures had to travel with the region |
| `2026-09-17` | `.31` | the partition run once; losslessness re-derived independently of the writer by re-harvesting only the marked payloads; every node lifecycle derived before the source lock | 268,250 bytes reproduced byte-for-byte, 56 of 56 nodes re-declared, root 2,998 -> 132 lines; one node declared `pending` over its own closure record and was corrected first |
| `2026-09-17` | `.32` | the widened node reader A/B'd against a copy with only the reader reverted; all five registered contracts re-checked | 466 of 1,362 node declarations state status inline and were unreadable; `EXTRACTION-QUALITY-GAUGE` corroborates 0 of 56 before and 56 of 56 after, the three migrated trees move by nothing; suite 64 -> 69 |
| `2026-09-17` | `.30c` | every ``row below`` pointer in the tree censused and each one resolved against the part it names | 8 pointers, all in one part: the 4 sealed ones still resolve to the chronology part, the 4 post-migration ones resolve to 0 rows because that part is sealed; no gate reads an unqualified route |
| `2026-09-17` | `.28` | the key budget's derivation read from `check_fact_card_catalog.pl` rather than from `.24a`'s note; both arms driven against the real 327-fact projection | the cap was never the authority: `max_question_keys` is derived at a fixed 8 keys per fact, so 505 rounds to 4,096 too and the contract would have accepted an eighth part |
| `2026-09-17` | `.30b` | the active part's writable budget separated from its sealed payload and measured; region moved byte-for-byte; `--check`/`--write`; `check_live_document_size.pl` before and after | 87.0% of the file was 87.3% of the budget, 1,997 bytes, less than one 3,667-byte closure; the sealed frontier region moves out and the budget goes 15,768 -> 36,773, six records of room |
| `2026-09-16` | `.29c` | reachability tested against 323 revisions of the archive floor; both arms re-derived; the suite extended to pin warning text | **the discrepancy was unreachable** — the census refuses at 91 current surfaces, needing archive/frozen <= 4 against 18 that never decreased; one-line fix, suite 28 -> 29 |
| `2026-09-16` | `.29b` | both registries' capacities traced to the leaves that sized them; per-surface cost measured; RED control on the reachable arm | the pair cannot agree by comparing headers — `surfaces.jsonl` bounds records, the census mirrors the CURRENT subset, and nothing bounds that; 95 surfaces need 236 of 224, so the identity is now checked with an error arm and a warning arm; suite 27 -> 28 |
| `2026-09-16` | `.27a` | per-section growth attributed across all 58 revisions; both citation forms censused; `check_live_document_size.pl` before and after | 270 of 274 added lines are INSTANCE material, so the file was two documents; 597 -> 312 lines, widest line 909 -> 444, and `workflow_standards` warns on nothing |
| `2026-09-16` | `.30a` | A/B of HEAD's checker and this one at the same path against the same tree; 3 new self-test assertions | HEAD refuses the supersession by name, this version accepts it; 22 sealed-open leaves across three trees unblocked; suite 61 to 64 |
| `2026-09-16` | `.30` | node lifecycle derived per block before the source lock | three closed leaves still declared `pending`; nothing checked it |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.34a` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.34a — cut the chapter before it breached, not after` | the first cut this programme has taken on a prediction instead of a refusal |
| `.38` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.38 — the remedy is an authoring rule, not a wider gate` | the cheapest remedy that a compliant author can actually take |
| `.34.2` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.34.2 — attribute the effect to the leaf that caused it` | a before/after span must start where the leaf started |
| `.34.1` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.34.1 — quote the method with the number` | a figure that a second reasonable method reproduces differently needs its method stated |
| `.34` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.34 — move the accreting block out of the chapter it was filling` | the cut is between narrative KINDS — a chronology grows, a doctrine statement does not |
| `.20` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.20 — close the routing rule and answer the chapter it hands to .34` | a rule that stops new growth does not undo the old; rule 3 does, and `.34` owns executing it |
| `.37` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.37 — give the post-seal chronology a writable home, then bound the root` | the root's own remedy needed one step more than its header advertised |
| `.36b.1` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.36b.1 — the envelope is not spent, and the number is derived now` | a decision treated as a bound; the repair is a gate, not a better sentence |
| `.36c` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.36c — the permission expires when the increase is in history` | the retirement also discharged the prediction the raise left behind |
| `.36b` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.36b — the sizing was stale and no coherent triple fitted the envelope` | a sizing handed forward is an input to re-derive, not a number to apply |
| `.33` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.33 — a sealed region cannot move out of an active part alone` | `.30a` reads two strata inside ONE file; splitting them re-derives the superseded lifecycle |
| `.31` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.31 — partition the file that became too full to hold its own remedy` | the remedy was routed, never deleted; `max_unverified_routes` is 0 because `.32` ran first |
| `.32` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.32 — read the node status shape a third of the trees are written in` | a gate that cannot read a declaration counts it as unproven, not as absent |
| `.30c` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.30c — make the post-migration verification pointers resolve` | a route with no filename is invisible to the anchor gate |
| `.28` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.28 — band the two fact-plane caps that refused without one` | a bound with no band is discovered by a refusal |
| `.30b` | shipped in `LIVE-DOCUMENT-PRESSURE-HEADROOM.28 — band the two fact-plane caps that refused without one` | three quarters of the part was history the budget still counted |
| `.29c` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.29c — the mirror warning compared bounds, not reachable states` | a bound-versus-bound comparison is not pressure until the state is reachable |
| `.29b` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.29b — make the census/surface-registry identity mechanical` | two sized-independently bounds joined by an enforced identity; `.29c` owns the capacity decision |
| `.27a` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.27a — route the instance out of the portable doctrine standard` | one routing change cleared both axes of the surface |
| `.30a` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.30a — read a migrated part as two strata so a sealed leaf can close` | the migration had just multiplied the stop by nineteen |
| `.30` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.30 — write the closing record before locking the partition source` | the only ordering that leaves the sealed evidence true |

## Closure narrative through `2026-09-19`

Moved verbatim by `.37` from the root's `## Current Frontier`, which is where it had accreted at a paragraph
per closure. These are dated records of each closure as it was written, not a current-facing statement; the
tree's present state is the root's `## Current Program State` and its next step is the root's frontier
section. **One sentence has been overtaken and is left standing rather than edited**: the first paragraph
calls `.36c` a remaining open leaf "mandatory on the next commit", and `.36c` closed on `2026-09-19` at
`a1da4e82`. That staleness is the reason a frontier section must be overwritten rather than appended to,
which is the rule `.37` put in its place.

The nearest measured stop is `.12`, which owns the 909-byte table row `.27a` relocated onto the
`doctrine_instance` surface. `.28` closed the fact plane by re-deriving it: the portable key cap was never the
binding authority, because a 4,096-key budget funds 484 facts against a `max_facts` of 449 — what was missing
was a band, and `max_facts` and `max_question_keys` were the last capacity authorities here that refused
without one. `.30b`, which `.28`'s own closure opened, gave this tree's writable stratum a budget again: the
active part was 87.0% full of a health target three quarters of which is sealed history. `.29c` closed the
census/surface-registry question the same way `.28` closed its own: the two bounds only disagree on a
fully-current plane the registry cannot hold, because archive records never leave. `.27a` closed the axis it owned:
`workflow_standards` now warns on neither dimension, and the 909-byte line it could not shrink moved to the
`doctrine_instance` surface, where `.12` follows it. The remaining open leaves are the assigned warning
successors `.8`-`.13`, the alignment-index residuals `.16`/`.17`, the semantic-part and ledger splits
`.14b`/`.14c`, the book routing rule `.20`, the classification-refusal control `.6`, the claim-registry
successors `.36c` (retire the consumed authority — mandatory on the next commit) and `.36d` (the lifecycle),
and the containers `.2`, `.4` and `.14`.

`.14d` closed on `2026-09-18` by taking a split at 80.3% instead of at the stop. The extraction-quality
tree's active part had 6,330 bytes of headroom against a measured 4,900-6,400 per closure, so one more leaf
would have reached mandatory rollover — `.30b`'s shape and `.31`'s hard escape, one leaf earlier and cheap.
The cut followed `.33` rather than the midpoint: the sealed region's own closures travelled with it, and the
new part is `active` because `.3j.1.b` is still open inside that region. 52,652 to 27,209 and 26,095 bytes,
every pinned digest preserved, no bound moved. The trade is stated and owned: parts 12 to 13 of a 16-file
health target crosses the `files` warning at 81.2%, with 11 parts of headroom to the ceiling, and `.14e`
owns that axis rather than leaving it standing.

`.36` opened on `2026-09-19` against `doctrine/claim_verification/claims.jsonl`, and **`.36a` corrected its
framing the same day**. The registry is not full of waste and does not need an archive: a banded registry's
three bounds are not independent, and nine of the ten that declare all three promise a record capacity their
byte bound cannot fund. The class census (`scripts/measure_registry_capacity_coherence.py`) found exactly one
coherent registry, and found that `claims.jsonl` is the only one where the incoherence BITES — it writes the
largest records in the class, a mean of 5,171 bytes against the next largest at 1,791, so its byte bound funds
**12 records while its contract declares 64**. It stopped at a number that appears nowhere. The remedy is
therefore a coherence rule rather than more room. `.2c`'s rule — a bound a surface can actually reach must
have a remedy compliant work can take — is satisfied by the derivation, not by a raise.

**`.36b` shipped that derivation, and had to re-derive the sizing it was handed before it could.** The 8,192
per-record ceiling was stale two commits after `.36a` measured it and would have refused a committed record,
and no coherent triple existed inside the old portable envelope at all. Shipped `12,288 x 21 = 258,048`
exactly, against a compiled cap moved to the 262,144 two sibling claim registries already use: bytes
**94.7% -> 24.0%**, `coherent` **NO -> yes**, nothing at or above a rollover milestone. The biting condition
now reports on every commit from the same place `.22b` computes the class band; plain incoherence stays a
measured property, because eight registries carry it without harm. `.36c` retired the consumed authority — RED
observed first, and emptying that registry also discharged `.36b`'s prediction that its one true positive was
visible only while an authority was banked. `.36d` owns the lifecycle, because the derivation buys **9
slots** (eight after `.36b`'s own claim record) and leaves the class envelope **nearly** spent —
`.36b.1` refused the stronger claim it first published: a further re-derivation still reaches 23
records, two more than shipped, at the price of the per-record ceiling's own headroom.

## Task nodes

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.37`
  Status: `done` (`2026-09-20`; opened the same day by `.20`'s correction)
  Goal: give this tree's post-seal chronology a writable home and bound the root's accreting sections, so
  the next closure has somewhere to be recorded
  Acceptance: no record leaves the repository; every moved row is byte-identical to the row it replaces; a
  row is dropped only where a chronology part already carries a superset of it for the same date and leaf,
  proved per row rather than assumed; the root leaves the warning band on all three axes; no bound moves.
  **The root had one line.** Measured `2026-09-20` at `daed3b34`: **229 lines of a 256-line health target
  is 89.5%**, 230 is 89.8%, and 231 is **90.2% — mandatory rollover**. Bytes 20,244 of 24,576 (**82.4%**)
  and the widest line 411 of 512 (**80.3%**) were in the warning band too, so all three axes were warning
  and one of them was a single line from refusing the tree's next closure.
  **The accreting section is `## Current Frontier`, and that is not where the remedy was pointed.**
  Attributed across all 19 post-migration revisions — `.30`'s migration left the root at 146 lines on
  `2026-09-16`, and it was 229 on `2026-09-19` — the +83 lines decompose as **Current Frontier +39**,
  Executable Owner Registry +18, Verification Log +13, Commit Log +13, Current Program State +0. On the
  binding axis the frontier section is the largest single writer by a factor of two, and it is the only one
  of the four whose growth is avoidable: a frontier describes the next step, and this one had accumulated a
  paragraph per closure. On the byte axis the order inverts — the two log windows wrote **+7,007 of the
  +11,656** — which is why all three sections are treated here rather than only the one the header named.
  **This is `.20`'s rule 3 arriving at the root that wrote it.** `.19` and `.3` each split a book chapter by
  reader concern and each bought about six slices, because the section holding 78.8% of the chapter stayed
  where it was; `.20` turned that into a rule — when a surface must be cut, move the section that is
  ACCRETING. Applied here the cut is not between the tree's topics, it is between its frontier and its
  chronology, and the root keeps neither log's rows.
  **The exposure `.20` published was under-counted, and the oracle is why.** `.20` recorded that seven
  leaves closed after the seal with no row in the sealed part. Re-derived here per row: the root's
  Verification Log holds **21 rows, of which 14 exist nowhere else**, and its Commit Log holds **19 rows,
  of which 14 exist nowhere else** — `.31`, `.32`, `.33`, `.36b`, `.36b.1` and `.36c` were missing from the
  published list, and they are the six most recent closures. Byte-presence alone clears only 1 verification
  row and 5 commit rows; the remaining 7 verification rows are condensed renderings the `.30` migration made
  for the bounded root, and each was cleared by testing that its content-bearing tokens are a subset of the
  sealed row for the same date and leaf. **That test refused one row a reading had passed**: the second
  `.30` row states that three closed leaves still declared `` `pending` ``, a token absent from the sealed
  `.30` row, so it moved instead of being dropped. Seven is the count of rows it is safe to delete, not the
  count of records at risk.
  **What it costs, stated rather than left to be discovered.** The part collection goes from 12 files to 13
  against a 16-file health target — **81.25%, a warning** — with 11 parts of headroom to the 24-file portable
  ceiling and collection totals at 44.0% of bytes and 42.6% of lines. That is the second instance of the axis
  `.14e` already owns, and it corroborates `.14e`'s framing rather than adding a new problem: a file count is
  the wrong instrument for a collection whose parts are semantic rather than sized, and the honest remedy is
  to re-derive the axis, not to raise it or to bury an accreting log inside a part that owns something else.
  Verification: `2026-09-20` row in this part's Verification Log
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.37 — give the post-seal chronology a writable home, then bound the root`
  Prerequisite: `.20`'s correction at `daed3b34`; the root could not accept this node before it
