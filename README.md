# SpecForge
This file is the single entry point for the project.
Use it first for the project objective, document navigation, and the current implementation map.

## Project objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from protocol, component, system, and software-interface specifications
- make the canonical deliverable a backend-independent `IntentIR`, serialized as JSON or a future equivalent interchange format
- treat `.fsm`, SystemVerilog, Verilog, and VHDL as adapter targets downstream of `IntentIR`, not as the core product boundary
- push automation as far as safely possible, while representing unresolved ambiguity as structured residual decision packets instead of ad hoc manual gaps
- preserve crash-safe continuity through live documentation so a new AI or LLM session can resume work quickly and correctly
- implement protocol semantics as a typed domain model plus evidence aggregation, using AI only as a bounded hypothesis generator rather than as an end-to-end black-box reader

## Current repository state
- the live-document surface has been pivoted around `IntentIR` as the canonical endpoint
- the Rust workspace and active CLI/crate identity are now `specforge`
- the current `specforge` CLI surface supports:
  - `inspect <path>`
  - `converge <source> --target fsm`
  - `ingest <source> --dry-run`
  - `ingest <source>`
  - `evidence <source-ir> --dry-run`
  - `evidence <source-ir>`
  - `semantic <evidence-ir> --dry-run`
  - `semantic <evidence-ir>`
  - `intent <semantic-ir> --dry-run`
  - `intent <semantic-ir>`
  - `adapt <intent-ir> --target fsm --dry-run`
  - `adapt <intent-ir> --target fsm`
  - `project-validation <artifact>...`
