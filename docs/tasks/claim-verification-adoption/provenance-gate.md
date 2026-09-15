# CLAIM-VERIFICATION-ADOPTION — provenance gate

- Part ID: `provenance-gate`
- State: `legacy`

<!-- claim-verification-task-source-region:provenance-gate-nodes:start -->
- ID: `CLAIM-VERIFICATION-ADOPTION.7.0`
  Status: `done` (`2026-08-30`)
  Goal: freeze the contract that ends the class, before writing the checker
  Acceptance: the design is derived from the ten recorded instances rather than invented, and every element below
  exists because a specific instance defeated everything else this repository has.
  **1. Bind a published value to a producer field, and execute it.** A registry record names the governed region
  (`path` + one-based line range + SHA-256 of those bytes), the exact `value` published there, an argv-form
  `producer`, and a `field` path into that producer's JSON report. The checker runs the producer, extracts the
  field, and compares. Instances 1–6 and 10 are all values that re-derived false while every existing control was
  green, because a digest proves a region has not changed and never that its number still re-derives.
  **2. Four honest outcomes, and only one of them is "carried".** `derived` re-executes and compares.
  `gated` names the control whose failure would follow the value moving, plus that control's known-bad case —
  `candidate_closure.unresolved` 0 is the model. `authored` names the decision record that fixes it — five
  `required_views` is the model. `dated` names the revision it is anchored to and is exempt while it stays
  anchored. Anything else is refused: there is no outcome for "a trajectory shows it has held", which is the
  licence `.10` retired and instance 8 disproved.
  **3. The population is derived at check time and never stored.** The checker enumerates governed regions itself
  and reports any published value inside one that no record lists, so the map cannot silently shrink. A stored
  surface list is forbidden, because instance 10 and `.11` both moved their own population by describing it:
  a commit that mentions a producer joins the set of surfaces citing it.
  **4. Self-reference is declared or refused — the rule `.11a` bought.** A record whose evidence population could
  include its own surface must set `excludes_self: true` and name the exclusion the producer applies. `.11`'s
  ownership screen turned green for the three surfaces it named *because it named them*; a check the act of
  writing satisfies is §2's shared-parent defect, and it is invisible unless the contract asks the question.
  **5. Cross-surface disagreement needs no producer.** Two records naming the same `producer` + `field` with
  different `value` fail immediately. Instance 5 is the case: two surfaces published different values for one
  quantity and nothing noticed, and this leg costs one comparison.
  **6. Membership tests are derived from the producer, not from a description of it.** A record whose `field`
  names a set must carry the enumerating command, and the checker compares the enumeration, not its size.
  Instances 4, 5 and `.11a` are all set claims published without their enumeration or with a stale one, and
  `.9`'s closed noun list is the same defect in the candidate grammar.
  **7. Mechanism claims are explicitly out of scope, with the reason recorded** (`.10`): no checker can decide
  whether two accounts predict the same observation. `.11a` measured what that costs — two of its four self-caught
  defects were mechanism claims, and all were found by a second reader, not a rule. The registry therefore carries
  an optional `adjudicated_against` field naming the prior ruling a mechanism claim was checked against; shape,
  not truth, which is what this repository's gates do well.
  **8. The RED matrix `.7.1` must observe.** A drifted value; a value bound to the wrong field; a published value
  in a governed region that no record lists; a `gated` record whose named control has no known-bad case; a
  self-referential record without `excludes_self`; two records disagreeing on one `producer` + `field`; a
  membership record whose enumeration drifted while its size held. The last is the one that separates this gate
  from a numeral scanner
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.11a`
  Verification: `design derived from the ten instances recorded in this tree, each element traced to the instance
  that defeated the alternative; no code, registry, or gate wiring in this slice; census/book/claim --check and
  --report green and unchanged; doctrine gate`
  Commit: `CLAIM-VERIFICATION-ADOPTION.7.0 — freeze the published-assertion gate design`

- ID: `CLAIM-VERIFICATION-ADOPTION.7.1`
  Status: `done` (`2026-08-30`)
  Goal: implement the gate and prove every RED case in `.7.0`'s matrix
  Acceptance: `scripts/check_published_assertions.pl` plus a self-bounded
  `doctrine/claim_verification/published_assertions.jsonl`, registered through `scripts/check_doctrines.sh`
  without duplicating hook or CI wiring. The checker never evaluates a shell string; producers are argv arrays;
  every region, producer and input is Git-tracked and digest-bound. `--self-test` instantiates every outcome
  family positively and drives all seven `.7.0` faults RED on a disposable repository-local fixture
  **Delivered.** The checker executes the named producer and compares its report field to the literal published
  in the governed region — the leg no existing control supplied. The four outcomes are closed: an outcome outside
  `derived`/`gated`/`authored`/`dated` is refused, so there is no expressible way to carry a value because a
  trajectory shows it has held. `gated` requires an exact known-bad region inside the named control; `authored`
  requires a tracked decision path; `dated` requires a revision that resolves through `git rev-parse`.
  Cross-surface disagreement fails before any producer runs. A `membership` field is compared as an enumeration
  and reports what is unexpected and what is absent, never a size. `excludes_self` is refused when declared
  without a membership, and required when the enumerator returns the record's own publishing surface.
  **The coverage grammar was corrected during implementation, and the correction matters.** A first version keyed
  governed regions on the line carrying the `[claim: <id>]` tag. Authors put that tag at the **end** of a
  paragraph, so the values it covers sit above it: keyed that way, the map would have been blind to exactly the
  sentences it exists to watch — `TOOLBOX.md`'s two annotated tags sit on lines carrying no quantity at all. The
  governed region is now the paragraph the tag closes, and inline code spans are excluded because a backticked
  literal is an example rather than a published quantity.
  **RED observed on real shipped prose, not only on fixtures.** By revert-and-re-apply: a probe record bound
  `TOOLBOX.md`'s `**5** views` to `check_current_claim_census.pl --report` field `views`, green at 5; the prose
  was then edited to `**6**` and the checker reported
  `assertion 'census-views-derived-probe' is stale: 'views' re-derives to '5', published '6'`; both files were
  restored byte-exact and the probe removed. A first attempt used `docs/knowledge/INDEX.md`'s card count and is
  recorded because it failed honestly: `check_fact_card_catalog.pl` is derive-and-diff over that file, so it
  exited nonzero before the field comparison ran. That is a producer whose report is not stable under prose
  drift, and it is the wrong instrument for this leg
  **The gate caught a live drift inside this very commit, unprompted.** Adding this leaf's own Knowledge Map
  fact card moved `check_fact_card_catalog.pl --report`'s `card_count` **249 -> 250**, and the seeded record
  bound to `docs/knowledge/INDEX.md:3` still published 249. All three legs fired at once — the region digest went
  stale, the value no longer appeared in its own region, and the comparison reported
  `is stale: 'card_count' re-derives to '250', published '249'`. That is the exact same-transaction shape of
  instances 6 and 10, which ten recorded rounds and five prose corrections could not observe, seen at commit time
  by a control rather than by a reader. Re-deriving and refreshing the record to 250 is **not** the anti-pattern
  §3 Leg 3 forbids: the value is now watched, so the distinction that rule draws — between a right unwatched
  number and a gated one — is the whole point of the record.
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.0`
  Verification: `--self-test 16/16 (positive x2, drift, wrong-field, unlisted, missing control region,
  self-reference, cross-surface disagreement, enumeration drift at constant size, stale region digest, untracked
  producer, refused fifth outcome, orphan excludes_self, duplicate id, value absent from its own region, bound
  breach); --check green on the real tree at four seeded assertions over two governed regions in inventory phase;
  drift observed RED on real shipped prose by revert-and-re-apply with both files restored byte-exact; and an UNPROMPTED live catch inside this commit when the new fact card moved card_count 249 -> 250; doctrine
  driver registers the twelfth entry and runs it through the existing hook/CI wiring; DOCTRINE_ENFORCEMENT.md
  §10 and the mdBook chapter both gain the row in the same commit`
  Commit: `CLAIM-VERIFICATION-ADOPTION.7.1 — execute the producer and compare the field`

