# 0015 — mdBook owns public product contracts; stable root documents are compatibility pointers

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

The mdBook has been SpecForge's declared public documentation product since commit `5d215c25`, which
reduced `USER_GUIDE.md` to a compatibility pointer. Later feature work appended commands and current
limitations to that pointer again. Three neighboring architecture roots also mixed durable design,
current implementation state, validation results, bugs, and delivery plans.

A claim-level audit found valuable unique extraction and `IntentIR` rules, but also April-era status
that contradicts shipped graph, temporal, VLM, command, and validation behavior. Exact paragraph
matching found no long-block copies; the duplication is semantic and time-layered.

## Decision

1. `docs/book/src/SUMMARY.md` remains the complete mandatory index for public product documentation.
2. Unique durable extraction requirements live in the book's Extraction Architecture Contract.
   Unique normative `IntentIR` boundary, residual, serialization, and guardrail rules live in the
   book's IntentIR Product Contract.
3. `USER_GUIDE.md`, `EXTRACTION_ARCHITECTURE.md`, `KNOWLEDGE_GRAPH_ARCHITECTURE.md`, and
   `INTENTIR_SPEC.md` keep their stable filenames only as concise direct compatibility pointers.
   They do not mirror current commands, implementation state, bugs, validation, or plans.
4. Each root pointer is an independently bounded snapshot whose currency uses the composed
   code/book/root verifier. They are no longer a partitioned canonical collection, so the generated
   root-architecture catalog and its configuration row are retired.
5. Obsolete root status remains retrievable through Git and owning task trees. It is historical
   implementation evidence, not current architecture and not a second maintained reference.

## Consequences

- Public readers have one maintained truth plane and direct topical routes.
- Stable external filenames continue to resolve without carrying a drift-prone duplicate manual.
- Unique product/specification prose remains complete inside directly indexed semantic book parts.
- Any later aggregate book change still needs exact fresh task-owned authority; pointer growth is
  independently capped and mechanically checked.
- Current implementation direction stays in `ROADMAP.md` and task trees; reviewed measurements stay
  in their explicit validation/corpus authorities.

## Links

- `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
- `docs/book/src/reference/extraction-architecture.md`
- `docs/book/src/reference/intentir-contract.md`
- `scripts/check_book_current_truth.sh`
- `doctrine/live_document_size/surfaces.jsonl`
