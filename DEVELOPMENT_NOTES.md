# DEVELOPMENT_NOTES
## Current project direction
- project name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- product shape: staged IR toolchain, not one-shot backend generation
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## 2026-04-17 KG EvidenceIR table-provenance count expectation
- Added `table_signal_declaration_provenance_count` to `EvidenceStageExpectations`.
- This count is evaluated directly against `EvidenceIr.table_signal_declaration_provenance.len()`.
- The point is negative leakage coverage: `table_signal_declaration_provenance_include` proves specific records exist, while the count can prove no unexpected records exist.
- `table_misclassification_field_table_negative` now uses the count to require zero EvidenceIR table-signal provenance from a deliberately misclassified `Bits | Name | Description` field table.
- The fixture also asserts the user-visible EvidenceIR validation metric `table_signal_declaration_provenance: 0`.
- This locks the intended boundary at the earliest provenance layer: field tables may remain present as structured tables, but they must not synthesize top-level signal declarations or table-backed signal provenance.
- Focused fixture validation, full tracked `kg-bench`, `corpus-kb`, docs CI, full local CI, and `git diff --check` passed; full local CI reports `323` Rust tests plus the mdBook build.

## 2026-04-17 KG EvidenceIR missing-provenance diagnostic self-test
- Added `kg_bench_reports_missing_evidence_table_provenance_failure`.
- The test creates a temporary KG fixture with a real structured signal table, then expects `XREQ` provenance from a deliberately wrong table id.
- This proves the missing-record branch of `table_signal_declaration_provenance_include` fails with enough context to debug the fixture:
  - fixture name
  - failed expectation field
  - missing expected table id
  - actual table id carried by EvidenceIR
- Refactored the two EvidenceIR provenance diagnostic tests through a shared one-row signal-table fixture writer so future diagnostic cases do not have to duplicate a full structured-table JSON patch.
- The previous self-test covered the subtle "matching signal/table but wrong synthesized statement text" branch; this one covers the "no matching signal/table provenance record" branch.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed; full local CI now reports `323` Rust tests plus the mdBook build.

## 2026-04-17 KG EvidenceIR table-provenance diagnostic self-test
- Added `kg_bench_reports_evidence_table_provenance_statement_failure`.
- The test creates a temporary KG fixture with a real structured signal table and an intentionally wrong expected synthesized statement text.
- This proves the new `table_signal_declaration_provenance_include` path fails for the important subtle case: the signal/table pair exists, but the provenance record does not point at the expected generated declaration text.
- The first run exposed a diagnostic asymmetry: missing provenance records named `table_signal_declaration_provenance_include`, but statement-text mismatches did not.
- The diagnostic now names the same expectation field in both failure modes, so users debugging tracked fixtures see the exact schema field that failed.
- This is harness quality work, not extractor behavior change: it strengthens trust in the executable benchmark surface that guards EvidenceIR table provenance.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed; full local CI now reports `322` Rust tests plus the mdBook build.

## 2026-04-17 EvidenceIR table provenance KG expectations
- Added a direct EvidenceIR expectation surface to `specforge kg-bench`.
- The first field is deliberately narrow: `table_signal_declaration_provenance_include`.
- Each expectation requires:
  - `signal_name`
  - `table_id`
  - optional `statement_text`
- This lets fixtures prove that the EvidenceIR table bridge contains exact records, not merely that validation counted some number of provenance links.
- `signal_table_inventory_authority_negative` now uses the field to assert that:
  - `XREQ` links to `table_protocol_signal_description` and references `Signal XREQ is output width 1.`
  - `XACK` links to `table_protocol_signal_description` and references `Signal XACK is input width 1.`
  - `PAYLOAD` links to `table_protocol_signal_description` and references `Signal PAYLOAD is output width DATA_WIDTH.`
- The focused fixture initially caught an incorrect expectation of `PAYLOAD` width `32`; the real table-derived statement is `DATA_WIDTH`, which proves the new assertion is checking useful truth rather than only a coarse count.
- This keeps the table-provenance ladder honest across stages:
  - EvidenceIR exact provenance record
  - EvidenceIR aggregate validation metric
  - SemanticIR / IntentIR exact canonical table support
  - SemanticIR / IntentIR aggregate table-support validation metric
- Focused and full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## 2026-04-17 README bootstrap continuity sync after EvidenceIR metric
- Executed the README handoff path after `f58ce2e feat(validation): report evidence table provenance`.
- `SESSION_BOOTSTRAP.md` still instructs future agents to read the referenced docs, analyze the Rust codebase, update `RUST_CODEBASE_ANALYSIS.md` if needed, and continue from the roadmap.
- The Rust codebase sanity pass observed the expected active crate shape:
  - `30` Rust source files under `crates/specforge/src`
  - `59,520` total Rust source lines in that tree
  - one workspace package named `specforge`, with library and binary targets from `cargo metadata --no-deps`
  - CLI dispatch still exposes the staged pipeline plus quality/learning/rescan commands documented in README and the mdBook
- No `RUST_CODEBASE_ANALYSIS.md` update was needed because it already records the committed EvidenceIR provenance validation metric, the current command surface, the staged IR architecture, and the present semantic-truthfulness risk picture.
- `MEMORY.md` did need the standard post-commit continuity correction: its latest committed baseline still pointed at `7d8e6e2`, while the current committed baseline is now `f58ce2e`.

## 2026-04-17 EvidenceIR table-signal provenance validation metric
- Added an EvidenceIR-side validation metric named `table_signal_declaration_provenance`.
- The metric reports `EvidenceIr.table_signal_declaration_provenance.len()`, making the source-table bridge visible before canonical signal carry-through happens in `SemanticIR` / `IntentIR`.
- This closes a small observability gap in the table-provenance story:
  - EvidenceIR now exposes whether table-synthesized declarations are carrying table ids at all
  - SemanticIR / IntentIR still expose whether canonical signal records retain table support through `with_table_support`
  - `kg-bench` canonical expectations still lock exact per-signal table provenance with `signal_supporting_table_ids_include`
- The metric remains diagnostic only. It does not author truth, mutate IR, fix missing provenance, or promote facts; it simply makes a typed evidence bridge countable and benchmarkable.
- `signal_table_inventory_authority_negative` now asserts the EvidenceIR metric value `3`, matching table-authored `XREQ`, `XACK`, and `PAYLOAD`.
- Focused validation passed for the new unit test and focused KG fixture; full tracked `kg-bench`, `corpus-kb`, docs CI, full local CI, and `git diff --check` also passed.
- Full local CI now reports `321` Rust tests plus the mdBook build.

## 2026-04-17 Direct unit coverage for table-support validation metrics
- Added `validate_semantic_and_intent_ir_count_signal_table_support` to the validator test suite.
- The test builds a real staged pipeline from markdown plus a structured signal-description table rather than constructing a report by hand.
- It checks that table-authored `XREQ`, `XACK`, and `PAYLOAD` produce `with_table_support: 3` in both canonical validation reports.
- This complements `signal_table_inventory_authority_negative`: KG-bench still proves exact per-signal table provenance and false-positive exclusion, while the unit test locks the validator metric itself.
- Focused validation passed for the new test, and full local CI passed with `320` Rust tests plus the mdBook build.

## 2026-04-17 README bootstrap continuity sync after validation metric
- Executed the README handoff path after `4f61ec6 feat(validation): report signal table support`.
- `SESSION_BOOTSTRAP.md` still instructs a future agent to read the referenced docs, analyze the Rust codebase, update `RUST_CODEBASE_ANALYSIS.md` if needed, and continue from the roadmap.
- The Rust codebase sanity pass observed the expected active crate shape:
  - `30` Rust source files under `crates/specforge/src`
  - `59,341` total Rust source lines in that tree
  - one workspace package named `specforge`, with library and binary targets from `cargo metadata --no-deps`
  - CLI dispatch still exposes the staged pipeline plus quality/learning/rescan commands documented in README and the mdBook
- No `RUST_CODEBASE_ANALYSIS.md` update was needed because it already records the committed table-support validation metric, the current command surface, the staged IR architecture, and the present semantic-truthfulness risk picture.
- `MEMORY.md` did need the standard post-commit continuity correction: its latest committed baseline still pointed at `2014352`, while the current committed baseline is now `4f61ec6`.

## 2026-04-17 Validation reports table-backed signal coverage
- Follow-on to the canonical `supporting_table_ids` carry-through: `specforge validate` now reports `with_table_support` for `SemanticIR` and `IntentIR`.
- This metric is intentionally diagnostic, not authoring:
  - it counts canonical signals that still have non-empty `supporting_table_ids`
  - it does not infer new facts, fix missing provenance, or promote table evidence by itself
  - exact per-signal provenance remains locked through `kg-bench` expectations such as `signal_supporting_table_ids_include`
- Semantic-stage counting walks all interface signal records.
- Intent-stage counting walks declared, non-`Low` signal records, matching the other IntentIR declared-signal coverage metrics.
- `signal_table_inventory_authority_negative` now asserts `with_table_support: 3` at both canonical stages, so the validation surface proves the three table-authored signals are visible as a table-backed inventory family, while the canonical expectations still prove `XREQ`, `XACK`, and `PAYLOAD` each carry `table_protocol_signal_description`.
- Focused and full `kg-bench` validation passed with `89/89` fixtures, `corpus-kb` refresh passed with `89` fixtures / `0` failures and no tracked content drift, and full local CI passed with `319` Rust tests plus the mdBook build.

## 2026-04-17 README bootstrap continuity sync
- Executed the README handoff path after `bb1def0`.
- `SESSION_BOOTSTRAP.md` still instructs a future agent to read the referenced docs, analyze the Rust codebase, update `RUST_CODEBASE_ANALYSIS.md` if needed, and continue from the roadmap.
- The Rust codebase sanity pass observed the expected active crate shape:
  - `30` Rust source files under `crates/specforge/src`
  - `58,038` total Rust source lines in that tree
  - CLI dispatch still exposes the staged pipeline plus quality/learning commands documented in README
- No `RUST_CODEBASE_ANALYSIS.md` update was needed because it already records the committed signal-table provenance bridge and current command surface.
- `MEMORY.md` did need a small correction: its latest committed baseline still pointed at `e96ebe1` and described the provenance work as in-flight, even though `bb1def0 feat(ir): carry signal table provenance` is now committed.

## 2026-04-17 Signal table provenance is first-class on canonical signals
- Follow-on to `signal_table_inventory_authority_negative`: table-derived signal inventory is now not only preserved, it is traceable at canonical signal-record granularity.
- `EvidenceIR` now records `table_signal_declaration_provenance` when `SourceIR.structured_tables` synthesize formal `Signal X is ...` statements from signal-description tables.
- `SemanticIR` builds a statement-id to table-id bridge from that provenance and copies those table ids into each `InterfaceSignalRecord.supporting_table_ids` when the declaration is lifted into canonical interface inventory.
- `IntentIR` inherits the same table support because it carries canonical semantic interface records forward rather than re-extracting them.
- `specforge kg-bench` now accepts `signal_supporting_table_ids_include` in semantic and intent expectations, so fixtures can prove that a signal is backed by a specific structured table.
- The hardened `signal_table_inventory_authority_negative` now checks both identity and provenance:
  - `XREQ`, `XACK`, and `PAYLOAD` must survive through `SemanticIR` and `IntentIR`
  - all three must carry `table_protocol_signal_description`
  - uppercase prose vocabulary such as `PDF`, `RTL`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, and `SoC` must stay excluded
- This keeps the model honest in the exact direction the user highlighted: chip-spec PDFs usually contain explicit signal tables, and those tables should be authoritative evidence with recoverable provenance, not merely a convenience that vanishes after statement synthesis.
- Focused and full KG-bench validation passed with `89/89` fixtures; corpus-KB refresh passed with `89` fixtures / `0` failures; full local CI passed with `319` Rust tests plus the mdBook build.

## 2026-04-17 Signal tables are executable inventory authority
- Added `signal_table_inventory_authority_negative` to encode the user's point that real chip/protocol PDFs usually carry actual protocol signals in dedicated signal/interface tables.
- The fixture uses no explicit markdown signal declarations. The real interface inventory comes from a structured `Signal | Direction | Width | Description` table injected through `SourceIR`.
- Surrounding prose deliberately mentions uppercase non-signal engineering terms: `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, and `SoC`.
- Expected behavior:
  - `XREQ`, `XACK`, and `PAYLOAD` enter `SemanticIR` / `IntentIR`
  - table-derived flat directions survive for all three signals
  - all non-signal engineering/context terms are excluded from canonical signal inventory
- Focused and full `kg-bench` validation passed with `89/89` fixtures, `corpus-kb` refreshed table coverage to `9/9` and truthfulness-negative/caution coverage to `36/36`, and full local CI passed with `319` Rust tests plus the mdBook build.

## 2026-04-17 KG signal-inventory exclusions
- Added `signal_names_exclude` to `CanonicalStageExpectations` in `specforge kg-bench`.
- The field checks canonical interface signal names directly, complementing `signal_names_include`.
- This fills a small but important truthfulness-testing gap: many fixtures could prove that required signals survived, but could not directly prove that tempting document or integration vocabulary stayed out of the signal inventory.
- The first consumer is `clock_reset_contract_scope_negative`, which now excludes `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `PLLs`, `DFT`, `SoC`, and `SOC` at both `SemanticIR` and `IntentIR`.
- The source now explicitly says `verification IP (VIP)` so the fixture protects the acronym form the user actually uses.
- Focused and full `kg-bench` validation passed after adding the exclusion expectations, `corpus-kb` refreshed from the `88/88` fixture suite with `0` failures, and full local CI passed with `319` Rust tests plus the mdBook build.

## 2026-04-17 Clock/reset protocol scope is now executable
- Added `clock_reset_contract_scope_negative` to move the protocol-PDF scope boundary from documentation-only steering into tracked KG benchmark coverage.
- The fixture models the intended document shape directly:
  - the protocol PDF defines `ACLK` and active-low asynchronous `ARESETN`
  - it states that the clock/reset contract is for RTL and verification IP
  - it states that the final physical clock tree and reset tree are built by the integrating SoC team
  - it names project-local dependencies such as PLLs, clock generators, reset controllers, power domains, DFT/scan constraints, floorplan, and methodology
- Expected behavior is intentionally narrow: preserve two infrastructure signals, preserve zero ordinary actor ports, and preserve zero concrete topology records.
- This protects against a plausible overreach bug where integration-scope language such as "physical clock tree" or "reset-tree construction" might look hardware-specific enough to become false topology.
- Focused and full `kg-bench` validation passed with `88/88` fixtures, `corpus-kb` refreshed the benchmark, infrastructure, and semantic/truthfulness pattern projections from that run, and full local CI passed with `319` Rust tests plus the mdBook build.

## 2026-04-17 Protocol PDFs expose clock/reset contracts, not physical trees
- User clarification accepted and recorded: AMBA, Intel, and similar chip-design/protocol PDFs are usually contract documents for RTL implementation and verification IP, not complete implementation plans for the final chip's physical clock/reset distribution.
- SPECFORGE should therefore treat these documents as sources of interface-visible clock/reset contract semantics:
  - clock and reset signal identity
  - polarity and active level
  - synchronous/asynchronous reset kind
  - reset assertion/release timing discipline
  - boundary-visible timing and protocol obligations that RTL and VIP can rely on
- Physical clock and reset trees are normally custom SoC integration artifacts owned by the team building the whole chip. They depend on project-local details such as clock generators, PLLs, reset controllers, power domains, CDC/RDC policy, DFT/scan constraints, CTS strategy, floorplan, and methodology.
- Implementation steering:
  - do not infer physical tree implementation from protocol PDFs
  - keep generic clock/reset advice as doctrine/caution, not topology evidence
  - allow only explicit current-document topology hints to enter bounded `infrastructure_topology`
  - even explicit topology hints remain intent-level evidence, not physical signoff or a complete tree recipe
  - if a downstream RTL/VIP flow needs real clock/reset tree construction, represent that as an integration residual or external project input rather than canonicalizing it from a protocol PDF

## 2026-04-17 Clock/reset generic advice stays non-authoring
- Added `clock_reset_generic_advice_negative` as a benchmark-hardening slice, not a production extractor rewrite.
- The fixture complements `clock_reset_topology_gold`:
  - the gold fixture proves explicit current-document topology phrases can author bounded `infrastructure_topology`
  - the new negative fixture proves generic doctrine/advice does not author topology on its own
- The intended semantic boundary is precise:
  - clock and reset signals remain first-class infrastructure semantics
  - explicit reset polarity and asynchronous assertion / synchronous release discipline can be represented
  - generic advice such as "avoid glitches", "avoid glue logic", or "may use a synchronizer" remains doctrine/caution unless the current document names a concrete gated branch, synchronizer stage count, or reset-tree target
- The fixture therefore expects `ACLK` and `ARESETN` to survive as `system_clock` / `system_reset` infrastructure signals while all topology counters remain zero through both `SemanticIR` and `IntentIR`.
- This protects the project from a subtle false-positive class: knowledgeable-sounding engineering prose should make the model more cautious, not more willing to hallucinate concrete implementation topology.
- Focused and full `kg-bench` validation passed with `87/87` fixtures after the addition, `corpus-kb` refreshed the benchmark, infrastructure, and semantic/truthfulness pattern projections from that run, and full local CI passed with `319` Rust tests plus the mdBook build.

## 2026-04-17 FSMGEN response accepted cross-project sync contract
- FSMGEN responded to `docs/FSMGEN_FEEDBACK.md` in its own tracked file at `/Users/richarddje/Documents/github/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, observed at FSMGEN commit `7475f07`.
- The response accepts the shared framing:
  - `.fsm` should remain precise rather than permissive
  - strict mode is the canonical future-facing authoring surface
  - compatibility syntax remains useful only when labeled as residue
  - mdBook remains the human language contract
  - tool consumers need machine-readable contracts
  - FSMGEN should not become SPECFORGE's PDF extraction engine or canonical `IntentIR`
- FSMGEN's accepted near-term priority order is now a planning input for SPECFORGE:
  - build capability metadata from R12 support accounting
  - start R13 with a bounded capability manifest
  - introduce stable diagnostic codes
  - add check-only JSON output
  - add normalized semantic JSON export
  - use those surfaces before deeper actor/channel/semantic-role/temporal/provenance language additions are treated as stable
- SPECFORGE should therefore keep the `.fsm` adapter strict-mode-first, keep compatibility syntax blocked by default, and use FSMGEN's mdBook plus regression corpus/support-accounting files as the current contract until machine-readable surfaces exist.
- FSMGEN explicitly agreed that longer-term language additions should not be unchecked annotations; they must be parsed, validated, normalized, documented, support-accounted, and either lowered honestly or preserved as checked metadata.

## 2026-04-17 FSMGEN feedback now introduces SPECFORGE
- `docs/FSMGEN_FEEDBACK.md` now has a self-contained `What SPECFORGE Is` section so FSMGEN can read the handoff without needing prior project context.
- The section presents SPECFORGE as a Rust toolchain for recovering typed implementation intent from chip-design specs, especially PDFs.
- It spells out the staged pipeline as `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`, names `IntentIR` as the backend-independent product, and explains that `.fsm` is one downstream adapter target rather than the sole product boundary.
- The important framing for FSMGEN is now explicit: SPECFORGE cares about FSMGEN because `.fsm` can become the most natural high-level lowering format for recovered control intent if the language evolves to preserve common `IntentIR` facts directly.

## 2026-04-17 FSMGEN feedback reframed around IntentIR-aligned `.fsm`
- The FSMGEN feedback needed a course correction: validation/check tooling is useful, but the deeper ask is for `.fsm` to become a natural lowering surface for SPECFORGE's captured `IntentIR`.
- `docs/FSMGEN_FEEDBACK.md` now separates two layers:
  - language-level suggestions that would let `.fsm` carry richer intent directly
  - support/tooling suggestions that would let SPECFORGE check the target boundary automatically
- The language-level suggestions are intentionally inline with FSMGEN's active direction rather than adversarial to it: strict-mode canonical syntax, live docs, typed diagnostics, aggregate/type/package support, and composition/toplink semantics are already moving toward a richer language contract.
- New IntentIR-aligned feature suggestions captured:
  - first-class clock/reset/system contracts, including polarity and reset timing semantics
  - actor-relative port semantics and target-actor declarations
  - protocol-neutral interface/channel grouping with semantic roles
  - optional temporal/stability contracts such as stable-while, handshake-complete, and bounded-cycle obligations
  - semantic signal roles for clock/reset/valid/ready/payload/select/enable/response/control meanings
  - structured assumptions, residuals, caveats, and provenance metadata for generated `.fsm`
  - a strict-mode direct-module root shape if FSMGEN wants that root to become canonical
  - contract-aware composition that preserves actor roles, clock/reset distribution, channel grouping, and link provenance across child boundaries
- The priority order from SPECFORGE's side is now explicit: reset/clock contract metadata, capability manifest, JSON diagnostics, normalized AST/IR export, actor/role annotations, temporal contracts, then adapter-facing examples.

## 2026-04-17 FSMGEN sync and `.fsm` adapter steering
- `subs/fsmgen` was fast-forwarded from `57f00e5` to `955f2bb` for reconnaissance, not because SPECFORGE wants to turn the submodule into an implementation surface.
- The refreshed FSMGEN baseline now includes a live mdBook at `subs/fsmgen/docs/book/`, plus substantial R11/R12 work around strict-mode support accounting, typed failure diagnostics, aggregate/package/type semantics, composition/toplink typing, and structural forward-IR layers.
- The useful SPECFORGE takeaway is that FSMGEN is becoming a better reference for `.fsm` syntax, support boundaries, and validation expectations. It should inform the adapter, but it should not replace the `IntentIR` boundary.
- Adapter rule reinforced by the sync:
  - emit `.fsm` only when canonical facts are explicit enough to map into a real FSMGEN-supported construct
  - prefer a blocked adapter artifact plus residual decision over compatibility-shaped target text when the canonical source lacks a safe root kind, direction, reset, topology, or control-shape fact
  - keep target-specific perspective recovery inside the adapter, without mutating canonical `IntentIR`
  - validate generated `.fsm` against FSMGEN when a stable local check surface exists, but do not treat a successful target parse as proof that the PDF intent was correct
- Current FSMGEN signals that are especially useful for SPECFORGE:
  - strict-supported corpus gates separate compatibility residue from canonical syntax
  - typed diagnostic work can become a future adapter validation oracle
  - aggregate/type/package expansion suggests richer canonical symbol lowering targets once SPECFORGE's own semantic facts are strong enough
  - composition/toplink expression support gives useful reference shapes for explicit top-root lowering
  - live mdBook chapters provide a public syntax/semantics map that should be checked before widening `.fsm` emission
- Recommended FSMGEN feature requests / orientation from the SPECFORGE side:
  - keep `docs/FSMGEN_FEEDBACK.md` as the focused tracked handoff document that FSMGEN can read directly
  - publish a machine-readable capability manifest covering root kinds, strict/canonical syntax, supported expression families, reset/system forms, composition forms, and known compatibility-only residue
  - provide a stable `--check --json` or equivalent mode that validates `.fsm` without requiring HDL generation and returns typed diagnostic codes, source spans, severity, and migration hints
  - provide a parse/normalize/export mode for `.fsm -> typed AST/IR JSON`, so SPECFORGE can compare emitted text against normalized target semantics instead of string shape alone
  - expose selected forward IR layers, such as intent/lowered/structural summaries, in a stable machine-readable form when practical
  - make reset/clock semantics more explicit in diagnostics or metadata, especially polarity, asynchronous assertion, synchronous release, and system-signal role constraints
  - maintain a small adapter-facing example corpus with input `.fsm`, normalized AST/IR expectations, and HDL-shape expectations for every canonical feature family
  - keep the strict-mode-first posture: new canonical behavior should be support-accounted with positive and negative fixtures rather than silently accepted through compatibility parsing
- Validation note: a local FSMGEN `./bin/ci-regression` run was started during the initial submodule-refresh path and intentionally stopped after the scope was clarified. It had reached `t/274-package-aggregate-values.t` with all reported tests green. This was not treated as a full FSMGEN certification run.

## 2026-04-17 Direct `.fsm` target inputs from control reads
- This slice follows the output-target actor selection work by recovering the other side of the target-actor perspective.
- Once a standalone direct `.fsm` root has selected one target actor from graph-owned outputs, explicit canonical control structure provides additional bounded evidence:
  - assignment targets are target outputs
  - guard signals and expression source signals are target inputs, unless they are also direct output targets
- The implementation collects input references from canonical DT guards/actions, rich control selectors/predicates/actions, and state-transition guards.
- It then overlays `direct_control_input` evidence only for signals already present in the direct local inventory and only after a target actor context has been selected.
- This avoids the old failure mode where `controller` clearly owned `DATA_OUT` / `ZERO_FLAG`, but `DATA_IN` stayed unresolved merely because the KG only said an external `environment` drives it.
- The recovery remains adapter-time perspective computation, not canonical fact mutation: `IntentIR.actor_ports` is not changed, graph-only signals are not added, and conflicting explicit/graph evidence can still collapse the adapter hint and block lowering.

## 2026-04-17 Direct `.fsm` target-actor selection from graph outputs
- This slice continues `R15` by making standalone direct `.fsm` lowering less dependent on a globally unique actor-port context.
- The previous direct-root graph overlay was safe but overly strict: if a `monitor` read an output signal or an `environment` drove an input signal, the relevant actor-port graph no longer had exactly one actor, so the adapter refused to use graph-backed directions at all.
- The new rule is still bounded by render-critical output obligations:
  - collect direct assignment/init targets from canonical DT/control/init surfaces
  - select one actor only if that actor graph-drives every inventoried output target
  - overlay only that actor's existing local inventory signals
  - keep ambiguous, incomplete, `in_out`, unknown, or internally conflicting graph evidence blocked
- This is a better target-actor-context rule for direct roots: it distinguishes the actor being lowered from external actors that merely share the same signals, without guessing from signal names or flattening producer/consumer roles globally.
- The regression clears all flat direct-interface direction hints, supplies `controller` output actor ports for `DATA_OUT` / `ZERO_FLAG`, plus an external `environment` producer for `DATA_IN` and a `monitor` reader for outputs, and proves `.fsm` lowering remains renderable.
- The adjacent standalone direct ambiguity and conflict tests still pass, so this is a precision-preserving widening rather than a blanket relaxation.

## 2026-04-17 Coordinated active-read KG coverage
- This slice strengthens `relative_clause_actor_noise_negative` so it no longer proves only coordinated producer recovery.
- The fixture source now uses `The Manager samples ARCHUNKEN and RCHUNKV.`, forcing the staged path to recover coordinated active reads for both chunking signals from one sentence.
- Added unit coverage for the read-side object scanner:
  - `coordinated_active_read_extracts_all_sampled_objects`
  - `coordinated_active_read_object_scan_stops_before_guard_clause`
- The guard-clause regression matters because broad object scans can otherwise become too eager and turn condition signals such as `RVALID` into sampled objects.
- The intended extractor contract is now symmetrical: active drive/read clauses can recover multiple object signals from the same clause, but object scanning stops at condition markers and does not cross into guard text.

## 2026-04-17 Relative-clause actor-noise KG fixture
- This slice turns the previous extractor-level AXI chunking fix into an executable staged KG fixture.
- `relative_clause_actor_noise_negative` proves that prose shaped like `An interconnect which connects to components with a mixture of chunking support can drive ARCHUNKEN and RCHUNKV` recovers the real head actor and keeps the descriptive phrase out of the graph.
- The first fixture run exposed a useful second-order gap: the fake actor was gone, but the active-drive extractor only attached the head actor to the first coordinated object.
- Active object parsing now scans the verb's current clause, so `drive ARCHUNKEN and RCHUNKV` can recover producer relations for both signals while still relying on the same sentence/relative-clause subject hygiene.
- The fixture asserts graph direction, actor-signal relations, actor ports, zero connectivity conflicts, and validation metrics at both `SemanticIR` and `IntentIR`, so this behavior is protected beyond unit-test scope.
- The refreshed corpus-KB projection now records the tracked suite as `86/86`, actor-connectivity coverage as `12/12`, and truthfulness-negative/caution coverage as `33/33`.

## 2026-04-17 Relative-clause actor extraction hygiene
- This slice fixes a precision bug exposed by live AXI read-data chunking prose, not a broad score-chasing rewrite.
- Active-drive extraction previously could search backward through a relative clause and choose `mixture of` from prose shaped like `components with a mixture of chunking support can drive ARCHUNKEN`.
- `extract_subject_phrase()` now narrows to the current sentence and strips relative clauses (`which`, `that`, `who`, `whose`) before selecting candidate subject terms.
- `is_meaningful_actor_term()` now rejects `mixture` / `mixture of` as descriptive corpus/prior actor terms, so stale support-phrase vocabulary cannot be harvested or reused as actor-taxonomy knowledge.
- The focused regression proves `interconnect` remains the actor for `ARCHUNKEN` and `mixture of` is not promoted.
- A local AXI rebuild confirms the fake `mixture of` producer is gone while the remaining `ARCHUNKEN` conflict is `Manager` versus `interconnect`, not solved by fiat; the validation score remains `85/100 GOOD`.
- This is the right truthfulness posture: remove demonstrably bogus actors, preserve real ambiguity, and let validation/arbitration keep exposing the unresolved protocol nuance.

## 2026-04-17 AHB exclusive/security wait-state stability KG fixture
- This slice adds focused AHB wait-state stability coverage for `HEXCL`, `HNONSEC`, and `HEXOKAY`.
- `ahb_exclusive_security_stability_gold` proves that exclusive-transfer, non-secure-transfer, and exclusive-response attributes survive as graph-backed actor outputs and actor-grounded stability obligations while an AHB transfer is stalled.
- The fixture intentionally uses `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables, matching specs where optional AHB5-style fields are still carried by the same table idiom as core AHB signals.
- The temporal expectations require `HREADY LOW` and `HSEL HIGH` antecedents with no handshake-completion predicate, preventing exclusive/security wait-state prose from becoming completed-transfer truth.
- The refreshed corpus-KB projection now records the tracked suite as `85/85`, AMBA-family fixture coverage as `36/36`, and timing-family coverage as `42/42`.