- ID: `CLAIM-VERIFICATION-ADOPTION.7.1a`
  Status: `done` (`2026-08-30`)
  Goal: correct the coverage grammar's measured blind spots before `.7.2` closes the population at zero
  Acceptance: the numeral grammar decides which published values *exist*, so a value it cannot see is worse
  than an unlisted one — no record can be asked for it, no `--produce` run can report it, and `frozen` phase
  will call the surface complete anyway. Closing the unlisted report at zero is therefore meaningless while
  the population is wrong, which is why this precedes `.7.2` instead of following it. Measure the blind spot
  against an independent tokenizer rather than by reading the regex, repair only what the measurement shows,
  and bind each repair to a RED case
  **The oracle was built to disagree with the producer, not to agree with it.** A probe re-enumerated the same
  governed paragraphs the gate enumerates, kept the gate's own lookbehind, and widened only the lookahead, so
  every difference it reported is a value the gate drops rather than a value the probe over-reaches for. A
  first probe without the lookbehind was discarded because it reported `SHA-256` and `H1` — digits inside
  identifiers, which the gate correctly refuses — and an oracle that reports the control's correct behaviour as
  a defect cannot separate the two hypotheses (`CLAIM_VERIFICATION.md` §3 Leg 2).
  **Defect 1 — sentence punctuation absorbed into the value.** `\d[\d,]*` treats every comma as part of the
  numeral, so `current_surfaces` 39 -> 40, noticed ... published the token `40,`. That token is not a value
  the prose publishes: an honest record listing `40` would have been reported unlisted, and a record listing
  `40,` would have bound the gate to punctuation. A comma now belongs to a numeral only when it separates
  exactly three digits, which keeps `1,922` one value and ends `40` at the comma.
  **Defect 2 — a numeral closing a compound adjective or a ratio was invisible.** The lookahead excluded every
  `-` and `/`, so `27-case`, `29-revision`, `56-unit`, `304-region` and `15/15` were dropped entirely. The
  exclusion exists to keep dates and identifiers out — `2026-08-28`, `segment-0013`, `SHA-256`, `1.95.0`,
  `.7.2` — and it is now narrowed to `-` followed by a **digit**, which is the form every one of those
  actually takes. The lookbehind is untouched, so a numeral glued to a preceding word, dot, slash or hyphen
  stays an identifier fragment.
  **What the blind spot cost, measured (`2026-08-30`).** Across the four claim-annotated files the gate
  dropped 19 published values: 16 compound-adjective forms, two ratio halves, and one comma over-capture. Two
  of them sit on the two *current-facing* governed surfaces, and one of those is live: `TOOLBOX.md`'s "The
  27-case self-test" is a current count of `check_current_claim_census.pl --self-test`, inside a
  claim-annotated paragraph, and the gate written to watch that paragraph could not see it. The real-tree
  unlisted report moves 21 -> 23 as a direct result.
  **Attribution is by revert-and-re-apply, not by reading the diff.** With the grammar line alone reverted and
  the rest of the slice identical, 5 of 19 self-test cases fail — both positive cases and all three new ones,
  the positives on `published value '40,' ... no assertion record lists`. Re-applied, 19/19. The
  absorbed-punctuation case is the mirror the repair needs: a record whose `value` is `40,` now covers nothing,
  where before it was accepted
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.1`
  **The measuring instrument was disposable; the control that replaces it is tracked.** The widened-lookahead
  probe lived in `.project-data/tmp/` and is deleted, because an oracle that exists only under ignored state
  cannot support a claim (`CLAIM_VERIFICATION.md` §9). It was the *discovery* instrument, and what it found is
  now encoded in three tracked RED cases on the self-test's repository-local fixture — a compound-adjective
  value, a ratio-form value, and a record whose value absorbed punctuation — each observed failing before the
  repair and passing after it. The probe is named here for reproducibility, not cited as durable evidence.
  **This leaf's own book edit moved the mdBook census, and the first draft of this line said it had not.** The
  new chapter paragraph contains `15/15`, which the book census's candidate grammar matches, so `regions`
  moved 321 -> 322 and `authority_outcomes.excluded` 224 -> 225. The new candidate is adjudicated `excluded`
  with scope reason `example_or_command_literal` — it is an illustration of the grammar, not a published
  quantity — and the `mdbook-quantitative-census-frozen` marker and result were re-derived from `--report`
  rather than edited by arithmetic. Recorded rather than smoothed over: it is the same-transaction shape this
  tree has now seen eleven times, and it was caught by re-running the gate, not by re-reading the sentence
  Verification: `perl scripts/check_published_assertions.pl --self-test 19/19 (the 16 .7.1 cases plus
  compound-adjective, ratio, and absorbed-punctuation, the last three proven grammar-dependent by
  revert-and-re-apply at 14/19); --check green on the real tree, unlisted 21 -> 23 with 29 at TOOLBOX.md:118
  and 27 at TOOLBOX.md:130 newly visible; check_current_claim_census.pl --check green at 71 frozen evidence
  units after one new rolling-ledger head row and 20 line-map re-pins; check_book_quantitative_claims.pl
  --check green at 322 regions with the new candidate adjudicated excluded; check_claim_verification.pl
  --check resolves 6 claims and 12 commands; scripts/check_doctrines.sh 11/11 executed doctrines PASS`
  Commit: `CLAIM-VERIFICATION-ADOPTION.7.1a — measure the coverage grammar's blind spot, then close it`

