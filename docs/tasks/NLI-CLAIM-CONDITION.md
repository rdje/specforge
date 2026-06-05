# NLI-CLAIM-CONDITION: carry a constraint's condition into its NLI claim (precision fix)

## Metadata

- Tree ID: `NLI-CLAIM-CONDITION`
- Status: `done` (CLOSED `2026-06-05`; `.1` — claim carries `condition_text`)
- Roadmap lane: `R16`/`R15e`
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: surfaced by running `nli-verify` on the **real** AMBA APB EvidenceIR (42
  constraints → 36 flagged). Inspection: most flags are correct (non-signals like `ACCESS`/`SETUP`/
  `USER_*_WIDTH`, the clock `PCLK`, and condition signals all got spurious constraints — the gate
  validates AND exposes constraint over-generation). But a real **false-positive pattern**:
  `constraint_claim_text` renders the claim WITHOUT the constraint's `condition_text`, so a
  legitimately-conditional constraint (`"PSTRB must be LOW"` from *"for read transfers, drive PSTRB
  LOW"*) reads as not-entailed purely because the claim dropped the condition.

## Design

- `SignalConstraintRecord` already carries `condition_text: Option<String>` ("when PSEL is
  asserted", "for read transfers", …). `constraint_claim_text` should **append** it so the claim
  states the same conditionality as the source: `"PSTRB must be LOW for read transfers"` instead
  of `"PSTRB must be LOW"`.
- This raises **precision** (fewer false not-entailed on truly-conditional constraints) without
  losing the **correct** catches: a *mis-attributed* condition signal (PSEL is the condition, not
  the subject) still fails, because the claim "PSEL must be VALID …" still isn't what the source
  obliges.
- Pure change to `constraint_claim_text`; update its unit test; re-run on APB to confirm fewer
  flags on the conditional constraints.

## Acceptance Criteria

- `.1`: `constraint_claim_text` carries `condition_text`; test updated; real-APB re-run shows the
  conditional false-positives drop; book/KM note; full CI GREEN; tree CLOSED.

## Task Tree

- ID: `NLI-CLAIM-CONDITION` · Status: `done` (CLOSED `2026-06-05`) · Children: `.1`
- ID: `NLI-CLAIM-CONDITION.1` · Status: `done` · Goal: carry the condition into the claim +
  test + re-run + record + close.
  Verification: passed (`2026-06-05`) — `constraint_claim_text` now appends `condition_text`
  ("PSTRB must be LOW **for read transfers**"); refactored to build the base then append the
  condition once (covers the `MustNotChange` early-return path too); empty/whitespace condition
  adds nothing. Unit test `constraint_claim_text_carries_condition` added. **Real-APB re-run
  confirms the fix: 36 → 33 not-entailed** (the legitimately-conditional constraints now pass; the
  genuine extraction errors — non-signals, condition signals — correctly stay flagged). KM card
  `nli-gate-real-apb-validation` (also records the validation + the constraint-over-generation
  finding). Full `scripts/run_ci.sh` GREEN.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `NLI-CLAIM-CONDITION.1` | `done` | condition-carrying claim; APB re-run 36→33 → **tree CLOSED** |

**Tree CLOSED `2026-06-05`.** `constraint_claim_text` carries `condition_text`; real-APB
not-entailed dropped 36→33 (conditional false-positives fixed; correct catches kept). The deeper
finding — the constraint extractor over-generates — is recorded in KM
`nli-gate-real-apb-validation` for a possible future investigation.

## Decisions

- `2026-06-05`: carry `condition_text` into the NLI claim so a conditional constraint is judged
  against the same condition the source states (precision, no loss of correct catches).

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `NLI-CLAIM-CONDITION.1` | `NLI-CLAIM-CONDITION.1 — carry condition_text into the NLI claim (real-APB 36→33); close` | +1 test; KM card |

## Changelog

- `2026-06-05`: Created — surfaced by the real-APB `nli-verify` run; carry `condition_text` into
  the claim to cut conditional false-positives.
- `2026-06-05`: **Tree CLOSED.** Claim carries the condition; APB re-run 36→33; KM
  `nli-gate-real-apb-validation`. CI green.
