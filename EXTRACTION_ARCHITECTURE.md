# EXTRACTION_ARCHITECTURE
## Purpose
This document records the complete and authoritative architectural vision for extracting high-quality `IntentIR` from chip protocol specification PDFs.
It covers every source of information in such documents, the quality gap in each current IR stage, the target architecture for each IR, and the full priority-ordered implementation plan with precise implementation status.
**This document is set in stone as the goal definition.** It is the reference against which every engineering decision in this pipeline shall be evaluated. Update only the status fields and implementation notes; never remove a modality, requirement, or type name without replacing it with something strictly better.
## Related documents
- `ROADMAP.md` — implementation sequence and workstream status (R8/R9/R10)
- `DEVELOPMENT_NOTES.md` — engineering decisions and rationale
- `INTENTIR_SPEC.md` — canonical product boundary and stage definitions
- `RUST_CODEBASE_ANALYSIS.md` — current codebase state and risk assessment

---

## The problem: information lives in six modalities
A chip protocol specification PDF (AMBA AHB, AXI, APB, CHI, etc.) carries implementation-relevant information across six distinct modalities. The current pipeline treats all of them as flat text. **That is the root cause of every extraction quality gap.**

### Modality 1 — Structured tables
Tables are the most information-dense artifacts in a chip spec. They carry five distinct semantic types of content:

- **Signal description tables** — signal name, direction (Manager output / Subordinate output), width (numeric or parameter), description. Must become `InterfaceSignalRecord` entries in `SemanticIR`/`IntentIR`.
- **Encoding tables** — field name with bit-pattern-to-value-name mappings (e.g. `HTRANS[1:0]: 00=IDLE, 01=BUSY, 10=NONSEQ, 11=SEQ`). Must become `SymbolDefinitionRecord` (Enum kind) entries.
- **Register map tables** — register name, offset address, bit fields (name, bits [n:m], access type R/W/RO/WO/RC, reset value, description). Must become `RegisterRecord` entries with `RegisterFieldRecord` children.
- **Timing parameter tables** — parameter name, min/typical/max value, unit. Must become `TimingConstraintRecord` entries.
- **Feature/property tables** — which capabilities are mandatory, optional, prohibited. Must become feature capability records.

Docling natively extracts the full cell grid (header rows + body rows + row/col spans) for every table. **We are now using this for signal tables and encoding tables. All remaining table types need the same treatment.**

### Modality 2 — Timing diagrams
Timing diagrams are often the most *normative* content in a chip spec — more precise than any prose description. A timing diagram for a 4-beat burst transfer contains exact cycle-by-cycle signal values. Currently: cropped PNG + caption. **That image is the most critical content in the spec and it is completely opaque to every IR stage.**

**What is needed:** A VLM (vision-language model) pass over figures classified as timing diagrams to extract:
- Signal names on the vertical axis
- Their values (HIGH / LOW / VALID / X) at each labeled clock cycle
- Annotations (setup time, hold time, cycle labels)

This output becomes a `TimingDiagramRecord` in `EvidenceIR` with typed signal-cycle pairs and timing annotations. Downstream `SemanticIR` parses these into `TimingConstraintRecord`.

### Modality 3 — State and flow diagrams
State machine diagrams define protocol states and transitions more precisely than the prose description. Block diagrams define component topology.

**What is needed:** VLM extraction from state machine diagrams of:
- State names (boxes)
- Transition labels (guard conditions on arrows)
- Initial/final state markers

This directly feeds `RegularStateRecord` and `StateTransitionRecord` in `SemanticIR` (which are currently populated only when the spec uses our formal `State X is initial.` syntax — never from diagrams).

### Modality 4 — Normative prose
Sentences like "HTRANS shall remain NONSEQ during address phase" or "HREADY must be asserted within two cycles". These are behavioral requirements. Currently extracted as `SourceFact` like everything else, producing low-quality invariant extraction.

**What is needed:** Proper NLP classification distinguishing:
- `NormativeStatement` — sentences with `shall`, `must`, `shall not`, `required`, `prohibited` in non-boilerplate sections (**now implemented**)
- `TimingConstraint` — sentences with cycle counts, setup/hold references, latency bounds
- `ConditionalRule` — `when X, Y shall...` / `if A then B` conditional behavioral structures
- `SignalValueConstraint` — `X must be HIGH when Y is asserted`, value constraints on signals

### Modality 5 — Section structure and classification
The document hierarchy carries semantic meaning. "Chapter 2 Signal Descriptions" tells you that the content below is normative signal data. "Appendix A" is not normative. "AMBA Specification Licence" is boilerplate.

**Now delivered:** `ContentSectionRecord` in `SourceIR` with `section_kind: SectionKind` covering `SignalDescription`, `Normative`, `Boilerplate`, `Timing`, `RegisterDescription`, `Glossary`, `Appendix`, `TableOfContents`. This classification drives what `EvidenceIR` does with each section's content — boilerplate sections are now filtered, signal sections trigger typed declaration synthesis.

