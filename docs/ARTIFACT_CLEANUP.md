# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-23 07:42 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.18`) — removed 50 paths / 1,093 files /
  8,626.9 MiB of scratch under `.project-data/tmp/` (unreachable roots' data, plus the committed slice snapshots in
  the one in-flight root; logs and scripts kept; re-plan empty); removed 19 superseded rustc incremental sessions with
  53 orphan locks (`target/debug/incremental` 13.1 -> 10.4 GiB). No fixture residue under `generated/`; the 33
  dot-prefixed fixture roots there are `SCRATCH-RESIDUE-CONTAINMENT.15`'s, still pending.
