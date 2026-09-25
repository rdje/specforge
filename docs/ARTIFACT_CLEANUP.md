# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-09-25 16:55 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.8`) — removed 49 superseded rustc incremental
  sessions and 75 orphan locks (`target/debug/incremental` 13.3 -> 8.7 GiB); 2.0 GiB of unreachable scratch left
  for `.9` (removal refused by the permission policy); no `.bin`/`.log` in `target/release` or `target/debug/deps`.
