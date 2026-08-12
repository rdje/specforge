# SPEC-TO-INTENT-ALIGNMENT: steer PDF evidence into complete executable-intent IR

## Metadata

- Tree ID: `SPEC-TO-INTENT-ALIGNMENT`
- Status: `active`
- Roadmap lane: extraction quality and breadth
- Created: `2026-08-11`
- Last updated: `2026-08-12`
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

- ID: `SPEC-TO-INTENT-ALIGNMENT.6`
  Status: `in_progress`
  Goal: `qualify current-binary truth before eliminating the fabrication and provenance defects that still reproduce`
  Acceptance: `hash-pinned isolated replay separates the retrospective baseline from current product truth across all 12 reviewed documents; a reversible upstream repair then reduces the exact current fabricated-fact and unprovenanced-record populations without sacrificing a source-grounded true positive; controlled fabrication, provenance, and artifact-currency mutants remain detected; a comparable snapshot records the result`
  Verification: `pending`
  Commit: `pending`
  Children: `.6a`, `.6b`, `.6c`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6a`
  Status: `done`
  Goal: `replay the largest frozen fabrication family with the current binary and separate historical baseline truth from current-product truth`
  Acceptance: `a repository-local isolated four-stage replay from the hash-equal SSD-local source copy proves whether the current timing-authority gate removes all 19 table-of-contents fabrications; no canonical generated artifact or frozen .4c result changes; the replay identity and exact before/after counts are durable; public/controller language cannot imply that an unreplayed pinned stage is current-binary output`
  Verification: `hash-equal 827,669-byte SSD source copy; isolated four-stage replay at production revision c4331a51; table_0004 retains 20x2 TOC shape but kind timing_parameter→unknown and promoted timing records 19→0 at EvidenceIR/SemanticIR/IntentIR; expected TP 0→0; strict replay-evidence validation and currency/preservation mutants; scratch copy and both exact output roots removed with residue absent; controller v2 input/report SHA-256 58dd10e5…33bc / 5bec293b…3238; full CI passes all eight doctrines, formatting, warning-deny Clippy, 1,857 tests / five ignored / zero failed, Rust docs, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6a — separate replay truth from the frozen baseline`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6b`
  Status: `done`
  Goal: `complete current-binary replay coverage, then rank and repair the remaining fabrication and provenance families`
  Acceptance: `all 12 reviewed documents have hash-pinned comparable replay evidence; that inventory, not the retrospective .4c baseline alone, identifies the next bounded production defect; the repair removes false canonical facts or closes genuine provenance without sacrificing a reviewed true positive`
  Verification: `all 12 current replays qualified; current Arm carrier family repaired and clean-replayed; resource-risk defect surfaced by replay repaired with exact live fidelity; child full gates green`
  Commit: `completed by .6b.i, .6b.ii.a, .6b.ii.b, and .6b.iii child commits`
  Children: `.6b.i`, `.6b.ii`, `.6b.iii`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6b.i`
  Status: `done`
  Goal: `replay and evaluate the complete twelve-document reviewed population with the current binary before selecting a repair`
  Acceptance: `all source bytes are hash-equal to the frozen portable identities; every four-stage replay is isolated below repository-volume scratch; a generic current-result projection evaluates the same reviewed cells and publishes exact per-document/stage outcomes without mutating the frozen dataset or canonical generated artifacts; all scratch copies and outputs are removed after durable evidence is captured`
  Verification: `all 12 frozen source identities reverified; 4 repository and 8 necessary read-only external sources reside on the SSD volume; 48 current-binary stage artifacts hash-pinned; exact current result TP/FP/FN 7/22/33, source capture 14/14, evidence capture 13/14, provenance closure 3/29, stage conservation 21/54, residual actionability 0/24; frozen-to-current AIA TOC delta removes exactly 19 fabrications/provenance failures; all 4,313 scratch files / 1,194,976 KiB removed with residue absent; full CI passes all eight doctrines, formatting, warning-deny Clippy, 1,861 tests / five ignored / zero failed, warning-deny rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6b.i — qualify the complete reviewed population`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6b.ii`
  Status: `done`
  Goal: `rank the qualified current defect families and repair the highest bounded honesty or provenance defect`
  Acceptance: `the selected family reproduces in .6b.i current evidence; the repair is generic, reversible, source-grounded, and measured before→after without sacrificing any reviewed true positive; a comparable trajectory snapshot records the result`
  Verification: `generic carrier proof and four-chain reconciliation in .6b.ii.a; clean committed-revision replay over 12/12 sources and 48/48 stages in .6b.ii.b; exact aggregate TP/FP/FN 7/22/33→19/10/21, provenance 3/29→17/29, conservation 21/54→57/78; all seven prior true positives retained and twelve added; controlled mutants and full CI green`
  Commit: `completed by .6b.ii.a and .6b.ii.b child commits`
  Children: `.6b.ii.a`, `.6b.ii.b`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a`
  Status: `done`
  Goal: `preserve register-level access and structured-table provenance through the canonical register carrier`
  Acceptance: `RegisterRecord represents register-level access separately from field access and carries direct table provenance; the row-per-register extractor populates both from generic header structure; SemanticIR and IntentIR preserve the values unchanged; the four retained corpus chains changed by the shared carrier are rebuilt and attributed under ADR 0025; focused and full gates pass without changing frozen or replay-result authorities`
  Verification: `source/table and carrier audit; backward-compatible Serde and generic extractor/merge tests; exact 12-register Arm probe through all three canonical stages; ADR 0025 reconciliation for the four changed retained chains with non-carrier equality and byte-identical adapters; EvidenceIR currency 24/24 and downstream currency 78/78; all eight doctrines, formatting, warning-deny Clippy, 1,862 tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a — preserve register access and table provenance`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b`
  Status: `done`
  Goal: `replay the clean access-carrier repair across the complete reviewed population and publish the exact comparable result`
  Acceptance: `all 12 sources and 48 stages are replayed at the committed .6b.ii.a production revision; the exact before→after result proves the Arm Debug family disposition and every collateral change; controller, public/live documentation, retrieval, and cleanup evidence agree`
  Verification: `12/12 unchanged source hashes and 48/48 isolated stages at bb152dfb; current result SHA-256 9e8cd99b…6b1; Arm Debug 0/12/12/12→12/0/0/0 TP/FP/FN/unprovenanced; AMD IOMMU and GIC-400 each provenance-only improvement; no other cell delta; controller v4 input/report SHA-256 0fb9e6e4…8cb / 3d82b0fb…9e2 selects .6c; exact 3,913-file / 1,090,884-KiB root and map absent; all eight doctrines, formatting, warning-deny Clippy, 1,862 tests passed / five ignored / zero failed, rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b — qualify the access-carrier repair`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6b.iii`
  Status: `done`
  Goal: `make bounded ingestion engage from measured resource risk rather than the disproved above-500-page assumption`
  Acceptance: `the 400-page Arm Debug source no longer enters a single-pass path that the host terminates; generic policy uses measured resource/shape authority rather than a document name; single-pass and bounded fidelity, typed memory-abort reporting, corpus currency, and full CI remain green`
  Verification: `resource/override/helper/signal and public lifecycle tests; override-free live 400-page replay at threshold 131 / batch 64 with exact path-normalized four-stage fidelity; exact 795-file / 184,164-KiB cleanup; all eight doctrines, formatting, warning-deny Clippy, 1,866 tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6b.iii — select bounded ingest from resource risk`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6c`
  Status: `in_progress`
  Goal: `repair the largest remaining current honesty and provenance family without sacrificing the 19 reviewed true positives`
  Acceptance: `the I2S receiver-timing family preserves its source-grounded ns unit across EvidenceIR, SemanticIR, and IntentIR so the exact five false positives, five false negatives, and five unprovenanced records close; no document-specific production branch is introduced; a complete comparable replay proves the result and re-ranks the remaining current gaps`
  Verification: `pending`
  Commit: `pending`
  Children: `.6c.i`, `.6c.ii`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6c.i`
  Status: `done`
  Goal: `preserve explicit table-wide timing units and direct table provenance through the canonical timing carrier`
  Acceptance: `the shared timing-table producer recognizes only an explicit closed caption-unit grammar, records direct table authority without misusing statement provenance, and the affected retained corpus chains are reconciled under ADR 0025; the five I2S facts are exact through IntentIR and focused/full gates pass`
  Verification: `closed caption grammar, explicit-row precedence, legacy Serde, scalar/variant authority tests; exact five-fact real I2S three-stage proof; ADR 0025 reconciliation of five retained chains with 231/231 direct table supports, carrier-neutralized 15/15 stage equality, byte-identical adapters, EvidenceIR 24/24 and downstream 78/78 currency; exact scratch cleanup; all eight doctrines, formatting, warning-deny Clippy, 1,866 tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6c.i — preserve timing unit and table provenance`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6c.ii`
  Status: `pending`
  Goal: `replay the timing-unit/provenance repair across the complete reviewed population and publish the exact comparable result`
  Acceptance: `all 12 sources and 48 stages replay at the committed .6c.i production revision; exact I2S and collateral provenance deltas are published; all 19 prior true positives survive; controller, public/live documentation, retrieval, and cleanup evidence agree`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.7`
  Status: `pending`
  Goal: `recover source-grounded canonical facts lost at the first failing source-to-evidence boundary`
  Acceptance: `the frozen vertical oracle identifies and repairs a bounded high-impact SourceIR-to-EvidenceIR loss family; stage conservation and held-out recall improve without fabrication, provenance, or category regression; a comparable snapshot records the result`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8`
  Status: `pending`
  Goal: `make required promotion-loss residuals typed, source-linked, and actionable`
  Acceptance: `a bounded residual family gains exact source linkage, typed cause, and operator action at SemanticIR and IntentIR; the vertical evaluator records the disposition without relabeling missing canonical facts as residual success`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.9`
  Status: `pending`
  Goal: `resolve the highest-value omitted canonical production capability after upstream honesty and conservation gaps`
  Acceptance: `the .2 ledger and a reviewed vertical establish which omitted island has measured value; the selected capability is integrated or explicitly scheduled with per-run accounting and no implied participation; a comparable snapshot records the outcome`
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
| 6 | `SPEC-TO-INTENT-ALIGNMENT.4b` | `done` | 12 portable reviewed verticals and 14 complete cells are frozen without tuning extraction |
| 7 | `SPEC-TO-INTENT-ALIGNMENT.4c` | `done` | exact result localizes the constraint to source-to-evidence fact formation and residualization |
| 8 | `SPEC-TO-INTENT-ALIGNMENT.5a` | `done` | strict multi-metric state/ranking engine and mutation controls are verified |
| 9 | `SPEC-TO-INTENT-ALIGNMENT.5b` | `done` | first retrospective snapshot is byte-current and selects the .6 honesty lane |
| 10 | `SPEC-TO-INTENT-ALIGNMENT.6a` | `done` | dominant frozen TOC defect replayed; 19 fabrications disappear and controller now gates 1/12 replay currency |
| 11 | `SPEC-TO-INTENT-ALIGNMENT.6b.i` | `done` | all 12 current-binary replays are hash-pinned; 19 historical TOC fabrications are retired without changing frozen or canonical artifacts |
| 12 | `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a` | `done` | generic carrier, exact retained-corpus reconciliation, and full CI are complete without changing frozen/current result authorities |
| 13 | `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b` | `done` | 12/12 replay proves exact Arm closure, two provenance-only collateral changes, and no other cell delta |
| 14 | `SPEC-TO-INTENT-ALIGNMENT.6b.iii` | `done` | resource-sized activation, fail-closed page counting, typed signal termination, exact live 400-page fidelity, cleanup, and full CI complete |
| 15 | `SPEC-TO-INTENT-ALIGNMENT.6c.i` | `done` | generic caption-unit/table-provenance carrier, five-chain reconciliation, cleanup, and full CI complete |
| 16 | `SPEC-TO-INTENT-ALIGNMENT.6c.ii` | `pending` | clean whole-population replay, exact result publication, and controller re-ranking |
| 17 | `SPEC-TO-INTENT-ALIGNMENT.7` | `pending` | recover source-to-evidence losses after the honesty floor is restored |
| 18 | `SPEC-TO-INTENT-ALIGNMENT.8` | `pending` | make required residuals actionable after canonical loss is bounded |
| 19 | `SPEC-TO-INTENT-ALIGNMENT.9` | `pending` | integrate measured capability breadth only after higher-ranked semantic gaps |

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

## Open Questions

- Which currently omitted capability produces the largest held-out source-to-IntentIR gain and should therefore
  be integrated first? `.4` supplies the first frozen evidence; `.5` must rank it against the `.2` capability
  ledger without hiding hard failures in one score.

## Blockers

- None for `.5b`: the committed generic engine, exact reviewed result, category contract, stage-loss split,
  capability ledger, and automatic-steering design are available. Semantic mutation remains review-gated.

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
| `2026-08-11` | `.4b` selection and review lock | source and four-stage SHA-256 pinning at the `.4a` selection boundary; direct review of 12 bounded regions including two platform figures | PASS: exactly two documents in each of six categories, 14 complete gold cells, 4 repository / 8 necessary external sources, and prose/table/figure coverage at 3/7/2 documents; no extractor was tuned and the population is explicitly retrospective rather than historically unseen |
| `2026-08-11` | `.4b` SSD locality repair | whole-workspace retired-prefix census; exact target existence/hash checks; generated path repair; `check_persisted_artifact_paths.pl`; project-data locality | PASS: 44 stale fields in 38 generated records moved to the verified same-volume SSD library; all 104 authorized external absolute fields resolve, zero repository-owned absolute paths and zero obsolete livework references remain; three selected SourceIR metadata hashes changed and every downstream stage hash stayed fixed |
| `2026-08-11` | `.4b` frontier authority correction | `perl scripts/check_corpus_frontier_census.pl --self-test`; real corpus check/report | PASS: 13/13 lifecycle-partition mutations and real 57 cohort = 52 refreshed + five remaining; refreshed/remaining state no longer depends on a workstation path |
| `2026-08-11` | `.4b` focused population/public checks | builder `--check`; `cargo test -p specforge --lib ir::source_to_intent_eval`; warning-deny Clippy; mdBook test/build; Knowledge Map at 215 facts / 1,597 questions; fact catalog at 204 cards; book/currentness/live-size/rolling-ledger checks | PASS: builder byte-stable; 13/13 evaluator tests; balanced/complete/portable/measurable population; no result snapshot or category conclusion published before `.4c` |
| `2026-08-11` | `.4b` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,833 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-11` | `.4c` persisted result | generic `cargo run --example source_to_intent_eval` over the frozen dataset, byte comparison, SHA-256/size census, and exact JSON aggregation | PASS: 104,669 bytes at `6b72f1fc…47eb`; all six categories incomplete; canonical TP/FP/FN = 7/41/33; 10 cells first fail SourceIR → EvidenceIR, four EvidenceIR → SemanticIR, none later |
| `2026-08-11` | `.4c` focused code/public checks | 14 evaluator tests; byte regeneration; format check; all-target warning-deny Clippy; mdBook test/build; Knowledge Map 216 facts / 1,604 questions; fact catalog 205 cards; book/currentness/roadmap/live-size/rolling-ledger checks | PASS: exact snapshot and every public/durable conclusion agree; registered warning bands remain below rollover triggers |
| `2026-08-11` | `.4c` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,834 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-12` | `.5a` controller semantics | `cargo test -p specforge --lib ir::trajectory`; JSON schema parse/semantic assertions; format check; all-target warning-deny Clippy | PASS: 16/16 tests cover five states, hard-failure precedence, dimension/fabrication/authority/path/owner/unknown-field mutants, estimated-history refusal, hard-first ranking, and deterministic serialization |
| `2026-08-12` | `.5a` public/retrieval/live alignment | mdBook test/build; Knowledge Map 217 facts / 1,611 questions; fact catalog 206 cards; book-current-truth; roadmap projection; active-task evidence; live-size | PASS: schema, engine, research, book, roadmap, status, architecture, and retrieval agree; warning bands stay below mandatory rollover |
| `2026-08-12` | `.5a` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,850 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-12` | `.5b` composition/currentness | three trajectory-snapshot tests; live provider-free ledger equality; generic runner byte comparison; regeneration `--check`; format; all-target warning-deny Clippy | PASS: exact `.4c` and `.2` counts produce `diverging` + `insufficient_history`; all four ranked gaps are owned and `.6` wins hard-first; input/report SHA-256 `9901b420…a654` / `2fef8189…0a34` |
| `2026-08-12` | `.5b` public/retrieval/live alignment | mdBook test/build; Knowledge Map 218 facts / 1,618 questions; fact catalog 207 cards; book-current-truth; roadmap projection; live-size | PASS: report, book, roadmap, research, architecture, live state, and retrieval agree; 775 Markdown files satisfy 52 governed surfaces without a mandatory rollover |
| `2026-08-12` | `.5b` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,854 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-12` | `.6a` isolated replay | source SHA/size equality; four replay-stage hashes; exact `table_0004` inspection; baseline/replay constraint counts; scratch census | PASS: source `2d359579…4c7c8` / 827,669 bytes; TOC stays 20×2, kind `timing_parameter`→`unknown`; EvidenceIR/SemanticIR/IntentIR timing records each 19→0; expected TP 0→0; three exact scratch roots removed and absent |
| `2026-08-12` | `.6a` controller authority | strict replay-evidence loader; cleanup/currency and surviving-fabrication mutants; snapshot regeneration and generic check | PASS: current-binary coverage is a hard 1/12 metric/gate; historical 41/45/33 counts no longer masquerade as current gates; `.6` remains the owned recommendation; input/report SHA-256 `58dd10e5…33bc` / `5bec293b…3238` |
| `2026-08-12` | `.6a` focused code/public checks | two replay path guards; four composition/currentness/mutant tests; all-target warning-deny Clippy; format; mdBook test/build; trajectory `--check` | PASS: replay rejects unsafe/external paths before output creation; evidence and artifacts are byte-current; public contract builds and tests cleanly |
| `2026-08-12` | `.6a` mandatory live-ledger containment | exact two-row plan dry run then root-last apply; protocol and live-size checks | PASS: dry run exact/warning-safe; 19 Development Notes records sealed at `17f28fd7…0eea`, 13 status records at `6dcff897…ef84`; both roots warning-safe; prior archive members unchanged |
| `2026-08-12` | `.6a` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,857 Rust tests passed / five ignored / zero failed, Rust docs, mdBook test/build, and final project-data-locality residue check |
| `2026-08-12` | `.6b.i` source authority and locality | SHA-256/byte comparison for 12 reviewed sources; external-map exact coverage; APFS device comparison; copy/verify/use/delete manifest | PASS: all reviewed identities match; four repository plus eight necessary read-only external inputs are on the SSD volume; no boot-volume livework reference exists; every external copy is hash-equal before use |
| `2026-08-12` | `.6b.i` complete isolated replay | generic population orchestrator over fresh `.project-data/tmp/spec-to-intent-6bi-population-r2`; 12 production replay executions; 48 stage hash/byte captures; current-result projection and evaluator | PASS: 12/12 documents and 48/48 stages captured; current dataset/result SHA-256 `c0490133…e5e` / `a8ee9dfd…ec7`; TP/FP/FN 7/22/33; source/evidence capture 14/14 and 13/14; provenance 3/29; conservation 21/54; residual actionability 0/24 |
| `2026-08-12` | `.6b.i` root cause and repair selection | exact frozen/current document-family comparison | PASS: AIA TOC accounts for the complete 41→22 fabrication and 45→26 provenance-failure improvement; 33 false negatives persist; current Arm Debug access-mode loss is the largest bounded honesty/provenance family at 12 FP + 12 FN + 12 provenance failures and owns `.6b.ii` |
| `2026-08-12` | `.6b.i` strict replay/controller checks | population-manifest coverage/hash/cleanup mutants; current-result population/fabrication mutants; warning-deny all-target check; six trajectory-snapshot tests; snapshot write/check | PASS: strict loaders reject every mutant; controller v3 gates 12/12 currency plus 22 fabrication and 26 provenance failures; input/report SHA-256 `5216f137…c4f` / `41db802b…942`; frozen dataset/result and canonical `generated/` artifacts remain byte-unchanged |
| `2026-08-12` | `.6b.i` scratch lifecycle | exact post-promotion removal and residue census for feasibility, failed, and authoritative roots plus external source map | PASS: authoritative 4,313 files / 1,194,976 KiB removed; all four exact scratch/map paths absent; portable manifest records the successful cleanup without retaining machine-specific external paths |
| `2026-08-12` | `.6b.i` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,861 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6b.ii.a` root cause and focused carrier proof | SourceIR `table_0044` audit; `register_map_preserves_register_access_and_table_provenance`; six fragment-consolidation tests; legacy/default Serde checks; bounded Arm replay | PASS: the extractor previously parsed then discarded row-level access because `RegisterRecord` lacked the carrier; all 12 reviewed names now retain exact access plus `table_0044` through EvidenceIR/SemanticIR/IntentIR; absent access remains absent; 1,195-file / 288,160-KiB probe removed |
| `2026-08-12` | `.6b.ii.a` ADR 0025 corpus reconciliation | exact four-chain backup; carrier-stripped semantic equality; four cascades rebuilt/validated; `scripts/check_chain_currency.sh` | PASS: register counts hold 45/1/6/1; Arm access/table support 16/29, other table support 1/1, 6/6, 1/1 with zero invented access; every non-carrier value equal, adapters byte-identical; EvidenceIR 24/24 and downstream 78/78 current; exact 31-file / 32,680-KiB rollback root removed |
| `2026-08-12` | `.6b.ii.a` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,862 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6b.ii.b` clean population replay | command-recorded threshold 399; 12 source hash checks; 48 stage identities; comparable fixture projection; exact cell diff | PASS: Arm Debug 0/12/12/12→12/0/0/0 TP/FP/FN/unprovenanced; AMD IOMMU and GIC-400 each close one provenance failure with semantics unchanged; no other cell changes; aggregate TP/FP/FN 19/10/21, provenance 17/29, conservation 57/78 |
| `2026-08-12` | `.6b.ii.b` controller, mutants, and cleanup | strict manifest/result loaders; coverage/hash/cleanup/fabrication/population mutants; snapshot write/check; generic controller byte comparison; exact cleanup census | PASS: controller v4 gates 12/12 currency plus ten fabrications / twelve provenance failures and selects `.6c`; manifest/result/input/report SHA-256 `06fb3dea…2f5` / `9e8cd99b…6b1` / `0fb9e6e4…8cb` / `3d82b0fb…9e2`; 3,913 files / 1,090,884 KiB and runtime map removed and absent |
| `2026-08-12` | `.6b.ii.b` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,862 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6b.iii` root cause and focused policy/error proof | resource-threshold/override/helper-contract tests; Unix signal/ordinary-exit distinction; two public materialization lifecycle tests; embedded Python compile; warning-deny all-target Clippy | PASS: Rust/Python split default and signal-collapsing exit path localized; 24-GiB/64-GiB/96-GiB thresholds 131/349/399; explicit 512/0 exact; unreadable count fail-closed; signal retains typed status and prior bundle; backend 37/37, lifecycle 2/2 |
| `2026-08-12` | `.6b.iii` override-free live fidelity and cleanup | real 400-page Arm replay with no threshold override; backend metadata; six SourceIR identity comparisons; validation-neutral three-stage comparisons; exact census/removal | PASS: default selected threshold 131 / batch 64; 400 pages / 210 tables / 176 figures / 6,784 elements / 1,321 sections; all SourceIR and downstream identities exact after only path/validation neutralization; 795 files / 184,164 KiB removed and exact root absent |
| `2026-08-12` | `.6b.iii` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,866 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6c.i` focused carrier proof | `cargo test -p specforge --lib timing_table_`; retained I2S SourceIR replay and exact three-stage inspection | PASS: closed caption grammar accepts explicit timing declarations and rejects misleading mentions; row units override captions; legacy Serde loads; exact five I2S rows retain `ns` plus `table_0004` through IntentIR |
| `2026-08-12` | `.6c.i` ADR 0025 corpus reconciliation | pre/post field-level comparison; five cascades rebuilt/validated; `scripts/check_chain_currency.sh`; exact scratch census | PASS: 55/60/37/5/74 timing counts hold; 231/231 records at each stage cite one existing SourceIR table; only five I2S units change; 15/15 carrier-neutralized stages and 5/5 adapters equal; EvidenceIR 24/24 and downstream 78/78 current; 89-file reconciliation root and 64-file probe removed and absent |
| `2026-08-12` | `.6c.i` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,866 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |

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

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6a`

