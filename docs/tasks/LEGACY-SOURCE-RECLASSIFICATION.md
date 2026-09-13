# LEGACY-SOURCE-RECLASSIFICATION: 51 documents are invisible to every classification-keyed reader, and the labels are re-derivable without a PDF

## Metadata

- Tree ID: `LEGACY-SOURCE-RECLASSIFICATION`
- Status: `active` (`2026-09-13`; `.0` open)
- Roadmap lane: `R2`/`R8` (extraction correctness and recall) with a `SPEC-TO-INTENT-ALIGNMENT` proof dependency
- Created: `2026-09-13`
- Last updated: `2026-09-13`
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

- ID: `LEGACY-SOURCE-RECLASSIFICATION` · Status: `active` (`2026-09-13`) · Children: `.0`

- ID: `LEGACY-SOURCE-RECLASSIFICATION.0` · Status: `pending` (opened `2026-09-13`) · Goal: **decide and implement
  where a re-derived classification is allowed to have authority.** The measurement above establishes that the
  labels are recoverable; what it does not establish is which consumers may act on them. Three candidate shapes,
  and the leaf must choose on evidence rather than convenience:
  (a) **re-derive on load** — `load_for_inspection` replaces neutralization with `classified_table_kind` over the
  artifact's own tables. Cheapest, and it immediately unblocks every classification-keyed reader; but it gives a
  non-proof-carrying artifact a classification that looks canonical, which is the exact conflation
  `SPEC-TO-INTENT-ALIGNMENT.6d.ii.e.iv.ii` closed.
  (b) **re-derive for diagnostics only** — an explicit opt-in the row-constraint reader and `replay-constraints`
  request, leaving the canonical loader untouched. Honest about authority, but every consumer must opt in and the
  two paths can drift.
  (c) **migrate the artifacts** — write the re-derived labels back at schema 3 with a proof context that records
  *derived-from-persisted-structure* as its premise. Strongest, and the only one that makes the recovered surface
  canonical rather than advisory; most work, and it needs the proof kernel to accept a premise kind that is not a
  capture.
  Prerequisite: none for the decision; (c) depends on the promotion kernel's premise vocabulary. Verification: the
  chosen shape demonstrated on AMBA LTI, whose `table_0031` is the corpus's known instance — 24 tables recovered,
  and the row reader's population re-derived with `replay-constraints` before and after; `row_stratum_unjudged_documents`
  falls from 51; observed RED; no persisted artifact's *content* changes under (a) or (b).

## Changelog

- `2026-09-13` — opened from the `EXTRACTION-QUALITY-GAUGE.3k` container amendment, which measured four consecutive
  row-path leaves at an actionable population of zero and attributed all four to this limit. The director's
  question — *who set this and why* — is answered above; the answer is that the gate is correct and the inference
  drawn from it (that the 51 are unreachable) was not.
