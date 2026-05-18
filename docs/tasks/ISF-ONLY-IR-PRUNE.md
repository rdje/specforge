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
  Status: `pending`
  Goal: >
    Full impact analysis of `init_assignments`,
    `decision_tree_fragments`, `regular_states`, `state_transitions`:
    enumerate every producer/carrier/consumer (semantic.rs, intent.rs,
    validate.rs, kg_bench.rs + fixtures, learn_priors.rs, converge.rs,
    book) and classify each surface as `prune-safe` or
    `load-bearing (keep / user-decision)`. Produce the exact removal plan
    + the recorded state-graph decision. Docs/analysis only.
  Acceptance: `Per-surface consumer inventory + classification + removal plan recorded; state-graph decision explicit; no code change.`
  Verification: `pending`
  Commit: `pending`

- ID: `ISF-ONLY-IR-PRUNE.2`
  Status: `pending`
  Goal: >
    Remove the `.1`-confirmed prune-safe surfaces (expected:
    `init_assignments`, `decision_tree_fragments`) from `IntentIr`
    (+ `intent.rs` builder/carry, `semantic.rs` producers if exclusively
    feeding them, `learn_priors.rs`/`converge.rs` references) without
    regressing kg-bench or any retained surface.
  Acceptance: `Prune-safe surfaces gone; scripts/run_ci.sh green; no fixture regression. May split per surface.`
  Verification: `pending`
  Commit: `pending`

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
| 1 | `ISF-ONLY-IR-PRUNE.1` | `pending` | Analysis gates the safe removal |
| 2 | `ISF-ONLY-IR-PRUNE.2` | `pending` | Prune only the confirmed-dead surfaces |
| — | `ISF-ONLY-IR-PRUNE.3` | `superseded` | User decision 2026-05-18: keep state-graph; no removal |
| 3 | `ISF-ONLY-IR-PRUNE.4` | `pending` | Close after .2 (.3 resolved as superseded) |

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
| `2026-05-18` | `ISF-ONLY-IR-PRUNE.1` | `pending` | `pending` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-ONLY-IR-PRUNE.1` | `pending` | `pending` |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit. Scope corrected:
  state-graph surfaces are load-bearing for R10/R15c/R15e/R7/R15f, not
  `.fsm` leftovers; their removal is a blocked user-decision leaf.
