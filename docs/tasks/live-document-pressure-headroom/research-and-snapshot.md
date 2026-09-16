# LIVE-DOCUMENT-PRESSURE-HEADROOM — research and snapshot

- Part ID: `research-and-snapshot`
- State: `legacy`

<!-- pressure-headroom-task-source-region:research-and-snapshot-nodes:start -->
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
  Children: `.4a`, `.4b`, `.4c`, `.4d`, `.4e`, `.4f`
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
  Status: `done` (`2026-08-31`)
  Goal: release the per-record line stop the file-count exemption does not touch
  Acceptance: `docs/research/production-genericity-pipeline-audit.md` is **639 of a 640-line per-file
  ceiling**, again with `health_targets.lines_each == enforcement_ceilings.lines_each`, so a one-line
  correction to that record is refused. Decide from measurement whether the remedy is a lossless split at a
  section boundary (the partitioned-canonical remedy, which costs one catalog row and therefore needs `.4a`
  first) or a re-derived per-file profile, and preserve every byte either way
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4a`
  **Decided: split, not a re-derived profile.** The population supports the existing 640 and refuses the
  re-derivation: across the 63 members the mean is 172 lines, the median 130, p95 372, and only **two** records
  exceed 80% of the ceiling. A profile re-derived to fit one record would be a bound raised to quiet one
  warning — this tree's Non-Goal — and would spend a single-use `ceiling_increase_authorities.jsonl` record
  for no structural gain. The measured cause is not that the ceiling is wrong; it is that this record is
  **two documents**: a genericity audit, plus a per-leaf qualification chronology that thirteen `.6d.ii` leaves
  appended to it over three weeks. Splitting at that seam retires the growth driver instead of buying 120 lines.
  **The cut preserves the reader's question in place** (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`): the audit keeps
  its question, boundary, feasibility, denominator, method, confirmed violations, source disposition,
  forbidden-vocabulary rebuttal, required signoff architecture, historical correction, and exit criteria, and a
  new 20-line `Qualification outcome` section states the composed structural and behavioral verdict by
  derivation. Only the per-leaf results move, verbatim and in their original appended order — which is not
  strict leaf order, since `.e.iv.vii` was written after `.e.v.iii`, and reordering evidence is not lossless
  Verification: `two contiguous section-boundary blocks moved: the .6d.ii.d.iv-.e.v.iii chronology (audit lines
  44-165, 122 lines) and the .e.iv.vii/.f chronology (lines 550-619, 70 lines), into
  docs/research/production-genericity-qualification-results.md (212 lines, 17,332 bytes, max line 231 bytes).
  Losslessness proved mechanically against git HEAD, not by inspection: both moved blocks occur byte-identically
  and in order inside the new record (at its lines 20 and 143); the retained prefix and the 20-line suffix are
  byte-identical to the original; and zero of the 639 original lines are absent from both files. No inbound
  route breaks - all 14 inbound references cite the file as a whole with no anchor deep-links, and the four
  retained intra-document back-references ("the detailed findings above", "every P0/P1 item above") still
  resolve to sections that stayed. research_records lines_each 639/640 (99.8%, 1 line below a stop) -> the audit
  at 467/640 (73%), and the surface maximum relocates to a different record at 559/640 (87.3%) with an 81-line
  band; the research bytes_each warning clears. One catalog row added by
  perl scripts/check_canonical_collection_catalogs.pl --write (5 indexes, 265 members). Gate reports 902
  Markdown files / 57 governed surfaces, exit 0; scripts/check_doctrines.sh green`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c / CHANGES-LEDGER-ROLLOVER.6 — partition the composite genericity audit and roll the ledger it filled`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e`
  Status: `done` (`2026-08-31`)
  Goal: own the research-line frontier `.4c` relocated the maximum onto
  Acceptance: `.4c` removed the stop but did not leave the axis quiet, and the successor is not a dormant
  record. `docs/research/generic-enum-conflation-measurement.md` is **559 of 640 lines (87.3%)**, above the
  80% warning and 19 points below rollover, and unlike the audit it has a **live writer**: it is owned by
  `KG-ISF-COMPLETENESS.5`, an active lane whose `.5.iii` and `.5.iv` leaves appended to it as recently as
  `5c2fe11f`. An active measurement record 81 lines below a no-warning-band ceiling is the `LIVE-DOC-STOP-RISK`
  condition again, reached by ordinary compliant work. Decide the lifecycle-correct remedy before that lane's
  next measurement lands: either the same chronology/result partition `.4c` applied, or a declared per-record
  rollover the measurement lane can take. Do not raise the ceiling
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c`
  **Decided: partition, and the deciding measurement is the size of the remaining WRITER SET, not the fact
  that a writer is live.** The acceptance offered a declared per-record rollover as the alternative, and a
  live writer is what makes that alternative look necessary. It is not, here, and the reason is countable.
  `KG-ISF-COMPLETENESS.5` has exactly **one** unwritten leaf left — `.5.iv.a`, the CODE slice `.5.iv`
  deliberately deferred; `.5.i`, `.5.ii`, `.5.iii` and `.5.iv` are all `done` and the `.5` umbrella carries no
  other child. The record's own history bounds what that leaf can cost: seven commits took it 149 → 559 lines,
  six appends of **+72 / +105 / +38 / +96 / +33 / +66** (mean 68, max 105). A rollover lifecycle — new
  surfaces, checker logic, and an ADR against `decision_records` axes already at 82.8% files / 92.6% lines /
  97.1% bytes — would be built to serve one append. Partition is the surface's own `partitioned_canonical`
  remedy and it fits the residual writer set with margin: the successor opens at 428 lines, and the worst
  append this lane has ever produced lands it at 533/640 (83%), inside the band, after which `.5` closes and
  the record becomes ordinary immutable evidence. **This is the general rule the leaf establishes:** partition
  when the residual writers fit the successor's band; declare a rollover when the writer set is open-ended.
  The seam is the same composite seam `.4c` found — a `2026-06-24` measurement (defect, extraction-side
  origin, corpus census, member-quality finding, decomposed decision, reproducer, conclusion) followed by a
  chronology of four `.5.x` measurement/LANDED cycles appended over seven weeks
  Verification: `the partition is EXHAUSTIVE, which is a stronger proof than .4c's: the retained prefix
  (original lines 1-149) concatenated with the moved block (original lines 150-559, 410 lines) reproduces the
  committed HEAD file byte-for-byte under cmp, so no line is dropped, duplicated or reordered. Independently:
  sha256 of the base record's first 149 lines equals sha256 of HEAD lines 1-149
  (61a02615fc9a...); sha256 of the results record's lines 19-428 equals sha256 of HEAD lines 150-559
  (1999b496e29f...); and the multiset difference original-minus-(base union results) is 0 lines.
  research_records lines_each 559/640 (87.3%) -> measurement 176/640 (27.5%) and results 428/640 (66.9%); the
  surface maximum relocates to production-genericity-pipeline-audit.md at 467/640 (73.0%), below the 80%
  warning, so the research line warning CLEARS and the axis has 173 lines of band. Routes: 8 inbound
  deep-links whose target section moved were repaired to the results record - KG-ISF-COMPLETENESS
  .5.ii/.5.iii/.5.iv nodes and the .5.ii/.5.iii-MEASUREMENT/.5.iii-CODE/
  .5.iv enforced acceptance checklists - while the 3 whole-record citations (.5 node, .5.i checklist measure
  step, Changelog entry) correctly still name the measurement, whose cited content stayed. The dated Changelog
  bare-section refs are not rewritten; the measurement's new Outcome section routes them one hop. Fact card
  generic-enum-conflation repointed its five Report routes and gained a where-did-it-move answer key;
  research-record-size-profile records the writer-set rule. One catalog row by
  perl scripts/check_canonical_collection_catalogs.pl --write (5 indexes, 266 members). Prepending the ledger
  record shifted all 12 line-pinned CHANGES.md census regions, re-anchored by CONTENT and re-verified, and
  needed ONE NEW census row for the new ledger head - a row the previous rollover also had to add, so it is
  the rule and not an exception. The first staged measurement showed the resume pointer itself had crossed
  into warning at 41/50 lines; MEMORY.md was tightened to 36 rather than accepting a warning on a surface
  this slice does not own. Gate reports 905 Markdown files / 57 governed surfaces with NO research_records
  and NO active_resume warning; scripts/check_doctrines.sh reports ALL 12 executed doctrines PASS
  (13 registered, tier=gate; CHAIN-CURRENCY deferred to CI per the standing CI policy)`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e — partition the composite enum-conflation record at its chronology seam`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4f`
  Status: `done` (`2026-08-31`)
  Goal: repair the section-anchor regression `.4e` introduced, and make the invariant mechanical
  Acceptance: `.4e` claimed its routes were handled by repointing current-facing authorities and leaving dated
  ledger history to be routed "one hop" by the retained record's `Outcome` section. **A pre/post control
  disproves that claim.** Counting qualified `` `<path>.md` §`<section>` `` references across every tracked
  Markdown file: `caf448fa` (pre-partition) had **20 resolving / 0 unresolved**; `07eba8d8` (after `.4e`) had
  **13 / 14**. The repository's actual standing invariant was ZERO unresolved anchors, and `.4e` broke it
  fourteen times. The "current authorities versus dated history" split was reasoning invented to justify the
  gap, not the repository's practice
  **Repair belongs at the TARGET end, and that is forced rather than chosen.** Seven of the fourteen links sit
  inside sealed `docs/archive/rolling-ledgers/*` segments, whose lifecycle is `archive_terminal` and whose
  source `COMMIT.md` forbids editing ("Never edit a segment"). The source end of a link written by sealed
  history can never be repaired, so any remedy that only rewrites live roots leaves seven permanently broken.
  Keeping the cited headings resolvable at the target — as redirects when the content moved — repairs all
  fourteen and rewrites no dated entry
  **The deeper finding is that no gate could see it.** `.4e` ran a full `scripts/check_doctrines.sh` and got
  ALL 12 executed doctrines PASS while shipping the regression. `builtin:markdown_links` proves catalog
  MEMBERSHIP (an index links every member file); nothing proved a cited SECTION exists. A route rule enforced
  only by the author's care is exactly the "trust me" `DOCTRINE_ENFORCEMENT.md` exists to remove, so the leaf
  registers it rather than recording a lesson
  Prerequisite: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e`
  **Honest correction to this leaf's own account of `.4e` (`2026-08-31`, on the director's re-verification
  request).** The `CHANGES.md` record written with this leaf said `.4e` broke the invariant "while reporting
  'no inbound route breaks'". **`.4e` never wrote that sentence.** It belongs to `.4c`, and `.4e`'s own
  engineering note quotes it precisely in order to *reject* it — "`.4c` reported that no inbound route broke
  because all 14 references cited the file as a whole. That was luck, not a property of partitions." So `.4e`
  had already identified the hazard, repaired the eight deep-links it found, and disclosed that dated ledger
  entries were left alone. Flattening that into a false quotation made the predecessor look less careful than
  it was, and it is the same defect this leaf exists to correct — a characterization asserted from memory
  instead of re-derived from the record.
  **What `.4e` actually got wrong, stated exactly.** Its commit message asserted that dated ledger history was
  "not rewritten and routed one hop by the new `Outcome` section". That assertion was **false when written**,
  not merely unverified: the `Outcome` section pointed at the successor in prose, but the cited headings
  themselves had no target in the retained record, so no `§` link resolved anywhere. And because `.4e` never
  established the repository's baseline, it had no way to see that leaving fourteen links unresolved was a
  regression from zero rather than an inherited condition. **Missing baseline is the root cause; the
  unverified "one hop" is the symptom.**
  Verification: `docs/research/generic-enum-conflation-measurement.md gains a Moved sections block holding the
  six partitioned headings as redirects (176 -> 216 lines, 33.8% of 640); every one of the 14 broken links
  resolves again and the repository goes to 27 resolving / 0 unresolved, above its own pre-partition baseline
  of 20/0 because the 7 new routes into the results record resolve too. No dated ledger entry and no sealed
  archive segment was edited. scripts/check_section_anchors.pl registered as the SECTION-ANCHORS gate-tier
  doctrine in scripts/check_doctrines.sh and mirrored in DOCTRINE_ENFORCEMENT.md section 10 (new row 777
  bytes, below the 877-byte surface maximum, so workflow_standards line_bytes_each stays at its pre-existing
  85.6%). CONTROLS, run in detached worktrees on the repository volume: RED at 07eba8d8 -> FAILED with
  exactly 14 unresolved references, exit 1, so the checker would have blocked the very commit that shipped
  the defect; GREEN control at caf448fa -> 20 resolve, exit 0, so it is not a checker that merely always
  fails; current tree -> 27 resolve, exit 0; --self-test proves the normalized containment accepts a cited
  heading and refuses an absent section. Honest limit declared in the script and the section 10 row: 40 bare
  section references carry no path, are resolved from prose context, and are counted but not checked.
  scripts/check_doctrines.sh: ALL 13 executed doctrines PASS (14 registered, tier=gate)`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4f — repair the anchor regression .4e shipped and gate the invariant`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d`
  Status: `done`
  Goal: classify and remedy the validation-snapshot and README member warnings
  Acceptance: `validation_snapshot` lines_each (544 of 640) and `readme_entrypoint` line_bytes_each (108 of
  120, already at its rollover milestone) each get a lifecycle-correct local remedy or a measured reason the
  surface is healthy; no generic warning is merely suppressed and README stays inside `README_POLICY.md`
  **Split (`2026-08-31`) once both were measured.** The two members share a row in the gate's warning table
  and nothing else. `check_live_document_size.pl` computes every `_each` dimension as a per-surface maximum,
  so the split is not maximum-versus-total; it is what each maximum ranges over. `line_bytes_each` maximizes a
  per-*line* width, which does not grow as the document does: no growth driver, and a remedy that finishes
  inside the surface in one edit. `validation_snapshot` `lines_each` maximizes a per-*file* line count that
  does accumulate — here as O(reviewed corpus) against a constant bound — and its only lifecycle-correct
  remedy changes a Rust producer, a currency contract, and the registry. Holding both in one leaf would have
  let the cheap half stand in for the expensive one
  Children: `.4d.i`, `.4d.ii`
  Verification: `pending`
  Commit: `pending`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d.i`
  Status: `done` (`2026-08-31`)
  Goal: classify both member warnings by dimension kind, and clear the README one at zero cost to its other
  two bounds
  Acceptance: each warning is classified by what its dimension maximizes, with its measured driver, before any
  remedy is chosen; the README remedy moves no bound, deletes no word, and leaves `lines_each` and
  `bytes_each` numerically unchanged; the validation-snapshot finding is measured precisely enough that
  `.4d.ii` inherits a decided remedy class rather than a re-derivation
  Prerequisite: none
  **README is not growing; one bullet drifted.** `readme_entrypoint` declares `health_targets ==
  enforcement_ceilings` on all three per-file dimensions (the same no-band shape `.5` found on
  `active_resume`), so with `warning_pct: 80` the bands are: `lines_each` 150 warning at 120, `bytes_each`
  5,800 warning at 4,640, `line_bytes_each` 120 warning at 96. Measured: **118 lines (78.7%), 4,637 bytes
  (79.9%), widest line 108 (90.0%, at rollover)**. The width population is 86 non-blank lines, mean 52.5,
  median 56, p90 92, p95 94 — and **exactly two lines exceed 96 bytes**, L72 (108) and L73 (107), both inside
  a single five-line bullet. Every other line in the file is 95 bytes or less. The dimension did not drift
  upward with the document; one block was written wide.
  **The remedy is adversarial to the other two bounds, which is why it had to be line-neutral.** Reducing a
  maximum line width means inserting line breaks, and at a two-space continuation indent each break converts
  one `" "` into `"\n  "`: **+1 line and +2 bytes**. README had **2 lines** and **3 bytes** of headroom. A
  whole-file rewrap to a narrower column would have cleared one warning and tripped two. The safe operation
  is to reflow only the enclosing block, at the narrowest column that preserves its line count exactly — same
  separator count, therefore the same byte count. Measured boundary: **width 88 wraps to 5 lines / 423 bytes;
  width 87 spills to 6 lines / 424 bytes.**
  **Line-neutrality is a correctness property here, not tidiness.**
  `doctrine/claim_verification/current_claim_census.jsonl` pins README by **line number plus a SHA-256 of the
  pinned lines**, not by content search: L1 (document identity anchor), L30 (`rust_prerequisite_copies`
  derived value) and **L88-L104** (the entrypoint route block, whose verifier is
  `scripts/check_readme_policy.sh`). Any edit that changes the line count above a pin moves that region and
  stales its digest, dragging a claim-plane re-anchor into a slice that had no business touching it. All
  three digests re-derive **unchanged** across this reflow.
  **The validation snapshot is the opposite case, and it is not healthy.** Its per-document marginal cost,
  measured from the file rather than estimated: **AXI 163, AHB 147, AXI-Stream 112, APB 110** lines
  (its rescan-recommendation blocks, plus one `Projected Artifacts` block, plus one summary bullet), over
  **12** fixed lines — and 12 + 532 = 544 reproduces the measured total exactly. Headroom to the ceiling is
  **96 lines**, so **the cheapest fifth reviewed document is refused whichever document it is**. The band
  fails the doctrine's own rule at the warning too: warning to refusal is 641 - 512 = **129 lines**, less
  than the largest normal update (163) *before* any rollover transaction, where
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` requires room for the largest normal update **plus** the rollover.
  Even holding the reviewed set at four is bounded: at the measured **14.7 lines per rescan recommendation**
  (427 lines / 29), the surface admits about **6** more recommendations before it refuses.
  **And it is structural, not a near miss.** `generated/intent_ir/*/intent_ir.json` already holds **78** built
  artifacts against the **4** reviewed here, and `CORPUS-COVERAGE` exists to grow that set. At the measured
  133-line mean, 78 reviewed documents need roughly **10,400 lines** against a 640-line bound — 16x. The
  surface's size is O(reviewed corpus) while its bound is a constant, so **no ceiling number fixes it**; the
  remedy class is a bounded landing over per-document parts, the shape this repository already runs for
  `docs/task-catalog/`, `docs/knowledge-catalog/`, and the knowledge-map shards.
  **Why that remedy is not this slice.** The shape is emitted by `render_validation_snapshot_doc` in
  `crates/specforge/src/commands/project_validation.rs`, and
  `doctrine/live_document_size/validation_snapshot.json` pins four producer regions by digest alongside the
  file's own sha256/lines/bytes/line_bytes, required H2 order, and required literals. The reviewed content
  additionally may not be regenerated: `review_boundary.local_artifacts_authoritative` is `false` and
  `CANONICAL-PROMOTION-SWEEP` deliberately did not refresh it. A hand-cut partition would leave the file
  disagreeing with its producer, and the next legitimate refresh would silently un-partition it. That is a
  Rust change carrying an acceptance checklist, not a documentation edit. `.4d.ii` owns it.
  **Self-audit on the director's challenge (`2026-08-31`), before commit.** Four findings were published in
  the session report; **three carried defects**, corrected here and in the ledger record rather than left to
  stand. (1) The extremal/accumulating framing was WRONG AS STATED: `check_live_document_size.pl` computes
  `bytes_each`, `lines_each` **and** `line_bytes_each` alike as per-surface maxima, so the distinction is not
  maximum-versus-total but what each maximum ranges over — a per-file count that grows, versus a per-line
  width that does not. The conclusion survives; the reason given for it did not. (2) "There is no rollover
  transaction you can perform on a maximum" was TOO STRONG: a rollover *can* lower a maximum if the extreme
  member falls in the sealed part. What holds is narrower, and had to be re-derived twice: the first
  correction said `readme_entrypoint` "declares no `remedy` field", which is **true but vacuous** — no surface
  in the registry declares one. The checkable statement is that the only rollover transaction is the
  rolling-ledger protocol, whose registry names exactly four sources (`CHANGES.md`, `DEVELOPMENT_NOTES.md`,
  `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md` — the four `rolling_ledger` surfaces); README is a
  `bounded_snapshot` appearing there only as a reader, so it has no rollover, and where one does exist it acts
  only incidentally (the last `CHANGES.md` rollover left `line_bytes` at 1629 either side). (3) "Fourth instance of the stale
  hand-maintained-table class" was an OVERCOUNT: the tree records two priors and counts one commit's
  correction as one instance, so this is the **third**; and the container-never-in-frontier rule cited as
  "the file's own" belongs to `docs/TASK_TREE.md` line 201. One reply-only claim is also withdrawn: the
  ledger record was said to have been kept "to 1,994 bytes against the 1,992-byte budget", which measured the
  draft file rather than the record — the record is **2,337 bytes** and is the **63rd of 90** above that
  budget (`oversized_records` 62 -> 63). Finding 2 (the hair-trigger band) re-derived intact.
  Verification: `README.md 118 lines / 4,637 bytes UNCHANGED, widest line 108 -> 94 (90.0% -> 78.3%); words
  and order identical; lines 1-71 and 77-118 byte-identical. Width 88 preserves the block at 5 lines / 423
  bytes, width 87 spills to 6 / 424, so 88 is the exact line-count-preserving boundary. The three
  current_claim_census.jsonl README regions (L1, L30, L88-L104) re-derive UNCHANGED. Wrap population at HEAD:
  11 lines at 90-94, one at 95, then nothing until 107 and 108 - a clean gap, so ~95 is the document's own
  column and the 96-byte warning threshold sits one byte above it. Validation snapshot re-derived from the
  file: marginal 163/147/112/110 lines over 12 fixed, 12 + 532 = 544; headroom 96; cheapest fifth document
  refused; warning-to-refusal 129 < largest update 163; 78 built intent_ir.json against 4 reviewed.
  scripts/check_doctrines.sh: ALL 13 executed doctrines PASS (14 registered, tier=gate). Cost recorded
  honestly: CHANGES.md lines_each crossed its 80% warning (1433 -> 1455 of 1800) and the new ledger record is
  2,337 bytes against a 1,992-byte derived budget, the 63rd of 90 over it`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d.i — classify both member warnings; README's dimension does not accumulate`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d.ii`
  Status: `done`
  Goal: give the reviewed validation snapshot a bound that does not scale with the corpus
  Acceptance: `VALIDATION_SNAPSHOT.md` becomes a bounded landing (summary plus a complete index of reviewed
  documents) over per-document parts, **emitted by `render_validation_snapshot_doc` rather than hand-cut**, so
  the file and its producer still agree after the next legitimate refresh; the reviewed content is preserved
  byte-exactly across the partition and proved so mechanically, with no score, finding, or recommendation
  regenerated from local artifacts (`review_boundary.local_artifacts_authoritative` stays `false`);
  `doctrine/live_document_size/validation_snapshot.json` re-pins the landing, the parts, and the four
  producer regions, and a `validation_snapshot_parts` surface is registered with per-part and aggregate
  bounds; every moved `###` heading stays reachable from the landing as a redirect so `SECTION-ANCHORS` stays
  at zero unresolved; and the resulting landing admits a fifth reviewed document at the largest observed
  marginal cost (163 lines) while staying inside its warning band
  Prerequisite: `.4d.i`
  **Executed `2026-09-16` as a producer change plus a byte-exact partition, because the reviewed content
  may not be regenerated.** `VALIDATION_SNAPSHOT.md` **544 -> 20 lines** (85.0% -> **3.1%** of its 640
  bound), with the 29 rescan blocks and 4 projection blocks moved into
  `docs/validation-snapshot/<document_key>.md` at **166/150/114/113** lines. A fifth reviewed document now
  costs the landing **one line** instead of 110-163, and the surface stopped being O(reviewed corpus)
  against a constant bound.
  **Two things had to agree that could not be produced the same way.** The reviewed bytes are authority
  (`review_boundary.local_artifacts_authoritative: false`), so the file was PARTITIONED as text and the
  producer changed to emit the same shape — then the two were compared. Losslessness: **33 of 33** `###`
  blocks byte-identical against `git show HEAD:`, identical sorted-block digest, and the only changed
  landing lines are the four score bullets that gained a route, their score text untouched. Agreement: the
  producer run against a scratch root emits a landing whose shape differs from the partitioned one by
  exactly one score bullet, because the probe had 3 documents and the reviewed set has 4.
  **An independent oracle arrived unplanned.** `corpus_kb/failures/validation-findings.md` carries a managed
  block DERIVED from the projection records and compared byte-for-byte against the committed page. After the
  partition that block re-derives **unchanged**, which proves the records survived without relying on the
  partition script that produced them.
  **The consumer was the real risk, and it was found by running the gate rather than by reading.** Three
  readers parsed `## Projected Artifacts` out of the landing: `check_validation_snapshot_currentness.pl`,
  `check_corpus_kb_currentness.pl`, and `parse_reviewed_validation_snapshot` in `corpus_kb.rs` — the last
  one a product command, `specforge corpus-kb --validation-snapshot`, which would have failed on the new
  landing. All three now follow the routes the landing publishes, and the Rust one still accepts a
  pre-partition snapshot, because a snapshot written before the partition is still a reviewed artifact and
  refusing it would strand it.
  **Bounds are sized from the measurement, not from today's population**: `validation_snapshot_parts` is
  registered as a `generated_projection` collection at **96 files x 320/384 lines** with aggregates as
  `files x per-file` (ADR 0029), which is ~2x the largest reviewed document and leaves room for the 78
  built artifacts already standing behind the 4 reviewed.

