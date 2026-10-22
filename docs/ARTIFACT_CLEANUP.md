# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-22 07:25 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.17`) — removed 118 paths / 3,237 files /
  22,556.8 MiB of unreachable scratch under `.project-data/tmp/` (logs and scripts kept; three in-flight roots
  retained; re-plan empty); removed 22 superseded rustc incremental sessions with 47 orphan locks
  (`target/debug/incremental` 13.2 -> 10.3 GiB); removed 196 fixture roots a killed gate run left under
  `generated/`. The 33 dot-prefixed fixture roots under `generated/` are `SCRATCH-RESIDUE-CONTAINMENT.15`'s, still
  pending.
