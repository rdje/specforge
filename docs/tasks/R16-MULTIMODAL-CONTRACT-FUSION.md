# R16-MULTIMODAL-CONTRACT-FUSION: cross-modal evidence → one contract (point #3)

## Metadata

- Tree ID: `R16-MULTIMODAL-CONTRACT-FUSION`
- Status: `proposed`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #3, order 4)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Today `temporal_rules` are built ~per-sentence/row, so a single
obligation spread across prose §3.1 + a timing table in §3.4 + Figure
3-2 + an exception in §3.5 is never assembled into one object — recall
is silently lost at the join. Add a **contract-assembly** phase that
clusters multimodal evidence keyed by `(actor, channel/signal-group,
phase)` into one ContractIR object with: full multi-modal provenance, a
typed merge, and an explicit **disagreement surface** (sources that
contradict become a residual/repair packet, not a silent pick).

## Non-Goals

- Not a new extractor — it fuses candidates produced by existing
  extraction + #4/#6 into the typed contract; it does not invent
  obligations no source licenses.

## Acceptance Criteria

- A fusion pass clusters by `(actor, channel/group, phase)` (using the
  `R16-KG-PROTOCOL-ONTOLOGY` identity), merges into one ContractIR
  contract with provenance, and surfaces inter-source disagreement as an
  explicit residual; measured by `R16-CAPTURE-FIDELITY-GATES`.
- Recall improvement demonstrated on the real corpus vs. the
  pre-fusion baseline; `scripts/run_ci.sh` green per leaf.

## Task Tree (proposed; expands at promotion)

- Children (sketch): `.1` fusion key + typed-merge + disagreement-policy
  design → `.2` clustering + provenance-preserving merge → `.3`
  disagreement → residual/repair routing → `.4` corpus recall eval +
  close

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (#1), `R16-KG-PROTOCOL-ONTOLOGY` (#2),
  `R16-CAPTURE-FIDELITY-GATES` (#5/order-3). Consumes candidates from
  `R16-WAVEFORM-CONTRACT-MINING` (#4) and
  `R16-CONSTRAINED-VERIFIED-EXTRACTION` (#6).

## Decisions

- `2026-05-19`: The join/aggregation step that converts statement-local
  facts into protocol-level intent; created `proposed`.

## Blockers

- None (proposed; promotion after its DAG predecessors).

## Changelog

- `2026-05-19`: Created `proposed` as program point #3, ordered 4th.
