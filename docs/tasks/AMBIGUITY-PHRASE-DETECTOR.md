# AMBIGUITY-PHRASE-DETECTOR: flag vague / under-specified spec prose for review

## Metadata

- Tree ID: `AMBIGUITY-PHRASE-DETECTOR`
- Status: `active` (`.1` design done; `.2` = code + close)
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
  Status: `pending`
  Goal: implement `ir/ambiguity.rs` (lexicon + detector + tests); wire the `validate`
    surface (summary + Info finding + metric); book note; KM card; close.
  Acceptance: tests green; `validate` flags ambiguous statements + metric; book + KM card;
    full CI GREEN; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `AMBIGUITY-PHRASE-DETECTOR.1` | `done` | owned + designed (lexicon + surface + grounding) |
| 2 | `AMBIGUITY-PHRASE-DETECTOR.2` | `pending` | detector + validate surface + book + KM card + close |

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `AMBIGUITY-PHRASE-DETECTOR.1` | `AMBIGUITY-PHRASE-DETECTOR.1 — own + design the weak-phrase / ambiguity flag detector` | docs-only |

## Changelog

- `2026-06-04`: Created — a grounded, flag-only weak-phrase/ambiguity detector
  (NASA ARM / Berry-Kamsties) that surfaces vague / under-specified spec prose in `validate`,
  in the residual-honesty spirit. Extraction-neutral; residual routing deferred.
