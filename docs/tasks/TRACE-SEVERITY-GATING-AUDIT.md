# TRACE-SEVERITY-GATING-AUDIT: warnings/errors/fatals must never be masked by a trace/verbosity level

## Metadata

- Tree ID: `TRACE-SEVERITY-GATING-AUDIT`
- Status: `done` (CLOSED — clean audit; invariant recorded)
- Roadmap lane: `R0` (correctness / observability invariant)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: user invariant — **severity ≥ warning must NEVER be gated by a
  verbosity/trace level; trace levels govern informational output only. A masked error
  is a silent failure.** Audit the entire codebase to guarantee warnings, errors, and
  fatals are never masked out, and establish the invariant for future trace support.

## Recon (scopes the audit)

- **No `tracing`/`log`/`env_logger`/`slog` crate; no `warn!`/`error!`/`info!` macros; no
  `--verbose`/`--quiet`/trace-level/log-level flag** anywhere in `crates/specforge/src`.
  So there is no general verbosity framework today — the invariant is mostly
  forward-looking, but the audit must still rule out *ad hoc* gating.
- **Output** is plain `println!` / `eprintln!` (779 sites) — overwhelmingly unconditional
  informational command output.
- **"Severity"** exists only as `ValidationFindingSeverity { Info, Warning, Error }`
  (`ir/source.rs`); findings carry a `severity`. `project_validation::severity_rank`
  maps Info=1 / Warning=2 / Error=3. (No `Fatal` variant — fatal = `AppError` returned via
  `Result` to `main`, or a `panic`.)
- **Errors** flow as `crate::error::Result<_, AppError>` — propagated, not printed-and-gated.

## Risk surfaces to audit (the prime suspects)

1. **Finding severity filtering** — any use of `severity_rank` (or a severity comparison)
   that *filters/hides* findings at or above Warning (e.g. `findings.retain(|f| rank(f) >=
   threshold)` where the threshold could exclude Warning/Error, or a "show only Info"
   path). This is the closest thing to "verbosity gating severity" in the current code.
2. **Conditional ≥Warning output** — any `if <flag/cond> { eprintln!/println!(<warning or
   error text>) }` where the condition could suppress the warning/error.
3. **Error masking** — `Result` errors swallowed in ways that hide a failure:
   `let _ = …`, `.ok()`, `if let Ok(..)` with no else, `unwrap_or_default()` on a fallible
   op whose error should surface, `eprintln` of an error followed by `continue`/`Ok(())`
   that drops it without recording it.
4. **Validation report emission** — confirm Warning/Error findings are ALWAYS recorded in
   the report data structure (never conditionally appended) and always surfaced by
   `validate` / `project-validation` regardless of any display option.

## Goal

Verify (and, where violated, fix) that no warning/error/fatal is gated or suppressed by a
verbosity/trace/debug/quiet condition anywhere in the codebase; then record the invariant
as a tracked decision so future trace support cannot regress it.

## Non-Goals

- NOT adding a trace/verbosity framework (none exists; that would be its own tree). This
  is an audit + an invariant, not a feature.
- NOT changing informational (Info / plain status) output behavior.

## Acceptance Criteria

- The four risk surfaces audited exhaustively, with per-site findings recorded (each site:
  masks-≥Warning? yes→fix / no→why-safe). Any real masking fixed (≥Warning ungated) with a
  test. A layer-C decision record states the invariant. fmt + clippy + full CI green; tree
  CLOSED. (If zero masking is found, that is the result — recorded as a clean audit.)

## Task Tree

- ID: `TRACE-SEVERITY-GATING-AUDIT`
  Status: `done`
  Children: `.1`, `.2`, `.3`

- ID: `TRACE-SEVERITY-GATING-AUDIT.1`
  Status: `done`
  Goal: own + scope + recon (this file) — establish the codebase's output/severity model
    and the 4 risk surfaces. Docs-only.
  Verification: passed (`2026-06-01`) — recon done: no log/trace framework or verbosity
    flag; output = println/eprintln (779); severity = `ValidationFindingSeverity`
    {Info,Warning,Error}; `severity_rank` in project_validation; errors = `Result`/`AppError`.
    4 risk surfaces enumerated. Registered.
  Commit: `see Commit Log`

