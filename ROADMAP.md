# ROADMAP
## Objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from specifications into canonical `IntentIR`
- keep `.fsm`, SystemVerilog, Verilog, and VHDL as adapter targets downstream of `IntentIR`
- preserve deterministic provenance, typed intermediate data, and explicit residual decisions across all stages
- treat text, layout, figures, captions, tables, and charts as first-class evidence rather than markdown decoration
- prioritize semantic truthfulness and KG quality ahead of adapter breadth until the canonical four-layer pipeline is top-notch
- make the workflow resumable and understandable through live project documentation

## Canonical pipeline
- `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

## Cross-cutting implementation doctrine
- do not try to program a general reader of English; program a compiler for protocol meaning
- define the protocol-world model first:
  - actors
  - signals
  - roles
  - timing predicates
  - state transitions
  - dependencies
  - handshake events
- prefer deterministic extraction when the source surface is crisp:
  - signal tables
  - widths
  - enum rows
  - clock/reset declarations
  - polarity cues
  - section/figure/table kinds
- treat prose, tables, figures, captions, and layout as evidence for typed domain facts, not as final outputs themselves
- use convergence, backannotation, and evidence aggregation to decide what survives into canonical IR
- use AI/VLM/NLP as bounded local hypothesis generators for hard spans or images, not as an unstructured end-to-end PDF-to-intent black box
- every AI-derived hypothesis must be forced back through:
  - schema checks
  - grounding checks
  - conflict/arbitration checks
  - convergence checks
  - validation
- preserve ambiguity explicitly:
  - keep alternatives when the evidence is not yet decisive
  - surface contradictions and residual decisions
  - do not fabricate semantic certainty
- keep canonical document truth local and provenance-pure:
  - each `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` pipeline run should remain grounded only in the current document
  - any future cross-document learning layer must learn reusable extraction priors rather than smuggling facts from earlier PDFs into later canonical artifacts
- roadmap progress should favor meaning-based role inference and protocol semantics over literal spelling heuristics whenever the evidence can support that shift

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
- status: Horizon (minimal `.fsm` slice landed; further adapter expansion is intentionally deferred)
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
  - `specforge converge <source> --target <adapter>` now materializes a whole-pipeline fixed-point loop: ingest once, reuse persisted `SourceIR`, rebuild `EvidenceIR` / `SemanticIR` / `IntentIR` / adapters, re-run Ollama VLM + NLP enrichment by default, and stop when the persisted knowledge snapshot is stable
  - `specforge validate <artifact>` now backannotates a deterministic `validation_report.json` sidecar next to the validated IR artifact and writes the latest report back into the artifact's `validation_reports` field
  - validation findings are now graph-aware for the four IR stages, including producer/consumer gaps and compatibility-surface lag on the actor-relative KG
  - `specforge project-validation <artifact>...` now validates the passed artifacts and projects their persisted reports into tracked live docs via `VALIDATION_SNAPSHOT.md` plus a managed validation block in `LIVE_ACHIEVEMENT_STATUS.md`
- remaining:
  - extend validation into the upcoming semantic-truthfulness surfaces (temporal rules, arbitration/conflict records, and KG-quality benchmarks)
  - keep adapter validation (SystemVerilog/Verilog/VHDL targets) as horizon work until the semantic pipeline is materially harder to fool

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
- follow-up guidance:
  - treat `SourceIR` and `specforge ingest` as architecturally strong but not universally solved
  - do not prioritize broad new Tier 1 concept expansion ahead of semantic truthfulness unless a real PDF exposes a capture bottleneck
  - the remaining Tier 1 work should be robustness-oriented:
    - benchmark against varied real chip-spec PDFs
    - detect ugly-layout / OCR / table-split failure modes honestly
    - improve fallback behavior and source-level validation
    - preserve residual decisions when structured capture is not trustworthy

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
  - representative local APB/AHB/AXI validation snapshots were refreshed from the current extraction stack:
    - APB 95/100 EXCELLENT
    - AHB 95/100 EXCELLENT
    - AXI 94/100 EXCELLENT

### R13 Actor-signal relation extraction: Tier 2 prose patterns
- status: Done
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
  - downstream `SemanticIR` / `IntentIR` now carry the extracted graph directly via `actor_signal_relations`, `actor_ports`, and `signal_connectivity` instead of forcing later stages to rediscover relation evidence from prose
  - refreshed baselines now show stable honest coverage improvements: APB 95/100, AHB 95/100, AXI 94/100
- completion criteria:
  - APB direction coverage reaches a stable honest baseline from relation extraction
  - AXI large-signal coverage improves without relying on misleading single-signal metrics
  - AHB cross-validation between table-derived and prose-derived facts is inspectable
  - downstream stages preserve enough relation information that actor-relative modeling does not need to rediscover the graph from raw prose
### R14 Actor-signal relation extraction: Tier 3 LLM
- status: Not Started
- reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md`
- prerequisites:
  - graph-first downstream semantics from `R15`
  - temporal/benchmark hardening from `R15b`–`R15e`
