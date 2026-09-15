# CLAIM-VERIFICATION-ADOPTION — census and withdrawal

- Part ID: `census-and-withdrawal`
- State: `legacy`

<!-- claim-verification-task-source-region:census-withdrawal-nodes:start -->
- ID: `CLAIM-VERIFICATION-ADOPTION.6`
  Status: `done` (`2026-08-28`)
  Goal: correct the claim-annotated prose counts that have drifted from their own producers, and name the
  mechanism that let them
  Acceptance: this tree's own review surface publishes counts that no longer hold. `TOOLBOX.md` states the
  census has "56 exact units … 32 excluded" where `check_current_claim_census.pl --report` now derives 59 and
  35, and "the 75 incomplete assertion regions exposed by the narrower mdBook contract" where
  `check_book_quantitative_claims.pl --report` derives 89. Each is re-derived from its named producer, each
  drift is attributed to the commit that caused it, and every remaining count in the same section is
  re-derived and either confirmed or corrected — a partial sweep would leave the surface exactly as
  untrustworthy as it is now. The measured cause must be stated rather than the symptom patched:
  `TOOLBOX.md` is digest-bound as a `canonical_input` of `claim-provenance-gate-active`, so the gate proves
  the file has not changed *without knowing what its sentences claim*, and a claim's own `assertion` prose is
  outside the join too — the same drift was found and repaired inside
  `mdbook-quantitative-census-frozen`, whose assertion said 318 regions while its pinned rederive marker
  said 319. `.7` owns the gate; `.6` owns the correction and the attribution
  Verification: producer reports re-derived at HEAD, per-revision attribution from Git, doctrine gate
  Evidence: all **11** counts the section publishes were re-derived from their named producers; **8 confirmed,
  3 stale**. Two of the three turned out to be **per-commit counters**, not constants: the census unit total and
  its excluded count rise by exactly one for every slice that prepends a rolling-ledger head, measured
  56 → 57 → 58 → 59 → 60 across `50775894`, `e6f5012d`, `f9e785ca`, `fdda3c53`, and this commit. Re-carrying
  them would have been stale on landing — the first attempt at this leaf wrote 59/35 and the very commit
  publishing it made them 60/36 — so `.6` **withdraws** both from the prose and routes the reader to
  `--report`. The one genuinely stale constant is corrected: mdBook incomplete regions 75 → **89**. Confirmed
  unchanged: 11 derived, 7 identity-gated, 6 registered, 0 incomplete, 86 produced
  anchors, 51 exact evidence keys, 35 registered annotations, 0 unresolved. Attribution is per revision, from
  Git rather than assumed — census units 56 at `50775894` (correct when written), 57 at `e6f5012d` (`.5`),
  58 at `f9e785ca` (`.11`), 59 at `fdda3c53` (`.12`); mdBook incomplete 75 at `50775894`, 89 from `e6f5012d`
  onward. So `.5` started both drifts and this tree's own `.11`/`.12` widened one of them, each under a fully
  green gate. Cause measured, not inferred: the census closes a `[claim: <id>]`-annotated region on the
  **presence** of the annotation — TOOLBOX's counts are 3 of the 35 `registered_annotations`, never among the
  51 `exact_evidence` keys — and `claim-provenance-gate-active` digest-binds `TOOLBOX.md` only as an unchanged
  file. Neither leg reads a number
  Commit: `CLAIM-VERIFICATION-ADOPTION.6 — re-derive the drifted claim-annotated prose counts`