### Acceptance Checklist (enforced) — `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d.ii`

- [x] **REPRODUCE / MEASURE** — `check_live_document_size.pl --report` before the change: `validation_snapshot`
  **544 of 640 lines = 85.0%**, per reviewed document 163/147/112/110 lines over 12 fixed (12 + 532 = 544
  exactly), and `generated/intent_ir/*/intent_ir.json` holds **78** built artifacts against the **4** reviewed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `render_validation_snapshot_doc` in
  `crates/specforge/src/commands/project_validation.rs` emitted every rescan and projection record inline, so
  the file's size is O(reviewed corpus) while its bound is a constant; no ceiling number fixes that shape.
- [x] **ADDRESSED (verified)** — landing **544 -> 20 lines / 63,628 -> 1,474 bytes**; parts 166/150/114/113
  lines; **33 of 33** `###` blocks byte-identical against `git show HEAD:`; the producer's own run reproduces
  the landing shape; `check_validation_snapshot_currentness.pl` and `check_corpus_kb_currentness.pl` both
  report current, the latter re-deriving its managed block unchanged.
- [x] **NO REGRESSION** — `cargo test` **473 + 168 + 1542 + 8 pass, 0 failed**; `cargo clippy --all-targets`
  and `cargo fmt --check` clean; `check_validation_snapshot_currentness.pl --self-test` **12/12** (10 -> 12,
  and its hardcoded `10/10` display replaced with the real total), `check_corpus_kb_currentness.pl
  --self-test` **15/15**; all **16** executed gate-tier doctrines PASS.
