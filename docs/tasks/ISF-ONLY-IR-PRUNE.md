# ISF-ONLY-IR-PRUNE: remove genuinely `.fsm`-era orphaned IR surfaces

## Metadata

- Tree ID: `ISF-ONLY-IR-PRUNE`
- Status: `active`
- Roadmap lane: `R6` (adapter-layer cleanup after the ISF-only pivot)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Goal

Remove IR surfaces that existed only to feed the deleted `.fsm`
scheduler and now have no consumer (FSMGen does scheduling; ISF is a
higher abstraction). Candidate surfaces from the audit:
`init_assignments`, `decision_tree_fragments`, `regular_states`,
`state_transitions`.

## Picky-auditor scope correction (2026-05-18)

The user's premise that all four are "leftover from the `.fsm` adapter"
is **only true for two of them**. Verified consumers:

- `init_assignments`, `decision_tree_fragments`: consumers are
  `semantic.rs` (producer), `intent.rs` (carry-through),
  `learn_priors.rs`, `converge.rs` (snapshot) — **no kg fixtures, no
  `validate.rs` findings, no adapter**. Genuine `.fsm`-era prune
  candidates.
- `regular_states`, `state_transitions`: ALSO consumed by `validate.rs`
  (R7 state-graph findings: `initial_regular_states_count`,
  initial-state cardinality, transition reporting), **7
  `vlm_state_machine_*` gold/negative kg_quality fixtures (R15e)**,
  `learn_priors.rs` (R15f), and they are the canonical home of **VLM
  state-machine diagram extraction (R10 "Done" / R15c)**. Removing them
  would regress R10/R15c/R15e/R7 and break `kg-bench`. They are **not**
  `.fsm` leftovers.

## Non-Goals

- Do not regress R10 (visual state-machine extraction), R15c, R15e
  (the 7 `vlm_state_machine_*` fixtures), R7 (state-graph validation),
  or R15f priors.
- Do not delete `regular_states` / `state_transitions` without an
  explicit user decision that accepts those regressions or re-scopes
  them — that removal is a recorded blocked leaf, not silent work.
- Do not change extraction semantics for the surfaces that stay.

## Acceptance Criteria

- The genuinely orphaned `.fsm`-era surfaces (`init_assignments`,
  `decision_tree_fragments`, pending `.1` confirmation) are removed from
  the IR + all their producers/carriers/consumers, with `scripts/run_ci.sh`
  green and no kg-bench/fixture regression.
- `regular_states` / `state_transitions` fate is explicitly decided and
  recorded (kept as R10/R15e/R7 canonical state-graph surface, or removed
  only under an explicit user decision).
- Every leaf committed through `COMMIT.md`.

## Task Tree

- ID: `ISF-ONLY-IR-PRUNE`
  Status: `active`
  Goal: `Prune truly-dead .fsm-era IR; decide state-graph fate explicitly.`
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `ISF-ONLY-IR-PRUNE.1`
  Status: `done`
  Goal: >
    Full impact analysis of `init_assignments`,
    `decision_tree_fragments`, `regular_states`, `state_transitions`:
    enumerate every producer/carrier/consumer (semantic.rs, intent.rs,
    validate.rs, kg_bench.rs + fixtures, learn_priors.rs, converge.rs,
    book) and classify each surface as `prune-safe` or
    `load-bearing (keep / user-decision)`. Produce the exact removal plan
    + the recorded state-graph decision. Docs/analysis only.
  Acceptance: `Per-surface consumer inventory + classification + removal plan recorded; state-graph decision explicit; no code change.`
  Verification: `passed` — full repo grep inventory recorded below
    ("Impact analysis"). `init_assignments`/`decision_tree_fragments`
    confirmed prune-safe (zero `validate.rs` / `kg_bench.rs` / kg-fixture
    / adapter consumers); `regular_states`/`state_transitions` confirmed
    load-bearing (`validate.rs` 15/4, `kg_bench.rs` 9/6, 5+
    `vlm_state_machine_*` fixtures) → `.3` supersede upheld. Exact
    per-file removal plan for `.2` recorded incl. the one care point
    (converge stability-sum). No code change.
  Commit: `see Commit Log`

