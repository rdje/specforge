# EXTRACTION_ARCHITECTURE
## Purpose
This document records the complete architectural vision for extracting high-quality `IntentIR` from chip protocol specification PDFs.
It covers every source of information in such documents, the quality gap in each current IR stage, the target architecture for each IR, and the full priority-ordered implementation plan.
This is a living steering document. Update it whenever new extraction capabilities are added or the vision is refined.
## Related documents
- `ROADMAP.md` — implementation sequence and workstream status
- `DEVELOPMENT_NOTES.md` — engineering decisions and rationale
- `INTENTIR_SPEC.md` — canonical product boundary and stage definitions
- `RUST_CODEBASE_ANALYSIS.md` — current codebase state and risk assessment

---

## The problem: information lives in six modalities
A chip protocol specification PDF (AMBA AHB, AXI, APB, CHI, etc.) carries implementation-relevant information across six distinct modalities. The current pipeline treats all of them as flat text. That is the root cause of every extraction quality gap.

### Modality 1 — Structured tables
Tables are the most information-dense artifacts in a chip spec. They come in at least five distinct types:
- **Signal description tables** — signal name, direction (Manager output / Subordinate output), width (numeric or parameter), description
- **Encoding tables** — field name with bit-pattern-to-value-name mappings (e.g. `HTRANS[1:0]: 2'b00=IDLE, 2'b01=BUSY, 2'b10=NONSEQ, 2'b11=SEQ`)
- **Register map tables** — register name, offset address, bit fields (name, bits [n:m], access type R/W/RO/WO/RC, reset value, description)
- **Timing parameter tables** — parameter name, min/typical/max value, unit
- **Feature / property tables** — which capabilities are mandatory, optional, prohibited

Docling natively extracts the full cell grid (header rows + body rows + row/col spans) for every table. The current pipeline saves only a cropped PNG image and caption text. The structured cell data is extracted but never stored anywhere.

### Modality 2 — Timing diagrams
Timing diagrams are often the most *normative* content in a chip spec — more precise than any prose description. A timing diagram for a burst transfer shows exact cycle-by-cycle signal values, setup/hold annotations, and labeled clock edges. Currently: cropped PNG + optional caption. The image is completely opaque to every downstream IR stage.

**What is needed:** A vision-language model (VLM) pass over figures classified as timing diagrams to extract:
- Signal names on the vertical axis
- Their logical values (HIGH / LOW / X / Z / VALID / UNKNOWN) at each labeled cycle
- Timing annotations (tSU, tHD, cycle counts)
- Any text labels on the diagram

This populates `VisualObservation.kind = TimingDiagramExtraction` in `EvidenceIR` and ultimately produces `TimingConstraintRecord` in `SemanticIR`.

### Modality 3 — State and flow diagrams
State machine diagrams (and occasionally flow charts) define protocol control flow more precisely than prose. A state machine diagram for AHB transfer control shows states (IDLE, NONSEQ, SEQ, BUSY), transitions, and guard conditions.

**What is needed:** VLM extraction of:
- State names (boxes)
- Transition labels (guard conditions on arrows)
- Initial/reset state marker

This feeds directly into `RegularStateRecord` and `StateTransitionRecord` in `SemanticIR` (which are currently populated only when the spec uses our formal `State X is initial.` syntax).

### Modality 4 — Normative prose
Sentences like "HTRANS shall remain NONSEQ throughout a fixed-length burst" or "HREADY must be asserted within two cycles of HSEL". These are behavioral requirements. They currently become `SourceFact` like everything else, producing low-quality invariant extraction.

**What is needed:** Richer statement classification distinguishing:
- `NormativeStatement` — sentences with `shall`, `must`, `shall not`, `required`, `prohibited` in non-boilerplate sections
- `TimingConstraint` — sentences with cycle counts, setup/hold references, latency bounds
- `ConditionalRule` — `when X, Y shall...` / `if A then B` structures

### Modality 5 — Section structure and classification
The document section hierarchy carries strong semantic intent. "Chapter 2 Signal Descriptions" tells downstream stages that the content below is a signal description. "AMBA Specification Licence" is boilerplate. Currently we discover this heuristically in `SemanticIR` — the stage that is supposed to do semantic *lifting*, not document navigation.

