# USER_GUIDE
## Purpose
- explain what `specforge` is intended to do from an end-user perspective
- document the expected staged workflow as the tool evolves toward canonical `IntentIR`

## What SpecForge is intended to become
- a staged Rust tool that turns protocol, component, system, and software-interface specifications into backend-independent `IntentIR`
- a system where backend formats are adapters, not endpoints
- an automation-first workflow that keeps ambiguity explicit through residual decision packets

## Current state
- the repository contains workflow and continuity documentation plus a runnable Rust CLI named `specforge`
- the current CLI already supports:
  - `specforge inspect <path>`
  - `specforge converge <source> --target fsm`
  - `specforge ingest <source> --dry-run`
  - `specforge ingest <source>`
  - `specforge evidence <source-ir> --dry-run`
  - `specforge evidence <source-ir>`
  - `specforge semantic <evidence-ir> --dry-run`
  - `specforge semantic <evidence-ir>`
  - `specforge intent <semantic-ir> --dry-run`
  - `specforge intent <semantic-ir>`
  - `specforge adapt <intent-ir> --target fsm --dry-run`
  - `specforge adapt <intent-ir> --target fsm`
  - `specforge enrich <source-ir>`
  - `specforge validate <artifact>`
  - `specforge kg-bench`
  - `specforge project-validation <artifact>...`
  - `specforge learn-priors <intent-ir>...`
  - `specforge nlp-enrich <evidence-ir>`
