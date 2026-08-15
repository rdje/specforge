---
id: current-claim-census-freeze
title: The current-claim census freezes 51 exact authority units and five incomplete repair keys
answers:
  - "what did the current claim census find"
  - "how many current claim evidence units are derived registered incomplete or excluded"
  - "why must a current claim census candidate key include the semantic view"
  - "which five current claim census gaps does CLAIM-VERIFICATION-ADOPTION.3b repair"
  - "how many current governed Markdown surfaces are in the claim census"
  - "how do I reverify the frozen current claim census"
date: 2026-08-15
status: current
tags: [claim-verification, census, authority, currentness, doctrine, task-tree]
evidence: doctrine/claim_verification/current_claim_census.jsonl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.3a.2); doctrine/claim_verification/claims.jsonl (current-claim-census-frozen)
reverify: perl scripts/check_current_claim_census.pl --self-test && perl scripts/check_current_claim_census.pl --check && perl scripts/check_current_claim_census.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.3a.2` freezes the nonhistorical claim-authority census before any source repair.
The denominator is derived from the live-document registry: 39 current surfaces, of which 32 enter semantic
inspection and seven have explicit standard-scope exclusions. All five views have evidence.

The frozen 51-unit result is **11 `derived`, four `identity_gated`, four `registered`, five `incomplete`, and
27 `excluded`**. The first production pass exposed why view membership cannot be a declaration-only property:
a candidate keyed by surface + path + line collapsed a second semantic view onto the first. Candidate identity
now includes surface + view + path + line, and frozen validation independently requires evidence for every
included surface and every required view.

Only these five incomplete evidence keys form the `.3b` repair frontier:

- `evidence-readme-entrypoint-maintained-references-e1a5013b8d02`;
- `evidence-shipped-behavior-mdbook-quantitative-claims-88501c2f5e66`;
- `evidence-workflow-standards-doctrine-baselines-7bf07db7c4e8`;
- `evidence-knowledge-cards-maintained-references-075a14c930ba`;
- `evidence-fsmgen-issue-packets-maintained-references-ecf3712243ed`.

Each is explicitly missing re-derivation, falsification, and durability at the surface-wide claim boundary.
The freeze changes no source assertion; `.3b` must repair or narrow those keys without touching the other 46
classified units.
