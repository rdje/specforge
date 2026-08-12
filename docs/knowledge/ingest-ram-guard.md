---
id: ingest-ram-guard
title: ingest has a built-in autonomous RAM guard that aborts cleanly before the host crosses a danger ceiling
answers:
  - "how does ingest avoid crashing the host when memory runs out"
  - "what is the built-in RAM guard / autonomous memory safeguard during ingest"
  - "what does SPECFORGE_INGEST_RAM_ABORT_PERCENT do"
  - "what does SPECFORGE_INGEST_RAM_SAMPLE_SECS do"
  - "how does specforge read system memory without a new dependency"
  - "why did ingest stop with 'ingest aborted to protect the host'"
  - "what is AppError::IngestAbortedForMemory"
  - "how is IngestTerminatedBySignal different from IngestAbortedForMemory"
  - "does a SIGKILL prove that Docling ran out of memory"
  - "where is the spawn+poll+kill memory guard in materialize_pdf"
date: 2026-06-14
tags: [ingest, docling, memory-bounded, ram, safety, source-ir]
evidence: crates/specforge/src/ir/source/docling_backend.rs (RamGuardConfig / run_backend_with_ram_guard / current_used_memory_percent / backend_exit_error); crates/specforge/src/error.rs (AppError::IngestAbortedForMemory / IngestTerminatedBySignal); docs/tasks/MEMORY-BOUNDED-INGEST.md (.4a); docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md (.6b.iii)
reverify: grep -n "run_backend_with_ram_guard\|RamGuardConfig\|SPECFORGE_INGEST_RAM_ABORT_PERCENT\|IngestAbortedForMemory\|IngestTerminatedBySignal\|backend_exit_error" crates/specforge/src/ir/source/docling_backend.rs crates/specforge/src/error.rs
---

`MEMORY-BOUNDED-INGEST.4a` makes the autonomous RAM guard a **first-class, built-in** ingest
safeguard (previously it was an external shell wrapper an operator had to remember). While the
Docling subprocess runs, `specforge` samples system memory and aborts the ingest cleanly before the
host crosses a configurable danger ceiling — encoding the owner's non-negotiable "kill at ≥85% used;
never crash the host" rule into the tool itself.

**No new dependency** (crate stays at 4 deps): memory is read via the platform's OWN tool, matching
the exact metric the owner monitors. macOS → run `memory_pressure`, parse the "free percentage"
line, used = 100 − free. Linux → read `/proc/meminfo`, used% = (1 − MemAvailable/MemTotal) × 100.
Other OS → unreadable → guard inert (never aborts on missing data). All text parsers
(`parse_macos_memory_pressure_used_percent`, `parse_linux_meminfo_used_percent`, `parse_meminfo_kb`,
`parse_leading_number`) are PURE fns gated `#[cfg(any(target_os = "...", test))]` so they unit-test
on every platform without dead-code on the other.

**Mechanism** lives in `docling_backend::run_backend_with_ram_guard` (called from `materialize_pdf`,
replacing the blocking `command.output()`): sample once BEFORE spawn (don't even launch a heavy
ingest if already in danger), then spawn with stdout/stderr redirected to temp FILES (avoids
pipe-fill deadlock while polling), poll `child.try_wait()` on a short fixed cadence
(`RAM_GUARD_POLL_INTERVAL` = 50 ms) and sample memory only every `sample_interval`. On breach:
`child.kill()` + `wait()` and return the typed error. The reader is injected
(`used_percent_fn: &dyn Fn() -> Option<f64>`) so tests cover pre-spawn abort / mid-run kill /
completes with NO real memory pressure. The staged-swap means an abort discards only the in-flight
`normalized.staging` tree — the last-good `normalized/` + `source_ir.json` are untouched.

**Config**: `SPECFORGE_INGEST_RAM_ABORT_PERCENT` (default **85**; `off`/`none`/`disabled` or any
value `<=0` / `>=100` → disabled; unparseable → default). `SPECFORGE_INGEST_RAM_SAMPLE_SECS`
(default 2, floor 1). New `AppError::IngestAbortedForMemory { program, used_percent, ceiling_percent }`
with an actionable Display (host preserved; free memory / raise the ceiling / set it off).

**Signal termination is intentionally different (`2026-08-12`).** If the child exits because Unix reports an
operating-system signal, `backend_exit_error` returns `AppError::IngestTerminatedBySignal` with that signal and
the captured diagnostics. A signal— including SIGKILL—can be external resource enforcement, but it does not by
itself prove OOM. Only this guard's observed threshold breach returns `IngestAbortedForMemory`. Both paths remove
the incomplete staging tree and preserve the last-good bundle; the signal diagnostic directs the operator to OS
resource logs and the bounded threshold/batch controls. See [[bounded-ingest-resource-risk-below-page-threshold]].

**Determinism gotcha (fixed here)**: the 3 source.rs stub-helper ingest tests
(`pdf_source_ir_materialization_*`, `..._failed_materialization_*`) now run through the guard with
the REAL reader at default 85% — a hot CI host (>85% used) could false-abort them, so they set
`SPECFORGE_INGEST_RAM_ABORT_PERCENT=off` via the existing `EnvVarGuard`/`env_var_lock` pattern;
behavior is then identical to the pre-guard blocking path. Related: [[page-image-disk-bounding]].
