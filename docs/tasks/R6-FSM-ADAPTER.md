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
  Children: `R6-FSM-ADAPTER.1`, `R6-FSM-ADAPTER.1.1`, `R6-FSM-ADAPTER.1.2`, `R6-FSM-ADAPTER.1.3`

### Batch 1: Audit

- ID: `R6-FSM-ADAPTER.1`
  Status: `done`
  Goal: `Audit remaining adapter test coverage gaps after the completed provenance-hardening and root_kind_decision lanes.`
  Acceptance: `All 34 adapter record struct fields inventoried. 4 zero-coverage fields found across AdapterArtifact (adapter_id, summary, document_identity), FsmDecisionTreeCandidate (summary), and FsmRenderableModule (symbol_definitions — end-to-end). 4 thin-coverage fields (1 assertion each). Defined 3 concrete hardening leaves.`
  Verification: `grep audit of all adapter struct fields in adapters.rs lines 351-735 cross-referenced against test module lines 6240-27515`
  Commit: `pending`

### Batch 2: Zero-coverage adapter fields

- ID: `R6-FSM-ADAPTER.1.1`
  Status: `pending`
  Goal: `Harden AdapterArtifact identity fields — adapter_identity (adapter_id, summary) and document_identity.`
  Acceptance: `Non-empty assertions on adapter_identity.adapter_id, adapter_identity.summary, and document_identity.document_key in existing adapter build tests.`
  Verification: `pending`
  Commit: `pending`

- ID: `R6-FSM-ADAPTER.1.2`
  Status: `pending`
  Goal: `Harden FsmDecisionTreeCandidate.summary — dynamic format string populated during DT candidate construction.`
  Acceptance: `Non-empty assertion on summary in at least one test that already checks other FsmDecisionTreeCandidate fields.`
  Verification: `pending`
  Commit: `pending`

- ID: `R6-FSM-ADAPTER.1.3`
  Status: `pending`
  Goal: `Harden FsmRenderableModule.symbol_definitions — end-to-end assertion through the adapter build pipeline.`
  Acceptance: `Non-empty assertion on symbol_definitions in at least one existing adapter build test.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-FSM-ADAPTER.1.1` | `pending` | Largest gap — AdapterArtifact identity fields populated every build but never asserted. |
| 2 | `R6-FSM-ADAPTER.1.2` | `pending` | FsmDecisionTreeCandidate.summary is a dynamic format string — zero coverage. |
| 3 | `R6-FSM-ADAPTER.1.3` | `pending` | FsmRenderableModule.symbol_definitions has only function-level coverage, no end-to-end assertions. |

## Decisions

- `2026-05-14`: Scoped to regression-only test assertion additions. Zero
  production behavior changes allowed.
- `2026-05-14`: Tree is `active` — PROVENANCE-HARDENING closed, this tree is now the PNT frontier.

## Open Questions

- Whether `schema_version` (hardcoded to `1`) warrants an assertion — low value, deferred.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-14` | `R6-FSM-ADAPTER.1` | grep audit of 34 adapter struct fields in adapters.rs | 4 zero-coverage fields found, 3 hardening leaves defined |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |

## Changelog

- `2026-05-14`: Created proposed task tree with audit leaf as first frontier.
  Hundreds of prior adapter hardening slices completed before task-tree
  tracking existed — captured in LIVE_ACHIEVEMENT_STATUS. This tree owns
  the forward-looking coverage work.
- `2026-05-14`: Audit complete (R6-FSM-ADAPTER.1). 34 adapter struct fields
  inventoried. 4 zero-coverage fields found: adapter_identity (adapter_id,
  summary), document_identity, FsmDecisionTreeCandidate.summary,
  FsmRenderableModule.symbol_definitions (end-to-end). Split into 3
  concrete hardening leaves (1.1, 1.2, 1.3).
