# Clarification and Assisted Completion

SpecForge's terminal goal is a well-structured, well-behaving FSMGen ISF specification, not merely a collection
of extracted text. The default path remains autonomous: recover and prove every conclusion that the current PDF,
captured visuals/tables, registered digital semantics, and validated identity-independent priors support. A user
is consulted only where a missing premise blocks valuable downstream work and cannot safely be recovered by the
governed pipeline.

The clarification foundation is now a versioned IR. It does **not** yet mean the runtime plans questions or
resumes a build; those capabilities land in the following task-tree leaves. The current implementation freezes
the interchange, lifecycle, currentness, and authority contracts so later automation cannot improvise them.

## The Product Loop

```text
PDF + governed inputs
        |
        v
autonomous proof-bearing extraction --------------------------+
        |                                                      |
        +-- all required premises present --> IntentIR --> ISF |
        |                                                      |
        +-- unresolved premise                                 |
                |                                              |
                v                                              |
        typed clarification packet                             |
                |                                              |
                v                                              |
        answer / unknown / unavailable / defer                 |
                |                                              |
                v                                              |
        current validation + conflict checks                   |
                |                                              |
                +-- rejected --> precise repair request        |
                |                                              |
                +-- accepted --> minimal affected replay ------+
```

Unaffected branches continue when safe. One ambiguous reset polarity, for example, should not prevent SpecForge
from completing an independent register map. A packet declares whether independent work may continue or the
whole current build is blocked.

## What a Question Contains

A clarification is a machine-readable `ClarificationQuestion`, not an ephemeral prompt. Its immutable definition
contains:

| Contract | What the user gets |
| --- | --- |
| Stable identity | Question id, positive revision, and SHA-256 of the exact immutable definition |
| Current context | Repository-relative source/current-artifact paths and digests plus the proof-ruleset digest |
| Evidence | Exact source span, table region, visual region, proof claim, residual packet, or validation finding |
| Missing premise | The unresolved engineering proposition and why autonomous evidence stopped |
| Alternatives | Known interpretations and the downstream consequence of each |
| Impact | Exact IR/adapter/ISF surfaces blocked or advised, with stable record keys where available |
| Scheduling | Priority, 0–100 information-gain score, exact dependencies, and autonomous-continuation policy |
| Response contract | Accepted authority types, typed value schema, units/bounds/choices, validation rules, examples |
| Lifecycle | Open/pending/accepted/rejected/deferred/resolved/stale/superseded/cancelled state and exact links |

Questions in one packet are dependency-ordered. A dependency identifies the earlier question's id, revision, and
definition digest. A missing, forward, cyclic, or stale dependency fails validation. Later planning will use this
structure to deduplicate equivalent gaps and ask high-information questions before dependent details.

The closed missing-information reasons are:

- `source_ambiguity`: the current source supports more than one interpretation;
- `source_contradiction`: current source regions disagree;
- `missing_source_content`: the required fact is absent from the governed source set;
- `extraction_limitation`: the source appears sufficient, but the current pipeline cannot recover it safely; and
- `external_design_choice`: the specification deliberately leaves an implementation/configuration decision open.

These reasons matter because they lead to different forms of authority. “Which table cell did we overlook?” and
“Which legal implementation option do you choose?” are not the same question.

## Accepted Answer Shapes

Schema version 1 supports atomic and nested digital-design answers:

- boolean;
- bounded integer or canonical decimal, with an optional unit;
- single or multiple choice enumeration;
- binary/four-state or hexadecimal bit vector;
- identifier or bounded text;
- exact source-reference set;
- typed record; and
- bounded list of another answer schema.

For example, a reset-polarity question can require a single enumeration value:

```json
{
  "kind": "enumeration",
  "choices": ["active-high", "active-low"],
  "allow_multiple": false
}
```

A compound timing answer can instead be a record whose fields separately declare an integer cycle count and an
enumerated reference event. The schema carries units, ranges, required fields, and validation rules rather than
forcing the user to infer an acceptable free-form format.