- goals:
  - add `signal_relation` extraction type to the NLP prompt surface
  - implement `specforge signal-resolve` (or equivalent integrated relation-resolution flow)
  - handle complex sentences where Tier 2 verb patterns do not match cleanly
- completion criteria:
  - Tier 3 relation extraction improves actor-signal gold-fixture recall on hard prose without materially reducing precision
  - extracted relation records participate cleanly in the convergent backannotation loop without duplicate inflation or graph drift
  - the relation-resolution workflow is documented in `USER_GUIDE.md`

### R15 Actor-relative direction model in SemanticIR
- status: In Progress
- reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md` §Phase 4
- goals:
  - replace `direction_hint: Option<InterfaceSignalDirection>` with an actor-relative model
  - make `InterfaceSignalRecord` carry actor-relative drive/read information instead of a single flattened perspective
  - compute adapter-facing port directions relative to the target actor at adapter time
- done:
  - `SemanticIR` now carries `actor_signal_relations`, `actor_ports`, and `signal_connectivity`
  - `IntentIR` now preserves the same actor-relative KG surface as canonical output
  - `ActorRecord` / `IntentActor` now preserve the surfaced actor name when it is grounded by relation evidence
  - `specforge validate` now reports actor-signal relation, actor-port, and connectivity counts for `SemanticIR` / `IntentIR`
  - `specforge validate` now scores semantic and intent signal-direction coverage from the actor-relative graph first, with flat `direction_hint` lag exposed as compatibility diagnostics rather than truth-model loss
- remaining:
  - make the actor-relative graph, not flat `direction_hint`, the primary downstream signal-direction surface
  - compute target-actor-relative port directions from the actor-relative graph whenever a downstream consumer needs them
- completion criteria:
  - downstream consumers can compute correct actor-relative port directions without depending on flat compatibility `direction_hint`
  - `IntentIR` carries a proper directed graph, not a flat list with implicit actor context

### R15b Explicit clock-tick temporal model in SemanticIR / IntentIR
- status: In Progress
- reference: `KNOWLEDGE_GRAPH_ARCHITECTURE.md`
- goals:
  - make the clock-tick mental model explicit in the typed IR, not only implicit in prose-derived timing strings
  - represent synchronous behavior in terms of pre/post tick phases, clock edges, and cycle windows
  - unify prose timing, table timing, and VLM timing observations under one canonical temporal-rule surface
- done:
  - `SemanticIR` now carries `temporal_rules: Vec<TemporalRuleRecord>`
  - `IntentIR` now carries the same `temporal_rules` forward as canonical output
  - the first temporal predicate set now represents:
    - signal value-at-phase facts
    - signal stability across `pre_tick -> post_tick`
    - signal sampling on clock edges, with optional actor grounding
    - ready/valid handshake completion when grounded `VALID` and `READY` assertions co-occur in the same temporal context
    - meaning-grounded handshake completion when signal-description text, prose descriptions, or alias-grounded prose descriptions already establish valid-like and ready-like roles even if the spellings are not literally `VALID` / `READY`
    - actor-relative drive predicates when the structural KG resolves a unique signal producer
    - actor-relative stability predicates when the structural KG resolves a unique producer for a stable/hold obligation
    - compound conjunctive guards as multiple antecedent predicates when each clause is grounded to a known signal
  - temporal rules now recover bounded `cycle_window` latency from phrases like `within 2 cycles`, `next cycle`, `next tick`, `next rising edge`, and from timing rows whose unit is already `cycles`
  - `SemanticIR` / `IntentIR` now also carry `temporal_conflicts: Vec<TemporalConflictRecord>` for contradictory value obligations that target the same signal/phase under the same grounded context
  - temporal derivation now uses an explicit clock declaration even when a full reset-bearing `SystemContractRecord` is not yet available
  - `specforge validate` now reports temporal-rule counts plus missing clock/edge grounding, actor-grounding diagnostics, handshake-predicate coverage, multi-predicate antecedent coverage, and typed temporal conflict counts
- completion criteria:
  - a typed temporal-rule representation exists in `SemanticIR` and carries forward into `IntentIR`
  - APB/AHB/AXI timing behavior can be represented in actor-relative, tick-relative form rather than only as free-form timing text
  - validation can flag unresolved or contradictory temporal grounding explicitly

### R15c KG-guided multimodal rescans
- status: In Progress
- reference: `DEVELOPMENT_NOTES.md`
- goals:
  - use known signals, actors, enum members, states, and value atoms as anchors for repeated rescans over tables, prose, and figures
  - make the KG a search index for the next pass instead of treating each modality as a one-shot extraction source
  - stop iterating only when backannotated knowledge stabilizes
  - evolve semantic role detection from literal spellings toward meaning-based inference using aliases, table descriptions, actor relations, and repeated multimodal grounding
- done:
  - `EvidenceIR` now persists `signal_semantic_hints: Vec<SignalSemanticHintRecord>` mined from `SignalDescription` table descriptions when the text establishes semantic roles such as request-valid or accept-ready meaning
  - `SemanticIR` / `IntentIR` now carry `semantic_tags` on interface signals so those meaning-grounded roles survive downstream instead of being trapped inside the table-extraction stage
  - handshake completion derivation now consults those semantic tags before falling back to literal signal-name heuristics, and contested semantic arbitration now blocks that fallback, so protocol meaning outranks spelling when the preserved evidence is explicit enough to disagree
  - `EvidenceIR` now also refreshes those semantic hints from `SourceFact` prose descriptions and alias-grounded prose descriptions, so Form 2 alias learning can feed semantic role inference instead of stopping at constraint reclassification
  - `specforge nlp-enrich` now refreshes `signal_semantic_hints` before persistence whenever alias learning or backannotation changes the evidence state
  - `EvidenceIR` now also refreshes `signal_semantic_hints` from grounded visual captions and VLM timing-diagram annotations when they explicitly name a single known signal and establish a role meaning
  - prose/visual semantic-hint synthesis now strips explicit signal identifiers before role-tag inference, so the surrounding descriptive language must establish valid-like or ready-like meaning instead of letting names like `AWVALID` / `AWREADY` self-justify consensus
  - prose and visual semantic-hint synthesis now also decomposes multi-signal regions into clause-local per-signal context windows, so one sentence/caption can ground different role meanings for different signals without forcing whole-text single-target resolution
  - explicit signal mentions now outrank alias-grounding for the same signal inside one prose/caption region, so aliases only contribute when they were actually needed to anchor the meaning
  - `SemanticIR` / `IntentIR` now also mark semantic candidates and consensus summaries as `alias_dependent` when the role meaning still depends only on alias-grounded evidence, and validation now reports that state explicitly
  - validation now also reports when typed `HandshakeComplete` predicates depend on alias-dependent semantic consensus, so the temporal layer does not hide weaker alias-grounded role meaning behind ordinary handshake coverage counts
  - alias-dependent handshake-completion semantics now also surface as a canonical `semantic_alias_dependent_handshake_completion` residual in `SemanticIR`, and `IntentIR` mirrors that state as an explicit assumption so weaker temporal grounding remains inspectable even before validation runs
  - caption/VLM-grounded semantic hints now carry explicit visual-evidence provenance instead of degrading to anonymous text-only hints
  - `SemanticIR` / `IntentIR` now carry per-signal `semantic_observations`, preserving role provenance and source kind in the canonical layers instead of only keeping merged `semantic_tags`
  - `SemanticIR` / `IntentIR` now also carry explicit per-signal `semantic_candidates`, so competing role hypotheses remain inspectable in the canonical layers even when no role is safely resolved
  - `SemanticIR` / `IntentIR` now also carry explicit per-signal `semantic_arbitration`, preserving the current lead role, runner-up, evidence margin, and decisive-vs-contested status so arbitration stays inspectable without forcing unsafe winner selection
  - blocked handshake-name fallback now also surfaces as an explicit semantic residual decision and validation metric/finding, so withheld heuristic promotion is visible to users rather than only enforced internally
  - `SemanticIR` / `IntentIR` now also resolve per-signal `resolved_semantic_role` plus modality-aware `semantic_grounding_strength` from those canonical observations, so downstream consumers can prefer provenance-backed role consensus and validation can separate single-source grounding, same-modality repetition, and true cross-modality reinforcement
  - `SemanticIR` / `IntentIR` now also carry an explicit per-signal `semantic_consensus` summary for observation-backed role meanings, including supporting source kinds, observation count, and strongest supporting automation confidence, and validation now flags any resolved role that still lacks that richer consensus profile
  - `specforge validate` now also reports decisive vs non-decisive semantic arbitration explicitly, so unresolved role competition is visible to users as a first-class canonical state rather than hidden behind absent resolved roles
  - fallback-only resolved semantic roles now also surface as an explicit `semantic_resolved_role_without_consensus` residual decision in `SemanticIR`, and `IntentIR` adds a matching assumption so provisional meaning is visible even before a validator runs
  - typed handshake-role recovery now only trusts observation-backed semantic consensus; fallback-only resolved roles no longer drive canonical handshake-role classification, and handshake-shaped signals with provisional role state now block raw name fallback too
- completion criteria:
  - anchored rescans over tables, prose, and figures are first-class parts of the convergent loop
  - weakly labeled signal-detail tables and additional polarity/timing/value facts can be recovered from known anchors
  - convergence reporting counts genuinely new persisted facts instead of duplicate vector growth
  - handshake, role, and timing semantics no longer depend only on literal signal naming when the document provides enough grounded evidence to infer the same meaning
  - literal handshake-name heuristics do not override preserved contested semantic evidence
  - provisional fallback-only semantic roles do not hide behind a silent canonical winner; they remain explicit residual/assumption state until stronger grounding arrives
  - provisional fallback-only semantic roles also do not silently drive typed handshake semantics until observation-backed consensus exists
  - signal identifiers alone do not create semantic-role consensus in prose/visual hint synthesis without descriptive language
  - multi-signal prose/caption regions can contribute different role hints to different signals when the local clause language is explicit enough to separate them safely
  - alias evidence does not inflate semantic-role support when an explicit signal mention already anchors the same local text region
  - alias-dependent canonical role meaning remains inspectable as a first-class property instead of being hidden inside raw source-kind vectors
  - alias-dependent temporal handshake semantics remain inspectable as weaker temporal grounding rather than blending invisibly into generic handshake-completion coverage
  - weaker alias-grounded temporal handshake semantics also remain explicit residual/assumption state in the canonical artifacts, not only validator output

### R15d Evidence arbitration and cross-modality conflict resolution
- status: In Progress
- goals:
  - define how table, prose, and figure evidence reinforce or conflict
  - preserve contradictory evidence explicitly instead of flattening it away
  - rank evidence by provenance strength and automation confidence without hiding disagreement
  - treat AI/VLM/NLP outputs as candidate evidence that must earn promotion into canonical facts through typed grounding and arbitration, not as self-justifying truth
- done:
  - `EvidenceIR` now persists `signal_polarity_conflicts: Vec<SignalPolarityConflictRecord>` when prose and signal-description tables disagree on active-high/active-low semantics
  - `specforge validate` now reports and flags those polarity conflicts explicitly, so contradictory polarity stays inspectable instead of only affecting the derived constraint kind
  - `SemanticIR` / `IntentIR` now carry those `signal_polarity_conflicts` forward too, and `specforge validate` now reports them at both canonical stages so contradictory active-level evidence no longer disappears after `EvidenceIR`
  - `EvidenceIR` now persists `signal_semantic_conflicts: Vec<SignalSemanticConflictRecord>` when meaning-based role evidence assigns incompatible roles to the same signal
  - `specforge validate` now reports and flags those semantic-role conflicts explicitly instead of leaving incompatible role evidence hidden inside a dual-tag ambiguity
  - `SemanticIR` / `IntentIR` now carry `signal_semantic_conflicts` forward, so unresolved role disagreement remains visible in the canonical layers instead of disappearing after `EvidenceIR`
  - `specforge validate` now reports and flags those carried semantic-role conflicts for both `SemanticIR` and `IntentIR`
  - `SemanticIR` / `IntentIR` now persist `interface_signal_conflicts: Vec<InterfaceSignalConflictRecord>` when conflicting declarations disagree on signal direction or width
  - `specforge validate` now reports and flags those interface-signal conflicts explicitly, so incompatible shape evidence stays inspectable instead of silently nulling the canonical hint
  - `SemanticIR` / `IntentIR` now persist `signal_connectivity_conflicts: Vec<SignalConnectivityConflictRecord>` when the structural KG resolves multiple producers for the same signal
  - `specforge validate` now reports and flags those structural connectivity conflicts explicitly, so producer ambiguity stays inspectable instead of remaining hidden inside connectivity vectors
- completion criteria:
  - a typed arbitration/conflict surface exists for unresolved multimodal disagreements
  - validation can flag contradictory direction, timing, and value facts
  - representative APB/AHB/AXI disagreements are inspectable rather than silently overwritten
  - AI-derived hypotheses that cannot be grounded or arbitrated remain explicit residuals/conflicts instead of silently entering canonical IR

### R15e KG-quality evaluation and benchmark hardening
- status: In Progress
- goals:
  - move quality assessment beyond aggregate score
  - add curated gold fixtures, negative fixtures, and precision/recall-style checks for the most important KG surfaces
  - measure false positives for relations, enums, widths, timing, and structured constraints
  - measure uncertainty quality too:
    - whether unresolved ambiguity is preserved honestly
    - whether contradiction surfacing fires when it should
    - whether weak AI hypotheses are rejected when grounding is insufficient
- completion criteria:
  - curated APB/AHB/AXI gold fixtures exist for actor relations, signal inventory, timing, and structured constraints
  - negative fixtures exist for alias noise, bogus actor attribution, spurious timing extraction, and table misclassification
  - roadmap progress is driven by KG accuracy and false-positive control, not only one scalar score
  - benchmark results include explicit checks for residual quality, conflict surfacing, and bounded-hypothesis rejection behavior
- done:
  - `specforge kg-bench` command now runs tracked KG-quality fixtures through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR`
  - tracked fixture support now lives under `crates/specforge/test_data/kg_quality/`
  - the initial fixture pack includes:
    - a gold fixture for actor-relative port recovery
    - a negative fixture for rejecting name-only semantic role noise
    - a negative fixture for surfacing multi-producer structural conflicts through validation
    - a residual-quality fixture for actor-boundary ambiguity
    - a stage-patched negative fixture proving contested handshake-shaped signal names do not leak into typed `HandshakeComplete` predicates
    - a stage-patched caveat fixture proving alias-dependent handshake completion stays usable while preserving its canonical residual/assumption trail
    - a stage-patched cross-modality gold fixture proving a semantic role can be grounded jointly by table and visual evidence while validation reports that stronger grounding explicitly
  - fixture expectations can now assert canonical semantic candidates plus decisive vs non-decisive semantic arbitration directly, so the benchmark harness checks the truth-model state itself rather than only downstream residual/finding side effects
  - fixture expectations can now also assert persisted validation metric values directly, and `SourceIR` fixture patches can now inject visual assets as well as tables, so the harness can lock cross-modality grounding behavior with tracked staged fixtures
