# CLAIM-VERIFICATION-ADOPTION — program foundation

- Part ID: `program-foundation`
- State: `legacy`

<!-- claim-verification-task-source-region:program-identity:start -->
# CLAIM-VERIFICATION-ADOPTION: adopt three-leg verification for published claims

## Metadata

- Tree ID: `CLAIM-VERIFICATION-ADOPTION`
- Status: `active` (`.0`–`.6`, `.6a`, `.6b`, `.10`, `.10a`, `.11`, `.11a`, `.7.0`, `.7.1`, `.7.1a`, `.7.2.0`, `.7.2.1`, `.7.2.1a` done — the surfaces publishing the census counts
  are now swept against an *enumerated* population rather than a remembered one, and the upstream standard is
  re-adopted with its refusals recorded; `.7` owns the gate that would have observed the drift, now scoped to
  counts; `.7.3` binds every published self-test case count to the script that declares it after three went
  stale at once; `.8` tracks the census registry's own capacity; `.9` CLOSED the candidate vocabulary's
  blind spot — four demonstrated nouns, up to two intervening words, a trailing-boundary precision fix,
  458/458 adjudicated, and a grammar control that goes RED when the gap is reverted)
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

<!-- claim-verification-task-source-region:program-identity:end -->

<!-- claim-verification-task-source-region:adoption-program-nodes:start -->
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

<!-- claim-verification-task-source-region:adoption-program-nodes:end -->
