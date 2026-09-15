# CLAIM-VERIFICATION-ADOPTION — count currency and grammar

- Part ID: `count-currency-and-grammar`
- State: `legacy`

<!-- claim-verification-task-source-region:count-currency-nodes:start -->
- ID: `CLAIM-VERIFICATION-ADOPTION.7.3`
  Status: `done` (`2026-09-14`, CODE/DOC)
  Goal: a shell check's SELF-TEST CASE COUNT is a published assertion, and nothing bound one to its script
  Result: **three were stale at once, under a fully green gate.** `TOOLBOX.md` §7.2a and §7.2a-i both said
  "sixteen fail-closed cases" for checks that had reached **22** and **21**, and `DOCTRINE_ENFORCEMENT.md`
  §10 repeated the first. They were found by hand while `CORPUS-CHAIN-CURRENCY.8` was editing the same
  section — which is the "gate is gated on the author noticing" shape `.13` describes, and the reason
  correcting the three numbers was not the fix.
  **What ships is the binding.** `scripts/report_self_test_totals.pl` emits one JSON object with the
  declared self-test total of each corpus-replay check, and three `derived` assertions in
  `published_assertions.jsonl` bind the `TOOLBOX.md` lines to it. The count is now published in exactly
  one place; `DOCTRINE_ENFORCEMENT.md` routes to that line instead of restating it, because a value
  stated in three places is a value that will disagree in three places.
  **The producer reads the DECLARED total rather than running the self-test, and that is a measured
  choice, not a shortcut.** `scripts/rebuild_stage_cascade.sh --self-test` builds the `specforge` binary
  and takes **43.6 s**, and `check_published_assertions.pl` executes every derived producer on every gate
  run — binding a doc sentence to a 43-second build would have moved the cost of the fix onto every
  commit. Reading the declaration is sound because the declaration is **self-guarding**: each script
  compares `passed` against that same constant, so a case added without bumping it goes RED in the
  script's own self-test, and a constant bumped without a case goes RED there too. The two legs compose
  and neither is a digest over the other (`CLAIM_VERIFICATION.md` §2).
  **Observed RED, and it is the real class rather than a fixture**: bumping `total=21` to `22` in
  `check_proof_seal_currency.sh` without touching the document fails the gate with
  *"'proof_seal_currency_self_test_total' re-derives to '22', published '21'"*. Reverted in the same
  measurement. The reader's own `--self-test` is **5/5** and includes the two fail-closed cases that
  matter: a script declaring two different totals, and a script with a `--self-test` mode whose total
  cannot be read — both are breaches rather than silent omissions, because dropping one would let a
  published count point at nothing while the report stayed valid JSON.
  **Honest limit:** the binding covers the three scripts a document actually publishes a count for. A
  fourth check that starts publishing one is not automatically governed — it needs a row in
  `%SCRIPTS` and its own assertion. That is a registry, with the same "enumerate the population"
  weakness every registry here has, and it is stated rather than hidden
  Prerequisite: none

