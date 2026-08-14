# SPEC-TO-INTENT-ALIGNMENT — program foundation

- Part ID: `program-foundation`
- State: `legacy`

<!-- spec-to-intent-task-source-region:program-identity:start -->
# SPEC-TO-INTENT-ALIGNMENT: steer PDF evidence into complete executable-intent IR

## Metadata

- Tree ID: `SPEC-TO-INTENT-ALIGNMENT`
- Status: `active`
- Roadmap lane: extraction quality and breadth
- Created: `2026-08-11`
- Last updated: `2026-08-13`
- Owner: project owner and repo-local workflow

## Goal

Keep SpecForge converging on its specification-to-executable-intent endpoint by making faithful PDF-to-IR
population the present program constraint, measuring the semantic content that survives every IR boundary,
and integrating production extraction capabilities into the canonical path before treating downstream ISF
expressiveness as the bottleneck.

## Non-Goals

- Do not weaken the specification-to-executable-intent north star.
- Do not expand ISF or FSMGen speculatively. Their active development can absorb demonstrated language gaps
  after upstream IR contains the source semantics that expose those gaps.
- Do not equate strict-valid `.isf`, passing unit tests, artifact currency, or an honest blocked emission with
  complete source-intent recovery.
- Do not refactor large IR modules without a measured extraction or integration need.

## Acceptance Criteria

- The evidence-backed trajectory assessment and the owner's PDF-to-IR priority decision are durable outside
  chat, with contradictions and unknowns stated explicitly.
- The automatic-steering design reuses existing completeness instruments, defines convergence/divergence/stall
  without a gameable blended score, and leaves canonical semantic mutation review-gated.
- The roadmap and mdBook state one current order: source capture and complete canonical IR first; demonstrated
  ISF/FSMGen gaps later.
- Each supported document category receives a measurable source-evidence-to-IntentIR content contract.
- The canonical production workflow accounts for every production extractor: integrated, deliberately
  scheduled, or explicitly blocked with a reason.
- Real multimodal inputs reach their typed IR consumers; synthetic downstream coverage is not reported as an
  operational PDF capability.
- Semantic recall, precision, provenance, residuals, and stage loss are evaluated on held-out vertical slices.
- Each completed leaf is committed through `COMMIT.md` after focused and doctrine verification.

<!-- spec-to-intent-task-source-region:program-identity:end -->

<!-- spec-to-intent-task-source-region:program-nodes-00-05b:start -->
## Task Tree

- ID: `SPEC-TO-INTENT-ALIGNMENT`
  Status: `active`
  Goal: `make PDF-to-IR semantic capture, not speculative ISF breadth, the measured path to executable intent`
  Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `SPEC-TO-INTENT-ALIGNMENT.0`
  Status: `done`
  Goal: `persist the code/roadmap/book trajectory assessment and the owner's upstream-first direction`
  Acceptance: `an ADR, Knowledge Map fact card, roadmap, current-status surface, mdBook, and resume pointer agree that PDF-to-IR completeness is the current constraint; concrete implementation gaps are separated from inference; all documentation and doctrine gates pass`
  Verification: `full run_ci gate passes: all 8 doctrines, formatting, clippy with warnings denied, 1,810 Rust tests passed / 5 ignored / 0 failed, Rust docs, mdBook examples/build, and post-producer project-data-locality; focused Knowledge Map, task catalog, canonical catalog, live-size, and book-current-truth checks pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.0 — fix the upstream objective and trajectory control`

- ID: `SPEC-TO-INTENT-ALIGNMENT.1`
  Status: `done`
  Goal: `define the per-document-category evidence-to-IntentIR content and completeness contract`
  Acceptance: `each supported category names required source modalities, typed IR surfaces, provenance and residual outcomes, measurable recall/precision floors, and honest non-applicable content without using ISF emission as the upstream completeness proxy`
  Verification: `the versioned machine-readable contract covers all six categories and passes strict semantic validation; mdBook test/build, Knowledge Map derive/check, fact-card catalog, 66 focused completeness tests, all doctrine checks, and the full repository CI gate pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.1 — define source-to-IntentIR category contracts`

- ID: `SPEC-TO-INTENT-ALIGNMENT.2`
  Status: `done`
  Goal: `make the canonical production workflow account for every production extraction capability`
  Acceptance: `the default workflow invokes, deliberately schedules, or explicitly reports omission of contract extraction, relation resolution, register recovery, NLI enforcement, and other production stages; tests prevent capability islands from being mistaken for end-to-end delivery`
  Verification: `the 17-row per-run ledger covers all 16 production commands; the Clap-derived test partitions all 28 subcommands and fails on an unclassified or unreported producer; provider-free CLI smoke, warning-deny Clippy, focused tests, mdBook, projections, exact rolling-ledger transaction, all doctrines, and full repository CI pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.2 — guard canonical capability accounting`

