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
| 2 | `CLAIM-VERIFICATION-ADOPTION.1` | `done` | standard, ADR, discovery, authoring, and review contract are published |
| 3 | `CLAIM-VERIFICATION-ADOPTION.1a` | `done` | measured 21-file profile restores headroom without changing explicit topology |
| 4 | `CLAIM-VERIFICATION-ADOPTION.1b` | `done` | exact transaction authority retired; committed profile remains unchanged |
| 5 | `CLAIM-VERIFICATION-ADOPTION.2` | `pending` | next: implement the bounded claim registry and unconditional semantic gate |
| 6 | `CLAIM-VERIFICATION-ADOPTION.3` | `pending` | the constant sweep needs the registered classification/gate shape |
| 7 | `CLAIM-VERIFICATION-ADOPTION.4` | `pending` | producer and RED-control closure needs the governed claim census |
| 8 | `CLAIM-VERIFICATION-ADOPTION.5` | `pending` | documentation and independent signoff close the implemented system |

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
  bootstrap and README, with ADR 0042 rationale and an exact claim/no-claim declaration; its mechanical provenance
  registry is still pending rather than implied.
- **Status:** `verified` for the publication/discovery assertion; mechanical registry enforcement is explicitly
  outside the assertion and remains `.2` work.
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
- **DURABILITY:** the producer, surface registry, exact increase authority, ADR 0043, and catalog checker are
  tracked. Generic live-size count warning is the stale-state trigger for a future re-measurement; `.1b` owns
  retirement of the transaction-only authority.

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
- `.1a` must decide whether measured workflow-standard growth warrants a re-derived flat profile or a bounded
  routed topology; ADR 0043 resolves this in favor of the minimal 21-file flat explicit profile.

## Blockers

- None. `.2` is the next executable leaf.

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `CLAIM-VERIFICATION-ADOPTION.0 — own and map three-leg claim verification` | standard/local seam audit; implementation remains pending |
| `.1` | `CLAIM-VERIFICATION-ADOPTION.1 — publish the claim-verification contract` | normative scope, ADR 0042, discovery, authoring/review contract, and mdBook alignment |
| `.1a` | `CLAIM-VERIFICATION-ADOPTION.1a — re-derive workflow-standard capacity` | tracked measurement, ADR 0043, exact authority, stable explicit topology |
| `.1b` | `CLAIM-VERIFICATION-ADOPTION.1b — retire the consumed workflow-capacity authority` | one-use authority removed after the 21-file profile became baseline |

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
