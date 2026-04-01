# INTENTIR_SPEC
## Purpose
- define the canonical product boundary for `specforge`
- specify the staged IR pipeline clearly enough that future implementation work does not drift back toward backend-specific thinking
- make it explicit that `IntentIR`, not `.fsm`, is the canonical deliverable of the tool

## Canonical product boundary
`specforge` exists to extract implementation-relevant intent from specifications.

The canonical output of the system is:
- `IntentIR`

`IntentIR` must be:
- target-independent
- serializable
- versioned
- precise enough to drive backend adapters
- explicit about uncertainty, abstractions, and residual decisions

Backend-specific outputs are not the primary product. They are adapters:
- `.fsm`
- SystemVerilog
- Verilog
- VHDL

## Why IntentIR instead of “AST”
`AST` is too syntactic for the actual problem.

The end state must capture:
- semantics
- behavioral intent
- protocol obligations
- timing constraints
- ordering rules
- actor boundaries
- environment assumptions
- abstractions
- unresolved decisions

That is richer than a syntax tree. The correct term is `IntentIR`.

## Stage model
The staged pipeline is explicit and ordered:
- `SourceIR`
- `EvidenceIR`
- `SemanticIR`
- `IntentIR`
- adapters

Each stage has a distinct responsibility.

## SOTA source-understanding stance
For PDFs and other visually rich sources, `specforge` should not behave like a plain text extractor.

The intended architecture is:
- structured parser first
- provenance-preserving page and asset capture second
- selective multimodal enrichment for figures, charts, diagrams, and image-heavy regions third
- semantic lifting only after the evidence is grounded

This means:
- markdown is a useful normalized view, but not the only source of truth
- page images, figure crops, captions, and backend-native structured outputs matter
- vision-language models should enrich targeted visual regions, not replace provenance-preserving parsing entirely
- backend choices should remain pluggable so the tool can track the state of the art over time

### SourceIR
`SourceIR` captures normalized source identity and ingest intent.

It is responsible for:
- source registration
- canonical paths
- source kind detection
- normalization planning
- promoted artifact layout
- parser backend identity
- page-artifact materialization
- visual-asset materialization
- backend raw output capture
- caption and visual-source linkage
- initial automation confidence
- source-side residual decisions
- explicit handoff toward `EvidenceIR`

It is not responsible for:
- statement extraction
- high-level semantic interpretation of figures
- actor inference
- semantic interpretation
- backend lowering

Minimal conceptual example:
```json
{
  "schema_version": 1,
  "stage": "source_ir",
  "artifact_layout": {
    "normalized_root": "generated/source_ir/axi_core/normalized",
    "page_artifact_manifest_path": "generated/source_ir/axi_core/normalized/page_artifacts.json",
    "visual_asset_manifest_path": "generated/source_ir/axi_core/normalized/visual_assets.json"
  },
  "document_identity": {
    "document_key": "axi_core",
    "display_name": "axi_core.md",
    "origin_kind": "markdown"
  },
  "normalization_plan": {
    "strategy": "existing_markdown",
    "backend": "direct_markdown",
    "status": "ready"
  },
  "page_artifacts": [],
  "visual_assets": [],
  "downstream_stages": [
    "evidence_ir",
    "semantic_ir",
    "intent_ir"
  ],
  "adapter_targets": [
    "fsm",
    "system_verilog",
    "verilog",
    "vhdl"
  ]
}
```

### EvidenceIR
`EvidenceIR` captures what the source explicitly says and where it says it.

It is responsible for:
- section anchors
- evidence spans
- multimodal evidence items
- figure and caption linkage
- OCR-over-image and picture-description evidence capture
- extracted statements
- provenance
- statement classification into:
  - source facts
  - derived rules
  - local design decisions
  - explicit abstractions
- visual evidence weighting into:
  - normative
  - explanatory
  - illustrative
  - ambiguous

It should avoid jumping too early into actor decomposition or backend logic.

Current implementation note:
- `specforge evidence <source-ir>` now materializes `generated/evidence_ir/<document_key>/evidence_ir.json`
- the first executable pass consumes ready `SourceIR` artifacts plus promoted markdown
- it currently builds section anchors, block-level evidence spans, visual evidence items from `SourceIR.visual_assets`, caption/figure-reference links, and heuristic statement classes
- deeper OCR-over-image, chart extraction, and formula/table transcription remain future enrichment work

