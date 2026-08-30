# CLAIM-VERIFICATION-ADOPTION: adopt three-leg verification for published claims

## Metadata

- Tree ID: `CLAIM-VERIFICATION-ADOPTION`
- Status: `active` (`.0`–`.6`, `.6a`, `.6b`, `.10`, `.10a`, `.11`, `.11a`, `.7.0`, `.7.1`, `.7.1a` done — the surfaces publishing the census counts
  are now swept against an *enumerated* population rather than a remembered one, and the upstream standard is
  re-adopted with its refusals recorded; `.7` owns the gate that would have observed the drift, now scoped to
  counts; `.8` tracks the census registry's own capacity; `.9` owns the candidate vocabulary's blind spot, now
  with a third measured demonstration)
- Roadmap lane: process / continuity / signoff evidence (cross-cutting)
- Created: `2026-08-15`
- Last updated: `2026-08-30`
- Owner: repo-local workflow
- Owner directive: adopt `/Volumes/SSD/Documents/github/pgen/docs/CLAIM_VERIFICATION.md` if SpecForge has not
  already adopted it.

## Goal

Adopt claim verification as SpecForge's fifth portable architecture. A quantitative or otherwise actionable
published claim must name three dimensionally different legs: a source-derived reproduction, a falsification
oracle capable of distinguishing a competing hypothesis, and a durable tracked producer plus stale-state gate.
Missing legs remain explicit instead of being silently represented as signoff.

Complete the source standard's adoption checklist across current-facing SpecForge surfaces: publish the local
standard and decision, make claim provenance mechanically reviewable, replace or gate carried repository-derived
constants, audit the tracked status of claim producers, prove controls can go RED on known-bad inputs, and teach
the workflow through the mdBook and repository review path.

## Non-Goals

- Do not rewrite historical measurements in sealed archives or dated ledgers; they are time-scoped evidence, not
  current constants. Correct a historical record only when its own stated evidence is false.
- Do not manufacture a nominal third leg. A claim with an honestly named missing leg is preferable to a false
  `verified` label.
- Do not make every prose number a heavyweight CI job. Cheap identity gates may guard expensive re-measurements,
  but every artifact dependency, including its producer, must participate in that identity.
- Do not treat the new checker as an oracle for the truth of a claim. It enforces visible provenance shape and
  durable routes; the named re-derivation and falsification evidence still earn the claim.
- Do not run full CI per ordinary leaf. Reserve it for the closing adoption checkpoint or push boundary under the
  repository CI policy.

## Acceptance Criteria

- Root `CLAIM_VERIFICATION.md` defines the three legs, publishing contract, anti-patterns, and SpecForge adoption
  route without absolute persisted project paths.
- A layer-C decision freezes scope: current-facing actionable claims are governed; sealed/durable historical
  observations retain their dated meaning; stochastic claims require intervals or an explicit evidence gap.
- One registered doctrine check validates the claim-provenance mechanism and its own bounded registry/inputs;
  local hooks and CI inherit it through `scripts/check_doctrines.sh`.
- The review/acceptance template requires a claim tag or an explicit no-published-claim declaration.
- A repository-wide current-facing constant census classifies each in-scope claim as derived, identity-gated,
  three-leg tagged, or explicitly incomplete; no in-scope carried constant is silently unwatched.
- Every tracked control named as falsification evidence has a durable known-bad mutation/probe that is observed
  failing; controls that cannot discriminate are repaired, narrowed, or no longer cited as a falsification leg.
- Every producer of an in-scope published claim is tracked, repository-local, and included in stale-state
  identity where applicable; ignored/scratch-only instruments cannot support signoff claims.
- The mdBook documents the user/reviewer workflow with concrete SpecForge examples, and a Knowledge Map fact card
  makes the five-architecture relationship retrievable.
- Focused negative/positive checks and the mandatory doctrine driver pass per leaf; selected full CI passes on
  the closing leaf; every completed leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `CLAIM-VERIFICATION-ADOPTION`
  Status: `done`
  Goal: adopt re-derive / falsify / durability as the enforceable definition of a checked published claim
  Children: `.0`, `.1`, `.1a`, `.1b`, `.2`, `.3`, `.4`, `.5`

- ID: `CLAIM-VERIFICATION-ADOPTION.0`
  Status: `done`
  Goal: own and map the external standard onto SpecForge before changing policy or enforcement
  Acceptance: read the source standard completely; inventory current claim-bearing surfaces, review templates,
  existing derived-state checks, doctrine registration seams, control/mutation mechanisms, and ignored producer
  risk; freeze the in-scope claim taxonomy, exclusions, implementation sequence, and bounded evidence artifacts
  in this tree without claiming adoption
  Verification: source standard read in full; 39 nonhistorical governed Markdown surfaces, 14 explicit
  derived-state contracts / eight executed core verifiers / three secondary copies, 33 tracked checker files / 24
  self-test-capable checkers, doctrine/hook/CI seams, workflow routes, review-template absence, ignored-producer
  scope, and the 42-of-44 decision-capacity prerequisite measured; task catalog, derived-state report, canonical
  catalogs, and live-size self-tests pass
  Commit: `CLAIM-VERIFICATION-ADOPTION.0 — own and map three-leg claim verification`

- ID: `CLAIM-VERIFICATION-ADOPTION.1`
  Status: `done`
  Goal: publish the local standard, architecture decision, discovery routes, and authoring contract
  Acceptance: root standard and decision record establish the three legs and scope; bootstrap, README route,
  COMMIT/TOOLBOX guidance, and the repository review template require a claim tag or explicit no-claim marker;
  current documents use repository-root-relative paths and do not copy external absolute paths
  Verification: root standard + ADR 0042 freeze scope/three-leg vocabulary; bootstrap, 36-record README route
  plane, COMMIT/TOOLBOX, and pull-request template require the exact declaration; 14 workflow standards catalog
  exactly; Knowledge Map is 251 facts / 1,922 questions / 14 shards; route/catalog fault controls, mdBook, and
  mandatory doctrines pass; the required note prepend triggers an exact warning-safe 15-record development-note
  rollover; no mechanical claim-registry capability is asserted
  Commit: `CLAIM-VERIFICATION-ADOPTION.1 — publish the claim-verification contract`

- ID: `CLAIM-VERIFICATION-ADOPTION.1a`
  Status: `done`
  Goal: restore measured capacity headroom for the canonical workflow-standard collection
  Acceptance: census population/growth, readers/writers, catalog and route coupling, and candidate remedies after
  `.1`'s required two-file addition; choose a minimal repeatable capacity/topology remedy before another workflow
  standard is added; do not raise or reshape an authority merely to silence the 14/16 warning
  Verification: tracked measurement reproduces 14 members over six active dates with a four-member peak; 21 is
  minimal because 14/21 = 66.7%, 18/21 = 85.7%, and 18/20 is exactly rollover; explicit targets remain below
  the 32-item registry cap and a full catalog is at most 31 lines / 11,138 bytes; exact authority moves both
  reachable aggregate bands with unchanged per-file bounds; five controlled derivation cases and doctrines pass
  Commit: `CLAIM-VERIFICATION-ADOPTION.1a — re-derive workflow-standard capacity`

- ID: `CLAIM-VERIFICATION-ADOPTION.1b`
  Status: `done`
  Goal: retire the exact workflow-capacity authority consumed by `.1a`
  Acceptance: after `.1a` commits, remove its now-banked increase row without changing any standard, capacity,
  catalog, decision, route, measurement producer, or book content; return gates green on the committed profile
  Verification: the authority registry contains only its bounded control record; the committed 21-file surface,
  14 explicit targets, ADR, producer, catalog, Knowledge Map, standard set, and mdBook remain unchanged; focused
  capacity/live-size/currentness checks and all gate-tier doctrines pass
  Commit: `CLAIM-VERIFICATION-ADOPTION.1b — retire the consumed workflow-capacity authority`

- ID: `CLAIM-VERIFICATION-ADOPTION.2`
  Status: `done`
  Goal: mechanize bounded claim provenance and register it in the doctrine driver
  Acceptance: a self-bounded registry/checker validates governed claim ids, source commands, falsification
  evidence, durability state, complete artifact-input identity, tracked producers, and stale-state behavior;
  positive and controlled negative fixtures prove missing/unknown/duplicate/stale/untracked cases fail clearly;
  `scripts/check_doctrines.sh` registers the check without duplicating hook/CI wiring
  Verification: strict JSONL parser and Git/digest/argv executor pass 22/22 positive plus missing/unknown/
  duplicate/stale/untracked/bound cases; three verified records execute eight source/control commands; pending
  message and HEAD ID resolution are covered; the tenth doctrine row runs through existing hook/CI wiring; all
  focused documentation/currentness checks and gate-tier doctrines pass
  Commit: `CLAIM-VERIFICATION-ADOPTION.2 — gate published claim provenance`

- ID: `CLAIM-VERIFICATION-ADOPTION.3`
  Status: `done`
  Goal: sweep current-facing published constants and remove silent carried-state claims
  Children: `.3a`, `.3b`, `.3c`
  Acceptance: a bounded census covers current status, roadmap/controller projections, maintained references,
  doctrine baselines, and mdBook quantitative claims; every in-scope claim is derived, identity-gated, registered
  with three legs, or explicitly marked incomplete; each repair preserves one canonical authority and avoids
  synchronizing hand-carried copies
  Verification: closes through `.3a`–`.3c`
  Commit: parent closes with `.3c`

- ID: `CLAIM-VERIFICATION-ADOPTION.3a`
  Status: `done`
  Goal: freeze and execute the bounded current-claim census before repairing any finding
  Children: `.3a.0`, `.3a.1`, `.3a.2`
  Acceptance: define an explicit self-bounded census over the live surface registry and the five mandated
  categories; derive the exact current denominator, existing authority class, claim-bearing regions, and gaps;
  make missing/unknown/duplicate/untracked/stale scope fail closed; preserve every source surface unchanged
  Verification: closes through `.3a.0`–`.3a.2`
  Commit: parent closes with `.3a.2`

- ID: `CLAIM-VERIFICATION-ADOPTION.3a.0`
  Status: `done`
  Goal: freeze the census denominator, semantic units, and authority-disposition schema before implementation
  Acceptance: derive the 39-surface denominator from the live registry; define the five required category views,
  unique evidence-unit/claim keys, exact path/region identity, closed authority outcomes, verifier/claim joins,
  self-bounds, and no-source-rewrite rule; separate producer implementation and result freeze into later leaves
  Verification: 55 total live surfaces minus 15 `archive_terminal` minus one `frozen_legacy` = 39; all four
  rolling-ledger windows remain in the current denominator; schema and sequencing decisions are explicit below;
  task catalog, memory, live-size, claim gate, and mandatory doctrines pass
  Commit: `CLAIM-VERIFICATION-ADOPTION.3a.0 — freeze the current-claim census design`

- ID: `CLAIM-VERIFICATION-ADOPTION.3a.1`
  Status: `done`
  Goal: implement the bounded census contract and deterministic producer
  Acceptance: strict data/checker derive all 39 current surfaces, validate five category views, exact tracked
  regions and authority joins, and fail missing/unknown/duplicate/untracked/stale/bound controls; do not repair
  or rewrite a source assertion
  Verification: focused parser/coverage/currentness tests plus report, live-size, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3a.1 — implement the current-claim census`

- ID: `CLAIM-VERIFICATION-ADOPTION.3a.2`
  Status: `done`
  Goal: freeze the resulting authority census and exact `.3b` repair frontier
  Acceptance: run `.3a.1` against the current tree; record every evidence unit as derived, identity-gated,
  registered, explicit incomplete, or excluded with authority; publish exact gap identities/counts without repair
  Verification: derive-and-diff result, focused existing verifiers, catalogs, live-size, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3a.2 — freeze current-claim census findings`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b`
  Status: `done`
  Goal: repair every census finding through one canonical authority
  Children: `.3b.0`, `.3b.1`, `.3b.2`, `.3b.3`, `.3b.4`
  Acceptance: derive, identity-gate, register with three legs, or explicitly mark incomplete every `.3a` gap;
  remove hand-carried copies rather than synchronizing them; update census/result identities atomically
  Verification: closes through `.3b.0`–`.3b.4`
  Commit: parent closes with `.3b.4`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.0`
  Status: `done`
  Goal: freeze the five-key repair map before changing a source assertion or evidence classification
  Acceptance: distinguish navigation/membership identity from authored policy and actionable quantitative
  assertions; assign every frozen incomplete key to one bounded repair leaf; preserve the other 46 outcomes
  Verification: source-authority route audit, task/current/book alignment, focused current-census check, and
  doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.0 — freeze the current-claim repair map`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.1`
  Status: `done`
  Goal: repair the README, knowledge-card, and FSMGen-packet reference anchors through exact route authorities
  Acceptance: replace each broad incomplete title anchor with an explicit non-claim identity disposition plus
  executable route/catalog evidence; do not assert that membership validation proves member prose true
  Verification: README-policy, fact-card/catalog, canonical-catalog, census, and doctrine fault controls
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.1 — bind maintained-reference authorities`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.2`
  Status: `done`
  Goal: repair the workflow/doctrine baseline anchor without treating authored policy as measured truth
  Acceptance: exclude only the exact authored identity anchor and bind the actionable 14 / 4 / 21 capacity
  paragraph to `workflow-standard-capacity-profile`; preserve one canonical capacity authority
  Verification: capacity derivation/RED controls, canonical collection, census, claim, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.2 — bind the workflow baseline authority`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3`
  Status: `done`
  Goal: replace the mdBook-wide quantitative placeholder with bounded assertion-region authority
  Children: `.3b.3.0`, `.3b.3.1`, `.3b.3.2`, `.3b.3.3`
  Acceptance: classify the maintained manual's actionable current quantitative assertions by exact region and
  canonical producer/claim authority; keep dated boundary evidence explicit; no blanket book-level verifier may
  stand in for uninspected prose
  Verification: closes through `.3b.3.0`–`.3b.3.3`
  Commit: parent closes with `.3b.3.3`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.0`
  Status: `done`
  Goal: freeze the quantitative candidate grammar, authority schema, bounds, and implementation sequence
  Acceptance: derive the prose-only lexical denominator; require exact coverage without treating syntax as
  semantic scope; freeze closed current/excluded outcomes, exact regions, joins, and portable caps before code
  Verification: independent lexical census, task/current/book alignment, frozen current census, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.3.0 — freeze the mdBook quantitative census design`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.1`
  Status: `done`
  Goal: implement the bounded mdBook quantitative inventory and fail-closed producer
  Acceptance: derive book membership, ignore fenced code deterministically, emit every candidate prose line,
  validate exact tracked regions and closed outcomes, and fail missing/overlap/stale/untracked/unknown/bounds
  Verification: syntax, positive/negative fixtures, real inventory/check/report/producer, book, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.3.1 — implement the mdBook quantitative census`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.2`
  Status: `done`
  Goal: adjudicate every candidate into current actionable authority or an exact standard-scope exclusion
  Acceptance: every lexical candidate line is covered exactly once; current assertions join executable derived/
  identity authority, a current claim id, or named missing legs; exclusions name authored/example/identity/dated
  authority and cannot hide an actionable current assertion
  Verification: derive-and-diff result, joined verifiers/claims, manual competing scan, book, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.3.2 — adjudicate mdBook quantitative assertions`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.3`
  Status: `done`
  Goal: freeze the exact-region result as the mdBook quantitative authority consumed by `.3b.4`
  Acceptance: publish bounded totals/outcomes, prove no uncovered or overlapping candidate, and expose any honest
  incomplete assertion rather than promoting the selective book-currentness check
  Verification: clean result replay, full fault matrix, currentness/docs build, claim/live-size, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.3.3 — freeze mdBook quantitative authorities`

