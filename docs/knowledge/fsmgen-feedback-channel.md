---
id: fsmgen-feedback-channel
title: SpecForge -> FSMGen feedback channel is docs/FSMGEN_FEEDBACK.md (+ issue bundles for bugs)
answers:
  - "where do I log feedback or a suggestion to FSMGen"
  - "where is the SpecForge FSMGen feedback or handoff channel"
  - "how do I file an FSMGen bug report or feature request"
  - "where did SpecForge suggest LTL/MTL support in ISF"
  - "what is the FSMGen issue bundle protocol"
date: 2026-06-04
tags: [fsmgen, isf, feedback, handoff]
evidence: docs/FSMGEN_FEEDBACK.md; subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md; docs/fsmgen-issues/
reverify: ls docs/FSMGEN_FEEDBACK.md
---

`docs/FSMGEN_FEEDBACK.md` is SpecForge's **stable, tracked feedback channel to FSMGen** — the
one document FSMGen reads to find SpecForge's suggestions and feature asks (IntentIR-aligned
`.fsm`/ISF features, temporal/stability contracts, the **2026-06-04 suggestion to add
first-class LTL/MTL temporal properties to ISF with a proposed ISF format**), clarity
requests, and pointers to filed bug bundles. FSMGen replies in
`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`.

For reproducible **bugs** (not suggestions), the formal mechanism is the issue-bundle protocol
`subs/fsmgen/docs/DOWNSTREAM_ISSUE_REPORTING.md` (`bin/fsmgen-issue-bundle`), with bundles
stored under `docs/fsmgen-issues/` and indexed from `FSMGEN_FEEDBACK.md`. SpecForge **never
patches the `subs/fsmgen` submodule** — it files forward reports. Related task-trees:
`FSMGEN-LTL-MTL-SUGGESTION`, `FSMGEN-ISSUE-REPORTING`. The deferred SpecForge-side alternative
to the LTL/MTL-in-ISF ask is `[[spec-mining-framing]]`'s sibling tree `TEMPORAL-RULE-SVA-RENDER`.
