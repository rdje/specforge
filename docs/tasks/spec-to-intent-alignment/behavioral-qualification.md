# SPEC-TO-INTENT-ALIGNMENT — behavioral qualification

- Part ID: `behavioral-qualification`
- State: `active`

## Active behavioral qualification

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`
  State: `in_progress`
  Goal: qualify genericity behaviorally and replay the reviewed population after remediation
  Acceptance: alpha-renaming, structure-preserving paraphrase, adversarial identity, negative-control, and
  held-out-document tests prove decisions are not coupled to names; the complete reviewed replay and full gates
  publish all intended deltas without truthfulness/provenance regression; public/live/book/retrieval truth is
  synchronized
  Verification: `activation decomposes the complete behavioral signoff into bounded oracle, transformation, held-out, population-replay, and closure children; 66 root owners/routes close; 44/44 active-task and 84/84 live-size cases, trajectory owner lookup, 220-card catalog, 236-fact / 1,817-key Knowledge Map, mdBook test/build, doctrines, and cleanup pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f — activate behavioral qualification`
  Children: `.f.i`, `.f.ii`, `.f.iii`, `.f.iv`, `.f.v`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.i`
  State: `done`
  Goal: freeze the behavioral oracle, governed population, transform relations, failure taxonomy, and evidence
  Acceptance: an accepted, executable design names every current measurable document and held-out stratum;
  defines symbol, identity, paraphrase, negative-control, and unchanged-source relations at every proof-bearing
  stage; separates permissible renamed presentation from semantic/proof-topology deltas; fails closed on missing
  source authority, transform ambiguity, unsupported provider dependence, or incomplete comparison coverage
  Verification: `24/24 current rows join exact retained/source/proof authority; 23 text-measurable and one
  vacuous/unmeasurable; 7 reviewed calibration / 17 prospective holdout, including 4 vendor-novel and 13
  family-novel; five-stage relational contract and 8/8 mutation controls pass; wrapper, doctrines, mdBook,
  retrieval, and cleanup pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.i — freeze behavioral qualification contract`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii`
  State: `in_progress`
  Goal: implement deterministic whole-document metamorphic generation and comparison
  Acceptance: a conformance-owned repository-local harness generates reproducible variants, runs the current
  production pipeline without granting named knowledge to core, normalizes only declared transform effects,
  emits machine-readable evidence, and rejects partial, escaped, stale, or ambiguous runs
  Verification: `pending`
  Commit: `completed by .f.ii.a through .f.ii.c child commits`
  Children: `.f.ii.a`, `.f.ii.b`, `.f.ii.c`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.a`
  State: `done`
  Goal: qualify alpha-renaming plus adversarial document and symbol identity perturbation
  Acceptance: source-bound identifiers, document key, filename, title, ordering, and misleading familiar names
  vary under a deterministic bijection; normalized canonical decisions and complete proof topology remain
  invariant, copied symbols rename consistently, and injected identity-coupled faults fail
  Verification: `conformance-owned harness runs isolated SourceIR, EvidenceIR, SemanticIR, IntentIR, and ISF
  adapter pairs; deterministic source-bound alpha bijection varies lexical order with familiar aliases; exact
  PDF copies vary path/display identity; closed normalization covers declared spellings, identity/path, keyed
  collection order, relation-bound digest/scope, and derived ids while a semantic-role fault and incomplete stage
  set reject; 8 focused tests pass, unchanged and adversarial provider-backed full-PDF tests pass explicitly,
  warning-denied Clippy and Rustdoc, all nine doctrines, 1,961/8/0 Rust tests, five compile-fail doctests,
  mdBook test/build, and final locality pass;
  exact 78-module inventory, 77-reachable/one-test-support graph, and 140-row flow boundary pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.a — implement identity and alpha qualification`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.b`
  State: `pending`
  Goal: qualify structure-preserving paraphrase and harmless layout perturbation
  Acceptance: reviewed intent-equivalent sentence, heading, table, and formatting variants preserve supported
  conclusions and residual accounting under declared relations; meaning-changing controls remain distinguishable
  and unsupported paraphrases fail closed instead of being silently labeled equivalent
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.c`
  State: `pending`
  Goal: qualify semantic negative controls and the behavioral gate's own sensitivity
  Acceptance: controlled omissions, contradictions, relation reversals, value/timing changes, undeclared symbols,
  misleading names, proof corruption, and disabled-stage faults produce the required changed fact or residual and
  make an invariant-only or incomplete comparison fail
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`
  State: `pending`
  Goal: qualify held-out vendors, layouts, categories, and protocol families without calibration leakage
  Acceptance: the frozen split is identity-disjoint and category-representative where evidence permits; results
  publish denominators and uncertainty; absence of adequate retained source is unmeasurable rather than passed;
  no held-out label, threshold, or result can steer canonical extraction
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv`
  State: `pending`
  Goal: replay and reconcile the complete reviewed population under the behavioral oracle
  Acceptance: every current measurable chain and every explicitly unmeasurable legacy chain is accounted for;
  all metamorphic and negative runs are attributed; exact canonical, residual, proof, validation, and lowering
  deltas are published; no accepted transform loses truthfulness, provenance, or currentness
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v`
  State: `pending`
  Goal: close behavioral genericity and the `.6d.ii` parent without overstating the evidence
  Acceptance: structural and behavioral gates compose unconditionally; complete results, limitations, cleanup,
  current status, task parents, controller, mdBook, retrieval facts, and full CI agree; `.6e` becomes eligible only
  if the production core is genuinely specification-instance-neutral
  Verification: `pending`
  Commit: `pending`

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.a`

