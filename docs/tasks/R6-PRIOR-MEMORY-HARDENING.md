# R6-PRIOR-MEMORY-HARDENING: Prior Memory Module Test Assertion Hardening

## Metadata

- Tree ID: `R6-PRIOR-MEMORY-HARDENING`
- Status: `active`
- Roadmap lane: `R6`
- Created: `2026-05-15`
- Last updated: `2026-05-15`
- Owner: repo-local workflow

## Goal

Fix 101 missed cargo-mutants in `crates/specforge/src/ir/prior_memory.rs` — systematically run mutation testing on prior memory module functions and close assertion gaps. The module currently has zero unit tests.

## Non-Goals

- Do not change production behavior. This is regression-only hardening.
- Do not fix low-value arithmetic mutations (+→-, +→*) that are false positives or near-equivalent.

## Acceptance Criteria

- High-value missed mutants (logic operators, comparison operators, negation) caught.
- All tests pass after every completed leaf.
- Each leaf committed with leaf-ID traceability.

## Task Tree

- ID: `R6-PRIOR-MEMORY-HARDENING`
  Status: `active`
  Goal: `Fix 101 missed cargo-mutants in prior_memory.rs functions.`
  Children: `R6-PRIOR-MEMORY-HARDENING.1` through `.N`

### Batch 1: is_word_boundary + NegativeKnowledgeKind::as_str + key functions

- ID: `R6-PRIOR-MEMORY-HARDENING.1`
  Status: `done`
  Goal: `Fix 12 missed mutants across is_word_boundary (8), NegativeKnowledgeKind::as_str (2), table_kind_key (2), diagram_kind_key (2).`
  Acceptance: `14 mutants caught: return values, ||→&&, ==→!=, >=→<, &&→||, delete !, !=→==.`
  Verification: `cargo test -p specforge --lib — 855/855 passed`
  Commit: `29ba572b`

### Batch 2: replace_term_with_placeholder

- ID: `R6-PRIOR-MEMORY-HARDENING.2`
  Status: `done`
  Goal: `Fix 15 missed mutants in replace_term_with_placeholder — return values, <→==/</>, &&→|| x4, >=→<, +→*/- x2, +=→-=/*= x4.`
  Acceptance: `11 caught, 3 timeouts (+=→-=/*= cause infinite loops), 1 missed (+→* low-value arithmetic deferred).`
  Verification: `cargo test -p specforge --lib — 863/863 passed`
  Commit: `48446f46`

### Batch 3: is_meaningful_actor_term

- ID: `R6-PRIOR-MEMORY-HARDENING.3`
  Status: `done`
  Goal: `Fix 8 missed mutants in is_meaningful_actor_term — all ||→&& across 8 stop-word checks.`
  Acceptance: `10 mutants caught: 8 ||→&& + 2 already covered.`
  Verification: `cargo test -p specforge --lib — 873/873 passed`
  Commit: `719ba634`

### Batch 4: normalized_text_contains_term + is_meaningful_prior_phrase + normalize_table_header_cell

- ID: `R6-PRIOR-MEMORY-HARDENING.4`
  Status: `done`
  Goal: `Fix 12 missed mutants across normalized_text_contains_term (2 ||→&&), is_meaningful_prior_phrase (9), normalize_table_header_cell (1 ==→!=).`
  Acceptance: `34 mutants tested, all 34 caught (includes 22 previously covered + 12 newly covered).`
  Verification: `cargo test -p specforge --lib — 888/888 passed`
  Commit: `e4fa8248`

### Batch 5: semantic_modality_reliability_prior_bonus

- ID: `R6-PRIOR-MEMORY-HARDENING.5`
  Status: `done`
  Goal: `Fix 4 missed mutants in semantic_modality_reliability_prior_bonus — &&→|| x2, >=→<, delete !.`
  Acceptance: `7 mutants tested, all 7 caught (includes 3 previously covered + 4 newly covered).`
  Verification: `cargo test -p specforge --lib — 893/893 passed`
  Commit: `a0fbd254`

### Batch 6: CorpusMemory filter methods (all 6 for_ query methods)

- ID: `R6-PRIOR-MEMORY-HARDENING.6`
  Status: `done`
  Goal: `Fix 29 missed mutants across semantic_phrase_priors_for (6), actor_taxonomy_priors_for (4), temporal_phrase_priors_for (7), table_shape_priors_for (4), visual_motif_priors_for (4), negative_knowledge_priors_for (4).`
  Acceptance: `All 35 mutants caught (29 caught + 6 unviable). Key catches: vec![], ==→!=, &&→||, delete !, ||→&&.`
  Verification: `cargo test -p specforge --lib — 909/909 passed`
  Commit: `273bad09`

### Batch 7: CorpusMemory search/resolve methods

- ID: `R6-PRIOR-MEMORY-HARDENING.7`
  Status: `in_progress`
  Goal: `Fix remaining mutants across temporal_cycle_window_in_text (5), semantic_modality_reliability_bonus (1), actor_taxonomy_role_for_term (2), diagram_kind_for_visual_caption (2), table_kind_for_structured_table (3), resolve_actor_taxonomy_role (3), resolve_semantic_phrase_role (3).`
  Acceptance: `All remaining CorpusMemory method mutants caught.`
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-PRIOR-MEMORY-HARDENING.7` | `in_progress` | CorpusMemory search/resolve methods |

## Decisions

- `2026-05-15`: Prioritize logic/operator mutants over arithmetic noise.
- `2026-05-15`: Group private helper functions in early batches (no fixtures needed), CorpusMemory methods in later batches (need record construction).

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- | --- |

## Changelog

- `2026-05-15`: Created task tree. 101 missed mutants across 23 functions in prior_memory.rs.
