# CLAIM-VERIFICATION-ADOPTION — standard readoption

- Part ID: `standard-readoption`
- State: `legacy`

<!-- claim-verification-task-source-region:standard-readoption-nodes:start -->
- ID: `CLAIM-VERIFICATION-ADOPTION.8`
  Status: `done` (`2026-09-15`)
  Goal: give `current_claim_census.jsonl` a lifecycle before it reaches its own bound
  Acceptance: the census registry declares `max_records: 128` and holds **109** (`2026-08-28`). It grows by
  **exactly one record per slice that prepends a rolling-ledger head**, because the first non-blank line of
  `CHANGES.md` is a produced candidate and each new head needs its own evidence row; measured 105 -> 105 ->
  105 -> 106 -> 107 -> 107 -> 108 -> 109 across `d94f11a3`, `f676c889`, `e6f5012d`, `f9e785ca`, `fdda3c53`,
  `5fe81128`, `245b3b60`, and `STATUS-LEDGER-ROLLOVER.4a`. That is roughly **19 slices** before the bounded
  registry refuses the append, and the growth is pure accumulation: a row for a former head is no longer a
  candidate and is retained only because nothing retires it. Decide the lifecycle — retire a row when its
  region stops being a produced candidate, or roll the registry the way its ledgers roll — and prove the
  retained evidence still resolves. Do not raise the bound to postpone it
  First retirement observed (`2026-08-28`, during `LIVE-DOCUMENT-PRESSURE-HEADROOM.5`'s `CHANGES.md`
  rollover): the accumulation is worse than "harmless rows". Two dead rows had drifted onto **blank
  lines** — region SHA-256 `01ba4719…546b`, which is the digest of a bare newline — and the rollover made
  them collide on one `evidence_id`, failing the gate outright. A region pinned to a newline addresses
  nothing; both were retired, taking the registry 114 -> 112. So a rollover is the natural retirement
  moment, and the rule `.8` needs is concrete: retire an evidence row when the record head it was created
  for leaves the live window, rather than relocating it onto whatever line now sits at its offset
  Prerequisite: none; it blocks nothing today
  **ANSWERED `2026-09-15`: the lifecycle this leaf asks for ALREADY EXISTS, and it runs measurably ahead
  of the bound.** The rolling-ledger rollover *is* the retirement mechanism, exactly as this leaf's own
  last paragraph proposed — and it has now been observed doing it a second time. At `ab339652`
  (`WIRE-BASED-100.10c`, carrying `CHANGES-LEDGER-ROLLOVER.8` in the same transaction) the census went
  **126 -> 115** as **11 `evidence-change-history-current-status-*` records** were retired: precisely the
  rows whose regions the rollover had sealed into a segment.
  **Measured over 120 revisions** it does not accumulate: **127 -> 115, net -12**. It grows `+1` per
  `CHANGES.md` prepend (25 in that window) and is reclaimed in blocks by the rollover; it has been at
  **127 of 128** and come back. The question was not "is there a lifecycle" but "does the reclaim arrive
  before the bound", and the margin is ~2.6x:

  | | current | trigger | prepends away |
  | --- | ---: | ---: | ---: |
  | census records | 115 / 128 (89.8 %) | the bound refuses the append | **13** |
  | `CHANGES.md` lines | 1,414 / 1,800 target | rolls at 90 % = 1,620 | **~5** |
  | `CHANGES.md` bytes | 204,698 / 255,000 target | rolls at 90 % = 229,500 | **~6** |

  Mean prepend over the same window: **42 lines / 4,010 bytes**. The ledger rolls — and reclaims — about
  **eight prepends before** the census could refuse an append; 89.8 % is the normal top of a sawtooth.
  **Inversion condition**, so a future session checks rather than re-derives: the census binds first only
  if the mean prepend falls below **~16 lines** (13 x mean < 1,620 - 1,414). Today it is 42.
  **Not done, deliberately:** no bound raised, no retirement rule added — a second mechanism would compete
  with the rollover for the same rows, and this leaf's own urgency evidence (two dead rows collided on
  blank lines) is now structurally prevented by `.12`'s refusal to relocate an ambiguous region.
  Verification: record counts per revision from Git; ledger fill from `wc` against the `change_history`
  surface's health targets; mean prepend from the byte/line delta of `CHANGES.md` across 23 prepends.
  Commit: see log

