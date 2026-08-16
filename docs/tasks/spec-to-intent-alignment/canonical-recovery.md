# SPEC-TO-INTENT-ALIGNMENT — canonical recovery

- Part ID: `canonical-recovery`
- State: `active`

## Active canonical recovery

- ID: `SPEC-TO-INTENT-ALIGNMENT.7`
  State: `in_progress`
  Goal: recover source-grounded canonical facts lost at the first failing SourceIR-to-EvidenceIR boundary
  Acceptance: the frozen vertical oracle identifies and repairs a bounded high-impact SourceIR-to-EvidenceIR loss
  family; stage conservation and held-out recall improve without fabrication, provenance, or category regression;
  a comparable snapshot records the result
  Verification: `activation reproduces the sole current APB canonical miss and localizes it to a compound
  inference sentence whose antecedent state has no independent EvidenceIR producer; the compiled specforge-core
  consequence-precision control passes 1/1; exact task evidence, claim/current/book censuses, live-size,
  Knowledge Map, mdBook test/build, and mandatory doctrines pass; .7a through .7c own the frozen contract,
  generic implementation plus chain currency, and complete population publication`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.7 — activate canonical recovery`
  Children: `.7a`, `.7b`, `.7c`

- ID: `SPEC-TO-INTENT-ALIGNMENT.7a`
  State: `done`
  Goal: freeze the first-boundary repair contract, exact witness, grammar, controls, and replay obligations
  Acceptance: current tracked authority deterministically reproduces the APB
  `PSEL|must_be_asserted|<missing>` loss;
  the design separates a source clause's own explicit signal state from a later inferred consequence; a closed,
  source-grounded grammar covers asserted/deasserted and active-high/active-low interpretation without borrowing
  consequent kind or value; the exact same-clause appositive establishes local `PSEL` while distinct declared
  `PSELX` remains opaque; positive, negative, paraphrase, ambiguity, and no-spelling-alias controls plus
  affected-chain and complete-population replay obligations are fixed before production changes
  Verification: `machine contract joins the exact 12-source/14-cell current witness at 39/0/1 TP/FP/FN,
  freezes seven positive and thirteen negative cases across five inference markers and the complete six-row
  polarity matrix, binds all 24 affected retained chains and the 48-stage publication replay, and rejects all
  eleven controlled mutations; current-result composition, consequence precision, and no-shape-alias baselines
  remain green`
  Commit: `SPEC-TO-INTENT-ALIGNMENT.7a — freeze antecedent recovery contract`

- ID: `SPEC-TO-INTENT-ALIGNMENT.7b`
  State: `pending`
  Goal: recover explicit inference-antecedent signal state and reconcile every production-identity consequence
  Acceptance: a structurally gated producer emits an independently proven constraint only for a source-grounded
  signal with an explicit antecedent state; a bounded same-clause signal appositive may establish that declaration,
  but identifier shape cannot alias it to another declaration; the existing consequent-subject precision rule remains intact; controlled
  positives, negatives, harmless rewrites, active-low polarity, duplication, and no-named-fixture tests pass;
  production-genericity structure and behavior remain green; every affected retained chain is rebuilt and exact
  chain currency reaches zero stale before commit
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.7a`
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-TO-INTENT-ALIGNMENT.7c`
  State: `pending`
  Goal: replay the complete reviewed population and publish comparable canonical-recovery closure
  Acceptance: all 12 reviewed sources and all four isolated stages replay under the frozen oracle; exact canonical,
  provenance, conservation, residual, category, held-out, genericity, and controller deltas are attributed; the APB
  miss closes without a new false fact or unexplained drop; tracked result/replay/controller authorities reproduce
  byte-for-byte; full selected CI, mdBook, retrieval truth, task parents, cleanup, and residue census agree
  Prerequisite: `SPEC-TO-INTENT-ALIGNMENT.7b`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | State | Why next |
| --- | --- | --- | --- |
| 1 | `SPEC-TO-INTENT-ALIGNMENT.7b` | `pending` | implement the frozen producer and reconcile production-identity consequences |
| 2 | `SPEC-TO-INTENT-ALIGNMENT.7c` | `pending` | publish complete population and controller evidence after repair |

## Frozen antecedent-state contract (`.7a`)

Machine authority is `doctrine/spec_to_intent/canonical_recovery_contract.json`; its independent checker is
`python3 -B scripts/validate_canonical_recovery_contract.py --check`. The recovery is a sibling producer inside
the existing `evidence.normative` signal-constraint family. It does not change the consequence splitter.

The producer may emit only when the bounded constraint-bearing sentence has exactly one supported inference
marker, the prefix ends in exactly one `is asserted` or `is deasserted` form, the governing identifier resolves
exact-first or unique-case-insensitively to one current-document declaration, and the prefix mentions no second
declared signal or state. The exact witness first establishes local `PSEL` through the same-clause descriptive
signal appositive; the description carries no meaning. Distinct declared `PSELX` remains opaque: suffix shape
cannot create an alias. Negation, a trailing qualifier, multiple markers, a case-fold collision, another
statement class, or any absent premise emits nothing. The output cites the complete statement and its exact id,
has no condition or negation, and enters the existing polarity post-pass as `MustBeAsserted` or
`MustBeDeasserted`. Active-high, active-low, and unknown polarity preserve the six already-defined outcomes.
The exact witness has unknown polarity, so its canonical kind remains `MustBeAsserted`; HIGH and LOW are both
forbidden without independent polarity evidence.

The conformance matrix fixes seven positives and thirteen refusal cases. It covers all five existing inference
markers, both state forms, the complete polarity matrix, descriptive and case-only rewrites, semantic
deduplication, undeclared/multiple/contradictory subjects, no-marker and condition-only prose, non-copular and
negated states, trailing qualifiers, missing states, multiple markers, declaration case-fold ambiguity, and
wrong classification. A dedicated case proves bare `PSEL` cannot resolve to declared `PSELX` by suffix spelling.
The exact APB case separately forbids `PSEL|must_be_value|VALID`, so consequence borrowing cannot masquerade as
recovery.

Any `.7b` production-semantic edit invalidates the EvidenceIR ruleset identity and therefore owns exact
reconciliation of all 24 retained chains through EvidenceIR, SemanticIR, IntentIR, and the ISF adapter. `.7c`
then owns the independent 12-source × four-stage reviewed replay and comparable publication; a focused APB pass
cannot substitute for the full population.

## Acceptance Checklist (enforced)

- [x] **REPRODUCE / MEASURE** — current authority remains 39/0/1 IntentIR TP/FP/FN, 42/42 provenance,
  117/118 conservation, and one APB first-boundary drop; the contract checker derives the same witness.
- [x] **ROOT CAUSE (WHY + WHERE)** — `current_result_snapshot.json` loses only
  `PSEL|must_be_asserted|<missing>`; `consequent_after_inference_marker` correctly protects suffix subjects,
  while no sibling producer captures the explicit prefix state.
- [x] **ADDRESSED (verified)** — the design gap is closed by an executable 7-positive/13-negative contract and
  11/11 mutation controls; production extraction is deliberately unchanged and remains owned by `.7b`.
- [x] **NO REGRESSION** — the contract checker and self-test pass; compiled consequence-precision,
  no-index-shape-alias, retrospective snapshot, and current trajectory composition tests each pass 1/1.
- [x] **GENERICITY (ADR 0006 / ADR 0037)** — authority comes from current-document declarations, bounded
  appositive syntax, and closed state/polarity structure; identifiers remain opaque and names occur only in the
  conformance witness and cases, never as production selectors or shape-derived aliases.
- [x] **LOCKSTEP** — task root, roadmap, live/resume/engineering records, mdBook, and Knowledge Map route `.7b`
  next without claiming a product-behavior improvement.

## Decisions

- `2026-08-16`: preserve the existing consequence extractor. It correctly excludes the causal antecedent from
  the later `must be valid` obligation; recovery must create an independently grounded antecedent-state fact.
- `2026-08-16`: freeze declaration grounding, explicit state grammar, polarity handling, ambiguity refusal, and
  whole-population proof before production mutation. The APB witness selects the family but cannot enter core.
- `2026-08-16`: treat `PSEL` as the exact span's source-local appositive declaration, not a spelling alias for
  the distinct catalog declaration `PSELX`. This preserves the review-locked oracle and ADR 0037 together.
- `2026-08-16`: correct the oracle's unsupported `asserted`→`HIGH` refinement. No selected evidence resolves
  polarity, so the missing fact stays `MustBeAsserted`; the population score remains 39/0/1.
- `2026-08-16`: keep the new interpretation inside the existing normative constraint family as a sibling
  producer. The common polarity post-pass and normal proof/carry path remain the only downstream authority.

## Open Questions

- None for `.7b`; its implementation must consume the frozen matrix without weakening a refusal case.

## Blockers

- None for `.7b`; missing authority or ambiguous clause structure remains a negative case, not a widened
  heuristic.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-16` | `.7a` contract freeze | exact witness/current join, 7/13 conformance matrix, local-appositive/opaque-no-alias/unknown-polarity controls, five markers, six polarity outcomes, 24-chain and 48-stage obligations pass; 11/11 mutations reject and all four focused 1/1 baselines remain green |
| `2026-08-16` | `.7` activation | current result and 1/1 compiled core precision control localize one explicit antecedent-state loss; 56-unit/86-candidate current census and 304-region book census stay closed; three bounded children own design, implementation/currency, and replay/signoff |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.7a` | `SPEC-TO-INTENT-ALIGNMENT.7a — freeze antecedent recovery contract` | freeze the executable witness, grammar/refusal matrix, affected chains, and population replay boundary without changing production |
| `.7` | `SPEC-TO-INTENT-ALIGNMENT.7 — activate canonical recovery` | open the bounded recovery program with `.7a` as sole frontier |

## Update protocol

Every child updates this part and the bounded root together. Route or measured-metric changes update the
contract, index, and manifest in the same commit. Legacy payloads, the completed behavioral part, and the exact
source capsule remain unchanged.
