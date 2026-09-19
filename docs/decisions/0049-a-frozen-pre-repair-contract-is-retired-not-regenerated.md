---
id: a-frozen-pre-repair-contract-is-retired-not-regenerated
title: A frozen pre-repair contract goes red when the repair lands, and the answer is retirement — never regenerating its witness
date: 2026-09-19
status: accepted
scope: spec-to-intent-alignment, doctrine-enforcement, claim-verification, measurement-integrity
evidence: doctrine/spec_to_intent/canonical_recovery_contract.json; crates/specforge/src/ir/evidence.rs (mod canonical_inference_antecedent_recovery); crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; docs/tasks/spec-to-intent-alignment/canonical-recovery.md (SPEC-TO-INTENT-ALIGNMENT.7a); docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md (.4)
reverify: "cargo test --offline -p specforge-core --lib canonical_inference_antecedent_recovery — expect 6 passed"
answers:
  - "what is ADR 0049"
  - "why was validate_canonical_recovery_contract.py deleted"
  - "what replaced validate_canonical_recovery_contract.py"
  - "a frozen contract validator is red, should I regenerate its witness"
  - "is a red validator that no gate runs a defect"
  - "why must a pre-repair freeze not be wired as a doctrine"
  - "how do I tell a stale contract from a broken one"
---

# ADR 0049: A frozen pre-repair contract goes red when the repair lands, and the answer is retirement — never regenerating its witness

## Context

`SPEC-TO-INTENT-ALIGNMENT.7a` (closed) froze the first-boundary repair contract for the APB canonical
loss `PSEL|must_be_asserted|<missing>`: the exact witness, the grammar, the controls, and the replay
obligations, explicitly **"before production changes"**. It shipped
`doctrine/spec_to_intent/canonical_recovery_contract.json` and a checker,
`scripts/validate_canonical_recovery_contract.py`.

`RETAINED-BUNDLE-POPULATION-FROZEN.1` found that checker reporting **10 errors** against the live tree,
and — decisively — that **nothing invokes it**. No commit ever added it to `scripts/check_doctrines.sh`
or `scripts/run_ci.sh`; `git log -S` over both files returns empty. So it had been wrong in silence,
and no gate could ever have said so.

## The finding

**It is red because the repair it was gating succeeded.** The contract froze two key sets side by side:
`current_canonical_keys`, what the pipeline produced at freeze time, and `expected_canonical_keys`,
what it should produce once repaired.

| | frozen `current` (pre-repair) | frozen `expected` (post-repair) | live today |
| --- | --- | --- | --- |
| witness canonical keys | 3, no `PSEL\|must_be_asserted` | 4, with it | **4, with it** |
| intent true positives | 39 | — | **40** |
| intent false negatives | 1 | — | **0** |
| canonical provenance | 42/42 | — | **45/45** |
| conservation | 117/117 | — | **120/120** |

The live snapshot matches the **expected** column. The validator fails precisely because it asserts the
live tree still equals the **current** column — that is, that the defect is still present.

## Decision

**Retire `scripts/validate_canonical_recovery_contract.py`. Do not wire it, and do not regenerate its
witness.**

Both alternatives are actively wrong, and for different reasons:

- **Wiring it as a doctrine** would gate the repository on the APB canonical loss *still being present*.
  A green build would then require the defect. That is the opposite of what `7a` wanted.
- **Regenerating the frozen witness** to make it pass would overwrite `7a`'s completion evidence with
  post-repair values, destroying the record of what the repair was measured against. A freeze whose
  witness is refreshed whenever it disagrees with production is not a freeze; it is a mirror.

**What replaced it:** `crates/specforge/src/ir/evidence.rs`, `mod canonical_inference_antecedent_recovery`
— 6 passing tests, including `frozen_contract_matrix_executes_against_the_production_sibling` and
`full_build_recovers_the_exact_witness_without_borrowing_the_consequence`. The frozen contract matrix is
therefore still executed against production on every `cargo test`, which is the coverage the Python
checker was written to give and the only one that ever ran in CI.

`doctrine/spec_to_intent/canonical_recovery_contract.json` **stays**. It is `7a`'s frozen evidence and
the Rust tests' input; only the redundant, unrun, and now-misleading checker goes.

## Consequence — the general rule

A contract frozen *before* a repair is a **pre-repair snapshot**, not a live invariant. When the repair
lands it must go red, and that red is success rather than drift. Such a contract has exactly two honest
end states: its obligations migrate into a test that runs against production, or it is retired with the
reason recorded. It must never be wired to a gate, and its witness must never be regenerated to silence
it.

The diagnostic that separates this case from a genuine defect is cheap: read the frozen contract's own
`expected` column. If the live tree matches **expected**, the repair landed and the contract is spent.
If it matches neither, that is real drift and belongs to whoever owns the boundary.
