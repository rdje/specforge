# SPEC-TO-INTENT-ALIGNMENT — genericity remediation

- Part ID: `genericity-remediation`
- State: `legacy`

<!-- spec-to-intent-task-source-region:genericity-nodes-06d-ii-06d-ii-d-iv:start -->
- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii`
  Status: `in_progress`
  Goal: `replay the decibel-domain disposition repair across the complete reviewed population, publish the exact comparable result, and make production genericity a repository-wide release gate`
  Acceptance: `all 12 sources and 48 stages replay at the committed .6d.i production revision; the exact reviewed disposition and every collateral change are published; all 24 prior true positives and 29/29 provenance survive; every production Rust surface is specification, PDF-identity, vendor, protocol, signal, and corpus-vocabulary neutral; a registered fail-closed doctrine distinguishes cfg(test) code and test support from production and rejects a controlled production leak; controller, public/live documentation, retrieval, and cleanup evidence agree`
  Verification: `pending`
  Commit: `completed by .6d.ii.a through .6d.ii.f child commits`
  Children: `.6d.ii.a`, `.6d.ii.b`, `.6d.ii.c`, `.6d.ii.d`, `.6d.ii.e`, `.6d.ii.f`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a`
  Status: `done`
  Goal: `publish the clean population replay, audit the complete current extraction pipeline, and remove reviewed-corpus snapshot composition from the production module graph`
  Acceptance: `the exact 12-source / 48-stage replay is durable; every production stage, source group, optional model path, prior/evaluation path, and adapter boundary has a neutrality verdict with decision sites named for each blocking family; the disproven historical genericity claim is corrected; corpus-specific replay assertions and composition are test-only; the audit defines the positive structural proof required instead of treating a finite vocabulary list as proof`
  Verification: `exact replay/result/controller/currentness tests 7 passed / one explicit writer ignored; capability-registry currentness passed; mdBook test/build and Knowledge Map passed; all seven commit-tier doctrines passed; full run_ci passed all eight doctrines, formatting, warning-deny Clippy, 1,870 tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook, and final project-data locality; audit scratch and replay workspace/runtime map absent`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a — audit production genericity`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`
  Status: `done`
  Goal: `make SourceIR PDF normalization and visual/table classification structurally generic`
  Acceptance: `production ingest contains no corpus-calibrated operation, protocol-role, packet-field, document, vendor, or signal spellings; classifications depend on typed page/table/diagram structure or document-derived evidence and retain an honest unknown path; exhaustive ADR 0025 replay names every affected retained chain and all drifted EvidenceIR/downstream artifacts are reconciled with exact delta attribution before commit`
  Verification: `55 focused SourceIR tests twice; exact 78-document classifier census; ten-chain ADR 0025 backup/rebuild/delta/validation; FSMGen strict on both renderable ISFs; exact cleanup; full CI passes all eight doctrines, formatting, warning-deny Clippy, 1,873 tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook, and final locality; chain currency 24/24 measurable EvidenceIR and 78/78 SemanticIR/IntentIR/adapters`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b — make SourceIR classification neutral`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c`
  Status: `done`
  Goal: `replace protocol-specific EvidenceIR schema and extractors with document-derived generic protocol semantics`
  Acceptance: `production EvidenceIR exposes only generic frame, operation, state, actor, and direction records; exact protocol/signal/response names and named extractor identities live only in tests or input data; downstream stages carry the neutral schema losslessly`
  Verification: `EvidenceIR schema 2 uses document-derived protocol operations, phase names, and participant-drive actors; schema 1 fails closed on named protocol carriers and future schemas reject; 42 focused extraction tests and 1,876 full tests pass with six ignored and zero failed; all 78 persisted EvidenceIRs are schema 2, chain currency is exact for 24/24 replayable EvidenceIR and 78/78 SemanticIR/IntentIR/adapters, retained source re-extraction accounts exactly for all 24 content deltas, and full CI passes all eight doctrines, formatting, warning-deny Clippy/rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c — make protocol EvidenceIR document-derived`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d`
  Status: `done`
  Goal: `remove identity- and spelling-driven prior, semantic, prompt, validation, and corpus-command decisions`
  Acceptance: `document key/display name and named family enums cannot steer extraction; signal spelling cannot infer handshake, clock, reset, or interface authority; prompts use neutral synthetic placeholders; production corpus organization is structural and data-derived`
  Verification: `three bounded child repairs plus .d.iv combined qualification; exact 89d8dee7..9c38b569 range census; 13/19/14/14/2/1/1 focused alpha/identity/prompt/spelling/fixture-name/empty-catalog/undeclared-contract controls; current-binary chain currency 24/24 measurable EvidenceIR plus 78/78 downstream; all 17 renderable ISFs FSMGen-strict clean; full CI; .e/.f remain explicit release blockers`
  Commit: `completed by .6d.ii.d.i through .6d.ii.d.iv child commits`
  Children: `.6d.ii.d.i`, `.6d.ii.d.ii`, `.6d.ii.d.iii`, `.6d.ii.d.iv`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i`
  Status: `done`
  Goal: `remove document-identity and named-family authority from prior memory`
  Acceptance: `prior production types and lookup paths contain no vendor/protocol family enum; learning groups only identity-independent normalized evidence or structural fingerprints; old named-family memory cannot steer current extraction; exact affected chains are reconciled under ADR 0025`
  Verification: `schema-7 scope/legacy/future tests 97/97; learn-priors tests 16/16; KG unit tests 14/14 and tracked fixtures 156/156; the 13-input store relearns byte-identically at a416cc8b…6239633 with one explicit contest; ADR 0025 currency is exact at EvidenceIR 24/24 measurable and 78/78 SemanticIR, IntentIR, and adapters; 58 changed IR artifacts and two adapters validate, two renderable ISFs pass pinned FSMGen strict, scratch is absent, and full run_ci passes all eight doctrines, warning-deny 1,880/6/0 Rust tests, rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i — make prior memory identity-independent`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii`
  Status: `done`
  Goal: `remove identifier-spelling authority from EvidenceIR through SemanticIR, IntentIR validation, and adapter lowering`
  Acceptance: `renaming a signal without changing its declarations/evidence cannot change signal admission, interface admission, handshake role, clock/reset role, validation outcome, or adapter lowering; unresolved semantics stay explicit and no downstream stage re-infers them from the identifier`
  Verification: `case/length/suffix/collision/empty-catalog and alpha-renaming controls across EvidenceIR, optional NLP, SemanticIR, validation, IntentIR, and ISF lowering; exact same-volume 562-file rollback attribution; 18/73/74/74 changed Evidence/Semantic/Intent/adapter artifacts; 397→344 evidence constraints, 31,767→28,876 invariants, 281→239 transactions, 2,156→2,489 signal-neutral conditional rules, and renderability 44→17; all 17 emitted ISFs pinned-FSMGen-strict clean; exact 24/24 measurable EvidenceIR plus 78/78 downstream currency; full repository CI`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii — make document identifiers opaque`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii`
  Status: `done`
  Goal: `neutralize production prompts and replace named corpus organization with structural organization`
  Acceptance: `model prompts describe generic typed digital intent with synthetic placeholders and current-document grounding only; production corpus/KG pages and routing use structural labels rather than a named protocol family`
  Verification: `complete census of 13 production prompt constructors/families; identifier-redacted entity judgment, declaration-ordered prompt catalogs, undeclared-contract residualization, and independent constraint typing controls; fixture structure/value/name invariance controls; 30/30 corpus-KB and 16/16 KG command tests; 156/156 tracked fixtures; ten managed Markdown outputs plus schema-2 JSON currentness; warning-deny Clippy; 1,900 Rust tests passed / six ignored / zero failed; all eight doctrines, mdBook test/build, and final locality pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii — neutralize prompts and corpus organization`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv`
  Status: `done`
  Goal: `qualify the complete identity-and-spelling remediation and publish exact current deltas`
  Acceptance: `all retained chains are current; identity and alpha-renaming controls cover the repaired paths; public/live/book/retrieval truth records intended recall changes and remaining .e/.f boundaries; full CI passes`
  Verification: `exact pre-.d range 89d8dee7..9c38b569 is three commits / 129 files / +5,922/-4,357, including 24 compiled-crate Rust source files; focused filters pass 13 alpha, 19 identity, 14 prompt, 14 spelling, two fixture-name, one empty-declaration, and one undeclared-contract test; current-binary doctrines prove 24/24 measurable EvidenceIR and 78/78 SemanticIR/IntentIR/adapters; all 17 current renderable ISFs retain prior FSMGen-strict qualification; exact recall and corpus/prompt deltas published; full CI green`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv — qualify identity remediation`

<!-- spec-to-intent-task-source-region:genericity-nodes-06d-ii-06d-ii-d-iv:end -->

<!-- spec-to-intent-task-source-region:acceptance-genericity-remediation:start -->
## Parent closure criteria — `SPEC-TO-INTENT-ALIGNMENT.6d.ii`

- **REPRODUCE / MEASURE** — replay all 12 unchanged source identities and 48 isolated stages at committed
  revision `b977a51f`; compare every reviewed cell and aggregate with the committed `.6c.ii` authority.
- **ROOT CAUSE (WHY + WHERE)** — prove that only the unit-derived applicability carrier changes the two
  OpenCAPI analog cells, separating the three repaired keys from every retained true positive and unrelated
  fabrication, loss, provenance, or capability defect.
- **ADDRESSED (verified)** — publish the portable replay/result/controller authorities with exact residual,
  category, aggregate, hard-gate, and next-owner outcomes; strict loaders and mutations must reject drift.
- **NO REGRESSION** — retain 24 true positives, 29/29 provenance, 12/12 replay currency, all unaffected cell
  semantics, frozen review authority, canonical generated artifacts, complete cleanup, and full CI.
- **GENERICITY (ADR 0006)** — the measured change follows the closed unit disposition and data-driven
  evaluator; no document/vendor/table/review-key branch or controller exception is introduced. Audit every
  production Rust source, remove all historical named-spec/vendor/protocol/signal vocabulary, and register a
  fail-closed whole-production-surface doctrine whose specificity authority and mutation fixtures remain test-only.
- **LOCKSTEP** — result, controller, task/roadmap/book/live/retrieval truth, cleanup evidence, and resume
  pointer agree that OpenCAPI closes while the two remaining register fabrications move to owned `.6e`.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a`

