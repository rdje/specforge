---
id: ingest-adaptive-batch-sizing
title: ingest sizes each page-range batch to the host's total physical RAM so a small machine completes
answers:
  - "how does ingest avoid being RAM-guard-aborted on a small/restricted machine"
  - "how is the page-range batch size chosen / adapted"
  - "what does SPECFORGE_INGEST_ADAPTIVE_BATCH do"
  - "is SPECFORGE_INGEST_BATCH_PAGES a fixed size or a ceiling"
  - "why does the batch size depend on total RAM instead of free memory"
  - "does adaptive batch sizing change the ingest output / break byte-identity"
  - "where is adaptive_batch_pages / BatchSizePolicy in the code"
  - "how does specforge read total physical RAM without a new dependency"
date: 2026-06-14
tags: [ingest, docling, memory-bounded, ram, batch, determinism, source-ir]
evidence: crates/specforge/src/ir/source/docling_backend.rs (BatchSizePolicy / adaptive_batch_pages / current_total_memory_mb / parse_sysctl_memsize_bytes / parse_linux_meminfo_total_mb; materialize_pdf child-env wiring); docs/tasks/MEMORY-BOUNDED-INGEST.md (.4c)
reverify: grep -n "adaptive_batch_pages\|BatchSizePolicy\|SPECFORGE_INGEST_ADAPTIVE_BATCH\|current_total_memory_mb\|INGEST_BATCH_PAGES_ENV" crates/specforge/src/ir/source/docling_backend.rs
---

`MEMORY-BOUNDED-INGEST.4c` sizes each page-range batch (the `.1` batching mechanism) to the **host's
total physical RAM** so a small/restricted machine COMPLETES a large-doc ingest — slower, in smaller
batches — instead of being killed every time by the `.4a` RAM guard. The fixed 64-page batch is
right for a 24 GB host but too large for, say, a 4 GB container (a 64-page batch + the layout/table
models would cross the danger ceiling, so the guard aborts the run no matter how patient the
operator is).

**Total RAM, NOT free memory — for determinism.** Free memory jitters run-to-run, which would make
the chosen batch size — and therefore the cross-batch boundary artifacts — non-deterministic,
violating the repo's determinism doctrine ([[evidence-build-nondeterminism]]). Total physical RAM is
a per-machine CONSTANT, so the batch size is a deterministic function of the machine: the SAME
machine always picks the SAME batch and re-ingest stays reproducible. Transient pressure from
co-tenant processes stays the `.4a` RAM guard's job (the unchanged hard backstop).

**`SPECFORGE_INGEST_BATCH_PAGES` is now a CEILING** (default 64, unchanged for the common case);
adaptive sizing only ever LOWERS it. Discrete RAM bands (jitter-free; target keeps a 64-page-batch
peak — ~4.8 GB, the `.2` CHI datum — near ~30 % of RAM): `>= 16 GB -> ceiling`, `8-16 GB ->
min(ceiling, 32)`, `4-8 GB -> min(ceiling, 16)`, `< 4 GB -> floor (8, clamped <= ceiling)`. Every
current verification host (>= 16 GB, incl. the 24 GB dev host) lands on the ceiling, so CHI (`.2`)
and all gold/intact docs re-ingest BYTE-IDENTICAL; only genuinely small machines get a smaller batch
(full per-page fidelity, only the same benign cross-batch boundary artifact `.1` already documents,
and no prior successful baseline on such a host to diverge from). `SPECFORGE_INGEST_ADAPTIVE_BATCH`
(default on; `off`/`none`/`disabled`) forces the fixed ceiling = the exact `.1` behavior.

**Rust owns the decision, Python is unchanged.** `materialize_pdf` reads total RAM via a no-dep
platform reader (`sysctl -n hw.memsize` on macOS, `/proc/meminfo` `MemTotal` on Linux; pure parsers
`parse_sysctl_memsize_bytes` / `parse_linux_meminfo_total_mb` gated `#[cfg(any(target_os, test))]`),
computes the effective batch via the pure `adaptive_batch_pages(total_mb, ceiling, floor)`, and sets
the child's `SPECFORGE_INGEST_BATCH_PAGES` env to that value — the Docling helper already reads it, so
no Python logic changes. The RAM reader is injected (`&dyn Fn() -> Option<u64>`) for host-safe DI
tests, exactly like the `.4a` guard's `used_percent_fn`; `None` (unreadable RAM) -> ceiling
(permissive — behave as today, never worse on missing data).

Verified live on the 24 GB dev host: a throwaway CAN copy forced large (`SPECFORGE_INGEST_BATCH_THRESHOLD=1`)
ingested adaptive-ON (-> 64) is `diff -r` byte-identical to adaptive-OFF (fixed 64); a ceiling of 32
yields 3 batches (`[[1,32],[33,64],[65,72]]`) with the same 72 pages / 98 region assets captured
(lever works end-to-end, fidelity intact). Related: [[ingest-ram-guard]], [[ingest-disk-preflight]],
[[page-image-disk-bounding]].
