# LIVE-DOCUMENT-PRESSURE-HEADROOM — resume pointer, task index, and ownership

- Part ID: `pointer-index-and-ownership`
- State: `legacy`

<!-- pressure-headroom-task-source-region:resume-pointer-node:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.5`
  Status: `done` (`2026-08-28`)
  Goal: give the resume pointer a band it can live in
  Acceptance: `MEMORY.md` is the `active_resume` surface, and its `health_targets` and
  `enforcement_ceilings` are **identical** — 50 lines, 32,768 bytes, 160-byte lines. A surface whose health
  target equals its ceiling has no warning band: it reports "at or above rollover" from 45 lines onward and
  then simply refuses. Measured `2026-08-28` it is 46 of 50 lines (92.0%), and every slice this session had
  to hand-compress the pointer to stay inside it — three times, which is a workflow tax rather than a
  containment control. `MEMORY_ARCHITECTURE.md` requires this file to carry one active unit, current state,
  one next action, in-flight work, and blockers; that is five sections plus a fixed how-to-resume preamble
  of 18 lines, leaving roughly 27 lines for all five. Decide whether the preamble belongs in the bounded
  pointer at all — it is stable prose that never changes and could be routed to `MEMORY_ARCHITECTURE.md`,
  which would give the mutable half of the file twice its current room without moving a bound. Do not raise
  the ceiling to buy space that a routing change already provides
  Prerequisite: none; it blocks nothing today
  Decided and delivered (`2026-08-28`, owner-delegated, "it's your call but it has to be signoff"): the
  history settles it and authoring discipline does not. Across the **last 30 commits that touched
  `MEMORY.md` the preamble is 19 lines in every single one** — a constant, never varying — while the
  mutable block grew 17 -> 28 against the 31 lines that leaves, i.e. **90% of its real budget already
  spent**. The growth is in the half that is supposed to grow, so the remedy is routing, not tighter
  prose. Every route the preamble stated is already reached *before* the pointer is read: a harness reads
  `AGENTS.md` first and `MEMORY.md` is step 4, and all eight of its tokens resolve upstream
  (`git rev-parse HEAD`, the no-shadow rule, `DOCTRINE_ENFORCEMENT.md`, `docs/TASK_TREE.md`, `COMMIT.md`,
  ADR 0003, `check_doctrines.sh`, `KNOWLEDGE_MAP.md`) — verified token by token before deleting a line.
  Nothing unique was moved and nothing was lost; this is deduplication.
  Delivered: fixed region **19 -> 8 lines**, so the mutable budget goes **31 -> 42** (+35%) with no bound
  moved. The four fields `check_memory_architecture.sh` requires are untouched.
  Gated, because an ungoverned split just drifts back: the checker now derives
  `MEMORY_POINTER_LINE_CAP / MEMORY_POINTER_FIXED_SHARE_DIVISOR` = 50/4 = **12 lines** for everything
  above and including the `## Current state` marker, reports the remaining mutable room on every run, and
  names routing as the remedy rather than a bigger cap. The bound is derived from the existing cap, so it
  cannot go stale the way a carried literal does.
  Correction caught by the gate, not by me (`2026-08-28`): the token-presence check above was necessary
  and **not sufficient**. One routed line was also a *registered* anchor — the derived-state contract
  `active_resume_repository_revision` pinned the exact heading `## How to resume (any AI, any harness)`
  as its `field_marker`, and the current-claim census pinned an evidence region on the same line. Route
  resolution says nothing about registry pins, so both broke and `check_doctrines.sh` refused the commit.
  Repaired by repointing both at the surviving declaration rather than restoring a heading to satisfy a
  literal: the contract now anchors on ``on read: revision from `git rev-parse HEAD` `` — the declaration
  itself, which is what the contract exists to pin — and the census evidence moves to that line with its
  identity re-derived. Verified after: derived-state 14 contracts / 47 self-test checks green, census 39
  surfaces / 66 evidence units, zero unresolved. **The lesson is the general one:** before routing a line
  out of a governed surface, check the registries that pin it by exact literal, not only the routes it
  states.
  Acceptance: `the fixed region is measured and capped at a derived share of the pointer cap; the mutable marker is required; the four resume fields still validate; every routed line is proven to resolve upstream; a known-bad pointer is observed RED; every registry pin on a routed line is repointed at surviving content, not restored as a literal`
  Verification: `fixed region 19 -> 8 lines, mutable budget 31 -> 42; four RED/boundary cases observed — the exact pre-change pointer at HEAD fails at 19 > 12 (a control observed failing on real shipped content, not a fixture), a missing '## Current state' marker fails as "no overwritable resume signal", 13 lines fails and exactly 12 passes; check_memory_architecture.sh green after; the eight preamble tokens each verified present in AGENTS.md or MEMORY_ARCHITECTURE.md before removal`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.5 — stop the resume pointer spending its budget on prose that never changes`

<!-- pressure-headroom-task-source-region:resume-pointer-node:end -->

<!-- pressure-headroom-task-source-region:task-index-and-ownership-nodes:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14`
  Status: `active`
  Goal: carry the three rows `.7` assigned to itself, on three separate lifecycle transactions
  Children: `.14a`, `.14b`, `.14c`
  Acceptance: each surface `.7` orphaned returns under its own milestone by a remedy legal for that surface's
  lifecycle, and the three do not land as one migration — this tree's own Non-Goal forbids combining
  independent lifecycle remedies, and `.2` was split into `.2a`/`.2b`/`.2c` for exactly that reason. The
  measured split: `alignment_task_evidence_index` is a bounded snapshot whose size is a pure function of
  lifetime leaf count (`.14a`), `alignment_task_evidence_parts` is a partitioned canonical collection whose
  remedy is a further part split (`.14b`), and `rust_analysis` is a rolling ledger with a declared rollover
  (`.14c`). Only `.14a` blocks `SPEC-TO-INTENT-ALIGNMENT.9c`
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14a`
  Status: `done` (`2026-08-31`)
  Goal: shard the alignment task-evidence index by lifecycle, and record the general rule for the other two trees
  Acceptance: `alignment_task_evidence_index` comes back under its mandatory-rollover milestone by the remedy
  `.2c` proved — shard by lifecycle, not by alphabet, so the bound measures concurrent work in flight rather
  than project age — with the complete route set still resolving and no hand-edited member list; measured, 77
  of 83 route rows belong to closed lanes and 54 to lane `.6` alone, so the lifecycle cut is the one that
  frees the budget. The landing carries the open leaves and routes the complete catalog to derived route
  parts; the index is derive-and-diff generated, an unplanned route part is refused, and each route's declared
  lifecycle is cross-checked against its primary part's own node status so the landing's claim is provable
  rather than asserted. **The general rule is recorded as a decision record in the same commit**, because all
  three task-evidence contracts share a `destinations` shape with a rollover route for the root and the parts
  and none for the index, whose size is a pure function of leaf count — `corpus-coverage` sits at 68% of the
  same un-routed bound and `pdf-variant-digestion` at 49.4%, so the third tree must inherit the answer rather
  than rediscover it, and `.10` consumes it rather than re-deriving it
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  **Two things the adoption found on its first run, and they are the reason the cross-check exists.** `.8` was
  `State: active` in `residual-actionability.md` while all four of its children were `done` and the bounded root
  records `.0`-`.8` complete; without the cross-check the new landing would have published as open a leaf the
  root publishes as closed. And `.9a` is routed by the index and declared in the root's owner registry but has
  **no node record in any part** — it is a section heading only, the single route whose lifecycle no evidence
  corroborates. `max_unverified_routes` is pinned at `1` so the population cannot grow; `.16` writes the record.
  **Adoption is staged, and the reason is evidential.** `pdf-variant-digestion` and `corpus-coverage` stay
  `inline`: their parts record leaves as prose with no `- ID:`/`State:` node blocks, so the cross-check has no
  authority there and declaring 52 and 56 lifecycles by hand would put an unverifiable claim on their landings.
  Their indexes are at 49.4% and 67.2% of the same un-routed bound, so the shape is the defect, not the schedule.
  Verification: `index 115 -> 43 lines (89.8% -> 33.6% of its 128-line health target, clear of the 90%
  mandatory-rollover milestone that refused the next leaf); routes 83 = 3 open / 80 closed, complete set in one
  93-line route catalog part (58.1% of 160, 1 of 6 files); check_active_task_evidence.pl --self-test 61/61 with
  eleven new RED cases (derived-index drift, closed leaf on the landing, route-catalog drift, unplanned route
  file, capacity below route count, health admitting no full part, missing/invalid lifecycle, ratchet breach,
  lifecycle disagreeing with its primary part, inline positives and inline field refusals) plus a --write
  round-trip and its preflight refusal; all three contracts --check green; live-size 900 Markdown files / 57
  governed surfaces with no alignment_task_evidence_index warning; claim census 41 current surfaces / 73 frozen
  evidence units; book claims 39 files / 325 adjudicated candidate lines; Knowledge Map 274 facts / 2,193 keys`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14a — shard the task-evidence index by lifecycle`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14b`
  Status: `pending`
  Goal: bring the alignment semantic-part collection back under warning
  Acceptance: `alignment_task_evidence_parts` lines_each (`behavioral-qualification.md`, 540 of a 640 health
  target) and the collection's `lines_total` return under warning by the split the contract already supports —
  a further semantic part, exactly as `residual-carrier` was split out of `residual-actionability` at that
  task boundary — with every pinned region digest and route preserved and no bound moved
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14a`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14c`
  Status: `pending`
  Goal: bring the Rust analysis ledger back under its record and line budgets
  Acceptance: `rust_analysis` lines_each returns under warning and the `rust-codebase-analysis` rolling ledger
  stops reporting 18 of 57 live records above its derived 1,872-byte budget, through the declared rollover
  transaction in `COMMIT.md` rather than a widened window; the record-budget half is the same finding `.11`
  carries for `development-notes`, so whichever lands first states the shared derivation
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.15`
  Status: `done`
  Goal: make "a cited owner is still open" a mechanical check instead of a review habit
  Acceptance: a gate refuses any ownership citation that names a `done` or `superseded` tree or leaf, over the
  assignment surfaces that carry them, with the owner's own `Status` line as the authority and a known-bad
  case observed RED before it is claimed green; the check states what it still permits — it can prove an owner
  is open, never that the open owner is the right one. **The population that justifies it is enumerated, not
  impressionistic:** five historical citations named closed trees (`DECISION-RECORD-CAPACITY-HEADROOM`,
  `FACT-CARD-CAPACITY-HEADROOM`, `FACT-CARD-CATALOG-CONTAINMENT`, `CORPUS-TASK-EVIDENCE-CONTAINMENT`,
  `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`), `CLAIM-VERIFICATION-ADOPTION.11` produced a sixth by screening
  with `grep -rl`, and `.7` produced a seventh by closing itself while holding three of its own rows — the
  last under full attention, immediately after documenting the class. That is the evidence that review does
  not hold this invariant and a gate must.
  **An eighth instance, on a surface `.7`'s assignment never screened (`2026-08-31`).** `.7` bound the
  live-size gate's 39 warned rows; nobody screened `ROADMAP.md`, whose "five active program groups" list
  named **two closed trees** as current owners of the repository-durability group — `LIVE-DOC-STOP-RISK`
  (`done`) for live-document containment and `CORPUS-CHAIN-CURRENCY` (`done`) for artifact currency — while
  `LIVE-DOCUMENT-PRESSURE-HEADROOM`, the tree actually holding that work and every warned-surface
  assignment, appeared **zero** times in the roadmap. The citation is corrected in this commit; the gate this
  leaf builds must therefore cover `ROADMAP.md`'s ownership prose, not only the live-size assignment
  surfaces, and must distinguish a *historical attribution* (a `Done` workstream row naming the tree that
  finished it, which is correct) from a *current-owner* citation. Found by re-reading the roadmap to check a
  claim, which is the point: the class is invisible to every screen that does not read the owner's `Status`
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  **Built `2026-09-16` as the registered `OWNERSHIP-CITATIONS` doctrine.** The design question the leaf left
  open — how a machine tells a current-owner citation from a historical attribution — was settled by
  MEASUREMENT rather than by a rule of thumb. `ROADMAP.md`'s generated workstream table holds **23**
  citations of which **18** name closed trees, and every one is correct, because a `Done` row names the tree
  that finished the work. So "in prose, not in the table" is not the discriminator either: the two remaining
  closed citations in the prose (`SWD-SERIAL-EXTRACTION`, `LIVE-DOC-STOP-RISK`) are ALSO correct history,
  sitting inside the very bullet list that carries the current owners. **No structural feature separates the
  two kinds, so the classification has to be declared** — and the gate's value is then the COMPLETENESS leg,
  not the classification: every citation inside a declared region must have a record, so a new one added with
  none fails closed. That is exactly how the eighth instance entered unseen.
  **Shape.** `doctrine/ownership_citations/citations.jsonl` declares the regions and one record per cited
  unit (`current_owner`, or `historical` with a required reason).
  `scripts/check_ownership_citations.pl` reads each declared region between
  `<!-- current_owners:start -->` and its `:end`, extracts every citation — a `docs/tasks/<TREE>.md` link OR
  a backticked work-unit id whose tree file exists — and refuses an unclassified citation, a stale record,
  and any `current_owner` whose own `Status:` is `done`/`superseded` or missing. It covers **27** citations
  across **2** regions: **18** current owners, **9** historical.
  **The first run found a real instance of its own class, in the grammar rather than the data.** One tree in
  167 writes `- Status: **`active`**` with emphasis markers, and a checker that only accepted the bare
  backticked form reported it as a non-existent work unit — an invitation to edit a tree to satisfy a regex.
  The emphasis is part of the real grammar; the parser accepts it and three observed status shapes are
  pinned as self-test cases.
  **And it refused the prose describing it, which is the third self-reference trap this tree has recorded.**
  The leaf record you are reading quotes the marker inside backticks while sitting in a declared region, so
  the first version counted that quotation as a second start marker and refused the very text explaining the
  mechanism — the same shape as `.7` orphaning the rows it had just assigned by closing itself. A document
  must be able to NAME its own mechanism, so only an UNQUOTED occurrence opens or closes a region, pinned by
  a case asserting that quoting the marker changes neither the count nor the extracted citations.
  **The markers had to be line-neutral, for the second time on this tree.** Added as their own lines they put
  `ROADMAP.md`'s `Current strategic priorities` section at **58 of its 56-line bound** and the projection
  gate refused them — an ordinary compliant edit refused by a section already sitting exactly at its bound.
  Markers are metadata, not content, so they are appended inline to existing lines instead: the file stays
  **189 lines** and all **574** claim regions re-pin **unchanged**, the property `.4d.i` had to preserve for
  README.
  Verification: `three RED controls observed on the REAL tree, each with its intended diagnostic - a closed tree declared current_owner ("its own Status is 'done'"), a new unclassified citation, and a stale record - with green restored after each; self-test 11/11 under an independently declared total; the population census showed 18 of 23 workstream-table citations correctly name closed trees, which is why that table is outside the declared region; live-size green at 993 files / 61 surfaces with line_bytes_each unchanged at 909; all 16 executed gate-tier doctrines PASS (18 registered)`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.15 — gate the closed-owner citation class instead of reviewing for it`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.16`
  Status: `pending`
  Goal: give `SPEC-TO-INTENT-ALIGNMENT.9a` the node record its routes already promise
  Acceptance: `.9a` is declared in the alignment root's owner registry and routed by the task-evidence index
  to `residual-actionability`, but no semantic part declares it as a node — it exists only as a section
  heading, so it is the one leaf route in the tree whose lifecycle cannot be cross-checked against its own
  evidence. Reconstruct the node record from tracked authority (the root's `.9` summary, the `.9a` section,
  and its commit rows), so the `.14a` cross-check covers every route with no declared exception. Found by
  `.14a` while deriving lifecycle from the parts; not fixed there because `residual-actionability.md` is at
  78.4% of its 640-line health target and the record would push the part into `.14b`'s warning band
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14b`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.17`
  Status: `pending`
  Goal: give a migrated task tree's leaf-route capacity a declared rollover
  Acceptance: `.14a` moved the alignment tree's nearest structural stop from the index's 115.2-line
  mandatory-rollover milestone to `limits.manifest.max_leaf_routes`, which `enforce_portable_caps` fixes at
  **128** against 83 declared routes — 45 leaves of headroom, in both bands, with no declared rollover: the
  `LIVE-DOC-STOP-RISK` shape. Decide from measurement whether the manifest's route array is the resource that
  needs bounding at all (the route catalog parts now carry the same membership under a `max_parts` x
  `routes_per_part` capacity with a declared remedy), and either give the bound a rollover route or remove it
  the way ADR 0045 removed the task-plane file count. State the relocation rather than claiming a removal
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.14a`
  Verification: `pending`
  Commit: `pending`

<!-- pressure-headroom-task-source-region:task-index-and-ownership-nodes:end -->
