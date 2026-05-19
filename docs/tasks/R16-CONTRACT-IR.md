# R16-CONTRACT-IR: typed timed-contract IR layer (point #1)

## Metadata

- Tree ID: `R16-CONTRACT-IR`
- Status: `proposed`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #1, order 1 — DAG root)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Replace the predicate-bag temporal model (`TemporalRuleRecord` =
antecedents→consequents + one optional `cycle_window` + edge) with a
typed **timed-contract IR**: per-actor `assume`/`guarantee` contracts
over boundary signals, expressed in a small, closed operator fragment —
`rose/fell(sig)`, `stable(sig) throughout [e1,e2]`, `s ##[m:n] t`,
`s until t`, `eventually(s) within N`, `mutex(a,b)`,
`ordered_before(phaseA,phaseB)` — so a single bound obligation is **one
object with full provenance**, not shredded across disjoint predicates.
IntentIR becomes a typed projection of ContractIR; `.isf` lowering
becomes near-mechanical (FSMGen already ships `bounded_eventually`,
ready/valid `(stage …)`).

## Non-Goals

- Do not change extraction sources in this tree (fusion/waveform/LLM are
  points #3/#4/#6). This tree delivers the **target shape only**.
- Do not break the residual-honesty doctrine: unrepresentable temporal
  intent stays an explicit residual.
- Do not regress `ISF-TEMPORAL-LOWERING`'s shipped windowed/`(rule …)`/
  residual behavior; subsume `ISF-HANDSHAKE-STAGE-LOWERING` cleanly.

## Acceptance Criteria

- A typed ContractIR exists (stage vs. typed layer decided in `.1`);
  `TemporalRuleRecord` either migrates onto it or is expressed by it
  with no temporal-fidelity loss.
- Round-trip: every currently-lowered temporal rule still lowers (CI
  parity), plus previously-shredded bound obligations now survive as one
  contract.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree (proposed; expands at promotion)

- ID: `R16-CONTRACT-IR`
  Status: `proposed`
  Goal: typed timed-contract IR; IntentIR projects it; mechanical `.isf`
  Children (sketch): `.1` design (placement + operator grammar +
  migration plan, docs-only) → `.2` typed model + serde → `.3` migrate
  `TemporalRuleRecord` producers/lowering onto it (CI parity) → `.4`
  close + book sync

## Dependencies / Order

- DAG root (no deps). Blocks `R16-KG-PROTOCOL-ONTOLOGY`,
  `R16-CAPTURE-FIDELITY-GATES`, and transitively all extraction trees —
  nothing can extract accurately into a shape that does not exist.

## Decisions

- `2026-05-19`: Highest program leverage (per `R16-INTENT-CAPTURE`
  thesis): representation loss is currently misdiagnosed as extraction
  loss. Created `proposed`; first to be promoted.

## Blockers

- None (proposed; awaits promotion by `R16-INTENT-CAPTURE.2`).

## Changelog

- `2026-05-19`: Created `proposed` as program point #1 / DAG root.
