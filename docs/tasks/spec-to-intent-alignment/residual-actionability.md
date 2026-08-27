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
  State: `done`
  Goal: freeze the required-residual contract, typed record grammar, refusal matrix, and replay obligations
  Acceptance: a machine contract defines when a residual is *required* at SemanticIR and at IntentIR for each
  reviewed disposition, including the fail-closed rule that every canonical key missing at a stage requires one
  actionable residual and can never be silently excused; the typed residual grammar fixes source region,
  provenance ids, typed cause, first failing stage, and operator replay route; the bounded family, exact witness,
  positive/refusal cases, affected-chain scope, and 48-stage publication obligation are fixed before any
  production or evaluator change; the published reproduction command for this gap executes real tests
  Verification: `the machine contract executes the required-residual rule over 18 closed cases deriving 17
  required and five met observations across 19 control classes, re-derives the 4/8/12 decomposition and the
  selected family from the pinned current result, binds the four typed causes and the three actionability
  fields, and joins the repaired reproduction command to both published authorities; 21/21 mutations reject,
  three new Rust controls plus the byte-current snapshot pass, warning-denied Clippy and all five
  production-genericity components pass, and the workspace suite is 470/165/1,365/4 with zero failures`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.8a — freeze the required residual contract`

- ID: `SPEC-TO-INTENT-ALIGNMENT.8b`
  State: `done`
  Goal: count only required residuals in residual actionability without excusing a missing canonical fact
  Acceptance: the vertical evaluator's residual-actionability denominator admits a residual observation only
  where the review requires one — every stage of a residual or non-applicable cell, and every canonical key
  actually missing at a stage; a canonical cell whose facts are exact at a stage contributes no observation and
  gains no credit; controlled mutations prove that removing a canonical fact adds its required observation and
  fails it while no residual explains it, and that a residual duplicating a promoted canonical key is not
  counted as success; conservation, provenance, disposition, fabrication, and drop metrics are unchanged; the
  tracked result and controller authorities are republished byte-exact from the corrected evaluator
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.8a`
  Verification: `the corrected evaluator moves the pinned current result from 4/24 to 4/16 and changes no other
  global dimension and no per-cell field; the frozen .4b first result moves 0/24 to 0/82 with one changed line;
  three new controls prove an exact canonical stage requires nothing, a lost canonical fact adds a required and
  unmet observation, only an exact provenanced actionable residual meets it, and duplicate or unprovenanced
  residual sets credit nothing; the published global block is now gated as a current summary of its own cells;
  the controller republishes 4 of 16 with an affected population of 12; the workspace suite is
  470/168/1,365/4 green, Clippy is clean, and all five genericity components pass`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.8b — count only required residual observations`

`.8c` and `.8d` are owned by [residual carrier](residual-carrier.md), which was split from this part at that
task boundary when the combined part reached its declared line-count rollover milestone. Their decisions,
verification, and commit records stay below so the `.8` program keeps one chronology.

## Current Frontier

`SPEC-TO-INTENT-ALIGNMENT.8` is complete. The rule is frozen (`.8a`), the evaluator agrees with it (`.8b`), the
bounded typed carrier ships (`.8c`), and `.8d` has replayed the complete reviewed population and published the
comparable result: residual actionability is **8/16**, source-region disposition 10/14, required-modality
accounting 8/12, and provenance closure 45/45, with conservation 120/120, IntentIR 40/0/0, and zero fabrication
or unexplained drops. `platform-system-ip` is the third supported reviewed category. The measured closure and
its attribution live in [residual carrier](residual-carrier.md); the next root child is `.9`.

## Localized residual-actionability gap (`.8`)

The published `.7c.ii` authority is
`crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json`, SHA-256
`43603bbb3dde77c95929e26e1adf25709aec400ca3b9ae737e49e2fe1d07ceda`. Twelve of its 14 reviewed cells declare
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

## Frozen required-residual contract (`.8a`)

Machine authority is `doctrine/spec_to_intent/residual_actionability_contract.json`; its independent checker is
`python3 -B scripts/validate_residual_actionability_contract.py --check`. The contract is executable rather than
declarative: the checker implements the required-residual rule and re-derives every case's required and met
counts instead of trusting the declared numbers.

The rule counts one observation per reviewed cell per promoted stage per required residual. A `residual` or
`non_applicable` cell requires exactly one observation at SemanticIR and one at IntentIR. A `canonical` cell
requires one observation at a promoted stage for each reviewed canonical key that stage does not promote, and
none when the stage promotes every reviewed key. A missing canonical key therefore always *adds* a required
observation and can never leave the denominator: it is met only by an exact, provenanced, actionable residual
that explains that exact key. A residual whose fact key duplicates a key the same stage already promotes never
satisfies an observation, and a stage carrying one is credited with nothing. SemanticIR and IntentIR are counted
independently.

The typed record grammar keeps the reviewed `/residuals` collection, `region_id`/`family`/`fact_key` identity,
nonempty `/source_ids` provenance, and the three required actionability fields `/reason`,
`/first_failing_stage`, and `/replay`. A boundary value must be one of the three source-to-intent promotions.
Four closed typed causes are declared: `outside_executable_digital_domain`, which is already carried by
`TimingIntentDisposition::NonApplicable`; `no_canonical_carrier_for_captured_region`; `non_contract_region`;
and `unresolved_grounding`. The contract requires the other three to declare that no carrier exists yet, so a
future slice cannot silently claim shipped coverage. Authority is current-document structure and closed grammar
only; document, vendor, protocol, and review-label identity are never production selectors.

Eighteen closed cases derive 17 required and five met observations and cover 19 control classes: both
non-canonical dispositions at both stages, an exact canonical stage that requires nothing, one and two missing
canonical keys, an explained and an unexplained miss in the same cell, each missing actionability field, an
undeclared boundary value, absent provenance, partial and extra residual key sets, and a residual duplicating a
promoted canonical key. The self-test rejects 21 mutations, including relabeling a missing canonical fact as
residual success, dropping a missing key from the denominator, crediting a duplicate or inactionable residual,
relaxing the fail-closed or duplicate rule, widening the boundary vocabulary or typed causes, claiming a carrier
that does not exist, shrinking the witness, widening the selected family, and reverting the reproduction repair.
A final control mutates the current result so no residual is missing and requires the frozen witness to reject
it, so the witness cannot go quietly green.

`.8b` must make `summarize_global` agree with this rule; `.8c` must emit the typed record for the selected
family; `.8d` must replay all 12 reviewed sources through the 48 isolated stages before publication.

## Repaired reproduction command (`.8a`)

The gap's published reproduction was `cargo test -p specforge --lib ir::source_to_intent_eval`. The evaluator
module lives in `specforge-conformance` and reaches `specforge` only through the `specforge::ir` compatibility
facade, so that command selected zero tests and exited zero — a silent green on this leaf's own evidence path.
The repaired command is `cargo test -p specforge-conformance --lib ir::source_to_intent_eval`, which runs 14.

The repair is now structural rather than a corrected literal. `crates/specforge/src/test_support/trajectory_snapshot.rs`
derives the conformance-owned test roots from that crate's own `pub mod` declarations through `include_str!`,
resolves the owning package for a composed filter, and fails controller-input composition when any published gap
reproduction is not an executable `cargo test -p <package> --lib <filter>` form for its owning package. Three
tests cover it: the derived-root check, the whole-snapshot check that every published gap reproduction selects
real tests, and a RED matrix that rejects the exact facade-package defect plus four unsupported command shapes.
Both tracked authorities were regenerated; their only change is that one field in three places.

## Corrected required-residual accounting (`.8b`)

`summarize_global` no longer adds two observations for every cell that *declares* residual queries. A new
`required_residual_observations` helper applies the frozen rule per cell per promoted stage: one observation for
a residual or non-applicable cell, and one per reviewed canonical key the stage fails to promote for a canonical
cell. A stage credits an observation only when its residual query is exact for the required keys, carries the
reviewed provenance, is actionable, and does not duplicate a key the same stage already promotes.

The change is confined to the aggregate. Regenerating the frozen `.4b` first result changes exactly one line —
its residual denominator moves from 24 to 82 — while every per-cell score, boundary, hard failure, and every
other global dimension stays byte-identical. The pinned `.7c.ii` current result moves from 4/24 to 4/16 by the
same isolated mechanism, and the controller now publishes `4 of 16 required residual observations are
actionable` with an affected population of twelve rather than twenty.

Because the published report is a derived aggregate over its own cell results, it was re-summarized in place
rather than replayed. That rewrite is auditable, not silent: the `.7c.ii` population replay evidence now keeps
the digest the replay itself produced alongside the published digest and names `.8b` as the re-summarizer, and
the composer refuses a re-summarized result that does not record both. A new gate also fails whenever the
published global block stops being a current summary of its own cells.

Four controls hold the accounting honest. A canonical cell exact at a stage contributes nothing. Losing that
canonical fact adds a required observation that no residual explains, so the loss cannot leave the denominator.
Only an exact, provenanced, actionable residual for that same key meets it. A residual set that duplicates a
promoted key, or that carries no reviewed provenance, credits nothing at that stage. A direct unit case also
proves partial explanation: four required observations across two stages with two explained.

The trajectory composer keeps an independent cross-check rather than calling the evaluator: it derives the
required denominator from the canonical stage false negatives, a different field than the evaluator's
matched-key path.

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

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.8a`