- ID: `SPEC-TO-INTENT-ALIGNMENT.3`
  Status: `done`
  Goal: `close the real PDF-to-typed-multimodal-IR producer gap`
  Acceptance: `a VLM timing observation on a real PDF visual asset produces a typed FigureRegion in EvidenceIR; only source-grounded lanes reach the FigureRegion-to-PartialTrace-to-verified-ActorContract path in SemanticIR/IntentIR; evidence validation counts both available and unavailable typed regions; a reviewed real-PDF fixture exercises the vertical path without pretending that a live VLM was available`
  Verification: `a reviewed retained-PDF fixture crosses every persisted stage and proves grounded WS contract survival plus model-only lane rejection; 14 FigureRegion tests, 17 waveform tests, the vertical semantic/intent test, the available/unavailable validation test, warning-deny Clippy, mdBook, Knowledge Map, doctrines, and full repository CI pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.3 — activate typed figure-region mining`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4`
  Status: `done`
  Goal: `prove trajectory with held-out source-to-IntentIR vertical slices and stage-loss accounting`
  Acceptance: `representative held-out PDFs have reviewed intent gold, per-stage survival/loss accounting, recall/precision/provenance/residual results, and an explicit conclusion about which remaining blocker is upstream extraction versus demonstrated ISF expressiveness`
  Verification: `the frozen 12-document / 14-cell report publishes every exact score and first stage; focused evaluator/example checks, formatting, warning-deny Clippy, mdBook, Knowledge Map/catalog, live/currentness projections, all eight doctrines, and full CI with 1,834 passed / five ignored / zero failed all pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.4c — publish the first reviewed result`
  Children: `.4a`, `.4b`, `.4c`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4a`
  Status: `done`
  Goal: `build the deterministic vertical-evaluation contract and prove the evaluator detects controlled faults`
  Acceptance: `a versioned, root-relative dataset schema and evaluator compute scoped precision/recall, source/modality disposition, provenance closure, residual actionability, stage conservation, and first-failing-stage results; omission, fabrication, provenance-loss, silent-drop, and missing-modality mutations are all detected`
  Verification: `12 focused evaluator tests; schema parse/semantic assertions; formatting and warning-deny Clippy; mdBook test/build; Knowledge Map, catalog, book-current-truth, live-size, and rolling-ledger checks; full CI with all eight doctrines and 1,832 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.4a — build the vertical evaluation engine`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4b`
  Status: `done`
  Goal: `lock two representative reviewed vertical documents per category without extractor tuning`
  Acceptance: `the 12-document population pins portable source and four-stage identities, records source presence and complete bounded-scope gold, distinguishes repository inputs from necessary read-only external inputs without persisting host paths, and removes the obsolete boot-volume livework prefix from generated repository records in favor of the verified SSD location`
  Verification: `12 documents / 14 complete cells / two documents in every category; four repository and eight portable external sources; deterministic builder and 13 evaluator tests; corpus-frontier self-test 13/13; zero obsolete livework references; 104 authorized SSD fields resolve; formatting, warning-deny Clippy, mdBook, projections, all eight doctrines, and full CI with 1,833 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.4b — lock the reviewed vertical population`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4c`
  Status: `done`
  Goal: `publish exact held-out outcomes, stage-loss diagnosis, and the next measured blocker`
  Acceptance: `the deterministic snapshot publishes every denominator and category status, names all hard failures and first failing stages, and concludes whether the next constraint is upstream capture or a demonstrated downstream expressiveness gap`
  Verification: `104,669-byte result snapshot at SHA-256 6b72f1fc…47eb; generic runner reproduces it byte-for-byte; 14 focused evaluator tests, formatting, all-target warning-deny Clippy, mdBook, projections, all doctrines, and full CI with 1,834 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.4c — publish the first reviewed result`

- ID: `SPEC-TO-INTENT-ALIGNMENT.5`
  Status: `done`
  Goal: `build an evidence-driven trajectory controller that detects convergence, stall, and divergence and proposes the next highest-value owned slice`
  Acceptance: `a machine-readable objective contract drives a multidimensional trajectory snapshot; held-out semantic outcomes, per-stage loss, modality/category coverage, residual debt, canonical-path participation, and hard honesty gates are measured; statistically justified drift/stall rules cannot be hidden by a blended score; recommended work is task-tree-owned and remains human-reviewable`
  Verification: `generic .5a state/ranking controls plus .5b live-equal evidence composition, byte-current input/report, owned ranking, all focused/public checks, and full repository CI with 1,854 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.5b — publish the first trajectory snapshot`
  Children: `.5a`, `.5b`

