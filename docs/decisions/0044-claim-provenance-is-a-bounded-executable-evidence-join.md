---
id: claim-provenance-is-a-bounded-executable-evidence-join
title: Claim provenance is a bounded executable evidence join
date: 2026-08-15
status: accepted
scope: claims, verification, registry, doctrine, currentness, review
evidence: CLAIM_VERIFICATION.md; doctrine/claim_verification/claims.jsonl; scripts/check_claim_verification.pl; scripts/check_doctrines.sh
reverify: "perl scripts/check_claim_verification.pl --self-test; perl scripts/check_claim_verification.pl --check; perl scripts/check_claim_verification.pl --report"
answers:
  - "what is ADR 0044"
  - "how is published claim provenance mechanically gated"
  - "what is the claim verification registry schema"
  - "how do Published-claims ids resolve"
  - "how does claim evidence become stale"
  - "why are claim evidence commands argv arrays"
  - "does a valid claim registry record prove the assertion true"
---

# ADR 0044: Claim provenance is a bounded executable evidence join

## Context

ADR 0042 reserves `verified` for three different legs, but prose alone cannot detect a mistyped claim ID, a
deleted producer, an omitted artifact dependency, or a result carried past an input change. Existing derived-state
contracts answer field/copy currentness and remain useful inputs; adding falsification semantics to them would
conflate two authorities and force unrelated live-document records to model claim publication.

The gate must also avoid two opposite failures. Treating registry commands as shell strings would make tracked
data executable through parsing and quoting. Validating only hashes would preserve stale logic perfectly while
never proving that a declared re-derivation or known-bad control still runs.

## Decision

### 1. Use one dedicated self-bounded JSONL registry

`doctrine/claim_verification/claims.jsonl` starts with one control record bounding records, total bytes, record
bytes, array items, and scalar bytes below checker-compiled portable hard caps. Every nested object has a closed
field set. Claim IDs are unique lowercase hyphenated identifiers; statuses are exactly `verified`, `incomplete`,
or `superseded`.

A verified record carries a precise assertion/owner and three typed legs:

- re-derivation boundary, result, and one or more commands;
- competing hypothesis, observed-RED result, and one or more controls;
- refresh owner/rule, complete tracked artifact identities, stale check, and retained evidence.

Incomplete records name missing legs and omit fabricated evidence. Superseded records point to a known successor
and cannot be published as current.

### 2. Execute argv, never a shell program string

Each command is an argument array with one declared repository-relative tracked producer, explicit inputs, an
expected zero exit, and a bounded output marker. The checker uses direct process execution from the repository
root; it performs no shell interpolation. Every verified re-derivation and falsification control runs on the gate.

### 3. Join command membership to exact tracked artifact identity

Every command producer and declared input must occur in the durability artifact set. Every artifact must be a
regular non-symlink Git-tracked path with its exact current SHA-256. The stale check's producer plus inputs must
equal the entire artifact set, and retained-evidence paths must carry that role. The registry control plane is
parsed and bounded directly, so it does not attempt an impossible self-hash.

Changing any watched dependency makes the claim RED until the owner re-runs both evidence legs and refreshes the
digest or records explicit incompleteness. The checker cannot infer an omitted semantic dependency; `.3` and `.4`
therefore still own the current-claim census and independent producer/control closure audit.

### 4. Resolve the publication declaration at the same gate

When the commit workflow has populated nonempty `git_message_brief.txt`, the checker validates that pending
message. Otherwise it validates `HEAD`. Exactly one `Published-claims:` declaration is required; `none` is
exclusive, listed IDs must be unique and known, and a superseded ID is rejected. The existing doctrine driver is
the only hook/CI composition seam.

### 5. Prove fail-closed behavior continuously

The real gate executes its self-test through the registered gate-active claim. Twenty-two repository-volume cases
cover valid verified/incomplete/superseded records and `none`/known publication positives plus missing legs/declarations, unknown
fields/status/IDs/successors, duplicate records/artifacts/declarations/IDs, digest staleness, omitted identity,
untracked/unsafe paths, false RED assertions, incomplete stale coverage, and portable hard-cap refusal.

## Consequences

- A publication ID, its current evidence identity, and its executable source/control behavior fail together.
- The registry makes provenance inspectable and rerunnable without claiming that schema validity proves semantic
  truth or that two commands are genuinely independent merely because both exit zero.
- Derived-state contracts remain canonical field/currentness inputs instead of becoming a parallel claim store.
- Future claim changes deliberately refresh watched hashes; silent copied-state preservation is no longer green.
- Commit hooks remain locally bypassable and hosted CI remains manual-only, as documented by doctrine enforcement.

## Links

- Standard: [`CLAIM_VERIFICATION.md`](../../CLAIM_VERIFICATION.md)
- Owning task: [`CLAIM-VERIFICATION-ADOPTION`](../tasks/CLAIM-VERIFICATION-ADOPTION.md)
- Three-leg decision: [ADR 0042](0042-actionable-published-claims-require-three-dimensionally-different-legs.md)
- Doctrine architecture: [ADR 0006](0006-doctrine-enforcement-architecture.md)