- ID: `CLAIM-VERIFICATION-ADOPTION.9`
  Status: `done` (`2026-09-14`, CODE/DOC)
  Goal: stop the book quantitative census passing while it cannot see the numbers on the page
  Acceptance: `check_book_quantitative_claims.pl:is_candidate` decides what counts as a published
  quantity with one regex whose unit vocabulary is a **closed list** — `files`, `lines`, `bytes`,
  `records`, `members`, `facts`, `questions`, `shards`, `cases`, `tests`, `checks`, `surfaces`,
  `claims`, `fields`, `families`, `documents`, `pages`, `fixtures`, `diagnostics`, `commands`,
  `doctrines`, `signals`, `registers`, `artifacts`, `rules` — plus bare `N%` and `N/N`. Nouns the
  pipeline actually publishes in are absent: `items`, `elements`, `texts`, `bundles`, `figures`,
  `tables`, `assets`, `refs`, `batches`.
  Demonstrated (`2026-08-28`, during `SOURCE-IR-REPRODUCIBILITY.8`): five new current-facing
  quantitative lines landed in `docs/book/src/pipeline/sourceir.md` — "13,506 carried", "464 converter
  text items, 115 reaching a record before", "**370** after … **255 → 0**", "stays at **115** … 255
  diagram labels", "27 figures … the other 19" — and the frozen census reported **39 book files / 321
  candidate lines / 321 adjudicated regions and passed**, because not one of the five matches the
  vocabulary.
  **Premise correction (`2026-08-28`, same day):** this leaf was first written claiming the census
  "reports full coverage of a set it defines too narrowly", which is worse than it deserves and
  misstates an existing decision. `.3b.3.0` (`2026-08-15`) *declared* the grammar "a prose-only
  lexical candidate grammar as a **completeness alarm, not a semantic classifier**", and `.3b.3.3`
  states that `mdbook-quantitative-census-frozen` "verifies the census mapping, not the truth of its
  assertions". The tool therefore does not over-claim, and 321/321 is an honest statement about the
  mapping over its declared denominator. The finding that survives is narrower and still worth acting
  on: the alarm's noun list has a blind spot for the nouns this pipeline actually publishes in, so an
  editor adding five quantities to a governed chapter gets no alarm at all — which is the one job an
  alarm has.
  The fix is not simply a longer list, and this leaf must establish that before editing one: widening
  the vocabulary reclassifies existing prose as candidates, and every newly matched line needs its own
  adjudicated region in the same commit or the gate fails closed. So the work is (1) measure how many
  new candidates each added noun produces before adding it, (2) prefer a rule that does not enumerate
  nouns at all — a grouped or emphasized numeral in prose is the actual signal — and (3) adjudicate the
  resulting population, starting with the five above. A RED control must prove a quantity the current
  vocabulary misses is observed after the change
  **Second demonstration (`2026-08-29`, measured by `.6b`), and it is sharper than the first because it is the
  census's own chapter.** At `40acadb2`, `docs/book/src/reference/doctrine-enforcement.md` was 557 lines and
  published two stale current sentences — "56 exact evidence units: 11 derived, seven identity-gated, six
  registered, zero incomplete, and 32 excluded", and `regions=307` / `registered=8` / `incomplete=78` /
  `excluded=221` with `dated=186`. The frozen census held exactly **five** regions in that whole chapter —
  lines 31, 49, 88, 306, 540 — and since `--check` was green there, that is also the complete candidate set:
  **not one candidate is a line carrying those counts**. Two grammar gaps explain it: `units` is not in the
  closed noun list, and a backticked `key=value` form such as `regions=307` matches no clause at all. So the
  surface that documents the census is a surface the census cannot see. Duration measured from the tracked
  registry rather than estimated: `regions=307/78/221` was **correct when written** at `be3b12e6`
  (`2026-08-16`) and then moved **six** times — 308/222, 309/80/221, 318/89, 319/222, 320/223, 321/224 —
  settling at `fdda3c53` (`2026-08-28`). It went stale at the **first** of those, `4dac5642`
  (`2026-08-27 02:14`), and was repaired at `2b9e8899` (`2026-08-29 22:23`): **stale for two days and twenty
  hours**, under a green gate, while the contract honestly reported full coverage of its declared denominator. Same shape as `.7`'s
  fifth instance (a stale count outside the denominator), now with the vocabulary cause and the duration
  measured rather than inferred
  Eighth instance (`2026-08-29`, produced by `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c` one commit after `.6a`
  measured it): `.6a` carried `current_surfaces` **39** on the strength of a 29-revision trajectory in which
  it never moved, and the very next structural slice registered a surface and made it **40**. Nothing about
  `.6a`'s method was wrong — the trajectory was real — which is the point: **a trajectory shows what has not
  happened, never what cannot.** So "stable across N revisions" is evidence for *withdrawing later* rather
  than a licence to carry, and `.7`'s gate is the only thing that separates the two. All three publishers
  withdrew the value in the same commit that moved it, which is the discipline working
  Ninth instance (`2026-08-30`, found by the director asking a second time whether the findings were
  trusted): the drift class reaches **derived quantities**, not just carried ones. `.6b` published that the
  book chapter was "stale for twelve days across seven registry changes". Re-derived from the same registry
  blobs: **six** moves, not seven, and the chapter was *correct* until `4dac5642` (`2026-08-27 02:14`), so it
  was stale for **two days and twenty hours**, not twelve. The twelve-day figure was the span from *writing*
  to *drift settling* — a real quantity, silently substituted for the one the sentence names. Both numbers
  were computed by hand from a table that was itself correct, which is the lesson: a derived figure needs its
  own derivation command, or the producer's own output should be quoted instead of arithmetic over it
  Third demonstration of the vocabulary blind spot (`2026-08-31`, produced by `KG-ISF-COMPLETENESS.5.iv.a`
  and caught only because that slice looked): a draft book paragraph in `pipeline/isf-adapter.md` published
  "the rule accepts 285 tables in nine documents", and the frozen census still reported **325 candidate
  lines / 325 adjudicated regions and passed**. `tables` is one of the nouns this leaf already names as
  absent, and `nine` is spelled, so neither clause fires — two independent misses in one sentence. The
  author withdrew the count into the measurement record rather than register a region, which is the right
  interim move but is exactly the "gate is gated on the author noticing" shape `.13` describes. Worth adding
  to this leaf's measurement input: spelled numerals are a second gap, orthogonal to the noun list
  **CLOSED (`2026-09-14`). The alarm now sees the values every demonstration published, and the fix is
  three changes, each sized before it was made — which is what this leaf asked for and why it stayed open.**
  **(a) Four nouns, one per demonstration.** `items`, `units`, `tables`, `cells` are the nouns with a
  RECORDED miss. Cost: **13** new candidate lines. Speculative nouns were measured and NOT added — the
  full 19-noun set costs 61 lines to buy 48 that no demonstration ever asked for.
  **(b) Up to TWO words between the numeral and the noun, and this is the gap the noun list alone cannot
  close.** The fourth demonstration published "126 **name** cells"; adding `cells` does not catch it,
  because the noun never touches the numeral. The bound is measured, not chosen: 0 words catches **1 of
  4** demonstrated misses, 1 catches **2 of 4**, **2 catches 4 of 4**, and a third word adds 12 lines
  while catching nothing new. Cost of (a)+(b): **118** lines.
  **(c) A trailing word boundary on the unit group — a precision bug this leaf found by measuring.**
  `signals?` was matching inside "Gbps PHY **Signal**ing", `records?` inside "**record**ed once", and
  `checks?` inside "catalog **check**er". The boundary removes exactly **5** lines and all 5 are false
  positives of that shape. It also required stripping an ordered-list marker before matching, because
  with the gap in place `5. Dependency-connected questions` reads as a count of questions.
  **The "no nouns at all" option was measured and REFUTED as a replacement.** Four noun-free signals
  (thousands-separated numeral, bolded numeral, `N -> M`, `N of M`) select **119** lines against the noun
  route's 61, and they **overlap by only 7**: 112 are B-only and 54 are A-only. Neither subsumes the
  other, so a noun-free rule is a COMPLEMENT and not a substitute. That is the measured answer to this
  leaf's step (2), and it is why no noun-free clause ships here.
  **The population is adjudicated, not stamped.** 114 new regions: **112** `excluded`
  /`dated_boundary_evidence` (a measurement tied to a moment), **1** `incomplete` with all three legs
  named — `quality/kg-bench.md:567` "currently runs all 156 tracked fixtures" is a LIVE claim and is
  recorded as one rather than filed as history — and **1** `excluded`/`authored_threshold_or_choice` for
  an ADR-declared ceiling. **Two existing region records were DELETED**: they adjudicated false
  positives the precision fix removed, including a `3.` list index that had been read as "3 shards".
  Denominator **346 -> 458** lines across **25** files, 458/458 adjudicated.
  **RED observed on the real class.** The self-test gains a twentieth case that reads the GRAMMAR rather
  than a record, because the failure this leaf exists to stop is not a malformed record but a quantity
  the scanner cannot see. Reverting the gap to zero fails it by name on three of the four demonstrated
  lines. It also asserts what must NOT be minted: a date, an identifier fragment, and a list marker.
  **Honest residual:** three words between numeral and noun are still invisible, and spelled numerals
  ("nine documents") remain a separate gap this leaf measured but did not close. Both are asserted as
  bounds in the control rather than left for a reader to find
  **Fourth demonstration, and this leaf's own step (1) now measured (`2026-09-14`, produced by
  `TEXT-LAYER-IDENTIFIER-SPLIT.2`).** That slice added a paragraph to `pipeline/evidence-failure-modes.md`
  publishing "126 name cells" and "81 of them are TileLink's"; the frozen census reported **346 candidate
  lines / 346 adjudicated regions and passed**, unchanged, because `cells` is not in the list. The same
  chapter already carried "195 parametric width cells" ungoverned for the same reason, so this is not a new
  hole — it is the existing one, in the chapter that documents the class.
  **The measurement this leaf asks for before any noun is added, taken corpus-wide:** **527** book lines
  carry a `<number> <lower-case word>` clause the grammar does not see. That number is the *upper bound*, not
  the work — most of its words are noise (`and` 25, `are` 9, `such` 8, `with` 6), which is precisely why the
  list is closed. The real domain nouns inside it are few and countable: `constraints` 9, `tables` 7, `rows`
  6, `behaviors` 6, `statements` 6, `bits` 5, `cycles` 5, `invariants` 5, plus `cells`. So a curated addition
  of ~9 nouns is a bounded job with a knowable adjudication cost, and the "prefer a rule that enumerates no
  nouns" option must be measured against that 527 upper bound rather than against a guess. Reproduce with the
  regex in `check_book_quantitative_claims.pl:is_candidate` complemented by `<number>\s+[a-z][a-z-]{2,}`
  Prerequisite: none

<!-- claim-verification-task-source-region:count-currency-nodes:end -->
