# R16-WAVEFORM-CONTRACT-MINING: timing diagram → contract (point #4 — the crux)

## Metadata

- Tree ID: `R16-WAVEFORM-CONTRACT-MINING`
- Status: `proposed`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #4, order 5 — **crux extraction
  thrust**, highest ceiling / research-grade)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Timing diagrams are the densest temporal-intent source in protocol PDFs —
they *are* the timed automaton, drawn — and per the program thesis,
**automating accurate/reliable extraction from timing diagrams (with
prose) is THE hard problem**. Current VLM timing extraction is largely
defensive (junk-label guarding). Deliver:

1. **Structured waveform → partial trace**: per figure, recover signal
   lanes, edges, value spans, relative-delay annotations (e.g. "≥2
   cycles"), and causal/dependency arrows as a typed partial trace /
   scenario.
2. **Trace → generalized contract**: induce a ContractIR contract from
   one-or-more traces (specification mining from scenarios), with
   calibrated confidence.
3. **Cross-check vs prose**: agreement with the prose-derived contract
   boosts confidence; disagreement → explicit residual (never silent
   pick) — fidelity scored by `R16-CAPTURE-FIDELITY-GATES`.

## Non-Goals

- Not pixel-perfect waveform OCR for its own sake — the deliverable is a
  *contract*, with the trace as evidence/provenance.
- No fabricated generalization beyond what the trace + annotations
  license; under-determined → residual.

## Acceptance Criteria

- A figure → typed partial-trace extractor + a trace → ContractIR
  generalizer with provenance and calibrated confidence; prose
  cross-check wired to residual on disagreement.
- Measured improvement in figure-conformance fidelity
  (`R16-CAPTURE-FIDELITY-GATES`) on the real corpus vs. baseline;
  negative fixtures prove junk waveforms do not mint contracts.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree (proposed; expands at promotion)

- Children (sketch): `.1` waveform→partial-trace schema + extraction
  approach (VLM-structured, verifier-gated) → `.2` trace→contract
  generalization + confidence calibration → `.3` prose cross-check +
  disagreement→residual → `.4` corpus conformance eval + negative
  fixtures + close

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (#1) and `R16-CAPTURE-FIDELITY-GATES`
  (#5/order-3, its objective function). Feeds
  `R16-MULTIMODAL-CONTRACT-FUSION` (#3). Pairs with
  `R16-CONSTRAINED-VERIFIED-EXTRACTION` (#6) on prose.

## Decisions

- `2026-05-19`: Explicitly the program crux per user direction
  (2026-05-19): "the most difficult thing is extracting accurate and
  reliable temporal information from prose and timing diagrams." Highest
  ceiling, research-grade; created `proposed`, sequenced after the
  target shape + objective metric exist.

## Blockers

- None (proposed; promotion after `R16-CONTRACT-IR` +
  `R16-CAPTURE-FIDELITY-GATES`).

## Changelog

- `2026-05-19`: Created `proposed` as program point #4 (crux), ordered
  5th.