- [x] **REPRODUCE / MEASURE** — publish the exact 12-source / 48-stage `b977a51f` replay and compare every cell,
  aggregate, artifact identity, and cleanup fact with `.6c.ii`.
- [x] **ROOT CAUSE (WHY + WHERE)** — audit every production source and extraction/optional/adapter boundary;
  distinguish universal structure/input-derived behavior from identity, named schema, signal spelling, corpus
  phrase, prompt, threshold, comment, or core→conformance coupling; focused `cargo test` localizes the boundary.
- [x] **ADDRESSED (verified)** — move exact reviewed snapshot composition/assertions below `#[cfg(test)]`, remove
  its production example/IR registration, and durably assign every remaining audit family to `.6d.ii.b`–`.f`.
- [x] **NO REGRESSION** — preserve generic evaluator/controller behavior, exact replay/result/controller bytes,
  all 24 true positives, 29/29 provenance, frozen authorities, canonical generated artifacts, and zero scratch
  residue; focused `cargo test` is green and `scripts/run_ci.sh` is the closing oracle.
- [x] **GENERICITY (ADR 0006)** — record that a finite vocabulary list is diagnostic only and define structural
  module/type/grammar enforcement plus alpha-renaming, identity, paraphrase, negative-control, and held-out proof.
- [x] **LOCKSTEP** — audit, ADR, historical correction, task tree, roadmap, live docs, architecture analysis,
  mdBook, Knowledge Map, replay authorities, and bounded resume pointer publish the same breach and next owner.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b`

- [x] **REPRODUCE / MEASURE** — apply the exact replacement classifier read-only to all 78 retained SourceIRs;
  record 2,293 diagram, 2,123 section, and 3,484 table re-ingest label changes without rewriting an artifact.
- [x] **ROOT CAUSE (WHY + WHERE)** — localize SourceIR bias to operation/participant/protocol-role caption cues,
  body-literal guesses, named packet exclusions, substring roles, named production examples, and the retained
  schema-1 label bypass in `ir/source.rs` plus embedded Docling Python.
- [x] **ADDRESSED (verified)** — emit schema 2 from whole generic form/role grammar, keep incomplete forms
  unknown, neutralize every schema-1 semantic source label on load, reject future schemas, and prove the shipped
  Python plus compatibility boundary with focused tests.
- [x] **NO REGRESSION** — formatting, warning-deny Clippy/rustdoc, full Rust tests, mdBook, all doctrines,
  project-data locality, and residue checks pass; all direct generated-artifact changes are exact, guarded, and
  reconciled under ADR 0025.
- [x] **GENERICITY (ADR 0006)** — identity/symbol/operation perturbations and table-column reordering cannot
  change classification authority; specific spellings appear only below the test boundary; the retained recall
  reduction is accepted rather than repaired with an exception.
- [x] **LOCKSTEP** — audit, SourceIR/mdBook contract, task/roadmap/live docs, architecture analysis, Knowledge
  Map, bounded resume pointer, and next `.6d.ii.c` owner agree at commit.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c`

