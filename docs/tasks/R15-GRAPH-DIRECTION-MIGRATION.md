# R15-GRAPH-DIRECTION-MIGRATION: Complete actor-relative graph direction migration

## Metadata

- Tree ID: `R15-GRAPH-DIRECTION-MIGRATION`
- Status: `active`
- Roadmap lane: `R15`
- Created: `2026-05-15`
- Last updated: `2026-05-15`
- Owner: repo-local workflow

## Goal

Finish replacing the remaining flat `direction_hint` consumers with actor-relative graph semantics across the adapter, validation, and semantic stages. This is the highest-priority remaining gap per LIVE_ACHIEVEMENT_STATUS.md.

## Non-Goals

- Do not widen backend scope (no new adapter targets).
- Do not remove the `direction_hint` field — it remains as a compatibility surface.
- Do not change the canonical data model.
- Do not break existing tests or fixtures.

## Background

`SemanticIR` and `IntentIR` now carry `actor_signal_relations`, `actor_ports`, and `signal_connectivity` alongside flat `direction_hint`. The `.fsm` adapter already has a `preferred_signal_direction_hint` resolver that prefers graph direction over flat. The remaining work is ensuring every consumer path uses graph-first resolution, particularly in renderability checks (size entries, output registration, control-read direction, etc.).

## Acceptance Criteria

- All identified remaining `direction_hint` consumers are audited.
- Consumers that should use graph-first resolution are converted.
- All 1014 tests continue to pass.
- All 151 kg-bench fixtures continue to pass.
- No production behavior regressions.

## Task Tree

- ID: `R15-GRAPH-DIRECTION-MIGRATION`
  Status: `active`
  Goal: `Complete graph-first direction migration.`
  Children: `R15-GRAPH-DIRECTION-MIGRATION.1`, `R15-GRAPH-DIRECTION-MIGRATION.2`, `R15-GRAPH-DIRECTION-MIGRATION.3`

### Batch 1: Audit remaining consumers

- ID: `R15-GRAPH-DIRECTION-MIGRATION.1`
  Status: `pending`
  Goal: `Audit all production-code direction_hint consumers across the codebase to identify those not yet behind graph-first resolution.`
  Acceptance: `Inventory of all direction_hint reads outside test code, categorized by whether they use graph-first resolution or raw flat direction.`
  Files to audit: `adapters.rs`, `validate.rs`, `semantic.rs`, `kg_bench.rs`, `learn_priors.rs`

### Batch 2: Convert adapter renderability consumers

- ID: `R15-GRAPH-DIRECTION-MIGRATION.2`
  Status: `pending`
  Goal: `Convert remaining adapter renderability direction_hint consumers to use preferred_signal_direction_hint.`
  Acceptance: `All size-entry, output-registration, and control-read consumers in adapters.rs use graph-first resolution.`
  Depends on: `R15-GRAPH-DIRECTION-MIGRATION.1`

### Batch 3: Convert validation and semantic consumers

- ID: `R15-GRAPH-DIRECTION-MIGRATION.3`
  Status: `pending`
  Goal: `Convert remaining validation and semantic direction_hint consumers where appropriate.`
  Acceptance: `Validation metrics correctly report graph coverage. Semantic consumers prefer graph direction.`
  Depends on: `R15-GRAPH-DIRECTION-MIGRATION.2`

## Current Frontier

- `R15-GRAPH-DIRECTION-MIGRATION.1` — Audit remaining consumers (pending)
