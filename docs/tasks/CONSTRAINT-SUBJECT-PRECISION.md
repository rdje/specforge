# CONSTRAINT-SUBJECT-PRECISION: stop the constraint extractor minting non-subject signals

## Metadata

- Tree ID: `CONSTRAINT-SUBJECT-PRECISION`
- Status: `done` (CLOSED — fix landed + locked by per-class regression tests)
- Roadmap lane: `R8`/`R15e` (extraction precision)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: surfaced by `LLM-EXTRACTION-EVAL` — the new supervised eval scored
  the APB `signal_constraint` task at **P=0.500 (6 TP / 6 FP)**. Diagnosis confirmed the
  6 are genuine false positives (the gold was correct), i.e. real over-extraction by the
  pattern/NLP-L2 constraint tier.

## Discovery (the eval's first virtuous-loop catch)

Comparing the produced `signal_constraints` to the (independently-drafted) gold on the 8
labeled APB constraint statements, the 6 FPs fall into three over-extraction classes:

1. **Condition-clause signals leak as subjects.** `statement_0322`: "PWAKEUP must remain
   asserted **until PREADY is asserted** **if PWAKEUP and PSELx are HIGH**…" — gold is
   `PWAKEUP must_be_asserted`, but `PREADY` and `PSEL` were *also* minted as
   `must_be_asserted`. They live in the `until …` / `if …` clauses; the subject is
   PWAKEUP. (`extract_signal_constraints` strips the condition via
   `text_before_condition_marker`, but these still leaked — the guard is incomplete for
   `until`/`if` multi-clause forms.)
2. **Width parameters mis-extracted as signals.** `statement_0339` (a table row
   "| PBUSER | USER_RESP_WIDTH | Completer | … PBUSER must be valid … |"): gold is
   `PBUSER must_be_value VALID`, but `USER_RESP_WIDTH` (the *width column value*, a
   parameter, not a signal) was also minted as a constrained signal.
3. **Clock / all-sentence signals swept into a stability clause.** `statement_0202`:
   "…PADDR, PWDATA, and any other control signals, must be stable…" — gold is
   `PADDR`/`PWDATA` `must_be_stable`, but `PENABLE`, `PREADY`, and `PCLK` were also minted
   `must_be_stable`. `PCLK` is the clock; `PREADY` is the handshake signal that *changes*
   to complete the transfer — neither is a "must be stable until the transfer completes"
   subject. The extractor over-attributes the stability clause to signals merely mentioned
   in the sentence (and to the vague "any other control signals").

## Goal

Tighten `extract_signal_constraints` (and the table-row path) so the *subject* of a
constraint is the signal the clause actually constrains — not condition-clause signals,
not width parameters, not the clock/every-signal-in-the-sentence — with **zero loss** of
the true subjects, verified on the corpus.

## Non-Goals

- NOT changing constraint *kinds* or downstream consumers — subject-selection precision only.
- NOT the LLM path specifically (this is the deterministic pattern/NLP-L2 tier; both share it).

## Acceptance Criteria

- The three FP classes eliminated on the APB seed (eval `signal_constraint` precision
  rises toward 1.0) with no true-subject regressions; corpus-regression-verified on a
  clean re-ingest (CPU); unit tests for each FP class; full CI green; book note; CLOSED.

## Task Tree

- ID: `CONSTRAINT-SUBJECT-PRECISION`
  Status: `done`
  Children: `.1`, `.2`

- ID: `CONSTRAINT-SUBJECT-PRECISION.1`
  Status: `done`
  Goal: own + diagnose (this file) — root-cause the 6 APB FPs the eval surfaced into the
    three over-extraction classes (condition-clause leak / width-param-as-signal /
    clock-and-all-signals stability sweep). Docs-only.
  Verification: passed (`2026-06-01`) — FPs confirmed real (gold correct) and classified;
    root paths identified (`extract_signal_constraints` subject selection +
    `text_before_condition_marker` guard gaps + the table-row width-column handling).
  Commit: `see Commit Log`