- [x] **REPRODUCE / MEASURE** — `cargo test -p specforge --lib ir::source_to_intent_eval` reports
  `test result: ok. 0 passed; 0 failed; ...; 470 filtered out` and exits `0`, while
  `cargo test -p specforge-conformance --lib ir::source_to_intent_eval` reports `14 passed`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/test_support/trajectory_snapshot.rs:1234` and
  `:1260` composed `-p specforge` for filters rooted at `ir::source_to_intent_eval`, whose module is declared in
  `crates/specforge-conformance/src/ir.rs` and only re-exported by `crates/specforge/src/lib.rs:9`; the
  `specforge` `--lib` binary therefore contains 470 tests and none of the evaluator's 14.
- [x] **ADDRESSED (verified)** — the composer now derives conformance-owned roots from the downstream crate and
  rejects a non-executable reproduction before composition; both published authorities change from
  `cargo test -p specforge --lib ir::source_to_intent_eval` to
  `cargo test -p specforge-conformance --lib ir::source_to_intent_eval` in three places and nothing else. The
  contract checker reports `12 declaring cells / 24 declared = 4 actionable + 8 not-required + 12
  required-absent; corrected 4/16; 18 rule cases derive 17 required / 5 met` and its self-test reports `21/21`.
- [x] **NO REGRESSION** — `cargo test --workspace --lib` is 470 / 165 / 1,365 / 4 passing with zero failures and
  eight ignored; `cargo clippy --workspace --all-targets --all-features -- -D warnings` is clean; all five
  production-genericity components pass; `persisted_controller_input_and_report_are_byte_current` passes after
  regeneration; the derived flow oracle is refreshed to 2,363 functions / 14,639 helper edges / 12,631 decision
  sites / 1,462 macros with graph derivation and production behavior unchanged.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — the contract's authority rule forbids document, vendor, protocol,
  and review-label identity as production selectors, its typed causes are structural, and the new control reads
  crate module wiring rather than any document or protocol name.
- [x] **LOCKSTEP** — the contract, checker, composer, both published authorities, this part, the bounded root,
  the resume pointer, and the live status agree that `.8b` is next.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.8b`

