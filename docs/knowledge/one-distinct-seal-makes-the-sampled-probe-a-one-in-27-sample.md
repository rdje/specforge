---
id: one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample
title: The corpus carries ONE distinct proof seal per stage across 27 artifacts, so a per-seal probe samples 1 in 27 — it passed while the intent stage refused a wire gold, and semantic/intent are now probed TOTALLY
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
  - "is the per-stage total seal probe active or inert"
  - "what does activating the per-stage total probe cost the gate"
  - "why does a self-test that passes its own configuration not test the default"
  - "would a topology-bearing seal key separate a refusing document"
  - "what turned on the total probe at semantic and intent"
  - "what does registered derivation output or input topology is stale mean"
  - "how do I find out which documents are chain-stale"
  - "which documents in the corpus are currently stale"
  - "which binary does check_proof_seal_currency.sh probe with"
  - "why is a debug probe 14 seconds and a release probe 1.2"
  - "which cargo profile do the corpus replay checks build"
  - "does a debug build and a release build accept the same persisted artifact"
  - "why is scripts/lib/corpus_replay_binary.sh a shared predicate"
  - "how long does a cold release build of this workspace take"
  - "does a clean checkout or a CI runner pay for the corpus replay build"
  - "how much of the corpus replay cost is a few large artifacts"
date: 2026-09-14
status: current
tags: [doctrine, chain-currency, proof-seal, corpus, cost, build-profile, corpus-chain-currency]
evidence: scripts/check_chain_currency.sh; scripts/check_proof_seal_currency.sh; scripts/lib/corpus_replay_binary.sh; docs/tasks/CORPUS-CHAIN-CURRENCY.md (.4, .5, .8, .9); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.4c)
reverify: "bash scripts/check_proof_seal_currency.sh --total — expect '1 distinct seal(s)' at source-ir, evidence, semantic, intent and isf-adapter, and a TOTAL probe that names any document its own loader refuses. Budget 2 minutes at the release profile the check now builds; budget 13 for bash scripts/check_chain_currency.sh. Run the second detached."
---

Two CI-tier doctrines answer "is the persisted corpus still what this build produces". Measured on a
warm build, `2026-09-14`, **before and after `CORPUS-CHAIN-CURRENCY.8` moved the replay profile from
debug to release**:

| check | what it proves | debug | release |
| --- | --- | ---: | ---: |
| `check_chain_currency.sh` | the persisted artifact is the CONTENT the current binary reproduces | 28m00s | **12m38.2s** |
| `check_proof_seal_currency.sh --total` | every persisted artifact's seal is ACCEPTED by the current build's canonical loader | 18m45s | **1m59.2s** |
| `check_proof_seal_currency.sh` with every stage sampled | the same, for one representative per distinct seal | 15.9 s | **7.4 s** |
| `check_proof_seal_currency.sh` **as it now ships** (gate tier) | the same, probing semantic and intent TOTALLY | — | **1m12.7s** |

The two CI-tier runs together were ≈ 47 minutes — the whole of "`check_doctrines.sh --all` did not
finish in 50 minutes" — and are now **14m37s**. Still not per-commit; comfortably per-push, which is
where the CI policy already puts them.

## A per-seal probe is a 1-in-27 sample, because the corpus has one seal

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

**The lever is the per-stage tier.** `--total`'s cost is not spread evenly: an accepted
`intent --dry-run` costs **1.2 s** and a refusal **0.24 s**, while the evidence and source-ir probes
replay extraction from the normalized bundle. Probing every one of the 27 artifacts at just two stages
costs **30.3 s** (semantic) + **35.5 s** (intent) = **66 s**, and finds every refusal the corpus
currently has. Sampling is right at source-ir and evidence and wrong at semantic and intent.

## Always say which binary a probe cost was measured with — and since `.8` there is one answer

The 66 s above was `target/release/specforge`; the check built `target/debug/specforge`, where the same
sweep costs **12m57.9s**. `CORPUS-CHAIN-CURRENCY.8` measured both profiles cold and warm and moved all
three corpus-replay entrypoints — `check_chain_currency.sh`, `check_proof_seal_currency.sh` and the
`rebuild_stage_cascade.sh` remedy — to the **release** profile through one shared predicate,
`scripts/lib/corpus_replay_binary.sh`. They each carried their own copy of the build before; a gate and
a remedy that disagree about *which loader answers* leave a debt no compliant work can clear, which is
the same `.11` reason the seal read itself lives in one file.