- the currently implemented executable IR stages are `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
- `SourceIR` now handles existing Markdown directly and performs Docling-backed structured PDF normalization for PDF inputs
- `EvidenceIR` now consumes ready `SourceIR` artifacts and extracts section anchors, evidence spans, visual evidence, figure/caption links, and heuristic statement classes
- `SemanticIR` now consumes ready `EvidenceIR` artifacts and lifts heuristic actors, interfaces, backend-neutral system/init records, first-class reset polarity/assertion/release/target semantics, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions; boilerplate sections (legal/admin) are filtered out before extraction, and signal-description tables in real chip specs are parsed for explicit direction and width
- `IntentIR` now consumes ready `SemanticIR` artifacts and canonicalizes actor responsibilities, interface/control/system/init surface, behaviors, constraints, assumptions, and residual decisions
- the first `.fsm` adapter slices now consume ready `IntentIR` artifacts and materialize typed adapter artifacts; explicit standalone combinational and sequential DT cases are renderable, explicit state-graph cases can now lower to structured `?fsm:name`, explicit module/top composition cases can now lower to `?top:name`, and direct-module alias roots remain deferred with explicit residual decisions

## Available commands today
### Inspect a path
```bash
cargo run -p specforge -- inspect README.md
```
- reports:
  - canonical path
  - path kind
  - detected source kind
  - extension
  - file size for regular files

### Preview a SourceIR artifact
```bash
cargo run -p specforge -- ingest README.md --dry-run
```
- prints computed `SourceIR` JSON without writing artifacts
- useful for checking:
  - source identity
  - normalization planning
  - downstream staged IR intent
  - residual decision packets for ambiguous sources
  - planned adapter targets

### Materialize a SourceIR artifact
```bash
cargo run -p specforge -- ingest README.md
```
- writes `generated/source_ir/<document_key>/source_ir.json`
- this is the first stage artifact emitted by the tool
- for Markdown inputs, the normalization plan points at the existing Markdown source
- for PDF inputs, execute mode now materializes:
  - promoted markdown
  - page images and page metadata sidecars
  - visual asset crops for pictures and tables
  - metadata JSON and backend raw JSON
  - `page_artifacts.json` and `visual_assets.json`
- PDF execute mode expects `docling` to be importable from `python3` or `python`; when needed, point `SPECFORGE_DOCLING_PYTHON` at the correct interpreter

### Preview an EvidenceIR artifact
```bash
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run
```
- prints computed `EvidenceIR` JSON without writing artifacts
- requires a `SourceIR` whose normalization status is `ready`
- useful for checking:
  - section anchors and line provenance
  - evidence spans and extracted statements
  - figure/caption linkage
  - visual evidence counts before materialization

### Materialize an EvidenceIR artifact
```bash
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json
```
- writes `generated/evidence_ir/<document_key>/evidence_ir.json`
- builds:
  - section anchors from promoted markdown headings
  - block-level evidence spans with line provenance

### Run tracked KG-quality fixtures
```bash
cargo run -p specforge -- kg-bench
```
- runs the tracked fixture set under `crates/specforge/test_data/kg_quality/`
- fails if gold expectations, negative expectations, residual-quality checks, or validation-finding checks drift
- this is the current user-facing truthfulness regression command for the staged KG surface
- the harness can also patch `SourceIR` and `EvidenceIR` inputs for a fixture, including `structured_tables`, `visual_assets`, and `document_sections`, which lets tracked regressions exercise richer structured/semantic cases than plain markdown text alone
- the harness can also patch a fixture-local `CorpusMemory`, which lets tracked regressions prove prior-guided improvements on unseen local phrases without depending on a shared mutable prior file
- the harness can also assert canonical actor-signal relations directly, so tracked regressions can lock `Drives` versus `Reads` semantics instead of checking only actor-port projections or relation counts
- the harness can also assert per-signal canonical direction directly, so tracked regressions can lock section-heading-driven protocol-family direction recovery instead of relying only on validator aggregate metrics
- the harness can also assert canonical semantic candidates and decisive/non-decisive semantic arbitration directly, so fixture regressions can lock contested versus settled signal meaning explicitly
- the harness can also assert persisted validation metric values directly at the evidence, semantic, and intent stages and patch `SourceIR` visual assets, so tracked regressions can now lock cross-modality grounding behavior, direct VLM timing-note semantics, as well as canonical IR shape
- current tracked fixtures cover blocked heuristic leakage, weaker alias-dependent semantics that must stay explicitly caveated, both positive and negative cross-modality grounding behavior, both positive and negative direct VLM timing-note semantic grounding, caption-vs-VLM visual semantic conflicts, AMBA-style `Source`-column and `Destination`-column gold paths, representative APB `Requester` / `Completer` and setup/access timing gold paths, representative AXI width-only plus prose-direction and next-cycle timing gold paths, representative AHB section-heading and wait-state timing gold paths, bogus actor-attribution rejection for AMBA-style `Source`-column infrastructure rows, field-table misclassification rejection for `Bits | Name | Description` layouts, spurious timing-annotation rejection for low-value VLM labels like `T0` and `Addr 1`, and prior-guided gold/negative pairs for actor-taxonomy direction recovery, bounded temporal recovery, semantic-role recovery, and visual-caption semantic recovery
  - visual evidence items from `SourceIR` visual assets
  - explicit `describes` and `cites` links for caption and figure/table references
  - extracted statements classified into source facts, derived rules, local design decisions, or explicit abstractions

### Preview a SemanticIR artifact
```bash
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run
```
- prints computed `SemanticIR` JSON without writing artifacts
- requires an `EvidenceIR` JSON artifact
- useful for checking:
  - actor and interface discovery
  - phase, invariant, and gate extraction
  - abstraction and decomposition candidate creation
  - semantic residual decisions before materialization

### Materialize a SemanticIR artifact
```bash
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json
```
- writes `generated/semantic_ir/<document_key>/semantic_ir.json`
- builds:
  - actors from role-like evidence terms or inferred channel groupings
  - interfaces from explicit typed signal declarations (`Signal X is input/output width N.`), markdown signal-description table rows when the section title identifies direction context (e.g. "Manager signals" → output, "Subordinate signals" → input), and heuristic co-mention grouping for remaining UPPERCASE tokens with expanded stop-word filtering and large-set noise reduction
  - backend-neutral system contract and init-assignment records from explicit `Clock ...`, `Reset ...`, and `Init ...` statements when the evidence is explicit enough, including reset kind, polarity, assertion/release timing, and target semantics
  - backend-neutral guarded/action control fragments from explicit `Block ...` statements when the evidence is explicit enough
  - phases from section structure and sequencing language
  - invariants, contracts, and gates from heuristic semantic lifting
  - abstractions and decomposition candidates with supporting statement ids
  - residual decisions when actor boundaries, interface grouping, or visual semantics remain ambiguous

### Preview an IntentIR artifact
```bash
cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run
```
- prints computed `IntentIR` JSON without writing artifacts
- requires a `SemanticIR` JSON artifact
- useful for checking:
  - canonical intent identity
  - actor responsibilities
  - behavior and constraint canonicalization
  - assumptions and preserved residual decisions

### Materialize an IntentIR artifact
```bash
cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json
```
- writes `generated/intent_ir/<document_key>/intent_ir.json`
- builds:
  - a canonical intent identity from the document and semantic theme
  - actor responsibilities from semantic actors, contracts, and phase overlap
  - canonical interface inventory carried forward from typed semantic interfaces
  - canonical backend-neutral system contract and init assignments carried forward from explicit semantic records, including first-class reset polarity/assertion/release/target semantics
  - canonical backend-neutral guarded/action fragments carried forward from typed semantic control blocks
  - behaviors from phases, contracts, and gate-like sequencing rules
  - constraints from invariants, assertions, and interface-coupled rules
  - assumptions from abstractions and conservative canonicalization heuristics
  - residual decisions preserved from `SemanticIR` plus any canonicalization-specific ambiguity

### Preview an `.fsm` adapter artifact
```bash
cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run
```
- prints computed adapter JSON without writing artifacts
- requires an `IntentIR` JSON artifact
- useful for checking:
  - root-kind selection (`?dt:name`, `?fsm:name`, `?top:name`, or future broader roots)
  - canonical signal inventory, including direction/width hints when known
  - canonical control-block candidates plus any explicit regular-state and transition candidates
  - explicit module/top candidate counts when broader-root composition facts are present
  - renderability blockers and required canonical enrichments
  - adapter-side residual decisions before materialization

### Materialize an `.fsm` adapter artifact
```bash
cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm
```
- writes `generated/adapters/fsm/<document_key>/adapter.json`
- currently:
  - selects `?dt:name` for honest standalone DT cases, `?fsm:name` when explicit regular states and transition targets are present, and `?top:name` when explicit module/top composition facts are present
  - consumes canonical interface inventory, backend-neutral system/init records, canonical symbol-definition sections, structured control blocks, explicit regular-state/transition records, and explicit module/top composition facts from `IntentIR`
  - writes a real standalone, structured, or explicit top-root `.fsm` file when every referenced signal has explicit width/direction, every rendered control fragment is fully typed, any sequential/stateful case has explicit system/init facts, and every explicit top child/link detail is renderable without invention
  - keeps blocked cases explicit when signal roles, widths, system/init facts, reset polarity cannot be preserved honestly through the reset signal name, selector/test-node predicates or compound-update targets fall outside the honest shorthand slice, the regular-state graph or top composition topology are incomplete, or compatibility-level `?mod:name` / `?module:name` spellings would require the adapter to invent a direct-module canonical distinction
- current explicit renderable cues:
  - `Signal DATA_IN is input width 8.`
  - `Signal DATA_OUT is output width 8.`
  - `Block route_data: DATA_OUT = DATA_IN.`
  - `Signal clk is input width 1.`
  - `Signal rst is input width 1.`
  - `Signal rst_n is input width 1.`
  - `Clock clk.`
  - `Reset rst is synchronous active high.`
  - `Reset rst_n is asynchronous active low.`
  - `Init ACC = 8'0.`
  - `Block accumulate: ACC <- DATA_IN.`
  - `Block decode select MODE when MODE == mode_t.idle: OUT = 1.`
  - `Block choose select A | B when A | B == 0: X = 1.`
  - `Block choose select A | B when A | B == 1: Y = 1.`
  - `Block bump: ACC += STEP.`
  - `Constant STEP = 8'1.`
  - `Define D0 = 8'4.`
  - `Param RESET_VALUE = 8'0.`
  - `Enum mode_t busy = 1.`
  - `SyncReset clear_acc: ACC <- RESET_VALUE.`
  - `AsyncReset clear_pulse: public PULSE_OUT = 0.`
  - `State idle is initial.`
  - `State busy.`
  - `Transition idle -> busy when GO.`
  - `Transition busy -> idle when DONE.`
  - `Top datapath.`
  - `Top datapath port result_data is output width 8.`
  - `Top datapath child producer uses module producer_core.`
  - `Top datapath link producer.output_data -> consumer.input_data.`
  - `Module producer_core signal output_data is output width 8.`
  - `Module producer_core block produce: output_data = 8'3.`
