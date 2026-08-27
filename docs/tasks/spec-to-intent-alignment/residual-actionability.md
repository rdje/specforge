# SPEC-TO-INTENT-ALIGNMENT — residual actionability

- Part ID: `residual-actionability`
- State: `active`

## Active residual actionability

- ID: `SPEC-TO-INTENT-ALIGNMENT.8`
  State: `active`
  Goal: make required promotion-loss residuals typed, source-linked, and actionable
  Acceptance: a bounded residual family gains exact source linkage, typed cause, and operator action at
  SemanticIR and IntentIR; the vertical evaluator records the disposition without relabeling missing canonical
  facts as residual success
  Verification: `activation only — the exact 24-observation decomposition, the six hard-failing cells, the
  selected bounded family, and the broken published reproduction command are reproduced from tracked authority
  without changing production`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.8 — activate required residual actionability`
  Children: `.8a`, `.8b`, `.8c`, `.8d`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8a`
  State: `pending`
  Goal: freeze the required-residual contract, typed record grammar, refusal matrix, and replay obligations
  Acceptance: a machine contract defines when a residual is *required* at SemanticIR and at IntentIR for each
  reviewed disposition, including the fail-closed rule that every canonical key missing at a stage requires one
  actionable residual and can never be silently excused; the typed residual grammar fixes source region,
  provenance ids, typed cause, first failing stage, and operator replay route; the bounded family, exact witness,
  positive/refusal cases, affected-chain scope, and 48-stage publication obligation are fixed before any
  production or evaluator change; the published reproduction command for this gap executes real tests
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8b`
  State: `pending`
  Goal: count only required residuals in residual actionability without excusing a missing canonical fact
  Acceptance: the vertical evaluator's residual-actionability denominator admits a residual observation only
  where the review requires one — every stage of a residual or non-applicable cell, and every canonical key
  actually missing at a stage; a canonical cell whose facts are exact at a stage contributes no observation and
  gains no credit; controlled mutations prove that removing a canonical fact adds its required observation and
  fails it while no residual explains it, and that a residual duplicating a promoted canonical key is not
  counted as success; conservation, provenance, disposition, fabrication, and drop metrics are unchanged; the
  tracked result and controller authorities are republished byte-exact from the corrected evaluator
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.8a`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8c`
  State: `pending`
  Goal: emit the bounded typed, source-linked, actionable residual family across SemanticIR and IntentIR
  Acceptance: a structurally gated producer emits one typed residual for a captured source region of the
  selected family that reaches no canonical carrier, carrying its exact region and evidence provenance, typed
  cause, first failing stage, and operator replay route; authority is document structure and closed grammar
  only, never document, vendor, protocol, or review-label identity; a region with a canonical carrier emits no
  residual, and no residual duplicates a promoted canonical key; focused positive/refusal tests, the full core
  suite, warning-denied Clippy, and all production-genericity components pass; every proof-affected retained
  chain is rebuilt to zero stale with zero public field delta
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.8b`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8d`
  State: `pending`
  Goal: replay the complete reviewed population and publish comparable residual-actionability closure
  Acceptance: all 12 reviewed sources and all 48 isolated stages replay from clean production under the frozen
  oracle; exact canonical, provenance, conservation, residual, disposition, category, and controller deltas are
  attributed; no reviewed canonical true positive is lost and no fabrication or unexplained drop appears;
  tracked replay, result, controller-input, and report authorities reproduce byte-for-byte; selected CI, mdBook,
  retrieval truth, task parents, cleanup, and residue census agree
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.8c`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

Active frontier: `SPEC-TO-INTENT-ALIGNMENT.8a`. It freezes the required-residual contract and typed grammar
before `.8b` corrects accounting, `.8c` changes production, and `.8d` replays and publishes.

## Localized residual-actionability gap (`.8`)

