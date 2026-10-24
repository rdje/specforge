# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-24 07:42 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.20`) — removed 9 paths / 295 files /
  2,115.7 MiB of scratch under `.project-data/tmp/` (a superseded seed baseline and the snapshots of two committed
  slices; the HEAD seed baseline `erg-67a/seeds_new` protected; logs and scripts kept; re-plan empty); removed 57
  superseded rustc incremental sessions with 126 orphan locks (`target/debug/incremental` 13.2 -> 11.3 GiB); removed
  three fixture roots a killed run left under `generated/`. The 33 dot-prefixed fixture roots there are
  `SCRATCH-RESIDUE-CONTAINMENT.15`'s, still pending.