- [x] **REPRODUCE / MEASURE** — copy the caller-authorized AIA PDF into repository-local SSD scratch, prove
  827,669-byte / SHA-256 equality, run SourceIR → EvidenceIR → SemanticIR → IntentIR below a fresh
  `.project-data/tmp` root, and record all four baseline/replay hashes plus exact table and promotion counts.
- [x] **ROOT CAUSE (WHY + WHERE)** — the frozen SourceIR persisted reviewed TOC `table_0004` as
  `timing_parameter`, producing 19 unprovenanced false timing records at every later stage. Current runtime
  normalization applies pre-selection commit `46af2eca` and classifies the unchanged 20×2 TOC shape `unknown`;
  `.4c` remained stale because its reclaimed normalized input had never been replayed.
- [x] **ADDRESSED (verified)** — the isolated replay produces zero timing records at EvidenceIR, SemanticIR, and
  IntentIR, with expected true positives holding 0→0. Controller v2 makes replay coverage a hard 1/12 metric and
  11-document gate, treats 41/45/33 as retrospective baseline measures, and keeps `.6` as the owned next action.
- [x] **NO REGRESSION** — canonical `generated/` artifacts and the frozen `.4c` result remain unchanged; strict
  replay-path, evidence-shape, cleanup, currency, surviving-fabrication, and preservation mutants pass; the
  source copy plus two exact replay roots were deleted after use and a residue census proved all three absent.
