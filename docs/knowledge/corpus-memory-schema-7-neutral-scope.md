---
id: corpus-memory-schema-7-neutral-scope
title: CorpusMemory schema 7 cannot route extraction by document identity
answers:
  - "why does a prior need two documents' support to act (CorpusMemory::acting_priors, MIN_ACTING_SUPPORT)"
  - "why did AXI's AWTAGOP ready-like hint disappear (a one-document phrase prior no longer acts)"
  - "why does a learned table kind not re-type a bit layout (is_recognized_layout_header)"
  - "Can CorpusMemory infer a protocol family from a filename?"
  - "What is PriorScope in CorpusMemory schema 7?"
  - "Can legacy named-family priors steer current extraction?"
  - "How are old CorpusMemory schemas migrated?"
  - "Did identity-independent prior learning converge?"
  - "Why was the old prior-memory store rejected?"
  - "What current priors are learned without protocol families?"
  - "can the corpus memory store be refreshed (legacy learning inputs)"
  - "what does the corpus memory change on the 31 chains"
  - "what is EXTRACTION-REPRESENTATION-GAPS.54 and .54.a"
date: 2026-10-21
status: current
tags: [genericity, prior-memory, learning, schema, validation]
evidence: crates/specforge/src/ir/prior_memory.rs; crates/specforge/src/commands/learn_priors.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; docs/decisions/0036-prior-memory-is-identity-independent.md; docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6d.ii.d.i)
reverify: "cargo test --workspace --exclude specforge-production-graph --lib ir::prior_memory && cargo test --workspace --exclude specforge-production-graph --lib commands::learn_priors && bash scripts/check_chain_currency.sh --check"
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

**`2026-10-21` (`EXTRACTION-REPRESENTATION-GAPS.54`).** The store is still `a416cc8b…`, pinned by
`doctrine/production_genericity/behavioral_qualification.json`, but its refresh cannot run: 8 of its 13 learning
inputs are now legacy schema-1 IntentIR, which `learn-priors` refuses. On the 31 chains it changes two evidence records
(a stale AXI `AWTAGOP` ready-like hint, and one duplicate AHB5 declaration it suppresses); a store relearned from the
31 chains would drift in every family and move four chains. The refresh, with re-declared inputs and a re-pinned
qualification, is owned by `.54.a`.

**`2026-10-21` (`.54.a`-`.54.a.i.a`).** All 31 current chains are accepted learning inputs, but nearly every prior in
both the pinned store and a relearned one rests on a single document. Such a prior replays that document's own past
reading back to it, so ADR 0036's fixed point kept the `AWTAGOP` hint. Table-shape priors learn the converter's raw
kind, so USB 3.2's mistyped status tables taught `bit | description` -> signal description to its own bit layouts. Now
every stage acts on `CorpusMemory::acting_priors()`, the priors with `support_count >= MIN_ACTING_SUPPORT` (2). The
rule counts documents, never reading the current one, and the store keeps every prior as provenance.
`effective_table_kind` gives no learned kind to a header a layout reader recognizes (`is_recognized_layout_header`).
On the 31 chains only AXI moves (the hint and its interface tags). A store relearned from all 31 chains is
EvidenceIR-identical to the pinned one under the rule, so the refresh (`.54.a.ii`) re-pins an equivalent store.
