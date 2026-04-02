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

**Classification (now partially implemented) — five statement sub-classes:**
- `SignalValueConstraint` — **most specific** — binds a named signal to a specific value/state. `"HAUSER must not change when HREADY is LOW"`. (**now implemented**)
- `TimingConstraint` — sentence references cycle counts, setup/hold, latency bounds. (**now implemented**)
- `ConditionalRule` — `when X, Y shall...` / `if A then B` temporal/causal structure. (**now implemented**)
- `NormativeStatement` — general `shall`/`must` not covered by a more specific class. (**now implemented**)
- `SourceFact` — everything else.

**Critical gap: classification ≠ extraction.**
Knowing a sentence is `SignalValueConstraint` is not the same as knowing what it constrains. The extraction layer is missing. The target is structured records, not classified strings:
```
"HAUSER must not change between cycles when HREADY is LOW"
     ↓ structured extraction
{ subject: HAUSER, constraint: must_not_change,
  condition: { signal: HREADY, state: LOW } }
```

**NLP architecture — four levels:**
- **Level 1 (done):** Keyword classification → routes sentences to sub-classes
- **Level 2 (next):** Syntactic pattern extraction → extracts `SignalConstraintRecord{signal, constraint_kind, target_value, condition}` and `ConditionalRuleRecord{antecedent, consequent}` from classified sentences
- **Level 3 (future):** LLM reclassification → send ambiguous `NormativeStatement` sentences to a small LLM with a structured extraction prompt
- **Level 4 (SOTA):** Fine-tuned NER/RE model → generalises to paraphrases, passive voice, negation, co-reference

**Current false negatives (sentences we miss):**
- `"HWRITE is tied HIGH for the entire burst"` — no must/shall → `SourceFact` (wrong)
- `"HTRANS cannot change during a waited transfer"` — `cannot` not in list → `SourceFact`
- `"Transfer type shall indicate NONSEQ"` — no `be` before value → `NormativeStatement` (not `SignalValueConstraint`)
- `"It must be asserted"` — co-reference; `It` refers to HREADYOUT earlier — no extraction
- `"shall not be asserted when X"` vs `"shall be asserted when not X"` — same class, opposite semantics

**Types needed for Level 2 extraction:**
```rust
SignalConstraintRecord { subject_signal, constraint_kind, target_value, condition_text, negated, confidence }
ConditionalRuleRecord  { antecedent_text, consequent_signal, consequent_action, confidence }
```
These flow as first-class records through `EvidenceIR.signal_constraints` → `SemanticIR.signal_constraints` → `IntentIR.signal_constraints`.

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
- ❌ `VisualAsset.diagram_kind: DiagramKind` — every figure still classified as generic `figure` or `diagram`
- ❌ VLM visual enrichment — all timing diagrams and state machine diagrams completely opaque
- ❌ For Markdown inputs: structure still not parsed

### EvidenceIR — current state
- ✅ Signal declarations synthesized from `signal_description` tables (High confidence)
- ✅ Encoding enum declarations synthesized from `encoding` tables
- ✅ `RegisterRecord` / `RegisterFieldRecord` extracted from `register_map` tables
- ✅ `TimingConstraintRecord` extracted from `timing_parameter` tables
- ✅ `NormativeStatement`, `TimingConstraint`, `ConditionalRule`, `SignalValueConstraint` statement classes
- ❌ `SignalConstraintRecord` — structured extraction from `SignalValueConstraint` sentences (Level 2 NLP)
- ❌ `ConditionalRuleRecord` — structured extraction from `ConditionalRule` sentences (Level 2 NLP)
- ❌ `VisualObservation.TimingDiagramExtraction` / `StateMachineExtraction` — requires VLM

### SemanticIR — current state
- ✅ Signal declarations from EvidenceIR through `parse_explicit_signal_declaration`
- ✅ Boilerplate section filter backed by `SectionKind`
- ✅ Encoding enums through `parse_explicit_symbol_definition`
- ✅ `register_records: Vec<RegisterRecord>` carried forward
- ✅ `timing_constraints: Vec<TimingConstraintRecord>` carried forward
- ❌ `signal_constraints: Vec<SignalConstraintRecord>` — not yet defined
- ❌ State machine extraction from VLM diagram records not yet wired

### IntentIR — current state
- ✅ 17+ AHB signals with direction+width from structured tables
- ✅ Encoding enums (HTRANS, HBURST etc.) via EvidenceIR synthesis
- ✅ `register_records` carried forward
- ✅ `timing_constraints` from timing parameter tables
- ❌ `signal_constraints` — structured constraint records missing
- ❌ State/transition records from VLM diagram extraction missing
- ❌ Behavioral requirements still stored as prose strings, not structured predicates