- [x] **GENERICITY (ADR 0006)** — the replay tool accepts any repository-contained source, fresh project-local
  output root, optional prior memory, and optional observed table id. The AIA document/table truth exists only
  in tracked replay evidence and the composition adapter, never in production extraction behavior.
- [x] **LOCKSTEP** — replay evidence, controller artifacts, task frontier, roadmap, book, research correction,
  live ledgers, architecture analysis, Knowledge Map, and resume pointer distinguish historical baseline truth
  from current product truth and retain repository-relative persisted paths.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6b.i`

- [x] **REPRODUCE / MEASURE** — resolve all eleven unreplayed portable source identities, verify each byte count
  and SHA-256 before use, replay SourceIR → EvidenceIR → SemanticIR → IntentIR in fresh repository-volume roots,
  and evaluate the same 13 reviewed cells plus the committed AIA cell with exact per-stage counts and hashes.
- [x] **ROOT CAUSE (WHY + WHERE)** — compare frozen and current results by document, family, first failing stage,
  fabrication, provenance, conservation, and residual actionability. Separate already-retired historical defects
  from failures that reproduce in the current binary; rank no repair from a frozen count alone.
- [x] **ADDRESSED (verified)** — publish strict portable replay evidence and a deterministic current-result
  projection that covers all 12 documents without changing the frozen reviewed dataset/result or canonical
  `generated/` artifacts. Select `.6b.ii` only from the measured current defect inventory.
- [x] **NO REGRESSION** — kill missing-source, hash/currency, incomplete-population, stage-preservation, frozen-
  authority-mutation, and cleanup mutants; run focused evaluator/replay/controller checks and risk-proportionate
  `cargo test` / `run_ci.sh`; prove every exact scratch copy/output root absent after evidence capture.
- [x] **GENERICITY (ADR 0006)** — orchestration and projection code is data-driven by the reviewed dataset and
  portable source manifest, with no vendor/protocol/document/signal/layout behavior in production extraction.
- [x] **LOCKSTEP** — replay report, current result, controller, task frontier, roadmap, mdBook, live docs,
  architecture analysis, Knowledge Map, and resume pointer publish the same coverage and next measured family.

## Planned closing criteria — `SPEC-TO-INTENT-ALIGNMENT.6b.ii`

- **REPRODUCE / MEASURE:** localize the 12 Arm Debug false-positive/false-negative/provenance failures to
  an exact source, EvidenceIR, SemanticIR, and IntentIR carrier transition using the qualified `.6b.i` evidence.
- **ROOT CAUSE (WHY + WHERE):** prove why correct register names lose reviewed access mode, distinguish
  extraction absence from projection/key-shape loss, and identify the narrowest generic production seam.
- **ADDRESSED (verified):** preserve source-grounded access semantics through the canonical register path
  so the 12 reviewed keys become true positives and the corresponding false positives, false negatives, and
  provenance failures disappear without hand-coding document, vendor, register, or layout knowledge.
- **NO REGRESSION:** retain every prior true positive, current AIA honesty, exact population currency, and
  all frozen authorities; kill access omission/fabrication/provenance mutants and run focused plus full CI gates.
- **GENERICITY (ADR 0006):** any repair is driven by typed source/evidence structure and applies to the
  same access-bearing register shape across documents rather than the Arm Debug fixture alone.
- **LOCKSTEP:** comparable replay/result/controller evidence, task frontier, roadmap, mdBook, live docs,
  architecture analysis, Knowledge Map, and resume pointer publish the exact before→after result and next owner.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a`

