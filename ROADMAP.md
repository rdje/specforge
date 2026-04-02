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
- status: Done
- goals:
  - lift `EvidenceIR` into actors, interfaces, typed signal records, backend-neutral control fragments, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
  - keep the representation backend-neutral
- completion criteria:
  - the tool can build a real `SemanticIR`
  - `specforge semantic` previews and materializes `SemanticIR` at `generated/semantic_ir/<document_key>/semantic_ir.json`
  - explicit signal declarations and guarded/action block fragments are preserved as typed semantic records when the evidence is explicit enough
  - explicit clock/reset/init statements are preserved as typed backend-neutral system/init records when the evidence is explicit enough
  - semantic residual decisions are explicit
  - actor-first extraction is visible in the typed model

### R5 IntentIR
- status: Done
- goals:
  - canonicalize the semantic model into backend-independent `IntentIR`
  - make `IntentIR` precise enough that adapters are lowering passes rather than semantic invention
- completion criteria:
  - a real `IntentIR` artifact can be emitted
  - `specforge intent` previews and materializes `IntentIR` at `generated/intent_ir/<document_key>/intent_ir.json`
  - `IntentIR` is versioned and serializable
  - canonical interface inventory and backend-neutral guarded/action fragments are carried forward when the semantic model makes them explicit
  - canonical backend-neutral system contract and init assignments are carried forward when the semantic model makes them explicit
  - assumptions, abstractions, and residual decisions remain explicit

### R6 Adapter layer
- status: In Progress
- goals:
  - define target-specific lowering boundaries for:
    - `.fsm`
    - SystemVerilog
    - Verilog
    - VHDL
  - land the first honest `.fsm` adapter slices for standalone DT, explicit FSM-root, and explicit top-root composition cases without leaking target assumptions backward into `IntentIR`
  - keep adapter concerns from leaking backward into `IntentIR`
- completion criteria:
  - adapter planning is typed
  - at least one real adapter artifact exists after `IntentIR` is stable
  - non-renderable adapter cases stop with explicit residual decisions instead of fabricated target text
  - real standalone `?dt:name` target text is emitted only when the canonical signal, control, system-contract, and init structure is renderable without semantic invention
  - standalone DT lowering can also carry canonical symbol-definition sections, reset-role blocks, selector/test-node branches, and compound-update shorthand when the widened canonical control/value surface maps directly to explicit `.fsm` syntax
  - explicit standalone sequential DT cases can lower with `(+system ...)` and `(:= ...)` without promoting a true FSM root
  - real structured `?fsm:name` target text is emitted only when the canonical state graph, state-body control, transition targets, and system/init surface are explicit enough to avoid semantic invention
  - real explicit `?top:name` target text is emitted only when the canonical model carries explicit top ports, child modules, renderable child roots, and width-compatible links
  - the canonical system contract preserves reset kind, polarity, assertion/release timing, and reset-target semantics explicitly enough that adapter lowering does not have to infer hardware reset behavior ad hoc
  - compatibility-level `?mod:name` and `?module:name` spellings stay outside the adapter root-kind model until the canonical layer carries an honest direct-module distinction

### R7 Validation and back-annotation
- status: In Progress
- goals:
  - validate stage outputs and adapter outputs
  - collect diagnostics and back-annotate findings into IR artifacts and live docs
- completion criteria:
  - validation reports are reproducible and tied to IR/artifact versions
  - adapter validation does not replace semantic validation
- done:
  - `specforge validate <artifact>` command: auto-detects IR stage, reports signal coverage %, NLP coverage, VLM readiness, structured extraction counts, quality score 0–100 with grade
  - validate tests for all four IR stages
- remaining:
  - back-annotation of findings into IR artifacts and live docs
  - adapter validation (SystemVerilog/Verilog/VHDL targets)

### R8 SourceIR SOTA capture (Tier 1 of EXTRACTION_ARCHITECTURE.md)
- status: In Progress
- reference: `EXTRACTION_ARCHITECTURE.md` §Tier 1
- goals:
  - extract structured table cell grids from Docling (not just image + caption)
  - type every text element (section_header, body_text, list_item, code, caption, footnote, formula)
  - build section hierarchy with semantic classification (SignalDescription, Boilerplate, Normative, Timing, RegisterDescription, etc.)
  - capture document profile (title, version, page/table/figure counts)
  - classify each table type (signal_description, encoding, register_map, timing_parameter, feature_matrix)
