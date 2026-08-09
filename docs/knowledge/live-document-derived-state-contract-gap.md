---
id: live-document-derived-state-contract-gap
title: Derived-state field and copy plane is independently closed
answers:
  - "did SpecForge adopt the 2026 08 09 FSMGen derived state containment revision"
  - "which exact current state copies are not yet independently verified in SpecForge"
  - "how should Rust version copies be verified across Cargo README book and CI"
  - "how should the current FSMGen gitlink in documentation be verified"
  - "why does the feedback protocol self test block derived state closure"
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

The three `.8a` value seams currently agree. Task-scoped corpus/cache counts remain outside the resume pointer;
`MEMORY.md` exposes `git rev-parse HEAD` and forbids a self-invalidating latest-commit shadow. Workspace
`rust-version` is normalized to patch form and compared with README, mdBook, and CI. The feedback Markdown and
JSON pin copies are compared with the stage-zero mode-`160000` Git-index object for `subs/fsmgen`.

`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.8b` implements the bounded local adoption. The neutral checker executes
eight existing projection/currentness verifiers plus the two declared local adapter contracts. Existing
generators remain authoritative and no ceiling is widened.

The `.8c` audit found that agreement is not yet complete data-plane ownership. The registry declares the README
Rust marker and feedback Markdown hash, while the adapter hardcodes the mdBook/CI Rust paths and feedback JSON
pin. Those comparisons and all 16 mutations pass, but a data-only audit cannot enumerate the complete copy set
without reading implementation source. `.8` therefore remains open: `.8d` owns bounded secondary-copy
declarations and neutral path/marker validation, and `.8e` independently re-audits before closure.

`.8d` now owns the complete copy membership in data. The Rust record declares mdBook Rust as a governed surface
copy and CI Rust as a control copy; the FSMGen record declares the feedback JSON pin as a control copy. The
registry remains 14 primary contracts and adds exactly three nested secondary entries. The neutral checker
validates closed roles/ownership, safe same-volume regular paths, exact markers, and current surface membership;
the adapter source contains none of the three paths and consumes only required declared roles. Forty-seven
neutral plus 25 adapter cases include alternate declared paths, missing/unknown/duplicate roles, marker drift,
unsafe paths, and surface/control misuse.

The `.8e` cold audit confirms every one of those 17 data members and all 72 mutations, but refuses closure on a
different stored value. `scripts/check_fsmgen_feedback_protocol.pl` embeds the live FSMGen hash while rendering
its synthetic self-test current root, even though that root is validated against the contract's declared
`current_root.required_literals`. This is update-coupled project-current executable state, not an independent
fixture or historical measurement. `.8f` must derive all rendered literals from contract data and prove a changed
pin has no fallback; `.8g` then repeats the repository-wide current-value scan before `.8` may close.

`.8f` removes that executable copy. The renderer now expands the full declared `required_literals` array and has
no live hash, literal role, positional selection, or shape inference. An eleventh self-test replaces both array
members with unrelated fixture strings before rendering; it passes together with the real protocol check/report,
all 72 derived-state mutations, and full CI. `.8g` remains responsible for the independent resulting-tree scan
and closure decision.

The `.8g` resulting-tree audit is clean. All 14 primaries plus three secondaries occur exactly once and agree with
their Cargo or Git-index authorities; no live FSMGen value remains in executable source. Historical occurrences
are revision evidence, while Rust values in the adapter suite are self-contained temporary contracts with no
repository-authority coupling. All 83 focused cases and full CI pass, so `.8` is closed and `.9a` becomes the
next containment frontier.

The card/index plane remains an explicit implementation constraint. `.8b` updates this same card rather than
adding another; any future crossing of the rollover threshold requires an owned partition, not a wider ceiling.
