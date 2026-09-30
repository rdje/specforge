---
id: a-sharded-task-trees-leaf-count-is-bounded-by-its-route-catalog
title: A sharded task tree's leaf count is bounded by its route catalog, and the manifest count that duplicated it is removed behind a declared exemption
date: 2026-09-30
status: accepted
scope: task-evidence, route-catalog, capacity, live-document-size, doctrine
evidence: docs/tasks/live-document-pressure-headroom/pointer-index-and-ownership.md (LIVE-DOCUMENT-PRESSURE-HEADROOM.17); scripts/check_active_task_evidence.pl (validate_leaf_route_exemption); doctrine/live_document_size/*_task_evidence.json
answers:
  - "why does a sharded task-evidence contract declare no max_leaf_routes"
  - "what is leaf_route_exemption in a task-evidence contract"
  - "how many leaves can a migrated task tree hold"
  - "what bounds a task tree's leaf routes after ADR 0046"
  - "what is the next structural stop for a task tree's leaf routes"
  - "what is LIVE-DOCUMENT-PRESSURE-HEADROOM.17"
---

# ADR 0056: A sharded task tree's leaf count is bounded by its route catalog

## Context

ADR 0046 sharded a migrated tree's route catalog into parts (`routes-0001.md`, …) and said where the nearest stop
went: to `limits.manifest.max_leaf_routes`, which `check_active_task_evidence.pl` caps at 128 in every contract. It
named the shape — a bound with no declared rollover — and gave it to `LIVE-DOCUMENT-PRESSURE-HEADROOM.17`.

On `2026-09-30` `EXTRACTION-REPRESENTATION-GAPS.41.e` brought that tree to **126 of 128** routes. The bound has no
warning band, so the first sign of it would have been a refused commit two leaves later. The same leaf-route
membership is already bounded a second way: a sharded contract declares `limits.route_parts` —
`max_parts` × `routes_per_part` (6 × 96 = 576 in every sharded contract today), per-part health targets with warning
and rollover milestones, and a remedy the writer can reach (another part, up to the portable caps of 16 parts of
128 routes). One bound held by two fields is how two surfaces come to disagree, and here the tighter copy was the
one with no bands and no remedy.

## Decision

1. A contract whose `route_catalog_state` is `sharded` declares **no** `limits.manifest.max_leaf_routes`. Its route
   count is bounded by its route catalog alone.
2. The removal is a **declared exemption**, never a deleted field any contract can copy: a
   `limits.manifest.leaf_route_exemption` object with an `authority` (this record, a tracked
   `docs/decisions/` file), a `work_unit` and a `rationale`. `check_active_task_evidence.pl` refuses, as separate
   faults, a sharded contract that keeps the count, one without the exemption, an exemption whose authority is not
   a tracked decision record or that lacks a field, and an inline contract that declares the exemption.
3. An `inline` contract has no route parts, so it keeps `max_leaf_routes`, still capped at 128, and the count
   check applies only where the count is declared.

## Consequences

- The six sharded contracts drop the count and declare the exemption: extraction representation gaps (126
  routes), pressure headroom (94), KG-ISF completeness (90), spec-to-intent alignment (84), extraction quality
  gauge (66) and claim verification (54). The two inline contracts keep theirs (52 of 128, 57 of 96).
- **The nearest stop relocates again, and this record says so rather than claiming a removal.** Every contract's
  leaf routes are also written into the tree's `manifest.json`, whose portable byte ceiling is 65,536. The
  extraction representation gaps manifest is 38,790 bytes at about 206 bytes per route, so it binds near 255
  routes, and it has no warning band either. `LIVE-DOCUMENT-PRESSURE-HEADROOM.17a` owns it.
- The route catalog's own capacity (576 routes today) is reported by the parts' bands and reached through a
  declared remedy, which is the property the removed count never had.

## Links

- Precedent: [ADR 0045](0045-task-plane-cardinality-is-removed-behind-a-declared-exemption.md) (the same removal
  shape for the task-plane file count) and [ADR 0046](0046-task-evidence-route-catalogs-shard-by-lifecycle.md) (the
  sharding that relocated this stop).
- Task tree: [`LIVE-DOCUMENT-PRESSURE-HEADROOM`](../tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md) (`.17`, `.17a`).