- ID: `CLAIM-VERIFICATION-ADOPTION.10`
  Status: `done` (`2026-08-30`)
  Goal: re-adopt the upstream standard, whose dropped rules would have caught this session's own defect
  Acceptance: directive 17 asks that the source standard be checked for updates after adoption. It had not
  been re-read since `.1`. Read `2026-08-29` at `/Volumes/SSD/Documents/github/pgen/docs/CLAIM_VERIFICATION.md`
  (245 lines, mtime `2026-08-26`; same volume, read-only, no copy taken). The local `CLAIM_VERIFICATION.md`
  is **205 lines and a restatement, not a copy**, so a byte diff says nothing — the gap has to be read
  section by section. Three upstream rules are absent locally, and each one independently forbids a mistake
  this repository has actually made:
  1. **§2, "the taxonomy of checks that cannot fail"** — a table of what each check class *still permits*
     (`sum(parts) == total` permits any redistribution; a hash of inputs permits every logic bug downstream;
     tests written from the spec over an implementation written from the same spec permit every misreading),
     closing with the general form: **a check and the thing it checks must not share a parent**. The local
     standard has the anti-patterns list but not the taxonomy that generates it.
  2. **Leg 2's illustration rule** — "two explanations that predict the same observation are not distinguished
     by *more* of that observation; if your evidence is consistent with both hypotheses, you have not tested,
     you have illustrated." That is exactly `.6a`'s false mechanism: "sealed 18 while adding 2" predicted the
     observation (the total held) and was never separated from the true account (one row retired, one added).
  3. **Leg 2's cheapest-oracle rule** — "the cheapest oracle is your own project's history: before publishing
     a finding, check whether a case of the same shape has already been adjudicated." `.6` had written the
     true mechanism in its own commit body; `.6a` invented a second wrong account without reading it.
  The leaf must carry these into `CLAIM_VERIFICATION.md` and the `TOOLBOX.md` published-claim section without
  bloating either past its bounds, keep the local registry/gate sections that the upstream does not have, and
  record explicitly which upstream material is deliberately **not** adopted and why. Then re-assess `.7`: if
  the illustration rule is normative locally, a mechanism gate may be a review obligation rather than a
  checker, and `.7`'s scope decision becomes evidence-based instead of open
  Prerequisite: none; it blocks nothing mechanically, but it is the cheapest fix for the defect class `.7`
  is trying to gate
  **Result (`2026-08-30`).** The source is unchanged since `.1` read it — mtime `2026-08-26`, SHA-256
  `3ac26c365ed6b0c9c4f714fec8c952379ed02c5ac61bd4edcd5a60553de0f85c` — so the gap dates from the original
  adoption rather than from an upstream revision, and only a re-reading could have found it. **The gap is larger
  than this leaf predicted**: reading upstream section by section and probing each rule against every governed
  claim surface found a set of absent normative rules, not three, now tabulated in `CLAIM_VERIFICATION.md` §11
  with its local home per rule. Both statements are true under different denominators and the leaf is not
  withdrawn: it named the three that had **already produced a recorded defect here**, which is the stronger
  filter; §11 enumerates every upstream normative rule with no local home, adopted or explicitly refused. Nine
  more rules were adopted alongside the three, the loudest being that a repository-derived constant is derived or
  gated **never carried**, that a set claim carries its enumeration in both directions, and that a classifier is
  derived from the producer rather than from a description of it — the last being `.9`'s defect stated as a rule.
  **One adopted rule was already here, demoted.** ADR 0042's Context paragraph states the general form — a check
  and the thing it checks must not share a parent — while the standard carried only the three instances that form
  generates. A rule living as decision-record rationale while its examples live in the normative text is
  under-specified for every reader who does not read ADRs; that is the preface rule the same reading adopted, so
  the promotion is the correction and the ADR keeps the sentence as reasoning.
  **The scoping error is recorded because the rule being adopted caught it in the same session.** The first
  absence probe ran **seventeen** terms over `CLAIM_VERIFICATION.md` alone and returned zero hits for every one
  of them; two of the seventeen discriminate nothing (they return zero upstream too) and the widened probe
  dropped them, which is why it ran fifteen. Concluding "absent locally" from it would have been a claim about
  the repository evidenced by data about one file — Leg 1's granularity rule, and the sixth row of the §2 taxonomy, both adopted in this very commit.
  Widening the probe to `TOOLBOX.md`, `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`, `AGENTS.md`, the pull-request
  template, ADRs 0042/0044, and the mdBook enforcement chapter is what surfaced the ADR hit above. Every other
  term stayed at zero across the widened set, and the remaining matches are unrelated word collisions
  (`population` in a renaming-behaviour sentence, `revert` in a hook narrative, `container` in a formal-id route)
  — a population classified before its size was published, per the rule adopted here.
  **The count of adopted rules is published in exactly one place.** §11's table is the enumeration and the
  authority; ADR 0042 and the fact card route to it rather than restating a number, because three synchronized
  copies of one count is the defect Leg 1 now forbids and this commit is the first thing the rule applies to.
  Deliberately not adopted, with reasons recorded in §11 so a later reading does not re-open each: the upstream
  reference-deployment measurements (dated evidence about another project; local instances teach the same rules),
  the five-architecture summary table (a third copy of a fact `README.md` and `DOCTRINE_ENFORCEMENT.md` already
  carry), §5A's inline provenance-tag syntax (superseded by the executable registry and `[claim: <id>]`), and
  adoption checklist items 1–5 (executed by `.0`–`.5`).
  **Second self-catch, same session, different rule.** Drafting the resume pointer, the leaf wrote that the
  `shipped_behavior` byte warning (`docs/book/src/pipeline/evidenceir.md`) was "untracked by any headroom leaf".
  That is a set assertion — *nothing owns this* — and it is **false**: `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s opening
  pressure boundary table has carried that exact surface and file since `92e59c97`, along with the other four
  warned axes. One `grep` refuted it, which is the whole content of the rule adopted here: a set claim is a census,
  not an impression, and the enumerating command belongs beside it. Corrected before the pointer was committed.
  Two independent self-catches in one adoption commit is not a coincidence worth celebrating — it is the measured
  base rate of this defect class in ordinary work, and it belongs in `.7`'s and `.9`'s evidence.
  **Census maintenance, and how the re-pointing was falsified rather than trusted.** The prepends shifted 13
  frozen census regions off their line offsets, in two passes as the ledgers grew. Each was relocated by finding
  its recorded region SHA-256 elsewhere in the same file rather than by re-hashing whatever now sits at the old
  offset — the difference between moving a region and silently repointing it at unrelated text, which is `.8`'s
  blank-line failure. The relocation is falsified by an invariant rather than trusted: within one pass, every
  region in a file must shift by the **same** delta, because a prepend moves all of them equally. A region that
  had matched a duplicate line elsewhere in the file would show a delta different from its file's; none did, in
  either pass. Read the exact per-pass deltas from the commit diff — they are a property of this transaction, not
  a durable fact, and the invariant is what carries. One region was not a shift at all —
  `docs/knowledge/INDEX.md:3` moved 248 -> 249 cards — and it is `identity_gated`, so its verifier re-derives the
  count and refreshing its digest re-affirms a gated value rather than carrying a new one. One new evidence row
  was added for the new `CHANGES.md` head, which is exactly the per-slice growth `.8` owns.
  **Mandatory ledger rollover, in-slice, on this tree's own precedent.** The rationale record took
  `DEVELOPMENT_NOTES.md` past the 90% line rollover milestone — 1,659 -> 1,739 lines against a 1,900-line health
  target, whose 90% signal is 1,710 — so the doctrine refuses the append unless the same change performs the
  rollover. The question of whether that needs a separate owning tree was already adjudicated here: `.1` of this
  tree performed a `DEVELOPMENT_NOTES.md` rollover in its own slice (`claim-verification-adoption-1-development-
  notes-rollover-plan.jsonl`, segment 0008), and `2b9e8899` committed a slice and a `CHANGES.md` rollover
  together. Same shape, no difference to name, so the earlier ruling wins and this is `.10`'s own transaction.
  Sealed 22 whole records into `segment-0009-2026-08-30.md` (354 lines, 28,263 bytes), keeping the 11 newest
  committed opening records plus this slice's prepend live over the exact 50-record migration suffix. Nothing was
  trimmed, reordered, or rewritten. Result: 62 records / 1,384 lines / 182,781 bytes — 72.8% of the line target
  and 73.1% of the byte target, both back under the 80% warning. The dry-run was exact before applying, and it
  reported `future_prepends: 1`, which is how the in-flight record was proved to survive the cut rather than
  assumed to.
  Verification: `check_book_quantitative_claims.pl --check green at 39 book files / 321 candidates / 321 regions,
  unchanged before and after the mdBook edit, which is the evidence the added chapter prose publishes no new
  quantity; one frozen region re-pointed 547 -> 580 with its line SHA-256 unchanged; knowledge-map derive-and-diff
  in sync at 271 facts / 2148 keys after one question-key collision with ADR 0042 was resolved in the fact card's
  favour; fact-card catalog valid for 249 cards; doctrine gate`
  Commit: `CLAIM-VERIFICATION-ADOPTION.10 — re-adopt the upstream claim standard`

- ID: `CLAIM-VERIFICATION-ADOPTION.10a`
  Status: `done` (`2026-08-30`)
  Goal: correct a false count and an unenumerated set claim that `.10` published while adopting the rules against both
  Acceptance: the director asked a second time whether `.10`'s findings were trusted — the same question that
  produced `.6b` — so every figure was re-derived instead of re-read. Two defects, both in the finding about
  `.10` catching itself:
  1. **A false count.** `.10` published the narrow absence probe as a **fifteen**-term probe on four surfaces.
     Replaying the exact command against `HEAD~1:CLAIM_VERIFICATION.md` returns **seventeen** terms, all zero.
     Fifteen is a real quantity — the *discriminating* terms, after dropping `derived or gated` and `domain free`,
     which return zero upstream as well and so separate nothing — and it is the count the **widened** probe ran.
     A real quantity silently substituted for the one the sentence names: `.7`'s ninth instance exactly, committed
     one commit after that instance was written up. The corrected account is strictly better than the original
     because it also explains *why* the two probes have different term counts, which "fifteen" everywhere hid.
  2. **A set claim without its enumeration.** `DEVELOPMENT_NOTES.md` published "neither is visible to any gate
     this repository has" — refuted by one counterexample, so a census rather than an impression, and the rule
     `.10` made normative in the same commit. Now enumerated: neither sentence is a claim registry record, so the
     claim-verification gate's execution/digest/publication legs never reach it; and neither carries a numeral, so
     no census candidate grammar makes it a candidate — `MEMORY.md` is a governed census surface with four
     evidence rows and the sentence still produced none. The conclusion survives; it was simply asserted where it
     should have been counted.
  Corrected on all four publishing surfaces (`docs/tasks/CLAIM-VERIFICATION-ADOPTION.md` in three places,
  `CHANGES.md`, `DEVELOPMENT_NOTES.md`, and the acceptance checklist's fifteen left standing because it correctly
  describes the widened probe). Findings two and three of the same report re-derive **unchanged**: `.7`'s scope
  decision is verified against the seventh instance's own text, and every rollover figure comes from
  `check_rolling_ledger_protocol.pl --report` — 62 records / 1,384 lines / 182,781 bytes — with the producer now
  emitting **no** warning at all for that ledger, including the record-budget warning it emitted before the cut.
  **What this costs `.7` and `.9`:** the self-catch rate `.10` published as "twice in one commit" is now **three
  times**, and the third was caught only because the director asked again — not by any rule, gate, or review step.
  Two of the three are set claims. That is the sharpest argument yet that `.9`'s alarm should not depend on a
  vocabulary, and that `.7`'s cheapest signal is a second reader rather than a richer checker
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.10`

