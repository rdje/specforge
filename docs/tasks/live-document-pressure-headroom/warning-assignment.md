# LIVE-DOCUMENT-PRESSURE-HEADROOM — warning assignment

- Part ID: `warning-assignment`
- State: `active`

<!-- pressure-headroom-task-source-region:warning-assignment-nodes:start -->
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
  Status: `done` (`2026-09-11`)
  Goal: keep the maintained EvidenceIR book chapter current below actionable pressure
  Acceptance: content is reorganized by reader concern without losing examples, links, or public behavior; the
  book aggregate and current-truth authorities remain exact and no size bound moves
  **It stopped being a warning and became a breach, which is what forced the transaction.**
  `WIRE-BASED-100.10e` added a section to `pipeline/evidenceir.md` and the chapter went to **131,583 bytes
  against its 131,072 ceiling** — `shipped_behavior` failed closed and blocked the commit. The chapter had
  been at 99% before that slice, so trimming the new section would have left it at ~99.6% and broken the
  next book edit: the reprieve this doctrine exists to refuse, and the same trap
  `STATUS-LEDGER-ROLLOVER.3` names for ledgers.
  **The split is by reader concern, not by size.** Nine consecutive sections —
  `PDF-VARIANT-DIGESTION.10a`…`10i` — are one subject: the table *shapes* a register map arrives in
  (four-column bit tables, two-column layout grids, three-column caption-decides cases, dword-relative
  cells, byte-location tables, fields as headings, reused mnemonics). They moved whole to
  `docs/book/src/pipeline/register-tables.md` under their own title, with a pointer left in place and a
  new `SUMMARY.md` route.
  **Moved byte-identically, deliberately.** Every relocated line keeps its exact bytes — no heading was
  promoted, no sentence rewritten — so all 14 pinned quantitative regions inside the block survived the
  move as a pure path+line re-point rather than a re-adjudication. Three further regions in the remaining
  chapter re-anchored by content; **zero regions were lost or invented**, and the frozen 332-candidate
  population is unchanged.
  Verification: `pipeline/evidenceir.md` **131,583 → 100,739 bytes = 76.9% of its 131,072 ceiling**, real
  headroom rather than one slice's worth; `pipeline/register-tables.md` 32,038 bytes. Registry denominators
  moved `39 → 40` book files and `21 → 22` candidate files with the `book_summary` source pin refreshed;
  `perl scripts/check_book_quantitative_claims.pl --check` reports 40 files / 332 candidates / 332
  adjudicated regions; `bash scripts/check_live_document_size.sh` and `scripts/check_doctrines.sh` green.
  No ceiling, milestone or bound moved.
  Commit: see log.

<!-- pressure-headroom-task-source-region:warning-assignment-nodes:end -->

<!-- pressure-headroom-task-source-region:assigned-successor-nodes:start -->
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

<!-- pressure-headroom-task-source-region:assigned-successor-nodes:end -->

<!-- pressure-headroom-task-source-region:reviewed-warning-assignment:start -->
## Reviewed Warning Assignment (`.7`, `2026-08-31`)
<!-- current_owners:start -->

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
| `task_evidence` lines_each | `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` (2,878 of 3,000) | `EXTRACTION-QUALITY-GAUGE` |
| `task_evidence` bytes_each | `docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md` (261,907 B, after `.29`) | `.30` (opened here) |
| `decision_records` bytes_each / lines_each / files | `docs/decisions/0038-…kernel.md` (97.1%) | `.8` (opened here) |
| `fact_card_titles` files; catalog planned title parts | `docs/knowledge-catalog/` (5 of 6) | `.9` (opened here) |
| `corpus_task_evidence_parts` files; corpus part files / lines_total | `corpus-coverage/` parts | `.10` (opened here) |
| ledger `development-notes` ×2; ledger `rust-codebase-analysis` | `DEVELOPMENT_NOTES.md`, `RUST_CODEBASE_ANALYSIS.md` | `.11` (opened here) |
| `workflow_standards` line_bytes_each | `DOCTRINE_ENFORCEMENT.md` (877 of 1024) | `.12` (opened here) |
| `workflow_standards` lines_each | `DOCTRINE_ENFORCEMENT.md` (597 of 700, after `.27`) | `.27a` (opened here) |
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
<!-- current_owners:end -->


