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
  Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`

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
  Status: `pending`
  Goal: publish the local standard, architecture decision, discovery routes, and authoring contract
  Acceptance: root standard and decision record establish the three legs and scope; bootstrap, README route,
  COMMIT/TOOLBOX guidance, and the repository review template require a claim tag or explicit no-claim marker;
  current documents use repository-root-relative paths and do not copy external absolute paths
  Verification: focused route/currentness/catalog checks, mdBook no-drift review, and mandatory doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.1 — publish the claim-verification contract`

- ID: `CLAIM-VERIFICATION-ADOPTION.2`
  Status: `pending`
  Goal: mechanize bounded claim provenance and register it in the doctrine driver
  Acceptance: a self-bounded registry/checker validates governed claim ids, source commands, falsification
  evidence, durability state, complete artifact-input identity, tracked producers, and stale-state behavior;
  positive and controlled negative fixtures prove missing/unknown/duplicate/stale/untracked cases fail clearly;
  `scripts/check_doctrines.sh` registers the check without duplicating hook/CI wiring
  Verification: checker self-test plus clean/controlled-failure cases and mandatory doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.2 — gate published claim provenance`

- ID: `CLAIM-VERIFICATION-ADOPTION.3`
  Status: `pending`
  Goal: sweep current-facing published constants and remove silent carried-state claims
  Acceptance: a bounded census covers current status, roadmap/controller projections, maintained references,
  doctrine baselines, and mdBook quantitative claims; every in-scope claim is derived, identity-gated, registered
  with three legs, or explicitly marked incomplete; each repair preserves one canonical authority and avoids
  synchronizing hand-carried copies
  Verification: deterministic census, focused producer/gate checks, current-truth/book checks, and doctrines
  Commit: `CLAIM-VERIFICATION-ADOPTION.3 — make current published constants derived or watched`

- ID: `CLAIM-VERIFICATION-ADOPTION.4`
  Status: `pending`
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
| 2 | `CLAIM-VERIFICATION-ADOPTION.1` | `pending` | first complete the existing decision-capacity remedy, then publish the normative contract |
| 3 | `CLAIM-VERIFICATION-ADOPTION.2` | `pending` | enforcement depends on the frozen local contract |
| 4 | `CLAIM-VERIFICATION-ADOPTION.3` | `pending` | the constant sweep needs the registered classification/gate shape |
| 5 | `CLAIM-VERIFICATION-ADOPTION.4` | `pending` | producer and RED-control closure needs the governed claim census |
| 6 | `CLAIM-VERIFICATION-ADOPTION.5` | `pending` | documentation and independent signoff close the implemented system |

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
- `2026-08-15`: no pull-request or review template exists. `.1` will add the repository-standard GitHub pull-
  request template and align `COMMIT.md` / `TOOLBOX.md`; authors must state either governed claim tags or that the
  slice publishes no actionable claim.

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

The decision collection is already 42/44 Markdown files and independently has one accepted record at 97.1% of
its byte ceiling. Adding the required claim-verification ADR now would consume scarce capacity contrary to the
already-active `DECISION-RECORD-CAPACITY-HEADROOM.1/.2` remedy. After `.0` commits cleanly, PNT must complete that
prerequisite before this tree begins `.1`; this is sequencing, not ambiguity in the claim architecture.

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

- `.2` must select the exact claim identity/input-digest schema after `.1` freezes the local normative vocabulary;
  the dedicated bounded-registry topology and required semantic fields are no longer open.

## Blockers

- `.1` must not add its required layer-C decision while the decision collection is at 42/44 files with no
  repeatable capacity remedy. Complete `DECISION-RECORD-CAPACITY-HEADROOM.1/.2` from a clean pivot, then resume.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-15` | `.0` | full source-standard read; claim/review/derived-state/doctrine/control/tracked-
  producer/capacity census; task catalog; derived-state report; canonical catalogs; live-size self-tests |
  `passed`; mapped 39 current surfaces, 14 derived-state contracts, 33/24 tracked/self-test checkers, zero
  governed ignored producers, and the 42/44 decision prerequisite without claiming adoption |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `CLAIM-VERIFICATION-ADOPTION.0 — own and map three-leg claim verification` | standard/local seam audit; implementation remains pending |

## Changelog

- `2026-08-15`: Created from the owner's explicit adoption directive after confirming that SpecForge has no
  local claim-verification standard, claim registry/checker, or dedicated task tree.
- `2026-08-15`: `.0` maps the source checklist onto existing exact-currentness and mutation infrastructure,
  freezes scope, and identifies decision-record capacity as the required clean-boundary prerequisite to `.1`.