- [x] **REPRODUCE / MEASURE** — the frozen 24-row contract requires unchanged-source, adversarial-identity,
  and symbol-alpha comparison across SourceIR, EvidenceIR, SemanticIR, IntentIR, and ISF adapter; 23 rows are
  text-measurable and one is explicitly text-unmeasurable.
- [x] **ROOT CAUSE (WHY + WHERE)** — at parent commit `5ea18fc`,
  `crates/specforge-conformance/src/lib.rs:1` exported no whole-document behavioral executor, so the accepted
  relations in `doctrine/production_genericity/behavioral_qualification.json` had no implementation or evidence.
- [x] **ADDRESSED (verified)** — `cargo test --offline -p specforge-conformance behavioral_genericity` passes
  8/8 provider-free controls with two provider tests intentionally ignored; an explicit serial ignored-test run
  passes both unchanged and adversarial full-PDF cases, and all three relations compare exactly five stages.
- [x] **NO REGRESSION** — `scripts/run_ci.sh` passes all nine doctrines, all 11 production-genericity components,
  1,961 Rust tests with eight intentional ignores and zero failures, five compile-fail doctests, warning-denied
  Clippy/Rustdoc, mdBook test/build, chain currency, and final project-data locality.
- [x] **GENERICITY (ADR 0006)** — transformation/comparison stays conformance-only; production sees only current
  input, the 78-module dependency graph has no core-to-conformance edge, and all 168 runtime field rules retain
  their structural alpha obligations without a document, vendor, protocol, or expected-outcome branch.
- [x] **LOCKSTEP** — the task frontier, behavioral contract, module/flow census, engineering rationale, mdBook,
  research audit, Knowledge Map fact/projections, live-document contracts, and resume pointer publish `.f.ii.b`
  as the next leaf and the same bounded identity/alpha qualification.

## Current Frontier