### Modality 6 — Document metadata
Document number, revision, organization, date. Critical for traceability and version tracking.

**Now delivered:** `DocumentProfile` in `SourceIR` capturing title, page count, table count, figure count, section count.

---

## Quality gap: current extraction vs. target

### SourceIR — current state
- ✅ Structured table cell grids (`StructuredTableRecord` with header/body rows)
- ✅ Table kind classification (`signal_description`, `encoding`, `register_map`, `timing_parameter`, `feature_matrix`, `unknown`)
- ✅ Typed content elements (`ContentElementRecord` with Docling labels)
- ✅ Section hierarchy with `SectionKind` classification
- ✅ Document profile (title, counts)
- ❌ Visual asset classification beyond caption heuristics (diagram type: timing/state/block)
- ❌ For Markdown inputs: structure still not parsed

### EvidenceIR — current state
- ✅ Signal declarations synthesized from `signal_description` tables
- ✅ Encoding enum declarations synthesized from `encoding` tables
- ✅ `NormativeStatement` classification for `shall`/`must`/`shall not` sentences
- ❌ Register field declarations from `register_map` tables
- ❌ Timing constraint records from `timing_parameter` tables
- ❌ `TimingConstraint` and `ConditionalRule` statement sub-classes
- ❌ `VisualObservation` types `Description`, `ChartExtraction`, `TimingDiagramExtraction`, `StateMachineExtraction` never populated

### SemanticIR — current state
- ✅ `parse_signal_table_row` band-aid removed; signal declarations flow from EvidenceIR
- ✅ Boilerplate section filter backed by `SectionKind` (correct layer)
- ✅ Encoding enums flow from EvidenceIR through existing `parse_explicit_symbol_definition`
- ❌ `RegisterRecord`, `RegisterFieldRecord` types not yet defined
- ❌ `TimingConstraintRecord` type not yet defined
- ❌ State machine extraction from VLM diagram records not yet wired

### IntentIR — current state
- ✅ 17+ AHB signals with direction+width from structured tables
- ✅ Encoding enums beginning to populate for HTRANS, HBURST etc.
- ❌ No `register_records` field
- ❌ No `timing_constraints` field
- ❌ Behavioral requirements identified but not distinguished from prose

---

## Target architecture: what each IR should contain

### SourceIR (SOTA capture layer)
```
SourceIr {
    page_artifacts          ✅ page images + geometry
    visual_assets           ✅ figure/table crops + captions
    structured_tables       ✅ Docling cell grids with header/body rows + table_kind
    content_elements        ✅ typed text elements in reading order
    document_sections       ✅ section hierarchy with SectionKind
    document_profile        ✅ title, counts
    [next] visual_asset_kind  TODO: classify each figure as timing_diagram / state_machine / block_diagram
}
```
Key principle: `SourceIR` drops nothing. Every fact Docling provides is captured in a typed form. The promoted markdown is a convenience view only — it is never the sole source of truth.

### EvidenceIR (typed evidence per modality)
```
EvidenceIr {
    text evidence spans          ✅ existing
    visual evidence items        ✅ existing
    evidence links               ✅ existing (caption/figure-ref)
    signal declarations          ✅ synthesized from signal_description tables
    encoding enum declarations   ✅ synthesized from encoding tables
    NormativeStatement class     ✅ shall/must/shall-not sentences classified
    [next] register declarations TODO: RegisterRecord from register_map tables
    [next] timing declarations   TODO: TimingConstraintRecord from timing_parameter tables
    [next] TimingConstraint class  TODO: sub-classify timing sentences
    [future] VLM visual content  TODO: TimingDiagramRecord, StateMachineExtraction from VLM
}
```
Key principle: `EvidenceIR` is typed. Every piece of evidence has a class. Downstream stages receive clean typed records, not raw text to re-parse.

### SemanticIR (semantic lifting, no heuristic re-parsing)
```
SemanticIr {
    actors                  ✅ role-term heuristics (acceptable)
    interfaces              ✅ enriched by signal declarations from EvidenceIR
    symbol definitions      ✅ formal + encoding enums from EvidenceIR
    system contract         ✅ formal syntax
    state machines          ✅ formal syntax; [future] from VLM
    control blocks          ✅ formal syntax
    invariants              ✅ sharpened by NormativeStatement; [next] TimingConstraint
    [next] register_records TODO: carry forward from EvidenceIR
    [next] timing_constraints TODO: carry forward from EvidenceIR
}
```
Key principle: `SemanticIR` lifts from typed evidence. It does not re-parse raw text to recover what `SourceIR`/`EvidenceIR` should have captured.

