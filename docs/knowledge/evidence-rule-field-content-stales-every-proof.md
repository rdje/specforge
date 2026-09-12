---
id: evidence-rule-field-content-stales-every-proof
title: Changing the content of an existing EvidenceIR rule field stales every persisted proof without moving the ruleset seal, and one probe per distinct seal cannot see it
answers:
  - "why does eval-extraction refuse every gold with 'evidence.claim.schema_version.root output or input topology is stale'"
  - "does adding a sub-field to extraction_manifest un-seal the persisted corpus"
  - "is avoiding a new EVIDENCE_RULE_FIELDS entry enough to keep persisted proofs valid"
  - "why did only four documents refuse to load after an EvidenceIR producer change"
  - "why did check_proof_seal_currency.sh pass while four artifacts would not load"
  - "is one canonical probe per distinct seal a sample"
  - "what does check_proof_seal_currency.sh --total do"
  - "why is the total proof-seal probe CI-tier rather than gate-tier"
  - "what is replay_bytes and what depends on it"
  - "how do I rebuild the AXI APB AHB chains when their normalized bundles are held out"
  - "path does not exist normalized/<key>.md when running specforge evidence"
  - "where are the held-out APB AXI AHB normalized bundles kept"
  - "can rebuild_stage_cascade.sh land a deliberate content change"
  - "how do I re-score the WIRE-BASED-100 golds"
date: 2026-09-11
status: current
tags: [evidence-ir, proof-kernel, chain-currency, gates, wire-based-100, adr-0025, signal-declaration-row-drop]
evidence: crates/specforge/src/ir/evidence.rs (public_field_values; EVIDENCE_RULE_FIELDS; the `evidence.current-replay` and `evidence.claim.<field>.root` registered derivations); crates/specforge/src/ir/derivation.rs (verify_premise, the RegisteredDerivation arm); scripts/check_proof_seal_currency.sh (header, "one per distinct seal per stage"); scripts/rebuild_stage_cascade.sh (the unattributed-delta stop); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.1b, .1c)
reverify: "for f in generated/evidence_ir/*/evidence_ir.json; do ./target/release/specforge semantic \"$f\" --dry-run >/dev/null 2>&1 || echo REFUSED $f; done — expect no output; then ./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_apb.json --provider skip and expect signal_constraint F1=1.000 (tp=6 fp=0 fn=0) in the WIRE-BASED-100 block"
---

There are **two** ways an EvidenceIR producer change stales the persisted corpus, and guarding against
one of them proves nothing about the other.

1. **Adding a field** to `EVIDENCE_RULE_FIELDS` restamps the stage ruleset digest. This is the one
   that is widely known here, and the one `[[declaration-reader-drops-uninterpretable-rows]]` records
   `SIGNAL-DECLARATION-ROW-DROP.1` avoiding by putting its accounting *inside* an existing field.
2. **Changing the content of a field that is already registered** moves nothing in the ruleset and
   stales every claim anyway. `public_field_values()` serializes all 39 rule fields into
   `replay_bytes`; `replay_bytes` is the *output* of the registered derivation
   `evidence.current-replay`; and that derivation is the *sole input* of every per-claim derivation
   `evidence.claim.<field>.root`. `verify_premise` rejects a premise whose recorded `output_sha256` or
   `inputs_sha256` no longer matches the current replay, so one changed byte anywhere in any rule
   field invalidates the whole artifact's proof. The seal is untouched throughout.

Measured instance. `.1` added `declaration_row_accounting` to `extraction_manifest` — a registered
rule field — and the corpus went from loading to refusing with
`registered derivation 'evidence.claim.schema_version.root' output or input topology is stale`. Not
everywhere: **23 of 27** proof-carrying documents still loaded, because the accounting is empty for a
document whose tables never reach the body-row reader and an absent field deserializes to that same
empty value. The **4** that refused were AXI, APB, AHB and ADIv6 — every wire-bearing specification in
the corpus, so `eval-extraction` refused all eight golds and the WIRE-BASED-100 hard gate was
unmeasurable for three commits.

## Why the gate said green

`scripts/check_proof_seal_currency.sh` runs the canonical probe **once per distinct seal per stage**
and argues in its own header that this is *"not a sample"* because the census establishes
representativeness. The argument is false for this defect and the reason generalises: **the seal is
homogeneous precisely because it does not depend on artifact content, and the loader checks a
per-document replay topology that does.** All 27 evidence artifacts carried one seal, so one probe
ran, and it happened to be one of the 23 that worked. This is `CLAIM_VERIFICATION.md` §2's fourth
row — a per-item assertion checked against per-container data, which reproduces perfectly while
getting it wrong. `[[doctrine-driver-runs-no-cargo-gate]]` is the same session's other instance of a
report that is silent rather than negative.

**Repaired in `SIGNAL-DECLARATION-ROW-DROP.1c`, and the tiering is measured rather than asserted.**
`check_proof_seal_currency.sh --total` probes every in-scope artifact at every non-terminal stage: on
this corpus that is 108 probes against the sampled mode's 4, and it takes **19m02s**, which is why it
is registered as its own CI-tier doctrine (`PROOF-SEAL-TOTAL`, `scripts/check_proof_seal_total.sh`)
rather than made the gate. The gate still samples — a 19-minute pre-commit hook is a bypassed hook —
but its output now names the sample size and the class it cannot see. Self-tests 17 and 18 pin the
difference with a loader that accepts one document and refuses its same-seal neighbour: the sampled
mode must miss it, the total mode must name it. Self-test 19 pins the third outcome — a probe whose own
input is absent (a held-out normalized bundle) yields NO VERDICT, counted and named separately, never
folded into the accepted set.

## How the corpus is repaired

`scripts/rebuild_stage_cascade.sh --write` is the remedy for a **seal-only** staling and refuses any
content delta by design (ADR 0025 decision 1), so it cannot land a deliberate producer change. Run
the stages directly instead, for the affected documents only, stage-major with exactly one `validate`
per artifact strictly upstream-to-downstream — `validate` is not idempotent, and revalidating an
upstream invalidates every downstream artifact built before it. AXI, APB and AHB additionally have no
`normalized/` bundle in place: restore them from
`generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/`, rebuild, then
`diff -r` and remove, leaving the declared retained population at exactly 24
(`[[retained-bundle-population-is-frozen]]`). Snapshot first and compare per stage: a correct repair
shows the attributed field as the only difference, with SemanticIR, IntentIR and the `.isf` emission
byte-identical.