**What is needed (now delivered):** `ContentSectionRecord` in `SourceIR` with `section_kind: SectionKind` (`SignalDescription`, `Normative`, `Boilerplate`, `Timing`, `RegisterDescription`, `Glossary`, `Appendix`). All downstream stages read this directly.

### Modality 6 — Document metadata
Document number, revision, organization, date. Critical for traceability and version tracking. Captured in `DocumentProfile` in `SourceIR`.

---

## Quality gap: what each IR currently misses

### SourceIR gaps (before this implementation slice)
- Does not extract structured table cell data (only image + caption)
- Does not type text elements (heading, body, list, caption, code all look the same downstream)
- Does not classify sections (`Boilerplate` vs `SignalDescription` vs `Normative`)
- Does not capture document profile (title, version, counts)
- For Markdown inputs: does not parse structure at all

### EvidenceIR gaps (before this implementation slice)
- Table rows processed as raw pipe-delimited strings — no typed signal/encoding/register declarations
- All statements classified as `SourceFact` (~99% of real content)
- No typed distinction between normative requirements and explanatory prose
- `VisualObservation` types `OcrTranscription`, `Description`, `ChartExtraction`, `TableTranscription`, `FormulaTranscription` exist in the type system but are never populated
- Caption/figure binding done heuristically on markdown text when Docling already resolved it natively

### SemanticIR gaps (consequence of upstream gaps)
- Had to compensate with ad hoc parsers for information that should have flowed from `SourceIR`/`EvidenceIR`
- `parse_signal_table_row` band-aid re-parsed pipe-delimited table text that Docling already structured
- Boilerplate filtering logic placed in `SemanticIR` (wrong layer)
- Section title lookup placed in `SemanticIR` (wrong layer)
- No register map records
- No timing constraint records
- State machine extraction only from formal `State X is initial.` syntax — never from diagrams

### IntentIR gaps (inherited from upstream)
- Carries whatever SemanticIR managed to rescue from shallow evidence
- Signal inventory: direction/width known only for signals present in our formal syntax or (now) signal tables
- No register records
- No timing constraint records
- No encoding enum definitions from spec tables

---

## Target architecture: what each IR should contain

### SourceIR (SOTA capture layer)
```
SourceIr {
    page_artifacts          ✅ page images + geometry
    visual_assets           ✅ figure/table crops + captions
    structured_tables       ✅ NEW: Docling cell grids with header/body rows
    content_elements        ✅ NEW: typed text elements in reading order
    document_sections       ✅ NEW: section hierarchy with SectionKind
    document_profile        ✅ NEW: title, version, counts
    [future] table_kind     TODO: classify each table as signal/encoding/register/timing
}
```
Key principle: `SourceIR` drops nothing. Every fact Docling provides is captured in a typed form. The promoted markdown is a convenience view only — it is never the sole source of truth.

### EvidenceIR (typed evidence per modality)
```
EvidenceIr {
    text evidence spans     ✅ existing
    visual evidence items   ✅ existing
    evidence links          ✅ existing (caption/figure-ref)
    signal declarations     ✅ NEW: synthesized from signal tables
    [next] encoding defs    TODO: enum declarations from encoding tables
    [next] register defs    TODO: register field declarations from register tables
    [next] timing params    TODO: timing constraint records from timing tables
    [future] VLM content    TODO: visual descriptions from timing/state diagrams
    [next] normative class  TODO: NormativeStatement class (not just SourceFact)
}
```
Key principle: `EvidenceIR` is typed. Every piece of evidence has a class. Downstream stages receive clean typed records, not raw text to re-parse.

### SemanticIR (semantic lifting, no heuristic re-parsing)
```
SemanticIr {
    actors                  ✅ role-term heuristics (acceptable)
    interfaces              ✅ now enriched by signal declarations from EvidenceIR
    symbol definitions      ✅ formal syntax; [next] enriched by encoding tables
    system contract         ✅ formal syntax only
    state machines          ✅ formal syntax; [future] from VLM diagram extraction
    control blocks          ✅ formal syntax
    invariants              [next] sharpened by NormativeStatement classification
    [next] register records TODO: carry forward from EvidenceIR
    [next] timing records   TODO: carry forward from EvidenceIR
}
```
Key principle: `SemanticIR` lifts from typed evidence. It does not re-parse raw text to recover what `SourceIR`/`EvidenceIR` should have captured.

