---
id: docling-device-cpu
title: Docling ingest must run on CPU on this stack (torch MPS lacks float64)
answers:
  - "why does Docling re-ingest fail on Apple Silicon"
  - "Cannot convert a MPS Tensor to float64"
  - "how do I run a Docling ingest or re-ingest on this machine"
  - "what does DOCLING_DEVICE do"
  - "torch MPS float64 error during ingest"
date: 2026-06-01
tags: [ingest, docling, environment]
evidence: docs/decisions/0001-docling-device-cpu.md; crates/specforge/src/ir/source/docling_backend.rs:444
reverify: grep -n DOCLING_DEVICE crates/specforge/src/ir/source/docling_backend.rs
---

Docling ingest must run on **CPU** on this stack. Fresh torch (2.12) on Apple Silicon routes
`auto` to MPS, which lacks float64 and dies with `Cannot convert a MPS Tensor to float64`. The
ingest helper honours an explicit `DOCLING_DEVICE` (cpu/cuda/mps); otherwise CUDA-if-available,
else CPU — it **never** auto-selects MPS. To (re-)ingest a spec, run with `DOCLING_DEVICE=cpu`.

Canonical home: `docs/decisions/0001-docling-device-cpu.md` (rationale) + the device-selection
block in `crates/specforge/src/ir/source/docling_backend.rs`. Related gotcha: never delete a
document's `source_ir/` before a re-ingest has actually succeeded.
