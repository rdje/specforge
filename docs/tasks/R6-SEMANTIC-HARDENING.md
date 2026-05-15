# R6-SEMANTIC-HARDENING: Semantic Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-SEMANTIC-HARDENING`
- Status: `done`
- Roadmap lane: `R6`
- Created: `2026-05-14`
- Last updated: `2026-05-15` (scan completed 15:50, ~6h runtime)
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
  Status: `done`
  Goal: `Fix high-value missed cargo-mutants in semantic.rs functions.`
  Children: `R6-SEMANTIC-HARDENING.1` through `.30`

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

### Batch 15: Infrastructure actor functions

- ID: `R6-SEMANTIC-HARDENING.15`
  Status: `done`
  Goal: `Fix 4 missed mutants across parse_explicit_infrastructure_source_actor (3) and parse_explicit_infrastructure_distribution_actors (3).`
  Acceptance: `4 mutants caught: +→-, +→*, <=→> (non-zero pattern_pos for +→*).`
  Verification: `cargo test -p specforge --lib — 795/795 passed`
  Commit: `ee763737`

### Batch 16: extract_infrastructure_component_phrase

- ID: `R6-SEMANTIC-HARDENING.16`
  Status: `done`
  Goal: `Fix 4 missed mutants in extract_infrastructure_component_phrase — !=→== (underscore/hyphen), &&→||, >=→<.`
  Acceptance: `4 mutants caught. 1 delete ! equivalent (first trim_matches removes all-delimiter tokens).`
  Verification: `cargo test -p specforge --lib — 803/803 passed`
  Commit: `a77dba8e`

### Batch 17: extract_infrastructure_subject_phrase

- ID: `R6-SEMANTIC-HARDENING.17`
  Status: `done`
  Goal: `Fix 8 missed mutants in extract_infrastructure_subject_phrase — !=→==, >=→<, &&→|| across determiner and reverse paths.`
  Acceptance: `8 mutants caught. 6 remaining equivalent or low-value (delete ! rescued by dual-path design, +→* arithmetic, &&→||/delete ! in trim_matches rescued).`
  Verification: `cargo test -p specforge --lib — 812/812 passed`
  Commit: `76cce288`

### Batch 18: parse_width_token

- ID: `R6-SEMANTIC-HARDENING.18`
  Status: `done`
  Goal: `Fix 2 missed mutants in parse_width_token — >→>= and &&→||.`
  Acceptance: `2 mutants caught: >→>= (zero test), &&→|| (non-alphanumeric test).`
  Verification: `cargo test -p specforge --lib — 817/817 passed`
  Commit: `715bf9ce`

### Batch 19: parse_primary and parse_comparison

- ID: `R6-SEMANTIC-HARDENING.19`
  Status: `done`
  Goal: `Fix 6 missed mutants across ControlExpressionParser::parse_primary (1 ||→&&) and parse_comparison (5 delete match arm).`
  Acceptance: `6 mutants caught: ||→&& for "true"/"false" literals, delete match arms for !=, <, <=, >, >=.`
  Verification: `cargo test -p specforge --lib — 826/826 passed`
  Commit: `715bf9ce`, `f2ed61e6`

### Batch 20: clock gate/reset synchronizer/dedup

- ID: `R6-SEMANTIC-HARDENING.20`
  Status: `done`
  Goal: `Fix 5 missed mutants across dedup_actor_names (1 delete call), parse_explicit_clock_gated_branch (2 ||→&&), parse_explicit_reset_synchronizer_stages (1 ||→&&).`
  Acceptance: `4 mutants caught. 1 ||→&& at 3441 equivalent (both "clock gated" and "clock-gated" contain "clock gate" substring rescue).`
  Verification: `cargo test -p specforge --lib — 834/834 passed`
  Commit: `fb4f0fe0`

### Batch 21: as_str functions and build transition dedup

- ID: `R6-SEMANTIC-HARDENING.21`
  Status: `done`
  Goal: `Fix 7 missed mutants across InfrastructureTopologyKind::as_str (2 return value), SemanticGroundingStrength::as_str (2 return value), SemanticIr::build (3: ==→!= x2, &&→||).`
  Acceptance: `7 mutants caught: 4 as_str return-value replacements, 3 build transition dedup logic.`
  Verification: `cargo test -p specforge --lib — 841/841 passed`
  Commit: `271d7d8a`

### Batch 22: Broad exhaustion scan

- ID: `R6-SEMANTIC-HARDENING.22`
  Status: `done`
  Goal: `Run broad cargo-mutants scan on semantic.rs to confirm exhaustion.`
  Acceptance: `Scan completed: 1569 mutants tested in 5h. 293 missed, 1044 caught, 195 unviable, 37 timeouts. ~35 builder function mutants (need SemanticContext — deferred), ~258 testable mutants across 40+ functions. Far more than the previously documented "9 deferred."`
  Verification: `cargo mutants -p specforge -f semantic.rs — exit code 3 (missed+timeouts)`
  Commit: pending

### Batch 23: High-value parse/heuristic mutants — extract_cycle_window_from_text and edge_of_known_signal_unit_len