- [x] **REPRODUCE / MEASURE** — prove the reviewed table has all 12 access cells while current EvidenceIR has
  12 names, no register-level access field, and no direct table provenance.
- [x] **ROOT CAUSE (WHY + WHERE)** — identify the absent `RegisterRecord` carrier and the register-map branch
  that parses then discards access, while proving both later canonical stages clone the lossy record unchanged.
- [x] **ADDRESSED (verified)** — add separate register-level access and supporting-table fields, populate them
  from generic structured-table evidence, and preserve both through Serde plus SemanticIR/IntentIR cloning.
- [x] **NO REGRESSION** — cover access/provenance presence, access absence, field-access separation, and
  fragment provenance union; reconcile the four exact retained corpus chains moved by the shared carrier; run
  focused schema/extractor tests and full CI without mutating frozen/replay truth.
- [x] **GENERICITY (ADR 0006)** — use only typed header roles and source cells; add no Arm, JTAG, register-name,
  access-token, or layout-specific production branch.
- [x] **LOCKSTEP** — task, live code analysis, mdBook carrier contract, changes, and resume pointer distinguish
  the shipped carrier repair from the still-pending clean-binary population replay owned by `.6b.ii.b`.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b`

- [x] **REPRODUCE / MEASURE** — replay all 12 unchanged source identities and compare all 48 new stage hashes
  plus exact current result against the committed `.6b.i` authority.
- [x] **ROOT CAUSE (WHY + WHERE)** — attribute every metric delta to the generic access/table-provenance carrier
  and surface any unexpected collateral change before closing the parent repair.
- [x] **ADDRESSED (verified)** — publish portable replay/result/controller evidence proving the Arm Debug family
  disposition and the remaining current honesty/provenance inventory.
- [x] **NO REGRESSION** — retain seven prior true positives, AIA honesty, source bytes, frozen authorities, and
  canonical generated artifacts; kill replay/result/controller mutants and remove exact scratch residue.
- [x] **GENERICITY (ADR 0006)** — the replay and projection remain population-driven, and measured improvement
  follows typed production behavior rather than fixture-specific postprocessing.
- [x] **LOCKSTEP** — result, controller, task frontier, roadmap, mdBook, live docs, architecture, Knowledge Map,
  and resume pointer agree on the exact before→after metrics and next owned activity.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6b.iii`

