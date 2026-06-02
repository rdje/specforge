# 0004 — Severity ≥ Warning is never gated by a verbosity/trace level

- Date: 2026-06-01
- Status: accepted
- Tags: observability, correctness, invariant, trace

## Context

A masked error is a silent failure. **Trace/verbosity levels govern informational
output only** — they must never decide whether a warning, error, or fatal is shown,
recorded, or propagated. This must hold across the entire codebase, now and as any trace
support is added later.

State at audit time (`TRACE-SEVERITY-GATING-AUDIT`, `2026-06-01`): SpecForge has **no
`tracing`/`log` framework and no verbosity/trace-level/`--quiet` flag**. Output is plain
`println!`/`eprintln!`; "severity" is `ValidationFindingSeverity { Info, Warning, Error }`
(`ir/source.rs`); fatals/errors flow as `AppError` via `Result` to `main`.

## Decision

1. **Warnings, errors, and fatals are emitted / recorded / propagated unconditionally.**
   They are never placed behind a verbosity/trace/debug/quiet condition. Only Info /
   Debug / Trace output may be level-gated.
2. **No finding-display path may hide a `Warning`/`Error` finding.** Sorting or
   summarizing by severity is fine (and should float ≥Warning to the top); filtering them
   out is not.
3. **Forward rule for any future trace framework:** the level gate applies to
   informational output *only*; the ≥Warning path must bypass it entirely. A reviewer (or
   agent) checks every new gated output against this record.

## Audit result (clean — invariant already satisfied)

- **Finding severity** (`project_validation::severity_rank`) is used only to *sort*
  findings (Reverse rank → highest first) and to *summarize* the worst (`primary_finding_summary`,
  `highest_severity_label`) — never to filter/hide. All findings are retained.
- **≥Warning output is unconditional:** the production warning sites
  (`enrich.rs`, `nlp_enrich.rs`) emit `"warning: … failed: {e}"` to stderr on the error
  path with no gate; the single `error:` print (`main.rs`) is the unconditional top-level
  handler before a non-zero exit. (The `=== ISF ===` `eprintln!` dumps in `isf_ir.rs` /
  `adapters.rs` are inside `#[cfg(test)]`.)
- **No errors are silently dropped:** no `let _ = <fallible>` / `.write_to_disk().ok()`
  patterns; the `.ok()` sites are numeric-parse predicates (Option semantics),
  path-canonicalize fallbacks, and one *optional* region-accounting enrichment load
  (best-effort; suppresses no ≥Warning finding).

## Consequences

- Any future trace/verbosity feature is constrained by this invariant (rule 3).
- If a verbosity framework is added, consider a guard (test/lint) asserting ≥Warning
  output is not reachable only under a non-default level — at present there is no
  framework to guard, so the invariant is documented rather than mechanically enforced.

## Links

- Task-tree: `TRACE-SEVERITY-GATING-AUDIT`.
