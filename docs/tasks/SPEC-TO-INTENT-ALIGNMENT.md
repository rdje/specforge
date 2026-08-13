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
  Children: `.6a`, `.6b`, `.6c`, `.6d`

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
  Status: `done`
  Goal: `repair the largest remaining current honesty and provenance family without sacrificing the 19 reviewed true positives`
  Acceptance: `the I2S receiver-timing family preserves its source-grounded ns unit across EvidenceIR, SemanticIR, and IntentIR so the exact five false positives, five false negatives, and five unprovenanced records close; no document-specific production branch is introduced; a complete comparable replay proves the result and re-ranks the remaining current gaps`
  Verification: `generic timing carrier and five-chain reconciliation in .6c.i; clean 12-source / 48-stage replay in .6c.ii; exact I2S 0/5/5/5→5/0/0/0 TP/FP/FN/unprovenanced; seven OpenCAPI records gain provenance only; aggregate TP/FP/FN 19/10/21→24/5/16, provenance 17/29→29/29, conservation 57/78→72/88; all 19 prior true positives retained; controlled mutants and full CI green`
  Commit: `completed by .6c.i and .6c.ii child commits`
  Children: `.6c.i`, `.6c.ii`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6c.i`
  Status: `done`
  Goal: `preserve explicit table-wide timing units and direct table provenance through the canonical timing carrier`
  Acceptance: `the shared timing-table producer recognizes only an explicit closed caption-unit grammar, records direct table authority without misusing statement provenance, and the affected retained corpus chains are reconciled under ADR 0025; the five I2S facts are exact through IntentIR and focused/full gates pass`
  Verification: `closed caption grammar, explicit-row precedence, legacy Serde, scalar/variant authority tests; exact five-fact real I2S three-stage proof; ADR 0025 reconciliation of five retained chains with 231/231 direct table supports, carrier-neutralized 15/15 stage equality, byte-identical adapters, EvidenceIR 24/24 and downstream 78/78 currency; exact scratch cleanup; all eight doctrines, formatting, warning-deny Clippy, 1,866 tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6c.i — preserve timing unit and table provenance`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6c.ii`
  Status: `done`
  Goal: `replay the timing-unit/provenance repair across the complete reviewed population and publish the exact comparable result`
  Acceptance: `all 12 sources and 48 stages replay at the committed .6c.i production revision; exact I2S and collateral provenance deltas are published; all 19 prior true positives survive; controller, public/live documentation, retrieval, and cleanup evidence agree`
  Verification: `12/12 unchanged source hashes and 48/48 isolated stages at 74a658b3; result SHA-256 9bd0a8f6…33d9; I2S 0/5/5/5→5/0/0/0 TP/FP/FN/unprovenanced; seven OpenCAPI records gain provenance only and no other cell changes; controller v5 input/report SHA-256 a622385c…c392 / d6f297aa…7082 meets 29/29 provenance and selects .6d; exact 3,913-file / 1,090,908-KiB root and map absent; all eight doctrines, formatting, warning-deny Clippy, 1,866 tests passed / five ignored / zero failed, rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6c.ii — qualify the timing-carrier repair`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d`
  Status: `in_progress`
  Goal: `remove the largest remaining qualified fabricated-fact family after canonical provenance reaches complete closure`
  Acceptance: `the three reviewed OpenCAPI analog channel-loss records no longer promote as canonical timing facts unless the source-to-IntentIR contract can justify that type; all 24 current true positives and 29/29 provenance closure survive; the repair is generic, replay-qualified, and followed by controller re-ranking`
  Verification: `pending`
  Commit: `completed by .6d.i and .6d.ii child commits`
  Children: `.6d.i`, `.6d.ii`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.i`
  Status: `done`
  Goal: `carry source-grounded physical timing observations without promoting decibel-domain quantities as executable digital timing intent`
  Acceptance: `a closed unit grammar assigns an actionable non-applicable disposition without deleting the source record; downstream stages carry the disposition losslessly and exclude only that disposition from executable temporal derivation; every retained chain changed by the shared producer is measured, attributed, reconciled, validated, and current under ADR 0025; focused and full gates pass`
  Verification: `closed unit grammar, legacy/schema/over-suppression guards, downstream execution/validation eligibility, exact two-chain ADR 0025 reconciliation, actionable reviewed projection, residue-free cleanup, live-doc catalog/rollover maintenance, and full CI; 115 timing records hold, 26 dispositions added, 6/6 stages otherwise equal, 2/2 adapters equal, EvidenceIR 24/24 plus downstream 78/78 current, 1,869 tests / five ignored / zero failed`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.i — preserve physical timing without canonical promotion`

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

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e`
  Status: `in_progress`
  Goal: `install a fail-closed proof-carrying production-genericity architecture and doctrine gate`
  Acceptance: `the production module graph is separated from conformance/oracle code; promoted claims carry machine-checkable derivations from typed current-document evidence; identifiers are opaque information-flow values whose spelling can be copied or identity-compared but cannot steer semantic branches; raw document text and identity cannot be inspected by extraction decision sites outside registered universal grammar interfaces; an AST-aware gate rejects forbidden dependency, taint-flow, and raw-literal decision mutations; every registered inference rule declares and passes an alpha-equivariance obligation; a finite vocabulary census is supplementary diagnostics only`
  Verification: `pending`
  Commit: `completed by .6d.ii.e.i through .6d.ii.e.vii child commits`
  Children: `.6d.ii.e.i`, `.6d.ii.e.ii`, `.6d.ii.e.iii`, `.6d.ii.e.iv`, `.6d.ii.e.v`, `.6d.ii.e.vi`, `.6d.ii.e.vii`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i`
  Status: `done`
  Goal: `freeze the proof-carrying genericity architecture and exact migration denominator before changing production code`
  Acceptance: `an accepted decision record defines the trusted promotion kernel, opaque-symbol capability boundary, typed evidence/grammar premises, rule registry, derivation schema, core-to-conformance dependency direction, AST information-flow gate, alpha obligation, compatibility policy, and residual behavior; every current production producer/claim family and named/calibrated surface is assigned exactly one migration lane with no vocabulary-list proof claim`
  Verification: `ADR 0038 accepted; exact checker classifies all 71 compiled Rust modules and all 168 top-level SourceIR/EvidenceIR/SemanticIR/IntentIR/adapter fields once across 38 families; three controlled self-tests pass and reject unclassified module/field drift; exact dry-run-first rollover seals 15/17/10 whole engineering/status/Rust records with warning-safe live roots and no residue; no production Rust or generated artifact changed; full CI passes all eight doctrines, formatting, warning-deny Clippy, 1,900/6/0 Rust tests, rustdoc, mdBook, and final locality`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i — freeze proof architecture`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii`
  Status: `done`
  Goal: `separate generic production core from conformance/oracle and named calibration surfaces`
  Acceptance: `the compiled core has no dependency on named fixtures, reviewed snapshots, corpus identities, or document-specific commentary/examples; generic evaluation primitives remain reusable behind a one-way conformance dependency; calibrated classifiers are either structurally justified and versioned or moved out of promotion authority; dependency mutations fail closed`
  Verification: `three-package workspace compiles with core <- conformance and application -> both; core Cargo tree has no internal SpecForge dependency; five dependency controls reject direct/aliased reverse edges, a conformance-to-application cycle, and oracle-module reinsertion; live inventory is 76 modules / 38 families / 168 fields; non-test expanded-core named-term diagnostic is empty while named fixtures remain in conformance; public facade paths compile; all 1,900 Rust tests pass with six intentional ignores; formatting, warning-deny Clippy/rustdoc, all doctrines including exact chain currency, mdBook, and final locality pass; no persisted schema or generated canonical artifact changes`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii — separate core from conformance`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii`
  Status: `done`
  Goal: `install the trusted derivation kernel and opaque identity capability types`
  Acceptance: `promotion accepts only typed current-document evidence, registered grammar output, or provenance-bearing model/prior proposals; document and symbol identities can be preserved, displayed, and exact-compared only through explicit capabilities; unsupported legacy/future derivations fail closed or residualize under a versioned compatibility policy`
  Verification: `opaque identity and capability API plus 16 focused unit tests and five external compile-fail tests; deterministic SHA-256 ruleset and conclusion digests; exact current-document premise, grounded proposal/prior, upstream topology, symbol-use, and unique-address verification; schema-valid envelopes cannot self-authorize; inventory 77 modules / 38 families / 168 fields; dependency checks and five mutations pass; 1,916 Rust tests pass / six intentional ignores / zero failures; five doctests pass; formatting, warning-deny Clippy/rustdoc, all doctrines including exact chain currency, mdBook, and final locality pass; no persisted schema or generated canonical artifact changes`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii — install trusted derivation kernel`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv`
  Status: `done`
  Goal: `register production grammar and inference rules and migrate promoted claim families to proof terms`
  Acceptance: `every raw-text interpretation and semantic promotion site is owned by a registered universal rule with typed premises, source span/provenance, declared output kind, and alpha obligation; unregistered production decisions cannot create canonical SourceIR, EvidenceIR, SemanticIR, IntentIR, or ISF authority`
  Verification: `all 41 claim families / 168 field rules across SourceIR, EvidenceIR, SemanticIR, IntentIR, and the ISF adapter are cumulative executable-proof-carrying; canonical build/load/serialize/write/mutate/lower seams replay current registered authority; the 24 reachable chains migrate with zero non-proof/non-validation public delta while 54 legacy chains remain explicit proof-unmeasurable; compiler-derived production-semantic registry/verifier/item/import/kernel closures bind implementation identity without test/comment churn; all child gates, exact chain replay, formatting, warning-deny Clippy/rustdoc, 1,948 Rust tests / six ignores / zero failures, five compile-fail doctests, 156/156 KG fixtures, mdBook, containment, cleanup, and final locality pass`
  Commit: `completed by .6d.ii.e.iv.i through .6d.ii.e.iv.vii child commits`
  Children: `.6d.ii.e.iv.i`, `.6d.ii.e.iv.ii`, `.6d.ii.e.iv.iii`, `.6d.ii.e.iv.iv`, `.6d.ii.e.iv.v`, `.6d.ii.e.iv.vi`, `.6d.ii.e.iv.vii`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i`
  Status: `done`
  Goal: `freeze the exact production rule, producer, mutation, and canonical-insertion denominator`
  Acceptance: `a machine-checked inventory maps every one of the 38 claim families to all current producer and mutator entrypoints, proof class, registered rule ids, typed premise kinds, symbol capability, alpha obligation, artifact schema owner, migration child, and canonical insertion/write/lower seam; missing modules, entrypoints, families, or duplicate rule ids fail closed; the artifact-level cumulative-ledger and proofless compatibility contract is fixed before a schema changes`
  Verification: `the exact checked join covers 38/38 claim families, expands to 168 unique field-root rules, resolves 113 current producer/mutator entrypoints and 39 canonical seams, and accounts for four conformance-only bypasses; six controlled mutations reject missing family, absent entrypoint, capability/alpha mismatch, wrong rule stem, and unsafe conformance disposition; cumulative-ledger, empty-field/per-record coverage, proofless compatibility, and noncanonical overlay contracts are durable; no production Rust, artifact schema, generated canonical artifact, or extraction behavior changes; all eight doctrines, formatting, warning-deny Clippy, 1,916 Rust tests passed / six ignored / zero failed, five compile-fail doctests, warning-deny Rustdoc, mdBook test/build, and final locality pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i — freeze rule migration graph`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii`
  Status: `done`
  Goal: `make every SourceIR family proof-carrying from exact captured source evidence`
  Acceptance: `all five SourceIR families are covered by registered capture, classification, integrity, residual, or evaluation rules; every rule binds executable verification logic and an implementation digest so an editor cannot self-attest by recomputing JSON hashes; classified records cite an exact unclassified capture projection; canonical reload re-executes the registered relation against exact premise and conclusion bytes; current-schema proofless SourceIR cannot feed EvidenceIR; legacy source remains inspectable only and future proof/schema versions reject`
  Verification: `SourceIR schema 3 proves all 19 public fields across five registered families with root and per-record claims; executable verifiers and implementation digests reject hash-consistent forgery, stale/current-proofless/malformed/future authority, edited conclusions, and mismatched grounded proposals; table and visual records carry exact typed capture premises; production mutation and every load/serialize/write/downstream seam reverify; noncanonical overlays cannot persist at any stage; the exact 24 retained bundles passed backing verification, zero-unintended-capture-mutation comparison, complete downstream rebuild, and chain currency at 24/24 measurable EvidenceIR plus 78/78 SemanticIR/IntentIR/adapters with zero stale; 1,930 Rust tests pass / six ignored / zero failed, all 156 KG fixtures pass, formatting and warning-deny Clippy/rustdoc pass, mdBook test/build pass, all eight doctrines pass, and SSD-local rollback/incremental scratch is absent`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii — prove SourceIR authority`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii`
  Status: `done`
  Goal: `make every EvidenceIR producer and post-build mutator proof-carrying`
  Acceptance: `all 11 EvidenceIR families, deterministic extractors, convergence passes, prior-guided paths, VLM/NLP/LLM proposals, alias reclassification, contract/signal mutation commands, and validation backannotations either extend a current verified ledger through registered rules or refuse canonical persistence; every semantic claim cites exact SourceIR claims or current source/table/visual premises`
  Verification: `EvidenceIR schema 3 proves all 39 public fields across 11 families, retains the exact verified SourceIR ledger as an ordered prefix, and replays exact normalized Markdown, consulted prior, and closed typed post-build proposals; every populated record/alias claim is covered; eight mutation kinds close all productive enrichment/backannotation commands and hidden carry-forward is removed; current-schema proofless, stale, forged, unauthorized, legacy, and future authority reject; exact migration of all 24 retained inputs changes zero extracted fields and SemanticIR dry replay changes zero non-validation fields; chain currency reports EvidenceIR 24 current / 54 bundle-unmeasurable, SemanticIR 24 current / 54 proof-unmeasurable, and IntentIR/adapters 78/78 stage-local with zero stale; 1,932 Rust tests pass / six ignored / zero failed plus five compile-fail doctests, all 156 KG fixtures pass, all eight doctrines, formatting, warning-deny Clippy/rustdoc, mdBook, and final locality pass; exact 596-MiB migration scratch and 6.7-GiB rebuildable incremental cache are removed with residue absent`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii — prove EvidenceIR authority`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iv`
  Status: `done`
  Goal: `make every SemanticIR carry, merge, conflict, synthesis, verification, and residual family proof-carrying`
  Acceptance: `all 12 SemanticIR families consume a verified cumulative EvidenceIR ledger; every true carried record cites its upstream claim, filtered or extended evidence uses registered projection authority, and every synthesis/conflict/residual cites all contributing premises under a registered rule; proofless or stale EvidenceIR cannot produce SemanticIR`
  Verification: `SemanticIR schema 2 proves all 49 public fields across 12 families, retains the exact cumulative EvidenceIR ledger as an ordered prefix, and gives every populated collection a record claim; every claim depends on exact registered replay, every true carried root/record additionally cites a direct EvidenceIR claim, timing/signal/conditional projections cannot claim lossless carry, and synthesis/conflict/residual topology includes every upstream claim through a digest-bound replay node; optional prior is exact and identity-independent; validation is the only production mutation kind and cannot change semantic fields; repository relocation normalizes before proof verification; current-schema proofless, stale, forged, unauthorized, legacy, and future authority reject at load/serialize/write/IntentIR seams; exact migration of 24 retained inputs changes zero semantic fields and chain currency is 24 current / 54 proof-unmeasurable through EvidenceIR/SemanticIR/IntentIR with 78/78 stage-local adapters and emitted ISF current; focused SemanticIR/adversarial proof/mutation/relocation controls, production registry, formatting, and warning-deny Clippy pass; full repository qualification recorded below`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iv — prove SemanticIR authority`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v`
  Status: `done`
  Goal: `make every IntentIR carry and canonical behavioral synthesis family proof-carrying`
  Acceptance: `all nine capability-homogeneous IntentIR families consume a verified cumulative SemanticIR ledger; every one of 49 public fields and each populated record has registered exact replay; the 30 true carried fields cite direct SemanticIR claims while filtered actors, NLI-filterable contracts, identity, product synthesis, behavioral relations, residuals, and validation cannot claim lossless authority; validation and demotion-only NLI are closed mutations; proofless or stale SemanticIR cannot produce IntentIR and proofless, forged, unauthorized, legacy, or future IntentIR cannot load, serialize, persist, or feed an adapter`
  Verification: `IntentIR schema 2 proves all 49 public fields across nine homogeneous families, retains the exact cumulative SemanticIR ledger as an ordered prefix, and gives every populated collection a record claim; every local claim depends on registered exact replay, all 30 true carried fields cite direct immediate SemanticIR antecedents, and actor/contract projection cannot claim lossless authority; validation changes only its field, NLI is transactional one-to-one order-preserving demotion, and separately compiled TestFixture projection cannot enter production; canonical load/serialize/write/prior/adapter seams reject proofless, stale, forged, unauthorized, legacy, and future authority; exact migration of 24 reachable chains changes zero non-validation public fields, all 24 adapters validate and emitted ISF matches, while 54 legacy chains remain explicitly proof-unmeasurable; chain self-tests pass 20/20; all eight doctrines, formatting, warning-deny Clippy/rustdoc, 1,943 Rust tests pass / six ignored / zero failed plus five compile-fail doctests, 156/156 KG fixtures, mdBook, and final locality pass; 5,012-file incremental and 71-directory checker scratch residue is removed and absent; production-semantic digest precision is owned by .e.iv.vii`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v — prove IntentIR authority`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi`
  Status: `done`
  Goal: `make ISF lowering proof-gated and qualify the complete 41-family cumulative chain`
  Acceptance: `all four adapter families consume a verified IntentIR ledger; target encoding and every rendered ISF construct cite registered lowering proofs while unsupported content remains source-linked residual; proofless/stale/future artifacts cannot emit ISF; all 41 families and every canonical write/lower seam are proof-covered and current artifacts are reconciled exactly`
  Verification: `Adapter schema 2 proves all 12 fields across four homogeneous families, retains the exact cumulative IntentIR ledger as its ordered prefix, and gives every populated array record, nonblank rendered ISF line, and blocking reason a claim; canonical build/load/serialize/write/emitted-target reconciliation and closed validation backannotation execute registered replay; the ordinary writer rejects proofless/forged artifacts even in test builds, while explicit synthetic fixture persistence is separately compiled; exact migration of 24 reachable manifests changes zero public fields outside schema/proof/validation, validates all 24, and reconciles 24 honestly blocked states to zero emitted files while 54 legacy chains remain proof-unmeasurable; chain currency is 24 current / zero stale / 54 unmeasurable at all four replayed stages, with 24 states / zero emitted files / 24 blocked states and 22/22 self-tests; persisted paths are 3,285 JSON / 947,050 values / zero repository-owned absolute paths; the graph resolves 41 families / 168 rules / 116 entrypoints / 53 seams / four bypasses; all eight doctrines, formatting, warning-deny Clippy/rustdoc, 1,947 Rust tests pass / six ignored / zero failed plus five compile-fail doctests, 156/156 KG fixtures, mdBook, containment, and final locality pass; exact cleanup removes 4,985 rebuildable incremental files / 4,982,260 KiB and leaves target/debug/incremental absent with zero Cargo .bin/.log residue; the retained 108-KiB signal-catalog workspace is active-task referenced and the remaining same-volume tool caches are not disposable`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi — prove ISF lowering authority`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vii`
  Status: `done`
  Goal: `make implementation digests exact to compiled production proof semantics rather than whole source files`
  Acceptance: `every ruleset implementation digest is mechanically derived from the compiled production verifier/rule implementation and its semantic dependency closure; edits confined to cfg(test), conformance fixtures, comments, or documentation cannot stale canonical proof, while any production relation change does; controlled positive and adversarial mutations prove both directions before the proof-migration parent closes`
  Verification: `compiler-visible schema-1 derivation roots all five stage digests at canonical production registries and includes selected verifier, recursively referenced stage-local items, exact import bindings, and complete trusted-kernel tokens; nested sanitizer removes comments/docs/formatting and test/conformance branches while retaining unknown configuration conservatively; controlled mutants prove inert edits preserve identity, referenced helper/import changes alter it, and absent/ambiguous roots fail; runtime closure and stale-ledger controls pass; exact proof-only migration changes zero non-proof/non-validation content across 120 artifacts; chain replay is 24 current / zero stale / 54 unmeasurable with 24 blocked adapter states / zero emitted files; all eight doctrines, formatting, warning-deny Clippy/rustdoc, 1,948 Rust tests / six ignores / zero failures, five compile-fail doctests, 156/156 KG fixtures, mdBook test/build, rolling-ledger containment, and final locality pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vii — scope proof digests to production semantics`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.v`
  Status: `pending`
  Goal: `enforce the module, raw-text, identity-flow, derivation, and rule-registry boundaries with an AST-aware doctrine`
  Acceptance: `one registered doctrine parses the compiled Rust surface and rejects core-to-conformance dependencies, raw identity/text semantic branches outside registered interfaces, unproved promotion constructors, undeclared inference rules, and bypasses hidden behind aliases/macros/modules; parse uncertainty fails closed and the finite vocabulary census remains diagnostic only`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.vi`
  Status: `pending`
  Goal: `prove the structural gate with adversarial mutations and per-rule alpha obligations`
  Acceptance: `controlled mutations for forbidden dependency, identity selector, raw literal/substring/regex decision, unregistered rule, proofless promotion, taint laundering, schema specialization, and missing alpha declaration all fail; every registered rule passes its structural alpha obligation and legal display/provenance/test uses remain admitted`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.vii`
  Status: `pending`
  Goal: `qualify the complete structural genericity boundary and publish exact migration deltas`
  Acceptance: `all retained chains are current; every migrated claim/proof and residual delta is attributed; core/conformance builds, mutation gates, rule obligations, full CI, public/live/book/retrieval truth, and cleanup agree; only `.f` behavioral population qualification remains before the genericity parent can close`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`
  Status: `pending`
  Goal: `qualify genericity behaviorally and replay the reviewed population after remediation`
  Acceptance: `alpha-renaming, structure-preserving paraphrase, adversarial identity, negative-control, and held-out-document tests prove decisions are not coupled to names; the complete reviewed replay and full gates publish all intended deltas without truthfulness/provenance regression; public/live/book/retrieval truth is synchronized`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.6e`
  Status: `pending`
  Goal: `remove the remaining qualified fabricated register facts after the OpenCAPI physical-timing family closes`
  Acceptance: `the AMD IOMMU packed-layout key and GIC-400 summary-caption key are reproduced and separated by root cause before implementation; each bounded repair removes only source-unjustified canonical facts, retains all 24 current true positives and 29/29 provenance, emits an actionable residual where the reviewed contract requires one, and receives clean whole-population qualification plus controller re-ranking`
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
| 16 | `SPEC-TO-INTENT-ALIGNMENT.6c.ii` | `done` | 12-source replay closes I2S and all provenance, publishes exact current result, and selects `.6d` |
| 17 | `SPEC-TO-INTENT-ALIGNMENT.6d.i` | `done` | typed disposition, exact two-chain reconciliation, cleanup, live-doc maintenance, and full CI complete |
| 18 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a` | `done` | clean replay, whole-pipeline audit, test-only snapshot boundary, durable signoff design, cleanup, and full CI complete |
| 19 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b` | `done` | SourceIR schema 2 and exact ten-chain reconciliation are committed and current |
| 20 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c` | `done` | EvidenceIR schema 2 and all 78 persisted downstream chains are generic, reconciled, and current |
| 21 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i` | `done` | identity-independent schema-7 prior memory, neutral KG controls, fixed-point learning, and exact chain reconciliation are complete |
| 22 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` | `done` | opaque identifiers, one-way declaration grounding, alpha-equivariant semantics, and typed-only ISF clock/reset lowering are complete |
| 23 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii` | `done` | opaque prompt information flow and structural corpus routing are verified |
| 24 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv` | `done` | combined qualification is exact; `.d` is closed without a named exception |
| 25 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i` | `done` | ADR 0038, exact inventories, lossless live-ledger rollover, and full CI are complete |
| 26 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii` | `done` | core/conformance direction is compiler-visible and mutation-tested; public facade is compatible |
| 27 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii` | `done` | sealed capability boundary and kernel-only verified authority are installed and adversarially tested |
| 28 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i` | `done` | 38 families / 168 rules / 113 producer-mutators / 39 seams / four bypasses are checked and the cumulative-ledger contract is fixed |
| 29 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii` | `done` | all five SourceIR families are executable-proof-carrying; 24 retained chains are reconciled and 54 legacy SourceIRs remain inspection-only |
| 30 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii` | `done` | all 11 EvidenceIR families and every productive post-build mutation are executable-proof-carrying; 24 retained chains migrate without field delta and 54 legacy inputs remain inspection-only |
| 31 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iv` | `done` | all 12 SemanticIR families are executable-proof-carrying; true carry, mixed evidence projection, relocation, and complete synthesis/conflict/residual topology are verified |
| 32 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v` | `done` | all nine IntentIR families and 49 fields are proof-carrying; 30 exact carries are separated from projection authority |
| 33 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi` | `done` | all four adapter families, rendered lines, blocked output, and exact filesystem reconciliation are proof-gated |
| 34 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vii` | `done` | compiler-derived production registry/verifier/item/import/kernel closures exclude test/comment churn and bind all five proof rulesets |
| 35 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.v` | `pending` | make structural violations mechanically unmergeable |
| 36 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.vi` | `pending` | prove fail-closure with mutations and per-rule alpha obligations |
| 37 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.vii` | `pending` | qualify exact migration deltas before behavioral population work |
| 37 | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` | `pending` | behavioral metamorphic and whole-population qualification closes the genericity parent |
| 38 | `SPEC-TO-INTENT-ALIGNMENT.6e` | `pending` | hard-first controller must remove the two remaining unrelated register fabrications before recall work |
| 39 | `SPEC-TO-INTENT-ALIGNMENT.7` | `pending` | recover source-to-evidence losses after the honesty floor is restored |
| 40 | `SPEC-TO-INTENT-ALIGNMENT.8` | `pending` | make required residuals actionable after canonical loss is bounded |
| 41 | `SPEC-TO-INTENT-ALIGNMENT.9` | `pending` | integrate measured capability breadth only after higher-ranked semantic gaps |

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
| `2026-08-12` | `.6c.ii` clean population replay and projection | 12 source hash checks; 48 isolated stage identities at committed revision `74a658b3`; exact prior/current 14-cell diff | PASS: I2S 0/5/5/5→5/0/0/0 TP/FP/FN/unprovenanced; seven OpenCAPI records gain table provenance only; no other cell changes; all 19 prior true positives survive; aggregate TP/FP/FN 24/5/16, provenance 29/29, conservation 72/88 |
| `2026-08-12` | `.6c.ii` controller, mutants, and cleanup | strict manifest/result loaders; coverage/hash/revision/cleanup/fabrication/population mutants; snapshot write/check; generic controller byte comparison; exact cleanup census | PASS: controller v5 meets 12/12 replay plus 29/29 provenance, fails exactly five fabrications, and selects `.6d`; manifest/result/input/report SHA-256 `e7fc88c0…826a` / `9bd0a8f6…33d9` / `a622385c…c392` / `d6f297aa…7082`; 3,913 files / 1,090,908 KiB and runtime map removed and absent |
| `2026-08-12` | `.6c.ii` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,866 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6d.i` root cause and retained census | exact reviewed-key/table comparison; retained-unit census; focused two-source replay | PASS: four UI digital-skew facts and three false dB/dB_RMS facts share mixed timing/limits tables; 74 retained decibel-domain rows exist across five chains; both reviewed sources retain exact source hashes and all target table support |
| `2026-08-12` | `.6d.i` focused carrier and eligibility tests | unit-only classification/over-suppression mutant; SemanticIR execution filter; SemanticIR/IntentIR validation filter; schema closure/legacy Serde | PASS: dB/dB_RMS/dBc/Hz are actionable non-applicable records; UI/ns and misleading `IL(settle)`+ns remain canonical; physical observations carry but create no temporal rule or false missing-surface diagnostic |
| `2026-08-12` | `.6d.i` ADR 0025 measurement and reconciliation | exhaustive pre-change currency oracle; exact two-chain backup; disposition-neutral comparison; six stage validators; post-rebuild currency oracle | PASS: only two OpenCAPI EvidenceIR chains stale; 115 timing counts hold, exactly 26 gain disposition, 6/6 stages otherwise equal, 2/2 adapters byte-identical; EvidenceIR 24/24 and downstream 78/78 current; retention exact at 24 bundles |
| `2026-08-12` | `.6d.i` projection and cleanup | physical-timing projector over both rebuilt IntentIRs; exact scratch census/removal | PASS: 4/4 digital facts stay canonical; 3/3 analog facts become actionable source-linked residual projections; exact 231-file / 59,296-KiB focused root and 241-file / 57,712-KiB reconciliation root removed and absent |
| `2026-08-12` | `.6d.i` required live-doc maintenance | fact-title derive/write/check; Rust-analysis rollover dry-run/apply/report | PASS: 215-card bounded title catalog current; exact ten-record / 147-line / 12,935-byte segment `75ee4242…404d` sealed; resulting 54-record / 1,072-line root `7c460d92…6cef` warning-safe and the current carrier record remains live |
| `2026-08-12` | `.6d.i` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including whole-chain currency, formatting, warning-deny Clippy, 1,869 Rust tests passed / five ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6d.ii.a` replay and boundary | `cargo test --quiet -p specforge --lib test_support::trajectory_snapshot`; live capability-registry currentness test | PASS: seven snapshot/currentness/mutation tests, one explicit tracked-writer test ignored; capability observation equals the live production registry; reviewed composer absent from `ir/mod.rs` |
| `2026-08-12` | `.6d.ii.a` full pipeline audit | all 71 `crates/specforge/src/` sources; canonical stages, optional VLM/text/NLI/prior/eval paths, embedded Docling Python, adapters, examples, comments/prompts, and module registry | PASS discovery / BLOCK production signoff: every source group is classified; identity routing, named schema/extractors, signal-spelling inference, corpus phrases/thresholds, and named production surfaces are durably owned by `.6d.ii.b`–`.f` |
| `2026-08-12` | `.6d.ii.a` documentation/projections | mdBook test/build; Knowledge Map; fact-card and research catalogs; live-document aggregate/currentness; commit-tier doctrines | PASS: 216 fact cards and 256 collection members route through bounded generated indexes; all seven commit-tier doctrines green |
| `2026-08-12` | `.6d.ii.a` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,870 tests / six ignored / zero failed, warning-deny rustdoc, mdBook, and final project-data locality |
| `2026-08-12` | `.6d.ii.b` SourceIR production census and retained replay | production prefixes of `ir/source.rs` and embedded Python; exact replacement classifier over all retained `generated/source_ir/*/source_ir.json` | PASS: no named protocol/vendor/participant/signal cue remains in the production source surfaces; 78 documents measured read-only with 2,293 diagram, 2,123 section, and 3,484 table re-ingest changes; zero artifact mutation |
| `2026-08-12` | `.6d.ii.b` focused behavioral/compatibility tests | `cargo test -p specforge 'ir::source::'` twice; full `cargo test -p specforge --lib` | PASS: 55 SourceIR tests repeat cleanly; embedded shipped-Python invariance/negative controls pass; schema-1 labels neutralize, future schema rejects; 1,873 passed / six ignored / zero failed |
| `2026-08-12` | `.6d.ii.b` ADR 0025 affected-set resolution | first all-tier currency gate; exact same-volume 80-file backup; ten EvidenceIR→SemanticIR→IntentIR→ISF rebuilds; 30 typed-artifact validations | PASS: affected set is exactly `102520_0101_01_2025_09_15_introducing_coresight_debug_and_trace`, `ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification`, both retained `opencapi` PHY chains, `opencapi_discovery_configuration_v201`, `um10204_rev7_0_2021_i2c_bus_specification`, `um11732_v3_2022_02_17_i2s_bus_specification`, `usb4_inter_domain_service_specification_v2_0_2025_11`, `usb_3_2_revision_1_0_2017_09`, and `wbspec_b4_wishbone_b4_specification`; backup 39,847,014 bytes / SHA-256 `d5a59914…e3a5f`; every rebuild and validator exits zero |
| `2026-08-12` | `.6d.ii.b` exact reconciliation attribution | backup/current structural JSON comparison excluding validation backannotations; real pinned FSMGen strict; `bash scripts/check_chain_currency.sh` | PASS: four visual-role-only chains; 194 weak table timing records removed across four chains; debug chain 21→5 actor relations / 45→33 registers / 195→125 contracts; discovery chain 1→16 registers; USB4 6→0 registers; USB 3.2 gains one message-field and loses one register plus 11 figure-label invariants; both renderable ISFs strict-clean; current at EvidenceIR 24/24 measurable and every downstream stage 78/78 |
| `2026-08-12` | `.6d.ii.b` full repository gate and cleanup | `cargo fmt --all -- --check`; `bash scripts/run_ci.sh`; exact rollback/comparison root removal and residue census | PASS: all eight doctrines including chain currency, formatting, warning-deny Clippy, 1,873 tests / six ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final locality; exact 80-file backup and temporary delta script removed and absent |
| `2026-08-12` | `.6d.ii.c` schema/extractor proof | EvidenceIR schema-1/2/future compatibility tests; 42 focused structural extraction tests; production schema/producer/consumer census | PASS: named SWD operation/direction/phase types and extractors are absent; schema 2 stores opaque operation, phase, actor, state, frame, and direction evidence; legacy named protocol carriers fail closed while unrelated legacy evidence survives; no operation or field is admitted without source-stated shape/binding |
| `2026-08-12` | `.6d.ii.c` ADR 0025 reconciliation and attribution | exact same-volume backup; all 24 retained bundles re-extracted; all 78 EvidenceIRs migrated; 78 SemanticIR/IntentIR/adapter chains rebuilt; exact pre/post structural comparison; `bash scripts/check_chain_currency.sh --check` | PASS: all 78 EvidenceIRs are schema 2; 24/24 replayable EvidenceIR and 78/78 downstream chains are current; 22 named frame records retire to zero because no retained source meets the stricter phase-binding contract, four named operations become five generic operations across two documents, and 76 weak states become 40 structurally supported states across three documents; exact backup/comparison scratch removed and absent |
| `2026-08-12` | `.6d.ii.c` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including chain currency, formatting, warning-deny Clippy, 1,876 Rust tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data-locality residue check |
| `2026-08-12` | `.6d.ii.d.i` stale-authority root cause | exact schema-6 store census; declared-source existence and persisted-validation audit; same-volume original/final artifact comparison | PASS: old 1,492,828-byte store declared 14 accepted sources including one deleted artifact; all 13 current sources lacked the validation backannotation required by policy, so seven named-family prior arrays were quarantined rather than relabelled |
| `2026-08-12` | `.6d.ii.d.i` neutral schema and fixture proof | 97 prior-memory tests; 16 learn-priors tests; 14 KG unit tests; tracked `kg-bench`; schema-6 named-family, schema-7 non-global, and future-schema controls | PASS: `ProtocolFamily` and identity inference are absent; only typed `PriorScope::Global` can be materialized; old scoped data cannot steer; 156/156 KG fixtures pass using term/phrase/header/caption/role/pattern mismatch controls rather than family routing |
| `2026-08-12` | `.6d.ii.d.i` fixed-point and ADR 0025 reconciliation | validate 13 learning inputs; learn/replay/revalidate loop; fourth independent learn byte comparison; 58 changed typed-IR validations plus two adapters; pinned FSMGen strict; `bash scripts/check_chain_currency.sh --check` | PASS: 13/13 accepted; 29/83/4/443/0/1,251/11 semantic prior records plus two structural profiles and one explicit contest; byte-stable SHA-256 `a416cc8b…6239633`; EvidenceIR 24/24 measurable and every downstream stage 78/78; two renderable ISFs strict-clean |
| `2026-08-12` | `.6d.ii.d.i` focused quality and cleanup | formatting; warning-deny all-target Clippy; complete Rust suite; warning-deny rustdoc; mdBook test/build; exact scratch removal/residue census | PASS: 1,880 Rust tests passed / six ignored / zero failed; docs build and test; the exact repository-local rollback/fixed-point root is absent |
| `2026-08-12` | `.6d.ii.d.i` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including exact chain currency, formatting, warning-deny Clippy, 1,880 Rust tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6d.ii.d.ii` production census and structural proof | EvidenceIR through optional NLP, SemanticIR, IntentIR, validation, completeness, and ISF adapter decision sites; case/length/suffix/collision/empty-catalog and alpha-renaming controls | PASS: no audited path assigns signal/interface/handshake/clock/reset/polarity/direction/transaction authority from identifier spelling; exact current-document declarations or typed evidence govern promotion, and unresolved semantics remain explicit |
| `2026-08-12` | `.6d.ii.d.ii` ADR 0025 reconciliation and attribution | exact same-volume 562-file / 437-MiB rollback; 24 EvidenceIR plus all 78 downstream rebuilds; typed validation; structural JSON comparison; `scripts/check_chain_currency.sh`; pinned FSMGen strict | PASS: 18/73/74/74 Evidence/Semantic/Intent/adapter artifacts change; constraints 397→344, invariants 31,767→28,876, transactions 281→239, signal-neutral conditionals 2,156→2,489, renderability 44→17; all 17 emitted targets strict-clean; currency 24/24 plus 78/78; rollback removed and absent |
| `2026-08-12` | `.6d.ii.d.ii` full repository gate | `bash scripts/run_ci.sh` | PASS: all doctrines including exact chain currency, formatting, warning-deny Clippy, 1,890 Rust tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6d.ii.d.iv` exact combined range and focused behavior | `git diff 89d8dee7..9c38b569`; `cargo test -p specforge --lib` filters `alpha`, `identity`, `prompt`, `spelling`, `fixture_name`, `empty_declaration_catalog`, and `undeclared_contract_signal` | PASS: exact three commits / 129 files / +5,922/-4,357, including 24 compiled-crate Rust source and 40 conformance-fixture files; filters pass 13/19/14/14/2/1/1 with zero failures and cover prior scope, opaque identity, prompts, corpus routing, and provider fail-closure |
| `2026-08-12` | `.6d.ii.d.iv` current state and full repository gate | `bash scripts/run_ci.sh`; generated chain-currency doctrine; prior `.d.ii` pinned-FSMGen strict population | PASS: current at 24/24 measurable EvidenceIR and 78/78 SemanticIR/IntentIR/adapters; 17/17 renderable ISFs retain strict qualification; all eight doctrines, formatting, warning-deny Clippy, 1,900 Rust tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final locality |
| `2026-08-12` | `.6d.ii.e.i` exact architecture denominator | full 71-file module census; top-level SourceIR/EvidenceIR/SemanticIR/IntentIR/`AdapterArtifact` field derivation; `perl scripts/check_production_genericity_inventory.pl --self-test`; live checker | PASS: 71/71 modules assigned once to current/target plane and one `.e.ii`–`.e.iv` lane; 168/168 fields assigned once across 38 families; clean fixture plus unclassified-module and unclassified-field controls pass 3/3; no residue |
| `2026-08-12` | `.6d.ii.e.i` mandatory live-ledger containment | exact three-row plan at clean boundary `f7da4ab8`; authenticated dry run; guarded root-last apply; protocol/report/residue census | PASS: development/status/Rust segments seal 15/17/10 whole records at `a72ebc47…1515` / `e97e52c5…5595` / `967477bd…e432`; resulting live roots are 61/51/51 records and 1,366/85/1,017 lines; every warning band and archive chain passes; no transaction workspace remains |
| `2026-08-12` | `.6d.ii.e.i` full repository gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including exact chain currency, formatting, warning-deny Clippy, 1,900 Rust tests passed / six ignored / zero failed, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-12` | `.6d.ii.e.ii` package and dependency boundary | three-package Cargo graph; `cargo tree -p specforge-core --edges normal`; workspace compile; compatibility facade; five controlled dependency mutations | PASS: core has no internal SpecForge dependency; conformance points one way to core; application composes both; direct/aliased reverse edges, application cycle, and oracle-module reinsertion reject 5/5; existing public module paths compile |
| `2026-08-12` | `.6d.ii.e.ii` exact surface and commentary audit | live inventory checker; non-test expanded-core named-term diagnostic; conformance fixture inspection | PASS: 76/76 modules, 38/38 claim families, and 168/168 top-level fields classify once; known corpus names are absent from expanded non-test core and remain legal only in downstream conformance/tests; vocabulary evidence is explicitly supplementary |
| `2026-08-12` | `.6d.ii.e.ii` behavioral and full repository qualification | `cargo test --workspace -- --format terse`; `cargo clippy --workspace --all-targets -- -D warnings`; `bash scripts/run_ci.sh` | PASS: 1,900 Rust tests / six intentional ignores / zero failures; all eight doctrines including exact chain currency, formatting, warning-deny Clippy/rustdoc, mdBook test/build, and final locality; no persisted schema or canonical artifact changed |
| `2026-08-12` | `.6d.ii.e.iii` capability and kernel architecture | opaque-type API review; sealed capability construction; deterministic rule registry; live inventory and dependency gates | PASS: opaque document/symbol atoms expose identity but no spelling conversion, ordering, display, or Serde; grammar/persistence/presentation/lowering and capture capabilities cannot be constructed downstream; ruleset and conclusion bytes use SHA-256; inventory is exact at 77/77 modules, 38/38 families, and 168/168 fields; the one-way package graph remains valid |
| `2026-08-12` | `.6d.ii.e.iii` adversarial derivation controls | 16 focused kernel unit tests; five external compile-fail doctests; compatibility, capture, topology, symbol-use, and tamper controls | PASS: stale/cross-document evidence, empty or indirect grounding, unknown/wrong rules, excessive symbol authority, duplicate addresses, unregistered axioms, conclusion tampering, and invalid upstream order reject; current proof envelopes, legacy, stale, malformed, and future schemas cannot self-authorize canonical truth |
| `2026-08-12` | `.6d.ii.e.iii` behavioral and full repository qualification | `cargo test --workspace --offline -- --format terse`; `cargo clippy --workspace --all-targets -- -D warnings`; `bash scripts/run_ci.sh` | PASS: 1,916 Rust tests / six intentional ignores / zero failures plus five compile-fail doctests; all eight doctrines including exact chain currency, warning-deny rustdoc, mdBook test/build, and final locality; current producers remain proofless and no persisted schema or canonical artifact changed |
| `2026-08-12` | `.6d.ii.e.iv.i` exact rule and canonical-seam census | `perl scripts/check_production_genericity_rules.pl`; path/function resolution against all inventory rows | PASS: 38/38 families expand to 168 unique field-root rules; 113 producer/mutator entrypoints, 39 canonical seams, and four conformance-only bypass obligations resolve; all five stage writers plus raw validation backannotation are named |
| `2026-08-12` | `.6d.ii.e.iv.i` fail-closed inventory controls | `perl -c scripts/check_production_genericity_rules.pl`; `perl scripts/check_production_genericity_rules.pl --self-test` | PASS: syntax clean and 6/6 clean/missing-family/absent-entrypoint/capability-alpha/wrong-stem/unsafe-bypass cases behave exactly; repository-volume fixture residue is absent |
| `2026-08-12` | `.6d.ii.e.iv.i` behavior and full repository qualification | source/schema/generated-artifact diff review; `bash scripts/run_ci.sh` | PASS: no production Rust, persisted schema, extraction behavior, or generated canonical artifact changed; all eight doctrines, formatting, warning-deny Clippy, 1,916 Rust tests / six intentional ignores / zero failures, five compile-fail doctests, warning-deny Rustdoc, mdBook test/build, and final locality pass |
| `2026-08-13` | `.6d.ii.e.iv.iii` exact EvidenceIR authority and mutation closure | schema-3 registry/ledger inspection; focused proof/mutation tests; checked production rule inventory | PASS: all 39 fields in 11 families receive roots, populated records and alias entries receive claims, the verified SourceIR ledger is an exact prefix, and every productive EvidenceIR mutation command extends proof through one of eight closed kinds; current inventory resolves 111 producer/mutator entrypoints, 39 seams, and four bypass obligations |
| `2026-08-13` | `.6d.ii.e.iv.iii` ADR 0025 proof migration and attribution | 144-path / 131-MiB same-volume backup at SHA-256 `c4334b47…9328`; exact 24-document EvidenceIR and SemanticIR dry-run comparison; persisted schema-3 rebuild; `bash scripts/check_chain_currency.sh --check` | PASS: all 24 retained SourceIR proofs are current; 24 EvidenceIRs migrate with zero extracted-field delta; 24 SemanticIR replays have zero non-validation delta; EvidenceIR and SemanticIR are 24/24 current with 54 explicit legacy/proofless unmeasurable inputs; IntentIR and adapters are 78/78 stage-locally current, not end-to-end proof-current; zero stale artifacts |
| `2026-08-13` | `.6d.ii.e.iv.iii` focused and workspace qualification | `cargo test -p specforge-core --lib --offline --no-fail-fast`; `cargo test -p specforge --lib --offline --no-fail-fast`; `cargo test --workspace --offline -- --format terse`; tracked `kg-bench`; warning-deny Clippy/rustdoc; mdBook test/build | PASS: core 1,323/5/0 and app 470/0/0; whole workspace 1,932 Rust tests / six intentional ignores / zero failures plus five compile-fail doctests; KG 156/156; formatting, warning-deny Clippy/rustdoc, and mdBook pass |
| `2026-08-13` | `.6d.ii.e.iv.iii` full repository and locality gate | `bash scripts/run_ci.sh`; exact scratch and requested `.bin`/`.log` census | PASS: all eight doctrines including full chain currency, formatting, warning-deny Clippy, 1,932/6/0 tests plus five compile-fail doctests, warning-deny rustdoc, mdBook test/build, and final project-data locality; removed exact 131-MiB rollback, 465-MiB comparison tree, and 6.7-GiB Cargo incremental cache; no `.bin`/`.log` residue remains in generated/release/debug-deps |
| `2026-08-13` | `.6d.ii.e.iv.iv` exact SemanticIR authority and mutation closure | schema-2 registry/ledger inspection; focused proof, forgery, proofless, downstream, direct-carry/projection, relocation, and mutation tests; checked production rule inventory | PASS: all 49 fields in 12 families receive roots, populated collections receive record claims, cumulative EvidenceIR is an exact prefix, true carried roots/records directly cite EvidenceIR, filtered/extended evidence has registered projection authority, complete upstream topology reaches every semantic replay, validation alone can mutate its exact field, and the current inventory resolves 112 producer/mutator entrypoints, 41 seams, and four bypass obligations |
| `2026-08-13` | `.6d.ii.e.iv.iv` proof migration and chain attribution | canonical rebuild of exactly the 24 schema-3 EvidenceIR inputs; public-field comparison; `bash scripts/check_chain_currency.sh --check` | PASS: all 24 reachable SemanticIRs migrate with zero semantic field delta; EvidenceIR, SemanticIR, and IntentIR are each 24 current / 54 explicit proof-unmeasurable; all 78 adapters and emitted ISF files remain stage-locally current; zero stale artifacts |
| `2026-08-13` | `.6d.ii.e.iv.iv` focused qualification | `cargo test -p specforge-core --lib ir::semantic::tests`; adversarial proof/projection/relocation filters; production inventory/rule checks; formatting; warning-deny all-target/all-feature Clippy | PASS: focused SemanticIR and all proof/forgery/mutation/projection/relocation controls pass; 39 families / 168 field rules / 112 producer-mutators / 41 seams / four bypasses resolve; formatting and Clippy pass |
| `2026-08-13` | `.6d.ii.e.iv.iv` full repository and locality gate | `bash scripts/run_ci.sh` | PASS: all eight doctrines including exact chain currency, formatting, warning-deny Clippy, 1,936 Rust tests / six intentional ignores / zero failures, five compile-fail doctests, warning-deny rustdoc, mdBook test/build, and final project-data locality |
| `2026-08-13` | `.6d.ii.e.iv.v` exact IntentIR authority and mutation closure | schema-2 registry/ledger inspection; focused proof, forgery, proofless, legacy/future, downstream, direct-carry/projection, relocation, validation, and NLI controls; checked production rule inventory | PASS: all 49 fields in nine homogeneous families receive roots, populated collections receive record claims, cumulative SemanticIR is an exact prefix, 30 true carried fields directly cite immediate SemanticIR, filtered actors and NLI-filterable contracts cannot claim lossless carry, validation changes one field, NLI is order-preserving one-to-one demotion only, and the current inventory resolves 116 producer/mutator entrypoints, 47 seams, and four bypass obligations |
| `2026-08-13` | `.6d.ii.e.iv.v` proof migration and chain attribution | exact 24-artifact schema-1 snapshot; release rebuild; pre-existing public-field comparison; typed validation and 24 adapter rebuilds; chain-currency validation/proof identity self-test | PASS: all 24 reachable IntentIRs migrate with zero public-field delta outside schema/proof/validation; 24 IntentIRs and adapters validate; the 11,032-KiB comparison scratch is removed and absent; chain comparator self-test is 20/20, excludes validation plus its proof-derived bytes while canonical downstream loaders still execute proof, and rejects inconsistent counts instead of reporting a negative current population |
| `2026-08-13` | `.6d.ii.e.iv.v` final-source proof currency and implementation-digest finding | full-gate stale-proof reproduction after a test-only `intent.rs` edit; exact current-schema selection; 24 IntentIR / 24 adapter SSD-local before snapshots; canonical rebuild, typed validation, adapter rebuild, and field comparison | PASS with owned follow-up: the whole-file implementation digest correctly failed closed but unnecessarily staled all 24 IntentIR proofs for a test-only source change; exact regeneration changes zero IntentIR public fields outside validation/proof and zero adapter fields outside validation; all 48 artifacts validate; the exact 209,756-KiB scratch is removed and absent; `.e.iv.vii` now owns production-semantic digest scoping |
| `2026-08-13` | `.6d.ii.e.iv.v` newly linked release readiness probe | release CLI pre-main stall; process sample; codesign/hash/dylib checks; same-SSD optimized trivial Rust control; delayed re-execution | PASS with environment note: both new binaries temporarily waited in macOS `dyld` before `main`, signatures and bytes verified, and both became immediately runnable after post-link processing; no SpecForge runtime deadlock or migration write occurred before readiness; exact probes/processes were removed |
| `2026-08-13` | `.6d.ii.e.iv.v` test-fixture closure and final artifact refresh | complete-core adapter fixture reproduction; separately compiled TestFixture projection; exact 24 IntentIR / 24 adapter before snapshots, canonical rebuild, public-field comparison, and validation | PASS: production carried fields still require direct SemanticIR antecedents; only test/test-support code can authorize synthetic projection and it still records the closed mutation premise; core is 1,334/5/0; all 24 IntentIR and adapter artifacts validate with zero non-validation public-field or adapter-content delta; exact 209,756-KiB scratch is absent |
| `2026-08-13` | `.6d.ii.e.iv.v` complete repository qualification | `bash scripts/run_ci.sh`; `cargo test --workspace -- --format terse`; tracked `kg-bench`; exact artifact census and cleanup | PASS: all eight doctrines including 24-current/54-unmeasurable exact chain replay, formatting, warning-deny Clippy/rustdoc, application 470/0/0 + conformance 139/1/0 + core 1,334/5/0 = 1,943 Rust tests passed / six ignored / zero failed, five compile-fail doctests, KG 156/156, mdBook test/build, and final locality; removed 5,012 incremental files / 5,058,468 KiB plus 71 checker scratch directories / 1,638 files / 6,844 KiB; requested `.bin`/`.log` residue is absent |
| `2026-08-13` | `.6d.ii.e.iv.vi` exact adapter/lowering authority | schema-2 registry/ledger inspection; focused proof, forgery, proofless, legacy/future, line/reason coverage, mutation, serialize, write, and emitted-file controls; checked production rule inventory | PASS: all 12 fields in four homogeneous families receive roots, every populated array record and nonblank rendered ISF line plus every blocking reason receives a claim, cumulative IntentIR is an exact prefix, canonical build/load/serialize/write/reconcile execute registered replay, validation changes only its field, and the live graph resolves 116 producer/mutator entrypoints, 53 seams, and four bypass obligations |
| `2026-08-13` | `.6d.ii.e.iv.vi` exact retained-adapter migration | exact 24-artifact schema-1 snapshot; canonical schema-2 rebuild; public-field comparison; typed validation; persisted-path census | PASS: all 24 reachable adapter manifests migrate with zero public delta outside schema/proof/validation; all validate and are honestly blocked/no-file; emitted-file population remains exactly 0→0; 54 upstream-legacy chains receive no synthetic authority; exact 208→268-KiB SSD-local comparison scratch is removed and absent; persisted paths remain repository-relative |
| `2026-08-13` | `.6d.ii.e.iv.vi` blocked-output currency reporting correction | full chain replay; root-cause counter inspection; `adapter_emission_summary` controls; `bash scripts/check_chain_currency.sh --self-test` | PASS: state/file reconciliation was already exact, but the summary had mislabeled one checked adapter state per document as one emitted file; it now derives emitted-file presence from `artifact_layout.emitted_target_path`, reports checked/emitted/blocked counts separately, rejects impossible arithmetic, and passes 22/22 fail-closed self-tests |
| `2026-08-13` | `.6d.ii.e.iv.vi` complete cumulative-chain replay | `bash scripts/check_chain_currency.sh --check` | PASS: EvidenceIR, SemanticIR, IntentIR, and ISF adapter are each 24 replayed / 24 current / zero stale / zero absent / 54 explicit proof-unmeasurable; adapter reconciliation checks 24 output states as zero emitted files plus 24 blocked/no-file states; retained bundles are exactly the declared 24 |
| `2026-08-13` | `.6d.ii.e.iv.vi` research-record rollover-pressure remedy | before/current line-and-byte measurement; duplicate-authority routing review; live-document gate | PASS: the genericity audit was already 613/640 lines before this leaf and reached 621 with current alignment; duplicated kernel/package/census narration was condensed with exact evidence retained in ADR 0038, task rows, and Knowledge Map; every conclusion remains and the canonical audit is 512 lines / 42,160 bytes without a ceiling increase |
| `2026-08-13` | `.6d.ii.e.iv.vi` roadmap projection containment | first full doctrine run; `perl scripts/check_roadmap_projection_contract.pl` | PASS after correction: current priorities initially reached 59/56 lines and blocked the gate; per-leaf detail was routed here and existing link groups compacted without removing direction, restoring the section below warning with source identity, 23-workstream coverage, reader contract, and migrated lifecycle current |
| `2026-08-13` | `.6d.ii.e.iv.vi` focused and workspace qualification | 14 adapter tests; `cargo fmt --all -- --check`; warning-deny all-workspace/all-target Clippy; `cargo test --workspace -- --format terse` | PASS: adapter proof/mutation/compatibility/emission controls are 14/14; formatting and Clippy pass; application 470/0/0 + conformance 139/1/0 + core 1,338/5/0 = 1,947 Rust tests passed / six ignored / zero failed plus five compile-fail doctests |
| `2026-08-13` | `.6d.ii.e.iv.vi` complete repository qualification | `bash scripts/run_ci.sh`; tracked `kg-bench`; final locality and book gates | PASS: all eight doctrines including exact cumulative chain currency, formatting, warning-deny Clippy/rustdoc, application 470/0/0 + conformance 139/1/0 + core 1,338/5/0 = 1,947 Rust tests passed / six ignored / zero failed, five compile-fail doctests, KG 156/156, mdBook test/build, and final project-data locality pass |
| `2026-08-13` | `.6d.ii.e.iv.vii` production-semantic implementation identity | compiler-visible schema-1 digest generator; five canonical registry roots; recursive local item/import closure; trusted-kernel closure; nested configuration sanitizer; controlled inert/sensitive mutants; runtime closure inspection | PASS: comments/docs/formatting, top-level and nested test code, known-inert configuration, unrelated production items, and unrelated imports preserve identity; referenced helper and import-binding mutations change it; absent/ambiguous roots fail; all five generated registry/verifier/rule-field closures are distinct and consume exactly six repository-relative production Rust inputs |
| `2026-08-13` | `.6d.ii.e.iv.vii` exact proof-only corpus migration | final frozen digest rebuild; 24 SourceIR + 24 EvidenceIR + 24 SemanticIR + 24 IntentIR + 24 adapter comparisons excluding `.proof_context`, `.proof_ledger`, and `.validation_reports`; same-volume rollback lifecycle | PASS: all 120 non-proof/non-validation comparisons are exact; the signoff snapshot was exactly 120 files / 964,137,330 bytes and is removed with path residue absent |
| `2026-08-13` | `.6d.ii.e.iv.vii` complete proof-chain replay | `bash scripts/check_chain_currency.sh --check`; adapter state/file reconciliation; retained-bundle census | PASS: EvidenceIR, SemanticIR, IntentIR, and adapter are each 24 replayed/current / zero stale / 54 explicit proof-unmeasurable; 24 adapter states reconcile to zero emitted files and 24 blocked/no-file states; retention remains exactly 24 bundles |
| `2026-08-13` | scheduled artifact cleanup | exact `.bin`/`.log` census under generated and Cargo release/debug trees; rebuildable incremental-cache removal; residue census | PASS: removed 5,085 files / 5,325,300 KiB from `target/debug/incremental`; the directory is absent and zero `.bin`/`.log` files remain in generated/release/debug-deps |
| `2026-08-12` | scheduled artifact cleanup | `.bin`/`.log` census under generated and Cargo release/debug trees; exact age/purpose inspection before deletion; residue census | PASS: removed one abandoned 23-file / 92-KiB Aug-11 live-document-size test workspace and the fully rebuildable 3,116,900-KiB `target/debug/incremental` cache; no `.bin`/`.log` remains in the requested generated/debug-deps/release census |

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

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6c`

- [x] **REPRODUCE / MEASURE** — pin the exact five reviewed I2S receiver-timing keys and prove where the `ns`
  unit exists in source/SourceIR and first disappears across EvidenceIR, SemanticIR, and IntentIR.
- [x] **ROOT CAUSE (WHY + WHERE)** — identify the shared typed carrier, parser, merge, or promotion seam that
  discards the source-grounded unit; distinguish extraction loss from evaluator/provenance projection error.
- [x] **ADDRESSED (verified)** — preserve the grounded unit and provenance through all canonical stages so the
  five false positives, five false negatives, and five unprovenanced records close exactly.
- [x] **NO REGRESSION** — retain all 19 current true positives, reject invented/default units, preserve unrelated
  records and retained corpus currency, and kill controlled unit/provenance/fabrication mutants.
- [x] **GENERICITY (ADR 0006)** — infer only from typed source structure or universal unit grammar; introduce no
  document, vendor, protocol, filename, reviewed-key, or page-layout exception.
- [x] **LOCKSTEP** — comparable whole-population replay, controller, task frontier, roadmap, mdBook, live docs,
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

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6c.ii`

- [x] **REPRODUCE / MEASURE** — replay all 12 unchanged source identities and compare all 48 stage hashes plus
  the exact current result against the committed `.6b.ii.b` authority at production revision `74a658b3`.
- [x] **ROOT CAUSE (WHY + WHERE)** — attribute every metric delta to the generic timing unit/table-provenance
  carrier and separate the reviewed I2S closure from provenance-only collateral or unexpected semantic change.
- [x] **ADDRESSED (verified)** — publish portable replay/result/controller evidence proving the complete five-
  record I2S disposition and the next ranked current honesty/provenance family.
- [x] **NO REGRESSION** — retain all 19 prior true positives, AIA and Arm honesty, source identities, frozen
  authorities, canonical generated artifacts, and cleanup; kill replay/result/controller mutants and run CI.
- [x] **GENERICITY (ADR 0006)** — replay and projection remain population-driven, and every measured change
  follows typed carrier behavior rather than document-specific postprocessing.
- [x] **LOCKSTEP** — result, controller, task frontier, roadmap, mdBook, live docs, architecture, Knowledge Map,
  cleanup proof, and resume pointer publish the same exact outcome and next owner.

## Planned closing criteria — `SPEC-TO-INTENT-ALIGNMENT.6d`

- **REPRODUCE / MEASURE** — pin the three OpenCAPI analog channel-loss keys and compare their complete typed
  source/table shape with the four correct OpenCAPI digital-skew facts and unrelated timing rows.
- **ROOT CAUSE (WHY + WHERE)** — identify why scalar analog insertion-loss rows enter the canonical timing
  contract despite empty reviewed gold, and distinguish producer overclassification from oracle/type mismatch.
- **ADDRESSED (verified)** — remove or correctly retype the three false canonical timing facts while retaining
  their source authority and without hiding them as unexplained disappearance.
- **NO REGRESSION** — retain all 24 current true positives, 29/29 provenance, the two other known fabrication
  families, stage currency, and exact category/cell authority; kill classification and over-suppression mutants.
- **GENERICITY (ADR 0006)** — decide from universal typed table/quantity semantics, never OpenCAPI, document,
  filename, table id, reviewed key, or page layout.
- **LOCKSTEP** — focused carrier evidence, clean whole-population replay, controller re-ranking, task/book/live/
  retrieval truth, cleanup, and full CI agree before advancing to `.7` or another honesty child.

The carrier/corpus transaction is owned by `.6d.i`; the clean committed-revision population qualification is
owned by `.6d.ii`. This is the same isolation boundary used by `.6c`: ADR 0025 reconciliation must land before
the reviewed replay can name its production revision.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.i`

- [x] **REPRODUCE / MEASURE** — pin the exact three false analog and four correct digital facts, compare their
  complete mixed-table shape, census the retained decibel-domain family, and run ADR 0025's exhaustive currency
  replay before rebuilding anything.
- [x] **ROOT CAUSE (WHY + WHERE)** — `synthesize_timing_constraints` promoted every independent scalar row from
  a timing-class table into one canonical record type; SemanticIR cloned all records into temporal derivation and
  validation had no typed applicability boundary.
- [x] **ADDRESSED (verified)** — a schema-closed disposition keeps values/table authority/actionability through
  IntentIR, while only canonical rows derive temporal rules; exactly 26/115 records move across the two measurable
  chains, with six stage validators and whole-chain currency green.
- [x] **NO REGRESSION** — focused timing/evaluator/trajectory tests, warning-deny Clippy, mdBook/retrieval/live-doc
  checks, full CI, exact scratch cleanup, and post-commit continuity must all pass.
- [x] **GENERICITY (ADR 0006)** — the first alphanumeric unit token alone recognizes closed `dB`/`dBc` grammar;
  no parameter, table, document, vendor, layout, or reviewed-key branch exists, and an `IL`-shaped name with `ns`
  is pinned canonical.
- [x] **LOCKSTEP** — code, rebuilt corpus, task/live status, architecture analysis, mdBook, Knowledge Map, bounded
  resume pointer, aggregate authority, and the `.6d.ii` clean-replay frontier must agree at commit.

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

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i`

- [x] **REPRODUCE / MEASURE** — derive the complete compiled Rust module set and all top-level fields of SourceIR,
  EvidenceIR, SemanticIR, IntentIR, and `AdapterArtifact`; classify 71 modules and 168 fields in 38 families
  exactly once rather than estimating the migration from vocabulary hits.
- [x] **ROOT CAUSE (WHY + WHERE)** — record that identity can be encoded through unseen strings, aliases, paths,
  hashes, ordering, numeric selectors, prompts, or learned labels, while direct record construction and partial
  manifests cannot prove per-claim authority. A finite denylist and syntax scan therefore cannot establish
  noninterference.
- [x] **ADDRESSED (verified)** — accept ADR 0038's compiler-visible core/conformance direction, opaque spelling
  capabilities, registered grounded proposals, sealed promotion kernel, artifact proof ledger, rule descriptors,
  per-rule alpha obligations, compatibility policy, and explicit residual behavior before production migration.
- [x] **NO REGRESSION** — change no production Rust behavior or generated canonical artifact; the inventory checker
  and three-part self-test pass, full CI remains green, and repository-volume scratch residue is absent.
- [x] **GENERICITY (ADR 0006)** — prove coverage through closed module/claim denominators and information-flow/
  derivation architecture. Vocabulary scanning remains supplementary diagnostics and cannot grant signoff.
- [x] **LOCKSTEP** — decision index, audit, task frontier, mdBook, live status, architecture analysis, development
  rationale, change ledger, Knowledge Map projection, and bounded resume pointer publish the same accepted-but-not-
  implemented boundary and next `.e.ii` owner.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii`

- [x] **REPRODUCE / MEASURE** — compile the same production sources through an isolated core package, derive the
  live post-split 76-module graph, and preserve the frozen 38-family / 168-field claim denominator.
- [x] **ROOT CAUSE (WHY + WHERE)** — one Rust crate allowed core, application, evaluation, calibrated
  completeness classification, replay, trajectory, and named snapshot code to import one another; a directory
  convention and vocabulary scan could not prevent conformance feedback from becoming extraction authority.
- [x] **ADDRESSED (verified)** — Cargo now enforces `specforge-core <- specforge-conformance`; the application
  depends on both and preserves legacy public paths. Evaluation, completeness, replay, trajectory, and the named
  reviewed snapshot compile downstream; shared transport/locality foundations compile below the application.
- [x] **NO REGRESSION** — the complete 1,900-test baseline passes with the same six intentional ignores; warning-
  deny Clippy/rustdoc, all doctrines including chain currency, mdBook, and locality pass; no persisted schema or
  generated canonical artifact changes.
- [x] **GENERICITY (ADR 0006)** — direct and aliased reverse dependency, application-cycle, and oracle-registry
  mutations fail. Named fixture identity is confined to conformance/tests; the empty known-name expanded-core
  diagnostic assists review but is not claimed as proof.
- [x] **LOCKSTEP** — Cargo manifests, module inventory, ADR reverify route, audit, roadmap, README, mdBook, live
  status, Rust analysis, development rationale, change ledger, Knowledge Map, task frontier, and resume pointer
  publish the shipped boundary and the still-open `.e.iii`–`.f` proof/behavior work.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii`

- [x] **REPRODUCE / MEASURE** — preserve the exact 38-family / 168-field migration denominator while expanding
  the live compiled inventory from 76 to 77 modules; distinguish syntax-valid proof data from kernel-verified
  authority, and measure the pre-migration state honestly as zero proof-carrying production claim families.
- [x] **ROOT CAUSE (WHY + WHERE)** — ordinary strings expose spelling through conversion, display, sorting, and
  serialization, while an ordinary serializable provenance record can be forged or replayed without validating
  its current evidence. Neither a forbidden-word list nor a proof-shaped DTO establishes semantic noninterference.
- [x] **ADDRESSED (verified)** — opaque document/symbol atoms retain private spelling behind sealed, purpose-
  specific capabilities. The closed rule registry and promotion kernel validate exact current-document typed
  premises, grounded proposals/priors, registered axioms, upstream proofs, declared symbol uses, and conclusion
  bytes before issuing the only witness that permits canonical authority.
- [x] **NO REGRESSION** — the complete 1,916-test workspace passes with six intentional ignores and zero failures;
  five external compile-fail contracts, warning-deny Clippy/rustdoc, all doctrines, mdBook, and locality pass.
  No existing producer, persisted schema, or generated canonical artifact changes in this substrate-only slice.
- [x] **GENERICITY (ADR 0006)** — authority follows typed information flow and current evidence rather than text
  fragments. Identity atoms have no semantic spelling API; every rule declares a non-optional alpha obligation;
  schema-valid, legacy, stale, malformed, and future proof envelopes all remain non-authoritative until verified.
- [x] **LOCKSTEP** — the ADR, pipeline audit, Knowledge Map fact/projection, roadmap, README, mdBook, live docs,
  inventory, task frontier, and resume pointer publish the shipped substrate and explicitly leave every current
  production family proofless for `.e.iv` migration rather than claiming genericity signoff early.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i`

- [x] **REPRODUCE / MEASURE** — join all 38 claim families to 168 deterministic field-root rules and resolve
  113 current producer/mutator entrypoints, 39 canonical seams, and four conformance-only bypass obligations.
- [x] **ROOT CAUSE (WHY + WHERE)** — canonical authority is created not only by stage constructors but by prior
  carry, dedup/reclassification, provider/condition/contract/register/signal mutation, validation backannotation,
  visual enrichment, and temporary conformance patches that can reuse ordinary writers or downstream builders.
- [x] **ADDRESSED (verified)** — checked inventories fix premise/capability/alpha/compatibility metadata and one
  stage owner for every family and bypass. The durable design uses one verified ordered ledger, field-root plus
  per-record coverage, proof-staling mutation, fail-closed history, and noncanonical conformance overlays.
- [x] **NO REGRESSION** — no production Rust, artifact schema, extraction behavior, or generated canonical
  artifact changes. The 1,916/6/0 Rust baseline, five compile-fail doctests, all doctrines, Clippy/Rustdoc,
  mdBook, and final locality pass.
- [x] **GENERICITY (ADR 0006)** — the contract constrains typed information flow, registered derivation, and
  canonical sinks rather than enumerating forbidden words. Temporary storage and fixture identity grant no trust.
- [x] **LOCKSTEP** — inventories, checker, ADR, audit, Knowledge Map, roadmap, README, mdBook, live docs, task
  frontier, and resume pointer publish the same exact migration denominator and leave stage behavior honestly open.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii`

- [x] **REPRODUCE / MEASURE** — all 19 SourceIR public fields across five families have field-root proof and
  every populated collection has record proof; only the 24 retained normalized captures satisfy migration input.
- [x] **ROOT CAUSE (WHY + WHERE)** — a digest-consistent ledger can still be forged if canonical load checks only
  hashes; SourceIR classification and grounded refinements require executable relations over exact captured input.
- [x] **ADDRESSED (verified)** — SourceIR schema 3 re-executes current registered capture, classification,
  integrity, residual, and evaluation relations at every load/write/downstream seam with implementation digests.
- [x] **NO REGRESSION** — the 24 backed captures show zero unintended field mutation, complete downstream replay
  was current at closure, full Rust/KG/warning-deny/book/doctrine gates passed, and exact scratch was removed.
- [x] **GENERICITY (ADR 0006)** — classifications start from an exact unknown-kind projection and may use typed
  table/visual/source evidence or grounded model response, never document identity or a synthetic proof wrapper.
- [x] **LOCKSTEP** — schema, registry, ADR 0038, book, task, live docs, Knowledge Map, and generated retained
  population publish the same 24-current / 54-inspection-only SourceIR authority boundary.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii`

- [x] **REPRODUCE / MEASURE** — all 39 EvidenceIR public fields across 11 families have roots; populated records
  and alias entries have claims; the live graph resolves 111 producer/mutator entrypoints and 39 canonical seams.
- [x] **ROOT CAUSE (WHY + WHERE)** — deterministic construction was followed by unrecorded carry-forward and
  multiple direct enrichment/backannotation mutations, while a typed model response alone cannot prove semantic
  truth and a per-record whole-stage replay would make verification quadratic.
- [x] **ADDRESSED (verified)** — schema 3 retains the exact SourceIR proof prefix and replays exact Markdown,
  optional validated prior, and ordered closed typed proposals. Eight mutation kinds restrict fields and support;
  registered per-claim projections depend on one verified global replay and keep ledger verification linear.
- [x] **NO REGRESSION** — all 24 retained EvidenceIR migrations and SemanticIR dry runs have zero semantic field
  delta; currency has zero stale artifacts; 1,932 Rust tests plus five compile-fail doctests and all 156 KG
  fixtures pass with warning-deny Clippy/rustdoc, formatting, and mdBook green.
- [x] **GENERICITY (ADR 0006)** — the production boundary authenticates information flow and exact current-source
  replay rather than tokens. A model proposal records bounded evidence but cannot authorize itself; legacy,
  proofless, stale, malformed, future, unsupported, or unauthorized mutations fail closed.
- [x] **LOCKSTEP** — code, schemas, rule inventory, ADR 0025/0038, roadmap, mdBook, live docs, Knowledge Map,
  task frontier, and generated corpus distinguish 24 end-to-Evidence proof-current chains, 54 proof-blocked
  Evidence-to-Semantic replays, and 78/78 later stage-local replays without claiming end-to-end closure.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iv`

- [x] **REPRODUCE / MEASURE** — all 49 SemanticIR public fields across 12 families have roots and every populated
  collection has record claims; the cumulative EvidenceIR ledger is an exact prefix and the live graph resolves
  112 producer/mutator entrypoints, 41 canonical seams, and four conformance-only bypass obligations.
- [x] **ROOT CAUSE (WHY + WHERE)** — deterministic semantic construction, prior-guided arbitration, lossless
  carry, merges/conflicts/synthesis/residuals, and validation backannotation previously produced ordinary JSON
  without an executable proof that the current verified EvidenceIR and registered transform determined it.
- [x] **ADDRESSED (verified)** — schema 2 independently rebuilds the stage. Every semantic claim binds exact replay;
  every true carried root/record directly cites EvidenceIR; filtered/extended timing, signal-constraint, and
  conditional-rule surfaces use projection authority; every synthesis/conflict/residual transitively cites the
  complete upstream graph through one digest-bound replay node; validation mutation covers only its fixed field.
- [x] **NO REGRESSION** — exact migration of the 24 verifiable chains changes no semantic field. Currency is 24
  current / 54 explicit proof-unmeasurable through IntentIR, with all 78 adapters and emitted ISF current; focused
  proof controls, all 417 SemanticIR tests, formatting, production registry, and warning-deny Clippy pass.
- [x] **GENERICITY (ADR 0006)** — authority follows exact upstream claims, registered computation, and an optional
  identity-independent prior. No document, vendor, protocol, filename, or symbol vocabulary grants promotion;
  proofless, stale, forged, unauthorized, legacy, and future artifacts fail closed at canonical seams.
- [x] **LOCKSTEP** — code, schema, rule inventory, roadmap, mdBook, live docs, Knowledge Map, task frontier, and
  retained population publish the same 24-current / 54-inspection-only SemanticIR authority boundary and leave
  IntentIR/adapter proof migration honestly open.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v`

- [x] **REPRODUCE / MEASURE** — all 49 IntentIR public fields classify once across nine homogeneous rule
  families; 30 exact carries are distinct from filtered actors, NLI-filterable contracts, synthesis, residual,
  and validation; the live graph resolves 116 producer/mutator entrypoints, 47 canonical seams, and four
  conformance-only bypass obligations.
- [x] **ROOT CAUSE (WHY + WHERE)** — ordinary IntentIR construction cloned, filtered, and synthesized from
  SemanticIR, while validation and optional NLI mutated the product afterward; schema-valid JSON and provenance
  could not prove the current verified SemanticIR plus registered implementation determined every conclusion.
- [x] **ADDRESSED (verified)** — schema 2 retains cumulative SemanticIR proof as an exact prefix, registers every
  field and record replay, directly binds every true carry to immediate SemanticIR, independently rebuilds the
  product, and closes validation plus one-to-one order-preserving NLI demotion before serialization or writing.
- [x] **NO REGRESSION** — exact migration of the 24 reachable chains changes zero pre-existing public field;
  focused transformation/proof/forgery/compatibility/relocation/mutation tests pass; the remaining 54 chains stay
  explicit proof-unmeasurable rather than receiving synthetic authority. Full repository qualification is
  recorded in this leaf's verification ledger before closure.
- [x] **GENERICITY (ADR 0006)** — authority follows exact current upstream claims, registered replay, and typed
  conservative mutation. No vendor, protocol, document, filename, corpus family, or source-symbol vocabulary
  selects admission; a forbidden-token list remains diagnostic only.
- [x] **LOCKSTEP** — code, schema, rule inventory, chain-currency validation identity, roadmap, mdBook, live docs,
  Knowledge Map, task frontier, and retained population publish the same 24-current / 54-inspection-only
  IntentIR boundary and leave adapter proof migration honestly open.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi`

- [x] **REPRODUCE / MEASURE** — all 12 adapter fields classify once across four homogeneous families; every
  populated array record, nonblank rendered ISF line, and blocking reason has a stable claim; the graph resolves
  116 producer/mutator entrypoints, 53 canonical seams, and four conformance-only bypass obligations.
- [x] **ROOT CAUSE (WHY + WHERE)** — typed lowering, renderability, residual generation, validation
  backannotation, manifest persistence, and emitted-file reconciliation previously created ordinary output
  without executable proof that current verified IntentIR plus the registered lowering determined it.
- [x] **ADDRESSED (verified)** — adapter schema 2 preserves cumulative IntentIR proof as an exact prefix, proves
  all local fields/records/rendered lines/blocking reasons, rebuilds the complete product at every canonical seam,
  closes validation mutation, and makes both emission and honest non-emission authoritative outcomes.
- [x] **NO REGRESSION** — exact migration of the 24 reachable adapters changes no pre-existing public field and
  reconciles 24 blocked states to zero emitted files; focused and workspace tests pass. Final all-doctrine,
  KG, rustdoc, mdBook, chain-currency, locality, and cleanup evidence must be recorded before closure.
- [x] **GENERICITY (ADR 0006)** — lowering authority follows exact verified IntentIR, registered typed
  computation, and explicit residual/blocking disposition. No vendor, protocol, document, filename, corpus
  family, source symbol, or forbidden-vocabulary match selects output.
- [x] **LOCKSTEP** — code, schema, rule inventory, chain-currency state/file accounting, roadmap, mdBook, live
  docs, Knowledge Map, task frontier, and retained population must pass derive-and-diff/full gates before closure.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vii`

- [x] **REPRODUCE / MEASURE** — whole-module hashes were shown to stale 24 valid chains after a test-only edit;
  final stage closures and all 120 migrated artifacts were enumerated and compared exactly.
- [x] **ROOT CAUSE (WHY + WHERE)** — `include_bytes!` treated comments, docs, tests, fixtures, and unrelated code
  as verifier authority, while omitting the structural distinction between a referenced production relation and
  inert source-file content.
- [x] **ADDRESSED (verified)** — compiler-visible schema-1 derivation roots each digest at its canonical registry,
  follows the selected production verifier, referenced local items and exact import bindings, includes the full
  trusted kernel, removes nested test/conformance/inert syntax, and fails on absent or ambiguous roots.
- [x] **NO REGRESSION** — controlled inert/sensitive mutants and runtime closure/stale-ledger controls pass; the
  final proof-only migration leaves every non-proof/non-validation value unchanged across 120 artifacts; full
  workspace, chain, KG, book, doctrine, and locality gates pass.
- [x] **GENERICITY (ADR 0006)** — digest authority derives from code structure and registered proof semantics,
  never a finite vocabulary or named corpus; test-only named fixtures cannot enter production identity.
- [x] **LOCKSTEP** — code, ADR 0038, task tree, roadmap, research audit, Knowledge Map, live docs, mdBook, retained
  proof population, archive rollover, and resume pointer state the same exact boundary and remaining AST work.

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
| `SPEC-TO-INTENT-ALIGNMENT.6c.ii` | `SPEC-TO-INTENT-ALIGNMENT.6c.ii — qualify the timing-carrier repair` | clean 12-source / 48-stage replay, exact I2S closure and seven provenance-only collateral records, controller v5 `.6d` selection, and residue-free cleanup |
| `SPEC-TO-INTENT-ALIGNMENT.6d.i` | `SPEC-TO-INTENT-ALIGNMENT.6d.i — preserve physical timing without canonical promotion` | typed decibel-domain disposition, exact two-chain reconciliation, catalog/ledger maintenance, clean-replay boundary, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii` | `completed by .6d.ii.a through .6d.ii.f child commits` | replay publication plus whole-production genericity remediation and qualification |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a — audit production genericity` | clean replay publication, complete pipeline audit, structural signoff design, test-only reviewed snapshot composition, and residue-free cleanup |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.b — make SourceIR classification neutral` | SourceIR schema-2 structural classifiers, legacy fail-closure, exact ten-chain ADR 0025 reconciliation, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c — make protocol EvidenceIR document-derived` | generic schema-2 protocol evidence, legacy fail-closure, exact 78-chain ADR 0025 reconciliation, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.i — make prior memory identity-independent` | schema-7 global prior scope, legacy quarantine, neutral KG controls, byte-stable relearning, and exact 78-chain currency |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.ii — make document identifiers opaque` | opaque identifiers, one-way current-document grounding, alpha-equivariant semantics, typed-only ISF clock/reset lowering, exact 78-chain reconciliation, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iii — neutralize prompts and corpus organization` | identifier-redacted provider policy, declaration-ordered catalogs, structural KG capabilities, schema-2 candidate projection, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.d.iv — qualify identity remediation` | exact three-commit range, combined alpha/identity/prompt/corpus qualification, current-chain proof, lockstep publication, and full CI |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.i — freeze proof architecture` | ADR 0038, exact 71-module / 38-family / 168-field migration inventories, drift-rejecting checker, and no production behavior change |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.ii — separate core from conformance` | compiler-visible one-way package graph, compatible facade, 76-module live inventory, five dependency controls, and unchanged behavior |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iii — install trusted derivation kernel` | sealed opaque identity capabilities, closed rule registry, kernel-only verified authority, fail-closed compatibility, and unchanged persisted behavior |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.i — freeze rule migration graph` | exact 38-family / 168-rule / 113-entrypoint / 39-seam graph, four fail-closed conformance bypasses, cumulative-ledger contract, and unchanged production behavior |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii — prove SourceIR authority` | SourceIR schema-3 executable relation proofs for five families / 19 fields, exact 24 retained migrations, and fail-closed history |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iii — prove EvidenceIR authority` | EvidenceIR schema-3 cumulative proof for 11 families / 39 fields, closed mutation replay, exact 24 retained migrations, and proof-frontier currency |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iv` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.iv — prove SemanticIR authority` | SemanticIR schema-2 cumulative proof for 12 families / 49 fields, direct true-carry antecedents, registered mixed evidence projection, repository relocation, closed validation mutation, exact 24 retained migrations, and proof-frontier currency |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.v — prove IntentIR authority` | IntentIR schema-2 cumulative proof for nine families / 49 fields, 30 direct true-carry antecedents, actor/contract projection separation, closed validation and NLI demotion, exact 24 retained migrations, and proof-frontier currency |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vi — prove ISF lowering authority` | Adapter schema-2 cumulative proof for four families / 12 fields plus every array record/rendered line/blocking reason, closed validation, exact 24 retained migrations, and emitted-versus-blocked reconciliation |
| `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vii` | `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.vii — scope proof digests to production semantics` | compiler-derived registry/verifier/item/import/kernel closures, inert/sensitive mutations, exact proof-only 120-artifact migration, and complete chain qualification |
| `SPEC-TO-INTENT-ALIGNMENT.6e` | `pending` | measured remaining AMD IOMMU and GIC-400 fabrication families, root-cause split, and clean qualification |

## Changelog

- `2026-08-12`: Activated `.6d.ii.d.iii` only after `.d.ii` committed at `f88d463d` and the post-commit
  tree was clean. This leaf owns compiled model prompts and production corpus/KG organization: prompts may teach
  generic digital-design concepts with synthetic symbols and current-document evidence, while vendor, protocol,
  ISA, document, and corpus-family identities must not select extraction behavior or production routing.
- `2026-08-12`: Closed `.6d.ii.d.iii` after the complete prompt census removed named framing, raw entity-name
  transport, spelling-ordered catalogs, and ungrounded provider promotion; corpus-KB routing now derives only
  typed structural capabilities and all 156 fixtures reproduce across ten managed pages. Full Rust, doctrine,
  mdBook, currentness, and locality gates pass. After this commit is clean, `.d.iv` owns combined qualification.
- `2026-08-12`: Activated `.6d.ii.d.iv` only after `.d.iii` committed at `9c38b569` and the post-commit tree was
  clean. This leaf owns combined current-binary chain currency, cross-path alpha/identity qualification, exact
  prompt/corpus deltas, and lockstep publication before parent `.d` may close.
- `2026-08-12`: Closed `.6d.ii.d.iv` and parent `.d` after exact `89d8dee7..9c38b569` range measurement,
  cross-path behavioral controls, current-binary chain currency, and full repository qualification. The closure
  preserves every measured honesty delta and makes no whole-core claim: `.e` owns structural proof enforcement,
  named production commentary/calibration disposition, and mutation gates; `.f` owns population metamorphics.
- `2026-08-12`: Activated `.6d.ii.e.i` only after `.d.iv` committed at `f7da4ab8` and the post-commit tree was
  clean. The parent is split into seven commit-bounded lanes so proof architecture, dependency separation,
  derivation/identity capabilities, rule migration, AST enforcement, adversarial mutations, and final
  qualification cannot collapse into an unauditable rewrite. `.e.i` is design/inventory only; no production
  edit may precede its accepted architecture and complete producer/claim migration denominator.
- `2026-08-12`: The first doctrine run after mandatory live-doc alignment correctly refused the commit because
  development notes, status, and Rust analysis crossed their declared 90% rollover milestones. The current leaf
  owns the required exact whole-record, repository-volume rollover as a closing dependency; no limit may widen and
  no record may be dropped, edited, or reordered.
- `2026-08-12`: Closed `.6d.ii.e.i` after the authenticated rollover dry-run and root-last apply sealed exact
  15/17/10-record engineering/status/Rust segments, restored every live warning band, and preserved all archive
  chains without residue. ADR 0038 plus the 71-module / 38-family / 168-field inventories are now the accepted
  implementation denominator; no production extraction behavior changed. Full CI passes, and `.e.ii` is next.
- `2026-08-12`: Activated `.6d.ii.e.ii` only after `.e.i` committed at `50a657a5` and the post-commit tree was
  clean. This leaf owns the compiler-visible one-way core/conformance boundary, relocation of named examples and
  calibrated/reviewed authority, preservation of generic metric primitives, dependency-failure controls, and
  exact behavior/currency qualification. It does not yet own the proof kernel or claim-rule migration.
- `2026-08-12`: Closed `.6d.ii.e.ii` after the three-package workspace made core-to-conformance imports
  unrepresentable, moved evaluation/replay/trajectory/named fixtures downstream, preserved application API paths,
  neutralized named compiled-core documentation, and passed five dependency controls plus the unchanged 1,900/6/0
  Rust baseline and full repository gate. `.e.iii` is next only after this commit is clean.
- `2026-08-12`: Activated `.6d.ii.e.iii` only after `.e.ii` committed at `f0cfc7b9` and the post-commit tree was
  clean. This leaf owns the sealed opaque identity/capability types, versioned proof schema, trusted promotion
  validator, and fail-closed legacy/future compatibility boundary. Per-rule producer migration remains `.e.iv`.
- `2026-08-12`: Closed `.6d.ii.e.iii` after installing sealed opaque identity capabilities, a deterministic closed
  ruleset, typed premise capture, a proof ledger, and kernel-only verified authority. Sixteen unit controls and five
  external compile-fail contracts reject spelling access, capability forgery, evidence laundering, proof replay,
  tampering, and compatibility self-authorization; the full 1,916/6/0 baseline and repository gate pass without a
  persisted schema or generated-artifact change. `.e.iv` next migrates all 38 claim families to registered rules.
- `2026-08-12`: Activated `.6d.ii.e.iv` only after `.e.iii` committed at `0ef93078` and the post-commit tree was
  clean with a zero-byte commit brief. This leaf owns the exact whole-production producer/promotion census, closed
  universal rule registration, proof attachment, kernel verification before canonical insertion, and honest
  residualization for every unproved claim across all 38 inventoried families. No named document gets an exception.
- `2026-08-12`: Closed `.6d.ii.e.iv.i` after the exact checked migration graph covered all 38 families, 168 field
  rules, 113 current producer/mutator entrypoints, 39 canonical seams, and four conformance-only bypasses. Six
  controlled mutations and full CI pass. The cumulative-ledger and noncanonical-overlay contracts change no
  production behavior; `.e.iv.ii` next owns SourceIR proof-bearing implementation after this commit is clean.
- `2026-08-12`: Activated `.6d.ii.e.iv.ii` only after `.e.iv.i` committed at `99fad58c` and the post-commit tree
  was clean with a zero-byte brief. This child owns the five SourceIR families, exact normalized-capture premise
  catalog, classification-from-unclassified projection, schema/proof compatibility, canonical load/write gating,
  SourceIR mutation authority, and conformance overlay separation. EvidenceIR remains the next clean child.
- `2026-08-12`: The first persisted-ledger integration found that digest/rule-id topology alone prevents stale
  accidental mutation but does not prevent a JSON editor from recomputing both a conclusion digest and its proof.
  Canonical authority therefore also requires executable registered-rule verification over exact premise and
  conclusion bytes, with the implementation digest committed into the ruleset hash. `.e.iv.ii` owns this kernel
  correction before SourceIR can ship; a hash-only or self-attesting ledger is not an acceptable partial result.
- `2026-08-12`: Closed `.6d.ii.e.iv.ii` after SourceIR schema 3 made all five families executable-proof-carrying,
  conformance fixture mutation moved behind noncanonical non-persistable overlays, and the exact 24 retained
  bundles migrated only after backing verification. Capture comparison found zero unintended mutations; complete
  downstream rebuild restored 24/24 measurable EvidenceIR and 78/78 later-stage exact currency with zero stale.
  The other 54 SourceIRs remain legacy inspection-only. Full Rust/KG/warning-deny qualification passes; `.e.iv.iii`
  is next only after this child commits cleanly.
- `2026-08-12`: Activated `.6d.ii.e.iv.iii` only after `.e.iv.ii` committed at `bb5047c2`, the post-commit
  tree was clean, and the commit brief was zero bytes. This child owns all 11 EvidenceIR families, deterministic
  builder extraction, prior/model proposal admission, convergence and post-build mutation, validation
  backannotation, cumulative SourceIR proof carry, and every canonical EvidenceIR load/serialize/write/downstream
  seam. SemanticIR remains the next clean child; no vocabulary-list result can substitute for executable proof.
- `2026-08-13`: `.e.iv.iii` replaces EvidenceIR hash/provenance trust with executable cumulative derivation.
  Exact SourceIR proof is preserved as the prefix; 39 field roots and every populated record/alias claim derive
  from verified SourceIR, captured normalized Markdown, an exact consulted prior, and closed ordered proposals.
  All productive enrichment/backannotation commands now authorize only their declared field set, and the old
  hidden carry-forward path is removed. Registry validation plus one global replay keeps claim verification linear.
- `2026-08-13`: The exact retained-corpus migration advances 24 EvidenceIRs to schema 3 with zero extracted-field
  delta and zero SemanticIR replay delta. The 54 historical EvidenceIRs remain inspection-only. Currency is exact
  at EvidenceIR 24 current / 54 bundle-unmeasurable, SemanticIR 24 current / 54 proof-unmeasurable, and IntentIR/
  adapters 78/78 stage-local with zero stale. The verified 131-MiB rollback and 465-MiB comparison workspaces were
  removed exactly with residue absent; later stage-local currency is not misrepresented as end-to-end proof.
- `2026-08-13`: Closed `.6d.ii.e.iv.iii` after all eight doctrines, the full 1,932-pass / six-ignore / zero-fail
  workspace plus five compile-fail doctests, 156/156 KG fixtures, warning-deny Clippy/rustdoc, mdBook, and final
  locality passed. The requested artifact sweep removed 6.7 GiB of rebuildable Cargo incremental cache and found
  no remaining `.bin`/`.log` in generated, release, or debug-deps. `.e.iv.iv` is next only after this commit is clean.
- `2026-08-13`: Activated `.6d.ii.e.iv.iv` only after `.e.iv.iii` committed at `1aa7f95d`, the post-commit tree
  was clean, and the commit brief was zero bytes. This child owns all 12 SemanticIR families, exact cumulative
  EvidenceIR prefix carry, registered carry/merge/conflict/synthesis/residual relations, post-build validation
  authority, compatibility, and every canonical SemanticIR load/serialize/write/IntentIR seam.
- `2026-08-13`: Closed `.6d.ii.e.iv.iv` after SemanticIR schema 2 made all 49 fields in 12 families executable-
  proof-carrying. The broad gate split filtered/extended timing, signal-constraint, and conditional-rule surfaces
  out of lossless carry into registered projection authority; each true carried root/record directly cites
  EvidenceIR, repository moves normalize before proof comparison, and all other conclusions retain the complete
  transitive upstream topology; validation is the only production mutation. Exact migration changes no semantic
  field. Currency is 24 current / 54 proof-unmeasurable through IntentIR, with all 78 adapters/ISF stage-locally
  current. `.e.iv.v` is next only after this child commits cleanly.
- `2026-08-13`: Activated `.6d.ii.e.iv.v` only after `.e.iv.iv` committed at `684b6214`, the post-commit tree
  was clean, and the commit brief was zero bytes. This child began from seven inventoried IntentIR families and
  owns all 49 public fields, exact cumulative
  SemanticIR prefix carry, canonical summary/behavior/relation/residual derivations, validation mutation,
  compatibility, and every canonical IntentIR load/serialize/write/adapter seam. It begins with an exact
  field-by-field carry-versus-projection audit so no filtered, synthesized, or extended surface can claim
  lossless authority. The audit split filtered actors and NLI-filterable actor contracts from exact carry,
  yielding nine capability-homogeneous families rather than preserving a misleading mixed family merely to keep
  the initial row count.
- `2026-08-13`: Closed `.6d.ii.e.iv.v` after exact replay made every IntentIR field and populated record
  proof-carrying, true carries gained immediate SemanticIR antecedents, actor/contract projections were kept
  distinct, and validation plus transactional NLI became closed mutations. Exact migration changes no
  non-validation product field across the 24 reachable chains; 54 legacy chains remain inspection-only. A
  test-only adapter fixture exposed and closed the need for separately compiled synthetic projection without
  weakening production. Full CI and 156/156 KG fixtures pass. `.e.iv.vi` is next only after this child commits
  cleanly; `.e.iv.vii` durably owns the surfaced whole-module digest precision debt.
- `2026-08-13`: Activated `.6d.ii.e.iv.vi` only after `.e.iv.v` committed at `e7eabdd8`, the post-commit tree
  was clean, and the commit brief was zero bytes. This child owns all four adapter families, exact cumulative
  IntentIR proof carry, registered target/lowering/residual conclusions, every adapter serialize/write/render
  seam, current artifact reconciliation, and complete 41-family chain qualification.
- `2026-08-13`: Adapter schema 2 now proves every public field, populated array record, nonblank rendered ISF
  line, and blocking reason from current verified IntentIR. Canonical build/load/serialize/write/reconciliation
  and validation backannotation execute the proof. Exact migration changes no pre-existing public field across
  the 24 reachable manifests; all are honestly blocked/no-file, while 54 upstream-legacy chains remain
  proof-unmeasurable. The audit also corrected a false chain-summary label: comparison was exact, but checked
  adapter states had been called emitted files; the gate now reports 24 states / zero files / 24 blocked states.
- `2026-08-13`: Closed `.6d.ii.e.iv.vi` after explicit test-only fixture persistence removed the last ordinary-
  writer ambiguity, exact adapter refresh and typed validation completed 24/24, all eight doctrines and the
  1,947-pass workspace plus 156/156 KG fixtures passed, and roadmap/research containment was restored without
  widening bounds. `.e.iv.vii` is next only after this child commits and the repository is clean.
- `2026-08-13`: Activated `.6d.ii.e.iv.vii` only after `.e.iv.vi` committed at `367353a5`, the post-commit tree
  was clean, and the commit brief was zero bytes. This child owns mechanical production-semantic implementation
  digests and the positive/adversarial proof that test, fixture, comment, and documentation edits are inert while
  every production proof-relation change stales canonical authority.
- `2026-08-13`: Froze the implementation-digest contract before proof-code edits. A build-time, schema-versioned
  compiler-token Merkle closure roots each stage at its canonical production registry constructor, recursively
  includes the selected production verifier, referenced stage-local production items and exact import bindings,
  plus the complete trusted-kernel token graph. It strips comments/docs/formatting and nested configured test or
  conformance branches, retains unknown platform configuration conservatively, and fails closed on an
  absent/ambiguous root. Current-binary stage replay remains the observational guard for builder/lowering
  dependencies outside this registry-rooted proof relation. In-memory controlled mutants prove
  comment/doc/test/fixture/formatting/unrelated-item invariance and referenced-helper/import sensitivity; runtime
  tests prove the generated closure is the one consumed by every stage registry.
- `2026-08-13`: The final binding-aware proof identity is installed for SourceIR, EvidenceIR, SemanticIR,
  IntentIR, and the adapter. Exact proof-only migration rebuilds 24 artifacts at each stage; after excluding proof
  context, proof ledger, and validation reports, all 120 comparisons are byte-identical. Complete chain replay is
  24 current / zero stale / 54 explicitly proof-unmeasurable at every downstream proof stage, with 24 blocked
  adapter states and zero emitted files. The exact 120-file / 964,137,330-byte same-volume rollback snapshot was
  removed and its path is absent. Final doctrine/workspace/book qualification remains before closure.
- `2026-08-13`: Closed `.6d.ii.e.iv.vii` and the `.e.iv` proof-migration parent after all eight doctrines,
  warnings-denied Clippy/rustdoc, 1,948 Rust tests / six ignores / zero failures, five compile-fail doctests,
  156/156 KG fixtures, mdBook test/build, exact rolling-ledger containment, and final locality passed. `.e.v` is
  next only after this child commits and the repository is clean; it owns whole-core AST information-flow
  enforcement rather than widening the completed registry-rooted digest claim.

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
- `2026-08-12`: Activated `.6c.ii` only after `.6c.i` committed at `74a658b3` and the post-commit tree was clean;
  this leaf owns the new isolated population replay, exact comparable result, controller re-ranking, and cleanup.
- `2026-08-12`: Closed `.6c.ii` and parent `.6c` with 12/12 clean-revision replay, exact 24/5/16 current
  TP/FP/FN, complete 29/29 provenance, 72/88 conservation, strict controller v5 and mutants, complete cleanup,
  and full CI. The controller selects `.6d`, the three-record OpenCAPI analog fabrication family.
- `2026-08-12`: Closed `.6d.i` with the schema-closed decibel-domain disposition, exact two-chain ADR 0025
  reconciliation, actionable projection, residue-free cleanup, required catalog/ledger maintenance, and all
  eight doctrines plus 1,869 tests green. Frontier advances to `.6d.ii`, clean-revision population qualification.
- `2026-08-12`: Activated `.6d.ii` after the clean `b977a51f` handoff; no replay or result authority was touched
  before this task-tree transition.
- `2026-08-12`: Split `.6d.ii` into `.a`–`.f` after the owner rejected corpus-specific production knowledge and
  finite-token enforcement. `.a` owns the completed replay publication, full-pipeline audit, historical
  correction, feasible neutral-system boundary, and first production/conformance module separation; `.b` is the
  next frontier only after `.a` commits cleanly.
- `2026-08-12`: Closed `.6d.ii.a` with the exact 24/2/16 replay authority, complete 71-source audit, corrected
  historical claim, structural-plus-behavioral signoff design, `#[cfg(test)]` reviewed snapshot composition,
  deterministic catalog/aggregate maintenance, residue-free cleanup, and full CI. Parent `.6d.ii` remains open;
  after this commit is clean, frontier advances to `.6d.ii.b` SourceIR/Docling neutralization.
- `2026-08-12`: Activated `.6d.ii.b` from the clean `6b155380` handoff. No SourceIR code changed before the
  transition; the first step is to classify every executable embedded-Python decision and named production
  comment, then replace corpus phrases with structural or current-document-derived evidence.
- `2026-08-12`: The first all-tier gate after schema-2 fail-closure found ten of 24 replayable EvidenceIR chains
  stale, exactly because they consume the retired SourceIR labels; all 78 later stages remain current against
  their persisted inputs. ADR 0025 requires this same leaf to back up, rebuild, compare, validate, and reconcile
  all ten cascades before commit. The neutral classifier will not be weakened to preserve those artifacts.
- `2026-08-12`: Reconciled all ten exact cascades under ADR 0025 after an 80-file / 39,847,014-byte same-volume
  backup. All 30 typed IR outputs validate, both renderable ISFs are pinned-FSMGen-strict clean, and exact
  current-binary equality is restored at 24/24 measurable EvidenceIR plus 78/78 SemanticIR, IntentIR, and
  adapters. Attribution records the 194 timing-record recall reset and every register/actor/visual collateral
  change; no document-specific classifier exception was introduced. Full signoff and cleanup then completed.
- `2026-08-12`: Closed `.6d.ii.b` after both renderable ISFs passed pinned FSMGen strict, all eight doctrines
  proved exact current-binary currency, 1,873 tests passed with six ignored and zero failed, rustdoc/mdBook/locality
  passed, and the exact 80-file rollback plus comparison script were removed with residue absent. After this
  commit is clean, frontier advances to `.6d.ii.c`; the measured recall losses remain owned generic recovery
  evidence and cannot justify restoring identity-specific authority.
- `2026-08-12`: Closed `.6d.ii.c` with generic EvidenceIR schema 2, document-derived protocol structure,
  fail-closed legacy named carriers, exact reconciliation of all 78 persisted chains, and full CI. The 24 retained
  replays intentionally retire unsupported frame/state claims rather than preserve named shortcuts. After this
  commit is clean, frontier advances to `.6d.ii.d`; production-core signoff remains blocked until `.d`–`.f` close.
- `2026-08-12`: Activated `.6d.ii.d` only after `.6d.ii.c` committed at `89d8dee7` and the tree was verified
  clean. This leaf owns every remaining identity- or spelling-driven decision in priors, SemanticIR, prompts,
  validation, and corpus commands; exact names remain permissible only as input-derived data or test fixtures.
- `2026-08-12`: `.6d.ii.d.i` replaces named-family prior routing with schema-7 global scope, quarantines legacy
  identity-scoped semantic authority, migrates KG fixtures to neutral mismatch controls, validates and relearns
  13 current inputs to a byte-stable fixed point, and restores exact 24/24 plus 78/78 chain currency. ADR 0036
  owns the migration boundary; `.d.ii` is next only after this leaf commits cleanly.
- `2026-08-12`: Activated `.6d.ii.d.ii` only after `.d.i` committed at `4b8895d6` and the post-commit tree was
  clean. This leaf owns every production decision where an arbitrary identifier spelling can admit a signal or
  assign interface, handshake, clock, reset, or polarity semantics; tests remain conformance-only. The complete
  production census found an additional downstream breach in `ir/isf_ir.rs`: adapter lowering selects
  clock/reset signals from `clk`/`clock`/`rst`/`reset` substrings, overrides explicit reset timing from `_n`/`_b`,
  and otherwise manufactures conventional clock/reset defaults. `.d.ii` owns that adapter path too; closing only
  the originally recorded SemanticIR and validation sites would leave alpha-renaming observably false.
- `2026-08-12`: The `.d.ii` production census found that spelling authority was broader than named-suffix
  heuristics: case-folded provenance keys and ambiguous case-insensitive catalog recovery could collapse two
  distinct current-document identifiers, while NLP enrichment could let a model-proposed signal synthesize the
  declaration that later purported to ground it. The repair therefore enforces one-way grounding and exact-first,
  unique-only identifier recovery; it deletes the circular declaration path. The future `.e` gate is strengthened
  from vocabulary/AST policing to proof-carrying promotion plus opaque-identifier information-flow and per-rule
  alpha-equivariance obligations. This is the durable architecture direction; vocabulary scanning remains only a
  diagnostic tripwire.
- `2026-08-12`: Director clarified the north star as domain-specialized but specification-instance-neutral:
  SpecForge must reason as a digital-chip-design colleague across protocols, ISAs, interfaces, registers, timing,
  state machines, and system infrastructure, produce justified SourceIR → EvidenceIR → SemanticIR → IntentIR, and
  lower supported executable intent to FSMGen ISF. Closed digital-design concepts and language grammar are required;
  vendor/family/document/identifier familiarity is never evidence. The `.d`–`.f` acceptance boundary uses this
  distinction so neutrality cannot be misimplemented as a domain-blind PDF parser.
- `2026-08-12`: Closed `.6d.ii.d.ii` with opaque identifier information flow and one-way current-document
  grounding from EvidenceIR through adapter lowering. Exact rollback attribution changes 18/73/74/74 stage
  artifacts, removes 2,891 ungrounded invariants and 42 name-created transactions, exposes 333 signal-neutral
  conditional rules, and changes honest ISF renderability 44→17; all 17 targets pass pinned FSMGen strict and
  exact chain currency remains green. ADR 0037 owns the boundary. After this commit is clean, `.d.iii` owns the
  still-compiled named prompt and corpus-organization surfaces; `.e`/`.f` remain signoff blockers.
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