- ID: `CLAIM-VERIFICATION-ADOPTION.3b.4`
  Status: `done`
  Goal: re-freeze the repaired result and close `.3b`
  Acceptance: update every affected region/source identity atomically; frozen report has zero incomplete or
  otherwise unclassified `.3a.2` frontier keys; the other 46 units retain their semantic outcomes
  Verification: derive-and-diff census, all joined producers, currentness/book/catalog/live-size checks, and
  doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3b.4 — close current-claim repairs`

- ID: `CLAIM-VERIFICATION-ADOPTION.3c`
  Status: `done`
  Goal: independently close current-claim coverage and the `.3` parent
  Acceptance: re-run the census from a clean committed boundary, mutate every classification family and coverage
  join to RED, confirm no silent current-facing constant remains in the mandated scope, and close `.3`
  Verification: independent census/control matrix, catalogs, mdBook/current-truth, live-size, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3c — close the current-claim sweep`

- ID: `CLAIM-VERIFICATION-ADOPTION.4`
  Status: `done`
  Goal: close producer-tracking and falsification-RED evidence across governed claims
  Acceptance: VCS ignored/untracked census proves no in-scope published claim depends on a scratch-only producer;
  every cited falsification control is exercised against a durable known-bad perturbation and demonstrably goes
  RED; non-discriminating controls are repaired or their claim leg is downgraded explicitly
  Verification: tracked-producer audit, controlled mutation matrix, residue/locality census, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.4 — prove tracked producers and falsifying controls`

- ID: `CLAIM-VERIFICATION-ADOPTION.5`
  Status: `done`
  Goal: document, independently audit, and close the fifth portable architecture
  Acceptance: the mdBook teaches claim classification, authoring, review, missing-leg reporting, stochastic
  intervals, and the auditor asymmetry with runnable SpecForge examples; Knowledge Map, task/public status,
  architecture analysis, and resume pointer agree; an independent closure audit finds no silent in-scope claim;
  selected full CI passes, including a current exact production-graph oracle, and project-owned verification
  artifacts are cleaned or retained by policy
  Verification: mdBook test/build, catalogs, full CI, final doctrine and locality gates
  Commit: `CLAIM-VERIFICATION-ADOPTION.5 — close three-leg claim verification adoption`

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
  Status: `active`
  Children: `.7.0`, `.7.1`, `.7.2`
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

- ID: `CLAIM-VERIFICATION-ADOPTION.7.2`
  Status: `pending`
  Goal: populate the registry from the current governed regions and close the unlisted-value report at zero
  Acceptance: every published value in a claim-annotated or registered governed region resolves to one record
  with a closed outcome, or the run reports it. The three survivors this tree already labelled are the first
  entries — `candidate_closure.unresolved` `gated`, `views` `authored`, and the producer-census zeroes `gated`
  **Two things `.7.1a` measured that this leaf must decide before it can freeze anything.**
  *(a) `governed_globs` is the stored surface list `.7.0` element 3 forbids.* The registry ships
  `governed_globs: ["TOOLBOX.md"]`, so flipping `phase` to `frozen` would make exactly one surface fatal and
  leave `docs/book/src/reference/doctrine-enforcement.md` — the surface instance 5 actually drifted on —
  outside the map. The population must be selected by a derived rule, not by a listed name; the honest
  candidates are the current-surface authority `check_current_claim_census.pl` already derives, or a rule that
  admits every claim-annotated tracked Markdown file and excludes sealed-archive and dated-evidence lifecycles
  by their lifecycle, not by their path.
  *(b) the bound and the population disagree.* Measured `2026-08-30` with `.7.1a`'s corrected grammar and
  `governed_globs: ["**/*.md"]`: 152 unlisted values over 32 governed regions in four files — 85 in this task
  tree, 40 in `segment-0013`, 23 in `TOOLBOX.md`, four in the book chapter — against a declared
  `max_records: 128`. So a whole-tree close is not merely laborious, it does not fit, and raising the bound to
  make it fit is the move `.8` refuses for the census registry. Re-derive both figures from
  `perl scripts/check_published_assertions.pl --produce` rather than carrying them; they are recorded here as
  a dated measurement of the decision, not as current state
  Prerequisite: `CLAIM-VERIFICATION-ADOPTION.7.1a`
  Verification: `pending`
  Commit: `pending`

- ID: `CLAIM-VERIFICATION-ADOPTION.8`
  Status: `pending` (tracking-only)
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

- ID: `CLAIM-VERIFICATION-ADOPTION.9`
  Status: `pending`
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
  Prerequisite: none

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CLAIM-VERIFICATION-ADOPTION.0` | `done` | external standard and local enforcement/current-claim planes are mapped |
| 2 | `CLAIM-VERIFICATION-ADOPTION.1` | `done` | standard, ADR, discovery, authoring, and review contract are published |
| 3 | `CLAIM-VERIFICATION-ADOPTION.1a` | `done` | measured 21-file profile restores headroom without changing explicit topology |
| 4 | `CLAIM-VERIFICATION-ADOPTION.1b` | `done` | exact transaction authority retired; committed profile remains unchanged |
| 5 | `CLAIM-VERIFICATION-ADOPTION.2` | `done` | bounded registry, executable evidence join, and doctrine gate active |
| 6 | `CLAIM-VERIFICATION-ADOPTION.3a.0` | `done` | 39-surface denominator and bounded semantic schema frozen before code |
| 7 | `CLAIM-VERIFICATION-ADOPTION.3a.1` | `done` | bounded inventory contract, deterministic producer, and fail-closed joins implemented |
| 8 | `CLAIM-VERIFICATION-ADOPTION.3a.2` | `done` | 51 evidence units frozen; five explicit incomplete units own the repair frontier |
| 9 | `CLAIM-VERIFICATION-ADOPTION.3b.0` | `done` | five frozen gaps are assigned to authority-specific repair leaves |
| 10 | `CLAIM-VERIFICATION-ADOPTION.3b.1` | `done` | existing route authorities and RED controls are proven sufficient |
| 11 | `CLAIM-VERIFICATION-ADOPTION.3b.2` | `done` | authored identity and registered capacity authority are separated |
| 12 | `CLAIM-VERIFICATION-ADOPTION.3b.3.0` | `done` | prose candidate grammar and exact-region schema frozen before code |
| 13 | `CLAIM-VERIFICATION-ADOPTION.3b.3.1` | `done` | bounded inventory and fail-closed producer derive the exact review denominator |
| 14 | `CLAIM-VERIFICATION-ADOPTION.3b.3.2` | `done` | every candidate has one exact semantic authority or honest missing legs |
| 15 | `CLAIM-VERIFICATION-ADOPTION.3b.3.3` | `done` | verified claim and fact card freeze the exact result without promoting incomplete lines |
| 16 | `CLAIM-VERIFICATION-ADOPTION.3b.4` | `done` | 56-unit repair freeze closes all five outer incomplete keys |
| 17 | `CLAIM-VERIFICATION-ADOPTION.3c` | `done` | 27-case family/join matrix and zero-unresolved candidate closure close `.3` |
| 18 | `CLAIM-VERIFICATION-ADOPTION.4` | `done` | seven controls and six producers close with exact RED evidence and zero scratch candidates |
| 19 | `CLAIM-VERIFICATION-ADOPTION.5` | `done` | public workflow, independent audit, and selected full CI close the fifth architecture |
| 20 | `CLAIM-VERIFICATION-ADOPTION.6a` | `done` | 28-consecutive-revision trajectory proves all four were already false inside `.6`'s own commit; withdrawn with producer fields named |
| 21 | `CLAIM-VERIFICATION-ADOPTION.6b` | `done` | book chapter, fact card, and `TOOLBOX.md`'s last carried mdBook count now all route to `--report`; drift attributed per registry revision |
| 22 | `CLAIM-VERIFICATION-ADOPTION.10` | `done` | upstream re-read section by section; absent rules adopted, refusals recorded, and `.7`'s open scope question answered |
| 23 | `CLAIM-VERIFICATION-ADOPTION.10a` | `done` | `.10`'s own probe count and one set claim re-derived and corrected; findings two and three verified unchanged |
| 24 | `CLAIM-VERIFICATION-ADOPTION.11` | `done` | population enumerated by command rather than recalled; six stale publications and three still-correct carried ones withdrawn as one class |
| 25 | `CLAIM-VERIFICATION-ADOPTION.11a` | `done` | asked a second time; five of `.11`'s findings re-derived false, including its own ownership classifier and its producer scope |
| 26 | `CLAIM-VERIFICATION-ADOPTION.7.0` | `done` | five rounds of correction proved prose cannot close this; the contract that can is frozen before any code |
| 27 | `CLAIM-VERIFICATION-ADOPTION.7.1` | `done` | the gate executes producers and compares fields; 16/16 RED matrix and a drift observed on real shipped prose |
| 28 | `CLAIM-VERIFICATION-ADOPTION.7.1a` | `done` | the coverage grammar's blind spot is measured and closed, so the population it reports is the population that exists |
| 29 | `CLAIM-VERIFICATION-ADOPTION.7.2` | `pending` | decide the derived governed scope, populate the registry, and close the unlisted-value report at zero |
| 30 | `CLAIM-VERIFICATION-ADOPTION.7` | `active` | ten instances now; scope settled to counts by `.10`, so the producer-field re-derivation gate is the remaining design |

## Decisions

- `2026-08-30` (`.7.1a`): the coverage grammar decides the population, so a value it cannot see is a silent
  hole rather than an unlisted one — no record can be asked for it and `frozen` phase will still call the
  surface complete. A grammar change is therefore a population change, and it must be measured against an
  independent tokenizer and bound to RED cases before the population is closed, never adjusted by reading.
- `2026-08-30` (`.7.1a`): a numeral closing a compound adjective (`27-case`) or a ratio (`15/15`) is a
  published quantity; only `-` followed by a **digit** is an identifier or date fragment. The exclusion is
  narrowed to the form those identifiers actually take instead of excluding the whole punctuation class,
  which is how 19 published values across four claim-annotated files became invisible to the gate.
- `2026-08-30` (`.7.1a`): a comma joins a numeral only when it separates exactly three digits. Absorbing
  sentence punctuation into a value binds the gate to prose rather than to a quantity, and it makes the
  honest record — the one listing the number a reader would act on — report as unlisted.
- `2026-08-30` (`.7.0`): a value published on a governed surface must carry one of exactly four outcomes —
  `derived` (re-executed and compared against a named producer field), `gated` (a named control fails if it
  moves, and that control has a known-bad case), `authored` (a named decision fixes it), or `dated` (anchored to
  a revision). There is deliberately no outcome for a value carried because a trajectory shows it has held.
- `2026-08-30` (`.7.0`): the governed population is derived on every run and may never be stored. Three
  consecutive commits moved their own published population by describing it, so a stored list is guaranteed to
  be wrong at the moment it lands.
- `2026-08-30` (`.7.0`): stop correcting a class that prose cannot close. Five rounds of correction were each
  invalidated by their own transaction. The correcting leaf is retired in favour of the gate; a finding of this
  class is now evidence for `.7`, not a new prose repair.
- `2026-08-30` (`.11a`): an ownership or coverage classifier must be one the finding cannot satisfy by existing.
  A `grep` over `docs/tasks/` for a surface id turns green the moment a leaf names the surface in order to report
  that nothing owns it, and it scores `done` trees as owners. Screens are still useful for *finding* candidates;
  they may not be published as the answer. The publishable form is an explicit reviewed assignment naming an open
  leaf per warned item.
- `2026-08-30` (`.11a`): when a census is published about a doctrine, the denominator is the **doctrine's own
  driver**, not whichever checker was convenient. `check_live_document_size.pl` is a component of `LIVE-DOC-SIZE`;
  quoting its warning set as the gate's is the container-for-item substitution §3 Leg 1 forbids.
- `2026-08-30` (`.11a`): a commit body is a hypothesis, not evidence. §3 Leg 2 makes project history the cheapest
  *oracle* — something to test a finding against — and `.11` quoted `d23e8bae`'s body as a source instead, which
  is how "two publishers" was published as three.
- `2026-08-30` (`.11`): a count published on a governed surface is legitimate only when a **control fails if it
  moves** or an **authored decision fixes it**. "A trajectory shows it has held" is neither, and is retired as a
  reason. Where neither applies, the remedy is withdrawal plus the producer's field name — never substitution of
  today's value, which leaves the next drift exactly as unobservable as the last.
- `2026-08-30` (`.11`): a sweep names its population with a command, not with recall. `.6`, `.6a`, and `.6b` each
  swept surfaces they remembered and each said honestly how far it went; the defect is not the honesty but that no
  step derived the denominator. The enumerating command for this class is a grep for the producer paths over
  tracked non-archive Markdown, and it belongs in the leaf beside the finding.
- `2026-08-15`: claim verification is additive to task trees, memory, retrieval, and doctrine enforcement. It
  defines what earns a published claim; doctrine enforcement makes that definition mechanically difficult to
  bypass.
- `2026-08-15`: the adoption governs current-facing claims someone may act on. Dated historical measurements stay
  immutable unless disproven; their current reuse creates a new claim and must satisfy the three legs.
- `2026-08-15`: enforcement will validate provenance and staleness structure, not pretend a syntactically valid tag
  proves the underlying measurement.
- `2026-08-15`: `.3` will census all 39 nonhistorical governed Markdown surfaces, but will register claims—not
  raw numerals. Version strings, schema ids, dates, example literals, exact hashes, authored priorities, and
  historical captures keep their existing authority class; a current quantitative assertion someone may act on
  is in scope.
- `2026-08-15`: `.2` will use a dedicated self-bounded JSONL registry under `doctrine/claim_verification/` and a
  focused checker registered through the existing doctrine driver. The 14-field derived-state plane remains an
  input/authority source rather than being overloaded with falsification semantics.
- `2026-08-15`: at `.0`, no pull-request or review template existed. `.1` adds the repository-standard GitHub
  pull-request template and aligns `COMMIT.md` / `TOOLBOX.md`; authors state either governed claim tags or that
  the slice publishes no actionable claim.
- `2026-08-15` (ADR 0042): reserve `verified` for re-derivation, dimensionally different falsification with an
  observed RED control, and tracked stale-detecting durability. Govern current actionable assertions rather than
  numeral syntax; current reuse of dated history creates a new claim.
- `2026-08-15`: the exact author/reviewer declaration is `Published-claims: none` or comma-separated stable claim
  IDs. `none` is scope-based, not a docs/code exemption; syntactic mechanical enforcement begins only in `.2`.
- `2026-08-15`: the required root standard plus PR template raise canonical workflow standards from 12/16 to
  14/16 files (87.5%). `.1a` owns a measured repeatable remedy; `.1` does not hide the warning or widen a bound.
- `2026-08-15` (ADR 0043): retain explicit stable membership and minimally re-derive 21 slots from 14 current
  members plus the measured four-member peak. Per-file limits stay fixed; aggregate reachability moves with count.
- `2026-08-15` (`.2` design): use strict self-bounded JSONL claim records, argv-form commands, exact tracked
  artifact digests, executed source/control probes, and exact message ID resolution. Do not execute a shell string
  or duplicate hook/CI wiring; the existing doctrine driver is the sole composition seam.
- `2026-08-15` (ADR 0044): verified records join executed re-derivation/RED-control commands to the complete
  tracked SHA-256 artifact set; incomplete/superseded records preserve uncertainty, and registry validity never
  substitutes for semantic truth.
- `2026-08-15`: decompose `.3` into denominator/gap freeze (`.3a`), measured repair (`.3b`), and independent
  clean-boundary closure (`.3c`). A census design and its repairs must not share one unreviewable transaction.
- `2026-08-15` (`.3a.0`): derive the denominator from the surface registry: 55 total minus 15 archive terminals
  minus one frozen legacy surface = 39 current surfaces. Rolling-ledger live windows remain included; archive
  members and the frozen dated capture do not become current merely because they contain constants.
- `2026-08-15` (`.3a.0`): the census has five required category views—current status, roadmap/controller
  projections, maintained references, doctrine baselines, and mdBook quantitative claims. Views may refer to one
  governed surface for different semantic questions, but each evidence unit/claim key is unique and each of the
  39 surfaces has exactly one inclusion/exclusion disposition.
