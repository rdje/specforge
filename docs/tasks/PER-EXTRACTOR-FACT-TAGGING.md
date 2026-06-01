# PER-EXTRACTOR-FACT-TAGGING: record which extractor found each fact (recall-gauge precondition)

## Metadata

- Tree ID: `PER-EXTRACTOR-FACT-TAGGING`
- Status: `active`
- Roadmap lane: `R15e` (KG-quality / completeness)
- Created: `2026-06-01`
- Owner: repo-local workflow

## Goal

Build the **precondition** the capture–recapture recall gauge is blocked on
(`INTENT-COMPLETENESS-RESEARCH.5` §3): the pipeline currently MERGES the
extractors' outputs into one `EvidenceIR` (with dedup), so "which extractor found
this fact" and "did two extractors find the same fact" are lost — yet that
overlap is exactly what capture–recapture needs to estimate the unseen
population. This tree records, per extractor, the facts it found — **including
overlaps** (pre-dedup) — under a **canonical key** so the same fact found by two
extractors is recognizable.

First fact type: **`SignalConstraintRecord`** — the central fact produced by TWO
genuinely independent extractors (the **Pattern** Tier-1/2 prose extractor at
`evidence` build, and the **LLM** Tier-3 at `nlp-enrich`), so it yields the first
real Pattern-vs-Nlp overlap the gauge can use.

## Non-Goals

- NOT the recall gauge itself (Chao Mh estimate + report) — that is the next
  owned tree, built on this index. This slice only records the per-extractor
  finds.
- NOT adding a field to `SignalConstraintRecord` (140 literal sites) — use a
  separate provenance index populated at the ~handful of producer sites.
- NOT changing extraction outcomes — additive recording only.

## Design

- `ExtractorTier { Pattern, Nlp, Vlm }` — the independent extractor families.
- `FactKind { SignalConstraint, … }` — extensible; SignalConstraint first.
- `FactProvenanceRecord { producer: ExtractorTier, fact_kind: FactKind,
  canonical_key: String }` — the canonical key normalizes the fact
  (subject signal upper-cased + constraint kind + target) so the SAME constraint
  found by Pattern and Nlp produces the SAME key (overlap detectable).
- `EvidenceIr.fact_provenance: Vec<FactProvenanceRecord>` (`serde(default,
  skip_serializing_if = empty)` — back-compat).
- **Pattern tagging** at `EvidenceIr::build` (after `converge_evidence_extractions`):
  every produced `signal_constraint` → a `(Pattern, SignalConstraint, key)` entry.
- **Nlp tagging** at `nlp_enrich.rs` (where an LLM `SignalConstraint` is
  extracted, **pre-dedup**, ~line 248): push a `(Nlp, SignalConstraint, key)`
  entry if that triple isn't already present (dedup the index, not the finds), so
  an Nlp find that duplicates a Pattern find is recorded as the **overlap**.
- A small `validate` visibility line + metrics (`fact_provenance_pattern` /
  `fact_provenance_nlp`) so the tagging is inspectable + testable.

## Acceptance Criteria

- The types + `EvidenceIr.fact_provenance` exist; Pattern finds tagged at build,
  Nlp finds tagged at nlp-enrich (pre-dedup, overlaps captured), index deduped.
- Canonical key normalizes so Pattern and Nlp finds of the same constraint match.
- Unit + wiring tests (key normalization; Pattern tagged at build; Nlp tagged +
  overlap recorded across passes). Extraction-neutral; full CI green; book note.

## Task Tree

- ID: `PER-EXTRACTOR-FACT-TAGGING`
  Status: `active`
  Goal: per-extractor fact-provenance index (SignalConstraint) — recall-gauge precondition
  Children: `.1`, `.2`

- ID: `PER-EXTRACTOR-FACT-TAGGING.1`
  Status: `done`
  Goal: own + design (this file); verify producer sites; register. Docs-only.
  Verification: >
    passed (`2026-06-01`) — owned; verified `SignalConstraintRecord` is produced
    by Pattern (`converge_evidence_extractions` at build) and Nlp
    (`nlp_enrich.rs:225–248`, pre-dedup) — the two independent extractors needed
    for a Pattern-vs-Nlp overlap; chose a provenance-INDEX design (not a 140-site
    struct field). Registered.
  Commit: `see Commit Log`

- ID: `PER-EXTRACTOR-FACT-TAGGING.2`
  Status: `pending`
  Goal: >
    Implement `ExtractorTier`/`FactKind`/`FactProvenanceRecord` +
    `EvidenceIr.fact_provenance` + `signal_constraint_fact_key`; tag Pattern at
    build + Nlp at nlp-enrich (pre-dedup, overlap-recording, deduped index);
    `validate` visibility + metrics; unit/wiring tests; book note; full CI; close.
  Acceptance: index populated (Pattern + Nlp + overlap) + canonical-key match + validate surface; CI green; book; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `PER-EXTRACTOR-FACT-TAGGING.1` | `done` | owned + design + producer sites verified |
| 2 | `PER-EXTRACTOR-FACT-TAGGING.2` | `pending` | implement the index + tagging + validate + tests + book + close — next |

## Decisions

- `2026-06-01`: provenance INDEX (not a `SignalConstraintRecord` field) — 140
  literal sites make a field change too invasive; the index is populated at the
  ~4 producer sites and is the semantically-right home for "who found what".
- `2026-06-01`: tag Nlp finds **pre-dedup** (capture the overlap with Pattern);
  dedup the index (unique producer+kind+key), not the finds — capture–recapture
  needs the per-extractor sets WITH overlaps.

## Open Questions

- Extend to `ActorSignalRelation` (also multi-produced: pattern + table +
  `signal-resolve` LLM) + `ConditionalRule` — follow-on once the SignalConstraint
  slice proves the surface.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | producer sites verified (Pattern@build, Nlp@nlp_enrich:248 pre-dedup); index design chosen over 140-site field; registered; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PER-EXTRACTOR-FACT-TAGGING.1` | `PER-EXTRACTOR-FACT-TAGGING.1 — own + design the fact-provenance index` | docs-only |

## Changelog

- `2026-06-01`: Created — own the per-extractor fact-tagging precondition for the
  capture–recapture recall gauge; provenance-index design over SignalConstraint
  (Pattern vs Nlp, overlap-capturing). Frontier → `.2` (implement).
