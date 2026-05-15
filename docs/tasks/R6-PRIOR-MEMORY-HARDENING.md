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
  Commit: pending

### Batch 3: is_meaningful_actor_term

- ID: `R6-PRIOR-MEMORY-HARDENING.3`
  Status: `pending`
  Goal: `Fix 8 missed mutants in is_meaningful_actor_term — all ||→&& across 8 stop-word checks.`
  Acceptance: `8 mutants caught: each ||→&& in the chain tested.`
  Verification: pending
  Commit: pending

### Batch 4: normalized_text_contains_term + is_meaningful_prior_phrase + normalize_table_header_cell

- ID: `R6-PRIOR-MEMORY-HARDENING.4`
  Status: `pending`
  Goal: `Fix 12 missed mutants across normalized_text_contains_term (2 ||→&&), is_meaningful_prior_phrase (9), normalize_table_header_cell (1 ==→!=).`
  Acceptance: `12 mutants caught.`
  Verification: pending
  Commit: pending

### Batch 5: semantic_modality_reliability_prior_bonus

- ID: `R6-PRIOR-MEMORY-HARDENING.5`
  Status: `pending`
  Goal: `Fix 4 missed mutants in semantic_modality_reliability_prior_bonus — &&→|| x2, >=→<, delete !.`
  Acceptance: `4 mutants caught.`
  Verification: pending
  Commit: pending

### Batch 6: CorpusMemory filter methods (semantic_phrase, actor_taxonomy, temporal_phrase)

- ID: `R6-PRIOR-MEMORY-HARDENING.6`
  Status: `pending`
  Goal: `Fix 17 missed mutants across semantic_phrase_priors_for (6), actor_taxonomy_priors_for (4), temporal_phrase_priors_for (7).`
  Acceptance: `17 mutants caught: vec![], ==→!=, &&→||, ||→&&, delete ! across 3 filter methods.`
  Verification: pending
  Commit: pending

### Batch 7: CorpusMemory filter methods (table_shape, visual_motif, negative_knowledge)

- ID: `R6-PRIOR-MEMORY-HARDENING.7`
  Status: `pending`
  Goal: `Fix 12 missed mutants across table_shape_priors_for (4), visual_motif_priors_for (4), negative_knowledge_priors_for (4).`
  Acceptance: `12 mutants caught: vec![], ==→!=, &&→|| across 3 filter methods.`
  Verification: pending
  Commit: pending

### Batch 8: temporal_cycle_window_in_text + semantic_modality_reliability_bonus + actor_taxonomy_role_for_term

- ID: `R6-PRIOR-MEMORY-HARDENING.8`
  Status: `pending`
  Goal: `Fix 8 missed mutants across temporal_cycle_window_in_text (5), semantic_modality_reliability_bonus (1), actor_taxonomy_role_for_term (2).`
  Acceptance: `8 mutants caught: ==→!=, >→==/< / >=, delete !.`
  Verification: pending
  Commit: pending

### Batch 9: diagram_kind_for_visual_caption + table_kind_for_structured_table

- ID: `R6-PRIOR-MEMORY-HARDENING.9`
  Status: `pending`
  Goal: `Fix 5 missed mutants across diagram_kind_for_visual_caption (2), table_kind_for_structured_table (3).`
  Acceptance: `5 mutants caught: >→==, >→<, >→>=.`
  Verification: pending
  Commit: pending

### Batch 10: resolve_actor_taxonomy_role + resolve_semantic_phrase_role

- ID: `R6-PRIOR-MEMORY-HARDENING.10`
  Status: `pending`
  Goal: `Fix 6 missed mutants across resolve_actor_taxonomy_role (3), resolve_semantic_phrase_role (3).`
  Acceptance: `6 mutants caught: >→==, >→<, >→>=.`
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R6-PRIOR-MEMORY-HARDENING.1` | `pending` | Simple helpers — fastest to start, no CorpusMemory fixture needed |
| 2 | `R6-PRIOR-MEMORY-HARDENING.2` | `pending` | Private helper — still no CorpusMemory fixture |
| 3 | `R6-PRIOR-MEMORY-HARDENING.3` | `pending` | Private helper — ||→&& chain |
| 4 | `R6-PRIOR-MEMORY-HARDENING.4` | `pending` | Private helpers — predicate functions |
| 5 | `R6-PRIOR-MEMORY-HARDENING.5` | `pending` | Private helper — bonus calculation |
| 6 | `R6-PRIOR-MEMORY-HARDENING.6` | `pending` | CorpusMemory filter methods — need fixture setup |
| 7 | `R6-PRIOR-MEMORY-HARDENING.7` | `pending` | More CorpusMemory filter methods |
| 8 | `R6-PRIOR-MEMORY-HARDENING.8` | `pending` | CorpusMemory search methods |
| 9 | `R6-PRIOR-MEMORY-HARDENING.9` | `pending` | CorpusMemory kind resolution methods |
| 10 | `R6-PRIOR-MEMORY-HARDENING.10` | `pending` | CorpusMemory role resolution methods |

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
