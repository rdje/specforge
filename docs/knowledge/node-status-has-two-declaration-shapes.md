---
id: node-status-has-two-declaration-shapes
title: A task node declares its status in two interchangeable shapes, and a reader that knows only one silently reports "unproven" instead of "absent"
answers:
  - "why did the route-catalog lifecycle gate corroborate 0 of 56 leaves in EXTRACTION-QUALITY-GAUGE"
  - "what are the two shapes a SpecForge task node uses to declare its status (indented on the continuation lines under the id, or inline on the id's own line after a separator)"
  - "how many docs/tasks node declarations state their status inline on the id line (466 of 1,362, 34%, measured 2026-09-17)"
  - "what does max_unverified_routes actually absorb in check_active_task_evidence.pl (routes whose primary part declares no READABLE status - which is not the same as no status)"
  - "can a migrated task tree satisfy the lifecycle gate with zero lifecycles re-derived (yes, if the whole tree is written in a node shape the reader cannot parse)"
  - "does widening declared_node_statuses to the inline form change any existing migrated tree (no - claim_verification 0, pressure_headroom 0, spec_to_intent 1, before and after)"
  - "why does declared_node_statuses not match the separator between the id and the status"
date: 2026-09-17
status: current
tags: [task-tree, live-document-size, doctrine, gate, lifecycle, measurement, live-document-pressure-headroom]
evidence: scripts/check_active_task_evidence.pl (declared_node_statuses, validate_route_lifecycles); docs/tasks/live-document-pressure-headroom/current-and-open-work.md (.32); docs/tasks/EXTRACTION-QUALITY-GAUGE.md
reverify: "perl scripts/check_active_task_evidence.pl --self-test (expect 69/69) && bash scripts/check_task_evidence_contracts.sh; re-census the shapes with: grep -rhoE '^- ID: `[^`]+` +· +Status: `' docs/tasks --include=*.md | wc -l  against  grep -rhoE '^- ID: `[^`]+` *$' docs/tasks --include=*.md | wc -l"
---

`validate_route_lifecycles` is the leg that stops a migrated tree's landing from *asserting* which leaves
are open. For every declared route it re-derives `open`/`closed` from the leaf's own node in its primary
part, and refuses when the landing disagrees. It is a good gate. It was also, for a third of this
repository, reading nothing at all.

## The two shapes

A node declares its status either on the continuation lines beneath the id:

```text
- ID: `TREE.4`
  Status: `done` (`2026-09-13`)
```

or inline, on the id's own line after a separator:

```text
- ID: `TREE.4` · Status: `done` (`2026-09-13`) · Goal: …
```

Both are the owner's own declaration and this repository uses them interchangeably. Measured across
`docs/tasks/` on `2026-09-17`: **466 inline against 896 indented, 1,362 in total — 34% inline.**

## Why the gap was invisible

`declared_node_statuses` anchored on `- ID: \`X\`` followed by *end of line*, so an inline declaration
never matched the id at all. The function then returned an empty list, which
`validate_route_lifecycles` classifies as **uncorroborated**, not as wrong — and uncorroborated routes
are absorbed by the contract's own `max_unverified_routes` budget. Nothing was ever reported as broken:
the gate simply had no opinion, and the contract's budget said that was allowed.

**The bound that follows is the finding.** A tree written entirely in the inline shape can satisfy this
gate with **zero** lifecycles re-derived, provided its contract declares a large enough
`max_unverified_routes`. `EXTRACTION-QUALITY-GAUGE` is exactly such a tree: of its 56 declared nodes the
old reader corroborated **0**; the widened one corroborates **56**, separating 14 open leaves from 42
closed. Partitioning it under the old reader would have required declaring `max_unverified_routes: 56`
and shipping a landing whose every open-leaf claim is an assertion — the shape `CLAIM_VERIFICATION.md`
exists to refuse.

## The rule the reader now follows

Take the first backticked `Status:`/`State:` on the id's own line; fall back to the indented
continuation lines when that line states none. Three properties are deliberate:

- **The separator is not part of the grammar.** The reader never matches the `·`, so it does not depend
  on one punctuation choice, and an id line annotated with a `Goal:` but whose status sits below still
  reads.
- **Inline wins when a node carries both**, because the owner's own line is the more specific
  declaration.
- **A node that states no status is still uncorroborated.** The reader must never invent one: an
  invented status would silently corroborate whatever the landing happens to claim, which is worse than
  reading nothing.

## Widening cost nothing where it already worked

All three migrated sharded trees are written in the indented shape, so this is measurably a no-op for
them: uncorroborated routes are 0 before and after for `claim_verification` and `pressure_headroom`, and
1 for `spec_to_intent`; all five registered contracts stay valid. The one non-node line the widened id
match now reaches — ``- ID: `LITERATURE-GROUNDING.4`–`.12` ``, a RANGE rather than a declaration —
states no status and is uncorroborated under both readers.

The suite is the oracle rather than the argument: two end-to-end cases append a post-migration node in
the inline shape and assert the positive and the lifecycle-disagreement refusal, three direct assertions
pin the pure reader, and the whole suite fails against a copy with only the reader reverted. 64 → 69.

Links: [[a-doctrine-subset-must-assert-the-leg-that-pays-for-the-rest]],
[[live-surface-edit-bookkeeping-chain]], [[claim-verification-task-evidence-migrated]].