## 2026-04-17 AHB transfer/lock wait-state stability KG fixture
- This slice adds focused AHB wait-state stability coverage for `HTRANS` and `HMASTLOCK`.
- `ahb_transfer_lock_stability_gold` proves that transfer type and locked-transfer attributes survive as graph-backed Manager outputs and actor-grounded stability obligations while an AHB transfer is stalled.
- The fixture intentionally uses `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables, matching specs where AHB ownership is conveyed by table placement and destination columns.
- The temporal expectations require `HREADY LOW` and `HSEL HIGH` antecedents with no handshake-completion predicate, preventing stalled transfer/control prose from becoming completed-transfer truth.
- The refreshed corpus-KB projection now records the tracked suite as `84/84`, AMBA-family fixture coverage as `35/35`, and timing-family coverage as `41/41`.

## 2026-04-17 APB address/protection wait-state stability KG fixture
- This slice adds focused APB wait-state stability coverage for `PADDR` and `PPROT`.
- `apb_address_protection_stability_gold` proves that address and protection attributes survive as graph-backed Requester outputs and actor-grounded stability obligations while an APB access is stalled.
- The fixture intentionally uses a `Signal | Source | Width | Description` table so actor ownership is recoverable from table structure, matching many bus-spec signal tables where ownership is columnar rather than repeated in prose.
- The temporal expectations require `PSEL HIGH`, `PENABLE HIGH`, and `PREADY LOW` antecedents with no `HandshakeComplete(PSEL, PREADY)` predicate, preventing stalled wait-state prose from being promoted into completed-transfer truth.
- The refreshed corpus-KB projection now records the tracked suite as `83/83`, AMBA-family fixture coverage as `34/34`, and timing-family coverage as `40/40`.

## 2026-04-17 AXI address/response USER sideband stability KG fixture
- This slice adds focused AXI address-channel and write-response `USER` sideband stability coverage for `AWUSER`, `ARUSER`, and `BUSER`.
- `axi_address_response_user_sideband_stability_gold` proves that optional user-defined address/response sidebands survive as graph-backed actor outputs and actor-grounded stability obligations under their matching channel handshakes.
- The fixture intentionally covers two ownership directions in one small regression: the Manager owns `AWUSER` and `ARUSER` on the address channels, while the Subordinate owns `BUSER` on the write-response channel.
- The temporal expectations require typed `HandshakeComplete(AWVALID, AWREADY)`, `HandshakeComplete(ARVALID, ARREADY)`, and `HandshakeComplete(BVALID, BREADY)` predicates plus producer-grounded `actor_maintains_signal_stable` consequents for the relevant `USER` sideband.
- The refreshed corpus-KB projection now records the tracked suite as `82/82`, AMBA-family fixture coverage as `33/33`, and timing-family coverage as `39/39`.

## 2026-04-17 AXI data USER sideband stability KG fixture
- This slice adds focused AXI data-channel `USER` sideband stability coverage for `WUSER` and `RUSER`.
- `axi_data_user_sideband_stability_gold` proves that optional user-defined data sidebands survive as graph-backed actor outputs and actor-grounded stability obligations under their respective data-channel handshakes.
- The fixture intentionally covers opposite producer directions in one small regression: the Manager owns `WUSER` on the write-data channel, while the Subordinate owns `RUSER` on the read-data channel.
- The temporal expectations require typed `HandshakeComplete(WVALID, WREADY)` / `HandshakeComplete(RVALID, RREADY)` predicates plus producer-grounded `actor_maintains_signal_stable` consequents for the relevant `USER` sideband.
- The refreshed corpus-KB projection now records the tracked suite as `81/81`, AMBA-family fixture coverage as `32/32`, and timing-family coverage as `38/38`.

## 2026-04-16 AXI address QoS/region sideband stability KG fixture
- This slice adds paired AXI address-channel QoS/region stability coverage for `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION`.
- `axi_address_qos_region_sideband_stability_gold` proves those sidebands survive as graph-backed Manager outputs and actor-grounded stability obligations under their respective `AWVALID` / `AWREADY` and `ARVALID` / `ARREADY` handshakes.
- The fixture intentionally uses one width-only table plus prose actor relations for both address channels, matching real AXI tables where the channel context carries ownership and the table mainly carries name/width/description evidence.
- The temporal expectations require the matching typed handshake-completion predicate for each channel and Manager-grounded `actor_maintains_signal_stable` consequents for all four QoS/region sidebands.
- The refreshed corpus-KB projection now records the tracked suite as `80/80`, AMBA-family fixture coverage as `31/31`, and timing-family coverage as `37/37`.

## 2026-04-16 AXI read-address control sideband stability KG fixture
- This slice mirrors the prior write-address control-sideband coverage on the read-address channel with `ARPROT`, `ARCACHE`, and `ARLOCK`.
- `axi_read_address_control_sideband_stability_gold` proves those sidebands survive as graph-backed Manager outputs and actor-grounded stability obligations under the controlling `ARVALID` / `ARREADY` handshake.
- The fixture intentionally stays width-only at the table level and relies on prose actor relations for direction, matching real AXI tables where ownership is often implied by channel context rather than repeated in every row.
- The temporal expectations require `HandshakeComplete(ARVALID, ARREADY)` and Manager-grounded `actor_maintains_signal_stable` consequents for all three read-address control sidebands, preventing these fields from being treated as inert inventory.
- The refreshed corpus-KB projection now records the tracked suite as `79/79`, AMBA-family fixture coverage as `30/30`, and timing-family coverage as `36/36`.

## 2026-04-16 AXI write-address control sideband stability KG fixture
- This slice extends write-address sideband stability coverage from burst geometry and transaction identity into control attributes: `AWPROT`, `AWCACHE`, and `AWLOCK`.
- `axi_write_address_control_sideband_stability_gold` proves those sidebands survive as graph-backed Manager outputs and actor-grounded stability obligations under the controlling `AWVALID` / `AWREADY` handshake.
- The fixture intentionally stays width-only at the table level and relies on prose actor relations for direction, matching real AXI tables where ownership is often implied by channel context rather than repeated in every row.
- The temporal expectations require `HandshakeComplete(AWVALID, AWREADY)` and Manager-grounded `actor_maintains_signal_stable` consequents for all three control sidebands, preventing these fields from being treated as inert inventory.
- The refreshed corpus-KB projection now records the tracked suite as `78/78`, AMBA-family fixture coverage as `29/29`, and timing-family coverage as `35/35`.

## 2026-04-16 AXI read-address ID stability KG fixture
- This slice completes the current explicit AXI transaction-ID stability quartet by adding `ARID` beside `AWID`, `BID`, and `RID`.
- `axi_read_address_id_stability_gold` proves that `ARID` survives as a graph-backed Manager output and actor-grounded stability obligation under the controlling `ARVALID` / `ARREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching real AXI tables where ID ownership is often implied by channel-level prose rather than repeated in every table row.
- The temporal expectation requires `HandshakeComplete(ARVALID, ARREADY)` and a Manager-grounded `actor_maintains_signal_stable` consequent, ensuring read-address transaction identity is treated as a stable protocol fact rather than inert table inventory.
- The refreshed corpus-KB projection now records the tracked suite as `77/77`, AMBA-family fixture coverage as `28/28`, and timing-family coverage as `34/34`.

## 2026-04-16 AXI write-address ID stability KG fixture
- This slice extends explicit AXI transaction-ID sideband coverage onto the write-address channel with `AWID`.
- `axi_write_address_id_stability_gold` proves that `AWID` survives as a graph-backed Manager output and actor-grounded stability obligation under the controlling `AWVALID` / `AWREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching real AXI tables where ID ownership is often implied by channel-level prose rather than repeated in every table row.
- The temporal expectation requires `HandshakeComplete(AWVALID, AWREADY)` and a Manager-grounded `actor_maintains_signal_stable` consequent, ensuring write transaction identity is treated as a stable protocol fact rather than inert table inventory.
- The refreshed corpus-KB projection now records the tracked suite as `76/76`, AMBA-family fixture coverage as `27/27`, and timing-family coverage as `33/33`.

## 2026-04-16 AXI read-data ID stability KG fixture
- This slice pairs the prior write-response `BID` fixture with the read-data channel transaction-ID sideband `RID`.
- `axi_read_data_id_stability_gold` proves that `RID` survives as a graph-backed Subordinate output and actor-grounded stability obligation under the controlling `RVALID` / `RREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching real AXI tables where ID ownership is often implied by channel-level prose rather than repeated in every table row.
- The temporal expectation requires `HandshakeComplete(RVALID, RREADY)` and a Subordinate-grounded `actor_maintains_signal_stable` consequent, ensuring read transaction identity is treated as a stable protocol fact rather than inert table inventory.
- The refreshed corpus-KB projection now records the tracked suite as `75/75`, AMBA-family fixture coverage as `26/26`, and timing-family coverage as `32/32`.

## 2026-04-16 AXI write-response ID stability KG fixture
- This slice begins explicit AXI transaction-ID sideband coverage, starting with `BID` on the write-response channel.
- `axi_write_response_id_stability_gold` proves that `BID` survives as a graph-backed Subordinate output and actor-grounded stability obligation under the controlling `BVALID` / `BREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching real AXI tables where ID ownership is often implied by channel-level prose rather than repeated in every table row.
- The temporal expectation requires `HandshakeComplete(BVALID, BREADY)` and a Subordinate-grounded `actor_maintains_signal_stable` consequent, ensuring transaction identity is treated as a stable protocol fact rather than inert table inventory.
- The refreshed corpus-KB projection now records the tracked suite as `74/74`, AMBA-family fixture coverage as `25/25`, and timing-family coverage as `31/31`.

## 2026-04-16 AXI read-data response stability KG fixture
- This slice fills the read-data response sideband gap left after `RDATA` payload stability and `RLAST` last-beat stability were covered separately.
- `axi_read_data_response_stability_gold` proves that `RRESP` survives as a graph-backed Subordinate output and actor-grounded stability obligation under the controlling `RVALID` / `RREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching real AXI tables where ownership is often stated outside the signal table itself.
- The temporal expectation requires `HandshakeComplete(RVALID, RREADY)` and a Subordinate-grounded `actor_maintains_signal_stable` consequent, giving the read-data channel response field the same executable truthfulness treatment as payload and last-beat fields.
- The refreshed corpus-KB projection now records the tracked suite as `73/73`, AMBA-family fixture coverage as `24/24`, and timing-family coverage as `30/30`.

