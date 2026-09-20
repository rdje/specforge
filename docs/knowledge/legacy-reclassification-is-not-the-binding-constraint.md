---
id: legacy-reclassification-is-not-the-binding-constraint
title: The 363 signal tables a legacy artifact's labels would recover cannot reach the canonical row reader, because the reclaimed normalized bundle stops the evidence build before any classification is consulted
answers:
  - "why can a legacy SourceIR's re-derived labels not reach the table-row reader"
  - "what stops a legacy artifact entering the EvidenceIR build"
  - "is neutralized classification the reason legacy documents extract nothing"
  - "how many signal tables would the current classifier recover on legacy artifacts"
  - "can a legacy source classification be re-derived without a PDF"
  - "may a diagnostic re-derive a neutralized source classification"
  - "why does the information-flow graph refuse a re-derived table label"
  - "does re-deriving a table label grant canonical authority"
  - "how many consumers call SourceIr::load_for_inspection"
  - "what does row_stratum_unjudged_documents falling to zero actually mean"
date: 2026-09-20
status: current
tags: [legacy-source-reclassification, source-ir, evidence-ir, authority, measurement]
evidence: crates/specforge/src/ir/source.rs (neutralize_legacy_source_classifications, classified_table_kind); crates/specforge/src/commands/replay_constraints.rs (sibling_source_ir); docs/tasks/LEGACY-SOURCE-RECLASSIFICATION.md (.0, .1)
reverify: "python3 - <<EOF over generated/source_ir/*/source_ir.json: 51 legacy artifacts, normalization_plan.status ready 51/51, promoted_markdown_path present 0/51 — the bundle, not the label, is what bars the canonical reader. The 363/30 and 344/24 figures re-derive by running classified_table_kind over the same artifacts with table_kind reset to Unknown."
---

`neutralize_legacy_source_classifications` withdraws `table_kind`, `diagram_kind` and `section_kind`
from every schema-1 artifact, because those labels came from a retired corpus-calibrated classifier
that ADR 0006 forbids. The inference that the labels are therefore **unrecoverable** was never
tested, and it is false: `classified_table_kind` is a pure function of caption, header rows and body
rows, all of which survive the legacy load. Measured over all 51 legacy artifacts with their labels
neutralized exactly as the loader leaves them: **1,744 non-`Unknown` tables, 363
`SignalDescription` across 30 documents**, of which **344 across 24** pass the row reader's own
top-level gate.

## The recovery does not reach the reader, and that is the fact worth carrying

The table-row constraint reader runs inside the **EvidenceIR build**, and a legacy artifact cannot
enter it for a reason unrelated to classification. Measured on AMBA LTI:
`normalization_plan.status` is `Ready`, and `build_unproved_from_source_ir` fails with *"path does
not exist: …/normalized/ihi0089_d….md"* — `assemble_evidence_statements` needs the **normalized
markdown bundle**, and it has been reclaimed. Across the stratum: `status: ready` **51/51**, bundle
present **0/51**.

**The bundle, not the label, is the binding constraint.** Only a re-ingest moves it. Any plan that
recovers classifications in order to widen the canonical reader's reach — including migrating the
artifacts to a current schema — buys nothing until that happens.

## What a re-derived label may be used for

`SourceIr::load_for_inspection` has exactly **one** non-test production consumer in the workspace:
`replay-constraints`, a read-only diagnostic. Neutralization conflates a label's **content** with
its **authority**; only the authority had to go, and
`carries_canonical_source_classifications` already expresses it by keying on the schema version
alone. So a diagnostic could re-derive the content — **and the compiled information-flow graph
refuses the obvious way of doing it**: a function that reads raw evidence and WRITES a semantic
classification is *"raw_evidence reaches semantic control outside its registered region"*. Reading
is fine; writing the label is the reach. `LEGACY-SOURCE-RECLASSIFICATION.1` owns that argument, and
a shape where the classifier stays a pure function may avoid it entirely.

## The counter that would fall to zero is not the one to read

Measured on a prototype before it was reverted: `row_stratum_unjudged_documents` falls **51 → 0**,
and judging those 51 judges an **empty set** — **0 of them carry a single persisted `row_sigcon_*`
record**, so there is nothing to reproduce or fail to reproduce. What the re-derivation buys is
**227 replayed records** across 51 previously unreadable documents, with nothing persisted to
compare them against: a **recall signal and not a reproduction verdict**. Publish all three
counters or none.

Links: [[legacy-artifact-declaration-drift]], [[direction-column-drift]].