- remaining:
  - expand from seed fixtures to representative APB/AHB/AXI gold fixtures
  - add broader negative fixtures for bogus actor attribution, spurious timing extraction, table misclassification, and multimodal arbitration drift
  - add broader metric-oriented expectation surfaces once the current canonical+metric fixture layer stabilizes

### R15f Cross-document extraction learning plane
- status: Not Started
- goals:
  - let the extraction system become stronger on PDF `N+1` because it has learned reusable analysis priors from PDFs `1..N`
  - keep the per-document four-layer IR pipeline provenance-pure while adding a separate global typed learning layer
  - teach the system how chip specifications tend to express meaning, not undocumented document facts
- design constraints:
  - the document plane stays local:
    - `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR` for one document contains only facts justified by that document
  - the learning plane stays separate:
    - store typed reusable extraction knowledge such as `CorpusMemory`, `PriorGraph`, or `ExperienceIR`
    - never let prior memory directly author canonical document facts without fresh local grounding
  - the system should learn priors, not smuggle facts:
    - good prior: phrases like `can accept the transfer` are often strong ready-like evidence
    - bad prior: APB used signal `PREADY`, so a new document must mean the same thing without local evidence
- target learned priors:
  - recurring semantic-role language
  - recurring alias language
  - recurring table shapes and table-kind cues
  - recurring visual motifs and caption cues
  - actor taxonomies and protocol-family vocabulary
  - temporal-language priors
  - modality reliability priors
  - false-positive patterns and known-dangerous heuristics
  - protocol-family scoped extraction patterns
