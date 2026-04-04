# DEVELOPMENT_NOTES
## Current project direction
- project name: `specforge`
- CLI/binary name: `specforge`
- implementation language: Rust
- canonical deliverable: `IntentIR`
- product shape: staged IR toolchain, not one-shot backend generation
- stage model: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

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

### Continuity as infrastructure
- live documentation is not optional process overhead
- `README.md`, `INTENTIR_SPEC.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `USER_GUIDE.md`, `DEVELOPMENT_NOTES.md`, `CHANGES.md`, and `MEMORY.md` are part of the engineering system
- they must be updated when work completes and at meaningful intermediate checkpoints during long-running tasks

## Current execution defaults and convergence accounting
- `specforge converge` is now the default full loop-backed pipeline entrypoint: it uses Ollama-backed VLM image enrichment and NLP Level 3 unless the caller explicitly opts out with `--vlm-provider skip` and/or `--nlp-provider skip`
- the converge command's `knowledge_fact_count` now tracks persisted IR knowledge rather than downstream adapter residual work, so fewer residual decisions on later passes do not falsely look like knowledge loss
- the current local AMBA validation baseline is APB 95/100, AHB 95/100, AXI 94/100 after full original-PDF converge runs with Ollama
- AXI `IHI0022_L` now converges cleanly in 2 outer passes; the old false failure was caused by counting decreasing adapter residual decisions against the monotone knowledge metric

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
- Form 2 alias learning in `specforge nlp-enrich` could still absorb markdown formatting noise when a normative sentence started with a bullet marker, table-cell marker, or heading marker before the real noun phrase.
- The concrete failure mode was learning aliases such as `- the address` instead of a real phrase such as `address bus`.

### Implementation shape
- `crates/specforge/src/commands/nlp_enrich.rs` now rejects alias subjects that begin with `-`, `|`, or `#` before article stripping and phrase normalization.
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
