---
id: retained-bundle-population-is-frozen
title: The retained normalized-bundle set can neither grow nor shrink, so a policy-mandated re-ingest has no compliant move
answers:
  - "why does declaring a newly re-ingested normalized bundle fail the doctrine gate"
  - "what happens to the doctrine gates when I add a key to doctrine/chain_currency/retained_bundles.json"
  - "why can I not record a reclamation in retained_bundles.json"
  - "which checks pin the retained-bundle population at 24"
  - "what does a re-ingest do to the behavioral genericity population"
  - "why is an in-repo gold document re-ingest blocked by PRODUCTION-GENERICITY"
  - "where is the APB normalized bundle after WIRE-BASED-100.9b"
  - "why does chain-currency report APB EvidenceIR as unmeasurable when its EvidenceIR is schema 3"
  - "is proof currency the same thing as normalized bundle retention"
date: 2026-09-10
status: current
tags: [doctrine-enforcement, corpus, currency, retention, behavioral-qualification, adr-0025]
evidence: scripts/validate_residual_actionability_contract.py; scripts/validate_canonical_recovery_contract.py; scripts/check_behavioral_genericity_contract.py; scripts/check_chain_currency.sh; doctrine/chain_currency/retained_bundles.json; doctrine/production_genericity/behavioral_qualification.json; docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md; docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md
reverify: bash scripts/check_doctrines.sh
---

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

Repair and restoration are owned by `RETAINED-BUNDLE-POPULATION-FROZEN` (`.1` retires the literal and the
`reclamations` freeze, `.2` decides what a newly retained key owes the frozen population, `.3` puts the APB
bundle back).

Links: [[chain-currency-doctrine]], [[corpus-canonical-currency-and-ownership]].
