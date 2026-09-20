# LEGACY-SOURCE-RECLASSIFICATION: 51 documents are invisible to every classification-keyed reader, and the labels are re-derivable without a PDF

## Metadata

- Tree ID: `LEGACY-SOURCE-RECLASSIFICATION`
- Status: `active` (`2026-09-20`; `.0` CLOSED — the decision is that the bundle, not the label, is
  the binding constraint, so the tree's stated prize needs a re-ingest it forbids. `.1` is open: the
  compiled information-flow graph refuses a diagnostic that WRITES a re-derived semantic label)
- Roadmap lane: `R2`/`R8` (extraction correctness and recall) with a `SPEC-TO-INTENT-ALIGNMENT` proof dependency
- Created: `2026-09-13`
- Last updated: `2026-09-20`
- Owner: repo-local workflow
- Owner directive (`2026-09-13`): *"the table-row producer should be able to see all documents … regardless we should
  get rid of it, of course."*

## Goal

Give every classification-keyed reader — starting with `extract_signal_description_row_constraints` — access to all
78 persisted documents, without re-admitting a label the current classifier would not produce.

## The limit, and where it came from

`SourceIr::load_for_inspection` calls `neutralize_legacy_source_classifications`, which sets `table_kind`,
`diagram_kind` and `section_kind` to `Unknown` on every schema-1 artifact. Introduced by
`SPEC-TO-INTENT-ALIGNMENT.6d.ii.b` (`dee0740f`, `2026-08-12`, *"make SourceIR classification neutral"*) and
hardened by `.6d.ii.e.iv.ii` (`bb5047c2`, *"prove SourceIR authority"*).

**It is not an arbitrary cap and it was not a size or performance decision.** Those labels were produced by a
**corpus-calibrated classifier** — one that keyed on document, vendor and protocol identity, which ADR 0006
forbids. That classifier was retired and replaced with structural generic roles. The old labels are therefore not
merely old: they are the output of a policy this project has ruled illegitimate, and a read-only 78-document
replay measured the difference as **2,293 diagram, 2,123 section and 3,484 table label changes**. Neutralizing
them refuses to let a retired non-generic classifier keep authority over current extraction. That reasoning is
sound and this tree does not overturn it.

**What was never true is that the labels could not be recovered.** An audited maintenance path re-derived the 24
documents whose capture bundles were retained; the other 54 (51 today) were left inspection-only, and the
inference that they therefore *cannot* be reclassified was never tested.

## The measurement that changes the answer

`classified_table_kind(table: &StructuredTableRecord) -> TableKind` is a **pure function of the persisted table
record** — caption, header rows, body rows. Every one of those survives the legacy load; only the LABEL is
withdrawn. So the current generic classifier can be run over a legacy artifact's own preserved structure, with no
PDF, no Docling and no model.

Measured `2026-09-13` by running the real `classified_table_kind` over all 51 legacy artifacts as loaded (labels
already neutralized):

| | |
| --- | --- |
| legacy documents | **51** |
| tables the current classifier gives a non-`Unknown` role | **1,744** |
| of those, `SignalDescription` | **363**, across **23** documents |

Recovered signal-description tables include AMBA AXI/ACE **89**, AMBA LTI **24**, CoreSight TMC **7**, CHI **8**,
HBM2 **8**, TileLink **7 + 7**, CXS **7**, eMMC **5**, APB (rev D) **8**. The table-row constraint reader
currently sees **102** such tables across 27 documents; this is a 3.5× increase in its reachable surface.

## Non-Goals

- Do not re-admit the persisted schema-1 labels. They are the retired classifier's output and the 3,484-change
  delta is the evidence that they differ from what this code would say.
- Do not weaken `carries_canonical_source_classifications` as a *proof* predicate. A re-derived label is a
  current-classifier fact; whether it carries canonical authority is a separate question this tree must answer
  rather than assume.
- Do not re-ingest. The whole point of the measurement is that re-ingest is not required for this.

## Task Tree

- ID: `LEGACY-SOURCE-RECLASSIFICATION` · Status: `active` (`2026-09-20`) · Children: `.0`

