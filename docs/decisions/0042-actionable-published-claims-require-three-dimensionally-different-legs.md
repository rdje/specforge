---
id: actionable-published-claims-require-three-dimensionally-different-legs
title: Actionable published claims require three dimensionally different verification legs
date: 2026-08-15
status: accepted
scope: claims, verification, doctrine, review, continuity, currentness
evidence: CLAIM_VERIFICATION.md; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md; COMMIT.md; TOOLBOX.md; .github/PULL_REQUEST_TEMPLATE.md
reverify: "rg -n 'Published-claims:|Re-derive|falsif|durab' CLAIM_VERIFICATION.md COMMIT.md TOOLBOX.md .github/PULL_REQUEST_TEMPLATE.md; perl scripts/check_canonical_collection_catalogs.pl --check; bash scripts/check_doctrines.sh"
answers:
  - "what is ADR 0042"
  - "what makes a SpecForge published claim verified"
  - "which three legs must an actionable current claim name"
  - "when may a commit use Published-claims none"
  - "are historical measurements governed as current claims"
  - "how are stochastic claims published"
  - "why is repeating the same check not independent verification"
  - "what should an auditor do when re-derivation disagrees with a published value"
---

# ADR 0042: Actionable published claims require three dimensionally different verification legs

## Context

SpecForge already has task ownership, durable memory, retrieval, and one doctrine driver. It also has strong
individual currentness checks, controlled mutations, deterministic projections, and CI oracles. What it lacks is
a per-claim contract joining three different questions: can the current assertion be reproduced from source, can
a different observation distinguish a plausible competing explanation, and will tracked state fail when the
assertion goes stale?

Repeated checking does not close that gap when the checks share a parent. A conservation check cannot detect
redistribution; a row count cannot detect a wrong value in every row; tests and implementation derived from the
same interpretation cannot independently validate that interpretation. A freshly corrected number can also go
stale immediately when its producer is untracked or its dependencies are not identity-gated.

The adoption census found no local claim-verification standard, registry/checker, or pull-request template.
Existing derived-state contracts remain useful source/currentness authorities, but they do not model a competing
hypothesis or prove a cited control has gone RED. Overloading them would conflate distinct concerns.

## Decision

### 1. Govern current actionable assertions, not digit syntax

An assertion is governed when it is current-facing, may change, and may influence a technical or operational
decision. Current counts, scores, capacities, compatibility, performance, completeness, and carried measured
constants are in scope. Clearly dated historical observations, authored policy thresholds, schema/version/date/
digest/path literals, examples, and immutable evidence captures retain their existing authority. Reusing a
historical observation as current creates a new governed claim.

### 2. Reserve “verified” for three different legs

Every verified claim names:

1. **re-derivation** — an exact repository-root-relative command or accessor reproduces it from canonical source;
2. **falsification** — a dimensionally different oracle separates a named competing hypothesis and has a tracked
   known-bad case observed going RED;
3. **durability** — all producers, controls, and artifact dependencies are tracked or explicit external inputs,
   and a deterministic stale-state gate covers their complete identity.

A missing or non-independent leg is recorded explicitly and the claim is `incomplete`. A valid record shape does
not itself prove the assertion.

### 3. Make every slice declare its publishing effect

Commit and pull-request descriptions carry exactly one `Published-claims:` declaration: either `none` or a
comma-separated set of stable claim IDs. `none` means the slice publishes or changes no governed current
assertion; it is not a blanket docs-only or code-only exemption. The review template, `COMMIT.md`, `TOOLBOX.md`,
bootstrap, and README discovery route all point to the repository-owned standard.

### 4. Keep provenance enforcement separate and bounded

`CLAIM-VERIFICATION-ADOPTION.2` will introduce a dedicated self-bounded JSONL registry and checker under
`doctrine/claim_verification/`, then add one entry to the existing doctrine driver. Derived-state contracts remain
inputs to a claim's source/currentness leg instead of becoming a second claim registry. Until `.2` lands, the
authoring/review contract is normative but not described as mechanically enforced.

### 5. Preserve uncertainty and auditor asymmetry

Stochastic claims publish intervals or distribution summaries with repeat/seed/environment identity. When a new
re-derivation conflicts with an existing result, neither wins automatically; the newer instrument may be wrong.
The disagreement stays explicit until the falsification leg separates the competing explanations.

## Consequences

- Review can distinguish “checked again” from evidence that could actually falsify the claim.
- A claim with a named gap remains usable as incomplete evidence; hidden gaps cannot masquerade as signoff.
- Current constants will be swept only after the bounded registry/checker freezes their identity and status shape.
- Historical records remain stable; only their reuse as current truth creates new verification work.
- Claim verification complements rather than replaces task-tree acceptance, regression oracles, Knowledge Map
  retrieval, or doctrine enforcement.
- The fifth architecture is published now; mechanical provenance enforcement, current-claim migration, RED-control
  closure, and full adoption signoff remain owned by `.2` through `.5`.

## Links

- Standard: [`CLAIM_VERIFICATION.md`](../../CLAIM_VERIFICATION.md)
- Task tree: [`CLAIM-VERIFICATION-ADOPTION`](../tasks/CLAIM-VERIFICATION-ADOPTION.md)
- Doctrine sibling: [ADR 0006](0006-doctrine-enforcement-architecture.md)
- Currentness precedent: [ADR 0025](0025-persisted-chain-currency-is-measured-not-assumed.md)