### IntentIR (canonical backend-independent intent)
Carries everything `SemanticIR` produces, canonicalized. The quality of `IntentIR` is determined entirely by the quality of `SourceIR` and `EvidenceIR`.

---

## Implementation plan: all steps in priority order

### Tier 1 — Structured capture from what Docling already provides (in progress)
These use zero additional dependencies. Docling already extracts this data; we just were not storing or using it.

**Step 1.1** — `SourceIR`: extract structured table cell grids from Docling `TableItem.data.grid`
- Add `StructuredTableRecord`, `StructuredTableCellRecord` to `source.rs`
- Update Python helper to collect header/body rows per table
- `SourceIr.structured_tables: Vec<StructuredTableRecord>`
- Status: **done**

**Step 1.2** — `SourceIR`: extract typed content elements from Docling labels
- Add `ContentElementRecord`, `ContentElementKind` to `source.rs`
- Python helper maps `DocItemLabel` → kind (title, section_header, body_text, list_item, code, caption, footnote, formula)
- Reading order, page provenance, source_ref carried on every element
- `SourceIr.content_elements: Vec<ContentElementRecord>`
- Status: **done**

**Step 1.3** — `SourceIR`: extract section hierarchy with semantic classification
- Add `ContentSectionRecord`, `SectionKind` to `source.rs`
- Python helper classifies each heading title → `signal_description`, `boilerplate`, `normative`, `timing`, `register_description`, `glossary`, `appendix`, `table_of_contents`
- `SourceIr.document_sections: Vec<ContentSectionRecord>`
- Status: **done**

**Step 1.4** — `SourceIR`: capture document profile
- `SourceIr.document_profile: Option<DocumentProfile>` (title, page/table/figure/section counts)
- Status: **done**

**Step 1.5** — `EvidenceIR`: synthesize signal declarations from signal tables
- Read `source_ir.structured_tables`; identify signal description tables by header pattern ("name"/"signal"/"port" in first header cell)
- Infer direction from `ContentSectionRecord.section_kind` and title keywords
- Synthesize formal `Signal X is output width N.` or `Signal X is input.` declarations
- Add to `extracted_statements` with `StatementClass::SourceFact` and High confidence
- These flow naturally through `SemanticIR`'s existing `parse_explicit_signal_declaration`
- Status: **in progress**

**Step 1.6** — `SemanticIR`: remove `parse_signal_table_row` band-aid
- Signal declarations now arrive properly from `EvidenceIR`
- Remove `parse_signal_table_row`, `section_title_by_id` lookup, and the table-row-parsing block from `build_interfaces`
- The SemanticIR boilerplate section filter (`is_boilerplate_section_title`) remains but is now backed by `SectionKind` classification from `SourceIR` rather than heuristic title matching
- Status: **in progress**

### Tier 2 — Typed evidence from other table categories (next slice)
Still zero additional dependencies. Structured cell data is already in `SourceIR`.

**Step 2.1** — `SourceIR`: add `table_kind` classification to `StructuredTableRecord`
- Python helper analyzes header text of each table and classifies:
  - `signal_description` — first column = Name/Signal/Port, other columns include Width/Direction
  - `encoding` — columns include Value/Encoding/Binary/Code + Name/Description
  - `register_map` — columns include Offset/Address + Name + Bits + Access + Reset
  - `timing_parameter` — columns include Parameter + Min + Typ/Max + Unit
  - `feature_matrix` — columns include Feature/Property + support levels
  - `unknown` — does not match any pattern
- `StructuredTableRecord.table_kind: TableKind`

**Step 2.2** — `EvidenceIR`: encoding table extraction → enum declarations
- Identify tables with `table_kind == Encoding`
- Extract caption/section name as the enum's signal name (e.g. "HTRANS Encoding" → enum name "HTRANS")
- Each body row → `Enum <name> <member> = <value>.` formal declaration
- These flow to `SemanticIR`'s existing `parse_explicit_symbol_definition` (Enum kind)
- Chip specs become correctly modeled: HTRANS, HBURST, HRESP, etc. as proper enums in `IntentIR`

