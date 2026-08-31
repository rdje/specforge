---
id: evidence-proof-binds-artifact-location
title: An EvidenceIR's proof binds the artifact's own storage location, so a byte-identical copy elsewhere fails canonical verification — relocation needs the proof-carrying seam
answers:
  - "why does moving an evidence_ir.json make it fail with EvidenceIR proof verification failed registered derivation evidence.claim.schema_version.root output or input topology is stale (because the proof's registered replay is taken over public_field_values, that map includes the artifact's own artifact_layout, and every evidence.claim.<surface>.<key> derivation takes the replay as its sole input — so each claim premise's inputs_sha256 binds the storage path and any relocation makes the recomputed topology differ)"
  - "how do I relocate a verified EvidenceIR without breaking its proof (EvidenceIr::load_relocated_to_artifact_base_root — it verifies the artifact where it is, moves it to <base>/<document_key>/evidence_ir.json, and re-derives the proof for the new location from the same verified SourceIR prefix and the same sealed proof context; an unsealed artifact_layout rewrite is still refused)"
  - "can WIRE-BASED-100 scores be re-derived right now (YES for the rebuildable stratum since 2026-09-01 / WIRE-BASED-100.8a. Before that eval-extraction refused every document. The 54 legacy schema-1 chains — including the APB, AHB, AXI, NVMe and RISC-V golds — are still refused as proofless and inspection-only and need re-ingest)"
  - "was the eval-extraction proof failure a regression from a recent slice (NO — it reproduced on target/release/specforge built 2026-08-28, before the KG-ISF-COMPLETENESS.5.iv.a change that found it)"
  - "how do I tell an artifact-relocation proof failure from proof-seal staleness (they are different: proof-seal staleness is a ruleset-hash mismatch that check_proof_seal_currency.sh reports and source_proof_migrate re-seals. Relocation passes the seal check, passes chain currency, and passes specforge semantic --dry-run; it fails only when the artifact is read from a different location than the one recorded in its artifact_layout)"
  - "how do I reproduce the artifact-relocation proof failure read-only (copy an evidence_ir.json, rewrite only its artifact_layout artifact_root and evidence_ir_path to the new directory, and run specforge entity-type on it: it fails. Run the same command on a byte-identical copy that keeps the original layout: it succeeds. Keeping the <base>/<document_key> convention does not help — relocation as such is what fails)"
  - "does this break repository portability (not for a repository move — artifact_layout stores repository-root-relative paths and tools derive absolute paths at runtime. It breaks relocation WITHIN the repository, which is what a read-only evaluation on a temp copy needs, which is why the relocation seam exists)"
  - "why was the deeper fix of unbinding artifact_layout from the replay topology not taken (it would change the frozen 38-family/170-field producer graph AND invalidate all 24 sealed chains at once, since every sealed proof was taken over a replay that already contains the location. Re-proving on relocation is bounded, keeps every existing seal valid, and cannot persist tampered content because write_to_disk re-verifies against an independent rebuild)"
date: 2026-09-01
status: current
tags: [wire-based-100, eval, proof-carrying, evidence-ir, gate-integrity, claim-verification]
evidence: crates/specforge/src/ir/evidence.rs (proof_kernel registers evidence.current-replay :1416 and each claim derivation over it :1424; public_field_values inserts artifact_layout :1244; load_relocated_to_artifact_base_root is the supported seam); crates/specforge/src/ir/derivation.rs:2149 (the RegisteredDerivation premise arm that raises the diagnostic); crates/specforge/src/commands/eval_extraction.rs:151 (extract_on_copy); docs/tasks/WIRE-BASED-100.md (.8, .8a)
reverify: "cargo test -p specforge-core --lib relocation_through_the_seam_preserves_verification && ./target/release/specforge eval-extraction crates/specforge/test_data/llm_eval/seed_i2c_signals.json --provider skip"
---

# A relocated EvidenceIR fails its own proof, unless it is re-proved

`eval-extraction` is the command every WIRE-BASED-100 number is scored with. It must not mutate the corpus, so
`extract_on_copy` runs the real extraction commands against a copy of the document's EvidenceIR under a
temporary root. That relocation is what used to fail:

```text
error: invalid stage artifact: EvidenceIR proof verification failed: registered derivation
'evidence.claim.schema_version.root' output or input topology is stale
```

The cause is the artifact's own recorded location rather than its content. `proof_kernel` registers a single
`evidence.current-replay` derivation over `serde_json::to_vec(public_field_values())`, and every
`evidence.claim.<surface>.<key>` derivation takes that replay as its sole input. `public_field_values` inserts
`artifact_layout`, so each claim premise's `inputs_sha256` binds where the artifact is stored. Verification
recomputes the replay from the artifact it is reading; at a new path the topology no longer matches, and
`validate_premise`'s `RegisteredDerivation` arm raises exactly that diagnostic.

Keeping the `<base>/<document_key>` layout convention does not rescue it. Two copies were probed read-only —
one convention-preserving, one flat — each with only `artifact_layout` rewritten and every other byte
identical; `specforge entity-type` refuses both, and succeeds on the artifact in place. Relocation as such is
what the seal forbids.

## The supported operation

`EvidenceIr::load_relocated_to_artifact_base_root(evidence_ir_path, artifact_base_root)` verifies the artifact
where it is, moves it to `<artifact_base_root>/<document_key>/evidence_ir.json`, and re-derives the proof for
that location from the same verified SourceIR prefix and the same sealed proof context, mutation chain
included. It cannot launder authority: the artifact must already verify, and `write_to_disk` re-verifies the
result against an independent rebuild from the SourceIR, so content that no longer replays cannot be
persisted. An unsealed `artifact_layout` rewrite stays refused, and both halves are pinned by
`relocation_through_the_seam_preserves_verification`.

The deeper alternative — taking the replay over everything *except* the artifact's location — was not chosen.
It changes the frozen 38-family/170-field producer graph and invalidates all 24 sealed chains simultaneously,
because every existing seal was taken over a replay that already contains the location.

## What this is not

It is not the proof-seal staleness that `SOURCE-IR-REPRODUCIBILITY.14`/`.15` resolved. Those were ruleset hash
mismatches. `check_proof_seal_currency.sh` reports the corpus clean, `check_chain_currency.sh` replays all 24
rebuildable documents to current at every stage, and `specforge semantic --dry-run` reads the same artifact
`eval-extraction` used to refuse — every one of those reads the artifact **in place**.

## Why it mattered

`WIRE-BASED-100`'s governing principle is that no score may be faked, and `KG-ISF-COMPLETENESS` requires a
before/after eval on any extraction change touching a scored document. While the oracle was down neither the
published wire scores nor any new measurement could be re-derived on demand — exactly the claim
`CLAIM_VERIFICATION.md` refuses. The legacy stratum still bounds what a working oracle can reach: 54 of 78
persisted chains are schema-1 proofless artifacts the current binary refuses outright, including the APB, AHB
and AXI wire golds, so only the SWD/ADI and I2C golds are scoreable until those chains are re-ingested.

Found `2026-08-31` while gating `KG-ISF-COMPLETENESS.5.iv.a` and reproduced on the pre-change binary, so it was
never a regression from that slice. Fixed by `WIRE-BASED-100.8a` on `2026-09-01`. The first thing the restored
oracle found is `[[swd-serial-frame-score-retired-by-genericity]]`.

Links: [[generic-enum-conflation]], [[chain-currency-doctrine]],
[[persisted-chain-currency-is-measured-not-assumed]], [[eval-scores-persisted-evidence]].