| Order | Leaf | State | Why next |
| --- | --- | --- | --- |
| 1 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.i` | `done` | oracle, population, relations, held-out split, and evidence states are frozen |
| 2 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.a` | `done` | deterministic alpha and full-PDF adversarial identity pairs pass the closed five-stage comparator |
| 3 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.b` | `pending` | next: add reviewed paraphrase and harmless layout relations |
| 4 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.c` | `pending` | prove the gate detects meaning and authority changes |
| 5 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii` | `pending` | measure identity-disjoint held-out generalization honestly |
| 6 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iv` | `pending` | reconcile the complete reviewed population and all variants |
| 7 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v` | `pending` | close behavioral signoff and release the controller-ranked frontier |

## Decisions

- `2026-08-14`: behavioral signoff is not one boolean replay. Freeze the oracle first, then land symbol/identity,
  paraphrase/layout, negative-sensitivity, held-out, population-reconciliation, and final closure slices.
- `2026-08-14`: transformation and comparison authority belongs to conformance. Production core receives only
  transformed current-document inputs and cannot observe named fixtures, expected outcomes, or held-out labels.
- `2026-08-14`: full-PDF unchanged/identity relations qualify rich capture; normalized-text relations qualify
  only downstream text behavior and must report page/visual/table/geometry surfaces as unmeasurable.
- `2026-08-14`: the seven reviewed/current overlaps are historically exposed calibration. The other 17 current
  rows are the prospective holdout; the 12 reviewed rows remain evaluation authority, not relabeled unseen data.
- `2026-08-14`: alpha aliases use unique, familiar engineering components and reverse source-symbol lexical
  order. The comparator realigns only keyed/set-valued schema collections and the ISF interface declaration set;
  ordered unkeyed arrays remain exact.
- `2026-08-14`: relation-bound ids are normalized only when their enclosing records are otherwise equivalent
  after declared symbol/path/digest normalization. Positional arbitrary-id pairing is forbidden because it could
  hide an identity-coupled semantic change.
- `2026-08-14`: isolated unchanged-PDF roots necessarily change verified proof scopes and conclusion/input
  digests because scratch paths are proof-context inputs. The oracle normalizes those cryptographic consequences
  while keeping ruleset/prior/validation identities, proof topology, and every non-digest value exact.

## Open Questions

- `.f.ii.b` must introduce only reviewed, exhaustively declared paraphrase/layout recipes; the implemented
  identity/alpha normalizer is not authority to label an arbitrary textual rewrite equivalent.

## Blockers

- None for `.f.i`. Missing retained source or provider-free transform support becomes explicit unmeasurable
  evidence, not permission to weaken the oracle.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-14` | `.f` activation | 66 owners/routes; 44/44 active-task and 84/84 live-size cases; trajectory owner lookup; 220 cards; 236 facts / 1,817 keys; mdBook and cleanup pass |
| `2026-08-14` | `.f.i` population census | 24 PDF authorities; 1,822 pages / 1,995 visuals / 906 tables / 22,088 elements / 3,899 sections; text replay yields 27,330 statements / 6,313 semantic / 9,543 intent records, with 23 non-vacuous rows |
| `2026-08-14` | `.f.i` executable contract | exact 24 retained rows and five stages join; 7 calibration / 17 prospective, 4 vendor-novel / 13 family-novel; baseline checker and 8/8 controlled faults pass |
| `2026-08-14` | `.f.ii.a` identity/alpha harness | eight provider-free focused tests plus explicit unchanged and adversarial Docling-backed PDF tests pass; all three implemented relations execute and compare five persisted proof-bearing stages; identity-coupled role, partial-stage, and symlink-escape controls reject |
| `2026-08-14` | `.f.ii.a` full repository gate | all 9 doctrines and 11 production-genericity components pass; Rust 1,961 passed / 8 ignored / 0 failed plus 5 compile-fail doctests; Clippy, Rustdoc, mdBook, and final locality pass |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.f` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f — activate behavioral qualification` | activate the parent and route `.f.i` as the precise design frontier |
| `.f.i` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.i — freeze behavioral qualification contract` | freeze source planes, current and held-out denominators, six relations, five-stage comparison, and closed evidence states |
| `.f.ii.a` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.a — implement identity and alpha qualification` | add deterministic source-bound alpha and byte-identical PDF identity generation, five-stage replay, closed comparison, evidence, and sensitivity controls |

## Activation protocol

Every child updates this part and the bounded root together. Route or measured-metric changes update the
contract, index, and manifest in the same commit. Legacy payloads and the exact source capsule remain unchanged.