- ID: `SPEC-TO-INTENT-ALIGNMENT.5a`
  Status: `done`
  Goal: `build the versioned multi-metric trajectory state and ranking engine before attaching product evidence`
  Acceptance: `a strict root-relative input contract preserves nine separate objective dimensions, denominators, oracles, uncertainty, hard gates, comparable history, and task ownership; deterministic classification distinguishes converging, diverging, stalled, mixed, and unmeasurable; lexicographic ranking cannot trade away hard failures; seeded omissions, fabrications, regressions, mixed deltas, missing history, and semantic-mutation attempts fail closed`
  Verification: `16 focused state/fault/authority tests; schema JSON semantics; format and all-target warning-deny Clippy; mdBook; Knowledge Map/catalog; live/currentness projections; all eight doctrines; full CI with 1,850 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.5a — build the trajectory control engine`

- ID: `SPEC-TO-INTENT-ALIGNMENT.5b`
  Status: `done`
  Goal: `compose the frozen reviewed result and canonical capability evidence into the first trajectory snapshot and owned task proposal`
  Acceptance: `the byte-pinned snapshot derives exact .4c category/stage metrics and .2 production-path participation without changing either authority, classifies missing trend history honestly, publishes every ranked gap and why the winner outranks the rest, and opens the recommended task-tree leaf before any implementation pivot`
  Verification: `three snapshot/currentness/mutant tests; live provider-free ledger equality; generic byte replay; format and all-target warning-deny Clippy; mdBook; Knowledge Map/catalog; public/live projections; all eight doctrines; full CI with 1,854 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.5b — publish the first trajectory snapshot`

<!-- spec-to-intent-task-source-region:program-nodes-00-05b:end -->

<!-- spec-to-intent-task-source-region:decisions:start -->
## Decisions

- `2026-08-11`: retain the specification-to-executable-intent objective; clarify and measure it rather than
  narrowing it to what the current pipeline happens to emit.
- `2026-08-11`: the project owner identifies faithful content population from PDFs through the IR pipeline as
  the present bottleneck. ISF is not the blocking point now; defer language expansion until populated IntentIR
  exposes a concrete, source-grounded expressiveness gap.
- `2026-08-11`: treat the current codebase as architecturally convergent but not yet product-complete. Passing
  gates establish implementation integrity, not semantic completeness.
- `2026-08-11`: do not use raw `.isf` emission count as the sole completion measure. Honest non-emission is
  evidence of anti-fabrication, but upstream coverage still needs an independent content oracle.
- `2026-08-11`: automatic steering will compose existing exact snapshots, gold evaluation, region accounting,
  closure invariants, recall estimates, and residuals into a versioned objective vector. It may gate exact
  invariants and propose the next owned task; it may not silently promote canonical semantic truth (ADR 0034).
- `2026-08-11`: category completeness is judged per reviewed source meaning and required modality, with exact
  precision/recall and conservation/provenance floors. A residual preserves loss honestly but does not satisfy
  the typed-content requirement it replaces.
- `2026-08-11`: non-applicability requires independent evidence that the source content is absent. A blocked
  adapter or zero emitted ISF is never sufficient evidence that upstream capture is complete.
- `2026-08-11`: capability participation and per-run execution are independent. A provider-backed stage can
  be integrated yet `not_executed` for one run; a standalone capability remains `omitted` until the canonical
  path composes or deliberately schedules it.
- `2026-08-11`: the CLI itself is the completeness oracle for capability accounting. All 28 subcommands need
  a role classification, and each of the 16 classified production commands must appear in the convergence
  ledger; adding an unclassified or unreported command fails tests.
- `2026-08-11`: do not imply integration to close `.2`. Constrained contracts, Tier-3 actor/signal resolution,
  register-bit recovery, and IntentIR NLI demotion remain explicit omissions; condition repair is covered only
  when the integrated LLM-primary replacement actually executes.
- `2026-08-11`: close `.3` at the existing production observation seam: carry an optional typed `FigureRegion`
  on the matching EvidenceIR visual item, mine only lanes grounded in the document's known signal catalog, and
  round-trip-verify every candidate before fusion/fidelity. Do not add a full-page pass (measured NO-GO), infer
  lane order from unlabeled samples, or claim a live VLM run while no endpoint is ready.
- `2026-08-11`: a later stable span is not lowerable without a trigger/phase anchor. The current `Stable`
  obligation cannot represent a nonzero trace origin, so moving such a span to tick zero would change the
  evidence; retain it as an explicit residual even if the lane is otherwise clean.
- `2026-08-11`: the frozen `.4c` result leaves all six categories incomplete. Ten cells first fail at SourceIR
  → EvidenceIR and four at EvidenceIR → SemanticIR; none first fails at SemanticIR → IntentIR. Therefore the
  measured next constraint is upstream fact formation and residualization, not a demonstrated ISF/FSMGen gap.
- `2026-08-11`: freeze the generic controller in `.5a` before composing the known `.4c` result in `.5b`. This
  prevents state and ranking branches from being tuned to produce a preferred label for the current population.
- `2026-08-12`: keep `.5b` composition outside the generic engine and preserve the provider-free `.2` ledger as
  contextual execution evidence. A current hard-gate failure may classify divergence without history, while
  `insufficient_history` independently forbids a trend or stall claim.
