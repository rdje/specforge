---
id: measured-stratum-promotion-population
title: The measured stratum's LLM-promotable population is 5 documents and 62 provider calls, not 27 — and it carries no quality gauge to drop
answers:
  - "how many of the measured-stratum documents can the LLM-primary constraint promotion actually be run on (5 of 27 — ihi0022_l_2025_08 AXI, ihi0024_e APB, ihi0033_c AHB, um10204 I2C, ihi0074_a ADIv6; the other 22 carry zero signal constraints, so the promotion's recall universe is empty on them)"
  - "what does one run of the LLM-primary promotion over the measured stratum cost (62 provider calls — one per distinct persisted source_text: AXI 37, AHB 11, APB 10, I2C 3, ADIv6 1)"
  - "what is the recall universe of promote_constraints (NOT the document — the distinct source_text of the constraints already persisted, one provider call each; a document with no Pattern constraints gets no LLM proposals at all)"
  - "does promoting a measured-stratum document destroy a persisted extraction quality gauge (no — zero of the 27 measured documents carry one; the only seven artifacts in generated/ that carry a gauge are exactly the seven historical documents that are already promoted, because nli-verify was only ever run where the promotion had been)"
  - "is running the LLM promotion on a document with no constraints a no-op (no — with zero sentences the surface replace changes nothing, but promote_constraints still records a constraints.llm_primary surface manifest and authorize_mutation still appends a ConstraintPromotion record to the proof ledger, so the artifact claims a surface the model never saw)"
  - "which of the seven historical LLM-measured documents have a measured-stratum counterpart (only two — AXI ihi0022_h_c to ihi0022_l_2025_08 and APB ihi0024_d to ihi0024_e; ATB, AXI-Stream, LTI and both OpenCAPI transaction-layer documents have none, so .3j's census cannot be re-derived document-for-document)"
  - "how do I re-derive the measured-stratum promotion population (cargo test -p specforge --lib measured_stratum_promotion_population -- --ignored --nocapture; the crate is specforge, not specforge-core, because commands/** does not #[path] into core)"
  - "is there a second independent derivation of ADR 0048's 27/51 corpus split (yes — EvidenceIr::load_from_path accepting 27 of the 78 persisted evidence_ir.json artifacts, which reaches the same split through a different file, field and production function than ADR 0048's proof-ledger grep over source_ir.json)"
date: 2026-09-18
status: current
tags: [extraction-quality-gauge, llm-primary, corpus, measurement, adjudication, adr-0048, method]
evidence: docs/tasks/extraction-quality-gauge/llm-path-family.md (.3j.4); docs/decisions/0048-the-persisted-corpus-has-a-measured-and-a-historical-stratum.md; crates/specforge/src/commands/extract_constraints_llm.rs (promote_constraints, measured_stratum_promotion_population_local_measurement, promote_constraints_records_manifest_and_drops_stale_gauge); crates/specforge/src/ir/evidence.rs (load_from_path, extraction_quality_gauge, EvidenceMutationKind::ConstraintPromotion)
reverify: "cargo test -p specforge --lib measured_stratum_promotion_population -- --ignored --nocapture — expect 78 read / MEASURED 27 / HISTORICAL 51 / MEASURED promotable 5 / provider calls 62 / no recall universe 22 / MEASURED gauge 0 / MEASURED promoted 0 / HISTORICAL gauge 7 / HISTORICAL promoted 7"
---

`ADR 0048` split `generated/` into a measured stratum of 27 and a historical stratum of 51, and ruled
that only the first may ground a current claim. It then framed the retarget of the LLM-primary
measurement as *pick from the 27*. Two properties of the artifacts — neither read before the plan was
written — make that framing wrong in both directions.

## The population is 5, because the recall universe is sentences and not documents

`promote_constraints` does not walk the document. Its first ten lines collect the **distinct
`source_text` of the constraints already persisted**, and issue one provider call per distinct sentence.
It is a refinement of what the Pattern surface found, never a discovery pass. So a document with no
Pattern constraints has an empty recall universe and the model is asked nothing at all.

Measured over all 27, only five carry any:

| document | pattern constraints | distinct sentences | catalog |
| --- | --- | --- | --- |
| `ihi0022_l_2025_08` AXI | 56 | 37 | 297 |
| `ihi0024_e` APB | 18 | 10 | 32 |
| `ihi0033_c` AHB | 11 | 11 | 41 |
| `um10204` I2C | 3 | 3 | 6 |
| `ihi0074_a` ADIv6 | 1 | 1 | 12 |

For scale, `.3j`'s historical seven were 84 distinct sentences and yielded 149 records.

The remaining 22 are not merely uninteresting — promoting them is **not** a no-op. With zero sentences
the surface replace changes nothing, but the manifest entry `constraints.llm_primary` is still recorded
(the in-tree control `promote_constraints_records_manifest_and_drops_stale_gauge` asserts exactly that on
a zero-constraint artifact) and `authorize_mutation` still appends a `ConstraintPromotion` record to a
proof-carrying artifact's ledger. The artifact would then assert that the model-primary extractor owns a
surface the model never saw.

## The hazard that was recorded is not on this stratum

`.3j.4` opened with the warning that a promotion *drops the persisted quality gauge, so it is not a
read-only act on a stratum the gates hold current*. Measured: **no measured-stratum document carries a
gauge**. The only seven artifacts in `generated/` that do are exactly the seven historical documents that
are already promoted — the same seven, because `nli-verify` was only ever run where the promotion had
been. The gauge cost of promoting the five is zero. What remains non-read-only is the surface replace and
the ledger append: real, smaller, and a different thing.

The general lesson is the same one `[[persisted-llm-constraint-corpus-predates-catalog-grounding]]`
taught about staleness, in a second dimension. There, a population was assumed to describe the current
producer and did not. Here, a population's *cost and hazard* were assumed from the plan rather than read
off the artifacts. Both are cheap to check and neither was checked.

## The split re-derives through a second route

`EvidenceIr::load_from_path` — the canonical loader `promote_constraints` calls on its first line —
accepts 27 of the 78 persisted `evidence_ir.json` artifacts and refuses 51. That is the same split
`ADR 0048`'s own `reverify` obtains by grepping `source_ir.json` for a proof ledger, reached through a
different file, a different field and a different production function. The strata are not an assertion
about the corpus; two independent production routes agree on their membership.

## Only two of the seven have a counterpart

AXI (`ihi0022_h_c` → `ihi0022_l_2025_08`) and APB (`ihi0024_d` → `ihi0024_e`) do. ATB, AXI-Stream, LTI
and both OpenCAPI transaction-layer documents do not. So `.3j`'s census is not re-derived
document-for-document; it is re-derived over a different five, of which two are counterparts and three
(AHB, I2C, ADIv6) are families the LLM path has never seen. The leaf that needs a *refreshed LTI*
specifically — `.3j.2.c`, the row-keyed compatibility-matrix obligations — cannot have one without a
deliberate re-ingest under `ADR 0048` §5.
