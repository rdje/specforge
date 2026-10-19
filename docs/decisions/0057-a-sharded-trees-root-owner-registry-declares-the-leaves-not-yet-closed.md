---
id: a-sharded-trees-root-owner-registry-declares-the-leaves-not-yet-closed
title: A sharded task tree's root owner registry declares the leaves not yet closed, and a closed leaf leaves it for the route catalog
date: 2026-10-19
status: accepted
scope: task-evidence, task-trees, route-catalog, capacity, live-document-size, doctrine
evidence: docs/tasks/extraction-representation-gaps/register-binding-leaves.md (EXTRACTION-REPRESENTATION-GAPS.48); scripts/check_active_task_evidence.pl (root_owner_ids, validate_route_lifecycles); doctrine/live_document_size/*_task_evidence.json
reverify: "perl scripts/check_active_task_evidence.pl --self-test; for f in doctrine/live_document_size/*_task_evidence.json; do perl scripts/check_active_task_evidence.pl --contract $f --check; done"
answers:
  - "what is ADR 0057"
  - "which leaves does a task tree root's Executable Owner Registry declare"
  - "why did a closed leaf disappear from a task tree root's owner registry"
  - "where do I find a closed leaf's owner (the route catalog routes-000N.md, and its own record in its part)"
  - "why does SPEC-TO-INTENT-ALIGNMENT.9a stay in its root's owner registry although it is closed"
  - "why did registering three leaves refuse the EXTRACTION-REPRESENTATION-GAPS root at its mandatory rollover"
  - "what is EXTRACTION-REPRESENTATION-GAPS.48"
---

# ADR 0057: A sharded tree's root owner registry declares the leaves not yet closed

## Context

A migrated task tree's bounded root carries an `## Executable Owner Registry`: one `- ID:` line per owner, required
by the contract's `root_required_literals`. `check_active_task_evidence.pl` required that registry to equal the
tree's complete leaf routes, closed leaves included, so the root gained a line for every leaf the tree had ever
opened and gave none back when one closed. ADR 0046 found and removed the same shape in the task-evidence index —
"its line count was a pure function of the tree's lifetime leaf count" — and left the root's copy of it in place.

On `2026-10-19` `EXTRACTION-REPRESENTATION-GAPS.3.d.ii.b.iii` registered three children and the root reached 232
of its 256-line health target, 90.6%, past the mandatory rollover milestone. 144 of its lines were owner ids, and
100 of those named closed leaves. Every sharded tree carried the same registry: pressure headroom 95 ids (20 not
closed), KG-ISF completeness 90 (29), spec-to-intent alignment 84 (3), extraction quality gauge 66 (11), claim
verification 54 (2). The route catalog already carries every leaf with its lifecycle and its primary part, and
ADR 0056 bounds the leaf count by it.

## Decision

1. Under a `sharded` route catalog the registry declares **exactly** the routes whose lifecycle is not `closed`, plus
   every closed route whose primary part records no status for it. `check_active_task_evidence.pl` refuses, as
   separate faults, a route that is not closed and is not declared; a closed route that its part records and the
   registry still declares; and a closed route that its part does not record and the registry no longer declares.
2. The cut is read from the same evidence ADR 0046's lifecycle cross-check reads — the leaf's own status line in
   its primary part — so the registry never drops the only owner declaration a leaf has. Today one route is in
   that position, `SPEC-TO-INTENT-ALIGNMENT.9a`, whose record `LIVE-DOCUMENT-PRESSURE-HEADROOM.16` writes; when it
   lands, the rule removes it from the root.
3. An `inline` catalog has no lifecycle to cut on, so a registry there still equals the complete routes.

## Consequences

- The six sharded roots drop 423 closed owner ids: extraction representation gaps 214 -> 116 lines (144 -> 44
  ids), spec-to-intent alignment 212 -> 134 (84 -> 4), extraction quality gauge 200 -> 147 (66 -> 11), KG-ISF
  completeness 195 -> 136 (90 -> 29), pressure headroom 187 -> 114 (95 -> 20) and claim verification 157 -> 107
  (54 -> 2). Each root now grows with work in flight, not with the tree's age.
- No node vanishes: every dropped id keeps its `- ID:` record in a part, which `TASK-NODE-RETENTION` verifies, and its
  route row in the catalog. A closed leaf's owner is found there, or by searching `docs/tasks/` for its id.
- Closing a leaf is now a root edit too: its `- ID:` line and its `root_required_literals` entry leave together, in
  the commit that closes it.

## Links

- Precedent: [ADR 0046](0046-task-evidence-route-catalogs-shard-by-lifecycle.md) (the same lifecycle cut for the
  index) and [ADR 0056](0056-a-sharded-task-trees-leaf-count-is-bounded-by-its-route-catalog.md) (the route catalog
  as the bound on leaf count).
- Task tree: [`EXTRACTION-REPRESENTATION-GAPS`](../tasks/EXTRACTION-REPRESENTATION-GAPS.md) (`.48`).