- ID: `CLAIM-VERIFICATION-ADOPTION.6a`
  Status: `done` (`2026-08-29`)
  Goal: withdraw the four `TOOLBOX.md` census counters that `.6` confirmed and that have drifted again, on
  measured trajectory rather than by analogy with `.6`
  Acceptance: `.6` re-derived all 11 counts, withdrew the two it proved were per-commit counters, and
  explicitly recorded the other eight as **"confirmed unchanged"**. Four of those eight no longer hold. The
  leaf must (a) re-derive the trajectory of each of the four across the revisions between `5fe81128` and
  `c1609558` using each commit's own checker in a worktree, so "per-commit counter" is measured the way `.6`
  measured its two and `.5` measured the pointer preamble — never asserted from one endpoint; (b) withdraw
  only those the trajectory proves are per-commit, correct any that are genuinely stale constants, and leave
  the confirmed-stable ones carried with their producer field named; and (c) state plainly that the
  mechanical gate remains `.7`, so this is a correction, not a control
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.6`
  Measured `2026-08-29` at `c1609558`, before any of this session's edits, by running each producer and
  comparing field by field with the section's prose. Two sentences are **current**: `--report`'s
  7 cited controls / 7 exact RED regions / 6 governed producers / 0 ignored / 0 untracked, and the
  27-case self-test (27/27). Two are **not**, and both are inside the counts `.6` recorded as confirmed:

  | `TOOLBOX.md` carries | `check_current_claim_census.pl --report` at `c1609558` | verdict |
  | --- | ---: | --- |
  | 11 derived | 11 | confirmed |
  | 7 identity-gated | 7 | confirmed |
  | 6 registered | 5 | **drifted** |
  | 0 incomplete | no incomplete outcome present | confirmed |
  | 86 produced anchors | 72 | **drifted** |
  | 51 exact evidence keys | 50 | **drifted** |
  | 35 registered annotations | 22 | **drifted** |
  | 0 unresolved | 0 | confirmed |

  Attribution is **partly documented and partly unmeasured, and the difference is not blurred**. For the
  closure triple the `current-claim-census-frozen` record already states the cause in its own `assertion`:
  `5fe81128`'s own `CHANGES.md` rollover moved 14 annotated regions into segment `0013` and took 86/51/35 to
  72/50/22 within the very commit that published them — so the claim record observed the invalidation and
  `TOOLBOX.md` did not. That is the record's account, not this leaf's re-derivation, and `.6a` must
  reproduce it. When `6 registered` became 5 is **not known** and must be measured; no endpoint comparison
  can supply it.
  Natural experiment obtained this session, which is dimensionally different from re-reading the file:
  commit `1507adbf` published no census result and touched no producer — `MEMORY.md`,
  `docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md`, and the two claim registries — and moved all four again,
  `registered` 5 -> 4 and closure 72/50/22 -> 69/49/20. (The opening note said "nothing owned by the census";
  that was wrong and is corrected here — `MEMORY.md` and `docs/tasks/*.md` are both governed census surfaces,
  which is precisely why the pointer rewrite could move the counts.) The rewrite dropped all three
  `[claim: <id>]` annotations `MEMORY.md` carried at `c1609558` (`claim-provenance-gate-active`,
  `current-claim-census-frozen`, `mdbook-quantitative-census-frozen`; `git show <rev>:MEMORY.md | grep -c
  '\[claim:'` gives 3 -> 0), which is exactly the mechanism `.6` named: the census closes a region on the
  **presence** of the annotation, so the counts move whenever annotations do
  Risk this leaf must not repeat: `.6`'s first attempt wrote 59/35 and the commit publishing it made them
  60/36. Re-carrying a per-commit counter is stale on landing, so the default is withdrawal plus a named
  producer field, and re-carrying requires the trajectory to show the number actually held
  **Measured trajectory (`2026-08-29`) — the 28 consecutive revisions `e6f5012d` -> `60a81db7`, plus the older
  `50775894` anchor `.6` itself cited (29 measurements), each with that revision's own checker, in a detached
  worktree**, which is what separates a stale constant from a counter ordinary work moves. Rig, exactly as run:

  ```sh
  git worktree add --detach .project-data/tmp/claim-census-trajectory/wt 5fe81128
  for rev in 50775894 $(git rev-list --reverse 5fe81128~3^..HEAD); do
    git -C .project-data/tmp/claim-census-trajectory/wt checkout --detach --quiet "$rev"
    rm -rf   .project-data/tmp/claim-census-trajectory/wt/subs/fsmgen
    mkdir -p .project-data/tmp/claim-census-trajectory/wt/subs/fsmgen
    git -C subs/fsmgen archive "$(git rev-parse "$rev:subs/fsmgen")" \
      | tar -x -C .project-data/tmp/claim-census-trajectory/wt/subs/fsmgen
    ( cd .project-data/tmp/claim-census-trajectory/wt \
      && perl scripts/check_current_claim_census.pl --report )
  done
  git worktree remove --force .project-data/tmp/claim-census-trajectory/wt
  ```

  The `subs/fsmgen` step is not optional and is the reason the first two attempts recorded nothing:
  `git worktree add` does not populate a gitlink, so `check_fsmgen_feedback_protocol.pl` fails, the
  `fsmgen_correspondence_projection` derived-state contract fails with it, and the census exits **before**
  printing its report at every revision. Recorded as `[[worktree-doctrine-measurement-gitlink]]`.

  | Band | `registered` | `candidate_closure` | Revisions |
  | --- | ---: | --- | --- |
  | A | 6 | 86 = 51 + 35 + 0 | `50775894`\*, `e6f5012d`, `f9e785ca`, `fdda3c53` |
  | B | 5 | 72 = 50 + 22 + 0 | `5fe81128`, `245b3b60`, `3b3ea863`, `024202dd`, `5cac6c67`, `75275b69`, `2172ad9e`, `10d66551`, `943381c8`, `5f44ea68`, `3833ad10`, `5f568381`, `4f7590a4`, `455e74bd`, `369e826a`, `01f385b0`, `b36e81b3`, `505fe7b4`, `1ccb7331`, `3840bb0e`, `69a0d6da`, `d1c22cd9`, `c1609558` |
  | C | 4 | 69 = 49 + 20 + 0 | `1507adbf`, `60a81db7` |

  \* `50775894`'s `--report` predates the `candidate_closure` field and emits only
  `producer_candidates: 76`; its `registered` is 6. The closure triple is therefore measured across the 28
  consecutive revisions from `e6f5012d`, and `registered` at all 29 measurements. `50775894` is `.6`'s own
  cited anchor and is 29 commits before `e6f5012d`, so it is a sampled point, not part of the consecutive run.
  **The unanswered question is answered: `6 registered` became 5 inside `5fe81128` itself** — the same commit,
  the same rollover, and the same transaction that published "6 registered" as confirmed unchanged. All four
  counts were already false in `.6`'s own commit. This is not slow decay a maintenance pass could have caught;
  it is same-transaction invalidation, and it is why `.6`'s wording ("confirmed unchanged") was unearnable at
  the moment it was written.
  **Stable at all 29 measurements**, and therefore carried with their producer fields named:
  `authority_outcomes.derived` **11**, `authority_outcomes.identity_gated` **7**, no `incomplete` outcome,
  `candidate_closure.unresolved` **0**, `current_surfaces` **39**, `views` **5**.
  **A fifth number in the same section is also corrected, and its stated mechanism was wrong.** `.6` published
  the unit trajectory "56 -> 57 -> 58 -> 59 -> **60** across `50775894`, `e6f5012d`, `f9e785ca`, `fdda3c53`,
  and this commit" and the rule "every slice that prepends a rolling-ledger head earns exactly one more
  excluded unit". Measured: 56 / 57 / 58 / 59 are right and the last element is **59, not 60**. The mechanism
  was re-derived from the registry blobs after `.6a` first published a wrong one (see the correction below):
  `5fe81128` removed exactly **one** census evidence row and added exactly **one**, so the total held at 59.
  And the rule is not a rule: across the
  27 transitions from `e6f5012d` to `60a81db7` the unit total rises 15 times, **falls twice**
  (`943381c8` 65 -> `5f44ea68` 64 and `c1609558` 71 -> `1507adbf` 70), and is unchanged 10 times. Withdrawn
  with the rest rather than re-carried.
  **Sweep boundary stated, because a partial sweep is what created this leaf.** `.6a` re-derived every count
  inside `TOOLBOX.md`'s claim-doctrine section, as `.6` did — and no further. Two other current-facing
  surfaces publish the same census numbers and are stale at `60a81db7`; they are owned by `.6b`, opened in
  this commit rather than reported.
  **This is a correction, not a control.** Nothing here observes the next drift. `.7` still owns the gate that
  binds a published count to its producer's exact report field, and `.6a` narrows `.7`'s design target: the
  gate has to be able to fail a commit whose *own* transaction invalidates a count it publishes.
  Verification: `28-consecutive-revision worktree trajectory plus the 50775894 anchor (each commit's own checker); TOOLBOX.md counts re-derived at
  HEAD from check_current_claim_census.pl --report (11/7/-/0 carried, registered+closure withdrawn),
  check_claim_verification.pl --report (7/7/6/0/0 current), check_book_quantitative_claims.pl --report
  (89 incomplete current), census --self-test 27/27; doctrine gate`
  Commit: `CLAIM-VERIFICATION-ADOPTION.6a — measure the drift trajectory before withdrawing the counters`

- ID: `CLAIM-VERIFICATION-ADOPTION.6b`
  Status: `done` (`2026-08-29`)
  Goal: apply `.6`/`.6a`'s remedy to the two remaining current-facing surfaces that publish the census counts
  Acceptance: `.6` and `.6a` swept `TOOLBOX.md` only, and said so. Found while running `.6a` and measured at
  `60a81db7` against the same two producers, two further surfaces publish the same stale numbers:
  `docs/book/src/reference/doctrine-enforcement.md` states "The repaired result contains 56 exact evidence
  units: 11 derived, seven identity-gated, six registered, zero incomplete, and 32 excluded" (producer: 70
  units, 11/7/**4**/0/**48**) and "The current authority freezes `regions=307`, `registered=8`,
  `incomplete=78`, and `excluded=221` ... `authored=26`, `example=8`, `identity=1`, and `dated=186`"
  (producer: **321**/8/**89**/**224**, exclusions 26/8/1/**189**); and the fact card
  `docs/knowledge/current-claim-census-freeze.md` states "The current 56-unit result is 11 derived, seven
  identity_gated, six registered, zero incomplete, and 32 excluded" and "78 incomplete assertion-level
  regions", with "56 exact authority units" in its own title. The leaf must re-derive every count on both
  surfaces, withdraw the ones `.6a`'s trajectory proves ordinary work moves, correct the genuinely stale
  constants, and leave the boundary-scoped historical sentences (`.3c` closed 79 = 51 + 28, `.4` closed
  84 = 51 + 33, `.5` closed 86 = 51 + 35) as dated observations rather than rewording them into current
  claims. Two ordering constraints are part of acceptance: editing the book moves the frozen mdBook region
  set, so `doctrine/claim_verification/book_quantitative_claims.jsonl` is regenerated in the same
  transaction; and the **89** mdBook incomplete count `TOOLBOX.md` carries must be re-derived *after* the book
  edit, because this leaf's own edit can move it — the precise failure `.6` recorded and `.6a` measured
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.6a`
  **Remedy applied, and it is withdrawal on both surfaces rather than fresh numbers.** The book chapter now
  states the `.3b.4` unit vector as a *dated boundary*, names the two mechanisms `.6a` measured, carries only
  `derived` 11 / `identity_gated` 7 / no `incomplete` / zero unresolved, and routes the rest to `--report`. Its
  quantitative-adjudication section stops printing `regions`/`registered`/`incomplete`/`excluded` at all and
  routes to `--report` plus the registry's own `expected_*` fields, because those totals move whenever the
  manual changes — *including the commit that edits the chapter*. The three closure boundaries (79 = 51 + 28,
  84 = 51 + 33, 86 = 51 + 35) are kept as dated observations, and the invariant that survives them is stated
  instead: unresolved is zero. The fact card gets the same treatment plus a retitle, because its old title
  published `56 exact authority units` as current.
  Ordering constraint discharged: the mdBook incomplete count was re-derived **after** the book edit and is
  **unchanged at 89** — this edit removed numbers rather than adding candidate lines, and the census still
  reports 39 book files / 21 candidate files / 321 regions. `TOOLBOX.md` nevertheless **withdraws** the 89 as
  well, for consistency with what the book and the card now say: it stepped once (75 -> 89 at `e6f5012d`) in
  the same 29 measurements over which `registered` stepped twice, so carrying one and withdrawing the other
  could not be defended.
  Registry work this required, exactly as predicted: one frozen mdBook region (`book-quantity-0bee17d3b260b5c9`)
  moved 540 -> 547 and was re-pointed; no region was added or removed. The mdBook is a `maintained_reference`
  surface, so editing it also required a **new** `shipped_behavior.reference_contract.aggregate_change`
  authority (16,949 -> 16,961 lines, 1,090,989 -> 1,092,351 bytes); the generic gate refuses a reused
  authority_id across an aggregate change and caught the omission on the first commit attempt.
  **This commit then demonstrated `.6a`'s finding on itself.** Its own `CHANGES.md` rollover retired **13**
  census evidence rows whose regions left the live window — each verified byte-exact in
  `segment-0015-2026-08-29.md` at lines 1, 52, 104, 140, 171, 201, 272, 324, 344, 394, 426, 458, 492, and
  retired rather than re-anchored, per `.8`'s rule — and added one for the new ledger head. So
  `evidence_units` fell **71 -> 59** inside a single slice that published no census result at all. Every field
  `.6a` carried held across it: `derived` 11, `identity_gated` 7, no `incomplete`, `unresolved` 0, **39**
  surfaces, **5** views, and `registered` 4 with closure 69/49/20 unchanged. One book region also had to be
  re-anchored (`bacf4da4d64a` -> `c1ca6cd11566`, line 422 -> 428) because the edit moved the annotated line.
  **Correction `.6b` owes `.6a`, found by the director asking whether the finding was trusted.** `.6a`
  published a *mechanism* for its own correction — "`.6`'s own rollover sealed 18 records while adding 2, so
  the total did not rise" — that it had **not measured**. Re-derived here from the tracked registry blobs:
  `git show fdda3c53:doctrine/claim_verification/current_claim_census.jsonl` and the same at `5fe81128` both
  hold **59** evidence rows with **5** `CHANGES.md` rows, and the diff between them is exactly **one row
  removed and one added**. So the total held because one retirement cancelled one addition — not because
  eighteen sealed records cancelled two. The measured *numbers* `.6a` published are unaffected (56/57/58/59
  and 59 at `5fe81128` all re-derive), and so is the withdrawal; only the causal sentence was wrong.
  Two things are worth keeping from this. First, `.6` itself had already written the true mechanism down —
  "one census evidence region went with the sealed records and was retired, not re-anchored" — so its
  published `60` was a prediction that contradicted its own commit body, and `.6a` then invented a second
  wrong account instead of reading either. Second, the leaf that exists to stop unmeasured numbers published
  an unmeasured cause in the same breath, which is the same defect one level up: `.7`'s gate must reach a
  claimed *mechanism*, not only a claimed count, or it will keep passing sentences like this one.
  **Attribution measured, not assumed, and cheaply.** The mdBook census's whole history is recoverable without
  a worktree, because its frozen contract is a tracked registry: counting outcomes in
  `git show <rev>:doctrine/claim_verification/book_quantitative_claims.jsonl` over the 31 commits that touched
  it gives the exact trajectory. `regions/registered/incomplete/excluded` ran 304/8/75/221 from `467928bb`
  through `a4a08cd4`, then 307/8/78/221 at `be3b12e6` — which is what the chapter says, and it was **right
  when written** — then **six** moves: 308/8/78/222 at `4dac5642`, 309/8/80/221 at `a26c283e`, 318/8/89/221
  at `893c2fba`, 319/8/89/222 at `e6f5012d`, 320/8/89/223 at `f9e785ca`, 321/8/89/224 at `fdda3c53`, unchanged
  since. The book was never updated after `be3b12e6`. The census sentence has the same story one surface over:
  "56 exact evidence units" was correct at `50775894` and false from `e6f5012d`. So neither sentence was ever
  wrong on the day it landed
  Verification: `check_book_quantitative_claims.pl --check green (39 files / 21 candidate files / 321 regions)
  and --report unchanged at 321/8/89/224 after the edit; check_current_claim_census.pl --check green;
  knowledge-map derive-and-diff in sync; fact-card catalog valid for 248 cards; doctrine gate`
  Commit: `CLAIM-VERIFICATION-ADOPTION.6b / CHANGES-LEDGER-ROLLOVER.3 — sweep the two surfaces .6 and .6a left`
  Verification: `pending`
  Commit: `pending`

- ID: `CLAIM-VERIFICATION-ADOPTION.7`
  Status: `done` (`2026-08-30`, via `.7.0`/`.7.1`/`.7.1a`/`.7.2.0`/`.7.2.1`)
  Children: `.7.0`, `.7.1`, `.7.1a`, `.7.2.0`, `.7.2.1`, `.7.2.1a`
  Goal: make a claim-annotated prose count re-derive against its producer, so this drift is observed
  **Split (`2026-08-30`, after `.11a`).** Ten instances and five consecutive rounds of correction — `.6` -> `.6a`
  -> `.6b`, then `.10` -> `.10a`, then `.11` -> `.11a` — have established that this cannot be closed by prose.
  Each correction was itself invalidated: `.11` published a population its own commit moved, `.11a` published a
  warning-line total its own commit moved, and both published a set size whose classifier their own text
  satisfied. A sixth correction would behave identically. The leaf is therefore split into a design freeze
  (`.7.0`), the executable gate (`.7.1`), and the registry population (`.7.2`), on this tree's own precedent that
  a census design and its results must not share one unreviewable transaction (`.3a.0`/`.3a.1`/`.3a.2`)
  Acceptance: a digest-bound live document proves only that it has not changed, which is why three published
  counts went stale under a fully green gate — and two of them went stale *because of* commits that the gate
  passed on the way past. A bounded, declared map binds each published count in a claim-annotated prose
  region to its producer command and the exact field of that producer's report, and the checker re-derives
  and compares rather than pattern-matching numbers out of prose; an unlisted count in a governed region is
  reported rather than ignored, so the map cannot silently shrink. RED controls prove a drifted count, a
  count bound to the wrong field, and an unmapped count in a governed region are each observed
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.6`
  **Scope decision (`2026-08-30`, from `.10`; this narrows `.7` rather than widening it).** The seventh instance
  left one question open: is a claimed *mechanism* in this gate's scope, or is the hole simply unnamed? It is out
  of scope, and now for a stated reason rather than by omission. `.10` made the illustration rule normative — an
  account of why a value moved is earned only by an observation the competing account would not have produced —
  and no checker can decide whether two accounts predict the same observation, because that is a judgment about
  hypotheses and not about text. So the mechanism obligation belongs to the reviewer workflow
  (`CLAIM_VERIFICATION.md` §3 Leg 2, §8), where it is now normative, and `.7` stays on the one thing a checker can
  actually do: re-derive a published **count** against a named producer field. Two mechanizable residues survive
  and stay in `.7`'s design: the cross-surface disagreement signal from the fifth instance, which needs no producer
  at all; and an optional registry field recording which prior adjudication a mechanism claim was checked against
  — shape, not truth, which is what this repository's gates do well. Had that field existed, `.6a` would have had
  to name `.6`'s commit body or write `none-found`, and the true account was in that body
  Fourth instance (`2026-08-28`, found by `STATUS-LEDGER-ROLLOVER.4a`'s alignment review, repaired in that
  commit as a `COMMIT.md` blocker): the drift `.6` corrected in `TOOLBOX.md` was also inside the claim
  registry itself. `current-claim-census-frozen`'s own **assertion** carried "56 exact evidence units",
  "11 derived, seven identity-gated, six registered, ... 32 excluded", and "86 produced candidates close
  through 51 exact evidence keys and 35 current registered annotations". The producer now reports 61 units,
  11/7/5/38, and 72/50/22. The cause is exactly `.6`'s own finding, one surface further out: the counts were
  measured before `.6` applied its own `CHANGES.md` rollover, and that rollover moved **14** claim-annotated
  regions out of the live window into `segment-0013`, which is not a current census surface — 86 - 14 = 72,
  exactly. So a commit made its own published claim false, under a green gate, in the same transaction. The
  repair applies `.6`'s remedy to the registry: the per-commit counters are withdrawn from the assertion and
  the reader is routed to `--report`. This is repair, not closure — `.7` still owns the gate that would have
  observed it, and it must reach claim *assertions*, not only claim-annotated prose
  Fifth instance (`2026-08-29`, found by `SOURCE-IR-REPRODUCIBILITY.16`'s alignment review, repaired in
  that commit as a `COMMIT.md` step-3 blocker): **two published suite counts had drifted from their
  producers, and the two surfaces publishing them disagreed with each other.** Measured at the same
  commit: `perl scripts/test_live_document_size.pl` reports **84**, `perl
  scripts/check_fact_card_catalog.pl --self-test` reports **60**. `DOCTRINE_ENFORCEMENT.md` §10 said 84
  (correct) and 58 (stale); `docs/book/src/reference/doctrine-enforcement.md` said 81 (stale) and 58
  (stale). The other four counts on the same two sentences were re-derived in the same pass and are
  correct — 47 (`test_derived_state_contracts.pl`), 25 (`test_derived_state_authorities.pl`), 15
  (`check_task_tree_archive.pl --self-test`), 44 (`check_active_task_evidence.pl --self-test`).
  What makes this the sharpest instance yet for `.7`'s design: the book's `81` sits at
  `doctrine-enforcement.md:86`, which the frozen census does **not** hold a region for, while the `58`
  sits at line 88, which it **does** — as an `incomplete` region missing all three legs. So one stale
  count was outside the census's denominator and the other was inside it and explicitly unverified.
  Neither the digest binding nor the frozen census could observe either, because both prove that a
  region has not changed, not that its number still re-derives — which is exactly the gate `.7` owns.
  The disagreement between two surfaces publishing the same count is a second, cheaper signal `.7`
  could exploit: it needs no producer at all, only the observation that two governed regions state
  different values for one quantity
  Seventh instance (`2026-08-29`, `.6b` correcting `.6a`): the drift class is not confined to counts.
  `.6a` published an unmeasured **mechanism** — "`.6`'s own rollover sealed 18 records while adding 2" — and
  it is false; `5fe81128` removed one census evidence row and added one. `.6` had published a different wrong
  account and the true one in the same commit body. A gate that re-derives a published *count* against its
  producer would have passed all three sentences, because none of them is a count. So `.7`'s design has to
  decide explicitly whether a claimed causal account is in or out of scope, and say so rather than leave the
  hole unnamed
  Sixth instance (`2026-08-29`, measured by `.6a`, owned by `.6b` rather than repaired in place): the
  **same-transaction** shape is now proved rather than suspected. All four counts `.6` recorded as
  "confirmed unchanged" were already false in `5fe81128`, the commit that published them, and the same
  commit's rollover is what falsified them. A gate that re-derives a published count at commit time is
  therefore not a convenience — it is the only instrument that can see this class at all, because there is
  no interval during which the published value was true. `.6a` also measured that the unit total is **not
  monotone** (it falls when a governed surface loses annotated lines), so "counts only ever grow" is not a
  simplification `.7` may rely on

<!-- claim-verification-task-source-region:census-withdrawal-nodes:end -->
