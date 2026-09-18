# LIVE-DOCUMENT-PRESSURE-HEADROOM — current and open work

- Part ID: `current-and-open-work`
- State: `active`

## Post-migration work

Declared after the `2026-09-16` containment migration. These nodes live outside every marked legacy
region, which is what the active part is for; the legacy payloads above are immutable.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.30a`
  Status: `done` (`2026-09-16`; opened the same day by `.30`)
  Goal: let a migrated tree close a leaf its own migration sealed as open
  Acceptance: a leaf whose `Status:` line sits inside a marked legacy region can be closed by a
  post-migration declaration outside the markers, and the route catalog's lifecycle re-derives from that
  declaration; two declarations in the same stratum remain a breach. Found by `.30` on the first attempt to
  close a leaf after its own migration.
  **Measured `2026-09-16` at `033e7b66`, and the number is why this is a stop rather than an inconvenience.**
  Every marked payload is byte-exact against the archived capsule and immutable by contract, so a sealed
  `Status:` can never change; `validate_route_lifecycles` then refused the only writable alternative —
  re-declaring the leaf outside the markers — as `is declared 2 times in its primary part`. Censused across
  the four migrated contracts: **22 sealed-open leaves cannot be closed in place**, 19 of them in this tree,
  plus `CLAIM-VERIFICATION-ADOPTION.16` and `SPEC-TO-INTENT-ALIGNMENT`/`.9`. `corpus-coverage` has none only
  because it has no open leaf. That is a bound ordinary work reaches with no legal move — the exact
  `LIVE-DOC-STOP-RISK` condition — and `.30` had just multiplied it by nineteen.
  **The rule, stated as two strata rather than as an exception.** Everything inside a marked region is
  PRE-MIGRATION history; everything outside is post-migration current state. A leaf may therefore carry one
  declaration in each, and the post-migration one is authoritative. Two declarations in the SAME stratum are
  still refused, because that is a contradiction rather than a supersession — the diagnostic now names which
  stratum it counted in, so the message says what was actually wrong.
  **Why this is a checker change and not a convention.** The alternative was to re-point a closed leaf's
  route at some other part so the cross-check would look elsewhere, which would leave the route catalog and
  the detail record disagreeing about where a leaf lives — a lie by placement that no gate would catch. The
  contract already distinguishes the two strata mechanically through its markers; the checker simply was not
  reading them.
  Verification: `2026-09-16` row in the root Verification Log; the chronology part is sealed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.30a — read a migrated part as two strata so a sealed leaf can close`
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.29c`
  Status: `done` (`2026-09-16`)
  Goal: decide how many live-document surfaces this repository may have
  **There is nothing to decide. `.29b`'s warning arm compared two bounds without asking whether the state
  that makes them disagree is reachable, and it is not** — this record supersedes the sealed `.29b` and
  `.29c` declarations above, which both overstate it.
  **Re-derived on the director's challenge (`2026-09-16`, at `3f727a26`).** The census refuses at **91**
  current surfaces. That needs the archive/frozen population to be **4 or fewer**. It is **18**, and over all
  **323 revisions** of `surfaces.jsonl` it has **never decreased** — 1 -> 18 monotonically. The floor is
  mechanically monotonic, not merely historically so: an `archive_terminal` file is immutable under the
  rollover doctrine and `LIVE-DOC-SIZE` refuses any tracked Markdown no surface classifies, so an archive
  record cannot be reclaimed; and every migration ADDS one (`.21` +1, `.30` +1, measured). So
  `surfaces.jsonl` refuses first, at 95 records = **77** current, where the census holds **195 of 224 =
  87.1%** — its warning band, never its stop.
  **So the defect was mine and it was one line.** `$ceiling_current` read `$declared - 1` where the reachable
  ceiling is `$declared - 1 - $archive`. The arm now reads *at the surface registry ceiling of 96 records
  less 18 archive/frozen, 77 current surfaces are reachable; mirroring them needs 195 census records of 224*
  — a true forward statement about a state the plane can actually reach, which is what this whole tree means
  by pressure. The ERROR arm was always sound and is unchanged: today plus one measured partition event must
  fit.
  **The control that was missing is the reason it shipped.** The census self-test loop could only inspect
  ERRORS, so an arm that warns wrongly rather than failing wrongly was ungatable by construction. A case may
  now pin the WARNING text, and the new control pins the fixture's `ceiling of 16 records less 2
  archive/frozen, 13 current surfaces`; removing the subtraction makes it read 15 and the case fails. Suite
  **28 -> 29**.
  **Two published numbers corrected while re-deriving, both sealed and both superseded here.** `.30`'s record
  calls `EXTRACTION-QUALITY-GAUGE.md` *an active tree with fifteen open leaves*; re-derived it is **nine**
  (5 `active`, 4 `pending`) — the figure was carried from the roadmap's phrasing instead of derived. And the
  first parser written to check it returned **zero**, because that tree writes `- ID: \`x\` · Status: \`y\``
  inline on one line while every other tree puts `Status:` on the next — the same non-uniform node grammar
  `OWNERSHIP-CITATIONS` already recorded for `Status: **\`active\`**`. A census keyed on one node shape is a
  census that silently reads zero.
  Verification: `2026-09-16` row in the root Verification Log; the chronology part is sealed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.29c — the mirror warning compared bounds, not reachable states`
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.30b`
  Status: `done` (`2026-09-17`; opened by the first leaf closure attempted after `.30`)
  Goal: give this tree's writable stratum back a budget an ordinary leaf closure fits in
  Acceptance: the active part's post-migration stratum has room for more than one closure record, measured,
  with every sealed region byte-identical and no bound moved
  **Found by closing one leaf, and the number is the whole finding.** `.30` partitioned this tree on
  `2026-09-16` and left `current-and-open-work.md` at **56,985 of a 65,536-byte health target = 87.0%**. But
  **76% of that file is sealed** — two immutable marked regions of 21,941 and 20,837 bytes — so the budget the
  tree can actually spend is `58,982 - 43,214` = **15,768 bytes**, and the five post-migration records written
  that same day already held **13,771 of them, 87.3%**. The remaining **1,997 bytes** is less than one record:
  the stratum's five average **3,487** and `.28`'s closing record is **3,667**. Writing it took the part to
  **92.5%** and the writer refused with `semantic part collection bytes_each is at or above mandatory
  rollover`, which is how this was found rather than by a warning — the aggregate reported **87.0%**, and 87%
  of a budget three quarters of which is dead history is not the same measurement.
  **The remedy is shape, not number.** The sealed `frontier-decisions-and-questions` region moves byte-for-byte
  into its own `legacy` part, `frontier-and-decisions.md`. `current-and-open-work.md` is **56,985 -> 35,980
  bytes (87.0% -> 54.9%)** and its writable budget goes **15,768 -> 36,773** bytes, **87.3% -> 37.4%** used —
  room for **six** more closure records instead of none. The new part sits at **21,122 (32.2%)** and is
  entirely sealed. Parts go **10 -> 11 of a 16-file health target (68.8%)**, no ceiling moved, and the surface
  maximum relocates to `verification-and-chronology.md` at **47,654 (72.7%)**, below its warning.
  **One rule was worth finding and is not a stop.** `current_frontier.mode: eligible` requires the frontier
  leaf's primary part to be `active`, and `.30` marked exactly one part so. Every remaining open leaf lives in
  a `legacy` part, so the frontier cannot simply be repointed — but marking that part `active` is a two-line
  move (the contract field and the part's own `- State:` literal, which sits outside every marked region), so
  the rule is *the part holding the leaf being worked is the active one*, not a bound. Stated here because the
  next frontier move needs it and nothing else records it.
  **State the relocation**: the sealed `toolbox-and-census-nodes` region, 21,941 bytes, still sits in the
  active part and is the same dead weight one region later. It does not bind today at 37.4% used, and the same
  move is available when it does.
  Verification: `2026-09-17` row in the root Verification Log; the chronology part is sealed
  Commit: shipped in `LIVE-DOCUMENT-PRESSURE-HEADROOM.28 — band the two fact-plane caps that refused without one`,
  which it unblocks; the two are one transaction because they share four claim registries
  Prerequisite: `.30a`; opened by `.28`'s closure, which it unblocks

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.30c`
  Status: `done` (`2026-09-17`)
  Goal: make the post-migration leaves' verification pointers resolve
  Acceptance: no closing record in the writable stratum cites evidence at a location that cannot hold it
  **Measured `2026-09-17` at `d806e9d1`.** Eight nodes in this tree write ``Verification: `<date>` row below``
  and all eight are in this one part. The **four sealed** ones (`.27`, `.29`, `.29a`, `.30`) still resolve:
  their rows are in `verification-and-chronology.md`, two matches each, so after `.30` split one document into
  parts the phrase became imprecise rather than false, and it is immutable in any case. The **four
  post-migration** ones — `.30a`, `.27a`, `.29b`, `.29c` — resolve to **nothing**: `grep` finds **0** rows for
  each in that part, because the chronology part is entirely sealed and a post-migration leaf cannot add one.
  Their rows went to the root's Verification Log instead, which is correct and which nothing said.
  **Why it survived four closures.** Each record was written by copying the shape of the record above it, and
  the shape was true before `.30`. No gate reads the phrase: `SECTION-ANCHORS` matches qualified references to
  a named file, and ``row below`` names no file, so this class of route is invisible to it — the same gap
  `.27` recorded for bare `§7.7` citations, one step further out.
  **Fixed by naming the location**: the four now read *row in the root Verification Log; the chronology part
  is sealed*, which is what `.28` and `.30b` already say. The sealed four are left exactly as they are and
  documented here rather than described as correct.
  Verification: `2026-09-17` row in the root Verification Log; the chronology part is sealed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.30c — make the post-migration verification pointers resolve`
  Prerequisite: `.30a`; found while closing `.28`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.31`
  Status: `done` (`2026-09-17`; opened the same day)
  Goal: partition `docs/tasks/EXTRACTION-QUALITY-GAUGE.md`, the file `.30` handed the
  `task_evidence.lines_each` maximum to
  Acceptance: the driving file returns under its rollover milestone through the partitioned
  `active_task_evidence` contract, losslessly, with every `- ID:` node re-declared in the bounded root
  **It is binding NOW, and a commit was blocked to prove it.** `.30` recorded this file at **2,878 of 3,000
  lines = 95.9%** and assigned the axis to its own tree. On `2026-09-17` an ordinary slice took it to
  **3,012** and `.githooks/pre-commit` refused: `live-document-size: surface 'task_evidence' exceeds
  lines_each ceiling: 3012 > 3000`. The slice landed only after its own new records were compacted to
  **2,998** — **two lines of headroom**.
  **The catch-22 is the finding, and it is a new shape for this tree.** At 2,998 the file cannot hold the
  ~12-line leaf that would own its own partition: **a surface can become too full to record the remedy that
  would fix it.** That is why this leaf lives here and not in `EXTRACTION-QUALITY-GAUGE.md`, and it does not
  contradict `.30`'s *"that tree owns its own axis"* — the pressure tree already owns the partition
  TRANSACTION twice (`.21` for `CLAIM-VERIFICATION-ADOPTION`, `.30` for its own evidence). What moves here is
  the record, because there is nowhere else to put it.
  **Compacting evidence to land a slice is a POLICY defect, not an author problem** (director, `2026-09-17`).
  `COMMIT-GATE-SINGLE-RUN.4` fixed the delivery half — the driver was discarding all **44** warnings from
  PASSING checks, including this file's own `99.9% — 2 below its 3000 ceiling`, so the stop arrived
  unannounced. This leaf fixes the remaining half: route the evidence, never delete it.
  **Carry `.30`'s hard-won ordering rule**: a migration seals every marked payload byte-exact and
  `validate_route_lifecycles` refuses a leaf declared twice in its primary part, so **a leaf cannot close
  itself inside the transaction it seals**. This leaf must be complete before the transaction runs.
  **Seams, from the file's own structure rather than by size**: the `.2*` prototype/pivot program, the
  `.3e`/`.3g`/`.3h` positional-gate family, the large `.3k*` dedup/span program, the `.3j*` LLM-path family,
  and the root's CHI measurement plus fix-backlog narrative are natural reader concerns.
  **Closed `2026-09-17`, and the catch-22 was resolved by routing the record rather than deleting evidence.**
  Twelve semantic parts over fourteen contiguous regions covering lines 1-2,998 with no gap and no overlap,
  cut by reader concern: the founding gauge/CHI/prototype narrative, the original `.0`-`.8` backlog leaves,
  the field ontology, the deterministic precision gates `.3`-`.3i`, the `.3k` scoping, the untyped-default
  `.3k.2*` family, the `.3k` successors, the LLM path `.3j*`, three checklist parts cut by the family each
  verifies, and the changelog. **Two parts own more than one region** — `original-backlog-leaves` holds
  171-228 AND 2,664-2,735, because `.0` and `.4` are original-backlog leaves stranded at the far end of an
  accreted file, which is exactly why no byte had to move to make the cut land.
  **Lossless, proved independently of the writer**: re-harvesting only the marker-delimited payloads out of
  the twelve part files and concatenating them in declared source order reproduces the committed
  pre-migration blob byte-for-byte at **268,250 bytes**; the archived capsule is that same blob; all **56**
  node declarations survive in the parts and all 56 are re-declared in the bounded root. The root is **132
  lines / 6,885 bytes**, from 2,998 lines — **99.9% of the line ceiling to 4.4%**.
  **`max_unverified_routes` is 0, which is the whole point of `.32`.** All 56 lifecycles re-derive from the
  nodes' own status lines — 13 open, 43 closed. Under the old reader this contract would have had to declare
  **56**, and its landing would have published fourteen open-leaf claims that nothing checked.
  **The preflight earned its place again.** Deriving each node's lifecycle found `.3j` still declaring
  `pending` while its own block ends *"Done `2026-09-17` … the answer is NO"* and carries its closure commit
  — corrected and committed BEFORE the source was locked, because a migration publishes every lifecycle as
  derived fact. The same census flagged `.3k` and `.3k` is CORRECT: its `Commit:` line records the scoping
  commit, and `.3k.2`/`.3k.9` are still open. One rule, two hits, one real.
  **State the relocation; the surface is not released.** `task_evidence` `lines_each` moves to
  `WIRE-BASED-100.md` at **2,275 of 3,000 (75.8%)** and `bytes_each` to the same file at **208,621 of
  278,528 (74.9%)** — the first time in this tree's record that the surface warns on NEITHER dimension.
  The active part is `llm-path-family` at **18.2%** of its byte budget, chosen so the next leaf has
  somewhere to close: `.30b`'s lesson applied at cut time rather than after a refusal.
  **Registration, priced rather than absorbed**: four surfaces join `surfaces.jsonl` (73 of 96 records), the
  contract joins `scripts/check_task_evidence_contracts.sh`, and the census gains six records with
  `expected_current_surfaces` 50 -> 53 — `.29b`'s two-records-per-surface price, paid in full. The two
  `ownership_citations` records naming this tree cite the UNIT, not a moved path, so nothing repathed.
  Verification: `2026-09-17` row in the root Verification Log; the chronology part is sealed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.31 — partition the file that became too full to hold its own remedy`
  Prerequisite: `.32`; relocated onto this file by `.30`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.32`
  Status: `done` (`2026-09-17`; opened the same day by `.31`'s preflight)
  Goal: let the route-catalog lifecycle gate read the node declaration shape a third of this repository is
  actually written in
  Acceptance: the lifecycle a landing claims is re-derived from the owner's own status line for a tree
  written in either node shape, the indented shape reads exactly as before, and every registered contract
  stays valid
  **Found by preflighting `.31`, and the number is the finding.** `validate_route_lifecycles` re-derives
  each route's `open`/`closed` claim from its primary part's own `- ID:` block, and `declared_node_statuses`
  only ever matched a status on the CONTINUATION lines beneath the id. This repository writes node status in
  two interchangeable shapes, and the other one states it inline on the id's own line: **466 of 1,362 node
  declarations under `docs/tasks/`, 34%**. For every one of those the reader returns no status, which the
  gate counts as *uncorroborated* rather than as *wrong* — so the bound that absorbs it is
  `max_unverified_routes`, and a tree written entirely in the inline shape satisfies this gate with **zero**
  lifecycles re-derived.
  **`EXTRACTION-QUALITY-GAUGE` is exactly that tree, which is how this was found.** Measured over its 56
  declared nodes: the old reader corroborates **0 of 56**, the widened one **56 of 56**, separating 14 open
  leaves from 42 closed. Without this, `.31` would have had to declare `max_unverified_routes: 56` and ship
  a landing whose every open-leaf claim is an assertion — the shape `CLAIM_VERIFICATION.md` exists to refuse.
  **Widened, not replaced, and the separator is deliberately not part of the grammar.** The reader takes the
  first backticked `Status:`/`State:` on the id's own line and falls back to the indented lines when that
  line states none. It never matches the separator, so it does not depend on one punctuation choice, and a
  node carrying both is read from its own line as the more specific declaration.
  **Behaviour-preserving where it already worked, proved rather than argued.** All three migrated sharded
  trees are written in the indented shape, so the widening moves nothing: uncorroborated routes are 0 before
  and after for `claim_verification` and `pressure_headroom`, and 1 for `spec_to_intent`; all five
  registered contracts stay valid on the real repository. The one non-node line the widened id match now
  reaches — ``- ID: `LITERATURE-GROUNDING.4`–`.12` ``, a RANGE rather than a declaration — states no status
  and is uncorroborated under both readers.
  **The suite is the oracle and it is RED without the change.** Two end-to-end cases append a post-migration
  node in the inline shape and assert the positive and the lifecycle-disagreement refusal; three direct
  assertions pin the inline read, the annotated-id-with-indented-status read, and the refusal to INVENT a
  status when a node declares none. Re-run against a copy with only the reader reverted and everything else
  byte-identical, the suite fails at `post-migration inline-declared append positive` with *3 leaf routes
  declare a lifecycle their primary part does not corroborate*. Suite **64 -> 69**.
  **Book lockstep**: `docs/book/src/reference/live-docs.md` said a leaf whose part declares no node cannot be
  checked and is capped by `max_unverified_routes`. That was incomplete in the way that mattered — a node
  whose status the reader could not PARSE arrived at the gate as one that stated none — so the chapter now
  shows both declaration shapes and carries the measured consequence. +19 lines / 1,274 bytes, re-derived
  through the `shipped_behavior` `aggregate_change` authority, which the hook caught because the book edit
  came AFTER this slice's `--only LIVE-DOC-SIZE` run.
  Verification: `2026-09-17` row in the root Verification Log; the chronology part is sealed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.32 — read the node status shape a third of the trees are written in`
  Prerequisite: none; `.31` is blocked on it

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.33`
  Status: `done` (`2026-09-17`; opened the same day by `.31`'s closure, which is what pushed it over)
  Goal: execute the relocation `.30b` measured and recorded but left unowned — move the sealed
  `toolbox-and-census-nodes` region out of this tree's active part
  Acceptance: the active part's writable stratum has room for more than one further closure record, measured
  the `.30b` way (against the WRITABLE budget, not the file), with every sealed region byte-identical and no
  bound moved
  **`.30b` predicted this exactly and said the move was available "when it does" bind. It did.** Closing
  `.32` and `.31` took `current-and-open-work.md` from 47,642 to **54,992 of a 65,536-byte health target**,
  and the gate said so out loud because `COMMIT-GATE-SINGLE-RUN.4` made passing checks speak. Measured the
  `.30b` way rather than by the file: **21,941** of those bytes were the immutable region, so the spendable
  budget was `58,982 - 21,941` = **37,041**, of which the post-migration records held **35,250 — 95.2%**,
  past the writable stratum's own rollover while the aggregate still read 87.5%.
  **THE FINDING, and it is a new rule for this contract: a sealed region cannot move out of an active part
  alone.** `.30a` reads a leaf's two strata inside ONE part file, and **three leaves — `.27a`, `.29b` and
  `.28` — are declared `pending` inside the sealed payload and `done` outside it.** Moving only the region
  puts each leaf's sealed declaration in one file and its supersession in another; the lifecycle then
  re-derives from the sealed stratum alone and the gate refuses the `closed` route it just proved correct.
  `.30b` never met this because the region it moved superseded nothing. **The three closures therefore travel
  WITH the region**, and the new part carries them under its own `## Post-migration closures` heading —
  which also makes the part what its name says: the complete record of one concern, not half of it.
  **Result, and the byte win is nearly double what moving the region alone would have bought.** The active
  part is **24,144 bytes, 266 lines**, and it now owns **no region at all**, so its writable budget is the
  whole thing: **37,041 -> 58,982 bytes, and 95.2% used -> 40.9%** — room for roughly nine more closure
  records where there was not one. The new `toolbox-and-census` part is **34,447 bytes (52.6%)**, sealed
  payload byte-identical and re-verified against the capsule by the gate.
  **Correct the estimate this leaf opened with**: parts go **11 -> 12** of a 16-file health target (75%), not
  12 -> 13. No ceiling moved and no health target was raised; `.22`'s authority protocol was not invoked
  because nothing here asked for one. The part collection now warns on **nothing**: `bytes_each` is 47,654
  (72.7%, and the maximum relocates to `verification-and-chronology`), `lines_each` 528 (68.8%), `files` 75%.
  Verification: `2026-09-17` row in the root Verification Log; the chronology part is sealed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.33 — a sealed region cannot move out of an active part alone`
  Prerequisite: none; opened by `.31`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34`
  Status: `pending` (opened `2026-09-17` by `.33`, whose own book edit crossed the milestone)
  Goal: own the SIZE of `docs/book/src/reference/live-docs.md`, which is now the `shipped_behavior` byte
  maximum and past its rollover, and which no leaf owns — `BOOK-METHOD-DOC`, `BOOK-BEHAVIOUR-CURRENCY` and
  `BOOK-USER-FRIENDLY-BACKFILL` own its CONTENT and routing, and none of them owns a byte
  Acceptance: the chapter returns below its warning band by a routing decision — which narrative belongs to
  the containment chapter and which to the surface it describes — not by deleting evidence and not by raising
  a ceiling
  **Measured `2026-09-17`**: `live-docs.md` is **118,277 of a 131,072-byte health target = 90.2%**, at or
  above rollover, with **12,795 bytes** left against a ceiling that is the same number — health and
  enforcement at one value, so the warning band IS the only room. It is the largest book file by a wide
  margin: `pipeline/evidenceir.md` is 100,885 and `pipeline/isf-adapter.md` 89,553.
  **The mechanism is `.20`'s, one chapter over, and that is what makes it structural rather than bad luck.**
  `.20` records that every extraction rule lands in the STAGE chapter by default, forcing two byte-identical
  re-splits in six slices. The same default applies here: every time the containment doctrine learns
  something, the lesson lands in the chapter that describes containment. It happened **three times today
  alone** — the inline node shape (`.32`), the partition adoptions (`.31`), and the two-stratum scope
  (`.33`) — for **+3,289 bytes**, and each addition was correct where it was put.
  **Do not simply split it.** `.20` exists precisely because splitting a chapter that everything lands in by
  default buys six slices of relief and then repeats; the deliverable is a routing rule cheap enough to
  follow at authoring time, and `.20` should probably absorb this chapter rather than be duplicated for it.
  Read `.20` first and decide whether this is its second instance or a separate concern.
  Verification: pending
  Commit: pending
  Prerequisite: none; opened by `.33`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.32a`
  Status: `done` (`2026-09-17`; opened by the director's audit of `.32`'s published findings)
  Goal: correct a published ratio whose denominator this programme's own work moves
  Acceptance: every number `.32` published either re-derives by its own `reverify` command or is stated as a
  dated observation with the mechanism that moves it named
  **The decision holds; two published numbers did not** — the same shape as
  `EXTRACTION-QUALITY-GAUGE.3j`'s correction, and found the same way, by being asked to re-derive rather than
  restate. `.32`'s MECHANISM is untouched: the reader could not see an inline node status and now can, and
  `EXTRACTION-QUALITY-GAUGE` still goes **0 of 56 -> 56 of 56**, which is the number the decision rested on.
  **First number.** `.32` published **466 inline of 1,362, 34%**, and its fact card shipped a `reverify`
  command that re-derives **467 and 959** on the same day — a staleness check that contradicts the card it
  is attached to, which is worse than shipping no check at all.
  **The cause is self-reference, and that is the durable part.** A partition RE-DECLARES every node of the
  tree it cuts in the bounded root, in the OWN-LINE shape. Measured: **253 of the 959 own-line declarations
  exist only because a tree was partitioned**, and **`.31` added 56 of them in the same session** in which
  `.32` published the share. This programme inflates its own denominator, so a raw share of it can never be
  a current number. The card and the book now say *about a third*, name the mechanism, and carry a
  `reverify` that RE-DERIVES the classification instead of comparing it to a frozen pair.
  **Second number.** `.32` said the tree splits **14 open / 42 closed**. That was true when `.32` measured
  it and false one commit later: `.31`'s preflight corrected `.3j` from `pending` to `done` before locking
  the source, so it is **13 open / 43 closed**. The card said 14/42 while carrying `status: current`; both
  surfaces now say 13/43.
  **One more claim was audited and tightened rather than withdrawn.** `.34` opened saying `live-docs.md`
  *"nothing has ever owned it"*. Its SIZE claim is exact — 118,277 of 131,072 bytes, 90.2%, health and
  enforcement at the same value — but three `BOOK-*` trees own that chapter's CONTENT and routing. The leaf
  now says no leaf owns a BYTE of it, which is the claim the measurement supports.
  Verification: the census re-derived by a second, independent method (classify every `- ID:` line rather
  than two greps); the capsule re-read for the 0-of-56 pair; `git show cf5348c8^` for the `.3j` status; the
  `BOOK-*` references read individually before the ownership claim was narrowed
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.32a — a share this programme moves cannot be published as current`
  Prerequisite: `.32`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.35`
  Status: `done` (`2026-09-17`; opened by the director: *"it is just to designate the next action, so why so
  many lines?"*)
  Goal: return `MEMORY.md` to the layer-A contract it already had, instead of banding the cap that was
  holding it there
  Acceptance: the pointer carries the four resume fields and nothing a session reaches before it; the
  ~50-line cap stops being a bound ordinary work has to fight
  **The cap was never the problem, and the leaf that proposed banding it was wrong.** This session hit the
  50-line ceiling on five consecutive updates and concluded the surface needed a warning band, the same
  remedy `.28` applied to the fact plane. That reading was backwards. `MEMORY_ARCHITECTURE.md` §6 states the
  cap as *"roughly one screen"* with the explicit rule that **"if it exceeds either cap, information is in
  the wrong layer; move it down to B or C"** — and its template is FIVE bullets. The ceiling was doing its
  job; the content was the breach.
  **Measured regression, from the tree that set the cap.** `MEMORY-RESUME-POINTER-BYTE-CAP.1` recorded the
  pointer at **2,610 bytes / 32 lines** on `2026-08-15`, and its own decision says the byte ceiling is *"not
  permission to accumulate chronology"*. It stood at **~6,000 bytes / 49 lines** before this leaf: the
  pointer had nearly doubled and was being treated as a budget to fill.
  **What it was carrying, and where each thing already lived.** The `--fast`/`--only` rule and the push
  cadence: `COMMIT.md` (5 and 2 mentions) and `DOCTRINE_ENFORCEMENT.md`. The claim re-pin chain: `COMMIT.md`'s
  routing contract. Corpus currency 27/27 and the 144/9 boundary counts: mechanically gated every commit by
  `CORPUS-FRONTIER` and `PRODUCTION-GENERICITY`, so restating them could only ever go stale. Eleven `[[card]]`
  hazard links: the Knowledge Map, which `AGENTS.md` step 7 sends a session to *before* this file. Per-tree
  frontier notes for `SIGNAL-DECLARATION-ROW-DROP` and `TEXT-LAYER-IDENTIFIER-SPLIT`: both trees' own
  `## Current Frontier` sections, read before dropping and **materially richer than the summaries here** —
  the pointer was a lossy shadow of an authoritative row, which §6 forbids by name.
  **Result: 49 lines / ~6,000 bytes -> 21 lines / 1,645 bytes**, below the `2026-08-15` reading, with the
  fixed preamble at 9 of its derived 12-line share. Nothing was deleted; it was already in a layer a session
  reads earlier. **The standing correction**: this pointer is a POINTER. If an update does not fit, the test
  is not "which line can I shorten" but "which layer does this belong to".
  **One line of that preamble is NOT prose, and the gate caught it.** The first rewrite dropped
  ``on read: revision from `git rev-parse HEAD` ``, which is the `field_marker` of the
  `active_resume_repository_revision` derived-state contract — the surface's own declaration that the
  revision is DERIVED rather than stored, paired with a `forbidden_storage_marker` of `latest_commit:`.
  `LIVE-DOC-SIZE` went red and `repin_claim_regions` refused the census pin that sits on that exact line.
  Restored byte-identical, so the pin MOVED rather than needing a decision. **The rule for the next trim**:
  the preamble is trimmable, but that one sentence is load-bearing — check
  `doctrine/live_document_size/derived_state_contracts.jsonl` before cutting any of it.
  Verification: `check_memory_architecture.sh` green on all invariants at 21 lines / 1,645 bytes / 10 fixed;
  `check_derived_state_contracts.pl` green on 14 field contracts; each dropped item located in its own layer
  first, quoted above
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.35 — the cap was right, the content was in the wrong layer`
  Prerequisite: none; supersedes this session's proposal to band the `active_resume` cap, which is withdrawn

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14d`
  Status: `done` (`2026-09-18`, SPLIT)
  Goal: split the extraction-quality tree's active semantic part before it can no longer hold the leaf that
  would own its own remedy
  **Taken at 80.3%, not at 90%, and that is the whole point.** `llm-path-family.md` reached **52,652 of a
  65,536-byte health target** after five consecutive `EXTRACTION-QUALITY-GAUGE.3j.2*` closures. Mandatory
  rollover sits at 58,982, so the headroom was **6,330 bytes** against a measured leaf cost of 4,900-6,400 —
  one more closure would have landed at or past the stop. That is precisely the shape `.30b` measured and
  `.31` had to escape the hard way: *a file that became too full to hold its own remedy*. This split is the
  same remedy taken one leaf earlier, while it was still cheap.
  **The cut follows `.33`'s rule rather than the file's midpoint.** The sealed region cannot move alone: a
  legacy or structural route must follow its payload, so the closures of leaves the migration sealed while
  they were still open — `.3j.1.a` and `.3j.2` — travel **with** the region into
  `llm-path-sealed.md` under its own `## Post-migration closures` heading. Leaving them behind would have
  put each leaf's sealed declaration in one file and the record closing it in another, and the lifecycle
  would re-derive from the sealed stratum alone. `.3j.1.b` is still open inside that region, so the new part
  is `active`, not `legacy` — it must be able to hold that closure when it lands.
  Result: `llm-path-family.md` **52,652 -> 27,209 bytes / 317 lines (41.5%)**, holding only the six leaves
  opened after the migration; `llm-path-sealed.md` **26,095 bytes / 309 lines (39.8%)**. Every pinned region
  digest is preserved — the region's `part_id` moved, its line range and digest did not — and no bound was
  moved.
  **The trade is stated rather than hidden.** Parts go 12 -> 13 of a 16-file health target, so the
  collection's `files` axis crosses its warning at **81.2%**. That converts an imminent hard stop on
  `bytes_each` into a distant soft one with **11 parts of headroom to the 24-file ceiling**, and the new
  warning is assigned an owner in the same slice rather than left standing: `.14e`.
  Acceptance: the extraction-quality contract validates with 13 parts, `bytes_each` returns under warning,
  every leaf route resolves to the part that now holds its payload, and the derived index, manifest and
  route catalog regenerate from the contract rather than by hand
  Prerequisite: none
  Verification: `perl scripts/check_active_task_evidence.pl --contract
  doctrine/live_document_size/extraction_quality_gauge_task_evidence.json --check` reports
  `migrated/complete contract is valid` with `semantic part collection bytes_each` no longer warning;
  `bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE` green
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14d — split the part before it could no longer hold its own remedy`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14e`
  Status: `pending`
  Goal: own the `files` axis warning `.14d` created on the extraction-quality part collection
  Acceptance: `extraction_quality_gauge` semantic parts return under the 80% `files` warning, or the axis is
  re-derived the way `.28` re-derived the fact plane's — by showing which bound is actually binding rather
  than by moving one. Measure first: 13 of a 16-file health target against a 24-file portable ceiling is 11
  parts of headroom, so unlike `bytes_each` this axis is **not** near a stop, and the honest outcome may be
  that the health target is the wrong instrument for a collection whose parts are semantic rather than sized.
  Do not raise the target to clear the warning; `.22`'s authority protocol applies if a bound must move
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14d`
  Verification: `pending`
  Commit: `pending`