- ID: `TRACE-SEVERITY-GATING-AUDIT.2`
  Status: `done`
  Goal: audit the 4 risk surfaces exhaustively; record per-site findings (masking vs safe).
  Acceptance: every `severity_rank`/severity-comparison site + every conditional ≥Warning
    output + the error-swallow patterns examined; findings table recorded.
  Verification: passed (`2026-06-01`) — **CLEAN audit, zero masking found**:
    1. *Severity filtering:* `severity_rank` used only to SORT findings (Reverse → highest
       first; `sorted_findings`) and SUMMARIZE the worst (`primary_finding_summary`,
       `highest_severity_label`); never to filter/hide. All findings retained.
    2. *Conditional ≥Warning output:* the 2 production warning sites (`enrich.rs:172/200`,
       `nlp_enrich.rs:279`) emit `"warning: … failed: {e}"` UNCONDITIONALLY on the error
       path; the single `error:` print (`main.rs:7`) is the unconditional top-level
       handler. The 13 `=== ISF ===`/`=== ISF source text ===` `eprintln!` dumps are all
       inside `#[cfg(test)]` (isf_ir mod tests @1903; adapters mod tests @357).
    3. *Error swallow:* NO `let _ = <fallible>` / `.write_to_disk().ok()` drops; the `.ok()`
       sites are numeric-parse predicates (Option semantics), path-canonicalize fallbacks,
       and one optional region-accounting load (best-effort; suppresses no ≥Warning).
    4. *Report emission:* Warning/Error findings are pushed when the condition warrants and
       never behind a verbosity gate (none exists).
    Conclusion: the invariant is SATISFIED in the current codebase.

- ID: `TRACE-SEVERITY-GATING-AUDIT.3`
  Status: `done`
  Goal: fix any masking found (ungate ≥Warning + test); record the invariant as a layer-C
    decision record (and a guard/test if feasible); book note if user-facing; close.
  Acceptance: masking (if any) fixed + tested; invariant recorded; CI green; tree CLOSED.
  Verification: passed (`2026-06-01`) — no masking to fix (clean audit). Invariant recorded
    as layer-C decision `docs/decisions/0004-severity-never-gated-by-verbosity.md` (the
    rule + the audit result + the forward rule for any future trace framework). No code
    change (docs-only); no book note (internal observability invariant, not a user
    feature). A mechanical guard is deferred until a verbosity framework exists (nothing to
    gate today). Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TRACE-SEVERITY-GATING-AUDIT.1` | `done` | owned + scoped (recon) |
| 2 | `TRACE-SEVERITY-GATING-AUDIT.2` | `done` | exhaustive audit — CLEAN, zero masking |
| 3 | `TRACE-SEVERITY-GATING-AUDIT.3` | `done` | invariant recorded (decision 0004); closed |

Tree **CLOSED** (`2026-06-01`): clean audit — no warning/error/fatal is gated by a
verbosity/trace level (there is no verbosity framework; ≥Warning output is unconditional;
severity is sorted-to-top + summarized, never filtered; errors propagate via `Result`).
The invariant is recorded for future trace support in `docs/decisions/0004`.

## Decisions

- `2026-06-01`: since no verbosity framework exists, the audit's value is (a) ruling out
  ad hoc gating now, and (b) recording the invariant so future trace support honors it.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | recon: no log/trace framework or verbosity flag; severity model + 4 risk surfaces enumerated; docs-only | `passed` |
| `2026-06-01` | `.2` | exhaustive audit of all 4 surfaces — severity_rank sorts/summarizes only; ≥Warning eprintln unconditional (enrich/nlp_enrich/main); ISF dumps test-only; no Result silently dropped; CLEAN (zero masking) | `passed` |
| `2026-06-01` | `.3` | invariant recorded as decision 0004; no masking to fix; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TRACE-SEVERITY-GATING-AUDIT.1` | `TRACE-SEVERITY-GATING-AUDIT.1 — own + scope the severity-never-masked audit` | docs-only |
| `TRACE-SEVERITY-GATING-AUDIT.{2,3}` | `TRACE-SEVERITY-GATING-AUDIT.2/.3 — exhaustive audit (CLEAN, zero masking) + record invariant (decision 0004); close` | docs-only |

## Changelog

- `2026-06-01`: Created — audit that no warning/error/fatal is gated by a verbosity/trace
  level; recon establishes there is no verbosity framework today (forward-looking invariant
  + rule out ad hoc gating). Frontier → `.2`.
- `2026-06-01`: CLOSED — exhaustive audit CLEAN (zero masking; ≥Warning output is
  unconditional, severity sorted-to-top not filtered, errors propagate via Result);
  invariant recorded as decision 0004 for future trace support.