---

## Target architecture: what each IR should contain

### SourceIR (SOTA capture layer)
```
SourceIr {
    page_artifacts            ✅ page images + geometry
    visual_assets             ✅ figure/table crops + captions + diagram_kind [next]
    structured_tables         ✅ Docling cell grids + table_kind classification
    content_elements          ✅ typed text elements in reading order
    document_sections         ✅ section hierarchy with SectionKind
    document_profile          ✅ title, counts
    [next] DiagramKind field  TODO: timing_diagram / state_machine / block_diagram / register_bitfield
}
```
Key principle: `SourceIR` drops nothing. Every fact Docling provides is captured in a typed form.

### EvidenceIR (typed evidence per modality)
```
EvidenceIr {
    text evidence spans          ✅ existing
    visual evidence items        ✅ existing
    evidence links               ✅ caption/figure-ref
    signal declarations          ✅ from signal_description tables
    encoding enum declarations   ✅ from encoding tables
    register_records             ✅ from register_map tables
    timing_constraints           ✅ from timing_parameter tables
    5 statement sub-classes      ✅ SignalValueConstraint, TimingConstraint, ConditionalRule,
                                    NormativeStatement, SourceFact
    [next] signal_constraints    TODO: SignalConstraintRecord (Level 2 NLP extraction)
    [next] conditional_rules     TODO: ConditionalRuleRecord (Level 2 NLP extraction)
    [future] VLM observations    TODO: TimingDiagramRecord, StateMachineExtraction
}
```
Key principle: `EvidenceIR` is typed. Classification is Stage 1; structured extraction (Level 2) is Stage 2.

### SemanticIR (semantic lifting, no heuristic re-parsing)
```
SemanticIr {
    actors                  ✅ role-term heuristics (acceptable)
    interfaces              ✅ enriched by signal declarations from EvidenceIR
    symbol definitions      ✅ formal + encoding enums from EvidenceIR
    system contract         ✅ formal syntax
    state machines          ✅ formal syntax + [future] from VLM
    control blocks          ✅ formal syntax
    invariants              ✅ NormativeStatement-backed
    register_records        ✅ carried from EvidenceIR
    timing_constraints      ✅ carried from EvidenceIR
    [next] signal_constraints  TODO: structured constraint records
}
```
Key principle: `SemanticIR` lifts from typed evidence. It does not re-parse raw text.

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
- Types: `RegisterRecord`, `RegisterFieldRecord` in `source.rs`; synthesized by `synthesize_register_records()` in `evidence.rs`
- Extracts: register name, offset address, bit fields (name, bits [n:m], access type, reset value)
- `EvidenceIr.register_records` → `SemanticIr.register_records` → `IntentIr.register_records`
- Status: **✅ done**

**Step 2.2** — `EvidenceIR` + `SemanticIR` + `IntentIR`: timing parameter extraction
- Types: `TimingConstraintRecord` in `source.rs`; synthesized by `synthesize_timing_constraints()` in `evidence.rs`
- Extracts: parameter name, min/typ/max values, unit from `timing_parameter` tables
- `EvidenceIr.timing_constraints` → `SemanticIr.timing_constraints` → `IntentIr.timing_constraints`
- Status: **✅ done**

**Step 2.3** — `EvidenceIR`: all five normative statement sub-classes
- `SignalValueConstraint` — `"SIGNAL must be HIGH/LOW/asserted/stable/IDLE/..."` + signal token required
- `TimingConstraint` — cycle counts, setup/hold, latency bound references
- `ConditionalRule` — `when X, Y shall...` / `if A then B` structures
- `NormativeStatement` — general shall/must not matching more specific class
- `SourceFact` — all other descriptive content
- Status: **✅ done**

**Step 2.4** — `EvidenceIR` + `SemanticIR` + `IntentIR`: Level 2 NLP — `SignalConstraintRecord` extraction
- For each `SignalValueConstraint` sentence, extract structured record via syntactic patterns:
  `{subject_signal, constraint_kind, target_value, condition_text, negated, confidence}`
- `SignalConstraintKind`: `MustBeHigh`, `MustBeLow`, `MustBeAsserted`, `MustBeDeasserted`, `MustNotChange`, `MustBeStable`, `MustBeValue{value}`
- Add `synthesize_signal_constraints()` in `evidence.rs`
- `EvidenceIr.signal_constraints` → `SemanticIr.signal_constraints` → `IntentIr.signal_constraints`
- Status: **❌ not started — next**

