---
id: extraction-and-isf-requests-advance-in-parallel
title: SpecForge extraction and FSMGen representation requests advance in parallel, on one gap ledger
date: 2026-09-25
status: accepted
scope: objective, roadmap, extraction, intentir, isf, fsmgen, requests
evidence: docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md; docs/book/src/reference/representation-gaps.md; docs/decisions/0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md; docs/upstream/fsmgen/PROCESS.md; docs/tasks/kg-isf-completeness/isf-parameter-widths.md (KG-ISF-COMPLETENESS.2a.vii.c.i-.c.iii)
answers:
  - "when may SpecForge file an FSMGen feature request for something not yet in IntentIR"
  - "does ADR 0033 still defer FSMGen requests until IntentIR carries the semantic"
  - "what did the owner decide about FSMGen feature requests on 2026-09-25"
  - "who owns a gap between what a PDF states and what the isf can say"
  - "where is the gap table between PDF extraction and isf representation"
  - "what evidence must an FSMGen feature request carry"
---

# ADR 0053: SpecForge extraction and FSMGen representation requests advance in parallel, on one gap ledger

## Context

ADR 0033 (`2026-08-11`) made PDF-to-IR fidelity the program constraint and deferred speculative ISF expansion:
item 4 opened an FSMGen request only for a semantic already present in canonical IntentIR, reviewed, required,
and unrepresentable.

On `2026-09-25`, while closing `KG-ISF-COMPLETENESS.2a.vii.c.iii`, the owner asked where each remaining gap
lives, SpecForge or FSMGen, and directed: *log as many feature requests as necessary to elegantly and
efficiently represent all the things SpecForge can extract from the PDF; log the gap table in the mdBook and
update it as FSMGen gains support; make sure SpecForge is capable of extracting any relevant data in the PDFs;
SpecForge does its part and FSMGen does its part.*

The same session had measured gaps on both sides of the boundary in one family. SpecForge was not reading
some parameter statements: a negative recommendation, a per-interface-class property table, yes/no presence
properties. FSMGen's published contract cannot state other things SpecForge already carries: a derived width,
a zero-valued or absent-by-default signal, a parameter's legal values, and a generated block that stays generic.
Waiting for every SpecForge half before asking for any FSMGen half would serialise two teams that can work
at once.

## Decision

1. **Both halves run in parallel.** An FSMGen request may be opened for a semantic that SpecForge does not yet
   carry in IntentIR, provided an open SpecForge leaf owns extracting it. This amends ADR 0033 item 4, whose
   first precondition becomes *present in canonical IntentIR **or** owned by an open SpecForge extraction
   leaf*. ADR 0033's other preconditions and its objective are unchanged: PDF-to-IR fidelity remains
   SpecForge's own constraint, and non-executable knowledge still stays typed in the IR rather than being
   forced into fabricated executable constructs.
2. **The evidence bar does not drop.** Every request still cites the specification text it serves, from a
   document in the corpus. It still reproduces FSMGen's current refusal through FSMGen's published CLI on the
   pinned and on the upstream revision, and follows `docs/upstream/fsmgen/PROCESS.md` to signoff. "Not shy"
   means one request for every real gap; it does not mean a request without a reproduced gap.
3. **One gap ledger, in the book.** `docs/book/src/reference/representation-gaps.md` lists every known gap
   between what a specification states, what SpecForge extracts, what IntentIR carries and what `.isf` can
   state, with the owner of each half. It is re-checked on every FSMGen pin refresh and whenever a leaf that
   owns a row closes. A row leaves the table only when both halves are done and verified on a chain.
4. **Completeness is measured, not asserted.** "SpecForge extracts any relevant data" is answered against
   the closed design-intent ontology (`docs/research/intent-ontology-coverage.md`), which this lane extends
   with the categories the corpus shows it lacks. Every category the census finds unextracted gets an owning
   SpecForge leaf.

## Consequences

- `EXTRACTION-REPRESENTATION-GAPS` owns the ledger, the two censuses (IntentIR to `.isf`, PDF to IR) and one
  request leaf per FSMGen-owned gap; `FSMGEN-REQUESTS` keeps owning the process and the tracking of every
  bundle once filed.
- Requests may reach FSMGen before SpecForge emits the construct. Each request's SpecForge half is named, so
  FSMGen is never waiting on an unowned counterpart.
- Forwarding stays the owner's act (`PROCESS.md` §8).

## Links

- Amends: [ADR 0033](0033-pdf-to-ir-fidelity-precedes-speculative-isf-expansion.md), item 4
- Task tree: [`EXTRACTION-REPRESENTATION-GAPS`](../tasks/EXTRACTION-REPRESENTATION-GAPS.md)
- Ledger: [mdBook](../book/src/reference/representation-gaps.md)
- Process: [`docs/upstream/fsmgen/PROCESS.md`](../upstream/fsmgen/PROCESS.md)
