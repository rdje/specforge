# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even while only the early IR stages are implemented

## Executive summary
- the repository now contains a renamed `specforge` crate and CLI rather than the older `spec2fsm` identity
- the canonical product boundary is now `IntentIR`, not `.fsm`
- the codebase has been reshaped around an explicit staged IR pipeline:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - adapters
- the implemented executable stages today are `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
- `SourceIR` now has a real Docling-backed PDF materialization path that emits promoted markdown, page artifacts, visual assets, metadata JSON, and backend raw JSON
- `EvidenceIR` now has a real builder that emits typed multimodal evidence records instead of remaining text-only scaffolding
- `SemanticIR` now has a real builder that lifts grounded evidence into inspectable semantic records and residual decisions
- `IntentIR` now has a real builder that canonicalizes semantic records into inspectable backend-neutral intent artifacts
- the first `.fsm` adapter slices now materialize typed adapter artifacts and can emit honest standalone `?dt:name`, structured `?fsm:name`, and explicit `?top:name` text when the canonical facts are complete, including canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, and compound-update shorthand from the widened semantic model
- the canonical reset model now preserves reset kind, polarity, assertion timing, release timing, and reset-target semantics explicitly, and the `.fsm` adapter keeps the reduced target surface honest by requiring reset polarity to remain recoverable from the reset signal name
- the current adapter root-kind model is now intentionally limited to `?dt:name`, `?fsm:name`, and `?top:name`; compatibility-level `?mod:name` / `?module:name` spellings remain outside that model until a real backend-neutral direct-module distinction exists
- the next slice should build the validation/back-annotation pipeline on top of the now-stable staged IR and `.fsm` adapter surfaces

## Observed current state
### Repository contents directly observed
- `.git/`
- `.gitmodules`
- live documentation surface
- `INTENTIR_SPEC.md`
- `Cargo.toml`
- `Cargo.lock`
- `crates/specforge/Cargo.toml`
- `crates/specforge/src/main.rs`
- `crates/specforge/src/lib.rs`
- `crates/specforge/src/cli.rs`
- `crates/specforge/src/error.rs`
- `crates/specforge/src/commands/inspect.rs`
- `crates/specforge/src/commands/ingest.rs`
- `crates/specforge/src/commands/evidence.rs`
- `crates/specforge/src/commands/semantic.rs`
- `crates/specforge/src/commands/intent.rs`
- `crates/specforge/src/commands/adapt.rs`
- `crates/specforge/src/ir/mod.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/src/ir/source/docling_backend.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/adapters.rs`
- `subs/fsmgen/`

### Rust-specific contents still absent
- no dedicated `specforge-source` crate
- no dedicated `specforge-evidence` crate
- no dedicated `specforge-semantic` crate
- no dedicated `specforge-intent` crate
- no dedicated `specforge-adapters` crate
- no dedicated validation crate
- no integration-test harness beyond crate-local unit tests
- no additional real builders beyond the current `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR` slices and the first `.fsm` adapter slice

### Immediate implication
- the codebase now has a clean top-level architectural story
- the current risk is no longer naming confusion; it is execution lag between the declared staged architecture and the still-limited implemented builders

## What the tool needs to do
- build `SourceIR` from raw specifications and normalized artifacts
- build `EvidenceIR` from normalized markdown, page assets, figures, captions, and evidence extraction
- build `SemanticIR` from actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- build canonical `IntentIR` as a backend-independent intent model
- lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- validate stage outputs and adapter outputs and back-annotate findings

## Current implemented architecture
### Root workspace
- `Cargo.toml`
  - workspace root
- `Cargo.lock`
  - dependency lockfile

### Active crate
- `crates/specforge`
  - single user-facing CLI crate and binary for the current slice

### Implemented module boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch and public module exports
- `src/cli.rs`
  - clap-based command model using the `specforge` binary name
- `src/error.rs`
  - typed error/result boundary
- `src/commands/inspect.rs`
  - deterministic source/path inspection command
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
  - stage identifiers for `source_ir`, `evidence_ir`, `semantic_ir`, and `intent_ir`
- `src/ir/source.rs`
  - concrete `SourceIR` implementation and PDF materialization lifecycle
- `src/ir/source/docling_backend.rs`
  - external backend discovery, Docling command orchestration, and the embedded Python helper for structured PDF normalization
- `src/ir/evidence.rs`
  - concrete `EvidenceIR` builder, markdown parsing, caption/reference linking, and extraction heuristics
- `src/ir/semantic.rs`
  - concrete `SemanticIR` builder, semantic lifting heuristics, and residual-decision generation
- `src/ir/intent.rs`
  - concrete `IntentIR` builder, canonicalization heuristics, and residual-decision preservation
- `src/ir/adapters.rs`
  - typed adapter artifacts, DT-centric `.fsm` lowering logic, renderability gating, and adapter-side residual-decision generation

## Assessment of current structure
### What is good
- the crate/binary identity now matches the repo direction
- the code no longer hardcodes `.fsm` as the conceptual endpoint
- a real typed `SourceIR` artifact exists instead of a handwritten ingest plan
- a real structured PDF normalization path now exists inside `SourceIR`, so the first stage is operational for both Markdown and PDF inputs
- a real typed `EvidenceIR` artifact now exists, so the staged pipeline is operational beyond raw source normalization
- a real typed `SemanticIR` artifact now exists, so the staged pipeline now reaches a backend-neutral semantic layer before the final canonicalization stage
- a real typed `IntentIR` artifact now exists, so the end-to-end source-to-intent pipeline is operational before adapter lowering
- the later stages have typed names and module homes, which reduces the risk of accidental backend-first growth
- adapter lowering is separated from the canonical IR stages
- the IR surface now carries page and visual manifests plus backend source references that later stages can ground against
- the repository now also contains a pinned local `fsmgen` checkout, which gives the next `.fsm` adapter slice a nearby reference implementation without changing the canonical `IntentIR` boundary
- that `fsmgen` checkout is now explicitly contextual and read-only from the `specforge` side; any observed upstream misbehavior should be captured as a local `FSMGEN-BUG-####` report instead of a submodule edit
- the first `.fsm` adapter slice already enforces honest renderability boundaries instead of fabricating target text from under-specified intent
- the canonical model now preserves typed signal inventory and backend-neutral guarded/action control fragments before the adapter boundary
- the canonical model now also preserves backend-neutral system contract and init-assignment records for explicit standalone sequential control, including first-class reset polarity/assertion/release/target semantics
- the canonical model now also preserves explicit regular-state and transition records for stateful lowering
- the canonical model now also preserves canonical symbol-definition sections and structured control blocks, including dedicated synchronous-reset and asynchronous-reset control roles
- the `.fsm` adapter can now emit a real standalone `?dt:name` file for explicit combinational and sequential DT cases, canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, compound-update shorthand, and a real structured `?fsm:name` file for explicit state-graph cases when those canonical facts are explicit enough