- `2026-08-15` (`.3a.0`): every evidence unit binds a tracked repository-relative path plus an exact region/
  assertion identity and one closed outcome: `derived`, `identity_gated`, `registered`, `incomplete`, or
  `excluded`. The first two require an executable verifier, `registered` requires a known current claim id,
  `incomplete` names missing legs, and `excluded` names a standard scope reason. Unknown fields/outcomes fail.
- `2026-08-15` (`.3a.0`): the JSONL control bounds records/bytes/record bytes/arrays/scalars below checker hard
  caps. `.3a.1` implements coverage and fault controls; `.3a.2` freezes results; neither may rewrite a source
  assertion. Repairs begin only in `.3b` from the frozen gap identities.
- `2026-08-15` (`.3a.1`): keep the real census contract in `inventory` phase with zero adjudicated evidence
  records. It derives rather than repeats lifecycle eligibility, gives all 39 current surfaces exactly one
  disposition, and emits deterministic review anchors plus known derived-state/registered-claim candidates.
- `2026-08-15` (`.3a.1`): exact evidence identity is a one-based line range plus SHA-256 of those source bytes.
  A frozen `derived`/`identity_gated` unit must execute a tracked argv-form producer/input join; `registered`
  resolves a non-superseded claim ID; `incomplete` and `excluded` carry their required explicit reason fields.
- `2026-08-15` (`.3a.2`): the first result-production pass exposed a structural gap: multi-view surfaces emitted
  only their first-view review anchor. The freeze therefore requires at least one evidence unit for every
  included surface **and** every required semantic view; the producer keys review anchors by surface + view +
  path + line so overlapping views remain independently adjudicable.
- `2026-08-15` (`.3a.2`): the final 51-unit authority census closes as 11 `derived`, four `identity_gated`, four
  `registered`, five `incomplete`, and 27 `excluded`. The incomplete units are the README maintained-reference
  surface, mdBook quantitative surface, workflow-doctrine baseline surface, knowledge-card reference surface,
  and FSMGen issue-packet reference surface; `.3b` may repair only those five exact keys.
- `2026-08-15` (`.3b.0`): a broad title anchor cannot earn authority for the prose below it. Route membership is
  repaired through the existing README/catalog producers, authored workflow identity remains out of claim scope
  while its actionable capacity paragraph joins the existing verified claim, and mdBook quantitative assertions
  receive their own exact-region contract instead of borrowing the selective book-currentness checker.
- `2026-08-15` (`.3b.1`): the three maintained-reference gaps need no new checker. README policy, routed fact-
  card membership, and canonical collection membership already derive their exact routes and each has a
  controlled missing/drift/bound class observed RED. `.3b.4` will bind those authorities without claiming that
  navigation identity verifies member prose.
- `2026-08-15` (`.3b.2`): workflow doctrine prose has two authority classes. The collection/title is authored
  normative identity; the actionable capacity paragraph already resolves to `workflow-standard-capacity-profile`
  with its tracked derivation, strict-boundary RED controls, and independent catalog-feasibility probe. No copied
  capacity value or second claim record is introduced.
- `2026-08-15` (`.3b.3.0`): the book quantitative repair uses a prose-only lexical candidate grammar as a
  completeness alarm, not a semantic classifier. The provisional shell census recorded 301 candidate lines
  across 21 files; exact region adjudication must cover each once, while code fences remain outside prose scope.
  The grammar and outcome schema freeze before `.3b.3.1` implementation.
- `2026-08-15` (`.3b.3.1`): the executable parser replayed the frozen grammar against both the `.3b.3.0` parent
  and current book and found 304 candidate lines across the same 21 files. The three-line delta is not book
  drift: the untracked provisional scan undercounted a stable input and left no reproducible artifact. The
  bounded contract now records the executable denominator, and future disagreement fails instead of silently
  changing scope.
- `2026-08-15` (`.3b.3.2`): exact single-line regions classify all 304 candidates: 75 current actionable lines
  remain honestly incomplete on all three claim legs, eight workflow-capacity lines join the verified current
  claim, and 221 lines are exact scope exclusions (26 authored thresholds/choices, eight examples, one schema/
  identity literal, and 186 dated boundary observations). A current-language competing scan reopened every
  excluded `current`/`live`/`today`/`now` hit and confirmed its dated section or named authored/example scope.
- `2026-08-15` (`.3b.3.2`): the first frozen replay found that the checker applied `max_array_items` to the JSONL
  record list rather than only arrays inside records. Inventory's four source records hid the dimensional error;
  the exact region set exposed it. Validation now leaves record capacity to `max_records`, applies array bounds
  per record, and a nineteenth controlled case prevents recurrence.
- `2026-08-15` (`.3b.3.3`): `mdbook-quantitative-census-frozen` verifies the census mapping, not the truth of its
  75 incomplete assertions. Re-derivation executes the current exact-coverage report; controlled mutations
  challenge coverage, regions, sources, outcomes, joins, and bounds; digest-complete durability watches the
  producer, frozen contract, source identities, lockstep publications, fact card, and retained task evidence.
  The Knowledge Map card makes that authority/uncertainty boundary retrievable before `.3b.4` consumes it.
- `2026-08-15` (`.3b.4`): close a broad incomplete title by narrowing its question, not by declaring the whole
  document verified. The five old anchors become explicit identity exclusions and gain five separate exact
  authorities: three executable routes plus the registered workflow-capacity and mdBook-mapping claims. The
  resulting 56-unit vector has zero outer incompletes while the 75 inner mdBook incompletes remain unchanged.
- `2026-08-15` (`.3c`): frozen evidence coverage and producer-candidate closure are independent invariants. Every
  produced anchor must have an exact evidence key or a current non-superseded registered annotation. The clean
  final lockstep boundary closes 79 candidates as 51 exact keys + 28 registered annotations + zero unresolved.
  The three-candidate increase from the pre-publication replay comes from the closure's own synchronized change,
  book, and task annotations; the
  27-case suite independently drives all five outcome families and every surface/view/path/identity join RED.

## Independent Current-Claim Closure (`CLAIM-VERIFICATION-ADOPTION.3c`)

- Clean revision `50775894c526108145aa978a854353dbd214f1af` replays the repaired 39-surface / five-view / 56-unit
  census and the independent 304-region mdBook census without rewriting a classification.
- The current checker now rejects a frozen produced candidate unless its surface/view/path/line key has exact
  evidence or its line carries a current registered claim annotation. The final lockstep report exposes 79
  candidates, 51 exact evidence keys, 28 registered annotations, and zero unresolved candidates.
- The self-test fixture positively instantiates `derived`, `identity_gated`, `registered`, `incomplete`, and
  `excluded`. Twenty-seven cases independently break every family plus surface disposition/evidence, view
  assignment/coverage, path ownership/tracking, region freshness, source/claim authority, evidence/claim-key
  uniqueness, schema, and portable bounds; a known current annotation passes while a silent derived marker goes
  RED.
- The 75 exact mdBook assertion incompletes remain explicit and therefore are not silent. The outer candidate
  closure proves only its mandated surface/view denominator; the manual's separate exact-region denominator
  continues to own quantitative assertion-level truth. `[claim: current-claim-census-frozen]`

## Producer And RED-Control Closure (`CLAIM-VERIFICATION-ADOPTION.4`)

- The governed registry cites seven falsification controls. Six already executed controlled mutation suites;
  `independent-catalog-feasibility` was the repair frontier because it proved the live catalog bound without
  exercising a known-bad bound inside the cited command.
- Every falsification command now binds a named known-bad case to an exact line-range/SHA-256 region in its
  tracked producer. The gate executes the cited command, while the exact region proves which durable mutation
  case participates; removing or moving that case makes the claim stale instead of leaving a self-test name as
  the only evidence.
- The workflow feasibility probe executes both the live computation and a controlled sub-ceiling mutation,
  requiring the latter to go RED before it publishes PASS.
- The claim report exposes six unique governed producers plus ignored and untracked producer-shaped files
  under `scripts/`, `doctrine/`, `docs/`, and `.github/`. Any such scratch instrument is RED even if no registry
  row currently names it, preventing an omitted producer from silently becoming signoff authority.
  `[claim: claim-provenance-gate-active]`

## Final Adoption Signoff (`CLAIM-VERIFICATION-ADOPTION.5`)

- The mdBook now teaches the complete workflow through one real capacity claim: scope classification, source
  replay, a controlled independent RED probe, tracked stale identity, exact publication declaration, an honest
  incomplete-record example, stochastic interval requirements, and auditor asymmetry.
- A dedicated Knowledge Map card joins the five architectures without conflating them: task ownership, durable
  resume, question-first retrieval, composed doctrine enforcement, and per-claim evidence.
- The clean `b101aaaa` boundary independently reproduced the outer candidate closure and inner manual map before
  closing edits. Final reports retain zero silent current candidates and every manual candidate has one exact
  authority or named missing legs. `[claim: claim-provenance-gate-active]`
- Public alignment found one stale authored-intent field: `LIVE_ACHIEVEMENT_STATUS.md` still selected completed
  `.6d.ii.f` work while the current snapshot and roadmap select `SPEC-TO-INTENT-ALIGNMENT.7`. The single current
  priority field is corrected; historical dated entries remain immutable evidence.
- Closing full CI found that `SPEC-CLARIFICATION-LOOP.2` had correctly expanded the production graph to 79
  modules / 141 boundary rows and recorded the resulting 2,347 functions / 14,294 edges / 12,514 decisions /
  1,459 macros, but the independent Rust determinism test retained its preceding 78 / 140 / 2,275 / 11,926 /
  11,396 / 1,446 expectations and five instead of six non-authoritative regions. `.5` owns the exact test-oracle
  currency repair because that defect blocks its selected-CI acceptance; no product path, public API, graph
  derivation, or roadmap direction changes.
- The roadmap direction stays byte-identical because it already selects `.7`; task, public status, book,
  architecture analysis, retrieval, and resume state now agree.

## Repaired Current-Claim Result (`CLAIM-VERIFICATION-ADOPTION.3b.4`)

- The frozen report now derives 56 exact units: 11 `derived`, seven `identity_gated`, six `registered`, zero
  `incomplete`, and 32 `excluded` across the same 39 current surfaces, 32 included surfaces, seven exclusions,
  and five semantic views. `[claim: current-claim-census-frozen]`
- Canonical semantic comparison to the `.3b.3.3` parent proves all 46 non-frontier outcomes are identical after
  removing record IDs and exact regions. Ledger, book, and task anchors rotate visibly with their lockstep lines;
  the five `.3a.2` frontier outcomes alone change and five new authority objects are added.
- README documentation routes execute `check_readme_policy.sh`; the fact-card route executes
  `check_fact_card_catalog.pl`; and the FSMGen packet H1 identity executes `check_canonical_collection_catalogs.pl`.
  Each proves navigation membership only, not the truth of linked prose.
- The pull-request-template H1 is authored normative identity while the doctrine capacity paragraph joins
  `workflow-standard-capacity-profile`. The book `SUMMARY.md` H1 is generated navigation identity while the exact
  quantitative mapping tag joins `mdbook-quantitative-census-frozen`.
- Zero outer incompletes means every broad surface/view question has an honest authority or scope disposition.
  It does not certify the 75 exact assertion-level `incomplete` regions retained by the mdBook contract.

## Frozen mdBook Quantitative-Census Design (`CLAIM-VERIFICATION-ADOPTION.3b.3.0`)

- Membership derives from the tracked mdBook `SUMMARY.md` plane and must agree with the governed
  `shipped_behavior` surface; no parallel hand-written file list is authoritative.
- Fence state is per file and recognizes backtick or tilde fenced blocks. Fence delimiter lines and all enclosed
  example code are excluded before lexical candidate discovery; unterminated fences fail closed.
- A prose line is a candidate when it contains a percentage, numeric fraction, or a number followed by a bounded
  quantitative unit such as files, lines, bytes, records, members, facts, questions, shards, cases, tests,
  checks, surfaces, claims, fields, families, documents, pages, fixtures, diagnostics, commands, doctrines,
  signals, registers, artifacts, or rules. Syntax finds review work; it never decides claim scope.
- Each candidate line is covered exactly once by a tracked exact line-range/SHA-256 region. Regions on one path
  cannot overlap and every region must contain at least one candidate.
- Current actionable regions use one of four closed authorities: executable `derived`, executable
  `identity_gated`, current `registered` claim id, or honest `incomplete` with named missing legs. Excluded regions
  use a closed standard-scope reason for authored threshold/choice, example literal, schema/version/date/path/
  digest identity, or explicitly bounded historical evidence.
- The JSONL contract is capped below portable checker maxima at 512 records, 262,144 bytes, 16,384 bytes per
  record, 32 array items, and 2,048 scalar bytes. Inventory implementation and result adjudication/freeze are
  separate leaves; `.3b.4` consumes only the frozen result.

## Implemented mdBook Quantitative Inventory (`CLAIM-VERIFICATION-ADOPTION.3b.3.1`)

- `doctrine/claim_verification/book_quantitative_claims.jsonl` binds exact digests for the governed surface,
  book-summary, and derived-state registries, joins the live validated claim registry by identity, and remains
  deliberately in `inventory` phase with no semantic region records.
- `scripts/check_book_quantitative_claims.pl` derives tracked book membership from both authorities, parses
  backtick and tilde fence state per file, emits stable path/line/SHA candidates, and validates exact region
  coverage plus closed authority outcomes for the next leaf.
- The real inventory derives 39 book files and 304 prose candidates across 21 files. Its eighteen-case fixture
  matrix observes inventory/frozen success plus fence, coverage-gap, overlap, stale-region, empty-region,
  outcome, exclusion, verifier, claim, tracked-path, source-digest, denominator, identity, schema, and portable-
  bound failures. Fixtures live and are cleaned on the repository volume.
- The producer has check, report, produce, and self-test modes; inventory permits no regions and frozen mode
  requires complete exact-once candidate coverage. `.3b.3.2` alone owns semantic adjudication.

## Adjudicated mdBook Quantitative Result (`CLAIM-VERIFICATION-ADOPTION.3b.3.2`)

- The contract is in `frozen` validation phase with 304 exact single-line regions over all 304 candidates; the
  checker proves complete coverage, no overlap, current region hashes, book/source membership, and closed outcome
  fields before accepting the result.
- Eight exact lines in the workflow-headroom subsection join `workflow-standard-capacity-profile`. Seventy-five
  present-tense product/status/measurement assertions are `incomplete` with re-derive, falsification, and
  durability all named; no selective currentness command is promoted to authority for them.
- The other 221 regions are outside current-actionable claim scope for one exact reason: 26 authored thresholds
  or choices, eight worked/quoted examples, one schema/version/date/path/digest identity, and 186 task-, revision-,
  migration-, or snapshot-bounded observations.
- The semantic audit used the enclosing section plus exact line, then challenged exclusions by independently
  listing any line containing current/live/today/now/present language and challenged incompletes for past-tense
  migration language. Every exception was reopened; none moved to a broader section-level default.
- The frozen-result replay exposed and repaired the top-level-array bound defect. `max_records` alone controls
  JSONL record count; `max_array_items` now applies within each record, and the nineteen-case suite exercises that
  dimensional distinction alongside the prior fence, region, authority, source, schema, and bound controls.

## Frozen mdBook Quantitative Authority (`CLAIM-VERIFICATION-ADOPTION.3b.3.3`)

- The verified `mdbook-quantitative-census-frozen` claim is deliberately about the completeness and exact identity
  of the review mapping. Its assertion preserves `incomplete=75`; it does not certify the measurements on those
  lines or erase any missing evidence leg.
- Re-derivation executes the current report over the tracked membership and exact regions. Falsification replays
  nineteen controlled positive/negative states spanning fence handling, coverage, overlap, stale and empty
  regions, outcomes, authority/claim joins, source identity, denominator, schema, and independent record/array
  bounds.
- Durability digest-binds the checker, frozen contract, surface/derived/summary sources, lockstep ledgers and
  manual, toolbox route, `mdbook-quantitative-census-freeze` fact card, its bounded catalog, and retained task
  evidence. The stale command re-executes exact coverage against the resulting tree.