<!-- pressure-headroom-task-source-region:reviewed-warning-assignment:end -->

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34b`
  Status: `done` (`2026-09-24`; opened by `KG-ISF-COMPLETENESS.2a.vii.b`)
  Goal: cut `docs/book/src/pipeline/isf-adapter.md` by reader concern before a book edit is refused
  Acceptance: the reviewed assignment above binds the `shipped_behavior` bytes_each warning to `.3` and
  `pipeline/evidenceir.md`. `.3` is `done`, and since `.34a`'s cut the chapter driving that warning is
  `pipeline/isf-adapter.md`, which no open leaf owns. It stood at **110,191 bytes (84.1% of 131,072)** at
  `2aa432d1` and at **113,711 (86.8%)** after `.2a.vii.b`'s section (read at `ee78ecdb`; the opening text said
  113,685, measured before that section's counts were rephrased as dated evidence); rollover is 117,965, and
  `KG-ISF-COMPLETENESS.2a.vii.c` will extend the same chapter. Apply `.34`'s rule: attribute the chapter's
  growth per block across its recent revisions, then move the ACCRETING block whole (the candidate is *Closed
  task trees — how each was implemented and verified*) under its own title with a `SUMMARY.md` route and a
  pointer left in place, re-deriving every pinned region at its new location.
  **The candidate was wrong, and measuring first is what showed it.** Growth attributed per `##` section by
  differencing the chapter's endpoints over its last 40 revisions (`ca3e1c9f` -> now, +49.8 KB): *Closed task
  trees* grew only +3.4 KB, while *ISF IR data model* grew +26.0 KB, and within it the eight consecutive enum
  subsections (*Only named enums reach the `.isf`* through *The member's name comes from the column the document
  names it in*) account for +24.0 KB of their 30.9 KB. They are one reader concern: which codes become an enum,
  how a member is named, where its value comes from. That block moved **byte-identically, no heading promoted**,
  to `docs/book/src/pipeline/isf-enums.md` (`#` title, a four-line intro, then the block's own `###`
  headings), with a `SUMMARY.md` route beside *ISF Adapter* and a pointer section left in place.
  Verification: `isf-adapter.md` **113,711 -> 83,026 bytes (86.8% -> 63.3%)**; the largest chapter is now
  `direction/temporal-intent-capture.md` at 68.0%, so `shipped_behavior` reports no byte warning. The 78
  pinned regions on the chapter: **26 re-pointed to the new chapter, 13 line-shifted, 39 untouched**, every
  digest re-derived at its new location before the files were written; `repin_claim_regions.py --check`
  reports 0 absent. Registry denominators 44 -> 45 book files and 28 -> 29 candidate files; candidate lines
  unchanged at 712 (the intro and pointer carry no quantity). No anchor link crossed the cut, and the one
  current-truth literal on the chapter (`#which-way-does-each-signal-point`) stays in place. The
  `shipped_behavior` aggregate change is authorised under this leaf; no ceiling or milestone moved. The root's
  open-leaf sentence also still named `.14e`, which the contract records closed on `2026-09-21`; removed.
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34b — move the accreting enum block, not the named candidate`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34c`
  Status: `open` (`2026-09-26`; opened by `EXTRACTION-GAP-FIX.6`)
  Goal: cut `docs/book/src/pipeline/isf-adapter.md` by reader concern again before a book edit is refused
  Acceptance: `.34b` left the chapter at 83,026 bytes (63.3%) on `2026-09-24`; it stood at **107,146 (81.7% of
  131,072)** at `b5446c76` two days later, so `shipped_behavior` warns on it again and no open leaf owns that
  warning. Rollover is 117,965. Apply `.34`'s rule as `.34b` did: attribute the growth per `##` section over the
  revisions since `.34b`'s cut, move the ACCRETING block whole under its own title with a `SUMMARY.md` route and a
  pointer left in place, and re-derive every pinned region at its new location before writing.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.42`
  Status: `open` (`2026-09-28`; opened by `EXTRACTION-GAP-FIX.9`)
  Goal: keep `doctrine/ownership_citations/citations.jsonl` writable before an ownership edit is refused
  Acceptance: the gate warns (80%) at **106 of 128 records** on `2026-09-28` and no open leaf owns it. Growth is a
  step, not a trend: ordinary commits move it by about one record (30 on `2026-09-16`, 59 on `2026-09-26`), and
  `EXTRACTION-REPRESENTATION-GAPS.36`'s `docs/gaps/` ownership regions added 45 in one commit (59 -> 104). Rollover
  (90%) is 116, so one more region of that size is refused. Decide from what a record is for: a `historical`
  citation stays as long as the text naming the unit stays, so records do not retire by age. Measure the per-region
  counts, then choose a derived capacity profile (ADR 0029: the bound from the part quantum, not a literal), a
  partition by declaring file, or a leaner record, and leave headroom for the next measured region.

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.43`
  Status: `done` (`2026-09-29`; DOCTRINE/CAPACITY; opened by `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f`)
  Goal: give the fact plane room for the next cards before a new card is refused
  Acceptance: on `2026-09-29` the plane is at rollover on two axes and warning on a third, and no open leaf owns
  any of them: `knowledge_cards` **385 of 394 files** (97.7%; 383 of `max_cards` 392 cards),
  `check_knowledge_map_shard_contract.pl` **414 of 449 facts** (92.2%) and **3,616 of 4,096 question keys**
  (88.3%). Cards arrive at about one per code slice (305 files on `2026-09-16`, 347 on `09-22`, 370 on `09-28`,
  385 on `09-29`), so the card axis is refused within days. `.24a` recorded that seven title parts is the last
  raise this bundle permits: an eighth part projects past the portable 4,096-key cap. Decide from what a card is
  for: consolidate cards that state one mechanism in several dated steps (losslessly, as `.1` did), move the
  portable cap in the `knowledge-map/` contract with its own derivation, or both, and leave measured headroom on
  every axis. Every join (catalog, shard contract, surface anchor, self-tests) moves in one transaction.
  **Decided and executed `2026-09-29` as ADR 0055.** Consolidation is ruled out by ADR 0026 decision 3, and an
  eighth part alone declares facts the key budget cannot fund, so the lever is the one ADR 0029 named: the
  bundle's caps. A fact costs 8.73 keys (3,616 / 414), so the budget funds it at the rollover band,
  `ceil(8.73 / 0.90)` = **10** keys per fact, stated once as `question_keys_per_fact` and derived everywhere
  else, fixtures included. The bundle's hard caps move to **1,024 facts / 10,240 keys**; shards, projection lines
  and projection bytes keep theirs. The laws in force give the rest: the card law (383 below 80%, a 25-card peak
  day below 90%) needs 479 slots, so `max_parts` 7 -> 9 and 504 cards / 506 files. The decision law, with
  ADR 0055 written (57 files, peak 9), needs 74 files. That makes 577 facts and 6,144 keys, and 42 shards at
  the measured 147 keys per 300-line shard (16,224 lines / 2,072,576 bytes). One transaction moves
  `check_fact_card_catalog.pl` (`fixed_limits`, the 506 anchor, the key constant), the two hard caps,
  `shard_contract.json`, `.knowledge_map.conf`, `fact_card_catalog.json`, the four surfaces, and four increase
  authorities that `.43a` retires. The next stop is named: 42 shards declare 2,072,576 of the 2,097,152
  projection bytes, so a 43rd shard (about 617 facts) needs the 49,152-byte shard bound lowered first. The
  projection-byte cap itself stays where it is, because the profile does not need it. The book states the raise as a
  reader summary and ADR 0055 carries the arithmetic. Written line by line, the arithmetic took nine regions and
  pushed `book_quantitative_claims.jsonl` past rollover (810 of 896), which `.44` owns. Summarized, it takes one
  region, leaving the registry at 802 (89.5%), so `.44` is next.
  Verification: `check_fact_card_catalog.pl --self-test` 60/60 and `--check` valid for 383 cards; `check_knowledge_map_shard_contract.pl --check` feasible for 415 facts / 3,623 keys in 25 shards with no band crossed (funded facts 703); knowledge-map derive-and-diff OK; LIVE-DOC-SIZE passes, and of the fact plane only the pre-existing per-file warnings remain (`decision_records` lines/bytes/width, owned by `.8`; `knowledge_cards` lines_each); RED: the key constant back at 8, the anchor back at 394, `max_parts` back at 7, and the 4,096-key hard cap each fail their gate; `book_quantitative_claims.jsonl` 802 of 896, the registry census still `at rollover 0`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.43 — fund a fact at its measured key cost; nine title parts`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.43a`
  Status: `done` (`2026-09-29`; DOCTRINE; opened by `.43`)
  Goal: retire the four increase authorities `.43` consumes
  Acceptance: the banked-authority refusal is observed RED on the real tree for `knowledge_cards`,
  `fact_card_titles`, `decision_records` and `fact_index` before the records are removed, then green after, with
  no ceiling, target or milestone moving, as `.24b` did for `.24a`
  **Executed `2026-09-29`.** Run on committed `1bc6e02b` before the registry was touched, the gate named all four
  surfaces as having unused or banked authority (exit 1, four violations), so retiring fewer would still be
  refused. The four `.43` records are removed and the registry is back to its meta record. The four surfaces'
  health and ceiling objects, captured before and after, are byte-identical.
  Verification: `RED at 1bc6e02b naming fact_index, decision_records, knowledge_cards and fact_card_titles; authorities 5 records -> 1; bands byte-identical across the removal; check_live_document_size.pl green at 1,231 Markdown files over 82 surfaces`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.43a — retire the four consumed fact-plane authorities`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.44`
  Status: `done` (`2026-09-29`; DOCTRINE; opened by `EXTRACTION-REPRESENTATION-GAPS.3.b.iii.f`)
  Goal: keep `doctrine/claim_verification/book_quantitative_claims.jsonl` writable before a book edit is refused
  Acceptance: the gate warns at **801 of 896 records** (89.4%; 298,635 of 393,216 bytes) on `2026-09-29`, five
  records below rollover, and no open leaf owns it. `.22d` sized 896 for 469 records plus one grammar widening
  plus 400 revisions of steady growth; the registry grew 471 -> 801 in the 171 commits since (`2026-09-16`), about
  two records per commit that edits the book, so the steady rate `.22d` assumed is spent. `.22d` kept 128 records
  of reserve below the loader's portable 1,024 cap so that a raise stays legal. Re-measure what a region record
  is for before spending that reserve: a record retires only when its candidate line leaves the manual, so decide
  between a lifecycle (records whose outcome is closed and whose line is frozen), a partition by chapter, or a
  derived raise with its authority, and leave headroom for the measured rate.
  **Decided and executed `2026-09-29`: a lifecycle, and no bound moves.** A raise cannot satisfy the sizing law:
  802 records below 80% needs about 1,003 slots, and the loader's portable cap is 1,024, so the declared bound
  would equal the cap with no band, the defect `.22d` refused. The lifecycle was already in the checker, unused:
  a region spans a line range and adjudicates every candidate inside it with its one outcome. The registry had
  used one region per candidate LINE. Measured, 263 of its 798 regions sat beside a region of the same Markdown
  block with the same outcome (the same exclusion reason, or the same missing legs). So the checker now refuses
  two such neighbours (`refuse_mergeable_neighbours`, with `block_ids` defining a block as a run of non-blank lines
  that no heading, list item, table row, block quote or fence begins). It compares neighbours in the order of
  every region, because a merge across a `registered` region would overlap it. `registered` and `derived` regions
  never merge. The new tracked writer `scripts/adjudicate_book_quantity_regions.py` covers new candidates (`--add`)
  and merges neighbours (`--compact`) under the same rule, replacing hand-written records. Compacted, the registry
  holds 536 regions for 802 candidates (540 records of 896, 60.3%). Every line any region covered before keeps
  exactly the outcome, reason, missing legs, claim and authority it had. Growth now follows paragraphs, not
  numerals.
  Verification: `self-test 22/22 (two new cases: same-outcome neighbours RED, different reasons clean; the denominator case re-aimed at 4); the first --check found 264 mergeable pairs; --compact 798 -> 535 regions; a per-line comparison of every covered line before and after shows 0 changed adjudications (803 lines before, 987 after, the extra lines non-candidate text inside merged paragraphs); one Python/Perl rule disagreement (a merge across two registered regions) found and fixed in the checker; --check green at 802 candidates / 536 regions; registry-capacity census still at rollover 0; claim mdbook-quantitative-census-frozen marker 20/20 -> 22/22 with the block rule added to its assertion`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.44 — one book-quantity region per paragraph and outcome`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.45`
  Status: `open` (`2026-09-29`; opened by `FSMGEN-REQUESTS.7`-`.10`)
  Goal: keep `docs/FSMGEN_FEEDBACK.md` inside its line target as requests to FSMGen accumulate
  Acceptance: the `fsmgen_feedback` surface's health target is 256 lines per file, and filing four packets on
  `2026-09-29` took the ledger from 225 to 274 lines (107%); the ceiling is 384. Open records stay open until FSMGen
  ships and SPECFORGE integrates, and ADR 0053 expects more concurrent requests, so the ledger grows about 11 lines
  per request with no lifecycle returning them. Its open-record cap moved 12 -> 16 the same day, on the same grounds
  as 8 -> 12 (`EXTRACTION-REPRESENTATION-GAPS.19`). Decide from what an open record is for: the ledger indexes the
  packets, whose README already holds the summary, so an open record may shrink to its fields and one line, or the
  open section may partition by status. Measure the per-record lines first, and leave headroom for the next
  batch of requests.