- [x] **REPRODUCE / MEASURE** — turn the twice-reproduced 400-page default-path signal termination and the
  successful bounded replay into deterministic policy/error tests, retaining the exact resource evidence.
- [x] **ROOT CAUSE (WHY + WHERE)** — prove which Rust/Python selection seam sends the 400-page shape to
  single-pass conversion and which subprocess exit path currently hides likely resource termination.
- [x] **ADDRESSED (verified)** — make the default generic policy select bounded conversion for the measured-risk
  shape and return typed actionable diagnostics for memory-guard or signal termination.
- [x] **NO REGRESSION** — preserve small-document single-pass behavior, explicit environment overrides,
  adaptive batch sizing, single-pass/bounded structural fidelity, canonical corpus currency, and locality.
- [x] **GENERICITY (ADR 0006)** — select from document/resource shape and process status only; add no document,
  vendor, protocol, filename, or reviewed-fixture branch.
- [x] **LOCKSTEP** — code, tests, task frontier, roadmap, mdBook, live docs, architecture, and Knowledge Map agree
  on the default threshold/policy, error contract, observed limitation, and next action.

## Planned closing criteria — `SPEC-TO-INTENT-ALIGNMENT.6c`

- **REPRODUCE / MEASURE** — pin the exact five reviewed I2S receiver-timing keys and prove where the `ns`
  unit exists in source/SourceIR and first disappears across EvidenceIR, SemanticIR, and IntentIR.