- [x] **GENERICITY (ADR 0006)** — the route is derived from `document_key` alone; no document, vendor or
  protocol name appears in any production decision, and the part path is computed by one function the landing
  link and the writer both call.
- [x] **LOCKSTEP** — `docs/book/src/quality/validation.md` and `quality/corpus-kb.md` described the snapshot as
  one inline section and now describe the bounded index plus its parts; `flow_census.json` re-derived through
  `aggregate_change` (**+5 functions / +34 decision sites / +13 helper edges / +4 semantic macros**, boundary
  counts unmoved); `validation_snapshot.json`, `corpus_kb.json`, `surfaces.jsonl` and the claim census
  re-pinned.

  Verification: `landing 544 -> 20 lines (85.0% -> 3.1%); 33 of 33 ### blocks byte-identical against git show HEAD: with an identical sorted-block digest; the only changed landing lines are 4 score bullets that gained a route; the producer's own run reproduces the landing shape and removes stale parts 3 -> 1; corpus-kb's derived managed block re-derives unchanged; cargo test 473+168+1542+8 pass with clippy and fmt clean; self-tests 12/12 and 15/15; all 16 executed gate-tier doctrines PASS`
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4d.ii — make the validation snapshot a bounded index over per-document parts`

<!-- pressure-headroom-task-source-region:research-and-snapshot-nodes:end -->
