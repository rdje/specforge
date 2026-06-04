# DEMPSTER-FUSION-COMBINER: corroboration-boosting confidence fusion (Dempster's rule)

## Metadata

- Tree ID: `DEMPSTER-FUSION-COMBINER`
- Status: `active` (`.1` design done; `.2` = implement + close)
- Roadmap lane: `R16`/`R15e` (multimodal fusion / literature grounding)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: user pick ("prior-decay first then the Dempster combiner"). Grounded gap from
  `docs/research/grounding/multimodal-fusion.md` + the adopt/defer ledger (Dempster entry):
  *"replace the `min()` confidence with a Dempster-Shafer belief combination + conflict mass."*
  Dempster, *Upper and Lower Probabilities Induced by a Multivalued Mapping*, Ann. Math. Stat.
  1967 (DOI 10.1214/aoms/1177698950).

## The gap (verified against the code)

`ir/fusion.rs::merge_cluster` clusters `ActorContract`s by `fusion_key` (actor, obligation
kind, primary signal). When a cluster **agrees**, it merges them and combines confidence with
`min_confidence` — which **caps at the weakest source**. That throws away *corroboration*: two
*independent* Medium-confidence sources agreeing on the same contract should yield **higher**
confidence than one, not stay at Medium. (Disagreement — different kind/obligation/guard — is
already routed to a `Residual`, separately.)

## Design — Dempster's rule for agreeing evidence

Model each ordinal confidence as a Dempster-Shafer **belief mass** in the supported proposition
(remainder = uncertainty on the frame): `High → 0.9`, `Medium → 0.7`, `Low → 0.5`. For
independent sources that **all support the same proposition** (the agreement case), Dempster's
rule combines to `m = 1 − ∏(1 − mᵢ)` — the combined belief is **≥ the strongest single source**
(corroboration), never capped at the weakest. Map back: `≥ 0.9 → High`, `≥ 0.7 → Medium`, else
`Low`. Worked: Medium+Medium = `1−0.3·0.3 = 0.91 → High`; Low+Low = `0.75 → Medium`;
High+anything stays High; a single source is unchanged.

`merge_cluster`: on **agreement**, set the merged confidence to the Dempster corroboration of
the whole cluster; on **disagreement**, keep the conservative `min` (it routes to a Residual
anyway — disagreeing sources must not be "corroborated"). `min_confidence` stays (the
disagreement/conservative path).

### Conflict mass K + the Zadeh pathology — why it does not arise here

Full DS tracks a **conflict mass K** and the **Zadeh high-conflict pathology** (K → 1 →
normalize-by-near-zero → counterintuitive certainty). On *this* path K = 0: `merge_cluster`
only combines confidences for clusters that **agree** (disagreement is pre-routed to a Residual
before any confidence combination). So graded-conflict DS (K > 0 + the Zadeh guard) is **not
needed here** and is a documented future extension (it would only matter if fusion later modeled
*partial* conflict instead of the current binary agree/residual split). The combiner is honest
about this rather than adding unreachable conflict-normalization.

## Non-Goals

- NOT graded-conflict DS (K > 0) / the Zadeh guard — K = 0 on this path (agreement-only);
  documented as a future extension.
- NOT changing the agree-vs-residual split (disagreement still → Residual).
- NOT a continuous confidence type — stays the ordinal `AutomationConfidence` (mass mapping is
  internal to the combiner).

## Fixture-churn plan

Boosting agreeing multi-source clusters changes fused-contract confidence (Medium+Medium→High,
etc.), which may shift `converge`/eval snapshots that contain size-> 1 agreeing clusters.
**Implement → run full CI → inspect each changed snapshot → confirm every change is a legitimate
corroboration boost on a genuinely multi-source agreeing cluster (not a regression) → update the
fixture.** If the churn is unexpectedly broad or any change looks wrong, stop and re-scope.

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: `dempster_corroborate_confidence` (+ mass mapping helpers) in `ir/fusion.rs`, wired into
  `merge_cluster` on the agreement path; unit tests (Medium+Medium→High; Low+Low→Medium;
  High caps; single-source identity; order-independence; disagreement keeps the conservative
  value + routes to Residual); any churned fixture inspected + updated with the boost confirmed
  legitimate; a user-friendly book subsection; KM card; full `scripts/run_ci.sh` GREEN; CLOSED.

## Task Tree

- ID: `DEMPSTER-FUSION-COMBINER`
  Status: `active`
  Children: `.1` (design) · `.2` (implement + wire + fixtures + book + KM + close)

- ID: `DEMPSTER-FUSION-COMBINER.1`
  Status: `done`
  Goal: own + design (this file) — the mass mapping + Dempster corroboration, the
    agreement-only K=0 honesty (Zadeh pathology N/A here), the merge_cluster wiring, the
    fixture-churn plan.
  Acceptance: design recorded; registered.
  Verification: passed (`2026-06-04`) — gap verified in `fusion.rs` (`min_confidence` caps at
    weakest; disagreement pre-routed to Residual); design fixed = ordinal→mass (0.9/0.7/0.5),
    `m = 1−∏(1−mᵢ)` corroboration, thresholds (0.9/0.7), agreement-path wiring keeping `min`
    for disagreement; K=0-here reasoned; fixture-churn plan set.
  Commit: `see Commit Log`

- ID: `DEMPSTER-FUSION-COMBINER.2`
  Status: `pending`
  Goal: implement + wire + tests; inspect/update churned fixtures; book + KM; close.
  Acceptance: tests green; fixtures verified; book + KM; full CI GREEN; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `DEMPSTER-FUSION-COMBINER.1` | `done` | owned + designed (Dempster corroboration; K=0 here) |
| 2 | `DEMPSTER-FUSION-COMBINER.2` | `pending` | implement + wire + fixtures + book + KM + close |

## Decisions

- `2026-06-04`: realize the Dempster gap as a **corroboration-boosting** confidence combination
  for agreeing evidence (`m = 1−∏(1−mᵢ)`), replacing `min` on the agreement path. Conflict mass
  K = 0 here (disagreement pre-routed to Residual), so the Zadeh guard is a documented future
  extension, not added as unreachable code. Keep the ordinal `AutomationConfidence` surface.

## Blockers

- None, but watch fixture churn (confidence boosts on multi-source agreeing clusters).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | gap verified in `fusion.rs` (`min_confidence`); Dempster corroboration design (mass map + `1−∏(1−mᵢ)` + thresholds + agreement-path wiring); K=0-here reasoned; churn plan | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `DEMPSTER-FUSION-COMBINER.1` | `DEMPSTER-FUSION-COMBINER.1 — own + design Dempster corroboration confidence fusion` | docs-only |

## Changelog

- `2026-06-04`: Created — replace `min_confidence` in `ir/fusion.rs::merge_cluster` with a
  **Dempster corroboration** for agreeing evidence (`m = 1−∏(1−mᵢ)`), so independent agreement
  raises confidence instead of capping at the weakest. K = 0 on this path (disagreement
  pre-routed to Residual) → the Zadeh conflict guard is a documented future extension.