- runtime shape:
  - analyze the new PDF through the normal staged IR pipeline
  - retrieve relevant priors from the cross-document memory
  - use those priors only to propose bounded hypotheses or prioritize rescans
  - require local grounding in the current PDF before promotion into canonical IR
  - let validation and arbitration decide what survives
  - feed only high-confidence, well-grounded, validated outcomes back into the learning plane
- completion criteria:
  - a typed cross-document prior store exists and is versioned separately from per-document IR artifacts
  - priors can be queried by modality, protocol family, and extractor task
  - `EvidenceIR` / `SemanticIR` builders can consume relevant priors as bounded suggestions without bypassing local grounding
  - only validated/promoted outcomes are allowed to update the learning plane
  - regression/benchmark coverage proves that prior memory improves extraction efficiency or recall without increasing fact leakage across documents
- remaining:
  - design the schema for typed prior memory and prior provenance
  - decide how protocol-family scoping works without hardcoding brittle protocol logic
  - define the trust/update policy for feeding validated outcomes back into memory
  - add benchmarks that measure whether prior memory improves analysis of unseen PDFs honestly

### R16 SystemVerilog adapter (Horizon)
- status: Horizon
- prerequisites:
  - `R15`, `R15b`, `R15c`, `R15d`, `R15e`, `R15f`, and `R14` are materially complete
