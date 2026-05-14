# R6-SEMANTIC-HARDENING: Semantic Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-SEMANTIC-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-14`
- Owner: repo-local workflow

## Goal

Fix missed cargo-mutants in semantic.rs — systematically run mutation testing on semantic module functions and close assertion gaps.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not fix low-value arithmetic mutations (+ → -, + → *) that are false positives or near-equivalent.

## Acceptance Criteria

- High-value missed mutants (logic operators, comparison operators, negation) caught.
- All tests pass after every completed leaf.
- Each leaf committed through `COMMIT.md` with leaf-ID traceability.

## Task Tree

- ID: `R6-SEMANTIC-HARDENING`
  Status: `active`
  Goal: `Fix high-value missed cargo-mutants in semantic.rs functions.`
  Children: `R6-SEMANTIC-HARDENING.1`

### Batch 1: build_symbol_definitions

- ID: `R6-SEMANTIC-HARDENING.1`
  Status: `done`
  Goal: `Fix 4 missed mutants in build_symbol_definitions — declaration_order arithmetic and ||→&& conflict detection.`
  Acceptance: `4 mutants caught: +→-, +→*, ||→&& at lines 2305, 2317, 2333.`
  Verification: `cargo test -p specforge --lib — 681/681 passed`
  Commit: `8db82b8a`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| No pending leaves. Run cargo-mutants to discover next target. |

## Decisions

- `2026-05-14`: Prioritize logic/operator mutants over arithmetic noise.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.1` | `cargo test -p specforge --lib` | 681/681 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-SEMANTIC-HARDENING.1` | `8db82b8a` | 4 assertion additions + conflicting symbols test |

## Changelog

- `2026-05-14`: Created task tree. Leaf 1 (build_symbol_definitions) already completed and backfilled.
