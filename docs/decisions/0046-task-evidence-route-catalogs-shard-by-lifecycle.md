---
id: task-evidence-route-catalogs-shard-by-lifecycle
title: Task-evidence route catalogs shard by lifecycle
date: 2026-08-31
status: accepted
scope: live-documents, task-trees, task-evidence, cardinality, capacity, routing, doctrine
evidence: doctrine/live_document_size/spec_to_intent_task_evidence.json; doctrine/live_document_size/surfaces.jsonl; scripts/check_active_task_evidence.pl; docs/tasks/spec-to-intent-alignment/INDEX.md; docs/tasks/spec-to-intent-alignment/routes-0001.md; docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md
reverify: "bash scripts/check_task_evidence_contracts.sh; perl scripts/check_active_task_evidence.pl --self-test; bash scripts/check_live_document_size.sh"
answers:
  - "what is ADR 0046"
  - "why does a task-evidence index only list open leaves"
  - "what is route_catalog_state"
  - "what is a route catalog part"
  - "how do I regenerate a task-evidence index"
  - "why is docs/tasks/spec-to-intent-alignment/routes-0001.md auto-generated"
  - "what bounds the number of leaves a migrated task tree may declare"
  - "what is max_unverified_routes"
  - "how is a leaf route lifecycle verified"
---

# ADR 0046: Task-evidence route catalogs shard by lifecycle

## Context

Three task trees are migrated under ADR 0039 into the same contract shape: a bounded current root, semantic
evidence parts, a Markdown index, a manifest, and a byte-exact archived source capsule. `destinations` gives
the root a rollover route (migrate to a capsule and parts) and gives the parts one (split into another part).
It gives the **index** none.

`scripts/check_active_task_evidence.pl` required that index to route every declared leaf exactly once, so its
line count was a pure function of the tree's *lifetime* leaf count: a fixed preamble plus one row per leaf that
has ever existed. Measured at `37d6ab16` (`2026-08-31`), the alignment index was **115 of a 128-line health
target — 89.8%**, and `apply_live_pressure` refuses at the 90% mandatory-rollover milestone, which is 115.2
lines. The next leaf declaration was therefore a hard failure, and the product frontier
`SPEC-TO-INTENT-ALIGNMENT.9c` needs child leaves. The other two indexes sat at 67.2% (corpus-coverage, 86/128)
and 49.4% (pdf-variant-digestion, 79/160) of the same un-routed bound, so the shape — not the tree — is the
defect.

The composition says what the bound was actually measuring. Of the alignment tree's 83 route rows, **77 are
`done` and one `superseded`**, and **54 belong to lane `.6` alone**. The index was spending its budget on work
nothing will act on again, exactly as `docs/TASK_TREE.md` did before
`LIVE-DOCUMENT-PRESSURE-HEADROOM.2c` sharded it by lifecycle rather than by alphabet.

## Decision

Every contract declares `route_catalog_state`, and it is the only thing that decides the index's shape.

`inline` is the legacy shape: the landing routes every declared leaf, and `route_part_prefix`,
`limits.route_parts`, and per-route `lifecycle` are refused as unknown structure.

`sharded` adopts the `.2c` remedy:

1. Every leaf route declares `lifecycle: open | closed`. The landing carries the **open** routes only, so its
   size measures work in flight rather than project age.
2. `destinations.route_part_prefix` derives the route catalog part paths — `<prefix>0001.md`, `0002.md`, … —
   from `limits.route_parts.routes_per_part`. There is no hand-edited member list anywhere.
3. The parts carry **complete** membership, every leaf with its lifecycle and its primary detail part, so the
   whole route set still resolves for a reader in Markdown.
4. The landing and every route part are **derive-and-diff generated**. `--write` is the only writer; `--check`
   rejects any byte that the contract does not derive, and an unplanned `<prefix>*.md` file in the collection
   directory is refused by name.
5. Route part bounds are derived from the generator's own structure, not copied: a structurally full part is
   `routes_per_part` rows plus 10 fixed lines, and the health target is set so a full part sits *below* its own
   80% warning band. A full part must never be a bound with no compliant remedy.
6. A declared lifecycle is **cross-checked against the leaf's own evidence**. Where the primary semantic part
   declares the node (`- ID: \`<leaf>\`` followed by `State:` or `Status:`), `done` and `superseded` must agree
   with `closed` and everything else with `open`; disagreement is a violation, and so is the same leaf declared
   twice in one part. Ownership is read from the owner's own status line, never inferred from a mention — the
   rule `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` had to learn seven times.
7. A route whose primary part declares no node at all cannot be cross-checked. Those are counted and bounded by
   `limits.route_parts.max_unverified_routes`, pinned at today's exact number, so the population is explicit
   and can only shrink.

## Consequences

The alignment index is **115 → 43 lines, 33.6% of its health target**, and its growth now tracks the three open
leaves rather than the 83 declared ones. `SPEC-TO-INTENT-ALIGNMENT.9c` is unblocked.

**The nearest structural stop relocates, and this record says so rather than claiming a removal.** For a
migrated tree it is now `limits.manifest.max_leaf_routes`, which the portable cap fixes at 128; the alignment
tree declares 83, so 45 leaves remain. That is a bound with no declared rollover, the `LIVE-DOC-STOP-RISK`
shape, and it is owned by `LIVE-DOCUMENT-PRESSURE-HEADROOM.17` rather than left implied.

`pdf-variant-digestion` and `corpus-coverage` stay `inline`, and the reason is evidential, not scheduling: their
semantic parts record leaves as free prose, with no `- ID:`/`State:` node blocks, so rule 6 has no authority to
cross-check against. Declaring 52 and 56 lifecycles by hand would put an unverifiable claim on their landings.
Those trees adopt the shard once their parts carry node records — `LIVE-DOCUMENT-PRESSURE-HEADROOM.10` owns the
corpus half — and until then the staged state is declared in the contract, the way `migration_state` already
stages this same doctrine.

Two things the cross-check found on its first run, both recorded because a gate that finds nothing on adoption
has not been tested by anything:

- `SPEC-TO-INTENT-ALIGNMENT.8` was `active` in `residual-actionability.md` while all four of its children were
  `done` and the bounded root recorded `.0`–`.8` complete. The part is corrected to `done`; without the
  cross-check the new landing would have published a leaf as open that the root publishes as closed.
- `SPEC-TO-INTENT-ALIGNMENT.9a` is routed by the index and declared in the root's owner registry but has **no
  node record in any part** — it exists only as a section heading. It is the one uncorroborated route,
  `max_unverified_routes` is pinned at `1`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.16` writes the record.

The alignment `manifest.json` is normalized to the canonical form its writer emits; it had been reformatted
out of band since the 2026-08-14 migration, which is why a derived artifact could drift from its generator at
all.

## Links

- Owning task tree: [`docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md`](../tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md) (`.14`, `.14a`, `.16`, `.17`)
- ADR 0039 (bounded spec-to-intent task evidence)
- ADR 0045 (task-plane cardinality removed behind a declared exemption) — the same lifecycle cut, one surface up
- Doctrine: [`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`](../../LIVE_DOCUMENT_SIZE_CONTAINMENT.md)
- Reachable-stop precedent: [`docs/tasks/LIVE-DOC-STOP-RISK.md`](../tasks/LIVE-DOC-STOP-RISK.md)