- The fact card routes future agents to the canonical contract and makes the crucial boundary explicit: eight
  regions inherit verified workflow authority; 75 remain actionable but incomplete; 221 are exact standard-scope
  exclusions. `.3b.4` may consume this census authority without promoting an underlying incomplete assertion.

## Frozen `.3b` Repair Frontier (`CLAIM-VERIFICATION-ADOPTION.3a.2`)

| Evidence key | Missing legs |
| --- | --- |
| `evidence-readme-entrypoint-maintained-references-e1a5013b8d02` | re-derive, falsification, durability |
| `evidence-shipped-behavior-mdbook-quantitative-claims-88501c2f5e66` | re-derive, falsification, durability |
| `evidence-workflow-standards-doctrine-baselines-7bf07db7c4e8` | re-derive, falsification, durability |
| `evidence-knowledge-cards-maintained-references-075a14c930ba` | re-derive, falsification, durability |
| `evidence-fsmgen-issue-packets-maintained-references-ecf3712243ed` | re-derive, falsification, durability |

## Adoption Mapping (`CLAIM-VERIFICATION-ADOPTION.0`)

| Source-standard requirement | Existing SpecForge seam | Adoption action |
| --- | --- | --- |
| Re-derive from source | 14 bounded derived-state contracts, eight executed core verifiers, generated catalogs, replay/currentness checks | reuse exact source/accessor authority; `.3` removes uncovered carried constants |
| Falsify with a different oracle | 24 self-test-capable checker fronts plus controlled genericity and chain-currentness mutations | `.4` inventories cited controls and records a known-bad RED result; a self-test name alone is insufficient |
| Durable tracked producer and staleness | all producer-shaped files under `scripts/`, `doctrine/`, `docs/`, and `.github/` are tracked; project outputs under ignored `generated/` are non-authoritative unless cited | `.2` rejects untracked producers; `.4` audits every governed claim and artifact dependency |
| Visible publishing contract | task acceptance checklist exists, but it has no claim-provenance field; no pull-request template exists | `.1` adds claim/no-claim authoring fields and the local standard |
| Mechanical gate | one nine-entry doctrine driver already serves pre-commit and CI | `.2` adds one gate-tier claim-verification entry; hook/CI wiring remains single-source |
| Bounded durable registry | existing JSONL control records cap records, bytes, arrays, and scalars | `.2` follows that pattern in a dedicated registry and self-tests its capacity/unknown-field behavior |

The initial ignored-file census found zero producer-shaped files below the governed source directories
`scripts/`, `doctrine/`, `docs/`, or `.github/`. Ignored `generated/` contains outputs and temporary fixture
copies, including four old corpus-KB self-test directories and a generated chain-currency replay helper. They
cannot support a published claim unless a registry row names them; `.4` owns the exact tracked-producer/residue
audit and cleanup evidence.

The decision collection was already 42/44 Markdown files at `.0`, so the required claim-verification ADR was
sequenced behind `DECISION-RECORD-CAPACITY-HEADROOM`. That prerequisite closed cleanly at 58 slots before ADR 0042
was added; the stable-path remedy and its consumed authority are complete.

## Claim Evidence — `claim-verification-contract-published`

- **Claim:** SpecForge publishes one repository-owned three-leg author/reviewer contract, discoverable from the
  bootstrap and README, with ADR 0042 rationale, an exact claim/no-claim declaration, and active bounded provenance
  enforcement.
- **Status:** `verified` for the publication/discovery/enforcement assertion.
- **RE-DERIVE:** `rg -n 'Published-claims:|Re-derive|falsif|durab' CLAIM_VERIFICATION.md AGENTS.md COMMIT.md
  TOOLBOX.md .github/PULL_REQUEST_TEMPLATE.md`; `scripts/check_readme_policy.sh`; and
  `perl scripts/check_canonical_collection_catalogs.pl --check` reproduce the contract, entry routes, and exact
  workflow-standard membership.
- **FALSIFY:** the competing hypothesis is “the prose exists but a reader/reviewer path can omit or strand it.”
  `scripts/check_readme_policy.sh --self-test` exercises missing/unclassified route failures, while
  `perl scripts/check_canonical_collection_catalogs.pl --self-test` exercises missing/drifted catalog membership;
  both controlled known-bad classes are observed RED before their suites report green.
- **DURABILITY:** `git ls-files --stage` must contain the standard, ADR, template, bootstrap, workflow files, and
  data registries; README-route, canonical-catalog, Knowledge Map, live-size, and doctrine checks re-run from
  tracked repository-relative inputs and reject stale derived projections. The exact repository-volume rollover
  plan preserves the rationale through segment 0008 rather than trimming a live record.

## Claim Evidence — `workflow-standard-capacity-profile`

- **Claim:** at the `.1a` boundary the canonical workflow collection has 14 explicit tracked members, a four-
  member peak active day, and a minimal warning-safe 21-file profile with unchanged stable paths/per-file bounds.
- **Status:** `verified` for the boundary/profile assertion.
- **RE-DERIVE:** `scripts/measure_workflow_standard_capacity.pl --check` reads the canonical surface targets and
  Git add history, reproducing 14 current / peak 4 / derived 21 and exact aggregate multiplication.
- **FALSIFY:** the competing hypotheses are “20 is sufficient” and “the catalog or portable target array binds
  before 21.” The tracked five-case self-test observes exact-90%, cap-20, and bad-aggregate controls go RED;
  full-profile arithmetic gives 31 catalog lines / at most 11,138 bytes under 384 / 65,536, and the registry's
  independent target-array hard cap is 32.
- **DURABILITY:** the producer, surface registry, ADR 0043, and catalog checker are tracked and digest-bound in the
  active claim registry. Generic live-size count warning is the stale-state trigger for a future re-measurement;
  `.1b` retired the transaction-only authority.

## Claim Evidence — `claim-provenance-gate-active`

- **Claim:** the bounded provenance checker is an unconditional gate-tier doctrine rejecting malformed, unknown,
  duplicate, stale, untracked, incompletely watched, or unresolved published claim evidence.
- **Status:** `verified` for the registered gate behavior; semantic truth still depends on the named independent
  evidence and remains auditable rather than inferred from record syntax.
- **RE-DERIVE:** `perl scripts/check_claim_verification.pl --check` validates three current verified records,
  executes eight argv-form source/control commands, authenticates every artifact digest, and resolves the pending
  message or `HEAD`; `--report` exposes the same census.
- **FALSIFY:** the competing hypothesis is “plausible malformed or stale provenance can remain green.” The tracked
  `--self-test` suite observes 22 positive/RED cases covering honest statuses plus missing/unknown/duplicate/stale/untracked/unsafe/
  false-RED/supersession/bound failures before it reports green.
- **DURABILITY:** ADR 0044, the self-bounded registry, checker, standard, and one doctrine-driver row are tracked;
  each verified record's stale check covers its exact artifact set, and the existing pre-commit/CI driver is the
  only wiring seam.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.10`

- [x] **REPRODUCE / MEASURE** — re-read `/Volumes/SSD/Documents/github/pgen/docs/CLAIM_VERIFICATION.md` section by
  section (same volume, read-only, no copy taken; mtime `2026-08-26`, unchanged since `.1` read it), then probed
  fifteen distinguishing terms for each upstream rule against every governed claim surface. The absent set and its
  local homes are tabulated in `CLAIM_VERIFICATION.md` §11.
- [x] **ROOT CAUSE (WHY + WHERE)** — the local standard is a **restatement, not a copy**, so neither a byte diff
  nor a digest comparison can answer directive 17's currency question, and nothing had re-read the source since
  adoption. The gap therefore dates from `.1`, not from an upstream revision. One rule was present but demoted to
  ADR 0042 rationale while the standard carried only the instances that rule generates.
- [x] **ADDRESSED (verified)** — every absent rule is now normative in `CLAIM_VERIFICATION.md` (preface, new §2,
  and the three legs), summarized for authors in `TOOLBOX.md`, taught in the mdBook enforcement chapter, and
  recorded with its reading boundary and explicit non-adoption set in §11. ADR 0042 records the re-adoption
  without changing what it decided.
- [x] **NO REGRESSION** — no product code, doctrine authority, limit, or registry semantics changed. The mdBook
  census re-derives to the same 39 files / 321 candidates / 321 regions before and after the chapter edit, which
  is the evidence the added prose publishes no new quantity; the one frozen region that moved was re-pointed
  547 -> 580 with its line SHA-256 unchanged. The mandatory `DEVELOPMENT_NOTES.md` rollover was applied as an
  exact whole-record transaction with a dry-run proven exact first, and the rolling-ledger protocol reports all
  four ledgers lossless afterwards.
- [x] **GENERICITY** — every adopted rule is stated domain-free, per the preface rule adopted in the same pass;
  no specification, vendor, protocol, corpus, or signal case appears in any of them.
- [x] **LOCKSTEP** — standard, ADR, `TOOLBOX.md`, mdBook chapter, fact card, Knowledge Map projection, this tree,
  the change and engineering ledgers, and the resume pointer all state the same re-adoption boundary, and the
  count of adopted rules is published in exactly one place so the set cannot drift between them.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.0`

- [x] **REPRODUCE / MEASURE** — SpecForge has zero local `CLAIM_VERIFICATION.md`, zero claim registry/checker,
  zero pull-request template, 39 nonhistorical governed Markdown surfaces, 14 explicit derived-state contracts,
  33 tracked checker files, and 24 self-test-capable checker fronts.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_doctrines.sh` distinguishes structural/oracle/evidence checks
  but no repository contract joins re-derivation, a dimensionally different falsification oracle, and tracked
  stale-state durability for each actionable published claim; existing task acceptance checks implementation
  evidence shape, not the authority of numbers later repeated in maintained documentation.
- [x] **ADDRESSED (verified)** — the six-leaf adoption owns the local standard, author/reviewer contract, bounded
  claim registry/checker, current-constant census/remediation, tracked-producer and known-bad-RED audit, public
  method documentation, and closure evidence before any implementation begins.
- [x] **NO REGRESSION** — this ownership/audit leaf changes no product code, doctrine authority, limit, decision,
  or mdBook behavior; task catalog, canonical catalogs, derived-state report, live-size self-tests, Knowledge Map,
  and mandatory doctrines pass.
- [x] **GENERICITY** — the taxonomy is authority/lifecycle/measurement based and applies to any project claim; it
  contains no specification, vendor, protocol, corpus, or signal special case.
- [x] **LOCKSTEP** — this task tree, generated task catalog, live status, change/engineering ledgers, and resume
  pointer state that the standard is mapped but not yet adopted, and preserve `.1` behind the existing decision-
  capacity prerequisite.

## Open Questions

- ADR 0044 resolves `.2`'s schema: closed self-bounded JSONL, unique stable ids/statuses, argv commands, exact
  tracked artifact SHA-256, full stale-check membership, and exact publication resolution.
- `.1a` must decide whether measured workflow-standard growth warrants a re-derived flat profile or a bounded
  routed topology; ADR 0043 resolves this in favor of the minimal 21-file flat explicit profile.

## Blockers

- None. `.7` and `.8` are open and block nothing today; `.8`'s capacity stop is roughly 19 slices out.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.5`

- [x] **REPRODUCE / MEASURE** — the clean `b101aaaa` replay closes the current and manual denominators before
  final edits; the final tree reruns both reports, all claim commands, exact RED controls, and scratch census.
- [x] **ROOT CAUSE (WHY + WHERE)** — the implementation was mechanically complete but the book lacked a single
  end-to-end author/auditor example and retrieval lacked the five-architecture join; public status also retained
  a stale authored priority below current completion entries. Closing CI additionally proved that `.2` expanded
  the governed production graph while `tools/production-genericity-graph/src/lib.rs` retained its preceding
  exact-count oracle.
- [x] **ADDRESSED (verified)** — runnable examples cover classification, missing/stochastic evidence, disagreement,
  and review; the relationship fact is derived into bounded retrieval; the priority now agrees with the roadmap;
  the Rust test oracle matches the independently derived current graph without changing graph production.
- [x] **NO REGRESSION** — focused production-graph tests plus selected full CI, all-tier doctrines, current/manual
  claim checks, catalogs, Knowledge Map, mdBook test/build, live-size, locality, and residue checks pass; the only
  Rust change corrects exact test expectations and no roadmap byte changes occur.
- [x] **GENERICITY** — examples depend on claim lifecycle, evidence dimensions, Git identity, and currentness,
  never a specification, vendor, protocol, language, document, signal, or corpus identity.
- [x] **LOCKSTEP** — standard/decisions, claim registry, book, five-architecture fact, public status, task,
  ledgers, and resume pointer agree the fifth architecture is adopted and `.7` is the next product frontier.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.4`

- [x] **REPRODUCE / MEASURE** — five verified claims cite 7 falsification controls through 6 unique producers;
  all 7 controls have exact known-bad source regions and the ignored/untracked governed census is 0/0.
- [x] **ROOT CAUSE (WHY + WHERE)** — executable control commands and tracked artifact joins prove current
  behavior but do not prove which durable known-bad case made each control discriminating; the workflow catalog
  feasibility probe had no controlled RED path in the cited command.
- [x] **ADDRESSED (verified)** — exact producer regions now bind every cited RED case; the workflow probe observes
  its controlled sub-ceiling failure; 27/27 fixtures make missing/stale/misdirected regions and ignored/untracked
  producer candidates RED.
- [x] **NO REGRESSION** — all governed controls, claim/current/book reports, catalogs, task/memory/book,
  live-size/locality, mdBook test/build, and mandatory doctrines pass; underlying source assertions and the
  frozen authority outcomes stay unchanged.
- [x] **GENERICITY** — closure keys claim/control IDs, tracked producer paths, exact regions, Git state, and
  controlled observations without depending on a product subject, protocol, vendor, language, or metric value.
- [x] **LOCKSTEP** — checker/schema, claim records, standard/toolbox/book, fact retrieval, task, ledgers, and
  resume pointer publish 7 controls / 7 exact RED regions / 6 producers / 0 ignored / 0 untracked, while the
  current census closes 84 candidates as 51 exact + 33 registered + 0 unresolved.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3c`

- [x] **REPRODUCE / MEASURE** — clean commit `50775894c526108145aa978a854353dbd214f1af` independently replays
  39 current surfaces, five views, 56 evidence units, and 79 produced candidates closed by 51 exact keys plus 28
  current registered annotations with zero unresolved. `[claim: current-claim-census-frozen]`
- [x] **ROOT CAUSE (WHY + WHERE)** — frozen evidence validates chosen classifications, but without a reverse
  candidate join a newly produced non-tagged authority marker could remain outside the frozen result.
- [x] **ADDRESSED (verified)** — frozen validation now joins every produced candidate to exact evidence or a
  current registered annotation; the 27-case fixture instantiates and drives RED all five outcome families plus
  every surface/view/path/region/source/identity coverage boundary, including a silent derived marker.
- [x] **NO REGRESSION** — repaired vector and 75 inner incompletes are unchanged; syntax, real check/report,
  candidate production, both self-test matrices, claim/catalog/Knowledge Map, task/memory/book/live-size/locality,
  mdBook test/build, and mandatory doctrines pass from the clean boundary.
- [x] **GENERICITY** — closure keys only surface, semantic view, repository-relative path, exact line, lifecycle
  authority, and claim status; no subject, protocol, vendor, language, metric, or document title changes behavior.