## 2026-04-16 AXI read-address sideband stability KG fixture
- This slice fills the read-address sideband gap left after `ARLEN` was covered by the broader sideband fixture.
- `axi_read_address_sideband_stability_gold` proves that `ARSIZE` and `ARBURST` survive as graph-backed Manager outputs and actor-grounded stability obligations under the controlling `ARVALID` / `ARREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching real AXI tables where ownership is often stated outside the signal table itself.
- The temporal expectations require `HandshakeComplete(ARVALID, ARREADY)` and Manager-grounded `actor_maintains_signal_stable` consequents for both sideband fields, pairing the existing write-address sideband coverage with the corresponding read-address size/type fields.
- The refreshed corpus-KB projection now records the tracked suite as `72/72`, AMBA-family fixture coverage as `23/23`, and timing-family coverage as `29/29`.

## 2026-04-16 AXI write-data last stability KG fixture
- This slice mirrors the prior `RLAST` coverage on the write-data channel by focusing on `WLAST`, the manager-driven write burst last-beat indicator.
- `axi_write_data_last_stability_gold` proves that `WLAST` survives as a graph-backed Manager output and actor-grounded stability obligation under the controlling `WVALID` / `WREADY` handshake.
- The fixture intentionally stays width-only at the table level and relies on prose actor relations for direction, matching real AXI tables where ownership is often stated outside the signal table itself.
- The temporal expectation requires `HandshakeComplete(WVALID, WREADY)` and a Manager-grounded `actor_maintains_signal_stable` consequent, giving the suite a paired read/write last-beat sideband stability check.
- The refreshed corpus-KB projection now records the tracked suite as `71/71`, AMBA-family fixture coverage as `22/22`, and timing-family coverage as `28/28`.

## 2026-04-16 AXI read-data last stability KG fixture
- This slice adds the subordinate-owned side of AXI sideband stability by focusing on `RLAST`, the read-data last-beat indicator.
- `axi_read_data_last_stability_gold` proves that `RLAST` survives as a graph-backed Subordinate output and actor-grounded stability obligation under the controlling `RVALID` / `RREADY` handshake.
- The fixture intentionally keeps the table width-only and relies on prose actor relations for direction, matching the same real-document AXI pattern used by the read-data timing and write-address sideband fixtures.
- The temporal expectation requires `HandshakeComplete(RVALID, RREADY)` and a Subordinate-grounded `actor_maintains_signal_stable` consequent, giving the suite a completed-handshake sideband stability case where the producer is not the Manager.
- The refreshed corpus-KB projection now records the tracked suite as `70/70`, AMBA-family fixture coverage as `21/21`, and timing-family coverage as `27/27`.

## 2026-04-16 AXI write-address sideband stability KG fixture
- This slice fills the write-address sideband hole left after the broader AXI sideband fixture covered `ARLEN` and `WSTRB`.
- `axi_write_address_sideband_stability_gold` proves that `AWLEN`, `AWSIZE`, and `AWBURST` survive as graph-backed Manager outputs and actor-grounded stability obligations under the controlling `AWVALID` / `AWREADY` handshake.
- The fixture intentionally uses a width-only table plus prose actor relations, matching the real AXI extraction path where channel direction often comes from prose or structural context rather than a direction column.
- The temporal expectations require `HandshakeComplete(AWVALID, AWREADY)` for all three sideband holds, which distinguishes completed-channel stability from APB/AHB wait-state stability fixtures that explicitly require zero completion predicates.
- The refreshed corpus-KB projection now records the tracked suite as `69/69`, AMBA-family fixture coverage as `20/20`, and timing-family coverage as `26/26`.

## 2026-04-16 AHB write-data stability KG fixture
- This slice adds the remaining obvious AHB wait-state stability flank: manager-owned write data.
- `ahb_write_data_stability_gold` proves that `HWDATA` is not merely another table row; it becomes a graph-backed Manager output and a typed temporal stability obligation under a three-predicate write wait-state guard.
- The guard `HREADY is LOW and HSEL is HIGH and HWRITE is HIGH` intentionally strengthens the multi-predicate temporal path beyond the prior two-predicate AHB control/response stability fixtures.
- The fixture still requires `0` handshake-completion predicates, preserving the stalled-transfer distinction while proving write-data stability.
- The refreshed corpus-KB projection now records the tracked suite as `68/68`, AMBA-family fixture coverage as `19/19`, and timing-family coverage as `25/25`.

## 2026-04-16 AHB response stability KG fixture
- This slice complements `ahb_control_stability_gold` by checking the subordinate-owned response side of an AHB wait state.
- `ahb_response_stability_gold` proves that `Manager signals` / `Subordinate signals` section context plus `Destination`-column signal tables recover graph-backed Manager/Subordinate ownership for `HSEL`, `HREADY`, `HRDATA`, and `HRESP`.
- The fixture focuses on wait-state stability for `HRDATA` and `HRESP` under the guard `HREADY is LOW and HSEL is HIGH`.
- The important truthfulness point is the same as the manager-control fixture: a stalled transfer can impose stability obligations, but it must not fabricate a completed-handshake predicate.
- The refreshed corpus-KB projection now records the tracked suite as `67/67`, AMBA-family fixture coverage as `18/18`, and timing-family coverage as `24/24`.

## 2026-04-16 AHB control stability KG fixture
- This slice extends the AHB side of the protocol-grade timing/stability suite instead of adding another AXI/APB variant.
- `ahb_control_stability_gold` proves that `Manager signals` / `Subordinate signals` section context plus `Destination`-column signal tables recover graph-backed Manager/Subordinate ownership for AHB address/control signals.
- The fixture focuses on wait-state stability for `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` under the guard `HREADY is LOW and HSEL is HIGH`.
- This intentionally mirrors the APB wait-state distinction: the pipeline must recover actor-grounded stability obligations while still reporting `0` handshake-completion predicates, because a stalled transfer is not a completed transfer.
- The refreshed corpus-KB projection now records the tracked suite as `66/66`, AMBA-family fixture coverage as `17/17`, and timing-family coverage as `23/23`.

## 2026-04-16 APB response stability KG fixture
- This slice pairs with the APB write-control wait-state fixture by proving the completed-transfer response side.
- `apb_response_stability_gold` proves that `Signal | Source | Width | Description` APB tables plus structured constraints recover completer-owned stability obligations for `PRDATA` and `PSLVERR`.
- The guard intentionally uses the completed APB access shape: `PSEL` is `HIGH`, `PENABLE` is `HIGH`, and `PREADY` is `HIGH`. Because `PSEL` and `PREADY` have table-grounded valid-like / ready-like roles, the resulting temporal rules must include `HandshakeComplete(PSEL, PREADY)`.
- This gives R15e a useful pair of APB structured-constraint gold fixtures: `PREADY LOW` proves no false handshake completion while requester-owned sidebands stay stable, and `PREADY HIGH` proves response stability with a real completion predicate and completer-owned stability.
- The refreshed corpus-KB projection now records the tracked suite as `65/65`, AMBA-family fixture coverage as `16/16`, and timing-family coverage as `22/22`.

## 2026-04-16 APB write-control stability KG fixture
- This slice continues R15e structured-constraint coverage on APB instead of adding more AXI-only variants.
- `apb_write_control_stability_gold` proves that `Signal | Source | Width | Description` APB tables plus structured constraints recover requester-owned stability obligations for `PWRITE`, `PWDATA`, and `PSTRB`.
- The guard is intentionally the APB wait-state shape: `PSEL` is `HIGH`, `PENABLE` is `HIGH`, and `PREADY` is `LOW`. That checks multi-predicate temporal antecedents and actor-grounded stability without allowing the model to mistake a wait state for completed ready/valid transfer semantics.
- The fixture asserts `0` temporal rules with handshake completion while still requiring three actor-grounded temporal rules, so this is a useful negative-edge inside a gold fixture: stability must be recovered, but handshake completion must not be fabricated.
- The refreshed corpus-KB projection now records the tracked suite as `64/64`, AMBA-family fixture coverage as `15/15`, and timing-family coverage as `21/21`.

## 2026-04-16 AXI sideband stability KG fixture
- This slice extends the AXI channel sweep from primary payload/address stability into explicit sideband stability, which is a separate truthfulness obligation in real protocol specs.
- `axi_sideband_stability_gold` proves that a width-only signal table plus prose actor relations can carry two AXI channel fragments at once: `ARVALID` / `ARREADY` / `ARLEN` for read-address sideband stability and `WVALID` / `WREADY` / `WSTRB` for write-data byte-lane sideband stability.
- The fixture intentionally focuses the stability rules on `ARLEN` and `WSTRB`, not the already-covered primary `ARADDR` or `WDATA` payloads. That checks whether non-handshake sideband fields survive as graph-grounded, actor-owned temporal obligations instead of being treated as inert table inventory.
- Temporal expectations stay typed and actor-grounded: `ARLEN must not change when ARVALID is HIGH and ARREADY is HIGH` and `WSTRB must not change when WVALID is HIGH and WREADY is HIGH` both require `HandshakeComplete` antecedents plus `actor_maintains_signal_stable` consequents for `Manager`.
- The corpus-KB fixture-family classifier now treats `stability` fixture names as temporal semantics too, so review projections count this fixture under both AMBA-family protocol coverage and temporal fixture coverage.
- The refreshed corpus-KB projection now records the tracked suite as `63/63`, AMBA-family fixture coverage as `14/14`, and timing-family coverage as `20/20`.

## 2026-04-16 AXI read-address timing KG fixture
- This slice closes the obvious AXI channel timing sweep by adding the manager-driven read-address path beside write-address, write-response, read-data, and write-data coverage.
- `axi_read_address_timing_gold` proves the width-only table plus prose actor-relation path on the AXI read-address channel: `ARVALID`, `ARADDR`, and `ARLEN` are driven by `Manager`, while `ARREADY` is driven by `Subordinate`, with reciprocal sample/read relations preserved.
- The fixture intentionally carries `ARLEN` as graph-grounded channel evidence even though the timing rule focuses on `ARADDR`; this checks that control sideband fields survive as canonical actor-relative interface records without forcing a handshake role.
- Temporal expectations stay typed and actor-grounded: `ARREADY must be asserted on the next cycle` binds the post-tick drive/value obligation to `Subordinate`, and `ARADDR must not change when ARVALID is HIGH and ARREADY is HIGH` binds stability to the `ARVALID` / `ARREADY` handshake.
- The refreshed corpus-KB projection now records the tracked suite as `62/62`, AMBA-family fixture coverage as `13/13`, and timing-family coverage as `19/19`.

## 2026-04-16 AXI write-data timing KG fixture
- This slice continues the same R15e AXI channel sweep with the manager-driven write-data path, rather than expanding the implementation surface before the executable protocol coverage is broader.
- `axi_write_data_timing_gold` proves the width-only table plus prose actor-relation path on the AXI write-data channel: `WVALID`, `WDATA`, and `WSTRB` are driven by `Manager`, while `WREADY` is driven by `Subordinate`, with reciprocal sample/read relations preserved.
- The fixture intentionally carries `WSTRB` as graph-grounded channel evidence even though the timing rule focuses on `WDATA`; this checks that byte-lane sideband signals survive as canonical actor-relative interface records without forcing a handshake role.
- Temporal expectations stay typed and actor-grounded: `WREADY must be asserted on the next cycle` binds the post-tick drive/value obligation to `Subordinate`, and `WDATA must not change when WVALID is HIGH and WREADY is HIGH` binds stability to the `WVALID` / `WREADY` handshake.
- The refreshed corpus-KB projection now records the tracked suite as `61/61`, AMBA-family fixture coverage as `12/12`, and timing-family coverage as `18/18`.

## 2026-04-16 AXI read-data timing KG fixture
- This slice continues `R15e` by filling another concrete AXI channel shape rather than broadening the code surface prematurely.
- `axi_read_data_timing_gold` proves the width-only table plus prose actor-relation path on the AXI read-data channel: `RVALID`, `RDATA`, and `RRESP` are driven by `Subordinate`, while `RREADY` is driven by `Manager`, with reciprocal sample/read relations preserved.
- The fixture intentionally carries `RRESP` as graph-grounded channel evidence even though the timing rule focuses on `RDATA`; this checks that non-timed payload/response signals still survive as canonical actor-relative interface records.
- Temporal expectations stay typed and actor-grounded: `RVALID must be asserted on the next cycle` binds the post-tick drive/value obligation to `Subordinate`, and `RDATA must not change when RVALID is HIGH and RREADY is HIGH` binds stability to the `RVALID` / `RREADY` handshake.
- The refreshed corpus-KB projection now records the tracked suite as `60/60`, AMBA-family fixture coverage as `11/11`, and timing-family coverage as `17/17`.

## 2026-04-15 AXI write-response timing KG fixture
- This slice deepens `R15e` along the current AXI quality gap rather than adding another broad feature.
- `axi_write_response_timing_gold` is the first tracked AXI write-response-channel timing fixture: a width-only signal table plus prose actor relations must recover `BVALID`, `BREADY`, and `BRESP` as graph-backed actor-relative signals.
- The fixture proves the same meaning-based handshake machinery generalizes beyond the existing AXI write-address `AW*` coverage: `BVALID` resolves as valid-like, `BREADY` resolves as ready-like, both carry single-source semantic grounding, and the `BVALID` / `BREADY` handshake grounds `BRESP` stability.
- Temporal expectations are typed, not aggregate-only: `BVALID must be asserted on the next cycle` carries an actor-grounded drive consequent for `Subordinate`, and `BRESP must not change when BVALID is HIGH and BREADY is HIGH` carries an actor-grounded stability consequent.
- The refreshed corpus-KB projection now records the tracked suite as `59/59`, AMBA-family fixture coverage as `10/10`, and timing-family coverage as `16/16`.

## 2026-04-14 Hosted CI temporarily manual-only
- The GitHub Actions account is near the included monthly minutes limit, so SPECFORGE hosted CI has been deliberately paused for automatic triggers.
- `.github/workflows/ci.yml` now exposes only `workflow_dispatch`, with `push` and `pull_request` triggers removed until the user explicitly asks to re-enable hosted automatic CI.
- This is a cost-control policy change, not a reduction of the quality gate: local `bash scripts/run_ci.sh` remains the canonical pre-commit/pre-push Rust + docs validation command and still matches the hosted workflow body.
- When automatic hosted CI is re-enabled, restore the `push` / `pull_request` triggers while keeping the workflow delegated to `./scripts/run_ci.sh`.

## 2026-04-14 KG-bench semantic-grounding strength expectations
- This slice moves semantic grounding-strength truth from validation-count confidence into direct canonical signal assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `InterfaceSignalRecord.semantic_grounding_strength` through `semantic_grounding_strengths_include`, matching a signal name plus exact strength such as `single_source`, `multi_source`, or `cross_modality`.
- `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` now prove exact grounding-strength shape for multimodal consensus, VLM timing-note semantic grounding, visual-caption prior guidance, and APB requester/completer handshake grounding.
- This keeps the grounding model executable as typed IR shape, so future changes cannot quietly collapse cross-modality evidence back into single-source-looking semantics or inflate a single local source into stronger grounding.

## 2026-04-14 KG-bench resolved semantic-role expectations
- This slice moves semantic-role truth from existence-only confidence into direct canonical role assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `InterfaceSignalRecord.resolved_semantic_role` through `resolved_semantic_roles_include`, matching a signal name plus exact role such as `handshake_valid_like` or `handshake_ready_like`.
- `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` now prove exact canonical role shape for table+visual consensus, VLM timing-note semantic grounding, visual-caption prior guidance, and APB requester/completer handshake grounding.
- This closes the gap where a fixture could pass by proving "a role resolved" while accidentally allowing the wrong semantic role to survive into `IntentIR`.

## 2026-04-14 KG-bench resolved signal-polarity expectations
- This slice moves resolved polarity truth from validation-count confidence into direct canonical signal assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `InterfaceSignalRecord.resolved_polarity` through `signal_polarities_include`, matching a signal name plus `active_high` / `active_low`.
- `non_reset_control_polarity_gold`, `multi_control_polarity_gold`, and `mixed_control_polarity_gold` now prove the exact resolved polarity shape for explicit asserted-when-level prose, collective active-low prose, and mixed clause-local active-low/active-high prose.
- This keeps the polarity surface executable as typed IR shape instead of only proving the aggregate `with_resolved_polarity` metric increased.

## 2026-04-14 KG-bench signal-polarity conflict expectations
- This slice moves active-level disagreement truth from count-only confidence into direct typed conflict assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `signal_polarity_conflicts` with partial matching on conflict signal, optional conflict id, observation polarity, observation source kind, supporting statement ids, and supporting table ids.
- The new `control_polarity_conflict_negative` fixture proves a `PRESETN` active-high prose observation and active-low signal-description-table observation survive as an explicit carried conflict.
- This is intentionally truth-preserving rather than winner-picking: contradictory polarity evidence should remain inspectable until stronger arbitration or current-document clarification exists.

## 2026-04-13 KG-bench interface-signal conflict expectations
- This slice moves interface declaration conflict truth from count-only confidence into direct typed conflict assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `interface_signal_conflicts` with partial matching on conflict signal, optional conflict id, conflict kind, and included observation values plus supporting statement ids.
- `negative_knowledge_prior_guided_interface_conflict_caution_gold` now proves the `DATA` interface-shape conflict is specifically both a `direction_mismatch` over `input` / `output` and a `width_mismatch` over `8` / `16`.
- This preserves the same prior boundary as the other negative-knowledge caution fixtures: prior memory can add caution/rescan/corroboration guidance, but it cannot repair, hide, or rewrite current-document interface-shape truth.

## 2026-04-13 KG-bench signal-connectivity conflict expectations
- This slice moves multi-producer graph conflict truth from count-only confidence into direct typed conflict assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `signal_connectivity_conflicts` with partial matching on conflict signal, optional conflict id, conflict kind, conflicting actor ids/names, and supporting statement ids.
- `multi_producer_conflict_negative` now proves the `PREADY` conflict is specifically a `multiple_producers` conflict over `Completer` and `Monitor`.
- `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` now proves the same shape survives when prior memory adds caution/rescan/corroboration guidance, preserving the boundary that priors cannot repair local graph truth.

## 2026-04-13 KG-bench signal-semantic conflict expectations
- This slice moves signal-semantic conflict truth from count-only confidence into direct typed conflict assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `signal_semantic_conflicts` with partial matching on conflict signal, optional conflict id, and included observations by semantic tags, source kind, source text, and supporting statement/table/visual evidence ids.
- `visual_sources_semantic_conflict_negative` now proves the `XCTRL` multimodal disagreement is a real shape: visual-caption evidence supports `handshake_valid_like`, VLM timing-diagram annotation evidence supports `handshake_ready_like`, and the canonical layers preserve the conflict plus non-decisive arbitration rather than forcing a winner.
- This keeps multimodal arbitration drift visible in the executable benchmark surface, not just in validation metrics or residual/finding side effects.

## 2026-04-13 KG-bench temporal-conflict expectations
- This slice moves temporal contradiction truth from count-only confidence into direct typed temporal-conflict assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `temporal_conflicts` with partial matching on signal name, phase, clock signal, edge, cycle window, antecedent predicates, conflicting values, supporting rule ids, and supporting statement ids.
- `negative_knowledge_prior_guided_temporal_conflict_caution_gold` now proves the `PREADY` post-tick `HIGH` / `LOW` conflict under `HREADY LOW` survives canonically with both contributing temporal rule ids and statement ids.
- This keeps negative-knowledge priors honest: they can add caution/rescan/corroboration guidance, but the current-document contradiction remains visible and cannot be silently repaired by prior memory.

## 2026-04-13 KG-bench temporal-rule expectations
- This slice moves representative APB/AHB/AXI timing truth from validation-count confidence into direct typed temporal-rule assertions.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `temporal_rules` with partial matching on `source_text`, `clock_signal`, `edge`, `cycle_window`, supporting statement ids, antecedent predicates, and consequent predicates.
- The matcher is intentionally partial rather than snapshot-based: fixtures can lock the meaning-bearing predicates they care about while staying robust to unrelated future temporal-rule metadata.
- `axi_next_cycle_timing_gold` now asserts the `AWREADY` one-cycle post-tick assertion and the `AWADDR` handshake-stability rule, including `HandshakeComplete(AWVALID, AWREADY)`.
- `apb_setup_access_timing_gold` now asserts the `PENABLE` one-cycle setup/access transition and the `PADDR` access-phase stability rule, including `HandshakeComplete(PSEL, PREADY)`.
- `ahb_wait_state_timing_gold` now asserts the `HREADY` next-cycle response and the `HTRANS` wait-state stability rule with the compound `HREADY LOW` / `HSEL HIGH` guard.

## 2026-04-13 KG-bench infrastructure topology expectations
- This slice moves clock/reset infrastructure topology from unit-test-only confidence into tracked KG-quality coverage.
- `kg-bench` can now assert `SemanticIR` / `IntentIR` `infrastructure_signals` by signal name, kind, optional source/distribution status, recovered source actor names, and distribution target actor names.
- It can also assert `infrastructure_topology` records by signal name, topology kind, optional component name, optional stage count, and expected target actors.
- The new `clock_reset_topology_gold` fixture locks the doctrine that explicit current-document phrases such as `The ACLK clock gate CGATE0 feeds the Requester branch`, `The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester`, and `The ARESETN reset tree targets the Requester registers and Completer registers` become typed infrastructure topology, while generic advice like `clock gate policy should avoid glitches` or `may use a synchronizer` does not add extra topology records.
- The fixture keeps `actor_ports = 0` in validation to prove topology-only clock/reset evidence stays in the infrastructure surface rather than becoming ordinary protocol actor ports.

## 2026-04-13 VLM timing motion-only annotation filtering
- This slice fixes a false-positive gap in the VLM timing path: label-only annotation filters and waveform-motion `signals[].values[].state` filters were already present, but prose annotations such as `XREQ rises, remains stable, then falls` could still become `TimingConstraintRecord`s.
- That was too permissive because motion-only annotation prose is often figure markup or OCR/VLM commentary, not a numeric timing requirement, temporal bound, setup/hold parameter, or protocol law.
- `SemanticIR` now rejects VLM timing annotations when they contain waveform-motion vocabulary but no timing/constraint indicators and no numeric/cycle-bearing anchor. Concrete signal samples in `signals[].values[]` still use the existing bounded path, so `XREQ` being `HIGH` at `T1` remains typed temporal evidence.
- The positive VLM annotation path remains intact: setup/hold-style annotations and direct semantic-grounding annotations still pass their focused tests/fixtures.
- The new `vlm_timing_motion_annotation_negative` KG fixture captured the bug first, failed with two unexpected timing constraints, and now passes with zero timing constraints, one concrete signal constraint, and one temporal rule.

## 2026-04-13 R15g prior-candidate readiness manifest
- This slice deepens the prior-candidate bridge again, but still keeps the corpus KB out of the truth path.
- `specforge corpus-kb --kg-fixtures-root ...` now emits a tracked `corpus_kb/prior_candidates/kg-fixture-candidates.json` manifest beside the Markdown planning page. The manifest is intentionally machine-readable for review, automation, and future implementation planning, not machine-promoting.
- The manifest records schema version, source, review scope, non-mutation flags, candidate readiness, fixture counts, positive/guard fixture names, required gate text, structured gate identifiers, and the `review_only_no_corpus_memory_or_canonical_ir_mutation` promotion boundary.
- The Markdown page now mirrors that state with a `Readiness Summary` table. Paired prior families use `fixture_paired_review_ready`; negative knowledge uses `caution_surface_review_ready` because its current fixture surface is caution-only rather than a normal gold/without-prior pair.
- This gives future sessions a durable answer to "which prior families look ready to review next?" while preserving the core rule: corpus-KB readiness is not approval, not `CorpusMemory`, and not canonical IR mutation.

## 2026-04-13 R15g typed prior-memory corpus KB page
- This slice adds a dedicated `corpus_kb/prior_memory/kg-fixtures.md` family page for typed prior-memory fixture behavior that was previously visible only through the aggregate benchmark, broad pattern page, and prior-candidate planning surface.
- The page is selected by the existing `typed prior memory` fixture-family label and is refreshed through the same managed-block path as the other KG fixture-family pages.
- The tracked refresh currently projects 21 passing fixtures across prior-guided actor-taxonomy, semantic phrase, semantic modality-reliability, temporal phrase, table-shape, visual semantic, visual-motif, and caution-only negative-knowledge behavior.
- The page is still corpus synthesis only. It can guide prior-memory fixture review and implementation planning, but it is not `generated/prior_memory/corpus_memory.json`, cannot write `CorpusMemory`, and cannot promote cross-document memory into canonical document truth.

## 2026-04-13 R15g state-machine corpus KB page
- This slice adds a dedicated `corpus_kb/state_machines/kg-fixtures.md` family page for VLM state-machine fixture behavior that was previously visible only inside the broader visual page.
- The page is selected by the existing `VLM state machines` fixture-family label and is refreshed through the same managed-block path as the other KG fixture-family pages.
- The tracked refresh currently projects five passing fixtures: duplicate-initial merge, label-noise filtering, missing-initial warning, multiple-initial warning, and undeclared-transition endpoint filtering.
- The page is still corpus synthesis only. It can guide state-machine extraction review, VLM fixture design, and adapter-readiness discussion, but it cannot promote visual hypotheses into canonical IR or typed prior memory.

## 2026-04-13 R15g prior-candidate gate matrix
- This slice deepens the prior-candidate bridge without allowing corpus KB to write prior memory or canonical IR.
- `corpus_kb/prior_candidates/kg-fixture-candidates.md` now explicitly states `review_scope: family_surface_not_individual_prior`, because the page is a family-level review index rather than an approval artifact for a concrete prior record.
- The managed block now emits a `Promotion Gate Review Matrix` with structured schema, fixture, harvest, consumer, and promotion-boundary gates for each candidate family.
- The statuses are intentionally conservative: they name implementation surfaces already visible in the code and KG fixtures, but every row still ends with `review_only_no_corpus_memory_or_canonical_ir_mutation`.

## 2026-04-13 R15g semantic/truthfulness corpus KB patterns
- This slice adds the missing `patterns/` corpus-KB page family for cross-cutting semantic and truthfulness motifs that do not fit cleanly into table, visual, timing, infrastructure, protocol, or prior-candidate pages.
- The page is refreshed from the same `kg_bench::collect_fixture_outcomes()` pass as the aggregate benchmark projection. It selects fixtures labeled as actor/connectivity, semantic role arbitration, negative knowledge, truthfulness negatives/cautions, or residual/caveat behavior.
- The current live projection captures `42/42` passing pattern fixtures with explicit fixture-path provenance. That makes the page useful as a review/debugging index for accepted, contested, blocked, and residual candidate facts.
- The boundary remains deliberately closed: pattern labels and page content are corpus synthesis only. They do not affect fixture evaluation, validation scoring, `CorpusMemory`, or canonical IR promotion.

## 2026-04-13 R15g prior-candidate corpus KB projection
- This slice adds the first explicit bridge from corpus-KB benchmark synthesis toward prior-candidate planning, while keeping the promotion boundary closed.
- `corpus_kb/prior_candidates/kg-fixture-candidates.md` is refreshed from KG fixture names that already encode prior-guided gold/negative/caution behavior. It groups candidates by target `CorpusMemory` schema family and lists the positive plus guard fixtures that make the candidate reviewable.
- The managed block is intentionally machine-shaped but not machine-promoting: it records `promotion_status: candidate_not_promoted_review_required`, `canonical_mutation_allowed: false`, and `corpus_memory_mutation_allowed: false`.
- This is the safe intermediate plane the roadmap asked for: corpus KB can now propose prior-candidate families, but any actual machine-usable prior still has to go through typed `CorpusMemory` schema, validated harvest inputs, local-grounding consumers, and KG-bench/validation gates.

## 2026-04-13 R15g dedicated corpus KB fixture-family pages
- The next `R15g` step turns the family summary from a single aggregate table into dedicated corpus-KB page families for tables, visuals, timing motifs, infrastructure semantics, and AMBA-family protocol notes.
- The implementation deliberately reuses the same `kg_bench::collect_fixture_outcomes()` refresh pass. That keeps the aggregate page and family pages grounded in the same executable benchmark outcomes rather than letting hand-maintained corpus notes drift away from the regression suite.
- The page-family labels are deterministic and review-facing. They are derived from fixture names and used only to organize corpus synthesis; they do not affect fixture evaluation, validation scoring, canonical IR, or typed prior memory.
- The infrastructure page intentionally includes active-low VLM timing and polarity fixtures as infrastructure-adjacent evidence, while keeping non-reset control polarity visible as polarity coverage rather than pretending it proves clock/reset topology.
- Each generated page keeps a human synthesis section outside the managed block, so future review notes can accumulate without being overwritten by refreshes.

## 2026-04-13 R15g KG fixture-family corpus KB summary
- The next `R15g` step deepens the managed KG benchmark projection from flat pass/fail accounting into reviewable fixture-family synthesis.
- The projection now emits a deterministic family summary table before the per-fixture list in `corpus_kb/benchmarks/kg-fixtures.md`. The current categories are intentionally orthogonal: a fixture can count under protocol-family coverage, temporal semantics, VLM evidence, typed-prior behavior, and truthfulness-negative coverage at the same time.
- This remains a corpus-KB review surface, not a new truth source. The family labels summarize benchmark coverage for humans and future LLM sessions; they do not change fixture execution, validation scoring, canonical IR, or `CorpusMemory`.
- The implementation keeps human-authored synthesis outside the managed block intact and keeps failed-family member reporting ready for future regressions, so the page can become a useful debugging index if a fixture family turns red.

## 2026-04-13 R15g quiet KG fixture validation path
- The previous corpus-KB benchmark projection exposed a quality issue: the fixture runner had to invoke `validate::run()` to produce validation sidecars for validation-backed expectations, and that command path printed the full validation report for every fixture.
- The fix keeps public `specforge validate` behavior unchanged while adding `validate::run_quiet()` for internal callers that need the side effects and reports without the user-facing printout.
- The quiet path uses a thread-local output guard around the existing validation implementation instead of duplicating validator logic. This keeps the validation semantics single-sourced and avoids a second, drifting report-construction path.
- `kg-bench` now uses the quiet entrypoint for fixture-local validation. As a result, both `specforge kg-bench` and `specforge corpus-kb --kg-fixtures-root ...` stay focused on fixture pass/fail outcomes rather than dumping validation details for every validation-backed fixture.
- The guard restores the previous output state through `Drop`, so early returns and test panics do not leave the current thread stuck in quiet mode.

## 2026-04-13 R15g KG fixture-result corpus KB projection
- The second `R15g` refreshable page family turns the KG fixture suite into reviewable corpus synthesis without replacing the executable `specforge kg-bench` gate.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` runs the tracked fixture suite through the existing benchmark engine and writes only the managed block in `corpus_kb/benchmarks/kg-fixtures.md`.
- The page is intentionally modest: fixture path, pass/fail status, and failure text if any. It does not summarize canonical truth, mutate IR artifacts, or approve any promotion.
- The implementation exposes a small internal `kg_bench::collect_fixture_outcomes()` seam so the standalone benchmark command and the corpus-KB projection share the same fixture discovery/evaluation path.
- This originally exposed a CLI-noise caveat because fixture evaluation invoked `validate::run()`. The follow-up quiet-validation slice now closes that caveat while keeping validation report construction single-sourced.

## 2026-04-13 R15g corpus knowledge-base bootstrap
- The README bootstrap now moves into `R15g` rather than adding more one-off extractor fixes. The missing plane was not another canonical IR field or another hidden memory file; it was a tracked, reviewable corpus synthesis root that can survive session loss and accumulate cross-document lessons without promoting facts.
- `corpus_kb/` is deliberately separate from `generated/prior_memory/corpus_memory.json`. The prior store is machine-usable extraction memory; the corpus KB is human/LLM-readable synthesis, negative-knowledge notes, validation pattern memory, and prior-candidate staging.
- `specforge corpus-kb` provides the first auto-refreshable page family. It reads validation report sidecars and updates only the managed block in `corpus_kb/failures/validation-findings.md`, preserving human-authored synthesis outside that block.
- This keeps the promotion boundary honest: corpus KB observations can inspire KG-bench fixtures, rescan targets, or future typed prior families, but they cannot mutate `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, adapters, or `CorpusMemory` without a separate validation-gated implementation path.

## 2026-04-12 VLM timing standalone bit-select annotation filtering
- The interrupted recovery slice continues the same KG-quality theme as the compact/bracketed timing-label filters: VLM timing `annotations[]` can contain figure markup that looks signal-like, not just generic tokens like `T0` or `DATA[3]`.
- Standalone annotation labels such as `XREQ[0]`, `XREQ<1>`, and `XREQ[3:0]` are now treated as waveform/bit-select labels rather than timing constraints. That is the safe interpretation because no timing relation, value obligation, or sentence-shaped rule is present in the annotation itself.
- The boundary is intentionally narrow. A real VLM `signals[].values[]` tuple for document-grounded `XREQ` still becomes typed temporal evidence when the value is concrete; only standalone index labels in `annotations[]` are filtered as low-value visual markup.
- The existing `vlm_timing_spurious_annotation_negative` fixture remains the right regression home because this is not a new semantic family. It is a deeper edge case in the same "VLM timing label noise must not become protocol law" contract.

## 2026-04-12 README bootstrap refresh
- The README handoff was executed again after `76274f6`. The current Rust surface now has 29 Rust source files and 56,024 lines under `crates/specforge/src`, with the full staged command set wired through `cli.rs`, `commands/mod.rs`, and `lib.rs`.
- No code change was required by this bootstrap pass. The drift was documentation/continuity drift: `README.md` was missing several active command/module paths in its implementation map, `RUST_CODEBASE_ANALYSIS.md` still carried stale current testing counts in its lower testing section, and `MEMORY.md` still pointed at `da0797f` instead of the latest committed baseline.
- The path-policy cleanup is also intentional. Older changelog/memory entries contained checkout-specific repo-internal markdown links; those were normalized to repo-relative links so tracked docs remain portable across clones.

## 2026-04-11 README bootstrap refresh
- The README terminal instruction currently routes future sessions through `SESSION_BOOTSTRAP.md`; executing that path means reading the referenced live docs, analyzing the Rust codebase, updating `RUST_CODEBASE_ANALYSIS.md` if it drifted, and then continuing from the roadmap.
- This refresh found that the Rust analysis lagged the implementation: `rescan-plan`, schema-v2 replay/execution summaries, the no-canonical-mutation promotion-review boundary, `CorpusMemory` schema v5, 45 tracked KG fixtures, and the 282-test full-CI baseline were all newer than parts of the analysis doc.
- The fix is documentation-only. The engineering decision is to keep `RUST_CODEBASE_ANALYSIS.md` as a living architecture snapshot rather than letting it become a stale historical essay, because it is part of the crash-recovery and handoff contract.
- A follow-up bootstrap hygiene pass after `35a8372` found no new Rust architecture drift. The correct fix was to refresh continuity state to the latest committed baseline and remove stray example bullets from `USER_GUIDE.md`'s root-document list, keeping the compatibility pointer clean while the mdBook remains the canonical public documentation surface.

## 2026-04-11 dead-code warning cleanup
- Rust warnings are treated as quality drift, not harmless background noise.
- The dead-code cleanup deletes stale helper paths instead of adding `#[allow(dead_code)]`: the removed adapter helpers belonged to the old direct `DecisionTreeFragmentRecord` renderability path, while active `.fsm` lowering now validates and renders `ControlBlockRecord` / `ControlActionRecord`; the removed semantic helpers were empty stubs after register/timing carry-through moved into `SemanticIr::build()`.
- Future placeholder helpers should either be wired into an active call path, covered by tests, or left out until the implementation slice genuinely needs them.

## 2026-04-11 CI warning-deny gate
- The clean Rust warning baseline is now enforced by the canonical local/hosted CI path.
- `scripts/run_ci.sh` runs the Rust test step with `RUSTFLAGS="-D warnings"` so warning regressions fail before push locally and in any manually launched GitHub Actions run.
- The warning gate lives in the shared script rather than only in `.github/workflows/ci.yml`; this keeps local and hosted CI behavior aligned.

## 2026-04-11 Clippy gate
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` is now clean and part of the shared `scripts/run_ci.sh` gate.
- Mechanical Clippy warnings should be fixed directly. Intentional broad IR plumbing is allowed only through localized `#[expect(...)]` attributes with a reason, so future unrelated Clippy drift still fails CI.
- GitHub Actions installs both `rustfmt` and `clippy`, then delegates to the shared local CI script so manual hosted behavior stays aligned with local pre-push validation.

## 2026-04-11 rustdoc warning-deny gate
- `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` is now part of the shared `scripts/run_ci.sh` gate.
- Broken intra-doc links and malformed public Rust documentation are treated as CI failures, not as optional cleanup.
- The runner composes caller-provided `RUSTDOCFLAGS` with `-D warnings`, matching the existing `RUSTFLAGS` handling for warning-denied Rust tests.

## 2026-04-11 active-low VLM deassertion fixture
- `vlm_timing_active_low_deassertion_equivalence_gold` now locks the reset-release side of polarity-relative VLM timing evidence.
- Active-low `ARESETN` reported as both `deasserted` and `HIGH` must produce typed temporal evidence without creating a false temporal conflict or polarity conflict.
- This complements the existing reset-entry fixture for `asserted` plus `LOW`, so both assertion and deassertion semantics are regression-protected.

## 2026-04-11 VLM state-machine label filtering
- VLM state-machine extraction is useful evidence, but raw visual labels are still hypotheses and must pass the same canonical-state boundary as explicit state syntax before entering `SemanticIR`.
- State names and transition endpoints extracted from `vlm_state_machine_extraction` are now accepted only when they parse as identifiers. Clean labels such as `IDLE` and `BUSY` survive, while prose/OCR labels such as `IDLE state` and `ACCESS phase` are filtered instead of becoming backend-facing FSM names.
- `vlm_state_machine_label_noise_negative` locks this end to end, and `kg-bench` can now assert state names plus transition endpoints directly.
- VLM transitions now have a second boundary too: both endpoints must refer to state names accepted from the same VLM observation. Identifier-shaped but undeclared endpoints such as `DONE` and `RESET` are filtered rather than treated as canonical graph facts. `vlm_state_machine_undeclared_transition_negative` locks that behavior end to end.
- Duplicate VLM state labels now merge by state name like explicit state declarations do. If any duplicate carries `is_initial: true`, the canonical state preserves that initial marker. `vlm_state_machine_duplicate_initial_gold` locks the case where `IDLE` appears first as non-initial and later as initial, while `kg-bench` now exposes direct initial-state expectations.
- `specforge validate` now also reports `initial_regular_states` for `SemanticIR` and `IntentIR`, so exactly-one-initial FSM truth is visible in validation reports and can be locked by KG fixtures instead of only inferred from state names.
- `specforge validate` now also emits semantic and intent state-machine initial-cardinality warnings when a state graph has zero or multiple initial states. `vlm_state_machine_multiple_initial_negative` locks the multi-initial VLM case while preserving the graph for review instead of hiding it.
- `vlm_state_machine_missing_initial_negative` locks the missing-initial companion case: VLM-authored states and transitions still remain visible, but both semantic and intent validation emit the same initial-cardinality warning when no state is marked initial.

## 2026-04-11 VLM timing waveform motion filtering
- VLM timing `signals[].values[].state` strings are not all equally authoritative signal values.
- Concrete sampled values such as `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` can become typed signal constraints when the signal name is document-grounded, but waveform motion descriptors such as `rising`, `falling`, `stable`, `steady`, `unchanged`, `toggle`, `RISING_EDGE`, `LOW_TO_HIGH`, `POS_EDGE`, `risingedge`, and `LOW2HIGH` should not be promoted through the generic symbolic-value fallback.
- This preserves useful timing observations while avoiding false facts like `XREQ == RISING`. The `vlm_timing_waveform_motion_negative` KG fixture locks the boundary end-to-end.
- The spurious-annotation side has the same boundary: standalone waveform/sample labels such as `D0`, `A1`, `DATA0`, `0xAA`, `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` are now treated like `T0`, `Addr 1`, and `Cycle 2` when they appear in VLM timing `annotations[]`. They remain figure markup, not timing constraints; `vlm_timing_spurious_annotation_negative` locks that expanded label-noise set.

## 2026-04-11 collective non-reset control polarity
- `EvidenceIR` now recovers unambiguous collective active-level prose such as `CS_N and WE_N are active LOW signals`, producing one explicit polarity observation per declared signal and letting later asserted/deasserted constraints refine into concrete low/high obligations.
- The extractor deliberately stays conservative for mixed compound polarity prose such as `CS_N is active LOW and ENABLE is active HIGH`; until there is a clause-local parser that can bind each polarity phrase to exactly one signal, the safer behavior is to leave the polarity unresolved rather than guess.
- The KG fixture exposed a second quality issue: collective polarity prose was also being treated as a low-confidence heuristic interface group, duplicating already declared `CS_N` / `WE_N` records and inflating `with_resolved_polarity` from `2` to `4`.
- The semantic fix is bounded to polarity-only co-mentions of already authoritative signals. Single-signal declared-control prose remains suppressed as before, collective polarity-only declarations enrich the authoritative records, and non-polarity multi-signal co-mentions can still become heuristic grouping evidence when they may carry real interface structure.
- `multi_control_polarity_gold` now locks the end-to-end behavior with two canonical signal records, two resolved polarities, two refined constraints, and zero polarity/temporal conflicts through `EvidenceIR`, `SemanticIR`, and `IntentIR`.

## 2026-04-11 mixed clause-local control polarity
- The mixed-polarity gap left by the collective slice now has a bounded parser: if prose says `CS_N is active LOW and ENABLE is active HIGH`, `EvidenceIR` can split the local clauses and recover active-low `CS_N` plus active-high `ENABLE`.
- This is intentionally not a general anaphora or implicit-subject parser. The fallback only succeeds when every mentioned known signal is recovered and every recovered polarity clause names exactly one known signal.
- Detached wording such as `CS_N is active LOW and active HIGH` still stays unresolved, because the second polarity phrase has no explicit signal anchor and could otherwise create a silent wrong fact.
- `mixed_control_polarity_gold` locks the end-to-end behavior with two canonical declared signal records, zero heuristic duplicates, two resolved polarities, and no polarity/temporal conflicts through the staged IR pipeline.

