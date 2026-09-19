---
id: retained-bundle-population-is-frozen
title: The retained normalized-bundle freeze is retired — growth is admitted, and a key above the frozen behavioral floor is declared debt
answers:
  - "why does declaring a newly re-ingested normalized bundle fail the doctrine gate"
  - "what happens to the doctrine gates when I add a key to doctrine/chain_currency/retained_bundles.json"
  - "how do I record a reclamation in retained_bundles.json"
  - "how do I declare a newly retained normalized bundle"
  - "which checks pin the retained-bundle population at 24"
  - "what does a re-ingest do to the behavioral genericity population"
  - "why is an in-repo gold document re-ingest blocked by PRODUCTION-GENERICITY"
  - "where is the APB normalized bundle after WIRE-BASED-100.9b"
  - "why does chain-currency report APB EvidenceIR as unmeasurable when its EvidenceIR is schema 3"
  - "is proof currency the same thing as normalized bundle retention"
date: 2026-09-19
status: current
tags: [doctrine-enforcement, corpus, currency, retention, behavioral-qualification, adr-0025]
evidence: doctrine/production_genericity/post_boundary_retention.json; docs/decisions/0050-a-frozen-qualification-population-is-a-subset-floor-not-an-equality.md; scripts/validate_residual_actionability_contract.py; scripts/validate_canonical_recovery_contract.py; scripts/check_behavioral_genericity_contract.py; scripts/check_chain_currency.sh; doctrine/chain_currency/retained_bundles.json; doctrine/production_genericity/behavioral_qualification.json; docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md; docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md
reverify: "python3 -B scripts/check_behavioral_genericity_contract.py --self-test — expect 23/23 RED and 1/1 admissible"
---

> **RETIRED `2026-09-19` by `RETAINED-BUNDLE-POPULATION-FROZEN.1` and `.2`. All three mechanisms are
> gone.** `.1` retired the `24` literal and the `reclamations != []` freeze, keeping the
> count/digest binding that was their real content. `.2` replaced mechanism 3's set equality with a
> **subset floor plus declared residual** (**ADR 0050**): the frozen behavioral population must stay a
> subset of `retained`, and every retained key above it is declared in
> `doctrine/production_genericity/post_boundary_retention.json` with the relations it owes and the leaf
> that owes them. **Measured end state:** `retained` grown to 27 with the three golds declared runs the
> behavioral gate **green**, reporting `3 retained post-boundary and unqualified`. Declaring a bundle
> and recording a reclamation are both compliant moves now. The history below is kept because it
> explains why the freeze existed and what each mechanism was actually protecting.
>
> **`.3` landed the same day and the tree is closed.** All three gold bundles are installed at their
> normalized roots and declared, `retained` is **27**, and `check_chain_currency.sh` reports
> **evidence 27 replayed / 27 current / 0 stale** with *retention: 27 — exactly the declared retained
> set*. The interim hold is over: the six preserved copies are gone (583,434,736 bytes reclaimed) and
> the held-out census is 0 ([[wire-golds-held-out-not-lost]]).

## History — what the freeze was, and what each mechanism protected

ADR 0025 decision 3 mandates two operations on `doctrine/chain_currency/retained_bundles.json`: a refresh
**keeps** its normalized bundle (retention is what makes a document's EvidenceIR stage replayable, and the
54 missing bundles are "backfilled at each document's own refresh"), and a reclamation stays possible as a
"deliberate, task-owned" recorded operation. Three independent mechanisms forbid exactly those two moves.

| # | Mechanism | Where | Forbids |
| --- | --- | --- | --- |
| 1 | `len(retained_ids) != 24` literal | `validate_residual_actionability_contract.py` (gate-tier `RESIDUAL-ACTIONABILITY`) and `validate_canonical_recovery_contract.py` | any size but 24 — growth **and** shrink |
| 2 | `retained.reclamations != []` | the same two validators | the mechanism ADR 0025 provides for recording a reclamation |
| 3 | behavioral population **set equality** with `retained` | `check_behavioral_genericity_contract.py` (gate-tier `PRODUCTION-GENERICITY`) | growth |

