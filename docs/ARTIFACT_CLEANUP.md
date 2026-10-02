# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-03 01:27 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.13`) — removed 143.8 MiB from 42 unreachable
  scratch roots (data subdirectories and old source/backup copies; logs and scripts kept) and 22 superseded rustc
  incremental sessions with 44 orphan locks (`target/debug/incremental` 11.7 -> 9.0 GiB); no `.bin`/`.log` in
  `target/release` or `target/debug/deps`.
