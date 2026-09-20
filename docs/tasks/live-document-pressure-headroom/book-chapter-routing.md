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
  Attributed across the last 40 revisions of the file, the +55,799 bytes decompose with
  `### Collection and projection closure order` at **+24,085**, the containment section's own preamble at
  +11,682 and the task-evidence-index section at +10,882. That block was also the largest thing in the
  chapter at **48,634 bytes**, so it is both the biggest and the fastest-growing — the unambiguous cut.
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
  Verification: `live-docs.md` **121,517 -> 75,698 bytes, 92.7% -> 57.8%** of its ceiling, and its largest
  remaining section is 48,621 of 75,698 (64.2%); `reference/containment-adoption.md` is 49,293 (37.6%).
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
  Status: `pending` (opened `2026-09-20` by `.34`)
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
  Prerequisite: none; opened by `.34` on the measurement its own move produced
  Verification: `pending`
  Commit: `pending`
