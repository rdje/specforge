---
id: claim-control-audit-closure
title: Published-claim controls bind exact known-bad evidence and reject scratch producers
answers:
  - "how many falsification controls are cited by verified SpecForge claims"
  - "how many governed claim producers are tracked"
  - "how does SpecForge prove a cited self-test contains a known-bad RED case"
  - "how does the claim gate find ignored or untracked scratch producers"
  - "which claim control needed a known-bad repair in CLAIM-VERIFICATION-ADOPTION.4"
date: 2026-08-15
status: current
tags: [claim-verification, falsification, provenance, git, doctrine, task-tree]
evidence: doctrine/claim_verification/claims.jsonl; scripts/check_claim_verification.pl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.4)
reverify: perl scripts/check_claim_verification.pl --self-test && perl scripts/check_claim_verification.pl --check && perl scripts/check_claim_verification.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.4` closes two reverse joins left deliberately open by the initial provenance gate.
The verified registry cites seven falsification controls across six unique command producers. Every control now
names a perturbation and expected RED diagnostic and binds them to an exact line-range/SHA-256 region in its
tracked producer. The gate executes the complete command and validates the narrower region, so a self-test name
cannot survive removal or substitution of the cited known-bad case.

Six controls already executed mutation suites. `independent-catalog-feasibility` was the repair frontier: it
computed the current full-catalog bound but had no known-bad path in that cited command. It now reuses the same
feasibility predicate with a controlled line ceiling one below the computed requirement and refuses to publish
PASS unless that mutation produces the expected RED diagnostic.

The durability audit also derives every producer used by verified source, control, and stale-check commands. It
asks Git independently for ignored and untracked producer-shaped files under `scripts/`, `doctrine/`, `docs/`,
and `.github/`; either result is a gate failure. The frozen result is seven cited controls, seven exact RED
regions, six governed producers, zero ignored candidates, and zero untracked candidates.
