# CORPUS-CHAIN-CURRENCY: prove, not assume, that every persisted chain matches the current binary

## Metadata

- Tree ID: `CORPUS-CHAIN-CURRENCY`
- Status: `active`
- Roadmap lane: `R15e`/`R16` corpus digestion (sibling of `CORPUS-COVERAGE`)
- Created: `2026-08-10`
- Last updated: `2026-08-10`
- Owner: repo-local workflow

## Goal

Make "the persisted corpus chain reflects the current binary" a **measured, gated** property instead of an
assumption. `CORPUS-COVERAGE`'s stated goal is to keep completed chains non-stale, and its `.1` stage-staleness
lane is marked complete — but nothing re-checks currency after a shared extractor changes. A repair leaf rebuilds
only the documents its own change touches, so every other completed document silently drifts one code delta at a
time.

## Non-Goals

- Do not re-ingest documents; currency measurement must be read-only or use `--dry-run` replay only.
- Do not silently refresh a document outside its owning `CORPUS-COVERAGE` leaf.
- Do not treat a non-rebuildable document (no normalized bundle) as current merely because it cannot be replayed.

## Reproduction and measurement (`2026-08-10`)

Discovered while completing `CORPUS-COVERAGE.2.50a`. Rebuilding the three documents that repair affected showed
their pre-existing chains were not reproducible by the **pre-repair** binary, so the rebuild also carried every
delta accumulated since each document's own refresh. Measured for USB 3.2 (`refresh #33`): timing constraints
102 → 74, actors 18 → 16, behaviors 2,705 → 866, all before the `.2.50a` subtraction.

A read-only `evidence --dry-run` replay of every rebuildable document against its persisted artifact then measured
the standing drift:

| Population | Count | Result |
| --- | ---: | --- |
| Rebuildable documents (normalized bundle present) | 22 | replayable read-only |
| Persisted chain already current | 19 | no section differs |
| Persisted chain would change on rebuild | 3 | timing surfaces only |
| Non-rebuildable documents | 58 | **not measurable** without re-ingest |

The three drifted documents are `ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification`
(timings 6 → 0), `opencapi_25gbps_phy_signaling_spec_1_0` (70 → 55), and `opencapi_discovery_configuration_v201`
(6 → 0) — residue of the `.2.47a`/`.2.48a` timing-authority repairs, which rebuilt only the documents they had
measured as affected.

The first of those is the **SWD wire gold document**, so the WIRE-BASED-100 gold eval currently scores an artifact
the current binary would not produce. That does not invalidate the recorded gold numbers — the eval is
artifact-pinned by design — but it means "the gold is 1.000" and "the current code scores 1.000" are two claims,
and only the first is presently gated.

## Open questions for the director

1. Should a repair leaf rebuild **every** rebuildable affected document, or only those its own measurement names?
   The former closes drift; the latter keeps a repair's blast radius small and its evidence sharp.
2. Should chain currency become a doctrine check (`check_chain_currency.sh`) that fails when a rebuildable
   document's persisted chain differs from a `--dry-run` replay, or stay an advisory report?
3. The 58 non-rebuildable documents cannot be measured at all without re-ingest. Is retaining normalized bundles
   for the whole corpus worth its disk cost, or is per-document refresh the only intended currency mechanism?

## Frontier

`.0` — decide the three questions above with the director, then either write the currency checker or record an
explicit decision record accepting bounded drift between refreshes.

## Verification Log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | read-only currency census | 22 rebuildable documents replayed; 19 current, three drifted on timing surfaces; 58 unmeasurable |

## Commit Log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-CHAIN-CURRENCY` ownership | `CORPUS-CHAIN-CURRENCY — track measured persisted-chain currency` |