- [x] **REPRODUCE / MEASURE** — before the change the published result reported
  `residual_actionability { met: 4, total: 24 }` while the frozen `.4b` result reported `0 / 24`; the frozen
  `.8a` contract independently derived the corrected `4 / 16`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `summarize_global` in
  `crates/specforge/src/ir/source_to_intent_eval.rs` executed `residual_total += 2` for every cell that declared
  residual queries, so four canonical cells exact at all three promoted stages contributed eight permanently
  unmet observations, and a canonical cell losing fifteen keys contributed the same two as one losing one.
- [x] **ADDRESSED (verified)** — `required_residual_observations` now derives the denominator from the frozen
  rule. Regenerating the frozen `.4b` result changes exactly one line (`"total": 24` → `"total": 82`) and the
  pinned current result changes two (`24` → `16`, ratio `0.1666…` → `0.25`); no per-cell field moves in either.
  The controller republishes `4 of 16 required residual observations are actionable` with
  `affected_population: 12`, and `python3 -B scripts/validate_residual_actionability_contract.py --check` still
  derives the same decomposition against the re-summarized authority.
- [x] **NO REGRESSION** — `cargo test --workspace --lib` is 470 / 168 / 1,365 / 4 passing with zero failures;
  `cargo clippy --workspace --all-targets --all-features -- -D warnings` is clean; all five
  production-genericity components pass; the `.8a` contract self-test still rejects 21/21 mutations; the derived
  flow oracle is refreshed to 2,364 functions / 14,642 helper edges / 12,643 decision sites / 1,462 macros.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — the new helper reads only reviewed disposition, canonical stage
  scores, and residual query scores; it names no document, vendor, protocol, family, or modality.