### What is still insufficient
- only `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR` have real builders today
- deeper visual enrichment beyond caption/reference grounding is not implemented yet
- the current renderable `.fsm` slices are intentionally narrow: they handle explicit standalone combinational/sequential DT control, canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, compound-update shorthand, explicit structured FSM-root cases, and the first explicit top-root composition slice, while unsupported selector/predicate shapes stay deferred and compatibility-level direct-module spellings remain outside the current canonical root-kind model
- validation/back-annotation is still absent

## Architectural recommendation
### Core architectural stance
- keep `IntentIR` as the canonical endpoint
- keep adapters downstream of `IntentIR`
- keep the internal system of record typed and stage-specific
- keep residual decisions explicit at every stage
- do not let convenience around one backend contaminate the stage-neutral model
- use structured parsing first and selective multimodal enrichment second, rather than collapsing the problem into markdown-only OCR or ungrounded VLM generation

### Recommended growth path from the current codebase
#### Keep in the current crate for one more slice
- start the validation/back-annotation pipeline for staged IR and adapter artifacts
- keep compatibility-level `?mod:name` / `?module:name` spellings outside the adapter root-kind model until a real backend-neutral direct-module distinction exists
- keep any new composition/control enrichment backend-neutral so the canonical model boundary stays intact

#### Split into dedicated crates when pressure becomes real
- `specforge-source`
  - source registration, normalization, converter orchestration
- `specforge-evidence`
  - section anchors, evidence spans, statement extraction and provenance
- `specforge-semantic`
  - actor and semantic lifting
- `specforge-intent`
  - canonical intent model and versioned serialization
- `specforge-adapters`
  - target-specific lowerings
- `specforge-validate`
  - validation, diagnostics, back-annotation

## Mapping from staged architecture to the current modules
### SourceIR
- current primary ownership:
  - `src/commands/ingest.rs`
  - `src/ir/source.rs`