- `2026-08-12`: `.4c` is a pinned retrospective stage-artifact baseline, not proof that every selected artifact
  was replayed by the selection-boundary binary. The dominant 19-fact TOC family persists in an unreplayed
  EvidenceIR artifact even though pre-boundary commit `46af2eca` added the generic structural timing gate; its
  normalized input was reclaimed. `.6a` must replay from the hash-equal repository-local SSD copy in an isolated
  repository-volume root before `.6` changes production code or claims a current-product repair.
- `2026-08-12`: `.6a` proves the distinction empirically. The hash-equal source replay keeps reviewed TOC
  `table_0004` at 20 rows × two columns, while current normalization classifies it `unknown` rather than
  `timing_parameter`; the 19 timing records disappear at all three promoted stages and the reviewed true-positive
  population remains zero. The controller therefore treats `.4c` quality counts as retrospective baseline
  measures, gates current-binary coverage at 1/12, and ranks qualification of the remaining 11 documents first.
- `2026-08-12`: split `.6b` into qualification (`.6b.i`) and repair (`.6b.ii`) so source recovery, replay,
  projection, and result interpretation complete without production extraction changes. The qualified current
  inventory, rather than any frozen count, is the sole repair-selection authority.
- `2026-08-12`: reverify all 12 source byte identities before replay. The four repository sources and eight
  necessary read-only external sources are all on the SSD filesystem; external inputs are copied into a fresh
  repository-derived scratch root, verified again, used there, and the exact copies and outputs are deleted
  only after portable evidence is promoted.
- `2026-08-12`: the first whole-population attempt stopped after two documents because an eager Python default
  dereferenced a repository path on an external-source row. Repair the generic manifest branch, delete the exact
  38 MiB failed root, rerun from a fresh root, and retain no partial evidence as current authority.
- `2026-08-12`: the authoritative current replay covers 12/12 sources and 48/48 stage artifacts. Current
  canonical TP/FP/FN are 7/22/33; the 19-count frozen-to-current fabrication and provenance improvement is
  exactly the AIA TOC family already disproved by `.6a`, while true positives, false negatives, and stage loss
  do not improve. Therefore `.6b.ii` selects the bounded Arm Debug family: 12 correct register names currently
  omit reviewed access mode and account for 12 false positives, 12 false negatives, and 12 provenance failures.
- `2026-08-12`: the `.6b.ii.a` full gate proves the shared register carrier changes four retained EvidenceIR
  artifacts: Arm Debug, OpenCAPI Discovery Configuration, USB4 Inter-Domain Service, and USB 3.2. The current
  repair leaf owns their deterministic EvidenceIR → SemanticIR → IntentIR → adapter reconciliation as the
  required ADR 0025 currency transaction; this is not a new document refresh or task-tree pivot. Exact
  baseline/change deltas must be captured before replacement, every stage revalidated, and all other retained
  chains must replay byte-current before signoff.
- `2026-08-12`: the first `.6b.ii.b` population run completed seven documents, then the current default Docling
  single-pass path was terminated by signal while starting the 400-page Arm Debug source; an independent fresh
  retry failed at the same post-weight-load point. Host memory read 85% free afterwards, no subprocess remained,
  and a forced bounded run (`SPECFORGE_INGEST_BATCH_THRESHOLD=256`) completed all four stages. Its SourceIR has
  the same profile, tables, elements, sections, and path-normalized visuals as the persisted single-pass
  authority. The exact failed 2,567-file / 765,728-KiB root and empty retry root were removed. `.6b.ii.b` may use
  a command-recorded threshold of 399, which affects only the 400-page reviewed source; `.6b.iii` owns repairing
  the disproved global assumption that every source at or below 500 pages is safe for single-pass conversion.
- `2026-08-12`: the clean `.6b.ii.b` replay at revision `bb152dfb` covers 12/12 sources and 48/48 stages.
  Arm Debug closes exactly from 0 TP / 12 FP / 12 FN / 12 unprovenanced to 12/0/0/0. AMD IOMMU and GIC-400
  each close one direct-table provenance failure while retaining the same false key; no other cell changes.
  Aggregate TP/FP/FN become 19/10/21, provenance 17/29, and conservation 57/78. Controller v4 therefore
  selects `.6c`, the five-record I2S missing-`ns` family, while `.6b.iii` remains the operational ingestion-risk
  frontier. The 3,913-file / 1,090,884-KiB population root and runtime source map are removed and absent.
- `2026-08-12`: `.6b.iii` localizes the unsafe selection to the split Rust/Python policy seam: Rust adapted only
  batch size, while embedded Python retained a flat 512-page activation default and treated an unreadable page
  count as permission for single-pass conversion. The generic repair resolves activation once from total physical
  RAM using the measured 75-MB/page working-set estimate, a 40% budget, and an unconditional 399-page cap; the
  24-GiB host therefore selects threshold 131 and batch size 64. Explicit nonnegative threshold overrides remain
  exact. Unknown page count now refuses unbounded conversion, and Unix signal termination has a distinct typed
  error that does not mislabel the signal as OOM.