## Foundational engineering choices
### IntentIR instead of AST
- the final canonical output must capture semantics and implementation-relevant intent, not only syntax structure
- `IntentIR` is therefore a better name and design target than a plain `AST`
- the canonical model must carry assumptions, constraints, abstractions, and residual decisions explicitly

### Backend independence first
- `.fsm` is not the product boundary
- `.fsm`, SystemVerilog, Verilog, and VHDL are adapter targets downstream of `IntentIR`
- the canonical model must not inherit backend-specific assumptions too early

### Software-interface documents are valid intent sources
- firmware-facing and software-interface documents associated with chips or components can carry implementation intent
- the canonical model should therefore capture interface and behavior facts without assuming the evidence is only RTL-facing hardware prose

### Typed IR first
- the internal system of record should be typed Rust data, not markdown prose or string templates
- JSON serialization is the first interchange surface for stage artifacts
- markdown docs explain and steer the system, but they must not become the hidden runtime IR

### SOTA document understanding, not markdown-only extraction
- PDFs must be treated as multimodal documents, not as plain text containers
- the preferred architecture is hybrid and provenance-first:
  - structured parser first
  - page and visual asset capture second
  - selective multimodal enrichment for figures, charts, diagrams, and image-heavy regions third
- markdown is a convenient normalized view for humans and some downstream text steps, but it is not the only system of record for PDF sources
- the normalization layer should remain backend-pluggable so `specforge` can keep pace with the state of the art without destabilizing later IR stages

### Multimodal semantic recovery as the core extraction strategy
- the real objective is not "parse PDFs" but recover enough grounded implementation intent from chip-design documents that downstream tools can generate RTL, verification artifacts, and related implementation-facing outputs
- this requires treating the full document as an evidence field instead of privileging prose alone:
  - tables are latent declarations, encodings, polarity facts, timing fragments, and actor-role hints
  - figures are executable behavioral evidence, not decorative assets
  - prose often carries the protocol law that explains how the tables and figures should be interpreted
- the system should therefore keep trying to make sense of as many document regions as possible, provided the recovered facts remain provenance-carrying and typed
- the elegant path is staged synthesis, not brute-force prompting and not a pile of protocol-specific heuristics:
  - let early recovered facts seed a KG
  - use that KG as a search index for the next rescan over tables, figures, and prose
  - let each pass unlock new anchors, attributes, and temporal relations
  - stop only when the backannotated knowledge stabilizes
- "thinking out of the box" in this project means inventing document-native recovery strategies when ordinary extraction fails, while still keeping the architecture disciplined:
  - preserve provenance
  - preserve ambiguity as residual decisions
  - keep the IR boundaries clean
  - prefer reusable evidence-to-knowledge lifting patterns over one-off protocol patches
- the downstream adapters should consume truth, not beautified guesses; when in doubt, the right move is to enrich the KG and temporal model, not to make the adapters more speculative
- VLM timing-diagram signal/value tuples are allowed to become typed temporal evidence only through the same bounded path:
  - document-grounded signal names can become `SignalConstraintRecord` entries
  - values like `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` should normalize into typed signal-constraint kinds
  - `ASSERTED` / `DEASSERTED` stay polarity-relative when temporal conflicts are evaluated, so an active-low reset observed as both `asserted` and `LOW` or as both `deasserted` and `HIGH` is equivalent rather than contradictory
  - generic visual words like `transfer` must remain rejected as fake signal names
  - unknown/don't-care values should stay unpromoted instead of creating false temporal facts
  - waveform motion descriptors and transition spellings like `rising`, `falling`, `stable`, `unchanged`, `RISING_EDGE`, `LOW_TO_HIGH`, `POS_EDGE`, `risingedge`, and `LOW2HIGH` should stay unpromoted instead of becoming fake symbolic signal values
  - the tracked `vlm_timing_active_low_assertion_equivalence_gold` and `vlm_timing_active_low_deassertion_equivalence_gold` KG fixtures now lock these active-low VLM timing edge cases end-to-end
- VLM state-machine guard text is still a bounded hypothesis, not an authority:
  - simple comparisons like `PREADY = 1` should be converted into typed guards
  - generic visual prose like `transfer` must not become a fake signal unless it is document-grounded as a signal
  - literal guard values such as `0`, `1`, `HIGH`, `LOW`, `ASSERTED`, and `DEASSERTED` should stay literal instead of being promoted into signal references
  - compound guard text should preserve the strongest grounded atomic clause rather than storing the whole VLM phrase as a signal name

### Staged IR pipeline
- `SourceIR` captures normalized source identity, parser backend choice, page artifacts, visual assets, and ingest intent
- `EvidenceIR` captures text anchors, visual evidence, cross-links between text and figures, extracted statements, and statement classification
- `SemanticIR` captures actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- `IntentIR` is the canonical backend-independent intent model
- adapters lower `IntentIR` into concrete targets

### Residual decision packets instead of ad hoc manual gaps
- when automation cannot safely choose a single interpretation, the system should emit a structured residual decision packet
- residual decisions must be explicit in the typed model, not buried in prose
- this keeps the manual surface reviewable and progressively reducible

### Deterministic versus assisted stages
- deterministic stages should own ingest, normalization, artifact materialization, and validation boundaries
- interpretation-heavy stages such as actor discovery and semantic lifting can use assisted reasoning later, but must still emit typed artifacts with provenance

### SourceIR maturity boundary
- `specforge ingest` and `SourceIR` are not the same thing:
  - `ingest` is the stage/command that performs source normalization and materialization
  - `SourceIR` is the typed artifact/model produced by that stage
- this distinction matters because implementation effort on Tier 1 was not "docs about PDFs"; it was information-preservation work on the deterministic foundation the later KG layers depend on
- the project should treat `SourceIR` as strategically high leverage because later stages cannot recover structure that ingest already lost:
  - table identity
  - table cell grids
  - section hierarchy
  - figure/caption linkage
  - page-local provenance
  - visual asset identity
- that earlier investment was objectively correct because it reduces AI uncertainty and raises the ceiling for every later stage
- but Tier 1 should not now become the default focus of the roadmap
- the honest maturity assessment is:
  - `SourceIR` is strong in architecture
  - `specforge ingest` is operational and useful
  - neither should be assumed robust against every real chip-design PDF "without flinching"
- the unresolved Tier 1 risks are mostly robustness risks, not missing-concept risks:
  - scanned/OCR-heavy PDFs
  - multi-column reading-order drift
  - rotated, split, or nested tables
  - unusual caption/figure layouts
  - backend-dependent table-kind classification errors
  - vendor-specific appendices, sidebars, and footnote-heavy pages
- the correct remaining Tier 1 posture is therefore surgical hardening, not broad feature expansion:
  - add robustness benchmarks on varied PDF corpora
  - improve failure-mode detection and honest residuals
  - strengthen source-level validation metrics
  - patch capture bottlenecks when real documents expose them
- unless a real PDF proves otherwise, the center of gravity should stay in `EvidenceIR -> SemanticIR -> IntentIR`, where semantic truthfulness is still the dominant risk

### Continuity as infrastructure
- live documentation is not optional process overhead
- `README.md`, `INTENTIR_SPEC.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `USER_GUIDE.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` are part of the engineering system
- they must be updated when work completes and at meaningful intermediate checkpoints during long-running tasks
- the mdBook under `docs/book/` is now part of that same continuity and product contract:
  - it is the canonical user-facing documentation surface
  - it should be treated as a live book that evolves with the project, not as a static scaffold
  - it is the world-facing documentation product for `specforge`, so it should openly explain what the tool does, how it works, and why it is designed that way
  - every meaningful user-facing aspect of the project should land in the book with its own section or chapter as the coverage grows
  - meaningful changes to commands, runtime setup, stage semantics, validation surfaces, and learning-plane behavior should update the book in the same task when they change the user-facing contract
- the root markdown docs still matter, but their roles are now more specialized:
  - the book is the primary layered public reference
  - root docs continue to carry roadmap, architecture, validation projection, crash continuity, engineering rationale, and handoff state after session loss or crash

## Current execution defaults and convergence accounting
- `specforge converge` is now the default full loop-backed pipeline entrypoint: it uses Ollama-backed VLM image enrichment and NLP Level 3 unless the caller explicitly opts out with `--vlm-provider skip` and/or `--nlp-provider skip`
- the converge command's `knowledge_fact_count` now tracks persisted IR knowledge rather than downstream adapter residual work, so fewer residual decisions on later passes do not falsely look like knowledge loss
- the current local AMBA validation baseline is AXI 85/100, APB 94/100, AHB 94/100, and AXI-Stream 90/100 after the latest refresh against the current extraction stack; the old optimistic snapshot is gone, fresh `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` rebuilds restored APB/AHB to the excellent lane, and AXI is now the main live protocol-quality outlier after field-like message tables stopped leaking pseudo-signals into the canonical layers
- AXI `IHI0022_L` now converges cleanly in 2 outer passes; the old false failure was caused by counting decreasing adapter residual decisions against the monotone knowledge metric

## GitHub CI baseline
- repository-hosted CI is now part of the project baseline rather than an optional afterthought
- the GitHub Actions workflow now calls the same checked-in runner used locally: `./scripts/run_ci.sh`
- the current canonical Rust CI runner executes:
  - `cargo fmt --all --check`
  - `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings`
  - `RUSTFLAGS="-D warnings" cargo test --manifest-path Cargo.toml`
  - `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps`
  - `./scripts/run_docs_ci.sh`

## Docling runtime hardening
- the Docling runtime should not depend on whichever `python3` happens to be first on `PATH`
- the supported operational shape is now:
  - explicit override with `SPECFORGE_DOCLING_PYTHON`
  - otherwise auto-discover a repo-local `.venv-docling`
  - otherwise probe versioned Python candidates such as `python3.11` before generic `python3` / `python`
- `specforge doctor [--strict]` is now the first-class readiness check for that runtime boundary, for the default Ollama loopback path (`/api/tags`, `/v1/chat/completions`, default model presence), and for the LM Studio fallback path (`/v1/models`, `/v1/chat/completions`, default model presence)
- `scripts/bootstrap_docling.sh` is now the supported repo-local bootstrap path and targets the known-good `docling==2.84.0` runtime family
- `.venv-docling/` must stay local and untracked, just like `generated/`
- this matters because fresh original-PDF reruns should fail for genuine ingest/extraction reasons, not because the CLI silently picked an unusable Python interpreter or launched a long converge run against an unusable local Ollama chat endpoint or an unstarted LM Studio fallback server
- this keeps manual hosted validation honest without inventing a different hosted workflow contract from the one used during local task completion
- future CI expansion should stay conservative and provenance-friendly:
  - add checks only when they are already trusted locally
  - prefer promoting existing quality gates over creating parallel shadow gates

## Roadmap reassessment: semantic truthfulness before adapters
- the current roadmap spine is correct: `IntentIR` remains the canonical boundary, the four-layer IR split remains the right architecture, and multimodal evidence remains the right extraction strategy
- the next phase should now be framed explicitly as semantic-truthfulness hardening, because the dominant risk is no longer "can we lower to more targets?" but "how trustworthy is the recovered knowledge?"
- the near-term sequence should therefore be:
  - finish making the actor-relative graph the primary downstream signal model
  - make the clock-tick temporal model explicit in `SemanticIR` / `IntentIR`
  - make KG-guided multimodal rescans a first-class convergent workstream
  - add typed evidence arbitration for cross-modality disagreement
  - add gold fixtures, negative fixtures, and false-positive tracking for the KG
  - only then push harder Tier 3 relation extraction
- adapter expansion and adapter validation should be treated as horizon work until the semantic truthfulness program above is materially complete
- the key principle is that adapters should consume truth, not compensate for missing truth; when the pipeline struggles, the right fix is usually better evidence lifting, better temporal modeling, or better KG evaluation rather than smarter lowering

## Programming semantic intent without a black-box PDF-to-code pipeline
- the right goal is not to program a general reader of English
- the right goal is to program a compiler for protocol meaning
- this means the implementation should be organized around a typed domain model first, then around increasingly strong evidence-to-model mappings

### The five implementation pillars
- deterministic extraction where the domain is crisp
  - tables, widths, enum rows, clock/reset declarations, polarity cues, section kinds, figure kinds, and layout metadata should be extracted deterministically whenever possible
  - these surfaces should not be deferred to AI if the source structure already makes them mechanically recoverable
- a typed protocol-world model as the semantic target
  - the system should define the world it is trying to recover:
    - actors
    - signals
    - actor-signal roles
    - timing predicates
    - state transitions
    - dependencies
    - handshake events
    - other domain-specific protocol facts
  - prose, tables, figures, and captions are then evidence for those typed facts rather than the final representation themselves
- evidence aggregation instead of one-shot interpretation
  - no single modality should have to "win" by default
  - the pipeline should accumulate candidate facts across prose, tables, figures, captions, and layout
  - convergence, rescans, backannotation, and validation should decide what survives into canonical IR
- bounded AI, not full-pipeline AI
  - VLM/NLP/LLM use is valuable for hard spans, images, and local ambiguity
  - but the correct role is local hypothesis generation, not unrestricted start-to-finish interpretation
  - every AI-derived candidate should be forced back through:
    - schema checks
    - grounding checks
    - conflict and arbitration checks
    - convergence checks
    - validation
- explicit uncertainty as a first-class output
  - if the system cannot safely promote a candidate fact, it should preserve that uncertainty explicitly
  - alternatives, conflicts, and residual decisions are not failures of the architecture; they are part of the truthfulness contract
  - the system should avoid fabricating semantic certainty just to look complete

### Why this is programmable
- unrestricted natural-language semantics is open-ended, but protocol semantics is much narrower
- chip-design specs repeatedly talk about a constrained universe:
  - who drives what
  - who samples what
  - when a transfer completes
  - when a signal must remain stable
  - what values mean
  - what state comes next
- that narrower universe is exactly what the staged IR should model and validate

### Near-term implementation consequences
- keep adding typed semantic surfaces instead of broadening free-form text dependence
- prefer meaning-based inference over spelling-only heuristics when the KG and evidence can support it
- use KG-guided rescans to turn one recovered fact into the search anchor for the next pass
- keep validation focused on both correctness and honesty:
  - precision and false-positive control
  - contradiction surfacing
  - residual quality
  - rejection of weakly grounded AI hypotheses
- do not treat end-to-end AI confidence as a substitute for typed provenance and arbitration

## Clause-local semantic grounding for multi-signal text
- semantic-role inference should not require an entire prose sentence or figure caption to resolve to exactly one signal before it can contribute meaning
- real protocol text often explains multiple signals in one region:
  - `XVALID indicates that the request is pending and XREADY indicates that the subordinate can accept the transfer`
  - a single caption can similarly describe both sides of a handshake
- the weaker version of the pipeline dropped or underused that evidence because it tried to resolve the whole text blob to one signal target
- the stronger design is to carve multi-signal prose/caption regions into clause-local per-signal context windows:
  - keep the whole-text path for genuinely single-target descriptions
  - when multiple signals are mentioned, isolate local context around each mention using clause separators
  - infer semantic tags from that local descriptive window, not from the full multi-signal text blob
- this matches the project doctrine:
  - deterministic and inspectable
  - less lossy than the whole-text single-target rule
  - avoids smearing valid-like and ready-like evidence across every mentioned signal
  - recovers more meaning without asking runtime AI to solve the whole sentence end to end
- one more arbitration rule is important here: if the same text region contains both an alias and the explicit signal name for the same signal, the explicit signal mention should win
- otherwise alias learning can accidentally overcount evidence in exactly the places where the document is already being explicit
- the right interpretation is:
  - aliases are a rescue path when the canonical signal name is absent
  - aliases are not extra votes when the canonical signal name is already present
- a related visibility rule matters too: when the best current role meaning still depends only on alias-grounded evidence, that dependency should be explicit in the canonical IR
- alias-grounded meaning is still useful, but SOTA-quality truthfulness requires downstream consumers and validators to see when a role rests on alias mapping rather than direct signal mention or corroborating non-alias evidence
- the same rule extends one step further into time:
  - if a typed `HandshakeComplete` predicate exists only because valid-like / ready-like roles were recovered through alias mapping, that temporal convenience should be visible too
  - otherwise the temporal layer can look more grounded than it really is
- the right posture is not to throw away alias-grounded handshake semantics by default, but to keep them explicitly marked as weaker temporal grounding until stronger direct or cross-modality evidence arrives
- that visibility should not live only in the validator:
  - the canonical artifacts themselves should carry a residual/assumption trail when alias-grounded role meaning is still what enables typed handshake completion
  - otherwise crash recovery or offline artifact inspection can miss a real semantic caveat that the validator knew how to print

## KG benchmark harness as part of the truthfulness contract
- aggregate quality scores are useful, but they are not enough to steer a SOTA-grade KG program by themselves
- the project needs tracked fixtures that answer narrower questions directly:
  - did we recover the actor-relative ports we expected?
  - did we reject a false positive that only looked plausible by signal spelling?
  - did a known structural conflict remain visible through validation and the canonical layers?
  - did unresolved ambiguity stay explicit as a residual instead of being silently flattened away?
- `specforge kg-bench` is the first implementation of that principle:
  - it runs tracked fixtures through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`
  - it can assert canonical IR expectations and persisted validation findings
  - it keeps gold behavior and negative behavior in the same executable harness
- the first fixture pack is intentionally small but strategically chosen:
  - actor-port gold recovery
  - name-only semantic-noise rejection
  - multi-producer conflict surfacing
  - actor-boundary residual quality
- the harness also needs to be able to express richer staged conditions than plain markdown prose can capture on its own
- that is why tracked fixtures are now allowed to patch `SourceIR` and `EvidenceIR` surfaces directly:
  - inject a structured signal-description table
  - inject a typed signal constraint
  - then verify the downstream truthfulness behavior
- this is an elegant middle ground:
  - stronger than a prose-only fixture corpus
  - much cheaper and more controllable than requiring a full external PDF for every narrow semantic regression
  - still honest, because the patches target real IR surfaces rather than hidden test-only shortcuts
- the contested handshake-name fallback negative fixture is the first example of that approach:
  - stage-patched signal-description evidence and a guard constraint create a real semantic-role conflict
  - the benchmark then proves that typed `HandshakeComplete` recovery stays blocked instead of leaking through signal spelling
- the alias-dependent handshake-completion caveat fixture is the complementary example:
  - stage-patched alias learning plus a guard constraint create a real typed handshake recovery path
  - the benchmark then proves that the recovery remains usable while still preserving the alias-dependent residual and intent assumption trail
- together, these two fixtures protect both sides of the truthfulness contract:
  - reject unsafe heuristic promotion
  - preserve weaker-but-useful semantics explicitly instead of flattening their caveats away
- the next hardening step after those two fixtures is to benchmark the arbitration surface itself, not only its consequences:
  - a contested fixture should prove the signal still has multiple semantic candidates and non-decisive arbitration
  - a decisively grounded fixture should prove the signal has semantic arbitration too, but that it is decisively settled
- that is a better quality bar because it checks the canonical truth model directly instead of backing into arbitration quality from blocked fallbacks, residuals, or validator findings alone
- the next hardening step after that is to lock validation metrics and multimodal staged inputs directly:
  - fixtures should be able to patch visual assets, not only tables and signal constraints
  - fixtures should be able to assert persisted validation metrics, not only finding ids
  - a strong tracked gold case should prove true cross-modality semantic grounding from table plus visual evidence
- that matters because some truthfulness properties are expressed best in the validator's quantitative surface:
  - `with_cross_modality_semantic_grounding`
  - `with_visual_semantic_grounding`
  - related metric families that distinguish stronger grounding from weaker single-source evidence
- the next equally important negative case is multimodal disagreement:
  - table evidence can say valid-like while a visual caption says ready-like
  - the correct behavior is not to erase the visual evidence
  - the correct behavior is also not to overclaim resolved cross-modality reinforcement
- so the benchmark contract should explicitly lock this distinction:
  - `with_visual_semantic_grounding` can still be non-zero when conflicting visual evidence is present
  - `with_cross_modality_semantic_grounding` must stay zero when arbitration is still non-decisive and no resolved consensus exists
- the next useful tightening after caption-based multimodal fixtures is direct VLM-note provenance:
  - a tracked gold fixture should be able to prove the semantic hint came from `vlm_timing_diagram_extraction`
  - that requires evidence-stage validation assertions, not only semantic/intent checks, because downstream `with_visual_semantic_grounding` alone does not distinguish caption meaning from timing-note meaning
  - once that fixture exists, the benchmark surface covers all three current visual semantic paths:
    - caption-only grounding
    - caption-versus-table multimodal arbitration
    - direct VLM timing-note grounding
- the matching negative case matters just as much:
  - a VLM timing note that only says a handshake-shaped signal rises/falls at a tick must not become semantic-role evidence just because the signal spelling contains `VALID` or `READY`
  - the honest benchmark should prove two things at once:
    - `timing_diagram_extractions` stays non-zero, so we are not throwing away legitimate timing recovery
    - `signal_semantic_hints_from_vlm_timing_annotations` stays zero, so waveform motion is not overpromoted into protocol meaning
- there is one more visual-arbitration nuance worth locking directly:
  - a caption and a VLM timing note can disagree inside the same visual asset
  - the correct behavior is not to erase either source
  - the correct behavior is also not to treat two conflicting visual sub-sources as same-modality consensus
  - so the benchmark should prove:
    - `signal_semantic_hints_from_visual_captions = 1`
    - `signal_semantic_hints_from_vlm_timing_annotations = 1`
    - `with_visual_semantic_grounding = 1`
    - `with_multi_source_semantic_grounding = 0`
    - semantic arbitration remains non-decisive
- the next benchmark step after these seed multimodal cases should start the protocol-grade side of `R15e`:
  - add representative AMBA-style gold fixtures, not only isolated negatives
  - the first such gold fixture should prove that a `Source` column with values like `Requester` / `Subordinate` is enough to recover:
    - driver-side actor-signal KG edges
    - actor-relative output ports for the named driving side
    - table-grounded semantic request/accept roles
    - typed handshake completion when a guarded constraint references both signals
  - this is a good first bridge from synthetic truthfulness fixtures toward real APB/AHB/AXI-style benchmark coverage
- the next protocol-grade truthfulness step after that first AMBA-style `Source` path is the receiver side:
  - a `Destination` column with values like `Requester` / `Subordinate` should survive canonically as `Reads` relations
  - the corresponding actor-relative port surface should become `input`, not `output`
  - this should be benchmarked end-to-end in the tracked fixture suite, not only held in local unit tests or aggregate relation counts
- the next protocol-grade truthfulness step after that receiver-side AMBA path is AHB-style section context:
  - some AHB extraction quality still depends on section headings like `Manager signals` / `Subordinate signals`
  - that path should be benchmarked in the tracked fixture suite, not left as an implicit side effect of one unit test
  - the harness therefore needs two more truth-model-native capabilities:
    - patching `SourceIR.document_sections`
    - asserting per-signal canonical direction directly
- the next protocol-grade truthfulness step after that AHB section-context path was APB-specific role vocabulary, and it is now locked in the tracked fixture suite:
  - APB-family signal tables often use `Requester` / `Completer` rather than `Requester` / `Subordinate`
  - that path now has explicit benchmark coverage instead of being treated as already covered by the broader AMBA `Requester` / `Subordinate` gold fixture
  - the tracked fixture proves that APB-specific source-role vocabulary is strong enough to recover:
    - `(Requester, drives, PSEL)`
    - `(Completer, drives, PREADY)`
    - actor-relative output ports for both driven handshake-side signals
    - table-grounded request/accept semantics
    - typed handshake completion from one guarded APB-style stability constraint
- the next protocol-grade truthfulness step after that APB role-vocabulary path is AXI width-only channel structure:
  - AXI-family signal tables often carry `Name | Width | Description` but no direction column
  - the missing directionality then has to come from prose drive/sample relations, not the table itself
  - that path should be benchmarked explicitly because it is a real family-specific truthfulness risk:
    - the table should still recover signal inventory and widths
    - prose should recover `(Manager, drives, AWVALID)`, `(Subordinate, reads, AWVALID)`, `(Manager, drives, AWADDR)`, `(Subordinate, reads, AWADDR)`, `(Subordinate, drives, AWREADY)`, `(Manager, reads, AWREADY)`
    - the combined table-plus-prose evidence should still yield request/accept semantics and typed handshake completion
  - this is the right first AXI benchmark because it locks exactly the path that was historically brittle: no table direction column, but still enough structured evidence to recover truthful actor-relative ports
  - the benchmark also exposed a real downstream gap: `EvidenceIR` already synthesized width-only declarations like `Signal AWVALID is width 1.`, but `SemanticIR` previously rejected them because its explicit-signal parser required `input` or `output`
  - that parser is now widened so width-only synthesized declarations survive as canonical signal records with `direction_hint = None` until graph evidence resolves direction later
- the next AXI benchmark step after that first width-only direction slice is timing recovery on the same family of channels:
  - keep the width-only `Name | Width | Description` table shape
  - keep prose drive/sample relations for `Manager` / `Subordinate`
  - add a next-cycle timing assertion such as `AWREADY must be asserted on the next cycle`
  - keep a guarded stability rule like `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH`
  - the benchmark should then prove all of these at once:
    - canonical AXI signal inventory still survives
    - actor-relative ports still survive
    - one typed temporal rule carries `cycle_window = [1,1]`
    - both temporal rules are actor-grounded
    - handshake completion still appears from the guarded stability rule
- the next APB timing benchmark after the first `Requester` / `Completer` handshake slice should lock setup/access semantics instead of only steady-state completion:
  - keep the APB `Signal | Source | Width | Description` table shape so `Requester` / `Completer` actor roles still come from canonical APB vocabulary
  - add `PENABLE must be asserted on the next cycle when PSEL is HIGH`
  - keep guarded stability rules for both wait-state and completion contexts:
    - `PADDR must not change when PSEL is HIGH and PREADY is LOW`
    - `PADDR must not change when PSEL is HIGH and PREADY is HIGH`
  - the benchmark should then prove all of these together:
    - actor-relative APB ports still survive
    - one typed temporal rule carries `cycle_window = [1,1]`
    - all temporal rules are actor-grounded from requester/completer ownership
    - multi-predicate guards survive canonically instead of flattening
    - handshake completion still appears only in the completion-phase guard, not in the wait-state guard
- the next AHB timing benchmark after the section-heading direction slice should lock wait-state timing on the same family-specific evidence path:
  - keep `Manager signals` / `Subordinate signals` section-heading context
  - keep `Destination`-column signal tables so AHB still relies on its family-specific direction cues rather than a generic source column
  - add explicit actor relations in prose for `HADDR`, `HTRANS`, `HSEL`, and `HREADY` so temporal grounding can attach to real producers and consumers
  - add `HREADY must be asserted on the next cycle when HSEL is HIGH`
  - keep waited-transfer stability rules like:
    - `HTRANS must not change when HREADY is LOW and HSEL is HIGH`
    - `HADDR must not change when HREADY is LOW and HSEL is HIGH`
  - the benchmark should then prove all of these together:
    - AHB section-heading direction recovery still survives
    - one typed temporal rule carries `cycle_window = [1,1]`
    - all temporal rules are actor-grounded
    - multi-predicate wait-state guards survive canonically
    - no false handshake completion is inferred just because the timing is rich
