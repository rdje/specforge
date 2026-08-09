# LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION archive

This bounded index keeps the stable closed task summary separate from the exact source committed before the
terminal migration.

## Sealed source

- [Bounded current task summary](../../../tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md)
- [Exact pre-migration task source](source-through-2026-08-09.md)
- [Identity and provenance manifest](manifest.json)

The source capsule is immutable. It contains every detailed activity, decision, acceptance checklist,
verification row, commit record, and changelog entry through `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.10b.i`.
The bounded current root records the terminal migration and final program closure.

## Verification

From the repository root, run:

```sh
perl scripts/check_task_tree_archive.pl --check
```
