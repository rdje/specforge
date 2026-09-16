# LIVE-DOCUMENT-PRESSURE-HEADROOM — task-plane cardinality

- Part ID: `task-plane-cardinality`
- State: `legacy`

<!-- pressure-headroom-task-source-region:task-plane-nodes:start -->
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

<!-- pressure-headroom-task-source-region:task-plane-nodes:end -->