**The verdict is profile-independent and that was measured, not argued.** Proof verification is digest
comparison and ordered-map lookup (`verify_ledger`/`validate_claim`, `crates/specforge/src/ir/derivation.rs`);
the workspace has **no** `cfg(debug_assertions)` and the only two `debug_assert!`s on the proof path
assert compile-time constants. Both profiles report 27 of 27 accepted at semantic and at intent, and
AXI's **43,419,318-byte** `intent --dry-run` is byte-identical between them at the same SHA-256.

**The cost is not.** One `intent --dry-run` over a 39.7 MB artifact is **63.1 s** at debug and **6.5 s**
at release; over a 15.9 KB one it is 0.00 s at both. The thirteen minutes were never spread over the
corpus — they were a few large artifacts deserialized by an unoptimized build.

Three things that "keep debug because the build is cheaper" had wrong, each measured:

- a **cold** release build of this workspace is **36.1 s**, not minutes — the dependency set is six crates;
- it is never paid on a clean checkout or a hosted CI runner, because all three entrypoints skip on an
  absent `generated/` **before** the build (**0.031 s, no cargo invocation at all**);
- the release tree is the **smaller** one — 234 MB against debug's 1.1 GB.

A build is a fixed cost and a probe is a per-document one, so the debug argument was right at four
probes and wrong at fifty-four; the crossover is about two large documents.

## The per-stage TOTAL probe is ACTIVE, and the default itself is now under a control

`TOTAL_PROBE_STAGES` defaults to `'semantic intent'` since `CORPUS-CHAIN-CURRENCY.9` (`2026-09-14`).
Both obstacles that held it were closed by measurement: `.7` rebuilt both refusing documents (APB-e, and
I2C from its **retained** bundle — it was never reclaimed), and `.8` moved the replay profile.

Measured before and after on the same tree:

| | sampled everywhere | semantic + intent TOTAL |
| --- | ---: | ---: |
| `check_proof_seal_currency.sh` | 7.4 s | **1m12.7s** |
| the whole gate-tier driver | 4m13.0s | **5m30.1s**, and 5m24.5s on a second sample (+72-77 s, ~+29%) |

`source-ir` and `evidence` stay sampled — their probes replay extraction — and the check still prints
that it is blind there; `--total` (1m59.2s, CI tier) closes them.

**`.6`'s controls proved the mechanism, not the shipped value.** Self-test 17b passes the stage set in
explicitly, so while the default was empty nothing tested it, and an edit that quietly emptied it would
have restored a 1-in-27 sample under a green gate. **Case 21** passes no override and requires the
default to catch a divergent same-seal document at `semantic`. Its RED is exact: with
`SPECFORGE_PROOF_SEAL_TOTAL_STAGES=''` the suite reports **20/21**, failing only on *"self-test 21: the
DEFAULT stage set passed over a document its own loader refuses"*.

The general rule this leaf leaves behind: **a control that passes its own configuration in tests the
mechanism, never the shipped default.** If a default carries the risk, one control must read it.

**The class was then enumerated rather than left as a warning.** Over every `check_*.sh` plus
`rebuild_stage_cascade.sh` carrying a `--self-test`, and covering both `${VAR:-d}` and `${VAR-d}` (a
first pass matched only the former and missed the very variable in question), the complete population of
shipped defaults is seven: `SPECFORGE_PROOF_SEAL_TOTAL_STAGES` is the only one that changes what a check
*proves*; three `*_GENERATED_ROOT` and one `*_RETENTION_CONTRACT` default to PATHS the gate exercises
against the real corpus every commit; three `*_BIN`/`*_WORK` default to empty, the ordinary path. There
is no second unguarded instance.

## What the sweep found, and the general law under it

- `um10204…i2c` — evidence CONTENT stale (`signal_constraints` 9 → 3, `fact_provenance` 21 → 15);
  everything downstream blocked. Reproduces at `1ada364a`. **Rebuilt `2026-09-14`**, and the eight
  removed constraints were fabrications the `.3k` series had already refused elsewhere.
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
