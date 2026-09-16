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