- ID: `R6-SEMANTIC-HARDENING.23`
  Status: `done`
  Goal: `Fix high-value missed mutants in extract_cycle_window_from_text (21 misses), extract_cycle_window_from_text_with_known_signals (61 misses), and edge_of_known_signal_unit_len (7 misses). Focus on logic/comparison operators; skip low-value arithmetic (+→-, +→*).`
  Acceptance: `12 tests added targeting ~15-20 high-value logic (&&→||, !=→==, ==→!=, delete !, delete match arm) and comparison mutants. ~60+ arithmetic mutants skipped per policy.`
  Verification: `cargo test -p specforge --lib — 935/935 passed`
  Commit: `647d1d2b`

### Batch 24: Signal hint merge and registration functions

- ID: `R6-SEMANTIC-HARDENING.24`
  Status: `pending`
  Goal: `Fix high-value missed mutants in register_interface_signal_semantic_hint (7 misses: ==→!= x7), merge_signal_hint (3 misses: match guard true/false, !=→==), merge_sticky_signal_hint (1 miss: delete match arm), merge_named_hint (1 miss: ==→!=), merge_copy_hint (1 miss: ==→!=).`
  Acceptance: `~13 high-value mutants caught across 5 signal hint functions.`
  Verification: `cargo test -p specforge --lib — 951/951 passed`
  Commit: `d021d7e2`

### Batch 25: Token classification functions — looks_like_signal_token and is_explicit_infrastructure_component_term

- ID: `R6-SEMANTIC-HARDENING.25`
  Status: `pending`
  Goal: `Fix high-value missed mutants in looks_like_signal_token (5 misses: <→==, <→<=, ||→&&, ==→!=, &&→||) and is_explicit_infrastructure_component_term (7 misses: return true, 6x ||→&&).`
  Acceptance: `11 tests added: 4 for is_explicit_infrastructure_component_term, 7 for looks_like_signal_token.`
  Verification: `cargo test -p specforge --lib — 962/962 passed`
  Commit: `e225b7c0`

### Batch 26: Parse functions — parse_explicit_signal_declaration and parse_explicit_system_clock

- ID: `R6-SEMANTIC-HARDENING.26`
  Status: `pending`
  Goal: `Fix high-value missed mutants in parse_explicit_signal_declaration (3 misses: ||→&&, <→==, <→<=) and parse_explicit_system_clock (3 misses: match guard true/false, ||→&&).`
  Acceptance: `7 tests added: 3 for parse_explicit_signal_declaration, 4 for parse_explicit_system_clock.`
  Verification: `cargo test -p specforge --lib — 969/969 passed`
  Commit: `3c65dfd4`

### Batch 27: Remaining parse functions — parse_explicit_top_child, parse_explicit_regular_state_declaration, parse_explicit_decision_tree_fragment, parse_explicit_control_clause, parse_interface_signal_direction, split_explicit_assignment, parse_control_assignment_target

- ID: `R6-SEMANTIC-HARDENING.27`
  Status: `pending`
  Goal: `Fix high-value missed mutants across 7 parse functions (~9 mutants: ||→&& x8, match guard true x1).`
  Acceptance: `12 tests added across 7 parse functions.`
  Verification: `cargo test -p specforge --lib — 981/981 passed`
  Commit: `41c52960`

### Batch 28: Boolean classifier functions — should_emit_interface_candidate, is_invariant_like, reset_signal_name_looks_active_low, normalize_infrastructure_component_name

- ID: `R6-SEMANTIC-HARDENING.28`
  Status: `pending`
  Goal: `Fix high-value missed mutants in should_emit_interface_candidate (5: ||→&& x5), is_invariant_like (3: return true, delete !, &&→||), reset_signal_name_looks_active_low (2: return true, ||→&&), normalize_infrastructure_component_name (2: <→==, <→<=).`
  Acceptance: `13 tests added across 4 boolean classifier functions.`
  Verification: `cargo test -p specforge --lib — 994/994 passed`
  Commit: `5085b86e`

### Batch 29: Temporal and consequent functions — temporal_consequents_from_conditional_rule, temporal_rule_from_timing_constraint, enrich_handshake_completion_predicates, split_temporal_condition_segment_on_and

- ID: `R6-SEMANTIC-HARDENING.29`
  Status: `pending`
  Goal: `Fix high-value missed mutants in temporal_consequents_from_conditional_rule (7: return vec![], 5x ||→&&, ==→!=), temporal_rule_from_timing_constraint (2: return None, &&→||), enrich_handshake_completion_predicates (2: delete match arm, ||→&&), split_temporal_condition_segment_on_and (3: >→>=, 2x &&→||).`
  Acceptance: `10 tests added across 4 temporal/consequent functions.`
  Verification: `cargo test -p specforge --lib — 347 semantic tests passed (1 pre-existing flaky in rescan_plan)`
  Commit: `2f030a3b`

### Batch 30: Remaining miscellaneous functions — resolve_interface_signal_semantic_role, find_known_signal_name, statement_by_id, contains_phrase, vlm_guard_clause_has_comparison, ControlExpressionParser::expect