- goals:
  - generate a correct SystemVerilog interface from `IntentIR`
  - generate a correct SystemVerilog module template for each actor
  - compute port directions from actor-relative signal relations

## Recommended implementation order
1. Keep `IntentIR` as the canonical product boundary in all code and docs
2. Finish the graph-first downstream signal model so `direction_hint` is no longer the primary semantic surface (`R15`)
3. Land the explicit clock-tick temporal model (`R15b`)
4. Make KG-guided multimodal rescans a first-class convergent workstream (`R15c`)
5. Add typed evidence arbitration and conflict resolution across modalities (`R15d`)
6. Harden evaluation with gold fixtures, negative fixtures, and false-positive control (`R15e`)
7. Add a separate cross-document learning plane for typed extraction priors while keeping canonical document truth local (`R15f`)
8. Extend relation extraction for harder prose with Tier 3 support only after the graph/temporal/eval surfaces are ready (`R14`)
9. Treat new adapter families and adapter validation as horizon work until the semantic pipeline is materially harder to fool (`R16`)

## Immediate next milestone
- `R15`: finish the transition from compatibility `direction_hint` fields to actor-relative graph-first downstream semantics
- `R15b`: introduce the explicit clock-tick temporal model so behavioral truth is first-class in `SemanticIR` / `IntentIR`
- `R15c`: make KG-guided multimodal rescans a named workstream in the convergent pipeline
- `R15f`: design the cross-document learning plane so the extractor can accumulate reusable priors without contaminating per-document canonical truth
