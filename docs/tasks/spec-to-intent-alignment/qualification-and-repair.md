# SPEC-TO-INTENT-ALIGNMENT — qualification and repair

- Part ID: `qualification-and-repair`
- State: `legacy`

<!-- spec-to-intent-task-source-region:qualification-nodes-06-06d-i:start -->
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

<!-- spec-to-intent-task-source-region:qualification-nodes-06-06d-i:end -->

<!-- spec-to-intent-task-source-region:acceptance-06a-06d-i:start -->
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

<!-- spec-to-intent-task-source-region:acceptance-06a-06d-i:end -->
