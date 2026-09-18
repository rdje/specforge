---
id: producer-change-and-corpus-rebuild-are-one-transaction
title: Composing a new reader into a registered evidence derivation invalidates every proof-carrying artifact — a producer change and a corpus rebuild are one transaction
answers:
  - "why did a proof-carrying EvidenceIR stop loading after I added a production function (because the function was COMPOSED INTO a registered evidence derivation: the proof binds the derivation topology, so extract_normative_signal_constraints gaining a call edge makes every artifact whose recorded topology no longer matches fail with registered derivation evidence.claim.schema_version.root output or input topology is stale)"
  - "does adding any Rust production function invalidate the persisted corpus (no — only a change to a REGISTERED evidence derivation does; EXTRACTION-QUALITY-GAUGE.3j.2.c.i added four production functions to the LLM-primary grounding path on the same day and every artifact kept loading, because that path is not part of the evidence derivation)"
  - "what does it cost to wire a new deterministic constraint reader (a corpus rebuild in the same transaction — with the reader composed, AXI leaves the measured stratum and the corpus census falls from 5 documents to 4 and from 379 obligations to 144; the change cannot be committed without rebuilding the artifacts whose topology it moves)"
  - "how do I tell whether a producer change invalidated the measured stratum (A/B the composition: remove the call edge and re-load the artifact — with it removed AXI loads 6,451 statements, with it restored the canonical loader refuses; PROOF-SEAL-CURRENCY in the gate tier catches it at commit either way)"
  - "can a producer change and its corpus rebuild be separate slices (no — separating them either commits a change that silently shrinks the measured stratum, or rebuilds artifacts for a producer that is not there yet; the rebuild belongs in the same commit as the composition)"
  - "what does a producer change actually cost (not a window — the rebuild is 0.5s per document for evidence and 0.8s for semantic, so 24 of the 27 rebuild in about a minute; the cost is the three that cannot be rebuilt at all)"
  - "which persisted documents cannot be rebuilt and why (ihi0022_l_2025_08 AXI, ihi0024_e APB and ihi0033_c AHB — they retain no normalized bundle, the retention declaration says retained 24 reclamations 0 and never included them, and the evidence stage reads that bundle; they are three of the four wire-based golds and their only route back is a re-ingest from PDF)"
date: 2026-09-18
status: current
tags: [evidence-ir, proof, derivation, corpus, producer-change, cost-of-change, method]
evidence: docs/tasks/EXTRACTION-GAP-FIX.md (.5a, .5b); crates/specforge/src/ir/evidence.rs (extract_normative_signal_constraints, verified_canonical_proof, load_from_path); docs/decisions/0048-the-persisted-corpus-has-a-measured-and-a-historical-stratum.md; docs/tasks/extraction-quality-gauge/kind-span-successors.md (.3k.9)
reverify: "compose any new reader into extract_normative_signal_constraints, then load a proof-carrying artifact with EvidenceIr::load_from_path — expect 'registered derivation evidence.claim.schema_version.root output or input topology is stale'; remove the call edge and the same artifact loads"
---

The proof a `EvidenceIR` carries binds the **derivation topology** that produced it, not merely its bytes.
So a reader composed into a registered evidence derivation changes what the current binary would derive,
and every artifact whose recorded topology no longer matches stops loading canonically:

```text
EvidenceIR proof verification failed: registered derivation
'evidence.claim.schema_version.root' output or input topology is stale
```

## Measured, by A/B

`EXTRACTION-GAP-FIX.5a` implemented a signal-keyed obligation-row reader and composed it into
`extract_normative_signal_constraints`. AXI's artifact immediately refused to load. With the **call edge
removed** and nothing else changed, the same artifact loads 6,451 statements. With it restored, the
document leaves the measured stratum and the corpus census falls from **5 documents to 4** and from **379
obligations to 144** — a measurement that silently gets smaller rather than an error that announces itself.

## It is the derivation, not the function count

On the same day `EXTRACTION-QUALITY-GAUGE.3j.2.c.i` added **four** production functions and every
proof-carrying artifact kept loading. Those functions are called from `ground_constraint_typed` on the
LLM-primary path, which is not part of the evidence derivation. The trigger is the **call edge into a
registered derivation**, not the existence of new code.

## The consequence for planning

**A producer change and the corpus rebuild it forces are one transaction.** Separating them gives a choice
of two wrong outcomes: commit the producer alone and the measured stratum shrinks without saying so, or
rebuild first and the rebuild is for a producer that does not exist yet. The rebuild — and the re-proof of
the stratum — belongs in the same commit as the composition, with the stratum measured **before** and
proven back to its full size **after**.

## The rebuild is cheap; three documents cannot have one

Measured `2026-09-19`, the rebuild itself is not the cost: `specforge evidence … --dry-run` takes **0.5 s**
and `semantic` **0.8 s** on a bundle-retaining document, so 24 of the 27 measured-stratum documents rebuild
through both stages in about a minute.

The cost is the other **three**. `ihi0022_l_2025_08` (AXI), `ihi0024_e` (APB) and `ihi0033_c` (AHB) retain
**no normalized bundle** — the retention declaration says `retained: 24, reclamations: 0` and simply never
included them — and the evidence stage reads that bundle, failing with *"path does not exist:
…/normalized/….md"*. They cannot be rebuilt at all. Their only route back into the measured stratum is a
re-ingest from PDF, and all three PDFs are under `corpus/`, but re-ingesting rewrites the SourceIR that
`WIRE-BASED-100` holds at `1.000` for exactly those three.

**So a producer change does not cost a window; it costs three of the four wire-based golds** unless that
re-ingest is decided first. `CORPUS-CHAIN-CURRENCY.10` owns the decision and
`EXTRACTION-GAP-FIX.5b` waits on it. `EXTRACTION-QUALITY-GAUGE.3k.9` is in the same position for the same
reason — its fix is at the shared identifier tokenization seam, which the evidence derivation reads — and
`[[measured-stratum-promotion-population]]` describes the stratum all of them must leave intact.
