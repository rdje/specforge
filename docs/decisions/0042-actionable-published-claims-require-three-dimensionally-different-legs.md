---
id: actionable-published-claims-require-three-dimensionally-different-legs
title: Actionable published claims require three dimensionally different verification legs
date: 2026-08-15
status: accepted
scope: claims, verification, doctrine, review, continuity, currentness
evidence: CLAIM_VERIFICATION.md; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md; COMMIT.md; TOOLBOX.md; .github/PULL_REQUEST_TEMPLATE.md; CLAIM_VERIFICATION.md §11
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
  - "did SpecForge re-adopt the upstream claim-verification standard after ADR 0042"
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
- The fifth architecture's implementation is owned by `.2` through `.5`: mechanical provenance enforcement,
  current-claim migration, RED-control closure, documentation, and final signoff close through those leaves.

## Implementation status

`CLAIM-VERIFICATION-ADOPTION.5` closes the planned adoption. The bounded registry and doctrine gate are active;
the current-surface and mdBook quantitative censuses expose silent or incomplete authority; every cited control
binds exact known-bad evidence; ignored or untracked producer candidates fail; and the mdBook teaches the author
and auditor workflow. This status records implementation of the decision without changing its scope or the
deliberate boundary that registry validity alone cannot prove semantic truth.

`CLAIM-VERIFICATION-ADOPTION.10` (`2026-08-30`) re-read the upstream source that directive 17 requires be checked
for updates. The source had not moved since adoption, but the local standard is a restatement rather than a copy,
and a section-by-section reading found a set of upstream normative rules with no home on any governed claim
surface — enumerated in `CLAIM_VERIFICATION.md` §11, and including the taxonomy of what each check class still permits, the rule that evidence consistent with both
hypotheses has illustrated rather than tested, and the rule that the cheapest falsification oracle is the project's
own adjudicated history. Each is now normative in `CLAIM_VERIFICATION.md`, and §11 records the reading boundary
plus the upstream material deliberately left upstream, so the next check starts from a boundary rather than from
scratch.

One of them was already present here, demoted: this ADR's own Context paragraph states the general form —
a check and the thing it checks must not share a parent — while the standard carried only the three instances that
form generates. A rule living as ADR rationale while its examples live in the normative text is under-specified for
every reader who does not read ADRs, which is why §2 now states the form and this paragraph keeps the reasoning.
This re-adoption changes what the standard says, not what this decision decided.

## Links

- Standard: [`CLAIM_VERIFICATION.md`](../../CLAIM_VERIFICATION.md)
- Task tree: [`CLAIM-VERIFICATION-ADOPTION`](../tasks/CLAIM-VERIFICATION-ADOPTION.md)
- Doctrine sibling: [ADR 0006](0006-doctrine-enforcement-architecture.md)
- Currentness precedent: [ADR 0025](0025-persisted-chain-currency-is-measured-not-assumed.md)