- [x] **LOCKSTEP** — the evaluator, both result authorities, the replay evidence, both controller authorities,
  the frozen contract, the mdBook contract and trajectory chapters, this part, the bounded root, the resume
  pointer, and the live status agree that the published ratio is `4/16` and that `.8c` is next.

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
- `2026-08-27`: make the contract executable. The checker implements the required-residual rule and re-derives
  each case's counts, so `.8b` has one machine definition to satisfy instead of prose to interpret.
- `2026-08-27`: refuse credit for a whole stage that carries a residual duplicating a promoted canonical key.
  A per-record rule would still let an internally contradictory artifact earn partial credit.
- `2026-08-27`: require the three not-yet-built typed causes to declare `existing_carrier: null`. This keeps a
  later slice from claiming shipped coverage by renaming an existing carrier.
- `2026-08-27`: repair the reproduction command structurally. Deriving the owning package from the conformance
  crate's own module declarations makes the whole silent-green class fail closed, not just this one literal.
- `2026-08-27`: re-summarize the published current result in place rather than replaying the population. Its
  per-cell scores are the measurement and did not move; only the derived aggregate did, and a full replay needs
  external sources this slice does not touch.
- `2026-08-27`: record both digests in the replay evidence. A re-summarized publication that silently swaps the
  attested identity would erase the fact that the artifact is no longer byte-identical to its replay output.
- `2026-08-27`: republish the frozen `.4b` first result and correct its book row rather than leaving a stale
  historical number that the current evaluator no longer reproduces. The unit change is stated on the page.
- `2026-08-27`: correct `.8c`'s acceptance before writing its code. A producer that adds records cannot also
  leave zero public field delta; the honest requirement is zero stale chains and no field change other than the
  intended new residual records.
- `2026-08-27`: mark `.8d` blocked rather than pending. Eight reviewed sources, including both selected
  static-topology documents, are authorized external read-only inputs that are not on disk, so a population
  replay is not runnable and a partial replay must not be published as a population result.
- `2026-08-27`: build the `.8c` gate as the figure-side sibling of the existing table region accounting rather
  than a new mechanism. Coverage stays a provenance membership test over concrete collections, so the producer
  cannot fabricate and cannot key on any document, vendor, protocol, or review label.
- `2026-08-27`: exclude table-kind visual assets from the carrier. They already reach canonical carriers through
  the register, signal, and timing paths, and a residual for them would duplicate a promoted fact.
- `2026-08-27`: host the new field in the existing `semantic.residual` and `intent.residual` claim families
  rather than minting new ones. Their residual capability and topology obligation already describe it exactly.
- `2026-08-27`: locate the supplied external sources by exact SHA-256 rather than by filename. Digest identity
  is what the review locked; a name match would not prove the replay reads the reviewed bytes.
- `2026-08-27`: keep the supplied sources git-ignored under a repository-derived path instead of tracking them
  under `corpus/`. Promoting them to repository sources would rewrite review-locked selection authority and the
  digests pinned to it; the recorded portable id, byte count, and digest keep the copy reproducible.
- `2026-08-27`: widen `.8c` coverage to accept the `figure:<asset_id>` citation the figure-region contract
  producer emits, not only the item's evidence id. The frozen design named one citation form, but a second
  production path from a figure to a canonical carrier exists; residualizing a region it covered would assert
  something false, which the contract's own no-fabrication rule forbids. The change only removes false positives.
- `2026-08-27`: refuse statement-mediated coverage under either reading. A caption that reaches a canonical
  carrier does not mean the figure's content did. Admitting the mediation explains 110 of 1,089 captured regions
  when applied to the collections coverage already reads and 466 when applied to every collection carrying
  `supporting_statement_ids`; the reviewed `picture_0001` appears only in the second, so the reading a naive
  implementation would pick is the one that reports success on a cell the review requires to fail.
- `2026-08-27`: exclude `Unknown` visual kinds as well as table kinds. The table-side sibling already skips
  unclassified kinds because capture never established them as intent-bearing; accounting them here would assert
  a region the classifier did not find.
- `2026-08-27`: record the shipped carrier in the frozen `.8a` contract and make the checker prove it. Leaving
  `existing_carrier: null` after shipping would publish a falsehood; trusting the new value would reintroduce
  exactly the "claim shipped coverage" failure `.8a` guarded against, so a named carrier must now resolve to a
  real production declaration in both directions.
- `2026-08-27`: register the IntentIR field under `intent.residual` rather than as an exact carry. The value is
  a byte-identical clone, but the residual family's `current_only` rule requires the current implementation to
  rebuild and compare it instead of matching an upstream claim — the stricter obligation for a record whose
  whole meaning is that no carrier accepted the region.
