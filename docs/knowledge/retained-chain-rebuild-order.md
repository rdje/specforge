---
id: retained-chain-rebuild-order
title: A retained chain must be validated stage by stage as it is rebuilt, not at the end
answers:
  - "in what order do I rebuild a retained corpus chain"
  - "why does validate fail with cumulative proof ledger does not retain the exact verified upstream prefix"
  - "why did IntentIR proof verification fail after I validated SemanticIR"
  - "does specforge validate change the artifact it validates"
  - "when do I run specforge validate during a chain rebuild"
  - "do I validate the isf adapter artifact after a rebuild"
  - "how do I rebuild every retained chain after a production change"
  - "why is my rebuilt chain stale even though the content matches"
  - "why does chain currency fail after regenerating EvidenceIR to score WIRE-BASED-100 with the HEAD binary"
  - "why did PROOF-SEAL-CURRENCY refuse a commit after I stashed a code change (its rebuilt chain is still on disk)"
date: 2026-08-27
status: current
tags: [chain-currency, proof-ledger, validation, corpus, operations, rebuild]
evidence: crates/specforge/src/commands/validate.rs; crates/specforge/src/ir/intent.rs (load_with_verified_proof); scripts/check_chain_currency.sh; docs/book/src/reference/extraction-architecture.md ("each stage must be built and validated before the next stage is built"); docs/tasks/spec-to-intent-alignment/residual-actionability.md (.8c)
reverify: "For any retained key, run `specforge semantic <evidence_ir.json>` then `specforge intent <semantic_ir.json>` then `specforge validate <semantic_ir.json>` then `specforge validate <intent_ir.json>`; the last command fails with `cumulative proof ledger does not retain the exact verified upstream prefix`. Re-running with validate before the downstream build succeeds."
---

**Established `2026-08-27` (`SPEC-TO-INTENT-ALIGNMENT.8c`).** `specforge validate` is not read-only. It
back-annotates `validation_reports` into the artifact it validates, and that back-annotation is a *registered
mutation*: the artifact's proof ledger is extended and its verified identity changes.

A downstream stage pins the exact verified ledger of the upstream artifact it was built from, as an ordered
prefix. So building the whole chain first and validating it afterwards produces an IntentIR whose prefix is the
*pre-validation* SemanticIR ledger, while the SemanticIR on disk now carries the post-validation one. Validating
the IntentIR then fails closed with:

```text
IntentIR proof verification failed: cumulative proof ledger does not retain the exact verified upstream prefix
```

The content is fine; the authority is not. Nothing is silently wrong — the gate refuses — but the whole chain
has to be rebuilt.

The correct per-document order interleaves build and validate:

```bash
specforge semantic generated/evidence_ir/<key>/evidence_ir.json
specforge validate generated/semantic_ir/<key>/semantic_ir.json
specforge intent   generated/semantic_ir/<key>/semantic_ir.json
specforge validate generated/intent_ir/<key>/intent_ir.json
specforge adapt    generated/intent_ir/<key>/intent_ir.json --target isf
specforge validate generated/adapters/isf/<key>/adapter.json
```

The adapter is validated too, last, since nothing is built from it (corrected `2026-09-28`,
`EXTRACTION-REPRESENTATION-GAPS.3.f`: this card said the adapter had no `validate`, while `validate` detects an
adapter artifact and every chain rebuilt that month carries its back-annotation and a refreshed
`validation_report.json` beside it). The book states
the same rule in one sentence — "each stage must be built and validated before the next stage is built" — in the
[extraction-architecture reference](../book/src/reference/extraction-architecture.md); this card exists so the
failure message itself leads back to it.

`scripts/check_chain_currency.sh` is the authority on whether the result is current. Its comparison excludes
`validation_reports`, `proof_context`, and `proof_ledger` from *content* identity, but every downstream replay
still enters through the upstream stage's canonical verified loader, so a mis-ordered rebuild fails there rather
than passing quietly.

**The same trap, one stage up (`2026-09-25`, `EXTRACTION-REPRESENTATION-GAPS.27.b`).** Scoring WIRE-BASED-100 with
the HEAD binary needs HEAD's EvidenceIR on disk, because `eval-extraction` replays proofs with the running binary.
Regenerating it for that run, and then regenerating the new one, leaves EvidenceIR content-identical but with a new
ledger, so the SemanticIR, IntentIR and adapter built before it no longer hold its prefix, and
`check_chain_currency.sh` fails at the adapter. After any such detour, rebuild every downstream stage in the order
above before running currency.

**The same trap, sideways (`2026-10-19`, `EXTRACTION-REPRESENTATION-GAPS.48.a`).** Setting a code change aside with
`git stash` to land something else first leaves the chain that change rebuilt in `generated/`. The HEAD build then
refuses it — `PROOF-SEAL-CURRENCY` fails the commit with *registered derivation 'evidence.claim.schema_version.root'
output or input topology is stale* — because the artifact carries fields the HEAD code does not know. Snapshot the
chain before rebuilding, restore the snapshot before committing the other change, and rebuild again after the
stash comes back; the rebuild is deterministic, so the second one is byte-identical in content to the first.
