# R16-CONTRACT-IR: typed timed-contract IR layer (point #1)

## Metadata

- Tree ID: `R16-CONTRACT-IR`
- Status: `active`
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

## Task Tree

- ID: `R16-CONTRACT-IR`
  Status: `active`
  Goal: typed timed-contract IR; IntentIR projects it; mechanical `.isf`
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `R16-CONTRACT-IR.1`
  Status: `pending`
  Goal: >
    Design leaf (docs-only): (a) placement decision — new IR stage vs.
    typed layer extending SemanticIR/IntentIR; (b) the closed operator
    grammar (`rose/fell`, `stable … throughout [e1,e2]`, `s ##[m:n] t`,
    `s until t`, `eventually within N`, `mutex`, `ordered_before`) with
    typed event/window operands; (c) exact `TemporalRuleRecord` →
    ContractIR migration map proving zero temporal-fidelity loss + CI
    parity strategy; (d) how `ISF-HANDSHAKE-STAGE-LOWERING` is subsumed.
  Acceptance: `Design doc recorded in this tree (placement + grammar + migration map + parity plan + subsumption); no code; reviewed against current TemporalRuleRecord/TickPhase/cycle_window/HandshakeComplete and the .isf contract/stage/bounded_eventually surface.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONTRACT-IR.2`
  Status: `pending`
  Goal: implement the typed ContractIR model + serde per the `.1` design.
  Acceptance: `Typed model + serde + unit tests; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONTRACT-IR.3`
  Status: `pending`
  Goal: migrate `TemporalRuleRecord` producers + `.isf` lowering onto
  ContractIR with CI parity (every currently-lowered rule still lowers;
  previously-shredded bound obligations now survive as one contract).
  Acceptance: `Producers/lowering on ContractIR; CI-parity proven on the real corpus; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONTRACT-IR.4`
  Status: `pending`
  Goal: close tree; sync mdBook (temporal-semantics + ISF chapters) +
  ROADMAP R16; record `ISF-HANDSHAKE-STAGE-LOWERING` disposition.
  Acceptance: `Tree done; docs synced; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-CONTRACT-IR.1` | `pending` | Design before code — placement + operator grammar + migration map + parity plan (docs-only); gates all R16 extraction |
| 2 | `R16-CONTRACT-IR.2` | `pending` | Typed model once `.1` design is fixed |
| 3 | `R16-CONTRACT-IR.3` | `pending` | Migrate producers/lowering with CI parity |
| 4 | `R16-CONTRACT-IR.4` | `pending` | Close + doc sync |

## Dependencies / Order

- DAG root (no deps). Blocks `R16-KG-PROTOCOL-ONTOLOGY`,
  `R16-CAPTURE-FIDELITY-GATES`, and transitively all extraction trees —
  nothing can extract accurately into a shape that does not exist.

## Decisions

- `2026-05-19`: Highest program leverage (per `R16-INTENT-CAPTURE`
  thesis): representation loss is currently misdiagnosed as extraction
  loss. Created `proposed`; first to be promoted.
- `2026-05-19`: **Promoted `proposed → active`** by `R16-INTENT-CAPTURE.2`
  after the extraction methodology was presented and accepted. Frontier
  is `.1` — a docs-only design leaf (no code until the placement +
  operator grammar + `TemporalRuleRecord` migration/parity map are
  recorded), honoring the no-code-without-an-agreed-design discipline.

## Blockers

- None. Active; frontier `R16-CONTRACT-IR.1` (design, docs-only).

## Changelog

- `2026-05-19`: Created `proposed` as program point #1 / DAG root.
- `2026-05-19`: Promoted to `active` (frontier `.1`, docs-only design)
  after the user accepted the extraction methodology and authorized the
  program to begin. Concrete `.1`–`.4` leaves defined.
