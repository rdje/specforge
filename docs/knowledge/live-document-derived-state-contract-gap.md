---
id: live-document-derived-state-contract-gap
title: SpecForge enforces explicit field-level derived-state contracts
answers:
  - "did SpecForge adopt the 2026 08 09 FSMGen derived state containment revision"
  - "which exact current state copies are not yet independently verified in SpecForge"
  - "how should Rust version copies be verified across Cargo README book and CI"
  - "how should the current FSMGen gitlink in documentation be verified"
  - "why should corpus counts leave MEMORY md"
  - "what does LIVE DOCUMENT SIZE CONTAINMENT ADOPTION 8 implement"
date: 2026-08-09
status: current
tags: [documentation, live-document-containment, derived-state, currentness, memory, gitlink, rust-toolchain]
evidence: docs/research/live-document-derived-state-adoption-delta.md; docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md (.8); LIVE_DOCUMENT_SIZE_CONTAINMENT.md; doctrine/live_document_size/derived_state_contracts.jsonl; scripts/check_derived_state_contracts.pl; scripts/check_derived_state_authorities.pl
reverify: perl scripts/check_derived_state_contracts.pl --report
---

SpecForge's `2026-08-08` containment adoption already had 41 governed surfaces, eight enforced currency contracts,
15 non-budget lifecycle verifiers, bounded generated projections, lossless rolling ledgers, and same-volume
locality. The deliberate `2026-08-09` revision now adds the missing exact-field plane: 14 bounded contracts classify
one derive-on-read field, two authored-intent regions, one immutable-evidence region, and ten verified copies.
Field discovery is literal and declared; dates, numbers, hash shapes, and prose are never scanned heuristically.

The three `.8a` seams are closed. Task-scoped corpus/cache counts remain outside the resume pointer;
`MEMORY.md` exposes `git rev-parse HEAD` and forbids a self-invalidating latest-commit shadow. Workspace
`rust-version` is normalized to patch form and compared with README, mdBook, and CI. The feedback Markdown and
JSON pin copies are compared with the stage-zero mode-`160000` Git-index object for `subs/fsmgen`.

`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8b` implements the bounded local adoption. The neutral checker executes
eight existing projection/currentness verifiers plus the two declared local adapter contracts. Forty-four focused
cases reject stored derive-on-read values, missing/off-surface/duplicate markers, invalid classes, absent or failing
verifiers, malformed capture boundaries, registry displacement, Cargo-copy drift, and gitlink mode/object drift.
Existing generators remain authoritative, `.8c` independently audits the result, and no ceiling is widened.

The card/index plane remains an explicit implementation constraint. `.8b` updates this same card rather than
adding another; any future crossing of the rollover threshold requires an owned partition, not a wider ceiling.