### IntentIR (canonical backend-independent intent)
Carries everything `SemanticIR` produces, canonicalized. The quality of `IntentIR` is determined entirely by the quality of `SourceIR` and `EvidenceIR`.

---

## Implementation plan: all steps in priority order

### Tier 1 — Structured capture from what Docling already provides
These use zero additional dependencies. Docling already extracts this data; we just were not storing or using it.

**Step 1.1** — `SourceIR`: structured table cell grids from Docling `TableItem.data.grid`
- Types: `StructuredTableRecord`, `StructuredTableCellRecord` in `source.rs`
- `SourceIr.structured_tables: Vec<StructuredTableRecord>` — Status: **✅ done**

**Step 1.2** — `SourceIR`: typed content elements from Docling type labels
- Types: `ContentElementRecord`, `ContentElementKind` in `source.rs`
- `SourceIr.content_elements: Vec<ContentElementRecord>` — Status: **✅ done**

**Step 1.3** — `SourceIR`: section hierarchy with semantic classification
- Types: `ContentSectionRecord`, `SectionKind` in `source.rs`
- `SourceIr.document_sections: Vec<ContentSectionRecord>` — Status: **✅ done**

**Step 1.4** — `SourceIR`: document profile
- Types: `DocumentProfile` in `source.rs`
- `SourceIr.document_profile: Option<DocumentProfile>` — Status: **✅ done**

**Step 1.5** — `SourceIR`: table kind classification
- Types: `TableKind` enum in `source.rs`; `classify_table_kind()` in Python helper
- Values: `signal_description`, `encoding`, `register_map`, `timing_parameter`, `feature_matrix`, `unknown`
- `StructuredTableRecord.table_kind: TableKind` — Status: **✅ done**

**Step 1.6** — `EvidenceIR`: synthesize signal declarations from signal tables
- `synthesize_signal_declarations()` reads `source_ir.structured_tables` where `table_kind == SignalDescription`
- Infers direction from `ContentSectionRecord.section_kind` + title keywords
- Synthesizes `Signal X is output width N.` / `Signal X is input.` formal declarations
- Flows through `SemanticIR`'s existing `parse_explicit_signal_declaration` — Status: **✅ done**

**Step 1.7** — `EvidenceIR`: synthesize encoding enum declarations
- `synthesize_encoding_declarations()` reads `source_ir.structured_tables` where `table_kind == Encoding`
- Derives enum name from table caption / section title (e.g. "HTRANS encoding" → enum `HTRANS`)
- Each body row → `Enum HTRANS IDLE = 0.` formal declaration
- Flows through `SemanticIR`'s existing `parse_explicit_symbol_definition` (Enum kind) — Status: **✅ done**

**Step 1.8** — `EvidenceIR`: `NormativeStatement` statement class
- Extend `StatementClass` with `NormativeStatement`
- Sentences with `shall`/`must`/`shall not`/`required to`/`prohibited` in non-boilerplate sections
- `SemanticIR.build_invariants()` uses `NormativeStatement` with higher precision — Status: **✅ done**

**Step 1.9** — `SemanticIR`: remove `parse_signal_table_row` band-aid
- Signal declarations flow from `EvidenceIR` through the correct architectural path
- `parse_signal_table_row`, `section_title_by_id` lookup, and table-row-parsing block removed — Status: **✅ done**

### Tier 2 — Remaining table types as typed records
Still zero additional dependencies. Structured cell data is already in `SourceIR`.

**Step 2.1** — `EvidenceIR` + `SemanticIR` + `IntentIR`: register map extraction
- Types to add: `RegisterRecord`, `RegisterFieldRecord` in `semantic.rs`
- `synthesize_register_declarations()` in `evidence.rs`: reads tables where `table_kind == RegisterMap`
- Extracts: register name, offset address, bit fields (name, bits [n:m], access type, reset value)
- `SemanticIr.register_records: Vec<RegisterRecord>` and `IntentIr.register_records` carry them forward
- Status: **❌ not started**

**Step 2.2** — `EvidenceIR` + `SemanticIR` + `IntentIR`: timing parameter extraction
- Types to add: `TimingConstraintRecord` in `semantic.rs`
- `synthesize_timing_declarations()` in `evidence.rs`: reads tables where `table_kind == TimingParameter`
- Extracts: parameter name (e.g. tSU, tHD), min/typ/max numeric values, unit
- `SemanticIr.timing_constraints: Vec<TimingConstraintRecord>` and `IntentIr.timing_constraints`
- Status: **❌ not started**

**Step 2.3** — `EvidenceIR`: `TimingConstraint` and `ConditionalRule` statement sub-classes
- Extend `StatementClass` with `TimingConstraint` (cycle counts, latency bounds, setup/hold in prose)
- Extend `StatementClass` with `ConditionalRule` (`when X, Y shall...` / `if A then B` structures)
- Sharpen `SemanticIR` invariant extraction to distinguish timing requirements from behavioral rules
- Status: **❌ not started**

