---
id: legacy-source-classifications-are-neutralized-on-load
title: A legacy SourceIR loads with every typed classification neutralized to Unknown, so any pass keyed on table_kind, diagram_kind or section_kind returns an empty result that looks exactly like "this document states none"
answers:
  - "why does a signal-description table pass find no tables in a persisted document"
  - "why does table_kind read Unknown when the persisted JSON says signal_description"
  - "what does load_for_inspection do to a legacy SourceIR"
  - "which documents can a classification-keyed extractor actually see"
  - "how many corpus documents have a current-schema SourceIR"
  - "why did the row-constraint replay judge only 26 documents"
  - "is an empty result from a table-kind-keyed pass an answer or a blind spot"
  - "does the persisted table_kind field carry authority"
  - "why did a Python census over persisted table_kind over-count the row extractor's population"
date: 2026-09-13
status: current
tags: [source-ir, schema, classification, census, claim-verification, extraction-quality-gauge]
evidence: crates/specforge/src/ir/source.rs (neutralize_legacy_source_classifications, load_for_inspection, carries_canonical_source_classifications); crates/specforge/src/ir/evidence.rs (should_treat_table_as_top_level_signal_description, mod extraction_quality_gauge_3k_2g); crates/specforge/src/commands/replay_constraints.rs (sibling_source_ir); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.2e, .3k.2g, .3k.2i)
reverify: "cargo run -- replay-constraints --evidence-root generated/evidence_ir — the report prints row_stratum_judged_documents and row_stratum_unjudged_documents; the second number is this card's population"
---

`SourceIr::load_for_inspection` accepts an artifact written by an older schema, and
`neutralize_legacy_source_classifications` then sets **every typed source classification on it to
`Unknown`** — `table_kind` on every structured table, `diagram_kind` on every visual asset,
`section_kind` on every section. That is deliberate and correct: only the current schema plus a
verified proof ledger carries classification authority, and a note saying so is pushed onto the
artifact's normalization plan.

The consequence is the part that bites, and it is not an error path:

> A pass keyed on a classification sees **nothing** over such an artifact, and returns an empty
> result that is **indistinguishable from "this document states none"**.

`extract_signal_description_row_constraints` selects tables by `TableKind::SignalDescription`. Over a
legacy artifact it selects none and mints no record — not because the document has no signal tables,
but because the loader withdrew the claim that it does. Measured on AMBA LTI: its persisted
`source_ir.json` marks **25** tables `signal_description`, and after a legacy load **0 of its 88**
tables pass `should_treat_table_as_top_level_signal_description`.

**Ask which artifacts can be seen before quoting any classification-keyed count.**

```bash
# how many documents carry a current-schema SourceIR at all
for f in generated/source_ir/*/source_ir.json; do
  python3 -c "import json,sys; print(json.load(open(sys.argv[1]))['schema_version'])" "$f"
done | sort | uniq -c
```

**Two failures this caused, both worth keeping.**

*A census that read the field instead of the producer.* `EXTRACTION-QUALITY-GAUGE.3k.2d` sized two
follow-up leaves with a Python census that selected tables on the persisted `table_kind` string. The
producer selects on the loaded one. The census reported a population of 17 clauses reaching an arm;
the real producer's population over every document it can see is **0**. Both numbers were published
before `.3k.2g` re-derived them. This is `CLAIM_VERIFICATION.md` §3 Leg 2 exactly — derive a
membership test from the producer, never from a description of the producer, or from a field the
producer no longer trusts.

*An instrument that would have reported the silence as a result.* `replay-constraints` gained the row
producer in `.3k.2g`. Gating it only on "the `SourceIr` file exists" made it claim **77** documents
judged; gating it on `carries_canonical_source_classifications` reports **26 judged / 51 not**, and
says why in the same line. The 51 are not a defect count and not a clean bill — they are unmeasurable
until re-ingest reaches them.

This is [[persisted-census-measures-published-not-current]] one layer deeper: that card says a
persisted RECORD may not be what today's code would mint. This one says a persisted
**classification** may not even be visible to today's code, so a census keyed on one measures a
population the producer cannot reach.

Related: [[persisted-census-measures-published-not-current]],
[[constraint-record-producer-strata]].
