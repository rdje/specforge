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
  - "how many digests move when the reviewed projection changes"
  - "how do I regenerate controller_input.json and trajectory_report.json"
  - "why does cargo test -p specforge not run the trajectory snapshot tests"
  - "can I re-stamp a published replay record"
  - "how do I resolve a reviewed region by content"
date: 2026-08-28
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

**Those three are the entry fee, not the bill.** `SOURCE-IR-REPRODUCIBILITY.2` changed the projection and
discharged **eight** pinned surfaces plus the replay authority identity and the published snapshot:

| Pinned surface | Why it moves |
| --- | --- |
| `REPLAY_PROJECTION_SHA256` | the builder's own digest |
| `POPULATION_REPLAY_PRODUCTION_REVISION` | the revision the replay ran at |
| `REPLAY_VERTICAL_RESULT_SHA256` | the published `current_result_snapshot.json` |
| `POPULATION_REPLAY_EVIDENCE_SHA256` | the replay record file itself |
| `cleanup.removed_file_count` / `removed_kib` | the exact residue-free scratch census, compiled as literals |
| `current_dataset.byte_count` | the projection's own size |
| `current_result.byte_count` | the scored result's size |
| `exact_source_regions != N` | **the measured metric is compiled into the validator** |

The last one is the sharp edge: a change that improves a published reviewed metric cannot land without
editing the literal that asserts the old value, and `replay_id` / `owner` / `dataset_id` are asserted too.
Regenerate the derived trajectory artifacts with
`cargo test -p specforge-conformance -- --ignored write_current_trajectory_artifacts_on_explicit_request`;
the module compiles into **`specforge-conformance`** through a `#[path]` include, so
`cargo test -p specforge` never runs any of it.

A published replay record is **never re-stamped**: it keeps the digest of the tool that produced it, so a
projection change publishes its own replay under a new `replay_id`, `owner`, and a current-replay
`dataset_id` distinct from the review-locked dataset's own id.

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
anchors (`SOURCE-IR-REPRODUCIBILITY.2`, done `2026-08-28`) changed how `source_record` *resolves* a region
without rewriting a single reviewed fact or predicate value. Note that "find the element carrying the
excerpt" is **not** the rule that works: measured against the persisted artifacts it resolves 8 of 12
reviewed regions and leaves 4 ambiguous. The shipped rule is a precedence over the region's own
natural-language surface — element text for prose, caption then caption-plus-cells for a table, caption for
a figure — never the record's serialization, failing closed on zero or multiple matches at the deciding
tier. See [[source-ir-ingest-not-reproducible]].

[[source-to-intent-vertical-evaluator]] owns the evaluator this fixture feeds.