- ID: `LEGACY-SOURCE-RECLASSIFICATION.0` · Status: `done` (`2026-09-20`; opened `2026-09-13`) · Goal:
  **decide and implement where a re-derived classification is allowed to have authority.**

  **THE MEASUREMENT REPRODUCES; TWO OF ITS DOCUMENT COUNTS DO NOT.** Re-derived `2026-09-20` by
  running the real `classified_table_kind` over all 51 legacy artifacts with their labels
  neutralized exactly as the loader leaves them: **51 legacy documents, 1,744 non-`Unknown` tables,
  363 `SignalDescription`** — all three exact. But they span **30 documents, not 23**; **344 of the
  363 across 24 documents** pass the row reader's own top-level gate, which is probably what the 23
  was reaching for. And *"the table-row constraint reader currently sees 102 such tables across 27
  documents"* is **102 tables across 4 documents** — 27 is the proof-carrying document count, not
  the count of documents contributing such a table.

  **THE DECISION: none of (a), (b) or (c) as framed, because the prize they compete over is not
  reachable by any of them.** The tree's claim is that recovering these labels is *"a 3.5x increase
  in [the row reader's] reachable surface"*. That reader runs inside the **EvidenceIR build**, and a
  legacy artifact cannot enter it for a reason that has nothing to do with classification. Measured
  on AMBA LTI, this tree's own demonstration document: `normalization_plan.status` is `Ready`, and
  `build_unproved_from_source_ir` fails with *"path does not exist:
  …/normalized/ihi0089_d….md"* — the **normalized markdown bundle has been reclaimed**, and
  `assemble_evidence_statements` needs it before any classification is consulted. All 51 are in that
  state (`status: ready` 51/51, bundle present **0/51**). **The bundle, not the label, is the
  binding constraint**, and only a re-ingest moves it — which this tree's own non-goals forbid and
  `CORPUS-CHAIN-CURRENCY.10a` already refused for the golds.
  So **(c) is the most work for no gain**, and **(a) buys nothing over (b)**: `SourceIr::load_for_inspection`
  has exactly **one** non-test production consumer in the whole workspace — `replay-constraints`.
  A loader-wide change and a one-consumer opt-in are the same change here, and only the second says so.

  **THE SHAPE IS (b), NARROWLY, AND THE IMPLEMENTATION IS `.1` — because the compiled
  information-flow graph refused it, and that refusal is worth more than the code was.** A
  prototype that recomputed `table_kind` on the loaded artifact and handed it to
  `replay-constraints` worked and produced the numbers below, then the gate rejected it:
  `production-genericity-graph: rederive_table_classifications: raw_evidence reaches semantic
  control outside its registered region`. The function reads raw evidence (caption, header rows,
  body rows) and WRITES a semantic classification, which is precisely the reach the 4th portable
  architecture exists to stop. Registering a new `diagnostics_only` region is the documented
  precedent (`EXTRACTION-QUALITY-GAUGE.3k.6`, `SIGNAL-DECLARATION-ROW-DROP.4e`) — but those
  diagnostics only REPORT, while this one's re-derived label DRIVES a production producer, so the
  precedent does not simply transfer. **Moving that boundary is a change to the product's authority
  structure and needs its own justification**, which is `.1`'s, not this leaf's. The prototype was
  reverted rather than registered in passing.

  **AND THE COUNTER THE LEAF ASKED FOR IS THE WRONG ONE TO PUBLISH ALONE — measured on the
  prototype before it was reverted.** `row_stratum_unjudged_documents` does fall **51 -> 0**,
  exactly as the leaf's verification specified. But judging those 51 judges an **EMPTY SET**:
  measured, **0 of them carry a single persisted `row_sigcon_*` record**, so there is nothing there
  to reproduce or fail to reproduce. Reporting `judged: 78` without that would be the inverse of the
  silent zero this instrument exists to retire. What the re-derivation buys is
  **227 replayed records** — records the CURRENT row producer mints across those 51 previously
  unreadable documents, with nothing persisted to compare them against, so a **recall signal and not
  a reproduction verdict**. `.1` must publish all three or none.
  Non-goal: re-admitting the persisted schema-1 labels (they are the retired classifier's output);
  weakening the proof predicate; re-ingesting.
  Prerequisite: none.
  Verification: see the acceptance checklist below.
  Commit: `LEGACY-SOURCE-RECLASSIFICATION.0 — the bundle, not the label, is the binding constraint`

- ID: `LEGACY-SOURCE-RECLASSIFICATION.1` · Status: `pending` (opened `2026-09-20` by `.0`) · Goal:
  **decide whether a diagnostic may hold a re-derived semantic label, and register it if so.**
  `.0` settled WHAT to build and measured what it is worth (227 replayed records over 51 previously
  unreadable documents, and the honest caveat that the counter which moves is the vacuous one). What
  it did not settle is whether the thing may exist where the compiled graph can see it.
  **The exact refusal, reproduced:** `specforge-core:…:impl_fn:rederive_table_classifications:
  raw_evidence reaches semantic control outside its registered region`. Reading raw evidence is
  fine; writing a semantic classification from it is the reach.
  **Why the precedent does not simply transfer.** `EXTRACTION-QUALITY-GAUGE.3k.6` and
  `SIGNAL-DECLARATION-ROW-DROP.4e` each registered a `non_authoritative_root` / `diagnostics_only`
  row for a replay that READS raw evidence and REPORTS. This one's output is consumed by a
  production producer (`extract_signal_description_row_constraints`) to select tables, inside a
  read-only command. That is a different claim and the leaf must argue it rather than inherit it.
  **A shape that may avoid the question entirely, and should be measured first:** expose the
  classifier as a PURE function (raw evidence in, a value out, no control write) and let the
  application-layer command hold the mutation. If the graph accepts that, no boundary moves and the
  answer is better than a registration.
  Acceptance: either a registered region with its rationale and the boundary counts re-pinned and
  named, or the pure-function shape demonstrated with the boundary untouched; plus `.0`'s three
  counters published together and `row_stratum_unjudged_documents` falling 51 -> 0.
  Non-goal: any canonical-pipeline change; the bundle, not the label, still bars that road (`.0`).
  Prerequisite: `.0`.
  Verification: pending
  Commit: pending

## Acceptance Checklist (enforced) — `LEGACY-SOURCE-RECLASSIFICATION.0`

- [x] **REPRODUCE / MEASURE** — `classified_table_kind` over all 51 legacy artifacts, labels
  neutralized as loaded: **51 documents, 1,744 non-`Unknown`, 363 `SignalDescription` across 30
  documents**, of which **344 across 24** pass the row reader's top-level gate; the reader sees
  **102 tables across 4 documents** today. Legacy `normalization_plan.status`: `ready` **51/51**;
  promoted markdown bundle present: **0/51**.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs`
  `neutralize_legacy_source_classifications` withdraws `table_kind`, `diagram_kind` and
  `section_kind` on every schema-1 artifact. That is right for AUTHORITY and unnecessary for
  CONTENT: `classified_table_kind` is a pure function of caption + header rows + body rows, all of
  which survive the load. The reason the recovered labels still cannot reach the canonical reader is
  elsewhere and was measured rather than assumed: `assemble_evidence_statements` needs the
  reclaimed normalized bundle, demonstrated failing on AMBA LTI.
- [x] **ADDRESSED (verified)** — the decision is made on evidence and the prototype that proves the
  numbers was built and measured before being reverted: `replay-constraints --evidence-root
  generated/evidence_ir` read `row_stratum_unjudged_documents` **51 -> 0**,
  `row_stratum_judged_from_rederived_labels` **51**, and the number that matters, **227** replayed
  records. Every other line was unchanged — `persisted_deterministic_records` 195, `reproduced` 122,
  `not_reproduced` 73, no per-document verdict moving — because no legacy artifact carries a record
  for the new stratum to judge. **That is what makes the vacuity claim measured rather than argued.**
- [x] **NO REGRESSION** — **this leaf ships no Rust.** The prototype is reverted; the tree, the fact
  card and the resume pointer are the deliverable. `cargo fmt --all -- --check` exit 0;
  `cargo clippy --offline --all-targets -- -D warnings` exit 0; workspace lib tests green;
  `scripts/check_doctrines.sh` green with `INFORMATION-FLOW` and `PRODUCTION-GENERICITY` back to
  PASS. No persisted artifact, gold or seal is touched.
- [x] **GENERICITY (ADR 0006)** — N/A: no rule ships. The finding that the boundary refuses this
  shape is itself the genericity control working, and `.1` owns the argument.
- [x] **LOCKSTEP** — **no book change, deliberately.** `replay-constraints`' output is unchanged
  because the prototype was reverted, and documenting behaviour that does not ship is exactly the
  drift the `AUDIT-DOC-RECONCILE` doctrine forbids. The durable surfaces are this node and
  `[[legacy-reclassification-is-not-the-binding-constraint]]`. No production rule is deleted or
  replaced.

## Changelog

- `2026-09-13` — opened from the `EXTRACTION-QUALITY-GAUGE.3k` container amendment, which measured four consecutive
  row-path leaves at an actionable population of zero and attributed all four to this limit. The director's
  question — *who set this and why* — is answered above; the answer is that the gate is correct and the inference
  drawn from it (that the 51 are unreachable) was not.