Minimal conceptual example:
```json
{
  "schema_version": 1,
  "stage": "evidence_ir",
  "source_ir_path": "generated/source_ir/axi_core/source_ir.json",
  "section_anchors": [
    {
      "section_id": "transport_rules",
      "title": "Valid/Ready Transport",
      "source_path": "normalized/axi_core.md",
      "page_start": 7,
      "page_end": 8,
      "line_start": 760,
      "line_end": 1045
    }
  ],
  "evidence_spans": [
    {
      "span_id": "span_0001",
      "modality": "text",
      "source_path": "normalized/axi_core.md",
      "source_page": 7,
      "line_start": 812,
      "line_end": 818,
      "visual_asset_id": null
    }
  ],
  "visual_evidence": [
    {
      "evidence_id": "fig_valid_ready_timing",
      "asset_id": "figure_0007",
      "asset_kind": "diagram",
      "role": "normative",
      "source_page": 8,
      "caption_text": "Figure 4: VALID/READY timing behavior.",
      "figure_reference_text": "see Figure 4",
      "observations": [
        {
          "observation_id": "obs_0001",
          "kind": "caption",
          "created_by": "source_document",
          "text": "VALID/READY timing behavior",
          "supporting_span_ids": ["span_0001"],
          "automation_confidence": "high"
        }
      ],
      "automation_confidence": "high"
    }
  ],
  "extracted_statements": [
    {
      "statement_id": "valid_holds_until_handshake",
      "class": "source_fact",
      "modality": "mixed",
      "text": "Once VALID is asserted it must remain asserted until handshake.",
      "evidence_span_ids": ["span_0001"],
      "related_visual_evidence_ids": ["fig_valid_ready_timing"]
    }
  ]
}
```

### SemanticIR
`SemanticIR` captures a backend-neutral semantic model derived from the evidence.

It is responsible for:
- actors
- interfaces
- phases
- invariants
- contracts
- gates
- assertions
- abstractions
- decomposition candidates
- semantic residual decisions

Current implementation note:
- `specforge semantic <evidence-ir>` now materializes `generated/semantic_ir/<document_key>/semantic_ir.json`
- the first executable pass consumes persisted `EvidenceIR` JSON
- it currently discovers actors, interfaces, typed signal records, backend-neutral system/init records, backend-neutral guarded/action control fragments, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions from deterministic heuristics over evidence statements and visual grounding
- the current output is intentionally conservative and inspectable; canonical semantic normalization still continues in the later `IntentIR` stage

Minimal conceptual example:
```json
{
  "schema_version": 1,
  "stage": "semantic_ir",
  "evidence_ir_path": "generated/evidence_ir/axi_core/evidence_ir.json",
  "actors": [
    {
      "actor_id": "valid_ready_channel",
      "role_summary": "generic transport actor for VALID/READY semantics"
    }
  ],
  "invariants": [
    {
      "invariant_id": "valid_holds_until_handshake",
      "statement": "VALID stays asserted until acceptance occurs."
    }
  ]
}
```

### IntentIR
`IntentIR` is the canonical endpoint.

It is responsible for capturing the backend-independent behavioral intent precisely enough that backend adapters can lower it without inventing semantics that were not already made explicit.

It should include at least:
- intent identity
- actors and responsibilities
- canonical interface inventory
- canonical backend-neutral system contract and init assignments
- backend-neutral guarded/action control fragments
- behaviors
- constraints
- assumptions
- abstractions
- residual decisions

Current implementation note:
- `specforge intent <semantic-ir>` now materializes `generated/intent_ir/<document_key>/intent_ir.json`
- the first executable pass consumes persisted `SemanticIR` JSON
- it currently canonicalizes intent identity, actor responsibilities, interface inventory, backend-neutral system/init records, backend-neutral control fragments, behaviors, constraints, assumptions, and residual decisions from deterministic heuristics over semantic records
- the current output is intentionally conservative and inspectable; adapter work should lower from this canonical surface rather than reconstruct semantics from scratch