- [x] **REPRODUCE / MEASURE** — census all 78 persisted chains, re-extract every one of the 24 retained source
  bundles, and attribute each schema/content/downstream delta against an exact same-volume backup.
- [x] **ROOT CAUSE (WHY + WHERE)** — localize the breach to named SWD operation, phase, signal-direction, and
  response inference in the EvidenceIR schema/extractors; named schema made corpus knowledge executable even
  when the code lived in a nominally general stage.
- [x] **ADDRESSED (verified)** — schema 2 carries only opaque source-derived operation/branch/phase/actor/state
  values, refuses incomplete structural claims, neutralizes named schema-1 protocol surfaces, rejects future
  schemas, and projects the generic records losslessly through SemanticIR and IntentIR into honest ISF residuals.
- [x] **NO REGRESSION** — chain currency is exact at 24/24 replayable EvidenceIR and 78/78 downstream chains;
  focused extraction tests, all eight doctrines, warning-deny Clippy/rustdoc, 1,876/6/0 full tests, mdBook, final
  locality, and exact scratch cleanup pass.
- [x] **GENERICITY (ADR 0006)** — production extraction stores no protocol, response, signal, participant, phase,
  or operation dictionary; tests may name concrete protocols as conformance inputs, while reduced recall remains
  visible instead of being repaired by a corpus exception.
