---
id: persisted-llm-constraint-corpus-predates-catalog-grounding
title: The 149 persisted llm_sigcon_* records were minted 78 minutes before declared_signal_catalog existed — every census over them measures a producer that no longer exists
answers:
  - "why does a persisted llm_sigcon_* record carry a subject the catalog would refuse today (it was minted 2026-08-12 17:44 at HEAD 4b8895d6, where promote_constraints typed a subject as classify_entity(gather_entity_evidence(...), |_| EntityType::Signal) — an LLM judgment stubbed to answer Signal, consulting no catalog at all; declared_signal_catalog was written at 9c38b569 at 19:02, 78 minutes later)"
  - "how many of the 149 persisted llm_sigcon_* subjects would today's grounding refuse (36 — 111 exact-signal, 2 case-folded-signal, 0 field, 36 ungrounded; per document AXI 65/11, LTI 35/9, APB 19/1, ATB 8/1, AXI-Stream 8/0, OpenCAPI-3.0 7/7, OpenCAPI-3.1 7/7)"
  - "what is the real membership test the LLM constraint path grounds a subject against (resolve_unique_document_identifier over declared_signal_catalog(ir) first and message_field_records names second — commands/extract_constraints_llm.rs:110-128 — NOT the seven signal-bearing EvidenceIR surfaces, which no producer consults)"
  - "why can EvidenceIr::load_from_path not read the persisted evidence_ir corpus (every stored artifact is schema 2 and EVIDENCE_IR_SCHEMA_VERSION is 3 since 1aa7f95d on 2026-08-13 01:32, so the canonical loader refuses them as legacy/proofless; a read-only census uses load_for_inspection, which neutralizes only the three retired protocol carriers)"
  - "is APB's PSEL a declared signal as far as entity typing is concerned (no — APB's catalog holds PSELX and PSELXCHK because the document declares the parameterised template PSELx, and resolve_unique_document_identifier is exact-then-case-fold, so the bare PSEL cannot resolve; a refusal of PSEL is the resolver working, not a broken census)"
  - "why did a census over persisted artifacts disagree with what the producer does (because the artifacts were written by an earlier producer — check the artifact mtimes against git log -L on the grounding function before treating a persisted population as evidence about current behaviour)"
  - "how do I re-run the llm_sigcon_* subject grounding census (cargo test -p specforge-core --lib llm_constraint_subject_grounding_census -- --ignored --nocapture; the crate is specforge-core because crates/specforge/src/ir/** compiles into it by #[path])"
date: 2026-09-18
status: current
tags: [extraction-quality-gauge, entity-typing, llm-primary, census, staleness, adjudication, method]
evidence: docs/tasks/extraction-quality-gauge/llm-path-family.md (.3j.2); crates/specforge/src/commands/extract_constraints_llm.rs:110-128; crates/specforge/src/ir/entity_typing.rs:26-73; 4b8895d6:crates/specforge/src/commands/extract_constraints_llm.rs; 9c38b569 (declared_signal_catalog); 1aa7f95d (EVIDENCE_IR_SCHEMA_VERSION 3); generated/evidence_ir/ihi0089_d_2025_08_amba_lti_protocol_specification/evidence_ir.json (llm_sigcon_0034)
reverify: "cargo test -p specforge-core --lib llm_constraint_subject_grounding_census -- --ignored --nocapture — expect TOTAL 149 records: 111 exact-signal / 2 case-folded-signal / 0 field / 36 ungrounded, and UNGROUNDED 36 = 16 carries a declared name / 2 truncates one / 18 has no declared relative"
---

A persisted artifact is a record of **what some producer did**, not of what the producer does. This
corpus is the clearest instance in the repository, because the gap is 78 minutes wide and both sides are
in Git.

## The two producers

At `4b8895d6` (`2026-08-12 15:03`), the HEAD when all seven artifacts were written at `17:44`,
`promote_constraints` typed a constraint subject like this:

```rust
let type_subject = |s: &str| {
    classify_entity(&gather_entity_evidence(s, &ir, std::slice::from_ref(sentence)), |_| EntityType::Signal)
};
```

The injected `propose` is stubbed to answer `Signal`, so **every token the document did not positively
contradict became a signal**, and no catalog was consulted anywhere on that path — `propose_constraints_llm`
took no carrier list either. Today the same lines are an identity test against the document's own
declarations, and `declared_signal_catalog` did not exist until `9c38b569` (`19:02`).

## What that costs a census

Re-running the **current** closure over the persisted population gives **36 ungrounded of 149**. So a
gate measured on this corpus is being scored against subjects a quarter of which the current producer
would never have minted. `EXTRACTION-QUALITY-GAUGE.3j` measured 7 refusals of 149 on it;
`.3j.1.b` already requires a refreshed population, and this is the size of the reason.

**The distinction that decides whether a finding survives the staleness:** a class that is a property of
the **resolver** reproduces on any catalog that declares the base name, and needs no refresh — the 16
subjects that carry a declared name under a bit slice or qualifier (`RRESP[3]`, `WSTRB bits`,
`Subordinate LAPM`), and the 2 that truncate one (`PSEL`, `ATB`). A class that is a property of the
**catalog's completeness** does not — `LASECSID` absent from LTI's persisted catalog says nothing about a
current build.

## The trap that cost a leaf

`.3j.2` was opened carrying a recorded caveat: a proxy census "flagged 36 of 149" and "still left 20"
after reducing sliced spellings, and was discarded because the survivors "include APB's `PSEL`, which is
unquestionably declared". The real test returns **36**, and 36 − 16 sliced/qualified = **20**. The proxy's
numbers were right. `PSEL` is refused because APB declares the **parameterised template** `PSELx`, so the
catalog holds `PSELX`/`PSELXCHK` and never the bare family name — a correct refusal, mistaken for a broken
census because "unquestionably declared" was a reading of the protocol rather than a check of the
document's catalog.

**The cheap rule:** before treating a persisted population as evidence about current behaviour, compare
the artifacts' mtimes against `git log -L` on the function you are measuring. It costs one command and it
is the difference between a finding and an archaeology report.

Links: [[a-cheap-structural-rule-overfires-until-you-read-its-selection]],
[[declaration-reader-drops-uninterpretable-rows]]
