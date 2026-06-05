# TABLE-GRITS-CONFORMAL: GriTS table-structure metric + split-conformal calibration

## Metadata

- Tree ID: `TABLE-GRITS-CONFORMAL`
- Status: `done` (CLOSED `2026-06-05`; `.1`)
- Roadmap lane: `R16`/`R15e` (eval quality / calibration)
- Created: `2026-06-05`
- Parent context: owner directive ("do all 5 bullets") — **item 4 of 5** (the data-gated metrics).

## Approach

The two metrics were "gated on data" (table-structure gold; a calibration set). Rather than block,
this builds the **measurement capability** as tested library functions (the gold/calibration data is
a separate plug-in input — same pattern as the per-relation eval). SpecForge already extracts
`StructuredTableRecord` (header/body rows), so the table metric has real input to score.

## What

- `crate::eval::grits_content(gold_rows, pred_rows) -> Scorecard` — **GriTS-content (positional)**
  table-structure similarity: P/R/F1 over cells matched by identical `(row, col, normalized text)`.
  The positional-alignment variant of GriTS_con (a sound lower bound when row/col order is preserved,
  which docling does; full GriTS does optimal 2-D alignment). Reuses the existing `Scorecard`.
- `crate::eval::conformal_threshold(samples, alpha) -> Option<ConformalThreshold>` — **split-conformal
  risk-controlling threshold**: given calibration `(confidence, is_correct)` pairs and target error
  `alpha`, returns the lowest accept threshold whose accepted set satisfies the conservative
  finite-sample bound `(errors+1)/(n+1) <= alpha` (max coverage subject to the risk guarantee), or
  `None` if unattainable. Gives SpecForge a principled accept/abstain threshold for confidence-scored
  records (the NLI gate / automation-confidence path).

## Verification

Passed (`2026-06-05`) — +2 unit tests (`grits_content_scores_table_cell_matches`: 3 tp / 1 fp / 1 fn,
F1 = 0.75; `conformal_threshold_controls_risk`: picks threshold 0.7 at α=0.25 with bounded error,
returns `None` at α=0.01). Full `scripts/run_ci.sh` GREEN (1260→1262).

## Follow-up (the data input, when available)

- A small **table-structure gold** (a few APB signal tables) → run `grits_content` on the extracted
  `StructuredTableRecord`s to score table extraction.
- A **calibration set** of confidence-scored predictions labeled correct/incorrect → `conformal_threshold`
  to set the gate's accept threshold at a target risk.

## Task Tree

- ID: `TABLE-GRITS-CONFORMAL` · Status: `done` · Children: `.1`
- ID: `TABLE-GRITS-CONFORMAL.1` · Status: `done` · Goal: implement + test the two metrics as reusable
  capability. Verification above.

## Changelog

- `2026-06-05`: Created + CLOSED — GriTS-content table metric + split-conformal threshold in
  `crate::eval`, with tests. Gold/calibration data is a documented plug-in input. (Owner "do all 5" —
  item 4/5.)