- [x] **LOCKSTEP** — ADR 0035, code, all persisted artifacts, public schema/book, research audit, task/live docs,
  category contract, Knowledge Map, bounded resume pointer, and next `.6d.ii.d` owner agree at commit.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i`

- [x] **REPRODUCE / MEASURE** — preserve the original schema-6 store and every affected stage in an exact
  same-volume rollback root; census its declared inputs, policy eligibility, prior families, affected artifacts,
  and adapter deltas before accepting a migration.
- [x] **ROOT CAUSE (WHY + WHERE)** — `ProtocolFamily::infer` read document key/display name, learning partitioned
  seven prior families by that inferred identity, and EvidenceIR/SemanticIR used the partition as executable
  lookup authority; the retained store was independently stale and non-reproducible under its validation policy.
- [x] **ADDRESSED (verified)** — schema 7 admits only typed global scope, learning/lookup never receives document
  identity, schemas 1–6 quarantine scoped semantic arrays, future/non-global schemas reject, and contested global
  values stay explicit; four learns after replay converge byte-identically.
- [x] **NO REGRESSION** — 156/156 tracked KG fixtures, byte-identical fixed-point output, 24/24 measurable
  EvidenceIR and 78/78 downstream chain currency, 58 changed-IR plus two-adapter validations, pinned FSMGen
  strict, warning-deny Clippy/rustdoc, 1,880/6/0 Rust tests, mdBook, and exact cleanup pass.
- [x] **GENERICITY (ADR 0006)** — filenames, titles, vendor/protocol names, and family labels are provenance only;
  prior applicability depends on normalized current-document evidence or a derived structural fingerprint, while
  named test documents remain permitted solely as conformance inputs.
- [x] **LOCKSTEP** — ADR 0036, schema/loaders, learning/validation/KG fixtures, all affected generated chains,
  mdBook, pipeline audit, live/architecture docs, Knowledge Map, task frontier, and bounded resume pointer state
  the same identity-independent boundary and the remaining `.d.ii`–`.f` blockers.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii`

- [x] **REPRODUCE / MEASURE** — preserve the exact pre-change four-stage population on the repository volume,
  rebuild every replayable EvidenceIR and all 78 downstream chains, and attribute every non-validation field plus
  aggregate constraint/invariant/transaction/renderability movement before deleting the rollback.
- [x] **ROOT CAUSE (WHY + WHERE)** — case/length/suffix filters, name-substring roles, case-folded identity
  collapse, circular NLP declaration synthesis, semantic/validation fallback, IntentIR pattern synthesis, and ISF
  conventional clock/reset defaults made arbitrary spelling executable; removing case rejection also exposed
  under-specified parenthetical/appositive and descriptive-invariant grammars.