The published `.7c.ii` authority is
`crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json`, SHA-256
`167980b369df71e67f068a3354f8881d66a7c303a3480c8ff01ac2194fd481e1`. Twelve of its 14 reviewed cells declare
residual queries, and the evaluator adds two observations per declaring cell, so the published denominator is
24. That denominator is *declared*, not *required*, and it decomposes exactly three ways.

| Class | Observations | Cells | State |
| --- | ---: | ---: | --- |
| Actionable today | 4 | 2 | both OpenCAPI `analog_channel_loss` cells |
| Not required at all | 8 | 4 | canonical cells whose facts are exact at all three stages |
| Required and absent | 12 | 6 | no residual record is produced at either promoted stage |

The eight not-required observations come from `setup_signal_state` (APB, one key), `receiver_timing` (I2S, five
keys), `register_summary` (GIC-400, 15 keys), and `debug_register_summary` (Arm Debug, 12 keys). Each cell's
review declares `residual_gold` equal to its canonical gold, so the residual query exists as the conservation
fallback that explains a canonical key if it is ever lost. All four cells are currently exact at EvidenceIR,
SemanticIR, and IntentIR with zero false positives, zero false negatives, and zero unprovenanced records, so no
key is lost and no residual is required. Satisfying those observations while the facts stay canonical would
require emitting a residual whose `fact_key` duplicates a promoted canonical fact — asserting in one artifact
that the same fact both reached and did not reach `IntentIR`. That is precisely the relabeling this leaf's
acceptance forbids, so the eight observations are unsatisfiable by construction, not a production deficit.

The 12 required-and-absent observations are the real gap. Their six cells are the complete current hard-failure
set, and every one of them carries `required_residual_missing_or_inactionable` together with
`source_region_disposition_unaccounted`:

| Document | Family | Modality | Disposition | Extra hard failure |
| --- | --- | --- | --- | --- |
| GIC overview guide | `informational_disclaimer` | prose | `non_applicable` | none |
| Cortex-A76 optimization guide | `software_guidance` | prose | `non_applicable` | none |
| RISC-V AIA | `table_of_contents` | table | `non_applicable` | `required_modality_capture_failed` |
| AMD IOMMU | `packed_page_table_entry` | table | `residual` | `required_modality_capture_failed` |
| RISC-V IOMMU | `static_component_topology` | figure | `residual` | none |
| CoreSight BSA | `static_component_topology` | figure | `residual` | none |

Production already owns exactly one typed residual carrier: `TimingIntentDisposition::NonApplicable` in
`crates/specforge-core/src/ir/source.rs` carries `quantity_domain`, `reason`, `first_failing_stage`, and
`replay` on a scalar timing row, and it is why the two OpenCAPI analog cells pass. No equivalent carrier exists
for a captured prose statement, table region, or visual region that reaches no canonical `IntentIR` surface, so
those regions disappear without an explanatory record.

## Selected bounded family (`.8`)

`.8c` implements `static_component_topology` first. It is the only single family whose closure can move a whole
reviewed category: both of its cells are the two `platform-system-ip` documents, both fail on the residual gap
alone, and both would clear their remaining hard failures. It is one family, one modality, one category, and
four of the 12 required-and-absent observations. The book already documents the underlying reason — static
component topology, hierarchy, clock domains, and asynchronous-domain relationships have no complete canonical
carrier — so an explicit residual is the correct honest outcome rather than a stopgap for a missing extractor.

The two prose non-contract families would close `methodology-guide` for the same cost but span two families, and
the `table_of_contents` and `packed_page_table_entry` cells cannot clear their categories from residual work
alone because each also fails required-modality capture. They stay ranked behind the selected family.

## Found defect — the published reproduction command does not reproduce (`.8`)

`crates/specforge/test_data/trajectory/trajectory_report.json` and its controller input publish this gap's
`reproduction` as `cargo test -p specforge --lib ir::source_to_intent_eval`. The evaluator module lives in
`specforge-conformance` and is only re-exported through the `specforge::ir` compatibility facade, so that
command matches no test:

