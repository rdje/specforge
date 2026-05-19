# R16-CONSTRAINED-VERIFIED-EXTRACTION: schema-constrained + verified extraction (point #6 — the crux, continuous)

## Metadata

- Tree ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION`
- Status: `proposed`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #6, order 6 — crux, continuous
  hardening of prose+waveform extraction precision/recall)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Make extraction high-precision by construction (the program thesis: prose
+ timing-diagram → typed KG accuracy is the crux):

1. **Schema-constrained extraction** — LLM/VLM emit **directly into the
   ContractIR schema** via grammar/JSON-schema-constrained decoding (no
   free-text post-parse).
2. **Entailment verifier** — a second pass checks the exact source span
   (sentence / table cell / figure region) actually *licenses* the
   extracted contract; failed entailment ⇒ residual, never fabricate.
   This makes the residual-honesty doctrine *enforceable*.
3. **Protocol-pattern template library** — canonical temporal templates
   (ready/valid, credit flow control, setup/access, async-assert/
   sync-release reset, burst+last) seeded into prior memory; matched
   templates instantiate at high confidence and concentrate uncertainty
   on the genuinely novel residue.
4. **Uncertainty-driven converge** — each VLM/NLP pass spends on the
   lowest-confidence, highest-impact contracts (value-of-information),
   not blanket rescans.

## Non-Goals

- Not a model/runtime swap — provider-agnostic, reuses existing
  enrich/nlp-enrich/converge infrastructure.
- The verifier may not "soften" a contract to pass; it accepts or routes
  to residual.

## Acceptance Criteria

- Constrained decoding into ContractIR + an entailment-verifier gate +
  a template library in prior memory + uncertainty-driven converge pass
  selection; precision/recall measured by
  `R16-CAPTURE-FIDELITY-GATES` on the real corpus vs. baseline.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree (proposed; expands at promotion)

- Children (sketch): `.1` constrained-decoding + verifier design →
  `.2` schema-constrained extractor into ContractIR → `.3` entailment
  verifier + residual routing → `.4` protocol template library in prior
  memory → `.5` uncertainty-driven converge selection → `.6` corpus
  precision/recall eval + close

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (#1, the schema to constrain into) and
  `R16-CAPTURE-FIDELITY-GATES` (#5/order-3, its objective function).
  Feeds `R16-MULTIMODAL-CONTRACT-FUSION` (#3). Pairs with
  `R16-WAVEFORM-CONTRACT-MINING` (#4).

## Decisions

- `2026-05-19`: Continuous precision/recall hardening on the crux;
  enforces the residual-honesty doctrine via the verifier. Created
  `proposed`, last in order (depends on schema + metric).

## Blockers

- None (proposed; promotion after `R16-CONTRACT-IR` +
  `R16-CAPTURE-FIDELITY-GATES`).

## Changelog

- `2026-05-19`: Created `proposed` as program point #6, ordered 6th.
