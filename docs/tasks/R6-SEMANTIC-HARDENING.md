# R6-SEMANTIC-HARDENING: Semantic Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-SEMANTIC-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-15`
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
  Children: `R6-SEMANTIC-HARDENING.1`, `R6-SEMANTIC-HARDENING.2`, `R6-SEMANTIC-HARDENING.3`, `R6-SEMANTIC-HARDENING.4`, `R6-SEMANTIC-HARDENING.5`, `R6-SEMANTIC-HARDENING.6`, `R6-SEMANTIC-HARDENING.7`, `R6-SEMANTIC-HARDENING.8`, `R6-SEMANTIC-HARDENING.9`, `R6-SEMANTIC-HARDENING.10`, `R6-SEMANTIC-HARDENING.11`, `R6-SEMANTIC-HARDENING.12`, `R6-SEMANTIC-HARDENING.13`, `R6-SEMANTIC-HARDENING.14`

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
  Commit: `34909758`

### Batch 5: VLM annotation label helpers

- ID: `R6-SEMANTIC-HARDENING.5`
  Status: `done`
  Goal: `Fix 6 missed mutants across is_signal_value_annotation_label (1), parse_allowed_vlm_observation_signal (2), normalize_vlm_waveform_motion_state (1), is_non_quantitative_waveform_motion_annotation (2).`
  Acceptance: `6 mutants caught: all ||→&& and &&→|| and delete ! across 4 functions.`
  Verification: `cargo test -p specforge --lib — 704/704 passed`
  Commit: `0b8cd737`

### Batch 6: Waveform token classifiers

- ID: `R6-SEMANTIC-HARDENING.6`
  Status: `done`
  Goal: `Fix 7 missed mutants across is_timing_annotation_constraint_token (1), is_compact_waveform_sample_label (6).`
  Acceptance: `7 mutants caught: return false, 5 &&→||, 1 delete !.`
  Verification: `cargo test -p specforge --lib — 714/714 passed`
  Commit: `4167903b`

### Batch 7: Utility functions

- ID: `R6-SEMANTIC-HARDENING.7`
  Status: `done`
  Goal: `Fix 9 missed mutants across extract_symbolic_value (2), find_ascii_case_insensitive (3), is_boilerplate_section_title (4).`
  Acceptance: `9 mutants caught: return value replacements, ||→&&, ==→!= across 3 functions.`
  Verification: `cargo test -p specforge --lib — 723/723 passed`
  Commit: `57b1b468`

### Batch 8: parse_identifier and contains_text_phrase

- ID: `R6-SEMANTIC-HARDENING.8`
  Status: `done`
  Goal: `Fix 4 missed mutants in parse_identifier (1) and contains_text_phrase (3).`
  Acceptance: `4 mutants caught: ==→!= in parse_identifier, 3 ||→&& in contains_text_phrase.`
  Verification: `cargo test -p specforge --lib — 727/727 passed`
  Commit: `8783ed70`

### Batch 9: Clock edge helpers

- ID: `R6-SEMANTIC-HARDENING.9`
  Status: `done`
  Goal: `Fix 3 missed mutants in explicit_clock_edge_from_text — delete match arm, return None, ==→!=.`
  Acceptance: `3 mutants caught across explicit_clock_edge_from_text.`
  Verification: `cargo test -p specforge --lib — 730/730 passed`
  Commit: `27eb9021`

### Batch 10: Edge/index/cycle-marker helpers

- ID: `R6-SEMANTIC-HARDENING.10`
  Status: `done`
  Goal: `Fix 3 missed mutants across contains_named_generic_edge_unit (1), parse_indexed_signal_annotation_base (2 equivalent), trailing_tokens_form_only_cycle_marker_label (2).`
  Acceptance: `3 mutants caught (2 ||→&& in parse_indexed_signal_annotation_base equivalent — downstream guards rescue).`
  Verification: `cargo test -p specforge --lib — 736/736 passed`
  Commit: `2e8b07fc`

### Batch 11: Cycle count and token phrase functions

- ID: `R6-SEMANTIC-HARDENING.11`
  Status: `done`
  Goal: `Fix 23 missed mutants across collect_known_actor_names (3), contains_token_phrase (1), parse_cardinal_cycle_count_value (10), parse_ordinal_cycle_count_value (9).`
  Acceptance: `23 mutants caught: delete match arms, return value, delete !, ||→&& across 4 functions.`
  Verification: `cargo test -p specforge --lib — 764/764 passed`
  Commit: `5421dbc6`, `b76507b8`

### Batch 12: VLM guard clause and decision tree value

- ID: `R6-SEMANTIC-HARDENING.12`
  Status: `done`
  Goal: `Fix 5 missed mutants across vlm_guard_clause_has_comparison (4) and parse_vlm_decision_tree_value (1).`
  Acceptance: `5 of 6 mutants caught: return true, return false, 2 ||→&&, 1 ||→&& in decision tree. 1 ||→&& equivalent (contains('=') rescues).`
  Verification: `cargo test -p specforge --lib — 771/771 passed`
  Commit: `b76507b8`

