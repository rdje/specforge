---
id: reviewed-fixture-projection-digest-lockstep
title: Changing the reviewed fixture builder is a three-way lockstep, not a one-file edit
answers:
  - "what breaks when build_fixture.py changes"
  - "why does the trajectory snapshot reject my fixture builder change"
  - "where is build_fixture.py digest pinned"
  - "what is REPLAY_PROJECTION_SHA256"
  - "how do I change the reviewed source-to-intent fixture projection"
  - "can the reviewed dataset source_region predicates be edited"
  - "why is the reviewed dataset review-locked"
  - "what must change together with the population replay orchestrator"
date: 2026-08-27
status: current
tags: [reviewed-dataset, fixture, trajectory, digest-pin, source-to-intent, lockstep]
evidence: crates/specforge/src/test_support/trajectory_snapshot.rs; crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json; crates/specforge/test_data/source_to_intent_vertical/build_fixture.py
reverify: "shasum -a 256 crates/specforge/test_data/source_to_intent_vertical/build_fixture.py scripts/replay_source_to_intent_population.py && grep -n 'REPLAY_PROJECTION_SHA256\\|POPULATION_REPLAY_ORCHESTRATOR_SHA256' -A1 crates/specforge/src/test_support/trajectory_snapshot.rs"
---

`crates/specforge/test_data/source_to_intent_vertical/build_fixture.py` projects the reviewed source-to-IntentIR
population into the fixture the evaluator scores. It looks like an ordinary test-data script. It is not: its
SHA-256 is pinned in **three** places that are validated against each other, so editing it alone always fails.

| Surface | What it pins |
| --- | --- |
| `crates/specforge/src/test_support/trajectory_snapshot.rs` | `REPLAY_PROJECTION_SHA256` — a compiled constant |
| `crates/specforge/test_data/trajectory/replays/reviewed_population_current_binary_replay.json` | `tools.projection.sha256` in the published replay record |
| the validator itself | compares the two and rejects a mismatch as `population replay tool identity is stale` |

`scripts/replay_source_to_intent_population.py` carries the identical arrangement through
`POPULATION_REPLAY_ORCHESTRATOR_SHA256` and `tools.orchestrator.sha256`. The same file also pins
`REVIEWED_DATASET_SHA256`, `POPULATION_REPLAY_PRODUCTION_REVISION`, and `REVIEWED_REVISION`, and asserts the
cleanup census of the replay scratch — so a change to the projection is a change to a *published measurement's*
identity, not to a test helper.

A projection change is therefore one atomic edit across the script, the constant, and the replay record. If the
change alters what the fixture projects, the published result it produced is no longer the result the current
tool would produce, and the replay must be re-run and re-published rather than re-stamped — that is what
`SPEC-TO-INTENT-ALIGNMENT.8d` did.

**The reviewed dataset is a separate lock with a stricter rule.** Each cell's `source_region` is a declarative
predicate set (`{"operator": "equals", "field": "/region_id", "value": "elem_00205"}`) inside
`reviewed_dataset.json`, which is review-locked and digest-pinned. `SOURCE-IR-REPRODUCIBILITY`'s non-goal is
explicit: an anchor must not be re-pointed at whatever ingest currently emits, because that makes the fixture
agree with drift instead of detecting it ([[source-ir-ingest-not-reproducible]]). So content-addressing reviewed
anchors (`SOURCE-IR-REPRODUCIBILITY.2`) has to change how `source_record` *resolves* a region — finding the
element whose text carries the reviewed excerpt, and failing closed on zero or multiple matches — without
rewriting a single reviewed fact or predicate value.

[[source-to-intent-vertical-evaluator]] owns the evaluator this fixture feeds.