- [x] **LOCKSTEP** — checker/report, claim record, fact card/projections, task, memory, ledgers, toolbox, and mdBook
  expose the same 79 = 51 + 28 + 0 closure and preserve the separate 304-region/75-incomplete book authority.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.4`

- [x] **REPRODUCE / MEASURE** — the repaired report derives 39 current surfaces, five views, and 56 exact units:
  11 derived + 7 identity-gated + 6 registered + 0 incomplete + 32 excluded.
  `[claim: current-claim-census-frozen]`
- [x] **ROOT CAUSE (WHY + WHERE)** — each frozen gap attached a surface-wide truth question to a deterministic H1
  review anchor. Identity, route completeness, and actionable numeric truth require distinct evidence units.
- [x] **ADDRESSED (verified)** — all five H1s have narrow identity exclusions; three route regions execute their
  existing producers, workflow capacity joins its verified claim, and mdBook quantitative scope joins its verified
  exact-region mapping without promoting any of the 75 inner incomplete assertions.
- [x] **NO REGRESSION** — canonical parent/current semantic comparison preserves all 46 non-frontier outcomes and
  changes only five frontier outcomes plus five additions; census self-test/report, every joined authority/control, fact/Knowledge Map,
  task/memory/book/live-size/locality, mdBook test/build, and mandatory doctrines pass.
- [x] **GENERICITY** — classification follows identity, navigation, authored policy, and registered quantitative
  authority boundaries; no document subject, vendor, protocol, language, or metric value selects an outcome.
- [x] **LOCKSTEP** — claim record, fact card/projections, task frontier, resume pointer, ledgers, toolbox, and
  mdBook publish the same 56-unit vector and the same outer-complete/inner-incomplete boundary; `.3b` closes.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.3.3`

- [x] **REPRODUCE / MEASURE** — the registered claim replays 39 book files, 304 candidates across 21 files, and
  304 exact regions with the frozen 8 registered / 75 incomplete / 221 excluded vector.
  `[claim: mdbook-quantitative-census-frozen]`
- [x] **ROOT CAUSE (WHY + WHERE)** — a complete semantic map and the truth of every mapped assertion are different
  claims. The new authority verifies the exact map while its assertion, fact card, and task evidence preserve the
  underlying 75-line uncertainty instead of citing the map as semantic proof.
- [x] **ADDRESSED (verified)** — report/check/producer, nineteen-case controlled suite, claim registry/stale joins,
  fact-card catalog, Knowledge Map derive-and-diff, and exact artifact digests pass independently.
- [x] **NO REGRESSION** — outer current census, claim registry, task/memory catalogs, live-size/locality, mdBook
  test/build, and all mandatory doctrines pass; `.3b.4` remains the only result-rewrite leaf.
  `[claim: current-claim-census-frozen]`
- [x] **GENERICITY** — the claim proves exact candidate/region/authority lifecycle and missing-leg representation,
  not a chapter subject, product metric value, protocol, vendor, language, or unchecked semantic conclusion.
- [x] **LOCKSTEP** — claim, fact card, bounded retrieval projections, task frontier, resume pointer, ledgers,
  toolbox, and mdBook expose the same result and uncertainty boundary; `.3b.3` closes.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.3.2`

- [x] **REPRODUCE / MEASURE** — frozen replay covers 304/304 emitted candidates exactly once across 21 files:
  eight registered, 75 incomplete, and 221 excluded under four closed scope reasons.
- [x] **ROOT CAUSE (WHY + WHERE)** — first replay located a dimensional validator defect at the contract-shape
  call: the complete record list was incorrectly treated as one field array, which inventory's tiny source set
  could not expose. Semantic review separately showed that title or section identity cannot classify line scope.
- [x] **ADDRESSED (verified)** — every exact line is adjudicated; the array/record bound is separated; nineteen
  positive/RED cases, frozen check/report/producer, an independent outcome diff, and current-language/past-tense
  competing scans pass with no uncovered, overlap, stale, unresolved, or classification-map difference.
- [x] **NO REGRESSION** — current census, claim registry, task/memory catalogs, live-size/locality, mdBook
  test/build, and all mandatory doctrines pass. The outer 51-unit result is unchanged until `.3b.4`.
  `[claim: current-claim-census-frozen]`
- [x] **GENERICITY** — exact syntax discovery and authority classes use lifecycle/evidence semantics only; no
  document subject, task name, protocol, vendor, language, or metric value changes checker behavior.
- [x] **LOCKSTEP** — frozen contract, checker, toolbox, task frontier, resume pointer, ledgers, and mdBook publish
  the same outcome vector and leave independent result authority to `.3b.3.3`.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.3.1`

- [x] **REPRODUCE / MEASURE** — the executable inventory derives 39 governed book files and 304 candidate lines
  across 21 files; an independent replay against the `.3b.3.0` parent proves the corrected denominator predates
  this implementation and is not source drift.
- [x] **ROOT CAUSE (WHY + WHERE)** — the design-time shell estimate undercounted three stable lines and retained
  neither code nor output, while the new tracked parser makes membership, fence state, grammar, and expected
  totals one reproducible fail-closed transaction.
- [x] **ADDRESSED (verified)** — syntax, self-test, check, report, and producer modes pass; the eighteen-case matrix
  observes every frozen missing/overlap/stale/untracked/unknown/authority/bound class go RED.
- [x] **NO REGRESSION** — current census, claim registry, task/memory catalogs, live-size/locality gates, mdBook
  test/build, and all mandatory doctrines pass; semantic regions and the outer frozen outcome vector are
  unchanged. `[claim: current-claim-census-frozen]`
- [x] **GENERICITY** — book membership, Markdown fences, dimensional syntax, exact regions, and authority joins
  contain no SpecForge feature, document, protocol, vendor, language, or metric-value special case.
- [x] **LOCKSTEP** — contract, checker, toolbox, task frontier, resume pointer, ledgers, and contributor-facing
  mdBook agree that the inventory is implemented and `.3b.3.2` owns adjudication.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.3.0`

- [x] **REPRODUCE / MEASURE** — the dated design scan provisionally estimated 301 quantitative-looking lines
  across 21 files; `.3b.3.1`'s tracked executable replay corrects that non-reproducible estimate to 304 without
  changing the frozen grammar or book input.
- [x] **ROOT CAUSE (WHY + WHERE)** — the frozen `SUMMARY.md` title anchor neither enumerates quantitative prose
  nor distinguishes current actionable assertions from examples, authored thresholds, identities, and dated
  evidence; the selective book-currentness checker covers only named product seams.
- [x] **ADDRESSED (verified)** — exact coverage, non-overlap, SHA regions, closed authority/exclusion outcomes,
  tracked verifier/claim joins, fence parsing, bounds, and staged inventory/adjudication/result leaves are frozen.
- [x] **NO REGRESSION** — design changes no book assertion, product code, checker, census outcome, registry schema,
  hook, CI path, or limit; current census, task/memory/book/live-size/claim and doctrines pass.
  `[claim: current-claim-census-frozen]`
- [x] **GENERICITY** — candidate discovery uses Markdown/fence and dimensional syntax only; authority follows
  lifecycle and evidence, never a chapter, protocol, vendor, metric value, model, or language.
- [x] **LOCKSTEP** — task, resume pointer, ledgers, and mdBook agree on design→inventory→adjudication→freeze;
  `.3b.4` remains the only leaf permitted to change the outer 51-unit census.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.2`

- [x] **REPRODUCE / MEASURE** — the tracked capacity producer derives the registered workflow profile from the
  explicit collection and Git creation history; the exact published paragraph is in the registered authority.
  `[claim: workflow-standard-capacity-profile]`
- [x] **ROOT CAUSE (WHY + WHERE)** — the frozen first-line pull-request-template anchor conflated authored
  normative workflow identity with one repository-derived capacity assertion in
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`.
- [x] **ADDRESSED (verified)** — `.3b.4` will exclude only the title identity and add an exact registered region
  for the capacity paragraph; it will not synchronize another literal or create another capacity authority.
- [x] **NO REGRESSION** — the capacity derivation, five controlled boundary cases, independent catalog-
  feasibility probe, canonical catalog, frozen census, claim, book, live-size, and doctrine checks pass; source
  policy and all 51 census outcomes remain unchanged. `[claim: current-claim-census-frozen]`
- [x] **GENERICITY** — the split follows authored-policy versus repository-derived measurement authority and
  applies without inspecting the policy subject, harness, vendor, protocol, or numeric spelling.
- [x] **LOCKSTEP** — task, resume pointer, ledgers, and mdBook name the existing claim as sole quantitative
  authority; `.3b.3` owns the remaining mdBook-wide placeholder before `.3b.4` changes census results.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.1`

- [x] **REPRODUCE / MEASURE** — the README route check, fact-card catalog check, and canonical collection check
  each derive the current tracked route from its registered authority rather than comparing hand-carried lists.
- [x] **ROOT CAUSE (WHY + WHERE)** — the frozen census used first-line anchors and did not join these existing
  route producers; it did not expose an absent membership control or a stale route.
- [x] **ADDRESSED (verified)** — the final binding is frozen: title anchors are non-claim document identities,
  while separate exact route regions execute the README, fact-card, and canonical-catalog authorities.
- [x] **NO REGRESSION** — real route checks and their missing, drift, residue, unsafe-path, and bound controls pass;
  the 51 census outcomes, source assertions, product code, registry schemas, hooks, CI paths, and limits do not
  change in this evidence-only slice. `[claim: current-claim-census-frozen]`
- [x] **GENERICITY** — route validity depends on registered path membership and generated projection identity,
  never a member's title subject, vendor, protocol, prose, numeric contents, or implementation language.
- [x] **LOCKSTEP** — task, resume pointer, ledgers, and mdBook agree that the three authorities are proven but the
  frozen census changes only once in `.3b.4`; `.3b.2` owns the workflow baseline next.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3b.0`

- [x] **REPRODUCE / MEASURE** — the frozen frontier contains exactly five incomplete keys: three maintained-
  reference collection anchors, one workflow/doctrine anchor, and one mdBook quantitative anchor; the other 46
  outcomes remain outside repair scope. `[claim: current-claim-census-frozen]`
- [x] **ROOT CAUSE (WHY + WHERE)** — all five point at first-line review anchors. A title can identify a surface
  but cannot prove navigation completeness, policy semantics, or the current truth of quantitative prose.
- [x] **ADDRESSED (verified)** — `.3b.1` owns exact route/catalog identity, `.3b.2` splits authored workflow
  identity from the registered capacity paragraph, `.3b.3` owns exact mdBook quantitative regions, and `.3b.4`
  alone re-freezes changed identities and outcomes.
- [x] **NO REGRESSION** — planning changes no source assertion, census classification, product code, producer,
  registry schema, hook, CI path, or bound; focused census/currentness/book checks and doctrines pass.
- [x] **GENERICITY** — the split follows assertion authority and lifecycle, not document subject, protocol,
  vendor, metric name, numeric spelling, or implementation language.
- [x] **LOCKSTEP** — task frontier, resume pointer, change/engineering ledgers, and contributor-facing mdBook
  name the same four-step repair sequence while preserving the frozen 51-unit result.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3a.2`

- [x] **REPRODUCE / MEASURE** — the frozen report derives 39 current surfaces, 32 inspection dispositions, seven
  surface exclusions, five evidence-covered views, and 51 exact units: 11 derived + 4 identity-gated + 4
  registered + 5 incomplete + 27 excluded. `[claim: current-claim-census-frozen]`
- [x] **ROOT CAUSE (WHY + WHERE)** — the first production pass keyed surface anchors without `view_id`, allowing
  a multi-view surface to satisfy surface coverage while leaving one semantic view without evidence.
- [x] **ADDRESSED (verified)** — producer identity now includes surface + view + path + line; frozen validation
  requires every included surface and every required view; all 51 units bind tracked line hashes and one closed
  authority result, and all 15 derived/identity verifiers execute successfully.
- [x] **NO REGRESSION** — 15/15 fault controls, frozen check/report/producer derive-and-diff, all joined focused
  verifiers, claim/task/memory/book/live-size/currentness checks, and mandatory doctrines pass; source assertions,
  product code, hooks, CI, and the five measured gaps remain unchanged.
- [x] **GENERICITY** — the finding split follows authority and lifecycle classes only; the five incomplete keys
  identify governed surfaces, not vendor, protocol, document-title, model, language, or metric exceptions.
- [x] **LOCKSTEP** — contract, task evidence, dated ledgers, mdBook, registered frozen claim, and resume pointer
  publish the same 51-unit outcome vector and exact `.3b` frontier; the 47-candidate inventory claim is superseded.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3a.1`

- [x] **REPRODUCE / MEASURE** — the real report derives 39 current surfaces, assigns 32 to inspection and seven
  to explicit standard-scope exclusions, covers all five required views, and emits 47 deterministic candidates
  while the inventory phase correctly contains zero frozen evidence units.
- [x] **ROOT CAUSE (WHY + WHERE)** — a hand-read constant sweep could omit a lifecycle, surface, overlapping
  semantic view, or stale evidence region; the missing seam was an executable join from the live registry to
  exact tracked source regions and closed authority outcomes.
- [x] **ADDRESSED (verified)** — the bounded JSONL contract and checker derive the denominator, expand exact
  tracked surface paths, validate line-range digests, execute direct argv verifiers, resolve current claim and
  derived-state authority IDs, and separate deterministic production from later result adjudication.
- [x] **NO REGRESSION** — syntax, real check/report/producer, 14/14 controlled cases, task/memory/live-size/book/
  claim checks, and mandatory doctrines pass; no source assertion, product file, hook, CI path, or existing
  authority outcome changes.
- [x] **GENERICITY** — selection uses registered lifecycles, semantic-view IDs, exact paths/regions, and authority
  classes; no protocol, vendor, document title, metric spelling, model, or language receives special validity.
- [x] **LOCKSTEP** — task evidence, dated ledgers, contributor-facing mdBook, and resume pointer all state the
  inventory-only boundary; `.3a.2` alone freezes findings and `.3b` alone may repair measured gaps.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.3a.0`

- [x] **REPRODUCE / MEASURE** — the live-size registry has 55 governed Markdown surfaces: removing 15
  `archive_terminal` and one `frozen_legacy` surface derives the exact 39-surface current denominator.
- [x] **ROOT CAUSE (WHY + WHERE)** — `.3` combined denominator selection, semantic classification, repair, and
  closure in one leaf, allowing measurement rules to move while their own findings were being repaired.
- [x] **ADDRESSED (verified)** — `.3a.0` freezes five category views, unique evidence-unit keys, exact tracked
  path/region identity, five closed authority outcomes, required verifier/claim joins, bounds, and sequencing.
- [x] **NO REGRESSION** — only task/continuity state changes; task catalog, memory, claim registry, live-size, and
  all gate-tier doctrines pass; no source assertion, registry schema, checker, policy, book, or product file moves.
- [x] **GENERICITY** — denominator and outcomes depend on lifecycle/evidence authority, never a feature, document,
  protocol, vendor, model, metric name, or numeric spelling.
- [x] **LOCKSTEP** — task frontier and resume pointer agree `.3a.1` implements the frozen producer next, `.3a.2`
  freezes findings, and `.3b` alone may repair source assertions.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.2`

- [x] **REPRODUCE / MEASURE** — the real checker reports three verified claim records, eight executed source/
  control commands, exact tracked artifact identities, and pending-message or `HEAD` publication resolution.
- [x] **ROOT CAUSE (WHY + WHERE)** — the active prose contract had no join between claim IDs, rerunnable evidence,
  complete artifact identity, stale-state behavior, and the existing doctrine composition seam.
- [x] **ADDRESSED (verified)** — ADR 0044, strict bounded JSONL, direct argv execution, tracked SHA-256 artifacts,
  complete stale membership, honest status semantics, message resolution, and one gate-tier row land together.
- [x] **NO REGRESSION** — 22/22 controlled cases, all eight declared commands, claim report, canonical/task/
  decision/Knowledge Map catalogs, README/current-book/live-size/locality checks, mdBook, and doctrines pass; no
  product Rust, derived-state contract, hook, CI workflow, claim threshold, or workflow capacity changes.
- [x] **GENERICITY** — records classify assertion lifecycle/evidence roles and execute opaque argv; no chip,
  document, vendor, protocol, corpus, model, language, or metric name grants validity.
- [x] **LOCKSTEP** — standard, ADR 0044, registry, checker, doctrine standard/driver, toolbox/commit workflow,
  mdBook, task/current status, ledgers, Knowledge Map, and resume pointer agree that provenance gating is active.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.1a`

- [x] **REPRODUCE / MEASURE** — the tracked producer reads all 14 explicit targets and Git history: additions by
  active date are 2/1/4/2/3/2, so the peak is four; current members total 2,632 lines / 166,521 bytes.