- `specforge ingest` now computes and materializes `SourceIR` at `generated/source_ir/<document_key>/source_ir.json`
- `specforge evidence` now computes and materializes `EvidenceIR` at `generated/evidence_ir/<document_key>/evidence_ir.json`
- `specforge semantic` now computes and materializes `SemanticIR` at `generated/semantic_ir/<document_key>/semantic_ir.json`
- `specforge intent` now computes and materializes `IntentIR` at `generated/intent_ir/<document_key>/intent_ir.json`
- `specforge adapt --target fsm` now computes and materializes typed adapter artifacts at `generated/adapters/fsm/<document_key>/adapter.json`
- `specforge converge <source> --target fsm` now ingests once, runs Ollama-backed VLM figure enrichment and NLP Level 3 by default, rebuilds the downstream IR stages, and stops when the persisted knowledge snapshot is stable across passes; use `--vlm-provider skip` and/or `--nlp-provider skip` only when you explicitly want a narrower run
- the intended architecture is not "teach code to understand unrestricted English"; it is "teach the pipeline to recover typed protocol facts from multimodal evidence and validate them aggressively"
- `specforge validate <artifact>` now writes a deterministic stage-local `validation_report.json` sidecar and backannotates the latest report into the artifact's `validation_reports` field
- `specforge project-validation <artifact>...` now validates the passed artifacts and refreshes the tracked validation snapshot docs from their persisted reports
- `SemanticIR` and `IntentIR` now preserve the structural KG downstream via `actor_signal_relations`, `actor_ports`, and `signal_connectivity`, while keeping flat `direction_hint` fields only as a compatibility surface
- `SemanticIR` and `IntentIR` now also carry `interface_signal_conflicts` so conflicting direction/width evidence for the same signal stays explicit instead of only collapsing the canonical hint to `None`
- `SemanticIR` and `IntentIR` now also carry `signal_connectivity_conflicts` so unresolved multi-producer structural ambiguity stays explicit in the canonical layers instead of hiding inside raw connectivity vectors
- `SemanticIR` and `IntentIR` now also carry an initial typed `temporal_rules` surface so clocked behavior is no longer represented only as free-form timing text and prose constraints
- that temporal surface now also recovers bounded `cycle_window` latency from prose/timing text like `within 2 cycles`, `next cycle`, `next tick`, and `next rising edge` instead of leaving every temporal rule unbounded
- ready/valid completion guards like `AWVALID is HIGH and AWREADY is HIGH` now also surface as typed `HandshakeComplete` temporal predicates instead of surviving only as two unrelated scalar guard clauses
- signal-description tables now contribute typed `signal_semantic_hints`, and prose plus alias-grounded prose descriptions can now contribute the same role hints too; `SemanticIR` / `IntentIR` carry those per-signal `semantic_tags`, so handshake completion can be recognized from request/accept meaning even when the signal spellings are not literally `*VALID*` / `*READY*`
- contested semantic arbitration now blocks literal handshake-name fallback, so a signal like `XVALID` no longer auto-becomes valid-like when the preserved evidence still disagrees about its meaning
- that blocked fallback is now surfaced explicitly too: `SemanticIR` emits a residual decision packet and validation reports the affected handshake-shaped signals instead of leaving the withheld promotion implicit
- typed handshake-role recovery is now stricter too: `SemanticIR` only lets observation-backed semantic consensus drive canonical handshake roles, and handshake-shaped signals with fallback-only provisional roles now block raw name fallback instead of quietly regaining handshake semantics through spelling alone
- visual captions and grounded VLM timing-diagram annotations can now contribute the same `signal_semantic_hints` too, with explicit visual-evidence provenance, so meaning-based role inference has an initial multimodal path rather than depending only on tables and prose
- `SemanticIR` and `IntentIR` now also preserve per-signal `semantic_observations` with source kind and provenance, so canonical consumers no longer have to rely on lossy merged `semantic_tags` alone
- `SemanticIR` and `IntentIR` now also preserve explicit per-signal `semantic_candidates`, so competing role hypotheses remain inspectable in the canonical layers instead of being flattened into only a winner-or-none outcome
- `SemanticIR` and `IntentIR` now also preserve explicit per-signal `semantic_arbitration`, so the current lead role, runner-up, evidence margin, and decisive-vs-contested status stay inspectable without forcing an unsafe canonical winner when multiple role candidates remain
- `SemanticIR` and `IntentIR` now also resolve a canonical per-signal semantic role plus modality-aware `semantic_grounding_strength`, so handshake-role consumers can prefer provenance-backed role consensus and validation can distinguish single-source grounding, same-modality repetition, and true cross-modality reinforcement
- `SemanticIR` and `IntentIR` now also carry a canonical `semantic_consensus` summary per resolved signal role, including the supporting source kinds, supporting observation count, and strongest supporting automation confidence, and validation flags any resolved role still lacking that richer consensus profile
- fallback-only resolved semantic roles no longer stay as validator-only drift: `SemanticIR` now emits an explicit residual decision when a role is still resolved without observation-backed consensus, and `IntentIR` adds a matching assumption so provisional role meaning stays inspectable in the canonical layers
- semantic-role hints from prose/visual text are now scrubbed of explicit signal identifiers before tag inference, so declarations like `Signal AWVALID is input width 1.` no longer create role consensus from the signal name alone; the surrounding descriptive language now has to carry the meaning
- prose statements and visual captions can now also ground different semantic roles for different signals from the same document region by extracting clause-local per-signal context windows instead of requiring the whole text to resolve to exactly one target
- contradictory semantic-role evidence for the same signal now surfaces explicitly as `signal_semantic_conflicts` in `EvidenceIR` and is reported by validation instead of disappearing into an ambiguous dual-tag fallback
- `SemanticIR` and `IntentIR` now also carry those `signal_semantic_conflicts`, so canonical consumers and validation can still see unresolved role disagreement instead of letting it disappear after the evidence stage
- and it now reuses unique KG producers to emit actor-grounded drive predicates for value-timed rules, so temporal semantics can point back to who actually drives the signal
- stable/hold rules now also keep that actor responsibility via actor-grounded stability predicates instead of flattening every producer obligation into a signal-only invariant
- compound `when/if` guards like `HREADY is LOW and HSEL is HIGH` now survive into the typed temporal layer as multiple antecedent predicates instead of being flattened into a single partial condition
- contradictory temporal value obligations under the same grounded context now surface as typed `temporal_conflicts` records instead of staying implicit in the rule set
- the convergent `EvidenceIR` loop now also mines `SignalDescription` tables for active-high/active-low facts, merges them conservatively with prose polarity before refining asserted/deasserted constraints, and persists explicit `signal_polarity_conflicts` records when prose/table evidence disagrees so validation can report the conflict instead of hiding it inside a neutral fallback
- a pinned `subs/fsmgen` git submodule now exists as a local `.fsm` reference implementation for upcoming adapter work
- the `SourceIR` schema now reserves:
  - parser-backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- PDF normalization is now explicitly treated as structured source capture with page images, figure/table assets, captions, and markdown as a convenience view rather than the sole system of record
