---
id: a-sealed-region-cannot-move-out-of-an-active-part-alone
title: A sealed region cannot be relocated out of an active task-evidence part on its own — the two-stratum supersession is scoped to ONE part file, so the closures must travel with it
answers:
  - "why did moving a sealed region out of an active task-evidence part break leaf lifecycles that were already correct"
  - "what is the scope of the LIVE-DOCUMENT-PRESSURE-HEADROOM.30a two-stratum rule (one part FILE - a leaf's sealed declaration and its post-migration supersession must live in the same file)"
  - "how do I relocate a sealed region to give an active part its budget back without breaking the gate (move the region AND every post-migration record that supersedes a declaration inside it)"
  - "how do I tell which post-migration records have to move with a sealed region (enumerate both strata for every route on that part; any leaf with a declaration in each is superseded and must travel)"
  - "why was moving only the region the wrong remedy even though LIVE-DOCUMENT-PRESSURE-HEADROOM.30b did exactly that (the region .30b moved superseded nothing, so it never met this)"
  - "how much writable budget does relocating a sealed region actually return (the active part stops owning any region, so its whole health target becomes spendable rather than the target minus the sealed payload)"
date: 2026-09-17
status: current
tags: [live-document-size, task-tree, doctrine, partition, two-strata, live-document-pressure-headroom]
evidence: scripts/check_active_task_evidence.pl (declared_node_statuses, strip_marked_regions, validate_route_lifecycles); docs/tasks/live-document-pressure-headroom/current-and-open-work.md (.33); docs/tasks/live-document-pressure-headroom/toolbox-and-census.md
reverify: "perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/pressure_headroom_task_evidence.json --check"
---

`LIVE-DOCUMENT-PRESSURE-HEADROOM.30a` lets a migrated tree close a leaf its own migration sealed as open: a
part is read as two strata, the sealed payload is pre-migration history and everything outside the markers
is current state, and the post-migration declaration wins. The rule is what makes a partitioned tree
workable at all.

**It is scoped to one part file, and nothing said so.** `validate_route_lifecycles` resolves a route
against its *primary part* and reads both strata out of that single file. So when a leaf is declared
`pending` inside the sealed payload and `done` outside it, the two declarations are only connected by
living in the same file.

## What goes wrong

Relocating a sealed region to give an active part its budget back is a compliant, proven remedy — `.30b`
did it. But the region's payload is also where the `- ID:` declarations of its leaves live, so a
`legacy`/`structural` route must follow its payload: the gate refuses `source literal … is absent from its
primary part payload` otherwise. Follow it, and a superseded leaf's sealed `pending` is now the only
declaration in its primary part. The lifecycle re-derives from it and the gate refuses the `closed` route
it had just proved correct — a remedy that breaks the thing it was protecting.

`.30b` never met this because the region it moved superseded nothing.

## The rule

**Move the region and every post-migration record that supersedes a declaration inside it, together.**
Find them by enumerating *both* strata for every route on the part before touching anything; any leaf with
a declaration in each stratum is superseded and has to travel. On `2026-09-17` that was three of the seven
leaves on `toolbox-and-census-nodes` — `.27a`, `.29b` and `.28` — which moved under the new part's own
`## Post-migration closures` heading.

The result is also the better shape on its own terms: the new part becomes the complete record of one
concern instead of half of it, and the active part stops owning a region at all, so its entire health
target becomes spendable rather than the target minus a sealed payload it can never reclaim. Measured:
the writable budget went `37,041 -> 58,982` bytes and its usage `95.2% -> 40.9%`, roughly nine closure
records of room where there had not been one.

Links: [[node-status-has-two-declaration-shapes]],
[[partitioning-a-task-tree-has-a-fixed-registration-price]], [[live-surface-edit-bookkeeping-chain]].
