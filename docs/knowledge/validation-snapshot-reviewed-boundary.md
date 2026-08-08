---
id: validation-snapshot-reviewed-boundary
title: VALIDATION_SNAPSHOT is the last reviewed projection, not ambient local artifact state
answers:
  - "is VALIDATION_SNAPSHOT the latest local artifact validation or the last reviewed projection"
  - "what keeps VALIDATION_SNAPSHOT current without mutating generated artifacts"
  - "why does the tracked validation snapshot differ from current generated IntentIR reports"
  - "what is the executable currentness authority for VALIDATION_SNAPSHOT"
date: 2026-08-08
status: current
tags: [validation, projection, review, currentness]
evidence: doctrine/live_document_size/validation_snapshot.json; docs/decisions/0012-reviewed-validation-snapshot-boundary.md; docs/tasks/CANONICAL-PROMOTION-SWEEP.md
reverify: perl scripts/check_validation_snapshot_currentness.pl --report
---

`VALIDATION_SNAPSHOT.md` is the tracked **last reviewed** projection. It is not a promise that the
newest git-ignored artifact on one workstation has been approved as validated state. The reviewed
boundary remains the four-report, 29-recommendation projection from commit `a44323d5`; later canonical
promotion work explicitly withheld unreviewed local scores from this tracked surface.

The distinction is material. The four current local IntentIR files contain zero embedded validation
reports, and AHB's sidecar now says fingerprint `8b5311edb20f7998`, score `63`, while the reviewed
snapshot says `f2b3e0591b4a5fbc`, score `65`. That local delta is not silently accepted or discarded:
it remains outside validated-state authority until an explicit reviewed refresh.

`doctrine/live_document_size/validation_snapshot.json` declares the reviewed source commit, exact
report identities and counts, snapshot/live-projection identities, review evidence, and the Rust
producer regions. `scripts/check_validation_snapshot_currentness.pl` validates those semantics and
identities without reading or mutating ambient generated artifacts, so the same check works in a fresh
clone. A future approved refresh must update the declaration, snapshot, live block, and producer
contract atomically.
