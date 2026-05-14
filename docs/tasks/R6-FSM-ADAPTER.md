# R6-FSM-ADAPTER: `.fsm` Adapter Hardening

## Metadata

- Tree ID: `R6-FSM-ADAPTER`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Continue systematic regression-only test assertion coverage for `.fsm` adapter
fields — renderability diagnostics, provenance preservation, confidence/support
IDs, blocking reasons, residual decisions, and graph-first direction migration —
across the adapter test surface.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not add test coverage to adapter paths with zero existing test coverage.
- Do not add assertions to fields already well-tested by existing checks.
- Do not widen `.fsm` backend scope (SystemVerilog, Verilog, VHDL remain
  deferred).
- Do not add assertions that require new test fixture infrastructure.

## Acceptance Criteria

- Remaining adapter field families with zero or thin test coverage are
  inventoried and hardened.
- All adapter tests pass after every completed leaf.
- Graph-first direction migration coverage gaps are identified and closed.
- Each leaf is committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-FSM-ADAPTER`
  Status: `active`
  Goal: `Continue systematic .fsm adapter test assertion hardening.`
  Children: `R6-FSM-ADAPTER.1`

- ID: `R6-FSM-ADAPTER.1`
  Status: `pending`
  Goal: `Audit remaining adapter test coverage gaps after the completed provenance-hardening and root_kind_decision lanes.`
  Acceptance: `All remaining adapter field families inventoried. Graph-direction migration coverage, blocking-reason completeness, residual-decision diagnostics, and signal-inventory provenance gaps documented. Next executable leaf defined.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-FSM-ADAPTER.1` | `pending` | Must inventory remaining gaps before picking specific hardening targets. |

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero
  production behavior changes allowed.
- `2026-05-14`: Tree is `active` — PROVENANCE-HARDENING closed, this tree is now the PNT frontier.

## Open Questions

- None yet. The audit leaf will surface specific coverage gaps.

## Blockers

- None. The tree is proposed, not yet active.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |

## Changelog

- `2026-05-14`: Created proposed task tree with audit leaf as first frontier.
  Hundreds of prior adapter hardening slices completed before task-tree
  tracking existed — captured in LIVE_ACHIEVEMENT_STATUS. This tree owns
  the forward-looking coverage work.