- ID: `ISF-ONLY-IR-PRUNE.2`
  Status: `done`
  Goal: >
    Remove the `.1`-confirmed prune-safe surfaces (expected:
    `init_assignments`, `decision_tree_fragments`) from `IntentIr`
    (+ `intent.rs` builder/carry, `semantic.rs` producers if exclusively
    feeding them, `learn_priors.rs`/`converge.rs` references) without
    regressing kg-bench or any retained surface.
  Acceptance: `Prune-safe surfaces gone; scripts/run_ci.sh green; no fixture regression. May split per surface.`
  Verification: `passed` — both surfaces removed end-to-end per the `.1`
    plan: `semantic.rs` (2 record types + 2 producer fns + 2 dead
    accumulators + `decision_tree_fragment_key` + main/secondary struct
    fields), `intent.rs` (fields, import, carries, summary fn+string),
    `converge.rs` (both snapshot structs + populators + stability sum +
    fixtures), `learn_priors.rs` (ctor + JSON test), `INTENTIR_SPEC.md`.
    Test surgery: removed-surface assertions deleted (3 tests renamed to
    drop the removed surface from their name, retained-surface
    assertions kept). Kept unified (not split) — parallel mechanical
    removal of structurally-identical carriers in the same sites. The
    `.1` converge care-point materialized exactly as predicted: the 2
    `fact_count` unit tests asserted an absolute total incl. the removed
    terms → literals 23→21 / 19→17 updated; convergence *delta* behavior
    unchanged (surface gone from every snapshot equally). In-scope
    cleanup: `ParsedDecisionTreeFragment` trimmed to its sole still-read
    field (`referenced_signal_names`) since `parse_explicit_decision_tree_fragment`
    is retained for signal-connectivity. `cargo check` zero warnings;
    full `scripts/run_ci.sh` green (1054 passed, 0 failed; kg-bench /
    `vlm_state_machine_*` fixtures intact).
  Commit: `see Commit Log`

- ID: `ISF-ONLY-IR-PRUNE.3`
  Status: `superseded`
  Goal: >
    `regular_states` / `state_transitions` removal. SUPERSEDED by user
    decision 2026-05-18: keep them as the canonical state-graph surface
    (R10/R15c VLM state-machine extraction, R15e `vlm_state_machine_*`
    fixtures, R7 state-graph validation, R15f priors). They are NOT
    `.fsm` leftovers; removal is explicitly rejected as a regression.
    No removal action; `.2` prunes only `init_assignments` /
    `decision_tree_fragments`.
  Acceptance: `Superseded — keep regular_states/state_transitions; audit picky-auditor catch upheld.`
  Verification: `passed` — user decision (2026-05-18) "keep them; close .3 as superseded"; no regression
  Commit: `(recorded with the .3 supersede continuity commit)`

- ID: `ISF-ONLY-IR-PRUNE.4`
  Status: `pending`
  Goal: `Close tree (after .2 done and .3 decided); live-doc sync.`
  Acceptance: `Tree closed with .3 resolved (done/superseded/decided); scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-ONLY-IR-PRUNE.1` | `done` | Inventory + per-file removal plan recorded; prune-safe confirmed |
| 2 | `ISF-ONLY-IR-PRUNE.2` | `done` | Both surfaces removed end-to-end; CI green 1054/0; no fixture regression |
| — | `ISF-ONLY-IR-PRUNE.3` | `superseded` | User decision 2026-05-18: keep state-graph; no removal |
| 3 | `ISF-ONLY-IR-PRUNE.4` | `pending` | Next — close tree (`.2` done, `.3` superseded) |

## Impact analysis (`.1`, 2026-05-18 — full repo grep inventory)

Classification:

| Surface | `semantic.rs` | `intent.rs` | `converge.rs` | `learn_priors.rs` | `validate.rs` | `kg_bench.rs` | kg fixtures | verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `init_assignments` | 12 (type@961, prod@2136, fields@60/1240, test@10967) | 8 (field@62, carry@166/204/236, summary-fn, tests) | 9 (2 snapshot count fields + stability sum + tests) | 2 (empty ctor @1799, JSON test str @2591) | **0** | **0** | **0** | **prune-safe** |
| `decision_tree_fragments` | 13 (type@991, prod@2257, fields@66/1246) | 9 (field@68, carry@169/207/239, summary-fn, tests) | 9 (snapshot + sum + tests) | 2 (empty ctor @1802, JSON test str @2594) | **0** | **0** | **0** | **prune-safe** |
| `regular_states` | 33 | 8 | 9 | 2 | **15** | **9** | **5+ `vlm_state_machine_*`** | **load-bearing → KEEP** |
| `state_transitions` | 31 | 8 | 9 | 2 | **4** | **6** | **5+ `vlm_state_machine_*`** | **load-bearing → KEEP** |

`InitAssignmentRecord` (`semantic.rs:961`) and `DecisionTreeFragmentRecord`
(`semantic.rs:991`) are referenced *only* by the two prune-safe fields
(import `intent.rs:15-16`, producers `semantic.rs:2136`/`2257`, one
`semantic.rs:10967` test) — so the types + producers + import are removed
with the fields.

### `.2` removal plan (exact, per file)

1. `semantic.rs`: delete `struct InitAssignmentRecord` (@961) and
   `struct DecisionTreeFragmentRecord` (@991); delete the two producer
   sites (@2136 init-from-non-conflicting-entries, @2257
   decision-tree-fragment map) and their wiring; drop both fields from the
   main `SemanticIr` (@60/@66) and the secondary snapshot/diff struct
   (@1240/@1246); fix the `semantic.rs:10967` test.
2. `intent.rs`: drop fields (@62/@68), the import (@15-16), the
   `semantic_ir.*.clone()` carries (@166/@169), the summary-fn params +
   `.len()` log uses (@204/207/470/473/488/491), the struct init
   (@236/@239); update carry-through tests (@2659/@2736/@2794/@2864/@2967)
   — assertions on the removed surfaces are deleted, not weakened.