- the next negative truthfulness step after that first AMBA-style gold path should protect against bogus actor attribution in the same family of tables:
  - `Clock` / `Reset` / direction-placeholder rows inside `Source` / `Driver` / `Destination` columns are metadata, not protocol actors
  - the KG should keep direction and system-contract recovery for those infrastructure signals without inventing actors named `Clock`, `Reset`, or `input`
  - `Destination` columns also need different semantics from `Source` columns:
    - `Source` / `Driver` rows imply `(actor, drives, signal)`
    - `Destination` rows imply `(actor, reads, signal)`
  - this is exactly the kind of false-positive control `R15e` should lock with a tracked negative fixture, not leave to comments or ad hoc tests
- the next table-truthfulness step after that is field-table misclassification:
  - a `Bits | Name | Description` register-field table can be mislabeled upstream as `signal_description`
  - if that happens, field names like `REQ` / `ACK` must not become fake top-level protocol signals or semantic roles
  - the correct fix is a table-level sanity gate that protects every table-driven top-level-signal path together, not one-off filters in just the semantic-hint extractor
  - that negative path is now important enough to stay locked in the tracked benchmark suite
- the next timing-truthfulness step after field-table rejection is spurious annotation rejection:
  - VLM timing-diagram output often includes low-value labels like `T0`, `Addr 1`, `Cycle 2`, lane markers, or other waveform annotations that are useful as figure markup but not meaningful timing semantics
  - the semantic lift must keep the underlying timing-diagram extraction visible in `EvidenceIR`
  - but it must not promote those label-only annotations into canonical `TimingConstraintRecord` or `TemporalRuleRecord`
  - the right shape is a narrow label/noise filter at the timing-lift boundary plus a tracked negative fixture that proves the evidence-stage extraction survives while semantic/intent timing stays at zero
- this is the right shape for `R15e`:
  - start with fixtures that protect truthfulness invariants
  - then grow toward APB/AHB/AXI protocol-grade gold suites and broader negative corpora
- the benchmark harness should remain graph-first and honesty-first:
  - benchmark canonical truth, not adapter output cosmetics
  - benchmark conflict surfacing and residual quality, not only successful extraction
  - benchmark bounded-hypothesis rejection, not only fact accumulation

## Cross-document learning without leaking facts across PDFs
- the current four-layer IR pipeline is intentionally document-local:
  - `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` for PDF `#1` should stay grounded in PDF `#1`
  - `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` for PDF `#2` should stay grounded in PDF `#2`
- that local grounding is a feature, not a weakness:
  - canonical truth should remain provenance-pure
  - earlier documents should not silently inject undocumented facts into later canonical artifacts
- but there is a valid next architectural step beyond that siloing:
  - add a separate cross-document learning plane so the extractor becomes stronger the more chip-spec PDFs it analyzes
  - the thing that should learn across documents is the extraction intelligence, not the canonical truth of the current document

### The two-plane architecture
- document plane:
  - `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`
  - contains only facts justified by the current PDF
  - remains the provenance-carrying canonical path
- learning plane:
  - a global typed memory of reusable extraction knowledge
  - potential names include `CorpusMemory`, `PriorGraph`, or `ExperienceIR`
  - stores reusable priors, reliability information, and false-positive knowledge
  - never becomes a back door that can directly author canonical facts without local evidence

### What the cross-document memory should learn
- recurring semantic-role language:
  - `can accept the transfer`
  - `request phase`
  - `acknowledge`
  - `response returned`
- recurring alias patterns
- recurring table shapes:
  - signal-description tables
  - encoding tables
  - field-meaning tables
  - timing tables
- recurring visual motifs:
  - handshake diagrams
  - burst timing
  - state bubbles
  - arbitration waveforms
- actor taxonomies:
  - requester / initiator / master / manager
  - subordinate / target / slave / peripheral
- protocol-semantic motifs:
  - ready / valid
  - request / acknowledge
  - grant
  - command / response
  - credit-based flow control
- temporal-language priors
- modality reliability priors
- negative knowledge:
  - common false positives
  - misleading captions
  - dangerous aliases
  - over-eager heuristics
- extractor reliability knowledge:
  - which patterns are strong
  - which are weak
  - which are family-specific
  - which are dangerous enough to require stronger corroboration

### What it must not learn
- it must not smuggle unsupported document facts from older PDFs into newer canonical outputs
- bad version:
  - `APB had signal X, so this new document probably means X too`
- good version:
  - `across many specifications, a phrase like "can accept the transfer" is strong evidence for a ready-like role`
  - `across many specifications, a 4-column table like Signal / Source / Width / Description is often a signal-description table`
  - `across many specifications, certain timing captions correlate strongly with handshake semantics`

### Runtime shape for prior-guided extraction
- analyze the new PDF through the normal staged pipeline
- retrieve relevant priors from the cross-document memory
- use those priors to:
  - prioritize rescans
  - propose bounded hypotheses
  - decide which extractor families are worth attempting first
  - phrase better bounded AI questions with stronger local context
- require local grounding in the current PDF before any candidate fact becomes canonical
- let validation and arbitration decide what survives into `IntentIR`
- feed only high-confidence, well-grounded, validated outcomes back into the learning plane

### Why this is the elegant version
- it improves the extractor without corrupting the truth model
- it lets the system become more expert about how chip specs express meaning
- it preserves the main doctrine of the project:
  - priors can guide extraction
  - only local evidence can justify canonical facts
- it should make the system progressively better at:
  - spotting meaningful tables faster
  - recognizing protocol roles from more varied prose
  - interpreting figures more reliably
  - rejecting weak heuristics earlier
  - converging in fewer passes
  - using AI in a more bounded and grounded way

### The update rule is the critical safety boundary
- the learning plane should only learn from promoted, well-grounded outcomes
- it should not learn directly from raw guesses, weak one-off hypotheses, or unvalidated AI output
- the right feedback source is validated/promoted knowledge, not transient extraction noise

### What actually grows over time
- the ability to learn is frozen in code:
  - `crates/specforge/src/commands/learn_priors.rs` defines how reusable priors are harvested
  - `crates/specforge/src/ir/prior_memory.rs` defines the typed memory schema and lookup behavior
  - `crates/specforge/src/ir/evidence.rs` and `crates/specforge/src/ir/semantic.rs` define when learned priors are allowed to influence extraction
- the thing that actually grows with more validated PDFs is the typed prior store, not the code and not any neural-network-style hidden weights
- today that growing memory is the local `CorpusMemory`, typically materialized at `generated/prior_memory/corpus_memory.json`
- as more validated documents are fed through `specforge learn-priors`, the prior store can accumulate:
  - more `actor_taxonomy_priors`
  - more `semantic_phrase_priors`
  - more `temporal_phrase_priors`
  - higher `support_count`
  - more `supporting_document_keys`
  - eventually more prior families such as table-shape, visual-motif, modality-reliability, and negative-knowledge priors
- this is explicit symbolic learning, not neural learning:
  - code defines what may be learned and how it may be used
  - the prior store records what has been learned so far
  - canonical per-document IR artifacts remain separate and provenance-pure
- if the code stays the same but `corpus_memory.json` grows, the extractor becomes stronger because its reusable prior memory becomes richer
- if the prior store is deleted, the extractor still knows how to learn, but it loses the accumulated experience
- this separation is a major architectural strength because the learned knowledge stays inspectable, diffable, and debuggable rather than being buried inside opaque weights

### Why the "how to learn" structure is critical
- yes, this part loosely echoes how humans learn, but the right target is not "mimic humans completely"
- the safer goal is:
  - borrow the parts of human learning that are structurally useful for extraction quality
  - reject the parts that would turn priors into hidden hallucination channels
- the useful human-like properties are:
  - accumulate experience across many documents
  - abstract patterns from repeated successful cases
  - keep confidence graded rather than binary
  - remember failures and false positives
  - use prior experience to guide attention
  - still require local evidence before believing a new fact
- that is why the learning-plane structure is so important:
  - if it is shaped badly, it can poison the pipeline by letting priors silently override document truth
  - if it is shaped well, it becomes a disciplined experience layer that improves extraction while preserving provenance purity

### Non-negotiable properties for the learning plane
- separation:
  - document truth and learned priors must remain distinct artifacts and distinct authority levels
- typed memory:
  - what is learned must stay explicit, inspectable, queryable, and diffable
- bounded influence:
  - priors may guide interpretation, ranking, and rescans, but must never directly author canonical facts
- validation-gated feedback:
  - only outcomes that survive arbitration and validation are allowed to feed back into learned memory
- negative learning:
  - the system must learn not only what works, but also what misleads, overfires, and creates false positives
- provenance:
  - every learned prior should retain what evidence family created it and from which documents it was harvested

### The learning plane is really an epistemology layer
- the real goal is not "more memory"; it is a better epistemology for extraction
- the learning plane should tell the system:
  - what kinds of prior knowledge are acceptable
  - how strongly that prior knowledge may influence extraction
  - when local evidence is still insufficient
  - when a pattern is known to be dangerous and should require stronger corroboration
- this is why the design should be treated as first-class architecture, not as a side feature:
  - extraction quality depends on the typed semantic world model
  - long-term scaling depends on the learning-plane design
  - therefore the structure of "how to learn" is one of the most important architectural problems in the project

### Implementation direction
- define a typed schema for cross-document prior memory
- keep that schema versioned separately from per-document IR artifacts
- scope priors by task, modality, and protocol family where appropriate
- make prior retrieval advisory, not authoritative
- benchmark whether prior memory helps on unseen PDFs without increasing cross-document fact leakage
- treat this as a first-class architectural expansion after the current graph/temporal/arbitration work, not as a shortcut around local truthfulness

### Corpus knowledge base layer
- there is a second cross-document layer the project should add after the typed prior plane matures: a persistent corpus knowledge base
- this should not be confused with `CorpusMemory`
  - `CorpusMemory` is the bounded machine-usable prior layer
  - the corpus knowledge base is the human+LLM synthesis layer
- the distinction matters:
  - the KG is best for typed facts, relations, provenance, residuals, and canonical truth
  - the prior store is best for machine-usable reusable extraction hints
  - the corpus knowledge base is best for durable synthesis across documents:
    - recurring protocol motifs
    - extraction failure archetypes
    - contradiction summaries
    - table and figure families
    - protocol-family notes
    - infrastructure-semantics notes
- the right three-plane model is:
  - per-document canonical IR: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`
  - cross-document typed priors: `CorpusMemory`
  - cross-document compiled synthesis: corpus knowledge base
- the safety boundary must stay explicit:
  - the corpus knowledge base must not directly author canonical IR truth
  - freeform synthesis must not replace typed KG facts
  - unvalidated output must not be promoted as trusted doctrine
  - any machine-usable promotion from the corpus knowledge base must flow back through typed schemas plus validation-gated promotion
- good uses of the corpus knowledge base:
  - explain recurring extraction wins and failures
  - capture negative knowledge that should remain visible even when it is not yet encoded as a typed prior
  - summarize protocol-family differences in how semantics are expressed
  - propose new prior candidates, benchmark fixtures, and KG-guided rescan strategies
  - give future LLM-assisted analysis a persistent compiled corpus artifact so it does not have to reconstruct the same higher-level synthesis from scratch every session
- candidate organization:
  - `protocols/`
  - `patterns/`
  - `tables/`
  - `visuals/`
  - `timing/`
  - `infra/`
  - `failures/`
- this is worth doing because the project should not stop at `document pipeline + priors`
- the stronger long-term architecture is:
  - document pipeline + priors + corpus knowledge base

### First landed `R15f` slice
- the first implementation now exists as a typed `CorpusMemory` store in `crates/specforge/src/ir/prior_memory.rs`
- `specforge learn-priors <intent_ir>...` is the first command that materializes that learning plane locally under `generated/prior_memory/corpus_memory.json`
- the first trust/update policy is intentionally narrow:
  - only validated `IntentIR` artifacts are eligible
  - artifacts with validation error findings are skipped
  - actor-taxonomy priors are harvested from decisive actor-grounded handshake-role evidence plus conservative self-identifying actor vocabulary
  - semantic-role priors are harvested only from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text
  - temporal-language priors are harvested from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`
  - the learned memory remains advisory-only and cannot directly author canonical document facts
- the first live AMBA run is already informative:
  - it now yields `16` actor-taxonomy priors from explicit actor vocabulary plus decisive actor-grounded handshake-role evidence
  - it yields `5` semantic phrase priors
  - it yields `4` semantic modality-reliability priors
  - it yields `266` temporal phrase priors
  - it yields `99` table-shape priors from AXI/APB/AHB/AXI-Stream `IntentIR` artifacts
- the next `R15f` step should not be “force more priors.” It should be:
  - broaden the store beyond the current actor-taxonomy / semantic / semantic-modality-reliability / temporal / table-shape families into visual-motif and negative-knowledge priors
  - then teach `EvidenceIR` / `SemanticIR` to consume those priors as bounded suggestions without weakening the local-grounding rule

### Sixth typed prior family expansion: visual motifs and negative knowledge
- `CorpusMemory` schema version `5` now has explicit `visual_motif_priors` and `negative_knowledge_priors`
- `visual_motif_priors` are harvested from validated artifacts through the linked `SourceIR.visual_assets` surface:
  - normalized caption phrases use the same signal/actor placeholder discipline as semantic and temporal phrase priors
  - diagram kind and asset kind stay typed, so the memory can distinguish timing diagrams, state-machine diagrams, figures, diagrams, and unknown assets
  - confidence is derived from whether a caption and typed diagram classification are both present
- `negative_knowledge_priors` are harvested from already-carried canonical caution surfaces:
  - signal-semantic conflicts
  - temporal value conflicts
  - interface-signal conflicts
  - signal-connectivity conflicts
  - residual decision packet classes
- this family expansion keeps the new priors inspectable and bounded:
  - visual-motif and negative-knowledge priors are memory, not a new authority channel
  - they may guide rescans, extractor selection, arbitration caution, role adjustment, and stronger-corroboration requirements
  - they must not directly author `SemanticIR` or `IntentIR` facts without current-document grounding
- the first unit regression now proves that a source-side timing-diagram caption can become a visual-motif prior and that a carried semantic-role conflict becomes a negative-knowledge signature without declaring either conflicting phrase false

### First visual-motif prior consumer
- `EvidenceIR` now has the first bounded runtime consumer for `visual_motif_priors`
- the consumer is intentionally narrow:
  - it only runs for a current `SourceIR.visual_assets` entry whose `diagram_kind` is still `Unknown`
  - it requires local caption text in the current document
  - it normalizes that caption with locally grounded signal and actor vocabulary before lookup
  - it only returns a diagram kind when the applicable prior scope has exactly one non-unknown match
- the output is an explicit `VisualObservationKind::Classification` observation created by `specforge_prior_memory`
- that prior-guided diagram kind may upgrade the visual evidence role, for example making a recovered timing diagram normative visual evidence
- the output deliberately stops at the evidence boundary:
  - it does not mutate `SourceIR`
  - it does not synthesize semantic-role hints
  - it does not create timing constraints, temporal rules, or canonical `IntentIR` facts
  - ambiguous prior memory stays silent instead of forcing a classification
- `validate evidence_ir` now reports `visual_classification_observations` plus visual-role metrics such as `visual_evidence_normative` and `visual_evidence_ambiguous`
- when a prior-classified visual becomes normative but still lacks VLM timing/state extraction observations, validation now emits `evidence_visual_motif_corroboration_guidance` and increments `visual_motif_corroboration_targets`
- `project-validation` consumes that finding as generic `rescan_guidance`, so visual-motif priors can route attention to targeted VLM/multimodal corroboration without becoming a truth-promotion channel
- visual-motif corroboration recommendations now carry an explicit replay sequence: local VLM `enrich_source_ir`, then `rebuild_evidence_ir`, then `validate_current_artifact`
- their provider hint is no longer hardcoded to Ollama: `project-validation --rescan-vlm-provider auto-local` prefers ready local Ollama, falls back to ready local LM Studio, and still allows explicit `ollama`, `lmstudio`, `skip`, and `--rescan-vlm-model <model>` overrides
- the tracked visual-motif fixture set now has the before/after shape:
  - `visual_motif_prior_guided_diagram_classification_gold` proves a staged prior creates a classification observation, normative role, corroboration target, and zero semantic hints
  - `visual_motif_prior_guided_diagram_classification_without_prior_negative` proves the same current caption stays ambiguous and unclassified when the prior is absent

### First negative-knowledge prior consumer
- `EvidenceIR` validation now has the first bounded runtime consumer for `negative_knowledge_priors`
- this first consumer is intentionally validation-only:
  - it loads the persisted `EvidenceIR.prior_memory_path`
  - it builds the same normalized signal-semantic conflict pattern used by `learn-priors`
  - it only matches when the current document already has a local `SignalSemanticConflictRecord`
  - it only reports an exact prior-memory match for the current conflict pattern
- the output is an info-level `evidence_negative_knowledge_prior_matches` validation finding plus a `negative_knowledge_prior_matches` metric
- the output deliberately stops at caution:
  - it does not mutate `EvidenceIR`
  - it does not suppress the current conflict finding
  - it does not change semantic arbitration
  - it does not create canonical `SemanticIR` or `IntentIR` facts
- the tracked `negative_knowledge_prior_guided_semantic_conflict_caution_gold` fixture locks that behavior, while the existing `visual_sources_semantic_conflict_negative` fixture now proves the same local conflict stays unmatched when no prior is staged

### Deep-layer negative-knowledge prior caution surfaces
- `SemanticIR` and `IntentIR` validation now broaden the same validation-only negative-knowledge consumer
- the deep-layer consumer recovers the linked prior memory through artifact provenance:
  - `SemanticIR.evidence_ir_path` -> `EvidenceIR.prior_memory_path`
  - `IntentIR.semantic_ir_path` -> `SemanticIR.evidence_ir_path` -> `EvidenceIR.prior_memory_path`
- the normalized pattern builders now live in `prior_memory.rs`, so `learn-priors` harvesting and validation consumption share one signature shape for:
  - signal-semantic conflicts
  - temporal value conflicts
  - interface-signal conflicts
  - signal-connectivity conflicts
  - residual decision packet classes
- `SemanticIR` validation now emits `semantic_negative_knowledge_prior_matches` plus the shared `negative_knowledge_prior_matches` metric when a carried current-document conflict/residual pattern exact-matches prior memory
- `IntentIR` validation now emits `intent_negative_knowledge_prior_matches` plus the same metric for its carried canonical surface
- the output deliberately remains caution-only:
  - it does not mutate `SemanticIR` or `IntentIR`
  - it does not suppress the carried conflict or residual finding
  - it does not change scoring, arbitration, or canonical promotion
  - it does not create facts from prior memory
- the tracked `negative_knowledge_prior_guided_temporal_conflict_caution_gold` fixture locks repeated temporal contradiction caution behavior, and `negative_knowledge_prior_guided_residual_caution_gold` locks repeated residual packet caution behavior
- the tracked `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` and `negative_knowledge_prior_guided_interface_conflict_caution_gold` fixtures now lock the remaining carried conflict families too: signal-connectivity multiple-producer caution plus interface direction/width mismatch caution
- those fixtures explicitly assert that the local conflicts remain present while prior memory only adds caution, rescan, and corroboration guidance

### Negative-knowledge now emits rescan/corroboration guidance
- validation now has the first bounded "beyond reporting" hook for negative-knowledge priors
- exact current-document prior matches still produce the existing `negative_knowledge_prior_matches` caution metric, but they also now emit:
  - `negative_knowledge_rescan_recommendations`
  - `negative_knowledge_corroboration_requirements`
  - stage-specific `*_negative_knowledge_rescan_guidance` findings
- the guidance is intentionally routing metadata, not an authority channel:
  - it gives downstream rescan/extractor-selection loops a deterministic related-id list
  - it says the current surface should be rechecked with stronger local corroboration before any canonical promotion
  - it still does not mutate artifacts, change score, rewrite arbitration, remove conflicts, remove residuals, or create facts from prior memory
- this is the first safe bridge from "the KG remembers a known failure shape" to "the next pass can choose where to spend extraction effort"
- the remaining follow-up is to make an actual rescan/extractor-selection consumer read these validation findings instead of only surfacing them in validation output

### Project validation now consumes negative-knowledge rescan guidance
- `specforge project-validation` now reads stage-specific `*_negative_knowledge_rescan_guidance` findings from persisted validation reports
- it materializes the first deterministic downstream consumer for that guidance:
  - a local generated `generated/validation/rescan_plan.json` file
  - a `Targeted Rescan Recommendations` section in `VALIDATION_SNAPSHOT.md`
  - a `Targeted rescan queue` block inside the managed live-status validation projection
- the rescan plan is an extractor-selection target list, not a canonical correction:
  - it carries document key, stage, artifact path, finding id, related current-surface ids, extractor lane, corroboration policy, and recommended action
  - it routes effort to known dangerous conflict/residual shapes
  - it still does not mutate IR, change score, remove findings, decide arbitration, or promote facts from prior memory
- `converge --rescan-plan <plan>` now makes those targets available to the fixed-point loop after stability; the remaining follow-up is richer arbitration over changed outcomes, not automatic truth promotion

### Project validation rescan plan is now replay-oriented
- `generated/validation/rescan_plan.json` is now schema version 2
- each recommendation now carries:
  - typed `replay_inputs` such as `source_document`, `source_ir`, `evidence_ir`, or `semantic_ir`
  - structured `recommended_commands` with executable, args, working directory, display string, and command intent
  - an explicit `automation_status: planned_not_executed`
- this keeps the plan useful for explicit executors without requiring them to parse prose or shell strings
- the command still does not run the rescans itself; it only writes replayable metadata and projects the queue into the continuity docs

### Rescan plan now has a bounded executor surface
- `specforge rescan-plan` now reads schema-v2 `generated/validation/rescan_plan.json`
- without `--execute`, it dry-runs the pending `planned_not_executed` recommendations and prints their command hints
- with `--execute`, it dispatches only whitelisted stage commands in-process:
  - `ingest`
  - `enrich` for local Ollama / LM Studio / skip-only replay, used by visual-motif corroboration hints
  - `evidence`
  - `semantic`
  - `intent`
  - `validate`
- the executor refuses non-`cargo` executables, non-repository working directories, malformed cargo prefixes, unsupported command intents, and OpenAI enrichment replay hints
- execution validates the target artifact before and after the whitelisted command hints
- successful execution marks the local plan recommendation as:
  - `executed_validated_no_change`
  - `executed_validated_changed`
- this is still not a truth-promotion channel:
  - it can rebuild and validate stages
  - a changed validation surface is not automatically classified as improved
  - it cannot let prior memory decide canonical facts
  - the remaining follow-up is richer changed-outcome arbitration before treating a rebuilt artifact as improved

### Converge can now consume rescan plans after stability
- `specforge converge <source> --rescan-plan <plan>` now runs the same schema-v2 rescan-plan consumer after the persisted pipeline snapshot stabilizes
- the convergence hook filters multi-document plans to the current source document key; standalone `rescan-plan` also exposes `--document-key <key>` for the same scoped inspection/execution behavior
- the hook is opt-in:
  - without `--rescan-plan`, `converge` behaves as before
  - with `--rescan-plan`, it dry-runs the queue after stability
  - with `--rescan-plan` plus `--execute-rescan-plan`, it executes only the same whitelisted in-process rebuild/validate hints accepted by `rescan-plan --execute`
- `--execute-rescan-plan` without `--rescan-plan <plan>` is rejected so execution cannot be implied by defaults
- the convergence result remains the stable pre-rescan snapshot; any post-rescan artifact change is reported as `snapshot_changed` and summarized through an arbitration status
- `changed_requires_validation_review` means the local plan or artifact surface changed and must be inspected by validation/evidence policy before any improvement claim or canonical promotion

### Rescan execution summaries now persist validation-backed arbitration hints
- executed schema-v2 rescan recommendations can now carry an optional `execution_summary`
- the summary records:
  - before/after validation fingerprints, scores, grades, finding counts, and finding ids
  - score delta, finding-count delta, added findings, removed findings, and whether the artifact fingerprint changed
  - a conservative verdict: `validated_no_change`, `possible_improvement_review_required`, `regression_review_required`, or `neutral_change_review_required`
- `converge --rescan-plan <plan>` now rolls those verdicts into review-required counters so the summary distinguishes possible improvement from regression or neutral artifact drift
- this is still not promotion: even favorable validation deltas need current-document evidence review before they can become canonical truth
- the summary now makes that policy machine-readable too:
  - `promotion_status: not_promoted_no_change` for unchanged validation snapshots
  - `promotion_status: not_promoted_review_required` for possible-improvement, regression, and neutral artifact-drift verdicts
  - `promotion_blockers` such as `canonical_ir_not_mutated_by_rescan_plan`, `validation_delta_is_not_truth_promotion`, and `current_document_evidence_review_required`
- the summary now also carries a structured `promotion_review` record:
  - unchanged validation snapshots are `not_reviewable_no_change`
  - changed validation snapshots are `human_review_required`
  - changed snapshots require an approval record plus explicit decisions for current-document evidence support, validation-delta review, canonical mutation scope approval, and prior-memory non-authority
  - `canonical_mutation_allowed` remains `false` because the executor still never mutates canonical IR from rescan outcomes
- this keeps old and new rescan consumers from quietly interpreting a better score or fewer findings as a canonical truth mutation

### Project validation now projects rescan execution summaries
- `project-validation` now reads the existing local schema-v2 rescan plan before refreshing it and preserves matching `automation_status` / `execution_summary` entries
- matching uses document key, stage, artifact path, finding id, extractor lane, and related ids so stale execution state does not attach to unrelated fresh recommendations
- `VALIDATION_SNAPSHOT.md` now shows rescan execution-summary counts plus per-recommendation verdict, promotion gate, promotion review, automation status, validation delta, and added/removed finding ids when execution summaries exist
- the managed live-status validation block now carries the same review-required count summary and inline verdict/promotion/review/delta detail for the targeted rescan queue
- this makes changed-outcome review possible from tracked docs without requiring a human or future agent to open raw generated JSON first

### Rescan approval artifacts stay local until canonical mutation exists
- `promotion_review` is a review-requirement descriptor, not an approval artifact
- when `promotion_review.approval_record_required` is true, it means an explicit future approval record would be needed before canonical mutation, not that the current `execution_summary` has already approved anything
- while `rescan-plan` and `converge --rescan-plan <plan>` cannot mutate canonical IR, any future approval artifact should stay local/generated by default, alongside `generated/validation/rescan_plan.json`
- tracked approval evidence should only be introduced with an explicit canonical mutation workflow and schema
- that future schema needs to record current-document evidence support, validation-delta direction, exact canonical mutation scope, prior-memory non-authority, reviewer intent, artifact fingerprints, and replayable provenance
- this keeps the current system honest: changed validation output can request review, but it cannot silently become durable truth or a tracked approval trail

### Fifth landed bounded prior family and semantic-stage arbitration slice
- the fifth bounded learning family is now real in `crates/specforge/src/ir/prior_memory.rs` and `crates/specforge/src/commands/learn_priors.rs`
- `specforge learn-priors` now harvests semantic modality-reliability priors from decisive, non-alias-dependent semantic consensus plus the supporting source kinds that carried that consensus
- the bounded behavior is intentionally strict:
  - only locally grounded semantic candidates can enter arbitration in the first place
  - the learned modality prior can only advisory-adjust the arbitration margin between already-present local candidates
  - the prior cannot create a semantic role when the current PDF has no local candidate for that role
  - the original semantic conflict record remains preserved even when prior-guided arbitration becomes decisive
- the first live AMBA prior-memory run shows this family is architecturally landed but still honestly sparse on real artifacts:
  - `16` actor-taxonomy priors
  - `5` semantic phrase priors
  - `4` semantic modality-reliability priors
  - `266` temporal phrase priors
  - `99` table-shape priors on the current four-document AMBA corpus