### Tier 3 — Visual content understanding (requires VLM integration)
These steps require an external VLM call (OpenAI GPT-4V, Anthropic Claude Vision, Google Gemini Vision). The type system in `VisualObservation` already reserves the observation kinds needed: `Description`, `Classification`, `ChartExtraction`, `OcrTranscription`, `TableTranscription`, `FormulaTranscription`.

**Step 3.1** — `SourceIR`/`EvidenceIR`: visual asset type classification
- Classify each `VisualAsset` with a specific diagram type beyond caption-text heuristics:
  - `timing_diagram` — waveforms on horizontal time axis (the most normative content in chip specs)
  - `state_machine_diagram` — boxes and arrows with guard labels
  - `block_diagram` — component rectangles with connections
  - `truth_table` — tabular with binary inputs/outputs
  - `register_bitfield` — horizontal bit field layout
  - `flow_chart` — diamond decision nodes
- Add `VisualAsset.diagram_kind: DiagramKind` field to `source.rs`
- A lightweight classifier on caption text reliably handles most chip spec figures
- Status: **❌ not started**

**Step 3.2** — `EvidenceIR`: VLM timing diagram extraction → `TimingDiagramRecord`
- For each `VisualAsset` classified as `timing_diagram`, send image + caption to VLM
- Extract: signal names on vertical axis, their values (HIGH/LOW/VALID/X) per labeled clock cycle, timing annotations (tSU, tHD, cycle counts)
- Output: `VisualObservation.kind = TimingDiagramExtraction` with a structured `TimingDiagramRecord` containing typed signal-cycle pairs
- Downstream: `SemanticIR` parses into `TimingConstraintRecord` entries
- Status: **❌ not started**

**Step 3.3** — `EvidenceIR`: VLM state machine extraction → `StateMachineExtraction`
- For each `VisualAsset` classified as `state_machine_diagram`, send to VLM
- Extract: state names (boxes), transition labels (guard conditions on arrows), initial state marker
- Output: `VisualObservation.kind = StateMachineExtraction` with typed state/transition records
- Downstream: `SemanticIR` parses into `RegularStateRecord` + `StateTransitionRecord` entries
- Status: **❌ not started**

### Tier 4 — Architecture hardening and validation

**Step 4.1** — Validation pipeline (R7 in ROADMAP)
- After each stage: validate artifact health (coverage score, confidence distribution, residual rate)
- Back-annotate findings: if `IntentIR` has low signal coverage, report which tables were unclassified and why
- Reproducible artifact-linked reports
- Status: **❌ not started**

**Step 4.2** — Domain knowledge grounding
- For AMBA family: build a reference vocabulary of known signal names, protocol terms, encoding values
- Use as a prior: if signal HADDR appears without a declared width, infer from the spec’s property table
- Detect version-specific features (AHB5 exclusive access vs. earlier AHB)
- Status: **❌ not started**

**Step 4.3** — Multi-pass enrichment architecture
- `SourceIR` pass 1: Docling structural extraction (current)
- `EvidenceIR` pass 1: typed evidence from structured records (current + Tier 2)
- `EvidenceIR` pass 2: VLM enrichment for visual evidence (Tier 3)
- Status: **❌ future**

---

## Why this ordering?
- **Tier 1** delivers immediately actionable improvements using only what Docling already provides — zero additional dependencies, high confidence. **Now complete.**
- **Tier 2** extends the same pattern to register maps and timing parameters — still zero dependencies, covers the remaining structured spec content. **Next priority.**
- **Tier 3** unlocks visual content (timing diagrams, state machines) which is qualitatively different but requires an external VLM call — best done after the structured pipeline is solid.
- **Tier 4** adds robustness, validation, and domain adaptation after the core extraction is working.

---

## Definition of top-notch IntentIR for a chip protocol spec
A top-notch `IntentIR` for the AMBA AHB spec would contain:
- All 32+ AHB signals with explicit direction, width, and description
- All encoding enums: HTRANS (IDLE/BUSY/NONSEQ/SEQ), HBURST (SINGLE/INCR/WRAP4/...), HRESP (OKAY/ERROR), HSIZE (8/16/32/...)
- All AHB5 properties and their data types
- All normative protocol requirements as typed invariants (not prose)
- Clock and reset contract (HCLK, HRESETn) with explicit timing semantics
- The AHB transfer state machine (IDLE → NONSEQ → SEQ → ...) from timing diagrams
- Key timing constraints (HREADY hold cycles, data phase duration)
- All optional/required feature flags
- Explicit residual decisions for anything that could not be extracted with confidence

That is the target. Every engineering decision in this pipeline should be evaluated against it.
