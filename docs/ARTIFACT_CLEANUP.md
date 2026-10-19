# Artifact cleanup — last run

> Overwrite-only; keep exactly one entry. A session reads the date below at startup and runs a sweep when it is
> more than 24 hours old or this file is missing. The sweep's rules and history are owned by
> [`SCRATCH-RESIDUE-CONTAINMENT`](tasks/SCRATCH-RESIDUE-CONTAINMENT.md).

- Last cleanup: `2026-10-19 22:59 +0200` (`SCRATCH-RESIDUE-CONTAINMENT.14`) — no unreachable scratch had accumulated
  under `.project-data/tmp/` (42 unreachable roots, plan empty) and no fixture residue under `generated/`; removed 18
  superseded rustc incremental sessions with 36 orphan locks (`target/debug/incremental` 10.6 -> 9.0 GiB); no
  `.bin`/`.log` in `target/release` or `target/debug/deps`.