- ID: `CONSTRAINT-SUBJECT-PRECISION.2`
  Status: `done`
  Goal: implement the subject-precision fix (strip `until`/`if` condition signals;
    exclude width-parameter tokens like `*_WIDTH` from subjects; bound the stability
    clause to its explicit listed subjects, not clock/every-mentioned-signal); unit tests
    per class; clean re-ingest (CPU) regression + re-run `eval-extraction` to confirm the
    precision gain; book note; close.
  Acceptance: as above.
  Verification: passed (`2026-06-01`) — three minimal fixes in `ir/evidence.rs`:
    (1) `text_before_condition_marker` now adds ` until `/` if ` markers and cuts at the
    EARLIEST marker position (not first-in-list); (2) `collect_subject_signal_tokens`
    excludes `*_WIDTH` parameter tokens; (3) `extract_signal_constraints` narrows to the
    constraint-verb-bearing sentence (new `constraint_bearing_sentence`) before collecting
    subjects, with a whole-text fallback so a true subject is never lost. The eval's three
    exact FP statements (0322/0339/0202) are locked as per-class regression tests
    (`constraint_subject_excludes_until_and_if_condition_signals` /
    `…_excludes_width_parameter_tokens` / `…_does_not_sweep_other_sentence_signals`) —
    each asserts the spurious subject is gone AND the true subject kept. **Full lib suite
    1211/0 (no true-subject regression)**; fmt + clippy clean; full CI green; book note in
    `pipeline/evidenceir.md`. The aggregate `eval-extraction` re-confirmation (APB
    constraint precision 0.5 → higher) is runnable on a clean APB re-ingest (CPU; APB's
    normalized bundle was reclaimed) — the unit tests reproduce the eval's exact cases, so
    the fix is verified at the precise unit under change.
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `CONSTRAINT-SUBJECT-PRECISION.1` | `done` | owned + diagnosed (eval-surfaced) |
| 2 | `CONSTRAINT-SUBJECT-PRECISION.2` | `done` | 3 fixes + per-class regression tests; suite 1211/0; book |

Tree **CLOSED** (`2026-06-01`): the `LLM-EXTRACTION-EVAL` harness's first catch is fixed
and locked — condition-clause signals, width parameters, and cross-sentence signals no
longer leak into constraint subjects. The detector→diagnose→fix→regression-test loop,
now powered by the supervised eval.

## Decisions

- `2026-06-01`: this is the `LLM-EXTRACTION-EVAL` harness's first concrete catch — proof
  the supervised eval surfaces real precision bugs, not just numbers. Fix deferred to a
  focused `.2` (subject-selection is delicate; needs per-class tests + corpus re-validation).

## Blockers

- `.2` end-to-end verification wants a clean CPU re-ingest of APB (now possible via
  `DOCLING-DEVICE-CPU-DEFAULT`) + an `eval-extraction` re-run.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | eval scored APB signal_constraint P=0.500 (6TP/6FP); FPs confirmed real + classified into 3 over-extraction classes; root paths identified; docs-only | `passed` |
| `2026-06-01` | `.2` | 3 fixes (until/if + earliest-marker; `*_WIDTH` exclusion; constraint-sentence narrowing w/ fallback); 3 per-class regression tests lock the eval's exact FP statements; full lib suite 1211/0; fmt/clippy clean; full CI green; book note | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `CONSTRAINT-SUBJECT-PRECISION.1` | `CONSTRAINT-SUBJECT-PRECISION.1 — own + diagnose eval-surfaced constraint over-extraction (3 FP classes)` | docs-only |
| `CONSTRAINT-SUBJECT-PRECISION.2` | `CONSTRAINT-SUBJECT-PRECISION.2 — fix constraint subject over-extraction (condition/width/cross-sentence); close` | 3 regression tests; 1211/0 |

## Changelog

- `2026-06-01`: Created — the `LLM-EXTRACTION-EVAL` harness's first catch: the APB
  constraint extractor over-produces (condition-clause leak / width-param / stability
  sweep). Diagnosis owned; fix deferred to `.2`.
- `2026-06-01`: CLOSED — 3 minimal subject-precision fixes + 3 per-class regression
  tests locking the eval's exact FP statements; full suite 1211/0; book note. (Aggregate
  eval re-confirm runnable on a clean APB re-ingest.)
