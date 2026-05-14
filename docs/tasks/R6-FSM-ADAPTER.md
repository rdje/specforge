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
  Commit: `1fb4b4e7` — Audit adapter test coverage

### Batch 2: Zero-coverage adapter fields

- ID: `R6-FSM-ADAPTER.1.1`
  Status: `done`
  Goal: `Harden AdapterArtifact identity fields — adapter_identity (adapter_id, summary) and document_identity.`
  Acceptance: `9 assertions across 2 tests: builds_blocked_dt_centric_fsm_adapter_artifact (6 assertions — adapter_id non-empty + prefix, summary non-empty + content, document_key + display_name non-empty) and builds_renderable_standalone_dt_fsm_adapter_artifact (3 assertions — adapter_id, summary, document_key non-empty).`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `9bbf01e3` — Add adapter_identity and document_identity assertions

- ID: `R6-FSM-ADAPTER.1.2`
  Status: `done`
  Goal: `Harden FsmDecisionTreeCandidate.summary — dynamic format string populated during DT candidate construction.`
  Acceptance: `2 assertions across 2 tests: builds_blocked_dt_centric_fsm_adapter_artifact and builds_renderable_standalone_dt_fsm_adapter_artifact_with_graph_direction_context. Summary is populated in both DT construction paths.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `b67de022` — Add FsmDecisionTreeCandidate.summary assertions

- ID: `R6-FSM-ADAPTER.1.3`
  Status: `done`
  Goal: `Harden FsmRenderableModule.symbol_definitions — end-to-end assertion through the adapter build pipeline.`
  Acceptance: `2 end-to-end assertions across 2 DT-root tests: builds_renderable_symbolic_dt_fsm_adapter_artifact and builds_renderable_compound_update_dt_fsm_adapter_artifact. Two other DT tests (standalone_dt, computed_selector) skip symbol_definitions — their fixtures carry no SymbolDefinitionRecords.`
  Verification: `cargo test -p specforge --lib — 666/666 passed`
  Commit: `pending — about to commit`

## Current Frontier

No executable leaves remain. All 4 R6-FSM-ADAPTER leaves (audit + 3 hardening) complete.

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
| `2026-05-14` | `R6-FSM-ADAPTER.1.1` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `R6-FSM-ADAPTER.1.2` | `cargo test -p specforge --lib` | 666/666 passed |
| `2026-05-14` | `R6-FSM-ADAPTER.1.3` | `cargo test -p specforge --lib` | 666/666 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-FSM-ADAPTER.1` | `1fb4b4e7` Audit adapter test coverage | Audit only — 3 hardening leaves defined |
| `R6-FSM-ADAPTER.1.1` | `9bbf01e3` Add adapter_identity and document_identity assertions | 9 assertions across 2 tests |
| `R6-FSM-ADAPTER.1.2` | `b67de022` Add FsmDecisionTreeCandidate.summary assertions | 2 assertions across 2 tests |
| `R6-FSM-ADAPTER.1.3` | pending — about to commit | 2 symbol_definitions end-to-end assertions |

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