Mechanism 1 is redundant: each contract also pins `affected_chain_ids_sha256` against the live authority,
which binds the exact membership and therefore already forces the contract to be updated whenever the set
changes. A size literal cannot catch a drift the digest misses; it can only stop compliant growth.

Mechanism 3 is the substantive one. `behavioral_qualification.json` declares a `selection_boundary_commit`
and a frozen 24-row population with a 7/17 calibration/holdout split, `declared_attempts: 51`
(17 prospective × 3 held-out relations), per-document held-out evidence with vendor/family novelty flags,
and `final_signoff: deferred_to_SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v`. A newly retained key is therefore not
admissible by bookkeeping: it needs three executed held-out relations and an amendment to a release-blocking
qualification that another tree owns and has not finished.

## How this shows up

`check_chain_currency.sh` fails closed on a bundle that is on disk but undeclared ("a refresh that did not
record what it retained"), so a re-ingest forces the declaration to grow — and growing it turns
`PRODUCTION-GENERICITY` and `RESIDUAL-ACTIONABILITY` red. Recording a reclamation instead turns the same two
red. `WIRE-BASED-100.9b` hit this on the first re-ingest after the freeze and took the only move that
deletes nothing and weakens no gate: it **held** the bundle at
`generated/preserved/WIRE-BASED-100.9b/apb-normalized-bundle-held-out/` (repository volume, byte-identical)
and left the declaration at 24.

## The distinction that keeps this readable

**Proof currency and bundle retention are different properties, and a single "24" or "25" hides that.**
After `WIRE-BASED-100.9b` the APB chain is canonical schema 3/3/2/2 and `eval-extraction` scores it, so the
canonical frontier is 25 current / 53 legacy — but only 24 documents have a declared retained bundle, so
`check_chain_currency.sh` reports `evidence: 24 replayed … 54 UNMEASURABLE` while `semantic`, `intent` and
`isf-adapter` each report 25. Both numbers are right; they answer different questions. Any surface that
states one of them should say which.

**The hold is not one document (corrected `2026-09-19` by `CORPUS-CHAIN-CURRENCY.10a`).** `.9c` and `.9d`
held out AHB and AXI on the same terms, and `WIRE-BASED-100.10` re-ingested and held out all three again on
`2026-09-11`. So the frozen population costs the EvidenceIR measurability of **three of the four wire-based
golds**, not one — and that cost is what led another tree to plan a destructive re-ingest of those same
golds, on the belief that a bundle absent from the normalized root was a bundle that no longer existed. All
three replay CONTENT SAME from their held-out bundles in 4.25 s together
([[wire-golds-held-out-not-lost]]).

Repair and restoration are **both done**, and the tree is closed. `.1` retired the literal and the
`reclamations` freeze, `.2` replaced the equality with ADR 0050's subset floor, and `.3` installed and
declared all three bundles. The end state, measured: `retained` 27, behavioral gate green reporting
**3 retained post-boundary and unqualified**, `affected_chain_count` 27 with its digest recomputed, and
`check_chain_currency.sh` **27 replayed / 27 current / 0 stale** at every stage.

**The three golds are retained but NOT behaviorally qualified, and that is recorded rather than
implied.** They are declared in `doctrine/production_genericity/post_boundary_retention.json`, each
owing three held-out relations to `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`. Retention and qualification
are different properties; ADR 0050 is what lets the first move without falsely claiming the second.

**The frozen boundary values were adjudicated and stay exact.** `population_assertions.current_documents`
and `frozen_census.aggregate.documents` are compared against `len(rows)` from
`behavioral_population.tsv`, not against `retained`, so growing the retained set leaves both green. They
are frozen values checked against the frozen thing they describe.

Links: [[chain-currency-doctrine]], [[corpus-canonical-currency-and-ownership]],
[[wire-golds-held-out-not-lost]], [[behavioral-text-projection-boundary]].
