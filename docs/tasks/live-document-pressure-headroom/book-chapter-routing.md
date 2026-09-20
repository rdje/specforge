# LIVE-DOCUMENT-PRESSURE-HEADROOM — book chapter routing

- Part ID: `book-chapter-routing`
- State: `legacy`

<!-- pressure-headroom-task-source-region:book-chapter-split-node:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.19`
  Status: `done` (`2026-09-13`)
  Goal: restore real headroom in the EvidenceIR book chapter, a second time, and by the same rule as the first
  Acceptance: content is reorganized by reader concern without losing examples, links, or public behavior; every
  moved line keeps its exact bytes; no bound moves; the book aggregate, route and quantitative authorities stay
  exact
  **It became a breach again, and the reprieve was refused again.** `.3` split this chapter on `2026-09-11`
  from a 131,583-byte breach to 100,739 bytes (76.9%). Six slices later — `EXTRACTION-QUALITY-GAUGE.3k.2a`
  through `.3k.2f`, each adding one section about one extraction rule — it was back at **130,413 bytes
  (99.5%)** at `0eac2799`, and `.3k.2e`'s book repair took it to **131,371 against the 131,072 ceiling**.
  `shipped_behavior` failed closed and blocked that commit. Trimming the new paragraph would have left the
  chapter at ~99.5% and broken the next book edit, which is exactly the reprieve `.3` names and refuses; the
  same reasoning applies unchanged, so the same remedy applies.
  **Two blocks, two reader concerns, and the second one is what `.3` did not take.** `.3` moved one subject and
  left the chapter carrying three: what the stage recovers, how one obligation sentence is read, and how
  extraction fails. The two that are separable as whole chapters move now:
  seven consecutive sections on reading a requirement — subject, negation, no-change spelling, passive verb,
  untyped kind, modal vocabulary, inter-operand bound — become
  `docs/book/src/pipeline/obligation-reading.md`, and the failure-mode catalogue becomes
  `docs/book/src/pipeline/evidence-failure-modes.md`. Both keep a pointer section in place and gain a
  `SUMMARY.md` route.
  **Moved byte-identically, deliberately, for the reason `.3` recorded.** No heading was promoted and no
  sentence rewritten inside either block, so a pinned quantitative region survives the move as a pure
  path+line re-point rather than a re-adjudication. **9 regions re-pointed to a new chapter and 9 re-anchored
  in the remaining one; zero lost, zero invented**, and the frozen candidate population is unchanged at 344.
  **Why it will recur, stated rather than left for the next reader to rediscover.** The chapter is the landing
  place for every extraction rule this repository ships, and `EXTRACTION-QUALITY-GAUGE` alone has fifteen open
  leaves. A split buys roughly six slices. The durable fix is a routing rule — a new extraction rule documents
  itself in the chapter that owns its concern rather than in the stage chapter — and `.20` owns deciding that.
  Verification: `pipeline/evidenceir.md` **131,371 → 100,885 bytes = 77.0%** of its ceiling;
  `pipeline/obligation-reading.md` 16,900 and `pipeline/evidence-failure-modes.md` 14,548, both far below it.
  Registry denominators moved `40 → 42` book files and `22 → 23` candidate files with the `book_summary`
  source pin refreshed; `perl scripts/check_book_quantitative_claims.pl --check` reports 42 files / 344
  candidates / 344 adjudicated regions; `scripts/check_doctrines.sh` all 13 gate-tier PASS, SECTION-ANCHORS
  included, so no cross-document route broke. No ceiling, milestone or bound moved.
  Commit: see log.

<!-- pressure-headroom-task-source-region:book-chapter-split-node:end -->

<!-- pressure-headroom-task-source-region:book-chapter-routing-node:start -->
- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.20`
  Status: `pending` (opened `2026-09-13` by `.19`)
  Goal: stop re-splitting one chapter by deciding where an extraction rule documents itself
  Acceptance: a stated routing rule, applied to at least the next extraction slice, that keeps
  `pipeline/evidenceir.md` describing the STAGE and sends a rule's own narrative to the chapter that owns its
  concern. The evidence this is structural rather than bad luck: `.3` split this chapter on `2026-09-11` and
  `.19` had to split it again on `2026-09-13`, six slices later, because every extraction rule lands in the
  stage chapter by default. Both splits were byte-identical moves of material that had accreted in the wrong
  place, and the second cost a blocked commit. The rule must be cheap enough to follow at authoring time —
  a one-line routing table in `COMMIT.md`'s documentation contract is the likely shape — and must not create
  a chapter per rule. Prerequisite: none; found by `.19` while executing the second split