- `specforge ingest <pdf>` now performs real Docling-backed structured normalization and materializes promoted markdown, page images, page metadata sidecars, visual assets, metadata JSON, backend raw JSON, and manifest files under `generated/source_ir/<document_key>/normalized`
- the first real `EvidenceIR` extraction pass now builds section anchors, evidence spans, visual evidence items, figure/caption links, and heuristic extracted statements from ready `SourceIR` artifacts
- the first real `SemanticIR` lifting pass now builds actors, interfaces, backend-neutral system/init records, first-class reset polarity/assertion/release/target semantics, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions from persisted `EvidenceIR` artifacts; boilerplate section filtering and markdown signal-table row parsing ensure the extraction is useful on real chip spec PDFs
- the first real `IntentIR` canonicalization pass now builds intent identity, actor responsibilities, carried interface/control/system/init surface, behaviors, constraints, assumptions, and residual decisions from persisted `SemanticIR` artifacts
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - typed adapter lowering
- the first real `.fsm` adapter slices now materialize typed adapter artifacts, emit explicit standalone `?dt:name` text for honest canonical DT cases, emit structured `?fsm:name` text when the canonical state graph is explicit, emit explicit `?top:name` source documents when module/top composition facts are explicit, lower canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, and compound-update shorthand from the widened semantic model when those canonical shapes map directly into `.fsm`, keep reset polarity honest through the reset signal name because emitted `.fsm` text still carries only `sreset` / `asreset` plus the signal, keep unsupported selector predicates and other unsafe broader-root cases blocked with explicit residual decisions instead of fabricating target syntax, and intentionally keep compatibility-level `?mod:name` / `?module:name` spellings outside the current canonical root-kind model until a real backend-neutral direct-module distinction exists
- the next implementation milestone is semantic-truthfulness hardening: finish the remaining graph-first direction migration, broaden the new meaning-based role inference beyond signal-description tables plus initial prose/alias grounding into richer multimodal grounding, deepen the temporal-rule surface into richer temporal arbitration across modalities and actors, add KG-guided rescans and evidence arbitration, and treat adapter expansion as horizon work until the canonical four-layer pipeline is top-notch

## Working naming
- repository / project / CLI / crate name: `specforge`
- canonical output: `IntentIR`
- adapter targets:
  - `.fsm`
  - SystemVerilog
  - Verilog
  - VHDL

## Fast ramp-up order
1. `README.md`
2. `SESSION_BOOTSTRAP.md`
3. `INTENTIR_SPEC.md`
4. `ROADMAP.md`
5. `LIVE_ACHIEVEMENT_STATUS.md`
6. `VALIDATION_SNAPSHOT.md`
7. `RUST_CODEBASE_ANALYSIS.md`
8. `USER_GUIDE.md`
9. `DEVELOPMENT_NOTES.md`
10. `CHANGES.md`
11. `MEMORY.md`
12. `COMMIT.md`

## Documentation index
- `README.md`
  - single project entry point and navigation hub
- `SESSION_BOOTSTRAP.md`
  - exact fresh-session instruction for a new AI or LLM instance
