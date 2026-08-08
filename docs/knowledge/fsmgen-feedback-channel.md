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

At the containment boundary the source is exactly 936 lines / 57,980 bytes / 1,705 maximum line
bytes with SHA-256 `5bf938d54559d1f80c5aed4d3a72a1ad3ec5792c6991b846beffb35d42739f03`.
It contains five directed exchanges and one two-bug episode; all six are closed with direct response
or resolution evidence, so the open-record count is zero. A 456-line legacy primer still says `.fsm`
is a SpecForge adapter and calls `030f8c273` the latest response pin, while current truth is `.isf`-
only at pinned FSMGen `d327129b7`.

ADR 0011 therefore keeps the stable root as a bounded current channel and requires an exact immutable
source capsule plus bounded archive index/manifest before migration. The executable contract pins five
exhaustive regions, six record identities/statuses/directions/evidence routes, 26 consumers, both stale-
current findings, open-record fields and limits, current-root bounds, and archive topology. In planned
state it freezes the source; migrated state will authenticate the capsule and validate the live open
region, concise closed register, and direct retrieval.
