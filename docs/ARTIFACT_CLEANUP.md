# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-28 21:21 +0100` (`SCRATCH-RESIDUE-CONTAINMENT.22`) — removed 12 reports from the superseded
  seed baseline, 38 superseded rustc incremental sessions and 76 orphan locks (`target/debug/incremental`
  15,208,344 -> 12,468,584 KiB, 2,739,760 KiB freed; both re-plans empty). No killed-run fixture residue; the 33
  dot-prefixed fixture roots under `generated/` are `SCRATCH-RESIDUE-CONTAINMENT.15`'s, still pending.
