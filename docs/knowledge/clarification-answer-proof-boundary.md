---
id: clarification-answer-proof-boundary
title: Clarification answers are untrusted envelopes and enter proof by authority kind
answers:
  - "Where is the executable schema for SpecForge clarification packets and answers?"
  - "Which Rust module implements clarification definition hashes and lifecycle state?"
  - "What does current clarification-envelope compatibility mean?"
  - "Why must a source-locator answer resolve back to native captured evidence?"
  - "Which locality rule applies to a supplemental clarification source?"
  - "Which task leaves still own clarification validation and minimal replay?"
  - "What explicit non-answer dispositions does the clarification IR support?"
  - "How can a question status change without invalidating its issued definition hash?"
date: 2026-08-15
status: current
tags: [clarification, answer, authority, proof, currentness]
evidence: crates/specforge/src/ir/clarification.rs; docs/decisions/0040-clarification-answers-are-untrusted-evidence-envelopes.md; docs/book/src/pipeline/clarification-loop.md
reverify: cargo test -p specforge-core clarification --offline && scripts/check_production_genericity.sh
---

`SPEC-CLARIFICATION-LOOP.1` implements schema-1 `ClarificationPacket` and
`ClarificationAnswerEnvelope` in `specforge_core::ir::clarification`. Packets bind immutable,
digest-addressed question definitions and separate lifecycle state to exact repository-local source/current
artifacts and a proof ruleset. Answers bind typed values or explicit non-answers to the exact question definition,
authority policy, responder/channel provenance, and answer supersession chain.

The serialized answer is never a proof witness. Current compatibility means “requires validation”; legacy is
inspection-only, while future, malformed, missing-version, unknown-field, stale-question, and stale-policy forms
reject. Compatibility always reports `permits_canonical_authority() == false`.

Authority is specific: a source locator must resolve to normal captured source/table/visual proof; a source
supplement must first become a governed repository-local source artifact; a genuine external design choice may
later enter only through a dedicated narrowly registered premise scoped to configurable surfaces. There is no
generic `UserAnswer` premise, because that would erase whether the PDF supplied a fact or a designer selected an
allowed implementation choice. Semantic value/grounding/authorization/conflict validation and proof replay remain
owned by `.4` and `.5`; the implemented `.1` foundation does not claim those runtime behaviors yet.