### Batch 13: is_false, is_zero, is_waveform_index_token

- ID: `R6-SEMANTIC-HARDENING.13`
  Status: `done`
  Goal: `Fix 7 missed mutants across is_false (1), is_zero (3), is_waveform_index_token (3).`
  Acceptance: `7 mutants caught: replace with false, return true, return false, ==→!=, &&→||, delete !.`
  Verification: `cargo test -p specforge --lib — 779/779 passed`
  Commit: `dc2e7454`

### Batch 14: is_compact_waveform_index_label

- ID: `R6-SEMANTIC-HARDENING.14`
  Status: `done`
  Goal: `Fix 1 missed mutant in is_compact_waveform_index_label — delete ! at line 10015.`
  Acceptance: `1 mutant caught: delete !. 2 ||→&& equivalent (downstream parse_identifier/is_waveform_index_token rescue).`
  Verification: `cargo test -p specforge --lib — 788/788 passed`
  Commit: `b7223fb1`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-SEMANTIC-HARDENING.13` | `pending` | Run cargo-mutants to discover next untested functions. |

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
| `2026-05-14` | `R6-SEMANTIC-HARDENING.5` | `cargo test -p specforge --lib` | 704/704 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.6` | `cargo test -p specforge --lib` | 714/714 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.7` | `cargo test -p specforge --lib` | 723/723 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.8` | `cargo test -p specforge --lib` | 727/727 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.9` | `cargo test -p specforge --lib` | 730/730 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.10` | `cargo test -p specforge --lib` | 736/736 passed |
| `2026-05-14` | `R6-SEMANTIC-HARDENING.11` | `cargo test -p specforge --lib` | 764/764 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.12` | `cargo test -p specforge --lib` | 771/771 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.13` | `cargo test -p specforge --lib` | 779/779 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.14` | `cargo test -p specforge --lib` | 788/788 passed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |
| `R6-SEMANTIC-HARDENING.1` | `8db82b8a` | 4 assertion additions + conflicting symbols test |
| `R6-SEMANTIC-HARDENING.2` | `ea0020bb` | 3 unit tests for parse_explicit_system_reset |
| `R6-SEMANTIC-HARDENING.3` | `3a91e4f6` | 4 unit tests for tokenize_control_expression |
| `R6-SEMANTIC-HARDENING.4` | `34909758` | 8 unit tests across extract_actor_after_by, signal_constraint_kind_from_vlm_state, is_vlm_waveform_motion_state |
| `R6-SEMANTIC-HARDENING.5` | `0b8cd737` | 8 unit tests across is_signal_value_annotation_label, parse_allowed_vlm_observation_signal, normalize_vlm_waveform_motion_state, is_non_quantitative_waveform_motion_annotation |
| `R6-SEMANTIC-HARDENING.6` | `4167903b` | 10 unit tests for is_timing_annotation_constraint_token, is_compact_waveform_sample_label |
| `R6-SEMANTIC-HARDENING.7` | `57b1b468` | 9 unit tests across extract_symbolic_value, find_ascii_case_insensitive, is_boilerplate_section_title |
| `R6-SEMANTIC-HARDENING.8` | `8783ed70` | 5 unit tests across parse_identifier, contains_text_phrase |
| `R6-SEMANTIC-HARDENING.9` | `27eb9021` | 5 unit tests for explicit_clock_edge_from_text |
| `R6-SEMANTIC-HARDENING.10` | `2e8b07fc` | 9 unit tests across contains_named_generic_edge_unit, parse_indexed_signal_annotation_base, trailing_tokens_form_only_cycle_marker_label |
| `R6-SEMANTIC-HARDENING.11` | `5421dbc6`, `b76507b8` | 20+ unit tests across collect_known_actor_names, contains_token_phrase, parse_cardinal_cycle_count_value, parse_ordinal_cycle_count_value |
| `R6-SEMANTIC-HARDENING.12` | `b76507b8` | 7 unit tests for vlm_guard_clause_has_comparison, parse_vlm_decision_tree_value |
| `R6-SEMANTIC-HARDENING.13` | `dc2e7454` | 8 unit tests for is_false, is_zero, is_waveform_index_token |
| `R6-SEMANTIC-HARDENING.14` | `b7223fb1` | 9 unit tests for is_compact_waveform_index_label |

## Changelog

- `2026-05-14`: Created task tree. Leaf 1 (build_symbol_definitions) already completed and backfilled.
- `2026-05-14`: Completed leaves 2-10 (parse_explicit_system_reset through edge/index/cycle-marker helpers).
- `2026-05-15`: Completed leaves 11-12 (cycle count functions, VLM guard/decision tree).