- **ROOT CAUSE (WHY + WHERE)** — identify the shared typed carrier, parser, merge, or promotion seam that
  discards the source-grounded unit; distinguish extraction loss from evaluator/provenance projection error.
- **ADDRESSED (verified)** — preserve the grounded unit and provenance through all canonical stages so the
  five false positives, five false negatives, and five unprovenanced records close exactly.
- **NO REGRESSION** — retain all 19 current true positives, reject invented/default units, preserve unrelated
  records and retained corpus currency, and kill controlled unit/provenance/fabrication mutants.
- **GENERICITY (ADR 0006)** — infer only from typed source structure or universal unit grammar; introduce no
  document, vendor, protocol, filename, reviewed-key, or page-layout exception.
- **LOCKSTEP** — comparable whole-population replay, controller, task frontier, roadmap, mdBook, live docs,
  architecture, Knowledge Map, cleanup proof, and full CI agree on the exact result and next ranked gap.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6c.i`

- [x] **REPRODUCE / MEASURE** — SourceIR `table_0004` retains five rows plus explicit caption unit; EvidenceIR
  first loses `ns` and direct table authority; SemanticIR/IntentIR clone the loss unchanged.
- [x] **ROOT CAUSE (WHY + WHERE)** — `synthesize_timing_constraints` reads only a dedicated unit column and
  initializes statement provenance empty; `TimingConstraintRecord` lacks direct table provenance.
- [x] **ADDRESSED (verified)** — explicit closed caption-unit grammar and typed table provenance survive all
  canonical stages, with the exact five I2S facts proven on real retained source bytes.
- [x] **NO REGRESSION** — legacy Serde, explicit-unit columns, trapped/body rows, scalar/variant authority, corpus
  currency, and unrelated adapter output remain valid; affected persisted chains are reconciled under ADR 0025.
- [x] **GENERICITY (ADR 0006)** — caption grammar and timing units are universal; no document, vendor, protocol,
  table id, reviewed-key, or page-layout exception exists.
- [x] **LOCKSTEP** — code, focused evidence, task tree, live docs, architecture, mdBook, Knowledge Map, cleanup,
  and full CI agree; clean whole-population result publication remains exclusively `.6c.ii`.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SPEC-TO-INTENT-ALIGNMENT.0` | `SPEC-TO-INTENT-ALIGNMENT.0 — fix the upstream objective and trajectory control` | durable trajectory and priority alignment; no product-code or generated-IR mutation |
