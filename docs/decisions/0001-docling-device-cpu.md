# 0001 — Docling ingest runs on CPU on this stack (torch MPS lacks float64)

- Date: 2026-06-01
- Status: accepted
- Tags: ingest, environment, docling, apple-silicon

## Context

PDF ingest uses Docling (via the repo-local `.venv-docling`, `docling==2.84.0`). A
fresh bootstrap installs the latest torch (2.12.0), which defaults its accelerator to
`auto` → resolves to Apple's **MPS (Metal)** backend on Apple Silicon. Docling's
layout/table models request float64, which torch's MPS backend cannot do, so **every
page** of a conversion fails: `ConversionError: Cannot convert a MPS Tensor to float64
dtype as the MPS framework doesn't support float64`. The original corpus ingest had run
on an older, compatible torch.

## Decision

The embedded Docling helper (`crates/specforge/src/ir/source/docling_backend.rs`,
`DOCLING_HELPER_SCRIPT`) selects the device deliberately rather than deferring to the
broken `auto` pick: honor an explicit `DOCLING_DEVICE` (`cpu`/`cuda`/`mps`); otherwise
use CUDA when `torch.cuda.is_available()`, else **CPU** — never `auto`→MPS — wrapped in
a defensive `try/except` fallback. On Apple Silicon this means CPU (correct, slower).

Operationally, if running an ingest with an older/un-patched binary, force CPU with the
env var: `DOCLING_DEVICE=cpu specforge ingest <pdf>`.

## Consequences

- Ingest works out of the box on Apple Silicon; no per-command env var needed.
- CPU is slower than GPU; a 585-page spec is a long (background) job. Correctness over
  speed. If a future torch build fixes MPS float64, `DOCLING_DEVICE=mps` opts back in.
- Process note: never `rm -rf generated/source_ir/<key>` to force a clean re-classify
  until **after** a re-ingest succeeds (or back it up first) — a failed re-ingest after
  deleting the artifact loses it (regenerable, but it was needed for diagnosis).
- The Docling models survive venv deletion (they live in `~/.cache/huggingface/hub`,
  ~506M); a venv rebuild does not re-download them. The venv is ~1.3G, local + untracked.

## Links

- Task-trees: `DOCLING-DEVICE-CPU-DEFAULT`, `CORPUS-HARDENING.4`.
