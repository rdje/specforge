# NLI-CLAIM-CONNECTOR: join the condition clause with a connective so claims read as English

## Metadata

- Tree ID: `NLI-CLAIM-CONNECTOR`
- Status: `done` (CLOSED `2026-06-06`; `.1`)
- Roadmap lane: `R16`/`R15e` (NLI gate quality)
- Created: `2026-06-06`
- Parent context: backlog Actionable/discrete follow-up #3. The real-APB `nli-verify` run flagged
  several VALID constraints purely on **phrasing**: `constraint_claim_text` appended the
  connective-stripped condition bare — *"PAUSER must be VALID PSELx is asserted"* — which the NLI
  judge reads as broken English and marks not-entailed.

## Fix

In `constraint_claim_text` (`ir/nli_verify.rs`): when appending `condition_text`, re-join it with a
connective so the claim is a grammatical sentence — *"PAUSER must be VALID **when** PSELx is
asserted"*. `"when"` by default; if the stored clause already begins with a subordinating
conjunction or a preposition (`for`/`during`/`while`/`if`/…), it is kept verbatim
(`"… for read transfers"`, `"… during the access phase"`). No-regression for the `unless`/`while`
cases (a wrong-connective claim is still flagged — the connective default never *falsely passes*).

## Verification

Passed (`2026-06-06`) — real-APB `nli-verify`: **not-entailed 8 → 4** (the four resolved were pure
phrasing flags; the remaining four are genuinely-soft / condition-dropping constraints, not phrasing).
+1 test (`constraint_claim_text_joins_a_bare_condition_clause_with_when`); the existing
`constraint_claim_text_carries_condition` ("for read transfers" kept) still passes. Full
`scripts/run_ci.sh` GREEN (1273→1274).

## Task Tree

- ID: `NLI-CLAIM-CONNECTOR` · Status: `done` · Children: `.1`
- ID: `NLI-CLAIM-CONNECTOR.1` · Status: `done` · Goal: connective-join the claim condition.
  Verification above.

## Changelog

- `2026-06-06`: Created + CLOSED — connective-join in `constraint_claim_text`; APB NLI not-entailed
  8→4. (Backlog Actionable/discrete #3.)