| `SPEC-TO-INTENT-ALIGNMENT.1` | `SPEC-TO-INTENT-ALIGNMENT.1 — define source-to-IntentIR category contracts` | versioned category contract, book contract, and retrieval fact; no product-code or generated-IR mutation |
| `SPEC-TO-INTENT-ALIGNMENT.2` | `SPEC-TO-INTENT-ALIGNMENT.2 — guard canonical capability accounting` | 17-row per-run ledger, full CLI partition guard, public contract, and required lossless CHANGES rollover |
| `SPEC-TO-INTENT-ALIGNMENT.3` | `SPEC-TO-INTENT-ALIGNMENT.3 — activate typed figure-region mining` | optional EvidenceIR region producer, source grounding, verifier gate, explicit availability metrics, reviewed retained-PDF vertical fixture, and required Rust-analysis rollover |
| `SPEC-TO-INTENT-ALIGNMENT.4a` | `SPEC-TO-INTENT-ALIGNMENT.4a — build the vertical evaluation engine` | strict portable dataset/API, exact stage-loss rollups, and six controlled-fault classes; no held-out category claim |
| `SPEC-TO-INTENT-ALIGNMENT.4b` | `SPEC-TO-INTENT-ALIGNMENT.4b — lock the reviewed vertical population` | 12 reviewed verticals / 14 cells, SSD source-authority repair, and path-neutral corpus-frontier lifecycle partition; `.4c` owns results |
| `SPEC-TO-INTENT-ALIGNMENT.4c` | `SPEC-TO-INTENT-ALIGNMENT.4c — publish the first reviewed result` | byte-pinned exact result; all six categories incomplete; first failures rank upstream fact formation and residualization |
| `SPEC-TO-INTENT-ALIGNMENT.5a` | `SPEC-TO-INTENT-ALIGNMENT.5a — build the trajectory control engine` | strict nine-dimension state/ranking contract, five states, statistical refusal, hard-first task ownership, and report-only authority |
| `SPEC-TO-INTENT-ALIGNMENT.5b` | `SPEC-TO-INTENT-ALIGNMENT.5b — publish the first trajectory snapshot` | live-equal `.4c`/`.2` composition, byte-current baseline, honest history refusal, and owned `.6` recommendation |
| `SPEC-TO-INTENT-ALIGNMENT.6a` | `SPEC-TO-INTENT-ALIGNMENT.6a — separate replay truth from the frozen baseline` | isolated four-stage replay tool/evidence, dominant 19-record TOC defect retired by current code, 1/12 currentness gate, and residue-free SSD-local scratch lifecycle |
| `SPEC-TO-INTENT-ALIGNMENT.6b.i` | `SPEC-TO-INTENT-ALIGNMENT.6b.i — qualify the complete reviewed population` | isolated 12-source / 48-stage current replay, exact current result and controller v3, residue-free external-source lifecycle, and current Arm Debug repair selection |
| `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a` | `SPEC-TO-INTENT-ALIGNMENT.6b.ii.a — preserve register access and table provenance` | backward-compatible typed carrier, generic producers/merge, exact four-chain ADR 0025 reconciliation, and clean-revision replay boundary |
| `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b` | `SPEC-TO-INTENT-ALIGNMENT.6b.ii.b — qualify the access-carrier repair` | clean 12-source / 48-stage replay, exact Arm closure and two provenance-only collateral changes, controller v4 `.6c` selection, bounded-ingest risk leaf, and residue-free cleanup |
| `SPEC-TO-INTENT-ALIGNMENT.6b.iii` | `SPEC-TO-INTENT-ALIGNMENT.6b.iii — select bounded ingest from resource risk` | resource-sized bounded activation, fail-closed page count, typed signal termination, exact live 400-page fidelity, residue-free cleanup, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6c.i` | `SPEC-TO-INTENT-ALIGNMENT.6c.i — preserve timing unit and table provenance` | closed caption grammar, backward-compatible typed provenance, exact five-chain ADR 0025 reconciliation, clean-replay boundary, and full CI |

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
- `2026-08-12`: Closed `.6b.i` with hash-pinned current-binary replay of all 12 reviewed documents, exact
  current-result projection, strict controller currency/honesty gates, and complete scratch cleanup; frontier
  advances to `.6b.ii`, the measured Arm Debug access-mode loss family.
- `2026-08-12`: Closed `.6b.ii.a` with the generic register-access/table-provenance carrier, focused and bounded
  three-stage proof, exact ADR 0025 reconciliation of four retained chains, and all eight doctrines green;
  frontier advances to `.6b.ii.b`, clean-revision replay and exact metric publication.
- `2026-08-12`: Closed `.6b.ii.b` with 12/12 clean-revision replay, exact 19/10/21 current TP/FP/FN,
  17/29 provenance, 57/78 conservation, strict controller v4 and mutants, complete cleanup, and full CI. The
  semantic controller selects `.6c`; the independently surfaced default single-pass resource failure advances
  the execution frontier first to `.6b.iii` before the next semantic leaf.
- `2026-08-12`: Closed `.6b.iii` with resource-sized bounded activation, fail-closed page counting, typed signal
  reporting, deterministic policy/error/lifecycle tests, an override-free exact 400-page four-stage fidelity
  replay, complete cleanup, and full CI; frontier advances to controller-selected `.6c`.
- `2026-08-12`: Closed `.6c.i` with a generic closed timing caption-unit grammar, direct table provenance,
  exact five-chain retained-corpus reconciliation, residue-free cleanup, and all eight doctrines green;
  frontier advances to `.6c.ii`, clean 12-source replay and exact result/controller publication.
- `2026-08-11`: Closed `.4a` with the strict portable dataset contract, deterministic exact evaluator, and six
  controlled-fault classes; frontier advances to `.4b`, lock two reviewed documents per category without tuning
  extraction before `.4c` publishes product conclusions.
- `2026-08-11`: Closed `.4b` with two reviewed documents per category, portable four-stage identities, complete
  bounded-scope gold, verified SSD source authority, and root-neutral corpus lifecycle accounting; frontier
  advances to `.4c`, publish exact outcomes without changing selection, gold, or extraction.
- `2026-08-11`: Closed `.4c` and parent `.4` with the byte-pinned first reviewed result. All six categories are
  incomplete; 10/14 cells first fail SourceIR → EvidenceIR and four EvidenceIR → SemanticIR. Frontier advances
  to `.5`, compose these exact dimensions into reviewable automatic steering.
- `2026-08-12`: Closed `.5a` with the strict nine-dimension state/ranking engine and 16 mutation/authority tests;
  frontier advances to `.5b`, compose the frozen `.4c` and `.2` authorities into the first snapshot and task.
- `2026-08-12`: Closed `.5b` and parent `.5` with byte-current input/report, a live-equal provider-free ledger,
  `diverging` hard gates plus honest `insufficient_history`, and owned `.6`/`.7`/`.8`/`.9` ranking. Full CI is
  green; after this slice commits cleanly, frontier advances to controller-selected `.6` honesty/provenance.
- `2026-08-12`: Activated `.6a` after localizing 19/41 frozen false positives to AIA table-of-contents timing
  records. The pinned stage hashes match the unreplayed generated artifacts, but current generic timing authority
  predates `.4c` selection and rejects that shape; an isolated replay must establish current truth before repair.
