# R16-CAPTURE-FIDELITY-GATES: objective capture-fidelity metric (point #5)

## Metadata

- Tree ID: `R16-CAPTURE-FIDELITY-GATES`
- Status: `proposed`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #5, **pulled to order 3** — the
  program's objective function)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Make "how well did we capture intent?" an objective, gating number,
because per the program thesis the hard problem (prose + timing-diagram →
typed KG) cannot be improved if it cannot be measured. Two gates over
ContractIR:

1. **Realizability / consistency** — automaton-product or SMT check that
   some implementation satisfies all `assume`/`guarantee` contracts and
   the handshake set is deadlock-free / latency bounds are mutually
   satisfiable. Unrealizable captured intent ⇒ strongest "capture is
   wrong" signal ⇒ high-value repair residual, never silent lowering.
2. **Figure conformance** — replay the spec's **own** example waveforms /
   sequence diagrams as traces against the mined contracts. "Figure 3-2
   satisfies the contract extracted for Figure 3-2" is near-ground-truth
   and free (the PDF ships its own test vectors).

The pair becomes the objective function that gates and steers #3/#4/#6.

## Non-Goals

- Not full formal verification of a design — a bounded realizability /
  conformance harness, not a model checker product.
- Does not itself improve extraction; it measures and gates it.

## Acceptance Criteria

- A realizability check + a figure-conformance replay run over ContractIR
  and emit a calibrated per-contract + per-document fidelity score with
  provenance; failures route to residual/repair, never fabrication.
- A corpus fidelity report (analogous to `kg-bench`, but semantic
  conformance) regression-locked.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree (proposed; expands at promotion)

- Children (sketch): `.1` metric + gate design (realizability fragment,
  trace-conformance semantics, scoring/calibration, docs-only) →
  `.2` figure-conformance replay harness → `.3` realizability/consistency
  gate + residual routing → `.4` corpus fidelity report + close

## Dependencies / Order

- Depends on `R16-CONTRACT-IR`. Pulled ahead of #3/#4/#6: it is their
  objective function and feedback driver. Gates (does not block creation
  of) `R16-MULTIMODAL-CONTRACT-FUSION` / `R16-WAVEFORM-CONTRACT-MINING` /
  `R16-CONSTRAINED-VERIFIED-EXTRACTION`.

## Decisions

- `2026-05-19`: Re-ordered from point #5 to program order 3 — measuring
  the hard problem must precede pouring effort into it; the spec's own
  figures are near-ground-truth. Created `proposed`.

## Blockers

- None (proposed; promotion after `R16-CONTRACT-IR`).

## Changelog

- `2026-05-19`: Created `proposed` as program point #5, ordered 3rd.