- ID: `CLAIM-VERIFICATION-ADOPTION.11`
  Status: `done` (`2026-08-30`)
  Goal: run the enumerating command `.6b` never ran, and withdraw the whole class the standard now forbids carrying
  Acceptance: `.6` swept `TOOLBOX.md` and said so; `.6a` swept `TOOLBOX.md` and said so; `.6b` opened on "two
  other current-facing surfaces publish the same census numbers" and swept two. None of the three ran a command
  that enumerates the population, which is the rule `.10` made normative one commit later: **a claim about a set
  carries its enumeration, in both directions.** Running it —
  `git ls-files '*.md' | grep -v '^docs/archive/' | grep -v '^subs/' | xargs grep -ln
  'check_current_claim_census\.pl\|check_book_quantitative_claims\.pl\|check_claim_verification\.pl'` —
  returns **15** surfaces at `9fc76685`, the parent commit. `.6b`'s "two" was an impression. Every count on all
  15 is re-derived here from the three producers' own `--report` output, and each is then classified by what
  makes it true rather than by whether it happens to be right today.
  **And the population is itself a per-commit counter, which this leaf proves on itself.** At the tree state
  this commit lands, the same command returns **16**: writing the `CHANGES.md` record above names
  `check_claim_verification.pl`, so the record enters the population it describes. Publishing a bare "15" would
  have made this leaf false in its own transaction — the sixth instance, inside the repair for the tenth. The
  count is therefore anchored to the revision it was measured at, and the durable artifact is the command. That
  is a hard constraint on `.7`, not a curiosity: its map must derive the governed population **at check time**
  and may never store a surface list, because any commit that mentions a producer joins the set
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.10a`
  **Tenth instance, and it is the eighth instance's own commit one sentence over.** `authority_outcomes.
  identity_gated` was published as **7** on three surfaces and the producer reports **8**. Attributed by
  re-deriving from each revision's own producer input (`git show <rev>:doctrine/claim_verification/
  current_claim_census.jsonl`) rather than by reading a diff: 7 at every revision through `efa6dd41`, 8 from
  `d23e8bae` (`2026-08-29 23:16`) onward, which added exactly one `identity_gated` evidence record —
  `evidence-task-tree-catalog-parts-roadmap-controller-projections-0d651dd0cc01`, for the
  `task_tree_catalog_parts` surface it registered. That is the **same registration, in the same commit**, that
  moved `current_surfaces` 39 -> 40. `d23e8bae`'s own body records noticing the second value and withdrawing it
  on all three publishers — and left the first carried, on the same sentence, moved by the same record. So the
  withdrawal was driven by the field that was noticed rather than by re-deriving the set the sentence names,
  which is `.6`'s recorded lesson recurring for the third time. Three later commits (`71b6d832`, `3079f945`,
  `9fc76685`) republished `identity_gated 7`, all three in this tree, all three about claim currency
  **Findings, enumerated, with the value each producer reports now. Six are stale — the published value differs
  from the producer today. Three more are carried and still correct, and are withdrawn under the same rule,
  because "correct today" is exactly the state findings 1 to 4 were in before their commit landed.**
  1. `authority_outcomes.identity_gated` **7** -> **8** at `TOOLBOX.md:108`,
     `docs/book/src/reference/doctrine-enforcement.md:457`, and `docs/knowledge/current-claim-census-freeze.md:36`.
  2. `docs/book/src/reference/doctrine-enforcement.md:486` — "the 78 incomplete assertion-level book regions
     remain explicitly unverified": **78** -> **89**. Correct when written at `be3b12e6` (`2026-08-16`), stale
     from `4dac5642` (`2026-08-27 02:14`). Present tense with no dated scope, so §1 governs it; and the
     repository already adjudicated this exact quantity as current when `.6` corrected its sibling 75 -> 89.
  3. `docs/knowledge/mdbook-quantitative-census-freeze.md` — the **whole** `.3b.3.3` vector, in its title and
     twice in its body: regions **307** -> **321**, incomplete **78** -> **89**, excluded **221** -> **224**,
     dated exclusions **186** -> **189**. Correct at `be3b12e6` and stale from `4dac5642` — the *same commit
     and the same drift event* `.6b` repaired on the book chapter, quoting these very numbers off it. The card
     carrying an identical vector was two directory entries away and was not looked for, because nothing looked.
     Its `39` book files, `21` candidate files, `8` registered, and `26`/`8`/`1` exclusion split do re-derive.
  4. `docs/knowledge/doctrine-enforcement-adoption.md:35`-`38` — "**Registered today (10):** nine gate-tier
     doctrines — …" naming nine, "runs all nine gate rows", "`--all` runs all ten". Re-derived from the driver's
     own `DOCTRINES` array: **11** registered, **10** gate-tier, **1** CI-tier. `PROOF-SEAL-CURRENCY` was
     registered at `1ccb7331` (`2026-08-29 02:16`) and is missing from the card's named list. This one is a set
     claim that *did* carry its enumeration and the enumeration is short by one member — the mirror failure of
     finding 3, and the reason `.10` adopted the rule "in both directions".
  **Fifth and sixth findings, from applying the same rule to the surfaces this leaf had to touch anyway.** The
  resume pointer published its own census — "its warnings currently name" five live-document surfaces, "all five
  are already on `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s opening pressure boundary table, so none is an unowned
  warning" — and the producer at `9fc76685` on a clean tree emits 18 warning lines over **13** surfaces. The two
  published lists do not even agree: the pointer names `rust_analysis` and `workflow_standards`, which the table
  does not carry, and omits `knowledge_cards` and `readme_entrypoint`, which it does. Classified before the size
  was published, by grepping each surface id across `docs/tasks/*.md`: ten are named by some tree and **three by
  none** — `alignment_task_evidence_index`, `alignment_task_evidence_parts`, and `rust_analysis`. So the
  conclusion was false, and the omitted set includes `active_resume` at 100% of its rollover band, which is the
  pointer itself. The remedy in layer A is deletion, not a longer list: a bounded overwrite-only pointer must not
  carry a census at all, so the sentence is withdrawn and the gap is owned by a new
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.7`. Opening it exposed the sixth: that tree's Current Frontier still showed
  `.2a`, `.2b`, and `.2c` as `pending` while its Task Tree section had them `done` since `2026-08-29` — a stale
  hand-maintained table that would have handed a fresh session three finished leaves. Both repaired in this
  commit. Neither is a claim-verification surface, which is the point worth carrying to `.7`: the failure is not
  specific to counts about claims, it is what happens to any hand-maintained restatement of a derivable set
  **Seventh, eighth, and ninth: carried but not yet wrong, withdrawn under the same rule.** Applying the rule to
  the sentence rather than to the number that moved turns up three more carried repository-derived constants that
  happen to re-derive today. `authority_outcomes.derived` **11**, on the same three surfaces as finding 1 and in
  the same clause. `mdbook-quantitative-census-frozen`'s own registry **assertion**, which still spells out
  39/321/21/321/8/89/224/26/8/1/189 — eleven constants, and the one census claim that never received `.6`'s
  remedy while its sibling `current-claim-census-frozen` did; that asymmetry is finding 3 one layer down, and it
  is why the enumerating command has to reach `doctrine/` and not only `*.md`, which is a stated limit of the
  command used here. And `control_audit`'s **7 / 7 / 6**. Separating that last one required reading the producer
  rather than its description: `check_claim_verification.pl` raises `governed producer census contains ignored or
  untracked candidates` on a nonzero census, so `ignored_candidates` and `untracked_candidates` **0** are gated
  and stay; and it fails a cited control with no exact RED region, so `cited_controls` == `exact_red_evidence` is
  gated while their common **value** is not. Publishing the relation and withdrawing the value is strictly more
  informative than publishing 7 and 7, because the relation is what the gate actually holds.
  **The remedy is one class, not nine patches, and the standard already decided it.** `.10` adopted
  "a repository-derived constant is derived or gated, never carried", and retired the licence under which
  `derived 11` and `identity_gated 7` were carried in the first place — `.6a`'s 29-revision trajectory. Applying
  that rule to the sentence rather than to the number that moved: `authority_outcomes.derived` is the same class
  as `identity_gated` and is withdrawn with it, unmoved rather than immovable; "no `incomplete` outcome" is a set
  claim over the same producer and is withdrawn; the book vector and the mdBook card's vector go to `--report`.
  Replacing **7** with **8** was available and is refused, because §3 Leg 3 is explicit that a right unwatched
  number replacing a wrong one is not a fix.
  **What may still be published, and why — verified in the producer, not assumed.**
  `candidate_closure.unresolved` **0** stays, because it is **gated**: `validate_candidate_closure` in
  `scripts/check_current_claim_census.pl` pushes an error for every candidate lacking exact evidence or a current
  registered annotation, so `--check` fails the moment it leaves zero. `views` **5** stays, because it is
  **authored**: `.3a.0` froze five `required_views` and the registry declares them, which §1 exempts as an
  architecture choice whose authority is a decision. Those two are the only survivors on the census sentence, and
  each now states which of the two makes it true — so a later reader can tell a gated value from an unmoved one
  without re-running the trajectory that failed here.
  **Recorded, not repaired, because both belong to open leaves.** `scripts/check_doctrines.sh`'s header calls
  `DOCTRINE_ENFORCEMENT.md` §10 its human-readable mirror "kept in lockstep", and **nothing checks that**:
  `check_claim_verification.pl` requires only that the single `CLAIM-VERIFICATION` row is present. Enumerated
  rather than asserted — the §10 table has 11 doctrine rows and the `DOCTRINES` array has 11 entries, so they
  agree today; this is an unwatched coupling for `.7`, not a defect, and it is the exact shape `.7` gates.
  And findings 1 and 2 sit at book lines 457 and 486, neither of which the frozen mdBook census holds a region
  for — that chapter's complete candidate set is five lines — so this is `.9`'s **third** demonstration, again on
  the census's own chapter, with `identity_gated` (7) failing the closed noun list the same way `regions=307` did
  Verification: `enumerating command run over tracked non-archive Markdown (15 surfaces at 9fc76685, 16 here); identity_gated attributed
  7 -> 8 at d23e8bae by per-revision re-derivation over 25 revisions of the producer input; check_current_claim_census.pl
  --check green at 40 surfaces / 66 units with --report unresolved 0 and views 5; check_book_quantitative_claims.pl
  --check and --report identical before and after the chapter edit, which is the evidence the replacement prose
  publishes no new quantity; 17 frozen regions relocated by recorded SHA-256 — 16 census, one book — plus one new evidence row for the new
  ledger head, counted by diffing both registries against HEAD rather than by adding up the relocation passes,
  which first produced 18; each relocation matched exactly one line, each file's shift was unanimous, and each
  equalled the cumulative insertions above it computed independently from the diff hunks; check_fact_card_catalog.pl --check
  valid for 249 cards; knowledge-map derive-and-diff in sync at 271 facts / 2151 keys; check_claim_verification.pl
  --check green after digest refresh; census --self-test 27/27, claim --self-test 27/27, book --self-test 19/19;
  doctrine gate`
  Commit: `CLAIM-VERIFICATION-ADOPTION.11 — enumerate the surfaces, then withdraw the class`

- ID: `CLAIM-VERIFICATION-ADOPTION.11a`
  Status: `done` (`2026-08-30`)
  Goal: re-derive `.11`'s own findings when asked, and correct the five that do not hold
  Acceptance: the director asked whether `.11`'s findings still held — the question that produced `.6b` and
  `.10a`, and which has now found a real defect on all three occasions. Every figure was re-derived rather than
  re-read. **Five defects, four of them in the finding about unenumerated sets.**
  1. **A mechanism asserted from co-occurrence.** `.11` published "three later commits (`71b6d832`, `3079f945`,
     `9fc76685`) republished `identity_gated 7`". Re-derived with `git show --name-only`: only **`3079f945`**
     (`.10`) edited any of the three publishers; `71b6d832` and `9fc76685` touched none of them. They *left* the
     value standing, which is not republishing it. "Republished" is a claim about what a commit did and it was
     read off the commit dates. The corrected fact is stronger than the withdrawn one: `.10` edited `TOOLBOX.md`
     **while adopting the rule that forbids carrying the value**, and did not re-derive the line it was editing.
  2. **A commit body trusted as evidence.** `.11` published that `d23e8bae` withdrew `current_surfaces` "on all
     three publishers". It withdrew it from **two**: `TOOLBOX.md` and `current-claim-census-freeze.md` carried
     `39`, and `docs/book/src/reference/doctrine-enforcement.md` never carried `current_surfaces` at all — the
     only thing `d23e8bae` changed in that chapter is `92 -> 93` lifecycle cases. "All three publishers" is
     `d23e8bae`'s own commit-body wording, repeated instead of re-derived. §3 Leg 2 names project history as the
     cheapest **oracle**; an oracle is a hypothesis to test, not a source to quote. The asymmetry survives and is
     sharper: both surfaces that carried `current_surfaces` carried `identity_gated` in the same clause, and only
     the first was withdrawn from each.
  3. **A census scoped to one producer, published as the doctrine's.** `.11` published "the producer emits 18
     warning lines over **13** surfaces". That is `perl scripts/check_live_document_size.pl`, not the doctrine:
     `LIVE-DOC-SIZE` runs `scripts/check_live_document_size.sh`, which composes **four** producers, and the
     `active-task-evidence` and `rolling-ledger` lines carry no `surface '...'` token at all, so a surface-keyed
     census is structurally blind to them. Taking a component's output as the gate's answer is §3 Leg 1's
     granularity rule, committed in the finding about enumeration. **The line totals themselves are withdrawn
     rather than corrected**, and this leaf's own first draft is why: it published the component's line count as
     20, and the very commit that published it crossed another band and made it 21. A warning-line total is a
     per-commit counter of the same kind as every other value this tree has withdrawn. Read the population from
     `bash scripts/check_live_document_size.sh` at the revision you care about; only the **four-producer
     composition** is a structural fact, because it is a property of the driver rather than of the tree.
  4. **A classifier that a mention satisfies, over a set that includes closed trees.** `.11` published "three are
     named by no tree" from `grep -rl <surface> docs/tasks/*.md`. That screen is wrong in **both** directions.
     It counted **`done`** trees as owners — `corpus_task_evidence_parts` was scored owned by `LIVE-DOC-STOP-RISK`,
     which is `done`, so a genuinely unowned warning was missed and the real gap was four, not three. And
     publishing the finding **flipped its own classifier**: `alignment_task_evidence_index`,
     `alignment_task_evidence_parts`, and `rust_analysis` now match `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md`
     because this leaf names them. A check that the act of writing the finding turns green is a check sharing a
     parent with the thing it checks — §2's general form, and the reason the remedy is an explicit reviewed
     assignment rather than a longer grep.
  5. **A dated snapshot treated as a competing current census.** `.11` said the pointer's list and
     `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s boundary table "do not even agree". That table is headed
     `## Opening Pressure Boundary (92e59c97)` and is explicitly anchored to a revision, which §1 exempts. It is
     not a current census and was never at fault. What survives is only the part about the pointer:
     `MEMORY.md` asserted "all five are already on" that table, and two of its five — `rust_analysis` and
     `workflow_standards` — are not on it, so the pointer's conclusion did not follow from the authority it cited.
  **What re-derives unchanged, checked one by one rather than assumed.** The drift itself:
  `authority_outcomes.identity_gated` is **8** and was published as **7** on three *current-facing* surfaces —
  a classification, not a raw match count, and `.11` published the size without saying so. Five tracked
  non-archive Markdown files carried the literal at `fd09708d~1`; the other two are dated records that §1 exempts
  (`LIVE_ACHIEVEMENT_STATUS.md:7`, `.6a`'s ledger entry, and `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md:417`,
  `.6a`'s own leaf text). The conclusion is unchanged and the predicate is now stated, per §3 Leg 2's rule that a
  population is classified before its size is published. The book
  chapter's **78 -> 89**. The `.3b.3.3` vector **307/78/221/186 -> 321/89/224/189** in the mdBook fact card. The
  doctrine card's **10/nine -> 11/10/1** with `PROOF-SEAL-CURRENCY` registered at `1ccb7331`. The sibling
  frontier calling `.2a`/`.2b`/`.2c` pending. The three carried-but-correct withdrawals. The enumerating
  population **15 -> 16**. **17** regions relocated and one added, re-derived HEAD against HEAD~1. §10's **11**
  rows against the driver's **11** entries. The book chapter's complete candidate set of **5** regions.
  **And this commit's own transaction moved the warned set again**, which is the sixth instance behaving exactly
  as documented: `.11`'s ledger prepends took `achievement_status` and `change_history` past their warning bands,
  so the live-document-size surface count is **15**, not the 13 measured at `9fc76685`. Both are owned by open
  trees (`STATUS-LEDGER-ROLLOVER`, `CHANGES-LEDGER-ROLLOVER`), so neither is a new gap — but the number `.11`
  published was invalidated by `.11`, again, and the durable artifact is the command rather than the count.
  **What this costs `.7`.** The self-catch rate for this adoption is now **four**, and three of the four were
  found only because the director asked a second time. No rule, gate, or review step in this repository found any
  of them. Two of the four are set claims and two are mechanism claims — which is the split `.10` predicted when
  it ruled mechanisms out of `.7`'s checker scope and into the reviewer workflow. `.7` should therefore stop
  treating "a second reader" as a fallback and record it as the primary control for the mechanism half
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.11`
  Verification: `git show --name-only over 71b6d832/3079f945/9fc76685 and d23e8bae; git show d23e8bae~1:<book> for
  the absent current_surfaces; check_live_document_size.pl vs check_live_document_size.sh warning populations
  (19+1 vs 35 across four producers); open-tree ownership screen recomputed over 24 open trees from each tree's
  own Status line; census/book/claim --check and --report all green and unchanged; 17 relocations re-derived
  HEAD vs HEAD~1; doctrine gate`
  Commit: `CLAIM-VERIFICATION-ADOPTION.11a — re-derive .11's findings and correct the five that do not hold`

<!-- claim-verification-task-source-region:standard-readoption-nodes:end -->
