# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-09-28 11:08 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.11`) — removed 632.8 MiB from 23 unreachable
  scratch roots (data subdirectories and old executable/source copies; logs and scripts kept) and 22 superseded rustc
  incremental sessions with 44 orphan locks (`target/debug/incremental` 11.0 -> 8.8 GiB), then, with the owner's
  permission, `erg7`'s 2,028.6 MiB of unreachable data (`.9`); no `.bin`/`.log` in `target/release` or `target/debug/deps`.