- [x] **ROOT CAUSE (WHY + WHERE)** — `.1` legitimately added two standards to a fixed 16-file canonical surface,
  crossing count warning; member shape, catalog, and registry topology are not the binding dimensions.
- [x] **ADDRESSED (verified)** — ADR 0043 selects the minimal 21-file profile, retains explicit stable targets,
  and moves aggregates to 14,700 lines / 1,376,256 bytes with unchanged per-file limits.
- [x] **NO REGRESSION** — five measurement controls, canonical catalog, decision/fact/task catalogs, Knowledge
  Map, live-size, mdBook, and doctrines pass; no standard, route, product file, or old decision moves.
- [x] **GENERICITY** — the equation uses lifecycle, current population, Git growth, and fixed milestones; it never
  inspects a standard's subject, title, author, harness, or content.
- [x] **LOCKSTEP** — producer, ADR 0043, exact authority, surface registry, containment prose, mdBook, task/current
  status, change/engineering ledgers, and resume pointer publish the same 14 / 4 / 21 profile.

## Acceptance Checklist (enforced) — `CLAIM-VERIFICATION-ADOPTION.1`

- [x] **REPRODUCE / MEASURE** — `.0` established zero local standard, registry/checker, or review template; `.1`
  adds one root standard, ADR 0042, one PR template, and exact routes in a 14-member workflow catalog.
- [x] **ROOT CAUSE (WHY + WHERE)** — existing task/doctrine/currentness mechanisms did not join a source
  reproduction, a different competing-hypothesis oracle, and tracked stale-state durability per actionable claim.
- [x] **ADDRESSED (verified)** — the standard freezes scope, three legs, stochastic/auditor rules, honest missing-
  leg status, and exact `Published-claims:` syntax across bootstrap, README, COMMIT, TOOLBOX, PR review, and book.
- [x] **NO REGRESSION** — workflow/decision/fact/task catalogs, README routes, Knowledge Map, mdBook, live-size,
  and all gate-tier doctrines pass; no product code, existing decision, limit, or currentness checker changes;
  the resulting legal 14/16 workflow-file warning is explicitly owned by `.1a`; development notes roll over
  losslessly to 61 records / 1,281 lines / 173,852 bytes with all older archive members byte-identical.
- [x] **GENERICITY** — classification depends on assertion authority/lifecycle and independence of evidence, not a
  SpecForge feature, protocol, corpus, vendor, language, harness, or metric name.
- [x] **LOCKSTEP** — ADR 0042, root standard, discovery routes, author/review workflows, mdBook, current status,
  task evidence, and resume pointer agree the contract is published while mechanical provenance remains `.2`.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-30` | `.7.1a` | independent widened-lookahead probe over the same governed paragraphs; `perl scripts/check_published_assertions.pl --self-test`; `--check`/`--produce` on the real tree; revert-and-re-apply of the grammar line alone; `scripts/check_doctrines.sh` | **the gate could not see two of the values it exists to watch.** An oracle built to disagree — the gate's own lookbehind kept, only the lookahead widened — reported **19** published values dropped across the four claim-annotated files: 16 compound-adjective forms (`27-case`, `56-unit`, `304-region`), two ratio halves (`304/304`, `15/15`), and one comma over-capture. A first probe without the lookbehind was **discarded**, not published: it reported `SHA-256` and `H1`, which the gate refuses correctly, and an oracle that calls correct behaviour a defect separates nothing. Two of the 19 sit on the current-facing surfaces and one is live — `TOOLBOX.md`'s "The 27-case self-test" is a current count of `check_current_claim_census.pl --self-test` (27/27 today) inside a claim-annotated paragraph the gate was written to watch. Real-tree unlisted **21 -> 23**. **Attribution by revert-and-re-apply, not by reading**: with the grammar line alone reverted, 5 of 19 self-test cases fail — both positives on `published value '40,' ... no assertion record lists`, and all three new cases; re-applied, **19/19**. The absorbed-punctuation case is the mirror the repair needs: a record whose `value` is `40,` now covers nothing, where before it was accepted |
| `2026-08-30` | `.7.1` | `perl scripts/check_published_assertions.pl --self-test`; `--check` on the real tree; revert-and-re-apply drift probe against `check_current_claim_census.pl --report`; `scripts/check_doctrines.sh` | **the leg no existing control supplied is now executable.** Self-test **16/16** with every `.7.0` fault plus five more driven RED on a disposable repository-local fixture. On the real tree the gate runs green at four seeded assertions over two governed regions in `inventory` phase, reporting unlisted values rather than failing on them until `.7.2` completes the population. **Drift observed RED on real shipped prose**, not a fixture: `TOOLBOX.md`'s `**5** views` bound to the census `views` field, edited to `**6**`, produced `is stale: 'views' re-derives to '5', published '6'`; both files restored byte-exact. A first probe against `docs/knowledge/INDEX.md` is recorded as an honest failure — its producer is derive-and-diff over that same file and exits nonzero before any field comparison, so it cannot demonstrate this leg. The coverage grammar was corrected mid-implementation from tag-line to **paragraph** scope, because a `[claim: <id>]` tag closes a paragraph and `TOOLBOX.md`'s two tags sit on lines carrying no quantity at all — keyed on the tag's own line the map would have been blind to precisely the sentences it exists to watch |
| `2026-08-30` | `.7.0` + `.11a` re-derivation | re-derived every figure `.11a` published: `git show --name-only` over the four attributed commits; `git show d23e8bae -- <book>` content-line count; `check_live_document_size.pl` warning population; literal match census over tracked non-archive Markdown at `fd09708d~1`; `§10` rows vs `DOCTRINES`; the three claim producers `--check`/`--report` | **`.11a` does not fully hold either.** Its warning-line total was **invalidated by its own commit** — published as 20, and that commit's `CHANGES.md` prepend crossed `change_history lines_each`, making it 21. And "exactly three surfaces" is a *classification*, not a match count: five tracked files carried the literal, two of them dated records §1 exempts; the conclusion holds and the predicate was unstated. Both are withdrawn rather than corrected. Everything else in `.11a` re-derives: the republication attribution, `d23e8bae`'s two changed book lines, the four-producer composition, the `done`-tree and self-satisfying classifier defects, and the dated-table withdrawal. **Five consecutive rounds of correction, each invalidated by its own transaction, is the evidence that closes the design question**: `.7` is split and `.7.0` freezes the contract |
| `2026-08-30` | `.11a` | `git show --name-only` over the four attributed commits; `git show d23e8bae~1:<book>`; `check_live_document_size.pl` vs `check_live_document_size.sh` warning populations; open-tree ownership screen recomputed from each tree's own `Status` line; all three claim producers `--check`/`--report`; 17 relocations re-derived HEAD vs HEAD~1; doctrine gate | **five of `.11`'s findings do not hold.** Only **one** of the three commits said to have republished `identity_gated 7` edited a publisher. `d23e8bae` withdrew `current_surfaces` from **two** surfaces, not three — the book chapter never carried it, and "all three" was quoted from that commit's own body. The **18 lines / 13 surfaces** census was one producer's, and only 17 of its lines named a surface; the gate-level producer emits **35** lines across **four**. The ownership screen counted `done` trees as owners, hiding `corpus_task_evidence_parts`, and the act of publishing the finding turned its own classifier green for the three surfaces it named. And the `Opening Pressure Boundary (92e59c97)` table is a **dated** snapshot §1 exempts, not a disagreeing census. Everything else re-derives unchanged: `identity_gated` 8 on three surfaces, 78 -> 89, 307/78/221/186 -> 321/89/224/189, 10/nine -> 11/10/1 at `1ccb7331`, the sibling frontier, the three withdrawals, 15 -> 16, 17 relocated + 1 added, 11 = 11, 5 book regions. `.11`'s own ledger prepends also moved the warned surface count 13 -> **15** (`achievement_status`, `change_history`; both owned) |
| `2026-08-30` | `.11` | enumerating command over tracked non-archive Markdown (anchored at `9fc76685`); per-revision re-derivation of `identity_gated` from `git show <rev>:doctrine/claim_verification/current_claim_census.jsonl` across 25 revisions; `check_current_claim_census.pl --check`/`--report`; `check_book_quantitative_claims.pl --check`/`--report` before and after the chapter edit; `check_fact_card_catalog.pl --write`/`--check`; knowledge-map derive-and-diff; `check_claim_verification.pl --check`; the three claim self-tests; doctrine gate | the population is **15** surfaces at `9fc76685` and **16** at this commit — writing the change record joins it — so the number is anchored and the command is the durable artifact; either way not the two `.6b` swept. **Six stale publications and three carried-but-correct ones**, withdrawn as one class: `.6`, `.6a`, and `.6b` each swept from memory and none ran an enumerating command. `identity_gated` **7 -> 8** on three surfaces, attributed to `d23e8bae`, which added exactly one `identity_gated` record for the `task_tree_catalog_parts` surface it registered — the same registration whose other effect (`current_surfaces` 39 -> 40) that commit *did* notice and withdraw, one sentence away; the book chapter's **78 -> 89** incomplete regions; the whole `.3b.3.3` vector (**307/78/221/186** -> 321/89/224/189) still carried in `mdbook-quantitative-census-freeze.md`, whose stale twin `.6b` repaired while quoting these numbers off it; and a registry set claim short by one member (**10/nine/ten** -> 11 registered / 10 gate / 1 CI, `PROOF-SEAL-CURRENCY` missing since `1ccb7331`); `MEMORY.md`'s five-surface live-document warning census against a producer that warns about **13**, three named by no tree; and this tree's sibling `LIVE-DOCUMENT-PRESSURE-HEADROOM` frontier still calling `.2a`/`.2b`/`.2c` pending. Three further constants are carried and still correct and go with them — `authority_outcomes.derived`, the mdBook claim assertion's eleven constants, and `control_audit`'s 7/7/6. All withdrawn as one class per §3 Leg 3 rather than replaced with today's values. The survivors are stated with what makes them true: `unresolved` **0** is gated by `validate_candidate_closure`, `views` **5** is authored by `.3a.0`. Book vector **321/8/89/224 identical** across the chapter edit; **17** regions relocated (16 census, one book) and one evidence row added for the new ledger head, derived by diffing both registries against HEAD rather than summing the passes — which first gave 18, `.9`'s ninth instance caught by deriving it; each relocation matched exactly one line and each file's shift equalled the cumulative insertions above it from the diff hunks; catalog valid for 249 cards; Knowledge Map 271 facts / 2152 keys; self-tests 27/27, 27/27, 19/19 |
| `2026-08-30` | `.10a` correction | replayed the narrow absence probe against `HEAD~1:CLAIM_VERIFICATION.md`; enumerated the census-governed surface set from `current_claim_census.jsonl`; `check_rolling_ledger_protocol.pl --report`; doctrine gate | the narrow probe ran **seventeen** terms, not fifteen — all seventeen zero. Fifteen is the *discriminating* subset and the count the **widened** probe ran; published on four surfaces as the narrow probe's size, which is `.7`'s ninth instance one commit after it was written. Also corrected an unenumerated set claim ("visible to no gate"), now counted: not a registry record, and carrying no numeral, so no candidate grammar reaches it — `MEMORY.md` is a governed census surface with four evidence rows and still produced none. Findings two and three re-derive **unchanged**; the ledger figures come from the producer, which now emits no warning for `development-notes` at all |
| `2026-08-30` | `.10` | section-by-section re-read of the upstream standard; seventeen-term absence probe over `CLAIM_VERIFICATION.md` alone, then a fifteen-term re-run over all governed claim surfaces; `check_book_quantitative_claims.pl --check` before and after the mdBook edit; knowledge-map derive-and-diff; `check_fact_card_catalog.pl --write`/`--check`; doctrine gate | source unchanged since `.1` (mtime `2026-08-26`), so the gap dates from the original adoption. The narrow probe returned zero hits for all seventeen terms in one file; **widening it to every governed claim surface changed the answer** — ADR 0042 already carried the general form as Context rationale. That is Leg 1's granularity rule catching its own adoption commit, and it is why the published set is scoped to "no home on any governed claim surface" rather than "absent from the standard". Remaining matches classified as unrelated word collisions before publishing. Book census **321/8/89/224 unchanged** across the edit; one region re-pointed 547 -> 580, line SHA-256 identical; Knowledge Map 271 facts / 2148 keys after one question-key collision was resolved in the fact card's favour; catalog valid for 249 cards |
| `2026-08-30` | `.6b` correction | re-derived the move count and staleness window from `git show <rev>:doctrine/claim_verification/book_quantitative_claims.jsonl` across its 31 revisions, with commit timestamps | `.6b` published "**seven** registry changes" and "stale for **twelve days**". Both wrong: **six** moves, and the chapter stayed *correct* until `4dac5642` (`2026-08-27 02:14`), repaired at `2b9e8899` (`2026-08-29 22:23`) — **two days and twenty hours** stale. The twelve-day span was writing-to-settling, substituted for staleness. Every other figure in the finding re-derives; corrected on all four live surfaces and logged as `.7`'s ninth instance |
| `2026-08-29` | `.6b` correcting `.6a` | outcome/row census of `current_claim_census.jsonl` at `fdda3c53` and `5fe81128` from Git blobs | both revisions hold **59** evidence rows and **5** `CHANGES.md` rows, differing by exactly **one removed / one added**. `.6a`'s published mechanism ("sealed 18 while adding 2") is **false**; the total held because one retirement cancelled one addition. Every number `.6a` published re-derives and the withdrawal stands. Recorded as `.7`'s seventh instance: a count-only gate passes a false mechanism |
| `2026-08-29` | `.6b` | outcome census over 31 registry revisions of `book_quantitative_claims.jsonl`; `check_book_quantitative_claims.pl --check`/`--report`/`--produce`; `check_current_claim_census.pl --check`; knowledge-map derive-and-diff; fact-card catalog; doctrine gate | the book's `regions=307/8/78/221` was **correct when written** at `be3b12e6` (`2026-08-16`) and then moved **six times** to 321/8/89/224 by `fdda3c53`, going stale at the first of those (`4dac5642`, `2026-08-27`) and repaired at `2b9e8899` — stale for two days and twenty hours; "56 exact evidence units" was correct at `50775894` and false from `e6f5012d`. Both withdrawn and routed to `--report`, the fact card retitled, and `TOOLBOX.md`'s last carried mdBook count withdrawn for consistency. Post-edit re-derive: **321/8/89/224 unchanged**, 39 book files / 21 candidate files. One frozen region re-pointed 540 -> 547. For `.9`: the chapter held **5** candidate lines in 557 and none was a drifted one — `units` and backticked `key=value` are both outside the grammar |
| `2026-08-29` | `.6a` | 28-consecutive-revision worktree trajectory (`e6f5012d` -> `60a81db7`) plus the `50775894` anchor, each with its own `check_current_claim_census.pl --report`; `check_claim_verification.pl --report`; `check_book_quantitative_claims.pl --report`; census `--self-test`; doctrine gate | `registered` **6 -> 5 -> 4** and closure **86/51/35 -> 72/50/22 -> 69/49/20**, stepping at `5fe81128` and `1507adbf`; so all four counts `.6` called confirmed were false **inside `.6`'s own commit**. `.6`'s unit trajectory is also corrected (**59**, not 60, at `5fe81128`) and its "one more excluded unit per rolling-ledger head" rule withdrawn — 15 rises, **2 falls**, 10 no-changes over 27 transitions. Stable across all 29: derived **11**, identity-gated **7**, no `incomplete`, unresolved **0**, surfaces **39**, views **5**; carried with producer fields named. `7/7/6/0/0` control audit and mdBook **89** incomplete re-derive; self-test **27/27** |
| `2026-08-29` | `.7` fifth stale-count instance | `test_live_document_size.pl`; `check_fact_card_catalog.pl --self-test`; `test_derived_state_contracts.pl`; `test_derived_state_authorities.pl`; `check_task_tree_archive.pl --self-test`; `check_active_task_evidence.pl --self-test` | producers report **84 / 60 / 47 / 25 / 15 / 44**. `DOCTRINE_ENFORCEMENT.md` published 84/**58**; the book published **81**/**58**. Two stale counts repaired; the other four re-derive. The book's stale `81` sits at a line the frozen census holds **no** region for; its stale `58` sits at a line the census holds an **incomplete** region for — one outside the denominator, one inside it and explicitly unverified. Neither a digest binding nor the frozen census can observe either, which is `.7`'s whole point |
--- |
| `2026-08-15` | `.0` | full source-standard read; claim/review/derived-state/doctrine/control/tracked-
  producer/capacity census; task catalog; derived-state report; canonical catalogs; live-size self-tests |
  `passed`; mapped 39 current surfaces, 14 derived-state contracts, 33/24 tracked/self-test checkers, zero
  governed ignored producers, and the 42/44 decision prerequisite without claiming adoption |
| `2026-08-15` | `.1` | route/catalog self-tests and real checks; contract phrase probe; tracked path
  census; Knowledge Map; mdBook; live-size; doctrines | repository-owned standard/ADR/template published; exact
  claim/no-claim declaration active; 14 workflow standards and 36 README routes close; exact segment 0008 seals
  15 engineering records and restores the 61-record live root below warning; registry gate remains pending |
| `2026-08-15` | `.1a` | tracked Git-growth report + five controls; full-catalog arithmetic; catalog/KM/
  live-size/mdBook/doctrine gates | 14 current / peak 4 derive 21; current 66.7%, one peak 85.7%; explicit paths
  retained; exact one-use authority remained for `.1b` and was subsequently retired |
| `2026-08-15` | `.1b` | authority census; capacity reproducer; canonical catalog; task/KM/currentness;
  live-size; doctrine gates | registry returns to its one control record; 21-file profile and all evidence stay
  unchanged; `.2` is executable |
| `2026-08-15` | `.2` | checker syntax/report + 22-case RED matrix; 8 executed evidence commands; catalogs/KM/
  README/book/live-size/locality/mdBook/doctrines | 3 verified records current; prepared/HEAD publications resolve;
  tenth doctrine registered through the existing driver only |
| `2026-08-15` | `.3a.0` | live-registry lifecycle census; task catalog; memory; claim check/report; live-size;
  doctrines | 55 total - 15 archive - 1 frozen = 39 current surfaces; five category views and five closed
  outcomes frozen; no source assertion changed |
| `2026-08-15` | `.3a.1` | checker syntax; 14-case fault matrix; real check/report/producer; task/memory/claim/
  live-size/book/doctrine gates | 39 current = 32 inspection + 7 explicit exclusions; five views; 47 deterministic
  candidates; inventory remains unfrozen and no source assertion changed |
| `2026-08-15` | `.3a.2` | multi-view RED reproduction; 15-case fault matrix; frozen check/report/producer;
  15 joined verifiers; claim/task/memory/book/live-size/doctrine gates | 51 units = 11 derived + 4 identity-gated
  + 4 registered + 5 incomplete + 27 excluded; five exact `.3b` keys; no source assertion changed |
| `2026-08-15` | `.3b.0` | five-key authority-route audit; current census/check/report; README/catalog/book/
  task/memory/claim/live-size/doctrine gates | repair sequence frozen without changing any source assertion or
  one of the 51 existing authority outcomes |
| `2026-08-15` | `.3b.1` | README policy real/self-test; fact-card catalog real/self-test; canonical collection
  real/self-test; current census; task/memory/book/claim/live-size/doctrine gates | all three existing route
  authorities derive exact membership and fail closed; frozen census remains unchanged for `.3b.4` |
| `2026-08-15` | `.3b.2` | capacity derivation; five-case boundary controls; independent catalog-feasibility
  probe; canonical catalog; current census; claim/book/live-size/doctrine gates | authored workflow identity is
  separated from the existing registered capacity paragraph without changing source or census outcomes |
| `2026-08-15` | `.3b.3.0` | independent prose-only quantitative scan; design/schema/bound review; current census;
  task/memory/book/claim/live-size/doctrine gates | bounded candidate and exact-region authority design frozen;
  no book assertion or outer census outcome changed |
| `2026-08-15` | `.3b.3.1` | checker syntax; 18-case fault matrix; real check/report/producer; parent-revision
  replay; task/memory/book/claim/live-size/doctrine gates | 39 book files, 304 candidates across 21 files; the
  provisional 301 estimate is corrected; inventory remains region-free and the outer census is unchanged |
| `2026-08-15` | `.3b.3.2` | frozen exact-coverage replay; 19-case matrix; independent outcome diff; excluded-
  current-language and incomplete-past-tense audits; task/memory/book/claim/live-size/doctrine gates | 304 exact
  regions: 8 registered + 75 incomplete + 221 excluded; record/array bound defect repaired; outer census unchanged |
| `2026-08-15` | `.3b.3.3` | clean result replay; 19-case controls; claim rederive/registry/stale joins; fact-card catalog
  and Knowledge Map derive-and-diff; task/memory/book/live-size/doctrine gates | verified census-mapping authority
  freezes 304 regions and preserves 75 incomplete assertions; no underlying assertion is promoted |
| `2026-08-15` | `.3b.4` | parent/current semantic-outcome diff plus exact identity audit; 15-case census controls; all five route/claim joins;
  current and mdBook census reports; fact/Knowledge Map; task/memory/book/live-size/locality/doctrine gates |
  56 exact units: 11 derived + 7 identity-gated + 6 registered + 0 incomplete + 32 excluded; `.3b` closed |
| `2026-08-15` | `.3c` | clean `50775894` replay; 27-case all-family/coverage matrix; candidate-closure report;
  independent 304-region book replay; claim/catalog/Knowledge Map/task/memory/book/live-size/locality/doctrines |
  79 candidates = 51 exact + 28 registered + 0 unresolved; 56-unit vector unchanged; `.3` closed |
| `2026-08-15` | `.4` | 27-case claim-gate matrix; all seven control commands and exact RED regions; controlled
  workflow sub-ceiling; tracked/ignored/untracked producer census; current/book/fact/Knowledge Map/task/memory/
  mdBook/live-size/locality/doctrine gates | 7 controls = 7 exact RED regions across 6 producers; 0 ignored and
  0 untracked candidates; current closure 84 = 51 + 33 + 0; `.4` closed |
| `2026-08-16` | `.5` | clean-boundary current/manual replay; author/auditor review; production-graph exact-
  oracle repair and focused four-test replay; fact/task/public-status/architecture alignment; all-tier doctrines;
  selected full CI; mdBook/live-size/locality/residue gates | 10 doctrines and 11 genericity qualifications;
  1,996 Rust tests / 8 ignored / 0 failed plus 5 compile-fail doctests; adoption closed at 79 modules / 141 rows,
  zero silent current candidates and 304/304 manual candidates adjudicated; exact cleanup removes 12,097
  rebuildable incremental files / about 9.6 GiB plus two zero-byte temp logs; `.7` aligned |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `CLAIM-VERIFICATION-ADOPTION.0 — own and map three-leg claim verification` | standard/local seam audit; implementation remains pending |
| `.7.1a` | `CLAIM-VERIFICATION-ADOPTION.7.1a — measure the coverage grammar's blind spot, then close it` | 19 published values were invisible to the population scanner; grammar repaired on measurement, 19/19 self-test with three grammar-dependent RED cases, real-tree unlisted 21 -> 23 |
| `.7.1` | `CLAIM-VERIFICATION-ADOPTION.7.1 — execute the producer and compare the field` | twelfth registered doctrine; 16/16 RED matrix; drift observed on real shipped prose; coverage corrected to paragraph scope; §10 and the mdBook chapter updated in the same commit |
| `.7.0` | `CLAIM-VERIFICATION-ADOPTION.7.0 — freeze the published-assertion gate design` | eight-element contract traced element-by-element to the instance that defeated the alternative; `.7` split into freeze, gate, and population; `.11a`'s two invalidated figures withdrawn in the same commit |
| `.11a` | `CLAIM-VERIFICATION-ADOPTION.11a — re-derive .11's findings and correct the five that do not hold` | two mechanism claims and two set claims withdrawn; the ownership classifier replaced because publishing the finding satisfied it; producer scope corrected from one checker to the gate |
| `.11` | `CLAIM-VERIFICATION-ADOPTION.11 — enumerate the surfaces, then withdraw the class` | population derived from a command instead of recalled (15 at the parent, 16 here); six stale publications and three carried-but-correct constants withdrawn as one class; every survivor labelled gated or authored; `.7` tenth instance, `.9` third demonstration |
| `.10` | `CLAIM-VERIFICATION-ADOPTION.10 — re-adopt the upstream claim standard` | absent upstream rules made normative; non-adoption recorded with reasons; `.7` scope settled to counts; mandatory `DEVELOPMENT_NOTES.md` rollover to segment 0009 |
| `.10a` | `CLAIM-VERIFICATION-ADOPTION.10a — correct .10's own probe count and set claim` | seventeen-term narrow probe restored on four surfaces; the "no gate sees this" assertion enumerated; findings two and three verified unchanged |
| `.1` | `CLAIM-VERIFICATION-ADOPTION.1 — publish the claim-verification contract` | normative scope, ADR 0042, discovery, authoring/review contract, and mdBook alignment |
| `.1a` | `CLAIM-VERIFICATION-ADOPTION.1a — re-derive workflow-standard capacity` | tracked measurement, ADR 0043, exact authority, stable explicit topology |
| `.1b` | `CLAIM-VERIFICATION-ADOPTION.1b — retire the consumed workflow-capacity authority` | one-use authority removed after the 21-file profile became baseline |
| `.2` | `CLAIM-VERIFICATION-ADOPTION.2 — gate published claim provenance` | bounded registry, executable evidence join, stale digest gate, publication resolution, tenth doctrine |
| `.3a.0` | `CLAIM-VERIFICATION-ADOPTION.3a.0 — freeze the current-claim census design` | 39-surface denominator, five category views, closed authority outcomes, staged implementation/results |
| `.3a.1` | `CLAIM-VERIFICATION-ADOPTION.3a.1 — implement the current-claim census` | bounded inventory contract, deterministic producer, exact region/authority joins, fail-closed controls |
| `.3a.2` | `CLAIM-VERIFICATION-ADOPTION.3a.2 — freeze current-claim census findings` | 51 exact outcomes, complete surface/view coverage, five-key repair frontier |
| `.3b.0` | `CLAIM-VERIFICATION-ADOPTION.3b.0 — freeze the current-claim repair map` | authority-specific route, workflow-baseline, mdBook-quantitative, and closure leaves |
| `.3b.1` | `CLAIM-VERIFICATION-ADOPTION.3b.1 — bind maintained-reference authorities` | existing README, fact-card, and canonical catalog authorities proven sufficient for later atomic binding |
| `.3b.2` | `CLAIM-VERIFICATION-ADOPTION.3b.2 — bind the workflow baseline authority` | authored title identity separated from the existing registered capacity paragraph |
| `.3b.3.0` | `CLAIM-VERIFICATION-ADOPTION.3b.3.0 — freeze the mdBook quantitative census design` | prose candidate grammar, exact coverage/authority schema, portable bounds, staged result freeze |
| `.3b.3.1` | `CLAIM-VERIFICATION-ADOPTION.3b.3.1 — implement the mdBook quantitative census` | bounded inventory contract, deterministic producer, exact membership/fence/region validation, 18-case RED matrix |
| `.3b.3.2` | `CLAIM-VERIFICATION-ADOPTION.3b.3.2 — adjudicate mdBook quantitative assertions` | 304 exact semantic regions, honest 75-line incomplete set, verified workflow join, record/array bound repair |
| `.3b.3.3` | `CLAIM-VERIFICATION-ADOPTION.3b.3.3 — freeze mdBook quantitative authorities` | verified mapping claim, digest-complete stale replay, retrievable fact card, explicit incomplete boundary |
| `.3b.4` | `CLAIM-VERIFICATION-ADOPTION.3b.4 — close current-claim repairs` | five exact narrow replacements, zero outer incompletes, 46 non-frontier semantic outcomes preserved |
| `.3c` | `CLAIM-VERIFICATION-ADOPTION.3c — close the current-claim sweep` | clean-boundary replay, all-family RED matrix, mechanically closed candidate denominator |
| `.4` | `CLAIM-VERIFICATION-ADOPTION.4 — prove tracked producers and falsifying controls` | exact known-bad regions, repaired workflow probe, derived six-producer/scratch census |
| `.5` | `CLAIM-VERIFICATION-ADOPTION.5 — close three-leg claim verification adoption` | complete author/reviewer workflow, five-architecture retrieval, public alignment, independent full signoff |
| `.6` | `CLAIM-VERIFICATION-ADOPTION.6 — re-derive the drifted claim-annotated prose counts` | 11 counts re-derived, two per-commit counters withdrawn, mdBook incomplete 75 -> 89; four "confirmed" counts later proved false in this same commit |
| `.6a` | `CLAIM-VERIFICATION-ADOPTION.6a — own four counters .6 confirmed that have drifted again` | leaf opened; withdrawal deliberately deferred until the trajectory was measured |
| `.6a` | `CLAIM-VERIFICATION-ADOPTION.6a — measure the drift trajectory before withdrawing the counters` | 28-consecutive-revision worktree measurement plus the `50775894` anchor; four counters withdrawn with producer fields named, six carried, `.6`'s unit trajectory corrected, `.6b` opened |
| `.6b` | `CLAIM-VERIFICATION-ADOPTION.6b / CHANGES-LEDGER-ROLLOVER.3 — sweep the two surfaces .6 and .6a left` | book chapter and fact card routed to `--report`, card retitled, drift attributed across 31 registry revisions, `.9` second demonstration |

