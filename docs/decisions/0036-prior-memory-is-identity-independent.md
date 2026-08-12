---
id: prior-memory-is-identity-independent
title: Prior memory is identity-independent, validation-gated, and fail-closed across migration
date: 2026-08-12
status: accepted
scope: genericity, prior-memory, learning, schema, validation, residual-honesty
evidence: crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; docs/research/production-genericity-pipeline-audit.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
answers:
  - "can prior memory select extraction by filename or protocol family"
  - "what replaced ProtocolFamily in CorpusMemory schema 7"
  - "what happens when old identity-scoped prior memory is loaded"
  - "how is the CorpusMemory feedback loop made reproducible"
  - "can learned priors remain useful without knowing a specification family"
---

# ADR 0036: Prior memory is identity-independent, validation-gated, and fail-closed across migration

## Context

CorpusMemory schemas 1 through 6 stored a named protocol family on seven prior families. Production inferred that
family from `document_key` and `display_name`, then used it to select EvidenceIR and SemanticIR guidance. Renaming
an otherwise identical input could therefore change extraction.

The persisted schema-6 store also proved stale and non-reproducible: it declared 14 accepted sources, one of which
no longer existed, while none of the 13 current sources carried the validation backannotation required by the
learning policy. Preserving its records would retain unauditable authority.

## Decision

1. CorpusMemory schema 7 has one identity-independent `PriorScope::Global`. Document key, display name, path,
   vendor, and protocol family are provenance only and cannot select a prior or lookup path.
2. `learn-priors`, EvidenceIR, SemanticIR, and validation use the global scope. Reusable keys remain normalized
   current-document phrases, structural header/caption signatures, typed source kind, role, and grounded support;
   extraction-profile priors remain keyed by structural fingerprints.
3. A schema-7 scope other than `global` fails closed. Schemas 1 through 6 load only through a compatibility
   firewall: their seven identity-scoped semantic prior families are quarantined, not relabelled as global truth.
   Their identity-independent structural extraction profiles may survive. Future schemas reject.
4. Learning inputs must carry current persisted IntentIR validation. A prior-store refresh validates the declared
   current inputs, relearns, replays every affected chain, validates changed learning inputs, and relearns until
   the store is byte-stable. Failure to converge is a release blocker, not a reason to restore identity routing.
5. Cross-document disagreement is global and explicit. `contested_priors()` surfaces competing values and never
   uses a family partition to hide or resolve the conflict.

## Consequences

- The same PDF produces the same prior lookup behavior after filename, title, or document-key perturbation.
- Old family-scoped memory cannot be laundered into current neutral authority. Recall can change, but every new
  prior must be rebuilt from current validated inputs under the neutral schema.
- The 2026-08-12 migration converged in two iterations. Thirteen validated inputs produce 29 actor, 83 semantic,
  four modality, 443 temporal, zero table, 1,251 visual, 11 negative-knowledge, and two structural-profile priors.
  A third learn is byte-identical at SHA-256 `a416cc8b7b6bd84947e4b6780d5253b13c2985c57ca4e8f1c02d6daf56239633`.
- One global actor-term contradiction is now visible instead of family-isolated. It remains contested.
- This closes identity-based prior selection only. Signal-spelling authority, prompts/corpus organization, the
  structural doctrine, and behavioral qualification remain release blockers in `.6d.ii.d.ii` through `.f`.

## Links

- Genericity invariant: [`0006-no-hardcoded-chip-spec-vocabulary.md`](0006-no-hardcoded-chip-spec-vocabulary.md)
- Generic EvidenceIR boundary: [`0035-protocol-evidence-is-generic-and-document-derived.md`](0035-protocol-evidence-is-generic-and-document-derived.md)
- Task tree: [`SPEC-TO-INTENT-ALIGNMENT.md`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Pipeline audit: [`production-genericity-pipeline-audit.md`](../research/production-genericity-pipeline-audit.md)
