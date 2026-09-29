# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-09-29 11:40 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.12`) — removed 926.2 MiB from 31 unreachable
  scratch roots (data subdirectories and old executable/source copies; logs and scripts kept) and 25 superseded rustc
  incremental sessions with 57 orphan locks (`target/debug/incremental` 11.0 -> 8.9 GiB); no `.bin`/`.log` in
  `target/release` or `target/debug/deps`.