<!-- pressure-headroom-task-source-region:book-chapter-routing-node:end -->

## Post-migration work

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.20`
  Status: `done` (`2026-09-20`; the sealed declaration above is its pre-migration stratum)
  Goal: stop re-splitting one chapter by deciding where an extraction rule documents itself
  **The rule shipped, and it is three rules rather than one, because the census changed the question.**
  `.19` asked where a new extraction rule should be written. The answer only holds once you know why the
  chapter grows, and the measurement says it does not grow all over: **`pipeline/evidenceir.md` was 104,483
  bytes and one section — *"Closed task trees — how each was implemented and verified"* — was 82,294 of
  them, 78.8%.** Every other section combined was 21 KB and stable, and **both earlier splits moved the
  tidy sections** and left the accreting one in place. That is the whole explanation for "a split buys six
  slices", and it is sharper than `.19`'s root cause, which named the chapter but not the section.
  **Why that section could only grow**: its subsections were named for the WORK UNIT that shipped them,
  which deletes the authoring question *where does this belong?* — every leaf simply gets another
  subsection, so the section has no shape that could ever be full.
  The three rules now in `COMMIT.md`'s documentation routing contract: **name a section for the behaviour
  it explains, never for the work unit** (the leaf id goes in the closing *Authoritative tracking*
  pointer); **extend the section that owns the reader's concern**, opening a new one only when none does;
  and **when a chapter must be split, move the section that is ACCRETING**, not the tidy ones.
  Acceptance, per leg: a stated rule — `COMMIT.md`, documentation routing contract; applied to at least
  the next slice — `SIGNAL-CATALOG-CAPTURE-GAP.3`'s section in `pipeline/evidenceir.md` lost its leaf-id
  heading in the same commit (``### `SIGNAL-CATALOG-CAPTURE-GAP` — a spec that declares its wires as
  section headings`` became `### A spec that declares its wires as section headings`), and `.37` applied it
  a second time by extending the section that already owned its concern instead of opening a chapter;
  no chapter per rule — rule 2 forbids it explicitly; cheap at authoring time — it is three lines in the
  contract a commit already reads.
  **What this leaf does NOT fix, stated so it is not read as fixed.** The rule is forward-looking. The
  headings already written stay: censused `2026-09-20`, **75 of 239 `###` headings in the manual still
  lead with a work-unit id**, 26 of them in `evidenceir.md` alone. Renaming them buys no bytes, because the
  bytes are the prose under them; the remedy for the existing mass is rule 3 applied to the accreting
  section, and that is `.34`'s work, not a rename pass.
  **`.34`'s open question is answered here, which is what it asked for.** `.34` asks whether
  `reference/live-docs.md` is `.20`'s second instance or a separate concern. It is the **same mechanism**:
  measured `2026-09-20`, its largest section — *"Containment and project-data locality are fully enforced"*
  — is **97,007 of the chapter's 124,084 bytes, 78.2%**, the same shape as `evidenceir.md`'s 78.8%, and it
  grows for the same reason: every lesson the containment doctrine learns lands in the chapter that
  describes containment. So `.34` executes rule 3 on it — **move the accreting section, not the tidy
  ones** — and does not need a new rule or a duplicate of this leaf.
  Verification: `2026-09-20` row in the current-chronology part's Verification Log
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.20 — close the routing rule and answer the chapter it hands to .34`
  Prerequisite: `.19`; unblocked by `.37`, which freed the root line its closure record needed

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34`
  Status: `done` (`2026-09-20`; declared here rather than in `current-and-open-work`, which is where it was
  filed by default and which is the part this tree keeps finding too full — see the note below)
  Goal: own the SIZE of `docs/book/src/reference/live-docs.md`, the `shipped_behavior` byte maximum, which
  was past its rollover and which no leaf owned
  Acceptance: the chapter returns below its warning band by a routing decision, not by deleting evidence
  and not by raising a ceiling.
  **`.20` answered the routing question and this leaf executed it.** `.34` asked whether this chapter was
  `.20`'s second instance; it is, so the rule applies unchanged: **move the section that is ACCRETING.**
  Attributed across the last 40 revisions of the file by **accumulating per-revision section deltas**,
  the +55,799 bytes decompose with `### Collection and projection closure order` at **+24,085**, the
  containment section's own preamble at +11,682 and the task-evidence-index section at +10,882. That block
  was also the largest thing in the chapter at **48,634 bytes**, so it is both the biggest and the
  fastest-growing — the unambiguous cut.
  **The figure is method-dependent to about half a percent, and `.34.1` says so rather than leaving an
  auditor to find it.** Differencing the two ENDPOINTS of the same window instead of accumulating
  intermediate revisions gives **+23,984 of +55,557** — the same ranking, the same >2x margin, the same
  decision, digits apart. Quote the method with the number.
  **What it is, which is why it moves cleanly.** The block is a chronology: eighteen `####` subsections,
  most of them named for a landing event (*"Task catalog migration landed"*, *"Bounded FSMGen feedback
  channel landed"*, *"Adoption program closed and verified"*). It is history, and the chapter around it is
  current doctrine. Splitting those two apart is a routing decision about narrative kind, which is exactly
  what this leaf's acceptance asked for, and it is why the cut will not have to be made again here: a
  chronology grows, a doctrine statement does not.
  **Moved byte-identically, and no heading promoted**, for `.19`'s reason: a pinned quantitative region
  then survives as a pure path-and-line re-point rather than a re-adjudication. The new chapter therefore
  opens `#` title, intro, then the block's own `###` heading — a deliberate one-level skip, traded for 100
  regions that did not have to be re-adjudicated.
  Verification: `live-docs.md` **124,084 -> 75,698 bytes, 94.7% -> 57.8%** of its ceiling (`.34.2`
  corrected the start value: 121,517/92.7% was the state before `.37`'s own book edit, so quoting it here
  credited this leaf with undoing a change another leaf made), and its largest remaining section is
  48,621 of 75,698 (64.2%); `reference/containment-adoption.md` is 49,293 (37.6%).
  **`shipped_behavior` no longer warns on bytes at all** — the new maximum is `pipeline/evidenceir.md` at
  104,450, 79.7%. Regions: **100 re-pointed to the new chapter, 12 line-shifted, 71 untouched, and every
  one of the 183 digests re-derived at its new location and required to match** before anything was
  committed; `repin_claim_regions.py --check` reports 683 unchanged and 0 absent. Registry denominators
  moved 42 -> 43 book files and 26 -> 27 candidate files with the `book_summary` pin refreshed; candidate
  lines are unchanged at 558, because the new intro and pointer carry no quantity. No ceiling, milestone or
  bound moved; the `shipped_behavior` aggregate change is authorised under this leaf.
  **One route followed the move and had to be repaired**: `check_book_current_truth.sh` required the
  literal `#### Adoption program closed and verified` in `live-docs.md`. The subsection still exists and is
  still true, so the check now requires it in the chapter that owns it and requires `live-docs.md` to carry
  the route to that chapter — `.30c`'s rule, that a pointer must name the location it resolves to.
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34 — move the accreting block out of the chapter it was filling`
  Prerequisite: `.20`, which decided the rule; `.33`, which opened this leaf

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34a`
  Status: `done` (`2026-09-21`)
  Goal: own `docs/book/src/pipeline/evidenceir.md`, which `.34` made the `shipped_behavior` byte maximum
  Acceptance: the chapter is below the warning band with the accreting section moved, or the axis is
  re-derived the way `.28` re-derived the fact plane's — by showing which bound actually binds. Do not
  raise a ceiling and do not delete a record; `.22`'s authority protocol applies if a bound must move.
  **Measured `2026-09-20`, and it is a stop this tree can see coming rather than discover.** With
  `live-docs.md` routed, `evidenceir.md` is the largest book file at **104,450 bytes, 79.7% of a 131,072
  health target that is also its enforcement ceiling** — three tenths of a point below the warning band,
  with **26,622 bytes** of room. Its own accreting section is the same shape `.20` and `.34` both found:
  *"Closed task trees — how each was implemented and verified"* is **82,900 of the file's 104,450 bytes,
  79.4%**, and `EXTRACTION-QUALITY-GAUGE` has open leaves that each land there by default.
  **So the remedy is known before the stop arrives**: rule 3, the same cut `.34` just made — the closed-tree
  chronology is history and the rest of the chapter is how the stage behaves. This leaf exists so that
  cut is scheduled rather than forced by a blocked commit, which is what happened to `.19`.
  **Taken `2026-09-21`, scheduled rather than forced — the first time this programme has cut a chapter
  before it breached anything.** `.3` and `.19` both cut `evidenceir.md` at a refusal, and `.19`'s cost a
  blocked commit. This one was taken at 79.7%, three tenths of a point below a warning, on a stop that
  `.34` predicted the day before.
  **The cut is the same shape, because the content is the same shape**: `## Closed task trees — how each
  was implemented and verified` is a chronology, one entry per rule that ships, sitting inside a chapter
  that describes what the stage *is* and does not grow with the work. It became
  `docs/book/src/pipeline/evidence-rule-chronology.md`. It was the last section in the file, so nothing
  after it shifted; moved byte-identically with no heading promoted, so all **29** pinned regions
  re-pointed by path and line alone and every digest was re-derived at its new location before the move
  was kept.
  **The relocation is stated rather than described as a removal** (`.2c`'s rule). The new chapter opens at
  **83,519 bytes, 63.7%** of the same 131,072 ceiling, and it is the accreting half — so the stop moved,
  it did not vanish. At the rate `.34` measured on the sibling chapter it is roughly 29 slices away
  instead of 16. **Trigger: at the 80% warning band — 104,857 bytes — cut again, and cut by reader
  concern**, because by then the clusters are visible: the register/bit-layout entries belong with
  `pipeline/register-tables.md`, the recall and completeness gauges with `quality/`, and that is rule 2
  rather than another chronology split.
  Verification: `pipeline/evidenceir.md` **104,450 -> 21,796 bytes, 79.7% -> 16.6%** of its ceiling;
  `pipeline/evidence-rule-chronology.md` 83,519 (63.7%). **The `shipped_behavior` byte maximum is now
  `pipeline/isf-adapter.md` at 68.3%, so no book file is in a warning band at all** — the first time that
  has been true since this tree opened. 29 regions re-pointed, 29 of 29 digests verified, 3 regions before
  the cut untouched; registry denominators 43 -> 44 book files and 27 -> 28 candidate files with the
  `book_summary` pin refreshed, candidate lines unchanged at 558; `check_book_current_truth.sh` green, so
  no currency literal followed this block out. No ceiling, milestone or bound moved.
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34a — cut the chapter before it breached, not after`
  Prerequisite: none; opened by `.34` on the measurement its own move produced

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34.1`
  Status: `done` (`2026-09-20`; opened the same day by the director's audit of this session's findings)
  Goal: re-derive `.34`'s published attribution by a second method and state the method beside the number
  Acceptance: every figure `.34` published is re-derived independently; any that does not reproduce is
  either corrected or has its derivation method stated, and the decision it supported is re-tested.
  **Four of `.34`'s figures reproduce exactly and one is method-dependent.** Re-derived by differencing
  the window's two endpoints rather than accumulating per-revision deltas:
  `### Collection and projection closure order` is **+23,984 of +55,557** against the published
  **+24,085 of +55,799** — about half a percent apart, from how the two methods treat intermediate
  churn. **The ranking, the >2x margin over the next block, and therefore the cut are unchanged.**
  Reproducing exactly: the root's own attribution (`146 -> 229` lines with Current Frontier **+39**, owner
  registry +18, both log windows +13 — matched digit for digit by endpoint differencing from the
  contract's declared `source.boundary_commit`); `live-docs.md` **124,084 -> 75,698** (see `.34.2`); and the heading
  census, where a stricter method — matching each heading's first token against the real set of
  `docs/tasks/*.md` tree ids rather than a capitals-and-hyphen regex — returns the same **75 of 239**
  with **zero** shape-only false positives.
  **Why this is worth a node rather than a silent edit.** `.5a`, closed the same day, refused a claim for
  not surviving an independent check. A five-significant-figure number published with no stated method,
  which a reasonable second method reproduces differently, fails the same standard in a smaller way. The
  repair is not a better number — both are right for their method — it is to **quote the method with the
  number**, so the next reader reproduces it instead of discovering a discrepancy.
  Verification: `2026-09-20` row in the current-chronology part's Verification Log
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34.1 — quote the method with the number`
  Prerequisite: `.34`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34.2`
  Status: `done` (`2026-09-20`; the second finding of the director's audit)
  Goal: attribute each leaf's measured effect to the leaf that caused it
  Acceptance: no published before/after span credits one leaf with a change another leaf made, and the
  task plane agrees with the registry on the same field.
  **`.34` published its own effect as `121,517 -> 75,698` bytes, `92.7% -> 57.8%`. The start value is
  wrong for this leaf.** Measured at each commit boundary: `live-docs.md` was **121,517 (92.7%)** before
  `.37`, **124,084 (94.7%)** after `.37` added the routing-rule section, and **75,698 (57.8%)** after
  `.34`. So `.34`'s own effect is **124,084 -> 75,698**, and quoting 121,517 credited it with undoing
  `.37`'s 2,567-byte addition as well as its own 48,386-byte move.
  **Two surfaces already disagreed, and nothing caught it.** The `shipped_behavior` aggregate-change
  rationale in `doctrine/live_document_size/surfaces.jsonl` — written in the same commit — correctly says
  *"drops from 94.7% to 57.8%"*. `PUBLISHED-ASSERTIONS` exists to refuse exactly this ("no two surfaces
  disagree on one field"), but its governed regions do not extend to task-tree prose, so a registry and a
  leaf could hold different values for one measurement with every gate green. That is a real coverage
  boundary of the gate, recorded here rather than left to be rediscovered.
  **The rest of the session's figures were re-derived at the same time and hold**: the root's
  `20,244 -> 8,257` bytes (82.4% -> 33.6%) and widest line `411 -> 113` (80.3% -> 22.1%);
  `containment-adoption.md` at 49,293 (37.6%); the largest remaining section at 48,621 of 75,698 (64.2%);
  `evidenceir.md` at 104,450 (79.7%) with its log section at 82,900 (79.4%).
  Verification: `2026-09-20` row in the current-chronology part's Verification Log
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.34.2 — attribute the effect to the leaf that caused it`
  Prerequisite: `.34`; found by the same audit as `.34.1`

- ID: `LIVE-DOCUMENT-PRESSURE-HEADROOM.38`
  Status: `done` (`2026-09-20`; opened and closed by the decision `.34.2` left to be made)
  Goal: decide what to do about the gate coverage boundary `.34.2` exposed, and own the remedy
  Acceptance: a stated decision with its reasoning, and a remedy that compliant work can actually take.
  **Decided: do NOT extend `PUBLISHED-ASSERTIONS` to task-tree prose.** A task tree is *dated history*.
  A closed leaf's number is correct as of the date on its row, and the live registries move on. An
  identity check between them would therefore refuse correctly-dated records as disagreements — which is
  `.25`'s rule (a gate that refuses most of its population on the day it arrives is a defect in the
  policy, not in the work) and `.29c`'s (a comparison is not pressure until the state is reachable),
  both already established by this tree. `PUBLISHED-ASSERTIONS` governs current-facing regions **on
  purpose**, and widening it to history would make it decorative — the `.18` failure mode.
  **The real defect is narrower than the gap it was found through**, and it is an authoring rule:
  `.34` hand-typed a before-value carried forward from a measurement two commits older instead of
  reading it at its own boundary. The remedy is in `COMMIT.md`'s documentation routing contract — *read
  the start value at `<this leaf's commit>^`* — which costs nothing at authoring time because the
  boundary commit is already in the record.
  **What is deliberately left standing**: two surfaces can still hold different values for one field if
  one of them is history. That is correct, and it is now stated rather than latent, so the next reader
  does not re-discover it as a defect.
  Verification: `2026-09-20` row in the current-chronology part's Verification Log
  Commit: `LIVE-DOCUMENT-PRESSURE-HEADROOM.38 — the remedy is an authoring rule, not a wider gate`
  Prerequisite: `.34.2`
