---
id: current-claim-census-freeze
title: The repaired current-claim census freezes 56 exact authority units with no incomplete outer key
answers:
  - "what did the current claim census find"
  - "how many current claim evidence units are derived registered incomplete or excluded"
  - "why must a current claim census candidate key include the semantic view"
  - "which five current claim census gaps does CLAIM-VERIFICATION-ADOPTION.3b repair"
  - "are any outer current claim census keys incomplete after CLAIM-VERIFICATION-ADOPTION.3b"
  - "how does the current claim census prove no produced candidate is silent"
  - "how many current claim census candidates join exact evidence or registered annotations"
  - "how many current governed Markdown surfaces are in the claim census"
  - "how do I reverify the frozen current claim census"
date: 2026-08-15
status: current
tags: [claim-verification, census, authority, currentness, doctrine, task-tree]
evidence: doctrine/claim_verification/current_claim_census.jsonl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.3a.2, .3b.4, and .3c); doctrine/claim_verification/claims.jsonl (current-claim-census-frozen)
reverify: perl scripts/check_current_claim_census.pl --self-test && perl scripts/check_current_claim_census.pl --check && perl scripts/check_current_claim_census.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.3b.4` re-freezes the nonhistorical claim-authority census after the bounded `.3b`
repair sequence.
The denominator is derived from the live-document registry: 39 current surfaces, of which 32 enter semantic
inspection and seven have explicit standard-scope exclusions. All five views have evidence.

The current 56-unit result is **11 `derived`, seven `identity_gated`, six `registered`, zero `incomplete`, and
32 `excluded`**. The first production pass exposed why view membership cannot be a declaration-only property:
a candidate keyed by surface + path + line collapsed a second semantic view onto the first. Candidate identity
now includes surface + view + path + line, and frozen validation independently requires evidence for every
included surface and every required view.

These five formerly incomplete evidence keys formed the `.3b` repair frontier:

- `evidence-readme-entrypoint-maintained-references-e1a5013b8d02`;
- `evidence-shipped-behavior-mdbook-quantitative-claims-88501c2f5e66`;
- `evidence-workflow-standards-doctrine-baselines-7bf07db7c4e8`;
- `evidence-knowledge-cards-maintained-references-075a14c930ba`;
- `evidence-fsmgen-issue-packets-maintained-references-ecf3712243ed`.

`.3b.4` preserves the semantic outcomes of the other 46 units; exact ledger, book, and task regions rotate when
their lockstep publications move. Each old title anchor is now an explicit identity exclusion: the three
maintained-reference surfaces add their executable route identity, workflow doctrine adds the registered capacity
paragraph, and the mdBook view adds the registered exact-region mapping authority. The outer census has no
incomplete key. That does **not** promote the 75 incomplete assertion-level regions inside the mdBook mapping;
their three missing legs remain explicit in the narrower quantitative contract.

`CLAIM-VERIFICATION-ADOPTION.3c` closes the reverse join from a clean committed boundary. All 79 produced
candidates must resolve to one of 51 exact frozen surface/view/path/line keys or 28 current registered claim
annotations; the report has zero unresolved. The 27-case control fixture positively instantiates all five outcome
families, then drives each family and every surface/view/path/region/source/identity coverage boundary RED. A
known registered annotation remains green, while an unrecorded derived marker fails candidate closure.

The `.3c` commit boundary was 79 = 51 exact + 28 registered + zero unresolved. `.4` publishes five additional
exact claim annotations in governed task, memory, ledger, toolbox, and book surfaces; the current closure is
therefore 84 = 51 exact + 33 registered + zero unresolved without changing the 56 evidence classifications.

The closing `.5` documentation adds two current annotations in the governed change ledger and task evidence.
Final adoption therefore closes 86 = 51 exact + 35 registered + zero unresolved, again without changing the
frozen 56-unit authority vector.