- `2026-08-27`: interleave build and validate when rebuilding a retained chain, and record the rule where the
  failure message leads. A first rebuild pass built every stage and validated afterwards; `specforge validate`
  back-annotates as a registered mutation, so each IntentIR pinned the pre-validation SemanticIR ledger and
  failed closed on the exact-prefix check. The gate behaved correctly and the book already stated the rule, so
  the durable fix is a Knowledge Map card (`retained-chain-rebuild-order`) rather than a production change.
- `2026-08-27`: leave the fixture-side projection and the published `4/16` ratio to `.8d`. The evaluator reads
  the reviewed population's replayed stage snapshots, so moving the ratio requires the population replay `.8d`
  owns; publishing a projection change here would report movement no replay had measured.
- `2026-08-27`: give every production residual carrier its own projector in the reviewed fixture instead of one
  priority chain. A region two carriers could explain is then projected by both rather than silently by
  whichever branch is written first, and each projector states its own applicability.
- `2026-08-27`: attribute the `.8d` delta with a control leg rather than by comparing against the previous
  publication. Re-projecting the *same* replayed artifacts with the frozen pre-change builder separates what the
  fresh replay changed from what the projection changed; without it, `.8c`'s production carrier and `.8d`'s
  projection would be indistinguishable in the published movement.
- `2026-08-27`: publish the replay's own evaluator output byte for byte instead of re-summarizing it. `.8b` had
  to record two digests because it re-summarized a pinned result; `.8d` measures and publishes in one step, so
  the published result and its replay output are the same bytes and the evidence needs no second identity.
- `2026-08-27`: add a closed `state` to the frozen contract's selected family rather than deleting the block or
  loosening its check. The block describes one family across its whole life, and the state names which derived
  bucket the family must be found in — so a family can neither go quietly green nor stay declared as a gap it
  no longer is. Four new RED cases cover both wrong directions, an out-of-vocabulary state, and a closed family
  naming a cause whose carrier does not ship.
- `2026-08-27`: publish the Cortex-A76 source-region regression rather than re-anchoring the reviewed cell to
  make it pass. The reviewed anchor is a frozen review identity; silently moving it to `elem_00230` would make
  the fixture agree with whatever ingest currently emits, which is exactly the drift the anchor exists to
  detect. The honest result is 13/14 with an owning task.
- `2026-08-27`: retain the eight owner-supplied external sources while reclaiming the population and diagnostic
  scratch. They are 6.3 MiB against 3.0 GiB reclaimed, git-ignored, digest-pinned in the part, and they are what
  makes this leaf's own measurement re-runnable.

## Open Questions

- None for `.8`. The rule is frozen, the evaluator agrees with it, the bounded production carrier ships, and the
  population replay has measured it at 8/16. The four remaining required-and-absent cells — two prose
  non-contract regions, one table-of-contents region, and one packed programming structure — need carriers for
  the `non_contract_region` cause, which no leaf owns yet and which the frozen contract still declares
  unbuilt.

## Blockers

