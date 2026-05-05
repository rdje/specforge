# CHANGES

## 2026-05-05 (`.fsm` FSM undriven graph output lock)

### Added: structured FSM graph-backed undriven outputs are regression-locked
- Added `structured_fsm_blocks_graph_backed_undriven_output_inventory`, the true-FSM sibling of the standalone DT graph-backed undriven-output regression.
- The test injects width-only `UNUSED_TRACE`, recovers its output role from `IntentIR.actor_ports`, and proves `.fsm` lowering blocks because no typed FSM-state action drives it.
- No production behavior changed; this locks the existing `validate_output_inventory_is_driven(...)` true-FSM branch after the previous graph-first renderability fix.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::structured_fsm_blocks_graph_backed_undriven_output_inventory -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `76` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `520` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures and `0` failures
- `git diff --check` -> passed

## 2026-05-05 (`.fsm` graph-backed undriven outputs block)

### Changed: graph-backed output inventory must be driven before DT/FSM emission
- `analyze_dt_root_renderability(...)` and `analyze_fsm_root_renderability(...)` now validate the full graph-first `FsmSignalCandidate` inventory for output roles, rather than only checking outputs that were already pulled into renderable `+size` entries.
- Added `standalone_dt_blocks_graph_backed_undriven_output_inventory`, proving a width-only `UNUSED_OUT` whose output role comes from `IntentIR.actor_ports` blocks lowering when no typed control action drives it.
- Kept top-linked child module endpoints scoped to composition validation, so `module_topology_link` outputs still produce the precise "unemitted child port" diagnostic when top links reference a child port that no child module emitted.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::standalone_dt_blocks_graph_backed_undriven_output_inventory -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `75` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `519` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures and `0` failures
- `git diff --check` -> passed

## 2026-05-05 (README/COMMIT bootstrap refresh)

### Changed: live bootstrap and continuity docs re-executed
- Re-read `README.md` as the project entrypoint, followed `SESSION_BOOTSTRAP.md` through the referenced root docs, mdBook chapters, corpus-KB surfaces, FSMGEN feedback, `COMMIT.md`, and direct Rust crate/module seams.
- Confirmed the active steering remains unchanged: `IntentIR` is canonical, `.fsm` stays downstream, and the next implementation pressure remains graph-first actor-relative direction consumers rather than backend expansion.
- Refreshed continuity docs so the latest committed baseline is `9b425e9` and the previous mixed DT/FSM top child-root slice is no longer described as in flight.

### Validation
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `518` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures and `0` failures
- `git diff --check` -> passed

## 2026-04-30 (`.fsm` mixed child root order stays stable)

### Added: mixed DT/FSM top child roots are regression-locked
- Added a renderable top-composition regression where one top instantiates a DT child followed by a structured FSM child.
- The test proves `FsmRenderableTopRoot.children` preserves child order and root kind as `producer`/`?dtc` then `controller`/`?fsmc`.
- The same fixture proves `renderable_document.direct_roots` preserves direct-root order and kind as `producer_core`/`?dt:name` then `controller_core`/`?fsm:name`, with final emitted text matching that order.
- No production behavior changed; this locks mixed child-root ordering across `renderable_modules_for_top(...)` and source-document text emission.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::renderable_top_document_preserves_mixed_child_root_order_and_kind -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `74` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `518` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` reused FSM child modules share renderable roots)

### Added: reused FSM child modules de-duplicate renderable direct roots
- Added a renderable top-composition regression where two child instances both use the same structured FSM child module.
- The test proves both `?fsmc` child instances remain in the renderable top root while `renderable_document.direct_roots` emits the shared `controller_core` `?fsm:name` module exactly once.
- The fixture checks the shared FSM direct root keeps state content and system-contract content, so the de-duplication proof covers real structured-FSM payload rather than only child-reference text.
- No production behavior changed; this locks the existing `renderable_modules_for_top(...)` de-duplication path for FSM child roots.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::renderable_top_document_deduplicates_reused_fsm_child_roots -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `73` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `517` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` top documents preserve FSM child roots)

### Added: top composition with FSM child roots is regression-locked
- Added a renderable top-composition regression where the top instantiates a structured FSM child module.
- The test proves `FsmTopChildCandidate`, `FsmRenderableTopRoot.children`, and `renderable_document.direct_roots` all preserve `FsmRootKind::Fsm` for the child module.
- The emitted `.fsm` text now has direct coverage for `(?fsmc:controller controller_core)` plus the child `(?fsm:controller_core ...)` direct root and its state/system content.
- No production behavior changed; this locks an already-supported mixed `?top` + `?fsm` child artifact boundary.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::renderable_top_document_preserves_fsm_child_root_kind -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `72` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `516` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` top source documents emit stable root order)

### Added: renderable top source-document emission order is regression-locked
- Added a top-composition regression proving emitted `.fsm` source documents keep the `?top:name` root before child direct module roots.
- The test also locks child direct-root emission order for the existing `producer_core` then `consumer_core` top-composition fixture.
- No production behavior changed; this protects `render_fsm_source_document(...)` after the recent direct-root presence and de-duplication regressions.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::renderable_top_document_emits_top_before_child_direct_roots -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `71` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `515` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` reused child modules share renderable roots)

### Added: reused top child modules de-duplicate renderable direct roots
- Added a top-composition regression with two child instances that both use the same `stage_core` module.
- The test proves the renderable top root preserves both child instances while `renderable_document.direct_roots` emits the shared `stage_core` module exactly once.
- The same fixture asserts the shared direct root stays `?dt:name` and keeps the child module size-entry surfaces consumed by the top links.
- No production behavior changed; this locks the existing `renderable_modules_for_top(...)` de-duplication path.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::renderable_top_document_deduplicates_reused_child_module_roots -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `70` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `514` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` renderable top direct roots keep child modules)

### Added: renderable source-document child roots are regression-locked
- Extended the main renderable top-composition regression to assert `renderable_document.direct_roots` contains the child module roots referenced by the top candidate.
- The test now checks direct-root order, module names, root kinds, and emitted child module size-entry surfaces for `producer_core` and `consumer_core`.
- This protects `renderable_modules_for_top(...)`, which de-duplicates top children by source module and copies each renderable child module into the final source-document model before `.fsm` text emission.
- No production behavior changed; this locks the existing renderable document model rather than broadening top-composition lowering.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::builds_renderable_top_composition_fsm_adapter_artifact -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `69` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `513` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` renderable top links keep topology provenance)

### Added: renderable top-root link provenance is regression-locked
- Extended the topology-link root-kind confidence regression to assert the child-to-child `FsmTopCandidate.links` entry keeps explicit top-link support IDs and high automation confidence.
- The same test now checks `FsmRenderableTopRoot.links`, proving the final renderable top document carries the topology evidence that drives `(?toplink:wiring ...)` emission.
- The fixture still keeps public top-port and child declaration confidence low, so this locks link evidence without relying on recovered top-boundary or child-declaration confidence.
- No production behavior changed; top renderability already clones explicit top links into the renderable top root.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_root_kind_confidence_follows_top_link_evidence -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `69` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `513` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` top root-kind confidence follows topology links)

### Added: top-link confidence now feeds root-kind confidence coverage
- Added a focused top-composition regression proving `build_top_root_kind_decision(...)` folds high-confidence explicit top-link evidence into the selected `?top:name` root-kind decision.
- The fixture intentionally keeps the public top port and both child declarations at low confidence while the only child-to-child link stays high confidence, isolating the root-kind confidence source to `FsmTopCandidate.links`.
- The link connects child endpoints only, so no recovered top-boundary port provenance can accidentally raise the public top-port confidence and mask the path being tested.
- No production behavior changed; this completes direct regression coverage for the three confidence lanes folded by `build_top_root_kind_decision(...)`: ports, children, and links.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_root_kind_confidence_follows_top_link_evidence -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `69` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `513` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` top root-kind confidence follows child declarations)

### Added: top child declaration confidence now feeds root-kind confidence coverage
- Added a focused top-composition regression proving `build_top_root_kind_decision(...)` folds high-confidence child declaration evidence into the selected `?top:name` root-kind decision.
- The fixture intentionally downgrades the explicit public top port to low confidence, keeps the top child declaration high confidence, and has no links, so the resulting high root-kind confidence is isolated to `FsmTopChildCandidate` evidence.
- The test also asserts the child resolves to the renderable `?dt:controller_core` kind while the recovered top port remains low confidence, guarding against accidental confidence leakage from unrelated top-boundary evidence.
- No production behavior changed; this locks the existing confidence fold path after the adjacent top-child provenance and recovered top-port confidence regressions.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_root_kind_confidence_follows_child_declaration_evidence -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `68` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `512` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` top child declarations keep provenance)

### Added: top child declaration provenance is regression-locked
- Extended the top system-contract distribution regression to assert the `controller` top-child candidate keeps child declaration support IDs and high automation confidence.
- The test now also proves the child resolves to `?dt:controller_core` and that the renderable top child keeps the same resolved DT kind.
- This complements the child endpoint and renderable-module system-contract locks by protecting the top-child declaration evidence that anchors the child module before link recovery and emitted child roots consume it.
- No production behavior changed; `build_top_candidate(...)` already copied top-child support IDs, confidence, and resolved child root kind into `FsmTopChildCandidate`.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` child renderable system contracts keep provenance)

### Added: child renderable system-contract provenance is regression-locked
- Extended the top system-contract distribution regression to assert the child `controller_core` renderable module keeps system-contract support IDs and high automation confidence.
- The same test now asserts the renderable source-document child direct root keeps that contract provenance, covering the emitted child module boundary inside a top-composition artifact.
- This complements the child signal-inventory endpoint lock: child `clk` / `rst_n` entries and the renderable child module system contract now both preserve the evidence that makes the distributed system block safe to render.
- No production behavior changed; explicit module renderability already cloned the canonical child `SystemContractRecord` into renderable modules.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` standalone renderable system contracts keep provenance)

### Added: standalone renderable system-contract provenance is regression-locked
- Extended the standalone sequential and explicit-module system-contract recovery regressions to assert `FsmRenderableModule.system_contract` keeps system-contract support IDs and high automation confidence.
- The same tests now assert the renderable source-document direct-root module carries that contract provenance too, covering the final artifact boundary used by `.fsm` text emission.
- This complements the standalone signal-inventory lock: recovered/materialized `clk` / `rst_n` entries and the renderable module system contract now both preserve the evidence that created the system block.
- No production behavior changed; renderability already cloned the canonical `SystemContractRecord` into renderable modules.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge system_signals_from_system_contract -- --nocapture` -> passed with `3` focused system-contract tests
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` standalone system signals keep contract provenance)

### Added: standalone system-contract signal provenance is regression-locked
- Extended the standalone sequential and explicit-module system-contract recovery regressions to assert recovered/materialized `clk` / `rst_n` inventory entries carry system-contract support IDs and high automation confidence.
- The direct sequential tests now cover both shape recovery and materialization when flat signal records are missing; the explicit-module test covers module-local system-contract recovery.
- This complements the child top-composition endpoint lock by protecting the direct standalone consumers of `overlay_system_contract_signal(...)` before renderability checks or emitted `.fsm` system blocks consume them.
- No production behavior changed; `overlay_system_contract_signal(...)` already copied system-contract support IDs and confidence into the standalone signal inventory.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge system_signals_from_system_contract -- --nocapture` -> passed with `3` focused system-contract tests
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` child system endpoints keep contract provenance)

### Added: child system-contract endpoint provenance is regression-locked
- Extended the top system-contract distribution regression to assert child module `controller_core.clk` / `rst_n` inventory entries carry system-contract support IDs and high automation confidence.
- The test already proved those child clock/reset endpoints were materialized as input, 1-bit `system_contract_signal` ports; it now also proves the emitted child endpoint surface keeps the contract evidence that created them.
- This complements the resolved top-port, selected top-inventory, and renderable top-root system-port locks by protecting the child endpoint provenance that top-link width recovery consumes.
- No production behavior changed; `overlay_system_contract_signal(...)` already copied system-contract support IDs and confidence into the child module signal inventory.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` resolved top ports keep system-port width provenance)

### Added: child-system-contract resolved top-port widths are regression-locked
- Extended the top system-contract distribution regression to assert `FsmTopCandidate.ports` carries recovered 1-bit width provenance for public `clk` / `rst_n` ports.
- The test now proves the resolved top-candidate port entries retain top-link supporting IDs and high automation confidence for widths recovered from child system-contract endpoints.
- This completes the immediate artifact-surface trio for child-system-contract top-width recovery: resolved top ports, selected top inventory, and final renderable top-root ports now all expose the same recovered provenance.
- No production behavior changed; the existing resolved-port provenance merge already carried this evidence.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` selected top inventory keeps system-port width provenance)

### Added: child-system-contract selected top-port widths are regression-locked
- Extended the top system-contract distribution regression to assert selected top `fsm.signal_inventory` carries recovered 1-bit width provenance for public `clk` / `rst_n` ports.
- The test now proves the selected top signal entries retain `module_topology_link`, top-link supporting IDs, and high automation confidence for widths recovered from child system-contract endpoints.
- This complements the renderable top-root system-port lock: the selected top inventory and final renderable document now both expose the same child-system-contract width provenance.
- No production behavior changed; the existing selected top inventory merge already carried this evidence.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` renderable top-root ports keep system-port width provenance)

### Added: child-system-contract renderable top-port widths are regression-locked
- Extended the top system-contract distribution regression to assert `renderable_document.top_root.ports` carries recovered 1-bit widths, supporting IDs, and automation confidence for public `clk` / `rst_n` ports.
- This closes the child-system-contract endpoint sibling of the renderable top-root width provenance surface: clock/reset widths recovered through explicit top links now remain visible at the final renderable-document boundary.
- The test proves high-confidence support from the `clk -> controller.clk` and `rst_n -> controller.rst_n` top links survives into the renderable `?top:soc` document model before text emission.
- No production behavior changed; the prior resolved-port provenance merge already feeds the renderable top-root port vector.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_system_port_widths_from_child_system_contract -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` renderable top-root ports keep actor-port width provenance)

### Added: actor-port-recovered renderable top-port widths are regression-locked
- Extended the top actor-port width recovery regression to assert `renderable_document.top_root.ports` carries recovered width, supporting IDs, and automation confidence.
- This closes the actor-port width sibling of the prior actor-port direction renderable lock: matching top actor-port evidence now proves both direction and width provenance at the final renderable-document boundary.
- The test proves a recovered `graph_wrapper_ext_data` support ID and high confidence survive into the renderable `?top:wrapper` document model before text emission.
- No production behavior changed; the prior resolved-port provenance merge already feeds the renderable top-root port vector.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_width_from_actor_ports -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` renderable top-root ports keep top-link width provenance)

### Added: topology-recovered renderable top-port widths are regression-locked
- Extended the child-link top-port width recovery regression to assert `renderable_document.top_root.ports` carries recovered width, supporting IDs, and automation confidence.
- This closes the width sibling of the prior top-link direction renderable lock: explicit top-link topology now proves both direction and width provenance at the final renderable-document boundary.
- The test proves high-confidence support from the `producer.output_data -> result_data` top link survives into the final `?top:datapath` document model before text emission.
- No production behavior changed; the prior resolved-port provenance merge already feeds the renderable top-root port vector.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_width_from_child_link_topology -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-30 (`.fsm` renderable top-root ports keep top-link provenance)

### Added: topology-recovered renderable top ports are regression-locked
- Extended the explicit top-link direction recovery regression to assert `renderable_document.top_root.ports` carries recovered direction, supporting IDs, and automation confidence.
- This closes the sibling artifact-surface proof after the actor-port renderable top-root lock: both actor-port graph evidence and explicit top-link topology evidence now have direct renderable-document coverage.
- The test proves high-confidence `module_topology_link` support from the `consumer.result_data -> result_data` top link survives into the final `?top:datapath` document model before text emission.
- No production behavior changed; the prior resolved-port provenance merge already feeds the renderable top-root port vector.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_direction_from_link_topology -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` renderable top-root ports keep recovered provenance)

### Added: renderable top-root port provenance is regression-locked
- Extended the top actor-port direction recovery regression to assert `renderable_document.top_root.ports` carries recovered direction, supporting IDs, and automation confidence.
- This locks the final renderable top-root artifact surface, not only `FsmTopCandidate.ports` and selected signal inventory.
- The test proves a recovered `graph_wrapper_ext_data` support ID and high confidence survive into the renderable `?top:name` document model before text emission.
- No production behavior changed; the prior resolved-port provenance merge already feeds this surface.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_direction_from_actor_ports -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` top-root decision confidence follows recovery)

### Added: root-kind confidence regression now isolates recovered top-port evidence
- Added a focused top-composition regression where the only explicit top port starts with low automation confidence and actor-port graph recovery supplies high-confidence direction evidence.
- The test proves the selected `?top:name` root-kind decision folds the recovered top-port confidence through `FsmTopCandidate.ports`.
- The scenario intentionally avoids child/link confidence so the root decision cannot pass by borrowing confidence from unrelated top-composition records.
- This locks the resolved-port provenance behavior from the previous slice at its root-kind decision consumer.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_root_kind_confidence_follows_recovered_top_port_evidence -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `67` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `511` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` resolved top ports keep recovered provenance)

### Fixed: recovered top-boundary evidence now reaches resolved top-port records
- `FsmTopCandidate.ports` and `renderable_top.ports` now merge recovered top-boundary supporting IDs and automation confidence from actor-port graph and explicit top-link topology evidence.
- The resolved top-port provenance merge reuses the same direction and width evidence ledgers that feed selected top signal inventory.
- Actor-port direction/width recovery now leaves graph support IDs and high confidence visible on the recovered top port itself.
- Top-link direction and child-endpoint width recovery now leave explicit top-link support IDs and high confidence visible on the recovered top port itself.
- Renderability and emitted `.fsm` text are unchanged; this is an adapter-artifact honesty fix for the top-candidate port surface.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_direction_from_actor_ports -- --exact --nocapture` -> failed before the fix because the recovered top port missed `graph_wrapper_ext_data`, then passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_direction_from_link_topology -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_width_from_actor_ports -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_width_from_child_link_topology -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `66` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` selected top confidence follows recovered evidence)

### Fixed: recovered top-boundary evidence now raises selected inventory confidence
- Selected `.fsm` top signal inventory now folds automation confidence from recovered top-boundary direction and width evidence.
- Top actor-port direction/width recovery carries actor-port confidence into the selected `FsmSignalCandidate`.
- Top-link direction and child-endpoint width recovery carry explicit top-link confidence into the selected `FsmSignalCandidate`.
- The confidence merge follows the same max-confidence policy used elsewhere in adapter inventory, so renderability and emitted `.fsm` text remain unchanged while selected artifacts no longer understate recovered high-confidence evidence.

### Validation
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_direction_from_actor_ports -- --exact --nocapture` -> failed before the fix with selected inventory confidence `Low` instead of `High`, then passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_direction_from_link_topology -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_width_from_actor_ports -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests::top_composition_recovers_top_port_width_from_child_link_topology -- --exact --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml -p specforge ir::adapters::tests -- --nocapture` -> passed with `66` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` selected top support IDs follow recovered evidence)

### Fixed: recovered top-boundary evidence now carries its supporting canonical IDs
- Selected `.fsm` top signal inventory now merges supporting canonical IDs from recovered top-boundary direction and width evidence.
- Top actor-port direction/width recovery carries the actor-port support ID into the selected `FsmSignalCandidate`.
- Top-link direction and child-endpoint width recovery carry the explicit top-link support ID into the selected `FsmSignalCandidate`.
- Renderability and emitted `.fsm` text are unchanged; selected artifacts now preserve both source category and replayable support IDs for recovered top evidence.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_actor_ports -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_width_from_child_link_topology -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_width_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_preserves_recovered_top_port_direction_when_still_blocked -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_link_topology -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `27` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `66` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` selected top direction provenance stays visible)

### Fixed: recovered top-boundary direction sources now stay visible in selected inventory
- Selected `.fsm` top signal inventory now carries recovered direction source categories from top actor-port and top-link topology evidence.
- Top actor-port direction recovery records `actor_port` on the selected `FsmSignalCandidate`.
- Top-link direction recovery records `module_topology_link` even when the top remains blocked before child endpoint width recovery can run.
- Renderability and emitted `.fsm` text are unchanged; the selected artifact now distinguishes graph-backed direction provenance the same way it already does for recovered widths.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_actor_ports -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_preserves_recovered_top_port_direction_when_still_blocked -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_link_topology -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `27` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `66` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` selected top width provenance stays visible)

### Fixed: recovered top-boundary width sources no longer collapse to plain top-port provenance
- Selected `.fsm` top signal inventory now carries recovered width source categories from top actor-port and top-link topology evidence.
- Top actor-port width recovery records `actor_port_width` on the selected `FsmSignalCandidate`.
- Child-link/topology width recovery records `module_topology_link` on the selected `FsmSignalCandidate`.
- Renderability and emitted `.fsm` text are unchanged; this makes blocked/renderable artifacts more honest for downstream debugging and future adapters.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_width_from_actor_ports -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_width_from_child_link_topology -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `27` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `66` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` top system-contract distribution is regression-locked)

### Added: top links now prove child system-contract ports remain usable endpoints
- Added a top-composition regression where widthless public `clk` / `rst_n` top ports drive a child module whose clock/reset ports exist only through its canonical `SystemContractRecord`.
- The test proves top-link width recovery pulls `1`-bit evidence from the child system-contract endpoints into selected top public IO.
- The child module inventory is also checked for materialized `system_contract_signal` provenance, locking the consumer path opened by the prior system-contract materialization slice.
- The emitted `.fsm` text now has coverage for bare 1-bit input top ports plus `/clk/controller.clk/` and `/rst_n/controller.rst_n/` wiring into the child system block.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_system_port_widths_from_child_system_contract -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `27` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `66` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `510` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` system contracts materialize clock/reset signals)

### Fixed: canonical system contracts can now supply clock/reset inventory entries
- `.fsm` system-contract overlays no longer require clock/reset signals to already exist in direct/module signal inventory before registering them.
- A canonical `SystemContractRecord` now materializes its clock and reset as input, 1-bit `system_contract_signal` evidence when the flat interface surface lacks those entries.
- Existing conflict behavior remains intact: contradictory flat direction or width evidence still poisons the merged clock/reset signal and blocks lowering.
- A regression deletes the flat `clk` / `rst_n` records from a sequential `IntentIR` while keeping the system contract, then proves the adapter emits the `(+system ...)` block and keeps the materialized signals in `fsm.signal_inventory`.

### Validation
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_materializes_system_signals_from_system_contract -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_recovers_system_signals_from_system_contract -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_blocks_conflicting_system_contract_signal_direction -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_blocks_conflicting_system_contract_signal_width -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_explicit_module_recovers_system_signals_from_system_contract -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `65` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `509` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` actor-port parametric widths stay explicit)

### Fixed: symbolic actor-port widths no longer degrade to missing width diagnostics
- Actor-port overlays now preserve `WidthHint::Parametric(...)` as `.fsm` signal-inventory `parametric_width_hint` when no numeric width evidence is already known.
- Canonical signal parametric widths remain strict while graph-backed actor-port parametric widths are recovery evidence, so explicit numeric widths stay authoritative.
- `.fsm` renderability now reports symbolic actor-port width evidence explicitly instead of falling through to the generic missing numeric-width blocker.
- A regression proves a widthless `DATA_IN` with actor-port `DATA_WIDTH` blocks with a parametric-width diagnostic, while a companion guard proves explicit numeric `DATA_IN width 16` is not poisoned by a symbolic actor-port width.

### Validation
- `cargo test --manifest-path Cargo.toml standalone_dt_blocks_parametric_actor_port_width_with_diagnostic -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml standalone_dt_keeps_explicit_numeric_width_over_actor_parametric_width -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_blocks_parametric_signal_width_with_diagnostic -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_recovers_control_input_width_from_actor_port_graph -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_explicit_module_recovers_control_input_width_from_actor_port_graph -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_width_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `64` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `508` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` parametric signal widths stay explicit)

### Fixed: non-top parametric widths no longer look like missing width evidence
- `FsmSignalCandidate` now preserves `parametric_width_hint` for canonical direct/module signal inventory entries whose width is symbolic.
- `.fsm` renderability now blocks parametric signal widths with a specific diagnostic before falling back to generic missing-width guidance.
- Numeric graph recovery can no longer silently mask a symbolic canonical width for the active `.fsm` slice, matching the conservative top-public-IO width policy.
- Selected top signal inventory now also carries parametric width text when a top port is blocked for symbolic public IO width evidence.

### Validation
- `cargo test --manifest-path Cargo.toml standalone_dt_blocks_parametric_signal_width_with_diagnostic -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_parametric_top_port_width_for_fsm_public_io -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `62` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `506` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` flat direction conflicts stay blocking)

### Fixed: graph recovery no longer overrides contradictory canonical direction hints
- `FsmSignalCandidate` now preserves `direction_hint_conflicted`, matching the existing graph-direction and width conflict provenance fields.
- `.fsm` renderability now blocks render-critical signals with conflicting flat canonical direction evidence before considering graph-backed recovery hints.
- System-contract clock/reset checks now report conflicting canonical direction evidence distinctly instead of calling it a missing direction hint.
- A regression now proves an unambiguous actor-port graph cannot make `DATA_OUT` renderable when duplicate canonical signal declarations disagree on its flat direction.

### Validation
- `cargo test --manifest-path Cargo.toml standalone_dt_blocks_conflicting_flat_direction_even_with_actor_graph -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_blocks_conflicting_system_contract_signal_direction -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `61` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `505` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` top links require emitted child ports)

### Fixed: top wiring can no longer target advisory-only child inventory entries
- Explicit top-link endpoint resolution now exposes child ports from the rendered child module surface (`+size` entries plus system-contract ports), not from the broader advisory signal inventory.
- A child signal that is declared in canonical inventory but never emitted by the child `.fsm` module now blocks top lowering instead of producing a `?toplink` reference to a phantom child port.
- Missing top-link diagnostics now distinguish absent explicit top ports from child endpoints that do not resolve to emitted child ports on a renderable child module.
- Existing child-link recovery remains intact because topology-recovered child ports that are used by rendered control still appear in the child module's emitted size surface.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_blocks_link_to_unemitted_child_port -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `26` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `60` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `504` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` top public IO widths must be explicit numeric evidence)

### Fixed: top ports no longer silently degrade missing widths to implicit 1-bit syntax
- Explicit top-root renderability now validates every public top port after actor-port and top-link width recovery have run.
- Missing top-boundary width evidence now blocks `.fsm` emission instead of letting `render_top_port_token(...)` print an implicit 1-bit port.
- Parametric top-boundary widths also block the active `.fsm` slice, matching the adapter-wide numeric-width-only rendering contract.
- Existing recovery lanes stay intact: explicit numeric top widths, matching top actor-port widths, and child-link topology widths still make top public IO renderable.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition_blocks_widthless_top_port_without_width_recovery -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_parametric_top_port_width_for_fsm_public_io -- --nocapture` -> failed before the fix, then passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `25` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `59` adapter tests
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `503` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` width conflicts get explicit artifact provenance and diagnostics)

### Improved: conflicting width evidence no longer looks like a missing width
- `FsmSignalCandidate` now preserves `width_hint_conflicted` in serialized adapter artifacts, matching the existing graph-direction conflict bit.
- Direct roots, explicit module roots, child-link topology overlays, actor-port width overlays, and system-contract clock/reset checks now report conflicting width evidence separately from genuinely absent width hints.
- Selected top signal inventory also carries the width-conflict bit when top-boundary width evidence is poisoned by duplicate declarations, top actor ports, or child-link topology.
- Rendering remains conservative: conflicted widths still block `.fsm` output and no fallback width is guessed.

### Validation
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_blocks_conflicting_control_input_actor_port_widths -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_keeps_conflicting_actor_port_width_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_explicit_module_blocks_conflicting_control_input_actor_port_widths -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_blocks_conflicting_system_contract_signal_width -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_top_port_widths_from_child_links -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_child_topology_widths -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `57` adapter tests
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `501` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` graph-direction conflicts get explicit renderability diagnostics)

### Improved: graph conflicts no longer look like missing direction hints
- `.fsm` renderability now reports conflicting graph-backed direction evidence separately from genuinely absent direction evidence.
- Direct roots, explicit module roots, child-link topology recovery, top actor-port recovery, and standalone system-contract checks keep the same conservative blocking behavior but now point at the graph conflict root cause.
- Top-boundary analysis now also emits a conflict-specific blocker when combined explicit/topology direction evidence is poisoned, instead of adding only the generic missing top-port direction message.
- Regression coverage now asserts that graph-conflicted inventory entries keep `graph_direction_hint_conflicted = true` and surface conflict wording in renderability blockers.

### Validation
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_keeps_conflicting_actor_port_direction_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_explicit_module_blocks_conflicting_module_control_read_direction -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_child_link_topology_directions -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_actor_port_directions -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `56` adapter tests
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `500` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` top signal inventory keeps graph conflicts sticky)

### Fixed: selected top inventory now preserves explicit direction while marking conflicting graph evidence
- Selected `.fsm` top signal inventories now keep explicit top-port declarations in flat `direction_hint` even when top-link topology or matching top actor-port graph evidence disagrees.
- Conflicting graph/topology recovery is now exposed through `graph_direction_hint_conflicted = true` with no graph hint, matching the module/direct inventory conflict contract.
- Renderability remains conservative: the recovered top candidate port direction still collapses to `None` and no `.fsm` target text is emitted until the top-boundary conflict is resolved.
- Duplicate explicit top declaration conflicts remain separate from graph conflicts and still collapse the flat direction evidence to unresolved.

### Validation
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_top_actor_port_direction -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_keeps_conflicting_top_port_direction_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `23` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `56` adapter tests
- `cargo fmt --manifest-path Cargo.toml -- --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `500` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (`.fsm` top signal inventory preserves graph-backed direction provenance)

### Improved: recovered top-boundary directions no longer flatten into compatibility hints
- Selected `.fsm` top surfaces now keep top-port directions recovered from explicit top-link topology or matching top actor ports in `graph_direction_hint` instead of writing them back as flat `direction_hint` values in `fsm.signal_inventory`.
- Renderable top ports still carry the resolved direction needed to emit `.fsm` top port tokens, so generated top output remains unchanged.
- Explicit top-port declarations still remain compatibility `direction_hint` evidence; only directions recovered from graph/topology context are projected as graph-backed inventory evidence.
- Blocked top candidates also preserve the recovered graph-backed direction in selected signal inventory, which keeps debugging context without pretending the flat top declaration was complete.

### Validation
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_link_topology -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_top_port_direction_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_preserves_recovered_top_port_direction_when_still_blocked -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with `23` top-composition tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `56` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `500` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (Semantic compat-direction lag is graph-scoped)

### Fixed: semantic validation no longer treats graph-resolved directions as incomplete
- `specforge validate` now mirrors the graph-first direction contract at the `SemanticIR` compatibility surface.
- Graph-resolved signals whose flat compatibility `direction_hint` is absent now report `semantic_compat_direction_hints_lag_graph`, matching the actual state: actor-relative graph truth exists, and only the flat compatibility projection is lagging.
- `semantic_compat_direction_hints_incomplete` remains available for signals that lack both a flat compatibility hint and non-conflicted actor-relative graph coverage, so genuinely unresolved direction evidence is still visible.
- Updated the tracked `compat_direction_hints_lag_graph_negative` fixture and the `kg-bench` fixture-seed test expectations to lock the split.

### Validation
- `cargo fmt --manifest-path Cargo.toml` -> passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_reports_graph_backed_compat_direction_lag_related_ids -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_keeps_incomplete_direction_finding_without_graph_coverage -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_scores_direction_from_graph_before_compat_hints -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality compat_direction_hints_lag_graph_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `500` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures

## 2026-04-29 (README bootstrap and corpus-KB fixture projection refresh)

### Updated: live continuity now matches the executable fixture baseline
- Re-ran the README handoff path through `SESSION_BOOTSTRAP.md`, the mdBook, corpus-KB pages, FSMGEN feedback, and the active Rust codebase surfaces.
- Refreshed managed corpus-KB fixture projections from the actual `kg-bench` run so the benchmark page now reports `127/127` passing fixtures instead of the stale `92/92` projection.
- Updated continuity analysis to reflect the current `31` Rust source files, `77,934` Rust source lines, `499` Rust tests, and latest committed baseline `d4f53bb`.
- No production Rust behavior changed in this slice.

### Validation
- `rustc --version` -> `rustc 1.95.0`
- `cargo --version` -> `cargo 1.95.0`
- `cargo run --manifest-path Cargo.toml -p specforge -- --help` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- inspect README.md` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- ingest README.md` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- evidence generated/source_ir/readme/source_ir.json` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- validate generated/intent_ir/readme/intent_ir.json` -> passed with the expected README-as-source `30/100 NEEDS IMPROVEMENT` artifact-quality result
- `cargo run --manifest-path Cargo.toml -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm` -> passed and blocked honestly with residuals
- `bash scripts/run_docs_ci.sh` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench` -> passed with `127` fixtures
- `bash scripts/run_ci.sh` -> passed with `499` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation

## 2026-04-29 (`.fsm` child port width now recovers through transitive top-link topology)

### Improved: explicit top-link widths now close over the endpoint graph
- Explicit top-link topology now computes a fixed-point numeric width closure across top ports and child module endpoints before module topology evidence is emitted.
- A top-boundary width recovered into one child endpoint can now flow through a sibling child link and recover another child endpoint in the same explicit top composition.
- Declared endpoint widths remain authoritative, while peer-link compatibility evidence is still fed into child module inventories so contradictory child/top widths collapse to unresolved instead of being silently accepted.
- Added a live-status tracker row for fixed-point explicit top-link width propagation.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_child_width_through_transitive_topology` first failed before the fix, then passed
- `cargo test -p specforge top_composition` -> passed with `23` topology tests
- `cargo test -p specforge ir::adapters::tests` -> passed with `56` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `499` Rust tests plus warning-deny Clippy/rustdoc and mdBook validation
- `cargo fmt --all -- --check` -> passed
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` sibling child-link source width coverage and SourceIR test isolation)

### Hardened: sibling child-link width recovery is now locked in both directions
- Added focused adapter coverage for recovering a widthless source child output from the connected target child input's numeric width.
- This complements the existing target-child recovery case and proves child-to-child topology width evidence is direction-symmetric across explicit top links.
- While rerunning full CI, fixed a real SourceIR/Docling test-isolation bug: PDF materialization tests now use the shared environment mutex instead of a private SourceIR-only lock, preventing parallel PATH mutations from hiding shell tools used by the stub backend.
- No production lowering behavior or live status row changed in this slice.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_source_child_width_from_sibling_child_link_topology` -> passed on first run
- `cargo test -p specforge ir::adapters::tests` -> passed with `55` adapter tests
- `bash scripts/run_ci.sh` first exposed the SourceIR/Docling PATH-isolation race, then the focused SourceIR and Docling runtime tests passed after the shared-lock fix
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed on rerun with `498` Rust tests plus warning-deny rustdoc and mdBook validation
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` child port width now recovers from sibling child links)

### Improved: child-to-child top links now propagate numeric endpoint width
- Explicit top-link topology can now recover a missing child module port width from the opposite child endpoint when both endpoints are child instances.
- The recovery uses the already-declared explicit module signal widths, so it does not depend on top-boundary ports and does not invent undeclared module signals.
- Conflicting sibling child endpoint widths for the same missing child port still collapse the resolved width to unresolved and keep top/module lowering blocked.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_child_width_from_sibling_child_link_topology` first failed before the fix, then passed
- `cargo test -p specforge top_composition_blocks_conflicting_sibling_child_link_widths` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed with `54` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `497` Rust tests plus warning-deny rustdoc and mdBook validation
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` top port width now recovers from child-link topology)

### Improved: child endpoint widths now carry back to top boundary ports
- Explicit top-root `.fsm` candidates can now recover a missing numeric width for a top boundary port from the connected child module endpoint in an explicit top link.
- The recovery is the mirror of child-port width recovery: width evidence can flow across a top link in either direction, but only when the opposite endpoint is already declared and renderable.
- Conflicting child endpoint widths for the same widthless top port still collapse the top-port width to unresolved and block top lowering.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_top_port_width_from_child_link_topology` first failed before the fix, then passed
- `cargo test -p specforge top_composition_blocks_conflicting_top_port_widths_from_child_links` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed with `52` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `495` Rust tests plus warning-deny rustdoc and mdBook validation
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` module control-input width now recovers from actor graph)

### Improved: explicit modules now share actor-port width evidence without importing external direction
- Explicit-module `.fsm` candidates can now recover a missing numeric width for module-local control-read inputs from existing `IntentIR.actor_ports` width evidence.
- The recovery reuses the width-only `actor_port_width` category, so external actor ports can contribute signal shape without defining the module actor's direction perspective.
- Conflicting actor-port widths for the same module input still collapse the resolved width to unresolved and keep module/top lowering blocked.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge standalone_explicit_module_recovers_control_input_width_from_actor_port_graph` first failed before the fix, then passed
- `cargo test -p specforge standalone_explicit_module_blocks_conflicting_control_input_actor_port_widths` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `493` Rust tests plus warning-deny rustdoc and mdBook validation
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` direct control-input width now recovers from actor graph)

### Improved: actor-port width evidence can now complete graph-backed direct control inputs
- Direct `.fsm` roots can now recover a missing numeric width for a control-read input from existing `IntentIR.actor_ports` width evidence, even when that actor is not the selected target actor for direction.
- The recovery uses a dedicated `actor_port_width` inventory category so external actor ports can contribute signal shape without importing their actor-relative direction into the selected root perspective.
- Conflicting actor-port widths for the same direct signal still collapse the resolved width to unresolved and keep lowering blocked.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge standalone_dt_recovers_control_input_width_from_actor_port_graph` first failed before the fix, then passed
- `cargo test -p specforge standalone_dt_blocks_conflicting_control_input_actor_port_widths` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` child port width now recovers from top-link topology)

### Improved: top-link topology now carries top-boundary width into child module ports
- Child module ports can now recover a missing numeric width from an explicit top link when the opposite top-boundary port already declares that width.
- The recovery stays bounded to existing child module ports and top-boundary numeric widths; it does not invent child signals or widen backend syntax.
- Contradictory explicit child width versus top-link width still blocks lowering by collapsing the resolved child width to unresolved instead of choosing a convenient side.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_child_width_from_top_link_topology` -> passed
- `cargo test -p specforge top_composition_blocks_conflicting_child_topology_widths` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed
- `bash scripts/run_ci.sh` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` top boundary width now recovers from top actor graph)

### Improved: top actor-port graph shape now preserves top-boundary width too
- Matching top actor ports now merge both direction and width evidence into explicit top-root boundary analysis.
- A direction-only top port can now render with the graph-backed actor-port width instead of silently degrading to an implicit 1-bit `.fsm` public port.
- Conflicting explicit top-port width versus top actor-port graph width remains blocking: the adapter collapses the resolved top width to unresolved instead of choosing a convenient side.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_top_port_width_from_actor_ports` -> passed
- `cargo test -p specforge top_composition_blocks_conflicting_top_actor_port_width` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-29 (`.fsm` top boundary ports recover from top actor graph)

### Improved: explicit top roots no longer need flat top-port directions when the actor graph already resolves them
- Top-root `.fsm` analysis now merges `IntentIR.actor_ports` for the explicit `top_name` into top boundary port direction evidence.
- A width-only top port can now become renderable when the top actor graph says the top-level actor drives or reads that port, even if no flat `ExplicitTopPortRecord.direction_hint` was present.
- Conflicting explicit top-port direction versus top actor-port graph direction remains blocking: the adapter collapses that port direction to unresolved rather than letting either surface silently win.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_top_port_direction_from_actor_ports` -> passed
- `cargo test -p specforge top_composition_blocks_conflicting_top_actor_port_direction` -> passed
- `cargo test -p specforge ir::adapters::tests` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-29 (Runtime doctor now tolerates cold local model load)

### Improved: local provider readiness no longer requires manual prewarming
- `specforge doctor --strict` still proves local provider readiness through the OpenAI-compatible chat-completions endpoint, but the local chat probe now allows a cold Ollama or LM Studio model load to finish before declaring the runtime unusable.
- GET probe failures now preserve curl failure context instead of collapsing connection or sandbox failures into a misleading empty-response parse error.
- This keeps the readiness check strict while avoiding the false-negative path observed when `/api/tags` was healthy, the default model was visible, and the first chat request only needed a few extra seconds to load `qwen2.5vl:7b`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge commands::doctor` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- doctor` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-29 (Cycle-qualified VLM timing-value labels no longer leak into fake timing constraints)

### Improved: richer VLM timing-annotation negatives now stay at the root cause boundary
- The timing-diagram parser already rejected:
  - bare sample/index labels like `T0`, `Addr 1`, `XREQ[0]`, and `XREQ<1>`
  - bare signal-value labels like `XREQ HIGH` and `XREQ asserted`
- But there was still a real hole around cycle-qualified signal-value annotation labels such as:
  - `XREQ HIGH at T1`
  - `XREQ LOW during T0`
  - `XREQ asserted on T1`
  - `XREQ deasserted in T0`
- Those strings are still low-value waveform labels, not real timing constraints. Treating them as constraints would create fake VLM timing records instead of preserving only the concrete waveform samples.
- Tightened the VLM timing-annotation filter so signal-value labels remain rejected even when they carry only a trailing cycle marker like `T1`, `at T1`, `on T1`, `during T0`, or `in T0`.
- Added a direct `SemanticIR` regression plus a tracked KG negative fixture in `crates/specforge/test_data/kg_quality/vlm_timing_cycle_qualified_signal_value_annotation_negative/` so the edge case is locked both locally and end to end.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_cycle_qualified_signal_value_labels` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-29 (Indexed VLM timing-value labels no longer leak into fake timing constraints)

### Improved: richer VLM timing-annotation negatives now stay at the root cause boundary
- The timing-diagram parser already rejected bare sample/index labels like `XREQ[0]` and `XREQ<1>`, but it still had a hole around indexed signal-value annotation labels such as:
  - `XREQ[0] HIGH`
  - `XREQ<1> LOW`
  - `XREQ[3:0] asserted`
  - `XREQ[7:4] deasserted`
- Those strings are low-value waveform labels, not real timing constraints. Treating them as constraints would create fake VLM timing records instead of preserving only the concrete waveform samples.
- Tightened the VLM timing-annotation filter so indexed/ranged signal-value labels are rejected alongside the already-filtered bare sample/index labels.
- Added a direct `SemanticIR` regression plus a tracked KG negative fixture in `crates/specforge/test_data/kg_quality/vlm_timing_indexed_signal_value_annotation_negative/` so the edge case is locked both locally and end to end.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_indexed_signal_value_labels` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Generic next-cycle direct coverage now proves the full lexical lane)

### Improved: the direct proof surface now matches the already-tracked generic next-cycle benchmark family
- The tracked corpus in `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/` already locked the supported generic one-cycle spellings:
  - `next cycle`
  - `next clock cycle`
  - `following cycle`
  - `subsequent cycle`
- Expanded direct extraction, semantic, and validator coverage so those same spellings now fail locally before the heavier KG fixture lane runs.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed generic next-cycle surface so the direct proof lane stays aligned with the tracked benchmark contract.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge extracts_single_cycle_window_from_idiomatic_clock_tick_phrases` -> passed
- `cargo test -p specforge derives_generic_next_cycle_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_generic_next_cycle_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Next-tick direct coverage now proves the full lexical lane)

### Improved: the direct proof surface now matches the already-tracked next-tick benchmark family
- The tracked corpus in `crates/specforge/test_data/kg_quality/next_tick_timing_gold/` already locked the supported next-tick one-cycle spellings:
  - `next tick`
  - `following tick`
  - `subsequent tick`
- Expanded direct extraction, semantic, and validator coverage so those same spellings now fail locally before the heavier KG fixture lane runs.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed next-tick surface so the direct proof lane stays aligned with the tracked benchmark contract.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge extracts_single_cycle_window_from_idiomatic_clock_tick_phrases` -> passed
- `cargo test -p specforge derives_next_tick_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_next_tick_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Clock-edge-of-clock benchmark coverage now proves the signal-leading exact lane)

### Improved: the tracked KG corpus now proves the signal-leading exact `clock edge` spelling inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PENABLE must be asserted on HCLK clock edge T5.`
- The expanded fixture now proves the `clock edge(s) of <clock>` family through both `SemanticIR` and `IntentIR`, including:
  - `clock edge T4 of HCLK`
  - `within 2 clock edges of HCLK`
  - `HCLK clock edge T5`
- Added direct semantic and validator coverage for the signal-leading exact `clock edge` variant so regressions in that lane fail outside the benchmark harness too.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed `clock edge(s) of <clock>` surface so the tracked corpus and direct tests now cover the supported signal-leading exact lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_clock_edge_of_clock_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_grounding_for_clock_edge_of_clock_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named quantified benchmark coverage now proves the signal-leading ordinal lane)

### Improved: the tracked KG corpus now proves the signal-leading ordinal named-edge spelling inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PSEL must be asserted on the third HCLK edge.`
- The expanded fixture now proves the named quantified and ordinal family through both `SemanticIR` and `IntentIR`, including:
  - `within 2 HCLK edges`
  - `third edge of HCLK`
  - `third HCLK edge`
- Added direct semantic and validator coverage for the signal-leading ordinal named-edge variant so regressions in that lane fail outside the benchmark harness too.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed named quantified-edge surface so the tracked corpus and direct tests now cover the supported signal-leading ordinal lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_named_quantified_edge_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_quantified_edge_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named diagram-edge benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves both named generic-edge diagram spellings inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PENABLE must be asserted on HCLK edge T4.`
- The expanded fixture now proves the named diagram-edge family through both `SemanticIR` and `IntentIR`, including:
  - `edge T3 of HCLK`
  - `HCLK edge T4`
- Added direct semantic and validator coverage for those named diagram-edge variants so regressions in the signal-leading lane fail outside the benchmark harness too.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed named diagram-edge surface so the tracked corpus and direct tests now cover the full supported lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_named_diagram_edge_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_grounding_for_named_diagram_edge_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Unit-first diagram-position benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on unit-first local-clock phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PENABLE must be asserted at tick T3 of HCLK.`
  - `PSEL must be asserted on rising edge T5 of HCLK.`
- The expanded fixture now proves the unit-first diagram-position family through both `SemanticIR` and `IntentIR`, including:
  - `tick T3 of HCLK`
  - `posedge T4 of HCLK`
  - `rising edge T5 of HCLK`
- Added direct semantic and validator coverage for those unit-first diagram-position variants so regressions in the `tick` or edge-word lane fail outside the benchmark harness too.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed unit-first local-clock diagram surface so the tracked corpus and direct tests now cover the full supported lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_unit_first_diagram_position_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_unit_first_diagram_position_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named one-cycle benchmark coverage now proves the full edge lane)

### Improved: the tracked KG corpus now proves generic and falling named one-cycle edge phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PSLVERR must be asserted on the next HCLK clock edge.`
  - `PSTRB must be asserted on the following HCLK falling edge.`
- The expanded fixture now proves the named one-cycle family through both `SemanticIR` and `IntentIR`, including:
  - `next ACLK cycle`
  - `following HCLK edge`
  - `subsequent HCLK rising edge`
  - `next HCLK clock edge`
  - `following HCLK falling edge`
- Added direct semantic and validator coverage for the named one-cycle edge variants so regressions in the explicit `clock edge` or falling-edge lane fail outside the benchmark harness too.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed named one-cycle local-clock surface so the tracked corpus and direct tests now cover the full supported edge lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_named_next_edge_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_next_edge_variants` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named local zero-cycle benchmark coverage now proves the full edge lane)

### Improved: the tracked KG corpus now proves generic and falling named zero-cycle edge phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PSLVERR must be asserted on the current HCLK clock edge.`
  - `PSTRB must be asserted on the current HCLK falling edge.`
- The expanded fixture now proves the named local zero-cycle family through both `SemanticIR` and `IntentIR`, including:
  - `same ACLK cycle`
  - `this HCLK tick`
  - `current HCLK edge`
  - `current HCLK clock edge`
  - `current HCLK falling edge`
- Added direct semantic and validator coverage for the named falling-edge lane so named zero-cycle edge regressions fail before or alongside the benchmark harness stage.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed named local zero-cycle surface so the tracked corpus and direct tests now cover the full supported edge lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_named_zero_cycle_edge_variants` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_named_cycle_text` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock zero-cycle benchmark coverage now proves the full edge lane)

### Improved: the tracked KG corpus now proves generic and falling zero-cycle edge phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/` so the tracked benchmark corpus now also locks:
  - `PSLVERR must be asserted on the current clock edge.`
  - `PSTRB must be asserted on the current falling edge.`
- The expanded fixture now proves the default-clock zero-cycle family through both `SemanticIR` and `IntentIR`, including:
  - `same cycle`
  - `same tick`
  - `this tick`
  - `current clock edge`
  - `current rising edge`
  - `current falling edge`
- Added direct extraction coverage in `crates/specforge/src/ir/semantic.rs` for `current clock edge` and `current falling edge` so zero-cycle edge regressions fail before the benchmark harness stage.
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed default-clock zero-cycle surface so the tracked corpus and direct parser tests now cover the full supported edge lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge extracts_zero_cycle_window_from_same_cycle_phrases` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock zero-cycle benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on default-clock zero-cycle phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted in the same cycle.`
  - `PENABLE must be asserted in the same tick.`
  - `PSEL must be asserted on this tick.`
  - `PWRITE must be asserted on the current rising edge.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 0..0`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed default-clock zero-cycle surface so the tracked corpus now covers the full supported lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (MSRV is now aligned to Rust 1.95)

### Improved: the declared Rust floor now matches the bumped toolchain baseline
- Updated the workspace `rust-version` in `Cargo.toml` from `1.89` to `1.95`.
- Updated `.github/workflows/ci.yml` so the hosted CI toolchain now installs Rust `1.95.0`.
- Updated `docs/book/src/getting-started.md` so the user-facing setup docs now advertise Rust `1.95.0`.
- This keeps the declared MSRV, the CI environment, and the public getting-started surface aligned after the toolchain bump.

### Validation
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named local zero-cycle benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on named zero-cycle phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `TVALID must be asserted in the same ACLK cycle.`
  - `TREADY must be asserted on this HCLK tick.`
  - `PSEL must be asserted on the current HCLK edge.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal`
  - `edge = rising`
  - `cycle_window = 0..0`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed named local zero-cycle surface so the tracked corpus now covers the full supported lexical lane.
- The slice also restored the repo’s full standard CI lane by fixing current clippy-rooted failures in `crates/specforge/src/commands/validate.rs`, `crates/specforge/src/ir/evidence.rs`, `crates/specforge/src/ir/semantic.rs`, and `crates/specforge/src/ir/source.rs` instead of weakening verification.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named one-cycle benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on named one-cycle phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next ACLK cycle.`
  - `PENABLE must be asserted on the following HCLK edge.`
  - `PSEL must be asserted on the subsequent HCLK rising edge.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal`
  - `edge = rising`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed named one-cycle local-clock surface so the tracked corpus now covers the full supported lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock explicit edge benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on explicit edge phrasing inside the existing default-clock family
- Expanded `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next rising edge.`
  - `PWAKEUP must be asserted on the next falling edge.`
  - `PSEL must be asserted on the following rising edge.`
  - `PWRITE must be asserted on the following falling edge.`
  - `PSLVERR must be asserted on the subsequent rising edge.`
  - `PSTRB must be asserted on the subsequent falling edge.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - preserved explicit `edge = rising` / `edge = falling`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed default-clock explicit-edge surface so the tracked corpus now covers the full supported lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Generic clock-edge benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on generic clock-edge phrasing inside the existing family
- Expanded `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next clock edge.`
  - `PSEL must be asserted on the following clock edge.`
  - `PWRITE must be asserted on the subsequent clock edge.`
  - `PENABLE must be asserted within 2 clock edges.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - the expected one-cycle or bounded `cycle_window`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed generic clock-edge surface so the tracked corpus now covers the full supported one-cycle lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Shorthand next-edge benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on shorthand edge phrasing inside the existing next-edge family
- Expanded `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next posedge.`
  - `PENABLE must be asserted on the next negedge.`
  - `PSEL must be asserted on the following posedge.`
  - `PWRITE must be asserted on the following negedge.`
  - `PSLVERR must be asserted on the subsequent posedge.`
  - `PSTRB must be asserted on the subsequent negedge.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - preserved explicit `edge = rising` / `edge = falling`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed shorthand next-edge surface so the tracked corpus now covers the full supported lexical lane.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Next-tick benchmark coverage now proves the full lexical lane)

### Improved: the tracked KG corpus now proves follow-on tick phrasing inside the existing next-tick family
- Expanded `crates/specforge/test_data/kg_quality/next_tick_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next tick.`
  - `PENABLE must be asserted on the following tick.`
  - `PSEL must be asserted on the subsequent tick.`
- The expanded fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model or add a new benchmark family. It hardens the already-landed next-tick surface so the tracked corpus now covers the same lexical symmetry already supported for one-cycle timing.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock later-edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves default-clock later-edge timing end to end
- Added `crates/specforge/test_data/kg_quality/default_clock_later_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted after 3 rising edges.`
  - `PWAKEUP must be asserted after 3 falling edges.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - preserved explicit `edge = rising` / `edge = falling`
  - `cycle_window = 3..3`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed default-clock later-edge family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock quantified edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves default-clock quantified and ordinal edge timing end to end
- Added `crates/specforge/test_data/kg_quality/default_clock_quantified_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the third rising edge.`
  - `PWAKEUP must be asserted on the third falling edge.`
  - `PSEL must be asserted within 2 rising edges.`
  - `PWRITE must be asserted within 2 falling edges.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - preserved explicit `edge = rising` / `edge = falling`
  - the expected exact or bounded `cycle_window`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed default-clock quantified and ordinal edge family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock explicit next-edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves default-clock explicit next-edge timing end to end
- Added `crates/specforge/test_data/kg_quality/default_clock_explicit_next_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next rising edge.`
  - `PWAKEUP must be asserted on the next falling edge.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - preserved explicit `edge = rising` / `edge = falling`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed default-clock explicit next-edge family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Generic exact cycle timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare exact-cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/generic_exact_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted after 2 cycles.`
  - `PENABLE must be stable for 2 cycles.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 2..2`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed generic exact-cycle family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Generic range cycle timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare range and one-sided cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/generic_range_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted after at least 2 cycles.`
  - `PENABLE must be asserted after at most 3 cycles.`
  - `PSEL must be asserted after between 1 and 3 cycles.`
  - `PWRITE must be asserted after no more than 2 cycles.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - the expected min/max `cycle_window` bounds for each phrase
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed generic range and one-sided cycle family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Generic bounded cycle timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare bounded cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/generic_bounded_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted within 2 cycles.`
- The new fixture proves that form through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window.max_cycles = 2`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed generic bounded cycle family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Generic next-cycle timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare next-cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/generic_next_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next cycle.`
  - `PENABLE must be asserted on the next clock cycle.`
  - `PSEL must be asserted on the following cycle.`
  - `PWRITE must be asserted on the subsequent cycle.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed generic next-cycle family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock zero-cycle timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare zero-cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/zero_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted in the same tick.`
  - `PENABLE must be asserted on the current rising edge.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 0..0`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed default-clock zero-cycle family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Default-clock generic clock-edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare generic clock-edge timing end to end
- Added `crates/specforge/test_data/kg_quality/generic_clock_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next clock edge.`
  - `PENABLE must be asserted within 2 clock edges.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - the expected one-cycle or bounded `cycle_window`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed default-clock generic clock-edge family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Next-tick timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves idiomatic next-tick timing end to end
- Added `crates/specforge/test_data/kg_quality/next_tick_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next tick.`
- The new fixture proves that form through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed idiomatic next-tick family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Later-phrase timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves explicit later-phrase timing end to end
- Added `crates/specforge/test_data/kg_quality/later_phrase_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted two cycles later.`
- The new fixture proves that form through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 2..2`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed explicit later-phrase family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Shorthand next-edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves bare posedge/negedge one-cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/shorthand_next_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next posedge.`
  - `PENABLE must be asserted on the next negedge.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - preserved explicit shorthand `edge`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed shorthand next-edge family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Tick-unit timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves generic tick-unit timing end to end
- Added `crates/specforge/test_data/kg_quality/tick_unit_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted within 2 ticks.`
  - `PENABLE must be asserted at tick T3.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - default-clock grounding to `clock_signal = HCLK`
  - `edge = rising`
  - the expected bounded or exact `cycle_window`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed generic tick-unit family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Plural edge-of-clock timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves plural edge-of-clock timing end to end
- Added `crates/specforge/test_data/kg_quality/plural_edge_of_clock_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted within 2 edges of HCLK.`
- The new fixture proves that form through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window.max_cycles = 2`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed plural edge-of-clock family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Unit-first diagram-position timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves unit-first diagram-position timing end to end
- Added `crates/specforge/test_data/kg_quality/unit_first_diagram_position_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on posedge T4 of HCLK.`
- The new fixture proves that form through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 4..4`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed unit-first diagram-position family into the tracked KG-quality corpus.

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named diagram-edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves named diagram-edge timing end to end
- Added `crates/specforge/test_data/kg_quality/named_diagram_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on edge T3 of HCLK.`
- The new fixture proves that form through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 3..3`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed named diagram-edge family into the tracked KG-quality corpus.

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named quantified and ordinal clock-edge timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves named bounded and exact edge timing end to end
- Added `crates/specforge/test_data/kg_quality/named_quantified_edge_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted within 2 HCLK edges.`
  - `PENABLE must be asserted on the third edge of HCLK.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal = HCLK`
  - `edge = rising`
  - the expected bounded or exact `cycle_window`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed named quantified and ordinal edge family into the tracked KG-quality corpus.

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named one-cycle clock timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves named one-cycle timing end to end
- Added `crates/specforge/test_data/kg_quality/named_next_clock_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on the next ACLK cycle.`
  - `PENABLE must be asserted on the next HCLK edge.`
  - `PSEL must be asserted on the next HCLK rising edge.`
- The new fixture proves those forms through both `SemanticIR` and `IntentIR`, with:
  - preserved local `clock_signal`
  - `edge = rising`
  - `cycle_window = 1..1`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed named one-cycle timing family into the tracked KG-quality corpus.

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Named local cycle timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves named local cycle grounding end to end
- Added `crates/specforge/test_data/kg_quality/named_cycle_timing_gold/` so the tracked benchmark corpus now locks:
  - `TVALID must be asserted in the same ACLK cycle.`
- The new fixture proves that phrase through both `SemanticIR` and `IntentIR`, with:
  - `clock_signal = ACLK`
  - `edge = rising`
  - `cycle_window = 0..0`
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed named local cycle timing family into the tracked KG-quality corpus.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Signal-leading clock timing is now benchmark-locked too)

### Improved: the tracked KG corpus now proves the full signal-leading local-clock family
- Added `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/` so the tracked benchmark corpus now locks:
  - `PREADY must be asserted on HCLK rising edge.`
  - `PWAKEUP must be asserted on HCLK falling edge.`
  - `PSEL must be asserted on HCLK posedge.`
  - `PWRITE must be asserted on HCLK negedge.`
- The new fixture proves those four forms through both `SemanticIR` and `IntentIR`, with:
  - `clock_signal = HCLK`
  - the correct preserved edge kind
  - `temporal_rules_missing_clock_grounding = 0`
- This does not widen the temporal model. It raises the already-landed signal-leading clock family into the tracked KG-quality corpus so the public benchmark surface matches the direct unit-proof surface.

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Signal-leading clock coverage is now lexically self-contained too)

### Improved: the direct signal-leading proof lane now carries both edge-word and edge-token forms
- `crates/specforge/src/ir/semantic.rs` now widens the existing signal-leading semantic regression so it proves all four direct local-clock variants:
  - `PREADY must be asserted on HCLK rising edge.`
  - `PWAKEUP must be asserted on HCLK falling edge.`
  - `PSEL must be asserted on HCLK posedge.`
  - `PWRITE must be asserted on HCLK negedge.`
- `crates/specforge/src/commands/validate.rs` now widens the existing signal-leading validator regression so it proves the same family across the complementary lexical pair:
  - `PREADY must be asserted on HCLK posedge.`
  - `PWAKEUP must be asserted on HCLK negedge.`
  - `PSEL must be asserted on HCLK rising edge.`
  - `PWRITE must be asserted on HCLK falling edge.`
- This does not widen the temporal model or add tracked fixtures. It removes the last lexical proof split inside the existing signal-leading clock family.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_explicit_clock_signal_from_signal_leading_edge_text` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_signal_leading_clock_text` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Signal-leading clock coverage is now falling-side complete too)

### Improved: the direct signal-leading clock proof lane now carries both edge directions
- `crates/specforge/src/ir/semantic.rs` now widens the existing signal-leading semantic regression so it proves both:
  - `PREADY must be asserted on HCLK rising edge.`
  - `PWAKEUP must be asserted on HCLK falling edge.`
- `crates/specforge/src/commands/validate.rs` now widens the existing signal-leading validator regression so it proves both:
  - `PREADY must be asserted on HCLK posedge.`
  - `PWAKEUP must be asserted on HCLK negedge.`
- This does not widen the temporal model or add tracked fixtures. It closes the remaining falling-side proof asymmetry inside the existing signal-leading clock family.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_explicit_clock_signal_from_signal_leading_edge_text` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_signal_leading_clock_text` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Word-edge validator coverage is now family-complete too)

### Improved: the direct intent-stage validator lane now proves the full trailing word-edge family
- `crates/specforge/src/commands/validate.rs` now widens the existing trailing word-edge validator regression so it carries:
  - `PREADY must be asserted on the third rising edge of HCLK.`
  - `PWAKEUP must be asserted on the third falling edge of HCLK.`
  - `PSEL must be asserted within 2 rising edges of HCLK.`
  - `PWRITE must be asserted within 2 falling edges of HCLK.`
- That keeps the spelled-out edge family self-contained in one direct validator lane instead of splitting:
  - exact word-edge timing into the broader `explicit_clock_text` proof lane
  - bounded word-edge timing into the `trailing_of_word_edge` proof lane
- This does not widen the temporal model or add new fixtures. It makes the direct validator contract for the trailing word-edge family internally consistent.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Exact shorthand-edge validator coverage is now symmetry-locked too)

### Improved: the direct intent-stage validator lane now proves the full exact shorthand-token pair
- `crates/specforge/src/commands/validate.rs` now widens the existing trailing shorthand-edge validator regression so it carries:
  - `PGRANT must be asserted on the third posedge of HCLK.`
  - `PLOCK must be asserted on the third negedge of HCLK.`
- That same direct validator lane already covered the bounded shorthand-token pair:
  - `within 2 posedges of HCLK`
  - `within 2 negedges of HCLK`
- The result is a cleaner symmetry story at the validator level:
  - bounded rising token: covered
  - bounded falling token: covered
  - exact rising token: covered
  - exact falling token: covered
- This does not widen the temporal model or the benchmark corpus. It hardens the direct validator proof so the exact shorthand-token pair is no longer split across semantic-only and fixture-only evidence.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Exact shorthand-edge timing is now symmetry-locked on the falling token side too)

### Improved: the existing trailing shorthand-edge benchmark now proves exact ordinal falling shorthand-token prose as well
- Extended `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` with:
  - `PLOCK must be asserted on the third negedge of HCLK.`
- The existing tracked fixture now locks eight complementary forms together:
  - exact singular rising shorthand token: `the third posedge of HCLK`
  - exact singular falling shorthand token: `the third negedge of HCLK`
  - bounded plural rising shorthand token form: `within 2 posedges of HCLK`
  - bounded plural falling shorthand token form: `within 2 negedges of HCLK`
  - bounded word-based rising-edge form: `within 2 rising edges of HCLK`
  - bounded word-based falling-edge form: `within 2 falling edges of HCLK`
  - exact ordinal word-based falling-edge form: `the third falling edge of HCLK`
  - exact ordinal word-based rising-edge form: `the third rising edge of HCLK`
- This closes the last exact shorthand-token asymmetry inside the trailing `of <clock>` timing family instead of leaving the falling-side token twin implied only by nearby logic.

### Added: focused extraction, semantic, and validator coverage for exact falling shorthand-token timing
- `crates/specforge/src/ir/semantic.rs` now also proves trailing `of <clock>` local-clock extraction for:
  - `the third negedge of HCLK`
- A new direct semantic regression now proves:
  - `PLOCK must be asserted on the third negedge of HCLK.`
  preserves:
  - `clock_signal = HCLK`
  - `edge = falling`
  - `cycle_window = 3..3`
- `crates/specforge/src/commands/validate.rs` now widens the existing trailing shorthand-edge validator regression so exact falling shorthand-token timing stays grounded alongside the already-locked bounded token pair.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge extracts_trailing_of_shorthand_edge_clock_phrases` -> passed
- `cargo test -p specforge derives_exact_falling_edge_from_trailing_of_shorthand_edge_text` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Ordinal edge-word timing is now benchmark-locked on the rising side too)

### Improved: the existing trailing shorthand-edge benchmark now proves exact ordinal rising-edge prose as well
- Extended `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` with:
  - `PGRANT must be asserted on the third rising edge of HCLK.`
- The existing tracked fixture now locks seven complementary forms together:
  - exact singular rising shorthand: `the third posedge of HCLK`
  - bounded plural rising shorthand token form: `within 2 posedges of HCLK`
  - bounded plural falling shorthand token form: `within 2 negedges of HCLK`
  - bounded word-based rising-edge form: `within 2 rising edges of HCLK`
  - bounded word-based falling-edge form: `within 2 falling edges of HCLK`
  - exact ordinal word-based falling-edge form: `the third falling edge of HCLK`
  - exact ordinal word-based rising-edge form: `the third rising edge of HCLK`
- This closes the remaining benchmark asymmetry inside the exact ordinal word-edge family instead of leaving the rising-side exact form only unit-locked.

### Added: benchmark and extraction coverage for exact ordinal rising-edge timing
- `crates/specforge/src/ir/semantic.rs` now also proves trailing `of <clock>` local-clock extraction for:
  - `the third rising edge of HCLK`
- The existing tracked fixture now locks that same exact word-based rising-edge phrase through both `SemanticIR` and `IntentIR`, with:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 3..3`
- The existing semantic and validator regressions for exact ordinal word-edge timing remain the direct proof lane for the rising/falling pair, and the fixture now matches that same contract at benchmark level.

### Validation
- `cargo test -p specforge extracts_trailing_of_shorthand_edge_clock_phrases` -> passed
- `cargo test -p specforge derives_exact_cycle_window_from_ordinal_edge_constraint_text` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_explicit_clock_text` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `cargo fmt --all` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Ordinal edge-word timing is now falling-side symmetry-locked too)

### Improved: the existing trailing shorthand-edge benchmark now proves exact ordinal falling-edge prose as well
- Extended `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` with:
  - `PWAKEUP must be asserted on the third falling edge of HCLK.`
- The existing tracked fixture now locks six complementary forms together:
  - exact singular rising shorthand: `the third posedge of HCLK`
  - bounded plural rising shorthand token form: `within 2 posedges of HCLK`
  - bounded plural falling shorthand token form: `within 2 negedges of HCLK`
  - bounded word-based rising-edge form: `within 2 rising edges of HCLK`
  - bounded word-based falling-edge form: `within 2 falling edges of HCLK`
  - exact ordinal word-based falling-edge form: `the third falling edge of HCLK`
- This keeps the exact ordinal word-edge family honest instead of leaving the falling-side twin implied only by the already-landed rising-side regression.

### Added: focused semantic and validator coverage for exact ordinal falling-edge timing
- `crates/specforge/src/ir/semantic.rs` now proves the ordinal edge family as a pair:
  - `PREADY must be asserted on the third rising edge of HCLK.`
  - `PWAKEUP must be asserted on the third falling edge of HCLK.`
- The same semantic regression now proves the falling-side twin preserves:
  - `clock_signal = HCLK`
  - `edge = falling`
  - `cycle_window = 3..3`
- `crates/specforge/src/commands/validate.rs` now proves both exact ordinal word-edge rules stay fully grounded together in one intent-stage validation pass.
- The explicit local-clock extraction regression for trailing `of <clock>` edge phrasing now also covers:
  - `the third falling edge of HCLK`

### Validation
- `cargo test -p specforge extracts_trailing_of_shorthand_edge_clock_phrases` -> passed
- `cargo test -p specforge derives_exact_cycle_window_from_ordinal_edge_constraint_text` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_explicit_clock_text` -> passed
- `cargo test -p specforge ordinal_edge_constraint_text` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `cargo fmt --all` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Trailing edge-word timing is now falling-side symmetry-locked too)

### Improved: the existing trailing shorthand-edge benchmark now proves bounded word-based falling-edge prose as well
- Extended `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` with:
  - `PWRITE must be asserted within 2 falling edges of HCLK.`
- The existing tracked fixture now locks five complementary forms together:
  - exact singular rising shorthand: `the third posedge of HCLK`
  - bounded plural rising shorthand token form: `within 2 posedges of HCLK`
  - bounded plural falling shorthand token form: `within 2 negedges of HCLK`
  - bounded word-based rising-edge form: `within 2 rising edges of HCLK`
  - bounded word-based falling-edge form: `within 2 falling edges of HCLK`
- This keeps the word-based edge family symmetric under regression instead of leaving the falling-side prose only implied by shared parser logic.

### Added: focused semantic and validator coverage for bounded falling word-edge timing
- `crates/specforge/src/ir/semantic.rs` now has a direct regression proving `PWRITE must be asserted within 2 falling edges of HCLK.` preserves:
  - `clock_signal = HCLK`
  - `edge = falling`
  - `cycle_window.max_cycles = 2`
- The same validator regression for trailing word-edge timing now proves both bounded word-edge twins stay fully grounded together:
  - `within 2 rising edges of HCLK`
  - `within 2 falling edges of HCLK`
- The explicit local-clock extraction regression for trailing `of <clock>` edge phrasing now also covers:
  - `within 2 falling edges of HCLK`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge derives_falling_edge_from_trailing_of_word_edge_text` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text` -> passed
- `cargo test -p specforge trailing_of_word_edge` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-22 (Trailing edge-word timing is now symmetry-locked too)

### Improved: the existing trailing shorthand-edge benchmark now proves bounded word-based rising-edge prose as well
- Extended `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` with:
  - `PSEL must be asserted within 2 rising edges of HCLK.`
- The existing tracked fixture now locks four complementary forms together:
  - exact singular rising shorthand: `the third posedge of HCLK`
  - bounded plural rising shorthand token form: `within 2 posedges of HCLK`
  - bounded plural falling shorthand token form: `within 2 negedges of HCLK`
  - bounded word-based rising-edge form: `within 2 rising edges of HCLK`
- This keeps the same temporal family symmetric under regression instead of leaving the spelled-out rising-edge form only implied by shared parser logic.

### Added: focused semantic and validator coverage for bounded word-edge timing
- `crates/specforge/src/ir/semantic.rs` now has a direct regression proving `PSEL must be asserted within 2 rising edges of HCLK.` preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window.max_cycles = 2`
- `crates/specforge/src/commands/validate.rs` now proves that bounded word-edge phrasing stays fully grounded through intent validation:
  - `temporal_rules = 1`
  - `temporal_rules_with_cycle_window = 1`
  - `temporal_rules_missing_clock_grounding = 0`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge trailing_of_shorthand_edge` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Plural shorthand-edge timing is now symmetry-locked)

### Improved: the trailing shorthand-edge benchmark now proves plural `posedges` as well as plural `negedges`
- Extended `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` with:
  - `PENABLE must be asserted within 2 posedges of HCLK.`
- The existing tracked fixture now locks three complementary forms together:
  - exact singular rising shorthand: `the third posedge of HCLK`
  - bounded plural rising shorthand: `within 2 posedges of HCLK`
  - bounded plural falling shorthand: `within 2 negedges of HCLK`
- This keeps the plural shorthand-edge family symmetric under regression instead of leaving the rising-side plural path only implied by shared detector logic.

### Added: focused semantic and validator coverage for plural rising shorthand timing
- `crates/specforge/src/ir/semantic.rs` now has a focused regression proving `PENABLE must be asserted within 2 posedges of HCLK.` preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window.max_cycles = 2`
- `crates/specforge/src/commands/validate.rs` now proves both plural shorthand-edge rules stay grounded together in one intent-stage validation pass:
  - `within 2 posedges of HCLK`
  - `within 2 negedges of HCLK`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge trailing_of_shorthand_edge` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Plural shorthand-edge timing now preserves explicit edge semantics)

### Fixed: plural `posedge` / `negedge` wording no longer falls back to the default edge
- `crates/specforge/src/ir/semantic.rs` had a narrow but real edge-grounding gap:
  - singular `posedge` / `negedge` phrasing preserved the explicit edge kind
  - plural `posedges` / `negedges` phrasing still preserved the local `clock_signal` and `cycle_window`
  - but the edge detector only recognized the singular tokens, so plural shorthand-edge timing could silently fall back to `edge = rising`
- The fix is small and bounded:
  - `explicit_clock_edge_from_text()` now recognizes plural `posedges` and `negedges`
  - the temporal model now keeps `edge = falling` for phrases like `within 2 negedges of HCLK`
  - nearby rising-edge plural phrasing now stays symmetric too

### Added: regression coverage for plural shorthand-edge edge semantics
- Added focused semantic coverage proving `PSLVERR must be asserted within 2 negedges of HCLK.` now preserves:
  - `clock_signal = HCLK`
  - `edge = falling`
  - `cycle_window.max_cycles = 2`

### Added: tracked fixture coverage for trailing shorthand-edge timing phrasing
- Added a new tracked KG-quality fixture at `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/` covering:
  - `PREADY must be asserted on the third posedge of HCLK.`
  - `PSLVERR must be asserted within 2 negedges of HCLK.`
- This raises the newer trailing shorthand-edge family into the same benchmark lane now used for the neighboring temporal phrasing families:
  - parser and semantic/validator unit coverage already existed
  - now the same phrasing family is also locked through the tracked fixture harness used for project-quality regression checks

### Added: benchmark expectations for exact rising-edge and bounded falling-edge shorthand timing
- The new tracked fixture proves that both `SemanticIR` and `IntentIR` preserve:
  - `clock_signal = HCLK`
  - `edge = rising` with `cycle_window = 3..3` for `the third posedge of HCLK`
  - `edge = falling` with `cycle_window.max_cycles = 2` for `within 2 negedges of HCLK`
- The validation expectations in that same fixture also prove:
  - `temporal_rules = 2`
  - `temporal_rules_with_cycle_window = 2`
  - `temporal_rules_missing_clock_grounding = 0`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `cargo test -p specforge trailing_of_shorthand_edge` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (KG benchmark now includes generic `clock edge(s) of <clock>` timing coverage)

### Added: tracked fixture coverage for generic clock-edge-of-clock timing phrasing
- Added a new tracked KG-quality fixture at `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/` covering:
  - `PREADY must be asserted on clock edge T4 of HCLK.`
  - `PSLVERR must be asserted within 2 clock edges of HCLK.`
- This strengthens the temporal contract at the benchmark level:
  - parser coverage already existed
  - semantic and validator unit coverage already existed
  - now the same phrasing family is also locked through the tracked fixture harness used for project-quality regression checks

### Added: benchmark expectations for exact and bounded generic clock-edge-of-clock timing
- The new tracked fixture proves that both `SemanticIR` and `IntentIR` preserve:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 4..4` for `clock edge T4 of HCLK`
  - `cycle_window.max_cycles = 2` for `within 2 clock edges of HCLK`
- The validation expectations in that same fixture also prove:
  - `temporal_rules = 2`
  - `temporal_rules_with_cycle_window = 2`
  - `temporal_rules_missing_clock_grounding = 0`

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `cargo test -p specforge clock_edge_of_clock` -> passed

## 2026-04-21 (Generic `clock edge(s) of <clock>` phrasing is now regression-locked end to end)

### Improved: documented `clock edge(s) of <clock>` support is now proven through the semantic and validation layers too
- `crates/specforge/src/ir/semantic.rs` already knew how to recover the generic clock-edge-of-clock family:
  - `clock edge T4 of HCLK`
  - `within 2 clock edges of HCLK`
- This slice closes a quality gap rather than widening capability:
  - the support was already described in the docs and already present in the parser helpers
  - but it was not locked end to end the way the neighboring temporal phrasing families were
- The model boundary stays the same:
  - explicit `clock edge(s) of <known clock>` phrasing remains accepted
  - arbitrary edge wording still does not become timing truth

### Added: parser, semantic, and validator regression coverage for generic clock-edge-of-clock phrasing
- Added parser-level coverage proving:
  - `clock edge T4 of HCLK` still becomes `cycle_window = 4..4`
  - `within 2 clock edges of HCLK` still becomes `cycle_window.max_cycles = 2`
  - both forms preserve `clock_signal = HCLK`
- Added end-to-end semantic coverage proving `PREADY must be asserted on clock edge T4 of HCLK` preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 4..4`
- Added validator coverage proving that same phrasing does not trigger either `intent_temporal_rules_missing_clock_grounding` or `intent_temporal_rules_missing_cycle_windows`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge clock_edge_of_clock` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Trailing `of <clock>` shorthand edge phrases now ground the local clock)

### Improved: shorthand edge phrases with trailing `of <clock>` now preserve explicit local clock grounding
- `crates/specforge/src/ir/semantic.rs` now recognizes the remaining non-diagram shorthand-edge family that was still half grounded:
  - `on the third posedge of HCLK`
  - `within 2 negedges of HCLK`
  - `within 2 rising edges of HCLK`
- This closes the next honest temporal asymmetry:
  - the parser already knew the edge and bounded window for these phrases
  - but the local clock detector still lagged unless the wording used the diagram-position `... T4 of HCLK` family
- The widening stays bounded:
  - only explicit trailing `of <known clock>` phrasing is added
  - support is limited to shorthand edge families already accepted as timing language
  - arbitrary `posedge of ...` wording without a known current-document clock still does not become timing truth

### Added: regression coverage for trailing `of <clock>` shorthand-edge grounding
- Added parser-level coverage proving:
  - `third posedge of HCLK`
  - `within 2 negedges of HCLK`
  - `within 2 rising edges of HCLK`
  now preserve `clock_signal = HCLK`
- Added end-to-end semantic coverage proving `PREADY must be asserted on the third posedge of HCLK` now preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 3..3`
- Added validator coverage proving `PREADY must be asserted within 2 negedges of HCLK` no longer triggers either `intent_temporal_rules_missing_clock_grounding` or `intent_temporal_rules_missing_cycle_windows`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge trailing_of_shorthand_edge` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Unit-first diagram positions now ground the local clock)

### Improved: unit-first diagram-position wording with trailing `of <clock>` now preserves explicit local clock grounding
- `crates/specforge/src/ir/semantic.rs` now preserves `clock_signal` for the remaining unit-first diagram-position family:
  - `tick T3 of HCLK`
  - `posedge T4 of HCLK`
  - `rising edge T5 of HCLK`
- This closes another half-grounded temporal state:
  - the parser already knew the exact window for those phrases
  - but the explicit local clock detector could still miss `clock_signal = HCLK` because the clock name came after the diagram-position token
- The fix stays bounded:
  - only explicit unit-first diagram positions that end in `of <known clock>` are added
  - arbitrary `tick T3 of ...` or `posedge T4 of ...` text without a known current-document clock still does not become timing truth

### Added: regression coverage for unit-first diagram-position clock grounding
- Added parser-level coverage proving:
  - `tick T3 of HCLK`, `posedge T4 of HCLK`, and `rising edge T5 of HCLK` still recover exact bounded windows
  - those same forms now also preserve `clock_signal = HCLK`
- Added end-to-end semantic coverage proving `PREADY must be asserted on posedge T4 of HCLK` now preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 4..4`
- Added validator coverage proving that same unit-first diagram-position phrasing no longer triggers `intent_temporal_rules_missing_clock_grounding`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge unit_first_diagram_position` -> passed
- `cargo test -p specforge extracts_unit_first_diagram_position_of_clock_phrases` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Named diagram-style generic edge positions now recover bounded windows)

### Improved: named diagram-style generic edge wording now joins the explicit temporal model
- `crates/specforge/src/ir/semantic.rs` now recovers exact bounded windows from the explicit known-clock diagram-style generic-edge family:
  - `HCLK edge T3`
  - `edge T3 of HCLK`
  - `clock edge T4 of HCLK`
- This closes another half-modeled temporal corner:
  - unit-first diagram labels like `tick T3` and `posedge T4` were already first-class
  - named bounded/ordinal generic-edge prose like `within 2 HCLK edges` and `the third edge of HCLK` was already first-class
  - but the explicit diagram-style generic-edge bridge between them still lagged
- The widening stays bounded and honest:
  - only explicit known-clock generic-edge diagram positions are accepted
  - arbitrary `edge T3` wording without a known local clock still does not become timing truth
  - the same slice now preserves `clock_signal = HCLK` for those phrases instead of only recovering a window

### Added: regression coverage for named diagram-style edge positions
- Added parser-level coverage proving:
  - `HCLK edge T3` now becomes `cycle_window = 3..3`
  - `edge T4 of HCLK` now becomes `cycle_window = 4..4`
  - both forms now preserve `clock_signal = HCLK`
- Added end-to-end semantic coverage proving `PREADY must be asserted on edge T3 of HCLK` now preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 3..3`
- Added validator coverage proving that same named diagram-style edge phrasing no longer triggers either `intent_temporal_rules_missing_clock_grounding` or `intent_temporal_rules_missing_cycle_windows`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge named_diagram_edge` -> passed
- `cargo test -p specforge named_generic_edge_diagram_position` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Plural `edge(s) of <clock>` phrases now ground the local clock)

### Improved: plural `edge(s) of <clock>` wording now preserves explicit local clock grounding
- `crates/specforge/src/ir/semantic.rs` now recognizes the remaining plural explicit-clock patterns that were still lagging:
  - `within 2 edges of HCLK`
  - `between 1 and 2 edges of HCLK`
  - `within 2 clock edges of HCLK`
- This closes a clock-grounding asymmetry:
  - the named generic-edge slice could already recover a bounded `cycle_window` for these phrases
  - but plural `edge(s) of <clock>` wording could still miss `clock_signal = HCLK` because the explicit local clock detector only matched singular `edge of HCLK`
- The fix stays narrow:
  - only explicit plural `edge(s) of <known clock>` / `clock edge(s) of <known clock>` forms are added
  - there is still no widening into arbitrary `edge` guessing

### Added: regression coverage for plural edge-of-clock grounding
- Added semantic coverage proving `PREADY must be asserted within 2 edges of HCLK` now preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window.max_cycles = 2`
- Added validator coverage proving that same plural edge-of-clock phrasing no longer triggers either `intent_temporal_rules_missing_clock_grounding` or `intent_temporal_rules_missing_cycle_windows`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge plural_edge_of_clock` -> passed
- `cargo test -p specforge missing_clock_grounding_for_plural_edge_of_clock_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Named quantified and ordinal clock-edge phrases now recover bounded windows)

### Improved: explicit named clock-edge phrasing now covers bounded and ordinal windows too
- `crates/specforge/src/ir/semantic.rs` now extends the known-signal-aware temporal resolver so explicit named generic-edge phrasing no longer drops bounded timing structure.
- That now covers phrases like:
  - `within 2 HCLK edges`
  - `after 3 HCLK edges`
  - `on the third edge of HCLK`
  - `between 1 and 2 edges of HCLK`
- This closes the next temporal asymmetry:
  - `next HCLK edge` was already first-class after the prior slice
  - but quantified or ordinal named-edge forms were still weaker than neighboring `cycle`, `tick`, `clock edge`, or `rising edge` language
- The widening stays bounded:
  - this support only lives in the known-signal-aware temporal resolver
  - generic `edge` still becomes timing structure only when attached to an explicit local clock signal
  - the raw parser still does not guess timing from arbitrary uses of the word `edge`

### Added: regression coverage for named bounded/ordinal edge phrasing
- Added semantic coverage proving:
  - `PREADY must be asserted within 2 HCLK edges` now preserves `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
  - `PREADY must be asserted on the third edge of HCLK` now preserves `clock_signal = HCLK`, `edge = rising`, and `cycle_window = 3..3`
- Added validator coverage proving the named bounded-edge case no longer triggers either `intent_temporal_rules_missing_cycle_windows` or `intent_temporal_rules_missing_clock_grounding`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge named_quantified_edge` -> passed
- `cargo test -p specforge edge_of_clock` -> passed
- `cargo test -p specforge named_next_edge_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Generic clock-edge phrases now recover bounded temporal windows)

### Improved: generic clock-edge phrasing now joins the explicit temporal model
- `crates/specforge/src/ir/semantic.rs` now treats bounded clock-edge phrasing as first-class timing language instead of only trusting the more explicit `rising edge` / `falling edge` / `posedge` / `negedge` family.
- That now covers:
  - `next clock edge`
  - `within 2 clock edges`
  - `next HCLK edge`
  - `same edge of HCLK`
- This closes another honest asymmetry in the clock model:
  - explicit local clock-edge naming like `next HCLK edge` should not preserve `clock_signal = HCLK` while still losing the bounded timing window
  - generic `clock edge` prose should not be weaker than the neighboring `cycle` / `tick` / shorthand-edge forms when the phrasing is still explicit timing language
- The widening stays bounded:
  - generic `edge` only becomes timing structure when it is tied to an explicit clock phrase such as `clock edge`, `<clock> edge`, or `edge of <clock>`
  - the parser is still not widened into arbitrary edge-word guessing

### Added: regression coverage for generic clock-edge temporal phrasing
- Added direct parser coverage proving:
  - `next clock edge` now becomes `cycle_window = 1..1`
  - `within 2 clock edges` now becomes `cycle_window.max_cycles = 2`
- Added end-to-end semantic coverage proving `PREADY must be asserted on the next HCLK edge` now preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 1..1`
- Added validator coverage proving that same named next-edge phrasing no longer triggers either `intent_temporal_rules_missing_cycle_windows` or `intent_temporal_rules_missing_clock_grounding`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge generic_clock_edge` -> passed
- `cargo test -p specforge named_next_edge_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Named one-cycle clock phrases now recover `cycle_window`)

### Improved: relative one-cycle phrasing now works even when a local clock name sits between `next` and the timing unit
- `crates/specforge/src/ir/semantic.rs` now recovers bounded one-cycle windows from named local clock phrasing such as:
  - `next ACLK cycle`
  - `next PCLK tick`
  - `next HCLK rising edge`
  - `next HCLK posedge`
- This closes the next clock-tick asymmetry:
  - plain `next cycle`, `next tick`, `next rising edge`, and `next posedge` were already first-class
  - named local clock variants still grounded `clock_signal` and edge but could miss the `cycle_window`
- The fix stays bounded:
  - the extra one-cycle recovery path only runs inside the known-signal-aware temporal resolver
  - it requires an actual known signal name between the relative phrase and the timing unit
  - the raw parser is not widened into arbitrary token-skipping heuristics

### Added: regression coverage for named one-cycle clock phrasing
- Added semantic coverage proving `PREADY must be asserted on the next ACLK cycle` now preserves:
  - `clock_signal = ACLK`
  - `edge = rising`
  - `cycle_window = 1..1`
- Added validator coverage proving `PREADY must be asserted on the next HCLK rising edge` no longer triggers `intent_temporal_rules_missing_cycle_windows`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge named_next_cycle_text` -> passed
- `cargo test -p specforge named_next_clock_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Named clock cycle/tick phrases now fully ground temporal rules)

### Improved: named cycle/tick timing now contributes both local `clock_signal` and default edge grounding
- `crates/specforge/src/ir/semantic.rs` now treats named clock cycle/tick phrases as fully grounded local clock-tick timing when the sentence explicitly names the clock signal.
- That means phrases like:
  - `same ACLK cycle`
  - `within 2 HCLK cycles`
  - `next PCLK tick`
  no longer stop at `clock_signal = <named clock>` while leaving `edge = unknown`
- They now inherit the same bounded rising-edge default already used for default-clock timing, but only when the current sentence explicitly names the clock signal.
- This closes an important half-grounded state:
  - named local clock text was already enough to recover the clock signal
  - but cycle/tick wording without an explicit edge still looked under-grounded to validation when no `Clock ...` declaration existed
  - the typed temporal model now treats explicit local clock cycle/tick phrasing the same way it already treated document-default cycle/tick phrasing

### Added: regression coverage for named cycle/tick grounding
- Added semantic coverage proving `TVALID must be asserted in the same ACLK cycle` now preserves:
  - `clock_signal = ACLK`
  - `edge = rising`
  - `cycle_window = 0..0`
- Added validator coverage proving named local cycle text no longer triggers `temporal_rules_missing_clock_grounding` even without a separate default clock declaration.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge named_cycle_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Temporal rules now ground signal-leading clock phrases)

### Improved: signal-leading clock phrases now preserve local `clock_signal` grounding too
- `crates/specforge/src/ir/semantic.rs` now recognizes signal-leading local clock phrases as explicit clock-signal grounding for typed temporal rules.
- That now covers both prose-style and shorthand ordering:
  - `HCLK rising edge`
  - `HCLK falling edge`
  - `HCLK posedge`
  - `HCLK negedge`
- This closes the next local-language gap in the clock model:
  - `rising edge of HCLK` was already grounded by the previous slice
  - but common signal-leading variants like `HCLK rising edge` and `HCLK posedge` still fell back to the default clock or to `None`
- The override remains bounded:
  - only explicit local timing phrases that actually name a clock signal are trusted
  - default-clock fallback still applies when the current sentence does not name a clock

### Added: regression coverage for signal-leading clock grounding
- Added semantic coverage proving `PREADY must be asserted on HCLK rising edge` now preserves:
  - `clock_signal = HCLK`
  - `edge = rising`
- Added validator coverage proving `PREADY must be asserted on HCLK posedge` no longer trips `temporal_rules_missing_clock_grounding` even without a separate default clock declaration.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge signal_leading_rising_edge_text` -> passed
- `cargo test -p specforge signal_leading_clock_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Temporal rules now preserve explicit local clock names from timing text)

### Improved: explicit local timing text now grounds the clock signal itself
- `crates/specforge/src/ir/semantic.rs` now treats explicit local clock names in timing prose as first-class temporal grounding instead of always inheriting the document default clock.
- The same bounded override now applies across signal-constraint, conditional-rule, and timing-constraint temporal synthesis:
  - `on the third rising edge of HCLK` now preserves `clock_signal = HCLK`
  - `same ACLK cycle` can now preserve `clock_signal = ACLK`
  - if no explicit local clock name is present, the existing default clock fallback still applies
- This closes a real typed-model gap:
  - the parser already recovered explicit windows and edges from named-clock prose
  - but the canonical temporal rule could still serialize the wrong clock source when a different default document clock existed
  - rules with explicit local clock names can now also count as fully grounded even when the document has no separate `Clock ...` declaration

### Added: regression coverage for explicit local clock grounding
- Strengthened ordinal-edge temporal derivation coverage so `PREADY must be asserted on the third rising edge of HCLK` now proves:
  - `clock_signal = HCLK`
  - `edge = rising`
  - `cycle_window = 3..3`
- Added validator coverage proving explicit local clock text is enough to avoid `temporal_rules_missing_clock_grounding` even without a separate default clock declaration.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge ordinal_rising_edge_constraint_text` -> passed
- `cargo test -p specforge explicit_clock_text` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Temporal shorthand edge grounding now preserves explicit edge semantics)

### Improved: symbolic edge shorthand now grounds the temporal rule edge, not just the cycle window
- `crates/specforge/src/ir/semantic.rs` now treats explicit shorthand edge language as first-class clock-edge grounding for signal-constraint and timing-derived temporal rules:
  - `next posedge` now preserves `edge = rising`
  - `next negedge` now preserves `edge = falling`
- This closes a real temporal-model gap:
  - bounded `cycle_window` recovery for shorthand edges was already working
  - but signal-constraint temporal rules were still silently inheriting the default clock edge instead of honoring the local shorthand
  - that meant `next negedge` could validate as "grounded" while still carrying the wrong edge semantics
- The fix is shared instead of one-off:
  - explicit edge detection is now reused across timing-constraint and signal-constraint temporal-rule derivation
  - conditional rules with explicit shorthand edge language inherit the same edge-grounding behavior too

### Added: regression coverage for shorthand edge grounding
- Added end-to-end `SemanticIR` temporal-rule derivation coverage for:
  - `PREADY must be asserted on the next negedge`
- Strengthened the existing shorthand-edge regression so `PREADY must be asserted on the next posedge` now also proves the recovered rule edge, not only the `cycle_window`.
- Added validator coverage proving `next negedge` no longer looks like a temporal rule with missing clock grounding.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge negedge_cycle_window` -> passed
- `cargo test -p specforge temporal_rules_missing_clock_grounding` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Temporal parser now recognizes posedge/negedge shorthand and diagram-style positions)

### Improved: built-in cycle-window recovery now covers symbolic edge shorthand and unit-first diagram labels
- `crates/specforge/src/ir/semantic.rs` now recognizes shorthand edge idioms such as:
  - `next posedge`
  - `next negedge`
- It also now recognizes unit-first diagram-style positions such as:
  - `tick T3`
  - `posedge T4`
- This closes the last obvious asymmetry in the current clock-tick parser:
  - `rising edge` / `falling edge` were already first-class timing units
  - counted `tick` / `edge` language had already become first-class in the prior slice
  - but common symbolic edge shorthand and unit-first diagram labels were still slipping past the built-in timing path
- The new support stays bounded and local:
  - current-document shorthand now maps directly onto the typed `CycleWindowRecord` surface
  - learned-only temporal idioms such as `one beat later` still remain prior-guided fallback, not built-in truth

### Added: regression coverage for shorthand edge and diagram-style position language
- Added direct parser coverage for:
  - `next posedge`
  - `next negedge`
  - `tick T3`
  - `posedge T4`
- Added end-to-end `SemanticIR` temporal-rule derivation coverage for:
  - `PREADY must be asserted on the next posedge`
  - `PREADY must be asserted at tick T3`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Temporal parser now recognizes quantified tick and edge units)

### Improved: built-in cycle-window recovery now treats counted tick and edge language like counted cycle language
- `crates/specforge/src/ir/semantic.rs` now recognizes quantitative tick and edge phrasing such as:
  - `within 2 ticks`
  - `after 3 falling edges`
  - `next falling edge`
- The quantitative branches now reuse the same cycle-like unit detector instead of hardcoding `cycle` / `cycles` in each branch, so bounded `tick`, `rising edge`, and `falling edge` language lands on the same typed `CycleWindowRecord` surface.
- Structured timing-constraint units now use that same cycle-like unit detector too, so numeric timing records with units like `ticks` no longer get stranded behind a cycle-only gate.
- Prior-guided timing recovery remains bounded and honest:
  - explicit local tick/edge language is handled directly by the built-in parser
  - learned-only idioms such as `one beat later` still remain prior-guided fallback, not built-in truth

### Added: regression coverage for quantified tick and edge timing language
- Added direct parser coverage for:
  - `within 2 ticks`
  - `after 3 falling edges`
  - `next falling edge`
- Added end-to-end `SemanticIR` temporal-rule derivation coverage for `PREADY must be asserted within 2 ticks`.
- Added direct timing-constraint-unit coverage proving `TimingConstraintRecord { typ_value: 2, unit: ticks }` now recovers a bounded `cycle_window`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Temporal parser now recognizes explicit later and ordinal edge phrases)

### Improved: built-in cycle-window recovery now covers more real protocol timing language
- `crates/specforge/src/ir/semantic.rs` now recognizes bounded later phrases such as `two cycles later` and ordinal edge phrases such as `on the third rising edge of HCLK`.
- The new timing forms land on the same typed `CycleWindowRecord` surface already used for `within 2 cycles`, `next tick`, and `same cycle`, so downstream `SemanticIR` / `IntentIR` temporal rules stay on one explicit clock-tick model instead of growing one-off heuristics.
- Ordinal parsing is intentionally bounded:
  - cardinal timing language still supports numeric and word forms
  - ordinal timing language now supports bounded numeric/word ordinals such as `3rd` and `third`
  - direct diagram-style `on T3` parsing stays conservative instead of widening positional matches everywhere
- Prior-guided timing recovery remains fallback-only:
  - local phrases like `one beat later` still do not become built-in timing truth
  - a validated temporal prior can still recover that pattern when current-document wording matches a learned prior exactly

### Added: regression coverage for explicit later and ordinal timing language
- Added direct parser coverage for:
  - `two cycles later`
  - `on the third rising edge of HCLK`
- Added end-to-end temporal-rule derivation coverage in `SemanticIR` for:
  - `PREADY must be asserted two cycles later`
  - `PREADY must be asserted on the third rising edge of HCLK`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge cycle_window` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Evidence-stage negative-knowledge caution now feeds replay planning)

### Improved: EvidenceIR caution is now a real replay target instead of a planner dead end
- Validation already emitted `evidence_negative_knowledge_rescan_guidance` when current EvidenceIR conflict or residual shapes matched learned caution patterns.
- That was honest, but still too passive:
  - reviewers could see the caution-linked ids
  - the replay planner ignored that evidence-stage finding entirely
  - the product said "be careful here" without a typed bounded next step
- This slice closes that gap without widening truth:
  - `project-validation` now consumes `evidence_negative_knowledge_rescan_guidance`
  - the replay contract is EvidenceIR-local in scope, not canonical-stage by implication: `SourceIR -> EvidenceIR -> validate`
  - the action text now says "related evidence conflict or residual ids" instead of implying those caution targets already escaped into downstream canonical stages
  - `evidence_input_for_snapshot_stage()` now accepts current EvidenceIR artifacts directly, so evidence-stage replay inputs can be derived from the persisted artifact path instead of requiring downstream replay metadata
  - the new regression `project_validation_collects_evidence_negative_knowledge_rescan_guidance` locks the replay inputs, action summary, and command hints for this narrower caution lane

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_collects_negative_knowledge_rescan_guidance` -> passed
- `cargo test -p specforge project_validation_collects_evidence_negative_knowledge_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-21 (Actor-port gaps now emit replayable rescan guidance)

### Improved: relation-only actor graph remnants now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried `actor_signal_relations` but no `actor_ports`.
- That was honest, but still too passive:
  - reviewers could see the stranded `asr_*` relation ids
  - the replay planner had no typed next step for revisiting those same actor-relative graph remnants
  - the gap looked like static canonical graph debt instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used by nearby canonical graph gaps:
  - `validate` now emits `semantic_actor_port_gap_surface_rescan_guidance` / `intent_actor_port_gap_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same actor-signal relation ids already reported by `semantic_actor_ports_missing` / `intent_actor_ports_missing`
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related actor-signal relation ids into actor-relative port direction records instead of leaving them as relation-only graph evidence
  - `kg-bench` now supports `semantic_ir_patch.clear_actor_ports`, and the new tracked fixture `actor_port_gap_surface_negative` locks the replay guidance against a relation-present, actor-port-missing canonical artifact

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_actor_port_gap_related_ids` -> passed
- `cargo test -p specforge project_validation_collects_actor_port_gap_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `428` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Source-stage missing VLM enrichment now emits replayable rescan guidance)

### Improved: classified SourceIR timing/state diagrams without VLM enrichment now advertise the next bounded replay
- Validation already reported when `SourceIR` carried timing/state diagrams that were still missing VLM enrichment.
- That was honest, but still too passive:
  - reviewers could see the stranded diagram asset ids
  - the replay planner had no source-stage next step for revisiting those same visuals
  - the gap looked like passive ingest-side enrichment debt instead of an explicit replay target
- This slice turns that weak state into a bounded source-stage replay contract:
  - `validate` now emits `source_vlm_enrichment_missing_surface_rescan_guidance`
  - that finding stays in `rescan_guidance` and carries the same asset ids already reported by `source_vlm_enrichment_missing`
  - `project-validation` maps it onto a local `enrich -> validate` replay lane against the current `SourceIR` artifact
  - the extractor lane is now explicit too: `source_ir_visual_enrichment_rescan`
  - `kg-bench` now supports `validation.source`, so tracked fixtures can lock SourceIR validation surfaces directly
  - the new tracked fixture `source_vlm_enrichment_surface_negative` now requires the new source-stage replay guidance so the benchmark locks that follow-up contract too

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_source_ir_reports_missing_vlm_enrichment_related_ids` -> passed
- `cargo test -p specforge project_validation_collects_source_vlm_enrichment_missing_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `426` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Evidence-stage missing VLM observations now emit replayable rescan guidance)

### Improved: visual evidence without extracted timing/state observations now advertises the next bounded replay
- Validation already reported when `EvidenceIR` carried visual evidence but no VLM timing/state observations.
- That was honest, but still too passive:
  - reviewers could see the stranded visual ids
  - the replay planner had no explicit source-side next step for revisiting those same visuals
  - the gap looked like passive visual-enrichment debt instead of an explicit replay target
- This slice turns that weak state into a bounded evidence-stage replay contract:
  - `validate` now emits `evidence_missing_vlm_observations_surface_rescan_guidance`
  - that finding stays in `rescan_guidance` and carries the same visual ids already reported by `evidence_missing_vlm_observations`
  - `project-validation` maps it onto a local `enrich -> evidence -> validate` replay lane rooted at the current `SourceIR`
  - the action text is explicit about gaining timing/state observations for the related visual ids before treating the current EvidenceIR visual surface as good enough
  - the new tracked fixture `evidence_missing_vlm_observations_surface_negative` now requires the new evidence-stage replay guidance so the benchmark locks that follow-up contract too

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_reports_missing_vlm_observation_related_ids` -> passed
- `cargo test -p specforge project_validation_collects_evidence_missing_vlm_observations_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `424` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Evidence-stage structural-KG gaps now emit replayable rescan guidance)

### Improved: stranded EvidenceIR behavioral records now advertise the next bounded replay
- Validation already reported when `EvidenceIR` carried behavioral records but no structural actor-signal graph.
- That was honest, but still too passive:
  - reviewers could see the stranded behavioral ids
  - the replay planner had no evidence-local next step for revisiting those same ungrounded records
  - the gap looked like passive extraction debt instead of an explicit replay target
- This slice turns that weak state into a bounded evidence-local replay contract:
  - `validate` now emits `evidence_structural_kg_missing_surface_rescan_guidance`
  - that finding stays in `rescan_guidance` and carries the same behavioral ids already reported by `evidence_structural_kg_missing`
  - `project-validation` maps it onto a local `nlp-enrich -> validate` replay lane against the current `EvidenceIR` artifact
  - the action text is explicit about collapsing the related behavioral ids into actor-grounded graph relations before any downstream canonical rebuild is considered
  - the new tracked fixture `evidence_structural_kg_missing_surface_negative` now requires the new evidence-stage replay guidance so the benchmark locks that follow-up contract too

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_reports_structural_kg_and_normative_related_ids` -> passed
- `cargo test -p specforge project_validation_collects_evidence_structural_kg_missing_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `422` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Evidence-stage normative residuals now emit replayable rescan guidance)

### Improved: partially structured EvidenceIR normative statements now advertise the next bounded replay
- Validation already reported when `EvidenceIR` still carried partially structured normative statements.
- That was honest, but still too passive:
  - reviewers could see the preserved statement ids
  - the replay planner had no evidence-local next step for revisiting those same unstructured obligations
  - the residual looked like passive extraction debt instead of an explicit replay target
- This slice turns that weak state into a bounded evidence-local replay contract:
  - `validate` now emits `evidence_normative_residual_surface_rescan_guidance`
  - that finding stays in `rescan_guidance` and carries the same statement ids already reported by `evidence_normative_residuals_remaining`
  - `project-validation` maps it onto a local `nlp-enrich -> validate` replay lane against the current `EvidenceIR` artifact
  - the action text is explicit about collapsing the related statement ids into typed constraints, rules, or structured evidence before any downstream canonical rebuild is considered
  - the new tracked fixture `evidence_normative_residual_surface_negative` now requires the new evidence-stage replay guidance so the benchmark locks that follow-up contract too

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_reports_structural_kg_and_normative_related_ids` -> passed
- `cargo test -p specforge project_validation_collects_evidence_normative_residual_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `421` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Evidence-stage signal-polarity conflicts now emit replayable rescan guidance)

### Improved: preserved evidence-stage active-level disagreement now advertises the next bounded replay
- Validation already reported when `EvidenceIR` carried `signal_polarity_conflicts`.
- That was honest, but still too passive:
  - reviewers could see the preserved `polarity_conflict_*` ids
  - the replay planner had no evidence-local next step for revisiting those same contradictions
  - the disagreement looked like a passive upstream warning instead of an explicit replay target
- This slice turns that weak state into a bounded evidence-local replay contract:
  - `validate` now emits `evidence_signal_polarity_conflict_surface_rescan_guidance`
  - that finding stays in `rescan_guidance` and carries the same `polarity_conflict_*` ids already reported by the evidence-stage conflict finding
  - `project-validation` maps it onto a local `nlp-enrich -> validate` replay lane against the current `EvidenceIR` artifact
  - the action text is explicit about collapsing the related conflict ids toward one locally corroborated active-level interpretation before any downstream canonical rebuild is considered
  - the tracked fixtures `control_polarity_conflict_negative` and `negative_knowledge_prior_guided_polarity_conflict_caution_gold` now require the new evidence-stage replay guidance as well

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_flags_signal_polarity_conflicts` -> passed
- `cargo test -p specforge project_validation_collects_evidence_signal_polarity_conflict_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `420` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Evidence-stage signal-semantic conflicts now emit replayable rescan guidance)

### Improved: preserved evidence-stage role disagreement now advertises the next bounded replay
- Validation already reported when `EvidenceIR` carried `signal_semantic_conflicts`.
- That was honest, but still too passive:
  - reviewers could see the preserved `semantic_conflict_*` ids
  - the replay planner had no evidence-local next step for revisiting those same contradictions
  - the disagreement looked like a passive upstream warning instead of an explicit replay target
- This slice turns that weak state into a bounded evidence-local replay contract:
  - `validate` now emits `evidence_signal_semantic_conflict_surface_rescan_guidance`
  - that finding stays in `rescan_guidance` and carries the same `semantic_conflict_*` ids already reported by the evidence-stage conflict finding
  - `project-validation` maps it onto a local `nlp-enrich -> validate` replay lane against the current `EvidenceIR` artifact
  - the action text is explicit about collapsing the related conflict ids toward one locally corroborated role meaning instead of merely rerunning extraction blindly
  - the tracked fixtures `cross_modality_semantic_conflict_negative` and `negative_knowledge_prior_guided_semantic_conflict_caution_gold` now require the new evidence-stage replay guidance as well

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_flags_signal_semantic_conflicts` -> passed
- `cargo test -p specforge project_validation_collects_evidence_signal_semantic_conflict_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `419` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Signal-semantic conflicts now emit replayable rescan guidance)

### Improved: preserved semantic-role disagreement now advertises the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried `signal_semantic_conflicts`.
- That was honest, but still too passive:
  - reviewers could see the preserved `semantic_conflict_*` ids
  - the replay planner had no conflict-id-specific next step for revisiting local role evidence on those same contradictions
  - semantic-role disagreement looked like a static warning instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_signal_semantic_conflict_surface_rescan_guidance` / `intent_signal_semantic_conflict_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same `semantic_conflict_*` ids already reported by the semantic-conflict finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related conflict ids toward one locally corroborated role meaning instead of merely rerunning extraction
  - the tracked fixtures `cross_modality_semantic_conflict_negative` and `negative_knowledge_prior_guided_semantic_conflict_caution_gold` now require the new semantic/intent replay guidance so the disagreement case is benchmark-locked both with and without prior-memory caution

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge signal_semantic_conflict` -> passed
- `cargo test -p specforge project_validation_collects_signal_semantic_conflict_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `418` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Graph-direction self-conflicts now emit replayable rescan guidance)

### Improved: preserved same-actor direction disagreement now advertises the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried same-actor `graph_direction_conflicts`.
- That was honest, but still too passive:
  - reviewers could see the preserved actor-aware conflict ids
  - the replay planner had no typed next step for revisiting local actor/role language on those same contradictions
  - graph-direction self-conflict looked like a static warning instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_graph_direction_conflict_surface_rescan_guidance` / `intent_graph_direction_conflict_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same actor-aware conflict ids already reported by the graph-direction-conflict finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related conflict ids toward one actor-relative direction per actor-signal edge instead of merely rerunning extraction
  - the tracked fixture `graph_direction_same_actor_conflict_negative` now requires the new semantic/intent replay guidance so the disagreement case is benchmark-locked end to end

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_collects_graph_direction_conflict_rescan_guidance` -> passed
- `cargo test -p specforge conflicting_same_actor_graph_direction` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `416` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-21 (Signal-polarity conflicts now emit replayable rescan guidance and learnable caution)

### Improved: preserved polarity disagreement now advertises the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried `signal_polarity_conflicts` such as prose-vs-table active-level disagreement.
- That was honest, but still too passive:
  - reviewers could see the preserved conflict ids
  - the replay planner had no typed next step for revisiting local polarity evidence on those same conflicts
  - polarity disagreement looked like a static warning instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_signal_polarity_conflict_surface_rescan_guidance` / `intent_signal_polarity_conflict_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same conflict ids already reported by the polarity-conflict finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related conflict ids toward a single locally corroborated active-level interpretation rather than merely rerunning extraction
  - the tracked fixture `control_polarity_conflict_negative` now requires the new semantic/intent replay guidance so the disagreement case is benchmark-locked even without prior-memory caution

### Improved: negative-knowledge caution now remembers polarity-conflict archetypes too
- `CorpusMemory` negative-knowledge priors can now carry `signal_polarity_conflict` patterns.
- `learn-priors` now harvests those patterns from validated `IntentIR` polarity conflicts.
- `validate` now matches those learned patterns in `EvidenceIR`, `SemanticIR`, and `IntentIR`, surfacing the existing advisory `*_negative_knowledge_prior_matches` and `*_negative_knowledge_rescan_guidance` findings without mutating current-document truth.
- The new tracked fixture `negative_knowledge_prior_guided_polarity_conflict_caution_gold` locks that caution path end to end, including exact related ids for the evidence-stage prior match and the semantic/intent replay guidance.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge signal_polarity_conflict` -> passed
- `cargo test -p specforge negative_knowledge_polarity` -> passed
- `cargo test -p specforge project_validation_collects_signal_polarity_conflict_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `414` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-20 (Temporal conflicts now emit replayable rescan guidance)

### Improved: preserved timing contradictions now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried `temporal_conflicts` such as contradictory `HIGH` / `LOW` obligations under the same grounded timing context.
- That was honest, but still too passive:
  - reviewers could see the preserved conflict ids
  - the replay planner had no typed next step for revisiting local timing evidence on those same contradictions
  - temporal contradiction looked like a static warning instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_temporal_conflict_surface_rescan_guidance` / `intent_temporal_conflict_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same conflict ids already reported by the temporal-conflict finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related conflict ids toward a single locally corroborated timing obligation rather than merely rerunning extraction
  - the tracked fixtures `temporal_conflict_negative` and `negative_knowledge_prior_guided_temporal_conflict_caution_gold` now require the new semantic/intent replay guidance so the contradiction case is benchmark-locked both with and without prior-memory caution

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge temporal_conflict` -> passed
- `cargo test -p specforge project_validation_collects_temporal_conflict_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed

## 2026-04-20 (Interface conflicts now emit replayable rescan guidance)

### Improved: preserved direction/width disagreement now advertises the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried `interface_signal_conflicts` such as direction or width mismatches.
- That was honest, but still too passive:
  - reviewers could see the preserved conflict ids
  - the replay planner had no typed next step for revisiting local declaration evidence on those same conflicts
  - interface disagreement looked like a static warning instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_interface_signal_conflict_surface_rescan_guidance` / `intent_interface_signal_conflict_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same conflict ids already reported by the interface-conflict finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related conflict ids toward consistent direction/width declarations rather than merely rerunning extraction
  - the tracked fixtures `interface_signal_conflict_negative` and `negative_knowledge_prior_guided_interface_conflict_caution_gold` now require the new semantic/intent replay guidance so the disagreement case is benchmark-locked both with and without prior-memory caution

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge interface_signal_conflict` -> passed
- `cargo test -p specforge signal_connectivity_conflict_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed

## 2026-04-20 (Signal-connectivity conflicts now emit replayable rescan guidance)

### Improved: preserved producer-ambiguity conflicts now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried `signal_connectivity_conflicts` such as multi-producer ambiguity.
- That was honest, but still too passive:
  - reviewers could see the preserved conflict ids
  - the replay planner had no typed next step for revisiting local actor/role evidence on those same conflicts
  - producer ambiguity looked like a static warning instead of an explicit replay target
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_signal_connectivity_conflict_surface_rescan_guidance` / `intent_signal_connectivity_conflict_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same conflict ids already reported by the connectivity-conflict finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about collapsing the related conflict ids toward single-producer connectivity rather than merely rerunning extraction
  - the tracked fixtures `multi_producer_conflict_negative` and `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` now require the new semantic/intent replay guidance so the ambiguity case is benchmark-locked both with and without prior-memory caution

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge signal_connectivity_conflict` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `400` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-19 (Connectivity endpoint gaps now emit replayable rescan guidance)

### Improved: producerless and consumerless protocol signals now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried protocol `signal_connectivity` records with only one endpoint:
  - a consumer but no producer
  - or a producer but no consumer
- That was honest, but still too passive:
  - reviewers could see the affected signal ids
  - the replay planner had no typed next step for revisiting local actor/role evidence on those same signals
  - endpoint-gapped signals looked like passive structural debt instead of explicit replay targets
- This slice turns that weak state into the same bounded replay contract used for nearby evidence-strength gaps:
  - `validate` now emits `semantic_connectivity_missing_producer_surface_rescan_guidance` / `intent_connectivity_missing_producer_surface_rescan_guidance`
  - `validate` now also emits `semantic_connectivity_missing_consumer_surface_rescan_guidance` / `intent_connectivity_missing_consumer_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same signal ids already reported by the endpoint-gap findings
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about gaining producer-side or consumer-side connectivity evidence rather than merely rerunning extraction
  - the tracked KG fixture `connectivity_endpoint_gaps_negative` now requires both semantic and intent guidance so the benchmark locks the replay surface end to end
- The scope stays intentionally narrow:
  - protocol connectivity endpoint gaps are replay targets
  - infrastructure clock/reset sourcing is not
  - missing infrastructure producers remain a dedicated `system_contract` note because many protocol PDFs do not name the final clock generator or reset controller

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge connectivity_endpoint_related_ids` -> passed
- `cargo test -p specforge connectivity_missing_` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `397` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-19 (Graph-direction coverage gaps now emit replayable rescan guidance)

### Improved: graph-uncovered canonical signals now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried canonical signal records that still lacked actor-relative graph direction coverage even though the actor-port graph was non-empty.
- That was truthful, but still too passive:
  - reviewers could see the affected signal ids
  - the replay planner had no typed next step for revisiting local actor/role evidence on those same signals
  - graph-uncovered signals looked like canonical graph debt instead of an explicit replay surface
- This slice turns that weak state into the same bounded replay contract used elsewhere:
  - `validate` now emits `semantic_graph_direction_coverage_surface_rescan_guidance` / `intent_graph_direction_coverage_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same signal ids already reported by the missing graph-direction finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about recovering actor-relative graph direction coverage rather than merely rerunning extraction
  - the tracked KG fixture `graph_direction_coverage_incomplete_negative` now requires the graph-direction guidance so the graph-gap case is benchmark-locked end to end

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_reports_missing_graph_direction_related_ids` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_missing_graph_direction_related_ids` -> passed
- `cargo test -p specforge graph_direction_coverage_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `393` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Temporal actor-grounding gaps now emit replayable rescan guidance)

### Improved: actorless typed temporal rules now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried typed temporal rules but none of them had actor-relative drive/sample grounding even though a non-empty actor graph existed.
- That was truthful, but still too passive:
  - reviewers could see the affected temporal rule ids
  - the replay planner had no typed next step for revisiting local timing language on those same rules
  - actorless rules looked like a scoring artifact instead of an explicit replay surface
- This slice turns that weak state into the same bounded replay contract used elsewhere:
  - `validate` now emits `semantic_temporal_actor_grounding_surface_rescan_guidance` / `intent_temporal_actor_grounding_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same temporal rule ids already reported by the missing-actor-grounding finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about recovering actor-relative drive/sample grounding rather than merely rerunning extraction
  - the tracked KG fixture `temporal_actor_grounding_surface_negative` now requires the actor-grounding guidance and metric so the actorless-but-clocked temporal case is benchmark-locked

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_temporal_gap_related_ids` -> passed
- `cargo test -p specforge temporal_actor_grounding_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `389` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Temporal clock-grounding gaps now emit replayable rescan guidance)

### Improved: clockless typed temporal rules now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried typed temporal rules that still lacked explicit clock or edge grounding.
- That was truthful, but still too passive:
  - reviewers could see the affected temporal rule ids
  - the replay planner had no typed next step for revisiting local timing language on those same rules
  - clockless rules looked like a scoring artifact instead of an explicit replay surface
- This slice turns that weak state into the same bounded replay contract used elsewhere:
  - `project-validation` now recognizes `semantic_temporal_clock_grounding_surface_rescan_guidance` / `intent_temporal_clock_grounding_surface_rescan_guidance`
  - those findings map onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about recovering clock or edge grounding rather than merely rerunning extraction
  - the tracked KG fixture `temporal_clock_grounding_surface_negative` now requires the clock-grounding guidance and metric so the clockless timing case is benchmark-locked without overclaiming against already clock-grounded fixtures

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_temporal_gap_related_ids` -> passed
- `cargo test -p specforge temporal_clock_grounding_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `387` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Temporal cycle-window gaps now emit replayable rescan guidance)

### Improved: unbounded typed temporal rules now advertise the next bounded replay
- Validation already reported when `SemanticIR` or `IntentIR` carried typed temporal rules but none of them had explicit `cycle_window` bounds.
- That was honest, but still too passive:
  - reviewers could see the affected temporal rule ids
  - the replay planner had no typed next step for revisiting local timing language on those same rules
  - unbounded rules looked like a score artifact instead of an explicit review surface
- This slice turns that gap into the same bounded replay contract used elsewhere:
  - `validate` now emits `semantic_temporal_cycle_window_surface_rescan_guidance` / `intent_temporal_cycle_window_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same temporal rule ids already reported by the missing-cycle-window finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicit about recovering `cycle_window` bounds rather than merely rerunning extraction
- The tracked KG fixture `temporal_prior_guided_cycle_window_without_prior_negative` now also requires those new findings, so the no-prior unbounded timing case is benchmark-locked.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_temporal_gap_related_ids` -> passed
- `cargo test -p specforge temporal_cycle_window_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `385` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Prior-guided semantic consensus now emits replayable rescan guidance)

### Improved: learned-prior-assisted final role meaning now points back to current-document replay
- Validation already surfaced when `SemanticIR` or `IntentIR` carried semantic-role consensus that had been strengthened by learned modality-reliability priors.
- That state was honest, but still too inert:
  - reviewers could see the prior-guided signal ids
  - the replay planner had no typed next step for seeking stronger current-document corroboration
  - the final consensus looked more settled than the policy really allows
- This slice makes that weakness actionable without changing the bounded replay contract:
  - `validate` now emits `semantic_prior_guided_semantic_consensus_surface_rescan_guidance` / `intent_prior_guided_semantic_consensus_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same signal ids already named by the prior-guided consensus finding
  - `project-validation` maps them onto the existing local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the action text is explicitly about gaining stronger current-document semantic-role consensus instead of relying on prior-guided carry-through
- The tracked KG fixture `semantic_modality_reliability_prior_guided_conflict_gold` now also requires those new findings, so the positive prior-guided semantic case is benchmark-locked.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_prior_guided_semantic_related_ids` -> passed
- `cargo test -p specforge prior_guided_semantic_consensus_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `383` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Alias-dependent semantic consensus now emits replayable rescan guidance)

### Improved: alias-grounded role meaning now advertises the next bounded evidence replay
- Validation already surfaced when `SemanticIR` or `IntentIR` carried resolved semantic-role consensus that still depended only on alias-grounded evidence.
- That was truthful, but it still left an avoidable gap:
  - reviewers could see which signal ids were weaker
  - the replay planner still had no typed next move for trying to strengthen those roles from current-document evidence
  - alias-grounded meaning looked more static than it should
- This slice routes that state into the same conservative replay machinery already used for other semantic evidence gaps:
  - `validate` now emits `semantic_alias_dependent_semantic_consensus_surface_rescan_guidance` / `intent_alias_dependent_semantic_consensus_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the same alias-dependent signal ids already reported by the consensus finding
  - `project-validation` maps them onto the local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the recommended action is phrased in terms of gaining direct or corroborating non-alias semantic-role consensus, not merely rerunning extraction blindly
- The tracked KG fixture `alias_dependent_handshake_completion_caveat` now also requires those new findings so the alias-grounded handshake case is benchmark-locked end-to-end.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_alias_dependent_semantic_consensus` -> passed
- `cargo test -p specforge validate_intent_ir_reports_alias_dependent_semantic_consensus` -> passed
- `cargo test -p specforge alias_dependent_semantic_consensus_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `381` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Fallback semantic-role consensus now emits replayable rescan guidance)

### Improved: provisional semantic meaning now routes into the bounded replay loop
- Validation already reported when `SemanticIR` or `IntentIR` carried a resolved semantic role without observation-backed `semantic_consensus`.
- That was honest, but still incomplete:
  - the canonical layers preserved the provisional meaning
  - the validator named the affected signal ids
  - the replay planner still had no typed next step for trying to strengthen that meaning from current-document evidence
- This slice closes that gap without turning the system into an auto-fixer:
  - `validate` now emits `semantic_role_consensus_surface_rescan_guidance` / `intent_role_consensus_surface_rescan_guidance`
  - those findings stay in `rescan_guidance` and carry the exact fallback-only signal ids already exposed by the consensus warning
  - `project-validation` now maps those findings onto the existing local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane
  - the recommended action text is consensus-specific: the operator is asked to check whether those signals gain observation-backed semantic-role consensus, not merely whether any role survives
- The behavior remains deliberately bounded:
  - no canonical fact is auto-promoted
  - no semantic truth is mutated in place
  - the replay plan only identifies the current-document evidence boundary and the deterministic downstream rebuild path

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_ir_flags_resolved_roles_without_consensus` -> passed
- `cargo test -p specforge validate_intent_ir_flags_resolved_roles_without_consensus` -> passed
- `cargo test -p specforge semantic_role_consensus_rescan_guidance` -> passed
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `bash scripts/run_ci.sh` -> passed with `379` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (KG fixtures now lock semantic-arbitration rescan guidance)

### Improved: tracked semantic-conflict fixtures now require the new replay guidance findings
- The previous slice added replay-oriented rescan guidance for non-decisive semantic-role arbitration.
- Unit coverage proved the validator and `project-validation` planner behavior, but the tracked KG benchmark fixtures had not yet been updated to require those new findings on realistic staged documents.
- This follow-on closes that gap by strengthening representative contested-semantic fixtures:
  - `visual_sources_semantic_conflict_negative`
  - `cross_modality_semantic_conflict_negative`
  - `negative_knowledge_prior_guided_semantic_conflict_caution_gold`
  - `contested_handshake_name_fallback_negative`
- Those fixtures now require:
  - `semantic_role_arbitration_surface_rescan_guidance` at the semantic stage
  - `intent_role_arbitration_surface_rescan_guidance` at the intent stage
  - `rescan_guidance` category plus signal-specific related ids such as `XCTRL` or `XVALID`
- Result:
  - the new replay guidance is no longer protected only by unit tests
  - tracked corpus regressions now prove the realistic semantic-conflict and blocked-handshake cases surface the same bounded rescan story end-to-end

### Validation
- `cargo test -p specforge kg_bench_runs_tracked_fixtures` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Non-decisive semantic arbitration now emits replayable rescan guidance)

### Improved: contested semantic-role arbitration now routes into the bounded local rescan loop
- The validator already surfaced non-decisive semantic-role arbitration honestly:
  - competing role candidates stayed visible
  - canonical artifacts preserved lead, runner-up, and evidence margin
  - handshake-name fallback stayed blocked when the role state was still contested
- But that state still stopped at observation.
- Operators could see the contested signals, but the replay planner had no typed guidance for what bounded next move should be attempted.
- This slice adds that missing bridge:
  - `validate` now emits stage-specific `rescan_guidance` findings when semantic-role arbitration remains non-decisive at `SemanticIR` or `IntentIR`
  - the related ids are the contested signal names already exposed by the arbitration finding
  - `project-validation` now turns those findings into replay-oriented recommendations
  - the replay contract uses the existing local NLP lane:
    - `SemanticIR`: `evidence_ir -> nlp-enrich -> semantic -> validate`
    - `IntentIR`: `evidence_ir -> semantic_ir -> nlp-enrich -> semantic -> intent -> validate`
- The action text is specialized too:
  - it explicitly says the goal is to see whether the related signals converge toward a decisive semantic-role outcome
- Result:
  - contested semantic arbitration is no longer only a passive review warning
  - it now has the same bounded, local, replayable follow-up shape as the other rescan-guidance families
  - canonical truth still stays untouched until current-document evidence and review say otherwise

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_ir_flags_signal_semantic_conflicts` -> passed
- `cargo test -p specforge validate_intent_ir_counts_multiple_semantic_candidates_for_conflicts` -> passed
- `cargo test -p specforge semantic_role_arbitration_rescan_guidance` -> passed with the new semantic/intent `project-validation` replay tests
- `bash scripts/run_ci.sh` -> passed with `377` Rust tests, rustdoc, and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Book and README now describe replay-scope-aware rescan review)

### Improved: public docs now explain the richer rescan review surfaces
- The last two rescan slices changed what operators actually see:
  - `project-validation` live-status projections now show a replay-input kind chain plus a concise action summary
  - `rescan-plan` dry-run output now shows `replay_inputs`, `recommended_action`, and `automation_status` before the command hints
- Those were meaningful user-facing workflow changes, but the public documentation still described the older, thinner review surfaces.
- This slice brings the docs back into sync:
  - `README.md` now says the compact live queue exposes replay scope and the dry-run preview surfaces the full replay/action status
  - the mdBook pages for `project-validation`, `rescan-plan`, validation, and generated artifacts now explain the same review behavior
- Result:
  - the public-facing book now matches the real operator workflow
  - future sessions and external readers do not have to infer the replay-scope review model from code or internal continuity notes

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `375` Rust tests, rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-19 (Rescan review surfaces now show the real replay boundary)

### Improved: compact rescan projections and dry-run previews now expose replay inputs plus action text
- The recent rescan slices made the plan itself much richer:
  - `replay_inputs` now say which upstream artifacts a replay actually needs
  - `recommended_action` now specializes by finding family instead of staying stage-generic
- But two operator-facing surfaces still lagged behind:
  - the compact targeted-rescan queue in `LIVE_ACHIEVEMENT_STATUS.md`
  - the `specforge rescan-plan` dry-run preview
- Both still mostly read like shorthand:
  - extractor lane
  - related ids
  - command-count summary
- That was enough to know a target existed, but not enough to understand the replay boundary without opening the JSON plan or the full validation snapshot.
- This slice makes those review surfaces more honest:
  - the live projected rescan queue now shows the replay-input kind chain and a concise action summary beside each recommendation
  - the `rescan-plan` dry-run renderer now prints full `replay_inputs`, `recommended_action`, and `automation_status` before the command hints
- Result:
  - the compact review plane now better matches the schema-v2 replay contract
  - a human can see what would be replayed and why without leaving the first-line review surface

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_collects_negative_knowledge_rescan_guidance` -> passed
- `cargo test -p specforge rescan_plan_dry_run_render_surfaces_replay_boundary_and_action` -> passed
- `bash scripts/run_ci.sh` -> passed with `375` Rust tests, rustdoc, and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Rescan guidance text now matches the replay contract)

### Improved: human-facing `recommended_action` text now specializes along with `replay_inputs` and command plans
- The recent rescan slices made the recommendation payload operationally precise:
  - `replay_inputs` now surface the actual upstream artifacts a replay needs
  - `recommended_commands` now describe the structured replay plan to execute
- But one field still lagged behind that stronger contract:
  - `recommended_action` was still chosen only from the artifact stage
  - so specialized rescans could carry the right command plan while still describing the work with a generic same-stage sentence
- That was honest enough to execute, but weaker than it should be for human review because the markdown-facing summary did not fully explain the real replay boundary.
- This slice makes the action text finding-aware in the same way the rest of the recommendation already is:
  - negative-knowledge canonical rescans now explicitly say to replay from `SourceIR` through the downstream canonical stage before re-validating
  - temporal-rule-surface gaps now explicitly say to run local NLP enrichment on `EvidenceIR` before rebuilding the canonical stages
  - visual corroboration rescans now explicitly say to rerun local visual enrichment from `SourceIR` and rebuild `EvidenceIR`
- Result:
  - the human-facing summary now says the same thing as the typed replay contract
  - snapshot docs, live status projections, and future automation all point at the same bounded rescan story

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_collects_negative_knowledge_rescan_guidance` -> passed
- `cargo test -p specforge project_validation_collects_visual_motif_rescan_guidance` -> passed
- `cargo test -p specforge project_validation_collects_temporal_rule_surface_rescan_guidance` -> passed
- `bash scripts/run_ci.sh` -> passed with `374` Rust tests, rustdoc, and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Rescan execution state now follows the replay contract)

### Improved: prior rescan execution summaries no longer carry across materially different replay plans
- `project-validation` already preserves previous `rescan-plan` execution state when a refreshed recommendation still refers to the same logical work item.
- The weakness was that the matching key only covered:
  - document/stage/artifact
  - finding id
  - extractor lane
  - related ids
- That was too loose once replay planning became finding-aware:
  - a recommendation could now change from a generic same-stage rebuild into a richer upstream replay plan
  - but an older `executed_validated_*` state could still be merged onto the refreshed recommendation because the key ignored both `replay_inputs` and `recommended_commands`
- This slice tightens that execution-state merge boundary:
  - the recommendation key now fingerprints normalized `replay_inputs`
  - and also fingerprints the structured command plan (`intent`, `executable`, `working_directory`, `args`)
- Result:
  - matching replay contracts still preserve execution summaries
  - replay-contract changes now invalidate the old execution state and leave the refreshed recommendation at `planned_not_executed`
- Added a direct regression that simulates an old intent negative-knowledge recommendation with the former generic `semantic_ir -> intent -> validate` contract and proves it no longer matches the current upstream replay contract.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_preserves_matching_rescan_execution_summary` -> passed
- `cargo test -p specforge project_validation_does_not_preserve_execution_summary_when_replay_contract_changes` -> passed
- `bash scripts/run_ci.sh` -> passed with `374` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Rescan replay inputs now match the real replay boundary)

### Improved: schema-v2 `replay_inputs` now surface the actual upstream artifacts used by specialized rescans
- The recent replay-planning slices made `recommended_commands` finding-aware:
  - temporal-surface gaps now emit `nlp-enrich -> semantic -> intent? -> validate`
  - semantic/intent negative-knowledge matches now emit upstream `evidence -> semantic -> intent? -> validate`
- But the recommendation payload still had one weak spot:
  - `recommended_commands` could name the right upstream replay path
  - while `replay_inputs` still mostly reflected the artifact stage's default direct input
  - so a schema-v2 rescan recommendation could be operationally correct but descriptively incomplete
- This slice makes `replay_inputs` finding-aware too:
  - semantic negative-knowledge guidance now lists `source_ir` and `evidence_ir`
  - intent negative-knowledge guidance now lists `source_ir`, `evidence_ir`, and `semantic_ir`
  - semantic temporal-surface guidance now lists `evidence_ir`
  - intent temporal-surface guidance now lists `evidence_ir` and `semantic_ir`
- That keeps the plan honest and self-describing:
  - the typed replay inputs now match the actual commands we ask the executor or a human reviewer to run
  - the schema-v2 plan remains a better future automation boundary because the replay contract is explicit even before command execution
- Tightened the projection regressions to assert those richer replay inputs directly for the intent negative-knowledge and semantic/intent temporal-surface cases.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_collects_negative_knowledge_rescan_guidance` -> passed
- `cargo test -p specforge project_validation_collects_temporal_rule_surface_rescan_guidance` -> passed
- `bash scripts/run_ci.sh` -> passed with `373` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Negative-knowledge rescans now restart upstream)

### Improved: semantic and intent negative-knowledge guidance now replay from the EvidenceIR boundary
- The previous rescan-plan shape for negative-knowledge findings was too weak at the canonical stages:
  - `semantic_negative_knowledge_rescan_guidance` only emitted `semantic -> validate`
  - `intent_negative_knowledge_rescan_guidance` only emitted `intent -> validate`
- That was objectively flimsy because those same-stage rebuilds do not create any new local corroboration:
  - the negative-knowledge priors are consulted upstream through the evidence path
  - the review question is whether the current document still reproduces the conflict/residual when replayed from the upstream extraction boundary
  - replaying only the current stage mostly re-serializes already-lowered state
- This slice tightens that replay plan for the canonical stages:
  - semantic negative-knowledge guidance now emits `evidence -> semantic -> validate`
  - intent negative-knowledge guidance now emits `evidence -> semantic -> intent -> validate`
- The planner resolves those replay inputs from persisted upstream artifacts:
  - for `SemanticIR`, load the tracked `EvidenceIR` and recover its `source_ir_path`
  - for `IntentIR`, load the tracked `SemanticIR`, then the upstream `EvidenceIR`, then recover its `source_ir_path`
- The behavior remains bounded and honest:
  - no new command types were needed in `rescan-plan`
  - no canonical truth is auto-mutated
  - if the persisted upstream artifacts cannot be loaded, the planner falls back to the generic rebuild path instead of emitting an unusable recommendation
- Updated the negative-knowledge projection regression to build real upstream artifacts and prove the stronger intent-stage replay plan now carries:
  - `generated/source_ir/spec/source_ir.json`
  - `generated/evidence_ir/spec/evidence_ir.json`
  - `generated/semantic_ir/spec/semantic_ir.json`
  - four total command hints instead of two

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge project_validation_collects_negative_knowledge_rescan_guidance` -> passed
- `cargo test -p specforge project_validation_collects_temporal_rule_surface_rescan_guidance` -> passed
- `bash scripts/run_ci.sh` -> passed with `373` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Temporal-surface rescans now emit replayable NLP hints)

### Improved: temporal-rule-surface gaps now project executable local replay plans
- The validator already knew when timing/constraint evidence existed but no typed temporal rules were materialized.
- The unfinished follow-on was that this new `rescan_guidance` surface stopped at observation:
  - `validate` could emit `semantic_temporal_rule_surface_rescan_guidance`
  - `validate` could emit `intent_temporal_rule_surface_rescan_guidance`
  - but `project-validation` still treated them like generic rebuilds instead of the stronger local replay they actually need
- This slice closes that loop by projecting a typed rescan path for those findings:
  - `nlp_enrich_evidence_ir`
  - `rebuild_semantic_ir`
  - `rebuild_intent_ir` when the stranded gap is observed at `IntentIR`
  - `validate_current_artifact`
- The plan stays local-first and bounded:
  - the generated command hints only allow local VLM providers (`ollama`, `lmstudio`, or `skip`)
  - `rescan-plan` now explicitly parses and whitelists `nlp-enrich` hints instead of treating them as opaque cargo invocations
  - OpenAI-backed replay remains intentionally rejected for schema-v2 rescan execution
- Tightened the validator so the sibling temporal findings use the same capped related-id set:
  - `*_temporal_rule_surface_missing`
  - `*_temporal_rule_surface_rescan_guidance`
- Added direct coverage for:
  - semantic-stage temporal rescan projection
  - intent-stage temporal rescan projection that recovers the upstream `EvidenceIR` path from persisted `SemanticIR`
  - rescan-plan parsing of local `nlp_enrich_evidence_ir` hints
  - rejection of OpenAI `nlp_enrich_evidence_ir` hints

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge temporal_rule_surface` -> passed
- `cargo test -p specforge nlp_enrich_hint` -> passed
- `bash scripts/run_ci.sh` -> passed with `373` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (Intent quality warnings now name score drivers)

### Improved: low-quality IntentIR findings now surface deterministic score-component IDs
- The remaining non-honest blank finding was the top-level quality warning:
  - `intent_quality_below_excellent_threshold` already knew the full score breakdown
  - the validator already knew which score components were below their maximum contribution
  - but the finding still emitted empty `related_ids`
- This slice tightens that surface by surfacing stable score-component IDs for each dimension that left points on the table:
  - `score_component:signal_direction`
  - `score_component:signal_width`
  - `score_component:nlp_constraints`
  - `score_component:encoding_enums`
  - `score_component:register_map`
  - `score_component:timing_constraints`
  - `score_component:state_machine`
  - `score_component:system_contract`
- Added a direct IntentIR regression that builds a minimal one-signal spec, validates the resulting low score, and proves the quality warning now reports the exact missing score components instead of an empty list.
- This is an observability hardening slice:
  - no scoring math changed
  - no canonical IR schema changed
  - validation now exposes the exact score dimensions it was already using to explain why the artifact stayed below `EXCELLENT`

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_reports_quality_gap_related_ids` -> passed
- `bash scripts/run_ci.sh` -> passed with `369` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Actor-port-gap findings now name relation ids)

### Improved: actor-port-missing findings now carry canonical actor-signal relation ids
- The next observability gap was in the canonical graph-to-port synthesis surface:
  - `semantic_actor_ports_missing` already knew `actor_signal_relations` existed
  - `intent_actor_ports_missing` already knew the same graph evidence survived into `IntentIR`
  - but both findings still emitted empty `related_ids`
- This slice tightens both findings by surfacing the exact canonical `ActorSignalRelation.relation_id` values that expose the synthesis gap.
- The validator helper deliberately uses the graph’s own ids instead of inventing a new review abstraction:
  - collect non-empty `relation_id`s from `actor_signal_relations`
  - deduplicate into a stable set
  - surface the first review-facing ids directly in the finding payload
- Added a direct semantic+intent regression that:
  - builds real actor-signal relations from source prose
  - deliberately clears synthesized `actor_ports`
  - proves both canonical findings now report the exact stranded relation ids instead of an empty list
- This is an observability hardening slice:
  - no actor-port synthesis behavior changed
  - no canonical IR schema changed
  - validation now exposes the exact graph relations it was already evaluating

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_actor_port_gap_related_ids` -> passed
- `bash scripts/run_ci.sh` -> passed with `368` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Evidence review findings now name stranded ids)

### Improved: EvidenceIR structural-KG and normative-residual findings now carry concrete ids
- Two Evidence-side review findings were still weaker than the recent semantic/intent validation surfaces:
  - `evidence_structural_kg_missing` knew behavioral records existed but did not name them
  - `evidence_normative_residuals_remaining` knew which statements were still normative-only but did not expose their ids
- This slice tightens both surfaces:
  - structural-KG-missing now emits the stranded behavioral record ids from `signal_constraints` and `conditional_rules`
  - normative-residuals now emit the surviving `ExtractedStatement.statement_id` values for `NormativeStatement` records
- Added a direct EvidenceIR regression that proves:
  - `evidence_structural_kg_missing` reports `["condrule_htrans_hold", "sigcon_hready_asserted"]`
  - `evidence_normative_residuals_remaining` reports `["stmt_normative_residual"]`
- This is an observability hardening slice:
  - no EvidenceIR extraction behavior changed
  - no canonical IR schema changed
  - validation now exposes the exact ids it was already counting

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_reports_structural_kg_and_normative_related_ids` -> passed
- `bash scripts/run_ci.sh` -> passed with `366` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Temporal-rule-surface-missing findings now name source ids)

### Improved: temporal-rule-surface-missing findings now carry upstream timing/constraint ids
- The previous temporal observability slice made under-grounded typed temporal rules name their canonical `rule_id`s.
- The neighboring `*_temporal_rule_surface_missing` findings were still weaker:
  - validation already knew which timing/constraint records survived into the artifact
  - validation already knew no typed temporal rules were derived from them
  - but the finding still emitted empty `related_ids`
- This slice tightens that surface for both `SemanticIR` and `IntentIR` by surfacing the upstream source record ids that remain unlowered into typed temporal rules:
  - timing constraint `constraint_id`s
  - signal constraint `constraint_id`s
  - conditional rule `rule_id`s
- Added a direct semantic+intent regression that proves a carried timing constraint with no typed temporal lowering now yields `related_ids: ["timing_hready_setup"]` for both stages.
- This is an observability hardening slice:
  - no temporal-rule derivation changed
  - no canonical IR schema changed
  - validation now exposes the exact source-side temporal inputs already known to be stranded below the typed temporal surface

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_temporal_rule_surface_missing_related_ids` -> passed
- `bash scripts/run_ci.sh` -> passed with `366` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Temporal-gap findings now name rule ids)

### Improved: temporal grounding gap findings now carry canonical temporal-rule IDs
- Several temporal grounding findings were still count-only even though the validator already had the exact `TemporalRuleRecord.rule_id` set in hand:
  - missing explicit clock / edge grounding
  - missing cycle-window bounds
  - missing actor-relative drive/sample grounding
- This slice tightens those findings for both `SemanticIR` and `IntentIR` by surfacing the canonical rule ids that actually satisfy each gap condition instead of emitting empty `related_ids`.
- Added a direct semantic+intent regression that rebuilds a single temporal rule lacking all three grounding surfaces and proves the following findings now report `related_ids: ["temporal_signal_constraint_sigcon_hready_stable"]`:
  - `*_temporal_rules_missing_clock_grounding`
  - `*_temporal_rules_missing_cycle_windows`
  - `*_temporal_rules_missing_actor_grounding`
- This is an observability hardening slice:
  - no temporal-rule derivation changed
  - no canonical IR schema changed
  - validation now exposes the exact canonical rule ids it was already evaluating

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_temporal_gap_related_ids` -> passed
- `bash scripts/run_ci.sh` -> passed with `365` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Alias-dependent handshake completion findings now name signals)

### Improved: alias-dependent handshake-completion findings now carry canonical handshake signal IDs
- The alias-dependent handshake-completion validation surface was still weaker than its sibling alias-dependent semantic-consensus finding:
  - the validator knew the exact handshake signals
  - the tracked fixture already modeled the caveat end to end
  - but the `semantic_*` and `intent_*` handshake-completion findings still emitted empty `related_ids`
- This slice tightens that review surface by deriving canonical signal names directly from `HandshakeComplete` predicates whose valid/ready signals rely on alias-dependent semantic consensus.
- Upgraded the direct validator regression so it now proves both `SemanticIR` and `IntentIR` report `related_ids: ["XACK", "XREQ"]` for the alias-grounded handshake-completion path.
- Upgraded the tracked fixture `alias_dependent_handshake_completion_caveat` so the end-to-end benchmark also locks the new payload for both validation stages.
- This is an observability hardening slice:
  - no temporal-rule derivation changed
  - no canonical IR schema changed
  - validation now exposes the exact canonical handshake signals already implicated by the carried rule

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_alias_dependent_handshake_completion_related_ids` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality alias_dependent_handshake_completion_caveat` -> passed
- `bash scripts/run_ci.sh` -> passed with `364` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Prior-guided semantic related-id regression coverage)

### Improved: prior-guided semantic-role related IDs are now locked by a direct semantic+intent regression
- The previous validation slice taught prior-guided semantic arbitration and prior-guided semantic consensus findings to name their canonical signals in `related_ids`.
- That behavior was already protected by tracked fixtures, but it still lacked a small direct unit proof that exercises the real prior-memory path and checks both canonical validation stages together.
- This recovery slice finishes the stalled modality-reliability prior-memory helper in `crates/specforge/src/commands/validate.rs` and adds a combined regression that:
  - seeds learned prior memory for a valid-like `SignalDescriptionTable` hint
  - rebuilds `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` over the contested `XCTRL` example
  - proves both `semantic_*` and `intent_*` prior-guided findings report `related_ids: ["XCTRL"]`
- This is a tests-only hardening slice:
  - no validation logic changed
  - no canonical IR schema changed
  - no tracked fixture semantics changed

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_report_prior_guided_semantic_related_ids` -> passed
- `bash scripts/run_ci.sh` -> passed with `364` Rust tests and the mdBook build
- `bash scripts/run_docs_ci.sh` -> passed

## 2026-04-19 (Semantic-stage related-id regression coverage)

### Improved: semantic-stage validator regressions now lock the new semantic-role related IDs directly
- The previous slice taught semantic-role arbitration/consensus findings to name their canonical signals in `related_ids`.
- This follow-on slice makes the semantic-stage proof surface match the intent-stage proof surface instead of relying mostly on tracked fixtures and shared code paths.
- Strengthened existing semantic validator tests so they now assert the exact related-id payload for:
  - non-decisive semantic arbitration on `XCTRL`
  - blocked handshake-name fallback on `XVALID`
- Added direct semantic-stage regressions for:
  - alias-dependent semantic consensus on `XREQ` / `XACK`
  - resolved semantic roles without consensus on `XREQ`
- This is a tests-only hardening slice:
  - no validation behavior changed
  - no fixture semantics changed
  - no canonical IR schema changed

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_ir_flags_signal_semantic_conflicts` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_blocked_handshake_name_fallback` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_alias_dependent_semantic_consensus` -> passed
- `cargo test -p specforge validate_semantic_ir_flags_resolved_roles_without_consensus` -> passed
- `bash scripts/run_ci.sh` -> passed with `363` Rust tests and the mdBook build

## 2026-04-19 (Semantic arbitration findings now name signals)

### Improved: semantic-role arbitration and consensus findings now carry signal-level related IDs
- `specforge validate` no longer leaves several semantic-role observability findings as count-only summaries when the exact implicated canonical signals are already known.
- The following findings now emit signal names in `related_ids` for both `SemanticIR` and `IntentIR`:
  - `*_non_decisive_semantic_arbitration_present`
  - `*_prior_guided_semantic_arbitration_present`
  - `*_resolved_roles_without_consensus_present`
  - `*_alias_dependent_semantic_consensus_present`
  - `*_prior_guided_semantic_consensus_present`
- This is an observability tightening, not a semantic mutation:
  - the metrics stay unchanged
  - arbitration, consensus, and conflict logic stay unchanged
  - validation simply exposes the specific canonical signal names already selected by those truth surfaces
- Upgraded tracked fixtures now lock the new payloads end to end for:
  - non-decisive arbitration with blocked handshake fallback
  - alias-dependent semantic consensus
  - prior-guided semantic arbitration and consensus

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_reports_alias_dependent_semantic_consensus` -> passed
- `cargo test -p specforge validate_intent_ir_flags_resolved_roles_without_consensus` -> passed
- `cargo test -p specforge validate_intent_ir_counts_multiple_semantic_candidates_for_conflicts` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality contested_handshake_name_fallback_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality alias_dependent_handshake_completion_caveat` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_modality_reliability_prior_guided_conflict_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with `361` Rust tests and the mdBook build

## 2026-04-19 (Compat-direction lag now names exact signals)

### Improved: compatibility-direction lag findings are now precise and graph-scoped
- `specforge validate` no longer leaves compatibility-direction lag findings as vague count-only notes when the exact lagging signals are already known.
- `semantic_compat_direction_hints_incomplete` now emits the missing flat-hint signal names in `related_ids`.
- `intent_compat_direction_hints_lag_graph` now does the same, but only for graph-backed declared signals:
  - if actor-relative graph evidence already recovers direction for a signal
  - and the flat compatibility `direction_hint` still lags behind
  - validation names that exact signal instead of counting every missing flat hint indiscriminately
- Added a narrow `kg-bench` patch lane, `semantic_ir_patch.clear_signal_direction_hints`, so tracked fixtures can clear flat compatibility direction hints without mutating canonical graph structure or inventing arbitrary semantic rewrites.
- Added the tracked fixture `compat_direction_hints_lag_graph_negative`, which locks the intended graph-first truth boundary end to end for both `SemanticIR` and `IntentIR`.
- Refreshed the managed corpus-KB benchmark/pattern projections so the tracked suite now reports `92/92` passing fixtures and the semantic/truthfulness family page now reports `51/51` passing fixtures.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_scores_direction_from_graph_before_compat_hints` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_missing_compat_direction_related_ids` -> passed
- `cargo test -p specforge kg_bench_supports_semantic_direction_hint_clear_patch` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality compat_direction_hints_lag_graph_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `361` Rust tests and the mdBook build
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `92` fixtures and `0` failures

## 2026-04-19 (Graph-direction conflict vs coverage-gap split)

### Improved: same-actor graph conflicts no longer also trip generic coverage-gap findings
- `specforge validate` now keeps the graph-direction warning surfaces separate:
  - `*_graph_direction_conflicts_present` still reports same-actor self-conflicts
  - `*_graph_direction_coverage_incomplete` now reports only signals with no graph-derived direction coverage outside that conflict set
- This removes a small but real diagnostic blur. A signal like `PREADY` that already has graph evidence but that evidence conflicts should be reported as a conflict, not as both a conflict and a generic missing-coverage case.
- Strengthened the direct semantic and intent validator regressions so conflict-only cases now assert the absence of `*_graph_direction_coverage_incomplete`.
- Updated the tracked `graph_direction_same_actor_conflict_negative` fixture so the benchmark harness also locks that absence directly.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_credit_conflicting_same_actor_graph_direction` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_conflicting_same_actor_graph_direction` -> passed
- `cargo test -p specforge validate_intent_ir_reports_missing_graph_direction_related_ids` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_missing_graph_direction_related_ids` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_same_actor_conflict_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_coverage_incomplete_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `359` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-19 (Graph-direction coverage finding related IDs)

### Improved: validation warnings now name signals still missing graph-derived direction
- `specforge validate` now emits `related_ids` for `semantic_graph_direction_coverage_incomplete` and `intent_graph_direction_coverage_incomplete` instead of leaving those findings as count-only warnings.
- The new related-id payload is intentionally signal-level rather than actor-level:
  - coverage gaps answer "which canonical signals still lack graph-derived direction?"
  - same-actor graph conflicts still use the separate actor-aware related-id surface added in the previous slice
- Added focused validator regressions for both `SemanticIR` and `IntentIR`, proving that a mixed graph-coverage case now points directly at `PSEL` when `PREADY` is graph-covered but `PSEL` remains directionless in the actor graph.
- Added tracked fixture `graph_direction_coverage_incomplete_negative`, which locks the same behavior end to end and keeps the benchmark harness honest about incomplete graph coverage.
- Refreshed the managed corpus-KB benchmark/pattern projections so the tracked truthfulness suite now reports `91/91` passing fixtures and `50` pattern fixtures.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_reports_missing_graph_direction_related_ids` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_missing_graph_direction_related_ids` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_coverage_incomplete_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `359` Rust tests and the mdBook build
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `91` fixtures and `0` failures
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-19 (README bootstrap analysis refresh)

### Refreshed: live Rust analysis after re-executing the README bootstrap path
- Re-ran the repository bootstrap contract from `README.md` through `SESSION_BOOTSTRAP.md`, then reread the linked continuity and user-facing markdown surfaces before re-surveying the active Rust workspace directly from disk.
- Refreshed `RUST_CODEBASE_ANALYSIS.md` so its bootstrap snapshot now matches the current repository reality:
  - `31` Rust source files under `crates/specforge/src`
  - `62,689` total Rust source lines under `crates/specforge/src`
  - `90` tracked KG-quality fixtures
  - `357` passing Rust tests in the canonical local CI path
- This is a docs-only continuity refresh:
  - no CLI command surface changed
  - no IR schema, validation rule, benchmark behavior, or adapter logic changed
  - the goal is to keep the bootstrap/handoff path truthful after the recent graph-direction validation slices

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with `357` Rust tests and the mdBook build
- `git diff --check` -> passed

## 2026-04-18 (Graph-direction conflict finding provenance)

### Improved: validation warnings now emit actor-aware graph-direction conflict related IDs
- `specforge validate` now formats same-actor graph-direction conflict findings with stable actor-aware related ids such as `graph_direction_conflict:actor_completer:PREADY` instead of collapsing them to raw signal names.
- This makes the warning payload match the sharper truth surface already used by the graph-direction coverage summary and the `kg-bench` canonical conflict expectations.
- The signal-count metric stays intentionally unchanged:
  - `graph_direction_conflicts` still counts conflicted signals, not actor-signal records
  - the warning summary now clarifies both the conflicted signal count and the actor-signal conflict record count
- Updated the tracked `graph_direction_same_actor_conflict_negative` fixture and focused regression coverage so both `SemanticIR` and `IntentIR` validation now lock the actor-aware related-id payload directly.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_intent_ir_does_not_credit_conflicting_same_actor_graph_direction` -> passed
- `cargo test -p specforge validate_semantic_ir_reports_conflicting_same_actor_graph_direction` -> passed
- `cargo test -p specforge kg_bench_supports_semantic_actor_port_patch_for_graph_direction_conflicts` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_same_actor_conflict_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `357` Rust tests and the tracked `90/90` fixture suite

## 2026-04-18 (KG-bench graph-direction conflict provenance)

### Added: actor-level graph-direction conflict expectations in `specforge kg-bench`
- `CanonicalStageExpectations` now supports `graph_direction_conflicts_include`, so tracked fixtures can assert same-actor graph-direction conflicts as typed actor-plus-signal records instead of only as conflicted signal-name sets.
- The harness derives those conflict records from the same graph-direction coverage summary used for validation and conflicted signal-name coverage, so this slice adds provenance precision without introducing a second interpretation of graph truth.
- Updated `graph_direction_same_actor_conflict_negative` so it no longer stops at "PREADY is conflicted"; it now locks the specific culprit too: `Completer` self-conflicts on `PREADY`.
- This remains an observability slice, not a semantic mutation:
  - canonical `SemanticIR` / `IntentIR` facts are unchanged
  - no graph conflict is auto-healed or reclassified
  - the benchmark contract simply gets sharper about why the signal stayed unresolved

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge canonical_expectations_exclude_conflicting_same_actor_graph_direction` -> passed
- `cargo test -p specforge graph_direction_coverage_summary_reports_same_actor_conflicts` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_same_actor_conflict_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `357` Rust tests and the tracked `90/90` fixture suite

## 2026-04-18 (KG-bench graph-direction conflict expectations)

### Added: canonical conflict-set expectations in `specforge kg-bench`
- `CanonicalStageExpectations` now supports `graph_direction_conflicted_signal_names_include` and `graph_direction_conflicted_signal_names_exclude`.
- The harness derives this conflicted set from the same canonical actor-port graph-direction conflict summary used by `specforge validate`, so fixtures can assert which signals were withheld from resolved graph-direction coverage because of same-actor self-conflicts.
- Updated `graph_direction_same_actor_conflict_negative` to lock both halves of the behavior explicitly:
  - `PADDR` remains in the resolved graph-direction set
  - `PREADY` stays out of the resolved set and appears in the conflicted set instead
- This slice does not mutate canonical truth or auto-repair conflicts; it makes the benchmark contract more explicit and reviewable.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge canonical_expectations_exclude_conflicting_same_actor_graph_direction` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_same_actor_conflict_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `357` Rust tests and the tracked `90/90` fixture suite
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-18 (Corpus-KB benchmark refresh after graph-direction fixture expansion)

### Refreshed: corpus-KB benchmark projections now reflect the `90/90` tracked fixture suite
- Re-ran `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` after adding `graph_direction_same_actor_conflict_negative`.
- The managed benchmark projection in `corpus_kb/benchmarks/kg-fixtures.md` now reports `90` tracked fixtures with `0` failures, up from `89`.
- The managed pattern-family projection in `corpus_kb/patterns/kg-fixtures.md` now reports `49` pattern fixtures and includes the new graph-direction self-conflict negative case under `actor connectivity` plus `truthfulness negatives and cautions`.
- This is a review-surface refresh only: no canonical IR semantics, validation behavior, or benchmark execution rules changed in this slice.

### Validation
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `90` fixtures and `0` failures
- `git diff --check` -> passed

## 2026-04-18 (KG-bench graph-direction conflict fixture lane)

### Changed: `kg-bench` can now express canonical-stage graph-direction conflict fixtures
- Added a narrow `semantic_ir_patch.actor_ports_append` fixture surface to `specforge kg-bench`, so tracked fixtures can append canonical actor-port records after `SemanticIR` build but before `IntentIR` carry-through.
- That patch lane is intentionally small and review-friendly: it is just enough to express canonical-stage honesty regressions that are hard to synthesize from raw source text alone, without turning the benchmark harness into a general semantic editor.
- Added tracked fixture `graph_direction_same_actor_conflict_negative`, which appends a conflicting `Completer -> PREADY` input actor port and proves:
  - `graph_direction_signal_names` includes `PADDR` but excludes `PREADY`
  - `SemanticIR` and `IntentIR` validation both report `graph_direction_conflicts: 1`
  - both stages emit the expected graph-direction conflict warning with `PREADY` as a related id
- Added focused harness unit coverage so the new patch surface itself is directly protected, not only the tracked fixture outcome.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_supports_semantic_actor_port_patch_for_graph_direction_conflicts` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality graph_direction_same_actor_conflict_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with `357` Rust tests and the tracked `90/90` fixture suite

## 2026-04-18 (Graph-direction conflict visibility)

### Changed: graph-direction self-conflicts now surface as first-class validation output
- `specforge validate` still withholds graph-direction coverage when the same actor claims contradictory directions for the same signal, but it no longer does so silently.
- Validation now emits an explicit `graph_direction_conflicts` metric at both `SemanticIR` and `IntentIR`, prints that count in the coverage summary, and raises stage-specific warning findings when those same-actor contradictions are present.
- Added focused regression coverage proving:
  - the graph-direction helper keeps separate resolved and conflicted signal sets
  - `SemanticIR` validation reports the new metric and warning finding for a same-actor self-conflict
  - `IntentIR` validation reports the same metric/finding while still refusing to credit the conflicted signal as graph-resolved

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge graph_direction` -> passed
- `cargo test -p specforge commands::kg_bench::tests::canonical_expectations_exclude_conflicting_same_actor_graph_direction` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-18 (Graph-direction coverage conflict guard)

### Changed: graph-direction coverage no longer credits same-actor self-conflicts
- `specforge validate` now treats graph-backed direction coverage as unresolved when the same actor claims contradictory directions for the same signal, instead of counting that signal as graph-resolved merely because non-`unknown` actor ports exist.
- `specforge kg-bench` now reuses the validator’s graph-direction helper, so fixture expectations and validation metrics share the same honesty rule instead of drifting into separate interpretations.
- Added focused regressions at both command surfaces:
  - validation helper and report-level coverage proving conflicting same-actor graph ports do not count as resolved graph direction when compatibility hints are absent
  - `kg-bench` canonical expectation coverage proving `graph_direction_signal_names_exclude` honors the same conflict guard

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge commands::validate::tests::` -> passed
- `cargo test -p specforge commands::kg_bench::tests::` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-18 (Adapter graph-backed direction surface split)

### Changed: adapter-local signal inventory now separates graph direction from compatibility direction
- `FsmSignalCandidate` now carries a dedicated `graph_direction_hint` plus sticky `graph_direction_hint_conflicted`, so actor-port, topology, and control-read overlays no longer have to overwrite compatibility-facing `direction_hint`.
- Graph-backed overlays now register through a separate adapter-local path for `actor_port`, `module_topology_link`, `module_control_input`, and `direct_control_input`, while flat interface and system-contract shape still remain visible as compatibility hints.
- Adapter renderability and module-port construction now use a preferred direction rule:
  - graph direction wins when it is present and unambiguous
  - compatibility direction can still fill the gap when graph evidence is absent
  - explicit graph conflict stays blocking instead of silently falling back to compatibility
- Updated the focused adapter regressions across standalone DT/FSM, explicit-module, sequential system-signal, and top-composition paths so the separation and the conflict semantics are locked in.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-18 (README bootstrap analysis refresh)

### Refreshed: live Rust analysis after executing the README bootstrap
- Re-ran the README handoff path through `SESSION_BOOTSTRAP.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `MEMORY.md`, `COMMIT.md`, and the current Rust codebase surface before selecting further roadmap work.
- Refreshed `RUST_CODEBASE_ANALYSIS.md` so its bootstrap snapshot now matches the current repository reality: `31` Rust source files, `62,017` lines under `crates/specforge/src`, `89` tracked KG fixtures, and the current `351`-test local CI baseline.
- Recorded the next high-leverage roadmap focus from that bootstrap pass: `R15` still has meaningful compatibility-`direction_hint` pockets in adapter/validation consumers even though the actor-relative graph surface is already in place.

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-18 (Signal-value VLM timing label rejection)

### Fixed: bare signal-value labels stay out of timing constraints
- `SemanticIR` now rejects VLM timing annotations that are only a known signal plus a bare sampled value, so labels such as `XREQ HIGH`, `XREQ LOW`, `XREQ asserted`, and `XREQ deasserted` no longer become fake `TimingConstraintRecord`s.
- This closes the natural follow-on leak after name-only label rejection: the right decision still depends on the current document's grounded signal inventory, but the label now carries only a lane-style sampled value that the structured `signals[].values[]` path already models more honestly.
- Strengthened the direct semantic regression and the tracked `vlm_timing_spurious_annotation_negative` KG fixture so known-signal sampled-value labels yield zero timing constraints while grounded `signals[].values[]` samples still author temporal rules.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_label_only_noise` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-18 (Name-only VLM timing label rejection)

### Fixed: bare known signal labels stay out of timing constraints
- `SemanticIR` now rejects VLM timing annotations that are only a known signal name, so bare labels such as `XREQ` no longer become fake `TimingConstraintRecord`s.
- This closes a different class of truthfulness leak from the generic waveform-markup filters: the right decision depends on the current document's grounded signal inventory, not just on generic label vocabulary.
- Strengthened the direct semantic regression and the tracked `vlm_timing_spurious_annotation_negative` KG fixture so a bare known signal label yields zero timing constraints while grounded `signals[].values[]` samples still author temporal rules.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_label_only_noise` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-18 (Multi-token VLM waveform-label rejection)

### Fixed: four-token waveform gutter labels stay out of timing constraints
- `SemanticIR` now treats pure generic VLM timing annotation groups up to four tokens as low-value waveform markup, so labels such as `Channel 1 Phase 2` and `Lane 0 Slot 1` are rejected alongside shorter annotation noise like `Burst 1` or `Phase1`.
- This closes a realistic chip-PDF edge case where cramped timing gutters use short multi-token lane/phase labels that still do not express timing law but previously slipped past the `<= 3` generic-token boundary.
- Strengthened the direct semantic regression and the tracked `vlm_timing_spurious_annotation_negative` KG fixture so these four-token markup labels yield zero timing constraints while grounded `signals[].values[]` samples still author temporal rules.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_label_only_noise` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-18 (Compact VLM phase/transfer label rejection)

### Fixed: compact phase and transfer labels stay out of timing constraints
- `SemanticIR` now treats compact VLM timing annotation labels such as `Phase1` and `Transfer2` as the same low-value waveform markup family as `Phase 1`, `Transfer 2`, `Burst1`, and other standalone annotation noise.
- This closes another cramped-layout chip-PDF edge case where waveform gutters compress bus-phase labels into a single token without changing their semantic status as figure markup rather than timing law.
- Strengthened the direct semantic regression and the tracked `vlm_timing_spurious_annotation_negative` KG fixture so compact phase/transfer labels yield zero timing constraints while grounded `signals[].values[]` samples still author temporal rules.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_label_only_noise` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-18 (Compact VLM timing bus-label rejection)

### Fixed: compact bus-phase labels stay out of timing constraints
- `SemanticIR` now treats compact VLM timing annotation labels such as `Burst1`, `Packet2`, `Frame3`, `Transaction4`, and `Txn5` as the same low-value waveform markup family as `Burst 1`, `Packet 2`, and other standalone annotation noise.
- This closes the compact-layout variant of the same chip-PDF edge case: when a waveform gutter is cramped, bus/transaction labels often lose the separating space but still do not express timing law.
- Strengthened the direct semantic regression and the tracked `vlm_timing_spurious_annotation_negative` KG fixture so both spaced and compact bus-label variants yield zero timing constraints while grounded `signals[].values[]` samples still author temporal rules.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_label_only_noise` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-18 (VLM timing bus-label annotation rejection)

### Fixed: bus-level waveform labels stay out of timing constraints
- `SemanticIR` now treats standalone VLM timing annotation labels such as `Burst 1`, `Packet 2`, `Frame 3`, `Transaction 4`, and `Txn 5` as the same low-value waveform markup family as `T0`, `Addr 1`, `Cycle 2`, `D0`, `DATA[3]`, and `XREQ[3:0]`.
- This closes a realistic chip-PDF edge case where bus/timing diagrams use bus-phase labels in the annotation lane, but those labels do not express protocol law and must not survive into canonical timing constraints.
- Strengthened the direct semantic regression and the tracked `vlm_timing_spurious_annotation_negative` KG fixture so grounded `signals[].values[]` evidence still produces temporal rules while the expanded annotation-noise set yields zero timing constraints.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge vlm_timing_diagram_observation_rejects_label_only_noise` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-18 (Generated-root cleanup scope for `specforge clean`)

### Added: full generated-root sweep inside the cleanup command
- Extended `specforge clean` with `--scope all-generated`.
- That scope dry-runs the whole local `generated/` root as one candidate and, with `--execute`, removes it in one sweep.
- This closes the last gap in the cleanup story: users no longer need a raw shell `rm -rf generated` when they intentionally want a full local artifact reset.

### Improved: cleanup scope guardrails
- `specforge clean --scope all-generated` now rejects `--document-key` explicitly instead of silently ignoring it.
- The command surface stays self-explanatory:
  - `source-normalized` for heavyweight normalized bundles
  - `document` for one document's generated stage trees
  - `all-generated` for the full local generated root

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge commands::clean::tests::` -> passed
- `cargo run -p specforge -- clean --scope all-generated` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `351` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Generated artifact cleanup and SourceIR normalized-bundle hygiene)

### Added: first-class generated artifact cleanup
- Added `specforge clean`, a local-only generated-artifact reclamation command.
- The default scope is intentionally narrow and safe: dry-run only, scanning heavyweight `generated/source_ir/*/normalized` bundles and reporting reclaimable size before deleting anything.
- `--execute` now deletes those rebuildable normalized bundles while preserving `source_ir.json`, and `--scope document [--document-key <key>]` can remove full per-document generated stage trees when a cold rebuild is intentional.

### Improved: repeated PDF ingest no longer layers stale normalized leftovers
- Docling-backed PDF materialization now stages into `generated/source_ir/<document_key>/normalized.staging` and swaps that tree into `normalized/` only after backend success.
- Re-ingesting the same document key now replaces the previous normalized bundle atomically instead of leaving stale page images, crops, or backend dumps from older runs beside the current output.
- Failed reruns now preserve the last good `normalized/` bundle instead of deleting it before the new backend attempt succeeds.

### Operational result
- Ran `specforge clean --execute` locally after landing the command and reclaimed the five current normalized AMBA bundles.
- `generated/source_ir` dropped from about `442 MiB` to about `4.9 MiB`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge commands::clean::tests::` -> passed
- `cargo test -p specforge pdf_source_ir_` -> passed
- `cargo run -p specforge -- clean` -> passed and reported `432.5 MiB` reclaimable across five normalized bundles
- `cargo run -p specforge -- clean --execute` -> passed and deleted those five normalized bundles
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `347` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-18 (Adapter duplicate top-port width conflict collapse)

### Improved: duplicate top-port widths cannot overwrite conflicts
- Added sticky width-conflict state for duplicate explicit top-port declarations.
- If duplicate top-port declarations disagree about width, the adapter now collapses the resolved top-port width to `None` instead of letting the later declaration overwrite the earlier one in blocked top artifacts.
- Added a regression where `drive_data` is declared twice as a top output with widths `8` and `16`; both duplicate top-port entries and both selected top signal-inventory entries keep unresolved width.
- This closes the sibling truthfulness gap to the duplicate-direction hardening: blocked top artifacts no longer imply a settled boundary width when duplicate canonical declarations disagree.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_keeps_duplicate_top_port_width_conflict_unresolved` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `40` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `341` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter duplicate top-port direction conflict collapse)

### Improved: duplicate top-port declarations cannot overwrite conflicts
- Routed duplicate explicit top-port declarations through the same sticky `TopPortDirectionEvidence` merger used for top-link topology recovery.
- If duplicate declarations disagree about a top boundary direction, the adapter now blocks and collapses the resolved top-port direction to `None` instead of letting the later declaration overwrite the earlier one.
- Added a regression where `drive_data` is declared once as a top output and once as a top input; the blocked top candidate and selected top signal inventory both expose unresolved direction.
- This closes another blocked-artifact truthfulness gap around explicit top composition.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_keeps_duplicate_top_port_direction_conflict_unresolved` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `39` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `340` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter top-port direction conflict collapse)

### Improved: conflicting top-boundary direction evidence stays unresolved
- Added sticky direction-conflict state for explicit top boundary ports during `.fsm` composition analysis.
- If a top port declaration and top-link topology disagree, the adapter now records the conflict, collapses the resolved top-port direction to `None`, and keeps the selected top signal inventory unresolved.
- Added a regression where `drive_data` is declared as a top output but used as a top-link source, which implies top input; lowering stays blocked and no `.fsm` target text is emitted.
- This keeps blocked adapter artifacts honest: users see the learned uncertainty instead of a stale declaration that looks canonical after contradictory topology was found.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_keeps_conflicting_top_port_direction_unresolved` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `38` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `339` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter system-contract signal conflict guard)

### Added: system-contract recovery cannot override contradictory signal shape
- Added a regression proving clock/reset recovery from canonical `SystemContractRecord` facts remains sticky when local signal evidence disagrees.
- The fixture marks `clk` as a flat output while the canonical system contract says `clk` is the clock.
- The adapter keeps `clk.direction_hint` unresolved, preserves `system_contract_signal` evidence, emits the system-contract residual, and blocks `.fsm` emission rather than silently treating the clock as an input.
- This protects the previous system-contract recovery slice: clock/reset facts can fill absent adapter shape, but contradictory shape still requires upstream correction.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge standalone_sequential_dt_blocks_conflicting_system_contract_signal_direction` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `37` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `338` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter system-contract signal recovery)

### Improved: `.fsm` lowering can recover clock/reset port shape from system contracts
- Added adapter-side recovery for existing clock/reset inventory entries from canonical `SystemContractRecord` facts.
- Direct roots and explicit module roots now overlay system-contract clock/reset signals as input, 1-bit `system_contract_signal` entries before renderability analysis.
- The overlay is bounded to signals already present in the local inventory, so it cannot create undeclared clock/reset ports, and it still uses sticky merge behavior if flat hints, actor ports, or other evidence disagree.
- Added regressions for standalone sequential DT and standalone explicit module roots where flat clock/reset direction and width hints are cleared; both still render honestly from system-contract facts while keeping canonical `IntentIR` unchanged.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge system_signals_from_system_contract` -> passed with both focused system-contract recovery tests
- `cargo test -p specforge ir::adapters::tests::` -> passed with `36` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `337` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter child-link topology conflict guard)

### Added: conflicting child-link topology stays blocked
- Added a regression proving child-module direction recovery from top-link topology remains sticky and truth-preserving when topology evidence contradicts itself.
- The fixture clears all flat module-local directions, then uses `producer.output_data` as both a child link source and a child link target through separate top links.
- The adapter keeps `producer_core.output_data.direction_hint` unresolved, preserves the `module_topology_link` evidence category, blocks the producer module, and emits no `.fsm` target text.
- This protects the previous topology recovery slice: explicit links can fill missing child port roles, but contradictory topology still requires upstream correction instead of adapter-side guessing.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_blocks_conflicting_child_link_topology_directions` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `34` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `335` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter child-link topology direction recovery)

### Improved: top links can recover existing child module port directions
- Added a bounded module-topology overlay derived from explicit top links before explicit module renderability analysis.
- A child endpoint used as a top-link source can recover that module signal as an output; a child endpoint used as a top-link target can recover that module signal as an input.
- The overlay is conservative: it only applies to signals already present in the child module inventory, does not create ports, does not mutate canonical `IntentIR`, and still uses sticky conflict collapse if other evidence disagrees.
- Added a top-composition regression where all flat module-local directions are cleared and no actor ports are supplied; the adapter still renders because `producer.output_data -> consumer.input_data` and `consumer.result_data -> result_data` recover the child module port roles from explicit topology.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_recovers_child_directions_from_link_topology` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `33` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `334` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter explicit-module read conflict guard)

### Added: module control-read recovery now has conflict regression coverage
- Added a regression proving explicit-module `module_control_input` recovery does not override contradictory module-actor graph evidence.
- The test clears flat module-local direction hints, marks `DATA_IN` as a `controller` actor output, then relies on module state-body assignments to read `DATA_IN`.
- The adapter now has coverage proving the actor-port output and module-control input evidence collapse to an unresolved direction, preserve both evidence categories, and block `.fsm` emission rather than guessing.
- This protects the truthfulness boundary around the previous explicit-module recovery slice: recovery fills missing roles, but contradictory evidence still requires upstream resolution.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge standalone_explicit_module_blocks_conflicting_module_control_read_direction` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `32` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `333` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter explicit-module control-read recovery)

### Improved: standalone explicit modules recover local inputs from module control reads
- Refactored direct-root output-target and control-read collection into reusable record-slice helpers.
- Explicit module candidates now overlay module-local `module_control_input` directions after actor-port overlay, using the module's own DT fragments, rich control blocks, state transitions, and init assignments.
- The recovery is bounded to signals already present in the module inventory and excludes module output targets, so it cannot invent new ports or turn assigned outputs into fake inputs.
- Added a standalone explicit-module `?fsm` regression where flat module-local direction hints are cleared, the `controller` actor graph only owns clock/reset inputs plus `ACC` / `TRACE` outputs, and external actors drive `DATA_IN`, `GO`, and `DONE`.
- The test proves the selected explicit module root still renders as `(?fsm:controller)` and recovers `DATA_IN`, `GO`, and `DONE` as module-local inputs from state-body assignments, transition guards, and standalone control blocks.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge standalone_explicit_module_recovers_inputs_from_module_control_reads` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `31` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `332` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter structured-FSM graph-read coverage)

### Added: true FSM roots now have graph-backed control-read regression coverage
- Added a structured `?fsm` adapter regression that clears all flat direct-interface direction hints, then supplies graph evidence only for the selected target actor's clock/reset inputs and produced outputs.
- The same fixture supplies external actors as producers for `DATA_IN`, `GO`, and `DONE`, proving external actor ports do not define the selected target perspective.
- The adapter must recover `DATA_IN`, `GO`, and `DONE` as target-actor inputs from the structured FSM's control reads: state-body assignments, transition guards, and standalone control blocks.
- The test proves the true FSM root still renders `(?fsm:explicit_fsm)` with `ACC <= DATA_IN`, `GO` / `DONE` guards, and the `TRACE` block after graph/control-read recovery.
- This is coverage hardening for existing graph-first adapter behavior, not a semantic widening or canonical IR mutation.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge structured_fsm_derives_guard_inputs_from_control_reads_after_output_actor_selection` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `30` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `331` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Adapter blocked-top direction retention)

### Improved: recovered top-boundary directions survive blocked composition artifacts
- Split top-composition renderability analysis so recovered top ports and renderable top roots are tracked separately.
- The `.fsm` adapter now preserves explicit top-link topology recovery on `FsmTopCandidate.ports` even when the top remains blocked for another reason such as a missing child module.
- Selected top-root signal inventory now reflects those recovered top boundary directions in blocked adapter artifacts too, instead of falling back to the raw flat `direction_hint` surface.
- Added a regression proving a width-only `result_data` top port becomes an output through `consumer.result_data -> result_data` topology while lowering still blocks honestly because `missing_module` is not declared.
- Updated the mdBook and live project docs to clarify that partial topology recovery remains visible as recovered adapter context, not only as emitted `.fsm` text.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge top_composition_preserves_recovered_top_port_direction_when_still_blocked` -> passed
- `cargo test -p specforge ir::adapters::tests::` -> passed with `29` adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `330` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (KG validation-finding payload expectations)

### Added: KG fixtures can assert validation finding payloads directly
- Added `validation.<stage>.findings_include` to the `specforge kg-bench` fixture expectation schema.
- Each expected finding can match `finding_id`, optional `severity`, optional `category`, optional `summary_contains`, required `related_ids_include`, and forbidden `related_ids_exclude`.
- Added a focused harness self-test proving mismatched related IDs produce a useful `findings_include` diagnostic.
- Strengthened the five negative-knowledge prior-guided caution fixtures so they now assert the exact conflict/residual IDs carried by their validation prior-match and rescan-guidance findings.
- Updated the mdBook quality chapters and live project docs to describe the new validation-finding payload surface and the non-authoring negative-knowledge boundary.

### Validation
- `cargo fmt --all` -> passed
- focused five-fixture negative-knowledge `kg-bench` run -> passed
- `cargo test -p specforge kg_bench` -> passed with `10` harness tests, including the tracked `89/89` fixture suite
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `329` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (Post-push continuity baseline sync)

### Changed: live handoff state now points at the pushed baseline
- Updated the continuity ledger after the 25-commit push so it now records `4dfb6b9` as the latest pushed baseline.
- Recorded that the canonical table-support diagnostic quartet is committed and pushed: wrong SemanticIR support id, wrong IntentIR support id, missing SemanticIR signal, and missing IntentIR signal.
- Clarified that there is no remaining in-flight code slice from the KG table-support diagnostic work.
- Kept this as a live-doc-only sync; no Rust, fixture, adapter, pipeline, mdBook, or user-facing behavior changed.

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-18 (KG IntentIR table-support missing-signal diagnostic coverage)

### Added: focused missing-signal coverage for IntentIR table-support expectations
- Added `kg_bench_reports_missing_intent_table_support_failure`, a focused `kg-bench` unit test for absent-signal failures under `intent.signal_supporting_table_ids_include`.
- The test reuses the one-row structured signal-table fixture and expects `MISSING_INTENT_SIGNAL` to carry `table_protocol_signal_description` even though the canonical signal set contains only `XREQ`.
- The assertion verifies that the failure names the fixture, the `intent` stage, the exact `signal_supporting_table_ids_include[MISSING_INTENT_SIGNAL]` field, the missing expected signal, and an actual recovered signal.
- The focused `cargo test -p specforge table_support_failure` filter now covers all four canonical table-support diagnostics: wrong SemanticIR support id, wrong IntentIR support id, missing SemanticIR signal, and missing IntentIR signal.
- `MEMORY.md` was also brought forward from the stale `5cdc36e` baseline to the latest committed local baseline `849e14a`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge table_support_failure` -> passed with `4` canonical table-support diagnostic tests
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `328` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG canonical table-support missing-signal diagnostic coverage)

### Improved: canonical table-support failures now identify missing signals precisely
- Tightened the missing-signal branch for `signal_supporting_table_ids_include`.
- When a fixture expects table support for a signal that is absent from the canonical signal inventory, the diagnostic now names the exact `signal_supporting_table_ids_include[<signal>]` field and prints the actual canonical signal set.
- Added `kg_bench_reports_missing_canonical_table_support_failure`, a focused unit test that expects `MISSING_SIGNAL` to carry `table_protocol_signal_description` while the one-row signal-table fixture only recovers `XREQ`.
- Renamed the test so the existing `cargo test -p specforge table_support_failure` filter now covers all canonical table-support diagnostic cases together.
- This completes the immediate diagnostic surface for canonical table support: wrong SemanticIR support id, wrong IntentIR support id, and absent expected canonical signal.
- `MEMORY.md` was also brought forward from the stale `c2bbd8f` baseline to the latest committed local baseline `5cdc36e`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge table_support_failure` -> passed with `3` canonical table-support diagnostic tests
- `cargo test -p specforge evidence_table_provenance` -> passed with the EvidenceIR table-provenance diagnostic tests
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `327` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG IntentIR table-support diagnostic coverage)

### Added: focused failure coverage for IntentIR signal table-support expectations
- Added `kg_bench_reports_intent_table_support_failure`, a focused `kg-bench` unit test for `signal_supporting_table_ids_include` at the `IntentIR` expectation stage.
- The test reuses the same realistic one-row `Signal | Direction | Width | Description` fixture helper as the SemanticIR and EvidenceIR diagnostics.
- It deliberately expects `XREQ` to carry `missing_intent_signal_table` even though the canonical `IntentIR` signal is backed by `table_protocol_signal_description`.
- The assertion verifies the failure message names the fixture, the `intent` stage, the `signal_supporting_table_ids_include[XREQ]` field, the missing expected table id, and the actual table id.
- This completes the immediate canonical table-support diagnostic pair: missing support is now self-tested at both `SemanticIR` and `IntentIR`.
- `MEMORY.md` was also brought forward from the stale `ae4ea54` baseline to the latest committed local baseline `c2bbd8f`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge table_support_failure` -> passed with the SemanticIR and IntentIR table-support diagnostic tests
- `cargo test -p specforge evidence_table_provenance` -> passed with the EvidenceIR table-provenance diagnostic tests
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `326` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG canonical table-support diagnostic coverage)

### Added: focused failure coverage for canonical signal table-support expectations
- Added `kg_bench_reports_canonical_table_support_failure`, a focused `kg-bench` unit test for `signal_supporting_table_ids_include`.
- Refactored the one-row structured signal-table fixture helper so it can drive either EvidenceIR expectations or canonical-stage expectations without duplicating the realistic `SourceIR` table patch.
- The new test deliberately expects `XREQ` to carry a missing table id even though the canonical signal is backed by `table_protocol_signal_description`.
- The assertion verifies the failure message names the fixture, the `signal_supporting_table_ids_include[XREQ]` field, the missing expected table id, and the actual table id carried by the canonical signal.
- This gives the canonical table-provenance expectation the same diagnostic protection now present for EvidenceIR table-provenance count, missing-record, and synthesized-statement mismatch failures.
- `MEMORY.md` was also brought forward from the stale `69351aa` baseline to the latest committed local baseline `ae4ea54`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_reports_canonical_table_support_failure` -> passed
- `cargo test -p specforge evidence_table_provenance` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `325` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG evidence provenance count diagnostic coverage)

### Added: focused failure coverage for EvidenceIR table-provenance count expectations
- Added `kg_bench_reports_evidence_table_provenance_count_failure`, a focused `kg-bench` unit test for the `table_signal_declaration_provenance_count` expectation path.
- The test reuses the realistic one-row `Signal | Direction | Width | Description` fixture helper, then deliberately expects zero provenance even though the table produces one EvidenceIR table-signal provenance record.
- The assertion verifies the failure message names the fixture, the `table_signal_declaration_provenance_count` field, and the expected/actual count mismatch.
- This completes the diagnostic self-test set around EvidenceIR table provenance expectations: count mismatch, missing table/signal provenance record, and mismatched synthesized statement text are all covered.
- `MEMORY.md` was also brought forward from the stale `cc7e533` baseline to the latest committed local baseline `69351aa`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge evidence_table_provenance` -> passed with `3` EvidenceIR provenance diagnostic tests
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `324` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG evidence table-provenance count expectation)

### Added: KG fixtures can assert EvidenceIR table-provenance counts directly
- Added `table_signal_declaration_provenance_count` to the `specforge kg-bench` EvidenceIR expectation surface.
- The count checks `EvidenceIr.table_signal_declaration_provenance.len()` without requiring the fixture to run validation first.
- Strengthened `table_misclassification_field_table_negative` so a misclassified `Bits | Name | Description` field table must produce zero EvidenceIR table-signal provenance records.
- The same fixture now also asserts the persisted EvidenceIR validation metric `table_signal_declaration_provenance: 0`, keeping exact IR-shape and validator-surface expectations aligned.
- This closes the negative side of the table-provenance guard: true signal tables must produce traceable declarations, while field tables must not create fake table-backed signal provenance at all.

### Validation
- `cargo fmt --all` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality table_misclassification_field_table_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `323` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG evidence provenance missing-record diagnostic coverage)

### Added: focused failure coverage for missing EvidenceIR table-provenance records
- Added `kg_bench_reports_missing_evidence_table_provenance_failure`, a sibling negative test for the `table_signal_declaration_provenance_include` expectation path.
- The test builds a temporary fixture with a real one-row signal table, then deliberately expects the right signal from the wrong source table id.
- It verifies that `kg-bench` reports the fixture name, the `table_signal_declaration_provenance_include` field, the missing expected table id, and the actual table id carried by EvidenceIR.
- Refactored the two EvidenceIR table-provenance diagnostic tests through a shared one-row signal-table fixture builder, keeping the failure coverage compact while preserving realistic `SourceIR` table patches.
- This complements the previous statement-text mismatch coverage: the harness now self-tests both important negative branches for EvidenceIR table provenance expectations.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge evidence_table_provenance` -> passed with both EvidenceIR provenance diagnostic tests
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `323` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (KG evidence provenance failure diagnostic coverage)

### Added: focused failure coverage for EvidenceIR table-provenance expectations
- Added a focused `kg-bench` unit test that builds a temporary fixture with a real one-row `Signal | Direction | Width | Description` table.
- The fixture deliberately expects the wrong synthesized statement text for `XREQ`, proving `table_signal_declaration_provenance_include` fails when the provenance record points at a different declaration than expected.
- Tightened the statement-text mismatch diagnostic so it names `table_signal_declaration_provenance_include`, matching the missing-record diagnostic and making fixture failures easier to interpret.
- This hardens the new EvidenceIR provenance expectation surface itself: tracked fixtures now have both positive coverage through `signal_table_inventory_authority_negative` and negative diagnostic coverage through the unit test.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge kg_bench_reports_evidence_table_provenance_statement_failure` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `322` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (EvidenceIR table provenance KG expectations)

### Added: KG fixtures can assert EvidenceIR table-signal provenance records directly
- Added an `evidence` expectation section to `specforge kg-bench`.
- Added `table_signal_declaration_provenance_include`, allowing tracked fixtures to require that a table-synthesized signal declaration links a specific `signal_name` to a specific `SourceIR` `table_id`.
- The expectation can also require the synthesized `statement_text`, proving the provenance record points at the intended generated declaration rather than only matching a table/signal pair by count.
- Strengthened `signal_table_inventory_authority_negative` so `XREQ`, `XACK`, and `PAYLOAD` must each carry exact EvidenceIR table provenance before the later SemanticIR / IntentIR table-support checks run.
- This complements, but does not replace, the `table_signal_declaration_provenance` validation metric: the metric gives aggregate visibility, while the KG expectation locks exact evidence shape.

### Validation
- `cargo fmt --all` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality signal_table_inventory_authority_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `321` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (README bootstrap continuity sync after EvidenceIR metric)

### Fixed: memory baseline now points at the committed EvidenceIR provenance metric slice
- Executed the README bootstrap path by reading `README.md`, `SESSION_BOOTSTRAP.md`, the current roadmap slice, the recent change log, and the Rust codebase analysis.
- Re-ran the Rust codebase sanity inventory from the current checkout: `30` Rust source files and `59,520` total Rust source lines under `crates/specforge/src`.
- Confirmed `cargo metadata --no-deps --format-version 1` still reports the single active `specforge` workspace package with library and binary targets.
- Confirmed the CLI dispatch still exposes the staged pipeline plus validation, benchmark, rescan, prior-memory, corpus-KB, and NLP/VLM enrichment commands documented in README.
- Updated `MEMORY.md` so the latest committed baseline is `f58ce2e feat(validation): report evidence table provenance` instead of the prior pre-commit baseline.
- Left `RUST_CODEBASE_ANALYSIS.md` unchanged because it already records the EvidenceIR provenance validation metric, the current command surface, the staged IR architecture, local CI boundary, and remaining semantic-truthfulness risks.

### Validation
- `git status --short --branch` -> confirmed branch was clean before the continuity-doc edit and ahead of `origin/main`
- Rust codebase inventory commands over `crates/specforge/src` -> completed
- `cargo metadata --no-deps --format-version 1` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (EvidenceIR table signal provenance validation metric)

### Added: EvidenceIR validation now reports table-signal declaration provenance
- Added `table_signal_declaration_provenance` to the `specforge validate` EvidenceIR structured-extraction surface.
- The metric counts the provenance links from formal signal declarations synthesized out of structured `SourceIR` signal-description tables back to the source table ids that authored them.
- Added focused unit coverage that builds markdown plus a structured `Signal | Direction | Width | Description` table, verifies `EvidenceIr.table_signal_declaration_provenance` contains three table links, and verifies EvidenceIR validation reports `table_signal_declaration_provenance: 3`.
- Strengthened `signal_table_inventory_authority_negative` so its persisted EvidenceIR validation expectations now lock the same metric, complementing the existing SemanticIR / IntentIR `with_table_support` checks.
- This is an evidence-stage visibility metric only: it does not promote truth, auto-fix missing provenance, or replace the canonical per-signal provenance checks in `SemanticIR` / `IntentIR`.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_evidence_ir_counts_table_signal_declaration_provenance` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality signal_table_inventory_authority_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `321` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (validator table-support unit coverage)

### Added: direct unit coverage for table-backed signal validation metrics
- Added a focused validator regression that builds the full `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` path from a structured `Signal | Direction | Width | Description` table.
- The test asserts `with_table_support: 3` at both `SemanticIR` and `IntentIR`, complementing the existing KG fixture-level assertion with a direct unit-level guard.
- The regression also keeps the metric honest as a table-provenance coverage signal: the signals are authored by the structured table, not by free-text signal declarations.

### Validation
- `cargo fmt --all` -> passed
- `cargo test -p specforge validate_semantic_and_intent_ir_count_signal_table_support` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `320` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (README bootstrap continuity sync after validation metric)

### Fixed: memory baseline now points at the latest committed validation metric slice
- Executed the README bootstrap path by reading `README.md`, `SESSION_BOOTSTRAP.md`, the fast-ramp root docs, the mdBook entrypoints, and the corpus-KB policy docs referenced by the project map.
- Re-ran the Rust codebase sanity inventory from the current checkout: `30` Rust source files and `59,341` total Rust source lines under `crates/specforge/src`.
- Confirmed `cargo metadata --no-deps` still reports the single active `specforge` workspace package with library and binary targets.
- Confirmed the CLI dispatch still exposes the staged pipeline plus validation, benchmark, rescan, prior-memory, corpus-KB, and NLP/VLM enrichment commands documented in README and the mdBook.
- Updated `MEMORY.md` so the latest committed baseline is `4f61ec6 feat(validation): report signal table support` instead of the prior pre-commit baseline.
- Left `RUST_CODEBASE_ANALYSIS.md` unchanged because it already records the current command surface, table-support validation metric, staged IR architecture, local CI boundary, and remaining semantic-truthfulness risks.

### Validation
- `git status --short --branch` -> confirmed branch was clean before the continuity-doc edit and ahead of `origin/main`
- Rust codebase inventory commands over `crates/specforge/src` -> completed
- `cargo metadata --no-deps --format-version 1` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (signal table support validation metric)

### Added: validator now reports table-backed signal coverage
- Added `with_table_support` to `specforge validate` for `SemanticIR` and `IntentIR`.
- The semantic-stage metric counts canonical interface signal records whose `supporting_table_ids` are non-empty.
- The intent-stage metric counts non-`Low` declared signal records whose `supporting_table_ids` are non-empty.
- This is a coverage and explainability metric only: table support helps users see whether recovered signals remain tied to structured `SourceIR` tables, while `kg-bench` per-signal expectations still lock the exact provenance shape.
- Hardened `signal_table_inventory_authority_negative` so its validation sidecar now requires `with_table_support: 3` at both `SemanticIR` and `IntentIR`, matching the three table-authored canonical signals `XREQ`, `XACK`, and `PAYLOAD`.

### Validation
- `cargo fmt --all` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality signal_table_inventory_authority_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no tracked corpus-KB content drift
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (README bootstrap continuity sync)

### Fixed: post-commit memory baseline no longer describes completed provenance work as in-flight
- Executed the README bootstrap path by reading `README.md`, `SESSION_BOOTSTRAP.md`, the referenced live ramp-up docs, and the Rust CLI/module inventory.
- Confirmed the current `RUST_CODEBASE_ANALYSIS.md` still matches the implemented command surface and the committed signal-table provenance architecture.
- Updated `MEMORY.md` so the latest committed baseline is `bb1def0 feat(ir): carry signal table provenance` instead of the prior `e96ebe1` baseline.
- Refreshed the recorded Rust source inventory from the current checkout: `30` Rust source files and `58,038` lines under `crates/specforge/src`.

### Validation
- `git status --short --branch` -> confirmed branch is clean before the continuity-doc edit and ahead of `origin/main`
- Rust codebase inventory commands over `crates/specforge/src` -> completed
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed
- README sentinel check -> passed

## 2026-04-17 (signal table provenance carry-through)

### Added: canonical interface signals retain their SourceIR table support
- Added `EvidenceIr.table_signal_declaration_provenance`, a compact provenance bridge from table-synthesized `Signal X is ...` declarations back to the originating `SourceIR` structured table id.
- Added `InterfaceSignalRecord.supporting_table_ids` in `SemanticIR` / `IntentIR`, so canonical interface signals can now say not only which synthesized statement supported them, but which source table produced that statement.
- Extended `specforge kg-bench` canonical expectations with `signal_supporting_table_ids_include`, allowing fixtures to assert table provenance as typed IR shape instead of relying on artifact inspection.
- Hardened `signal_table_inventory_authority_negative` so `XREQ`, `XACK`, and `PAYLOAD` must carry `table_protocol_signal_description` through both `SemanticIR` and `IntentIR`.
- Updated hand-built learning and semantic test records for the widened interface-signal schema while preserving empty table provenance for non-table-backed synthetic records.

### Validation
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality signal_table_inventory_authority_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` fixtures / `0` failures and no managed-page content drift
- `cargo test -p specforge --no-run` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (signal table inventory authority KG fixture)

### Added: signal tables remain authoritative over uppercase prose context
- Added tracked fixture `signal_table_inventory_authority_negative` under `crates/specforge/test_data/kg_quality/`.
- The fixture models a common protocol-PDF shape where surrounding prose mentions `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, and `SoC`, while the real interface signals are supplied by a structured `Signal | Direction | Width | Description` table.
- It proves `XREQ`, `XACK`, and `PAYLOAD` survive through `SemanticIR` / `IntentIR` with table-derived directions, while the uppercase engineering/context terms stay out of canonical signal inventory.
- Refreshed corpus-KB fixture projections from the `89/89` tracked KG suite; table extraction and hygiene coverage is now `9/9`, and truthfulness-negative/caution coverage is now `36/36`.

### Validation
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality signal_table_inventory_authority_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `89` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `89` fixtures / `0` failures
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-17 (KG signal-inventory exclusion expectations)

### Added: canonical signal inventories can now be guarded against false positives
- Added `signal_names_exclude` to `specforge kg-bench` canonical stage expectations, mirroring the existing include/exclude pattern used for graph direction, semantic roles, arbitration, residuals, and assumptions.
- Hardened `clock_reset_contract_scope_negative` so the fixture now proves that integration/document-scope vocabulary such as `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `PLLs`, `DFT`, and `SoC`/`SOC` does not enter the canonical `SemanticIR` or `IntentIR` signal inventory.
- Updated the fixture source to mention the `VIP` acronym explicitly while still preserving only `ACLK` and `ARESETN` as canonical signals and infrastructure records.
- This keeps the clock/reset protocol-scope guard precise: protocol PDFs may discuss verification IP, DFT, PLLs, and SoC-owned physical tree construction without those uppercase engineering terms becoming fake interface signals.

### Validation
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality clock_reset_contract_scope_negative` -> passed
- `cargo run -p specforge -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `88` passed / `0` failed fixtures
- `cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `88` fixtures / `0` failures
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-17 (clock/reset protocol scope KG negative fixture)

### Added: protocol PDFs cannot author physical clock/reset tree construction
- Added tracked fixture `clock_reset_contract_scope_negative` under `crates/specforge/test_data/kg_quality/`.
- The fixture proves an AMBA-style protocol PDF can define boundary-visible clock/reset contract semantics for RTL and verification IP while explicitly leaving physical clock-tree and reset-tree construction to the integrating SoC team.
- It preserves `ACLK` / `ARESETN` as first-class `system_clock` / `system_reset` infrastructure signals, but keeps concrete topology records, gated branches, reset-synchronizer stages, reset-tree targets, and ordinary actor ports at zero.
- Refreshed corpus-KB fixture projections from the `88/88` tracked KG suite; infrastructure semantics coverage is now `5/5`, infrastructure/polarity page coverage is `10/10`, and truthfulness-negative/caution coverage is `35/35`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality clock_reset_contract_scope_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `88` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `88` fixtures / `0` failures
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-17 (clock/reset protocol-document scope clarified)

### Clarified: protocol PDFs define interface contracts, not physical trees
- Captured the implementation boundary that AMBA/Intel-style protocol and chip-interface PDFs are primarily contract documents for RTL designers and verification-IP authors.
- Clarified that SPECFORGE should recover clock/reset contract semantics such as signal identity, polarity, reset kind, assertion/release timing, and boundary-visible obligations, but should not infer physical clock-tree or reset-tree implementation from those PDFs.
- Documented that real clock/reset trees are usually SoC/team-specific integration artifacts involving local clock generators, reset controllers, power domains, CDC/RDC policy, DFT constraints, CTS strategy, and physical implementation choices.
- Updated the public mdBook clock/reset chapter, architecture rationale, README, roadmap, and live continuity notes so future implementation keeps this scope boundary explicit.

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (clock/reset generic advice KG negative fixture)

### Added: generic clock/reset doctrine cannot author topology facts
- Added tracked fixture `clock_reset_generic_advice_negative` under `crates/specforge/test_data/kg_quality/`.
- The fixture proves generic clock/reset best-practice prose preserves real `ACLK` / `ARESETN` infrastructure signals, but does not create `infrastructure_topology` records, gated-clock branches, reset-synchronizer stages, reset-tree targets, or ordinary protocol actor ports.
- The guarded prose includes glitch-avoidance clock-gate policy, no-glue reset-tree guidance, possible synchronizer usage, and asynchronous reset assertion / synchronous reset release discipline.
- Refreshed corpus-KB fixture projections from the `87/87` tracked KG suite; infrastructure semantics coverage is now `4/4`, infrastructure/polarity page coverage is `9/9`, and truthfulness-negative/caution coverage is `34/34`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality clock_reset_generic_advice_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `87` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `87` fixtures / `0` failures
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-17 (FSMGEN response sync contract captured)

### Added: SPECFORGE records FSMGEN's accepted response
- Read FSMGEN's tracked response at `/Users/richarddje/Documents/github/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, observed at commit `7475f07`.
- Updated `docs/FSMGEN_FEEDBACK.md` with a `FSMGEN Response Received` section that records the accepted shared direction, near-term integration sequence, longer-term language features, and practical SPECFORGE planning rules.
- Updated the `R6 Adapter layer` roadmap so `.fsm` adapter planning targets strict-mode canonical `.fsm`, blocks compatibility syntax unless FSMGEN explicitly marks it safe for generated output, and plans future validation around capability manifests, stable diagnostic codes, check-only JSON, and normalized semantic JSON export.

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-17 (FSMGEN feedback introduces SPECFORGE explicitly)

### Clarified: FSMGEN handoff can be read without prior SPECFORGE context
- Added a `What SPECFORGE Is` section to `docs/FSMGEN_FEEDBACK.md`.
- The feedback now introduces SPECFORGE as a Rust toolchain that recovers typed implementation intent from chip-design specs through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`.
- The note now states that `IntentIR` is the backend-independent product and `.fsm` is one downstream adapter target, with FSMGEN important because `.fsm` can become the natural high-level lowering format for recovered control intent.

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-17 (FSMGEN feedback now targets IntentIR-aligned `.fsm` evolution)

### Clarified: feedback is about `.fsm` language co-evolution, not only validation tooling
- Expanded `docs/FSMGEN_FEEDBACK.md` so the primary request is now explicit: make `.fsm` a natural lowering format for SPECFORGE `IntentIR`, while staying aligned with FSMGEN's active development goals.
- Added language-level suggestions for first-class system contracts, actor-relative port semantics, interface/channel grouping, temporal/stability contracts, semantic signal roles, assumptions/residual/provenance metadata, direct-module root shape, and contract-aware composition.
- Kept the previous machine-readable manifest, JSON diagnostics, normalized AST/IR export, and adapter-facing corpus requests as support/tooling features rather than the whole feedback.

### Validation
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-17 (FSMGEN reference sync for `.fsm` adapter planning)

### Changed: FSMGEN submodule refreshed for adapter reconnaissance
- Advanced `subs/fsmgen` from `57f00e5` to `955f2bb` so SPECFORGE can use the current FSMGEN baseline while shaping the downstream `.fsm` adapter.
- The refreshed baseline includes FSMGEN's live mdBook under `subs/fsmgen/docs/book/`, making the reference no longer only a code checkout but also a progressive syntax/semantics documentation surface.
- The sync was scoped as reconnaissance for SPECFORGE adapter work: the submodule remains contextual and read-only from this repository, and SPECFORGE should continue changing its own adapter/IR layers rather than patching FSMGEN in-place.

### Captured: adapter-facing lessons and FSMGEN feedback
- The current FSMGEN direction reinforces SPECFORGE's adapter rule: target `.fsm` text should be emitted only when `IntentIR` facts are explicit enough; otherwise the adapter should preserve residual decisions instead of inventing syntax.
- FSMGEN's strict-mode/support-accounting work, typed diagnostic direction, aggregate/package/type expansion, composition/toplink typing, and live mdBook now provide better reference material for `.fsm` renderability gates and future adapter validation.
- Added tracked `docs/FSMGEN_FEEDBACK.md` so FSMGEN can read a focused SPECFORGE feedback document rather than mining scattered continuity notes.
- Suggested upstream FSMGEN features were captured there and in the live engineering notes: a machine-readable capability manifest, stable JSON diagnostics/check mode, normalized AST/IR export, richer reset/clock metadata, adapter-facing examples, and continued strict-mode-first support accounting.

### Validation
- `git diff --submodule=log -- subs/fsmgen` -> reviewed the upstream range from `57f00e5` to `955f2bb`
- `./bin/ci-regression` inside `subs/fsmgen` -> intentionally stopped after scope correction; it had reached `t/274-package-aggregate-values.t` with all reported tests green before termination
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-17 (FSM adapter derives direct target inputs from control reads)

### Changed: direct `.fsm` target-actor recovery now uses explicit control reads for inputs
- Extended the standalone direct `.fsm` graph overlay so once a target actor is selected, signals read by canonical DT/control guards and assignment expressions can become target-actor inputs in the adapter inventory.
- Output targets are explicitly excluded from this read-side recovery, so self-referential or assigned signals do not become fake inputs.
- Recovery remains adapter-local and bounded: it only strengthens signals already present in the direct inventory, it does not mutate canonical `IntentIR`, and it only runs after the target actor context is selected.

### Tests
- Added `standalone_dt_derives_target_inputs_from_control_reads_after_output_actor_selection`, which clears flat direct-interface directions, supplies only output-side `controller` actor ports, adds an external `environment` producer for `DATA_IN`, and proves `DATA_IN` is recovered as the target input from the explicit control reads rather than from the external actor perspective.
- Re-ran the focused standalone direct adapter set, including the ambiguity, direction-conflict, width-conflict, unrelated-actor, and output-owner selection guards.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_derives_target_inputs_from_control_reads_after_output_actor_selection -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_ -- --nocapture` -> passed with `8` focused tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `28` adapter tests
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (FSM adapter selects direct target actor from graph outputs)

### Changed: direct `.fsm` roots can ignore external reader/driver actors when the target actor is clear
- Tightened the standalone direct `.fsm` graph overlay so it first looks for one actor that graph-drives every render-critical assignment/init target already present in the direct inventory.
- This lets a direct root recover the intended target actor even when other local KG actors also share the same signals as external readers or input drivers.
- The bounded safety gates remain intact: the overlay still adds no graph-only signals, still falls back to the previous one-actor context rule when no output-target actor is available, and still leaves ambiguous or incomplete target-output graphs blocked.

### Tests
- Added `standalone_dt_selects_output_actor_when_external_actors_share_signals`, which clears flat direct-interface directions and proves the `.fsm` adapter still lowers when `controller` drives `DATA_OUT` / `ZERO_FLAG`, while an `environment` drives `DATA_IN` and a `monitor` reads the outputs.
- Re-ran the focused standalone direct adapter set, including the existing ambiguous-context, direction-conflict, width-conflict, and unrelated-actor guards.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_selects_output_actor_when_external_actors_share_signals -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt_ -- --nocapture` -> passed with `7` focused tests
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with `27` adapter tests
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `318` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (coordinated active-read KG coverage)

### Strengthened: relative-clause actor-noise fixture now locks coordinated reads too
- Tightened the tracked `relative_clause_actor_noise_negative` fixture so the manager-side sample relation is one coordinated sentence: `The Manager samples ARCHUNKEN and RCHUNKV.`
- Added focused extractor regressions proving coordinated active-read clauses recover every sampled object and stop before guard clauses such as `when RVALID is HIGH`.
- The fixture now proves the same staged graph path handles both producer and consumer coordination: `interconnect` drives both chunking signals, `Manager` reads both chunking signals, `mixture of` stays out of the graph, and `signal_connectivity_conflicts` remains `0`.

### Validation
- `cargo test --manifest-path Cargo.toml coordinated_active_read -- --nocapture` -> passed with `2` focused tests
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality relative_clause_actor_noise_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `86` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `317` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (relative-clause actor-noise KG fixture)

### Added: staged KG benchmark coverage for relative-clause actor hygiene
- Added tracked fixture `relative_clause_actor_noise_negative` for AXI-style chunking prose where an interconnect with a relative clause can drive both `ARCHUNKEN` and `RCHUNKV`.
- The fixture locks the full staged behavior through `SemanticIR` and `IntentIR`: `interconnect` must drive both chunking signals, `Manager` must read both signals, and `signal_connectivity_conflicts` must remain `0`.
- Strengthened active object parsing so coordinated objects after an active verb are recovered from the same clause; `can drive ARCHUNKEN and RCHUNKV` now yields producer edges for both signals, not only the first one.
- Refreshed corpus-KB KG projections so the tracked suite now reports `86` fixtures / `0` failures, actor-connectivity coverage reports `12/12`, and truthfulness-negative/caution coverage reports `33/33`.

### Validation
- `cargo test --manifest-path Cargo.toml relative_clause_active_drive_extracts_head_subject_not_mixture_phrase -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml coordinated_active_drive_extracts_real_actor_not_payload_phrase -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml coordinated_active_drive_object_scan_stops_before_guard_clause -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality relative_clause_actor_noise_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `86` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `86` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `315` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (relative-clause actor extraction hygiene)

### Fixed: active-drive prose no longer promotes descriptive relative-clause phrases into actors
- Tightened active actor extraction so subject parsing stays inside the current sentence and trims relative clauses such as `which ...`, `that ...`, `who ...`, and `whose ...` before selecting the actor for `drives SIGNAL` / `can drive SIGNAL` prose.
- Added shared actor-term hygiene for `mixture` / `mixture of`, preventing descriptive support phrases from becoming protocol actors or learned actor-taxonomy terms.
- Added regression coverage for AXI read-data chunking prose where an interconnect connected to components with a mixture of chunking support can drive `ARCHUNKEN` / `RCHUNKV`; the parser now keeps `interconnect` as the actor and rejects the fake `mixture of` actor.
- Rebuilt the local AXI `EvidenceIR -> SemanticIR -> IntentIR` chain from the existing generated `SourceIR`; the stale fake `mixture of` producer is gone, while the remaining `ARCHUNKEN` ambiguity is now the real `Manager` versus `interconnect` nuance and the score remains `85/100 GOOD`.

### Validation
- `cargo test --manifest-path Cargo.toml relative_clause_active_drive_extracts_head_subject_not_mixture_phrase -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml coordinated_active_drive_extracts_real_actor_not_payload_phrase -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml active_drive_pattern_extracts_actor_and_signal -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` -> passed with `6974` extracted statements
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` -> passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` -> passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` -> passed with score `85/100 GOOD`; `ARCHUNKEN` now reports `Manager` versus `interconnect`, not `Manager` versus `mixture of`
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `314` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (AHB exclusive/security stability KG fixture)

### Added: AHB exclusive/security wait-state stability truthfulness fixture
- Added tracked KG fixture `ahb_exclusive_security_stability_gold` for AHB `HEXCL`, `HNONSEC`, and `HEXOKAY` hold behavior while `HREADY` is low and `HSEL` is high.
- The fixture uses `Manager signals` / `Subordinate signals` section context plus `Name | Destination | Width | Description` tables so exclusive, security, select, wait-state, and exclusive-response ownership are recovered from AHB-style document structure.
- The fixture intentionally mixes Manager-owned controls (`HEXCL`, `HNONSEC`) with a Subordinate-owned response (`HEXOKAY`) under the same wait-state guard, proving actor-grounded stability survives across both sides without fabricating a handshake-completion predicate.
- Refreshed corpus-KB KG projections so the tracked suite now reports `85` fixtures / `0` failures, AMBA-family coverage reports `36/36`, and temporal fixture coverage reports `42/42`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_exclusive_security_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `85` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `85` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (AHB transfer/lock stability KG fixture)

### Added: AHB transfer/lock wait-state stability truthfulness fixture
- Added tracked KG fixture `ahb_transfer_lock_stability_gold` for AHB `HTRANS` and `HMASTLOCK` hold behavior while `HREADY` is low and `HSEL` is high.
- The fixture uses `Manager signals` / `Subordinate signals` section context plus `Name | Destination | Width | Description` tables so transfer/lock ownership is recovered from document structure rather than hardcoded signal names.
- Each temporal rule requires the stalled-transfer antecedents and actor-grounded Manager stability while intentionally preserving `temporal_rules_with_handshake_completion: 0`, proving wait-state transfer/control attributes do not become false completed handshakes.
- Refreshed corpus-KB KG projections so the tracked suite now reports `84` fixtures / `0` failures, AMBA-family coverage reports `35/35`, and temporal fixture coverage reports `41/41`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_transfer_lock_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `84` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `84` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (APB address/protection stability KG fixture)

### Added: APB address/protection wait-state stability truthfulness fixture
- Added tracked KG fixture `apb_address_protection_stability_gold` for APB `PADDR` and `PPROT` hold behavior while `PSEL` and `PENABLE` are high and `PREADY` is low.
- The fixture uses a `Signal | Source | Width | Description` table so Requester/Completer ownership is recovered from structured table evidence and reinforced by prose actor relations.
- Each temporal rule requires three explicit wait-state antecedents and actor-grounded Requester stability while intentionally preserving `temporal_rules_with_handshake_completion: 0`, proving stalled APB accesses do not become false completed handshakes.
- Refreshed corpus-KB KG projections so the tracked suite now reports `83` fixtures / `0` failures, AMBA-family coverage reports `34/34`, and temporal fixture coverage reports `40/40`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_address_protection_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `83` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `83` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (AXI address/response USER sideband stability KG fixture)

### Added: AXI address/response USER sideband stability truthfulness fixture
- Added tracked KG fixture `axi_address_response_user_sideband_stability_gold` for AXI address-channel and write-response `USER` sideband hold behavior across `AWUSER`, `ARUSER`, and `BUSER` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture intentionally pairs Manager-owned address USER fields with a Subordinate-owned write-response USER field: `AWUSER` is guarded by `AWVALID` / `AWREADY`, `ARUSER` is guarded by `ARVALID` / `ARREADY`, and `BUSER` is guarded by `BVALID` / `BREADY`.
- Each temporal rule requires the matching typed `HandshakeComplete(...)` predicate and actor-grounded stability for the owning producer, extending optional user-defined sideband coverage beyond the data channels.
- Refreshed corpus-KB KG projections so the tracked suite now reports `82` fixtures / `0` failures, AMBA-family coverage reports `33/33`, and temporal fixture coverage reports `39/39`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_address_response_user_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `82` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `82` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-17 (AXI data USER sideband stability KG fixture)

### Added: AXI data USER sideband stability truthfulness fixture
- Added tracked KG fixture `axi_data_user_sideband_stability_gold` for AXI write/read data `USER` sideband hold behavior across `WUSER` and `RUSER` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture intentionally pairs opposite producer directions: `WUSER` is a Manager-owned write-data sideband under `WVALID` / `WREADY`, while `RUSER` is a Subordinate-owned read-data sideband under `RVALID` / `RREADY`.
- Each temporal rule requires the matching typed `HandshakeComplete(...)` predicate and actor-grounded stability for the owning producer, proving optional user-defined sidebands remain owned protocol obligations rather than inert table rows.
- Refreshed corpus-KB KG projections so the tracked suite now reports `81` fixtures / `0` failures, AMBA-family coverage reports `32/32`, and temporal fixture coverage reports `38/38`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_data_user_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `81` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `81` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI address QoS/region sideband stability KG fixture)

### Added: AXI address QoS/region sideband stability truthfulness fixture
- Added tracked KG fixture `axi_address_qos_region_sideband_stability_gold` for paired AXI write/read address QoS and region sideband hold behavior across `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks both address-channel handshakes: `AWVALID` / `AWREADY` guard write-address QoS/region stability, and `ARVALID` / `ARREADY` guard read-address QoS/region stability.
- Each temporal rule requires the matching typed `HandshakeComplete(...)` predicate and a Manager-grounded `actor_maintains_signal_stable` consequent, proving these sidebands remain actor-owned protocol obligations rather than inert table rows.
- Refreshed corpus-KB KG projections so the tracked suite now reports `80` fixtures / `0` failures, AMBA-family coverage reports `31/31`, and temporal fixture coverage reports `37/37`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_address_qos_region_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `80` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `80` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address control sideband stability KG fixture)

### Added: AXI read-address control sideband stability truthfulness fixture
- Added tracked KG fixture `axi_read_address_control_sideband_stability_gold` for AXI read-address control sideband hold behavior across `ARPROT`, `ARCACHE`, and `ARLOCK` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARPROT`, `ARCACHE`, and `ARLOCK` through graph-backed actor relations, actor-relative ports, resolved `ARVALID` / `ARREADY` semantic roles, and three manager-grounded temporal stability rules.
- Each temporal rule requires `HandshakeComplete(ARVALID, ARREADY)` and actor-grounded stability for the sideband subject, pairing the prior write-address control-sideband coverage with the read-address protection, cache, and lock attributes.
- Refreshed corpus-KB KG projections so the tracked suite now reports `79` fixtures / `0` failures, AMBA-family coverage reports `30/30`, and temporal fixture coverage reports `36/36`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_control_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `79` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `79` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-address control sideband stability KG fixture)

### Added: AXI write-address control sideband stability truthfulness fixture
- Added tracked KG fixture `axi_write_address_control_sideband_stability_gold` for AXI write-address control sideband hold behavior across `AWPROT`, `AWCACHE`, and `AWLOCK` with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `AWVALID`, `AWREADY`, `AWPROT`, `AWCACHE`, and `AWLOCK` through graph-backed actor relations, actor-relative ports, resolved `AWVALID` / `AWREADY` semantic roles, and three manager-grounded temporal stability rules.
- Each temporal rule requires `HandshakeComplete(AWVALID, AWREADY)` and actor-grounded stability for the sideband subject, extending write-address coverage beyond burst length/size/type and transaction ID into protection, cache, and lock attributes.
- Refreshed corpus-KB KG projections so the tracked suite now reports `78` fixtures / `0` failures, AMBA-family coverage reports `29/29`, and temporal fixture coverage reports `35/35`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_address_control_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `78` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `78` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address ID stability KG fixture)

### Added: AXI read-address ID stability truthfulness fixture
- Added tracked KG fixture `axi_read_address_id_stability_gold` for AXI read-address `ARID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, and `ARID` through graph-backed actor relations, actor-relative ports, resolved `ARVALID` / `ARREADY` semantic roles, and one manager-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(ARVALID, ARREADY)` and actor-grounded stability for `ARID`, completing explicit transaction-ID stability coverage across AXI address, response, and read-data channels.
- Refreshed corpus-KB KG projections so the tracked suite now reports `77` fixtures / `0` failures, AMBA-family coverage reports `28/28`, and temporal fixture coverage reports `34/34`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `77` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `77` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-address ID stability KG fixture)

### Added: AXI write-address ID stability truthfulness fixture
- Added tracked KG fixture `axi_write_address_id_stability_gold` for AXI write-address `AWID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `AWVALID`, `AWREADY`, and `AWID` through graph-backed actor relations, actor-relative ports, resolved `AWVALID` / `AWREADY` semantic roles, and one manager-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(AWVALID, AWREADY)` and actor-grounded stability for `AWID`, extending write-address sideband coverage into transaction identity rather than only burst length/size/type fields.
- Refreshed corpus-KB KG projections so the tracked suite now reports `76` fixtures / `0` failures, AMBA-family coverage reports `27/27`, and temporal fixture coverage reports `33/33`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_address_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `76` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `76` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data ID stability KG fixture)

### Added: AXI read-data ID stability truthfulness fixture
- Added tracked KG fixture `axi_read_data_id_stability_gold` for AXI read-data `RID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, and `RID` through graph-backed actor relations, actor-relative ports, resolved `RVALID` / `RREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(RVALID, RREADY)` and actor-grounded stability for `RID`, pairing the prior write-response `BID` identity-sideband coverage with the read-data channel identity path.
- Refreshed corpus-KB KG projections so the tracked suite now reports `75` fixtures / `0` failures, AMBA-family coverage reports `26/26`, and temporal fixture coverage reports `32/32`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `75` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `75` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-response ID stability KG fixture)

### Added: AXI write-response ID stability truthfulness fixture
- Added tracked KG fixture `axi_write_response_id_stability_gold` for AXI write-response `BID` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `BVALID`, `BREADY`, and `BID` through graph-backed actor relations, actor-relative ports, resolved `BVALID` / `BREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(BVALID, BREADY)` and actor-grounded stability for `BID`, extending write-response coverage from payload response semantics into transaction identity sideband stability.
- Refreshed corpus-KB KG projections so the tracked suite now reports `74` fixtures / `0` failures, AMBA-family coverage reports `25/25`, and temporal fixture coverage reports `31/31`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_response_id_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `74` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `74` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data response stability KG fixture)

### Added: AXI read-data response stability truthfulness fixture
- Added tracked KG fixture `axi_read_data_response_stability_gold` for AXI read-data `RRESP` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, and `RRESP` through graph-backed actor relations, actor-relative ports, resolved `RVALID` / `RREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(RVALID, RREADY)` and actor-grounded stability for `RRESP`, extending read-data channel stability coverage beyond `RDATA` payload stability and `RLAST` last-beat stability.
- Refreshed corpus-KB KG projections so the tracked suite now reports `73` fixtures / `0` failures, AMBA-family coverage reports `24/24`, and temporal fixture coverage reports `30/30`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_response_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `73` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `73` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address sideband stability KG fixture)

### Added: AXI read-address sideband stability truthfulness fixture
- Added tracked KG fixture `axi_read_address_sideband_stability_gold` for AXI read-address `ARSIZE` / `ARBURST` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARSIZE`, and `ARBURST` through graph-backed actor relations, actor-relative ports, resolved `ARVALID` / `ARREADY` semantic roles, and two manager-grounded temporal stability rules.
- The temporal rules require `HandshakeComplete(ARVALID, ARREADY)` and actor-grounded stability for `ARSIZE` and `ARBURST`, extending read-address sideband coverage beyond the earlier `ARLEN` path.
- Refreshed corpus-KB KG projections so the tracked suite now reports `72` fixtures / `0` failures, AMBA-family coverage reports `23/23`, and temporal fixture coverage reports `29/29`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `72` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `72` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-data last stability KG fixture)

### Added: AXI write-data last-beat stability truthfulness fixture
- Added tracked KG fixture `axi_write_data_last_stability_gold` for AXI write-data `WLAST` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `WVALID`, `WREADY`, and `WLAST` through graph-backed actor relations, actor-relative ports, resolved `WVALID` / `WREADY` semantic roles, and one manager-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(WVALID, WREADY)` and actor-grounded stability for `WLAST`, proving a last-beat sideband flag remains an owned manager temporal obligation rather than inert table inventory.
- Refreshed corpus-KB KG projections so the tracked suite now reports `71` fixtures / `0` failures, AMBA-family coverage reports `22/22`, and temporal fixture coverage reports `28/28`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_data_last_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `71` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `71` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data last stability KG fixture)

### Added: AXI read-data last-beat stability truthfulness fixture
- Added tracked KG fixture `axi_read_data_last_stability_gold` for AXI read-data `RLAST` hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, and `RLAST` through graph-backed actor relations, actor-relative ports, resolved `RVALID` / `RREADY` semantic roles, and one subordinate-grounded temporal stability rule.
- The temporal rule requires `HandshakeComplete(RVALID, RREADY)` and actor-grounded stability for `RLAST`, proving a last-beat sideband flag remains an owned subordinate temporal obligation rather than inert table inventory.
- Refreshed corpus-KB KG projections so the tracked suite now reports `70` fixtures / `0` failures, AMBA-family coverage reports `21/21`, and temporal fixture coverage reports `27/27`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_last_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `70` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `70` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-address sideband stability KG fixture)

### Added: AXI write-address sideband stability truthfulness fixture
- Added tracked KG fixture `axi_write_address_sideband_stability_gold` for AXI write-address sideband hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `AWVALID`, `AWREADY`, `AWLEN`, `AWSIZE`, and `AWBURST` through graph-backed actor relations, actor-relative ports, resolved `AWVALID` / `AWREADY` semantic roles, and three manager-grounded temporal stability rules.
- The temporal rules require `HandshakeComplete(AWVALID, AWREADY)` and actor-grounded stability for `AWLEN`, `AWSIZE`, and `AWBURST`, proving non-handshake write-address sideband fields remain owned temporal obligations rather than inert table inventory.
- Refreshed corpus-KB KG projections so the tracked suite now reports `69` fixtures / `0` failures, AMBA-family coverage reports `20/20`, and temporal fixture coverage reports `26/26`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_address_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `69` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `69` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AHB write-data stability KG fixture)

### Added: AHB write-data stability truthfulness fixture
- Added tracked KG fixture `ahb_write_data_stability_gold` for AHB write-data hold behavior with `Manager signals` / `Subordinate signals` section context, `Name | Destination | Width | Description` table evidence, and an explicit structured signal constraint.
- The fixture locks `HSEL`, `HWRITE`, `HWDATA`, and `HREADY` through graph-backed actor relations, actor-relative ports, and one manager-grounded temporal stability rule.
- The temporal rule deliberately uses the write wait-state guard `HREADY LOW`, `HSEL HIGH`, and `HWRITE HIGH`, so the fixture requires `0` handshake-completion predicates while proving actor-grounded stability for the manager-owned write-data bus.
- Refreshed corpus-KB KG projections so the tracked suite now reports `68` fixtures / `0` failures, AMBA-family coverage reports `19/19`, and temporal fixture coverage reports `25/25`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_write_data_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `68` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `68` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AHB response stability KG fixture)

### Added: AHB response stability truthfulness fixture
- Added tracked KG fixture `ahb_response_stability_gold` for AHB wait-state read-data/response hold behavior with `Manager signals` / `Subordinate signals` section context, `Name | Destination | Width | Description` table evidence, and explicit structured signal constraints.
- The fixture locks `HSEL`, `HREADY`, `HRDATA`, and `HRESP` through graph-backed actor relations, actor-relative ports, and two subordinate-grounded temporal stability rules.
- The temporal rules deliberately use the stalled-transfer guard `HREADY LOW` and `HSEL HIGH`, so the fixture requires `0` handshake-completion predicates while still proving actor-grounded stability for subordinate-owned response outputs.
- Refreshed corpus-KB KG projections so the tracked suite now reports `67` fixtures / `0` failures, AMBA-family coverage reports `18/18`, and temporal fixture coverage reports `24/24`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_response_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `67` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `67` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AHB control stability KG fixture)

### Added: AHB control stability truthfulness fixture
- Added tracked KG fixture `ahb_control_stability_gold` for AHB wait-state address/control hold behavior with `Manager signals` / `Subordinate signals` section context, `Name | Destination | Width | Description` table evidence, and explicit structured signal constraints.
- The fixture locks `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, `HPROT`, `HSEL`, and `HREADY` through graph-backed actor relations, actor-relative ports, and five manager-grounded temporal stability rules.
- The temporal rules deliberately use the stalled-transfer guard `HREADY LOW` and `HSEL HIGH`, so the fixture requires `0` handshake-completion predicates while still proving actor-grounded stability for manager-owned address/control outputs.
- Refreshed corpus-KB KG projections so the tracked suite now reports `66` fixtures / `0` failures, AMBA-family coverage reports `17/17`, and temporal fixture coverage reports `23/23`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_control_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `66` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `66` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (APB response stability KG fixture)

### Added: APB response stability truthfulness fixture
- Added tracked KG fixture `apb_response_stability_gold` for APB completion-side response hold behavior with `Signal | Source | Width | Description` table evidence plus explicit structured signal constraints.
- The fixture locks `PSEL`, `PENABLE`, `PREADY`, `PRDATA`, and `PSLVERR` through graph-backed actor relations, actor-relative ports, resolved `PSEL` / `PREADY` semantic roles, and two completer-grounded temporal stability rules.
- Unlike the APB wait-state write-control fixture, this fixture expects `HandshakeComplete(PSEL, PREADY)` because `PREADY` is `HIGH`; this makes the pair an executable contrast between wait-state stability and completed-transfer response stability.
- Refreshed corpus-KB KG projections so the tracked suite now reports `65` fixtures / `0` failures, AMBA-family coverage reports `16/16`, and temporal fixture coverage reports `22/22`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_response_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `65` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `65` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (APB write-control stability KG fixture)

### Added: APB write-control stability truthfulness fixture
- Added tracked KG fixture `apb_write_control_stability_gold` for APB wait-state control/data hold behavior with `Signal | Source | Width | Description` table evidence plus explicit structured signal constraints.
- The fixture locks `PSEL`, `PENABLE`, `PREADY`, `PWRITE`, `PWDATA`, and `PSTRB` through graph-backed actor relations, actor-relative ports, resolved `PSEL` / `PREADY` semantic roles, and three actor-grounded temporal stability rules.
- The temporal rules deliberately require no `HandshakeComplete` predicate because the guard is the APB wait-state shape `PSEL HIGH`, `PENABLE HIGH`, and `PREADY LOW`; the value is in proving stable requester-owned control/data sidebands without pretending a transfer completed.
- Refreshed corpus-KB KG projections so the tracked suite now reports `64` fixtures / `0` failures, AMBA-family coverage reports `15/15`, and temporal fixture coverage reports `21/21`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_write_control_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `64` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `64` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI sideband stability KG fixture)

### Added: AXI sideband stability truthfulness fixture
- Added tracked KG fixture `axi_sideband_stability_gold` for AXI read-address and write-data sideband hold behavior with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARLEN`, `WVALID`, `WREADY`, and `WSTRB` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, and two actor-grounded handshake-stability temporal rules for `ARLEN` and `WSTRB`.
- Tightened corpus-KB fixture-family labeling so stability-hold fixtures count under temporal semantics, not only protocol-family coverage.
- Refreshed corpus-KB KG projections so the tracked suite now reports `63` fixtures / `0` failures, AMBA-family coverage reports `14/14`, and temporal fixture coverage reports `20/20`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_sideband_stability_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `63` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib corpus_kb::tests::kg_fixture_family_labels_are_deterministic_and_review_facing -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `63` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-address timing KG fixture)

### Added: AXI read-address timing truthfulness fixture
- Added tracked KG fixture `axi_read_address_timing_gold` for an AXI read-address channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `ARVALID`, `ARREADY`, `ARADDR`, and `ARLEN` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle read-address ready assertion, and `ARADDR` stability across an `ARVALID` / `ARREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `62` fixtures / `0` failures, AMBA-family coverage reports `13/13`, and temporal fixture coverage reports `19/19`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_address_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `62` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `62` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI write-data timing KG fixture)

### Added: AXI write-data timing truthfulness fixture
- Added tracked KG fixture `axi_write_data_timing_gold` for an AXI write-data channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `WVALID`, `WREADY`, `WDATA`, and `WSTRB` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle write-data ready assertion, and `WDATA` stability across a `WVALID` / `WREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `61` fixtures / `0` failures, AMBA-family coverage reports `12/12`, and temporal fixture coverage reports `18/18`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_data_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `61` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `61` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-16 (AXI read-data timing KG fixture)

### Added: AXI read-data timing truthfulness fixture
- Added tracked KG fixture `axi_read_data_timing_gold` for an AXI read-data channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `RVALID`, `RREADY`, `RDATA`, and `RRESP` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle read-data valid assertion, and `RDATA` stability across an `RVALID` / `RREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `60` fixtures / `0` failures, AMBA-family coverage reports `11/11`, and temporal fixture coverage reports `17/17`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_read_data_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `60` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `60` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-15 (AXI write-response timing KG fixture)

### Added: AXI write-response timing truthfulness fixture
- Added tracked KG fixture `axi_write_response_timing_gold` for an AXI write-response channel with width-only `Name | Width | Description` table evidence plus prose actor relations.
- The fixture locks `BVALID`, `BREADY`, and `BRESP` through graph-backed actor ports, resolved valid-like / ready-like semantic roles, single-source semantic grounding, next-cycle response assertion, and `BRESP` stability across a `BVALID` / `BREADY` handshake.
- Refreshed corpus-KB KG projections so the tracked suite now reports `59` fixtures / `0` failures, AMBA-family coverage reports `10/10`, and temporal fixture coverage reports `16/16`.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_write_response_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `59` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `59` fixtures / `0` failures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build
- `git diff --check` -> passed

## 2026-04-14 (GitHub Actions temporarily manual-only)

### Changed: hosted CI no longer auto-runs on push or pull request
- `.github/workflows/ci.yml` is temporarily restricted to `workflow_dispatch` to conserve the account's remaining GitHub Actions minutes.
- The workflow still exists and still delegates to `./scripts/run_ci.sh`, so it can be run manually from GitHub when hosted validation is explicitly desired.
- Local validation remains unchanged: `bash scripts/run_ci.sh` is still the canonical Rust + docs gate to run before commits or before any future push.

### Validation
- `git diff --check` -> passed
- `rg -n '^on:|workflow_dispatch|push:|pull_request:' .github/workflows/ci.yml` -> passed with only the workflow root and `workflow_dispatch` trigger key present
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts semantic grounding strength)

### Added: canonical semantic-grounding strength expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `InterfaceSignalRecord.semantic_grounding_strength` directly at the `SemanticIR` and `IntentIR` stages.
- The new `semantic_grounding_strengths_include` matcher checks a signal name plus expected strength such as `single_source`, `multi_source`, or `cross_modality`, instead of relying only on aggregate validation counters.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so multimodal, VLM timing-note, visual-prior, and APB handshake paths lock exact grounding-strength shape.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality cross_modality_semantic_grounding_gold vlm_timing_semantic_grounding_gold visual_semantic_prior_guided_caption_gold apb_requester_completer_handshake_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts resolved semantic-role shape)

### Added: canonical resolved semantic-role expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `InterfaceSignalRecord.resolved_semantic_role` directly at the `SemanticIR` and `IntentIR` stages.
- The new `resolved_semantic_roles_include` matcher checks a signal name plus expected role such as `handshake_valid_like` or `handshake_ready_like`, instead of only proving that some semantic role was resolved.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so table, visual-caption, VLM timing-note, prior-guided visual, and APB handshake paths lock exact valid-like / ready-like canonical role shape.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality cross_modality_semantic_grounding_gold vlm_timing_semantic_grounding_gold visual_semantic_prior_guided_caption_gold apb_requester_completer_handshake_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts resolved polarity shape)

### Added: canonical resolved signal-polarity expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `InterfaceSignalRecord.resolved_polarity` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_polarities_include` matcher checks a signal name plus expected `active_high` / `active_low` polarity instead of relying only on `with_resolved_polarity` validation counts.
- Strengthened `non_reset_control_polarity_gold`, `multi_control_polarity_gold`, and `mixed_control_polarity_gold` so explicit asserted-when-level prose, collective active-low prose, and mixed clause-local active-low/active-high prose lock the exact resolved canonical polarity shape.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality non_reset_control_polarity_gold multi_control_polarity_gold mixed_control_polarity_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-14 (KG bench asserts polarity conflict shape)

### Added: canonical signal-polarity conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `signal_polarity_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_polarity_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, and included observations by polarity, source kind, supporting statement ids, and supporting table ids.
- Added tracked fixture `control_polarity_conflict_negative`, proving a `PRESETN` active-high prose observation and active-low signal-description-table observation remain an explicit carried polarity conflict instead of forcing a winner or hiding behind aggregate conflict counts.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality control_polarity_conflict_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `58` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `58` fixtures / `0` failures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `58` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts interface conflict shape)

### Added: canonical interface-signal conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `interface_signal_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `interface_signal_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, conflict kind, and included observation values plus supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_interface_conflict_caution_gold` so it locks the `DATA` conflict shape directly: `direction_mismatch` preserves `input` / `output` observations, `width_mismatch` preserves `8` / `16` observations, and prior-memory caution still cannot mutate that local interface-shape conflict away.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality negative_knowledge_prior_guided_interface_conflict_caution_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts connectivity conflict shape)

### Added: canonical signal-connectivity conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `signal_connectivity_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_connectivity_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, conflict kind, included conflicting actor ids/names, and supporting statement ids.
- Strengthened `multi_producer_conflict_negative` and `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` so they lock the `PREADY` multiple-producer conflict shape directly: `Completer` and `Monitor` both drive `PREADY`, and prior-memory caution still cannot mutate that local graph conflict away.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality multi_producer_conflict_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality negative_knowledge_prior_guided_connectivity_conflict_caution_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts semantic conflict shape)

### Added: canonical signal-semantic conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `signal_semantic_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `signal_semantic_conflicts_include` matcher supports partial checks for conflict signal, optional conflict id, and included conflict observations by semantic tag, source kind, source text, and supporting statement/table/visual evidence ids.
- Strengthened `visual_sources_semantic_conflict_negative` so it locks the multimodal disagreement shape directly: `XCTRL` has visual-caption evidence for `handshake_valid_like` and VLM timing-diagram annotation evidence for `handshake_ready_like`, with arbitration remaining non-decisive instead of forcing consensus.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_sources_semantic_conflict_negative` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts temporal conflict shape)

### Added: canonical temporal-conflict expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `temporal_conflicts` directly at the `SemanticIR` and `IntentIR` stages.
- The new `temporal_conflicts_include` matcher supports partial checks for signal name, phase, clock signal, edge, cycle window, antecedent predicates, conflicting values, supporting rule ids, and supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_temporal_conflict_caution_gold` so it locks the current-document `PREADY` `HIGH` / `LOW` contradiction shape directly while still proving prior-memory caution only adds rescan/corroboration guidance instead of mutating the conflict away.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality negative_knowledge_prior_guided_temporal_conflict_caution_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts temporal rule shape)

### Added: canonical temporal-rule expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `temporal_rules` directly at the `SemanticIR` and `IntentIR` stages.
- The new `temporal_rules_include` matcher supports partial checks for source text, clock signal, edge, cycle window, supporting statement ids, antecedent predicates, and consequent predicates.
- Strengthened the representative AXI next-cycle, APB setup/access, and AHB wait-state timing fixtures so they lock actor-grounded drive/stability predicates, compound guard predicates, handshake-completion predicates, and one-cycle windows as typed IR instead of only aggregate validation metrics.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_next_cycle_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_setup_access_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_wait_state_timing_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (KG bench asserts clock/reset infrastructure topology)

### Added: canonical infrastructure expectations in KG fixtures
- `kg-bench` fixtures can now assert canonical `infrastructure_signals` and `infrastructure_topology` records directly at the `SemanticIR` and `IntentIR` stages.
- Added tracked fixture `clock_reset_topology_gold`, proving explicit current-document clock-gate, reset-synchronizer, and reset-tree topology survives canonically while generic clock-gate/synchronizer advice does not inflate topology counts.
- Refreshed the corpus-KB benchmark and infrastructure/polarity fixture pages from the now `57`-fixture KG suite.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality clock_reset_topology_gold` -> passed
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed with the tracked fixture suite reporting `57` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `57` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `57` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build

## 2026-04-13 (VLM timing filters motion-only annotations)

### Fixed: motion-only VLM timing annotations no longer become timing constraints
- `SemanticIR` now treats non-quantitative waveform-motion prose in VLM timing `annotations[]`, such as `XREQ rises, remains stable, then falls`, as visual markup rather than a typed timing constraint.
- The filter is deliberately conservative: setup/hold/delay/timing terms, explicit temporal relation words, and numeric/cycle-bearing annotations remain eligible for timing extraction instead of being blanket-suppressed.
- Added tracked KG fixture `vlm_timing_motion_annotation_negative`, proving motion-only annotations stay out of `TimingConstraintRecord`s while a concrete document-grounded `HIGH` sample still becomes typed temporal evidence.
- Refreshed the corpus-KB benchmark, timing, visual, and semantic/truthfulness pattern pages from the now `56`-fixture KG suite.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_accepts_fenced_json_with_trailing_prose -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_motion_annotation_negative` -> passed after failing before the parser fix with `timing_constraints = 2`
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_semantic_grounding_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality` -> passed with `56` passed / `0` failed fixtures
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed corpus-KB fixture projections with `56` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g prior-candidate readiness manifest)

### Added: fixture-surface readiness for prior candidates
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_candidates/kg-fixture-candidates.json` beside the Markdown prior-candidate page.
- The JSON manifest is schema-versioned and records candidate kind, target `CorpusMemory` schema, readiness, fixture counts, supporting/positive/guard fixture names, gate identifiers, and the explicit non-mutation promotion boundary.
- `corpus_kb/prior_candidates/kg-fixture-candidates.md` now includes a `Readiness Summary` table, with paired prior families marked as `fixture_paired_review_ready` and the caution-only negative-knowledge family marked as `caution_surface_review_ready`.
- The manifest and table remain review-only corpus-KB artifacts. They do not approve individual priors, write `generated/prior_memory/corpus_memory.json`, or mutate canonical IR.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, fixture-family, Markdown prior-candidate, and JSON prior-candidate corpus-KB artifacts with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g typed prior-memory corpus KB page)

### Added: dedicated typed prior-memory fixture-family page
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_memory/kg-fixtures.md` from KG fixtures tagged as `typed prior memory`.
- Added `corpus_kb/prior_memory/README.md` to define this page family as reviewable synthesis for prior-guided gold/negative pairs, caution-only negative knowledge, local-grounding boundaries, and future `CorpusMemory` benchmark gaps.
- The live refresh currently projects `21` passing typed-prior-memory fixtures with explicit fixture-path provenance, while preserving human synthesis outside the managed block.
- This page remains non-promoting corpus knowledge: it is not `generated/prior_memory/corpus_memory.json`, does not write `CorpusMemory`, and cannot mutate canonical IR.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, typed prior-memory, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g state-machine corpus KB page)

### Added: dedicated state-machine fixture-family page
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/state_machines/kg-fixtures.md` from KG fixtures tagged as `VLM state machines`.
- Added `corpus_kb/state_machines/README.md` to define this page family as reviewable synthesis for VLM state labels, transition endpoint grounding, duplicate state merging, initial marker handling, and initial-cardinality validation behavior.
- The live refresh currently projects `5` passing state-machine fixtures with explicit fixture-path provenance, while preserving human synthesis outside the managed block.
- This page remains non-promoting corpus knowledge: it does not change KG-bench execution, validation scoring, canonical IR, typed prior memory, or adapter lowering.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, state-machine, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g prior-candidate gate matrix)

### Added: review-gated prior-candidate surface
- `corpus_kb/prior_candidates/kg-fixture-candidates.md` now includes `review_scope: family_surface_not_individual_prior` so the managed projection cannot be mistaken for individual prior promotion.
- The managed prior-candidate block now emits a `Promotion Gate Review Matrix` covering `schema_gate`, `fixture_gate`, `harvest_gate`, `consumer_gate`, and the non-mutation `promotion_boundary` for every candidate family.
- The gate matrix reflects already visible implementation surfaces for the seven `CorpusMemory` families while keeping the corpus-KB page review-only: it does not write prior memory, approve a prior record, or mutate canonical IR.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the prior-candidate corpus-KB page with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g semantic/truthfulness corpus KB patterns)

### Added: semantic and truthfulness pattern page family
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/patterns/kg-fixtures.md` from KG fixtures tagged as actor/connectivity, semantic role arbitration, negative knowledge, truthfulness negatives/cautions, or residual/caveat patterns.
- Added `corpus_kb/patterns/README.md` to define this page family as reviewable synthesis for why candidate facts are accepted, contested, blocked, or left as residuals.
- The live refresh currently projects `42` passing semantic/truthfulness pattern fixtures with explicit fixture-path provenance, while preserving human synthesis outside the managed block.
- This page remains non-promoting corpus knowledge: it does not change KG-bench execution, validation scoring, canonical IR, or typed prior memory.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, semantic/truthfulness pattern, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g prior-candidate corpus KB projection)

### Added: review-only prior-candidate bridge
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_candidates/kg-fixture-candidates.md` from KG fixtures that already encode prior-guided gold/negative/caution behavior.
- The candidate projection groups fixture-backed candidates for `actor_taxonomy_prior`, `semantic_phrase_prior`, `semantic_modality_reliability_prior`, `temporal_phrase_prior`, `table_shape_prior`, `visual_motif_prior`, and `negative_knowledge_prior`.
- Each candidate row records the target `CorpusMemory` schema surface, supporting fixture count, positive fixtures, guard/caution fixtures, and required promotion gates.
- The page is explicitly non-promoting: `promotion_status` is `candidate_not_promoted_review_required`, `canonical_mutation_allowed` is `false`, and `corpus_memory_mutation_allowed` is `false`.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate, fixture-family, and prior-candidate corpus-KB pages with `55` passed / `0` failed fixtures
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g dedicated corpus KB fixture-family pages)

### Added: dedicated KG fixture-derived page families
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes the aggregate benchmark page plus dedicated managed corpus-KB pages for table, visual, timing, infrastructure, and AMBA-family fixture patterns.
- Added tracked page-family roots and READMEs under `corpus_kb/tables/`, `corpus_kb/visuals/`, `corpus_kb/timing/`, `corpus_kb/infra/`, and `corpus_kb/protocols/`.
- Refreshed the new managed pages from the tracked KG suite: table fixtures `8/8`, visual fixtures `18/18`, timing fixtures `14/14`, infrastructure/polarity fixtures `6/6`, and AMBA-family fixtures `9/9`, all with explicit fixture-path provenance.
- The new pages remain review-only corpus synthesis. They preserve human synthesis outside managed blocks and do not change KG-bench execution, canonical IR, validation scoring, or typed prior memory.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed the aggregate plus dedicated KG fixture-family corpus-KB pages with `55` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g KG fixture-family corpus KB summary)

### Added: fixture-family benchmark synthesis
- `specforge corpus-kb --kg-fixtures-root ...` now projects a managed fixture-family summary table above the per-fixture KG benchmark results.
- The family summary is review-facing only: fixtures can appear in multiple orthogonal families, and the table does not mutate canonical IR, typed priors, or the executable `kg-bench` gate.
- Refreshed `corpus_kb/benchmarks/kg-fixtures.md` so the tracked suite now reports `55` total fixtures, `55` passed, `0` failed, plus family coverage for VLM timing/state-machine, actor connectivity, multimodal visual grounding, negative knowledge, polarity, AMBA-family protocols, semantic arbitration, table hygiene, temporal semantics, truthfulness negatives, and typed prior memory.
- Added focused corpus-KB coverage proving the managed KG projection preserves human synthesis while emitting family summaries.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed `corpus_kb/benchmarks/kg-fixtures.md` with `55` passed / `0` failed fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `313` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g quiet KG fixture validation path)

### Changed: benchmark validation can run quietly
- Added an internal quiet validation entrypoint so KG fixture execution can still persist validation sidecars/backannotations without printing full stage reports for every fixture.
- Switched `kg-bench` fixture evaluation to use that quiet path, which also keeps `specforge corpus-kb --kg-fixtures-root ...` concise when it projects KG fixture outcomes into the corpus knowledge base.
- Added a focused guard test proving the quiet-validation output flag restores its prior state after use.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib quiet_validation_output_guard_restores_previous_state -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib kg_bench_reports_fixture_failure -- --nocapture` -> passed with concise fixture-failure output
- `cargo test --manifest-path Cargo.toml --lib corpus_kb_refreshes_kg_fixture_results_without_replacing_human_synthesis -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed with concise corpus-KB refresh output and `55` passed / `0` failed fixtures
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed, including the full `55`-fixture tracked KG suite
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `312` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g KG fixture-result corpus KB projection)

### Added: benchmark-result page family
- Added `corpus_kb/benchmarks/` with a page-family README and the managed `kg-fixtures.md` projection.
- The new page records the tracked KG fixture suite outcome as reviewable corpus synthesis: `55` total fixtures, `55` passed, and `0` failed, with fixture paths kept as provenance.
- The page preserves human-authored synthesis outside the managed block and remains guidance rather than canonical truth promotion.

### Changed: `specforge corpus-kb`
- `specforge corpus-kb` now accepts optional `--kg-fixtures-root <fixture-root>` and repeated `--kg-fixture <fixture>` selectors, so it can refresh benchmark-result pages independently of validation-report pages.
- `kg_bench` now exposes an internal fixture-outcome collection seam reused by the corpus-KB projection while preserving the standalone `specforge kg-bench` command behavior.
- Documented the then-open follow-up quality caveat that the benchmark projection reused validation paths that printed detailed stage reports during fixture execution.

### Validation
- `cargo fmt --all` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib kg_bench -- --nocapture` -> passed, including the full `55`-fixture tracked KG suite
- `cargo run --manifest-path Cargo.toml -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` -> passed and refreshed `corpus_kb/benchmarks/kg-fixtures.md`
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `311` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-13 (R15g corpus knowledge base bootstrap)

### Added: tracked corpus knowledge-base plane
- Added `corpus_kb/` with a schema/policy root and the first `failures/` page family for reviewable cross-document synthesis.
- Seeded `corpus_kb/failures/validation-findings.md` from the current four projected AMBA `IntentIR` validation reports, preserving human synthesis outside the managed block.

### Added: `specforge corpus-kb`
- Added a new CLI command that refreshes the managed validation-finding block from validation report sidecars.
- The command preserves existing human-authored notes and only replaces the managed block, so corpus KB pages remain guidance and synthesis rather than canonical truth promotion.
- Added focused coverage for CLI parsing and managed-block preservation.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib corpus_kb -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- corpus-kb --repo-root . generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/validation_report.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/validation_report.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/validation_report.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/validation_report.json` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `307` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (VLM timing filters signal bit-select annotation labels)

### Fixed: standalone signal index labels stay out of timing constraints
- `SemanticIR` now treats standalone VLM timing annotation labels such as `XREQ[0]`, `XREQ<1>`, and `XREQ[3:0]` as waveform/bit-select markup when they appear only in `annotations[]`.
- The filter remains scoped to standalone timing annotations: grounded `signals[].values[]` observations for `XREQ` still become typed signal constraints and temporal rules when they carry concrete sampled values such as `LOW` and `HIGH`.
- Strengthened the existing `vlm_timing_spurious_annotation_negative` fixture and direct semantic regression again, deepening the same 55-fixture KG-quality surface rather than adding a redundant fixture.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (README bootstrap refresh)

### Changed: live bootstrap docs match the current Rust surface
- Executed the README -> `SESSION_BOOTSTRAP.md` handoff, re-read the referenced live docs, and compared their current claims against the active Rust codebase.
- Refreshed the README implementation-path map so it includes the active command modules for `doctor`, `converge`, `enrich`, `validate`, `rescan-plan`, `learn-priors`, `nlp-enrich`, plus `commands/mod.rs`, `test_support.rs`, and `ir/prior_memory.rs`.
- Refreshed `RUST_CODEBASE_ANALYSIS.md` and `MEMORY.md` so the current continuity baseline reflects commit `76274f6`, `29` Rust source files, `56,024` Rust source lines, `55` tracked KG-quality fixtures, and the latest `305`-test full-CI baseline.
- Normalized old checkout-specific repo-internal markdown links in the tracked changelog/memory surface to repo-relative paths.

### Validation
- repo-internal absolute checkout path scan across tracked markdown -> passed with no matches after the path-policy cleanup
- `git diff --check` -> passed
- `cargo fmt --all --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (NLP alias learning rejects list markers)

### Fixed: Form 2 alias learning rejects broader markdown/list prefixes
- `extract_alias_phrase()` now rejects alias subjects beginning with common markdown bullets, block quotes, and ordered-list markers such as `*`, `+`, `>`, `1.`, and `2)` in addition to the existing `-`, `|`, and `#` guards.
- This prevents `specforge nlp-enrich` from learning garbage aliases from formatted list/table text while leaving ordinary prose aliases such as `address bus` unchanged.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib extract_alias_phrase_rejects_markdown_marker_prefixes -- --nocapture` -> passed
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (VLM timing filters bracketed sample labels)

### Fixed: bracketed waveform sample labels stay out of timing constraints
- `SemanticIR` now treats bracketed VLM timing annotation labels such as `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` as low-value waveform/sample markup when they appear as standalone annotations.
- Strengthened `vlm_timing_spurious_annotation_negative` and the direct semantic regression again so bracketed bus/sample labels do not become `TimingConstraintRecord`s while grounded signal samples still produce typed temporal evidence.
- The tracked KG-quality fixture count remains `55`; this slice deepens the existing spurious-annotation negative fixture.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (VLM timing filters compact sample labels)

### Fixed: compact waveform sample labels stay out of timing constraints
- `SemanticIR` now treats compact VLM timing annotation labels such as `D0`, `A1`, `DATA0`, and `0xAA` as low-value waveform/sample markup when they appear as standalone annotations.
- Strengthened the existing `vlm_timing_spurious_annotation_negative` KG fixture and the direct semantic regression so those labels do not become `TimingConstraintRecord`s while real signal samples still produce typed temporal evidence.
- The tracked KG-quality fixture count remains `55`; this slice deepens an existing negative fixture rather than adding a duplicate case.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (Validation flags missing FSM initial states)

### Changed: missing initial states are covered like multiple initial states
- Added `vlm_state_machine_missing_initial_negative`, proving a VLM state-machine extraction with `IDLE` and `BUSY` but no initial marker keeps the state graph visible while validation flags the unsafe cardinality at both `SemanticIR` and `IntentIR`.
- Added a direct validation regression for explicit state declarations with zero initial states, sharing the same staged IR build helper as the multiple-initial regression.
- The tracked KG-quality fixture count is now `55`.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib validate_semantic_and_intent_ir_flag_missing_initial_state -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_missing_initial_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, `305` Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build

## 2026-04-12 (Validation flags bad FSM initial cardinality)

### Changed: state machines with zero or multiple initial states are validation-visible
- `specforge validate` now emits `semantic_state_machine_initial_cardinality` and `intent_state_machine_initial_cardinality` warnings when a canonical state graph exists but does not have exactly one initial state.
- Added `vlm_state_machine_multiple_initial_negative`, proving a VLM state-machine extraction with both `IDLE` and `BUSY` marked initial keeps the graph visible while validation flags the unsafe initial-state cardinality at both `SemanticIR` and `IntentIR`.
- Added a direct validation regression for explicit state declarations with two initial states, so the warning is not only covered through the KG fixture path.

### Validation
- `cargo test --manifest-path Cargo.toml --lib validate_semantic_and_intent_ir_flag_multiple_initial_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_multiple_initial_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 304 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-12 (Validation counts initial FSM states)

### Changed: validation makes initial-state cardinality visible
- `specforge validate` now reports `initial_regular_states` for both `SemanticIR` and `IntentIR`, next to the existing `regular_states` and `state_transitions` metrics.
- The `vlm_state_machine_duplicate_initial_gold` fixture now asserts that duplicate VLM state labels still leave exactly one canonical initial state after merge, not merely that `IDLE` appears in the initial-state name set.
- The mdBook KG-bench chapter documents this stronger validation surface for VLM state-machine truthfulness.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib validate_semantic_ir_artifact_reports_without_error -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_duplicate_initial_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 303 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-12 (VLM state machine merges duplicate initial markers)

### Fixed: duplicate VLM state labels preserve initial-state evidence
- `SemanticIR` now merges duplicate VLM state-machine state labels by state name before they enter canonical `RegularStateRecord`s, preserving an `is_initial` marker if any duplicate carries it.
- This mirrors the explicit state-declaration parser and prevents a VLM output like `IDLE` non-initial followed by duplicate `IDLE` initial from losing the true initial-state marker.
- `kg-bench` can now assert initial-state names directly, and `vlm_state_machine_duplicate_initial_gold` locks that `IDLE` stays the single initial state while `BUSY` stays non-initial.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_state_machine_observation_merges_duplicate_state_initial_markers -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_duplicate_initial_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 303 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM state machine gates transition endpoints)

### Fixed: VLM transitions must target declared states
- `SemanticIR` now accepts VLM state-machine transitions only when both `from` and `to` endpoints refer to state labels accepted from the same `vlm_state_machine_extraction` observation.
- This keeps identifier-shaped but undeclared VLM endpoints such as `DONE` or `RESET` from becoming canonical transition graph facts just because they look like plausible FSM state names.
- Added `vlm_state_machine_undeclared_transition_negative`, proving `IDLE` / `BUSY` states and `IDLE->BUSY` survive while `BUSY->DONE` and `RESET->IDLE` are filtered.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_state_machine_observation_rejects_undeclared_transition_endpoints -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_undeclared_transition_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 302 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM state machine rejects prose labels)

### Fixed: state-machine VLM labels must be canonical identifiers
- `SemanticIR` now parses VLM state-machine `states[].name` and transition `from` / `to` endpoints through the same identifier boundary used by explicit state syntax, so prose labels such as `IDLE state` and `ACCESS phase` cannot become canonical FSM states or transition endpoints.
- `kg-bench` can now assert canonical state names and state-transition endpoints directly, which lets tracked fixtures lock VLM FSM truthfulness without inspecting generated artifacts by hand.
- Added `vlm_state_machine_label_noise_negative`, proving valid `IDLE` / `BUSY` state evidence and `IDLE->BUSY` transition evidence survive while prose-like VLM labels are filtered.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml --lib vlm_state_machine_observation_rejects_non_identifier_state_labels -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_state_machine_label_noise_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 301 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM timing rejects compact edge spellings)

### Fixed: compact edge labels stay out of signal values
- `SemanticIR` now rejects additional VLM timing waveform-motion spellings such as `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` before the symbolic-value fallback can promote them into false signal-value facts.
- Strengthened the existing semantic regression and `vlm_timing_waveform_motion_negative` fixture so the only surviving VLM-authored signal constraint remains the concrete `HIGH` sample.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_waveform_motion_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 300 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM timing normalizes motion spellings)

### Fixed: separator variants of waveform transitions stay non-factual
- `SemanticIR` now normalizes VLM timing motion-state spellings across spaces, underscores, and hyphens before filtering them, so values like `RISING_EDGE`, `LOW_TO_HIGH`, and `HIGH_TO_LOW` cannot slip through as symbolic `MustBeValue` facts.
- Strengthened the existing semantic regression and `vlm_timing_waveform_motion_negative` fixture so they cover both plain motion labels and identifier-shaped transition labels while still preserving the concrete `HIGH` sample.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_waveform_motion_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 300 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (VLM timing rejects waveform motion as values)

### Fixed: waveform motion labels no longer become fake signal values
- `SemanticIR` VLM timing lift now rejects motion-only `signals[].values[].state` labels such as `rising`, `falling`, `stable`, and `UNCHANGED` instead of promoting them as symbolic `MustBeValue` temporal facts.
- Concrete sampled values such as `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` still survive through the existing bounded VLM signal-value path.
- Added semantic regression coverage proving a mixed waveform sequence keeps only the concrete `XREQ == HIGH @ T1` sample.
- Added tracked KG-quality fixture `vlm_timing_waveform_motion_negative`, which keeps the VLM timing extraction visible while requiring exactly one signal constraint / temporal rule and zero semantic-role hints from motion labels.

### Validation
- `cargo test --manifest-path Cargo.toml --lib vlm_timing_diagram_observation_rejects_waveform_motion_states -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_waveform_motion_negative` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 300 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (SemanticIR keeps interface conflicts sticky)

### Changed: canonical interface shape conflicts cannot self-heal by repetition
- `SemanticIR` interface-signal accumulation now distinguishes unknown direction/width hints from conflict-collapsed hints, so a later duplicate declaration cannot resurrect a canonical direction or width after disagreement.
- Strengthened the existing interface-conflict regression with `Signal DATA is input width 8`, `Signal DATA is output width 16`, and then `Signal DATA is input width 8` again; `DATA.direction_hint` and `DATA.width_hint` must remain unresolved while the explicit direction and width conflict records stay visible.

### Validation
- `cargo test --manifest-path Cargo.toml --lib surfaces_interface_signal_conflicts_for_conflicting_explicit_declarations -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib ir::semantic::tests -- --nocapture` -> passed with 65 semantic tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 299 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter locks sticky actor-port width conflicts)

### Added: width regression for sticky adapter conflicts
- Added a standalone direct `.fsm` regression proving actor-port width disagreement stays unresolved after conflict collapse: a flat `DATA_OUT` width `8`, graph-backed `DATA_OUT` width `16`, and later duplicate graph-backed width `8` must still block lowering instead of self-healing.
- This completes regression coverage for the sticky adapter inventory conflict path across both role direction and numeric width hints.

### Validation
- `cargo test --manifest-path Cargo.toml --lib standalone_dt_keeps_conflicting_actor_port_width_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib ir::adapters::tests -- --nocapture` -> passed with 26 adapter tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 299 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter keeps actor-port direction conflicts sticky)

### Changed: adapter inventory conflicts cannot self-heal by repetition
- `FsmSignalCandidate` inventory evidence now distinguishes unknown direction/width hints from already-conflicted hints, so a later duplicate actor-port or interface hint cannot resurrect a value after disagreement collapsed it to unresolved.
- Added a standalone direct `.fsm` regression where one unambiguous `controller` actor repeats `DATA_OUT` as `output`, then `input`, then `output`; the adapter must keep `DATA_OUT` unresolved and block lowering instead of treating the final duplicate as truth.

### Validation
- `cargo test --manifest-path Cargo.toml --lib standalone_dt_keeps_conflicting_actor_port_direction_unresolved -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml --lib ir::adapters::tests -- --nocapture` -> passed with 25 adapter tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 298 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter locks graph-backed sequential system directions)

### Added: graph-backed sequential system-contract regression
- Added a focused `.fsm` adapter regression proving standalone sequential DT lowering remains renderable when flat top-level `direction_hint` values are cleared, as long as one unambiguous `IntentIR.actor_ports` context supplies graph-backed directions for `clk`, `rst_n`, `DATA_IN`, and `ACC`.
- The test locks the system-contract path specifically: graph-backed `clk` / `rst_n` inputs must satisfy `(+system ...)` renderability and avoid the `fsm_adapter_system_contract` residual.

### Validation
- `cargo test --manifest-path Cargo.toml standalone_sequential_dt_recovers_system_directions_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 24 adapter tests
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 297 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (KG bench locks detached mixed polarity rejection)

### Added: tracked negative fixture for detached mixed polarity
- Added [detached_mixed_control_polarity_negative](crates/specforge/test_data/kg_quality/detached_mixed_control_polarity_negative/fixture.json), which proves detached wording like `CS_N is active LOW and active HIGH` does not borrow an implicit subject or resolve polarity through `SemanticIR` / `IntentIR`.
- The fixture keeps `CS_N` declared and constrained, but requires `with_resolved_polarity: 0`, zero heuristic signal records, zero polarity conflicts, and zero temporal conflicts.

### Validation
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench detached_mixed_control_polarity_negative` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 49 tracked KG fixtures
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 296 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (EvidenceIR recovers mixed clause-local control polarity)

### Changed: safe clause-local polarity parsing
- Added a bounded clause-local fallback for mixed active-level prose, so a statement like `CS_N is active LOW and ENABLE is active HIGH` can recover active-low polarity for `CS_N` and active-high polarity for `ENABLE`.
- Kept the fallback strict: every recovered clause must bind one polarity phrase to exactly one known signal, every mentioned signal in the statement must be recovered, and detached wording such as `CS_N is active LOW and active HIGH` stays unresolved instead of inheriting an implicit subject.
- The whole-statement detector still returns no polarity for mixed low/high text; the new recovery path is separate and only promotes facts after clause-local validation succeeds.

### Added: tracked KG fixture for mixed control polarity
- Added [mixed_control_polarity_gold](crates/specforge/test_data/kg_quality/mixed_control_polarity_gold/fixture.json), which proves mixed clause-local polarity recovery produces two canonical declared signal records, two resolved polarities, zero heuristic duplicates, and zero polarity or temporal conflicts through `SemanticIR` and `IntentIR`.

### Validation
- `cargo test --manifest-path Cargo.toml mixed_polarity_prose_recovers_clause_local_control_polarities -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml detached_mixed_polarity_prose_does_not_guess_implicit_control_polarity -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml signal_polarity_detector_accepts_asserted_when_level_phrases -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench mixed_control_polarity_gold` -> passed
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` -> passed with 25 focused polarity tests
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 48 tracked KG fixtures
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 296 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (EvidenceIR recovers collective control polarity)

### Changed: collective active-level prose can ground multiple controls
- Broadened `EvidenceIR` prose polarity recovery so a statement like `CS_N and WE_N are active LOW signals` produces active-low polarity observations for both declared controls.
- Kept the path conservative: mixed compound prose such as `CS_N is active LOW and ENABLE is active HIGH` stays unresolved until the extractor can parse each clause safely, and polarity still is not inferred from `_N` / `_B` suffixes alone.
- Tightened `SemanticIR` interface construction so polarity-only co-mentions of already declared signals enrich the authoritative signal records instead of minting duplicate low-confidence heuristic interface records.

### Added: tracked KG fixture for collective control polarity
- Added [multi_control_polarity_gold](crates/specforge/test_data/kg_quality/multi_control_polarity_gold/fixture.json), which proves collective active-low prose recovers two resolved polarities, refines asserted/deasserted constraints correctly, and preserves zero polarity or temporal conflicts through `SemanticIR` and `IntentIR`.

### Validation
- `cargo test --manifest-path Cargo.toml collective_active_low_prose_recovers_multiple_control_polarities -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml mixed_polarity_prose_does_not_guess_collective_control_polarity -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml collective_polarity_prose_does_not_duplicate_interface_records -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -p specforge -- kg-bench multi_control_polarity_gold` -> passed
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` -> passed with 23 focused polarity tests
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 47 tracked KG fixtures
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 295 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (EvidenceIR recovers asserted-when-level control polarity)

### Changed: explicit polarity prose covers more control-signal wording
- Broadened `EvidenceIR` signal-polarity detection to treat local phrases like `asserted when LOW`, `LOW when asserted`, `asserted by driving LOW`, and `driven LOW to assert` as active-low evidence.
- Added the symmetric active-high phrase forms for `HIGH`.
- This remains evidence-grounded polarity recovery: it does not infer polarity from a `_N` suffix alone and it applies only when the current document explicitly says how assertion maps to a logic level.
- Tightened `SemanticIR` interface construction so a one-signal local polarity/control sentence enriches an already declared signal instead of minting a duplicate low-confidence heuristic interface record.

### Tests
- Added a detector regression for asserted-when-level wording.
- Added a non-reset `CS_N` control-signal regression proving explicit `CS_N is asserted when LOW` recovers active-low polarity and refines `CS_N must be asserted` / `CS_N must be deasserted` into LOW / HIGH constraints.
- Added a `kg-bench` fixture locking the non-reset control polarity path with no polarity or temporal conflicts.

### Validation
- `cargo test --manifest-path Cargo.toml polarity -- --nocapture` -> passed with 21 focused polarity tests
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with 46 tracked KG fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 292 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (KG bench can assert graph-backed direction coverage)

### Added: graph-native direction expectation surface
- Added `graph_direction_signal_names_include` / `graph_direction_signal_names_exclude` to `specforge kg-bench` canonical stage expectations.
- The new expectation checks `actor_ports` directly for non-`unknown` graph direction coverage by signal name, so fixtures can lock the graph-native surface without overloading flat `signal_directions_include`.
- Existing flat direction expectations are unchanged and still check `InterfaceSignalRecord.direction_hint` only.

### Tests
- Strengthened `actor_ports_gold` so both `SemanticIR` and `IntentIR` assert that `PREADY` has graph-backed direction coverage and `PSEL` does not.

### Validation
- `cargo fmt --all --check` -> passed
- `git diff --check` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` -> passed with all 45 tracked KG fixtures
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 288 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (FSM adapter recovers width-only top port directions from links)

### Changed: top-boundary lowering can use explicit link topology
- `ExplicitTopPortRecord.direction_hint` is now optional, so `SemanticIR` / `IntentIR` can preserve width-only top boundary port records instead of dropping them before the adapter sees the composition.
- The `.fsm` top-composition adapter now recovers missing top boundary port directions from explicit top-link position: a top endpoint used as a link source is a top input, and a top endpoint used as a link target is a top output.
- Recovery remains deterministic and bounded: unresolved top ports still block, conflicting explicit direction versus link topology still blocks, and the adapter only uses this for explicit `?top:name` composition topology rather than inventing actor-relative roles.

### Tests
- Added a regression proving `Top datapath port result_data is width 8` survives into canonical top composition and renders as `result_data>8` only because the explicit link `consumer.result_data -> result_data` establishes it as a top output.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed with 5 focused tests
- `cargo test --manifest-path Cargo.toml extracts_explicit_modules_and_tops_from_markdown -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 23 adapter tests
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 288 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-11 (FSM adapter consumes graph-backed direct directions)

### Changed: standalone direct lowering uses bounded actor-relative port evidence
- Updated the standalone direct `.fsm` adapter inventory path so it can recover missing local signal directions from `IntentIR.actor_ports` when the actor-port graph relevant to the direct local signal inventory has exactly one actor context.
- Kept the direct-root rule stricter than explicit module lowering: direct roots have no module name to identify the target actor, so mixed producer/consumer graph contexts remain blocked instead of guessing a perspective.
- The direct-root overlay only strengthens signals already present in the local direct inventory; it does not add graph-only signals for standalone lowering.
- Tightened actor-port provenance merging so a conflicting graph direction with multiple supporting ids cannot accidentally restore a resolved direction after the merge collapsed the conflict to unresolved.

### Tests
- Added a regression proving standalone direct `.fsm` lowering still renders when flat `direction_hint` values are cleared but one `controller` actor supplies `DATA_IN`, `DATA_OUT`, and `ZERO_FLAG` actor-port directions.
- Added a regression proving unrelated graph-only actor ports are ignored for the direct-root context gate and are not added to the standalone signal inventory.
- Added a regression proving mixed `producer` / `consumer` actor-port context is ambiguous for a standalone direct root and remains blocked with missing canonical direction hints.
- Strengthened the top-composition graph-conflict regression with duplicate supporting provenance ids to lock the conservative conflict merge behavior.

### Validation
- `cargo fmt --all --check` -> passed
- `cargo test --manifest-path Cargo.toml standalone_dt -- --nocapture` -> passed with 4 focused tests
- `cargo test --manifest-path Cargo.toml top_composition_blocks_conflicting_actor_port_directions -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 22 adapter tests
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 287 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-11 (FSM adapter consumes graph-backed module directions)

### Changed: explicit module lowering uses actor-relative port evidence
- Updated the `.fsm` adapter so explicit module candidates overlay matching `IntentIR.actor_ports` before renderability analysis.
- If an explicit module signal has width/provenance but its flat `direction_hint` is missing, the adapter can now recover that module-local input/output role from actor-relative graph evidence for the matching module actor.
- Conflicting or non-renderable graph directions still stay conservative: `in_out` and `unknown` do not become fake `.fsm` input/output hints, and normal hint merging still collapses contradictions to `None`.
- Added a regression that clears all flat child-module directions in an explicit top composition and proves the composition still lowers when `actor_ports` provide `producer_core.output_data` as output, `consumer_core.input_data` as input, and `consumer_core.result_data` as output.
- Added a companion regression proving a conflicting graph direction for `producer_core.output_data` blocks top lowering instead of silently overriding the flat module-local direction.

### Validation
- `cargo test --manifest-path Cargo.toml top_composition -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml top_composition_recovers_child_directions_from_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml ir::adapters::tests -- --nocapture` -> passed with 19 adapter tests
- `bash scripts/run_docs_ci.sh` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 284 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build
- `git diff --check` -> passed

## 2026-04-11 (README bootstrap handoff hygiene)

### Fixed: compatibility guide root-doc list
- Re-ran the README -> `SESSION_BOOTSTRAP.md` handoff path and checked the high-signal live docs plus current Rust CLI/module/KG-fixture surface.
- Removed stray datapath/reset example bullets from `USER_GUIDE.md`'s root-document list so the compatibility pointer no longer mixes old extraction examples into the continuity-doc inventory.
- Refreshed continuity state so `MEMORY.md` records the latest committed baseline `35a8372 test(kg): lock active-low vlm reset release`.
- Updated the bootstrap analysis note with the current Rust source-file sanity count: 28 files and about 53,858 lines under `crates/specforge/src`.

### Validation
- `cargo run -p specforge -- --help` -> passed
- `bash scripts/run_docs_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-11 (Active-low VLM reset release polarity is fixture-locked)

### Added: reset-release timing-annotation KG fixture
- Added tracked KG-quality fixture `vlm_timing_active_low_deassertion_equivalence_gold`.
- The fixture proves that a VLM timing diagram reporting active-low `ARESETN` as both `deasserted` and `HIGH` produces typed temporal evidence without a false temporal or polarity conflict.
- This complements `vlm_timing_active_low_assertion_equivalence_gold`, so the benchmark surface now locks both active-low reset entry (`ASSERTED` == `LOW`) and reset release (`DEASSERTED` == `HIGH`) behavior.

### Validation
- `cargo run -p specforge -- kg-bench vlm_timing_active_low_deassertion_equivalence_gold` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 282 Rust tests under `RUSTFLAGS="-D warnings"` including the tracked 45-fixture KG benchmark test, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (Rustdoc warning gate joins CI)

### Changed: Rust API docs are warning-denied
- Fixed a broken rustdoc intra-doc-link interpretation in the `ActorSignalRelation` documentation by formatting the derived `output_of(A)` / `input_of(others)` wording as code/prose instead of bracket syntax.
- Updated `scripts/run_ci.sh` so local CI runs `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` before the mdBook build.
- The new gate preserves caller-provided `RUSTDOCFLAGS`, matching the existing Rust warning flag composition for tests.

### Validation
- `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 282 Rust tests under `RUSTFLAGS="-D warnings"`, rustdoc under `RUSTDOCFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (Clippy warning gate joins CI)

### Changed: Clippy is now part of the shared quality gate
- Fixed mechanical Clippy findings across enrichment, prior learning, rescan planning, validation, adapter, evidence, semantic, intent, and Docling helper code.
- Added localized `#[expect(...)]` attributes with reasons for intentional broad IR builder/evaluator signatures and coupled semantic/evidence return surfaces instead of globally allowing those lints.
- Updated `scripts/run_ci.sh` so local CI runs `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` before the Rust test suite.
- Updated GitHub Actions to install the `clippy` component, so hosted CI runs the same Clippy gate via the shared script.

### Validation
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` -> passed
- `bash scripts/run_ci.sh` -> passed with Clippy `-D warnings`, 282 Rust tests under `RUSTFLAGS="-D warnings"`, and mdBook build

## 2026-04-11 (CI now denies Rust warnings)

### Changed: warning-clean baseline is enforced
- Updated `scripts/run_ci.sh` so the Rust test step runs with `RUSTFLAGS="-D warnings"`.
- Because GitHub Actions calls the same script, the warning gate now applies both locally and on hosted CI without duplicating workflow logic.
- Updated the README, mdBook getting-started page, Rust codebase analysis, memory, development notes, and live status to document the warning-deny CI contract.

### Validation
- `bash scripts/run_ci.sh` -> passed with 282 Rust tests under `RUSTFLAGS="-D warnings"` plus mdBook build

## 2026-04-11 (Rust dead-code warning baseline is clean)

### Fixed: stale warning-only code paths
- Removed the orphaned `.fsm` adapter renderability helper path that still operated on legacy `DecisionTreeFragmentRecord` actions after the active lowering path moved to `ControlBlockRecord` branches.
- Removed the unused `render_action` helper; active rendering now goes through `render_control_action`.
- Removed empty semantic-stage register/timing builder stubs that no longer had call sites because `SemanticIR` carries register and timing records directly from `EvidenceIR` plus VLM timing extraction.

### Validation
- `cargo test --manifest-path Cargo.toml --lib` -> passed with 282 tests and no dead-code warning output
- `bash scripts/run_ci.sh` -> passed with 282 tests, mdBook build, and no dead-code warning output

## 2026-04-11 (README bootstrap refresh updates Rust analysis)

### Updated: bootstrap and codebase analysis
- Executed the README handoff path by reading `SESSION_BOOTSTRAP.md` and the high-signal referenced live docs.
- Refreshed `RUST_CODEBASE_ANALYSIS.md` so it reflects the current CLI surface, including `project-validation`, `rescan-plan`, `kg-bench`, `learn-priors`, and `nlp-enrich`.
- Updated the analysis to capture schema-v2 rescan execution, the no-canonical-mutation promotion-review boundary, `CorpusMemory` schema v5 prior families, the 45-fixture KG benchmark surface, and the latest observed 282-test Rust baseline.
- Updated continuity memory so the latest committed baseline is `4cfb825 test(kg): lock active-low vlm timing polarity`.

### Validation
- `bash scripts/run_ci.sh` -> passed

## 2026-04-11 (Active-low VLM timing polarity is fixture-locked)

### Added: timing-annotation KG fixture
- Added tracked KG-quality fixture `vlm_timing_active_low_assertion_equivalence_gold`.
- The fixture proves that a VLM timing diagram that reports the same active-low reset as `asserted` and `LOW` produces typed timing-derived signal constraints and temporal rules without creating a false temporal conflict.
- The fixture also asserts that resolved reset polarity remains visible and no signal-polarity conflict is reported.

### Validation
- `cargo run -p specforge -- kg-bench vlm_timing_active_low_assertion_equivalence_gold` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-11 (Rescan approval artifact boundary is explicit)

### Documented: review metadata is not approval evidence
- Documented that `promotion_review` is a review-requirement descriptor, not an approval record and not permission to mutate canonical IR.
- Decided that future approval artifacts stay local/generated by default while no canonical IR mutation workflow exists.
- Tracked approval evidence should only be designed together with an explicit canonical mutation path, including current-document evidence links, validation-delta review, mutation scope approval, prior-memory non-authority, and replayable provenance.

### Validation
- Documentation-only change; no runtime behavior changed.

## 2026-04-11 (Negative-knowledge benchmark coverage now spans connectivity and interface conflicts)

### Added: prior-guided caution fixtures
- Added tracked KG-quality fixture `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` for `signal_connectivity_conflict:multiple_producers`.
- Added tracked KG-quality fixture `negative_knowledge_prior_guided_interface_conflict_caution_gold` for `interface_signal_conflict:direction_mismatch` and `interface_signal_conflict:width_mismatch`.
- Both fixtures assert that the current local conflict remains present in `SemanticIR` and `IntentIR`; prior memory only adds `negative_knowledge_prior_matches`, rescan recommendations, corroboration requirements, and stage-specific rescan-guidance findings.

### Documentation
- Updated the README, roadmap, memory, development notes, and mdBook quality chapters so the public and continuity docs say the negative-knowledge benchmark suite now covers carried interface and connectivity conflict families too.

### Validation
- `cargo run -p specforge -- kg-bench negative_knowledge_prior_guided_connectivity_conflict_caution_gold negative_knowledge_prior_guided_interface_conflict_caution_gold` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Explicit clock/reset topology hints are typed)

### Added: bounded infrastructure topology records
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so `InfrastructureSignalRecord` now carries `infrastructure_topology` records for explicit current-document clock/reset topology hints.
- The first topology kinds are `clock_gated_branch`, `reset_synchronizer_stages`, and `reset_tree_targets`.
- The extractor records component names, reset synchronizer stage counts, target actor names, supporting statement IDs, and automation confidence when the source text is explicit enough.
- Vague wording such as a reset that "may use a synchronizer" remains ignored; the new surface preserves evidence but does not claim full physical clock-tree/reset-tree proof.

### Added: validation visibility
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `SemanticIR` and `IntentIR` validation report `infrastructure_topology_records`, `infrastructure_clock_gated_branches`, `infrastructure_reset_synchronizer_stages`, and `infrastructure_reset_tree_targets`.
- Validation console output now shows per-signal topology counts and record summaries under `Infrastructure Signals`.
- Updated the mdBook clock/reset, SemanticIR, and IntentIR chapters plus live continuity docs to document the new boundary.

### Validation
- `cargo test --manifest-path Cargo.toml explicit_clock_reset_topology_recovers_only_current_document_evidence -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_explicit_infrastructure_topology -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Rescan promotion review path is explicit)

### Added: promotion-review policy record
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `ProjectRescanExecutionSummary` now carries a structured `promotion_review` record beside `promotion_status` and `promotion_blockers`.
- The review record captures `review_status`, `approval_policy`, `required_decisions`, `approval_record_required`, and `canonical_mutation_allowed`.
- Changed rescan outcomes now require `human_review_required`, an approval record, current-document evidence support, validation-delta review, explicit canonical mutation scope approval, and a check that prior memory was not used as truth authority.
- No-change outcomes are marked `not_reviewable_no_change`, and every path keeps `canonical_mutation_allowed: false`.

### Preserved: no canonical mutation path yet
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so executed recommendations persist the new review record.
- Legacy execution summaries without `promotion_review` still deserialize and are normalized by `project-validation`.
- The validation snapshot and live-status projection now show the review status in addition to the not-promoted gate.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation_normalizes_legacy_rescan_execution_summary_gate -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_collects_negative_knowledge_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_execute_marks_validated_no_change -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_arbitration_verdict_tracks_validation_direction -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Visual-motif rescans now replay local enrichment)

### Added: explicit visual corroboration replay command
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `evidence_visual_motif_corroboration_guidance` recommendations now emit `enrich_source_ir` before the downstream `rebuild_evidence_ir` and `validate_current_artifact` hints.
- Added `project-validation --rescan-vlm-provider auto-local|ollama|lmstudio|skip` and optional `--rescan-vlm-model <model>` so generated visual-motif enrichment hints can prefer ready local Ollama, fall back to ready local LM Studio, or be forced by policy instead of always emitting Ollama.
- Updated [doctor.rs](crates/specforge/src/commands/doctor.rs) with a reusable local default-model presence helper and bounded curl timeouts for readiness probes.
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so `rescan-plan --execute` can parse and run whitelisted local `enrich` hints in-process.
- The enrich replay parser accepts local Ollama, local LM Studio, or `skip`, supports an optional model and `--classify-only`, and intentionally rejects OpenAI replay hints so generated rescans stay local-first.
- Updated [cli.rs](crates/specforge/src/cli.rs) so VLM provider args can participate in typed rescan invocation equality tests.

### Documentation
- Updated the README, live notes, roadmap, memory, and mdBook validation/quality/multimodal/corpus-memory chapters so the public contract is clear: visual-motif prior memory may route a local enrichment rescan, but it still does not promote truth.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation_collects_visual_motif_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_rescan_vlm_policy_can_emit_lmstudio_hint -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_auto_rescan_vlm_policy_prefers_ready_lmstudio_when_ollama_absent -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_defaults_to_auto_local_rescan_vlm_provider -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_parses_whitelisted_local_enrich_hint -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan_rejects_openai_enrich_hints -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml doctor -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Visual-motif priors now emit corroboration targets)

### Added: validation-visible visual corroboration targets
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `EvidenceIR` validation now reports role-level visual evidence metrics: `visual_evidence_normative`, `visual_evidence_explanatory`, `visual_evidence_illustrative`, `visual_evidence_ambiguous`, and `visual_evidence_unknown`.
- Prior-classified normative visual evidence now also reports `visual_motif_corroboration_targets` and emits `evidence_visual_motif_corroboration_guidance` when it still lacks VLM timing/state extraction observations.
- The guidance is explicitly review/rescan routing only: it asks for targeted VLM/multimodal corroboration and still does not rewrite `SourceIR`, synthesize semantic facts, or promote canonical truth from prior memory.

### Added: gold/negative benchmark pair
- Strengthened [visual_motif_prior_guided_diagram_classification_gold](crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json) so it now proves the prior-backed classification produces one normative visual evidence item, one corroboration target, and zero semantic hints.
- Added [visual_motif_prior_guided_diagram_classification_without_prior_negative](crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_without_prior_negative/fixture.json), proving the same local `XREQ cycle trace` visual stays ambiguous and unclassified without staged visual-motif memory.
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so generic `rescan_guidance` findings with related ids can enter the schema-v2 rescan target list, including the new visual-motif corroboration finding.

### Validation
- `cargo test --manifest-path Cargo.toml visual_motif -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_collects_visual_motif_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_evidence_ir -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_motif_prior_guided_diagram_classification_gold` -> passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_motif_prior_guided_diagram_classification_without_prior_negative` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan execution summaries now gate promotion explicitly)

### Added: machine-readable not-promoted gate
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `ProjectRescanExecutionSummary` carries `promotion_status` and `promotion_blockers` in addition to automation status, arbitration verdict, and validation deltas.
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so executed recommendations now write `not_promoted_no_change` for unchanged validation snapshots and `not_promoted_review_required` for possible-improvement, regression, or neutral artifact-drift verdicts.
- The blockers make the policy explicit in generated schema-v2 plans: rescans do not mutate canonical IR, validation deltas are not truth promotion, and changed outcomes still require current-document evidence review.

### Preserved: backward-compatible local plans
- Older local execution summaries that do not yet contain promotion fields, or that contain stale nonmatching promotion text, deserialize safely and are normalized when `project-validation` preserves matching execution state.
- `VALIDATION_SNAPSHOT.md` and the managed live-status validation block now project the promotion gate beside verdict and delta details, so future review does not need to inspect raw JSON to see that a favorable delta remains unpromoted.

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Project validation projects rescan execution summaries)

### Added: review-facing rescan execution projection
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `project-validation` preserves matching executed rescan summaries from the existing local schema-v2 plan before refreshing `generated/validation/rescan_plan.json`.
- `VALIDATION_SNAPSHOT.md` now projects rescan execution-summary counts and per-recommendation verdict/delta details when they exist.
- The managed live-status validation block now includes the same review-required counts and inline verdict/delta summary for queued recommendations.

### Preserved: refreshes do not erase review state
- Matching is keyed on document, stage, artifact path, finding id, extractor lane, and related ids.
- The refresh carries forward `automation_status` plus `execution_summary` only for matching recommendations, so stale unrelated execution state does not leak into fresh rescan targets.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan execution persists arbitration summaries)

### Added: validation-backed execution summaries
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so schema-v2 rescan recommendations can carry an optional `execution_summary` after execution.
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so executed recommendations persist before/after validation snapshots, score deltas, finding-count deltas, added/removed finding ids, and an arbitration verdict.
- Verdicts are deliberately conservative: `validated_no_change`, `possible_improvement_review_required`, `regression_review_required`, or `neutral_change_review_required`.

### Preserved: review before promotion
- Updated [converge.rs](crates/specforge/src/commands/converge.rs) so convergence summaries count review-required verdicts and split them across possible-improvement, regression, and neutral artifact-change buckets.
- A possible improvement is still not a canonical truth promotion; it only means validation deltas moved in a favorable direction and need current-document evidence review.

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml converge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_collects_negative_knowledge_rescan_guidance -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml project_validation_writes_snapshot_doc_and_updates_live_status -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Converge can consume rescan plans after stability)

### Added: opt-in rescan hook for the fixed-point loop
- Updated [converge.rs](crates/specforge/src/commands/converge.rs) so `specforge converge <source> --rescan-plan <plan>` consumes a schema-v2 validation rescan plan after the persisted pipeline snapshot stabilizes.
- Added `--execute-rescan-plan` and `--rescan-plan-limit` to keep execution explicit and bounded.
- Added a `--document-key` filter to standalone `rescan-plan`; the `converge` hook automatically filters multi-document plans to the current source document key.
- `--execute-rescan-plan` without `--rescan-plan <plan>` is rejected.

### Preserved: convergence is not auto-promotion
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) to expose a structured run report reused by `converge`.
- The convergence summary now reports selected recommendations, changed/no-change validation outcomes, post-rescan snapshot drift, and an arbitration status.
- The convergence result remains the stable pre-rescan snapshot; `changed_requires_validation_review` is a review signal, not an improvement claim.

### Validation
- `cargo test --manifest-path Cargo.toml converge_defaults_to_ollama_for_vlm_and_nlp -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml converge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan execution records validation deltas)

### Added: before/after validation accounting
- Updated [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) so `specforge rescan-plan --execute` validates the recommendation artifact before and after whitelisted command execution.
- The local generated plan now records neutral execution outcome statuses:
  - `executed_validated_no_change`
  - `executed_validated_changed`
- Added an execution-path regression that builds a real temporary `SourceIR`, executes a validate-only rescan recommendation, and verifies the plan is marked `executed_validated_no_change`.

### Preserved: changed does not mean improved
- The executor still does not classify a rebuild as improved.
- It only records whether the validation fingerprint, score, grade, or finding count changed.
- The deeper remaining step is promotion-grade arbitration over changed outcomes before treating rebuilt artifacts as better canonical truth.

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan plan now has a bounded executor)

### Added: dry-run-first rescan-plan command
- Added [rescan_plan.rs](crates/specforge/src/commands/rescan_plan.rs) with the new `specforge rescan-plan` command.
- The command reads schema-v2 `generated/validation/rescan_plan.json`, reports pending `planned_not_executed` targets by default, and supports `--limit` plus `--plan`.
- Added `--execute` to dispatch only whitelisted in-process `ingest`, `evidence`, `semantic`, `intent`, and `validate` hints from the structured command args.
- Successful `--execute` runs update local recommendation status in the generated plan.

### Preserved: execution is not truth promotion
- The executor refuses non-`cargo` executables, non-repository working directories, malformed cargo prefixes, and unsupported command intents.
- It does not shell out through command display strings.
- It does not let prior memory decide canonical facts; execution only rebuilds and validates stages, while changed outcomes still need promotion-grade evidence policy before they can count as improved.

### Documentation
- Updated the public mdBook command and validation pages with the `rescan-plan` behavior.
- Updated [README.md](README.md), [ROADMAP.md](ROADMAP.md), [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md), and [MEMORY.md](MEMORY.md).

### Validation
- `cargo test --manifest-path Cargo.toml rescan_plan -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- rescan-plan` -> passed (`rescan_queue: empty`)
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Rescan plan now carries replayable command hints)

### Added: schema-v2 replay metadata for targeted rescans
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `generated/validation/rescan_plan.json` now uses schema version 2.
- Each recommendation now carries typed `replay_inputs` such as `source_document`, `source_ir`, `evidence_ir`, or `semantic_ir`.
- Each recommendation now carries structured `recommended_commands` with executable, args, working directory, display string, and command intent.
- Recommendations now carry `automation_status: planned_not_executed` so future loops can distinguish planned targets from executed rescans.

### Preserved: replay hints are not auto-rescans
- `project-validation` still does not execute the command hints.
- It does not mutate IR beyond the existing validation backannotation behavior.
- It does not suppress findings, change scoring, decide arbitration, or promote facts from prior memory.
- The new metadata is what lets explicit rescan consumers execute safe args instead of scraping prose or shell strings.

### Documentation
- Updated the public mdBook validation and command pages to describe the replay-oriented plan contract.
- Updated [README.md](README.md), [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md), and [MEMORY.md](MEMORY.md) with the schema-v2 behavior and remaining executor follow-up.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` -> passed (`rescan_recommendations: 0`, schema-v2 local plan refreshed)
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Project validation now consumes rescan guidance)

### Added: generated rescan/extractor-selection plan
- Updated [project_validation.rs](crates/specforge/src/commands/project_validation.rs) so `specforge project-validation` now consumes `*_negative_knowledge_rescan_guidance` findings from validation reports.
- The command now writes a local generated `generated/validation/rescan_plan.json` plan with document key, stage, artifact path, finding id, related current-surface ids, extractor lane, corroboration policy, and recommended action.
- `VALIDATION_SNAPSHOT.md` now renders a `Targeted Rescan Recommendations` section from the same plan.
- The managed `LIVE_ACHIEVEMENT_STATUS.md` validation projection now includes a concise `Targeted rescan queue`.

### Preserved: the plan routes work, not truth
- The consumer does not mutate IR artifacts.
- It does not suppress findings.
- It does not change scoring, arbitration, or canonical promotion.
- It keeps prior memory advisory by routing extraction effort toward dangerous current-document conflict/residual shapes.

### Validation
- `cargo test --manifest-path Cargo.toml project_validation -- --nocapture` -> passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` -> passed (`rescan_recommendations: 0`)
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Negative-knowledge now routes rescan/corroboration guidance)

### Added: machine-readable guidance from known failure shapes
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so exact negative-knowledge prior matches now emit `negative_knowledge_rescan_recommendations` and `negative_knowledge_corroboration_requirements`.
- Validation now also emits stage-specific `*_negative_knowledge_rescan_guidance` findings for `EvidenceIR`, `SemanticIR`, and `IntentIR`, with the matched current conflict/residual ids preserved as `related_ids`.
- This gives future rescan/extractor-selection loops a deterministic routing hook instead of only a human-readable caution.

### Preserved: guidance is not correction
- The new guidance still requires a current-document conflict or residual before any prior can match.
- It does not mutate artifacts.
- It does not suppress conflict/residual findings.
- It does not change scoring, arbitration, or canonical promotion.
- It does not create canonical facts from prior memory.

### Added: unit, fixture, and docs coverage
- Extended negative-knowledge validation regressions so rescan/corroboration metrics and findings are locked for evidence, semantic, intent, temporal-conflict, and residual-decision matches.
- Updated KG-quality fixtures so prior-guided cases require the new guidance while the no-prior semantic-conflict fixture excludes it and expects zero guidance metrics.
- Updated the public mdBook corpus-memory, validation, and pipeline chapters so this routing hook is documented as end-user-visible behavior.

### Validation
- `cargo test --manifest-path Cargo.toml negative_knowledge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (Explicit infrastructure distribution recovery stays bounded)

### Added: locally grounded clock/reset distribution evidence
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so explicit current-document phrases such as `ACLK is distributed to the Requester and Completer` can recover `distributed_to_*` targets directly into `InfrastructureSignalRecord`.
- Added active fanout parsing for bounded phrases such as `reset synchronizer feeds ARESETN to the Requester`, allowing the same local sentence to recover a reset infrastructure source and a distribution target.
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) with a regression proving recovered distribution updates the existing infrastructure distribution metrics.

### Preserved: distribution is not ordinary protocol connectivity
- Distribution-only evidence does not create ordinary `ActorPortRecord`s.
- It does not fabricate source actors.
- It still rejects generic labels through the existing infrastructure component / meaningful actor filters.

### Validation
- `cargo test --manifest-path Cargo.toml explicit_clock_distribution_recovers_infrastructure_targets -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml explicit_reset_synchronizer_fanout_recovers_source_and_target -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_recovered_infrastructure_distribution -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_recovered_infrastructure_source -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Explicit infrastructure source recovery stays bounded)

### Added: locally grounded infrastructure source evidence
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so explicit current-document phrases such as `clock generator drives ACLK` can recover a source actor directly into `InfrastructureSignalRecord`.
- The parser accepts bounded infrastructure component terms such as `clock generator`, `reset controller`, `PLL`, `DLL`, `oscillator`, and synchronizer/gating-style component names without relaxing the ordinary protocol-actor filters.
- Existing graph-derived sources still contribute when the local KG already recovered a real source such as `PLL generates ACLK`.

### Preserved: infrastructure sources are not automatic consumers
- Tightened the system-contract fanout pass so implicit clock/reset read ports are added only to actors with non-infrastructure protocol relations.
- A recovered source actor for `ACLK` is no longer also marked as an `ACLK` consumer solely because a system contract exists.
- Generic labels such as `Clock`, `Reset`, `External`, `input`, and `output` remain blocked from becoming ordinary producer actors.

### Validation
- `cargo test --manifest-path Cargo.toml explicit_clock_generator_recovers_infrastructure_source_status -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml infrastructure_source_actor_is_not_marked_as_own_consumer -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_recovered_infrastructure_source -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Infrastructure source/distribution status is first-class)

### Added: canonical infrastructure signal status
- Added `InfrastructureSignalRecord` to [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SemanticIR` now records clock/reset infrastructure kind, unresolved vs recovered source status, recovered distribution status, supporting statements, and automation confidence.
- `SemanticIR` derives this surface from the local `SystemContractRecord` plus recovered `SignalConnectivityRecord`s, so `ACLK` / `ARESETN` can be marked as infrastructure while keeping unresolved source ownership explicit.
- Updated [intent.rs](crates/specforge/src/ir/intent.rs) so `IntentIR` carries the same `infrastructure_signals` surface forward as part of the canonical product artifact.

### Preserved: no fake clock/reset producer actors
- Unresolved infrastructure sourcing is now represented as `unresolved_source`.
- The model does not invent producer actors named `Clock`, `External`, or `input`.
- Recovered distribution status reports whether the signal reaches zero, one, or multiple recovered actors; it is not a physical clock-tree or reset-tree proof.

### Added: validation and docs
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) with `infrastructure_signals`, unresolved-source, recovered-source, and recovered-distribution metrics for `SemanticIR` and `IntentIR`.
- Updated the public mdBook clock/reset and pipeline chapters so the new surface is documented as project-facing behavior, not just a continuity note.

### Validation
- `cargo test --manifest-path Cargo.toml system_contract_emits_infrastructure_records_without_actor_ports -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml carries_infrastructure_signal_connectivity_class_into_intent_ir -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_treats_clock_and_reset_as_infrastructure_connectivity -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (Negative-knowledge cautions now reach carried semantic/intent surfaces)

### Added: deep-layer validation-only caution matching
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with shared normalized pattern builders for temporal value conflicts, interface-signal conflicts, signal-connectivity conflicts, and residual decision packets.
- Updated [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so negative-knowledge harvesting and validation consumption now use the same shared pattern builders for every harvested negative-knowledge kind.
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `SemanticIR` and `IntentIR` validation can recover the linked `EvidenceIR.prior_memory_path` and surface exact-match `negative_knowledge_prior_matches`.
- `SemanticIR` validation now emits `semantic_negative_knowledge_prior_matches` when a current carried conflict or residual packet class matches prior negative knowledge.
- `IntentIR` validation now emits `intent_negative_knowledge_prior_matches` for the same carried caution surface.

### Preserved: caution is not correction
- The new deep-layer consumer does not mutate `SemanticIR` or `IntentIR`.
- It does not suppress temporal, interface, connectivity, semantic-role, or residual findings.
- It does not change arbitration, scoring, or canonical promotion.
- It only makes repeated conflict/residual shapes visible as prior-memory caution.

### Added: unit and KG proof for deeper caution surfaces
- Added focused validation regressions for temporal-conflict and residual-decision negative-knowledge matches across `SemanticIR` and `IntentIR`.
- Strengthened the existing semantic-conflict caution fixtures so prior-guided and no-prior cases now also lock `SemanticIR` / `IntentIR` negative-knowledge validation behavior.
- Added [negative_knowledge_prior_guided_temporal_conflict_caution_gold](crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_temporal_conflict_caution_gold/fixture.json), proving a repeated high/low temporal contradiction is flagged as caution while the conflict remains present.
- Added [negative_knowledge_prior_guided_residual_caution_gold](crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_residual_caution_gold/fixture.json), proving a repeated residual packet class is flagged as caution without removing the residual.

### Validation
- `cargo test --manifest-path Cargo.toml negative_knowledge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (EvidenceIR validation now consumes negative-knowledge priors)

### Added: bounded negative-knowledge caution surfacing
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with exact-pattern lookup support for `negative_knowledge_priors`.
- Moved the signal-semantic conflict pattern builder into the prior-memory module so harvesting and validation consumption use the same signature shape.
- Updated [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) to reuse that shared pattern builder for `SignalSemanticConflict` negative-knowledge harvesting.
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so `EvidenceIR` validation can load the persisted `prior_memory_path`, match current signal-semantic conflict patterns against prior negative knowledge, and report `negative_knowledge_prior_matches`.
- When a match exists, validation emits `evidence_negative_knowledge_prior_matches` as an info-level caution finding.

### Preserved: negative knowledge cannot suppress evidence
- The consumer is validation-only in this slice.
- It requires a current local `EvidenceIR.signal_semantic_conflicts` record before any prior can match.
- It does not mutate `EvidenceIR`.
- It does not change semantic arbitration.
- It does not delete conflicts, weaken findings, or synthesize canonical `SemanticIR` / `IntentIR` facts.

### Added: KG-quality proof for caution-only behavior
- Strengthened [visual_sources_semantic_conflict_negative](crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/fixture.json) so the same local conflict reports `negative_knowledge_prior_matches = 0` when no prior memory is staged.
- Added [negative_knowledge_prior_guided_semantic_conflict_caution_gold](crates/specforge/test_data/kg_quality/negative_knowledge_prior_guided_semantic_conflict_caution_gold/fixture.json), which proves a staged negative-knowledge prior surfaces a validation caution while the current semantic conflict remains contested downstream.

### Validation
- `cargo test --manifest-path Cargo.toml negative_knowledge -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml validate_evidence_ir_surfaces_negative_knowledge_prior_matches -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (EvidenceIR now consumes visual-motif priors)

### Added: bounded visual-motif prior classification in `EvidenceIR`
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with a visual-caption lookup that resolves a unique learned `DiagramKind` only after normalizing a current caption against locally grounded signal and actor vocabulary.
- Updated [evidence.rs](crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can use that lookup for a current `SourceIR.visual_assets` entry when:
  - the current visual asset has `diagram_kind = unknown`
  - the current visual asset has local caption text
  - the normalized caption matches exactly one learned visual-motif prior in the applicable protocol scope
- The result is an explicit `VisualObservationKind::Classification` observation created by `specforge_prior_memory` with medium confidence.
- The prior-guided diagram kind is allowed to influence the visual evidence role, so a recovered timing diagram can be treated as normative visual evidence.

### Preserved: local-grounding safety boundary
- This path does not mutate `SourceIR`.
- This path does not synthesize semantic-role, temporal, or canonical `IntentIR` facts.
- Explicit local `SourceIR.diagram_kind` values still win outright; prior memory only helps when the current source asset is still unknown.
- Ambiguous visual-motif memory stays silent rather than picking a diagram kind.

### Added: validation metric and KG-quality fixture
- Updated [validate.rs](crates/specforge/src/commands/validate.rs) so EvidenceIR validation now reports `visual_classification_observations`.
- Added [visual_motif_prior_guided_diagram_classification_gold](crates/specforge/test_data/kg_quality/visual_motif_prior_guided_diagram_classification_gold/fixture.json), which proves a locally unknown `XREQ cycle trace` visual asset gains a prior-backed timing-diagram classification only through staged prior memory.

### Validation
- `cargo test --manifest-path Cargo.toml visual_motif -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (CorpusMemory now has visual-motif and negative-knowledge prior families)

### Added: typed learning memory families for visual motifs and negative knowledge
- Updated [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so `CorpusMemory` schema version `5` can carry `visual_motif_priors` and `negative_knowledge_priors`.
- Visual-motif priors remember reusable source-side visual patterns such as diagram kind, asset kind, normalized caption phrase, protocol family, support count, source documents, and strongest confidence.
- Negative-knowledge priors remember cautionary extraction archetypes such as semantic conflicts, temporal value conflicts, interface-signal conflicts, connectivity conflicts, and unresolved residual-decision classes.
- Added query helpers for retrieving visual-motif and negative-knowledge priors by protocol family and prior kind.
- The new families are typed, inspectable, and advisory-only; this slice does not let visual-motif or negative-knowledge priors directly author canonical `EvidenceIR`, `SemanticIR`, or `IntentIR` truth.

### Added: `learn-priors` harvesting for the new families
- Updated [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so validated `IntentIR` artifacts can harvest visual motifs from their linked `SourceIR.visual_assets`.
- `learn-priors` now harvests negative-knowledge signatures from carried conflicts and residual decisions without declaring any individual conflicting phrase false.
- `learn-priors` now reports `visual_motif_priors` and `negative_knowledge_priors` counts in its CLI output.
- Updated the fixture-local prior-memory patch path in [kg_bench.rs](crates/specforge/src/commands/kg_bench.rs) so future KG-quality fixtures can seed those two prior families explicitly.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors_harvests_visual_motif_and_negative_knowledge_priors -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (VLM timing tuples now lift into temporal signal values)

### Added: typed signal-value lift from timing-diagram VLM observations
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so `TimingDiagramExtraction` observations now read `signals[].values[]` tuples, not only free-text `annotations`.
- Signal/value tuples such as `XREQ` at `T1` with state `HIGH` now become VLM-backed `SignalConstraintRecord` entries and feed the existing temporal-rule builder as `SignalValue` predicates.
- VLM timing signal names are gated against the document-grounded signal universe when possible, using interface signal records plus locally extracted statement signal tokens.
- Diagram cycle labels such as `T0` and `T1` are preserved as explicit cycle windows, so distinct timing-diagram states do not collapse into false same-cycle conflicts.
- Generic visual words such as `transfer` are still rejected as signal names, so timing-diagram lift does not turn diagram prose into fake hardware signals.
- VLM states such as `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` are normalized into typed signal-constraint kinds, while unknown/don't-care values remain unpromoted.

### Added: regression coverage for timing tuple lift
- Strengthened `vlm_timing_diagram_observation_produces_timing_constraint_records` so it proves:
  - VLM timing annotations still become timing constraint records
  - VLM `signals[].values[]` tuples now become grounded signal constraints
  - those signal constraints feed temporal `SignalValue` predicates
  - generic timing-diagram words do not become VLM-authored signal constraints

### Validation
- `cargo test --manifest-path Cargo.toml vlm_timing -- --nocapture` -> passed
- `cargo test --manifest-path Cargo.toml kg_bench_runs_tracked_fixtures -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed
- `git diff --check` -> passed

## 2026-04-10 (VLM state-machine guards no longer become raw fake signals)

### Fixed: bounded guard parsing for visual state-machine observations
- Updated [semantic.rs](crates/specforge/src/ir/semantic.rs) so `StateMachineExtraction` VLM transition guards are parsed conservatively before entering `SemanticIR`.
- `SemanticIR` now builds a document-grounded signal universe for VLM guard parsing from interface signal records plus extracted local statement signal tokens.
- VLM guard strings now prefer simple typed comparisons such as `PREADY = 1`, `PREADY == 1`, and `PREADY != 0` instead of turning the entire guard text into a `SignalIsHigh` record.
- Generic VLM words such as `transfer`, `transaction`, `request`, `response`, `beat`, `cycle`, and `phase` no longer become fake signal names unless they are explicitly grounded as document signal names.
- VLM guard values such as `0`, `1`, `HIGH`, `LOW`, `true`, `false`, `ASSERTED`, and `DEASSERTED` stay as literal guard values instead of being misread as signal references.

### Added: regression coverage for visual guard normalization
- Strengthened `vlm_state_machine_observation_accepts_fenced_json_with_trailing_prose` so it proves:
  - generic guard prose like `Transfer` is dropped when it is not document-grounded as a signal
  - compound guard prose like `PREADY = 1 and transfer` preserves the declared signal comparison `PREADY == 1`

### Validation
- `cargo test --manifest-path Cargo.toml vlm_state_machine -- --nocapture` -> passed
- `bash scripts/run_ci.sh` -> passed

## 2026-04-10 (book now explains multimodal evidence and visual grounding)

### Added: dedicated public chapter for visual evidence
- Added [multimodal-evidence.md](docs/book/src/pipeline/multimodal-evidence.md) under the mdBook Pipeline Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [pipeline/overview.md](docs/book/src/pipeline/overview.md), [sourceir.md](docs/book/src/pipeline/sourceir.md), [evidenceir.md](docs/book/src/pipeline/evidenceir.md), [semanticir.md](docs/book/src/pipeline/semanticir.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), and [validation.md](docs/book/src/quality/validation.md) so the visual path is discoverable from the public docs.

### Clarified: VLM output is bounded evidence, not canonical truth
- The new chapter explains:
  - how `SourceIR.visual_assets` preserves figures, captions, placeholders, notes, and diagram classifications
  - how `EvidenceIR.visual_evidence` carries asset ids, roles, page/source grounding, captions, figure references, observations, and confidence
  - how `specforge enrich` can add bounded VLM notes such as `vlm_timing_diagram_extraction:` and `vlm_state_machine_extraction:`
  - how those notes become `TimingDiagramExtraction` and `StateMachineExtraction` observations
  - how `SemanticIR` can parse timing-diagram observations into timing constraints and state-machine observations into state / transition records
  - why captions can contribute semantic evidence only through the same observation, candidate, arbitration, and consensus machinery as prose and tables
  - why cross-modality agreement is stronger than repeated same-modality evidence, but still not automatic truth
  - why passive figure references should not create residuals by themselves
  - which visual and multimodal validation metrics users should inspect when debugging visual behavior

## 2026-04-10 (book now explains temporal semantics and timing rules)

### Added: dedicated public chapter for typed timing
- Added [temporal-semantics.md](docs/book/src/domain/temporal-semantics.md) under the mdBook Domain Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [domain/overview.md](docs/book/src/domain/overview.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), [semanticir.md](docs/book/src/pipeline/semanticir.md), and [validation.md](docs/book/src/quality/validation.md) so the public docs now expose the temporal-rule model directly.

### Clarified: timing prose becomes typed obligations when grounded
- The new chapter explains:
  - `TemporalRuleRecord` structure at a public level
  - clock edges and tick phases
  - `SignalValue`, actor-grounded predicates, `SignalStable`, `SignalSampled`, and `HandshakeComplete`
  - cycle windows for same-cycle, next-cycle, and bounded timing language
  - temporal conflicts and the conflict evidence they preserve
  - polarity-aware comparison for `ASSERTED` / `DEASSERTED`
  - prior-guided timing phrase recovery, with the same local-grounding safety rule used elsewhere
  - validation metrics such as `temporal_rules_with_cycle_window`, `temporal_rules_with_actor_grounding`, and `temporal_rules_with_handshake_completion`

## 2026-04-09 (book now explains graph-first actor connectivity)

### Added: dedicated public chapter for actor connectivity
- Added [actor-connectivity.md](docs/book/src/domain/actor-connectivity.md) under the mdBook Domain Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [domain/overview.md](docs/book/src/domain/overview.md), [introduction.md](docs/book/src/introduction.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), and [semanticir.md](docs/book/src/pipeline/semanticir.md) so the new graph-direction domain chapter is visible from the public docs path.

### Clarified: direction is a structural KG problem first
- The new chapter explains:
  - `Drives` / `Reads` as actor-signal graph relation types
  - how graph relations become actor-relative ports
  - how actor ports group into signal connectivity records
  - why graph-derived direction is stronger than flat compatibility direction hints
  - where graph facts come from today, including source/destination tables, drive/sample prose, prior-guided section headings, and local complementary actor recovery
  - why bogus labels such as `Clock`, `Reset`, `External`, `Tie-off`, `input`, and payload/event nouns must not become fake protocol actors
  - how multiple-producer ambiguity remains visible as connectivity conflict state
  - how graph facts support actor-grounded temporal predicates

## 2026-04-09 (book now explains handshake semantic-role arbitration)

### Added: dedicated public chapter for handshake and semantic roles
- Added [handshake-semantics.md](docs/book/src/domain/handshake-semantics.md) under the mdBook Domain Model section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [domain/overview.md](docs/book/src/domain/overview.md), [introduction.md](docs/book/src/introduction.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), and [semanticir.md](docs/book/src/pipeline/semanticir.md) so the new domain chapter is discoverable from the public docs path.

### Clarified: semantic-role truthfulness is now documented as a domain model
- The new chapter explains the current role surface:
  - `handshake_valid_like`
  - `handshake_ready_like`
  - semantic observations before winners
  - candidates, grounding strength, arbitration, and consensus
  - alias-dependent and prior-guided meaning
  - `HandshakeComplete` temporal predicates
  - blocked name fallback when signal spelling is unsafe
- This makes the earlier semantic-arbitration doctrine public, instead of requiring readers to infer it from `SemanticIR` fields or validation findings.

## 2026-04-09 (book now has a domain-model section for clock/reset infrastructure)

### Added: dedicated public domain-model chapter
- Added [domain/overview.md](docs/book/src/domain/overview.md) as the first mdBook domain-model landing page.
- Added [domain/clock-reset.md](docs/book/src/domain/clock-reset.md) to explain why clocks and resets are infrastructure semantics, not ordinary protocol edges.

### Clarified: clock/reset truthfulness doctrine is now public
- The new chapter explains:
  - why clock and reset trees are sensitive digital-system infrastructure
  - why `ASSERTED` / `DEASSERTED` must remain polarity-relative
  - how `system_contract` records clock/reset signal, reset kind, reset polarity, assertion timing, release timing, and target kind
  - why `system_clock` / `system_reset` connectivity classes are distinct from `protocol`
  - why labels like `External` and `Tie-off` should not become ordinary protocol actors
  - what validation surfaces can show today
  - what the current model still does not attempt to cover physically, such as full clock-tree or reset-tree topology

### Changed: book navigation now exposes domain semantics explicitly
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [introduction.md](docs/book/src/introduction.md), [architecture-rationale.md](docs/book/src/architecture-rationale.md), [semanticir.md](docs/book/src/pipeline/semanticir.md), and [README.md](README.md) so domain-model semantics are no longer buried inside the IR-stage discussion.

## 2026-04-09 (book reference section now explains generated artifacts and continuity boundaries)

### Added: dedicated mdBook reference landing page
- Added [overview.md](docs/book/src/reference/overview.md) so the book now has a stable reference entry point instead of making the generated-artifacts page double as the whole reference section.
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md) so [Generated Artifacts](docs/book/src/reference/generated-artifacts.md) is now an explicit reference chapter.

### Expanded: generated artifact documentation
- Expanded [generated-artifacts.md](docs/book/src/reference/generated-artifacts.md) so it now explains:
  - why `generated/` is local execution state rather than source code
  - how the stage artifact roots map to `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
  - what source-side sidecars, validation reports, adapter artifacts, and `CorpusMemory` are for
  - why arbitrary live generated artifacts stay untracked while curated fixtures can still be tracked
  - how to inspect artifacts by tracing problems backward through the staged pipeline

### Expanded: live-docs versus book contract
- Expanded [live-docs.md](docs/book/src/reference/live-docs.md) so the public book now spells out the distinction between:
  - the book as the world-facing documentation product
  - root markdown docs as the operational continuity plane for scores, roadmap state, handoff notes, and crash recovery
- Added practical guidance for when to update the book, the live docs, or both.

## 2026-04-09 (book IR stage chapters now explain their real boundaries)

### Expanded: the mdBook pipeline chapters are no longer only thin stage summaries
- Expanded [pipeline/overview.md](docs/book/src/pipeline/overview.md) so it now explains stage boundaries and the different truthfulness contracts each IR stage is supposed to uphold.
- Expanded [pipeline/sourceir.md](docs/book/src/pipeline/sourceir.md) so it now explains what `SourceIR` practically preserves, why early investment there mattered, and what structural failure modes still belong to Tier 1.
- Expanded [pipeline/evidenceir.md](docs/book/src/pipeline/evidenceir.md) so it now explains what `EvidenceIR` is allowed to extract, why provenance matters there, and what kinds of false promotion the stage is supposed to avoid.
- Expanded [pipeline/semanticir.md](docs/book/src/pipeline/semanticir.md) so it now explains semantic arbitration, graph-first direction, infrastructure handling, and why `SemanticIR` is the main semantic safety boundary before canonical intent.
- Expanded [pipeline/intentir.md](docs/book/src/pipeline/intentir.md) so it now explains what canonical means in this project, why adapters come later, and why honest incompleteness is still acceptable there.

### Changed: the public book now explains not just the stages, but the allowed decisions at each stage
- This moves the book closer to the intended public role: not only listing the pipeline, but explaining what each layer is for, what it should and should not decide, and why the staged separation exists.

## 2026-04-09 (book now has a first-class architecture rationale chapter)

### Added: dedicated mdBook chapter for why `specforge` is built this way
- Added [architecture-rationale.md](docs/book/src/architecture-rationale.md).
- It explains the core public-facing design logic:
  - why the tool is staged
  - why it is provenance-first
  - why it uses bounded AI instead of a black-box "read the whole PDF" approach
  - why residuals and conflicts are first-class
  - why the learning plane is symbolic and separate from canonical per-document truth

### Changed: the book entry path now exposes rationale earlier
- Updated [SUMMARY.md](docs/book/src/SUMMARY.md), [introduction.md](docs/book/src/introduction.md), [README.md](README.md), and [USER_GUIDE.md](USER_GUIDE.md) so readers encounter the architecture explanation before diving straight into usage details.

## 2026-04-09 (book now covers validation and learning as first-class topics)

### Added: dedicated mdBook chapters for validation and the learning plane
- Added [validation.md](docs/book/src/quality/validation.md) to explain how `specforge` judges artifact quality, why scores are secondary to findings, and how validation gates the learning plane.
- Added [kg-bench.md](docs/book/src/quality/kg-bench.md) to explain the fixture harness as a truthfulness regression system rather than just another CLI command.
- Added [corpus-memory.md](docs/book/src/quality/corpus-memory.md) to explain what actually grows over time, what the prior store learns, and why it is explicit symbolic memory rather than hidden model weights.

### Changed: command docs now point readers toward deeper rationale chapters
- Updated [quality-and-learning.md](docs/book/src/commands/quality-and-learning.md) so it stays the operational CLI page while linking to the deeper validation and learning chapters.
- Updated [introduction.md](docs/book/src/introduction.md), [SUMMARY.md](docs/book/src/SUMMARY.md), [README.md](README.md), and [USER_GUIDE.md](USER_GUIDE.md) so the new book coverage is visible from the entry path.

## 2026-04-09 (book and continuity docs now have an explicit split contract)

### Changed: the mdBook is now explicitly the public documentation product
- Updated [README.md](README.md), [ROADMAP.md](ROADMAP.md), and [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) so the repo now states this plainly:
  - the mdBook is what the outside world should read
  - it should openly explain what `specforge` does, how it works, and why it is designed that way
  - every meaningful user-facing aspect of the project should ultimately land in the book with its own section or chapter

### Added: a dedicated book page for documentation scope
- Added [documentation-scope.md](docs/book/src/reference/documentation-scope.md) and linked it from [SUMMARY.md](docs/book/src/SUMMARY.md).
- Updated [introduction.md](docs/book/src/introduction.md) and [live-docs.md](docs/book/src/reference/live-docs.md) so the book now explains the split directly instead of only implying it.

### Clarified: root markdown docs are a separate continuity plane
- The root docs are now described consistently as continuity / steering infrastructure for:
  - crash recovery
  - session handoff
  - roadmap and validation projection
  - engineering-state tracking
- They are no longer described as a second competing public documentation surface.

## 2026-04-09 (tie-off appendix rows no longer create fake AXI producers)

### Fixed: `Tie-off` is no longer treated as a real protocol actor
- Tightened [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so shared actor-term hygiene now rejects `Tie-off` / `tie off` the same way it already rejects infrastructure placeholders like `External`.
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so `Source = Tie-off` rows no longer author `ActorSignalRelation::Drives` edges through table relation recovery.

### Fixed: tie-off source rows now synthesize honest input declarations
- Updated [evidence.rs](crates/specforge/src/ir/evidence.rs) so `Tie-off` source labels still contribute local direction information, but as `input` declarations instead of fake `output` declarations.
- This keeps appendix control pins like `BROADCASTATOMIC`, `BROADCASTSHAREABLE`, `BROADCASTCACHEMAINT`, `BROADCASTCMOPOPA`, `BROADCASTPERSIST`, and `BROADCASTSTORAGE` in the declared signal surface without pretending there is a real driving actor named `Tie-off`.

### Added: regression coverage for tie-off input rows
- Added `tie_off_source_rows_become_input_declarations_without_fake_actor`, which proves `Tie-off` rows synthesize `Signal ... is input width 1.` declarations while keeping `Tie-off` out of the structural KG.
- Kept `external_source_rows_do_not_synthesize_infrastructure_outputs` green, so the broader infrastructure-label hygiene remains intact.

### Changed: AXI stays at `85/100 GOOD`, but the structural surface is more truthful
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the tracked four-artifact projection.
- The fake `Tie-off` actor is gone.
- The old AXI warning about `6` consumer-less `BROADCAST*` connectivity records is gone.
- AXI now carries `7` findings instead of `8`, even though the overall score stays `85/100 GOOD`.
- The score stayed flat because graph-direction coverage became more honest after removing the fake producer path:
  - `with_graph_direction` dropped from `170` to `164`
  - graph-derived direction lag rose from `118` to `124`
- The remaining honest AXI outliers are now:
  - `ARCHUNKEN` producer ambiguity
  - graph-direction coverage lag
  - `15` typed temporal conflicts
  - the dedicated infrastructure sourcing note for `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml tie_off_source_rows_become_input_declarations_without_fake_actor -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml external_source_rows_do_not_synthesize_infrastructure_outputs -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed (`extracted_statement_count: 6974`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 15`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`actor_count: 15`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`, `finding_count: 7`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (external infrastructure rows no longer synthesize false AXI outputs)

### Fixed: `External` is no longer treated as a protocol actor in source-column relation recovery
- Tightened [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so the shared actor-term hygiene now rejects generic environment labels like `External`.
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so source-column signal-table relation extraction now uses the stricter relation-actor normalizer instead of the weaker raw table-label normalizer.

### Added: regression coverage for external infrastructure rows
- Added `external_source_rows_do_not_synthesize_infrastructure_outputs`, which proves `ACLK` / `ARESETN` rows with `Source = External` still recover clock/reset semantics locally but no longer synthesize fake protocol actors or false `output` declarations.
- Kept `source_table_relations_skip_infrastructure_labels` green, so the broader infrastructure-row rejection path remains intact.

### Changed: AXI still scores `85/100 GOOD`, but the interface-conflict surface is cleaner again
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI now carries:
  - `0` interface signal conflicts
  - `0` residual decisions
  - `0` semantic-role conflicts
  - `0` blocked handshake-name fallbacks
- The score stays `85/100 GOOD`, so the remaining drag is now clearly elsewhere:
  - `ARCHUNKEN` producer ambiguity
  - graph-direction coverage lag
  - `15` typed temporal conflicts
  - infrastructure sourcing still intentionally lives under the dedicated `[info:system_contract]` note for `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml external_source_rows_do_not_synthesize_infrastructure_outputs -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_skip_infrastructure_labels -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 16`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`actor_count: 16`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`, `interface_signal_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (AXI semantic-hint hygiene removed false handshake conflict paths)

### Fixed: generic acknowledged-event prose no longer masquerades as ready-like semantics
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so generic acknowledgment wording no longer becomes `handshake_ready_like` by default.
- Ready-like acknowledgment recovery now requires more specific request/transfer/receipt phrasing instead of treating any `acknowledged` sentence as handshake acceptance semantics.

### Fixed: table-of-contents dot-leader lines no longer produce semantic-role hints
- Added a structural-noise guard in [evidence.rs](crates/specforge/src/ir/evidence.rs) so dot-leader contents rows and similar non-semantic structural lines stop contributing prose semantic hints outside real signal-description tables.
- This closes the exact false-positive path that had been turning the AXI contents line for `A14.1.1 AWAKEUP rules and recommendations` into a bogus `valid_like` + `ready_like` conflict.

### Added: focused regressions for both false-positive paths
- Added `acknowledged_event_prose_does_not_create_ready_like_hint`.
- Added `dot_leader_contents_lines_do_not_create_semantic_hints`.

### Changed: AXI stays at `85/100 GOOD`, but the artifact is cleaner again
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI now carries:
  - `0` residual decisions
  - `0` semantic-role conflicts
  - `0` blocked handshake-name fallbacks
- The score stays `85/100 GOOD`, so the remaining drag is no longer semantic-role noise; the main live AXI gaps are now the `ARCHUNKEN` producer ambiguity, the `ACLK` / `ARESETN` interface direction disagreement, graph-direction coverage lag, and `15` temporal conflicts.

### Validation
- `cargo test --manifest-path Cargo.toml acknowledged_event_prose_does_not_create_ready_like_hint -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml dot_leader_contents_lines_do_not_create_semantic_hints -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`, `signal_semantic_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (mdBook is explicitly a live project book, not a static scaffold)

### Changed: the documentation contract now treats the book as a living project surface
- Updated [README.md](README.md) so the entry-point docs now say the `mdBook` should be treated as a live book that evolves with user-facing project changes.
- Updated [ROADMAP.md](ROADMAP.md) so the cross-cutting doctrine now says the `mdBook` under `docs/book/` must evolve alongside user-facing commands, runtime behavior, IR semantics, validation surfaces, and cross-document learning behavior.
- Updated [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) so the book is explicitly part of the continuity contract: it is the canonical user-facing documentation surface, it should be treated as a live book, and meaningful user-facing changes should refresh it in the same task instead of being left to drift.

## 2026-04-09 (mdBook is now the canonical user-facing docs surface)

### Added: a real `mdBook` for layered user-facing documentation
- Added the canonical book scaffold under [docs/book/book.toml](docs/book/book.toml) and [docs/book/src/SUMMARY.md](docs/book/src/SUMMARY.md).
- Seeded the first layered chapter set for:
  - introduction
  - getting started
  - runtime and `doctor`
  - command workflow
  - pipeline model (`SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`)
  - reference material for generated artifacts, live docs, and troubleshooting

### Changed: root docs now point to the book instead of trying to be the full user-doc surface themselves
- [README.md](README.md) now marks the `mdBook` as the canonical user-facing documentation path and explains how to build it locally.
- [USER_GUIDE.md](USER_GUIDE.md) is now a compatibility pointer to the book instead of a second large parallel user-doc surface.

### Changed: CI now treats docs as first-class project quality, not an optional side task
- Added [scripts/run_docs_ci.sh](scripts/run_docs_ci.sh) as the canonical local docs build entrypoint.
- [scripts/run_ci.sh](scripts/run_ci.sh) now runs the mdBook build after Rust formatting and tests.
- [.github/workflows/ci.yml](.github/workflows/ci.yml) now installs `mdbook v0.5.2` before running the shared CI script, so GitHub checks the same Rust + docs path that local CI runs.

### Validation
- `bash scripts/run_docs_ci.sh` → passed
- `bash scripts/run_ci.sh` → passed (`245/245` tests, then mdBook build)

## 2026-04-09 (abstract transport tables no longer leak into canonical AXI interfaces)

### Fixed: generic `Tx` / `Rx` transport-primitives no longer masquerade as top-level interface signals
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so `signal_description` tables are rejected from the top-level signal surface when they are really abstract transport exemplars: bare transport primitive names such as `VALID`, `PENDING`, `CRDT`, `CRDTSH`, `SHAREDCRD`, and `RP` combined with only `Tx` / `Rx` actor terms.
- This keeps real prefixed interface tables like `AWVALID`, `ARCRDT`, or `AWSHAREDCRD` intact, while preventing appendix-level transport teaching tables from authoring canonical declarations, actor relations, and semantic hints.

### Added: regression coverage for abstract transport-table leakage
- Added a focused evidence regression proving that a `Credited channel signals` table containing only abstract `Tx` / `Rx` transport primitives does not synthesize top-level signal declarations, actor relations, or semantic hints.
- Kept the existing standalone `VALID` / `READY` semantic regression green, so the fix stays narrow instead of globally banning simple protocols that really do use those signal names.

### Changed: AXI still scores `85/100 GOOD`, but the artifact is much cleaner and more honest
- Rebuilt AXI from `EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- The fake bare-transport `VALID` surface is gone from canonical AXI. Actor count dropped from `19` to `17`, unresolved consumer-less connectivity collapsed from `178` signals to `6`, and structural producer ambiguity dropped from `2` signal-connectivity conflicts to `1`.
- The remaining AXI residual/finding surface is now narrower and more truthful:
  - blocked handshake fallback moved from contested bare `VALID` to contested `CRVALID`
  - semantic conflicts are now specific to `AWAKEUP` and `CRVALID`
  - the remaining structural producer ambiguity is `ARCHUNKEN`
  - interface conflicts remain the infrastructure direction disagreement on `ACLK` / `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml abstract_transport_signal_tables_do_not_become_top_level_interfaces -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml builds_semantic_ir_from_handshake_evidence -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 17`, `residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`behavior_count: 1202`, `constraint_count: 1244`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-09 (axi field-like message tables no longer leak pseudo-signals)

### Fixed: field-like `Name | Width | Description` tables no longer masquerade as interface signal tables
- Tightened [evidence.rs](crates/specforge/src/ir/evidence.rs) so top-level signal-table recovery now considers the nearest section title as well as the local caption and headers.
- Continued-page DVM message-field tables now stay classified as field-like context instead of leaking pseudo-signals such as `IS`, `PA`, and `COMPLETION` into `EvidenceIR`.

### Added: regression coverage for continued-page field-table leakage
- Added a focused evidence regression proving that a misclassified field-like `Name | Width | Description` continuation table does not synthesize fake signal declarations, polarity facts, or semantic hints.

### Changed: AXI live quality improved and its remaining gaps are more honest
- Rebuilt AXI from `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> validate` and refreshed the four-artifact validation projection.
- AXI improved from `84/100 GOOD` to `85/100 GOOD`.
- The fake `PA` / `COMPLETION` missing-producer warning is gone, the fake `IS` polarity conflict is gone, the canonical signal denominator dropped from `312` to `294`, graph-derived direction coverage improved from `57%` to `59%`, and the remaining AXI residual surface is now the single blocked handshake-name fallback on contested `VALID`.

### Validation
- `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml field_like_width_table_does_not_leak_message_fields_as_signals -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`actor_count: 19`, `residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`behavior_count: 1202`, `constraint_count: 1272`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed (`85/100 GOOD`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (passive visual links no longer force semantic residuals)

### Fixed: ambiguous-visual residuals now require live semantic lift
- Tightened [semantic.rs](crates/specforge/src/ir/semantic.rs) so `semantic_ambiguous_visual_grounding` is emitted only when ambiguous or unknown visual evidence actually survives into carried semantic observations.
- Passive figure references that are merely linked from prose no longer keep a semantic-stage residual alive by themselves.

### Added: regression coverage for passive-vs-live visual grounding
- Added a focused semantic regression proving that a passive ambiguous figure link does not emit a residual packet.
- Added a paired regression proving that an actually lifted visual-backed semantic observation still keeps the residual visible when the visual role remains ambiguous.

### Changed: APB, AHB, and AXI now carry zero residual decisions
- Rebuilt `SemanticIR` / `IntentIR` / validation for the live APB, AHB, and AXI artifacts and refreshed the tracked four-artifact projection.
- APB, AHB, and AXI now all carry `0` residual decisions end to end; the old common `semantic_ambiguous_visual_grounding` residual is gone because those live artifacts were only carrying passive figure links, not active visual semantic lift.
- The refreshed live projection is now AXI `85/100 GOOD`, APB `94/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`; a follow-on rebuild from current `SourceIR` / `EvidenceIR` restored APB and AHB to the excellent lane while leaving AXI as the main live quality outlier.

### Validation
- `cargo test --manifest-path Cargo.toml passive_ambiguous_visual_links_do_not_emit_residual_decision -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml emits_residual_decision_for_ambiguous_visual_semantic_grounding -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (authoritative signal surface now anchors interface grouping)

### Fixed: heuristic interface grouping now respects declared signal vocabularies
- Tightened [semantic.rs](crates/specforge/src/ir/semantic.rs) so statement-derived interface fragments are filtered against document-grounded explicit signal declarations whenever that authoritative signal surface exists.
- This means phase words, enum labels, width symbols, and similar metadata no longer survive into heuristic interface grouping just because they were co-mentioned next to real signals in prose.

### Added: regression coverage for authoritative grouping filters
- Added a focused semantic regression proving authoritative signal vocabularies suppress undeclared metadata like `SETUP` / `ACCESS` while retaining real declared signals.

### Changed: APB, AHB, and AXI all lost the carried interface-grouping residual
- Rebuilt `SemanticIR` / `IntentIR` / validation for the live APB, AHB, and AXI artifacts and refreshed the tracked four-artifact projection.
- `semantic_interface_grouping` is gone across all three; a later follow-up also removed the remaining passive visual residuals, so the current live baseline no longer carries any residual decisions on APB/AHB/AXI.
- AXI improved from the stale projected `79/100 GOOD` back to `84/100 GOOD`; APB remains `84/100 GOOD`, AHB later moved to `85/100 GOOD`, and AXI-Stream stays `90/100 EXCELLENT`.

### Validation
- `cargo test --manifest-path Cargo.toml retain_authoritative_interface_candidate_signals_prefers_declared_surface -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed
- `bash scripts/run_ci.sh` → passed
## 2026-04-08 (AXI-Stream interface grouping residual removed cleanly)

### Fixed: heuristic interface grouping now ignores width/table metadata noise
- Tightened [semantic.rs](crates/specforge/src/ir/semantic.rs) so heuristic interface grouping filters out metadata-only symbols like `*_WIDTH`, `_WIDTH`, `MIN`, and `MAX` before building statement-derived interface fragments.
- This complements the earlier signal-token gate that already rejected leading-digit hex-like values such as `0A`, `0B`, `0E`, and `0F`.

### Fixed: explicit interfaces now subsume smaller grouped fragments for overlap review
- `semantic_interface_grouping` residual generation now ignores heuristic fragments that are fully subsumed by an explicit interface, so carried overlap is only reported when there is still a real unresolved grouping question.
- In AXI-Stream, that resolves the last carried residual decision instead of preserving a bookkeeping artifact caused by one explicit interface plus many smaller statement fragments.

### Added: regression coverage for metadata filtering and explicit-subsumption overlap handling
- Added focused semantic regressions proving width/table metadata is filtered from heuristic interface candidates.
- Added focused semantic regressions proving explicit interfaces suppress already-subsumed overlap while genuinely unsubsumed heuristic overlap still remains visible.

### Changed: AXI-Stream now carries zero residual decisions without score inflation
- Rebuilt AXI-Stream `SemanticIR` and `IntentIR`, then refreshed the tracked four-artifact validation projection.
- AXI-Stream remains at `90/100 EXCELLENT`, but interface count drops from `46` to `29`, `SemanticIR` / `IntentIR` residual decisions both drop to `0`, and the only remaining projected finding is the infrastructure `system_contract` note for `ACLK` / `ARESETN`.

### Validation
- `cargo test --manifest-path Cargo.toml filtered_interface_candidate_signals_drop_width_and_table_metadata_noise -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml overlapping_interface_signals_keep_unsubsumed_heuristic_overlap_visible -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed (`interface_count: 29`, `residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed (`residual_decision_count: 0`)
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `residual_decisions: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (doctor now checks LM Studio fallback readiness too)

### Added: doctor now verifies the LM Studio fallback path as well as the default Ollama path
- Extended [doctor.rs](crates/specforge/src/commands/doctor.rs) so `specforge doctor [--strict]` now checks and reports:
  - LM Studio `/v1/models`
  - default-model presence for `qwen2.5vl:7b`
  - LM Studio OpenAI-compatible `/v1/chat/completions`
- The strict gate still reflects the default local-first pipeline (`Docling` + `Ollama`), but the CLI now surfaces whether the `lmstudio` fallback is actually usable before a long rerun depends on it.

### Added: shared OpenAI-compatible parsing for local provider preflight
- `doctor.rs` now parses OpenAI-compatible `/v1/models` payloads and reuses the same chat-response parser for both Ollama and LM Studio, instead of keeping the loopback preflight logic Ollama-specific.
- Added focused unit coverage for `/v1/models` parsing and kept the OpenAI-compatible chat parsing under test.

### Changed: the live runtime picture is now more honest
- Verified outside the sandbox that `cargo run --manifest-path Cargo.toml -- doctor --strict` now reports:
  - Docling ready via `python3.11` + `docling 2.84.0`
  - Ollama loopback fully ready for `qwen2.5vl:7b`
  - LM Studio fallback not currently reachable at `http://localhost:1234`, even though LM Studio is installed locally
- That distinction matters: “installed” is not the same as “serving a model,” and `doctor` now makes that operational difference explicit.

### Validation
- `cargo test --manifest-path Cargo.toml parse_openai_models_response_detects_default_model -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_openai_chat_response_accepts_openai_compatible_string_content -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed (outside sandbox; Docling + Ollama ready, LM Studio fallback reported unavailable)

## 2026-04-08 (doctor now checks Ollama loopback readiness too)

### Added: doctor now verifies the default local Ollama runtime, not just Docling
- Extended [doctor.rs](crates/specforge/src/commands/doctor.rs) so `specforge doctor [--strict]` now checks:
  - Docling ingest readiness
  - Ollama `/api/tags`
  - default-model presence for `qwen2.5vl:7b`
  - Ollama OpenAI-compatible `/v1/chat/completions`
- This catches the exact failure mode discovered during the fresh AXI rerun: a long `converge` can otherwise get all the way through fresh ingest before discovering that the local chat-completions path is not actually usable in the current execution environment.

### Added: typed parsing and reporting for the default Ollama loopback path
- `doctor.rs` now parses visible Ollama models from `/api/tags`, validates OpenAI-compatible chat responses from `/v1/chat/completions`, and reports both readiness and resolution text explicitly.
- Added focused unit coverage for Ollama tags parsing and chat-response parsing.

### Changed: the local runtime preflight now covers the full default local-first pipeline
- Verified outside the sandbox that `cargo run --manifest-path Cargo.toml -- doctor --strict` now reports:
  - Docling ready via `python3.11` + `docling 2.84.0`
  - Ollama tags reachable
  - Ollama chat-completions reachable
  - default model `qwen2.5vl:7b` present
- That means the default `specforge converge` runtime can now be preflighted honestly before a large PDF run instead of discovering the Ollama-side failure deep into the loop.

### Validation
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_tags_response_detects_default_model -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_chat_response_accepts_openai_compatible_string_content -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml parse_ollama_chat_response_accepts_array_content -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed (outside sandbox; Docling + Ollama loopback both ready)

## 2026-04-08 (Docling runtime discovery, doctor command, and bootstrap path)

### Added: a first-class Docling runtime doctor command
- Added [doctor.rs](crates/specforge/src/commands/doctor.rs) and wired `specforge doctor [--strict]` into the CLI in [cli.rs](crates/specforge/src/cli.rs), [commands/mod.rs](crates/specforge/src/commands/mod.rs), and [lib.rs](crates/specforge/src/lib.rs).
- The new command reports Docling readiness, the selected Python candidate, version information, all probe results, the repo-local bootstrap script path, and the exact missing-runtime resolution when `--strict` is used.

### Changed: Docling runtime discovery is now operationally stronger
- Extended [docling_backend.rs](crates/specforge/src/ir/source/docling_backend.rs) so the backend no longer depends only on ambient `python3` / `python`.
- Runtime resolution now proceeds in this order:
  - `SPECFORGE_DOCLING_PYTHON`
  - repo-local `.venv-docling`
  - versioned Python probes such as `python3.11`, `python3.12`, and `python3.10`
  - generic `python3` / `python`
- The resolver now keeps a typed diagnosis surface instead of a one-bit import probe, which is shared by both `specforge doctor` and the actual ingest backend.

### Added: supported repo-local Docling bootstrap path
- Added [bootstrap_docling.sh](scripts/bootstrap_docling.sh) as the supported repository-local Docling runtime bootstrap entrypoint.
- The script creates `.venv-docling`, installs the known-good `docling==2.84.0` runtime family by default, and prints the resulting interpreter/version state.
- Added `/.venv-docling/` to [.gitignore](.gitignore) so that runtime stays local and untracked.

### Changed: the local runtime issue is now concretely verified, not just documented
- `cargo run --manifest-path Cargo.toml -- doctor --strict` now succeeds locally and selects `python3.11` with `docling 2.84.0`, while explicitly reporting that the ambient `python3` probe is still broken because it resolves to Python `3.14.3` without `docling`.
- A fresh original-PDF ingest rerun on the AHB spec now succeeds again:
  - `cargo run --manifest-path Cargo.toml -- ingest /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf`
  - result: `normalization_status: ready`, `page_artifact_count: 104`, `visual_asset_count: 70`

### Validation
- `cargo test --manifest-path Cargo.toml doctor_defaults_to_non_strict -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml inspect_docling_runtime_prefers_repo_local_venv -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml inspect_docling_runtime_prefers_python311_path_probe_over_generic_python3 -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- doctor --strict` → passed
- `cargo run --manifest-path Cargo.toml -- ingest /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/ahb/current/IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf` → passed


## 2026-04-08 (system-contract infrastructure signals now populate canonical interfaces)

### Fixed: clock/reset signals from the system contract now reach the canonical interface surface
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so grounded `system_contract` clock/reset signals are synthesized into the top-level explicit interface when ordinary signal declarations do not already carry them.
- This lets infrastructure signals like `HCLK` and `HRESETN` contribute honest canonical interface direction/width coverage, and it allows reset polarity grounded only through system-contract text to surface as `resolved_polarity` in both `SemanticIR` and `IntentIR`.
- Added focused regressions in [semantic.rs](crates/specforge/src/ir/semantic.rs) and [validate.rs](crates/specforge/src/commands/validate.rs) covering the exact system-contract-only clock/reset case at semantic and intent validation time.

### Changed: AHB now reports resolved polarity in the live baseline
- Rebuilt AHB `SemanticIR` and `IntentIR` sequentially from the current `EvidenceIR`, re-validated the artifact, and refreshed the tracked four-document projection.
- AHB now reports `with_resolved_polarity: 1` and the live AMBA polarity line is now `1 / 1 / 1 / 1`; the overall AHB score stays `84/100 GOOD`, but the infrastructure reset polarity is now represented honestly in the canonical interface surface.
- A full original-PDF `converge` rerun was attempted first, but the local environment currently lacks an importable `docling` runtime for `python3`, so authoritative fresh-ingest reruns remain blocked until `docling` is installed or `SPECFORGE_DOCLING_PYTHON` points at a working interpreter.

### Validation
- `cargo test --manifest-path Cargo.toml system_contract_signals_become_explicit_interface_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_counts_system_contract_resolved_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_system_contract_resolved_polarity -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json` → passed (`84/100 GOOD`, `with_resolved_polarity: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (resolved signal polarity now lives on canonical interface records)

### Added: canonical interface records now carry resolved polarity directly
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so each `InterfaceSignalRecord` can now carry `resolved_polarity` directly instead of forcing downstream consumers to reconstruct polarity only from the carried top-level side list.
- [intent.rs](crates/specforge/src/ir/intent.rs) now preserves that same per-signal polarity surface into `IntentIR`.
- [validate.rs](crates/specforge/src/commands/validate.rs) now reports `with_resolved_polarity` for both `SemanticIR` and `IntentIR`.

### Changed: the live corpus now shows the gap honestly
- Rebuilt the live AMBA `SemanticIR` / `IntentIR` artifacts, re-validated the four-document projection, and refreshed the tracked snapshot docs.
- The canonical polarity surface is now present in the live corpus too: AXI, APB, AHB, and AXI-Stream each currently report `with_resolved_polarity: 1`, so the remaining polarity work is broader non-reset control coverage rather than carry-through plumbing.
- That refresh also replaced a stale optimistic validation snapshot; after the later current-`SourceIR` / current-`EvidenceIR` rebuild and the follow-on AXI field-table truthfulness fix, the tracked live baseline now stands at AXI `85/100 GOOD`, APB `94/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`.

### Validation
- `cargo test --manifest-path Cargo.toml carries_resolved_signal_polarity_into_interface_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml carries_resolved_signal_polarity_into_intent_ir -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_semantic_ir_counts_resolved_signal_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_counts_resolved_signal_polarity -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `with_resolved_polarity: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (temporal conflict comparison is now polarity-aware)

### Fixed: asserted/deasserted temporal semantics now respect signal polarity
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs), [evidence.rs](crates/specforge/src/ir/evidence.rs), and [intent.rs](crates/specforge/src/ir/intent.rs) so resolved signal polarity now survives into canonical IR and can guide temporal-conflict comparison.
- `ASSERTED` and `DEASSERTED` are now treated as polarity-relative assertion semantics, not as fixed synonyms for `HIGH` and `LOW`.
- When the current document grounds polarity, conflict detection now maps assertion semantics through that local polarity:
  - active-high: `ASSERTED -> HIGH`, `DEASSERTED -> LOW`
  - active-low: `ASSERTED -> LOW`, `DEASSERTED -> HIGH`
- When polarity is still unknown, `ASSERTED` stays abstract instead of manufacturing or suppressing a level conflict.

### Changed: AXI-Stream timing semantics are now cleaner again without changing the score
- Rebuilt `SemanticIR` and `IntentIR` for AXI-Stream from the current `EvidenceIR`, re-validated the artifact, and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, but AXI-Stream now carries `0` typed temporal conflicts instead of `1`.
- The remaining dominant honest gaps are now:
  - the infrastructure-sourcing/system-contract note for `ACLK` and `ARESETN`
  - the carried `semantic_interface_grouping` residual decision

### Validation
- `cargo test --manifest-path Cargo.toml derives_typed_temporal_conflicts_from_conflicting_value_rules -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml asserted_and_high_do_not_form_temporal_conflicts_without_known_polarity -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml asserted_and_high_form_temporal_conflict_for_active_low_signal -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/evidence_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/semantic_ir.json` → passed
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `temporal_conflicts: 0`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (same-cycle timing language now lands as bounded temporal semantics)

### Fixed: same-cycle timing phrases now recover explicit `0`-cycle windows
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `extract_cycle_window_from_text()` now recognizes bounded same-cycle language such as `in the same ACLK cycle`, `in the same tick`, and `on the current rising edge`.
- Added focused semantic regressions that lock both layers of the behavior:
  - direct phrase recovery from same-cycle timing language
  - end-to-end temporal-rule derivation from a same-cycle signal constraint

### Changed: AXI-Stream timing semantics are now more explicit without changing the score
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- Re-validated the rebuilt artifact and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, but six AXI-Stream temporal rules now carry explicit `0`-cycle windows for same-cycle handshake/timing language, so the old `intent_temporal_rules_missing_cycle_windows` warning is gone.
- The timing surface also got cleaner as a side effect: AXI-Stream now carries `1` typed temporal conflict instead of `2`.
- The remaining dominant honest gaps are now:
  - the dedicated infrastructure-sourcing note for `ACLK` / `ARESETN`
  - the single remaining typed temporal conflict
  - the carried `semantic_interface_grouping` residual decision

### Validation
- `cargo test --manifest-path Cargo.toml extracts_zero_cycle_window_from_same_cycle_phrases -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml derives_zero_cycle_window_from_same_cycle_constraint_text -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `temporal_rules_with_cycle_window: 6`, `temporal_conflicts: 1`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (clock/reset connectivity now validates as infrastructure)

### Changed: clock/reset connectivity is now classified as infrastructure in canonical IR
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SignalConnectivityRecord` now carries an explicit `connectivity_class`, with `SystemClock` and `SystemReset` derived from the local system contract instead of flattening those signals into ordinary protocol connectivity.
- Extended [intent.rs](crates/specforge/src/ir/intent.rs) so that infrastructure classification survives into `IntentIR` unchanged.
- Extended [validate.rs](crates/specforge/src/commands/validate.rs) so missing producers on infrastructure connectivity no longer emit the generic `[warning:signal_connectivity]` finding; they now surface as a dedicated `[info:system_contract]` note that keeps canonical sourcing on the system-contract side of the model.

### Changed: AXI-Stream still validates at `90/100 EXCELLENT`, but the remaining gap is now represented more honestly
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- Re-validated the rebuilt artifact and refreshed the tracked four-document projection.
- The score stayed at `90/100 EXCELLENT`, with declared graph-direction and width coverage still at `22/22`, but `ACLK` and `ARESETN` now surface under `infrastructure_signal_connectivity: 2` with an `[info:system_contract]` finding instead of a generic missing-producer warning.
- The dominant remaining honest gaps are now:
  - typed temporal rules that still have no explicit cycle-window bounds
  - the two carried temporal conflicts
  - the remaining interface-grouping residual decision

### Validation
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml carries_infrastructure_signal_connectivity_class_into_intent_ir -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml validate_intent_ir_treats_clock_and_reset_as_infrastructure_connectivity -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, `infrastructure_signal_connectivity: 2`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (corpus knowledge base plane added to roadmap)

### Added: explicit `R15g` workstream for a corpus knowledge base layer
- Logged a new roadmap slice in [ROADMAP.md](ROADMAP.md) for a persistent corpus knowledge base that sits beside the per-document IR pipeline and the typed `CorpusMemory` prior store.
- The design boundary is explicit:
  - per-document canonical truth stays in `SourceIR` / `EvidenceIR` / `SemanticIR` / `IntentIR`
  - typed machine-usable reuse stays in `CorpusMemory`
  - the new corpus knowledge base becomes the human+LLM synthesis layer for recurring motifs, failures, contradiction summaries, table/figure families, and protocol-family notes

### Changed: live architecture guidance now targets three cross-document planes, not one
- Updated [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md), [README.md](README.md), and [LIVE_ACHIEVEMENT_STATUS.md](LIVE_ACHIEVEMENT_STATUS.md) so future work treats the long-term shape as:
  - document-local canonical IR
  - typed cross-document priors
  - corpus-level compiled knowledge base
- The docs also now make the safety boundary explicit: the corpus knowledge base may guide humans, LLM synthesis, benchmark design, and prior-candidate generation, but it must not directly author canonical IR truth.

## 2026-04-08 (AXI-Stream parity-check width semantics now survive end to end)

### Fixed: parity-check rows now recover bounded width hints from their local table semantics
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so `Check Signal / Signals Covered / Width / Granularity / Check Enable` rows can recover width hints from the `Signals Covered` cell when the literal `Width` cell is only a range placeholder like `1-8`.
- Added a bounded fallback from `Check Enable` / `Granularity` to the grounded base signal when `Signals Covered` only carries a width expression, so the structural and width semantics stay tied to local evidence instead of remaining partially orphaned.
- Tightened graph-derived declaration synthesis so width-only statements no longer block stronger relation-grounded `Signal X is output width ...` declarations for the same signal.

### Changed: AXI-Stream now validates at `90/100 EXCELLENT`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but the carried `*CHK` surface is now complete enough to count honestly: declared signal records rose from `21` to `22`, actor-signal relations rose from `38` to `40`, actor ports rose from `42` to `44`, signal connectivity rose from `21` to `22`, compatibility direction hints reached `22/22`, width coverage reached `22/22`, and the projected score improved from `88/100 GOOD` to `90/100 EXCELLENT`.
- The remaining dominant gaps are now:
  - unresolved producer attribution for infrastructure signals `ACLK` and `ARESETN`
  - typed temporal rules that still have no explicit cycle-window bounds
  - the two carried temporal conflicts

### Validation
- `cargo test --manifest-path Cargo.toml check_signal_tables_inherit_relations_from_covered_signals -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`90/100 EXCELLENT`, declared graph-direction and width coverage `22/22`)
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed

## 2026-04-08 (AXI-Stream parity-check table now restores structural ownership)

### Fixed: parity-check tables now recover actor-signal relations from covered base signals
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) with a bounded second-pass relation recovery path for `Check Signal / Signals Covered` tables.
- When a local parity-check row explicitly ties a check signal to a covered base signal that already has grounded actor relations, the check signal now inherits those local `drives` / `reads` edges instead of remaining structurally orphaned.
- Added the focused regression `check_signal_tables_inherit_relations_from_covered_signals`.

### Changed: AXI-Stream now validates at `88/100 GOOD`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but parity-check ownership now survives end to end: declared signal records rose from `16` to `21`, graph-direction coverage rose from `12/16` to `21/21`, actor ports rose from `24` to `42`, signal connectivity rose from `12` to `21`, and the projected score improved from `84/100 GOOD` to `88/100 GOOD`.
- The remaining dominant gaps are now:
  - missing widths on `TDESTCHK`, `TIDCHK`, `TSTRBCHK`, `TUSERCHK`, and `TWAKEUPCHK`
  - unresolved producer attribution for infrastructure signals `ACLK` and `ARESETN`

### Validation
- `cargo test --manifest-path Cargo.toml check_signal_tables_inherit_relations_from_covered_signals -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`88/100 GOOD`, graph-direction coverage `21/21`)

## 2026-04-08 (clock/reset semantics logged as infrastructure-first steering)

### Changed: design steering now treats clocks and resets as infrastructure semantics, not ordinary protocol edges
- Logged the implementation doctrine in [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md): clocks and resets should remain first-class infrastructure semantics with conservative sourcing/distribution modeling, not flattened into ordinary protocol producer/consumer behavior.
- Updated [ROADMAP.md](ROADMAP.md) so `R15b` now explicitly carries that requirement forward into the clock-tick temporal-model workstream.
- This locks an important architectural boundary for future work on `ACLK`, `ARESETN`, and similar infrastructure signals: graph carry-through is allowed as a local aid, but long-term canonical truth should prefer dedicated infrastructure semantics over false graph completeness.

## 2026-04-08 (AXI-Stream graph-direction coverage rises after width-symbol cleanup)

### Fixed: width-only `_WIDTH` declarations no longer masquerade as interface signals
- Hardened [semantic.rs](crates/specforge/src/ir/semantic.rs) so synthesized declarations like `Signal TDATA_WIDTH is width LOW.` no longer become canonical interface-signal records when they carry width metadata but no real port direction.
- Added focused regressions for both sides of the boundary:
  - `width_only_width_parameter_declarations_do_not_become_interface_signal_records`
  - `width_only_signal_declarations_become_interface_signal_records`

### Fixed: relation-grounded actors now inherit clock/reset input ports from explicit system contracts
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so actors already grounded by structural KG evidence now receive `input` actor ports for the explicit clock and reset signals instead of leaving `ACLK` / `ARESETN` outside the graph-backed port surface.
- Added the regression `clock_and_reset_gain_input_actor_ports_for_relation_actors`, which locks that actor-relative clock/reset carry-through path.

### Changed: AXI-Stream now validates at `84/100 GOOD`
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations, but the canonical denominator is now more honest: declared interface signals dropped from `20` to `16`, graph-direction coverage rose from `10/20` to `12/16`, and the projected score improved from `80/100 GOOD` to `84/100 GOOD`.
- The remaining dominant gaps are now narrower and clearer:
  - the four `*CHK` signals still lack graph-derived direction coverage
  - `ACLK` and `ARESETN` still lack resolved producer actors even though they now have graph-backed consumer ports

### Validation
- `cargo test --manifest-path Cargo.toml width_only_width_parameter_declarations_do_not_become_interface_signal_records -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml clock_and_reset_gain_input_actor_ports_for_relation_actors -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`84/100 GOOD`, graph-direction coverage `12/16`)

## 2026-04-08 (AXI-Stream consumer-side connectivity now survives from source tables)

### Fixed: source-column signal tables can now recover the opposite-side reader when it is uniquely grounded
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so a `Source` / `Driver` column no longer stops at `(actor, Drives, signal)` when the current document already exposes exactly one opposite actor role locally.
- The new helper path builds a small local actor-role inventory from signal-description tables and section headings, then adds the complementary `Reads` edge only when the opposite requester-like/completer-like actor is unique.
- Added focused regressions for both the positive case and the ambiguity guard:
  - `source_table_relations_infer_unique_complementary_reads`
  - `source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous`
- Updated the tracked KG fixtures whose expected graph shape now honestly includes these complementary consumer edges.

### Changed: AXI-Stream now keeps consumer-side structural connectivity without changing its score
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The run still converged in `2` pipeline iterations and still validates at `80/100 GOOD`, but `IntentIR` now carries the missing consumer-side structural KG edges:
  - `Receiver reads TVALID`
  - `Transmitter reads TREADY`
  - `Receiver reads TDATA/TSTRB/TKEEP/TLAST/TID/TDEST/TUSER/TWAKEUP`
- The AXI-Stream validation finding for missing consumer actors is now gone; the remaining dominant gap is graph-derived direction coverage, not missing connectivity.

### Validation
- `cargo test --manifest-path Cargo.toml source_table_relations_infer_unique_complementary_reads -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml source_table_relations_skip_complementary_reads_when_opposite_actor_is_ambiguous -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- full `specforge converge` on AXI-Stream with Ollama VLM + NLP Level 3 → converged in `2` iterations
- `cargo run --manifest-path Cargo.toml -- validate generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`80/100 GOOD`, consumer-gap finding removed)

## 2026-04-08 (learning plane now rejects bogus actor vocabulary)

### Fixed: actor-taxonomy learning no longer harvests payload nouns as actors
- Hardened [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so actor-taxonomy priors now skip non-actor payload/event terms like `control information`, even if an earlier document-local bug let that text survive into `IntentIR`.
- Added a shared actor-term hygiene guard in [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) so actor-taxonomy prior lookup also ignores those bogus terms if an older local `CorpusMemory` still contains stale entries.
- Reused the same guard in [evidence.rs](crates/specforge/src/ir/evidence.rs), so live relation extraction and cross-document learning now reject the same class of bogus actor terms instead of drifting apart.

### Changed: local `CorpusMemory` is now cleaned of the stale AXI-Stream actor prior
- Re-ran `specforge learn-priors` across AXI/APB/AHB/AXI-Stream `IntentIR` artifacts.
- The local prior store now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors.
- The stale `control information -> requester_like` actor-taxonomy prior is now gone from local `generated/prior_memory/corpus_memory.json`.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors_skips_payload_like_actor_terms -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed (`16 / 5 / 4 / 266 / 99`)
- `bash scripts/run_ci.sh` → passed (`209/209` tests)

## 2026-04-08 (AXI-Stream bogus prose actor extraction fixed)

### Fixed: prose KG extraction no longer promotes payload nouns into actors
- Hardened [evidence.rs](crates/specforge/src/ir/evidence.rs) so prose actor extraction now normalizes candidate actor phrases through the same non-actor guard used by table extraction and rejects generic payload/event nouns like `control information`, `data`, and `transfer`.
- Tightened active-clause subject recovery so coordinated prose like `the Transmitter presents ... and asserts TVALID` keeps the real actor subject instead of capturing trailing payload phrases or clause verbs.
- Added the regression `coordinated_active_drive_extracts_real_actor_not_payload_phrase`, which locks the AXI-Stream-style sentence shape that previously leaked `control information` into the structural KG.

### Changed: AXI-Stream structural truthfulness improved without score inflation
- Re-ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The artifact still converges in `2` pipeline iterations and still validates at `80/100 GOOD`, but the carried multi-producer conflict on `TVALID` is now gone: `control information` no longer appears as an actor in `EvidenceIR`, `SemanticIR`, or `IntentIR`, and `signal_connectivity_conflicts` for AXI-Stream dropped from `1` to `0`.
- The remaining honest AXI-Stream gap is now clearer: unresolved consumer actors and graph-direction coverage, not bogus producer attribution.

### Validation
- `cargo test --manifest-path Cargo.toml coordinated_active_drive_extracts_real_actor_not_payload_phrase -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- project-validation generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/intent_ir.json` → passed
- `bash scripts/run_ci.sh` → passed (`208/208` tests)

## 2026-04-07 (AXI-Stream unseen-protocol run populates semantic priors)

### Added: first unseen-protocol full converge + learning refresh
- Ran full `specforge converge` with Ollama VLM + NLP Level 3 on [IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf](/Users/richarddje/Documents/livework/chipdoc/arm/amba/supporting/axi-stream/current/IHI0051_B_2021-04_AMBA_AXI_Stream_Protocol_Specification.pdf).
- The AXI-Stream artifact converged in `2` pipeline iterations and validates at `80/100 GOOD`; it is now included in the tracked [VALIDATION_SNAPSHOT.md](VALIDATION_SNAPSHOT.md) projection and the managed validation block in [LIVE_ACHIEVEMENT_STATUS.md](LIVE_ACHIEVEMENT_STATUS.md).
- Refreshing `specforge learn-priors` across AXI/APB/AHB/AXI-Stream now yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors in local `CorpusMemory`.

### Fixed: multi-signal table-row semantic-role leakage
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so signal-description table rows now sanitize against the full local known-signal set before semantic-role inference, preventing a secondary signal mention like `TREADY` from leaking a ready-like role onto a row subject like `TVALID`.
- Added the regression `signal_table_descriptions_ignore_other_handshake_signal_mentions`, which locks the AXI-Stream-style case where `TVALID` should stay valid-like even when its row also describes the handshake condition involving `TREADY`.

### Why this matters
- This is the first concrete proof that a new unseen protocol document can both expose an extraction flaw and then materially strengthen the learning plane once the artifact is repaired enough to be harvested.
- It also marks the first real-corpus point where semantic phrase priors and semantic modality-reliability priors become nonzero instead of remaining only architecturally possible.

## 2026-04-07 (learning-plane structure documented as first-class architecture)

### Added: explicit doctrine for how `R15f` should learn
- Logged in [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) that the cross-document learning plane should not try to "mimic humans completely," but should instead borrow the useful structural properties of human learning:
  - accumulate experience across many documents
  - abstract patterns from repeated successful cases
  - keep confidence graded
  - remember failures and false positives
  - use prior experience to guide attention
  - still require local evidence before promoting a fact
- Logged the non-negotiable architectural properties for the learning plane:
  - separation between document truth and learned priors
  - typed, inspectable memory
  - bounded influence
  - validation-gated feedback
  - negative learning
  - provenance on learned priors

### Why this matters
- This frames `R15f` as an epistemology layer, not just a bigger cache.
- It makes explicit that the design risk is not "too little learning," but poorly structured learning that can poison canonical truth.

## 2026-04-07 (table-shape timing-table benchmark added)

### Added: second table-shape gold/negative pair
- Added [table_shape_prior_guided_timing_table_gold](crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_gold/fixture.json) and [table_shape_prior_guided_timing_table_without_prior_negative](crates/specforge/test_data/kg_quality/table_shape_prior_guided_timing_table_without_prior_negative/fixture.json).
- The pair proves a locally `unknown` `Parameter | Min | Max | Unit` table stays inert without prior memory and yields `timing_constraints = 1` across `EvidenceIR`, `SemanticIR`, and `IntentIR` only when a matching table-shape prior is staged.

### Why this matters
- It broadens the first table-shape prior family beyond signal-description recovery and shows that the same bounded consumer already generalizes to timing-table interpretation.

## 2026-04-07 (semantic modality-reliability priors landed as the fifth bounded learning slice)

### Added: first semantic modality-reliability prior family in `CorpusMemory`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with typed `semantic_modality_reliability_priors` plus advisory lookup helpers that score how reliable a given semantic source kind has been for a given role and protocol family.
- Extended [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests those priors from decisive, non-alias-dependent semantic consensus records rather than from raw guesses.
- The latest local AMBA run over APB/AHB/AXI still yields `0` semantic modality-reliability priors, which is the honest current state: the family is landed, but the real canonical artifacts are not yet surfacing enough promoted semantic consensus to populate it automatically.

### Added: bounded semantic arbitration consumption and benchmark coverage
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SemanticIR` can advisory-adjust local semantic arbitration using modality-reliability priors, but only when the current PDF already contains multiple locally grounded semantic candidates; the original conflict remains visible either way.
- Extended [validate.rs](crates/specforge/src/commands/validate.rs) so semantic and intent validation now report prior-guided semantic arbitration and prior-guided semantic consensus explicitly instead of hiding that path inside the canonical result.
- Added the tracked gold/negative fixture pair [semantic_modality_reliability_prior_guided_conflict_gold](crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_gold/fixture.json) and [semantic_modality_reliability_prior_guided_conflict_without_prior_negative](crates/specforge/test_data/kg_quality/semantic_modality_reliability_prior_guided_conflict_without_prior_negative/fixture.json), which prove locally conflicted role evidence stays contested without the staged prior and becomes decisively resolved only when the matching modality-reliability prior is present.

### Why this matters
- This is the first learning slice that improves semantic arbitration itself instead of only widening local phrase, actor-vocabulary, timing, or table-shape interpretation.
- It stays within the project doctrine: the learning plane can guide which locally grounded evidence should carry more weight, but it still cannot author canonical facts that the current document did not expose.

### Validation
- `cargo test --manifest-path Cargo.toml modality_reliability_priors_can_resolve_local_semantic_conflicts -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_modality_reliability_prior_guided_conflict_gold semantic_modality_reliability_prior_guided_conflict_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`207/207` tests)

## 2026-04-07 (table-shape priors landed as the fourth bounded learning slice)

### Added: first table-shape prior family in `CorpusMemory`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with typed `table_shape_priors`, normalized structured-table header signatures, and advisory lookup helpers that can resolve a local table kind only when the signature matches uniquely.
- Extended [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests table-shape priors from validated document chains by walking `IntentIR -> SemanticIR -> EvidenceIR -> SourceIR`.
- The latest local AMBA run over APB/AHB/AXI now yields `94` table-shape priors in addition to the existing actor-taxonomy and temporal families.

### Added: bounded table-shape prior consumption and benchmark coverage
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can advisory-recover a local table kind from prior memory, but only when the current table is still `unknown`; explicit local `SourceIR.table_kind` values remain authoritative.
- Added the tracked gold/negative fixture pair [table_shape_prior_guided_signal_table_gold](crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_gold/fixture.json) and [table_shape_prior_guided_signal_table_without_prior_negative](crates/specforge/test_data/kg_quality/table_shape_prior_guided_signal_table_without_prior_negative/fixture.json), which prove a locally `unknown` `Name | Direction | Width` table stays inert without prior memory and gains signal-description recovery only when a matching learned prior is staged.

### Why this matters
- This is the first cross-document learning slice that improves table interpretation directly, not just actor vocabulary or phrase interpretation.
- It stays fully within the project doctrine: prior memory widens local interpretation, but it does not rewrite `SourceIR` or override explicit local classifications.

## 2026-04-07 (Learning-plane growth model clarified)

### Added: explicit note on what grows to materialize learning
- Logged in [DEVELOPMENT_NOTES.md](DEVELOPMENT_NOTES.md) that the learning capability is defined in code, while the thing that actually grows over time is the typed prior store, typically [corpus_memory.json](generated/prior_memory/corpus_memory.json).
- Added the matching short entry-point note in [README.md](README.md), so future sessions do not confuse `R15f` with neural-network-style hidden-weight learning.

### Why this matters
- This makes the learning model explicit: `specforge` uses symbolic, inspectable, typed memory rather than opaque learned weights.
- It also clarifies the safety boundary between:
  - code that defines how learning works
  - data that stores what has been learned
  - canonical per-document IR that remains provenance-pure

## 2026-04-07 (Actor-taxonomy priors now recover structural KG from section headings)

### Added: prior-guided structural KG recovery for width-only section-guided signal tables
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so actor-taxonomy priors can now lift width-only `Signal | Width` tables under headings like `Issuer signals` or `Acceptor signals` into structural `ActorSignalRelation::Drives` edges, not just flat compatibility directions.
- Strengthened the tracked gold fixture [actor_taxonomy_prior_guided_section_direction_gold](crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json) so it now requires graph-backed recovery too: `actor_signal_relations = 3`, `actor_ports = 3`, and `with_graph_direction = 3`.

### Why this matters
- This closes an important quality gap in the first learning-plane consumer: prior-guided section headings now improve the canonical structural KG, not just the compatibility hint surface.
- It directly supports the graph-first roadmap because learned actor vocabulary can now produce actor-relative port structure from locally grounded width-only tables.

### Validation
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_gold actor_taxonomy_prior_guided_section_direction_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided visual semantic recovery)

### Added: prior-guided visual semantic gold/negative pair
- Added [visual_semantic_prior_guided_caption_gold](crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_gold/fixture.json), which proves the unseen local visual-caption phrase `XACK can sink the transfer` gains ready-like semantic recovery only when a matching `visual_caption` semantic prior is staged into the fixture.
- Added [visual_semantic_prior_guided_caption_without_prior_negative](crates/specforge/test_data/kg_quality/visual_semantic_prior_guided_caption_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local caption stays semantically unresolved when prior memory is absent.

### Why this matters
- This broadens the first benchmark surface into a second modality without inventing a new unsafe prior family prematurely.
- It proves the bounded semantic-prior doctrine is not prose-only: prior memory can widen interpretation of local visual-caption phrasing, but it still cannot manufacture a role when the matching prior is absent.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_semantic_prior_guided_caption_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality visual_semantic_prior_guided_caption_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided actor-taxonomy recovery)

### Added: prior-guided actor-taxonomy gold/negative pair
- Added [actor_taxonomy_prior_guided_section_direction_gold](crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_gold/fixture.json), which proves width-only `Issuer signals` / `Acceptor signals` sections gain canonical signal directions only when matching actor-taxonomy priors are staged into the fixture.
- Added [actor_taxonomy_prior_guided_section_direction_without_prior_negative](crates/specforge/test_data/kg_quality/actor_taxonomy_prior_guided_section_direction_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local section headings stay directionless when prior memory is absent.

### Why this matters
- This completes the first benchmark triangle for the three initial bounded prior families: actor-taxonomy, semantic-role phrases, and temporal-language phrases.
- It proves the local-grounding doctrine for actor vocabulary too: prior memory can widen how the extractor interprets explicit local actor terms, but it cannot manufacture direction when the matching prior is absent.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality actor_taxonomy_prior_guided_section_direction_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided semantic recovery)

### Added: prior-guided unseen-phrase semantic gold/negative pair
- Added [semantic_prior_guided_phrase_gold](crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_gold/fixture.json), which proves the unseen local phrase `XACK can receive the transfer` gains ready-like semantic recovery only when a matching semantic prior is staged into the fixture.
- Added [semantic_prior_guided_phrase_without_prior_negative](crates/specforge/test_data/kg_quality/semantic_prior_guided_phrase_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local phrase stays semantically unresolved when prior memory is absent.

### Why this matters
- This is the second tracked benchmark proof that the cross-document learning plane can strengthen analysis of an unseen local phrase without leaking canonical facts across documents.
- It shows the same “local text required, prior only widens interpretation” contract now holds for both temporal-language priors and semantic-role priors.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_prior_guided_phrase_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality semantic_prior_guided_phrase_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-07 (KG fixtures now lock prior-guided temporal recovery)

### Added: fixture-owned prior-memory patching in `specforge kg-bench`
- Extended [kg_bench.rs](crates/specforge/src/commands/kg_bench.rs) so tracked fixtures can now stage a local `CorpusMemory` before `EvidenceIR` is built.
- The new patch surface is generic across actor-taxonomy, semantic, and temporal priors, so later `R15f` benchmark slices can exercise more prior families without depending on a shared mutable prior file.

### Added: prior-guided unseen-phrase temporal gold/negative pair
- Added [temporal_prior_guided_cycle_window_gold](crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_gold/fixture.json), which proves the unseen local phrase `PREADY must be asserted one beat later` gains a one-cycle `cycle_window` only when a matching temporal prior is staged into the fixture.
- Added [temporal_prior_guided_cycle_window_without_prior_negative](crates/specforge/test_data/kg_quality/temporal_prior_guided_cycle_window_without_prior_negative/fixture.json), which locks the honest fallback behavior that the same local phrase still yields a temporal rule but no bounded `cycle_window` without prior memory.

### Why this matters
- This is the first tracked benchmark evidence that the cross-document learning plane can improve analysis of an unseen local phrase without leaking canonical facts across documents.
- It upgrades `R15f` from “consumers exist” to “consumers are benchmarked against honest before/after behavior.”

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality temporal_prior_guided_cycle_window_gold` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality temporal_prior_guided_cycle_window_without_prior_negative` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-06 (bounded temporal prior consumption landed)

### Added: prior-guided cycle-window recovery in `SemanticIR`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with a typed temporal phrase lookup that can resolve a unique learned `CycleWindowRecord` from locally grounded timing text.
- Extended [semantic.rs](crates/specforge/src/ir/semantic.rs) so `SemanticIR` now loads advisory prior memory from the persisted upstream `prior_memory_path` and uses temporal phrase priors only as a fallback when direct cycle-window parsing cannot recover the local timing window.
- Added a direct semantic regression proving `PREADY must be asserted one beat later` still yields no built-in cycle window on its own, but does recover a one-cycle temporal rule when a validated temporal prior is present.

### Why this matters
- The cross-document learning plane now has a third real bounded consumer, and it lives in the temporal model instead of only in evidence extraction.
- This lets the extractor become stronger on previously unseen local timing phrase shapes without weakening the rule that canonical timing still has to be justified by the current PDF.
- The temporal prior path is still honest: without the local timing sentence there is no rule, and without a unique learned prior there is no learned cycle-window fallback.

### Validation
- `cargo test --manifest-path Cargo.toml derives_cycle_window_from_temporal_phrase_prior_when_builtin_parser_cannot -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml extracts_single_cycle_window_from_idiomatic_clock_tick_phrases -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`204/204` tests)

## 2026-04-06 (bounded semantic prior consumption landed)

### Added: prior-guided semantic hint recovery in `EvidenceIR`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with shared semantic-phrase normalization and lookup helpers, so the same phrase-shape logic now powers both `learn-priors` and runtime prior consumption.
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) so `EvidenceIR` can now use semantic phrase priors to recover local signal-role hints from non-hardcoded grounded phrases.
- `EvidenceIR` now also persists the consulted `prior_memory_path`, so later `refresh_signal_semantic_hints()` calls during NLP loopback keep the same advisory prior guidance instead of silently dropping it.

### Why this matters
- The cross-document learning plane now has a second real bounded consumer, beyond actor-taxonomy direction guidance.
- This lets the extractor become stronger on phrases it has learned from prior validated documents without breaking the local-grounding rule.
- The semantic prior path is still honest: without the local phrase, there is no semantic promotion; with the local phrase, the prior only helps interpret it.

### Validation
- `cargo test --manifest-path Cargo.toml semantic_phrase_priors_guide_local_semantic_hint_recovery -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml learn_priors_harvests_semantic_and_temporal_priors -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`203/203` tests)

## 2026-04-06 (first bounded prior-consumption path landed)

### Added: advisory prior-guided direction recovery in `EvidenceIR`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with reusable actor-taxonomy lookup helpers, normalized actor-term matching, and protocol-family inference so the learning plane can be queried safely during extraction.
- Extended [evidence.rs](crates/specforge/src/ir/evidence.rs) with `build_with_prior_memory(...)` plus the first bounded prior consumer:
  - section-heading direction inference can now use actor-taxonomy priors
  - `Source` / `Destination` column direction inference can now use actor-taxonomy priors
  - prior guidance still requires explicit local actor terms already present in the current document
- Extended [cli.rs](crates/specforge/src/cli.rs), [evidence.rs](crates/specforge/src/commands/evidence.rs), and [converge.rs](crates/specforge/src/commands/converge.rs) so `specforge evidence` and `specforge converge` now consult `--prior-memory generated/prior_memory/corpus_memory.json` by default.

### Why this matters
- The cross-document learning plane is no longer just storing priors; it now has its first real bounded consumer in the staged pipeline.
- This replaces another brittle hardcoded actor-vocabulary heuristic with typed reusable memory while preserving the project’s truthfulness rule: priors may guide local interpretation, but they must not author canonical facts on their own.
- It gives the extractor a safe path to improve on PDF `N+1` from validated experience on PDFs `1..N` without letting document pipelines contaminate one another.

### Validation
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_source_column_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml actor_taxonomy_priors_guide_section_heading_direction_inference -- --nocapture` → passed
- `cargo test --manifest-path Cargo.toml converge_defaults_to_ollama_for_vlm_and_nlp -- --nocapture` → passed
- `bash scripts/run_ci.sh` → passed (`202/202` tests)

## 2026-04-06 (typed prior memory now learns actor taxonomy too)

### Added: actor-taxonomy priors for `R15f`
- Extended [prior_memory.rs](crates/specforge/src/ir/prior_memory.rs) with a typed `actor_taxonomy_priors` family plus query helpers by protocol family and taxonomy role.
- Extended [learn_priors.rs](crates/specforge/src/commands/learn_priors.rs) so `specforge learn-priors` now harvests actor-taxonomy priors from:
  - decisive, non-alias-dependent actor-grounded handshake-role evidence
  - conservative self-identifying actor vocabulary such as `requester`, `completer`, `manager`, and `subordinate`

### Why this matters
- The cross-document learning plane can now accumulate reusable protocol-role vocabulary, not just timing language.
- This is the first prior family that directly teaches the extractor how actor terminology varies across specs while still keeping canonical per-document truth local and validated.
- The latest live AMBA prior-memory run is now materially richer: `16` actor-taxonomy priors and `222` temporal phrase priors, while semantic phrase priors remain honestly at `0`.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` → passed (`16` actor-taxonomy priors, `222` temporal phrase priors)
- `bash scripts/run_ci.sh` → passed (`199/199` tests)

## 2026-04-06 (local and hosted CI now share one entrypoint)

### Added: checked-in local CI runner
- Added [run_ci.sh](scripts/run_ci.sh), a repository-local CI entrypoint that runs the canonical Rust quality gate from the repo root:
  - `cargo fmt --all --check`
  - `cargo test --manifest-path Cargo.toml`

### Changed: GitHub Actions now reuses the local runner
- Updated [.github/workflows/ci.yml](.github/workflows/ci.yml) so GitHub Actions calls `./scripts/run_ci.sh` instead of duplicating the commands inline.

### Why this matters
- The full Rust CI path can now be run locally before push, which makes CI breakage easier to catch on the developer machine instead of waiting for GitHub.
- Using one checked-in entrypoint removes local-versus-hosted drift and makes future CI expansion safer.

### Validation
- `./scripts/run_ci.sh` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (GitHub Actions CI baseline established)

### Added: repo-hosted Rust CI on `push` / `pull_request`
- Added [.github/workflows/ci.yml](.github/workflows/ci.yml), a GitHub Actions workflow that installs Rust `1.89.0`, caches Cargo artifacts, and runs the same baseline Rust gate used locally:
  - `cargo fmt --all --check`
  - `cargo test --manifest-path Cargo.toml`

### Why this matters
- The project now has a real hosted validation path instead of relying only on local discipline before commits and pushes.
- The CI contract stays intentionally narrow and trustworthy by mirroring the commands already used as the canonical local gate.
- This also closes a repository-bootstrap gap: the new GitHub repo now validates Rust changes automatically on every push and pull request.

### Validation
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (first typed cross-document prior store landed)

### Added: `specforge learn-priors <intent_ir>...`
- Added a new CLI command that builds the first local typed `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`.
- The command only learns from validated `IntentIR` artifacts and skips artifacts whose latest validation report carries error findings, so the new learning plane stays downstream of validation instead of becoming a shortcut around it.

### Added: first typed prior families for `R15f`
- Added `crates/specforge/src/ir/prior_memory.rs` with a typed `CorpusMemory` schema, explicit update-policy record, protocol-family scoping, and advisory query helpers.
- The first semantic prior family learns reusable semantic-role phrases only from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text.
- The first temporal prior family learns reusable timing/constraint language from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`, which makes the learning plane immediately useful even while real-document temporal-rule lift remains conservative.

### Why this matters
- This is the first real implementation of the separate cross-document learning plane captured in the roadmap and development notes.
- It keeps the document plane provenance-pure while finally giving the extractor a place to accumulate reusable knowledge about how chip specifications express meaning.
- The first live AMBA run is already informative: AXI/APB/AHB `IntentIR` artifacts currently yield `222` temporal phrase priors and `0` semantic phrase priors, which is exactly the kind of honest signal the project needs while the canonical semantic-consensus surface on real PDFs is still strengthening.

### Validation
- `cargo test --manifest-path Cargo.toml learn_priors -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json --output generated/prior_memory/corpus_memory.json` → passed (`222` temporal phrase priors)

## 2026-04-06 (KG fixtures now lock AHB wait-state timing recovery)

### Added: AHB-style wait-state timing gold fixture
- Added a tracked staged fixture proving that AHB-style `Manager signals` / `Subordinate signals` section-heading context, `Destination`-column signal tables, and explicit actor relations can recover wait-state timing semantics in addition to the earlier section-heading direction path.
- The fixture locks `HREADY must be asserted on the next cycle when HSEL is HIGH` together with waited-transfer hold rules on `HTRANS` and `HADDR`, and expects canonical actor-relative ports, one bounded `cycle_window`, multi-predicate temporal guards, actor-grounded temporal predicates, and no false handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The earlier AHB gold fixture proved that family-specific section headings can recover truthful per-signal direction. This follow-on slice proves the same AHB evidence path can also recover real wait-state timing without falling back to generic protocol heuristics.
- It closes an important family gap between “AHB direction works” and “AHB wait-state timing works,” which is necessary if the KG benchmark suite is going to be honest about protocol behavior rather than only port orientation.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_wait_state_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock APB setup/access timing recovery)

### Added: APB-style setup/access timing gold fixture
- Added a tracked staged fixture proving that APB-style `Signal | Source | Width | Description` tables plus guarded constraints can recover setup/access timing semantics in addition to the earlier requester/completer handshake path.
- The fixture locks `PENABLE must be asserted on the next cycle when PSEL is HIGH` together with wait-state and completion hold rules on `PADDR`, and expects canonical actor-relative ports, one bounded `cycle_window`, multi-predicate temporal guards, actor-grounded temporal predicates, and typed handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The earlier APB gold fixture proved that requester/completer roles plus one guarded constraint can recover handshake completion. This follow-on slice proves the same APB vocabulary can also recover setup-to-access timing and wait-state stability without losing actor grounding or collapsing guarded temporal structure.
- It closes an important protocol-family gap between “APB handshake meaning works” and “APB access timing works,” which is necessary if the KG benchmark suite is going to be honest about behavioral protocol semantics rather than just request/accept roles.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_setup_access_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all --check` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AXI next-cycle timing recovery)

### Added: AXI-style next-cycle timing gold fixture
- Added a tracked staged fixture proving that AXI-style width-only channel tables plus prose `Manager` / `Subordinate` drive-sample relations can recover next-cycle timing semantics in addition to direction and signal inventory.
- The fixture locks `AWREADY must be asserted on the next cycle` together with `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH`, and expects canonical actor-relative ports, one bounded `cycle_window`, actor-grounded temporal predicates, and typed handshake completion to survive through both `SemanticIR` and `IntentIR`.

### Why this matters
- The first AXI gold fixture proved that width-only tables plus prose can recover truthful actor-relative direction. This follow-on slice proves the same family of evidence can also recover temporal meaning instead of stopping at static ports.
- It closes an important roadmap gap between “AXI direction works” and “AXI timing works,” which is necessary if the KG benchmark suite is going to be honest about protocol semantics rather than only signal inventory.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_next_cycle_timing_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AXI width-only prose-direction recovery)

### Added: AXI-style width-only plus prose-direction gold fixture
- Added a tracked staged fixture proving that AXI-style `Name | Width | Description` signal tables still recover truthful actor-relative ports when prose drive/sample relations provide the missing directionality.
- The fixture locks AXI write-address-channel recovery end-to-end: table-grounded widths for `AWVALID`, `AWREADY`, and `AWADDR`; prose-grounded `Manager` / `Subordinate` `Drives` and `Reads` relations; request/accept semantic grounding; and typed handshake completion from one guarded `AWADDR must not change when AWVALID is HIGH and AWREADY is HIGH` constraint.

### Fixed: width-only synthesized declarations now survive into canonical signal records
- Widened the semantic explicit-signal parser so synthesized statements like `Signal AWVALID is width 1.` are treated as real interface-signal declarations even without an immediate `input` / `output` token.
- This closes the AXI-family gap where width-only channel tables previously stopped at actor relations and connectivity instead of becoming canonical `SemanticIR` / `IntentIR` signal records.

### Why this matters
- AXI-family specs are a real stress case because the signal tables often omit direction columns entirely. If that mixed table-plus-prose recovery path regresses, the generic AMBA/APB/AHB fixture suite can still look healthy while AXI truth quietly drifts.
- This turns that family-specific extraction pattern into executable benchmark coverage instead of leaving it protected only by aggregate PDF scores.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality axi_width_only_prose_direction_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock APB Requester/Completer semantics)

### Added: APB-style `Requester` / `Completer` gold fixture
- Added a tracked staged fixture proving that APB-style `Source`-column signal tables recover `Requester` / `Completer` actor roles canonically instead of only being covered indirectly by broader AMBA fixtures.
- The fixture locks `(Requester, drives, PSEL)` and `(Completer, drives, PREADY)`, the corresponding actor-relative output ports, table-grounded request/accept semantics, and a typed handshake-completion temporal rule from one guarded `PADDR must not change when PSEL is HIGH and PREADY is HIGH` constraint.

### Why this matters
- APB-family specs use `Completer` as real protocol vocabulary. If that family-specific role word regresses, the broader AMBA fixture set can still look healthy while APB truth quietly degrades.
- This turns APB-specific table vocabulary into executable benchmark coverage rather than assuming it is already protected by the generic `Requester` / `Subordinate` path.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality apb_requester_completer_handshake_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → passed

## 2026-04-06 (KG fixtures now lock AHB section-heading direction recovery)

### Added: AHB-style section-heading gold fixture
- Added a tracked staged fixture proving that `Manager signals` / `Subordinate signals` section context recovers per-signal direction and width correctly for AHB-style signal tables.
- The fixture locks canonical direction on `HADDR`, `HWRITE`, `HTRANS`, `HREADYOUT`, and `HRESP` through both `SemanticIR` and `IntentIR`.

### Improved: `kg-bench` can now patch `document_sections` and assert per-signal direction directly
- Added fixture support for patching `SourceIR.document_sections`, so section-heading-driven extraction paths are benchmarkable without needing heavyweight source documents.
- Added canonical per-signal direction expectations, so tracked fixtures can lock actual signal direction instead of inferring it through aggregate validation metrics.

### Why this matters
- AHB extraction quality genuinely depends on section-heading context in some real specs. If that path regresses, the pipeline can still look healthy at a coarse metric level while silently losing signal truth.
- This turns that protocol-family-specific path into executable benchmark coverage instead of leaving it as a fragile unit-test-only behavior.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality ahb_section_heading_direction_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock AMBA destination-column receiver semantics)

### Added: AMBA-style `Destination`-column gold fixture
- Added a tracked staged fixture proving that AMBA-style `Destination` signal-description tables recover consumer-side actor-signal relations canonically as `Reads`.
- The fixture locks that `Subordinate` reads `XREQ` and `Requester` reads `XRESP`, and that those same relations survive downstream as actor-relative input ports in both `SemanticIR` and `IntentIR`.

### Improved: `kg-bench` can now assert canonical actor-signal relations directly
- Added canonical fixture expectations for actor-signal relations, so tracked truthfulness checks can lock `Drives` versus `Reads` semantics directly.
- This makes protocol-grade relation benchmarks stronger than relying only on actor-port projections or relation counts.

### Why this matters
- `Destination`-oriented AMBA tables carry receiver semantics, not producer semantics. If the KG flattens those into output-side relations, downstream truthfulness quietly drifts.
- Locking the relation itself, not just derived port shape, makes the benchmark harness more faithful to the graph-first architecture.

### Validation
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality amba_destination_column_reads_gold` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock spurious timing-annotation rejection)

### Added: spurious timing-annotation negative fixture
- Added a tracked staged fixture proving that low-value VLM timing-diagram labels like `T0`, `Addr 1`, and `Cycle 2` remain visible as timing-diagram extraction at the evidence stage but do not synthesize canonical timing constraints or temporal rules downstream.
- The fixture locks `timing_diagram_extractions = 1` together with `timing_constraints = 0` and `temporal_rules = 0`, so the pipeline keeps the observation without overclaiming semantics.

### Fixed: semantic timing lift now rejects label-only waveform noise
- Added a narrow semantic-stage filter so label-only VLM timing annotations are treated as waveform labels instead of timing semantics.
- This keeps the `EvidenceIR` timing observation honest while preventing `SemanticIR` / `IntentIR` from fabricating timing meaning from low-value annotation fragments alone.

### Why this matters
- Chip-spec timing diagrams often contain a mix of true behavioral annotations and low-value figure labels. The KG should learn from the former without hallucinating meaning from the latter.
- This closes a real false-positive path in the multimodal timing lift and makes that truthfulness boundary executable in the tracked benchmark suite.

### Validation
- `cargo test --manifest-path Cargo.toml vlm_timing_diagram_observation_rejects_label_only_noise -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality vlm_timing_spurious_annotation_negative` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 192/192 passed

## 2026-04-06 (KG fixtures now lock field-table misclassification rejection)

### Added: field-table misclassification negative fixture
- Added a tracked staged fixture proving that a misclassified `Bits | Name | Description` table does not synthesize fake top-level signals or semantic roles from field names like `REQ` and `ACK`.
- The fixture locks that only the real declared top-level signal survives in the canonical semantic/intent surface, while evidence-stage semantic hints stay at zero.

### Fixed: table-driven top-level signal synthesis now rejects field-like layouts
- Added a shared table-level sanity gate so field-like `Bits | Name | Description` layouts and `... signal fields` captions are filtered before they can generate fake signal declarations, semantic hints, or related top-level table-derived facts.
- This guard now protects the table-driven name/semantic paths together instead of relying on one-off downstream cleanup.

### Why this matters
- This closes another KG false-positive path: register-field tables and bit-field tables often contain uppercase names that look like signals, but they are not top-level interface ports.
- It also makes table misclassification a benchmarked truthfulness property instead of an implicit hope in the upstream classifier.

### Validation
- `cargo test --manifest-path Cargo.toml misclassified_field_table_does_not_synthesize_fake_signal_semantics -- --nocapture` → passed
- `cargo run --manifest-path Cargo.toml -- kg-bench --fixtures-root crates/specforge/test_data/kg_quality table_misclassification_field_table_negative` → passed
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 191/191 passed

## 2026-04-06 (KG fixtures now lock bogus source-column actor rejection)

### Added: bogus actor-attribution negative fixture
- Added a tracked staged fixture proving that AMBA-style `Source`-column infrastructure rows like `Clock` and `Reset` do not become protocol actors in the KG.
- The fixture locks that only the true `Requester` / `Subordinate` rows survive as actor-signal relations and actor-relative ports while table-grounded semantic request/accept meaning still remains recoverable.

### Fixed: table relation extraction now distinguishes source vs destination semantics
- `Source` / `Driver` columns now yield `Drives` relations.
- `Destination` columns now yield `Reads` relations.
- Direction and infrastructure placeholders like `input`, `Clock`, and `Reset` are filtered instead of being promoted into fake actor names.

### Why this matters
- This closes a real KG false-positive path: chip-spec signal tables often mix protocol rows with clock/reset/infrastructure rows, and the graph should not silently invent actors from those metadata labels.
- It also hardens a subtle semantics boundary: destination-oriented tables are receiver facts, not disguised driver facts.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 190/190 passed

## 2026-04-06 (KG fixtures now lock multimodal conflict behavior too)

### Added: cross-modality semantic-conflict negative fixture
- Added a tracked staged fixture proving that table evidence and visual-caption evidence can disagree about the same signal role without collapsing into false cross-modality consensus.
- The fixture locks that `XCTRL`:
  - carries a semantic conflict
  - keeps multiple candidates and non-decisive arbitration
  - does not gain a resolved role or consensus

### Added: validation-metric expectations for contested multimodal grounding
- The new fixture also locks an important validator nuance:
  - `with_visual_semantic_grounding` stays non-zero because visual evidence is still present
  - `with_cross_modality_semantic_grounding` stays zero because the multimodal evidence never resolved into consensus

### Why this matters
- This protects against a subtle multimodal failure mode: “two modalities spoke” must not be mistaken for “two modalities agreed.”
- The benchmark harness now guards both the positive and negative sides of multimodal grounding quality.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures now lock cross-modality grounding metrics)

### Added: validation-metric expectations in `specforge kg-bench`
- KG fixtures can now assert persisted validation metric values directly, not only finding ids.
- This lets the tracked harness lock quantitative truthfulness surfaces like:
  - `with_cross_modality_semantic_grounding`
  - `with_visual_semantic_grounding`
  - decisive semantic-arbitration counts

### Added: visual-asset patching in `SourceIR` fixtures
- `specforge kg-bench` fixtures can now patch `SourceIR.visual_assets` directly in addition to structured tables.
- That makes tracked multimodal regressions practical without needing a heavyweight external PDF for each case.

### Added: cross-modality semantic-grounding gold fixture
- Added a tracked staged fixture proving that `XREQ` can become valid-like through joint signal-description-table evidence plus visual-caption evidence.
- The fixture locks both canonical outcomes and validator metrics, proving the resulting role is:
  - resolved
  - decisive
  - cross-modally grounded
  - visually grounded

### Why this matters
- This extends `R15e` from canonical-shape assertions into quantitative grounding checks.
- The benchmark harness can now protect multimodal semantic-strength behavior directly instead of leaving it to crate-local unit tests or manual inspection.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (cross-document learning plane captured in steering docs)

### Added: explicit roadmap target for cross-document extractor learning
- Logged the architecture for a separate cross-document learning plane that can improve extraction on PDF `N+1` using reusable priors learned from PDFs `1..N`.
- Made the safety boundary explicit:
  - per-document `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` truth stays local and provenance-pure
  - cross-document memory learns extraction priors, not undocumented facts

### Added: detailed engineering note for prior-guided extraction
- Captured the full doctrine in `DEVELOPMENT_NOTES.md`, including:
  - the two-plane architecture
  - examples of good priors versus bad fact leakage
  - candidate memory shapes like `CorpusMemory`, `PriorGraph`, and `ExperienceIR`
  - the retrieval / grounding / validation / feedback loop
  - the rule that only validated/promoted outcomes should feed the learning plane

### Why this matters
- This is the clean path to making the extractor progressively more expert across many chip-spec PDFs without breaking the truthfulness contract of the canonical IR.
- It steers future implementation toward learning reusable extraction intelligence rather than contaminating document-local truth.

## 2026-04-06 (KG fixtures now lock semantic arbitration state directly)

### Added: canonical semantic-arbitration expectations in `specforge kg-bench`
- KG fixtures can now assert:
  - signals with any semantic candidates
  - signals with multiple semantic candidates
  - signals carrying semantic arbitration
  - signals with decisive semantic arbitration
  - signals with non-decisive semantic arbitration

### Added: direct arbitration checks to the staged handshake fixtures
- The contested handshake fallback fixture now proves that `XVALID` stays canonically contested while `XACK` stays decisively grounded.
- The alias-dependent handshake caveat fixture now proves that `XREQ` and `XACK` stay decisively grounded even though their accepted handshake meaning remains explicitly caveated as alias-dependent.

### Why this matters
- This upgrades `R15e` from benchmarking arbitration side effects to benchmarking arbitration state directly.
- The harness now checks the canonical truth model itself instead of inferring arbitration quality only from blocked heuristic fallback, residual packets, or validator findings.

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures now cover alias-dependent semantic caveats)

### Added: richer canonical expectations in `specforge kg-bench`
- KG fixtures can now assert:
  - alias-dependent semantic consensus by signal
  - alias-dependent semantic candidates by signal
  - alias-dependent handshake-completion counts
- `EvidenceIR` fixture patches can now seed alias maps and refresh semantic hints before downstream stages run.

### Added: alias-dependent handshake-completion caveat fixture
- Added a stage-patched tracked fixture proving that alias-grounded handshake recovery remains canonical only when its weaker grounding stays explicit through:
  - semantic residual decisions
  - intent assumptions
  - validator findings

### Why this matters
- This extends `R15e` from benchmarking only hard rejection cases to also benchmarking “useful but caveated” semantic recovery.
- The harness now protects both sides of the truthfulness contract:
  - unsafe heuristic promotion must stay blocked
  - weak but acceptable semantics must retain their caveat trail

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG fixtures can now patch staged inputs)

### Added: stage-patched KG fixtures
- `specforge kg-bench` fixtures can now patch `SourceIR` and `EvidenceIR` inputs directly before downstream stages run.
- This lets the tracked benchmark harness express richer structured/semantic conditions than plain markdown prose alone.

### Added: contested handshake-name fallback negative fixture
- Added a stage-patched tracked fixture proving that contested meaning for a handshake-shaped signal like `XVALID` blocks typed `HandshakeComplete` recovery.
- The fixture injects:
  - a structured signal-description table
  - a typed signal constraint guard
  - validation expectations showing the blocked fallback and preserved semantic-role conflict

### Why this matters
- This strengthens `R15e` from “benchmark simple markdown cases” into “benchmark real staged semantics.”
- The harness now protects a high-value truthfulness invariant:
  - contested semantic evidence must outrank handshake-name heuristics

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (KG benchmark now includes a first AMBA-style gold fixture)

### Added: representative AMBA-style handshake gold fixture
- Added `crates/specforge/test_data/kg_quality/amba_source_column_handshake_gold/`.
- The fixture patches an AMBA-style `Signal | Source | Width | Description` table with `Requester` / `Subordinate` source roles and one guarded `PAYLOAD must not change when XREQ is HIGH and XACK is HIGH` constraint.
- It proves:
  - `EvidenceIR` recovers `actor_signal_relations = 2`
  - `SemanticIR` / `IntentIR` recover driver-side actor ports for `Requester -> XREQ` and `Subordinate -> XACK`
  - table-grounded semantic role consensus survives for both handshake signals
  - the guarded constraint lifts into one typed temporal rule with `HandshakeComplete`

### Why this matters
- This is the first tracked benchmark step from seed synthetic truthfulness checks toward representative APB/AHB/AXI-style gold coverage.
- It locks an important real-doc pattern: AMBA-style `Source` columns can now be benchmarked end-to-end instead of only being covered by ad hoc unit tests.

## 2026-04-06 (KG benchmark now locks caption-vs-VLM visual semantic conflicts)

### Added: visual-source semantic-conflict negative fixture
- Added `crates/specforge/test_data/kg_quality/visual_sources_semantic_conflict_negative/`.
- The fixture patches one timing-diagram visual asset with:
  - a caption that implies valid-like meaning
  - a `vlm_timing_diagram_extraction` note that implies ready-like meaning
- It proves:
  - `EvidenceIR` reports one visual-caption semantic hint and one VLM timing-annotation semantic hint
  - downstream `SemanticIR` / `IntentIR` preserve a semantic conflict, multiple candidates, and non-decisive arbitration
  - `with_visual_semantic_grounding = 1` while `with_multi_source_semantic_grounding = 0`

### Why this matters
- The benchmark harness now locks an important same-asset arbitration nuance: two conflicting visual sub-sources must stay visibly grounded without being overpromoted into same-modality consensus.

## 2026-04-06 (KG benchmark now locks VLM-note semantic-noise rejection)

### Added: direct VLM timing-note semantic-noise negative fixture
- Added `crates/specforge/test_data/kg_quality/vlm_timing_name_only_semantic_noise_negative/`.
- The fixture patches a timing-diagram `VisualAsset.note` with `vlm_timing_diagram_extraction` content that only describes waveform motion around `XVALID`.
- It proves:
  - `EvidenceIR` reports `timing_diagram_extractions = 1`
  - `EvidenceIR` reports `signal_semantic_hints_from_vlm_timing_annotations = 0`
  - downstream `SemanticIR` / `IntentIR` keep semantic-role candidates, arbitration, and consensus at zero

### Why this matters
- The benchmark harness now locks both sides of direct VLM-note truthfulness:
  - real timing extraction should survive
  - semantic-role meaning must not leak from handshake-shaped signal spelling alone

## 2026-04-06 (KG benchmark now locks direct VLM-note semantics)

### Added: evidence-stage validation expectations in `specforge kg-bench`
- `specforge kg-bench` fixtures can now assert persisted validation metrics at the `EvidenceIR` stage, not only at `SemanticIR` and `IntentIR`.
- This lets tracked regressions prove exactly where a semantic hint came from when downstream visual-grounding metrics would be too coarse.

### Added: direct VLM timing-note semantic-grounding gold fixture
- Added `crates/specforge/test_data/kg_quality/vlm_timing_semantic_grounding_gold/`.
- The fixture patches a timing-diagram `VisualAsset.note` with `vlm_timing_diagram_extraction` content and proves:
  - `EvidenceIR` reports `signal_semantic_hints_from_vlm_timing_annotations = 1`
  - `EvidenceIR` reports `signal_semantic_hints_from_visual_captions = 0`
  - downstream `SemanticIR` / `IntentIR` still resolve the signal meaning with visual grounding

### Why this matters
- The benchmark harness now locks direct image-note-derived meaning explicitly instead of only checking downstream visual-grounding side effects.
- This closes an important quality gap in `R15e`: tracked fixture coverage now reaches caption grounding, caption-versus-table arbitration, and direct VLM timing-note grounding.

## 2026-04-06 (tracked KG-quality benchmark harness landed)

### Added: `specforge kg-bench` command
- Added a new `specforge kg-bench` CLI command that runs tracked KG-quality fixtures through the staged `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` pipeline.
- The command can assert canonical IR expectations and persisted validation findings, and it fails the run when any gold or negative fixture drifts.

### Added: first tracked KG-quality fixture pack
- Added tracked fixtures under `crates/specforge/test_data/kg_quality/` for:
  - actor-relative port recovery
  - rejection of name-only semantic role noise
  - multi-producer structural conflict surfacing through validation
  - actor-boundary residual quality

### Why this matters
- This starts `R15e` as a real executable benchmark surface instead of leaving KG-quality evaluation as roadmap text or scalar scores alone.
- The benchmark harness now protects:
  - canonical graph truthfulness
  - false-positive control
  - conflict surfacing
  - residual-quality honesty

### Validation
- `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` → passed
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 188/188 passed

## 2026-04-06 (polarity conflicts now survive into canonical IR)

### Added: canonical carry-through for polarity disagreement
- `SemanticIR` now carries `signal_polarity_conflicts` forward from `EvidenceIR`.
- `IntentIR` now carries the same polarity-conflict surface forward from `SemanticIR`.

### Added: semantic and intent validation for polarity conflicts
- `specforge validate` now reports `signal_polarity_conflicts` for both `SemanticIR` and `IntentIR`, not only for `EvidenceIR`.
- Contradictory active-high/active-low evidence now stays visible all the way to the canonical artifacts instead of disappearing after the evidence stage.

### Added: regression coverage for polarity-conflict carry-through
- Added semantic-stage and intent-stage carry-through regressions for `signal_polarity_conflicts`.
- Added validator regressions proving polarity conflicts are flagged at both canonical stages.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 186/186 passed

## 2026-04-05 (alias-dependent handshake completion is now canonical residual state)

### Added: canonical residual and assumption carry-through for alias-dependent handshake semantics
- `SemanticIR` now emits `semantic_alias_dependent_handshake_completion` when typed `HandshakeComplete` predicates still depend on alias-grounded semantic role consensus.
- `IntentIR` now carries that caution forward as `assumption_alias_dependent_handshake_completion`, so the weaker grounding remains inspectable even before validation runs.

### Why this matters
- Validator findings are useful, but SOTA-quality continuity needs the canonical artifacts themselves to preserve important semantic caveats.
- Alias-grounded transfer-progress structure remains usable, while still being marked as weaker than directly grounded or corroborated handshake semantics.

### Added: regression coverage for canonical alias-dependent handshake caveats
- Added semantic-stage coverage proving alias-grounded handshake completion emits a residual packet.
- Added intent-stage coverage proving that residual packet becomes an explicit canonical assumption.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 182/182 passed

## 2026-04-05 (alias-dependent handshake completion is now explicit in validation)

### Added: validation visibility for alias-dependent temporal handshake semantics
- `specforge validate` now reports:
  - `temporal_rules_with_alias_dependent_handshake_completion`
- It also emits an explicit finding when typed `HandshakeComplete` predicates depend on alias-dependent semantic role consensus.

### Why this matters
- Alias-grounded handshake recovery remains useful, but it no longer looks as grounded as directly supported handshake semantics.
- The temporal layer now makes that weaker grounding visible instead of blending it into the generic handshake-completion count.

### Added: regression coverage for alias-dependent handshake validation
- Added a validation regression proving that alias-grounded semantic role consensus feeding a typed handshake predicate is reported explicitly.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 181/181 passed

## 2026-04-05 (alias-dependent semantic role meaning is now explicit)

### Added: explicit alias-dependence on semantic candidates and consensus summaries
- `SemanticIR` and `IntentIR` now mark semantic role candidates and consensus summaries as `alias_dependent` when the current meaning still depends only on alias-grounded evidence.
- This keeps alias-grounded meaning usable while making that dependency explicit in the canonical IR instead of hiding it inside source-kind lists.

### Added: validation reporting for alias-dependent resolved roles
- `specforge validate` now reports:
  - `with_alias_dependent_semantic_consensus`
  - `alias_dependent_semantic_candidates`
- It also emits an explicit finding when resolved semantic roles still depend only on alias-grounded evidence.

### Added: regression coverage for alias-dependent canonical role visibility
- Added assertions proving:
  - alias-grounded semantic consensus is marked `alias_dependent`
  - direct visual/table grounded semantic consensus is not marked `alias_dependent`
  - intent validation reports alias-dependent resolved roles explicitly

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 180/180 passed

## 2026-04-05 (explicit signal names now outrank aliases for semantic grounding)

### Changed: alias-grounded semantic hints no longer double-count when direct signal names are present
- `EvidenceIR` now suppresses alias-grounded targeting for a signal when the same prose sentence or visual caption already contains an explicit mention of that signal.
- This means aliases stay a rescue path for implicit references, not an extra vote when the document is already explicit.

### Why this matters
- A sentence like `The request phase XREQ indicates that address and control information are valid for transfer.` now produces exactly one semantic hint for `XREQ`, grounded as direct prose rather than both direct and alias-grounded support.
- That keeps semantic-role arbitration honest and prevents artificial support inflation.

### Added: regression coverage for direct-name precedence over aliases
- Added an evidence-stage regression proving that explicit signal mentions outrank alias-grounding for the same statement.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 179/179 passed

## 2026-04-05 (multi-signal prose and captions now ground per-signal semantic roles)

### Changed: semantic-role hint extraction now decomposes multi-signal text into per-signal context windows
- `EvidenceIR` no longer requires a whole prose statement or visual caption to resolve to exactly one signal before it can contribute a semantic role hint.
- When multiple known signals appear in the same sentence or caption, the extractor now carves out clause-local context windows around each signal mention and infers role meaning from that local description.

### Why this matters
- Text like `XVALID indicates request pending and XREADY indicates the subordinate can accept the transfer` can now produce:
  - a valid-like hint for `XVALID`
  - a ready-like hint for `XREADY`
- The previous weaker behavior either dropped that region entirely or would have required unsafe whole-text attribution.

### Added: regression coverage for multi-signal prose and caption grounding
- Added evidence-stage regressions proving that:
  - one prose sentence can contribute different semantic role hints to different signals
  - one visual caption can contribute different semantic role hints to different signals

### Cleanup
- removed the dead single-target semantic-role helper after the stronger per-signal path replaced it

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 178/178 passed

## 2026-04-05 (signal names no longer self-justify semantic role hints)

### Changed: semantic-role hint inference now strips signal identifiers from prose/visual text
- `EvidenceIR` now removes explicit signal tokens before semantic-role tag inference on prose descriptions, alias-grounded prose, visual captions, VLM timing annotations, and signal-description row text.
- This means identifiers like `AWVALID` / `AWREADY` no longer create valid-like or ready-like consensus by themselves.

### Added: regression coverage for declaration-only handshake-shaped names
- Added an evidence-stage regression proving that plain declarations such as `Signal AWVALID is input width 1.` do not create semantic handshake hints without descriptive language.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 176/176 passed

## 2026-04-05 (provisional semantic roles no longer drive typed handshake recovery)

### Changed: handshake-role recovery now requires observation-backed consensus
- `SemanticIR` no longer lets fallback-only resolved semantic roles populate the canonical handshake-role context by themselves.
- Typed handshake-role recovery now trusts observation-backed `semantic_consensus` instead of any resolved role value that still lacks preserved grounding.

### Changed: provisional fallback-only role state now blocks handshake name fallback too
- Handshake-shaped signals with fallback-only provisional roles now block literal `VALID` / `READY` name fallback, not only signals with contested semantic arbitration.
- `specforge validate` now reports those blocked fallback cases under the existing handshake-fallback metric/finding surface.

### Added: regression coverage for provisional-role handshake blocking
- Added assertions proving:
  - provisional fallback-only semantic roles do not populate canonical handshake-role context
  - handshake-shaped provisional-role signals are reported as blocked name-fallback cases in validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 175/175 passed

## 2026-04-05 (fallback-only semantic roles are now explicit provisional state)

### Added: residual surfacing for resolved semantic roles without consensus
- `SemanticIR` now emits a `semantic_resolved_role_without_consensus` residual decision when a signal still carries a resolved semantic role but lacks preserved observation-backed consensus.

### Added: explicit IntentIR assumption for provisional semantic meaning
- `IntentIR` now turns that carried residual into `assumption_semantic_role_without_consensus` so provisional role meaning stays visible in the canonical artifact.

### Added: regression coverage for provisional semantic-role visibility
- Added assertions proving:
  - the semantic residual packet is emitted for fallback-only resolved roles
  - the intent stage carries that packet forward
  - `IntentIR` emits the matching provisional-role assumption

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 173/173 passed

## 2026-04-05 (blocked handshake fallback is now explicit residual state)

### Added: residual surfacing for intentionally blocked handshake promotion
- `SemanticIR` now emits a `semantic_handshake_name_fallback_blocked` residual decision when a signal looks handshake-shaped by name but preserved semantic arbitration is still contested.
- `IntentIR` carries that residual packet forward unchanged.

### Added: validation visibility for blocked handshake fallback
- `specforge validate` now reports:
  - `with_blocked_handshake_name_fallback`
- It also emits explicit semantic and intent findings when handshake-shaped signals intentionally block literal `VALID` / `READY` fallback.

### Added: regression coverage for blocked fallback visibility
- Added assertions proving:
  - the semantic residual packet is emitted
  - the intent stage carries it forward
  - semantic and intent validation both report the blocked-fallback state

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 171/171 passed


## 2026-04-05 (contested semantic evidence outranks handshake-name heuristics)

### Changed: temporal handshake derivation now respects contested semantic arbitration
- `SemanticIR` now builds a richer handshake-role context instead of relying only on a resolved-role map plus raw signal-name fallback.
- Signals with non-decisive `semantic_arbitration` now block literal `VALID` / `READY` name fallback during typed `HandshakeComplete` derivation.

### Why this matters
- A signal like `XVALID` can now stay honestly unresolved when preserved evidence disagrees about whether it is valid-like or ready-like.
- Literal spelling no longer overrides explicit contested semantic evidence in the temporal layer.

### Added: regression coverage for blocked handshake-name fallback
- Added a semantic regression proving that contested role evidence suppresses typed `HandshakeComplete` derivation even when the signal name looks handshake-shaped.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 168/168 passed


## 2026-04-05 (canonical semantic arbitration summaries)

### Added: explicit semantic arbitration summaries on canonical interface signals
- `SemanticIR` now carries `semantic_arbitration` on `InterfaceSignalRecord`.
- `IntentIR` carries that same semantic-arbitration surface forward unchanged.

### Added: lead-vs-runner-up visibility without unsafe role forcing
- Each arbitration summary currently records:
  - candidate count
  - leading role
  - leading evidence weight
  - runner-up role and evidence weight when present
  - lead margin over the runner-up
  - decisive vs non-decisive status
- Multiple candidates still do not force a resolved semantic role; the arbitration surface is preserved for inspection while the canonical winner remains `None`.

### Added: validation reporting for decisive vs contested semantic roles
- `specforge validate` now reports:
  - `with_semantic_arbitration`
  - `with_decisive_semantic_arbitration`
  - `with_non_decisive_semantic_arbitration`
- It also emits an explicit finding when canonical semantic-role arbitration remains non-decisive.

### Added: regression coverage for semantic arbitration summaries
- Added assertions for:
  - decisive arbitration on single-candidate role meaning
  - non-decisive arbitration on conflicting role meaning
  - arbitration carry-through into `IntentIR`
  - validation metrics and findings for contested semantic arbitration

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 167/167 passed
## 2026-04-04 (canonical semantic candidate arbitration surface)

### Added: explicit semantic candidates on canonical interface signals
- `SemanticIR` now carries `semantic_candidates` on `InterfaceSignalRecord`.
- `IntentIR` carries that same candidate-arbitration surface forward unchanged.

### Added: typed candidate profiles for competing role meanings
- Each semantic candidate now records:
  - role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence
  - deterministic evidence weight

### Changed: resolved semantic roles now build from canonical candidates
- Observation-backed resolved roles and consensus summaries are now built from the canonical candidate layer.
- When multiple role candidates exist, the signal keeps those candidates explicit instead of flattening the situation into only a conflict record.

### Added: validation metrics for canonical semantic arbitration
- `specforge validate` now reports:
  - `semantic_candidates`
  - `with_semantic_candidates`
  - `with_multiple_semantic_candidates`

### Added: regression coverage for canonical semantic candidates
- Added tests for:
  - candidate details on single-source, same-modality multi-source, and cross-modality role meanings
  - conflicting role meanings producing multiple canonical candidates without a resolved role
  - candidate carry-through into `IntentIR`
  - validation counts for multiple semantic candidates

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 167/167 passed

## 2026-04-04 (canonical semantic consensus summaries)

### Added: explicit semantic consensus summaries on canonical interface signals
- `SemanticIR` now carries `semantic_consensus` on `InterfaceSignalRecord` when a resolved role is backed by preserved observations.
- `IntentIR` now carries that same consensus summary forward unchanged.

### Added: semantic consensus profile details
- `semantic_consensus` currently records:
  - winning role
  - grounding strength
  - supporting source kinds
  - supporting observation count
  - strongest supporting automation confidence

### Changed: validation now surfaces fallback-only resolved roles
- `specforge validate` now reports:
  - `with_semantic_consensus`
  - `with_high_confidence_semantic_consensus`
  - `resolved_semantic_roles_without_consensus`
- It also emits an explicit finding when a resolved semantic role still lacks canonical consensus metadata.

### Added: regression coverage for canonical semantic consensus
- Added tests for:
  - consensus details on single-source, same-modality multi-source, and cross-modality semantic grounding
  - consensus carry-through into `IntentIR`
  - validation reporting and findings for resolved roles without consensus

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 166/166 passed

## 2026-04-04 (modality-aware semantic grounding strength)

### Changed: semantic grounding strength now distinguishes cross-modality reinforcement
- `SemanticIR` now derives `semantic_grounding_strength` as:
  - `single_source`
  - `multi_source`
  - `cross_modality`
- `IntentIR` carries that stronger distinction forward unchanged.

### Changed: repeated same-modality evidence no longer overclaims cross-modality support
- Repeated observations from one modality family now stay `multi_source`.
- Support spanning more than one modality family across table/prose/visual evidence now upgrades to `cross_modality`.

### Added: validation metric for cross-modality semantic grounding
- `specforge validate` now reports:
  - `with_cross_modality_semantic_grounding`

### Added: regression coverage for modality-aware role grounding
- Added tests for:
  - cross-modality semantic grounding on interface signals
  - same-modality multi-source semantic grounding on interface signals
  - cross-modality grounding carry-through into `IntentIR`
  - validation counts for both cross-modality and same-modality multi-source grounding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 165/165 passed

## 2026-04-04 (canonical semantic-role consensus from preserved observations)

### Added: resolved semantic-role consensus on canonical interface signals
- `SemanticIR` now resolves `resolved_semantic_role` on `InterfaceSignalRecord` from canonical `semantic_observations` before falling back to merged `semantic_tags`.
- `IntentIR` carries that same canonical resolved-role surface forward unchanged.

### Added: grounding-strength visibility for semantic roles
- `InterfaceSignalRecord` now also carries `semantic_grounding_strength`.
- The canonical layers can now distinguish single-source grounding from multi-source grounding for resolved role meaning.

### Changed: validation now exposes semantic grounding quality directly
- `specforge validate` now reports:
  - `with_resolved_semantic_role`
  - `with_single_source_semantic_grounding`
  - `with_multi_source_semantic_grounding`
- This makes it visible when canonical role meaning is merely present versus reinforced by multiple preserved observations.

### Added: regression coverage for observation-backed role consensus
- Added tests for:
  - single-source resolved semantic roles on interface signals
  - multi-source semantic grounding on interface signals
  - validation counts for multi-source semantic grounding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 162/162 passed

## 2026-04-04 (canonical semantic-role observation carry-through)

### Added: per-signal semantic observations in canonical IR
- `SemanticIR` now carries `semantic_observations` on `InterfaceSignalRecord`.
- `IntentIR` now carries the same role-observation surface forward.
- These observations preserve source kind, source text, and statement/table/visual provenance instead of flattening everything into merged `semantic_tags`.

### Changed: validation now exposes canonical role-grounding depth
- `specforge validate` now reports:
  - `semantic_observations`
  - `with_visual_semantic_grounding`
- This makes it visible when canonical signal meaning is actually grounded in preserved provenance rather than only implied by merged tags.

### Added: regression coverage for canonical observation preservation
- Added tests for:
  - carrying semantic observations into `SemanticIR` interface records
  - carrying semantic observations into `IntentIR`
  - exposing canonical semantic-observation counts in validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 160/160 passed

## 2026-04-04 (initial multimodal semantic-role grounding)

### Added: visual semantic-role hints in `EvidenceIR`
- `EvidenceIR` now mines `signal_semantic_hints` from grounded visual captions and VLM timing-diagram annotations.
- The visual path stays conservative: it only promotes hints when the text implies a role meaning and resolves to exactly one known signal.

### Added: explicit visual provenance for semantic-role hints
- `SignalSemanticHintRecord` now carries `supporting_visual_evidence_ids`.
- This keeps caption/VLM-derived role hints tied to concrete visual evidence instead of degrading into anonymous strings.

### Changed: validation now reports multimodal role-hint sources
- `specforge validate` now emits:
  - `signal_semantic_hints_from_visual_captions`
  - `signal_semantic_hints_from_vlm_timing_annotations`

### Added: end-to-end proof that visual grounding affects semantics
- Added tests for:
  - caption-grounded semantic-role hints
  - VLM timing-annotation-grounded semantic-role hints
  - semantic handshake completion derived from caption-grounded role hints

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 159/159 passed

## 2026-04-04 (semantic-role conflict carry-through into canonical IR)

### Added: carried semantic-role conflicts in `SemanticIR` and `IntentIR`
- `SemanticIR` now carries `signal_semantic_conflicts` forward from `EvidenceIR`.
- `IntentIR` now carries the same explicit role-conflict surface into the canonical endpoint.

### Changed: validation now reports semantic-role disagreement end-to-end
- `specforge validate` now prints and flags `signal_semantic_conflicts` for `SemanticIR` and `IntentIR`, not only for `EvidenceIR`.
- This keeps unresolved role disagreement visible to downstream consumers instead of letting it disappear after the evidence stage.

### Added: regression coverage for canonical conflict carry-through
- Added tests for:
  - carrying semantic-role conflicts into `SemanticIR`
  - carrying semantic-role conflicts into `IntentIR`
  - flagging those conflicts from semantic-stage and intent-stage validation

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 155/155 passed

## 2026-04-04 (explicit semantic-role conflict surfacing)

### Added: typed semantic-role conflicts in `EvidenceIR`
- `EvidenceIR` now persists `signal_semantic_conflicts` when meaning-based role evidence assigns incompatible roles to the same signal.
- This keeps role disagreement explicit instead of leaving it hidden inside a dual-tag ambiguity.

### Changed: validation now reports semantic-role disagreement clearly
- `specforge validate` now prints a dedicated semantic-role-conflict section for `EvidenceIR`.
- Validation now emits a `signal_semantic_conflicts` metric and a warning finding when incompatible role evidence is present.

### Added: regression coverage for semantic-role conflict surfacing
- Added tests for:
  - surfacing a role conflict when one source makes a signal look valid-like and another makes it look ready-like
  - flagging that conflict explicitly in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 151/151 passed

## 2026-04-04 (SourceIR and ingest maturity guidance logged)

### Added: explicit steering on SourceIR maturity and remaining Tier 1 work
- Logged a durable implementation note clarifying that `specforge ingest` and `SourceIR` are different responsibilities:
  - ingest is the stage/command
  - `SourceIR` is the typed artifact/model
- Recorded the current maturity boundary:
  - architecturally strong and high leverage
  - not yet proven universal against arbitrary chip-spec PDFs

### Changed: roadmap now treats remaining Tier 1 work as robustness hardening
- `ROADMAP.md` now says the remaining `SourceIR` / ingest work should be:
  - robustness benchmarking
  - failure-mode detection
  - better fallback behavior
  - stronger source-level validation
- It also makes explicit that broad new Tier 1 expansion should stay secondary unless real PDFs expose a capture bottleneck.

### Validation
- docs-only change; Rust tests were not run

## 2026-04-04 (prose and alias-grounded semantic handshake roles)

### Added: prose and alias-grounded semantic role hints in `EvidenceIR`
- `EvidenceIR` now refreshes `signal_semantic_hints` from direct prose descriptions and alias-grounded prose descriptions, not only from `SignalDescription` tables.
- This means learned aliases can now contribute to typed semantic role grounding instead of only helping constraint reclassification.

### Changed: `nlp-enrich` now refreshes role hints before persistence
- `specforge nlp-enrich` now calls `refresh_signal_semantic_hints()` before writing updated `EvidenceIR`.
- Alias learning and backannotation can therefore feed the same loop-backed semantic-role surface immediately.

### Changed: validation now exposes where semantic role hints came from
- `specforge validate` now reports:
  - `signal_semantic_hints_from_tables`
  - `signal_semantic_hints_from_prose`
  - `signal_semantic_hints_from_alias_grounded_prose`

### Added: regression coverage for prose / alias-grounded role inference
- Added tests for:
  - alias-grounded prose descriptions producing semantic handshake hints in `EvidenceIR`
  - deriving a typed handshake predicate from alias-grounded semantic hints in `SemanticIR`
  - reporting alias-grounded semantic-hint counts in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 149/149 passed

## 2026-04-04 (meaning-grounded handshake roles from signal descriptions)

### Added: typed semantic role hints in `EvidenceIR`
- `EvidenceIR` now persists `signal_semantic_hints` mined from `SignalDescription` table descriptions when the text establishes handshake-like request/valid or accept/ready meaning.
- The new records preserve source text, supporting table ids, and automation confidence instead of collapsing immediately into opaque downstream behavior.

### Changed: handshake detection now prefers grounded meaning before literal naming
- `SemanticIR` now carries per-signal `semantic_tags`, and `IntentIR` preserves the same surface at the canonical endpoint.
- Typed `HandshakeComplete` derivation now consults those meaning-grounded semantic tags before falling back to literal `VALID` / `READY` signal-name heuristics.

### Changed: validation now reports the new meaning-grounded role surface
- `specforge validate` now reports `signal_semantic_hints` for `EvidenceIR`.
- `specforge validate` now reports `with_semantic_tags` for `SemanticIR` and `IntentIR`.

### Added: regression coverage for meaning-grounded handshake-role carry-through
- Added tests for:
  - mining handshake-role semantic hints from signal-description tables in `EvidenceIR`
  - deriving a typed handshake predicate from semantic signal hints in `SemanticIR`
  - carrying signal semantic tags into `IntentIR`
  - reporting the new validation metrics in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 146/146 passed

## 2026-04-04 (semantic-programming doctrine logged)

### Changed: the live roadmap now encodes how semantic intent should be programmed
- Logged a cross-cutting implementation doctrine in `ROADMAP.md`:
  - deterministic extraction where the source is crisp
  - typed protocol-world modeling as the semantic target
  - evidence aggregation and convergence instead of one-shot interpretation
  - bounded AI hypotheses rather than end-to-end black-box AI
  - explicit uncertainty, conflicts, and residual decisions as part of the truthfulness contract

### Added: detailed engineering guidance for programming semantics without full-pipeline AI
- Logged the full steering rationale in `DEVELOPMENT_NOTES.md` as a durable implementation note for future sessions.
- Refreshed `README.md` so the project objective explicitly states that `specforge` is building a typed protocol compiler, not an unrestricted English reader.

### Validation
- docs-only change; no Rust tests were run
## 2026-04-04 (typed ready/valid handshake completion)

### Added: protocol-native handshake predicates in the temporal layer
- `SemanticIR` now derives `TemporalPredicateRecord::HandshakeComplete` when a temporal rule contains grounded asserted `VALID` and `READY` signals in the same phase.
- This keeps ready/valid transfer completion visible as a first-class protocol event instead of only as two separate scalar guard clauses.

### Changed: validation now reports handshake-predicate coverage
- `specforge validate` now reports `temporal_rules_with_handshake_completion` for `SemanticIR` and `IntentIR`.
- This makes handshake-semantic coverage visible in the live validation surface instead of hiding it inside raw temporal-rule counts.

### Added: regression coverage for handshake temporal lift
- Added tests for:
  - deriving a typed handshake predicate from a valid/ready guard in `SemanticIR`
  - carrying that predicate into `IntentIR`
  - reporting handshake-predicate coverage in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 141/141 passed

## 2026-04-04 (idiomatic one-cycle temporal language)

### Added: idiomatic one-cycle latency recovery in the temporal-rule layer
- `SemanticIR` temporal derivation now recognizes common protocol phrases such as `next cycle`, `next clock cycle`, `next tick`, and `next rising edge`.
- `following` and `subsequent` one-cycle variants now also map onto the canonical `CycleWindowRecord` surface instead of being left as unbounded prose.

### Changed: the explicit clock-tick model now covers both numeric and idiomatic latency language
- One-cycle prose no longer needs an explicit numeral like `within 1 cycle` to become a bounded temporal rule.
- This keeps the temporal model aligned with how real chip-design PDFs often describe synchronous behavior.

### Added: regression coverage for idiomatic one-cycle phrases
- Added tests for:
  - direct parser recovery of a single-cycle window from `next cycle`, `next tick`, and `next rising edge`
  - end-to-end temporal-rule derivation from a `next tick` signal constraint in `SemanticIR`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 138/138 passed

## 2026-04-04 (interface-signal conflict surfacing for conflicting declarations)

### Added: typed interface-signal conflicts in the canonical IR layers
- `SemanticIR` now persists `interface_signal_conflicts` when conflicting declarations disagree on a signal's direction or width.
- `IntentIR` now carries the same conflict surface forward so canonical interface-shape disagreement remains explicit downstream.

### Changed: validation now clearly reports interface-shape disagreement
- `specforge validate` now prints a dedicated interface-signal-conflict section for `SemanticIR` and `IntentIR`.
- Validation now emits a warning finding and metric when conflicting direction/width evidence is still unresolved in the canonical interface surface.

### Added: regression coverage for interface-signal conflict surfacing
- Added tests for:
  - deriving direction and width conflicts from contradictory explicit declarations in `SemanticIR`
  - carrying those conflicts into `IntentIR`
  - flagging them in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 136/136 passed

## 2026-04-04 (structural KG conflict surfacing for multi-producer ambiguity)

### Added: typed structural connectivity conflicts in the canonical IR layers
- `SemanticIR` now persists `signal_connectivity_conflicts` when the structural KG resolves more than one producer for the same signal.
- `IntentIR` now carries the same conflict surface forward so unresolved producer ambiguity remains explicit at the canonical endpoint.

### Changed: validation now clearly reports structural producer ambiguity
- `specforge validate` now prints a dedicated signal-connectivity-conflict section for `SemanticIR` and `IntentIR`.
- Validation now emits a warning finding and metric when the structural KG still has unresolved multi-producer ambiguity.

### Added: regression coverage for structural KG conflict surfacing
- Added tests for:
  - deriving a signal-connectivity conflict from two producer claims in `SemanticIR`
  - carrying that conflict into `IntentIR`
  - flagging that carried conflict in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 133/133 passed

## 2026-04-04 (explicit polarity-conflict surfacing in EvidenceIR validation)

### Added: typed polarity-conflict records in `EvidenceIR`
- `EvidenceIR` now persists `signal_polarity_conflicts` when prose and signal-description tables disagree on active-high/active-low semantics for the same anchored signal.
- This keeps contradictory polarity inspectable instead of only letting it disappear into a polarity-neutral derived constraint.

### Changed: validation now clearly reports polarity disagreement
- `specforge validate` now prints a dedicated polarity-conflict section for `EvidenceIR`.
- Validation now emits a warning finding and metric when signal polarity evidence disagrees across sources.

### Added: regression coverage for polarity-conflict reporting
- Added tests for:
  - persisting a polarity conflict while keeping the derived constraint neutral
  - flagging that persisted conflict in `specforge validate`

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 130/130 passed

## 2026-04-04 (signal-table polarity refinement in convergent evidence)

### Added: signal-description tables now contribute polarity facts
- The convergent `EvidenceIR` loop now scans `SignalDescription` tables for active-high/active-low signal facts using known signals as anchors.
- This lets table rows refine asserted/deasserted constraints even when the polarity never appears in prose.

### Changed: polarity merging is now cross-modality and conservative
- Prose polarity and signal-table polarity are now merged before constraint refinement.
- Conflicting polarity across prose and tables cancels the refinement instead of forcing a wrong `MustBeHigh` / `MustBeLow` conversion.

### Added: regression coverage for table-driven polarity refinement
- Added end-to-end tests for:
  - table-driven active-low polarity refinement
  - preserving polarity neutrality when prose and table evidence disagree

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 129/129 passed

## 2026-04-04 (typed temporal conflict records)

### Added: explicit temporal conflict records in the canonical IR layers
- `SemanticIR` now derives `temporal_conflicts: Vec<TemporalConflictRecord>` from contradictory typed temporal value obligations.
- `IntentIR` now carries the same conflict surface forward so disagreement remains explicit downstream.

### Changed: validation now reports and flags typed temporal conflicts
- `specforge validate` now reports `temporal_conflicts` for `SemanticIR` and `IntentIR`.
- Validation now emits a dedicated warning when contradictory temporal value obligations are present in the typed rule set.

### Added: regression coverage for temporal contradiction surfacing
- Added end-to-end tests for:
  - deriving a typed temporal conflict from contradictory value obligations in `SemanticIR`
  - carrying the conflict into `IntentIR`
  - validating that the contradiction is surfaced as a temporal-conflict finding

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 127/127 passed

## 2026-04-04 (compound temporal antecedents in typed temporal rules)

### Added: conjunctive temporal guards now survive as multiple typed antecedents
- `parse_temporal_condition_predicates()` now preserves compound guards like `when HREADY is LOW and HSEL is HIGH` as multiple antecedent predicates when each clause grounds to a known signal.
- This means the typed temporal layer no longer drops half of a conjunctive protocol precondition during semantic lift.

### Changed: validation now reports multi-predicate temporal guard coverage
- `specforge validate` now reports `temporal_rules_with_multi_predicate_antecedents` for `SemanticIR` and `IntentIR`.
- This gives the live validation surface an explicit signal that conjunctive temporal guards are surviving into the canonical IR.

### Added: regression coverage for compound temporal guards
- Added end-to-end tests for:
  - deriving multi-predicate antecedents from a compound temporal guard in `SemanticIR`
  - carrying those antecedents into `IntentIR`
  - validation metrics for multi-predicate temporal antecedents

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 124/124 passed

## 2026-04-04 (actor-grounded stability semantics in temporal rules)

### Added: actor-relative stability predicates in the temporal layer
- `TemporalPredicateRecord` now includes `ActorMaintainsSignalStable`.
- Stable/hold-style temporal consequents now emit actor-grounded stability predicates when the structural KG resolves a unique producer for the signal.
- This means the temporal layer can now express not just that a signal remains stable, but which actor is responsible for maintaining that stability across the tick window.

### Changed: actor-grounding validation now counts actor-grounded stability too
- `specforge validate` now treats `ActorMaintainsSignalStable` as actor-grounded temporal evidence alongside `ActorDrivesSignal` and `ActorSamplesSignal`.

### Added: regression coverage for actor-grounded stability lift
- Added end-to-end tests for:
  - deriving `ActorMaintainsSignalStable` from a stable constraint with a unique producer
  - carrying actor-grounded stability rules into `IntentIR`
  - validation metrics for actor-grounded stability rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 121/121 passed

## 2026-04-04 (actor-grounded temporal drive events)

### Added: actor-relative drive predicates in typed temporal rules
- `TemporalPredicateRecord` now includes `ActorDrivesSignal`.
- Temporal-rule derivation now emits actor-relative drive predicates when the structural KG provides a unique producer for the constrained signal.
- This keeps the temporal layer aligned with the structural graph instead of representing every bounded/value rule as a signal-only event.

### Changed: validation now counts actor-grounded temporal rules
- `specforge validate` now reports `temporal_rules_with_actor_grounding` for `SemanticIR` and `IntentIR`.
- Validation now flags temporal-rule sets that exist alongside a non-empty actor-signal graph but still carry no actor-relative drive/sample grounding at all.

### Added: regression coverage for actor-grounded temporal lift
- Added end-to-end tests for:
  - deriving `ActorDrivesSignal` from a value constraint with a unique producer in the KG
  - carrying actor-grounded temporal rules into `IntentIR`
  - validation metrics for actor-grounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 118/118 passed

## 2026-04-04 (cycle-window recovery in temporal rules)

### Added: bounded latency windows in the temporal-rule layer
- `SemanticIR` temporal-rule derivation now recovers `CycleWindowRecord` bounds from prose such as:
  - `within 2 cycles`
  - `for 2 cycles`
  - `at least 1 cycle`
  - `at most 3 cycles`
  - `between 1 and 3 cycles`
- Timing rows whose unit is already `cycles` now also project their numeric min/max/typ values into `cycle_window`.

### Changed: validation now counts bounded temporal rules
- `specforge validate` now reports `temporal_rules_with_cycle_window` for `SemanticIR` and `IntentIR`.
- Validation now flags when typed temporal rules exist but none of them currently carry explicit cycle-window bounds.

### Added: regression coverage for cycle-window carry-through
- Added tests for:
  - cycle-window derivation from a cycle-bounded signal constraint
  - cycle-window carry-through into `IntentIR`
  - validation metrics for bounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 115/115 passed

## 2026-04-04 (typed temporal-rule surface in SemanticIR / IntentIR)

### Added: initial clock-tick temporal rules in the canonical IR layers
- `SemanticIR` now carries `temporal_rules: Vec<TemporalRuleRecord>` alongside legacy timing/constraint records.
- `IntentIR` now carries the same `temporal_rules` surface forward so downstream consumers can target a typed temporal layer instead of only free-form timing text.
- The first predicate set covers:
  - signal value predicates at explicit tick phases
  - signal stability across `pre_tick -> post_tick`
  - signal sampling on clock edges, with optional actor grounding

### Changed: temporal grounding no longer depends on a full reset contract
- Temporal-rule derivation now reuses an explicit clock declaration even when the spec has not yet surfaced a full `SystemContractRecord`.
- This lets timing/constraint semantics ground to a real clock as soon as `Clock <signal>.` is known, instead of waiting for both clock and reset declarations.

### Changed: validation now reports temporal-rule presence and grounding gaps
- `specforge validate` now reports `temporal_rules` and `temporal_rules_missing_clock_grounding` for `SemanticIR` and `IntentIR`.
- Validation findings now explicitly call out:
  - when typed temporal rules exist but still lack clock/edge grounding
  - when timing/constraint evidence exists but no typed temporal rules were derived

### Added: regression coverage for the new temporal layer
- Added end-to-end tests for:
  - temporal-rule derivation from a conditioned signal constraint plus explicit clock context
  - temporal-rule carry-through from `SemanticIR` into `IntentIR`
  - validation diagnostics for ungrounded temporal rules

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 114/114 passed

## 2026-04-04 (graph-first direction scoring in validation)

### Changed: `specforge validate` now scores direction coverage from the actor-relative graph first
- `validate_semantic_ir()` and `validate_intent_ir()` now treat actor-relative `actor_ports` coverage as the primary signal-direction surface and only fall back to flat `direction_hint` values as a compatibility layer.
- Validation metrics now split direction coverage into:
  - `with_resolved_direction`
  - `with_graph_direction`
  - `with_compat_direction_hint`
- Compatibility lag still surfaces as an informational finding, but flat `direction_hint` absence no longer lowers direction coverage when the actor-relative graph already resolves the signal.

### Added: regression coverage for graph-first validation behavior
- Added `validate_intent_ir_scores_direction_from_graph_before_compat_hints`.
- The regression locks the expected behavior: removing flat compatibility hints from an `IntentIR` fixture with intact actor-relative ports must not lower the direction score.

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 111/111 passed

## 2026-04-04 (roadmap retuned around semantic truthfulness)

### Changed: roadmap priorities now explicitly favor KG quality over adapter breadth
- Logged the roadmap reassessment in `DEVELOPMENT_NOTES.md`: the existing four-layer architecture is still right, but the near-term program must be semantic-truthfulness hardening rather than adapter expansion.
- Updated `ROADMAP.md` so the next named milestones are:
  - graph-first downstream semantics
  - explicit clock-tick temporal modeling
  - KG-guided multimodal rescans
  - cross-modality evidence arbitration
  - KG-quality evaluation with gold and negative fixtures
- Demoted SystemVerilog/Verilog/VHDL adapter expansion and adapter validation to horizon work until the semantic pipeline is materially harder to fool.

### Changed: continuity docs now steer future sessions toward the truth-model program
- Updated `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `README.md`, and `USER_GUIDE.md` so they no longer imply that adapter validation is the next priority.
- The repo now consistently states that adapters should consume truth, not compensate for missing truth in the KG and temporal model.

## 2026-04-04 (steering note: multimodal semantic recovery)

### Changed: implementation guidance now explicitly centers multimodal semantic recovery
- Logged the current extraction philosophy in `DEVELOPMENT_NOTES.md` as a steering principle for future work.
- The note makes the target explicit: recover enough grounded intent from tables, figures, and prose for downstream RTL/verification generation rather than treating PDF parsing as the end goal.
- It also records the preferred tactic for future hurdles: use the KG as a search index for repeated rescans, keep the pipeline provenance-first, and prefer reusable document-native lifting strategies over speculative adapter-side inference.

## 2026-04-04 (full-converge defaults + AXI convergence stabilization)

### Changed: `specforge converge` now defaults to the full Ollama-backed loop
- `ConvergeArgs` now default `--vlm-provider` and `--nlp-provider` to `ollama` instead of `skip`.
- The intended default pipeline path is now encoded in the CLI itself: figure enrichment and NLP Level 3 run automatically during `specforge converge` unless the caller explicitly opts out.

### Fixed: monotone knowledge accounting no longer treats fewer residuals as less knowledge
- `crates/specforge/src/commands/converge.rs` no longer counts downstream adapter residual work toward `knowledge_fact_count`.
- This fixes the false `pipeline knowledge shrank` failure mode seen on a full AXI rerun, where later passes correctly reduced residual decisions but the old accounting treated that as regression.
- `EvidenceIr` and `specforge nlp-enrich` now also normalize duplicate loopback NLP records before persistence so repeated identical extractions do not inflate later passes.

### Validation
- Added regression tests:
  - `dedup_loopback_records_removes_duplicate_constraints_and_rules`
  - `nlp_enrich_dedups_duplicate_extractions_before_persisting`
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 110/110 passed
- Full original-PDF AXI converge:
  - `cargo run -p specforge -- converge /Users/richarddje/Documents/livework/chipdoc/arm/amba/core/axi/current/IHI0022_L_2025-08_AMBA_AXI_Protocol_Specification.pdf --target fsm --max-iterations 5 --vlm-provider ollama --vlm-model qwen2.5vl:7b --nlp-provider ollama --nlp-model qwen2.5vl:7b` → converged in 2 passes
- Refreshed projected AMBA validation snapshot:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 94/100 EXCELLENT
  - `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## 2026-04-04 (validation snapshot projection into tracked live docs)

### Added: deterministic live-doc projection for persisted validation reports
- Added `specforge project-validation <artifact>...`.
- The new command validates each passed IR artifact, reuses the persisted `validation_reports`, writes a tracked `VALIDATION_SNAPSHOT.md`, and refreshes the managed validation projection block in `LIVE_ACHIEVEMENT_STATUS.md`.
- Projection output is deterministic: artifact ordering uses `document_key`, findings sort by severity/category/id, and repo-internal artifact paths are rendered as relative paths.

### Changed: staged validation continuity no longer depends on manual markdown edits
- `generated/` remains untracked, but validation snapshots can now be pulled back into tracked docs on demand after local APB/AHB/AXI or other validation runs.
- The live roadmap/status/docs now treat staged IR validation projection as implemented; remaining validation work is adapter-focused.
- Refreshed the tracked validation snapshot against the current AMBA `IntentIR` artifacts:
  - APB `IHI0024_D`: 95/100 EXCELLENT
  - AHB `IHI0033_C`: 95/100 EXCELLENT
  - AXI `IHI0022_L`: 89/100 GOOD

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 107/107 passed
- `cargo run -p specforge -- project-validation generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json` → passed

## 2026-04-04 (validation back-annotation on IR artifacts)

### Added: persisted validation reports for the four IR stages
- `specforge validate <artifact>` now writes a deterministic `validation_report.json` sidecar next to `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` artifacts
- the same validation report is now backannotated into the artifact itself via a `validation_reports` field
- `SemanticIR` / `IntentIR` validation findings now include graph-aware checks for missing producers, missing consumers, and compatibility-surface lag relative to the actor-relative KG

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 106/106 passed

## 2026-04-04 (actor-relative KG carry-through into SemanticIR / IntentIR)

### Added: downstream preservation of the structural knowledge graph
- `SemanticIR` now preserves the extracted actor-signal graph via:
  - `actor_signal_relations`
  - `actor_ports`
  - `signal_connectivity`
- `IntentIR` now carries the same actor-relative KG surface forward as canonical output instead of forcing downstream consumers to rediscover relation evidence from `EvidenceIR`
- actor records now preserve grounded actor names when relation evidence makes them explicit

### Changed: validation now surfaces KG-native counts
- `specforge validate` now reports actor-signal relation, actor-port, and signal-connectivity counts for `SemanticIR` and `IntentIR`
- flat `direction_hint` fields remain as a compatibility surface, but they are no longer the only downstream representation of signal direction semantics

### Validation
- `cargo fmt --all` → passed
- `cargo test --manifest-path Cargo.toml` → 104/104 passed

## 2026-04-04 (continuity sync + local validation snapshot)

### Changed: live continuity docs now reflect the current post-converge state
- Updated the live documentation surface so crash recovery and handoff notes match the current repository status after the converge/VLM work.
- `generated/` is now treated as a local artifact root only: artifacts still materialize there, but the directory is git-ignored and no longer versioned.

### Validation
- `cargo test --manifest-path Cargo.toml` → 102/102 passed
- Current local validation snapshot:
  - APB `IHI0024_D`: 95/100 EXCELLENT after a full original-PDF `specforge converge` run with Ollama VLM + NLP Level 3; converged in 2 passes
  - AHB `IHI0033_C`: 95/100 EXCELLENT from the current local `IntentIR` snapshot
  - AXI `IHI0022_L`: 89/100 GOOD from the current local `IntentIR` snapshot
- Current AXI caveat:
  - the local AXI `SourceIR` has 20 timing diagrams classified, but the current local artifact still lacks persisted VLM timing enrichment, so timing remains the most obvious remaining score gap

## 2026-04-03 (whole-pipeline converge command + preserved loopback knowledge)

### Added: `specforge converge`
- Introduced a new top-level `converge` command that materializes the staged pipeline as a fixed-point loop instead of a one-shot chain.
- The command ingests a source once, optionally re-runs VLM figure enrichment and NLP Level 3 backannotation, rebuilds `EvidenceIR`, `SemanticIR`, `IntentIR`, and the selected adapter artifact, and stops when the persisted knowledge snapshot is unchanged between passes.
- The snapshot currently covers the staged artifacts that already persist facts today: `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter artifacts.

### Changed: `EvidenceIr::build()` now preserves prior loopback knowledge across rebuilds
- When rebuilding from the same persisted `SourceIR`, `EvidenceIr::build()` now carries forward:
  - learned signal aliases,
  - NLP-upgraded `ExtractedStatement` classes,
  - prior `SignalConstraintRecord`s,
  - prior `ConditionalRuleRecord`s.
- This closes the architectural gap where pass `N+1` could previously forget what `nlp-enrich` discovered in pass `N`.

### Changed: `specforge enrich` is now idempotent for already-enriched figures
- Timing/state-machine diagrams whose `VisualAsset.note` already contains a VLM extraction payload are skipped on later passes.
- This keeps multi-pass orchestration from re-querying the same diagram needlessly.

### Validation
- Added regression tests:
  - `converge_rebuilds_pipeline_until_snapshot_stabilizes`
  - a shared process-global test env lock now serializes `SPECFORGE_VLM_HELPER` mutations across convergence/NLP tests
- `cargo test --manifest-path Cargo.toml` → 100/100 passed

## 2026-04-03 (nlp-enrich alias marker filter + README staged-flow validation)

### Fixed: `extract_alias_phrase()` in `crates/specforge/src/commands/nlp_enrich.rs`
- Alias learning now rejects subject phrases that begin with markdown/table markers `-`, `|`, or `#`.
- This closes the remaining Form 2 cleanup gap where bullet rows, table cells, or heading-prefixed text could otherwise be learned as garbage aliases such as `- the address`.
- Ordinary prose alias learning remains unchanged for real noun phrases such as `address bus`.

### Validation
- Added regression test:
  - `extract_alias_phrase_rejects_markdown_marker_prefixes`
- `cargo test --manifest-path Cargo.toml` → 99/99 passed
- Re-executed the documented README entry flow on `README.md`:
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- inspect README.md`
  - `cargo run -p specforge -- ingest README.md --dry-run`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - `cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run`
- Materialized README artifacts now validate the staged flow end-to-end:
  - SourceIR: 0 page artifacts, 0 visual assets
  - EvidenceIR: 15 section anchors, 190 evidence spans, 190 extracted statements
  - SemanticIR: 2 actors, 6 phases, 5 invariants, 8 gates
  - IntentIR: 2 actors, 14 behaviors, 6 constraints, 1 assumption

## 2026-04-03 (convergent EvidenceIR enrichment + refreshed APB/AHB/AXI artifacts)

### Added: monotone convergent extraction loop in `evidence.rs`
- Replaced the one-shot post-table extraction tail in `EvidenceIr::build()` with `converge_evidence_extractions()`.
- Each pass now:
  - seeds from table-derived signal/enum facts,
  - rescans signal-anchored encoding tables,
  - collects newly discovered enum/value atoms,
  - extracts additional prose value constraints,
  - refines asserted/deasserted constraints with active-low/high polarity prose,
  - synthesizes KG-derived direction declarations,
  - repeats until no new synthesized statements appear.
- Rationale: the previous build order could synthesize useful `Enum ...` facts and then end the build before later prose extraction had a chance to reuse them in the same build.

### Added: anchored encoding-table recovery without hardcoded protocol value lists
- New helpers in `crates/specforge/src/ir/evidence.rs`:
  - `scan_encoding_tables_by_signal_anchor()`
  - `collect_discovered_enum_values()`
  - `extract_discovered_state_value_from_text()`
  - `extract_signal_polarity_from_prose()`
  - `apply_signal_polarity_to_constraints()`
  - `extract_dynamic_signal_constraints()`
  - `dedup_actor_signal_relations()`
- `synthesize_encoding_declarations()` now delegates to `synthesize_encoding_declarations_for_enum()` so the same encoding synthesis logic can be reused by the convergence loop.
- No new APB/AHB/AXI-specific enum/value list was added; discovered values come from extracted tables and synthesized `Enum ...` statements only.

### Changed: `classify_table_kind()` in `docling_backend.rs`
- Signature widened to `classify_table_kind(header_rows, body_rows=None, caption_text=None)`.
- Table classification now uses caption cues, header cues, and first-column body content together instead of headers alone.
- Added content-based encoding detection for weakly labeled tables by scanning body rows for binary/hex literals and bit-field references such as `HTRANS[1:0]`.
- The call site now passes `body_rows` into the helper so the classifier can use real cell content.

### Validation
- Added regression tests:
  - `anchored_encoding_scan_unlocks_dynamic_value_constraint_extraction`
  - `prose_polarity_refines_asserted_constraint_kind`
- `cargo test --manifest-path Cargo.toml` → 98/98 passed
- `cargo build --release --manifest-path Cargo.toml` → passed
- Refreshed generated APB/AHB/AXI artifacts and revalidated the current `IntentIR` outputs:
  - APB: 90/100 EXCELLENT
  - AHB: 95/100 EXCELLENT
  - AXI: 90/100 GOOD
- Net score change from the earlier baseline:
  - APB: 75 → 90
  - AHB: 95 → 95
  - AXI: 90 → 90

## 2026-04-03 (broader timing_diagram classification for figure captions)

### Fixed: classify_diagram_kind() in docling_backend.rs Python helper
- Added a second pass to the timing_diagram check: for any asset whose caption
  contains "figure", also classify as timing_diagram when caption uses protocol
  execution vocabulary: "transfer", "transaction", "handshake", "burst",
  "exit from reset", "sequence diagram".
- Rationale: bus protocol specs name clocked waveform figures after the operation
  they depict. Explicit "timing" / "waveform" words are often absent. The check
  is intentionally inclusive; VLM handles borderline cases gracefully.
- Simulated impact on AXI after re-ingest:
  - timing_diagram: 2 → 20 (+18)
  - New: VALID/READY handshake waveforms, write/read transaction dependencies,
    atomic transactions, wrapping transfers, PCMO, snoop, sequence diagrams.
  - Non-timing figures (architecture, data structure, topology) stay unknown.
- 96/96 tests pass.
## 2026-04-03 (caption-gated signal_description classification in Docling ingest helper)

### Fixed: classify_table_kind() in docling_backend.rs Python helper
- Added `caption_text=None` parameter; caption is now checked BEFORE header-based classification.
- Root cause: protocol payload/message field tables share the Name|Width|Description
  header structure with interface signal tables but describe message payload fields,
  not hardware interface pins. Without a caption check they were misclassified as
  signal_description.
  - AXI "Table A15.3: DVM message fields" is the confirmed instance: VA, PA, ASID,
    ASIDV, VMID, VMIDV, DVMType are DVM message payload fields, not interface signals.
- Fix: `caption_is_payload` flag blocks signal_description when caption contains:
  "message field(s)", "payload field(s)", "packet field(s)", "command field(s)",
  "frame field(s)".
- Call site updated: `classify_table_kind(header_rows, caption_text)`.
- Impact (requires re-ingest to take effect):
  - AXI DVM message field table: signal_description → unknown
  - DVM fields removed from declared signal set
  - 82 legitimate AXI signal_description tables unaffected
  - Width coverage: 98% → 100% after re-ingest + pipeline re-run
- 96/96 tests pass
## 2026-04-03 (row-scan clock/reset detection, immune to Docling column-ordering bugs)

### Fixed: synthesize_system_contract_from_table_descriptions() (evidence.rs)
- Replaced column-index-based detection with a full row-scan that inspects every
  cell in each body row, independent of column position.
- Root cause: Docling mis-assigns body cells to wrong column buckets for tables with
  visually distinctive (bold/boxed) cells whose PDF span-count arithmetic shifts.
  AHB Table 2-1 "Global signals" is a confirmed instance: HCLK/HRESETn are in
  col 0 of the PDF but Docling places them in col 3 of the parsed grid.
- New per-row algorithm:
  - Signal candidate: cell with ≤2 whitespace tokens, first token is a valid
    hardware signal name (not a role word like CLOCK/RESET/SOURCE).
    Excludes description cells (many words) and role cells ("Clock source" etc.).
  - Clock/reset desc: scan all cells for keyword patterns; keep the longest
    matching text so "The bus clock times all bus transfers…" beats "Clock source",
    giving accurate polarity/kind inference for resets.
- Result: AHB system contract (HCLK + HRESETn) correctly detected.
- AHB score: 90/100 → 95/100 EXCELLENT (contract_bonus 0/5 → 5/5).
- APB and AXI scores unchanged (signal tables were already correct).
- 96/96 tests pass (no new tests needed — existing contract detection tests pass).
## 2026-04-03 (header-clue + positional column detection; AMBA 5 direction mapping)

### Fixed: synthesize_signal_declarations() — direction column semantics (evidence.rs)
- Split the single dir_col into three distinct column types with correct semantics:
  - `explicit_dir_col`: header contains "direction" → literal input/output cell value
  - `source_col`: header contains "source" or "driver" → cell names the DRIVING actor
    - Requester/Initiator/Master → output; Completer/Subordinate/Slave/Target/Responder → input
    - Clock/Reset/System-bus/Global → input (infrastructure distributed into all blocks)
  - `dest_col`: header contains "destination" → cell names the RECEIVING actor (inverted)
    - Signal flows TO Subordinate/Completer → output; flows TO Manager/Requester → input
- Fixes APB: "Source" column with values "Requester"/"Completer"/"Clock"/"System bus reset"
  was previously unrecognised → all APB signals silently dropped; now correctly mapped
- Fixes AHB test: "Destination" column with "Subordinate"/"Manager" values now uses
  inverted semantics (flowing TO Subordinate = output, not input)

### Fixed: name column now header-detected with positional fallback (evidence.rs)
- name_col: search headers for signal/name/port/pin; fall back to col 0 (leftmost)
- Previously hardcoded to row.first(); now honours header position when available

### Fixed: emit width-only declaration when direction is unknown (evidence.rs)
- Added (None, Some(WidthHint::Numeric)) and (None, Some(WidthHint::Parametric)) arms
- Signals with known width but no determinable direction now emit "Signal X is width N."
  instead of being silently dropped

### Fixed: infer_signal_direction_from_section() (evidence.rs)
- Added "requester" → output (AMBA 5 APB terminology)
- Added "completer"/"target" → input
- Added "reset" to the infrastructure group → input

### Fixed: synthesize_system_contract_from_table_descriptions() (evidence.rs)
- Replaced misleading comment "No header analysis needed" with header-first detection
- name_col: headers with signal/name/port/pin → else col 0
- desc_col: headers with description/desc → else last column

### Fixed: duplicate OKAY pattern in collect_subject_signal_tokens() (evidence.rs)
- Removed second OKAY from the exclusion match arm (compiler unreachable_patterns warning)

### Test suite: 96/96 pass (no change in count)
## 2026-04-03 (WidthHint: parametric widths + table width map for KG synthesis)

### New type: WidthHint (source.rs)
- Replaces Option<u32> for signal width throughout the IR
- WidthHint::Numeric(u32) — fixed bit width (1, 2, 32, 64, ...)
- WidthHint::Parametric(String) — user-configurable RTL parameter (ADDR_WIDTH, DATA_WIDTH/8, ...)
- Backward-compatible serde: Numeric(32) -> JSON 32, Parametric("ADDR_WIDTH") -> JSON "ADDR_WIDTH"
- Both variants count as "known width" in coverage metrics and scoring

### Changed: synthesize_signal_declarations() in evidence.rs
- Width parsing now returns Option<WidthHint> instead of Option<u32>
- Numeric: parse::<u32>() -> WidthHint::Numeric
- Parametric: non-numeric, non-empty, contains alphabetic -> WidthHint::Parametric
- No artificial upper bound on numeric widths (removed the w <= 1024 filter)
- Synthesized text includes parametric widths: "Signal PADDR is output width ADDR_WIDTH."

### New: collect_signal_widths_from_tables() in evidence.rs
- Extracts width (numeric or parametric) from signal-description table Width columns
- Passed to synthesize_directions_from_relations() so KG-synthesized declarations carry width

### Changed: synthesize_directions_from_relations() in evidence.rs
- Now accepts width_map: HashMap<String, WidthHint>
- Produces "Signal PADDR is output width ADDR_WIDTH." instead of just "Signal PADDR is output."

### Changed throughout: semantic.rs, intent.rs, adapters.rs
- InterfaceSignalRecord.width_hint: Option<u32> -> Option<WidthHint>
- ExplicitTopPortRecord.width_hint: Option<u32> -> Option<WidthHint>
- InterfaceSignalAccumulator.width_hint: Option<u32> -> Option<WidthHint>
- ParsedInterfaceSignalDeclaration.width_hint: Option<u32> -> Option<WidthHint>
- parse_width_token() -> returns Option<WidthHint> (both numeric and parametric)
- merge_signal_hint<T: Copy+Eq> -> <T: Clone+Eq> (WidthHint is Clone but not Copy)
- register_interface_signal_record() signature updated
- WidthCast in expression parser kept as u32 (literal numeric, not parametric)
- Adapters convert Option<WidthHint> -> Option<u32> via .as_numeric() for FSM emission

### validate.rs: display numeric vs parametric width breakdown
- "with_width: N (X%) [N numeric, N parametric]"
- Both numeric and parametric count in width coverage score

### Results
| Spec | Before | After | Change |
|------|--------|-------|--------|
| AHB | 88/100 GOOD, 80% width | 90/100 EXCELLENT, 100% width [10 num, 11 para] | +2 pts |
| APB | 60/100 ADEQUATE, 0% width | 70/100 GOOD, 100% width [10 num, 8 para] | +10 pts |
| AXI | 75/100 GOOD, 0% width | 85/100 GOOD, 99% width [126 num, 51 para] | +10 pts |
## 2026-04-03 (R13: Tier 2 Knowledge Graph — actor-signal relation extraction)

### New types (source.rs)
- RelationKind enum (Drives | Reads)
- ActorSignalRelation struct { relation_id, actor_name, signal_name, relation, source_statement_ids, automation_confidence }

### New EvidenceIR field (evidence.rs)
- actor_signal_relations: Vec<ActorSignalRelation> (serde default = empty; extracted at build time)

### Tier 2 extraction: two complementary sources
1. **Prose verb-pattern extraction** (extract_actor_signal_relations()):
   - Passive drives: "SIGNAL is {driven|asserted|returned|...} by ACTOR" and "...from ACTOR"
   - Active drives: "ACTOR {drives|asserts|returns|...} SIGNAL" and "ACTOR must {drive|...} SIGNAL"
   - Passive reads: "SIGNAL is {read|sampled|monitored|...} by ACTOR"
   - Active reads: "ACTOR {reads|samples|...} SIGNAL"
   - Only searches for confirmed signal names (from tables + declarations)
2. **Signal table Source column extraction** (extract_relations_from_signal_tables()):
   - Reads Source/Driver/Direction column from signal_description tables
   - APB "PADDR | Requester | ..." → (Requester, Drives, PADDR)
   - APB "PREADY | Completer | ..." → (Completer, Drives, PREADY)
   - AHB tables already have direction column (covered by existing synthesis)
   - AXI tables have no Source column (no triples from tables, only from prose)

### Signal name collection: two sources
- collect_signal_names_from_tables(): ALL first-column signal names from signal_description tables (regardless of whether direction was extracted — covers APB/AXI where Source column is non-standard)
- collect_known_signal_names(): Signal names from existing "Signal X is input/output" prose declarations
- Union of both used as the search universe for prose verb patterns

### Direction synthesis: non-conflicting
- synthesize_directions_from_relations(): creates "Signal X is output." for Drives triples
- Skips signals already declared from tables (table declarations are authoritative)
- KG synthesis only adds direction for signals that had NO prior table-derived declaration

### Updated Layer D (semantic.rs) and Layer E (validate.rs)
- Declared signal set now includes Medium confidence (KG-derived) signals in addition to High confidence (table-derived)

### Results after R13
| Spec | Before R13 | After R13 | Change |
|------|-----------|-----------|--------|
| AHB  | 86/100 GOOD     | 85/100 GOOD | -1 (21 vs 17 declared, 100% dir, 47% width) |
| APB  | 35/100 NEEDS IMP | 60/100 ADEQUATE | +25 pts, 18 declared signals, 100% dir |
| AXI  | 85/100 (misleading, 1 sig) | 75/100 GOOD (honest, 182 signals) | Honest |

### Tests: 6 new (90 → 96, all passing)
- passive_drive_pattern_extracts_actor_and_signal
- active_drive_pattern_extracts_actor_and_signal
- passive_read_pattern_extracts_actor_and_signal
- must_drive_pattern_extracts_actor_from_requester_sentence
- synthesize_directions_produces_signal_is_output_declaration
- kg_extraction_produces_graph_declarations_in_evidence_ir
## 2026-04-02 (AHB + APB end-to-end pipeline validation run)

### AHB (IHI0033_C) results — full feedback loop on existing SourceIR
- Layer A suppressed 13 boilerplate NormativeStatements (85 → 72 residuals before nlp-enrich)
- nlp-enrich Pass 1: 72 candidates → 26 extracted (13 signal + 13 conditional), Form 1 backannotated 26, 1 alias learned (low-quality: "- the address" from markdown table row)
- nlp-enrich Pass 2: 46 candidates → 0 extracted → convergence at residual=46 (pass 3 stable check)
- Layer D gating: 17 declared signals (100% direction, 58% width), 248 heuristic noise excluded
- Final score: **86/100 — GOOD** (signal_dir=25/25, width=5.8/10, constraints=30/30, enums=15/15, registers=5/5, timing=5/5)
- Residual NormativeStatements: 46 (architectural/infrastructure sentences with no named signal)

### APB (IHI0024_E) results — full ingest from PDF + feedback loop
- Ingested: 48 pages, 35 visual assets
- EvidenceIR: 517 statements, 18 NormativeStatements, 37 signal_constraints (Level 2)
- nlp-enrich Pass 1: 18 candidates → 8 extracted, Form 1 backannotated 8, 0 aliases learned
- nlp-enrich Pass 2: 10 candidates → 0 extracted → convergence at residual=10
- Layer D gating: **0 declared signals** — APB signal description tables not detected as SignalDescription kind, so no High-confidence records; direction/width coverage = 0%
- Final score: **35/100 — NEEDS IMPROVEMENT** (constraints=30/30, registers=5/5, all signal coverage zero)
- Root cause: APB table classification is returning Unknown instead of SignalDescription for the signal description tables → no synthesized "Signal X is input/output" statements → Layer D has no declared set → direction/width = 0 → score tank

### Issues identified
1. **APB signal table classification**: APB tables not being classified as SignalDescription; need to inspect APB structured_tables
2. **Alias extraction quality**: "- the address" alias from markdown table row prefix is garbage — need to filter phrases starting with "-" or pure markdown tokens
## 2026-04-02 (remove --max-passes: residual-stable convergence criterion)
- **Removed --max-passes CLI option** from NlpEnrichArgs: was a safety net that is no longer needed.
- **New convergence criterion**: loop stops when residual(N) == residual(N-1).  Termination is guaranteed because the residual pool is finite and can only decrease or stay flat (monotone).  The criterion covers Form 2 alias reclassifications AND LLM extractions together, unlike the previous "pass_extracted == 0" check which only counted LLM extractions and could stop prematurely.
- **Loop structure**:  replaced by  with pass counter for display only.  dry-run breaks after one pass.
- **All 90 tests updated**: removed max_passes field from all NlpEnrichArgs struct literals; convergence test comment updated to describe residual-stable criterion.
## 2026-04-02 (Form 2: signal alias learning feedback loop)
- **EvidenceIr.signal_alias_map** (evidence.rs): new BTreeMap<String,String> field (serde default = empty). Persisted to JSON so aliases accumulate across nlp-enrich runs.
- **apply_alias_reclassification()** (EvidenceIr pub method): applies accumulated alias map to re-classify remaining NormativeStatements WITHOUT LLM calls. For each sentence containing a known alias phrase, substitutes the signal name (uppercase) and re-checks is_signal_value_constraint(). If true: reclassifies statement to SignalValueConstraint, synthesises a SignalConstraintRecord (AutomationConfidence::Low, alias-derived).
- **detect_constraint_kind_from_substituted()** (evidence.rs): helper detects must_not_change / must_be_stable / must_be_high / must_be_low / must_be_asserted / must_be_deasserted from substituted mixed-case text.
- **extract_alias_phrase()** (nlp_enrich.rs): after each successful LLM extraction, if the signal name does not appear literally in the source text, extracts a 2-4 word noun phrase (strips leading articles, rejects pronouns, limits to 4 words) and inserts it into evidence_ir.signal_alias_map.
- **Pass loop integration**: (1) START of each pass: apply_alias_reclassification() shrinks candidate pool for free; (2) AFTER each LLM extraction: learn alias if signal not in text; (3) write EvidenceIR even when only aliases were learned (no LLM extractions). Summary reports total_alias_reclassified and signal_alias_map_size.
- **Converging loop**: with --max-passes N, iteration 1 builds alias dict; iteration 2+ applies it, progressively reducing NormativeStatement residuals without LLM calls; converges when neither LLM extraction nor alias reclassification produces anything new.
- **7 new tests** (83 -> 90 total, all passing):
  - evidence.rs: apply_alias_reclassification_reclassifies_normative_statement_with_alias, apply_alias_reclassification_skips_already_covered_sentences
  - nlp_enrich.rs: extract_alias_phrase_returns_none_when_signal_appears_literally, extract_alias_phrase_extracts_noun_phrase_when_signal_absent, extract_alias_phrase_rejects_pronoun_only_subjects, extract_alias_phrase_limits_to_four_words, nlp_enrich_learns_alias_and_stores_in_evidence_ir
## 2026-04-02 (Form 1: backannotation feedback loop)
- **Form 1: backannotation** (nlp_enrich.rs): after each nlp-enrich pass, ExtractedStatement.class updated in-place: NormativeStatement -> SignalValueConstraint or ConditionalRule. Closes feedback loop from Level 3 back to EvidenceIR.
- **Fixed test parallelism bug**: added vlm_helper_lock() mutex (OnceLock<Mutex<()>>) to serialize 4 tests sharing SPECFORGE_VLM_HELPER env var.
- **Test suite: 82 -> 83 (+1, all passing)**
## 2026-04-02 (NLP pipeline Layers A/B/C/D/E: boilerplate suppression, grounded multi-pass NLP, declared-signal gating, spec-type-aware scoring)
- **Layer A — Section-aware boilerplate suppression** (`evidence.rs`)
  - New `is_boilerplate_section_title()` helper: matches Introduction, Revision History, Legal Notice, Normative/Informative References, Glossary, Acronyms, Bibliography, Scope, Terms and Definitions, About this Document, and related headings
  - `EvidenceIr::build()` block loop: looks up each sentence's section heading; if boilerplate, downgrades `NormativeStatement` → `SourceFact`
  - Effect: ~12 legal/compliance normative sentences removed from residual pool per real spec (e.g. AHB). Residuals: 59 → ~47
  - 2 tests: `is_boilerplate_section_title` unit test (12 positive + 5 negative assertions), integration test verifying intro section normative sentence becomes SourceFact while protocol section stays NormativeStatement
- **Layer D — Declared-signal gating** (`semantic.rs`)
  - `SemanticIr::build()`: after `build_interfaces()`, extracts declared signal set from `AutomationConfidence::High` interface records (those from formal `Signal X is input/output` synthesized declarations)
  - Filters `signal_constraints` and `conditional_rules` to only records where the subject/consequent signal is in the declared set; gating is disabled (all kept) if no signal declarations exist (prose-only specs)
  - Effect: heuristic noise signals (from NLP token extraction) suppressed from NLP records; only real declared signals survive. Eliminates the signal noise that diluted coverage metrics
  - 1 test: `signal_constraints_for_undeclared_signals_are_filtered_by_layer_d` — HREADY (declared) survives, NOTSIG (undeclared) removed
- **Layer E — Spec-type-aware quality scoring** (`validate.rs`)
  - Imports `AutomationConfidence` to filter signal records in `validate_intent_ir()`
  - Coverage metrics now count ONLY `AutomationConfidence::High` (declared) signals; heuristic signals reported separately as `heuristic_signal_records (excluded from coverage)`
  - New formula (100 pt max, additive, no FSM penalty for non-FSM specs):
    - Signal direction coverage (declared only): 0–25 pts
    - Signal width coverage (declared only): 0–10 pts
    - NLP constraint richness (signal + conditional, capped at 30): 0–30 pts
    - Encoding enum definitions: 0–15 pts
    - Register map records: 0–5 pts
    - Timing constraint records: 0–5 pts
    - State machine (bonus, not penalty): 0–5 pts
    - System contract (bonus, not penalty): 0–5 pts
  - Score breakdown printed per component for transparency
  - AHB projected score after all layers: ~80/100 (GOOD) vs. 27/100 before
- **Layers B+C — Grounded multi-pass NLP Level 3** (`cli.rs`, `nlp_enrich.rs`)
  - `NlpEnrichArgs`: added `--grounding-signals` (comma-separated declared signal names, or omit for auto-extraction) and `--max-passes` (default 1; multi-pass stops early on convergence)
  - `auto_extract_declared_signals()`: parses `Signal X is input/output` statements from EvidenceIR to auto-build grounding list
  - `build_nlp_prompt()` now accepts `grounding_signals: &[String]`; injects "Known hardware signals: HADDR, HTRANS, ..." section before the sentence when non-empty
  - Multi-pass loop: each pass re-derives candidates (skipping already-extracted sentences); stops when pass extracts 0 new records (convergence) OR max_passes reached; writes EvidenceIR after every productive pass
  - `count_candidate_statements()` extracted as helper for `skip` mode hint
  - 5 new tests: prompt includes grounding signals, no grounding section when empty, `parse_signal_declaration_name` extracts uppercase name, multi-pass convergence test (max_passes=3 stops after 1 productive pass)
  - Updated existing tests to include new `grounding_signals: None, max_passes: 1` fields
- **Test suite: 75 → 82 (+7 tests, all passing)**
## 2026-04-02 (NLP Level 1+2 pattern expansion: ~50%→70%+ coverage uplift)
- **Level 1 `classify_statement()` vocabulary expanded significantly**
  - `NormativeStatement`: added `cannot/can not`, `is not permitted/allowed`, `are not permitted/allowed`, `may not`, `is forbidden/illegal`, `will not`, `must/shall never`, `it is mandatory`, `is not valid/legal/supported`, `are required`
  - `ConditionalRule`: added `unless`, `provided that`, `as long as` (both leading and embedded); `while/during/after/before` now also work as leading conditionals; `cannot` added to consequent verb list
  - `SignalValueConstraint` (`is_signal_value_constraint()`): added `is tied high/low/to`, `is driven high/low`, `is held/kept high/low/stable/asserted`, `remains high/low/asserted/deasserted/stable`, `cannot change`, `cannot/will not/must not/shall not be changed`, `must/shall indicate`, `must/shall not be asserted/deasserted` (passive negation forms)
  - `TimingConstraint`: added `tco/tpd/toh/tih`, `rising/falling/clock/positive/negative edge`, `within one/two clock`, `cycles` plural
- **Level 2 `extract_signal_constraints()` multi-signal extraction**
  - Strip condition clause before scanning subject signals: `HREADY` in `"...when HREADY is LOW"` is no longer confused with the constrained signal
  - New helper `text_before_condition_marker()`: returns text before first `when/while/during/unless/provided/after/before` marker
  - New helper `collect_subject_signal_tokens()`: collects ALL valid uppercase signal tokens from a text fragment (excludes logic levels, protocol states, protocol family names, role names)
  - Multi-signal sentences like `"Both HTRANS and HADDR shall be stable"` now produce one `SignalConstraintRecord` per signal instead of one
  - Negation detection now includes `cannot` and `will not`
- **Level 2 `split_conditional_sentence()`**: added `unless`, `provided that`, `as long as` as leading conditional markers
- **Level 2 `extract_protocol_state_value()`**: added INCR4/INCR8/INCR16, WRAP4/WRAP8/WRAP16, EXCLUSIVE, RETRY, SPLIT, BYTE, HALFWORD, WORD
- **15 NLP regression tests added** (60 → 75 total; all passing)
  - Tests confirm: `cannot/is not permitted/may not` → NormativeStatement; `is tied high/is held stable/cannot change/remains stable` → SignalValueConstraint; `unless/provided that/before` → ConditionalRule; `rising edge period` → TimingConstraint; multi-signal subject extraction; condition-clause stripping; logic-level exclusion from subjects
  - Clarifying comments: tests document that more-specific `SignalValueConstraint` correctly wins over NormativeStatement when a sentence contains both a value-binding phrase and a prohibition keyword
## 2026-04-02 (qwen2.5vl:7b integration: VLM fix + NLP Level 3 nlp-enrich command)
- **Critical VLM truncation bug fixed in `enrich.rs`**
  - Removed `.min(120)` cap on VLM response storage that silently corrupted every real VLM response
  - Replaced fragile `"content":` string-search with proper `serde_json` parsing of `{choices[0].message.content}`; handles both string and array content parts with clear error on invalid JSON
  - `max_tokens` increased 1024 → 2048 for VLM diagram responses
- **Default Ollama model updated**: `llava:13b` → `qwen2.5vl:7b` (both VLM enrichment and NLP Level 3)
  - `qwen2.5vl:7b` outperforms GPT-4o-mini on document/diagram understanding benchmarks; available via `ollama pull qwen2.5vl:7b` (6GB)
  - `qwen2.5vl:7b` pulled and ready on local Ollama instance
- **`specforge nlp-enrich` command (NLP Level 3) implemented**
  - `specforge nlp-enrich <evidence-ir> --vlm-provider ollama [--vlm-model qwen2.5vl:7b] [--dry-run] [--max-sentences N]`
  - Reads `NormativeStatement` sentences from EvidenceIR not already covered by Level 2
  - Sends each sentence to LLM with a structured extraction prompt (text-only, no image)
  - Prompt yields a single JSON: `signal_constraint / conditional_rule / none`
  - Robust to markdown code-fence wrapping; validates uppercase signal names; graceful `none` handling
  - Writes new `SignalConstraintRecord` / `ConditionalRuleRecord` entries back to EvidenceIR JSON
  - `SPECFORGE_VLM_HELPER` env var override for unit testing
  - 5 tests: end-to-end pipeline, dry-run isolation, code-fence JSON parsing, invalid signal rejection, conditional rule extraction
- **Test suite: 55 → 60 (+5 NLP Level 3 tests)**
## 2026-04-02 (VLM wiring, validate command, 55-test suite, doc corrections)
- **VLM observations wired into EvidenceIR** (Steps 3.2/3.3 complete end-to-end)
  - Added `TimingDiagramExtraction` and `StateMachineExtraction` to `VisualObservationKind` in `evidence.rs`
  - New `inject_vlm_observations()`: reads `VisualAsset.note` prefix `"vlm_timing_diagram_extraction: {json}"` / `"vlm_state_machine_extraction: {json}"` and injects typed `VisualObservation` entries into the matching `VisualEvidenceItem`
  - Enriched figures automatically upgraded to `VisualEvidenceRole::Normative` (highest-priority evidence)
- **SemanticIR VLM observation parsing** (timing + state machine → typed records)
  - New `extract_records_from_vlm_observations()` in `semantic.rs` iterates EvidenceIR visual observations
  - `parse_timing_diagram_observation()`: each VLM annotation string → `TimingConstraintRecord { description: annotation, confidence: Medium }`; merged with table-synthesized timing constraints
  - `parse_state_machine_observation()`: each VLM state → `RegularStateRecord`; each VLM transition → `StateTransitionRecord`; merged with formal syntax records (non-duplicate append)
  - Result: timing constraints from both tables and VLM diagrams, state records from both formal syntax and VLM extraction
- **specforge validate command** (Step 4.1 complete)
  - New `crates/specforge/src/commands/validate.rs` — auto-detects IR stage from `stage` field in artifact JSON
  - `validate_source_ir`: document profile, table classification, diagram classification, VLM readiness, section classification, residual count
  - `validate_evidence_ir`: statement classification breakdown, NLP coverage %, structured extraction counts, VLM observation counts
  - `validate_semantic_ir`: signal coverage (with_direction %, with_width %, fully_typed %), semantic record counts, system contract, residual decisions
  - `validate_intent_ir`: signal coverage, intent record counts, quality score (0–100) with grade EXCELLENT/GOOD/ADEQUATE/NEEDS IMPROVEMENT/INCOMPLETE
  - Wired in `cli.rs` as `Commands::Validate(ValidateArgs)` and dispatched in `lib.rs`
- **Test suite expanded: 49 → 55 (+6)**
  - `ir::semantic::tests::vlm_timing_diagram_observation_produces_timing_constraint_records` — full chain: SourceIR note → EvidenceIR observation → SemanticIR timing constraints
  - `ir::semantic::tests::vlm_state_machine_observation_produces_state_and_transition_records` — full chain: SourceIR note → EvidenceIR observation → SemanticIR states/transitions
  - `commands::validate::tests::validate_source_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_evidence_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_semantic_ir_artifact_reports_without_error`
  - `commands::validate::tests::validate_intent_ir_artifact_reports_without_error`
  - All 55 tests pass, 0 failures
- **EXTRACTION_ARCHITECTURE.md** corrected with accurate status for all completed steps:
  - SourceIR: DiagramKind ✅, VLM enrichment ✅
  - EvidenceIR: SignalConstraintRecord ✅, ConditionalRuleRecord ✅, TimingDiagramExtraction/StateMachineExtraction ✅
  - SemanticIR: signal_constraints ✅, conditional_rules ✅, VLM state/transition merge ✅, VLM timing merge ✅
  - IntentIR: signal_constraints ✅, conditional_rules ✅, VLM state/transition records ✅
  - Tier 3 Steps 3.1/3.2/3.3: ✅ done; added Step 3.4 (NLP Level 3: LLM-based reclassification) as planned next step
  - Step 4.1 Validation: ✅ done
## 2026-04-02 (NLP Level 2 structured extraction + VLM enrichment pipeline)
- **EXTRACTION_ARCHITECTURE.md** updated as authoritative reference: NLP 4-level pyramid, classification-vs-extraction gap analysis, VLM provider architecture (Ollama/OpenAI/LM Studio), all implementation steps with precise ✅/❌ status
- **Level 2 NLP: SignalConstraintRecord extraction**
  - New types in `source.rs`: `SignalConstraintKind`, `SignalConstraintRecord`, `ConditionalRuleRecord`
  - `extract_signal_constraints()` in `evidence.rs`: for each `SignalValueConstraint` sentence, extracts `{subject_signal, constraint_kind, target_value, condition_text, negated}` via syntactic pattern matching; stop-worded for AMBA/ARM/company names
  - `extract_conditional_rules()` in `evidence.rs`: for each `ConditionalRule` sentence, extracts `{antecedent_text, consequent_signal, consequent_action}` by sentence splitting on when/if/while/during
  - `EvidenceIr.signal_constraints` + `EvidenceIr.conditional_rules` as first-class typed fields
  - Carried through `SemanticIr.signal_constraints` and `IntentIr.signal_constraints`
  - AHB result: 12 `SignalConstraintRecord` (HAUSER must_not_change, HEXOKAY must_be_deasserted, etc.), 36 `ConditionalRuleRecord`
- **DiagramKind classification in SourceIR (Step 3.1)**
  - New `DiagramKind` enum in `source.rs`: `TimingDiagram`, `StateMachineDiagram`, `BlockDiagram`, `RegisterBitfield`, `TruthTable`, `FlowChart`, `Unknown`
  - `VisualAsset.diagram_kind` field set from caption text in Docling Python helper
  - `classify_diagram_kind()` in Python helper: pattern-matches AMBA-specific caption vocabulary ("read transfer", "write transfer", "burst", "wait state" → `timing_diagram`; "Manager interface", "multiplexor interconnection" → `block_diagram`; etc.)
  - AHB result: **17 timing diagrams** correctly classified, 3 block diagrams
- **specforge enrich command (Step 3.2/3.3)**
  - New `specforge enrich <source-ir> --vlm-provider <provider>` command
  - Providers: `ollama` (localhost:11434, model `llava:13b`), `openai` (OPENAI_API_KEY, model `gpt-4o`), `lmstudio` (localhost:1234), `skip` (default)
  - `--classify-only` flag: shows timing/state-machine diagram counts without calling VLM
  - `--vlm-model` override for custom models
  - `--dry-run` shows which figures would be sent to VLM without making calls
  - `SPECFORGE_VLM_HELPER` env var override for unit testing (same pattern as `SPECFORGE_DOCLING_HELPER`)
  - Structured prompts: timing diagram → `{signals, cycles, annotations}` JSON; state machine → `{states, transitions}` JSON
  - All providers use OpenAI-compatible chat completions API (supports Docling's granite-docling model via Ollama or LM Studio)
  - VLM enrichment writes updated `VisualAsset.note` with typed extraction; downstream `specforge evidence` picks it up
- 48 tests, 0 failures
## 2026-04-02 (Tier 2: Register/Timing type system + NormativeStatement sub-classes)
- added `RegisterRecord` and `RegisterFieldRecord` types to `source.rs` (foundation layer, no circular deps)
- added `TimingConstraintRecord` type to `source.rs`
- re-exported these types from `semantic.rs` so `IntentIR` and adapters import from `semantic` as before
- added `synthesize_register_records()` in `evidence.rs`: reads `SourceIR.structured_tables` where `table_kind == RegisterMap`; extracts register name, offset address, bit field rows
- added `synthesize_timing_constraints()` in `evidence.rs`: reads `SourceIR.structured_tables` where `table_kind == TimingParameter`; extracts parameter name, min/typ/max values, unit
- added `EvidenceIr.register_records: Vec<RegisterRecord>` and `EvidenceIr.timing_constraints: Vec<TimingConstraintRecord>` as typed first-class fields
- carried `register_records` and `timing_constraints` through `SemanticIr` and `IntentIr` unchanged
- extended `StatementClass` with `TimingConstraint` (cycle counts, setup/hold references, latency bounds) and `ConditionalRule` (`when X then Y` / `if A then B` conditional behavioral structures)
- updated `classify_statement()` to detect `TimingConstraint` and `ConditionalRule` patterns before the generic `NormativeStatement` check
- updated `EXTRACTION_ARCHITECTURE.md` to reflect precise current done/pending status and sharpen modality descriptions with exact type names
- validated on AMBA AHB PDF: 21 register records, 8 timing constraints, statement classes: 91 normative_statement, 42 conditional_rule, 30 timing_constraint, 14 derived_rule, 5 explicit_abstraction (vs. 100% source_fact before)
- all `cargo fmt` and `cargo test` pass: 48 tests, 0 failures
## 2026-04-02 (SOTA SourceIR and EvidenceIR)
- created `EXTRACTION_ARCHITECTURE.md` — comprehensive reference document capturing the full SOTA extraction vision for chip spec PDFs: six information modalities, quality gap analysis per IR stage, target architecture, and priority-ordered implementation plan (Tier 1–4)
- updated `ROADMAP.md` with new workstreams R8 (SourceIR SOTA capture), R9 (EvidenceIR SOTA typed evidence), and R10 (EvidenceIR VLM visual content)
- extended Docling Python helper to extract in a single pass:
  - **structured table cell grids** (`StructuredTableRecord` with header/body row cells, row/col spans, `is_header` flags)
  - **table kind classification** (`classify_table_kind`): `signal_description`, `encoding`, `register_map`, `timing_parameter`, `feature_matrix`, `unknown`
  - **typed content elements** (`ContentElementRecord`): all text elements with Docling type labels (title, section_header, body_text, list_item, code, caption, footnote, formula), reading order, page provenance
  - **section hierarchy with semantic classification** (`ContentSectionRecord` with `SectionKind`): `Boilerplate`, `SignalDescription`, `Normative`, `Timing`, `RegisterDescription`, `Glossary`, `Appendix`, `TableOfContents`
  - **document profile** (`DocumentProfile`): title, page/table/figure/section counts
- added new Rust types to `source.rs`: `StructuredTableRecord`, `StructuredTableCellRecord`, `TableKind`, `ContentElementRecord`, `ContentElementKind`, `ContentSectionRecord`, `SectionKind`, `DocumentProfile`
- updated `SourceIr` struct with new `#[serde(default)]` fields: `structured_tables`, `content_elements`, `document_sections`, `document_profile`
- updated `DoclingBackendSummary` to deserialize all new fields; updated `SourceIr::materialize()` to populate them
- added `StatementClass::NormativeStatement` to `EvidenceIR` statement classification — sentences with `shall`/`must`/`shall not` in non-boilerplate sections are now correctly classified as behavioral requirements rather than generic `SourceFact`
- added `synthesize_declarations_from_tables()` in `EvidenceIR` that reads `source_ir.structured_tables` and synthesizes formal typed declarations:
  - `SignalDescription` tables → `Signal X is output/input [width N].` declarations (High confidence)
  - `Encoding` tables → `Enum <name> <member> = <value>.` declarations (High confidence)
  - Direction inferred from `ContentSectionRecord.section_kind` + section title keywords; width from numeric cell values; non-signal tokens filtered via `is_signal_synthesis_non_signal()`
- **removed `parse_signal_table_row` band-aid from `SemanticIR`** — signal declarations now flow cleanly from `EvidenceIR` structured table synthesis through `SemanticIR`'s existing `parse_explicit_signal_declaration` and `parse_explicit_symbol_definition` parsers
- validated on AMBA AHB Protocol Specification PDF (SOTA pipeline, re-ingested):
  - `source_ir.structured_tables`: 40 tables (11 signal_description, 2 encoding, 3 register_map, 2 timing_parameter, 22 unknown)
  - `source_ir.content_elements`: 1004 typed text elements
  - `source_ir.document_sections`: 172 sections (115 normative, 37 signal_description, 8 boilerplate, 5 timing, 4 appendix, 2 glossary, 1 table_of_contents)
  - `source_ir.document_profile`: page_count=104, table_count=40, figure_count=30, section_count=172
  - 17 AHB signals with explicit direction+width in adapter signal inventory (HSELX newly added from Decoder table)
  - `NormativeStatement` classification active for behavioral requirements
  - No SemanticIR band-aid; signal declarations flow architecturally
- all `cargo fmt` and `cargo test` checks pass: 48 tests, 0 failures
## 2026-04-02 (continued)
- improved `SemanticIR` extraction quality for real chip specification PDFs with three targeted fixes:
  - **expanded `signal_stop_words()`** with ~200 entries covering legal/contractual vocabulary, common English all-caps words (HIGH, LOW, etc.), AMBA/ARM protocol family names, company names, document structure words, and technology abbreviations that are never hardware signal names; this eliminates legal front-matter contamination from signal extraction
  - **added boilerplate section filtering** in `SemanticContext::from_evidence_ir` so statements from sections matching legal/admin patterns (licence, proprietary notice, change history, etc.) are excluded from actor/interface/invariant extraction entirely
  - **added interface noise filtering** in `build_interfaces` so heuristic interfaces with >8 signals require ≥2 supporting statements; this eliminates the large spurious interfaces created by co-mentions in legal paragraphs while keeping all small hardware signal groups
- added **markdown signal-table row parsing** in `build_interfaces`: when a table row's first cell looks like a hardware signal name and the section title matches a known direction context ("Manager signals" → output, "Subordinate signals" → input, "Global/Decoder signals" → input), the row is parsed directly into a typed `InterfaceSignalRecord` with explicit direction and width, extracted at Medium automation confidence
- validated improvements on the AMBA AHB Protocol Specification PDF (`IHI0033_C_2021-09_AMBA_5_AHB_Protocol_Specification.pdf`):
  - 104 page artifacts and 70 visual assets materialized correctly by Docling
  - `interface_count` reduced from 172 → 94 (45% reduction, legal text gone)
  - `signal_candidate_count` in the adapter reduced from 250 → 57 (77% reduction, mostly real AHB signals)
  - 16 AHB signals now carry explicit direction and width from signal table parsing:
    - Manager outputs (direction=output): HADDR, HBURST, HEXCL, HMASTER, HMASTLOCK, HNONSEC, HPROT, HSIZE, HTRANS, HWDATA, HWRITE, HWSTRB
    - Subordinate outputs / Manager inputs (direction=input): HEXOKAY, HRDATA, HREADYOUT, HRESP
    - Key widths extracted: HTRANS=2, HSIZE=3, HWRITE=1, HMASTLOCK=1, HEXCL=1, HNONSEC=1, HREADYOUT=1, HRESP=1, HEXOKAY=1
  - adapter correctly blocked (honest: AHB spec prose does not carry formal control blocks or system contract declarations)
- added 1 new regression test: `extracts_signal_direction_and_width_from_markdown_signal_description_table`
  - verifies Manager-section rows are extracted as Output with correct numeric widths
  - verifies Subordinate-section rows are extracted as Input with correct numeric widths
  - total tests: 48 passing, 0 failing
- installed Docling 2.84.0 globally into Python 3.11 (`/opt/homebrew/lib/python3.11/site-packages/`) to enable PDF processing
- all `cargo fmt` and `cargo test` checks pass
## 2026-04-02
- widened `SemanticIR` so it now preserves canonical `.fsm`-relevant symbol-definition and structured-control surface rather than relying only on legacy decision-tree fragments:
  - canonical symbol definitions for `Constant`, `Define`, `Param`, and `Enum`
  - canonical control expressions, branch-local actions, and dedicated synchronous-reset/asynchronous-reset control-block roles
  - richer module-scoped carry-through for the same widened semantic surface
- widened the canonical reset contract so `SystemContractRecord` now preserves reset kind, reset polarity, assertion timing, release timing, and reset-target semantics explicitly instead of leaving real hardware reset behavior implicit
- tightened reset normalization so explicit reset declarations now:
  - preserve synchronous reset as synchronous assertion + synchronous release through the data-input path
  - preserve asynchronous reset as asynchronous assertion + synchronous release through the dedicated reset pin
  - infer active-low polarity from `_n` / `_b` reset naming and otherwise fall back to active-high with lower automation confidence when explicit polarity wording is omitted
- widened `IntentIR` so it now carries canonical symbol-definition sections and structured control blocks unchanged for downstream adapters
- refactored the `.fsm` adapter to lower from canonical `symbol_definitions` and `control_blocks` first, with legacy decision-tree fragments kept only as a fallback when the widened canonical surface is absent
- tightened `.fsm` system-contract renderability so reset polarity must stay recoverable honestly from `sreset` / `asreset` plus the reset signal name in the current target slice
- widened emitted `.fsm` text so the renderable slices now cover:
  - `+constants`, `+define`, `+params`, and `+enums` sections
  - structured standalone/DT and FSM-root lowering from canonical control blocks
  - canonical synchronous-reset and asynchronous-reset control-role blocks
  - explicit public-output targets and dual-output assignment forms carried through the widened control model when renderable
- widened the `.fsm` adapter so selector/test-node control and canonical compound-update actions now lower honestly into emitted `.fsm` text when their canonical selector/predicate/update shapes map directly to explicit `.fsm` test-selector tokens and update shorthand, while unsupported selector predicates remain blocked explicitly
- repaired accidental corruption in the `adapters.rs` regression module and tightened adapter residual logic so renderable compound-update artifacts no longer keep a stale `fsm_adapter_dt_action_graph` packet
- reviewed the current `fsmgen` direct-root contract and confirmed that `?mod:name` / `?module:name` are still compatibility-level accepted spellings on a shared single-module path rather than a settled backend-neutral semantic distinction for SpecForge
- tightened the SpecForge `.fsm` adapter root-kind model so it now only represents the current honest canonical roots (`dt`, `fsm`, `top`) and no longer carries speculative `mod` / `module` placeholder variants in adapter JSON or deferred-root decisions
- tightened explicit reset parsing so both of these phrasing styles now normalize into the widened backend-neutral reset contract:
  - `Reset rst_n is asynchronous active low.`
  - `Reset rst is synchronous active high.`
- added regression coverage for:
  - semantic extraction of synchronous active-high reset phrasing
  - intent carry-through of synchronous active-high reset phrasing
  - semantic and intent carry-through of inferred active-low reset polarity from `rst_n`
  - honest adapter blocking when reset polarity cannot be preserved through the reset signal name
  - renderable standalone DT lowering with canonical symbol-definition sections
  - renderable structured FSM lowering with canonical reset-role blocks
  - renderable selector-based standalone DT lowering
  - renderable computed-selector standalone DT lowering
  - honest blocking when a selector branch predicate does not map relative to the chosen selector
  - renderable compound-update standalone DT lowering
  - tightened deferred-root decisions so renderable/blocked adapter artifacts keep only the current honest root-kind set (`dt`, `fsm`, `top`)
- validated the widened `.fsm` semantic slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo fmt --all --manifest-path Cargo.toml --check`
  - `cargo test --manifest-path Cargo.toml adapters`
  - `cargo test --manifest-path Cargo.toml`
  - an execute-mode end-to-end CLI pipeclean on a temporary inferred-polarity reset sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary synchronous-active-high reset sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary selector/test-node sample through `ingest -> evidence -> semantic -> intent -> adapt`
  - an execute-mode end-to-end CLI pipeclean on a temporary compound-update sample through `ingest -> evidence -> semantic -> intent -> adapt`
- confirmed the representative inferred-polarity end-to-end adapter output is now safely renderable while preserving the widened reset contract in JSON:
  - `document_key: inferred_reset_cli`
  - `semantic/system_contract.reset_polarity: active_low`
  - `semantic/system_contract.assertion_timing: asynchronous_to_clock`
  - `semantic/system_contract.release_timing: synchronous_to_clock`
  - `semantic/system_contract.target_kind: dedicated_reset_pin`
  - `semantic/system_contract.automation_confidence: medium`
  - `intent/system_contract` matches the widened semantic reset contract exactly
  - `emitted_target_path: generated/adapters/fsm/inferred_reset_cli/inferred_reset_cli.fsm`
- confirmed the representative synchronous-active-high end-to-end adapter output is now safely renderable:
  - `document_key: sync_control`
  - `lowering_status: renderable`
  - `selected_root_kind: fsm`
  - `emitted_target_path: generated/adapters/fsm/sync_control/sync_control.fsm`
- confirmed the representative selector/test-node end-to-end adapter output is now safely renderable:
  - `document_key: selector_dt`
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/selector_dt/selector_dt.fsm`
  - emitted test-node block includes `(?MODE ...)` and the selector branch token `=mode_t.idle`
- confirmed the representative compound-update end-to-end adapter output is now safely renderable:
  - `document_key: compound_update_dt`
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `residual_decision_count: 4`
  - `emitted_target_path: generated/adapters/fsm/compound_update_dt/compound_update_dt.fsm`
  - emitted update block includes `(-bump` and `(+= ACC STEP)`
- refreshed the live documentation surface so the README, user guide, live status tracker, codebase analysis, development notes, roadmap, and continuity records now describe the widened canonical reset contract, the landed selector/test-node and compound-update slice, the current reset-naming convention, and the remaining direct-module alias gap plus explicit unsupported selector/predicate boundaries
- refreshed the same live documentation surface again so it now records the direct-module defer decision explicitly, removes speculative adapter root-kind language, and advances the next milestone to validation/back-annotation
## 2026-04-01
- enriched `SemanticIR` so it now preserves explicit backend-neutral module and top-composition records from `Module ...` and `Top ...` statements, including typed top ports, child-module references, and explicit wiring links
- tightened semantic extraction so module-scoped and top-scoped statements are handled through scoped parsing helpers and no longer leak into document-global direct-root inference
- enriched `IntentIR` so it now carries canonical explicit module and top-composition surface forward unchanged for downstream adapters
- widened the `.fsm` adapter beyond a single direct-root model so it now:
  - inventories explicit module candidates and explicit top candidates from canonical intent records
  - selects an honest `?top:name` root when exactly one explicit top composition is renderable
  - renders stable top-level support blocks such as `?ports:public_io` and `?toplink:wiring`
  - embeds referenced renderable child module roots after the selected `?top:name` root
  - keeps standalone direct `?mod:name` / `?module:name` alias roots deferred until there is a real backend-neutral direct-module distinction
- tightened adapter renderability checks for explicit top composition so emitted `.fsm` text now requires:
  - fully typed explicit top ports
  - existing referenced child modules
  - renderable child module roots
  - width-compatible and direction-compatible explicit link endpoints
  - explicit links for the current multi-child composition slice
- tightened adapter residual logic so standalone DT residuals are suppressed when an explicit `?top:name` source document is selected and composition-specific residuals remain honest when child modules or links are missing
- extended `specforge adapt` execute-mode summaries with `module_candidate_count` and `top_candidate_count`
- added regression coverage for:
  - explicit module/top extraction in `SemanticIR`
  - explicit module/top carry-through in `IntentIR`
  - renderable and blocked explicit top-composition adapter paths
- validated the new explicit composition slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_top.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_top/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_top/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_top/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_top/intent_ir.json --target fsm`
- confirmed the representative explicit top-composition end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: top`
  - `signal_candidate_count: 1`
  - `decision_tree_candidate_count: 0`
  - `state_candidate_count: 0`
  - `transition_candidate_count: 0`
  - `module_candidate_count: 2`
  - `top_candidate_count: 1`
  - `residual_decision_count: 3`
  - `emitted_target_path: generated/adapters/fsm/explicit_top/datapath.fsm`
- refreshed the live documentation surface so the roadmap, status trackers, user guide, architecture docs, and continuity files now describe the landed explicit `?top:name` slice and the still-deferred direct module-alias roots
- enriched `SemanticIR` so it now preserves backend-neutral regular-state and transition records from explicit `State ...` and `Transition ...` statements
- enriched `IntentIR` so it now carries canonical regular-state and transition surface forward for downstream adapters
- widened the `.fsm` adapter so it now selects honest `?fsm:name` roots from the explicit canonical state graph, groups state-matching control fragments into state bodies, preserves unmatched control fragments as standalone `-block` children, and renders sequential state-body assignments with `<=`
- tightened adapter-side residual logic so the unresolved state-graph packet only remains when structured FSM lowering is actually blocked
- extended `specforge adapt` execute-mode summaries with `transition_candidate_count` for explicit FSM-root pipecleans
- added regression coverage for:
  - explicit regular-state and transition extraction in `SemanticIR`
  - canonical regular-state and transition carry-through in `IntentIR`
  - renderable and blocked structured `?fsm:name` adapter paths
- validated the new structured FSM slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_fsm.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_fsm/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_fsm/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_fsm/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_fsm/intent_ir.json --target fsm`
- confirmed the representative explicit FSM end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: fsm`
  - `signal_candidate_count: 7`
  - `decision_tree_candidate_count: 1`
  - `state_candidate_count: 2`
  - `transition_candidate_count: 2`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/explicit_fsm/explicit_fsm.fsm`
- enriched `SemanticIR` so it now preserves backend-neutral system contract and init-assignment records from explicit `Clock ...`, `Reset ...`, and `Init ...` statements
- enriched `IntentIR` so it now carries canonical system contract and init-assignment surface forward for downstream adapters
- widened the `.fsm` adapter so it now renders explicit standalone sequential `?dt:name` text with `(+system ...)` and `(:= ...)` when the canonical system/init facts are complete
- added a dedicated adapter-side residual for unresolved system/init surface so sequential standalone DT cases stay blocked explicitly instead of inventing reset semantics
- added regression coverage for:
  - explicit system/init extraction in `SemanticIR`
  - canonical system/init carry-through in `IntentIR`
  - renderable and blocked standalone sequential `.fsm` adapter paths
- validated the new standalone sequential slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/seq_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/seq_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/seq_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/seq_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/seq_dt/intent_ir.json --target fsm`
- confirmed the representative explicit sequential end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 4`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 4`
  - `emitted_target_path: generated/adapters/fsm/seq_dt/seq_dt.fsm`
- enriched `SemanticIR` so it now preserves typed signal records and backend-neutral guarded/action control fragments when the evidence is explicit enough
- enriched `IntentIR` so it now carries the canonical interface inventory and backend-neutral control fragments forward for downstream adapters
- widened the `.fsm` adapter so it now consumes the canonical interface/control surface instead of relying only on mined prose hints
- the `.fsm` adapter now emits a real standalone `?dt:name` file for explicit canonical cases and keeps broader sequential/system-contract/composition cases blocked instead of inventing semantics
- added regression coverage for:
  - explicit typed-signal/control extraction in `SemanticIR`
  - canonical interface/control carry-through in `IntentIR`
  - blocked and renderable `.fsm` adapter paths
- validated the new canonical/renderable slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/comb_dt.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/comb_dt/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/comb_dt/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/comb_dt/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/comb_dt/intent_ir.json --target fsm`
- confirmed the representative explicit end-to-end adapter output is now safely renderable:
  - `lowering_status: renderable`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 3`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 2`
  - `emitted_target_path: generated/adapters/fsm/comb_dt/comb_dt.fsm`
- implemented the first real adapter slice on top of persisted `IntentIR` artifacts
- added `specforge adapt <intent-ir> --target fsm [--dry-run]` to preview or materialize `generated/adapters/fsm/<document_key>/adapter.json`
- replaced the old adapter planning-only scaffolding with a typed adapter artifact model in `crates/specforge/src/ir/adapters.rs`
- the first `.fsm` adapter slice now:
  - loads persisted `IntentIR` JSON from disk
  - selects a conservative DT-oriented root instead of inventing FSM or composition semantics
  - inventories low-confidence signal candidates and DT/state candidate structure from canonical intent records
  - preserves upstream residual decisions and emits adapter-side residuals for missing signal inventory, DT fragments, and broader root-kind expansion
  - blocks emitted `.fsm` text when target syntax would require semantic invention
- added adapter-stage unit tests for:
  - handshake-driven `.fsm` adapter artifact construction
  - wrong-stage input rejection before deserializing as `IntentIR`
- validated the new adapter slice with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run --manifest-path Cargo.toml -- ingest <temp>/handshake.md`
  - `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/handshake/source_ir.json`
  - `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/handshake/evidence_ir.json`
  - `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/handshake/semantic_ir.json`
  - `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/handshake/intent_ir.json --target fsm`
- confirmed the representative end-to-end adapter output is currently honest and blocked rather than fabricated:
  - `lowering_status: blocked`
  - `selected_root_kind: dt`
  - `signal_candidate_count: 2`
  - `decision_tree_candidate_count: 1`
  - `residual_decision_count: 3`
- pivoted the repository objective so `IntentIR` is now the canonical product boundary
- rewrote the core docs around the explicit staged pipeline:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters
- added `INTENTIR_SPEC.md` as the canonical architecture/specification document for the new direction
- renamed the active Rust crate and CLI direction from `spec2fsm` to `specforge` with no compatibility aliasing
- renamed the workspace member path to `crates/specforge`
- refactored the Rust code layout around explicit staged IR modules:
  - `crates/specforge/src/ir/source.rs`
  - `crates/specforge/src/ir/evidence.rs`
  - `crates/specforge/src/ir/semantic.rs`
  - `crates/specforge/src/ir/intent.rs`
  - `crates/specforge/src/ir/adapters.rs`
- replaced the previous ingest-manifest framing with a real `SourceIR` artifact
- updated `specforge ingest` so:
  - dry-run prints computed `SourceIR` JSON
  - execute mode materializes `generated/source_ir/<document_key>/source_ir.json`
- formalized a stricter SOTA ingestion stance:
  - structured parser first
  - provenance-preserving page and visual asset capture second
  - selective multimodal enrichment for figures, charts, diagrams, and image-heavy regions third
- extended `SourceIR` scaffolding so it now reserves:
  - parser backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- extended `EvidenceIR` scaffolding so it now reserves:
  - multimodal evidence spans
  - visual evidence items
  - text-to-figure links
  - picture-description / OCR-over-image / chart-extraction observations
- recorded adapter targets as downstream of `IntentIR`:
  - `.fsm`
  - SystemVerilog
  - Verilog
  - VHDL
- kept residual decision packets as a first-class mechanism for unresolved automation
- updated the live status tracker so the next highest-priority gap is:
  - close the remaining `SourceIR` structured-PDF-normalization gap and build the first real multimodal `EvidenceIR` extractor
- validated the renamed crate and staged IR refactor with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- --help`
  - `cargo run -p specforge -- ingest README.md --dry-run`
- confirmed that `specforge ingest README.md --dry-run` now exposes parser backend, page-artifact manifest, visual-asset manifest, and placeholder-binding fields in `SourceIR`
- confirmed the remaining `spec2fsm` mentions are historical continuity references rather than active product naming
- implemented the first real structured PDF normalization backend for `SourceIR`
- added `crates/specforge/src/ir/source/docling_backend.rs` to orchestrate a Docling-backed PDF conversion flow from Rust
- `specforge ingest <pdf>` now materializes:
  - promoted markdown
  - page images and per-page metadata sidecars
  - cropped picture and table assets
  - backend raw JSON and metadata JSON
  - `page_artifacts.json` and `visual_assets.json`
- `SourceIR` PDF execute mode now upgrades its normalization status from `planned_conversion` to `ready` after successful backend materialization
- added a `source_ref` field to visual-asset records so later stages can trace assets back into backend-native structured output
- added runtime dependency guidance:
  - discover `docling` from `python3` or `python`
  - optionally override with `SPECFORGE_DOCLING_PYTHON`
- added a backend-override seam for tests and advanced local integration with `SPECFORGE_DOCLING_HELPER`
- added a stubbed PDF materialization unit test so the real SourceIR backend path is exercised without requiring Docling inside `cargo test`
- validated the new PDF backend with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test`
  - `cargo run -p specforge -- ingest README.md`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
- updated the live status tracker so the remaining top-priority gap is now the first real `EvidenceIR` extractor rather than the SourceIR PDF-normalization backend
- implemented the first real `EvidenceIR` extractor on top of persisted `SourceIR` artifacts
- added `specforge evidence <source-ir> [--dry-run]` to preview or materialize `generated/evidence_ir/<document_key>/evidence_ir.json`
- `EvidenceIR::build` now:
  - loads persisted `SourceIR` JSON from disk
  - requires `normalization_status: ready`
  - parses promoted markdown into section anchors and block-level evidence spans
  - projects `SourceIR` visual assets into typed visual evidence items
  - links caption spans with `describes` and figure/table references with `cites`
  - emits heuristic statement classes for source facts, derived rules, local design decisions, and explicit abstractions
- added stage-artifact loading support and deserialize coverage needed to rebuild `EvidenceIR` from saved `SourceIR` JSON
- added unit tests for:
  - markdown-only `EvidenceIR` construction
  - caption plus figure-reference grounding into visual evidence
- validated the new `EvidenceIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `EvidenceIR`: 14 section anchors, 167 evidence spans, 167 extracted statements
  - PDF-backed `EvidenceIR`: 18 section anchors, 225 evidence spans, 11 visual evidence items, 19 evidence links, 225 extracted statements
- updated the live status tracker so the remaining top-priority gap is now the first real `SemanticIR` constructor rather than the `EvidenceIR` extraction stage
- implemented the first real `SemanticIR` extractor on top of persisted `EvidenceIR` artifacts
- added `specforge semantic <evidence-ir> [--dry-run]` to preview or materialize `generated/semantic_ir/<document_key>/semantic_ir.json`
- `SemanticIR::build` now:
  - loads persisted `EvidenceIR` JSON from disk
  - derives typed semantic artifacts under `generated/semantic_ir/<document_key>/semantic_ir.json`
  - discovers actors, interfaces, phases, invariants, contracts, gates, abstractions, and decomposition candidates from deterministic heuristics
  - emits residual decisions for unresolved actor boundaries, overlapping interface groups, and ambiguous visual semantics
- added stage-artifact loading support needed to rebuild `SemanticIR` from saved `EvidenceIR` JSON
- added unit tests for:
  - handshake-driven actor/interface/invariant extraction
  - ambiguous visual grounding residual decisions
- validated the new `SemanticIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `SemanticIR`: 2 actors, 0 interfaces, 4 phases, 3 invariants, 3 gates, 1 abstraction, 12 decomposition candidates, 0 residual decisions
  - PDF-backed `SemanticIR`: 2 actors, 22 interfaces, 7 phases, 17 invariants, 2 contracts, 20 gates, 12 decomposition candidates, 2 residual decisions
- updated the live status tracker so the remaining top-priority gap is now the first real canonical `IntentIR` constructor rather than the `SemanticIR` stage
- implemented the first real `IntentIR` constructor on top of persisted `SemanticIR` artifacts
- added `specforge intent <semantic-ir> [--dry-run]` to preview or materialize `generated/intent_ir/<document_key>/intent_ir.json`
- `IntentIR::build` now:
  - loads persisted `SemanticIR` JSON from disk
  - derives canonical intent artifacts under `generated/intent_ir/<document_key>/intent_ir.json`
  - canonicalizes actor responsibilities, behaviors, constraints, and assumptions from deterministic heuristics over semantic records
  - preserves semantic residual decisions and emits additional canonicalization residuals only when the intent model would otherwise become speculative
- added unit tests for:
  - handshake-driven intent identity, behavior, constraint, and assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
- validated the new `IntentIR` stage with:
  - `cargo fmt --all --manifest-path Cargo.toml`
  - `cargo test --manifest-path Cargo.toml`
  - `cargo run -p specforge -- ingest README.md`
  - `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
  - `cargo run -p specforge -- intent generated/semantic_ir/specforge_docling_sample/semantic_ir.json`
- confirmed live execute-mode outputs for validation:
  - markdown-backed `IntentIR`: 2 actors, 7 behaviors, 3 constraints, 1 assumption, 0 residual decisions
  - PDF-backed `IntentIR`: 2 actors, 28 behaviors, 24 constraints, 1 assumption, 2 residual decisions
- updated the live status tracker so the remaining top-priority gap is now the first real adapter lowering pass rather than the `IntentIR` stage
- added `subs/fsmgen` as a pinned git submodule using the SSH remote `git@github.com:rdje/fsmgen.git`
- pinned the local `fsmgen` reference checkout at submodule revision `57f00e581b4fc9a2aa02318846d1eb8a726c8960`
- updated the live documentation surface so the repo map and continuity notes now treat `subs/fsmgen` as the local `.fsm` reference implementation for upcoming adapter work
- recorded the workflow rule that `subs/fsmgen` is contextual and read-only inside `specforge`
- established the local upstream bug-report ID format `FSMGEN-BUG-####` for any future `fsmgen` misbehavior found during adapter work
- no new `fsmgen` misbehavior was identified in this slice, so no local `FSMGEN-BUG-####` report was filed yet

## 2026-03-31
- initialized the `specforge` Git repository
- established the initial live documentation surface:
  - `README.md`
  - `SESSION_BOOTSTRAP.md`
  - `ROADMAP.md`
  - `LIVE_ACHIEVEMENT_STATUS.md`
  - `RUST_CODEBASE_ANALYSIS.md`
  - `USER_GUIDE.md`
  - `DEVELOPMENT_NOTES.md`
  - `CHANGES.md`
  - `MEMORY.md`
- defined `README.md` as the single project entry point
- recorded the working project/binary naming:
  - project: `specforge`
  - CLI: `spec2fsm`
- recorded the staged-tool architecture direction and the initial Rust architecture baseline
- updated `COMMIT.md` to reinforce live-document continuity requirements during long-running tasks
- added `.gitignore` rules so local workflow files and build artifacts remain untracked
- created the initial Rust workspace and bootstrap CLI
- established the initial continuity workflow and live-doc surface
- created the first repository baseline commit
