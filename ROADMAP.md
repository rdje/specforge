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
  - `specforge converge <source> --target <adapter>` now materializes a whole-pipeline fixed-point loop: ingest once, reuse persisted `SourceIR`, rebuild `EvidenceIR` / `SemanticIR` / `IntentIR` / adapters, optionally re-run VLM + NLP enrichment, and stop when the persisted knowledge snapshot is stable
- remaining:
  - back-annotation of findings into IR artifacts and live docs
  - adapter validation (SystemVerilog/Verilog/VHDL targets)

### R8 SourceIR SOTA capture (Tier 1 of EXTRACTION_ARCHITECTURE.md)
- status: Done
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
- status: Mostly Done (typed evidence + convergent enrichment landed; remaining completeness work is incremental hardening)
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
  - later extraction passes can reuse newly synthesized enum facts and polarity facts without hardcoded protocol-specific value lists
  - `NormativeStatement` class used for shall/must sentences in normative sections
  - SemanticIR `parse_signal_table_row` band-aid removed
  - representative APB/AHB/AXI runs preserve the current honest 90/95/90 baseline from generated artifacts
- done:
  - signal, enum, register, and timing table synthesis landed in `EvidenceIR`
  - `EvidenceIr::build()` now uses a monotone convergence loop so newly synthesized enum facts can unlock later value-constraint extraction in the same build
  - weakly labeled encoding tables can now be recovered via signal anchors instead of requiring a hardcoded per-protocol value list
  - prose polarity extraction now refines asserted/deasserted constraints into polarity-aware low/high constraints when the spec says active-low or active-high
  - regression tests cover anchored encoding scanning and polarity refinement

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

### R11 NLP Level 3 enrichment + feedback loops
- status: Done (2026-04-03)
- reference: `EXTRACTION_ARCHITECTURE.md` §Step 3.4
- goals:
  - reclassify ambiguous `NormativeStatement` sentences that Level 2 pattern-matching cannot handle
  - reuse the same provider surface already used for visual enrichment
  - write learned results back into `EvidenceIR` instead of keeping them as one-off reports
- completion criteria:
  - `specforge nlp-enrich <evidence-ir> --vlm-provider <provider>` exists and is usable from the CLI
  - upgraded sentences produce medium-confidence structured records in `EvidenceIR`
  - successful Level 3 passes backannotate the originating statements and preserve learned alias state
  - the workspace test suite covers the Level 3 pipeline
- done:
  - `specforge nlp-enrich` command implemented with Ollama/OpenAI/LM Studio support, `--dry-run`, `--max-sentences`, and grounding-signal support
  - default `qwen2.5vl:7b` local-model path integrated for Ollama/LM Studio
  - Layer A/B/C/D/E controls landed: boilerplate suppression, prompt grounding, residual-stable convergence, declared-signal gating, and spec-type-aware scoring
  - Form 1 backannotation and Form 2 signal alias learning landed
  - initial AHB/APB/AXI validation runs established the post-NLP-L3 baseline that R12/R13 now refine

### R12 Multi-spec validation + quick fixes
- status: Done
- goals:
  - finish the remaining alias garbage filter in `extract_alias_phrase()`
  - keep AMBA-style signal-table direction parsing stable for `Source` / `Driver` / `Destination` columns and role names such as Requester, Completer, Manager, Subordinate, clock, and reset
  - keep a representative AHB/APB/AXI validation baseline recorded from the current KG-enabled + convergent extraction pipeline
- done:
  - direction/source/destination column handling widened for AMBA 5 terminology and infrastructure signals
  - width-only and parametric-width declarations landed so coverage reporting is more honest
  - `extract_alias_phrase()` now rejects alias subjects beginning with markdown/table markers `-`, `|`, or `#`
  - representative APB/AHB/AXI baselines were refreshed from the current extraction stack:
    - APB 90/100 EXCELLENT
    - AHB 95/100 EXCELLENT
    - AXI 90/100 GOOD

### R13 Actor-signal relation extraction: Tier 2 prose patterns
- status: Mostly Done (Tier 2 relations and convergent EvidenceIR reuse landed; downstream actor-relative carry-through is still deferred)
- reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md`
- goals:
  - represent actor-signal relations explicitly in the typed pipeline
  - extract `Drives` / `Reads` triples from prose verb patterns and signal-description tables
  - feed those relations into downstream direction synthesis without inventing semantics
- done:
  - `RelationKind` and `ActorSignalRelation` added to `source.rs`
  - `actor_signal_relations: Vec<ActorSignalRelation>` added to `EvidenceIr`
  - `extract_actor_signal_relations()` implemented in `evidence.rs` for active/passive drive/read patterns
  - signal-table `Source` / `Driver` column extraction implemented as a second Tier 2 relation source
  - KG-derived direction declarations synthesized back into `EvidenceIR` for downstream `SemanticIR` parsing
  - regression tests added for relation extraction and direction synthesis
  - relation-derived declarations now feed the convergent `EvidenceIR` loop, so discovered signal anchors can unlock additional encoding enums and value constraints
  - refreshed baselines now show stable honest coverage improvements: APB 90/100, AHB 95/100, AXI 90/100
- completion criteria:
  - APB direction coverage reaches a stable honest baseline from relation extraction
  - AXI large-signal coverage improves without relying on misleading single-signal metrics
  - AHB cross-validation between table-derived and prose-derived facts is inspectable
  - downstream stages preserve enough relation information that actor-relative modeling does not need to rediscover the graph from raw prose
- remaining:
  - carry actor-signal relations into the downstream signal model instead of flattening them to actor-agnostic direction hints before adapter time

### R14 Actor-signal relation extraction: Tier 3 LLM
- status: Not Started
- reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md`
- goals:
  - add `signal_relation` extraction type to the NLP prompt surface
  - implement `specforge signal-resolve` (or equivalent integrated relation-resolution flow)
  - handle complex sentences where Tier 2 verb patterns do not match cleanly
- completion criteria:
  - all three specs (AHB, APB, AXI) score ≥85/100 with honest signal counts
  - the relation-resolution workflow is documented in `USER_GUIDE.md`

### R15 Actor-relative direction model in SemanticIR
- status: Not Started
- reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md` §Phase 4
- goals:
  - replace `direction_hint: Option<InterfaceSignalDirection>` with an actor-relative model
  - make `InterfaceSignalRecord` carry actor-relative drive/read information instead of a single flattened perspective
  - compute adapter-facing port directions relative to the target actor at adapter time
- completion criteria:
  - SystemVerilog adapter can generate correct port directions for any actor
  - `IntentIR` carries a proper directed graph, not a flat list with implicit actor context

### R16 SystemVerilog adapter
- status: Not Started
- prerequisites: R15 (or an equivalent actor-relative direction surface)
- goals:
  - generate a correct SystemVerilog interface from `IntentIR`
  - generate a correct SystemVerilog module template for each actor
  - compute port directions from actor-relative signal relations

## Recommended implementation order
1. Keep `IntentIR` as the canonical product boundary in all code and docs
2. Complete validation/back-annotation on the current IR surface (R7)
3. Promote the downstream signal model from flat direction hints to actor-relative semantics (R15)
4. Extend relation extraction for harder prose with Tier 3 support (R14)
5. Build the SystemVerilog adapter on top of the actor-relative model (R16)

## Immediate next milestone
- R7 remaining slice: build validation/back-annotation and artifact-linked reporting on the current staged IR surface
- R15 remains the next larger architectural slice before downstream RTL adapters
