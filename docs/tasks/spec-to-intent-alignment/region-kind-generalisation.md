# SPEC-TO-INTENT-ALIGNMENT — region-kind generalisation

- Part ID: `region-kind-generalisation`
- State: `active`

This part owns `SPEC-TO-INTENT-ALIGNMENT.9`'s generalisation of the captured-region residual carrier
from figures to the remaining captured region kinds — `.9b` (tables, shipped), `.9c` (prose), and `.9d`
(the frozen contract's re-pin and gate). It was split from
[residual carrier](residual-carrier.md) at the `.8`/`.9` task boundary when that part reached its
declared line-count rollover milestone; the closed `.8c`/`.8d` carrier and population-replay evidence
stays there, and the `.9a` re-derivation that routes here stays in
[residual actionability](residual-actionability.md).

## Owned leaves

- ID: `SPEC-TO-INTENT-ALIGNMENT.9b`
  State: `done`
  Goal: generalise the captured-region residual carrier from figures to captured table regions
  Acceptance: the `.8c` producer's region population widens to `VisualAssetKind::TableRegion` with a coverage
  test expressed in the vocabulary a table region's own canonical records actually use, so a table that reached
  a register, timing, polarity, or interface-signal carrier is never residualised and a table that reached none
  earns exactly one typed residual carrying its region identity, `EvidenceIR` provenance, the closed cause
  `no_canonical_carrier_for_captured_region`, boundary `evidence_to_semantic_ir`, and an operator replay route;
  every figure-kind residual already shipped by `.8c` is byte-unchanged; the reviewed vertical fixture projects
  table-kind regions; authority stays structural, with no document, vendor, protocol, or review label reaching
  production; focused positive/refusal tests, the workspace suite, warning-denied Clippy, all
  production-genericity components, and the frozen residual contract self-test pass
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.9a`
  Verification: `the producer emits one typed residual per captured table region no canonical record cites, with
  coverage read from `supporting_table_ids` over every SemanticIR surface that declares it; the reviewed
  AMD `table_0067` is uncited at all three stages in the published result, so it earns the residual the reviewed
  cell requires. Seven focused controls pass, including both refusal directions — the visual vocabulary never
  explains a table region and the table vocabulary never explains a visual one, which is what makes every
  figure-kind residual `.8c` published bit-identical. The gatherer-completeness control gives each of the seven
  declared table-provenance surfaces a distinct id and cross-checks the typed gatherer against the artifact's own
  serialized provenance. `cargo test --workspace --lib` is 470 / 168 / 1,372 / 4 with zero failures, `cargo fmt`
  is clean, all five production-genericity components pass at an unchanged 41 families / 170 field rules, and the
  compiled flow census moves only its size dimensions — 2,370 to 2,373 functions, 14,681 to 14,684 helper edges,
  12,656 to 12,669 decision sites, 1,464 to 1,466 semantic macros — while every boundary, rule-root, seam,
  proof-gate, trusted-region and protected-type dimension is unchanged. All 24 proof-carrying chains rebuild
  through semantic, intent, and the ISF adapter. The frozen residual contract is red at this commit and `.9b` did
  not move it: the same two failures reproduce with `.9b`'s production and fixture changes reverted, and `.9d`
  owns the cause`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.9b — generalise the captured-region carrier to captured table regions`

- ID: `SPEC-TO-INTENT-ALIGNMENT.9d`
  State: `done`
  Goal: re-pin the frozen residual-actionability witness and put its executable contract under a gate
  Acceptance: the `.8a` contract's `witness` pins the tracked current result again and its
  `required_and_absent_cells` re-derive from it, with the change limited to what
  `SOURCE-IR-REPRODUCIBILITY.2` actually repaired and no published count moved; the checker is registered in
  `scripts/check_doctrines.sh` so that a future republication of the current result cannot leave the frozen
  contract stale without failing a gate; its self-test is green and its RED matrix covers the exact staleness
  this leaf repairs
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.9b`
  Verification: `the staleness is attributed by re-deriving at each revision from its own producer, never from a
  diff: over the only two commits that touched either authority since the freeze, the witness digest MATCHES at
  893c2fba and MISMATCHES at 245b3b60, and the contract's own decomposition run against each revision's own
  snapshot is identical at the freeze and differs at the repair in exactly one field —
  software_guidance.hard_failures losing source_region_missing_or_ambiguous. It had been red for 43 commits. The
  re-pin re-derives the three cell collections from the contract's own decompose_current_result rather than
  hand-editing them, and no published scalar moves: 12 declared cells, 24/8/8/8 observations and corrected 16/8
  are all unchanged, so the diff is exactly the identity plus that one hard-failure entry. --check now passes
  and re-derives the published 8/16; the self-test is 28/28. Registered gate-tier at 0.04s, and proven
  fail-closed in both shapes: a drifted witness identity makes the driver report FAIL and block the commit, and
  a silently edited published count is rejected with the derived value named; the contract restores byte-exact
  after both. The driver registry and the DOCTRINE_ENFORCEMENT.md section-10 mirror are verified in lockstep at
  13 entries in the same order`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.9d — re-pin the frozen residual contract and put it under a gate`

- ID: `SPEC-TO-INTENT-ALIGNMENT.9c`
  State: `pending`
  Goal: carry the SourceIR prose region identity into EvidenceIR and generalise the carrier to prose
  Acceptance: `EvidenceIR` records, for every extracted statement, the `SourceIR` content-element identity the
  statement was split from, so a prose residual can name the region the reviewed cell anchors on; the carrier
  then emits one typed residual per captured intent-bearing prose statement no canonical record cites, with the
  fallback statement class excluded on the same "capture never established the region as intent-bearing" rule
  that already excludes `VisualAssetKind::Unknown`; the schema move is reconciled across every retained chain
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.9b`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.9e`
  State: `active`
  Goal: re-derive the carried-forward findings, withdraw what does not hold, and own the reviewed-label repair
  Acceptance: every statement `.9b` and `.9d` carried forward for owner judgement is re-derived from the tracked
  authorities and either stands with its evidence or is withdrawn in place, naming what was asserted rather than
  derived; the containment arithmetic is re-derived from the checker's own report instead of restated; and the
  surviving finding — that `table_of_contents` and `informational_disclaimer` expect residual labels
  (`table_0004|toc_non_contract`, `elem_00017|informational_non_contract`) that no non-circular projection can
  produce — gains an owner here rather than remaining a report. The label repair itself must show that the
  reviewed key is redundant with the cell's own region-and-family predicate before normalising it, and must be
  proved metric-neutral at the pre-change baseline
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.9d`
  Verification: `the re-derivation half is complete and is what this leaf publishes: two of the three
  carried-forward statements do not hold. "informational_disclaimer is unreachable by ANY structural rule" is
  false — a positional predicate is structural and reaches it (body_text on the first two pages selects nine
  elements including elem_00017); the defensible claim is that the CARRIER's intent-bearing test cannot reach it.
  "One further leaf route crosses mandatory rollover" is false — the index is 114 of a 128-line target, so one
  row is 89.84% and legal and two are 90.62%; and .9c was already declared, so it was never blocked. The third
  statement stands and is re-derived here: restricted to the four required-and-absent cells, whose only carrier
  is region-scoped and whose key law is <region_id>|<family>, two follow it and two carry hand-written cause
  labels no projection can produce. The label repair is this leaf's remaining open half`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.9e — re-derive the carried-forward findings and withdraw the two that do not hold`

## Measured generalisation design (`.9b`, before implementation)

`.9a` fixed the rule: one typed residual per captured region that no canonical record cites, asked of table
regions and prose statements as well as figures. Measuring that rule against the retained corpus *before*
implementing it establishes three things the pinned result alone could not show, and splits `.9` accordingly.

**The naive widening would residualise regions that already reached a carrier.** `.8c` decides coverage by
asking whether a canonical `SemanticIR` record cites the region's `evidence_id` or its `figure:<asset_id>`
form. No table-derived record ever cites either: table provenance in this pipeline is `supporting_table_ids`,
carried by `RegisterRecord`, `TimingConstraintRecord`, `ConditionalRuleRecord`, `SignalPolarityRecord`, and
the interface-signal records. Lifting the `TableRegion` exclusion without changing the coverage vocabulary
therefore reports *every* captured table region as unexplained — 354 of 354 in the AMD IOMMU chain and 210 of
210 in the Arm Debug chain — including the tables that produced the reviewed register and timing facts. That
is precisely the "a residual duplicating a promoted canonical key" self-contradiction the frozen `.8a`
contract forbids. Coverage for a table region must be asked in the table-provenance vocabulary, which is what
`.9b` builds; measured that way the current Arm Debug chain resolves to 16 cited and 194 unexplained of 210.

**The prose leg is blocked upstream, and not by anything in this program.** `EvidenceIR` carries no `SourceIR`
content-element identity at all: `elem_\d+` occurs zero times in a complete `evidence_ir.json`, `EvidenceSpan`
records only `span_id` plus page/line, and `ExtractedStatement` records only `statement_id`. The reviewed
prose cells anchor on `elem_00219` and `elem_00017`, so a prose residual produced at the
`evidence_to_semantic_ir` boundary cannot name the region the review requires. `.9c` owns closing that
provenance gap before the prose carrier can exist.

**Two of the four required-and-absent observations are unsatisfiable as written.** The scope matters and is
narrower than it first looks. A *canonical* cell's residual key is correctly the canonical fact key, because
`.8b`'s rule meets a missing canonical key only with a residual for that same key; and the two
`analog_channel_loss` cells are served by the fact-scoped `project_non_applicable_timing` carrier, whose key is
a timing key. Neither is region-scoped and neither is affected. Restricted to the four required-and-absent
cells, whose only possible carrier is the region-scoped one and whose key law is therefore
`<region_id>|<reviewed family>`, two follow that law — `table_0067|packed_page_table_entry` and
`elem_00219|software_guidance` — and two do not: `table_of_contents` expects `table_0004|toc_non_contract` and
`informational_disclaimer` expects `elem_00017|informational_non_contract`, hand-written review labels naming a
*cause* rather than the cell's own family. No non-circular projection can produce either spelling, so those two
observations cannot be met by any correct implementation. This is the `.8b` defect class — an observation that
stays in the denominator while nothing can satisfy it — and it gets its own leaf rather than being quietly
repaired inside a production slice. Re-derive with the reviewed dataset's `residual.semantic_ir.expected_keys`
against each cell's own region predicate and `semantic_family`.

**And one of those two cells is out of reach of the carrier's own rule.** `informational_disclaimer` anchors on
`statement_0013`, whose class is `source_fact` — the bucket `classify_statement` returns when no typed intent
shape is found, the prose analogue of `VisualAssetKind::Unknown` and of the unclassified table kinds
`unexplained_intent_bearing_tables` already skips. The carrier's population test is "capture typed this region
as intent-bearing", and capture did not, so the carrier cannot reach it. Admitting the fallback class instead
would emit one residual per uncited paragraph — in the current Arm Debug chain, 6,555 of 6,658 statements are
uncited, against 440 of 543 once the fallback is excluded.

*(`.9e` withdrew a stronger claim here. This paragraph first read "unreachable by **any** structural rule",
which does not hold and was asserted rather than derived: a purely positional predicate is structural, reads no
English, and does reach the region — in this document `body_text` on the first two pages selects nine elements
including `elem_00017`. What is true is narrower and is what makes this an owner decision: such a predicate
would assert `non_contract_region` — "states no implementable obligation" — from position alone, over eight
other front-matter elements about which it has no evidence, which is exactly the positive structural
demonstration `.9a` ruled prose cannot supply. The choice between admitting the fallback class, accepting a
weaker warrant, and leaving the cell permanently unmet is the owner's.)*

## Decisions (`.9b`)

- `2026-08-30`: split `.9` by region kind rather than shipping one widening. The table leg is buildable now;
  the prose leg is blocked on an `EvidenceIR` provenance gap; the two label-blocked observations are an oracle
  defect. Bundling them would have published one number whose parts have three different causes.
- `2026-08-30`: express table coverage as `supporting_table_ids` membership rather than as "some record id
  embeds the table id". The contract's own rule is that a region is explained only when a canonical record
  *names it in its own provenance list*; an id-embedding test would additionally let a fabricated record
  suppress a residual, which is the opposite of what the carrier is for.
- `2026-08-30`: gather table provenance from every `SemanticIR` surface that carries `supporting_table_ids`,
  not only the ones a table is believed to reach. Over-inclusive coverage can only *remove* residuals, so it
  fails safe against the contract's duplicate-a-promoted-key prohibition; an under-inclusive list fails unsafe.
- `2026-08-30`: carry both the visual-evidence id and the table id in a table region's
  `supporting_evidence_ids`. A table region has two real `EvidenceIR` identities — `visual_NNNN`, under which
  it was captured, and `table_NNNN`, under which every table-derived record cites it — and a residual that
  names only one cannot be joined to the other half of the artifact.
- `2026-08-30`: leave every figure-kind residual `.8c` already publishes byte-unchanged. The generalisation
  adds a population; it is not licence to restate the shipped one.
- `2026-08-30`: record the unsatisfiable labels and the unreachable cell as findings on this leaf rather than
  repairing them here, so the measured movement `.9b` publishes is attributable to the production change alone.

## Shipped table-region carrier (`.9b`)

The producer is one function with two coverage vocabularies, not two mechanisms.
`unexplained_captured_regions` (renamed from `unexplained_captured_visual_regions`) now admits
`VisualAssetKind::TableRegion`, and `residual_accountable_region_kind` excludes only `Unknown` — the one bucket
where capture never established the region as intent-bearing. Everything a table region needs beyond that is
answered by asking the right question:

- `captured_region_is_explained` is **kind-scoped, not a union**. A visual region is explained by its
  `evidence_id` or its `figure:<asset_id>` form; a table region by its table id in the new
  `SemanticIr::cited_table_ids`. Because neither vocabulary can ever match the other's ids, widening the
  accounting leaves every figure-kind residual `.8c` published bit-identical *by construction* rather than by
  luck, and two focused controls assert exactly that in both directions.
- `cited_table_ids` reads every `SemanticIr` surface that declares `supporting_table_ids`: `register_records`,
  `timing_constraints`, `signal_polarities`, the observations of `signal_polarity_conflicts` and
  `signal_semantic_conflicts`, and `interfaces[].signal_records` with their `semantic_observations`.
  `MessageFieldRecord` and `SignalSemanticHintRecord` declare the field too but are `EvidenceIR`-only.
- `captured_region_evidence_ids` gives a table region **both** of its real `EvidenceIR` identities —
  `visual_NNNN`, the record that captured the rendered region, and `table_NNNN`, the id every table-derived
  record cites. A residual naming only one could not be joined to the other half of the artifact.

Nothing about the record's grammar moved: same closed cause, same boundary, same operator replay route, same
two claim families, and no new public field — the registry stays at 41 families and 170 field rules.

The anti-drift control is worth naming because the failure it guards is silent.
`cited_table_ids_gathers_every_declared_table_provenance_surface` gives each of the seven declared surfaces a
distinct table id, asserts all seven are gathered, and then walks the same artifact's serialized JSON for every
`supporting_table_ids` array and requires the two sets to agree. A surface added later that does not join the
typed gatherer therefore fails a test instead of quietly emitting a residual for a region a promoted record
already explains.

## Found defect — the frozen residual contract is red and nothing runs it (`.9b`, routed to `.9d`)

`python3 -B scripts/validate_residual_actionability_contract.py --check` fails at this commit with two
violations: `witness no longer pins the tracked current-result identity`, and
`witness.required_and_absent_cells differs from the current-result derivation`.

**`.9b` did not cause it.** Reverting only `.9b`'s production and fixture changes reproduces both failures
unchanged. The cause is exact: `current_result_snapshot.json` was last written by
[`SOURCE-IR-REPRODUCIBILITY.2`](../SOURCE-IR-REPRODUCIBILITY.md) (`245b3b60`), *after* `.8d` (`893c2fba`) froze
the witness. That leaf repaired the Cortex-A76 anchor regression `.8d` had recorded — resolving the reviewed
region by content rather than by ordinal position — which correctly removed `source_region_missing_or_ambiguous`
from the `software_guidance` cell's hard failures and republished the result. The frozen contract still pins the
pre-repair digest and the pre-repair hard-failure list. Every published count is unaffected: the derivation is
still 4 actionable / 4 not-required / 4 required-and-absent cells and 24 / 8 / 8 / 8 / 16 / 8 observations, and
the only differing field is that one cell's `hard_failures` array.

**The second half is the more important one: nothing executes the checker.** `.8a` made the required-residual
rule executable precisely so it could not drift, and the whole repository references
`scripts/validate_residual_actionability_contract.py` only in prose — it is absent from
`scripts/check_doctrines.sh`, from `scripts/run_ci.sh`, and from every test. So the contract went stale several
commits ago and no gate said so. That is the same defect class `.8a` itself repaired on its own evidence path,
where a published reproduction command exited zero without running a test. `.9d` owns both halves; the re-pin is
not done here so that the measured movement `.9b` publishes stays attributable to the production change alone.

## Chain reconciliation (`.9b`)

`scripts/rebuild_stage_cascade.sh --write` is a remedy for a stale *seal*, and it refuses to absorb a content
delta it cannot attribute (ADR 0025 decision 1). It therefore stopped at the SemanticIR stage and named the
differing top-level section for all 24 proof-carrying documents — which is exactly the attribution evidence this
leaf needs, not an obstacle:

- **The only differing public field is `captured_region_residuals`**, in every one of the 24 chains. No other
  section moved at any document.
- **Every added record is table-kind.** Comparing the retained pre-write snapshot against the rebuilt artifact,
  each chain's growth equals its table-region residual count exactly — Arm Debug `176 → 370` is exactly its 194
  uncited table regions, CoreSight `7 → 19` its 12, Cortex-A76 `3 → 71` its 68, the GIC guide `31 → 44` its 13,
  I2S `20 → 25` its 5, OpenCAPI 25G `7 → 17` its 10 and 32G `12 → 29` its 17. Those figures were measured from
  the persisted corpus *before* the producer was written, and the rebuild re-derives every one of them.
- **Every pre-change residual survives byte-identical.** Across all 24 chains the pre-change collection equals
  the rebuilt collection with the table records removed, element for element, and no pre-change artifact
  contained a table record. 854 table residuals were added in total, each carrying exactly two provenance ids
  with the region's table id second.

With that delta attributed, the cascade completes. IntentIR reproduces it exactly — every chain's IntentIR
growth equals its SemanticIR growth, which is what "carried unchanged" has to mean — and the ISF adapter needed
no content change at all, because the residual collection is an IR-level disposition that reaches no adapter
row. The closing run reports all four stages at **24 rebuilt / 24 content-identical / 0 content-changed / 24
validated / 0 failed**, with one distinct seal per stage and no seal transition, and 54 legacy proofless
documents explicitly out of scope.

The cascade is a remedy, not the oracle, so `scripts/check_chain_currency.sh` certifies the result independently
by replaying every stage from its persisted input with the current binary: **24 replayed / 24 current / 0 stale**
at EvidenceIR, SemanticIR, IntentIR, and the ISF adapter, 54 documents explicitly unmeasurable, 24 adapter output
states checked, and 24 normalized bundles on disk matching the declared retained set exactly.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.9b`

- [x] **REPRODUCE / MEASURE** — the four required-and-absent cells re-derive from
  `current_result_snapshot.json` exactly as `.9a` recorded, and the widening was measured before it was written:
  under `.8c`'s coverage test every captured table region reads as unexplained — 354 of 354 in the AMD IOMMU
  chain and 210 of 210 in the Arm Debug chain — while the reviewed `table_0067` scores 0/0/0 canonical at
  `evidence_ir`, `semantic_ir`, and `intent_ir`, so production genuinely produces nothing from it.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs#residual_accountable_region_kind`
  excluded `VisualAssetKind::TableRegion` on the premise that a table region always reaches a register, signal,
  or timing carrier. It does not: the coverage test at `semantic.rs#unexplained_captured_visual_regions` asked
  only for the region's `evidence_id` / `figure:<asset_id>`, and no table-derived record cites either — table
  provenance in this pipeline is `supporting_table_ids`. Both halves were wrong together, which is why lifting
  the kind exclusion alone would have residualised 354 → 354 and 210 → 210 regions including promoted ones.
- [x] **ADDRESSED (verified)** — the kind gate now excludes only `Unknown`, and coverage is asked per kind:
  `captured_region_is_explained` routes a table region to the new `SemanticIr::cited_table_ids`, which reads all
  seven `SemanticIr` surfaces declaring `supporting_table_ids`. Measured on the current Arm Debug chain the
  answer moves from 210 of 210 unexplained to 16 cited / 194 unexplained. A table residual carries both
  `visual_NNNN` and `table_NNNN`, which is what the reviewed cell's `required_provenance_ids: ["table_0067"]`
  needs. Seven focused controls pass, including both refusal directions and the seven-surface
  gatherer-completeness cross-check against the artifact's own serialized provenance.
- [x] **NO REGRESSION** — `cargo test --workspace --lib` is 470 / 168 / 1,372 / 4 with zero failures and
  `cargo fmt --all --check` is clean; all five production-genericity components pass with the registry unchanged
  at 41 families / 170 field rules / 118 producer entrypoints / 53 seams; the compiled information-flow census
  moves only its size dimensions (2,370 → 2,373 functions, 14,681 → 14,684 helper edges, 12,656 → 12,669
  decision sites, 1,464 → 1,466 semantic macros) while all 141 boundary rows, 120 rule roots, 64 declassifiers,
  12 seams, 25 proof gates, 11 trusted regions, 15 protected types, 19 protected constructions and 28 protected
  calls are unchanged; every figure-kind residual `.8c` published is unchanged by construction, asserted in both
  refusal directions in the unit tests *and* measured byte-for-byte against the rebuild's retained pre-write
  snapshot across all 24 proof-carrying chains, where the sole public-field delta is 854 added table records;
  and the CHAIN-CURRENCY oracle independently certifies 24 replayed / 24 current / 0 stale at all four stages
  with retention exactly the declared set. **One leg
  is explicitly not green and is not claimed:** the frozen residual-actionability contract self-test is red at
  this commit for a cause `.9b` did not create — reverting only this leaf's production and fixture changes
  reproduces the same two failures — and `.9d` owns it.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — the widened gate reads only `VisualAssetKind`, `asset_id`,
  `evidence_id`, and record-level provenance lists. No document, vendor, protocol, family, or review label
  reaches production: the reviewed family names `packed_page_table_entry` and `table_of_contents` appear only in
  the conformance fixture that projects the record, never in the producer, and the table-coverage test asks a
  membership question over concrete collections rather than parsing an id or scanning text.
- [x] **LOCKSTEP** — the producer, the rule-family inventory entrypoint, the flow census pins, the mdBook
  SemanticIR chapter, the Knowledge Map fact card and its regenerated projection, the fact-card catalog, this
  part, the bounded root, the resume pointer, and the change ledger agree that the carrier now covers captured
  table regions and that the prose leg is blocked on `EvidenceIR` provenance.

## Repaired contract and its gate (`.9d`)

The `.9b` finding had two halves and they are not the same defect.

**The witness was stale, and the attribution is mechanical.** Over the only two commits that touched either
authority since the freeze, the pinned digest MATCHES at `893c2fba` and MISMATCHES at `245b3b60`, and running
each revision's *own* `decompose_current_result` against that revision's *own* snapshot is identical at the
freeze and differs at the repair in exactly one field: `software_guidance.hard_failures` loses
`source_region_missing_or_ambiguous`. That is `SOURCE-IR-REPRODUCIBILITY.2` correctly repairing the Cortex-A76
anchor regression `.8d` had recorded. The contract was red for **43 commits**. The re-pin re-derives all three
cell collections from the contract's own decomposition rather than hand-editing a field, so the change cannot
quietly carry anything else, and no published scalar moves — 12 declared cells, 24 declared / 8 actionable /
8 not-required / 8 required-and-absent observations, corrected 16/8 — leaving a diff of exactly the identity
plus that one hard-failure entry.

**The gate is the half that matters.** `.8a` made the rule executable so it could not drift; nothing ran it, so
it drifted anyway. `RESIDUAL-ACTIONABILITY` is now a gate-tier doctrine in `scripts/check_doctrines.sh` at
0.04s, and it is proven fail-closed in both shapes rather than assumed: a drifted witness identity makes the
driver print `FAIL RESIDUAL-ACTIONABILITY` and `commit/merge blocked`, and a silently edited published count is
rejected naming the value the result actually derives. Both restore the contract byte-exact. The driver registry
and the `DOCTRINE_ENFORCEMENT.md` §10 mirror are verified equal and in the same order at 13 entries — the
lockstep §10 claims and nothing had been checking.

Had this gate existed, `SOURCE-IR-REPRODUCIBILITY.2` would have been blocked and forced to re-examine the
frozen contract in the same commit. That is the whole point, and it is why the repair alone would not have been
a fix.

## Acceptance Checklist (enforced) — `SPEC-TO-INTENT-ALIGNMENT.9d`

- [x] **REPRODUCE / MEASURE** — `./scripts/validate_residual_actionability_contract.py` exits `1` with
  `witness no longer pins the tracked current-result identity` and
  `witness.required_and_absent_cells differs from the current-result derivation`; `git rev-list --count
  245b3b60..HEAD` is 43.
- [x] **ROOT CAUSE (WHY + WHERE)** — attributed by re-deriving at each revision from its own producer input,
  never by reading a diff: digest MATCH at `893c2fba` → MISMATCH at `245b3b60`, and the per-revision
  decomposition differs in exactly `software_guidance.hard_failures`. The second, deeper cause is that the
  checker had **zero** executable references — absent from `scripts/check_doctrines.sh`, `scripts/run_ci.sh`,
  and every test — so no gate required the contract to be re-examined when the result was republished.
- [x] **ADDRESSED (verified)** — the witness re-pins to the tracked snapshot and its cell collections are
  re-derived from the contract's own decomposition; `--check` passes and re-derives the published `8/16`, and
  the self-test is 28/28. `RESIDUAL-ACTIONABILITY` is registered gate-tier and the driver executes it.
- [x] **NO REGRESSION** — no published scalar moves (12 / 24 / 8 / 8 / 8 / 16 / 8 all unchanged), so the diff is
  the identity plus one hard-failure entry; `scripts/check_doctrines.sh` passes all 12 executed gate-tier
  doctrines; the two RED controls prove the new gate blocks a drifted identity and a forged count and restore
  the contract byte-exact; `cargo` is untouched by this leaf, which changes no Rust.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — no production source changes; the contract is conformance/doctrine
  authority and carries no document, vendor, or protocol identity.
- [x] **LOCKSTEP** — the driver registry, the `DOCTRINE_ENFORCEMENT.md` §10 mirror (verified equal, 13 entries,
  same order), the mdBook doctrine-enforcement chapter, this part, the bounded root, the resume pointer, and the
  change ledger agree that the contract is repaired and gated.

## Update protocol

`.9b` is complete; `.9c` and `.9d` are declared and own the remaining legs. A correction to a published
metric or route here updates the containment contract, index, and manifest in the same commit. Decisions
and verification for the `.9` program stay in this part so it keeps one chronology, and the bounded root
carries only the current summary.

**Containment, re-derived by `.9e`.** Splitting this part out of
[residual carrier](residual-carrier.md) returned the semantic-part collection to 83.8% of its `lines_each`
target at the cost of one row on the active task index. `.9b` then recorded that "a single further leaf route
crosses its mandatory rollover" and that `.9c`/`.9d` must roll the index before declaring a leaf. **Both halves
of that are wrong and `.9e` withdraws them.** The arithmetic: the index is 114 lines against a 128-line health
target, so rollover begins at 115.2 lines — one added row is 115 (89.84%, legal) and *two* are 116 (90.62%,
which trips). And `.9c` was already declared with its route and index row by the `.9b` declaration commit, so it
was never blocked at all; the pressure only binds when a leaf beyond the declared set is added. Declaring `.9e`
consumes the one legal row, which leaves the index at 115 lines: **the next new leaf after `.9e` is the one that
must roll the index first.** Re-derive rather than trusting this sentence:
`perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report`.