## Changelog

- `2026-08-30`: closed `.7.1a`. **The gate had a blind spot in the one thing it derives itself.** `.7.1`
  shipped a numeral grammar that excluded every `-` and `/` after a numeral — written to keep dates and
  identifiers out, and it does — and that also dropped every count closing a compound adjective or a ratio.
  Measured against an independent tokenizer rather than by reading the regex: **19** published values across
  the four claim-annotated files were invisible, including `TOOLBOX.md`'s live "The 27-case self-test", a
  current count of `check_current_claim_census.pl --self-test` sitting inside a paragraph this gate exists to
  watch. A second defect absorbed sentence commas into values (`40,`), which would have forced records to bind
  punctuation and reported the honest record as unlisted. Both are repaired to the form the identifiers
  actually take: a comma joins a numeral only before exactly three digits, and only `-` followed by a digit is
  excluded. Three new RED cases, all proven grammar-dependent by reverting the single line (14/19 reverted,
  19/19 re-applied). This is why `.7.2` could not have gone first: closing an unlisted report at zero certifies
  nothing when the population is wrong.
- `2026-08-30`: closed `.7.1`. The gate exists and executes. `scripts/check_published_assertions.pl` runs a
  record's named producer and compares its report field to the literal published in the governed region — the leg
  that ten recorded instances went stale for want of, because a digest proves a region has not changed and never
  that its numbers still re-derive. Registered as the twelfth doctrine through the existing driver, so hook and
  CI wiring are unchanged. The four outcomes are closed by construction: a fifth is refused, so there is no
  expressible way to carry a value on a trajectory. Cross-surface disagreement fails before any producer runs;
  a set is compared as an enumeration rather than a size; and `excludes_self` is required the moment an
  enumerator returns the record's own publishing surface. Self-test 16/16 on a disposable repository-local
  fixture, and the drift leg additionally observed RED on **real shipped prose** by revert-and-re-apply rather
  than on a fixture alone. Two implementation findings are recorded rather than smoothed over: the coverage
  grammar first keyed on the `[claim: <id>]` tag's own line, which would have been blind to every value the tag
  actually governs, since authors close a paragraph with it — `TOOLBOX.md`'s two tags sit on lines with no
  quantity at all; and the first real-content drift probe chose a producer that is derive-and-diff over the file
  it was meant to watch, so it exited nonzero before comparing anything. The registry ships in `inventory` phase
  where an unlisted value is reported, not fatal; `.7.2` completes the population and flips it to `frozen`.