- completion criteria:
  - `SourceIr.structured_tables` carries cell grids for all PDF tables
  - `SourceIr.content_elements` carries all typed text elements in reading order
  - `SourceIr.document_sections` carries section hierarchy with `SectionKind`
  - `SourceIr.document_profile` carries title and counts
  - `StructuredTableRecord.table_kind` classifies every table
  - `SourceIR` drops no structured information that Docling provides

### R9 EvidenceIR SOTA typed evidence (Tier 2 of EXTRACTION_ARCHITECTURE.md)
- status: In Progress
- reference: `EXTRACTION_ARCHITECTURE.md` §Tier 2
- goals:
  - synthesize formal signal declarations from signal description tables (no band-aid in SemanticIR)
  - synthesize enum definitions from encoding tables
  - synthesize register records from register map tables
  - synthesize timing constraint records from timing parameter tables
  - add `NormativeStatement` class to statement classification
- completion criteria:
  - signal tables produce `Signal X is output width N.` statements in EvidenceIR
  - encoding tables produce `Enum <name> <member> = <value>.` statements in EvidenceIR
  - `NormativeStatement` class used for shall/must sentences in normative sections
  - SemanticIR `parse_signal_table_row` band-aid removed
  - `IntentIR` for AMBA AHB carries all 32+ signals with direction and width, all encoding enums

### R10 EvidenceIR visual content (Tier 3 of EXTRACTION_ARCHITECTURE.md)
- status: Done
- reference: `EXTRACTION_ARCHITECTURE.md` §Tier 3
- goals:
  - classify visual assets beyond caption heuristics (timing_diagram, state_machine, block_diagram, etc.)
  - VLM extraction of timing diagram content → typed timing observations
  - VLM extraction of state machine content → typed state/transition observations
- completion criteria:
  - `VisualObservation` types `Description`, `ChartExtraction` populated for classified diagrams
  - timing diagrams produce `TimingConstraintRecord` in `SemanticIR`/`IntentIR`
  - state machine diagrams produce `RegularStateRecord`/`StateTransitionRecord` in `SemanticIR`/`IntentIR`
- done:
  - `DiagramKind` classification from caption text (Steps 3.1): TimingDiagram, StateMachineDiagram, BlockDiagram
  - `specforge enrich` with Ollama/OpenAI/LM Studio providers (Steps 3.2/3.3)
  - VLM JSON stored in `VisualAsset.note`; EvidenceIR injects `TimingDiagramExtraction`/`StateMachineExtraction` observations
  - SemanticIR merges VLM-sourced timing constraints and state/transition records
  - Full test coverage for the VLM wiring chain

### R11 NLP Level 3 enrichment
- status: Not Started
- reference: `EXTRACTION_ARCHITECTURE.md` §Step 3.4
- goals:
  - reclassify ambiguous `NormativeStatement` sentences that Level 2 pattern-matching cannot handle
  - use same VLM provider infrastructure already in place from R10
- completion criteria:
  - `specforge nlp-enrich <evidence-ir> --vlm-provider <provider>` command implemented
  - `NormativeStatement` residual count reduced by ≥50% for AMBA AHB spec
  - upgraded sentences produce `SignalConstraintRecord` entries with `confidence: Medium`
  - full test coverage for the NLP Level 3 pipeline

## Recommended implementation order
1. keep the `IntentIR` product boundary explicit in all docs and code
2. build adapters after `IntentIR` is stable
3. push data richness into `SourceIR` and `EvidenceIR` (R8, R9) before patching downstream stages
4. integrate VLM visual understanding (R10) after structured extraction is solid
5. integrate validation and back-annotation (R7)

## Immediate next milestone
- complete R8/R9 Tier 1 and Tier 2 so `SourceIR` captures all structured table data and `EvidenceIR` produces typed signal/encoding declarations from tables
