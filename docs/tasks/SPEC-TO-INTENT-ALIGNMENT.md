# SPEC-TO-INTENT-ALIGNMENT: steer PDF evidence into complete executable-intent IR

## Metadata

- Tree ID: `SPEC-TO-INTENT-ALIGNMENT`
- Status: `active`
- Roadmap lane: extraction quality and breadth
- Created: `2026-08-11`
- Last updated: `2026-08-11`
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
  Status: `in_progress`
  Goal: `prove trajectory with held-out source-to-IntentIR vertical slices and stage-loss accounting`
  Acceptance: `representative held-out PDFs have reviewed intent gold, per-stage survival/loss accounting, recall/precision/provenance/residual results, and an explicit conclusion about which remaining blocker is upstream extraction versus demonstrated ISF expressiveness`
  Verification: `pending`
  Commit: `pending`
  Children: `.4a`, `.4b`, `.4c`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4a`
  Status: `done`
  Goal: `build the deterministic vertical-evaluation contract and prove the evaluator detects controlled faults`
  Acceptance: `a versioned, root-relative dataset schema and evaluator compute scoped precision/recall, source/modality disposition, provenance closure, residual actionability, stage conservation, and first-failing-stage results; omission, fabrication, provenance-loss, silent-drop, and missing-modality mutations are all detected`
  Verification: `12 focused evaluator tests; schema parse/semantic assertions; formatting and warning-deny Clippy; mdBook test/build; Knowledge Map, catalog, book-current-truth, live-size, and rolling-ledger checks; full CI with all eight doctrines and 1,832 passed / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.4a — build the vertical evaluation engine`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4b`
  Status: `pending`
  Goal: `lock two representative reviewed vertical documents per category without extractor tuning`
  Acceptance: `the 12-document population pins portable source and four-stage identities, records source presence and complete bounded-scope gold, and distinguishes repository inputs from necessary read-only external inputs without persisting host paths`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.4c`
  Status: `pending`
  Goal: `publish exact held-out outcomes, stage-loss diagnosis, and the next measured blocker`
  Acceptance: `the deterministic snapshot publishes every denominator and category status, names all hard failures and first failing stages, and concludes whether the next constraint is upstream capture or a demonstrated downstream expressiveness gap`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.5`
  Status: `pending`
  Goal: `build an evidence-driven trajectory controller that detects convergence, stall, and divergence and proposes the next highest-value owned slice`
  Acceptance: `a machine-readable objective contract drives a multidimensional trajectory snapshot; held-out semantic outcomes, per-stage loss, modality/category coverage, residual debt, canonical-path participation, and hard honesty gates are measured; statistically justified drift/stall rules cannot be hidden by a blended score; recommended work is task-tree-owned and remains human-reviewable`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `SPEC-TO-INTENT-ALIGNMENT.0` | `done` | assessment, owner direction, and controller design are durable and verified |
| 2 | `SPEC-TO-INTENT-ALIGNMENT.1` | `done` | six category-aware source-to-IntentIR contracts and strict honesty floors are durable |
| 3 | `SPEC-TO-INTENT-ALIGNMENT.2` | `done` | every production command has guarded integrated/scheduled/omitted accounting |
| 4 | `SPEC-TO-INTENT-ALIGNMENT.3` | `done` | typed timing observations now cross a grounded, verified retained-PDF vertical path |
| 5 | `SPEC-TO-INTENT-ALIGNMENT.4a` | `done` | strict portable evaluator and controlled-fault adequacy are verified; no category claim made |
| 6 | `SPEC-TO-INTENT-ALIGNMENT.4b` | `pending` | lock two portable reviewed verticals per category without tuning extraction |
| 7 | `SPEC-TO-INTENT-ALIGNMENT.4c` | `pending` | publish exact stage losses and the evidence-ranked blocker |
| 8 | `SPEC-TO-INTENT-ALIGNMENT.5` | `pending` | convert measured outcomes into automatic, reviewable steering |

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

