# DOCLING-DEVICE-CPU-DEFAULT: make Docling ingest avoid the broken MPS auto-device

## Metadata

- Tree ID: `DOCLING-DEVICE-CPU-DEFAULT`
- Status: `done` (CLOSED)
- Roadmap lane: `R12` (ingest robustness)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: surfaced by `CORPUS-HARDENING.4` (the CHI re-validation re-ingest).

## Discovery

After restoring the Docling venv (fresh `pip install docling==2.84.0` → torch
2.12.0), every page of a CHI re-ingest failed:

```
docling.exceptions.ConversionError: Conversion failed ...
Page N: Cannot convert a MPS Tensor to float64 dtype as the MPS framework
        doesn't support float64. Please use float32 instead.
```

Root cause: Docling's `PdfPipelineOptions()` defaults `accelerator_options.device`
to `auto`, which `decide_device()` resolves to **MPS** on Apple Silicon. torch's
MPS backend cannot perform the float64 ops Docling's layout/table models request,
so conversion fails on every page. The original ingest evidently ran on an older,
compatible torch. The embedded helper (`docling_backend.rs` `DOCLING_HELPER_SCRIPT`)
builds `PdfPipelineOptions()` with no device, so it inherits the broken `auto`.

## Goal

Make the embedded Docling helper select a working device by default — never
auto-pick MPS — so ingest works out of the box on Apple Silicon with current torch,
without requiring a per-command `DOCLING_DEVICE=cpu` env var. Stay portable (don't
pessimize CUDA machines) and keep an explicit override.

## Design

In the helper, right after `pipeline_options = PdfPipelineOptions()`:

- Honor an explicit `DOCLING_DEVICE` env var (`cpu` / `cuda` / `mps`) — pass it to
  `AcceleratorOptions(device=...)`.
- Otherwise pick `cuda` when `torch.cuda.is_available()`, else `cpu` — i.e. avoid
  `auto`→`mps`. CUDA machines keep GPU acceleration; Apple Silicon falls back to
  correct-but-slower CPU; a user can still opt back into MPS with
  `DOCLING_DEVICE=mps` on a torch build that supports it.
- Wrap in try/except so any import/version drift falls back to the prior default
  (defensive — ingest must keep working).

## Non-Goals

- NOT changing extraction/classification logic — device selection only.
- NOT forcing CPU on CUDA hosts. NOT trying to make MPS float64 work (a torch
  limitation; out of our control).

## Acceptance Criteria

- Helper selects cpu/cuda (never auto→mps) by default; honors `DOCLING_DEVICE`;
  defensive fallback. A fresh ingest on this Mac runs **without** the env var.
  fmt + clippy + full CI green; book note; tree CLOSED.

## Task Tree

- ID: `DOCLING-DEVICE-CPU-DEFAULT`
  Status: `done`
  Children: `.1`

- ID: `DOCLING-DEVICE-CPU-DEFAULT.1`
  Status: `done`
  Goal: implement the device-selection block in the helper; rebuild; verify a fresh
    ingest (no env var) uses CPU and converts pages; CI; book note; close.
  Acceptance: as above.
  Verification: >
    passed (`2026-06-01`) — added a device-selection block to the embedded helper
    (`docling_backend.rs`) right after `PdfPipelineOptions()`: honor `DOCLING_DEVICE`
    if set; else `cuda` when `torch.cuda.is_available()` else `cpu` (never auto→mps);
    try/except defensive fallback. Verified: (a) helper logic resolves to `cpu` with
    no env on this Mac, `mps` still honored when explicit; (b) **fresh `--dry-run`
    ingest of a small PDF with the NEW binary and NO env var converted successfully
    — 0 MPS/float64 errors, valid SourceIR produced** (the failing run errored on
    every page before the fix). fmt + clippy clean; full CI green. Book note in
    `pipeline/sourceir.md`. The per-command `DOCLING_DEVICE=cpu` workaround is no
    longer needed for future ingests.
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `DOCLING-DEVICE-CPU-DEFAULT.1` | `done` | implemented + verified end-to-end (fresh no-env ingest converts); CI green |

Tree **CLOSED** (`2026-06-01`): ingest now works out-of-the-box on Apple Silicon
with current torch — no `DOCLING_DEVICE=cpu` env var needed.

## Decisions

- `2026-06-01`: avoid `auto` (not "force CPU always") — keeps CUDA hosts fast while
  fixing Apple Silicon; explicit `DOCLING_DEVICE` still wins.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | helper device-selection added (honor DOCLING_DEVICE; else cuda-or-cpu, never auto→mps; defensive fallback); logic verified (no-env→cpu, explicit mps honored); fresh no-env `--dry-run` ingest of a small PDF converted with 0 MPS errors + valid SourceIR; fmt/clippy clean; full CI green (1198) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `DOCLING-DEVICE-CPU-DEFAULT.1` | `DOCLING-DEVICE-CPU-DEFAULT.1 — avoid Docling auto→MPS device (fixes ingest on Apple Silicon w/ torch 2.12); close` | helper + book + close |

## Changelog

- `2026-06-01`: Created — fix the Docling helper's auto→MPS device selection that
  breaks ingest on Apple Silicon with torch 2.12.
- `2026-06-01`: CLOSED — helper now selects cuda-or-cpu (never auto→mps), honors
  `DOCLING_DEVICE`, defensive fallback; verified end-to-end (fresh no-env ingest
  converts, 0 MPS errors); CI green; book note. No env var needed going forward.
