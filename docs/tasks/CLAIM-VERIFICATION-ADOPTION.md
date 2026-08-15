# CLAIM-VERIFICATION-ADOPTION: adopt three-leg verification for published claims

## Metadata

- Tree ID: `CLAIM-VERIFICATION-ADOPTION`
- Status: `active`
- Roadmap lane: process / continuity / signoff evidence (cross-cutting)
- Created: `2026-08-15`
- Last updated: `2026-08-15`
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
  Status: `active`
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
  Status: `pending`
  Goal: document, independently audit, and close the fifth portable architecture
  Acceptance: the mdBook teaches claim classification, authoring, review, missing-leg reporting, stochastic
  intervals, and the auditor asymmetry with runnable SpecForge examples; Knowledge Map, task/public status,
  architecture analysis, and resume pointer agree; an independent closure audit finds no silent in-scope claim;
  selected full CI passes and project-owned verification artifacts are cleaned or retained by policy
  Verification: mdBook test/build, catalogs, full CI, final doctrine and locality gates
  Commit: `CLAIM-VERIFICATION-ADOPTION.5 — close three-leg claim verification adoption`

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
| 19 | `CLAIM-VERIFICATION-ADOPTION.5` | `pending` | documentation and independent signoff close the implemented system |

## Decisions

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

- None. `.4` is closed; `.5` owns independent documentation/signoff and selected full CI next.

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `CLAIM-VERIFICATION-ADOPTION.0 — own and map three-leg claim verification` | standard/local seam audit; implementation remains pending |
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

## Changelog

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
