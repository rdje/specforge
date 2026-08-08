# Roadmap archive

[`ROADMAP.md`](../../../ROADMAP.md) is the bounded current-direction and workstream-status view.
This archive retains the exact source that preceded its first containment migration.

## Sealed source

- [Exact source capsule](source-through-2026-08-08.md)
- [Identity and verification manifest](manifest.json)

The capsule is immutable. Its manifest records the source digest and exact size, while Git preserves
later changes to the bounded current roadmap.

## Verification

From the repository root, run:

```sh
perl scripts/check_roadmap_projection_contract.pl --check
```
