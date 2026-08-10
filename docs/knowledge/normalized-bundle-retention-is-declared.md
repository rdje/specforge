---
id: normalized-bundle-retention-is-declared
title: Normalized bundles are retained by declaration, and the declaration is gated
answers:
  - "should a corpus refresh clean the normalized bundle when it finishes"
  - "why does SpecForge keep normalized bundles instead of reclaiming them"
  - "where is the set of retained normalized bundles declared"
  - "what happens if someone runs clean --scope source-normalized on a declared document"
  - "how does a deliberate normalized-bundle reclamation get authorized"
  - "how does the measurable corpus population grow"
  - "how much disk do the retained normalized bundles cost"
  - "why is a retained bundle count not the same as corpus refresh progress"
date: 2026-08-10
status: current
tags: [corpus, retention, doctrine-enforcement, artifacts, lifecycle, currency]
evidence: doctrine/chain_currency/retained_bundles.json; scripts/check_chain_currency.sh; docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md; docs/tasks/CORPUS-CHAIN-CURRENCY.md (.2)
reverify: "bash scripts/check_chain_currency.sh --self-test && find generated/source_ir -maxdepth 2 -type d -name normalized | wc -l"
---

A refresh **keeps** its normalized bundle. Routine `clean --scope source-normalized` is retired from the
refresh routine by ADR 0025 decision 3, because retention is exactly what makes a document replayable at
the evidence stage: `EvidenceIr::build_with_prior_memory` resolves
`normalization_plan.promoted_markdown_path`, which lives inside `normalized/`. Every later stage reads
only the persisted JSON one stage upstream, so a reclaimed bundle costs measurability at that one stage
and nowhere else — and a document that cannot be replayed cannot be proven current.

The retained set is **declared, not inferred**: `doctrine/chain_currency/retained_bundles.json` names
every `document_key` whose bundle must be present. `CHAIN-CURRENCY` compares that declaration with the
bundles actually on disk and fails closed both ways — a declared bundle that has vanished is an
unauthorised reclamation, and a bundle on disk that no leaf declared is a refresh that never recorded
what it kept. The measurable population therefore grows exactly one document per refresh, deliberately.

Reclamation stays available but becomes accountable: drop the key from `retained` and add a
`reclamations` record naming the `owning_leaf`, `date`, and `reason`, in the task that decides it. The
declaration is schema-closed — unknown, missing, mistyped, unsorted, duplicated, or self-contradictory
fields are breaches — so it cannot decay into free-form prose that no longer means anything.

The cost was measured before the policy was adopted, not assumed: 22 bundles occupy about 1.4 GB against
3.4 TB free, and the full 78-document corpus extrapolates to roughly 4.7 GB (~0.14% of available space).

Retention is still **not** the progress measure. A completed refresh means a verified current-binary
EvidenceIR and downstream chain; the bundle count is the separate *measurability* census, which
`check_chain_currency.sh` reports as its own number. See
[[corpus-refresh-completion-vs-normalized-retention]] and [[chain-currency-doctrine]].
