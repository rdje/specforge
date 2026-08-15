---
id: clarification-answer-authority
title: Clarification answers are untrusted evidence envelopes with authority-specific promotion
date: 2026-08-15
status: accepted
scope: clarification, user-assistance, evidence, proof-authority, lifecycle, currentness
evidence: crates/specforge/src/ir/clarification.rs; docs/tasks/SPEC-CLARIFICATION-LOOP.md; docs/book/src/pipeline/clarification-loop.md
reverify: cargo test -p specforge-core clarification --offline && scripts/check_production_genericity.sh
answers:
  - "Can a persisted clarification answer directly authorize canonical intent?"
  - "How does a source-locator answer enter SpecForge proof?"
  - "How does supplemental user information enter SpecForge proof?"
  - "How may an external design choice authorize generated ISF?"
  - "Why is there no generic UserAnswer proof premise?"
  - "What binds a clarification answer to the question that was actually asked?"
---

# ADR 0040: Clarification answers are untrusted evidence envelopes with authority-specific promotion

## Context

SpecForge must complete PDF-to-FSMGen-ISF autonomously wherever governed evidence suffices and ask the user only
where a premise remains genuinely unavailable. That feedback is useful only if it is both efficient and honest.
A vague chat answer is not durable, current, typed, or necessarily correct. Conversely, treating every response
as ordinary source evidence would let a user statement silently override the PDF and bypass ADR 0038's trusted
promotion kernel.

The answer may mean three materially different things:

1. “The information is already here; inspect this source location.”
2. “The current project source set is incomplete; here is additional authoritative material.”
3. “The source intentionally leaves this configurable; I am making the external design choice.”

Those cases cannot share one undifferentiated authority. Unknown, unavailable, not-applicable, and deferred are
also legitimate responses; none supplies a canonical premise.

## Decision

### 1. Packet definitions are immutable; lifecycle is a separate current projection

A schema-1 `ClarificationPacket` binds an ordered question set to exact repository-local source/current-artifact
digests and the current proof ruleset. Each question has an immutable definition—origin, missing-information
reason, exact evidence links, proposition, alternatives, downstream impact, priority/information gain,
dependencies, autonomous-continuation disposition, and answer schema. Answers reference the definition's SHA-256,
not mutable status prose. Lifecycle state is carried separately with a monotonic sequence, answer reference,
reason, and question supersession link.

Dependencies must resolve to exact earlier definitions in the same packet. This makes packets deterministic,
cycle-free by construction, and safe to batch without making display order semantic authority.

### 2. A persisted answer envelope is never a proof witness

A schema-1 `ClarificationAnswerEnvelope` binds one typed response to the exact question id, revision, and
definition digest. It records responder/channel/submission provenance, the claimed authority policy id and policy
digest, answer revision/supersession, and either a typed value or an explicit non-answer.

Deserialization, content hashing, policy-name agreement, lifecycle status, or a stored “accepted” label grants no
canonical authority. Compatibility classifies a current envelope only as `requires_validation`; old envelopes are
inspection-only and future, malformed, or unknown-field envelopes reject. The runtime validator must create a
non-serializable current witness after checking value shape, grounding, responder authorization, currentness,
conflicts, and the exact question/policy bindings. `.4` owns that implementation.

### 3. Authority-specific promotion replaces a generic user-answer premise

There is deliberately no generic `UserAnswer` proof premise.

- A **source locator** must resolve to existing current captured evidence. Any resulting canonical claim cites
  the native `SourceSpan`, `TableCell`, `VisualRegion`, or current upstream claim. The answer explains where to
  look; it is not the semantic premise.
- A **source supplement** must first be copied into the repository-local governed source set, hashed, ingested,
  and captured under the normal source pipeline. Canonical claims again cite native captured evidence. An answer
  attachment cannot masquerade as already-verified source.
- An **external design decision** is the only case that may eventually require a new proof premise kind. `.4`/`.5`
  may introduce a dedicated `ExternalDesignDecision` premise consumed only by registered rules whose declared
  surfaces are externally configurable. It must cite the validated answer, authorization policy, exact decision
  scope, and current question context. It cannot contradict or overwrite a source-owned claim, grant document
  meaning from identifier spelling, or authorize unrelated descendants.

This refines ADR 0038 without weakening it: external choice becomes explicit premise topology, while statements
about what the PDF says still earn ordinary source-grounded proof.

### 4. Non-answers preserve uncertainty and resume state

`unknown`, `unavailable`, `not_applicable`, and `defer` are typed dispositions with explanations. They carry no
authority claim and cannot resume blocked canonical work. They may close an advisory question, keep a blocking
question open/deferred, or trigger a more precise repair request according to the later lifecycle engine. No
default value is synthesized to make progress appear complete.

### 5. Currentness and locality are part of the schema

All persisted artifact paths are normalized repository-root-relative paths. Question and answer revisions are
positive; supersession is digest-bound; source/current artifacts, evidence regions, rulesets, authority policies,
and prior answers use exact SHA-256 identities. Moving the repository therefore preserves the exchange, while a
changed source, proof ruleset, question definition, policy, or answer makes the old binding stale rather than
silently reusable.

## Consequences

- The schema foundation is implemented, but it does not yet plan questions, expose CLI workflows, validate user
  authority, add an external-decision proof premise, or replay a dependency closure. Those are `.2`–`.5`.
- Source clarification remains as strong as the normal proof path because canonical claims cite the recovered
  source—not the user's confidence.
- Legitimate configuration choices remain expressible without laundering them into “the PDF said so.”
- Every accepted response is attributable, currentness-bound, supersedable, and reviewable; every non-answer
  remains explicit.
- The extra hashes and lifecycle records are intentional. They buy deterministic resume and prevent stale or
  tampered feedback from steering an unrelated artifact.

## Links

- Generic proof authority: [`0038-proof-carrying-genericity-kernel.md`](0038-proof-carrying-genericity-kernel.md)
- Repository-local artifact policy: [`0007-live-document-containment-and-data-locality.md`](0007-live-document-containment-and-data-locality.md)
- Task tree: [`SPEC-CLARIFICATION-LOOP.md`](../tasks/SPEC-CLARIFICATION-LOOP.md)
- User guide: [`clarification-loop.md`](../book/src/pipeline/clarification-loop.md)
