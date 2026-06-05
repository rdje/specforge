# LOGIC-LEVEL-BOUNDARY: resolve HIGH/LOW as universal "how"; centralize the vocabulary

## Metadata

- Tree ID: `LOGIC-LEVEL-BOUNDARY`
- Status: `done` (CLOSED `2026-06-05`; `.1`)
- Roadmap lane: `R16`/`R15e` (extraction architecture / ADR 0006)
- Created: `2026-06-05`
- Parent context: owner directive ("do all 5 bullets") — **item 2 of 5**, the logic-level derivation.

## Decision (the honest engineering finding)

A full "derive `HIGH`/`LOW`" — collapsing `MustBeHigh`/`MustBeLow` into the generic value path — is
**rejected** as harmful, with reasons recorded in ADR 0006:

1. **Lossy.** The high/low distinction drives correct `.isf` / SVA lowering; flattening to
   `MustBeValue{"high"}` discards it.
2. **9-file blast radius.** `MustBeHigh`/`MustBeLow` are consumed across 9 files (eval, nli_verify,
   source, semantic, evidence, intent, project_validation, nlp_enrich, validate).
3. **Ill-posed.** A spec *assumes* high = 1 and never *defines* it; you cannot derive from the
   document a fact the document does not state. Knowing high = 1 is the irreducible **"how."**

What is done instead (the defensible, non-lossy action): a logic level is recognized through a
**convention-agnostic** spelling set (`1`/`0`, `high`/`low`, `true`/`false`, `1'b1`/`1'b0`) — so no
single convention is presumed (addressing the owner's "a spec may use 1/0, H/L, asserted") — and the
*level semantic* is retained as universal "how". A spec's **own** value/state names (`NONSEQ`,
`IDLE`, custom states) stay **derived from the document** (unchanged; PDF-AGNOSTIC-EXTRACTION).

## What changed

- Centralized the logic-level spellings into `ir/normative_vocab.rs` as `LOGIC_HIGH_VALUES` /
  `LOGIC_LOW_VALUES` — the single, documented authority (marked universal "how", not names).
- Refactored the value→kind normalizer (semantic.rs) to reference them (behavior-preserving).
- Amended ADR 0006: refined the boundary to classify logic levels as universal "how" (convention-
  agnostic), distinct from a spec's own derived value names; updated the value-path consequence.

## Verification

Passed (`2026-06-05`) — behavior-preserving refactor (full lib suite unchanged at 1260; the logic-
level normalization tests still pass). Full `scripts/run_ci.sh` GREEN. **Owner CONFIRMED the ADR
refinement (`2026-06-05`)** — logic levels are universal "how", retained.

## Task Tree

- ID: `LOGIC-LEVEL-BOUNDARY` · Status: `done` · Children: `.1`
- ID: `LOGIC-LEVEL-BOUNDARY.1` · Status: `done` · Goal: decide the boundary, centralize the
  vocabulary, amend ADR 0006, keep behavior. Verification above.

## Changelog

- `2026-06-05`: Created + CLOSED — logic levels resolved as universal "how" (convention-agnostic),
  vocabulary centralized in `normative_vocab`, ADR 0006 refined. (Owner "do all 5" — item 2/5.)
