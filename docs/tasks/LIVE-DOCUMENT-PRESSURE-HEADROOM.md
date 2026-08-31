# LIVE-DOCUMENT-PRESSURE-HEADROOM: keep current-facing canonical surfaces writable

## Metadata

- Tree ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
- Status: `active` (`.0`/`.5`/`.7`/`.2a`/`.2b`/`.2c`/`.14a` done; `.1`/`.3`/`.4`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18` pending)
- Roadmap lane: repository durability and portability
- Created: `2026-08-14`
- Last updated: `2026-08-31`
- Owner: repo-local workflow

## Goal

Restore actionable headroom in the non-rolling current-facing surfaces whose next ordinary update is near a
hard refusal, while distinguishing writable authorities from large immutable evidence and preserving every
canonical byte and route.

## Non-Goals

- Do not raise a bound, shrink evidence, or silence a warning merely to make the pressure report quiet.
- Do not rewrite accepted decisions, completed research, or historical task evidence in place.
- Do not combine independent lifecycle remedies into one migration transaction.
- Do not interrupt the active alignment task-evidence root-last transaction.

## Opening Pressure Boundary (`92e59c97`)

The composed live-size gate passes, but these non-rolling axes have little local room:

| Surface | Exact largest/current state | Registered target/ceiling | Remaining |
| --- | ---: | ---: | ---: |
| `knowledge_cards.lines_each` | `production-genericity-boundary.md` 299 | 300 | **1 line** |
| `task_evidence.files` | 144 root task Markdown files | 160 | **16 files** |
| `task_tree_index.lines_each` | `docs/TASK_TREE.md` 397 | 480 health / 512 ceiling | 83 / 115 lines |
| `shipped_behavior.bytes_each` | `pipeline/evidenceir.md` 118,004 | 131,072 | 13,068 bytes |
| `research_records.lines_each` | `production-genericity-pipeline-audit.md` 604 | 640 | 36 lines |
| `validation_snapshot.lines_each` | `VALIDATION_SNAPSHOT.md` 544 | 640 | 96 lines |
| `readme_entrypoint.line_bytes_each` | `README.md` 108 | 120 | 12 bytes |

The new ownership file deliberately consumes one task slot; after catalog regeneration the resulting task plane
is 145/160 files and the derived task index is 398 lines. That cost is explicit and buys one route for the
remedies instead of scattering unowned warnings across future product commits.

`decision_records` pressure is excluded because `DECISION-RECORD-CAPACITY-HEADROOM` already owns its independent
41/44 collection count and ADR 0038 member-shape axes. Rolling ledgers and the roadmap root have declared
repeatable rollover/remedy paths and remain under their existing owners.

## Acceptance Criteria

- Each pressure axis is classified by lifecycle, writer, expected growth, hard stop, and a remedy that is legal
  for that lifecycle before any authority moves.
- The 299-line current knowledge card is losslessly split or superseded through the fact-card protocol before a
  structural qualification fact needs to update it.
- Task-tree collection and derived-index capacity are redesigned together; adding an ownership task cannot move
  one axis while hiding the other.
- Maintained book content stays user-readable and current; immutable research/snapshot evidence is routed or
  partitioned only through an accepted lossless transaction.
- README policy and every canonical catalog remain exact; focused gates and full CI run in proportion to each
  leaf, and every slice commits through `COMMIT.md`.

## Task Tree

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM`
  Status: `active`
  Goal: keep non-rolling current-facing canonical surfaces writable without losing evidence
  Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`, `.6`, `.7`, `.8`, `.9`, `.10`, `.11`, `.12`, `.13`, `.14`, `.15`, `.16`, `.17`, `.18`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.0`
  Status: `done`
  Goal: own and pin the exact pressure frontier before any surface or bound changes
  Acceptance: the clean boundary, exact axes, existing owner split, and ordered remediation leaves are durable;
  no governed content, bound, product, decision, research record, snapshot, or book page changes
  Verification: `opening 92e59c97: knowledge card 299/300 lines; task plane 144/160 files and index 397/480 health lines; EvidenceIR book page 118,004/131,072 bytes; largest research 604/640 lines; validation snapshot 544/640 lines; README max line 108/120 bytes; ownership adds one task file and one derived catalog row only; resulting 145 files / 398 index lines; catalogs, retrieval, live-size, and doctrines pass`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.0 — own the current live-surface pressure frontier`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`
  Status: `pending`
  Goal: restore writable headroom for the 299-line production-genericity fact card
  Acceptance: current and immutable fact roles are separated losslessly through the existing catalog/map
  lifecycle; every answer route remains exact; no card bound moves; the next ordinary fact update succeeds
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2`
  Status: `active`
  Goal: re-derive task-tree collection and bounded catalog capacity as one profile
  Children: `.2a`, `.2b`, `.2c`
  Acceptance: exact growth, readers/writers, route cardinality, catalog shape, aggregate reachability, capacity,
  and boundary faults are measured and decided before changing either the file or index authority
  **Measured composition (`2026-08-29`, director question: why is there a cap at all, and can the structure
  make it stop mattering).** The pressured axis is `task_evidence.files` — the COUNT of root Markdown files
  matching `docs/tasks/*.md`, top level only; the three part-directories are separate surfaces with their own
  bounds. It is **151 of 160**, and both `health_targets.files` and `enforcement_ceilings.files` are 160, so
  like `.4`'s research axis there is no warning band. The per-file caps on the same surface (3,000 lines,
  278,528 bytes, 6,400 line-bytes) are NOT the pressure point.
  The decisive number is the composition, not the count: of those 151 roots, **120 are `done`, 5 are
  `superseded`, 24 are `active`, 1 is `proposed`**. So **83% of the live collection is finished work**, and the
  bound is currently measuring cumulative project history rather than actionable state. That is why it reads as
  an arbitrary ceiling — it is being spent on trees nothing will ever act on again.
  **Candidate design this leaf should decide against (not yet accepted).** Make the collection's membership
  SEMANTIC instead of cumulative: `docs/tasks/` holds what can still be acted on, and a root whose every leaf
  is `done`/`superseded` with a closed commit log is TERMINAL and retires. Retirement already exists and is
  already proven four times — `archive_terminal` plus `check_task_tree_archive.pl`'s `source_locked` ->
  `migrated` transition seals a byte-exact capsule under `docs/archive/tasks/` reachable from a bounded index —
  but it has only ever been applied to PARTS of one oversized tree, never to a whole finished root. Extending it
  to terminal roots converts the bound from a countdown against project lifetime into a statement about
  concurrent WIP, which is a signal worth having.
  Why NOT simply raise 160: this tree's own Non-Goals forbid raising a bound to quiet a warning, and doubling it
  only moves the countdown. The caps exist because every one of these surfaces is read by a fresh session with a
  bounded context; an unbounded `docs/tasks/` makes the frontier unfindable and pushes `docs/TASK_TREE.md` past
  its own 512-line ceiling. The cap is a proxy for "a fresh session can still find the frontier".
  Risk to design against, stated before building: retirement must not dangle a live pointer. Task trees
  cross-reference each other by leaf id, and claim records carry `retained_evidence` paths into
  `docs/tasks/*.md`. A terminal-root transaction therefore needs a route-rewrite step and a gate proving no live
  reference resolves into the archive by accident — the same obligation the parts migration already discharges,
  widened from parts to roots
  **Director's decision (`2026-08-29`), and the retirement recommendation above is WITHDRAWN as the primary
  remedy.** Directive: there is to be **no limit on the number of task-trees**, and a completed tree is project
  history that must be kept — at session start you read specific trees, never all of them. Measurement supports
  the directive on every axis, so this is not a bound raised to quiet a warning (this tree's Non-Goal), it is a
  bound removed because it protects nothing another axis does not already protect:
  1. **The number has no derivation.** `files: 160` was authored in `cb65d5c7` (`2026-08-08`), the bulk
     activation of the whole doctrine, when the plane held **122** files. It is "current plus headroom, rounded",
     never a task-plane-specific analysis.
  2. **An unbounded collection is already legal.** `shipped_behavior` — the mdBook, also `locator: collection` —
     declares `files: null` in both health and ceiling. So a file-count cap is a per-surface choice, not a
     doctrine requirement.
  3. **Nothing but the count is under load.** Measured `2026-08-29`: files **151/160 = 94.4%**, but
     `lines_total` **36,074/480,000 = 7.5%**, `bytes_total` **2,655,253/44,564,480 = 6.0%**, and the largest
     single file **1,287/3,000 lines = 42.9%**. The aggregate axes already bound the real resource — content —
     and they are near-empty.
  4. **There is no rollover declared for `task_evidence` at all**, so the cap is a bound the surface can reach
     with no remedy compliant work can take: the exact `LIVE-DOC-STOP-RISK` condition, same shape as `.4`'s
     research-records finding. Removing the cap dissolves that too.
  5. **The reader-facing limit is a different surface, and the directive names it.** Nobody reads
     `docs/tasks/` linearly; a session reads `docs/TASK_TREE.md` then opens the one tree it needs. That index is
     the thing with a real bounded-context cost, and it carries one row per tree: **404 of 512 lines, 158 rows**.
     So cardinality pressure belongs there, not on the directory.
  **Decided remedy, replacing retirement.** (a) Set `task_evidence.files` to `null` in both health and ceiling,
  with an authority record stating this rationale; leave every per-file and aggregate bound untouched, because
  those bound the resource that actually exists. (b) Give `docs/TASK_TREE.md` the sharding remedy this
  repository already proves twice — bounded landing plus parts, as the Knowledge Map does with 15 question
  shards and `fact_card_titles` with 6 title parts — so index cardinality stops being a ceiling and becomes a
  routing problem. Nothing is deleted and nothing is archived for capacity. Archiving stays available only for a
  single tree outgrowing its **per-file** bound, which is what produced the existing three (sealed at 2,393 /
  2,308 / 2,049 lines against the 3,000 cap).
  **Withdrawn by the director (`2026-08-29`): the "materialize a task-tree as a directory" thread.** It was
  raised about a different subject than the collection-cardinality bound this leaf owns, and its assessment is
  removed rather than left as off-topic weight on a leaf reserved for the cap discussion. Do not re-raise it
  here; if a single tree ever outgrows its per-file bound, that is the existing parts-and-archive remedy and it
  belongs to that tree, not to this one
  **Found while splitting the decision into remedies (`2026-08-29`): the cap has TWO enforcers, and the
  measurement above named only one.** `doctrine/live_document_size/surfaces.jsonl` carries
  `task_evidence.enforcement_ceilings.files = 160`, and `scripts/check_task_tree_catalog.pl:18` independently
  carries `my $MAX_TASKS = 160;` with its own refusal at line 85. The registry is therefore NOT the single
  authority for this bound, so setting the registry dimension to `null` alone would leave the plane still
  capped at 160 by a literal in a different enforcer — a change that reads as delivered and is not. Both move
  in the same transaction or neither does. The catalog checker's remaining real bounds are its
  `$MAX_SECTION_BYTES` (49,152) and `$MAX_ROW_BYTES` (512), which bound content rather than cardinality
  **Relocation, stated plainly, because "removed" would overstate it (`2026-08-29`).** Removing the file cap
  does not leave the plane unbounded in practice; it moves the binding stop to the derived index, which is the
  surface the directive itself names as the real reader-facing limit. Measured at `c1609558` from
  `scripts/check_live_document_size.pl --report`: `task_evidence` is 151 files with the generic gate reporting
  `files is at or above rollover (94.4%) — 9 below its 160 ceiling`, while `task_tree_index` is 404 lines with
  `lines_each is at or above warning (84.2%) — 108 below its 512 ceiling`. The index holds one catalog row per
  tree (150 rows over 162 catalog lines, inside a 22,561-of-49,152-byte section), and 242 of its 404 lines are
  fixed workflow prose, so its 512-line ceiling admits about 108 further trees. **So `.2a` moves the nearest
  stop from 9 trees to about 108 and does not abolish it.** That residual stop has no declared rollover, which
  is the `LIVE-DOC-STOP-RISK` condition, so `.2c` is not optional polish — it is the half of the remedy that
  makes the directive true rather than deferred
  Verification: `container — closes when .2a, .2b, and .2c close`
  Commit: `see child leaves`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`
  Status: `done` (`2026-08-29`)
  Goal: apply the no-cardinality-cap task-plane profile through a declared, gated exemption
  Acceptance: `task_evidence` declares no file-count bound in either band; the removal is a **declared
  exemption with conditions a checker enforces**, never a bare `null` any surface can adopt — the exempt
  surface must keep every per-file and aggregate dimension numeric, must null the count in both bands
  together, and must name a bounded reader-facing route that is a different registered surface covering its
  declared index; `scripts/check_task_tree_catalog.pl`'s independent `$MAX_TASKS` literal is removed in the
  same transaction so the plane is not still capped by a second enforcer; the change is authorized by one
  exact record in `doctrine/live_document_size/ceiling_increase_authorities.jsonl`; an ADR records the
  rationale; and RED controls prove an undeclared null, a half-declared null, an exemption that also unbounds
  the resource axes, and an exemption whose route is unregistered or unbounded are each refused
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2` measurement and director decision (met)
  **Delivered as a declared exemption, not a null.** `doctrine/live_document_size/surfaces.jsonl` gains a
  `cardinality_exemption` object on `task_evidence` naming `authority` (ADR 0045), `work_unit`,
  `route_surface_id`, and `rationale`; `scripts/check_live_document_size.pl` gains
  `validate_cardinality_exemption`, which refuses, as separate faults: a null `files` in either band with no
  exemption; an exemption that nulls only one band; an exemption whose surface also nulls any resource
  dimension (`lines_each`, `bytes_each`, `lines_total`, `bytes_total`, `line_bytes_each` all stay numeric); an
  exemption routed to itself, to an unregistered surface, to one that does not cover the surface's declared
  `index`, or to one unbounded in any dimension; and an exemption whose authority is not a repository file.
  `validate_limits` widens its null allowance to exactly these two cases — `maintained_reference` by lifecycle,
  or a declared exemption — so a bare null stays refused and cannot be adopted by copying a line.
  **Both enforcers moved in the same transaction.** `scripts/check_task_tree_catalog.pl`'s independent
  `my $MAX_TASKS = 160;` and its refusal are deleted, with a comment recording why the literal must not come
  back. Its `$MAX_SECTION_BYTES` (49,152) and `$MAX_ROW_BYTES` (512) stay: they bound content, not cardinality.
  **One authority, and it is single-use.** `ceiling_increase_authorities.jsonl` carries one exact record whose
  `old`/`new` match the committed and new bands exactly — the generic gate compares them by canonical encoding,
  so the null cannot be authorised loosely. `validate_authority_schema` now allows a null inside `new` only,
  because that exact-match comparison is what actually constrains it. `.2b` must retire the record.
  Verification: `check_live_document_size.pl` green (888 Markdown files / 55 surfaces) with the exemption
  active and the ceiling-history check accepting the authority; `check_task_tree_catalog.pl` green at 150
  trees with no count cap; `test_live_document_size.pl` gains eight cases — one positive and seven RED, the
  four the acceptance names plus route-to-self and untracked-authority; knowledge map regenerated for ADR 0045
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a — remove the task-plane cap through a declared exemption`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2b`
  Status: `done` (`2026-08-29`)
  Goal: retire the consumed ceiling-increase authority
  Acceptance: the single-use authority record `.2a` consumed is removed once HEAD already carries the new
  ceilings, so the registry cannot bank it; the generic gate's own "unused or banked ceiling-increase
  authority" refusal is the control that proves the retirement was required rather than cosmetic
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`
  **The control was observed RED before the retirement, which is the whole point of this leaf.** Immediately
  after `.2a` landed at `ef3b4bb4` — clean tree, nothing uncommitted —
  `perl scripts/check_live_document_size.pl` reported `surface 'task_evidence' has unused or banked
  ceiling-increase authority` and exited non-zero. So HEAD was gate-failing on a clean tree, by design: the
  authority is single-use, `.2a` could not land without it, and the registry refuses to keep it once the new
  bands are committed. That is the difference between a retirement that was required and one that is
  cosmetic, and it is why `.2b` is a separate leaf rather than a tidy-up inside `.2a`.
  Retired the one `increase` record for `task_evidence`; the registry is back to its control record alone.
  Verification: `RED before — "surface 'task_evidence' has unused or banked ceiling-increase authority",
  exit 1 on a clean tree at ef3b4bb4; GREEN after — 889 Markdown files satisfy 55 governed surfaces, exit 0;
  ceiling_increase_authorities.jsonl holds only its registry control record`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2b — retire the consumed ceiling-increase authority`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c`
  Status: `done` (`2026-08-29`)
  Goal: give the derived task index the sharding remedy so its cardinality stops being a stop
  Acceptance: `docs/TASK_TREE.md` becomes a bounded landing plus derived catalog parts, following the two
  remedies this repository already proves — the Knowledge Map's landing plus question shards and
  `fact_card_titles`' six title parts; the catalog stays derive-and-diff generated with no hand-edited member
  list; every existing route into the index still resolves; and the residual 108-tree stop `.2` measured is
  replaced by a bound that ordinary compliant work can pass without an authority edit
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a`
  **The shard is by lifecycle, not by alphabet, and that is the whole point.** Copying the Knowledge Map's
  quantum split would have relocated the stop again — a landing listing every tree grows with project
  lifetime whatever the quantum. The landing now carries every **open** tree (anything not `done` or
  `superseded`) and routes the complete catalog to derived parts under `docs/task-catalog/`. So the bound
  measures **concurrent work in flight**, which ordinary work reduces by finishing trees, instead of measuring
  how long the project has existed. Measured: the landing went **404 -> 295 lines** with 25 open trees, and
  150 trees route through 3 parts.
  Generation and proof follow the two existing remedies exactly: `--write` renders the landing section and
  every part, `--check` derive-and-diffs all of them and refuses an unplanned part file, and no member list is
  hand-edited. `$TREES_PER_PART` is 56 and `$MAX_PARTS` 16.
  **One checker rule had to change, and it was over-strict rather than wrong.** `routed_membership` required
  its index to live *inside* the surface, which is true for `docs/knowledge/INDEX.md` but false for
  `docs/TASK_TREE.md` — a task index is not a task tree. Where the landing lives and whether membership may
  take one hop are orthogonal, so a routed index may now sit outside its collection provided it is itself a
  classified surface; the landing therefore stays bounded by a registered surface rather than floating. The
  existing control that asserted the old location rule is rewritten to assert what actually protects the
  reader — an outside index still has to prove membership — and a new control covers the surviving refusal.
  **Parts bounds are derived from the generator's own structure, not copied.** A full part is 56 rows plus 11
  fixed lines = 67 lines, and a row is capped at 512 bytes, so the health targets are set such that a
  structurally full part sits *below* the 80% warning (84 lines, 704 line-bytes, 12 of 16 files). Copying
  `fact_card_titles`' 80-line target would have made a full part warn at 83.8% with no action available —
  the exact "bound with no compliant remedy" this tree exists to remove.
  **Found while implementing, not fixed here:** `external_membership`'s "is not a classified Markdown
  surface" refusal actually tests only that the path is a *tracked Markdown file* (`%path_seen`), not that any
  surface claims it. In a valid registry the two coincide, because unclassified Markdown is refused
  separately, so this is a weak control rather than a hole — but its message claims more than it checks. The
  new routed rule tests real classification; the older one is left for its owner. Tracked as
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.6`.
  **This slice falsified one of `.6a`'s own carried values, and that is worth more than the shard.** `.6a`
  carried `current_surfaces` **39** as stable across all 29 measured revisions and published it in
  `TOOLBOX.md`, the census fact card, and the `current-claim-census-frozen` assertion. Registering
  `task_tree_catalog_parts` moved it to **40** on the first structural slice after that measurement. A value
  stable across 29 revisions is not a constant — it is an unmoved one, and a trajectory can only ever show
  what has not happened yet. All three surfaces withdraw it to `--report` in this commit. Also re-pinned: the
  landing's own census evidence pinned `stdout_contains: "task-tree-catalog: 150 real task trees"`, a marker
  containing a tree count that adding one tree would break; it now pins the stable phrase instead.
  Verification: `check_task_tree_catalog.pl --self-test 13/13; --write then --check derive-and-diff green at
  150 trees across 3 parts with 25 open on the landing; check_live_document_size.pl green at 892 Markdown
  files / 56 surfaces with no warning on the new surface; test_live_document_size.pl 93 cases`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c — shard the task index by lifecycle, not by alphabet`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.6`
  Status: `pending` (tracking-only)
  Goal: make `external_membership`'s classification refusal test classification
  Acceptance: `scripts/check_live_document_size.pl` refuses an `external_membership` index that "is not a
  classified Markdown surface", but the test is `%path_seen` — which only proves the path is a tracked
  Markdown file. Any surface could point its external index at an unclassified tracked file and the message
  would not fire; the separate unclassified-Markdown refusal is what actually catches it today, so this is a
  weak control rather than an open hole. Make the test match the message the way
  `routed_membership` now does (a surface must actually match the path), and add a RED control that
  distinguishes the two — a tracked-but-unclassified index must be refused by *this* rule, named, rather than
  only by the coverage rule
  Prerequisite: none; found by `.2c` while widening the routed rule

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Status: `done` (`2026-08-31`)
  Goal: assign every gate-level live-document warning to an open leaf, by review rather than by grep
  Acceptance: this leaf was opened by `CLAIM-VERIFICATION-ADOPTION.11` and its premises were corrected by
  `.11a` the same day, because both the population and the ownership test it used were wrong.
  **Population.** `.11` measured `perl scripts/check_live_document_size.pl`; the doctrine `LIVE-DOC-SIZE` runs
  `scripts/check_live_document_size.sh`, which composes **four** producers — `live-document-size`,
  `active-task-evidence`, `rolling-ledger`, and `fact-card-catalog`. The middle two name no `surface '...'` token
  at all, so any census keyed on that token is structurally blind to them; that composition is the durable fact
  here because it is a property of the driver. **No line or surface totals are carried in this leaf.** They are
  per-commit counters: `.11`'s ledger prepends crossed two bands inside its own commit and `.11a`'s crossed
  another inside its own, which is three same-transaction invalidations in three consecutive commits. Derive the
  population from the driver at the revision you care about.
  **Ownership test.** `.11` used `grep -rl <surface> docs/tasks/*.md`, which fails in both directions. It scores
  a `done` tree as an owner: `corpus_task_evidence_parts` was counted owned by `LIVE-DOC-STOP-RISK`, which is
  `done`, so a real gap was hidden. And it is satisfied by the act of reporting: `alignment_task_evidence_index`,
  `alignment_task_evidence_parts`, and `rust_analysis` matched `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md` as
  soon as that leaf named them in order to say nothing owned them. A screen may find candidates; it may not be
  published as the answer.
  **What the leaf must deliver.** An explicit assignment, reviewed per row, binding each warned item from the
  gate-level run to one open leaf that will act on it, or to a stated exemption with its reason — the form this
  tree already uses for `decision_records`, which is excluded because `DECISION-RECORD-CAPACITY-HEADROOM` owns
  its axes. Screened against the 24 open trees, the items with no open owner today are
  `corpus_task_evidence_parts` (its only namer, `LIVE-DOC-STOP-RISK`, is closed) and the three this leaf already
  owns: `alignment_task_evidence_index`, `alignment_task_evidence_parts`, and `rust_analysis`. The `Opening
  Pressure Boundary` table above is **not** a defect and must not be rewritten to match: it is anchored to
  `92e59c97` and is a dated snapshot, which is exactly why it cannot serve as the current assignment
  Prerequisite: none; opened by `CLAIM-VERIFICATION-ADOPTION.11`, premises corrected by `.11a`
  **Population derived, and it corrects this leaf's own premise a third time.** Run at `057710cd`,
  `scripts/check_live_document_size.sh` composes **22** producers that emit a line and **five** that emit a
  warning — not the four recorded above. The fifth is `roadmap-projection`, and it is missed for a reason
  worth more than the count: it emits `WARNING section ...` in **uppercase with no colon**, so a census keyed
  on the lowercase `warning:` token reads 38 of 42 lines and is blind to all four of its rows. `.11` was
  blind to two producers by the `surface '...'` token; the same class of blindness reappears here on the
  warning token itself, which is why the population must be derived from the driver's own output rather than
  from any keyed screen.
  **The driver also double-emits.** `fact-card-catalog` and `roadmap-projection` each run twice — once in the
  block guarded by `[ "$ROOT" = "$ADAPTER_ROOT" ]` and once in the gate path — so three rows appear twice. A
  naive count reports 42 warned items; deduplicated the population is **39**, from `live-document-size` (22),
  `active-task-evidence` (7), `rolling-ledger` (7), `roadmap-projection` (2), and `fact-card-catalog` (1).
  No totals are carried forward from this note: they are per-commit counters, exactly as this leaf already
  records, and the derivation command is the authority.
  **The assignment is delivered above, and it found the trap this leaf was opened to fix, inside this leaf.**
  Ownership was read from each owner's own `Status` line rather than from any mention of the surface, and five
  named owners turned out to be `done` trees — including `DECISION-RECORD-CAPACITY-HEADROOM`, which this
  leaf's own acceptance cites as the model form of a clean exclusion. Its row is the most pressured item in
  the population. Twenty-three rows bind to eight open owners, fifteen are bound to six leaves opened here,
  and exactly one — `active_resume` — is exempt with its reason.
  Verification: `derived at 5ceb27c8 from bash scripts/check_live_document_size.sh: 22 producers emit, five
  emit warnings, 39 distinct warned items after removing three double-emissions; grep 'warning:' returns 38 of
  42 lines and misses every roadmap-projection row. All 39 rows assigned: 23 to open owners, 15 to .8-.13
  opened here, 1 exempt; 23+15+1 = 39. Every per-file warning resolved to its driving file. Five named owners
  re-read from their own Status lines are done trees`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7 — derive the warned population from the driver, not from a screen`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.3`
  Status: `pending`
  Goal: keep the maintained EvidenceIR book chapter current below actionable pressure
  Acceptance: content is reorganized by reader concern without losing examples, links, or public behavior; the
  book aggregate and current-truth authorities remain exact and no size bound moves
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`
  Status: `pending`
  Goal: classify and remedy the research, validation-snapshot, and README member warnings
  Acceptance: each axis has a lifecycle-correct local remedy or a measured reason it is immutable/healthy;
  accepted transactions preserve exact evidence and no generic warning is merely suppressed
  Re-measured (`2026-08-28`, found while running `SOURCE-IR-REPRODUCIBILITY.9`): the research axis moved
  from a line warning to a **membership stop**, which the `2026-08-14` boundary above did not record
  because it did not exist then. `docs/research/*.md` is **63 of a 64-file ceiling**, and for this surface
  `health_targets.files` and `enforcement_ceilings.files` are both 64 — so there is no warning band left
  and the *next* research record is the last one this collection can accept. The line axis is equally
  tight at 639 of 640. Two active trees write research records as their normal output
  (`SOURCE-IR-REPRODUCIBILITY` published three in two days), so this is reachable within a slice or two,
  and unlike a rolling ledger this surface has **no declared rollover transaction** to release it. That is
  the condition `LIVE-DOC-STOP-RISK` exists to prevent: a bound a surface can reach with no remedy
  compliant work can take. Note the `.jsonl` rollover plans under the same directory do **not** count —
  the surface targets `*.md` only.
  Children: `.4a`, `.4b`, `.4c`, `.4d`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a`
  Status: `done` (`2026-08-31`)
  Goal: remove the research-plane file count behind a declared, gated exemption
  Acceptance: `research_records` declares no `files` bound in either band, behind an ADR 0045
  `cardinality_exemption` naming this decision, this leaf, and `canonical_collection_indexes` as the route
  that covers its declared index; every resource dimension stays numeric. The measurement must support the
  removal rather than quiet a warning (this tree's Non-Goal): at `276abfc3` the collection is **63 of 64
  files = 98.4%** with `health_targets.files == enforcement_ceilings.files`, so there is no warning band,
  while `lines_total` is **10,840 of 40,960 = 26.5%** and `bytes_total` **740,829 of 4,194,304 = 17.7%**.
  The aggregates bound the resource that exists and bind first: at the measured 172-line / 11.8 KB record
  mean they admit about 238 records against the catalog's ~374 rows, so the relocation is to a *resource*
  bound with a live warning band rather than to another countdown. Nulling a ceiling is an increase, so the
  transaction consumes one exact single-use `ceiling_increase_authorities.jsonl` record in the same commit
  Prerequisite: none
  **No new decision record.** ADR 0045's Decision section is written as the general mechanism — the exemption
  object, its four required fields, and the four separate refusals — so applying it is not a new decision, and
  the per-surface measurement lives where the gate can read it: the registry's own `rationale` field and this
  leaf. A second near-duplicate ADR would also spend `decision_records.files`, an axis already at 82.8%.
  Verification: `research_records files 63/64 -> unbounded in both bands behind the declared exemption;
  lines_total 10,840/40,960 (26.5%) and bytes_total 740,829/4,194,304 (17.7%) unchanged and still numeric,
  as are lines_each, bytes_each and line_bytes_each; route canonical_collection_indexes covers
  docs/catalogs/research-records.md and is bounded in every dimension; one exact single-use authority added
  and consumed (old/new enforcement_ceilings match byte-for-byte); the three research warnings are gone from
  bash scripts/check_live_document_size.sh, which reports 901 Markdown files / 57 governed surfaces and
  exits 0; census 41 surfaces / 74 evidence units; book claims 39 files / 325 adjudicated lines`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a — remove the research-plane cap through a declared exemption`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4b`
  Status: `done` (`2026-08-31`)
  Goal: retire the single-use authority `.4a` consumes
  Acceptance: the ceiling-increase authority record is removed once HEAD carries the new bands, because the
  generic gate refuses an unused or banked authority on the very next commit; nothing else moves. This is the
  same mandatory second transaction `.2b` performed for `.2a`
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a`
  Verification: `the refusal was observed RED at 3cf7f6d0 before the fix — "surface 'research_records' has
  unused or banked ceiling-increase authority", the generic gate failing with 1 violation — and green after,
  at 901 Markdown files / 57 governed surfaces with no research warning. The authority registry is back to
  its bare registry record; the surface bands, the exemption, and every other authority are untouched`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4b — retire the consumed research ceiling authority`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c`
  Status: `pending`
  Goal: release the per-record line stop the file-count exemption does not touch
  Acceptance: `docs/research/production-genericity-pipeline-audit.md` is **639 of a 640-line per-file
  ceiling**, again with `health_targets.lines_each == enforcement_ceilings.lines_each`, so a one-line
  correction to that record is refused. Decide from measurement whether the remedy is a lossless split at a
  section boundary (the partitioned-canonical remedy, which costs one catalog row and therefore needs `.4a`
  first) or a re-derived per-file profile, and preserve every byte either way
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d`
  Status: `pending`
  Goal: classify and remedy the validation-snapshot and README member warnings
  Acceptance: `validation_snapshot` lines_each (544 of 640) and `readme_entrypoint` line_bytes_each (108 of
  120, already at its rollover milestone) each get a lifecycle-correct local remedy or a measured reason the
  surface is healthy; no generic warning is merely suppressed and README stays inside `README_POLICY.md`
  Prerequisite: none
  Verification: `pending`
  Commit: `pending`

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

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.8`
  Status: `pending`
  Goal: restore writable headroom for the decision-record surface, whose named owner is closed
  Acceptance: `decision_records` bytes_each and lines_each come back under their rollover milestones by a
  remedy the surface can sustain, not by widening a bound; `DECISION-RECORD-CAPACITY-HEADROOM` being `done` is
  recorded as the reason this axis has no owner, and either that tree is reopened or this leaf carries the axis
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.9`
  Status: `pending`
  Goal: restore title-part headroom for the fact-card catalog, whose two named owners are both closed
  Acceptance: `fact_card_titles` files and the catalog's planned title-part count come back under warning by a
  derived remedy following `.2c`'s form — bounds set so a structurally full part sits below its own warning —
  with no hand-edited member list
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.10`
  Status: `pending`
  Goal: carry the corpus task-evidence containment axis its closed owner left behind
  Acceptance: `corpus_task_evidence_parts` files and the corpus semantic-part collection come back under
  warning; `CORPUS-TASK-EVIDENCE-CONTAINMENT` being `done` is recorded as the reason, and the corpus index is
  checked for the same cardinality stop the alignment index has
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.11`
  Status: `pending`
  Goal: give the two rolling ledgers outside `CHANGES-LEDGER-ROLLOVER` an owner for their record-budget breach
  Acceptance: the `development-notes` and `rust-codebase-analysis` ledgers either meet their derived per-record
  budgets or their windows are re-derived from measured record means, using the same reasoning the `changes`
  ledger warning already states — that the byte dimension binds first and a rollover only resets the clock;
  `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.4` being `done` is recorded as the reason this had no owner
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.12`
  Status: `pending`
  Goal: bring the workflow-standards line budget back under warning
  Acceptance: `workflow_standards` line_bytes_each returns under its milestone; the driving line is
  `DOCTRINE_ENFORCEMENT.md` at 877 of 1024 bytes, so the remedy is a routing or wrapping change to that
  surface, never a bound edit
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.13`
  Status: `pending`
  Goal: relieve the two roadmap root sections the projection contract reports over budget
  Acceptance: `Current strategic priorities` (96.4% of 56 lines) and `Objective` (85.7% of 14) come back under
  their bounds by the remedy the checker itself names — route per-leaf detail to its owning `docs/tasks/` tree,
  or roll the root per ADR 0030 — and the uppercase `WARNING` token these rows use is noted wherever a census
  keys on warning text
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

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
  Status: `pending`
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
  not hold this invariant and a gate must
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`
  Verification: `pending`
  Commit: `pending`

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

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.18`
  Status: `pending`
  Goal: execute the staleness gate every claim declares, or stop calling it a gate
  Acceptance: `scripts/check_claim_verification.pl` executes `rederive.commands` and
  `falsification.controls` under `--execute` but **never** `durability.stale_check`; it only schema-validates it
  and checks that its `inputs` cover the watched artifacts. So every claim's staleness gate is decorative, and
  the proof is that `current-claim-census-frozen`'s pinned `stdout_contains` read `current-claim-census: 39
  current surfaces` while the producer printed 40 before this slice and 41 after — a marker that would have
  refused for at least one prior commit had anything run it. `.14a` repaired that one pin to the stable phrase
  the claim's own assertion requires (the denominator is read from `--report`, never carried); this leaf
  decides whether the stale gate is executed, folded into the rederive commands, or removed. The durable owner
  is `CLAIM-VERIFICATION-ADOPTION`; it is carried here because that tree is at **91.2%** of its
  `task_evidence.bytes_each` ceiling and a new leaf there spends the axis its own commit must protect
  Prerequisite: none; found by `.14a` while refreshing the claim pins its inputs moved

## Reviewed Warning Assignment (`.7`, `2026-08-31`)

Derived at `5ceb27c8` from `bash scripts/check_live_document_size.sh`, deduplicated to 39 distinct items.
Reviewed per row against the open trees; **not** screened by grep, because that test scores a `done` tree as
an owner and is satisfied by the act of reporting. Owner status was read from each tree's own `Status` line
and each per-file warning was resolved to the file actually driving it. No totals are carried: re-derive.

| Warned item | Driver | Assigned owner |
| --- | --- | --- |
| `knowledge_cards` lines_each | `docs/knowledge/production-genericity-boundary.md` | `.1` |
| `shipped_behavior` bytes_each | `docs/book/src/pipeline/evidenceir.md` | `.3` |
| `research_records` files / lines_each / bytes_each | `docs/research/` (63 of 64 files; widest 639 of 640) | `.4` |
| `validation_snapshot` lines_each | `VALIDATION_SNAPSHOT.md` | `.4` |
| `readme_entrypoint` line_bytes_each | `README.md` | `.4` |
| `alignment_task_evidence_index` lines_each; active task index lines | `spec-to-intent-alignment/INDEX.md` | `.14a` |
| `alignment_task_evidence_parts` lines_each; semantic part lines_each | alignment parts collection | `.14b` |
| `rust_analysis` lines_each | `RUST_CODEBASE_ANALYSIS.md` | `.14c` |
| `change_history` bytes_each / lines_each; ledger `changes` ×2 | `CHANGES.md` | `CHANGES-LEDGER-ROLLOVER.4` |
| ledger `live-achievement-status` ×2 | `LIVE_ACHIEVEMENT_STATUS.md` | `STATUS-LEDGER-ROLLOVER.2` |
| bounded active root bytes / line_bytes / lines | `docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md` | `SPEC-TO-INTENT-ALIGNMENT` |
| `task_evidence` bytes_each / lines_each | `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md` (254,031 B) | `CLAIM-VERIFICATION-ADOPTION` |
| `decision_records` bytes_each / lines_each / files | `docs/decisions/0038-…kernel.md` (97.1%) | `.8` (opened here) |
| `fact_card_titles` files; catalog planned title parts | `docs/knowledge-catalog/` (5 of 6) | `.9` (opened here) |
| `corpus_task_evidence_parts` files; corpus part files / lines_total | `corpus-coverage/` parts | `.10` (opened here) |
| ledger `development-notes` ×2; ledger `rust-codebase-analysis` | `DEVELOPMENT_NOTES.md`, `RUST_CODEBASE_ANALYSIS.md` | `.11` (opened here) |
| `workflow_standards` line_bytes_each | `DOCTRINE_ENFORCEMENT.md` (877 of 1024) | `.12` (opened here) |
| roadmap `Current strategic priorities` 96.4%; `Objective` 85.7% | `ROADMAP.md` | `.13` (opened here) |
| `active_resume` lines_each / line_bytes_each | `MEMORY.md` | **exempt** — see below |

**The exemption, with its reason.** `active_resume` is the one surface whose pressure is *by design*. `.5`
gave the resume pointer a deliberate band and the whole point of a band is to be lived in; ordinary compliant
work moves it back down, demonstrated this session when rewriting the pointer took it from **94.0% to 80.0%**
of `lines_each` without any authority edit. A surface that ordinary work already regulates does not need a
remedy leaf; it needs the band it has. Re-open this only if a rewrite ever fails to recover it.

**Corrected the same day, and the correction is the finding.** The three rows above first read `.7`, which
this commit closes — so closing the leaf that assigned rows to itself orphaned them, which is the exact defect
this leaf exists to eliminate, committed inside the commit that eliminated it. `.7` delivered the assignment;
the *remedies* for the rows it assigned to itself need an open owner, and that is `.14`. A class that survives
being reviewed, documented, and guarded against by its own author at full attention is not a discipline
problem, it is a missing mechanical check — `.15` supplies it.

**Four owners named in this tree or in `COMMIT.md` are `done` trees, so their rows were unowned.** Read from
each tree's own `Status`: `DECISION-RECORD-CAPACITY-HEADROOM`, `FACT-CARD-CAPACITY-HEADROOM`,
`FACT-CARD-CATALOG-CONTAINMENT`, `CORPUS-TASK-EVIDENCE-CONTAINMENT` and
`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION` are all closed. **`.7`'s own acceptance cites `decision_records` as
the model form of a clean exclusion — "excluded because `DECISION-RECORD-CAPACITY-HEADROOM` owns its axes" —
and that tree is `done`.** So the row this leaf held up as correctly-owned was in fact unowned, and it is the
most pressured item in the population at 97.1% of bytes and 92.6% of lines. This leaf fell into the exact trap
it was opened to fix, on the exact row it used as its example. That is why ownership is now read from the
owner's `Status` line rather than from any mention of the surface.


## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.0` | `done` | exact clean pressure and owner boundaries are pinned |
| 2 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.1` | `pending` | one line remains before the next current structural fact is refused |
| 3 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.2a` | `done` | the nearest measured stop on the plane: 9 trees below a ceiling the director has decided to remove, and it has two enforcers |
| 4 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.2b` | `done` | a consumed single-use ceiling authority is refused as banked on the very next commit |
| 5 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c` | `done` | `.2a` relocates the stop to the index at ~108 trees; this is the half that removes it |
| 6 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.4` | `pending` | re-ranked `2026-08-28`: `docs/research/*.md` is 63 of a 64-file ceiling with no warning band and no rollover, and two active trees write research records |
| 7 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.5` | `done` | routed the 19-line constant preamble out; mutable budget 31 -> 42 with no bound moved, and the split is now gated |
| 8 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` | `done` | the gate warns 35 lines across four producers and no reviewed assignment exists; a grep screen cannot serve, since reporting a gap closes it |
| 9 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.14a` | `done` | the index was one leaf from a hard refusal and it gated the product frontier `SPEC-TO-INTENT-ALIGNMENT.9c` |
| 10 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a` | `done` | `research_records` is 63 of a 64-file ceiling with no warning band and no rollover: the next record is the last one |
| 11 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.4b` | `done` | the single-use authority `.4a` consumes is refused as banked on the very next commit |
| 12 | `LIVE-DOCUMENT-PRESSURE-HEADROOM.15` | `pending` | seven instances of the closed-owner class in one session is the evidence that review does not hold the invariant |

## Decisions

- `2026-08-31`: split `.4` into `.4a`/`.4b`/`.4c`/`.4d` and apply ADR 0045 rather than raise the research
  ceiling. The measurement carries the decision on every axis this tree's Non-Goal cares about: the count is
  at 98.4% with no warning band and no rollover while the aggregates are at 26.5% and 17.7%, and at the
  measured record mean those aggregates bind at about 238 records — before the catalog's own row capacity.
  So the stop relocates to a resource bound that still has a live warning band, not to another countdown.
  Raising 64 to some larger number would have moved the countdown and left the no-rollover defect intact.

- `2026-08-31`: split `.14` into `.14a`/`.14b`/`.14c` before implementing any of it. `.7` assigned three rows
  to one leaf, but they are three different lifecycles — a bounded snapshot whose size is a pure function of
  lifetime leaf count, a partitioned canonical collection, and a rolling ledger — and this tree's own Non-Goal
  forbids combining independent lifecycle remedies into one migration. `.2` was split for the same reason.
- `2026-08-31`: `.14a` shards by lifecycle and stages adoption per tree rather than migrating all three
  contracts at once. The schema change is one mechanism, but the *lifecycle* it publishes needs an authority:
  only the alignment parts carry `- ID:`/`State:` node blocks. Migrating the other two would have meant
  authoring 108 lifecycle values no evidence can corroborate, which is a worse defect than the bound being
  fixed. `route_catalog_state` declares the shape, exactly as `migration_state` already stages this doctrine.
- `2026-08-31`: state the relocation. `.14a` moves the alignment tree's nearest structural stop from the
  index's 115.2-line milestone to `limits.manifest.max_leaf_routes` at 128 against 83 declared routes. Calling
  it "the index bound is removed" would repeat the error `.2` caught in `.2a`; `.17` owns the residual.

- `2026-08-14`: open one pressure-frontier tree rather than one task per warning. The surfaces need distinct
  remediation transactions, but one bounded owner can preserve the exact measured ordering without consuming
  several more task-file slots at the already-triggered collection milestone.
- `2026-08-28`: `.5` closed. The decision was delegated with one condition — signoff quality — so it was
  taken on measurement rather than taste: the preamble is a 19-line constant in all 30 sampled commits and
  the mutable half had spent 90% of what remained, which rules out authoring discipline as the remedy.
  Every routed line was proven to resolve upstream before deletion, and the split is now a derived,
  RED-controlled bound rather than a convention, so it cannot creep back.
- `2026-08-28`: re-measured while running an unrelated slice, and two axes are worse than the opening
  boundary recorded. `docs/research/*.md` is 63 of 64 files and 639 of 640 lines on its widest member, with
  `health_targets.files == enforcement_ceilings.files`, so there is no warning band and no declared
  rollover — the next research record is the last one. `.4` now owns that explicitly. `MEMORY.md` has the
  same shape at 46 of 50 lines, which is why every slice this session had to hand-compress it; `.5` opens
  to decide whether the 18-line fixed preamble belongs inside the bounded pointer at all.
- `2026-08-29`: split `.2` into `.2a`/`.2b`/`.2c` rather than implement the director's decision inside the
  leaf that made it. Two reasons, both measured rather than procedural. First, the cap has a **second
  enforcer** the decision did not name — `scripts/check_task_tree_catalog.pl:18` carries its own
  `$MAX_TASKS = 160` — so a registry-only edit would have shipped as "cap removed" while the plane stayed
  capped; that is a real defect the split forced into view before it could land. Second, the remedy is two
  different transactions on two different surfaces: nulling a registry dimension under a gated exemption, and
  sharding a derived index. This tree's own Non-Goal forbids combining independent lifecycle remedies into one
  migration, and the single-use ceiling-increase authority protocol adds a mandatory third commit to retire
  what `.2a` consumes.
- `2026-08-29`: state the relocation instead of claiming removal. `.2a` moves the binding stop from 9 trees
  (`task_evidence` files, 151/160) to about 108 (`task_tree_index`, 404/512 lines at one row per tree). That
  residual stop has no declared rollover, so calling `.2a` alone "no limit on task-trees" would be false;
  `.2c` is the leaf that makes the directive true.
- `2026-08-14`: prioritize the current knowledge card, then the task plane. The card has one line left and is a
  likely `.f` writer target; the task collection is already at 90% but still has 16 opening-boundary slots.
- `2026-08-14`: record maintained and immutable large-member warnings without assuming they share a remedy.
  Book content is writable product documentation; accepted research/snapshot evidence may require a route or
  lifecycle change rather than an in-place edit.

## Open Questions

- Which fact-card content is current authority versus immutable structural-qualification history? `.1` owns the
  exact route-preserving split.
- Should task capacity use routed partitioning, a newly derived profile, or both? `.2` must decide from census.

## Blockers

- Execution is intentionally sequenced after alignment containment. The tracking boundary blocks no current
  migration leaf; `.1` must close before behavioral `.f` needs another production-genericity fact update.

### Acceptance Checklist (enforced) — `LIVE-DOCUMENT-PRESSURE-HEADROOM.0`

- [x] **REPRODUCE / MEASURE** — exact repository metrics reproduce every opening row; the resulting ownership
  cost is also measured at 145 task files and a 398-line derived catalog.
- [x] **ROOT CAUSE (WHY + WHERE)** — canonical current surfaces grew under individually valid writes, while their
  fixed count/per-member authorities have no shared pressure frontier or ordered remedy owner.
- [x] **ADDRESSED (verified)** — this task owns four lifecycle-specific leaves and prioritizes the one-line card
  stop before task-plane capacity; no content or bound is changed by ownership.
- [x] **NO REGRESSION** — governed source/content, decisions, research, validation, README, mdBook, product code,
  and all live-size literals are byte-identical; catalogs, Knowledge Map, live-size, and doctrines pass.
- [x] **GENERICITY** — leaves are separated by lifecycle and authority coupling, not by document subject; a future
  remedy must remain portable and measured rather than special-casing current filenames in a checker.
- [x] **LOCKSTEP** — this tree, derived task catalog, live-size registry metrics, existing decision-pressure owner,
  and `MEMORY.md` agree on the exact boundary and next eligible remediation.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-31` | `.4b` | generic live-size checker before and after removing the record | RED observed first at `3cf7f6d0`: "unused or banked ceiling-increase authority", 1 violation. Green after: 901 files / 57 surfaces. The single-use protocol is therefore proven in both directions, not just claimed |
| `2026-08-31` | `.4a` | `check_live_document_size.sh`; the generic size checker with the authority registry; census, book-claim and doctrine gates | research files **63/64 -> unbounded** behind the declared exemption; every resource dimension stays numeric; the single-use authority is added and consumed in the same commit; the composed gate reports **901 Markdown files / 57 governed surfaces** and exits 0 with no research warning. The stop relocates to `lines_total`/`bytes_total`, which at the measured record mean bind at ~238 records and still have a warning band |
| `2026-08-31` | `.14a` | `check_active_task_evidence.pl --self-test`; the three contracts `--check`; `--write` round-trip; `check_live_document_size.sh`; census, book-claim, knowledge-map and doctrine gates | index **115 -> 43 lines** (89.8% -> 33.6%), clear of the 90% milestone that refused the next leaf; 83 routes = 3 open / 80 closed in one 93-line catalog part; self-test **61/61** with eleven new RED cases and a writer round-trip plus preflight refusal; live-size **900 files / 57 surfaces**; census 41 surfaces / 73 evidence units; book claims 325/325; Knowledge Map 274 facts / 2,193 keys |
| `2026-08-31` | `.14a` cross-check | first run of the declared-vs-observed lifecycle rule over all 83 alignment routes | **two findings on adoption.** `.8` was `active` in its part while its four children were `done` and the root records `.0`-`.8` complete — the landing would have published it open; corrected. `.9a` has no node record in any part, the one route no evidence corroborates, pinned by `max_unverified_routes: 1` and owned by `.16` |
| `2026-08-31` | `.7` self-orphan | re-read the assignment table against the leaf's own closing `Status` | the three rows `.7` assigned to itself were orphaned by closing it — the class this leaf exists to eliminate, committed inside the commit that eliminated it, and the seventh instance overall. Rows move to `.14`; `.15` makes the invariant mechanical because review demonstrably does not hold it |
| `2026-08-31` | `.7` assignment | 39 rows reviewed per row against open trees; owner status read from each owner's own `Status` line; every per-file warning resolved to its driving file | 23 rows bind to eight open owners, 15 to `.8`-`.13` opened here, one exempt with reason (23+15+1=39). **Five named owners are `done` trees**, including `DECISION-RECORD-CAPACITY-HEADROOM`, which this leaf cited as its model exclusion and whose row is the most pressured in the population |
| `2026-08-31` | `.7` population | `bash scripts/check_live_document_size.sh` at `057710cd`, deduplicated and classified by emitting producer | 22 producers emit, **five** emit warnings — not the four this leaf recorded — and 39 distinct warned items remain after removing three double-emissions; the missed producer `roadmap-projection` uses uppercase `WARNING` with no colon, so a `warning:`-keyed census reads 38 of 42 lines and is blind to all of its rows |
| `2026-08-14` | `.0` ownership | exact metrics; Knowledge Map routing; existing owner census; task catalog; content/authority diffs; live-size/doctrine | seven axes pinned; ownership-only resulting tree 145 task files / 398 index lines; no governed content or bound change |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.4b` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.4b — retire the consumed research ceiling authority` | the banked-authority refusal observed RED at `3cf7f6d0` first |
| `.4a` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a — remove the research-plane cap through a declared exemption` | ADR 0045 applied to a second surface; `.4b` must retire the consumed authority next |
| `.14a` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.14a — shard the task-evidence index by lifecycle` | ADR 0046; the stop relocates to `max_leaf_routes` and `.17` says so |
| `.7` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.7 — assign every gate-level warning by review` | the leaf found its own documented trap inside itself; ownership now reads the owner's `Status`, not a mention |
| `.7` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.7 — derive the warned population from the driver, not from a screen` | the leaf's own four-producer premise is corrected a third time; the reviewed per-row assignment is the open half |
| `.0` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.0 — own the current live-surface pressure frontier` | one bounded owner over ordered independent remedies |
| `.5` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.5 — stop the resume pointer spending its budget on prose that never changes` | fixed region 19 -> 8 lines; the split is gated at a derived cap |
| `.2` | `LIVE-DOCUMENT-PRESSURE-HEADROOM.2 — split the decided no-cap remedy into the three transactions it actually is` | container; found the second `$MAX_TASKS` enforcer and measured the relocation |

## Changelog

- `2026-08-31`: `.4` becomes a container over `.4a` (the membership stop, done), `.4b` (retire the consumed
  authority), `.4c` (the 639-of-640 per-record line stop) and `.4d` (validation snapshot and README). Also
  corrected a duplicated `.4` row the `.14a` commit left in the Current Frontier table — the same stale
  hand-maintained-table defect this tree already caught once on `.2a`/`.2b`/`.2c`.

- `2026-08-31`: `.14` becomes a container over `.14a`/`.14b`/`.14c`, and `.14a` lands the lifecycle shard with
  ADR 0046. Opened by the work rather than by review: `.16` (`.9a` has no node record anywhere), `.17` (the
  stop relocated to `max_leaf_routes` 128 against 83 declared routes) and `.18` (`durability.stale_check` is
  declared and schema-validated but never executed, proven by a pin reading `39 current surfaces` against a
  producer printing 40 before this slice).

- `2026-08-30`: corrected `.7`'s premises the same day it was opened, from
  `CLAIM-VERIFICATION-ADOPTION.11a`. Two of the three facts `.11` used were wrong. The warning population it cited
  was `check_live_document_size.pl`'s, not the doctrine driver's — the gate emits 35 lines across four producers,
  and the `active-task-evidence` and `rolling-ledger` lines name no surface token, so they are invisible to any
  surface-keyed census. And the ownership test was a grep that scores `done` trees as owners (hiding
  `corpus_task_evidence_parts`, whose only namer `LIVE-DOC-STOP-RISK` is closed) and that the finding satisfied by
  being written. `.7` now requires a reviewed assignment rather than a screen. One thing `.11` reported as a
  defect is withdrawn entirely: the `Opening Pressure Boundary (92e59c97)` table is a dated snapshot, correctly
  anchored, and disagreeing with a current producer is what a dated snapshot is supposed to do.
- `2026-08-30`: opened `.7` and corrected this tree's own frontier. `CLAIM-VERIFICATION-ADOPTION.11`, running an
  enumerating command over a different population, measured this tree's warning set: the producer warns about
  **13** surfaces and both published lists — the Opening Pressure Boundary table and `MEMORY.md`'s five — were
  shorter, disagreed with each other, and carried the conclusion "none is an unowned warning". Classified rather
  than counted: three surfaces are named by no task tree at all. The pointer sentence is withdrawn there and the
  gap is owned here. Separately, the Current Frontier still showed `.2a`, `.2b`, and `.2c` as `pending` while the
  Task Tree section had them `done` since `2026-08-29` — a stale hand-maintained table that would have handed a
  fresh session three completed leaves; corrected in the same commit that found it.
- `2026-08-29`: `.2` becomes a container over `.2a` (apply the gated no-cap profile), `.2b` (retire the
  consumed single-use authority), and `.2c` (shard the derived index). Records the second enforcer
  (`$MAX_TASKS`) and the exact 9-trees -> ~108-trees relocation the decision implies.
- `2026-08-14`: created from the post-`.2.2` live-size report; pins seven non-rolling pressure axes, excludes the
  separately owned decision plane, and leaves the active alignment migration transaction unchanged.
