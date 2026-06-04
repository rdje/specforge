# PRIOR-DECAY: detect & flag contested priors (revision-on-contradiction for CorpusMemory)

## Metadata

- Tree ID: `PRIOR-DECAY`
- Status: `active` (`.1` design done; `.2` = implement + close)
- Roadmap lane: `R15c`/`R15e` (cross-document learning / residual honesty)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: user picked it ("prior-decay first then the Dempster combiner"). Grounded gap
  from `docs/research/grounding/cross-document-learning.md` (and the adopt/defer ledger, Parisi
  entry): *"No forgetting/decay — priors only accrete; add staleness or revision when a later
  validated doc contradicts a prior (Parisi-style plasticity)."* Parisi et al., *Continual
  lifelong learning with neural networks*, arXiv:1802.07569.

## The gap (verified against the code)

`CorpusMemory` priors are pure key→value mappings that **only accrete**: e.g.
`ActorTaxonomyPriorRecord { normalized_actor_term → taxonomy_role, support_count,
supporting_document_keys, strongest_automation_confidence }`, `SemanticPhrasePriorRecord
{ normalized_phrase → role, … }`, `TableShapePriorRecord { normalized_header_signature →
table_kind, … }`. When two validated documents map the **same key to different values** (doc A:
`manager → Initiator`; doc B: `manager → Target`), the harvest stores **two independent priors**,
each accreting its own support — and nothing notices the contradiction. A later contradicting
document never *revises* an earlier prior; trust only ever grows. That is the Parisi
plasticity/forgetting gap.

## Scope (bounded, additive, read-only — no behavior/fixture risk)

A **derived, read-only** `CorpusMemory::contested_priors()` that, per prior family, groups
priors by their normalized key and flags any key carried by **≥2 distinct values** as a
**contested prior**, reporting for each: the key, the competing `(value, support_count,
strongest_confidence, supporting_document_keys)` rows, the strongest-supported value, and a
recommended **disposition** (`Contested` → *advisory-only: do not auto-apply; surface the
contest*). Surfaced in the `learn-priors` summary. **No mutation** of the harvest/merge/stored
priors and **no change to consultation** — so zero fixture churn and zero behavior change; this
is the *detect + revise-disposition* half of the gap (like the region-accounting / ambiguity
detectors: report-only honesty). Actually down-weighting contested priors *in consultation* is
a deliberate follow-up; recency/time **staleness** is out of scope (CorpusMemory has no clean
recency ordering — priors are document-keyed, not time-ordered — so "decay" here is
contradiction-driven revision, not temporal decay; noted as a Non-Goal).

## Families covered

Families with an unambiguous single-expected-value per key (a second value = a genuine
contradiction): **ActorTaxonomy** (`normalized_actor_term → taxonomy_role`), **SemanticPhrase**
(`normalized_phrase → role`), **TableShape** (`normalized_header_signature → table_kind`). The
fuzzier families (TemporalPhrase, ModalityReliability, VisualMotif — where one key may
legitimately carry several shapes) are **out of this slice** (extensible later); documented.

## Non-Goals

- NOT mutating stored priors, the harvest, the merge, or consultation (read-only analysis).
- NOT temporal/recency decay (no clean recency in document-keyed priors) — only
  contradiction-driven revision.
- NOT the fuzzy families (TemporalPhrase/ModalityReliability/VisualMotif) in this slice.
- NOT auto-resolving a contest (advisory: surface it; the strongest-supported value is reported
  as a hint, not auto-applied).

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: `contested_priors()` + a `ContestedPrior` record (key, family, competing values,
  strongest, disposition) on `CorpusMemory`; unit tests (a contested ActorTaxonomy/SemanticPhrase
  key flagged; a single-value key NOT flagged; a one-doc key NOT flagged; the strongest-supported
  value reported); surfaced in the `learn-priors` summary (a "Contested priors" section + count);
  a user-friendly book subsection; a KM card; full `scripts/run_ci.sh` GREEN (no existing
  learn-priors/kg-bench test broken — additive); tree CLOSED.

## Task Tree

- ID: `PRIOR-DECAY`
  Status: `active`
  Children: `.1` (design) · `.2` (implement read-only contested-prior detection + surface + close)

- ID: `PRIOR-DECAY.1`
  Status: `done`
  Goal: own + design (this file) — the gap (priors-only-accrete, verified), the bounded
    read-only `contested_priors()` slice (no mutation, no fixture risk), the covered families,
    the staleness/consultation-downweight deferrals.
  Acceptance: design recorded; registered.
  Verification: passed (`2026-06-04`) — gap verified against `prior_memory.rs` (key→value priors,
    accrete-only, `support_count`/`supporting_document_keys`/`strongest_automation_confidence`,
    no contradiction detection); slice fixed = read-only `contested_priors()` over
    ActorTaxonomy/SemanticPhrase/TableShape, report-only (no harvest/merge/consultation change →
    zero fixture/behavior risk); Parisi-grounded; temporal staleness + consultation down-weight
    deferred with reasons.
  Commit: `see Commit Log`

- ID: `PRIOR-DECAY.2`
  Status: `pending`
  Goal: implement `contested_priors()` + `ContestedPrior` + tests; surface in `learn-priors`;
    book subsection; KM card; close.
  Acceptance: tests green; surfaced; book + KM; full CI GREEN; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `PRIOR-DECAY.1` | `done` | owned + designed (read-only contested-prior detection) |
| 2 | `PRIOR-DECAY.2` | `pending` | implement + surface + tests + book + KM + close |

## Decisions

- `2026-06-04`: realize the Parisi revision gap as **read-only contested-prior detection**
  (additive, no mutation) — the safe, bounded, residual-honesty slice. Defer temporal staleness
  (no recency ordering) and consultation down-weighting (behavior change) to follow-ups.

## Blockers

- None. Additive read-only analysis over existing prior records.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | gap verified in `prior_memory.rs`; bounded read-only `contested_priors()` slice (ActorTaxonomy/SemanticPhrase/TableShape; no mutation; Parisi-grounded); deferrals reasoned | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PRIOR-DECAY.1` | `PRIOR-DECAY.1 — own + design contested-prior detection (revision-on-contradiction)` | docs-only |

## Changelog

- `2026-06-04`: Created — detect & flag **contested priors** (same key, conflicting values across
  documents) in `CorpusMemory`, the Parisi-grounded revision-on-contradiction gap. Read-only /
  additive (no harvest/merge/consultation change); temporal staleness + consultation
  down-weighting deferred.