- ID: `CLAIM-VERIFICATION-ADOPTION.7.2.0`
  Status: `done` (`2026-08-30`)
  Goal: replace the stored scope list with a fail-closed derived one, before any value is written down
  Acceptance: `.7.1a` measured that `governed_globs: ["TOOLBOX.md"]` is the stored surface list `.7.0`
  element 3 forbids. Freezing on it would have made exactly one surface fatal and left
  `docs/book/src/reference/doctrine-enforcement.md` — the surface instance 5 actually drifted on — outside
  the map entirely. Replace it with a rule under which membership is discovered and only a **surface's
  disposition** is authored, and prove the new failure modes RED before populating anything
  **What element 3 actually wants is fail-closed, and that is what a glob list cannot give.** The checker now
  discovers every tracked Markdown file carrying a `[claim: <id>]` tag, resolves each to the live-document
  surface that owns it, and **errors** if that surface declares no disposition or if no surface claims the
  file at all. So a new claim-annotated file joins the map by itself, and nothing leaves the map without
  someone writing down which surface it left through. A glob list fails open in exactly the direction that
  matters: a file nobody listed is a file nobody checks, and nothing says so.
  **The classifier is derived from the producer, not from a description of it** (`CLAIM_VERIFICATION.md` §3
  Leg 2). Surface membership comes from `doctrine/live_document_size/surfaces.jsonl`, the registry that
  already owns which surface a path belongs to, bound by a `source` record so the scope cannot drift under a
  stale identity. A disposition naming a surface that registry does not have is refused, so a typo cannot
  quietly govern nothing.
  **Two exemptions, each with its reason, because an exemption is how a population legitimately shrinks.**
  `task_evidence` is exempt: every claim-annotated region on it is a verification-log row whose first column
  is the date it was measured, which §1 exempts as an observation scoped to its original boundary.
  `change_history_archive_segments` is exempt: a sealed segment is an immutable capture whose currentness is
  not asserted, and the rollover protocol forbids editing one, so a value inside it cannot be re-derived in
  place. `workflow_standards` and `shipped_behavior` are governed. An exemption without a stated reason is
  refused.
  **A lifecycle-based rule was tried first and does not work — recorded so it is not tried again.** The
  obvious derived predicate is the live-document `lifecycle` field, but `TOOLBOX.md` and this task tree share
  one (`partitioned_canonical`), so lifecycle cannot separate a governed surface from an exempt one. Reusing
  the census's own `disposition` fails differently: it marks `task_evidence` **included**, which would put 85
  dated verification-log values in scope and exceed the registry's own record bound. Both are honest
  candidates that a reader would reach for, and both are wrong for a stated reason rather than by omission
  **The exemption is proven load-bearing, not incidental.** A green run with an exempt file's value
  unreported is equally consistent with that file never having been discovered — evidence consistent with
  both hypotheses is no evidence. So the matrix flips one surface from `exempt` to `governed` and requires
  the very same value to become fatal
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.1a`
  Verification: `perl scripts/check_published_assertions.pl --self-test 25/25 — the 19 .7.1a cases plus an
  undeclared surface, a file no surface owns, a reasonless exemption, a disposition naming an unknown
  surface, a stale surface-registry source digest, and the load-bearing-exemption flip; --check green on the
  real tree at 2 governed and 2 exempt claim-annotated files over 6 governed regions, unlisted 23 -> 27
  because the book chapter is now in scope where the glob list had excluded it; scripts/check_doctrines.sh`
  Commit: `CLAIM-VERIFICATION-ADOPTION.7.2.0 — derive the governed scope, and fail closed on an undeclared surface`

- ID: `CLAIM-VERIFICATION-ADOPTION.7.2.1`
  Status: `done` (`2026-08-30`)
  Goal: populate the registry from the current governed regions and close the unlisted-value report at zero
  Acceptance: every published value in a claim-annotated or registered governed region resolves to one record
  with a closed outcome, or the run reports it. The three survivors this tree already labelled are the first
  entries — `candidate_closure.unresolved` `gated`, `views` `authored`, and the producer-census zeroes `gated`
  **Scope is settled by `.7.2.0` and is no longer this leaf's problem.** What remains is the population.
  Two TOOLBOX sentences cannot honestly take a record as written and must be repaired first, not recorded:
  "the 27 transitions measured from `e6f5012d`" names an **open-ended** window — measured `2026-08-30`, 39
  commits now separate `e6f5012d` from `HEAD`, so the sentence's own boundary has moved since it was written
  and a `dated` record would anchor it to a revision it does not describe; and "The 27-case self-test" is a
  **live** count of `check_current_claim_census.pl --self-test` which no closed outcome fits, because that
  producer reports its case count only as prose on stderr, so it is neither `derived` nor `gated` today.
  *(b) the bound.* Measured `2026-08-30` with `.7.1a`'s corrected grammar and every surface governed: 152
  unlisted values over 32 governed regions in four files, against a declared `max_records: 128`. Under
  `.7.2.0`'s two exemptions the governed population is far smaller and fits; if a later slice governs a
  further surface, raising the bound to make it fit is the move `.8` refuses for the census registry.
  Re-derive both figures from `perl scripts/check_published_assertions.pl --produce` rather than carrying
  them; they are recorded here as a dated measurement of the decision, not as current state
  **Two sentences were repaired rather than recorded, because no closed outcome fitted them as written.**
  *(1) A window with no end.* "the 27 transitions measured from `e6f5012d`" named only where the measurement
  started, so a `dated` record would have anchored it to a revision the sentence does not describe — and the
  boundary had already moved, because `e6f5012d..HEAD` names a different window every time it is read.
  (**`.7.2.1a` withdrew a count from this sentence.** It said "39 commits now separating `e6f5012d` from
  `HEAD`", which was true when written and false four commits later. Count today's with
  `git rev-list --count e6f5012d..HEAD`; the point was never the number.) The task tree holds the closed
  window (`.6a` measured 28 consecutive revisions `e6f5012d` -> `60a81db7`, which is 27 transitions, and
  15 + 2 + 10 = 27 checks out), so the repair names the end revision and the value is then honestly `dated`.
  *(2) A live count no outcome fits.* "The 27-case self-test" is a current count of
  `check_current_claim_census.pl --self-test`, and that producer reports its case count only as prose on
  stderr — so the value is neither `derived` (no JSON field to compare) nor `gated` (nothing fails when it
  moves). The numeral is removed and the reader runs the command, which is the remedy the same paragraph
  already applies to every other census figure. Removing a value is a legitimate repair rather than the
  right-unwatched-number anti-pattern of §3 Leg 3: that rule forbids replacing an unwatched value with
  another unwatched value, and nothing is stale about a number a surface no longer publishes.
  **A near-contradiction was adjudicated rather than "corrected".** `TOOLBOX.md` says `.6a`'s trajectory was
  **29** revisions and the book chapter says **28** consecutive revisions. Both are right, and the tree says
  why: 28 consecutive revisions `e6f5012d` -> `60a81db7` **plus** the older `50775894` anchor `.6` had cited,
  which is 29 measurements. Neither was touched; the earlier ruling wins because no difference could be named
  against it (`CLAIM_VERIFICATION.md` §3 Leg 2, project history as the cheapest oracle).
  **All 26 remaining values are `dated`, and that is the honest outcome rather than the convenient one.**
  Every one is an observation the surrounding sentence explicitly anchors to a named revision — `.6a`'s
  trajectory at `40acadb2`, the `current_surfaces`/`identity_gated` moves at `d23e8bae`, the ledger-transition
  split at `60a81db7`, and the registered/closure vectors at `fdda3c53`, `5fe81128` and `1507adbf`, plus the
  book's `.3b.4` boundary at `50775894`. `dated` is the weakest of the four outcomes — the checker proves only
  that the revision resolves — so it is worth stating that this is what these sentences are: a history of how
  the counters moved, published to teach that they move, not a claim about the tree today.
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.2.0`
  Verification: `perl scripts/check_published_assertions.pl --check green in **frozen** phase at 30 assertions
  — 1 derived, 2 gated, 1 authored, 26 dated — over 6 governed regions in 2 governed and 2 exempt files, with
  **0 unlisted**; --self-test 25/25; three probes on REAL SHIPPED PROSE by revert-and-re-apply, each restored
  byte-exact: an unmapped numeral added to a governed paragraph produced `published value '41' at
  TOOLBOX.md:122 ... no assertion record lists`; editing a dated value 39 -> 38 fired three legs at once
  (value absent from its own region, stale region digest, and a new unlisted value); and a claim-annotated
  file staged under `research_records` produced `belongs to surface 'research_records', which declares no
  disposition`. census/book/claim --check green; scripts/check_doctrines.sh`
  Commit: `CLAIM-VERIFICATION-ADOPTION.7.2.1 — close the population at zero and freeze`