- the first benchmark pair for this family now locks the before/after contract:
  - without prior memory, a locally conflicted role like `XCTRL` stays honestly contested
  - with a matching modality-reliability prior, the same local evidence can become decisively resolved through `SemanticIR` and `IntentIR`
- this matters because it is the first learning slice that improves semantic arbitration itself rather than only local phrase/table interpretation, while still keeping the arbitration process inspectable and provenance-safe

### Fourth landed bounded prior-consumption slice
- the fourth consumer is now real in `crates/specforge/src/ir/evidence.rs`
- `EvidenceIR` can now advisory-recover a local table kind from learned table-shape prior memory, but only when the current table is still `unknown`
- the bounded behavior is intentionally strict:
  - explicit local `SourceIR.table_kind` values always win
  - prior-guided table-shape recovery widens local interpretation, but it does not rewrite `SourceIR`
  - only the current table’s own normalized header signature is consulted, so the prior cannot invent a table that is not already present in the current document
- the first live AMBA prior-memory run now shows this family is materially real, not speculative:
  - `16` actor-taxonomy priors
  - `5` semantic phrase priors
  - `4` semantic modality-reliability priors
  - `266` temporal phrase priors
  - `99` table-shape priors on the current four-document AMBA corpus
- actor-taxonomy learning now also has an explicit epistemic hygiene guard:
  - payload/event nouns like `control information`, `data`, and `transfer` are rejected at both harvest time and lookup time
  - that matters because a repaired extraction bug should be removable from `CorpusMemory`, not allowed to survive as stale actor vocabulary that quietly biases later runs
- the first benchmark pair for this family now locks the before/after contract:
  - without prior memory, a locally `unknown` `Name | Direction | Width` table stays inert
  - with a matching table-shape prior, the same local table recovers signal-description semantics through `EvidenceIR`, `SemanticIR`, and `IntentIR`
- the benchmark surface now covers a second table-kind shape too:
  - without prior memory, a locally `unknown` `Parameter | Min | Max | Unit` table stays inert
  - with a matching table-shape prior, the same local table recovers `timing_constraints` through `EvidenceIR`, `SemanticIR`, and `IntentIR`
- this matters because it is the first cross-document learning slice that improves table interpretation directly while still preserving the doctrine that priors guide extraction and never silently override explicit local truth

### First landed bounded prior-consumption slice
- the first consumer is now real in `crates/specforge/src/ir/evidence.rs`
- `specforge evidence <source_ir>` and `specforge converge <source>` now consult the local `CorpusMemory` by default through `--prior-memory generated/prior_memory/corpus_memory.json`
- the first bounded use is intentionally narrow:
  - actor-taxonomy priors can widen how `EvidenceIR` interprets explicit local actor labels in section headings and `Source` / `Destination` columns
  - width-only section-guided signal tables can now also emit structural `ActorSignalRelation::Drives` edges when the heading carries a locally grounded actor term plus a matching learned taxonomy prior
  - the prior can help classify `Producer` / `Consumer` as requester-like / completer-like when those terms are already present in the current PDF
  - the prior cannot create a signal, actor, direction, or relation that is not explicitly grounded in the current document
- this is the right first consumer because it replaces a brittle hardcoded vocabulary list with reusable typed memory without violating the “priors guide extraction, they do not author truth” rule
- the next `R15f` consumer steps should still stay bounded:
  - semantic-role priors as suggestion/ranking only
  - temporal-language priors as phrase-prioritized parsing/rescan hints only
  - table-shape / visual-motif / negative-knowledge priors after that

### Second landed bounded prior-consumption slice
- the second consumer is also now real in `crates/specforge/src/ir/evidence.rs`
- `EvidenceIR` can now use semantic phrase priors to recover local signal-role hints from grounded phrases that are not part of the current hardcoded heuristic list
- the bounded behavior is still strict:
  - built-in name-noise protections remain in place
  - learned semantic priors only fire when the current PDF contains a local phrase whose normalized shape matches the learned prior
  - the prior still cannot author a semantic role without that local phrase being present in the current document
- `EvidenceIR` now persists the `prior_memory_path` it consulted so later refreshes during NLP loopback keep the same advisory prior guidance instead of silently dropping it
- a direct regression now proves the new bounded behavior on a non-hardcoded phrase:
  - without prior memory, `XACK can receive the transfer` yields no semantic hint
  - with prior memory, the same local phrase yields a ready-like hint
  - after writing/reloading `EvidenceIR`, `refresh_signal_semantic_hints()` preserves that prior-guided hint because the consulted prior-memory path is now part of the local artifact state

### Third landed bounded prior-consumption slice
- the third consumer is now real in `crates/specforge/src/ir/semantic.rs`
- `SemanticIR` can now use temporal phrase priors to advisory-recover a `cycle_window` from local timing text whose phrase shape matches learned prior memory when the built-in parser still cannot recover the timing window directly
- the first bounded prior families are now benchmarked as a set, not just implemented in isolation:
  - actor-taxonomy prior benchmark: width-only `Issuer signals` / `Acceptor signals` headings recover structural KG edges, actor ports, and canonical directions only when matching actor-taxonomy priors are present
  - semantic prior benchmark: `XACK can receive the transfer` recovers a role only when a matching semantic prior is present
  - temporal prior benchmark: `PREADY must be asserted one beat later` recovers a `cycle_window` only when a matching temporal prior is present
- the bounded semantic-prior path is now also benchmarked on a second modality: a visual-caption phrase like `XACK can sink the transfer` now proves the same before/after contract for `SignalSemanticHintSourceKind::VisualCaption`
- that benchmark shape matters because it proves the same doctrine across all three families: prior memory widens interpretation of local evidence, but it does not author facts when the matching local evidence is absent or when the prior is not staged
- the bounded behavior remains strict:
  - the current PDF still has to contain the local timing sentence or timing-note text
  - the prior does not create a temporal rule by itself; it only helps interpret the local phrase once a local rule already exists
  - built-in direct cycle-window parsing still runs first, so learned priors are fallback guidance rather than replacement logic
- a direct regression now proves the new bounded behavior on an unseen phrase shape:
  - without prior memory, `PREADY must be asserted one beat later` yields no cycle window from the built-in parser
  - with prior memory, the same local phrase yields `CycleWindowRecord { min_cycles: Some(1), max_cycles: Some(1) }`
  - the recovered timing still stays document-local and provenance-pure because the rule only exists when the local timing text is present
- the KG benchmark harness now also locks this behavior at the cross-document-learning boundary:
  - fixtures can stage a local `CorpusMemory`
  - the unseen local phrase stays unbounded without prior memory
  - the same local phrase gains only the bounded `cycle_window` when the matching temporal prior is present
- the same benchmark doctrine now also covers semantic-role priors:
  - fixtures can stage a local semantic prior for `XACK can receive the transfer`
  - the unseen local phrase stays semantically unresolved without prior memory
  - the same local phrase gains only a locally grounded ready-like recovery when the matching semantic prior is present

## Current repository observations
- the repository now contains a renamed `specforge` crate and CLI
- the active Rust codebase no longer treats `spec2fsm` as the primary identity
- the canonical product boundary is now described consistently as `IntentIR`
- the first real implemented stages are `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
- the repository now also includes `subs/fsmgen` as a pinned git submodule for local `.fsm` reference work during adapter implementation
- `subs/fsmgen` is now explicitly treated as contextual and read-only from `specforge`
- `SourceIR` now includes a real Docling-backed structured PDF materialization path with promoted markdown, page artifacts, visual assets, metadata JSON, and backend raw JSON
- `EvidenceIR` now builds multimodal evidence records instead of remaining text-only scaffolding
- `SemanticIR` now builds a first backend-neutral semantic layer instead of remaining scaffolding only
- `IntentIR` now builds a first canonical backend-neutral intent layer instead of remaining scaffolding only
- the first `.fsm` adapter slices now build typed adapter artifacts that can lower honest standalone DT, structured FSM, and explicit top-root composition cases instead of leaving adapters as planning-only scaffolding

## Structured PDF normalization implementation
- execute-mode PDF ingest is now orchestrated from `crates/specforge/src/ir/source.rs`
- the backend runner lives in `crates/specforge/src/ir/source/docling_backend.rs`
- Rust remains the owner of canonical `SourceIR`, manifest paths, and final `source_ir.json` persistence
- an embedded Python helper drives Docling to materialize:
  - promoted markdown with referenced picture assets
  - page images and per-page metadata sidecars
  - cropped picture and table assets
  - metadata JSON and backend raw JSON
- runtime discovery prefers `python3` or `python` with `docling` importable, and can be overridden with `SPECFORGE_DOCLING_PYTHON`
- tests can override the backend command with `SPECFORGE_DOCLING_HELPER` so `cargo test` exercises the full SourceIR materialization path without depending on a live Docling install
- visual assets now carry a `source_ref` pointing back into backend-native structured output so later stages can ground evidence against the raw parser representation

## First executable EvidenceIR stage
- execute-mode `EvidenceIR` construction is now orchestrated from `crates/specforge/src/commands/evidence.rs`
- the core builder lives in `crates/specforge/src/ir/evidence.rs`
- `EvidenceIR::build` now:
  - loads persisted `SourceIR` JSON from disk
  - requires `normalization_status: ready`
  - reads the promoted markdown path from `SourceIR`
  - builds section anchors from markdown headings
  - builds block-level evidence spans with line provenance
  - projects `SourceIR` visual assets into typed visual evidence items
  - links caption spans to visual assets with `describes`
  - links textual `Figure N` / `Fig. N` / `Table N` references with `cites`
  - emits heuristic extracted-statement classes for source facts, derived rules, local design decisions, and explicit abstractions
- the current first-pass implementation is intentionally deterministic and inspectable rather than LLM-driven
- deeper OCR, chart extraction, and richer visual interpretation remain future enrichment work for later EvidenceIR/SemanticIR slices

## First executable SemanticIR stage
- execute-mode `SemanticIR` construction is now orchestrated from `crates/specforge/src/commands/semantic.rs`
- the core builder lives in `crates/specforge/src/ir/semantic.rs`
- `SemanticIR::build` now:
  - loads persisted `EvidenceIR` JSON from disk
  - derives artifact layout under `generated/semantic_ir/<document_key>/semantic_ir.json`
  - filters statements from boilerplate sections (legal/licence/admin headings) before semantic extraction so legal front-matter in chip specs does not contaminate actor, interface, or invariant discovery
  - discovers actors from explicit role terms and falls back to interface-derived channel actors when the evidence names signals but not endpoints
  - discovers interfaces through three complementary paths:
    - explicit `Signal X is input/output width N.` declarations (High confidence)
    - markdown signal-description table rows when the section heading identifies a known direction context such as "Manager signals" or "Subordinate signals" (Medium confidence)
    - heuristic co-mention grouping for remaining UPPERCASE tokens, with expanded stop-word filtering to exclude legal terms, protocol family names, and common English all-caps words, and with large-set noise filtering requiring ≥2 supporting statements for groups >8 signals
  - preserves backend-neutral system contract and init-assignment records from explicit `Clock ...`, `Reset ...`, and `Init ...` statements when the evidence is explicit enough, including reset kind, polarity, assertion/release timing, and target semantics
  - preserves backend-neutral guarded/action control fragments from explicit `Block ...` statements when the evidence is explicit enough
  - preserves explicit module and top-composition facts from explicit `Module ...` and `Top ...` statements when the evidence is explicit enough
  - derives phases from section structure and sequencing language
  - extracts invariants, contracts, gates, and abstractions from inspectable heuristics over evidence statements
  - emits decomposition candidates from section/topic clustering
  - emits explicit residual decisions when actor boundaries, overlapping interfaces, or ambiguous visual evidence remain unresolved
- the current first-pass implementation remains deterministic and conservative; it is meant to expose candidate semantics and unresolved ambiguity, not to invent a final canonical intent model
- validated against the AMBA AHB Protocol Specification PDF: signal candidate count reduced 250 → 57, interface count 172 → 94, 16 signals carry explicit direction+width from signal-table parsing

## First executable IntentIR stage
- execute-mode `IntentIR` construction is now orchestrated from `crates/specforge/src/commands/intent.rs`
- the core builder lives in `crates/specforge/src/ir/intent.rs`
- `IntentIR::build` now:
  - loads persisted `SemanticIR` JSON from disk
  - derives artifact layout under `generated/intent_ir/<document_key>/intent_ir.json`
  - canonicalizes actor responsibilities from semantic actors, contracts, and phase overlap
  - carries forward canonical interface inventory from typed semantic interfaces
  - carries forward canonical backend-neutral system contract and init assignments from typed semantic records, including first-class reset polarity/assertion/release/target semantics
  - carries forward backend-neutral guarded/action control fragments from typed semantic control blocks
  - carries forward explicit module and top-composition facts without reinterpreting scope inside the adapter
  - canonicalizes behaviors from phases, contracts, and gate-like sequencing rules
  - canonicalizes constraints from invariants, assertions, and interface-coupled rules
  - derives assumptions from abstractions and conservative backend-neutral heuristics
  - preserves semantic residual decisions and adds canonicalization-specific residuals only when the intent model would otherwise become speculative
- the current first-pass implementation remains deterministic and conservative; it is meant to produce a stable canonical intent surface before adapter work, not to overfit one backend target

## First executable adapter stage
- execute-mode adapter construction is now orchestrated from `crates/specforge/src/commands/adapt.rs`
- the core builder lives in `crates/specforge/src/ir/adapters.rs`
- `AdapterArtifact::build` now:
  - loads persisted `IntentIR` JSON from disk
  - derives typed adapter artifacts under `generated/adapters/fsm/<document_key>/adapter.json`
  - chooses `?dt:name` for explicit standalone DT cases, `?fsm:name` when explicit regular-state and transition records are present, and `?top:name` when explicit top/module composition facts are present
  - consumes canonical interface inventory, backend-neutral system/init records, backend-neutral guarded/action fragments, explicit regular-state/transition records, and explicit module/top composition facts from `IntentIR`
  - emits real standalone `?dt:name` text when every referenced signal has explicit width/direction, every control block is fully typed, and any sequential standalone DT case also has explicit system/init facts
  - emits real structured `?fsm:name` text when the canonical state graph, transition targets, and state-body control are explicit enough to avoid semantic invention
  - emits real explicit `?top:name` source documents when explicit top ports, child modules, and width-compatible links are complete enough to avoid semantic invention
  - keeps reset polarity honest in emitted `.fsm` text by requiring it to remain recoverable from `sreset` / `asreset` plus the reset signal name because the current target syntax does not carry a separate polarity token
  - preserves upstream residual decisions and emits adapter-side residual decisions only for unresolved signal inventory, system/init surface, state graph, composition topology, and broader root-kind expansion
  - keeps compatibility-level `?mod:name` / `?module:name` spellings outside the current canonical root-kind model because the current canonical surface does not yet carry an honest direct-module distinction
- the current renderable slices are still intentionally narrow rather than speculative; they now cover explicit standalone combinational and sequential DT cases, canonical symbol-definition/reset-role lowering, selector/test-node branches, compound-update shorthand, explicit structured FSM-root cases, and the first explicit top-root composition slice while keeping compatibility-level direct-module spellings outside the canonical root-kind model and still deferring unsupported selector predicate shapes

## Widened `.fsm` semantic slice
- `SemanticIR` and `IntentIR` now preserve canonical symbol-definition sections and structured control blocks instead of relying only on legacy decision-tree fragments
- the widened canonical surface now carries:
  - `+constants`, `+define`, `+params`, and `+enums` style symbol definitions
  - structured control expressions and action records
  - dedicated synchronous-reset and asynchronous-reset control-block roles
  - state-body control that can keep branch-local actions together instead of forcing every action through older fragment-only shapes
- the `.fsm` adapter now lowers from canonical `symbol_definitions` and `control_blocks` first and only falls back to legacy fragment candidates when the widened canonical surface is absent
- the `.fsm` adapter now lowers honest selector/test-node branches and compound-update shorthand when the canonical selector/predicate/update shapes map directly to explicit `.fsm` syntax, still blocks unsupported selector predicates or target/update shapes explicitly instead of inventing approximations, and now keeps the adapter root-kind surface limited to `dt` / `fsm` / `top` until a real backend-neutral direct-module distinction exists
- the canonical system contract now preserves reset kind, reset polarity, assertion timing, release timing, and reset-target semantics explicitly rather than leaving hardware reset behavior implicit
- the current reset normalization maps:
  - synchronous reset to synchronous assertion, synchronous release, and data-input-path semantics
  - asynchronous reset to asynchronous assertion, synchronous release, and dedicated-reset-pin semantics
- explicit reset phrasing accepts both `Reset rst_n is asynchronous active low.` and `Reset rst is synchronous active high.`
- when explicit polarity wording is omitted, the current parser infers active-low from `_n` / `_b` reset naming and otherwise falls back to active-high with lower automation confidence

### Clock/reset are infrastructure semantics, not ordinary protocol edges
- clocks and resets should not be modeled long-term as ordinary protocol payload/control signals
- they are special system infrastructure with stricter design rules and should be preserved as such in the canonical model
- clock generation and distribution require dedicated handling because glitch-free behavior, tree quality, and timing discipline matter more than ordinary signal connectivity
- reset handling also needs a dedicated semantic model:
  - assertion can be asynchronous to the destination clock
  - release should be synchronous to the destination clock when the spec or design discipline indicates that behavior
  - reset trees should avoid arbitrary glue logic and should be modeled conservatively
- this means graph carry-through for signals like `ACLK` / `ARESETN` is useful, but only as an intermediate structural aid
- the stronger target model is:
  - infrastructure-class signals distinct from ordinary protocol signals
  - conservative producer attribution for clocks/resets
  - first-class sourcing/distribution semantics
  - explicit reset discipline rather than flattening resets into generic control edges
- implementation consequence:
  - do not let clock/reset graph edges silently imply "ordinary producer/consumer semantics"
  - preserve them through `SystemContractRecord` and infrastructure-specific canonical records instead
  - prefer truthfulness over apparent graph completeness when clock/reset ownership is not explicit in the current PDF
- current implementation:
  - `SignalConnectivityRecord.connectivity_class` keeps clocks/resets separate from ordinary `protocol` connectivity
  - `InfrastructureSignalRecord` now makes clock/reset source status and recovered distribution status first-class in `SemanticIR` and `IntentIR`
  - unresolved sourcing is modeled as `unresolved_source`, not as a fabricated `Clock`, `External`, or `input` producer actor
  - explicit local phrases such as `clock generator drives ACLK` or `PLL generates ACLK` can recover infrastructure source status, but this does not relax the ordinary actor filters for generic `Clock`, `Reset`, or `External` labels
  - implicit clock/reset read-port fanout applies only to actors with non-infrastructure protocol relations, so a recovered clock/reset source is not automatically treated as consuming its own signal
  - explicit local distribution/fanout phrases such as `ACLK is distributed to the Requester and Completer` or `reset synchronizer feeds ARESETN to the Requester` can recover infrastructure distribution targets without creating ordinary protocol actor ports
  - `InfrastructureSignalRecord.infrastructure_topology` now preserves only explicit current-document topology hints for `clock_gated_branch`, `reset_synchronizer_stages`, and `reset_tree_targets`
  - examples include `The ACLK clock gate CGATE0 feeds the Requester branch`, `The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester`, and `The ARESETN reset tree targets the Requester registers and Completer registers`
  - vague implementation advice such as "may use a synchronizer" must not create topology records, and the topology surface still is not a full physical clock-tree/reset-tree proof

## Knowledge graph extraction — design decisions (2026-04-03)

### Why direction_hint is architecturally incomplete
The current `InterfaceSignalRecord.direction_hint: Option<InterfaceSignalDirection>` is relative to an unnamed implicit actor. "PREADY is input" is meaningless without knowing input-to-whom. "PREADY is input_of[Manager]" is meaningful. This must eventually become an actor-relative model.

### Tables vs prose: complementary roles, not redundant
Tables provide signal NAMES reliably and WIDTH sometimes. Tables rarely provide direction in a machine-readable form across all specs. AMBA 5 specs (APB, AXI5) use "Requester"/"Completer" instead of "output"/"input" in their Source columns. AXI5 signal tables have no direction column at all. The prose always has the directionality information encoded in verb phrases.

### Actor identity is behavioral, not lexical
Do not anchor actor detection to vocabulary. "Manager", "master", "initiator", "Requester" all mean the same thing: an entity that initiates transactions. "Subordinate", "slave", "completer", "Responder" all mean: an entity that responds. What matters is what the entity DOES in sentences, not what it is called.

### Verb phrases are relations
Every sentence that connects an actor to a signal encodes a typed relation:
- Drives: drives, asserts, activates, outputs, returns, generates, provides (and passives: is driven by, is asserted by, etc.)
- Reads: reads, samples, monitors, accepts, receives (and passives: is read by, is sampled by, etc.)
- Transfer: A transfers X to B → A drives X, B reads X
These triples (actor, relation, signal) form the structural knowledge graph of the spec.

### The two-layer model of a chip spec
- Layer 1 (structural): who the actors are, what signals connect them, direction per actor — this is the block diagram
- Layer 2 (behavioral): how signals change over clock cycles, state machines, timing — this is the waveforms/FSM
All digital protocols are synchronous. The clock is the universal time reference. All timing is in clock cycles.

### Validated pipeline results (2026-04-03)
- AHB (IHI0033_C): 86/100 GOOD — works because section headings happen to say "Manager signals"
- APB (IHI0024_E): 35/100 NEEDS IMPROVEMENT — "Requester"/"Completer" in Source column not recognized → 0 declared signals
- AXI (IHI0022_L): 85/100 (misleading) — 1 declared signal out of ~100+; score inflated by 1/1=100%
Reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md` for full analysis and implementation plan.


## Convergent EvidenceIR enrichment without hardcoded value lists (2026-04-03)

### Why the earlier one-shot build order was insufficient
- the old `EvidenceIr::build()` sequence could synthesize useful `Enum ...` facts from tables and then end before later prose extraction had a chance to reuse those values
- weakly labeled encoding tables were easy to miss unless their headers already looked like explicit encoding tables
- asserted/deasserted signal constraints stayed polarity-agnostic even when the prose explicitly said a reset or control signal was active low/high
- a hardcoded APB/AHB/AXI value list was explicitly rejected; value recovery had to stay grounded in extracted PDF content

### Implementation shape
- `crates/specforge/src/ir/evidence.rs` now includes:
  - `scan_encoding_tables_by_signal_anchor()`
  - `collect_discovered_enum_values()`
  - `extract_discovered_state_value_from_text()`
  - `extract_signal_polarity_from_prose()`
  - `apply_signal_polarity_to_constraints()`
  - `extract_dynamic_signal_constraints()`
  - `dedup_actor_signal_relations()`
  - `converge_evidence_extractions()`
- `crates/specforge/src/commands/converge.rs` now provides the top-level fixed-point entrypoint for the staged pipeline: materialize `SourceIR` once, optionally enrich figures and normative prose, rebuild downstream IR stages, lower adapters, snapshot the resulting artifact facts, and stop when the snapshot is unchanged
- `EvidenceIr::build()` now carries forward persisted alias-learning state, NLP-upgraded statement classes, and structured NLP records when the rebuilt source/evidence surface still matches, so a second pass does not forget what the first pass learned
- `crates/specforge/src/commands/enrich.rs` now skips figures whose `VisualAsset.note` already contains a VLM extraction payload, keeping multi-pass orchestration idempotent instead of re-querying the same diagram every pass
- `synthesize_encoding_declarations()` now delegates to `synthesize_encoding_declarations_for_enum()` so the same enum synthesis logic can be reused by both the initial table pass and the anchored rescan path
- the convergence loop is monotone: each pass only adds new synthesized statements/records, then stops when no new evidence is created
- discovered enum/value atoms now come from extracted tables and synthesized `Enum ...` source facts rather than a protocol-specific baked-in list
- polarity refinement happens after prose extraction so `must_be_asserted` / `must_be_deasserted` can collapse to `must_be_low` / `must_be_high` when the spec explicitly states active-low/high semantics
- `crates/specforge/src/ir/source/docling_backend.rs` now lets `classify_table_kind()` look at captions, headers, and body rows together, which improves signal-description and encoding-table detection before `EvidenceIR` sees the table

### Validation and observed impact
- regression tests added:
  - `anchored_encoding_scan_unlocks_dynamic_value_constraint_extraction`
  - `prose_polarity_refines_asserted_constraint_kind`
- `converge_rebuilds_pipeline_until_snapshot_stabilizes`
- `cargo test --manifest-path Cargo.toml` now passes with 102 tests
- `cargo build --release --manifest-path Cargo.toml` passes
- refreshed representative local validation snapshots:
  - APB: 95/100 EXCELLENT
  - AHB: 95/100 EXCELLENT
  - AXI: 89/100 GOOD
- `generated/` is now git-ignored and intentionally untracked, so these validation snapshots live in the docs rather than in versioned artifacts

## Markdown-marker alias cleanup (2026-04-03)

### Root cause
- Form 2 alias learning in `specforge nlp-enrich` could still absorb markdown formatting noise when a normative sentence started with a bullet marker, table-cell marker, heading marker, block quote, or ordered-list marker before the real noun phrase.
- The concrete failure mode was learning aliases such as `- the address` instead of a real phrase such as `address bus`.

### Implementation shape
- `crates/specforge/src/commands/nlp_enrich.rs` now rejects alias subjects that begin with markdown/table/list prefixes such as `-`, `|`, `#`, `*`, `+`, `>`, `1.`, or `2)` before article stripping and phrase normalization.
- The ordinary noun-phrase path is unchanged, so genuine prose aliases still accumulate in `signal_alias_map`.
- Added regression coverage for marker-prefixed alias subjects.

### Validation
- `cargo test --manifest-path Cargo.toml` now passes with 102 tests.
- The staged README workflow was re-run end-to-end on `README.md` through:
  - `inspect`
  - `ingest`
  - `evidence`
  - `semantic`
  - `intent`
  - `.fsm` adapter dry-run
- The repo entry flow remains executable after the alias cleanup, and the README-derived staged artifacts still materialize successfully under `generated/.../readme/` as local ignored outputs.

### Remaining follow-up
- validation/back-annotation on staged IR and adapter artifacts is now the next workflow gap
- the larger downstream architectural gap is still actor-relative direction modeling in `SemanticIR` / `IntentIR`

## Validation back-annotation on IR artifacts (2026-04-04)

### Why this slice landed now
- the validation command already computed useful stage-aware diagnostics, but they vanished after printing
- the roadmap required reproducible artifact-linked reports, and the new actor-relative KG surface made graph-aware validation materially more useful
- the best next `R7` slice was therefore to persist validation state on the four IR stages before attempting automated live-doc projection

### Implementation shape
- shared validation report types now live in `crates/specforge/src/ir/source.rs`:
  - `ValidationReportRecord`
  - `ValidationMetricRecord`
  - `ValidationFindingRecord`
  - `ValidationFindingSeverity`
- `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` now carry `validation_reports: Vec<ValidationReportRecord>`
- `crates/specforge/src/commands/validate.rs` now:
  - computes a deterministic fingerprint for the artifact content with existing validation reports stripped
  - prints the stage-aware validation summary as before
  - writes a stage-local `validation_report.json` sidecar next to the artifact
  - backannotates the latest report into the artifact's `validation_reports` field
- the semantic/intent validators now emit graph-aware findings for:
  - signals with no resolved producers
  - signals with no resolved consumers
  - compatibility `direction_hint` lag relative to the actor-relative KG

### Validation
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 106 tests
- new regression coverage landed for:
  - source-stage validation backannotation + sidecar persistence
  - intent-stage score backannotation + sidecar persistence

