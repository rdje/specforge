---
id: reviewed-residual-gold-key-law
title: A reviewed residual gold key must be one a carrier can emit, and the captured-region carrier's key is <region_id>|<family>
answers:
  - "why is a reviewed residual cell unmet no matter what production does"
  - "what key does the captured-region residual carrier project"
  - "can a reviewed gold key name the reason a region is not a contract"
  - "what is residual_gold_law"
  - "why was table_0004|toc_non_contract changed"
  - "why was elem_00017|informational_non_contract changed"
  - "how do I add a reviewed residual gold key"
  - "what determines the fact_key of a projected captured-region residual"
  - "is the reviewed residual expected_keys redundant with its predicates"
  - "which reviewed cells are region-scoped and which are fact-scoped"
  - "how is the published current result kept in agreement with the reviewed gold"
  - "what does a review label in a gold key cost"
date: 2026-08-31
status: current
tags: [reviewed-dataset, residual, gold, source-to-intent, gate, spec-to-intent-alignment]
evidence: crates/specforge/test_data/source_to_intent_vertical/build_fixture.py (project_captured_regions, residual_cell); doctrine/spec_to_intent/residual_actionability_contract.json (residual_gold_law); scripts/validate_residual_actionability_contract.py (validate_residual_gold_law); docs/tasks/spec-to-intent-alignment/region-kind-generalisation.md (.9e)
reverify: "python3 scripts/validate_residual_actionability_contract.py --check && python3 scripts/validate_residual_actionability_contract.py --self-test"
---

A reviewed cell's `residual.<stage>.expected_keys` is not free text. It names a record some **carrier** has to
be able to emit, and the captured-region carrier is **region-scoped**: `project_captured_regions` in
`build_fixture.py` builds each projected residual as

```python
fact(region_id, cell["family"], f'{region_id}|{cell["family"]}', ...)
```

Both the `/family` the residual query matches on and the `/fact_key` it keys on come from the reviewed cell
itself. So for a cell whose residual query pins exactly one region and one family, **the predicates already
determine the key**, and the gold can only agree with `<region_id>|<family>` or be unsatisfiable.

## Why an unsatisfiable gold is worse than a wrong number

It never fails loudly. The cell simply stays unmet, and its unmet observation reads as a pipeline gap when it
is a gold defect — so the ratio it feeds cannot be interpreted as a capability at all. Two concrete costs were
measured at `SPEC-TO-INTENT-ALIGNMENT.9e`:

- a perfect prose carrier from `.9c` still could not close `informational_disclaimer`;
- once `.9b`'s table carrier reaches `table_0004`, the projection emits `table_0004|table_of_contents` against
  a gold of `table_0004|toc_non_contract` — a false positive **and** a false negative, worse than the unmet
  cell it replaces.

## Where the review's own judgement actually lives

Not in the key. The cell's **disposition** is `expected_disposition`; the record's **cause** is its typed
`cause`/`reason` fields; both are scored separately. A gold spelling out "this region is not a contract" asks
production to emit a review label, which the genericity doctrine forbids and which `.9a` had already ruled out
for prose (no structural predicate separates advisory from normative prose without a deontic grammar).

## The gate

`residual_gold_law` in `doctrine/spec_to_intent/residual_actionability_contract.json`, executed by
`validate_residual_gold_law`. Every reviewed cell whose `expected_disposition` is not `canonical` must either
satisfy the region law derived from its own predicates, or be declared with the differently scoped production
carrier that owns it and that carrier's exact keys. The physical-link analog cells are the declared
fact-scoped case, owned by `TimingIntentDisposition::NonApplicable`. The same check requires the published
current result's gold to equal the review-locked dataset's, because a scored report echoes its input gold and
the two are synchronized copies that can go stale independently.

**What it still permits:** a declared fact-scoped entry naming a real carrier that could not in fact emit its
keys. That carrier's key law is pinned there, not re-derived.

Changing a gold is still a lockstep operation — see [[reviewed-fixture-projection-digest-lockstep]] — and the
denominator the golds feed is [[required-residual-actionability-denominator]].