- `2026-08-12`: a live Arm Debug replay with no threshold override selected 131/64 and completed all four stages.
  Its 400-page / 210-table / 176-figure / 6,784-element / 1,321-section SourceIR matches the retained authority:
  profile, tables, elements, and sections are byte-identical, while page/visual manifests match after path
  normalization. EvidenceIR, SemanticIR, and IntentIR also match after removing validation backannotations and
  normalizing paths. The exact 795-file / 184,164-KiB replay root was removed and is absent.
- `2026-08-12`: `.6c.i` keeps carrier implementation and retained-corpus reconciliation separate from result
  publication. Fixed retained SourceIR inputs identify exactly five moved EvidenceIR chains and prove the shared
  delta is 231 direct table ids plus five I2S `ns` units; `.6c.ii` alone will re-ingest the reviewed 12-source
  population at the clean carrier revision and publish the comparable product/controller result.
- `2026-08-12`: the clean `.6c.ii` replay at revision `74a658b3` covers the same 12 source identities and all 48
  isolated stages. Fresh root/revision metadata changes every stage digest, so semantic causality comes from the
  locked 14-cell projection: only I2S changes keys, from 0/5/5/5 to 5/0/0/0 TP/FP/FN/unprovenanced; seven
  OpenCAPI facts gain provenance only. Aggregate TP/FP/FN become 24/5/16, provenance reaches 29/29, conservation
  reaches 72/88, and all 19 prior true positives survive. The remaining five fabrications are three OpenCAPI
  analog records plus one AMD IOMMU and one GIC-400 key, so the hard-first controller selects owned `.6d` before
  `.7` recall work. The exact 3,913-file / 1,090,908-KiB root and runtime map are removed and absent.
- `2026-08-12`: activated `.6d` after committing the owner-requested FSMGen currentness verification. Start from
  the qualified replay/result/controller authorities, pin the exact three false analog keys, and compare their
  complete typed source/table shape with the four correct OpenCAPI digital-skew facts before changing production.
- `2026-08-12`: `.6d` localizes the defect to `synthesize_timing_constraints` plus lossless downstream cloning:
  every independent scalar row in a `timing_parameter` table enters the same canonical vector, with no typed
  quantity-domain applicability. The reviewed controls are four `UI` lane-skew facts; the false facts are
  `IL(f)|21 dB`, `ILD(f)|0.45 dB_RMS`, and `IL(f)|30 dB`, all directly supported by the mixed tables. The retained
  census finds 74 decibel-domain records across five EvidenceIR artifacts: 11 and 15 in the two reviewed OpenCAPI
  chains (all direct-table provenanced), plus 48 in three older CCIX chains. The generic seam is a closed unit-
  grammar disposition on each captured record: keep its values and source authority, mark decibel-domain physical
  quantities non-applicable to executable digital intent with actionable reason/boundary/replay data, and exclude
  only that typed disposition from executable temporal derivation. Do not key on parameter, table, document, or
  vendor; qualify the reviewed OpenCAPI family before deciding how much retained-corpus reconciliation is required.
- `2026-08-12`: split `.6d` at the same isolation boundary as `.6c`. `.6d.i` owns the typed carrier, validation,
  and ADR 0025 reconciliation; `.6d.ii` must replay the 12 reviewed sources at the committed `.6d.i` revision
  before publishing metrics or controller state. The currency oracle names exactly two measurable EvidenceIR
  chains. Three older CCIX chains have 48 matching retained records but no normalized bundles, so their EvidenceIR
  is explicitly refresh-owned and unmeasurable rather than silently claimed current.
- `2026-08-12`: activated `.6d.ii` only after `.6d.i` committed at `b977a51f` and the post-commit tree was clean.
  This leaf owns a fresh 12-source / 48-stage replay at that production revision, exact result comparison,
  controller re-ranking, portable authority promotion, and residue-free cleanup.
- `2026-08-12`: `.6d.ii` replay measurement changes exactly the two OpenCAPI analog cells: all three false
  canonical keys become exact actionable SemanticIR/IntentIR residuals while the other 12 cells remain
  semantically equal. TP/FP/FN are 24/2/16, provenance stays 29/29, physical-link becomes supported, and the
  remaining hard failures are the unrelated AMD IOMMU and GIC-400 register fabrications. Hard-first ordering
  therefore opens `.6e`; `.7` cannot precede it while the zero-fabrication gate still has two violations.