### EvidenceIR
- current declared ownership:
  - `src/commands/evidence.rs`
  - `src/ir/evidence.rs`
- current executable behavior:
  - builds `EvidenceIR` from ready `SourceIR`, promoted markdown, and visual-asset manifests

### SemanticIR
- current declared ownership:
  - `src/commands/semantic.rs`
  - `src/ir/semantic.rs`
- dependency:
  - requires real `EvidenceIR`
- current executable behavior:
  - builds `SemanticIR` from persisted `EvidenceIR`, deriving actors, interfaces, typed signal records, backend-neutral control fragments, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions

### IntentIR
- current declared ownership:
  - `src/commands/intent.rs`
  - `src/ir/intent.rs`
- dependency:
  - requires real `SemanticIR`
- current executable behavior:
  - builds `IntentIR` from persisted `SemanticIR`, deriving intent identity, actor responsibilities, interface inventory, backend-neutral control fragments, behaviors, constraints, assumptions, and residual decisions

### Adapters
- current declared ownership:
  - `src/commands/adapt.rs`
  - `src/ir/adapters.rs`
- dependency:
  - requires stable `IntentIR`
- nearby reference implementation:
  - `subs/fsmgen/`
- local workflow rule:
  - treat `subs/fsmgen` as read-only contextual input
  - if upstream behavior looks wrong, file a local tracked bug report under `FSMGEN-BUG-####` rather than patching the submodule here
- current executable behavior:
  - builds a typed `.fsm` adapter artifact from persisted `IntentIR`
  - chooses `?dt:name` for explicit standalone DT cases, `?fsm:name` when explicit regular-state and transition records are present, and `?top:name` when explicit module/top composition facts are present
  - consumes canonical signal inventory, backend-neutral system/init records, backend-neutral control fragments, explicit regular-state/transition records, and explicit module/top composition facts when present
  - emits a real standalone `?dt:name` file only when widths, directions, guarded/action blocks, and any required standalone sequential system/init facts are explicit enough to avoid semantic invention
  - emits a real structured `?fsm:name` file only when the state graph, transition targets, and state-body control are explicit enough to avoid semantic invention
  - emits a real explicit `?top:name` source document only when the top ports, child modules, and links are explicit enough to avoid semantic invention
- next real implementation target:
  - start validation/back-annotation on the current staged IR and `.fsm` adapter artifact surface

## Major risks
### Risk: backend leakage into IntentIR
- if `.fsm` or RTL-specific assumptions creep back into the canonical model, the pivot fails even if the names remain correct

### Risk: stage scaffolding without stage execution
- if `EvidenceIR`, `SemanticIR`, and `IntentIR` remain only structs for too long, the architecture becomes performative rather than operational

### Risk: incomplete provenance in EvidenceIR
- if evidence ranges are not carried forward precisely, later semantic lifting and validation will be fragile

### Risk: markdown-only drift for PDFs
- if the real builder treats markdown as the only normalized representation, the system will silently lose figure, chart, and layout semantics before `EvidenceIR`

### Risk: ungrounded visual descriptions
- if multimodal descriptions are generated without stable links back to page regions, captions, and source references, later stages will be vulnerable to hallucinated evidence
### Risk: mixed Rust/Python backend seam
- the SourceIR PDF path now depends on a Rust-to-Python orchestration boundary and an external Docling runtime
- interpreter discovery, package installation, and first-run model downloads are operational concerns that must stay explicit in docs and tests
- this is acceptable temporarily, but should be closed soon so the first stage is truly operational for PDFs

## Testing implications
- current tests cover:
  - source-kind detection
  - deterministic source key naming
  - `SourceIR` JSON materialization
  - directory residual decision emission
  - PDF normalization planning
  - PDF materialization through a stubbed backend override that exercises the manifest-writing path
  - markdown-backed `EvidenceIR` construction
  - caption and figure-reference grounding into visual evidence
  - handshake-driven `SemanticIR` actor/interface/invariant extraction
  - ambiguous visual-grounding residual decisions in `SemanticIR`
  - handshake-driven `IntentIR` identity/behavior/constraint/assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
- next tests should cover:
  - structured PDF normalization failure handling against missing runtimes and malformed backend output
  - additional `EvidenceIR` extraction on richer visual-asset fixtures
  - figure extraction and caption-linking fidelity across ambiguous numbering cases
  - visual evidence grounding and confidence propagation beyond the current heuristic pass
  - provenance retention across stage boundaries
  - richer `SemanticIR` snapshots and residual-decision coverage on protocol-heavy fixtures
  - richer `IntentIR` snapshots and canonicalization coverage on protocol-heavy fixtures
  - wider `.fsm` renderability coverage and snapshot stability, especially direct-module alias and broader composition-root cases
  - future adapter targets beyond the first `.fsm` slice

