---
id: claim-standard-upstream-readoption
title: The local claim standard is a restatement, so currency is re-read section by section, not diffed
answers:
  - "is SpecForge's CLAIM_VERIFICATION.md a copy of the upstream standard"
  - "why does a byte diff not prove the local claim standard is current"
  - "when was the upstream claim-verification standard last re-read"
  - "where is the upstream claim-verification source"
  - "which upstream claim-verification rules did SpecForge adopt in CLAIM-VERIFICATION-ADOPTION.10"
  - "which upstream claim-verification material is deliberately not adopted"
  - "what class of defect does a given check still permit"
  - "why must a check and the thing it checks not share a parent"
  - "when has a falsification leg illustrated instead of tested"
  - "how is a published mechanism or causal account verified in SpecForge"
  - "what is the cheapest falsification oracle available in this repository"
  - "how should a red check be attributed to a change"
date: 2026-08-30
status: current
tags: [claim-verification, doctrine, workflow, upstream, currency, task-tree]
evidence: CLAIM_VERIFICATION.md (§11); docs/decisions/0042-actionable-published-claims-require-three-dimensionally-different-legs.md; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.10); TOOLBOX.md
reverify: rg -n 'share a parent|still permits|illustrated|cheapest|enumeration, in both directions|from the producer|revert-and-re-apply' CLAIM_VERIFICATION.md TOOLBOX.md && sed -n '/## 11. Upstream provenance/,$p' CLAIM_VERIFICATION.md
---

SpecForge's `CLAIM_VERIFICATION.md` is a **restatement** of the portable upstream standard at
`/Volumes/SSD/Documents/github/pgen/docs/CLAIM_VERIFICATION.md`, not a copy of it. That is the durable fact a
future currency check needs: `diff` and digest comparison say nothing about whether the local standard still
carries what the source teaches, so directive 17's "check whether an update has been made" can only be answered
by reading both documents section by section. Comparing sizes is the same mistake one level up — the local
document being shorter is not evidence of anything, in either direction.

`CLAIM-VERIFICATION-ADOPTION.10` performed that reading on `2026-08-30`. The source had not moved since adoption
(mtime `2026-08-26`, unchanged across both readings), yet a set of upstream normative rules — enumerated in
`CLAIM_VERIFICATION.md` §11 — had no home on any governed claim surface — `CLAIM_VERIFICATION.md`, `TOOLBOX.md`, `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`,
`AGENTS.md`, the pull-request template, ADRs 0042 and 0044, or the mdBook enforcement chapter. So the gap dated
from the original adoption rather than from an upstream revision, and re-reading is the only instrument that
could have found it.

The adopted rules and their local homes are tabulated in `CLAIM_VERIFICATION.md` §11. The load-bearing ones are
the taxonomy of what each check class **still permits** and its general form — a check and the thing it checks
must not share a parent; the rule that evidence equally consistent with both hypotheses has **illustrated**
rather than tested, which binds a claimed mechanism exactly as it binds a claimed number; the rule that the
cheapest falsification oracle is this project's own adjudicated history in the owning task tree and the commit
body that introduced the value; deriving classifiers and candidate vocabularies from the **producer** rather than
from a description of it; attributing a red check by revert-and-re-apply rather than by reading a diff; and the
rule that a repository-derived constant is derived or gated, never carried.

One adopted rule was present in the repository but **demoted**: ADR 0042's Context paragraph already stated the
general form, while the normative standard carried only the three instances that form generates. A rule living as
decision-record rationale while its examples live in the standard is under-specified for every reader who does
not read ADRs — which is the preface rule the same reading adopted. The promotion is the correction; the ADR
keeps the sentence as the reasoning that produced the decision.

`CLAIM_VERIFICATION.md` §11 also records what stays upstream and why, so a later reading does not re-open each
question: the source's reference-deployment measurements (dated evidence about another project, replaced locally
by this repository's own recorded instances), the five-architecture summary table (already carried by `README.md`
and `DOCTRINE_ENFORCEMENT.md`, so a third copy would be the N-synchronized-copies defect), the inline
provenance-tag syntax (superseded by the executable registry and the `[claim: <id>]` annotation), and adoption
checklist items 1–5 (executed by `.0`–`.5`).