**Step 2.3** — `EvidenceIR`: `NormativeStatement` statement class
- Extend `StatementClass` enum with `NormativeStatement`
- In `classify_statement()`: if sentence contains `shall`/`must`/`shall not`/`required to` AND is in a non-boilerplate section → `NormativeStatement`
- `SemanticIR.build_invariants()` uses `NormativeStatement` with higher precision than the current "any sentence with shall" heuristic

**Step 2.4** — `EvidenceIR` + `SemanticIR`: register map type system
- Add `RegisterRecord`, `RegisterFieldRecord` types to `SemanticIR`
- Add register declaration synthesis from register map tables in `EvidenceIR`
- `SemanticIR` and `IntentIR` carry `register_records: Vec<RegisterRecord>`

**Step 2.5** — `EvidenceIR` + `SemanticIR`: timing parameter type system
- Add `TimingConstraintRecord` type
- Synthesize from timing parameter tables
- `SemanticIR` and `IntentIR` carry `timing_constraints: Vec<TimingConstraintRecord>`

### Tier 3 — Visual content understanding (requires VLM integration)
These steps require an external VLM call (OpenAI GPT-4V, Anthropic Claude Vision, Google Gemini Vision). The type system is already reserved (`VisualObservation` kinds: `Description`, `Classification`, `ChartExtraction`, `OcrTranscription`).

**Step 3.1** — `SourceIR` or `EvidenceIR`: visual asset classification
- Classify each `VisualAsset` more precisely than caption-text heuristics:
  - `timing_diagram` — waveforms on horizontal time axis
  - `state_machine_diagram` — boxes and arrows with guard labels
  - `block_diagram` — component rectangles with connections
  - `truth_table` — tabular with binary inputs/outputs
  - `register_bitfield` — horizontal bit field layout
  - `flow_chart` — diamond decision nodes
- A lightweight classifier on caption text + image can do this reliably for chip specs

**Step 3.2** — `EvidenceIR`: VLM timing diagram extraction
- For each `VisualAsset` classified as `timing_diagram`, send image + caption to VLM
- Extract: signal names on vertical axis, their values per cycle, timing annotations
- Produce `VisualObservation.kind = TimingDiagramExtraction` with structured text
- Downstream: `SemanticIR` parses this into `TimingConstraintRecord`

**Step 3.3** — `EvidenceIR`: VLM state machine extraction
- For each `VisualAsset` classified as `state_machine_diagram`, send to VLM
- Extract: state names, transitions with guards, initial state
- Produce `VisualObservation.kind = StateMachineExtraction` with structured text
- Downstream: `SemanticIR` parses into `RegularStateRecord` + `StateTransitionRecord`

### Tier 4 — Architecture hardening (future)

**Step 4.1** — Multi-pass enrichment
- `SourceIR` pass 1: Docling structural extraction (current)
- `SourceIR` pass 2 [optional]: table type classification if Docling classification insufficient
- `EvidenceIR` pass 1: typed evidence from structured records (current + Tier 2)
- `EvidenceIR` pass 2 [optional]: VLM enrichment for visual evidence (Tier 3)

**Step 4.2** — Validation pipeline (R7 in ROADMAP)
- After each stage: validate artifact health (coverage, confidence distribution, residual rate)
- Back-annotate findings: if `IntentIR` has low signal coverage, report which tables were unclassified and why

**Step 4.3** — Domain knowledge grounding
- For AMBA family: build a reference vocabulary of known signal names, protocol terms, encoding values
- Use as a prior: if signal HADDR appears without a declared width, infer ADDR_WIDTH from the property table
- Detect version-specific features (AHB5 exclusive access vs. earlier AHB)

---

## Why this ordering?
- Tier 1 delivers immediately actionable improvements using only what Docling already provides — zero additional dependencies, high confidence
- Tier 2 extends the same pattern to other table types — still zero dependencies, covers the bulk of structured spec content
- Tier 3 unlocks visual content which is qualitatively different but requires an external call — best done after the structured pipeline is solid
- Tier 4 adds robustness and domain adaptation after the core extraction is working

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