- `INTENTIR_SPEC.md`
  - canonical product and stage specification for `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- `ROADMAP.md`
  - live roadmap for project objectives, sequencing, and remaining work
- `LIVE_ACHIEVEMENT_STATUS.md`
  - authoritative live progress snapshot using the project status vocabulary
- `VALIDATION_SNAPSHOT.md`
  - tracked projection of the latest persisted validation reports into a crash-safe markdown summary
- `RUST_CODEBASE_ANALYSIS.md`
  - live deep-dive analysis of the Rust codebase and its architecture
- `USER_GUIDE.md`
  - end-user oriented guide to how the tool is expected to work
- `DEVELOPMENT_NOTES.md`
  - engineering rationale, design choices, and implementation context
- `CHANGES.md`
  - full detailed summary of the current set of changes
- `MEMORY.md`
  - compact but actionable continuity record for crash/session-loss recovery
- `COMMIT.md`
  - exact commit workflow and commit-time reporting requirements
- `.gitmodules`
  - git submodule manifest for pinned local reference dependencies

## Project file and directory map
### Current workflow and documentation paths
- `README.md`
- `SESSION_BOOTSTRAP.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `VALIDATION_SNAPSHOT.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `USER_GUIDE.md`
- `DEVELOPMENT_NOTES.md`
- `CHANGES.md`
- `MEMORY.md`
- `COMMIT.md`
- `.gitignore`
- `.gitmodules`

### Local-only workflow paths
- `git_message_brief.txt`
  - short commit-message file used by the commit workflow
  - must remain untracked
- `questions_keep_untracked.txt`
  - local user backlog/questions scratch file
  - must remain untracked
- `trace.log`
  - optional trace/log output
  - must remain untracked
- `target/`
  - Rust build output
  - must remain untracked

### Reference adapter/tooling paths
- `subs/fsmgen/`
  - pinned local checkout of `fsmgen`, used as read-only contextual reference while building the first `.fsm` adapter
  - do not modify it from this repository; if upstream misbehavior is discovered, track it locally as a bug report instead

### Current Rust implementation paths
- `Cargo.toml`
  - root Rust workspace manifest
- `Cargo.lock`
  - dependency lockfile
- `crates/specforge/Cargo.toml`
  - active CLI crate manifest
- `crates/specforge/src/main.rs`
  - binary entrypoint
- `crates/specforge/src/lib.rs`
  - top-level command dispatch
- `crates/specforge/src/cli.rs`
  - clap-based CLI model
- `crates/specforge/src/error.rs`
  - typed error boundary
- `crates/specforge/src/commands/inspect.rs`
  - source/path inspection command
- `crates/specforge/src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `crates/specforge/src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `crates/specforge/src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `crates/specforge/src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `crates/specforge/src/commands/adapt.rs`
  - `.fsm` adapter preview/materialization command for the current honest DT/FSM/top lowering slices
- `crates/specforge/src/commands/project_validation.rs`
  - validation snapshot projection command for tracked live docs
- `crates/specforge/src/ir/mod.rs`
  - staged IR namespace and stage identifiers
- `crates/specforge/src/ir/source.rs`
  - `SourceIR` types, parser-backend selection, page/visual artifact manifests, and ingest-side residual decisions
- `crates/specforge/src/ir/source/docling_backend.rs`
  - Docling backend orchestration and the embedded Python helper that materializes structured PDF artifacts for `SourceIR`
- `crates/specforge/src/ir/evidence.rs`
  - first real multimodal `EvidenceIR` builder for text spans, captions, figure/table references, visual evidence, and extracted statements
- `crates/specforge/src/ir/semantic.rs`
  - first real `SemanticIR` builder for actors, interfaces, typed signal records, backend-neutral control fragments, explicit module/top composition facts, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions
- `crates/specforge/src/ir/intent.rs`
  - first real `IntentIR` builder for canonical intent identity, actor responsibilities, carried interface inventory, carried backend-neutral control fragments, carried explicit module/top composition facts, behaviors, constraints, assumptions, and residual decisions
- `crates/specforge/src/ir/adapters.rs`
  - typed adapter artifacts, honest standalone/structured/top-root `.fsm` lowering logic, renderability analysis, and adapter-side residual decisions

### Planned future implementation paths
- `fixtures/`
  - sample specifications, PDFs, markdown conversions, and expected IR snapshots
- `examples/`
  - example invocations and example stage outputs
- `generated/`
  - local generated IR artifacts; git-ignored by default

## Quick start
```bash
cargo test
cargo run -p specforge -- --help
cargo run -p specforge -- inspect README.md
cargo run -p specforge -- converge README.md --target fsm
cargo run -p specforge -- ingest README.md --dry-run
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run
cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run
cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run
cargo run -p specforge -- project-validation generated/intent_ir/readme/intent_ir.json
```
- `specforge ingest <source> --dry-run` prints computed `SourceIR` JSON without writing artifacts
- `specforge ingest <source>` materializes `generated/source_ir/<document_key>/source_ir.json`
- `specforge evidence <source-ir> --dry-run` prints computed `EvidenceIR` JSON without writing artifacts
- `specforge evidence <source-ir>` materializes `generated/evidence_ir/<document_key>/evidence_ir.json`
- `specforge semantic <evidence-ir> --dry-run` prints computed `SemanticIR` JSON without writing artifacts
- `specforge semantic <evidence-ir>` materializes `generated/semantic_ir/<document_key>/semantic_ir.json`
- `specforge intent <semantic-ir> --dry-run` prints computed `IntentIR` JSON without writing artifacts
- `specforge intent <semantic-ir>` materializes `generated/intent_ir/<document_key>/intent_ir.json`
- `specforge adapt <intent-ir> --target fsm --dry-run` prints computed adapter JSON without writing artifacts
- `specforge adapt <intent-ir> --target fsm` materializes `generated/adapters/fsm/<document_key>/adapter.json` and writes an emitted `.fsm` file when the canonical interface/control/system/init/state surface or explicit module/top composition surface is explicit enough for honest standalone DT, structured FSM, or first-slice `?top:name` lowering
- `specforge converge <source> --target fsm` materializes the loop-backed pipeline entrypoint, defaults to full Ollama VLM + NLP Level 3 enrichment, and stops when `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR`/adapter facts stop changing
- `specforge project-validation <artifact>...` validates the passed artifacts, persists their latest reports, refreshes `VALIDATION_SNAPSHOT.md`, and updates the managed validation projection block in `LIVE_ACHIEVEMENT_STATUS.md`
- PDF execute-mode ingest expects `docling` to be importable from `python3` or `python`; when it lives elsewhere, set `SPECFORGE_DOCLING_PYTHON=/path/to/python`

## Planned product shape
- stage 0: build `SourceIR` from raw sources, normalized text views, structured page artifacts, and extracted visual assets
- stage 1: build `EvidenceIR` from normalized text, section anchors, evidence spans, figure/caption links, visual evidence, and statement extraction
- stage 2: build `SemanticIR` from actors, interfaces, backend-neutral system/init records, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- stage 3: build `IntentIR` as the canonical backend-independent intent model, including explicit interface inventory, backend-neutral guarded/action fragments, and backend-neutral system/init records when supported by the evidence
- stage 4: lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- stage 5: validate adapters and back-annotate findings into the IR/documentation surface

## Key operating principles
- `IntentIR` is the canonical endpoint
- `.fsm` is an adapter target, not the core endpoint
- the pipeline is explicit: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- the staged pipeline can now be driven through a fixed-point entrypoint that re-runs downstream stages until the persisted knowledge snapshot stabilizes
- automation-first, manual-last
- actor-first extraction
- typed IR over string-based generation
- deterministic steps where possible
- structured document parsing first, selective multimodal enrichment second
- LLM assistance where interpretation is required
- markdown is a lossy convenience view for PDFs, not the only normalized representation
- residual decision packets for irreducible ambiguity
- live documentation as critical continuity infrastructure

Read SESSION_BOOTSTRAP.md and start from there.
