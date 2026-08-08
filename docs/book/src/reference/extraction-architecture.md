# Extraction Architecture Contract

This chapter is the durable goal contract for extracting implementation-relevant intent from chip,
protocol, component, system, and software-interface specifications. It defines what the pipeline must
preserve and recover. It does not mirror delivery status; current work belongs in the roadmap and
task trees, while shipped behavior belongs in the topical chapters of this book.

## Evidence is multimodal

A specification distributes meaning across six evidence modalities. A complete extraction design
must account for all six:

1. **Structured tables** — signal inventories, encodings, register maps, timing parameters, and
   feature/capability matrices.
2. **Normative prose** — obligations, conditions, signal-value constraints, timing language, and
   other behavioral requirements.
3. **Timing diagrams** — signal values, edges, cycles, setup/hold annotations, and handshake timing.
4. **State and flow diagrams** — states, transitions, guards, initial/final markers, and topology.
5. **Section structure** — hierarchy and local context that distinguish normative material, signal
   descriptions, appendices, glossary material, and boilerplate.
6. **Document metadata** — source identity, revision, page/asset identity, parser provenance, and
   traceability information.

No one modality is a substitute for the others. Tables are often strongest for inventory and shape;
prose is often strongest for relations and obligations; diagrams may be the most precise timing or
state evidence. The pipeline must preserve their provenance and arbitrate them explicitly.

## Stage obligations

The staged model is normative:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

- `SourceIR` is a loss-minimizing capture layer. It preserves normalized source structure, page and
  visual assets, structured tables, content order, metadata, and backend provenance.
- `EvidenceIR` records what the source says and where it says it. It may classify and extract typed
  observations, but it must keep source links and competing evidence visible.
- `SemanticIR` lifts grounded evidence into backend-neutral actors, interfaces, connectivity,
  constraints, state, timing, and other typed meaning. It must not silently re-invent evidence that
  an earlier stage discarded.
- `IntentIR` is the canonical product surface. It packages the strongest justified implementation
  intent without erasing conflicts, assumptions, or residual decisions.
- Adapters lower canonical intent; they do not author missing source semantics.

The individual [pipeline chapters](../pipeline/overview.md) define the current record surfaces and
operational behavior. [Multimodal Evidence And Visual Grounding](../pipeline/multimodal-evidence.md)
defines the current visual path, while the domain chapters define graph and temporal lifting.

## Extraction rules

- Preserve before interpreting. Markdown is useful, but it must not replace meaningful page,
  geometry, table, image, caption, or backend-native evidence.
- Prefer structured extraction over classified strings. Recognizing that a sentence is normative is
  only routing; the goal is a typed constraint with grounded subjects, values, conditions, and
  provenance.
- Keep document truth separate from cross-document experience. Priors can guide local attention or
  arbitration, but cannot become canonical facts without current-document support.
- Keep AI bounded. Models may enrich or propose; grounding, schema checks, arbitration, and
  validation decide what survives.
- Preserve uncertainty. Conflicts and unresolved choices become typed residual decision packets,
  never silent guesses.
- Remain specification-independent. Extraction logic may encode grammar and hardware concepts, but
  must not memorize one protocol's actor, signal, state, or value vocabulary.

## Target quality

A high-quality result should expose, when the source supports them:

- complete signal and field inventories with direction, width, role, and source support;
- actor-relative producer/consumer connectivity rather than perspective-free direction guesses;
- symbol/encoding, register, message-field, feature, and capability structure;
- explicit clock/reset and infrastructure semantics;
- typed state, transition, temporal, handshake, and other behavioral constraints;
- source-backed visual and cross-modality evidence;
- conflicts, coverage gaps, assumptions, and residual decisions that remain unresolved; and
- enough backend-independent structure for an adapter to lower without inventing semantics.

This target is deliberately stronger than a score or one protocol fixture. Validation and corpus
benchmarks measure progress; they do not redefine the architecture.

## Authority routes

- [Architecture Rationale](../architecture-rationale.md) explains why the design takes this shape.
- [IntentIR Product Contract](intentir-contract.md) defines the canonical output boundary.
- [Validation And Learning](../quality/validation.md) explains current quality signals.
- [Current roadmap](../../../../ROADMAP.md) and the [task-tree catalog](../../../TASK_TREE.md) own
  implementation direction and delivery state.
