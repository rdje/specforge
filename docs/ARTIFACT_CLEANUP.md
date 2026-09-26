# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-09-26 17:03 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.10`) — removed 10.2 GiB of per-slice snapshot data
  from 19 unreachable lane roots (logs and scripts kept) and 39 superseded rustc incremental sessions with 150 orphan
  locks (`target/debug/incremental` 12.8 -> 8.8 GiB); `erg7`'s 2.0 GiB still waits for the owner (`.9`); no `.bin`/`.log`
  in `target/release` or `target/debug/deps`.