- `2026-08-12`: owner review of corpus-specific assertions triggers a complete production audit rather than a
  token-list patch. All 71 Rust sources, embedded Docling Python, canonical and optional model/prior/evaluation
  paths, adapters, prompts, comments, examples, and module boundaries are in scope. The audit disproves the old
  whole-repository neutrality claim and finds release-blocking identity-routed families, named protocol schema/
  extractors, signal-spelling inference, corpus-calibrated phrases/thresholds, and core/conformance coupling.
  A finite forbidden-vocabulary file is diagnostic only. Parent `.6d.ii` is split into `.a`–`.f`: publish/audit
  and test-boundary correction, SourceIR, EvidenceIR/schema, downstream identity/spelling removal, structural
  doctrine enforcement, and behavioral/whole-population qualification.
- `2026-08-12`: the theoretical boundary is explicit. A neutral extractor may own universal digital semantics
  and document grammar while carrying current-input names opaquely; it cannot promise facts absent or ambiguous
  in the source. Signoff means one identity-independent engine promotes only supported intent and emits explicit
  residuals otherwise. The breach requires removing shortcuts, not abandoning the project.
- `2026-08-12`: activated `.6d.ii.b` only after `.6d.ii.a` committed cleanly at `6b155380`. The SourceIR scope
  includes all production Rust plus embedded Python in `ir/source.rs` and `ir/source/docling_backend.rs`; tests
  remain conformance authority. Classification must use typed layout/geometry, generic document grammar, or
  current-document-derived evidence, retain `unknown`, and contain no named protocol/example commentary.
- `2026-08-12`: activated `.6d.ii.c` only after `.6d.ii.b` and the owner-requested FSMGen refresh committed
  cleanly at `2127c68a`. This leaf owns the complete production EvidenceIR schema/extractor audit and neutral
  replacement; no protocol, signal, response, document, or family spelling may survive as production authority.
- `2026-08-12`: ADR 0036 rejects identity-preserving prior migration. Schema 7 has only a typed global scope;
  schemas 1–6 quarantine seven family-scoped semantic arrays, while structural extraction profiles may survive
  because their applicability key is a derived document fingerprint rather than a name. Neutral learning must
  validate its declared inputs and converge byte-for-byte after every affected-chain replay.
- `2026-08-13`: `.e.v` is not one honest signoff slice. Its acceptance combines compiler-visible package/type
  boundaries, exact production-module discovery, syntax/name/macro closure, interprocedural information flow,
  canonical-promotion enforcement, and doctrine registration. Split it into `.e.v.i`–`.iv`: freeze the executable
  trust contract; derive a fail-closed production graph; enforce raw/identity noninterference and proof-only
  promotion; then register and qualify the composed doctrine. Compiler privacy and the existing proof kernel stay
  primary. AST analysis supplies whole-surface closure and reviewable diagnostics, while the finite vocabulary
  census remains diagnostic only and `.e.vi` retains adversarial mutations plus executable per-rule alpha proof.

<!-- spec-to-intent-task-source-region:decisions:end -->

<!-- spec-to-intent-task-source-region:legacy-open-question:start -->
## Open Questions

- Which currently omitted capability produces the largest held-out source-to-IntentIR gain and should therefore
  be integrated first? `.4` supplies the first frozen evidence; `.5` must rank it against the `.2` capability
  ledger without hiding hard failures in one score.

<!-- spec-to-intent-task-source-region:legacy-open-question:end -->

<!-- spec-to-intent-task-source-region:legacy-blocker:start -->
## Blockers

- None for `.5b`: the committed generic engine, exact reviewed result, category contract, stage-loss split,
  capability ledger, and automatic-steering design are available. Semantic mutation remains review-gated.

<!-- spec-to-intent-task-source-region:legacy-blocker:end -->

<!-- spec-to-intent-task-source-region:acceptance-02-05b:start -->
## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.2`

- [x] **REPRODUCE / MEASURE** — the pre-change command/path audit measured 28 clap subcommands, 16 production
  commands, and four standalone capability islands absent from canonical `converge`; the provider-free CLI
  smoke emitted no capability-accounting surface at baseline.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/cli.rs:19-76` exposes every producer independently,
  while `crates/specforge/src/commands/converge.rs:205-254` directly composes only enrich, EvidenceIR, NLP
  enrichment, SemanticIR, IntentIR, and adapter construction. Therefore `extract-contracts`, `signal-resolve`,
  `recover-register-bits`, and IntentIR NLI demotion could remain uninvoked without an operator-visible record.
- [x] **ADDRESSED (verified)** — provider-free `converge README.md` now emits exactly 17 JSON capability rows:
  all 16 production commands are accounted for, ordinary IntentIR and its NLI mode remain distinct, and the
  four genuine islands report `omitted` / `not_executed`. The Clap-derived test partitions all 28 commands and
  fails if a production command is unclassified, unreported, or assigned a duplicate capability ID.
- [x] **NO REGRESSION** — `bash scripts/run_ci.sh` exits zero with all eight doctrines, warnings-denied Clippy,
  1,813 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and the final locality
  residue check; the provider-free smoke's exact five reproducible artifacts were removed and the corpus
  frontier remains 57 = 52 refreshed + five remaining.
- [x] **GENERICITY (ADR 0006)** — accounting is derived from command roles and runtime participation, with no
  document, vendor, protocol, signal, or chip-specific exception.
- [x] **LOCKSTEP** — code, task frontier, roadmap, current status, architecture analysis, public mdBook,
  Knowledge Map fact/projections, bounded live state, and the lossless CHANGES archive transaction agree.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.3`

