---
id: fsmgen-feedback-channel
title: FSMGen feedback uses a bounded current channel and exact correspondence history
answers:
  - "where do I log feedback or a suggestion to FSMGen"
  - "where is the SpecForge FSMGen feedback or handoff channel"
  - "how do I file an FSMGen bug report or feature request"
  - "where did SpecForge suggest LTL/MTL support in ISF"
  - "what is the FSMGen issue bundle protocol"
  - "how is the FSMGen feedback channel kept bounded without losing old requests and responses"
  - "are any SpecForge requests to FSMGen currently open"
  - "what is the exact pre-containment FSMGEN_FEEDBACK source identity"
date: 2026-06-04
tags: [fsmgen, isf, feedback, handoff]
evidence: docs/FSMGEN_FEEDBACK.md; doctrine/live_document_size/fsmgen_feedback.json; docs/decisions/0011-bounded-fsmgen-feedback-channel.md; subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md; docs/fsmgen-issues/
reverify: perl scripts/check_fsmgen_feedback_protocol.pl --report
---

`docs/FSMGEN_FEEDBACK.md` is SpecForge's **stable, tracked feedback channel to FSMGen** — the
one document FSMGen reads to find SpecForge's questions, answers, suggestions, feature requests, and
pointers to filed bug bundles. FSMGen replies in
`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`; the current downstream boundary is strictly
`SpecForge IntentIR → .isf → FSMGen`.

For reproducible **bugs** (not suggestions), the formal mechanism is the issue-bundle protocol
`subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md` (`bin/fsmgen-issue-bundle`), with bundles
stored under `docs/fsmgen-issues/` and indexed from `FSMGEN_FEEDBACK.md`. SpecForge **never
patches the `subs/fsmgen` submodule** — it files forward reports. Related task-trees:
`FSMGEN-LTL-MTL-SUGGESTION`, `FSMGEN-ISSUE-REPORTING`. The deferred SpecForge-side alternative
to the LTL/MTL-in-ISF ask is `[[spec-mining-framing]]`'s sibling tree `TEMPORAL-RULE-SVA-RENDER`.

The sealed source capsule is exactly 936 lines / 57,980 bytes / 1,705 maximum line bytes with SHA-256
`5bf938d54559d1f80c5aed4d3a72a1ad3ec5792c6991b846beffb35d42739f03`. It contains five directed
exchanges and one two-bug episode; all six are closed with direct response or resolution evidence, so
the open-record count is zero. The capsule preserves a 456-line legacy primer whose `.fsm` adapter and
`030f8c273` “latest” claims are historical; current truth is `.isf`-only at pinned FSMGen `d327129b7`.

ADR 0011 keeps the stable root as an 81-line / 5,243-byte bounded current channel over the exact
immutable capsule plus bounded archive index/manifest. Its open region is empty at the sealed boundary;
its six-row closed register routes every exchange directly to complete history and independent closure
evidence. The executable migrated contract pins five exhaustive source regions, six record identities/
statuses/directions/evidence routes, 26 consumers, both stale-current findings, open-record fields and
limits, current-root bounds, and archive topology. It authenticates the capsule and validates the live
open region, concise closed register, manifest, index, and direct retrieval on every doctrine run.