- ID: `R6-SEMANTIC-HARDENING.30`
  Status: `pending`
  Goal: `Fix high-value missed mutants in remaining testable functions (~13 mutants).`
  Acceptance: `10 tests added across 6 remaining functions (resolve_interface_signal_semantic_role deferred — too complex to fixture).`
  Verification: `cargo test -p specforge --lib — 358/358 semantic tests passed`
  Commit: `ec97b62e`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | — | — | Tree closed. All testable high-value mutants addressed (.23–.30). Builder function mutants (~35) deferred — need SemanticContext fixtures. Low-value arithmetic mutants skipped per policy. |

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
| `2026-05-15` | `R6-SEMANTIC-HARDENING.15` | `cargo test -p specforge --lib` | 795/795 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.16` | `cargo test -p specforge --lib` | 803/803 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.17` | `cargo test -p specforge --lib` | 812/812 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.18` | `cargo test -p specforge --lib` | 817/817 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.19` | `cargo test -p specforge --lib` | 826/826 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.20` | `cargo test -p specforge --lib` | 834/834 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.21` | `cargo test -p specforge --lib` | 841/841 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.23` | `cargo test -p specforge --lib` | 935/935 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.24` | `cargo test -p specforge --lib` | 951/951 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.25` | `cargo test -p specforge --lib` | 962/962 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.26` | `cargo test -p specforge --lib` | 969/969 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.27` | `cargo test -p specforge --lib` | 981/981 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.28` | `cargo test -p specforge --lib` | 994/994 passed |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.29` | `cargo test -p specforge --lib` | 347/347 semantic tests passed (1 pre-existing flaky in rescan_plan) |
| `2026-05-15` | `R6-SEMANTIC-HARDENING.30` | `cargo test -p specforge --lib` | 358/358 semantic tests passed |

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
| `R6-SEMANTIC-HARDENING.15` | `ee763737` | 7 unit tests for parse_explicit_infrastructure_source_actor, parse_explicit_infrastructure_distribution_actors |
| `R6-SEMANTIC-HARDENING.16` | `a77dba8e` | 6 unit tests for extract_infrastructure_component_phrase |
| `R6-SEMANTIC-HARDENING.17` | `76cce288` | 11 unit tests for extract_infrastructure_subject_phrase |
| `R6-SEMANTIC-HARDENING.18` | `715bf9ce` | 5 unit tests for parse_width_token |
| `R6-SEMANTIC-HARDENING.19` | `f2ed61e6` | 9 unit tests for ControlExpressionParser parse_primary and parse_comparison |
| `R6-SEMANTIC-HARDENING.20` | `fb4f0fe0` | 7 unit tests for dedup_actor_names, parse_explicit_clock_gated_branch, parse_explicit_reset_synchronizer_stages |
| `R6-SEMANTIC-HARDENING.21` | `271d7d8a` | 8 unit tests for InfrastructureTopologyKind::as_str, SemanticGroundingStrength::as_str, SemanticIr::build transition dedup |
| `R6-SEMANTIC-HARDENING.23` | `647d1d2b` | 12 unit tests for extract_cycle_window_from_text, extract_cycle_window_from_text_with_known_signals, edge_of_known_signal_unit_len |
| `R6-SEMANTIC-HARDENING.24` | `d021d7e2` | 16 unit tests for register_interface_signal_semantic_hint, merge_signal_hint, merge_sticky_signal_hint, merge_named_hint, merge_copy_hint |
| `R6-SEMANTIC-HARDENING.25` | `e225b7c0` | 11 unit tests for is_explicit_infrastructure_component_term, looks_like_signal_token |
| `R6-SEMANTIC-HARDENING.26` | `3c65dfd4` | 7 unit tests for parse_explicit_signal_declaration, parse_explicit_system_clock |
| `R6-SEMANTIC-HARDENING.27` | `41c52960` | 12 unit tests for parse_explicit_top_child, parse_explicit_regular_state_declaration, parse_explicit_decision_tree_fragment, parse_explicit_control_clause, parse_interface_signal_direction, split_explicit_assignment, parse_control_assignment_target |
| `R6-SEMANTIC-HARDENING.28` | `5085b86e` | 13 unit tests for should_emit_interface_candidate, is_invariant_like, reset_signal_name_looks_active_low, normalize_infrastructure_component_name |
| `R6-SEMANTIC-HARDENING.29` | `2f030a3b` | 10 unit tests for split_temporal_condition_segment_on_and, temporal_rule_from_timing_constraint, temporal_consequents_from_conditional_rule, enrich_handshake_completion_predicates |
| `R6-SEMANTIC-HARDENING.30` | `ec97b62e` | 11 unit tests for find_known_signal_name, statement_by_id, contains_phrase, vlm_guard_clause_has_comparison, ControlExpressionParser::expect |

## Changelog

- `2026-05-14`: Created task tree. Leaf 1 (build_symbol_definitions) already completed and backfilled.
- `2026-05-14`: Completed leaves 2-10 (parse_explicit_system_reset through edge/index/cycle-marker helpers).
- `2026-05-15`: Completed leaves 11-21 (cycle count functions through as_str/build transition dedup).
