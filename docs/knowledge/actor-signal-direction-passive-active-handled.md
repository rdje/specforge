---
id: actor-signal-direction-passive-active-handled
title: Actor-signal drive/read DIRECTION is already correct in production (voice-separated verb lexicon); the prose "to/recipient" frame is negative-EV → NLP-SHALLOW-PARSE.2h measured NO-GO
answers:
  - "does specforge handle passive voice for actor-signal relations (X is driven by Y)"
  - "is the drive/read direction correct for passive vs active prose relations"
  - "should NLP-SHALLOW-PARSE.2h add new passive/verb-sense direction code"
  - "why was the spike's 'manager Reads ARID' direction error not in production"
  - "should specforge recover consumer edges from 'X is sent/returned to Y' recipient frames"
  - "where is actor-signal relation drive/read direction decided in evidence.rs"
date: 2026-06-15
tags: [nlp-shallow-parse, actor-signal-relations, direction, passive-voice, verb-lexicon, measured-defer, grounding-gate, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (extract_actor_signal_relations, ~L2822-3039 — passive/active × drives/reads patterns); crates/specforge/src/ir/normative_vocab.rs (PASSIVE_DRIVES_VERBS / ACTIVE_DRIVES_VERBS / PASSIVE_READS_VERBS / ACTIVE_READS_VERBS); docs/tasks/NLP-SHALLOW-PARSE.md (.2h)
reverify: "grep -roh ' is sent to \\| is returned to ' generated/evidence_ir/*/evidence_ir.json | wc -l (~150) — but the SUBJECTS are event/notification/response/request/Snoop/MSI (messages, not declared signals), so the signal-subject grounding gate rejects them; 'is driven to' is value-dominant (zero / output pin / RSP); wire docs contain the frame (AXI=10, AHB=1) so a 'to' frame is NOT additive-safe vs the wire-based-100% gold gate"
---

**`NLP-SHALLOW-PARSE.2h` is a measured NO-GO as NEW production code (`2026-06-15`).** The leaf was scoped
("passive + verb sense — fixes the spike's direction errors, e.g. AXI `manager Reads ARID` when ARID is
manager-DRIVEN") on the assumption that production has a drive/read direction bug. **Measurement overturned
that premise.**

**1. Production already gets direction right — by design, not by luck.** `extract_actor_signal_relations`
(`evidence.rs`) matches FOUR voice-separated patterns against the centralized lexicon (`normative_vocab.rs`):
passive drive `"{sig} is {verb} by|from {actor}"` (PASSIVE_DRIVES_VERBS → `Drives`), passive read (PASSIVE_READS_VERBS
→ `Reads`), active drive `"{actor} {verb} {sig}"` (ACTIVE_DRIVES_VERBS → `Drives`, with `active_object_contains_signal`
checking the signal is in the object clause AFTER the verb), and active read (ACTIVE_READS_VERBS → `Reads`). The
active and passive verb lists are disjoint by inflection (`driven`/`drives`, `sampled`/`samples`), so a passive
participle never fires an active pattern and vice-versa. The spike's `manager Reads ARID` error was a property of
the **throwaway generic-SVO prototype** (naive subject-verb-object with no verb-sense), NOT this hand grammar.
There is no demonstrable production direction bug to fix.

**2. The only prose delta — the "to/recipient" transfer frame — is negative-EV.** Production handles `by`/`from`
(agent) but not `to` (recipient). Measured over the 78 persisted `evidence_ir.json`:
- `"to {recipient}"` frames are common lexically (~255: `is sent to` 97, `is provided to` 54, `is returned to`
  47, `is presented to` 31, `is driven to` 14, …) — MORE than the handled by/from frames.
- **But grounded yield is ≈0:** the SUBJECTS of `is sent to` / `is returned to` are overwhelmingly messages/
  transactions (`event` 24, `notification` 19, `response`, `request`, `Snoop`, `MSI`, `interrupt`, `command`),
  not declared wire signals — so the signal-subject grounding gate (ADR 0006) rejects essentially all of them.
- **Precision-fraught:** `is driven to` is value-dominant (`driven to zero`, `to the output pin`, `to RSP`), so a
  "to" frame must exclude value/level objects; recipients are document-specific node names (RN/SN/HN/PE/hart —
  ADR 0006, not hardcodable), and `extract_actor_phrase` does not actor-ground (it grabs 1-2 words after the prep).
- **Wire-doc risk:** AXI (10) and AHB (1) contain the frame, so adding it is NOT byte-identical-safe and would
  put the wire-based-100% gold gate at risk for ≈0 real benefit.
- It **contradicts the spike's own measured guidance**: free-prose SVO is the low-yield area (recall lives in
  TABLES + the VLM arm); `.2h` was a *precision* play, and the recipient frame is *recall*, not a direction fix.

**Decision:** do NOT add new `.2h` direction code (would be duplicate-of-existing or negative-EV / gate-risking).
Frontier advances to `.2f` (coordination distribution) — the genuine prose gap the spike isolated — measured-first
before any build. Mirrors the project's measured-DEFER precedents [[source-ir-size-scaling]] (MEMORY-BOUNDED-INGEST.5)
and FULL-PAGE-INTENT-CAPTURE.1 NO-GO. Honors [[feedback_scoring_rigor]] (measured, not assumed) and
[[feedback_avoid_denylists_prefer_structural]] / ADR 0006 (the recipient names are document-specific, not a fixed list).