- ID: `CLAIM-VERIFICATION-ADOPTION.7.2.1a`
  Status: `done` (`2026-08-30`)
  Goal: re-derive `.7.2.1`'s published findings on a third reading, and correct the three that do not hold
  Acceptance: the director asked whether the findings still hold. This tree's answer has been "not entirely"
  every time it has been asked — `.6` -> `.6a` -> `.6b`, `.10` -> `.10a`, `.11` -> `.11a` — so the obligation
  is to re-derive with a command, never to re-read the claim. Each figure is re-derived at the revision it
  describes, and anything that does not reproduce is withdrawn rather than quietly restated.
  **Two findings re-derive exactly.** The blind-spot measurement was reproduced from each file's content at
  `a88b91a3~1` with the old grammar against a widened lookahead, lookbehind held constant: **19 dropped, 16
  compound-adjective, 2 ratio, 1 comma over-capture** — the published decomposition, to the value. And the
  scope finding holds: at `3c17ae5c~1` the registry's `governed_globs` was exactly `["TOOLBOX.md"]`, the book
  chapter carried four claim tags, and none of the four assertion records pointed at it, so freezing then
  would have left the surface instance 5 drifted on outside the map.
  **Finding three does not hold, and it is the eleventh instance of this tree's own class.** `.7.2.1`
  published "39 commits now separating `e6f5012d` from `HEAD`" — in the sentence explaining that a window
  naming its start but not its end cannot be anchored *because it silently grows*. It is 43 four commits
  later. **The defect was committed inside the sentence diagnosing the defect**, on three surfaces, and this
  is now the fourth consecutive correction round in which a slice invalidated its own publication.
  **Finding four's numbers do not hold; its substance does.** "20 and then 24 rows re-pinned" were the repair
  script's counters from repeated runs *within* a slice, so they answered a different question than the
  sentence asked (§3 Leg 1's granularity rule). Re-derived per commit from Git: 21 / 22 / 23 / 20. The claim
  that this is unautomated hand work with a silent-wrong-line hazard is unaffected and stays with `.12`.
  **Finding five does not hold as generalised.** "a pinned migration suffix that consumes two thirds of each
  byte budget" is true of `changes` (66.9%) and false of `live-achievement-status` (**59.2%**). The
  two-thirds figure was measured on one ledger against the *warning* budget and then restated across both
  against the *health* budget — one number, two denominators, two populations. The generalisation is
  withdrawn and `MEMORY.md` now routes to `--report` instead of carrying either share.
  **The mechanism for the eleventh instance was derived from the producers, not read off the code.** A first
  account — "`commits` is missing from the census's closed noun vocabulary" — was **discarded before
  publication**: that vocabulary belongs to `check_book_quantitative_claims.pl`, which governs only
  `docs/book/src/**`, and the current-claim census does not scan prose for `number + noun` at all. What the
  producers actually report: `check_current_claim_census.pl --produce` yields exactly **one** `CHANGES.md`
  candidate, on basis `surface_review`, which is its first non-blank line; and
  `check_published_assertions.pl --produce` yields **zero**, because `CHANGES.md` carries **zero** claim tags
  and so never enters the discovered population. `CHANGES.md:56` was therefore outside every governed
  population, which is the fact `.13` now owns
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.2.1`
  Verification: `git rev-list --count e6f5012d..HEAD` = 43 against the published 39; blind-spot probe
  re-derived at `a88b91a3~1` = 19/16/2/1 exactly; `git show 3c17ae5c~1:doctrine/claim_verification/published_assertions.jsonl`
  confirms `governed_globs: ["TOOLBOX.md"]` and four records none of which name the book chapter, while
  `git show 3c17ae5c~1:<book chapter> | grep -c 'claim: '` = 4; per-commit re-pin counts re-derived from four
  `git show ... | grep -c '^-{'` runs; `check_rolling_ledger_protocol.pl --report` gives 66.9% and 59.2%;
  `--produce` on both census producers gives one `CHANGES.md` candidate and zero governed values. Ledger
  records are corrected by appending, never by editing a sealed record, which is the idiom `.11a` used
  (`git show 3895226e -- CHANGES.md | grep -c '^-'` = 0). Doctrine gate green`
  Commit: `CLAIM-VERIFICATION-ADOPTION.7.2.1a — re-derive .7.2.1's findings and withdraw the three that do not hold`

<!-- claim-verification-task-source-region:provenance-gate-nodes:end -->
