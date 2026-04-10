# Multimodal Evidence And Visual Grounding

Chip specifications are not only prose and tables.
Figures, captions, waveform diagrams, state diagrams, and page-local layout can carry normative information too.

`specforge` treats those visual surfaces as evidence that must stay grounded, typed where possible, and caveated when ambiguous.

## Where visual evidence starts

The visual path begins in `SourceIR`.

`SourceIR` can preserve visual assets with fields such as:

- asset id
- asset kind
- page id
- image path
- caption text
- caption source path
- source reference
- placeholder text
- note
- diagram kind

The `diagram_kind` classification is intentionally lightweight at ingest time.
It can identify categories such as:

- timing diagram
- state machine diagram
- block diagram
- register bitfield
- truth table
- flow chart
- unknown

This is source preservation, not final semantic truth.

## Visual evidence in `EvidenceIR`

`EvidenceIR` converts source visual assets into visual evidence items.

A visual evidence item can carry:

- evidence id
- asset id
- asset kind
- visual role
- source path
- source page
- caption text
- figure reference text
- observations
- automation confidence

It also records links between text spans and visual evidence, such as caption links or figure references.

That gives later stages a grounded trail from semantic facts back to a figure, caption, or diagram.

`EvidenceIR` can also attach a prior-memory `Classification` observation for a locally captioned visual asset whose `diagram_kind` is still `unknown`.
That path uses visual-motif priors from `CorpusMemory`, but only when the current caption normalizes to a unique learned motif.
It can adjust the visual evidence role, for example treating a recovered timing diagram as normative visual evidence, but it does not rewrite `SourceIR` or synthesize semantic facts.
Validation reports that boundary explicitly with role metrics such as `visual_evidence_normative` / `visual_evidence_ambiguous`.
If the normative role came only from a visual-motif prior and no VLM timing/state extraction is present yet, validation emits `evidence_visual_motif_corroboration_guidance` so downstream rescan planning can route the visual to targeted multimodal corroboration before any promotion.
`project-validation` turns that finding into an explicit replay sequence: local VLM `enrich_source_ir`, rebuild `EvidenceIR`, then validate the current artifact.
The generated local provider hint defaults to `auto-local`: prefer ready Ollama, then ready LM Studio, with explicit overrides available when a run should force one provider.

## VLM observations

`specforge enrich` can write structured VLM extraction into `SourceIR.visual_assets[*].note`.

Today the important note prefixes are:

- `vlm_timing_diagram_extraction:`
- `vlm_state_machine_extraction:`

When `EvidenceIR` is rebuilt, those notes become typed visual observations:

- `TimingDiagramExtraction`
- `StateMachineExtraction`

This keeps the model output bounded.
The VLM does not get to directly author canonical `IntentIR`.
It contributes a structured observation that later stages can parse, validate, and reject if it is too weak or noisy.

## Timing diagrams

Timing-diagram observations can produce timing-related records downstream.

The expected VLM extraction shape is conceptually:

```json
{
  "signals": [
    {
      "name": "PCLK",
      "values": [
        { "cycle": "T4", "state": "HIGH" }
      ]
    }
  ],
  "annotations": [
    "setup time of data signal during T4"
  ]
}
```

`SemanticIR` can parse those observations into timing constraints.
It can also lift grounded `signals[].values[]` tuples into signal-value constraints that feed temporal predicates, such as `XREQ` being `HIGH` at a diagram cycle.
It also filters low-value labels such as pure cycle markers or address labels when they do not carry real timing semantics.
Generic visual words such as `transfer` should not become fake signal names just because they appear in a timing diagram observation.

## State-machine diagrams

State-machine observations can produce typed state and transition records.

The expected extraction shape is conceptually:

```json
{
  "states": [
    { "name": "IDLE", "is_initial": true },
    { "name": "BUSY", "is_initial": false }
  ],
  "transitions": [
    { "from": "IDLE", "to": "BUSY", "guard": "HTRANS_NONSEQ" }
  ]
}
```

This lets state diagrams contribute structured behavior without forcing the entire diagram into untyped prose.

Guard text extracted from state-machine diagrams is treated as a bounded hypothesis.
Simple comparisons such as `PREADY = 1` can become typed guards, but generic visual prose such as `transfer` should not become a fake signal unless it is grounded as a signal elsewhere in the document.

## Captions as semantic evidence

Captions can also ground semantic roles.

For example, a caption such as:

`Figure 1: XREQ valid timing.`

can contribute role evidence for `XREQ` when the surrounding context is strong enough.

The important safety rule is that visual captions are not magic.
They become observations with supporting visual evidence ids, and those observations still flow through candidates, arbitration, and consensus before they become canonical role meaning.

## Cross-modality grounding

Visual evidence becomes especially strong when it agrees with another modality.

For example:

- a signal table says `XREQ` carries valid-like request information
- a timing figure caption also says `XREQ` is valid-like

That can produce `cross_modality` semantic grounding.

This is stronger than repeated same-modality evidence, but it is still not automatic.
If table evidence and visual evidence disagree, `specforge` should preserve the conflict and avoid inventing consensus.

## Ambiguous visual grounding

Some figure links are passive.
For example, a sentence may merely say that a block diagram is shown in a figure.

`specforge` should not treat every passive figure reference as missing semantic work.

The current rule is more conservative:

- passive visual links do not create residuals by themselves
- ambiguous visual grounding only survives when ambiguous visual evidence actually contributes carried semantic observations
- unclear visual evidence should stay explicit instead of being silently promoted

## What validation can report

Validation exposes several visual and multimodal surfaces, including:

- visual asset counts
- diagram classification coverage
- figures ready for VLM
- figures already enriched
- visual evidence counts
- visual evidence with captions
- prior-memory visual classification observations
- timing diagram extractions
- semantic hints from visual captions
- visual semantic grounding
- cross-modality semantic grounding

These metrics help distinguish:

- the document has figures, but they are not classified
- figures are classified, but not VLM-enriched
- VLM observations exist, but do not carry usable semantics
- visual semantics exist, but conflict with text/table evidence
- visual evidence genuinely strengthens a canonical role

## What users should inspect

When debugging visual behavior, inspect:

- `SourceIR.visual_assets`
- visual asset `diagram_kind`
- visual asset `note`
- `EvidenceIR.visual_evidence`
- `EvidenceIR.evidence_links`
- visual observations
- `visual_classification_observations`
- semantic observations with supporting visual evidence ids
- validation metrics for VLM readiness and visual semantic grounding

The goal is not to let images bypass the typed pipeline.
The goal is to give figures and diagrams a grounded, inspectable path into the same truthfulness machinery as tables and prose.