### Remaining follow-up
- live-doc projection is no longer manual for staged IR artifacts; persisted reports can now be re-projected into tracked docs through `specforge project-validation`
- adapter validation remains outside this slice

## Live-doc projection of persisted validation reports (2026-04-04)

### Why this slice landed now
- `validation_reports` already existed on the staged IR artifacts, but the tracked markdown continuity surface still had to be edited by hand after validation runs
- `generated/` is intentionally untracked, so the repo needed a deterministic way to pull validation state back into tracked docs after meaningful local runs
- keeping the projection flow separate from `specforge validate` preserves a clean boundary: validation owns artifact truth, projection owns tracked-document continuity

### Implementation shape
- added `crates/specforge/src/commands/project_validation.rs` plus the `specforge project-validation <artifact>...` CLI command
- the command now:
  - validates each passed artifact through the existing `specforge validate` flow so persisted reports are current
  - reloads the latest backannotated `validation_reports` from those artifacts
  - writes a tracked `VALIDATION_SNAPSHOT.md` summary document
  - updates the managed `Validation Projection` block in `LIVE_ACHIEVEMENT_STATUS.md`
- the projection is deterministic:
  - artifacts are sorted by `document_key`
  - findings are sorted by severity then category/id
  - repo-internal artifact paths are rendered relative to the repo root, never as checkout-specific absolute paths

### Validation
- regression coverage now verifies that `specforge project-validation`:
  - validates an `IntentIR` artifact when needed
  - writes `VALIDATION_SNAPSHOT.md`
  - updates the managed live-status block
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 107 tests
- the tracked validation snapshot was refreshed from the current APB/AHB/AXI `IntentIR` artifacts:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 89/100 GOOD

### Remaining follow-up
- adapter validation is still outside the current projection flow
- `R15` still needs to demote flat compatibility `direction_hint` handling in favor of the actor-relative graph as the primary downstream surface

## Actor-relative KG carry-through in SemanticIR / IntentIR (2026-04-04)

### Why this slice landed now
- `EvidenceIR` already held the best structural graph in the pipeline via `actor_signal_relations`
- leaving that graph trapped in `EvidenceIR` meant later stages still defaulted to actor-agnostic `direction_hint` values
- the first necessary `R15` slice was therefore to preserve the graph downstream before trying to make adapters depend on it

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now carries:
  - `actor_signal_relations: Vec<ActorSignalRelation>`
  - `actor_ports: Vec<ActorPortRecord>`
  - `signal_connectivity: Vec<SignalConnectivityRecord>`
- `build_actors()` now seeds actor records from relation evidence, preserving grounded actor names when available
- `crates/specforge/src/ir/intent.rs` now preserves the same actor-relative KG surface as canonical output
- `crates/specforge/src/commands/validate.rs` now reports actor-signal relation, actor-port, and connectivity counts for `SemanticIR` and `IntentIR`
- legacy `InterfaceSignalRecord.direction_hint` remains in place as a compatibility surface; it is no longer the only downstream signal-direction representation

### Validation
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 104 tests
- new regression coverage landed for:
  - actor-relative port/connectivity construction in `SemanticIR`
  - actor-relative KG carry-through into `IntentIR`

### Remaining follow-up
- validation/back-annotation should become graph-aware so missing producers/consumers and contradictory actor relations surface explicitly
- `direction_hint` still drives some scoring/compatibility paths, so the remaining `R15` work is to make the graph-native actor-relative surface the primary downstream direction model

## Graph-first direction scoring in validation (2026-04-04)

### Why this slice landed now
- the previous `R15` slice preserved the actor-relative KG downstream, but the validator still treated flat `direction_hint` coverage as the effective truth surface for scoring
- that created the wrong incentive: a graph-complete artifact could still look incomplete merely because the compatibility view lagged behind
- the right next step was to make validation honest about the canonical signal model before continuing into temporal semantics or deeper KG work

### Implementation shape
- `crates/specforge/src/commands/validate.rs` now derives direction coverage from `actor_ports` first and only uses `InterfaceSignalRecord.direction_hint` as compatibility fallback
- semantic and intent validation metrics now separate:
  - `with_resolved_direction`
  - `with_graph_direction`
  - `with_compat_direction_hint`
- compatibility-surface lag remains a visible informational finding so older downstream consumers still get called out, but the quality score now follows the actor-relative graph when it is sufficient

### Validation
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 111 tests
- new regression coverage verifies that removing flat compatibility hints from an otherwise graph-complete `IntentIR` fixture does not reduce direction scoring

### Remaining follow-up
- some downstream consumers still read flat `direction_hint` fields directly, so this slice fixes scoring truthfulness but does not finish the whole `R15` program
- the next graph-first work should move remaining consumer logic onto actor-relative relations and keep `direction_hint` purely as a derived compatibility surface

## Graph-backed `.fsm` module direction recovery (2026-04-11)

### Why this slice landed now
- `R15` was still blocked by downstream consumers that required flat `direction_hint` values even when the actor-relative graph already knew the target actor's port direction.
- Explicit module/top lowering was a safe first adapter consumer because the module name provides the target actor context needed to interpret `IntentIR.actor_ports`.
- This moves one adapter path from "flat hint required" toward "graph first, flat hint compatible" without widening `.fsm` lowering into speculative direction inference.

### Implementation shape
- `crates/specforge/src/ir/adapters.rs` now overlays explicit module signal inventory with matching `IntentIR.actor_ports` before renderability analysis.
- For a module actor such as `producer_core`, an actor port `output_data: output` can fill a missing module-local direction hint while preserving the signal's existing width and provenance.
- `ActorRelativeDirection::Input` maps to an adapter input and `ActorRelativeDirection::Output` maps to an adapter output; `InOut` and `Unknown` stay unresolved for the current `.fsm` slice.
- Existing hint merge behavior remains conservative: conflicting flat and graph directions collapse to `None`, which keeps lowering blocked instead of selecting a hidden winner.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_recovers_child_directions_from_actor_ports -- --nocapture` passed.
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` passed across the top-composition focused regression set.
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with 19 adapter tests.
- `bash scripts/run_ci.sh` passed with Clippy `-D warnings`, 284 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build.
- The first regression deliberately clears the flat child-module signal directions in an explicit top composition and proves the composition still lowers when graph-backed actor ports provide the child module directions.
- The second regression supplies a conflicting graph direction for `producer_core.output_data` and proves lowering stays blocked instead of silently overriding the flat module-local direction.

### Remaining follow-up
- Extend the same graph-first discipline into the remaining direct adapter/control consumers that still consult `direction_hint` without a target actor context.
- Keep bidirectional and unknown graph directions blocked until a target backend slice has an honest representation for them.

## Graph-backed `.fsm` direct-root direction recovery (2026-04-11)

### Why this slice landed now
- After explicit module/top lowering, standalone direct `.fsm` roots were the next safe `R15` consumer still blocked by flat `direction_hint` lag.
- Direct roots are riskier than explicit modules because they do not carry a module name that identifies the target actor context.
- The safe rule is therefore narrower: graph-backed actor-port evidence can fill missing direct-root signal directions only when all renderable actor ports relevant to the direct local signal inventory point at one unambiguous actor.

### Implementation shape
- `crates/specforge/src/ir/adapters.rs` now checks `IntentIR.actor_ports` during direct signal-inventory construction.
- If the renderable actor-port graph for signals already present in the direct inventory has exactly one actor, matching `Input` / `Output` actor-relative directions can fill missing local signal directions.
- The direct-root overlay deliberately does not add graph-only signals; it only strengthens the already-local signal inventory.
- Unrelated graph-only actor ports are ignored by the direct-root actor-context gate and remain absent from the standalone signal inventory.
- If the graph mixes multiple actors, the adapter leaves missing flat directions unresolved and renderability remains blocked.
- Actor-port provenance is now merged once per actor-port record before hint reconciliation, so a conflicting graph direction with multiple supporting ids cannot accidentally restore a direction after the first merge collapsed it to unresolved.

### Validation
- The new regressions prove the gate: one standalone direct root lowers when a single `controller` actor supplies the missing `DATA_IN`, `DATA_OUT`, and `ZERO_FLAG` directions, an unrelated graph-only `SIDE_BAND` actor port stays ignored, and a mixed producer/consumer actor-port graph stays blocked with missing direction hints.
- The top-composition conflict regression now includes a duplicate supporting id on the conflicting graph port, proving multi-provenance graph evidence still blocks instead of weakening the conflict.
- `cargo test --manifest-path Cargo.toml standalone_dt -- --nocapture` passed across the focused standalone direct regression set.
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_actor_port_directions -- --nocapture` passed for the duplicate-provenance conflict regression.
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with 22 adapter tests.
- `bash scripts/run_ci.sh` passed with Clippy `-D warnings`, 287 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build.

## Top-link-backed `.fsm` boundary direction recovery (2026-04-11)

### Why this slice landed now
- After module and standalone actor-port overlays, top boundary ports were still more fragile than necessary: `SemanticIR` dropped explicit top ports that had width but no flat direction even when explicit composition links made the boundary role deterministic.
- A top endpoint used as a link source has a clear `.fsm` top-boundary role: input. A top endpoint used as a link target has a clear role: output.
- This is not actor-graph inference; it is explicit composition-topology recovery, so it stays bounded to the `?top:name` adapter path.

### Implementation shape
- `ExplicitTopPortRecord.direction_hint` is now optional, allowing width-only top boundary ports to survive `SemanticIR` / `IntentIR`.
- `crates/specforge/src/ir/adapters.rs` now merges top boundary port directions from explicit top-link endpoints before renderability analysis.
- Conflicting explicit direction versus link topology stays blocked, unresolved top boundary ports stay blocked, and renderable top roots carry resolved port directions before emitting `.fsm` text.

### Validation
- The new regression proves a width-only top port `result_data` stays canonical and renders as `result_data>8` only because the explicit top link `consumer.result_data -> result_data` recovers the top-output role.
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` passed across the focused top-composition set.
- `cargo test --manifest-path Cargo.toml extracts_explicit_modules_and_tops_from_markdown -- --nocapture` passed after making top port direction optional.
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` passed with 23 adapter tests.
- `bash scripts/run_ci.sh` passed with Clippy `-D warnings`, 288 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build.

## KG-bench graph-backed direction assertions (2026-04-11)

### Why this slice landed now
- R15 is deliberately moving downstream consumers from flat compatibility `direction_hint` toward actor-relative graph surfaces.
- `kg-bench` could already assert specific actor ports, and it could assert flat per-signal directions, but it did not have a direct expectation for graph direction coverage by signal name.
- Reusing `signal_directions_include` for this would be wrong because some fixtures intentionally preserve a flat interface perspective while actor ports describe producer/consumer-relative roles.

### Implementation shape
- `CanonicalStageExpectations` now accepts `graph_direction_signal_names_include` and `graph_direction_signal_names_exclude`.
- The harness computes the set from canonical `actor_ports` with any non-`unknown` actor-relative direction.
- The existing `signal_directions_include` behavior remains flat and compatibility-specific, so graph-native and compatibility surfaces stay testable independently.

### Validation
- `actor_ports_gold` now proves the new expectation surface across both `SemanticIR` and `IntentIR` by requiring graph-backed coverage for `PREADY` and excluding unrelated `PSEL`.
- `cargo fmt --all --check` passed.
- `git diff --check` passed.
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` passed with all 45 tracked KG fixtures.
- `bash scripts/run_ci.sh` passed with Clippy `-D warnings`, 288 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build.

## Asserted-when-level control polarity recovery (2026-04-11)

### Why this slice landed now
- The canonical polarity surface already carries active-high / active-low facts and uses them to keep `ASSERTED` / `DEASSERTED` polarity-relative.
- Reset polarity was well covered, but the user reminded us that every single-bit control signal has a polarity, not only resets.
- A safe next step was to widen explicit local phrase coverage for non-reset controls without inferring polarity from suffixes alone.

### Implementation shape
- `EvidenceIR` polarity detection now recognizes explicit forms such as `asserted when LOW`, `LOW when asserted`, `asserted by driving LOW`, and `driven LOW to assert` as active-low evidence.
- The same asserted-when-level forms are supported for active-high evidence.
- This is still local-evidence recovery: `CS_N` can be active-low when the current document says it is asserted when low, but `_N` alone is not treated as sufficient.
- `SemanticIR` now suppresses redundant one-signal heuristic interface candidates when that signal is already explicitly declared, so local polarity/control prose enriches the canonical declared signal instead of creating a duplicate low-confidence interface record.

### Validation
- Added a detector-level regression for asserted-when-level phrase forms.
- Added a non-reset `CS_N` control-signal regression proving `CS_N is asserted when LOW` recovers active-low polarity and refines `CS_N must be asserted` / `CS_N must be deasserted` into LOW / HIGH constraints.
- Added a KG-quality fixture for non-reset control polarity recovery with no polarity or temporal conflicts.
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` passed with 21 focused polarity tests.
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` passed with all 46 tracked KG fixtures.
- `bash scripts/run_ci.sh` passed with Clippy `-D warnings`, 292 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build.

## Initial typed temporal-rule surface in SemanticIR / IntentIR (2026-04-04)

### Why this slice landed now
- the roadmap already promoted explicit clock-tick semantics to first-class status, but the code still represented timing mainly as free-form timing-parameter records plus prose-derived signal/conditional constraints
- that meant the behavioral KG had useful ingredients but no canonical temporal rule layer to unify them
- the right first `R15b` move was to add a typed temporal surface now, even if the extraction heuristics are still intentionally narrow

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now defines:
  - `ClockEdge`
  - `TickPhase`
  - `CycleWindowRecord`
  - `TemporalPredicateRecord`
  - `TemporalRuleRecord`
- `SemanticIR` now carries `temporal_rules`
- `IntentIR` now carries the same `temporal_rules` forward as canonical behavioral structure
- the first derivation pass currently lifts:
  - conditioned signal constraints like `HTRANS must not change when HREADY is LOW`
  - structured conditional rules with recognizable value/stability consequents
  - timing descriptions that say a signal is sampled on a rising/falling edge
- temporal grounding now uses an explicit `Clock <signal>.` declaration even when a full `SystemContractRecord` is not yet available

### Validation
- `crates/specforge/src/commands/validate.rs` now reports:
  - `temporal_rules`
  - `temporal_rules_missing_clock_grounding`
- validation now flags:
  - temporal evidence without any typed temporal-rule derivation
  - typed temporal rules that still lack explicit clock/edge grounding
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 114 tests

### Remaining follow-up
- this is the first typed temporal layer, not the full temporal semantics program
- at that point, cycle windows, richer drive-maintains-stability semantics, multi-predicate antecedents, contradiction detection, and richer VLM timing lift still needed to land before `R15b` could be considered complete

## Cycle-window recovery in temporal rules (2026-04-04)

### Why this slice landed now
- the first temporal-rule pass captured phase-relative value/stability/sampling semantics, but still left latency unbounded
- the roadmap explicitly calls out cycle windows, and the docs already frame many protocol guarantees in terms of bounded cycle counts
- recovering bounded windows from the prose we already structure is a high-value next increment because it upgrades the temporal layer from "what happens on an edge" to "within how many cycles it must happen"

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now derives `CycleWindowRecord` from temporal source text patterns such as:
  - `within N cycles`
  - `for N cycles`
  - `at least N cycles`
  - `at most N cycles`
  - `between N and M cycles`
- timing constraints whose unit is already `cycles` now also project numeric `min/typ/max` values into the typed temporal-rule `cycle_window`
- the temporal layer remains conservative: if no trustworthy cycle-bound phrase is found, the rule stays unbounded instead of inventing latency

### Validation
- `crates/specforge/src/commands/validate.rs` now reports `temporal_rules_with_cycle_window`
- validation now explicitly flags temporal-rule sets that still have no bounded cycle windows at all
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 115 tests

### Remaining follow-up
- this still does not cover richer latency language like protocol-phase aliases, burst-relative windows, or contradictory latency evidence across modalities
- at that point, multi-step temporal rules, richer actor-relative stability semantics, and contradiction handling were still the next meaningful `R15b` deepening steps

## Actor-grounded temporal drive events (2026-04-04)

### Why this slice landed now
- the temporal layer had started to recover value, stability, edge sampling, and bounded latency, but it still lost the producer actor even when the structural KG already knew exactly who drives the signal
- that mismatch weakened the whole “KG-first” story: structural truth and temporal truth were still partially disconnected
- the next honest `R15b` step was therefore to let temporal rules reuse unique producer information from `signal_connectivity`

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now adds `TemporalPredicateRecord::ActorDrivesSignal`
- temporal derivation now emits `ActorDrivesSignal` for value-oriented consequents when:
  - the rule targets a specific signal
  - the structural KG resolves exactly one producer actor for that signal
- the derivation remains conservative: ambiguous/multi-producer signals stay signal-only instead of inventing a wrong actor binding

### Validation
- `crates/specforge/src/commands/validate.rs` now reports `temporal_rules_with_actor_grounding`
- validation now flags temporal-rule sets that coexist with a non-empty actor-signal graph but still have zero actor-grounded temporal predicates
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 118 tests

### Remaining follow-up
- this is still only the first actor-aware temporal slice
- at that point, actor-relative drive-maintains-stability semantics, multi-step temporal chains, and contradiction/arbitration across competing actor-grounded rules still needed to land before `R15b` was mature

## Actor-grounded stability semantics in temporal rules (2026-04-04)

### Why this slice landed now
- the temporal layer had learned who drives a signal for value-setting rules, but stable/hold constraints still dropped back to signal-only semantics
- that was an important semantic gap because many protocol rules are really producer obligations: not just “signal remains stable,” but “the producer must keep it stable”
- the next honest `R15b` step was therefore to connect stability semantics back to the same unique-producer KG surface already used for value-drive rules

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now adds `TemporalPredicateRecord::ActorMaintainsSignalStable`
- stable/hold-style consequents now emit `ActorMaintainsSignalStable` when:
  - the rule targets a specific signal
  - the structural KG resolves exactly one producer actor for that signal
- the signal-level `SignalStable` predicate is still kept, so the temporal layer preserves both the abstract invariant and the actor-responsibility view

### Validation
- `crates/specforge/src/commands/validate.rs` now counts actor-grounded stability predicates as part of `temporal_rules_with_actor_grounding`
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 121 tests

### Remaining follow-up
- multi-predicate antecedents, richer temporal composition, and contradiction/arbitration across actor-grounded temporal rules are still the next meaningful `R15b` deepening steps

## Compound temporal antecedents in typed temporal rules (2026-04-04)

### Why this slice landed now
- the temporal layer had started to capture edge-relative value/stability obligations, but compound guards were still being flattened to a single partial condition during semantic lift
- that was a real semantic loss for protocols, because many obligations are conjunctive rather than unary: `when HREADY is LOW and HSEL is HIGH` should survive as two grounded preconditions, not one half-parsed hint
- the next honest `R15b` step was therefore to preserve conjunctive guards without over-claiming full contradiction solving yet

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now splits compound temporal guard text on conjunctions only when each resulting clause is anchored to a known signal
- the temporal layer remains conservative: ambiguous `and` usage that cannot be grounded clause-by-clause stays unsplit rather than inventing structure
- `parse_temporal_condition_predicates()` now returns multiple `SignalValue` antecedents for compound guards like `when HREADY is LOW and HSEL is HIGH`
- `IntentIR` carries those richer antecedent vectors forward unchanged as part of the canonical temporal-rule surface

### Validation
- `crates/specforge/src/commands/validate.rs` now reports `temporal_rules_with_multi_predicate_antecedents`
- added end-to-end tests for:
  - deriving multi-predicate antecedents in `SemanticIR`
  - carrying them into `IntentIR`
  - validating that the richer temporal guard surface is counted explicitly
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 124 tests

### Remaining follow-up
- contradiction detection and cross-modality arbitration are still the next major `R15b` steps
- the current compound-guard lift is intentionally conjunctive-only; disjunctive and more symbolic temporal composition still need a first-class model

## Typed temporal conflict records (2026-04-04)

### Why this slice landed now
- once the temporal layer could preserve richer guards, the next truthfulness gap was no longer “can we express the precondition,” but “can we say when two typed rules disagree under that same precondition”
- leaving that disagreement implicit would weaken the whole provenance-first story, because downstream consumers would still need to rediscover contradictions by re-reading the rule set
- the next honest `R15b` move was therefore to preserve contradictory temporal value obligations as explicit typed records before attempting broader arbitration

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now defines `TemporalConflictRecord` and derives it from typed temporal rules
- the first conflict detector is intentionally narrow and high-confidence:
  - it groups signal-value consequents by clock/edge, antecedent set, cycle window, signal, and phase
  - it emits a conflict only when multiple distinct values are required for the same signal/phase under the same grounded context
- `IntentIR` now carries the same `temporal_conflicts` surface forward so contradiction information is preserved beyond the semantic stage

### Validation
- `crates/specforge/src/commands/validate.rs` now reports `temporal_conflicts` for `SemanticIR` and `IntentIR`
- validation now emits explicit findings when typed temporal conflicts are present
- added end-to-end tests for:
  - deriving a typed temporal conflict from contradictory value obligations in `SemanticIR`
  - carrying that conflict into `IntentIR`
  - validating that the contradiction surfaces as a typed temporal-conflict finding
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 127 tests

### Remaining follow-up
- this is still only the first contradiction slice; it does not yet arbitrate between prose/table/figure evidence or handle richer predicate clashes like stability-vs-transition or actor-vs-actor disagreements
- broader temporal arbitration remains a follow-on task, not something this slice pretends to solve

## Polarity-aware assertion semantics in temporal conflicts (2026-04-08)

### Why this matters
- `ASSERTED` and `DEASSERTED` are not fixed level semantics on their own
- for any single-bit control signal, assertion semantics are defined by the signal polarity
- active-low controls like `ARESETN` and `rst_n` are asserted when `LOW`, not when `HIGH`

### Steering rule
- temporal conflict detection must never treat `ASSERTED == HIGH` or `DEASSERTED == LOW` as universal truths
- instead, assertion semantics should only collapse into level semantics when the current document grounds the signal polarity
- when polarity is unknown, assertion semantics should stay abstract so the pipeline does not fabricate or hide contradictions

### Current implementation shape
- `EvidenceIR` now persists resolved `signal_polarities`, not just `signal_polarity_conflicts`
- `SemanticIR` and `IntentIR` now carry that polarity surface downstream
- temporal conflict grouping now compares values in a polarity-aware way:
  - active-high: `ASSERTED -> HIGH`, `DEASSERTED -> LOW`
  - active-low: `ASSERTED -> LOW`, `DEASSERTED -> HIGH`
  - unknown polarity: `ASSERTED` / `DEASSERTED` remain assertion-domain values
- this keeps `ARESETN`-style active-low semantics correct without flattening unknown-polarity control signals into unsafe level assumptions

## Signal-table polarity refinement in convergent EvidenceIR (2026-04-04)

### Why this slice landed now
- the evidence loop already used known signals to unlock encoding tables, but polarity refinement was still prose-only even though protocol PDFs often place active-high/active-low semantics in signal-description rows
- that left a real multimodal gap: the KG could know the signal inventory and still miss polarity facts that were sitting in the same table family that introduced those signals
- the next honest evidence-side step was therefore to let the convergent loop mine `SignalDescription` tables for polarity using known signals as anchors

### Implementation shape
- `crates/specforge/src/ir/evidence.rs` now collects polarity facts from both:
  - prose statements mentioning a known signal with active-high/active-low language
  - `SignalDescription` table rows whose signal cell anchors to a known signal and whose row text carries active-high/active-low language
- polarity facts from prose and tables are merged conservatively:
  - matching polarity reinforces the fact
  - contradictory polarity removes the fact instead of forcing a wrong refinement
- the merged polarity map is then reused by the existing asserted/deasserted constraint refinement step
- `EvidenceIR` now also persists a typed `signal_polarity_conflicts` surface, so contradictory polarity remains explicit in the artifact instead of only being visible indirectly through a polarity-neutral derived constraint
- `specforge validate` now prints and flags those polarity conflicts, including which polarity each modality asserted and which statement/table ids supported it
- that conflict should not stop at the evidence stage:
  - if downstream canonical layers carry reset and signal-level behavior, they also need to carry contradictory active-level evidence instead of pretending the disagreement ended upstream
  - the right shape is the same one used for semantic-role conflicts: preserve the typed polarity conflict surface through `SemanticIR` and `IntentIR`, then report it there too

### Validation
- added end-to-end tests for:
  - refining an asserted constraint from a signal-description table row that says the signal is active low
  - keeping a constraint polarity-neutral when prose and table polarity disagree
  - reporting the polarity conflict explicitly from `specforge validate`
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 130 tests

### Remaining follow-up
- this is the first evidence-arbitration slice, not the whole arbitration story; only polarity disagreement is typed so far
- the current polarity scan is still text-pattern based; richer table-structure understanding and non-signal-description table rescans remain future work

## Structural KG conflict surfacing for multi-producer ambiguity (2026-04-04)

### Why this slice landed now
- after surfacing polarity disagreement, the next obvious truthfulness gap was in the structural KG itself: `signal_connectivity` could already show more than one producer for a signal, but that ambiguity remained implicit in raw vectors instead of becoming a typed, validator-visible conflict
- for a project that wants the KG to be trustworthy, unresolved producer ambiguity cannot stay hidden behind “just inspect the connectivity list”

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now derives `signal_connectivity_conflicts` from `signal_connectivity`
- the first conflict kind is `multiple_producers`
- each conflict keeps:
  - the signal name
  - the conflicting actor ids / actor names
  - the supporting statement ids
  - automation confidence
- `crates/specforge/src/ir/intent.rs` now carries that structural conflict surface forward so the canonical endpoint keeps the ambiguity explicit
- `crates/specforge/src/commands/validate.rs` now prints and flags those conflicts for both `SemanticIR` and `IntentIR`

### Validation
- added end-to-end tests for:
  - deriving a structural signal-connectivity conflict in `SemanticIR`
  - carrying that conflict into `IntentIR`
  - reporting the conflict in `specforge validate`
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 133 tests

### Remaining follow-up
- this is still the first structural-KG arbitration slice, not the whole graph-conflict story
- it currently surfaces multi-producer ambiguity only; broader graph disagreement like conflicting widths, contradictory read/write claims, or modality-ranked arbitration remains future work

## Interface-signal conflict surfacing for conflicting declarations (2026-04-04)

### Why this slice landed now
- after surfacing polarity conflicts and multi-producer ambiguity, another quiet truthfulness failure remained in the canonical interface surface itself: conflicting explicit signal declarations could disagree on direction or width, and the builder would only collapse the hint to `None`
- that meant disagreement was technically preserved only as absence, which is too implicit for a project that wants a top-notch KG and canonical IR

### Implementation shape
- `crates/specforge/src/ir/semantic.rs` now derives `interface_signal_conflicts` while building interfaces
- the first conflict kinds are:
  - `direction_mismatch`
  - `width_mismatch`
- each conflict keeps:
  - the signal name
  - the conflicting observed values
  - the supporting statement ids for each observed value
  - automation confidence
- `crates/specforge/src/ir/intent.rs` now carries that interface-shape conflict surface forward
- `crates/specforge/src/commands/validate.rs` now prints and flags those conflicts for both `SemanticIR` and `IntentIR`

### Validation
- added end-to-end tests for:
  - deriving direction/width conflicts from contradictory explicit declarations in `SemanticIR`
  - carrying those interface conflicts into `IntentIR`
  - reporting them in `specforge validate`
- `cargo fmt --all` passed
- `cargo test --manifest-path Cargo.toml` now passes with 136 tests