Every question also accepts four explicit non-answers: `unknown`, `unavailable`, `not_applicable`, and `defer`,
each with an explanation. A non-answer carries no authority claim and never fabricates a default. It preserves
the exact unresolved state so the exchange can resume later.

## The Three Authority Paths

An answer is useful evidence, but the serialized envelope is never canonical proof. SpecForge distinguishes three
authority paths:

1. **Source locator.** The user points to information already present in the current PDF/capture. SpecForge must
   resolve and re-check that region. A resulting canonical conclusion cites its normal source/table/visual proof
   premise; the user's statement does not replace the source.
2. **Source supplement.** The user supplies an authoritative missing document or correction. It must first become
   a repository-local governed artifact, be hashed and ingested, and enter ordinary capture proof. An attachment
   is not trusted merely because it arrived in an answer.
3. **External design decision.** The source intentionally allows a choice and an authorized design owner selects
   one. Later implementation may admit this only through a dedicated, narrowly registered external-decision
   proof premise scoped to configurable downstream surfaces. It cannot override a source-owned claim or authorize
   unrelated work.

There is deliberately no generic `UserAnswer` premise. That would erase the distinction between “the PDF says
this” and “the designer chose this,” making provenance misleading and conflict handling unsafe.

## Answer Envelopes and Currentness

A `ClarificationAnswerEnvelope` records:

- answer id and positive revision;
- exact question id/revision/definition digest;
- responder id, interaction channel, and submission sequence;
- claimed authority kind plus exact policy id and policy digest;
- typed value or explicit non-answer; and
- exact previous-answer reference when superseding a response.

This is an untrusted interchange envelope. Parsing it, hashing it, or seeing an `accepted` lifecycle label does
not grant proof authority. A current envelope is classified only as `requires_validation`. A legacy schema is
inspection-only; future, malformed, missing-version, unknown-field, stale-question, and stale-policy envelopes
fail closed.

The later validator must additionally check the value against the issued schema, source grounding, responder
authorization, source and cross-answer conflicts, policy currentness, and answer supersession. Only an in-memory
validation witness may enter the appropriate authority-specific promotion path. That witness is not serializable,
so copying an old “accepted” JSON file cannot recreate authority.

## Lifecycle and Resume

The question definition is immutable; lifecycle is a separate current projection with a monotonic state sequence.
This prevents an answer binding from changing merely because status moved from `open` to
`answered_pending_validation`. Superseding a question creates a new definition revision and exact digest link.
Superseding an answer similarly requires an older digest-bound revision of the same answer identity.

Later replay will invalidate only claims descended from an accepted premise, preserve every unrelated current
claim and proof byte, and rebuild the affected dependency closure transactionally. Withdrawal, supersession, a
changed source artifact, a changed proof ruleset, or a changed authority policy must stale exactly the dependent
work. Adapter/ISF emission remains blocked until every required premise verifies.

## Repository Locality

Every persisted artifact path in the clarification IR is normalized relative to the repository root. Absolute,
home-relative, parent-traversing, empty-component, and Windows-style external paths reject. Supplemental source
material must follow the project locality policy before it can participate. The repository may move to another
directory or filesystem without invalidating an otherwise unchanged packet.

## Current Implementation Boundary

Implemented now:

- schema-1 packet, immutable question definition, lifecycle, typed answer schema, answer envelope, authority
  claims, current binding, compatibility classification, bounded validation, deterministic hashes, and focused
  fail-closed tests;
- public Rust access through `specforge_core::ir::clarification`; and
- the authority decision in ADR 0040.

Not implemented yet:

- residual/completeness/validation/adapter question planning and deduplication (`.2`);
- CLI list/inspect/export/answer workflows (`.3`);
- semantic value, grounding, authorization, conflict, and tamper-evident acceptance (`.4`);
- proof-kernel integration and minimal transactional replay (`.5`);
- adversarial efficiency/genericity qualification (`.6`); and
- governed zero-question and assisted PDF-to-FSMGen-ISF end-to-end closure (`.7`).

Until those leaves land, the schema is an implemented foundation, not a claim that interactive completion is
already operational.