## Open Questions

- Which representative PDFs form the first held-out category set for `.1` and `.4`?
- Which currently omitted capability produces the largest held-out source-to-IntentIR gain and should therefore
  be integrated first? `.4` supplies the evidence; `.2` makes each omission measurable in the meantime.

## Blockers

- None for `.4b`: repository and necessary external read-only source identities are available. Live-provider
  absence remains measured input availability, not permission to substitute synthetic output or claim support.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | `.0` ownership | pre-change `git status --short --branch`; read `MEMORY.md`, task-tree doctrine, commit workflow, Knowledge Map routes, roadmap, current status, book contracts, and the load-bearing code-path evidence established during ramp-up | repository clean at opening; task ownership established before durable assessment edits |
| `2026-08-11` | `.0` existing-instrument audit | `INTENT-COMPLETENESS-RESEARCH`, capture–recapture tasks/cards, exact convergence snapshots, category completeness, source-region and closure designs | controller can compose substantial shipped measurement; missing layer is objective history, cross-instrument state classification, capability participation, held-out breadth, and task ranking |
| `2026-08-11` | `.0` literature grounding | Basili/Weiss GQM; Bifet/Gavaldà ADWIN; NIST CUSUM; Chen/Cheung/Yiu metamorphic testing; Jia/Harman mutation testing; Pnueli/Siegel/Singerman translation validation; Kephart/Chess autonomic control; Li/Chen/Yao Pareto evaluation | techniques mapped to objective traceability, drift detection, evaluator strength, per-run conservation, closed-loop planning, and non-scalar trade-off handling in `docs/research/specforge-trajectory-control.md` |
| `2026-08-11` | `.0` focused documentation and projection gates | `mdbook test docs/book`; `mdbook build docs/book`; Knowledge Map derive/check; fact-card catalog check at 199 cards; task-tree catalog check at 138 trees; canonical collection catalog check at five indexes / 254 Markdown members; roadmap projection; book-current-truth; live-document-size | all pass; generated projections agree with their canonical inputs |
| `2026-08-11` | `.0` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, clippy with warnings denied, 1,810 tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-11` | `.1` ownership and source audit | complete startup read of roadmap, codebase, and mdBook; Knowledge Map routing; audit of category classification, completeness instrumentation, and SourceIR/EvidenceIR/SemanticIR/IntentIR carriers | all changes owned before editing; six category contracts are grounded in current typed surfaces and explicitly mark the wire/register message-field and platform-topology carrier gaps incomplete |
| `2026-08-11` | `.1` contract semantics | JSON parse plus schema assertions over `doctrine/spec_to_intent_category_contract.json` | PASS: exactly six categories; each declares required modalities, artifact families, and typed outcomes; reviewed precision/recall are 1.0; fabrication, unexplained-drop, and silent-conflict-loss floors are zero |
| `2026-08-11` | `.1` focused implementation checks | `mdbook test docs/book`; `mdbook build docs/book`; Knowledge Map derive/check; fact-card catalog check at 200 cards; `cargo test -p specforge --lib -- completeness` | PASS: book and projections current; 66 completeness tests passed / zero failed |
| `2026-08-11` | `.1` startup-artifact hygiene | remove the exact repo-local `generated/{source_ir,evidence_ir,semantic_ir,intent_ir,adapters/isf}/readme` smoke chain, prove no `readme` residue, then run `bash scripts/check_corpus_frontier.sh` | PASS: only the reproducible startup smoke output was removed; corpus identity restored to 57 cohort = 52 refreshed + five remaining |
| `2026-08-11` | `.1` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, clippy with warnings denied, 1,810 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and post-producer project-data locality |
| `2026-08-11` | `.2` capability audit | compare all clap subcommands and dispatch paths with `converge`, rescan execution, post-stability promotion, NLI measurement, IntentIR construction, and every standalone producer | 28 commands partition into 16 production commands plus orchestrator/quality/corpus/diagnostic/maintenance roles; 17 capability rows are required because ordinary IntentIR and NLI demotion are distinct modes |
| `2026-08-11` | `.2` focused code checks | warning-deny `cargo clippy -p specforge --all-targets`; `cargo test -p specforge --lib commands::converge` | PASS: 28/28 focused tests; CLI partition, production coverage, unique IDs, provider-free omissions, promotion coverage, and JSON serialization are pinned |
| `2026-08-11` | `.2` provider-free CLI and hygiene | `cargo run -p specforge -- converge README.md --target isf --vlm-provider skip --nlp-provider skip --max-iterations 3`; remove exact five-stage `generated/**/readme` outputs; `bash scripts/check_corpus_frontier.sh` | PASS: stable after two passes; all 17 JSON rows emitted; scratch residue empty; corpus restored to 57 = 52 + five |
| `2026-08-11` | `.2` documentation and projections | mdBook test/build; book-current-truth; Knowledge Map derive/check at 211 facts / 1,568 questions; fact-card catalog at 200 cards; roadmap projection; live-size; rolling-ledger protocol | PASS: code, public contract, retrieval, roadmap, and bounded live surfaces agree |
| `2026-08-11` | `.2` required CHANGES rollover | dry-run then apply `docs/research/spec-to-intent-alignment-2-changes-rollover-plan.jsonl` | PASS: exact `changes-0007` seals 15 records / 302 lines / 27,085 bytes at SHA-256 `609abef2…d9d2f`; live root is warning-safe and all four archive chains validate |
| `2026-08-11` | `.2` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, clippy with warnings denied, 1,813 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-11` | `.3` pre-change producer audit | Knowledge Map reverify plus source/evidence/semantic/waveform call graph and corpus census | region crops already reach `enrich` and its JSON note reaches typed visual observations, but zero persisted VLM notes exist; `FigureRegion` has no producer or IR field and appears only in synthetic adapter tests; the retained I2S PDF supplies a real timing asset for reviewed-fixture proof while live VLM availability remains honestly absent |
| `2026-08-11` | `.3` reviewed retained-PDF vertical fixture | `cargo test -p specforge --lib reviewed_i2s_pdf_figure_reaches_verified_intent_contract`; retained NXP UM11732 PDF size/header and fixture identity; persisted SourceIR → EvidenceIR → SemanticIR → IntentIR | PASS: explicit `WS` samples produce a typed region and verified figure contract; `INVENTED_BY_MODEL` remains visible in EvidenceIR but is absent from semantic/intent contracts; fixture declares `reviewed_fixture_no_live_vlm` |
| `2026-08-11` | `.3` typed-producer and honesty tests | `cargo test -p specforge --lib ir::figure_region`; `cargo test -p specforge --lib ir::waveform`; `cargo test -p specforge --lib validate_evidence_ir_counts_available_and_unavailable_typed_regions`; warning-deny `cargo clippy -p specforge --lib` | PASS: 14 region tests, 17 waveform tests, validation reports 2 raw timing observations / 1 available region / 1 unavailable region, and Clippy emits no warnings |
| `2026-08-11` | `.3` retrieval and public-contract checks | Knowledge Map derive/check; mdBook test/build; book-current-truth and projection/catalog checks | PASS: typed producer, limits, real-fixture status, and next measurement frontier agree across code, book, roadmap, analysis, and retrieval |
| `2026-08-11` | `.3` required Rust-analysis rollover | dry-run then root-last apply `docs/research/spec-to-intent-alignment-3-rust-analysis-rollover-plan.jsonl` | PASS: exact `rust-codebase-analysis-0003` seals 14 records / 203 lines / 17,582 bytes at SHA-256 `12fbadf1…631f`; the current `.3` prepend survives and all four archive chains validate warning-safe |
| `2026-08-11` | `.3` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,820 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-11` | `.4a` evaluator/schema boundary | version-1 public JSON schema; Serde round trip and strict unknown-field behavior; repository-relative/external-portable identity validation; data-defined query and exact multiset scoring audit | PASS: one generic runtime module owns no document/vendor/protocol/layout cases; all four original stage identities and selection boundary are pinned; incomplete review authority stays `unmeasurable` |
| `2026-08-11` | `.4a` controlled-fault adequacy | `cargo test -p specforge --lib ir::source_to_intent_eval`; schema parse plus semantic assertions; `cargo fmt --all -- --check`; `cargo clippy -p specforge --lib -- -D warnings` | PASS: 12/12 focused tests; omission, fabrication, provenance loss, silent stage drop, missing modality, and inactionable residual each move the intended hard result; stable serialization, portability rejection, and root-derived loading pass |
| `2026-08-11` | `.4a` public/retrieval/live alignment | mdBook test/build; book-current-truth; Knowledge Map derive/check at 213 facts / 1,586 questions; fact-card catalog at 202 cards; live-size and rolling-ledger protocols | PASS: schema, API, public semantics, roadmap, current status, architecture, retrieval, and bounded-size authorities agree that this is evaluator adequacy, not category support |
| `2026-08-11` | `.4a` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,832 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SPEC-TO-INTENT-ALIGNMENT.0` | `SPEC-TO-INTENT-ALIGNMENT.0 — fix the upstream objective and trajectory control` | durable trajectory and priority alignment; no product-code or generated-IR mutation |
| `SPEC-TO-INTENT-ALIGNMENT.1` | `SPEC-TO-INTENT-ALIGNMENT.1 — define source-to-IntentIR category contracts` | versioned category contract, book contract, and retrieval fact; no product-code or generated-IR mutation |
| `SPEC-TO-INTENT-ALIGNMENT.2` | `SPEC-TO-INTENT-ALIGNMENT.2 — guard canonical capability accounting` | 17-row per-run ledger, full CLI partition guard, public contract, and required lossless CHANGES rollover |
| `SPEC-TO-INTENT-ALIGNMENT.3` | `SPEC-TO-INTENT-ALIGNMENT.3 — activate typed figure-region mining` | optional EvidenceIR region producer, source grounding, verifier gate, explicit availability metrics, reviewed retained-PDF vertical fixture, and required Rust-analysis rollover |
| `SPEC-TO-INTENT-ALIGNMENT.4a` | `SPEC-TO-INTENT-ALIGNMENT.4a — build the vertical evaluation engine` | strict portable dataset/API, exact stage-loss rollups, and six controlled-fault classes; no held-out category claim |