## Validation completed in this session
- `cargo fmt --all --manifest-path Cargo.toml`
  - passed after the first-class reset-contract widening
- `cargo fmt --all --manifest-path Cargo.toml --check`
  - passed after the documentation refresh, confirming the Rust workspace still formats cleanly
- `cargo test --manifest-path Cargo.toml`
  - passed for the widened reset-contract slice with 44 tests passing
- `cargo run --manifest-path Cargo.toml -- ingest <temp>/inferred_reset_cli.md`
  - passed and started a temporary inferred-polarity reset CLI pipeclean for the widened reset-contract slice
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/inferred_reset_cli/source_ir.json`
  - passed and preserved the inferred reset fixture into `EvidenceIR`
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/inferred_reset_cli/evidence_ir.json`
  - passed and materialized `SemanticIR` with an asynchronous active-low reset contract inferred from `rst_n`, `assertion_timing: asynchronous_to_clock`, `release_timing: synchronous_to_clock`, `target_kind: dedicated_reset_pin`, and `automation_confidence: medium`
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/inferred_reset_cli/semantic_ir.json`
  - passed and carried the widened reset contract unchanged into `IntentIR`
- `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/inferred_reset_cli/intent_ir.json --target fsm`
  - passed and materialized a renderable `.fsm` adapter artifact with `lowering_status: renderable`, `selected_root_kind: dt`, and emitted `generated/adapters/fsm/inferred_reset_cli/inferred_reset_cli.fsm`
- `cargo run -p specforge -- --help`
  - passed and reports the staged `SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, and adapter direction
- `cargo run -p specforge -- ingest README.md --dry-run`
  - passed and emits `SourceIR` JSON with parser backend, page-artifact manifests, visual-asset manifests, placeholder bindings, and downstream stage planning fields
- `cargo run -p specforge -- ingest README.md`
  - passed after the PDF materialization changes, confirming the markdown execute path still behaves correctly
- `SPECFORGE_DOCLING_PYTHON=/tmp/specforge-docling-venv/bin/python cargo run -p specforge -- ingest /tmp/specforge-docling-sample.pdf`
  - passed and materialized a real PDF `SourceIR` with 9 page artifacts, 11 visual assets, promoted markdown, metadata JSON, backend raw JSON, and manifest files
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - passed and previewed a markdown-backed `EvidenceIR`
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - passed and materialized a markdown-backed `EvidenceIR` with 14 section anchors, 167 evidence spans, and 167 extracted statements
- `cargo run -p specforge -- evidence generated/source_ir/specforge_docling_sample/source_ir.json`
  - passed and materialized a PDF-backed `EvidenceIR` with 18 section anchors, 225 evidence spans, 11 visual evidence items, 19 evidence links, and 225 extracted statements
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - passed and previewed a markdown-backed `SemanticIR`
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - passed and materialized a markdown-backed `SemanticIR` with 2 actors, 4 phases, 3 invariants, 3 gates, 1 abstraction, and 12 decomposition candidates
- `cargo run -p specforge -- semantic generated/evidence_ir/specforge_docling_sample/evidence_ir.json`
  - passed and materialized a PDF-backed `SemanticIR` with 2 actors, 22 interfaces, 7 phases, 17 invariants, 2 contracts, 20 gates, 12 decomposition candidates, and 2 residual decisions
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - passed and previewed a markdown-backed `IntentIR`
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - passed and materialized a markdown-backed `IntentIR` with 2 actors, 7 behaviors, 3 constraints, 1 assumption, and 0 residual decisions
- `cargo run -p specforge -- intent generated/semantic_ir/specforge_docling_sample/semantic_ir.json`
  - passed and materialized a PDF-backed `IntentIR` with 2 actors, 28 behaviors, 24 constraints, 1 assumption, and 2 residual decisions
- `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/handshake/intent_ir.json --target fsm`
  - passed and materialized a typed `.fsm` adapter artifact with `lowering_status: blocked`, `selected_root_kind: dt`, 2 signal candidates, 1 DT candidate, and 3 adapter residual decisions
