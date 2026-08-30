---
id: current-claim-census-freeze
title: The repaired current-claim census freezes exact authority units whose totals ordinary work moves
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
  - "which current claim census counts are stable and which ones move"
  - "which current claim census counts may be published and which must be read from the report"
  - "why did a current claim census count change without the producer changing"
date: 2026-08-30
status: current
tags: [claim-verification, census, authority, currentness, doctrine, task-tree]
evidence: doctrine/claim_verification/current_claim_census.jsonl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.3a.2, .3b.4, and .3c); doctrine/claim_verification/claims.jsonl (current-claim-census-frozen)
reverify: perl scripts/check_current_claim_census.pl --self-test && perl scripts/check_current_claim_census.pl --check && perl scripts/check_current_claim_census.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.3b.4` re-freezes the nonhistorical claim-authority census after the bounded `.3b`
repair sequence.
The denominator is derived from the live-document registry, so it moves whenever a surface is registered:
read `current_surfaces` / `included_surfaces` / `excluded_surfaces` from `--report`. All five views have
evidence, and every included surface has one, which is the property that does not move.

At the `.3b.4` boundary the result was **56 units: 11 `derived`, seven `identity_gated`, six `registered`,
zero `incomplete`, and 32 `excluded`**. None of those fields is a constant.
`CLAIM-VERIFICATION-ADOPTION.6a` measured the whole vector across 28 consecutive revisions and found that
`evidence_units`, `excluded`, `registered`, and every candidate-closure field move under **ordinary, unrelated
work** — a rolling-ledger rollover carries claim-annotated regions out of the live window, and any commit that
adds or drops a `[claim: <id>]` annotation moves the rest. `registered` went 6 -> 5 -> 4 and closure
86/51/35 -> 72/50/22 -> 69/49/20 across that run, stepping inside `5fe81128` and `1507adbf`.

The durable rule is **not** "carry what the trajectory shows has held", which is how `derived` **11** and
`identity_gated` **7** stayed published. `LIVE-DOCUMENT-PRESSURE-HEADROOM.2c` registered one derived
task-catalog surface and moved two fields at once: `current_surfaces` 39 -> 40, withdrawn in that same commit,
and `identity_gated` 7 -> 8 on the same sentence, which nothing noticed until
`CLAIM-VERIFICATION-ADOPTION.11` enumerated the 15 surfaces that cite these producers. A count is publishable
only when a **control** fails if it moves or an **authored decision** fixes it. Exactly two qualify: zero
unresolved candidates, because `validate_candidate_closure` raises an error for any candidate without exact
evidence or a current registered annotation, and **5** views, frozen by `.3a.0`. Read everything else,
`derived` and `identity_gated` included, from `--report`.

The first production pass exposed why view membership cannot be a declaration-only property:
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
incomplete key. That does **not** promote the incomplete assertion-level regions inside the mdBook mapping
(`check_book_quantitative_claims.pl --report`, `authority_outcomes.incomplete`); their three missing legs
remain explicit in the narrower quantitative contract, and that count moves whenever the manual does.

`CLAIM-VERIFICATION-ADOPTION.3c` closed the reverse join from a clean committed boundary. Every produced
candidate must resolve to an exact frozen surface/view/path/line key or a current registered claim annotation;
at that boundary the split was 79 = 51 + 28 and the report had zero unresolved. The 27-case control fixture positively instantiates all five outcome
families, then drives each family and every surface/view/path/region/source/identity coverage boundary RED. A
known registered annotation remains green, while an unrecorded derived marker fails candidate closure.

Three dated closure boundaries are on record: 79 = 51 exact + 28 registered at `.3c`, 84 = 51 + 33 at `.4`
after five additional exact annotations landed in governed task, memory, ledger, toolbox, and book surfaces,
and 86 = 51 + 35 at `.5` after two more in the change ledger and task evidence. Each was correct when written
and none is current — which is the point `.6a` makes. The invariant that survives is the closure property, not
the triple: every produced candidate resolves to exact frozen evidence or a current registered annotation, and
unresolved is zero. See [[worktree-doctrine-measurement-gitlink]] for how the trajectory was measured.