- [x] **ADDRESSED (verified)** — identities are opaque; typed structure or bounded grammar declares them;
  exact-first unique-only current-document grounding governs optional proposals; later stages may carry, narrow,
  contest, or residualize but cannot re-infer; missing clock/reset authority blocks target emission.
- [x] **NO REGRESSION** — all 17 honestly renderable ISFs pass pinned FSMGen strict; validators and currency are
  exact; complete Rust, rustdoc, mdBook, doctrine, and locality gates pass; the 562-file rollback is absent.
- [x] **GENERICITY (ADR 0006)** — alpha-renaming changes only copied symbol identity, never admission, role,
  validation, or lowering eligibility. No replacement allowlist/denylist was introduced; positive grammar and
  current-document evidence supply authority.
- [x] **LOCKSTEP** — ADR 0037, code, exact generated population, mdBook, audit, live/architecture docs, Knowledge
  Map, task frontier, and bounded resume pointer publish the same one-way grounding boundary and remaining
  `.d.iii`–`.f` release blockers.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii`

- [x] **REPRODUCE / MEASURE** — census every production prompt constructor and corpus-KB routing decision;
  preserve the exact 156-fixture outcome surface while proving which old pages/candidates were selected by names.
- [x] **ROOT CAUSE (WHY + WHERE)** — prompts carried named examples/protocol framing, entity typing exposed the raw
  identifier to model world knowledge, spelling-sorted catalogs could change order under alpha-renaming, contract
  and constraint proposals lacked closed catalogs, and corpus-KB classified fixture-name fragments.
- [x] **ADDRESSED (verified)** — thirteen prompt families state typed digital-hardware policy; entity names are
  redacted from model/helper inputs; catalogs preserve declaration order; undeclared contracts residualize;
  constraint typing is independently catalog-grounded; KG capabilities use populated typed fields only.
- [x] **NO REGRESSION** — all 156 fixtures pass; ten managed Markdown pages and schema-2 candidate JSON are exact;
  focused prompt/corpus tests, warning-deny Clippy, 1,900/6/0 full Rust tests, mdBook, all eight doctrines, and
  project-data locality pass.
- [x] **GENERICITY (ADR 0006)** — vendor/protocol/document/symbol identity cannot choose prompt instruction,
  entity type, declaration order, corpus page, candidate presence, or control status; exact fixture names remain
  conformance provenance only, and a vocabulary census is not treated as proof.
- [x] **LOCKSTEP** — code, generated corpus KB, currentness authority, audit, mdBook, live/architecture docs,
  Knowledge Map, task frontier, and bounded resume pointer agree that `.d.iii` is closed while `.d.iv`–`.f`
  remain release blockers.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv`

- [x] **REPRODUCE / MEASURE** — compare the exact post-generic-EvidenceIR boundary `89d8dee7` with the clean
  `.d.iii` revision `9c38b569`, partition the complete changed-file range, and retain every child artifact delta.
- [x] **ROOT CAUSE (WHY + WHERE)** — identity authority crossed four independent seams: prior selection,
  identifier-derived semantics/lowering, model-facing policy, and corpus organization. A finite vocabulary
  search or one local unit test cannot prove their composition.
- [x] **ADDRESSED (verified)** — schema-7 global priors, opaque one-way-grounded symbols, typed/grounded neutral
  prompts, and schema-structural corpus capabilities compose without a fallback identity or spelling branch.
- [x] **NO REGRESSION** — chain currency is exact at 24/24 measurable EvidenceIR and 78/78 downstream; all 17
  honestly renderable ISFs retain FSMGen-strict qualification; the complete Rust, doctrine, rustdoc, mdBook,
  and locality gates pass.
- [x] **GENERICITY (ADR 0006)** — focused alpha, identity, prompt, spelling, fixture-name, and fail-closed
  proposal filters pass. Exact recall reductions remain visible and are not repaired with a document exception.
- [x] **LOCKSTEP** — the pipeline audit, Knowledge Map fact/projection, mdBook, live docs, task status, and resume
  pointer publish the exact combined range and state that `.e`/`.f`, not `.d`, still block whole-core signoff.

<!-- spec-to-intent-task-source-region:acceptance-genericity-remediation:end -->