- `cargo run --manifest-path Cargo.toml -- ingest <temp>/comb_dt.md`
  - passed and started a temporary explicit-control CLI pipeclean for the first renderable standalone `.fsm` slice
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/comb_dt/source_ir.json`
  - passed and preserved the explicit-control fixture into `EvidenceIR`
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/comb_dt/evidence_ir.json`
  - passed and materialized `SemanticIR` with one typed interface and two canonical control fragments
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/comb_dt/semantic_ir.json`
  - passed and carried the canonical interface/control surface into `IntentIR`
- `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/comb_dt/intent_ir.json --target fsm`
  - passed and materialized a renderable `.fsm` adapter artifact with `lowering_status: renderable`, `selected_root_kind: dt`, 3 signal candidates, 1 DT candidate, 2 residual decisions, and an emitted `generated/adapters/fsm/comb_dt/comb_dt.fsm`
- `cargo run --manifest-path Cargo.toml -- ingest <temp>/seq_dt.md`
  - passed and started a temporary explicit sequential-control CLI pipeclean for the standalone sequential `.fsm` slice
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/seq_dt/source_ir.json`
  - passed and preserved the explicit sequential-control fixture into `EvidenceIR`
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/seq_dt/evidence_ir.json`
  - passed and materialized `SemanticIR` with one typed interface, an explicit system contract, one init assignment, and one sequential DT fragment
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/seq_dt/semantic_ir.json`
  - passed and carried the canonical interface/system/init/control surface into `IntentIR`
- `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/seq_dt/intent_ir.json --target fsm`
  - passed and materialized a renderable `.fsm` adapter artifact with `lowering_status: renderable`, `selected_root_kind: dt`, 4 signal candidates, 1 DT candidate, 4 residual decisions, and an emitted `generated/adapters/fsm/seq_dt/seq_dt.fsm`
- `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_fsm.md`
  - passed and started a temporary explicit FSM-root CLI pipeclean for the structured `?fsm:name` slice
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_fsm/source_ir.json`
  - passed and preserved the explicit FSM fixture into `EvidenceIR`
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_fsm/evidence_ir.json`
  - passed and materialized `SemanticIR` with one typed interface, explicit regular states, explicit transitions, and one standalone guarded DT fragment
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_fsm/semantic_ir.json`
  - passed and carried the canonical interface/system/init/control/state/transition surface into `IntentIR`
- `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_fsm/intent_ir.json --target fsm`
  - passed and materialized a renderable `.fsm` adapter artifact with `lowering_status: renderable`, `selected_root_kind: fsm`, 7 signal candidates, 1 DT candidate, 2 state candidates, 2 transition candidates, 2 residual decisions, and an emitted `generated/adapters/fsm/explicit_fsm/explicit_fsm.fsm`
- `cargo run --manifest-path Cargo.toml -- ingest <temp>/explicit_top.md`
  - passed and started a temporary explicit top-composition CLI pipeclean for the first `?top:name` slice
- `cargo run --manifest-path Cargo.toml -- evidence generated/source_ir/explicit_top/source_ir.json`
  - passed and preserved the explicit top-composition fixture into `EvidenceIR`
- `cargo run --manifest-path Cargo.toml -- semantic generated/evidence_ir/explicit_top/evidence_ir.json`
  - passed and materialized `SemanticIR` with explicit module and top-composition records
- `cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/explicit_top/semantic_ir.json`
  - passed and carried the explicit module/top composition surface into `IntentIR`
- `cargo run --manifest-path Cargo.toml -- adapt generated/intent_ir/explicit_top/intent_ir.json --target fsm`
  - passed and materialized a renderable `.fsm` adapter artifact with `lowering_status: renderable`, `selected_root_kind: top`, 2 module candidates, 1 top candidate, 3 residual decisions, and an emitted `generated/adapters/fsm/explicit_top/datapath.fsm`
- repo-wide stale-name sweep
  - remaining `spec2fsm` references are historical notes only, not active CLI or architecture surfaces
- `cargo fmt --all --manifest-path Cargo.toml --check` and `cargo test --manifest-path Cargo.toml`
  - passed after the full widened reset contract, selector/test-node, compound-update, and direct-module decision slice with 47 tests passing (commit `27511f840d5b8f4e8a45f772b97602471aea5828`)

## Current recommendation
- keep the current single-crate workspace for one more slice
- next, start the validation/back-annotation pipeline so stage and adapter outputs have reproducible artifact-linked reports
- keep `IntentIR` canonical and resist any temptation to make `.fsm` the hidden endpoint again