### Remaining follow-up
- this still only covers explicit interface-shape disagreement; it does not yet arbitrate conflicting width/direction evidence across all modalities or between canonical interface hints and actor-relative graph evidence

## Documentation surface currently steering the implementation
- `README.md`
  - single entry point and quick orientation
- `INTENTIR_SPEC.md`
  - canonical architecture and stage specification
- `ROADMAP.md`
  - live implementation sequence
- `RUST_CODEBASE_ANALYSIS.md`
  - architecture/risk assessment
- `USER_GUIDE.md`
  - current and planned CLI/user workflow
- `MEMORY.md`
  - continuity record for restart/handoff

## Current Rust code boundaries
### Workspace shape
- root workspace manifest: `Cargo.toml`
- active CLI crate: `crates/specforge`

### Module boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch and module exports
- `src/cli.rs`
  - clap CLI model for `specforge`
- `src/error.rs`
  - typed error/result boundary
- `src/commands/inspect.rs`
  - source/path inspection command
- `src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `src/commands/adapt.rs`
  - `.fsm` adapter preview/materialization command
- `src/ir/mod.rs`
  - stage identifiers and IR namespace
- `src/ir/source.rs`
  - `SourceIR` types, normalization planning, parser backend selection, page/visual artifact manifests, and source-side residual decisions
- `src/ir/source/docling_backend.rs`
  - runtime backend discovery, external Docling orchestration, and the embedded Python helper for structured PDF materialization
- `src/ir/evidence.rs`
  - first real multimodal `EvidenceIR` builder for text spans, figure/caption linking, visual evidence, and extracted statements
- `src/ir/semantic.rs`
  - first real `SemanticIR` builder for deterministic semantic lifting and residual-decision generation
- `src/ir/intent.rs`
  - first real `IntentIR` builder for deterministic canonicalization and residual-decision preservation
- `src/ir/adapters.rs`
  - typed adapter artifacts, honest standalone/structured `.fsm` lowering logic, and adapter-side residual-decision/renderability reporting

## Newly completed architectural pivot
- the CLI/crate identity is now `specforge`
- the repo objective has been rewritten around `IntentIR`
- `.fsm` is now documented as an adapter target instead of the core endpoint
- `specforge ingest` now materializes `SourceIR` at `generated/source_ir/<document_key>/source_ir.json`
- explicit scaffolding exists for the full staged pipeline:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters
- `INTENTIR_SPEC.md` now records the canonical long-form architecture and examples for future implementation work

## Immediate implementation consequences
- do not jump to `.fsm` generation from `SourceIR`
- keep the current `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` types stable enough that later adapter builders can depend on them
- use the newly materialized `IntentIR` actors, interface inventory, control fragments, behaviors, constraints, assumptions, and residual decisions as the substrate for adapter lowerings
- use `subs/fsmgen` as a local reference implementation for `.fsm` expectations and comparisons, but do not let that reference redefine the canonical `IntentIR` boundary
- do not edit `subs/fsmgen` from this repository; if upstream behavior appears wrong, file a thorough local tracked bug report instead
- use the local upstream bug-report ID format `FSMGEN-BUG-####` when such issues are found
- keep the current `EvidenceIR`, `SemanticIR`, and `IntentIR` passes provenance-first so later adapter lowering stays grounded
- do not let figures, charts, or diagrams collapse into throwaway markdown placeholders if they may carry normative meaning

## Immediate next engineering target
- build the validation/back-annotation pipeline so staged IR and adapter outputs have reproducible artifact-linked reports
- keep broader target structure deferred until the canonical model carries it explicitly

## 2026-04-04 - idiomatic one-cycle temporal language
- `crates/specforge/src/ir/semantic.rs` now recognizes idiomatic one-cycle latency phrases in the temporal lift:
  - `next cycle`
  - `next clock cycle`
  - `next tick`
  - `next rising edge`
  - `following` / `subsequent` variants of those phrases
- these phrases now map onto the same canonical `CycleWindowRecord { min_cycles: Some(1), max_cycles: Some(1) }` surface already used for numeric latency bounds
- this keeps the clock-tick model unified instead of creating a side heuristic for prose that describes one-cycle latency without an explicit numeral
- added regression coverage for:
  - direct parser recovery of single-cycle windows from idiomatic phrases
  - end-to-end temporal-rule derivation from a `next tick` signal constraint
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `138/138`

## 2026-04-04 - typed ready/valid handshake completion
- `crates/specforge/src/ir/semantic.rs` now defines `TemporalPredicateRecord::HandshakeComplete`
- the semantic temporal lift now adds that predicate when a temporal context contains a grounded `VALID`-like signal and a grounded `READY`-like signal that are both asserted in the same phase
- this is additive, not lossy:
  - the original `SignalValue` guard predicates are still preserved
  - the higher-level handshake event is carried alongside them for downstream protocol reasoning
- `crates/specforge/src/commands/validate.rs` now reports `temporal_rules_with_handshake_completion`
- added regression coverage for:
  - deriving a handshake predicate from a valid/ready compound guard in `SemanticIR`
  - carrying that predicate into `IntentIR`
  - surfacing the handshake metric in `specforge validate`
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `141/141`

## 2026-04-04 - meaning-grounded handshake roles from signal descriptions
- the previous handshake slice was intentionally conservative and still depended on literal `VALID` / `READY` signal naming when no richer role evidence existed
- that was useful, but it was not yet aligned with the project doctrine that protocol meaning should outrank spelling when the document provides enough grounded evidence
- `crates/specforge/src/ir/evidence.rs` now mines `SignalDescription` tables for typed `signal_semantic_hints`:
  - `HandshakeValidLike`
  - `HandshakeReadyLike`
- these hints are conservative and provenance-carrying:
  - they only land when the description text itself says something semantically close to "information/request is valid" or "the receiver can accept / acknowledge / complete the transfer"
  - they preserve the source text, table provenance, and automation confidence instead of collapsing immediately into an irreversible interpretation
- `crates/specforge/src/ir/semantic.rs` now carries those roles forward as per-signal `semantic_tags` on `InterfaceSignalRecord`
- `crates/specforge/src/ir/intent.rs` now preserves the same `semantic_tags` surface at the canonical endpoint
- handshake derivation now consults those meaning-grounded semantic tags before falling back to literal signal-name heuristics
- this is the right architectural direction:
  - protocol meaning can now begin to outrank orthography
  - the pipeline is still not pretending to solve open-ended language understanding
  - instead, it is recovering a narrow typed protocol-role surface from grounded table evidence and then reusing it downstream
- `crates/specforge/src/commands/validate.rs` now reports:
  - `signal_semantic_hints` for `EvidenceIR`
  - `with_semantic_tags` for `SemanticIR` and `IntentIR`
- this keeps the new meaning-grounded role surface visible in validation instead of hiding it inside the temporal-rule count
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `146/146`
- the next honest follow-up is to broaden the same meaning-based role inference beyond signal-description tables into aliases, prose, and multimodal grounding so handshake and role semantics do not depend on one table shape

## 2026-04-04 - prose and alias-grounded handshake roles in EvidenceIR
- the previous slice established the typed role surface, but it still depended on signal-description tables as the only evidence source for `signal_semantic_hints`
- that was not enough for the roadmap target:
  - some specs explain role meaning in prose paragraphs rather than in the table row itself
  - some later passes learn a useful alias but, before this slice, that alias only helped constraint reclassification and not semantic role grounding
- `crates/specforge/src/ir/evidence.rs` now refreshes `signal_semantic_hints` from:
  - signal-description tables
  - `SourceFact` prose descriptions that explicitly mention a signal
  - alias-grounded prose descriptions where Form 2 alias learning resolves the prose subject to a canonical signal
- the current prose path is still intentionally conservative:
  - it only promotes `SourceFact` statements, not arbitrary normative text
  - it requires exactly one resolved signal target after combining direct signal mentions and alias resolution
  - it reuses the same narrow handshake-role tagger instead of inventing a second looser semantic path
- `crates/specforge/src/commands/nlp_enrich.rs` now calls `refresh_signal_semantic_hints()` before persisting updated `EvidenceIR`
- this closes an important loopback gap:
  - alias learning no longer stops at `SignalConstraintRecord` recovery
  - the same learned alias vocabulary can now immediately feed role grounding for downstream temporal semantics
- `crates/specforge/src/commands/validate.rs` now also breaks out `signal_semantic_hints` by source kind, including alias-grounded prose
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `149/149`
- the next honest follow-up is multimodal grounding beyond prose and tables:
  - diagram captions
  - VLM timing/state explanations
  - richer actor/role phrasing in normative prose

## 2026-04-04 - explicit semantic-role conflicts in EvidenceIR
- the previous role-inference slices expanded the evidence sources for `signal_semantic_hints`, which made the truthfulness risk more obvious:
  - the same signal can accumulate incompatible role evidence
  - for example, one source can make it look valid-like while another makes it look ready-like
- before this slice, that disagreement would survive only as a dual-tag ambiguity on the signal and later handshake-role classification would quietly return `None`
- that was too silent for a project that is explicitly trying to surface conflicts instead of hiding them
- `crates/specforge/src/ir/evidence.rs` now persists `signal_semantic_conflicts` as a first-class typed record
- each conflict keeps:
  - the signal name
  - the conflicting role observations
  - the source kind
  - supporting statement/table references
- `crates/specforge/src/commands/validate.rs` now:
  - prints a dedicated `Signal Semantic Conflicts` section for `EvidenceIR`
  - emits a `signal_semantic_conflicts` metric
  - raises a warning finding when incompatible role evidence is present
- this is the right truthfulness behavior:
  - the pipeline can still carry the underlying evidence forward
  - but the disagreement is no longer silent
  - users can inspect and judge whether the semantic role inference needs more evidence or arbitration
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `151/151`

## 2026-04-04 - carry semantic-role conflicts into SemanticIR and IntentIR
- surfacing `signal_semantic_conflicts` only in `EvidenceIR` was a good first truthfulness step, but it still left a canonical-layer gap:
  - downstream consumers could inspect `semantic_tags`
  - validation at the semantic/intent stages could see that some roles were missing
  - but the explicit reason, contradictory role evidence for the same signal, disappeared once the pipeline moved past `EvidenceIR`
- that was still too silent for a graph-first canonical pipeline
- `crates/specforge/src/ir/semantic.rs` now carries `signal_semantic_conflicts` forward from `EvidenceIR`
- `crates/specforge/src/ir/intent.rs` now carries the same conflict surface forward again into the canonical endpoint
- `crates/specforge/src/commands/validate.rs` now:
  - reports `signal_semantic_conflicts` for both `SemanticIR` and `IntentIR`
  - prints the same conflict details there, not only at the evidence stage
  - raises explicit warning findings when those carried semantic-role conflicts are still unresolved
- this keeps the truthfulness story intact end-to-end:
  - evidence can disagree
  - that disagreement can survive into canonical IR
  - downstream lowering or review can see the unresolved ambiguity instead of only seeing the absence of a derived handshake role
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `155/155`
- the next honest follow-up remains richer multimodal role grounding and broader arbitration:
  - diagram captions
  - VLM timing/state explanations
  - modality-aware arbitration once multiple grounded role candidates survive into the same canonical signal

## 2026-04-04 - initial multimodal semantic-role grounding
- the previous role-inference slices were still too text-centric:
  - signal-description tables worked
  - direct prose and alias-grounded prose worked
  - but captions and VLM timing explanations, both first-class evidence sources in this project, still could not contribute to the role surface
- that was below the intended quality bar for a multimodal protocol compiler
- `crates/specforge/src/ir/evidence.rs` now refreshes `signal_semantic_hints` from:
  - grounded visual captions
  - VLM timing-diagram annotations
- the new visual path stays intentionally conservative:
  - it only promotes hints when the caption or annotation implies a handshake-like role meaning
  - it only accepts the evidence when exactly one known signal can be resolved from the text
  - VLM timing annotations reuse the same robust fenced/prose-wrapped JSON extraction path that the semantic VLM lift already needed for real Ollama output
- `SignalSemanticHintRecord` now also carries `supporting_visual_evidence_ids`, so caption/VLM-derived hints keep explicit provenance rather than collapsing into anonymous text
- `crates/specforge/src/commands/validate.rs` now breaks out:
  - `signal_semantic_hints_from_visual_captions`
  - `signal_semantic_hints_from_vlm_timing_annotations`
- the new multimodal slice is not just stored; it already feeds downstream semantics:
  - a new end-to-end semantic test shows caption-grounded role hints can derive a typed `HandshakeComplete` predicate
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `159/159`
- the next honest follow-up remains broader multimodal and arbitration depth:
  - richer actor/role phrasing in normative prose
  - state-machine/VLM explanation grounding beyond timing annotations
  - modality-aware arbitration when caption, prose, table, and VLM role candidates disagree

## 2026-04-04 - canonical semantic-role observations replace lossy tag-only carry-through
- after the multimodal grounding slice, a new quality gap became obvious in the canonical layers:
  - `EvidenceIR` had rich semantic-role hints with provenance
  - `SemanticIR` / `IntentIR` kept only merged `semantic_tags`
  - that meant canonical consumers lost the distinction between table/prose/visual support and could not inspect how a role meaning had been established
- that was a lossy design, so it was not good enough for the project quality bar
- `crates/specforge/src/ir/semantic.rs` now defines `semantic_observations` on `InterfaceSignalRecord`
- each observation keeps:
  - semantic tags
  - source kind
  - source text
  - statement/table/visual provenance ids
  - automation confidence
- `SemanticIR` now builds those observations directly from `EvidenceIR.signal_semantic_hints`
- `IntentIR` now carries the same per-signal semantic observation surface forward unchanged
- `crates/specforge/src/commands/validate.rs` now reports:
  - `semantic_observations`
  - `with_visual_semantic_grounding`
- this is a better canonical design because:
  - merged `semantic_tags` still exist for quick downstream use
  - but the canonical layers no longer destroy the richer provenance needed for inspection, arbitration, and future SOTA-quality consumers
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `160/160`
- the next honest follow-up is to let those canonical observations participate in stronger arbitration, not just preservation:
  - modality-aware role preference when evidence strengths differ
  - richer reporting of which canonical role meanings are single-source vs multi-source grounded

## 2026-04-04 - canonical semantic-role consensus now uses preserved observations
- the previous slice preserved canonical `semantic_observations`, but one remaining downstream consumer was still weaker than the new data model:
  - handshake-role resolution still consulted merged `semantic_tags`
  - validation could count observations, but it could not tell whether a canonical role meaning was weakly grounded or reinforced by multiple sources
- that was no longer good enough for the SOTA-quality target because the canonical layers still had richer provenance than the consumer logic was using
- `crates/specforge/src/ir/semantic.rs` now resolves a per-signal `resolved_semantic_role` from canonical observations first and only falls back to merged tags when no observation-backed consensus exists
- `InterfaceSignalRecord` now also carries `semantic_grounding_strength` so the canonical layers can distinguish:
  - `single_source`
  - `multi_source`
- grounding strength is currently derived from the count of distinct preserved observations supporting the resolved role, which keeps the model honest without pretending a single observation is stronger than it is
- handshake-role derivation now uses that canonical resolved role surface before any tag-only fallback, which means a provenance-backed role consensus outranks the older lossy tag merge
- `crates/specforge/src/commands/validate.rs` now reports:
  - `with_resolved_semantic_role`
  - `with_single_source_semantic_grounding`
  - `with_multi_source_semantic_grounding`
- regression coverage now proves:
  - single-source table/caption grounding resolves a semantic role with `single_source`
  - agreeing visual + table evidence resolves the same role with `multi_source`
  - `IntentIR` carries that new canonical surface forward unchanged
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `162/162`
- the next honest follow-up is stronger arbitration rather than more preservation:
  - weigh source kinds and automation confidence when multiple compatible observations support a role
  - distinguish repeated same-modality support from truly cross-modality reinforcement
  - reuse the new resolved-role surface when richer protocol meanings beyond ready/valid are added

## 2026-04-04 - semantic grounding strength is now modality-aware
- the previous semantic-role consensus slice still had one remaining quality issue:
  - `semantic_grounding_strength = multi_source` only meant "more than one supporting observation"
  - that overclaimed confidence because two table observations are not the same thing as table-plus-visual reinforcement
- that distinction matters for a SOTA-grade KG because downstream consumers should know whether semantic agreement is repeated within one modality or reinforced across independent evidence modalities
- `crates/specforge/src/ir/semantic.rs` now derives `semantic_grounding_strength` as:
  - `single_source`
  - `multi_source` for repeated support within the same modality family
  - `cross_modality` when support spans more than one modality family across table/prose/visual evidence
- `specforge validate` now reports:
  - `with_cross_modality_semantic_grounding`
- regression coverage now proves:
  - visual + table support upgrades a resolved role to `cross_modality`
  - repeated table-only support remains `multi_source`
  - `IntentIR` carries the stronger distinction unchanged
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `165/165`
- the next honest follow-up is stronger arbitration inside those buckets:
  - rank source kinds and automation confidence within compatible cross-modality sets
  - distinguish cross-modality agreement from cross-modality contradiction with stronger canonical arbitration metadata
  - generalize the same grounding-quality model beyond ready/valid-style semantic roles

## 2026-04-04 - canonical semantic consensus now summarizes why a role won
- after the modality-aware grounding slice, one more canonical gap was still visible:
  - `resolved_semantic_role` and `semantic_grounding_strength` told us the winner and a coarse bucket
  - but downstream consumers still had to inspect raw `semantic_observations` to learn how many observations backed that role, which source kinds contributed, and what the strongest supporting confidence was
  - resolved roles could also still exist without an explicit consensus summary if they came from older fallback carry-through
- that was not strong enough for the project quality bar because canonical consumers should not have to reverse-engineer consensus state from raw observations just to judge whether a role meaning is robust
- `crates/specforge/src/ir/semantic.rs` now defines `semantic_consensus` on `InterfaceSignalRecord`
- that summary currently carries:
  - winning role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence
- `semantic_consensus` is only present when the role is backed by preserved observations, which keeps the fallback path explicit rather than pretending every resolved role has the same quality of support
- `crates/specforge/src/commands/validate.rs` now reports:
  - `with_semantic_consensus`
  - `with_high_confidence_semantic_consensus`
  - `resolved_semantic_roles_without_consensus`
- validation now also emits an explicit info finding when resolved semantic roles still exist without a canonical consensus summary
- regression coverage now proves:
  - semantic consensus details are preserved for single-source, same-modality multi-source, and cross-modality grounding
  - `IntentIR` carries the semantic-consensus summary unchanged
  - validation flags an `IntentIR` when a resolved role still lacks consensus metadata
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `166/166`
- the next honest follow-up is stronger arbitration, not more summary:
  - weight source kinds and automation confidence inside compatible consensus sets
  - add explicit winner-vs-runner-up style arbitration metadata when compatible evidence competes in strength
  - extend the same consensus/arbitration model beyond the current ready/valid semantic-role family

## 2026-04-04 - canonical semantic candidates now preserve competing role hypotheses
- after the semantic-consensus slice, another gap was still obvious:
  - canonical consumers could inspect the winning consensus when a role resolved cleanly
  - but they still could not inspect competing role candidates without going back to raw `semantic_observations`
  - that meant unresolved role competition was visible only indirectly through conflict records, not as a first-class candidate surface on the signal itself
- that was too lossy for the project quality bar because arbitration work should happen on typed candidate profiles, not by forcing every downstream consumer to reconstruct them from raw observations
- `crates/specforge/src/ir/semantic.rs` now carries `semantic_candidates` on `InterfaceSignalRecord`
- each candidate currently records:
  - role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence
  - deterministic evidence weight
- the current deterministic evidence weight is intentionally simple and transparent:
  - source-kind prior
  - plus automation-confidence prior
  - preserved for inspection, not yet used to force unsafe winner selection across incompatible roles
- `resolved_semantic_role` / `semantic_consensus` now build from that candidate layer when exactly one role candidate survives
- when multiple role candidates exist, the canonical signal now preserves them explicitly instead of flattening the situation to only `signal_semantic_conflicts`
- `crates/specforge/src/ir/semantic.rs` now also carries `semantic_arbitration` on `InterfaceSignalRecord`
- each arbitration summary currently records:
  - candidate count
  - leading role
  - leading evidence weight
  - runner-up role and evidence weight when present
  - lead margin over the runner-up
  - decisive vs non-decisive status
- the current policy remains intentionally conservative:
  - arbitration metadata is preserved for inspection
  - multiple candidates still do not force a resolved role
  - consensus is still emitted only when exactly one candidate survives safely
- `crates/specforge/src/commands/validate.rs` now reports:
  - `semantic_candidates`
  - `with_semantic_candidates`
  - `with_multiple_semantic_candidates`
  - `with_semantic_arbitration`
  - `with_decisive_semantic_arbitration`
  - `with_non_decisive_semantic_arbitration`
- regression coverage now proves:
  - candidate details are preserved for single-source, same-modality multi-source, and cross-modality role meanings
  - arbitration details are preserved for both decisive and contested role meanings
  - conflicting ready-like vs valid-like evidence produces two canonical candidates with no resolved role or consensus
  - `IntentIR` carries those candidate profiles forward unchanged
  - validation counts signals with multiple semantic candidates explicitly
  - validation reports non-decisive semantic arbitration explicitly
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `167/167`

## 2026-04-05 - contested semantic evidence now blocks literal handshake-name fallback
- after adding canonical `semantic_arbitration`, one remaining weakness was still visible in the temporal layer:
  - `HandshakeComplete` derivation could still fall back to raw signal spelling when a signal name looked like `*VALID*` or `*READY*`
  - that meant a signal with explicit contested semantic evidence could still be coerced back into a handshake role by its spelling alone
- that was below the project quality bar because preserved semantic disagreement should outrank heuristic spelling, not the other way around
- `crates/specforge/src/ir/semantic.rs` now builds a handshake-role context instead of a bare role map
- that context still carries resolved handshake roles, but it also tracks signals whose literal name fallback must be blocked because `semantic_arbitration` is non-decisive
- the temporal handshake derivation path now uses that richer context for both antecedent parsing and conditional-rule consequent parsing
- the practical effect is:
  - resolved semantic meaning still drives handshake-role recovery
  - plain literal `VALID` / `READY` naming still works when no preserved semantic disagreement exists
  - but contested semantic-role evidence now suppresses literal handshake-name fallback instead of getting silently overridden by it
- regression coverage now proves that a contested signal like `XVALID` does not generate a typed `HandshakeComplete` predicate merely because of its spelling when preserved evidence still disagrees about whether it is valid-like or ready-like
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `168/168`

## 2026-04-05 - blocked handshake fallback is now explicit residual/validation state
- after the handshake-fallback hardening landed, one usability gap remained:
  - the temporal layer behaved more honestly, but a user still had to infer from missing `HandshakeComplete` predicates that a handshake-shaped signal had been intentionally withheld
  - that was too implicit for the project quality bar because withheld heuristic promotion should be inspectable, not just silently absent
- `crates/specforge/src/ir/semantic.rs` now emits `semantic_handshake_name_fallback_blocked` residual decisions when a signal looks handshake-shaped by name but preserved `semantic_arbitration` is still non-decisive
- `crates/specforge/src/commands/validate.rs` now reports:
  - `with_blocked_handshake_name_fallback`
  - `semantic_handshake_name_fallback_blocked_present`
  - `intent_handshake_name_fallback_blocked_present`
- that makes the system say, explicitly:
  - this signal looked like a `VALID` / `READY` candidate by name
  - preserved evidence still disagreed
  - so the heuristic promotion was intentionally withheld
- regression coverage now proves:
  - `SemanticIR` emits the new residual decision packet
  - `IntentIR` carries that packet forward
  - both semantic and intent validation report the blocked-fallback state explicitly
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `171/171`

## 2026-04-05 - fallback-only semantic roles are now explicit provisional state
- another quiet truthfulness gap remained after semantic consensus and arbitration became first-class:
  - validation could already tell us when a resolved semantic role still lacked preserved observation-backed consensus
  - but the canonical IR itself still looked more confident than it really was unless a user happened to run `specforge validate`
- that was below the project bar because provisional meaning should be visible in the artifact itself, not only in post-hoc diagnostics
- `crates/specforge/src/ir/semantic.rs` now emits a `semantic_resolved_role_without_consensus` residual decision whenever a signal still carries a resolved semantic role but no preserved `semantic_consensus`
- `crates/specforge/src/ir/intent.rs` now turns that carried residual into an explicit `assumption_semantic_role_without_consensus`, so the canonical intent layer says plainly that some role meaning is still provisional in this pass
- this keeps the truthfulness contract aligned across layers:
  - canonical role structure can still be useful
  - weaker fallback-only meaning is not hidden as if it were fully grounded
  - users and downstream tools can see the provisional status directly from the IR
- regression coverage now proves:
  - `SemanticIR` emits the residual packet for resolved roles without consensus
  - `IntentIR` carries the packet and emits the matching assumption
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `173/173`

## 2026-04-05 - provisional semantic roles no longer drive typed handshake semantics
- after making fallback-only semantic roles explicit residual/assumption state, one consumer still remained too trusting:
  - the handshake-role context could still read `resolved_semantic_role` directly even when no preserved `semantic_consensus` existed
  - that meant provisional fallback-only meaning could still shape typed `HandshakeComplete` semantics more strongly than the truthfulness contract allowed
- `crates/specforge/src/ir/semantic.rs` now hardens that path:
  - typed handshake-role recovery only trusts observation-backed `semantic_consensus`
  - fallback-only resolved roles no longer populate canonical handshake-role context by themselves
  - handshake-shaped signals with provisional fallback-only roles now also block raw name fallback, not just contested-arbitration cases
- `crates/specforge/src/commands/validate.rs` now reports that blocked fallback state for handshake-shaped provisional-role cases too, so the user can see when spelling was intentionally refused because role grounding stayed weaker than consensus
- this is the stronger semantic shape:
  - consensus-backed role meaning can drive typed handshake semantics
  - provisional fallback-only meaning remains visible but does not get promoted into stronger protocol events silently
  - spelling can still help when no richer semantic state exists, but it no longer overrides either contested or provisional role evidence
- regression coverage now proves:
  - provisional fallback-only semantic roles do not drive handshake-role context
  - handshake-shaped provisional-role signals are counted as blocked name-fallback cases during validation
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `175/175`

## 2026-04-05 - signal names no longer self-justify semantic role hints
- the previous semantic-role grounding work made one quiet shortcut more visible:
  - prose and visual semantic-hint inference was still scanning raw source text
  - that meant a declaration like `Signal AWVALID is input width 1.` could create a valid-like semantic hint from the identifier token itself, even when no descriptive language explained the role
- that was below the SOTA bar because consensus should come from meaning-bearing text, not from the signal name being embedded in a sentence
- `crates/specforge/src/ir/evidence.rs` now strips explicit signal identifiers before running semantic-role tag inference for:
  - prose `SourceFact` descriptions
  - alias-grounded prose descriptions
  - visual captions
  - VLM timing-diagram annotations
  - signal-description table rows already keep using their description cell, and now also strip the row signal token if it appears in the description text
- the practical result is:
  - declarations like `Signal AWVALID is input width 1.` no longer create semantic-role hints by themselves
  - descriptive phrases such as `request pending`, `can accept`, or `accept the transfer` still work exactly as intended
  - downstream consensus and arbitration surfaces now reflect descriptive grounding more honestly
- regression coverage now proves:
  - signal declarations with handshake-shaped identifiers alone do not create semantic hints
  - the existing table/prose/visual semantic-hint paths still work when descriptive language is present
- validation for this slice:
  - `cargo fmt --all` passed
  - `cargo test --manifest-path Cargo.toml` passed with `176/176`
