# Roadmap archive

[`ROADMAP.md`](../../../ROADMAP.md) is the bounded current-direction and workstream-status view.
This archive retains the exact source that preceded its first containment migration, plus every
bounded root a later rollover retired.

## Sealed source

- [Exact source capsule](source-through-2026-08-08.md)
- [Identity and verification manifest](manifest.json)

The capsule is immutable. Its manifest records the source digest and exact size.

## Sealed root rollovers

A rollover replaces the bounded current root and seals the exact bytes it retired, oldest first. Each
capsule's identity is declared in `doctrine/live_document_size/roadmap_projection.json` and re-proved
on every gate run, so recovering a retired direction never depends on Git object reachability.

| Sealed | Retired bounded root | Lines | Bytes |
| --- | --- | ---: | ---: |
| `2026-08-11` | [root through 2026-08-11](root-through-2026-08-11.md) | 364 | 34,938 |

The series is bounded at 16 capsules and warns at 12. Its remedy at that point is to seal the series
into one dated deep-archive capsule, reset the series, and record the new identity in the contract.

## Verification

From the repository root, run:

```sh
perl scripts/check_roadmap_projection_contract.pl --check
```
