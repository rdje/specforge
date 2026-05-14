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
  Children: `R6-SEMANTIC-HARDENING.1`, `R6-SEMANTIC-HARDENING.2`, `R6-SEMANTIC-HARDENING.3`, `R6-SEMANTIC-HARDENING.4`

### Batch 1: build_symbol_definitions

- ID: `R6-SEMANTIC-HARDENING.1`
  Status: `done`
  Goal: `Fix 4 missed mutants in build_symbol_definitions — declaration_order arithmetic and ||→&& conflict detection.`
  Acceptance: `4 mutants caught: +→-, +→*, ||→&& at lines 2305, 2317, 2333.`
  Verification: `cargo test -p specforge --lib — 681/681 passed`
  Commit: `8db82b8a`

### Batch 2: parse_explicit_system_reset

- ID: `R6-SEMANTIC-HARDENING.2`
  Status: `done`
  Goal: `Fix 2 missed mutants in parse_explicit_system_reset — >=→< comparison at lines 9671, 9672.`
  Acceptance: `2 mutants caught: >=→< at lines 9671, 9672.`
  Verification: `cargo test -p specforge --lib — 685/685 passed`
  Commit: `ea0020bb`

### Batch 3: tokenize_control_expression

- ID: `R6-SEMANTIC-HARDENING.3`
  Status: `done`
  Goal: `Fix 4 missed mutants in tokenize_control_expression — +→* at pair-guard, ||→&& and ==→!= at identifier lexing.`
  Acceptance: `4 mutants caught: +→* at line 5338, ||→&& at line 5371, ==→!= (underscore) at line 5371, ==→!= (apostrophe) at line 5371.`
  Verification: `cargo test -p specforge --lib — 688/688 passed`
  Commit: `3a91e4f6`

### Batch 4: VLM helper functions

- ID: `R6-SEMANTIC-HARDENING.4`
  Status: `done`
  Goal: `Fix 7 missed mutants across extract_actor_after_by (5), signal_constraint_kind_from_vlm_state (1), is_vlm_waveform_motion_state (1).`
  Acceptance: `7 mutants caught: return value replacements, arithmetic +→- +→*, delete match arm, bool→true.`
  Verification: `cargo test -p specforge --lib — 696/696 passed`
  Commit: pending

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
| `2026-05-14` | `R6-SEMANTIC-HARDENING.2` | `cargo test -p specforge --lib` | 685/685 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.3` | `cargo test -p specforge --lib` | 688/688 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.4` | `cargo test -p specforge --lib` | 696/696 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-SEMANTIC-HARDENING.1` | `8db82b8a` | 4 assertion additions + conflicting symbols test |
| `R6-SEMANTIC-HARDENING.2` | `ea0020bb` | 3 unit tests for parse_explicit_system_reset |
| `R6-SEMANTIC-HARDENING.3` | `3a91e4f6` | 4 unit tests for tokenize_control_expression |
| `R6-SEMANTIC-HARDENING.4` | pending | 8 unit tests across extract_actor_after_by, signal_constraint_kind_from_vlm_state, is_vlm_waveform_motion_state |

## Changelog

- `2026-05-14`: Created task tree. Leaf 1 (build_symbol_definitions) already completed and backfilled.