**Step 2.5** — `EvidenceIR` + `SemanticIR` + `IntentIR`: Level 2 NLP — `ConditionalRuleRecord` extraction
- For each `ConditionalRule` sentence, extract `{antecedent_text, consequent_signal, consequent_action, confidence}`
- These represent the most common protocol behavioral rules: `"when HREADY is LOW, HTRANS shall remain NONSEQ"`
- Status: **❌ not started**

### Tier 3 — Visual content understanding (VLM required)
The type system in `VisualObservation` reserves the needed observation kinds: `Description`, `Classification`, `ChartExtraction`, `TimingDiagramExtraction`, `StateMachineExtraction`, `OcrTranscription`, `TableTranscription`, `FormulaTranscription`. The infrastructure for VLM is already in place. What remains is implementing the enrichment call.

**VLM provider architecture — two-tier approach:**

Tier A — Caption-based classification (zero VLM deps, Docling metadata only):
- `VisualAsset.diagram_kind` is set from caption text patterns in the Python helper
- Reliable for chip specs because captions follow standard patterns: `"Figure N-M: HTRANS timing diagram"`, `"Figure N-M: Transfer state machine"`

Tier B — VLM enrichment (requires running VLM service):
- Triggered by `specforge enrich <source-ir> --vlm-provider <provider>`
- Providers supported:
  - **`ollama`** (recommended for local/offline) — `http://localhost:11434/v1/chat/completions`; use `llava:13b` or `ibm/granite-docling:258m` model; no API key required
  - **`openai`** (cloud) — `https://api.openai.com/v1/chat/completions`; set `OPENAI_API_KEY`; use `gpt-4o` model
  - **`lmstudio`** — `http://localhost:1234/v1/chat/completions`; load a vision model in LM Studio
  - **`skip`** (default) — no VLM enrichment
- The enrichment step is decoupled from `ingest` so it can be retried without re-running Docling
- A `SPECFORGE_VLM_HELPER` env var override enables unit testing without a live VLM

Docling itself provides a `VlmPipeline` that can process entire documents with a VLM. For our use case we prefer selective enrichment (figure-by-figure) after standard Docling ingest, because:
1. It is cheaper (only send figures classified as diagrams, not entire pages)
2. It allows retrying/switching VLM providers without re-running Docling
3. It keeps SourceIR and EvidenceIR cleanly separated

**Step 3.1** — `SourceIR`: `DiagramKind` classification from caption text
- Add `DiagramKind` enum: `TimingDiagram`, `StateMachineDiagram`, `BlockDiagram`, `RegisterBitfield`, `TruthTable`, `FlowChart`, `Unknown`
- Add `VisualAsset.diagram_kind: DiagramKind` field (set in Python helper from caption pattern matching)
- Caption patterns: `"timing diagram"`, `"timing waveform"`, `"transfer state machine"`, `"state diagram"`, `"block diagram"`, etc.
- Status: **❌ not started — next (zero VLM deps)**

**Step 3.2** — `EvidenceIR`/`SourceIR`: VLM enrichment → `TimingDiagramRecord`
- New `specforge enrich` command (or `--enrich-vlm` flag)
- For each `VisualAsset` where `diagram_kind == TimingDiagram`: send image + caption to VLM
- VLM prompt (structured): `"Describe this timing diagram. For each signal shown, list its name and value (HIGH/LOW/X/VALID) at each labeled clock cycle. List any timing annotations."`
- Parse VLM response into `TimingDiagramRecord { signals: Vec<{name, cycles: Vec<{label, value}>}>, annotations: Vec<String> }`
- Store as `VisualObservation { kind: TimingDiagramExtraction, text: json(record) }`
- Downstream `SemanticIR` parses into additional `TimingConstraintRecord` entries
- Status: **❌ not started**

**Step 3.3** — `EvidenceIR`/`SourceIR`: VLM enrichment → `StateMachineExtraction`
- For each `VisualAsset` where `diagram_kind == StateMachineDiagram`: send to VLM
- VLM prompt: `"Describe this state machine diagram. List all states, their names, and any initial/reset markers. For each transition arrow, give the source state, target state, and guard condition label."`
- Parse VLM response into `StateMachineRecord { states: Vec<{name, is_initial}>, transitions: Vec<{from, to, guard}> }`
- Store as `VisualObservation { kind: StateMachineExtraction, text: json(record) }`
- Downstream `SemanticIR` merges these into `RegularStateRecord` + `StateTransitionRecord` entries
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