```console
$ cargo test -p specforge --lib ir::source_to_intent_eval
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 470 filtered out
$ echo $?
0
$ cargo test -p specforge-conformance --lib ir::source_to_intent_eval
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 151 filtered out
```

A published reproduction that exits zero while running nothing is a silent green: a reviewer following the
report is told the gap reproduces when nothing executed. The same wrong crate is composed for the `.7` gap in
`crates/specforge/src/test_support/trajectory_snapshot.rs`. `.8a` owns the repair and a control that rejects a
composed reproduction command which selects zero tests.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.8` activation

- [x] **REPRODUCE / MEASURE** — the tracked `.7c.ii` result decomposes exactly into 4 actionable, 8 not-required,
  and 12 required-and-absent residual observations, and its six hard-failing cells are enumerated with their
  exact families, modalities, dispositions, and additional failures.
- [x] **ROOT CAUSE (WHY + WHERE)** — `summarize_global` in
  `crates/specforge/src/ir/source_to_intent_eval.rs` adds two observations for every cell that *declares*
  residual queries, so a canonical cell that is exact at every stage permanently contributes two unmet
  observations; separately, no production carrier emits a typed residual for a captured prose, table, or visual
  region that reaches no canonical `IntentIR` surface.
- [x] **ADDRESSED (verified)** — activation only. No production, evaluator, fixture, result, or controller
  authority changes in this slice; the repair is decomposed into `.8a` contract, `.8b` accounting, `.8c`
  production carrier, and `.8d` population replay.
- [x] **NO REGRESSION** — `cargo test -p specforge-conformance --lib ir::source_to_intent_eval` passes 14 tests
  and `scripts/check_doctrines.sh` passes all nine gate-tier doctrines before and after the documentation change.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — the selected family is named only as review-locked oracle scope.
  `.8c`'s acceptance requires structural, closed-grammar authority and forbids document, vendor, protocol, or
  review-label identity from reaching production.
- [x] **LOCKSTEP** — the task root, roadmap, live status, resume pointer, mdBook trajectory chapter, and
  Knowledge Map publish the same decomposition and route `.8a` next.

## Decisions

- `2026-08-27`: treat the published 4/24 as a *declared* denominator, not a required one. The contract's
  denominator is "all residuals", and a canonical fact that is correctly promoted has no residual to describe.
- `2026-08-27`: correct the accounting before changing production. Measuring the production repair against a
  denominator that contains eight unsatisfiable observations would understate the repair and leave the honest
  target undefined.
- `2026-08-27`: keep the correction fail-closed. A canonical key missing at a stage must *add* a required
  observation and fail it unless an exact actionable residual explains it, so the correction can never convert a
  recall loss into residual success.
- `2026-08-27`: select `static_component_topology` as the first bounded production family. It is the only single
  family that can move a reviewed category to `supported`, and its missing canonical carrier is already
  published, so an explicit residual is the correct terminal outcome rather than a placeholder.
- `2026-08-27`: own the broken reproduction command inside `.8a` rather than reporting it. A published command
  that exits zero without running a test is a measurement-integrity defect on this leaf's own evidence path.

## Open Questions

- None. The gap is localized, the denominator defect is exact, and the bounded family is selected.

## Blockers

- None. The activation changes no production or measurement authority.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-27` | `.8` activation | the tracked `.7c.ii` result decomposes into 4 actionable / 8 not-required / 12 required-and-absent residual observations across 12 declaring cells; the six hard-failing cells and their extra failures are exact; the published gap reproduction runs zero tests at exit zero while the conformance-crate command runs 14; all nine gate-tier doctrines pass |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.8` | `SPEC-TO-INTENT-ALIGNMENT.8 — activate required residual actionability` | localize the declared-versus-required denominator defect and the absent typed residual carrier, select the bounded family, and route `.8a`–`.8d` |

## Update protocol

Every child updates this part and the bounded root together. A measured-metric or route change updates the
containment contract, index, and manifest in the same commit. The completed canonical-recovery part, the frozen
`current-and-future` source regions, and the exact source capsule remain unchanged.
