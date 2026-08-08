---
id: mdbook-current-truth-drift-lock
title: Two mdBook current-state facts are mechanically locked to their code seams
answers:
  - "does the ISF adapter lower actor-relative direction"
  - "does constrained contract extraction ship code"
  - "what prevents the actor direction and extract-contracts book claims from drifting"
  - "where is the mdBook current-truth verifier"
date: 2026-08-08
status: current
tags: [mdbook, documentation, direction, extraction, doctrine]
evidence: scripts/check_book_current_truth.sh
reverify: bash scripts/check_book_current_truth.sh
---

The `.isf` adapter selects a structurally grounded initiator from `IntentIR.actor_ports` and lowers
that actor's unambiguous graph directions; only unresolved signals use the compatibility fallback.
The constrained-extraction design also has a live producer: `specforge extract-contracts` is declared
by the CLI and dispatched to `commands::extract_contracts::run`.

`scripts/check_book_current_truth.sh` binds those code seams to the current mdBook statements, rejects
the two superseded formulations, and requires links to the detailed ISF-direction and live-command
chapters. The `shipped_behavior` maintained-reference record executes this verifier through
`LIVE-DOC-SIZE`, so future code or prose changes must reconcile the contract in the same commit.