- [x] **REPRODUCE / MEASURE** — the baseline census found zero persisted VLM notes and no EvidenceIR
  `FigureRegion` field or producer; the retained NXP UM11732 page-4 timing asset was visually reviewed and pinned
  by PDF/crop identities while `doctor --strict` showed no ready live endpoint.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs` already injects timing JSON notes as
  `VisualObservation`, while `crates/specforge/src/ir/figure_region.rs` and `waveform.rs` began only at a typed
  synthetic consumer. The observation therefore stopped before `FigureRegion`; no source grounding or verifier
  could govern a real producer.
- [x] **ADDRESSED (verified)** — the reviewed vertical `cargo test` proves retained PDF → persisted EvidenceIR
  typed region → grounded/verified SemanticIR contract → identical IntentIR carry. Validation independently
  measures two observations as one available + one unavailable; a model-only lane is rejected per item.
- [x] **NO REGRESSION** — warning-deny `cargo clippy`, focused region/waveform/vertical/validation tests, and
  `bash scripts/run_ci.sh` are green with 1,820 passed / five ignored / zero failed; artifacts without timing
  notes omit the serde-default field and the persisted corpus remains unchanged. The exact Rust-analysis
  rollover leaves 51 live records / 1,018 lines / 85,828 bytes and preserves every sealed record through the
  validated archive chain.
- [x] **GENERICITY (ADR 0006)** — the producer accepts the bounded observation schema and grounds against each
  document's own signal catalog. Runtime code contains no vendor, protocol, document, or signal exception; I2S
  exists only in reviewed test data.
- [x] **LOCKSTEP** — code, fixture, validation metrics, roadmap, current status, architecture analysis, mdBook,
  Knowledge Map, task frontier, bounded resume pointer, and the owned rollover plan/archive describe the same
  delivered boundary and live-VLM limitation.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.4a`

- [x] **REPRODUCE / MEASURE** — lock the evaluator schema before population review; every score carries exact
  TP/FP/FN denominators, source and stage identities, semantic family, modality, and oracle metadata.
- [x] **ROOT CAUSE (WHY + WHERE)** — distinguish source absence, missing EvidenceIR capture, EvidenceIR →
  SemanticIR loss, SemanticIR → IntentIR loss, fabrication, provenance loss, and missing/inactionable residuals
  rather than collapsing them into one failed score.
- [x] **ADDRESSED (verified)** — a reusable Rust module validates repository-relative paths and dataset shape,
  evaluates stage snapshots deterministically, and serializes a stable machine-readable result.
- [x] **NO REGRESSION** — controlled omission, fabrication, provenance-loss, silent-drop, and missing-modality
  mutations each change the expected hard metric and are killed by focused tests; formatting, warning-deny
  Clippy, doctrines, and full CI pass.
- [x] **GENERICITY (ADR 0006)** — evaluation queries are data-defined JSON pointers, filters, keys, provenance
  fields, categories, families, and modalities; runtime code contains no document/vendor/protocol exceptions.
- [x] **LOCKSTEP** — task ownership, evaluator schema/API, public evaluator contract, architecture analysis,
  Knowledge Map, and resume pointer agree that `.4a` builds the oracle but makes no category-support claim.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.4b`

- [x] **REPRODUCE / MEASURE** — select exactly two documents in each of the six reviewed categories at the
  committed pre-selection boundary; pin every source and original four-stage artifact identity before reading
  the score or changing extraction behavior; census all obsolete
  boot-volume livework references before replacing them with the verified SSD path.
- [x] **ROOT CAUSE (WHY + WHERE)** — review bounded source regions directly and record exhaustive positive and
  negative gold for the stated scope, including source presence, required modality, canonical/residual
  disposition, provenance ids, and any first boundary where the persisted representation changes shape. For
  locality, prove that a source location is not evidence of a completed current-binary corpus refresh and that
  repairing retired paths would otherwise weaken the frontier's omission check.
- [x] **ADDRESSED (verified)** — a tracked version-1 dataset loads through the `.4a` root-derived strict loader,
  contains 12 unique documents / two per category, and evaluates deterministically without an unknown field,
  unsafe path, duplicate key, incomplete gold cell, or missing category/modality declaration. The corpus
  frontier uses an explicit root-neutral refreshed/remaining partition, and a seeded silent-drop mutation fails
  even when every external source path already names the SSD.
- [x] **NO REGRESSION** — no extractor or canonical corpus artifact changes; every migrated path resolves to the
  same source bytes on SSD, no boot-volume livework reference remains, downstream stage identities are
  re-pinned after generated metadata changes, and fixture identity, schema, focused evaluator tests,
  formatting, warning-deny Clippy, mdBook, doctrines, and full CI pass.
- [x] **GENERICITY (ADR 0006)** — document-specific truth exists only in reviewed fixture data; evaluator/runtime
  code remains free of vendor, protocol, document-key, signal-name, and page-layout exceptions.
- [x] **LOCKSTEP** — dataset review notes, task frontier, public fixture boundary, Knowledge Map, live status,
  architecture record, and resume pointer agree that `.4b` freezes inputs and `.4c` owns product conclusions.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.4c`

