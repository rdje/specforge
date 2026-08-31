---
id: evidence-proof-binds-artifact-location
title: An EvidenceIR's proof binds the artifact's own storage location, so relocating a byte-identical copy fails canonical verification — which is why `eval-extraction` refuses every document in the corpus
answers:
  - "why does eval-extraction fail with EvidenceIR proof verification failed registered derivation evidence.claim.schema_version.root output or input topology is stale (because extract_on_copy relocates the artifact into a temp root so the corpus is never mutated, and the EvidenceIR proof binds the artifact's own artifact_layout into a derivation's topology — rewriting only artifact_root/evidence_ir_path, every other byte identical, fails verification)"
  - "can WIRE-BASED-100 scores be re-derived right now (NO — eval-extraction refuses every document in the corpus as of 2026-08-31: the 54 legacy chains as EvidenceIR schema version 2 is legacy/proofless and inspection-only, and the 24 current ones as the artifact_layout topology failure. Owned by WIRE-BASED-100.8; the last re-derivation of record is SWD-SERIAL-EXTRACTION on 2026-08-09)"
  - "is the eval-extraction proof failure a regression from a recent slice (NO — it reproduces on target/release/specforge built 2026-08-28, before the KG-ISF-COMPLETENESS.5.iv.a change that found it)"
  - "how do I tell an artifact-relocation proof failure from proof-seal staleness (they are different: proof-seal staleness is a ruleset-hash mismatch that check_proof_seal_currency.sh reports and source_proof_migrate re-seals. This one passes the seal check — 24/24 accepted — and passes chain currency and specforge semantic --dry-run; it fails only when the artifact is read from a different location than the one recorded in its artifact_layout)"
  - "how do I reproduce the artifact-relocation proof failure read-only (copy an evidence_ir.json, rewrite only its artifact_layout artifact_root and evidence_ir_path to the new directory, and run specforge entity-type on it: it fails. Run the same command on a byte-identical copy that keeps the original layout: it succeeds)"
  - "which commands still work on a persisted EvidenceIR that eval-extraction refuses (specforge entity-type, specforge semantic --dry-run, and the whole check_chain_currency.sh replay — they read the artifact in place, so the recorded layout still matches)"
  - "does this break repository portability (not for a repository move — artifact_layout stores repository-root-relative paths and tools derive absolute paths at runtime. It breaks relocation WITHIN the repository, which is what a read-only evaluation on a temp copy needs)"
date: 2026-08-31
status: current
tags: [wire-based-100, eval, proof-carrying, evidence-ir, gate-integrity, claim-verification, defect]
evidence: crates/specforge/src/commands/eval_extraction.rs (extract_on_copy :157-163 relocates into project_data::tempdir before running the real command); docs/tasks/WIRE-BASED-100.md (.8); docs/tasks/KG-ISF-COMPLETENESS.md (.5.iv.a acceptance checklist, HONEST GAP); scripts/check_proof_seal_currency.sh; scripts/check_chain_currency.sh
reverify: "S=.project-data/tmp/evprobe; mkdir -p $S; python3 -c \"import json;d=json.load(open('generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json'));d['artifact_layout']={'artifact_root':'$S','evidence_ir_path':'$S/evidence_ir.json'};json.dump(d,open('$S/evidence_ir.json','w'))\"; ./target/debug/specforge entity-type $S/evidence_ir.json; cp generated/evidence_ir/um10204_rev7_0_2021_i2c_bus_specification/evidence_ir.json $S/verbatim.json; ./target/debug/specforge entity-type $S/verbatim.json"
---

# A relocated EvidenceIR fails its own proof

`eval-extraction` is the command every WIRE-BASED-100 number is scored with. It must not mutate the corpus,
so `extract_on_copy` loads the document's persisted EvidenceIR, redirects `artifact_layout` at a temporary
root, writes the copy there, and runs the real extraction command against it. That relocation is what fails:

```text
error: invalid stage artifact: EvidenceIR proof verification failed: registered derivation
'evidence.claim.schema_version.root' output or input topology is stale
```

The cause is isolated read-only, and it is the artifact's own recorded location rather than its content.
Rewriting **only** `artifact_layout.artifact_root` and `artifact_layout.evidence_ir_path` — every other byte
identical — makes canonical verification fail, while a byte-identical copy that keeps its original layout
verifies and runs normally. So the proof's derivation topology includes where the artifact is stored.

## What this is not

It is not the proof-seal staleness that `SOURCE-IR-REPRODUCIBILITY.14`/`.15` resolved. Those were ruleset
hash mismatches, and `check_proof_seal_currency.sh` reports the corpus clean today: 24/24 persisted and
sealed, 1 accepted, 0 refused at every stage. `check_chain_currency.sh` likewise replays all 24 rebuildable
documents to 24 current at evidence, semantic, intent, and the ISF adapter, and `specforge semantic
--dry-run` reads the same artifact `eval-extraction` refuses. Every one of those reads the artifact **in
place**.

## Why it matters

`WIRE-BASED-100`'s governing principle is that no score may be faked, and `KG-ISF-COMPLETENESS` requires a
before/after eval on any extraction change touching a scored document. With the oracle down, neither the
published `1.000` wire scores nor any new measurement can be re-derived on demand — which is exactly the
kind of claim `CLAIM_VERIFICATION.md` refuses. The legacy stratum compounds it: 54 of 78 persisted chains
are schema-1 proofless artifacts the current binary refuses outright, including the APB, AHB, and AXI wire
golds, so even a fixed `eval-extraction` can only score the SWD/ADI and I2C golds until those chains are
re-ingested.

Found `2026-08-31` while gating `KG-ISF-COMPLETENESS.5.iv.a`, and reproduced on the pre-change binary, so it
is not a regression from that slice. Owned by `WIRE-BASED-100.8`.

Links: [[generic-enum-conflation]], [[chain-currency-doctrine]],
[[persisted-chain-currency-is-measured-not-assumed]], [[eval-scores-persisted-evidence]].