- when explicit `active high` / `active low` wording is omitted, the current canonical reset slice infers active-low from `_n` / `_b` reset naming and otherwise falls back to active-high with lower automation confidence

## Planned user workflow
1. provide a source specification
2. build `SourceIR`
3. build `EvidenceIR` from text, figures, captions, charts, and other grounded evidence
4. build `SemanticIR`
5. build canonical `IntentIR`
6. lower `IntentIR` through an adapter such as `.fsm` or RTL
7. validate the result and back-annotate findings

## Planned command shape
- `specforge converge <source> --target <fsm|systemverilog|verilog|vhdl>` defaults to full Ollama-backed VLM image enrichment plus NLP Level 3 backannotation; pass `--vlm-provider skip` and/or `--nlp-provider skip` only when you intentionally want a narrower run
- `specforge ingest <source>`
- `specforge inspect <artifact-or-path>`
- `specforge evidence <source-ir>`
- `specforge semantic <evidence-ir>`
- `specforge intent <semantic-ir>`
- `specforge adapt <intent-ir> --target <fsm|systemverilog|verilog|vhdl>` (today, only `fsm` is implemented)
- `specforge validate <artifact>`
- `specforge project-validation <artifact>...`
- `specforge learn-priors <intent-ir>...`

## Expected user-visible principles
- the tool should be staged and inspectable
- intermediate IR artifacts should be preserved, not hidden
- evidence and provenance should be visible
- images, figures, captions, and charts should stay available as first-class evidence when they carry semantic value
- unresolved ambiguity should be emitted as residual decision packets instead of being hidden in ad hoc notes
- `IntentIR` should remain backend-independent
- `.fsm` is only one adapter target among several

