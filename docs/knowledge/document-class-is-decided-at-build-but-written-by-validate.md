---
id: document-class-is-decided-at-build-but-written-by-validate
title: document_class is a pure function of five EvidenceIR structural counts, yet only validate computes it and back-annotates it into the artifact — a fresh evidence build carries no class at all, and the SourceIR front-matter it seems to need reaches the under-extraction flag, not the class
answers:
  - "what determines a document's document_class"
  - "does document_class depend on the SourceIR front-matter"
  - "where is document_class computed and where is it stored"
  - "why does an EvidenceIR artifact have no document_class"
  - "what does under_extracted_spec mean and what produces it"
  - "what breaks if I clean a document's source_ir"
  - "does a freshly built EvidenceIR carry a document_class"
  - "what clean scopes does specforge offer"
  - "how many corpus documents are flagged as under-extracted specs"
  - "what is the document_class distribution across the corpus"
  - "where does declared_type come from in validate"
  - "is a census that returns zero a finding"
  - "why does a rebuilt chain report the previous binary's document category"
  - "is validation_report.json current after specforge evidence"
  - "does a refresh need specforge validate"
date: 2026-09-20
status: current
tags: [extraction, validation, document-class, corpus, method, provenance]
evidence: crates/specforge/src/ir/completeness.rs (classify_document); crates/specforge/src/commands/validate.rs (DocumentClassCensus site); docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.5)
reverify: "run `specforge evidence <a current source_ir.json> --dry-run` and read validation_reports — expect 0 and no document_class, against the persisted artifact for the same document which carries one. Then for each generated/evidence_ir/*/validation_report.json read metrics[] by name — expect 78 of 78 to carry document_class, distributed guide 28 / protocol 20 / register 16 / interface 14; then count findings[] whose finding_id is evidence_document_class and whose summary contains UNDER-EXTRACTED — expect 16, all of them guide-class. Read classify_document to confirm the class expression names only registers, declared_signals + actor_signal_relations, signal_constraints, fsm_states and serial_frame_fields"
---

## The class is decided by the EvidenceIR alone

`classify_document` picks the class from **five structural counts and nothing else**:

| class | condition |
| --- | --- |
| `Register` | `registers >= floor` and `registers >=` both the interface surface and `signal_constraints` |
| `Protocol` | `signal_constraints >= floor`, or any `fsm_states` / `serial_frame_fields` |
| `Interface` | `declared_signals + actor_signal_relations >= floor` |
| `Guide` | otherwise |

Every input is an `ir.<field>` read at the census site in `validate.rs` — including `declared_signals`,
which is `collect_known_signal_names(&ir.extracted_statements)` and not an upstream read. **So the class
is fully determined the moment the EvidenceIR is written.**

**But only `validate` computes it.** `evidence.rs` contains zero references to the classifier. `validate`
computes the class and **back-annotates its report into `evidence_ir.json`**, so the value does end up in
the artifact — all 78 persisted artifacts embed a `validation_reports[0]` carrying the `document_class`
metric, byte-identical to the sibling `validation_report.json`. **The gap is ordering, not location.**

Proven by execution, not inference: `evidence --dry-run` on a current chain yields `validation_reports: 0`
and no class, while the persisted artifact for the same document carries `document_class: guide`. And the
path is documented — `README.md`'s staged run is `ingest`, `evidence`, `semantic`, `intent`, and `validate`
appears nowhere in it. `SIGNAL-CATALOG-CAPTURE-GAP.7` proposed moving the computation to the `evidence` stage and
was superseded by `.7a` (`2026-09-21`): the classifier lives in `specforge-conformance`, which core cannot depend on,
and no production path reads the class.

**A rebuild leaves the previous build's report in place, and a measurement will read it.** `specforge evidence`
rewrites `evidence_ir.json` with no embedded report and does not touch the sibling `validation_report.json`, so
after a rebuild without `validate` the sibling still describes the artifact the previous binary built.
`EXTRACTION-REPRESENTATION-GAPS.3.a` re-ingested four documents that way on `2026-09-26` and published their
categories from `2026-08-12` reports: CHI C2C read register-or-platform, while the current binary says
wire-protocol (high). The route that leaves a chain fully described validates each stage once, before the next
is built: `evidence`, `validate`, `semantic`, `validate`, `intent`, `validate`, `adapt`, `validate`
([[retained-chain-rebuild-order]]). A reader of the sibling report should refuse one older than its artifact,
as `scripts/measure_category_family_census.py` now does.

Measured `2026-09-20` over all 78 persisted artifacts: `guide` 28, `protocol` 20, `register` 16,
`interface` 14 — 78 of 78 classified.

## The SourceIR dependency is real, and it is not on the class

The census site reads `declared_type` from the **sibling SourceIR**'s front-matter (title plus the
first twelve section headings) and its comment warns the value is `Unknown` "when the SourceIR is
unavailable". That reads like a class hazard. **It is not.** The class expression above never mentions
`declared_type`. What it reaches is:

- `under_extracted_spec = class == Guide && declared_type == Specification`, and
- the human-readable rationale.

`message_field_records`, `signal_presence_records`, `front_matter_isa` and `front_matter_phy` are
likewise recognizer-only and explicitly ignored by `classify_document`.

## What it would cost if the SourceIR were ever lost — and why that is not reachable today

`under_extracted_spec` is the sentence *"likely an UNDER-EXTRACTED spec (image/table-heavy), not a
true guide → route to VLM rescan, do not dismiss"*. **16 of the 28 guide-class artifacts carry it**
(RISC-V AIA, Bosch CAN, CoreSight BSA, two AMBA CHI C2C revisions, JESD235 HBM, seven OpenCAPI
documents, SMBus 3.3.1, I2S, USB4 inter-domain), and it is read through
`SourceIr::load_from_path(&ir.source_ir_path).ok()` — a best-effort load whose failure becomes `None`.
**So losing the SourceIR would cost the flag and keep the class.** That asymmetry is the durable fact.

**It is not currently reachable, and saying otherwise was a published error** (`SIGNAL-CATALOG-CAPTURE-GAP.5a`).
`CleanScopeArg` offers exactly three scopes — `source-normalized` (normalized bundles only), `document`
(all four stages plus adapters, so the EvidenceIR goes too) and `all-generated`; **none** yields
*EvidenceIR present, SourceIR absent*. Chain-currency reclamation likewise targets the normalized
markdown bundle, not `source_ir.json`. All 78 `source_ir_path` values are repository-relative and all 78
resolve, so repo relocation does not reach it either. And it would not be silent: the `None` arm prints
`(skipped — upstream SourceIR not available)`. Reachability is **nil**; this is recorded as a latent
property so a change that makes it reachable is caught, not as a live risk.

## A zero from an unvalidated selector is not a measurement

Two censuses run while establishing the above returned **0**, and both were wrong:

- *"no report carries `document_class`"* — it is a `metrics[]` entry, not a top-level key.
- *"no guide-class document is flagged UNDER-EXTRACTED"* — findings carry `finding_id` and `summary`,
  not `code` and `message`.

Either zero, published, would have been a confident false finding with a plausible story attached.
**Before a 0 counts, show the selector matching something** — print one real record and read its keys.

And the companion rule, learned the harder way in `.5a`: **reading one function does not license a claim
about a second one.** The claim that a `clean` scope reached this state was inferred from having read
`classify_document` carefully; the scope enum was never opened and the command was never run. A published
hazard needs its trigger executed, or its reachability measured and stated as nil.

Links: [[direction-suffix-is-one-documents-convention]].
