---
id: published-assertion-gate
title: A published value re-derives against its producer field, or names the control, decision, or revision that fixes it
answers:
  - "how does SpecForge stop a published count going stale under a green gate"
  - "what does the PUBLISHED-ASSERTIONS doctrine prove"
  - "why is a digest binding not enough to watch a number in prose"
  - "which outcomes may a published value have"
  - "may a value be published because a trajectory shows it has not moved"
  - "how are set claims and membership lists checked"
  - "what is excludes_self and why does a classifier need it"
  - "how does SpecForge detect two surfaces disagreeing about one quantity"
  - "are mechanism or causal claims mechanically checked"
  - "how do I reverify the published-assertion gate"
date: 2026-08-30
status: current
tags: [claim-verification, doctrine, currentness, producer, gate, task-tree]
evidence: scripts/check_published_assertions.pl; doctrine/claim_verification/published_assertions.jsonl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.7.0 and .7.1); DOCTRINE_ENFORCEMENT.md (§10)
reverify: perl scripts/check_published_assertions.pl --self-test && perl scripts/check_published_assertions.pl --check && perl scripts/check_published_assertions.pl --report
---

A **digest binding proves that a region's bytes have not changed; it never proves that the number inside it
still re-derives.** That is the whole reason values SpecForge published about its own state went stale ten
separate times under a fully green gate, twice inside the very commit that published them, and why five
consecutive rounds of prose correction were each invalidated by their own transaction. The enumerated instances
live in `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md`.

`PUBLISHED-ASSERTIONS` closes it by **executing the producer**. A registry record names the governed region
(path, line range, SHA-256 of those bytes), the literal value published there, and a field path into a
producer's JSON report; the checker runs that producer with an argv array — never a shell string — and compares.

A value has exactly one of four outcomes, and there is deliberately no fifth:

- `derived` — re-executed and compared;
- `gated` — names the control whose failure would follow the value moving, bound to an exact known-bad region
  inside that control's own tracked source;
- `authored` — names the tracked decision record that fixes it;
- `dated` — names the revision it is anchored to, which must resolve.

**"A trajectory shows it has held" is not expressible**, which is the point. A trajectory shows what has not
happened, never what cannot: a value stable across 29 measured revisions moved on the next structural slice, and
that is the instance the missing fifth outcome commemorates.

Three further rules each exist because one leg alone was insufficient. **Cross-surface disagreement** fails with
no producer run at all — two records naming one producer field with different values is a defect regardless of
which is right. **A membership field is compared as an enumeration, never as a size**, because a set can drift
while its count holds. And a record whose enumerator could return the very surface that publishes it must
declare `excludes_self`: a screen that the act of writing a finding turns green shares a parent with the thing
it checks, which is how an ownership grep reported three surfaces as unowned and then owned them by saying so.

**Mechanism claims are out of scope by decision**, not by omission: no checker can decide whether two accounts
predict the same observation. A record may carry `adjudicated_against` to name the prior ruling a mechanism was
checked against — shape, not truth. Mechanisms remain a reviewer obligation under `CLAIM_VERIFICATION.md` §3
Leg 2 and §8.

The governed population is **derived on every run and never stored**, because a commit that annotates a region
joins it; three commits in a row moved their own published population by describing it. A `[claim: <id>]` tag
governs the **paragraph it closes**, not the line it sits on — authors put the tag at the end of a block, so
keying coverage on the tag's own line would be blind to exactly the sentences it exists to watch.
See [[current-claim-census-freeze]] and [[claim-control-audit-closure]] for the sibling claim doctrines.
