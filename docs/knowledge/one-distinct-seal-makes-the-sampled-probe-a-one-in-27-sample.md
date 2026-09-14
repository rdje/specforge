---
id: one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample
title: The corpus carries ONE distinct proof seal per stage across 27 artifacts, so the gate-tier seal probe samples 1 in 27 — and it passed while the intent stage refused a wire-gold document
answers:
  - "how long does check_chain_currency.sh take"
  - "how long does check_proof_seal_currency.sh --total take"
  - "why did check_doctrines.sh --all not finish in 50 minutes"
  - "how many distinct proof seals does the corpus carry"
  - "why does PROOF-SEAL-CURRENCY pass while a document is refused by its consumer"
  - "what is the sample size of the gate-tier seal probe"
  - "can a load-refusing artifact pass the gate"
  - "which is cheaper, a seal check or a chain-currency replay"
  - "should PROOF-SEAL-TOTAL be raised to gate tier"
  - "how much does it cost to probe every artifact at the semantic and intent stages"
  - "would a topology-bearing seal key separate a refusing document"
  - "why does the per-stage total seal probe ship inert"
  - "what turns on the total probe at semantic and intent"
  - "what does registered derivation output or input topology is stale mean"
  - "how do I find out which documents are chain-stale"
  - "which documents in the corpus are currently stale"
date: 2026-09-14
status: current
tags: [doctrine, chain-currency, proof-seal, corpus, cost, corpus-chain-currency]
evidence: scripts/check_chain_currency.sh; scripts/check_proof_seal_currency.sh; docs/tasks/CORPUS-CHAIN-CURRENCY.md (.4, .5); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.4c)
reverify: "bash scripts/check_proof_seal_currency.sh --total — expect '1 distinct seal(s)' at source-ir, evidence, semantic, intent and isf-adapter, and a TOTAL probe that refuses the documents the sampled tier accepts. Budget 19 minutes; budget 28 for bash scripts/check_chain_currency.sh. Run both detached."
---

Two CI-tier doctrines answer "is the persisted corpus still what this build produces". Measured on a
warm build, `2026-09-14`:

| check | what it proves | cost |
| --- | --- | ---: |
| `check_chain_currency.sh` | the persisted artifact is the CONTENT the current binary reproduces | **28m00s** |
| `check_proof_seal_currency.sh --total` | every persisted artifact's seal is ACCEPTED by the current build's canonical loader | **18m45s** |
| `check_proof_seal_currency.sh` (gate tier) | the same, for one representative per distinct seal | seconds |

Together the two CI-tier runs are ≈ 47 minutes, which is the whole of "`check_doctrines.sh --all` did
not finish in 50 minutes". Neither is affordable per commit; both are affordable per push, which is
where the CI policy already puts them.

## The sampled tier is a 1-in-27 sample, because the corpus has one seal

`PROOF-SEAL-CURRENCY`'s gate tier probes **one representative per distinct seal per stage**, on the
stated bet that a divergent document raises the distinct count and so earns its own probe. The census
measures the actual count:

```text
source-ir   27/27 sealed, 1 distinct seal   evidence  27/27 sealed, 1 distinct seal
semantic    27/27 sealed, 1 distinct seal   intent    27/27 sealed, 1 distinct seal
```

One seal per stage means **one probe for twenty-seven artifacts**. On `2026-09-14` that gate passed in a
tree where `specforge intent …/ihi0024_e…apb/semantic_ir.json --dry-run` was REFUSED
(`registered derivation 'semantic.claim.schema_version.root' output or input topology is stale`), and
`--total` refused two artifacts in the same working tree with the right diagnostic — *"this is NOT the
stale-seal diagnostic … find out why the loader refuses this artifact"*.

The refusal is a property of the artifact's recorded **derivation topology**, and the seal digest does
not vary with it. Folding the topology into the census key looks like the cheap repair and **is not**:
measured, that key gives 27 distinct keys for 27 documents at every stage — root derivations included,
because each root's `output_sha256`/`inputs_sha256` are digests over its own document's content. A
topology-bearing key *is* `--total`.

**The lever is the per-stage tier.** `--total`'s 18m45s is not spread evenly: an accepted
`intent --dry-run` costs **1.2 s** and a refusal **0.24 s**, while the evidence and source-ir probes
replay extraction from the normalized bundle. Probing every one of the 27 artifacts at just two stages
costs **30.3 s** (semantic) + **35.5 s** (intent) = **66 s**, and finds every refusal the corpus
currently has. Sampling is right at source-ir and evidence and wrong at semantic and intent.

The per-stage mechanism is shipped (`probe_scope_for`, self-tests 17 and 17b) and **inert**:
`TOTAL_PROBE_STAGES` defaults to empty because activating it fails the gate on a corpus that is
genuinely broken and not yet repaired — APB-e needs a rebuild and I2C a re-ingest (its normalized bundle
is reclaimed). A gate that fails closed over a broken corpus is correct and unlandable at the same time,
so the mechanism lands and the activation waits (`CORPUS-CHAIN-CURRENCY.7`). Activation is one constant.

## What the sweep found, and the general law under it

- `um10204…i2c` — evidence CONTENT stale (`signal_constraints` 9 → 3, `fact_provenance` 21 → 15);
  everything downstream blocked. Reproduces at `1ada364a`.
- `ihi0024_e…apb` — semantic CONTENT stale (`interface_signal_conflicts` 0 → 1, `PADDRCHK` loses its
  `width_hint`) and REFUSED by `intent` and `isf-adapter`. Reproduces at `956fbcce`; it is
  `SIGNAL-DECLARATION-ROW-DROP.4a`'s effect, tracked as `.4c`. **The conflict is real**: the document
  states `ceil(ADDR_WIDTH/8)` in `Table 5-1` and `ADDR_WIDTH/8` in its version matrix. The rebuild costs
  one SemanticIR `width_hint` and **no emitted `.isf` byte** — that signal already ships
  `(output PADDRCHK (width 1))`, as do 31 of APB-e's 32.

**A leaf that changes a reader moves every document it does not rebuild**, and only a CI-tier sweep sees
it. Both drifts here were found by a detached run, not by any gate, and both had been in the tree for
days. When a slice changes a reader, the documents it did not rebuild are the ones to check.

Links: [[chain-currency-doctrine]], [[persisted-chain-currency-is-measured-not-assumed]].
