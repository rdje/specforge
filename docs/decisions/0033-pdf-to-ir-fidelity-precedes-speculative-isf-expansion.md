---
id: pdf-to-ir-fidelity-precedes-speculative-isf-expansion
title: PDF-to-IR semantic fidelity precedes speculative ISF expansion
date: 2026-08-11
status: accepted
scope: objective, roadmap, extraction, intentir, isf, fsmgen
evidence: ROADMAP.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md; docs/tasks/INTENT-COMPLETENESS-RESEARCH.md; docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md; docs/book/src/reference/extraction-architecture.md; docs/book/src/reference/intentir-contract.md
answers:
  - "is ISF the current SpecForge bottleneck"
  - "what is the present blocking point on the specification-to-executable-intent path"
  - "should SpecForge expand ISF before filling IntentIR from PDFs"
  - "what did the owner decide about PDF-to-IR work versus FSMGen work on 2026-08-11"
  - "when should SpecForge request a new ISF or FSMGen construct"
  - "does the upstream-first decision weaken the executable-intent objective"
---

# ADR 0033: PDF-to-IR semantic fidelity precedes speculative ISF expansion

## Context

SpecForge's objective is specification-to-executable intent, with `IntentIR` as its canonical product and
`.isf` as the sole downstream adapter target. The codebase has a real four-stage pipeline and a strict-valid
ISF path, but current source evidence shows the principal loss is upstream:

- 78 persisted documents reach each of SourceIR, EvidenceIR, SemanticIR, and IntentIR, while 44 currently emit
  `.isf`; honest non-emission proves anti-fabrication, not complete source understanding;
- one confirmed signal-catalog miss and a normalized-Markdown identifier escape affect the source-to-evidence
  boundary, with the latter present in 67 of 78 documents;
- message/structure content, figure interpretation, hard prose contracts, actor-signal relations, and register
  bit recovery have known capture, carrier, producer, or orchestration gaps before ISF is the deciding limit;
- the typed waveform consumer is deliberately dormant because no production PDF path creates `FigureRegion`;
  downstream synthetic tests therefore do not establish an operational multimodal capability.

FSMGen is actively developed and can add ISF constructs when SpecForge demonstrates a source-grounded
expressiveness gap. That makes ISF evolution available, but it does not make it the present constraint.

## Decision

1. Keep the specification-to-executable-intent objective unchanged.
2. Treat faithful population of the canonical IR pipeline from PDF text, structure, tables, figures, captions,
   and diagrams as the present program constraint.
3. Measure source-to-IntentIR completeness independently of `.isf` emission. Strict validity, renderability,
   output count, and honest residuals remain necessary evidence but cannot serve as upstream recall oracles.
4. Defer speculative ISF/FSMGen expansion. Open an ISF language request only when a source-grounded semantic:
   - is present with provenance in canonical IntentIR;
   - has passed conflict, confidence, and residual review;
   - is required by the executable-intent contract; and
   - cannot be represented faithfully by the current ISF/FSMGen contract.
5. When that trigger fires, evolve the path end to end: source evidence → typed IntentIR → typed ISF → FSMGen
   observable behavior → executable-equivalence verification.
6. Non-executable source knowledge remains typed and queryable in canonical IR. It must not be discarded, and
   it must not be forced into fabricated executable constructs.

## Consequences

- Current extraction and multimodal work can advance substantially without waiting on FSMGen.
- Existing ISF fidelity tasks remain valid evidence and future work, but they do not outrank an upstream
  capture/carrier gap merely because `.isf` is the final adapter.
- The roadmap needs content-level, per-category IR acceptance criteria and stage-loss measurements.
- A future ISF request will be smaller and better specified because it will be driven by a real IntentIR value
  and an executable oracle rather than by anticipated syntax.
- The existing upstream-first roadmap doctrine is sharpened, not reversed: semantic truth and KG quality stay
  ahead of adapter breadth.

## Links

- Task tree: [`SPEC-TO-INTENT-ALIGNMENT`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Extraction contract: [mdBook](../book/src/reference/extraction-architecture.md)
- IntentIR contract: [mdBook](../book/src/reference/intentir-contract.md)
- Existing honest-lowering precedent: [ADR 0016](0016-swd-protocol-projection-and-honest-isf-boundary.md)
