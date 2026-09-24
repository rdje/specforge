# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-09-24 02:10 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.7`) — removed 22 superseded rustc incremental
  sessions (2.70 GiB; `target/debug/incremental` 9.0 -> 7.1 GiB); no unreachable `.project-data/tmp/` root and no
  `.bin`/`.log` in `target/release`, `target/debug/incremental` or `target/debug/deps`.
