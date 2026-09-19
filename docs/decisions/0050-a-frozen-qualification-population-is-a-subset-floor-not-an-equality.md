---
id: a-frozen-qualification-population-is-a-subset-floor-not-an-equality
title: A frozen qualification population binds as a subset floor of the live set, and the excess is a declared residual — never an equality
date: 2026-09-19
status: accepted
scope: production-genericity, doctrine-enforcement, chain-currency, measurement-integrity
evidence: doctrine/production_genericity/post_boundary_retention.json; doctrine/production_genericity/behavioral_qualification.json; scripts/check_behavioral_genericity_contract.py; doctrine/chain_currency/retained_bundles.json; docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md; docs/decisions/0049-a-frozen-pre-repair-contract-is-retired-not-regenerated.md; docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md (.2)
reverify: "python3 -B scripts/check_behavioral_genericity_contract.py --self-test"
answers:
  - "what is ADR 0050"
  - "why is the behavioral population a subset of retained rather than equal to it"
  - "how do I retain a new normalized bundle without qualifying it behaviorally"
  - "what does a newly retained document owe the frozen behavioral population"
  - "why did declaring a new retained bundle turn PRODUCTION-GENERICITY red"
  - "how is an unqualified retained key declared"
  - "where does the post-boundary retention declaration live and why"
  - "why is a growing list kept out of a frozen contract"
  - "can a frozen snapshot be wired as a live equality invariant"
  - "which frozen boundary values in behavioral_qualification.json stay exact"
---

# ADR 0050: A frozen qualification population binds as a subset floor of the live set, and the excess is a declared residual — never an equality

## Context

`doctrine/production_genericity/behavioral_qualification.json` freezes the population that
`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f` behaviorally qualified: 24 rows in
`doctrine/production_genericity/behavioral_population.tsv`, a 7/17 calibration/holdout split, and
`declared_attempts: 51` (17 prospective × 3 relations). Its `held_out_policy.prospective_definition`
scopes that population explicitly to the documents held out **"at the selection boundary"**, and the
contract pins `selection_boundary_commit`.

`scripts/check_behavioral_genericity_contract.py` joined that frozen population to the **live**
retained set with **set equality**:

```python
if set(row_keys) != set(retained_keys):
```

ADR 0025 decision 3 mandates that a refresh **keeps** its normalized bundle. So the first refresh after
the freeze had no compliant move: declaring the bundle it retained grew `retained` to 25 and turned
`PRODUCTION-GENERICITY` red. `WIRE-BASED-100.9b`, `.9c`, `.9d` and `WIRE-BASED-100.10` each hit this and
each took the same hold — three of the four wire-based golds are still parked under
`generated/preserved/`, costing their EvidenceIR replay measurability, and the hold is what later made a
destructive re-ingest of those same golds look necessary to another tree.

## The finding

**A snapshot at a commit was wired as a live invariant** — ADR 0049's class of error, one layer up. The
contract's own prose says the population was fixed at a boundary; the checker required it to equal a set
that ADR 0025 requires to move. Equality conflates two genuinely different facts:

| Direction | What it means | Is it a defect? |
| --- | --- | --- |
| a frozen row is **absent** from `retained` | the qualification's evidence rests on a bundle no longer declared retained | **yes** — the guarantee worth keeping |
| a retained key is **absent** from the frozen population | it was retained after the selection boundary and was never qualified | **no** — it is unqualified, which is a debt, not a drift |

### What is actually joined to the live set — measured, not read

`.4` scoped this leaf as needing three separate adjudications, on the reading that
`population_assertions.current_documents: 24` and `frozen_census` are frozen boundary values wired the
same way. **Perturbation shows that is not so, and the correction shrinks the slice.** Declaring the
three held-out gold keys (`retained` 24 → 27) with the frozen TSV untouched yields **exactly one**
problem:

```
behavioral population differs from retained current population:
missing=['ihi0022_l_…', 'ihi0024_e_…', 'ihi0033_c_…'], extra=[]
```

`population_assertions.current_documents` and `frozen_census.aggregate.documents` both stay **green**,
because both are compared against `len(rows)` — the frozen TSV — and the TSV did not move. They are
frozen values checked against the frozen thing they describe, which is self-consistent and correct.
**They stay exact**; they are release evidence for a population that does not move, and relaxing them
would weaken a binding that was never the problem.

## Decision

**The frozen population binds as a SUBSET FLOOR of the live retained set, and every retained key above
that floor is a declared, owned residual.**

1. `set(row_keys) ⊆ set(retained_keys)`. A frozen population row that is no longer retained is **RED**,
   with the same strength equality gave it.
2. Every retained key outside the frozen population must be declared in
   `doctrine/production_genericity/post_boundary_retention.json`, naming the owed held-out relations and
   the leaf that owes them. An **undeclared** one is **RED** — growth is admitted, silence is not.
3. A declaration whose key is not retained, or whose key *is* in the frozen population, is **RED**. The
   block records real debt, not a bypass list.
4. The gate **reports** the unqualified count on its summary line, so an unqualified key is visible on
   every green run rather than discoverable only by reading JSON.

`behavioral_qualification.json` is **not amended at all** — see below.

### Where the declaration lives, and why that is part of the decision

The declaration was first drafted as a `post_boundary_retention` block *inside*
`behavioral_qualification.json`, which is the intuitive home: that contract owns the frozen population.
Measurement rejected it. The contract's digest is pinned in `behavioral_holdout_evidence.json` at the top
level **and in each of the 35 completed held-out attempt identities**, so any amendment re-pins **36**
digests — and `unqualified_keys` is a list that **grows with every retention**, so that cost recurs
forever and each recurrence rewrites the identity records of a closed qualification.

That is this ADR's own error one level down: a moving set housed inside a frozen artifact. So the
declaration lives in its own live file, `doctrine/production_genericity/post_boundary_retention.json`,
referenced by the checker as a module constant rather than through the contract's digest-pinned
`declarations` map. The frozen qualification is left byte-identical, and the change costs **zero**
re-pins.

`population_assertions` and `frozen_census` are unchanged.

## Consequence — the general rule

**A frozen population is a floor, not a fence.** When a contract freezes the population it measured, the
durable guarantee is that nothing it measured may silently vanish — not that the world may never grow
past it. Wiring the second as an equality makes the contract forbid the very operations its own policy
mandates, and the failure surfaces as a stop with no compliant move rather than as an unmet obligation.

A corollary the implementation forced: **a declaration that grows does not belong in an artifact that
is frozen.** Frozen evidence is pinned by whatever consumed it, so amending it to carry live state
charges every future update the full cost of re-pinning a closed record — and quietly erodes the record
itself. Give the moving part its own file.

The honest shape is subset-plus-declared-residual: the floor stays gated at full strength, the excess is
named with its owner and its outstanding work, and the gate states the debt on every run. A qualification
population may then be superseded deliberately, by executing the relations and re-freezing, instead of
being defended by blocking the refresh that would have grown it.

This does **not** admit a newly retained key into the qualified population. It records that the key is
retained, unqualified, and owed — which is what was true all along and what equality made unsayable.
