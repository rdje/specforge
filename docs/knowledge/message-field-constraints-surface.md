---
id: message-field-constraints-surface
title: message_field_constraints — field-subject obligations are routed, not dropped (CHI TagOp/PBHA live)
answers:
  - "where do obligations on message fields (TagOp must be 0) live in EvidenceIR"
  - "what is MessageFieldConstraintRecord / ground_constraint_typed / GroundedConstraint"
  - "why a parallel field-constraint surface instead of a subject-kind discriminator"
  - "do field constraints pass the same grounding gates as signal constraints"
  - "how was the CHI field-constraint routing measured without re-ingesting the PDF"
  - "what is the message_field_catalog_dump measurement harness"
  - "why does the MPAM 'must be included' sentence extract nothing"
date: 2026-06-10
tags: [extraction-quality, fields, packet-protocols, llm-primary, constraints, evidence-ir, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (MessageFieldConstraintRecord, EvidenceIr.message_field_constraints, message_field_catalog_dump_local_measurement); crates/specforge/src/ir/constraint_extract_llm.rs (GroundedConstraint, ground_constraint_typed, dedup_merge_by, dedup_field_constraints); crates/specforge/src/commands/extract_constraints_llm.rs; docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.FIELD.4)
reverify: cargo test -p specforge --lib constraint_extract_llm 2>&1 | tail -2   # → 29 passed; full re-measure = the .8 redirected-copy protocol + the catalog-dump harness below
---

`EvidenceIr.message_field_constraints` (`EXTRACTION-QUALITY-GAUGE.FIELD.4`, `2026-06-10`)
captures typed obligations whose subject is a catalog-declared MESSAGE FIELD ("the TagOp field
is inapplicable and must be 0") — real protocol intent about flit/message CONTENT that the
`.FIELD.3` signal-subject gate would otherwise silently drop. **Decision: a parallel surface,
not a subject-kind discriminator on `SignalConstraintRecord`** — downstream `signal_constraints`
consumers (eval keys, nli-verify, semantic carry-through, ISF adapter) keep seeing wires only,
by construction. `ground_constraint_typed` (`GroundedConstraint::Signal|Field`) dispatches on
the entity type AFTER the shared gates — `.3a` condition-only-subject, `.3b` permissive-frame,
`.8` value recovery, `.2` condition grounding — so a field obligation gets NO discipline
discount (fixture-locked); `ground_constraint` remains the signal-only view. `dedup_merge_by`
generalizes the `.4` provenance-merging dedup over both surfaces (field key =
subject+kind+value+negation+condition; `containers` are catalog provenance, not identity).
`extract-constraints-llm` REPLACES both surfaces (`llm_fieldcon_NNNN` ids).

**Measured live** (persisted CHI evidence, `.8` redirected-copy protocol, qwen2.5:14b-instruct
temp 0; catalog injected from the REAL `.FIELD.2` extractor over persisted SourceIR via the new
`#[ignore]`d `message_field_catalog_dump_local_measurement` harness — 106 fields, reproducing
`.FIELD.2`): baseline (no catalog) mis-types `TagOp`/`PBHA` `must_be_value 0` as SIGNAL
constraints; with catalog, the signal surface is EXACTLY the 4 real flit-valid wires
(`REQFLITV`/`RSPFLITV`/`SNPFLITV`/`DATFLITV`) and the 2 field obligations land field-scoped with
containers + merged provenance (twice-stated TagOp = ONE record, both statement ids). Wire
controls: APB 20 / AHB 12 / AXI 54→50 — exact `.3b`/`.4` volumes, zero field constraints,
eval P=R=F1=1.000 ×3.

GOTCHAS: (1) packet-doc evidence artifacts predate `message_field_records` — inject the catalog
into a REDIRECTED copy (`SPECFORGE_MEASURE_SOURCE_IR=<source_ir.json> cargo test -p specforge
--lib message_field_catalog_dump -- --ignored --nocapture`); never run `extract-constraints-llm`
on a corpus original ([[message-field-records-surface]]). (2) Field PRESENCE requirements
("the MPAM field must be included on the REQ and SNP channels") have no constraint-kind slot —
probed live, the model outputs `[]` (the same root-cause shape as
[[llm-primary-must-be-value-recall]]); a future kind needs the `.8` probe-first method, never a
guess. (3) The extraction prompt deliberately still says "wire/pin/field" — the model PROPOSES
field subjects so Rust can TYPE and ROUTE them; do not "fix" the prompt to exclude fields.