## Changelog

- `2026-08-11`: Created on owner request so the ramp-up trajectory assessment and upstream-first direction do
  not remain chat-only.
- `2026-08-11`: Added the automatic-steering design request as `.5`; ADR 0033/0034 and the research design own
  the upstream-first sequence and reviewable multi-metric controller.
- `2026-08-11`: Closed `.0` after focused projection checks and the full repository gate; frontier advances
  to `.1`, the per-document-category EvidenceIR-to-IntentIR content contract.
- `2026-08-11`: Closed `.1` with strict, source-grounded acceptance for all six categories; frontier advances
  to `.2`, canonical production-workflow capability accounting.
- `2026-08-11`: Closed `.2` with complete guarded capability accounting and explicit current omissions;
  frontier advances to `.3`, real PDF-to-typed-multimodal producer proof.
- `2026-08-11`: Closed `.3` with a typed timing-observation producer, source-grounded and verifier-gated
  contract mining, explicit availability accounting, and reviewed real-PDF vertical proof; frontier advances to
  `.4`, held-out source-to-IntentIR evaluation and stage-loss accounting.
- `2026-08-11`: Closed `.4a` with the strict portable dataset contract, deterministic exact evaluator, and six
  controlled-fault classes; frontier advances to `.4b`, lock two reviewed documents per category without tuning
  extraction before `.4c` publishes product conclusions.
