---
id: persisted-census-measures-published-not-current
title: A census over the persisted corpus measures what SpecForge PUBLISHED, not what today's extractor does — 54 of 78 documents cannot have their evidence stage re-run and are frozen at the generation that wrote them
answers:
  - "does a count over generated/evidence_ir measure current extractor behaviour"
  - "why does a record in the persisted corpus not reproduce when I run the extractor on its source_text"
  - "how many persisted documents can have their evidence stage rebuilt"
  - "which documents have a retained normalized bundle"
  - "how do I tell whether a persisted constraint record is still reproducible"
  - "why did EXTRACTION-QUALITY-GAUGE.3k.1 have zero currently-reproducible instances"
  - "is the persisted corpus one code generation"
  - "how do I size the population of an extractor change"
  - "what is the difference between the published population and the actionable population"
date: 2026-09-12
status: current
tags: [evidence-ir, census, claim-verification, corpus, chain-currency, extraction-quality-gauge]
evidence: generated/source_ir/*/normalized (24 of 78); generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out; crates/specforge/src/ir/evidence.rs (is_post_passive_binding_only_subject, CORPUS-COVERAGE.2.50a); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k, .3k.1); docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md
reverify: "cargo run -- replay-constraints --evidence-root generated/evidence_ir — expect 144 of 179 persisted deterministic records reproduced, 67 granted declarations, and one named skip"
---

The persisted corpus is **not one code generation**. Only 24 of the 78 documents keep a
`generated/source_ir/<doc>/normalized/` bundle (three more — APB, AHB, AXI — are held out under
`generated/preserved/WIRE-BASED-100.10/` and restored for a rebuild), and the evidence stage reads
that bundle. Every other document's `evidence_ir.json` is **frozen at whatever generation last wrote
it** and can only move if its source PDF is re-supplied and the document re-ingested.

So two different populations exist and they are routinely confused:

| question | population |
| --- | --- |
| what has SpecForge **published**? | the persisted records — the right measure of the shipped corpus's precision |
| how many records will **this code change** move? | only records the current extractor still reproduces |

A leaf sized from the first number and shipped against the second is measuring the wrong thing.

**The instance that cost a slice.** `EXTRACTION-QUALITY-GAUGE.3k.1` opened on four AMBA DTI records
publishing `OAS must_be_stable, negated` — *"OAS must not be stable"* — from *"The range given by
this field must not be greater than the size indicated by the OAS field of the DTI\_TBU\_CONDIS\_ACK
message"*. Running `extract_signal_constraints` on that exact sentence yields **nothing today**:
every candidate subject in it is named only AFTER the obligation's lead, so
`CORPUS-COVERAGE.2.50a`'s pre-lead subject authority (`is_post_passive_binding_only_subject`) reaches
it first. The four records predate that gate and the document has no bundle to rebuild from, so they
sit there. The published population was 4; the currently-reproducible population was 0.

The class was still real — the same grammar with the constrained signal named *before* the lead
(*"ZETARANGE must not be greater than the size indicated by the ZETAOAS field"*) still mints
`MustBeStable` + `negated` — so the leaf shipped, with the correct population statement. Had the
class not been reachable, the right outcome would have been to close the leaf as already-covered.

**The instrument** is `specforge replay-constraints` (`EXTRACTION-QUALITY-GAUGE.3k.6`). It re-runs the
REAL deterministic producer over an artifact's own `extracted_statements` — the constraint surface is
a function of the statements, not of the PDF, which is why it works for the frozen 54 — and reports
per published record whether today's code still mints it, naming the gate that stands in the way when
it does not. A Python mirror of the rule could not answer this: mirror and original agreeing carries
no information (`CLAIM_VERIFICATION.md` §2).

```bash
cargo run -- replay-constraints --evidence-root generated/evidence_ir
# persisted_deterministic_records: 179   reproduced: 144   granted_declarations: 67   skipped: 1
```

Read the verdict **asymmetrically**. "Not reproduced" is sound: the replay grants a declaration to any
published subject the artifact no longer declares, and a wider catalog can only admit more subjects,
never withdraw one. The opposite direction is not sound — the build applies convergence stages the
replay does not — so `unpersisted_replay_records` is evidence to read, never a number to quote.

Calibration: artifacts the current binary itself wrote reproduce completely (APB 15/15, AHB 13/13,
both rebuilt by `EXTRACTION-QUALITY-GAUGE.3i`), and AMBA CXS reproduces 0/2 because `.3i` retyped
exactly its `must not be asserted` shape — two independent confirmations that the instrument is
measuring generation drift rather than noise.

Do this before sizing any extractor change; the artifact census tells you where to look, not what will
move.

Related: [[constraint-record-producer-strata]] (the other half — count the population of the
FUNCTION being changed, not of the table the records land in),
[[evidence-rule-field-content-stales-every-proof]] (which documents can be rebuilt, and how).