Minimal conceptual example:
```json
{
  "schema_version": 1,
  "stage": "intent_ir",
  "semantic_ir_path": "generated/semantic_ir/axi_core/semantic_ir.json",
  "intent_identity": {
    "intent_id": "axi_core_transport",
    "summary": "backend-neutral intent for the AXI core transport and transaction rules"
  },
  "actors": [
    {
      "actor_id": "manager_write_path",
      "responsibilities": [
        "launch write request",
        "emit write beats",
        "observe completion response"
      ]
    }
  ],
  "interfaces": [
    {
      "interface_id": "interface_write_path",
      "signal_records": [
        {
          "signal_name": "clk",
          "direction_hint": "input",
          "width_hint": 1
        },
        {
          "signal_name": "rst_n",
          "direction_hint": "input",
          "width_hint": 1
        },
        {
          "signal_name": "DATA_IN",
          "direction_hint": "input",
          "width_hint": 8
        },
        {
          "signal_name": "ACC",
          "direction_hint": "output",
          "width_hint": 8
        }
      ]
    }
  ],
  "system_contract": {
    "clock_signal": "clk",
    "reset_signal": "rst_n",
    "reset_kind": "asynchronous"
  },
  "init_assignments": [
    {
      "target_signal": "ACC",
      "value": {
        "kind": "literal",
        "literal": "8'0"
      }
    }
  ],
  "decision_tree_fragments": [
    {
      "fragment_id": "dt_fragment_accumulate",
      "block_name": "accumulate",
      "actions": [
        {
          "kind": "assign",
          "target_signal": "ACC",
          "assignment_kind": "sequential",
          "value": {
            "kind": "signal_ref",
            "signal_name": "DATA_IN"
          }
        }
      ]
    }
  ],
  "constraints": [
    {
      "constraint_id": "same_id_ordering",
      "statement": "same-ID responses preserve request order"
    }
  ]
}
```

### Adapters
Adapters lower `IntentIR` into concrete targets.

Initial planned targets:
- `.fsm`
- SystemVerilog
- Verilog
- VHDL
Current implementation note:
- `specforge adapt <intent-ir> --target fsm` now materializes `generated/adapters/fsm/<document_key>/adapter.json`
- the first executable adapter slice consumes persisted `IntentIR` JSON
- it currently selects a conservative DT-oriented root, consumes canonical interface/control/system/init records when available, emits real standalone `?dt:name` text for fully specified explicit combinational and sequential cases, and keeps broader cases blocked with residual decisions plus renderability blockers rather than fabricating `.fsm` text

Minimal conceptual adapter artifact:
```json
{
  "target": "fsm",
  "required_input_stage": "intent_ir",
  "intent_ir_path": "generated/intent_ir/axi_core/intent_ir.json",
  "lowering_status": "blocked",
  "fsm": {
    "root_name": "axi_core",
    "root_kind_decision": {
      "selected_root_kind": "dt"
    },
    "renderability": {
      "is_renderable": false
    }
  }
}
```

## Residual decision packets
Residual decisions are first-class across all stages.

They must record:
- the unresolved question
- why automation could not safely decide
- candidate interpretations
- downstream impact
- automation confidence

They exist so the system can be highly automated without hiding ambiguity.

Example:
```json
{
  "packet_id": "directory_source_ir_interpretation",
  "question": "How should this directory source be interpreted at the SourceIR stage?",
  "why_unresolved": "The directory might be a converted-document bundle or a multi-document corpus.",
  "automation_confidence": "low",
  "candidate_interpretations": [
    {
      "interpretation_id": "converted_bundle",
      "description": "Treat the directory as one normalized document bundle.",
      "downstream_impact": "Proceed toward one SourceIR artifact."
    },
    {
      "interpretation_id": "multi_document_corpus",
      "description": "Treat the directory as multiple independent specifications.",
      "downstream_impact": "Emit multiple SourceIR artifacts or require selection."
    }
  ]
}
```

## Serialization rules
- JSON is the first interchange format
- every stage artifact must carry a schema version
- stable identifiers matter more than pretty display text
- backend-neutral semantics must not be collapsed into target-specific syntax too early
- tracked markdown documentation should use repo-relative paths when referencing repository files
- PDFs should preserve structured page and visual artifacts even when markdown is also emitted

## Guardrails
- do not let `IntentIR` collapse into `.fsm` assumptions
- do not let adapter concerns leak backward into the canonical model
- do not let markdown reports become the real system of record
- do not reduce visually rich PDFs to text-only markdown when the figures, charts, or diagrams carry semantics
- do not hide abstractions or ambiguity in prose only
- do not claim full automation if residual decisions still exist

## Immediate implementation implications
- `specforge ingest` produces `SourceIR`, and for execute-mode PDF inputs it now materializes Docling-backed normalized artifacts under `generated/source_ir/<document_key>/normalized`
- `specforge evidence` constructs `EvidenceIR` from normalized markdown, page/asset manifests, and visual evidence anchors
- `specforge semantic` constructs `SemanticIR` from grounded evidence
- `specforge intent` now constructs canonical `IntentIR`
- `specforge adapt --target fsm` now constructs a typed `.fsm` adapter artifact, emits real standalone `.fsm` text for explicit canonical standalone cases, and blocks unsafe target text emission with explicit residual decisions otherwise
- adapter work should follow `IntentIR`, not precede it

## Long-term documentation requirement
`IntentIR` must eventually have:
- a stable specification
- a user guide
- schema documentation
- many worked examples
- end-to-end examples from source documents to `IntentIR`
- examples from `IntentIR` to each adapter target