- `2026-08-30`: closed `.7.0` and stopped correcting. Asked a third time whether the findings held, `.11a` was
  re-derived and **it does not fully hold either**: its warning-line total was invalidated by its own commit
  (20 published, 21 after that commit's own prepend crossed a band), and its "exactly three surfaces" was a
  classification published without its predicate — five files carried the literal, two of them dated records §1
  exempts. Both withdrawn rather than corrected, because correcting is what has failed. **Five consecutive rounds
  — `.6`/`.6a`/`.6b`, `.10`/`.10a`, `.11`/`.11a` — each invalidated by its own transaction, settle the design
  question that `.7` had left open.** A sixth correction would behave identically, so `.7` is split and `.7.0`
  freezes the contract that ends the class, every element traced to the instance that defeated the alternative:
  execute the producer and compare the field, because a digest proves only that a region has not changed; four
  outcomes with no slot for "a trajectory shows it has held"; a population derived at check time and never
  stored, because three commits in a row moved their own population by describing it; a declared
  `excludes_self`, because `.11`'s classifier was satisfied by the act of writing the finding; cross-surface
  disagreement, which needs no producer at all; membership compared as an enumeration rather than a size; and
  mechanism claims explicitly out of scope with an `adjudicated_against` field instead, since no checker can
  decide whether two accounts predict the same observation.
- `2026-08-30`: closed `.11a`. Asked a second time whether `.11`'s findings held — the question that produced
  `.6b` and `.10a`, and which has now found a real defect all three times — every figure was re-derived instead
  of re-read, and **five did not hold**. Two were mechanism claims taken from co-occurrence or from a commit
  body: only one of three commits actually republished `identity_gated 7`, and `d23e8bae` withdrew
  `current_surfaces` from two publishers rather than three, because the book chapter never carried it. Two were
  set claims with the wrong denominator: the warning census was one checker's output presented as the doctrine's,
  where the gate-level producer emits 35 lines across four producers rather than 18 over 13 surfaces; and the
  ownership screen both counted `done` trees as owners — hiding `corpus_task_evidence_parts` — and turned green
  for the three surfaces it named, because publishing the finding is what made `docs/tasks/` mention them. The
  fifth was a category error: the `Opening Pressure Boundary (92e59c97)` table is a dated snapshot §1 exempts,
  not a competing census. Everything else re-derives unchanged. The corrections are each **stronger** than what
  they replace: `.10` edited `TOOLBOX.md` while adopting the rule against carrying the value; the real unowned
  set was four, not three; and a classifier a finding can satisfy by being written is §2's shared-parent defect,
  which is why `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` now requires an explicit reviewed assignment over the
  gate-level population instead of a grep. Self-catch rate for this adoption: **four**, three of them found only
  because the director asked again, none by any rule, gate, or review step this repository has.
- `2026-08-30`: closed `.11`. Ran the command that enumerates the population three earlier leaves swept from
  memory — every tracked non-archive Markdown surface citing one of the three claim producers — and it returns
  **15**, where `.6b` had written "two other current-facing surfaces". Six stale publications, none of which any
  gate could see. `authority_outcomes.identity_gated` **7 -> 8**, on `TOOLBOX.md`, the mdBook enforcement chapter,
  and the census fact card, attributed by per-revision re-derivation to `d23e8bae` — the commit that registered the
  task-catalog parts surface, noticed that the registration moved `current_surfaces` 39 -> 40, withdrew *that* value
  on all three publishers, and left `identity_gated` carried on the same sentence. The book chapter's `78` incomplete
  assertion-level regions (now 89). The entire `.3b.3.3` book vector still carried in
  `mdbook-quantitative-census-freeze.md`, including its title — the exact numbers `.6b` corrected on the chapter
  while quoting them off it, in a card two directory entries away that nothing looked for. And a set claim whose own
  enumeration was short by one: the doctrine-adoption card's registered-doctrine list, missing `PROOF-SEAL-CURRENCY`
  since `1ccb7331`. Two more of the same class on surfaces the leaf had to touch anyway: `MEMORY.md`'s
  five-surface live-document warning census against a producer that warns about 13, three of them named by no
  tree at all; and `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s own frontier still calling `.2a`/`.2b`/`.2c` pending.
  Three further constants are carried and still *correct* and go with them — `authority_outcomes.derived`,
  the `mdbook-quantitative-census-frozen` assertion's eleven constants, and `control_audit`'s 7/7/6.
  **The remedy is one class, not nine numbers.** `.10` adopted "a repository-derived constant is
  derived or gated, never carried" and retired the trajectory licence these values were carried under, so
  `authority_outcomes.derived` — which had not moved — is withdrawn with `identity_gated`, which had, because
  unmoved is not the same as immovable. Writing `8` for `7` was available and refused: §3 Leg 3 says a right
  unwatched number replacing a wrong one is not a fix. What survives is stated with its reason — `unresolved` **0**
  because `validate_candidate_closure` raises an error the moment it leaves zero, `views` **5** because `.3a.0`
  froze them — so a later reader can tell a gated value from an unmoved one without re-running the trajectory that
  failed here. Recorded and not repaired: nothing checks `DOCTRINE_ENFORCEMENT.md` §10's row set against the
  driver's `DOCTRINES` array, although the driver's own header calls §10 its mirror "kept in lockstep"; enumerated
  rather than asserted, the two agree today at 11 each, so it is an unwatched coupling for `.7` rather than a
  defect. Both stale book counts again sit outside the mdBook census denominator, which is `.9`'s third
  demonstration and its second on the census's own chapter.
- `2026-08-30`: closed `.10a`. The director asked a second time whether `.10`'s findings were trusted — the
  question that produced `.6b` — so they were re-derived rather than re-read, and one number was wrong. The narrow
  absence probe ran **seventeen** terms, not fifteen; fifteen is the discriminating subset and the count the
  widened probe ran. A real quantity substituted for the one the sentence names, which is `.7`'s ninth instance
  committed one commit after that instance was described. A second defect in the same paragraph: "visible to no
  gate this repository has" is a set claim published without its enumeration, the rule `.10` had just made
  normative. Both corrected on every publishing surface; the corrected probe account is strictly better because it
  explains why the two probes differ, which the single number hid. Findings two and three re-derive unchanged, with
  every rollover figure taken from the producer rather than from arithmetic. The self-catch rate is therefore
  **three** in this adoption, two of them set claims, and the third was caught only because the director asked
  again — not by any rule, gate, or review step this repository has.
- `2026-08-30`: closed `.10`. Re-read the upstream standard section by section, which directive 17 requires and
  which nothing had done since `.1`. The source had not moved, so the gap was original to the adoption — and it is
  wider than `.10` predicted: the leaf named the three rules that had already produced a recorded defect here,
  while the reading found a larger set with no home on any governed claim surface, now enumerated in
  `CLAIM_VERIFICATION.md` §11 with each rule's local home. Both statements hold under different denominators, so
  the leaf's own text stands. Adopted: the taxonomy of what each check class still permits and its general form, the
  illustration rule, the set-enumeration rule in both directions, the project-history oracle, deriving classifiers
  from the producer, revert-and-re-apply attribution, granularity matching, one derived source over N copies,
  derived-or-gated-never-carried, the unwatched-number rule, and the domain-free statement rule. Recorded four
  deliberate refusals with reasons. One rule was already here but demoted to ADR 0042 rationale while the standard
  carried only its instances; promoted, with the ADR keeping the sentence as reasoning. The first absence probe was
  scoped to one file and would have published a claim about the repository on evidence about that file — the
  granularity rule being adopted in the same commit — so it was widened before anything was published, which is
  what found the ADR hit. `.7`'s open scope question is answered and it **narrows**: a claimed mechanism is a
  review obligation under the now-normative illustration rule, because no checker can decide whether two accounts
  predict the same observation; the gate stays on counts, keeping the cross-surface disagreement signal and an
  optional prior-adjudication field as its mechanizable residues.
- `2026-08-30`: `.10` performed the mandatory `DEVELOPMENT_NOTES.md` rollover in its own slice. The rationale
  record crossed the 90% line milestone, and this tree's `.1` had already adjudicated the same shape by rolling
  the same ledger inside its own slice; `2b9e8899` did the equivalent for `CHANGES.md`. Segment
  `development-notes-0009` seals 22 whole records; the live root returns to 62 records / 1,384 lines, under the
  80% warning on every dimension. The dry-run was proven exact before applying and reported `future_prepends: 1`,
  which is how the in-flight record was shown to survive the cut rather than assumed to.
- `2026-08-29`: opened `.6a`. Re-derived `TOOLBOX.md`'s census sentences against their producers at
  `c1609558` and found four of the eight counts `.6` recorded as "confirmed unchanged" no longer hold
  (`6 registered` -> 5; closure `86/51/35` -> `72/50/22`); the other four and the provenance sentence are
  current. `1507adbf`, which touched only the resume pointer and two claim registries, then moved all four
  again to 4 and `69/49/20`. The leaf is scoped to measure the trajectory before withdrawing anything, so
  "per-commit counter" is proved the way `.6` and `LIVE-DOCUMENT-PRESSURE-HEADROOM.5` proved theirs rather
  than inferred from two endpoints.
- `2026-08-29`: closed `.6a`. The trajectory was measured over the 28 consecutive revisions since `.6`, plus
  `.6`'s own older `50775894` anchor, in a detached worktree, each with its own checker, and it answers the question the leaf opened with: `6 registered` became 5 **inside
  `5fe81128`**, alongside the closure triple, so nothing in `.6`'s "confirmed unchanged" list was true when
  `.6` wrote it. All four are withdrawn from `TOOLBOX.md` in favour of `--report` with their producer fields
  named; the six values that did hold at every measurement are carried. `.6`'s own unit trajectory and its
  stated growth rule are corrected as a fifth number in the same section. Opened `.6b` for the mdBook doctrine
  chapter and the census fact card, which publish the same counts and were outside `.6`/`.6a`'s stated sweep
  boundary. Added `[[worktree-doctrine-measurement-gitlink]]`, because the rig fails closed and silently until
  the `subs/fsmgen` gitlink is populated in the worktree.
- `2026-08-29`: closed `.6b`, which finishes the sweep `.6` started and `.6a` bounded. The mdBook doctrine
  chapter and the `current-claim-census-freeze` fact card both stop publishing census totals and route to
  `--report`; the card is retitled, because its old title published `56 exact authority units` as current.
  Attribution came free: the mdBook census's frozen contract is a tracked registry, so counting outcomes in
  each of its 31 revisions gives the exact trajectory with no worktree at all — a cheaper instrument than
  `.6a`'s, and worth reaching for first when the thing being measured is itself a tracked artifact. Both stale
  sentences were correct on the day they landed and wrong afterwards. `.9` gains its second and
  sharper demonstration: the chapter documenting the census held five candidate lines in 557, and none of them
  was a line the census got wrong.
- `2026-08-28`: `.7` gained its fourth instance and its repair, and `.8` was opened. `STATUS-LEDGER-ROLLOVER.4a`
  found `current-claim-census-frozen`'s own assertion stale and attributed it exactly: `.6` measured 86/51/35
  before applying its own `CHANGES.md` rollover, and that rollover moved 14 claim-annotated regions into an
  archive segment that is not a current surface. The volatile counters are withdrawn from the assertion. The
  same review measured that the census registry grows one record per ledger-prepending slice against a
  declared 128-record bound, which `.8` now owns.
- `2026-08-15`: Created from the owner's explicit adoption directive after confirming that SpecForge has no
  local claim-verification standard, claim registry/checker, or dedicated task tree.
- `2026-08-15`: `.0` maps the source checklist onto existing exact-currentness and mutation infrastructure,
  freezes scope, and identifies decision-record capacity as the required clean-boundary prerequisite to `.1`.
- `2026-08-15`: `.1` publishes the local standard and ADR 0042, requires one exact claim/no-claim declaration in
  commit and PR review, closes bootstrap/README/workflow-catalog discovery, and leaves mechanical provenance
  honestly pending for `.2`; `.1a` owns the newly visible 14/16 workflow-standard capacity warning first.
- `2026-08-15`: `.1`'s required engineering rationale crosses the development-note line rollover; the exact
  boundary-authenticated plan seals 15 whole records as segment 0008 and retains the current record in a warning-
  safe live root without editing any older archive member.
- `2026-08-15`: `.1a` re-derives 21 workflow slots from 14 current members and a four-member peak, preserves the
  stable explicit topology, and leaves exact authority retirement to `.1b` after commit.
- `2026-08-15`: `.1b` removes the consumed 16→21 authority while leaving the committed workflow profile and all
  of its current evidence unchanged; `.2` is executable.
- `2026-08-15`: `.2` activates ADR 0044's bounded executable evidence join, migrates three verified current
  claims, and registers the tenth doctrine without adding another hook or CI path; `.3` owns the constant sweep.
- `2026-08-15`: `.3a.0` derives the exact 39-surface current denominator and freezes the bounded five-view
  evidence schema before `.3a.1` implements any producer or `.3b` changes any source assertion.
- `2026-08-15`: `.3a.1` implements the self-bounded inventory contract and deterministic producer, proves its
  missing/unknown/duplicate/untracked/stale/bound controls, and leaves every finding unfrozen for `.3a.2`.
- `2026-08-15`: `.3a.2` repairs a first-view-only candidate-key defect before freezing 51 exact evidence units;
  five incomplete keys become the only legal `.3b` repair frontier and all source assertions remain unchanged.
- `2026-08-15`: `.3b.0` decomposes repair by authority: three maintained-reference routes, the authored-policy /
  registered-capacity split, exact mdBook quantitative regions, then one atomic result re-freeze.
- `2026-08-15`: `.3b.1` proves existing README, fact-card, and canonical-catalog route authorities plus their RED
  controls; no new checker or premature frozen-result rewrite is needed.
- `2026-08-15`: `.3b.2` reuses `workflow-standard-capacity-profile` for the exact actionable paragraph and keeps
  authored workflow identity outside measurement scope; no hand-carried copy or duplicate claim is added.
- `2026-08-15`: `.3b.3.0` freezes a prose-only quantitative candidate grammar and exact-region authority schema,
  separating inventory, adjudication, and result freeze before the outer census can consume it.
- `2026-08-15`: `.3b.3.1` implements the self-bounded inventory and exact-region validator, corrects the
  unreproducible design estimate from 301 to 304 stable candidates, and leaves semantic adjudication to
  `.3b.3.2` without changing the outer result.
- `2026-08-15`: `.3b.3.2` adjudicates every exact candidate, binds eight workflow-capacity lines to their verified
  claim, exposes 75 actionable lines with all three evidence legs missing, and excludes 221 exact authored,
  example/identity, or dated observations; frozen replay also repairs the record/array bound dimension defect.
- `2026-08-15`: `.3b.3.3` registers and routes the exact census-mapping authority with executable replay,
  controlled falsification, digest-complete staleness, and a Knowledge Map fact card; the verified claim preserves
  rather than certifies the 75 incomplete underlying assertions, and `.3b.3` closes.
- `2026-08-15`: `.3b.4` atomically replaces the five broad incomplete title anchors with narrow identity plus
  route/registered authorities, re-freezes 56 units with zero outer incompletes, proves all other 46 semantic
  outcomes unchanged, and closes `.3b` without promoting the 75 inner mdBook gaps.
- `2026-08-15`: `.3c` adds the missing reverse candidate join, expands controls from 15 derived-heavy cases to a
  27-case matrix spanning every outcome family and coverage boundary, proves the final lockstep boundary closes
  79 current candidates as 51 exact + 28 registered + zero unresolved, and independently closes `.3` from clean
  commit `50775894`.
- `2026-08-15`: `.4` binds all seven cited controls to exact known-bad producer regions, repairs the independent
  workflow feasibility probe with a controlled sub-ceiling RED path, derives six governed producers with zero
  ignored/untracked candidates, and expands claim-gate controls from 22 to 27 before handing signoff to `.5`.
- `2026-08-16`: `.5` publishes the runnable classification/author/auditor workflow and five-architecture join,
  repairs the stale public priority and exact production-graph test oracle, independently closes both claim
  censuses, runs selected full CI, and closes the adoption tree with `SPEC-TO-INTENT-ALIGNMENT.7` next.
