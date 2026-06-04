# AMBIGUITY-PHRASE-DETECTOR: flag vague / under-specified spec prose for review

## Metadata

- Tree ID: `AMBIGUITY-PHRASE-DETECTOR`
- Status: `done` (CLOSED `2026-06-04` — flag-only weak-phrase/ambiguity detector surfaces vague
  spec prose in `validate`; grounded NASA ARM/Berry-Kamsties; extraction-neutral; CI green)
- Roadmap lane: `R8`/`R15e` (extraction quality / residual-honesty)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: a grounded gap from the `LITERATURE-GROUNDING` backlog
  (`docs/research/grounding/requirements-extraction.md`): *"implement a Berry-Kamsties
  weak-phrase/ambiguity classifier that routes flagged sentences to residual decisions rather
  than silently dropping them."* The requirements-engineering literature (Wilson, Rosenberg &
  Hyatt — NASA ARM, ICSE 1997, DOI 10.1145/253228.253258; Berry & Kamsties ambiguity handbook)
  established that **weak phrases** ("as appropriate", "if necessary", "and/or", "TBD", …) mark
  language a tool cannot pin down. SpecForge silently treats such prose as precise; this
  surfaces it, in the residual-honesty spirit (say what you *cannot* resolve, don't fake it).

## Goal

A **flag-only** detector that scans EvidenceIR statement prose for vagueness /
under-specification indicators and surfaces the flagged statements in `validate` — so a user
sees where the spec is genuinely vague (and extraction is necessarily uncertain) instead of
it being silently dropped. Extraction-neutral: the IR's extracted facts are unchanged; only
the validation report gains a finding + metric (exactly like the region-accounting and
register-tiling completeness detectors).

## The lexicon (high-precision, grounded; excludes modal verbs)

A curated set of vagueness / under-specification markers — NASA ARM "weak phrases" plus the
chip-spec under-specification idioms that matter for this domain. It **excludes** the RFC 2119
modal verbs (MUST/SHALL/SHOULD/MAY) — those carry *normative strength*, handled by the
constraint/obligation extraction, and flagging every "may" would be noise.

- as appropriate · as applicable · as required · as necessary · as needed · where appropriate
  · where applicable · where necessary · if appropriate · if necessary · if possible ·
  if practical · if needed
- and/or · etc. · to be determined · to be defined · to be decided · tbd
- but not limited to · not limited to · as a minimum · at a minimum
- implementation-defined · implementation defined · implementation specific ·
  vendor-specific · vendor specific

(Chip specs genuinely use `implementation-defined` / `vendor-specific` to mark
under-specified behaviour — high-value, high-precision flags here.)

## Non-Goals

- NOT routing flagged statements into typed `ResidualDecisionPacket`s yet (that is an IR
  change + a bigger step) — this tree FLAGS them in `validate`; residual routing is a future
  follow-up.
- NOT flagging modal verbs (MUST/SHALL/SHOULD/MAY) — normative strength, not vagueness.
- NOT changing extraction behaviour, the IRs, or any extracted fact — flag-only.

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: a pure `crates/specforge/src/ir/ambiguity.rs` (`WEAK_PHRASES` lexicon +
  `weak_phrase_findings(&[ExtractedStatement]) -> Vec<WeakPhraseFinding>`, case-insensitive),
  with unit tests (a flagged statement; a clean statement; case-insensitivity; a modal-only
  statement is NOT flagged). Wired into `validate_evidence_ir`: a summary line, an `Info`
  finding `evidence_ambiguous_statements` (capped list), and an `ambiguous_statements` metric.
  A book note (`quality/validation.md` or the evidence chapter) + a Knowledge Map card. Full
  `scripts/run_ci.sh` GREEN (update any validate test that asserts an exact metric set). Tree
  CLOSED.

## Task Tree

- ID: `AMBIGUITY-PHRASE-DETECTOR`
  Status: `active`
  Children: `.1` (design) · `.2` (detector + validate surface + book + KM card + close)

- ID: `AMBIGUITY-PHRASE-DETECTOR.1`
  Status: `done`
  Goal: own + design (this file) — the flag-only scope, the grounded high-precision lexicon
    (excluding modal verbs), the `validate` surface, and the explicit deferral of residual
    routing. Docs-only.
  Acceptance: design + verified grounding recorded; registered.
  Verification: passed (`2026-06-04`) — scoped flag-only (report finding + metric, like the
    region-accounting detector) against the real `ExtractedStatement` (`.text`) +
    `validate_evidence_ir` finding/metric pattern; lexicon curated for precision (NASA ARM
    weak phrases + chip-spec under-specification idioms; modal verbs excluded); grounding
    citations (NASA ARM ICSE'97 DOI 10.1145/253228.253258; Berry-Kamsties) reused from the
    verified `requirements-extraction.md`.
  Commit: `see Commit Log`

- ID: `AMBIGUITY-PHRASE-DETECTOR.2`
  Status: `done`
  Goal: implement `ir/ambiguity.rs` (lexicon + detector + tests); wire the `validate`
    surface (summary + Info finding + metric); book note; KM card; close.
  Acceptance: tests green; `validate` flags ambiguous statements + metric; book + KM card;
    full CI GREEN; tree CLOSED.
  Verification: passed (`2026-06-04`) — new pure `crates/specforge/src/ir/ambiguity.rs`
    (`pub mod ambiguity;`): a curated `WEAK_PHRASES` lexicon (NASA ARM weak phrases + chip-spec
    "implementation-defined"/"vendor-specific"; modal verbs excluded, with a test asserting
    the exclusion) + `weak_phrase_findings(&[ExtractedStatement])` (case-insensitive,
    deterministic). 5 unit tests (flagged / clean / case-insensitive / modal-only-not-flagged /
    lexicon-excludes-modals). Wired into `validate_evidence_ir` (`commands/validate.rs`): an
    "Ambiguity / Weak Phrases" summary, an Info `evidence_ambiguous_statements` finding (related
    statement ids), and an `ambiguous_statements` metric — flag-only, extraction-neutral. 1
    validate-level test builds an EvidenceIR from markdown ("…stable as appropriate.") and
    asserts the metric ≥ 1 + the finding. User-friendly book subsection in
    `quality/validation.md`; Knowledge Map card `docs/knowledge/ambiguity-weak-phrase-detector.md`
    (KM now 5 facts / 25 question keys). Full `scripts/run_ci.sh` GREEN (1223→1229; no existing
    validate test broke). Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `AMBIGUITY-PHRASE-DETECTOR.1` | `done` | owned + designed (lexicon + surface + grounding) |
| 2 | `AMBIGUITY-PHRASE-DETECTOR.2` | `done` | detector + 6 tests + validate finding/metric + book + KM card + close (CI green 1229) |

**Tree CLOSED `2026-06-04`.** `validate` now flags statements carrying vague / under-specified
language (NASA ARM weak phrases + chip-spec "implementation-defined"/"vendor-specific") via the
pure `ir/ambiguity.rs` detector — surfacing where the spec is genuinely vague instead of
treating it as precise (residual-honesty), flag-only and extraction-neutral. Routing flagged
statements into typed residual packets is the deferred follow-up.

## Decisions

- `2026-06-04`: flag-only (a `validate` finding + metric), extraction-neutral — mirrors the
  existing completeness detectors. Curate the lexicon for precision (exclude modal verbs);
  residual-packet routing deferred to a future tree.

## Blockers

- None. Pure detector + an additive validate finding/metric.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | flag-only scope fixed vs `ExtractedStatement.text` + `validate_evidence_ir` finding/metric pattern; high-precision grounded lexicon (modal verbs excluded); verified citations reused; docs-only | `passed` |
| `2026-06-04` | `.2` | `ir/ambiguity.rs` (`WEAK_PHRASES` + `weak_phrase_findings`) + 5 unit tests (incl. modal-exclusion); `validate_evidence_ir` summary + `evidence_ambiguous_statements` Info finding + `ambiguous_statements` metric; 1 validate-level test (markdown→EvidenceIR→metric≥1+finding); book subsection in `quality/validation.md`; KM card (KM 5 facts/25 keys); flag-only/extraction-neutral; full CI GREEN 1223→1229; tree CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `AMBIGUITY-PHRASE-DETECTOR.1` | `AMBIGUITY-PHRASE-DETECTOR.1 — own + design the weak-phrase / ambiguity flag detector` (`fe3b19e9`) | docs-only |
| `AMBIGUITY-PHRASE-DETECTOR.2` | `AMBIGUITY-PHRASE-DETECTOR.2 — flag vague spec prose in validate (ir/ambiguity.rs); book + KM card; close tree` | detector + 6 tests + validate finding/metric + book + KM; CI green 1229; CLOSED |

## Changelog

- `2026-06-04`: Created — a grounded, flag-only weak-phrase/ambiguity detector
  (NASA ARM / Berry-Kamsties) that surfaces vague / under-specified spec prose in `validate`,
  in the residual-honesty spirit. Extraction-neutral; residual routing deferred.
- `2026-06-04`: **Tree CLOSED.** `.2` done — `ir/ambiguity.rs` detector + 5 unit tests, wired
  into `validate_evidence_ir` (summary + Info finding + `ambiguous_statements` metric) + 1
  validate-level test, a user-friendly book subsection (`quality/validation.md`), and a KM
  card. Flag-only/extraction-neutral; full CI green (1223→1229). Typed-residual routing of
  flagged statements remains a deferred follow-up.