## Current limitation
- `SourceIR`, the first real `EvidenceIR` pass, the first real `SemanticIR` pass, and the first real `IntentIR` pass are implemented
- the current `EvidenceIR` extraction logic is still heuristic and does not yet perform deeper OCR, chart extraction, or semantic lifting from visual regions
- the current `SemanticIR` extraction logic is still heuristic and conservative, so later `IntentIR` work will need refinement rather than semantic invention
- the current `IntentIR` canonicalization logic is still heuristic and conservative, so adapter work should refine backend lowering rather than treat the current pass as a complete semantic endpoint
- the first `.fsm` adapter slices are implemented and can now emit standalone renderable `?dt:name` text for explicit canonical combinational and sequential DT cases, structured renderable `?fsm:name` text for explicit canonical state-graph cases, canonical symbol-definition/reset-role lowering, selector/test-node branches, compound-update shorthand for honest canonical cases, and first-slice renderable `?top:name` text for explicit canonical composition cases, but unsupported selector/predicate shapes still remain deferred and compatibility-level `?mod:name` / `?module:name` spellings stay outside the current canonical root-kind model
- `specforge validate <artifact>` now backannotates the latest validation report into the artifact and writes a stage-local `validation_report.json` sidecar
- `specforge project-validation <artifact>...` now turns those persisted reports into tracked markdown continuity docs, and broader adapter validation remains intentionally deferred until semantic truthfulness is stronger
- `specforge learn-priors <intent-ir>...` now turns validated canonical `IntentIR` artifacts into a local `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`, and the current slice learns reusable actor-taxonomy, semantic-role, and temporal-language priors without letting that memory author canonical facts directly
- `specforge evidence <source-ir>` and `specforge converge <source>` now consult that same local prior store by default through `--prior-memory generated/prior_memory/corpus_memory.json`; missing prior files are treated as a no-op, and the first bounded consumers now use:
  - actor-taxonomy priors to interpret explicit local actor terms already present in section headings and `Source` / `Destination` table columns, including structural KG recovery from width-only section-guided signal tables
  - semantic phrase priors to interpret locally grounded non-hardcoded role phrases that normalize to learned semantic evidence
- `SemanticIR` now also consults the persisted prior-memory path indirectly through the upstream `EvidenceIR`, and the first bounded temporal consumer uses temporal phrase priors only as a fallback for local timing text when the built-in cycle-window parser cannot recover the timing window directly

## Where to look next
- `README.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `RUST_CODEBASE_ANALYSIS.md`