- None. Every `.8` child is complete. The replay did surface one condition outside this program's scope:
  reviewed cells are anchored on ordinal SourceIR element ids, and ingest is not reproducible across time, so
  the Cortex-A76 cell's anchor no longer resolves and exact source regions are 13/14.
  [`SOURCE-IR-REPRODUCIBILITY`](../SOURCE-IR-REPRODUCIBILITY.md) owns it.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-27` | `.8d` published population closure | all 12 reviewed sources and all 48 isolated stages replay from clean production at `483e525d` under unchanged reviewed authority; residual actionability moves 4/16 to 8/16, disposition 8/14 to 10/14, modality accounting 6/12 to 8/12, provenance 43/43 to 45/45, and `platform-system-ip` becomes the third supported category, while conservation stays 120/120, IntentIR stays 40/0/0, and fabrication and unexplained drops stay zero; a control leg re-projecting the same artifacts with the frozen pre-change builder reproduces 4/16 exactly, so exactly two cells moved and the whole delta is the projection's; the published result is the replay's own output byte for byte; the contract self-test is 28/28, the workspace suite is 470/168/1,369/4, Clippy is clean, all nine gate-tier doctrines pass, and 3,095 files / 1,158,476 KiB of population scratch plus 4,192 files / 1,903,200 KiB of diagnostic scratch are removed residue-free; the replay surfaced a reviewed-anchor regression (exact source regions 13/14) routed to `SOURCE-IR-REPRODUCIBILITY` |
| `2026-08-27` | `.8c` captured-region carrier | the producer emits one typed residual per captured figure-kind region no canonical record cites; the reviewed CoreSight chain carries `picture_0001` / `visual_0008` / `no_canonical_carrier_for_captured_region` / `evidence_to_semantic_ir`; two pre-change chains compared field-by-field differ only in `captured_region_residuals` (I2S 20, I2C 103) at both SemanticIR and IntentIR; the registry is 170 rules over 50 SemanticIR and 50 IntentIR fields; four focused positive/refusal tests, the workspace suite, Clippy, all five genericity components, and the 24/24 contract self-test pass |
| `2026-08-27` | `.8d` input supplied | all eight authorized external sources are located by exact SHA-256 in the sibling repository, copied to a repository-derived path on the same volume, and re-verified: eight of eight digests and byte counts match the reviewed lock, and the orchestrator's own map-path, absolute-path, basename, same-volume, and coverage predicates pass with zero missing and zero extra entries |
| `2026-08-27` | `.8c` contract correction and `.8d` boundary | the chain-currency baseline is 24 replayed / 24 current / zero stale at all four stages with exactly the declared retained bundles; `.8c` acceptance replaces an unsatisfiable zero-public-delta clause with zero stale plus intended-record-only change; `.8d` is blocked because eight reviewed sources, including both selected static-topology documents, are absent external read-only inputs |
| `2026-08-27` | `.8b` corrected accounting | the pinned current result moves 4/24 to 4/16 and the frozen first result 0/24 to 0/82 with no other global or per-cell change; four fail-closed controls plus a direct partial-explanation case pass; the published global block is gated as a current summary of its own cells; the re-summarization is recorded with both digests; the controller republishes 4 of 16 over an affected population of 12; the workspace suite, Clippy, all five genericity components, and the 21/21 contract self-test pass |
| `2026-08-27` | `.8a` contract freeze | the executable contract derives 17 required / five met across 18 cases and 19 control classes, re-derives the 4/8/12 decomposition and the selected family from the pinned current result, and joins the repaired reproduction command to both authorities; 21/21 mutations reject, three new controls and the byte-current snapshot pass, Clippy and all five genericity components pass, and the workspace suite is green with the refreshed 2,363 / 14,639 / 12,631 / 1,462 flow oracle |
| `2026-08-27` | `.8` activation | the tracked `.7c.ii` result decomposes into 4 actionable / 8 not-required / 12 required-and-absent residual observations across 12 declaring cells; the six hard-failing cells and their extra failures are exact; the published gap reproduction runs zero tests at exit zero while the conformance-crate command runs 14; all nine gate-tier doctrines pass |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.8d` | `SPEC-TO-INTENT-ALIGNMENT.8d — publish the reviewed population residual closure` | replay all 12 reviewed sources through all 48 isolated stages, attribute the whole delta with a frozen-projection control leg, and publish 8/16 with `platform-system-ip` supported |
| `.8c` | `SPEC-TO-INTENT-ALIGNMENT.8c — emit the captured-region residual carrier` | ship the typed captured-region residual on SemanticIR and IntentIR, prove the carrier claim in the frozen contract, and rebuild every retained chain to zero stale |
| `.8` | `SPEC-TO-INTENT-ALIGNMENT.8 — correct the carrier contract and record the replay blocker` | make `.8c`'s acceptance satisfiable, publish the chain-currency baseline, and block `.8d` on the absent external source map |
| `.8b` | `SPEC-TO-INTENT-ALIGNMENT.8b — count only required residual observations` | make the evaluator's denominator the frozen required-residual rule, keep every missing canonical key required and unmet, and republish the result, replay-evidence, and controller authorities |
| `.8a` | `SPEC-TO-INTENT-ALIGNMENT.8a — freeze the required residual contract` | freeze the executable required-residual rule, typed record grammar, closed case matrix, and selected family, and repair the silent-green reproduction command structurally |
| `.8` | `SPEC-TO-INTENT-ALIGNMENT.8 — activate required residual actionability` | localize the declared-versus-required denominator defect and the absent typed residual carrier, select the bounded family, and route `.8a`–`.8d` |

## Update protocol

Every child updates this part and the bounded root together. A measured-metric or route change updates the
containment contract, index, and manifest in the same commit. The completed canonical-recovery part, the frozen
`current-and-future` source regions, and the exact source capsule remain unchanged.