3. `converge.rs`: remove the `init_assignments`/`decision_tree_fragments`
   `usize` fields from BOTH snapshot structs (@614/617, @699/702), their
   `ir.*.len()` populators (@647/650, @728/731), and their terms in the
   stability **sum** (@675/678, @752/755). **Care point (not blind
   delete):** the sum is a convergence-stability total; removing a
   surface that is deleted *everywhere* contributes 0 to every snapshot,
   so convergence deltas are unchanged — but `.2` MUST keep every other
   sum term intact and re-verify convergence tests. Update the snapshot
   test fixtures (@1133/1136, @1171/1174, @1202/1205).
4. `learn_priors.rs`: delete the empty-ctor field inits (@1799/@1802) and
   the `"init_assignments": []` / `"decision_tree_fragments": []` lines in
   the JSON test string (@2591/@2594). No prior-extraction logic consumes
   them.
5. Docs: `INTENTIR_SPEC.md` describes both fields — update to drop them
   (no kg-bench/book-behavior doc references them; `regular_states`/
   `state_transitions` doc stays).
6. `.2` MAY split per surface (`init_assignments` then
   `decision_tree_fragments`) if the single diff is too broad
   (TASK_TREE rule 5).

## Decisions

- `2026-05-18`: Scope corrected from the audit — only
  `init_assignments` / `decision_tree_fragments` are treated as
  `.fsm`-era prune candidates; `regular_states` / `state_transitions`
  are R10/R15c/R15e/R7/R15f load-bearing and are NOT removed without an
  explicit user decision (recorded as blocked `.3`).
- `2026-05-18` (user decision): keep `regular_states` /
  `state_transitions` as the canonical state-graph surface; `.3`
  `superseded`, no regression. Tree scope is now firmly: prune only
  `init_assignments` / `decision_tree_fragments` (gated by `.1`).

## Open Questions

- `.1` may find `init_assignments`/`decision_tree_fragments` also have a
  retained consumer (e.g., a kg fixture or validation finding); if so
  they move to the same user-decision posture as `.3`.

## Blockers

- None. `ISF-ONLY-IR-PRUNE.3` was blocked pending a user decision on the
  state-graph surfaces; **resolved 2026-05-18** — user chose "keep them;
  close `.3` as superseded". No regression to R10/R15c/R15e/R7/R15f.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `ISF-ONLY-IR-PRUNE.1` | full repo grep inventory of all 4 surfaces (code/fixtures/book); per-file removal plan | `passed` (prune-safe confirmed for 2; load-bearing confirmed for 2; no code change) |
| `2026-05-18` | `ISF-ONLY-IR-PRUNE.2` | end-to-end removal of `init_assignments`+`decision_tree_fragments`; `cargo check` 0 warnings; full `scripts/run_ci.sh` | `passed` (1054 passed, 0 failed; kg-bench/vlm fixtures intact; converge fact_count literals 23→21/19→17, delta-behavior unchanged) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-ONLY-IR-PRUNE.1` | `ISF-ONLY-IR-PRUNE.1 — impact analysis + per-file removal plan` | docs-only; prune-safe confirmed, state-graph keep upheld |
| `ISF-ONLY-IR-PRUNE.2` | `ISF-ONLY-IR-PRUNE.2 — remove dead .fsm-era init_assignments + decision_tree_fragments IR` | 5 files + spec + tests; converge delta-preserving; CI 1054/0 |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit. Scope corrected:
  state-graph surfaces are load-bearing for R10/R15c/R15e/R7/R15f, not
  `.fsm` leftovers; their removal is a blocked user-decision leaf.
- `2026-05-18`: `.1` done — full repo grep inventory recorded ("Impact
  analysis"). Confirmed with hard evidence: `init_assignments` /
  `decision_tree_fragments` have zero `validate.rs`/`kg_bench.rs`/
  fixture/adapter consumers (prune-safe); `regular_states` /
  `state_transitions` are validate+kg_bench+5-fixture load-bearing
  (`.3` supersede upheld). Exact per-file `.2` removal plan recorded,
  including the converge stability-sum care point (remove-everywhere ⇒
  0 delta ⇒ convergence-preserving, but verify, don't blind-delete).
- `2026-05-18`: `.2` done — `init_assignments` + `decision_tree_fragments`
  removed end-to-end (semantic.rs/intent.rs/converge.rs/learn_priors.rs
  + INTENTIR_SPEC.md + ~19 test sites + dead-helper cascade +
  `ParsedDecisionTreeFragment` trim). The `.1` converge care-point
  materialized exactly as predicted and was handled correctly (not
  blind-deleted): 2 `fact_count` unit tests asserted an absolute total
  incl. the removed terms → literals 23→21 / 19→17; convergence delta
  behavior unchanged. Kept unified (parallel identical-site removal).
  Full CI 1054/0; kg-bench / `vlm_state_machine_*` fixtures intact.