- [x] **REPRODUCE / MEASURE** — generate the complete pretty-JSON report from the frozen `.4b` dataset with the
  generic evaluator runner; pin its identity and every category, cell, denominator, failure, and first stage.
- [x] **ROOT CAUSE (WHY + WHERE)** — distinguish exact region/modality capture from typed canonical correctness,
  residual disposition, provenance, and boundary conservation; rank the blocker from the first-failing-stage
  evidence rather than from an aggregate score or adapter output count.
- [x] **ADDRESSED (verified)** — publish a byte-current result snapshot plus exact public/retrieval summaries;
  preserve selection, reviewed gold, extractor behavior, and every source/four-stage artifact unchanged.
- [x] **NO REGRESSION** — the result regenerates byte-identically; the focused evaluator suite, formatting,
  warning-deny Clippy, mdBook, projections, doctrines, and full repository CI pass.
- [x] **GENERICITY (ADR 0006)** — the result runner accepts any safe repository-relative dataset and contains no
  document, vendor, protocol, signal, category-outcome, or page-layout exception.
- [x] **LOCKSTEP** — snapshot, evaluator test, task frontier, roadmap, public book, live docs, Knowledge Map,
  architecture record, and resume pointer publish the same exact outcomes and upstream blocker conclusion.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.5a`

- [x] **REPRODUCE / MEASURE** — define a strict public input that carries all nine accepted dimensions, bounded
  rational denominators, targets, materiality, hard/required status, oracle, population, uncertainty, evidence,
  comparable history, gaps, and task ownership without a blended score.
- [x] **ROOT CAUSE (WHY + WHERE)** — encode the distinct evidence preconditions for current hard-failure,
  exact paired trend, mixed deltas, configured stall windows, missing oracles/history, and estimated uncertainty;
  do not let one state rule manufacture authority for another.
- [x] **ADDRESSED (verified)** — deterministically classify five states, rank gaps hard-first with visible
  tie-breakers, verify every proposed task ID in its root-relative task tree, and fix authority to report-only.
- [x] **NO REGRESSION** — state/fault/authority mutants, schema semantics, stable serialization, formatting,
  warning-deny Clippy, mdBook, projections, doctrines, and full repository CI pass.
- [x] **GENERICITY (ADR 0006)** — controller code contains no `.4c` metric, document, vendor, protocol, signal,
  or page-layout branch; `.5b` owns baseline-evidence composition after the engine is committed.
- [x] **LOCKSTEP** — schema, engine, research status, task frontier, roadmap, public book, Knowledge Map, live
  docs, architecture record, and resume pointer agree on state/ranking semantics and review authority.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.5b`

- [x] **REPRODUCE / MEASURE** — authenticate the frozen `.4c` result and exact provider-free `.2` ledger, derive
  all nine dimensions without hand-maintained metric totals, persist byte-current input/report artifacts, and
  reproduce the report through a generic repository-relative runner.
- [x] **ROOT CAUSE (WHY + WHERE)** — preserve the frozen baseline's hard failures while independently reporting
  `insufficient_history`; expose fabrication, provenance, first-boundary loss, residual debt, and capability
  participation without a scalar score or downstream-language inference. `.6a` later proved that `.5b` had not
  established whole-population artifact currency and corrected the controller's currentness claim.
- [x] **ADDRESSED (verified)** — publish every ranked gap and open its owner before evaluation; recommend the
  `.6` honesty lane ahead of `.7` source loss, `.8` residual actionability, and `.9` capability breadth. `.6a`
  later corrected the lane's first action from assumed repair to current-binary qualification.
- [x] **NO REGRESSION** — live-ledger equality, authority/profile mutants, byte currentness, generic replay,
  formatting, warning-deny Clippy, mdBook, projections, doctrines, and full repository CI pass without changing
  the reviewed dataset, `.4c` result, extractor behavior, or canonical IR.
- [x] **GENERICITY (ADR 0006)** — product constants exist only in the `.5b` composition adapter and tracked
  evidence; the `.5a` engine and generic input runner remain independent of document, vendor, protocol, signal,
  category outcome, and page layout.
- [x] **LOCKSTEP** — input/report, code, task ranking, roadmap, public book, live docs, research, architecture,
  Knowledge Map, and resume pointer publish the same exact state, denominators, history authority, and `.6` next.

<!-- spec-to-intent-task-source-region:acceptance-02-05b:end -->
