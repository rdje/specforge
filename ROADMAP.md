# ROADMAP
## Objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from specifications into canonical `IntentIR`
- keep `.fsm`, SystemVerilog, Verilog, and VHDL as adapter targets downstream of `IntentIR`
- preserve deterministic provenance, typed intermediate data, and explicit residual decisions across all stages
- treat text, layout, figures, captions, tables, and charts as first-class evidence rather than markdown decoration
- make the workflow resumable and understandable through live project documentation

## Canonical pipeline
- `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## Major workstreams
### R0 Repository, workflow, and continuity bootstrap
- status: Done
- goals:
  - establish the live documentation surface
  - define the project objective and staged direction
  - define the commit workflow and continuity expectations
  - create a usable session bootstrap path for future AI/LLM sessions
- completion criteria:
  - core live documents exist
  - README is the single entry point
  - SESSION_BOOTSTRAP is in place
  - roadmap and live-status files are established

### R1 IntentIR pivot and CLI identity
- status: Done
- goals:
  - make `IntentIR` the canonical endpoint
  - rename the CLI/crate direction to `specforge`
  - remove `.fsm` as the apparent primary product boundary
- completion criteria:
  - docs describe `IntentIR` as the canonical output
  - active crate/binary name is `specforge`
  - adapter targets are described as downstream of `IntentIR`

### R2 SourceIR
- status: Done
- goals:
  - detect source kinds
  - record source identity and canonical paths
  - plan normalization into promoted artifacts
  - record parser backend identity for structured document conversion
  - reserve page-artifact and visual-asset manifests
  - emit a typed `SourceIR` JSON artifact
  - emit source-side residual decisions when automation is not yet safe
- completion criteria:
  - `specforge ingest` materializes `SourceIR`
  - markdown inputs are represented cleanly
  - PDF inputs materialize promoted markdown, page-artifact manifests, page metadata sidecars, metadata JSON, backend raw JSON, and visual-asset manifests
  - directory and unknown inputs produce residual decisions instead of implicit failure

### R3 EvidenceIR
- status: Done
- goals:
  - extract section anchors and evidence spans from normalized sources
  - link text references to figures, captions, charts, and page crops
  - represent visual evidence as typed, provenance-carrying records
  - classify extracted statements into source facts, derived rules, local design decisions, and explicit abstractions
  - preserve precise provenance into a typed `EvidenceIR`
- completion criteria:
  - the tool can build a real `EvidenceIR` from normalized markdown plus structured page/visual artifacts
  - `specforge evidence` previews and materializes `EvidenceIR` at `generated/evidence_ir/<document_key>/evidence_ir.json`
  - evidence items retain provenance to source ranges
  - figure/caption linkage is explicit and inspectable
  - statement classification is explicit and inspectable

### R4 SemanticIR
- status: In Progress
- goals:
  - lift `EvidenceIR` into actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
  - keep the representation backend-neutral
- completion criteria:
  - the tool can build a real `SemanticIR`
  - semantic residual decisions are explicit
  - actor-first extraction is visible in the typed model

### R5 IntentIR
- status: In Progress
- goals:
  - canonicalize the semantic model into backend-independent `IntentIR`
  - make `IntentIR` precise enough that adapters are lowering passes rather than semantic invention
- completion criteria:
  - a real `IntentIR` artifact can be emitted
  - `IntentIR` is versioned and serializable
  - assumptions, abstractions, and residual decisions remain explicit

### R6 Adapter layer
- status: In Progress
- goals:
  - define target-specific lowering boundaries for:
    - `.fsm`
    - SystemVerilog
    - Verilog
    - VHDL
  - keep adapter concerns from leaking backward into `IntentIR`
- completion criteria:
  - adapter planning is typed
  - at least one real adapter exists after `IntentIR` is stable

### R7 Validation and back-annotation
- status: Not Started
- goals:
  - validate stage outputs and adapter outputs
  - collect diagnostics and back-annotate findings into IR artifacts and live docs
- completion criteria:
  - validation reports are reproducible and tied to IR/artifact versions
  - adapter validation does not replace semantic validation

## Recommended implementation order
1. keep the `IntentIR` product boundary explicit in all docs and code
2. lift `EvidenceIR` into `SemanticIR`
3. canonicalize into `IntentIR`
4. build adapters after `IntentIR` is stable
5. integrate validation and back-annotation

## Immediate next milestone
- build the first real `SemanticIR` lifter from `EvidenceIR` actors, interfaces, phases, invariants, and residual decisions
