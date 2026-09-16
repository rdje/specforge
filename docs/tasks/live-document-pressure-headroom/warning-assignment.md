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
