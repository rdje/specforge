---
id: task-plane-cardinality-is-removed-behind-a-declared-exemption
title: Task-plane cardinality is removed behind a declared exemption
date: 2026-08-29
status: accepted
scope: live-documents, task-trees, cardinality, doctrine, capacity, routing
evidence: doctrine/live_document_size/surfaces.jsonl; doctrine/live_document_size/ceiling_increase_authorities.jsonl; scripts/check_live_document_size.pl; scripts/check_task_tree_catalog.pl; docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md
reverify: "bash scripts/check_live_document_size.sh; perl scripts/test_live_document_size.pl; perl scripts/check_task_tree_catalog.pl"
answers:
  - "what is ADR 0045"
  - "is there a limit on the number of task trees"
  - "why does docs/tasks have no file count cap"
  - "how can a live surface null a size dimension"
  - "what is a cardinality exemption"
  - "why was MAX_TASKS removed from check_task_tree_catalog.pl"
  - "which surface bounds the number of task trees now"
---

# ADR 0045: Task-plane cardinality is removed behind a declared exemption

## Context

`doctrine/live_document_size/surfaces.jsonl` capped `task_evidence` at `files: 160` in both bands, so the
collection had no warning band: it reported "at or above rollover" and then refused. The number was authored in
`cb65d5c7` (`2026-08-08`) during the bulk activation of the doctrine, when the plane held **122** files. It is
current-plus-headroom, rounded — not a task-plane analysis.

Measured `2026-08-29` at `c1609558`, only the count was under load: **151/160 files (94.4%)**, against
`lines_total` **7.5%**, `bytes_total` **6.0%**, and a largest single file at **42.9%** of its 3,000-line bound.
Of the 151 roots, **120 are `done`, 5 `superseded`, 24 `active`, 1 `proposed`** — 83% of the collection is
finished work, so the bound was measuring cumulative project lifetime rather than concurrent work.

`task_evidence` also has **no declared rollover**, which makes the cap a bound a surface can reach with a
remedy no compliant work can take: the `LIVE-DOC-STOP-RISK` condition, the same shape as the research-records
finding. An unbounded collection is already legal in this registry — `shipped_behavior`, the mdBook, declares
`files: null` in both bands — so a file-count cap is a per-surface choice, not a doctrine requirement.

The director decided (`2026-08-29`) that there is to be no limit on the number of task-trees and that a
completed tree stays as project history. A session never reads `docs/tasks/` in bulk; it reads
`docs/TASK_TREE.md` and opens the one tree it needs.

## Decision

`task_evidence` declares no file-count bound in either band, and the removal is a **declared exemption with
conditions a checker enforces** — never a bare `null` any surface can adopt by editing one field.

A `cardinality_exemption` object names an `authority` (this record), a `work_unit`, a `route_surface_id`, and a
`rationale`. `scripts/check_live_document_size.pl` refuses, as separate faults:

1. a surface that nulls `files` in either band with no exemption declared;
2. an exemption that nulls `files` in only one band;
3. an exemption whose surface also nulls any **resource** dimension — `lines_each`, `bytes_each`,
   `lines_total`, `bytes_total`, `line_bytes_each` all stay numeric, because those bound the resource that
   actually exists;
4. an exemption whose route is the surface itself, is not a registered surface, does not cover the surface's
   declared `index`, or is itself unbounded in any dimension.

`scripts/check_task_tree_catalog.pl`'s independent `my $MAX_TASKS = 160;` and its refusal are deleted in the
same transaction. That literal was a second enforcer of the same number, so a registry-only change would have
read as delivered while the plane stayed capped at 160 by a different file. The catalog checker keeps
`$MAX_SECTION_BYTES` and `$MAX_ROW_BYTES`, which bound content rather than cardinality.

## Consequences

The bound on how many trees may exist is now `task_tree_index` — `docs/TASK_TREE.md`, one catalog row per tree,
bounded at 480 health / 512 ceiling lines. This **relocates** the nearest stop rather than abolishing it:
measured at `c1609558` the index is 404 lines with 242 of them fixed workflow prose, so it admits roughly **108**
further trees against the 9 the file cap allowed. That residual stop has no declared rollover either, which is
why `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c` shards the index the way the Knowledge Map and the fact-card catalog
are already sharded. Until it lands, the directive is relocated, not delivered, and this record says so.

The exemption is single-use in the authority registry: `ceiling_increase_authorities.jsonl` carries one exact
record for this change, and the generic gate refuses an unused or banked authority, so
`LIVE-DOCUMENT-PRESSURE-HEADROOM.2b` must retire it once HEAD carries the new bands.

Nothing about retirement changes. Archiving a tree stays available only for a single tree outgrowing its
**per-file** bound, which is what produced the three existing sealed capsules; no tree is archived for capacity.

## Links

- Owning task tree: [`docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md`](../tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md) (`.2`, `.2a`, `.2b`, `.2c`)
- Doctrine: [`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`](../../LIVE_DOCUMENT_SIZE_CONTAINMENT.md)
- Reachable-stop precedent: [`docs/tasks/LIVE-DOC-STOP-RISK.md`](../tasks/LIVE-DOC-STOP-RISK.md)
- ADR 0032 (aggregate bounds a per-file bound cannot see)
