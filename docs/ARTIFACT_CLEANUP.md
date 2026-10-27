# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-27 16:47 +0100` (`SCRATCH-RESIDUE-CONTAINMENT.21`) — no unreachable scratch or
  killed-run fixture residue to remove; removed 3 superseded rustc incremental sessions with 12 orphan locks
  (`target/debug/incremental` 12,010,128 -> 11,907,228 KiB, 102,900 KiB freed; re-plans empty). The 33
  dot-prefixed fixture roots under `generated/` are `SCRATCH-RESIDUE-CONTAINMENT.15`'s, still pending.
