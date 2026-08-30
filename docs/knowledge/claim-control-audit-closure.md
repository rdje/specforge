---
id: claim-control-audit-closure
title: Published-claim controls bind exact known-bad evidence and reject scratch producers
answers:
  - "how many falsification controls are cited by verified SpecForge claims"
  - "how many governed claim producers are tracked"
  - "which claim control-audit fields are gated and which must be read from the report"
  - "how does SpecForge prove a cited self-test contains a known-bad RED case"
  - "how does the claim gate find ignored or untracked scratch producers"
  - "which claim control needed a known-bad repair in CLAIM-VERIFICATION-ADOPTION.4"
date: 2026-08-30
status: current
tags: [claim-verification, falsification, provenance, git, doctrine, task-tree]
evidence: doctrine/claim_verification/claims.jsonl; scripts/check_claim_verification.pl; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.4)
reverify: perl scripts/check_claim_verification.pl --self-test && perl scripts/check_claim_verification.pl --check && perl scripts/check_claim_verification.pl --report
---

`CLAIM-VERIFICATION-ADOPTION.4` closes two reverse joins left deliberately open by the initial provenance gate.
Every falsification control cited by a verified claim names a perturbation and an expected RED diagnostic and
binds them to an exact line-range/SHA-256 region in its tracked producer. The gate executes the complete command
and validates the narrower region, so a self-test name cannot survive removal or substitution of the cited
known-bad case. How many controls and producers there are is a repository-derived count nothing fails on: read
`control_audit.cited_controls`, `.exact_red_evidence`, and `.governed_producers` from `--report`.

At the `.4` boundary all but one cited control already executed a mutation suite. `independent-catalog-feasibility` was the repair frontier: it
computed the current full-catalog bound but had no known-bad path in that cited command. It now reuses the same
feasibility predicate with a controlled line ceiling one below the computed requirement and refuses to publish
PASS unless that mutation produces the expected RED diagnostic.

The durability audit also derives every producer used by verified source, control, and stale-check commands. It
asks Git independently for ignored and untracked producer-shaped files under `scripts/`, `doctrine/`, `docs/`,
and `.github/`; either result is a gate failure. Two invariants therefore hold by construction rather than by
observation, and those are the ones worth publishing: `ignored_candidates` and `untracked_candidates` are
**zero**, because a nonzero value raises `governed producer census contains ignored or untracked candidates`;
and `cited_controls` equals `exact_red_evidence`, because a cited control without an exact RED region fails.
Their common value is not an invariant. The `.4` boundary read seven / seven / six with both censuses at zero —
a dated observation, superseded on read by `--report` (`CLAIM-VERIFICATION-ADOPTION.11`).
