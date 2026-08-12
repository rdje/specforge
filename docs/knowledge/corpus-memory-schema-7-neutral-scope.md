---
id: corpus-memory-schema-7-neutral-scope
title: CorpusMemory schema 7 cannot route extraction by document identity
answers:
  - "Can CorpusMemory infer a protocol family from a filename?"
  - "What is PriorScope in CorpusMemory schema 7?"
  - "Can legacy named-family priors steer current extraction?"
  - "How are old CorpusMemory schemas migrated?"
  - "Did identity-independent prior learning converge?"
  - "Why was the old prior-memory store rejected?"
  - "What current priors are learned without protocol families?"
date: 2026-08-12
status: current
tags: [genericity, prior-memory, learning, schema, validation]
evidence: crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; docs/decisions/0036-prior-memory-is-identity-independent.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6d.ii.d.i)
reverify: "cargo test -p specforge --lib ir::prior_memory && cargo test -p specforge --lib commands::learn_priors && bash scripts/check_chain_currency.sh --check"
---

CorpusMemory schema 7 has only `PriorScope::Global`. EvidenceIR and SemanticIR prior loading no longer receive a
document key or display name, and `learn-priors` never infers a vendor or protocol family. Persisted document
identity remains provenance and cannot select extraction behavior.

The compatibility boundary is fail-closed. A schema-7 scope other than `global` rejects. Schemas 1 through 6 have
their seven identity-scoped semantic prior families quarantined before typed deserialization; those records are
not relabelled as global. Identity-independent structural extraction profiles may survive, and future schemas
reject.

The old schema-6 store was not reproducible: it named 14 accepted artifacts including one deleted source, while
the 13 current sources lacked the persisted validation required by its own policy. The current inputs were
validated and relearned under schema 7. After one downstream replay changed one learning residual, the second
learn produced 29 actor, 83 semantic, four modality, 443 temporal, zero table, 1,251 visual, 11 negative, and two
structural-profile priors; a third learn was byte-identical at SHA-256
`a416cc8b7b6bd84947e4b6780d5253b13c2985c57ca4e8f1c02d6daf56239633`.

The later proof migration leaves 24 documents measurable through current EvidenceIR; the other 54 are an explicit
legacy/proofless frontier rather than current replay claims. The neutral store can influence a document only
through matching normalized current-document evidence or a structural fingerprint, and schema-3 EvidenceIR
captures the exact prior used by its executable replay. One globally contested actor term remains unresolved.
